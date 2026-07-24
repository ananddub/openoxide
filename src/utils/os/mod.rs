use crate::utils::exec::CommandExecutor;
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
    pub fn has_command(&self, bin: impl IntoCommand) -> system::SystemCommandBuilder<'a> {
        system::SystemCommandBuilder::new(
            self.executor,
            "command",
            vec!["-v".to_string(), bin.build_str()],
        )
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

    pub fn awk(&self, target: impl IntoCommand, expr: impl IntoCommand) -> AwkBuilder<'a> {
        AwkBuilder {
            _executor: self.executor,
            target: target.build_str(),
            expr: expr.build_str(),
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
impl<'a> IntoCommand for CaptureStdoutBuilder<'a> {
    fn build_str(&self) -> String {
        format!("$({})", self.cmd)
    }
}

pub struct CaptureStatusBuilder<'a> {
    _executor: &'a CommandExecutor,
    cmd: String,
}
impl<'a> IntoCommand for CaptureStatusBuilder<'a> {
    fn build_str(&self) -> String {
        format!("$(if {}; then echo true; else echo false; fi)", self.cmd)
    }
}

pub struct JqBuilder<'a> {
    _executor: &'a CommandExecutor,
    var: String,
    query: String,
}
impl<'a> IntoCommand for JqBuilder<'a> {
    fn build_str(&self) -> String {
        format!(
            "$(echo {} | jq -r {})",
            escape_arg(&self.var),
            escape_arg(&self.query)
        )
    }
}

pub struct JqFileBuilder<'a> {
    _executor: &'a CommandExecutor,
    file: String,
    query: String,
}
impl<'a> IntoCommand for JqFileBuilder<'a> {
    fn build_str(&self) -> String {
        format!(
            "$(jq -r {} {})",
            escape_arg(&self.query),
            escape_arg(&self.file)
        )
    }
}

pub struct AwkBuilder<'a> {
    _executor: &'a CommandExecutor,
    target: String,
    expr: String,
}
impl<'a> IntoCommand for AwkBuilder<'a> {
    fn build_str(&self) -> String {
        if self.target.starts_with('$')
            || (!self.target.contains(' ') && !self.target.contains('|'))
        {
            format!(
                "$(echo {} | awk {})",
                escape_arg(&self.target),
                escape_arg(&self.expr)
            )
        } else {
            format!("$({} | awk {})", self.target, escape_arg(&self.expr))
        }
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
            format!(
                "$(echo {} | grep {})",
                escape_arg(&self.target),
                escape_arg(&self.pattern)
            )
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
        format!(
            "$(grep {} {})",
            escape_arg(&self.pattern),
            escape_arg(&self.file)
        )
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
pub mod dir;
pub mod disk;
pub mod env;
pub mod file;
pub mod firewall;
pub mod http;
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
