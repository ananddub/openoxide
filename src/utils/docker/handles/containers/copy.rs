use crate::utils::docker::{DockerCli, DockerOutput, DockerResult, core::ArgBuilder};

#[derive(Clone, Copy)]
enum CopyDirection {
    ToContainer,
    FromContainer,
}

pub struct ContainerCopyBuilder<'a> {
    cli: &'a DockerCli,
    container: String,
    path: String,
    direction: CopyDirection,
}

impl<'a> ContainerCopyBuilder<'a> {
    pub(crate) fn upload(cli: &'a DockerCli, container: String, destination: String) -> Self {
        Self {
            cli,
            container,
            path: destination,
            direction: CopyDirection::ToContainer,
        }
    }
    pub(crate) fn download(cli: &'a DockerCli, container: String, source: String) -> Self {
        Self {
            cli,
            container,
            path: source,
            direction: CopyDirection::FromContainer,
        }
    }
    pub fn from_host(self, source: impl Into<String>) -> DockerCopyCommand<'a> {
        DockerCopyCommand {
            cli: self.cli,
            args: self.args(source.into()),
        }
    }
    pub fn to_host(self, destination: impl Into<String>) -> DockerCopyCommand<'a> {
        DockerCopyCommand {
            cli: self.cli,
            args: self.args(destination.into()),
        }
    }
    fn args(&self, other: String) -> Vec<String> {
        let mut args = ArgBuilder::cmd(&["cp"]);
        match self.direction {
            CopyDirection::ToContainer => {
                args.push(other);
                args.push(format!("{}:{}", self.container, self.path));
            }
            CopyDirection::FromContainer => {
                args.push(format!("{}:{}", self.container, self.path));
                args.push(other);
            }
        }
        args.build()
    }
}

pub struct DockerCopyCommand<'a> {
    cli: &'a DockerCli,
    args: Vec<String>,
}
impl DockerCopyCommand<'_> {
    pub fn build_command_args(&self) -> Vec<String> {
        self.args.clone()
    }
    pub fn print(&self) -> String {
        self.args.join(" ")
    }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        self.cli.run(self.args).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_typed_copy_directions() {
        let docker = DockerCli::new_local();
        let upload = docker
            .container("web")
            .upload("/app/config.toml")
            .from_host("/tmp/config.toml")
            .build_command_args();
        assert_eq!(upload, ["cp", "/tmp/config.toml", "web:/app/config.toml"]);

        let download = docker
            .container("web")
            .download("/app/log.txt")
            .to_host("/tmp/log.txt")
            .build_command_args();
        assert_eq!(download, ["cp", "web:/app/log.txt", "/tmp/log.txt"]);
    }
}
