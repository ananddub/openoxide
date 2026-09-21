//! Stateful per-service migration control plane: the phase loop that ties
//! the S3–S7 primitives (`planner`/`mounts`/`runner`/`quiesce`/`cutover`)
//! into one resumable execution driven by the `application_migrations` row.
//!
//! Phase discipline (`PREFLIGHT → QUIESCE → SYNC → VERIFY → APPLY →
//! REPOINT → REDEPLOY → HEALTH → SUPERSEDED`):
//!
//! * **Cancel is edge-polite and bounded**: it is honoured only at
//!   phase/object boundaries while the row is still `RUNNING` (pre-REPOINT).
//!   A cancel re-enables paused schedules and redeploys the *source* — the
//!   metadata never moved, so the source is still authoritative.
//! * **APPLY is the first live-target mutation**: verified staging bytes are
//!   promoted with a local `rsync -aHAX`; a destination that already holds
//!   foreign content is refused (never overwritten), per the plan's
//!   "rename-after-snapshot; foreign target data → refuse" rule, simplified
//!   to refuse-unconditionally in this slice.
//! * **The queue gate is released at REDEPLOY**: the row flips
//!   `RUNNING → APPLYING` exactly when the REPOINT metadata flips land, so
//!   the orchestrator's own REDEPLOY deployment is no longer held back by
//!   the `NOT EXISTS (… status='RUNNING')` dispatch gate, and
//!   `ApplicationMigrationsRequestCancel` (which matches `RUNNING` only)
//!   naturally refuses from that point on — the cutover is committed.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use crate::db::queries::FlashQueries;
use crate::flash_gen::application_migrations::ApplicationMigrationsUpdatePhaseParams;
use crate::flash_gen::models::MigrationObjects;
use crate::flash_gen::servers::{
    ServersMigrateApplicationServersByIdsParams, ServersMigrateBuildAssignmentsByIdsParams,
    ServersMigrateComposeServersByIdsParams, ServersMigrateSchedulesByIdsParams,
};
use crate::services::compute::application::management::remote_executor;
use crate::services::compute::application::{ApplicationOperation, ApplicationService};
use crate::services::compute::compose::{ComposeOperation, ComposeService};
use crate::utils::exec::CommandExecutor;
use crate::utils::paths::OpenOxidePaths;

use super::cutover::{HealthVerdict, classify_health, health_poll_attempts, may_mark_superseded};
use super::mounts::plan_service_objects;
use super::planner::PlannedObject;
use super::quiesce::{CancelSignal, SchedulePauseRecord, decide_schedule_restore};
use super::runner::{ObjectCheckpoint, SyncCredentials, SyncOutcome, sync_and_verify_object};
use super::scope::MigrationPhase;

/// Which canonical `services` row a stateful migration acts on. DATABASE
/// services are refused at the service layer until the S5 engine path is
/// wired into this loop — this slice moves APPLICATION and COMPOSE state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MigratedServiceKind {
    Application,
    Compose,
}

/// Everything one orchestrator run needs; constructed by
/// `RemoteServerService::migrate_service` and moved into the spawned task.
#[derive(Clone, Debug)]
pub struct StatefulMigrationRequest {
    pub migration_id: String,
    pub service_id: i64,
    pub kind: MigratedServiceKind,
    /// `services.app_name` — the on-disk directory name for path planning.
    pub app_name: String,
    pub source_server_id: i64,
    pub target_server_id: i64,
    pub organization_id: i64,
}

/// HEALTH polling budget for the redeployed target workload.
const HEALTH_INTERVAL: Duration = Duration::from_secs(5);
const HEALTH_TIMEOUT: Duration = Duration::from_secs(600);

/// Terminal deployment statuses that mean the REDEPLOY never completed.
fn deployment_failed(status: &str) -> bool {
    matches!(status, "FAILED" | "CANCELLED" | "ERROR")
}

/// SYNC progress mapping: the sync window spans 10–80% of the migration.
pub fn sync_progress(done: i64, total: i64) -> i64 {
    if total <= 0 {
        return 80;
    }
    10 + (70 * done.min(total)) / total
}

