use super::{IntoCommand, shell_single_quote};
use std::ops::{BitAnd, BitOr, Not};

#[derive(Clone, Debug)]
pub enum Condition {
    DirExists(String),
    FileExists(String),
    CmdSucceeds(String),
    EnvSet(String),
    Not(Box<Condition>),
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
}

impl Condition {
    pub fn dir_exists(path: impl Into<String>) -> Self {
        Condition::DirExists(path.into())
    }
    pub fn file_exists(path: impl Into<String>) -> Self {
        Condition::FileExists(path.into())
    }
    pub fn cmd_succeeds(cmd: impl IntoCommand) -> Self {
        Condition::CmdSucceeds(cmd.build_str())
    }
    pub fn env_set(key: impl Into<String>) -> Self {
        Condition::EnvSet(key.into())
    }

    // ── Method-chaining API (existing) ────────────────────────
    pub fn not(self) -> Self {
        Condition::Not(Box::new(self))
    }
    pub fn and(self, other: Condition) -> Self {
        Condition::And(Box::new(self), Box::new(other))
    }
    pub fn or(self, other: Condition) -> Self {
        Condition::Or(Box::new(self), Box::new(other))
    }

    fn to_bash_inner(&self, parent: Option<&str>) -> String {
        match self {
            Condition::DirExists(p) => format!("[ -d {} ]", shell_single_quote(p)),
            Condition::FileExists(p) => format!("[ -f {} ]", shell_single_quote(p)),
            Condition::CmdSucceeds(c) => c.clone(),
            Condition::EnvSet(k) => format!("[ -n \"${{{}}}\" ]", k),
            Condition::Not(c) => format!("! {}", c.to_bash_inner(Some("not"))),
            Condition::And(a, b) => {
                let s = format!(
                    "{} && {}",
                    a.to_bash_inner(Some("and")),
                    b.to_bash_inner(Some("and")),
                );
                // Wrap And in parens when inside Not or Or (for clarity)
                if matches!(parent, Some("not") | Some("or")) {
                    format!("({})", s)
                } else {
                    s
                }
            }
            Condition::Or(a, b) => {
                let s = format!(
                    "{} || {}",
                    a.to_bash_inner(Some("or")),
                    b.to_bash_inner(Some("or")),
                );
                // Wrap Or in parens when inside And or Not
                if matches!(parent, Some("and") | Some("not")) {
                    format!("({})", s)
                } else {
                    s
                }
            }
        }
    }

    pub fn to_bash(&self) -> String {
        self.to_bash_inner(None)
    }
}

impl IntoCommand for Condition {
    fn build_str(&self) -> String {
        self.to_bash()
    }
}

// ── Operator overloading — owned values ───────────────────────

impl BitAnd for Condition {
    type Output = Condition;
    fn bitand(self, rhs: Condition) -> Condition {
        self.and(rhs)
    }
}

impl BitOr for Condition {
    type Output = Condition;
    fn bitor(self, rhs: Condition) -> Condition {
        self.or(rhs)
    }
}

impl Not for Condition {
    type Output = Condition;
    fn not(self) -> Condition {
        Condition::not(self)
    }
}

// ── Operator overloading — reference values (clone() likhne se bachne ke liye) ──

impl BitAnd for &Condition {
    type Output = Condition;
    fn bitand(self, rhs: &Condition) -> Condition {
        self.clone().and(rhs.clone())
    }
}

impl BitOr for &Condition {
    type Output = Condition;
    fn bitor(self, rhs: &Condition) -> Condition {
        self.clone().or(rhs.clone())
    }
}

impl Not for &Condition {
    type Output = Condition;
    fn not(self) -> Condition {
        Condition::not(self.clone())
    }
}

impl std::ops::BitAndAssign<Condition> for Condition {
    fn bitand_assign(&mut self, rhs: Condition) {
        let old = std::mem::replace(self, Condition::EnvSet(String::new()));
        *self = old.and(rhs);
    }
}

impl std::ops::BitOrAssign<Condition> for Condition {
    fn bitor_assign(&mut self, rhs: Condition) {
        let old = std::mem::replace(self, Condition::EnvSet(String::new()));
        *self = old.or(rhs);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_and_method_chain() {
        let c = Condition::dir_exists("/tmp").and(Condition::file_exists("/tmp/f"));
        assert_eq!(c.to_bash(), "[ -d '/tmp' ] && [ -f '/tmp/f' ]");
    }

    #[test]
    fn simple_and_operator() {
        let c = Condition::dir_exists("/tmp") & Condition::file_exists("/tmp/f");
        assert_eq!(c.to_bash(), "[ -d '/tmp' ] && [ -f '/tmp/f' ]");
    }

    #[test]
    fn or_with_and_precedence_method_chain() {
        let c = Condition::dir_exists("/a")
            .and(Condition::file_exists("/b"))
            .or(Condition::env_set("X"));
        assert_eq!(
            c.to_bash(),
            "([ -d '/a' ] && [ -f '/b' ]) || [ -n \"${X}\" ]"
        );
    }

    #[test]
    fn or_with_and_precedence_operator() {
        // Rust precedence: & binds tighter than | — same as bash's && vs ||
        let c =
            (Condition::dir_exists("/a") & Condition::file_exists("/b")) | Condition::env_set("X");
        let bash = c.to_bash();
        assert_eq!(bash, "([ -d '/a' ] && [ -f '/b' ]) || [ -n \"${X}\" ]");
    }

    #[test]
    fn not_wraps_and() {
        let c = !(Condition::dir_exists("/a") & Condition::file_exists("/b"));
        assert_eq!(c.to_bash(), "! ([ -d '/a' ] && [ -f '/b' ])");
    }

    #[test]
    fn assign_operators() {
        let mut c = Condition::dir_exists("/a");
        c &= Condition::file_exists("/b");
        assert_eq!(c.to_bash(), "[ -d '/a' ] && [ -f '/b' ]");
    }

    #[test]
    fn cmd_succeeds_from_into_command() {
        let c = Condition::cmd_succeeds("git status");
        assert_eq!(c.to_bash(), "git status");
    }
}
