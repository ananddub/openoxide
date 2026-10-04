-- DNS provider credentials. provider_type is the wire code of dns::DnsProviderKind and
-- credentials_json is that provider's credential document (keys per DnsProviderKind::
-- credential_fields), with secrets masked on the way out to the panel.
-- CLOUDFLARE / HOSTINGER / INFOMANIAK: {"apiToken":...}
-- ROUTE53:  {"accessKeyId":..., "secretAccessKey":...}
-- PORKBUN:  {"apiKey":..., "secretApiKey":...}
-- SPACESHIP:{"apiKey":..., "apiSecret":...}
-- OVH:      {"endpoint":..., "applicationKey":..., "applicationSecret":..., "consumerKey":...}
CREATE TABLE dns_providers (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    provider_type TEXT NOT NULL,
    credentials_json TEXT NOT NULL,
    organization_id BIGINT NOT NULL,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL,
    CONSTRAINT dns_provider_type_check CHECK (provider_type IN ('CLOUDFLARE', 'ROUTE53', 'HOSTINGER', 'PORKBUN', 'INFOMANIAK', 'OVH', 'SPACESHIP')),
    CONSTRAINT dns_provider_json_check CHECK (json_valid(credentials_json)),
    CONSTRAINT dns_provider_organization_name_unique UNIQUE (organization_id, name)
);

CREATE INDEX idx_dns_providers_organization_id ON dns_providers(organization_id);
