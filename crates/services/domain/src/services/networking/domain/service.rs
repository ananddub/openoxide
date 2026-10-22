use std::sync::Arc;

use auto_di::singleton;

use crate::db::queries::FlashQueries;
use crate::flash_gen::domains::DomainsRouteInUseExcludingParams;
use crate::services::compute::compose::ComposeType;
use crate::utils::builder::application::{
    adapter::ApplicationSpecAdapter, stack::application_traefik_labels,
};
use crate::utils::docker::DockerCli;
use crate::utils::traefik::types::CertificateType;

use super::labels::build_compose_service_labels;
use super::types::{CreateDomainDto, DnsValidation, DomainRecord, PatchDomainDto};

#[derive(Clone)]
pub struct DomainService {
    queries: Arc<FlashQueries>,
    application_spec: Arc<ApplicationSpecAdapter>,
}

#[singleton]
impl DomainService {
    fn new(queries: Arc<FlashQueries>, application_spec: Arc<ApplicationSpecAdapter>) -> Self {
        Self {
            queries,
            application_spec,
        }
    }

    pub async fn get_by_id(&self, id: i64, organization_id: i64) -> sqlx::Result<DomainRecord> {
        let record = self.queries.domains_get_record_by_id(id).await?;
        if record.organization_id != organization_id {
            return Err(sqlx::Error::RowNotFound);
        }
        Ok(DomainRecord::from(record))
    }

    pub async fn list_by_application(
        &self,
        application_id: i64,
    ) -> sqlx::Result<Vec<DomainRecord>> {
        let rows = self
            .queries
            .domains_list_by_application(application_id)
            .await?;
        Ok(rows.into_iter().map(DomainRecord::from).collect())
    }

    pub async fn list_by_compose(&self, compose_id: i64) -> sqlx::Result<Vec<DomainRecord>> {
        let rows = self.queries.domains_list_by_compose(compose_id).await?;
        Ok(rows.into_iter().map(DomainRecord::from).collect())
    }

    pub async fn list_all(&self, organization_id: i64) -> sqlx::Result<Vec<DomainRecord>> {
        let rows = self.queries.domains_list_all(organization_id).await?;
        Ok(rows.into_iter().map(DomainRecord::from).collect())
    }

    /// Live DNS check: resolve the host's A/AAAA records (CNAME chains are
    /// followed by the system resolver) and compare them with the target
    /// server's IP. No `server_id` means the panel's own machine.
    pub async fn validate_dns(
        &self,
        host: &str,
        server_id: Option<i64>,
        organization_id: i64,
    ) -> sqlx::Result<DnsValidation> {
        let host = normalize_host(host)?;
        let expected_ips = match server_id {
            Some(id) => {
                let server = self.queries.get_server_by_i_d(id).await?;
                if server.organization_id != organization_id {
                    return Err(sqlx::Error::RowNotFound);
                }
                vec![server.ip_address]
            }
            None => local_ip_addresses(),
        }
        .into_iter()
        .map(|ip| ip.trim().to_owned())
        .filter(|ip| !ip.is_empty())
        .collect::<Vec<_>>();

        let mut resolved_ips = match tokio::net::lookup_host((host.as_str(), 0)).await {
            Ok(addrs) => addrs.map(|addr| addr.ip().to_string()).collect::<Vec<_>>(),
            Err(_) => Vec::new(),
        };
        resolved_ips.sort();
        resolved_ips.dedup();

        let (is_valid, error) = if resolved_ips.is_empty() {
            (
                false,
                Some(format!("{host} does not resolve to any IP address yet")),
            )
        } else if expected_ips.is_empty() || resolved_ips.iter().any(|ip| expected_ips.contains(ip))
        {
            (true, None)
        } else {
            (
                false,
                Some(format!(
                    "{host} resolves to {} but should point to {}",
                    resolved_ips.join(", "),
                    expected_ips.join(", ")
                )),
            )
        };

        Ok(DnsValidation {
            is_valid,
            resolved_ips,
            expected_ips,
            error,
        })
    }

