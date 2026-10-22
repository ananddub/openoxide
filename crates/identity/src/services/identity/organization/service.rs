use std::sync::Arc;

use auto_di::singleton;

use crate::{
    api::connect::generated::openoxide::api::v1::{
        CreateOrganizationRequest, UpdateOrganizationRequest,
    },
    db::queries::FlashQueries,
    flash_gen::{
        models::Organization,
        organization_members::OrganizationMembersAddMemberInTransactionParams,
        organizations::{CreateOrganizationParams, UpdateOrganizationParams},
    },
};

pub struct OrganizationService {
    queries: Arc<FlashQueries>,
}

#[singleton]
impl OrganizationService {
    fn new(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<Organization> {
        self.queries.get_organization_by_i_d(id).await
    }

    pub async fn list_by_owner(&self, owner_id: i64) -> sqlx::Result<Vec<Organization>> {
        self.queries.list_organizations_by_owner(owner_id).await
    }

    /// Whether the user owns or joined the organization.
    pub async fn is_member(&self, organization_id: i64, user_id: i64) -> sqlx::Result<bool> {
        if self
            .queries
            .organization_members_role(user_id, organization_id)
            .await
            .ok()
            .flatten()
            .is_some()
        {
            return Ok(true);
        }
        match self.get_by_id(organization_id).await {
            Ok(org) => Ok(org.owner_id == user_id),
            Err(sqlx::Error::RowNotFound) => Ok(false),
            Err(error) => Err(error),
        }
    }

    /// Self-service exit: a member removes their own membership. Owners
    /// must delete the organization instead (their membership row may not
    /// even exist, and leaving an orphaned workspace is never allowed).
    pub async fn leave(&self, organization_id: i64, user_id: i64) -> sqlx::Result<()> {
        let org = self.get_by_id(organization_id).await?;
        if org.owner_id == user_id {
            return Err(sqlx::Error::Protocol(
                "the organization owner cannot leave; delete the organization instead".into(),
            ));
        }
        self.queries
            .organization_members_remove_member(user_id, organization_id)
            .await
    }

    /// Every organization the user can act inside — owned or joined via
    /// membership — with the caller's effective role per organization.
    /// Owned organizations always report OWNER even when a (stale) member
    /// row carries a lesser role.
    pub async fn list_for_user(&self, user_id: i64) -> sqlx::Result<Vec<(Organization, String)>> {
        let owned = self.queries.list_organizations_by_owner(user_id).await?;
        let joined = self.queries.list_organizations_by_member(user_id).await?;
        let mut result: Vec<(Organization, String)> = owned
            .into_iter()
            .map(|org| (org, "OWNER".to_string()))
            .collect();
        let seen: std::collections::HashSet<i64> = result.iter().map(|(org, _)| org.id).collect();
        for row in joined {
            if seen.contains(&row.id) {
                continue;
            }
            let role = row.member_role.clone().unwrap_or_else(|| "MEMBER".into());
            result.push((
                Organization {
                    id: row.id,
                    name: row.name,
                    logo: row.logo,
                    slug: row.slug,
                    owner_id: row.owner_id,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                },
                role,
            ));
        }
        result.sort_by_key(|(org, _)| std::cmp::Reverse(org.created_at));
        Ok(result)
    }

    pub async fn create(
        &self,
        owner_id: i64,
        input: CreateOrganizationRequest,
    ) -> sqlx::Result<Organization> {
        let slug = normalize_slug(input.slug.as_deref().unwrap_or(&input.name));
        if slug.is_empty() {
            return Err(sqlx::Error::Protocol(
                "organization slug cannot be empty".into(),
            ));
        }

        let organization = self
            .queries
            .create_organization(&CreateOrganizationParams {
                name: input.name,
                logo: input.logo.unwrap_or_default(),
                slug,
                owner_id,
            })
            .await?;

        self.queries
            .organization_members_add_member_in_transaction(
                &OrganizationMembersAddMemberInTransactionParams {
                    role: "ADMIN".into(),
                    user_id: owner_id,
                    organization_id: organization.id,
                },
            )
            .await?;

        self.seed_organization_defaults(organization.id).await?;
        Ok(organization)
    }

    pub async fn update(
        &self,
        id: i64,
        input: UpdateOrganizationRequest,
    ) -> sqlx::Result<Organization> {
        let current = self.get_by_id(id).await?;
        let name = input.name.unwrap_or(current.name);
        let logo = input.logo.or(current.logo);
        let slug = input
            .slug
            .map(|value| normalize_slug(&value))
            .unwrap_or(current.slug);
        if slug.is_empty() {
            return Err(sqlx::Error::Protocol(
                "organization slug cannot be empty".into(),
            ));
        }

        self.queries
            .update_organization(&UpdateOrganizationParams {
                name,
                logo: logo.unwrap_or_default(),
                slug,
                id,
            })
            .await
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.get_by_id(id).await?;
        self.queries.delete_organization(id).await
    }

    async fn seed_organization_defaults(&self, organization_id: i64) -> sqlx::Result<i64> {
        let hit = self
            .queries
            .permission_groups_admin_group_exists(organization_id)
            .await?;
        if hit > 0
            && let Some(existing) = self
                .queries
                .list_groups_for_organization(organization_id)
                .await?
                .into_iter()
                .find(|group| group.name == "Organization Admin")
        {
            return Ok(existing.id);
        }
        let admin = self
            .queries
            .permission_groups_insert_admin_group(organization_id)
            .await?;
        let policies = self.queries.list_policies().await?;
        let crud_policy_ids = policies
            .into_iter()
            .filter(|policy| {
                [":read", ":create", ":update", ":delete"]
                    .iter()
                    .any(|suffix| policy.action.ends_with(suffix))
            })
            .map(|policy| policy.id);
        for policy_id in crud_policy_ids {
            self.queries
                .permission_groups_add_policy_to_group(&admin.to_string(), &policy_id.to_string())
                .await?;
        }
        Ok(admin)
    }
}

fn normalize_slug(value: &str) -> String {
    value
        .trim()
        .to_ascii_lowercase()
        .chars()
        .fold((String::new(), false), |(mut output, separator), ch| {
            if ch.is_ascii_alphanumeric() {
                output.push(ch);
                (output, false)
            } else if !output.is_empty() && !separator {
                output.push('-');
                (output, true)
            } else {
                (output, separator)
            }
        })
        .0
        .trim_end_matches('-')
        .to_owned()
}

#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqlitePoolOptions;

