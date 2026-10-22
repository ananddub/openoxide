//! The assistant's action catalog, **generated at build time** from the proto descriptor set (`crates/proto/build.rs::generate_ai_catalog`); only the name-based destructive flag and an optional allow-list are layered on here.

pub use openoxide_proto::ai_catalog::{GENERATED_ACTIONS, GeneratedAction};

/// An action the assistant may invoke, enriched with the destructive flag.
#[derive(Debug, Clone, Copy)]
pub struct ActionDef {
    pub name: &'static str,
    pub connect_path: &'static str,
    pub description: &'static str,
    pub params: &'static str,
    pub destructive: bool,
}

/// Classify an action as destructive by name; those need explicit user confirmation before invoke runs them.
fn is_destructive(name: &str) -> bool {
    const MARKERS: &[&str] = &[
        "delete",
        "remove",
        "destroy",
        "stop",
        "reset",
        "purge",
        "prune",
        "rotate",
        "revoke",
        "teardown",
        "uninstall",
        "clear",
        "wipe",
    ];
    MARKERS.iter().any(|m| name.contains(m))
}

/// Look up an action by name, enriching it with the destructive flag.
pub fn find(name: &str) -> Option<ActionDef> {
    GENERATED_ACTIONS
        .iter()
        .find(|a| a.name == name)
        .map(|a| ActionDef {
            name: a.name,
            connect_path: a.connect_path,
            description: a.description,
            params: a.params,
            destructive: is_destructive(a.name),
        })
}

/// Number of available actions (all unary RPCs across services).
pub fn count() -> usize {
    GENERATED_ACTIONS.len()
}

/// Render the catalog as a compact text block for the model; `filter` narrows it.
pub fn describe(filter: Option<&str>) -> String {
    let mut out = String::from("Available actions (use with the `invoke` tool):\n");
    for a in GENERATED_ACTIONS {
        if let Some(f) = filter {
            if !a.name.contains(f) && !a.description.to_lowercase().contains(&f.to_lowercase()) {
                continue;
            }
        }
        out.push_str(&format!(
            "- {}{} — {} params: {}\n",
            a.name,
            if is_destructive(a.name) {
                " [destructive, needs confirmation]"
            } else {
                ""
            },
            a.description,
            a.params,
        ));
    }
    out
}

/// Full catalog as text (no filter).
pub fn describe_all() -> String {
    describe(None)
}

/// Catalog text limited to actions whose connect path is registered in the router (empty set = don't filter).
pub fn describe_registered(registered: &std::collections::HashSet<String>) -> String {
    if registered.is_empty() {
        return describe_all();
    }
    let mut out = String::from("Available actions (use with the `invoke` tool):\n");
    for a in GENERATED_ACTIONS {
        if !registered.contains(a.connect_path) {
            continue;
        }
        out.push_str(&format!(
            "- {}{} — {} params: {}\n",
            a.name,
            if is_destructive(a.name) {
                " [destructive, needs confirmation]"
            } else {
                ""
            },
            a.description,
            a.params,
        ));
    }
    out
}

/// Keyword search over the catalog so the model pulls only the handful of
/// actions relevant to the current request instead of the whole (552-entry)
/// list. Each whitespace-separated query term is matched (case-insensitive)
/// against the action name + description; actions matching more terms rank
/// higher. Only router-registered actions are considered.
pub fn search(query: &str, registered: &std::collections::HashSet<String>, limit: usize) -> String {
    let terms: Vec<String> = query
        .to_lowercase()
        .split_whitespace()
        .filter(|t| t.len() >= 2)
        .map(|t| t.to_string())
        .collect();
    if terms.is_empty() {
        return "Provide a search query, e.g. \"deploy compose\" or \"ssh key\".".into();
    }

    let mut scored: Vec<(i32, &GeneratedAction)> = Vec::new();
    for a in GENERATED_ACTIONS {
        if !registered.is_empty() && !registered.contains(a.connect_path) {
            continue;
        }
        let name = a.name.to_lowercase();
        let desc = a.description.to_lowercase();
        let mut score = 0i32;
        for t in &terms {
            if name.contains(t.as_str()) {
                score += 3;
            }
            if desc.contains(t.as_str()) {
                score += 1;
            }
        }
        if score > 0 {
            scored.push((score, a));
        }
    }
    if scored.is_empty() {
        return format!("No actions match '{query}'. Try different keywords.");
    }
    scored.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.name.cmp(b.1.name)));
    scored.truncate(limit.max(1));

    let mut out = format!("Actions matching '{query}' (use with `invoke`):\n");
    for (_, a) in scored {
        out.push_str(&format!(
            "- {}{} — {} params: {}\n",
            a.name,
            if is_destructive(a.name) {
                " [destructive, needs confirmation]"
            } else {
                ""
            },
            a.description,
            a.params,
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_is_populated_from_proto() {
        // The generated catalog should carry many actions across services.
        assert!(
            count() > 50,
            "expected the generated catalog to be large, got {}",
            count()
        );
    }

    #[test]
    fn proto_comment_becomes_description() {
        // compose_stop has a `//` comment in compose.proto; it should flow
        // through as the action description (not the generic "Stop on …").
        let a = find("compose_stop").expect("action exists");
        assert!(
            a.description.to_lowercase().contains("stop")
                && a.description.len() > "Stop on ComposeService".len(),
            "expected the proto comment as description, got: {}",
            a.description
        );
    }

    #[test]
    fn search_returns_only_relevant_actions() {
        let all = std::collections::HashSet::new(); // empty = don't filter
        let out = search("install template", &all, 25);
        // Must surface the template install action, and NOT be the whole catalog.
        assert!(out.contains("compose_install_template"), "got: {out}");
        let lines = out.lines().count();
        assert!(
            lines > 1 && lines < 60,
            "expected a focused list, got {lines} lines"
        );

        // A nonsense query returns nothing (not the full catalog).
        let none = search("zzzznotathing", &all, 25);
        assert!(
            none.to_lowercase().contains("no actions match"),
            "got: {none}"
        );

        // Empty query is rejected with guidance, not a dump.
        assert!(search("", &all, 25).to_lowercase().contains("search query"));
    }

    #[test]
    fn destructive_classifier_flags_delete_and_stop() {
        assert!(is_destructive("compose_delete_compose"));
        assert!(is_destructive("compose_stop"));
        assert!(!is_destructive("compose_deploy"));
        assert!(!is_destructive("overview_list_organization_services"));
    }

    #[test]
    fn find_enriches_with_destructive() {
        // A deploy action exists and is non-destructive; filter proves lookup works.
        let any_deploy = GENERATED_ACTIONS.iter().find(|a| a.name.contains("deploy"));
        if let Some(a) = any_deploy {
            let found = find(a.name).expect("found");
            assert!(!found.destructive);
        }
    }
}
