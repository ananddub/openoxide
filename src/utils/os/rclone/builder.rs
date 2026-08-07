use super::command::RcloneCommand;
use super::target::RcloneTarget;
use crate::utils::exec::{CommandExecutor, ExecOutput, ExecResult};
use std::collections::HashMap;
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RcloneListFormat {
    PathSizeModified,
}

impl RcloneListFormat {
    const fn value(self) -> &'static str {
        match self {
            Self::PathSizeModified => "pst",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RcloneSeparator {
    Tab,
    Comma,
}

impl RcloneSeparator {
    const fn value(self) -> &'static str {
        match self {
            Self::Tab => "\t",
            Self::Comma => ",",
        }
    }
}

#[derive(Clone)]
pub struct RcloneBuilder {
    command: RcloneCommand,
    source: Option<RcloneTarget>,
    destination: Option<RcloneTarget>,

    // Performance & Optimization
    transfers: Option<u32>,
    checkers: Option<u32>,
    buffer_size: Option<String>,
    bwlimit: Option<String>,
    fast_list: bool,
    use_mmap: bool,

    // Timeouts & Retries
    retries: Option<u32>,
    low_level_retries: Option<u32>,
    timeout: Option<String>,
    connect_timeout: Option<String>,
    retry_delay: Option<String>,

    // Safety & Flags
    dry_run: bool,
    check_first: bool,
    ignore_errors: bool,
    ignore_existing: bool,
    update: bool,
    inplace: bool,

    // Logging
    log_file: Option<String>,
    log_level: Option<String>,
    stats: Option<String>,

    recursive: bool,
    files_only: bool,
    list_format: Option<RcloneListFormat>,
    separator: Option<RcloneSeparator>,
}

impl RcloneBuilder {
    pub fn new(command: RcloneCommand) -> Self {
        Self {
            command,
            source: None,
            destination: None,
            transfers: None,
            checkers: None,
            buffer_size: None,
            bwlimit: None,
            fast_list: false,
            use_mmap: false,
            retries: None,
            low_level_retries: None,
            timeout: None,
            connect_timeout: None,
            retry_delay: None,
            dry_run: false,
            check_first: false,
            ignore_errors: false,
            ignore_existing: false,
            update: false,
            inplace: false,
            log_file: None,
            log_level: None,
            stats: None,
            recursive: false,
            files_only: false,
            list_format: None,
            separator: None,
        }
    }

    pub fn source(mut self, target: RcloneTarget) -> Self {
        self.source = Some(target);
        self
    }

    pub fn destination(mut self, target: RcloneTarget) -> Self {
        self.destination = Some(target);
        self
    }

    pub fn transfers(mut self, transfers: u32) -> Self {
        self.transfers = Some(transfers);
        self
    }

    pub fn checkers(mut self, checkers: u32) -> Self {
        self.checkers = Some(checkers);
        self
    }

    pub fn buffer_size(mut self, buffer_size: impl Into<String>) -> Self {
        self.buffer_size = Some(buffer_size.into());
        self
    }

    pub fn bwlimit(mut self, bwlimit: impl Into<String>) -> Self {
        self.bwlimit = Some(bwlimit.into());
        self
    }

    pub fn fast_list(mut self) -> Self {
        self.fast_list = true;
        self
    }

    pub fn use_mmap(mut self) -> Self {
        self.use_mmap = true;
        self
    }

    pub fn retries(mut self, retries: u32) -> Self {
        self.retries = Some(retries);
        self
    }

    pub fn low_level_retries(mut self, low_level_retries: u32) -> Self {
        self.low_level_retries = Some(low_level_retries);
        self
    }

    pub fn timeout(mut self, timeout: impl Into<String>) -> Self {
        self.timeout = Some(timeout.into());
        self
    }

    pub fn connect_timeout(mut self, connect_timeout: impl Into<String>) -> Self {
        self.connect_timeout = Some(connect_timeout.into());
        self
    }

    pub fn retry_delay(mut self, retry_delay: impl Into<String>) -> Self {
        self.retry_delay = Some(retry_delay.into());
        self
    }

    pub fn dry_run(mut self) -> Self {
        self.dry_run = true;
        self
    }

    pub fn check_first(mut self) -> Self {
        self.check_first = true;
        self
    }

    pub fn ignore_errors(mut self) -> Self {
        self.ignore_errors = true;
        self
    }

    pub fn ignore_existing(mut self) -> Self {
        self.ignore_existing = true;
        self
    }

    pub fn update(mut self) -> Self {
        self.update = true;
        self
    }

    pub fn inplace(mut self) -> Self {
        self.inplace = true;
        self
    }

    pub fn log_file(mut self, log_file: impl Into<String>) -> Self {
        self.log_file = Some(log_file.into());
        self
    }

    pub fn log_level(mut self, log_level: impl Into<String>) -> Self {
        self.log_level = Some(log_level.into());
        self
    }

    pub fn stats(mut self, stats: impl Into<String>) -> Self {
        self.stats = Some(stats.into());
        self
    }

    pub fn recursive(mut self) -> Self {
        self.recursive = true;
        self
    }

    pub fn files_only(mut self) -> Self {
        self.files_only = true;
        self
    }

    pub fn list_format(mut self, format: RcloneListFormat) -> Self {
        self.list_format = Some(format);
        self
    }

    pub fn separator(mut self, separator: RcloneSeparator) -> Self {
        self.separator = Some(separator);
        self
    }