    use super::*;

    #[tokio::test]
    async fn create_normalizes_slug_and_adds_owner_as_admin() {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("CREATE TABLE users (id INTEGER PRIMARY KEY)")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("CREATE TABLE organization (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL UNIQUE, logo TEXT, slug TEXT NOT NULL UNIQUE, owner_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE, created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')), updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))) STRICT").execute(&pool).await.unwrap();
        sqlx::query("CREATE TABLE organization_members (id INTEGER PRIMARY KEY AUTOINCREMENT, role TEXT DEFAULT 'MEMBER', user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE, organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE, created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')), updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')), group_id INTEGER) STRICT").execute(&pool).await.unwrap();
        sqlx::query("CREATE TABLE groups (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL, is_system INTEGER NOT NULL DEFAULT 0, organization_id INTEGER, created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')), updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))) STRICT").execute(&pool).await.unwrap();
        sqlx::query("CREATE TABLE policy (id INTEGER PRIMARY KEY AUTOINCREMENT, action TEXT NOT NULL UNIQUE, created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))) STRICT").execute(&pool).await.unwrap();
        sqlx::query("CREATE TABLE group_policy (id INTEGER PRIMARY KEY AUTOINCREMENT, group_id INTEGER NOT NULL, policy_id INTEGER NOT NULL, created_at INTEGER NOT NULL DEFAULT (unixepoch()), UNIQUE(group_id, policy_id)) STRICT").execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO users (id) VALUES (7)")
            .execute(&pool)
            .await
            .unwrap();

        let db = Arc::new(pool);
        let service = OrganizationService {
            queries: Arc::new(FlashQueries::new(db.clone())),
        };

        let organization = service
            .create(
                7,
                CreateOrganizationRequest {
                    name: "My Cool Team".into(),
                    logo: None,
                    slug: None,
                },
            )
            .await
            .unwrap();

        assert_eq!(organization.slug, "my-cool-team");
        let role: String = sqlx::query_scalar(
            "SELECT role FROM organization_members WHERE organization_id = ? AND user_id = 7",
        )
        .bind(organization.id)
        .fetch_one(db.as_ref())
        .await
        .unwrap();
        assert_eq!(role, "ADMIN");
    }
}
