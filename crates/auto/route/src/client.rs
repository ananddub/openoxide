#[doc(hidden)]
pub struct LiveClientRouteDescriptor {
    pub client_name: &'static str,
    pub namespace: &'static str,
    pub endpoint: &'static str,
    pub event: &'static str,
    pub path: &'static str,
    pub arguments: &'static [LiveClientArgumentDescriptor],
}

impl LiveClientRouteDescriptor {
    pub const fn new(
        client_name: &'static str,
        namespace: &'static str,
        endpoint: &'static str,
        event: &'static str,
        path: &'static str,
        arguments: &'static [LiveClientArgumentDescriptor],
    ) -> Self {
        Self {
            client_name,
            namespace,
            endpoint,
            event,
            path,
            arguments,
        }
    }
}

inventory::collect!(LiveClientRouteDescriptor);

#[doc(hidden)]
#[derive(Clone, Copy, serde::Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum LiveClientArgumentDescriptor {
    Path {
        index: usize,
        names: &'static [&'static str],
    },
    Query {
        index: usize,
    },
}

pub fn live_client_manifest() -> serde_json::Value {
    let endpoints = inventory::iter::<LiveClientRouteDescriptor>
        .into_iter()
        .map(|item| serde_json::json!({
            "hook": format!("use{}", pascal_case(item.client_name)),
            "namespace": item.namespace,
            "endpoint": item.endpoint,
            "event": item.event,
            "path": item.path,
            "arguments": item.arguments,
            "parameters": (0..item.arguments.len()).map(|index| format!("arg{index}: unknown")).collect::<Vec<_>>().join(", "),
            "result": "unknown",
        }))
        .collect::<Vec<_>>();
    serde_json::json!({"types": [], "endpoints": endpoints})
}

fn pascal_case(value: &str) -> String {
    value
        .split(|character: char| !character.is_ascii_alphanumeric())
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            chars
                .next()
                .map(|first| first.to_ascii_uppercase().to_string() + chars.as_str())
                .unwrap_or_default()
        })
        .collect()
}