/// SYNC-phase resume: an object checkpoint already at these statuses needs
/// no re-rsync (the runner lifecycle ends at VERIFIED; APPLIED/DONE imply a
/// later pass).
pub fn sync_done(status: &str) -> bool {
    matches!(status, "VERIFIED" | "APPLIED" | "DONE")
}

/// APPLY-phase resume: only `VERIFIED` still needs promotion; APPLIED/DONE
/// already mutated the live target.
pub fn apply_needed(status: &str) -> bool {
    status == "VERIFIED"
}

/// SUPERSEDED accounting: checkpoints not yet in a settled state. The
/// runner/APPLY lifecycle settles at VERIFIED→APPLIED (nothing in this
/// slice writes DONE), so both count as settled for the gate.
pub fn unsettled_objects(rows: &[MigrationObjects]) -> i64 {
    rows.iter()
        .filter(|row| !matches!(row.status.as_str(), "VERIFIED" | "APPLIED" | "DONE"))
        .count() as i64
}

/// Extract the docker volume name from a canonical mountpoint
/// (`/var/lib/docker/volumes/<name>/_data`). Planning only ever produces
/// this shape because unresolvable mountpoints surface as `MountIssue`s.
pub fn volume_name_from_mountpoint(path: &str) -> Option<String> {
    let trimmed = path.trim_end_matches('/');
    if !trimmed.ends_with("/_data") {
        return None;
    }
    let name = trimmed[..trimmed.len() - "/_data".len()].rsplit('/').next()?;
    if name.is_empty() || name.contains('/') {
        return None;
    }
    // Reuse the planner's identifier discipline.
    super::mounts::volume_inspect_args(name).ok()?;
    Some(name.to_owned())
}

/// Outcome of one health poll against the redeployed target workload. A
/// DONE deployment plus a running service is the only pass; a failed
/// deployment fails immediately; anything else keeps waiting.
pub fn health_verdict(app_status: &str, deployment_status: Option<&str>) -> HealthVerdict {
    if let Some(status) = deployment_status {
        if deployment_failed(status) {
            return HealthVerdict::Down {
                reason: format!("redeploy deployment ended as {status}"),
            };
        }
    }
    let probe_ok = matches!(deployment_status, Some("DONE"));
    classify_health(Some(&app_status.to_lowercase()), probe_ok.then_some(true))
}

/// Why the loop stopped early. `Cancelled` is only reachable pre-REPOINT.
enum Flow {
    Cancelled,
    Failed(String),
}

impl From<sqlx::Error> for Flow {
    fn from(error: sqlx::Error) -> Self {
        Flow::Failed(error.to_string())
    }
}

/// Mutable cross-phase context the terminal handlers need for restore.
struct RunContext {
    /// Schedules observed (and paused) during QUIESCE.
    paused: Vec<SchedulePauseRecord>,
    /// Every schedule row that REPOINT moved with the service.
    schedule_ids: Vec<i64>,
    /// True once the metadata flips landed — cancel/rollback windows closed.
    repointed: bool,
}

/// Drives one `application_migrations` row from PREFLIGHT to SUPERSEDED (or
/// a terminal FAILED/CANCELLED) on a background task.
pub struct MigrationOrchestrator {
    queries: Arc<FlashQueries>,
    applications: Arc<ApplicationService>,
    compose: Arc<ComposeService>,
}

impl MigrationOrchestrator {
    pub fn new(
        queries: Arc<FlashQueries>,
        applications: Arc<ApplicationService>,
        compose: Arc<ComposeService>,
    ) -> Self {
        Self {
            queries,
            applications,
            compose,
        }
    }

