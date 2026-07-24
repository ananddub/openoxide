use crate::db::models::users::User;
use auto_di::singleton;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct UserRepository {
    pool: Arc<SqlitePool>,
}

#[singleton]
impl UserRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn get_all(&self) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"SELECT id AS "id?: i64", email AS "email?: String", last_name AS "last_name?: String", first_name AS "first_name?: String", avatar AS "avatar: String", role AS "role?: String", about_me AS "about_me?: String", password AS "password: String", is_email_verify AS "is_email_verify?: i64", email_verify_at AS "email_verify_at?: i64", two_factor_enable AS "two_factor_enable?: i64", is_registered AS "is_registered: i64", added_by AS "added_by?: i64", group_id AS "group_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM users"#
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"SELECT id AS "id?: i64", email AS "email?: String", last_name AS "last_name?: String", first_name AS "first_name?: String", avatar AS "avatar: String", role AS "role?: String", about_me AS "about_me?: String", password AS "password: String", is_email_verify AS "is_email_verify?: i64", email_verify_at AS "email_verify_at?: i64", two_factor_enable AS "two_factor_enable?: i64", is_registered AS "is_registered: i64", added_by AS "added_by?: i64", group_id AS "group_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64" FROM users WHERE id = ?"#,
            id
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }

    pub async fn create(&self, item: &User) -> Result<i64, sqlx::Error> {
        let _res = sqlx::query!(
            r#"INSERT INTO users (email, last_name, first_name, avatar, role, about_me, password, is_email_verify, email_verify_at, two_factor_enable, is_registered, added_by, group_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            &item.email,
            &item.last_name,
            &item.first_name,
            &item.avatar,
            &item.role,
            &item.about_me,
            &item.password,
            item.is_email_verify,
            item.email_verify_at,
            item.two_factor_enable,
            item.is_registered,
            item.added_by,
            item.group_id,
            item.created_at,
            item.updated_at
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(_res.last_insert_rowid())
    }

    pub async fn update(&self, id: i64, item: &User) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE users SET email = ?, last_name = ?, first_name = ?, avatar = ?, role = ?, about_me = ?, password = ?, is_email_verify = ?, email_verify_at = ?, two_factor_enable = ?, is_registered = ?, added_by = ?, group_id = ?, created_at = ?, updated_at = ? WHERE id = ?"#,
            &item.email,
            &item.last_name,
            &item.first_name,
            &item.avatar,
            &item.role,
            &item.about_me,
            &item.password,
            item.is_email_verify,
            item.email_verify_at,
            item.two_factor_enable,
            item.is_registered,
            item.added_by,
            item.group_id,
            item.created_at,
            item.updated_at,
            id
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(r#"DELETE FROM users WHERE id = ?"#, id)
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    pub async fn create_owner_and_return(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        email: String,
        first_name: Option<String>,
        last_name: Option<String>,
        avatar: String,
        password: String,
        group_id: i64,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"INSERT INTO users (
                    email, first_name, last_name, avatar, role,
                    password, is_registered, group_id, added_by
               ) VALUES (?, ?, ?, ?, 'OWNER', ?, 1, ?, NULL)
               RETURNING id AS "id?", email, last_name, first_name, avatar, role,
                         about_me, password, is_email_verify, email_verify_at,
                         two_factor_enable, is_registered, added_by, group_id,
                         created_at, updated_at"#,
            email,
            first_name,
            last_name,
            avatar,
            password,
            group_id
        )
        .fetch_one(&mut **tx)
        .await
    }

    pub async fn get_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"SELECT id AS "id?: i64", email AS "email?: String", last_name AS "last_name?: String", first_name AS "first_name?: String", avatar AS "avatar: String", role AS "role?: String", about_me AS "about_me?: String", password AS "password: String", is_email_verify AS "is_email_verify?: i64", email_verify_at AS "email_verify_at?: i64", two_factor_enable AS "two_factor_enable?: i64", is_registered AS "is_registered: i64", added_by AS "added_by?: i64", group_id AS "group_id: i64", created_at AS "created_at: i64", updated_at AS "updated_at: i64"
               FROM users WHERE email = ?"#,
            email
        )
        .fetch_optional(self.pool.as_ref())
        .await
    }
}