    pub fn build(self) -> (Vec<String>, HashMap<String, String>) {
        let mut args = Vec::new();
        let mut envs = HashMap::new();

        args.push(self.command.as_str().to_string());

        if let Some(t) = self.transfers {
            args.push(format!("--transfers={}", t));
        }
        if let Some(c) = self.checkers {
            args.push(format!("--checkers={}", c));
        }
        if let Some(ref b) = self.buffer_size {
            args.push(format!("--buffer-size={}", b));
        }
        if let Some(ref bw) = self.bwlimit {
            args.push(format!("--bwlimit={}", bw));
        }
        if self.fast_list {
            args.push("--fast-list".to_string());
        }
        if self.use_mmap {
            args.push("--use-mmap".to_string());
        }

        if let Some(r) = self.retries {
            args.push(format!("--retries={}", r));
        }
        if let Some(lr) = self.low_level_retries {
            args.push(format!("--low-level-retries={}", lr));
        }
        if let Some(ref t) = self.timeout {
            args.push(format!("--timeout={}", t));
        }
        if let Some(ref ct) = self.connect_timeout {
            args.push(format!("--contimeout={}", ct));
        }
        if let Some(ref rd) = self.retry_delay {
            args.push(format!("--retry-delay={}", rd));
        }

        if self.dry_run {
            args.push("--dry-run".to_string());
        }
        if self.check_first {
            args.push("--check-first".to_string());
        }
        if self.ignore_errors {
            args.push("--ignore-errors".to_string());
        }
        if self.ignore_existing {
            args.push("--ignore-existing".to_string());
        }
        if self.update {
            args.push("--update".to_string());
        }
        if self.inplace {
            args.push("--inplace".to_string());
        }

        if let Some(ref lf) = self.log_file {
            args.push(format!("--log-file={}", lf));
        }
        if let Some(ref ll) = self.log_level {
            args.push(format!("--log-level={}", ll));
        }
        if let Some(ref st) = self.stats {
            args.push(format!("--stats={}", st));
        }

        if self.recursive {
            args.push("--recursive".into());
        }
        if self.files_only {
            args.push("--files-only".into());
        }
        if let Some(format) = self.list_format {
            args.push(format!("--format={}", format.value()));
        }
        if let Some(separator) = self.separator {
            args.push(format!("--separator={}", separator.value()));
        }

        if let Some(ref src) = self.source {
            let (target_path, target_envs) = src.compile("src");
            args.push(target_path);
            envs.extend(target_envs);
        }

        if let Some(ref dest) = self.destination {
            let (target_path, target_envs) = dest.compile("dest");
            args.push(target_path);
            envs.extend(target_envs);
        }

        (args, envs)
    }

    pub fn to_command_string(self) -> String {
        let (args, envs) = self.build();
        let env_string = envs
            .into_iter()
            .map(|(k, v)| format!("{}={}", k, shell_single_quote(&v)))
            .collect::<Vec<String>>()
            .join(" ");

        if env_string.is_empty() {
            format!("rclone {}", args.join(" "))
        } else {
            format!("{} rclone {}", env_string, args.join(" "))
        }
    }

    pub async fn execute(self, executor: &CommandExecutor) -> ExecResult<ExecOutput> {
        let cmd = self.to_command_string();
        executor.run("sh", &["-c", &cmd]).await
    }

    pub async fn execute_cancelled(
        self,
        executor: &CommandExecutor,
        cancel: &CancellationToken,
    ) -> ExecResult<ExecOutput> {
        let cmd = self.to_command_string();
        executor.run_cancelled("sh", &["-c", &cmd], cancel).await
    }
}

fn shell_single_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rclone_sftp_upload() {
        let src = RcloneTarget::Local {
            path: "/tmp/source.txt".to_string(),
        };
        let dest = RcloneTarget::Sftp {
            host: "100.117.217.5".to_string(),
            port: Some(22),
            user: "das".to_string(),
            pass: None,
            key_file: Some("/home/das/.ssh/id_ed25519".to_string()),
            key_use_agent: false,
            path: "/home/das/dest.txt".to_string(),
        };

        let builder = RcloneBuilder::new(RcloneCommand::Copyto)
            .source(src)
            .destination(dest)
            .transfers(4)
            .connect_timeout("10s")
            .ignore_errors();

        let cmd_str = builder.to_command_string();

        // Check that the generated environment variables are present
        assert!(cmd_str.contains("RCLONE_CONFIG_DEST_SFTP_TYPE='sftp'"));
        assert!(cmd_str.contains("RCLONE_CONFIG_DEST_SFTP_HOST='100.117.217.5'"));
        assert!(cmd_str.contains("RCLONE_CONFIG_DEST_SFTP_USER='das'"));
        assert!(cmd_str.contains("RCLONE_CONFIG_DEST_SFTP_PORT='22'"));
        assert!(cmd_str.contains("RCLONE_CONFIG_DEST_SFTP_KEY_FILE='/home/das/.ssh/id_ed25519'"));

        // Check that rclone flags and targets are correct
        assert!(cmd_str.contains("rclone copyto"));
        assert!(cmd_str.contains("--transfers=4"));
        assert!(cmd_str.contains("--contimeout=10s"));
        assert!(cmd_str.contains("--ignore-errors"));
        assert!(cmd_str.contains("/tmp/source.txt dest_sftp:/home/das/dest.txt"));
    }
}
