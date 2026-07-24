use super::{DirCreateBuilder, DirDeleteBuilder, DirExistsBuilder, DirWalkBuilder};
use crate::utils::exec::CommandExecutor;
use crate::utils::exec::script::IntoCommand;

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
    pub fn walk(self) -> DirWalkBuilder<'a> {
        DirWalkBuilder::new(self.executor, self.path)
    }
}
