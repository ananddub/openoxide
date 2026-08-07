#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PidsLimit {
    Count(u32),
    Max,
}

impl PidsLimit {
    pub fn to_cgroup_value(&self) -> String {
        match self {
            Self::Count(c) => c.to_string(),
            Self::Max => "max".to_string(),
        }
    }
}
