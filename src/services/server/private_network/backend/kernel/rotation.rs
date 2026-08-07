use crate::utils::os::OsCli;

use super::KernelWireGuardBackend;

impl KernelWireGuardBackend<'_> {
    pub(super) async fn restore_pair(
        &self,
        interface: &str,
        local_backup: &mut String,
        remote_backup: &mut String,
    ) -> Result<(), String> {
        let local = OsCli::new(self.local)
            .wireguard()
            .interface(interface)
            .restore_config(local_backup)
            .await;
        let remote = OsCli::new(self.remote)
            .wireguard()
            .interface(interface)
            .restore_config(remote_backup)
            .await;
        match (local, remote) {
            (Ok(_), Ok(_)) => Ok(()),
            (Err(local), Ok(_)) => Err(format!("panel restore failed: {local}")),
            (Ok(_), Err(remote)) => Err(format!("remote restore failed: {remote}")),
            (Err(local), Err(remote)) => Err(format!(
                "panel restore failed: {local}; remote restore failed: {remote}"
            )),
        }
    }

    pub(super) fn rotation_error(
        reason: impl std::fmt::Display,
        rollback: Result<(), String>,
    ) -> sqlx::Error {
        match rollback {
            Ok(()) => sqlx::Error::Protocol(format!(
                "{reason}; previous WireGuard configuration was restored"
            )),
            Err(rollback_error) => sqlx::Error::Protocol(format!(
                "{reason}; WireGuard rollback was incomplete: {rollback_error}"
            )),
        }
    }
}
