use auto_di::singleton;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    db::queries::FlashQueries,
    flash_gen::{models::Tags, tags::TagsCreateParams},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTagDto {
    pub name: String,
    pub color: String,
    pub organization_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTagDto {
    pub name: Option<String>,
    pub color: Option<String>,
}

#[derive(Clone)]
pub struct TagService {
    queries: Arc<FlashQueries>,
}

#[singleton]
impl TagService {
    fn new(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }

    pub async fn list_all(&self) -> sqlx::Result<Vec<Tags>> {
        self.queries.tags_get_all().await.map(|rows| {
            rows.into_iter()
                .map(|row| Tags {
                    id: row.id,
                    name: row.name,
                    color: row.color,
                    organization_id: row.organization_id,
                    created_at: row.created_at,
                })
                .collect()
        })
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<Option<Tags>> {
        match self.queries.tags_get_by_id(id).await {
            Ok(row) => Ok(Some(row)),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(error) => Err(error),
        }
    }

    pub async fn create(&self, dto: CreateTagDto) -> sqlx::Result<Tags> {
        let params = TagsCreateParams {
            name: dto.name,
            color: dto.color,
            organization_id: dto.organization_id.unwrap_or(1),
            created_at: chrono::Utc::now().timestamp(),
        };
        self.queries.tags_create(&params).await?;
        let id = self.queries.tags_get_last_insert_rowid().await?;
        self.get_by_id(id).await?.ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn update(&self, id: i64, dto: UpdateTagDto) -> sqlx::Result<Tags> {
        let existing = self.get_by_id(id).await?.ok_or(sqlx::Error::RowNotFound)?;
        let updated = Tags {
            id,
            name: dto.name.unwrap_or(existing.name),
            color: dto.color.unwrap_or(existing.color),
            organization_id: existing.organization_id,
            created_at: existing.created_at,
        };
        self.queries
            .tags_update(&crate::flash_gen::tags::TagsUpdateParams {
                name: updated.name.clone(),
                color: updated.color.clone(),
                organization_id: updated.organization_id,
                created_at: updated.created_at,
                id,
            })
            .await?;
        Ok(updated)
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.queries.tags_delete(id).await
    }

    pub async fn list_project_tags(&self, project_id: i64) -> sqlx::Result<Vec<Tags>> {
        self.queries.list_tags_by_project(project_id).await
    }

    pub async fn attach_project_tag(&self, project_id: i64, tag_id: i64) -> sqlx::Result<()> {
        self.queries.tags_add_to_project(project_id, tag_id).await
    }

    pub async fn detach_project_tag(&self, project_id: i64, tag_id: i64) -> sqlx::Result<()> {
        self.queries
            .tags_remove_from_project(project_id, tag_id)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::fixture;

    async fn service(tag: &str) -> (TagService, sqlx::SqlitePool) {
        let (queries, pool) = fixture(tag).await;
        (TagService { queries }, pool)
    }

    fn tag(name: &str, color: &str) -> CreateTagDto {
        CreateTagDto {
            name: name.into(),
            color: color.into(),
            organization_id: None,
        }
    }

    #[tokio::test]
    async fn create_round_trips_and_defaults_to_organization_one() {
        let (service, _pool) = service("tag-create").await;

        let created = service.create(tag("edge", "#0d9488")).await.unwrap();

        assert_eq!(created.name, "edge");
        assert_eq!(created.color, "#0d9488");
        assert_eq!(
            created.organization_id, 1,
            "a tag created without an organization must land in the default one"
        );
        assert!(
            created.created_at > 0,
            "created_at should be a real timestamp"
        );

        let read = service.get_by_id(created.id).await.unwrap().unwrap();
        assert_eq!(
            (
                &read.name,
                &read.color,
                read.organization_id,
                read.created_at
            ),
            (
                &created.name,
                &created.color,
                created.organization_id,
                created.created_at
            )
        );
        let listed = service.list_all().await.unwrap();
        assert_eq!(listed.len(), 1);
        assert_eq!((&listed[0].name, listed[0].id), (&created.name, created.id));
    }

    #[tokio::test]
    async fn update_changes_only_the_fields_that_were_sent() {
        let (service, _pool) = service("tag-update").await;
        let created = service.create(tag("edge", "#0d9488")).await.unwrap();

        let updated = service
            .update(
                created.id,
                UpdateTagDto {
                    name: None,
                    color: Some("#f59e0b".into()),
                },
            )
            .await
            .unwrap();

        assert_eq!(updated.name, "edge", "an absent name must keep its value");
        assert_eq!(updated.color, "#f59e0b");
        assert_eq!(updated.created_at, created.created_at);
        let read = service.get_by_id(created.id).await.unwrap().unwrap();
        assert_eq!(read.name, updated.name);
        assert_eq!(read.color, updated.color);
        assert_eq!(read.created_at, created.created_at);
    }

    #[tokio::test]
    async fn update_and_get_of_a_missing_tag_are_row_not_found() {
        let (service, _pool) = service("tag-missing").await;

        assert!(service.get_by_id(999).await.unwrap().is_none());
        let error = service
            .update(
                999,
                UpdateTagDto {
                    name: Some("ghost".into()),
                    color: None,
                },
            )
            .await
            .unwrap_err();
        assert!(matches!(error, sqlx::Error::RowNotFound), "{error}");
    }

    #[tokio::test]
    async fn delete_removes_the_row() {
        let (service, _pool) = service("tag-delete").await;
        let created = service.create(tag("edge", "#0d9488")).await.unwrap();

        service.delete(created.id).await.unwrap();

        assert!(service.get_by_id(created.id).await.unwrap().is_none());
        assert!(service.list_all().await.unwrap().is_empty());
    }

    /// Re-attaching an existing pair is a no-op: `project_tags` has a unique
    /// `(project_id, tag_id)` index and the insert ignores conflicts.
    #[tokio::test]
    async fn project_tags_attach_list_and_detach() {
        let (service, pool) = service("tag-project").await;
        sqlx::query(
            "INSERT INTO projects (name, env_var, organization_id, created_at, updated_at)
             VALUES ('site', '', 1, 0, 0)",
        )
        .execute(&pool)
        .await
        .unwrap();
        let edge = service.create(tag("edge", "#0d9488")).await.unwrap();
        let infra = service.create(tag("infra", "#6366f1")).await.unwrap();

        service.attach_project_tag(1, edge.id).await.unwrap();
        service.attach_project_tag(1, infra.id).await.unwrap();
        // Attaching the same pair twice must not duplicate the link.
        service.attach_project_tag(1, edge.id).await.unwrap();

        let attached = service.list_project_tags(1).await.unwrap();
        let mut names: Vec<_> = attached.iter().map(|tag| tag.name.as_str()).collect();
        names.sort();
        assert_eq!(names, ["edge", "infra"]);

        service.detach_project_tag(1, edge.id).await.unwrap();

        let remaining = service.list_project_tags(1).await.unwrap();
        assert_eq!(
            remaining.iter().map(|tag| &tag.name).collect::<Vec<_>>(),
            ["infra"]
        );
        // The tag itself survives losing its project link.
        assert!(service.get_by_id(edge.id).await.unwrap().is_some());
    }
}
