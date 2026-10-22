-- TwoFactorUpsert relies on ON CONFLICT(user_id) and the service expects at
-- most one row per user, but two_factor never declared user_id unique, so
-- SetupTwoFactor failed with "ON CONFLICT clause does not match any PRIMARY
-- KEY or UNIQUE constraint". Keep the newest row per user before enforcing it.

DELETE FROM two_factor
WHERE id NOT IN (
    SELECT MAX(id) FROM two_factor GROUP BY user_id
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_two_factor_user_id
    ON two_factor (user_id);
