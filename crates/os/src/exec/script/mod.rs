pub use cmd::IntoCommand;
pub use condition::Condition;
pub use dsl::ShellIR;
pub use pipeline::{IfBuilder, IfThenBuilder, ScriptPipeline};
pub use sh_macros::sh;

pub fn shell_single_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

pub mod cmd;
pub mod condition;
pub mod dsl;
pub mod macros;
pub mod pipeline;
#[cfg(test)]
pub mod tests;
