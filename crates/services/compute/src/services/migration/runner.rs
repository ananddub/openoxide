//! S4 sync execution: materialise one-use ssh credentials on the SOURCE,
//! push a [`PlannedObject`] to its containment-checked staging prefix with
//! rsync (argv, cancel-aware, `--info=progress2` telemetry), verify the empty
//! delta, and advance the `migration_objects` checkpoint through the
//! PENDING → SYNCING → SYNCED → VERIFIED discipline. Nothing here ever points
//! rsync at a live target path — destinations come exclusively from
//! [`super::planner`].

use crate::db::queries::FlashQueries;
use crate::utils::exec::{CommandExecutor, ExecStreamEvent};
use crate::utils::os::rsync::parse_progress2;

use std::path::Path;

use super::planner::{
    PlannedObject, known_hosts_entry, remote_destination, sync_argv, verify_argv,
};

/// Everything needed to reach the target with the source's own ssh client:
/// the managed private key (written 0600 locally, never shipped) and the
/// pinned host key captured during setup (Gate G1).
#[derive(Clone)]
pub struct SyncCredentials {
    pub ssh_port: u16,
    pub username: String,
    pub connect_host: String,
    pub private_key: String,
    pub host_key_openssh: String,
}

/// Temp key + known_hosts files, lives only for one object's sync/verify.
/// Drop removes the directory (the private key never outlives the step).
pub struct TempCredentials {
    dir: tempfile::TempDir,
}

const KEY_FILE_NAME: &str = "id";
const KNOWN_HOSTS_FILE_NAME: &str = "known_hosts";

impl TempCredentials {
    /// Write the pinned credentials to a fresh 0700 temp dir. Both files are
    /// explicitly chmod-ed 0600 (the private key must never be group/world
    /// readable regardless of umask); the known_hosts line is re-validated
    /// through [`known_hosts_entry`] so a corrupt DB value can never inject
    /// extra ssh tokens.
    pub fn create(credentials: &SyncCredentials) -> Result<Self, String> {
        use std::os::unix::fs::PermissionsExt;
        if credentials.private_key.trim().is_empty() {
            return Err("target ssh key has no private key material".into());
        }
        let known_hosts =
            known_hosts_entry(&credentials.connect_host, &credentials.host_key_openssh)
                .map_err(|error| error.to_string())?;
        let dir = tempfile::tempdir().map_err(|error| error.to_string())?;
        std::fs::write(
            dir.path().join(KEY_FILE_NAME),
            credentials.private_key.as_bytes(),
        )
        .map_err(|error| error.to_string())?;
        std::fs::set_permissions(
            dir.path().join(KEY_FILE_NAME),
            std::fs::Permissions::from_mode(0o600),
        )
        .map_err(|error| error.to_string())?;
        std::fs::write(
            dir.path().join(KNOWN_HOSTS_FILE_NAME),
            format!("{known_hosts}\n").as_bytes(),
        )
        .map_err(|error| error.to_string())?;
        std::fs::set_permissions(
            dir.path().join(KNOWN_HOSTS_FILE_NAME),
            std::fs::Permissions::from_mode(0o600),
        )
        .map_err(|error| error.to_string())?;
        // rsync tokenises the `-e` string on whitespace: our temp paths must
        // stay whitespace-free even under a customised TMPDIR.
        for path in [Self::key_path_in(&dir), Self::known_hosts_path_in(&dir)] {
            if path.chars().any(char::is_whitespace) {
                return Err(format!(
                    "temp credential path contains whitespace: {path:?}"
                ));
            }
        }
        Ok(Self { dir })
    }

    pub fn key_path(&self) -> String {
        Self::key_path_in(&self.dir)
    }

    pub fn known_hosts_path(&self) -> String {
        Self::known_hosts_path_in(&self.dir)
    }

    fn key_path_in(dir: &tempfile::TempDir) -> String {
        dir.path()
            .join(KEY_FILE_NAME)
            .to_string_lossy()
            .into_owned()
    }

    fn known_hosts_path_in(dir: &tempfile::TempDir) -> String {
        dir.path()
            .join(KNOWN_HOSTS_FILE_NAME)
            .to_string_lossy()
            .into_owned()
    }
}

