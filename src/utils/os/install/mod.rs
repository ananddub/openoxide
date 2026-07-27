use crate::utils::exec::script::{IntoCommand, ShellIR, sh, shell_single_quote};
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};

pub struct ShellInstallerBuilder<'a> {
    executor: &'a CommandExecutor,
    url: String,
    shell: String,
    environment: Vec<(String, String)>,
    arguments: Vec<String>,
}

impl<'a> ShellInstallerBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, url: impl IntoCommand) -> Self {
        Self {
            executor,
            url: url.build_str(),
            shell: "sh".to_owned(),
            environment: Vec::new(),
            arguments: Vec::new(),
        }
    }

    pub fn shell(mut self, shell: impl Into<String>) -> Self {
        self.shell = shell.into();
        self
    }

    pub fn env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.environment.push((key.into(), value.into()));
        self
    }

    pub fn arg(mut self, argument: impl Into<String>) -> Self {
        self.arguments.push(argument.into());
        self
    }

    pub async fn run(self) -> ExecResult<()> {
        let temporary = self
            .executor
            .run("mktemp", ["-t", "rustploy-installer.XXXXXX"])
            .await?
            .stdout_trimmed()
            .to_owned();

        let result = async {
            self.executor
                .run(
                    "curl",
                    ["-fsSL", self.url.as_str(), "-o", temporary.as_str()],
                )
                .await?;

            let mut args = self
                .environment
                .into_iter()
                .map(|(key, value)| format!("{key}={value}"))
                .collect::<Vec<_>>();
            args.push(self.shell);
            args.push(temporary.clone());
            args.extend(self.arguments);
            self.executor.run("env", args).await?;
            Ok(())
        }
        .await;

        let _ = self.executor.run("rm", ["-f", temporary.as_str()]).await;
        result
    }
}

impl IntoCommand for ShellInstallerBuilder<'_> {
    fn build_str(&self) -> String {
        let mut env_args = self
            .environment
            .iter()
            .map(|(key, value)| format!("{key}={value}"))
            .map(|value| shell_single_quote(&value))
            .collect::<Vec<_>>();
        env_args.push(shell_single_quote(&self.shell));
        env_args.push("\"$_rustploy_installer\"".to_owned());
        env_args.extend(self.arguments.iter().map(|arg| shell_single_quote(arg)));

        format!(
            "_rustploy_installer=$(mktemp -t rustploy-installer.XXXXXX)\n\
if curl -fsSL {} -o \"$_rustploy_installer\" && env {}; then\n\
    rm -f \"$_rustploy_installer\"\n\
else\n\
    _rustploy_status=$?\n\
    rm -f \"$_rustploy_installer\"\n\
    exit \"$_rustploy_status\"\n\
fi",
            shell_word(&self.url),
            env_args.join(" ")
        )
    }
}

pub struct TarballInstallerBuilder<'a> {
    executor: &'a CommandExecutor,
    url: String,
    destination: String,
    members: Vec<String>,
}

pub struct PackInstallerBuilder<'a> {
    executor: &'a CommandExecutor,
    version: String,
    destination: String,
}

impl<'a> PackInstallerBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, version: impl Into<String>) -> Self {
        Self {
            executor,
            version: version.into(),
            destination: "/usr/local/bin".to_owned(),
        }
    }

    pub fn destination(mut self, destination: impl Into<String>) -> Self {
        self.destination = destination.into();
        self
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.execute(self.executor).await
    }
}

impl<'a> TarballInstallerBuilder<'a> {
    pub(crate) fn new(
        executor: &'a CommandExecutor,
        url: impl IntoCommand,
        destination: impl IntoCommand,
    ) -> Self {
        Self {
            executor,
            url: url.build_str(),
            destination: destination.build_str(),
            members: Vec::new(),
        }
    }

    pub fn member(mut self, member: impl Into<String>) -> Self {
        self.members.push(member.into());
        self
    }

    pub async fn run(self) -> ExecResult<()> {
        let archive = self
            .executor
            .run("mktemp", ["-t", "rustploy-archive.XXXXXX"])
            .await?
            .stdout_trimmed()
            .to_owned();

        let result = async {
            self.executor
                .run("curl", ["-fsSL", self.url.as_str(), "-o", archive.as_str()])
                .await?;
            let mut args = vec![
                "-xzf".to_owned(),
                archive.clone(),
                "-C".to_owned(),
                self.destination,
                "--no-same-owner".to_owned(),
            ];
            args.extend(self.members);
            self.executor.run("tar", args).await?;
            Ok(())
        }
        .await;

        let _ = self.executor.run("rm", ["-f", archive.as_str()]).await;
        result
    }
}

impl IntoCommand for TarballInstallerBuilder<'_> {
    fn build_str(&self) -> String {
        let mut tar_args = vec![
            "-xzf".to_owned(),
            "\"$_rustploy_archive\"".to_owned(),
            "-C".to_owned(),
            shell_single_quote(&self.destination),
            "--no-same-owner".to_owned(),
        ];
        tar_args.extend(self.members.iter().map(|member| shell_single_quote(member)));

        format!(
            "_rustploy_archive=$(mktemp -t rustploy-archive.XXXXXX)\n\
if curl -fsSL {} -o \"$_rustploy_archive\" && tar {}; then\n\
    rm -f \"$_rustploy_archive\"\n\
else\n\
    _rustploy_status=$?\n\
    rm -f \"$_rustploy_archive\"\n\
    exit \"$_rustploy_status\"\n\
fi",
            shell_word(&self.url),
            tar_args.join(" ")
        )
    }
}

impl IntoCommand for PackInstallerBuilder<'_> {
    fn build_str(&self) -> String {
        let version = self.version.as_str();
        let url_prefix = format!(
            "https://github.com/buildpacks/pack/releases/download/v{version}/pack-v{version}-linux"
        );
        let tarball = TarballInstallerBuilder {
            executor: self.executor,
            url: "$_rustploy_pack_url".to_owned(),
            destination: self.destination.clone(),
            members: vec!["pack".to_owned()],
        };

        let mut steps = sh!(
            let _rustploy_pack_arch = capture_stdout! {
                cmd("uname", "-m");
            };
            let _rustploy_pack_suffix = "";
            if cmd("test", _rustploy_pack_arch, "=", "aarch64") {
                let _rustploy_pack_suffix = "-arm64";
            } else if cmd("test", _rustploy_pack_arch, "=", "arm64") {
                let _rustploy_pack_suffix = "-arm64";
            }
            let _rustploy_pack_url = word![rust!(url_prefix), _rustploy_pack_suffix, ".tgz"];
        );
        steps.push(ShellIR::Raw(tarball.build_str()));
        steps.build_str()
    }
}

fn shell_word(value: &str) -> String {
    let Some(name) = value.strip_prefix('$') else {
        return shell_single_quote(value);
    };
    let mut chars = name.chars();
    let valid_start = chars
        .next()
        .is_some_and(|ch| ch == '_' || ch.is_ascii_alphabetic());
    if valid_start && chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric()) {
        format!("\"${name}\"")
    } else {
        shell_single_quote(value)
    }
}
