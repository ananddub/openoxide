-- ============================================================
-- MIGRATION OBJECTS (per-object resume checkpoints)
-- ============================================================

-- name: MigrationObjectsUpsert :exec
INSERT INTO migration_objects (
    migration_id, obj_key, kind, source_ref, dest_ref, status, bytes, created_at, updated_at
) VALUES (?, ?, ?, ?, ?, ?, 0, ?, ?)
ON CONFLICT (migration_id, obj_key) DO NOTHING;

-- name: MigrationObjectsSetStatus :exec
UPDATE migration_objects
SET status = ?, error = ?, updated_at = ?
WHERE migration_id = ? AND obj_key = ?;

-- name: MigrationObjectsRecordBytes :exec
UPDATE migration_objects
SET bytes = ?, updated_at = ?
WHERE migration_id = ? AND obj_key = ?;

-- name: MigrationObjectsListByMigration :many
SELECT * FROM migration_objects WHERE migration_id = ? ORDER BY obj_key;

-- name: MigrationObjectsPending :many
SELECT * FROM migration_objects
WHERE migration_id = ? AND status <> 'DONE'
ORDER BY obj_key;
