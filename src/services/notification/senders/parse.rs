use serde_json::Value;

pub fn parse_addresses(raw: &str) -> Vec<String> {
    if let Ok(Value::Array(items)) = serde_json::from_str::<Value>(raw) {
        return items
            .into_iter()
            .filter_map(|v| v.as_str().map(str::to_string))
            .filter(|s| !s.trim().is_empty())
            .collect();
    }

    raw.split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}