    pub async fn create(
        &self,
        input: CreateDomainDto,
        organization_id: i64,
    ) -> sqlx::Result<DomainRecord> {
        validate_owner(input.application_id, input.compose_id)?;
        let certificate_type = input.certificate_type.to_uppercase();
        let port = input.port.or(Some(3000));
        validate_domain_configuration(
            input.https,
            port,
            &input.path,
            &input.internal_path,
            input.strip_path,
            &certificate_type,
            input.custom_cert_resolver.as_deref(),
            input.compose_id,
            input.service_name.as_deref(),
        )?;
        self.validate_route_available(&input.host, &input.path, None)
            .await?;
        let https = bool_to_i64(input.https);
        let strip_path = bool_to_i64(input.strip_path);
        let domain_type = match (input.application_id, input.compose_id) {
            (Some(_), None) => "APPLICATION",
            (None, Some(_)) => "COMPOSE",
            _ => "APPLICATION",
        };
        let service_id = input.application_id.or(input.compose_id);

        let domain = self
            .queries
            .domains_create_and_return(&crate::flash_gen::domains::DomainsCreateAndReturnParams {
                host: input.host,
                https,
                port: port.unwrap_or(3000),
                path: input.path,
                internal_path: input.internal_path,
                custom_entrypoint: input.custom_entrypoint.unwrap_or_default(),
                service_name: input.service_name.unwrap_or_default(),
                custom_cert_resolver: input.custom_cert_resolver.unwrap_or_default(),
                strip_path,
                middlewares: input.middlewares,
                domain_type: domain_type.to_string(),
                certificate_type,
                service_id: service_id.unwrap_or(0),
                organization_id,
            })
            .await
            .map(DomainRecord::from)?;

        self.spawn_apply(&domain);
        Ok(domain)
    }

    pub async fn patch(
        &self,
        id: i64,
        input: PatchDomainDto,
        organization_id: i64,
    ) -> sqlx::Result<DomainRecord> {
        let current = self.get_by_id(id, organization_id).await?;
        let host = input.host.unwrap_or(current.host);
        let https = input.https.map(bool_to_i64).unwrap_or(current.https);
        let port = input.port.or(current.port);
        let path = input.path.or(current.path);
        self.validate_route_available(&host, path.as_deref().unwrap_or("/"), Some(id))
            .await?;
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
        validate_domain_configuration(
            https != 0,
            port,
            path.as_deref().unwrap_or("/"),
            internal_path.as_deref().unwrap_or("/"),
            strip_path != 0,
            &certificate_type,
            custom_cert_resolver.as_deref(),
            if current.domain_type == "COMPOSE" {
                current.service_id
            } else {
                None
            },
            service_name.as_deref(),
        )?;

        let domain = self
            .queries
            .domains_update_and_return(&crate::flash_gen::domains::DomainsUpdateAndReturnParams {
                host,
                https,
                port: port.unwrap_or(3000),
                path: path.unwrap_or_else(|| "/".into()),
                internal_path: internal_path.unwrap_or_else(|| "/".into()),
                custom_entrypoint: custom_entrypoint.unwrap_or_default(),
                service_name: service_name.unwrap_or_default(),
                custom_cert_resolver: custom_cert_resolver.unwrap_or_default(),
                strip_path,
                middlewares,
                certificate_type,
                id,
                organization_id,
            })
            .await
            .map(DomainRecord::from)?;

        // Auto-apply traefik labels asynchronously after domain update.
        self.spawn_apply(&domain);
        Ok(domain)
    }

    pub async fn delete(&self, id: i64, organization_id: i64) -> sqlx::Result<()> {
        let domain = self.get_by_id(id, organization_id).await?;
        self.queries.domains_delete(id, organization_id).await?;
        self.spawn_apply(&domain);
        Ok(())
    }

