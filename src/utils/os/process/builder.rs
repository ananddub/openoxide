use super::action::ProcessActionBuilder;
use crate::utils::exec::CommandExecutor;
use crate::utils::exec::script::IntoCommand;

pub struct ProcessBuilder<'a> {
    executor: &'a CommandExecutor,
    pid_or_name: String,
}

impl<'a> ProcessBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, pid_or_name: impl IntoCommand) -> Self {
        Self {
            executor,
            pid_or_name: pid_or_name.build_str(),
        }
    }
    pub fn kill(self) -> ProcessActionBuilder<'a> {
        ProcessActionBuilder::new(
            self.executor,
            vec!["kill".to_string(), "-9".to_string(), self.pid_or_name],
        )
    }
    pub fn stop(self) -> ProcessActionBuilder<'a> {
        ProcessActionBuilder::new(
            self.executor,
            vec!["kill".to_string(), "-STOP".to_string(), self.pid_or_name],
        )
    }
    pub fn resume(self) -> ProcessActionBuilder<'a> {
        ProcessActionBuilder::new(
            self.executor,
            vec!["kill".to_string(), "-CONT".to_string(), self.pid_or_name],
        )
    }
    pub fn status(self) -> ProcessActionBuilder<'a> {
        ProcessActionBuilder::new(
            self.executor,
            vec!["ps".to_string(), "-p".to_string(), self.pid_or_name],
        )
    }
    pub fn priority(self) -> ProcessActionBuilder<'a> {
        ProcessActionBuilder::new_shell(
            self.executor,
            "sh -c 'ps -o ni -p \"$1\" | tail -n 1' dummy",
            vec![self.pid_or_name],
        )
    }
    pub fn set_priority(self, val: impl IntoCommand) -> ProcessActionBuilder<'a> {
        ProcessActionBuilder::new(
            self.executor,
            vec![
                "renice".to_string(),
                val.build_str(),
                "-p".to_string(),
                self.pid_or_name,
            ],
        )
    }
}
