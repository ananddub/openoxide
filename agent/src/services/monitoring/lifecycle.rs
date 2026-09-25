use super::hub::EventsHub;
use super::state;
use crate::api::proto::{ContainerStatesResponse, WatchRequest};
use crate::docker::api::DockerApi;
use std::{pin::Pin, sync::Arc, time::Duration};
use tokio_stream::{Stream, StreamExt};
use tonic::{Response, Status};

pub type LifecycleStream =
    Pin<Box<dyn Stream<Item = Result<ContainerStatesResponse, Status>> + Send + 'static>>;

pub struct LifecycleService;

impl LifecycleService {
    /// Every watcher subscribes to the agent's single shared `/events` feed;
    /// any event line (or the 15 s heartbeat) refreshes this watcher's latest
    /// state snapshot through its watch channel, so only the newest state is
    /// ever delivered.
    pub async fn watch(
        docker: Arc<DockerApi>,
        hub: EventsHub,
        request: WatchRequest,
    ) -> Result<Response<LifecycleStream>, Status> {
        if request.server_id <= 0 {
            return Err(Status::invalid_argument("server_id is required"));
        }
        let initial = state::StateService::current(&docker).await?;
        let (tx, rx) = tokio::sync::watch::channel(initial);
        let mut events = hub.subscribe().await;
        tokio::spawn(async move {
            let mut heartbeat = tokio::time::interval(Duration::from_secs(15));
            heartbeat.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
            let mut changed = false;

            loop {
                tokio::select! {
                    _ = heartbeat.tick() => {
                        if let Ok(snapshot) = state::StateService::current(&docker).await
                            && tx.send(snapshot).is_err()
                        {
                            return;
                        }
                    }
                    event = events.recv() => match event {
                        Ok(_line) => changed = true,
                        // Hub restarted: resubscribe and keep watching.
                        Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
                        Err(_) => {
                            events = hub.subscribe().await;
                            continue;
                        }
                    }
                }
                if changed {
                    // Coalesce a burst of Docker events into one latest-state read.
                    tokio::time::sleep(Duration::from_millis(25)).await;
                    changed = false;
                    if let Ok(snapshot) = state::StateService::current(&docker).await
                        && tx.send(snapshot).is_err()
                    {
                        return;
                    }
                }
            }
        });

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::WatchStream::new(rx).map(Ok),
        )))
    }
}
