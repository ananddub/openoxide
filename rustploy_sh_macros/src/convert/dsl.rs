use quote::quote;
use syn::ExprMethodCall;

fn is_command_receiver(expr: &syn::Expr) -> bool {
    match expr {
        syn::Expr::Path(expr_path) => expr_path
            .path
            .get_ident()
            .map(|id| id == "os" || id == "docker")
            .unwrap_or(false),
        syn::Expr::MethodCall(method_call) => is_command_receiver(&method_call.receiver),
        syn::Expr::Field(expr_field) => is_command_receiver(&expr_field.base),
        _ => false,
    }
}

fn is_special_method_name(name: &str) -> bool {
    matches!(
        name,
        "stdout" | "stderr" | "sudo" | "success" | "failure" | "ok" | "empty" | "non_empty"
    )
}

fn is_special_method_call(expr: &syn::Expr) -> bool {
    matches!(
        expr,
        syn::Expr::MethodCall(method_call)
            if is_special_method_name(&method_call.method.to_string())
    )
}

/// Returns true if `name` was declared inside the current `sh!` block.
fn is_sh_var(name: &str) -> bool {
    crate::SH_VARS.with(|cell| cell.borrow().contains(name))
}

/// Walk an `os.*` method-call chain and replace every argument that is a plain
/// identifier declared in the sh! scope with its shell-variable form `"$varname"`.
/// Identifiers NOT in sh! scope (outer Rust variables) are left as-is so the
/// Rust compiler can resolve them normally.
fn substitute_sh_vars(expr: &syn::Expr) -> syn::Expr {
    match expr {
        syn::Expr::MethodCall(mc) => {
            let new_receiver = Box::new(substitute_sh_vars(&mc.receiver));

            let new_args: syn::punctuated::Punctuated<syn::Expr, syn::Token![,]> = mc
                .args
                .iter()
                .map(|arg| match arg {
                    syn::Expr::Path(ep) => {
                        if let Some(ident) = ep.path.get_ident() {
                            // Only substitute identifiers that are sh!-declared variables
                            if ident != "os" && ident != "docker" && is_sh_var(&ident.to_string()) {
                                let shell_var = format!("${}", ident);
                                return syn::Expr::Lit(syn::ExprLit {
                                    attrs: vec![],
                                    lit: syn::Lit::Str(syn::LitStr::new(&shell_var, ident.span())),
                                });
                            }
                        }
                        arg.clone()
                    }
                    syn::Expr::Macro(em) => {
                        let macro_name = em
                            .mac
                            .path
                            .get_ident()
                            .map(|i| i.to_string())
                            .unwrap_or_default();
                        if macro_name == "rust" {
                            let parser = <syn::Expr as syn::parse::Parse>::parse;
                            if let Ok(inner) = em.mac.parse_body_with(parser) {
                                return inner;
                            }
                        }
                        substitute_sh_vars(arg)
                    }
                    _ => substitute_sh_vars(arg),
                })
                .collect();

            syn::Expr::MethodCall(syn::ExprMethodCall {
                attrs: mc.attrs.clone(),
                receiver: new_receiver,
                dot_token: mc.dot_token,
                method: mc.method.clone(),
                turbofish: mc.turbofish.clone(),
                paren_token: mc.paren_token,
                args: new_args,
            })
        }
        other => other.clone(),
    }
}

