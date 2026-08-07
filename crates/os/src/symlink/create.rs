use crate::exec::script::IntoCommand;
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::escape_arg;

pub struct SymlinkCreateBuilder<'a> {
    executor: &'a CommandExecutor,
    target: String,
    link: String,
    symbolic: bool,
    force: bool,
    no_dereference: bool,
    verbose: bool,
}

impl<'a> SymlinkCreateBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, target: String, link: String) -> Self {
        Self {
            executor,
            target,
            link,
            symbolic: true,
            force: true,
            no_dereference: false,
            verbose: false,
        }
    }
    pub fn symbolic(mut self, val: bool) -> Self {
        self.symbolic = val;
        self
    }
    pub fn force(mut self, val: bool) -> Self {
        self.force = val;
        self
    }
    pub fn no_dereference(mut self, val: bool) -> Self {
        self.no_dereference = val;
        self
    }
    pub fn verbose(mut self, val: bool) -> Self {
        self.verbose = val;
        self
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        let mut args = Vec::new();
        let mut flags = String::new();
        if self.symbolic {
            flags.push('s');
        }
        if self.force {
            flags.push('f');
        }
        if self.no_dereference {
            flags.push('n');
        }
        if self.verbose {
            flags.push('v');
        }
        if !flags.is_empty() {
            args.push(format!("-{}", flags));
        }
        args.push(self.target);
        args.push(self.link);
        self.executor.run("ln", &args).await
    }
}

impl<'a> IntoCommand for SymlinkCreateBuilder<'a> {
    fn build_str(&self) -> String {
        let mut parts = vec!["ln".to_string()];
        let mut flags = String::new();
        if self.symbolic {
            flags.push('s');
        }
        if self.force {
            flags.push('f');
        }
        if self.no_dereference {
            flags.push('n');
        }
        if self.verbose {
            flags.push('v');
        }
        if !flags.is_empty() {
            parts.push(format!("-{}", flags));
        }
        parts.push(escape_arg(&self.target));
        parts.push(escape_arg(&self.link));
        parts.join(" ")
    }
}
