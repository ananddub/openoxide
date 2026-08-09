# Rustploy Permission & Access Control Architecture

This document provides a comprehensive guide to the Permission System in Rustploy, combining **Role-Based Access Control (RBAC)** with **Fine-Grained Per-User & Resource-Level Overrides (ABAC)**.

---

## 1. System Architecture Overview

Rustploy uses a multi-tenant RBAC system with per-user override layers:

```
                               ┌──────────────┐
                               │    users     │ (Global User Accounts)
                               └──────┬───────┘
                                      │ (1:N)
                                      ▼
                           ┌─────────────────────┐
                           │ organization_members│ ◄── Primary Junction (User + Org + Group)
                           └──────────┬──────────┘
             (1:N Organization)       │ (N:1 Group Assignment)
                     ┌────────────────┴────────────────┐
                     ▼                                 ▼
            ┌────────────────┐                   ┌───────────┐
            │  organization  │                   │  groups   │ (Roles: Admin, Dev, Viewer)
            └────────────────┘                   └─────┬─────┘
                                                       │ (1:N)
                                                       ▼
                                             ┌──────────────────┐
                                             │   group_policy   │ ◄── Group-Policy Mapping
                                             └─────────┬────────┘
                                                 (N:1) │
                                                       ▼
                                                  ┌────────┐
                                                  │ policy │ (Actions: 'applications:read', etc.)
                                                  └────────┘

 ┌─────────────────┐             ┌─────────────────┐
 │   user_policy   │ ◄───────────┤ resource_access │ (Direct Per-User Overrides)
 └─────────────────┘             └─────────────────┘
 (Action DENY/GRANT)             (Item-Level Scoping)
```

---

## 2. Database Schema & Tables

### 2.1 Core Entities

#### `users`
Represents individual user accounts across the system.
- `id` (INTEGER PRIMARY KEY)
- `email` (TEXT UNIQUE)
- `password_hash` (TEXT)
- `is_owner` (INTEGER): `1` for Super Admin / Owner (bypasses all permission checks).

#### `organization`
Multi-tenant workspace container.
- `id` (INTEGER PRIMARY KEY)
- `name` (TEXT UNIQUE)
- `slug` (TEXT UNIQUE)
- `owner_id` (INTEGER REFERENCES users(id))

---

### 2.2 Role-Based Access Control (RBAC) Tables

#### `groups`
Defines roles/permission groups.
- `id` (INTEGER PRIMARY KEY)
- `name` (TEXT NOT NULL)
- `is_system` (INTEGER DEFAULT 0): `1` for locked system roles (e.g. `Organization Admin`).
- `organization_id` (INTEGER NULLABLE REFERENCES organization(id)): NULL for global system groups, non-NULL for organization-custom groups.

#### `policy`
Master catalog of granular system permissions.
- `id` (INTEGER PRIMARY KEY)
- `action` (TEXT NOT NULL UNIQUE): Formatted as `<resource>:<operation>` (e.g. `applications:read`, `servers:delete`).

#### `group_policy`
Junction table linking Roles/Groups to Policies.
- `group_id` (INTEGER REFERENCES groups(id))
- `policy_id` (INTEGER REFERENCES policy(id))
- `UNIQUE(group_id, policy_id)`

#### `organization_members`
Main user assignment table mapping a User to an Organization with a specific Group/Role.
- `id` (INTEGER PRIMARY KEY)
- `user_id` (INTEGER REFERENCES users(id))
- `organization_id` (INTEGER REFERENCES organization(id))
- `group_id` (INTEGER REFERENCES groups(id))
- `role` (TEXT DEFAULT 'MEMBER')

---

### 2.3 Per-User Override Tables (ABAC)

#### `user_policy`
Direct action-level overrides assigned specifically to a single user, overriding their group rules.
- `id` (INTEGER PRIMARY KEY)
- `user_id` (INTEGER REFERENCES users(id))
- `org_id` (INTEGER REFERENCES organization(id))
- `policy_id` (INTEGER REFERENCES policy(id))
- `effect` (TEXT NOT NULL): `'GRANT'` (Add extra permission) or `'DENY'` (Strictly block permission).
- `UNIQUE(user_id, org_id, policy_id)`

#### `resource_access`
Item-level access controls restricting or allowing access to specific resources.
- `id` (INTEGER PRIMARY KEY)
- `user_id` (INTEGER REFERENCES users(id))
- `org_id` (INTEGER REFERENCES organization(id))
- `resource_type` (TEXT): e.g. `'PROJECT'`, `'SERVER'`, `'ENVIRONMENT'`.
- `resource_id` (INTEGER): Specific database ID of the target resource.

---

## 3. Effective Permission Evaluation Algorithm

When a user executes an API request, their effective permissions are computed using the following CTE SQL evaluation:

```sql
WITH overrides AS (
    -- Fetch direct user overrides for the current Organization
    SELECT p.action, up.effect
    FROM user_policy up
    JOIN policy p ON p.id = up.policy_id
    WHERE up.user_id = ? AND up.org_id = ?
)
SELECT DISTINCT action FROM (
    -- 1. Base permissions from assigned Organization Group/Role
    SELECT p.action
    FROM organization_members om
    JOIN group_policy gp ON gp.group_id = om.group_id
    JOIN policy p ON p.id = gp.policy_id
    WHERE om.user_id = ? AND om.organization_id = ?
    
    UNION ALL
    
    -- 2. Include extra GRANT overrides
    SELECT action FROM overrides WHERE effect = 'GRANT'
)
-- 3. Exclude DENY overrides (DENY overrides take absolute precedence!)
WHERE action NOT IN (SELECT action FROM overrides WHERE effect = 'DENY')
ORDER BY action;
```

