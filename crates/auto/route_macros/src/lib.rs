use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Attribute, Expr, FnArg, GenericArgument, Ident, ImplItem, Item, ItemFn, ItemImpl, ItemMod, Lit,
    LitStr, Meta, PatType, PathArguments, ReturnType, Token, Type, parse::Parse,
    parse::ParseStream, parse_macro_input, spanned::Spanned,
};

const METHODS: &[&str] = &["get", "post", "put", "delete", "patch", "options", "head"];

/// Declares an Axum controller whose receiver methods can carry route markers.
///
/// Supported markers are `#[get]`, `#[post]`, `#[put]`, `#[delete]`,
/// `#[patch]`, `#[options]`, and `#[head]`, each with an optional path.
#[proc_macro_attribute]
pub fn controller(attr: TokenStream, item: TokenStream) -> TokenStream {
    let controller_options = parse_macro_input!(attr as ControllerOptions);
    let item = parse_macro_input!(item as Item);

    let expanded = match item {
        Item::Impl(item_impl) => expand_controller(controller_options, item_impl),
        Item::Mod(item_mod) => expand_controller_module(controller_options, item_mod),
        other => Err(syn::Error::new_spanned(
            other,
            "#[controller] must be placed on an inherent impl or inline module",
        )),
    };

    match expanded {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

macro_rules! route_attribute {
    ($name:ident) => {
        #[proc_macro_attribute]
        pub fn $name(attr: TokenStream, item: TokenStream) -> TokenStream {
            standalone_route(stringify!($name), attr, item)
        }
    };
}

route_attribute!(get);
route_attribute!(post);
route_attribute!(put);
route_attribute!(delete);
route_attribute!(patch);
route_attribute!(options);
route_attribute!(head);

fn standalone_route(method: &str, attr: TokenStream, item: TokenStream) -> TokenStream {
    let route_options = parse_macro_input!(attr as RouteOptions);
    let function = parse_macro_input!(item as ItemFn);
    match expand_standalone_route(method, route_options, function) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

struct RouteOptions {
    path: LitStr,
    sse_body: Option<Type>,
    docs: Docs,
}

#[derive(Clone, Default)]
struct Docs {
    tag: Option<LitStr>,
    tag_description: Option<LitStr>,
    summary: Option<LitStr>,
    description: Option<LitStr>,
    request_description: Option<LitStr>,
    response_description: Option<LitStr>,
}

struct ControllerOptions {
    path: LitStr,
    docs: Docs,
}

impl ControllerOptions {
    fn empty() -> Self {
        Self {
            path: LitStr::new("", proc_macro2::Span::call_site()),
            docs: Docs::default(),
        }
    }
}

impl RouteOptions {
    fn empty() -> Self {
        Self {
            path: LitStr::new("", proc_macro2::Span::call_site()),
            sse_body: None,
            docs: Docs::default(),
        }
    }
}

impl Parse for ControllerOptions {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let mut options = ControllerOptions::empty();

        let mut needs_comma = false;
        if input.peek(LitStr) {
            options.path = input.parse()?;
            needs_comma = true;
        }

        while !input.is_empty() {
            if needs_comma {
                input.parse::<Token![,]>()?;
                if input.is_empty() {
                    break;
                }
            }

            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            parse_docs_option(&mut options.docs, key, input)?;
            needs_comma = true;
        }

        Ok(options)
    }
}

impl Parse for RouteOptions {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let mut options = RouteOptions::empty();

        let mut needs_comma = false;
        if input.peek(LitStr) {
            options.path = input.parse()?;
            needs_comma = true;
        }

        while !input.is_empty() {
            if needs_comma {
                input.parse::<Token![,]>()?;
                if input.is_empty() {
                    break;
                }
            }

            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            if key == "sse" {
                if options.sse_body.is_some() {
                    return Err(syn::Error::new_spanned(key, "duplicate `sse` route option"));
                }
                options.sse_body = Some(input.parse()?);
            } else {
                parse_docs_option(&mut options.docs, key, input)?;
            }
            needs_comma = true;
        }

        Ok(options)
    }
}

fn parse_docs_option(docs: &mut Docs, key: Ident, input: ParseStream<'_>) -> syn::Result<()> {
    let value: LitStr = input.parse()?;
    match key.to_string().as_str() {
        "tag" => set_lit(&mut docs.tag, key, value),
        "tag_description" => set_lit(&mut docs.tag_description, key, value),
        "summary" => set_lit(&mut docs.summary, key, value),
        "description" | "docs" => set_lit(&mut docs.description, key, value),
        "request_description" | "request_docs" => {
            set_lit(&mut docs.request_description, key, value)
        }
        "response_description" | "response_docs" => {
            set_lit(&mut docs.response_description, key, value)
        }
        _ => Err(syn::Error::new_spanned(
            key,
            "unsupported route option; expected `sse = Type`, `tag`, `summary`, `description`, `request_description`, or `response_description`",
        )),
    }
}

fn set_lit(slot: &mut Option<LitStr>, key: Ident, value: LitStr) -> syn::Result<()> {
    if slot.is_some() {
        return Err(syn::Error::new_spanned(key, "duplicate route option"));
    }
    *slot = Some(value);
    Ok(())
}

fn expand_standalone_route(
    method: &str,
    route_options: RouteOptions,
    function: ItemFn,
) -> syn::Result<proc_macro2::TokenStream> {
    if function.sig.asyncness.is_none() {
        return Err(syn::Error::new_spanned(
            &function.sig,
            "standalone route handlers must be async",
        ));
    }
    if !function.sig.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            &function.sig.generics,
            "generic standalone route handlers are not supported",
        ));
    }

    let handler = &function.sig.ident;
    let operation_id = format!("{handler}");
    let method = format_ident!("{method}");
    let method_name = method.to_string().to_ascii_uppercase();
    let factory = format_ident!("__auto_route_factory_{}", handler);
    let path = LitStr::new(
        &join_paths("", &route_options.path.value()),
        route_options.path.span(),
    );
    let argument_types = function
        .sig
        .inputs
        .iter()
        .map(|argument| match argument {
            FnArg::Typed(PatType { ty, .. }) => Ok((**ty).clone()),
            FnArg::Receiver(receiver) => Err(syn::Error::new_spanned(
                receiver,
                "standalone route handlers cannot take self",
            )),
        })
        .collect::<syn::Result<Vec<_>>>()?;
    let params = infer_params(&path.value(), &argument_types);
    let params = param_descriptor_tokens(&params);
    let request_schema =
        request_schema_descriptor_tokens(infer_request_body(&argument_types).as_ref());
    let docs = merged_docs(&route_options.docs, doc_comment_summary(&function.attrs));
    let tag = docs
        .tag
        .unwrap_or_else(|| LitStr::new(&openapi_tag(&path.value()), path.span()));
    let tag_description = option_lit_tokens(docs.tag_description.as_ref());
    let summary = option_lit_tokens(docs.summary.as_ref());
    let description = option_lit_tokens(docs.description.as_ref());
    let request_description = option_lit_tokens(docs.request_description.as_ref());
    let response_description = option_lit_tokens(docs.response_description.as_ref());
    let response_body = route_options
        .sse_body
        .map(|ty| ResponseBody::Sse(Some(ty)))
        .or_else(|| infer_response_body(&function.sig.output));
    let response_schema = response_schema_descriptor_tokens(response_body.as_ref());

    Ok(quote! {
        #function

        #[doc(hidden)]
        fn #factory<'a>(
            _container: &'a ::auto_route::__private::auto_di::Container,
        ) -> ::auto_route::__private::auto_di::BoxFuture<
            'a,
            ::std::result::Result<
                ::auto_route::__private::axum::Router<()>,
                ::auto_route::__private::auto_di::DiError,
            >,
        > {
            ::std::boxed::Box::pin(async move {
                Ok(::auto_route::__private::axum::Router::new().route(
                    #path,
                    ::auto_route::__private::axum::routing::#method(#handler),
                ))
            })
        }

        ::auto_route::__private::inventory::submit! {
            ::auto_route::RouteDescriptor::new(#factory)
        }

        ::auto_route::__private::inventory::submit! {
            ::auto_route::OpenApiRouteDescriptor::new(
                #method_name,
                #path,
                #operation_id,
                #tag,
                #tag_description,
                #summary,
                #description,
                #params,
                #request_description,
                #request_schema,
                #response_description,
                #response_schema,
            )
        }
    })
}