/// Outcome of syncing + verifying one object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncOutcome {
    /// rsync exit 0 and the dry-run itemize reported an empty delta.
    Verified { bytes: i64 },
    /// rsync exit 0 but the verify probe still saw a delta (or errored).
    Dirty { message: String },
    /// rsync exited non-zero or transport failed.
    Failed { message: String },
}

/// ssh argv for the pre-`mkdir -p` on the target (same pinned options as the
/// rsync `-e` string, assembled as argv — never a shell string).
pub fn ssh_exec_args(
    credentials: &SyncCredentials,
    known_hosts_path: &str,
    key_path: &str,
    remote_command: &str,
) -> Result<Vec<String>, String> {
    // Re-validate the pinned key line before it reaches the temp file usage.
    known_hosts_entry(&credentials.connect_host, &credentials.host_key_openssh)
        .map_err(|error| error.to_string())?;
    for path in [key_path, known_hosts_path] {
        if path.is_empty() || path.chars().any(char::is_whitespace) {
            return Err(format!(
                "credential path must be non-empty and whitespace-free: {path:?}"
            ));
        }
    }
    if remote_command.is_empty() || remote_command.contains('\0') {
        return Err("remote command is empty".into());
    }
    Ok(vec![
        "-p".into(),
        credentials.ssh_port.to_string(),
        "-i".into(),
        key_path.into(),
        "-o".into(),
        format!("UserKnownHostsFile={known_hosts_path}"),
        "-o".into(),
        "StrictHostKeyChecking=yes".into(),
        "-o".into(),
        "BatchMode=yes".into(),
        format!("{}@{}", credentials.username, credentials.connect_host),
        "mkdir".into(),
        "-p".into(),
        "--".into(),
        remote_command.into(),
    ])
}

/// Parse `rsync --dry-run --itemize-changes` output; anything that is not a
/// directory header, a summary line or an itemised-directory marker means the
/// destination still differs → not VERIFIED.
pub fn has_dirty_delta(output: &str) -> bool {
    for raw in output.lines() {
        let line = raw.trim_end_matches('\r');
        let trimmed = line.trim_start();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with("sending ")
            || trimmed.starts_with("total size is")
            || trimmed.starts_with("speedup is")
            || trimmed.starts_with("Number of")
        {
            continue;
        }
        let mut chars = trimmed.chars();
        let kind = chars.next().unwrap_or(' ');
        let second = chars.next().unwrap_or(' ');
        // An itemise line is a marker token followed by the path (rsync
        // quotes paths containing spaces as "..."). Only the transfer root
        // (`.` / `./`) — or a bare marker — is structurally clean; any named
        // directory or file entry means the destination still differs.
        let name_of = |line: &str| -> String {
            let marker_len = line.find(char::is_whitespace).unwrap_or(line.len());

            line[marker_len..].trim().trim_matches('"').to_owned()
        };
        if kind == '.' && trimmed.starts_with("./") {
            continue;
        }
        if kind == 'c' && second == 'd' {
            let name = name_of(trimmed);
            if name.is_empty() || name == "." || name == "./" {
                continue;
            }
            return true;
        }
        if kind == 'd' {
            let name = name_of(trimmed);
            if name.is_empty() || name == "." || name == "./" {
                continue;
            }
            return true;
        }
        // Any other itemised entry (">f", ">*deleting", "*f", links, specials)
        // is a real delta.
        return true;
    }
    false
}

/// Extract the final cumulative byte count from streamed progress2 samples.
#[derive(Default)]
pub struct ProgressAccumulator {
    last_bytes: u64,
    buffer: String,
}

impl ProgressAccumulator {
    pub fn push(&mut self, bytes: &[u8]) {
        self.buffer.push_str(&String::from_utf8_lossy(bytes));
        // progress2 rewrites one line with \r; drain complete segments.
        while let Some(cut) = self.buffer.find(['\r', '\n']) {
            let segment: String = self.buffer.drain(..=cut).collect();
            if let Some(progress) = parse_progress2(&segment) {
                self.last_bytes = progress.transferred_bytes;
            }
        }
        if self.buffer.len() > 4096 {
            // Never let a non-progress tail grow unbounded.
            self.buffer.clear();
        }
    }

