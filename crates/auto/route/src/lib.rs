#![doc = include_str!("../README.md")]

use std::collections::{BTreeMap, HashMap};

use auto_di::{BoxFuture, Container, DiError};
use axum::{Json, Router, response::Html, routing::get};
use poem_openapi::{
    __private::poem::{endpoint::BoxEndpoint, http::Method},
    OpenApi, OpenApiService, ParameterStyle,
    registry::{
        MetaApi, MetaMediaType, MetaOperation, MetaOperationParam, MetaParamIn, MetaPath,
        MetaRequest, MetaResponse, MetaResponses, MetaSchemaRef, MetaTag, Registry,
    },
    types::Type,
};
use scalar_api_reference::axum as scalar;
use serde_json::Value;

pub use auto_route_macros::{controller, delete, get, head, options, patch, post, put};

/// A controller router factory submitted by the `#[controller]` macro.
#[doc(hidden)]
pub struct RouteDescriptor {
    factory: for<'a> fn(&'a Container) -> BoxFuture<'a, Result<Router<()>, DiError>>,
}

impl RouteDescriptor {
    #[doc(hidden)]
    pub const fn new(
        factory: for<'a> fn(&'a Container) -> BoxFuture<'a, Result<Router<()>, DiError>>,
    ) -> Self {
        Self { factory }
    }
}

inventory::collect!(RouteDescriptor);

/// OpenAPI metadata submitted by the route macros.
#[doc(hidden)]
pub struct OpenApiRouteDescriptor {
    method: &'static str,
    path: &'static str,
    operation_id: &'static str,
    tag: &'static str,
    tag_description: Option<&'static str>,
    summary: Option<&'static str>,
    description: Option<&'static str>,
    params: &'static [OpenApiParamDescriptor],
    request_description: Option<&'static str>,
    request: Option<OpenApiSchemaDescriptor>,
    response_description: Option<&'static str>,
    response: Option<OpenApiSchemaDescriptor>,
}

impl OpenApiRouteDescriptor {
    #[doc(hidden)]
    pub const fn new(
        method: &'static str,
        path: &'static str,
        operation_id: &'static str,
        tag: &'static str,
        tag_description: Option<&'static str>,
        summary: Option<&'static str>,
        description: Option<&'static str>,
        params: &'static [OpenApiParamDescriptor],
        request_description: Option<&'static str>,
        request: Option<OpenApiSchemaDescriptor>,
        response_description: Option<&'static str>,
        response: Option<OpenApiSchemaDescriptor>,
    ) -> Self {
        Self {
            method,
            path,
            operation_id,
            tag,
            tag_description,
            summary,
            description,
            params,
            request_description,
            request,
            response_description,
            response,
        }
    }
}

inventory::collect!(OpenApiRouteDescriptor);

/// OpenAPI schema metadata submitted by the route macros.
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct OpenApiSchemaDescriptor {
    schema_ref: fn() -> MetaSchemaRef,
    register: fn(&mut Registry),
    content_type: &'static str,
}

impl OpenApiSchemaDescriptor {
    #[doc(hidden)]
    pub const fn json<T: Type>() -> Self {
        Self {
            schema_ref: schema_ref::<T>,
            register: register_type::<T>,
            content_type: "application/json",
        }
    }

    #[doc(hidden)]
    pub const fn form<T: Type>() -> Self {
        Self {
            schema_ref: schema_ref::<T>,
            register: register_type::<T>,
            content_type: "application/x-www-form-urlencoded",
        }
    }

    #[doc(hidden)]
    pub const fn sse() -> Self {
        Self {
            schema_ref: sse_schema_ref,
            register: noop_register,
            content_type: "text/event-stream",
        }
    }

    #[doc(hidden)]
    pub const fn sse_json<T: Type>() -> Self {
        Self {
            schema_ref: schema_ref::<T>,
            register: register_type::<T>,
            content_type: "text/event-stream",
        }
    }
}

/// OpenAPI parameter metadata submitted by the route macros.
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct OpenApiParamDescriptor {
    name: &'static str,
    in_type: MetaParamIn,
    schema: OpenApiSchemaDescriptor,
    required: bool,
    deprecated: bool,
    explode: bool,
    style: Option<ParameterStyle>,
}

impl OpenApiParamDescriptor {
    #[doc(hidden)]
    pub const fn path<T: Type>(name: &'static str) -> Self {
        Self {
            name,
            in_type: MetaParamIn::Path,
            schema: OpenApiSchemaDescriptor::json::<T>(),
            required: true,
            deprecated: false,
            explode: false,
            style: None,
        }
    }

    #[doc(hidden)]
    pub const fn query<T: Type>(name: &'static str) -> Self {
        Self {
            name,
            in_type: MetaParamIn::Query,
            schema: OpenApiSchemaDescriptor::json::<T>(),
            required: true,
            deprecated: false,
            explode: true,
            style: Some(ParameterStyle::Form),
        }
    }
}

fn schema_ref<T: Type>() -> MetaSchemaRef {
    T::schema_ref()
}

fn register_type<T: Type>(registry: &mut Registry) {
    T::register(registry);
}

fn noop_register(_registry: &mut Registry) {}

fn sse_schema_ref() -> MetaSchemaRef {
    let mut schema = poem_openapi::registry::MetaSchema::new_with_format("string", "event-stream");
    schema.description =
        Some("Server-sent events stream. Each message is encoded as text/event-stream frames.");
    schema.example = Some(serde_json::json!(
        "event: message\ndata: {\"type\":\"message\"}\n\n"
    ));
    MetaSchemaRef::Inline(Box::new(schema))
}

