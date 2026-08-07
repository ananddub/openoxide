use crate::exec::script::IntoCommand;
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::escape_arg;

pub struct SymlinkPointsToBuilder<'a> {
    executor: &'a CommandExecutor,
    link: String,
    canonicalize: bool,
}

impl<'a> SymlinkPointsToBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, link: String) -> Self {
        Self {
            executor,
            link,
            canonicalize: true,
        }
    }
    pub fn canonicalize(mut self, val: bool) -> Self {
        self.canonicalize = val;
        self
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        let mut args = Vec::new();
        if self.canonicalize {
            args.push("-f".to_string());
        }
        args.push(self.link);
        self.executor.run("readlink", &args).await
    }
}

impl<'a> IntoCommand for SymlinkPointsToBuilder<'a> {
    fn build_str(&self) -> String {
        let mut parts = vec!["readlink".to_string()];
        if self.canonicalize {
            parts.push("-f".to_string());
        }
        parts.push(escape_arg(&self.link));
        parts.join(" ")
    }
}
