ALTER TABLE notifications ADD COLUMN on_schedule_success INTEGER NOT NULL DEFAULT 0 CHECK (on_schedule_success IN (0, 1));
ALTER TABLE notifications ADD COLUMN on_schedule_failure INTEGER NOT NULL DEFAULT 0 CHECK (on_schedule_failure IN (0, 1));
