use super::{PackageManager, detect_manager};
use crate::escape_arg;
use crate::exec::script::{IntoCommand, sh};
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};

#[allow(unused_macros)]
macro_rules! rust {
    ($($t:tt)*) => { $($t)* };
}

pub struct PackageCheckInstalledBuilder<'a> {
    executor: &'a CommandExecutor,
    name: String,
    manager: Option<PackageManager>,
}

impl<'a> PackageCheckInstalledBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, name: String) -> Self {
        Self {
            executor,
            name,
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
            PackageManager::Apt => self.executor.run("dpkg", &["-s", &self.name]).await,
            PackageManager::Dnf => self.executor.run("rpm", &["-q", &self.name]).await,
            PackageManager::Yum => self.executor.run("rpm", &["-q", &self.name]).await,
            PackageManager::Apk => self.executor.run("apk", &["info", "-e", &self.name]).await,
            PackageManager::Pacman => self.executor.run("pacman", &["-Q", &self.name]).await,
            PackageManager::Zypper => self.executor.run("rpm", &["-q", &self.name]).await,
            PackageManager::Xbps => self.executor.run("xbps-query", &["-S", &self.name]).await,
            PackageManager::Emerge => self.executor.run("qlist", &["-I", "-e", &self.name]).await,
            PackageManager::Nix => self.executor.run("nix-env", &["-q", &self.name]).await,
            PackageManager::Brew => self.executor.run("brew", &["list", &self.name]).await,
        }
    }
}

impl<'a> IntoCommand for PackageCheckInstalledBuilder<'a> {
    fn build_str(&self) -> String {
        if let Some(mgr) = self.manager {
            match mgr {
                PackageManager::Apt => format!("dpkg -s {}", escape_arg(&self.name)),
                PackageManager::Dnf => format!("rpm -q {}", escape_arg(&self.name)),
                PackageManager::Yum => format!("rpm -q {}", escape_arg(&self.name)),
                PackageManager::Apk => format!("apk info -e {}", escape_arg(&self.name)),
                PackageManager::Pacman => format!("pacman -Q {}", escape_arg(&self.name)),
                PackageManager::Zypper => format!("rpm -q {}", escape_arg(&self.name)),
                PackageManager::Xbps => format!("xbps-query -S {}", escape_arg(&self.name)),
                PackageManager::Emerge => format!("qlist -I -e {}", escape_arg(&self.name)),
                PackageManager::Nix => format!("nix-env -q {}", escape_arg(&self.name)),
                PackageManager::Brew => format!("brew list {}", escape_arg(&self.name)),
            }
        } else {
            let pkg = &self.name;
            let script = sh!(if cmd("command", "-v", "dpkg")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("dpkg", "-s", dynamic!(pkg));
            } else if cmd("command", "-v", "rpm")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("rpm", "-q", dynamic!(pkg));
            } else if cmd("command", "-v", "apk")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("apk", "info", "-e", dynamic!(pkg));
            } else if cmd("command", "-v", "pacman")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("pacman", "-Q", dynamic!(pkg));
            } else if cmd("command", "-v", "xbps-query")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("xbps-query", "-S", dynamic!(pkg));
            } else if cmd("command", "-v", "qlist")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("qlist", "-I", "-e", dynamic!(pkg));
            } else if cmd("command", "-v", "nix-env")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("nix-env", "-q", dynamic!(pkg));
            } else if cmd("command", "-v", "brew")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("brew", "list", dynamic!(pkg));
            } else {
                echo("No supported package manager found")
                    .stderr(crate::exec::script::dsl::OutputTarget::StandardError);
                cmd("exit", "1");
            });
            script.build_str()
        }
    }
}
