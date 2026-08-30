-- Extracted from src/db/repository/webhooks.rs. Do not edit repository SQL directly.

-- name: WebhooksCredentialSecret1 :one
SELECT webhook_secret FROM git_providers WHERE id = ?;

-- name: WebhooksCredentialSecret2 :one
SELECT webhook_secret FROM git_providers WHERE id = ?;

-- name: WebhooksCredentialSecret3 :one
SELECT webhook_secret FROM git_providers WHERE id = ?;

-- name: WebhooksCredentialSecret4 :one
SELECT webhook_secret FROM git_providers WHERE id = ?;
