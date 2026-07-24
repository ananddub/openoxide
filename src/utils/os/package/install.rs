use super::{PackageManager, detect_manager};
use crate::utils::exec::script::{IntoCommand, sh};
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::utils::os::escape_arg;

#[allow(unused_macros)]
macro_rules! rust {
    ($($t:tt)*) => { $($t)* };
}

pub struct PackageInstallBuilder<'a> {
    executor: &'a CommandExecutor,
    name: String,
    manager: Option<PackageManager>,
    yes: bool,
    update: bool,
    no_cache: bool,
}

impl<'a> PackageInstallBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, name: String) -> Self {
        Self {
            executor,
            name,
            manager: None,
            yes: true,
            update: false,
            no_cache: false,
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
    pub fn update(mut self, val: bool) -> Self {
        self.update = val;
        self
    }
    pub fn no_cache(mut self, val: bool) -> Self {
        self.no_cache = val;
        self
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        let mgr = match self.manager {
            Some(m) => m,
            None => detect_manager(self.executor).await,
        };
        match mgr {
            PackageManager::Apt => {
                if self.update {
                    let _ = self.executor.run("apt-get", &["update", "-y"]).await;
                }
                let mut args = vec!["install".to_string()];
                if self.yes {
                    args.push("-y".to_string());
                }
                args.push(self.name);
                self.executor.run("apt-get", &args).await
            }
            PackageManager::Dnf => {
                let mut args = vec!["install".to_string()];
                if self.yes {
                    args.push("-y".to_string());
                }
                args.push(self.name);
                self.executor.run("dnf", &args).await
            }
            PackageManager::Yum => {
                let mut args = vec!["install".to_string()];
                if self.yes {
                    args.push("-y".to_string());
                }
                args.push(self.name);
                self.executor.run("yum", &args).await
            }
            PackageManager::Apk => {
                let mut args = vec!["add".to_string()];
                if self.no_cache {
                    args.push("--no-cache".to_string());
                }
                args.push(self.name);
                self.executor.run("apk", &args).await
            }
            PackageManager::Pacman => {
                let mut args = vec!["-S".to_string()];
                if self.yes {
                    args.push("--noconfirm".to_string());
                }
                args.push(self.name);
                self.executor.run("pacman", &args).await
            }
            PackageManager::Zypper => {
                let mut args = vec!["--non-interactive".to_string(), "install".to_string()];
                args.push(self.name);
                self.executor.run("zypper", &args).await
            }
            PackageManager::Xbps => {
                let mut args = vec!["-Sy".to_string()];
                args.push(self.name);
                self.executor.run("xbps-install", &args).await
            }
            PackageManager::Emerge => {
                let args = vec![self.name];
                self.executor.run("emerge", &args).await
            }
            PackageManager::Nix => {
                let args = vec!["-i".to_string(), self.name];
                self.executor.run("nix-env", &args).await
            }
            PackageManager::Brew => {
                let args = vec!["install".to_string(), self.name];
                self.executor.run("brew", &args).await
            }
        }
    }
}

impl<'a> IntoCommand for PackageInstallBuilder<'a> {
    fn build_str(&self) -> String {
        if let Some(mgr) = self.manager {
            match mgr {
                PackageManager::Apt => {
                    let mut cmd = String::new();
                    if self.update {
                        cmd.push_str("apt-get update -y && ");
                    }
                    cmd.push_str(&format!("apt-get install -y {}", escape_arg(&self.name)));
                    cmd
                }
                PackageManager::Dnf => format!("dnf install -y {}", escape_arg(&self.name)),
                PackageManager::Yum => format!("yum install -y {}", escape_arg(&self.name)),
                PackageManager::Apk => {
                    let flag = if self.no_cache { " --no-cache" } else { "" };
                    format!("apk add{flag} {}", escape_arg(&self.name))
                }
                PackageManager::Pacman => {
                    format!("pacman -S --noconfirm {}", escape_arg(&self.name))
                }
                PackageManager::Zypper => format!(
                    "zypper --non-interactive install {}",
                    escape_arg(&self.name)
                ),
                PackageManager::Xbps => format!("xbps-install -Sy {}", escape_arg(&self.name)),
                PackageManager::Emerge => format!("emerge {}", escape_arg(&self.name)),
                PackageManager::Nix => format!("nix-env -i {}", escape_arg(&self.name)),
                PackageManager::Brew => format!("brew install {}", escape_arg(&self.name)),
            }
        } else {
            let pkg = &self.name;
            let apt_cmd = if self.update {
                format!(
                    "apt-get update -y && apt-get install -y {}",
                    escape_arg(pkg)
                )
            } else {
                format!("apt-get install -y {}", escape_arg(pkg))
            };
            let apk_cmd = if self.no_cache {
                format!("apk add --no-cache {}", escape_arg(pkg))
            } else {
                format!("apk add {}", escape_arg(pkg))
            };

            let script = sh!(if cmd("command", "-v", "apt-get").stdout("/dev/null") {
                cmd("sh", "-c", rust!(apt_cmd));
            } else if cmd("command", "-v", "dnf").stdout("/dev/null") {
                cmd("dnf", "install", "-y", rust!(pkg));
            } else if cmd("command", "-v", "yum").stdout("/dev/null") {
                cmd("yum", "install", "-y", rust!(pkg));
            } else if cmd("command", "-v", "apk").stdout("/dev/null") {
                cmd("sh", "-c", rust!(apk_cmd));
            } else if cmd("command", "-v", "pacman").stdout("/dev/null") {
                cmd("pacman", "-S", "--noconfirm", rust!(pkg));
            } else if cmd("command", "-v", "zypper").stdout("/dev/null") {
                cmd("zypper", "--non-interactive", "install", rust!(pkg));
            } else if cmd("command", "-v", "xbps-install").stdout("/dev/null") {
                cmd("xbps-install", "-Sy", rust!(pkg));
            } else if cmd("command", "-v", "emerge").stdout("/dev/null") {
                cmd("emerge", rust!(pkg));
            } else if cmd("command", "-v", "nix-env").stdout("/dev/null") {
                cmd("nix-env", "-i", rust!(pkg));
            } else if cmd("command", "-v", "brew").stdout("/dev/null") {
                cmd("brew", "install", rust!(pkg));
            } else {
                echo("No supported package manager found").stderr("/dev/stderr");
                cmd("exit", "1");
            });
            script.build_str()
        }
    }
}
