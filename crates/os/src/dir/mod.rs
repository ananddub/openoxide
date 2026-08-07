use crate::exec::CommandExecutor;
use crate::exec::script::IntoCommand;

pub mod builder;
pub mod command;
pub mod copy;
pub mod create;
pub mod delete;
pub mod exists;
pub mod remove_empty;
pub mod temp;
pub mod walk;

pub use builder::DirBuilder;
pub use command::DirCommandBuilder;
pub use copy::DirCopyBuilder;
pub use create::DirCreateBuilder;
pub use delete::DirDeleteBuilder;
pub use exists::DirExistsBuilder;
pub use remove_empty::DirRemoveEmptyBuilder;
pub use temp::DirTempBuilder;
pub use walk::{DirWalkBuilder, DirWalkOutput};

pub struct DirCli<'a> {
    pub(crate) executor: &'a CommandExecutor,
}

impl<'a> DirCli<'a> {
    pub fn current(&self) -> DirCommandBuilder<'a> {
        DirCommandBuilder::new(self.executor, "pwd", vec![])
    }
    pub fn change(&self, path: impl IntoCommand) -> DirCommandBuilder<'a> {
        DirCommandBuilder::new(self.executor, "cd", vec![path.build_str()])
    }
    pub fn temp(&self) -> DirTempBuilder<'a> {
        DirTempBuilder::new(self.executor)
    }
    pub fn dir(&self, path: impl IntoCommand) -> DirBuilder<'a> {
        DirBuilder::new(self.executor, path)
    }
}

#[cfg(test)]
mod tests {
    use crate::exec::script::IntoCommand;
    use crate::exec::{CommandExecutor, LocalExecutor};
    use crate::OsCli;

    #[test]
    fn directory_copy_and_empty_remove_build_commands() {
        let executor = CommandExecutor::Local(LocalExecutor::new());
        let os = OsCli::new(&executor);

        assert_eq!(
            os.dir("/tmp/source")
                .copy_to("/tmp/target")
                .contents_only(true)
                .build_str(),
            "cp -a '/tmp/source/.' '/tmp/target'"
        );
        assert_eq!(
            os.dir("/tmp/empty").remove_if_empty().build_str(),
            "rmdir '/tmp/empty'"
        );
    }
}
