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

    pub async fn execute(self) -> ExecResult<ExecOutput> {
        let mut args = Vec::new();
        if self.append {
            args.push("-a".to_owned());
        }
        args.push(self.path);
        self.executor
            .run_with_stdin("tee", args, self.content.as_bytes())
            .await
    }
}

impl<'a> IntoCommand for FileWriteBuilder<'a> {
    fn build_str(&self) -> String {
        let safe_path = escape_arg(&self.path);
        let safe_content = escape_arg(&self.content);
        let op = if self.append { ">>" } else { ">" };
        format!("printf '%s' {} {} {}", safe_content, op, safe_path)
    }
}
