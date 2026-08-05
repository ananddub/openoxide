use crate::utils::exec::{CommandExecutor, ExecResult};

pub struct WireGuardKeyBuilder<'a> {
    executor: &'a CommandExecutor,
}

impl<'a> WireGuardKeyBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor) -> Self {
        Self { executor }
    }

    pub async fn generate(self) -> ExecResult<String> {
        Ok(self
            .executor
            .run("wg", ["genkey"])
            .await?
            .stdout_trimmed()
            .to_owned())
    }

    pub async fn public_from_private(self, private_key: &str) -> ExecResult<String> {
        Ok(self
            .executor
            .run_with_stdin("wg", ["pubkey"], private_key)
            .await?
            .stdout_trimmed()
            .to_owned())
    }
}
