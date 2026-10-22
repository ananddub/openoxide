mod configure;
mod query;
mod types;

use crate::exec::CommandExecutor;

pub use configure::NvidiaConfigureBuilder;
pub use query::NvidiaQueryBuilder;
pub use types::{ContainerRuntime, NvidiaQueryField, NvidiaQueryFormat};

pub struct NvidiaGpuBuilder<'a> {
    executor: &'a CommandExecutor,
}

impl<'a> NvidiaGpuBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor) -> Self {
        Self { executor }
    }

    pub fn query(
        self,
        fields: impl IntoIterator<Item = NvidiaQueryField>,
    ) -> NvidiaQueryBuilder<'a> {
        NvidiaQueryBuilder::new(self.executor).fields(fields)
    }

    pub fn configure(self) -> NvidiaConfigureBuilder<'a> {
        NvidiaConfigureBuilder::new(self.executor)
    }
}
