use crate::utils::docker::{DockerOutput, DockerResult, client::DockerCli, core::ArgBuilder};

pub struct SwarmUpdateBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
}

impl<'a> SwarmUpdateBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["swarm", "update"]),
        }
    }

    pub fn autolock(mut self, lock: bool) -> Self {
        self.args.pair("--autolock", lock.to_string());
        self
    }
    pub fn task_history_limit(mut self, limit: u32) -> Self {
        self.args.pair("--task-history-limit", limit.to_string());
        self
    }
    pub fn snapshot_interval(mut self, interval: u32) -> Self {
        self.args.pair("--snapshot-interval", interval.to_string());
        self
    }
    pub fn dispatcher_heartbeat(mut self, duration: impl AsRef<str>) -> Self {
        self.args.pair("--dispatcher-heartbeat", duration);
        self
    }
    pub fn arg(mut self, v: impl Into<String>) -> Self {
        self.args.push(v.into());
        self
    }

    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(SwarmUpdateBuilder);
