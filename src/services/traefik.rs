use auto_di::singleton;
use sqlx::SqlitePool;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::api::dto::traefik::{
    StructuredMiddlewareDto, StructuredMiddlewareKind, StructuredMiddlewareResponseDto,
    TraefikFileContentDto, TraefikFileNodeDto, TraefikFileTreeNodeDto, TraefikHealthResponseDto,
    TraefikLogEntryDto, TraefikRequestsStatusDto, TraefikStatsLogsQueryDto,
    TraefikStatsLogsResponseDto, TraefikToggleRequestsDto, TraefikVersionDto, TraefikWriteFileDto,
    UpdateTraefikVersionDto,
};
use crate::utils::exec::{CommandExecutor, LocalExecutor, RemoteExecutor};
use crate::utils::os::OsCli;
use crate::utils::paths::rustploy_paths;

pub struct TraefikService {
    db: Arc<SqlitePool>,
}

#[singleton]
impl TraefikService {
    fn new(db: Arc<SqlitePool>) -> Self {
        Self { db }
    }

    pub fn get_traefik_base_path() -> String {
        std::env::var("TRAEFIK_BASE_PATH")
            .ok()
            .map(|v| v.trim().trim_end_matches('/').to_string())
            .filter(|v| !v.is_empty())
            .unwrap_or_else(|| format!("{}/traefik", rustploy_paths().base))
    }

    async fn get_executor(&self, server_id: Option<i64>) -> Result<CommandExecutor, String> {
        match server_id {
            Some(id) if id > 0 => {
                let server_repo = auto_di::resolve::<crate::repository::ServerRepository>()
                    .await
                    .map_err(|e| e.to_string())?;
                let creds = server_repo
                    .get_ssh_credentials(id)
                    .await
                    .map_err(|e| e.to_string())?
                    .ok_or_else(|| format!("Server {} not found", id))?;
                let port = u16::try_from(creds.1).map_err(|e| e.to_string())?;
                let remote = RemoteExecutor::new(
                    creds.0,
                    port,
                    creds.2,
                    crate::utils::exec::SshAuth::key_pair(creds.3, creds.4),
                    crate::utils::exec::SshHostKey::InsecureAcceptAny,
                )
                .with_sudo();
                Ok(CommandExecutor::Remote(remote))
            }
            _ => Ok(CommandExecutor::Local(LocalExecutor::new())),
        }
    }

    pub fn sanitize_read_path(relative_path: &str) -> Result<PathBuf, String> {
        Self::sanitize_traefik_path(relative_path, true)
    }

    pub fn sanitize_write_path(relative_path: &str) -> Result<PathBuf, String> {
        Self::sanitize_traefik_path(relative_path, false)
    }

    fn sanitize_traefik_path(
        relative_path: &str,
        allow_etc_traefik: bool,
    ) -> Result<PathBuf, String> {
        let clean_path = relative_path.trim();
        if clean_path.is_empty() {
            return Err("Security Error: File path is required".to_string());
        }
        if clean_path.contains('\0') || clean_path.contains('\\') {
            return Err("Security Error: Invalid file path".to_string());
        }

        let base_dir = Self::get_traefik_base_path();
        let base_path = Path::new(&base_dir);
        let requested_path = Path::new(clean_path);
        let full_path = if requested_path.is_absolute() {
            requested_path.to_path_buf()
        } else {
            base_path.join(requested_path)
        };

        if full_path
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
        {
            return Err("Security Error: Path traversal detected in file path".to_string());
        }

        let inside_base = full_path.starts_with(base_path);
        let inside_etc_traefik =
            allow_etc_traefik && full_path.starts_with(Path::new("/etc/traefik"));
        if !inside_base && !inside_etc_traefik {
            return Err(format!(
                "Security Error: File path must remain inside {}",
                base_dir
            ));
        }

        let file_name = full_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        let is_yml = full_path
            .extension()
            .map_or(false, |ext| ext == "yml" || ext == "yaml" || ext == "json");
        let is_acme = file_name == "acme.json";

        if !is_yml && !is_acme {
            return Err(
                "Security Restriction: Only .yml, .yaml, and .json files are accessible"
                    .to_string(),
            );
        }

        Ok(full_path)
    }

