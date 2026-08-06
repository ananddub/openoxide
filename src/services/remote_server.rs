use std::sync::Arc;

use auto_di::singleton;
use uuid::Uuid;

use crate::{
    api::dto::remote_server::{CreateRemoteServerDto, PatchRemoteServerDto},
    db::models::server_private_networks::{PrivateNetworkStatus, ServerPrivateNetwork},
    db::models::servers::Server,
    repository::{
        ServerMigrationRepository, ServerPrivateNetworkRepository, ServerRepository,
        SshKeyRepository,
    },
    services::{
        application::{ApplicationOperation, ApplicationService},
        compose::{ComposeOperation, ComposeService},
    },
};

pub struct ServerService {
    repo_server: Arc<ServerRepository>,
    repo_ssh: Arc<SshKeyRepository>,
    private_networks: Arc<ServerPrivateNetworkRepository>,
    migrations: Arc<ServerMigrationRepository>,
    applications: Arc<ApplicationService>,
    compose: Arc<ComposeService>,
}

#[singleton]
impl ServerService {
    fn new(
        repo_server: Arc<ServerRepository>,
        repo_ssh: Arc<SshKeyRepository>,
        private_networks: Arc<ServerPrivateNetworkRepository>,
        migrations: Arc<ServerMigrationRepository>,
        applications: Arc<ApplicationService>,
        compose: Arc<ComposeService>,
    ) -> Self {
        Self {
            repo_server,
            repo_ssh,
            private_networks,
            migrations,
            applications,
            compose,
        }
    }

