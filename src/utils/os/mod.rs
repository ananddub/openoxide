use crate::utils::exec::CommandExecutor;
use crate::utils::exec::script::dsl::{ArgToken, CaptureSource, Command, ShellIR};
use crate::utils::exec::script::{IntoCommand, shell_single_quote};

pub struct OsCli<'a> {
    pub(crate) executor: &'a CommandExecutor,
}

impl<'a> OsCli<'a> {
    pub fn new(executor: &'a CommandExecutor) -> Self {
        Self { executor }
    }

    pub fn port(&self) -> port::PortCli<'a> {
        port::PortCli {
            executor: self.executor,
        }
    }
    pub fn lock(&self) -> lock::LockCli<'a> {
        lock::LockCli {
            executor: self.executor,
        }
    }
    pub fn http(&self) -> http::HttpCli<'a> {
        http::HttpCli {
            executor: self.executor,
        }
    }
    pub fn system(&self) -> system::SystemCli<'a> {
        system::SystemCli {
            executor: self.executor,
        }
    }
    pub fn process_api(&self) -> process::ProcessCli<'a> {
        process::ProcessCli {
            executor: self.executor,
        }
    }
    pub fn service_api(&self) -> service::ServiceCli<'a> {
        service::ServiceCli {
            executor: self.executor,
        }
    }
    pub fn package_api(&self) -> package::PackageCli<'a> {
        package::PackageCli {
            executor: self.executor,
        }
    }
    pub fn dir_api(&self) -> dir::DirCli<'a> {
        dir::DirCli {
            executor: self.executor,
        }
    }
    pub fn network(&self) -> network::NetworkCli<'a> {
        network::NetworkCli {
            executor: self.executor,
        }
    }
    pub fn env(&self) -> env::EnvCli<'a> {
        env::EnvCli {
            executor: self.executor,
        }
    }
    pub fn mount_api(&self) -> mount::MountCli<'a> {
        mount::MountCli {
            executor: self.executor,
        }
    }
    pub fn disk(&self) -> disk::DiskCli<'a> {
        disk::DiskCli {
            executor: self.executor,
        }
    }
    pub fn firewall(&self) -> firewall::FirewallCli<'a> {
        firewall::FirewallCli {
            executor: self.executor,
        }
    }
    pub fn resource(&self) -> resource::ResourceCli<'a> {
        resource::ResourceCli {
            executor: self.executor,
        }
    }
    pub fn file_api(&self) -> file::FileCli<'a> {
        file::FileCli {
            executor: self.executor,
        }
    }
    pub fn symlink_api(&self) -> symlink::SymlinkCli<'a> {
        symlink::SymlinkCli {
            executor: self.executor,
        }
    }
    pub fn archive(&self, path: impl IntoCommand) -> archive::ArchiveBuilder<'a> {
        archive::ArchiveBuilder::new(self.executor, path)
    }
    pub fn crypto(&self) -> crypto::CryptoCli<'a> {
        crypto::CryptoCli {
            executor: self.executor,
        }
    }
    pub fn gpu(&self) -> gpu::GpuCli<'a> {
        gpu::GpuCli {
            executor: self.executor,
        }
    }
    pub fn diagnostics(&self) -> diagnostics::DiagnosticsCli<'a> {
        diagnostics::DiagnosticsCli {
            executor: self.executor,
        }
    }
    pub fn wireguard(&self) -> wireguard::WireGuardCli<'a> {
        wireguard::WireGuardCli {
            executor: self.executor,
        }
    }

    // Direct methods for zero-boilerplate usage
    pub fn file(&self, path: impl IntoCommand) -> file::FileBuilder<'a> {
        file::FileBuilder::new(self.executor, path)
    }
    pub fn dir(&self, path: impl IntoCommand) -> dir::DirBuilder<'a> {
        dir::DirBuilder::new(self.executor, path)
    }
    pub fn package(&self, name: impl IntoCommand) -> package::PackageBuilder<'a> {
        package::PackageBuilder::new(self.executor, name)
    }
    pub fn service(&self, name: impl IntoCommand) -> service::ServiceBuilder<'a> {
        service::ServiceBuilder::new(self.executor, name)
    }
    pub fn process(&self, pid_or_name: impl IntoCommand) -> process::ProcessBuilder<'a> {
        process::ProcessBuilder::new(self.executor, pid_or_name)
    }
    pub fn mount(
        &self,
        source: impl IntoCommand,
        target: impl IntoCommand,
    ) -> mount::MountBuilder<'a> {
        mount::MountBuilder::new(self.executor, Some(source), target)
    }
    pub fn mount_ref(&self, target: impl IntoCommand) -> mount::MountBuilder<'a> {
        mount::MountBuilder::new(self.executor, None::<&str>, target)
    }
    pub fn symlink(
        &self,
        target: impl IntoCommand,
        link: impl IntoCommand,
    ) -> symlink::SymlinkBuilder<'a> {
        symlink::SymlinkBuilder::new(self.executor, Some(target.build_str()), link.build_str())
    }
    pub fn symlink_ref(&self, link: impl IntoCommand) -> symlink::SymlinkBuilder<'a> {
        symlink::SymlinkBuilder::new(self.executor, None, link.build_str())
    }
    pub fn has_command(&self, bin: impl IntoCommand) -> system::CommandExistsBuilder<'a> {
        system::CommandExistsBuilder::new(self.executor, bin)
    }

    pub fn shell_installer(&self, url: impl IntoCommand) -> install::ShellInstallerBuilder<'a> {
        install::ShellInstallerBuilder::new(self.executor, url)
    }

    pub fn tarball_installer(
        &self,
        url: impl IntoCommand,
        destination: impl IntoCommand,
    ) -> install::TarballInstallerBuilder<'a> {
        install::TarballInstallerBuilder::new(self.executor, url, destination)
    }

    pub fn pack_installer(&self, version: impl Into<String>) -> install::PackInstallerBuilder<'a> {
        install::PackInstallerBuilder::new(self.executor, version)
    }

    pub fn capture_stdout(&self, cmd: impl IntoCommand) -> CaptureStdoutBuilder<'a> {
        CaptureStdoutBuilder {
            _executor: self.executor,
            cmd: cmd.build_str(),
        }
    }

    pub fn capture_status(&self, cmd: impl IntoCommand) -> CaptureStatusBuilder<'a> {
        CaptureStatusBuilder {
            _executor: self.executor,
            cmd: cmd.build_str(),
        }
    }

    pub fn jq(&self, var: impl IntoCommand, query: impl IntoCommand) -> JqBuilder<'a> {
        JqBuilder {
            _executor: self.executor,
            var: var.build_str(),
            query: query.build_str(),
        }
    }

    pub fn jq_file(&self, file: impl IntoCommand, query: impl IntoCommand) -> JqFileBuilder<'a> {
        JqFileBuilder {
            _executor: self.executor,
            file: file.build_str(),
            query: query.build_str(),
        }
    }

    pub fn sed_file(
        &self,
        file: impl IntoCommand,
        pattern: impl IntoCommand,
    ) -> SedFileBuilder<'a> {
        SedFileBuilder {
            _executor: self.executor,
            file: file.build_str(),
            pattern: pattern.build_str(),
        }
    }

    pub fn grep(&self, target: impl IntoCommand, pattern: impl IntoCommand) -> GrepBuilder<'a> {
        GrepBuilder {
            _executor: self.executor,
            target: target.build_str(),
            pattern: pattern.build_str(),
        }
    }

    pub fn grep_file(
        &self,
        file: impl IntoCommand,
        pattern: impl IntoCommand,
    ) -> GrepFileBuilder<'a> {
        GrepFileBuilder {
            _executor: self.executor,
            file: file.build_str(),
            pattern: pattern.build_str(),
        }
    }
}