    pub async fn list_files(
        &self,
        server_id: Option<i64>,
    ) -> Result<Vec<TraefikFileNodeDto>, String> {
        let executor = self.get_executor(server_id).await?;
        let os = OsCli::new(&executor);
        let base_dir = Self::get_traefik_base_path();

        let output = os
            .dir(&base_dir)
            .walk()
            .also("/etc/traefik")
            .max_depth(3)
            .type_file()
            .names(["*.yml", "*.yaml", "*.json"])
            .output(crate::utils::os::dir::DirWalkOutput::PathSizeModifiedEpoch)
            .ignore_errors()
            .run()
            .await
            .map_err(|e| format!("Failed to list Traefik configuration files: {}", e))?;

        let stdout = output.stdout.as_str();
        let mut nodes = Vec::new();

        for line in stdout.lines() {
            let mut parts = line.splitn(3, '\t');
            let path_str = parts.next().unwrap_or_default().trim();
            if path_str.is_empty() {
                continue;
            }
            let size = parts
                .next()
                .and_then(|value| value.parse::<u64>().ok())
                .unwrap_or(0);
            let modified_at = parts
                .next()
                .and_then(|value| value.parse::<f64>().ok())
                .map(|value| value.max(0.0) as u64)
                .unwrap_or(0);
            let path_obj = Path::new(path_str);

            let file_name = path_obj
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| path_str.to_string());

            let is_valid_ext = file_name.ends_with(".yml")
                || file_name.ends_with(".yaml")
                || file_name.ends_with(".json");
            if !is_valid_ext {
                continue;
            }

            let relative = if path_str.starts_with(&base_dir) {
                path_str
                    .strip_prefix(&base_dir)
                    .unwrap_or(path_str)
                    .trim_start_matches('/')
                    .to_string()
            } else if path_str.starts_with("/etc/traefik") {
                path_str
                    .strip_prefix("/etc/traefik")
                    .unwrap_or(path_str)
                    .trim_start_matches('/')
                    .to_string()
            } else {
                file_name.clone()
            };

            let is_readonly = file_name == "acme.json";
            nodes.push(TraefikFileNodeDto {
                name: file_name,
                relative_path: relative,
                size,
                is_readonly,
                modified_at,
            });
        }

