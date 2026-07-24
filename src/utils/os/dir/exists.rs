use crate::utils::exec::script::IntoCommand;
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::utils::os::escape_arg;

pub struct DirExistsBuilder<'a> {
    executor: &'a CommandExecutor,
    path: String,
}

impl<'a> DirExistsBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, path: String) -> Self {
        Self { executor, path }
    }
    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.executor.run("test", &["-d", &self.path]).await
    }
}

impl<'a> IntoCommand for DirExistsBuilder<'a> {
    fn build_str(&self) -> String {
        format!("test -d {}", escape_arg(&self.path))
    }
}
