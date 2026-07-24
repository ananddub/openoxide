use crate::utils::exec::script::IntoCommand;
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};

pub struct FirewallRulesBuilder<'a> {
    executor: &'a CommandExecutor,
    numbered: bool,
    verbose: bool,
}

impl<'a> FirewallRulesBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor) -> Self {
        Self {
            executor,
            numbered: false,
            verbose: false,
        }
    }
    pub fn numbered(mut self, val: bool) -> Self {
        self.numbered = val;
        self
    }
    pub fn verbose(mut self, val: bool) -> Self {
        self.verbose = val;
        self
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        let mut args = vec!["status".to_string()];
        if self.numbered {
            args.push("numbered".to_string());
        }
        if self.verbose {
            args.push("verbose".to_string());
        }
        self.executor.run("ufw", &args).await
    }
}

impl<'a> IntoCommand for FirewallRulesBuilder<'a> {
    fn build_str(&self) -> String {
        let mut parts = vec!["ufw".to_string(), "status".to_string()];
        if self.numbered {
            parts.push("numbered".to_string());
        }
        if self.verbose {
            parts.push("verbose".to_string());
        }
        parts.join(" ")
    }
}
