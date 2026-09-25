use std::sync::Arc;

use auto_di::singleton;
use bytes::Bytes;
use serde::Deserialize;
use tokio::sync::{Mutex, broadcast};
use tokio_stream::StreamExt;

use crate::docker::api::DockerApi;
use crate::docker::json_lines::JsonAccumulator;
use crate::docker::types::{ContainerStats, ContainerSummary};

/// Raw stats document: CLI-relevant extras (`name`, `id`, pids) layered over
/// the metric fields `ContainerStats` already models.
#[derive(Debug, Deserialize)]
pub struct RawStats {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "pids_stats", default)]
    pub pids: PidsStats,
    #[serde(flatten)]
    pub stats: ContainerStats,
}

#[derive(Debug, Default, Deserialize)]
pub struct PidsStats {
    #[serde(default)]
    pub current: u64,
}

const BROADCAST_CAPACITY: usize = 256;

/// One shared stats feed for the whole agent: a per-container `/stats`
/// stream is opened only while listeners exist and every document is
/// broadcast to all of them, so the number of panel consumers never
/// multiplies Docker connections. Feeds are torn down when the last
/// listener goes away.
#[derive(Clone)]
pub struct StatsHub {
    docker: Arc<DockerApi>,
    state: Arc<Mutex<HubState<Arc<RawStats>>>>,
    sizes: Arc<Mutex<std::collections::HashMap<String, u64>>>,
}

/// One `/events` connection for the whole agent, broadcast as raw JSON lines.
#[derive(Clone)]
pub struct EventsHub {
    docker: Arc<DockerApi>,
    state: Arc<Mutex<HubState<Bytes>>>,
}

struct HubState<T> {
    tx: Option<broadcast::Sender<T>>,
    /// Bumped on every pump start; a finished pump only clears the slot if
    /// it is still the current generation.
    generation: u64,
}

impl<T> Default for HubState<T> {
    fn default() -> Self {
        Self {
            tx: None,
            generation: 0,
        }
    }
}

#[singleton]
impl StatsHub {
    pub fn new(docker: Arc<DockerApi>) -> Self {
        Self {
            docker,
            state: Arc::new(Mutex::new(HubState::default())),
            sizes: Arc::new(Mutex::new(std::collections::HashMap::new())),
        }
    }
}

#[singleton]
impl EventsHub {
    pub fn new(docker: Arc<DockerApi>) -> Self {
        Self {
            docker,
            state: Arc::new(Mutex::new(HubState::default())),
        }
    }
}

impl StatsHub {
    /// Subscribe to the shared stats feed, starting the single pump on first
    /// use. Documents carry their own `id`/`name`; callers filter.
    pub async fn subscribe(&self) -> broadcast::Receiver<Arc<RawStats>> {
        let mut state = self.state.lock().await;
        if let Some(tx) = &state.tx {
            return tx.subscribe();
        }
        let (tx, rx) = broadcast::channel(BROADCAST_CAPACITY);
        state.generation += 1;
        let generation = state.generation;
        state.tx = Some(tx.clone());
        let shared_tx = tx;
        let shared_state = self.state.clone();
        let shared_sizes = self.sizes.clone();
        let docker = self.docker.clone();
        tokio::spawn(async move {
            pump_stats(&docker, &shared_tx, &shared_sizes).await;
            // The pump never touches the state lock, so clearing here cannot
            // race with a start; the generation gate protects a replacement.
            let mut state = shared_state.lock().await;
            if state.generation == generation {
                state.tx = None;
            }
        });
        rx
    }

    /// Writable-layer size for a container id (full or 12-char short). The
    /// stats API never carries `size_rw`, so the pump fetches it once per
    /// stream open with one inspect round.
    pub async fn size_rw(&self, doc_id: &str) -> u64 {
        let sizes = self.sizes.lock().await;
        sizes
            .iter()
            .find(|(id, _)| {
                doc_id == id.as_str()
                    || doc_id.starts_with(id.as_str())
                    || id.starts_with(doc_id.get(..12).unwrap_or(doc_id))
            })
            .map(|(_, size)| *size)
            .unwrap_or(0)
    }
}

