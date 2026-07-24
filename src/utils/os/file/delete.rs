use crate::utils::exec::script::IntoCommand;
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::utils::os::escape_arg;

pub struct FileDeleteBuilder<'a> {
    executor: &'a CommandExecutor,
    path: String,
    force: bool,
    interactive: bool,
}

impl<'a> FileDeleteBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, path: String) -> Self {
        Self {
            executor,
            path,
            force: true,
            interactive: false,
        }
    }
    pub fn force(mut self, val: bool) -> Self {
        self.force = val;
        self
    }
    pub fn interactive(mut self, val: bool) -> Self {
        self.interactive = val;
        self
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        let mut args = Vec::new();
        if self.force {
            args.push("-f".to_string());
        }
        if self.interactive {
            args.push("-i".to_string());
        }
        args.push(self.path);
        self.executor.run("rm", &args).await
    }
}

impl<'a> IntoCommand for FileDeleteBuilder<'a> {
    fn build_str(&self) -> String {
        let mut parts = vec!["rm".to_string()];
        if self.force {
            parts.push("-f".to_string());
        }
        if self.interactive {
            parts.push("-i".to_string());
        }
        parts.push(escape_arg(&self.path));
        parts.join(" ")
    }
}
