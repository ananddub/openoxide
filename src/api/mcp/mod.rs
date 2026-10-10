//! Model Context Protocol (MCP) server for OpenOxide.
//!
//! Lets any external AI client (Claude Desktop, Cursor, Cline, Zed, …) drive
//! the panel through the **same in-process Connect/gRPC dispatch** the built-in
//! assistant uses — no second HTTP hop, no duplicate action list.
//!
//! - Transport: MCP Streamable HTTP — JSON-RPC 2.0 over `POST /mcp`.
//! - Tools: every non-`@internal`, router-registered catalog action becomes an
//!   MCP tool (name = action, description = proto comment, inputSchema derived
//!   from the action's params). Plus a `search_actions` helper for the large
//!   catalog.
//! - Auth: the client sends the user's Bearer token / PAT in `Authorization`;
//!   every `tools/call` runs through `invoke` under that identity, so the same
//!   permission + validation + audit apply as a manual API call.

use serde_json::{Value, json};

use crate::core::middleware::auth_context::AuthContext;
use crate::services::platform::ai::agent::{catalog, invoke};

const PROTOCOL_VERSION: &str = "2024-11-05";
const SEARCH_TOOL: &str = "search_actions";

/// Handle one JSON-RPC request body and return the JSON-RPC response value.
/// `authorization` / `organization` are the raw request headers used to build
/// the per-call identity for `tools/call`. Returns `None` for notifications
/// (no response expected, e.g. `notifications/initialized`).
pub async fn handle_jsonrpc(
    body: &[u8],
    authorization: Option<&str>,
    organization: Option<&str>,
) -> Option<Value> {
    let req: Value = match serde_json::from_slice(body) {
        Ok(v) => v,
        Err(e) => return Some(error_response(Value::Null, -32700, &format!("parse error: {e}"))),
    };

    // Batch requests: handle each, drop notification nulls.
    if let Value::Array(items) = req {
        let mut out = Vec::new();
        for item in items {
            if let Some(resp) = handle_one(item, authorization, organization).await {
                out.push(resp);
            }
        }
        return if out.is_empty() {
            None
        } else {
            Some(Value::Array(out))
        };
    }

    handle_one(req, authorization, organization).await
}

async fn handle_one(
    req: Value,
    authorization: Option<&str>,
    organization: Option<&str>,
) -> Option<Value> {
    let id = req.get("id").cloned().unwrap_or(Value::Null);
    let method = req.get("method").and_then(|m| m.as_str()).unwrap_or("");
    let params = req.get("params").cloned().unwrap_or(Value::Null);

    // Notifications carry no id and expect no response.
    let is_notification = req.get("id").is_none();

    match method {
        "initialize" => Some(ok_response(id, initialize_result())),
        "notifications/initialized" | "notifications/cancelled" => None,
        "ping" => Some(ok_response(id, json!({}))),
        "tools/list" => Some(ok_response(id, tools_list_result().await)),
        "tools/call" => {
            let resp = tools_call(params, authorization, organization).await;
            Some(match resp {
                Ok(result) => ok_response(id, result),
                Err((code, msg)) => error_response(id, code, &msg),
            })
        }
        _ if is_notification => None,
        _ => Some(error_response(id, -32601, &format!("method not found: {method}"))),
    }
}

fn initialize_result() -> Value {
    json!({
        "protocolVersion": PROTOCOL_VERSION,
        "capabilities": { "tools": { "listChanged": false } },
        "serverInfo": { "name": "openoxide", "version": env!("CARGO_PKG_VERSION") }
    })
}

/// Build the MCP tool list from the catalog (registered, non-internal actions)
/// plus the `search_actions` helper.
async fn tools_list_result() -> Value {
    let registered = invoke::registered_paths().await;
    let mut tools = Vec::new();

    // Meta tool: search the large catalog by keyword.
    tools.push(json!({
        "name": SEARCH_TOOL,
        "description": "Search OpenOxide's action catalog by keyword (e.g. \"deploy compose\", \"list databases\"). Returns matching action names you can then call directly.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "query": { "type": "string", "description": "Keywords to search for." }
            },
            "required": ["query"]
        }
    }));

    for a in catalog::GENERATED_ACTIONS {
        if !registered.is_empty() && !registered.contains(a.connect_path) {
            continue;
        }
        tools.push(json!({
            "name": a.name,
            "description": a.description,
            "inputSchema": params_to_schema(a.params),
        }));
    }

    json!({ "tools": tools })
}

