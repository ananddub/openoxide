use crate::utils::exec::script::IntoCommand;
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::utils::os::escape_arg;

pub struct EnvUnsetBuilder<'a> {
    executor: &'a CommandExecutor,
    key: String,
}

impl<'a> EnvUnsetBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, key: impl IntoCommand) -> Self {
        Self {
            executor,
            key: key.build_str(),
        }
    }
    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.executor
            .run("sh", &["-c", &format!("unset {}", self.key)])
            .await
    }
}

impl<'a> IntoCommand for EnvUnsetBuilder<'a> {
    fn build_str(&self) -> String {
        format!("unset {}", escape_arg(&self.key))
    }
}
