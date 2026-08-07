use crate::convert::{convert_expr, convert_macro};
use crate::parser::ShStmt;
use quote::quote;
use syn::{Expr, Pat, Stmt};

/// Entry point for ShStmt (top-level, supports untyped `fn` params)
pub fn convert_sh_stmt(sh_stmt: &ShStmt) -> Result<proc_macro2::TokenStream, syn::Error> {
    match sh_stmt {
        ShStmt::ShFn { name, params, body } => {
            let mut body_stmts = Vec::new();
            for s in body {
                body_stmts.push(convert_sh_stmt(s)?);
            }
            Ok(quote! {
                (crate::utils::exec::script::dsl::ShellIR::Function {
                    name: #name.to_string(),
                    params: vec![ #( #params.to_string() ),* ],
                    body: vec![ #( #body_stmts ),* ],
                })
            })
        }
        ShStmt::Syn(stmt) => convert_stmt(stmt),
    }
}

pub fn convert_stmt(stmt: &syn::Stmt) -> Result<proc_macro2::TokenStream, syn::Error> {
    match stmt {
        Stmt::Local(local) => {
            let name_ident = match &local.pat {
                Pat::Ident(pat_ident) => &pat_ident.ident,
                _ => {
                    return Err(syn::Error::new_spanned(
                        &local.pat,
                        "Expected variable name",
                    ));
                }
            };
            let name_str = name_ident.to_string();

            let mut val_expr = match &local.init {
                Some(local_init) => &local_init.expr,
                None => {
                    return Err(syn::Error::new_spanned(
                        local,
                        "Variable must be initialized",
                    ));
                }
            };

            // Check if RHS is a closure
            if let Expr::Closure(closure) = &**val_expr {
                let mut params = Vec::new();
                for param in &closure.inputs {
                    if let Pat::Ident(pat_ident) = param {
                        params.push(pat_ident.ident.to_string());
                    } else {
                        return Err(syn::Error::new_spanned(
                            param,
                            "Expected identifier for closure parameter",
                        ));
                    }
                }
                let mut body_stmts = Vec::new();
                match &*closure.body {
                    Expr::Block(block) => {
                        for stmt in &block.block.stmts {
                            body_stmts.push(convert_stmt(stmt)?);
                        }
                    }
                    _ => {
                        return Err(syn::Error::new_spanned(
                            &closure.body,
                            "Expected block for closure body",
                        ));
                    }
                }
                return Ok(quote! {
                    (crate::utils::exec::script::dsl::ShellIR::Function {
                        name: #name_str.to_string(),
                        params: vec![ #( #params.to_string() ),* ],
                        body: vec![ #( #body_stmts ),* ],
                    })
                });
            }

            let mut default_tokens = quote! { None };
            if let Expr::MethodCall(method_call) = &**val_expr {
                if method_call.method == "default" {
                    let arg = method_call.args.first().ok_or_else(|| {
                        syn::Error::new_spanned(method_call, "default method needs one argument")
                    })?;
                    default_tokens = quote! { Some((#arg).to_string()) };
                    val_expr = &method_call.receiver;
                }
            }

            let val_tokens = convert_expr(val_expr)?;

            Ok(quote! {
                (crate::utils::exec::script::dsl::ShellIR::Statement(
                    crate::utils::exec::script::dsl::Statement::VarAssign {
                        name: #name_str.to_string(),
                        val: Box::new((#val_tokens)),
                        default: #default_tokens,
                    }
                ))
            })
        }
        Stmt::Expr(expr, _) => {
            if let Expr::Path(expr_path) = expr {
                Ok(quote! { #expr_path })
            } else {
                match convert_expr(expr) {
                    Ok(tokens) => Ok(tokens),
                    Err(_) => Ok(quote! { #expr }),
                }
            }
        }
        Stmt::Macro(stmt_macro) => convert_macro(&stmt_macro.mac),
        Stmt::Item(syn::Item::Fn(item_fn)) => {
            let func_name = item_fn.sig.ident.to_string();
            let mut params = Vec::new();
            for input in &item_fn.sig.inputs {
                if let syn::FnArg::Typed(pat_type) = input {
                    if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                        params.push(pat_ident.ident.to_string());
                    }
                }
            }

            let mut body_stmts = Vec::new();
            for stmt in &item_fn.block.stmts {
                body_stmts.push(convert_stmt(stmt)?);
            }

            Ok(quote! {
                (crate::utils::exec::script::dsl::ShellIR::Function {
                    name: #func_name.to_string(),
                    params: vec![ #( #params.to_string() ),* ],
                    body: vec![ #( #body_stmts ),* ],
                })
            })
        }
        Stmt::Item(_) => Err(syn::Error::new_spanned(
            stmt,
            "Unsupported item in sh! block",
        )),
    }
}
