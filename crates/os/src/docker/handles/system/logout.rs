use crate::docker::{DockerCli, DockerOutput, DockerResult};

pub struct SystemLogoutBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) registry: Option<String>,
}

impl<'a> SystemLogoutBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self { cli, registry: None }
    }

    pub fn registry(mut self, r: impl Into<String>) -> Self {
        self.registry = Some(r.into());
        self
    }

    pub async fn run(self) -> DockerResult<DockerOutput> {
        let mut args = vec!["logout"];
        if let Some(ref reg) = self.registry {
            args.push(reg.as_str());
        }
        self.cli.run(args).await
    }
}
