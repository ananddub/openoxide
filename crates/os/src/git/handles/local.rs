use crate::utils::{
    exec::{ArgBuilder, ExecOutput, ExecResult},
    git::{client::GitCli, types::GitAuth},
};

pub struct AddBuilder<'a> {
    cli: &'a GitCli,
    args: ArgBuilder,
}

impl<'a> AddBuilder<'a> {
    pub(crate) fn new(cli: &'a GitCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["add"]),
        }
    }

    pub fn all(mut self) -> Self {
        self.args.flag("--all");
        self
    }
    pub fn update(mut self) -> Self {
        self.args.flag("--update");
        self
    }
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.args.push(path.into());
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
}

pub struct CommitBuilder<'a> {
    cli: &'a GitCli,
    args: ArgBuilder,
}

impl<'a> CommitBuilder<'a> {
    pub(crate) fn new(cli: &'a GitCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["commit"]),
        }
    }

    pub fn message(mut self, msg: impl Into<String>) -> Self {
        self.args.pair("-m", msg.into());
        self
    }
    pub fn all(mut self) -> Self {
        self.args.flag("--all");
        self
    }
    pub fn amend(mut self) -> Self {
        self.args.flag("--amend");
        self
    }
    pub fn no_verify(mut self) -> Self {
        self.args.flag("--no-verify");
        self
    }
    pub fn no_edit(mut self) -> Self {
        self.args.flag("--no-edit");
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
}

pub struct CheckoutBuilder<'a> {
    cli: &'a GitCli,
    args: ArgBuilder,
}

impl<'a> CheckoutBuilder<'a> {
    pub(crate) fn new(cli: &'a GitCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["checkout"]),
        }
    }

    pub fn branch(mut self, branch: impl Into<String>) -> Self {
        self.args.push(branch.into());
        self
    }
    pub fn create(mut self) -> Self {
        self.args.flag("-b");
        self
    }
    pub fn force(mut self) -> Self {
        self.args.flag("--force");
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
}

pub struct WorktreeBuilder<'a> {
    cli: &'a GitCli,
}

impl<'a> WorktreeBuilder<'a> {
    pub(crate) fn new(cli: &'a GitCli) -> Self {
        Self { cli }
    }

    pub fn add(
        &self,
        path: impl Into<String>,
        branch: impl Into<String>,
    ) -> WorktreeAddBuilder<'a> {
        WorktreeAddBuilder {
            cli: self.cli,
            args: ArgBuilder::cmd(&["worktree", "add"]),
            path: path.into(),
            branch: branch.into(),
        }
    }
}

pub struct WorktreeAddBuilder<'a> {
    cli: &'a GitCli,
    args: ArgBuilder,
    path: String,
    branch: String,
}

impl<'a> WorktreeAddBuilder<'a> {
    pub fn force(mut self) -> Self {
        self.args.flag("--force");
        self
    }
    pub fn create_branch(mut self) -> Self {
        self.args.flag("-b");
        self
    }
    pub fn detach(mut self) -> Self {
        self.args.flag("--detach");
        self
    }

    pub fn print(&self) -> String {
        let mut a = self.args.clone();
        a.push(&self.path);
        a.push(&self.branch);
        a.preview()
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        let mut a = self.args;
        a.push(&self.path);
        a.push(&self.branch);
        let built = a.build();
        let refs: Vec<&str> = built.iter().map(String::as_str).collect();
        self.cli.run(&refs).await
    }
}

pub struct RemoteBuilder<'a> {
    cli: &'a GitCli,
    args: ArgBuilder,
}

impl<'a> RemoteBuilder<'a> {
    pub(crate) fn new(cli: &'a GitCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["remote"]),
        }
    }

    pub fn set_url(mut self, name: impl Into<String>, url: impl Into<String>) -> Self {
        self.args.push("set-url");
        self.args.push(name.into());
        self.args.push(url.into());
        self
    }

    pub fn add(mut self, name: impl Into<String>, url: impl Into<String>) -> Self {
        self.args.push("add");
        self.args.push(name.into());
        self.args.push(url.into());
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
}

pub struct ResetBuilder<'a> {
    cli: &'a GitCli,
    args: ArgBuilder,
}

impl<'a> ResetBuilder<'a> {
    pub(crate) fn new(cli: &'a GitCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["reset"]),
        }
    }

    pub fn hard(mut self) -> Self {
        self.args.flag("--hard");
        self
    }
    pub fn soft(mut self) -> Self {
        self.args.flag("--soft");
        self
    }
    pub fn mixed(mut self) -> Self {
        self.args.flag("--mixed");
        self
    }
    pub fn commit(mut self, commit: impl Into<String>) -> Self {
        self.args.push(commit.into());
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
}

pub struct SubmoduleBuilder<'a> {
    cli: &'a GitCli,
    args: ArgBuilder,
}

impl<'a> SubmoduleBuilder<'a> {
    pub(crate) fn new(cli: &'a GitCli) -> Self {
        Self {
            cli,
            args: ArgBuilder::cmd(&["submodule"]),
        }
    }

    pub fn update(mut self) -> Self {
        self.args.push("update");
        self
    }
    pub fn init(mut self) -> Self {
        self.args.flag("--init");
        self
    }
    pub fn recursive(mut self) -> Self {
        self.args.flag("--recursive");
        self
    }
    pub fn auth(mut self, auth: GitAuth) -> Self {
        let (k, v) = auth.to_config();
        self.args.insert_pair(0, "-c", format!("{}={}", k, v));
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
}

impl crate::exec::pipeline::IntoCommand for AddBuilder<'_> {
    fn build_str(&self) -> String {
        self.cli.compile_command(&self.args.clone().build())
    }
}

impl crate::exec::pipeline::IntoCommand for CommitBuilder<'_> {
    fn build_str(&self) -> String {
        self.cli.compile_command(&self.args.clone().build())
    }
}

impl crate::exec::pipeline::IntoCommand for CheckoutBuilder<'_> {
    fn build_str(&self) -> String {
        self.cli.compile_command(&self.args.clone().build())
    }
}

impl crate::exec::pipeline::IntoCommand for WorktreeAddBuilder<'_> {
    fn build_str(&self) -> String {
        let mut a = self.args.clone();
        a.push(&self.path);
        a.push(&self.branch);
        self.cli.compile_command(&a.build())
    }
}

impl crate::exec::pipeline::IntoCommand for RemoteBuilder<'_> {
    fn build_str(&self) -> String {
        self.cli.compile_command(&self.args.clone().build())
    }
}

impl crate::exec::pipeline::IntoCommand for ResetBuilder<'_> {
    fn build_str(&self) -> String {
        self.cli.compile_command(&self.args.clone().build())
    }
}

impl crate::exec::pipeline::IntoCommand for SubmoduleBuilder<'_> {
    fn build_str(&self) -> String {
        self.cli.compile_command(&self.args.clone().build())
    }
}
