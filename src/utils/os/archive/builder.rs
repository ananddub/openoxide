use crate::utils::exec::CommandExecutor;
use crate::utils::exec::script::IntoCommand;

pub struct ArchiveBuilder<'a> {
    executor: &'a CommandExecutor,
    path: String,
}

impl<'a> ArchiveBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, path: impl IntoCommand) -> Self {
        Self {
            executor,
            path: path.build_str(),
        }
    }

    pub fn tar(self) -> super::tar::TarBuilder<'a> {
        super::tar::TarBuilder::new(self.executor, self.path)
    }

    pub fn zip(self) -> super::zip::ZipBuilder<'a> {
        super::zip::ZipBuilder::new(self.executor, self.path)
    }
}
