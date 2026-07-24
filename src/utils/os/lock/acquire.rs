use crate::utils::exec::script::IntoCommand;
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::utils::os::escape_arg;

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
        let lock_path = format!("{}/rustploy_lock_{}", self.lock_dir, self.name);
        let cmd = format!(
            "while ! mkdir \"$1\" 2>/dev/null; do sleep {}; done",
            self.sleep_seconds
        );
        self.executor
            .run("sh", &["-c", &cmd, "dummy", &lock_path])
            .await
    }
}

impl<'a> IntoCommand for LockAcquireBuilder<'a> {
    fn build_str(&self) -> String {
        let lock_path = format!("{}/rustploy_lock_{}", self.lock_dir, self.name);
        let cmd = format!(
            "while ! mkdir \"$1\" 2>/dev/null; do sleep {}; done",
            self.sleep_seconds
        );
        format!("sh -c '{}' dummy {}", cmd, escape_arg(&lock_path))
    }
}
