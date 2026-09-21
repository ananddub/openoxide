//! S4 data-movement planning: turn a source reference (volume mountpoint,
//! bind host path, source-code directory) into a fully-validated rsync
//! invocation against the migration-scoped staging prefix.
//!
//! Three invariants make this module load-bearing:
//!
//! 1. **Staging containment** — every destination (and therefore every
//!    `--delete`) is strictly under `<base>/migrations/<id>/staging/<obj-token>/`.
//!    [`assert_staging_contained`] is called on every destination before argv
//!    assembly; a destination outside the prefix is a hard error, never a
//!    warning.
//! 2. **No shell, no macro expansion** — arguments reach `rsync` as an argv
//!    vector (paths may freely contain spaces / quotes / `$()`); only the
//!    panel-generated `-e` string is tokenised (by rsync itself). Remote URLs
//!    are validated to contain no whitespace and no `::`/`%` so neither
//!    daemon-mode nor `--readtimeout`-style macro expansion can trigger.
//! 3. **Only `obj_key`-derived tokens in URLs** — raw source refs never leak
//!    into the destination path; the staging dir name is the deterministic
//!    hash from [`derive_obj_key`].

use std::path::PathBuf;

use crate::utils::os::rsync::{RsyncBuilder, RsyncSsh};
use crate::utils::paths::OpenOxidePaths;

use super::scope::{MigrationObjectKind, derive_obj_key, normalize_ref};

/// One unit of data to move, with its checkpoint identity and validated
/// remote staging destination.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedObject {
    pub kind: MigrationObjectKind,
    /// Normalised source reference (absolute host path / volume name).
    pub source_ref: String,
    /// Checkpoint key written to `migration_objects.obj_key`.
    pub obj_key: String,
    /// Filesystem-safe staging token (`obj_key` with `:` → `-`).
    pub obj_token: String,
    /// Canonical remote staging prefix (no trailing slash).
    pub dest_staging: String,
}

/// Errors from planning; every variant is a refusal, never a downgrade.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanError {
    InvalidReference(String),
    UnsafeIdentifier(String),
    UnusableHost(String),
    ContainmentViolation(String),
}

impl std::fmt::Display for PlanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlanError::InvalidReference(s) => write!(f, "invalid source reference: {s}"),
            PlanError::UnsafeIdentifier(s) => write!(f, "unsafe identifier: {s}"),
            PlanError::UnusableHost(s) => write!(f, "unusable connect host: {s}"),
            PlanError::ContainmentViolation(s) => write!(f, "staging containment violated: {s}"),
        }
    }
}

impl std::error::Error for PlanError {}

/// Migration ids and obj tokens must be plain `[A-Za-z0-9_-]+` so they are
/// safe inside an `rsync://`-style remote URL: no whitespace (rsync/ssh
/// tokenising), no `::` (daemon-mode parsing) and no `%` (rsync path macros).
fn is_safe_token(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

/// Plan one object. `source_ref` is normalised (traversal rejected); the
/// staging destination is derived purely from `migration_id` + the hashed
/// `obj_key`, never from the source string.
pub fn plan_object(
    paths: &OpenOxidePaths,
    migration_id: &str,
    kind: MigrationObjectKind,
    source_ref: &str,
) -> Result<PlannedObject, PlanError> {
    if !is_safe_token(migration_id) {
        return Err(PlanError::UnsafeIdentifier(migration_id.to_owned()));
    }
    let obj_key =
        derive_obj_key(kind, source_ref).ok_or_else(|| PlanError::InvalidReference(
            source_ref.to_owned(),
        ))?;
    let obj_token = obj_key.replace(':', "-");
    let dest_staging = paths.migration_staging(migration_id, &obj_token);
    assert_staging_contained(paths, migration_id, &dest_staging)?;
    Ok(PlannedObject {
        kind,
        source_ref: normalize_ref(source_ref).unwrap_or_else(|| source_ref.trim().to_owned()),
        obj_key,
        obj_token,
        dest_staging,
    })
}

/// Hard containment guard: `dest` must be exactly
/// `<base>/migrations/<id>/staging/<token>` for a safe token. Called for
/// every rsync destination — this is what makes `--delete` safe.
pub fn assert_staging_contained(
    paths: &OpenOxidePaths,
    migration_id: &str,
    dest: &str,
) -> Result<(), PlanError> {
    let prefix = format!("{}/migrations/{}/staging/", paths.base, migration_id);
    let Some(token) = dest.strip_prefix(&prefix) else {
        return Err(PlanError::ContainmentViolation(format!(
            "{dest:?} is not under {prefix:?}"
        )));
    };
    if token.is_empty()
        || token.contains('/')
        || token.contains(':')
        || token.contains("\x00")
        || !is_safe_token(token)
    {
        return Err(PlanError::ContainmentViolation(format!(
            "staging token {token:?} is not a single safe path segment"
        )));
    }
    Ok(())
}

/// Build one `known_hosts` line from a stored host-agnostic OpenSSH key form
/// (`"{algorithm} {base64-blob}"`) and the host rsync will actually connect
/// to. Validated so a corrupt DB value can never inject extra ssh options.
pub fn known_hosts_entry(connect_host: &str, openssh_key: &str) -> Result<String, PlanError> {
    let host = connect_host.trim();
    if host.is_empty()
        || host.chars().any(char::is_whitespace)
        || host.starts_with('!')
        || host.contains('#')
    {
        return Err(PlanError::UnusableHost(connect_host.to_owned()));
    }
    let key = openssh_key.trim();
    let mut tokens = key.split_whitespace();
    let (Some(algo), Some(blob), None) = (tokens.next(), tokens.next(), tokens.next()) else {
        return Err(PlanError::UnsafeIdentifier(format!(
            "host key must be exactly '{{algorithm}} {{base64}}', got {openssh_key:?}"
        )));
    };
    if algo.starts_with('-')
        || blob.contains(|c| !matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '+' | '/' | '='))
    {
        return Err(PlanError::UnsafeIdentifier(format!(
            "malformed host key component in {openssh_key:?}"
        )));
    }
    Ok(format!("{host} {algo} {blob}"))
}

