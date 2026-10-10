-- Cache of AI-generated, build-verified Dockerfiles keyed by a repo manifest
-- signature, so repeat deploys of the same repo skip AI regeneration.
CREATE TABLE IF NOT EXISTS ai_dockerfile_cache (
    signature  TEXT    NOT NULL PRIMARY KEY,
    dockerfile TEXT    NOT NULL,
    updated_at INTEGER NOT NULL
);
