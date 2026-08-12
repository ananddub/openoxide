use rustploy_todo_test::{POOL, db};

#[tokio::main]
async fn main() {
    POOL.set(db::connect().await)
        .expect("pool initialized twice");
    let app = auto_route::routes()
        .await
        .expect("failed to build auto routes");
    let (socket_layer, io) = socketioxide::SocketIo::new_layer();
    auto_socket::register_global(&io)
        .await
        .expect("failed to register sockets");
    let app = app.layer(socket_layer);
    let address = std::env::var("TODO_ADDRESS").unwrap_or_else(|_| "127.0.0.1:3100".into());
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    println!("Todo API: http://{address}/api/todos");
    println!("React app: cd crates/todo-test/react && bun run dev");
    axum::serve(listener, app).await.unwrap();
}
