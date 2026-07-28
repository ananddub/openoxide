use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;

use crate::api::dto::domain::{CreateDomainDto, PatchDomainDto};
use crate::repository::{ApplicationRepository, ComposeProjectRepository, DomainRepository};
use crate::services::compose::ComposeType;
use crate::utils::builder::compose::labels::build_compose_service_labels;
use crate::utils::builder::shared::traefik::build_traefik_labels;
use crate::utils::docker::DockerCli;
use crate::utils::exec::{CommandExecutor, LocalExecutor};

#[derive(Debug, Clone)]
pub struct DomainRecord {
    pub id: i64,
    pub host: String,
    pub https: i64,
    pub port: Option<i64>,
    pub path: Option<String>,
    pub internal_path: Option<String>,
    pub custom_entrypoint: Option<String>,
    pub service_name: Option<String>,
    pub custom_cert_resolver: Option<String>,
    pub strip_path: i64,
    pub middlewares: String,
    pub domain_type: String,
    pub certificate_type: String,
    pub application_id: Option<i64>,
    pub compose_id: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Clone)]
pub struct DomainService {
    db: Arc<SqlitePool>,
    repo_domain: Arc<DomainRepository>,
    repo_app: Arc<ApplicationRepository>,
    repo_compose: Arc<ComposeProjectRepository>,
}