        nodes.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));
        nodes.dedup_by(|a, b| a.relative_path == b.relative_path);

        Ok(nodes)
    }

    pub async fn list_file_tree(
        &self,
        server_id: Option<i64>,
    ) -> Result<Vec<TraefikFileTreeNodeDto>, String> {
        let files = self.list_files(server_id).await?;
        Ok(build_file_tree(&files))
    }

    pub async fn read_file(
        &self,
        server_id: Option<i64>,
        relative_path: &str,
    ) -> Result<TraefikFileContentDto, String> {
        let full_path = Self::sanitize_read_path(relative_path)?;
        let path_str = full_path.to_string_lossy().to_string();
        let executor = self.get_executor(server_id).await?;
        let os = OsCli::new(&executor);

        let output = os
            .file(&path_str)
            .read()
            .execute()
            .await
            .map_err(|e| format!("Failed to read file content: {}", e))?;

        if !output.status.success() {
            let err_msg = output.stderr.clone();
            return Err(format!("Could not read file '{}': {}", path_str, err_msg));
        }

        let content = output.stdout.clone();
        let is_readonly = path_str.ends_with("acme.json");

        Ok(TraefikFileContentDto {
            path: relative_path.to_string(),
            content,
            is_readonly,
        })
    }

    pub async fn write_file(&self, payload: TraefikWriteFileDto) -> Result<(), String> {
        let full_path = Self::sanitize_write_path(&payload.path)?;
        let path_str = full_path.to_string_lossy().to_string();

        if path_str.ends_with("acme.json") {
            return Err(
                "Security Violation: acme.json is read-only and cannot be modified".to_string(),
            );
        }

        if path_str.ends_with(".yml") || path_str.ends_with(".yaml") {
            let _parsed: serde_yaml::Value = serde_yaml::from_str(&payload.content)
                .map_err(|e| format!("YAML Syntax Validation Error: {}", e))?;
        }

        let executor = self.get_executor(payload.server_id).await?;
        let os = OsCli::new(&executor);

        if let Some(parent) = full_path.parent() {
            let parent_str = parent.to_string_lossy().to_string();
            let _ = os.dir(&parent_str).create().run().await;
        }

        let backup_path = format!("{}.bak", path_str);
        let _ = os.file(&path_str).copy(&backup_path).execute().await;

        let tmp_path = format!("{}.tmp", path_str);
        let write_out = os
            .file(&tmp_path)
            .write(&payload.content)
            .execute()
            .await
            .map_err(|e| format!("Failed to write temporary file: {}", e))?;

        if !write_out.status.success() {
            let err = write_out.stderr.clone();
            return Err(format!("Failed to write content: {}", err));
        }

        let rename_out = os
            .file(&tmp_path)
            .move_to(&path_str)
            .execute()
            .await
            .map_err(|e| format!("Failed to apply file updates: {}", e))?;

        if !rename_out.status.success() {
            let err = rename_out.stderr.clone();
            return Err(format!("Failed to finalize file update: {}", err));
        }

        tokio::time::sleep(std::time::Duration::from_millis(750)).await;
        let health = self.check_health(payload.server_id).await?;
        if !health.is_healthy {
            let rollback = os.file(&backup_path).copy(&path_str).execute().await;
            return Err(format!(
                "Traefik rejected the configuration; previous file restored: {}{}",
                health.configuration_errors.join("; "),
                rollback
                    .err()
                    .map(|error| format!("; rollback error: {error}"))
                    .unwrap_or_default()
            ));
        }

        Ok(())
    }

    pub async fn version(&self, server_id: Option<i64>) -> Result<TraefikVersionDto, String> {
        let executor = self.get_executor(server_id).await?;
        let inspect = crate::utils::docker::DockerCli::from_executor(executor)
            .containers()
            .inspect("rustploy-traefik")
            .await
            .map_err(|error| error.to_string())?;
        Ok(TraefikVersionDto {
            server_id,
            current_image: inspect.config.image,
            desired_version: "3.6.7".into(),
        })
    }

    pub async fn update_version(
        &self,
        input: UpdateTraefikVersionDto,
    ) -> Result<TraefikVersionDto, String> {
        let version = input.version.trim().trim_start_matches('v');
        if version.is_empty()
            || !version
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-'))
        {
            return Err("invalid Traefik version".into());
        }
        let executor = self.get_executor(input.server_id).await?;
        let docker = crate::utils::docker::DockerCli::from_executor(executor.clone());
        docker
            .images()
            .pull(format!("traefik:v{version}"))
            .pull()
            .await
            .map_err(|error| error.to_string())?;
        let _ = docker
            .containers()
            .rm("rustploy-traefik")
            .force()
            .run()
            .await;
        let mut config = crate::utils::setup::SetupConfig::default();
        config.traefik_version = version.to_owned();
        crate::utils::setup::ServerSetup::new(executor, config)
            .ensure_traefik()
            .await
            .map_err(|error| error.to_string())?;
        self.version(input.server_id).await
    }

    pub fn structured_middleware(
        input: StructuredMiddlewareDto,
    ) -> Result<StructuredMiddlewareResponseDto, String> {
        use crate::utils::traefik::middleware::Middleware;
        let name = input.name.trim().to_owned();
        if name.is_empty()
            || !name
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_'))
        {
            return Err("invalid middleware name".into());
        }
        let value = |key: &str| {
            input
                .values
                .get(key)
                .cloned()
                .ok_or_else(|| format!("missing middleware value: {key}"))
        };
        let middleware = match input.kind {
            StructuredMiddlewareKind::StripPrefix => Middleware::StripPrefix {
                name,
                prefixes: input.list,
            },
            StructuredMiddlewareKind::AddPrefix => Middleware::AddPrefix {
                name,
                prefix: value("prefix")?,
            },
            StructuredMiddlewareKind::RedirectScheme => Middleware::RedirectScheme {
                name,
                scheme: value("scheme")?,
                permanent: input.values.get("permanent").is_some_and(|v| v == "true"),
            },
            StructuredMiddlewareKind::Compress => Middleware::Compress { name },
            StructuredMiddlewareKind::RateLimit => Middleware::RateLimit {
                name,
                average: value("average")?.parse().map_err(|_| "invalid average")?,
                burst: value("burst")?.parse().map_err(|_| "invalid burst")?,
            },
            StructuredMiddlewareKind::IpAllowList => Middleware::IpAllowList {
                name,
                source_ranges: input.list,
            },
            StructuredMiddlewareKind::RequestHeaders => Middleware::RequestHeaders {
                name,
                headers: input.values.into_iter().collect(),
            },
            StructuredMiddlewareKind::ResponseHeaders => Middleware::ResponseHeaders {
                name,
                headers: input.values.into_iter().collect(),
            },
        };
        Ok(StructuredMiddlewareResponseDto {
            reference: middleware.reference(),
            labels: middleware.labels().into_iter().collect(),
        })
    }

    pub async fn check_health(
        &self,
        server_id: Option<i64>,
    ) -> Result<TraefikHealthResponseDto, String> {
        let executor = self.get_executor(server_id).await?;
        let os = OsCli::new(&executor);

        let output = os
            .http()
            .get("http://127.0.0.1:8080/api/rawdata")
            .timeout(5)
            .execute()
            .await;

        let raw = match output {
            Ok(out) if out.status.success() => out.stdout.trim().to_string(),
            _ => String::new(),
        };

        if raw.is_empty() {
            return Ok(TraefikHealthResponseDto {
                is_healthy: false,
                rawdata_status: "unreachable".to_string(),
                configuration_errors: vec![
                    "Traefik API (:8080) unreachable or returning empty rawdata".to_string(),
                ],
            });
        }

        let mut config_errors = Vec::new();
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&raw) {
            if let Some(routers) = v.get("routers").and_then(|r| r.as_object()) {
                for (name, r_val) in routers {
                    if let Some(err) = r_val.get("error").and_then(|e| e.as_str()) {
                        config_errors.push(format!("Router '{}': {}", name, err));
                    }
                }
            }
            if let Some(services) = v.get("services").and_then(|s| s.as_object()) {
                for (name, s_val) in services {
                    if let Some(err) = s_val.get("error").and_then(|e| e.as_str()) {
                        config_errors.push(format!("Service '{}': {}", name, err));
                    }
                }
            }
        }

        let is_healthy = config_errors.is_empty();

        Ok(TraefikHealthResponseDto {
            is_healthy,
            rawdata_status: "ok".to_string(),
            configuration_errors: config_errors,
        })
    }

    pub async fn have_activated_requests(
        &self,
        server_id: Option<i64>,
    ) -> Result<TraefikRequestsStatusDto, String> {
        let executor = self.get_executor(server_id).await?;
        let os = OsCli::new(&executor);
        let base_dir = Self::get_traefik_base_path();
        let main_config_path = format!("{}/traefik.yml", base_dir);

        let out = os.file(&main_config_path).read().execute().await;
        let is_active = match out {
            Ok(o) if o.status.success() => {
                let content = o.stdout;
                content.contains("accessLog:") || content.contains("accesslog:")
            }
            _ => false,
        };

        Ok(TraefikRequestsStatusDto {
            is_active,
            log_path: format!("{}/dynamic/access.log", base_dir),
            cron_expression: "0 0 * * *".to_string(),
        })
    }

    pub async fn toggle_requests(&self, payload: TraefikToggleRequestsDto) -> Result<bool, String> {
        let executor = self.get_executor(payload.server_id).await?;
        let os = OsCli::new(&executor);
        let base_dir = Self::get_traefik_base_path();
        let main_config_path = format!("{}/traefik.yml", base_dir);

        let existing_content = match os.file(&main_config_path).read().execute().await {
            Ok(o) if o.status.success() => o.stdout,
            _ => "# Traefik Static Configuration\n".to_string(),
        };

        let mut val: serde_yaml::Value = serde_yaml::from_str(&existing_content)
            .unwrap_or_else(|_| serde_yaml::Value::Mapping(serde_yaml::Mapping::new()));

        if let serde_yaml::Value::Mapping(ref mut map) = val {
            if payload.enable {
                let mut access_log_map = serde_yaml::Mapping::new();
                access_log_map.insert(
                    serde_yaml::Value::String("filePath".to_string()),
                    serde_yaml::Value::String(format!("{}/dynamic/access.log", base_dir)),
                );
                access_log_map.insert(
                    serde_yaml::Value::String("format".to_string()),
                    serde_yaml::Value::String("json".to_string()),
                );
                access_log_map.insert(
                    serde_yaml::Value::String("bufferingSize".to_string()),
                    serde_yaml::Value::Number(serde_yaml::Number::from(100)),
                );

                map.insert(
                    serde_yaml::Value::String("accessLog".to_string()),
                    serde_yaml::Value::Mapping(access_log_map),
                );
            } else {
                map.remove(&serde_yaml::Value::String("accessLog".to_string()));
                map.remove(&serde_yaml::Value::String("accesslog".to_string()));
            }
        }

        let updated_str =
            serde_yaml::to_string(&val).map_err(|e| format!("YAML Serialization Error: {}", e))?;

        let tmp_path = format!("{}.tmp", main_config_path);
        let _ = os
            .file(&tmp_path)
            .write(&updated_str)
            .execute()
            .await
            .map_err(|e| format!("Failed to write temporary config file: {}", e))?;
        let _ = os
            .file(&tmp_path)
            .move_to(&main_config_path)
            .execute()
            .await
            .map_err(|e| format!("Failed to activate config: {}", e))?;

        Ok(payload.enable)
    }

    pub async fn read_stats_logs(
        &self,
        query: TraefikStatsLogsQueryDto,
    ) -> Result<TraefikStatsLogsResponseDto, String> {
        let executor = self.get_executor(query.server_id).await?;
        let os = OsCli::new(&executor);
        let base_dir = Self::get_traefik_base_path();
        let log_file_path = format!("{}/dynamic/access.log", base_dir);

        let out = os.file(&log_file_path).read().execute().await;
        let stdout = match out {
            Ok(o) if o.status.success() => o.stdout,
            _ => String::new(),
        };

        let mut entries = Vec::new();
        let search_query = query.search.as_deref().unwrap_or("").to_lowercase();

        for line in stdout.lines().rev() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            if let Ok(v) = serde_json::from_str::<serde_json::Value>(trimmed) {
                let timestamp = v
                    .get("time")
                    .or_else(|| v.get("StartUTC"))
                    .and_then(|s| s.as_str())
                    .unwrap_or("")
                    .to_string();

                let client_ip = v
                    .get("ClientHost")
                    .or_else(|| v.get("ClientAddr"))
                    .and_then(|s| s.as_str())
                    .unwrap_or("-")
                    .to_string();

                let method = v
                    .get("RequestMethod")
                    .and_then(|s| s.as_str())
                    .unwrap_or("GET")
                    .to_string();
                let path = v
                    .get("RequestPath")
                    .and_then(|s| s.as_str())
                    .unwrap_or("/")
                    .to_string();

                let status = v
                    .get("DownstreamStatus")
                    .or_else(|| v.get("OriginStatus"))
                    .and_then(|s| s.as_u64())
                    .unwrap_or(200) as u16;

                let duration_ns = v.get("Duration").and_then(|s| s.as_f64()).unwrap_or(0.0);
                let duration_ms = duration_ns / 1_000_000.0;

                let service_name = v
                    .get("ServiceName")
                    .and_then(|s| s.as_str())
                    .unwrap_or("-")
                    .to_string();
                let router_name = v
                    .get("RouterName")
                    .and_then(|s| s.as_str())
                    .unwrap_or("-")
                    .to_string();

                if !search_query.is_empty() {
                    let matches = path.to_lowercase().contains(&search_query)
                        || method.to_lowercase().contains(&search_query)
                        || service_name.to_lowercase().contains(&search_query)
                        || router_name.to_lowercase().contains(&search_query)
                        || status.to_string().contains(&search_query);
                    if !matches {
                        continue;
                    }
                }

                entries.push(TraefikLogEntryDto {
                    timestamp,
                    client_ip,
                    method,
                    path,
                    status,
                    duration_ms,
                    service_name,
                    router_name,
                });
            }
        }

        let total_count = entries.len();
        let page = query.page.unwrap_or(1).max(1);
        let page_size = query.page_size.unwrap_or(25).clamp(1, 100);

        let start = (page - 1) * page_size;
        let items = if start < total_count {
            let end = (start + page_size).min(total_count);
            entries[start..end].to_vec()
        } else {
            Vec::new()
        };

        Ok(TraefikStatsLogsResponseDto {
            items,
            total_count,
            page,
            page_size,
        })
    }
}

