-- Migration: scope ssh_keys, registries, git_providers, servers, domains by organization
-- Created: 2026-09-13T12:00:00Z
--
-- These five resource tables were global (no organization_id), so switching
-- organizations never changed what a member saw. Add the column and backfill
-- it. Single-organization installs keep working via the DEFAULT 1; multi-org
-- rows are corrected from their owning relationship where one exists.
-- STRICT tables accept ADD COLUMN ... NOT NULL DEFAULT.

ALTER TABLE ssh_keys      ADD COLUMN organization_id INTEGER NOT NULL DEFAULT 1;
ALTER TABLE registries    ADD COLUMN organization_id INTEGER NOT NULL DEFAULT 1;
ALTER TABLE git_providers ADD COLUMN organization_id INTEGER NOT NULL DEFAULT 1;
ALTER TABLE servers       ADD COLUMN organization_id INTEGER NOT NULL DEFAULT 1;
ALTER TABLE domains       ADD COLUMN organization_id INTEGER NOT NULL DEFAULT 1;

-- Servers already carry an organization binding in monitoring_agents (the
-- agent registration row); prefer it over the default.
UPDATE servers
SET organization_id = (
    SELECT ma.organization_id
    FROM monitoring_agents ma
    WHERE ma.server_id = servers.id
)
WHERE EXISTS (
    SELECT 1 FROM monitoring_agents ma
    WHERE ma.server_id = servers.id
);

-- Domains hang off a deployable service; walk service -> environment ->
-- project to inherit the project's organization.
UPDATE domains
SET organization_id = (
    SELECT p.organization_id
    FROM services s
    JOIN environments e ON e.id = s.environment_id
    JOIN projects p    ON p.id = e.project_id
    WHERE s.id = domains.service_id
)
WHERE domains.service_id IS NOT NULL
  AND EXISTS (
      SELECT 1
      FROM services s
      JOIN environments e ON e.id = s.environment_id
      JOIN projects p    ON p.id = e.project_id
      WHERE s.id = domains.service_id
  );