    /// Entry point for the spawned task. Writes the terminal row state and
    /// performs the restore best-effort; never panics and never propagates.
    pub async fn run(&self, req: StatefulMigrationRequest, cancel: CancelSignal) {
        let mut ctx = RunContext {
            paused: Vec::new(),
            schedule_ids: Vec::new(),
            repointed: false,
        };
        match self.drive(&req, &cancel, &mut ctx).await {
            Ok(()) => {
                tracing::info!(migration_id = %req.migration_id, "stateful migration succeeded");
            }
            Err(Flow::Cancelled) => {
                tracing::info!(migration_id = %req.migration_id, "stateful migration cancelled");
                self.on_cancelled(&req, &ctx).await;
            }
            Err(Flow::Failed(message)) => {
                tracing::error!(
                    migration_id = %req.migration_id,
                    %message,
                    "stateful migration failed"
                );
                self.on_failed(&req, &message, &ctx).await;
            }
        }
    }

    #[allow(clippy::too_many_lines)]
    async fn drive(
        &self,
        req: &StatefulMigrationRequest,
        cancel: &CancelSignal,
        ctx: &mut RunContext,
    ) -> Result<(), Flow> {
        // ---- PREFLIGHT ----------------------------------------------------
        self.set_phase(req, MigrationPhase::Preflight, "RUNNING", 0)
            .await?;
        let source = CommandExecutor::Remote(
            remote_executor(&self.queries.pool, req.source_server_id)
                .await
                .map_err(Flow::Failed)?,
        );
        let target = CommandExecutor::Remote(
            remote_executor(&self.queries.pool, req.target_server_id)
                .await
                .map_err(Flow::Failed)?,
        );
        let credentials = self.sync_credentials(req.target_server_id).await?;
        self.require_rsync(&source, "source").await?;
        self.require_rsync(&target, "target").await?;
        self.check_cancel(req, cancel).await?;

        // ---- QUIESCE ------------------------------------------------------
        self.set_phase(req, MigrationPhase::Quiesce, "RUNNING", 5)
            .await?;
        self.quiesce(req, ctx).await?;
        self.check_cancel(req, cancel).await?;

        // ---- SYNC ---------------------------------------------------------
        self.set_phase(req, MigrationPhase::Sync, "RUNNING", 10)
            .await?;
        let planned = self.plan(req, &source).await?;
        let existing = self
            .queries
            .migration_objects_list_by_migration(&req.migration_id)
            .await?;
        let mut statuses: HashMap<String, String> = existing
            .into_iter()
            .map(|row| (row.obj_key, row.status))
            .collect();
        let total = planned.len() as i64;
        let mut done = 0_i64;
        for object in &planned {
            let status = statuses
                .get(&object.obj_key)
                .map(String::as_str)
                .unwrap_or("MISSING");
            if sync_done(status) {
                done += 1;
                continue;
            }
            self.check_cancel(req, cancel).await?;
            let checkpoint = ObjectCheckpoint {
                migration_id: req.migration_id.clone(),
                obj_key: object.obj_key.clone(),
            };
            checkpoint
                .ensure_exists(
                    &self.queries,
                    object.kind.as_str(),
                    &object.source_ref,
                    &object.dest_staging,
                )
                .await?;
            if status == "FAILED" {
                checkpoint.set_status(&self.queries, "PENDING", "").await?;
            }
            match sync_and_verify_object(
                &source,
                &target,
                &self.queries,
                &req.migration_id,
                &credentials,
                object,
            )
            .await
            {
                SyncOutcome::Verified { .. } => {
                    done += 1;
                    statuses.insert(object.obj_key.clone(), "VERIFIED".into());
                    self.set_phase(req, MigrationPhase::Sync, "RUNNING", sync_progress(done, total))
                        .await?;
                }
                SyncOutcome::Dirty { message } | SyncOutcome::Failed { message } => {
                    return Err(Flow::Failed(format!(
                        "sync of {} ({}) failed: {message}",
                        object.obj_key, object.source_ref
                    )));
                }
            }
        }

        // ---- VERIFY -------------------------------------------------------
        self.set_phase(req, MigrationPhase::Verify, "RUNNING", 80)
            .await?;
        for object in &planned {
            if !sync_done(statuses.get(&object.obj_key).map(String::as_str).unwrap_or("")) {
                return Err(Flow::Failed(format!(
                    "{} never reached VERIFIED",
                    object.obj_key
                )));
            }
        }
        self.check_cancel(req, cancel).await?;

        // ---- APPLY (first live-target mutation; forward-only afterwards) --
        self.set_phase(req, MigrationPhase::Apply, "RUNNING", 82)
            .await?;
        self.apply(req, &target, &planned, &mut statuses).await?;

        // Cancel is no longer offered from here: bytes are on the target's
        // live paths and the DB flag can only match RUNNING rows anyway.

        // ---- REPOINT ------------------------------------------------------
        self.set_phase(req, MigrationPhase::Repoint, "RUNNING", 90)
            .await?;
        self.repoint(req, ctx).await?;
        ctx.repointed = true;
        // Release the queue dispatch gate for our own REDEPLOY: RUNNING →
        // APPLYING while the remaining phases run.
        self.set_phase(req, MigrationPhase::Redeploy, "APPLYING", 91)
            .await?;

        // ---- REDEPLOY -----------------------------------------------------
        let deployment_id = match req.kind {
            MigratedServiceKind::Application => self
                .applications
                .run_operation(req.service_id, ApplicationOperation::Redeploy, 0)
                .await
                .map(|result| result.deployment_id)?,
            MigratedServiceKind::Compose => self
                .compose
                .run_operation(req.service_id, ComposeOperation::Redeploy, 0)
                .await
                .map(|result| result.deployment_id)?,
        };

        // ---- HEALTH -------------------------------------------------------
        self.set_phase(req, MigrationPhase::Health, "APPLYING", 92)
            .await?;
        self.health(req, deployment_id).await?;

        // ---- SUPERSEDED ---------------------------------------------------
        self.set_phase(req, MigrationPhase::Superseded, "APPLYING", 98)
            .await?;
        let rows = self
            .queries
            .migration_objects_list_by_migration(&req.migration_id)
            .await?;
        if !may_mark_superseded(MigrationPhase::Superseded, unsettled_objects(&rows)) {
            return Err(Flow::Failed(
                "checkpoints unsettled at SUPERSEDED".into(),
            ));
        }
        // Nothing on the source is deleted: the workload was stopped at
        // QUIESCE and its rows now point at the target. Staging prefixes are
        // left for manual/operator cleanup (deletion safety).
        self.queries
            .application_migrations_finish_success(&req.migration_id)
            .await?;
        Ok(())
    }

