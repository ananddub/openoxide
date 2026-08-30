use std::sync::Arc;

use auto_di::singleton;

use crate::services::dns::DnsService as AppDnsService;

pub struct DnsController {
    pub(super) service: Arc<AppDnsService>,
}

#[singleton]
impl DnsController {
    fn new(service: Arc<AppDnsService>) -> Self {
        Self { service }
    }
}
