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
    /// watcher — no shared feed, no timers, no background task. The watcher
    /// yields the live state on request, then re-reads it fresh whenever the
    /// Docker daemon signals any event; event payloads themselves are
    /// discarded unread. If the panel drops the stream the connection
    /// closes with it, and nothing survives the call.
    pub async fn watch(
        docker: Arc<DockerApi>,
        request: WatchRequest,
    ) -> Result<Response<LifecycleStream>, Status> {
        if request.server_id <= 0 {
            return Err(Status::invalid_argument("server_id is required"));
        }
        let initial = state::StateService::current(&docker).await?;
        let events = docker
            .get_stream("/events?filters=%7B%7D")
            .await
            .map_err(|error| Status::unavailable(error.to_string()))?;

        let stream = async_stream::stream! {
            yield Ok(initial);
            let mut events = events;
            loop {
                match events.next().await {
                    Some(Ok(_payload)) => {
                        if let Ok(snapshot) = state::StateService::current(&docker).await {
                            yield Ok(snapshot);
                        }
                    }
                    // Stream ended or errored: the watcher ends with it.
                    _ => return,
                }
            }
        };
        Ok(Response::new(Box::pin(stream)))
    }
}
