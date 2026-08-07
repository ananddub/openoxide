#![doc = include_str!("../README.md")]

use std::{collections::HashMap, sync::Arc};

use auto_di::{BoxFuture, Container, DiError};
use socketioxide::{SocketIo, extract::SocketRef};

pub use auto_socket_macros::{auto_socket, on};

#[doc(hidden)]
pub type SocketRegistrar = Arc<dyn Fn(SocketRef) + Send + Sync + 'static>;

/// A namespace event registrar submitted by the socket macros.
#[doc(hidden)]
pub struct SocketDescriptor {
    namespace: &'static str,
    factory: for<'a> fn(&'a Container) -> BoxFuture<'a, Result<SocketRegistrar, DiError>>,
}

impl SocketDescriptor {
    #[doc(hidden)]
    pub const fn new(
        namespace: &'static str,
        factory: for<'a> fn(&'a Container) -> BoxFuture<'a, Result<SocketRegistrar, DiError>>,
    ) -> Self {
        Self { namespace, factory }
    }
}

inventory::collect!(SocketDescriptor);

/// Resolves socket handler objects and registers each namespace exactly once.
pub async fn register(io: &SocketIo, container: &Container) -> Result<(), DiError> {
    let mut namespaces: HashMap<&'static str, Vec<SocketRegistrar>> = HashMap::new();

    for descriptor in inventory::iter::<SocketDescriptor> {
        namespaces
            .entry(descriptor.namespace)
            .or_default()
            .push((descriptor.factory)(container).await?);
    }

    for (namespace, registrars) in namespaces {
        io.ns(namespace, move |socket: SocketRef| {
            let registrars = registrars.clone();
            async move {
                for registrar in registrars {
                    registrar(socket.clone());
                }
            }
        });
    }

    Ok(())
}

/// Registers all socket handlers using auto-di's global container.
pub async fn register_global(io: &SocketIo) -> Result<(), DiError> {
    register(io, auto_di::global_container()?).await
}

#[doc(hidden)]
pub mod __private {
    pub use auto_di;
    pub use inventory;
    pub use socketioxide;
}
