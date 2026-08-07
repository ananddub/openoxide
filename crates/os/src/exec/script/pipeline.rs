use super::{Condition, IntoCommand, shell_single_quote};
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::rclone::RcloneBuilder;
use tokio_util::sync::CancellationToken;

/// Represents a single step in the execution pipeline.
#[derive(Clone, Debug)]
pub enum PipelineStep {
    /// A standalone command or builder that runs sequentially (on a new line).
    Sequential(String),
    /// Runs only if the previous command succeeds (&&).
    And(String),
    /// Runs only if the previous command fails (||).
    Or(String),
    /// Pipes stdout of the previous command to stdin of this command (|).
    Pipe(String),
    /// A conditional if-else block.
    IfElse {
        condition: String,
        then_branch: ScriptPipeline,
        else_branch: Option<ScriptPipeline>,
    },
}

/// A pipeline for executing multiple commands in a single shell invocation.
#[derive(Default, Clone, Debug)]
pub struct ScriptPipeline {
    working_dir: Option<String>,
    env_vars: Vec<(String, String)>,
    steps: Vec<PipelineStep>,
    trace: bool,
    verbose_headers: bool,
}

impl ScriptPipeline {
    /// Creates a new empty `ScriptPipeline`.
    pub fn new() -> Self {
        Self {
            working_dir: None,
            env_vars: Vec::new(),
            steps: Vec::new(),
            trace: false,
            verbose_headers: false,
        }
    }

    /// Sets the pipeline-wide working directory.
    pub fn working_dir(mut self, dir: impl Into<String>) -> Self {
        self.working_dir = Some(dir.into());
        self
    }

    /// Adds a pipeline-wide environment variable.
    pub fn env(mut self, key: impl Into<String>, val: impl Into<String>) -> Self {
        self.env_vars.push((key.into(), val.into()));
        self
    }

    /// Enables or disables command execution tracing (`set -x`).
    pub fn trace(mut self, enabled: bool) -> Self {
        self.trace = enabled;
        self
    }

    /// Enables or disables verbose headers printed before sequential steps.
    pub fn verbose_headers(mut self, enabled: bool) -> Self {
        self.verbose_headers = enabled;
        self
    }

    /// Adds a command or builder that runs sequentially (on a new line).
    pub fn cmd(mut self, cmd: impl IntoCommand) -> Self {
        self.steps.push(PipelineStep::Sequential(cmd.build_str()));
        self
    }

    /// Appends multiple sequential commands or builders to the pipeline.
    pub fn extend_cmds(mut self, cmds: impl IntoIterator<Item = impl IntoCommand>) -> Self {
        for cmd in cmds {
            self = self.cmd(cmd);
        }
        self
    }

    /// Runs this command only if the previous step succeeded (&&).
    pub fn and(mut self, cmd: impl IntoCommand) -> Self {
        self.steps.push(PipelineStep::And(cmd.build_str()));
        self
    }

    /// Runs this command only if the previous step failed (||).
    pub fn or(mut self, cmd: impl IntoCommand) -> Self {
        self.steps.push(PipelineStep::Or(cmd.build_str()));
        self
    }

    /// Pipes stdout of the previous command to stdin of this command (|).
    pub fn pipe(mut self, cmd: impl IntoCommand) -> Self {
        self.steps.push(PipelineStep::Pipe(cmd.build_str()));
        self
    }

    /// Adds a conditional if/else block to the pipeline.
    pub fn if_else(
        mut self,
        condition: impl IntoCommand,
        then_branch: ScriptPipeline,
        else_branch: Option<ScriptPipeline>,
    ) -> Self {
        self.steps.push(PipelineStep::IfElse {
            condition: condition.build_str(),
            then_branch,
            else_branch,
        });
        self
    }

    /// Runs then_branch if the given path is a directory.
    pub fn if_dir_exists(
        self,
        path: impl AsRef<str>,
        then_branch: ScriptPipeline,
        else_branch: Option<ScriptPipeline>,
    ) -> Self {
        let cond = format!("[ -d {} ]", shell_single_quote(path.as_ref()));
        self.if_else(cond, then_branch, else_branch)
    }

    /// Runs then_branch if the given path is a regular file.
    pub fn if_file_exists(
        self,
        path: impl AsRef<str>,
        then_branch: ScriptPipeline,
        else_branch: Option<ScriptPipeline>,
    ) -> Self {
        let cond = format!("[ -f {} ]", shell_single_quote(path.as_ref()));
        self.if_else(cond, then_branch, else_branch)
    }

    /// Runs then_branch if the given command exits 0.
    pub fn if_cmd_succeeds(
        self,
        cmd: impl IntoCommand,
        then_branch: ScriptPipeline,
        else_branch: Option<ScriptPipeline>,
    ) -> Self {
        self.if_else(cmd.build_str(), then_branch, else_branch)
    }

