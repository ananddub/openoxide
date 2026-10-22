pub mod builder;
pub mod request;
pub mod stream;

pub use builder::{SocatAddress, SocatBuilder};
pub use request::{HttpMethod, SocatRequestBuilder};
pub use stream::SocatStream;

use crate::exec::CommandExecutor;

pub struct SocatCli<'a> {
    pub(crate) executor: &'a CommandExecutor,
}

impl<'a> SocatCli<'a> {
    pub fn new(executor: &'a CommandExecutor) -> Self {
        Self { executor }
    }

    /// Generic OS socat CLI command builder
    pub fn command(&self) -> SocatBuilder<'a> {
        SocatBuilder::new(self.executor)
    }

    /// Generic UNIX socket HTTP / Upgrade request builder
    pub fn request(&self, method: HttpMethod, path: impl Into<String>) -> SocatRequestBuilder {
        SocatRequestBuilder::new(method, path)
    }

    pub fn get(&self, path: impl Into<String>) -> SocatRequestBuilder {
        SocatRequestBuilder::get(path)
    }

    pub fn post(&self, path: impl Into<String>) -> SocatRequestBuilder {
        SocatRequestBuilder::post(path)
    }
}
