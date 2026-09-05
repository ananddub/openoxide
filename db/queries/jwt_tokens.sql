-- Extracted from src/db/repository/jwt_tokens.rs. Do not edit repository SQL directly.

-- name: JwtTokensGetAll :many
SELECT id AS id, jti AS jti, role AS role, user_id AS user_id, is_blacklist AS is_blacklist, blacklist_at AS blacklist_at, expired_at AS expired_at, created_at AS created_at, updated_at AS updated_at FROM jwt_tokens;

-- name: JwtTokensGetById :one
SELECT id AS id, jti AS jti, role AS role, user_id AS user_id, is_blacklist AS is_blacklist, blacklist_at AS blacklist_at, expired_at AS expired_at, created_at AS created_at, updated_at AS updated_at FROM jwt_tokens WHERE id = ?;

-- name: JwtTokensCreate :exec
INSERT INTO jwt_tokens (jti, role, user_id, is_blacklist, blacklist_at, expired_at, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?);

-- name: JwtTokensUpdate :exec
UPDATE jwt_tokens SET jti = ?, role = ?, user_id = ?, is_blacklist = ?, blacklist_at = ?, expired_at = ?, created_at = ?, updated_at = ? WHERE id = ?;

-- name: JwtTokensDelete :exec
DELETE FROM jwt_tokens WHERE id = ?;

-- name: JwtTokensBlacklistByJti :exec
UPDATE jwt_tokens SET is_blacklist = 1, blacklist_at = CURRENT_TIMESTAMP WHERE jti = ?;

-- name: JwtTokensBlacklistAllByUser :exec
UPDATE jwt_tokens SET is_blacklist = 1, blacklist_at = CURRENT_TIMESTAMP WHERE user_id = ? AND is_blacklist = 0;

-- name: JwtTokensInsertToken :exec
INSERT INTO jwt_tokens (jti, role, user_id, is_blacklist, expired_at, session_id, token_kind) VALUES (?, ?, ?, 0, ?, ?, ?);

-- name: JwtTokensSessionIdForJti :one
SELECT session_id FROM jwt_tokens WHERE jti = ?;

-- name: JwtTokensListSessions :many
SELECT session_id AS session_id,
                      MIN(created_at) AS created_at,
                      MAX(expired_at) AS expires_at,
                      MAX(CASE WHEN is_blacklist = 0 AND expired_at > CURRENT_TIMESTAMP THEN 1 ELSE 0 END) != 0 AS active
               FROM jwt_tokens
               WHERE user_id = ? AND session_id IS NOT NULL
               GROUP BY session_id
               ORDER BY created_at DESC
               LIMIT 100;

-- name: JwtTokensRevokeSession :exec
UPDATE jwt_tokens SET is_blacklist = 1, blacklist_at = CURRENT_TIMESTAMP WHERE user_id = ? AND session_id = ? AND is_blacklist = 0;

-- name: JwtTokensIsTokenActive :one
SELECT EXISTS(SELECT 1 FROM jwt_tokens WHERE jti = ? AND COALESCE(is_blacklist, 0) = 0 AND expired_at > CURRENT_TIMESTAMP);
