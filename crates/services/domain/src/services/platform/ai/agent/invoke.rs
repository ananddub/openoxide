//! Runs a catalog action by dispatching to the panel's own Connect-RPC router **in-process**, so the AI can only do what the user could do manually.

use std::collections::HashSet;
use std::sync::Arc;

use connectrpc::ConnectRpcService;
use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::{Request, Response};
use serde_json::Value;
use tokio::sync::OnceCell;
use tower::Service;

use super::catalog;

/// Per-request identity; `token` makes every dispatched call permission-checked exactly like a manual API call.
#[derive(Clone)]
pub struct UserContext {
    pub token: String,
    pub organization_id: i64,
    pub user_id: i64,
}

pub enum InvokeOutcome {
    Done { body: String },
    NeedsConfirmation { action: String, summary: String },
    UnknownAction { name: String },
}

/// The in-process Connect service, built once from the global DI container's registered routes.
static SERVICE: OnceCell<Arc<ConnectRpcService>> = OnceCell::const_new();

async fn connect_service() -> Result<Arc<ConnectRpcService>, String> {
    SERVICE
        .get_or_try_init(|| async {
            let router = auto_grpc::connect_routes()
                .await
                .map_err(|e| format!("could not build connect routes: {e}"))?;
            Ok(Arc::new(ConnectRpcService::new(router)))
        })
        .await
        .cloned()
}

/// Connect paths actually registered: some RPCs are declared in proto but never implemented, so the catalog is filtered against this set.
static REGISTERED: OnceCell<HashSet<String>> = OnceCell::const_new();

pub async fn registered_paths() -> HashSet<String> {
    // Tests have no DI container / DB, so fail open (empty set = "don't filter") to keep the loop testable.
    #[cfg(test)]
    {
        return HashSet::new();
    }
    #[cfg(not(test))]
    REGISTERED
        .get_or_init(|| async {
            match auto_grpc::connect_routes().await {
                Ok(router) => router.methods().map(|m| m.to_string()).collect(),
                // On failure, an empty set means "don't filter" (see is_registered).
                Err(_) => HashSet::new(),
            }
        })
        .await
        .clone()
}

/// True if the path is registered, or the route set is unknown (fail-open).
fn is_registered(set: &HashSet<String>, connect_path: &str) -> bool {
    set.is_empty() || set.contains(connect_path)
}

/// Run one action. `confirmed` must be true to execute a destructive action.
pub async fn invoke(
    ctx: &UserContext,
    action_name: &str,
    params: Value,
    confirmed: bool,
) -> InvokeOutcome {
    let Some(action) = catalog::find(action_name) else {
        return InvokeOutcome::UnknownAction {
            name: action_name.to_string(),
        };
    };

    // Reject phantom actions (proto-declared, not router-registered) so the model retries with a real one.
    let registered = registered_paths().await;
    if !is_registered(&registered, action.connect_path) {
        return InvokeOutcome::UnknownAction {
            name: action_name.to_string(),
        };
    }

    if action.destructive && !confirmed {
        return InvokeOutcome::NeedsConfirmation {
            action: action.name.to_string(),
            summary: format!(
                "This will run `{}` ({}). Confirm before it is executed.",
                action.name, action.description
            ),
        };
    }

    match dispatch(ctx, action.connect_path, params).await {
        Ok(body) => InvokeOutcome::Done { body },
        Err(e) => InvokeOutcome::Done {
            body: format!("action failed: {e}"),
        },
    }
}

/// Dispatch a Connect unary call to the in-process router, running the full auth -> permission -> validation -> handler chain.
async fn dispatch(ctx: &UserContext, connect_path: &str, params: Value) -> Result<String, String> {
    let service = connect_service().await?;

    let body_bytes = serde_json::to_vec(&if params.is_null() {
        serde_json::json!({})
    } else {
        params
    })
    .map_err(|e| format!("encode params: {e}"))?;

    let request = Request::builder()
        .method("POST")
        .uri(format!("/{connect_path}"))
        .header("content-type", "application/json")
        .header("connect-protocol-version", "1")
        .header("authorization", format!("Bearer {}", ctx.token))
        .header("x-organization-id", ctx.organization_id.to_string())
        .body(Full::new(Bytes::from(body_bytes)))
        .map_err(|e| format!("build request: {e}"))?;

    let mut svc = (*service).clone();
    let response: Response<_> = svc
        .call(request)
        .await
        .map_err(|e| format!("dispatch failed: {e}"))?;

    let status = response.status();
    let collected = response
        .into_body()
        .collect()
        .await
        .map_err(|e| format!("read response body: {e}"))?;
    let text = String::from_utf8_lossy(&collected.to_bytes()).to_string();

    if status.is_success() {
        Ok(text)
    } else {
        // Connect encodes permission/validation failures as a JSON error body; hand it to the model.
        Err(format!("status {status}: {text}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn unknown_action_is_reported() {
        let ctx = UserContext {
            token: "t".into(),
            organization_id: 1,
            user_id: 1,
        };
        match invoke(&ctx, "does_not_exist", Value::Null, false).await {
            InvokeOutcome::UnknownAction { name } => assert_eq!(name, "does_not_exist"),
            _ => panic!("expected UnknownAction"),
        }
    }

    #[tokio::test]
    async fn destructive_requires_confirmation() {
        let ctx = UserContext {
            token: "t".into(),
            organization_id: 1,
            user_id: 1,
        };
        match invoke(
            &ctx,
            "compose_delete_compose",
            serde_json::json!({"id": 1}),
            false,
        )
        .await
        {
            InvokeOutcome::NeedsConfirmation { action, .. } => {
                assert_eq!(action, "compose_delete_compose")
            }
            _ => panic!("expected NeedsConfirmation"),
        }
    }
}
