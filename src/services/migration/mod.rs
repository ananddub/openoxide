//! Control-plane types for stateful per-service node→node migration.
//!
//! The metadata-only (whole-server) path lives in
//! [`crate::services::infrastructure::server`]. This module holds the shared
//! vocabulary that both the metadata path and the upcoming stateful
//! `MigrationRunner` (phases + per-object checkpoints) build on: the phase
//! machine, object kinds, the tamper-proof `obj_key` derivation, and the
//! [`MigrationScope`] id grouping.

mod cutover;
mod engine;
mod mounts;
mod orchestrator;
mod planner;
mod quiesce;
mod runner;
mod scope;

pub use cutover::{
    CertificateAction, CertificateOrigin, HealthVerdict, assert_not_acme_store, classify_health,
    decide_certificate_action, health_poll_attempts, may_mark_superseded,
};

pub use engine::{
    CleanEvidence, CopyMode, DbEngine, GateVerdict, decide_copy_mode, image_inspect_args,
    libsql_checkpointed, mongo_clean_lock, mysql_clean_shutdown, parse_running, pg_clean_shutdown,
    plan_dbdata_object, probe_run_args, redis_persisted, running_inspect_args, stop_args,
    version_token, versions_match,
};

pub use mounts::{
    MountIssue, MountPlanScope, default_volume_path, fetch_mount_rows, plan_service_objects,
    resolve_volume_paths, volume_inspect_args,
};

pub use orchestrator::{MigratedServiceKind, MigrationOrchestrator, StatefulMigrationRequest};

pub use planner::{
    PlanError, PlannedObject, assert_staging_contained, known_hosts_entry, plan_object,
    remote_destination, source_operand, sync_argv, verify_argv,
};
pub use quiesce::{
    CancelSignal, ReplicaRecord, ReplicaRestore, SchedulePauseRecord, decide_replica_restore,
    decide_schedule_restore,
};
pub use runner::{
    ObjectCheckpoint, SyncCredentials, SyncOutcome, TempCredentials, has_dirty_delta,
    ssh_exec_args, sync_and_verify_object,
};
pub use scope::{
    MigrationObjectKind, MigrationPhase, MigrationScope, derive_obj_key, normalize_ref,
};
