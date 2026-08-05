pub use bitbucket::BitbucketClient;
pub use custom::CustomClient;
pub use gitea::GiteaClient;
pub use github::GithubClient;
pub use gitlab::GitlabClient;
pub use types::{CloneProtocol, WebhookEvent};

pub mod bitbucket;
pub mod client;
pub mod custom;
pub mod discovery;
pub mod gitea;
pub mod github;
pub mod gitlab;
pub mod oauth;
pub mod sync;
pub mod types;
