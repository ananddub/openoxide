#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResponseShape {
    Raw,
    Array,
    CreatedId,
}

#[derive(Clone, Debug)]
pub struct ApiRequest {
    pub method: &'static str,
    pub path: String,
    pub body: Option<serde_json::Value>,
    pub response_shape: ResponseShape,
    pub start_created: bool,
}

include!("dispatch.rs");
include!("common_core.rs");
include!("common_ops.rs");
include!("containers.rs");
include!("resources.rs");
include!("services.rs");
include!("swarm.rs");
include!("helpers.rs");
