mod dig;
mod types;

use std::net::IpAddr;

use crate::utils::exec::CommandExecutor;

pub use dig::{DigBuilder, DigResponse};
pub use types::{
    DigAddressFamily, DigEdnsBufferSize, DigEdnsVersion, DigPort, DigQueryClass, DigRecordType,
    DigSection, DigTimeout, DigTransport, DigTries, DnsName,
};

pub struct DnsCli<'a> {
    pub(crate) executor: &'a CommandExecutor,
}

impl<'a> DnsCli<'a> {
    pub fn dig(&self, name: DnsName) -> DigBuilder<'a> {
        DigBuilder::new(self.executor, name)
    }

    pub fn reverse(&self, address: IpAddr) -> DigBuilder<'a> {
        DigBuilder::reverse(self.executor, address)
    }
}