struct Route {
    method: syn::Ident,
    handler: syn::Ident,
    path: LitStr,
    docs: Docs,
    argument_types: Vec<Type>,
    request_body: Option<RequestBody>,
    response_body: Option<ResponseBody>,
    params: Vec<OpenApiParam>,
    live_event: Option<LitStr>,
    live_return_type: Option<Type>,
}

struct ModuleRoute {
    method: syn::Ident,
    handler: syn::Ident,
    path: LitStr,
    docs: Docs,
    request_body: Option<RequestBody>,
    response_body: Option<ResponseBody>,
    params: Vec<OpenApiParam>,
}

#[derive(Clone)]
struct RequestBody {
    ty: Type,
    content: RequestContent,
}

#[derive(Clone, Copy)]
enum RequestContent {
    Json,
    Form,
}

#[derive(Clone)]
enum ResponseBody {
    Json(Type),
    Sse(Option<Type>),
}

struct OpenApiParam {
    name: String,
    ty: Type,
    source: ParamSource,
}

#[derive(Clone, Copy)]
enum ParamSource {
    Path,
    Query,
}

fn expand_controller_module(
    controller_options: ControllerOptions,
    mut item_mod: ItemMod,
) -> syn::Result<proc_macro2::TokenStream> {
    let module_ident = item_mod.ident.clone();
    let controller_docs = merged_docs(
        &controller_options.docs,
        doc_comment_summary(&item_mod.attrs),
    );
    let Some((_, items)) = &mut item_mod.content else {
        return Err(syn::Error::new_spanned(
            &item_mod,
            "#[controller] requires an inline module: `mod name { ... }`",
        ));
    };
    let mut routes = Vec::new();

    for item in items.iter_mut() {
        let Item::Fn(function) = item else {
            continue;
        };
        let mut route_attributes = Vec::new();
        let mut retained = Vec::new();
        for attribute in std::mem::take(&mut function.attrs) {
            if let Some(method) = route_method(&attribute) {
                route_attributes.push((attribute, method));
            } else {
                retained.push(attribute);
            }
        }
        function.attrs = retained;

        if route_attributes.len() > 1 {
            return Err(syn::Error::new_spanned(
                &function.sig,
                "a module route function can have only one route attribute",
            ));
        }
        let Some((attribute, method)) = route_attributes.pop() else {
            continue;
        };
        validate_route_function(function, "module route functions")?;
        let argument_types = route_argument_types(function.sig.inputs.iter())?;
        let route_options = marker_options(&attribute)?;
        let route_path = route_options.path.value();
        routes.push(ModuleRoute {
            method,
            handler: function.sig.ident.clone(),
            path: LitStr::new(
                &join_paths(&controller_options.path.value(), &route_path),
                attribute.span(),
            ),
            docs: merged_docs(&route_options.docs, doc_comment_summary(&function.attrs)),
            request_body: infer_request_body(&argument_types),
            response_body: route_options
                .sse_body
                .map(|ty| ResponseBody::Sse(Some(ty)))
                .or_else(|| infer_response_body(&function.sig.output)),
            params: infer_params(
                &join_paths(&controller_options.path.value(), &route_path),
                &argument_types,
            ),
        });
    }

    if routes.is_empty() {
        return Err(syn::Error::new_spanned(
            &module_ident,
            "controller module contains no route functions",
        ));
    }

    let registrations = routes.iter().map(|route| {
        let method = &route.method;
        let handler = &route.handler;
        let path = &route.path;
        quote! {
            router = router.route(
                #path,
                ::auto_route::__private::axum::routing::#method(#handler),
            );
        }
    });
    let generated: Item = syn::parse2(quote! {
        #[doc(hidden)]
        fn __auto_route_factory_module<'a>(
            _container: &'a ::auto_route::__private::auto_di::Container,
        ) -> ::auto_route::__private::auto_di::BoxFuture<
            'a,
            ::std::result::Result<
                ::auto_route::__private::axum::Router<()>,
                ::auto_route::__private::auto_di::DiError,
            >,
        > {
            ::std::boxed::Box::pin(async move {
                let mut router = ::auto_route::__private::axum::Router::new();
                #(#registrations)*
                Ok(router)
            })
        }
    })?;
    let submission: Item = syn::parse2(quote! {
        ::auto_route::__private::inventory::submit! {
            ::auto_route::RouteDescriptor::new(__auto_route_factory_module)
        }
    })?;
    let openapi_submissions = routes
        .iter()
        .map(|route| {
            let method = route.method.to_string().to_ascii_uppercase();
            let path = &route.path;
            let operation_id = format!("{}::{}", module_ident, route.handler);
            let tag = route
                .docs
                .tag
                .as_ref()
                .or(controller_docs.tag.as_ref())
                .cloned()
                .unwrap_or_else(|| LitStr::new(&openapi_tag(&path.value()), path.span()));
            let tag_description = option_lit_tokens(
                route
                    .docs
                    .tag_description
                    .as_ref()
                    .or(controller_docs.tag_description.as_ref())
                    .or(controller_docs.description.as_ref()),
            );
            let summary = option_lit_tokens(route.docs.summary.as_ref());
            let description = option_lit_tokens(route.docs.description.as_ref());
            let request_description = option_lit_tokens(route.docs.request_description.as_ref());
            let response_description = option_lit_tokens(route.docs.response_description.as_ref());
            let params = param_descriptor_tokens(&route.params);
            let request_schema = request_schema_descriptor_tokens(route.request_body.as_ref());
            let response_schema = response_schema_descriptor_tokens(route.response_body.as_ref());
            quote! {
                ::auto_route::__private::inventory::submit! {
                    ::auto_route::OpenApiRouteDescriptor::new(
                        #method,
                        #path,
                        #operation_id,
                        #tag,
                        #tag_description,
                        #summary,
                        #description,
                        #params,
                        #request_description,
                        #request_schema,
                        #response_description,
                        #response_schema,
                    )
                }
            }
        })
        .collect::<Vec<_>>();
    items.push(generated);
    items.push(submission);
    for submission in openapi_submissions {
        items.push(syn::parse2(submission)?);
    }
    Ok(quote!(#item_mod))
}

fn expand_controller(
    controller_options: ControllerOptions,
    mut item_impl: ItemImpl,
) -> syn::Result<proc_macro2::TokenStream> {
    if item_impl.trait_.is_some() {
        return Err(syn::Error::new_spanned(
            &item_impl,
            "#[controller] requires an inherent impl block",
        ));
    }
    if !item_impl.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            &item_impl.generics,
            "generic controller impl blocks are not supported",
        ));
    }

    let self_ty = item_impl.self_ty.as_ref().clone();
    let controller_docs = merged_docs(
        &controller_options.docs,
        doc_comment_summary(&item_impl.attrs),
    );
    let has_singleton = item_impl.attrs.iter().any(|attribute| {
        attribute
            .path()
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "singleton")
    });
    let type_ident = type_ident(&self_ty)?;
    let factory_ident = format_ident!("__auto_route_factory_{}", type_ident);
    let mut routes = Vec::new();

    for impl_item in &mut item_impl.items {
        let ImplItem::Fn(function) = impl_item else {
            continue;
        };

        let mut route_attributes = Vec::new();
        let mut retained_attributes = Vec::new();
        let mut is_live = false;
        let mut live_event = None;
        for attribute in std::mem::take(&mut function.attrs) {
            if let Some(method) = route_method(&attribute) {
                route_attributes.push((attribute, method));
            } else if attribute.path().is_ident("live") {
                if !matches!(attribute.meta, Meta::Path(_)) {
                    return Err(syn::Error::new_spanned(&attribute, "use #[live]"));
                }
                is_live = true;
            } else if attribute.path().is_ident("on") {
                live_event = Some(attribute.parse_args::<LitStr>().map_err(|_| {
                    syn::Error::new_spanned(&attribute, "expected #[on(\"event:name\")]")
                })?);
            } else {
                retained_attributes.push(attribute);
            }
        }
        function.attrs = retained_attributes;

        if route_attributes.len() > 1 {
            return Err(syn::Error::new(
                function.sig.span(),
                "a controller method can have only one route attribute",
            ));
        }
        let Some((attribute, method)) = route_attributes.pop() else {
            continue;
        };

        if !is_live && live_event.is_some() {
            return Err(syn::Error::new_spanned(
                &function.sig,
                "controller #[on] requires #[live]",
            ));
        }

        validate_route_signature(&function.sig, "controller route methods")?;

        let mut inputs = function.sig.inputs.iter();
        match inputs.next() {
            Some(FnArg::Receiver(receiver))
                if receiver.reference.is_some() || receiver.colon_token.is_some() => {}
            _ => {
                return Err(syn::Error::new_spanned(
                    &function.sig,
                    "controller route methods must take &self or self: Arc<Self> as their first argument",
                ));
            }
        }

        let argument_types = inputs
            .map(|argument| match argument {
                FnArg::Typed(PatType { ty, .. }) => Ok((**ty).clone()),
                FnArg::Receiver(receiver) => Err(syn::Error::new_spanned(
                    receiver,
                    "unexpected receiver argument",
                )),
            })
            .collect::<syn::Result<Vec<_>>>()?;
        let live_return_type = match &function.sig.output {
            ReturnType::Type(_, ty) if is_live => {
                Some(wrapper_inner_type(ty, &["Json"]).unwrap_or_else(|| (**ty).clone()))
            }
            _ => None,
        };
        if is_live && live_event.is_none() {
            live_event = Some(LitStr::new(
                &function.sig.ident.to_string(),
                function.sig.ident.span(),
            ));
        }
        let route_options = marker_options(&attribute)?;
        let route_path = route_options.path.value();
        let full_path = join_paths(&controller_options.path.value(), &route_path);

        routes.push(Route {
            method,
            handler: function.sig.ident.clone(),
            path: LitStr::new(&full_path, attribute.span()),
            docs: merged_docs(&route_options.docs, doc_comment_summary(&function.attrs)),
            request_body: infer_request_body(&argument_types),
            response_body: route_options
                .sse_body
                .map(|ty| ResponseBody::Sse(Some(ty)))
                .or_else(|| infer_response_body(&function.sig.output)),
            params: infer_params(&full_path, &argument_types),
            argument_types,
            live_event,
            live_return_type,
        });
    }

    if routes.is_empty() {
        return Err(syn::Error::new_spanned(
            &item_impl.self_ty,
            "controller contains no route methods",
        ));
    }

    // ── PATH constants: __PATH_restart = "/apps/restart" ──────────────────────
    let path_const_names: Vec<_> = routes
        .iter()
        .map(|r| format_ident!("__PATH_{}", r.handler))
        .collect();
    let path_const_values: Vec<_> = routes.iter().map(|r| &r.path).collect();

    let controller_name = type_ident.to_string();
    let module_name = controller_name
        .strip_suffix("Controller")
        .unwrap_or(&controller_name)
        .to_ascii_lowercase();
    let live_module_ident = format_ident!("{}_live", module_name);
    let live_handles = routes.iter().filter_map(|route| {
        let event = route.live_event.as_ref()?;
        let return_type = route.live_return_type.as_ref()?;
        let handler = &route.handler;
        let endpoint = format!("{type_ident}::{handler}");
        let arguments = route
            .argument_types
            .iter()
            .enumerate()
            .map(|(index, ty)| {
                let name = format_ident!("arg_{index}");
                let public_ty = live_argument_type(ty);
                quote!(#name: #public_ty)
            })
            .collect::<Vec<_>>();
        let names = (0..route.argument_types.len()).map(|index| format_ident!("arg_{index}")).collect::<Vec<_>>();
        let args = match names.len() {
            0 => quote!(::std::vec::Vec::<()>::new()),
            1 => { let n = &names[0]; quote!((#n,)) },
            _ => quote!((#(#names),*)),
        };
        let event_handler = format_ident!("{}_event", handler);
        let subscription = format_ident!("{}_subscription", handler);
        Some(quote! {
            pub fn #handler(#(#arguments),*)
                -> ::std::result::Result<
                    ::auto_route::__private::auto_socket::LivePublisher<#return_type>,
                    ::auto_route::__private::auto_socket::PublishError,
                >
            {
                ::auto_route::__private::auto_socket::LivePublisher::new(
                    "/_rustploy/live",
                    #endpoint,
                    #event,
                ).room(#args)
            }
            pub fn #event_handler() -> ::auto_route::__private::auto_socket::LivePublisher<#return_type> {
                ::auto_route::__private::auto_socket::LivePublisher::new(
                    "/_rustploy/live", #endpoint, #event,
                )
            }
            pub fn #subscription(#(#arguments),*) -> ::std::result::Result<::auto_route::__private::auto_socket::LiveSubscription<#return_type>, ::auto_route::__private::auto_socket::PublishError> {
                ::auto_route::__private::auto_socket::LiveSubscription::new("/_rustploy/live", #endpoint, #event, #args)
            }
        })
    });

    let registrations = routes.iter().map(|route| {
        let method = &route.method;
        let handler = &route.handler;
        let path = &route.path;
        let arguments = route
            .argument_types
            .iter()
            .enumerate()
            .map(|(index, ty)| {
                let name = format_ident!("__auto_route_arg_{index}");
                quote!(#name: #ty)
            })
            .collect::<Vec<_>>();
        let argument_names = (0..route.argument_types.len())
            .map(|index| format_ident!("__auto_route_arg_{index}"))
            .collect::<Vec<_>>();

        quote! {
            router = router.route(
                #path,
                ::auto_route::__private::axum::routing::#method({
                    let controller = ::std::sync::Arc::clone(&controller);
                    move |#(#arguments),*| {
                        let controller = ::std::sync::Arc::clone(&controller);
                        async move {
                            let response = controller.#handler(#(#argument_names),*).await;
                            ::auto_route::__private::axum::response::IntoResponse::into_response(response)
                        }
                    }
                }),
            );
        }
    });
    let openapi_submissions = routes.iter().map(|route| {
        let method = route.method.to_string().to_ascii_uppercase();
        let path = &route.path;
        let handler = &route.handler;
        let operation_id = format!("{type_ident}::{handler}");
        let tag = route
            .docs
            .tag
            .as_ref()
            .or(controller_docs.tag.as_ref())
            .cloned()
            .unwrap_or_else(|| LitStr::new(&openapi_tag(&path.value()), path.span()));
        let tag_description = option_lit_tokens(
            route
                .docs
                .tag_description
                .as_ref()
                .or(controller_docs.tag_description.as_ref())
                .or(controller_docs.description.as_ref()),
        );
        let summary = option_lit_tokens(route.docs.summary.as_ref());
        let description = option_lit_tokens(route.docs.description.as_ref());
        let request_description = option_lit_tokens(route.docs.request_description.as_ref());
        let response_description = option_lit_tokens(route.docs.response_description.as_ref());
        let params = param_descriptor_tokens(&route.params);
        let request_schema = request_schema_descriptor_tokens(route.request_body.as_ref());
        let response_schema = response_schema_descriptor_tokens(route.response_body.as_ref());
        quote! {
            ::auto_route::__private::inventory::submit! {
                ::auto_route::OpenApiRouteDescriptor::new(
                    #method,
                    #path,
                    #operation_id,
                    #tag,
                    #tag_description,
                    #summary,
                    #description,
                    #params,
                    #request_description,
                    #request_schema,
                    #response_description,
                    #response_schema,
                )
            }
        }
    });

    let managed_impl = if has_singleton {
        quote!(#item_impl)
    } else {
        quote! {
            #[auto_route::__private::auto_di::singleton]
            #item_impl
        }
    };
    let live_socket_registration = if routes.iter().any(|route| route.live_event.is_some()) {
        let live_factory = format_ident!("__auto_route_live_factory_{}", type_ident);
        quote! {
            #[doc(hidden)]
            #[allow(non_snake_case)]
            fn #live_factory<'a>(
                _container: &'a ::auto_route::__private::auto_di::Container,
            ) -> ::auto_route::__private::auto_di::BoxFuture<'a, ::std::result::Result<::auto_route::__private::auto_socket::SocketRegistrar, ::auto_route::__private::auto_di::DiError>> {
                ::std::boxed::Box::pin(async move {
                    let registrar: ::auto_route::__private::auto_socket::SocketRegistrar =
                        ::std::sync::Arc::new(|_socket| {});
                    Ok(registrar)
                })
            }
            ::auto_route::__private::inventory::submit! {
                ::auto_route::__private::auto_socket::SocketDescriptor::new("/_rustploy/live", #live_factory)
            }
        }
    } else {
        quote! {}
    };

    Ok(quote! {
        #managed_impl

        pub mod #live_module_ident {
            use super::*;
            #(#live_handles)*
        }

        #live_socket_registration

        // ── PATH constants for html! on:click={Self::method} ──────────────────
        #[allow(non_upper_case_globals)]
        impl #self_ty {
            #(
                #[doc(hidden)]
                pub const #path_const_names: &'static str = #path_const_values;
            )*
        }

        #[doc(hidden)]
        #[allow(non_snake_case)]
        fn #factory_ident<'a>(
            container: &'a ::auto_route::__private::auto_di::Container,
        ) -> ::auto_route::__private::auto_di::BoxFuture<
            'a,
            ::std::result::Result<
                ::auto_route::__private::axum::Router<()>,
                ::auto_route::__private::auto_di::DiError,
            >,
        > {
            ::std::boxed::Box::pin(async move {
                let controller = container.resolve::<#self_ty>().await?;
                let mut router = ::auto_route::__private::axum::Router::new();
                #(#registrations)*
                Ok(router)
            })
        }

        ::auto_route::__private::inventory::submit! {
            ::auto_route::RouteDescriptor::new(#factory_ident)
        }

        #(#openapi_submissions)*
    })
}

