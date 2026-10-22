use crate::exec::CommandExecutor;
use crate::exec::script::IntoCommand;

pub mod builder;
pub mod chmod;
pub mod chown;
pub mod copy;
pub mod delete;
pub mod exists;
pub mod move_to;
pub mod read;
pub mod replace;
pub mod write;

pub use builder::FileBuilder;
pub use chmod::FileChmodBuilder;
pub use chown::FileChownBuilder;
pub use copy::FileCopyBuilder;
pub use delete::FileDeleteBuilder;
pub use exists::FileExistsBuilder;
pub use move_to::FileMoveBuilder;
pub use read::FileReadBuilder;
pub use replace::FileReplaceBuilder;
pub use write::FileWriteBuilder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileMode {
    OwnerReadWrite,
    OwnerReadWriteExecute,
    OwnerReadWriteGroupReadWorldRead,
    OwnerAllGroupReadExecuteWorldReadExecute,
}

impl FileMode {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::OwnerReadWrite => "0600",
            Self::OwnerReadWriteExecute => "0700",
            Self::OwnerReadWriteGroupReadWorldRead => "0644",
            Self::OwnerAllGroupReadExecuteWorldReadExecute => "0755",
        }
    }
}

pub struct FileCli<'a> {
    pub(crate) executor: &'a CommandExecutor,
}

impl<'a> FileCli<'a> {
    pub fn file(&self, path: impl IntoCommand) -> FileBuilder<'a> {
        FileBuilder::new(self.executor, path)
    }
}
