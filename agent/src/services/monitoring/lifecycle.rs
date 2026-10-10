use std::{pin::Pin, sync::Arc};

use tokio_stream::{Stream, StreamExt};
use tonic::{Response, Status};

use super::state;
use crate::api::proto::{ContainerStatesResponse, WatchRequest};
use crate::docker::api::DockerApi;

pub type LifecycleStream =
    Pin<Box<dyn Stream<Item = Result<ContainerStatesResponse, Status>> + Send + 'static>>;

pub struct LifecycleService;

impl LifecycleService {
    /// One dedicated `/events` connection for the life of this single
    /// watcher — no shared feed, no polling loop, no background task. The watcher
    /// yields the live state on request, then re-reads it after each Docker
    /// event burst settles (debounced, so a healthcheck storm costs one
    /// container-list refresh, not one per event; event payloads themselves
    /// are discarded unread). If the panel drops the stream the connection
    /// closes with it, and nothing survives the call.
    pub async fn watch(
        docker: Arc<DockerApi>,
        request: WatchRequest,
    ) -> Result<Response<LifecycleStream>, Status> {
        if request.server_id <= 0 {
            return Err(Status::invalid_argument("server_id is required"));
        }
        tracing::info!(server_id = request.server_id, "watch: request received");
        let initial = match state::StateService::current(&docker).await {
            Ok(snapshot) => {
                tracing::info!(
                    server_id = request.server_id,
                    containers = snapshot.containers.len(),
                    "watch: initial snapshot ready"
                );
                snapshot
            }
            Err(error) => {
                tracing::warn!(server_id = request.server_id, %error, "watch: initial snapshot failed");
                return Err(error);
            }
        };
        let events = match docker.get_stream("/events?filters=%7B%7D").await {
            Ok(events) => {
                tracing::info!(server_id = request.server_id, "watch: /events stream open");
                events
            }
            Err(error) => {
                tracing::warn!(server_id = request.server_id, %error, "watch: /events open failed");
                return Err(Status::unavailable(error.to_string()));
            }
        };

        let stream = async_stream::stream! {
            tracing::info!(server_id = request.server_id, "watch: polling — yielding initial snapshot");
            yield Ok(initial);
            let mut events = events;
            // Docker emits several events per container operation, and three
            // per healthcheck (exec_create/start/die) every interval. A full
            // `/containers/json?all=true` re-read per event turned those
            // high-frequency bursts into a dockerd fan-out storm. Coalesce
            // instead: block until something changes, drain the burst until a
            // short quiet gap (or a staleness cap) closes it, then refresh
            // once. Nothing is lost — the trailing refresh sees the final
            // state after the burst settles.
            const DEBOUNCE: std::time::Duration = std::time::Duration::from_millis(2_000);
            const MAX_WAIT: std::time::Duration = std::time::Duration::from_millis(10_000);
            // Idle keepalive refresh. The panel times a stream out if no frame
            // arrives for 30s — on a quiet host (no container ops, no
            // healthchecks) `/events` can stay silent far longer, so without
            // this tick a perfectly healthy agent was marked offline and the
            // watcher reconnected in an endless loop. 15s stays well inside
            // that budget even with jitter.
            const IDLE_REFRESH: std::time::Duration = std::time::Duration::from_secs(15);
            enum Wake {
                Event,
                EventsEnded,
                Idle,
            }
            impl Wake {
                fn is_event(&self) -> bool {
                    matches!(self, Wake::Event)
                }
            }
            loop {
                // Block until a docker event arrives or the idle tick fires.
                let wake = tokio::select! {
                    payload = events.next() => match payload {
                        Some(Ok(_payload)) => Wake::Event,
                        // Stream ended or errored: the watcher ends with it.
                        _ => Wake::EventsEnded,
                    },
                    _ = tokio::time::sleep(IDLE_REFRESH) => Wake::Idle,
                };
                let mut ended = matches!(wake, Wake::EventsEnded);
                if wake.is_event() {
                    // A burst started: drain it until a short quiet gap (or
                    // the staleness cap) closes it, then refresh once.
                    let burst_start = std::time::Instant::now();
                    loop {
                        let remaining = MAX_WAIT.saturating_sub(burst_start.elapsed());
                        if remaining.is_zero() {
                            break;
                        }
                        match tokio::time::timeout(DEBOUNCE.min(remaining), events.next()).await {
                            Ok(Some(Ok(_payload))) => continue,
                            Ok(None) | Ok(Some(Err(_))) => {
                                ended = true;
                                break;
                            }
                            // Quiet gap: the burst settled.
                            Err(_elapsed) => break,
                        }
                    }
                }
                if let Ok(snapshot) = state::StateService::current(&docker).await {
                    yield Ok(snapshot);
                }
                if ended {
                    return;
                }
            }
        };
        Ok(Response::new(Box::pin(stream)))
    }
}
