use crate::exec::CommandExecutor;
use crate::exec::script::IntoCommand;

pub mod builder;
pub mod create;
pub mod delete;
pub mod exists;
pub mod points_to;

pub use builder::SymlinkBuilder;
pub use create::SymlinkCreateBuilder;
pub use delete::SymlinkDeleteBuilder;
pub use exists::SymlinkExistsBuilder;
pub use points_to::SymlinkPointsToBuilder;

pub struct SymlinkCli<'a> {
    pub(crate) executor: &'a CommandExecutor,
}

impl<'a> SymlinkCli<'a> {
    pub fn symlink(&self, target: impl IntoCommand, link: impl IntoCommand) -> SymlinkBuilder<'a> {
        SymlinkBuilder::new(self.executor, Some(target.build_str()), link.build_str())
    }

    pub fn symlink_ref(&self, link: impl IntoCommand) -> SymlinkBuilder<'a> {
        SymlinkBuilder::new(self.executor, None, link.build_str())
    }
}
