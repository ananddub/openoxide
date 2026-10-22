use crate::docker::{DockerCli, DockerOutput, DockerResult};

pub struct SystemLoginBuilder<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) username: Option<String>,
    pub(crate) password: Option<String>,
    pub(crate) registry: Option<String>,
    api: bool,
}

impl<'a> SystemLoginBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli) -> Self {
        Self {
            cli,
            username: None,
            password: None,
            registry: None,
            api: false,
        }
    }
    pub fn api(mut self) -> Self {
        self.api = true;
        self
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
        if self.api {
            use base64::Engine;
            let auth = serde_json::json!({"username": self.username.unwrap_or_default(), "password": self.password.unwrap_or_default(), "serveraddress": self.registry.unwrap_or_default()});
            let output = self.cli.api_json_request("/auth", auth.clone()).await?;
            self.cli.set_registry_auth(
                base64::engine::general_purpose::STANDARD
                    .encode(serde_json::to_vec(&auth).unwrap()),
            );
            return Ok(output);
        }
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
