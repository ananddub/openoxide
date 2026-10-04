use super::{destination::S3Destination, dumper::DatabaseDumper};
use crate::utils::docker::query::filter::{ContainerFilter, ContainerStatus};
use crate::utils::docker::{DockerCli, DockerError};
use crate::utils::exec::script::IntoCommand;
use crate::utils::exec::script::sh;
use crate::utils::exec::{CommandExecutor, ExecError, ExecOutput, ExecResult, ScriptPipeline};
use crate::utils::rclone::{RcloneBuilder, RcloneCommand};
use tokio_util::sync::CancellationToken;

pub struct BackupRunner<'a> {
    executor: &'a CommandExecutor,
    dumper: &'a DatabaseDumper,
    destination: &'a S3Destination,
}

impl<'a> BackupRunner<'a> {
    pub fn new(
        executor: &'a CommandExecutor,
        dumper: &'a DatabaseDumper,
        destination: &'a S3Destination,
    ) -> Self {
        Self {
            executor,
            dumper,
            destination,
        }
    }

    fn docker(&self) -> DockerCli {
        DockerCli::from_executor(self.executor.clone())
    }

    async fn resolve_container_id(&self) -> ExecResult<String> {
        let service_name = self.dumper.service_name();

        let label_key = self.dumper.container_label();

        let containers = self
            .docker()
            .containers()
            .ps()
            .filter(ContainerFilter::Status(ContainerStatus::Running))
            .filter(ContainerFilter::Label(
                label_key.into(),
                service_name.into(),
            ))
            .list()
            .await
            .map_err(docker_to_exec_error)?;

        let container = containers
            .into_iter()
            .next()
            .ok_or_else(|| ExecError::CommandFailed {
                code: Some(1),
                stderr: format!("no running container found for service '{service_name}'"),
            })?;

        Ok(container.id)
    }

    async fn verify_connection(&self, container_id: &str) -> ExecResult<()> {
        let check_script = self.dumper.connection_check_command();

        let docker = self.docker();
        let containers = docker.containers();
        let mut exec = containers.exec(container_id).interactive();

        if let Some((key, value)) = self.dumper.secret_env() {
            exec = exec.env(key, value);
        }

        let out = exec
            .run(["sh", "-c", &check_script])
            .await
            .map_err(docker_to_exec_error)?;

        if !out.success() {
            return Err(ExecError::CommandFailed {
                code: out.status.code(),
                stderr: format!("connection check failed: {}", out.stderr),
            });
        }
        Ok(())
    }

    /// `docker exec -i <id> sh -c '<inner>' | rclone rcat <target>` written as
    /// a shell pipeline with `sh!`: the exec step and the rclone step are both
    /// typed builders rendered with full per-token quoting (Gates G2/G3), and
    /// `pipefail` makes any stage failure abort the whole stream.
    pub async fn run(
        &self,
        object_key: &str,
        cancel: &CancellationToken,
    ) -> ExecResult<ExecOutput> {
        let container_id = self.resolve_container_id().await?;

        self.verify_connection(&container_id).await?;

        let inner_cmd = self.dumper.inner_dump_command(self.executor);

        let rclone_target = self.destination.to_rclone_target(object_key);

        let rclone = RcloneBuilder::new(RcloneCommand::Rcat).destination(rclone_target);

        let docker = self.docker();
        let containers = docker.containers();
        let mut exec = containers
            .exec(&container_id)
            .interactive()
            .command(["sh", "-c", &inner_cmd]);

        if let Some((key, value)) = self.dumper.secret_env() {
            exec = exec.env(key, value);
        }

        let script = sh!(pipe![exec, rclone];);

        ScriptPipeline::new()
            .pipefail(true)
            .cmd(script)
            .execute_cancelled(self.executor, cancel)
            .await
    }

    pub async fn run_restore(
        &self,
        object_key: &str,
        cancel: &CancellationToken,
    ) -> ExecResult<ExecOutput> {
        let container_id = self.resolve_container_id().await?;

        self.verify_connection(&container_id).await?;

        let inner_cmd = self.dumper.inner_restore_command(self.executor);

        let rclone_target = self.destination.to_rclone_target(object_key);

        let rclone = RcloneBuilder::new(RcloneCommand::Cat).source(rclone_target);

        let docker = self.docker();
        let containers = docker.containers();
        let mut exec = containers
            .exec(&container_id)
            .interactive()
            .command(["sh", "-c", &inner_cmd]);

        if let Some((key, value)) = self.dumper.secret_env() {
            exec = exec.env(key, value);
        }

        let script = sh!(pipe![rclone, exec];);

        ScriptPipeline::new()
            .pipefail(true)
            .cmd(script)
            .execute_cancelled(self.executor, cancel)
            .await
    }
}

fn docker_to_exec_error(e: DockerError) -> ExecError {
    ExecError::CommandFailed {
        code: None,
        stderr: e.to_string(),
    }
}