    pub fn finish(&mut self) -> i64 {
        if !self.buffer.is_empty() {
            if let Some(progress) = parse_progress2(&self.buffer) {
                self.last_bytes = progress.transferred_bytes;
            }
            self.buffer.clear();
        }
        i64::try_from(self.last_bytes).unwrap_or(i64::MAX)
    }
}

fn failure_message(stderr: &str, code: Option<i32>) -> String {
    let tail: String = stderr
        .trim()
        .chars()
        .rev()
        .take(600)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
    format!("rsync/ssh failed (exit {code:?}): {tail}")
}

/// Full lifecycle for one object: SYNCING → rsync push → SYNCED (+bytes) →
/// dry-run verify → VERIFIED. Any failure records FAILED with the error.
#[allow(clippy::too_many_arguments)]
pub async fn sync_and_verify_object(
    source: &CommandExecutor,
    target: &CommandExecutor,
    queries: &FlashQueries,
    migration_id: &str,
    credentials: &SyncCredentials,
    planned: &PlannedObject,
) -> SyncOutcome {
    let checkpoint = ObjectCheckpoint {
        migration_id: migration_id.to_owned(),
        obj_key: planned.obj_key.clone(),
    };
    if let Err(error) = checkpoint.set_status(queries, "SYNCING", "").await {
        return SyncOutcome::Failed {
            message: format!("checkpoint write failed: {error}"),
        };
    }
    match run_sync_and_verify(source, target, credentials, planned).await {
        Ok(SyncOutcome::Verified { bytes }) => {
            let _ = checkpoint.record_bytes(queries, bytes).await;
            let _ = checkpoint.set_status(queries, "SYNCED", "").await;
            let _ = checkpoint.set_status(queries, "VERIFIED", "").await;
            SyncOutcome::Verified { bytes }
        }
        Ok(other @ SyncOutcome::Dirty { .. }) => {
            if let SyncOutcome::Dirty { message } = &other {
                let _ = checkpoint.set_status(queries, "SYNCED", message).await;
            }
            other
        }
        Ok(outcome @ SyncOutcome::Failed { .. }) => {
            if let SyncOutcome::Failed { message } = &outcome {
                let _ = checkpoint.set_status(queries, "FAILED", message).await;
            }
            outcome
        }
        Err(message) => {
            let _ = checkpoint.set_status(queries, "FAILED", &message).await;
            SyncOutcome::Failed { message }
        }
    }
}

/// Thin typed handle for one `migration_objects` checkpoint row.
pub struct ObjectCheckpoint {
    pub migration_id: String,
    pub obj_key: String,
}

impl ObjectCheckpoint {
    pub async fn ensure_exists(
        &self,
        queries: &FlashQueries,
        kind: &str,
        source_ref: &str,
        dest_ref: &str,
    ) -> Result<(), sqlx::Error> {
        let now = now_unix();
        queries
            .migration_objects_upsert(
                &crate::flash_gen::migration_objects::MigrationObjectsUpsertParams {
                    migration_id: self.migration_id.clone(),
                    obj_key: self.obj_key.clone(),
                    kind: kind.to_owned(),
                    source_ref: source_ref.to_owned(),
                    dest_ref: dest_ref.to_owned(),
                    status: "PENDING".into(),
                    created_at: now,
                    updated_at: now,
                },
            )
            .await
    }

    pub async fn set_status(
        &self,
        queries: &FlashQueries,
        status: &str,
        error: &str,
    ) -> Result<(), sqlx::Error> {
        queries
            .migration_objects_set_status(
                &crate::flash_gen::migration_objects::MigrationObjectsSetStatusParams {
                    status: status.to_owned(),
                    error: error.to_owned(),
                    updated_at: now_unix(),
                    migration_id: self.migration_id.clone(),
                    obj_key: self.obj_key.clone(),
                },
            )
            .await
    }

    pub async fn record_bytes(
        &self,
        queries: &FlashQueries,
        bytes: i64,
    ) -> Result<(), sqlx::Error> {
        queries
            .migration_objects_record_bytes(
                &crate::flash_gen::migration_objects::MigrationObjectsRecordBytesParams {
                    bytes,
                    updated_at: now_unix(),
                    migration_id: self.migration_id.clone(),
                    obj_key: self.obj_key.clone(),
                },
            )
            .await
    }
}

