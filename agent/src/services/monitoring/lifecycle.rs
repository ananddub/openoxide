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
                    control = controls.message() => match control {
                        Ok(Some(_)) => {}
                        _ => break,
                    },
                    _ = heartbeat.tick() => {
                        if let Ok(snapshot) = state::StateService::current(&docker).await { tx.send_replace(snapshot); }
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
                                    tx.send_replace(snapshot);
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
