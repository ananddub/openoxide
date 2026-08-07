use crate::exec::script::IntoCommand;
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};

pub struct EnvAllBuilder<'a> {
    executor: &'a CommandExecutor,
}

impl<'a> EnvAllBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor) -> Self {
        Self { executor }
    }
    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.executor.run("printenv", &[] as &[&str]).await
    }
}

impl<'a> IntoCommand for EnvAllBuilder<'a> {
    fn build_str(&self) -> String {
        "printenv".to_string()
    }
}
