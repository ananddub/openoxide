use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContainerStatus {
    Created,
    Restarting,
    Running,
    Removing,
    Paused,
    Exited,
    Dead,
}

impl fmt::Display for ContainerStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Created => "created",
            Self::Restarting => "restarting",
            Self::Running => "running",
            Self::Removing => "removing",
            Self::Paused => "paused",
            Self::Exited => "exited",
            Self::Dead => "dead",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HealthStatus {
    Starting,
    Healthy,
    Unhealthy,
    None,
}

impl fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Starting => "starting",
            Self::Healthy => "healthy",
            Self::Unhealthy => "unhealthy",
            Self::None => "none",
        })
    }
}

/// Typesafe filter for `docker container ls --filter`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContainerFilter {
    Id(String),
    Name(String),
    Status(ContainerStatus),
    Health(HealthStatus),
    /// `label=KEY=VALUE`
    Label(String, String),
    /// `label=KEY` (key-only label presence check)
    LabelKey(String),
    Network(String),
    /// Filter by ancestor image name/id.
    Ancestor(String),
    Volume(String),
    /// Containers created after this container id/name.
    Since(String),
    /// Containers created before this container id/name.
    Before(String),
    /// Containers that exited with the given exit code.
    Exited(i32),
    /// Filter by published ports.
    Publish(String),
    /// Filter by exposed ports.
    Expose(String),
    /// Filter by isolation technology.
    Isolation(String),
    /// Filter containers that are swarm tasks.
    IsTask(bool),
}

impl ContainerFilter {
    pub fn id(v: impl Into<String>) -> Self {
        Self::Id(v.into())
    }
    pub fn name(v: impl Into<String>) -> Self {
        Self::Name(v.into())
    }
    pub fn label(k: impl Into<String>, v: impl Into<String>) -> Self {
        Self::Label(k.into(), v.into())
    }
    pub fn label_key(k: impl Into<String>) -> Self {
        Self::LabelKey(k.into())
    }
    pub fn network(v: impl Into<String>) -> Self {
        Self::Network(v.into())
    }
    pub fn ancestor(v: impl Into<String>) -> Self {
        Self::Ancestor(v.into())
    }
    pub fn volume(v: impl Into<String>) -> Self {
        Self::Volume(v.into())
    }
    pub fn since(v: impl Into<String>) -> Self {
        Self::Since(v.into())
    }
    pub fn before(v: impl Into<String>) -> Self {
        Self::Before(v.into())
    }
    pub fn publish(v: impl Into<String>) -> Self {
        Self::Publish(v.into())
    }
    pub fn expose(v: impl Into<String>) -> Self {
        Self::Expose(v.into())
    }
    pub fn isolation(v: impl Into<String>) -> Self {
        Self::Isolation(v.into())
    }
    pub fn is_task(v: bool) -> Self {
        Self::IsTask(v)
    }
}

impl fmt::Display for ContainerFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Id(v) => write!(f, "id={v}"),
            Self::Name(v) => write!(f, "name={v}"),
            Self::Status(s) => write!(f, "status={s}"),
            Self::Health(h) => write!(f, "health={h}"),
            Self::Label(k, v) => write!(f, "label={k}={v}"),
            Self::LabelKey(k) => write!(f, "label={k}"),
            Self::Network(v) => write!(f, "network={v}"),
            Self::Ancestor(v) => write!(f, "ancestor={v}"),
            Self::Volume(v) => write!(f, "volume={v}"),
            Self::Since(v) => write!(f, "since={v}"),
            Self::Before(v) => write!(f, "before={v}"),
            Self::Exited(c) => write!(f, "exited={c}"),
            Self::Publish(v) => write!(f, "publish={v}"),
            Self::Expose(v) => write!(f, "expose={v}"),
            Self::Isolation(v) => write!(f, "isolation={v}"),
            Self::IsTask(b) => write!(f, "is-task={b}"),
        }
    }
}
