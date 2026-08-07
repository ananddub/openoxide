CREATE TABLE users (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	email TEXT UNIQUE,
	last_name TEXT,
	first_name TEXT,
	avatar TEXT NOT NULL,
	-- User role: OWNER | ADMIN | MEMBER
	role TEXT DEFAULT 'OWNER',
	is_owner INTEGER DEFAULT 0,
	about_me TEXT,
	password TEXT NOT NULL,
	is_email_verify INTEGER DEFAULT 0,
	email_verify_at INTEGER,
	two_factor_enable INTEGER DEFAULT 0,
	is_registered INTEGER DEFAULT 0 NOT NULL,
	added_by INTEGER DEFAULT NULL REFERENCES users(id),
	group_id INTEGER NOT NULL REFERENCES groups(id),
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	CONSTRAINT role_check CHECK (role IN ('OWNER', 'ADMIN', 'MEMBER'))
) STRICT;

CREATE TABLE user_policy (
	id         INTEGER PRIMARY KEY AUTOINCREMENT,
	user_id    INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	org_id     INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
	policy_id  INTEGER NOT NULL REFERENCES policy(id) ON DELETE CASCADE,
	-- grant = extra permission add, deny = permission remove
	effect     TEXT NOT NULL DEFAULT 'GRANT',
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	CONSTRAINT effect_check CHECK (effect IN ('GRANT', 'DENY')),
	UNIQUE(user_id, org_id, policy_id)
) STRICT;

CREATE TABLE resource_access (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
	org_id  INTEGER REFERENCES organization(id) ON DELETE CASCADE,
	resource_type TEXT,  --  "project", "server", "environment"
	resource_id   INTEGER,
	created_at INTEGER DEFAULT (strftime('%s', 'now')),
	CONSTRAINT resource_type_check CHECK (
		resource_type IN ('PROJECT', 'SERVER', 'ENVIRONMENT', 'SERVICE', 'GIT_PROVIDER')
	)
) STRICT;

CREATE TABLE two_factor (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	secret TEXT NOT NULL,
	backup_codes TEXT NOT NULL,
	user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE
) STRICT;

CREATE UNIQUE INDEX idx_two_factor_user_id ON two_factor(user_id);

CREATE TABLE jwt_tokens (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	jti TEXT NOT NULL,
	-- Role at time of token issuance: OWNER | ADMIN | MEMBER
	role TEXT NOT NULL,
	user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	session_id TEXT,
	token_kind TEXT,
	is_blacklist INTEGER DEFAULT 0,
	blacklist_at INTEGER,
	expired_at INTEGER,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	CONSTRAINT role_check CHECK (role IN ('OWNER', 'ADMIN', 'MEMBER'))
) STRICT;

CREATE INDEX idx_jwt_tokens_user_session ON jwt_tokens(user_id, session_id, is_blacklist);

CREATE TABLE password_reset_tokens (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash TEXT NOT NULL UNIQUE,
    expires_at INTEGER NOT NULL,
    used_at INTEGER,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
) STRICT;

CREATE INDEX idx_password_reset_tokens_user ON password_reset_tokens(user_id, expires_at);
CREATE INDEX idx_password_reset_tokens_active ON password_reset_tokens(token_hash, expires_at, used_at);

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

CREATE INDEX idx_personal_access_tokens_user ON personal_access_tokens(user_id, revoked_at, expires_at);

CREATE TABLE email_verification_tokens (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash TEXT NOT NULL UNIQUE,
    expires_at INTEGER NOT NULL,
    used_at INTEGER,
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
) STRICT;

CREATE INDEX idx_email_verification_tokens_active ON email_verification_tokens(user_id, token_hash, expires_at, used_at);

-- Only one owner allowed across all users
CREATE UNIQUE INDEX idx_single_owner ON users(is_owner) WHERE is_owner = 1;

-- Trigger Function
CREATE TRIGGER users_updated_at
AFTER UPDATE ON users
FOR EACH ROW
BEGIN
	UPDATE users
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;

CREATE TRIGGER jwt_tokens_updated_at
AFTER UPDATE ON jwt_tokens
FOR EACH ROW
BEGIN
	UPDATE jwt_tokens
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;