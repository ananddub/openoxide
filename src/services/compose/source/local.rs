use crate::utils::exec::{CommandExecutor, LocalExecutor};
use crate::utils::zip::ZipBuilder;
use std::path::Path;

/// Extract a ZIP archive to `dest_path` on the local machine.
pub async fn deploy_zip_locally(zip_path: &Path, dest_path: &str) -> Result<(), String> {
    let executor = CommandExecutor::Local(LocalExecutor::new());

    ZipBuilder::new(&executor)
        .source(zip_path)
        .destination(dest_path)
        .overwrite()
        .unzip()
        .await
        .map_err(|e| format!("Failed to extract ZIP locally: {e}"))
}
