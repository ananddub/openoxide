use crate::utils::exec::CommandExecutor;
use crate::utils::exec::script::IntoCommand;

pub mod command;

pub use command::SystemCommandBuilder;

pub struct SystemCli<'a> {
    pub(crate) executor: &'a CommandExecutor,
}

impl<'a> SystemCli<'a> {
    pub fn info(&self) -> SystemCommandBuilder<'a> {
        SystemCommandBuilder::new(self.executor, "uname", vec!["-a".to_string()])
    }
    pub fn hostname(&self) -> SystemCommandBuilder<'a> {
        SystemCommandBuilder::new(self.executor, "hostname", vec![])
    }
    pub fn set_hostname(&self, name: impl IntoCommand) -> SystemCommandBuilder<'a> {
        SystemCommandBuilder::new(self.executor, "hostname", vec![name.build_str()])
    }
    pub fn kernel(&self) -> SystemCommandBuilder<'a> {
        SystemCommandBuilder::new(self.executor, "uname", vec!["-r".to_string()])
    }
    pub fn arch(&self) -> SystemCommandBuilder<'a> {
        SystemCommandBuilder::new(self.executor, "uname", vec!["-m".to_string()])
    }
    pub fn distribution(&self) -> SystemCommandBuilder<'a> {
        SystemCommandBuilder::new(self.executor, "cat", vec!["/etc/os-release".to_string()])
    }
    pub fn uptime(&self) -> SystemCommandBuilder<'a> {
        SystemCommandBuilder::new(self.executor, "uptime", vec!["-p".to_string()])
    }
    pub fn shell(&self) -> SystemCommandBuilder<'a> {
        SystemCommandBuilder::new_shell(self.executor, "echo \"$SHELL\"", vec![])
    }
    pub fn which(&self, bin: impl IntoCommand) -> SystemCommandBuilder<'a> {
        SystemCommandBuilder::new(self.executor, "which", vec![bin.build_str()])
    }
    pub fn has_command(&self, bin: impl IntoCommand) -> SystemCommandBuilder<'a> {
        SystemCommandBuilder::new(
            self.executor,
            "command",
            vec!["-v".to_string(), bin.build_str()],
        )
    }
    pub fn timezone(&self) -> SystemCommandBuilder<'a> {
        SystemCommandBuilder::new(
            self.executor,
            "timedatectl",
            vec!["show".to_string(), "--property=Timezone".to_string()],
        )
    }
    pub fn set_timezone(&self, tz: impl IntoCommand) -> SystemCommandBuilder<'a> {
        SystemCommandBuilder::new(
            self.executor,
            "timedatectl",
            vec!["set-timezone".to_string(), tz.build_str()],
        )
    }
    pub fn reboot(&self) -> SystemCommandBuilder<'a> {
        SystemCommandBuilder::new(self.executor, "reboot", vec![])
    }
    pub fn shutdown(&self) -> SystemCommandBuilder<'a> {
        SystemCommandBuilder::new(
            self.executor,
            "shutdown",
            vec!["-h".to_string(), "now".to_string()],
        )
    }
    pub fn cpu_count(&self) -> SystemCommandBuilder<'a> {
        SystemCommandBuilder::new(self.executor, "nproc", vec![])
    }
    pub fn total_memory(&self) -> SystemCommandBuilder<'a> {
        SystemCommandBuilder::new_shell(self.executor, "free -b | awk '/^Mem:/{print $2}'", vec![])
    }
    pub fn free_memory(&self) -> SystemCommandBuilder<'a> {
        SystemCommandBuilder::new_shell(self.executor, "free -b | awk '/^Mem:/{print $4}'", vec![])
    }
}
