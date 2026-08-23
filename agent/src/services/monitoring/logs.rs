use crate::api::proto::{LogChunk, LogStreamRequest};
use crate::{docker::api::DockerApi, docker::types::ContainerId, logs::ContainerLogs};
use std::pin::Pin;
use tokio_stream::Stream;
use tonic::{Response, Status};
use tracing::debug;

const DEFAULT_TAIL_LINES: usize = 100;

pub type LogStream = Pin<Box<dyn Stream<Item = Result<LogChunk, Status>> + Send + 'static>>;

pub struct LogsService;

impl LogsService {
    pub fn stream(docker: DockerApi, request: LogStreamRequest) -> Response<LogStream> {
        let tail_lines = if request.tail_lines > 0 {
            request.tail_lines as usize
        } else {
            DEFAULT_TAIL_LINES
        };
        debug!(container = %request.container_id, tail_lines, "serving log tail");

        let (tx, rx) = tokio::sync::mpsc::channel(100);
        let container_id = request.container_id;
        tokio::spawn(async move {
            let timestamp = chrono::Utc::now().timestamp();
            let typed_id = ContainerId::new(&container_id);
            for line in ContainerLogs::tail_container_logs(&docker, &typed_id, tail_lines).await {
                let chunk = LogChunk {
                    container_id: container_id.clone(),
                    log_line: line,
                    timestamp,
                    is_stderr: false,
                };
                if tx.send(Ok(chunk)).await.is_err() {
                    break;
                }
            }
        });
        Response::new(Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }
}