/// Execute a `tools/call`.
async fn tools_call(
    params: Value,
    authorization: Option<&str>,
    organization: Option<&str>,
) -> Result<Value, (i64, String)> {
    let name = params
        .get("name")
        .and_then(|n| n.as_str())
        .ok_or((-32602, "missing tool name".to_string()))?;
    let args = params
        .get("arguments")
        .cloned()
        .unwrap_or_else(|| json!({}));

    // The search helper does not need auth — it only reads the static catalog.
    if name == SEARCH_TOOL {
        let query = args.get("query").and_then(|q| q.as_str()).unwrap_or("");
        let registered = invoke::registered_paths().await;
        let text = catalog::search(query, &registered, 25);
        return Ok(tool_text(text, false));
    }

    // Everything else runs under the caller's identity.
    let auth = AuthContext::from_headers(authorization, organization)
        .await
        .map_err(|e| (-32001, e.message()))?;
    let token = authorization
        .and_then(|v| {
            v.strip_prefix("Bearer ")
                .or_else(|| v.strip_prefix("bearer "))
        })
        .unwrap_or_default()
        .to_string();

    let ctx = invoke::UserContext {
        token,
        organization_id: auth.organization_id,
        user_id: auth.user_id(),
    };

    // MCP tool calls are explicit user intent from the external AI, so a
    // destructive action is treated as confirmed (the catalog still gates which
    // actions exist at all; @internal/credential ones are never exposed).
    match invoke::invoke(&ctx, name, args, true).await {
        invoke::InvokeOutcome::Done { body } => Ok(tool_text(body, false)),
        invoke::InvokeOutcome::NeedsConfirmation { summary, .. } => Ok(tool_text(summary, false)),
        invoke::InvokeOutcome::UnknownAction { name } => {
            Ok(tool_text(format!("unknown action: {name}"), true))
        }
    }
}

/// Wrap a string as an MCP tool result (content array of one text block).
fn tool_text(text: String, is_error: bool) -> Value {
    json!({
        "content": [ { "type": "text", "text": text } ],
        "isError": is_error
    })
}

/// Convert a catalog params string into a JSON Schema object.
///
/// The catalog encodes params as `{ name!:type, other?:type, kind!:enum(A|B) }`
/// where `!` marks a required field, `?` optional, and `enum(..)` lists the
/// accepted values. `{}` means no parameters.
pub fn params_to_schema(params: &str) -> Value {
    let inner = params.trim();
    let inner = inner
        .strip_prefix('{')
        .and_then(|s| s.strip_suffix('}'))
        .unwrap_or(inner)
        .trim();

    let mut properties = serde_json::Map::new();
    let mut required: Vec<Value> = Vec::new();

    if !inner.is_empty() {
        for field in split_top_level(inner) {
            let field = field.trim();
            if field.is_empty() {
                continue;
            }
            // Split "name<!|?>:type".
            let Some(colon) = field.find(':') else {
                continue;
            };
            let (name_marker, raw_type) = field.split_at(colon);
            let raw_type = raw_type[1..].trim();
            let name_marker = name_marker.trim();

            let (name, is_required) = if let Some(n) = name_marker.strip_suffix('!') {
                (n.trim(), true)
            } else if let Some(n) = name_marker.strip_suffix('?') {
                (n.trim(), false)
            } else {
                (name_marker, false)
            };
            if name.is_empty() {
                continue;
            }

            properties.insert(name.to_string(), type_schema(raw_type));
            if is_required {
                required.push(Value::String(name.to_string()));
            }
        }
    }

    json!({
        "type": "object",
        "properties": Value::Object(properties),
        "required": required
    })
}

/// Map a catalog type hint to a JSON Schema fragment.
fn type_schema(raw: &str) -> Value {
    let raw = raw.trim();
    if let Some(values) = raw.strip_prefix("enum(").and_then(|s| s.strip_suffix(')')) {
        let variants: Vec<Value> = values
            .split('|')
            .map(|v| Value::String(v.trim().to_string()))
            .filter(|v| v.as_str().map(|s| !s.is_empty()).unwrap_or(false))
            .collect();
        return json!({ "type": "string", "enum": variants });
    }
    match raw {
        "number" => json!({ "type": "number" }),
        "boolean" => json!({ "type": "boolean" }),
        "object" => json!({ "type": "object" }),
        _ => json!({ "type": "string" }),
    }
}

