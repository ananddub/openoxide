use auto_di::singleton;
use std::path::Path;
use std::sync::Arc;

use crate::{
    api::dto::global_op::{
        ContainerActionDto, DockerActionResponseDto, DockerPruneRequestDto, DockerPruneResponseDto,
    },
    repository::ServerRepository,
    utils::{
        docker::{ContainerSummary, DockerCli, ImageSummary, NetworkSummary, VolumeSummary},
        exec::{CommandExecutor, LocalExecutor, RemoteExecutor, SshAuth, SshHostKey},
    },
};

pub struct DockerManagementService {
    servers: Arc<ServerRepository>,
}

#[singleton]
impl DockerManagementService {
    pub async fn upload_container_file(
        &self,
        server_id: Option<i64>,
        id: &str,
        destination: &str,
        filename: &str,
        local_path: &Path,
    ) -> Result<crate::api::dto::global_op::DockerActionResponseDto, String> {
        validate_resource_name(id)?;
        validate_transfer_path(destination)?;
        validate_transfer_path(filename)?;
        let container_path = format!("{}/{}", destination.trim_end_matches('/'), filename);
        validate_transfer_path(&container_path)?;
        match server_id {
            None => self
                .docker(None)
                .await?
                .container(id)
                .upload(&container_path)
                .from_host(local_path.to_string_lossy())
                .run()
                .await
                .map(Into::into)
                .map_err(error),
            Some(server_id) => {
                let remote_path = format!(
                    "/tmp/rustploy-container-upload-{}-{filename}",
                    uuid::Uuid::new_v4()
                );
                let remote = crate::utils::upload::upload_via_rclone(
                    self.servers.as_ref(),
                    server_id,
                    local_path,
                    &remote_path,
                )
                .await?;
                let executor = CommandExecutor::Remote(remote);
                let docker = DockerCli::from_executor(executor.clone());
                let result = docker
                    .container(id)
                    .upload(&container_path)
                    .from_host(&remote_path)
                    .run()
                    .await
                    .map(Into::into)
                    .map_err(error);
                let _ = crate::utils::os::OsCli::new(&executor)
                    .file(&remote_path)
                    .delete()
                    .run()
                    .await;
                result
            }
        }
    }

    pub async fn download_container_bytes(
        &self,
        server_id: Option<i64>,
        id: &str,
        source: &str,
    ) -> Result<Vec<u8>, String> {
        validate_resource_name(id)?;
        validate_transfer_path(source)?;
        self.docker(server_id)
            .await?
            .container(id)
            .download(source)
            .read()
            .await
            .map_err(error)
    }
    fn new(servers: Arc<ServerRepository>) -> Self {
        Self { servers }
    }