    pub async fn setup_advertise_addr(
        &self,
        server_id: i64,
        requested: Option<String>,
    ) -> sqlx::Result<Option<String>> {
        if let Some(address) = requested {
            return validate_advertise_addr(address).map(Some);
        }
        let Some(network) = self.private_networks.get(server_id).await? else {
            return Ok(None);
        };
        private_advertise_addr(&network)
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<Server> {
        self.repo_server
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn list(&self) -> sqlx::Result<Vec<Server>> {
        self.repo_server.list_ordered().await
    }

    pub async fn connection_details(
        &self,
        id: i64,
    ) -> sqlx::Result<(Server, crate::db::models::ssh_keys::SshKey)> {
        let server = self.get_by_id(id).await?;
        let key_id = server.ssh_key_id.ok_or(sqlx::Error::RowNotFound)?;
        let key = self
            .repo_ssh
            .get_by_id(key_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        Ok((server, key))
    }

    pub async fn create(&self, input: CreateRemoteServerDto) -> sqlx::Result<Server> {
        let app_name = generate_app_name(&input.name);
        let server_type = match input.server_type.to_uppercase().as_str() {
            "BUILD" => "BUILD".to_string(),
            _ => "DEPLOY".to_string(),
        };

        self.repo_server
            .create_and_return(
                input.name,
                input.description,
                input.ip_address,
                input.port,
                input.username,
                app_name,
                server_type,
                input.ssh_key_id,
                input.build_memory_limit,
                input.build_cpu_limit,
            )
            .await
    }

    pub async fn patch(&self, id: i64, input: PatchRemoteServerDto) -> sqlx::Result<Server> {
        let current = self.get_by_id(id).await?;
        let name = input.name.unwrap_or(current.name);
        let description = input.description.or(current.description);
        let ip_address = input.ip_address.unwrap_or(current.ip_address);
        let port = input.port.unwrap_or(current.port);
        let username = input.username.unwrap_or(current.username);
        let server_status = input.server_status.unwrap_or(current.server_status);
        let server_type = input.server_type.unwrap_or(current.server_type);
        let server_type = match server_type.to_uppercase().as_str() {
            "BUILD" => "BUILD".to_string(),
            _ => "DEPLOY".to_string(),
        };
        let enable_docker_cleanup = input
            .enable_docker_cleanup
            .unwrap_or(current.enable_docker_cleanup);
        let log_cleanup_cron = input.log_cleanup_cron.or(current.log_cleanup_cron);
        let command = input.command.unwrap_or(current.command);
        let metrics_config = input.metrics_config.unwrap_or(current.metrics_config);
        let ssh_key_id = input.ssh_key_id.or(current.ssh_key_id);
        let build_memory_limit = input.build_memory_limit.or(current.build_memory_limit);
        let build_cpu_limit = input.build_cpu_limit.or(current.build_cpu_limit);

        self.repo_server
            .update_and_return(
                id,
                name,
                description,
                ip_address,
                port,
                username,
                server_status,
                server_type,
                enable_docker_cleanup,
                log_cleanup_cron,
                command,
                metrics_config,
                ssh_key_id,
                build_memory_limit,
                build_cpu_limit,
            )
            .await
    }

    pub async fn set_status(&self, id: i64, status: &str) -> sqlx::Result<Server> {
        self.repo_server.set_status(id, status).await
    }

    pub async fn touch_test_connection(&self, id: i64) -> sqlx::Result<Server> {
        let server = self.get_by_id(id).await?;
        if let Some(ssh_key_id) = server.ssh_key_id {
            self.repo_ssh.touch_ssh_key(ssh_key_id).await?;
        }
        Ok(server)
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.get_by_id(id).await?;
        let dependencies = self.repo_server.dependency_counts(id).await?;
        if dependencies.total() > 0 {
            return Err(sqlx::Error::Protocol(format!(
                "server has active dependencies: applications={}, build_assignments={}, compose_projects={}, databases={}, certificates={}, schedules={}",
                dependencies.applications,
                dependencies.build_assignments,
                dependencies.compose_projects,
                dependencies.databases,
                dependencies.certificates,
                dependencies.schedules,
            )));
        }
        self.repo_server.delete(id).await
    }

    pub async fn migrate_dependencies(
        &self,
        source: i64,
        target: i64,
    ) -> sqlx::Result<crate::api::dto::remote_server::ServerDependencyMigrationDto> {
        if source == target {
            return Err(sqlx::Error::Protocol(
                "source and target server must differ".into(),
            ));
        }
        self.get_by_id(source).await?;
        self.get_by_id(target).await?;
        let dependencies = self.repo_server.dependency_counts(source).await?;
        if dependencies.databases > 0 {
            return Err(sqlx::Error::Protocol(format!(
                "stateful migration is required for {} databases; database metadata was not moved",
                dependencies.databases
            )));
        }
        let (application_ids, build_ids, compose_ids, certificate_ids, schedule_ids) =
            self.repo_server.migratable_resource_ids(source).await?;
        let migration_id = Uuid::new_v4().simple().to_string();
        let encode = |values: &[i64]| {
            serde_json::to_string(values).map_err(|error| sqlx::Error::Protocol(error.to_string()))
        };
        self.migrations
            .begin(
                &migration_id,
                source,
                target,
                &encode(&application_ids)?,
                &encode(&build_ids)?,
                &encode(&compose_ids)?,
                &encode(&certificate_ids)?,
                &encode(&schedule_ids)?,
            )
            .await?;
        let mut stop_failures = Vec::new();
        for id in &application_ids {
            if let Err(error) = self.applications.cancel_operation(*id).await {
                stop_failures.push(format!("application {id}: {error}"));
            }
        }
        for id in &compose_ids {
            if let Err(error) = self.compose.cancel_operation(*id).await {
                stop_failures.push(format!("compose {id}: {error}"));
            }
        }
        if !stop_failures.is_empty() {
            let error = format!(
                "source workloads could not be stopped: {}",
                stop_failures.join("; ")
            );
            self.migrations
                .finish(&migration_id, false, 0, 0, Some(&error))
                .await?;
            return self.migration_status(&migration_id).await;
        }
        let counts = self
            .repo_server
            .migrate_dependencies(source, target)
            .await?;
        let mut queued_applications = 0_i64;
        let mut queued_compose = 0_i64;
        let mut failures = Vec::new();
        for id in &application_ids {
            match self
                .applications
                .run_operation(*id, ApplicationOperation::Redeploy)
                .await
            {
                Ok(_) => queued_applications += 1,
                Err(error) => failures.push(format!("application {id}: {error}")),
            }
        }
        for id in &compose_ids {
            match self
                .compose
                .run_operation(*id, ComposeOperation::Redeploy)
                .await
            {
                Ok(_) => queued_compose += 1,
                Err(error) => failures.push(format!("compose {id}: {error}")),
            }
        }
        let error = (!failures.is_empty()).then(|| failures.join("; "));
        self.migrations
            .finish(
                &migration_id,
                error.is_none(),
                queued_applications,
                queued_compose,
                error.as_deref(),
            )
            .await?;
        let migration = self
            .migrations
            .get(&migration_id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        let mut dto =
            crate::api::dto::remote_server::ServerDependencyMigrationDto::try_from(migration)?;
        dto.databases = counts.databases;
        Ok(dto)
    }

    pub async fn migration_status(
        &self,
        id: &str,
    ) -> sqlx::Result<crate::api::dto::remote_server::ServerDependencyMigrationDto> {
        self.migrations
            .get(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?
            .try_into()
    }

    pub async fn rollback_migration(
        &self,
        id: &str,
    ) -> sqlx::Result<crate::api::dto::remote_server::ServerDependencyMigrationDto> {
        let migration = self
            .migrations
            .get(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        if migration.status == "ROLLED_BACK" {
            return migration.try_into();
        }
        let decode = |value: &str| {
            serde_json::from_str::<Vec<i64>>(value)
                .map_err(|error| sqlx::Error::Protocol(error.to_string()))
        };
        let application_ids = decode(&migration.application_ids)?;
        let compose_ids = decode(&migration.compose_ids)?;
        self.repo_server
            .rollback_migrated_resources(
                migration.source_server_id,
                migration.target_server_id,
                &application_ids,
                &decode(&migration.build_application_ids)?,
                &compose_ids,
                &decode(&migration.certificate_ids)?,
                &decode(&migration.schedule_ids)?,
            )
            .await?;
        self.migrations.mark_rolled_back(id).await?;
        let mut failures = Vec::new();
        for resource_id in application_ids {
            if let Err(error) = self
                .applications
                .run_operation(resource_id, ApplicationOperation::Redeploy)
                .await
            {
                failures.push(format!("application {resource_id}: {error}"));
            }
        }
        for resource_id in compose_ids {
            if let Err(error) = self
                .compose
                .run_operation(resource_id, ComposeOperation::Redeploy)
                .await
            {
                failures.push(format!("compose {resource_id}: {error}"));
            }
        }
        if !failures.is_empty() {
            return Err(sqlx::Error::Protocol(format!(
                "metadata rolled back but some source redeploys failed: {}",
                failures.join("; ")
            )));
        }
        self.migration_status(id).await
    }
}

fn private_advertise_addr(network: &ServerPrivateNetwork) -> sqlx::Result<Option<String>> {
    if PrivateNetworkStatus::try_from(network.status.as_str())? != PrivateNetworkStatus::Active {
        return Ok(None);
    }
    network
        .private_host
        .clone()
        .map(validate_advertise_addr)
        .transpose()
}

fn validate_advertise_addr(value: String) -> sqlx::Result<String> {
    let address = value.parse::<std::net::IpAddr>().map_err(|error| {
        sqlx::Error::Protocol(format!(
            "invalid Swarm advertise address {value:?}: {error}"
        ))
    })?;
    if address.is_unspecified() || address.is_loopback() || address.is_multicast() {
        return Err(sqlx::Error::Protocol(
            "Swarm advertise address must be a routable unicast IP".into(),
        ));
    }
    Ok(address.to_string())
}

fn generate_app_name(name: &str) -> String {
    let mut slug = String::new();
    let mut previous_dash = false;

    for ch in name.to_lowercase().chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch);
            previous_dash = false;
        } else if !previous_dash && !slug.is_empty() {
            slug.push('-');
            previous_dash = true;
        }
    }

    let slug = slug.trim_matches('-');
    let base = if slug.is_empty() { "server" } else { slug };
    let suffix = Uuid::new_v4().simple().to_string();
    format!("{}-{}", base, &suffix[..6])
}

#[cfg(test)]
mod tests {
    use super::validate_advertise_addr;

    #[test]
    fn swarm_advertise_address_accepts_private_provider_ips() {
        assert_eq!(
            validate_advertise_addr("100.64.10.2".into()).unwrap(),
            "100.64.10.2"
        );
        assert_eq!(
            validate_advertise_addr("10.77.8.2".into()).unwrap(),
            "10.77.8.2"
        );
    }

    #[test]
    fn swarm_advertise_address_rejects_unroutable_values() {
        assert!(validate_advertise_addr("127.0.0.1".into()).is_err());
        assert!(validate_advertise_addr("0.0.0.0".into()).is_err());
        assert!(validate_advertise_addr("tailscale0".into()).is_err());
    }
}
