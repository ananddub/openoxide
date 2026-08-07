use crate::escape_arg;
use crate::exec::script::IntoCommand;
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};

pub struct DirRemoveEmptyBuilder<'a> {
    executor: &'a CommandExecutor,
    path: String,
}

impl<'a> DirRemoveEmptyBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, path: String) -> Self {
        Self { executor, path }
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.executor.run("rmdir", [self.path]).await
    }
}

impl IntoCommand for DirRemoveEmptyBuilder<'_> {
    fn build_str(&self) -> String {
        format!("rmdir {}", escape_arg(&self.path))
    }
}
