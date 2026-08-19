use crate::string_enum;

string_enum! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
    pub enum SourceType {
        default = Docker;

        Docker => "DOCKER",
        Git => "GIT",
        Github => "GITHUB",
        Gitlab => "GITLAB",
        Bitbucket => "BITBUCKET",
        Gitea => "GITEA",
        Raw => "RAW",
        Drop => "DROP",
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AdapterError {
    #[error("Missing field: {0}")]
    MissingField(&'static str),
    #[error("Invalid field {field}: {message}")]
    InvalidField {
        field: &'static str,
        message: String,
    },
    #[error("Unsupported source type: {0}")]
    UnsupportedSourceType(String),
}

pub enum GitProvider {
    Github { owner: String, repo: String },
    Gitlab { owner: String, repo: String },
    Bitbucket { owner: String, repo: String },
    Gitea { url: String },
    Custom { url: String },
}

impl GitProvider {
    pub fn repository_url(&self) -> String {
        match self {
            Self::Github { owner, repo } => format!("https://github.com/{owner}/{repo}.git"),
            Self::Gitlab { owner, repo } => format!("https://gitlab.com/{owner}/{repo}.git"),
            Self::Bitbucket { owner, repo } => format!("https://bitbucket.org/{owner}/{repo}.git"),
            Self::Gitea { url } | Self::Custom { url } => url.clone(),
        }
    }
}

pub struct GitProviderBuilder<'a> {
    pub source_type: SourceType,
    pub repository: Option<&'a str>,
    pub owner: Option<&'a str>,
    pub gitlab_repository: Option<&'a str>,
    pub gitlab_owner: Option<&'a str>,
    pub gitea_repository: Option<&'a str>,
    pub bitbucket_repository: Option<&'a str>,
    pub bitbucket_owner: Option<&'a str>,
    pub custom_git_url: Option<&'a str>,
}

impl<'a> GitProviderBuilder<'a> {
    pub fn new(source_type: impl Into<SourceType>) -> Self {
        Self {
            source_type: source_type.into(),
            repository: None,
            owner: None,
            gitlab_repository: None,
            gitlab_owner: None,
            gitea_repository: None,
            bitbucket_repository: None,
            bitbucket_owner: None,
            custom_git_url: None,
        }
    }

    pub fn github(mut self, owner: Option<&'a str>, repository: Option<&'a str>) -> Self {
        self.owner = owner;
        self.repository = repository;
        self
    }

    pub fn gitlab(mut self, owner: Option<&'a str>, repository: Option<&'a str>) -> Self {
        self.gitlab_owner = owner;
        self.gitlab_repository = repository;
        self
    }

    pub fn bitbucket(mut self, owner: Option<&'a str>, repository: Option<&'a str>) -> Self {
        self.bitbucket_owner = owner;
        self.bitbucket_repository = repository;
        self
    }

    pub fn gitea(mut self, url: Option<&'a str>) -> Self {
        self.gitea_repository = url;
        self
    }

    pub fn custom(mut self, url: Option<&'a str>) -> Self {
        self.custom_git_url = url;
        self
    }

    pub fn build(self) -> Result<GitProvider, AdapterError> {
        match self.source_type {
            SourceType::Github => {
                let repo_raw = self.repository.unwrap_or("").trim();
                let owner_raw = self.owner.unwrap_or("").trim();
                let (parsed_owner, parsed_repo) = parse_owner_repo(owner_raw, repo_raw);
                if parsed_repo.contains("://") || parsed_repo.starts_with("git@") {
                    Ok(GitProvider::Custom { url: parsed_repo })
                } else {
                    Ok(GitProvider::Github {
                        owner: if parsed_owner.is_empty() {
                            "unknown".into()
                        } else {
                            parsed_owner
                        },
                        repo: if parsed_repo.is_empty() {
                            "unknown".into()
                        } else {
                            parsed_repo
                        },
                    })
                }
            }
            SourceType::Gitlab => {
                let repo_raw = self.gitlab_repository.unwrap_or("").trim();
                let owner_raw = self.gitlab_owner.unwrap_or("").trim();
                let (parsed_owner, parsed_repo) = parse_owner_repo(owner_raw, repo_raw);
                if parsed_repo.contains("://") || parsed_repo.starts_with("git@") {
                    Ok(GitProvider::Custom { url: parsed_repo })
                } else {
                    Ok(GitProvider::Gitlab {
                        owner: if parsed_owner.is_empty() {
                            "unknown".into()
                        } else {
                            parsed_owner
                        },
                        repo: if parsed_repo.is_empty() {
                            "unknown".into()
                        } else {
                            parsed_repo
                        },
                    })
                }
            }
            SourceType::Bitbucket => {
                let repo_raw = self.bitbucket_repository.unwrap_or("").trim();
                let owner_raw = self.bitbucket_owner.unwrap_or("").trim();
                let (parsed_owner, parsed_repo) = parse_owner_repo(owner_raw, repo_raw);
                if parsed_repo.contains("://") || parsed_repo.starts_with("git@") {
                    Ok(GitProvider::Custom { url: parsed_repo })
                } else {
                    Ok(GitProvider::Bitbucket {
                        owner: if parsed_owner.is_empty() {
                            "unknown".into()
                        } else {
                            parsed_owner
                        },
                        repo: if parsed_repo.is_empty() {
                            "unknown".into()
                        } else {
                            parsed_repo
                        },
                    })
                }
            }
            SourceType::Gitea => {
                let url = self
                    .gitea_repository
                    .filter(|value| value.contains("://") || value.starts_with("git@"))
                    .ok_or_else(|| AdapterError::InvalidField {
                        field: "gitea_repository",
                        message: "Gitea repository must be a full URL".into(),
                    })?;
                Ok(GitProvider::Gitea { url: url.into() })
            }
            SourceType::Git => {
                let url = self
                    .custom_git_url
                    .ok_or(AdapterError::MissingField("custom_git_url"))?;
                Ok(GitProvider::Custom { url: url.into() })
            }
            other => Err(AdapterError::UnsupportedSourceType(format!("{other:?}"))),
        }
    }
}

fn parse_owner_repo(owner_input: &str, repo_input: &str) -> (String, String) {
    if !owner_input.is_empty() && !repo_input.is_empty() {
        return (owner_input.to_string(), repo_input.to_string());
    }
    if repo_input.contains('/') && !repo_input.contains("://") && !repo_input.starts_with("git@") {
        let parts: Vec<&str> = repo_input.splitn(2, '/').collect();
        return (parts[0].to_string(), parts[1].to_string());
    }
    (owner_input.to_string(), repo_input.to_string())
}
