pub mod container;
pub mod grpc;
pub mod host;
pub mod retention;

/// Strongly-typed enum representing background agent tasks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaskKind {
    GrpcServer,
    HostCollector,
    ContainerCollector,
    RetentionSweeper,
}

impl TaskKind {
    pub fn name(&self) -> &'static str {
        match self {
            Self::GrpcServer => "grpc_server",
            Self::HostCollector => "host_collector",
            Self::ContainerCollector => "container_collector",
            Self::RetentionSweeper => "retention_sweeper",
        }
    }
}
