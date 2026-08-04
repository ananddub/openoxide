use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::services::compose::ComposeType;
use crate::services::schedule::types::{ScheduleAction, ScheduleType, ShellType};
use crate::utils::docker::query::ContainerFilter;
use crate::{
    api::dto::schedule::{CreateScheduleDto, PatchScheduleDto},
    db::models::schedules::Schedule,
    db::models::types::{BackupsDatabaseTypeEnum, VolumeBackupsServiceTypeEnum},
    repository::{
        ApplicationRepository, BackupRepository, ComposeProjectRepository, DestinationRepository,
        LibsqlRepository, MariadbRepository, MongoRepository, MysqlRepository, PostgresRepository,
        RedisRepository, ScheduleRepository, VolumeBackupRepository,
    },
    services::{
        application::{ApplicationOperation, ApplicationService},
        compose::{ComposeOperation, ComposeService, remote::remote_executor},
    },
    utils::{
        docker::DockerCli,
        exec::{CommandExecutor, LocalExecutor},
    },
};

#[derive(Debug, Clone)]
pub struct ScheduleRunResult {
    pub schedule: Schedule,
    pub action: String,
    pub deployment_id: Option<i64>,
    pub message: String,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}

pub struct ScheduleService {
    pub db: Arc<SqlitePool>,
    pub applications: Arc<ApplicationService>,
    pub compose: Arc<ComposeService>,
    pub repo_schedule: Arc<ScheduleRepository>,
    pub repo_app: Arc<ApplicationRepository>,
    pub repo_compose: Arc<ComposeProjectRepository>,
    pub repo_backup: Arc<BackupRepository>,
    pub repo_volume_backup: Arc<VolumeBackupRepository>,
    pub repo_destination: Arc<DestinationRepository>,
    pub repo_postgres: Arc<PostgresRepository>,
    pub repo_mysql: Arc<MysqlRepository>,
    pub repo_mariadb: Arc<MariadbRepository>,
    pub repo_mongo: Arc<MongoRepository>,
    pub repo_redis: Arc<RedisRepository>,
    pub repo_libsql: Arc<LibsqlRepository>,
}

