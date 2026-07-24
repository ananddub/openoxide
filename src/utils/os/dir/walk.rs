use crate::utils::exec::script::IntoCommand;
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::utils::os::escape_arg;

pub struct DirWalkBuilder<'a> {
    executor: &'a CommandExecutor,
    path: String,
    max_depth: Option<u32>,
    min_depth: Option<u32>,
    type_filter: Option<String>,
    name_pattern: Option<String>,
}

impl<'a> DirWalkBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, path: String) -> Self {
        Self {
            executor,
            path,
            max_depth: None,
            min_depth: None,
            type_filter: None,
            name_pattern: None,
        }
    }
    pub fn max_depth(mut self, val: u32) -> Self {
        self.max_depth = Some(val);
        self
    }
    pub fn min_depth(mut self, val: u32) -> Self {
        self.min_depth = Some(val);
        self
    }
    pub fn type_file(mut self) -> Self {
        self.type_filter = Some("f".to_string());
        self
    }
    pub fn type_dir(mut self) -> Self {
        self.type_filter = Some("d".to_string());
        self
    }
    pub fn name(mut self, val: impl Into<String>) -> Self {
        self.name_pattern = Some(val.into());
        self
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        let mut args = vec![self.path.clone()];
        if let Some(max) = self.max_depth {
            args.push("-maxdepth".to_string());
            args.push(max.to_string());
        }
        if let Some(min) = self.min_depth {
            args.push("-mindepth".to_string());
            args.push(min.to_string());
        }
        if let Some(ref t) = self.type_filter {
            args.push("-type".to_string());
            args.push(t.clone());
        }
        if let Some(ref name) = self.name_pattern {
            args.push("-name".to_string());
            args.push(name.clone());
        }
        self.executor.run("find", &args).await
    }
}

impl<'a> IntoCommand for DirWalkBuilder<'a> {
    fn build_str(&self) -> String {
        let mut parts = vec!["find".to_string(), escape_arg(&self.path)];
        if let Some(max) = self.max_depth {
            parts.push("-maxdepth".to_string());
            parts.push(max.to_string());
        }
        if let Some(min) = self.min_depth {
            parts.push("-mindepth".to_string());
            parts.push(min.to_string());
        }
        if let Some(ref t) = self.type_filter {
            parts.push("-type".to_string());
            parts.push(escape_arg(t));
        }
        if let Some(ref name) = self.name_pattern {
            parts.push("-name".to_string());
            parts.push(escape_arg(name));
        }
        parts.join(" ")
    }
}
