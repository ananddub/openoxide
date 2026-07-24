use super::{PackageManager, detect_manager};
use crate::utils::exec::script::{IntoCommand, sh};
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};

#[allow(unused_macros)]
macro_rules! rust {
    ($($t:tt)*) => { $($t)* };
}

pub struct PackageUpdateIndexBuilder<'a> {
    executor: &'a CommandExecutor,
    manager: Option<PackageManager>,
}

impl<'a> PackageUpdateIndexBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor) -> Self {
        Self {
            executor,
            manager: None,
        }
    }
    pub fn manager(mut self, mgr: PackageManager) -> Self {
        self.manager = Some(mgr);
        self
    }
    pub async fn run(self) -> ExecResult<ExecOutput> {
        let mgr = match self.manager {
            Some(m) => m,
            None => detect_manager(self.executor).await,
        };
        match mgr {
            PackageManager::Apt => self.executor.run("apt-get", &["update", "-y"]).await,
            PackageManager::Dnf => self.executor.run("dnf", &["check-update"]).await,
            PackageManager::Yum => self.executor.run("yum", &["check-update"]).await,
            PackageManager::Apk => self.executor.run("apk", &["update"]).await,
            PackageManager::Pacman => self.executor.run("pacman", &["-Sy"]).await,
            PackageManager::Zypper => self.executor.run("zypper", &["refresh"]).await,
            PackageManager::Xbps => self.executor.run("xbps-install", &["-S"]).await,
            PackageManager::Emerge => self.executor.run("emerge", &["--sync"]).await,
            PackageManager::Nix => self.executor.run("nix-channel", &["--update"]).await,
            PackageManager::Brew => self.executor.run("brew", &["update"]).await,
        }
    }
}

impl<'a> IntoCommand for PackageUpdateIndexBuilder<'a> {
    fn build_str(&self) -> String {
        if let Some(mgr) = self.manager {
            match mgr {
                PackageManager::Apt => "apt-get update -y".to_string(),
                PackageManager::Dnf => "dnf check-update".to_string(),
                PackageManager::Yum => "yum check-update".to_string(),
                PackageManager::Apk => "apk update".to_string(),
                PackageManager::Pacman => "pacman -Sy".to_string(),
                PackageManager::Zypper => "zypper refresh".to_string(),
                PackageManager::Xbps => "xbps-install -S".to_string(),
                PackageManager::Emerge => "emerge --sync".to_string(),
                PackageManager::Nix => "nix-channel --update".to_string(),
                PackageManager::Brew => "brew update".to_string(),
            }
        } else {
            let script = sh!(if cmd("command", "-v", "apt-get").stdout("/dev/null") {
                cmd("apt-get", "update", "-y");
            } else if cmd("command", "-v", "dnf").stdout("/dev/null") {
                cmd("dnf", "check-update");
            } else if cmd("command", "-v", "yum").stdout("/dev/null") {
                cmd("yum", "check-update");
            } else if cmd("command", "-v", "apk").stdout("/dev/null") {
                cmd("apk", "update");
            } else if cmd("command", "-v", "pacman").stdout("/dev/null") {
                cmd("pacman", "-Sy");
            } else if cmd("command", "-v", "zypper").stdout("/dev/null") {
                cmd("zypper", "refresh");
            } else if cmd("command", "-v", "xbps-install").stdout("/dev/null") {
                cmd("xbps-install", "-S");
            } else if cmd("command", "-v", "emerge").stdout("/dev/null") {
                cmd("emerge", "--sync");
            } else if cmd("command", "-v", "nix-channel").stdout("/dev/null") {
                cmd("nix-channel", "--update");
            } else if cmd("command", "-v", "brew").stdout("/dev/null") {
                cmd("brew", "update");
            } else {
                echo("No supported package manager found").stderr("/dev/stderr");
                cmd("exit", "1");
            });
            script.build_str()
        }
    }
}
