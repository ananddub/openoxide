use super::{
    FileChmodBuilder, FileChownBuilder, FileDeleteBuilder, FileExistsBuilder, FileReadBuilder,
    FileReplaceBuilder, FileWriteBuilder,
};
use crate::utils::exec::CommandExecutor;
use crate::utils::exec::script::IntoCommand;

pub struct FileBuilder<'a> {
    pub(crate) executor: &'a CommandExecutor,
    pub(crate) path: String,
}

impl<'a> FileBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, path: impl IntoCommand) -> Self {
        Self {
            executor,
            path: path.build_str(),
        }
    }
    pub fn read(self) -> FileReadBuilder<'a> {
        FileReadBuilder::new(self.executor, self.path)
    }
    pub fn write(self, content: impl IntoCommand) -> FileWriteBuilder<'a> {
        FileWriteBuilder::new(self.executor, self.path, content, false)
    }
    pub fn append(self, content: impl IntoCommand) -> FileWriteBuilder<'a> {
        FileWriteBuilder::new(self.executor, self.path, content, true)
    }
    pub fn exists(self) -> FileExistsBuilder<'a> {
        FileExistsBuilder::new(self.executor, self.path)
    }
    pub fn delete(self) -> FileDeleteBuilder<'a> {
        FileDeleteBuilder::new(self.executor, self.path)
    }
    pub fn chmod(self, mode: impl IntoCommand) -> FileChmodBuilder<'a> {
        FileChmodBuilder::new(self.executor, self.path, mode)
    }
    pub fn chown(self, owner: impl IntoCommand) -> FileChownBuilder<'a> {
        FileChownBuilder::new(self.executor, self.path, owner)
    }
    pub fn replace(self, old: impl IntoCommand, new: impl IntoCommand) -> FileReplaceBuilder<'a> {
        FileReplaceBuilder::new(self.executor, self.path, old, new)
    }
}
