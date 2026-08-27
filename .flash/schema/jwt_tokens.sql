CREATE TABLE jwt_tokens (
    id INTEGER NOT NULL PRIMARY KEY,
    jti TEXT NOT NULL,
    role TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    is_blacklist INTEGER,
    blacklist_at INTEGER,
    expired_at INTEGER,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    session_id TEXT,
    token_kind TEXT
);
