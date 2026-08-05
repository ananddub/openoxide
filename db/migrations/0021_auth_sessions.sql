ALTER TABLE jwt_tokens ADD COLUMN session_id TEXT;
ALTER TABLE jwt_tokens ADD COLUMN token_kind TEXT;

UPDATE jwt_tokens
SET session_id = jti,
    token_kind = 'LEGACY'
WHERE session_id IS NULL;

CREATE INDEX idx_jwt_tokens_user_session
    ON jwt_tokens(user_id, session_id, is_blacklist);

CREATE INDEX idx_password_reset_tokens_active
    ON password_reset_tokens(token_hash, expires_at, used_at);
