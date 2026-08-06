use crate::utils::docker::{
    DockerCli, DockerExitStatus, DockerResult, DockerStreamEvent, core::ArgBuilder,
};
use tokio::sync::mpsc;

pub struct ComposeLogsBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) args: ArgBuilder,
    service: Option<String>,
}

impl<'a> ComposeLogsBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, service: impl Into<String>) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["compose", "logs"]),
            service: Some(service.into()),
        }
    }
    pub(crate) fn all(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["compose", "logs"]),
            service: None,
        }
    }
    pub fn follow(mut self) -> Self {
        self.args.flag("--follow");
        self
    }
    pub fn tail(mut self, n: usize) -> Self {
        self.args.pair("--tail", n.to_string());
        self
    }
    pub fn timestamps(mut self) -> Self {
        self.args.flag("--timestamps");
        self
    }
    pub fn no_color(mut self) -> Self {
        self.args.flag("--no-color");
        self
    }
    pub fn since(mut self, s: impl Into<String>) -> Self {
        self.args.pair("--since", s.into());
        self
    }
    pub fn until(mut self, u: impl Into<String>) -> Self {
        self.args.pair("--until", u.into());
        self
    }
    pub fn print(&self) -> String {
        self.command().preview()
    }
    pub fn build_command_args(&self) -> Vec<String> {
        self.command().build()
    }
    pub async fn stream(
        self,
        sender: mpsc::Sender<DockerStreamEvent>,
    ) -> DockerResult<DockerExitStatus> {
        self.cli.execute_stream(&self.command(), sender).await
    }
    pub async fn output(self) -> DockerResult<crate::utils::docker::DockerOutput> {
        self.cli.execute(&self.command()).await
    }
    /// Specify an alternate compose file
    pub fn file(mut self, f: impl Into<String>) -> Self {
        self.args.insert_pair(1, "--file", f.into());
        self
    }

    /// Specify an alternate environment file
    pub fn env_file(mut self, f: impl Into<String>) -> Self {
        self.args.insert_pair(1, "--env-file", f.into());
        self
    }

    /// Specify an alternate project name
    pub fn project(mut self, p: impl Into<String>) -> Self {
        self.args.insert_pair(1, "--project-name", p.into());
        self
    }

    pub fn project_directory(mut self, directory: impl Into<String>) -> Self {
        self.args
            .insert_pair(1, "--project-directory", directory.into());
        self
    }

    /// Specify a profile to enable
    pub fn profile(mut self, p: impl Into<String>) -> Self {
        self.args.insert_pair(1, "--profile", p.into());
        self
    }

    fn command(&self) -> ArgBuilder {
        let mut args = self.args.clone();
        if let Some(service) = &self.service {
            args.push(service);
        }
        args
    }
}

crate::impl_builder_opts!(ComposeLogsBuilder);

impl crate::utils::exec::script::IntoCommand for ComposeLogsBuilder<'_> {
    fn build_str(&self) -> String {
        self.command().preview()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::docker::DockerCli;

    #[test]
    fn builds_flags_before_service_from_typed_methods() {
        let docker = DockerCli::new_local();
        let compose = docker.compose();
        let args = compose
            .logs("api")
            .file("compose.prod.yml")
            .project_directory("/srv/app")
            .project("production")
            .follow()
            .timestamps()
            .tail(250)
            .since("1h")
            .until("10m")
            .build_command_args();

        assert_eq!(args.first().map(String::as_str), Some("compose"));
        assert_eq!(args.last().map(String::as_str), Some("api"));
        assert!(args.windows(2).any(|pair| pair == ["--tail", "250"]));
        assert!(args.windows(2).any(|pair| pair == ["--since", "1h"]));
        assert!(args.windows(2).any(|pair| pair == ["--until", "10m"]));
    }

    #[test]
    fn builds_all_services_without_empty_service_argument() {
        let docker = DockerCli::new_local();
        let compose = docker.compose();
        let args = compose.logs_all().tail(100).build_command_args();
        assert_eq!(args, ["compose", "logs", "--tail", "100"]);
    }
}
