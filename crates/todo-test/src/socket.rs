use crate::{controller::load_todos, models::Todo};
use auto_di::singleton;
use auto_socket::auto_socket;
use sqlx::SqlitePool;

pub struct TodoSocket {
    pool: SqlitePool,
}

#[singleton]
#[auto_socket("/todo")]
impl TodoSocket {
    fn new() -> Self {
        Self {
            pool: crate::POOL.get().expect("database is initialized").clone(),
        }
    }

    #[live]
    async fn todos(&self) -> Vec<Todo> {
        load_todos(&self.pool).await
    }
}

pub async fn publish_todos(pool: &SqlitePool) {
    let todos = load_todos(pool).await;
    if let Ok(publisher) = todo_live::todos() {
        let _ = publisher.publish(todos).await;
    }
}
