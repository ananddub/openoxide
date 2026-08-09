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

use os::string_enum;

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum GitProviderType {
        default = Github;

        Github => "GITHUB",
        Gitlab => "GITLAB",
        Gitea => "GITEA",
        Bitbucket => "BITBUCKET",
    }
}
