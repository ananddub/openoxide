use crate::docker::DockerCli;

pub use build::ComposeBuildBuilder;
pub use config::ComposeConfigBuilder;
pub use down::DownBuilder;
pub use exec::ComposeExecBuilder;
pub use logs::ComposeLogsBuilder;
pub use ps::ComposePsBuilder;
pub use pull::ComposePullBuilder;
pub use push::ComposePushBuilder;
pub use restart::ComposeRestartBuilder;
pub use rm::ComposeRmBuilder;
pub use run::ComposeRunBuilder;
pub use start::ComposeStartBuilder;
pub use stop::ComposeStopBuilder;
pub use up::UpBuilder;

pub struct ComposeHandle<'a>(pub(crate) &'a DockerCli);

impl<'a> ComposeHandle<'a> {
    pub fn up(&self) -> UpBuilder<'_> {
        UpBuilder::new(self.0)
    }
    pub fn down(&self) -> DownBuilder<'_> {
        DownBuilder::new(self.0)
    }
    pub fn build(&self) -> ComposeBuildBuilder<'_> {
        ComposeBuildBuilder::new(self.0)
    }
    pub fn pull(&self) -> ComposePullBuilder<'_> {
        ComposePullBuilder::new(self.0)
    }
    pub fn push(&self) -> ComposePushBuilder<'_> {
        ComposePushBuilder::new(self.0)
    }
    pub fn exec(&self, service: impl Into<String>) -> ComposeExecBuilder<'_> {
        ComposeExecBuilder::new(self.0, service)
    }
    pub fn run(&self, service: impl Into<String>) -> ComposeRunBuilder<'_> {
        ComposeRunBuilder::new(self.0, service)
    }
    pub fn start(&self) -> ComposeStartBuilder<'_> {
        ComposeStartBuilder::new(self.0)
    }
    pub fn stop(&self) -> ComposeStopBuilder<'_> {
        ComposeStopBuilder::new(self.0)
    }
    pub fn rm(&self) -> ComposeRmBuilder<'_> {
        ComposeRmBuilder::new(self.0)
    }
    pub fn config(&self) -> ComposeConfigBuilder<'_> {
        ComposeConfigBuilder::new(self.0)
    }
    pub fn logs(&self, service: impl Into<String>) -> ComposeLogsBuilder<'_> {
        ComposeLogsBuilder::new(self.0, service)
    }
    pub fn logs_all(&self) -> ComposeLogsBuilder<'_> {
        ComposeLogsBuilder::all(self.0)
    }
    pub fn ps(&self) -> ComposePsBuilder<'_> {
        ComposePsBuilder::new(self.0)
    }
    pub fn restart(
        &self,
        services: impl IntoIterator<Item = impl Into<String>>,
    ) -> ComposeRestartBuilder<'_> {
        ComposeRestartBuilder::new(self.0, services)
    }
}

pub mod build;
pub mod config;
pub mod down;
pub mod exec;
pub mod logs;
pub mod ps;
pub mod pull;
pub mod push;
pub mod restart;
pub mod rm;
pub mod run;
pub mod start;
pub mod stop;
pub mod up;
