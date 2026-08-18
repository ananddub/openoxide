use crate::exec::script::IntoCommand;
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};

pub struct CommandExistsBuilder<'a> {
    executor: &'a CommandExecutor,
    binary: String,
}

impl<'a> CommandExistsBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, binary: impl IntoCommand) -> Self {
        Self {
            executor,
            binary: binary.build_str(),
        }
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.script().execute(self.executor).await
    }

    fn script(&self) -> Vec<crate::exec::script::ShellIR> {
        let binary = self.binary.as_str();
        vec![crate::exec::script::ShellIR::Raw(format!(
            "export PATH=\"$PATH:/sbin:/usr/sbin:/usr/local/bin:/usr/local/sbin:/nix/var/nix/profiles/default/bin:~/.nix-profile/bin\"; command -v {} || [ -x /sbin/{} ] || [ -x /usr/sbin/{} ] || [ -x /usr/bin/{} ] || [ -x /usr/local/bin/{} ]",
            binary, binary, binary, binary, binary
        ))]
    }
}

impl IntoCommand for CommandExistsBuilder<'_> {
    fn build_str(&self) -> String {
        self.script().build_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exec::LocalExecutor;

    #[tokio::test]
    async fn checks_shell_builtins_and_missing_commands() {
        let executor = CommandExecutor::Local(LocalExecutor::new());
        assert!(
            CommandExistsBuilder::new(&executor, "command")
                .run()
                .await
                .is_ok()
        );
        assert!(
            CommandExistsBuilder::new(&executor, "openoxide-command-that-does-not-exist")
                .run()
                .await
                .is_err()
        );
    }

    #[test]
    fn renders_through_shell_ir() {
        let executor = CommandExecutor::Local(LocalExecutor::new());
        let command = CommandExistsBuilder::new(&executor, "docker").build_str();
        assert!(command.contains("docker"));
    }
}
