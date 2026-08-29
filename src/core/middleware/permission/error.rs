use axum::{Json, http::StatusCode};
use serde_json::{Value, json};

pub type PermissionRejection = (StatusCode, Json<Value>);

pub fn internal(message: &str) -> PermissionRejection {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "error": message })),
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
