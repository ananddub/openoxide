use super::state;
use crate::api::proto::{ContainerStatesResponse, LifecycleControl};
use crate::docker::api::DockerApi;
use std::{pin::Pin, time::Duration};
use tokio_stream::{Stream, StreamExt};
use tonic::{Response, Status};

pub type LifecycleStream =
    Pin<Box<dyn Stream<Item = Result<ContainerStatesResponse, Status>> + Send + 'static>>;

pub struct LifecycleService;

impl LifecycleService {
    pub async fn watch(
        docker: DockerApi,
        mut controls: tonic::Streaming<LifecycleControl>,
    ) -> Result<Response<LifecycleStream>, Status> {
        controls
            .message()
            .await?
            .ok_or_else(|| Status::invalid_argument("missing lifecycle subscription"))?;

        let (tx, rx) = tokio::sync::mpsc::channel(8);
        tokio::spawn(async move {
            let mut events = match docker.get_stream("/events").await {
                Ok(events) => events,
                Err(error) => {
                    let _ = tx.send(Err(Status::unavailable(error.to_string()))).await;
                    return;
                }
            };
            let mut heartbeat = tokio::time::interval(Duration::from_secs(15));
            heartbeat.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
            let mut buffer = Vec::new();

            if state::StateService::send_snapshot(&docker, &tx)
                .await
                .is_err()
            {
                return;
            }

            loop {
                tokio::select! {
                    control = controls.message() => match control {
                        Ok(Some(_)) => {}
                        _ => break,
                    },
                    _ = heartbeat.tick() => {
                        if state::StateService::send_snapshot(&docker, &tx).await.is_err() { break; }
                    }
                    chunk = events.next() => match chunk {
                        Some(Ok(bytes)) => {
                            buffer.extend_from_slice(&bytes);
                            while let Some(position) = buffer.iter().position(|byte| *byte == b'\n') {
                                buffer.drain(..=position);
                                if state::StateService::send_snapshot(&docker, &tx).await.is_err() { return; }
                            }
                        }
                        Some(Err(error)) => {
                            let _ = tx.send(Err(Status::unavailable(error.to_string()))).await;
                            break;
                        }
                        None => break,
                    }
                }
            }
        });

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(rx),
        )))
    }
}
