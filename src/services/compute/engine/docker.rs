use crate::services::compute::engine::{ComposeLogOptions, DeploymentService, DockerLogOptions};
use crate::utils::docker::{DockerCli, DockerStreamEvent};
use tokio::sync::mpsc;

/// Stats stream with nothing to watch. An empty target list must never reach
/// the Agent — it treats "no ids" as "every running container", which made
/// stopped/unresolved apps, databases and compose stacks display someone
/// else's CPU/RAM.
fn empty_stats_stream() -> mpsc::Receiver<DockerStreamEvent> {
    let (sender, receiver) = mpsc::channel(1);
    drop(sender);
    receiver
}

/// Checks a `docker ps` Labels cell for an exact key/value pair. Handles both
/// serialisations seen in the wild: the CLI's `k=v,k2=v2` string and a compact
/// JSON object emitted by other Docker versions.
fn label_matches(labels: &str, key: &str, value: &str) -> bool {
    let trimmed = labels.trim();
    if trimmed.starts_with('{') {
        return serde_json::from_str::<std::collections::HashMap<String, String>>(trimmed)
            .map(|map| map.get(key).is_some_and(|found| found == value))
            .unwrap_or(false);
    }
    trimmed
        .split(',')
        .any(|pair| pair == format!("{key}={value}"))
}

fn find_best_matching_container<'a>(
    containers: &'a [crate::utils::docker::ContainerSummary],
    target: &str,
) -> Option<&'a crate::utils::docker::ContainerSummary> {
    let target_lower = target.to_lowercase();
    let target_hyphen = target_lower.replace('_', "-");
    let target_underscore = target_lower.replace('-', "_");

    // Rank candidates by specificity instead of taking the first loose hit:
    // swarm task names (`{service}.N.{task}`) were beating their own service
    // match on iteration order, letting a compose container sharing the
    // display-name prefix (`test-76e37f1d_…`) steal a database's log stream.
    let rank = |name: &str| -> Option<u8> {
        let base = name.split('.').next().unwrap_or(name);
        let db_anchored = base.contains(&format!("{target_lower}_db"))
            || base.contains(&format!("{target_hyphen}_db"))
            || base.contains(&format!("{target_lower}-db"))
            || base.contains(&format!("{target_hyphen}-db"));
        if base == target_lower || base == target_hyphen || base == target_underscore {
            Some(0)
        } else if db_anchored {
            Some(1)
        } else if base.starts_with(&format!("{target_lower}_"))
            || base.starts_with(&format!("{target_hyphen}_"))
            || base.starts_with(&format!("{target_lower}-"))
            || base.starts_with(&format!("{target_hyphen}-"))
        {
            Some(2)
        } else if base.ends_with(&format!("-{target_lower}-1"))
            || base.ends_with(&format!("_{target_lower}_1"))
            || base.ends_with(&format!("-{target_lower}"))
            || base.ends_with(&format!("_{target_lower}"))
        {
            Some(3)
        } else if base.contains(&format!("-{target_lower}-"))
            || base.contains(&format!("-{target_hyphen}-"))
            || base.contains(&format!("_{target_lower}_"))
        {
            Some(4)
        } else if base.contains(&target_lower) || base.contains(&target_hyphen) {
            Some(5)
        } else {
            None
        }
    };

    containers
        .iter()
        .filter_map(|container| {
            let name = container.names.trim_start_matches('/');
            rank(name).map(|rank| (rank, container))
        })
        .min_by_key(|(rank, _)| *rank)
        .map(|(_, container)| container)
}

