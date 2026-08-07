use super::ContainerRuntime;
use crate::exec::script::IntoCommand;
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::escape_arg;

pub struct NvidiaConfigureBuilder<'a> {
    executor: &'a CommandExecutor,
    runtime: Option<ContainerRuntime>,
    config_path: Option<String>,
    runtime_name: Option<String>,
    set_as_default: bool,
    cdi_enabled: Option<bool>,
}
impl<'a> NvidiaConfigureBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor) -> Self {
        Self {
            executor,
            runtime: None,
            config_path: None,
            runtime_name: None,
            set_as_default: false,
            cdi_enabled: None,
        }
    }
    pub fn runtime(mut self, runtime: ContainerRuntime) -> Self {
        self.runtime = Some(runtime);
        self
    }
    pub fn config_path(mut self, path: impl Into<String>) -> Self {
        self.config_path = Some(path.into());
        self
    }
    pub fn runtime_name(mut self, name: impl Into<String>) -> Self {
        self.runtime_name = Some(name.into());
        self
    }
    pub fn set_as_default(mut self) -> Self {
        self.set_as_default = true;
        self
    }
    pub fn enable_cdi(mut self) -> Self {
        self.cdi_enabled = Some(true);
        self
    }
    pub fn disable_cdi(mut self) -> Self {
        self.cdi_enabled = Some(false);
        self
    }
    fn args(&self) -> Vec<String> {
        let mut args = vec!["runtime".into(), "configure".into()];
        if let Some(runtime) = self.runtime {
            args.push(format!("--runtime={}", runtime.as_str()));
        }
        if let Some(path) = &self.config_path {
            args.push(format!("--config={path}"));
        }
        if let Some(name) = &self.runtime_name {
            args.push(format!("--nvidia-runtime-name={name}"));
        }
        if self.set_as_default {
            args.push("--set-as-default".into());
        }
        if let Some(enabled) = self.cdi_enabled {
            args.push(format!("--cdi.enabled={enabled}"));
        }
        args
    }
    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.executor.run("nvidia-ctk", self.args()).await
    }
}
impl IntoCommand for NvidiaConfigureBuilder<'_> {
    fn build_str(&self) -> String {
        format!(
            "nvidia-ctk {}",
            self.args()
                .iter()
                .map(escape_arg)
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}
