use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

use crate::api::dto::overview::{
    OverviewBackupItemDto, OverviewDomainItemDto, OverviewServiceItemDto,
};

pub struct OverviewService {
    db: Arc<SqlitePool>,
}

#[singleton]
impl OverviewService {
    fn new(db: Arc<SqlitePool>) -> Self {
        Self { db }
    }

    pub async fn get_all_services(
        &self,
        organization_id: i64,
    ) -> sqlx::Result<Vec<OverviewServiceItemDto>> {
        let mut services = Vec::new();

        // 1. Applications
        let apps = sqlx::query!(
            r#"
            SELECT 
                a.id, 
                a.name, 
                a.app_status AS status, 
                a.created_at, 
                p.id AS project_id, 
                p.name AS project_name, 
                e.id AS environment_id, 
                e.name AS environment_name
            FROM applications a
            JOIN environments e ON a.environment_id = e.id
            JOIN projects p ON e.project_id = p.id
            WHERE p.organization_id = ?
            ORDER BY a.created_at DESC
            "#,
            organization_id
        )
        .fetch_all(self.pool_ref())
        .await?;

        for a in apps {
            services.push(OverviewServiceItemDto {
                id: a.id,
                name: a.name,
                service_type: "APP".into(),
                status: a.status,
                created_at: a.created_at,
                project_id: a.project_id,
                project_name: a.project_name,
                environment_id: a.environment_id,
                environment_name: a.environment_name,
                db_kind: None,
            });
        }

        // 2. Compose Stacks
        let composes = sqlx::query!(
            r#"
            SELECT 
                c.id, 
                c.name, 
                c.compose_status AS status, 
                c.created_at, 
                p.id AS project_id, 
                p.name AS project_name, 
                e.id AS environment_id, 
                e.name AS environment_name
            FROM compose_projects c
            JOIN environments e ON c.environment_id = e.id
            JOIN projects p ON e.project_id = p.id
            WHERE p.organization_id = ?
            ORDER BY c.created_at DESC
            "#,
            organization_id
        )
        .fetch_all(self.pool_ref())
        .await?;

        for c in composes {
            services.push(OverviewServiceItemDto {
                id: c.id,
                name: c.name,
                service_type: "COMPOSE".into(),
                status: c.status,
                created_at: c.created_at,
                project_id: c.project_id,
                project_name: c.project_name,
                environment_id: c.environment_id,
                environment_name: c.environment_name,
                db_kind: None,
            });
        }

        // 3. Postgres Databases
        let pg_dbs = sqlx::query!(
            r#"
            SELECT 
                d.id, 
                d.name, 
                d.app_status AS status, 
                d.created_at, 
                p.id AS project_id, 
                p.name AS project_name, 
                e.id AS environment_id, 
                e.name AS environment_name
            FROM postgres_dbs d
            JOIN environments e ON d.environment_id = e.id
            JOIN projects p ON e.project_id = p.id
            WHERE p.organization_id = ?
            ORDER BY d.created_at DESC
            "#,
            organization_id
        )
        .fetch_all(self.pool_ref())
        .await?;

        for db in pg_dbs {
            services.push(OverviewServiceItemDto {
                id: db.id,
                name: db.name,
                service_type: "DATABASE".into(),
                status: db.status,
                created_at: db.created_at,
                project_id: db.project_id,
                project_name: db.project_name,
                environment_id: db.environment_id,
                environment_name: db.environment_name,
                db_kind: Some("postgres".into()),
            });
        }

        // 4. MySQL Databases
        let my_dbs = sqlx::query!(
            r#"
            SELECT 
                d.id, 
                d.name, 
                d.app_status AS status, 
                d.created_at, 
                p.id AS project_id, 
                p.name AS project_name, 
                e.id AS environment_id, 
                e.name AS environment_name
            FROM mysql_dbs d
            JOIN environments e ON d.environment_id = e.id
            JOIN projects p ON e.project_id = p.id
            WHERE p.organization_id = ?
            ORDER BY d.created_at DESC
            "#,
            organization_id
        )
        .fetch_all(self.pool_ref())
        .await?;

        for db in my_dbs {
            services.push(OverviewServiceItemDto {
                id: db.id,
                name: db.name,
                service_type: "DATABASE".into(),
                status: db.status,
                created_at: db.created_at,
                project_id: db.project_id,
                project_name: db.project_name,
                environment_id: db.environment_id,
                environment_name: db.environment_name,
                db_kind: Some("mysql".into()),
            });
        }

        // 5. MariaDB Databases
        let maria_dbs = sqlx::query!(
            r#"
            SELECT 
                d.id, 
                d.name, 
                d.app_status AS status, 
                d.created_at, 
                p.id AS project_id, 
                p.name AS project_name, 
                e.id AS environment_id, 
                e.name AS environment_name
            FROM mariadb_dbs d
            JOIN environments e ON d.environment_id = e.id
            JOIN projects p ON e.project_id = p.id
            WHERE p.organization_id = ?
            ORDER BY d.created_at DESC
            "#,
            organization_id
        )
        .fetch_all(self.pool_ref())
        .await?;

        for db in maria_dbs {
            services.push(OverviewServiceItemDto {
                id: db.id,
                name: db.name,
                service_type: "DATABASE".into(),
                status: db.status,
                created_at: db.created_at,
                project_id: db.project_id,
                project_name: db.project_name,
                environment_id: db.environment_id,
                environment_name: db.environment_name,
                db_kind: Some("mariadb".into()),
            });
        }

        // 6. Mongo Databases
        let mongo_dbs = sqlx::query!(
            r#"
            SELECT 
                d.id, 
                d.name, 
                d.app_status AS status, 
                d.created_at, 
                p.id AS project_id, 
                p.name AS project_name, 
                e.id AS environment_id, 
                e.name AS environment_name
            FROM mongo_dbs d
            JOIN environments e ON d.environment_id = e.id
            JOIN projects p ON e.project_id = p.id
            WHERE p.organization_id = ?
            ORDER BY d.created_at DESC
            "#,
            organization_id
        )
        .fetch_all(self.pool_ref())
        .await?;

        for db in mongo_dbs {
            services.push(OverviewServiceItemDto {
                id: db.id,
                name: db.name,
                service_type: "DATABASE".into(),
                status: db.status,
                created_at: db.created_at,
                project_id: db.project_id,
                project_name: db.project_name,
                environment_id: db.environment_id,
                environment_name: db.environment_name,
                db_kind: Some("mongo".into()),
            });
        }

        // 7. Redis Databases
        let redis_dbs = sqlx::query!(
            r#"
            SELECT 
                d.id, 
                d.name, 
                d.app_status AS status, 
                d.created_at, 
                p.id AS project_id, 
                p.name AS project_name, 
                e.id AS environment_id, 
                e.name AS environment_name
            FROM redis_dbs d
            JOIN environments e ON d.environment_id = e.id
            JOIN projects p ON e.project_id = p.id
            WHERE p.organization_id = ?
            ORDER BY d.created_at DESC
            "#,
            organization_id
        )
        .fetch_all(self.pool_ref())
        .await?;

        for db in redis_dbs {
            services.push(OverviewServiceItemDto {
                id: db.id,
                name: db.name,
                service_type: "DATABASE".into(),
                status: db.status,
                created_at: db.created_at,
                project_id: db.project_id,
                project_name: db.project_name,
                environment_id: db.environment_id,
                environment_name: db.environment_name,
                db_kind: Some("redis".into()),
            });
        }

        Ok(services)
    }

