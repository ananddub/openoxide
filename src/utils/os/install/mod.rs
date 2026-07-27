use crate::utils::exec::script::IntoCommand;
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