---

## 4. Permission Resolution Precedence Rules

When determining if a user is allowed to perform an action:

1. **Super Admin / Server Owner (`users.is_owner = 1`)**:
   - **Result**: `ALLOW` immediately. Bypasses all group and policy checks.
2. **`DENY` Override (`user_policy.effect = 'DENY'`)**:
   - **Result**: `BLOCK`. Takes precedence over group permissions and `GRANT` overrides.
3. **`GRANT` Override (`user_policy.effect = 'GRANT'`)**:
   - **Result**: `ALLOW`. Grants permission even if the user's Group role lacks it.
4. **Group Role Permissions (`organization_members` -> `groups` -> `group_policy`)**:
   - **Result**: Standard role-based access.
5. **Default Fallback**:
   - **Result**: `BLOCK` (HTTP 403 Forbidden).

---

## 5. Dedicated Per-User Overrides (ABAC Deep Dive)

Per-user permissions allow administrators to bypass group roles and assign granular exceptions or item-level restrictions to specific users.

```
                  ┌──────────────────────┐
                  │    users (User ID)   │
                  └──────────┬───────────┘
                             │
            ┌────────────────┴────────────────┐
            ▼                                 ▼
┌───────────────────────┐         ┌───────────────────────┐
│      user_policy      │         │    resource_access    │
│───────────────────────│         │───────────────────────│
│ user_id: 5            │         │ user_id: 5            │
│ org_id: 1             │         │ org_id: 1             │
│ policy_id: 10         │         │ resource_type:        │
│ effect: 'DENY'        │         │   'PROJECT'           │
└───────────────────────┘         │ resource_id: 42       │
                                  └───────────────────────┘
  (Action-Level Exception)           (Resource-Level Scoping)
```

### 5.1 Action-Level Per-User Overrides (`user_policy`)

#### A. Granting Extra Permission (`GRANT`)
If a user belongs to a `Viewer` group, but needs specific access to deploy applications:
```rust
user_policy_repo.upsert(user_id, org_id, policy_id, "GRANT").await?;
```
- **SQL Executed**:
  ```sql
  INSERT INTO user_policy (user_id, org_id, policy_id, effect)
  VALUES (?, ?, ?, 'GRANT')
  ON CONFLICT(user_id, org_id, policy_id) DO UPDATE SET effect = 'GRANT';
  ```

#### B. Revoking Permission (`DENY`)
If an `Admin` user should be blocked from deleting servers:
```rust
user_policy_repo.upsert(user_id, org_id, policy_id, "DENY").await?;
```
- **SQL Executed**:
  ```sql
  INSERT INTO user_policy (user_id, org_id, policy_id, effect)
  VALUES (?, ?, ?, 'DENY')
  ON CONFLICT(user_id, org_id, policy_id) DO UPDATE SET effect = 'DENY';
  ```

---

### 5.2 Resource-Level Per-User Scoping (`resource_access`)

Restricts a user's visibility so they can only view or manage specific items (e.g., Project #42 or Server #205).

#### A. Granting Specific Resource Access
```rust
resource_access_repo.grant_access(user_id, org_id, "PROJECT", 42).await?;
```
- **SQL Executed**:
  ```sql
  INSERT INTO resource_access (user_id, org_id, resource_type, resource_id)
  VALUES (?, ?, 'PROJECT', 42);
  ```

#### B. Checking Resource Access in Services
```rust
let allowed = resource_access_repo.check_access(user_id, org_id, "PROJECT", 42).await?;
if !allowed {
    return Err(PermissionError::Forbidden("Access to project 42 denied"));
}
```

---

### 5.3 Real-World Evaluation Matrix Example

Assume **User: Rahul (User ID: 5)** belongs to the **Developer** Group in **Org ID: 1**:

| Action / Resource | Group Role (Developer) | `user_policy` Override | `resource_access` Scope | Final Resolved Access |
| :--- | :--- | :--- | :--- | :--- |
| `applications:read` | ✅ ALLOW | *(None)* | All Projects | ✅ **ALLOWED** |
| `servers:delete` | ❌ BLOCKED | *(None)* | N/A | ❌ **BLOCKED** |
| `servers:reboot` | ✅ ALLOW | ❌ **`DENY`** | N/A | ❌ **BLOCKED (DENY Override Wins)** |
| `billing:view` | ❌ BLOCKED | ✅ **`GRANT`** | N/A | ✅ **ALLOWED (GRANT Override Wins)** |
| `project:access` | ✅ ALLOW | *(None)* | Only `PROJECT #42` | 🔒 **RESTRICTED to Project 42 Only** |

---

## 6. Code Integration & Middleware Guards

### 6.1 Route Guard Usage in Axum Controllers

Endpoints require permissions using Axum extractor guards:

```rust
#[get("/server/{id}")]
async fn get_server_metrics(
    &self,
    RequirePermission(_claims, permission): RequirePermission<Server, CanMonitor>,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    // Permission guard automatically verified:
    // 1. JWT Claims & Auth token
    // 2. Resolved Organization ID
    // 3. Checked action "server:monitor"
}
```

### 6.2 Axum Extractor Flow (`src/core/middleware/permission/extractor.rs`)

1. Extract `Claims` from JWT Authorization header.
2. Resolve `x-organization-id` header or fallback to user's primary organization.
3. Format Policy Action string: `PolicyAction::new(Resource::NAME, Operation::NAME)` -> `"server:monitor"`.
4. Call `PermissionService::check_permission(user_id, org_id, action)`.
5. If allowed, inject `PermissionOrganization(org_id)` into request extensions and proceed.
6. If denied, abort immediately with `HTTP 403 Forbidden ("Permission Denied: server:monitor")`.
