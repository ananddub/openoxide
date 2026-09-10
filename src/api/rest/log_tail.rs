use std::time::{Duration, Instant};

use bytes::Bytes;
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

use crate::db::queries::FlashQueries;

/// How often the tail polls the log file for appended bytes. The builder
/// flushes every line, so this only bounds notification latency, not
/// delivery reliability.
const TAIL_POLL: Duration = Duration::from_millis(400);

/// Hard stop so a forgotten open modal doesn't tail forever. Completed
/// deployments end the stream on their own; this cap only covers the
/// pathological case (deployment row unreadable, queries unavailable).
const MAX_TAIL: Duration = Duration::from_secs(15 * 60);

/// Live tail of a deployment log file served in SSE framing.
///
/// Streams the file as it grows: the full existing content first, then
/// appended lines as the builder writes them. The stream ends once the
/// deployment row reaches a terminal state (`finished_at` set) or the row
/// disappears (history cleared) — the client sees a normal stream end and
/// can reopen later.
pub struct DeploymentLogTail;

impl DeploymentLogTail {
    /// Spawns the tail task; the returned stream yields SSE frames
    /// (`data: <line>\n\n`), one per log line.
    pub fn stream(path: String, deployment_id: i64) -> ReceiverStream<Bytes> {
        let (tx, rx) = mpsc::channel::<Bytes>(64);
        tokio::spawn(Self::run(path, deployment_id, tx));
        ReceiverStream::new(rx)
    }

    async fn run(path: String, deployment_id: i64, tx: mpsc::Sender<Bytes>) {
        let deadline = Instant::now() + MAX_TAIL;
        let queries = auto_di::resolve::<FlashQueries>().await.ok();

        let mut file: Option<tokio::fs::File> = None;
        let mut offset: u64 = 0;
        let mut partial = String::new();

        loop {
            // A queued deployment may not have started writing yet — keep
            // polling until the file appears.
            if file.is_none() {
                file = tokio::fs::File::open(&path).await.ok();
            }

            if let Some(handle) = file.as_mut() {
                match handle.metadata().await {
                    Ok(meta) => {
                        let len = meta.len();
                        if len < offset {
                            // File truncated/rewritten: start over.
                            offset = 0;
                            partial.clear();
                        }
                        if len > offset
                            && handle.seek(std::io::SeekFrom::Start(offset)).await.is_ok()
                        {
                            let mut buf = vec![0u8; (len - offset) as usize];
                            if handle.read_exact(&mut buf).await.is_ok() {
                                offset = len;
                                partial.push_str(&String::from_utf8_lossy(&buf));
                                let sse = Self::sse_frames(&mut partial);
                                if !sse.is_empty() && tx.send(Bytes::from(sse)).await.is_err() {
                                    return;
                                }
                            }
                        }
                    }
                    Err(_) => file = None,
                }
            }

            // Terminal deployment (or deleted row) ends the tail.
            match queries.as_ref().map(|q| q.deployments_get_by_id(deployment_id)) {
                Some(fut) => match fut.await {
                    Ok(d) if d.finished_at.is_some_and(|f| f > 0) => return,
                    Ok(_) => {}
                    Err(_) => return,
                },
                None => {}
            }

            if Instant::now() > deadline {
                return;
            }
            tokio::time::sleep(TAIL_POLL).await;
        }
    }

    /// Splits accumulated text into SSE frames, keeping the trailing
    /// partial line (no newline yet) for the next poll.
    fn sse_frames(text: &mut String) -> String {
        let mut lines = text.split('\n');
        let last = lines.next_back().unwrap_or("").to_string();
        let mut sse = String::new();
        for line in lines {
            sse.push_str("data: ");
            sse.push_str(line);
            sse.push_str("\n\n");
        }
        *text = last;
        sse
    }
}
