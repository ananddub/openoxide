pub mod services {
    pub mod identity;
    // identity sends notification emails; integrations lives in openoxide-compute.
    pub use openoxide_compute::services::integrations;
}

// migration façades: keep the moved code on its original crate:: paths.
pub mod core {
    pub use openoxide_core::*;
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
}
