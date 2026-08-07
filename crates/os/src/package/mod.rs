use crate::exec::CommandExecutor;
use crate::exec::script::IntoCommand;

pub mod clean;
pub mod install;
pub mod installed;
pub mod list;
pub mod package_builder;
pub mod remove;
pub mod search;
pub mod update;
pub mod upgrade;

pub use clean::PackageCleanBuilder;
pub use install::PackageInstallBuilder;
pub use installed::PackageCheckInstalledBuilder;
pub use list::PackageListInstalledBuilder;
pub use package_builder::PackageBuilder;
pub use remove::PackageRemoveBuilder;
pub use search::PackageSearchBuilder;
pub use update::PackageUpdateIndexBuilder;
pub use upgrade::PackageUpgradeAllBuilder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageManager {
    Apt,
    Dnf,
    Yum,
    Apk,
    Pacman,
    Zypper,
    Xbps,
    Emerge,
    Nix,
    Brew,
}

pub(crate) async fn detect_manager(executor: &CommandExecutor) -> PackageManager {
    let os = crate::OsCli::new(executor);
    if os.has_command("apt-get").run().await.is_ok() {
        PackageManager::Apt
    } else if os.has_command("dnf").run().await.is_ok() {
        PackageManager::Dnf
    } else if os.has_command("yum").run().await.is_ok() {
        PackageManager::Yum
    } else if os.has_command("apk").run().await.is_ok() {
        PackageManager::Apk
    } else if os.has_command("pacman").run().await.is_ok() {
        PackageManager::Pacman
    } else if os.has_command("zypper").run().await.is_ok() {
        PackageManager::Zypper
    } else if os.has_command("xbps-install").run().await.is_ok() {
        PackageManager::Xbps
    } else if os.has_command("emerge").run().await.is_ok() {
        PackageManager::Emerge
    } else if os.has_command("nix-env").run().await.is_ok() {
        PackageManager::Nix
    } else if os.has_command("brew").run().await.is_ok() {
        PackageManager::Brew
    } else {
        PackageManager::Apt // Default fallback
    }
}

pub struct PackageCli<'a> {
    pub(crate) executor: &'a CommandExecutor,
}

impl<'a> PackageCli<'a> {
    pub fn list_installed(&self) -> PackageListInstalledBuilder<'a> {
        PackageListInstalledBuilder::new(self.executor)
    }
    pub fn search(&self, query: impl IntoCommand) -> PackageSearchBuilder<'a> {
        PackageSearchBuilder::new(self.executor, query)
    }
    pub fn update_index(&self) -> PackageUpdateIndexBuilder<'a> {
        PackageUpdateIndexBuilder::new(self.executor)
    }
    pub fn upgrade_all(&self) -> PackageUpgradeAllBuilder<'a> {
        PackageUpgradeAllBuilder::new(self.executor)
    }
    pub fn clean(&self) -> PackageCleanBuilder<'a> {
        PackageCleanBuilder::new(self.executor)
    }
    pub fn package(&self, name: impl IntoCommand) -> PackageBuilder<'a> {
        PackageBuilder::new(self.executor, name)
    }
}
