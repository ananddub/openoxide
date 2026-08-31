use connectrpc::{
    ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult,
};

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, ExecutionCommandRequest, OpenTerminalRequest, TerminalEvent, TerminalKind,
    TerminalService, TerminalServiceExt,
};
use crate::services::infrastructure::terminal::TerminalCommandError;

use super::helper::TerminalEventStream;
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
        let kind = match request.kind {
            ::buffa::EnumValue::Known(kind) if kind != TerminalKind::Unspecified => kind,
            _ => return Err(ConnectError::invalid_argument("terminal kind is required")),
        };

        let session = self.service.open(kind, request, auth.user_id()).await;
        Response::stream_ok(TerminalEventStream::new(session))
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

        self.service
            .command(&request.session_id, auth.user_id(), command)
            .await
            .map_err(|error| match error {
                TerminalCommandError::PermissionDenied => ConnectError::new(
                    ErrorCode::PermissionDenied,
                    "terminal session belongs to another user",
                ),
                TerminalCommandError::Failed(message) => {
                    ConnectError::new(ErrorCode::Internal, message)
                }
            })?;

        Response::ok(Empty {})
    }
}
