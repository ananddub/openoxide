use std::path::Path;

use crate::utils::{exec::CommandExecutor, os::OsCli};

pub async fn extract_zip(
    executor: &CommandExecutor,
    archive: impl AsRef<Path>,
    destination: &str,
) -> Result<(), String> {
    OsCli::new(executor)
        .archive(archive.as_ref())
        .zip()
        .extract_to(destination)
        .overwrite()
        .run()
        .await
        .map(|_| ())
        .map_err(|error| format!("failed to extract ZIP: {error}"))
}
