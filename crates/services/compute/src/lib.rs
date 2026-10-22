pub mod cache;
pub mod services {
    pub mod builder;
    pub mod compute;
    pub mod infrastructure;
    pub mod integrations;
    pub mod migration;
    pub mod observability;
}

// migration façades: keep the moved code on its original crate:: paths.
pub mod core {
    pub use openoxide_core::*;

    pub mod cache {
        pub use crate::cache::*;
    }
}

pub use openoxide_db::db;
pub use openoxide_db::flash_gen;

pub mod api {
    pub mod connect {
        pub mod generated {
            pub use openoxide_proto::generated::*;
        }
    }
}

pub mod utils {
    pub use openoxide_core::{jwt, paths};
    pub use openoxide_platform::utils::*;
    pub use os;

    pub mod builder {
        pub use crate::services::builder::{database, queue};
        pub use openoxide_builder::utils::builder::*;
    }
}
