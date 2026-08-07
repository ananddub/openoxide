use super::{PackageManager, detect_manager};
use crate::exec::script::{IntoCommand, sh};
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::escape_arg;

#[allow(unused_macros)]
macro_rules! rust {
    ($($t:tt)*) => { $($t)* };
}

pub struct PackageRemoveBuilder<'a> {
    executor: &'a CommandExecutor,
    name: String,
    manager: Option<PackageManager>,
    yes: bool,
}

impl<'a> PackageRemoveBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, name: String) -> Self {
        Self {
            executor,
            name,
            manager: None,
            yes: true,
        }
    }
    pub fn manager(mut self, mgr: PackageManager) -> Self {
        self.manager = Some(mgr);
        self
    }
    pub fn yes(mut self, val: bool) -> Self {
        self.yes = val;
        self
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        let mgr = match self.manager {
            Some(m) => m,
            None => detect_manager(self.executor).await,
        };
        match mgr {
            PackageManager::Apt => {
                let mut args = vec!["remove".to_string()];
                if self.yes {
                    args.push("-y".to_string());
                }
                args.push(self.name);
                self.executor.run("apt-get", &args).await
            }
            PackageManager::Dnf => {
                let mut args = vec!["remove".to_string()];
                if self.yes {
                    args.push("-y".to_string());
                }
                args.push(self.name);
                self.executor.run("dnf", &args).await
            }
            PackageManager::Yum => {
                let mut args = vec!["remove".to_string()];
                if self.yes {
                    args.push("-y".to_string());
                }
                args.push(self.name);
                self.executor.run("yum", &args).await
            }
            PackageManager::Apk => self.executor.run("apk", &["del", &self.name]).await,
            PackageManager::Pacman => {
                let mut args = vec!["-R".to_string()];
                if self.yes {
                    args.push("--noconfirm".to_string());
                }
                args.push(self.name);
                self.executor.run("pacman", &args).await
            }
            PackageManager::Zypper => {
                let mut args = vec!["--non-interactive".to_string(), "remove".to_string()];
                args.push(self.name);
                self.executor.run("zypper", &args).await
            }
            PackageManager::Xbps => {
                let mut args = vec!["-y".to_string()];
                args.push(self.name);
                self.executor.run("xbps-remove", &args).await
            }
            PackageManager::Emerge => {
                let args = vec!["--unmerge".to_string(), self.name];
                self.executor.run("emerge", &args).await
            }
            PackageManager::Nix => {
                let args = vec!["-e".to_string(), self.name];
                self.executor.run("nix-env", &args).await
            }
            PackageManager::Brew => {
                let args = vec!["uninstall".to_string(), self.name];
                self.executor.run("brew", &args).await
            }
        }
    }
}

impl<'a> IntoCommand for PackageRemoveBuilder<'a> {
    fn build_str(&self) -> String {
        if let Some(mgr) = self.manager {
            match mgr {
                PackageManager::Apt => format!("apt-get remove -y {}", escape_arg(&self.name)),
                PackageManager::Dnf => format!("dnf remove -y {}", escape_arg(&self.name)),
                PackageManager::Yum => format!("yum remove -y {}", escape_arg(&self.name)),
                PackageManager::Apk => format!("apk del {}", escape_arg(&self.name)),
                PackageManager::Pacman => {
                    format!("pacman -R --noconfirm {}", escape_arg(&self.name))
                }
                PackageManager::Zypper => {
                    format!("zypper --non-interactive remove {}", escape_arg(&self.name))
                }
                PackageManager::Xbps => format!("xbps-remove -y {}", escape_arg(&self.name)),
                PackageManager::Emerge => format!("emerge --unmerge {}", escape_arg(&self.name)),
                PackageManager::Nix => format!("nix-env -e {}", escape_arg(&self.name)),
                PackageManager::Brew => format!("brew uninstall {}", escape_arg(&self.name)),
            }
        } else {
            let pkg = &self.name;
            let script = sh!(if cmd("command", "-v", "apt-get")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("apt-get", "remove", "-y", dynamic!(pkg));
            } else if cmd("command", "-v", "dnf")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("dnf", "remove", "-y", dynamic!(pkg));
            } else if cmd("command", "-v", "yum")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("yum", "remove", "-y", dynamic!(pkg));
            } else if cmd("command", "-v", "apk")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("apk", "del", dynamic!(pkg));
            } else if cmd("command", "-v", "pacman")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("pacman", "-R", "--noconfirm", dynamic!(pkg));
            } else if cmd("command", "-v", "zypper")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("zypper", "--non-interactive", "remove", dynamic!(pkg));
            } else if cmd("command", "-v", "xbps-remove")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("xbps-remove", "-y", dynamic!(pkg));
            } else if cmd("command", "-v", "emerge")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("emerge", "--unmerge", dynamic!(pkg));
            } else if cmd("command", "-v", "nix-env")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("nix-env", "-e", dynamic!(pkg));
            } else if cmd("command", "-v", "brew")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("brew", "uninstall", dynamic!(pkg));
            } else {
                echo("No supported package manager found")
                    .stderr(crate::exec::script::dsl::OutputTarget::StandardError);
                cmd("exit", "1");
            });
            script.build_str()
        }
    }
}
