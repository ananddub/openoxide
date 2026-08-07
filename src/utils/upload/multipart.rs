use std::{collections::HashMap, path::Path};

use axum::extract::Multipart;
use tokio::io::AsyncWriteExt;

pub const MAX_UPLOAD_BYTES: u64 = 100 * 1024 * 1024;

#[derive(Debug)]
pub struct MultipartFile {
    pub filename: String,
    pub fields: HashMap<String, String>,
    pub bytes_written: u64,
}

pub async fn stream_multipart_file(
    mut multipart: Multipart,
    file_field: &str,
    destination: &Path,
    limit: u64,
) -> Result<MultipartFile, String> {
    let mut output = tokio::fs::File::create(destination)
        .await
        .map_err(|error| format!("failed to create temporary upload: {error}"))?;
    let mut filename = None;
    let mut fields = HashMap::new();
    let mut bytes_written = 0u64;

    while let Some(mut field) = multipart
        .next_field()
        .await
        .map_err(|error| format!("invalid multipart upload: {error}"))?
    {
        let Some(name) = field.name().map(str::to_owned) else {
            continue;
        };
        if name == file_field {
            if filename.is_some() {
                return Err("only one file may be uploaded".into());
            }
            filename = Some(sanitize_filename(
                field.file_name().unwrap_or("upload.bin"),
            )?);
            while let Some(chunk) = field
                .chunk()
                .await
                .map_err(|error| format!("failed to read upload: {error}"))?
            {
                bytes_written = bytes_written
                    .checked_add(chunk.len() as u64)
                    .ok_or_else(|| "upload size overflow".to_owned())?;
                if bytes_written > limit {
                    return Err(format!("file exceeds {} MiB limit", limit / 1024 / 1024));
                }
                output
                    .write_all(&chunk)
                    .await
                    .map_err(|error| format!("failed to store upload: {error}"))?;
            }
        } else {
            let value = field
                .text()
                .await
                .map_err(|error| format!("invalid multipart field {name}: {error}"))?;
            fields.insert(name, value);
        }
    }

    output
        .flush()
        .await
        .map_err(|error| format!("failed to flush upload: {error}"))?;
    let filename = filename.ok_or_else(|| format!("{file_field} is required"))?;
    if bytes_written == 0 {
        return Err("uploaded file is empty".into());
    }
    Ok(MultipartFile {
        filename,
        fields,
        bytes_written,
    })
}

fn sanitize_filename(value: &str) -> Result<String, String> {
    let filename = value.rsplit(['/', '\\']).next().unwrap_or_default().trim();
    if filename.is_empty()
        || filename.len() > 255
        || filename.contains(['\0', '\r', '\n'])
        || filename == "."
        || filename == ".."
    {
        return Err("invalid upload filename".into());
    }
    Ok(filename.to_owned())
}

#[cfg(test)]
mod tests {
    use super::sanitize_filename;

    #[test]
    fn strips_client_paths_and_rejects_traversal_names() {
        assert_eq!(
            sanitize_filename("C:\\fakepath\\app.zip").unwrap(),
            "app.zip"
        );
        assert!(sanitize_filename("..").is_err());
        assert!(sanitize_filename("bad\nname").is_err());
    }
}