    async fn set_phase(
        &self,
        req: &StatefulMigrationRequest,
        phase: MigrationPhase,
        status: &str,
        progress: i64,
    ) -> Result<(), Flow> {
        self.queries
            .application_migrations_update_phase(&ApplicationMigrationsUpdatePhaseParams {
                phase: phase.as_str().to_owned(),
                status: status.to_owned(),
                progress,
                id: req.migration_id.clone(),
            })
            .await
            .map_err(Into::into)
    }

    /// Boundary cancel probe: the DB flag (`cancel_requested`, mirrored by
    /// the CancelMigration RPC) or this process's token.
    async fn check_cancel(
        &self,
        req: &StatefulMigrationRequest,
        cancel: &CancelSignal,
    ) -> Result<(), Flow> {
        let row = self
            .queries
            .application_migrations_get(&req.migration_id)
            .await?;
        if cancel.is_cancelled(row.cancel_requested != 0) {
            return Err(Flow::Cancelled);
        }
        Ok(())
    }

    async fn fetch_schedules(
        &self,
        req: &StatefulMigrationRequest,
    ) -> Result<Vec<crate::flash_gen::models::Schedules>, Flow> {
        let rows = match req.kind {
            MigratedServiceKind::Application => {
                self.queries.list_schedules_by_application(req.service_id).await?
            }
            MigratedServiceKind::Compose => {
                self.queries.list_schedules_by_compose(req.service_id).await?
            }
        };
        Ok(rows)
    }

