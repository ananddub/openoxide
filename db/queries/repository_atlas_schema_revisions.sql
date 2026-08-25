-- Extracted from src/db/repository/atlas_schema_revisions.rs. Do not edit repository SQL directly.

-- name: AtlasSchemaRevisionsGetAll :many
SELECT version AS version, description AS description, type AS atlas_schema_revision_type, applied AS applied, total AS total, executed_at AS executed_at, execution_time AS execution_time, error AS error, error_stmt AS error_stmt, hash AS hash, partial_hashes AS partial_hashes, operator_version AS operator_version FROM atlas_schema_revisions;

-- name: AtlasSchemaRevisionsGetById :one
SELECT version AS version, description AS description, type AS atlas_schema_revision_type, applied AS applied, total AS total, executed_at AS executed_at, execution_time AS execution_time, error AS error, error_stmt AS error_stmt, hash AS hash, partial_hashes AS partial_hashes, operator_version AS operator_version FROM atlas_schema_revisions WHERE version = ?;

-- name: AtlasSchemaRevisionsCreate :exec
INSERT INTO atlas_schema_revisions (version, description, type, applied, total, executed_at, execution_time, error, error_stmt, hash, partial_hashes, operator_version) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

-- name: AtlasSchemaRevisionsUpdate :exec
UPDATE atlas_schema_revisions SET description = ?, type = ?, applied = ?, total = ?, executed_at = ?, execution_time = ?, error = ?, error_stmt = ?, hash = ?, partial_hashes = ?, operator_version = ? WHERE version = ?;

-- name: AtlasSchemaRevisionsDelete :exec
DELETE FROM atlas_schema_revisions WHERE version = ?;
