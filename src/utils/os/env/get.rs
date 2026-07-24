use crate::utils::exec::script::IntoCommand;
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::utils::os::escape_arg;

pub struct EnvGetBuilder<'a> {
    executor: &'a CommandExecutor,
    key: String,
}

impl<'a> EnvGetBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, key: impl IntoCommand) -> Self {
        Self {
            executor,
            key: key.build_str(),
        }
    }
    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.executor
            .run("sh", &["-c", "eval echo \"\\$$1\"", "dummy", &self.key])
            .await
    }
}

impl<'a> IntoCommand for EnvGetBuilder<'a> {
    fn build_str(&self) -> String {
        format!(
            "sh -c 'eval echo \"\\$$1\"' dummy {}",
            escape_arg(&self.key)
        )
    }
}
