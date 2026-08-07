use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    FnArg, Ident, ImplItem, Item, ItemFn, ItemImpl, ItemMod, LitStr, PatType, Token, Type,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

/// Declares an auto-di managed socket event handler group for one namespace.
#[proc_macro_attribute]
pub fn auto_socket(attr: TokenStream, item: TokenStream) -> TokenStream {
    let namespace = parse_macro_input!(attr as LitStr);
    let item = parse_macro_input!(item as Item);
    let expanded = match item {
        Item::Impl(item_impl) => expand_socket_impl(namespace, item_impl),
        Item::Mod(item_mod) => expand_socket_module(namespace, item_mod),
        other => Err(syn::Error::new_spanned(
            other,
            "#[auto_socket] must be placed on an inherent impl or inline module",
        )),
    };
    expanded
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Registers a standalone Socketioxide event handler.
///
/// Examples: `#[on("message")]` for the default `/` namespace, or
/// `#[on("message", namespace = "/chat")]` for an explicit namespace.
#[proc_macro_attribute]
pub fn on(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as StandaloneOnArgs);
    let function = parse_macro_input!(item as ItemFn);
    expand_standalone_on(args, function)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

struct Event {
    name: LitStr,
    handler: Ident,
    argument_types: Vec<Type>,
}

fn expand_socket_module(
    namespace: LitStr,
    mut item_mod: ItemMod,
) -> syn::Result<proc_macro2::TokenStream> {
    let module_ident = item_mod.ident.clone();
    let Some((_, items)) = &mut item_mod.content else {
        return Err(syn::Error::new_spanned(
            &item_mod,
            "#[auto_socket] requires an inline module: `mod name { ... }`",
        ));
    };
    let mut events = Vec::new();

    for item in items.iter_mut() {
        let Item::Fn(function) = item else {
            continue;
        };
        let mut on_attributes = Vec::new();
        let mut retained = Vec::new();
        for attribute in std::mem::take(&mut function.attrs) {
            if attribute.path().is_ident("on") {
                on_attributes.push(attribute);
            } else {
                retained.push(attribute);
            }
        }
        function.attrs = retained;

        if on_attributes.len() > 1 {
            return Err(syn::Error::new_spanned(
                &function.sig,
                "a module socket function can have only one #[on] attribute",
            ));
        }
        let Some(attribute) = on_attributes.pop() else {
            continue;
        };
        validate_async(&function.sig)?;
        events.push((
            attribute.parse_args::<LitStr>()?,
            function.sig.ident.clone(),
        ));
    }

    if events.is_empty() {
        return Err(syn::Error::new_spanned(
            &module_ident,
            "socket module contains no #[on] functions",
        ));
    }

    let registrations = events.iter().map(|(event, handler)| {
        quote! { socket.on(#event, #handler); }
    });
    let generated: Item = syn::parse2(quote! {
        #[doc(hidden)]
        fn __auto_socket_factory_module<'a>(
            _container: &'a ::auto_socket::__private::auto_di::Container,
        ) -> ::auto_socket::__private::auto_di::BoxFuture<
            'a,
            ::std::result::Result<
                ::auto_socket::SocketRegistrar,
                ::auto_socket::__private::auto_di::DiError,
            >,
        > {
            ::std::boxed::Box::pin(async move {
                let registrar: ::auto_socket::SocketRegistrar =
                    ::std::sync::Arc::new(move |socket| {
                        #(#registrations)*
                    });
                Ok(registrar)
            })
        }
    })?;
    let submission: Item = syn::parse2(quote! {
        ::auto_socket::__private::inventory::submit! {
            ::auto_socket::SocketDescriptor::new(#namespace, __auto_socket_factory_module)
        }
    })?;
    items.push(generated);
    items.push(submission);
    Ok(quote!(#item_mod))
}

fn expand_socket_impl(
    namespace: LitStr,
    mut item_impl: ItemImpl,
) -> syn::Result<proc_macro2::TokenStream> {
    if item_impl.trait_.is_some() {
        return Err(syn::Error::new_spanned(
            &item_impl,
            "#[auto_socket] requires an inherent impl block",
        ));
    }
    if !item_impl.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            &item_impl.generics,
            "generic socket handler impl blocks are not supported",
        ));
    }

    let self_ty = item_impl.self_ty.as_ref().clone();
    let type_ident = type_ident(&self_ty)?;
    let factory = format_ident!("__auto_socket_factory_{}", type_ident);
    let mut events = Vec::new();

    for item in &mut item_impl.items {
        let ImplItem::Fn(function) = item else {
            continue;
        };
        let mut on_attributes = Vec::new();
        let mut retained = Vec::new();
        for attribute in std::mem::take(&mut function.attrs) {
            if attribute.path().is_ident("on") {
                on_attributes.push(attribute);
            } else {
                retained.push(attribute);
            }
        }
        function.attrs = retained;

        if on_attributes.len() > 1 {
            return Err(syn::Error::new_spanned(
                &function.sig,
                "a socket method can have only one #[on] attribute",
            ));
        }
        let Some(attribute) = on_attributes.pop() else {
            continue;
        };
        validate_async(&function.sig)?;

        let mut inputs = function.sig.inputs.iter();
        match inputs.next() {
            Some(FnArg::Receiver(receiver))
                if receiver.reference.is_some() && receiver.mutability.is_none() => {}
            _ => {
                return Err(syn::Error::new_spanned(
                    &function.sig,
                    "socket event methods must take &self as their first argument",
                ));
            }
        }
        let argument_types = typed_arguments(inputs)?;
        events.push(Event {
            name: attribute.parse_args::<LitStr>()?,
            handler: function.sig.ident.clone(),
            argument_types,
        });
    }

    if events.is_empty() {
        return Err(syn::Error::new_spanned(
            &item_impl.self_ty,
            "socket handler contains no #[on] methods",
        ));
    }

    let registrations = events.iter().map(event_registration);
    Ok(quote! {
        #item_impl

        #[doc(hidden)]
        #[allow(non_snake_case)]
        fn #factory<'a>(
            container: &'a ::auto_socket::__private::auto_di::Container,
        ) -> ::auto_socket::__private::auto_di::BoxFuture<
            'a,
            ::std::result::Result<
                ::auto_socket::SocketRegistrar,
                ::auto_socket::__private::auto_di::DiError,
            >,
        > {
            ::std::boxed::Box::pin(async move {
                let controller = container.resolve::<#self_ty>().await?;
                let registrar: ::auto_socket::SocketRegistrar =
                    ::std::sync::Arc::new(move |socket| {
                        #(#registrations)*
                    });
                Ok(registrar)
            })
        }

        ::auto_socket::__private::inventory::submit! {
            ::auto_socket::SocketDescriptor::new(#namespace, #factory)
        }
    })
}

fn event_registration(event: &Event) -> proc_macro2::TokenStream {
    let name = &event.name;
    let handler = &event.handler;
    let arguments = event
        .argument_types
        .iter()
        .enumerate()
        .map(|(index, ty)| {
            let argument = format_ident!("__auto_socket_arg_{index}");
            quote!(#argument: #ty)
        })
        .collect::<Vec<_>>();
    let argument_names = (0..event.argument_types.len())
        .map(|index| format_ident!("__auto_socket_arg_{index}"))
        .collect::<Vec<_>>();

    quote! {
        socket.on(#name, {
            let controller = ::std::sync::Arc::clone(&controller);
            move |#(#arguments),*| {
                let controller = ::std::sync::Arc::clone(&controller);
                async move { controller.#handler(#(#argument_names),*).await }
            }
        });
    }
}

struct StandaloneOnArgs {
    event: LitStr,
    namespace: LitStr,
}

impl Parse for StandaloneOnArgs {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let event = input.parse()?;
        if input.is_empty() {
            return Ok(Self {
                event,
                namespace: LitStr::new("/", proc_macro2::Span::call_site()),
            });
        }

        input.parse::<Token![,]>()?;
        let key: Ident = input.parse()?;
        if key != "namespace" {
            return Err(syn::Error::new_spanned(key, "expected `namespace`"));
        }
        input.parse::<Token![=]>()?;
        let namespace = input.parse()?;
        Ok(Self { event, namespace })
    }
}

fn expand_standalone_on(
    args: StandaloneOnArgs,
    function: ItemFn,
) -> syn::Result<proc_macro2::TokenStream> {
    validate_async(&function.sig)?;
    let handler = &function.sig.ident;
    let factory = format_ident!("__auto_socket_factory_{}", handler);
    let event = args.event;
    let namespace = args.namespace;

    Ok(quote! {
        #function

        #[doc(hidden)]
        fn #factory<'a>(
            _container: &'a ::auto_socket::__private::auto_di::Container,
        ) -> ::auto_socket::__private::auto_di::BoxFuture<
            'a,
            ::std::result::Result<
                ::auto_socket::SocketRegistrar,
                ::auto_socket::__private::auto_di::DiError,
            >,
        > {
            ::std::boxed::Box::pin(async move {
                let registrar: ::auto_socket::SocketRegistrar =
                    ::std::sync::Arc::new(move |socket| socket.on(#event, #handler));
                Ok(registrar)
            })
        }

        ::auto_socket::__private::inventory::submit! {
            ::auto_socket::SocketDescriptor::new(#namespace, #factory)
        }
    })
}

fn validate_async(signature: &syn::Signature) -> syn::Result<()> {
    if signature.asyncness.is_none() {
        return Err(syn::Error::new_spanned(
            signature,
            "socket handlers must be async",
        ));
    }
    if !signature.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            &signature.generics,
            "generic socket handlers are not supported",
        ));
    }
    Ok(())
}

fn typed_arguments<'a>(inputs: impl Iterator<Item = &'a FnArg>) -> syn::Result<Vec<Type>> {
    inputs
        .map(|argument| match argument {
            FnArg::Typed(PatType { ty, .. }) => Ok((**ty).clone()),
            FnArg::Receiver(receiver) => Err(syn::Error::new_spanned(
                receiver,
                "unexpected receiver argument",
            )),
        })
        .collect()
}

fn type_ident(ty: &Type) -> syn::Result<&Ident> {
    let Type::Path(path) = ty else {
        return Err(syn::Error::new_spanned(
            ty,
            "socket handler type must be a named type",
        ));
    };
    path.path
        .segments
        .last()
        .map(|segment| &segment.ident)
        .ok_or_else(|| syn::Error::new_spanned(ty, "socket handler type must be a named type"))
}
