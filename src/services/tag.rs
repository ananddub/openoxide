use auto_di::singleton;
use std::sync::Arc;

use crate::api::dto::tag::{CreateTagDto, UpdateTagDto};
use crate::db::models::tags::Tag;
use crate::db::repository::project_tags::ProjectTagRepository;
use crate::db::repository::tags::TagRepository;

#[derive(Clone)]
pub struct TagService {
    repo_tag: Arc<TagRepository>,
    repo_project_tag: Arc<ProjectTagRepository>,
}

#[singleton]
impl TagService {
    fn new(repo_tag: Arc<TagRepository>, repo_project_tag: Arc<ProjectTagRepository>) -> Self {
        Self {
            repo_tag,
            repo_project_tag,
        }
    }

    pub async fn list_all(&self) -> sqlx::Result<Vec<Tag>> {
        self.repo_tag.get_all().await
    }

    pub async fn get_by_id(&self, id: i64) -> sqlx::Result<Option<Tag>> {
        self.repo_tag.get_by_id(id).await
    }

    pub async fn create(&self, dto: CreateTagDto) -> sqlx::Result<Tag> {
        let now = chrono::Utc::now().timestamp();
        let tag = Tag {
            id: None,
            name: dto.name,
            color: dto.color,
            organization_id: dto.organization_id.unwrap_or(1),
            created_at: now,
        };
        let id = self.repo_tag.create(&tag).await?;
        let created = self
            .repo_tag
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        Ok(created)
    }

    pub async fn update(&self, id: i64, dto: UpdateTagDto) -> sqlx::Result<Tag> {
        let existing = self
            .repo_tag
            .get_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;
        let updated = Tag {
            id: Some(id),
            name: dto.name.unwrap_or(existing.name),
            color: dto.color.unwrap_or(existing.color),
            organization_id: existing.organization_id,
            created_at: existing.created_at,
        };
        self.repo_tag.update(id, &updated).await?;
        Ok(updated)
    }

    pub async fn delete(&self, id: i64) -> sqlx::Result<()> {
        self.repo_tag.delete(id).await
    }

    pub async fn list_project_tags(&self, project_id: i64) -> sqlx::Result<Vec<Tag>> {
        let project_tags = self.repo_project_tag.get_all().await?;
        let matching_tag_ids: Vec<i64> = project_tags
            .into_iter()
            .filter(|pt| pt.project_id == project_id)
            .map(|pt| pt.tag_id)
            .collect();

        let all_tags = self.repo_tag.get_all().await?;
        let filtered = all_tags
            .into_iter()
            .filter(|t| t.id.map_or(false, |id| matching_tag_ids.contains(&id)))
            .collect();
        Ok(filtered)
    }

    pub async fn attach_project_tag(&self, project_id: i64, tag_id: i64) -> sqlx::Result<()> {
        let pt = crate::db::models::project_tags::ProjectTag {
            id: None,
            project_id,
            tag_id,
        };
        self.repo_project_tag.create(&pt).await?;
        Ok(())
    }

    pub async fn detach_project_tag(&self, project_id: i64, tag_id: i64) -> sqlx::Result<()> {
        let all_pt = self.repo_project_tag.get_all().await?;
        if let Some(target) = all_pt
            .into_iter()
            .find(|pt| pt.project_id == project_id && pt.tag_id == tag_id)
        {
            if let Some(id) = target.id {
                self.repo_project_tag.delete(id).await?;
            }
        }
        Ok(())
    }
}
