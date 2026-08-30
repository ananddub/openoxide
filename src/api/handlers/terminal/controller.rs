use std::sync::Arc;

use connectrpc::{
    ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult,
};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, ExecutionCommandRequest, OpenTerminalRequest, TerminalEvent, TerminalKind,
    TerminalService, TerminalServiceExt, TerminalStarted, execution_command_request,
    terminal_event,
};
use crate::services::observability::monitoring::proto::{ShellAction, ShellMessage};

use super::helper::*;
use super::init::TerminalController;

#[auto_grpc::service(connect = true, auth = true)]
impl TerminalService for TerminalController {
    #[permission(Server, CanRead)]
    async fn open_terminal(
        &self,
        ctx: RequestContext,
        request: ServiceRequest<'_, OpenTerminalRequest>,
    ) -> ServiceResult<connectrpc::ServiceStream<impl connectrpc::Encodable<TerminalEvent> + Send + use<>>> {
        let request = request.to_owned_message();
        let owner_user_id = auth.user_id();
        let kind = match request.kind {
            ::buffa::EnumValue::Known(kind) if kind != TerminalKind::Unspecified => kind,
            _ => return Err(ConnectError::invalid_argument("terminal kind is required")),
        };
        let (cols, rows) = clamp_size(request.cols, request.rows);

        let session_id = uuid::Uuid::new_v4().to_string();
        let (events_tx, events_rx) = mpsc::channel::<Result<TerminalEvent, ConnectError>>(64);
        let cancel = CancellationToken::new();

        let setup = match kind {
            TerminalKind::Docker => {
                Self::open_agent_session(&request, cols, rows).await
            }
            TerminalKind::RemoteServer => {
                Self::open_russh_session(&request, cols, rows).await
            }
            TerminalKind::Unspecified => unreachable!("kind validated above"),
        };

        let (handle, pump, host) = match setup {
            Ok(setup) => setup,
            Err(message) => {
                // Surface late startup failures inside the stream so the
                // terminal UI sees the same error shape as runtime errors.
                tracing::warn!("terminal session failed to open: {message}");
                let _ = events_tx.send(error_event(&session_id, message)).await;
                return Response::stream_ok(TerminalEventStream {
                    receiver: events_rx,
                    sessions: Arc::clone(&self.sessions),
                    session_id,
                });
            }
        };

        self.sessions.insert(
            session_id.clone(),
            TerminalSessionEntry {
                owner_user_id,
                cancel: cancel.clone(),
                session: handle,
            },
        );

        let _ = events_tx
            .send(Ok(terminal_event(
                &session_id,
                terminal_event::Payload::Started(Box::new(TerminalStarted {
                    kind: ::buffa::EnumValue::Known(kind),
                    host,
                })),
            )))
            .await;

        match pump {
            TerminalPump::Agent { stream } => spawn_agent_pump(
                session_id.clone(),
                stream,
                events_tx,
                cancel,
                Arc::clone(&self.sessions),
            ),
            TerminalPump::Russh { terminal } => spawn_russh_pump(
                session_id.clone(),
                terminal,
                events_tx,
                cancel,
                Arc::clone(&self.sessions),
            ),
        }

        Response::stream_ok(TerminalEventStream {
            receiver: events_rx,
            sessions: Arc::clone(&self.sessions),
            session_id,
        })
    }

    #[permission(Server, CanRead)]
    async fn execution_command<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ExecutionCommandRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        let request = request.to_owned_message();
        let command = request
            .command
            .ok_or_else(|| ConnectError::invalid_argument("terminal command is required"))?;

        let (owner_user_id, handle) = {
            let Some(entry) = self.sessions.get(&request.session_id) else {
                // The browser can flush a queued command after the session
                // ended (startup failure or natural exit); the terminal
                // error is already streamed, so stay silent here.
                tracing::debug!(
                    session_id = %request.session_id,
                    "terminal command for closed session"
                );
                return Response::ok(Empty {});
            };
            (entry.owner_user_id, entry.session.clone())
        };

        if owner_user_id != auth.user_id() {
            return Err(ConnectError::new(
                ErrorCode::PermissionDenied,
                "terminal session belongs to another user",
            ));
        }

        match command {
            execution_command_request::Command::Input(input) => match handle {
                TerminalSessionHandle::AgentShell { sender } => {
                    let sender = sender.lock().await;
                    sender
                        .send(ShellMessage {
                            action: ShellAction::Input as i32,
                            data: input.data.to_vec(),
                            ..Default::default()
                        })
                        .await
                        .map_err(|error| {
                            ConnectError::new(
                                ErrorCode::Internal,
                                format!("could not write Agent shell input: {error}"),
                            )
                        })?;
                }
                TerminalSessionHandle::RusshShell { terminal } => {
                    terminal.write(&input.data).await.map_err(|error| {
                        ConnectError::new(
                            ErrorCode::Internal,
                            format!("could not write SSH terminal input: {error}"),
                        )
                    })?;
                }
            },
            execution_command_request::Command::Resize(resize) => {
                let (cols, rows) = clamp_size(Some(resize.cols), Some(resize.rows));
                match handle {
                    TerminalSessionHandle::AgentShell { sender } => {
                        let sender = sender.lock().await;
                        sender
                            .send(ShellMessage {
                                action: ShellAction::Resize as i32,
                                cols: cols as u32,
                                rows: rows as u32,
                                ..Default::default()
                            })
                            .await
                            .map_err(|error| {
                                ConnectError::new(
                                    ErrorCode::Internal,
                                    format!("could not resize Agent shell: {error}"),
                                )
                            })?;
                    }
                    TerminalSessionHandle::RusshShell { terminal } => {
                        terminal.resize(cols, rows).await.map_err(|error| {
                            ConnectError::new(
                                ErrorCode::Internal,
                                format!("could not resize SSH terminal: {error}"),
                            )
                        })?;
                    }
                }
            }
            execution_command_request::Command::Stop(_) => {
                if let Some((_, entry)) = self.sessions.remove(&request.session_id) {
                    Self::terminate(entry).await;
                }
            }
        }

        Response::ok(Empty {})
    }
}