    pub async fn get_all_domains(
        &self,
        organization_id: i64,
    ) -> sqlx::Result<Vec<OverviewDomainItemDto>> {
        let rows = sqlx::query!(
            r#"
            SELECT 
                dom.id, 
                dom.host, 
                dom.path, 
                dom.port, 
                dom.https, 
                dom.application_id, 
                dom.compose_id,
                COALESCE(a.name, c.name, 'Service') AS service_name,
                COALESCE(pa.name, pc.name, 'Project') AS project_name
            FROM domains dom
            LEFT JOIN applications a ON dom.application_id = a.id
            LEFT JOIN environments ea ON a.environment_id = ea.id
            LEFT JOIN projects pa ON ea.project_id = pa.id
            LEFT JOIN compose_projects c ON dom.compose_id = c.id
            LEFT JOIN environments ec ON c.environment_id = ec.id
            LEFT JOIN projects pc ON ec.project_id = pc.id
            WHERE pa.organization_id = ? OR pc.organization_id = ?
            ORDER BY dom.created_at DESC
            "#,
            organization_id,
            organization_id
        )
        .fetch_all(self.pool_ref())
        .await?;

        let items = rows
            .into_iter()
            .map(|r| OverviewDomainItemDto {
                id: r.id,
                host: r.host,
                path: r.path,
                port: r.port.map(|p| p as i32),
                https: r.https != 0,
                application_id: r.application_id,
                compose_id: r.compose_id,
                service_name: r.service_name,
                project_name: r.project_name,
            })
            .collect();

        Ok(items)
    }

    pub async fn get_all_backups(
        &self,
        organization_id: i64,
    ) -> sqlx::Result<Vec<OverviewBackupItemDto>> {
        let db_backups = sqlx::query!(
            r#"
            SELECT 
                b.id, 
                b.app_name AS name, 
                b.enabled, 
                b.destination_id, 
                b.created_at
            FROM backups b
            WHERE b.organization_id = ?
            "#,
            organization_id
        )
        .fetch_all(self.pool_ref())
        .await?;

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

        let vol_backups = sqlx::query!(
            r#"
            SELECT 
                vb.id, 
                vb.name, 
                vb.enabled, 
                vb.destination_id, 
                vb.created_at
            FROM volume_backups vb
            WHERE vb.organization_id = ?
            "#,
            organization_id
        )
        .fetch_all(self.pool_ref())
        .await?;

        for b in vol_backups {
            items.push(OverviewBackupItemDto {
                id: b.id,
                name: b.name,
                backup_type: "VOLUME".into(),
                status: if b.enabled != 0 {
                    "ENABLED".into()
                } else {
                    "DISABLED".into()
                },
                destination: format!("Destination #{}", b.destination_id),
                created_at: b.created_at,
            });
        }

        Ok(items)
    }

    fn pool_ref(&self) -> &SqlitePool {
        self.db.as_ref()
    }
}
