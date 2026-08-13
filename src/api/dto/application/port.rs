use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::db::models::ports::Port;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, poem_openapi::Enum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[oai(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PortProtocol {
    Tcp,
    Udp,
}
impl PortProtocol {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Tcp => "TCP",
            Self::Udp => "UDP",
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, poem_openapi::Enum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[oai(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PortPublishMode {
    Host,
    Ingress,
}
impl PortPublishMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Host => "HOST",
            Self::Ingress => "INGRESS",
        }
    }
}

#[derive(Debug, Validate, Deserialize, poem_openapi::Object)]
pub struct UpsertApplicationPortDto {
    #[validate(range(min = 1, max = 65535))]
    pub published_port: i64,
    #[validate(range(min = 1, max = 65535))]
    pub target_port: i64,
    pub protocol: PortProtocol,
    pub publish_mode: PortPublishMode,
}

impl UpsertApplicationPortDto {
    pub fn normalize(self) -> Result<Self, String> {
        Ok(self)
    }
}

#[derive(Debug, Clone, Serialize, poem_openapi::Object, ts_rs::TS)]
pub struct ApplicationPortResponseDto {
    pub id: i64,
    pub published_port: i64,
    pub target_port: i64,
    pub protocol: String,
    pub publish_mode: String,
    pub application_id: i64,
    pub created_at: i64,
}

impl From<Port> for ApplicationPortResponseDto {
    fn from(value: Port) -> Self {
        Self {
            id: value.id.unwrap_or_default(),
            published_port: value.published_port,
            target_port: value.target_port,
            protocol: value.protocol,
            publish_mode: value.publish_mode,
            application_id: value.application_id,
            created_at: value.created_at,
        }
    }
}
