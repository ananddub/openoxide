use connectrpc::{ConnectError, RequestContext, Response, ServiceRequest, ServiceResult};
use futures::StreamExt;

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, ResourceRequest, ResourceResponse, SettingsService, SettingsServiceExt,
};
use crate::db::reactive::realtime;
use crate::flash_gen::models::Settings;
use crate::services::platform::settings::{SettingsResponseDto, UpdateSettingsDto};

use super::helper::*;
use super::init::SettingsController;

#[auto_grpc::service(connect = true, auth = true)]
impl SettingsService for SettingsController {
    #[permission(Organization, CanWrite)]
    async fn stream_get_settings(
        &self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, Empty>,
    ) -> ServiceResult<
        connectrpc::ServiceStream<
            impl connectrpc::Encodable<ResourceResponse> + Send + use<>,
        >,
    > {
        let stream = {
            let sql = "SELECT * FROM settings".to_string();
            realtime()
                .watch_query::<Settings>(sql)
                .await
                .map_err(map_error)?
        };
        Response::stream_ok(stream.map(move |mut rows| match rows.pop() {
            Some(settings) => {
                let dto = SettingsResponseDto::from(settings);
                let payload = serde_json::to_string(&dto).unwrap_or_default();
                Ok(ResourceResponse {
                    id: dto.id,
                    kind: "settings".into(),
                    payload,
                    ..Default::default()
                })
            }
            None => Err(ConnectError::not_found("settings not found")),
        }))
    }

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
