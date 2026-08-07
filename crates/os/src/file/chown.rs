use crate::exec::script::IntoCommand;
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::escape_arg;

pub struct FileChownBuilder<'a> {
    executor: &'a CommandExecutor,
    path: String,
    owner: String,
    recursive: bool,
    reference: Option<String>,
}

impl<'a> FileChownBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, path: String, owner: impl IntoCommand) -> Self {
        Self {
            executor,
            path,
            owner: owner.build_str(),
            recursive: false,
            reference: None,
        }
    }
    pub fn recursive(mut self, val: bool) -> Self {
        self.recursive = val;
        self
    }
    pub fn reference(mut self, val: impl Into<String>) -> Self {
        self.reference = Some(val.into());
        self
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        let mut args = Vec::new();
        if self.recursive {
            args.push("-R".to_string());
        }
        if let Some(ref r) = self.reference {
            args.push(format!("--reference={}", r));
        } else {
            args.push(self.owner.clone());
        }
        args.push(self.path.clone());
        self.executor.run("chown", &args).await
    }
}

impl<'a> IntoCommand for FileChownBuilder<'a> {
    fn build_str(&self) -> String {
        let mut parts = vec!["chown".to_string()];
        if self.recursive {
            parts.push("-R".to_string());
        }
        if let Some(ref r) = self.reference {
            parts.push(format!("--reference={}", escape_arg(r)));
        } else {
            parts.push(escape_arg(&self.owner));
        }
        parts.push(escape_arg(&self.path));
        parts.join(" ")
    }
}
