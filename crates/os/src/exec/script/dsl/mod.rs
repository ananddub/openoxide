use crate::exec::script::{IntoCommand, shell_single_quote};

#[derive(Debug, Clone)]
pub enum ShellIR {
    Raw(String),
    Expr(Expr),
    Statement(Statement),
    Pipeline(Vec<Command>),
    Sequence(Vec<ShellIR>),
    Redirect {
        cmd: Box<ShellIR>,
        target: String,
        append: bool,
        fd: RedirectionFd,
    },
    Capture {
        cmd: Box<ShellIR>,
        source: CaptureSource,
    },
    Command(Command),
    If {
        cond: Box<ShellIR>,
        then_branch: Vec<ShellIR>,
        else_branch: Option<Vec<ShellIR>>,
    },
    Loop {
        var: String,
        iterator: Box<ShellIR>,
        body: Vec<ShellIR>,
    },
    While {
        cond: Box<ShellIR>,
        body: Vec<ShellIR>,
    },
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<ShellIR>,
    },
    Defer {
        body: Vec<ShellIR>,
    },
    Parallel {
        body: Vec<ShellIR>,
    },
    Retry {
        count: Box<ShellIR>,
        body: Vec<ShellIR>,
    },
    CaptureBlock(Vec<ShellIR>),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(String),
    Variable(String),
    EnvVar(String),
    Glob(String),
    Array(Vec<Expr>),
    Word(Vec<Expr>),
}

#[derive(Debug, Clone)]
pub enum Statement {
    VarAssign {
        name: String,
        val: Box<ShellIR>,
        default: Option<String>,
    },
    Echo(Box<ShellIR>),
}

#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub args: Vec<ArgToken>,
}

