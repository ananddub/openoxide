use crate::exec::CommandExecutor;
use crate::exec::script::IntoCommand;

pub mod action;
pub mod builder;
pub mod command;

pub use action::ProcessActionBuilder;
pub use builder::ProcessBuilder;
pub use command::ProcessCommandBuilder;

pub struct ProcessCli<'a> {
    pub(crate) executor: &'a CommandExecutor,
}

impl<'a> ProcessCli<'a> {
    pub fn list(&self) -> ProcessCommandBuilder<'a> {
        ProcessCommandBuilder::new(self.executor, "ps", vec!["-ef".to_string()])
    }
    pub fn find(&self, name: impl IntoCommand) -> ProcessCommandBuilder<'a> {
        ProcessCommandBuilder::new(
            self.executor,
            "pgrep",
            vec!["-f".to_string(), name.build_str()],
        )
    }
    pub fn kill_pid(&self, pid: impl IntoCommand) -> ProcessCommandBuilder<'a> {
        ProcessCommandBuilder::new(
            self.executor,
            "kill",
            vec!["-9".to_string(), pid.build_str()],
        )
    }
    pub fn signal_pid(
        &self,
        pid: impl IntoCommand,
        sig: impl IntoCommand,
    ) -> ProcessCommandBuilder<'a> {
        ProcessCommandBuilder::new(
            self.executor,
            "kill",
            vec![format!("-{}", sig.build_str()), pid.build_str()],
        )
    }
    pub fn pid(&self, name: impl IntoCommand) -> ProcessCommandBuilder<'a> {
        ProcessCommandBuilder::new(self.executor, "pidof", vec![name.build_str()])
    }
    pub fn running(&self, name: impl IntoCommand) -> ProcessCommandBuilder<'a> {
        ProcessCommandBuilder::new(self.executor, "pgrep", vec!["-x".into(), name.build_str()])
    }
    pub fn wait(&self, pid: impl IntoCommand) -> ProcessCommandBuilder<'a> {
        ProcessCommandBuilder::new(self.executor, "wait", vec![pid.build_str()])
    }
    pub fn priority_pid(&self, pid: impl IntoCommand) -> ProcessCommandBuilder<'a> {
        ProcessCommandBuilder::new(
            self.executor,
            "ps",
            vec!["-o".into(), "ni=".into(), "-p".into(), pid.build_str()],
        )
    }
    pub fn children(&self, pid: impl IntoCommand) -> ProcessCommandBuilder<'a> {
        ProcessCommandBuilder::new(
            self.executor,
            "pgrep",
            vec!["-P".to_string(), pid.build_str()],
        )
    }
    pub fn parent(&self, pid: impl IntoCommand) -> ProcessCommandBuilder<'a> {
        ProcessCommandBuilder::new(
            self.executor,
            "ps",
            vec![
                "-o".to_string(),
                "ppid=".to_string(),
                "-p".to_string(),
                pid.build_str(),
            ],
        )
    }
    pub fn env(&self, pid: impl IntoCommand) -> ProcessCommandBuilder<'a> {
        ProcessCommandBuilder::new(
            self.executor,
            "cat",
            vec![format!("/proc/{}/environ", pid.build_str())],
        )
    }
    pub fn cwd(&self, pid: impl IntoCommand) -> ProcessCommandBuilder<'a> {
        ProcessCommandBuilder::new(
            self.executor,
            "readlink",
            vec![format!("/proc/{}/cwd", pid.build_str())],
        )
    }
    pub fn process(&self, pid_or_name: impl IntoCommand) -> ProcessBuilder<'a> {
        ProcessBuilder::new(self.executor, pid_or_name)
    }
}
