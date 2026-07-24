use crate::utils::exec::ArgBuilder;
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::utils::rclone::RcloneBuilder;
use tokio_util::sync::CancellationToken;

/// Trait for types that can be compiled into a shell command string.
pub trait IntoCommand {
    fn build_str(&self) -> String;

    #[allow(async_fn_in_trait)]
    async fn execute(&self, executor: &CommandExecutor) -> ExecResult<ExecOutput> {
        let cmd_str = self.build_str();
        executor.run("sh", &["-c", &cmd_str]).await
    }

    #[allow(async_fn_in_trait)]
    async fn execute_cancelled(
        &self,
        executor: &CommandExecutor,
        cancel: &CancellationToken,
    ) -> ExecResult<ExecOutput> {
        let cmd_str = self.build_str();
        executor
            .run_cancelled("sh", &["-c", &cmd_str], cancel)
            .await
    }
}

impl IntoCommand for String {
    fn build_str(&self) -> String {
        self.clone()
    }
}

impl IntoCommand for &str {
    fn build_str(&self) -> String {
        self.to_string()
    }
}

impl IntoCommand for &String {
    fn build_str(&self) -> String {
        (*self).clone()
    }
}

impl IntoCommand for ArgBuilder {
    fn build_str(&self) -> String {
        self.preview()
    }
}

impl IntoCommand for RcloneBuilder {
    fn build_str(&self) -> String {
        self.clone().to_command_string()
    }
}

impl IntoCommand for Vec<String> {
    fn build_str(&self) -> String {
        self.iter()
            .map(|arg| super::shell_single_quote(arg))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl IntoCommand for Vec<&str> {
    fn build_str(&self) -> String {
        self.iter()
            .map(|arg| super::shell_single_quote(arg))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl<const N: usize> IntoCommand for [String; N] {
    fn build_str(&self) -> String {
        self.iter()
            .map(|arg| super::shell_single_quote(arg))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl<const N: usize> IntoCommand for [&str; N] {
    fn build_str(&self) -> String {
        self.iter()
            .map(|arg| super::shell_single_quote(arg))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl IntoCommand for &[String] {
    fn build_str(&self) -> String {
        self.iter()
            .map(|arg| super::shell_single_quote(arg))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl IntoCommand for &[&str] {
    fn build_str(&self) -> String {
        self.iter()
            .map(|arg| super::shell_single_quote(arg))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl<const N: usize> IntoCommand for &[String; N] {
    fn build_str(&self) -> String {
        self.iter()
            .map(|arg| super::shell_single_quote(arg))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl<const N: usize> IntoCommand for &[&str; N] {
    fn build_str(&self) -> String {
        self.iter()
            .map(|arg| super::shell_single_quote(arg))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl IntoCommand for i32 {
    fn build_str(&self) -> String {
        self.to_string()
    }
}

impl IntoCommand for u16 {
    fn build_str(&self) -> String {
        self.to_string()
    }
}

impl IntoCommand for u32 {
    fn build_str(&self) -> String {
        self.to_string()
    }
}

impl IntoCommand for usize {
    fn build_str(&self) -> String {
        self.to_string()
    }
}

impl IntoCommand for i64 {
    fn build_str(&self) -> String {
        self.to_string()
    }
}

impl IntoCommand for u64 {
    fn build_str(&self) -> String {
        self.to_string()
    }
}
