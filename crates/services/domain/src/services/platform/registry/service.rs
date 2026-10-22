use crate::db::queries::FlashQueries;
use crate::flash_gen::models::Registries;
use crate::flash_gen::registries::{
    RegistriesCreateParams, RegistriesUpdateParams, RegistriesUsageReferencesParams,
};
use auto_di::singleton;
use serde::Deserialize;
use std::sync::Arc;

use super::types::{CreateRegistryDto, PatchRegistryDto};

pub struct RegistryService {
    queries: Arc<FlashQueries>,
}

#[derive(Deserialize)]
struct RepositoryCatalogResponse {
    #[serde(default)]
    repositories: Vec<String>,
}

#[derive(Deserialize)]
struct RepositoryTagsResponse {
    #[serde(default)]
    tags: Vec<String>,
}

/// Registry URLs reach `docker login` as CLI arguments, so they must be a
/// bare `hostname[:port]` (or empty for Docker Hub) — never a URL with a
/// scheme/path and never shell metacharacters. Mirrors 's
/// `registryUrl` refine rule.
fn validate_registry_url(value: &str) -> sqlx::Result<()> {
    let value = value.trim();
    if value.is_empty() {
        return Ok(());
    }
    let host = match value.split_once(':') {
        Some((host, port)) if !host.is_empty() && port.len() <= 5 && port.bytes().all(|b| b.is_ascii_digit()) => host,
        Some(_) => return Err(sqlx::Error::Protocol("registry URL must be a valid hostname or hostname:port (e.g. registry.example.com or localhost:5000)".into())),
        None => value,
    };
    let valid_host = (2..=255).contains(&host.len())
        && host
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-' | b'_'))
        && !host.starts_with('-')
        && !host.ends_with('-');
    if !valid_host {
        return Err(sqlx::Error::Protocol("registry URL must be a valid hostname or hostname:port (e.g. registry.example.com or localhost:5000)".into()));
    }
    Ok(())
}

/// Image prefixes become repository path segments in generated image names;
/// restrict to lowercase OCI path characters (optionally with a port-suffixed
/// host, e.g. `registry.local:5000/team`).
fn validate_image_prefix(value: &str) -> sqlx::Result<()> {
    let value = value.trim();
    if value.is_empty() {
        return Ok(());
    }
    let ok = value.len() <= 200
        && value.bytes().all(|b| {
            b.is_ascii_lowercase()
                || b.is_ascii_digit()
                || matches!(b, b'/' | b'.' | b'-' | b'_' | b':' | b'~')
        })
        && !value.starts_with('/')
        && !value.ends_with('/');
    if !ok {
        return Err(sqlx::Error::Protocol(
            "image prefix must be a lowercase repository path (a-z, 0-9, / . - _ : ~), e.g. myorg"
                .into(),
        ));
    }
    Ok(())
}

#[singleton]
impl RegistryService {
    fn new(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }

    pub async fn get_by_id(&self, id: i64, organization_id: i64) -> sqlx::Result<Registries> {
        let registry = self.queries.registries_get_by_id(id).await?;
        if registry.organization_id != organization_id {
            return Err(sqlx::Error::RowNotFound);
        }
        Ok(registry)
    }

    pub async fn list(&self, organization_id: i64) -> sqlx::Result<Vec<Registries>> {
        self.queries
            .registries_list_by_organization(organization_id)
            .await
    }

    pub async fn create(
        &self,
        input: CreateRegistryDto,
        organization_id: i64,
    ) -> sqlx::Result<Registries> {
        validate_registry_url(&input.registry_url)?;
        if let Some(prefix) = &input.image_prefix {
            validate_image_prefix(prefix)?;
        }
        let now = chrono::Utc::now().timestamp();
        let new_id = self
            .queries
            .registries_create(&RegistriesCreateParams {
                registry_name: input.registry_name,
                image_prefix: input.image_prefix.unwrap_or_default(),
                username: input.username,
                password: input.password,
                registry_url: input.registry_url,
                registry_type: input.registry_type,
                organization_id,
                created_at: now,
                updated_at: now,
            })
            .await?;
        self.get_by_id(new_id, organization_id).await
    }

    pub async fn patch(
        &self,
        id: i64,
        input: PatchRegistryDto,
        organization_id: i64,
    ) -> sqlx::Result<Registries> {
        if let Some(v) = &input.registry_url {
            validate_registry_url(v)?;
        }
        if let Some(Some(v)) = &input.image_prefix {
            validate_image_prefix(v)?;
        }
        let mut current = self.get_by_id(id, organization_id).await?;
        let now = chrono::Utc::now().timestamp();

        if let Some(v) = input.registry_name {
            current.registry_name = v;
        }
        if let Some(v) = input.image_prefix {
            current.image_prefix = v;
        }
        if let Some(v) = input.username {
            current.username = v;
        }
        if let Some(v) = input.password {
            current.password = v;
        }
        if let Some(v) = input.registry_url {
            current.registry_url = v;
        }
        if let Some(v) = input.registry_type {
            current.registry_type = v;
        }
        current.updated_at = now;

        self.update_row(id, &current).await?;
        self.get_by_id(id, organization_id).await
    }