/// Remote `user@host:` URL prefix. User and host come from the (already
/// strictly validated on setup) server record; re-checked here because they
/// land inside rsync's whitespace-tokenised `-e` ecosystem.
fn remote_host_part(username: &str, connect_host: &str) -> Result<String, PlanError> {
    for (name, value) in [("username", username), ("host", connect_host)] {
        if value.trim().is_empty()
            || !value.trim().chars().all(|c| !c.is_whitespace())
            || value.contains(':')
            || value.contains('%')
        {
            return Err(PlanError::UnsafeIdentifier(format!(
                "{name} {value:?} is not usable in an rsync remote URL"
            )));
        }
    }
    Ok(format!("{username}@{connect_host}"))
}

/// Full remote destination URL for a planned object (trailing slash included:
/// "copy *into* this directory", idempotent across re-syncs).
pub fn remote_destination(
    username: &str,
    connect_host: &str,
    planned: &PlannedObject,
) -> Result<String, PlanError> {
    let host_part = remote_host_part(username, connect_host)?;
    // dest_staging is panel-derived and already containment-checked; the
    // guard here protects against a future base path with whitespace.
    if planned.dest_staging.chars().any(char::is_whitespace) {
        return Err(PlanError::UnsafeIdentifier(format!(
            "staging path contains whitespace: {:?}",
            planned.dest_staging
        )));
    }
    Ok(format!("{host_part}:{}/", planned.dest_staging))
}

/// Local source operand: absolute, `::`-free (daemon-URL safety), with a
/// trailing slash so directory *contents* land inside the staging dir.
/// Arbitrary other characters (spaces, quotes, `$()`) are fine — this operand
/// travels as its own argv token, never through a shell.
pub fn source_operand(planned: &PlannedObject) -> Result<String, PlanError> {
    let src = planned.source_ref.trim();
    if src.is_empty() {
        return Err(PlanError::InvalidReference(src.to_owned()));
    }
    if normalize_ref(src).as_deref() != Some(src) {
        return Err(PlanError::InvalidReference(src.to_owned()));
    }
    if !src.starts_with('/') {
        return Err(PlanError::InvalidReference(format!(
            "source must be an absolute host path: {src:?}"
        )));
    }
    if src.contains("::") {
        return Err(PlanError::InvalidReference(format!(
            "source must not contain `::` (rsync daemon URL): {src:?}"
        )));
    }
    Ok(if src.ends_with('/') {
        src.to_owned()
    } else {
        format!("{src}/")
    })
}

/// Assemble the SYNC argv for one object: archive + metadata + resume +
/// per-object `--delete` scoped strictly to the staging prefix.
pub fn sync_argv(
    planned: &PlannedObject,
    destination: &str,
    identity_file: &PathBuf,
    known_hosts_file: &PathBuf,
    ssh_port: u16,
) -> Result<Vec<String>, PlanError> {
    let source = source_operand(planned)?;
    RsyncBuilder::new()
        .archive()
        .preserve_metadata()
        .resumable()
        .delete()
        .progress()
        .ssh(RsyncSsh {
            port: ssh_port,
            identity_file: identity_file.clone(),
            known_hosts_file: known_hosts_file.clone(),
        })
        .source(source)
        .destination(destination)
        .build_args()
        .map_err(|e| PlanError::UnsafeIdentifier(e.to_string()))
}

