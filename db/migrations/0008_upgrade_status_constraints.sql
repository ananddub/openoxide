-- Migration 0008: Upgrade status check constraints across compose_projects, applications, and database tables
-- Allows QUEUED, STARTING, STOPPING, STOPPED, CANCELLING, CANCELLED statuses

-- Note: SQLite check constraints updated in table schema definitions.
SELECT 1;
