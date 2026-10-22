mod public;
mod server;

pub mod generated {
    pub use openoxide_proto::generated::*;
}

pub use public::PublicConnectService;
pub use server::{serve, tls_config};
