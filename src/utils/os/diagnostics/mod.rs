mod server;
mod types;
use crate::utils::exec::CommandExecutor;
pub use server::{ServerDiagnosticsBuilder, ServerDiagnosticsOutput};
pub use types::DiagnosticScope;

pub struct DiagnosticsCli<'a> {
    pub(crate) executor: &'a CommandExecutor,
}

impl<'a> DiagnosticsCli<'a> {
    pub fn server(&self) -> ServerDiagnosticsBuilder<'a> {
        ServerDiagnosticsBuilder::new(self.executor)
    }
}