fn validate_route_function(function: &ItemFn, label: &str) -> syn::Result<()> {
    validate_route_signature(&function.sig, label)
}

fn route_argument_types<'a>(inputs: impl Iterator<Item = &'a FnArg>) -> syn::Result<Vec<Type>> {
    inputs
        .map(|argument| match argument {
            FnArg::Typed(PatType { ty, .. }) => Ok((**ty).clone()),
            FnArg::Receiver(receiver) => Err(syn::Error::new_spanned(
                receiver,
                "module route functions cannot take self",
            )),
        })
        .collect()
}

fn infer_request_body(arguments: &[Type]) -> Option<RequestBody> {
    arguments.iter().find_map(|argument| {
        wrapper_inner_type(argument, &["Json", "ValidatedJson"])
            .map(|ty| RequestBody {
                ty,
                content: RequestContent::Json,
            })
            .or_else(|| {
                wrapper_inner_type(argument, &["Form"]).map(|ty| RequestBody {
                    ty,
                    content: RequestContent::Form,
                })
            })
    })
}

fn infer_response_body(output: &ReturnType) -> Option<ResponseBody> {
    match output {
        ReturnType::Default => None,
        ReturnType::Type(_, ty) => response_body_type(ty),
    }
}

fn response_body_type(ty: &Type) -> Option<ResponseBody> {
    if is_sse_response_type(ty) {
        return Some(ResponseBody::Sse(None));
    }

    if let Some(inner) = wrapper_inner_type(ty, &["Json"]) {
        return Some(ResponseBody::Json(inner));
    }

    match ty {
        Type::Paren(paren) => response_body_type(&paren.elem),
        Type::Reference(reference) => response_body_type(&reference.elem),
        Type::Tuple(tuple) => tuple.elems.iter().find_map(response_body_type),
        Type::Path(type_path) => {
            let segment = type_path.path.segments.last()?;
            if segment.ident != "Result" && segment.ident != "Option" {
                return None;
            }
            let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
                return None;
            };
            arguments.args.iter().find_map(|argument| match argument {
                GenericArgument::Type(ty) => response_body_type(ty),
                _ => None,
            })
        }
        _ => None,
    }
}