    fn spawn_apply(&self, domain: &DomainRecord) {
        let this = self.clone();
        let domain = domain.clone();
        tokio::spawn(async move {
            if let Some(service_id) = domain.service_id.filter(|&id| id > 0) {
                if domain.domain_type == "COMPOSE" {
                    this.apply_compose_traefik(service_id).await;
                } else {
                    this.apply_application_traefik(service_id).await;
                }
            }
        });
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

    async fn validate_route_available(
        &self,
        host: &str,
        path: &str,
        exclude_id: Option<i64>,
    ) -> sqlx::Result<()> {
        let host = normalize_host(host)?;
        let path = normalize_path(path);
        let in_use = match exclude_id {
            Some(exclude_id) => {
                self.queries
                    .domains_route_in_use_excluding(&DomainsRouteInUseExcludingParams {
                        host: host.clone(),
                        path: path.clone(),
                        id: exclude_id,
                    })
                    .await?
            }
            None => self.queries.domains_route_in_use(&host, &path).await?,
        };
        if in_use != 0 {
            return Err(sqlx::Error::Protocol(format!(
                "domain route already exists for {host}{path}"
            )));
        }
        Ok(())
    }

    async fn try_apply_application_traefik(&self, application_id: i64) -> Result<(), String> {
        let app = self
            .queries
            .get_service_by_i_d(application_id)
            .await
            .map_err(|e| format!("app check error: {e}"))?;
        let app_name = app.app_name;
        let server_id = app.server_id;

        let service_name = format!("{app_name}_{app_name}");

        let spec = self
            .application_spec
            .load(application_id)
            .await
            .map_err(|e| format!("could not load application routing configuration: {e}"))?;
        let new_labels = application_traefik_labels(&spec);

        // Build docker executor (local or remote).
        let executor = crate::services::compute::compose::remote::agent_executor_for(server_id)
            .await
            .map_err(|e| format!("Agent executor error: {e}"))?;

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
            .queries
            .get_service_by_i_d(compose_id)
            .await
            .map_err(|e| format!("compose check error: {e}"))?;
        let app_name = compose.app_name;
        let server_id = compose.server_id;
        let compose_type = self
            .queries
            .get_service_compose(compose_id)
            .await
            .map_err(|e| format!("compose type load error: {e}"))?
            .compose_type;

        let executor = crate::services::compute::compose::remote::agent_executor_for(server_id)
            .await
            .map_err(|e| format!("Agent executor: {e}"))?;
        let docker = DockerCli::from_executor(executor.clone());

        let domains = self
            .list_by_compose(compose_id)
            .await
            .map_err(|e| format!("could not load domains: {e}"))?;
        let v = ComposeType::from(compose_type.as_str());
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
                // The compose files live on the server's filesystem (Agent
                // executor), not the panel container — every read/write goes
                // through OsCli, and the canonical paths come from the spec
                // (RAW sources keep their file outside `source/`).
                let adapter = crate::utils::builder::compose::adapter::db::ComposeSpecAdapter::new(
                    self.queries.clone(),
                );
                let spec = adapter
                    .load(compose_id)
                    .await
                    .map_err(|e| format!("compose spec load failed: {e}"))?;
                let compose_file_path = spec.compose_file_path();
                let os = crate::utils::os::OsCli::new(&executor);
                if let Some(parent) = std::path::Path::new(&compose_file_path).parent() {
                    let _ = os
                        .dir(parent.to_string_lossy().as_ref())
                        .create()
                        .run()
                        .await;
                }

                // A wiped server workspace must not brick domain updates:
                // RAW stacks keep their compose content in the database, so
                // re-materialize the file when it is gone.
                let raw_present = os.file(&compose_file_path).exists().run().await.is_ok();
                if !raw_present
                    && let crate::utils::builder::compose::spec::ComposeSource::Raw { content } =
                        &spec.source
                {
                    let _ = os.file(&compose_file_path).write(content).execute().await;
                }

                if let Ok(output) = os.file(&compose_file_path).read().execute().await
                    && output.status.success()
                    && let Ok(updated_yaml) =
                        crate::utils::builder::compose::labels::inject_compose_yaml_labels(
                            &output.stdout,
                            &spec,
                        )
                {
                    let _ = os
                        .file(&compose_file_path)
                        .write(&updated_yaml)
                        .execute()
                        .await;
                }

                // `compose up --env-file` hard-fails on a missing file; the
                // builder writes it at deploy time, but a wiped workspace
                // must not brick domain updates. Recreate it from the stored
                // environment, mirroring the builder's env-file format.
                if os.file(&spec.env_file).exists().run().await.is_err() {
                    let env_text: String = spec
                        .environment
                        .iter()
                        .map(|(key, value)| format!("{key}={value}\n"))
                        .collect();
                    let _ = os.file(&spec.env_file).write(env_text).execute().await;
                }

                docker
                    .compose()
                    .up()
                    .project(&app_name)
                    .env_file(&spec.env_file)
                    .file(&compose_file_path)
                    .detach()
                    .no_build()
                    .run()
                    .await
                    .map_err(|e| format!("compose up failed: {e}"))?;
                tracing::info!(compose_id, "traefik labels updated via compose up");
            }
        }

