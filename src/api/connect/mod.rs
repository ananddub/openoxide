mod public;
mod server;

pub mod generated {
    connectrpc::include_generated!();
}

pub use public::PublicConnectService;
pub use server::serve;