fn is_sse_response_type(ty: &Type) -> bool {
    match ty {
        Type::Paren(paren) => is_sse_response_type(&paren.elem),
        Type::Reference(reference) => is_sse_response_type(&reference.elem),
        Type::Path(type_path) => type_path.path.segments.last().is_some_and(|segment| {
            let ident = segment.ident.to_string();
            ident == "Sse"
                || ident.ends_with("Sse")
                || ident.ends_with("EventStream")
                || ident.ends_with("EventStreamResponse")
                || ident.ends_with("EventsStream")
                || ident.ends_with("EventsStreamResponse")
        }),
        _ => false,
    }
}

fn infer_params(path: &str, arguments: &[Type]) -> Vec<OpenApiParam> {
    let mut params = infer_path_params(path, arguments);
    params.extend(arguments.iter().filter_map(|argument| {
        wrapper_inner_type(argument, &["Query"]).map(|ty| OpenApiParam {
            name: "query".to_owned(),
            ty,
            source: ParamSource::Query,
        })
    }));
    params
}

fn infer_path_params(path: &str, arguments: &[Type]) -> Vec<OpenApiParam> {
    let names = path_parameter_names(path);
    let path_types = arguments
        .iter()
        .find_map(|argument| wrapper_inner_type(argument, &["Path"]))
        .map(|ty| path_inner_types(&ty))
        .unwrap_or_default();

    names
        .into_iter()
        .enumerate()
        .map(|(index, name)| OpenApiParam {
            name,
            ty: path_types.get(index).cloned().unwrap_or_else(string_type),
            source: ParamSource::Path,
        })
        .collect()
}

