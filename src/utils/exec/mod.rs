pub use arg_builder::ArgBuilder;
pub use error::{ExecError, ExecResult};
pub use exec_local::LocalExecutor;
pub use exec_remote::{RemoteExecutor, RemoteTerminal};
pub use script::{Condition, IfBuilder, IfThenBuilder, IntoCommand, ScriptPipeline};
pub use types::{
    CommandExecutor, ExecExitStatus, ExecOutput, ExecStreamEvent, SshAuth, SshHostKey,
};

pub mod pipeline {
    pub use super::script::{Condition, IfBuilder, IfThenBuilder, IntoCommand, ScriptPipeline};
}

pub mod arg_builder;
pub mod error;
pub mod exec_local;
pub mod exec_remote;
pub mod script;
pub mod types;
