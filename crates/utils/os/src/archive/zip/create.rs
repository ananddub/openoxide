use super::{CompressionLevel, ZipError, ZipPathMode};
use crate::exec::script::IntoCommand;
use crate::exec::script::shell_single_quote;
use crate::exec::{CommandExecutor, ExecOutput};
use std::path::PathBuf;

pub struct ZipCreateBuilder<'a> {
    executor: &'a CommandExecutor,
    archive: PathBuf,
    source: PathBuf,
    recursive: bool,
    contents_only: bool,
    path_mode: ZipPathMode,
    compression: CompressionLevel,
    excludes: Vec<String>,
    update: bool,
    freshen: bool,
    move_files: bool,
}

impl IntoCommand for ZipCreateBuilder<'_> {
    fn build_str(&self) -> String {
        let mut args = self.args();
        if self.contents_only {
            if let Some(source) = args.last_mut() {
                *source = ".".into();
            }
            return format!(
                "cd {} && zip {}",
                shell_single_quote(&self.source.to_string_lossy()),
                args.iter()
                    .map(|arg| shell_single_quote(arg))
                    .collect::<Vec<_>>()
                    .join(" ")
            );
        }
        format!(
            "zip {}",
            args.iter()
                .map(|arg| shell_single_quote(arg))
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}
impl<'a> ZipCreateBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, archive: PathBuf, source: PathBuf) -> Self {
        Self {
            executor,
            archive,
            source,
            recursive: false,
            contents_only: false,
            path_mode: ZipPathMode::Preserve,
            compression: CompressionLevel::Default,
            excludes: Vec::new(),
            update: false,
            freshen: false,
            move_files: false,
        }
    }
    pub fn recursive(mut self) -> Self {
        self.recursive = true;
        self
    }
    pub fn contents_only(mut self) -> Self {
        self.contents_only = true;
        self.recursive = true;
        self
    }
    pub fn path_mode(mut self, mode: ZipPathMode) -> Self {
        self.path_mode = mode;
        self
    }
    pub fn compression(mut self, level: CompressionLevel) -> Self {
        self.compression = level;
        self
    }
    pub fn exclude(mut self, pattern: impl Into<String>) -> Self {
        self.excludes.push(pattern.into());
        self
    }
    pub fn update_existing(mut self) -> Self {
        self.update = true;
        self
    }
    pub fn freshen_existing(mut self) -> Self {
        self.freshen = true;
        self
    }
    pub fn move_files(mut self) -> Self {
        self.move_files = true;
        self
    }
    fn args(&self) -> Vec<String> {
        let mut args = Vec::new();
        if self.recursive {
            args.push("-r".into());
        }
        if self.path_mode == ZipPathMode::FileNamesOnly {
            args.push("-j".into());
        }
        args.push(self.compression.flag().into());
        if self.update {
            args.push("-u".into());
        }
        if self.freshen {
            args.push("-f".into());
        }
        if self.move_files {
            args.push("-m".into());
        }
        for pattern in &self.excludes {
            args.extend(["-x".into(), pattern.clone()]);
        }
        args.push(self.archive.to_string_lossy().into_owned());
        args.push(self.source.to_string_lossy().into_owned());
        args
    }
    pub async fn run(self) -> Result<ExecOutput, ZipError> {
        let output = if self.contents_only {
            let mut args = self.args();
            if let Some(source) = args.last_mut() {
                *source = ".".into();
            }
            let command = format!(
                "cd {} && zip {}",
                shell_single_quote(&self.source.to_string_lossy()),
                args.iter()
                    .map(|arg| shell_single_quote(arg))
                    .collect::<Vec<_>>()
                    .join(" ")
            );
            self.executor.run("sh", ["-c", &command]).await?
        } else {
            self.executor.run("zip", self.args()).await?
        };
        if output.success() {
            Ok(output)
        } else {
            Err(ZipError::Failed(output.stderr))
        }
    }
}
