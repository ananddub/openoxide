use crate::exec::CommandExecutor;
use crate::exec::script::IntoCommand;

pub mod command;
pub mod exists;

pub use command::SystemCommandBuilder;
pub use exists::CommandExistsBuilder;

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
    pub fn host_addresses(&self) -> SystemCommandBuilder<'a> {
        SystemCommandBuilder::new(self.executor, "hostname", vec!["-I".into()])
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
        SystemCommandBuilder::new(self.executor, "printenv", vec!["SHELL".into()])
    }
    pub fn which(&self, bin: impl IntoCommand) -> SystemCommandBuilder<'a> {
        SystemCommandBuilder::new(self.executor, "which", vec![bin.build_str()])
    }
    pub fn tool_version(&self, bin: impl IntoCommand) -> SystemCommandBuilder<'a> {
        SystemCommandBuilder::new(self.executor, &bin.build_str(), vec!["--version".into()])
    }
    pub fn current_groups(&self) -> SystemCommandBuilder<'a> {
        SystemCommandBuilder::new(self.executor, "groups", vec![])
    }
    pub fn has_command(&self, bin: impl IntoCommand) -> CommandExistsBuilder<'a> {
        CommandExistsBuilder::new(self.executor, bin)
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
        SystemCommandBuilder::new(
            self.executor,
            "awk",
            vec![
                "/MemTotal:/ { print $2 * 1024 }".into(),
                "/proc/meminfo".into(),
            ],
        )
    }
    pub fn free_memory(&self) -> SystemCommandBuilder<'a> {
        SystemCommandBuilder::new(
            self.executor,
            "awk",
            vec![
                "/MemAvailable:/ { print $2 * 1024 }".into(),
                "/proc/meminfo".into(),
            ],
        )
    }
}
