use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};

pub struct WireGuardShowConfigBuilder<'a> {
    executor: &'a CommandExecutor,
    interface: String,
}

impl<'a> WireGuardShowConfigBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, interface: impl Into<String>) -> Self {
        Self {
            executor,
            interface: interface.into(),
        }
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        super::validate_interface_name(&self.interface)?;
        self.executor.run("wg", self.args()).await
    }

    pub(super) fn args(&self) -> Vec<String> {
        vec!["showconf".into(), self.interface.clone()]
    }
}
