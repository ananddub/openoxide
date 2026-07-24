use crate::utils::exec::script::IntoCommand;
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::utils::os::escape_arg;

pub struct EnvExistsBuilder<'a> {
    executor: &'a CommandExecutor,
    key: String,
}

impl<'a> EnvExistsBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, key: impl IntoCommand) -> Self {
        Self {
            executor,
            key: key.build_str(),
        }
    }
    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.executor
            .run("sh", &["-c", "[ -n \"${1+x}\" ]", "dummy", &self.key])
            .await
    }
}

impl<'a> IntoCommand for EnvExistsBuilder<'a> {
    fn build_str(&self) -> String {
        format!(
            "sh -c '[ -n \"${{1+x}}\" ]' dummy {}",
            escape_arg(&self.key)
        )
    }
}
