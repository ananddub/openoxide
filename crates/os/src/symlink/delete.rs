use crate::escape_arg;
use crate::exec::script::IntoCommand;
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};

pub struct SymlinkDeleteBuilder<'a> {
    executor: &'a CommandExecutor,
    link: String,
    force: bool,
}

impl<'a> SymlinkDeleteBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, link: String) -> Self {
        Self {
            executor,
            link,
            force: true,
        }
    }
    pub fn force(mut self, val: bool) -> Self {
        self.force = val;
        self
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        let mut args = Vec::new();
        if self.force {
            args.push("-f".to_string());
        }
        args.push(self.link);
        self.executor.run("rm", &args).await
    }
}

impl<'a> IntoCommand for SymlinkDeleteBuilder<'a> {
    fn build_str(&self) -> String {
        let mut parts = vec!["rm".to_string()];
        if self.force {
            parts.push("-f".to_string());
        }
        parts.push(escape_arg(&self.link));
        parts.join(" ")
    }
}
