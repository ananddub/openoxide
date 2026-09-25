use connectrpc::{RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    SettingsGetSettingsRequest, SettingsGetSettingsResponse, SettingsService, SettingsServiceExt,
    SettingsUpdateSettingsRequest, SettingsUpdateSettingsResponse,
};
use crate::services::platform::settings::UpdateSettingsDto;

use super::helper::*;
use super::init::SettingsController;

#[auto_grpc::service(connect = true, auth = true)]
impl SettingsService for SettingsController {
    #[permission(Organization, CanUpdate)]
    async fn get_settings<'a>(
        &'a self,
        ctx: RequestContext,
        _request: ServiceRequest<'_, SettingsGetSettingsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<SettingsGetSettingsResponse> + Send + use<'a>>
    {
        let settings = self.service.get().await.map_err(map_error)?;
        Response::ok(SettingsGetSettingsResponse {
            settings: ::buffa::MessageField::some(settings_to_proto(settings)),
            ..Default::default()
        })
    }

    #[permission(Organization, CanUpdate)]
    async fn update_settings<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, SettingsUpdateSettingsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<SettingsUpdateSettingsResponse> + Send + use<'a>>
    {
        let dto = UpdateSettingsDto {
            server_ip: request.server_ip.map(str::to_owned),
            certificate_type: request
                .certificate_type
                .map(proto_to_cert_type)
                .transpose()?,
            custom_cert_resolver: request.custom_cert_resolver.map(str::to_owned),
            https: request.https,
            host: request.host.map(str::to_owned),
            lets_encrypt_email: request.lets_encrypt_email.map(str::to_owned),
            enable_docker_cleanup: request.enable_docker_cleanup,
            log_cleanup_cron: request.log_cleanup_cron.map(str::to_owned),
            metrics_config: request.metrics_config.map(str::to_owned),
            panel_backup_cron: request.panel_backup_cron.map(str::to_owned),
            log_retention_days: request.log_retention_days,
            panel_backup_enabled: request.panel_backup_enabled,
            log_cleanup_enabled: request.log_cleanup_enabled,
        };
        let settings = self.service.update(dto).await.map_err(map_error)?;
        Response::ok(SettingsUpdateSettingsResponse {
            settings: ::buffa::MessageField::some(settings_to_proto(settings)),
            ..Default::default()
        })
    }
}
