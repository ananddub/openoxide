use crate::exec::script::{IntoCommand, shell_single_quote};
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};

pub struct HttpGetBuilder<'a> {
    executor: &'a CommandExecutor,
    url: String,
    timeout_secs: u32,
    connect_timeout_secs: Option<u32>,
    silent: bool,
    ipv4: bool,
    ipv6: bool,
    fail_ok: bool,
}

impl<'a> HttpGetBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, url: impl IntoCommand) -> Self {
        Self {
            executor,
            url: url.build_str(),
            timeout_secs: 5,
            connect_timeout_secs: None,
            silent: true,
            ipv4: false,
            ipv6: false,
            fail_ok: false,
        }
    }

    pub fn timeout(mut self, secs: u32) -> Self {
        self.timeout_secs = secs;
        self
    }

    pub fn connect_timeout(mut self, secs: u32) -> Self {
        self.connect_timeout_secs = Some(secs);
        self
    }

    pub fn ipv4(mut self) -> Self {
        self.ipv4 = true;
        self.ipv6 = false;
        self
    }

    pub fn ipv6(mut self) -> Self {
        self.ipv6 = true;
        self.ipv4 = false;
        self
    }

    pub fn silent(mut self, silent: bool) -> Self {
        self.silent = silent;
        self
    }

    pub fn fail_ok(mut self) -> Self {
        self.fail_ok = true;
        self
    }

    pub async fn execute(self) -> ExecResult<ExecOutput> {
        if self.fail_ok {
            let command = self.build_str();
            return self.executor.run("sh", ["-c", command.as_str()]).await;
        }

        let mut args = Vec::new();
        if self.ipv4 {
            args.push("-4".to_owned());
        }
        if self.ipv6 {
            args.push("-6".to_owned());
        }
        if self.silent {
            args.push("-s".to_owned());
        }
        if let Some(connect_timeout) = self.connect_timeout_secs {
            args.extend(["--connect-timeout".to_owned(), connect_timeout.to_string()]);
        }
        args.extend(["-m".to_owned(), self.timeout_secs.to_string(), self.url]);
        self.executor.run("curl", args).await
    }
}

impl IntoCommand for HttpGetBuilder<'_> {
    fn build_str(&self) -> String {
        let safe_url = shell_single_quote(&self.url);
        let mut args = Vec::new();
        if self.ipv4 {
            args.push("-4".to_owned());
        }
        if self.ipv6 {
            args.push("-6".to_owned());
        }
        if self.silent {
            args.push("-s".to_owned());
        }
        if let Some(connect_timeout) = self.connect_timeout_secs {
            args.push("--connect-timeout".to_owned());
            args.push(connect_timeout.to_string());
        }
        args.push("-m".to_owned());
        args.push(self.timeout_secs.to_string());
        args.push(safe_url);

        let command = format!("curl {}", args.join(" "));
        if self.fail_ok {
            format!("{command} || true")
        } else {
            command
        }
    }
}
