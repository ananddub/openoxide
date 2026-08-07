use crate::utils::docker::{DockerCli, DockerOutput, DockerResult};

pub struct SystemLoginBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) username: Option<String>,
    pub(crate) password: Option<String>,
    pub(crate) registry: Option<String>,
}

impl<'a> SystemLoginBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            username: None,
            password: None,
            registry: None,
        }
    }

    pub fn username(mut self, u: impl Into<String>) -> Self {
        self.username = Some(u.into());
        self
    }

    pub fn password(mut self, p: impl Into<String>) -> Self {
        self.password = Some(p.into());
        self
    }

    pub fn registry(mut self, r: impl Into<String>) -> Self {
        self.registry = Some(r.into());
        self
    }

    pub async fn run(self) -> DockerResult<DockerOutput> {
        let mut args = vec!["login"];
        let u = self.username.unwrap_or_default();
        if !u.is_empty() {
            args.extend(["--username", &u]);
        }
        if self.password.is_some() {
            args.push("--password-stdin");
        }
        let reg = self.registry.clone();
        if let Some(r) = &reg {
            args.push(r.as_str());
        }
        if let Some(pass) = self.password {
            self.cli.run_with_stdin(args, pass).await
        } else {
            self.cli.run(args).await
        }
    }
}
