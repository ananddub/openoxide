#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FreezeState {
    Frozen,
    Thawed,
}

impl FreezeState {
    pub fn to_cgroup_value(&self) -> &'static str {
        match self {
            Self::Frozen => "1",
            Self::Thawed => "0",
        }
    }
}
