use auto_di::{BoxFuture, Container, DiError};

#[cfg(feature = "connect")]
use connectrpc::Router;
#[cfg(feature = "tonic")]
use tonic::service::RoutesBuilder;

pub use auto_grpc_macros::middleware;
pub use auto_grpc_macros::service;

pub trait MiddlewareOrder {
    const ORDER: i32;
}

#[doc(hidden)]
pub mod __private {
    pub use auto_di;
    #[cfg(feature = "connect")]
    pub use connectrpc;
    pub use inventory;
    #[cfg(feature = "tonic")]
    pub use tonic;
}

#[cfg(feature = "tonic")]
pub type TonicRegistrar =
    for<'a> fn(&'a Container, &'a mut RoutesBuilder) -> BoxFuture<'a, Result<(), DiError>>;

#[cfg(feature = "tonic")]
pub struct TonicDescriptor {
    pub registrar: TonicRegistrar,
}

#[cfg(feature = "tonic")]
inventory::collect!(TonicDescriptor);

#[cfg(feature = "tonic")]
pub async fn register(container: &Container, builder: &mut RoutesBuilder) -> Result<(), DiError> {
    for descriptor in inventory::iter::<TonicDescriptor> {
        (descriptor.registrar)(container, builder).await?;
    }
    Ok(())
}

#[cfg(feature = "connect")]
pub type ConnectRegistrar =
    for<'a> fn(&'a Container, Router) -> BoxFuture<'a, Result<Router, DiError>>;

#[cfg(feature = "connect")]
pub struct ConnectDescriptor {
    pub registrar: ConnectRegistrar,
}

#[cfg(feature = "connect")]
inventory::collect!(ConnectDescriptor);

#[cfg(feature = "connect")]
pub async fn build_connect_routes(container: &Container) -> Result<Router, DiError> {
    let mut router = Router::new();
    for descriptor in inventory::iter::<ConnectDescriptor> {
        router = (descriptor.registrar)(container, router).await?;
    }
    Ok(router)
}

#[cfg(feature = "connect")]
pub async fn connect_routes() -> Result<Router, DiError> {
    build_connect_routes(auto_di::global_container()?).await
}
