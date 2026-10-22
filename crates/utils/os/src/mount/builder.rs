use super::{MountActionBuilder, MountCommandBuilder};
use crate::exec::CommandExecutor;
use crate::exec::script::IntoCommand;

pub struct MountBuilder<'a> {
    executor: &'a CommandExecutor,
    source: Option<String>,
    target: String,
}

impl<'a> MountBuilder<'a> {
    pub(crate) fn new(
        executor: &'a CommandExecutor,
        source: Option<impl IntoCommand>,
        target: impl IntoCommand,
    ) -> Self {
        Self {
            executor,
            source: source.map(|s| s.build_str()),
            target: target.build_str(),
        }
    }
    pub fn bind(self) -> MountCommandBuilder<'a> {
        MountCommandBuilder::new(
            self.executor,
            vec![
                "--bind".to_string(),
                self.source.expect("source required for mount bind"),
                self.target,
            ],
        )
    }
    pub fn unmount(self) -> MountActionBuilder<'a> {
        MountActionBuilder::new(self.executor, "umount", vec![self.target])
    }
    pub fn is_mounted(self) -> MountActionBuilder<'a> {
        MountActionBuilder::new(
            self.executor,
            "mountpoint",
            vec!["-q".to_string(), self.target],
        )
    }
}
