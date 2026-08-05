#[derive(Debug)]
pub enum ProviderCredentials {
    Github {
        app_name: Option<String>,
        app_id: Option<i64>,
        client_id: Option<String>,
        client_secret: Option<String>,
        installation_id: Option<String>,
        private_key: Option<String>,
    },
    Gitlab {
        url: String,
        internal_url: Option<String>,
        application_id: Option<String>,
        redirect_uri: Option<String>,
        secret: Option<String>,
        access_token: Option<String>,
        refresh_token: Option<String>,
        group_name: Option<String>,
    },
    Gitea {
        url: String,
        internal_url: Option<String>,
        redirect_uri: Option<String>,
        client_id: Option<String>,
        client_secret: Option<String>,
        access_token: Option<String>,
        refresh_token: Option<String>,
        scopes: Option<String>,
    },
    Bitbucket {
        username: Option<String>,
        email: Option<String>,
        app_password: Option<String>,
        api_token: Option<String>,
        workspace: Option<String>,
    },
}

#[derive(Debug)]
pub struct CreateProvider {
    pub name: String,
    pub shared: bool,
    pub credentials: ProviderCredentials,
}

#[derive(Debug)]
pub struct UpdateProvider {
    pub name: String,
    pub shared: Option<bool>,
    pub credentials: ProviderCredentials,
}

#[derive(Debug, Clone)]
pub struct GitProviderView {
    pub id: i64,
    pub name: String,
    pub provider_type: String,
    pub shared: bool,
    pub configured: bool,
    pub webhook_configured: bool,
    pub created_at: i64,
    pub updated_at: i64,
}
