//! S7 cutover semantics — the pure decision rules at the end of a
//! migration, unit-testable without live nodes:
//!
//! * **Certificates**: portable material (uploaded/self-signed rows carry
//!   `certificate_data` + `private_key`) moves with the service; ACME-origin
//!   certificates are NEVER shipped — the target must re-issue through its
//!   own edge, and the source edge must be stopped or it keeps renewing
//!   (rate-limit hazard). The node-owned lego store
//!   `<base>/traefik/dynamic/acme.json` must never appear in any transfer
//!   path ([`assert_not_acme_store`]).
//! * **Health**: fail-closed — only an explicit `running`/`healthy` reading
//!   passes; unknown, error output, or timeout fail.
//! * **Superseded**: the source may only be marked SUPERSEDED after the
//!   migration reached its terminal success phase with zero non-DONE
//!   objects — delete nothing, stop the source workload only.

use crate::utils::paths::OpenOxidePaths;

use super::scope::MigrationPhase;

/// How a certificate row came to exist. The unified `certificates` table
/// stores material for every kind; origin is inferred from issuance
/// configuration at plan time (uploaded vs ACME-issued).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CertificateOrigin {
    /// Material uploaded or self-signed by the user — portable.
    Material,
    /// Issued by the SOURCE node's Traefik/lego ACME flow — not portable.
    Acme,
}

/// Decision for one certificate in scope.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CertificateAction {
    /// Re-point the row + regenerate files on the target from material.
    Migrate { certificate_id: i64 },
    /// ACME-origin: re-point the row but the TARGET must issue its own cert;
    /// the source edge must be stopped so it stops renewing (warn the user).
    ReissueOnTarget { certificate_id: i64, warning: String },
}

pub fn decide_certificate_action(
    certificate_id: i64,
    origin: CertificateOrigin,
    has_material: bool,
) -> CertificateAction {
    match origin {
        CertificateOrigin::Material if has_material => CertificateAction::Migrate {
            certificate_id,
        },
        // Material row with empty material is broken data; reissue path is
        // the only safe one and it surfaces a warning.
        CertificateOrigin::Material => CertificateAction::ReissueOnTarget {
            certificate_id,
            warning: "certificate row stores no material — target must re-issue".into(),
        },
        CertificateOrigin::Acme => CertificateAction::ReissueOnTarget {
            certificate_id,
            warning: format!(
                "certificate {certificate_id} was ACME-issued by the source edge; its cert/key \
                 are not copied — the target will issue its own, and the source edge must be \
                 stopped during SUPERSEDED or it keeps renewing (Let's Encrypt rate limits)"
            ),
        },
    }
}

/// The acme.json guard: no sync/apply destination or source may ever be the
/// node-owned lego store (exact match on `<base>/traefik/dynamic/acme.json`,
/// robust to trailing-slash/`.` noise via normalisation).
pub fn assert_not_acme_store(paths: &OpenOxidePaths, candidate: &str) -> Result<(), String> {
    let normalized = crate::services::migration::scope::normalize_ref(candidate)
        .ok_or_else(|| format!("unusable path reference {candidate:?}"))?;
    let acme = crate::services::migration::scope::normalize_ref(
        &format!("{}/acme.json", paths.traefik_dynamic()),
    )
    .expect("canonical acme path normalises");
    if normalized == acme {
        return Err(format!(
            "{candidate:?} is the node-owned lego ACME store — it is never migrated, copied, \
             or deleted by any migration step"
        ));
    }
    Ok(())
}

/// Fail-closed health classification from a docker state reading (plus an
/// optional HTTP/TCP probe result against the domain).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HealthVerdict {
    Up,
    Waiting { reason: String },
    Down { reason: String },
}

pub fn classify_health(container_state: Option<&str>, probe_ok: Option<bool>) -> HealthVerdict {
    match container_state {
        None => HealthVerdict::Down {
            reason: "no container state reported".into(),
        },
        Some("running") => match probe_ok {
            Some(true) => HealthVerdict::Up,
            Some(false) => HealthVerdict::Waiting {
                reason: "container running; domain/TLS probe not green yet".into(),
            },
            None => HealthVerdict::Waiting {
                reason: "container running; awaiting edge probe".into(),
            },
        },
        Some("restarting") | Some("created") | Some("paused") => HealthVerdict::Waiting {
            reason: format!("container is {container_state:?}"),
        },
        Some(other) => HealthVerdict::Down {
            reason: format!("container is {other:?}"),
        },
    }
}

