use std::{fmt, path::Path};

#[derive(Debug, Clone, PartialEq, Eq)]
enum MountKind {
    Volume,
    Bind,
    Tmpfs,
}

/// Strongly-typed Docker mount specification.
///
/// ```
/// Mount::volume("pg-data", "/var/lib/postgresql/data")
/// Mount::bind("/etc/nginx.conf", "/etc/nginx/nginx.conf")
/// Mount::bind_ro("/etc/ssl", "/etc/ssl")
/// Mount::tmpfs("/tmp")
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mount {
    kind: MountKind,
    source: String,
    target: String,
    read_only: bool,
}

impl Mount {
    /// Named volume mount.
    pub fn volume(name: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            kind: MountKind::Volume,
            source: name.into(),
            target: target.into(),
            read_only: false,
        }
    }

    /// Bind-mount a host path (read-write).
    pub fn bind(host: impl AsRef<Path>, target: impl Into<String>) -> Self {
        Self {
            kind: MountKind::Bind,
            source: host.as_ref().to_string_lossy().into_owned(),
            target: target.into(),
            read_only: false,
        }
    }

    /// Bind-mount a host path (read-only).
    pub fn bind_ro(host: impl AsRef<Path>, target: impl Into<String>) -> Self {
        let mut m = Self::bind(host, target);
        m.read_only = true;
        m
    }

    /// In-memory tmpfs mount.
    pub fn tmpfs(target: impl Into<String>) -> Self {
        Self {
            kind: MountKind::Tmpfs,
            source: String::new(),
            target: target.into(),
            read_only: false,
        }
    }

    /// Mark an existing mount read-only.
    pub fn read_only(mut self) -> Self {
        self.read_only = true;
        self
    }

    /// Render as the Docker `--volume` short-syntax string (`source:target[:ro]`).
    /// Volume and bind mounts use `--volume`; tmpfs uses `--mount type=tmpfs`.
    pub fn to_volume_flag(&self) -> String {
        match self.kind {
            MountKind::Volume | MountKind::Bind => {
                if self.read_only {
                    format!("{}:{}:ro", self.source, self.target)
                } else {
                    format!("{}:{}", self.source, self.target)
                }
            }
            MountKind::Tmpfs => format!("type=tmpfs,target={}", self.target),
        }
    }

    /// CLI flag name — `--volume` or `--mount` for tmpfs.
    pub fn flag_name(&self) -> &'static str {
        match self.kind {
            MountKind::Tmpfs => "--mount",
            _ => "--volume",
        }
    }
}

impl fmt::Display for Mount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_volume_flag())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn volume_rw() {
        let m = Mount::volume("pg", "/data");
        assert_eq!(m.to_string(), "pg:/data");
        assert_eq!(m.flag_name(), "--volume");
    }
    #[test]
    fn volume_ro() {
        let m = Mount::volume("pg", "/data").read_only();
        assert_eq!(m.to_string(), "pg:/data:ro");
    }
    #[test]
    fn bind_rw() {
        let m = Mount::bind("/host/path", "/container");
        assert_eq!(m.to_string(), "/host/path:/container");
    }
    #[test]
    fn bind_ro() {
        let m = Mount::bind_ro("/etc/ssl", "/etc/ssl");
        assert_eq!(m.to_string(), "/etc/ssl:/etc/ssl:ro");
    }
    #[test]
    fn tmpfs_flag() {
        let m = Mount::tmpfs("/tmp");
        assert_eq!(m.flag_name(), "--mount");
        assert!(m.to_string().contains("tmpfs"));
    }
}
