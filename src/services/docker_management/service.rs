use auto_di::singleton;
use std::sync::Arc;

use crate::{
    api::dto::docker_management::{
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
            .system_df()
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
