CREATE TABLE notifications (
    id BIGINT NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    notification_type TEXT NOT NULL,
    config_json TEXT NOT NULL DEFAULT '{}',
    on_app_deploy BIGINT NOT NULL,
    on_app_build_error BIGINT NOT NULL,
    on_database_backup BIGINT NOT NULL,
    on_volume_backup BIGINT NOT NULL,
    on_panel_backup BIGINT NOT NULL,
    on_panel_restart BIGINT NOT NULL,
    on_docker_cleanup BIGINT NOT NULL,
    on_server_threshold BIGINT NOT NULL,
    on_server_migration BIGINT NOT NULL,
    on_security_alert BIGINT NOT NULL,
    on_schedule_success BIGINT NOT NULL,
    on_schedule_failure BIGINT NOT NULL,
    organization_id BIGINT NOT NULL,
    created_at BIGINT NOT NULL,
    updated_at BIGINT NOT NULL,
    CONSTRAINT notification_config_json_check CHECK (json_valid(config_json))
);

CREATE INDEX idx_notifications_organization_id ON notifications(organization_id);
