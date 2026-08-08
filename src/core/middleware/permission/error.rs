use axum::{Json, http::StatusCode};
use serde_json::{Value, json};

pub type PermissionRejection = (StatusCode, Json<Value>);

pub fn internal(message: &str) -> PermissionRejection {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "error": message })),
    )
}

pub fn no_organization() -> PermissionRejection {
    (
        StatusCode::FORBIDDEN,
        Json(json!({ "error": "user has no organization membership" })),
    )
}

pub fn denied(required_action: String) -> PermissionRejection {
    (
        StatusCode::FORBIDDEN,
        Json(json!({
            "error": "permission denied",
            "required_action": required_action,
        })),
    )
}

pub fn evaluation(error: sqlx::Error) -> PermissionRejection {
    tracing::error!(%error, "permission evaluation failed");
    internal("failed to evaluate permission")
}
