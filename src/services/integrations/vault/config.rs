use serde_json::Value;

pub(crate) fn field(raw: Option<&str>, key: &str) -> Option<String> {
    raw.and_then(|value| serde_json::from_str::<Value>(value).ok())
        .and_then(|value| {
            value[key]
                .as_str()
                .map(str::trim)
                .filter(|v| !v.is_empty())
                .map(str::to_string)
        })
}

pub(crate) fn encode_path(path: &str) -> String {
    path.split('/')
        .map(|segment| urlencoding::encode(segment).into_owned())
        .collect::<Vec<_>>()
        .join("/")
}
