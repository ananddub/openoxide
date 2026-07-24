use crate::utils::backup::database::destination::S3Destination;
use crate::utils::docker::query::ServiceFilter;
use crate::utils::docker::query::filter::{ContainerFilter, ContainerStatus};
use crate::utils::docker::{DockerCli, DockerError};
use crate::utils::exec::{CommandExecutor, ExecError, ExecOutput, ExecResult};
use crate::utils::rclone::{RcloneBuilder, RcloneCommand};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
pub enum VolumeServiceTarget {
    SwarmService { service_name: String },
    ComposeService { project: String, service: String },
    Standalone,
}

#[derive(Debug, Clone)]
pub struct VolumeBackup {
    pub volume_name: String,
    pub service: VolumeServiceTarget,
    pub turn_off: bool,
}

pub struct VolumeBackupRunner<'a> {
    executor: &'a CommandExecutor,
    backup: &'a VolumeBackup,
    destination: &'a S3Destination,
}

impl<'a> VolumeBackupRunner<'a> {
    pub fn new(
        executor: &'a CommandExecutor,
        backup: &'a VolumeBackup,
        destination: &'a S3Destination,
    ) -> Self {
        Self {
            executor,
            backup,
            destination,
        }
    }

    fn docker(&self) -> DockerCli {
        DockerCli::from_executor(self.executor.clone())
    }

    fn docker_err(e: DockerError) -> ExecError {
        ExecError::CommandFailed {
            code: None,
            stderr: e.to_string(),
        }
    }

    async fn swarm_stop(&self, service_name: &str) -> ExecResult<u32> {
        let json = self
            .docker()
            .services()
            .list()
            .filter(ServiceFilter::Name(service_name.into()))
            .run_json()
            .await
            .map_err(Self::docker_err)?;

        let replicas = json
            .first()
            .and_then(|s| {
                let i: u32 = s.replicas.parse().ok()?;
                Some(i)
            })
            .ok_or_else(|| ExecError::CommandFailed {
                code: Some(1),
                stderr: format!("service '{service_name}' not found or has no replicas"),
            })?;

        self.docker()
            .services()
            .scale()
            .service(service_name, 0)
            .run()
            .await
            .map_err(Self::docker_err)?;

        Ok(replicas)
    }

    async fn swarm_start(&self, service_name: &str, replicas: u32) -> ExecResult<()> {
        self.docker()
            .services()
            .scale()
            .service(service_name, replicas)
            .run()
            .await
            .map_err(Self::docker_err)?;

        Ok(())
    }

    async fn compose_stop(&self, project: &str, service: &str) -> ExecResult<String> {
        let containers = self
            .docker()
            .containers()
            .ps()
            .filter(ContainerFilter::Status(ContainerStatus::Running))
            .filter(ContainerFilter::Label(
                "com.docker.compose.project".into(),
                project.into(),
            ))
            .filter(ContainerFilter::Label(
                "com.docker.compose.service".into(),
                service.into(),
            ))
            .list()
            .await
            .map_err(Self::docker_err)?;

        let id = containers
            .into_iter()
            .next()
            .ok_or_else(|| ExecError::CommandFailed {
                code: Some(1),
                stderr: format!(
                    "no running container for compose project={project} service={service}"
                ),
            })?
            .id;

        self.docker()
            .containers()
            .stop(&id)
            .run()
            .await
            .map_err(Self::docker_err)?;

        Ok(id)
    }

    async fn compose_start(&self, container_id: &str) -> ExecResult<()> {
        self.docker()
            .containers()
            .start(container_id)
            .run()
            .await
            .map_err(Self::docker_err)?;

        Ok(())
    }

    async fn stream_to_s3(
        &self,
        object_key: &str,
        cancel: &CancellationToken,
    ) -> ExecResult<ExecOutput> {
        let vol = &self.backup.volume_name;

        let rclone_target = self.destination.to_rclone_target(object_key);

        let mut builder = RcloneBuilder::new(RcloneCommand::Rcat).destination(rclone_target);

        if let Some(ref extra) = self.destination.additional_flags {
            for flag in extra.split_whitespace() {
                builder = builder.arg(flag);
            }
        }

        let rclone_cmd = builder.to_command_string();

        let pipeline = format!(
            "set -eo pipefail; docker run --rm -v {vol}:/volume_data ubuntu tar cf - -C /volume_data . | {rclone}",
            vol = vol,
            rclone = rclone_cmd,
        );

        self.executor
            .run_cancelled("sh", ["-c", &pipeline], cancel)
            .await
    }

    async fn stream_from_s3(
        &self,
        object_key: &str,
        cancel: &CancellationToken,
    ) -> ExecResult<ExecOutput> {
        let vol = &self.backup.volume_name;

        let rclone_target = self.destination.to_rclone_target(object_key);

        let mut builder = RcloneBuilder::new(RcloneCommand::Cat).source(rclone_target);

        if let Some(ref extra) = self.destination.additional_flags {
            for flag in extra.split_whitespace() {
                builder = builder.arg(flag);
            }
        }

        let rclone_cmd = builder.to_command_string();

        let pipeline = format!(
            "set -eo pipefail; {rclone} | docker run -i --rm -v {vol}:/volume_data ubuntu tar xf - -C /volume_data",
            vol = vol,
            rclone = rclone_cmd,
        );

        self.executor
            .run_cancelled("sh", ["-c", &pipeline], cancel)
            .await
    }

    pub async fn run(
        &self,
        object_key: &str,
        cancel: &CancellationToken,
    ) -> ExecResult<ExecOutput> {
        let result = match (&self.backup.service, self.backup.turn_off) {
            (VolumeServiceTarget::SwarmService { service_name }, true) => {
                let replicas = self.swarm_stop(service_name).await?;
                let out = self.stream_to_s3(object_key, cancel).await;
                let _ = self.swarm_start(service_name, replicas).await;
                out
            }
            (VolumeServiceTarget::SwarmService { .. }, false) => {
                self.stream_to_s3(object_key, cancel).await
            }
            (VolumeServiceTarget::ComposeService { project, service }, true) => {
                let id = self.compose_stop(project, service).await?;
                let out = self.stream_to_s3(object_key, cancel).await;
                let _ = self.compose_start(&id).await;
                out
            }
            (VolumeServiceTarget::ComposeService { .. }, false) => {
                self.stream_to_s3(object_key, cancel).await
            }
            (VolumeServiceTarget::Standalone, _) => self.stream_to_s3(object_key, cancel).await,
        };

        result
    }

    pub async fn run_restore(
        &self,
        object_key: &str,
        cancel: &CancellationToken,
    ) -> ExecResult<ExecOutput> {
        let result = match (&self.backup.service, self.backup.turn_off) {
            (VolumeServiceTarget::SwarmService { service_name }, true) => {
                let replicas = self.swarm_stop(service_name).await?;
                let out = self.stream_from_s3(object_key, cancel).await;
                let _ = self.swarm_start(service_name, replicas).await;
                out
            }
            (VolumeServiceTarget::SwarmService { .. }, false) => {
                self.stream_from_s3(object_key, cancel).await
            }
            (VolumeServiceTarget::ComposeService { project, service }, true) => {
                let id = self.compose_stop(project, service).await?;
                let out = self.stream_from_s3(object_key, cancel).await;
                let _ = self.compose_start(&id).await;
                out
            }
            (VolumeServiceTarget::ComposeService { .. }, false) => {
                self.stream_from_s3(object_key, cancel).await
            }
            (VolumeServiceTarget::Standalone, _) => self.stream_from_s3(object_key, cancel).await,
        };

        result
    }
}
