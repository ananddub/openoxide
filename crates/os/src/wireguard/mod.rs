#[cfg(test)]
mod architecture_tests;
mod cli;
mod config;
mod interface;
mod key;

use crate::exec::CommandExecutor;

pub use cli::{
    AllowedIpAction, AllowedIpChange, FirewallMark, WireGuardConfigAction,
    WireGuardConfigFileBuilder, WireGuardPeerUpdate, WireGuardQuickAction, WireGuardQuickBuilder,
    WireGuardSetBuilder, WireGuardShowConfigBuilder, WireGuardShowField, WireGuardShowTarget,
};
pub use config::{
    WireGuardConfig, WireGuardConfigBuilder, WireGuardConfigError, WireGuardHook, WireGuardPeer,
    WireGuardPeerBuilder, WireGuardRoutingTable,
};
pub use interface::{WireGuardHandshake, WireGuardInterfaceBuilder};
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

    pub fn show(&self, target: WireGuardShowTarget) -> cli::WireGuardShowBuilder<'a> {
        cli::WireGuardShowBuilder::new(self.executor, target)
    }

    pub fn show_config(&self, interface: impl Into<String>) -> WireGuardShowConfigBuilder<'a> {
        WireGuardShowConfigBuilder::new(self.executor, interface)
    }

    pub fn set(&self, interface: impl Into<String>) -> WireGuardSetBuilder<'a> {
        WireGuardSetBuilder::new(self.executor, interface)
    }

    pub fn apply_config(
        &self,
        action: WireGuardConfigAction,
        interface: impl Into<String>,
        path: impl Into<std::path::PathBuf>,
    ) -> WireGuardConfigFileBuilder<'a> {
        WireGuardConfigFileBuilder::new(self.executor, action, interface, path)
    }

    pub fn quick(
        &self,
        action: WireGuardQuickAction,
        target: impl Into<String>,
    ) -> WireGuardQuickBuilder<'a> {
        WireGuardQuickBuilder::new(self.executor, action, target)
    }
}
