use connectrpc::{ConnectError, ErrorCode, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    Empty, SshKeyCreateKeyRequest, SshKeyCreateKeyResponse, SshKeyDeleteKeyRequest,
    SshKeyGenerateKeyRequest, SshKeyGenerateKeyResponse, SshKeyGeneratePairRequest,
    SshKeyGeneratePairResponse, SshKeyGetKeyRequest, SshKeyGetKeyResponse, SshKeyListKeysRequest,
    SshKeyListKeysResponse, SshKeyMarkUsedRequest, SshKeyService, SshKeyServiceExt,
    SshKeyUpdateKeyRequest, SshKeyUpdateKeyResponse,
};
use crate::services::infrastructure::ssh_key::{CreateSshKeyDto, PatchSshKeyDto};

use super::helper::*;
use super::init::SshKeyController;

#[auto_grpc::service(connect = true, auth = true)]
impl SshKeyService for SshKeyController {
    #[permission(Server, CanRead)]
    async fn list_keys<'a>(
        &'a self,
        ctx: RequestContext,
        _query: ServiceRequest<'_, SshKeyListKeysRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<SshKeyListKeysResponse> + Send + use<'a>> {
        let items = self.service.list(auth.organization_id).await.map_err(map_db_error)?;
        let keys = items.into_iter().map(key_to_proto).collect();
        Response::ok(SshKeyListKeysResponse {
            keys,
            ..Default::default()
        })
    }

    #[permission(Server, CanRead)]
    async fn get_key<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, SshKeyGetKeyRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<SshKeyGetKeyResponse> + Send + use<'a>> {
        let key = self
            .service
            .get_by_id(request.id, auth.organization_id)
            .await
            .map_err(map_db_error)?;
        Response::ok(SshKeyGetKeyResponse {
            key: ::buffa::MessageField::some(key_to_proto(key)),
            ..Default::default()
        })
    }

    #[permission(Server, CanCreate)]
    async fn create_key<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, SshKeyCreateKeyRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<SshKeyCreateKeyResponse> + Send + use<'a>> {
        let dto = CreateSshKeyDto {
            name: request.name.to_owned(),
            description: request.description.map(str::to_owned),
            public_key: request.public_key.to_owned(),
            private_key: request.private_key.to_owned(),
        };
        let key = self
            .service
            .create(dto, auth.organization_id)
            .await
            .map_err(map_db_error)?;
        Response::ok(SshKeyCreateKeyResponse {
            key: ::buffa::MessageField::some(key_to_proto(key)),
            ..Default::default()
        })
    }

    #[permission(Server, CanCreate)]
    async fn generate_key<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, SshKeyGenerateKeyRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<SshKeyGenerateKeyResponse> + Send + use<'a>> {
        let key_type = key_type_to_str(request.key_type)?;
        let key = self
            .service
            .generate(
                request.name.to_owned(),
                request.description.map(str::to_owned),
                key_type,
                auth.organization_id,
            )
            .await
            .map_err(map_db_error)?;
        Response::ok(SshKeyGenerateKeyResponse {
            key: ::buffa::MessageField::some(key_to_proto(key)),
            ..Default::default()
        })
    }

    async fn generate_pair<'a>(
        &'a self,
        _ctx: RequestContext,
        request: ServiceRequest<'_, SshKeyGeneratePairRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<SshKeyGeneratePairResponse> + Send + use<'a>>
    {
        let key_type = match request.key_type {
            Some(value) => key_type_to_str(value)?,
            None => "ed25519",
        };
        let (private_key, public_key) = crate::utils::ssh::generate_keypair(key_type)
            .map_err(|e| ConnectError::new(ErrorCode::Internal, e.to_string()))?;
        Response::ok(SshKeyGeneratePairResponse {
            private_key,
            public_key,
            ..Default::default()
        })
    }

    #[permission(Server, CanCreate)]
    async fn update_key<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, SshKeyUpdateKeyRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<SshKeyUpdateKeyResponse> + Send + use<'a>> {
        let dto = PatchSshKeyDto {
            name: request.name.map(str::to_owned),
            description: request.description.map(str::to_owned),
            public_key: request.public_key.map(str::to_owned),
            private_key: request.private_key.map(str::to_owned),
        };
        let key = self
            .service
            .patch(request.id, dto, auth.organization_id)
            .await
            .map_err(map_db_error)?;
        Response::ok(SshKeyUpdateKeyResponse {
            key: ::buffa::MessageField::some(key_to_proto(key)),
            ..Default::default()
        })
    }

    #[permission(Server, CanCreate)]
    async fn mark_used<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, SshKeyMarkUsedRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service
            .mark_used(request.id, auth.organization_id)
            .await
            .map_err(map_db_error)?;
        Response::ok(Empty::default())
    }

    #[permission(Server, CanDelete)]
    async fn delete_key<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, SshKeyDeleteKeyRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service
            .delete(request.id, auth.organization_id)
            .await
            .map_err(map_db_error)?;
        Response::ok(Empty::default())
    }
}
