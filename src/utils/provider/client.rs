use reqwest::{Client, RequestBuilder};

#[derive(Clone)]
pub struct ProviderClient {
    pub(crate) client: Client,
    pub(crate) token: Option<String>,
}

impl ProviderClient {
    pub fn new(token: Option<String>) -> Self {
        Self {
            client: Client::builder()
                .user_agent("openoxide-builder")
                .build()
                .unwrap_or_default(),
            token,
        }
    }

    pub fn authenticate(&self, mut req: RequestBuilder) -> RequestBuilder {
        if let Some(token) = &self.token {
            if token.contains(':') {
                let parts: Vec<&str> = token.splitn(2, ':').collect();
                req = req.basic_auth(parts[0], Some(parts[1]));
            } else {
                req = req.bearer_auth(token);
            }
        }
        req
    }
}
