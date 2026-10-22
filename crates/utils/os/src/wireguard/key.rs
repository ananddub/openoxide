use crate::exec::{CommandExecutor, ExecResult};

use super::cli::{WireGuardKeyAction, WireGuardKeyCommand};

pub struct WireGuardKeyBuilder<'a> {
    executor: &'a CommandExecutor,
}

impl<'a> WireGuardKeyBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor) -> Self {
        Self { executor }
    }

    pub async fn generate(self) -> ExecResult<String> {
        Ok(
            WireGuardKeyCommand::new(self.executor, WireGuardKeyAction::GeneratePrivate)
                .run()
                .await?
                .stdout_trimmed()
                .to_owned(),
        )
    }

    pub async fn generate_preshared(self) -> ExecResult<String> {
        Ok(
            WireGuardKeyCommand::new(self.executor, WireGuardKeyAction::GeneratePreshared)
                .run()
                .await?
                .stdout_trimmed()
                .to_owned(),
        )
    }

    pub async fn public_from_private(self, private_key: &str) -> ExecResult<String> {
        Ok(
            WireGuardKeyCommand::new(self.executor, WireGuardKeyAction::DerivePublic)
                .run_with_stdin(private_key)
                .await?
                .stdout_trimmed()
                .to_owned(),
        )
    }
}
