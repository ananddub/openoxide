use crate::escape_arg;
use crate::exec::script::IntoCommand;
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};

pub struct DirCopyBuilder<'a> {
    executor: &'a CommandExecutor,
    source: String,
    target: String,
    preserve: bool,
    contents_only: bool,
}

impl<'a> DirCopyBuilder<'a> {
    pub(crate) fn new(
        executor: &'a CommandExecutor,
        source: String,
        target: impl IntoCommand,
    ) -> Self {
        Self {
            executor,
            source,
            target: target.build_str(),
            preserve: true,
            contents_only: false,
        }
    }

    pub fn preserve(mut self, enabled: bool) -> Self {
        self.preserve = enabled;
        self
    }

    pub fn contents_only(mut self, enabled: bool) -> Self {
        self.contents_only = enabled;
        self
    }

    fn source(&self) -> String {
        if self.contents_only {
            format!("{}/.", self.source.trim_end_matches('/'))
        } else {
            self.source.clone()
        }
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        let mut args = vec![if self.preserve { "-a" } else { "-R" }.to_owned()];
        args.extend([self.source(), self.target.clone()]);
        self.executor.run("cp", args).await
    }
}

impl IntoCommand for DirCopyBuilder<'_> {
    fn build_str(&self) -> String {
        format!(
            "cp {} {} {}",
            if self.preserve { "-a" } else { "-R" },
            escape_arg(self.source()),
            escape_arg(&self.target)
        )
    }
}
