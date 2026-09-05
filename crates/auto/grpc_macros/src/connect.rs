use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{Attribute, Expr, Ident, ItemImpl, Stmt};

struct PermissionAttr {
    resource: Ident,
    operation: Ident,
}

fn parse_permission_attr(attrs: &[Attribute]) -> Option<PermissionAttr> {
    for attr in attrs {
        if attr.path().is_ident("permission") {
            let meta = attr.meta.require_list().ok()?;
            let tokens = meta.tokens.clone();
            if let Ok(parsed) = syn::parse2::<Expr>(tokens) {
                match &parsed {
                    Expr::Tuple(tuple) if tuple.elems.len() == 2 => {
                        let resource = match &tuple.elems[0] {
                            Expr::Path(p) => p.path.segments.last()?.ident.clone(),
                            _ => return None,
                        };
                        let operation = match &tuple.elems[1] {
                            Expr::Path(p) => p.path.segments.last()?.ident.clone(),
                            _ => return None,
                        };
                        return Some(PermissionAttr { resource, operation });
                    }
                    Expr::Path(p) => {
                        let segs = &p.path.segments;
                        if segs.len() >= 2 {
                            return Some(PermissionAttr {
                                resource: segs[0].ident.clone(),
                                operation: segs[1].ident.clone(),
                            });
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    None
}

/// Find the name of the `RequestContext` parameter in a method signature.
fn find_ctx_param_name(sig: &syn::Signature) -> Option<Ident> {
    for arg in &sig.inputs {
        if let syn::FnArg::Typed(pat) = arg {
            if let syn::Type::Path(tp) = &*pat.ty {
                if tp.path.segments.last().map_or(false, |s| s.ident == "RequestContext") {
                    if let syn::Pat::Ident(ident) = &*pat.pat {
                        return Some(ident.ident.clone());
                    }
                }
            }
        }
    }
    None
}

fn inject_auth_into_impl(impl_block: &mut syn::ImplItemFn, auth: bool) {
    if !auth {
        return;
    }

    // Public RPCs (signup/login/refresh/...) opt out with #[public]: no
    // auth extraction, no permission check. The attribute is a directive
    // for this macro only and is stripped from the output.
    if impl_block.attrs.iter().any(|a| a.path().is_ident("public")) {
        impl_block.attrs.retain(|a| !a.path().is_ident("public"));
        return;
    }

    let ctx_ident = match find_ctx_param_name(&impl_block.sig) {
        Some(ident) => ident,
        None => return,
    };

    let mut injected: Vec<Stmt> = Vec::new();

    // 1. Auto-extract auth: let auth = Self::extract_auth(&ctx).await?;
    //    If the ctx param name starts with '_' (e.g. _ctx), use _auth to avoid unused warnings.
    let ctx_name = ctx_ident.to_string();
    let auth_ident = if ctx_name.starts_with('_') {
        format_ident!("_auth")
    } else {
        format_ident!("auth")
    };
    let auth_stmt: Stmt = syn::parse_quote! {
        let #auth_ident = Self::extract_auth(&#ctx_ident).await?;
    };
    injected.push(auth_stmt);

    // 2. Auto-permission check if #[permission(Resource, Operation)] is present
    if let Some(perm) = parse_permission_attr(&impl_block.attrs) {
        let resource = perm.resource;
        let operation = perm.operation;
        let perm_stmt: Stmt = syn::parse_quote! {
            auth.require_permission(
                crate::services::permission::#resource::NAME,
                crate::services::permission::#operation::NAME,
            ).await.map_err(|e| e.into_connect_error())?;
        };
        injected.push(perm_stmt);
    }

    // Remove ALL #[permission] attributes from the output
    impl_block.attrs.retain(|a| !a.path().is_ident("permission"));

    // Prepend the injected statements to the method body
    let body_stmts = std::mem::take(&mut impl_block.block.stmts);
    impl_block.block.stmts = injected;
    impl_block.block.stmts.extend(body_stmts);
}

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

    let mut modified_item = item;
    if auth {
        for impl_item in &mut modified_item.items {
            if let syn::ImplItem::Fn(method) = impl_item {
                inject_auth_into_impl(method, true);
            }
        }
    }

    let auth_impl = if auth {
        quote! {
            impl #service_type {
                /// Extract a validated [`AuthContext`] from the ConnectRPC request.
                ///
                /// Called automatically by `#[auto_grpc::service(auth = true)]` —
                /// handlers receive `auth` as a local variable without calling this.
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
        #modified_item

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
