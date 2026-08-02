use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::broadcast;
use tokio_stream::StreamExt;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

use super::api::DockerApi;
use super::json_lines::JsonAccumulator;
use super::stats::ContainerStats;
use crate::filter::ContainerFilter;

/// A single container's latest stats, pushed to subscribers.
#[derive(Clone, Debug)]
pub struct ContainerSample {
    pub container_id: String,
    pub name: String,
    pub stats: Arc<ContainerStats>,
}

/// How often the set of running containers is re-checked, so containers that
/// start or stop are picked up within this window.
const RECONCILE_INTERVAL: Duration = Duration::from_secs(30);
/// If a stats stream produces nothing for this long, it is presumed dead and
/// reconnected. The daemon pushes a frame per second, so this is generous.
const STREAM_TIMEOUT: Duration = Duration::from_secs(300);
/// Pause before retrying a stream after it dies. The daemon re-pushes the full
/// connection state on reconnect, so no backfill is needed.
const RECONNECT_DELAY: Duration = Duration::from_secs(3);

/// Streams per-container stats from the docker daemon and fans them out over a
/// broadcast channel.
///
/// Every running container gets a persistent `GET /containers/{id}/stats?stream=true`
/// connection that pushes one JSON document per second. `run` reconciles the
/// container set on an interval and the individual stream tasks handle their
/// own reconnection and timeout, so a crash of the daemon or a network blip
/// recovers without any coordination.
pub struct ContainerStreamer {
    docker: DockerApi,
    broadcast: broadcast::Sender<ContainerSample>,
    /// Narrows which containers get a stream. At density, opening a connection
    /// per container is the cost that matters, so excluded containers must be
    /// skipped before a stream is spawned rather than filtered downstream.
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
        // container id -> token used to cancel its stream task.
        let mut streams: HashMap<String, CancellationToken> = HashMap::new();

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
        streams: &mut HashMap<String, CancellationToken>,
        shutdown: &CancellationToken,
    ) {
        let containers: Vec<(String, String)> = match self
            .docker
            .get_json::<Vec<super::stats::ContainerSummary>>("/containers/json")
            .await
        {
            Ok(list) => list
                .into_iter()
                .filter_map(|c| {
                    let name = c.display_name();
                    // Skip before spawning: an unwanted stream costs a
                    // connection and daemon work, not just a wasted row.
                    if !self.filter.should_monitor(&name) {
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

        // Containers that stopped get their streams cancelled.
        let gone: Vec<String> = streams
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
        id: String,
        name: String,
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

/// Streams one container's stats until the connection ends or times out.
///
/// Returns Ok when the stream ended cleanly, Err when it failed with an error.
/// Either way the caller reconnects; the daemon re-pushes the connection state
/// on the new connection.
async fn stream_one(
    docker: &DockerApi,
    broadcast: &broadcast::Sender<ContainerSample>,
    id: &str,
    name: &str,
) -> Result<(), String> {
    let path = format!("/containers/{id}/stats?stream=true");
    let chunks = docker.get_stream(&path).await?;

    let mut accumulator = JsonAccumulator::new();

    let outcome = tokio::time::timeout(STREAM_TIMEOUT, async {
        let mut chunks = chunks;
        while let Some(chunk) = chunks.next().await {
            let chunk = chunk?;

            // A document cut short by a dropped connection can never complete,
            // so anything still buffered when the stream breaks is discarded by
            // the caller creating a fresh accumulator on reconnect.
            for stats in accumulator.push::<ContainerStats>(&chunk) {
                let sample = ContainerSample {
                    container_id: id.to_string(),
                    name: name.to_string(),
                    stats: Arc::new(stats),
                };

                // A slow subscriber is dropped by the broadcast channel rather
                // than stalling this stream.
                let _ = broadcast.send(sample);
            }
        }
        Ok::<_, String>(())
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
