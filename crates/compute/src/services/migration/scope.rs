use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Phases of the migration state machine persisted in
/// `application_migrations.phase`. Terminal states are `Superseded`, `Failed`,
/// `RolledBack` and `Cancelled`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MigrationPhase {
    Preflight,
    Quiesce,
    Sync,
    Verify,
    Apply,
    Repoint,
    Redeploy,
    Health,
    Superseded,
    Failed,
    RolledBack,
    Cancelled,
}

impl MigrationPhase {
    pub fn as_str(&self) -> &'static str {
        match self {
            MigrationPhase::Preflight => "PREFLIGHT",
            MigrationPhase::Quiesce => "QUIESCE",
            MigrationPhase::Sync => "SYNC",
            MigrationPhase::Verify => "VERIFY",
            MigrationPhase::Apply => "APPLY",
            MigrationPhase::Repoint => "REPOINT",
            MigrationPhase::Redeploy => "REDEPLOY",
            MigrationPhase::Health => "HEALTH",
            MigrationPhase::Superseded => "SUPERSEDED",
            MigrationPhase::Failed => "FAILED",
            MigrationPhase::RolledBack => "ROLLED_BACK",
            MigrationPhase::Cancelled => "CANCELLED",
        }
    }
}

/// The class of data a `migration_objects` checkpoint row tracks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MigrationObjectKind {
    Volume,
    Bind,
    DbData,
    Dump,
    Src,
    Cert,
}

impl MigrationObjectKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            MigrationObjectKind::Volume => "VOLUME",
            MigrationObjectKind::Bind => "BIND",
            MigrationObjectKind::DbData => "DBDATA",
            MigrationObjectKind::Dump => "DUMP",
            MigrationObjectKind::Src => "SRC",
            MigrationObjectKind::Cert => "CERT",
        }
    }
}

/// Normalise a source/dest reference before hashing it into an `obj_key`.
/// Collapses redundant separators and rejects path traversal so a hostile or
/// malformed reference can never collide with — or escape — another object's
/// staging prefix. Returns `None` for refs containing a `..` segment or that
/// are empty after trimming.
pub fn normalize_ref(reference: &str) -> Option<String> {
    let trimmed = reference.trim();
    if trimmed.is_empty() {
        return None;
    }
    let mut segments = Vec::new();
    for segment in trimmed.split('/') {
        match segment {
            "" | "." => continue,
            ".." => return None,
            other => segments.push(other),
        }
    }
    if segments.is_empty() {
        return None;
    }
    // Preserve an absolute prefix; everything else is joined canonically.
    let prefix = if trimmed.starts_with('/') { "/" } else { "" };
    Some(format!("{prefix}{}", segments.join("/")))
}

/// Derive the tamper-proof staging key for an object:
/// `{KIND}:{first 16 hex chars of sha256(normalised_ref)}`. Only this value —
/// never a raw user string — reaches rsync argv or the target URL path.
pub fn derive_obj_key(kind: MigrationObjectKind, reference: &str) -> Option<String> {
    let normalized = normalize_ref(reference)?;
    let digest = Sha256::digest(normalized.as_bytes());
    let hex: String = digest
        .iter()
        .take(8)
        .map(|byte| format!("{byte:02x}"))
        .collect();
    Some(format!("{}:{}", kind.as_str(), hex))
}

/// The set of resource ids a single migration acts on. The whole-server path
/// fills every list from `servers_migratable_*_ids(source)`; the per-service
/// path fills only the lists relevant to the chosen service.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MigrationScope {
    pub application_ids: Vec<i64>,
    pub compose_ids: Vec<i64>,
    pub build_ids: Vec<i64>,
    pub certificate_ids: Vec<i64>,
    pub schedule_ids: Vec<i64>,
}

impl MigrationScope {
    pub fn is_empty(&self) -> bool {
        self.application_ids.is_empty()
            && self.compose_ids.is_empty()
            && self.build_ids.is_empty()
            && self.certificate_ids.is_empty()
            && self.schedule_ids.is_empty()
    }

    /// Application + compose service ids, matching the combined `service_ids`
    /// column written to `server_migrations`.
    pub fn service_ids(&self) -> Vec<i64> {
        let mut ids = self.application_ids.clone();
        ids.extend_from_slice(&self.compose_ids);
        ids
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn obj_key_is_stable_and_kind_prefixed() {
        let a = derive_obj_key(MigrationObjectKind::Volume, "app-data").unwrap();
        let b = derive_obj_key(MigrationObjectKind::Volume, "app-data").unwrap();
        assert_eq!(a, b);
        assert!(a.starts_with("VOLUME:"));
        // Different kinds produce different keys even for the same reference.
        let c = derive_obj_key(MigrationObjectKind::Bind, "app-data").unwrap();
        assert_ne!(a, c);
    }

    #[test]
    fn obj_key_rejects_traversal_and_empty() {
        assert!(normalize_ref("/data/../../etc/passwd").is_none());
        assert!(derive_obj_key(MigrationObjectKind::Volume, "../escape").is_none());
        assert!(derive_obj_key(MigrationObjectKind::Volume, "   ").is_none());
    }

    #[test]
    fn normalize_collapses_redundant_separators() {
        assert_eq!(
            normalize_ref("/var//lib///docker").as_deref(),
            Some("/var/lib/docker")
        );
        assert_eq!(normalize_ref("a/./b").as_deref(), Some("a/b"));
    }

    #[test]
    fn phase_strings_match_the_state_machine() {
        assert_eq!(MigrationPhase::Preflight.as_str(), "PREFLIGHT");
        assert_eq!(MigrationPhase::Superseded.as_str(), "SUPERSEDED");
        assert_eq!(MigrationPhase::RolledBack.as_str(), "ROLLED_BACK");
    }
}
