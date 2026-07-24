use crate::utils::exec::script::IntoCommand;
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::utils::os::escape_arg;

pub struct FileChmodBuilder<'a> {
    executor: &'a CommandExecutor,
    path: String,
    mode: String,
    recursive: bool,
    reference: Option<String>,
}

impl<'a> FileChmodBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, path: String, mode: impl IntoCommand) -> Self {
        Self {
            executor,
            path,
            mode: mode.build_str(),
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
            args.push(self.mode.clone());
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
            parts.push(escape_arg(&self.mode));
        }
        parts.push(escape_arg(&self.path));
        parts.join(" ")
    }
}
