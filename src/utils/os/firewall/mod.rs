use crate::utils::exec::CommandExecutor;
use crate::utils::exec::script::IntoCommand;

pub mod admin;
pub mod allow;
pub mod rules;

pub use admin::FirewallAdminBuilder;
pub use allow::FirewallAllowBuilder;
pub use rules::FirewallRulesBuilder;

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
    pub fn reload(&self) -> FirewallAdminBuilder<'a> {
        FirewallAdminBuilder::new(self.executor, vec!["reload".to_string()])
    }
    pub fn enable(&self) -> FirewallAdminBuilder<'a> {
        FirewallAdminBuilder::new(
            self.executor,
            vec!["--force".to_string(), "enable".to_string()],
        )
    }
    pub fn disable(&self) -> FirewallAdminBuilder<'a> {
        FirewallAdminBuilder::new(self.executor, vec!["disable".to_string()])
    }
}
