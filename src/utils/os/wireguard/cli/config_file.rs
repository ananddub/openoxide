use std::path::PathBuf;

use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};

use super::WireGuardConfigAction;

pub struct WireGuardConfigFileBuilder<'a> {
    executor: &'a CommandExecutor,
    action: WireGuardConfigAction,
    interface: String,
    path: PathBuf,
}

impl<'a> WireGuardConfigFileBuilder<'a> {
    pub(crate) fn new(
        executor: &'a CommandExecutor,
        action: WireGuardConfigAction,
        interface: impl Into<String>,
        path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            executor,
            action,
            interface: interface.into(),
            path: path.into(),
        }
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        super::validate_interface_name(&self.interface)?;
        self.executor.run("wg", self.args()).await
    }

    pub(super) fn args(&self) -> Vec<String> {
        vec![
            self.action.as_arg().into(),
            self.interface.clone(),
            self.path.to_string_lossy().into_owned(),
        ]
    }
}
