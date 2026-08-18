use crate::escape_arg;
use crate::exec::script::IntoCommand;
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};

pub struct ServiceActionBuilder<'a> {
    executor: &'a CommandExecutor,
    action: &'static str,
    name: String,
    now: bool,
    non_fatal: bool,
}

impl<'a> ServiceActionBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, action: &'static str, name: String) -> Self {
        Self {
            executor,
            action,
            name,
            now: false,
            non_fatal: false,
        }
    }

    pub fn now(mut self) -> Self {
        self.now = true;
        self
    }

    pub fn non_fatal(mut self) -> Self {
        self.non_fatal = true;
        self
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        let mut args = Vec::new();
        if self.now {
            args.push("--now");
        }
        args.push(self.action);
        args.push(&self.name);

        let res = self.executor.run("systemctl", &args).await;
        if self.non_fatal {
            res.or_else(|_| Ok(ExecOutput { status: crate::exec::ExecExitStatus::Remote(0), stdout: String::new(), stderr: String::new() }))
        } else {
            res
        }
    }
}

impl<'a> IntoCommand for ServiceActionBuilder<'a> {
    fn build_str(&self) -> String {
        let now_flag = if self.now { "--now " } else { "" };
        let base = format!("systemctl {} {}{}", self.action, now_flag, escape_arg(&self.name));
        if self.non_fatal {
            format!("{base} 2>/dev/null || true")
        } else {
            base
        }
    }
}
