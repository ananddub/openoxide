use std::path::Path;

use crate::utils::{
    exec::{CommandExecutor, LocalExecutor},
    os::OsCli,
};

pub async fn sanitize_zip(input: &Path, output: &Path) -> Result<(), String> {
    let executor = CommandExecutor::Local(LocalExecutor::new());
    OsCli::new(&executor)
        .archive(input)
        .zip()
        .sanitize()
        .to(output)
        .run()
        .await
        .map(|_| ())
        .map_err(|error| format!("ZIP validation failed: {error}"))
}

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
