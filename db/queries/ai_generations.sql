-- Extracted from src/db/repository/ai_generations.rs. Do not edit repository SQL directly.

-- name: AiGenerationsCreate :one
INSERT INTO ai_generations
               (ai_setting_id, organization_id, created_by, prompt, output_json, status)
               VALUES (?, ?, ?, ?, ?, 'GENERATED')
               RETURNING id AS id, ai_setting_id, organization_id, created_by,
                         prompt, output_json, status, service_id, created_at, updated_at;

-- name: AiGenerationsGetForOrganization :one
SELECT id AS id, ai_setting_id, organization_id, created_by,
                      prompt, output_json, status, service_id, created_at, updated_at
               FROM ai_generations
               WHERE id = ? AND organization_id = ?;

-- name: AiGenerationsListForOrganization :many
SELECT id AS id, ai_setting_id, organization_id, created_by,
                      prompt, output_json, status, service_id, created_at, updated_at
               FROM ai_generations
               WHERE organization_id = ?
               ORDER BY created_at DESC, id DESC
               LIMIT ?;

-- name: AiGenerationsReview :one
UPDATE ai_generations
               SET output_json = ?, status = 'REVIEWED'
               WHERE id = ? AND organization_id = ? AND status != 'DEPLOYED'
               RETURNING id AS id, ai_setting_id, organization_id, created_by,
                         prompt, output_json, status, service_id, created_at, updated_at;

-- name: AiGenerationsMarkDeployed :one
UPDATE ai_generations
               SET status = 'DEPLOYED', service_id = ?
               WHERE id = ? AND organization_id = ? AND status = 'DEPLOYING'
               RETURNING id AS id, ai_setting_id, organization_id, created_by,
                         prompt, output_json, status, service_id, created_at, updated_at;

-- name: AiGenerationsClaimForDeploy :one
UPDATE ai_generations SET status = 'DEPLOYING'
               WHERE id = ? AND organization_id = ? AND status = 'REVIEWED'
               RETURNING id AS id;

-- name: AiGenerationsReleaseDeploy :exec
UPDATE ai_generations SET status = 'REVIEWED'
               WHERE id = ? AND organization_id = ? AND status = 'DEPLOYING';
