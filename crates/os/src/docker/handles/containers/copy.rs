use crate::docker::{DockerCli, DockerOutput, DockerResult, core::ArgBuilder};

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
    pub async fn read(self) -> DockerResult<Vec<u8>> {
        match self.direction {
            CopyDirection::FromContainer => self.to_host("-").download_bytes().await,
            CopyDirection::ToContainer => Err(transfer_error("upload builder cannot read")),
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

    pub async fn download_bytes(self) -> DockerResult<Vec<u8>> {
        let output = self.cli.run_bytes(self.args).await?;
        Ok(extract_single_tar_file(&output.stdout)?)
    }
}

fn extract_single_tar_file(archive: &[u8]) -> DockerResult<Vec<u8>> {
    if archive.len() < 512 || archive[257..263] != *b"ustar\0" {
        return Err(transfer_error("docker returned invalid tar archive"));
    }
    let size = parse_octal(&archive[124..136])? as usize;
    let end = 512usize
        .checked_add(size)
        .ok_or_else(|| transfer_error("archive too large"))?;
    if end > archive.len() {
        return Err(transfer_error("truncated tar archive"));
    }
    Ok(archive[512..end].to_vec())
}

fn parse_octal(field: &[u8]) -> DockerResult<u64> {
    let text = String::from_utf8_lossy(field)
        .trim_matches(char::from(0))
        .trim()
        .to_owned();
    u64::from_str_radix(&text, 8).map_err(|_| transfer_error("invalid tar size"))
}
fn transfer_error(message: &str) -> crate::docker::DockerError {
    crate::docker::DockerError::CommandFailed {
        code: None,
        stderr: message.into(),
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
