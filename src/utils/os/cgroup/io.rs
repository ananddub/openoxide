#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IoLimit {
    Max(String),
}

impl IoLimit {
    pub fn to_cgroup_value(&self) -> &str {
        match self {
            Self::Max(s) => s,
        }
    }
}
