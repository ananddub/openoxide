CREATE TABLE notifications (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	-- notification_type: SLACK | TELEGRAM | DISCORD | EMAIL | RESEND | GOTIFY | NTFY | MATTERMOST | PUSHOVER | CUSTOM | LARK | TEAMS
	notification_type TEXT NOT NULL,
	on_app_deploy INTEGER NOT NULL DEFAULT 0,
	on_app_build_error INTEGER NOT NULL DEFAULT 0,
	on_database_backup INTEGER NOT NULL DEFAULT 0,
	on_volume_backup INTEGER NOT NULL DEFAULT 0,
	on_panel_backup INTEGER NOT NULL DEFAULT 0,
	on_panel_restart INTEGER NOT NULL DEFAULT 0,
	on_docker_cleanup INTEGER NOT NULL DEFAULT 0,
	on_server_threshold INTEGER NOT NULL DEFAULT 0,
	on_server_migration INTEGER NOT NULL DEFAULT 0,
	on_security_alert INTEGER NOT NULL DEFAULT 0,
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
	updated_at INTEGER DEFAULT (strftime('%s', 'now')) NOT NULL, on_schedule_success INTEGER NOT NULL DEFAULT 0 CHECK (on_schedule_success IN (0, 1)), on_schedule_failure INTEGER NOT NULL DEFAULT 0 CHECK (on_schedule_failure IN (0, 1)),
	CONSTRAINT notif_type_check CHECK (
		notification_type IN ('SLACK', 'TELEGRAM', 'DISCORD', 'EMAIL', 'RESEND', 'GOTIFY', 'NTFY', 'MATTERMOST', 'PUSHOVER', 'CUSTOM', 'LARK', 'TEAMS')
	)
) STRICT;
CREATE INDEX idx_notifications_organization_id ON notifications(organization_id);
CREATE TRIGGER notifications_updated_at
AFTER UPDATE ON notifications
FOR EACH ROW
BEGIN
	UPDATE notifications
	SET updated_at = strftime('%s', 'now')
	WHERE id = OLD.id;
END;
