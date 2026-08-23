use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{Expr, ExprArray, ItemImpl, Path, Token, parse_macro_input};

struct Options {
    middleware: Vec<Path>,
    server: Option<Path>,
}

#[proc_macro_attribute]
pub fn middleware(attr: TokenStream, item: TokenStream) -> TokenStream {
    let order = parse_macro_input!(attr as syn::ExprAssign);
    let item_impl = parse_macro_input!(item as ItemImpl);
    let value = match *order.right {
        Expr::Lit(expr) => expr,
        other => {
            return syn::Error::new_spanned(other, "middleware order must be an integer")
                .to_compile_error()
                .into();
        }
    };
    let self_ty = item_impl.self_ty.clone();
    quote! {
        #[::auto_grpc::__private::auto_di::singleton]
        #item_impl
        impl ::auto_grpc::MiddlewareOrder for #self_ty {
            const ORDER: i32 = #value;
        }
    }
    .into()
}

impl syn::parse::Parse for Options {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let mut middleware = Vec::new();
        let mut server = None;
        while !input.is_empty() {
            let key: syn::Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            if key == "middleware" {
                let array: ExprArray = input.parse()?;
                for expr in array.elems {
                    match expr {
                        Expr::Path(path) => middleware.push(path.path),
                        other => {
                            return Err(syn::Error::new_spanned(
                                other,
                                "middleware entries must be types",
                            ));
                        }
                    }
                }
            } else if key == "server" {
                server = Some(input.parse()?);
            } else {
                return Err(syn::Error::new_spanned(key, "unknown auto_grpc option"));
            }
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(Self { middleware, server })
    }
}

#[proc_macro_attribute]
pub fn service(attr: TokenStream, item: TokenStream) -> TokenStream {
    let options = parse_macro_input!(attr as Options);
    let item_impl = parse_macro_input!(item as ItemImpl);
    match expand(options, item_impl) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn expand(options: Options, item: ItemImpl) -> syn::Result<proc_macro2::TokenStream> {
    let trait_path = item
        .trait_
        .as_ref()
        .ok_or_else(|| {
            syn::Error::new(
                Span::call_site(),
                "#[auto_grpc::service] requires a trait impl",
            )
        })?
        .1
        .clone();
    let trait_ident = trait_path.segments.last().unwrap().ident.clone();
    let service_ident = match &*item.self_ty {
        syn::Type::Path(path) => path.path.segments.last().unwrap().ident.clone(),
        other => {
            return Err(syn::Error::new_spanned(
                other,
                "gRPC service must be a named type",
            ));
        }
    };
    let server_path = options.server.unwrap_or_else(|| {
        let ident = format_ident!("{}Server", trait_ident);
        syn::parse_quote!(#ident)
    });
    let registrar = format_ident!(
        "__auto_grpc_register_{}",
        service_ident.to_string().to_lowercase()
    );
    let resolves = options.middleware.iter().enumerate().map(|(index, path)| {
        let var = format_ident!("middleware_{index}");
        quote! { let #var = container.resolve::<#path>().await?; }
    });
    let vars: Vec<_> = options
        .middleware
        .iter()
        .enumerate()
        .map(|(index, _)| format_ident!("middleware_{index}"))
        .collect();
    let service = quote! { container.resolve::<#service_ident>().await? };
    let registration = if vars.is_empty() {
        quote! { builder.add_service(#server_path::new((*service).clone())); }
    } else {
        let first = &vars[0];
        let chain = vars.iter().skip(1).fold(
            quote! { #first.authenticate(request) },
            |acc, middleware| quote! { #middleware.authenticate(#acc?) },
        );
        quote! {
            builder.add_service(#server_path::with_interceptor((*service).clone(), move |request| {
                #chain
            }));
        }
    };
    Ok(quote! {
        #[tonic::async_trait]
        #item
        fn #registrar<'a>(container: &'a ::auto_grpc::__private::auto_di::Container, builder: &'a mut ::auto_grpc::__private::tonic::service::RoutesBuilder) -> ::auto_grpc::__private::auto_di::BoxFuture<'a, Result<(), ::auto_grpc::__private::auto_di::DiError>> {
            Box::pin(async move {
                #(#resolves)*
                let service = #service;
                #registration
                Ok(())
            })
        }
        ::auto_grpc::__private::inventory::submit! {
            ::auto_grpc::Descriptor { registrar: #registrar }
        }
    })
}
