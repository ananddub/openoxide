use super::{
    DatabaseKind, DatabaseRecord, DatabaseService,
    queries::{generate_app_name, random_secret, slug_value},
};
use crate::api::dto::database::{CreateDatabaseDto, PatchDatabaseDto};

impl DatabaseService {
    pub async fn list_by_environment(
        &self,
        environment_id: i64,
    ) -> sqlx::Result<Vec<DatabaseRecord>> {
        self.repo_postgres
            .list_all_by_environment(environment_id)
            .await
    }

    pub async fn get_by_id(&self, kind: DatabaseKind, id: i64) -> sqlx::Result<DatabaseRecord> {
        match kind {
            DatabaseKind::Postgres => self.repo_postgres.get_by_id(id).await,
            DatabaseKind::Mysql => self.repo_mysql.get_by_id(id).await,
            DatabaseKind::Mariadb => self.repo_mariadb.get_by_id(id).await,
            DatabaseKind::Mongo => self.repo_mongo.get_by_id(id).await,
            DatabaseKind::Redis => self.repo_redis.get_by_id(id).await,
            DatabaseKind::Libsql => self.repo_libsql.get_by_id(id).await,
        }
    }

    pub async fn create(
        &self,
        kind: DatabaseKind,
        input: CreateDatabaseDto,
    ) -> sqlx::Result<DatabaseRecord> {
        let app_name = generate_app_name(&input.name, kind.as_str());
        let image = input
            .docker_image
            .clone()
            .unwrap_or_else(|| kind.default_image().into());
        let db_name = input
            .database_name
            .clone()
            .unwrap_or_else(|| slug_value(&input.name));
        let db_user = input
            .database_user
            .clone()
            .unwrap_or_else(|| "rustploy".into());
        let db_password = input
            .database_password
            .clone()
            .unwrap_or_else(random_secret);
        let root_password = input
            .database_root_password
            .clone()
            .unwrap_or_else(random_secret);

        match kind {
            DatabaseKind::Postgres => {
                self.repo_postgres
                    .create(&input, &app_name, &image, &db_name, &db_user, &db_password)
                    .await?;
            }
            DatabaseKind::Mysql => {
                self.repo_mysql
                    .create(
                        &input,
                        &app_name,
                        &image,
                        &db_name,
                        &db_user,
                        &db_password,
                        &root_password,
                    )
                    .await?;
            }
            DatabaseKind::Mariadb => {
                self.repo_mariadb
                    .create(
                        &input,
                        &app_name,
                        &image,
                        &db_name,
                        &db_user,
                        &db_password,
                        &root_password,
                    )
                    .await?;
            }
            DatabaseKind::Mongo => {
                self.repo_mongo
                    .create(&input, &app_name, &image, &db_user, &db_password)
                    .await?;
            }
            DatabaseKind::Redis => {
                self.repo_redis
                    .create(&input, &app_name, &image, &db_password)
                    .await?;
            }
            DatabaseKind::Libsql => {
                self.repo_libsql
                    .create(&input, &app_name, &image, &db_user, &db_password)
                    .await?;
            }
        }

        let id = sqlx::query_scalar!("SELECT last_insert_rowid() AS \"id!: i64\"")
            .fetch_one(self.db.as_ref())
            .await?;
        self.get_by_id(kind, id).await
    }

    pub async fn patch(
        &self,
        kind: DatabaseKind,
        id: i64,
        input: PatchDatabaseDto,
    ) -> sqlx::Result<DatabaseRecord> {
        match kind {
            DatabaseKind::Postgres => self.repo_postgres.update(id, &input).await?,
            DatabaseKind::Mysql => self.repo_mysql.update(id, &input).await?,
            DatabaseKind::Mariadb => self.repo_mariadb.update(id, &input).await?,
            DatabaseKind::Mongo => self.repo_mongo.update(id, &input).await?,
            DatabaseKind::Redis => self.repo_redis.update(id, &input).await?,
            DatabaseKind::Libsql => self.repo_libsql.update(id, &input).await?,
        }

        self.get_by_id(kind, id).await
    }

    pub async fn delete(&self, kind: DatabaseKind, id: i64) -> sqlx::Result<()> {
        self.get_by_id(kind, id).await?;
        match kind {
            DatabaseKind::Postgres => self.repo_postgres.delete(id).await?,
            DatabaseKind::Mysql => self.repo_mysql.delete(id).await?,
            DatabaseKind::Mariadb => self.repo_mariadb.delete(id).await?,
            DatabaseKind::Mongo => self.repo_mongo.delete(id).await?,
            DatabaseKind::Redis => self.repo_redis.delete(id).await?,
            DatabaseKind::Libsql => self.repo_libsql.delete(id).await?,
        }
        Ok(())
    }
}
