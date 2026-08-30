CREATE TABLE personal_access_tokens (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    user_id BIGINT NOT NULL,
    name TEXT NOT NULL,
    token_prefix TEXT NOT NULL,
    token_hash TEXT NOT NULL,
    expires_at BIGINT,
    last_used_at BIGINT,
    revoked_at BIGINT,
    created_at BIGINT NOT NULL
);
