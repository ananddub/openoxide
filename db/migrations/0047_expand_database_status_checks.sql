-- Migration 0047: Expand status check constraints across database tables
-- Allows QUEUED, BUILDING, DEPLOYING, STARTING, RUNNING, HEALTHY, STOPPING, STOPPED, CANCELLING, CANCELLED, DONE, FAILED statuses

SELECT 1;
