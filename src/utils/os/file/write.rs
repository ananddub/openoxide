use crate::utils::exec::script::{IntoCommand, shell_single_quote};
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::utils::os::escape_arg;

pub struct FileWriteBuilder<'a> {
    executor: &'a CommandExecutor,
    path: String,
    content: String,
    append: bool,
}

impl<'a> FileWriteBuilder<'a> {
    pub fn new(
        executor: &'a CommandExecutor,
        path: String,
        content: impl IntoCommand,
        append: bool,
    ) -> Self {
        Self {
            executor,
            path,
            content: content.build_str(),
            append,
        }
    }

    pub fn append(mut self, val: bool) -> Self {
        self.append = val;
        self
    }

    pub async fn execute(self) -> ExecResult<ExecOutput> {
        let mut args = Vec::new();
        if self.append {
            args.push("-a".to_owned());
        }
        args.push(self.path);
        self.executor
            .run_with_stdin("tee", args, self.content.as_bytes())
            .await
    }
}

impl<'a> IntoCommand for FileWriteBuilder<'a> {
    fn build_str(&self) -> String {
        let safe_path = escape_arg(&self.path);
        let op = if self.append { ">>" } else { ">" };
        if self.content.starts_with('$') {
            let safe_content = escape_arg(&self.content);
            format!("printf '%s' {} {} {}", safe_content, op, safe_path)
        } else {
            let safe_content = shell_single_quote(&printf_b_escape(&self.content));
            format!("printf '%b' {} {} {}", safe_content, op, safe_path)
        }
    }
}

fn printf_b_escape(content: &str) -> String {
    let mut escaped = String::with_capacity(content.len());
    for ch in content.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            _ => escaped.push(ch),
        }
    }
    escaped
}