impl DeploymentService {
    pub async fn docker_container_logs(
        &self,
        server_id: Option<i64>,
        target: String,
        options: DockerLogOptions,
    ) -> sqlx::Result<crate::utils::docker::DockerOutput> {
        let docker = self.docker_for_server(server_id).await?;
        let mut resolved_target = target.clone();
        let mut is_service = false;

        if let Ok(containers) = docker.containers().ps().all().list().await {
            if let Some(matched) = find_best_matching_container(&containers, &target) {
                resolved_target = matched.names.trim_start_matches('/').to_string();
            } else {
                let candidates = [
                    format!("{}_db", target),
                    format!("{}-db", target),
                    target.clone(),
                ];
                for cand in &candidates {
                    if let Ok(services) = docker
                        .services()
                        .list()
                        .filter(crate::utils::docker::query::ServiceFilter::name(cand))
                        .run_json()
                        .await
                    {
                        if !services.is_empty() {
                            resolved_target = services[0].name.clone();
                            is_service = true;
                            break;
                        }
                    }
                }
                if !is_service
                    && (target.contains("postgres")
                        || target.contains("mysql")
                        || target.contains("redis")
                        || target.contains("mariadb")
                        || target.contains("mongo")
                        || target.contains("libsql"))
                {
                    is_service = true;
                    resolved_target = if target.ends_with("_db") || target.ends_with("-db") {
                        target.clone()
                    } else {
                        format!("{}_db", target)
                    };
                }
            }
        } else if target.contains("postgres")
            || target.contains("mysql")
            || target.contains("redis")
            || target.contains("mariadb")
            || target.contains("mongo")
            || target.contains("libsql")
        {
            is_service = true;
            resolved_target = if target.ends_with("_db") || target.ends_with("-db") {
                target.clone()
            } else {
                format!("{}_db", target)
            };
        }

        let handle = docker.containers();
        let mut builder =
            handle
                .logs(resolved_target)
                .kind(if is_service { "service" } else { "container" });
        if options.timestamps {
            builder = builder.timestamps();
        }
        builder = builder.tail(options.tail);

        builder
            .output_streams()
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))
    }

    pub async fn stream_docker_container_logs(
        &self,
        server_id: Option<i64>,
        target: String,
        options: DockerLogOptions,
    ) -> sqlx::Result<mpsc::Receiver<DockerStreamEvent>> {
        let docker = self.docker_for_server(server_id).await?;
        let mut resolved_target = target.clone();
        let mut is_service = false;

        if let Ok(containers) = docker.containers().ps().all().list().await {
            if let Some(matched) = find_best_matching_container(&containers, &target) {
                resolved_target = matched.names.trim_start_matches('/').to_string();
            } else {
                let candidates = [
                    format!("{}_db", target),
                    format!("{}-db", target),
                    target.clone(),
                ];
                for cand in &candidates {
                    if let Ok(services) = docker
                        .services()
                        .list()
                        .filter(crate::utils::docker::query::ServiceFilter::name(cand))
                        .run_json()
                        .await
                    {
                        if !services.is_empty() {
                            resolved_target = services[0].name.clone();
                            is_service = true;
                            break;
                        }
                    }
                }
                if !is_service
                    && (target.contains("postgres")
                        || target.contains("mysql")
                        || target.contains("redis")
                        || target.contains("mariadb")
                        || target.contains("mongo")
                        || target.contains("libsql"))
                {
                    is_service = true;
                    resolved_target = if target.ends_with("_db") || target.ends_with("-db") {
                        target.clone()
                    } else {
                        format!("{}_db", target)
                    };
                }
            }
        } else if target.contains("postgres")
            || target.contains("mysql")
            || target.contains("redis")
            || target.contains("mariadb")
            || target.contains("mongo")
            || target.contains("libsql")
        {
            is_service = true;
            resolved_target = if target.ends_with("_db") || target.ends_with("-db") {
                target.clone()
            } else {
                format!("{}_db", target)
            };
        }

        let handle = docker.containers();
        let mut builder =
            handle
                .logs(resolved_target)
                .kind(if is_service { "service" } else { "container" });
        if options.follow {
            builder = builder.follow();
        }
        if options.timestamps {
            builder = builder.timestamps();
        }
        builder = builder.tail(options.tail);
        if let Some(value) = options.since {
            builder = builder.since(value);
        }
        if let Some(value) = options.until {
            builder = builder.until(value);
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

        if let Ok(containers) = docker.containers().ps().all().list().await {
            if let Some(matched) = find_best_matching_container(&containers, &target) {
                resolved_target = matched.names.trim_start_matches('/').to_string();
            }
        }

        let server_id = crate::services::compute::compose::remote::agent_server_id(server_id)
            .await
            .map_err(sqlx::Error::Protocol)?;
        agent_stats_stream(server_id, vec![resolved_target], stream).await
    }

    pub async fn stream_application_stats(
        &self,
        application_id: i64,
        stream: bool,
    ) -> sqlx::Result<mpsc::Receiver<DockerStreamEvent>> {
        let app = self.queries.get_service_by_i_d(application_id).await?;
        let app_name = app.app_name;
        let server_id = app.server_id;

        let docker = self.docker_for_server(server_id).await?;

        let containers = match docker.containers().ps().list().await {
            Ok(containers) => containers,
            Err(error) => {
                tracing::warn!(server_id, %app_name, %error, "docker ps failed for application stats");
                Vec::new()
            }
        };

        let service_name = format!("{app_name}_{app_name}");
        let mut targets: Vec<String> = containers
            .iter()
            .filter(|c| label_matches(&c.labels, "com.docker.swarm.service.name", &service_name))
            .map(|c| c.id.trim().to_string())
            .filter(|id| !id.is_empty())
            .collect();
        if targets.is_empty() {
            // Standalone mode: find container by name matching.
            if let Some(matched) = find_best_matching_container(&containers, &app_name) {
                let id = matched.id.trim().to_string();
                if !id.is_empty() {
                    targets.push(id);
                }
            }
        }

        if targets.is_empty() {
            tracing::info!(
                server_id,
                %app_name,
                running = containers.len(),
                "no running container resolved for application stats"
            );
            return Ok(empty_stats_stream());
        }

        let agent_id = crate::services::compute::compose::remote::agent_server_id(server_id)
            .await
            .map_err(sqlx::Error::Protocol)?;
        agent_stats_stream(agent_id, targets, stream).await
    }

    pub async fn stream_compose_stats(
        &self,
        compose_id: i64,
        stream: bool,
    ) -> sqlx::Result<mpsc::Receiver<DockerStreamEvent>> {
        let compose = self.queries.get_service_by_i_d(compose_id).await?;
        let app_name = compose.app_name;
        let server_id = compose.server_id;

        let docker = self.docker_for_server(server_id).await?;

        let containers = match docker.containers().ps().list().await {
            Ok(containers) => containers,
            Err(error) => {
                tracing::warn!(server_id, %app_name, %error, "docker ps failed for compose stats");
                Vec::new()
            }
        };

        let mut targets: Vec<String> = containers
            .iter()
            .filter(|c| label_matches(&c.labels, "com.docker.compose.project", &app_name))
            .map(|c| c.id.trim().to_string())
            .filter(|id| !id.is_empty())
            .collect();
        if targets.is_empty() {
            // Fallback: name-based match against the compose project prefix.
            let matched: Vec<String> = containers
                .iter()
                .filter(|c| {
                    let name = c.names.trim_start_matches('/').to_lowercase();
                    name.starts_with(&format!("{}-", app_name.to_lowercase()))
                        || name.starts_with(&format!("{}_", app_name.to_lowercase()))
                })
                .map(|c| c.id.trim().to_string())
                .filter(|id| !id.is_empty())
                .collect();
            targets = matched;
        }

        if targets.is_empty() {
            if let Some(matched) = find_best_matching_container(&containers, &app_name) {
                let id = matched.id.trim().to_string();
                if !id.is_empty() {
                    targets.push(id);
                }
            }
        }

        if targets.is_empty() {
            tracing::info!(
                server_id,
                %app_name,
                running = containers.len(),
                "no running container resolved for compose stats"
            );
            return Ok(empty_stats_stream());
        }

        let agent_id = crate::services::compute::compose::remote::agent_server_id(server_id)
            .await
            .map_err(sqlx::Error::Protocol)?;
        agent_stats_stream(agent_id, targets, stream).await
    }

    pub async fn stream_database_stats(
        &self,
        database_id: i64,
        stream: bool,
    ) -> sqlx::Result<mpsc::Receiver<DockerStreamEvent>> {
        let database = self.queries.get_service_by_i_d(database_id).await?;
        let app_name = database.app_name;
        let server_id = database.server_id;

        let docker = self.docker_for_server(server_id).await?;

        let containers = match docker.containers().ps().list().await {
            Ok(containers) => containers,
            Err(error) => {
                tracing::warn!(server_id, %app_name, %error, "docker ps failed for database stats");
                Vec::new()
            }
        };

        // Databases run in Swarm as: {app_name}_db service
        let swarm_service = format!("{app_name}_db");
        let mut targets: Vec<String> = containers
            .iter()
            .filter(|c| label_matches(&c.labels, "com.docker.swarm.service.name", &swarm_service))
            .map(|c| c.id.trim().to_string())
            .filter(|id| !id.is_empty())
            .collect();
        if targets.is_empty() {
            // Fallback: name-based match (standalone / non-Swarm).
            if let Some(matched) = find_best_matching_container(&containers, &app_name) {
                let id = matched.id.trim().to_string();
                if !id.is_empty() {
                    targets.push(id);
                }
            }
        }

        if targets.is_empty() {
            tracing::info!(
                server_id,
                %app_name,
                running = containers.len(),
                "no running container resolved for database stats"
            );
            return Ok(empty_stats_stream());
        }

        let agent_id = crate::services::compute::compose::remote::agent_server_id(server_id)
            .await
            .map_err(sqlx::Error::Protocol)?;
        agent_stats_stream(agent_id, targets, stream).await
    }

    pub async fn stream_global_stats(
        &self,
        server_id: Option<i64>,
        stream: bool,
    ) -> sqlx::Result<mpsc::Receiver<DockerStreamEvent>> {
        let agent_id = crate::services::compute::compose::remote::agent_server_id(server_id)
            .await
            .map_err(sqlx::Error::Protocol)?;
        // Empty targets: the agent streams every running container.
        agent_stats_stream(agent_id, Vec::new(), stream).await
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
        options: DockerLogOptions,
    ) -> sqlx::Result<mpsc::Receiver<DockerStreamEvent>> {
        let effective_server_id = match server_id {
            Some(sid) => Some(sid),
            None => {
                let mut found_sid: Option<i64> = None;
                let db_queries =
                    ["SELECT server_id FROM services WHERE app_name = ? OR name = ? LIMIT 1"];
                for q in db_queries {
                    if let Ok(Some(row)) = sqlx::query_as::<_, (Option<i64>,)>(q)
                        .bind(&target)
                        .bind(&target)
                        .fetch_optional(self.database())
                        .await
                    {
                        if row.0.is_some() {
                            found_sid = row.0;
                            break;
                        }
                    }
                }
                found_sid
            }
        };

        let docker = self.docker_for_server(effective_server_id).await?;
        let mut resolved_target = target.clone();
        let mut is_service = false;

        if let Ok(containers) = docker.containers().ps().all().list().await {
            if let Some(matched) = find_best_matching_container(&containers, &target) {
                resolved_target = matched.names.trim_start_matches('/').to_string();
            } else {
                let candidates = [
                    format!("{}_db", target),
                    format!("{}-db", target),
                    target.clone(),
                ];
                for cand in &candidates {
                    if let Ok(services) = docker
                        .services()
                        .list()
                        .filter(crate::utils::docker::query::ServiceFilter::name(cand))
                        .run_json()
                        .await
                    {
                        if !services.is_empty() {
                            resolved_target = services[0].name.clone();
                            is_service = true;
                            break;
                        }
                    }
                }
                if !is_service
                    && (target.contains("postgres")
                        || target.contains("mysql")
                        || target.contains("redis")
                        || target.contains("mariadb")
                        || target.contains("mongo")
                        || target.contains("libsql"))
                {
                    is_service = true;
                    resolved_target = if target.ends_with("_db") || target.ends_with("-db") {
                        target.clone()
                    } else {
                        format!("{}_db", target)
                    };
                }
            }
        } else if target.contains("postgres")
            || target.contains("mysql")
            || target.contains("redis")
            || target.contains("mariadb")
            || target.contains("mongo")
            || target.contains("libsql")
        {
            is_service = true;
            resolved_target = if target.ends_with("_db") || target.ends_with("-db") {
                target.clone()
            } else {
                format!("{}_db", target)
            };
        }

        let handle = docker.containers();
        let mut builder =
            handle
                .logs(resolved_target)
                .kind(if is_service { "service" } else { "container" });
        if options.follow {
            builder = builder.follow();
        }
        if options.timestamps {
            builder = builder.timestamps();
        }
        builder = builder.tail(options.tail);
        if let Some(value) = options.since {
            builder = builder.since(value);
        }
        if let Some(value) = options.until {
            builder = builder.until(value);
        }
        let cmd_args = builder.build_command_args();
        Ok(spawn_docker_stream(docker, cmd_args))
    }

    pub async fn stream_docker_compose_logs(
        &self,
        server_id: Option<i64>,
        options: ComposeLogOptions,
    ) -> sqlx::Result<mpsc::Receiver<DockerStreamEvent>> {
        let docker = self.docker_for_server(server_id).await?;
        let compose = docker.compose();
        let mut builder = match options.service {
            Some(service) => compose.logs(service),
            None => compose.logs_all(),
        };
        if let Some(file) = options.file {
            builder = builder.file(file);
        }
        if let Some(directory) = options.project_directory {
            builder = builder.project_directory(directory);
        }
        if let Some(project) = options.project_name {
            builder = builder.project(project);
        }
        if options.logs.follow {
            builder = builder.follow();
        }
        if options.logs.timestamps {
            builder = builder.timestamps();
        }
        builder = builder.tail(options.logs.tail);
        if let Some(since) = options.logs.since {
            builder = builder.since(since);
        }
        if let Some(until) = options.logs.until {
            builder = builder.until(until);
        }
        let command = builder.build_command_args();
        Ok(spawn_docker_stream(docker, command))
    }

    pub(super) async fn docker_for_server(
        &self,
        server_id: Option<i64>,
    ) -> sqlx::Result<DockerCli> {
        crate::services::compute::compose::remote::agent_executor_for(server_id)
            .await
            .map(DockerCli::from_executor)
            .map_err(sqlx::Error::Protocol)
    }
}

