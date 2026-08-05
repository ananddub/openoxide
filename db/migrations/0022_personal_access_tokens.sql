CREATE TABLE personal_access_tokens (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    token_prefix TEXT NOT NULL,
    token_hash TEXT NOT NULL UNIQUE,
    expires_at INTEGER,
    last_used_at INTEGER,
    revoked_at INTEGER,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
) STRICT;

CREATE INDEX idx_personal_access_tokens_user
    ON personal_access_tokens(user_id, revoked_at, expires_at);
