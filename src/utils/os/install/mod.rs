use crate::utils::exec::script::{IntoCommand, shell_single_quote};
use crate::utils::exec::{CommandExecutor, ExecResult};

pub struct ShellInstallerBuilder<'a> {
    executor: &'a CommandExecutor,
    url: String,
    environment: Vec<(String, String)>,
    arguments: Vec<String>,
}

impl<'a> ShellInstallerBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, url: impl IntoCommand) -> Self {
        Self {
            executor,
            url: url.build_str(),
            environment: Vec::new(),
            arguments: Vec::new(),
        }
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
            args.extend(["sh".to_owned(), temporary.clone()]);
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
        env_args.push("sh".to_owned());
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