fn path_inner_types(ty: &Type) -> Vec<Type> {
    match ty {
        Type::Tuple(tuple) => tuple.elems.iter().cloned().collect(),
        Type::Paren(paren) => path_inner_types(&paren.elem),
        Type::Reference(reference) => path_inner_types(&reference.elem),
        _ => vec![ty.clone()],
    }
}

fn path_parameter_names(path: &str) -> Vec<String> {
    path.split('/')
        .filter_map(|segment| {
            Some(
                segment
                    .strip_prefix('{')?
                    .strip_suffix('}')?
                    .trim_start_matches('*')
                    .to_owned(),
            )
        })
        .collect()
}

fn wrapper_inner_type(ty: &Type, wrappers: &[&str]) -> Option<Type> {
    match ty {
        Type::Paren(paren) => wrapper_inner_type(&paren.elem, wrappers),
        Type::Reference(reference) => wrapper_inner_type(&reference.elem, wrappers),
        Type::Path(type_path) => {
            let segment = type_path.path.segments.last()?;
            let PathArguments::AngleBracketed(arguments) = &segment.arguments else {
                return None;
            };

            if segment.ident == "Option" {
                return arguments.args.iter().find_map(|argument| match argument {
                    GenericArgument::Type(ty) => wrapper_inner_type(ty, wrappers),
                    _ => None,
                });
            }

            if !wrappers.iter().any(|wrapper| segment.ident == wrapper) {
                return None;
            }

            arguments.args.iter().find_map(|argument| match argument {
                GenericArgument::Type(ty) => Some(ty.clone()),
                _ => None,
            })
        }
        _ => None,
    }
}

