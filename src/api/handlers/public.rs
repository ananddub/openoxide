use std::sync::Arc;

use auto_route::controller;
use axum::Json;
use axum::extract::Query;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::utils::git::GitCli;

#[derive(Deserialize, poem_openapi::Object)]
pub struct BranchQuery {
    pub query: String,
}

#[derive(Deserialize, poem_openapi::Object)]
pub struct DomainQuery {
    pub name: String,
}

#[derive(Serialize, poem_openapi::Object)]
pub struct GeneratedDomainDto {
    pub domain: String,
}

pub struct Public {
    db: Arc<SqlitePool>,
}

#[controller("/public")]
impl Public {
    pub fn new(db: Arc<SqlitePool>) -> Self {
        Self { db }
    }

    #[get("/git/list_branches")]
    pub async fn list_branches(
        &self,
        Query(params): Query<BranchQuery>,
    ) -> Result<Json<Vec<String>>, StatusCode> {
        let repository_url = params.query.trim();
        if repository_url.is_empty() {
            return Err(StatusCode::BAD_REQUEST);
        }

        GitCli::new_local()
            .queries()
            .remote_branches(repository_url, None)
            .await
            .map(|branches| Json(branches.into_iter().map(|branch| branch.name).collect()))
            .map_err(|error| {
                tracing::warn!(
                    repository_url,
                    error = %error,
                    "could not list git remote branches"
                );
                StatusCode::BAD_REQUEST
            })
    }

    /// Generate a domain for a given app name based on the server IP.
    ///
    /// GET /public/domain/generate?name=my-app
    /// → { "domain": "my-app.192-168-1-1.sslip.io" }
    #[get("/domain/generate")]
    pub async fn generate_domain(
        &self,
        Query(params): Query<DomainQuery>,
    ) -> Result<Json<GeneratedDomainDto>, (StatusCode, String)> {
        let name = slugify(&params.name);
        if name.is_empty() {
            return Err((StatusCode::BAD_REQUEST, "name cannot be empty".into()));
        }

        let server_ip =
            sqlx::query_scalar::<_, Option<String>>("SELECT server_ip FROM settings LIMIT 1")
                .fetch_optional(self.db.as_ref())
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
                .flatten();

        let domain = match server_ip.as_deref() {
            Some(ip) if !ip.is_empty() => {
                let dashed_ip = ip.replace('.', "-");
                format!("{name}.{dashed_ip}.sslip.io")
            }
            _ => {
                // No server IP configured — return localhost-based domain.
                format!("{name}.localhost")
            }
        };

        Ok(Json(GeneratedDomainDto { domain }))
    }
}

/// Converts a name to a URL-safe slug: lowercase, alphanumeric + hyphens.
fn slugify(value: &str) -> String {
    value
        .trim()
        .to_ascii_lowercase()
        .chars()
        .fold((String::new(), false), |(mut out, prev_was_sep), ch| {
            if ch.is_ascii_alphanumeric() {
                out.push(ch);
                (out, false)
            } else if !out.is_empty() && !prev_was_sep {
                out.push('-');
                (out, true)
            } else {
                (out, prev_was_sep)
            }
        })
        .0
        .trim_end_matches('-')
        .to_owned()
}