pub fn convert_method_call(
    method_call: &ExprMethodCall,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let receiver = &*method_call.receiver;
    let method_name = method_call.method.to_string();
    let is_special_method = is_special_method_name(&method_name);

    let receiver_tokens = if is_special_method {
        if is_special_method_call(receiver) {
            crate::convert::convert_expr(receiver)?
        } else if is_command_receiver(receiver) {
            let subst = substitute_sh_vars(receiver);
            quote! {
                (crate::utils::exec::script::dsl::IntoShellIRGeneric::into_shell_ir(
                    crate::utils::exec::script::dsl::ShellIRWrapper(#subst)
                ))
            }
        } else {
            crate::convert::convert_expr(receiver)?
        }
    } else {
        if is_command_receiver(receiver) {
            let subst = substitute_sh_vars(receiver);
            quote! { #subst }
        } else {
            crate::convert::convert_expr(receiver)?
        }
    };

    if method_name == "stdout" {
        if method_call.args.is_empty() {
            return Ok(quote! {
                (match #receiver_tokens {
                    crate::utils::exec::script::dsl::ShellIR::Expr(crate::utils::exec::script::dsl::Expr::Variable(v)) => {
                        crate::utils::exec::script::dsl::ShellIR::Expr(
                            crate::utils::exec::script::dsl::Expr::Variable(format!("{}_stdout", v))
                        )
                    }
                    _ => panic!("stdout() getter can only be called on a variable reference"),
                })
            });
        }
        let arg = method_call.args.first().ok_or_else(|| {
            syn::Error::new_spanned(method_call, "stdout method needs one argument")
        })?;
        let arg_tokens = crate::convert::convert_expr(arg)?;
        return Ok(quote! {
            (#receiver_tokens).stdout(
                match #arg_tokens {
                    crate::utils::exec::script::dsl::ShellIR::Expr(crate::utils::exec::script::dsl::Expr::Literal(l)) => l,
                    _ => panic!("stdout redirect target must be a string literal"),
                }
            )
        });
    }

    if method_name == "stderr" {
        if method_call.args.is_empty() {
            return Ok(quote! {
                (match #receiver_tokens {
                    crate::utils::exec::script::dsl::ShellIR::Expr(crate::utils::exec::script::dsl::Expr::Variable(v)) => {
                        crate::utils::exec::script::dsl::ShellIR::Expr(
                            crate::utils::exec::script::dsl::Expr::Variable(format!("{}_stderr", v))
                        )
                    }
                    _ => panic!("stderr() getter can only be called on a variable reference"),
                })
            });
        }
        let arg = method_call.args.first().ok_or_else(|| {
            syn::Error::new_spanned(method_call, "stderr method needs one argument")
        })?;
        let arg_tokens = crate::convert::convert_expr(arg)?;
        return Ok(quote! {
            (#receiver_tokens).stderr(
                match #arg_tokens {
                    crate::utils::exec::script::dsl::ShellIR::Expr(crate::utils::exec::script::dsl::Expr::Literal(l)) => l,
                    _ => panic!("stderr redirect target must be a string literal"),
                }
            )
        });
    }

    if method_name == "sudo" {
        return Ok(quote! { (#receiver_tokens).sudo() });
    }

    if method_name == "success" {
        return Ok(quote! { (#receiver_tokens).success() });
    }

    if method_name == "failure" {
        return Ok(quote! { (#receiver_tokens).failure() });
    }

    if method_name == "ok" {
        return Ok(quote! { (#receiver_tokens).ok() });
    }

    if method_name == "empty" {
        return Ok(quote! { (#receiver_tokens).empty() });
    }

    if method_name == "non_empty" {
        return Ok(quote! { (#receiver_tokens).non_empty() });
    }

    // Non-special OS chain methods: substitute sh vars in args too
    let mut args_tokens = Vec::new();
    for arg in &method_call.args {
        if let syn::Expr::Path(expr_path) = arg {
            let ident = expr_path.path.get_ident();
            let is_command_root = ident
                .map(|id| id == "os" || id == "docker")
                .unwrap_or(false);
            if is_command_root {
                args_tokens.push(quote! { #arg });
            } else if let Some(ident) = ident {
                if is_sh_var(&ident.to_string()) {
                    // sh!-declared variable → shell "$var" string
                    let shell_var = format!("${}", ident);
                    args_tokens.push(quote! { #shell_var });
                } else {
                    // outer Rust variable → pass through as-is
                    args_tokens.push(quote! { #arg });
                }
            } else {
                args_tokens.push(quote! { #arg });
            }
        } else if let syn::Expr::Macro(_) = arg {
            args_tokens.push(crate::convert::convert_expr(arg)?);
        } else {
            args_tokens.push(quote! { #arg });
        }
    }
    let method_ident = &method_call.method;
    Ok(quote! {
        (crate::utils::exec::script::dsl::ShellIRWrapper(
            (#receiver_tokens).#method_ident( #( #args_tokens ),* )
        ).into_shell_ir())
    })
}