    pub async fn delete(&self, id: i64, organization_id: i64) -> sqlx::Result<()> {
        self.get_by_id(id, organization_id).await?;
        let usage = self
            .queries
            .registries_usage_references(&RegistriesUsageReferencesParams {
                registry_id: id,
                rollback_registry_id: id,
                build_registry_id: id,
            })
            .await?;
        if !usage.is_empty() {
            return Err(sqlx::Error::Protocol(format!(
                "registry is used by {} application configuration(s)",
                usage.len()
            )));
        }
        self.queries.registries_delete(id, organization_id).await
    }

    /// Anonymous reachability check: `GET /v2/` on the registry API.
    /// 2xx/4xx-with-`Docker-Distribution-Api-Version` proves a registry is
    /// there; a 401 means one exists but demands credentials.
    async fn ping_registry(&self, registry_url: &str) -> Result<(), String> {
        let host = registry_url.trim();
        let host = host
            .trim_start_matches("https://")
            .trim_start_matches("http://")
            .trim_end_matches('/');
        let host = if host.is_empty() || host.contains("docker.io") {
            "registry-1.docker.io"
        } else {
            host
        };
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .map_err(|e| e.to_string())?;
        // Local registries commonly speak plain HTTP; try https then http.
        let mut last_error = String::new();
        for scheme in ["https", "http"] {
            match client.get(format!("{scheme}://{host}/v2/")).send().await {
                Ok(response) => {
                    let status = response.status();
                    let is_registry = response
                        .headers()
                        .contains_key("docker-distribution-api-version");
                    if status.is_success() {
                        return Ok(());
                    }
                    if u16::from(status) == 401 || is_registry {
                        return Err(format!(
                            "registry '{host}' is reachable but requires credentials"
                        ));
                    }
                    last_error = format!("registry '{host}' responded {status}");
                }
                Err(error) => last_error = error.to_string(),
            }
        }
        Err(format!("registry '{host}' unreachable: {last_error}"))
    }
    pub async fn test_connection_raw(
        &self,
        registry_url: &str,
        username: &str,
        password: &str,
    ) -> Result<(), String> {
        // No credentials stored: verify reachability with the registry's
        // anonymous `GET /v2/` ping instead of `docker login` (which errors
        // with "Must provide --username with --password-stdin").
        if username.trim().is_empty() || password.trim().is_empty() {
            return self.ping_registry(registry_url).await;
        }
        let executor = crate::services::compute::compose::remote::agent_executor_for(None).await?;
        let docker = crate::utils::docker::DockerCli::from_executor(executor);
        let registry = if registry_url.trim().is_empty() || registry_url.contains("docker.io") {
            None
        } else {
            Some(registry_url)
        };

        let mut login_builder = docker
            .system()
            .login()
            .username(username)
            .password(password);
        if let Some(r) = registry {
            login_builder = login_builder.registry(r);
        }
        let res = login_builder.run().await;
        match res {
            Ok(output) => {
                if output.success() {
                    let mut logout_builder = docker.system().logout();
                    if let Some(r) = registry {
                        logout_builder = logout_builder.registry(r);
                    }
                    let _ = logout_builder.run().await;
                    Ok(())
                } else {
                    Err(format!("Login failed: {}", output.stderr))
                }
            }
            Err(e) => Err(format!("Docker execution error: {}", e)),
        }
    }

    pub async fn test_connection(&self, id: i64, organization_id: i64) -> Result<(), String> {
        let reg = self
            .get_by_id(id, organization_id)
            .await
            .map_err(|e| e.to_string())?;
        self.test_connection_raw(&reg.registry_url, &reg.username, &reg.password)
            .await
    }

    pub async fn repositories(&self, id: i64, organization_id: i64) -> Result<Vec<String>, String> {
        let registry = self
            .get_by_id(id, organization_id)
            .await
            .map_err(|error| error.to_string())?;
        let url = api_url(&registry.registry_url, "/v2/_catalog?n=1000");
        let value: RepositoryCatalogResponse = reqwest::Client::new()
            .get(url)
            .basic_auth(&registry.username, Some(&registry.password))
            .send()
            .await
            .map_err(|error| error.to_string())?
            .error_for_status()
            .map_err(|error| error.to_string())?
            .json()
            .await
            .map_err(|error| error.to_string())?;
        Ok(value.repositories)
    }

