use crate::exec::script::IntoCommand;
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::escape_arg;

pub struct DirDeleteBuilder<'a> {
    executor: &'a CommandExecutor,
    path: String,
    recursive: bool,
    force: bool,
    interactive: bool,
}

impl<'a> DirDeleteBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, path: String) -> Self {
        Self {
            executor,
            path,
            recursive: true,
            force: true,
            interactive: false,
        }
    }
    pub fn recursive(mut self, val: bool) -> Self {
        self.recursive = val;
        self
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
        let mut flags = String::new();
        if self.recursive {
            flags.push('r');
        }
        if self.force {
            flags.push('f');
        }
        if self.interactive {
            flags.push('i');
        }
        if !flags.is_empty() {
            args.push(format!("-{}", flags));
        }
        args.push(self.path.clone());
        self.executor.run("rm", &args).await
    }
}

impl<'a> IntoCommand for DirDeleteBuilder<'a> {
    fn build_str(&self) -> String {
        let mut parts = vec!["rm".to_string()];
        let mut flags = String::new();
        if self.recursive {
            flags.push('r');
        }
        if self.force {
            flags.push('f');
        }
        if self.interactive {
            flags.push('i');
        }
        if !flags.is_empty() {
            parts.push(format!("-{}", flags));
        }
        parts.push(escape_arg(&self.path));
        parts.join(" ")
    }
}
