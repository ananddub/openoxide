use crate::services::deployment::DeploymentService;
use crate::utils::docker::{DockerCli, DockerStreamEvent};
use tokio::sync::mpsc;

impl DeploymentService {
    pub async fn stream_docker_container_logs(
        &self,
        server_id: Option<i64>,
        target: String,
        options: Vec<String>,
    ) -> sqlx::Result<mpsc::Receiver<DockerStreamEvent>> {
        let docker = self.docker_for_server(server_id).await?;
        Ok(spawn_docker_stream(
            docker,
            docker_logs_command("container", target, options),
        ))
    }

    pub async fn stream_docker_container_stats(
        &self,
        server_id: Option<i64>,
        target: String,
        stream: bool,
    ) -> sqlx::Result<mpsc::Receiver<DockerStreamEvent>> {
        let docker = self.docker_for_server(server_id).await?;
        let mut command = vec![
            "container".into(),
            "stats".into(),
            "--format".into(),
            "{{json .}}".into(),
        ];
        if !stream {
            command.push("--no-stream".into());
        }
        command.push(target);
        Ok(spawn_docker_stream(docker, command))
    }

    pub async fn stream_application_stats(
        &self,
        application_id: i64,
        stream: bool,
    ) -> sqlx::Result<mpsc::Receiver<DockerStreamEvent>> {
        let app = self
            .repo_app
            .get_by_id(application_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        let app_name = app.app_name;
        let server_id = app.server_id;

        let docker = self.docker_for_server(server_id).await?;
        let service_name = format!("{app_name}_{app_name}");
        let filter = crate::utils::docker::query::filter::ContainerFilter::Label(
            "com.docker.swarm.service.name".to_string(),
            service_name,
        );
        let containers = docker
            .containers()
            .ps()
            .filter(filter)
            .list()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let targets = containers
            .into_iter()
            .map(|container| container.id)
            .filter(|id| !id.trim().is_empty())
            .collect::<Vec<_>>();

        Ok(spawn_stats_stream(docker, targets, stream))
    }

    pub async fn stream_compose_stats(
        &self,
        compose_id: i64,
        stream: bool,
    ) -> sqlx::Result<mpsc::Receiver<DockerStreamEvent>> {
        let compose = self
            .repo_compose
            .get_by_id(compose_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        let app_name = compose.app_name;
        let server_id = compose.server_id;

        let docker = self.docker_for_server(server_id).await?;
        let filter = crate::utils::docker::query::filter::ContainerFilter::Label(
            "com.docker.compose.project".to_string(),
            app_name,
        );
        let containers = docker
            .containers()
            .ps()
            .filter(filter)
            .list()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let targets = containers
            .into_iter()
            .map(|container| container.id)
            .filter(|id| !id.trim().is_empty())
            .collect::<Vec<_>>();

        Ok(spawn_stats_stream(docker, targets, stream))
    }

    pub async fn stream_global_stats(
        &self,
        server_id: Option<i64>,
        stream: bool,
    ) -> sqlx::Result<mpsc::Receiver<DockerStreamEvent>> {
        let docker = self.docker_for_server(server_id).await?;
        Ok(spawn_stats_stream(docker, Vec::new(), stream))
    }

    pub async fn stream_docker_service_logs(
        &self,
        server_id: Option<i64>,
        target: String,
        options: Vec<String>,
    ) -> sqlx::Result<mpsc::Receiver<DockerStreamEvent>> {
        let docker = self.docker_for_server(server_id).await?;
        let mut logs_subcommand = "service";
        let mut resolved_target = target.clone();

        if let Ok(containers) = docker
            .containers()
            .ps()
            .filter(crate::utils::docker::query::ContainerFilter::Name(target.clone()))
            .list()
            .await
        {
            if let Some(first) = containers.first() {
                resolved_target = first.names.trim_start_matches('/').to_string();
                logs_subcommand = "container";
            }
        }

        Ok(spawn_docker_stream(
            docker,
            docker_logs_command(logs_subcommand, resolved_target, options),
        ))
    }

    pub async fn stream_docker_compose_logs(
        &self,
        server_id: Option<i64>,
        args: Vec<String>,
    ) -> sqlx::Result<mpsc::Receiver<DockerStreamEvent>> {
        let docker = self.docker_for_server(server_id).await?;
        Ok(spawn_docker_stream(docker, args))
    }

    async fn docker_for_server(&self, server_id: Option<i64>) -> sqlx::Result<DockerCli> {
        match server_id {
            Some(server_id) => {
                let executor =
                    crate::services::compose::remote::remote_executor(self.db.as_ref(), server_id)
                        .await
                        .map_err(sqlx::Error::Protocol)?;
                Ok(DockerCli::from_remote_executor(executor))
            }
            None => Ok(DockerCli::new_local()),
        }
    }
}

fn spawn_stats_stream(
    docker: DockerCli,
    targets: Vec<String>,
    stream: bool,
) -> mpsc::Receiver<DockerStreamEvent> {
    let mut command = vec![
        "container".into(),
        "stats".into(),
        "--format".into(),
        "{{json .}}".into(),
    ];
    if !stream {
        command.push("--no-stream".into());
    }
    command.extend(targets);
    spawn_docker_stream(docker, command)
}

fn docker_logs_command(kind: &str, target: String, options: Vec<String>) -> Vec<String> {
    let mut command = vec![kind.into(), "logs".into()];
    command.extend(options);
    command.push(target);
    command
}

fn spawn_docker_stream(
    docker: DockerCli,
    command: Vec<String>,
) -> mpsc::Receiver<DockerStreamEvent> {
    let (sender, receiver) = mpsc::channel(128);
    let error_sender = sender.clone();

    tokio::spawn(async move {
        if let Err(error) = docker.run_stream(command, sender).await {
            let _ = error_sender
                .send(DockerStreamEvent::Stderr(
                    format!("docker stream failed: {error}\n").into_bytes(),
                ))
                .await;
        }
    });

    receiver
}
