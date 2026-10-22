pub mod generated {
    connectrpc::include_generated!();
}

pub mod agent {
    tonic::include_proto!("agent");
}

pub mod ai_catalog {

    include!(concat!(env!("OUT_DIR"), "/ai_catalog_generated.rs"));
}

pub static DESCRIPTOR_SET: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/openoxide.fds.bin"));
