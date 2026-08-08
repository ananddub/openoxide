use serde::Deserialize;

use super::types::{GitProviderKind, PullRequestEvent};

#[derive(Deserialize)]
struct RepositoryOwner {
    login: Option<String>,
    username: Option<String>,
    nickname: Option<String>,
}

#[derive(Deserialize)]
struct Repository {
    name: String,
    owner: RepositoryOwner,
}

#[derive(Deserialize)]
struct GithubRepo {
    name: String,
    owner: RepositoryOwner,
}

#[derive(Deserialize)]
struct GithubRef {
    #[serde(rename = "ref")]
    branch: String,
    sha: Option<String>,
    repo: Option<GithubRepo>,
}

#[derive(Deserialize)]
struct GithubPullRequest {
    head: GithubRef,
    base: GithubRef,
    user: RepositoryOwner,
}

#[derive(Deserialize)]
struct GithubPayload {
    action: Option<String>,
    number: i64,
    repository: Repository,
    pull_request: GithubPullRequest,
}

#[derive(Deserialize)]
struct GitlabProject {
    path: String,
    path_with_namespace: String,
}

#[derive(Deserialize)]
struct GitlabUser {
    username: Option<String>,
}

#[derive(Deserialize)]
struct GitlabCommit {
    id: String,
}

#[derive(Deserialize)]
struct GitlabAttributes {
    iid: i64,
    action: Option<String>,
    state: Option<String>,
    source_branch: String,
    target_branch: String,
    last_commit: Option<GitlabCommit>,
}

#[derive(Deserialize)]
struct GitlabPayload {
    project: GitlabProject,
    source: Option<GitlabProject>,
    user: Option<GitlabUser>,
    object_attributes: GitlabAttributes,
}

#[derive(Deserialize)]
struct BitbucketBranch {
    name: String,
}

#[derive(Deserialize)]
struct BitbucketCommit {
    hash: String,
}

#[derive(Deserialize)]
struct BitbucketRepo {
    name: String,
    full_name: Option<String>,
}

#[derive(Deserialize)]
struct BitbucketSide {
    branch: BitbucketBranch,
    commit: Option<BitbucketCommit>,
    repository: Option<BitbucketRepo>,
}

#[derive(Deserialize)]
struct BitbucketPullRequest {
    id: i64,
    source: BitbucketSide,
    destination: BitbucketSide,
    author: RepositoryOwner,
}

#[derive(Deserialize)]
struct BitbucketPayload {
    repository: Repository,
    pullrequest: BitbucketPullRequest,
}

pub(super) fn parse(
    provider: GitProviderKind,
    event_name: &str,
    body: &[u8],
) -> Result<PullRequestEvent, String> {
    match provider {
        GitProviderKind::Github | GitProviderKind::Gitea => github_like(provider, body),
        GitProviderKind::Gitlab => gitlab(body),
        GitProviderKind::Bitbucket => bitbucket(event_name, body),
    }
}

fn github_like(provider: GitProviderKind, body: &[u8]) -> Result<PullRequestEvent, String> {
    let payload: GithubPayload = decode(body)?;
    Ok(PullRequestEvent {
        provider,
        owner: owner_name(&payload.repository.owner).ok_or("repository owner is missing")?,
        repository: payload.repository.name,
        number: payload.number.to_string(),
        action: payload.action.unwrap_or_else(|| "updated".into()),
        source_branch: payload.pull_request.head.branch,
        source_owner: payload
            .pull_request
            .head
            .repo
            .as_ref()
            .and_then(|repo| owner_name(&repo.owner)),
        source_repository: payload.pull_request.head.repo.map(|repo| repo.name),
        target_branch: payload.pull_request.base.branch,
        commit: payload.pull_request.head.sha,
        author: owner_name(&payload.pull_request.user),
    })
}

fn gitlab(body: &[u8]) -> Result<PullRequestEvent, String> {
    let payload: GitlabPayload = decode(body)?;
    let owner =
        namespace(&payload.project.path_with_namespace).ok_or("project namespace is missing")?;
    Ok(PullRequestEvent {
        provider: GitProviderKind::Gitlab,
        owner,
        repository: payload.project.path,
        number: payload.object_attributes.iid.to_string(),
        action: payload
            .object_attributes
            .action
            .or(payload.object_attributes.state)
            .unwrap_or_else(|| "update".into()),
        source_branch: payload.object_attributes.source_branch,
        source_owner: payload
            .source
            .as_ref()
            .and_then(|source| namespace(&source.path_with_namespace)),
        source_repository: payload.source.map(|source| source.path),
        target_branch: payload.object_attributes.target_branch,
        commit: payload
            .object_attributes
            .last_commit
            .map(|commit| commit.id),
        author: payload.user.and_then(|user| user.username),
    })
}

fn bitbucket(event_name: &str, body: &[u8]) -> Result<PullRequestEvent, String> {
    let payload: BitbucketPayload = decode(body)?;
    let pull = payload.pullrequest;
    Ok(PullRequestEvent {
        provider: GitProviderKind::Bitbucket,
        owner: owner_name(&payload.repository.owner).ok_or("repository owner is missing")?,
        repository: payload.repository.name,
        number: pull.id.to_string(),
        action: event_name
            .strip_prefix("pullrequest:")
            .unwrap_or("updated")
            .to_owned(),
        source_branch: pull.source.branch.name,
        source_owner: pull
            .source
            .repository
            .as_ref()
            .and_then(|repo| repo.full_name.as_deref())
            .and_then(namespace),
        source_repository: pull.source.repository.map(|repo| repo.name),
        target_branch: pull.destination.branch.name,
        commit: pull.source.commit.map(|commit| commit.hash),
        author: owner_name(&pull.author),
    })
}

fn decode<T: for<'de> Deserialize<'de>>(body: &[u8]) -> Result<T, String> {
    serde_json::from_slice(body).map_err(|error| format!("invalid pull request payload: {error}"))
}

fn owner_name(owner: &RepositoryOwner) -> Option<String> {
    owner
        .login
        .clone()
        .or_else(|| owner.username.clone())
        .or_else(|| owner.nickname.clone())
}

fn namespace(name: &str) -> Option<String> {
    name.rsplit_once('/').map(|(owner, _)| owner.to_owned())
}
