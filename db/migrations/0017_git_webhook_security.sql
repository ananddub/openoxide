ALTER TABLE git_providers ADD COLUMN webhook_secret TEXT;

CREATE INDEX IF NOT EXISTS idx_git_providers_webhook_secret
ON git_providers(webhook_secret)
WHERE webhook_secret IS NOT NULL;
