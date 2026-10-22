//! S6 quiesce/cancel policy — the decision logic behind the QUIESCE and
//! teardown steps, kept pure so the safety semantics are unit-tested even
//! where live execution needs a two-node harness:
//!
//! * cancel is **edge-polite**: `cancel_requested` (DB) or the process token
//!   both flip the migration to CANCELLED, checked only at phase/object
//!   boundaries — never mid-rsync-byte.
//! * replica restore is **conservative**: QUIESCE records the pre-stop
//!   replica count, restore only touches services still sitting at the
//!   scale-0 sentinel (a human changed it meanwhile ⇒ leave it alone).
//! * schedule pause/restore is **state-preserving**: only schedules that were
//!   originally enabled get re-enabled.

use tokio_util::sync::CancellationToken;

/// Cancellation surface for one migration run.
#[derive(Clone)]
pub struct CancelSignal {
    token: CancellationToken,
}

impl CancelSignal {
    pub fn new(token: CancellationToken) -> Self {
        Self { token }
    }

    /// Polled at every phase boundary and per-object checkpoint: `true` once
    /// the panel requested cancel (DB flag mirrored into the token) or the
    /// process is shutting down.
    pub fn is_cancelled(&self, cancel_requested_db: bool) -> bool {
        cancel_requested_db || self.token.is_cancelled()
    }

    pub fn token(&self) -> &CancellationToken {
        &self.token
    }
}

/// Pre-stop replica snapshot for one service.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplicaRecord {
    pub service_id: i64,
    pub recorded_replicas: i64,
}

/// Outcome of the restore-replicas decision.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReplicaRestore {
    /// Still at the scale-0 sentinel → safe to put back.
    Restore { service_id: i64, to: i64 },
    /// Someone changed the count during the window → do not clobber it.
    Skipped { service_id: i64, reason: String },
}

/// Decide whether to restore one service's replicas. `current_replicas` is
/// re-read at restore time; anything other than the scale-0 sentinel (or a
/// recorded value that was already ≤1) means restore is a no-op by design —
/// QUIESCE never scaled anything, it only stopped it.
pub fn decide_replica_restore(
    record: &ReplicaRecord,
    current_replicas: Option<i64>,
) -> ReplicaRestore {
    let Some(current) = current_replicas else {
        return ReplicaRestore::Skipped {
            service_id: record.service_id,
            reason: "service row disappeared during migration".into(),
        };
    };
    if record.recorded_replicas <= 1 {
        return ReplicaRestore::Skipped {
            service_id: record.service_id,
            reason: format!(
                "recorded replicas {} needed no scale-down",
                record.recorded_replicas
            ),
        };
    }
    // The scale-0 sentinel is the ONLY state QUIESCE left behind.
    if current != 0 {
        return ReplicaRestore::Skipped {
            service_id: record.service_id,
            reason: format!(
                "replicas changed to {current} during the migration window (QUIESCE left 0) — \
                 leaving the human's value alone"
            ),
        };
    }
    ReplicaRestore::Restore {
        service_id: record.service_id,
        to: record.recorded_replicas,
    }
}

/// Pre-pause state of one schedule (`schedules.enabled` is the source of
/// truth; the pause writes 0, restore writes back exactly what was read).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SchedulePauseRecord {
    pub schedule_id: i64,
    pub was_enabled: bool,
}

/// Decide the restore value for one schedule. A schedule already re-enabled
/// (e.g. by a human) stays enabled; one that was never enabled is never
/// switched on. Returns `Some(true)` only when the caller must write.
pub fn decide_schedule_restore(
    record: &SchedulePauseRecord,
    current_enabled: bool,
) -> Option<bool> {
    match (record.was_enabled, current_enabled) {
        // Restore required: still paused, was on before QUIESCE.
        (true, false) => Some(true),
        // Already matching the pre-pause state (or was off → stays off).
        (true, true) | (false, false) => None,
        // Disabled before AND enabled now: a human turned it on — leave it.
        (false, true) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cancel_signal_honors_db_flag_and_token() {
        let token = CancellationToken::new();
        let signal = CancelSignal::new(token.clone());
        assert!(!signal.is_cancelled(false));
        assert!(signal.is_cancelled(true), "DB flag alone cancels");
        token.cancel();
        assert!(signal.is_cancelled(false), "token alone cancels");
        assert!(signal.is_cancelled(true));
    }

    #[test]
    fn replica_restore_only_touches_the_scale0_sentinel() {
        let record = ReplicaRecord {
            service_id: 7,
            recorded_replicas: 3,
        };
        assert_eq!(
            decide_replica_restore(&record, Some(0)),
            ReplicaRestore::Restore {
                service_id: 7,
                to: 3
            }
        );
        // Human scaled during the window → skip.
        let skipped = decide_replica_restore(&record, Some(5));
        assert!(matches!(skipped, ReplicaRestore::Skipped { .. }));
        // Row gone → skip, never resurrect blindly.
        let gone = decide_replica_restore(&record, None);
        assert!(matches!(gone, ReplicaRestore::Skipped { .. }));
        // Single-replica services were never scaled → no-op restore.
        let single = ReplicaRecord {
            service_id: 8,
            recorded_replicas: 1,
        };
        assert!(matches!(
            decide_replica_restore(&single, Some(0)),
            ReplicaRestore::Skipped { .. }
        ));
    }

    #[test]
    fn schedule_restore_writes_only_when_needed() {
        let was_on = SchedulePauseRecord {
            schedule_id: 1,
            was_enabled: true,
        };
        assert_eq!(decide_schedule_restore(&was_on, false), Some(true));
        assert_eq!(decide_schedule_restore(&was_on, true), None);
        let was_off = SchedulePauseRecord {
            schedule_id: 2,
            was_enabled: false,
        };
        assert_eq!(decide_schedule_restore(&was_off, false), None);
        // Human enabled it during the window → leave it on.
        assert_eq!(decide_schedule_restore(&was_off, true), None);
    }
}
