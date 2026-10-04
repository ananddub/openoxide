//! S5 stateful-engine policy: the raw data-dir copy path is gated, never
//! assumed. An engine's on-disk format may only be rsynced to another host
//! when (a) the container is verifiably stopped (clean shutdown evidence per
//! engine), and (b) source and target run the **identical** engine version.
//! Any gate unmet ⇒ [`CopyMode::Dump`] (the logical-dump fallback via the
//! existing `DatabaseDumper`) — the gate, not the speed, is the feature.
//!
//! All functions here are pure argv-builders / output classifiers so the
//! safety logic is unit-testable without two live VPS; the async glue feeds
//! them real `docker inspect` / `pg_controldata` output.

use super::planner::{PlanError, PlannedObject, plan_object};
use super::scope::MigrationObjectKind;

use crate::utils::paths::OpenOxidePaths;

/// The six managed engines (`service_database.db_type`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DbEngine {
    Postgres,
    Mysql,
    Mariadb,
    Mongo,
    Redis,
    Libsql,
}

impl DbEngine {
    pub fn from_db_type(db_type: &str) -> Option<Self> {
        match db_type.trim().to_ascii_uppercase().as_str() {
            "POSTGRES" => Some(DbEngine::Postgres),
            "MYSQL" => Some(DbEngine::Mysql),
            "MARIADB" => Some(DbEngine::Mariadb),
            "MONGO" => Some(DbEngine::Mongo),
            "REDIS" => Some(DbEngine::Redis),
            "LIBSQL" => Some(DbEngine::Libsql),
            _ => None,
        }
    }

    /// Whether ANY raw-copy path exists for this engine (even behind a
    /// version lock). REDIS/LIBSQL are file-format-stable enough to copy
    /// warm; MONGO raw copies additionally need the clean-lock evidence.
    pub const fn raw_copy_possible(&self) -> bool {
        matches!(
            self,
            DbEngine::Postgres
                | DbEngine::Mysql
                | DbEngine::Mariadb
                | DbEngine::Mongo
                | DbEngine::Redis
                | DbEngine::Libsql
        )
    }

    /// Data directory inside the engine's canonical image layout.
    pub const fn data_dir(&self) -> &'static str {
        match self {
            DbEngine::Postgres => "/var/lib/postgresql/data",
            DbEngine::Mysql => "/var/lib/mysql",
            DbEngine::Mariadb => "/var/lib/mysql",
            DbEngine::Mongo => "/data/db",
            DbEngine::Redis => "/data",
            DbEngine::Libsql => "/data",
        }
    }
}

/// Chosen movement strategy for one database after the gates ran.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CopyMode {
    /// Stop-locked raw data-dir rsync (identical version + clean shutdown).
    Raw,
    /// Logical dump/restore (`DatabaseDumper`) — safe across versions.
    Dump,
}

/// Verdict of the stop / clean-shutdown / version gates.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GateVerdict {
    Pass,
    /// Engine refuses raw copy outright (unknown db_type, no clean path).
    RefuseRawCopy {
        reason: String,
    },
    /// Gate failed at runtime (still running, dirty, version mismatch).
    Blocked {
        reason: String,
    },
}

/// Normalise a docker image reference to a comparable version token. Only a
/// pinned tag counts as a version; `latest`/missing tags are not comparable
/// and return `None`, which forces the dump fallback.
pub fn version_token(image: &str) -> Option<String> {
    let image = image.trim();
    if image.is_empty() {
        return None;
    }
    // Strip an optional registry host (first segment containing `.`/`:`
    // without being the tag separator).
    let (repository, tag) = match image.rsplit_once(':') {
        Some((repo, tag)) if !repo.is_empty() && !tag.is_empty() && !tag.contains('/') => {
            (repo, Some(tag))
        }
        _ => (image, None),
    };
    let tag = tag?;
    if tag.eq_ignore_ascii_case("latest") || tag == "master" || tag.eq_ignore_ascii_case("edge") {
        return None;
    }
    Some(format!("{repository}@{tag}"))
}

/// True only when both sides report an identical pinned version token.
pub fn versions_match(source_image: &str, target_image: &str) -> bool {
    match (version_token(source_image), version_token(target_image)) {
        (Some(a), Some(b)) => a == b,
        _ => false,
    }
}

