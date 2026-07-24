use super::{PackageManager, detect_manager};
use crate::utils::exec::script::{IntoCommand, sh};
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::utils::os::escape_arg;

#[allow(unused_macros)]
macro_rules! rust {
    ($($t:tt)*) => { $($t)* };
}

pub struct PackageSearchBuilder<'a> {
    executor: &'a CommandExecutor,
    query: String,
    manager: Option<PackageManager>,
}

impl<'a> PackageSearchBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, query: impl IntoCommand) -> Self {
        Self {
            executor,
            query: query.build_str(),
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
            PackageManager::Apt => {
                self.executor
                    .run("apt-cache", &["search", &self.query])
                    .await
            }
            PackageManager::Dnf => self.executor.run("dnf", &["search", &self.query]).await,
            PackageManager::Yum => self.executor.run("yum", &["search", &self.query]).await,
            PackageManager::Apk => self.executor.run("apk", &["search", &self.query]).await,
            PackageManager::Pacman => self.executor.run("pacman", &["-Ss", &self.query]).await,
            PackageManager::Zypper => self.executor.run("zypper", &["search", &self.query]).await,
            PackageManager::Xbps => self.executor.run("xbps-query", &["-Rs", &self.query]).await,
            PackageManager::Emerge => {
                self.executor
                    .run("emerge", &["--search", &self.query])
                    .await
            }
            PackageManager::Nix => self.executor.run("nix-env", &["-qa", &self.query]).await,
            PackageManager::Brew => self.executor.run("brew", &["search", &self.query]).await,
        }
    }
}

impl<'a> IntoCommand for PackageSearchBuilder<'a> {
    fn build_str(&self) -> String {
        if let Some(mgr) = self.manager {
            match mgr {
                PackageManager::Apt => format!("apt-cache search {}", escape_arg(&self.query)),
                PackageManager::Dnf => format!("dnf search {}", escape_arg(&self.query)),
                PackageManager::Yum => format!("yum search {}", escape_arg(&self.query)),
                PackageManager::Apk => format!("apk search {}", escape_arg(&self.query)),
                PackageManager::Pacman => format!("pacman -Ss {}", escape_arg(&self.query)),
                PackageManager::Zypper => format!("zypper search {}", escape_arg(&self.query)),
                PackageManager::Xbps => format!("xbps-query -Rs {}", escape_arg(&self.query)),
                PackageManager::Emerge => format!("emerge --search {}", escape_arg(&self.query)),
                PackageManager::Nix => format!("nix-env -qa {}", escape_arg(&self.query)),
                PackageManager::Brew => format!("brew search {}", escape_arg(&self.query)),
            }
        } else {
            let q = &self.query;
            let script = sh!(if cmd("command", "-v", "apt-cache").stdout("/dev/null") {
                cmd("apt-cache", "search", rust!(q));
            } else if cmd("command", "-v", "dnf").stdout("/dev/null") {
                cmd("dnf", "search", rust!(q));
            } else if cmd("command", "-v", "yum").stdout("/dev/null") {
                cmd("yum", "search", rust!(q));
            } else if cmd("command", "-v", "apk").stdout("/dev/null") {
                cmd("apk", "search", rust!(q));
            } else if cmd("command", "-v", "pacman").stdout("/dev/null") {
                cmd("pacman", "-Ss", rust!(q));
            } else if cmd("command", "-v", "zypper").stdout("/dev/null") {
                cmd("zypper", "search", rust!(q));
            } else if cmd("command", "-v", "xbps-query").stdout("/dev/null") {
                cmd("xbps-query", "-Rs", rust!(q));
            } else if cmd("command", "-v", "emerge").stdout("/dev/null") {
                cmd("emerge", "--search", rust!(q));
            } else if cmd("command", "-v", "nix-env").stdout("/dev/null") {
                cmd("nix-env", "-qa", rust!(q));
            } else if cmd("command", "-v", "brew").stdout("/dev/null") {
                cmd("brew", "search", rust!(q));
            } else {
                echo("No supported package manager found").stderr("/dev/stderr");
                cmd("exit", "1");
            });
            script.build_str()
        }
    }
}
