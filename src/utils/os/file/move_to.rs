use crate::utils::exec::script::{IntoCommand, shell_single_quote};
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};

pub struct FileMoveBuilder<'a> {
    executor: &'a CommandExecutor,
    source: String,
    target: String,
}

impl<'a> FileMoveBuilder<'a> {
    pub fn new(
        executor: &'a CommandExecutor,
        source: impl IntoCommand,
        target: impl IntoCommand,
    ) -> Self {
        Self {
            executor,
            source: source.build_str(),
            target: target.build_str(),
        }
    }

    pub async fn execute(self) -> ExecResult<ExecOutput> {
        self.executor.run("mv", [self.source, self.target]).await
    }
}

impl IntoCommand for FileMoveBuilder<'_> {
    fn build_str(&self) -> String {
        let safe_src = shell_single_quote(&self.source);
        let safe_dst = shell_single_quote(&self.target);
        format!("mv {} {}", safe_src, safe_dst)
    }
}
