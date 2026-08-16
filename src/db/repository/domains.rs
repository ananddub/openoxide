use crate::db::models::domains::Domain;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct DomainRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl DomainRepository {
    pub async fn route_in_use(
        &self,
        host: &str,
        path: &str,
        exclude_id: Option<i64>,
    ) -> Result<bool, sqlx::Error> {
        Ok(sqlx::query_scalar!(
            r#"SELECT COUNT(*) AS "count!: i64"
               FROM domains
               WHERE lower(trim(host, '.')) = lower(trim(?, '.'))
                 AND COALESCE(NULLIF(rtrim(path, '/'), ''), '/') = ?
                 AND (? IS NULL OR id != ?)"#,
            host,
            path,
            exclude_id,
            exclude_id
        )
        .fetch_one(self.pool.as_ref())
        .await?
            > 0)
    }

    pub async fn host_in_use(&self, host: &str) -> Result<bool, sqlx::Error> {
        Ok(sqlx::query_scalar!(
            r#"SELECT COUNT(*) AS "count!: i64" FROM domains WHERE lower(host) = lower(?)"#,
            host
        )
        .fetch_one(self.pool.as_ref())
        .await?
            > 0)
    }
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<Domain>, sqlx::Error> {
        sqlx::query_as!(
            Domain,
            r#"SELECT id AS "id?: i64", host AS "host: String", https AS "https: i64", port AS "port?: i64", path AS "path?: String", internal_path AS "internal_path?: String", custom_entrypoint AS "custom_entrypoint?: String", service_name AS "service_name?: String", custom_cert_resolver AS "custom_cert_resolver?: String", strip_path AS "strip_path: i64", middlewares AS "middlewares: String", domain_type AS "domain_type: String", certificate_type AS "certificate_type: String", application_id AS "application_id?: i64", compose_id AS "compose_id?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM domains"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Domain>, sqlx::Error> {
        sqlx::query_as!(
            Domain,
            r#"SELECT id AS "id?: i64", host AS "host: String", https AS "https: i64", port AS "port?: i64", path AS "path?: String", internal_path AS "internal_path?: String", custom_entrypoint AS "custom_entrypoint?: String", service_name AS "service_name?: String", custom_cert_resolver AS "custom_cert_resolver?: String", strip_path AS "strip_path: i64", middlewares AS "middlewares: String", domain_type AS "domain_type: String", certificate_type AS "certificate_type: String", application_id AS "application_id?: i64", compose_id AS "compose_id?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM domains WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &Domain) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO domains (host, https, port, path, internal_path, custom_entrypoint, service_name, custom_cert_resolver, strip_path, middlewares, domain_type, certificate_type, application_id, compose_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            &item.host,
            item.https,
            item.port,
            &item.path,
            &item.internal_path,
            &item.custom_entrypoint,
            &item.service_name,
            &item.custom_cert_resolver,
            item.strip_path,
            &item.middlewares,
            &item.domain_type,
            &item.certificate_type,
            item.application_id,
            item.compose_id,
            item.created_at,
            item.updated_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &Domain) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE domains SET host = ?, https = ?, port = ?, path = ?, internal_path = ?, custom_entrypoint = ?, service_name = ?, custom_cert_resolver = ?, strip_path = ?, middlewares = ?, domain_type = ?, certificate_type = ?, application_id = ?, compose_id = ?, created_at = ?, updated_at = ? WHERE id = ?"#,
            &item.host,
            item.https,
            item.port,
            &item.path,
            &item.internal_path,
            &item.custom_entrypoint,
            &item.service_name,
            &item.custom_cert_resolver,
            item.strip_path,
            &item.middlewares,
            &item.domain_type,
            &item.certificate_type,
            item.application_id,
            item.compose_id,
            item.created_at,
            item.updated_at,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM domains WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn list_by_application(
        &self,
        application_id: i64,
    ) -> Result<Vec<crate::services::domain::DomainRecord>, sqlx::Error> {
        sqlx::query_as!(
            crate::services::domain::DomainRecord,
            r#"SELECT id AS "id!: i64", host, https, port, path, internal_path,
               custom_entrypoint, service_name, custom_cert_resolver, strip_path,
               middlewares, domain_type, certificate_type, application_id, compose_id,
               created_at, updated_at
               FROM domains
               WHERE application_id = ?
               ORDER BY created_at DESC, id DESC"#,
            application_id
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn list_by_compose(
        &self,
        compose_id: i64,
    ) -> Result<Vec<crate::services::domain::DomainRecord>, sqlx::Error> {
        sqlx::query_as!(
            crate::services::domain::DomainRecord,
            r#"SELECT id AS "id!: i64", host, https, port, path, internal_path,
               custom_entrypoint, service_name, custom_cert_resolver, strip_path,
               middlewares, domain_type, certificate_type, application_id, compose_id,
               created_at, updated_at
               FROM domains
               WHERE compose_id = ?
               ORDER BY created_at DESC, id DESC"#,
            compose_id
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn list_all(
        &self,
    ) -> Result<Vec<crate::services::domain::DomainRecord>, sqlx::Error> {
        sqlx::query_as!(
            crate::services::domain::DomainRecord,
            r#"SELECT id AS "id!: i64", host, https, port, path, internal_path,
               custom_entrypoint, service_name, custom_cert_resolver, strip_path,
               middlewares, domain_type, certificate_type, application_id, compose_id,
               created_at, updated_at
               FROM domains
               ORDER BY created_at DESC, id DESC"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn list_by_application_raw(
        &self,
        application_id: i64,
    ) -> Result<Vec<Domain>, sqlx::Error> {
        sqlx::query_as!(
            Domain,
            r#"SELECT id AS "id?: i64", host AS "host: String", https AS "https: i64", port AS "port?: i64", path AS "path?: String", internal_path AS "internal_path?: String", custom_entrypoint AS "custom_entrypoint?: String", service_name AS "service_name?: String", custom_cert_resolver AS "custom_cert_resolver?: String", strip_path AS "strip_path: i64", middlewares AS "middlewares: String", domain_type AS "domain_type: String", certificate_type AS "certificate_type: String", application_id AS "application_id?: i64", compose_id AS "compose_id?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64"
               FROM domains
               WHERE application_id = ?
               ORDER BY id"#,
            application_id
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn list_by_compose_raw(&self, compose_id: i64) -> Result<Vec<Domain>, sqlx::Error> {
        sqlx::query_as!(
            Domain,
            r#"SELECT id AS "id?: i64", host AS "host: String", https AS "https: i64", port AS "port?: i64", path AS "path?: String", internal_path AS "internal_path?: String", custom_entrypoint AS "custom_entrypoint?: String", service_name AS "service_name?: String", custom_cert_resolver AS "custom_cert_resolver?: String", strip_path AS "strip_path: i64", middlewares AS "middlewares: String", domain_type AS "domain_type: String", certificate_type AS "certificate_type: String", application_id AS "application_id?: i64", compose_id AS "compose_id?: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64"
               FROM domains
               WHERE compose_id = ?
               ORDER BY id"#,
            compose_id
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn create_and_return(
        &self,
        host: String,
        https: i64,
        port: Option<i64>,
        path: Option<String>,
        internal_path: Option<String>,
        custom_entrypoint: Option<String>,
        service_name: Option<String>,
        custom_cert_resolver: Option<String>,
        strip_path: i64,
        middlewares: String,
        domain_type: String,
        certificate_type: String,
        application_id: Option<i64>,
        compose_id: Option<i64>,
    ) -> Result<crate::services::domain::DomainRecord, sqlx::Error> {
        sqlx::query_as!(
            crate::services::domain::DomainRecord,
            r#"INSERT INTO domains
               (host, https, port, path, internal_path, custom_entrypoint, service_name,
                custom_cert_resolver, strip_path, middlewares, domain_type, certificate_type,
                application_id, compose_id)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
               RETURNING id AS "id!: i64", host, https, port, path, internal_path,
               custom_entrypoint, service_name, custom_cert_resolver, strip_path,
               middlewares, domain_type, certificate_type, application_id, compose_id,
               created_at, updated_at"#,
            host,
            https,
            port,
            path,
            internal_path,
            custom_entrypoint,
            service_name,
            custom_cert_resolver,
            strip_path,
            middlewares,
            domain_type,
            certificate_type,
            application_id,
            compose_id
        )
        .fetch_one(self.pool.as_ref())
        .await
    }

    pub async fn update_and_return(
        &self,
        id: i64,
        host: String,
        https: i64,
        port: Option<i64>,
        path: Option<String>,
        internal_path: Option<String>,
        custom_entrypoint: Option<String>,
        service_name: Option<String>,
        custom_cert_resolver: Option<String>,
        strip_path: i64,
        middlewares: String,
        certificate_type: String,
    ) -> Result<crate::services::domain::DomainRecord, sqlx::Error> {
        sqlx::query_as!(
            crate::services::domain::DomainRecord,
            r#"UPDATE domains SET
               host = ?, https = ?, port = ?, path = ?, internal_path = ?,
               custom_entrypoint = ?, service_name = ?, custom_cert_resolver = ?,
               strip_path = ?, middlewares = ?, certificate_type = ?
               WHERE id = ?
               RETURNING id AS "id!: i64", host, https, port, path, internal_path,
               custom_entrypoint, service_name, custom_cert_resolver, strip_path,
               middlewares, domain_type, certificate_type, application_id, compose_id,
               created_at, updated_at"#,
            host,
            https,
            port,
            path,
            internal_path,
            custom_entrypoint,
            service_name,
            custom_cert_resolver,
            strip_path,
            middlewares,
            certificate_type,
            id
        )
        .fetch_one(self.pool.as_ref())
        .await
    }

    pub async fn get_record_by_id(
        &self,
        id: i64,
    ) -> Result<Option<crate::services::domain::DomainRecord>, sqlx::Error> {
        sqlx::query_as!(
            crate::services::domain::DomainRecord,
            r#"SELECT id AS "id!: i64", host, https, port, path, internal_path,
               custom_entrypoint, service_name, custom_cert_resolver, strip_path,
               middlewares, domain_type, certificate_type, application_id, compose_id,
               created_at, updated_at
               FROM domains
               WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }
}
