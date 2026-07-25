use std::sync::Arc;

use auto_di::singleton;
use uuid::Uuid;

use crate::{
    api::dto::remote_server::{CreateRemoteServerDto, PatchRemoteServerDto},
    db::models::servers::Server,
    repository::{ServerRepository, SshKeyRepository},
};

pub struct ServerService {
    repo_server: Arc<ServerRepository>,
    repo_ssh: Arc<SshKeyRepository>,
}

#[singleton]
impl ServerService {
    fn new(repo_server: Arc<ServerRepository>, repo_ssh: Arc<SshKeyRepository>) -> Self {
        Self {
            repo_server,
            repo_ssh,
        }
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
        self.repo_server.delete(id).await
    }
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
