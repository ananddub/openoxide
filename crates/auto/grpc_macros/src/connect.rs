use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::ItemImpl;

pub(crate) fn expand_connect(item: ItemImpl, auth: bool) -> syn::Result<proc_macro2::TokenStream> {
    let trait_path = item
        .trait_
        .as_ref()
        .ok_or_else(|| {
            syn::Error::new(
                Span::call_site(),
                "#[auto_grpc::service(connect = true)] requires a trait impl",
            )
        })?
        .1
        .clone();
    let service_type = item.self_ty.clone();
    let service_ident = match service_type.as_ref() {
        syn::Type::Path(path) => path.path.segments.last().unwrap().ident.clone(),
        other => {
            return Err(syn::Error::new_spanned(
                other,
                "Connect service must be a named type",
            ));
        }
    };
    let mut extension_path = trait_path;
    let trait_name = extension_path.segments.last_mut().unwrap();
    trait_name.ident = format_ident!("{}Ext", trait_name.ident);
    let registrar = format_ident!(
        "__auto_grpc_connect_register_{}",
        service_ident.to_string().to_lowercase()
    );

    // When auth = true, generate an inherent impl with extract_auth helper
    // and require_permission shorthand so RPC methods can write:
    //   let auth = Self::extract_auth(&ctx).await?;
    //   auth.require_permission("resource", "operation").await?;
    let auth_impl = if auth {
        quote! {
            impl #service_type {
                /// Extract a validated [`AuthContext`] from the ConnectRPC request.
                ///
                /// Returns a `ConnectError` if the caller is not authenticated
                /// or has no organization membership.
                pub async fn extract_auth(
                    ctx: &::auto_grpc::__private::connectrpc::RequestContext,
                ) -> Result<
                    crate::core::middleware::auth_context::AuthContext,
                    ::auto_grpc::__private::connectrpc::ConnectError,
                > {
                    crate::core::middleware::auth_context::AuthContext::from_request_context(ctx).await
                }
            }
        }
    } else {
        quote! {}
    };

    Ok(quote! {
        #item

        #auth_impl

        fn #registrar<'a>(
            container: &'a ::auto_grpc::__private::auto_di::Container,
            router: ::auto_grpc::__private::connectrpc::Router,
        ) -> ::auto_grpc::__private::auto_di::BoxFuture<
            'a,
            Result<
                ::auto_grpc::__private::connectrpc::Router,
                ::auto_grpc::__private::auto_di::DiError,
            >,
        > {
            Box::pin(async move {
                let service = container.resolve::<#service_type>().await?;
                Ok(<#service_type as #extension_path>::register(service, router))
            })
        }

        ::auto_grpc::__private::inventory::submit! {
            ::auto_grpc::ConnectDescriptor { registrar: #registrar }
        }
    })
}
