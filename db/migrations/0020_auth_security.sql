CREATE UNIQUE INDEX idx_two_factor_user_id ON two_factor(user_id);

CREATE TABLE password_reset_tokens (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash TEXT NOT NULL UNIQUE,
    expires_at INTEGER NOT NULL,
    used_at INTEGER,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
) STRICT;

CREATE INDEX idx_password_reset_tokens_user
    ON password_reset_tokens(user_id, expires_at);
