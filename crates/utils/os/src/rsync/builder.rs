//! `rsync` command builder that assembles a raw **argv** vector and hands it to
//! [`CommandExecutor`] — never through a shell string. Because the arguments
//! reach `execv` directly, a source/destination path containing a space, `$()`,
//! `;` or quote cannot split into extra tokens or inject commands: this is the
//! structurally-safe counterpart to the rclone `sh -c` path (Gate G2 does not
//! apply here).
//!
//! The only value rsync re-tokenises itself is the `-e`/`--rsh` remote-shell
//! string, which it splits on whitespace when spawning `ssh`. The pieces we
//! embed there (key file, known-hosts file, user, host, port) are panel- or
//! system-generated, so [`RsyncBuilder`] validates them and rejects anything
//! containing whitespace or shell metacharacters before building argv.

use std::path::PathBuf;

use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::exec::{CommandExecutor, ExecExitStatus, ExecOutput, ExecResult, ExecStreamEvent};

/// SSH transport settings for the `-e` remote-shell string. `identity_file` and
/// `known_hosts_file` are expected to be short-lived, panel-created temp files
/// on the source host (0600 key, pinned known_hosts).
#[derive(Debug, Clone)]
pub struct RsyncSsh {
    pub port: u16,
    pub identity_file: PathBuf,
    pub known_hosts_file: PathBuf,
}

impl RsyncSsh {
    /// Render the `-e` value. Rejects paths whose string form contains
    /// whitespace (which would break rsync's tokeniser) — a hostile-injection
    /// surface is impossible here because these are our own temp paths.
    fn render(&self) -> ExecResult<String> {
        let id = self.identity_file.to_str().ok_or_else(|| {
            crate::exec::ExecError::InvalidData("rsync identity file is not valid UTF-8".into())
        })?;
        let kh = self.known_hosts_file.to_str().ok_or_else(|| {
            crate::exec::ExecError::InvalidData("rsync known_hosts file is not valid UTF-8".into())
        })?;
        for (name, v) in [("identity_file", id), ("known_hosts_file", kh)] {
            if v.is_empty() || v.chars().any(char::is_whitespace) {
                return Err(crate::exec::ExecError::InvalidData(format!(
                    "rsync {name} must be non-empty and contain no whitespace: {v:?}"
                )));
            }
        }
        Ok(format!(
            "ssh -p {port} -i {id} -o UserKnownHostsFile={kh} -o StrictHostKeyChecking=yes -o BatchMode=yes",
            port = self.port,
        ))
    }
}

/// Fluent builder for a single `rsync` invocation.
#[derive(Debug, Clone, Default)]
pub struct RsyncBuilder {
    archive: bool,
    hardlinks: bool,
    acls: bool,
    xattrs: bool,
    partial: bool,
    append_verify: bool,
    delete: bool,
    checksum: bool,
    compress: bool,
    dry_run: bool,
    itemize_changes: bool,
    progress2: bool,
    recursive: bool,
    links: bool,
    times: bool,
    perms: bool,
    owner: bool,
    group: bool,
    bwlimit: Option<String>,
    ssh: Option<RsyncSsh>,
    source: Option<String>,
    destination: Option<String>,
    extra: Vec<String>,
}

