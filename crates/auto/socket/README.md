# auto_socket

`auto_socket` automatically registers [Socketioxide](https://crates.io/crates/socketioxide) events and resolves handler objects with [`auto-di`](https://crates.io/crates/auto-di).

## Installation

```toml
[dependencies]
auto-di = "0.6"
auto_socket = "0.1"
axum = "0.8"
socketioxide = "0.18"
tokio = { version = "1", features = ["full"] }
```

## DI-managed socket handler

```rust,no_run
use auto_di::singleton;
use auto_socket::auto_socket;
use socketioxide::extract::{Data, SocketRef};

struct ChatSocket;

#[singleton]
#[auto_socket("/chat")]
impl ChatSocket {
    fn new() -> Self {
        Self
    }

    #[on("message")]
    async fn message(&self, socket: SocketRef, Data(message): Data<String>) {
        let _ = socket.emit("message", &message);
    }
}
```

## Standalone event

The namespace is optional and defaults to `/`:

```rust,no_run
use auto_socket::on;
use socketioxide::extract::SocketRef;

#[on("ping")]
async fn ping(socket: SocketRef) {
    let _ = socket.emit("pong", &"ok");
}

#[on("message", namespace = "/chat")]
async fn chat_message(_socket: SocketRef) {}
```

## Module event group

```rust,no_run
use auto_socket::auto_socket;

#[auto_socket("/room")]
mod room_events {
    use socketioxide::extract::{Data, SocketRef};

    #[on("join")]
    async fn join(_socket: SocketRef, Data(_room): Data<String>) {}
}
```

Only inline modules are supported.

## Start with Axum

Call `register_global` after creating `SocketIo` and before serving the application:

```rust,no_run
use axum::Router;
use socketioxide::SocketIo;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let (layer, io) = SocketIo::new_layer();
    auto_socket::register_global(&io)
        .await
        .expect("failed to register socket handlers");

    let app = Router::new().layer(layer);
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

Registrations sharing a namespace are grouped so Socketioxide installs each namespace once. Handler objects are singleton values resolved by `auto-di`.

## License

MIT
