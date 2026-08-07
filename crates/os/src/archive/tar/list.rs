use super::TarCompression;
use crate::exec::script::IntoCommand;
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::escape_arg;

pub struct ArchiveListBuilder<'a> {
    executor: &'a CommandExecutor,
    archive: String,
    compression: TarCompression,
    verbose: bool,
}

impl<'a> ArchiveListBuilder<'a> {
    pub(crate) fn new(
        executor: &'a CommandExecutor,
        archive: String,
        compression: TarCompression,
    ) -> Self {
        Self {
            executor,
            archive,
            compression,
            verbose: false,
        }
    }
    pub fn verbose(mut self) -> Self {
        self.verbose = true;
        self
    }
    fn args(&self) -> Vec<String> {
        let mut args = self.compression.list_args();
        if self.verbose {
            args.push("--verbose".into());
        }
        args.push(self.archive.clone());
        args
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.executor.run("tar", self.args()).await
    }
}

impl IntoCommand for ArchiveListBuilder<'_> {
    fn build_str(&self) -> String {
        format!(
            "tar {}",
            self.args()
                .iter()
                .map(escape_arg)
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}
