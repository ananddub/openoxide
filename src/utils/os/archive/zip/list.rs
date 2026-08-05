use super::ZipError;
use crate::utils::exec::script::{IntoCommand, shell_single_quote};
use crate::utils::exec::{CommandExecutor, ExecOutput};
use std::path::PathBuf;

pub struct ZipListBuilder<'a> {
    executor: &'a CommandExecutor,
    archive: PathBuf,
    names_only: bool,
    verbose: bool,
}

impl IntoCommand for ZipListBuilder<'_> {
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
impl<'a> ZipListBuilder<'a> {
    pub(crate) fn new(executor: &'a CommandExecutor, archive: PathBuf) -> Self {
        Self {
            executor,
            archive,
            names_only: false,
            verbose: false,
        }
    }
    pub fn names_only(mut self) -> Self {
        self.names_only = true;
        self.verbose = false;
        self
    }
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self.names_only = false;
        self
    }
    fn args(&self) -> Vec<String> {
        let mut args = Vec::new();
        if self.names_only {
            args.push("-Z1".into());
        } else if self.verbose {
            args.push("-v".into());
        } else {
            args.push("-l".into());
        }
        args.push(self.archive.to_string_lossy().into_owned());
        args
    }
    pub async fn run(self) -> Result<ExecOutput, ZipError> {
        self.executor
            .run("unzip", self.args())
            .await
            .map_err(Into::into)
    }
}
