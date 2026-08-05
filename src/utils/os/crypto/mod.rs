pub mod certificate;
use crate::utils::exec::CommandExecutor;
pub use certificate::{
    CertificateBuilder, CertificateExpiryBuilder, CertificatePairValidation,
    CertificatePairValidationBuilder,
};

pub struct CryptoCli<'a> {
    pub(crate) executor: &'a CommandExecutor,
}

impl<'a> CryptoCli<'a> {
    pub fn certificate(&self, certificate: impl Into<String>) -> CertificateBuilder<'a> {
        CertificateBuilder::new(self.executor, certificate)
    }
}