    pub async fn containers(
        &self,
        server_id: Option<i64>,
    ) -> Result<Vec<ContainerSummary>, String> {
        self.docker(server_id)
            .await?
            .containers()
            .ps()
            .all()
            .list()
            .await
            .map_err(error)
    }
    pub async fn images(&self, server_id: Option<i64>) -> Result<Vec<ImageSummary>, String> {
        self.docker(server_id)
            .await?
            .images()
            .list()
            .all()
            .list()
            .await
            .map_err(error)
    }
    pub async fn networks(&self, server_id: Option<i64>) -> Result<Vec<NetworkSummary>, String> {
        self.docker(server_id)
            .await?
            .networks()
            .list()
            .list()
            .await
            .map_err(error)
    }
    pub async fn volumes(&self, server_id: Option<i64>) -> Result<Vec<VolumeSummary>, String> {
        self.docker(server_id)
            .await?
            .volumes()
            .list()
            .list()
            .await
            .map_err(error)
    }
    pub async fn container_inspect(
        &self,
        server_id: Option<i64>,
        id: &str,
    ) -> Result<serde_json::Value, String> {
        validate_resource_name(id)?;
        self.docker(server_id)
            .await?
            .container(id)
            .inspect_raw()
            .await
            .map_err(error)
    }
    pub async fn image_inspect(
        &self,
        server_id: Option<i64>,
        id: &str,
    ) -> Result<serde_json::Value, String> {
        validate_resource_name(id)?;
        self.docker(server_id)
            .await?
            .image(id)
            .inspect_raw()
            .await
            .map_err(error)
    }
    pub async fn network_inspect(
        &self,
        server_id: Option<i64>,
        id: &str,
    ) -> Result<serde_json::Value, String> {
        validate_resource_name(id)?;
        self.docker(server_id)
            .await?
            .network(id)
            .inspect_raw()
            .await
            .map_err(error)
    }
    pub async fn volume_inspect(
        &self,
        server_id: Option<i64>,
        id: &str,
    ) -> Result<serde_json::Value, String> {
        validate_resource_name(id)?;
        self.docker(server_id)
            .await?
            .volume(id)
            .inspect_raw()
            .await
            .map_err(error)
    }
    pub async fn container_action(
        &self,
        server_id: Option<i64>,
        id: &str,
        action: ContainerActionDto,
    ) -> Result<DockerActionResponseDto, String> {
        validate_resource_name(id)?;
        let docker = self.docker(server_id).await?;
        let container = docker.container(id);
        let output = match action {
            ContainerActionDto::Start => container.start().run().await,
            ContainerActionDto::Stop => container.stop().run().await,
            ContainerActionDto::Restart => container.restart().run().await,
            ContainerActionDto::Kill => container.kill().run().await,
            ContainerActionDto::Pause => container.pause().run().await,
            ContainerActionDto::Unpause => container.unpause().run().await,
        }
        .map_err(error)?;
        Ok(output.into())
    }
    pub async fn remove_container(
        &self,
        server_id: Option<i64>,
        id: &str,
        force: bool,
        volumes: bool,
    ) -> Result<DockerActionResponseDto, String> {
        validate_resource_name(id)?;
        let docker = self.docker(server_id).await?;
        let container = docker.container(id);
        let mut command = container.remove();
        if force {
            command = command.force();
        }
        if volumes {
            command = command.volumes();
        }
        command.run().await.map(Into::into).map_err(error)
    }
    pub async fn disk_usage(&self, server_id: Option<i64>) -> Result<serde_json::Value, String> {
        self.docker(server_id)
            .await?
            .system()
            .df()
            .run()
            .await
            .map(serde_json::Value::Object)
            .map_err(error)
    }
    pub async fn prune(
        &self,
        server_id: Option<i64>,
        input: DockerPruneRequestDto,
    ) -> Result<DockerPruneResponseDto, String> {
        let docker = self.docker(server_id).await?;
        let mut operations = Vec::new();
        if input.containers {
            operations.push(
                docker
                    .containers()
                    .prune()
                    .run()
                    .await
                    .map_err(error)?
                    .into(),
            );
        }
        if input.images {
            operations.push(
                docker
                    .images()
                    .prune()
                    .all()
                    .run()
                    .await
                    .map_err(error)?
                    .into(),
            );
        }
        if input.networks {
            operations.push(docker.networks().prune().run().await.map_err(error)?.into());
        }
        if input.volumes {
            operations.push(
                docker
                    .volumes()
                    .prune()
                    .all()
                    .run()
                    .await
                    .map_err(error)?
                    .into(),
            );
        }
        Ok(DockerPruneResponseDto { operations })
    }
    async fn docker(&self, server_id: Option<i64>) -> Result<DockerCli, String> {
        match server_id {
            None => Ok(DockerCli::from_executor(CommandExecutor::Local(
                LocalExecutor::new(),
            ))),
            Some(server_id) => {
                let (host, port, username, private_key, public_key) = self
                    .servers
                    .get_direct_ssh_credentials(server_id)
                    .await
                    .map_err(|error| error.to_string())?
                    .ok_or_else(|| "server or SSH key not found".to_owned())?;
                let port = u16::try_from(port).map_err(|_| "invalid SSH port".to_owned())?;
                Ok(DockerCli::from_executor(CommandExecutor::Remote(
                    RemoteExecutor::new(
                        host,
                        port,
                        username,
                        SshAuth::key_pair(private_key, public_key),
                        SshHostKey::InsecureAcceptAny,
                    )
                    .with_sudo(),
                )))
            }
        }
    }
}

impl From<crate::utils::docker::DockerOutput> for DockerActionResponseDto {
    fn from(value: crate::utils::docker::DockerOutput) -> Self {
        Self {
            stdout: value.stdout,
            stderr: value.stderr,
        }
    }
}
fn validate_resource_name(value: &str) -> Result<(), String> {
    if !value.is_empty()
        && value.len() <= 255
        && value
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || ".:_/@+-".contains(character))
    {
        Ok(())
    } else {
        Err("invalid Docker resource identifier".into())
    }
}
fn validate_transfer_path(value: &str) -> Result<(), String> {
    if value.is_empty()
        || value.len() > 4096
        || value.contains('\0')
        || value.split('/').any(|part| part == "..")
    {
        Err("invalid transfer path".into())
    } else {
        Ok(())
    }
}
fn error(error: impl std::fmt::Display) -> String {
    error.to_string()
}

#[cfg(test)]
mod tests {
    use super::validate_resource_name;
    #[test]
    fn validates_docker_resource_identifiers() {
        assert!(validate_resource_name("registry.example.com/team/app:latest").is_ok());
        assert!(validate_resource_name("container_name-1").is_ok());
        assert!(validate_resource_name("bad name").is_err());
        assert!(validate_resource_name(";shutdown").is_err());
    }
}
