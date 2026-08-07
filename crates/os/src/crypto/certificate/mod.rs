mod expiry;
mod handle;
mod public_key;
mod temp_files;
mod types;
mod validate;

pub use expiry::CertificateExpiryBuilder;
pub use handle::CertificateBuilder;
pub use types::CertificatePairValidation;
pub use validate::CertificatePairValidationBuilder;

pub(crate) use expiry::certificate_expiry;
pub(crate) use public_key::{PublicKeyBuilder, PublicKeySource};
pub(crate) use temp_files::TemporaryPemFiles;

use crate::exec::{ExecError, ExecResult};

pub(crate) fn validate_pem(value: &str, marker: &str, message: &str) -> ExecResult<()> {
    if value.contains(marker) {
        Ok(())
    } else {
        Err(validation_error(message))
    }
}

pub(crate) fn validation_error(message: &str) -> ExecError {
    ExecError::CommandFailed {
        code: None,
        stderr: message.to_owned(),
    }
}
