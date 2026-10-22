use crate::OsCli;
use crate::exec::{CommandExecutor, ExecResult};
use crate::file::FileMode;

pub(crate) struct TemporaryPemFiles<'a> {
    executor: &'a CommandExecutor,
    certificate_path: String,
    private_key_path: Option<String>,
}

impl<'a> TemporaryPemFiles<'a> {
    pub(crate) async fn certificate(
        executor: &'a CommandExecutor,
        certificate: &str,
    ) -> ExecResult<Self> {
        let files = Self {
            executor,
            certificate_path: temporary_path("certificate"),
            private_key_path: None,
        };
        OsCli::new(executor)
            .file(&files.certificate_path)
            .write(certificate)
            .execute()
            .await?;
        Ok(files)
    }

    pub(crate) async fn pair(
        executor: &'a CommandExecutor,
        certificate: &str,
        private_key: &str,
    ) -> ExecResult<Self> {
        let mut files = Self::certificate(executor, certificate).await?;
        let key_path = temporary_path("private-key");
        let os = OsCli::new(executor);
        if let Err(error) = async {
            os.file(&key_path).write(private_key).execute().await?;
            os.file(&key_path)
                .chmod(FileMode::OwnerReadWrite)
                .run()
                .await
        }
        .await
        {
            files.cleanup().await;
            let _ = os.file(&key_path).delete().run().await;
            return Err(error);
        }
        files.private_key_path = Some(key_path);
        Ok(files)
    }

    pub(crate) fn certificate_path(&self) -> &str {
        &self.certificate_path
    }

    pub(crate) fn private_key_path(&self) -> Option<&str> {
        self.private_key_path.as_deref()
    }

    pub(crate) async fn cleanup(&self) {
        let os = OsCli::new(self.executor);
        let _ = os.file(&self.certificate_path).delete().run().await;
        if let Some(path) = &self.private_key_path {
            let _ = os.file(path).delete().run().await;
        }
    }
}

fn temporary_path(kind: &str) -> String {
    format!(
        "/tmp/openoxide-{kind}-{}.pem",
        uuid::Uuid::new_v4().simple()
    )
}
