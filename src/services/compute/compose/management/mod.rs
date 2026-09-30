mod mount;
mod patch;
mod template_vars;
mod transfer;

pub use mount::ComposeMountService;
pub use patch::{ComposePatchService, UpsertComposePatchDto};
pub use transfer::{ComposeTransferService, ImportComposeDto};

use std::sync::Arc;

use auto_di::{resolve, singleton};
use serde::{Deserialize, Serialize};

use crate::{
    core::cache::{AppStateCache, CacheKey},
    db::queries::FlashQueries,
    flash_gen::compose_extra::ComposeMoveToEnvironmentParams,
    services::compute::engine::DeploymentService,
    utils::builder::{compose::transform, queue::BuilderQueue},
};

use super::{
    crud::{CreateComposeDto, PatchComposeDto}, ComposeRecord, ComposeService, ComposeType,
};

pub struct ComposeManagementService {
    compose: Arc<ComposeService>,
    queries: Arc<FlashQueries>,
    deployment_service: Arc<DeploymentService>,
    cache: Arc<AppStateCache>,
}

#[singleton]
impl ComposeManagementService {
    fn new(
        compose: Arc<ComposeService>,
        queries: Arc<FlashQueries>,
        deployment_service: Arc<DeploymentService>,
        cache: Arc<AppStateCache>,
    ) -> Self {
        Self {
            compose,
            queries,
            deployment_service,
            cache,
        }
    }