    /// Pause the service's schedules (so nothing fires mid-migration), then
    /// stop the source workload.
    async fn quiesce(
        &self,
        req: &StatefulMigrationRequest,
        ctx: &mut RunContext,
    ) -> Result<(), Flow> {
        for schedule in self.fetch_schedules(req).await? {
            ctx.schedule_ids.push(schedule.id);
            ctx.paused.push(SchedulePauseRecord {
                schedule_id: schedule.id,
                was_enabled: schedule.enabled != 0,
            });
            if schedule.enabled != 0 {
                self.queries.toggle_schedule(0, schedule.id).await?;
            }
        }
        match req.kind {
            MigratedServiceKind::Application => {
                self.applications.stop_or_cancel_smart(req.service_id).await?;
            }
            MigratedServiceKind::Compose => {
                self.compose.stop_or_cancel_smart(req.service_id).await?;
            }
        }
        Ok(())
    }

    /// Build the containment-checked object plan for the single service.
    async fn plan(
        &self,
        req: &StatefulMigrationRequest,
        source: &CommandExecutor,
    ) -> Result<Vec<PlannedObject>, Flow> {
        let paths = OpenOxidePaths::from_env();
        let (application_ids, compose_ids): (Vec<i64>, Vec<i64>) = match req.kind {
            MigratedServiceKind::Application => (vec![req.service_id], Vec::new()),
            MigratedServiceKind::Compose => (Vec::new(), vec![req.service_id]),
        };
        let mut app_names = HashMap::new();
        let mut compose_names = HashMap::new();
        match req.kind {
            MigratedServiceKind::Application => {
                app_names.insert(req.service_id, req.app_name.clone());
            }
            MigratedServiceKind::Compose => {
                compose_names.insert(req.service_id, req.app_name.clone());
            }
        }
        let (planned, issues) = plan_service_objects(
            &paths,
            &req.migration_id,
            &self.queries,
            source,
            &application_ids,
            &compose_ids,
            &app_names,
            &compose_names,
        )
        .await
        .map_err(|error| Flow::Failed(error.to_string()))?;
        if !issues.is_empty() {
            // Fail closed: an unresolved volume or escaping bind means the
            // byte inventory is incomplete; the tar-stream fallback is not
            // implemented, so syncing "whatever we can" would lose state.
            let detail = issues
                .iter()
                .map(|issue| format!("{}: {}", issue.volume_name, issue.reason))
                .collect::<Vec<_>>()
                .join("; ");
            return Err(Flow::Failed(format!(
                "mount inventory unresolved on source — nothing was synced: {detail}"
            )));
        }
        Ok(planned)
    }

    /// Promote verified staging bytes into the live target paths. Local
    /// rsync on the target (staging and destinations share the host); live
    /// destinations that already carry foreign content are refused.
    async fn apply(
        &self,
        req: &StatefulMigrationRequest,
        target: &CommandExecutor,
        planned: &[PlannedObject],
        statuses: &mut HashMap<String, String>,
    ) -> Result<(), Flow> {
        let mut applied = 0_i64;
        let total = planned.len() as i64;
        for object in planned {
            let status = statuses
                .get(&object.obj_key)
                .map(String::as_str)
                .unwrap_or("");
            if !apply_needed(status) {
                if matches!(status, "APPLIED" | "DONE") {
                    applied += 1;
                }
                continue;
            }
            let dest = match object.kind {
                super::scope::MigrationObjectKind::Volume => {
                    let name = volume_name_from_mountpoint(&object.source_ref).ok_or_else(|| {
                        Flow::Failed(format!(
                            "cannot derive volume name from {}",
                            object.source_ref
                        ))
                    })?;
                    // Idempotent; a pre-existing volume keeps its data and
                    // trips the foreign-content refusal below.
                    target
                        .run("docker", ["volume", "create", &name])
                        .await
                        .map_err(|error| Flow::Failed(error.to_string()))?;
                    object.source_ref.clone()
                }
                super::scope::MigrationObjectKind::Src
                | super::scope::MigrationObjectKind::Bind => object.source_ref.clone(),
                other => {
                    return Err(Flow::Failed(format!(
                        "{} objects are not supported by this migration slice",
                        other.as_str()
                    )));
                }
            };
            self.refuse_foreign_destination(target, &dest).await?;
            let staging = format!("{}/", object.dest_staging);
            let argv = vec!["-aHAX".to_string(), staging, format!("{dest}/")];
            let output = target
                .run("rsync", &argv)
                .await
                .map_err(|error| Flow::Failed(error.to_string()))?;
            if !output.success() {
                let tail: String = output
                    .stderr
                    .trim()
                    .chars()
                    .rev()
                    .take(400)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect();
                return Err(Flow::Failed(format!(
                    "apply of {} onto {dest} failed: {tail}",
                    object.obj_key
                )));
            }
            let checkpoint = ObjectCheckpoint {
                migration_id: req.migration_id.clone(),
                obj_key: object.obj_key.clone(),
            };
            checkpoint.set_status(&self.queries, "APPLIED", "").await?;
            statuses.insert(object.obj_key.clone(), "APPLIED".into());
            applied += 1;
            self.set_phase(
                req,
                MigrationPhase::Apply,
                "RUNNING",
                82 + (8 * applied.min(total)) / total.max(1),
            )
            .await?;
        }
        Ok(())
    }

