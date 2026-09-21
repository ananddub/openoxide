-- Attribute deployments to the acting user. 0 = system-triggered deploys
-- (webhooks, schedules, preview factory) — the UI renders those as "System".
ALTER TABLE deployments ADD COLUMN user_id INTEGER NOT NULL DEFAULT 0;
