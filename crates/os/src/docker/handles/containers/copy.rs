use crate::docker::{DockerCli, DockerOutput, DockerResult, core::ArgBuilder};
use tokio::io::AsyncWriteExt;

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
    api: bool,
}

impl<'a> ContainerCopyBuilder<'a> {
    pub(crate) fn upload(cli: &'a DockerCli, container: String, destination: String) -> Self {
        Self {
            cli,
            container,
            path: destination,
            direction: CopyDirection::ToContainer,
            api: false,
        }
    }
    pub(crate) fn download(cli: &'a DockerCli, container: String, source: String) -> Self {
        Self {
            cli,
            container,
            path: source,
            direction: CopyDirection::FromContainer,
            api: false,
        }
    }
    pub fn api(mut self) -> Self {
        self.api = true;
        self
    }
    pub fn from_host(self, source: impl Into<String>) -> DockerCopyCommand<'a> {
        DockerCopyCommand {
            cli: self.cli,
            args: self.args(source.into()),
            api: self.api,
        }
    }
    pub fn to_host(self, destination: impl Into<String>) -> DockerCopyCommand<'a> {
        DockerCopyCommand {
            cli: self.cli,
            args: self.args(destination.into()),
            api: self.api,
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
    api: bool,
}
impl DockerCopyCommand<'_> {
    pub fn api(mut self) -> Self {
        self.api = true;
        self
    }
    pub fn build_command_args(&self) -> Vec<String> {
        self.args.clone()
    }
    pub fn print(&self) -> String {
        self.args.join(" ")
    }
    pub async fn run(self) -> DockerResult<DockerOutput> {
        if self.api {
            return self.run_api().await;
        }
        self.cli.run(self.args).await
    }

    pub async fn download_bytes(self) -> DockerResult<Vec<u8>> {
        if self.api {
            let archive = self.download_archive().await?;
            return extract_single_tar_file(&archive);
        }
        let output = self.cli.run_bytes(self.args).await?;
        Ok(extract_single_tar_file(&output.stdout)?)
    }

    async fn run_api(self) -> DockerResult<DockerOutput> {
        if self.args.get(1).is_some_and(|value| value.contains(':')) {
            let archive = self.download_archive().await?;
            let destination = self
                .args
                .get(2)
                .ok_or_else(|| transfer_error("copy destination missing"))?;
            if destination != "-" {
                let destination = std::path::Path::new(destination);
                tokio::fs::create_dir_all(destination).await?;
                let mut command = tokio::process::Command::new("tar");
                command
                    .args(["-xf", "-", "-C"])
                    .arg(destination)
                    .stdin(std::process::Stdio::piped())
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::piped())
                    .kill_on_drop(true);
                let mut child = command.spawn()?;
                if let Some(mut stdin) = child.stdin.take() {
                    stdin.write_all(&archive).await?;
                }
                let output = child.wait_with_output().await?;
                if !output.status.success() {
                    return Err(transfer_error(&format!(
                        "failed to extract container archive: {}",
                        String::from_utf8_lossy(&output.stderr)
                    )));
                }
            }
        } else {
            let source = self
                .args
                .get(1)
                .ok_or_else(|| transfer_error("copy source missing"))?;
            let target = self
                .args
                .get(2)
                .ok_or_else(|| transfer_error("copy target missing"))?;
            let (container, destination) = target
                .split_once(':')
                .ok_or_else(|| transfer_error("invalid container destination"))?;
            let path = std::path::Path::new(source);
            let parent = path.parent().unwrap_or(std::path::Path::new("."));
            let name = path
                .file_name()
                .ok_or_else(|| transfer_error("invalid copy source"))?;
            let archive = tokio::process::Command::new("tar")
                .arg("-C")
                .arg(parent)
                .arg("-cf")
                .arg("-")
                .arg(name)
                .kill_on_drop(true)
                .output()
                .await?;
            if !archive.status.success() {
                return Err(transfer_error("failed to archive copy source"));
            }
            self.cli
                .api_bytes_request(
                    "PUT",
                    &format!(
                        "/containers/{container}/archive?path={}",
                        crate::docker::api::encode_query_public(destination)
                    ),
                    Some(archive.stdout),
                    "application/x-tar",
                )
                .await?;
        }
        Ok(DockerOutput {
            status: crate::exec::ExecExitStatus::Api(200),
            stdout: String::new(),
            stderr: String::new(),
        })
    }

    async fn download_archive(&self) -> DockerResult<Vec<u8>> {
        let source = self
            .args
            .get(1)
            .ok_or_else(|| transfer_error("copy source missing"))?;
        let (container, path) = source
            .split_once(':')
            .ok_or_else(|| transfer_error("invalid container source"))?;
        self.cli
            .api_bytes_request(
                "GET",
                &format!(
                    "/containers/{container}/archive?path={}",
                    crate::docker::api::encode_query_public(path)
                ),
                None,
                "application/json",
            )
            .await
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
