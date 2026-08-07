use super::{PackageManager, detect_manager};
use crate::exec::script::dsl::{ArgToken, Command, ShellIR};
use crate::exec::script::{IntoCommand, sh};
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};

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
        let command = |name: &str, args: Vec<String>| {
            ShellIR::Command(Command {
                name: name.to_owned(),
                args: args.into_iter().map(ArgToken::Literal).collect(),
            })
        };
        if let Some(mgr) = self.manager {
            let install = match mgr {
                PackageManager::Apt => {
                    let mut commands = Vec::new();
                    if self.update {
                        commands.push(command("apt-get", vec!["update".into(), "-y".into()]));
                    }
                    commands.push(command(
                        "apt-get",
                        vec!["install".into(), "-y".into(), self.name.clone()],
                    ));
                    ShellIR::Sequence(commands)
                }
                PackageManager::Dnf => command(
                    "dnf",
                    vec!["install".into(), "-y".into(), self.name.clone()],
                ),
                PackageManager::Yum => command(
                    "yum",
                    vec!["install".into(), "-y".into(), self.name.clone()],
                ),
                PackageManager::Apk => {
                    let mut args = vec!["add".into()];
                    if self.no_cache {
                        args.push("--no-cache".into());
                    }
                    args.push(self.name.clone());
                    command("apk", args)
                }
                PackageManager::Pacman => command(
                    "pacman",
                    vec!["-S".into(), "--noconfirm".into(), self.name.clone()],
                ),
                PackageManager::Zypper => command(
                    "zypper",
                    vec![
                        "--non-interactive".into(),
                        "install".into(),
                        self.name.clone(),
                    ],
                ),
                PackageManager::Xbps => {
                    command("xbps-install", vec!["-Sy".into(), self.name.clone()])
                }
                PackageManager::Emerge => command("emerge", vec![self.name.clone()]),
                PackageManager::Nix => command("nix-env", vec!["-i".into(), self.name.clone()]),
                PackageManager::Brew => command("brew", vec!["install".into(), self.name.clone()]),
            };
            install.build_str()
        } else {
            let pkg = &self.name;
            let command = |name: &str, args: Vec<&str>| {
                ShellIR::Command(Command {
                    name: name.to_owned(),
                    args: args
                        .into_iter()
                        .map(|value| ArgToken::Literal(value.to_owned()))
                        .collect(),
                })
            };
            let mut apt = Vec::new();
            if self.update {
                apt.push(command("apt-get", vec!["update", "-y"]));
            }
            apt.push(command("apt-get", vec!["install", "-y", pkg]));
            let apt = ShellIR::Sequence(apt);
            let mut apk_args = vec!["add"];
            if self.no_cache {
                apk_args.push("--no-cache");
            }
            apk_args.push(pkg);
            let apk = command("apk", apk_args);

            let script = sh!(if cmd("command", "-v", "apt-get")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                ir!(apt.clone());
            } else if cmd("command", "-v", "dnf")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("dnf", "install", "-y", dynamic!(pkg));
            } else if cmd("command", "-v", "yum")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("yum", "install", "-y", dynamic!(pkg));
            } else if cmd("command", "-v", "apk")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                ir!(apk.clone());
            } else if cmd("command", "-v", "pacman")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("pacman", "-S", "--noconfirm", dynamic!(pkg));
            } else if cmd("command", "-v", "zypper")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("zypper", "--non-interactive", "install", dynamic!(pkg));
            } else if cmd("command", "-v", "xbps-install")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("xbps-install", "-Sy", dynamic!(pkg));
            } else if cmd("command", "-v", "emerge")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("emerge", dynamic!(pkg));
            } else if cmd("command", "-v", "nix-env")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("nix-env", "-i", dynamic!(pkg));
            } else if cmd("command", "-v", "brew")
                .stdout(crate::exec::script::dsl::OutputTarget::Null)
            {
                cmd("brew", "install", dynamic!(pkg));
            } else {
                echo("No supported package manager found")
                    .stderr(crate::exec::script::dsl::OutputTarget::StandardError);
                cmd("exit", "1");
            });
            script.build_str()
        }
    }
}
