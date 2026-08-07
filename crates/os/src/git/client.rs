use super::{GitBranch, GitStatusEntry, types::GitAuth};
use crate::exec::{
    CommandExecutor, ExecExitStatus, ExecOutput, ExecResult, ExecStreamEvent, LocalExecutor,
    RemoteExecutor, SshAuth, SshHostKey,
};
use std::{ffi::OsStr, path::PathBuf};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

#[derive(Clone, Debug)]
pub struct GitCli {
    executor: CommandExecutor,
    executable: String,
    repository: Option<String>,
}

impl Default for GitCli {
    fn default() -> Self {
        Self::new_local()
    }
}

impl GitCli {
    pub fn new_local() -> Self {
        Self {
            executor: CommandExecutor::Local(LocalExecutor::new()),
            executable: "git".into(),
            repository: None,
        }
    }
    pub fn from_executor(executor: CommandExecutor) -> Self {
        Self {
            executor,
            executable: "git".into(),
            repository: None,
        }
    }
    pub fn set_repository<S: Into<String>>(&mut self, repository: S) {
        self.repository = Some(repository.into());
    }

    pub fn new_remote(
        host: impl Into<String>,
        port: u16,
        username: impl Into<String>,
        auth: SshAuth,
        host_key: SshHostKey,
    ) -> Self {
        Self::from_remote_executor(RemoteExecutor::new(host, port, username, auth, host_key))
    }
    pub fn from_remote_executor(executor: RemoteExecutor) -> Self {
        Self {
            executor: CommandExecutor::Remote(executor),
            executable: "git".into(),
            repository: None,
        }
    }
    pub fn with_executable(mut self, executable: impl Into<PathBuf>) -> Self {
        self.executable = executable.into().to_string_lossy().into_owned();
        self
    }
    pub fn with_repository(mut self, path: impl Into<String>) -> Self {
        self.repository = Some(path.into());
        self
    }
    pub fn with_remote_sudo(mut self) -> Self {
        if let CommandExecutor::Remote(remote) = self.executor {
            self.executor = CommandExecutor::Remote(remote.with_sudo());
        }
        self
    }
    pub fn with_remote_sudo_password(mut self, password: impl Into<String>) -> Self {
        if let CommandExecutor::Remote(remote) = self.executor {
            self.executor = CommandExecutor::Remote(remote.with_sudo_password(password));
        }
        self
    }

