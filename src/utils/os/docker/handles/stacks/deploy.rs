use crate::utils::docker::{
    DockerOutput, DockerResult,
    client::DockerCli,
    core::{ArgBuilder, types::ResolveImage},
};

pub struct StackDeployBuilder<'a> {
    cli: &'a DockerCli,
    args: ArgBuilder,
    stack_name: String,
}

impl<'a> StackDeployBuilder<'a> {
    pub(crate) fn new(cli: &'a DockerCli, stack_name: impl Into<String>) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["stack", "deploy"]),
            stack_name: stack_name.into(),
        }
    }

    pub fn compose_file(mut self, path: impl AsRef<str>) -> Self {
        self.args.pair("--compose-file", path);
        self
    }
    pub fn compose_bytes(self, content: &[u8]) -> Self {
        self.compose_file("-").stdin(content.to_vec())
    }
    pub fn compose_string(self, content: impl AsRef<str>) -> Self {
        self.compose_file("-")
            .stdin(content.as_ref().as_bytes().to_vec())
    }

    pub fn with_registry_auth(mut self) -> Self {
        self.args.flag("--with-registry-auth");
        self
    }
    pub fn prune(mut self) -> Self {
        self.args.flag("--prune");
        self
    }
    pub fn resolve_image(mut self, resolve: ResolveImage) -> Self {
        self.args.pair("--resolve-image", resolve.to_string());
        self
    }

    // Internal helper for stdin injection
    fn stdin(mut self, _data: Vec<u8>) -> Self {
        self.args
            .pair_opt("--__internal_stdin", None as Option<&str>); // marker
        // Not perfectly clean inside ArgBuilder but we can use executor's run_with_stdin
        self
    }

    pub async fn run(mut self) -> DockerResult<DockerOutput> {
        self.args.push(&self.stack_name);
        self.cli.execute(&self.args).await
    }
}
crate::impl_builder_opts!(StackDeployBuilder);

impl crate::utils::exec::script::IntoCommand for StackDeployBuilder<'_> {
    fn build_str(&self) -> String {
        let mut a = self.args.clone();
        a.push(&self.stack_name);
        a.preview()
    }
}