pub struct CaptureStdoutBuilder<'a> {
    _executor: &'a CommandExecutor,
    cmd: String,
}

fn shell_arg(value: &str) -> ArgToken {
    value
        .strip_prefix('$')
        .filter(|name| !name.is_empty())
        .map(|name| ArgToken::Variable(name.to_owned()))
        .unwrap_or_else(|| ArgToken::Literal(value.to_owned()))
}

fn shell_command(name: &str, args: impl IntoIterator<Item = ArgToken>) -> ShellIR {
    ShellIR::Command(Command {
        name: name.to_owned(),
        args: args.into_iter().collect(),
    })
}

fn capture_stdout(command: ShellIR) -> String {
    ShellIR::Capture {
        cmd: Box::new(command),
        source: CaptureSource::Stdout,
    }
    .to_bash()
}

impl<'a> IntoCommand for CaptureStdoutBuilder<'a> {
    fn build_str(&self) -> String {
        capture_stdout(ShellIR::Raw(self.cmd.clone()))
    }
}

pub struct CaptureStatusBuilder<'a> {
    _executor: &'a CommandExecutor,
    cmd: String,
}
impl<'a> IntoCommand for CaptureStatusBuilder<'a> {
    fn build_str(&self) -> String {
        ShellIR::Capture {
            cmd: Box::new(ShellIR::Raw(self.cmd.clone())),
            source: CaptureSource::Status,
        }
        .to_bash()
    }
}

