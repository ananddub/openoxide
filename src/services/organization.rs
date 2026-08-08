use std::sync::Arc;

use auto_di::singleton;
use sqlx::SqlitePool;

use crate::{
    api::dto::organization::{CreateOrganizationDto, PatchOrganizationDto},
    db::models::organization::Organization,
    repository::{OrganizationMemberRepository, OrganizationRepository, PermissionGroupRepository},
};

pub struct OrganizationService {
    db: Arc<SqlitePool>,
    repo_org: Arc<OrganizationRepository>,
    repo_member: Arc<OrganizationMemberRepository>,
    repo_permissions: Arc<PermissionGroupRepository>,
}

#[singleton]
impl OrganizationService {
    fn new(
        db: Arc<SqlitePool>,
        repo_org: Arc<OrganizationRepository>,
        repo_member: Arc<OrganizationMemberRepository>,
        repo_permissions: Arc<PermissionGroupRepository>,
    ) -> Self {
        Self {
            db,
            repo_org,
            repo_member,
            repo_permissions,
        }
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<Organization> {
        self.repo_org
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn list_by_owner(&self, owner_id: i64) -> sqlx::Result<Vec<Organization>> {
        self.repo_org.list_by_owner(owner_id).await
    }

    pub async fn create(
        &self,
        owner_id: i64,
        input: CreateOrganizationDto,
    ) -> sqlx::Result<Organization> {
        let slug = normalize_slug(input.slug.as_deref().unwrap_or(&input.name));
        if slug.is_empty() {
            return Err(sqlx::Error::Protocol(
                "organization slug cannot be empty".into(),
            ));
        }

        let mut tx = self.db.begin().await?;
        let organization = self
            .repo_org
            .create_in_transaction(&mut tx, input.name, input.logo, slug, owner_id)
            .await?;

        let org_id = organization
            .id
            .ok_or_else(|| sqlx::Error::Protocol("missing organization id".into()))?;
        self.repo_member
            .add_member_in_transaction(&mut tx, "ADMIN", owner_id, org_id)
            .await?;

        tx.commit().await?;
        self.repo_permissions
            .seed_organization_defaults(org_id)
            .await?;
        Ok(organization)
    }

    pub async fn update(&self, id: i64, input: PatchOrganizationDto) -> sqlx::Result<Organization> {
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

        self.repo_org.update_and_return(id, name, logo, slug).await
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.get_by_id(id).await?;
        self.repo_org.delete(id).await
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
        sqlx::query("CREATE TABLE organization_members (id INTEGER PRIMARY KEY AUTOINCREMENT, role TEXT DEFAULT 'MEMBER', user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE, organization_id INTEGER NOT NULL REFERENCES organization(id) ON DELETE CASCADE, created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')), updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))) STRICT").execute(&pool).await.unwrap();
        sqlx::query("CREATE TABLE groups (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL, is_system INTEGER NOT NULL DEFAULT 0, organization_id INTEGER, created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')), updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))) STRICT").execute(&pool).await.unwrap();
        sqlx::query("CREATE TABLE policy (id INTEGER PRIMARY KEY AUTOINCREMENT, action TEXT NOT NULL UNIQUE) STRICT").execute(&pool).await.unwrap();
        sqlx::query("CREATE TABLE group_policy (group_id INTEGER NOT NULL, policy_id INTEGER NOT NULL, UNIQUE(group_id, policy_id)) STRICT").execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO users (id) VALUES (7)")
            .execute(&pool)
            .await
            .unwrap();

        let db = Arc::new(pool);
        let service = OrganizationService {
            db: db.clone(),
            repo_org: Arc::new(OrganizationRepository::new(db.clone())),
            repo_member: Arc::new(OrganizationMemberRepository::new(db.clone())),
            repo_permissions: Arc::new(PermissionGroupRepository::new(db.clone())),
        };

        let organization = service
            .create(
                7,
                CreateOrganizationDto {
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
        .fetch_one(service.db.as_ref())
        .await
        .unwrap();
        assert_eq!(role, "ADMIN");
    }
}
