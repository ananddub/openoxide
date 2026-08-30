use os::string_enum;

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
