-- name: GetR :one
SELECT * FROM redirects WHERE id = ?;

-- name: GetRKey :one
SELECT cfg_key FROM redirects WHERE id = ?;