    pub fn preview(&self, input: ComposePreviewDto) -> sqlx::Result<ComposePreviewResponseDto> {
        let suffix = input.randomize.then(|| {
            input
                .suffix
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(generate_suffix)
        });
        let app_name = input.isolated_deployment.then(|| {
            input
                .app_name
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| "compose-preview".into())
        });
        let compose_file = transform::transform_compose(
            &input.compose_file,
            suffix.as_deref(),
            app_name.as_deref(),
            input.isolated_deployments_volume,
        )
        .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        let services = transform::list_services(&compose_file)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        Ok(ComposePreviewResponseDto {
            compose_file,
            services,
        })
    }

    pub async fn install_template(
        &self,
        input: InstallComposeTemplateDto,
    ) -> sqlx::Result<ComposeRecord> {
        let created = self
            .compose
            .create(CreateComposeDto {
                name: input.name,
                description: input.description,
                environment_id: input.environment_id,
                server_id: input.server_id,
                source_type: "RAW".into(),
                compose_type: ComposeType::DockerCompose,
                compose_file: input.compose_file,
            })
            .await?;

        let Some(template_toml) = input.env_var else {
            return Ok(created);
        };

 // Resolve a full template.toml (variables + env + domains).
        // Fall back to plain generator expansion for a bare env block.
        let server_ip = self.server_ip(input.server_id).await;
        match template_vars::resolve_template_toml(&template_toml, server_ip.as_deref()) {
            Some(resolved) => {
                if !resolved.env.is_empty() {
                    self.patch_env(created.id, resolved.env).await?;
                }
                self.create_template_domains(created.id, &resolved.domains).await;
                self.create_template_mounts(created.id, &resolved.mounts).await;
            }
            None => {
                let env = template_vars::expand_env_template(&template_toml);
                self.patch_env(created.id, env).await?;
            }
        }
        self.compose.get_by_id(created.id).await
    }

    async fn patch_env(&self, id: i64, env_var: String) -> sqlx::Result<ComposeRecord> {
        self.compose
            .patch(
                id,
                PatchComposeDto {
                    name: None,
                    description: None,
                    env_var: Some(env_var),
                    compose_file: None,
                    compose_type: None,
                    trigger_type: None,
                    command: None,
                    enable_submodules: None,
                    compose_path: None,
                    suffix: None,
                    randomize: None,
                    isolated_deployment: None,
                    isolated_deployments_volume: None,
                    watch_paths: None,
                    service_networks: None,
                    server_id: None,
                },
            )
            .await
    }

    /// A reachable IP for sslip.io domain generation. Remote servers use their
    /// stored IP; local deploys fall back to OPENOXIDE_SERVER_URL's host (if an
    /// IP) or the panel's routable LAN IPv4 — never a bare random host.
    async fn server_ip(&self, server_id: Option<i64>) -> Option<String> {
        if let Some(server_id) = server_id {
            if let Ok(server) = self.queries.get_server_by_i_d(server_id).await {
                let ip = server.ip_address;
                if !matches!(ip.as_str(), "" | "0.0.0.0" | "localhost" | "127.0.0.1") {
                    return Some(ip);
                }
            }
        }
        // Local deploy: prefer an explicitly configured public host, then a
        // public URL host, then a probed routable IP (internal ranges skipped).
        env_ip("OPENOXIDE_PUBLIC_IP")
            .or_else(|| server_url_ip("OPENOXIDE_PUBLIC_URL"))
            .or_else(|| server_url_ip("OPENOXIDE_SERVER_URL"))
            .or_else(routable_ipv4)
    }

    async fn create_template_domains(
        &self,
        compose_id: i64,
        domains: &[template_vars::ResolvedDomain],
    ) {
        if domains.is_empty() {
            return;
        }
        let organization_id = match self
            .queries
            .deployments_compose_organization_id(compose_id)
            .await
        {
            Ok(org) => org,
            Err(error) => {
                tracing::warn!(compose_id, %error, "template domains: could not resolve organization");
                return;
            }
        };
        let mut seen: std::collections::HashSet<(String, String)> =
            std::collections::HashSet::new();
        for domain in domains {
            let path = domain.path.clone().unwrap_or_else(|| "/".into());
            // A route is unique by host+path; skip duplicates so the create
            // never hits the "already exists" guard.
            if !seen.insert((domain.host.clone(), path.clone())) {
                continue;
            }
            let params = crate::flash_gen::domains::DomainsCreateAndReturnParams {
                host: domain.host.clone(),
                https: 0,
                port: domain.port,
                path,
                internal_path: "/".into(),
                custom_entrypoint: String::new(),
                service_name: domain.service_name.clone(),
                custom_cert_resolver: String::new(),
                strip_path: 0,
                middlewares: "[]".into(),
                domain_type: "compose".into(),
                certificate_type: "none".into(),
                service_id: compose_id,
                organization_id,
            };
            if let Err(error) = self.queries.domains_create_and_return(&params).await {
                tracing::warn!(compose_id, host = %domain.host, %error, "template domain creation failed");
            }
        }
    }

    async fn create_template_mounts(
        &self,
        compose_id: i64,
        mounts: &[template_vars::ResolvedMount],
    ) {
        for mount in mounts {
            let params = crate::flash_gen::mount::MountCreateComposeFileParams {
                file_path: mount.file_path.clone(),
                content: mount.content.clone(),
                service_id: compose_id,
            };
            if let Err(error) = self.queries.mount_create_compose_file(&params).await {
                tracing::warn!(compose_id, path = %mount.file_path, %error, "template mount creation failed");
            }
        }
    }

    pub async fn remove_service(&self, id: i64, service_name: &str) -> sqlx::Result<ComposeRecord> {
        let current = self.compose.get_by_id(id).await?;
        if crate::utils::builder::spec::SourceType::from_str(current.source_type.as_str())
            .ok_or_else(|| sqlx::Error::Protocol("invalid compose source type".into()))?
            != crate::utils::builder::spec::SourceType::Raw
        {
            return Err(sqlx::Error::Protocol(
                "service removal requires a RAW compose source".into(),
            ));
        }
        let compose_file = transform::remove_service(&current.compose_file, service_name)
            .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        self.queries.update_compose_file(&compose_file, id).await?;
        self.cache.invalidate(&CacheKey::Compose(id)).await;
        self.compose.get_by_id(id).await
    }

    pub async fn upsert_resource(
        &self,
        id: i64,
        input: UpsertComposeResourceDto,
    ) -> sqlx::Result<ComposeRecord> {
        let current = self.compose.get_by_id(id).await?;
        if crate::utils::builder::spec::SourceType::from_str(current.source_type.as_str())
            .ok_or_else(|| sqlx::Error::Protocol("invalid compose source type".into()))?
            != crate::utils::builder::spec::SourceType::Raw
        {
            return Err(sqlx::Error::Protocol(
                "config and secret editing requires a RAW compose source".into(),
            ));
        }
        let compose_file = transform::upsert_resource(
            &current.compose_file,
            input.kind.as_str(),
            &input.name,
            input.file.as_deref(),
            input.external,
            &input.services,
        )
        .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        self.queries.update_compose_file(&compose_file, id).await?;
        self.cache.invalidate(&CacheKey::Compose(id)).await;
        self.compose.get_by_id(id).await
    }

    pub async fn remove_resource(
        &self,
        id: i64,
        input: DeleteComposeResourceDto,
    ) -> sqlx::Result<ComposeRecord> {
        let current = self.compose.get_by_id(id).await?;
        if crate::utils::builder::spec::SourceType::from_str(current.source_type.as_str())
            .ok_or_else(|| sqlx::Error::Protocol("invalid compose source type".into()))?
            != crate::utils::builder::spec::SourceType::Raw
        {
            return Err(sqlx::Error::Protocol(
                "config and secret editing requires a RAW compose source".into(),
            ));
        }
        let compose_file =
            transform::remove_resource(&current.compose_file, input.kind.as_str(), &input.name)
                .map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
        self.queries.update_compose_file(&compose_file, id).await?;
        self.cache.invalidate(&CacheKey::Compose(id)).await;
        self.compose.get_by_id(id).await
    }

    pub async fn move_to_environment(
        &self,
        id: i64,
        target_environment_id: i64,
    ) -> sqlx::Result<ComposeRecord> {
        self.compose.get_by_id(id).await?;
        if self
            .queries
            .deployments_has_running_compose_deployment(&id.to_string())
            .await?
        {
            return Err(sqlx::Error::Protocol(
                "compose deployment is already queued or running".into(),
            ));
        }
        // Old move_to_environment_in_same_organization returned None when no
        // row matched; :one RETURNING id surfaces that as RowNotFound.
        self.queries
            .compose_move_to_environment(&ComposeMoveToEnvironmentParams {
                environment_id: target_environment_id,
                id,
                id2: target_environment_id,
            })
            .await
            .map_err(|error| match error {
                sqlx::Error::RowNotFound => sqlx::Error::Protocol(
                    "target environment not found or belongs to another organization".into(),
                ),
                other => other,
            })?;
        self.cache.invalidate(&CacheKey::Compose(id)).await;
        self.compose.get_by_id(id).await
    }

    pub async fn rotate_webhook_token(&self, id: i64) -> sqlx::Result<String> {
        self.compose.get_by_id(id).await?;
        let token = generate_token()?;
        // Old rotate_refresh_token returned rows_affected == 1; a miss
        // surfaces here as RowNotFound.
        self.queries
            .compose_rotate_webhook_token(&token, id)
            .await?;
        self.cache.invalidate(&CacheKey::Compose(id)).await;
        Ok(token)
    }

    pub async fn cleanup_queue(&self, id: i64) -> sqlx::Result<u64> {
        self.compose.get_by_id(id).await?;
        if let Ok(queue) = resolve::<BuilderQueue>().await {
            let _ = queue.cancel_queued_compose(id).await;
        }
        // Old cancel_queued_for_compose_count returned the UPDATE's
        // rows_affected; the queued ids are collected first so the count
        // matches the rows the follow-up statement cancels.
        let queued = self
            .queries
            .deployments_get_queued_ids_for_compose(id)
            .await?;
        self.queries
            .deployments_cancel_queued_for_compose(id)
            .await?;
        Ok(queued.len() as u64)
    }

    pub async fn clear_history(&self, id: i64) -> sqlx::Result<u64> {
        self.compose.get_by_id(id).await?;
        let deployment_ids = self
            .queries
            .deployments_list_finished_ids_for_compose(id)
            .await?;
        // Old delete_finished_for_compose returned rows_affected, which the
        // id list above mirrors for the follow-up DELETE.
        let affected = deployment_ids.len() as u64;
        self.queries
            .deployments_delete_finished_for_compose(id)
            .await?;
        for deployment_id in deployment_ids {
            let path = crate::utils::paths::openoxide_paths().deployment_log_file(deployment_id);
            if let Err(error) = tokio::fs::remove_file(path).await
                && error.kind() != std::io::ErrorKind::NotFound
            {
                tracing::warn!(deployment_id, %error, "could not delete compose deployment log");
            }
        }
        Ok(affected)
    }

    pub async fn force_kill(
        &self,
        id: i64,
    ) -> sqlx::Result<crate::services::compute::engine::CancelDeploymentResult> {
        self.compose.get_by_id(id).await?;
        // Old running_for_compose returned Option; :one surfaces a miss as
        // RowNotFound, mapped to the original Protocol error.
        let deployment = self
            .queries
            .deployments_running_for_compose(id)
            .await
            .map_err(|error| match error {
                sqlx::Error::RowNotFound => {
                    sqlx::Error::Protocol("compose has no running deployment".into())
                }
                other => other,
            })?;
        self.deployment_service.cancel(deployment.id).await
    }
}

