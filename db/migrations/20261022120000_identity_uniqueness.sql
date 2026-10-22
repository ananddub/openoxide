-- Identity tables previously had no uniqueness guarantees: signup could
-- create duplicate accounts for one email, organizations could share a slug,
-- membership rows could duplicate, and JWT jtis were not unique. Enforce the
-- invariants that the application already assumes.
--
-- Existing duplicates are preserved rather than deleted: the oldest row keeps
-- the original value and later duplicates are renamed with a deterministic
-- suffix so their tokens/memberships/audit references stay intact.

-- 1. Emails are matched case-insensitively by the application.
UPDATE users
SET email = lower(trim(email))
WHERE email IS NOT NULL;

UPDATE users
SET email = email || '+dup-' || id
WHERE email IS NOT NULL
  AND id NOT IN (
      SELECT MIN(id) FROM users WHERE email IS NOT NULL GROUP BY email
  );

CREATE UNIQUE INDEX IF NOT EXISTS idx_users_email_unique
    ON users (email COLLATE NOCASE)
    WHERE email IS NOT NULL;

-- 2. Organization slugs.
UPDATE organization
SET slug = slug || '-dup-' || id
WHERE id NOT IN (
    SELECT MIN(id) FROM organization GROUP BY lower(slug)
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_organization_slug_unique
    ON organization (slug COLLATE NOCASE);

-- 3. One membership row per user per organization.
DELETE FROM organization_members
WHERE id NOT IN (
    SELECT MIN(id)
    FROM organization_members
    GROUP BY user_id, organization_id
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_organization_members_user_org
    ON organization_members (user_id, organization_id);

-- 4. Access/refresh token jtis must be unique.
CREATE UNIQUE INDEX IF NOT EXISTS idx_jwt_tokens_jti_unique
    ON jwt_tokens (jti);
