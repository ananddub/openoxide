use crate::exec::script::{IntoCommand, sh, shell_single_quote};
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};

pub struct FileCopyBuilder<'a> {
    executor: &'a CommandExecutor,
    source: String,
    target: String,
    force: bool,
}

impl<'a> FileCopyBuilder<'a> {
    pub fn new(
        executor: &'a CommandExecutor,
        source: impl IntoCommand,
        target: impl IntoCommand,
    ) -> Self {
        Self {
            executor,
            source: source.build_str(),
            target: target.build_str(),
            force: true,
        }
    }

    pub fn force(mut self, force: bool) -> Self {
        self.force = force;
        self
    }

    pub async fn execute(self) -> ExecResult<ExecOutput> {
        if self.force {
            self.script().execute(self.executor).await
        } else {
            self.executor.run("cp", [self.source, self.target]).await
        }
    }

    fn script(&self) -> Vec<crate::exec::script::ShellIR> {
        let source = self.source.as_str();
        let target = self.target.as_str();
        sh!(if cmd("test", "-f", dynamic!(source)) {
            cmd("cp", "-f", dynamic!(source), dynamic!(target));
        } else {
            cmd("true");
        })
    }
}

impl IntoCommand for FileCopyBuilder<'_> {
    fn build_str(&self) -> String {
        let safe_src = shell_single_quote(&self.source);
        let safe_dst = shell_single_quote(&self.target);
        if self.force {
            self.script().build_str()
        } else {
            format!("cp {} {}", safe_src, safe_dst)
        }
    }
}
