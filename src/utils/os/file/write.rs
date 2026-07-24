use crate::utils::exec::script::IntoCommand;
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::utils::os::escape_arg;

pub struct FileWriteBuilder<'a> {
    executor: &'a CommandExecutor,
    path: String,
    content: String,
    append: bool,
}

impl<'a> FileWriteBuilder<'a> {
    pub fn new(
        executor: &'a CommandExecutor,
        path: String,
        content: impl IntoCommand,
        append: bool,
    ) -> Self {
        Self {
            executor,
            path,
            content: content.build_str(),
            append,
        }
    }
    pub fn append(mut self, val: bool) -> Self {
        self.append = val;
        self
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        let op = if self.append { ">>" } else { ">" };
        let cmd = format!("echo \"$1\" {} \"$2\"", op);
        self.executor
            .run("sh", &["-c", &cmd, "dummy", &self.content, &self.path])
            .await
    }
}

impl<'a> IntoCommand for FileWriteBuilder<'a> {
    fn build_str(&self) -> String {
        let op = if self.append { ">>" } else { ">" };
        let cmd = format!("echo \"$1\" {} \"$2\"", op);
        format!(
            "sh -c '{}' dummy {} {}",
            cmd,
            escape_arg(&self.content),
            escape_arg(&self.path)
        )
    }
}
