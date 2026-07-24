use crate::utils::{
    exec::{ArgBuilder, ExecExitStatus, ExecOutput, ExecResult, ExecStreamEvent},
    git::{client::GitCli, types::GitAuth},
};
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

pub struct CloneBuilder<'a> {
    cli: &'a GitCli,
    url: String,
    args: ArgBuilder,
    destination: Option<String>,
}

impl<'a> CloneBuilder<'a> {
    pub(crate) fn new(cli: &'a GitCli, url: impl Into<String>) -> Self {
        Self {
            cli,
            url: url.into(),
            args: ArgBuilder::cmd(&["clone"]),
            destination: None,
        }
    }

    pub fn destination(mut self, path: impl Into<String>) -> Self {
        self.destination = Some(path.into());
        self
    }
    pub fn auth(mut self, auth: GitAuth) -> Self {
        let (k, v) = auth.to_config();
        self.args.insert_pair(0, "-c", format!("{}={}", k, v));
        self
    }
    pub fn branch(mut self, name: impl Into<String>) -> Self {
        self.args.pair("--branch", name.into());
        self
    }
    pub fn depth(mut self, n: u32) -> Self {
        self.args.pair("--depth", n.to_string());
        self
    }
    pub fn single_branch(mut self) -> Self {
        self.args.flag("--single-branch");
        self
    }
    pub fn no_tags(mut self) -> Self {
        self.args.flag("--no-tags");
        self
    }
    pub fn bare(mut self) -> Self {
        self.args.flag("--bare");
        self
    }
    pub fn mirror(mut self) -> Self {
        self.args.flag("--mirror");
        self
    }
    pub fn recurse_submodules(mut self) -> Self {
        self.args.flag("--recurse-submodules");
        self
    }
    pub fn quiet(mut self) -> Self {
        self.args.flag("--quiet");
        self
    }
    pub fn arg(mut self, v: impl Into<String>) -> Self {
        self.args.push(v.into());
        self
    }

    fn finalize(&self) -> ArgBuilder {
        let mut a = self.args.clone();
        a.push(&self.url);
        if let Some(dest) = &self.destination {
            a.push(dest);
        }
        a
    }

    pub fn print(&self) -> String {
        self.finalize().preview()
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        let built = self.finalize().build();
        let refs: Vec<&str> = built.iter().map(String::as_str).collect();
        // Do not use run() directly because clone doesn't need -c safe.directory prefix usually,
        // but GitCli's run method adds it if repository is set. For clone, it should be fine.
        self.cli.run_raw(&refs).await
    }

    pub async fn run_cancelled(self, cancel: &CancellationToken) -> ExecResult<ExecOutput> {
        let built = self.finalize().build();
        let refs: Vec<&str> = built.iter().map(String::as_str).collect();
        self.cli.run_raw_cancelled(&refs, cancel).await
    }

    pub async fn stream(self, sender: mpsc::Sender<ExecStreamEvent>) -> ExecResult<ExecExitStatus> {
        let built = self.finalize().build();
        let refs: Vec<&str> = built.iter().map(String::as_str).collect();
        self.cli.run_raw_stream(&refs, sender).await
    }
}

pub struct FetchBuilder<'a> {
    cli: &'a GitCli,
    args: ArgBuilder,
}

impl<'a> FetchBuilder<'a> {
    pub(crate) fn new(cli: &'a GitCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["fetch"]),
        }
    }

    pub fn remote(mut self, remote: impl Into<String>) -> Self {
        self.args.push(remote.into());
        self
    }
    pub fn auth(mut self, auth: GitAuth) -> Self {
        let (k, v) = auth.to_config();
        self.args.insert_pair(0, "-c", format!("{}={}", k, v));
        self
    }
    pub fn all(mut self) -> Self {
        self.args.flag("--all");
        self
    }
    pub fn prune(mut self) -> Self {
        self.args.flag("--prune");
        self
    }
    pub fn tags(mut self) -> Self {
        self.args.flag("--tags");
        self
    }
    pub fn no_tags(mut self) -> Self {
        self.args.flag("--no-tags");
        self
    }
    pub fn depth(mut self, n: u32) -> Self {
        self.args.pair("--depth", n.to_string());
        self
    }
    pub fn arg(mut self, v: impl Into<String>) -> Self {
        self.args.push(v.into());
        self
    }

    pub fn print(&self) -> String {
        self.args.preview()
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        let built = self.args.build();
        let refs: Vec<&str> = built.iter().map(String::as_str).collect();
        self.cli.run(&refs).await
    }

    pub async fn run_cancelled(self, cancel: &CancellationToken) -> ExecResult<ExecOutput> {
        let built = self.args.build();
        let refs: Vec<&str> = built.iter().map(String::as_str).collect();
        self.cli.run_cancelled(&refs, cancel).await
    }

    pub async fn stream(self, sender: mpsc::Sender<ExecStreamEvent>) -> ExecResult<ExecExitStatus> {
        let built = self.args.build();
        let refs: Vec<&str> = built.iter().map(String::as_str).collect();
        self.cli.run_stream(&refs, sender).await
    }
}

