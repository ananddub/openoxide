mod kernel;

pub(super) use kernel::{KernelWireGuardBackend, KernelWireGuardHealth, WireGuardInstallPlan};

pub(super) trait ManagedWireGuardBackend {
    async fn install(&self, plan: &WireGuardInstallPlan<'_>) -> sqlx::Result<String>;
    async fn teardown(&self, interface: &str) -> sqlx::Result<()>;
    async fn health(&self, interface: &str) -> sqlx::Result<KernelWireGuardHealth>;
    async fn rotate(&self, plan: &WireGuardInstallPlan<'_>) -> sqlx::Result<String>;
}