pub struct JqBuilder<'a> {
    _executor: &'a CommandExecutor,
    var: String,
    query: String,
}
impl<'a> IntoCommand for JqBuilder<'a> {
    fn build_str(&self) -> String {
        capture_stdout(ShellIR::Pipeline(vec![
            match shell_command("echo", [shell_arg(&self.var)]) {
                ShellIR::Command(command) => command,
                _ => unreachable!(),
            },
            match shell_command("jq", [shell_arg("-r"), shell_arg(&self.query)]) {
                ShellIR::Command(command) => command,
                _ => unreachable!(),
            },
        ]))
    }
}

pub struct JqFileBuilder<'a> {
    _executor: &'a CommandExecutor,
    file: String,
    query: String,
}
impl<'a> IntoCommand for JqFileBuilder<'a> {
    fn build_str(&self) -> String {
        capture_stdout(shell_command(
            "jq",
            [
                shell_arg("-r"),
                shell_arg(&self.query),
                shell_arg(&self.file),
            ],
        ))
    }
}

pub struct SedFileBuilder<'a> {
    _executor: &'a CommandExecutor,
    file: String,
    pattern: String,
}
impl<'a> IntoCommand for SedFileBuilder<'a> {
    fn build_str(&self) -> String {
        format!(
            "sed -i {} {}",
            escape_arg(&self.pattern),
            escape_arg(&self.file)
        )
    }
}

pub struct GrepBuilder<'a> {
    _executor: &'a CommandExecutor,
    target: String,
    pattern: String,
}
impl<'a> IntoCommand for GrepBuilder<'a> {
    fn build_str(&self) -> String {
        if self.target.starts_with('$')
            || (!self.target.contains(' ') && !self.target.contains('|'))
        {
            capture_stdout(ShellIR::Pipeline(vec![
                match shell_command("echo", [shell_arg(&self.target)]) {
                    ShellIR::Command(command) => command,
                    _ => unreachable!(),
                },
                match shell_command("grep", [shell_arg(&self.pattern)]) {
                    ShellIR::Command(command) => command,
                    _ => unreachable!(),
                },
            ]))
        } else {
            format!("$({} | grep {})", self.target, escape_arg(&self.pattern))
        }
    }
}

pub struct GrepFileBuilder<'a> {
    _executor: &'a CommandExecutor,
    file: String,
    pattern: String,
}
impl<'a> IntoCommand for GrepFileBuilder<'a> {
    fn build_str(&self) -> String {
        capture_stdout(shell_command(
            "grep",
            [shell_arg(&self.pattern), shell_arg(&self.file)],
        ))
    }
}

pub(crate) fn escape_arg(c: impl AsRef<str>) -> String {
    let s = c.as_ref();
    if s.starts_with('$') {
        format!("\"{}\"", s)
    } else {
        shell_single_quote(s)
    }
}

// Submodules
pub mod archive;
pub mod crypto;
pub mod diagnostics;
pub mod dir;
pub mod disk;
pub mod env;
pub mod file;
pub mod firewall;
pub mod gpu;
pub mod http;
pub mod install;
pub mod lock;
pub mod mount;
pub mod network;
pub mod package;
pub mod port;
pub mod process;
pub mod resource;
pub mod service;
pub mod symlink;
pub mod system;
pub mod wireguard;
