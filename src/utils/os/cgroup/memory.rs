#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryLimit {
    B(u64),
    KB(u64),
    MB(u64),
    GB(u64),
    Max,
}

impl MemoryLimit {
    pub fn to_cgroup_value(&self) -> String {
        match self {
            Self::B(b) => b.to_string(),
            Self::KB(k) => format!("{}K", k),
            Self::MB(m) => format!("{}M", m),
            Self::GB(g) => format!("{}G", g),
            Self::Max => "max".to_string(),
        }
    }
}