#[derive(Debug, Clone)]
pub enum ArgToken {
    Literal(String),
    NullDevice,
    Variable(String),
    EnvVar(String),
    Glob(String),
    Rendered(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedirectionFd {
    Stdout,
    Stderr,
    Both,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputTarget {
    Null,
    StandardError,
    File(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemDevice {
    Null,
    StandardError,
}

impl SystemDevice {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Null => "/dev/null",
            Self::StandardError => "/dev/stderr",
        }
    }
}

impl OutputTarget {
    pub fn file(path: impl Into<String>) -> Self {
        Self::File(path.into())
    }

    fn render(self) -> String {
        match self {
            Self::Null => SystemDevice::Null.as_str().into(),
            Self::StandardError => SystemDevice::StandardError.as_str().into(),
            Self::File(path) => shell_single_quote(&path),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureSource {
    Stdout,
    Stderr,
    Both,
    Status,
}

impl ShellIR {
    pub fn to_bash(&self) -> String {
        match self {
            ShellIR::Raw(s) => s.clone(),
            ShellIR::Command(cmd) => cmd.to_bash(),
            ShellIR::Pipeline(cmds) => cmds
                .iter()
                .map(|c| c.to_bash())
                .collect::<Vec<_>>()
                .join(" | "),
            ShellIR::Sequence(commands) => commands
                .iter()
                .map(ShellIR::to_bash)
                .collect::<Vec<_>>()
                .join("\n"),
            ShellIR::Redirect {
                cmd,
                target,
                append,
                fd,
            } => {
                let op = if *append { ">>" } else { ">" };
                let fd_prefix = match fd {
                    RedirectionFd::Stdout => "",
                    RedirectionFd::Stderr => "2",
                    RedirectionFd::Both => "&",
                };
                format!("{} {}{} {}", cmd.to_bash(), fd_prefix, op, target)
            }
            ShellIR::Capture { cmd, source } => match source {
                CaptureSource::Status => {
                    format!(
                        "$(if {}; then echo true; else echo false; fi)",
                        cmd.to_bash()
                    )
                }
                _ => format!("$({})", cmd.to_bash()),
            },
            ShellIR::If {
                cond,
                then_branch,
                else_branch,
            } => {
                let cond_str = cond.to_bash();
                let then_str = then_branch
                    .iter()
                    .map(|s| s.to_bash())
                    .collect::<Vec<_>>()
                    .join("\n");
                let mut out = format!("if {}; then\n{}", cond_str, indent(&then_str));
                if let Some(eb) = else_branch {
                    let else_str = eb
                        .iter()
                        .map(|s| s.to_bash())
                        .collect::<Vec<_>>()
                        .join("\n");
                    out.push_str(&format!("\nelse\n{}", indent(&else_str)));
                }
                out.push_str("\nfi");
                out
            }
            ShellIR::Loop {
                var,
                iterator,
                body,
            } => {
                let iter_str = match &**iterator {
                    ShellIR::Expr(Expr::Variable(v)) => format!("\"${{{}[@]}}\"", v),
                    _ => iterator.to_bash(),
                };
                let body_str = body
                    .iter()
                    .map(|s| s.to_bash())
                    .collect::<Vec<_>>()
                    .join("\n");
                format!(
                    "for {} in {}; do\n{}\ndone",
                    var,
                    iter_str,
                    indent(&body_str)
                )
            }
            ShellIR::While { cond, body } => {
                let body = body
                    .iter()
                    .map(ShellIR::to_bash)
                    .collect::<Vec<_>>()
                    .join("\n");
                format!("while {}; do\n{}\ndone", cond.to_bash(), indent(&body))
            }
            ShellIR::Function { name, params, body } => {
                let body_str = body
                    .iter()
                    .map(|s| s.to_bash())
                    .collect::<Vec<_>>()
                    .join("\n");
                let mut inside = String::new();
                for (i, param) in params.iter().enumerate() {
                    inside.push_str(&format!("local {}=\"${}\"\n", param, i + 1));
                }
                inside.push_str(&body_str);
                format!("{}() {{\n{}\n}}", name, indent(&inside))
            }
            ShellIR::Defer { body } => {
                let body_str = body
                    .iter()
                    .map(|s| s.to_bash())
                    .collect::<Vec<_>>()
                    .join("\n");
                format!(
                    "_cleanup() {{\n{}\n}}\ntrap _cleanup EXIT",
                    indent(&body_str)
                )
            }
            ShellIR::Parallel { body } => {
                let body_str = body
                    .iter()
                    .map(|s| format!("{} &", s.to_bash()))
                    .collect::<Vec<_>>()
                    .join("\n");
                format!("{}\nwait", body_str)
            }
            ShellIR::Retry { count, body } => {
                let body_str = body
                    .iter()
                    .map(|s| s.to_bash())
                    .collect::<Vec<_>>()
                    .join("\n");
                let count_str = count.to_bash();
                format!(
                    "for i in $(seq 1 {}); do\n{}\n    && break || sleep 1\ndone",
                    count_str,
                    indent(&body_str)
                )
            }
            ShellIR::CaptureBlock(body) => {
                let body_str = body
                    .iter()
                    .map(|s| s.to_bash())
                    .collect::<Vec<_>>()
                    .join("\n");
                format!("(\n{}\n)", indent(&body_str))
            }
            ShellIR::Expr(expr) => expr.to_bash(),
            ShellIR::Statement(stmt) => stmt.to_bash(),
        }
    }

    // Fluent redirect builders
    pub fn stdout(self, target: OutputTarget) -> Self {
        ShellIR::Redirect {
            cmd: Box::new(self),
            target: target.render(),
            append: false,
            fd: RedirectionFd::Stdout,
        }
    }

    pub fn stderr(self, target: OutputTarget) -> Self {
        ShellIR::Redirect {
            cmd: Box::new(self),
            target: target.render(),
            append: false,
            fd: RedirectionFd::Stderr,
        }
    }

    pub fn stdout_null(self) -> Self {
        self.stdout(OutputTarget::Null)
    }

    pub fn stderr_null(self) -> Self {
        self.stderr(OutputTarget::Null)
    }

    pub fn stderr_console(self) -> Self {
        self.stderr(OutputTarget::StandardError)
    }

    pub fn append(self, file: impl Into<String>) -> Self {
        ShellIR::Redirect {
            cmd: Box::new(self),
            target: file.into(),
            append: true,
            fd: RedirectionFd::Stdout,
        }
    }

    pub fn redirect_to(self, target: ArgToken) -> Self {
        ShellIR::Redirect {
            cmd: Box::new(self),
            target: target.to_bash(),
            append: false,
            fd: RedirectionFd::Stdout,
        }
    }

    pub fn append_to(self, target: ArgToken) -> Self {
        ShellIR::Redirect {
            cmd: Box::new(self),
            target: target.to_bash(),
            append: true,
            fd: RedirectionFd::Stdout,
        }
    }

    pub fn sudo(self) -> Self {
        match self {
            ShellIR::Raw(s) => ShellIR::Raw(format!("sudo {}", s)),
            ShellIR::Command(cmd) => {
                let mut args = vec![ArgToken::Literal(cmd.name)];
                for arg in cmd.args {
                    args.push(arg);
                }
                ShellIR::Command(Command {
                    name: "sudo".to_string(),
                    args,
                })
            }
            other => other,
        }
    }

    pub fn success(self) -> Self {
        match self {
            ShellIR::Expr(Expr::Variable(v)) => ShellIR::Raw(format!(
                "[ \"${{_status:-}}\" = \"true\" ] 2>/dev/null || [ \"${{{}_status:-}}\" = \"true\" ] || [ \"${{{}:-}}\" = \"true\" ]",
                v, v
            )),
            ShellIR::Raw(s) => {
                if s.starts_with('$') {
                    let clean = s
                        .trim_start_matches('$')
                        .trim_matches('"')
                        .trim_matches('{')
                        .trim_matches('}');
                    ShellIR::Raw(format!(
                        "[ \"${{{}_status:-}}\" = \"true\" ] || [ \"{}\" = \"true\" ]",
                        clean, s
                    ))
                } else {
                    ShellIR::Raw(s)
                }
            }
            other => other,
        }
    }

    pub fn ok(self) -> Self {
        self.success()
    }

    pub fn failure(self) -> Self {
        match self {
            ShellIR::Expr(Expr::Variable(v)) => ShellIR::Raw(format!(
                "[ \"${{_status:-}}\" = \"false\" ] 2>/dev/null || [ \"${{{}_status:-}}\" = \"false\" ] || [ \"${{{}:-}}\" = \"false\" ]",
                v, v
            )),
            ShellIR::Raw(s) => {
                if s.starts_with('$') {
                    let clean = s
                        .trim_start_matches('$')
                        .trim_matches('"')
                        .trim_matches('{')
                        .trim_matches('}');
                    ShellIR::Raw(format!(
                        "[ \"${{{}_status:-}}\" = \"false\" ] || [ \"{}\" = \"false\" ]",
                        clean, s
                    ))
                } else {
                    ShellIR::Raw(format!("! {}", s))
                }
            }
            other => {
                let bash_str = other.to_bash();
                ShellIR::Raw(format!("! {}", bash_str))
            }
        }
    }

    pub fn empty(self) -> Self {
        match self {
            ShellIR::Expr(Expr::Variable(v)) => ShellIR::Raw(format!("[ -z \"${{{v}:-}}\" ]")),
            ShellIR::Raw(s) => ShellIR::Raw(format!("[ -z \"{}\" ]", s)),
            other => ShellIR::Raw(format!("[ -z \"{}\" ]", other.to_bash())),
        }
    }

    pub fn non_empty(self) -> Self {
        match self {
            ShellIR::Expr(Expr::Variable(v)) => ShellIR::Raw(format!("[ -n \"${{{v}:-}}\" ]")),
            ShellIR::Raw(s) => ShellIR::Raw(format!("[ -n \"{}\" ]", s)),
            other => ShellIR::Raw(format!("[ -n \"{}\" ]", other.to_bash())),
        }
    }
}

impl Expr {
    pub fn to_bash(&self) -> String {
        match self {
            Expr::Literal(lit) => shell_single_quote(lit),
            Expr::Variable(var) => format!("\"${}\"", var),
            Expr::EnvVar(env) => format!("\"${}\"", env),
            Expr::Glob(glob) => glob.clone(),
            Expr::Array(arr) => {
                let elements = arr
                    .iter()
                    .map(|e| e.to_bash())
                    .collect::<Vec<_>>()
                    .join(" ");
                format!("({})", elements)
            }
            Expr::Word(parts) => {
                let content = parts
                    .iter()
                    .map(|part| match part {
                        Expr::Literal(value) => value
                            .replace('\\', "\\\\")
                            .replace('"', "\\\"")
                            .replace('$', "\\$")
                            .replace('`', "\\`"),
                        Expr::Variable(name) => format!("${{{name}}}"),
                        Expr::EnvVar(name) => format!("${{{name}}}"),
                        Expr::Glob(value) => value.clone(),
                        Expr::Array(_) | Expr::Word(_) => part.to_bash(),
                    })
                    .collect::<String>();
                format!("\"{content}\"")
            }
        }
    }
}

impl Statement {
    pub fn to_bash(&self) -> String {
        match self {
            Statement::VarAssign { name, val, default } => match &**val {
                ShellIR::CaptureBlock(body) => {
                    let body_str = body
                        .iter()
                        .map(|s| s.to_bash())
                        .collect::<Vec<_>>()
                        .join("\n");
                    let indent_body = indent(&body_str);
                    format!(
                        "{name}_stdout_file=$(mktemp)\n\
                             {name}_stderr_file=$(mktemp)\n\
                             if (\n\
                             {indent_body}\n\
                             ) > \"${name}_stdout_file\" 2> \"${name}_stderr_file\"; then\n\
                             {name}_status=true\n\
                             else\n\
                             {name}_status=false\n\
                             fi\n\
                             {name}_stdout=$(cat \"${name}_stdout_file\")\n\
                             {name}_stderr=$(cat \"${name}_stderr_file\")\n\
                             rm -f \"${name}_stdout_file\" \"${name}_stderr_file\"",
                        name = name,
                        indent_body = indent_body
                    )
                }
                _ => {
                    let mut assign = format!("{}={}", name, val.to_bash());
                    if let Some(def) = default {
                        assign.push_str(&format!(
                            "\n[ -z \"${}\" ] && {}={}",
                            name,
                            name,
                            shell_single_quote(def)
                        ));
                    }
                    assign
                }
            },
            Statement::Echo(val) => {
                format!("echo {}", val.to_bash())
            }
        }
    }
}

impl Command {
    pub fn to_bash(&self) -> String {
        let mut parts = vec![self.name.clone()];
        for arg in &self.args {
            parts.push(arg.to_bash());
        }
        parts.join(" ")
    }
}

impl ArgToken {
    pub fn dynamic(value: impl Into<String>) -> Self {
        let value = value.into();
        if let Some(name) = value.strip_prefix('$') {
            let mut chars = name.chars();
            let valid_start = chars
                .next()
                .is_some_and(|ch| ch == '_' || ch.is_ascii_alphabetic());
            if valid_start && chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric()) {
                return Self::Variable(name.to_owned());
            }
        }
        Self::Literal(value)
    }

    pub fn to_bash(&self) -> String {
        match self {
            ArgToken::Literal(lit) => shell_single_quote(lit),
            ArgToken::NullDevice => SystemDevice::Null.as_str().into(),
            ArgToken::Variable(var) => format!("\"${}\"", var),
            ArgToken::EnvVar(env) => format!("\"${}\"", env),
            ArgToken::Glob(glob) => glob.clone(),
            ArgToken::Rendered(value) => value.clone(),
        }
    }
}

impl IntoCommand for ShellIR {
    fn build_str(&self) -> String {
        self.to_bash()
    }
}

impl IntoCommand for Vec<ShellIR> {
    fn build_str(&self) -> String {
        self.iter()
            .map(|s| s.to_bash())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn indent(s: &str) -> String {
    s.lines()
        .map(|line| {
            if line.is_empty() {
                String::new()
            } else {
                format!("    {}", line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub struct ShellIRWrapper<T>(pub T);

impl ShellIRWrapper<ShellIR> {
    pub fn into_shell_ir(self) -> ShellIR {
        self.0
    }
}

pub trait IntoShellIRGeneric {
    fn into_shell_ir(self) -> ShellIR;
}

impl<T: IntoCommand> IntoShellIRGeneric for ShellIRWrapper<T> {
    fn into_shell_ir(self) -> ShellIR {
        ShellIR::Raw(self.0.build_str())
    }
}
