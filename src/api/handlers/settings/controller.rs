use connectrpc::{ConnectError, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, ResourceRequest, ResourceResponse, SettingsService, SettingsServiceExt,
};
use crate::services::settings::UpdateSettingsDto;

use super::helper::*;
use super::init::SettingsController;

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
