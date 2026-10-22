use crate::escape_arg;
use crate::exec::script::IntoCommand;
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::file::FileMode;

pub struct DirCreateBuilder<'a> {
    executor: &'a CommandExecutor,
    path: String,
    parents: bool,
    verbose: bool,
    mode: Option<FileMode>,
}

impl<'a> DirCreateBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, path: String) -> Self {
        Self {
            executor,
            path,
            parents: true,
            verbose: false,
            mode: None,
        }
    }
    pub fn parents(mut self, val: bool) -> Self {
        self.parents = val;
        self
    }
    pub fn with_parents(mut self) -> Self {
        self.parents = true;
        self
    }
    pub fn verbose(mut self, val: bool) -> Self {
        self.verbose = val;
        self
    }
    pub fn mode(mut self, val: FileMode) -> Self {
        self.mode = Some(val);
        self
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        let mut args = Vec::new();
        if self.parents {
            args.push("-p".to_string());
        }
        if self.verbose {
            args.push("-v".to_string());
        }
        if let Some(ref m) = self.mode {
            args.push("-m".to_string());
            args.push(m.as_str().to_owned());
        }
        args.push(self.path.clone());
        self.executor.run("mkdir", &args).await
    }
}

impl<'a> IntoCommand for DirCreateBuilder<'a> {
    fn build_str(&self) -> String {
        let mut parts = vec!["mkdir".to_string()];
        if self.parents {
            parts.push("-p".to_string());
        }
        if self.verbose {
            parts.push("-v".to_string());
        }
        if let Some(ref m) = self.mode {
            parts.push("-m".to_string());
            parts.push(escape_arg(m.as_str()));
        }
        parts.push(escape_arg(&self.path));
        parts.join(" ")
    }
}
