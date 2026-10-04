use connectrpc::{ConnectError, RequestContext, Response, ServiceRequest, ServiceResult};

use crate::api::connect::generated::openoxide::api::v1::{
    CreateDestinationRequest, CreateDestinationResponse, DeleteDestinationRequest,
    DestinationService, DestinationServiceExt, Empty, GetDestinationRequest,
    GetDestinationResponse, ListDestinationsRequest, ListDestinationsResponse,
    TestDestinationRequest, TestDestinationResponse, UpdateDestinationRequest,
    UpdateDestinationResponse,
};
use crate::core::cache::{CacheEnum, CacheKey};
use crate::services::automation::destination::{CreateDestinationDto, PatchDestinationDto};

use super::helper::*;
use super::init::DestinationController;

#[auto_grpc::service(connect = true, auth = true)]
impl DestinationService for DestinationController {
    #[permission(Database, CanRead)]
    async fn list_destinations<'a>(
        &'a self,
        ctx: RequestContext,
        _query: ServiceRequest<'_, ListDestinationsRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<ListDestinationsResponse> + Send + use<'a>> {
        let organization_id = auth.organization_id;
        let cache_key = CacheKey::DestinationsList(organization_id);
        if let Some(CacheEnum::DestinationsList(cached)) = self.cache.get(&cache_key).await {
            let destinations = cached.into_iter().map(destination_to_proto).collect();
            return Response::ok(ListDestinationsResponse { destinations });
        }

        let items = self
            .service
            .list(organization_id)
            .await
            .map_err(map_sqlx_error)?;
        self.cache
            .insert(cache_key, CacheEnum::DestinationsList(items.clone()))
            .await;

        let destinations = items.into_iter().map(destination_to_proto).collect();
        Response::ok(ListDestinationsResponse { destinations })
    }

    #[permission(Database, CanRead)]
    async fn get_destination<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, GetDestinationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<GetDestinationResponse> + Send + use<'a>> {
        let dest = self
            .service
            .get_by_id(request.id, auth.organization_id)
            .await
            .map_err(map_sqlx_error)?;
        Response::ok(GetDestinationResponse {
            destination: ::buffa::MessageField::some(destination_to_proto(dest)),
        })
    }

    #[permission(Database, CanCreate)]
    async fn create_destination<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, CreateDestinationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<CreateDestinationResponse> + Send + use<'a>> {
        // The authenticated organization always owns the row.
        let dto = CreateDestinationDto {
            name: request.name.to_owned(),
            provider: request.provider.to_owned(),
            access_key: request.access_key.to_owned(),
            secret_access_key: request.secret_access_key.to_owned(),
            bucket: request.bucket.to_owned(),
            region: request.region.to_owned(),
            endpoint: request.endpoint.to_owned(),
            organization_id: auth.organization_id,
        };
        let created = self.service.create(dto).await.map_err(map_sqlx_error)?;
        self.cache
            .invalidate(&CacheKey::DestinationsList(auth.organization_id))
            .await;
        Response::ok(CreateDestinationResponse {
            destination: ::buffa::MessageField::some(destination_to_proto(created)),
        })
    }

    #[permission(Database, CanCreate)]
    async fn update_destination<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, UpdateDestinationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<UpdateDestinationResponse> + Send + use<'a>> {
        let dto = PatchDestinationDto {
            name: request.name.map(str::to_owned),
            provider: request.provider.map(str::to_owned),
            access_key: request.access_key.map(str::to_owned),
            secret_access_key: request.secret_access_key.map(str::to_owned),
            bucket: request.bucket.map(str::to_owned),
            region: request.region.map(str::to_owned),
            endpoint: request.endpoint.map(str::to_owned),
        };
        let updated = self
            .service
            .patch(request.id, dto, auth.organization_id)
            .await
            .map_err(map_sqlx_error)?;
        self.cache
            .invalidate(&CacheKey::DestinationsList(auth.organization_id))
            .await;
        Response::ok(UpdateDestinationResponse {
            destination: ::buffa::MessageField::some(destination_to_proto(updated)),
        })
    }

    #[permission(Database, CanDelete)]
    async fn delete_destination<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, DeleteDestinationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<Empty> + Send + use<'a>> {
        self.service
            .delete(request.id, auth.organization_id)
            .await
            .map_err(map_sqlx_error)?;
        self.cache
            .invalidate(&CacheKey::DestinationsList(auth.organization_id))
            .await;
        Response::ok(Empty::default())
    }

    #[permission(Database, CanCreate)]
    async fn test_destination<'a>(
        &'a self,
        ctx: RequestContext,
        request: ServiceRequest<'_, TestDestinationRequest>,
    ) -> ServiceResult<impl connectrpc::Encodable<TestDestinationResponse> + Send + use<'a>> {
        // Raw credentials test used before a destination row exists.
        self.service
            .test_connection_raw(
                request.provider,
                request.access_key,
                request.secret_access_key,
                request.bucket,
                request.region,
                request.endpoint,
            )
            .await
            .map_err(ConnectError::invalid_argument)?;
        Response::ok(TestDestinationResponse { success: true })
    }
}
