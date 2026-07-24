use super::{
    SymlinkCreateBuilder, SymlinkDeleteBuilder, SymlinkExistsBuilder, SymlinkPointsToBuilder,
};
use crate::utils::exec::CommandExecutor;

pub struct SymlinkBuilder<'a> {
    pub(crate) executor: &'a CommandExecutor,
    pub(crate) target: Option<String>,
    pub(crate) link: String,
}

impl<'a> SymlinkBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, target: Option<String>, link: String) -> Self {
        Self {
            executor,
            target,
            link,
        }
    }
    pub fn create(self) -> SymlinkCreateBuilder<'a> {
        SymlinkCreateBuilder::new(
            self.executor,
            self.target.expect("target required for symlink create"),
            self.link,
        )
    }
    pub fn exists(self) -> SymlinkExistsBuilder<'a> {
        SymlinkExistsBuilder::new(self.executor, self.link)
    }
    pub fn points_to(self) -> SymlinkPointsToBuilder<'a> {
        SymlinkPointsToBuilder::new(self.executor, self.link)
    }
    pub fn delete(self) -> SymlinkDeleteBuilder<'a> {
        SymlinkDeleteBuilder::new(self.executor, self.link)
    }
}
