fn parse_duration_ns(value: &str) -> u64 {
    value
        .strip_suffix("ms")
        .and_then(|v| v.parse::<u64>().ok())
        .map(|v| v * 1_000_000)
        .or_else(|| {
            value
                .strip_suffix('s')
                .and_then(|v| v.parse::<u64>().ok())
                .map(|v| v * 1_000_000_000)
        })
        .unwrap_or(0)
}

fn positional_arg(args: &[String], start: usize, value_flags: &[&str]) -> Option<String> {
    let mut i = start;
    while i < args.len() {
        let arg = args[i].as_str();
        if value_flags.contains(&arg) {
            i += 2;
        } else if arg.starts_with('-') {
            i += 1;
        } else {
            return Some(args[i].clone());
        }
    }
    None
}

fn parse_bytes(input: &str) -> i64 {
    let s = input.trim().to_ascii_lowercase();
    let (number, multiplier) = if let Some(v) = s.strip_suffix("gib") {
        (v, 1024_i64.pow(3))
    } else if let Some(v) = s.strip_suffix("gb") {
        (v, 1_000_000_000)
    } else if let Some(v) = s.strip_suffix("mib") {
        (v, 1024_i64.pow(2))
    } else if let Some(v) = s.strip_suffix("mb") {
        (v, 1_000_000)
    } else if let Some(v) = s.strip_suffix("kib") {
        (v, 1024)
    } else if let Some(v) = s.strip_suffix('g') {
        (v, 1024_i64.pow(3))
    } else if let Some(v) = s.strip_suffix('m') {
        (v, 1024_i64.pow(2))
    } else if let Some(v) = s.strip_suffix('k') {
        (v, 1024)
    } else {
        (s.as_str(), 1)
    };
    number
        .trim()
        .parse::<f64>()
        .map(|v| (v * multiplier as f64) as i64)
        .unwrap_or(0)
}

fn parse_device(value: &str) -> (&str, &str, &str) {
    let mut parts = value.split(':');
    let src = parts.next().unwrap_or(value);
    let dst = parts.next().unwrap_or(src);
    let mode = parts.next().unwrap_or("rwm");
    (src, dst, mode)
}

fn parse_mount(value: &str) -> Option<serde_json::Value> {
    let mut mount = serde_json::json!({"Type": "volume", "ReadOnly": false});
    for part in value.split(',') {
        let (key, val) = part.split_once('=').unwrap_or((part, "true"));
        match key {
            "type" => mount["Type"] = serde_json::json!(val),
            "source" | "src" => mount["Source"] = serde_json::json!(val),
            "target" | "dst" | "destination" => mount["Target"] = serde_json::json!(val),
            "readonly" | "ro" => mount["ReadOnly"] = serde_json::json!(val != "false"),
            "consistency" => mount["Consistency"] = serde_json::json!(val),
            _ => {}
        }
    }
    mount.get("Target")?;
    Some(mount)
}

fn restart_policy(value: &str) -> serde_json::Value {
    let (name, max) = value
        .split_once(':')
        .map(|(n, m)| (n, m.parse::<u64>().unwrap_or(0)))
        .unwrap_or((value, 0));
    serde_json::json!({"Name": name, "MaximumRetryCount": max})
}

pub fn encode_query_public(value: &str) -> String {
    value
        .bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                (b as char).to_string()
            }
            _ => format!("%{b:02X}"),
        })
        .collect()
}

fn encode_query(value: &str) -> String {
    encode_query_public(value)
}
