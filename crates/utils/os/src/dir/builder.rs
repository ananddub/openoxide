use super::{
    DirCopyBuilder, DirCreateBuilder, DirDeleteBuilder, DirExistsBuilder, DirRemoveEmptyBuilder,
    DirWalkBuilder,
};
use crate::exec::CommandExecutor;
use crate::exec::script::IntoCommand;

pub struct DirBuilder<'a> {
    pub(crate) executor: &'a CommandExecutor,
    pub(crate) path: String,
}

impl<'a> DirBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, path: impl IntoCommand) -> Self {
        Self {
            executor,
            path: path.build_str(),
        }
    }
    pub fn create(self) -> DirCreateBuilder<'a> {
        DirCreateBuilder::new(self.executor, self.path)
    }
    pub fn exists(self) -> DirExistsBuilder<'a> {
        DirExistsBuilder::new(self.executor, self.path)
    }
    pub fn delete(self) -> DirDeleteBuilder<'a> {
        DirDeleteBuilder::new(self.executor, self.path)
    }
    pub fn copy_to(self, target: impl IntoCommand) -> DirCopyBuilder<'a> {
        DirCopyBuilder::new(self.executor, self.path, target)
    }
    pub fn walk(self) -> DirWalkBuilder<'a> {
        DirWalkBuilder::new(self.executor, self.path)
    }
    pub fn remove_if_empty(self) -> DirRemoveEmptyBuilder<'a> {
        DirRemoveEmptyBuilder::new(self.executor, self.path)
    }
}
