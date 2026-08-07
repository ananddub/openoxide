use super::{PackageManager, detect_manager};
use crate::exec::script::{IntoCommand, sh};
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};

#[allow(unused_macros)]
macro_rules! rust {
    ($($t:tt)*) => { $($t)* };
}

pub struct PackageListInstalledBuilder<'a> {
    executor: &'a CommandExecutor,
    manager: Option<PackageManager>,
}

impl<'a> PackageListInstalledBuilder<'a> {
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
            PackageManager::Apt => self.executor.run("dpkg", &["-l"]).await,
            PackageManager::Dnf => self.executor.run("rpm", &["-qa"]).await,
            PackageManager::Yum => self.executor.run("rpm", &["-qa"]).await,
            PackageManager::Apk => self.executor.run("apk", &["info"]).await,
            PackageManager::Pacman => self.executor.run("pacman", &["-Q"]).await,
            PackageManager::Zypper => self.executor.run("rpm", &["-qa"]).await,
            PackageManager::Xbps => self.executor.run("xbps-query", &["-l"]).await,
            PackageManager::Emerge => self.executor.run("qlist", &["-I"]).await,
            PackageManager::Nix => self.executor.run("nix-env", &["-q"]).await,
            PackageManager::Brew => self.executor.run("brew", &["list"]).await,
        }
    }
}

impl<'a> IntoCommand for PackageListInstalledBuilder<'a> {
    fn build_str(&self) -> String {
        if let Some(mgr) = self.manager {
            match mgr {
                PackageManager::Apt => "dpkg -l".to_string(),
                PackageManager::Dnf => "rpm -qa".to_string(),
                PackageManager::Yum => "rpm -qa".to_string(),
                PackageManager::Apk => "apk info".to_string(),
                PackageManager::Pacman => "pacman -Q".to_string(),
                PackageManager::Zypper => "rpm -qa".to_string(),
                PackageManager::Xbps => "xbps-query -l".to_string(),
                PackageManager::Emerge => "qlist -I".to_string(),
                PackageManager::Nix => "nix-env -q".to_string(),
                PackageManager::Brew => "brew list".to_string(),
            }
        } else {
            let script = sh!(if cmd("command", "-v", "dpkg")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("dpkg", "-l");
            } else if cmd("command", "-v", "rpm")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("rpm", "-qa");
            } else if cmd("command", "-v", "apk")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("apk", "info");
            } else if cmd("command", "-v", "pacman")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("pacman", "-Q");
            } else if cmd("command", "-v", "xbps-query")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("xbps-query", "-l");
            } else if cmd("command", "-v", "qlist")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("qlist", "-I");
            } else if cmd("command", "-v", "nix-env")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("nix-env", "-q");
            } else if cmd("command", "-v", "brew")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("brew", "list");
            } else {
                echo("No supported package manager found")
                    .stderr(crate::exec::script::dsl::OutputTarget::StandardError);
                cmd("exit", "1");
            });
            script.build_str()
        }
    }
}
