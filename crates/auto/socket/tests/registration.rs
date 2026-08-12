use auto_di::{Container, singleton};
use auto_socket::{auto_socket, on, register};
use serde::{Deserialize, Serialize};
use socketioxide::{
    SocketIo,
    extract::{Data, SocketRef},
};

#[derive(Deserialize)]
struct Message {
    text: String,
}

struct ChatSocket;

#[derive(Clone, Serialize)]
struct Status {
    online: bool,
}

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

    #[live]
    async fn status(&self, user_id: i64) -> Status {
        Status {
            online: user_id > 0,
        }
    }

    #[on("presence:update")]
    async fn presence(&self, user_id: i64) -> Status {
        Status {
            online: user_id > 0,
        }
    }
}

#[test]
fn generates_typed_live_handles_for_live_and_outbound_on() {
    let status = chat_live::status();
    assert_eq!(status.endpoint(), "ChatSocket::status");

    let presence = chat_live::presence();
    assert_eq!(presence.endpoint(), "ChatSocket::presence");
    let _broadcast = presence.broadcast(Status { online: true });

    let _publish = chat_live::status().publish((7_i64, "mobile"), Status { online: true });
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
