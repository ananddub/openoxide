-- Extracted from src/db/repository/personal_access_tokens.rs. Do not edit repository SQL directly.

-- name: PersonalAccessTokensCreate :one
INSERT INTO personal_access_tokens (user_id, name, token_prefix, token_hash, expires_at)
               VALUES (?, ?, ?, ?, ?)
               RETURNING id AS id, user_id AS user_id, name AS name,
                         token_prefix AS token_prefix, expires_at, last_used_at,
                         revoked_at, created_at AS created_at;

-- name: PersonalAccessTokensList :many
SELECT id AS id, user_id AS user_id, name AS name,
                      token_prefix AS token_prefix, expires_at, last_used_at,
                      revoked_at, created_at AS created_at
               FROM personal_access_tokens WHERE user_id = ? ORDER BY created_at DESC;

-- name: PersonalAccessTokensAuthenticate1 :one
SELECT id AS id, user_id AS user_id, name AS name,
                      token_prefix AS token_prefix, expires_at, last_used_at,
                      revoked_at, created_at AS created_at
               FROM personal_access_tokens
               WHERE token_hash = ? AND revoked_at IS NULL
                 AND (expires_at IS NULL OR expires_at > CURRENT_TIMESTAMP);

-- name: PersonalAccessTokensAuthenticate2 :exec
UPDATE personal_access_tokens SET last_used_at = CURRENT_TIMESTAMP WHERE id = ?;

-- name: PersonalAccessTokensRevoke :exec
UPDATE personal_access_tokens SET revoked_at = CURRENT_TIMESTAMP WHERE id = ? AND user_id = ? AND revoked_at IS NULL;
