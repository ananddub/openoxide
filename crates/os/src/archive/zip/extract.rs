use super::ZipError;
use crate::exec::script::{IntoCommand, shell_single_quote};
use crate::exec::{CommandExecutor, ExecOutput};
use std::path::PathBuf;

pub struct ZipExtractBuilder<'a> {
    executor: &'a CommandExecutor,
    archive: PathBuf,
    destination: PathBuf,
    overwrite: bool,
    never_overwrite: bool,
    quiet: bool,
    excludes: Vec<String>,
    entries: Vec<String>,
}

impl IntoCommand for ZipExtractBuilder<'_> {
    fn build_str(&self) -> String {
        format!(
            "unzip {}",
            self.args()
                .iter()
                .map(|arg| shell_single_quote(arg))
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}
impl<'a> ZipExtractBuilder<'a> {
    pub(crate) fn new(
        executor: &'a CommandExecutor,
        archive: PathBuf,
        destination: PathBuf,
    ) -> Self {
        Self {
            executor,
            archive,
            destination,
            overwrite: false,
            never_overwrite: false,
            quiet: false,
            excludes: Vec::new(),
            entries: Vec::new(),
        }
    }
    pub fn overwrite(mut self) -> Self {
        self.overwrite = true;
        self.never_overwrite = false;
        self
    }
    pub fn never_overwrite(mut self) -> Self {
        self.never_overwrite = true;
        self.overwrite = false;
        self
    }
    pub fn quiet(mut self) -> Self {
        self.quiet = true;
        self
    }
    pub fn exclude(mut self, pattern: impl Into<String>) -> Self {
        self.excludes.push(pattern.into());
        self
    }
    pub fn entry(mut self, entry: impl Into<String>) -> Self {
        self.entries.push(entry.into());
        self
    }
    fn args(&self) -> Vec<String> {
        let mut args = Vec::new();
        if self.overwrite {
            args.push("-o".into());
        }
        if self.never_overwrite {
            args.push("-n".into());
        }
        if self.quiet {
            args.push("-q".into());
        }
        args.push(self.archive.to_string_lossy().into_owned());
        args.extend(self.entries.clone());
        for pattern in &self.excludes {
            args.extend(["-x".into(), pattern.clone()]);
        }
        args.extend(["-d".into(), self.destination.to_string_lossy().into_owned()]);
        args
    }
    pub async fn run(self) -> Result<ExecOutput, ZipError> {
        let output = self.executor.run("unzip", self.args()).await?;
        if output.success() {
            Ok(output)
        } else {
            Err(ZipError::Failed(output.stderr))
        }
    }
}
