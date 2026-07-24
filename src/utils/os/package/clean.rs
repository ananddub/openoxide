use super::{PackageManager, detect_manager};
use crate::utils::exec::script::{IntoCommand, sh};
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};

#[allow(unused_macros)]
macro_rules! rust {
    ($($t:tt)*) => { $($t)* };
}

pub struct PackageCleanBuilder<'a> {
    executor: &'a CommandExecutor,
    manager: Option<PackageManager>,
}

impl<'a> PackageCleanBuilder<'a> {
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
            PackageManager::Apt => self.executor.run("apt-get", &["clean"]).await,
            PackageManager::Dnf => self.executor.run("dnf", &["clean", "all"]).await,
            PackageManager::Yum => self.executor.run("yum", &["clean", "all"]).await,
            PackageManager::Apk => self.executor.run("apk", &["cache", "clean"]).await,
            PackageManager::Pacman => self.executor.run("pacman", &["-Sc", "--noconfirm"]).await,
            PackageManager::Zypper => self.executor.run("zypper", &["clean", "--all"]).await,
            PackageManager::Xbps => self.executor.run("xbps-remove", &["-O"]).await,
            PackageManager::Emerge => self.executor.run("eclean", &["distfiles"]).await,
            PackageManager::Nix => self.executor.run("nix-store", &["--gc"]).await,
            PackageManager::Brew => self.executor.run("brew", &["cleanup"]).await,
        }
    }
}

impl<'a> IntoCommand for PackageCleanBuilder<'a> {
    fn build_str(&self) -> String {
        if let Some(mgr) = self.manager {
            match mgr {
                PackageManager::Apt => "apt-get clean".to_string(),
                PackageManager::Dnf => "dnf clean all".to_string(),
                PackageManager::Yum => "yum clean all".to_string(),
                PackageManager::Apk => "apk cache clean".to_string(),
                PackageManager::Pacman => "pacman -Sc --noconfirm".to_string(),
                PackageManager::Zypper => "zypper clean --all".to_string(),
                PackageManager::Xbps => "xbps-remove -O".to_string(),
                PackageManager::Emerge => "eclean distfiles".to_string(),
                PackageManager::Nix => "nix-store --gc".to_string(),
                PackageManager::Brew => "brew cleanup".to_string(),
            }
        } else {
            let script = sh!(if cmd("command", "-v", "apt-get").stdout("/dev/null") {
                cmd("apt-get", "clean");
            } else if cmd("command", "-v", "dnf").stdout("/dev/null") {
                cmd("dnf", "clean", "all");
            } else if cmd("command", "-v", "yum").stdout("/dev/null") {
                cmd("yum", "clean", "all");
            } else if cmd("command", "-v", "apk").stdout("/dev/null") {
                cmd("apk", "cache", "clean");
            } else if cmd("command", "-v", "pacman").stdout("/dev/null") {
                cmd("pacman", "-Sc", "--noconfirm");
            } else if cmd("command", "-v", "zypper").stdout("/dev/null") {
                cmd("zypper", "clean", "--all");
            } else if cmd("command", "-v", "xbps-remove").stdout("/dev/null") {
                cmd("xbps-remove", "-O");
            } else if cmd("command", "-v", "eclean").stdout("/dev/null") {
                cmd("eclean", "distfiles");
            } else if cmd("command", "-v", "nix-store").stdout("/dev/null") {
                cmd("nix-store", "--gc");
            } else if cmd("command", "-v", "brew").stdout("/dev/null") {
                cmd("brew", "cleanup");
            } else {
                echo("No supported package manager found").stderr("/dev/stderr");
                cmd("exit", "1");
            });
            script.build_str()
        }
    }
}
