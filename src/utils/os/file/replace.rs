use crate::utils::exec::script::IntoCommand;
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::utils::os::escape_arg;

pub struct FileReplaceBuilder<'a> {
    executor: &'a CommandExecutor,
    path: String,
    old: String,
    new: String,
}

impl<'a> FileReplaceBuilder<'a> {
    pub fn new(
        executor: &'a CommandExecutor,
        path: String,
        old: impl IntoCommand,
        new: impl IntoCommand,
    ) -> Self {
        Self {
            executor,
            path,
            old: old.build_str(),
            new: new.build_str(),
        }
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        let pattern = format!("s|{}|{}|g", self.old, self.new);
        self.executor
            .run("sed", &["-i".to_string(), pattern, self.path])
            .await
    }
}

impl<'a> IntoCommand for FileReplaceBuilder<'a> {
    fn build_str(&self) -> String {
        let pattern = format!("s|{}|{}|g", self.old, self.new);
        format!("sed -i {} {}", escape_arg(&pattern), escape_arg(&self.path))
    }
}
