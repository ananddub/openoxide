use crate::utils::exec::script::IntoCommand;
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::utils::os::escape_arg;

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
        let lock_path = format!("{}/rustploy_lock_{}", self.lock_dir, self.name);
        self.executor.run("rmdir", &[lock_path]).await
    }
}

impl<'a> IntoCommand for LockReleaseBuilder<'a> {
    fn build_str(&self) -> String {
        let lock_path = format!("{}/rustploy_lock_{}", self.lock_dir, self.name);
        format!("rmdir {}", escape_arg(&lock_path))
    }
}
