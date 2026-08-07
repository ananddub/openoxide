use crate::escape_arg;
use crate::exec::script::IntoCommand;
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};

pub struct DirTempBuilder<'a> {
    executor: &'a CommandExecutor,
    directory: bool,
    tmpdir: Option<String>,
    suffix: Option<String>,
}

impl<'a> DirTempBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor) -> Self {
        Self {
            executor,
            directory: true,
            tmpdir: None,
            suffix: None,
        }
    }
    pub fn directory(mut self, val: bool) -> Self {
        self.directory = val;
        self
    }
    pub fn tmpdir(mut self, val: impl Into<String>) -> Self {
        self.tmpdir = Some(val.into());
        self
    }
    pub fn suffix(mut self, val: impl Into<String>) -> Self {
        self.suffix = Some(val.into());
        self
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        let mut args = Vec::new();
        if self.directory {
            args.push("-d".to_string());
        }
        if let Some(ref tmp) = self.tmpdir {
            args.push(format!("--tmpdir={}", tmp));
        }
        if let Some(ref suf) = self.suffix {
            args.push(format!("--suffix={}", suf));
        }
        self.executor.run("mktemp", &args).await
    }
}

impl<'a> IntoCommand for DirTempBuilder<'a> {
    fn build_str(&self) -> String {
        let mut parts = vec!["mktemp".to_string()];
        if self.directory {
            parts.push("-d".to_string());
        }
        if let Some(ref tmp) = self.tmpdir {
            parts.push(format!("--tmpdir={}", escape_arg(tmp)));
        }
        if let Some(ref suf) = self.suffix {
            parts.push(format!("--suffix={}", escape_arg(suf)));
        }
        parts.join(" ")
    }
}
