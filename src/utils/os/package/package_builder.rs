use super::{PackageCheckInstalledBuilder, PackageInstallBuilder, PackageRemoveBuilder};
use crate::utils::exec::CommandExecutor;
use crate::utils::exec::script::IntoCommand;

pub struct PackageBuilder<'a> {
    pub(crate) executor: &'a CommandExecutor,
    pub(crate) name: String,
}

impl<'a> PackageBuilder<'a> {
    pub fn new(executor: &'a CommandExecutor, name: impl IntoCommand) -> Self {
        Self {
            executor,
            name: name.build_str(),
        }
    }
    pub fn install(self) -> PackageInstallBuilder<'a> {
        PackageInstallBuilder::new(self.executor, self.name)
    }
    pub fn remove(self) -> PackageRemoveBuilder<'a> {
        PackageRemoveBuilder::new(self.executor, self.name)
    }
    pub fn installed(self) -> PackageCheckInstalledBuilder<'a> {
        PackageCheckInstalledBuilder::new(self.executor, self.name)
    }
}
