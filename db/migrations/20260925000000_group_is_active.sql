-- Per-role enable/disable: a group's actions only reach members while the
-- group is active. Disabled groups keep their policy set and membership so
-- access can be restored without recreating the role.
ALTER TABLE groups ADD COLUMN is_active INTEGER NOT NULL DEFAULT 1;
