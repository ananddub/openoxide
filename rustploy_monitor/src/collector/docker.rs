use serde::Deserialize;
use tokio::process::Command;
use tracing::{error, warn};

use crate::store::ContainerMetricRow;

/// One line of `docker stats --format json` output.
#[derive(Debug, Deserialize)]
struct DockerStatsLine {
    #[serde(rename = "BlockIO")]
    block_io: String,
    #[serde(rename = "CPUPerc")]
    cpu_perc: String,
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "MemPerc")]
    mem_perc: String,
    #[serde(rename = "MemUsage")]
    mem_usage: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "NetIO")]
    net_io: String,
}

const STATS_FORMAT: &str = r#"{"BlockIO":"{{.BlockIO}}","CPUPerc":"{{.CPUPerc}}","ID":"{{.ID}}","MemPerc":"{{.MemPerc}}","MemUsage":"{{.MemUsage}}","Name":"{{.Name}}","NetIO":"{{.NetIO}}"}"#;

/// Collects a one-shot stats snapshot for every running container.
///
/// Uses the docker CLI rather than the API socket: the agent already ships with
/// `docker-cli` and this avoids hand-rolling the API's two-sample CPU delta.
/// The tradeoff is a process spawn per cycle, which is fine at a 60s cadence.
pub async fn collect_container_metrics() -> Vec<ContainerMetricRow> {
    let output = match Command::new("docker")
        .args(["stats", "--no-stream", "--format", STATS_FORMAT])
        .output()
        .await
    {
        Ok(out) if out.status.success() => out.stdout,
        Ok(out) => {
            error!(
                stderr = %String::from_utf8_lossy(&out.stderr).trim(),
                "docker stats exited non-zero"
            );
            return Vec::new();
        }
        Err(err) => {
            error!(error = %err, "could not run docker stats — is the socket mounted?");
            return Vec::new();
        }
    };

    parse_stats_output(&String::from_utf8_lossy(&output))
}

/// Splits raw `docker stats` output into rows. Kept separate from the process
/// spawn so it can be tested against fixture text.
fn parse_stats_output(raw: &str) -> Vec<ContainerMetricRow> {
    let timestamp = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

    raw.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter_map(|line| match serde_json::from_str::<DockerStatsLine>(line) {
            Ok(stats) => Some(stats),
            Err(err) => {
                warn!(error = %err, line, "skipping unparseable docker stats line");
                None
            }
        })
        .map(|stats| {
            let (mem_used_mb, mem_total_mb) = split_pair(&stats.mem_usage);
            let (net_in_mb, net_out_mb) = split_pair(&stats.net_io);
            let (block_read_mb, block_write_mb) = split_pair(&stats.block_io);

            ContainerMetricRow {
                id: None,
                timestamp: timestamp.clone(),
                container_id: stats.id,
                name: stats.name,
                cpu_perc: parse_percent(&stats.cpu_perc),
                mem_perc: parse_percent(&stats.mem_perc),
                mem_used_mb,
                mem_total_mb,
                net_in_mb,
                net_out_mb,
                block_read_mb,
                block_write_mb,
            }
        })
        .collect()
}

/// Parses `"12.34%"` into `12.34`.
fn parse_percent(value: &str) -> f64 {
    value.trim().trim_end_matches('%').parse().unwrap_or(0.0)
}

/// Splits docker's `"<a> / <b>"` pairs (MemUsage, NetIO, BlockIO) into MB.
fn split_pair(value: &str) -> (f64, f64) {
    match value.split_once('/') {
        Some((left, right)) => (parse_size_mb(left), parse_size_mb(right)),
        None => (0.0, 0.0),
    }
}

/// Parses a docker size string such as `"1.5GiB"` or `"324.2 MB"` into MB.
///
/// Docker mixes SI and binary units in the same output, and whether there's a
/// space before the unit varies by field, so both forms are handled.
fn parse_size_mb(value: &str) -> f64 {
    let value = value.trim();

    let split_at = value
        .find(|c: char| !c.is_ascii_digit() && c != '.' && c != '-')
        .unwrap_or(value.len());
    let (number, unit) = value.split_at(split_at);

    let number: f64 = match number.trim().parse() {
        Ok(n) => n,
        Err(_) => return 0.0,
    };

    match unit.trim().to_ascii_lowercase().as_str() {
        "b" | "" => number / 1_048_576.0,
        "kb" | "kib" => number / 1024.0,
        "mb" | "mib" => number,
        "gb" | "gib" => number * 1024.0,
        "tb" | "tib" => number * 1_048_576.0,
        other => {
            warn!(unit = other, "unrecognised docker size unit, treating as MB");
            number
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_percentages() {
        assert_eq!(parse_percent("12.34%"), 12.34);
        assert_eq!(parse_percent("0.00%"), 0.0);
        assert_eq!(parse_percent("--"), 0.0);
    }

    #[test]
    fn parses_sizes_across_units() {
        assert_eq!(parse_size_mb("512MiB"), 512.0);
        assert_eq!(parse_size_mb("1GiB"), 1024.0);
        assert_eq!(parse_size_mb("1024kB"), 1.0);
        assert_eq!(parse_size_mb("324.2 MB"), 324.2);
        assert!((parse_size_mb("1048576B") - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn unparseable_sizes_become_zero() {
        assert_eq!(parse_size_mb(""), 0.0);
        assert_eq!(parse_size_mb("N/A"), 0.0);
    }

    #[test]
    fn splits_docker_pairs() {
        let (used, total) = split_pair("512MiB / 1GiB");
        assert_eq!(used, 512.0);
        assert_eq!(total, 1024.0);
    }

    #[test]
    fn pair_without_separator_is_zero() {
        assert_eq!(split_pair("N/A"), (0.0, 0.0));
    }

    #[test]
    fn parses_a_full_stats_line() {
        let raw = r#"{"BlockIO":"1.2MB / 3.4MB","CPUPerc":"5.25%","ID":"abc123","MemPerc":"10.50%","MemUsage":"512MiB / 2GiB","Name":"web","NetIO":"100kB / 200kB"}"#;

        let rows = parse_stats_output(raw);
        assert_eq!(rows.len(), 1);

        let row = &rows[0];
        assert_eq!(row.container_id, "abc123");
        assert_eq!(row.name, "web");
        assert_eq!(row.cpu_perc, 5.25);
        assert_eq!(row.mem_perc, 10.50);
        assert_eq!(row.mem_used_mb, 512.0);
        assert_eq!(row.mem_total_mb, 2048.0);
    }

    #[test]
    fn skips_malformed_lines_but_keeps_good_ones() {
        let raw = concat!(
            "not json at all\n",
            r#"{"BlockIO":"0B / 0B","CPUPerc":"1.00%","ID":"ok","MemPerc":"2.00%","MemUsage":"1MiB / 2MiB","Name":"fine","NetIO":"0B / 0B"}"#,
            "\n\n"
        );

        let rows = parse_stats_output(raw);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].container_id, "ok");
    }

    #[test]
    fn empty_output_yields_no_rows() {
        assert!(parse_stats_output("").is_empty());
    }
}
