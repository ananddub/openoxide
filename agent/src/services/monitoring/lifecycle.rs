use super::state;
use crate::api::proto::{ContainerStatesResponse, WatchRequest};
use crate::docker::api::DockerApi;
use std::{pin::Pin, time::Duration};
use tokio::sync::watch::Receiver;
use tokio_stream::{Stream, StreamExt};
use tonic::{Response, Status};
use crate::services::monitoring::state::StateService;

pub type LifecycleStream =
    Pin<Box<dyn Stream<Item = Result<ContainerStatesResponse, Status>> + Send + 'static>>;

pub struct LifecycleService{
    watch_state: Receiver<StateService>,
}



impl LifecycleService {
    pub async fn watch(
        docker: DockerApi,
        request: WatchRequest,
    ) -> Result<Response<LifecycleStream>, Status> {
        if request.server_id <= 0 {
            return Err(Status::invalid_argument("server_id is required"));
        }
        let initial = state::StateService::current(&docker).await?;
        let (tx, rx) = tokio::sync::watch::channel(initial);
        tokio::spawn(async move {
            let mut events = match docker.get_stream("/events").await {
                Ok(events) => events,
                Err(_error) => {
                    return;
                }
            };
            let mut heartbeat = tokio::time::interval(Duration::from_secs(15));
            heartbeat.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
            let mut buffer = Vec::new();

            loop {
                tokio::select! {
                    _ = heartbeat.tick() => {
                        if let Ok(snapshot) = state::StateService::current(&docker).await {
                            if tx.send(snapshot).is_err() {
                                return;
                            }
                        }
                    }
                    chunk = events.next() => match chunk {
                        Some(Ok(bytes)) => {
                            buffer.extend_from_slice(&bytes);
                            let mut changed = false;
                            while let Some(position) = buffer.iter().position(|byte| *byte == b'\n') {
                                buffer.drain(..=position);
                                changed = true;
                            }
                            if changed {
                                // Coalesce a burst of Docker events into one latest-state read.
                                tokio::time::sleep(Duration::from_millis(25)).await;
                                if let Ok(snapshot) = state::StateService::current(&docker).await {
                                    if tx.send(snapshot).is_err() {
                                        return;
                                    }
                                }
                            }
                        }
                        Some(Err(_)) | None => break,
                    }
                }
            }
        });

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::WatchStream::new(rx).map(Ok),
        )))
    }
}
