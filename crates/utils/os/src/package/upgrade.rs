use super::{PackageManager, detect_manager};
use crate::exec::script::{IntoCommand, sh};
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};

#[allow(unused_macros)]
macro_rules! rust {
    ($($t:tt)*) => { $($t)* };
}

pub struct PackageUpgradeAllBuilder<'a> {
    executor: &'a CommandExecutor,
    manager: Option<PackageManager>,
}

impl<'a> PackageUpgradeAllBuilder<'a> {
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
            PackageManager::Apt => self.executor.run("apt-get", &["upgrade", "-y"]).await,
            PackageManager::Dnf => self.executor.run("dnf", &["upgrade", "-y"]).await,
            PackageManager::Yum => self.executor.run("yum", &["upgrade", "-y"]).await,
            PackageManager::Apk => self.executor.run("apk", &["upgrade"]).await,
            PackageManager::Pacman => self.executor.run("pacman", &["-Syu", "--noconfirm"]).await,
            PackageManager::Zypper => {
                self.executor
                    .run("zypper", &["--non-interactive", "update"])
                    .await
            }
            PackageManager::Xbps => self.executor.run("xbps-install", &["-Syu"]).await,
            PackageManager::Emerge => {
                self.executor
                    .run("emerge", &["--update", "--deep", "--newuse", "@world"])
                    .await
            }
            PackageManager::Nix => self.executor.run("nix-env", &["-u"]).await,
            PackageManager::Brew => self.executor.run("brew", &["upgrade"]).await,
        }
    }
}

impl<'a> IntoCommand for PackageUpgradeAllBuilder<'a> {
    fn build_str(&self) -> String {
        if let Some(mgr) = self.manager {
            match mgr {
                PackageManager::Apt => "apt-get upgrade -y".to_string(),
                PackageManager::Dnf => "dnf upgrade -y".to_string(),
                PackageManager::Yum => "yum upgrade -y".to_string(),
                PackageManager::Apk => "apk upgrade".to_string(),
                PackageManager::Pacman => "pacman -Syu --noconfirm".to_string(),
                PackageManager::Zypper => "zypper --non-interactive update".to_string(),
                PackageManager::Xbps => "xbps-install -Syu".to_string(),
                PackageManager::Emerge => "emerge --update --deep --newuse @world".to_string(),
                PackageManager::Nix => "nix-env -u".to_string(),
                PackageManager::Brew => "brew upgrade".to_string(),
            }
        } else {
            let script = sh!(if cmd("command", "-v", "apt-get")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("apt-get", "upgrade", "-y");
            } else if cmd("command", "-v", "dnf")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("dnf", "upgrade", "-y");
            } else if cmd("command", "-v", "yum")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("yum", "upgrade", "-y");
            } else if cmd("command", "-v", "apk")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("apk", "upgrade");
            } else if cmd("command", "-v", "pacman")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("pacman", "-Syu", "--noconfirm");
            } else if cmd("command", "-v", "zypper")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("zypper", "--non-interactive", "update");
            } else if cmd("command", "-v", "xbps-install")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("xbps-install", "-Syu");
            } else if cmd("command", "-v", "emerge")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("emerge", "--update", "--deep", "--newuse", "@world");
            } else if cmd("command", "-v", "nix-env")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("nix-env", "-u");
            } else if cmd("command", "-v", "brew")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("brew", "upgrade");
            } else {
                echo("No supported package manager found")
                    .stderr(crate::exec::script::dsl::OutputTarget::StandardError);
                cmd("exit", "1");
            });
            script.build_str()
        }
    }
}
