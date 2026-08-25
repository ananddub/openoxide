-- Extracted from src/db/repository/users.rs. Do not edit repository SQL directly.

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

-- name: UsersGetByEmail :one
SELECT id AS id, email AS email, last_name AS last_name, first_name AS first_name, avatar AS avatar, role AS role, about_me AS about_me, password AS password, is_email_verify AS is_email_verify, email_verify_at AS email_verify_at, two_factor_enable AS two_factor_enable, is_registered AS is_registered, added_by AS added_by, group_id AS group_id, created_at AS created_at, updated_at AS updated_at
               FROM users WHERE email = ?;

-- name: UsersSetTwoFactorEnabled :exec
UPDATE users SET two_factor_enable = ? WHERE id = ?;
