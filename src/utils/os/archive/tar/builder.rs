use crate::utils::exec::CommandExecutor;
use crate::utils::exec::script::IntoCommand;

use super::TarCompression;
use super::{ArchiveCreateBuilder, ArchiveExtractBuilder, ArchiveListBuilder};

pub struct TarBuilder<'a> {
    executor: &'a CommandExecutor,
    path: String,
    compression: TarCompression,
}

impl<'a> TarBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, path: String) -> Self {
        Self {
            executor,
            path,
            compression: TarCompression::default(),
        }
    }

    pub fn compression(mut self, compression: TarCompression) -> Self {
        self.compression = compression;
        self
    }

    pub fn create(self) -> ArchiveCreateBuilder<'a> {
        ArchiveCreateBuilder::new(self.executor, self.path, self.compression)
    }

    pub fn list(self) -> ArchiveListBuilder<'a> {
        ArchiveListBuilder::new(self.executor, self.path, self.compression)
    }

    pub fn extract_to(self, destination: impl IntoCommand) -> ArchiveExtractBuilder<'a> {
        ArchiveExtractBuilder::new(self.executor, self.path, destination, self.compression)
    }
}
