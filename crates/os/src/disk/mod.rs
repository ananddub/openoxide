use crate::exec::CommandExecutor;

pub mod command;

pub use command::DiskCommandBuilder;

pub struct DiskCli<'a> {
    pub(crate) executor: &'a CommandExecutor,
}

impl<'a> DiskCli<'a> {
    pub fn list_mounts(&self) -> DiskCommandBuilder<'a> {
        DiskCommandBuilder::new(self.executor, "df", vec!["-h".to_string()])
    }
    pub fn partitions(&self) -> DiskCommandBuilder<'a> {
        DiskCommandBuilder::new(self.executor, "lsblk", vec![])
    }
}
