#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceType {
    Github,
    Gitlab,
    Bitbucket,
    Gitea,
    Git,
    Docker,
    DockerImage,
}

#[derive(Debug, thiserror::Error)]
pub enum AdapterError {
    #[error("Missing field: {0}")]
    MissingField(&'static str),
    #[error("Invalid field {field}: {message}")]
    InvalidField { field: &'static str, message: String },
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
    pub fn new(source_type: SourceType) -> Self {
        Self {
            source_type,
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
            SourceType::Github => Ok(GitProvider::Github {
                owner: self
                    .owner
                    .ok_or(AdapterError::MissingField("owner"))?
                    .into(),
                repo: self
                    .repository
                    .ok_or(AdapterError::MissingField("repository"))?
                    .into(),
            }),
            SourceType::Gitlab => {
                let url = self
                    .gitlab_repository
                    .ok_or(AdapterError::MissingField("gitlab_repository"))?;
                if url.contains("://") || url.starts_with("git@") {
                    Ok(GitProvider::Custom { url: url.into() })
                } else {
                    Ok(GitProvider::Gitlab {
                        owner: self
                            .gitlab_owner
                            .ok_or(AdapterError::MissingField("gitlab_owner"))?
                            .into(),
                        repo: url.into(),
                    })
                }
            }
            SourceType::Bitbucket => {
                let url = self
                    .bitbucket_repository
                    .ok_or(AdapterError::MissingField("bitbucket_repository"))?;
                if url.contains("://") || url.starts_with("git@") {
                    Ok(GitProvider::Custom { url: url.into() })
                } else {
                    Ok(GitProvider::Bitbucket {
                        owner: self
                            .bitbucket_owner
                            .ok_or(AdapterError::MissingField("bitbucket_owner"))?
                            .into(),
                        repo: url.into(),
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
