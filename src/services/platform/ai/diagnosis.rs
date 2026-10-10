//! AI build-failure diagnosis.
//!
//! Given a failed deployment's error output + its config (compose YAML /
//! Dockerfile / env), the model classifies the failure as:
//!   - `fixable`    — the problem is in config/compose/dockerfile/env and can
//!                    be fixed WITHOUT touching repo source; a concrete fix is
//!                    returned.
//!   - `code_error` — the problem is in the user's application code/repo; a
//!                    config change won't help (no "Fix with AI" button).
//!   - `unknown`    — not confident.
//!
//! The returned fix only ever rewrites config (compose/dockerfile/env), never
//! repo source.

use crate::utils::ai::{AiClient, AiProviderConfig};
use serde::{Deserialize, Serialize};

/// What kind of config the failed deploy uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigKind {
    Compose,
    Dockerfile,
    Env,
}

impl ConfigKind {
    fn as_str(self) -> &'static str {
        match self {
            ConfigKind::Compose => "compose",
            ConfigKind::Dockerfile => "dockerfile",
            ConfigKind::Env => "env",
        }
    }
}

/// Input for a diagnosis run.
pub struct DiagnosisInput<'a> {
    /// "application" or "compose".
    pub kind: &'a str,
    /// The build/runtime error output (tail is fine).
    pub error: &'a str,
    /// What config we can offer to fix + its current content.
    pub config_kind: ConfigKind,
    pub config_content: &'a str,
    /// Optional extra context (env vars, repo manifests) for the model.
    pub extra: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Classification {
    Fixable,
    CodeError,
    Unknown,
}

impl Classification {
    pub fn as_str(self) -> &'static str {
        match self {
            Classification::Fixable => "fixable",
            Classification::CodeError => "code_error",
            Classification::Unknown => "unknown",
        }
    }
    pub fn parse(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "fixable" => Classification::Fixable,
            "code_error" | "code-error" => Classification::CodeError,
            _ => Classification::Unknown,
        }
    }
}