    pub async fn tags(
        &self,
        id: i64,
        repository: &str,
        organization_id: i64,
    ) -> Result<Vec<String>, String> {
        let registry = self
            .get_by_id(id, organization_id)
            .await
            .map_err(|error| error.to_string())?;
        if repository.is_empty() || repository.contains("..") {
            return Err("invalid repository name".into());
        }
        let url = api_url(
            &registry.registry_url,
            &format!("/v2/{repository}/tags/list"),
        );
        let value: RepositoryTagsResponse = reqwest::Client::new()
            .get(url)
            .basic_auth(&registry.username, Some(&registry.password))
            .send()
            .await
            .map_err(|error| error.to_string())?
            .error_for_status()
            .map_err(|error| error.to_string())?
            .json()
            .await
            .map_err(|error| error.to_string())?;
        Ok(value.tags)
    }

    pub async fn rotate_credentials(
        &self,
        id: i64,
        username: String,
        password: String,
        organization_id: i64,
    ) -> Result<Registries, String> {
        let current = self
            .get_by_id(id, organization_id)
            .await
            .map_err(|error| error.to_string())?;
        self.test_connection_raw(&current.registry_url, &username, &password)
            .await?;
        self.patch(
            id,
            PatchRegistryDto {
                registry_name: None,
                image_prefix: None,
                username: Some(username),
                password: Some(password),
                registry_url: None,
                registry_type: None,
            },
            organization_id,
        )
        .await
        .map_err(|error| error.to_string())
    }

    pub async fn usage(
        &self,
        id: i64,
        organization_id: i64,
    ) -> sqlx::Result<Vec<crate::flash_gen::registries::RegistriesUsageReferencesRow>> {
        self.get_by_id(id, organization_id).await?;
        self.queries
            .registries_usage_references(&RegistriesUsageReferencesParams {
                registry_id: id,
                rollback_registry_id: id,
                build_registry_id: id,
            })
            .await
    }
}

impl RegistryService {
    async fn update_row(&self, id: i64, r: &Registries) -> sqlx::Result<()> {
        self.queries
            .registries_update(&RegistriesUpdateParams {
                registry_name: r.registry_name.clone(),
                image_prefix: r.image_prefix.clone().unwrap_or_default(),
                username: r.username.clone(),
                password: r.password.clone(),
                registry_url: r.registry_url.clone(),
                registry_type: r.registry_type.clone(),
                created_at: r.created_at,
                updated_at: r.updated_at,
                id,
                organization_id: r.organization_id,
            })
            .await
    }
}