    /// Runs then_branch if the given env var is set (non-empty).
    pub fn if_env_set(
        self,
        key: impl AsRef<str>,
        then_branch: ScriptPipeline,
        else_branch: Option<ScriptPipeline>,
    ) -> Self {
        let cond = format!("[ -n \"${{{}}}\" ]", key.as_ref());
        self.if_else(cond, then_branch, else_branch)
    }

    /// Compatibility alias to add an `RcloneBuilder`.
    pub fn rclone(self, builder: RcloneBuilder) -> Self {
        self.cmd(builder)
    }

    /// Adds a conditional block using fluent chaining.
    pub fn if_condition(self, cond: Condition) -> IfBuilder {
        IfBuilder {
            condition: cond,
            parent: self,
        }
    }
}

pub struct IfBuilder {
    condition: Condition,
    parent: ScriptPipeline,
}

impl IfBuilder {
    pub fn then<F>(self, build: F) -> IfThenBuilder
    where
        F: FnOnce(ScriptPipeline) -> ScriptPipeline,
    {
        let then_branch = build(ScriptPipeline::new());
        IfThenBuilder {
            condition: self.condition,
            then_branch,
            parent: self.parent,
        }
    }
}

pub struct IfThenBuilder {
    condition: Condition,
    then_branch: ScriptPipeline,
    parent: ScriptPipeline,
}

impl IfThenBuilder {
    pub fn otherwise<F>(self, build: F) -> ScriptPipeline
    where
        F: FnOnce(ScriptPipeline) -> ScriptPipeline,
    {
        let else_branch = build(ScriptPipeline::new());
        self.parent
            .if_else(self.condition, self.then_branch, Some(else_branch))
    }

    pub fn end_if(self) -> ScriptPipeline {
        self.parent.if_else(self.condition, self.then_branch, None)
    }
}

impl ScriptPipeline {
    /// Compiles all commands and settings in the pipeline into a single POSIX-compliant shell script.
    pub fn compile(&self) -> String {
        let mut script = String::new();
        script.push_str("set -e\n");
        if self.trace {
            script.push_str("set -x\n");
        }
        script.push_str(&self.compile_inner());
        script.push('\n');
        script
    }

    /// Helper method to recursively compile inner steps.
    pub fn compile_inner(&self) -> String {
        let mut script = String::new();
        for (k, v) in &self.env_vars {
            script.push_str(&format!("export {}={}\n", k, shell_single_quote(v)));
        }
        if let Some(ref dir) = self.working_dir {
            script.push_str(&format!("cd {}\n", shell_single_quote(dir)));
        }

        for (i, step) in self.steps.iter().enumerate() {
            match step {
                PipelineStep::Sequential(cmd) => {
                    if i > 0 {
                        script.push('\n');
                    }
                    if self.verbose_headers {
                        script.push_str(&format!("echo '=== [Step {}] ==='\n", i + 1));
                    }
                    script.push_str(cmd);
                }
                PipelineStep::And(cmd) => {
                    script.push_str(" && ");
                    script.push_str(cmd);
                }
                PipelineStep::Or(cmd) => {
                    script.push_str(" || ");
                    script.push_str(cmd);
                }
                PipelineStep::Pipe(cmd) => {
                    script.push_str(" | ");
                    script.push_str(cmd);
                }
                PipelineStep::IfElse {
                    condition,
                    then_branch,
                    else_branch,
                } => {
                    if i > 0 {
                        script.push('\n');
                    }
                    script.push_str(&format!("if {}; then\n", condition));
                    let then_compiled = then_branch.compile_inner();
                    if then_compiled.trim().is_empty() {
                        script.push_str("    :\n");
                    } else {
                        for line in then_compiled.lines() {
                            script.push_str(&format!("    {}\n", line));
                        }
                    }
                    if let Some(eb) = else_branch {
                        script.push_str("else\n");
                        let else_compiled = eb.compile_inner();
                        if else_compiled.trim().is_empty() {
                            script.push_str("    :\n");
                        } else {
                            for line in else_compiled.lines() {
                                script.push_str(&format!("    {}\n", line));
                            }
                        }
                    }
                    script.push_str("fi");
                }
            }
        }
        script
    }

    /// Executes the entire pipeline using the provided executor by piping the compiled script into `sh`.
    pub async fn execute(&self, executor: &CommandExecutor) -> ExecResult<ExecOutput> {
        let script = self.compile();
        executor.run_with_stdin("sh", &[] as &[&str], script).await
    }

    /// Executes the entire pipeline with cancellation support.
    pub async fn execute_cancelled(
        &self,
        executor: &CommandExecutor,
        cancel: &CancellationToken,
    ) -> ExecResult<ExecOutput> {
        let script = self.compile();
        executor
            .run_with_stdin_cancelled("sh", &[] as &[&str], script, cancel)
            .await
    }
}
