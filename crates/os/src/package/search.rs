use super::{PackageManager, detect_manager};
use crate::escape_arg;
use crate::exec::script::{IntoCommand, sh};
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};

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
            let script = sh!(if cmd("command", "-v", "apt-cache")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("apt-cache", "search", dynamic!(q));
            } else if cmd("command", "-v", "dnf")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("dnf", "search", dynamic!(q));
            } else if cmd("command", "-v", "yum")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("yum", "search", dynamic!(q));
            } else if cmd("command", "-v", "apk")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("apk", "search", dynamic!(q));
            } else if cmd("command", "-v", "pacman")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("pacman", "-Ss", dynamic!(q));
            } else if cmd("command", "-v", "zypper")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("zypper", "search", dynamic!(q));
            } else if cmd("command", "-v", "xbps-query")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("xbps-query", "-Rs", dynamic!(q));
            } else if cmd("command", "-v", "emerge")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("emerge", "--search", dynamic!(q));
            } else if cmd("command", "-v", "nix-env")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("nix-env", "-qa", dynamic!(q));
            } else if cmd("command", "-v", "brew")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("brew", "search", dynamic!(q));
            } else {
                echo("No supported package manager found")
                    .stderr(crate::exec::script::dsl::OutputTarget::StandardError);
                cmd("exit", "1");
            });
            script.build_str()
        }
    }
}