        Ok(())
    }
}

#[allow(clippy::too_many_arguments)]
fn validate_domain_configuration(
    https: bool,
    port: Option<i64>,
    path: &str,
    internal_path: &str,
    strip_path: bool,
    certificate_type: &str,
    custom_cert_resolver: Option<&str>,
    compose_id: Option<i64>,
    service_name: Option<&str>,
) -> sqlx::Result<()> {
    if !path.starts_with('/') || !internal_path.starts_with('/') {
        return Err(sqlx::Error::Protocol(
            "domain paths must start with /".into(),
        ));
    }
    if !matches!(port, Some(1..=65_535)) {
        return Err(sqlx::Error::Protocol(
            "container port must be between 1 and 65535".into(),
        ));
    }
    if strip_path && path == "/" {
        return Err(sqlx::Error::Protocol(
            "strip_path requires a non-root public path".into(),
        ));
    }

    let certificate_type = CertificateType::from(certificate_type);
    if !https && certificate_type != CertificateType::None {
        return Err(sqlx::Error::Protocol(
            "certificate_type must be NONE when HTTPS is disabled".into(),
        ));
    }
    if https
        && certificate_type == CertificateType::Custom
        && custom_cert_resolver.is_none_or(|value| value.trim().is_empty())
    {
        return Err(sqlx::Error::Protocol(
            "custom_cert_resolver is required for CUSTOM certificates".into(),
        ));
    }
    if compose_id.is_some() && service_name.is_none_or(|value| value.trim().is_empty()) {
        return Err(sqlx::Error::Protocol(
            "service_name is required for compose domains".into(),
        ));
    }
    Ok(())
}

fn bool_to_i64(value: bool) -> i64 {
    if value { 1 } else { 0 }
}

fn validate_owner(application_id: Option<i64>, compose_id: Option<i64>) -> sqlx::Result<()> {
    if application_id.is_some() == compose_id.is_some() {
        return Err(sqlx::Error::Protocol(
            "exactly one of application_id or compose_id is required".into(),
        ));
    }
    Ok(())
}

/// The machine's own IPs: loopback plus the outbound address each UDP
/// "connect" probe selects (no packets are actually sent).
fn local_ip_addresses() -> Vec<String> {
    let mut ips = vec!["127.0.0.1".to_string(), "::1".to_string()];
    for (bind, probe) in [
        ("0.0.0.0:0", "8.8.8.8:80"),
        ("[::]:0", "[2001:4860:4860::8888]:53"),
    ] {
        if let Ok(socket) = std::net::UdpSocket::bind(bind)
            && socket.connect(probe).is_ok()
            && let Ok(addr) = socket.local_addr()
        {
            ips.push(addr.ip().to_string());
        }
    }
    ips
}

fn normalize_host(value: &str) -> sqlx::Result<String> {
    let host = value.trim().trim_end_matches('.').to_ascii_lowercase();
    if host.is_empty()
        || host.contains('/')
        || host.contains(':')
        || host.chars().any(char::is_whitespace)
    {
        return Err(sqlx::Error::Protocol("invalid domain host".into()));
    }
    Ok(host)
}

fn normalize_path(value: &str) -> String {
    let value = value.trim();
    if value.is_empty() || value == "/" {
        "/".into()
    } else {
        format!("/{}", value.trim_matches('/'))
    }
}
