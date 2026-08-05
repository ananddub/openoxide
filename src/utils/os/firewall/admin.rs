use crate::utils::exec::script::IntoCommand;
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::utils::os::escape_arg;

pub struct FirewallAdminBuilder<'a> {
    executor: &'a CommandExecutor,
    action: FirewallAdminAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FirewallAdminAction {
    Reload,
    Enable,
    Disable,
}

impl<'a> FirewallAdminBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, action: FirewallAdminAction) -> Self {
        Self { executor, action }
    }
    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.executor.run("ufw", self.args()).await
    }

    fn args(&self) -> &'static [&'static str] {
        match self.action {
            FirewallAdminAction::Reload => &["reload"],
            FirewallAdminAction::Enable => &["--force", "enable"],
            FirewallAdminAction::Disable => &["disable"],
        }
    }
}

impl<'a> IntoCommand for FirewallAdminBuilder<'a> {
    fn build_str(&self) -> String {
        let mut parts = vec!["ufw".to_string()];
        for arg in self.args() {
            parts.push(escape_arg(arg));
        }
        parts.join(" ")
    }
}
