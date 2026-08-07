-- Health checks report table
CREATE TABLE health_reports (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	target_id INTEGER NOT NULL,
	target_type TEXT NOT NULL, -- 'APPLICATION', 'DATABASE', 'SERVER'
	status TEXT NOT NULL,      -- 'HEALTHY', 'UNHEALTHY', 'DEGRADED'
	response_time_ms INTEGER NOT NULL DEFAULT 0,
	error_message TEXT,
	created_at INTEGER NOT NULL DEFAULT (unixepoch())
);

CREATE INDEX idx_health_reports_target ON health_reports(target_type, target_id);
CREATE INDEX idx_health_reports_created ON health_reports(created_at);

-- Alerting rules configuration
CREATE TABLE alert_rules (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL CHECK (length(trim(name)) BETWEEN 1 AND 255),
	target_type TEXT NOT NULL CHECK (target_type IN ('SERVER', 'APPLICATION', 'DATABASE', 'COMPOSE')),
	target_id INTEGER NOT NULL CHECK (target_id >= 0),
	metric_name TEXT NOT NULL CHECK (metric_name IN ('CPU', 'MEMORY', 'DISK')),
	operator TEXT NOT NULL CHECK (operator IN ('GT', 'GTE', 'LT', 'LTE', 'EQ')),
	threshold REAL NOT NULL CHECK (threshold >= 0.0 AND threshold <= 100.0),
	duration_seconds INTEGER NOT NULL DEFAULT 60 CHECK (duration_seconds BETWEEN 0 AND 86400),
	notification_channel TEXT NOT NULL DEFAULT 'SYSTEM' CHECK (notification_channel = 'SYSTEM'),
	enabled INTEGER NOT NULL DEFAULT 1 CHECK (enabled IN (0, 1)),
	organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
	created_at INTEGER NOT NULL DEFAULT (unixepoch()),
	updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

CREATE INDEX idx_alert_rules_target ON alert_rules(target_type, target_id);
CREATE INDEX idx_alert_rules_organization ON alert_rules(organization_id);

-- Monitoring agent tokens and status
CREATE TABLE monitoring_agents (
	server_id INTEGER PRIMARY KEY REFERENCES servers(id) ON DELETE CASCADE,
	organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
	token_hash TEXT NOT NULL,
	query_token TEXT,
	last_seen_at INTEGER,
	agent_version TEXT,
	created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
) STRICT;

-- Monitoring retention and agent version policy
CREATE TABLE monitoring_policy (
    organization_id INTEGER PRIMARY KEY REFERENCES organization(id) ON DELETE CASCADE,
    desired_agent_version TEXT,
    retention_days INTEGER NOT NULL DEFAULT 7 CHECK (retention_days BETWEEN 1 AND 3650),
    updated_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

-- Monitoring maintenance windows (silences alerts)
CREATE TABLE monitoring_maintenance_windows (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
    server_id INTEGER REFERENCES servers(id) ON DELETE CASCADE,
    starts_at INTEGER NOT NULL,
    ends_at INTEGER NOT NULL CHECK (ends_at > starts_at),
    reason TEXT NOT NULL DEFAULT '',
    created_at INTEGER NOT NULL DEFAULT (unixepoch())
) STRICT;

CREATE INDEX idx_monitoring_windows_scope ON monitoring_maintenance_windows(organization_id, starts_at, ends_at);

-- Alert events lifecycle history
CREATE TABLE alert_events (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	alert_rule_id INTEGER NOT NULL REFERENCES alert_rules(id) ON DELETE CASCADE,
	organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
	target_key TEXT NOT NULL,
	state TEXT NOT NULL CHECK (state IN ('FIRING', 'RESOLVED', 'NO_DATA')),
	value REAL,
	threshold REAL,
	message TEXT NOT NULL,
	acknowledged_at INTEGER,
	acknowledged_by INTEGER,
	silenced_until INTEGER,
	resolved_at INTEGER,
	notification_correlation_id TEXT,
	created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
) STRICT;

CREATE INDEX idx_alert_events_org_created ON alert_events(organization_id, created_at DESC);
CREATE INDEX idx_alert_events_rule_target_created ON alert_events(alert_rule_id, target_key, created_at DESC);
