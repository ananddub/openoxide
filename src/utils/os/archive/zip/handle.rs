use super::{ZipCreateBuilder, ZipExtractBuilder, ZipListBuilder, ZipSanitizeBuilder};
use crate::utils::exec::CommandExecutor;
use std::path::PathBuf;

pub struct ZipBuilder<'a> {
    executor: &'a CommandExecutor,
    archive: PathBuf,
}
impl<'a> ZipBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, archive: impl Into<PathBuf>) -> Self {
        Self {
            executor,
            archive: archive.into(),
        }
    }
    pub fn create_from(self, source: impl Into<PathBuf>) -> ZipCreateBuilder<'a> {
        ZipCreateBuilder::new(self.executor, self.archive, source.into())
    }
    pub fn extract_to(self, destination: impl Into<PathBuf>) -> ZipExtractBuilder<'a> {
        ZipExtractBuilder::new(self.executor, self.archive, destination.into())
    }
    pub fn list(self) -> ZipListBuilder<'a> {
        ZipListBuilder::new(self.executor, self.archive)
    }
    pub fn sanitize(self) -> ZipSanitizeBuilder<'a> {
        ZipSanitizeBuilder::new(self.executor, self.archive)
    }
}
