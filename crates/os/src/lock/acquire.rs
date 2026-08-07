use crate::exec::script::{IntoCommand, sh};
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};

pub struct LockAcquireBuilder<'a> {
    executor: &'a CommandExecutor,
    name: String,
    lock_dir: String,
    sleep_seconds: f64,
}

impl<'a> LockAcquireBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, name: impl IntoCommand) -> Self {
        Self {
            executor,
            name: name.build_str(),
            lock_dir: "/tmp".to_string(),
            sleep_seconds: 0.5,
        }
    }
    pub fn lock_dir(mut self, path: impl Into<String>) -> Self {
        self.lock_dir = path.into();
        self
    }

    pub fn sleep_seconds(mut self, seconds: f64) -> Self {
        self.sleep_seconds = seconds;
        self
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.script().execute(self.executor).await
    }

    fn script(&self) -> Vec<crate::exec::script::ShellIR> {
        let lock_dir = self.lock_dir.as_str();
        let name = self.name.as_str();
        let sleep_seconds = self.sleep_seconds.to_string();
        sh!(while cmd(
            "mkdir",
            word![dynamic!(lock_dir), "/rustploy_lock_", dynamic!(name)]
        )
        .stderr(crate::exec::script::dsl::OutputTarget::Null)
        .failure()
        {
            sleep(rust!(sleep_seconds.as_str()));
        })
    }
}

impl<'a> IntoCommand for LockAcquireBuilder<'a> {
    fn build_str(&self) -> String {
        self.script().build_str()
    }
}