#[singleton]
impl DomainService {
    fn new(
        db: Arc<SqlitePool>,
        repo_domain: Arc<DomainRepository>,
        repo_app: Arc<ApplicationRepository>,
        repo_compose: Arc<ComposeProjectRepository>,
    ) -> Self {
        Self {
            db,
            repo_domain,
            repo_app,
            repo_compose,
        }
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<DomainRecord> {
        self.repo_domain
            .get_record_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn list_by_application(
        &self,
        application_id: i64,
    ) -> sqlx::Result<Vec<DomainRecord>> {
        self.repo_domain.list_by_application(application_id).await
    }

    pub async fn list_by_compose(&self, compose_id: i64) -> sqlx::Result<Vec<DomainRecord>> {
        self.repo_domain.list_by_compose(compose_id).await
    }

    pub async fn create(&self, input: CreateDomainDto) -> sqlx::Result<DomainRecord> {
        let https = bool_to_i64(input.https);
        let strip_path = bool_to_i64(input.strip_path);
        let port = input.port.or(Some(3000));
        let certificate_type = input.certificate_type.to_uppercase();
        let domain_type = match (input.application_id, input.compose_id) {
            (Some(_), None) => "APPLICATION",
            (None, Some(_)) => "COMPOSE",
            _ => "APPLICATION",
        };

        let domain = self
            .repo_domain
            .create_and_return(
                input.host,
                https,
                port,
                Some(input.path),
                Some(input.internal_path),
                input.custom_entrypoint,
                input.service_name,
                input.custom_cert_resolver,
                strip_path,
                input.middlewares,
                domain_type.to_string(),
                certificate_type,
                input.application_id,
                input.compose_id,
            )
            .await?;

        let this = self.clone();
        let domain_clone = domain.clone();
        tokio::spawn(async move {
            if let Some(app_id) = domain_clone.application_id.filter(|&id| id > 0) {
                this.apply_application_traefik(app_id).await;
            }
            if let Some(compose_id) = domain_clone.compose_id.filter(|&id| id > 0) {
                this.apply_compose_traefik(compose_id).await;
            }
        });

        Ok(domain)
    }

    pub async fn patch(&self, id: i64, input: PatchDomainDto) -> sqlx::Result<DomainRecord> {
        let current = self.get_by_id(id).await?;
        let host = input.host.unwrap_or(current.host);
        let https = input.https.map(bool_to_i64).unwrap_or(current.https);
        let port = input.port.or(current.port);
        let path = input.path.or(current.path);
        let internal_path = input.internal_path.or(current.internal_path);
        let custom_entrypoint = input.custom_entrypoint.or(current.custom_entrypoint);
        let service_name = input.service_name.or(current.service_name);
        let custom_cert_resolver = input.custom_cert_resolver.or(current.custom_cert_resolver);
        let strip_path = input
            .strip_path
            .map(bool_to_i64)
            .unwrap_or(current.strip_path);
        let middlewares = input.middlewares.unwrap_or(current.middlewares);
        let certificate_type = input
            .certificate_type
            .map(|v| v.to_uppercase())
            .unwrap_or(current.certificate_type);

        let domain = self
            .repo_domain
            .update_and_return(
                id,
                host,
                https,
                port,
                path,
                internal_path,
                custom_entrypoint,
                service_name,
                custom_cert_resolver,
                strip_path,
                middlewares,
                certificate_type,
            )
            .await?;

        // Auto-apply traefik labels asynchronously after domain update.
        let this = self.clone();
        let domain_clone = domain.clone();
        tokio::spawn(async move {
            if let Some(app_id) = domain_clone.application_id.filter(|&id| id > 0) {
                this.apply_application_traefik(app_id).await;
            }
            if let Some(compose_id) = domain_clone.compose_id.filter(|&id| id > 0) {
                this.apply_compose_traefik(compose_id).await;
            }
        });

        Ok(domain)
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        let domain = self.get_by_id(id).await?;
        self.repo_domain.delete(id).await?;

        let this = self.clone();
        tokio::spawn(async move {
            if let Some(app_id) = domain.application_id.filter(|&id| id > 0) {
                this.apply_application_traefik(app_id).await;
            }
            if let Some(compose_id) = domain.compose_id.filter(|&id| id > 0) {
                this.apply_compose_traefik(compose_id).await;
            }
        });

        Ok(())
    }

    async fn apply_application_traefik(&self, application_id: i64) {
        if let Err(e) = self.try_apply_application_traefik(application_id).await {
            tracing::warn!(
                application_id,
                error = %e,
                "could not apply traefik labels after domain change"
            );
        }
    }

    async fn try_apply_application_traefik(&self, application_id: i64) -> Result<(), String> {
        let app = self
            .repo_app
            .get_by_id(application_id)
            .await
            .map_err(|e| format!("app check error: {e}"))?
            .ok_or_else(|| format!("app not found: {application_id}"))?;
        let app_name = app.app_name;
        let server_id = app.server_id;

        let service_name = format!("{app_name}_{app_name}");

        // Load all remaining domains for this application.
        let domains = self
            .list_by_application(application_id)
            .await
            .map_err(|e| format!("could not load domains: {e}"))?;

        let shared_domains: Vec<crate::utils::builder::shared::traefik::SharedDomain> = domains
            .iter()
            .map(|d| crate::utils::builder::shared::traefik::SharedDomain {
                key: d.id.to_string(),
                host: d.host.clone(),
                https: d.https != 0,
                port: d.port.unwrap_or(3000) as u16,
                service_name: d.service_name.clone(),
                path: d.path.clone().unwrap_or_else(|| "/".into()),
                internal_path: d.internal_path.clone().unwrap_or_else(|| "/".into()),
                strip_path: d.strip_path != 0,
                entrypoint: d.custom_entrypoint.clone(),
                certificate_type: d.certificate_type.clone(),
                custom_cert_resolver: d.custom_cert_resolver.clone(),
                middlewares: serde_json::from_str(&d.middlewares).unwrap_or_default(),
            })
            .collect();

        let traefik_map = build_traefik_labels(&app_name, &shared_domains);
        let new_labels: Vec<String> = traefik_map.into_values().flatten().collect();

        // Build docker executor (local or remote).
        let executor = match server_id {
            Some(sid) => {
                use crate::services::application::remote::remote_executor;
                let remote = remote_executor(self.db.as_ref(), sid)
                    .await
                    .map_err(|e| format!("remote executor error: {e}"))?;
                CommandExecutor::Remote(remote)
            }
            None => CommandExecutor::Local(LocalExecutor::new()),
        };

        let docker = DockerCli::from_executor(executor);

        // Get current traefik labels to remove stale ones.
        let inspect_res = docker
            .services()
            .inspect(&service_name)
            .await
            .map_err(|e| format!("service inspect failed: {e}"))?;

        let current = inspect_res.spec.labels;

        let stale: Vec<String> = current
            .keys()
            .filter(|k| k.starts_with("traefik."))
            .cloned()
            .collect();

        let mut update = docker.services().update(&service_name);
        let mut has_changes = false;
        for key in stale {
            update = update.label_rm(key);
            has_changes = true;
        }
        for label in &new_labels {
            if let Some((k, v)) = label.split_once('=') {
                update = update.label_add(k, v);
                has_changes = true;
            }
        }

        if !has_changes {
            return Ok(());
        }

        update
            .run()
            .await
            .map_err(|e| format!("service update failed: {e}"))?;

        tracing::info!(
            application_id,
            service = %service_name,
            labels = new_labels.len(),
            "traefik labels updated after domain change"
        );

        Ok(())
    }

    async fn apply_compose_traefik(&self, compose_id: i64) {
        if let Err(e) = self.try_apply_compose_traefik(compose_id).await {
            tracing::warn!(
                compose_id,
                error = %e,
                "could not apply traefik labels after compose domain change"
            );
        }
    }

    async fn try_apply_compose_traefik(&self, compose_id: i64) -> Result<(), String> {
        let compose = self
            .repo_compose
            .get_by_id(compose_id)
            .await
            .map_err(|e| format!("compose check error: {e}"))?
            .ok_or_else(|| format!("compose not found: {compose_id}"))?;
        let app_name = compose.app_name;
        let server_id = compose.server_id;
        let compose_type = compose.compose_type;

        let executor = match server_id {
            Some(sid) => {
                use crate::services::compose::remote::remote_executor;
                let remote = remote_executor(self.db.as_ref(), sid)
                    .await
                    .map_err(|e| format!("remote executor: {e}"))?;
                CommandExecutor::Remote(remote)
            }
            None => CommandExecutor::Local(LocalExecutor::new()),
        };
        let docker = DockerCli::from_executor(executor.clone());

        let domains = self
            .list_by_compose(compose_id)
            .await
            .map_err(|e| format!("could not load domains: {e}"))?;
        let v = ComposeType::try_from(compose_type.as_str())
            .map_err(|e| format!("invalid compose type: {e}"))?;
        match v {
            ComposeType::Stack => {
                let labels = build_compose_service_labels(&app_name, &domains);
                for (service_name, service_labels) in labels {
                    // Remove stale traefik labels first.
                    let inspect_res = docker
                        .services()
                        .inspect(&service_name)
                        .await
                        .map_err(|e| format!("service inspect failed: {e}"))?;
                    let current = inspect_res.spec.labels;
                    let stale: Vec<String> = current
                        .keys()
                        .filter(|k| k.starts_with("traefik."))
                        .cloned()
                        .collect();

                    let mut update = docker.services().update(&service_name);
                    let mut has_changes = false;
                    for key in stale {
                        update = update.label_rm(key);
                        has_changes = true;
                    }
                    for label in &service_labels {
                        if let Some((k, v)) = label.split_once('=') {
                            update = update.label_add(k, v);
                            has_changes = true;
                        }
                    }
                    if !has_changes {
                        continue;
                    }
                    update
                        .run()
                        .await
                        .map_err(|e| format!("service update labels failed: {e}"))?;

                    tracing::info!(compose_id, service = %service_name, "traefik labels updated for compose stack service");
                }
            }
            ComposeType::DockerCompose => {
                use crate::utils::paths::rustploy_paths;
                let paths = rustploy_paths();
                let env_file = format!(
                    "{}/{app_name}/.env",
                    paths.compose_dir("").trim_end_matches('/')
                );
                let compose_file_path = format!(
                    "{}/{app_name}/source/docker-compose.yml",
                    paths.compose_dir("").trim_end_matches('/')
                );

                if let Ok(raw_yaml) = tokio::fs::read_to_string(&compose_file_path).await {
                    let adapter =
                        crate::utils::builder::compose::adapter::db::ComposeSpecAdapter::new(
                            self.repo_compose.clone(),
                            self.repo_domain.clone(),
                            Arc::new(crate::repository::MountRepository::new(self.db.clone())),
                        );
                    if let Ok(spec) = adapter.load(compose_id).await {
                        if let Ok(updated_yaml) =
                            crate::utils::builder::compose::labels::inject_compose_yaml_labels(
                                &raw_yaml, &spec,
                            )
                        {
                            let _ = tokio::fs::write(&compose_file_path, updated_yaml).await;
                        }
                    }
                }

                docker
                    .compose()
                    .up()
                    .project(&app_name)
                    .env_file(&env_file)
                    .file(&compose_file_path)
                    .detach()
                    .custom_arg("--no-build")
                    .run()
                    .await
                    .map_err(|e| format!("compose up failed: {e}"))?;
                tracing::info!(compose_id, "traefik labels updated via compose up");
            }
        }

        Ok(())
    }
}

fn bool_to_i64(value: bool) -> i64 {
    if value { 1 } else { 0 }
}
