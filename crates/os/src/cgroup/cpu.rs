#[derive(Debug, Clone, PartialEq)]
pub enum CpuLimit {
    Cores(f32),
    Millicores(u32),
    Max,
}

impl CpuLimit {
    pub fn to_cgroup_value(&self) -> String {
        match self {
            Self::Cores(c) => {
                let period = 100000u32;
                let quota = (c * (period as f32)) as u32;
                format!("{} {}", quota, period)
            }
            Self::Millicores(m) => {
                let period = 100000u32;
                let quota = m * 100;
                format!("{} {}", quota, period)
            }
            Self::Max => "max".to_string(),
        }
    }
}
