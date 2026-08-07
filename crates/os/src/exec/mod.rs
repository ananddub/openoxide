pub use advertise_addr::{detect_advertise_addr, detect_advertise_addr_cancelled};
pub use arg_builder::ArgBuilder;
pub use error::{ExecError, ExecResult};
pub use exec_local::LocalExecutor;
pub use exec_remote::{RemoteExecutor, RemoteTerminal};
pub use script::{Condition, IfBuilder, IfThenBuilder, IntoCommand, ScriptPipeline};
pub use types::{
    CommandExecutor, ExecBytesOutput, ExecExitStatus, ExecOutput, ExecStreamEvent, SshAuth,
    SshHostKey,
};

pub mod pipeline {
    pub use super::script::{Condition, IfBuilder, IfThenBuilder, IntoCommand, ScriptPipeline};
}

pub mod advertise_addr;
pub mod arg_builder;
pub mod error;
pub mod exec_local;
pub mod exec_remote;
pub mod script;
pub mod types;
