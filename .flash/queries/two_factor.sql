-- Extracted from src/db/repository/two_factor.rs. Do not edit repository SQL directly.

-- name: TwoFactorGetAll :many
SELECT id AS id, secret AS secret, backup_codes AS backup_codes, user_id AS user_id FROM two_factor;

-- name: TwoFactorGetById :one
SELECT id AS id, secret AS secret, backup_codes AS backup_codes, user_id AS user_id FROM two_factor WHERE id = ?;

-- name: TwoFactorCreate :exec
INSERT INTO two_factor (secret, backup_codes, user_id) VALUES (?, ?, ?);

-- name: TwoFactorUpdate :exec
UPDATE two_factor SET secret = ?, backup_codes = ?, user_id = ? WHERE id = ?;

-- name: TwoFactorDelete :exec
DELETE FROM two_factor WHERE id = ?;

-- name: TwoFactorGetByUserId :one
SELECT id AS id, secret AS secret, backup_codes AS backup_codes, user_id AS user_id FROM two_factor WHERE user_id = ?;

-- name: TwoFactorUpsert :exec
INSERT INTO two_factor (secret, backup_codes, user_id) VALUES (?, ?, ?)
               ON CONFLICT(user_id) DO UPDATE SET secret = excluded.secret, backup_codes = excluded.backup_codes;

-- name: TwoFactorConsumeRecoveryCode :exec
UPDATE two_factor SET backup_codes = ? WHERE user_id = ? AND backup_codes = ?;

-- name: TwoFactorReplaceRecoveryCodes :exec
UPDATE two_factor SET backup_codes = ? WHERE user_id = ?;

-- name: TwoFactorDeleteByUserId :exec
DELETE FROM two_factor WHERE user_id = ?;