fn api_url(base: &str, path: &str) -> String {
    let base = base.trim().trim_end_matches('/');
    let base = if base.starts_with("http://") || base.starts_with("https://") {
        base.to_owned()
    } else {
        format!("https://{base}")
    };
    format!("{base}{path}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::fixture;

    async fn service(tag: &str) -> (RegistryService, sqlx::SqlitePool) {
        let (queries, pool) = fixture(tag).await;
        (RegistryService { queries }, pool)
    }

    fn registry(name: &str, url: &str) -> CreateRegistryDto {
        CreateRegistryDto {
            registry_name: name.into(),
            image_prefix: Some("team".into()),
            username: "deployer".into(),
            password: "secret".into(),
            registry_url: url.into(),
            registry_type: "GENERIC".into(),
        }
    }

    #[test]
    fn registry_url_accepts_bare_hosts_only() {
        for good in ["", "ghcr.io", "localhost:5000", "registry.example.com:443"] {
            assert!(validate_registry_url(good).is_ok(), "{good:?} should pass");
        }
        for bad in [
            "https://ghcr.io",
            "ghcr.io/v2",
            "ghcr.io:notaport",
            ":5000",
            "ghcr io",
            "ghcr.io;rm -rf /",
        ] {
            assert!(validate_registry_url(bad).is_err(), "{bad:?} should fail");
        }
    }

    #[test]
    fn image_prefix_must_be_a_lowercase_repository_path() {
        for good in ["", "team", "registry.local:5000/team", "a-b_c.d~e"] {
            assert!(validate_image_prefix(good).is_ok(), "{good:?} should pass");
        }
        for bad in ["Team", "/team", "team/", "team space", "team?x"] {
            assert!(validate_image_prefix(bad).is_err(), "{bad:?} should fail");
        }
    }

    #[tokio::test]
    async fn create_round_trips_and_scopes_to_the_organization() {
        let (service, _pool) = service("registry-create").await;

        let created = service
            .create(registry("ghcr", "ghcr.io"), 7)
            .await
            .unwrap();

        assert_eq!(created.registry_name, "ghcr");
        assert_eq!(created.registry_url, "ghcr.io");
        assert_eq!(created.image_prefix.as_deref(), Some("team"));
        assert_eq!(created.organization_id, 7);
        assert!(created.created_at > 0, "created_at should be a timestamp");
        assert!(created.updated_at > 0, "updated_at should be a timestamp");

        let read = service.get_by_id(created.id, 7).await.unwrap();
        assert_eq!(
            read.password, "secret",
            "the stored password must round-trip"
        );

        assert!(
            matches!(
                service.get_by_id(created.id, 8).await,
                Err(sqlx::Error::RowNotFound)
            ),
            "another organization must not read this registry"
        );
        assert!(service.list(8).await.unwrap().is_empty());
        assert_eq!(service.list(7).await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn create_rejects_invalid_urls_without_persisting() {
        let (service, _pool) = service("registry-reject").await;

        for (name, url) in [
            ("scheme", "https://ghcr.io"),
            ("path", "ghcr.io/v2"),
            ("space", "ghcr io"),
            ("shell", "ghcr.io;id"),
        ] {
            let error = service
                .create(registry(name, url), 1)
                .await
                .expect_err(name);
            assert!(matches!(error, sqlx::Error::Protocol(_)), "{name}: {error}");
        }

        assert!(
            service.list(1).await.unwrap().is_empty(),
            "a rejected create must not leave a row behind"
        );
    }

    #[tokio::test]
    async fn patch_updates_only_the_fields_that_were_sent() {
        let (service, _pool) = service("registry-patch").await;
        let created = service
            .create(registry("ghcr", "ghcr.io"), 1)
            .await
            .unwrap();

        let patched = service
            .patch(
                created.id,
                PatchRegistryDto {
                    registry_name: Some("dockerhub".into()),
                    image_prefix: None,
                    username: None,
                    password: Some("rotated".into()),
                    registry_url: None,
                    registry_type: None,
                },
                1,
            )
            .await
            .unwrap();

        assert_eq!(patched.registry_name, "dockerhub");
        assert_eq!(patched.password, "rotated");
        assert_eq!(
            patched.image_prefix.as_deref(),
            Some("team"),
            "an omitted image_prefix must keep its value"
        );
        assert_eq!(patched.registry_url, "ghcr.io");
        assert!(patched.updated_at >= created.updated_at);
    }

    #[tokio::test]
    async fn delete_removes_an_unused_registry() {
        let (service, _pool) = service("registry-delete").await;
        let created = service
            .create(registry("ghcr", "ghcr.io"), 1)
            .await
            .unwrap();

        service.delete(created.id, 1).await.unwrap();

        assert!(
            matches!(
                service.get_by_id(created.id, 1).await,
                Err(sqlx::Error::RowNotFound)
            ),
            "the row must be gone"
        );
    }

    #[tokio::test]
    async fn delete_refuses_a_registry_still_used_by_an_application() {
        let (service, pool) = service("registry-used").await;
        let created = service
            .create(registry("ghcr", "ghcr.io"), 1)
            .await
            .unwrap();

        sqlx::raw_sql(
            "INSERT INTO organization (name, slug, owner_id, created_at, updated_at)
                 VALUES ('org', 'org', 1, 0, 0);
             INSERT INTO projects (name, env_var, organization_id, created_at, updated_at)
                 VALUES ('site', '', 1, 0, 0);
             INSERT INTO environments (name, env_var, is_default, project_id, created_at, updated_at)
                 VALUES ('production', '', 1, 1, 0, 0);
             INSERT INTO services
                 (id, service_type, name, app_name, app_status, replicas, network_ids,
                  detach_rustploy_network, environment_id, created_at, updated_at)
                 VALUES (1, 'APPLICATION', 'app', 'app', 'IDLE', 1, '[]', 0, 1, 0, 0);",
        )
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(
            "INSERT INTO service_application
                 (service_id, build_type, create_env_file, clean_cache, rollback_active,
                  preview_https, preview_certificate_type,
                  is_preview_deployments_active, preview_require_collaborator_permissions,
                  registry_id)
             VALUES (1, 'NIXPACKS', 0, 0, 0, 0, 'NONE', 0, 0, ?)",
        )
        .bind(created.id)
        .execute(&pool)
        .await
        .unwrap();

        let error = service.delete(created.id, 1).await.unwrap_err();
        assert!(
            error.to_string().contains("used by 1 application"),
            "{error}"
        );
        assert!(
            service.get_by_id(created.id, 1).await.is_ok(),
            "a refused delete must keep the registry"
        );
        assert_eq!(service.usage(created.id, 1).await.unwrap().len(), 1);
    }
}
