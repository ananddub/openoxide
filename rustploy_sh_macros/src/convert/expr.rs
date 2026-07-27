use crate::convert::dsl::convert_method_call;
use crate::convert::{convert_macro, convert_stmt};
use quote::quote;
use syn::spanned::Spanned;
use syn::{Expr, Pat};

pub fn convert_expr(expr: &syn::Expr) -> Result<proc_macro2::TokenStream, syn::Error> {
    match expr {
        Expr::Call(expr_call) => {
            let func_name = match &*expr_call.func {
                Expr::Path(expr_path) => expr_path
                    .path
                    .segments
                    .iter()
                    .map(|s| s.ident.to_string())
                    .collect::<Vec<_>>()
                    .join("::"),
                _ => {
                    return Err(syn::Error::new_spanned(
                        expr_call,
                        "Expected function name path",
                    ));
                }
            };

            if func_name == "cmd" {
                let first_arg = expr_call.args.first().ok_or_else(|| {
                    syn::Error::new_spanned(
                        expr_call,
                        "cmd requires at least one argument (the command name)",
                    )
                })?;

                let cmd_name_tokens = convert_expr(first_arg)?;

                let mut args_tokens = Vec::new();
                for arg in expr_call.args.iter().skip(1) {
                    let tokens = convert_expr(arg)?;
                    args_tokens.push(quote! {
                        match #tokens {
                            crate::utils::exec::script::dsl::ShellIR::Expr(crate::utils::exec::script::dsl::Expr::Literal(l)) => {
                                crate::utils::exec::script::dsl::ArgToken::Literal(l)
                            }
                            crate::utils::exec::script::dsl::ShellIR::Expr(crate::utils::exec::script::dsl::Expr::Variable(v)) => {
                                crate::utils::exec::script::dsl::ArgToken::Variable(v)
                            }
                            crate::utils::exec::script::dsl::ShellIR::Expr(crate::utils::exec::script::dsl::Expr::EnvVar(e)) => {
                                crate::utils::exec::script::dsl::ArgToken::EnvVar(e)
                            }
                            crate::utils::exec::script::dsl::ShellIR::Expr(crate::utils::exec::script::dsl::Expr::Glob(g)) => {
                                crate::utils::exec::script::dsl::ArgToken::Glob(g)
                            }
                            crate::utils::exec::script::dsl::ShellIR::Expr(word @ crate::utils::exec::script::dsl::Expr::Word(_)) => {
                                crate::utils::exec::script::dsl::ArgToken::Rendered(word.to_bash())
                            }
                            _ => panic!("Unsupported argument type in cmd call"),
                        }
                    });
                }

                return Ok(quote! {
                    (crate::utils::exec::script::dsl::ShellIR::Command(
                        crate::utils::exec::script::dsl::Command {
                            name: match #cmd_name_tokens {
                                crate::utils::exec::script::dsl::ShellIR::Expr(crate::utils::exec::script::dsl::Expr::Literal(l)) => l,
                                _ => panic!("Command name must be a string literal"),
                            },
                            args: vec![ #( #args_tokens ),* ],
                        }
                    ))
                });
            }

            if matches!(func_name.as_str(), "grep" | "jq" | "awk" | "sed") {
                return crate::convert::macros::convert_text_tool(
                    &func_name,
                    expr_call.args.iter().cloned(),
                    expr_call.span(),
                );
            }

            if func_name == "temp_file" {
                return Ok(quote! {
                    (crate::utils::exec::script::dsl::ShellIR::Capture {
                        cmd: Box::new(crate::utils::exec::script::dsl::ShellIR::Command(
                            crate::utils::exec::script::dsl::Command {
                                name: "mktemp".to_string(),
                                args: vec![],
                            }
                        )),
                        source: crate::utils::exec::script::dsl::CaptureSource::Stdout,
                    })
                });
            }

            if func_name == "os_id" || func_name == "os_family" || func_name == "os_arch" {
                let cmd_name = match func_name.as_str() {
                    "os_id" => "oid",
                    "os_family" => "family",
                    "os_arch" => "arch",
                    _ => unreachable!(),
                };
                let cmd_binary = match func_name.as_str() {
                    "os_id" => "sh",
                    "os_family" => "uname",
                    "os_arch" => "uname",
                    _ => unreachable!(),
                };
                let cmd_args_tokens = match func_name.as_str() {
                    "os_id" => quote! {
                        vec![
                            crate::utils::exec::script::dsl::ArgToken::Literal("-c".to_string()),
                            crate::utils::exec::script::dsl::ArgToken::Literal(". /etc/os-release && echo \"$ID\"".to_string()),
                        ]
                    },
                    "os_family" => quote! {
                        vec![
                            crate::utils::exec::script::dsl::ArgToken::Literal("-s".to_string()),
                        ]
                    },
                    "os_arch" => quote! {
                        vec![
                            crate::utils::exec::script::dsl::ArgToken::Literal("-m".to_string()),
                        ]
                    },
                    _ => unreachable!(),
                };
                return Ok(quote! {
                    (crate::utils::exec::script::dsl::ShellIR::Statement(
                        crate::utils::exec::script::dsl::Statement::VarAssign {
                            name: #cmd_name.to_string(),
                            val: Box::new(crate::utils::exec::script::dsl::ShellIR::Capture {
                                cmd: Box::new(crate::utils::exec::script::dsl::ShellIR::Command(
                                    crate::utils::exec::script::dsl::Command {
                                        name: #cmd_binary.to_string(),
                                        args: #cmd_args_tokens,
                                    }
                                )),
                                source: crate::utils::exec::script::dsl::CaptureSource::Stdout,
                            }),
                            default: None,
                        }
                    ))
                });
            }

            let mut args_tokens = Vec::new();
            for arg in &expr_call.args {
                let tokens = convert_expr(arg)?;
                args_tokens.push(quote! {
                    match #tokens {
                        crate::utils::exec::script::dsl::ShellIR::Expr(crate::utils::exec::script::dsl::Expr::Literal(l)) => {
                            crate::utils::exec::script::dsl::ArgToken::Literal(l)
                        }
                        crate::utils::exec::script::dsl::ShellIR::Expr(crate::utils::exec::script::dsl::Expr::Variable(v)) => {
                            crate::utils::exec::script::dsl::ArgToken::Variable(v)
                        }
                        crate::utils::exec::script::dsl::ShellIR::Expr(crate::utils::exec::script::dsl::Expr::EnvVar(e)) => {
                            crate::utils::exec::script::dsl::ArgToken::EnvVar(e)
                        }
                        crate::utils::exec::script::dsl::ShellIR::Expr(crate::utils::exec::script::dsl::Expr::Glob(g)) => {
                            crate::utils::exec::script::dsl::ArgToken::Glob(g)
                        }
                        _ => panic!("Unsupported argument type in DSL call"),
                    }
                });
            }

            return Ok(quote! {
                (crate::utils::exec::script::dsl::ShellIR::Command(
                    crate::utils::exec::script::dsl::Command {
                        name: #func_name.to_string(),
                        args: vec![ #( #args_tokens ),* ],
                    }
                ))
            });
        }
        Expr::Path(expr_path) => {
            let ident = expr_path.path.get_ident().ok_or_else(|| {
                syn::Error::new_spanned(
                    expr_path,
                    "Expected single identifier for variable reference",
                )
            })?;
            let name_str = ident.to_string();
            let is_sh_var = crate::SH_VARS.with(|cell| cell.borrow().contains(&name_str));
            if is_sh_var {
                Ok(quote! {
                    (crate::utils::exec::script::dsl::ShellIR::Expr(
                        crate::utils::exec::script::dsl::Expr::Variable(#name_str.to_string())
                    ))
                })
            } else {
                Ok(quote! {
                    (crate::utils::exec::script::dsl::ShellIR::Expr(
                        crate::utils::exec::script::dsl::Expr::Literal((#ident).build_str())
                    ))
                })
            }
        }
        Expr::MethodCall(method_call) => convert_method_call(method_call),
        Expr::Macro(expr_macro) => {
            let macro_name = expr_macro
                .mac
                .path
                .get_ident()
                .map(|i| i.to_string())
                .unwrap_or_default();
            if macro_name == "rust" {
                let parser = <syn::Expr as syn::parse::Parse>::parse;
                let inner_expr = expr_macro.mac.parse_body_with(parser)?;
                return Ok(quote! {
                    (crate::utils::exec::script::dsl::ShellIR::Expr(
                        crate::utils::exec::script::dsl::Expr::Literal((#inner_expr).build_str())
                    ))
                });
            }
            convert_macro(&expr_macro.mac)
        }
        Expr::Array(expr_array) => {
            let mut elem_tokens = Vec::new();
            for elem in &expr_array.elems {
                let elem_tok = convert_expr(elem)?;
                elem_tokens.push(quote! {
                    match #elem_tok {
                        crate::utils::exec::script::dsl::ShellIR::Expr(e) => e,
                        _ => panic!("Expected Expr in array"),
                    }
                });
            }
            Ok(quote! {
                (crate::utils::exec::script::dsl::ShellIR::Expr(
                    crate::utils::exec::script::dsl::Expr::Array(vec![ #( #elem_tokens ),* ])
                ))
            })
        }
        Expr::If(expr_if) => {
            let cond_tokens = convert_expr(&expr_if.cond)?;
            let mut then_stmts = Vec::new();
            for stmt in &expr_if.then_branch.stmts {
                then_stmts.push(convert_stmt(stmt)?);
            }
            let else_tokens = if let Some((_, else_expr)) = &expr_if.else_branch {
                let mut else_stmts = Vec::new();
                match &**else_expr {
                    Expr::Block(block) => {
                        for stmt in &block.block.stmts {
                            else_stmts.push(convert_stmt(stmt)?);
                        }
                    }
                    _ => else_stmts.push(convert_expr(else_expr)?),
                }
                quote! { Some(vec![ #( #else_stmts ),* ]) }
            } else {
                quote! { None }
            };
            Ok(quote! {
                (crate::utils::exec::script::dsl::ShellIR::If {
                    cond: Box::new(#cond_tokens),
                    then_branch: vec![ #( #then_stmts ),* ],
                    else_branch: #else_tokens,
                })
            })
        }
        Expr::ForLoop(expr_for) => {
            let var_name = match &*expr_for.pat {
                Pat::Ident(pat_ident) => pat_ident.ident.to_string(),
                _ => {
                    return Err(syn::Error::new_spanned(
                        &expr_for.pat,
                        "Expected identifier for loop variable",
                    ));
                }
            };
            let iter_tokens = convert_expr(&expr_for.expr)?;
            let mut body_stmts = Vec::new();
            for stmt in &expr_for.body.stmts {
                body_stmts.push(convert_stmt(stmt)?);
            }
            Ok(quote! {
                (crate::utils::exec::script::dsl::ShellIR::Loop {
                    var: #var_name.to_string(),
                    iterator: Box::new(#iter_tokens),
                    body: vec![ #( #body_stmts ),* ],
                })
            })
        }
        Expr::While(expr_while) => {
            let cond_tokens = convert_expr(&expr_while.cond)?;
            let mut body_stmts = Vec::new();
            for stmt in &expr_while.body.stmts {
                body_stmts.push(convert_stmt(stmt)?);
            }
            Ok(quote! {
                (crate::utils::exec::script::dsl::ShellIR::While {
                    cond: Box::new(#cond_tokens),
                    body: vec![ #( #body_stmts ),* ],
                })
            })
        }
        Expr::Break(_) => Ok(quote! {
            crate::utils::exec::script::dsl::ShellIR::Command(
                crate::utils::exec::script::dsl::Command {
                    name: "break".to_string(),
                    args: vec![],
                }
            )
        }),
        Expr::Continue(_) => Ok(quote! {
            crate::utils::exec::script::dsl::ShellIR::Command(
                crate::utils::exec::script::dsl::Command {
                    name: "continue".to_string(),
                    args: vec![],
                }
            )
        }),
        Expr::Lit(expr_lit) => {
            let val = match &expr_lit.lit {
                syn::Lit::Str(lit_str) => lit_str.value(),
                syn::Lit::Int(lit_int) => lit_int.to_string(),
                syn::Lit::Float(lit_float) => lit_float.to_string(),
                syn::Lit::Bool(lit_bool) => lit_bool.value.to_string(),
                _ => {
                    return Err(syn::Error::new_spanned(
                        expr_lit,
                        "Unsupported literal type",
                    ));
                }
            };
            Ok(quote! {
                (crate::utils::exec::script::dsl::ShellIR::Expr(
                    crate::utils::exec::script::dsl::Expr::Literal(#val.to_string())
                ))
            })
        }
        Expr::Binary(expr_binary) => {
            let left_tokens = convert_expr(&expr_binary.left)?;
            let right_tokens = convert_expr(&expr_binary.right)?;
            let op_str = match expr_binary.op {
                syn::BinOp::And(_) => "&&",
                syn::BinOp::Or(_) => "||",
                _ => {
                    return Err(syn::Error::new_spanned(
                        expr_binary,
                        "Unsupported binary operator in sh! block",
                    ));
                }
            };
            Ok(quote! {
                (crate::utils::exec::script::dsl::ShellIR::Raw(format!(
                    "{} {} {}",
                    (#left_tokens).to_bash(),
                    #op_str,
                    (#right_tokens).to_bash()
                )))
            })
        }
        Expr::Unary(expr_unary) => {
            let inner_tokens = convert_expr(&expr_unary.expr)?;
            match expr_unary.op {
                syn::UnOp::Not(_) => Ok(quote! {
                    (crate::utils::exec::script::dsl::ShellIR::Raw(format!(
                        "! {}",
                        (#inner_tokens).to_bash()
                    )))
                }),
                _ => Err(syn::Error::new_spanned(
                    expr_unary,
                    "Unsupported unary operator in sh! block",
                )),
            }
        }
        Expr::Group(expr_group) => {
            let inner_tokens = convert_expr(&expr_group.expr)?;
            Ok(quote! {
                (crate::utils::exec::script::dsl::ShellIR::Raw(format!(
                    "( {} )",
                    (#inner_tokens).to_bash()
                )))
            })
        }
        _ => Err(syn::Error::new_spanned(
            expr,
            "Unsupported expression in sh! block",
        )),
    }
}