#[singleton]
impl ScheduleService {
    fn new(
        db: Arc<SqlitePool>,
        applications: Arc<ApplicationService>,
        compose: Arc<ComposeService>,
        repo_schedule: Arc<ScheduleRepository>,
        repo_app: Arc<ApplicationRepository>,
        repo_compose: Arc<ComposeProjectRepository>,
        repo_backup: Arc<BackupRepository>,
        repo_volume_backup: Arc<VolumeBackupRepository>,
        repo_destination: Arc<DestinationRepository>,
        repo_postgres: Arc<PostgresRepository>,
        repo_mysql: Arc<MysqlRepository>,
        repo_mariadb: Arc<MariadbRepository>,
        repo_mongo: Arc<MongoRepository>,
        repo_redis: Arc<RedisRepository>,
        repo_libsql: Arc<LibsqlRepository>,
    ) -> Self {
        Self {
            db,
            applications,
            compose,
            repo_schedule,
            repo_app,
            repo_compose,
            repo_backup,
            repo_volume_backup,
            repo_destination,
            repo_postgres,
            repo_mysql,
            repo_mariadb,
            repo_mongo,
            repo_redis,
            repo_libsql,
        }
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<Schedule> {
        self.repo_schedule
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn list_by_application(&self, application_id: i64) -> sqlx::Result<Vec<Schedule>> {
        self.repo_schedule.list_by_application(application_id).await
    }

    pub async fn list_by_compose(&self, compose_id: i64) -> sqlx::Result<Vec<Schedule>> {
        self.repo_schedule.list_by_compose(compose_id).await
    }

    pub async fn list_by_server(&self, server_id: i64) -> sqlx::Result<Vec<Schedule>> {
        self.repo_schedule.list_by_server(server_id).await
    }

    pub async fn list_by_organization(&self, organization_id: i64) -> sqlx::Result<Vec<Schedule>> {
        self.repo_schedule
            .list_by_organization(organization_id)
            .await
    }

    pub async fn list_enabled(&self) -> sqlx::Result<Vec<Schedule>> {
        self.repo_schedule.list_enabled().await
    }

    pub async fn create(&self, input: CreateScheduleDto) -> sqlx::Result<Schedule> {
        let shell_type = normalize_shell_type(input.shell_type.as_deref())?;
        let schedule_type = if let Some(st) = input
            .schedule_type
            .as_deref()
            .filter(|s| !s.trim().is_empty())
        {
            normalize_schedule_type(Some(st))?
        } else if input.compose_id.is_some() {
            "COMPOSE".to_string()
        } else if input.server_id.is_some() {
            "SERVER".to_string()
        } else {
            "APPLICATION".to_string()
        };
        let schedule_action = normalize_schedule_action(
            input.schedule_action.as_deref(),
            &schedule_type,
            &input.command,
        )?;
        let enabled = normalize_enabled(input.enabled.unwrap_or(1))?;
        validate_target(
            &schedule_type,
            &schedule_action,
            input.application_id,
            input.compose_id,
            input.server_id,
            input.service_name.as_deref(),
        )?;
        let app_name = if let Some(an) = input.app_name.as_deref().filter(|s| !s.trim().is_empty())
        {
            format!("{an}-sched-{}", uuid::Uuid::new_v4().simple())
        } else {
            generate_schedule_app_name(&input.name)
        };

        self.repo_schedule
            .create_and_return(
                input.name,
                input.description,
                input.cron_expression,
                app_name,
                input.service_name,
                shell_type,
                schedule_type,
                schedule_action,
                input.command,
                input.script,
                input.timezone,
                enabled,
                input.application_id,
                input.compose_id,
                input.server_id,
                input.organization_id,
            )
            .await
    }

    pub async fn patch(&self, id: i64, input: PatchScheduleDto) -> sqlx::Result<Schedule> {
        let current = self.get_by_id(id).await?;
        let name = input.name.unwrap_or(current.name);
        let description = input.description.or(current.description);
        let cron_expression = input.cron_expression.unwrap_or(current.cron_expression);
        let app_name = input.app_name.unwrap_or(current.app_name);
        let service_name = input.service_name.or(current.service_name);
        let shell_type = match input.shell_type {
            Some(value) => normalize_shell_type(Some(&value))?,
            None => current.shell_type,
        };
        let schedule_type = match input.schedule_type {
            Some(value) => normalize_schedule_type(Some(&value))?,
            None => current.schedule_type,
        };
        let command = input.command.unwrap_or(current.command);
        let schedule_action = match input.schedule_action {
            Some(value) => normalize_schedule_action(Some(&value), &schedule_type, &command)?,
            None => current.schedule_action,
        };
        let script = input.script.or(current.script);
        let timezone = input.timezone.or(current.timezone);
        let enabled = normalize_enabled(input.enabled.unwrap_or(current.enabled))?;
        let application_id = input.application_id.or(current.application_id);
        let compose_id = input.compose_id.or(current.compose_id);
        let server_id = input.server_id.or(current.server_id);
        let organization_id = input.organization_id.or(current.organization_id);
        validate_target(
            &schedule_type,
            &schedule_action,
            application_id,
            compose_id,
            server_id,
            service_name.as_deref(),
        )?;

        self.repo_schedule
            .update_and_return(
                id,
                name,
                description,
                cron_expression,
                app_name,
                service_name,
                shell_type,
                schedule_type,
                schedule_action,
                command,
                script,
                timezone,
                enabled,
                application_id,
                compose_id,
                server_id,
                organization_id,
            )
            .await
    }

    pub async fn set_enabled(&self, id: i64, enabled: bool) -> sqlx::Result<Schedule> {
        self.repo_schedule.set_enabled(id, enabled).await
    }

    pub async fn restore_database_backup(&self, id: i64, backup_file: &str) -> sqlx::Result<()> {
        let backup = self
            .repo_backup
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;

        let server_id = self.resolve_database_backup_server_id(&backup).await?;
        let executor = if let Some(sid) = server_id {
            CommandExecutor::Remote(
                remote_executor(self.db.as_ref(), sid)
                    .await
                    .map_err(sqlx::Error::Protocol)?,
            )
        } else {
            CommandExecutor::Local(LocalExecutor::new())
        };

        let dumper = self.resolve_database_dumper(&backup).await?;
        let dest_model = self
            .repo_destination
            .get_by_id(backup.destination_id)
            .await?
            .ok_or_else(|| sqlx::Error::Protocol("destination not found".into()))?;

        let destination = crate::utils::backup::database::S3Destination {
            access_key: dest_model.access_key,
            secret_key: dest_model.secret_access_key,
            bucket: dest_model.bucket,
            region: dest_model.region,
            endpoint: dest_model.endpoint,
            provider: Some(dest_model.provider),
            additional_flags: dest_model.additional_flags,
        };

        let runner =
            crate::utils::backup::database::BackupRunner::new(&executor, &dumper, &destination);

        let cancel = tokio_util::sync::CancellationToken::new();
        runner
            .run_restore(backup_file, &cancel)
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

        Ok(())
    }

    pub async fn restore_volume_backup(&self, id: i64, backup_file: &str) -> sqlx::Result<()> {
        let backup = self
            .repo_volume_backup
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;

        let server_id = self.resolve_volume_backup_server_id(&backup).await?;
        let executor = if let Some(sid) = server_id {
            CommandExecutor::Remote(
                remote_executor(self.db.as_ref(), sid)
                    .await
                    .map_err(sqlx::Error::Protocol)?,
            )
        } else {
            CommandExecutor::Local(LocalExecutor::new())
        };

        let target = self.resolve_volume_service_target(&backup).await?;
        let dest_model = self
            .repo_destination
            .get_by_id(backup.destination_id)
            .await?
            .ok_or_else(|| sqlx::Error::Protocol("destination not found".into()))?;

        let destination = crate::utils::backup::database::S3Destination {
            access_key: dest_model.access_key,
            secret_key: dest_model.secret_access_key,
            bucket: dest_model.bucket,
            region: dest_model.region,
            endpoint: dest_model.endpoint,
            provider: Some(dest_model.provider),
            additional_flags: dest_model.additional_flags,
        };

        let volume_backup = crate::utils::backup::volume::VolumeBackup {
            volume_name: backup.volume_name.clone(),
            service: target,
            turn_off: backup.turn_off == 1,
        };

        let runner = crate::utils::backup::volume::VolumeBackupRunner::new(
            &executor,
            &volume_backup,
            &destination,
        );

        let cancel = tokio_util::sync::CancellationToken::new();
        runner
            .run_restore(backup_file, &cancel)
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

        Ok(())
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.get_by_id(id).await?;
        self.repo_schedule.delete(id).await
    }

    pub async fn run_now(&self, id: i64) -> sqlx::Result<ScheduleRunResult> {
        let schedule = self.get_by_id(id).await?;
        if schedule.enabled == 0 {
            return Err(sqlx::Error::Protocol("schedule is disabled".into()));
        }
        let v = ScheduleType::try_from(schedule.schedule_type.as_str()).map_err(|_| {
            sqlx::Error::Protocol(format!("invalid schedule type: {}", schedule.schedule_type))
        })?;
        match v {
            ScheduleType::Application => self.run_application_schedule(schedule).await,
            ScheduleType::Compose => self.run_compose_schedule(schedule).await,
            ScheduleType::Server | ScheduleType::DokpanelServer => {
                self.run_shell_schedule(schedule).await
            }
        }
    }

    async fn run_application_schedule(
        &self,
        schedule: Schedule,
    ) -> sqlx::Result<ScheduleRunResult> {
        let application_id = schedule.application_id.ok_or_else(|| {
            sqlx::Error::Protocol("application schedule requires application_id".into())
        })?;
        if schedule.schedule_action == ScheduleAction::Exec.as_str() {
            return self
                .run_application_container_command(schedule, application_id)
                .await;
        }

        let operation = parse_application_operation(&schedule.schedule_action)?;
        let result = self
            .applications
            .run_operation(application_id, operation)
            .await?;
        Ok(ScheduleRunResult {
            schedule,
            action: operation.as_str().into(),
            deployment_id: result.deployment_id,
            message: "application operation queued".into(),
            stdout: None,
            stderr: None,
        })
    }

    async fn run_compose_schedule(&self, schedule: Schedule) -> sqlx::Result<ScheduleRunResult> {
        let compose_id = schedule
            .compose_id
            .ok_or_else(|| sqlx::Error::Protocol("compose schedule requires compose_id".into()))?;
        if schedule.schedule_action == ScheduleAction::Exec.as_str() {
            return self
                .run_compose_container_command(schedule, compose_id)
                .await;
        }

        let operation = parse_compose_operation(&schedule.schedule_action)?;
        let result = self.compose.run_operation(compose_id, operation).await?;
        Ok(ScheduleRunResult {
            schedule,
            action: operation.as_str().into(),
            deployment_id: result.deployment_id,
            message: "compose operation queued".into(),
            stdout: None,
            stderr: None,
        })
    }

    async fn run_shell_schedule(&self, schedule: Schedule) -> sqlx::Result<ScheduleRunResult> {
        let command = schedule
            .script
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .unwrap_or(schedule.command.trim());
        if command.is_empty() {
            return Err(sqlx::Error::Protocol("schedule command is empty".into()));
        }

        let executor = if let Some(server_id) = schedule.server_id {
            match remote_executor(self.db.as_ref(), server_id).await {
                Ok(remote) => CommandExecutor::Remote(remote),
                Err(err) => {
                    tracing::warn!(server_id, error = %err, "remote SSH unavailable for server, falling back to local executor");
                    CommandExecutor::Local(LocalExecutor::new())
                }
            }
        } else {
            CommandExecutor::Local(LocalExecutor::new())
        };
        let v = ShellType::try_from(schedule.shell_type.as_str()).map_err(|_| {
            sqlx::Error::Protocol(format!("invalid shell type: {}", schedule.shell_type))
        })?;
        let shell = v.executable();
        let output = executor
            .run(shell, ["-lc", command])
            .await
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        Ok(ScheduleRunResult {
            schedule,
            action: "shell".into(),
            deployment_id: None,
            message: "schedule command executed".into(),
            stdout: Some(output.stdout),
            stderr: Some(output.stderr),
        })
    }

    async fn run_application_container_command(
        &self,
        schedule: Schedule,
        application_id: i64,
    ) -> sqlx::Result<ScheduleRunResult> {
        let app = self
            .repo_app
            .get_by_id(application_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        let app_name = app.app_name;
        let server_id = app.server_id;
        // Applications are always deployed as Swarm services.
        let service_name = format!("{app_name}_{app_name}");
        self.run_swarm_container_command(schedule, server_id, &service_name)
            .await
    }

    async fn run_compose_container_command(
        &self,
        schedule: Schedule,
        compose_id: i64,
    ) -> sqlx::Result<ScheduleRunResult> {
        let compose = self
            .repo_compose
            .get_by_id(compose_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        let app_name = compose.app_name;
        let server_id = compose.server_id;
        let compose_type = compose.compose_type;

        let compose_service = schedule.service_name.clone().ok_or_else(|| {
            sqlx::Error::Protocol("compose EXEC schedule requires service_name".into())
        })?;
        let v = ComposeType::try_from(compose_type.as_str()).map_err(|_| {
            sqlx::Error::Protocol(format!("invalid compose type: {}", compose_type))
        })?;
        match v {
            ComposeType::Stack => {
                let service_name = if compose_service.starts_with(&format!("{app_name}_")) {
                    compose_service
                } else {
                    format!("{app_name}_{compose_service}")
                };
                self.run_swarm_container_command(schedule, server_id, &service_name)
                    .await
            }
            ComposeType::DockerCompose => {
                self.run_compose_project_container_command(
                    schedule,
                    server_id,
                    &app_name,
                    &compose_service,
                )
                .await
            }
        }
    }

    /// Execute command in a Swarm service container (Application or Stack).
    async fn run_swarm_container_command(
        &self,
        schedule: Schedule,
        server_id: Option<i64>,
        service_name: &str,
    ) -> sqlx::Result<ScheduleRunResult> {
        let command = schedule_command(&schedule)?;
        let docker = self.docker_for_server(server_id).await?;
        let container = docker
            .containers()
            .ps()
            .filter(ContainerFilter::label(
                "com.docker.swarm.service.name",
                service_name,
            ))
            .list()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?
            .into_iter()
            .find(|c| !c.id.trim().is_empty())
            .ok_or_else(|| {
                sqlx::Error::Protocol(format!(
                    "running container not found for swarm service {service_name}"
                ))
            })?;
        let v = ShellType::try_from(schedule.shell_type.as_str()).map_err(|_| {
            sqlx::Error::Protocol(format!("invalid shell type: {}", schedule.shell_type))
        })?;
        let shell = v.executable();
        let output = docker
            .containers()
            .exec(&container.id)
            .run([shell, "-lc", command])
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
        Ok(ScheduleRunResult {
            action: "exec".into(),
            deployment_id: None,
            message: format!("container command executed on swarm service {service_name}"),
            stdout: Some(output.stdout),
            stderr: Some(output.stderr),
            schedule,
        })
    }

    /// Execute command in a docker compose container.
    async fn run_compose_project_container_command(
        &self,
        schedule: Schedule,
        server_id: Option<i64>,
        project_name: &str,
        service_name: &str,
    ) -> sqlx::Result<ScheduleRunResult> {
        let command = schedule_command(&schedule)?;
        let docker = self.docker_for_server(server_id).await?;
        let container = docker
            .containers()
            .ps()
            .filter(ContainerFilter::label("com.docker.compose.project", project_name))
            .filter(ContainerFilter::label("com.docker.compose.service", service_name))
            .list()
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?
            .into_iter()
            .find(|c| !c.id.trim().is_empty())
            .ok_or_else(|| {
                sqlx::Error::Protocol(format!(
                    "running container not found for compose project={project_name} service={service_name}"
                ))
            })?;
        let v = ShellType::try_from(schedule.shell_type.as_str()).map_err(|_| {
            sqlx::Error::Protocol(format!("invalid shell type: {}", schedule.shell_type))
        })?;
        let shell = v.executable();
        let output = docker
            .containers()
            .exec(&container.id)
            .run([shell, "-lc", command])
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
        Ok(ScheduleRunResult {
            action: "exec".into(),
            deployment_id: None,
            message: format!("container command executed on compose {project_name}/{service_name}"),
            stdout: Some(output.stdout),
            stderr: Some(output.stderr),
            schedule,
        })
    }

    async fn docker_for_server(&self, server_id: Option<i64>) -> sqlx::Result<DockerCli> {
        if let Some(server_id) = server_id {
            if let Ok(executor) = remote_executor(self.db.as_ref(), server_id).await {
                return Ok(DockerCli::from_remote_executor(executor));
            }
        }
        Ok(DockerCli::new_local())
    }

    async fn resolve_volume_backup_server_id(
        &self,
        backup: &crate::db::models::volume_backups::VolumeBackup,
    ) -> sqlx::Result<Option<i64>> {
        if let Some(app_id) = backup.application_id {
            if let Some(app) = self.repo_app.get_by_id(app_id).await? {
                return Ok(app.server_id);
            }
        }
        if let Some(compose_id) = backup.compose_id {
            if let Some(comp) = self.repo_compose.get_by_id(compose_id).await? {
                return Ok(comp.server_id);
            }
        }
        if let Some(id) = backup.postgres_id {
            return Ok(self.repo_postgres.get_server_id_and_name(id).await?.0);
        }
        if let Some(id) = backup.mysql_id {
            return Ok(self.repo_mysql.get_server_id_and_name(id).await?.0);
        }
        if let Some(id) = backup.mariadb_id {
            return Ok(self.repo_mariadb.get_server_id_and_name(id).await?.0);
        }
        if let Some(id) = backup.mongo_id {
            return Ok(self.repo_mongo.get_server_id_and_name(id).await?.0);
        }
        if let Some(id) = backup.redis_id {
            return Ok(self.repo_redis.get_server_id_and_name(id).await?.0);
        }
        if let Some(id) = backup.libsql_id {
            return Ok(self.repo_libsql.get_server_id_and_name(id).await?.0);
        }
        Ok(None)
    }

    async fn resolve_volume_service_target(
        &self,
        backup: &crate::db::models::volume_backups::VolumeBackup,
    ) -> sqlx::Result<crate::utils::backup::volume::VolumeServiceTarget> {
        let service_type = parse_volume_service_type(&backup.service_type)
            .unwrap_or(VolumeBackupsServiceTypeEnum::Application);

        match service_type {
            VolumeBackupsServiceTypeEnum::Postgres => {
                let id = backup
                    .postgres_id
                    .ok_or_else(|| sqlx::Error::Protocol("missing postgres_id".into()))?;
                let details = self.repo_postgres.get_details(id).await?;
                Ok(
                    crate::utils::backup::volume::VolumeServiceTarget::SwarmService {
                        service_name: details.app_name,
                    },
                )
            }
            VolumeBackupsServiceTypeEnum::Mysql => {
                let id = backup
                    .mysql_id
                    .ok_or_else(|| sqlx::Error::Protocol("missing mysql_id".into()))?;
                let details = self.repo_mysql.get_details(id).await?;
                Ok(
                    crate::utils::backup::volume::VolumeServiceTarget::SwarmService {
                        service_name: details.app_name,
                    },
                )
            }
            VolumeBackupsServiceTypeEnum::Mariadb => {
                let id = backup
                    .mariadb_id
                    .ok_or_else(|| sqlx::Error::Protocol("missing mariadb_id".into()))?;
                let details = self.repo_mariadb.get_details(id).await?;
                Ok(
                    crate::utils::backup::volume::VolumeServiceTarget::SwarmService {
                        service_name: details.app_name,
                    },
                )
            }
            VolumeBackupsServiceTypeEnum::Mongo => {
                let id = backup
                    .mongo_id
                    .ok_or_else(|| sqlx::Error::Protocol("missing mongo_id".into()))?;
                let details = self.repo_mongo.get_details(id).await?;
                Ok(
                    crate::utils::backup::volume::VolumeServiceTarget::SwarmService {
                        service_name: details.app_name,
                    },
                )
            }
            VolumeBackupsServiceTypeEnum::Redis => {
                let id = backup
                    .redis_id
                    .ok_or_else(|| sqlx::Error::Protocol("missing redis_id".into()))?;
                let details = self.repo_redis.get_details(id).await?;
                Ok(
                    crate::utils::backup::volume::VolumeServiceTarget::SwarmService {
                        service_name: details.app_name,
                    },
                )
            }
            VolumeBackupsServiceTypeEnum::Libsql => {
                let id = backup
                    .libsql_id
                    .ok_or_else(|| sqlx::Error::Protocol("missing libsql_id".into()))?;
                let details = self.repo_libsql.get_details(id).await?;
                Ok(
                    crate::utils::backup::volume::VolumeServiceTarget::SwarmService {
                        service_name: details.app_name,
                    },
                )
            }
            VolumeBackupsServiceTypeEnum::Application => {
                if let Some(id) = backup.postgres_id {
                    let details = self.repo_postgres.get_details(id).await?;
                    return Ok(
                        crate::utils::backup::volume::VolumeServiceTarget::SwarmService {
                            service_name: details.app_name,
                        },
                    );
                }
                if let Some(id) = backup.mysql_id {
                    let details = self.repo_mysql.get_details(id).await?;
                    return Ok(
                        crate::utils::backup::volume::VolumeServiceTarget::SwarmService {
                            service_name: details.app_name,
                        },
                    );
                }
                if let Some(id) = backup.mariadb_id {
                    let details = self.repo_mariadb.get_details(id).await?;
                    return Ok(
                        crate::utils::backup::volume::VolumeServiceTarget::SwarmService {
                            service_name: details.app_name,
                        },
                    );
                }
                if let Some(id) = backup.mongo_id {
                    let details = self.repo_mongo.get_details(id).await?;
                    return Ok(
                        crate::utils::backup::volume::VolumeServiceTarget::SwarmService {
                            service_name: details.app_name,
                        },
                    );
                }
                if let Some(id) = backup.redis_id {
                    let details = self.repo_redis.get_details(id).await?;
                    return Ok(
                        crate::utils::backup::volume::VolumeServiceTarget::SwarmService {
                            service_name: details.app_name,
                        },
                    );
                }
                if let Some(id) = backup.libsql_id {
                    let details = self.repo_libsql.get_details(id).await?;
                    return Ok(
                        crate::utils::backup::volume::VolumeServiceTarget::SwarmService {
                            service_name: details.app_name,
                        },
                    );
                }

                let service_name = backup
                    .service_name
                    .clone()
                    .unwrap_or_else(|| format!("{}_{}", backup.app_name, backup.app_name));
                Ok(
                    crate::utils::backup::volume::VolumeServiceTarget::SwarmService {
                        service_name,
                    },
                )
            }
            VolumeBackupsServiceTypeEnum::Compose => {
                let service = backup.service_name.clone().unwrap_or_default();
                Ok(
                    crate::utils::backup::volume::VolumeServiceTarget::ComposeService {
                        project: backup.app_name.clone(),
                        service,
                    },
                )
            }
        }
    }

    async fn resolve_database_backup_server_id(
        &self,
        backup: &crate::db::models::backups::Backup,
    ) -> sqlx::Result<Option<i64>> {
        if let Some(id) = backup.postgres_id {
            return Ok(self.repo_postgres.get_server_id_and_name(id).await?.0);
        }
        if let Some(id) = backup.mysql_id {
            return Ok(self.repo_mysql.get_server_id_and_name(id).await?.0);
        }
        if let Some(id) = backup.mariadb_id {
            return Ok(self.repo_mariadb.get_server_id_and_name(id).await?.0);
        }
        if let Some(id) = backup.mongo_id {
            return Ok(self.repo_mongo.get_server_id_and_name(id).await?.0);
        }
        if let Some(id) = backup.redis_id {
            return Ok(self.repo_redis.get_server_id_and_name(id).await?.0);
        }
        if let Some(id) = backup.libsql_id {
            return Ok(self.repo_libsql.get_server_id_and_name(id).await?.0);
        }
        if let Some(compose_id) = backup.compose_id {
            if let Some(comp) = self.repo_compose.get_by_id(compose_id).await? {
                return Ok(comp.server_id);
            }
        }
        Ok(None)
    }

    async fn resolve_database_dumper(
        &self,
        backup: &crate::db::models::backups::Backup,
    ) -> sqlx::Result<crate::utils::backup::database::DatabaseDumper> {
        if let Some(id) = backup.postgres_id {
            let details = self.repo_postgres.get_details(id).await?;
            return Ok(crate::utils::backup::database::DatabaseDumper::Postgres {
                creds: crate::utils::backup::database::DbCredentials {
                    user: details.database_user,
                    password: details.database_password,
                    database: details.database_name,
                },
                target: crate::utils::backup::database::ContainerTarget {
                    service_name: details.app_name,
                },
            });
        }
        if let Some(id) = backup.mysql_id {
            let details = self.repo_mysql.get_details(id).await?;
            return Ok(crate::utils::backup::database::DatabaseDumper::Mysql {
                creds: crate::utils::backup::database::DbCredentials {
                    user: details.database_user,
                    password: details.database_password,
                    database: details.database_name,
                },
                target: crate::utils::backup::database::ContainerTarget {
                    service_name: details.app_name,
                },
            });
        }
        if let Some(id) = backup.mariadb_id {
            let details = self.repo_mariadb.get_details(id).await?;
            return Ok(crate::utils::backup::database::DatabaseDumper::MariaDb {
                creds: crate::utils::backup::database::DbCredentials {
                    user: details.database_user,
                    password: details.database_password,
                    database: details.database_name,
                },
                target: crate::utils::backup::database::ContainerTarget {
                    service_name: details.app_name,
                },
            });
        }
        if let Some(id) = backup.mongo_id {
            let details = self.repo_mongo.get_details(id).await?;
            return Ok(crate::utils::backup::database::DatabaseDumper::Mongo {
                creds: crate::utils::backup::database::DbCredentials {
                    user: details.database_user,
                    password: details.database_password,
                    database: backup.database_name.clone(),
                },
                target: crate::utils::backup::database::ContainerTarget {
                    service_name: details.app_name,
                },
            });
        }
        if let Some(id) = backup.redis_id {
            let details = self.repo_redis.get_details(id).await?;
            return Ok(crate::utils::backup::database::DatabaseDumper::Redis {
                target: crate::utils::backup::database::ContainerTarget {
                    service_name: details.app_name,
                },
            });
        }
        if let Some(id) = backup.libsql_id {
            let details = self.repo_libsql.get_details(id).await?;
            return Ok(crate::utils::backup::database::DatabaseDumper::LibSql {
                database: backup.database_name.clone(),
                target: crate::utils::backup::database::ContainerTarget {
                    service_name: details.app_name,
                },
            });
        }

        let (user, password) = if let Some(meta_str) = &backup.metadata {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(meta_str) {
                let u = json
                    .get("databaseUser")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let p = json
                    .get("databasePassword")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                (u, p)
            } else {
                ("".to_string(), "".to_string())
            }
        } else {
            ("".to_string(), "".to_string())
        };

        let service_name = backup
            .service_name
            .clone()
            .unwrap_or_else(|| backup.app_name.clone());

        let db_type = parse_database_type(&backup.database_type).ok_or_else(|| {
            sqlx::Error::Protocol(format!(
                "unsupported database type: {}",
                backup.database_type
            ))
        })?;

        match db_type {
            BackupsDatabaseTypeEnum::Postgres => {
                Ok(crate::utils::backup::database::DatabaseDumper::Postgres {
                    creds: crate::utils::backup::database::DbCredentials {
                        user,
                        password,
                        database: backup.database_name.clone(),
                    },
                    target: crate::utils::backup::database::ContainerTarget { service_name },
                })
            }
            BackupsDatabaseTypeEnum::Mysql => {
                Ok(crate::utils::backup::database::DatabaseDumper::Mysql {
                    creds: crate::utils::backup::database::DbCredentials {
                        user,
                        password,
                        database: backup.database_name.clone(),
                    },
                    target: crate::utils::backup::database::ContainerTarget { service_name },
                })
            }
            BackupsDatabaseTypeEnum::Mariadb => {
                Ok(crate::utils::backup::database::DatabaseDumper::MariaDb {
                    creds: crate::utils::backup::database::DbCredentials {
                        user,
                        password,
                        database: backup.database_name.clone(),
                    },
                    target: crate::utils::backup::database::ContainerTarget { service_name },
                })
            }
            BackupsDatabaseTypeEnum::Mongo => {
                Ok(crate::utils::backup::database::DatabaseDumper::Mongo {
                    creds: crate::utils::backup::database::DbCredentials {
                        user,
                        password,
                        database: backup.database_name.clone(),
                    },
                    target: crate::utils::backup::database::ContainerTarget { service_name },
                })
            }
            BackupsDatabaseTypeEnum::Redis => {
                Ok(crate::utils::backup::database::DatabaseDumper::Redis {
                    target: crate::utils::backup::database::ContainerTarget { service_name },
                })
            }
            BackupsDatabaseTypeEnum::Libsql => {
                Ok(crate::utils::backup::database::DatabaseDumper::LibSql {
                    database: backup.database_name.clone(),
                    target: crate::utils::backup::database::ContainerTarget { service_name },
                })
            }
        }
    }

    pub async fn run_database_backup(&self, id: i64) -> sqlx::Result<()> {
        let backup = self
            .repo_backup
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        if backup.enabled == 0 {
            return Err(sqlx::Error::Protocol("backup is disabled".into()));
        }

        let server_id = self.resolve_database_backup_server_id(&backup).await?;
        let executor = if let Some(sid) = server_id {
            CommandExecutor::Remote(
                remote_executor(self.db.as_ref(), sid)
                    .await
                    .map_err(sqlx::Error::Protocol)?,
            )
        } else {
            CommandExecutor::Local(LocalExecutor::new())
        };

        let dumper = self.resolve_database_dumper(&backup).await?;
        let dest_model = self
            .repo_destination
            .get_by_id(backup.destination_id)
            .await?
            .ok_or_else(|| sqlx::Error::Protocol("destination not found".into()))?;

        let destination = crate::utils::backup::database::S3Destination {
            access_key: dest_model.access_key,
            secret_key: dest_model.secret_access_key,
            bucket: dest_model.bucket,
            region: dest_model.region,
            endpoint: dest_model.endpoint,
            provider: Some(dest_model.provider),
            additional_flags: dest_model.additional_flags,
        };

        let runner =
            crate::utils::backup::database::BackupRunner::new(&executor, &dumper, &destination);

        let ext = dumper.file_extension();
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let object_key = format!(
            "{}/{}_{}.{}",
            backup.prefix.trim_matches('/'),
            backup.database_name,
            timestamp,
            ext
        );

        let cancel = tokio_util::sync::CancellationToken::new();
        runner
            .run(&object_key, &cancel)
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

        Ok(())
    }

    pub async fn run_volume_backup(&self, id: i64) -> sqlx::Result<()> {
        let backup = self
            .repo_volume_backup
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        if backup.enabled == 0 {
            return Err(sqlx::Error::Protocol("volume backup is disabled".into()));
        }

        let server_id = self.resolve_volume_backup_server_id(&backup).await?;
        let executor = if let Some(sid) = server_id {
            CommandExecutor::Remote(
                remote_executor(self.db.as_ref(), sid)
                    .await
                    .map_err(sqlx::Error::Protocol)?,
            )
        } else {
            CommandExecutor::Local(LocalExecutor::new())
        };

        let target = self.resolve_volume_service_target(&backup).await?;
        let dest_model = self
            .repo_destination
            .get_by_id(backup.destination_id)
            .await?
            .ok_or_else(|| sqlx::Error::Protocol("destination not found".into()))?;

        let destination = crate::utils::backup::database::S3Destination {
            access_key: dest_model.access_key,
            secret_key: dest_model.secret_access_key,
            bucket: dest_model.bucket,
            region: dest_model.region,
            endpoint: dest_model.endpoint,
            provider: Some(dest_model.provider),
            additional_flags: dest_model.additional_flags,
        };

        let volume_backup = crate::utils::backup::volume::VolumeBackup {
            volume_name: backup.volume_name.clone(),
            service: target,
            turn_off: backup.turn_off == 1,
        };

        let runner = crate::utils::backup::volume::VolumeBackupRunner::new(
            &executor,
            &volume_backup,
            &destination,
        );

        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let object_key = format!(
            "{}/{}_{}.tar.gz",
            backup.prefix.trim_matches('/'),
            backup.name,
            timestamp
        );

        let cancel = tokio_util::sync::CancellationToken::new();
        runner
            .run(&object_key, &cancel)
            .await
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

        Ok(())
    }
}

fn normalize_shell_type(value: Option<&str>) -> sqlx::Result<String> {
    let value = value.unwrap_or("BASH").trim().to_ascii_uppercase();
    let v = ShellType::try_from(value.as_str())
        .map_err(|_| sqlx::Error::Protocol(format!("invalid shell type: {}", value)))?;
    match v {
        ShellType::Bash => Ok(value),
        ShellType::Sh => Ok(value),
    }
}

fn normalize_schedule_type(value: Option<&str>) -> sqlx::Result<String> {
    let value = value.unwrap_or("APPLICATION").trim().to_ascii_uppercase();
    let v = ScheduleType::try_from(value.as_str())
        .map_err(|_| sqlx::Error::Protocol(format!("invalid schedule type: {}", value)))?;
    match v {
        ScheduleType::Application => Ok(value),
        ScheduleType::Compose => Ok(value),
        ScheduleType::Server => Ok(value),
        ScheduleType::DokpanelServer => Ok(value),
    }
}

fn normalize_schedule_action(
    value: Option<&str>,
    schedule_type: &str,
    command: &str,
) -> sqlx::Result<String> {
    let value = value
        .map(|value| value.trim().to_ascii_uppercase())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| infer_schedule_action(schedule_type, command));
    let v = ScheduleAction::try_from(value.as_str())
        .map_err(|_| sqlx::Error::Protocol(format!("invalid schedule action: {}", value)))?;
    match v {
        ScheduleAction::Exec => Ok(value),
        ScheduleAction::Deploy => Ok(value),
        ScheduleAction::Redeploy => Ok(value),
        ScheduleAction::Rebuild => Ok(value),
        ScheduleAction::Reload => Ok(value),
        ScheduleAction::Start => Ok(value),
        ScheduleAction::Stop => Ok(value),
    }
}

fn infer_schedule_action(schedule_type: &str, command: &str) -> String {
    let command = command.trim().to_ascii_lowercase();
    let schedule = ScheduleType::try_from(schedule_type)
        .map_err(|_| sqlx::Error::Protocol(format!("invalid schedule type: {}", schedule_type)))
        .unwrap();
    let is_operation = match schedule {
        ScheduleType::Application => matches!(
            command.as_str(),
            "deploy" | "redeploy" | "rebuild" | "reload" | "start"
        ),
        ScheduleType::Compose => matches!(
            command.as_str(),
            "deploy" | "redeploy" | "reload" | "start" | "stop"
        ),
        _ => false,
    };
    if is_operation {
        command.to_ascii_uppercase()
    } else {
        "EXEC".into()
    }
}

fn normalize_enabled(value: i64) -> sqlx::Result<i64> {
    match value {
        0 | 1 => Ok(value),
        _ => Err(sqlx::Error::Protocol("enabled must be 0 or 1".into())),
    }
}

fn validate_target(
    schedule_type: &str,
    schedule_action: &str,
    application_id: Option<i64>,
    compose_id: Option<i64>,
    server_id: Option<i64>,
    service_name: Option<&str>,
) -> sqlx::Result<()> {
    let v = ScheduleType::try_from(schedule_type)
        .map_err(|_| sqlx::Error::Protocol(format!("invalid schedule type: {}", schedule_type)))
        .unwrap();
    match v {
        ScheduleType::Application if application_id.is_none() => Err(sqlx::Error::Protocol(
            "APPLICATION schedule requires application_id".into(),
        )),
        ScheduleType::Compose if compose_id.is_none() => Err(sqlx::Error::Protocol(
            "COMPOSE schedule requires compose_id".into(),
        )),
        ScheduleType::Compose
            if schedule_action == "EXEC" && service_name.is_none_or(|v| v.trim().is_empty()) =>
        {
            Err(sqlx::Error::Protocol(
                "COMPOSE EXEC schedule requires service_name".into(),
            ))
        }
        ScheduleType::Server if server_id.is_none() => Err(sqlx::Error::Protocol(
            "SERVER schedule requires server_id".into(),
        )),
        ScheduleType::Server if schedule_action != "EXEC" => Err(sqlx::Error::Protocol(
            "SERVER schedule only supports EXEC action".into(),
        )),
        ScheduleType::DokpanelServer if schedule_action != "EXEC" => Err(sqlx::Error::Protocol(
            "DOKPANEL-SERVER schedule only supports EXEC action".into(),
        )),
        _ => Ok(()),
    }
}

fn parse_application_operation(value: &str) -> sqlx::Result<ApplicationOperation> {
    let v = ScheduleAction::try_from(value.trim().to_ascii_uppercase().as_str())
        .map_err(|_| sqlx::Error::Protocol(format!("invalid schedule action: {}", value)))
        .unwrap();
    match v {
        ScheduleAction::Deploy => Ok(ApplicationOperation::Deploy),
        ScheduleAction::Redeploy => Ok(ApplicationOperation::Redeploy),
        ScheduleAction::Rebuild => Ok(ApplicationOperation::Rebuild),
        ScheduleAction::Reload => Ok(ApplicationOperation::Reload),
        ScheduleAction::Start => Ok(ApplicationOperation::Start),
        _ => Err(sqlx::Error::Protocol(
            "application schedule command must be deploy, redeploy, rebuild, reload or start"
                .into(),
        )),
    }
}

fn parse_compose_operation(value: &str) -> sqlx::Result<ComposeOperation> {
    let v = ScheduleAction::try_from(value.trim().to_ascii_uppercase().as_str())
        .map_err(|_| sqlx::Error::Protocol(format!("invalid schedule action: {}", value)))
        .unwrap();
    match v {
        ScheduleAction::Deploy => Ok(ComposeOperation::Deploy),
        ScheduleAction::Redeploy => Ok(ComposeOperation::Redeploy),
        ScheduleAction::Reload => Ok(ComposeOperation::Reload),
        ScheduleAction::Start => Ok(ComposeOperation::Start),
        ScheduleAction::Stop => Ok(ComposeOperation::Stop),
        _ => Err(sqlx::Error::Protocol(
            "compose schedule command must be deploy, redeploy, reload, start or stop".into(),
        )),
    }
}

fn schedule_command(schedule: &Schedule) -> sqlx::Result<&str> {
    let command = schedule
        .script
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(schedule.command.trim());
    if command.is_empty() {
        return Err(sqlx::Error::Protocol("schedule command is empty".into()));
    }
    Ok(command)
}

fn generate_schedule_app_name(name: &str) -> String {
    let slug = name
        .trim()
        .to_ascii_lowercase()
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    let slug = if slug.is_empty() {
        "schedule".into()
    } else {
        slug
    };
    format!("{slug}-{}", Uuid::new_v4().simple())
}

fn parse_database_type(value: &str) -> Option<BackupsDatabaseTypeEnum> {
    match value.to_ascii_uppercase().as_str() {
        "POSTGRES" | "POSTGRESQL" => Some(BackupsDatabaseTypeEnum::Postgres),
        "MYSQL" => Some(BackupsDatabaseTypeEnum::Mysql),
        "MARIADB" => Some(BackupsDatabaseTypeEnum::Mariadb),
        "MONGO" | "MONGODB" => Some(BackupsDatabaseTypeEnum::Mongo),
        "REDIS" => Some(BackupsDatabaseTypeEnum::Redis),
        "LIBSQL" => Some(BackupsDatabaseTypeEnum::Libsql),
        _ => None,
    }
}

fn parse_volume_service_type(value: &str) -> Option<VolumeBackupsServiceTypeEnum> {
    match value.to_ascii_uppercase().as_str() {
        "APPLICATION" => Some(VolumeBackupsServiceTypeEnum::Application),
        "COMPOSE" => Some(VolumeBackupsServiceTypeEnum::Compose),
        "POSTGRES" => Some(VolumeBackupsServiceTypeEnum::Postgres),
        "MYSQL" => Some(VolumeBackupsServiceTypeEnum::Mysql),
        "MARIADB" => Some(VolumeBackupsServiceTypeEnum::Mariadb),
        "MONGO" | "MONGODB" => Some(VolumeBackupsServiceTypeEnum::Mongo),
        "REDIS" => Some(VolumeBackupsServiceTypeEnum::Redis),
        "LIBSQL" => Some(VolumeBackupsServiceTypeEnum::Libsql),
        _ => None,
    }
}
