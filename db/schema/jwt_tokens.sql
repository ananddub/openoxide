CREATE TABLE jwt_tokens (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    jti TEXT NOT NULL UNIQUE,
    role TEXT NOT NULL,
    user_id BIGINT NOT NULL,
    is_blacklist BIGINT,
    blacklist_at BIGINT,
    expired_at BIGINT,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL,
    session_id TEXT,
    token_kind TEXT
);