    pub async fn run<I, S>(&self, args: I) -> ExecResult<ExecOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.executor
            .run(&self.executable, self.arguments(args))
            .await
    }
    pub async fn run_cancelled<I, S>(
        &self,
        args: I,
        cancel: &CancellationToken,
    ) -> ExecResult<ExecOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.executor
            .run_cancelled(&self.executable, self.arguments(args), cancel)
            .await
    }
    pub async fn run_with_stdin<I, S>(
        &self,
        args: I,
        stdin: impl AsRef<[u8]>,
    ) -> ExecResult<ExecOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.executor
            .run_with_stdin(&self.executable, self.arguments(args), stdin)
            .await
    }
    pub async fn run_stream<I, S>(
        &self,
        args: I,
        sender: mpsc::Sender<ExecStreamEvent>,
    ) -> ExecResult<ExecExitStatus>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.executor
            .run_stream(&self.executable, self.arguments(args), sender)
            .await
    }
    fn arguments<I, S>(&self, args: I) -> Vec<String>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut result = Vec::new();
        if let Some(repository) = &self.repository {
            result.extend(["-c".into(), format!("safe.directory={repository}")]);
            result.extend(["-C".into(), repository.clone()]);
        }
        result.extend(
            args.into_iter()
                .map(|v| v.as_ref().to_string_lossy().into_owned()),
        );
        result
    }

    pub async fn run_raw<I, S>(&self, args: I) -> ExecResult<ExecOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.executor
            .run(
                &self.executable,
                args.into_iter()
                    .map(|v| v.as_ref().to_string_lossy().into_owned())
                    .collect::<Vec<_>>(),
            )
            .await
    }

    pub async fn run_raw_cancelled<I, S>(
        &self,
        args: I,
        cancel: &CancellationToken,
    ) -> ExecResult<ExecOutput>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.executor
            .run_cancelled(
                &self.executable,
                args.into_iter()
                    .map(|v| v.as_ref().to_string_lossy().into_owned())
                    .collect::<Vec<_>>(),
                cancel,
            )
            .await
    }

    pub async fn run_raw_stream<I, S>(
        &self,
        args: I,
        sender: mpsc::Sender<ExecStreamEvent>,
    ) -> ExecResult<ExecExitStatus>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.executor
            .run_stream(
                &self.executable,
                args.into_iter()
                    .map(|v| v.as_ref().to_string_lossy().into_owned())
                    .collect::<Vec<_>>(),
                sender,
            )
            .await
    }

    pub fn compile_command(&self, args: &[String]) -> String {
        let mut full_args = Vec::new();
        if let Some(ref repository) = self.repository {
            full_args.extend(["-c".into(), format!("safe.directory={repository}")]);
            full_args.extend(["-C".into(), repository.clone()]);
        }
        full_args.extend(args.iter().cloned());
        format!("{} {}", self.executable, full_args.join(" "))
    }

    pub fn compile_command_raw(&self, args: &[String]) -> String {
        format!("{} {}", self.executable, args.join(" "))
    }

    pub fn queries(&self) -> super::handles::GitQueries<'_> {
        super::handles::GitQueries(self)
    }

    pub fn clone(&self, url: impl Into<String>) -> super::handles::CloneBuilder<'_> {
        super::handles::CloneBuilder::new(self, url)
    }

    pub fn fetch(&self) -> super::handles::FetchBuilder<'_> {
        super::handles::FetchBuilder::new(self)
    }

    pub fn pull(&self) -> super::handles::PullBuilder<'_> {
        super::handles::PullBuilder::new(self)
    }

    pub fn push(&self) -> super::handles::PushBuilder<'_> {
        super::handles::PushBuilder::new(self)
    }

    pub fn add(&self) -> super::handles::AddBuilder<'_> {
        super::handles::AddBuilder::new(self)
    }

    pub fn commit(&self) -> super::handles::CommitBuilder<'_> {
        super::handles::CommitBuilder::new(self)
    }

    pub fn checkout(&self) -> super::handles::CheckoutBuilder<'_> {
        super::handles::CheckoutBuilder::new(self)
    }

    pub fn worktree(&self) -> super::handles::WorktreeBuilder<'_> {
        super::handles::WorktreeBuilder::new(self)
    }

    pub fn remote(&self) -> super::handles::RemoteBuilder<'_> {
        super::handles::RemoteBuilder::new(self)
    }

    pub fn reset(&self) -> super::handles::ResetBuilder<'_> {
        super::handles::ResetBuilder::new(self)
    }

    pub fn submodule(&self) -> super::handles::SubmoduleBuilder<'_> {
        super::handles::SubmoduleBuilder::new(self)
    }

    // ── Legacy Methods ───────────────────────────────────────────────────────────

    pub async fn init(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["init"], args).await
    }

    // pub async fn list_branches(&self, repo &)

    pub async fn clone_repository_raw(
        &self,
        url: &str,
        destination: Option<&str>,
        options: &[&str],
    ) -> ExecResult<ExecOutput> {
        let mut args = vec!["clone"];
        args.extend_from_slice(options);
        args.push(url);
        if let Some(destination) = destination {
            args.push(destination);
        }
        self.run_raw(args).await
    }
    pub async fn clone_repository_raw_cancelled(
        &self,
        url: &str,
        destination: Option<&str>,
        options: &[&str],
        cancel: &CancellationToken,
    ) -> ExecResult<ExecOutput> {
        let mut args = vec!["clone"];
        args.extend_from_slice(options);
        args.push(url);
        if let Some(destination) = destination {
            args.push(destination);
        }
        self.run_raw_cancelled(args, cancel).await
    }
    pub async fn clone_repository_raw_stream(
        &self,
        url: &str,
        destination: Option<&str>,
        options: &[&str],
        sender: mpsc::Sender<ExecStreamEvent>,
    ) -> ExecResult<ExecExitStatus> {
        let mut args = vec!["clone"];
        args.extend_from_slice(options);
        args.push(url);
        if let Some(destination) = destination {
            args.push(destination);
        }
        self.run_raw_stream(args, sender).await
    }
    pub async fn fetch_raw(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["fetch"], args).await
    }
    pub async fn fetch_raw_cancelled(
        &self,
        args: &[&str],
        cancel: &CancellationToken,
    ) -> ExecResult<ExecOutput> {
        let mut command = vec!["fetch"];
        command.extend_from_slice(args);
        self.run_cancelled(command, cancel).await
    }
    pub async fn fetch_raw_stream(
        &self,
        args: &[&str],
        sender: mpsc::Sender<ExecStreamEvent>,
    ) -> ExecResult<ExecExitStatus> {
        let mut command = vec!["fetch"];
        command.extend_from_slice(args);
        self.run_stream(command, sender).await
    }
    pub async fn pull_raw(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["pull"], args).await
    }
    pub async fn pull_raw_stream(
        &self,
        args: &[&str],
        sender: mpsc::Sender<ExecStreamEvent>,
    ) -> ExecResult<ExecExitStatus> {
        let mut command = vec!["pull"];
        command.extend_from_slice(args);
        self.run_stream(command, sender).await
    }
    pub async fn push_raw(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["push"], args).await
    }
    pub async fn push_raw_stream(
        &self,
        args: &[&str],
        sender: mpsc::Sender<ExecStreamEvent>,
    ) -> ExecResult<ExecExitStatus> {
        let mut command = vec!["push"];
        command.extend_from_slice(args);
        self.run_stream(command, sender).await
    }
    pub async fn checkout_raw(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["checkout"], args).await
    }
    pub async fn switch_raw(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["switch"], args).await
    }
    pub async fn add_raw(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["add"], args).await
    }
    pub async fn commit_raw(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["commit"], args).await
    }
    pub async fn merge(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["merge"], args).await
    }
    pub async fn rebase(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["rebase"], args).await
    }
    pub async fn reset_raw(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["reset"], args).await
    }
    pub async fn restore(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["restore"], args).await
    }
    pub async fn clean(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["clean"], args).await
    }
    pub async fn tag(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["tag"], args).await
    }
    pub async fn remote_raw(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["remote"], args).await
    }
    pub async fn config(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["config"], args).await
    }
    pub async fn submodule_raw(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["submodule"], args).await
    }
    pub async fn worktree_raw(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["worktree"], args).await
    }
    pub async fn stash(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["stash"], args).await
    }
    pub async fn log(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["log"], args).await
    }
    pub async fn diff(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["diff"], args).await
    }
    pub async fn show(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["show"], args).await
    }
    pub async fn rev_parse(&self, revision: &str) -> ExecResult<String> {
        Ok(self
            .run(["rev-parse", revision])
            .await?
            .stdout
            .trim()
            .into())
    }
    pub async fn ls_remote(&self, args: &[&str]) -> ExecResult<ExecOutput> {
        self.prefixed(&["ls-remote"], args).await
    }
    pub async fn status(&self) -> ExecResult<Vec<GitStatusEntry>> {
        let output = self.run(["status", "--porcelain=v1", "-z"]).await?;
        let mut records = output
            .stdout
            .split('\0')
            .filter(|record| !record.is_empty());
        let mut result = Vec::new();
        while let Some(record) = records.next() {
            if let Some(mut entry) = GitStatusEntry::parse(record) {
                if matches!(entry.index_status, 'R' | 'C')
                    || matches!(entry.worktree_status, 'R' | 'C')
                {
                    entry.original_path = records.next().map(str::to_owned);
                }
                result.push(entry);
            }
        }
        Ok(result)
    }
    pub async fn branches(&self) -> ExecResult<Vec<GitBranch>> {
        Ok(self
            .run(["branch", "--format=%(HEAD)%00%(refname:short)%00"])
            .await?
            .stdout
            .lines()
            .filter_map(|line| {
                let mut p = line.split('\0');
                Some(GitBranch {
                    current: p.next()? == "*",
                    name: p.next()?.into(),
                })
            })
            .collect())
    }
    pub async fn remote_branches(
        &self,
        repository_url: &str,
        auth: Option<GitAuth>,
    ) -> ExecResult<Vec<GitBranch>> {
        self.queries().remote_branches(repository_url, auth).await
    }
    pub async fn remote_default_branch(
        &self,
        repository_url: &str,
        auth: Option<GitAuth>,
    ) -> ExecResult<Option<String>> {
        self.queries()
            .remote_default_branch(repository_url, auth)
            .await
    }
    pub async fn remote_default_branch_cancelled(
        &self,
        repository_url: &str,
        auth: Option<GitAuth>,
        cancel: &CancellationToken,
    ) -> ExecResult<Option<String>> {
        self.queries()
            .remote_default_branch_cancelled(repository_url, auth, cancel)
            .await
    }
    async fn prefixed(&self, prefix: &[&str], args: &[&str]) -> ExecResult<ExecOutput> {
        let mut command = prefix.to_vec();
        command.extend_from_slice(args);
        self.run(command).await
    }
}

