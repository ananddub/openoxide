use std::io;
use tokio::{
    fs::{OpenOptions, create_dir_all},
    io::AsyncWriteExt,
};

pub async fn read(key: &str) -> io::Result<String> {
    match tokio::fs::read_to_string(
        crate::utils::paths::openoxide_paths().schedule_log_file(&sanitize(key)),
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
    let paths = crate::utils::paths::openoxide_paths();
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
