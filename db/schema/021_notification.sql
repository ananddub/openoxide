-- SUB-TABLES: Provider Configurations
CREATE TABLE notif_slack (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	webhook_url TEXT NOT NULL,
	channel TEXT
) STRICT;

CREATE TABLE notif_telegram (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	bot_token TEXT NOT NULL,
	chat_id TEXT NOT NULL,
	message_thread_id TEXT
) STRICT;

CREATE TABLE notif_discord (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	webhook_url TEXT NOT NULL,
	decoration INTEGER NOT NULL DEFAULT 0
) STRICT;

CREATE TABLE notif_email (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	smtp_server TEXT NOT NULL,
	smtp_port INTEGER NOT NULL,
	username TEXT NOT NULL,
	password TEXT NOT NULL,
	from_address TEXT NOT NULL,
	to_addresses TEXT NOT NULL DEFAULT '[]' -- JSON array of strings
) STRICT;

CREATE TABLE notif_resend (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	api_key TEXT NOT NULL,
	from_address TEXT NOT NULL,
	to_addresses TEXT NOT NULL DEFAULT '[]' -- JSON array of strings
) STRICT;

CREATE TABLE notif_gotify (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	server_url TEXT NOT NULL,
	app_token TEXT NOT NULL,
	priority INTEGER NOT NULL DEFAULT 5,
	decoration INTEGER NOT NULL DEFAULT 0
) STRICT;

CREATE TABLE notif_ntfy (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	server_url TEXT NOT NULL,
	topic TEXT NOT NULL,
	access_token TEXT,
	priority INTEGER NOT NULL DEFAULT 3
) STRICT;

CREATE TABLE notif_mattermost (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	webhook_url TEXT NOT NULL,
	channel TEXT,
	username TEXT
) STRICT;

CREATE TABLE notif_teams (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	webhook_url TEXT NOT NULL
) STRICT;

CREATE TABLE notif_lark (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	webhook_url TEXT NOT NULL
) STRICT;

CREATE TABLE notif_pushover (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	user_key TEXT NOT NULL,
	api_token TEXT NOT NULL,
	priority INTEGER NOT NULL DEFAULT 0,
	retry INTEGER,
	expire INTEGER
) STRICT;

CREATE TABLE notif_custom (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	endpoint TEXT NOT NULL,
	headers TEXT -- JSON object (headers Record<string, string>)
) STRICT;

-- MAIN TABLE: notifications (Triggers & Links)
CREATE TABLE notifications (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	-- notification_type: SLACK | TELEGRAM | DISCORD | EMAIL | RESEND | GOTIFY | NTFY | MATTERMOST | PUSHOVER | CUSTOM | LARK | TEAMS
	notification_type TEXT NOT NULL,
	on_app_deploy INTEGER NOT NULL DEFAULT 0,
	on_app_build_error INTEGER NOT NULL DEFAULT 0,
	on_database_backup INTEGER NOT NULL DEFAULT 0,
	on_volume_backup INTEGER NOT NULL DEFAULT 0,
	on_panel_restart INTEGER NOT NULL DEFAULT 0,
	on_docker_cleanup INTEGER NOT NULL DEFAULT 0,
	on_server_threshold INTEGER NOT NULL DEFAULT 0,
	on_panel_backup INTEGER NOT NULL DEFAULT 0,
	on_schedule_success INTEGER NOT NULL DEFAULT 0 CHECK (on_schedule_success IN (0, 1)),
	on_schedule_failure INTEGER NOT NULL DEFAULT 0 CHECK (on_schedule_failure IN (0, 1)),
	-- Foreign keys
	slack_id INTEGER REFERENCES notif_slack(id) ON DELETE CASCADE,
	telegram_id INTEGER REFERENCES notif_telegram(id) ON DELETE CASCADE,
	discord_id INTEGER REFERENCES notif_discord(id) ON DELETE CASCADE,
	email_id INTEGER REFERENCES notif_email(id) ON DELETE CASCADE,
	resend_id INTEGER REFERENCES notif_resend(id) ON DELETE CASCADE,
	gotify_id INTEGER REFERENCES notif_gotify(id) ON DELETE CASCADE,
	ntfy_id INTEGER REFERENCES notif_ntfy(id) ON DELETE CASCADE,
	mattermost_id INTEGER REFERENCES notif_mattermost(id) ON DELETE CASCADE,
	custom_id INTEGER REFERENCES notif_custom(id) ON DELETE CASCADE,
	lark_id INTEGER REFERENCES notif_lark(id) ON DELETE CASCADE,
	pushover_id INTEGER REFERENCES notif_pushover(id) ON DELETE CASCADE,
	teams_id INTEGER REFERENCES notif_teams(id) ON DELETE CASCADE,
	organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
	created_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL,
	CONSTRAINT notif_type_check CHECK (
		notification_type IN ('SLACK', 'TELEGRAM', 'DISCORD', 'EMAIL', 'RESEND', 'GOTIFY', 'NTFY', 'MATTERMOST', 'PUSHOVER', 'CUSTOM', 'LARK', 'TEAMS')
	)
) STRICT;

CREATE TABLE notification_delivery_attempts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    notification_id INTEGER NOT NULL REFERENCES notifications(id) ON DELETE CASCADE,
    organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
    trigger_name TEXT NOT NULL,
    correlation_id TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('PENDING', 'DELIVERED', 'FAILED')),
    attempt INTEGER NOT NULL DEFAULT 1,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    error TEXT,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    finished_at INTEGER
) STRICT;

CREATE INDEX idx_notification_attempts_org_created ON notification_delivery_attempts(organization_id, created_at DESC);
CREATE INDEX idx_notification_attempts_retry ON notification_delivery_attempts(status, attempt, created_at);

CREATE TABLE notification_resource_bindings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    notification_id INTEGER NOT NULL REFERENCES notifications(id) ON DELETE CASCADE,
    organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
    resource_type TEXT NOT NULL CHECK (resource_type IN ('SERVER', 'APPLICATION', 'COMPOSE', 'DATABASE')),
    resource_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch()),
    UNIQUE(notification_id, resource_type, resource_id)
) STRICT;

-- Indexes for faster queries
CREATE INDEX idx_notifications_organization_id ON notifications(organization_id);

-- Trigger Function
CREATE TRIGGER notifications_updated_at
AFTER UPDATE ON notifications
FOR EACH ROW
BEGIN
	UPDATE notifications
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;