impl EventsHub {
    /// Subscribe to raw Docker event lines (one JSON document per line),
    /// starting the single events connection on first use.
    pub async fn subscribe(&self) -> broadcast::Receiver<Bytes> {
        let mut state = self.state.lock().await;
        if let Some(tx) = &state.tx {
            return tx.subscribe();
        }
        let (tx, rx) = broadcast::channel(BROADCAST_CAPACITY);
        state.generation += 1;
        let generation = state.generation;
        state.tx = Some(tx.clone());
        let shared_tx = tx;
        let shared_state = self.state.clone();
        let docker = self.docker.clone();
        tokio::spawn(async move {
            pump_events(&docker, &shared_tx).await;
            let mut state = shared_state.lock().await;
            if state.generation == generation {
                state.tx = None;
            }
        });
        rx
    }
}

async fn pump_stats(
    docker: &DockerApi,
    tx: &broadcast::Sender<Arc<RawStats>>,
    sizes: &Arc<Mutex<std::collections::HashMap<String, u64>>>,
) {
    let ids: Vec<String> = match docker
        .get_json::<Vec<ContainerSummary>>("/containers/json")
        .await
    {
        Ok(containers) => containers
            .into_iter()
            .map(|container| container.id.to_string())
            .collect(),
        Err(error) => {
            tracing::warn!(%error, "stats hub: could not list containers");
            return;
        }
    };
    if ids.is_empty() {
        return;
    }
    // One inspect round at stream open fills the SizeRw map; values drift
    // slowly and failed lookups just report 0.
    let inspected: Vec<(String, u64)> = futures::future::join_all(ids.iter().map(|id| {
        let docker = docker.clone();
        async move {
            let size_rw = docker
                .get_json::<serde_json::Value>(&format!("/containers/{id}/json?size=1"))
                .await
                .ok()
                .and_then(|doc| doc.get("SizeRw").and_then(|v| v.as_u64()))
                .unwrap_or(0);
            (id.clone(), size_rw)
        }
    }))
    .await
    .into_iter()
    .collect();
    *sizes.lock().await = inspected.into_iter().collect();
    // One `/stats?stream=true` feed per running container — joined-id
    // multiplexing was removed from Docker 29. The feeds fan into this one
    // broadcast, so listener count never adds Docker connections.
    let feeds: Vec<tokio::task::JoinHandle<()>> = ids
        .iter()
        .map(|id| {
            let docker = docker.clone();
            let tx = tx.clone();
            let path = format!("/containers/{id}/stats?stream=true");
            tokio::spawn(async move {
                let mut chunks = match docker.get_stream(&path).await {
                    Ok(chunks) => chunks,
                    Err(error) => {
                        tracing::debug!(%error, "stats hub: could not open container feed");
                        return;
                    }
                };
                let mut accumulator = JsonAccumulator::new();
                while let Some(chunk) = chunks.next().await {
                    match chunk {
                        Ok(chunk) => {
                            for doc in accumulator.push::<RawStats>(&chunk) {
                                let _ = tx.send(Arc::new(doc));
                            }
                        }
                        Err(error) => {
                            tracing::debug!(%error, "stats hub: container feed error");
                            return;
                        }
                    }
                }
            })
        })
        .collect();
    // Tear every feed down once the last listener is gone.
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        if tx.receiver_count() == 0 || feeds.iter().all(|feed| feed.is_finished()) {
            break;
        }
    }
    for feed in feeds {
        feed.abort();
    }
}

async fn pump_events(docker: &DockerApi, tx: &broadcast::Sender<Bytes>) {
    let mut lines = match docker.get_stream("/events?filters=%7B%7D").await {
        Ok(lines) => lines,
        Err(error) => {
            tracing::warn!(%error, "events hub: could not open /events");
            return;
        }
    };
    let mut buffer: Vec<u8> = Vec::new();
    while let Some(chunk) = lines.next().await {
        match chunk {
            Ok(chunk) => {
                buffer.extend_from_slice(&chunk);
                while let Some(position) = buffer.iter().position(|byte| *byte == b'\n') {
                    let line = Bytes::from(buffer.drain(..=position).collect::<Vec<u8>>());
                    if tx.receiver_count() == 0 {
                        return;
                    }
                    let _ = tx.send(line);
                }
                // A daemon without newline framing must not grow the buffer
                // without bound; past the cap, wait for a line boundary.
                if buffer.len() > 1024 * 1024 {
                    tracing::debug!("events hub: oversized line, resetting buffer");
                    buffer.clear();
                }
            }
            Err(error) => {
                tracing::debug!(%error, "events hub: stream error");
                return;
            }
        }
    }
}