fn build_file_tree(files: &[TraefikFileNodeDto]) -> Vec<TraefikFileTreeNodeDto> {
    let mut roots = Vec::new();

    for file in files {
        let parts = file
            .relative_path
            .split('/')
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>();
        if parts.is_empty() {
            continue;
        }
        insert_file_tree_node(&mut roots, &parts, file);
    }

    sort_file_tree(&mut roots);
    roots
}

fn insert_file_tree_node(
    siblings: &mut Vec<TraefikFileTreeNodeDto>,
    parts: &[&str],
    file: &TraefikFileNodeDto,
) {
    let name = parts[0];
    let is_file = parts.len() == 1;
    let relative_path = if is_file {
        file.relative_path.clone()
    } else {
        let depth = file.relative_path.split('/').count() - parts.len() + 1;
        file.relative_path
            .split('/')
            .take(depth)
            .collect::<Vec<_>>()
            .join("/")
    };

    let node_index = siblings
        .iter()
        .position(|node| node.name == name && node.relative_path == relative_path)
        .unwrap_or_else(|| {
            siblings.push(TraefikFileTreeNodeDto {
                name: name.to_string(),
                relative_path: relative_path.clone(),
                node_type: if is_file { "file" } else { "directory" }.to_string(),
                size: if is_file { file.size } else { 0 },
                is_readonly: is_file && file.is_readonly,
                modified_at: if is_file { file.modified_at } else { 0 },
                children: Vec::new(),
            });
            siblings.len() - 1
        });

    if is_file {
        let node = &mut siblings[node_index];
        node.node_type = "file".to_string();
        node.size = file.size;
        node.is_readonly = file.is_readonly;
        node.modified_at = file.modified_at;
    } else {
        insert_file_tree_node(&mut siblings[node_index].children, &parts[1..], file);
    }
}

