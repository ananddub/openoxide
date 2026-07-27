use crate::utils::exec::script::{IntoCommand, shell_single_quote};
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};

pub struct FileReadBuilder<'a> {
    executor: &'a CommandExecutor,
    path: String,
    number: bool,
    squeeze_blank: bool,
}

impl<'a> FileReadBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, path: String) -> Self {
        Self {
            executor,
            path,
            number: false,
            squeeze_blank: false,
        }
    }

    pub fn number(mut self, val: bool) -> Self {
        self.number = val;
        self
    }

    pub fn squeeze_blank(mut self, val: bool) -> Self {
        self.squeeze_blank = val;
        self
    }

    pub async fn execute(self) -> ExecResult<ExecOutput> {
        let mut args = Vec::new();
        if self.number {
            args.push("-n".to_owned());
        }
        if self.squeeze_blank {
            args.push("-s".to_owned());
        }
        args.push(self.path);
        self.executor.run("cat", args).await
    }
}

impl<'a> IntoCommand for FileReadBuilder<'a> {
    fn build_str(&self) -> String {
        let mut parts = vec!["cat".to_string()];
        if self.number {
            parts.push("-n".to_string());
        }
        if self.squeeze_blank {
            parts.push("-s".to_string());
        }
        parts.push(shell_single_quote(&self.path));
        parts.join(" ")
    }
}
