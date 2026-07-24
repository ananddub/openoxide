use super::{
    client::ProviderClient,
    sync::ProviderSyncBuilder,
    types::{CloneProtocol, WebhookEvent},
};
use reqwest::Method;

pub struct BitbucketClient {
    client: ProviderClient,
}

impl BitbucketClient {
    pub fn new(token: Option<String>) -> Self {
        Self {
            client: ProviderClient::new(token),
        }
    }

    pub fn repository<'a>(
        &'a self,
        workspace: &'a str,
        repo_slug: &'a str,
    ) -> BitbucketRepoBuilder<'a> {
        BitbucketRepoBuilder {
            client: &self.client,
            workspace,
            repo_slug,
        }
    }
}

pub struct BitbucketRepoBuilder<'a> {
    client: &'a ProviderClient,
    workspace: &'a str,
    repo_slug: &'a str,
}

impl<'a> BitbucketRepoBuilder<'a> {
    pub fn clone_url(&self, protocol: CloneProtocol) -> String {
        match protocol {
            CloneProtocol::Https => format!(
                "https://bitbucket.org/{}/{}.git",
                self.workspace, self.repo_slug
            ),
            CloneProtocol::Ssh => format!(
                "git@bitbucket.org:{}/{}.git",
                self.workspace, self.repo_slug
            ),
        }
    }

    pub fn sync_into(
        &self,
        destination: &'a str,
        protocol: CloneProtocol,
    ) -> ProviderSyncBuilder<'a> {
        ProviderSyncBuilder::new(self.clone_url(protocol), destination)
    }

    pub async fn get(&self) -> Result<String, String> {
        let url = format!(
            "https://api.bitbucket.org/2.0/repositories/{}/{}",
            self.workspace, self.repo_slug
        );
        let req = self
            .client
            .authenticate(self.client.client.request(Method::GET, url));

        req.send()
            .await
            .map_err(|e| e.to_string())?
            .text()
            .await
            .map_err(|e| e.to_string())
    }

    pub fn webhooks(&self) -> BitbucketWebhookBuilder<'a> {
        BitbucketWebhookBuilder {
            client: self.client,
            workspace: self.workspace,
            repo_slug: self.repo_slug,
            webhook_url: None,
            events: vec![],
            active: true,
        }
    }
}

pub struct BitbucketWebhookBuilder<'a> {
    client: &'a ProviderClient,
    workspace: &'a str,
    repo_slug: &'a str,
    webhook_url: Option<&'a str>,
    events: Vec<WebhookEvent>,
    active: bool,
}

impl<'a> BitbucketWebhookBuilder<'a> {
    pub fn create(mut self, url: &'a str) -> Self {
        self.webhook_url = Some(url);
        self
    }

    pub fn events(mut self, events: Vec<WebhookEvent>) -> Self {
        self.events = events;
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub async fn run(self) -> Result<String, String> {
        let target_url = self
            .webhook_url
            .ok_or_else(|| "Webhook URL is required to create a webhook".to_string())?;

        let url = format!(
            "https://api.bitbucket.org/2.0/repositories/{}/{}/hooks",
            self.workspace, self.repo_slug
        );

        let event_strings: Vec<&str> = self.events.iter().map(|e| e.as_bitbucket_event()).collect();

        let payload = serde_json::json!({
            "description": "rustploy-webhook",
            "url": target_url,
            "active": self.active,
            "events": if event_strings.is_empty() { vec!["repo:push"] } else { event_strings },
            "skip_cert_verification": true
        });

        let req = self
            .client
            .authenticate(self.client.client.post(url).json(&payload));

        req.send()
            .await
            .map_err(|e| e.to_string())?
            .text()
            .await
            .map_err(|e| e.to_string())
    }
}
