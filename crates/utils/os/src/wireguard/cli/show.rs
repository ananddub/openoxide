use crate::exec::{CommandExecutor, ExecOutput, ExecResult};

use super::{WireGuardShowField, WireGuardShowTarget};

pub struct WireGuardShowBuilder<'a> {
    executor: &'a CommandExecutor,
    target: WireGuardShowTarget,
    field: Option<WireGuardShowField>,
}

impl<'a> WireGuardShowBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, target: WireGuardShowTarget) -> Self {
        Self {
            executor,
            target,
            field: None,
        }
    }

    pub fn field(mut self, field: WireGuardShowField) -> Self {
        self.field = Some(field);
        self
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.executor.run("wg", self.args()).await
    }

    pub(super) fn args(&self) -> Vec<String> {
        let mut args = vec!["show".into(), self.target.to_string()];
        if let Some(field) = self.field {
            args.push(field.as_arg().into());
        }
        args
    }
}
