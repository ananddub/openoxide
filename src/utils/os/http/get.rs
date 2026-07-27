use crate::utils::exec::script::{IntoCommand, shell_single_quote};
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};

pub struct HttpGetBuilder<'a> {
    executor: &'a CommandExecutor,
    url: String,
    timeout_secs: u32,
    silent: bool,
}

impl<'a> HttpGetBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, url: impl IntoCommand) -> Self {
        Self {
            executor,
            url: url.build_str(),
            timeout_secs: 5,
            silent: true,
        }
    }

    pub fn timeout(mut self, secs: u32) -> Self {
        self.timeout_secs = secs;
        self
    }

    pub fn silent(mut self, silent: bool) -> Self {
        self.silent = silent;
        self
    }

    pub async fn execute(self) -> ExecResult<ExecOutput> {
        let mut args = Vec::new();
        if self.silent {
            args.push("-s".to_owned());
        }
        args.extend(["-m".to_owned(), self.timeout_secs.to_string(), self.url]);
        self.executor.run("curl", args).await
    }
}

impl IntoCommand for HttpGetBuilder<'_> {
    fn build_str(&self) -> String {
        let safe_url = shell_single_quote(&self.url);
        let silent_flag = if self.silent { "-s" } else { "" };
        format!("curl {} -m {} {}", silent_flag, self.timeout_secs, safe_url)
    }
}
