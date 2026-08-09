# Rustploy Ephemeral Preview Deployments Architecture

The **Preview Deployment Service** provisions automated, temporary preview environments for GitHub / GitLab / Gitea / Bitbucket Pull Requests (PRs), generating ephemeral domain URLs and destroying preview infrastructure when PRs are closed.

---

## 1. Architecture Overview

```
                      ┌───────────────────────────────┐
                      │    Incoming PR Webhook Event  │
                      │   (Opened / Reopened / Closed)│
                      └───────────────┬───────────────┘
                                      │
                                      ▼
                      ┌───────────────────────────────┐
                      │  PreviewDeploymentService     │
                      └───────────────┬───────────────┘
                                      │
      ┌──────────────────┬────────────┴─────────────┬──────────────────┐
      ▼                  ▼                          ▼                  ▼
┌──────────────┐ ┌──────────────┐          ┌──────────────┐   ┌──────────────┐
│ PR Permission│ │ Ephemeral App│          │ Dynamic Domain│  │ Auto Cleanup │
│ Security Check││ Clone Engine │          │ Provisioner  │   │ Manager      │
└──────────────┘ └──────────────┘          └──────────────┘   └──────────────┘
```

---

## 2. Detailed Internal Working Mechanism

Preview Deployments operate through a 5-phase lifecycle:

```
[1. PR Webhook]     ──► Pull Request Event Received ('opened' / 'synchronize' / 'closed')
                             │
[2. Security Guard] ──► Collaborator Permission Check (Require Collaborator Approval)
                             │
[3. Ephemeral Clone]──► Clone Base App Config + Create Isolated Preview Application
                             │
[4. Domain & Deploy]──► Assign Ephemeral Domain (`pr-42-app.preview.domain.com`) + Deploy
                             │
[5. Auto Destruction]─► PR Closed/Merged ──► Destroy Containers, Volumes, & Domains
```

### Phase 1: Pull Request Webhook Ingestion (`lifecycle.rs`)
1. Receives incoming Pull Request webhook payload from GitHub, GitLab, Gitea, or Bitbucket.
2. Extracts PR number (`#42`), action (`opened`, `synchronize`, `closed`), source branch, and target branch.

### Phase 2: Security & Collaborator Verification (`lifecycle.rs`)
1. Checks if `is_preview_deployments_active = 1` on base application.
2. Evaluates `preview_require_collaborator_permissions`: Verifies if PR author has write/collaborator access to prevent arbitrary code execution from malicious public PRs.

### Phase 3: Ephemeral Application Cloning (`lifecycle.rs`)
1. Creates an ephemeral isolated clone of the base application.
2. Overrides Git branch to PR source branch (`pr-42-branch`) and commit SHA.
3. Sets up isolated environment variables and container names (`app-preview-pr-42`).

### Phase 4: Ephemeral Domain Assignment & Deployment (`lifecycle.rs`)
1. Provisions temporary subdomain routing (`pr-42-app.preview.domain.com`).
2. Triggers `DeploymentService` to build and deploy the PR preview stack.
3. Posts preview URL status back to GitHub PR comments / commit checks via Git API.

### Phase 5: Automated Cleanup on PR Closure (`lifecycle.rs`)
1. When PR is closed or merged:
2. Stops preview container, deletes ephemeral application DB record, removes persistent volumes, and unregisters Traefik domain routes.

---

## 3. Database Schema & Models

### 3.1 `preview_deployments` Table
- `id` (INTEGER PRIMARY KEY)
- `base_application_id` (INTEGER REFERENCES applications(id) ON DELETE CASCADE)
- `preview_application_id` (INTEGER NULLABLE REFERENCES applications(id))
- `provider_type` (TEXT NOT NULL): `'GITHUB'`, `'GITLAB'`, `'GITEA'`, `'BITBUCKET'`.
- `owner` (TEXT NOT NULL)
- `repository` (TEXT NOT NULL)
- `pull_request_number` (TEXT NOT NULL)
- `source_branch` (TEXT NOT NULL)
- `target_branch` (TEXT NOT NULL)
- `commit_sha` (TEXT NULLABLE)
- `domain` (TEXT NOT NULL)
- `status` (TEXT NOT NULL): `'ACTIVE'`, `'CLOSED'`.
- `created_at` (INTEGER DEFAULT (unixepoch())).
