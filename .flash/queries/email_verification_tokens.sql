-- Extracted from src/db/repository/email_verification_tokens.rs. Do not edit repository SQL directly.

-- name: EmailVerificationTokensIssue1 :exec
UPDATE email_verification_tokens SET used_at = CURRENT_TIMESTAMP WHERE user_id = ? AND used_at IS NULL;

-- name: EmailVerificationTokensIssue2 :exec
INSERT INTO email_verification_tokens (user_id, token_hash, expires_at) VALUES (?, ?, ?);

-- name: EmailVerificationTokensComplete1 :one
UPDATE email_verification_tokens
               SET used_at = CURRENT_TIMESTAMP
               WHERE token_hash = ? AND used_at IS NULL
                 AND expires_at > CURRENT_TIMESTAMP
               RETURNING user_id AS user_id;

-- name: EmailVerificationTokensComplete2 :exec
UPDATE users SET is_email_verify = 1, email_verify_at = CURRENT_TIMESTAMP WHERE id = ?;
