use auto_di::{Container, singleton};
use auto_socket::{auto_socket, on, register};
use serde::Deserialize;
use socketioxide::{
    SocketIo,
    extract::{Data, SocketRef},
};

#[derive(Deserialize)]
struct Message {
    text: String,
}

struct ChatSocket;

#[singleton]
#[auto_socket("/chat")]
impl ChatSocket {
    fn new() -> Self {
        Self
    }

    #[on("message")]
    async fn message(&self, _socket: SocketRef, Data(message): Data<Message>) {
        let _ = message.text;
    }
}

#[on("ping", namespace = "/chat")]
async fn ping(_socket: SocketRef) {}

#[on("health")]
async fn health(_socket: SocketRef) {}

#[auto_socket("/room")]
mod room_events {
    use socketioxide::extract::{Data, SocketRef};

    #[on("join")]
    async fn join(_socket: SocketRef, Data(_room): Data<String>) {}
}

#[tokio::test]
async fn groups_impl_and_standalone_events_into_one_namespace() {
    let (_, io) = SocketIo::new_svc();
    let container = Container::new().unwrap();
    register(&io, &container).await.unwrap();

    let namespaces = io.nsps();
    assert_eq!(namespaces.len(), 3);
    assert!(
        namespaces
            .iter()
            .any(|namespace| namespace.ns_path() == "/chat")
    );
    assert!(
        namespaces
            .iter()
            .any(|namespace| namespace.ns_path() == "/")
    );
    assert!(
        namespaces
            .iter()
            .any(|namespace| namespace.ns_path() == "/room")
    );
}
