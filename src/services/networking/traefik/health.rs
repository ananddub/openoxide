use crate::utils::os::OsCli;

use super::service::TraefikService;
use super::types::{
    TraefikHealthResponseDto, TraefikLogEntryDto, TraefikRequestsStatusDto,
    TraefikStatsLogsQueryDto, TraefikStatsLogsResponseDto, TraefikToggleRequestsDto,
};

impl TraefikService {
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

                if query.status.is_some_and(|expected| expected != status) {
                    continue;
                }

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
