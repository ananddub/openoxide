use os::string_enum;
use serde::Deserialize;
use std::collections::BTreeSet;

pub type GitProviderKind = crate::utils::provider::GitProviderType;

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum GitTrigger {
        default = Push;

        Push => "PUSH",
        Tag => "TAG",
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PushEvent {
    pub provider: GitProviderKind,
    pub trigger: GitTrigger,
    pub owner: String,
    pub repository: String,
    pub branch: String,
    pub before: Option<String>,
    pub after: Option<String>,
    pub changed_paths: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PullRequestEvent {
    pub provider: GitProviderKind,
    pub owner: String,
    pub repository: String,
    pub number: String,
    pub action: String,
    pub source_branch: String,
    pub source_owner: Option<String>,
    pub source_repository: Option<String>,
    pub target_branch: String,
    pub commit: Option<String>,
    pub author: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WebhookEvent {
    Ping,
    GitRef(PushEvent),
    PullRequest(PullRequestEvent),
}

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