pub struct PullBuilder<'a> {
    cli: &'a GitCli,
    args: ArgBuilder,
}

impl<'a> PullBuilder<'a> {
    pub(crate) fn new(cli: &'a GitCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["pull"]),
        }
    }

    pub fn remote(mut self, remote: impl Into<String>) -> Self {
        self.args.push(remote.into());
        self
    }
    pub fn branch(mut self, name: impl Into<String>) -> Self {
        self.args.push(name.into());
        self
    }
    pub fn auth(mut self, auth: GitAuth) -> Self {
        let (k, v) = auth.to_config();
        self.args.insert_pair(0, "-c", format!("{}={}", k, v));
        self
    }
    pub fn rebase(mut self) -> Self {
        self.args.flag("--rebase");
        self
    }
    pub fn no_rebase(mut self) -> Self {
        self.args.flag("--no-rebase");
        self
    }
    pub fn ff_only(mut self) -> Self {
        self.args.flag("--ff-only");
        self
    }
    pub fn arg(mut self, v: impl Into<String>) -> Self {
        self.args.push(v.into());
        self
    }

    pub fn print(&self) -> String {
        self.args.preview()
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        let built = self.args.build();
        let refs: Vec<&str> = built.iter().map(String::as_str).collect();
        self.cli.run(&refs).await
    }

    pub async fn stream(self, sender: mpsc::Sender<ExecStreamEvent>) -> ExecResult<ExecExitStatus> {
        let built = self.args.build();
        let refs: Vec<&str> = built.iter().map(String::as_str).collect();
        self.cli.run_stream(&refs, sender).await
    }
}

pub struct PushBuilder<'a> {
    cli: &'a GitCli,
    args: ArgBuilder,
}

impl<'a> PushBuilder<'a> {
    pub(crate) fn new(cli: &'a GitCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["push"]),
        }
    }

    pub fn remote(mut self, remote: impl Into<String>) -> Self {
        self.args.push(remote.into());
        self
    }
    pub fn branch(mut self, name: impl Into<String>) -> Self {
        self.args.push(name.into());
        self
    }
    pub fn force(mut self) -> Self {
        self.args.flag("--force");
        self
    }
    pub fn force_with_lease(mut self) -> Self {
        self.args.flag("--force-with-lease");
        self
    }
    pub fn tags(mut self) -> Self {
        self.args.flag("--tags");
        self
    }
    pub fn set_upstream(mut self) -> Self {
        self.args.flag("--set-upstream");
        self
    }
    pub fn arg(mut self, v: impl Into<String>) -> Self {
        self.args.push(v.into());
        self
    }

    pub fn print(&self) -> String {
        self.args.preview()
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        let built = self.args.build();
        let refs: Vec<&str> = built.iter().map(String::as_str).collect();
        self.cli.run(&refs).await
    }

    pub async fn stream(self, sender: mpsc::Sender<ExecStreamEvent>) -> ExecResult<ExecExitStatus> {
        let built = self.args.build();
        let refs: Vec<&str> = built.iter().map(String::as_str).collect();
        self.cli.run_stream(&refs, sender).await
    }
}

impl crate::utils::exec::pipeline::IntoCommand for CloneBuilder<'_> {
    fn build_str(&self) -> String {
        self.cli.compile_command_raw(&self.finalize().build())
    }
}

impl crate::utils::exec::pipeline::IntoCommand for FetchBuilder<'_> {
    fn build_str(&self) -> String {
        self.cli.compile_command(&self.args.clone().build())
    }
}

impl crate::utils::exec::pipeline::IntoCommand for PullBuilder<'_> {
    fn build_str(&self) -> String {
        self.cli.compile_command(&self.args.clone().build())
    }
}

impl crate::utils::exec::pipeline::IntoCommand for PushBuilder<'_> {
    fn build_str(&self) -> String {
        self.cli.compile_command(&self.args.clone().build())
    }
}
