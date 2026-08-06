use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};

use super::WireGuardQuickAction;

pub struct WireGuardQuickBuilder<'a> {
    executor: &'a CommandExecutor,
    action: WireGuardQuickAction,
    target: String,
}

impl<'a> WireGuardQuickBuilder<'a> {
    pub(crate) fn new(
        executor: &'a CommandExecutor,
        action: WireGuardQuickAction,
        target: impl Into<String>,
    ) -> Self {
        Self {
            executor,
            action,
            target: target.into(),
        }
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        super::validate_interface_or_config(&self.target)?;
        self.executor.run("wg-quick", self.args()).await
    }

    pub(super) fn args(&self) -> Vec<String> {
        vec![self.action.as_arg().into(), self.target.clone()]
    }
}
