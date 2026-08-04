-- Table to store host server metrics
CREATE TABLE server_metrics (
	timestamp INTEGER PRIMARY KEY,
	server_id INTEGER NOT NULL DEFAULT 1 REFERENCES servers(id) ON DELETE CASCADE,
	cpu REAL NOT NULL,
	cpu_model TEXT NOT NULL,
	cpu_cores INTEGER NOT NULL,
	cpu_physical_cores INTEGER NOT NULL,
	cpu_speed REAL NOT NULL,
	os TEXT NOT NULL,
	distro TEXT NOT NULL,
	kernel TEXT NOT NULL,
	arch TEXT NOT NULL,
	mem_used REAL NOT NULL,
	mem_used_gb REAL NOT NULL,
	mem_total REAL NOT NULL,
	uptime INTEGER NOT NULL,
	disk_used REAL NOT NULL,
	total_disk REAL NOT NULL,
	network_in REAL NOT NULL,
	network_out REAL NOT NULL
) STRICT;

CREATE INDEX idx_server_metrics_server_timestamp ON server_metrics(server_id, timestamp DESC);

-- Table to store individual docker container metrics
CREATE TABLE container_metrics (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	timestamp INTEGER NOT NULL,
	server_id INTEGER NOT NULL DEFAULT 1 REFERENCES servers(id) ON DELETE CASCADE,
	application_id INTEGER,
	compose_id INTEGER,
	container_id TEXT NOT NULL,
	container_name TEXT NOT NULL,
	metrics_json TEXT NOT NULL
) STRICT;

CREATE INDEX idx_container_metrics_timestamp ON container_metrics(timestamp);
CREATE INDEX idx_container_metrics_name ON container_metrics(container_name);
CREATE INDEX idx_container_metrics_server_container_timestamp ON container_metrics(server_id, container_id, timestamp DESC);

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
	name TEXT NOT NULL,
	target_type TEXT NOT NULL, -- 'SERVER', 'APPLICATION', 'DATABASE'
	target_id INTEGER NOT NULL,
	metric_name TEXT NOT NULL, -- 'CPU', 'MEMORY', 'DISK', 'HEALTH'
	operator TEXT NOT NULL,    -- 'GT', 'GTE', 'LT', 'LTE', 'EQ'
	threshold REAL NOT NULL,
	duration_seconds INTEGER NOT NULL DEFAULT 60,
	notification_channel TEXT NOT NULL DEFAULT 'SYSTEM',
	enabled INTEGER NOT NULL DEFAULT 1,
	organization_id INTEGER NOT NULL DEFAULT 1 REFERENCES organization(id) ON DELETE CASCADE,
	created_at INTEGER NOT NULL DEFAULT (unixepoch()),
	updated_at INTEGER NOT NULL DEFAULT (unixepoch())
);

CREATE INDEX idx_alert_rules_target ON alert_rules(target_type, target_id);
CREATE INDEX idx_alert_rules_organization ON alert_rules(organization_id);

-- Monitoring agent tokens and status
CREATE TABLE monitoring_agents (
	server_id INTEGER PRIMARY KEY REFERENCES servers(id) ON DELETE CASCADE,
	organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
	token_hash TEXT NOT NULL,
	last_seen_at INTEGER,
	agent_version TEXT,
	created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
) STRICT;

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
	created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
) STRICT;

CREATE INDEX idx_alert_events_org_created ON alert_events(organization_id, created_at DESC);