/// The proposed fix (config-only).
#[derive(Debug, Clone)]
pub struct Fix {
    /// "compose" | "dockerfile" | "env".
    pub target: String,
    /// The full replacement content for that target.
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct Diagnosis {
    pub classification: Classification,
    pub root_cause: String,
    pub explanation: String,
    pub fix: Option<Fix>,
}

/// Raw JSON shape we ask the model to return.
#[derive(Deserialize)]
struct RawDiagnosis {
    classification: String,
    #[serde(default)]
    root_cause: String,
    #[serde(default)]
    explanation: String,
    #[serde(default)]
    fix: Option<RawFix>,
}

#[derive(Deserialize)]
struct RawFix {
    #[serde(default)]
    target: String,
    #[serde(default)]
    content: String,
}

/// Run the AI diagnosis. Returns `Err` only on provider/parse failure.
pub async fn diagnose(
    config: &AiProviderConfig,
    input: &DiagnosisInput<'_>,
) -> Result<Diagnosis, String> {
    let prompt = build_prompt(input);
    let raw = AiClient
        .complete_text(config, &prompt)
        .await
        .map_err(|e| format!("diagnosis AI call failed: {e}"))?;

    let json = extract_json(&raw);
    let parsed: RawDiagnosis = serde_json::from_str(&json).map_err(|e| {
        format!(
            "could not parse diagnosis JSON: {e}; raw: {}",
            tail(&raw, 300)
        )
    })?;

    let classification = Classification::parse(&parsed.classification);
    // A fix only makes sense (and is only trusted) when classified fixable.
    let fix = if classification == Classification::Fixable {
        parsed
            .fix
            .filter(|f| !f.content.trim().is_empty())
            .map(|f| {
                let target = if f.target.trim().is_empty() {
                    input.config_kind.as_str().to_string()
                } else {
                    f.target.trim().to_lowercase()
                };
                Fix {
                    target,
                    content: f.content,
                }
            })
    } else {
        None
    };

    Ok(Diagnosis {
        classification,
        root_cause: parsed.root_cause.trim().to_string(),
        explanation: parsed.explanation.trim().to_string(),
        fix,
    })
}

fn build_prompt(input: &DiagnosisInput<'_>) -> String {
    format!(
        "You are a senior DevOps engineer diagnosing a failed {kind} deployment \
         on a self-hosted platform. Decide whether this failure can be fixed by \
         editing CONFIG ONLY (the {ck} file / env / Dockerfile) WITHOUT changing \
         the user's application source code.\n\n\
         Respond with ONLY a JSON object (no markdown, no prose) of this shape:\n\
         {{\n\
           \"classification\": \"fixable\" | \"code_error\" | \"unknown\",\n\
           \"root_cause\": \"one sentence\",\n\
           \"explanation\": \"2-4 sentences the user can understand\",\n\
           \"fix\": {{ \"target\": \"{ck}\", \"content\": \"<the FULL corrected {ck} content>\" }}\n\
         }}\n\n\
         Rules:\n\
         - \"fixable\" ONLY if the fix is entirely in the {ck} (e.g. missing env \
           var, wrong port/host, missing depends_on/healthcheck, bad image tag, \
           missing build arg, wrong WORKDIR/COPY). Then include the full \
           corrected {ck} in fix.content.\n\
         - \"code_error\" if the failure is in the user's application code \
           (compile error, failing test, runtime bug, missing source file) — a \
           config change will NOT help; omit fix.\n\
         - \"unknown\" if you cannot tell; omit fix.\n\
         - NEVER propose changing application source. fix.content is the whole \
           replacement {ck}, valid and complete.\n\n\
         ## Build / runtime error\n```\n{err}\n```\n\n\
         ## Current {ck}\n```\n{cfg}\n```\n{extra}",
        kind = input.kind,
        ck = input.config_kind.as_str(),
        err = tail(input.error, 4_000),
        cfg = input.config_content,
        extra = if input.extra.trim().is_empty() {
            String::new()
        } else {
            format!("\n## Extra context\n{}\n", input.extra)
        },
    )
}

/// Pull the first {...} JSON object out of a possibly fenced/prose response.
fn extract_json(text: &str) -> String {
    let t = text.trim();
    let t = t
        .strip_prefix("```json")
        .or_else(|| t.strip_prefix("```"))
        .unwrap_or(t);
    let t = t.strip_suffix("```").unwrap_or(t);
    if let (Some(start), Some(end)) = (t.find('{'), t.rfind('}')) {
        if end > start {
            return t[start..=end].to_string();
        }
    }
    t.trim().to_string()
}

fn tail(s: &str, n: usize) -> &str {
    if s.len() <= n { s } else { &s[s.len() - n..] }
}

/// Stable signature of an error for caching (strip volatile ids/timestamps).
pub fn error_signature(error: &str) -> String {
    let normalized: String = error
        .chars()
        .filter(|c| !c.is_ascii_digit())
        .collect::<String>()
        .split_whitespace()
        .take(400)
        .collect::<Vec<_>>()
        .join(" ");
    let mut h: u64 = 0xcbf29ce484222325;
    for b in normalized.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("{h:016x}")
}

/// On-demand diagnosis for a failed deployment, computed when the user asks
/// (e.g. clicks Analyze). Returns a stored diagnosis if one already exists,
/// otherwise resolves the error + service from the deployment record, runs the
/// AI diagnosis, stores and returns it. Returns None if nothing can be produced
/// (no provider, deployment not failed, etc.).
pub async fn diagnose_on_demand(deployment_id: i64, force: bool) -> Option<StoredDiagnosis> {
    // Already diagnosed → reuse, unless the caller asked for a fresh run.
    if !force {
        if let Some(existing) = get(deployment_id).await {
            return Some(existing);
        }
    }

    use crate::db::queries::FlashQueries;
    let queries = auto_di::resolve::<FlashQueries>().await.ok()?;
    let deployment = queries.get_deployment_by_i_d(deployment_id).await.ok()?;
    let service_id = deployment.service_id?;

    // Only diagnose genuine failures.
    let failed = deployment.status.to_ascii_uppercase().contains("ERROR")
        || deployment.state.to_ascii_uppercase().contains("FAIL")
        || deployment.error_message.is_some();
    if !failed {
        return None;
    }

    // Prefer the stored error message; fall back to the tail of the log file.
    let error = match deployment.error_message.filter(|e| !e.trim().is_empty()) {
        Some(e) => e,
        None => tokio::fs::read_to_string(&deployment.log_path)
            .await
            .ok()
            .map(|s| {
                s.lines()
                    .rev()
                    .take(60)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect::<Vec<_>>()
                    .join("\n")
            })
            .unwrap_or_default(),
    };
    if error.trim().is_empty() {
        return None;
    }

    let is_compose = queries
        .get_service_by_i_d(service_id)
        .await
        .ok()
        .map(|s| s.service_type.eq_ignore_ascii_case("COMPOSE"))
        .unwrap_or(false);

    diagnose_deployment(deployment_id, service_id, is_compose, error, force).await;
    get(deployment_id).await
}

/// End-to-end: resolve the org's AI provider + the deployment's config, run a
/// diagnosis (reusing a cached one for the same error signature), and store it
/// against the deployment. Best-effort — logs and returns on any failure.
pub async fn diagnose_deployment(
    deployment_id: i64,
    service_id: i64,
    is_compose: bool,
    error: String,
    force: bool,
) {
    use crate::db::queries::FlashQueries;

    let Ok(queries) = auto_di::resolve::<FlashQueries>().await else {
        return;
    };
    let provider = match resolve_provider(&queries, service_id).await {
        Some(p) => p,
        None => return,
    };

    let sig = error_signature(&error);
    if !force {
        if let Some(cached) = get_by_signature(&sig).await {
            store(deployment_id, Some(service_id), &sig, &cached).await;
            return;
        }
    }

    let (kind, config_kind, config_content, extra) = if is_compose {
        let cf = queries
            .get_service_compose(service_id)
            .await
            .ok()
            .map(|c| c.compose_file)
            .unwrap_or_default();
        let env = queries
            .get_service_by_i_d(service_id)
            .await
            .ok()
            .and_then(|s| s.env_var)
            .unwrap_or_default();
        (
            "compose",
            ConfigKind::Compose,
            cf,
            format!("## Current env (.env)\n{env}"),
        )
    } else {
        let env = queries
            .get_service_by_i_d(service_id)
            .await
            .ok()
            .and_then(|s| s.env_var)
            .unwrap_or_default();
        ("application", ConfigKind::Env, env, String::new())
    };

    let input = DiagnosisInput {
        kind,
        error: &error,
        config_kind,
        config_content: &config_content,
        extra: &extra,
    };
    match diagnose(&provider, &input).await {
        Ok(d) => {
            tracing::info!(
                deployment_id,
                classification = d.classification.as_str(),
                "ai build diagnosis stored"
            );
            store(deployment_id, Some(service_id), &sig, &d).await;
        }
        Err(e) => tracing::debug!(deployment_id, error = %e, "ai build diagnosis failed"),
    }
}

async fn resolve_provider(
    queries: &crate::db::queries::FlashQueries,
    service_id: i64,
) -> Option<AiProviderConfig> {
    let ctx = queries
        .services_get_deployment_context(service_id)
        .await
        .ok()?;
    let project = queries.get_project_by_i_d(ctx.project_id).await.ok()?;
    let settings = queries
        .ai_settings_list_by_organization(project.organization_id)
        .await
        .ok()?;
    let s = settings.into_iter().find(|s| s.is_enabled != 0)?;
    Some(AiProviderConfig {
        api_url: s.api_url,
        api_key: s.api_key,
        model: s.model,
    })
}

/// A diagnosis as stored/read back for a deployment.
#[derive(Debug, Clone)]
pub struct StoredDiagnosis {
    pub classification: Classification,
    pub root_cause: String,
    pub explanation: String,
    pub fix_target: Option<String>,
    pub fix_content: Option<String>,
}

impl StoredDiagnosis {
    pub fn is_fixable(&self) -> bool {
        self.classification == Classification::Fixable && self.fix_content.is_some()
    }
}

/// Persist a diagnosis for a deployment (upsert). Best-effort.
pub async fn store(
    deployment_id: i64,
    service_id: Option<i64>,
    error_signature: &str,
    d: &Diagnosis,
) {
    let Ok(pool) = auto_di::resolve::<sqlx::SqlitePool>().await else {
        return;
    };
    let (ft, fc) = match &d.fix {
        Some(f) => (Some(f.target.clone()), Some(f.content.clone())),
        None => (None, None),
    };
    let now = chrono::Utc::now().timestamp();
    let result = sqlx::query(
        "INSERT INTO ai_build_diagnosis \
         (deployment_id, service_id, classification, root_cause, explanation, \
          fix_target, fix_content, error_signature, created_at) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?) \
         ON CONFLICT(deployment_id) DO UPDATE SET \
          service_id = excluded.service_id, classification = excluded.classification, \
          root_cause = excluded.root_cause, explanation = excluded.explanation, \
          fix_target = excluded.fix_target, fix_content = excluded.fix_content, \
          error_signature = excluded.error_signature, created_at = excluded.created_at",
    )
    .bind(deployment_id)
    .bind(service_id)
    .bind(d.classification.as_str())
    .bind(&d.root_cause)
    .bind(&d.explanation)
    .bind(ft)
    .bind(fc)
    .bind(error_signature)
    .bind(now)
    .execute(&*pool)
    .await;
    if let Err(e) = result {
        tracing::warn!(deployment_id, error = %e, "ai build diagnosis store failed");
    }
}

/// Read a stored diagnosis for a deployment.
pub async fn get(deployment_id: i64) -> Option<StoredDiagnosis> {
    let pool = auto_di::resolve::<sqlx::SqlitePool>().await.ok()?;
    let row: Option<(String, String, String, Option<String>, Option<String>)> = sqlx::query_as(
        "SELECT classification, root_cause, explanation, fix_target, fix_content \
         FROM ai_build_diagnosis WHERE deployment_id = ?",
    )
    .bind(deployment_id)
    .fetch_optional(&*pool)
    .await
    .ok()
    .flatten();
    row.map(|(cls, rc, ex, ft, fc)| StoredDiagnosis {
        classification: Classification::parse(&cls),
        root_cause: rc,
        explanation: ex,
        fix_target: ft,
        fix_content: fc,
    })
}

/// Reuse a prior diagnosis with the same error signature (cache).
pub async fn get_by_signature(error_signature: &str) -> Option<Diagnosis> {
    let pool = auto_di::resolve::<sqlx::SqlitePool>().await.ok()?;
    let row: Option<(String, String, String, Option<String>, Option<String>)> = sqlx::query_as(
        "SELECT classification, root_cause, explanation, fix_target, fix_content \
         FROM ai_build_diagnosis WHERE error_signature = ? ORDER BY created_at DESC LIMIT 1",
    )
    .bind(error_signature)
    .fetch_optional(&*pool)
    .await
    .ok()
    .flatten();
    row.map(|(cls, rc, ex, ft, fc)| Diagnosis {
        classification: Classification::parse(&cls),
        root_cause: rc,
        explanation: ex,
        fix: match (ft, fc) {
            (Some(t), Some(c)) if !c.trim().is_empty() => Some(Fix {
                target: t,
                content: c,
            }),
            _ => None,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classification_parse() {
        assert_eq!(Classification::parse("fixable"), Classification::Fixable);
        assert_eq!(
            Classification::parse("code_error"),
            Classification::CodeError
        );
        assert_eq!(
            Classification::parse("CODE-ERROR"),
            Classification::CodeError
        );
        assert_eq!(Classification::parse("weird"), Classification::Unknown);
    }

    #[test]
    fn extract_json_from_fenced() {
        let s = "```json\n{\"classification\":\"fixable\"}\n```";
        assert_eq!(extract_json(s), "{\"classification\":\"fixable\"}");
        let p = "Here you go: {\"a\":1} thanks";
        assert_eq!(extract_json(p), "{\"a\":1}");
    }

    #[test]
    fn signature_ignores_digits_and_is_stable() {
        let a = error_signature("error at line 42: timeout after 30s");
        let b = error_signature("error at line 99: timeout after 10s");
        assert_eq!(a, b); // digits stripped → same signature
        assert_ne!(a, error_signature("permission denied"));
    }
}