    /// Refuse to clobber target paths that already hold data (an operator
    /// placed something there, or a previous run applied under a different
    /// migration id). Missing or empty destinations are free to take.
    async fn refuse_foreign_destination(
        &self,
        target: &CommandExecutor,
        dest: &str,
    ) -> Result<(), Flow> {
        if !target
            .run("test", ["-d", dest])
            .await
            .map(|output| output.success())
            .unwrap_or(false)
        {
            return Ok(());
        }
        let listing = target
            .run("ls", ["-A", dest])
            .await
            .map_err(|error| Flow::Failed(error.to_string()))?;
        if listing.success() && !listing.stdout.trim().is_empty() {
            return Err(Flow::Failed(format!(
                "target path {dest:?} already holds data — refusing to overwrite it (remove \
                 or relocate the foreign content and retry the migration)"
            )));
        }
        Ok(())
    }

    /// Flip this service's assignment rows to the target. Certificates are
    /// deliberately untouched in this slice (per-service cert repoint +
    /// ACME re-issue is a follow-up); the queries validate the old
    /// ownership in-statement, so a mismatch simply moves nothing.
    async fn repoint(
        &self,
        req: &StatefulMigrationRequest,
        ctx: &RunContext,
    ) -> Result<(), Flow> {
        let id_list = serde_json::to_string(&[req.service_id])
            .map_err(|error| Flow::Failed(error.to_string()))?;
        match req.kind {
            MigratedServiceKind::Application => {
                self.queries
                    .servers_migrate_application_servers_by_ids(
                        &ServersMigrateApplicationServersByIdsParams {
                            server_id: req.target_server_id,
                            server_id2: req.source_server_id,
                            id_list: id_list.clone(),
                        },
                    )
                    .await?;
            }
            MigratedServiceKind::Compose => {
                self.queries
                    .servers_migrate_compose_servers_by_ids(
                        &ServersMigrateComposeServersByIdsParams {
                            server_id: req.target_server_id,
                            server_id2: req.source_server_id,
                            id_list: id_list.clone(),
                        },
                    )
                    .await?;
            }
        }
        self.queries
            .servers_migrate_build_assignments_by_ids(
                &ServersMigrateBuildAssignmentsByIdsParams {
                    build_server_id: req.target_server_id,
                    build_server_id2: req.source_server_id,
                    service_id_list: id_list,
                },
            )
            .await?;
        if !ctx.schedule_ids.is_empty() {
            let schedule_list = serde_json::to_string(&ctx.schedule_ids)
                .map_err(|error| Flow::Failed(error.to_string()))?;
            self.queries
                .servers_migrate_schedules_by_ids(&ServersMigrateSchedulesByIdsParams {
                    server_id: req.target_server_id,
                    server_id2: req.source_server_id,
                    id_list: schedule_list,
                })
                .await?;
        }
        Ok(())
    }