fn sort_file_tree(nodes: &mut [TraefikFileTreeNodeDto]) {
    nodes.sort_by(|a, b| {
        match (
            a.node_type.as_str() == "directory",
            b.node_type.as_str() == "directory",
        ) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.cmp(&b.name),
        }
    });
    for node in nodes {
        sort_file_tree(&mut node.children);
    }
}

#[cfg(test)]
mod tests {
    use super::{TraefikService, build_file_tree};
    use crate::api::dto::traefik::TraefikFileNodeDto;
    use std::path::Path;

    #[test]
    fn sanitize_write_path_keeps_relative_paths_inside_rustploy_traefik_base() {
        let path = TraefikService::sanitize_write_path("dynamic/middlewares.yml").unwrap();
        assert!(path.starts_with(TraefikService::get_traefik_base_path()));
        assert!(path.ends_with(Path::new("dynamic/middlewares.yml")));
    }

    #[test]
    fn sanitize_path_rejects_traversal_and_backslashes() {
        assert!(TraefikService::sanitize_read_path("../secret.yml").is_err());
        assert!(TraefikService::sanitize_read_path("dynamic/../secret.yml").is_err());
        assert!(TraefikService::sanitize_read_path("dynamic\\secret.yml").is_err());
    }

    #[test]
    fn sanitize_path_allows_etc_traefik_reads_but_not_writes() {
        assert!(TraefikService::sanitize_read_path("/etc/traefik/traefik.yml").is_ok());
        assert!(TraefikService::sanitize_write_path("/etc/traefik/traefik.yml").is_err());
    }

