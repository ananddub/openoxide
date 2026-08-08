#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum GitProviderKind {
    Github,
    Gitlab,
    Gitea,
    Bitbucket,
}

impl GitProviderKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Github => "github",
            Self::Gitlab => "gitlab",
            Self::Gitea => "gitea",
            Self::Bitbucket => "bitbucket",
        }
    }
}

impl std::str::FromStr for GitProviderKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "github" => Ok(Self::Github),
            "gitlab" => Ok(Self::Gitlab),
            "gitea" => Ok(Self::Gitea),
            "bitbucket" => Ok(Self::Bitbucket),
            other => Err(format!("unsupported Git provider: {other}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitTrigger {
    Push,
    Tag,
}

impl GitTrigger {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Push => "PUSH",
            Self::Tag => "TAG",
        }
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