fn now_unix() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or_default()
}

/// rsync push + empty-delta verify for one object (no checkpoint writes).
async fn run_sync_and_verify(
    source: &CommandExecutor,
    target: &CommandExecutor,
    credentials: &SyncCredentials,
    planned: &PlannedObject,
) -> Result<SyncOutcome, String> {
    let temp = TempCredentials::create(credentials)?;
    let destination = remote_destination(&credentials.username, &credentials.connect_host, planned)
        .map_err(|error| error.to_string())?;

    // Parent of the staging prefix (…/migrations/<id>/staging) must exist:
    // OpenSSH only creates ONE remote directory level for the rsync leaf.
    let staging_parent = planned
        .dest_staging
        .rsplit_once('/')
        .map(|(parent, _)| {
            if parent.is_empty() {
                "/".into()
            } else {
                parent.to_owned()
            }
        })
        .ok_or_else(|| "staging path has no parent".to_owned())?;
    let ssh_args = ssh_exec_args(
        credentials,
        &temp.known_hosts_path(),
        &temp.key_path(),
        &staging_parent,
    )?;
    target
        .run("ssh", &ssh_args)
        .await
        .map_err(|error| failure_message(&error.to_string(), None))?;

    let identity_file = temp.key_path();
    let known_hosts_file = temp.known_hosts_path();
    let argv = sync_argv(
        planned,
        &destination,
        Path::new(&identity_file),
        Path::new(&known_hosts_file),
        credentials.ssh_port,
    )
    .map_err(|error| error.to_string())?;
    let (sender, mut receiver) = tokio::sync::mpsc::channel::<ExecStreamEvent>(128);
    let rsync = source
        .run_stream("rsync", &argv, sender)
        .await
        .map_err(|error| error.to_string())?;
    let mut accumulator = ProgressAccumulator::default();
    let mut stderr_tail = String::new();
    while let Some(event) = receiver.recv().await {
        match event {
            ExecStreamEvent::Stdout(bytes) => accumulator.push(&bytes),
            ExecStreamEvent::Stderr(bytes) => {
                accumulator.push(&bytes);
                let chunk = String::from_utf8_lossy(&bytes).into_owned();
                stderr_tail.push_str(&chunk);
                if stderr_tail.len() > 4096 {
                    let cut = stderr_tail.len() - 2048;
                    stderr_tail = stderr_tail[cut..].to_owned();
                }
            }
        }
    }
    if !rsync.success() {
        return Err(failure_message(&stderr_tail, rsync.code()));
    }
    let bytes = accumulator.finish();

    // VERIFY — dry-run checksum itemize; empty delta → VERIFIED.
    let verify = verify_argv(
        planned,
        &destination,
        Path::new(&identity_file),
        Path::new(&known_hosts_file),
        credentials.ssh_port,
    )
    .map_err(|error| error.to_string())?;
    let probe = match source.run("rsync", &verify).await {
        Ok(probe) => probe,
        Err(crate::utils::exec::ExecError::CommandFailed { code, stderr }) => {
            return Err(failure_message(&stderr, code));
        }
        Err(error) => return Err(failure_message(&error.to_string(), None)),
    };
    if has_dirty_delta(&probe.stdout) {
        return Ok(SyncOutcome::Dirty {
            message: "verify probe reported a non-empty delta".into(),
        });
    }
    Ok(SyncOutcome::Verified { bytes })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::paths::OpenOxidePaths;

    fn credentials() -> SyncCredentials {
        SyncCredentials {
            ssh_port: 22,
            username: "root".into(),
            connect_host: "10.0.0.5".into(),
            private_key: "-----BEGIN OPENSSH PRIVATE KEY-----\nb0rked\n".into(),
            host_key_openssh: "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAI0v2rQ==".into(),
        }
    }

    fn planned() -> PlannedObject {
        super::super::planner::plan_object(
            &OpenOxidePaths {
                base: "/etc/openoxide".into(),
            },
            "mig-1",
            crate::services::migration::scope::MigrationObjectKind::Volume,
            "/var/lib/docker/volumes/app-data/_data",
        )
        .unwrap()
    }

    #[test]
    fn temp_credentials_are_0600_with_pinned_known_hosts() {
        let temp = TempCredentials::create(&credentials()).unwrap();
        let mode = {
            use std::os::unix::fs::PermissionsExt;
            std::fs::metadata(temp.key_path())
                .unwrap()
                .permissions()
                .mode()
        };
        assert_eq!(mode & 0o777, 0o600, "private key must be 0600");
        let kh = std::fs::read_to_string(temp.known_hosts_path()).unwrap();
        assert_eq!(
            kh.trim(),
            "10.0.0.5 ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAI0v2rQ=="
        );
    }

    #[test]
    fn temp_credentials_reject_empty_key_and_corrupt_pin() {
        let empty = SyncCredentials {
            private_key: "  ".into(),
            ..credentials()
        };
        assert!(TempCredentials::create(&empty).is_err());
        let corrupt = SyncCredentials {
            host_key_openssh: "ssh-ed25519 AAA= $(evil)".into(),
            ..credentials()
        };
        assert!(TempCredentials::create(&corrupt).is_err());
    }

    #[test]
    fn ssh_exec_args_pin_and_mkdir_parent() {
        let args = ssh_exec_args(
            &credentials(),
            "/tmp/kh",
            "/tmp/key",
            "/etc/openoxide/migrations/mig-1/staging",
        )
        .unwrap();
        assert_eq!(args[0], "-p");
        assert!(args.contains(&"StrictHostKeyChecking=yes".to_string()));
        assert!(args.contains(&"BatchMode=yes".to_string()));
        assert!(args.contains(&"UserKnownHostsFile=/tmp/kh".to_string()));
        assert!(args.contains(&"root@10.0.0.5".to_string()));
        let mkdir = args.iter().position(|t| t == "mkdir").unwrap();
        assert_eq!(
            &args[mkdir..],
            [
                "mkdir",
                "-p",
                "--",
                "/etc/openoxide/migrations/mig-1/staging"
            ]
        );
        assert!(ssh_exec_args(&credentials(), "/tmp /kh", "/tmp/key", "/x").is_err());
    }

    #[test]
    fn dirty_delta_parser_classifies_itemize_output() {
        assert!(!has_dirty_delta(""));
        assert!(!has_dirty_delta(
            "sending incremental file list\n./\ncd+++++++++\n\ntotal size is 1,000  speedup is 1.0\n"
        ));
        assert!(!has_dirty_delta(
            "sending incremental file list\nd..t------ ./\n"
        ));
        assert!(!has_dirty_delta(
            "sending incremental file list\ncd+++++++++ .\n"
        ));
        assert!(has_dirty_delta(
            "sending incremental file list\n>f+++++++++ big-file\n"
        ));
        assert!(has_dirty_delta(
            "sending incremental file list\ncd+++++++++ sub/dir/\n"
        ));
        assert!(has_dirty_delta("*deleting orphan\n"));
        assert!(has_dirty_delta(
            "sending incremental file list\ncd+++++++++ \"dir with space/\"\n"
        ));
    }

    #[test]
    fn progress_accumulator_tracks_last_sample() {
        let mut acc = ProgressAccumulator::default();
        acc.push(b"1000  1%  1kB/s");
        acc.push(b"\r500000  40%  2MB/s\r\n");
        assert_eq!(acc.finish(), 500000);
        let mut quiet = ProgressAccumulator::default();
        quiet.push(b"rsync: connection unexpectedly closed\n");
        assert_eq!(quiet.finish(), 0);
    }

    #[test]
    fn staging_parent_for_mkdir_is_contained() {
        let p = planned();
        let parent = p.dest_staging.rsplit_once('/').unwrap().0;
        assert_eq!(parent, "/etc/openoxide/migrations/mig-1/staging");
        assert!(
            super::super::planner::assert_staging_contained(
                &OpenOxidePaths {
                    base: "/etc/openoxide".into(),
                },
                "mig-1",
                &p.dest_staging
            )
            .is_ok()
        );
    }
}
