use super::action::ProcessActionBuilder;
use crate::exec::CommandExecutor;
use crate::exec::script::IntoCommand;

pub struct ProcessBuilder<'a> {
    executor: &'a CommandExecutor,
    pid_or_name: String,
}

impl<'a> ProcessBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, pid_or_name: impl IntoCommand) -> Self {
        Self {
            executor,
            pid_or_name: pid_or_name.build_str(),
        }
    }
    pub fn kill(self) -> ProcessActionBuilder<'a> {
        ProcessActionBuilder::new(
            self.executor,
            vec!["kill".to_string(), "-9".to_string(), self.pid_or_name],
        )
    }
    pub fn stop(self) -> ProcessActionBuilder<'a> {
        ProcessActionBuilder::new(
            self.executor,
            vec!["kill".to_string(), "-STOP".to_string(), self.pid_or_name],
        )
    }
    pub fn resume(self) -> ProcessActionBuilder<'a> {
        ProcessActionBuilder::new(
            self.executor,
            vec!["kill".to_string(), "-CONT".to_string(), self.pid_or_name],
        )
    }
    pub fn status(self) -> ProcessActionBuilder<'a> {
        ProcessActionBuilder::new(
            self.executor,
            vec!["ps".to_string(), "-p".to_string(), self.pid_or_name],
        )
    }
    pub fn priority(self) -> ProcessActionBuilder<'a> {
        ProcessActionBuilder::new(
            self.executor,
            vec![
                "ps".into(),
                "-o".into(),
                "ni=".into(),
                "-p".into(),
                self.pid_or_name,
            ],
        )
    }
    pub fn set_priority(self, val: impl IntoCommand) -> ProcessActionBuilder<'a> {
        ProcessActionBuilder::new(
            self.executor,
            vec![
                "renice".to_string(),
                val.build_str(),
                "-p".to_string(),
                self.pid_or_name,
            ],
        )
    }
}

pub struct PkillBuilder<'a> {
    executor: &'a CommandExecutor,
    signal: Option<String>,
    full_match: bool,
    exact_match: bool,
    user: Option<String>,
    pattern: Option<String>,
    env_var: Option<(String, String)>,
}

impl<'a> PkillBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor) -> Self {
        Self {
            executor,
            signal: None,
            full_match: true,
            exact_match: false,
            user: None,
            pattern: None,
            env_var: None,
        }
    }

    pub fn signal(mut self, sig: impl Into<String>) -> Self {
        self.signal = Some(sig.into());
        self
    }

    pub fn sigkill(mut self) -> Self {
        self.signal = Some("9".to_string());
        self
    }

    pub fn sigterm(mut self) -> Self {
        self.signal = Some("15".to_string());
        self
    }

    pub fn full_match(mut self, enabled: bool) -> Self {
        self.full_match = enabled;
        self
    }

    pub fn exact_match(mut self, enabled: bool) -> Self {
        self.exact_match = enabled;
        self
    }

    pub fn user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }

    pub fn env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.env_var = Some((key.into(), value.into()));
        self
    }

    pub fn pattern(mut self, pattern: impl Into<String>) -> Self {
        self.pattern = Some(pattern.into());
        self
    }

    pub async fn run(self) -> crate::exec::ExecResult<crate::exec::ExecOutput> {
        let mut args = Vec::new();
        if let Some(sig) = &self.signal {
            args.push(format!("-{}", sig));
        }
        if self.full_match {
            args.push("-f".to_string());
        }
        if self.exact_match {
            args.push("-x".to_string());
        }
        if let Some(u) = &self.user {
            args.push("-u".to_string());
            args.push(u.clone());
        }
        if let Some((k, v)) = &self.env_var {
            args.push(format!("{}={}", k, v));
        } else if let Some(p) = &self.pattern {
            args.push(p.clone());
        }

        self.executor.run("pkill", &args).await
    }
}