/// Proxies container stats from the Agent's in-process Docker API stream and
/// repackages each frame as CLI-shaped stdout bytes, so every browser
/// consumer costs zero subprocesses (previously one `docker stats` CLI per
/// stream, ~11 MB RSS each on the Agent host).
async fn agent_stats_stream(
    server_id: i64,
    targets: Vec<String>,
    stream: bool,
) -> sqlx::Result<mpsc::Receiver<DockerStreamEvent>> {
    use auto_di::resolve;
    use tokio_stream::StreamExt;

    let monitoring = resolve::<
        crate::services::observability::monitoring::service::MonitoringService,
    >()
    .await
    .map_err(|error| sqlx::Error::Protocol(format!("monitoring Agent unavailable: {error}")))?;
    let mut frames = monitoring
        .stream_container_stats(server_id, targets, stream)
        .await
        .map_err(sqlx::Error::Protocol)?;

    let (sender, receiver) = mpsc::channel(128);
    tokio::spawn(async move {
        loop {
            match frames.next().await {
                Some(Ok(frame)) => {
                    let mut line = frame.json_line;
                    line.push('\n');
                    if sender
                        .send(DockerStreamEvent::Stdout(line.into_bytes()))
                        .await
                        .is_err()
                    {
                        break; // browser consumer went away; closes the gRPC stream
                    }
                }
                Some(Err(error)) => {
                    tracing::warn!(server_id, %error, "Agent stats stream failed");
                    break;
                }
                None => break,
            }
        }
    });
    Ok(receiver)
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
