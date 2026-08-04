-- Migration 0013: Scope alert rules to an organization.
--
-- Without this, an alert raised for one organization's server fans out to every
-- organization's notification channels.

ALTER TABLE alert_rules ADD COLUMN organization_id INTEGER NOT NULL DEFAULT 1;

CREATE INDEX IF NOT EXISTS idx_alert_rules_organization ON alert_rules(organization_id);
