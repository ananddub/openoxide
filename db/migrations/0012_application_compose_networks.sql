-- Migration 0012: Add application and compose custom network selection.

ALTER TABLE applications ADD COLUMN network_ids TEXT NOT NULL DEFAULT '[]';
ALTER TABLE applications ADD COLUMN detach_rustploy_network INTEGER NOT NULL DEFAULT 0;

ALTER TABLE compose_projects ADD COLUMN service_networks TEXT NOT NULL DEFAULT '[]';