    /// Poll the redeployed workload until it reads Up, or the budget runs
    /// out (fail-closed — SUPERSEDED is never granted on timeout).
    async fn health(
        &self,
        req: &StatefulMigrationRequest,
        deployment_id: Option<i64>,
    ) -> Result<(), Flow> {
        let attempts = health_poll_attempts(HEALTH_TIMEOUT, HEALTH_INTERVAL);
        let mut last = String::from("service has not reported a state yet");
        for _ in 0..attempts {
            tokio::time::sleep(HEALTH_INTERVAL).await;
            let deployment_status = match deployment_id {
                Some(id) => {
                    let row = self.queries.deployments_get_by_id(id).await?;
                    Some(row.status)
                }
                None => None,
            };
            let service = self.queries.get_service_by_i_d(req.service_id).await?;
            match health_verdict(&service.app_status, deployment_status.as_deref()) {
                HealthVerdict::Up => return Ok(()),
                HealthVerdict::Waiting { reason } => last = reason,
                HealthVerdict::Down { reason } => {
                    return Err(Flow::Failed(format!(
                        "target workload unhealthy after redeploy: {reason}"
                    )));
                }
            }
        }
        Err(Flow::Failed(format!(
            "health budget exhausted ({:?}): {last}",
            HEALTH_TIMEOUT
        )))
    }

    /// Build the raw SSH credential bundle the source's rsync uses to push
    /// into the target (same private-network preference as deployments).
    async fn sync_credentials(&self, server_id: i64) -> Result<SyncCredentials, Flow> {
        let row = self.queries.servers_get_ssh_credentials(server_id).await?;
        let mut connect_host = row.public_ip;
        if let Some(private_ip) = row.private_ip.as_deref() {
            let mode = row.connection_mode.as_deref().unwrap_or_default();
            let status = row.private_status.as_deref().unwrap_or_default();
            if !private_ip.trim().is_empty()
                && mode.eq_ignore_ascii_case("MANAGED_WIREGUARD")
                && status.eq_ignore_ascii_case("ACTIVE")
            {
                connect_host = private_ip.trim().to_string();
            }
        }
        let private_key = row
            .private_key
            .ok_or_else(|| Flow::Failed("target server has no ssh private key".into()))?;
        let username = row.username;
        Ok(SyncCredentials {
            ssh_port: u16::try_from(row.port)
                .map_err(|_| Flow::Failed("ssh port out of range".into()))?,
            username,
            connect_host,
            private_key,
            host_key_openssh: row.host_key_openssh,
        })
    }

    async fn require_rsync(&self, executor: &CommandExecutor, which: &str) -> Result<(), Flow> {
        let output = executor
            .run("which", ["rsync"])
            .await
            .map_err(|error| Flow::Failed(format!("{which}: rsync probe failed: {error}")))?;
        if !output.success() || output.stdout.trim().is_empty() {
            return Err(Flow::Failed(format!(
                "rsync is not installed on the {which} server; install it and retry"
            )));
        }
        Ok(())
    }

    // ---- terminal handlers ------------------------------------------------

    async fn on_cancelled(&self, req: &StatefulMigrationRequest, ctx: &RunContext) {
        let _ = self
            .queries
            .application_migrations_finish_cancelled(&req.migration_id)
            .await;
        self.restore(req, ctx).await;
    }

    async fn on_failed(
        &self,
        req: &StatefulMigrationRequest,
        message: &str,
        ctx: &RunContext,
    ) {
        let _ = self
            .queries
            .application_migrations_finish_failure(message, &req.migration_id)
            .await;
        self.restore(req, ctx).await;
    }

