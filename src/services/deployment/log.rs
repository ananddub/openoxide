use crate::services::deployment::DeploymentService;
use tokio::sync::mpsc;

impl DeploymentService {
    pub async fn read_deployment_log(&self, deployment_id: i64) -> sqlx::Result<String> {
        let deployment = self.get_by_id(deployment_id).await?;
        tokio::fs::read_to_string(&deployment.log_path)
            .await
            .map_err(|e| sqlx::Error::Protocol(format!("could not read log file: {e}")))
    }

    pub async fn stream_deployment_log(
        &self,
        deployment_id: i64,
    ) -> sqlx::Result<tokio::sync::mpsc::Receiver<String>> {
        let deployment = self.get_by_id(deployment_id).await?;
        let log_path_str = deployment.log_path.clone();

        let (sender, receiver) = mpsc::channel(100);
        let repo_deploy = self.repo_deploy.clone();

        tokio::spawn(async move {
            let log_path = std::path::Path::new(&log_path_str);

            for _ in 0..10 {
                if log_path.exists() {
                    break;
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
            }

            if !log_path.exists() {
                let _ = sender
                    .send(format!("Log file not found: {}", log_path_str))
                    .await;
                return;
            }

            let mut file = match tokio::fs::File::open(log_path).await {
                Ok(f) => f,
                Err(e) => {
                    let _ = sender.send(format!("Failed to open log file: {}", e)).await;
                    return;
                }
            };

            let mut pos = 0u64;
            let mut buffer = Vec::new();

            if let Ok(meta) = file.metadata().await {
                let len = meta.len();
                if len > 0 {
                    let mut chunk = vec![0u8; len as usize];
                    use tokio::io::AsyncReadExt;
                    if file.read_exact(&mut chunk).await.is_ok() {
                        buffer.extend_from_slice(&chunk);
                        pos = len;
                        send_lines(&mut buffer, &sender).await;
                    }
                }
            }

            let (event_tx, mut event_rx) = mpsc::channel(10);
            let watcher = {
                use notify::{RecommendedWatcher, RecursiveMode, Watcher};
                let watcher_res = RecommendedWatcher::new(
                    move |res: Result<notify::Event, notify::Error>| {
                        if let Ok(event) = res {
                            if event.kind.is_modify() {
                                let _ = event_tx.blocking_send(());
                            }
                        }
                    },
                    notify::Config::default(),
                );

                match watcher_res {
                    Ok(mut w) => {
                        if w.watch(log_path, RecursiveMode::NonRecursive).is_err() {
                            None
                        } else {
                            Some(w)
                        }
                    }
                    Err(_) => None,
                }
            };

            loop {
                let status = repo_deploy
                    .get_status(deployment_id)
                    .await
                    .unwrap_or_default()
                    .unwrap_or_default();

                let is_finished = status != "RUNNING" && status != "QUEUED";

                if let Ok(meta) = tokio::fs::metadata(log_path).await {
                    let len = meta.len();
                    if len > pos {
                        use tokio::io::{AsyncReadExt, AsyncSeekExt};
                        let _ = file.seek(std::io::SeekFrom::Start(pos)).await;
                        let mut chunk = vec![0u8; (len - pos) as usize];
                        if file.read_exact(&mut chunk).await.is_ok() {
                            buffer.extend_from_slice(&chunk);
                            pos = len;
                            send_lines(&mut buffer, &sender).await;
                        }
                    }
                }

                if is_finished {
                    break;
                }

                if watcher.is_some() {
                    let _ =
                        tokio::time::timeout(tokio::time::Duration::from_secs(2), event_rx.recv())
                            .await;
                } else {
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                }
            }

            if let Ok(meta) = tokio::fs::metadata(log_path).await {
                let len = meta.len();
                if len > pos {
                    use tokio::io::{AsyncReadExt, AsyncSeekExt};
                    let _ = file.seek(std::io::SeekFrom::Start(pos)).await;
                    let mut chunk = vec![0u8; (len - pos) as usize];
                    if file.read_exact(&mut chunk).await.is_ok() {
                        buffer.extend_from_slice(&chunk);
                        send_lines(&mut buffer, &sender).await;
                    }
                }
            }

            if !buffer.is_empty() {
                if let Ok(line) = String::from_utf8(buffer) {
                    let _ = sender.send(line).await;
                }
            }

            drop(watcher);
        });

        Ok(receiver)
    }

    pub async fn stream_application_latest_log(
        &self,
        application_id: i64,
    ) -> sqlx::Result<tokio::sync::mpsc::Receiver<String>> {
        let deployment_id = self
            .repo_deploy
            .get_latest_application_deployment_id(application_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;

        self.stream_deployment_log(deployment_id).await
    }

    pub async fn stream_compose_latest_log(
        &self,
        compose_id: i64,
    ) -> sqlx::Result<tokio::sync::mpsc::Receiver<String>> {
        let deployment_id = self
            .repo_deploy
            .get_latest_compose_deployment_id(compose_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;

        self.stream_deployment_log(deployment_id).await
    }
}

async fn send_lines(buffer: &mut Vec<u8>, sender: &tokio::sync::mpsc::Sender<String>) {
    let mut last_newline = None;
    for (i, &byte) in buffer.iter().enumerate() {
        if byte == b'\n' {
            last_newline = Some(i);
        }
    }

    if let Some(idx) = last_newline {
        let lines_part = buffer[..idx].to_vec();
        *buffer = buffer[idx + 1..].to_vec();
        if let Ok(s) = String::from_utf8(lines_part) {
            for line in s.split('\n') {
                let _ = sender.send(line.to_string()).await;
            }
        }
    }
}
