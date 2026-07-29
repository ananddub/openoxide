use crate::services::deployment::DeploymentService;
use crate::utils::docker::{DockerCli, DockerStreamEvent};
use tokio::sync::mpsc;

fn find_best_matching_container<'a>(
    containers: &'a [crate::utils::docker::ContainerSummary],
    target: &str,
) -> Option<&'a crate::utils::docker::ContainerSummary> {
    let target_lower = target.to_lowercase();
    containers
        .iter()
        .find(|c| {
            let name = c.names.trim_start_matches('/').to_lowercase();
            name == target_lower
                || name.ends_with(&format!("-{target_lower}-1"))
                || name.ends_with(&format!("_{target_lower}_1"))
                || name.ends_with(&format!("-{target_lower}"))
                || name.ends_with(&format!("_{target_lower}"))
        })
        .or_else(|| {
            containers.iter().find(|c| {
                let name = c.names.trim_start_matches('/').to_lowercase();
                name.contains(&format!("-{target_lower}-"))
                    || name.contains(&format!("_{target_lower}_"))
            })
        })
        .or_else(|| {
            containers.iter().find(|c| {
                let name = c.names.trim_start_matches('/').to_lowercase();
                name.contains(&target_lower)
            })
        })
        .or_else(|| containers.first())
}

impl DeploymentService {
    pub async fn stream_docker_container_logs(
        &self,
        server_id: Option<i64>,
        target: String,
        options: Vec<String>,
    ) -> sqlx::Result<mpsc::Receiver<DockerStreamEvent>> {
        let docker = self.docker_for_server(server_id).await?;
        let mut resolved_target = target.clone();

        if let Ok(containers) = docker
            .containers()
            .ps()
            .all()
            .list()
            .await
        {
            if let Some(matched) = find_best_matching_container(&containers, &target) {
                resolved_target = matched.names.trim_start_matches('/').to_string();
            }
        }

        let handle = docker.containers();
        let mut builder = handle.logs(resolved_target).kind("container");
        if options.iter().any(|o| o == "--follow" || o == "-f") {
            builder = builder.follow();
        }
        if options.iter().any(|o| o == "--timestamps" || o == "-t") {
            builder = builder.timestamps();
        }
        if let Some(pos) = options.iter().position(|o| o == "--tail" || o == "-n") {
            if let Some(val) = options.get(pos + 1) {
                if let Ok(n) = val.parse::<usize>() {
                    builder = builder.tail(n);
                }
            }
        }

        let cmd_args = builder.build_command_args();
        Ok(spawn_docker_stream(docker, cmd_args))
    }

    pub async fn stream_docker_container_stats(
        &self,
        server_id: Option<i64>,
        target: String,
        stream: bool,
    ) -> sqlx::Result<mpsc::Receiver<DockerStreamEvent>> {
        let docker = self.docker_for_server(server_id).await?;
        let mut resolved_target = target.clone();

        if let Ok(containers) = docker
            .containers()
            .ps()
            .all()
            .list()
            .await
        {
            if let Some(matched) = find_best_matching_container(&containers, &target) {
                resolved_target = matched.names.trim_start_matches('/').to_string();
            }
        }

        let handle = docker.containers();
        let mut builder = handle.stats(resolved_target);
        if !stream {
            builder = builder.no_stream();
        }

        let cmd_args = builder.build_command_args();
        Ok(spawn_docker_stream(docker, cmd_args))
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

        // Try Swarm service label first
        let service_name = format!("{app_name}_{app_name}");
        let swarm_filter = crate::utils::docker::query::filter::ContainerFilter::Label(
            "com.docker.swarm.service.name".to_string(),
            service_name,
        );
        let swarm_containers = docker
            .containers()
            .ps()
            .filter(swarm_filter)
            .list()
            .await
            .unwrap_or_default();

        let targets = if !swarm_containers.is_empty() {
            // Swarm mode: use container IDs from label filter
            swarm_containers
                .into_iter()
                .map(|c| c.id)
                .filter(|id| !id.trim().is_empty())
                .collect::<Vec<_>>()
        } else {
            // Standalone mode: find container by name matching
            let all_containers = docker
                .containers()
                .ps()
                .list()
                .await
                .unwrap_or_default();
            if let Some(matched) = find_best_matching_container(&all_containers, &app_name) {
                vec![matched.id.clone()]
            } else {
                Vec::new()
            }
        };

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
            app_name.clone(),
        );
        let compose_containers = docker
            .containers()
            .ps()
            .filter(filter)
            .list()
            .await
            .unwrap_or_default();

