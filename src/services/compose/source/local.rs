use crate::utils::exec::{CommandExecutor, LocalExecutor};
use crate::utils::os::OsCli;
use std::path::Path;

/// Extract a ZIP archive to `dest_path` on the local machine.
pub async fn deploy_zip_locally(zip_path: &Path, dest_path: &str) -> Result<(), String> {
    let executor = CommandExecutor::Local(LocalExecutor::new());

    OsCli::new(&executor)
        .archive(zip_path)
        .zip()
        .extract_to(dest_path)
        .overwrite()
        .run()
        .await
        .map(|_| ())
        .map_err(|e| format!("Failed to extract ZIP locally: {e}"))
}
