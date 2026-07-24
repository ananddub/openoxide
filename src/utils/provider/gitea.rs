use super::{
    client::ProviderClient,
    sync::ProviderSyncBuilder,
    types::{CloneProtocol, WebhookEvent},
};
use reqwest::Method;

pub struct GiteaClient {
    client: ProviderClient,
    base_url: String,
}

impl GiteaClient {
    pub fn new(base_url: &str, token: Option<String>) -> Result<Self, String> {
        Ok(Self {
            client: ProviderClient::new(token),
            base_url: base_url.trim_end_matches('/').to_string(),
        })
    }

    pub fn repository<'a>(&'a self, owner: &'a str, repo: &'a str) -> GiteaRepoBuilder<'a> {
        GiteaRepoBuilder {
            client: &self.client,
            base_url: &self.base_url,
            owner,
            repo,
        }
    }
}

pub struct GiteaRepoBuilder<'a> {
    client: &'a ProviderClient,
    base_url: &'a str,
    owner: &'a str,
    repo: &'a str,
}

impl<'a> GiteaRepoBuilder<'a> {
    pub fn clone_url(&self, protocol: CloneProtocol) -> String {
        let clean_base = self
            .base_url
            .trim_start_matches("https://")
            .trim_start_matches("http://");
        match protocol {
            CloneProtocol::Https => format!("{}/{}/{}.git", self.base_url, self.owner, self.repo),
            CloneProtocol::Ssh => format!("git@{}:{}/{}.git", clean_base, self.owner, self.repo),
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
            "{}/api/v1/repos/{}/{}",
            self.base_url, self.owner, self.repo
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

    pub fn webhooks(&self) -> GiteaWebhookBuilder<'a> {
        GiteaWebhookBuilder {
            client: self.client,
            base_url: self.base_url,
            owner: self.owner,
            repo: self.repo,
            webhook_url: None,
            events: vec![],
            active: true,
        }
    }
}

pub struct GiteaWebhookBuilder<'a> {
    client: &'a ProviderClient,
    base_url: &'a str,
    owner: &'a str,
    repo: &'a str,
    webhook_url: Option<&'a str>,
    events: Vec<WebhookEvent>,
    active: bool,
}

impl<'a> GiteaWebhookBuilder<'a> {
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
            "{}/api/v1/repos/{}/{}/hooks",
            self.base_url, self.owner, self.repo
        );

        let event_strings: Vec<&str> = self.events.iter().map(|e| e.as_gitea_event()).collect();

        let payload = serde_json::json!({
            "type": "gitea",
            "active": self.active,
            "events": if event_strings.is_empty() { vec!["push"] } else { event_strings },
            "config": {
                "url": target_url,
                "content_type": "json"
            }
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
