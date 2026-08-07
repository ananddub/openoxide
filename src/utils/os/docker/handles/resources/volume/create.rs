use crate::utils::docker::{DockerCli, DockerResult, VolumeDriver, core::ArgBuilder};

pub struct VolumeCreate<'a> {
    pub(crate) cli: &'a DockerCli,
    pub(crate) name: String,
    pub(crate) args: ArgBuilder,
}

impl<'a> VolumeCreate<'a> {
    pub(crate) fn new(cli: &'a DockerCli, name: impl Into<String>) -> Self {
        Self {
            cli,
            name: name.into(),
            args: ArgBuilder::default(),
        }
    }
    pub fn driver(mut self, d: VolumeDriver) -> Self {
        self.args.pair("--driver", d.as_str().to_string());
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
    pub fn print(&self) -> String {
        let mut a = ArgBuilder::cmd(&["volume", "create"]);
        a.push_all(self.args.clone().build());
        a.push(&self.name);
        a.preview()
    }
    pub async fn run(self) -> DockerResult<String> {
        let mut a = ArgBuilder::cmd(&["volume", "create"]);
        a.push_all(self.args.build());
        a.push(&self.name);
        Ok(self.cli.execute(&a).await?.stdout.trim().to_string())
    }
}
crate::impl_builder_opts!(VolumeCreate);
