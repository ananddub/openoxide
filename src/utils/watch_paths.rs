use serde_json::Value;

/// Returns whether a webhook containing `changed_paths` should trigger a deploy.
///
/// Empty watch rules mean "deploy on every matching branch". Rules accept a
/// JSON string array (the persisted API format), newline/comma separated text,
/// directory prefixes and `*`/`**` glob segments. A rule prefixed with `!`
/// excludes a path after it was included by another rule.
pub fn should_deploy(raw_rules: Option<&str>, changed_paths: &[String]) -> Result<bool, String> {
    let rules = parse_rules(raw_rules)?;
    if rules.is_empty() || changed_paths.is_empty() {
        return Ok(true);
    }

    Ok(changed_paths.iter().any(|path| {
        let path = normalize(path);
        let mut included = false;
        for rule in &rules {
            if matches_pattern(&rule.pattern, &path) {
                included = !rule.exclude;
            }
        }
        included
    }))
}

#[derive(Debug, PartialEq, Eq)]
struct Rule {
    pattern: String,
    exclude: bool,
}

fn parse_rules(raw: Option<&str>) -> Result<Vec<Rule>, String> {
    let Some(raw) = raw.map(str::trim).filter(|value| !value.is_empty()) else {
        return Ok(Vec::new());
    };

    let values = if raw.starts_with('[') {
        let value: Value = serde_json::from_str(raw)
            .map_err(|error| format!("watch_paths must be a JSON string array: {error}"))?;
        value
            .as_array()
            .ok_or_else(|| "watch_paths must be a JSON string array".to_string())?
            .iter()
            .map(|item| {
                item.as_str()
                    .map(str::to_owned)
                    .ok_or_else(|| "watch_paths entries must be strings".to_string())
            })
            .collect::<Result<Vec<_>, _>>()?
    } else {
        raw.split([',', '\n'])
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_owned)
            .collect()
    };

    Ok(values
        .into_iter()
        .filter_map(|value| {
            let value = value.trim();
            let (exclude, value) = value
                .strip_prefix('!')
                .map_or((false, value), |value| (true, value));
            let pattern = normalize(value).trim_end_matches('/').to_owned();
            (!pattern.is_empty()).then_some(Rule { pattern, exclude })
        })
        .collect())
}

fn normalize(value: &str) -> String {
    value
        .trim()
        .trim_start_matches("./")
        .trim_start_matches('/')
        .replace('\\', "/")
}

fn matches_pattern(pattern: &str, path: &str) -> bool {
    if !pattern.contains('*') {
        return path == pattern || path.starts_with(&format!("{pattern}/"));
    }
    glob_match(pattern.as_bytes(), path.as_bytes())
}

fn glob_match(pattern: &[u8], path: &[u8]) -> bool {
    match pattern {
        [] => path.is_empty(),
        [b'*', b'*', rest @ ..] => {
            let rest = rest.strip_prefix(b"/").unwrap_or(rest);
            glob_match(rest, path) || (!path.is_empty() && glob_match(pattern, &path[1..]))
        }
        [b'*', rest @ ..] => {
            glob_match(rest, path)
                || (!path.is_empty() && path[0] != b'/' && glob_match(pattern, &path[1..]))
        }
        [head, rest @ ..] => path.first() == Some(head) && glob_match(rest, &path[1..]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn paths(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_owned()).collect()
    }

    #[test]
    fn empty_rules_deploy_every_change() {
        assert!(should_deploy(None, &paths(&["README.md"])).unwrap());
    }

    #[test]
    fn supports_json_prefix_and_glob_rules() {
        let rules = Some(r#"["apps/api", "packages/**/*.rs"]"#);
        assert!(should_deploy(rules, &paths(&["apps/api/src/main.rs"])).unwrap());
        assert!(should_deploy(rules, &paths(&["packages/core/src/lib.rs"])).unwrap());
        assert!(!should_deploy(rules, &paths(&["docs/index.md"])).unwrap());
    }

    #[test]
    fn later_exclusion_wins() {
        let rules = Some("apps/**,!apps/docs/**");
        assert!(!should_deploy(rules, &paths(&["apps/docs/readme.md"])).unwrap());
        assert!(should_deploy(rules, &paths(&["apps/api/main.rs"])).unwrap());
    }

    #[test]
    fn rejects_non_string_json_entries() {
        assert!(should_deploy(Some(r#"["src", 42]"#), &paths(&["src/main.rs"])).is_err());
    }
}
