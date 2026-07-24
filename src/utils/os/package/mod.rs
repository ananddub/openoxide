use crate::utils::exec::CommandExecutor;
use crate::utils::exec::script::IntoCommand;

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
    if executor
        .run("sh", &["-c", "command -v apt-get"])
        .await
        .map(|o| o.success())
        .unwrap_or(false)
    {
        PackageManager::Apt
    } else if executor
        .run("sh", &["-c", "command -v dnf"])
        .await
        .map(|o| o.success())
        .unwrap_or(false)
    {
        PackageManager::Dnf
    } else if executor
        .run("sh", &["-c", "command -v yum"])
        .await
        .map(|o| o.success())
        .unwrap_or(false)
    {
        PackageManager::Yum
    } else if executor
        .run("sh", &["-c", "command -v apk"])
        .await
        .map(|o| o.success())
        .unwrap_or(false)
    {
        PackageManager::Apk
    } else if executor
        .run("sh", &["-c", "command -v pacman"])
        .await
        .map(|o| o.success())
        .unwrap_or(false)
    {
        PackageManager::Pacman
    } else if executor
        .run("sh", &["-c", "command -v zypper"])
        .await
        .map(|o| o.success())
        .unwrap_or(false)
    {
        PackageManager::Zypper
    } else if executor
        .run("sh", &["-c", "command -v xbps-install"])
        .await
        .map(|o| o.success())
        .unwrap_or(false)
    {
        PackageManager::Xbps
    } else if executor
        .run("sh", &["-c", "command -v emerge"])
        .await
        .map(|o| o.success())
        .unwrap_or(false)
    {
        PackageManager::Emerge
    } else if executor
        .run("sh", &["-c", "command -v nix-env"])
        .await
        .map(|o| o.success())
        .unwrap_or(false)
    {
        PackageManager::Nix
    } else if executor
        .run("sh", &["-c", "command -v brew"])
        .await
        .map(|o| o.success())
        .unwrap_or(false)
    {
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
