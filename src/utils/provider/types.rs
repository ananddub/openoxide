use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebhookEvent {
    Push,
    PullRequest,
    Issues,
    Release,
    TagPush,
    IssueComment,
}

impl WebhookEvent {
    pub fn as_github_event(&self) -> &'static str {
        match self {
            Self::Push => "push",
            Self::PullRequest => "pull_request",
            Self::Issues => "issues",
            Self::Release => "release",
            Self::TagPush => "create",
            Self::IssueComment => "issue_comment",
        }
    }

    pub fn as_gitlab_event(&self) -> &'static str {
        match self {
            Self::Push => "push_events",
            Self::PullRequest => "merge_requests_events",
            Self::Issues => "issues_events",
            Self::Release => "releases_events",
            Self::TagPush => "tag_push_events",
            Self::IssueComment => "note_events",
        }
    }

    pub fn as_bitbucket_event(&self) -> &'static str {
        match self {
            Self::Push => "repo:push",
            Self::PullRequest => "pullrequest:created",
            Self::Issues => "issue:created",
            Self::Release => "repo:push",
            Self::TagPush => "repo:push",
            Self::IssueComment => "issue:comment_created",
        }
    }

    pub fn as_gitea_event(&self) -> &'static str {
        match self {
            Self::Push => "push",
            Self::PullRequest => "pull_request",
            Self::Issues => "issues",
            Self::Release => "release",
            Self::TagPush => "create",
            Self::IssueComment => "issue_comment",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CloneProtocol {
    Https,
    Ssh,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GitProviderType {
    Github,
    Gitlab,
    Gitea,
    Bitbucket,
}

impl GitProviderType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Github => "GITHUB",
            Self::Gitlab => "GITLAB",
            Self::Gitea => "GITEA",
            Self::Bitbucket => "BITBUCKET",
        }
    }
}

impl std::str::FromStr for GitProviderType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GITHUB" => Ok(Self::Github),
            "GITLAB" => Ok(Self::Gitlab),
            "GITEA" => Ok(Self::Gitea),
            "BITBUCKET" => Ok(Self::Bitbucket),
            other => Err(format!("Unknown provider type: {other}")),
        }
    }
}

impl std::fmt::Display for GitProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