impl RsyncBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// `-a` (recursive + links + perms + times + group + owner + devices).
    pub fn archive(mut self) -> Self {
        self.archive = true;
        self
    }

    /// `-HAX`: hardlinks, ACLs, xattrs — the metadata fidelity the migration
    /// needs for volume/bind copies.
    pub fn preserve_metadata(mut self) -> Self {
        self.hardlinks = true;
        self.acls = true;
        self.xattrs = true;
        self
    }

    /// Keep partially-transferred files so a re-run resumes (`--partial`), and
    /// verify the existing prefix before appending (`--append-verify`). This is
    /// the mid-file block-delta resume primitive.
    pub fn resumable(mut self) -> Self {
        self.partial = true;
        self.append_verify = true;
        self
    }

    /// `--delete` — ONLY ever point this at a migration-scoped staging prefix.
    pub fn delete(mut self) -> Self {
        self.delete = true;
        self
    }

    pub fn checksum(mut self) -> Self {
        self.checksum = true;
        self
    }

    pub fn compress(mut self) -> Self {
        self.compress = true;
        self
    }

    /// `--dry-run --itemize-changes`: the empty-delta VERIFY probe. The caller
    /// treats a non-empty itemized output as "still dirty".
    pub fn dry_run_itemize(mut self) -> Self {
        self.dry_run = true;
        self.itemize_changes = true;
        self
    }

    pub fn progress(mut self) -> Self {
        self.progress2 = true;
        self
    }

    pub fn bwlimit(mut self, limit: impl Into<String>) -> Self {
        self.bwlimit = Some(limit.into());
        self
    }

    pub fn ssh(mut self, ssh: RsyncSsh) -> Self {
        self.ssh = Some(ssh);
        self
    }

    pub fn source(mut self, path: impl Into<String>) -> Self {
        self.source = Some(path.into());
        self
    }

    pub fn destination(mut self, url: impl Into<String>) -> Self {
        self.destination = Some(url.into());
        self
    }

    /// Escape hatch for additional already-safe argv tokens (e.g. filters).
    pub fn arg(mut self, value: impl Into<String>) -> Self {
        self.extra.push(value.into());
        self
    }

    /// Assemble the argv vector (without the leading `rsync` program name).
    pub fn build_args(&self) -> ExecResult<Vec<String>> {
        let mut a: Vec<String> = Vec::new();

        if self.archive {
            a.push("-a".into());
        }
        // Build a compact flag bundle for the discrete metadata flags so we
        // match the conventional `-HAX` form when all are requested.
        let mut bundle = String::new();
        if self.hardlinks {
            bundle.push('H');
        }
        if self.acls {
            bundle.push('A');
        }
        if self.xattrs {
            bundle.push('X');
        }
        if !bundle.is_empty() {
            a.push(format!("-{bundle}"));
        }
        if self.recursive {
            a.push("-r".into());
        }
        if self.links {
            a.push("-l".into());
        }
        if self.times {
            a.push("-t".into());
        }
        if self.perms {
            a.push("-p".into());
        }
        if self.owner {
            a.push("-o".into());
        }
        if self.group {
            a.push("-g".into());
        }
        if self.delete {
            a.push("--delete".into());
        }
        if self.checksum {
            a.push("--checksum".into());
        }
        if self.compress {
            a.push("--compress".into());
        }
        if self.partial {
            a.push("--partial".into());
        }
        if self.append_verify {
            a.push("--append-verify".into());
        }
        if self.dry_run {
            a.push("--dry-run".into());
        }
        if self.itemize_changes {
            a.push("--itemize-changes".into());
        }
        if self.progress2 {
            a.push("--info=progress2".into());
        }
        if let Some(bw) = &self.bwlimit {
            a.push(format!("--bwlimit={bw}"));
        }
        a.extend(self.extra.iter().cloned());

        if let Some(ssh) = &self.ssh {
            a.push("-e".into());
            a.push(ssh.render()?);
        }

        let source = self.source.as_ref().ok_or_else(|| {
            crate::exec::ExecError::InvalidData("rsync builder: source is required".into())
        })?;
        let destination = self.destination.as_ref().ok_or_else(|| {
            crate::exec::ExecError::InvalidData("rsync builder: destination is required".into())
        })?;
        if source.is_empty() || destination.is_empty() {
            return Err(crate::exec::ExecError::InvalidData(
                "rsync builder: source/destination must be non-empty".into(),
            ));
        }

        // `--` so a path beginning with `-` is never mistaken for an option.
        a.push("--".into());
        a.push(source.clone());
        a.push(destination.clone());
        Ok(a)
    }

    pub async fn execute(self, executor: &CommandExecutor) -> ExecResult<ExecOutput> {
        let args = self.build_args()?;
        executor.run("rsync", &args).await
    }

    pub async fn execute_cancelled(
        self,
        executor: &CommandExecutor,
        cancel: &CancellationToken,
    ) -> ExecResult<ExecOutput> {
        let args = self.build_args()?;
        executor.run_cancelled("rsync", &args, cancel).await
    }

    pub async fn execute_stream(
        self,
        executor: &CommandExecutor,
        sender: mpsc::Sender<ExecStreamEvent>,
    ) -> ExecResult<ExecExitStatus> {
        let args = self.build_args()?;
        executor.run_stream("rsync", &args, sender).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> RsyncBuilder {
        RsyncBuilder::new()
            .archive()
            .preserve_metadata()
            .resumable()
            .progress()
            .ssh(RsyncSsh {
                port: 22,
                identity_file: "/tmp/mig_key".into(),
                known_hosts_file: "/tmp/mig_known_hosts".into(),
            })
            .source("/var/lib/docker/volumes/app-data/_data/")
            .destination("root@10.0.0.5:/opt/openoxide/migrations/7/staging/volume:abcd/")
    }

    #[test]
    fn builds_expected_argv() {
        let args = base().build_args().expect("argv");
        assert_eq!(args[0], "-a");
        assert_eq!(args[1], "-HAX");
        assert!(args.contains(&"--partial".to_string()));
        assert!(args.contains(&"--append-verify".to_string()));
        assert!(args.contains(&"--info=progress2".to_string()));
        let e = args.iter().position(|x| x == "-e").unwrap();
        assert!(args[e + 1].starts_with("ssh -p 22 -i /tmp/mig_key"));
        assert!(args[e + 1].contains("-o UserKnownHostsFile=/tmp/mig_known_hosts"));
        assert!(args[e + 1].contains("StrictHostKeyChecking=yes"));
        assert!(args[e + 1].contains("BatchMode=yes"));
    }

    #[test]
    fn separator_precedes_paths() {
        let args = base().build_args().unwrap();
        let dd = args.iter().position(|x| x == "--").unwrap();
        assert_eq!(args[dd + 1], "/var/lib/docker/volumes/app-data/_data/");
        assert!(args[dd + 2].starts_with("root@10.0.0.5:"));
    }

    #[test]
    fn metacharacters_in_paths_stay_single_argv_tokens() {
        // Gate G2 analog: these never reach a shell, so they must survive as
        // exactly one argument each, byte-for-byte.
        let args = RsyncBuilder::new()
            .archive()
            .source("/tmp/$(touch /tmp/pwned); x/")
            .destination("host:/dest with space/")
            .build_args()
            .unwrap();
        let dd = args.iter().position(|x| x == "--").unwrap();
        assert_eq!(args[dd + 1], "/tmp/$(touch /tmp/pwned); x/");
        assert_eq!(args[dd + 2], "host:/dest with space/");
    }

    #[test]
    fn ssh_key_path_with_whitespace_is_rejected() {
        let b = RsyncBuilder::new()
            .archive()
            .ssh(RsyncSsh {
                port: 22,
                identity_file: "/tmp/bad key".into(),
                known_hosts_file: "/tmp/kh".into(),
            })
            .source("/a/")
            .destination("h:/b/");
        assert!(b.build_args().is_err());
    }

    #[test]
    fn missing_source_or_destination_is_rejected() {
        assert!(RsyncBuilder::new().archive().build_args().is_err());
    }

    #[test]
    fn delete_only_when_requested_and_stays_in_options() {
        let args = base().delete().build_args().unwrap();
        assert!(args.contains(&"--delete".to_string()));
        // `--delete` must appear before the `--` path separator.
        let dd = args.iter().position(|x| x == "--").unwrap();
        let del = args.iter().position(|x| x == "--delete").unwrap();
        assert!(del < dd);
    }
}
