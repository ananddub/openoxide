use crate::utils::builder::spec::BuilderEvent;
use crate::utils::paths::openoxide_paths;
use std::io;
use tokio::fs::{File, OpenOptions, create_dir_all};
use tokio::io::AsyncWriteExt;

tokio::task_local! {
    pub static DEPLOYMENT_SENDER: tokio::sync::mpsc::Sender<crate::utils::builder::spec::BuilderEvent>;
}

pub fn log_message(msg: String) {
    let _ = DEPLOYMENT_SENDER.try_with(|tx| {
        let event = crate::utils::builder::spec::BuilderEvent::Message(msg);
        if let Err(tokio::sync::mpsc::error::TrySendError::Full(event)) = tx.try_send(event) {
            let tx = tx.clone();
            tokio::spawn(async move {
                let _ = tx.send(event).await;
            });
        }
    });
}

pub struct DeploymentLog {
    file: File,
    path: String,
}

impl DeploymentLog {
    /// Opens or creates the deployment log file: {base}/logs/deployments/{deployment_id}.log
    pub async fn open(deployment_id: i64) -> io::Result<Self> {
        let paths = openoxide_paths();
        let dir = paths.deployment_logs();
        create_dir_all(&dir).await?;
        let path = paths.deployment_log_file(deployment_id);
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .await?;
        Ok(Self { file, path })
    }

    /// Writes a single line to the log file with a UTC timestamp.
    pub async fn write_line(&mut self, line: &str) -> io::Result<()> {
        let ts = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ");
        let formatted = format!("[{ts}] {line}\n");
        self.file.write_all(formatted.as_bytes()).await?;
        self.file.flush().await
    }

    /// Formats and writes a BuilderEvent to the log file.
    pub async fn write_event(&mut self, event: &BuilderEvent) -> io::Result<()> {
        match event {
            BuilderEvent::Message(msg) => {
                for line in msg.lines() {
                    self.write_line(line).await?;
                }
            }
            BuilderEvent::Failed(err) => {
                self.write_line(&format!("[ERROR] {err}")).await?;
            }
            other => {
                let state = super::common::builder_event_state(other);
                self.write_line(&format!("[{state}]")).await?;
            }
        }
        Ok(())
    }

    /// Returns the path to the log file.
    pub fn path(&self) -> &str {
        &self.path
    }
}

pub async fn record_builder_events(
    deployment_id: i64,
    mut events: tokio::sync::mpsc::Receiver<BuilderEvent>,
    log_context: &'static str,
) {
    let mut log = match DeploymentLog::open(deployment_id).await {
        Ok(log) => {
            if let Ok(repo) = auto_di::resolve::<crate::repository::DeploymentRepository>().await {
                repo.update_log_path(deployment_id, log.path()).await.ok();
            }
            Some(log)
        }
        Err(e) => {
            tracing::error!(deployment_id, error = %e, "could not open deployment log file");
            None
        }
    };

    let repo = match auto_di::resolve::<crate::repository::DeploymentRepository>().await {
        Ok(r) => Some(r),
        Err(e) => {
            tracing::error!(error = %e, "could not resolve DeploymentRepository");
            None
        }
    };

    while let Some(event) = events.recv().await {
        if let Some(ref mut l) = log {
            if let Err(e) = l.write_event(&event).await {
                tracing::error!(deployment_id, error = %e, "could not write to deployment log");
            }
        }

        if let BuilderEvent::Message(message) = &event {
            tracing::info!(deployment_id, message = %message, "{} deployment message", log_context);
            continue;
        }
        let state = super::common::builder_event_state(&event);
        let message = match &event {
            BuilderEvent::Failed(e) => Some(e.as_str()),
            _ => None,
        };

        if let Some(ref r) = repo {
            if let Err(e) = r
                .update_state_if_running(deployment_id, state, message)
                .await
            {
                tracing::error!(deployment_id, error = %e, "could not persist {} builder event", log_context);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_deployment_log_write_and_read() {
        let workspace_tmp = std::path::Path::new("target/test-deployments-9999");
        let _ = tokio::fs::remove_dir_all(&workspace_tmp).await;
        tokio::fs::create_dir_all(&workspace_tmp).await.unwrap();

        unsafe {
            env::set_var("OPENOXIDE_BASE_PATH", workspace_tmp.to_str().unwrap());
        }

        let deployment_id = 9999;
        let mut log = DeploymentLog::open(deployment_id).await.unwrap();

        let expected_path = format!(
            "{}/logs/deployments/9999.log",
            workspace_tmp.to_str().unwrap()
        );
        assert_eq!(log.path(), expected_path);

        log.write_event(&BuilderEvent::Preparing).await.unwrap();
        log.write_event(&BuilderEvent::Message("Hello World\nLine 2".into()))
            .await
            .unwrap();
        log.write_event(&BuilderEvent::Failed("Some build error".into()))
            .await
            .unwrap();

        let content = tokio::fs::read_to_string(expected_path).await.unwrap();
        assert!(content.contains("[PREPARING]"));
        assert!(content.contains("Hello World"));
        assert!(content.contains("Line 2"));
        assert!(content.contains("[ERROR] Some build error"));

        unsafe {
            env::remove_var("OPENOXIDE_BASE_PATH");
        }
        let _ = tokio::fs::remove_dir_all(&workspace_tmp).await;
    }

    #[tokio::test]
    async fn test_deployment_sender_task_local() {
        let (tx, mut rx) = tokio::sync::mpsc::channel(10);
        let future = async move {
            log_message("Capturing this line".to_string());
        };
        DEPLOYMENT_SENDER.scope(tx, future).await;

        let received = rx.recv().await.unwrap();
        match received {
            BuilderEvent::Message(msg) => assert_eq!(msg, "Capturing this line"),
            _ => panic!("Expected Message event"),
        }
    }
}
