use super::{TemporaryPemFiles, validate_pem, validation_error};
use crate::utils::exec::{CommandExecutor, ExecResult};

pub struct CertificateExpiryBuilder<'a> {
    executor: &'a CommandExecutor,
    certificate: String,
}

impl<'a> CertificateExpiryBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, certificate: String) -> Self {
        Self {
            executor,
            certificate,
        }
    }

    pub async fn run(self) -> ExecResult<i64> {
        validate_pem(
            &self.certificate,
            "BEGIN CERTIFICATE",
            "invalid PEM certificate",
        )?;
        let files = TemporaryPemFiles::certificate(self.executor, &self.certificate).await?;
        let result = certificate_expiry(self.executor, files.certificate_path()).await;
        files.cleanup().await;
        result
    }
}

pub(crate) async fn certificate_expiry(executor: &CommandExecutor, path: &str) -> ExecResult<i64> {
    let output = executor
        .run("openssl", ["x509", "-in", path, "-noout", "-enddate"])
        .await?;
    let raw = output.stdout_trimmed().trim_start_matches("notAfter=");
    chrono::NaiveDateTime::parse_from_str(raw, "%b %e %H:%M:%S %Y GMT")
        .map(|value| value.and_utc().timestamp())
        .map_err(|error| validation_error(&format!("invalid certificate expiry: {error}")))
}
