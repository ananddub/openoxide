use auto_di::singleton;
use std::sync::Arc;

use crate::db::queries::FlashQueries;

use super::types::{OverviewBackupItemDto, OverviewDomainItemDto, OverviewServiceItemDto};

pub struct OverviewService {
    queries: Arc<FlashQueries>,
}

#[singleton]
impl OverviewService {
    fn new(queries: Arc<FlashQueries>) -> Self {
        Self { queries }
    }

    pub async fn get_all_services(
        &self,
        organization_id: i64,
    ) -> sqlx::Result<Vec<OverviewServiceItemDto>> {
        let rows = self.queries.overview_list_services(organization_id).await?;
        Ok(rows
            .into_iter()
            .map(|row| {
                let service_type = match row.service_type.as_str() {
                    "APPLICATION" => "APP",
                    "COMPOSE" => "COMPOSE",
                    _ => "DATABASE",
                };
                OverviewServiceItemDto {
                    id: row.id,
                    name: row.name,
                    service_type: service_type.into(),
                    status: row.status,
                    created_at: row.created_at,
                    project_id: row.project_id,
                    project_name: row.project_name,
                    environment_id: row.environment_id,
                    environment_name: row.environment_name,
                    db_kind: row.db_kind.map(|kind| kind.to_lowercase()),
                    server_id: row.server_id,
                    server_name: row.server_name,
                }
            })
            .collect())
    }

    pub async fn get_all_domains(
        &self,
        organization_id: i64,
    ) -> sqlx::Result<Vec<OverviewDomainItemDto>> {
        let rows = self.queries.overview_list_domains(organization_id).await?;
        Ok(rows
            .into_iter()
            .map(|r| OverviewDomainItemDto {
                id: r.id,
                host: r.host,
                path: r.path,
                port: r.port.map(|p| p as i32),
                https: r.https != 0,
                application_id: (r.service_type == "APPLICATION").then_some(r.service_id).flatten(),
                compose_id: (r.service_type == "COMPOSE").then_some(r.service_id).flatten(),
                service_name: r
                    .service_name
                    .unwrap_or_else(|| "Service".into()),
                project_name: r
                    .project_name
                    .unwrap_or_else(|| "Project".into()),
            })
            .collect())
    }

    pub async fn get_all_backups(
        &self,
        organization_id: i64,
    ) -> sqlx::Result<Vec<OverviewBackupItemDto>> {
        let db_backups = self.queries.overview_list_backups(organization_id).await?;

        let mut items = Vec::new();
        for b in db_backups {
            items.push(OverviewBackupItemDto {
                id: b.id,
                name: b.name,
                backup_type: "DATABASE".into(),
                status: if b.enabled != 0 {
                    "ENABLED".into()
                } else {
                    "DISABLED".into()
                },
                destination: format!("Destination #{}", b.destination_id),
                created_at: b.created_at,
            });
        }

        let vol_backups = self
            .queries
            .overview_list_volume_backups(organization_id)
            .await?;

        for b in vol_backups {
            items.push(OverviewBackupItemDto {
                id: b.id.unwrap_or_default(),
                name: b.name.unwrap_or_default(),
                backup_type: "VOLUME".into(),
                status: if b.enabled.unwrap_or_default() != 0 {
                    "ENABLED".into()
                } else {
                    "DISABLED".into()
                },
                destination: format!(
                    "Destination #{}",
                    b.destination_id.unwrap_or_default()
                ),
                created_at: b.created_at.unwrap_or_default(),
            });
        }

        Ok(items)
    }
}
