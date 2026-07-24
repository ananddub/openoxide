use crate::utils::docker::{
    DockerCli, DockerResult, NetworkDriver, NetworkScope, core::ArgBuilder,
};

pub struct NetworkCreate<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) name: String,
    pub(crate) args: ArgBuilder,
}

impl<'a> NetworkCreate<'a> {
    pub(crate) fn new(cli: &'a DockerCli, name: impl Into<String>) -> Self {
        Self {
            cli,
            name: name.into(),
            args: ArgBuilder::default(),
        }
    }
    pub fn driver(mut self, d: NetworkDriver) -> Self {
        self.args.pair("--driver", d.as_str().to_string());
        self
    }
    pub fn subnet(mut self, v: impl Into<String>) -> Self {
        self.args.pair("--subnet", v.into());
        self
    }
    pub fn gateway(mut self, v: impl Into<String>) -> Self {
        self.args.pair("--gateway", v.into());
        self
    }
    pub fn label(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self {
        self.args.label(k, v);
        self
    }
    pub fn opt(mut self, k: impl AsRef<str>, v: impl AsRef<str>) -> Self {
        self.args
            .pair("--opt", format!("{}={}", k.as_ref(), v.as_ref()));
        self
    }
    pub fn attachable(mut self) -> Self {
        self.args.flag("--attachable");
        self
    }
    pub fn internal(mut self) -> Self {
        self.args.flag("--internal");
        self
    }
    pub fn ipv6(mut self) -> Self {
        self.args.flag("--ipv6");
        self
    }
    pub fn scope(mut self, s: NetworkScope) -> Self {
        self.args.pair("--scope", s.to_string());
        self
    }
    pub fn print(&self) -> String {
        let mut a = ArgBuilder::cmd(&["network", "create"]);
        a.push_all(self.args.clone().build());
        a.push(&self.name);
        a.preview()
    }
    pub async fn run(self) -> DockerResult<String> {
        let mut a = ArgBuilder::cmd(&["network", "create"]);
        a.push_all(self.args.build());
        a.push(&self.name);
        Ok(self.cli.execute(&a).await?.stdout.trim().to_string())
    }
}
crate::impl_builder_opts!(NetworkCreate);
