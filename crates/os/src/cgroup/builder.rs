use super::{
    cpu::CpuLimit, cpuset::CpusetLimit, io::IoLimit, manager::Cgroup, memory::MemoryLimit,
    pids::PidsLimit,
};
use crate::exec::CommandExecutor;

pub struct CgroupBuilder {
    pub(crate) name: String,
    pub(crate) base_path: String,
    pub(crate) executor: CommandExecutor,
    pub(crate) memory_limit: Option<MemoryLimit>,
    pub(crate) memory_high: Option<MemoryLimit>,
    pub(crate) memory_low: Option<MemoryLimit>,
    pub(crate) memory_swap: Option<MemoryLimit>,
    pub(crate) cpu_limit: Option<CpuLimit>,
    pub(crate) cpu_weight: Option<u32>,
    pub(crate) cpuset: Option<CpusetLimit>,
    pub(crate) pids_limit: Option<PidsLimit>,
    pub(crate) io_limit: Option<IoLimit>,
}

impl CgroupBuilder {
    pub fn new(name: impl Into<String>, executor: CommandExecutor) -> Self {
        Self {
            name: name.into(),
            base_path: "/sys/fs/cgroup".into(),
            executor,
            memory_limit: None,
            memory_high: None,
            memory_low: None,
            memory_swap: None,
            cpu_limit: None,
            cpu_weight: None,
            cpuset: None,
            pids_limit: None,
            io_limit: None,
        }
    }

    pub fn with_base_path(mut self, path: impl Into<String>) -> Self {
        self.base_path = path.into();
        self
    }

    pub fn memory(mut self, limit: MemoryLimit) -> Self {
        self.memory_limit = Some(limit);
        self
    }

    pub fn memory_high(mut self, limit: MemoryLimit) -> Self {
        self.memory_high = Some(limit);
        self
    }

    pub fn memory_low(mut self, limit: MemoryLimit) -> Self {
        self.memory_low = Some(limit);
        self
    }

    pub fn memory_swap(mut self, limit: MemoryLimit) -> Self {
        self.memory_swap = Some(limit);
        self
    }

    pub fn cpu(mut self, limit: CpuLimit) -> Self {
        self.cpu_limit = Some(limit);
        self
    }

    pub fn cpu_weight(mut self, weight: u32) -> Self {
        self.cpu_weight = Some(weight);
        self
    }

    pub fn cpuset(mut self, limit: CpusetLimit) -> Self {
        self.cpuset = Some(limit);
        self
    }

    pub fn pids(mut self, limit: PidsLimit) -> Self {
        self.pids_limit = Some(limit);
        self
    }

    pub fn io(mut self, limit: IoLimit) -> Self {
        self.io_limit = Some(limit);
        self
    }

    pub fn build(self) -> Cgroup {
        Cgroup {
            name: self.name,
            base_path: self.base_path,
            executor: self.executor,
            memory_limit: self.memory_limit,
            memory_high: self.memory_high,
            memory_low: self.memory_low,
            memory_swap: self.memory_swap,
            cpu_limit: self.cpu_limit,
            cpu_weight: self.cpu_weight,
            cpuset: self.cpuset,
            pids_limit: self.pids_limit,
            io_limit: self.io_limit,
        }
    }
}