/// Split a param list on top-level commas, ignoring commas inside `enum(...)`.
fn split_top_level(s: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut cur = String::new();
    for c in s.chars() {
        match c {
            '(' | '{' | '[' => {
                depth += 1;
                cur.push(c);
            }
            ')' | '}' | ']' => {
                depth -= 1;
                cur.push(c);
            }
            ',' if depth == 0 => {
                parts.push(std::mem::take(&mut cur));
            }
            _ => cur.push(c),
        }
    }
    if !cur.trim().is_empty() {
        parts.push(cur);
    }
    parts
}

fn ok_response(id: Value, result: Value) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "result": result })
}

fn error_response(id: Value, code: i64, message: &str) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "error": { "code": code, "message": message } })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_from_empty_params() {
        let s = params_to_schema("{}");
        assert_eq!(s["type"], "object");
        assert_eq!(s["properties"], json!({}));
        assert_eq!(s["required"], json!([]));
    }

    #[test]
    fn schema_marks_required_and_optional() {
        let s = params_to_schema("{ id!:number, env_var?:string }");
        assert_eq!(s["properties"]["id"]["type"], "number");
        assert_eq!(s["properties"]["env_var"]["type"], "string");
        assert_eq!(s["required"], json!(["id"]));
    }

    #[test]
    fn schema_handles_enum_with_internal_pipe() {
        let s = params_to_schema("{ kind!:enum(A|B|C), name?:string }");
        assert_eq!(s["properties"]["kind"]["type"], "string");
        assert_eq!(s["properties"]["kind"]["enum"], json!(["A", "B", "C"]));
        // The pipe inside enim(...) must not be split as a field separator.
        assert_eq!(s["required"], json!(["kind"]));
        assert!(s["properties"].get("name").is_some());
    }

    #[test]
    fn initialize_has_protocol_and_tools_capability() {
        let r = initialize_result();
        assert_eq!(r["protocolVersion"], PROTOCOL_VERSION);
        assert!(r["capabilities"]["tools"].is_object());
        assert_eq!(r["serverInfo"]["name"], "openoxide");
    }

    #[tokio::test]
    async fn initialize_over_jsonrpc() {
        let body = br#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#;
        let resp = handle_jsonrpc(body, None, None).await.expect("response");
        assert_eq!(resp["id"], 1);
        assert_eq!(resp["result"]["serverInfo"]["name"], "openoxide");
    }

    #[tokio::test]
    async fn notification_has_no_response() {
        let body = br#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#;
        assert!(handle_jsonrpc(body, None, None).await.is_none());
    }

    #[tokio::test]
    async fn tools_list_includes_search_and_actions() {
        let body = br#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#;
        let resp = handle_jsonrpc(body, None, None).await.expect("response");
        let tools = resp["result"]["tools"].as_array().expect("tools array");
        assert!(tools.iter().any(|t| t["name"] == SEARCH_TOOL));
        // The catalog (tests fail-open on registered set) should expose many.
        assert!(tools.len() > 50, "expected many tools, got {}", tools.len());
        // Internal/credential actions must never be present.
        assert!(!tools.iter().any(|t| t["name"] == "auth_login"));
        assert!(!tools.iter().any(|t| t["name"] == "registry_rotate_credentials"));
    }

    #[tokio::test]
    async fn unknown_method_errors() {
        let body = br#"{"jsonrpc":"2.0","id":3,"method":"bogus/method"}"#;
        let resp = handle_jsonrpc(body, None, None).await.expect("response");
        assert_eq!(resp["error"]["code"], -32601);
    }

    #[tokio::test]
    async fn search_tool_needs_no_auth() {
        let body = br#"{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"search_actions","arguments":{"query":"deploy compose"}}}"#;
        let resp = handle_jsonrpc(body, None, None).await.expect("response");
        let text = resp["result"]["content"][0]["text"].as_str().unwrap_or("");
        assert!(text.to_lowercase().contains("compose"), "got: {text}");
    }
}
