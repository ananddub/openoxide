use super::{TarCompression, TarOverwritePolicy};
use crate::exec::script::IntoCommand;
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::escape_arg;

pub struct ArchiveExtractBuilder<'a> {
    executor: &'a CommandExecutor,
    archive: String,
    destination: String,
    compression: TarCompression,
    overwrite: TarOverwritePolicy,
    strip_components: Option<u32>,
    numeric_owner: bool,
    preserve_permissions: bool,
}

impl<'a> ArchiveExtractBuilder<'a> {
    pub(crate) fn new(
        executor: &'a CommandExecutor,
        archive: String,
        destination: impl IntoCommand,
        compression: TarCompression,
    ) -> Self {
        Self {
            executor,
            archive,
            destination: destination.build_str(),
            compression,
            overwrite: TarOverwritePolicy::Replace,
            strip_components: None,
            numeric_owner: false,
            preserve_permissions: false,
        }
    }
    pub fn overwrite_policy(mut self, policy: TarOverwritePolicy) -> Self {
        self.overwrite = policy;
        self
    }
    pub fn strip_components(mut self, count: u32) -> Self {
        self.strip_components = Some(count);
        self
    }
    pub fn numeric_owner(mut self) -> Self {
        self.numeric_owner = true;
        self
    }
    pub fn preserve_permissions(mut self) -> Self {
        self.preserve_permissions = true;
        self
    }
    fn args(&self) -> Vec<String> {
        let mut args = self.compression.extract_args();
        args.push(self.archive.clone());
        args.extend(["-C".into(), self.destination.clone()]);
        match self.overwrite {
            TarOverwritePolicy::Replace => {}
            TarOverwritePolicy::KeepExisting => args.push("--keep-old-files".into()),
            TarOverwritePolicy::UnlinkFirst => args.push("--unlink-first".into()),
        }
        if let Some(count) = self.strip_components {
            args.push(format!("--strip-components={count}"));
        }
        if self.numeric_owner {
            args.push("--numeric-owner".into());
        }
        if self.preserve_permissions {
            args.push("--preserve-permissions".into());
        }
        args
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.executor.run("tar", self.args()).await
    }
}

impl IntoCommand for ArchiveExtractBuilder<'_> {
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
