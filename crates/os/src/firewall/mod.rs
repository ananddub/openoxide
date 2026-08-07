use crate::exec::CommandExecutor;
use crate::exec::script::IntoCommand;

pub mod admin;
pub mod allow;
mod inspect;
pub mod rules;

use admin::FirewallAdminAction;
pub use admin::FirewallAdminBuilder;
pub use allow::FirewallAllowBuilder;
pub use inspect::{FirewallBackend, FirewallInspectBuilder};
pub use rules::FirewallRulesBuilder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkProtocol {
    Tcp,
    Udp,
}

impl NetworkProtocol {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Tcp => "tcp",
            Self::Udp => "udp",
        }
    }
}

pub struct FirewallCli<'a> {
    pub(crate) executor: &'a CommandExecutor,
}

impl<'a> FirewallCli<'a> {
    pub fn allow_port(&self, port: impl IntoCommand) -> FirewallAllowBuilder<'a> {
        FirewallAllowBuilder::new(self.executor, port)
    }
    pub fn rules(&self) -> FirewallRulesBuilder<'a> {
        FirewallRulesBuilder::new(self.executor)
    }
    pub fn inspect(&self, backend: FirewallBackend) -> FirewallInspectBuilder<'a> {
        FirewallInspectBuilder::new(self.executor, backend)
    }
    pub fn reload(&self) -> FirewallAdminBuilder<'a> {
        FirewallAdminBuilder::new(self.executor, FirewallAdminAction::Reload)
    }
    pub fn enable(&self) -> FirewallAdminBuilder<'a> {
        FirewallAdminBuilder::new(self.executor, FirewallAdminAction::Enable)
    }
    pub fn disable(&self) -> FirewallAdminBuilder<'a> {
        FirewallAdminBuilder::new(self.executor, FirewallAdminAction::Disable)
    }
}

#[cfg(test)]
mod tests {
    use super::FirewallBackend;
    use crate::exec::script::IntoCommand;
    use crate::exec::{CommandExecutor, LocalExecutor};
    use crate::OsCli;

    #[test]
    fn firewall_actions_build_typed_commands() {
        let executor = CommandExecutor::Local(LocalExecutor::new());
        let os = OsCli::new(&executor);
        assert_eq!(
            os.firewall().rules().numbered().verbose().build_str(),
            "ufw status numbered verbose"
        );
        assert_eq!(
            os.firewall()
                .inspect(FirewallBackend::Firewalld)
                .build_str(),
            "firewall-cmd --list-all"
        );
    }
}
