use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct OAuthConfig {
    pub base_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub scopes: String,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct AuthorizationInfo {
    pub url: String,
    pub state: String,
}

#[derive(Debug, Clone)]
pub struct OAuthTokens {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<i64>,
    pub scopes: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: Option<String>,
    expires_in: Option<i64>,
    scope: Option<String>,
}

#[derive(Default)]
pub struct OAuthClient {
    client: Client,
}

impl OAuthClient {
    pub fn authorization_url(&self, config: &OAuthConfig, state: &str) -> AuthorizationInfo {
        let url = format!(
            "{}/oauth/authorize?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
            config.base_url.trim_end_matches('/'),
            encode(&config.client_id),
            encode(&config.redirect_uri),
            encode(&config.scopes),
            encode(state),
        );
        AuthorizationInfo {
            url,
            state: state.to_owned(),
        }
    }

    pub async fn exchange_code(
        &self,
        config: &OAuthConfig,
        code: &str,
    ) -> Result<OAuthTokens, String> {
        let response = self
            .client
            .post(format!(
                "{}/oauth/token",
                config.base_url.trim_end_matches('/')
            ))
            .form(&[
                ("client_id", config.client_id.as_str()),
                ("client_secret", config.client_secret.as_str()),
                ("code", code),
                ("grant_type", "authorization_code"),
                ("redirect_uri", config.redirect_uri.as_str()),
            ])
            .send()
            .await
            .map_err(|error| error.to_string())?;
        let status = response.status();
        let body = response.text().await.map_err(|error| error.to_string())?;
        if !status.is_success() {
            return Err(format!(
                "OAuth token exchange failed ({status}): {}",
                body.chars().take(500).collect::<String>()
            ));
        }
        let value: TokenResponse = serde_json::from_str(&body)
            .map_err(|error| format!("invalid OAuth token response: {error}"))?;
        Ok(OAuthTokens {
            access_token: value.access_token,
            refresh_token: value.refresh_token,
            expires_at: value
                .expires_in
                .map(|seconds| chrono::Utc::now().timestamp() + seconds),
            scopes: value.scope,
        })
    }
}

pub fn encode(value: &str) -> String {
    value
        .bytes()
        .map(|byte| {
            if byte.is_ascii_alphanumeric() || b"-_.~".contains(&byte) {
                (byte as char).to_string()
            } else {
                format!("%{byte:02X}")
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authorization_url_encodes_all_user_values() {
        let config = OAuthConfig {
            base_url: "https://git.example.com/".into(),
            client_id: "client id".into(),
            client_secret: "secret".into(),
            redirect_uri: "https://panel/callback?source=git".into(),
            scopes: "read_user api".into(),
        };
        let info = OAuthClient::default().authorization_url(&config, "signed.state");
        assert!(
            info.url
                .starts_with("https://git.example.com/oauth/authorize?")
        );
        assert!(info.url.contains("client_id=client%20id"));
        assert!(info.url.contains("state=signed.state"));
    }
}