        let targets = if !compose_containers.is_empty() {
            compose_containers
                .into_iter()
                .map(|c| c.id)
                .filter(|id| !id.trim().is_empty())
                .collect::<Vec<_>>()
        } else {
            // Fallback: name-based match
            let all_containers = docker
                .containers()
                .ps()
                .list()
                .await
                .unwrap_or_default();
            if let Some(matched) = find_best_matching_container(&all_containers, &app_name) {
                vec![matched.id.clone()]
            } else {
                Vec::new()
            }
        };

        Ok(spawn_stats_stream(docker, targets, stream))
    }

    pub async fn stream_database_stats(
        &self,
        database_id: i64,
        stream: bool,
    ) -> sqlx::Result<mpsc::Receiver<DockerStreamEvent>> {
        let db = self
            .repo_postgres
            .get_by_id(database_id)
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let app_name = db.app_name;
        let server_id = db.server_id;

        let docker = self.docker_for_server(server_id).await?;

        // Databases run in Swarm as: {app_name}_db service
        let swarm_service = format!("{app_name}_db");
        let swarm_filter = crate::utils::docker::query::filter::ContainerFilter::Label(
            "com.docker.swarm.service.name".to_string(),
            swarm_service,
        );
        let swarm_containers = docker
            .containers()
            .ps()
            .filter(swarm_filter)
            .list()
            .await
            .unwrap_or_default();

        let targets = if !swarm_containers.is_empty() {
            swarm_containers
                .into_iter()
                .map(|c| c.id)
                .filter(|id| !id.trim().is_empty())
                .collect::<Vec<_>>()
        } else {
            // Fallback: name-based match (standalone / non-Swarm)
            let all_containers = docker
                .containers()
                .ps()
                .list()
                .await
                .unwrap_or_default();
            if let Some(matched) = find_best_matching_container(&all_containers, &app_name) {
                vec![matched.id.clone()]
            } else {
                Vec::new()
            }
        };

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

    pub async fn list_docker_containers(
        &self,
        server_id: Option<i64>,
        all: bool,
    ) -> sqlx::Result<Vec<crate::utils::docker::ContainerSummary>> {
        let docker = self.docker_for_server(server_id).await?;
        let handle = docker.containers();
        let mut query = handle.ps();
        if all {
            query = query.all();
        }
        query
            .list()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))
    }

    pub async fn stream_docker_service_logs(
        &self,
        server_id: Option<i64>,
        target: String,
        options: Vec<String>,
    ) -> sqlx::Result<mpsc::Receiver<DockerStreamEvent>> {
        let docker = self.docker_for_server(server_id).await?;
        let logs_subcommand = "container";
        let mut resolved_target = target.clone();

        if let Ok(containers) = docker
            .containers()
            .ps()
            .all()
            .list()
            .await
        {
            if let Some(matched) = find_best_matching_container(&containers, &target) {
                resolved_target = matched.names.trim_start_matches('/').to_string();
            }
        }

        let handle = docker.containers();
        let mut builder = handle.logs(resolved_target).kind(logs_subcommand);
        if options.iter().any(|o| o == "--follow" || o == "-f") {
            builder = builder.follow();
        }
        if options.iter().any(|o| o == "--timestamps" || o == "-t") {
            builder = builder.timestamps();
        }
        if let Some(pos) = options.iter().position(|o| o == "--tail" || o == "-n") {
            if let Some(val) = options.get(pos + 1) {
                if let Ok(n) = val.parse::<usize>() {
                    builder = builder.tail(n);
                }
            }
        }

        let cmd_args = builder.build_command_args();
        Ok(spawn_docker_stream(docker, cmd_args))
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