/// Assemble the VERIFY argv: `--dry-run --itemize-changes --checksum`; an
/// empty itemized delta (beyond the directory line) means VERIFIED.
pub fn verify_argv(
    planned: &PlannedObject,
    destination: &str,
    identity_file: &PathBuf,
    known_hosts_file: &PathBuf,
    ssh_port: u16,
) -> Result<Vec<String>, PlanError> {
    let source = source_operand(planned)?;
    RsyncBuilder::new()
        .archive()
        .preserve_metadata()
        .checksum()
        .dry_run_itemize()
        .ssh(RsyncSsh {
            port: ssh_port,
            identity_file: identity_file.clone(),
            known_hosts_file: known_hosts_file.clone(),
        })
        .source(source)
        .destination(destination)
        .build_args()
        .map_err(|e| PlanError::UnsafeIdentifier(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn paths() -> OpenOxidePaths {
        OpenOxidePaths {
            base: "/etc/openoxide".into(),
        }
    }

    fn planned_hostile() -> PlannedObject {
        plan_object(
            &paths(),
            "mig-1",
            MigrationObjectKind::Volume,
            "/var/lib/docker/volumes/dash$(touch pwned)'q;rm -rf /-mount/Ünïcödé dir/_data",
        )
        .expect("hostile-but-valid path plans")
    }

    #[test]
    fn plans_hostile_source_into_hashed_staging() {
        let p = planned_hostile();
        // Raw source never appears in the staging path or obj key.
        assert!(!p.dest_staging.contains('$'));
        assert!(!p.dest_staging.contains('\''));
        assert!(!p.dest_staging.contains(':'));
        assert!(
            p.dest_staging
                .starts_with("/etc/openoxide/migrations/mig-1/staging/VOLUME-")
        );
        assert!(p.obj_key.starts_with("VOLUME:"));
    }

    #[test]
    fn planning_is_deterministic_for_resume() {
        let a = plan_object(&paths(), "mig-1", MigrationObjectKind::Bind, "/data/app").unwrap();
        let b = plan_object(&paths(), "mig-1", MigrationObjectKind::Bind, "/data/app").unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn rejects_traversal_and_unsafe_ids() {
        assert!(matches!(
            plan_object(&paths(), "mig-1", MigrationObjectKind::Volume, "../etc"),
            Err(PlanError::InvalidReference(_))
        ));
        assert!(matches!(
            plan_object(&paths(), "mig/../1", MigrationObjectKind::Volume, "/data"),
            Err(PlanError::UnsafeIdentifier(_))
        ));
        assert!(matches!(
            plan_object(&paths(), "mig 1", MigrationObjectKind::Volume, "/data"),
            Err(PlanError::UnsafeIdentifier(_))
        ));
    }

    #[test]
    fn containment_guard_refuses_live_paths() {
        let prefix = "/etc/openoxide/migrations/mig-1/staging";
        assert!(assert_staging_contained(&paths(), "mig-1", &format!("{prefix}/VOLUME-ab")).is_ok());
        for bad in [
            "/etc/openoxide/applications/myapp",
            "/etc/openoxide/migrations/mig-1",
            "/etc/openoxide/migrations/mig-1/staging",
            "/etc/openoxide/migrations/mig-1/staging/VOLUME-ab/../../..",
            "/etc/openoxide/migrations/mig-1/staging/VOLUME-ab/more",
            "/etc/openoxide/migrations/mig-2/staging/VOLUME-ab",
            "/var/lib/docker/volumes/app/_data",
        ] {
            assert!(
                matches!(
                    assert_staging_contained(&paths(), "mig-1", bad),
                    Err(PlanError::ContainmentViolation(_))
                ),
                "{bad} must be refused"
            );
        }
    }

    #[test]
    fn known_hosts_entry_formats_and_validates() {
        let line = known_hosts_entry(
            "10.0.0.5",
            "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIKn示例=",
        );
        // Non-base64 character in blob → refused.
        assert!(matches!(line, Err(PlanError::UnsafeIdentifier(_))));
        let ok = known_hosts_entry(
            "10.0.0.5",
            "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAI0v2rQ==",
        )
        .unwrap();
        assert_eq!(ok, "10.0.0.5 ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAI0v2rQ==");
        assert!(matches!(
            known_hosts_entry("10 0.0.5", "ssh-ed25519 AAAA="),
            Err(PlanError::UnusableHost(_))
        ));
        assert!(matches!(
            known_hosts_entry("-p 2222 evil", "ssh-ed25519 AAAA="),
            Err(PlanError::UnusableHost(_))
        ));
        assert!(matches!(
            known_hosts_entry("host", "ssh-ed25519 AAAA= extra comment here"),
            Err(PlanError::UnsafeIdentifier(_))
        ));
    }

    #[test]
    fn remote_urls_reject_whitespace_and_daemon_syntax() {
        let p = plan_object(&paths(), "mig-1", MigrationObjectKind::Volume, "/data").unwrap();
        assert!(matches!(
            remote_destination("ro ot", "10.0.0.5", &p),
            Err(PlanError::UnsafeIdentifier(_))
        ));
        assert!(matches!(
            remote_destination("root", "evil::module", &p),
            Err(PlanError::UnsafeIdentifier(_))
        ));
        assert!(matches!(
            remote_destination("root", "ho%st", &p),
            Err(PlanError::UnsafeIdentifier(_))
        ));
        let url = remote_destination("root", "10.0.0.5", &p).unwrap();
        assert_eq!(
            url,
            format!(
                "root@10.0.0.5:/etc/openoxide/migrations/mig-1/staging/{}/",
                p.obj_token
            )
        );
        assert!(url.ends_with('/'));
    }

    #[test]
    fn source_operand_requires_absolute_canonical_path() {
        let p = plan_object(&paths(), "mig-1", MigrationObjectKind::Bind, "/data/app").unwrap();
        let s = source_operand(&p).unwrap();
        assert_eq!(s, "/data/app/");
        let with_slash =
            plan_object(&paths(), "mig-1", MigrationObjectKind::Bind, "/data/app/").unwrap();
        assert_eq!(source_operand(&with_slash).unwrap(), "/data/app/");
        let relative = PlannedObject {
            source_ref: "data/app".into(),
            ..p.clone()
        };
        assert!(matches!(
            source_operand(&relative),
            Err(PlanError::InvalidReference(_))
        ));
        let daemon = PlannedObject {
            source_ref: "/data::module".into(),
            ..p.clone()
        };
        assert!(matches!(
            source_operand(&daemon),
            Err(PlanError::InvalidReference(_))
        ));
    }

    #[test]
    fn sync_argv_keeps_hostile_path_single_tokenised() {
        let p = planned_hostile();
        let dest = remote_destination("root", "10.0.0.5", &p).unwrap();
        let argv = sync_argv(
            &p,
            &dest,
            &PathBuf::from("/tmp/mig/key"),
            &PathBuf::from("/tmp/mig/known_hosts"),
            22,
        )
        .unwrap();
        // The full hostile source is exactly ONE argv token.
        assert!(argv.contains(&format!(
            "/var/lib/docker/volumes/dash$(touch pwned)'q;rm -rf /-mount/Ünïcödé dir/_data/"
        )));
        assert!(argv.contains(&"--delete".to_string()));
        assert!(argv.contains(&"--append-verify".to_string()));
        assert!(argv.contains(&"--info=progress2".to_string()));
        let dashdash = argv.iter().position(|t| t == "--").unwrap();
        // Only source + destination follow `--`.
        assert_eq!(argv.len(), dashdash + 3);
        // `-e` value is whitespace-tokenised by rsync itself: the embedded
        // temp-file paths must be whitespace-free (RsyncSsh::render enforces).
        let e_idx = argv.iter().position(|t| t == "-e").unwrap();
        assert!(argv[e_idx + 1].starts_with("ssh -p 22 -i /tmp/mig/key"));
        assert!(!argv[e_idx + 1].contains("$("));
    }

    #[test]
    fn verify_argv_is_dry_run_checksum_only() {
        let p = plan_object(&paths(), "mig-1", MigrationObjectKind::Volume, "/data").unwrap();
        let dest = remote_destination("root", "10.0.0.5", &p).unwrap();
        let argv = verify_argv(
            &p,
            &dest,
            &PathBuf::from("/tmp/mig/key"),
            &PathBuf::from("/tmp/mig/known_hosts"),
            2222,
        )
        .unwrap();
        assert!(argv.contains(&"--dry-run".to_string()));
        assert!(argv.contains(&"--itemize-changes".to_string()));
        assert!(argv.contains(&"--checksum".to_string()));
        assert!(!argv.contains(&"--delete".to_string()));
        assert!(!argv.contains(&"--append-verify".to_string()));
        assert!(argv.contains(&"ssh -p 2222 -i /tmp/mig/key -o UserKnownHostsFile=/tmp/mig/known_hosts -o StrictHostKeyChecking=yes -o BatchMode=yes".to_string()));
    }
}
