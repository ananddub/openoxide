-- ============================================================
-- USERS
-- ============================================================

-- name: GetUserByID :one
SELECT * FROM users WHERE id = ?;

-- name: GetUserByEmail :one
SELECT * FROM users WHERE email = ?;

-- name: ListUsers :many
SELECT * FROM users ORDER BY created_at DESC;

-- name: ListUsersByGroup :many
SELECT * FROM users WHERE group_id = ? ORDER BY created_at DESC;

-- name: CreateUser :one
INSERT INTO users (
    email, first_name, last_name, avatar,
    role, password, is_registered, group_id, added_by
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
RETURNING *;

-- name: UpdateUser :one
UPDATE users
SET first_name = ?, last_name = ?, avatar = ?, about_me = ?
WHERE id = ?
RETURNING *;

-- name: UpdateUserRole :exec
UPDATE users SET role = ? WHERE id = ?;

-- name: UpdateUserPassword :exec
UPDATE users SET password = ? WHERE id = ?;

-- name: VerifyUserEmail :exec
UPDATE users
SET is_email_verify = 1, email_verify_at = CURRENT_TIMESTAMP
WHERE id = ?;

-- name: EnableTwoFactor :exec
UPDATE users SET two_factor_enable = 1 WHERE id = ?;

-- name: DisableTwoFactor :exec
UPDATE users SET two_factor_enable = 0 WHERE id = ?;

-- name: DeleteUser :exec
DELETE FROM users WHERE id = ?;

-- ============================================================
-- TWO FACTOR AUTH
-- ============================================================

-- name: GetTwoFactorByUserID :one
SELECT * FROM two_factor WHERE user_id = ?;

-- name: CreateTwoFactor :one
INSERT INTO two_factor (secret, backup_codes, user_id)
VALUES (?, ?, ?)
RETURNING *;

-- name: UpdateTwoFactor :exec
UPDATE two_factor SET secret = ?, backup_codes = ? WHERE user_id = ?;

-- name: DeleteTwoFactor :exec
DELETE FROM two_factor WHERE user_id = ?;

-- ============================================================
-- JWT TOKENS
-- ============================================================

-- name: CreateJwtToken :one
INSERT INTO jwt_tokens (jti, role, user_id, expired_at)
VALUES (?, ?, ?, ?)
RETURNING *;

-- name: GetJwtTokenByJti :one
SELECT * FROM jwt_tokens WHERE jti = ?;

-- name: ListActiveTokensByUser :many
SELECT * FROM jwt_tokens
WHERE user_id = ? AND is_blacklist = 0
ORDER BY created_at DESC;

-- name: BlacklistJwtToken :exec
UPDATE jwt_tokens
SET is_blacklist = 1, blacklist_at = CURRENT_TIMESTAMP
WHERE jti = ?;

-- name: BlacklistAllUserTokens :exec
UPDATE jwt_tokens
SET is_blacklist = 1, blacklist_at = CURRENT_TIMESTAMP
WHERE user_id = ? AND is_blacklist = 0;

-- name: DeleteExpiredTokens :exec
DELETE FROM jwt_tokens WHERE expired_at < strftime('%s','now');


-- name: UsersGetAll :many
SELECT id AS id, email AS email, last_name AS last_name, first_name AS first_name, avatar AS avatar, role AS role, about_me AS about_me, password AS password, is_email_verify AS is_email_verify, email_verify_at AS email_verify_at, two_factor_enable AS two_factor_enable, is_registered AS is_registered, added_by AS added_by, group_id AS group_id, created_at AS created_at, updated_at AS updated_at FROM users;

-- name: UsersGetById :one
SELECT id AS id, email AS email, last_name AS last_name, first_name AS first_name, avatar AS avatar, role AS role, about_me AS about_me, password AS password, is_email_verify AS is_email_verify, email_verify_at AS email_verify_at, two_factor_enable AS two_factor_enable, is_registered AS is_registered, added_by AS added_by, group_id AS group_id, created_at AS created_at, updated_at AS updated_at FROM users WHERE id = ?;

-- name: UsersCreate :exec
INSERT INTO users (email, last_name, first_name, avatar, role, about_me, password, is_email_verify, email_verify_at, two_factor_enable, is_registered, added_by, group_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);

-- name: UsersUpdate :exec
UPDATE users SET email = ?, last_name = ?, first_name = ?, avatar = ?, role = ?, about_me = ?, password = ?, is_email_verify = ?, email_verify_at = ?, two_factor_enable = ?, is_registered = ?, added_by = ?, group_id = ?, created_at = ?, updated_at = ? WHERE id = ?;

-- name: UsersDelete :exec
DELETE FROM users WHERE id = ?;

-- name: UsersCreateOwnerAndReturn :one
INSERT INTO users (
                    email, first_name, last_name, avatar, role,
                    password, is_registered, group_id, added_by
               ) VALUES (?, ?, ?, ?, 'OWNER', ?, 1, ?, NULL)
               RETURNING id AS id, email, last_name, first_name, avatar, role,
                         about_me, password, is_email_verify, email_verify_at,
                         two_factor_enable, is_registered, added_by, group_id,
                         created_at, updated_at;

-- Platform owner = the setup account: the first registered user, still
-- carrying the OWNER role. Signup hardcodes 'OWNER' for every account, so
-- membership in that role alone must not grant cross-organization power.
-- name: UsersIsPlatformOwner :one
SELECT EXISTS(
    SELECT 1 FROM users
    WHERE id = ? AND role = 'OWNER'
      AND id = (SELECT MIN(id) FROM users)
) AS is_owner;

-- name: UsersGetByEmail :one
SELECT id AS id, email AS email, last_name AS last_name, first_name AS first_name, avatar AS avatar, role AS role, about_me AS about_me, password AS password, is_email_verify AS is_email_verify, email_verify_at AS email_verify_at, two_factor_enable AS two_factor_enable, is_registered AS is_registered, added_by AS added_by, group_id AS group_id, created_at AS created_at, updated_at AS updated_at
               FROM users WHERE email = ?;

-- name: UsersSetTwoFactorEnabled :exec
UPDATE users SET two_factor_enable = ? WHERE id = ?;
