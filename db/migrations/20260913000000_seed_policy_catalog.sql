-- Migration: seed the permission policy catalog
-- Created: 2026-09-13T00:00:00Z
--
-- The `policy` table shipped empty, so group editors had no actions to
-- grant and `seed_organization_defaults` attached nothing to the
-- "Organization Admin" group. These are exactly the `resource:operation`
-- pairs enforced by #[permission(...)] attributes in src/api/handlers.
-- Idempotent: only actions missing from the table are inserted.
WITH seed(candidate) AS (VALUES
    ('alert:monitor'),
    ('alert:write'),
    ('app:create'),
    ('app:delete'),
    ('app:deploy'),
    ('app:read'),
    ('app:update'),
    ('audit_log:read'),
    ('certificate:create'),
    ('certificate:delete'),
    ('certificate:read'),
    ('certificate:update'),
    ('database:create'),
    ('database:delete'),
    ('database:deploy'),
    ('database:read'),
    ('database:update'),
    ('deployment:delete'),
    ('deployment:deploy'),
    ('deployment:read'),
    ('env:read'),
    ('env:write'),
    ('group:create'),
    ('group:delete'),
    ('group:read'),
    ('group:update'),
    ('invitation:create'),
    ('invitation:delete'),
    ('invitation:read'),
    ('logs:read'),
    ('member:create'),
    ('member:delete'),
    ('member:read'),
    ('member:update'),
    ('monitoring:monitor'),
    ('notification:create'),
    ('notification:delete'),
    ('notification:read'),
    ('org:read'),
    ('org:write'),
    ('project:create'),
    ('project:delete'),
    ('project:read'),
    ('project:update'),
    ('registry:create'),
    ('registry:delete'),
    ('registry:read'),
    ('schedule:create'),
    ('schedule:delete'),
    ('schedule:deploy'),
    ('schedule:read'),
    ('server:create'),
    ('server:delete'),
    ('server:read'),
    ('server:update'),
    ('traefik:read'),
    ('traefik:write'))
INSERT INTO policy (action)
SELECT candidate FROM seed
WHERE NOT EXISTS (SELECT 1 FROM policy p WHERE p.action = seed.candidate);

-- Backfill: organizations created while the catalog was empty got an
-- "Organization Admin" group with zero policies. Attach the same CRUD
-- suffix set seed_organization_defaults uses. group_policy has no unique
-- index, so dedupe explicitly to stay idempotent.
INSERT INTO group_policy (group_id, policy_id)
SELECT g.id, p.id
FROM groups g
JOIN policy p ON p.action LIKE '%:read'
    OR p.action LIKE '%:create'
    OR p.action LIKE '%:update'
    OR p.action LIKE '%:delete'
WHERE g.name = 'Organization Admin'
  AND NOT EXISTS (
      SELECT 1 FROM group_policy gp
      WHERE gp.group_id = g.id AND gp.policy_id = p.id
  );
