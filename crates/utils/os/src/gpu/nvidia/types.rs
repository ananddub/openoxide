#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerRuntime {
    Docker,
    Containerd,
    Crio,
}
impl ContainerRuntime {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Docker => "docker",
            Self::Containerd => "containerd",
            Self::Crio => "crio",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NvidiaQueryFormat {
    Csv,
}
impl NvidiaQueryFormat {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Csv => "csv",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NvidiaQueryField {
    Name,
    Uuid,
    DriverVersion,
    Temperature,
    GpuUtilization,
    MemoryUtilization,
    MemoryTotal,
    MemoryUsed,
    MemoryFree,
    PowerDraw,
    PowerLimit,
    FanSpeed,
    PerformanceState,
    ComputeMode,
    DisplayActive,
}
impl NvidiaQueryField {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Name => "name",
            Self::Uuid => "uuid",
            Self::DriverVersion => "driver_version",
            Self::Temperature => "temperature.gpu",
            Self::GpuUtilization => "utilization.gpu",
            Self::MemoryUtilization => "utilization.memory",
            Self::MemoryTotal => "memory.total",
            Self::MemoryUsed => "memory.used",
            Self::MemoryFree => "memory.free",
            Self::PowerDraw => "power.draw",
            Self::PowerLimit => "power.limit",
            Self::FanSpeed => "fan.speed",
            Self::PerformanceState => "pstate",
            Self::ComputeMode => "compute_mode",
            Self::DisplayActive => "display_active",
        }
    }
}
