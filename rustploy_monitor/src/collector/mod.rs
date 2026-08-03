pub mod cgroup;
pub mod system;

pub use cgroup::CgroupCollector;
pub use system::SystemCollector;

pub const BYTES_PER_MB: f64 = 1_048_576.0;
pub const BYTES_PER_GB: f64 = 1_073_741_824.0;

#[inline]
pub fn bytes_to_mb(bytes: f64) -> f64 {
    bytes / BYTES_PER_MB
}

#[inline]
pub fn bytes_to_gb(bytes: f64) -> f64 {
    bytes / BYTES_PER_GB
}
