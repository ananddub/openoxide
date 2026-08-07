use super::FileMode;
use crate::exec::script::IntoCommand;
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::escape_arg;

pub struct FileChmodBuilder<'a> {
    executor: &'a CommandExecutor,
    path: String,
    mode: FileMode,
    recursive: bool,
    reference: Option<String>,
}

impl<'a> FileChmodBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, path: String, mode: FileMode) -> Self {
        Self {
            executor,
            path,
            mode,
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
            args.push(self.mode.as_str().to_owned());
        }
        args.push(self.path.clone());
        self.executor.run("chmod", &args).await
    }
}

impl<'a> IntoCommand for FileChmodBuilder<'a> {
    fn build_str(&self) -> String {
        let mut parts = vec!["chmod".to_string()];
        if self.recursive {
            parts.push("-R".to_string());
        }
        if let Some(ref r) = self.reference {
            parts.push(format!("--reference={}", escape_arg(r)));
        } else {
            parts.push(escape_arg(self.mode.as_str()));
        }
        parts.push(escape_arg(&self.path));
        parts.join(" ")
    }
}
