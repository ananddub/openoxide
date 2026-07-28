use crate::utils::exec::script::IntoCommand;
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::utils::os::escape_arg;

pub struct DirWalkBuilder<'a> {
    executor: &'a CommandExecutor,
    path: String,
    extra_paths: Vec<String>,
    max_depth: Option<u32>,
    min_depth: Option<u32>,
    type_filter: Option<String>,
    name_patterns: Vec<String>,
    printf_format: Option<String>,
    ignore_errors: bool,
}

impl<'a> DirWalkBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, path: String) -> Self {
        Self {
            executor,
            path,
            extra_paths: Vec::new(),
            max_depth: None,
            min_depth: None,
            type_filter: None,
            name_patterns: Vec::new(),
            printf_format: None,
            ignore_errors: false,
        }
    }
    pub fn also(mut self, path: impl IntoCommand) -> Self {
        self.extra_paths.push(path.build_str());
        self
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
        self.name_patterns = vec![val.into()];
        self
    }
    pub fn names<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.name_patterns = values.into_iter().map(Into::into).collect();
        self
    }
    pub fn printf(mut self, format: impl Into<String>) -> Self {
        self.printf_format = Some(format.into());
        self
    }
    pub fn ignore_errors(mut self) -> Self {
        self.ignore_errors = true;
        self
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.executor.run("sh", ["-c", &self.build_str()]).await
    }
}

impl<'a> IntoCommand for DirWalkBuilder<'a> {
    fn build_str(&self) -> String {
        let mut parts = vec!["find".to_string(), escape_arg(&self.path)];
        parts.extend(self.extra_paths.iter().map(escape_arg));
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
        match self.name_patterns.as_slice() {
            [] => {}
            [name] => {
                parts.push("-name".to_string());
                parts.push(escape_arg(name));
            }
            names => {
                parts.push("\\(".to_string());
                for (idx, name) in names.iter().enumerate() {
                    if idx > 0 {
                        parts.push("-o".to_string());
                    }
                    parts.push("-name".to_string());
                    parts.push(escape_arg(name));
                }
                parts.push("\\)".to_string());
            }
        }
        if let Some(format) = &self.printf_format {
            parts.push("-printf".to_string());
            parts.push(escape_arg(format));
        }
        let mut command = parts.join(" ");
        if self.ignore_errors {
            command.push_str(" 2>/dev/null || true");
        }
        command
    }
}

#[cfg(test)]
mod tests {
    use super::DirWalkBuilder;
    use crate::utils::exec::script::IntoCommand;
    use crate::utils::exec::{CommandExecutor, LocalExecutor};

    #[test]
    fn walk_builder_supports_multiple_roots_names_and_printf() {
        let executor = CommandExecutor::Local(LocalExecutor::new());
        let command = DirWalkBuilder::new(&executor, "/etc/rustploy/traefik".to_string())
            .also("/etc/traefik")
            .max_depth(3)
            .type_file()
            .names(["*.yml", "*.yaml", "*.json"])
            .printf("%p\t%s\t%T@\n")
            .ignore_errors()
            .build_str();

        assert!(command.contains("find '/etc/rustploy/traefik' '/etc/traefik'"));
        assert!(command.contains("\\( -name '*.yml' -o -name '*.yaml' -o -name '*.json' \\)"));
        assert!(command.contains("-printf '%p\t%s\t%T@\n'"));
        assert!(command.ends_with(" 2>/dev/null || true"));
    }
}
