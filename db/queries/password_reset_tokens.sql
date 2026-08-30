-- Extracted from src/db/repository/password_reset_tokens.rs. Do not edit repository SQL directly.

-- name: PasswordResetTokensIssue1 :exec
UPDATE password_reset_tokens SET used_at = CURRENT_TIMESTAMP WHERE user_id = ? AND used_at IS NULL;

-- name: PasswordResetTokensIssue2 :exec
INSERT INTO password_reset_tokens (user_id, token_hash, expires_at) VALUES (?, ?, ?);

-- name: PasswordResetTokensComplete1 :one
UPDATE password_reset_tokens
               SET used_at = CURRENT_TIMESTAMP
               WHERE token_hash = ? AND used_at IS NULL
                 AND expires_at > CURRENT_TIMESTAMP
               RETURNING user_id AS user_id;

-- name: PasswordResetTokensComplete2 :exec
UPDATE users SET password = ? WHERE id = ?;

-- name: PasswordResetTokensComplete3 :exec
UPDATE jwt_tokens SET is_blacklist = 1, blacklist_at = CURRENT_TIMESTAMP WHERE user_id = ? AND is_blacklist = 0;
