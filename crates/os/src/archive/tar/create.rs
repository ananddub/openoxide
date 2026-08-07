use super::TarCompression;
use crate::exec::script::IntoCommand;
use crate::exec::{CommandExecutor, ExecOutput, ExecResult};
use crate::escape_arg;

pub struct ArchiveCreateBuilder<'a> {
    executor: &'a CommandExecutor,
    archive: String,
    entries: Vec<(Option<String>, String)>,
    ignore_failed_read: bool,
    compression: TarCompression,
    exclude_patterns: Vec<String>,
    follow_symlinks: bool,
    one_file_system: bool,
    numeric_owner: bool,
    sparse: bool,
}

impl<'a> ArchiveCreateBuilder<'a> {
    pub(crate) fn new(
        executor: &'a CommandExecutor,
        archive: String,
        compression: TarCompression,
    ) -> Self {
        Self {
            executor,
            archive,
            entries: Vec::new(),
            ignore_failed_read: false,
            compression,
            exclude_patterns: Vec::new(),
            follow_symlinks: false,
            one_file_system: false,
            numeric_owner: false,
            sparse: false,
        }
    }

    pub fn entry(mut self, path: impl IntoCommand) -> Self {
        self.entries.push((None, path.build_str()));
        self
    }

    pub fn entry_from(mut self, base: impl IntoCommand, path: impl IntoCommand) -> Self {
        self.entries
            .push((Some(base.build_str()), path.build_str()));
        self
    }

    pub fn ignore_failed_reads(mut self) -> Self {
        self.ignore_failed_read = true;
        self
    }
    pub fn exclude(mut self, pattern: impl Into<String>) -> Self {
        self.exclude_patterns.push(pattern.into());
        self
    }
    pub fn exclude_version_control(mut self) -> Self {
        self.exclude_patterns
            .extend([".git".into(), ".hg".into(), ".svn".into()]);
        self
    }
    pub fn follow_symlinks(mut self) -> Self {
        self.follow_symlinks = true;
        self
    }
    pub fn stay_on_file_system(mut self) -> Self {
        self.one_file_system = true;
        self
    }
    pub fn numeric_owner(mut self) -> Self {
        self.numeric_owner = true;
        self
    }
    pub fn sparse_files(mut self) -> Self {
        self.sparse = true;
        self
    }

    fn args(&self) -> Vec<String> {
        let mut args = self.compression.create_args();
        args.push(self.archive.clone());
        if self.ignore_failed_read {
            args.push("--ignore-failed-read".into());
        }
        if self.follow_symlinks {
            args.push("--dereference".into());
        }
        if self.one_file_system {
            args.push("--one-file-system".into());
        }
        if self.numeric_owner {
            args.push("--numeric-owner".into());
        }
        if self.sparse {
            args.push("--sparse".into());
        }
        for pattern in &self.exclude_patterns {
            args.push(format!("--exclude={pattern}"));
        }
        for (base, path) in &self.entries {
            if let Some(base) = base {
                args.extend(["-C".into(), base.clone()]);
            }
            args.push(path.clone());
        }
        args
    }

    pub async fn run(self) -> ExecResult<ExecOutput> {
        self.executor.run("tar", self.args()).await
    }
}

impl IntoCommand for ArchiveCreateBuilder<'_> {
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
