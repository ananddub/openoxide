use crate::db::ContainerMetricRow;
use serde::Deserialize;
use std::process::Command;
use tracing::error;

#[derive(Debug, Deserialize)]
pub struct DockerStatsJson {
    #[serde(rename = "BlockIO")]
    pub block_io: String,
    #[serde(rename = "CPUPerc")]
    pub cpu_perc: String,
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "MemPerc")]
    pub mem_perc: String,
    #[serde(rename = "MemUsage")]
    pub mem_usage: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "NetIO")]
    pub net_io: String,
}

pub fn collect_docker_container_metrics() -> Vec<ContainerMetricRow> {
    let output = match Command::new("docker")
        .args([
            "stats",
            "--no-stream",
            "--format",
            r#"{"BlockIO":"{{.BlockIO}}","CPUPerc":"{{.CPUPerc}}","ID":"{{.ID}}","MemPerc":"{{.MemPerc}}","MemUsage":"{{.MemUsage}}","Name":"{{.Name}}","NetIO":"{{.NetIO}}"}"#,
        ])
        .output()
    {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).to_string(),
        Ok(out) => {
            error!("docker stats command failed: {}", String::from_utf8_lossy(&out.stderr));
            return vec![];
        }
        Err(err) => {
            error!("Failed to execute docker stats: {:?}", err);
            return vec![];
        }
    };

    let mut metrics = Vec::new();
    let timestamp = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Nanos, true);

    for line in output.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Ok(stats) = serde_json::from_str::<DockerStatsJson>(line) {
            let cpu_perc = stats
                .cpu_perc
                .trim_end_matches('%')
                .parse::<f64>()
                .unwrap_or(0.0);
            let mem_perc = stats
                .mem_perc
                .trim_end_matches('%')
                .parse::<f64>()
                .unwrap_or(0.0);

            // Process Memory Usage "used / total"
            let mem_parts: Vec<&str> = stats.mem_usage.split(" / ").collect();
            let (mem_used_mb, mem_total_mb) = if mem_parts.len() == 2 {
                (parse_memory_mb(mem_parts[0]), parse_memory_mb(mem_parts[1]))
            } else {
                (0.0, 0.0)
            };

            // Process Net I/O "in / out"
            let net_parts: Vec<&str> = stats.net_io.split(" / ").collect();
            let (net_in_mb, net_out_mb) = if net_parts.len() == 2 {
                (parse_memory_mb(net_parts[0]), parse_memory_mb(net_parts[1]))
            } else {
                (0.0, 0.0)
            };

            // Process Block I/O "read / write"
            let block_parts: Vec<&str> = stats.block_io.split(" / ").collect();
            let (block_read_mb, block_write_mb) = if block_parts.len() == 2 {
                (
                    parse_memory_mb(block_parts[0]),
                    parse_memory_mb(block_parts[1]),
                )
            } else {
                (0.0, 0.0)
            };

            metrics.push(ContainerMetricRow {
                id: None,
                timestamp: timestamp.clone(),
                container_id: stats.id,
                name: stats.name,
                cpu_perc,
                mem_perc,
                mem_used_mb,
                mem_total_mb,
                net_in_mb,
                net_out_mb,
                block_read_mb,
                block_write_mb,
            });
        }
    }

    metrics
}

pub fn tail_container_logs(container_id: &str, tail_lines: usize) -> Vec<String> {
    let output = match Command::new("docker")
        .args(["logs", "--tail", &tail_lines.to_string(), container_id])
        .output()
    {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let stderr = String::from_utf8_lossy(&out.stderr);
            format!("{}{}", stdout, stderr)
        }
        Err(_) => String::new(),
    };

    output.lines().map(|s| s.to_string()).collect()
}

fn parse_memory_mb(val: &str) -> f64 {
    let parts: Vec<&str> = val.split_whitespace().collect();
    if parts.is_empty() {
        return 0.0;
    }

    let num = parts[0]
        .trim_end_matches(|c: char| c.is_alphabetic())
        .parse::<f64>()
        .unwrap_or(0.0);
    let unit = parts[0].trim_start_matches(|c: char| c.is_ascii_digit() || c == '.');

    match unit {
        "B" | "b" => num / 1024.0 / 1024.0,
        "kB" | "KB" | "KiB" => num / 1024.0,
        "MB" | "MiB" => num,
        "GB" | "GiB" => num * 1024.0,
        "TB" | "TiB" => num * 1024.0 * 1024.0,
        _ => num,
    }
}
