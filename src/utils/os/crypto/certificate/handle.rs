use super::{CertificateExpiryBuilder, CertificatePairValidationBuilder};
use crate::utils::exec::CommandExecutor;

pub struct CertificateBuilder<'a> {
    executor: &'a CommandExecutor,
    certificate: String,
}

impl<'a> CertificateBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, certificate: impl Into<String>) -> Self {
        Self {
            executor,
            certificate: certificate.into(),
        }
    }

    pub fn expiry(self) -> CertificateExpiryBuilder<'a> {
        CertificateExpiryBuilder::new(self.executor, self.certificate)
    }

    pub fn validate_with_key(
        self,
        private_key: impl Into<String>,
    ) -> CertificatePairValidationBuilder<'a> {
        CertificatePairValidationBuilder::new(self.executor, self.certificate, private_key.into())
    }
}
