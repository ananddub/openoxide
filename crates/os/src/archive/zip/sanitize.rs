use super::ZipError;
use crate::OsCli;
use crate::exec::{CommandExecutor, LocalExecutor};
use std::path::Path;

/// Security-check `input` ZIP, then write a sanitized copy to `output`.
///
/// Checks:
/// - Path traversal (`../`, absolute paths, Windows drive letters)
/// - Symlinks
/// - Empty archive after stripping `__MACOSX`
///
/// Also unwraps a single root-folder wrapper so contents land flat at dest.
pub async fn sanitize_zip(input: &Path, output: &Path) -> Result<(), ZipError> {
    let local = CommandExecutor::Local(LocalExecutor::new());
    sanitize_zip_with_executor(&local, input, output).await
}

pub(crate) async fn sanitize_zip_with_executor(
    executor: &CommandExecutor,
    input: &Path,
    output: &Path,
) -> Result<(), ZipError> {
    // 1. List entries via `unzip -Z1` (one path per line, no headers)
    let os = OsCli::new(executor);
    let list_out = os.archive(input).zip().list().names_only().run().await?;
    if !list_out.status.success() {
        return Err(ZipError::Failed(format!(
            "unzip -Z1 failed: {}",
            list_out.stderr
        )));
    }

    let entries: Vec<&str> = list_out
        .stdout
        .lines()
        .filter(|l| !l.contains("__MACOSX"))
        .collect();

    if entries.is_empty() {
        return Err(ZipError::Failed(
            "ZIP archive is empty after filtering __MACOSX".into(),
        ));
    }

    // 2. Path traversal + symlink checks
    for entry in &entries {
        check_path(entry)?;
    }
    check_symlinks(input, executor).await?;

    // 3. Extract to temp dir (strip __MACOSX)
    let temp_dir = tempfile::tempdir().map_err(|e| ZipError::Io(e))?;
    let extract_dir = temp_dir.path();

    os.archive(input)
        .zip()
        .extract_to(extract_dir)
        .overwrite()
        .exclude("__MACOSX/*")
        .exclude("*/__MACOSX/*")
        .run()
        .await?;

    // 4. Strip single root-folder wrapper if present
    let source_dir = match detect_single_root(&entries) {
        Some(root) => extract_dir.join(&root),
        None => extract_dir.to_path_buf(),
    };

    // 5. Re-zip sanitized contents into output
    //    `cd <source_dir> && zip -r <output> .`
    os.archive(output)
        .zip()
        .create_from(&source_dir)
        .contents_only()
        .run()
        .await?;

    Ok(())
}

// ── Security checks ───────────────────────────────────────────────────────────

fn check_path(entry: &str) -> Result<(), ZipError> {
    if entry.starts_with('/') || entry.starts_with('\\') {
        return Err(ZipError::Failed(format!(
            "path traversal in entry: {entry}"
        )));
    }
    if entry.len() >= 2 && entry.chars().nth(1) == Some(':') {
        return Err(ZipError::Failed(format!(
            "absolute Windows path in entry: {entry}"
        )));
    }
    for component in entry.split(['/', '\\']) {
        if component == ".." {
            return Err(ZipError::Failed(format!(
                "path traversal in entry: {entry}"
            )));
        }
    }
    Ok(())
}

async fn check_symlinks(zip: &Path, executor: &CommandExecutor) -> Result<(), ZipError> {
    let out = OsCli::new(executor)
        .archive(zip)
        .zip()
        .list()
        .verbose()
        .run()
        .await?;
    if !out.status.success() {
        return Ok(());
    } // non-fatal if -v unsupported

    for line in out.stdout.lines() {
        // `unzip -v` attribute column starts with 'l' for symlinks
        if line.trim_start().starts_with('l') {
            let name = line.split_whitespace().last().unwrap_or("");
            if !name.contains("__MACOSX") {
                return Err(ZipError::Failed(format!(
                    "symlink forbidden in entry: {name}"
                )));
            }
        }
    }
    Ok(())
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn detect_single_root(entries: &[&str]) -> Option<String> {
    let mut root: Option<&str> = None;
    for entry in entries {
        let top = entry.split(['/', '\\']).next().filter(|s| !s.is_empty())?;
        match root {
            None => root = Some(top),
            Some(r) if r != top => return None,
            _ => {}
        }
    }
    let r = root?.to_string();
    // Only strip if at least one entry is nested (i.e. it's actually a folder)
    if entries.iter().any(|e| e.contains('/') || e.contains('\\')) {
        Some(r)
    } else {
        None
    }
}
