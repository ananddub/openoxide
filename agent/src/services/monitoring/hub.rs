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
            pump_stats(&docker, &shared_tx, &shared_sizes, &shared_state, generation).await;
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

/// How often the pump reconciles its live feed set against the containers
/// Docker actually reports running. Container churn (deploys, restarts) is
/// picked up here without the whole pump being torn down.
const RECONCILE_INTERVAL: std::time::Duration = std::time::Duration::from_secs(5);

/// Opens one container's `/stats` feed into the broadcast; returns when it ends.
fn spawn_container_feed(
    docker: DockerApi,
    tx: broadcast::Sender<Arc<RawStats>>,
    id: String,
) -> tokio::task::JoinHandle<()> {
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
        // Dropping `chunks` releases the dedicated Docker connection.
    })
}

/// Inspects a container's writable-layer size (stats API omits `size_rw`).
async fn inspect_size_rw(docker: &DockerApi, id: &str) -> u64 {
    docker
        .get_json::<serde_json::Value>(&format!("/containers/{id}/json?size=1"))
        .await
        .ok()
        .and_then(|doc| doc.get("SizeRw").and_then(|v| v.as_u64()))
        .unwrap_or(0)
}

async fn pump_stats(
    docker: &DockerApi,
    tx: &broadcast::Sender<Arc<RawStats>>,
    sizes: &Arc<Mutex<std::collections::HashMap<String, u64>>>,
    state: &Arc<Mutex<HubState<Arc<RawStats>>>>,
    generation: u64,
) {
    // One `/stats?stream=true` feed per running container, fanned into this
    // broadcast. The feed set is reconciled against the live container list
    // each tick, so churn never leaks a feed and new containers are picked up.
    let mut feeds: std::collections::HashMap<String, tokio::task::JoinHandle<()>> =
        std::collections::HashMap::new();

    loop {
        // Exit when the last listener leaves. Checked under the subscriber's
        // lock so a concurrent subscriber can't attach to an exiting pump.
        if tx.receiver_count() == 0 {
            let mut guard = state.lock().await;
            if tx.receiver_count() == 0 {
                if guard.generation == generation {
                    guard.tx = None;
                }
                drop(guard);
                break;
            }
        }

        let live_ids: Vec<String> = match docker
            .get_json::<Vec<ContainerSummary>>("/containers/json")
            .await
        {
            Ok(containers) => containers
                .into_iter()
                .map(|container| container.id.to_string())
                .collect(),
            Err(error) => {
                tracing::debug!(%error, "stats hub: could not list containers");
                tokio::time::sleep(RECONCILE_INTERVAL).await;
                continue;
            }
        };
        let live: std::collections::HashSet<&str> =
            live_ids.iter().map(String::as_str).collect();

        // Reap feeds for gone/finished containers, releasing their connections.
        feeds.retain(|id, handle| {
            let keep = live.contains(id.as_str()) && !handle.is_finished();
            if !keep {
                handle.abort();
            }
            keep
        });

        // Open feeds for new containers, inspecting each one's size once.
        let new_ids: Vec<String> = live_ids
            .iter()
            .filter(|id| !feeds.contains_key(id.as_str()))
            .cloned()
            .collect();
        if !new_ids.is_empty() {
            let inspected: Vec<(String, u64)> =
                futures::future::join_all(new_ids.iter().map(|id| {
                    let docker = docker.clone();
                    async move { (id.clone(), inspect_size_rw(&docker, id).await) }
                }))
                .await;
            {
                let mut sizes = sizes.lock().await;
                for (id, size) in inspected {
                    sizes.insert(id, size);
                }
            }
            for id in new_ids {
                let handle = spawn_container_feed(docker.clone(), tx.clone(), id.clone());
                feeds.insert(id, handle);
            }
        }

        // Bound the size cache to the live set.
        {
            let mut sizes = sizes.lock().await;
            sizes.retain(|id, _| live.contains(id.as_str()));
        }

        tokio::time::sleep(RECONCILE_INTERVAL).await;
    }

    for (_, handle) in feeds {
        handle.abort();
    }
    sizes.lock().await.clear();
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
