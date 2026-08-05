use crate::utils::exec::{CommandExecutor, ExecResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PublicKeySource {
    Certificate,
    PrivateKey,
}

pub(crate) struct PublicKeyBuilder<'a> {
    executor: &'a CommandExecutor,
    source: PublicKeySource,
    path: &'a str,
}

impl<'a> PublicKeyBuilder<'a> {
    pub(crate) fn new(
        executor: &'a CommandExecutor,
        source: PublicKeySource,
        path: &'a str,
    ) -> Self {
        Self {
            executor,
            source,
            path,
        }
    }

    pub(crate) async fn run(self) -> ExecResult<String> {
        let args = match self.source {
            PublicKeySource::Certificate => vec!["x509", "-in", self.path, "-pubkey", "-noout"],
            PublicKeySource::PrivateKey => vec!["pkey", "-in", self.path, "-pubout"],
        };
        Ok(self.executor.run("openssl", args).await?.stdout)
    }
}