/// Resolves every registered controller from `container` and merges its routes.
pub async fn build_routes(container: &Container) -> Result<Router<()>, DiError> {
    let mut router = Router::new();
    for descriptor in inventory::iter::<RouteDescriptor> {
        router = router.merge((descriptor.factory)(container).await?);
    }
    Ok(router)
}

/// Builds all routes using auto-di's global container.
pub async fn routes() -> Result<Router<()>, DiError> {
    build_routes(auto_di::global_container()?).await
}

#[doc(hidden)]
pub struct AutoRouteOpenApi;

impl OpenApi for AutoRouteOpenApi {
    fn meta() -> Vec<MetaApi> {
        let mut paths = BTreeMap::<String, Vec<MetaOperation>>::new();

        for descriptor in inventory::iter::<OpenApiRouteDescriptor> {
            paths
                .entry(descriptor.path.to_owned())
                .or_default()
                .push(openapi_operation(descriptor));
        }

        vec![MetaApi {
            paths: paths
                .into_iter()
                .map(|(path, operations)| MetaPath { path, operations })
                .collect(),
        }]
    }

    fn register(registry: &mut Registry) {
        for descriptor in inventory::iter::<OpenApiRouteDescriptor> {
            registry.tags.insert(MetaTag {
                name: descriptor.tag,
                description: descriptor.tag_description,
                external_docs: None,
            });
            if let Some(schema) = descriptor.request {
                (schema.register)(registry);
            }
            if let Some(schema) = descriptor.response {
                (schema.register)(registry);
            }
            for param in descriptor.params {
                (param.schema.register)(registry);
            }
        }
    }

    fn add_routes(self, _route_table: &mut HashMap<String, HashMap<Method, BoxEndpoint<'static>>>) {
        // Axum serving is still handled by `RouteDescriptor`; this hidden Poem
        // adapter exists only so OpenAPI generation goes through poem-openapi.
    }
}

/// Builds an OpenAPI document from every registered auto-route using
/// poem-openapi's `OpenApiService`.
pub fn openapi_json() -> Value {
    serde_json::from_str(&openapi_spec()).expect("poem-openapi spec must be valid JSON")
}

/// Builds an OpenAPI JSON string using poem-openapi's native service.
pub fn openapi_spec() -> String {
    OpenApiService::new(
        AutoRouteOpenApi,
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    )
    .spec()
}

fn openapi_operation(descriptor: &OpenApiRouteDescriptor) -> MetaOperation {
    MetaOperation {
        method: method_from_str(descriptor.method),
        tags: vec![descriptor.tag],
        summary: descriptor.summary,
        description: descriptor.description,
        external_docs: None,
        params: descriptor
            .params
            .iter()
            .map(|param| param.to_meta())
            .collect(),
        request: descriptor.request.map(|schema| MetaRequest {
            description: descriptor.request_description,
            content: vec![schema.media_type()],
            required: true,
        }),
        responses: MetaResponses {
            responses: vec![MetaResponse {
                description: descriptor
                    .response_description
                    .unwrap_or("Successful response"),
                status: Some(200),
                status_range: None,
                content: descriptor
                    .response
                    .map(|schema| vec![schema.media_type()])
                    .unwrap_or_default(),
                headers: Vec::new(),
            }],
        },
        deprecated: false,
        security: Vec::new(),
        operation_id: Some(descriptor.operation_id),
        code_samples: Vec::new(),
    }
}

impl OpenApiSchemaDescriptor {
    fn media_type(self) -> MetaMediaType {
        MetaMediaType {
            content_type: self.content_type,
            schema: (self.schema_ref)(),
        }
    }
}

impl OpenApiParamDescriptor {
    fn to_meta(self) -> MetaOperationParam {
        MetaOperationParam {
            name: self.name.trim_start_matches('*').to_owned(),
            schema: (self.schema.schema_ref)(),
            in_type: self.in_type,
            description: None,
            required: self.required,
            deprecated: self.deprecated,
            explode: self.explode,
            style: self.style,
        }
    }
}

fn method_from_str(method: &str) -> Method {
    match method {
        "GET" => Method::GET,
        "POST" => Method::POST,
        "PUT" => Method::PUT,
        "DELETE" => Method::DELETE,
        "PATCH" => Method::PATCH,
        "OPTIONS" => Method::OPTIONS,
        "HEAD" => Method::HEAD,
        _ => Method::GET,
    }
}

/// Adds `/openapi.json` and `/swagger-ui` style routes to an Axum router.
pub fn openapi_routes(json_path: &'static str, ui_path: &'static str) -> Router<()> {
    Router::new()
        .route(json_path, get(|| async { Json(openapi_json()) }))
        .route(ui_path, get(|| async { Html(swagger_ui_html()) }))
}

/// Adds a Scalar API Reference route backed by the generated OpenAPI document.
pub fn scalar_routes(scalar_path: &'static str, openapi_json_path: &'static str) -> Router<()> {
    let configuration = serde_json::json!({
        "url": openapi_json_path,
        "theme": "Deep Space",
        "layout": "modern",
        "showSidebar": true,
        // "hideDownloadButton": false,
        "agent": { "disabled": true },
    });

    scalar::router(scalar_path, &configuration)
}

fn swagger_ui_html() -> String {
    OpenApiService::new(
        AutoRouteOpenApi,
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    )
    .swagger_ui_html()
}

/// Re-exports used by generated code. Applications don't need these dependencies
/// solely because the macro expands to them.
#[doc(hidden)]
pub mod __private {
    pub use auto_di;
    pub use auto_socket;
    pub use axum;
    pub use inventory;
    pub use poem_openapi;
}
