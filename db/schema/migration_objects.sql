CREATE TABLE migration_objects (
    migration_id TEXT NOT NULL,
    obj_key      TEXT NOT NULL,
    kind         TEXT NOT NULL,
    source_ref   TEXT NOT NULL,
    dest_ref     TEXT NOT NULL,
    status       TEXT NOT NULL,
    bytes        BIGINT NOT NULL DEFAULT 0,
    error        TEXT,
    created_at   BIGINT NOT NULL,
    updated_at   BIGINT NOT NULL,
    PRIMARY KEY (migration_id, obj_key)
);

CREATE INDEX idx_migration_objects_migration ON migration_objects(migration_id);
CREATE INDEX idx_migration_objects_status ON migration_objects(migration_id, status);