    #[test]
    fn sanitize_path_restricts_file_types() {
        assert!(TraefikService::sanitize_read_path("dynamic/middlewares.yml").is_ok());
        assert!(TraefikService::sanitize_read_path("dynamic/acme.json").is_ok());
        assert!(TraefikService::sanitize_read_path("dynamic/access.log").is_err());
    }

    #[test]
    fn builds_tree_from_flat_file_nodes() {
        let tree = build_file_tree(&[
            TraefikFileNodeDto {
                name: "traefik.yml".into(),
                relative_path: "traefik.yml".into(),
                size: 620,
                is_readonly: false,
                modified_at: 10,
            },
            TraefikFileNodeDto {
                name: "middlewares.yml".into(),
                relative_path: "dynamic/middlewares.yml".into(),
                size: 112,
                is_readonly: false,
                modified_at: 11,
            },
            TraefikFileNodeDto {
                name: "acme.json".into(),
                relative_path: "dynamic/acme.json".into(),
                size: 2,
                is_readonly: true,
                modified_at: 12,
            },
        ]);

        assert_eq!(tree.len(), 2);
        assert_eq!(tree[0].name, "dynamic");
        assert_eq!(tree[0].node_type, "directory");
        assert_eq!(tree[0].children.len(), 2);
        assert_eq!(tree[0].children[0].name, "acme.json");
        assert!(tree[0].children[0].is_readonly);
        assert_eq!(tree[1].name, "traefik.yml");
        assert_eq!(tree[1].node_type, "file");
    }
}
