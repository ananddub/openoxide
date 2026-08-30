CREATE TABLE password_reset_tokens (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id BIGINT NOT NULL,
    token_hash TEXT NOT NULL,
    expires_at BIGINT NOT NULL,
    used_at BIGINT,
    created_at BIGINT NOT NULL
);
