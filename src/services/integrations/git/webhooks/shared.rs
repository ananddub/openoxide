use serde::Deserialize;
use std::collections::BTreeSet;

#[derive(Deserialize, Default)]
pub(super) struct ChangedCommit {
    #[serde(default)]
    added: Vec<String>,
    #[serde(default)]
    modified: Vec<String>,
    #[serde(default)]
    removed: Vec<String>,
}

pub(super) fn branch_from_ref(value: &str) -> Result<String, String> {
    value
        .strip_prefix("refs/heads/")
        .or_else(|| value.strip_prefix("refs/tags/"))
        .filter(|branch| !branch.is_empty())
        .map(str::to_owned)
        .ok_or_else(|| format!("unsupported Git ref: {value}"))
}

pub(super) fn collect_paths(commits: Vec<ChangedCommit>) -> Vec<String> {
    commits
        .into_iter()
        .flat_map(|commit| {
            commit
                .added
                .into_iter()
                .chain(commit.modified)
                .chain(commit.removed)
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}
