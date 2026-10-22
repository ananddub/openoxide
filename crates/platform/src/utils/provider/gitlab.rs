use super::{
    client::ProviderClient,
    sync::ProviderSyncBuilder,
    types::{CloneProtocol, WebhookEvent},
};
use reqwest::Method;

pub struct GitlabClient {
    client: ProviderClient,
    base_url: String,
}

impl GitlabClient {
    pub fn new(token: Option<String>) -> Self {
        Self {
            client: ProviderClient::new(token),
            base_url: "https://gitlab.com".to_string(),
        }
    }

    pub fn new_with_base(base_url: &str, token: Option<String>) -> Self {
        Self {
            client: ProviderClient::new(token),
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    pub fn repository<'a>(&'a self, owner: &'a str, repo: &'a str) -> GitlabRepoBuilder<'a> {
        GitlabRepoBuilder {
            client: &self.client,
            base_url: &self.base_url,
            owner,
            repo,
        }
    }
}

pub struct GitlabRepoBuilder<'a> {
    client: &'a ProviderClient,
    base_url: &'a str,
    owner: &'a str,
    repo: &'a str,
}

impl<'a> GitlabRepoBuilder<'a> {
    fn project_id(&self) -> String {
        format!("{}%2F{}", self.owner, self.repo)
    }

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
        let url = format!("{}/api/v4/projects/{}", self.base_url, self.project_id());
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

    pub fn webhooks(&self) -> GitlabWebhookBuilder<'a> {
        GitlabWebhookBuilder {
            client: self.client,
            base_url: self.base_url,
            project_id: self.project_id(),
            webhook_url: None,
            events: vec![],
        }
    }
}

pub struct GitlabWebhookBuilder<'a> {
    client: &'a ProviderClient,
    base_url: &'a str,
    project_id: String,
    webhook_url: Option<&'a str>,
    events: Vec<WebhookEvent>,
}

impl<'a> GitlabWebhookBuilder<'a> {
    pub fn create(mut self, url: &'a str) -> Self {
        self.webhook_url = Some(url);
        self
    }

    pub fn events(mut self, events: Vec<WebhookEvent>) -> Self {
        self.events = events;
        self
    }

    pub async fn run(self) -> Result<String, String> {
        let target_url = self
            .webhook_url
            .ok_or_else(|| "Webhook URL is required to create a webhook".to_string())?;

        let url = format!(
            "{}/api/v4/projects/{}/hooks",
            self.base_url, self.project_id
        );

        let mut payload = serde_json::Map::new();
        payload.insert("url".to_string(), serde_json::json!(target_url));
        payload.insert(
            "enable_ssl_verification".to_string(),
            serde_json::json!(false),
        );

        if self.events.is_empty() {
            payload.insert("push_events".to_string(), serde_json::json!(true));
        } else {
            for event in self.events {
                payload.insert(event.as_gitlab_event().to_string(), serde_json::json!(true));
            }
        }

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
