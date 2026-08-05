use crate::utils::{
    builder::{shared::BuilderContext, spec::BuilderEvent},
    exec::{CommandExecutor, ExecResult, LocalExecutor, ScriptPipeline},
    git::{client::GitCli, types::GitAuth},
};
use std::path::Path;
use tokio_util::sync::CancellationToken;

pub struct ProviderSyncBuilder<'a> {
    url: String,
    destination: &'a str,
    branch: Option<&'a str>,
    executor: Option<CommandExecutor>,
    auth: Option<GitAuth>,
    submodules: bool,
    cancel: Option<&'a CancellationToken>,
    ctx: Option<&'a BuilderContext>,
}

impl<'a> ProviderSyncBuilder<'a> {
    pub fn new(url: impl Into<String>, destination: &'a str) -> Self {
        Self {
            url: url.into(),
            destination,
            branch: None,
            executor: None,
            auth: None,
            submodules: false,
            cancel: None,
            ctx: None,
        }
    }

    pub fn branch(mut self, branch: &'a str) -> Self {
        self.branch = Some(branch);
        self
    }

    pub fn executor(mut self, executor: CommandExecutor) -> Self {
        self.executor = Some(executor);
        self
    }

    pub fn auth(mut self, auth: GitAuth) -> Self {
        self.auth = Some(auth);
        self
    }

    pub fn submodules(mut self, submodules: bool) -> Self {
        self.submodules = submodules;
        self
    }

    pub fn cancel_with(mut self, cancel: &'a CancellationToken) -> Self {
        self.cancel = Some(cancel);
        self
    }

    pub fn context(mut self, ctx: &'a BuilderContext) -> Self {
        self.ctx = Some(ctx);
        self.executor = Some(ctx.executor.clone());
        self
    }

    /// Compiles the entire deployment sync workflow into a single atomic ScriptPipeline.
    pub fn to_script(&self, branch: &str) -> String {
        let git = GitCli::new_local().with_repository(self.destination);
        let git_dir = format!("{}/.git", self.destination);

        // 1. Then branch (if .git exists)
        let mut then_branch = ScriptPipeline::new().cmd(git.remote().set_url("origin", &self.url));

        let mut fetch = git.fetch().prune().remote("origin").refspec(branch);
        if let Some(auth) = &self.auth {
            fetch = fetch.auth(auth.clone());
        }
        then_branch = then_branch
            .cmd(fetch)
            .cmd(git.reset().hard().commit("FETCH_HEAD"));

        // 2. Else branch (if .git does not exist)
        let mut else_branch = ScriptPipeline::new();
        if let Some(parent) = Path::new(self.destination).parent() {
            else_branch = else_branch.cmd(format!(
                "mkdir -p {}",
                shell_single_quote(&parent.to_string_lossy())
            ));
        }

        let clone_git = GitCli::new_local();
        let mut clone = clone_git
            .clone(&self.url)
            .destination(self.destination)
            .branch(branch)
            .single_branch();
        if let Some(auth) = &self.auth {
            clone = clone.auth(auth.clone());
        }
        else_branch = else_branch.cmd(clone);

        // 3. Main pipeline
        let mut pipeline =
            ScriptPipeline::new().if_dir_exists(&git_dir, then_branch, Some(else_branch));

        if self.submodules {
            let mut sub = git.submodule().update().init().recursive();
            if let Some(auth) = &self.auth {
                sub = sub.auth(auth.clone());
            }
            pipeline = pipeline.cmd(sub);
        }

        pipeline.compile()
    }

    pub async fn run(self) -> ExecResult<()> {
        let executor = self
            .executor
            .clone()
            .unwrap_or_else(|| CommandExecutor::Local(LocalExecutor::new()));

        let git_dir = format!("{}/.git", self.destination);

        let resolved_branch: String = match self.branch {
            Some(b) => b.to_string(),
            None => {
                let bare_git = GitCli::from_executor(executor.clone());
                bare_git
                    .queries()
                    .remote_default_branch(&self.url, self.auth.clone())
                    .await?
                    .unwrap_or_else(|| "main".into())
            }
        };
        let branch = resolved_branch.as_str();

        let dummy_cancel = CancellationToken::new();
        let cancel = self.cancel.unwrap_or(&dummy_cancel);

        // Print context messages if present
        if let Some(ctx) = self.ctx {
            if crate::utils::os::OsCli::new(&executor)
                .dir(&git_dir)
                .exists()
                .run()
                .await
                .is_ok()
            {
                ctx.emit(BuilderEvent::Message(format!(
                    "fetching source {} branch {} into {}",
                    self.url, branch, self.destination
                )))
                .await;
            } else {
                ctx.emit(BuilderEvent::Message(format!(
                    "cloning source {} branch {} into {}",
                    self.url, branch, self.destination
                )))
                .await;
            }
        }

        let script = self.to_script(branch);
        executor
            .run_with_stdin_cancelled("sh", &[] as &[&str], script, cancel)
            .await?;

        Ok(())
    }
}

fn shell_single_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_sync_builder_to_script() {
        let builder = ProviderSyncBuilder::new("https://github.com/org/repo.git", "/var/www/app")
            .submodules(true);
        let script = builder.to_script("main");

        assert!(script.contains("if [ -d '/var/www/app/.git' ]; then"));
        assert!(script.contains("git -c safe.directory=/var/www/app -C /var/www/app remote set-url origin https://github.com/org/repo.git"));
        assert!(script.contains(
            "git -c safe.directory=/var/www/app -C /var/www/app fetch --prune origin main"
        ));
        assert!(script.contains(
            "git -c safe.directory=/var/www/app -C /var/www/app reset --hard FETCH_HEAD"
        ));
        assert!(script.contains("mkdir -p '/var/www'"));
        assert!(script.contains(
            "git clone --branch main --single-branch https://github.com/org/repo.git /var/www/app"
        ));
        assert!(script.contains(
            "git -c safe.directory=/var/www/app -C /var/www/app submodule update --init --recursive"
        ));
    }
}
