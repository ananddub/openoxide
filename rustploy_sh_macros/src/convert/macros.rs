use crate::convert::{convert_expr, convert_sh_stmt, convert_stmt};
use crate::parser::ShInput;
use quote::quote;
use syn::parse::Parse;

pub fn convert_macro(mac: &syn::Macro) -> Result<proc_macro2::TokenStream, syn::Error> {
    let macro_name = mac
        .path
        .get_ident()
        .map(|i| i.to_string())
        .ok_or_else(|| syn::Error::new_spanned(mac, "Expected macro name"))?;

    if macro_name == "rust" {
        let parser = <syn::Expr as syn::parse::Parse>::parse;
        let inner_expr = mac.parse_body_with(parser)?;
        return Ok(quote! {
            (crate::utils::exec::script::dsl::ShellIR::Expr(
                crate::utils::exec::script::dsl::Expr::Literal((#inner_expr).build_str())
            ))
        });
    }

    if macro_name == "json" {
        let parsed = mac.parse_body_with(JsonMacroInput::parse)?;
        let mut parts = Vec::new();
        for pair in parsed.pairs {
            let val_str = match &pair.value {
                syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
                    syn::Lit::Str(s) => format!("\\\"{}\\\"", s.value()),
                    syn::Lit::Bool(b) => b.value.to_string(),
                    syn::Lit::Int(i) => i.to_string(),
                    _ => {
                        return Err(syn::Error::new_spanned(
                            &pair.value,
                            "Unsupported literal inside json!",
                        ));
                    }
                },
                syn::Expr::Path(p) => {
                    let var_name = p
                        .path
                        .get_ident()
                        .map(|i| i.to_string())
                        .unwrap_or_default();
                    if var_name == "isEnabled"
                        || var_name == "keyAuth"
                        || var_name == "enabled"
                        || var_name == "key_auth"
                    {
                        format!("${}", var_name)
                    } else {
                        format!("\\\"${}\\\"", var_name)
                    }
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        &pair.value,
                        "Unsupported value inside json!",
                    ));
                }
            };
            parts.push(format!("\\\"{}\\\": {}", pair.key, val_str));
        }
        let json_str = format!("{{{}}}", parts.join(", "));
        let raw_cmd = format!("echo \"{}\"", json_str);
        return Ok(quote! {
            (crate::utils::exec::script::dsl::ShellIR::Raw(#raw_cmd.to_string()))
        });
    }

    if macro_name == "any" {
        let parser = syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated;
        let exprs = mac.parse_body_with(parser)?;

        let mut cmd_tokens = Vec::new();
        for expr in exprs {
            cmd_tokens.push(convert_expr(&expr)?);
        }

        let mut cmd_bash_strings = Vec::new();
        for tokens in cmd_tokens {
            cmd_bash_strings.push(quote! { (#tokens).to_bash() });
        }
        return Ok(quote! {
            (crate::utils::exec::script::dsl::ShellIR::Command(
                crate::utils::exec::script::dsl::Command {
                    name: vec![ #( #cmd_bash_strings ),* ].join(" || "),
                    args: vec![],
                }
            ))
        });
    }

    if macro_name == "capture" {
        let parser = ShInput::parse;
        let inner_input = mac.parse_body_with(parser)?;
        let mut inner_stmts = Vec::new();
        for stmt in inner_input.stmts {
            inner_stmts.push(convert_sh_stmt(&stmt)?);
        }
        return Ok(quote! {
            (crate::utils::exec::script::dsl::ShellIR::CaptureBlock(
                vec![ #( #inner_stmts ),* ]
            ))
        });
    }

    if macro_name == "capture_stdout" {
        let parser = ShInput::parse;
        let inner_input = mac.parse_body_with(parser)?;
        let mut inner_stmts = Vec::new();
        for stmt in inner_input.stmts {
            inner_stmts.push(convert_sh_stmt(&stmt)?);
        }
        let cmd_tokens = if inner_stmts.len() == 1 {
            let stmt = &inner_stmts[0];
            quote! { Box::new(#stmt) }
        } else {
            quote! {
                Box::new((crate::utils::exec::script::dsl::ShellIR::If {
                    cond: Box::new((crate::utils::exec::script::dsl::ShellIR::Command(
                        crate::utils::exec::script::dsl::Command { name: "true".to_string(), args: vec![] }
                    ))),
                    then_branch: vec![ #( #inner_stmts ),* ],
                    else_branch: None,
                }))
            }
        };
        return Ok(quote! {
            (crate::utils::exec::script::dsl::ShellIR::Capture {
                cmd: #cmd_tokens,
                source: crate::utils::exec::script::dsl::CaptureSource::Stdout,
            })
        });
    }

    if macro_name == "capture_status" {
        let parser = ShInput::parse;
        let inner_input = mac.parse_body_with(parser)?;
        let mut inner_stmts = Vec::new();
        for stmt in inner_input.stmts {
            inner_stmts.push(convert_sh_stmt(&stmt)?);
        }
        let cmd_tokens = if inner_stmts.len() == 1 {
            let stmt = &inner_stmts[0];
            quote! { Box::new(#stmt) }
        } else {
            quote! {
                Box::new((crate::utils::exec::script::dsl::ShellIR::If {
                    cond: Box::new((crate::utils::exec::script::dsl::ShellIR::Command(
                        crate::utils::exec::script::dsl::Command { name: "true".to_string(), args: vec![] }
                    ))),
                    then_branch: vec![ #( #inner_stmts ),* ],
                    else_branch: None,
                }))
            }
        };
        return Ok(quote! {
            (crate::utils::exec::script::dsl::ShellIR::Capture {
                cmd: #cmd_tokens,
                source: crate::utils::exec::script::dsl::CaptureSource::Status,
            })
        });
    }

    if macro_name == "sudo" {
        let parser = syn::Expr::parse;
        let expr = mac.parse_body_with(parser)?;
        let expr_tokens = convert_expr(&expr)?;
        return Ok(quote! {
            (#expr_tokens).sudo()
        });
    }

    if macro_name == "glob" {
        let parser = syn::Expr::parse;
        let expr = mac.parse_body_with(parser)?;
        let expr_tokens = convert_expr(&expr)?;
        return Ok(quote! {
            (crate::utils::exec::script::dsl::ShellIR::Expr(
                crate::utils::exec::script::dsl::Expr::Glob(
                    match #expr_tokens {
                        crate::utils::exec::script::dsl::ShellIR::Expr(crate::utils::exec::script::dsl::Expr::Literal(l)) => l,
                        _ => panic!("glob! macro requires a string literal argument"),
                    }
                )
            ))
        });
    }

    if macro_name == "shell_env" || macro_name == "env_var" {
        let parser = syn::Expr::parse;
        let expr = mac.parse_body_with(parser)?;
        let expr_tokens = convert_expr(&expr)?;
        return Ok(quote! {
            (crate::utils::exec::script::dsl::ShellIR::Expr(
                crate::utils::exec::script::dsl::Expr::EnvVar(
                    match #expr_tokens {
                        crate::utils::exec::script::dsl::ShellIR::Expr(crate::utils::exec::script::dsl::Expr::Literal(l)) => l,
                        _ => panic!("shell_env!/env_var! macro requires a string literal argument"),
                    }
                )
            ))
        });
    }

    if macro_name == "defer" {
        let parser = ShInput::parse;
        let inner_input = mac.parse_body_with(parser)?;
        let mut inner_stmts = Vec::new();
        for stmt in inner_input.stmts {
            inner_stmts.push(convert_sh_stmt(&stmt)?);
        }
        return Ok(quote! {
            (crate::utils::exec::script::dsl::ShellIR::Defer {
                body: vec![ #( #inner_stmts ),* ],
            })
        });
    }

    if macro_name == "parallel" {
        let parser = ShInput::parse;
        let inner_input = mac.parse_body_with(parser)?;
        let mut inner_stmts = Vec::new();
        for stmt in inner_input.stmts {
            inner_stmts.push(convert_sh_stmt(&stmt)?);
        }
        return Ok(quote! {
            (crate::utils::exec::script::dsl::ShellIR::Parallel {
                body: vec![ #( #inner_stmts ),* ],
            })
        });
    }

    if macro_name == "retry" {
        struct RetryInput {
            count: syn::Expr,
            _comma: syn::Token![,],
            body: syn::ExprBlock,
        }
        impl Parse for RetryInput {
            fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                Ok(RetryInput {
                    count: input.parse()?,
                    _comma: input.parse()?,
                    body: input.parse()?,
                })
            }
        }
        let parsed = mac.parse_body_with(RetryInput::parse)?;
        let count_tokens = convert_expr(&parsed.count)?;
        let mut body_stmts = Vec::new();
        for stmt in &parsed.body.block.stmts {
            body_stmts.push(convert_stmt(stmt)?);
        }

        return Ok(quote! {
            (crate::utils::exec::script::dsl::ShellIR::Retry {
                count: Box::new(#count_tokens),
                body: vec![ #( #body_stmts ),* ],
            })
        });
    }

    if macro_name == "info" || macro_name == "warn" || macro_name == "error" {
        let parser = syn::Expr::parse;
        let expr = mac.parse_body_with(parser)?;
        let expr_tokens = convert_expr(&expr)?;
        let prefix = format!("[{}] ", macro_name.to_uppercase());
        return Ok(quote! {
            (crate::utils::exec::script::dsl::ShellIR::Statement(
                crate::utils::exec::script::dsl::Statement::Echo(
                    Box::new(crate::utils::exec::script::dsl::ShellIR::Expr(
                        crate::utils::exec::script::dsl::Expr::Literal(
                            match #expr_tokens {
                                crate::utils::exec::script::dsl::ShellIR::Expr(crate::utils::exec::script::dsl::Expr::Literal(l)) => {
                                    format!("{}{}", #prefix, l)
                                }
                                _ => panic!("Logging macros require a string literal argument"),
                            }
                        )
                    ))
                )
            ))
        });
    }

    if macro_name == "jq" {
        struct JqInput {
            target: syn::Expr,
            _comma: syn::Token![,],
            query: syn::Expr,
        }
        impl syn::parse::Parse for JqInput {
            fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                Ok(JqInput {
                    target: input.parse()?,
                    _comma: input.parse()?,
                    query: input.parse()?,
                })
            }
        }
        let parsed = mac.parse_body_with(JqInput::parse)?;
        let target_tokens = convert_expr(&parsed.target)?;
        let query_tokens = convert_expr(&parsed.query)?;

        return Ok(quote! {
            (crate::utils::exec::script::dsl::ShellIR::Raw(
                format!("$(echo \"${}\" | jq -r {})",
                    match #target_tokens {
                        crate::utils::exec::script::dsl::ShellIR::Expr(crate::utils::exec::script::dsl::Expr::Variable(ref v)) => v,
                        _ => panic!("jq! target must be a variable"),
                    },
                    match #query_tokens {
                        crate::utils::exec::script::dsl::ShellIR::Expr(crate::utils::exec::script::dsl::Expr::Literal(ref l)) => {
                            crate::utils::exec::script::shell_single_quote(l)
                        }
                        _ => panic!("jq! query must be a string literal"),
                    }
                )
            ))
        });
    }

    if macro_name == "jq_file" {
        struct JqFileInput {
            file: syn::Expr,
            _comma: syn::Token![,],
            query: syn::Expr,
        }
        impl syn::parse::Parse for JqFileInput {
            fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                Ok(JqFileInput {
                    file: input.parse()?,
                    _comma: input.parse()?,
                    query: input.parse()?,
                })
            }
        }
        let parsed = mac.parse_body_with(JqFileInput::parse)?;
        let file_tokens = convert_expr(&parsed.file)?;
        let query_tokens = convert_expr(&parsed.query)?;

        return Ok(quote! {
            (crate::utils::exec::script::dsl::ShellIR::Raw(
                format!("$(jq -r {} {})",
                    match #query_tokens {
                        crate::utils::exec::script::dsl::ShellIR::Expr(crate::utils::exec::script::dsl::Expr::Literal(ref l)) => {
                            crate::utils::exec::script::shell_single_quote(l)
                        }
                        _ => panic!("jq_file! query must be a string literal"),
                    },
                    match #file_tokens {
                        crate::utils::exec::script::dsl::ShellIR::Expr(crate::utils::exec::script::dsl::Expr::Literal(ref l)) => {
                            crate::utils::exec::script::shell_single_quote(l)
                        }
                        _ => panic!("jq_file! file path must be a string literal"),
                    }
                )
            ))
        });
    }

    let parser = syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated;
    let exprs = mac.parse_body_with(parser)?;

    let mut args_tokens = Vec::new();
    for expr in exprs {
        let tokens = convert_expr(&expr)?;
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
                _ => panic!("Unsupported argument type in command macro"),
            }
        });
    }

    let parser = syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated;
    let exprs_to_validate = mac.parse_body_with(parser)?;
    validate_command_args(&macro_name, &exprs_to_validate)?;

    Ok(quote! {
        (crate::utils::exec::script::dsl::ShellIR::Command(
            crate::utils::exec::script::dsl::Command {
                name: #macro_name.to_string(),
                args: vec![ #( #args_tokens ),* ],
            }
        ))
    })
}

fn validate_command_args(
    name: &str,
    exprs: &syn::punctuated::Punctuated<syn::Expr, syn::Token![,]>,
) -> Result<(), syn::Error> {
    match name {
        "grep" => validate_grep(exprs),
        "sed" => validate_sed(exprs),
        "awk" => validate_awk(exprs),
        "find" => validate_find(exprs),
        "xargs" => validate_xargs(exprs),
        "tar" => validate_tar(exprs),
        "curl" => validate_curl(exprs),
        _ => Ok(()),
    }
}

fn validate_grep(
    exprs: &syn::punctuated::Punctuated<syn::Expr, syn::Token![,]>,
) -> Result<(), syn::Error> {
    let allowed_long = [
        "--ignore-case",
        "--invert-match",
        "--line-number",
        "--quiet",
        "--silent",
        "--only-matching",
        "--count",
        "--files-with-matches",
        "--recursive",
        "--word-regexp",
        "--line-regexp",
        "--no-filename",
        "--with-filename",
        "--text",
        "--binary-files",
        "--color",
        "--colour",
        "--include",
        "--exclude",
        "--exclude-dir",
        "--help",
        "--version",
    ];
    let allowed_short = "EFGPefinvnqoclrrwxhHae";

    let mut pattern_validated = false;
    for expr in exprs {
        if let syn::Expr::Lit(expr_lit) = expr {
            if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                let val = lit_str.value();
                if val.starts_with('-') {
                    if val.starts_with("--") {
                        let flag_name = val.split('=').next().unwrap();
                        if !allowed_long.contains(&flag_name) {
                            return Err(syn::Error::new_spanned(
                                lit_str,
                                format!("Invalid grep flag: {}", val),
                            ));
                        }
                    } else {
                        for c in val.chars().skip(1) {
                            if c != '=' && !allowed_short.contains(c) {
                                return Err(syn::Error::new_spanned(
                                    lit_str,
                                    format!("Invalid grep short flag '{}' in {}", c, val),
                                ));
                            }
                        }
                    }
                } else if !pattern_validated {
                    if let Err(e) = regex::Regex::new(&val) {
                        return Err(syn::Error::new_spanned(
                            lit_str,
                            format!("Invalid regular expression pattern in grep: {}", e),
                        ));
                    }
                    pattern_validated = true;
                }
            }
        }
    }
    Ok(())
}

fn validate_sed(
    exprs: &syn::punctuated::Punctuated<syn::Expr, syn::Token![,]>,
) -> Result<(), syn::Error> {
    let allowed_long = [
        "--quiet",
        "--silent",
        "--expression",
        "--file",
        "--in-place",
        "--regexp-extended",
        "--posix",
        "--sandbox",
        "--help",
        "--version",
    ];
    let allowed_short = "nEefir";

    let mut script_validated = false;
    for expr in exprs {
        if let syn::Expr::Lit(expr_lit) = expr {
            if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                let val = lit_str.value();
                if val.starts_with('-') {
                    if val.starts_with("--") {
                        let flag_name = val.split('=').next().unwrap();
                        if !allowed_long.contains(&flag_name) {
                            return Err(syn::Error::new_spanned(
                                lit_str,
                                format!("Invalid sed flag: {}", val),
                            ));
                        }
                    } else {
                        for c in val.chars().skip(1) {
                            if c != '=' && !allowed_short.contains(c) {
                                return Err(syn::Error::new_spanned(
                                    lit_str,
                                    format!("Invalid sed short flag '{}' in {}", c, val),
                                ));
                            }
                        }
                    }
                } else if !script_validated {
                    if val.starts_with('s') || val.starts_with('y') {
                        if val.len() < 2 {
                            return Err(syn::Error::new_spanned(
                                lit_str,
                                "Invalid sed script: too short",
                            ));
                        }
                        let delim = val.chars().nth(1).unwrap();
                        let parts = split_sed_script(&val, delim);
                        if parts.len() < 3 {
                            return Err(syn::Error::new_spanned(
                                lit_str,
                                format!(
                                    "Invalid sed script structure, expected at least 3 parts separated by '{}'",
                                    delim
                                ),
                            ));
                        }
                        if val.starts_with('s') {
                            if let Err(e) = regex::Regex::new(&parts[0]) {
                                return Err(syn::Error::new_spanned(
                                    lit_str,
                                    format!(
                                        "Invalid regular expression pattern in sed script: {}",
                                        e
                                    ),
                                ));
                            }
                        }
                    }
                    script_validated = true;
                }
            }
        }
    }
    Ok(())
}

fn split_sed_script(s: &str, delim: char) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut escaped = false;
    let mut chars = s.chars().skip(2);
    while let Some(c) = chars.next() {
        if escaped {
            current.push(c);
            escaped = false;
        } else if c == '\\' {
            current.push(c);
            escaped = true;
        } else if c == delim {
            parts.push(std::mem::take(&mut current));
        } else {
            current.push(c);
        }
    }
    parts.push(current);
    parts
}

fn validate_awk(
    exprs: &syn::punctuated::Punctuated<syn::Expr, syn::Token![,]>,
) -> Result<(), syn::Error> {
    let allowed_long = [
        "--field-separator",
        "--assign",
        "--file",
        "--help",
        "--version",
        "--posix",
        "--traditional",
    ];
    let allowed_short = "FvW";

    let mut script_validated = false;
    for expr in exprs {
        if let syn::Expr::Lit(expr_lit) = expr {
            if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                let val = lit_str.value();
                if val.starts_with('-') {
                    if val.starts_with("--") {
                        let flag_name = val.split('=').next().unwrap();
                        if !allowed_long.contains(&flag_name) {
                            return Err(syn::Error::new_spanned(
                                lit_str,
                                format!("Invalid awk flag: {}", val),
                            ));
                        }
                    } else {
                        for c in val.chars().skip(1) {
                            if c != '=' && !allowed_short.contains(c) {
                                return Err(syn::Error::new_spanned(
                                    lit_str,
                                    format!("Invalid awk short flag '{}' in {}", c, val),
                                ));
                            }
                        }
                    }
                } else if !script_validated {
                    let mut brace_count = 0;
                    for c in val.chars() {
                        if c == '{' {
                            brace_count += 1;
                        } else if c == '}' {
                            brace_count -= 1;
                            if brace_count < 0 {
                                return Err(syn::Error::new_spanned(
                                    lit_str,
                                    "Unmatched closing brace '}' in awk script",
                                ));
                            }
                        }
                    }
                    if brace_count != 0 {
                        return Err(syn::Error::new_spanned(
                            lit_str,
                            "Unclosed opening brace '{' in awk script",
                        ));
                    }
                    script_validated = true;
                }
            }
        }
    }
    Ok(())
}

fn validate_find(
    exprs: &syn::punctuated::Punctuated<syn::Expr, syn::Token![,]>,
) -> Result<(), syn::Error> {
    let allowed = [
        "-name",
        "-iname",
        "-path",
        "-ipath",
        "-regex",
        "-iregex",
        "-type",
        "-size",
        "-mtime",
        "-atime",
        "-ctime",
        "-amin",
        "-cmin",
        "-mmin",
        "-perm",
        "-user",
        "-group",
        "-nouser",
        "-nogroup",
        "-links",
        "-inum",
        "-maxdepth",
        "-mindepth",
        "-depth",
        "-mount",
        "-xdev",
        "-print",
        "-print0",
        "-printf",
        "-prune",
        "-quit",
        "-exec",
        "-execdir",
        "-ok",
        "-okdir",
        "-delete",
        "-ls",
        "-fls",
        "-and",
        "-or",
        "-not",
        "-a",
        "-o",
    ];

    for expr in exprs {
        if let syn::Expr::Lit(expr_lit) = expr {
            if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                let val = lit_str.value();
                if val.starts_with('-') {
                    let suffix = &val[1..];
                    if !suffix.chars().all(|c| c.is_ascii_digit()) {
                        if !allowed.contains(&val.as_str()) {
                            return Err(syn::Error::new_spanned(
                                lit_str,
                                format!("Invalid find option/test: {}", val),
                            ));
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn validate_xargs(
    exprs: &syn::punctuated::Punctuated<syn::Expr, syn::Token![,]>,
) -> Result<(), syn::Error> {
    let allowed = [
        "-0",
        "--null",
        "-d",
        "--delimiter",
        "-E",
        "-e",
        "--eof",
        "-I",
        "-i",
        "--replace",
        "-L",
        "-l",
        "--max-lines",
        "-n",
        "--max-args",
        "-P",
        "--max-procs",
        "-p",
        "--interactive",
        "-r",
        "--no-run-if-empty",
        "-s",
        "--max-chars",
        "-t",
        "--verbose",
        "-x",
        "--exit",
        "--help",
        "--version",
    ];

    for expr in exprs {
        if let syn::Expr::Lit(expr_lit) = expr {
            if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                let val = lit_str.value();
                if val.starts_with('-') {
                    if !allowed.contains(&val.as_str()) {
                        return Err(syn::Error::new_spanned(
                            lit_str,
                            format!("Invalid xargs option: {}", val),
                        ));
                    }
                }
            }
        }
    }
    Ok(())
}

fn validate_tar(
    exprs: &syn::punctuated::Punctuated<syn::Expr, syn::Token![,]>,
) -> Result<(), syn::Error> {
    let allowed_long = [
        "--create",
        "--extract",
        "--get",
        "--list",
        "--file",
        "--directory",
        "--gzip",
        "--bzip2",
        "--xz",
        "--lzma",
        "--verbose",
        "--exclude",
        "--help",
        "--version",
        "--strip-components",
    ];
    let allowed_short = "cxtrudAazjJZaVfCpkhmkPNOOW";

    let mut is_first = true;
    for expr in exprs {
        if let syn::Expr::Lit(expr_lit) = expr {
            if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                let val = lit_str.value();
                if val.starts_with("--") {
                    let flag_name = val.split('=').next().unwrap();
                    if !allowed_long.contains(&flag_name) {
                        return Err(syn::Error::new_spanned(
                            lit_str,
                            format!("Invalid tar long option: {}", val),
                        ));
                    }
                } else if val.starts_with('-') {
                    for c in val.chars().skip(1) {
                        if !allowed_short.contains(c) {
                            return Err(syn::Error::new_spanned(
                                lit_str,
                                format!("Invalid tar short option '{}' in {}", c, val),
                            ));
                        }
                    }
                } else if is_first {
                    for c in val.chars() {
                        if !allowed_short.contains(c) {
                            return Err(syn::Error::new_spanned(
                                lit_str,
                                format!("Invalid tar combined short option '{}' in {}", c, val),
                            ));
                        }
                    }
                }
            }
        }
        is_first = false;
    }
    Ok(())
}

fn validate_curl(
    exprs: &syn::punctuated::Punctuated<syn::Expr, syn::Token![,]>,
) -> Result<(), syn::Error> {
    let allowed = [
        "-X",
        "--request",
        "-H",
        "--header",
        "-d",
        "--data",
        "--data-raw",
        "--data-binary",
        "-o",
        "--output",
        "-O",
        "--remote-name",
        "-s",
        "--silent",
        "-S",
        "--show-error",
        "-L",
        "--location",
        "-u",
        "--user",
        "-F",
        "--form",
        "-i",
        "--include",
        "-I",
        "--head",
        "-v",
        "--verbose",
        "--url",
        "-k",
        "--insecure",
        "-f",
        "--fail",
        "-m",
        "--max-time",
        "--connect-timeout",
        "-A",
        "--user-agent",
        "-e",
        "--referer",
        "-b",
        "--cookie",
        "-c",
        "--cookie-jar",
        "-G",
        "--get",
        "-J",
        "--remote-header-name",
        "-x",
        "--proxy",
        "--help",
        "--version",
    ];

    for expr in exprs {
        if let syn::Expr::Lit(expr_lit) = expr {
            if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                let val = lit_str.value();
                if val.starts_with('-') {
                    if !allowed.contains(&val.as_str()) {
                        return Err(syn::Error::new_spanned(
                            lit_str,
                            format!("Invalid curl option: {}", val),
                        ));
                    }
                }
            }
        }
    }
    Ok(())
}

struct JsonPair {
    key: String,
    value: syn::Expr,
}

impl syn::parse::Parse for JsonPair {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let key = if input.peek(syn::LitStr) {
            let lit: syn::LitStr = input.parse()?;
            lit.value()
        } else {
            let ident: syn::Ident = input.parse()?;
            ident.to_string()
        };
        input.parse::<syn::Token![:]>()?;
        let value: syn::Expr = input.parse()?;
        Ok(JsonPair { key, value })
    }
}

struct JsonMacroInput {
    pairs: syn::punctuated::Punctuated<JsonPair, syn::Token![,]>,
}

impl syn::parse::Parse for JsonMacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::token::Brace) {
            let content;
            syn::braced!(content in input);
            let pairs = syn::punctuated::Punctuated::<JsonPair, syn::Token![,]>::parse_terminated(
                &content,
            )?;
            Ok(JsonMacroInput { pairs })
        } else {
            let pairs =
                syn::punctuated::Punctuated::<JsonPair, syn::Token![,]>::parse_terminated(input)?;
            Ok(JsonMacroInput { pairs })
        }
    }
}
