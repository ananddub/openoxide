#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CgroupStats {
    pub memory_current: u64,
    pub cpu_usec: u64,
}
