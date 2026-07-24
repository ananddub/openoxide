use std::path::Path;
use tokio::io::AsyncWriteExt;

// ── Temp file cleanup guard ───────────────────────────────────────────────────

pub struct TempFileGuard {
    paths: Vec<std::path::PathBuf>,
}

impl TempFileGuard {
    pub fn new() -> Self {
        Self { paths: Vec::new() }
    }

    pub fn track(&mut self, path: &Path) {
        self.paths.push(path.to_path_buf());
    }
}

impl Drop for TempFileGuard {
    fn drop(&mut self) {
        for path in &self.paths {
            let _ = std::fs::remove_file(path);
        }
    }
}

pub async fn save_multipart_to_file(
    mut multipart: axum::extract::Multipart,
    dest: &Path,
) -> Result<(), String> {
    let mut field = None;
    while let Some(f) = multipart
        .next_field()
        .await
        .map_err(|e| format!("Multipart error: {e}"))?
    {
        field = Some(f);
        break;
    }
    let Some(mut field) = field else {
        return Err("No file uploaded".to_string());
    };

    let mut file = tokio::fs::File::create(dest)
        .await
        .map_err(|e| format!("Failed to create temp file: {e}"))?;

    while let Some(chunk) = field
        .chunk()
        .await
        .map_err(|e| format!("Failed to read chunk: {e}"))?
    {
        file.write_all(&chunk)
            .await
            .map_err(|e| format!("Failed to write chunk: {e}"))?;
    }
    file.flush()
        .await
        .map_err(|e| format!("Failed to flush file: {e}"))
}

pub async fn sanitize_zip(input: &Path, output: &Path) -> Result<(), String> {
    crate::utils::zip::sanitize_zip(input, output)
        .await
        .map_err(|e| format!("ZIP validation error: {e}"))
}