#[cfg(test)]
mod tests {
    //     use super::*;
    //
    //     #[tokio::test]
    //     async fn local_repository_status_commit_and_revision_are_typed() {
    //         let dir = tempfile::tempdir().unwrap();
    //         let path = dir.path().to_string_lossy().into_owned();
    //         let git = GitCli::new_local().with_repository(path);
    //         git.init(&[]).await.unwrap();
    //         git.config(&["user.name", "Rustploy Test"]).await.unwrap();
    //         git.config(&["user.email", "test@rustploy.local"])
    //             .await
    //             .unwrap();
    //         std::fs::write(dir.path().join("hello.txt"), "hello").unwrap();
    //         let status = git.status().await.unwrap();
    //         assert_eq!(status.len(), 1);
    //         assert_eq!(status[0].path, "hello.txt");
    //         assert_eq!(status[0].index_status, '?');
    //         git.add(&["hello.txt"]).await.unwrap();
    //         git.commit(&["-m", "initial"]).await.unwrap();
    //         let revision = git.rev_parse("HEAD").await.unwrap();
    //         assert_eq!(revision.len(), 40);
    //         assert!(git.status().await.unwrap().is_empty());
    //     }
    //
    //     #[test]
    //     fn parses_remote_branch_names_from_ls_remote_heads() {
    //         let output = "\
    // 1111111111111111111111111111111111111111\trefs/heads/main
    // 2222222222222222222222222222222222222222\trefs/heads/feature/login
    // 3333333333333333333333333333333333333333\trefs/tags/v1.0.0
    // ";
    //         let branches = parse_remote_branches(output).unwrap();
    //         assert_eq!(
    //             branches,
    //             vec![
    //                 GitBranch {
    //                     name: "feature/login".into(),
    //                     current: false,
    //                 },
    //                 GitBranch {
    //                     name: "main".into(),
    //                     current: false,
    //                 },
    //             ]
    //         );
    //     }
    //
    //     #[test]
    //     fn parses_remote_default_branch_from_symref_head() {
    //         let output = "\
    // ref: refs/heads/master\tHEAD
    // 1111111111111111111111111111111111111111\tHEAD
    // ";
    //         assert_eq!(
    //             parse_remote_default_branch(output),
    //             Some("master".into())
    //         );
    //     }
}
