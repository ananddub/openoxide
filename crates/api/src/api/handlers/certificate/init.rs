use std::sync::Arc;

use auto_di::singleton;

use crate::services::networking::certificate::CertificateService as AppCertificateService;

pub struct CertificateController {
    pub(super) service: Arc<AppCertificateService>,
}

#[singleton]
impl CertificateController {
    fn new(service: Arc<AppCertificateService>) -> Self {
        Self { service }
    }
}
