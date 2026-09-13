-- Unified git provider credentials (vault-style: provider_type + config_json).
-- GITHUB: {"app_name":..., "app_id":..., "client_id":..., "client_secret":..., "installation_id":..., "private_key":..., "webhook_secret":...}
-- GITLAB: {"url":..., "internal_url":..., "application_id":..., "redirect_uri":..., "secret":..., "access_token":..., "refresh_token":..., "group_name":..., "expires_at":...}
-- GITEA:  {"url":..., "internal_url":..., "redirect_uri":..., "client_id":..., "client_secret":..., "access_token":..., "refresh_token":..., "expires_at":..., "scopes":..., "last_authenticated_at":...}
-- BITBUCKET: {"username":..., "email":..., "app_password":..., "api_token":..., "workspace_name":...}
CREATE TABLE git_providers (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    provider_type TEXT NOT NULL,
    config_json TEXT NOT NULL DEFAULT '{}',
    shared BIGINT NOT NULL,
    webhook_secret TEXT,
    organization_id BIGINT NOT NULL,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL,
    CONSTRAINT git_provider_type_check CHECK (provider_type IN ('GITHUB', 'GITLAB', 'GITEA', 'BITBUCKET')),
    CONSTRAINT git_provider_config_json_check CHECK (json_valid(config_json))
);