    /// Best-effort bring-back: re-enable schedules that QUIESCE paused and,
    /// while the metadata still points at the source, redeploy it. Past
    /// REPOINT the target owns the service — a source redeploy would split
    /// assignments, so only schedules are restored there.
    async fn restore(&self, req: &StatefulMigrationRequest, ctx: &RunContext) {
        if let Ok(rows) = self.fetch_schedules(req).await {
            let current: HashMap<i64, bool> = rows
                .into_iter()
                .map(|row| (row.id, row.enabled != 0))
                .collect();
            for record in &ctx.paused {
                let enabled = *current.get(&record.schedule_id).unwrap_or(&true);
                if decide_schedule_restore(record, enabled) == Some(true) {
                    let _ = self.queries.toggle_schedule(1, record.schedule_id).await;
                }
            }
        }
        if ctx.repointed {
            return;
        }
        let restart = match req.kind {
            MigratedServiceKind::Application => self
                .applications
                .run_operation(req.service_id, ApplicationOperation::Redeploy, 0)
                .await
                .map(|_| ()),
            MigratedServiceKind::Compose => self
                .compose
                .run_operation(req.service_id, ComposeOperation::Redeploy, 0)
                .await
                .map(|_| ()),
        };
        if let Err(error) = restart {
            tracing::warn!(
                migration_id = %req.migration_id,
                %error,
                "source redeploy during restore failed — the service stays stopped on the source"
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flash_gen::models::MigrationObjects;

    fn object(status: &str) -> MigrationObjects {
        MigrationObjects {
            migration_id: "m1".into(),
            obj_key: "VOLUME:aa".into(),
            kind: "VOLUME".into(),
            source_ref: "/v".into(),
            dest_ref: "/s".into(),
            status: status.into(),
            bytes: 0,
            error: None,
            created_at: 0,
            updated_at: 0,
        }
    }

    #[test]
    fn progress_spans_the_sync_window() {
        assert_eq!(sync_progress(0, 4), 10);
        assert_eq!(sync_progress(2, 4), 45);
        assert_eq!(sync_progress(4, 4), 80);
        // Overcounting never runs past the window; empty plans skip it.
        assert_eq!(sync_progress(9, 4), 80);
        assert_eq!(sync_progress(0, 0), 80);
    }

    #[test]
    fn resume_skips_settled_checkpoints_only() {
        for status in ["VERIFIED", "APPLIED", "DONE"] {
            assert!(sync_done(status), "{status} needs no re-sync");
        }
        for status in ["PENDING", "SYNCING", "SYNCED", "FAILED", ""] {
            assert!(!sync_done(status), "{status} must be re-synced");
        }
        // APPLY only promotes fresh VERIFIED rows.
        assert!(apply_needed("VERIFIED"));
        assert!(!apply_needed("APPLIED"));
        assert!(!apply_needed("DONE"));
        assert!(!apply_needed("SYNCED"));
    }

    #[test]
    fn superseded_gate_counts_unsettled_checkpoints() {
        assert_eq!(unsettled_objects(&[]), 0);
        assert_eq!(
            unsettled_objects(&[object("APPLIED"), object("DONE"), object("VERIFIED")]),
            0
        );
        assert_eq!(unsettled_objects(&[object("SYNCED"), object("FAILED")]), 2);
    }

    #[test]
    fn volume_names_only_from_canonical_mountpoints() {
        assert_eq!(
            volume_name_from_mountpoint("/var/lib/docker/volumes/app-data/_data").as_deref(),
            Some("app-data")
        );
        assert_eq!(
            volume_name_from_mountpoint("/var/lib/docker/volumes/app-data/_data/").as_deref(),
            Some("app-data")
        );
        assert_eq!(volume_name_from_mountpoint("/srv/bind"), None);
        assert_eq!(volume_name_from_mountpoint("/_data"), None);
        assert_eq!(
            volume_name_from_mountpoint("/var/lib/docker/volumes/bad name/_data"),
            None
        );
    }

    #[test]
    fn health_is_fail_closed_until_the_deployment_lands() {
        // Deployment still in flight → running service alone only waits.
        assert!(matches!(
            health_verdict("RUNNING", Some("QUEUED")),
            HealthVerdict::Waiting { .. }
        ));
        assert_eq!(health_verdict("running", Some("DONE")), HealthVerdict::Up);
        assert!(matches!(
            health_verdict("RUNNING", Some("FAILED")),
            HealthVerdict::Down { .. }
        ));
        assert!(matches!(
            health_verdict("idle", Some("DONE")),
            HealthVerdict::Down { .. }
        ));
        // No deployment id at all → never Up (budget decides).
        assert!(matches!(
            health_verdict("RUNNING", None),
            HealthVerdict::Waiting { .. }
        ));
    }
}
