use crate::escape_arg;
use crate::exec::script::IntoCommand;
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};

pub struct SymlinkExistsBuilder<'a> {
    executor: &'a CommandExecutor,
    link: String,
}

impl<'a> SymlinkExistsBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, link: String) -> Self {
        Self { executor, link }
    }
    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.executor.run("test", &["-h", &self.link]).await
    }
}

impl<'a> IntoCommand for SymlinkExistsBuilder<'a> {
    fn build_str(&self) -> String {
        format!("test -h {}", escape_arg(&self.link))
    }
}
