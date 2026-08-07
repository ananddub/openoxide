use crate::exec::script::{IntoCommand, sh};
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};

pub struct LockReleaseBuilder<'a> {
    executor: &'a CommandExecutor,
    name: String,
    lock_dir: String,
}

impl<'a> LockReleaseBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, name: impl IntoCommand) -> Self {
        Self {
            executor,
            name: name.build_str(),
            lock_dir: "/tmp".to_string(),
        }
    }
    pub fn lock_dir(mut self, path: impl Into<String>) -> Self {
        self.lock_dir = path.into();
        self
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.script().execute(self.executor).await
    }

    fn script(&self) -> Vec<crate::exec::script::ShellIR> {
        let lock_dir = self.lock_dir.as_str();
        let name = self.name.as_str();
        sh!(cmd("rmdir", word![dynamic!(lock_dir), "/rustploy_lock_", dynamic!(name)]);)
    }
}

impl<'a> IntoCommand for LockReleaseBuilder<'a> {
    fn build_str(&self) -> String {
        self.script().build_str()
    }
}
