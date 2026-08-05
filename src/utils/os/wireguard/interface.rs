use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::utils::os::OsCli;
use crate::utils::os::file::FileMode;
use zeroize::Zeroize;

use super::WireGuardConfig;

pub struct WireGuardInterfaceBuilder<'a> {
    executor: &'a CommandExecutor,
    name: String,
}

impl<'a> WireGuardInterfaceBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, name: String) -> Self {
        Self { executor, name }
    }

    pub async fn install(self, config: &WireGuardConfig) -> ExecResult<ExecOutput> {
        validate_name(&self.name)?;
        config
            .validate()
            .map_err(|error| crate::utils::exec::ExecError::CommandFailed {
                code: None,
                stderr: error.to_string(),
            })?;
        let path = format!("/etc/wireguard/{}.conf", self.name);
        let os = OsCli::new(self.executor);
        os.dir("/etc/wireguard")
            .create()
            .with_parents()
            .run()
            .await?;
        let mut rendered = config.render();
        let write_result = os.file(&path).write(&rendered).execute().await;
        rendered.zeroize();
        write_result?;
        os.file(&path).chmod(FileMode::OwnerReadWrite).run().await?;
        match self.executor.run("wg-quick", ["up", &self.name]).await {
            Ok(output) => Ok(output),
            Err(error) => {
                let _ = os.file(&path).delete().run().await;
                Err(error)
            }
        }
    }

    pub async fn remove(self) -> ExecResult<ExecOutput> {
        validate_name(&self.name)?;
        let _ = self.executor.run("wg-quick", ["down", &self.name]).await;
        OsCli::new(self.executor)
            .file(format!("/etc/wireguard/{}.conf", self.name))
            .delete()
            .run()
            .await
    }

    pub async fn latest_handshakes(self) -> ExecResult<ExecOutput> {
        validate_name(&self.name)?;
        self.executor
            .run("wg", ["show", &self.name, "latest-handshakes"])
            .await
    }
}

fn validate_name(name: &str) -> ExecResult<()> {
    if !name.is_empty()
        && name.len() <= 15
        && name
            .chars()
            .all(|value| value.is_ascii_alphanumeric() || matches!(value, '-' | '_' | '=' | '+'))
    {
        Ok(())
    } else {
        Err(crate::utils::exec::ExecError::CommandFailed {
            code: None,
            stderr: "invalid WireGuard interface name".into(),
        })
    }
}
