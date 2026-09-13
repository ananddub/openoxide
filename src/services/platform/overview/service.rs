use auto_di::singleton;
use std::sync::Arc;

use crate::db::queries::FlashQueries;

use super::types::{
    BackupLinkage, OverviewBackupItemDto, OverviewDomainItemDto, OverviewServiceItemDto,
};

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
                    .filter(|value| !value.trim().is_empty())
                    .unwrap_or_else(|| "Service".into()),
                project_name: r.project_name.unwrap_or_else(|| "Project".into()),
                internal_path: r.internal_path,
                custom_entrypoint: r.custom_entrypoint,
                custom_cert_resolver: r.custom_cert_resolver,
                strip_path: r.strip_path != 0,
                middlewares: r.middlewares,
                domain_type: r.domain_type,
                certificate_type: r.certificate_type,
                domain_service_name: r.domain_service_name,
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
            // DATABASE rows carry their kind in service_database.db_type;
            // APPLICATION/COMPOSE rows key off the joined service_type.
            let kind = if b.service_type == "DATABASE" {
                b.db_type.as_deref().unwrap_or_default()
            } else {
                b.service_type.as_str()
            };
            let linkage = backup_linkage(kind, b.service_id);
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
                linkage,
                created_at: b.created_at,
            });
        }

        let vol_backups = self
            .queries
            .overview_list_volume_backups(organization_id)
            .await?;

        for b in vol_backups {
            let linkage = backup_linkage(&b.service_type.unwrap_or_default(), b.service_id);
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
                linkage,
                created_at: b.created_at.unwrap_or_default(),
            });
        }

        Ok(items)
    }
}

/// Expands one (kind, service_id) pair into the typed linkage fields the
/// detail-page tabs filter on. Unknown kinds leave every mirror unset; the
/// app tab still matches those by app_name.
fn backup_linkage(kind: &str, service_id: Option<i64>) -> BackupLinkage {
    let upper = kind.to_ascii_uppercase();
    let kind = match upper.as_str() {
        "STANDALONE" => "APPLICATION",
        "MONGODB" => "MONGO",
        other => other,
    };
    let id = service_id.filter(|id| *id > 0);
    let on = |target: &str| id.filter(|_| kind == target);
    BackupLinkage {
        service_id: id,
        application_id: on("APPLICATION"),
        compose_id: on("COMPOSE"),
        postgres_id: on("POSTGRES"),
        mysql_id: on("MYSQL"),
        mariadb_id: on("MARIADB"),
        mongo_id: on("MONGO"),
        redis_id: on("REDIS"),
        libsql_id: on("LIBSQL"),
    }
}