fn live_argument_type(ty: &Type) -> Type {
    wrapper_inner_type(ty, &["Path", "Query", "Json", "Form"]).unwrap_or_else(|| ty.clone())
}

fn string_type() -> Type {
    syn::parse_quote!(::std::string::String)
}

fn request_schema_descriptor_tokens(body: Option<&RequestBody>) -> proc_macro2::TokenStream {
    match body {
        Some(RequestBody {
            ty,
            content: RequestContent::Json,
        }) => quote! {
            ::std::option::Option::Some(::auto_route::OpenApiSchemaDescriptor::json::<#ty>())
        },
        Some(RequestBody {
            ty,
            content: RequestContent::Form,
        }) => quote! {
            ::std::option::Option::Some(::auto_route::OpenApiSchemaDescriptor::form::<#ty>())
        },
        None => quote!(::std::option::Option::None),
    }
}

fn response_schema_descriptor_tokens(body: Option<&ResponseBody>) -> proc_macro2::TokenStream {
    match body {
        Some(ResponseBody::Json(ty)) => quote! {
            ::std::option::Option::Some(::auto_route::OpenApiSchemaDescriptor::json::<#ty>())
        },
        Some(ResponseBody::Sse(Some(ty))) => quote! {
            ::std::option::Option::Some(::auto_route::OpenApiSchemaDescriptor::sse_json::<#ty>())
        },
        Some(ResponseBody::Sse(None)) => quote! {
            ::std::option::Option::Some(::auto_route::OpenApiSchemaDescriptor::sse())
        },
        None => quote!(::std::option::Option::None),
    }
}

fn param_descriptor_tokens(params: &[OpenApiParam]) -> proc_macro2::TokenStream {
    let params = params.iter().map(|param| {
        let name = LitStr::new(&param.name, proc_macro2::Span::call_site());
        let ty = &param.ty;
        match param.source {
            ParamSource::Path => quote!(::auto_route::OpenApiParamDescriptor::path::<#ty>(#name)),
            ParamSource::Query => quote!(::auto_route::OpenApiParamDescriptor::query::<#ty>(#name)),
        }
    });

    quote!(&[#(#params),*])
}

fn validate_route_signature(signature: &syn::Signature, label: &str) -> syn::Result<()> {
    if signature.asyncness.is_none() {
        return Err(syn::Error::new_spanned(
            signature,
            format!("{label} must be async"),
        ));
    }
    if !signature.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            &signature.generics,
            format!("generic {label} are not supported"),
        ));
    }
    Ok(())
}

fn marker_options(attribute: &Attribute) -> syn::Result<RouteOptions> {
    attribute.parse_args::<RouteOptions>().or_else(|error| {
        if matches!(&attribute.meta, syn::Meta::Path(_)) {
            Ok(RouteOptions::empty())
        } else {
            Err(error)
        }
    })
}

fn merged_docs(explicit: &Docs, fallback_summary: Option<LitStr>) -> Docs {
    let mut docs = explicit.clone();
    if docs.summary.is_none() {
        docs.summary = fallback_summary;
    }
    docs
}

fn option_lit_tokens(value: Option<&LitStr>) -> proc_macro2::TokenStream {
    match value {
        Some(value) => quote!(::std::option::Option::Some(#value)),
        None => quote!(::std::option::Option::None),
    }
}

fn doc_comment_summary(attributes: &[Attribute]) -> Option<LitStr> {
    let mut lines = attributes
        .iter()
        .filter_map(|attribute| {
            if !attribute.path().is_ident("doc") {
                return None;
            }
            let syn::Meta::NameValue(name_value) = &attribute.meta else {
                return None;
            };
            let Expr::Lit(expr_lit) = &name_value.value else {
                return None;
            };
            let Lit::Str(value) = &expr_lit.lit else {
                return None;
            };
            Some(value.value().trim().to_owned())
        })
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    if lines.is_empty() {
        return None;
    }

    let summary = lines.remove(0);
    Some(LitStr::new(&summary, proc_macro2::Span::call_site()))
}

fn route_method(attribute: &Attribute) -> Option<syn::Ident> {
    let ident = attribute.path().get_ident()?;
    METHODS
        .contains(&ident.to_string().as_str())
        .then(|| ident.clone())
}

fn type_ident(ty: &Type) -> syn::Result<&syn::Ident> {
    let Type::Path(path) = ty else {
        return Err(syn::Error::new_spanned(
            ty,
            "controller type must be a named type",
        ));
    };
    path.path
        .segments
        .last()
        .map(|segment| &segment.ident)
        .ok_or_else(|| syn::Error::new_spanned(ty, "controller type must be a named type"))
}

fn join_paths(base: &str, route: &str) -> String {
    let base = base.trim_matches('/');
    let route = route.trim_matches('/');
    let joined = match (base.is_empty(), route.is_empty()) {
        (true, true) => "/".to_owned(),
        (false, true) => format!("/{base}"),
        (true, false) => format!("/{route}"),
        (false, false) => format!("/{base}/{route}"),
    };

    joined
        .split('/')
        .map(|segment| {
            if let Some(parameter) = segment.strip_prefix(':') {
                format!("{{{parameter}}}")
            } else if let Some(parameter) = segment.strip_prefix('*') {
                format!("{{*{parameter}}}")
            } else {
                segment.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("/")
}

fn openapi_tag(path: &str) -> String {
    path.trim_start_matches('/')
        .split('/')
        .next()
        .filter(|segment| !segment.is_empty())
        .unwrap_or("default")
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::join_paths;

    #[test]
    fn joins_and_converts_spring_style_parameters() {
        assert_eq!(join_paths("/users/", "/:id"), "/users/{id}");
        assert_eq!(join_paths("", ""), "/");
        assert_eq!(join_paths("/files", "/*path"), "/files/{*path}");
    }
}