/// A bare IP address held directly in the named env var.
fn env_ip(var: &str) -> Option<String> {
    let value = std::env::var(var).ok()?;
    let value = value.trim();
    value.parse::<std::net::IpAddr>().ok().map(|_| value.to_owned())
}

/// The host of a `scheme://host:port` URL env var when it is a bare IP.
fn server_url_ip(var: &str) -> Option<String> {
    let url = std::env::var(var).ok()?;
    let host = url
        .split("://")
        .nth(1)
        .unwrap_or(&url)
        .split('/')
        .next()?
        .split(':')
        .next()?;
    host.parse::<std::net::IpAddr>().ok().map(|_| host.to_owned())
}

/// The panel's routable IPv4 (UDP-connect probe, no packets sent). Skips
/// container-internal ranges (Docker bridge 172.16-31.x, 10.x, loopback,
/// link-local) since those are never reachable from a browser.
fn routable_ipv4() -> Option<String> {
    let socket = std::net::UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    let std::net::IpAddr::V4(ip) = socket.local_addr().ok()?.ip() else {
        return None;
    };
    let [a, b, ..] = ip.octets();
    let internal = ip.is_loopback()
        || ip.is_link_local()
        || a == 10
        || (a == 172 && (16..=31).contains(&b))
        || (a == 192 && b == 168);
    (!internal).then(|| ip.to_string())
}

