use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub use super::types::CgroupSample;

pub struct CgroupReader {
    root: PathBuf,
}

impl CgroupReader {
    pub fn discover() -> Option<Self> {
        let cgroup = fs::read_to_string("/proc/self/mounts").ok()?;
        let root = cgroup.lines().find_map(|line| {
            let mut parts = line.split_whitespace();
            let source = parts.next()?;
            let mount = parts.next()?;
            if source == "cgroup2" {
                Some(mount.to_string())
            } else {
                None
            }
        })?;

        Some(Self {
            root: PathBuf::from(root),
        })
    }

    fn cgroup_dir(&self, container_id: &str) -> Option<PathBuf> {
        let systemd = self
            .root
            .join("system.slice")
            .join(format!("docker-{container_id}.scope"));
        if systemd.join("cpu.stat").exists() {
            return Some(systemd);
        }

        let cgroupfs = self.root.join("docker").join(container_id);
        if cgroupfs.join("cpu.stat").exists() {
            return Some(cgroupfs);
        }

        if let Ok(entries) = fs::read_dir(&self.root) {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                if (name.contains(container_id) || name.starts_with(container_id))
                    && path.join("cpu.stat").is_file()
                {
                    return Some(path);
                }
            }
        }

        None
    }

    pub fn read(&self, container_id: &str) -> Option<CgroupSample> {
        let dir = self.cgroup_dir(container_id)?;

        let cpu = fs::read_to_string(dir.join("cpu.stat")).ok()?;
        let cpu_usage_usec = parse_field(&cpu, "usage_usec").unwrap_or_else(|| {
            parse_field(&cpu, "user_usec").unwrap_or(0)
                + parse_field(&cpu, "system_usec").unwrap_or(0)
        });

        let memory_usage: u64 = fs::read_to_string(dir.join("memory.current"))
            .ok()?
            .trim()
            .parse()
            .ok()?;

        let memory_stat = fs::read_to_string(dir.join("memory.stat")).ok()?;
        let memory_cache = parse_field(&memory_stat, "inactive_file")
            .or_else(|| parse_field(&memory_stat, "total_inactive_file"))
            .or_else(|| parse_field(&memory_stat, "cache"))
            .unwrap_or(0);

        let memory_limit: u64 = fs::read_to_string(dir.join("memory.max"))
            .ok()?
            .trim()
            .parse()
            .unwrap_or(0);

        let (io_read, io_write) = fs::read_to_string(dir.join("io.stat"))
            .ok()
            .map(|stat| {
                let mut read = 0u64;
                let mut write = 0u64;
                for line in stat.lines() {
                    for field in line.split_whitespace() {
                        if let Some(v) = field.strip_prefix("rbytes=") {
                            read += v.parse().unwrap_or(0);
                        } else if let Some(v) = field.strip_prefix("wbytes=") {
                            write += v.parse().unwrap_or(0);
                        }
                    }
                }
                (read, write)
            })
            .unwrap_or((0, 0));

        Some(CgroupSample {
            cpu_usage_usec,
            memory_usage,
            memory_cache,
            memory_limit,
            io_read_bytes: io_read,
            io_write_bytes: io_write,
        })
    }

    pub fn read_many(
        &self,
        containers: &[String],
    ) -> HashMap<String, CgroupSample> {
        containers
            .iter()
            .filter_map(|id| self.read(id).map(|sample| (id.clone(), sample)))
            .collect()
    }
}

fn parse_field(content: &str, key: &str) -> Option<u64> {
    content
        .lines()
        .find(|l| l.starts_with(&format!("{key} ")))
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|v| v.parse().ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_cpu_stat() {
        let content = "usage_usec 1353656\nuser_usec 771626\nsystem_usec 582029\n";
        assert_eq!(parse_field(content, "usage_usec"), Some(1353656));
        assert_eq!(parse_field(content, "system_usec"), Some(582029));
    }

    #[test]
    fn parse_missing_field_is_none() {
        assert_eq!(parse_field("usage_usec 100\n", "missing"), None);
        assert_eq!(parse_field("", "usage_usec"), None);
    }

    #[test]
    fn reads_io_stat_accumulating_per_device() {
        let content = "259:4 rbytes=2908160 wbytes=0 rios=41 wios=0\n8:0 rbytes=100 wbytes=500\n";
        let mut read = 0u64;
        let mut write = 0u64;
        for line in content.lines() {
            for field in line.split_whitespace() {
                if let Some(v) = field.strip_prefix("rbytes=") {
                    read += v.parse().unwrap_or(0);
                } else if let Some(v) = field.strip_prefix("wbytes=") {
                    write += v.parse().unwrap_or(0);
                }
            }
        }
        assert_eq!(read, 2908160 + 100);
        assert_eq!(write, 0 + 500);
    }
}
