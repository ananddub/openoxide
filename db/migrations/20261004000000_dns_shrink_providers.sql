-- Migration: shrink the DNS provider catalog to the seven providers crates/dns implements
-- Created: 2026-10-04T00:00:00Z
--
-- Every other provider_type was only ever served by the vendored dns-update bridge, which
-- this build no longer carries: the row would load, then fail at first use with "no client
-- for this provider". Deleting them here beats leaving an unusable credential behind.
-- The table is then rebuilt with the CHECK that pins provider_type to the supported set and
-- the UNIQUE that stops two providers sharing a name inside one organization (SQLite has no
-- ALTER ADD CONSTRAINT, so the only way in is create-copy-drop-rename).

-- 1. Rows whose type no client can serve go first, so the surviving set satisfies the CHECK.
DELETE FROM dns_providers
WHERE TRIM(UPPER(provider_type)) NOT IN
    ('CLOUDFLARE', 'ROUTE53', 'HOSTINGER', 'PORKBUN', 'INFOMANIAK', 'OVH', 'SPACESHIP');

-- 2. Canonicalize the survivors: the CHECK compares exactly, the wire code is upper-case.
UPDATE dns_providers SET provider_type = TRIM(UPPER(provider_type));

-- 3. The UNIQUE needs room. Keep both rows and label the older one, rather than dropping a
--    credential someone configured.
UPDATE dns_providers
SET name = name || ' (#' || id || ')'
WHERE id IN (
    SELECT d.id
    FROM dns_providers d
    WHERE EXISTS (
        SELECT 1 FROM dns_providers newer
        WHERE newer.organization_id = d.organization_id
          AND TRIM(LOWER(newer.name)) = TRIM(LOWER(d.name))
          AND newer.id > d.id
    )
);

-- 4. Rebuild with the constraints. The column types and STRICT match the table being
--    replaced — a rebuild that quietly drops STRICT would start coercing values.
CREATE TABLE dns_providers_new (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    provider_type TEXT NOT NULL,
    credentials_json TEXT NOT NULL,
    organization_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch()),
    CONSTRAINT dns_provider_type_check CHECK (provider_type IN ('CLOUDFLARE', 'ROUTE53', 'HOSTINGER', 'PORKBUN', 'INFOMANIAK', 'OVH', 'SPACESHIP')),
    CONSTRAINT dns_provider_json_check CHECK (json_valid(credentials_json)),
    CONSTRAINT dns_provider_organization_name_unique UNIQUE (organization_id, name)
) STRICT;

INSERT INTO dns_providers_new
    (id, name, provider_type, credentials_json, organization_id, created_at, updated_at)
SELECT id, name, provider_type, credentials_json, organization_id, created_at, updated_at
FROM dns_providers
ORDER BY id;

DROP TABLE dns_providers;

ALTER TABLE dns_providers_new RENAME TO dns_providers;

CREATE INDEX idx_dns_providers_organization_id ON dns_providers(organization_id);