/// HEALTH polling budget: fixed attempt count × interval; surfaces the
/// timeout as a Down verdict the caller records (migration → FAILED, source
/// stays untouched — superseded is never granted on timeout).
pub fn health_poll_attempts(timeout: std::time::Duration, interval: std::time::Duration) -> u32 {
    let interval_ms = interval.as_millis().max(1);
    let timeout_ms = timeout.as_millis();
    u32::try_from(timeout_ms / interval_ms).unwrap_or(u32::MAX).max(1)
}

/// SUPERSEDED gate: terminal success may only be recorded when the phase
/// machine has run past REDEPLOY/HEALTH and every checkpoint object is
/// DONE/VERIFIED+APPLIED — `pending_objects` is the count of
/// `migration_objects` rows with status NOT IN ('DONE').
pub fn may_mark_superseded(phase: MigrationPhase, pending_objects: i64) -> bool {
    matches!(phase, MigrationPhase::Health | MigrationPhase::Superseded) && pending_objects == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn paths() -> OpenOxidePaths {
        OpenOxidePaths {
            base: "/etc/openoxide".into(),
        }
    }

    #[test]
    fn acme_store_is_never_a_transfer_operand() {
        assert!(assert_not_acme_store(&paths(), "/etc/openoxide/traefik/dynamic/acme.json").is_err());
        // Trailing-slash / `.` noise normalises into the same refusal.
        assert!(
            assert_not_acme_store(&paths(), "/etc/openoxide/traefik/dynamic/./acme.json").is_err()
        );
        assert!(assert_not_acme_store(&paths(), "/etc/openoxide/traefik/dynamic/acme.json.bak").is_ok());
        assert!(
            assert_not_acme_store(&paths(), "/etc/openoxide/migrations/m1/staging/CERT-aa").is_ok()
        );
        assert!(assert_not_acme_store(&paths(), "").is_err());
    }

    #[test]
    fn certificate_matrix() {
        assert_eq!(
            decide_certificate_action(1, CertificateOrigin::Material, true),
            CertificateAction::Migrate { certificate_id: 1 }
        );
        let acme = decide_certificate_action(2, CertificateOrigin::Acme, true);
        let CertificateAction::ReissueOnTarget { warning, .. } = acme else {
            panic!("ACME must reissue")
        };
        assert!(warning.contains("rate limits"));
        assert!(matches!(
            decide_certificate_action(3, CertificateOrigin::Material, false),
            CertificateAction::ReissueOnTarget { .. }
        ));
    }

    #[test]
    fn health_fails_closed() {
        assert_eq!(classify_health(Some("running"), Some(true)), HealthVerdict::Up);
        assert!(matches!(
            classify_health(Some("running"), None),
            HealthVerdict::Waiting { .. }
        ));
        assert!(matches!(
            classify_health(Some("running"), Some(false)),
            HealthVerdict::Waiting { .. }
        ));
        assert!(matches!(
            classify_health(Some("exited"), Some(true)),
            HealthVerdict::Down { .. }
        ));
        assert!(matches!(
            classify_health(None, Some(true)),
            HealthVerdict::Down { .. }
        ));
        assert!(matches!(
            classify_health(Some("bogus-state"), None),
            HealthVerdict::Down { .. }
        ));
    }

    #[test]
    fn health_budget_rounds_sensibly() {
        use std::time::Duration;
        assert_eq!(health_poll_attempts(Duration::from_secs(60), Duration::from_secs(2)), 30);
        assert_eq!(health_poll_attempts(Duration::ZERO, Duration::from_secs(2)), 1);
        // A zero interval is floored to 1ms so the budget stays finite.
        assert_eq!(health_poll_attempts(Duration::from_secs(5), Duration::ZERO), 5000);
    }

    #[test]
    fn superseded_requires_finished_objects() {
        assert!(may_mark_superseded(MigrationPhase::Health, 0));
        assert!(!may_mark_superseded(MigrationPhase::Health, 2));
        assert!(!may_mark_superseded(MigrationPhase::Sync, 0));
        assert!(!may_mark_superseded(MigrationPhase::Redeploy, 0));
        assert!(may_mark_superseded(MigrationPhase::Superseded, 0));
    }
}
