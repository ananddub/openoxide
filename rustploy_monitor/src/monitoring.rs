use crate::db::ServerMetric;
use reqwest::Client;
use serde::Serialize;
use std::sync::Mutex;
use sysinfo::{Disks, Networks, System};
use tracing::{error, info};

#[derive(Debug, Serialize)]
pub struct AlertPayloadInner {
    #[serde(rename = "ServerType")]
    pub server_type: String,
    #[serde(rename = "Type")]
    pub alert_type: String,
    #[serde(rename = "Value")]
    pub value: f64,
    #[serde(rename = "Threshold")]
    pub threshold: f64,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "Timestamp")]
    pub timestamp: String,
    #[serde(rename = "Token")]
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct AlertPayloadWrapper {
    pub json: AlertPayloadInner,
}

pub struct ServerMetricsMonitor {
    sys: Mutex<System>,
    client: Client,
}

impl ServerMetricsMonitor {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        Self {
            sys: Mutex::new(sys),
            client: Client::new(),
        }
    }

    pub fn get_server_metrics(&self) -> ServerMetric {
        let timestamp = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Nanos, true);

        let (
            cpu,
            cpu_model,
            cpu_cores,
            cpu_physical,
            cpu_speed,
            os_name,
            distro,
            kernel,
            arch,
            mem_used,
            mem_used_gb,
            mem_total_gb,
            uptime,
        ) = {
            let mut sys = self.sys.lock().unwrap();
            sys.refresh_cpu_usage();
            sys.refresh_memory();

            let c = sys.global_cpu_usage() as f64;
            let cpus = sys.cpus();
            let cpu_model = if !cpus.is_empty() {
                format!("{} {}", cpus[0].vendor_id(), cpus[0].brand())
            } else {
                "".to_string()
            };
            let cores = cpus.len() as i32;
            let phys_cores = sys.physical_core_count().unwrap_or(cores as usize) as i32;
            let speed = if !cpus.is_empty() {
                cpus[0].frequency() as f64
            } else {
                0.0
            };

            let os = System::name().unwrap_or_else(|| "linux".to_string());
            let distro = System::long_os_version().unwrap_or_else(|| os.clone());
            let kernel = System::kernel_version().unwrap_or_default();
            let arch = System::cpu_arch();

            let total_gb = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
            let avail_gb = sys.available_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
            let used_gb = (total_gb - avail_gb).max(0.0);
            let used_percent = if total_gb > 0.0 {
                (used_gb / total_gb) * 100.0
            } else {
                0.0
            };
            let uptime = System::uptime();

            (
                c,
                cpu_model,
                cores,
                phys_cores,
                speed,
                os,
                distro,
                kernel,
                arch,
                used_percent,
                used_gb,
                total_gb,
                uptime,
            )
        };

        let disks = Disks::new_with_refreshed_list();
        let mut total_disk_bytes = 0u64;
        let mut used_disk_bytes = 0u64;
        for disk in &disks {
            let total = disk.total_space();
            let avail = disk.available_space();
            total_disk_bytes += total;
            used_disk_bytes += total.saturating_sub(avail);
        }
        let total_disk_gb = total_disk_bytes as f64 / 1024.0 / 1024.0 / 1024.0;
        let used_disk_gb = used_disk_bytes as f64 / 1024.0 / 1024.0 / 1024.0;
        let disk_used_percent = if total_disk_gb > 0.0 {
            (used_disk_gb / total_disk_gb) * 100.0
        } else {
            0.0
        };

        let networks = Networks::new_with_refreshed_list();
        let mut net_in_bytes = 0u64;
        let mut net_out_bytes = 0u64;
        for (_interface, data) in &networks {
            net_in_bytes += data.received();
            net_out_bytes += data.transmitted();
        }
        let network_in_mb = net_in_bytes as f64 / 1024.0 / 1024.0;
        let network_out_mb = net_out_bytes as f64 / 1024.0 / 1024.0;

        ServerMetric {
            id: None,
            timestamp,
            cpu,
            cpu_model,
            cpu_cores,
            cpu_physical_cores: cpu_physical,
            cpu_speed,
            os: os_name,
            distro,
            kernel,
            arch,
            mem_used,
            mem_used_gb,
            mem_total: mem_total_gb,
            uptime,
            disk_used: disk_used_percent,
            total_disk: total_disk_gb,
            network_in: network_in_mb,
            network_out: network_out_mb,
        }
    }

    pub async fn check_thresholds(
        &self,
        m: &ServerMetric,
        cpu_threshold: f64,
        mem_threshold: f64,
        callback_url: &str,
        metrics_token: &str,
        server_type: &str,
    ) {
        if cpu_threshold > 0.0 && m.cpu > cpu_threshold {
            let alert = AlertPayloadWrapper {
                json: AlertPayloadInner {
                    server_type: server_type.to_string(),
                    alert_type: "CPU".to_string(),
                    value: m.cpu,
                    threshold: cpu_threshold,
                    message: format!(
                        "CPU usage ({:.2}%) exceeded threshold ({:.2}%)",
                        m.cpu, cpu_threshold
                    ),
                    timestamp: m.timestamp.clone(),
                    token: metrics_token.to_string(),
                },
            };
            let _ = self.send_alert(callback_url, &alert).await;
        }

        if mem_threshold > 0.0 && m.mem_used > mem_threshold {
            let alert = AlertPayloadWrapper {
                json: AlertPayloadInner {
                    server_type: server_type.to_string(),
                    alert_type: "Memory".to_string(),
                    value: m.mem_used,
                    threshold: mem_threshold,
                    message: format!(
                        "Memory usage ({:.2}%) exceeded threshold ({:.2}%)",
                        m.mem_used, mem_threshold
                    ),
                    timestamp: m.timestamp.clone(),
                    token: metrics_token.to_string(),
                },
            };
            let _ = self.send_alert(callback_url, &alert).await;
        }
    }

    async fn send_alert(
        &self,
        callback_url: &str,
        payload: &AlertPayloadWrapper,
    ) -> Result<(), String> {
        if callback_url.is_empty() {
            return Err("callback URL is empty".to_string());
        }

        match self.client.post(callback_url).json(payload).send().await {
            Ok(resp) if resp.status().is_success() => {
                info!("Sent alert notification to callback URL {}", callback_url);
                Ok(())
            }
            Ok(resp) => {
                let status = resp.status();
                error!("Alert callback returned status {}", status);
                Err(format!("alert callback returned status {}", status))
            }
            Err(err) => {
                error!("Failed to post alert callback: {:?}", err);
                Err(err.to_string())
            }
        }
    }
}
