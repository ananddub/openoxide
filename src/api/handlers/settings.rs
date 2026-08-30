use std::sync::Arc;

use auto_di::singleton;
use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, ResourceRequest, ResourceResponse, SettingsService, SettingsServiceExt,
};
use crate::services::settings::{SettingsService as AppSettingsService, UpdateSettingsDto};

pub struct SettingsController {
    service: Arc<AppSettingsService>,
}

#[singleton]
impl SettingsController {
    pub fn new(service: Arc<AppSettingsService>) -> Self {
        Self { service }
    }
}

fn map_error(err: sqlx::Error) -> ConnectError {
    match &err {
        sqlx::Error::Protocol(msg) => ConnectError::invalid_argument(msg.clone()),
        other => {
            tracing::error!(error = %other, "settings operation failed");
            ConnectError::new(ErrorCode::Internal, "settings operation failed")
        }
    }
}

#[auto_grpc::service(connect = true, auth = true)]
impl SettingsService for SettingsController {
    #[permission(Organization, CanWrite)]
    async fn get_settings<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, Empty>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let settings = self.service.get().await.map_err(map_error)?;
        let payload = serde_json::to_string(&settings).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: settings.id,
            kind: "settings".into(),
            payload,
            ..Default::default()
        })
    }

    #[permission(Organization, CanWrite)]
    async fn update_settings<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, ResourceRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ResourceResponse> + Send + use<'a>> {
        let dto: UpdateSettingsDto = serde_json::from_str(request.payload)
            .map_err(|e| ConnectError::invalid_argument(e.to_string()))?;
        let settings = self.service.update(dto).await.map_err(map_error)?;
        let payload = serde_json::to_string(&settings).unwrap_or_default();
        Response::ok(ResourceResponse {
            id: settings.id,
            kind: "settings".into(),
            payload,
            ..Default::default()
        })
    }
}
