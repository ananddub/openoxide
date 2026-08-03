use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::broadcast;
use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

use super::api::DockerApi;
use super::json_lines::JsonAccumulator;
use super::types::{ContainerId, ContainerName, ContainerStats, ContainerSummary};
pub use super::types::ContainerSample;
use crate::error::DockerError;
use crate::filter::ContainerFilter;

const RECONCILE_INTERVAL: Duration = Duration::from_secs(30);
const STREAM_TIMEOUT: Duration = Duration::from_secs(300);
const RECONNECT_DELAY: Duration = Duration::from_secs(3);

pub struct ContainerStreamer {
    docker: DockerApi,
    broadcast: broadcast::Sender<ContainerSample>,
    filter: ContainerFilter,
}

impl ContainerStreamer {
    pub fn new(
        docker: DockerApi,
        filter: ContainerFilter,
    ) -> (Self, broadcast::Receiver<ContainerSample>) {
        let (broadcast, rx) = broadcast::channel(256);
        (
            Self {
                docker,
                broadcast,
                filter,
            },
            rx,
        )
    }

    pub async fn run(&mut self, shutdown: CancellationToken) {
        let mut streams: HashMap<ContainerId, CancellationToken> = HashMap::new();

        let mut reconcile = tokio::time::interval(RECONCILE_INTERVAL);
        reconcile.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

        loop {
            tokio::select! {
                _ = shutdown.cancelled() => {
                    info!("container streamer stopped");
                    return;
                }
                _ = reconcile.tick() => {
                    self.reconcile(&mut streams, &shutdown).await;
                }
            }
        }
    }

    async fn reconcile(
        &self,
        streams: &mut HashMap<ContainerId, CancellationToken>,
        shutdown: &CancellationToken,
    ) {
        let containers: Vec<(ContainerId, ContainerName)> = match self
            .docker
            .get_json::<Vec<ContainerSummary>>("/containers/json")
            .await
        {
            Ok(list) => list
                .into_iter()
                .filter_map(|c| {
                    let name = c.display_name();
                    if !self.filter.should_monitor(name.as_str()) {
                        return None;
                    }
                    Some((c.id, name))
                })
                .collect(),
            Err(error) => {
                error!(%error, "could not list containers for stream reconciliation");
                return;
            }
        };

        let mut seen = std::collections::HashSet::new();
        for (id, name) in containers {
            seen.insert(id.clone());

            if !streams.contains_key(&id) {
                let cancel = CancellationToken::new();
                self.spawn_stream(id.clone(), name, cancel.clone(), shutdown.clone());
                streams.insert(id.clone(), cancel);
            }
        }

        let gone: Vec<ContainerId> = streams
            .keys()
            .filter(|id| !seen.contains(*id))
            .cloned()
            .collect();
        for id in gone {
            debug!(%id, "container gone, cancelling its stats stream");
            if let Some(cancel) = streams.remove(&id) {
                cancel.cancel();
            }
        }
    }

    fn spawn_stream(
        &self,
        id: ContainerId,
        name: ContainerName,
        cancel: CancellationToken,
        shutdown: CancellationToken,
    ) {
        let docker = self.docker.clone();
        let broadcast = self.broadcast.clone();

        tokio::spawn(async move {
            loop {
                let result = stream_one(&docker, &broadcast, &id, &name).await;

                match result {
                    Ok(()) => debug!(%id, %name, "stats stream ended, reconnecting"),
                    Err(error) => warn!(%id, %name, %error, "stats stream failed, reconnecting"),
                }

                tokio::select! {
                    _ = cancel.cancelled() => return,
                    _ = shutdown.cancelled() => return,
                    _ = tokio::time::sleep(RECONNECT_DELAY) => {}
                }
            }
        });
    }
}

async fn stream_one(
    docker: &DockerApi,
    broadcast: &broadcast::Sender<ContainerSample>,
    id: &ContainerId,
    name: &ContainerName,
) -> Result<(), DockerError> {
    let path = format!("/containers/{id}/stats?stream=true");
    let chunks = docker.get_stream(&path).await?;

    let mut accumulator = JsonAccumulator::new();

    let outcome = tokio::time::timeout(STREAM_TIMEOUT, async {
        let mut chunks = chunks;
        while let Some(chunk) = chunks.next().await {
            let chunk = chunk?;

            for stats in accumulator.push::<ContainerStats>(&chunk) {
                let sample = ContainerSample {
                    container_id: id.clone(),
                    name: name.clone(),
                    stats: Arc::new(stats),
                };

                let _ = broadcast.send(sample);
            }
        }
        Ok::<_, DockerError>(())
    })
    .await;

    match outcome {
        Ok(Ok(())) => Ok(()),
        Ok(Err(error)) => Err(error),
        Err(_) => {
            debug!(%id, "stats stream produced nothing for {STREAM_TIMEOUT:?}, reconnecting");
            Ok(())
        }
    }
}
