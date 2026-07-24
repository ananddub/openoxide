use crate::utils::exec::CommandExecutor;
use crate::utils::exec::script::IntoCommand;

pub mod action;
pub mod builder;
pub mod list;
pub mod logs;

pub use action::ServiceActionBuilder;
pub use builder::ServiceBuilder;
pub use list::ServiceCommandBuilder;
pub use logs::ServiceLogsBuilder;

pub struct ServiceCli<'a> {
    pub(crate) executor: &'a CommandExecutor,
}

impl<'a> ServiceCli<'a> {
    pub fn list(&self) -> ServiceCommandBuilder<'a> {
        ServiceCommandBuilder::new(
            self.executor,
            vec!["list-units".to_string(), "--type=service".to_string()],
        )
    }
    pub fn list_running(&self) -> ServiceCommandBuilder<'a> {
        ServiceCommandBuilder::new(
            self.executor,
            vec![
                "list-units".to_string(),
                "--type=service".to_string(),
                "--state=running".to_string(),
            ],
        )
    }
    pub fn list_failed(&self) -> ServiceCommandBuilder<'a> {
        ServiceCommandBuilder::new(
            self.executor,
            vec![
                "list-units".to_string(),
                "--type=service".to_string(),
                "--state=failed".to_string(),
            ],
        )
    }
    pub fn service(&self, name: impl IntoCommand) -> ServiceBuilder<'a> {
        ServiceBuilder::new(self.executor, name)
    }
}