/// Classify `docker inspect -f '{{.State.Running}}'` output. Anything other
/// than an exact `false` is treated as "still running" (fail-closed).
pub fn parse_running(inspect_output: &str) -> bool {
    inspect_output.trim() != "false"
}

/// PG gate: `pg_controldata` must report `Database cluster state: shut down`
/// (the "in production" / crash states refuse a raw copy).
pub fn pg_clean_shutdown(controldata_output: &str) -> bool {
    controldata_output.lines().any(|line| {
        let normalized = line.trim_start();
        match normalized.strip_prefix("Database cluster state:") {
            Some(state) => state.trim().eq_ignore_ascii_case("shut down"),
            None => false,
        }
    })
}

/// MySQL/MariaDB gate: the tail of the engine error log must contain the
/// final clean-shutdown record (`Shutdown complete` / `Normal shutdown`).
/// A crash log (`mysqld got signal`) never carries it.
pub fn mysql_clean_shutdown(error_log_tail: &str) -> bool {
    let lowered = error_log_tail.to_ascii_lowercase();
    lowered.contains("shutdown complete") || lowered.contains("normal shutdown")
}

/// Mongo gate: `mongod.lock` must be empty (clean) and a `mongod.lock`-bearing
/// canonical layout must be present. A non-empty lock = dirty → refuse.
pub fn mongo_clean_lock(mongod_lock_contents: &str) -> bool {
    mongod_lock_contents.trim().is_empty()
}

/// Redis gate: AOF must carry the trailing `EOF` marker, or a completed RDB
/// (`dump.rdb`) must exist. Contents-only classification.
pub fn redis_persisted(aof_tail: Option<&str>, rdb_present: bool) -> bool {
    if rdb_present {
        return true;
    }
    match aof_tail {
        Some(tail) => tail.trim_end().ends_with("*EOF"),
        None => false,
    }
}

/// LibSQL/sqld gate: the `-wal` sidecar must be absent or zero-length
/// (checkpointed) for a consistent raw copy.
pub fn libsql_checkpointed(wal_size_bytes: Option<u64>) -> bool {
    matches!(wal_size_bytes, None | Some(0))
}

/// Full gate for one database. `clean_evidence` is the engine-specific
/// probe output (controldata / log tail / lock contents / (aof, rdb) / wal).
pub fn decide_copy_mode(
    engine: DbEngine,
    still_running: bool,
    source_image: &str,
    target_image: &str,
    clean_evidence: CleanEvidence<'_>,
) -> GateVerdict {
    if !engine.raw_copy_possible() {
        return GateVerdict::RefuseRawCopy {
            reason: format!("{engine:?} has no raw-copy path"),
        };
    }
    if still_running {
        return GateVerdict::Blocked {
            reason: "database container is still running after stop (fail-closed)".into(),
        };
    }
    if !versions_match(source_image, target_image) {
        return GateVerdict::Blocked {
            reason: format!(
                "engine version mismatch: source {source_image:?} vs target {target_image:?} \
                 (pinned identical tags required) — fall back to logical dump"
            ),
        };
    }
    let clean = match clean_evidence {
        CleanEvidence::PgControldata(out) => pg_clean_shutdown(out),
        CleanEvidence::MySqlLogTail(out) => mysql_clean_shutdown(out),
        CleanEvidence::MongoLock(out) => mongo_clean_lock(out),
        CleanEvidence::Redis {
            aof_tail,
            rdb_present,
        } => redis_persisted(aof_tail, rdb_present),
        CleanEvidence::LibsqlWal(wal_size) => libsql_checkpointed(wal_size),
    };
    if !clean {
        return GateVerdict::Blocked {
            reason: format!("{engine:?} clean-shutdown evidence missing — refuse raw copy"),
        };
    }
    GateVerdict::Pass
}

