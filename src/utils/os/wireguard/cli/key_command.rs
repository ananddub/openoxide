use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WireGuardKeyAction {
    GeneratePrivate,
    GeneratePreshared,
    DerivePublic,
}

impl WireGuardKeyAction {
    const fn as_arg(self) -> &'static str {
        match self {
            Self::GeneratePrivate => "genkey",
            Self::GeneratePreshared => "genpsk",
            Self::DerivePublic => "pubkey",
        }
    }
}

pub(crate) struct WireGuardKeyCommand<'a> {
    executor: &'a CommandExecutor,
    action: WireGuardKeyAction,
}

impl<'a> WireGuardKeyCommand<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, action: WireGuardKeyAction) -> Self {
        Self { executor, action }
    }

    pub(crate) async fn run(self) -> ExecResult<ExecOutput> {
        self.executor.run("wg", [self.action.as_arg()]).await
    }

    pub(crate) async fn run_with_stdin(self, value: &str) -> ExecResult<ExecOutput> {
        self.executor
            .run_with_stdin("wg", [self.action.as_arg()], value)
            .await
    }
}