fn generate_suffix() -> String {
    let mut bytes = [0_u8; 4];
    let _ = getrandom::fill(&mut bytes);
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn generate_token() -> sqlx::Result<String> {
    let mut bytes = [0_u8; 32];
    getrandom::fill(&mut bytes).map_err(|error| sqlx::Error::Protocol(error.to_string()))?;
    Ok(bytes.iter().map(|byte| format!("{byte:02x}")).collect())
}

// ── JSON payload contract (moved from the retired api::dto layer) ──────

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ComposeResourceKind {
    Config,
    Secret,
}
impl ComposeResourceKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Config => "CONFIG",
            Self::Secret => "SECRET",
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct InstallComposeTemplateDto {
    pub name: String,
    pub description: Option<String>,
    pub environment_id: i64,
    pub server_id: Option<i64>,
    pub compose_file: String,
    pub env_var: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ComposePreviewDto {
    pub compose_file: String,
    pub randomize: bool,
    pub suffix: Option<String>,
    pub isolated_deployment: bool,
    pub isolated_deployments_volume: bool,
    pub app_name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ComposePreviewResponseDto {
    pub compose_file: String,
    pub services: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpsertComposeResourceDto {
    pub kind: ComposeResourceKind,
    pub name: String,
    pub file: Option<String>,
    pub external: bool,
    pub services: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteComposeResourceDto {
    pub kind: ComposeResourceKind,
    pub name: String,
}
