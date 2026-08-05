use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use reqwest::{Client, RequestBuilder, StatusCode};
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone)]
pub enum ProviderAccess {
    GithubApp {
        app_id: i64,
        installation_id: String,
        private_key: String,
    },
    Gitlab {
        base_url: String,
        token: String,
    },
    Gitea {
        base_url: String,
        token: String,
    },
    Bitbucket {
        username: Option<String>,
        app_password: Option<String>,
        api_token: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct RepositoryInfo {
    pub owner: String,
    pub name: String,
    pub full_name: String,
    pub private: bool,
    pub default_branch: Option<String>,
    pub clone_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct GitReferenceInfo {
    pub name: String,
    pub commit: Option<String>,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct ProviderWebhookInfo {
    pub id: String,
    pub url: String,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object)]
pub struct CollaboratorPermission {
    pub username: String,
    pub level: String,
    pub can_write: bool,
    pub can_admin: bool,
}

pub struct ProviderDiscovery {
    client: Client,
}

impl Default for ProviderDiscovery {
    fn default() -> Self {
        Self {
            client: Client::builder()
                .user_agent("rustploy")
                .build()
                .expect("valid HTTP client"),
        }
    }
}

impl ProviderDiscovery {
    pub async fn test(&self, access: &ProviderAccess) -> Result<(), String> {
        self.repositories(access).await.map(|_| ())
    }

    pub async fn repositories(
        &self,
        access: &ProviderAccess,
    ) -> Result<Vec<RepositoryInfo>, String> {
        match access {
            ProviderAccess::GithubApp { .. } => {
                let token = self.github_installation_token(access).await?;
                let value = self
                    .json(
                        self.client
                            .get("https://api.github.com/installation/repositories")
                            .bearer_auth(token)
                            .query(&[("per_page", "100")]),
                    )
                    .await?;
                parse_repositories(
                    value.get("repositories").and_then(Value::as_array),
                    "github",
                )
            }
            ProviderAccess::Gitlab { base_url, token } => {
                let value = self
                    .json(
                        self.client
                            .get(format!("{}/api/v4/projects", trim(base_url)))
                            .header("PRIVATE-TOKEN", token)
                            .query(&[("membership", "true"), ("per_page", "100")]),
                    )
                    .await?;
                parse_repositories(value.as_array(), "gitlab")
            }
            ProviderAccess::Gitea { base_url, token } => {
                let value = self
                    .json(
                        self.client
                            .get(format!("{}/api/v1/user/repos", trim(base_url)))
                            .bearer_auth(token)
                            .query(&[("limit", "100")]),
                    )
                    .await?;
                parse_repositories(value.as_array(), "gitea")
            }
            ProviderAccess::Bitbucket { .. } => {
                let request = authenticate_bitbucket(
                    self.client
                        .get("https://api.bitbucket.org/2.0/repositories")
                        .query(&[("role", "member"), ("pagelen", "100")]),
                    access,
                );
                let value = self.json(request).await?;
                parse_repositories(value.get("values").and_then(Value::as_array), "bitbucket")
            }
        }
    }

    pub async fn branches(
        &self,
        access: &ProviderAccess,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<GitReferenceInfo>, String> {
        self.references(access, owner, repo, false).await
    }

    pub async fn tags(
        &self,
        access: &ProviderAccess,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<GitReferenceInfo>, String> {
        self.references(access, owner, repo, true).await
    }

    pub async fn collaborator_permission(
        &self,
        access: &ProviderAccess,
        owner: &str,
        repo: &str,
        username: &str,
    ) -> Result<CollaboratorPermission, String> {
        let level = match access {
            ProviderAccess::GithubApp { .. } => {
                let token = self.github_installation_token(access).await?;
                let value = self
                    .json(
                        self.client
                            .get(format!(
                                "https://api.github.com/repos/{owner}/{repo}/collaborators/{username}/permission"
                            ))
                            .bearer_auth(token),
                    )
                    .await?;
                value
                    .get("permission")
                    .and_then(Value::as_str)
                    .unwrap_or("none")
                    .to_owned()
            }
            ProviderAccess::Gitlab { base_url, token } => {
                let project = percent_encode(&format!("{owner}/{repo}"));
                let value = self
                    .json(
                        self.client
                            .get(format!(
                                "{}/api/v4/projects/{project}/members/all",
                                trim(base_url)
                            ))
                            .header("PRIVATE-TOKEN", token)
                            .query(&[("query", username), ("per_page", "100")]),
                    )
                    .await?;
                let access_level = value
                    .as_array()
                    .and_then(|items| {
                        items.iter().find(|item| {
                            item.get("username").and_then(Value::as_str) == Some(username)
                        })
                    })
                    .and_then(|item| item.get("access_level"))
                    .and_then(Value::as_i64)
                    .unwrap_or_default();
                match access_level {
                    50 => "owner",
                    40 => "maintain",
                    30 => "write",
                    20 => "read",
                    10 => "guest",
                    _ => "none",
                }
                .to_owned()
            }
            ProviderAccess::Gitea { base_url, token } => {
                let value = self
                    .json(
                        self.client
                            .get(format!(
                                "{}/api/v1/repos/{owner}/{repo}/collaborators/{username}/permission",
                                trim(base_url)
                            ))
                            .bearer_auth(token),
                    )
                    .await?;
                value
                    .get("permission")
                    .and_then(Value::as_str)
                    .unwrap_or("none")
                    .to_owned()
            }
            ProviderAccess::Bitbucket { .. } => {
                let value = self
                    .json(authenticate_bitbucket(
                        self.client.get(format!(
                            "https://api.bitbucket.org/2.0/repositories/{owner}/{repo}/permissions-config/users/{username}"
                        )),
                        access,
                    ))
                    .await?;
                value
                    .get("permission")
                    .and_then(Value::as_str)
                    .unwrap_or("none")
                    .to_owned()
            }
        };
        let normalized = level.to_ascii_lowercase();
        Ok(CollaboratorPermission {
            username: username.to_owned(),
            can_write: matches!(
                normalized.as_str(),
                "write" | "admin" | "maintain" | "maintainer" | "owner"
            ),
            can_admin: matches!(normalized.as_str(), "admin" | "owner"),
            level,
        })
    }

    pub async fn webhook_status(
        &self,
        access: &ProviderAccess,
        owner: &str,
        repo: &str,
        callback_url: &str,
    ) -> Result<Option<ProviderWebhookInfo>, String> {
        let hooks = self.webhooks(access, owner, repo).await?;
        Ok(hooks.into_iter().find(|hook| hook.url == callback_url))
    }

    pub async fn install_webhook(
        &self,
        access: &ProviderAccess,
        owner: &str,
        repo: &str,
        callback_url: &str,
        secret: &str,
    ) -> Result<ProviderWebhookInfo, String> {
        if let Some(existing) = self
            .webhook_status(access, owner, repo, callback_url)
            .await?
        {
            return Ok(existing);
        }
        let value = match access {
            ProviderAccess::GithubApp { .. } => {
                let token = self.github_installation_token(access).await?;
                self.json(self.client.post(format!("https://api.github.com/repos/{owner}/{repo}/hooks")).bearer_auth(token).json(&serde_json::json!({"name":"web","active":true,"events":["push","pull_request"],"config":{"url":callback_url,"content_type":"json","secret":secret,"insecure_ssl":"0"}}))).await?
            }
            ProviderAccess::Gitlab { base_url, token } => {
                let project = percent_encode(&format!("{owner}/{repo}"));
                self.json(self.client.post(format!("{}/api/v4/projects/{project}/hooks", trim(base_url))).header("PRIVATE-TOKEN", token).json(&serde_json::json!({"url":callback_url,"token":secret,"push_events":true,"tag_push_events":true,"merge_requests_events":true,"enable_ssl_verification":true}))).await?
            }
            ProviderAccess::Gitea { base_url, token } => self.json(self.client.post(format!("{}/api/v1/repos/{owner}/{repo}/hooks", trim(base_url))).bearer_auth(token).json(&serde_json::json!({"type":"gitea","active":true,"events":["push","pull_request"],"config":{"url":callback_url,"content_type":"json","secret":secret}}))).await?,
            ProviderAccess::Bitbucket { .. } => self.json(authenticate_bitbucket(self.client.post(format!("https://api.bitbucket.org/2.0/repositories/{owner}/{repo}/hooks")).json(&serde_json::json!({"description":"rustploy-webhook","url":callback_url,"active":true,"secret":secret,"events":["repo:push","repo:refs_changed","pullrequest:created","pullrequest:updated","pullrequest:fulfilled","pullrequest:rejected"]})), access)).await?,
        };
        parse_webhook(&value).ok_or_else(|| "provider returned an invalid webhook response".into())
    }

    pub async fn remove_webhook(
        &self,
        access: &ProviderAccess,
        owner: &str,
        repo: &str,
        callback_url: &str,
    ) -> Result<bool, String> {
        let Some(hook) = self
            .webhook_status(access, owner, repo, callback_url)
            .await?
        else {
            return Ok(false);
        };
        let request = match access {
            ProviderAccess::GithubApp { .. } => {
                let token = self.github_installation_token(access).await?;
                self.client
                    .delete(format!(
                        "https://api.github.com/repos/{owner}/{repo}/hooks/{}",
                        hook.id
                    ))
                    .bearer_auth(token)
            }
            ProviderAccess::Gitlab { base_url, token } => self
                .client
                .delete(format!(
                    "{}/api/v4/projects/{}/hooks/{}",
                    trim(base_url),
                    percent_encode(&format!("{owner}/{repo}")),
                    hook.id
                ))
                .header("PRIVATE-TOKEN", token),
            ProviderAccess::Gitea { base_url, token } => self
                .client
                .delete(format!(
                    "{}/api/v1/repos/{owner}/{repo}/hooks/{}",
                    trim(base_url),
                    hook.id
                ))
                .bearer_auth(token),
            ProviderAccess::Bitbucket { .. } => authenticate_bitbucket(
                self.client.delete(format!(
                    "https://api.bitbucket.org/2.0/repositories/{owner}/{repo}/hooks/{}",
                    hook.id
                )),
                access,
            ),
        };
        let response = request.send().await.map_err(|error| error.to_string())?;
        if response.status().is_success() || response.status() == StatusCode::NOT_FOUND {
            Ok(true)
        } else {
            Err(provider_error(
                response.status(),
                &response.text().await.unwrap_or_default(),
            ))
        }
    }

    async fn webhooks(
        &self,
        access: &ProviderAccess,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<ProviderWebhookInfo>, String> {
        let value = match access {
            ProviderAccess::GithubApp { .. } => {
                let token = self.github_installation_token(access).await?;
                self.json(
                    self.client
                        .get(format!("https://api.github.com/repos/{owner}/{repo}/hooks"))
                        .bearer_auth(token)
                        .query(&[("per_page", "100")]),
                )
                .await?
            }
            ProviderAccess::Gitlab { base_url, token } => {
                self.json(
                    self.client
                        .get(format!(
                            "{}/api/v4/projects/{}/hooks",
                            trim(base_url),
                            percent_encode(&format!("{owner}/{repo}"))
                        ))
                        .header("PRIVATE-TOKEN", token)
                        .query(&[("per_page", "100")]),
                )
                .await?
            }
            ProviderAccess::Gitea { base_url, token } => {
                self.json(
                    self.client
                        .get(format!(
                            "{}/api/v1/repos/{owner}/{repo}/hooks",
                            trim(base_url)
                        ))
                        .bearer_auth(token)
                        .query(&[("limit", "100")]),
                )
                .await?
            }
            ProviderAccess::Bitbucket { .. } => {
                self.json(authenticate_bitbucket(
                    self.client
                        .get(format!(
                            "https://api.bitbucket.org/2.0/repositories/{owner}/{repo}/hooks"
                        ))
                        .query(&[("pagelen", "100")]),
                    access,
                ))
                .await?
            }
        };
        let items = value
            .get("values")
            .and_then(Value::as_array)
            .or_else(|| value.as_array())
            .ok_or_else(|| "provider returned an invalid webhook list".to_string())?;
        Ok(items.iter().filter_map(parse_webhook).collect())
    }

    async fn references(
        &self,
        access: &ProviderAccess,
        owner: &str,
        repo: &str,
        tags: bool,
    ) -> Result<Vec<GitReferenceInfo>, String> {
        let kind = if tags { "tags" } else { "branches" };
        let value = match access {
            ProviderAccess::GithubApp { .. } => {
                let token = self.github_installation_token(access).await?;
                self.json(
                    self.client
                        .get(format!(
                            "https://api.github.com/repos/{owner}/{repo}/{kind}"
                        ))
                        .bearer_auth(token)
                        .query(&[("per_page", "100")]),
                )
                .await?
            }
            ProviderAccess::Gitlab { base_url, token } => {
                let project = percent_encode(&format!("{owner}/{repo}"));
                self.json(
                    self.client
                        .get(format!(
                            "{}/api/v4/projects/{project}/repository/{kind}",
                            trim(base_url)
                        ))
                        .header("PRIVATE-TOKEN", token)
                        .query(&[("per_page", "100")]),
                )
                .await?
            }
            ProviderAccess::Gitea { base_url, token } => {
                self.json(
                    self.client
                        .get(format!(
                            "{}/api/v1/repos/{owner}/{repo}/{kind}",
                            trim(base_url)
                        ))
                        .bearer_auth(token)
                        .query(&[("limit", "100")]),
                )
                .await?
            }
            ProviderAccess::Bitbucket { .. } => {
                self.json(authenticate_bitbucket(
                    self.client
                        .get(format!(
                            "https://api.bitbucket.org/2.0/repositories/{owner}/{repo}/refs/{kind}"
                        ))
                        .query(&[("pagelen", "100")]),
                    access,
                ))
                .await?
            }
        };
        let items = value
            .get("values")
            .and_then(Value::as_array)
            .or_else(|| value.as_array())
            .ok_or_else(|| "provider returned an invalid references response".to_string())?;
        Ok(items
            .iter()
            .filter_map(|item| {
                let name = item.get("name")?.as_str()?.to_owned();
                let commit = item
                    .pointer("/commit/id")
                    .or_else(|| item.pointer("/target/hash"))
                    .or_else(|| item.pointer("/commit/sha"))
                    .and_then(Value::as_str)
                    .map(str::to_owned);
                Some(GitReferenceInfo { name, commit })
            })
            .collect())
    }

    async fn github_installation_token(&self, access: &ProviderAccess) -> Result<String, String> {
        let ProviderAccess::GithubApp {
            app_id,
            installation_id,
            private_key,
        } = access
        else {
            return Err("GitHub App access required".into());
        };
        #[derive(Serialize)]
        struct Claims {
            iat: i64,
            exp: i64,
            iss: String,
        }
        let now = chrono::Utc::now().timestamp();
        let jwt = encode(
            &Header::new(Algorithm::RS256),
            &Claims {
                iat: now - 60,
                exp: now + 540,
                iss: app_id.to_string(),
            },
            &EncodingKey::from_rsa_pem(private_key.as_bytes())
                .map_err(|error| format!("invalid GitHub private key: {error}"))?,
        )
        .map_err(|error| error.to_string())?;
        let value = self
            .json(
                self.client
                    .post(format!(
                        "https://api.github.com/app/installations/{installation_id}/access_tokens"
                    ))
                    .bearer_auth(jwt)
                    .header("Accept", "application/vnd.github+json"),
            )
            .await?;
        value
            .get("token")
            .and_then(Value::as_str)
            .map(str::to_owned)
            .ok_or_else(|| "GitHub did not return an installation token".into())
    }

    async fn json(&self, request: RequestBuilder) -> Result<Value, String> {
        let response = request.send().await.map_err(|error| error.to_string())?;
        let status = response.status();
        let body = response.text().await.map_err(|error| error.to_string())?;
        if !status.is_success() {
            return Err(provider_error(status, &body));
        }
        serde_json::from_str(&body).map_err(|error| format!("invalid provider response: {error}"))
    }
}

fn authenticate_bitbucket(request: RequestBuilder, access: &ProviderAccess) -> RequestBuilder {
    let ProviderAccess::Bitbucket {
        username,
        app_password,
        api_token,
    } = access
    else {
        return request;
    };
    if let Some(token) = api_token {
        request.bearer_auth(token)
    } else {
        request.basic_auth(
            username.as_deref().unwrap_or_default(),
            app_password.as_deref(),
        )
    }
}

fn parse_repositories(
    items: Option<&Vec<Value>>,
    provider: &str,
) -> Result<Vec<RepositoryInfo>, String> {
    let items =
        items.ok_or_else(|| format!("{provider} returned an invalid repository response"))?;
    Ok(items
        .iter()
        .filter_map(|item| {
            let full_name = item
                .get("full_name")
                .or_else(|| item.get("path_with_namespace"))?
                .as_str()?
                .to_owned();
            let (owner, name) = full_name
                .rsplit_once('/')
                .map(|(owner, name)| (owner.to_owned(), name.to_owned()))?;
            Some(RepositoryInfo {
                owner,
                name,
                full_name,
                private: item
                    .get("private")
                    .or_else(|| item.get("visibility"))
                    .map(|value| {
                        value
                            .as_bool()
                            .unwrap_or_else(|| value.as_str() == Some("private"))
                    })
                    .unwrap_or(false),
                default_branch: item
                    .get("default_branch")
                    .and_then(Value::as_str)
                    .map(str::to_owned),
                clone_url: item
                    .get("clone_url")
                    .or_else(|| item.get("http_url_to_repo"))
                    .or_else(|| item.pointer("/links/clone/0/href"))
                    .and_then(Value::as_str)
                    .map(str::to_owned),
            })
        })
        .collect())
}

fn parse_webhook(value: &Value) -> Option<ProviderWebhookInfo> {
    let id = value
        .get("id")
        .or_else(|| value.get("uuid"))?
        .as_str()
        .map(str::to_owned)
        .or_else(|| value.get("id")?.as_i64().map(|id| id.to_string()))?;
    let url = value
        .pointer("/config/url")
        .or_else(|| value.get("url"))?
        .as_str()?
        .to_owned();
    let active = value.get("active").and_then(Value::as_bool).unwrap_or(true);
    Some(ProviderWebhookInfo { id, url, active })
}

fn trim(value: &str) -> &str {
    value.trim_end_matches('/')
}
fn percent_encode(value: &str) -> String {
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
fn provider_error(status: StatusCode, body: &str) -> String {
    format!(
        "provider request failed ({status}): {}",
        body.chars().take(500).collect::<String>()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_numeric_and_string_webhook_ids() {
        let github =
            serde_json::json!({"id": 12, "active": true, "config": {"url": "https://panel/hook"}});
        let bitbucket =
            serde_json::json!({"uuid": "{abc}", "active": false, "url": "https://panel/hook"});
        assert_eq!(parse_webhook(&github).unwrap().id, "12");
        assert_eq!(parse_webhook(&bitbucket).unwrap().id, "{abc}");
        assert!(!parse_webhook(&bitbucket).unwrap().active);
    }

    #[test]
    fn parses_nested_repository_namespaces() {
        let values = vec![serde_json::json!({
            "path_with_namespace": "acme/platform/api",
            "visibility": "private",
            "default_branch": "main",
            "http_url_to_repo": "https://gitlab/acme/platform/api.git"
        })];
        let repositories = parse_repositories(Some(&values), "gitlab").unwrap();
        assert_eq!(repositories[0].owner, "acme/platform");
        assert_eq!(repositories[0].name, "api");
        assert!(repositories[0].private);
    }

    #[test]
    fn percent_encodes_provider_project_paths() {
        assert_eq!(percent_encode("acme/platform api"), "acme%2Fplatform%20api");
    }
}
