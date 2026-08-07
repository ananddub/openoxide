#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CpusetLimit {
    Cpus(String),
}

impl CpusetLimit {
    pub fn to_cgroup_value(&self) -> &str {
        match self {
            Self::Cpus(c) => c,
        }
    }
}