/// Engine-specific clean-shutdown probe inputs.
#[derive(Clone, Copy, Debug)]
pub enum CleanEvidence<'a> {
    PgControldata(&'a str),
    MySqlLogTail(&'a str),
    MongoLock(&'a str),
    Redis {
        aof_tail: Option<&'a str>,
        rdb_present: bool,
    },
    LibsqlWal(Option<u64>),
}

/// argv to stop a database container with a generous timeout and verify it
/// stayed down (`docker stop` then `docker inspect` are issued separately by
/// the caller).
pub fn stop_args(container: &str, timeout_secs: u32) -> Result<Vec<String>, PlanError> {
    validate_container(container)?;
    Ok(vec![
        "stop".into(),
        "--time".into(),
        timeout_secs.to_string(),
        container.to_owned(),
    ])
}

/// argv for the state probe.
pub fn running_inspect_args(container: &str) -> Result<Vec<String>, PlanError> {
    validate_container(container)?;
    Ok(vec![
        "inspect".into(),
        "-f".into(),
        "{{.State.Running}}".into(),
        container.to_owned(),
    ])
}

/// argv for the image probe (feeds the version gate).
pub fn image_inspect_args(container: &str) -> Result<Vec<String>, PlanError> {
    validate_container(container)?;
    Ok(vec![
        "inspect".into(),
        "-f".into(),
        "{{.Config.Image}}".into(),
        container.to_owned(),
    ])
}

/// argv for a clean-shutdown probe run INSIDE the stopped container
/// (`docker start --attach` is avoided: probes execute against the mounted
/// volume via a throwaway `docker run` is caller's job; this only builds the
/// exec-shaped argv for the engine command).
pub fn probe_run_args(image: &str, volume_mount: &str, inner_argv: &[String]) -> Vec<String> {
    let mut a = vec![
        "run".into(),
        "--rm".into(),
        "-v".into(),
        volume_mount.to_owned(),
        image.to_owned(),
    ];
    a.extend(inner_argv.iter().cloned());
    a
}

fn validate_container(container: &str) -> Result<(), PlanError> {
    if container.trim().is_empty()
        || container.chars().any(char::is_whitespace)
        || container.starts_with('-')
    {
        return Err(PlanError::UnsafeIdentifier(format!(
            "invalid container name {container:?}"
        )));
    }
    Ok(())
}

/// Plan the DBDATA staging object for one stopped database volume.
pub fn plan_dbdata_object(
    paths: &OpenOxidePaths,
    migration_id: &str,
    host_data_path: &str,
) -> Result<PlannedObject, PlanError> {
    plan_object(
        paths,
        migration_id,
        MigrationObjectKind::DbData,
        host_data_path,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_mapping_and_data_dirs() {
        assert_eq!(DbEngine::from_db_type("postgres"), Some(DbEngine::Postgres));
        assert_eq!(DbEngine::from_db_type("LIBSQL"), Some(DbEngine::Libsql));
        assert_eq!(DbEngine::from_db_type("CASSANDRA"), None);
        assert_eq!(DbEngine::Postgres.data_dir(), "/var/lib/postgresql/data");
    }

    #[test]
    fn version_tokens_require_pinned_tags() {
        assert_eq!(
            version_token("postgres:16.4-alpine"),
            Some("postgres@16.4-alpine".into())
        );
        assert_eq!(
            version_token("registry.example.com/library/postgres:16.4"),
            Some("registry.example.com/library/postgres@16.4".into())
        );
        assert_eq!(version_token("postgres"), None);
        assert_eq!(version_token("postgres:latest"), None);
        assert_eq!(version_token("redis:edge"), None);
        assert!(
            !versions_match("mongo:7.0.12", "docker.io/mongo:7.0.12-x"),
            "different repositories must not match"
        );
        assert!(versions_match("postgres:16.4", "postgres:16.4"));
        assert!(!versions_match("postgres:16.4", "postgres:17.0"));
        assert!(!versions_match("postgres:latest", "postgres:latest"));
    }

    #[test]
    fn running_probe_fails_closed() {
        assert!(parse_running("true\n"));
        assert!(!parse_running("false\n"));
        // Unparseable / error output must count as "still running".
        assert!(parse_running("error: no such container"));
        assert!(parse_running(""));
    }

    #[test]
    fn postgres_gate_needs_shut_down_state() {
        assert!(pg_clean_shutdown(
            "pg_control version number:            1300\nDatabase cluster state:  shut down\n"
        ));
        assert!(!pg_clean_shutdown(
            "Database cluster state:  in production\n"
        ));
        assert!(!pg_clean_shutdown(
            "Database cluster state:           shut down in recovery\n"
        ));
        assert!(!pg_clean_shutdown(
            "bash: pg_controldata: command not found"
        ));
    }

    #[test]
    fn mysql_gate_needs_shutdown_complete() {
        assert!(mysql_clean_shutdown(
            "2026-09-20T10:00:00Z 0 [System] [MY-010116] [Server] /usr/sbin/mysqld (initiated by: root) [MY-010910] [Server] /usr/sbin/mysqld: Shutdown complete [MY-013172]."
        ));
        assert!(!mysql_clean_shutdown(
            "2026-09-20T09:00:00Z 0 [System] [MY-010116] [Server] mysqld ready for connections"
        ));
        assert!(!mysql_clean_shutdown("mysqld got signal 6"));
    }

    #[test]
    fn mongo_redis_libsql_gates() {
        assert!(mongo_clean_lock(""));
        assert!(mongo_clean_lock("\n"));
        assert!(!mongo_clean_lock("12\n"));
        assert!(redis_persisted(Some("...\n*EOF\n"), false));
        assert!(!redis_persisted(Some("...*MULTI"), false));
        assert!(redis_persisted(None, true));
        assert!(libsql_checkpointed(None));
        assert!(libsql_checkpointed(Some(0)));
        assert!(!libsql_checkpointed(Some(4096)));
    }

    #[test]
    fn decide_copy_mode_matrix() {
        // Version mismatch → Blocked (dump fallback), even when clean.
        let v = decide_copy_mode(
            DbEngine::Postgres,
            false,
            "postgres:16.4",
            "postgres:17.0",
            CleanEvidence::PgControldata("Database cluster state:  shut down"),
        );
        assert!(matches!(v, GateVerdict::Blocked { .. }));
        // Still running → Blocked even with matching versions + clean state.
        let v = decide_copy_mode(
            DbEngine::Postgres,
            true,
            "postgres:16.4",
            "postgres:16.4",
            CleanEvidence::PgControldata("Database cluster state:  shut down"),
        );
        assert!(matches!(v, GateVerdict::Blocked { .. }));
        // All gates satisfied → raw copy allowed.
        let v = decide_copy_mode(
            DbEngine::Postgres,
            false,
            "postgres:16.4",
            "postgres:16.4",
            CleanEvidence::PgControldata("Database cluster state:  shut down"),
        );
        assert_eq!(v, GateVerdict::Pass);
        // Matching versions but DIRTY → Blocked.
        let v = decide_copy_mode(
            DbEngine::Mysql,
            false,
            "mysql:8.4.2",
            "mysql:8.4.2",
            CleanEvidence::MySqlLogTail("mysqld got signal 6"),
        );
        assert!(matches!(v, GateVerdict::Blocked { .. }));
    }

    #[test]
    fn docker_argv_builders_validate_names() {
        assert_eq!(
            stop_args("db-pg", 120).unwrap(),
            vec!["stop", "--time", "120", "db-pg"]
        );
        assert!(stop_args("", 10).is_err());
        assert!(stop_args("-la", 10).is_err());
        assert!(stop_args("my db", 10).is_err());
        assert_eq!(
            running_inspect_args("db-pg").unwrap(),
            vec!["inspect", "-f", "{{.State.Running}}", "db-pg"]
        );
        assert_eq!(
            image_inspect_args("db-pg").unwrap(),
            vec!["inspect", "-f", "{{.Config.Image}}", "db-pg"]
        );
        let args = probe_run_args(
            "postgres:16.4",
            "pg-data:/var/lib/postgresql/data",
            &["sync".into(), "-c".into(), "0".into()],
        );
        assert_eq!(
            args,
            vec![
                "run",
                "--rm",
                "-v",
                "pg-data:/var/lib/postgresql/data",
                "postgres:16.4",
                "sync",
                "-c",
                "0"
            ]
        );
    }

    #[test]
    fn dbdata_object_lands_in_staging() {
        let paths = OpenOxidePaths {
            base: "/etc/openoxide".into(),
        };
        let planned =
            plan_dbdata_object(&paths, "mig7", "/var/lib/docker/volumes/svc-42-data/_data")
                .unwrap();
        assert!(planned.obj_key.starts_with("DBDATA:"));
        assert!(
            planned
                .dest_staging
                .starts_with("/etc/openoxide/migrations/mig7/staging/DBDATA-")
        );
    }
}
