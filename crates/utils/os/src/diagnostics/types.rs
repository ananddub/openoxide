#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagnosticScope {
    System,
    Disk,
    Memory,
    Docker,
    Firewall,
}

impl DiagnosticScope {
    pub(crate) const ALL: [Self; 5] = [
        Self::System,
        Self::Disk,
        Self::Memory,
        Self::Docker,
        Self::Firewall,
    ];
}
