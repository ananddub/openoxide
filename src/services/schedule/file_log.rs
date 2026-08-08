use dashmap::DashMap;
use std::{
    io,
    sync::{Arc, OnceLock},
};
use tokio::sync::broadcast;
use tokio::{
    fs::{OpenOptions, create_dir_all},
    io::AsyncWriteExt,
};

static CHANNELS: OnceLock<DashMap<String, Arc<broadcast::Sender<String>>>> = OnceLock::new();

fn channels() -> &'static DashMap<String, Arc<broadcast::Sender<String>>> {
    CHANNELS.get_or_init(DashMap::new)
}

pub fn subscribe(key: &str) -> broadcast::Receiver<String> {
    let sender = channels()
        .entry(key.to_owned())
        .or_insert_with(|| Arc::new(broadcast::channel(128).0))
        .clone();
    sender.subscribe()
}

pub async fn read(key: &str) -> io::Result<String> {
    match tokio::fs::read_to_string(
        crate::utils::paths::rustploy_paths().schedule_log_file(&sanitize(key)),
    )
    .await
    {
        Ok(content) => Ok(content),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(String::new()),
        Err(error) => Err(error),
    }
}

pub async fn append(
    key: &str,
    execution_id: Option<i64>,
    status: &str,
    message: Option<&str>,
    stdout: Option<&str>,
    stderr: Option<&str>,
) -> io::Result<()> {
    let paths = crate::utils::paths::rustploy_paths();
    create_dir_all(paths.schedule_logs()).await?;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(paths.schedule_log_file(&sanitize(key)))
        .await?;
    let timestamp = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ");
    let mut entry = format!("[{timestamp}] execution={execution_id:?} status={status}\n");
    for (label, value) in [("message", message), ("stdout", stdout), ("stderr", stderr)] {
        if let Some(value) = value.filter(|value| !value.is_empty()) {
            entry.push_str(&format!("[{label}]\n{value}\n"));
        }
    }
    file.write_all(entry.as_bytes()).await?;
    file.flush().await?;
    if let Some(sender) = channels().get(key) {
        let _ = sender.send(entry);
    }
    Ok(())
}

fn sanitize(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_') {
                ch
            } else {
                '_'
            }
        })
        .collect()
}
