mod config;
#[cfg(test)]
mod integration_tests;
mod interface;
mod key;

use crate::utils::exec::CommandExecutor;

pub use config::{WireGuardConfig, WireGuardConfigError, WireGuardPeer};
pub use interface::WireGuardInterfaceBuilder;
pub use key::WireGuardKeyBuilder;

pub struct WireGuardCli<'a> {
    pub(crate) executor: &'a CommandExecutor,
}

impl<'a> WireGuardCli<'a> {
    pub fn key(&self) -> WireGuardKeyBuilder<'a> {
        WireGuardKeyBuilder::new(self.executor)
    }

    pub fn interface(&self, name: impl Into<String>) -> WireGuardInterfaceBuilder<'a> {
        WireGuardInterfaceBuilder::new(self.executor, name.into())
    }
}
