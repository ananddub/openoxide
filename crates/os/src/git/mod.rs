pub use client::GitCli;
pub use types::{GitBranch, GitStatusEntry};

pub use provider::{GitProvider, GitProviderBuilder};

pub mod client;
pub mod handles;
pub mod provider;
pub mod types;
