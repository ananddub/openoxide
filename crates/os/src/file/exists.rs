use crate::exec::script::IntoCommand;
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::escape_arg;

pub struct FileExistsBuilder<'a> {
    executor: &'a CommandExecutor,
    path: String,
}

impl<'a> FileExistsBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, path: String) -> Self {
        Self { executor, path }
    }
    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.executor.run("test", &["-f", &self.path]).await
    }
}

impl<'a> IntoCommand for FileExistsBuilder<'a> {
    fn build_str(&self) -> String {
        format!("test -f {}", escape_arg(&self.path))
    }
}
