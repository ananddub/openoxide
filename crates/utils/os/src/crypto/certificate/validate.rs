use super::{
    CertificatePairValidation, PublicKeyBuilder, PublicKeySource, TemporaryPemFiles,
    certificate_expiry, validate_pem, validation_error,
};
use crate::exec::{CommandExecutor, ExecResult};

pub struct CertificatePairValidationBuilder<'a> {
    executor: &'a CommandExecutor,
    certificate: String,
    private_key: String,
}

impl<'a> CertificatePairValidationBuilder<'a> {
    pub(crate) fn new(
        executor: &'a CommandExecutor,
        certificate: String,
        private_key: String,
    ) -> Self {
        Self {
            executor,
            certificate,
            private_key,
        }
    }

    pub async fn run(self) -> ExecResult<CertificatePairValidation> {
        validate_pem(
            &self.certificate,
            "BEGIN CERTIFICATE",
            "invalid PEM certificate",
        )?;
        validate_pem(&self.private_key, "PRIVATE KEY", "invalid PEM private key")?;
        let files =
            TemporaryPemFiles::pair(self.executor, &self.certificate, &self.private_key).await?;
        let result = self.validate_files(&files).await;
        files.cleanup().await;
        result
    }

    async fn validate_files(
        &self,
        files: &TemporaryPemFiles<'_>,
    ) -> ExecResult<CertificatePairValidation> {
        let cert_public = PublicKeyBuilder::new(
            self.executor,
            PublicKeySource::Certificate,
            files.certificate_path(),
        )
        .run()
        .await?;
        let key_public = PublicKeyBuilder::new(
            self.executor,
            PublicKeySource::PrivateKey,
            files
                .private_key_path()
                .expect("pair always has a private key"),
        )
        .run()
        .await?;
        if cert_public != key_public {
            return Err(validation_error("certificate and private key do not match"));
        }
        let expires_at = certificate_expiry(self.executor, files.certificate_path()).await?;
        if expires_at <= chrono::Utc::now().timestamp() {
            return Err(validation_error("certificate is already expired"));
        }
        Ok(CertificatePairValidation { expires_at })
    }
}
