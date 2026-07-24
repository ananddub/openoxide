use crate::utils::exec::script::IntoCommand;
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::utils::os::escape_arg;

pub struct FirewallAdminBuilder<'a> {
    executor: &'a CommandExecutor,
    args: Vec<String>,
}

impl<'a> FirewallAdminBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, args: Vec<String>) -> Self {
        Self { executor, args }
    }
    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.executor.run("ufw", &self.args).await
    }
}

impl<'a> IntoCommand for FirewallAdminBuilder<'a> {
    fn build_str(&self) -> String {
        let mut parts = vec!["ufw".to_string()];
        for arg in &self.args {
            parts.push(escape_arg(arg));
        }
        parts.join(" ")
    }
}
