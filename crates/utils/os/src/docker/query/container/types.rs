use std::fmt;

// ── RestartPolicy ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestartPolicy {
    No,
    Always,
    /// Restart on failure up to `n` times.
    OnFailure(u32),
    UnlessStopped,
}

impl fmt::Display for RestartPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::No => write!(f, "no"),
            Self::Always => write!(f, "always"),
            Self::OnFailure(n) => write!(f, "on-failure:{n}"),
            Self::UnlessStopped => write!(f, "unless-stopped"),
        }
    }
}

// ── Protocol ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Protocol {
    #[default]
    Tcp,
    Udp,
    Sctp,
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Tcp => "tcp",
            Self::Udp => "udp",
            Self::Sctp => "sctp",
        })
    }
}

// ── Internal helpers ──────────────────────────────────────────────────────────

pub(crate) struct PortBinding {
    pub host: u16,
    pub container: u16,
    pub proto: Protocol,
}
pub(crate) struct Mount {
    pub source: String,
    pub target: String,
    pub read_only: bool,
    pub kind: MountKind,
}
pub(crate) enum MountKind {
    Volume,
    Bind,
    Tmpfs,
}
