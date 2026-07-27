use std::collections::{HashMap, HashSet};
use syn::spanned::Spanned;
use syn::{Expr, Pat, Stmt};

pub struct ScopeTracker {
    scopes: Vec<HashSet<String>>,
    /// Every variable name ever declared — survives scope pops.
    /// Used by substitute_sh_vars to distinguish sh!-local vars from outer Rust vars.
    all_declared: HashSet<String>,
    /// Declared DSL functions: function name -> expected argument count (arity)
    declared_functions: HashMap<String, usize>,
}

impl ScopeTracker {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashSet::new()],
            all_declared: HashSet::new(),
            declared_functions: HashMap::new(),
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashSet::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn declare(&mut self, name: String) {
        self.all_declared.insert(name.clone());
        if let Some(current) = self.scopes.last_mut() {
            current.insert(name);
        }
    }

    pub fn declare_fn(&mut self, name: String, arity: usize) {
        self.declared_functions.insert(name, arity);
    }

    /// Returns true if `name` is declared in any currently-active scope
    pub fn contains(&self, name: &str) -> bool {
        self.scopes.iter().any(|s| s.contains(name))
    }

    /// All variables ever declared in this sh! block (including those in
    /// already-popped scopes like loop variables).
    pub fn all_vars(&self) -> HashSet<String> {
        self.all_declared.clone()
    }

    pub fn check(&self, name: &str, span: proc_macro2::Span) -> Result<(), syn::Error> {
        for scope in self.scopes.iter().rev() {
            if scope.contains(name) {
                return Ok(());
            }
        }

        let mut best_match = None;
        let mut min_dist = usize::MAX;
        for scope in &self.scopes {
            for var in scope {
                let dist = levenshtein_distance(name, var);
                if dist < min_dist && dist <= 2 {
                    min_dist = dist;
                    best_match = Some(var.clone());
                }
            }
        }

        if let Some(suggestion) = best_match {
            Err(syn::Error::new(
                span,
                format!(
                    "Undefined variable '{}'. Did you mean '{}'?",
                    name, suggestion
                ),
            ))
        } else {
            // Not a local sh! variable or typo of one — auto-detected as an outer Rust variable!
            Ok(())
        }
    }

    pub fn check_fn(
        &self,
        name: &str,
        given_args: usize,
        span: proc_macro2::Span,
    ) -> Result<(), syn::Error> {
        if let Some(&expected_arity) = self.declared_functions.get(name) {
            if given_args != expected_arity {
                return Err(syn::Error::new(
                    span,
                    format!(
                        "Function '{}' expects {} argument(s), but {} were given",
                        name, expected_arity, given_args
                    ),
                ));
            }
            return Ok(());
        }

        // If not found, look for typos among declared functions
        let mut best_match = None;
        let mut min_dist = usize::MAX;
        for fn_name in self.declared_functions.keys() {
            let dist = levenshtein_distance(name, fn_name);
            if dist < min_dist && dist <= 2 {
                min_dist = dist;
                best_match = Some(fn_name.clone());
            }
        }

        if let Some(suggestion) = best_match {
            Err(syn::Error::new(
                span,
                format!(
                    "Undefined function '{}'. Did you mean '{}'?",
                    name, suggestion
                ),
            ))
        } else {
            Err(syn::Error::new(
                span,
                format!("Undefined function '{}'", name),
            ))
        }
    }
}

fn is_builtin_func(name: &str) -> bool {
    matches!(
        name,
        "cmd"
            | "echo"
            | "sleep"
            | "exit"
            | "temp_file"
            | "capture"
            | "capture_stdout"
            | "capture_status"
            | "glob"
            | "shell_env"
            | "systemctl"
            | "apt"
            | "apk"
            | "dnf"
            | "yum"
            | "pacman"
            | "zypper"
            | "xbps"
            | "emerge"
            | "nix"
            | "brew"
            | "package"
            | "os_id"
            | "os_family"
            | "os_arch"
            | "os_release"
            | "os_codename"
            | "os_version"
            | "grep"
            | "jq"
            | "awk"
            | "sed"
    )
}

/// Scope-check for the top-level `ShStmt` list (which may include untyped `fn` defs)
pub fn check_sh_stmts(
    stmts: &[crate::parser::ShStmt],
    tracker: &mut ScopeTracker,
) -> Result<(), syn::Error> {
    // Pass 1: Register all function declarations first
    for stmt in stmts {
        match stmt {
            crate::parser::ShStmt::ShFn { name, params, .. } => {
                tracker.declare_fn(name.clone(), params.len());
            }
            crate::parser::ShStmt::Syn(Stmt::Item(syn::Item::Fn(item_fn))) => {
                let name = item_fn.sig.ident.to_string();
                let arity = item_fn.sig.inputs.len();
                tracker.declare_fn(name, arity);
            }
            _ => {}
        }
    }

    // Pass 2: Check scopes and statements
    for stmt in stmts {
        match stmt {
            crate::parser::ShStmt::ShFn {
                name: _,
                params,
                body,
            } => {
                tracker.push_scope();
                for p in params {
                    tracker.declare(p.clone());
                }
                check_sh_stmts(body, tracker)?;
                tracker.pop_scope();
            }
            crate::parser::ShStmt::Syn(syn_stmt) => {
                check_stmts(std::slice::from_ref(syn_stmt), tracker)?;
            }
        }
    }
    Ok(())
}

fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let len_a = a_chars.len();
    let len_b = b_chars.len();
    let mut dp = vec![vec![0; len_b + 1]; len_a + 1];

    for i in 0..=len_a {
        dp[i][0] = i;
    }
    for j in 0..=len_b {
        dp[0][j] = j;
    }

    for i in 1..=len_a {
        for j in 1..=len_b {
            if a_chars[i - 1] == b_chars[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] =
                    1 + std::cmp::min(dp[i - 1][j - 1], std::cmp::min(dp[i - 1][j], dp[i][j - 1]));
            }
        }
    }
    dp[len_a][len_b]
}

pub fn check_stmts(stmts: &[Stmt], tracker: &mut ScopeTracker) -> Result<(), syn::Error> {
    for stmt in stmts {
        match stmt {
            Stmt::Local(local) => {
                if let Some(init) = &local.init {
                    check_expr(&init.expr, tracker)?;
                }
                let name = match &local.pat {
                    Pat::Ident(pat_ident) => pat_ident.ident.to_string(),
                    _ => {
                        return Err(syn::Error::new_spanned(
                            &local.pat,
                            "Expected variable name",
                        ));
                    }
                };
                tracker.declare(name);
            }
            Stmt::Expr(expr, _) => {
                if let Expr::Path(_) = expr {
                    // Skip checking standalone path statement
                } else {
                    check_expr(expr, tracker)?;
                }
            }
            Stmt::Macro(stmt_macro) => {
                let macro_name = stmt_macro
                    .mac
                    .path
                    .get_ident()
                    .map(|i| i.to_string())
                    .unwrap_or_default();
                if macro_name != "rust" {
                    check_macro(&stmt_macro.mac, tracker)?;
                }
            }
            Stmt::Item(syn::Item::Fn(item_fn)) => {
                let name = item_fn.sig.ident.to_string();
                let arity = item_fn.sig.inputs.len();
                tracker.declare_fn(name, arity);
                tracker.push_scope();
                for input in &item_fn.sig.inputs {
                    if let syn::FnArg::Typed(pat_type) = input {
                        if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                            tracker.declare(pat_ident.ident.to_string());
                        }
                    }
                }
                check_stmts(&item_fn.block.stmts, tracker)?;
                tracker.pop_scope();
            }
            Stmt::Item(_) => {}
        }
    }
    Ok(())
}

fn check_expr(expr: &Expr, tracker: &mut ScopeTracker) -> Result<(), syn::Error> {
    match expr {
        Expr::Path(expr_path) => {
            if let Some(ident) = expr_path.path.get_ident() {
                let name = ident.to_string();
                if name != "os" && name != "docker" {
                    tracker.check(&name, ident.span())?;
                }
            }
        }
        Expr::Call(expr_call) => {
            let func_name = match &*expr_call.func {
                Expr::Path(expr_path) => expr_path
                    .path
                    .segments
                    .iter()
                    .map(|s| s.ident.to_string())
                    .collect::<Vec<_>>()
                    .join("::"),
                _ => String::new(),
            };

            if !func_name.is_empty() && !is_builtin_func(&func_name) && !func_name.contains("::") {
                tracker.check_fn(&func_name, expr_call.args.len(), expr_call.span())?;
            }

            let invalid_text_tool = match func_name.as_str() {
                "jq" => !(1..=2).contains(&expr_call.args.len()),
                "grep" | "awk" | "sed" => expr_call.args.is_empty(),
                _ => false,
            };
            if invalid_text_tool {
                return Err(syn::Error::new_spanned(
                    expr_call,
                    format!("{func_name} expects at least one command argument"),
                ));
            }

            for arg in &expr_call.args {
                check_expr(arg, tracker)?;
            }
        }
        Expr::MethodCall(method_call) => {
            let method_name = method_call.method.to_string();
            if method_name == "stdout"
                || method_name == "stderr"
                || method_name == "sudo"
                || method_name == "ok"
            {
                check_expr(&method_call.receiver, tracker)?;
                for arg in &method_call.args {
                    check_expr(arg, tracker)?;
                }
            } else {
                let is_os_receiver = match &*method_call.receiver {
                    Expr::Path(p) => p
                        .path
                        .get_ident()
                        .map_or(false, |i| i == "os" || i == "docker"),
                    _ => false,
                };
                if !is_os_receiver {
                    check_expr(&method_call.receiver, tracker)?;
                }
                for arg in &method_call.args {
                    check_expr(arg, tracker)?;
                }
            }
        }
        Expr::Macro(expr_macro) => {
            let macro_name = expr_macro
                .mac
                .path
                .get_ident()
                .map(|i| i.to_string())
                .unwrap_or_default();
            if macro_name == "rust"
                || macro_name == "ir"
                || macro_name == "dynamic"
                || macro_name == "awk"
            {
                return Ok(());
            }
            check_macro(&expr_macro.mac, tracker)?;
        }
        Expr::Array(expr_array) => {
            for elem in &expr_array.elems {
                check_expr(elem, tracker)?;
            }
        }
        Expr::If(expr_if) => {
            check_expr(&expr_if.cond, tracker)?;
            tracker.push_scope();
            check_stmts(&expr_if.then_branch.stmts, tracker)?;
            tracker.pop_scope();
            if let Some((_, else_expr)) = &expr_if.else_branch {
                tracker.push_scope();
                check_expr(else_expr, tracker)?;
                tracker.pop_scope();
            }
        }
        Expr::Block(expr_block) => {
            tracker.push_scope();
            check_stmts(&expr_block.block.stmts, tracker)?;
            tracker.pop_scope();
        }
        Expr::ForLoop(expr_for) => {
            check_expr(&expr_for.expr, tracker)?;
            tracker.push_scope();
            let var_name = match &*expr_for.pat {
                Pat::Ident(pat_ident) => pat_ident.ident.to_string(),
                _ => {
                    return Err(syn::Error::new_spanned(
                        &expr_for.pat,
                        "Expected identifier for loop variable",
                    ));
                }
            };
            tracker.declare(var_name);
            check_stmts(&expr_for.body.stmts, tracker)?;
            tracker.pop_scope();
        }
        Expr::While(expr_while) => {
            check_expr(&expr_while.cond, tracker)?;
            tracker.push_scope();
            check_stmts(&expr_while.body.stmts, tracker)?;
            tracker.pop_scope();
        }
        Expr::Closure(closure) => {
            tracker.push_scope();
            for input in &closure.inputs {
                if let Pat::Ident(pat_ident) = input {
                    tracker.declare(pat_ident.ident.to_string());
                }
            }
            check_expr(&closure.body, tracker)?;
            tracker.pop_scope();
        }
        Expr::Lit(_) => {}
        _ => {}
    }
    Ok(())
}

fn check_macro(mac: &syn::Macro, tracker: &mut ScopeTracker) -> Result<(), syn::Error> {
    let parser = syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated;
    if let Ok(exprs) = mac.parse_body_with(parser) {
        let macro_name = mac
            .path
            .get_ident()
            .map(|ident| ident.to_string())
            .unwrap_or_default();
        let invalid_text_tool = match macro_name.as_str() {
            "jq" => !(1..=2).contains(&exprs.len()),
            "grep" | "awk" | "sed" => exprs.is_empty(),
            _ => false,
        };
        if invalid_text_tool {
            return Err(syn::Error::new_spanned(
                mac,
                format!("{macro_name}! expects at least one command argument"),
            ));
        }
        for expr in exprs {
            check_expr(&expr, tracker)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::{Stmt, parse_quote};

    #[test]
    fn test_scope_validation_success() {
        let stmts: Vec<Stmt> = parse_quote! {
            let config = "foo";
            cmd("echo", config);
        };
        let mut tracker = ScopeTracker::new();
        assert!(check_stmts(&stmts, &mut tracker).is_ok());
    }

    #[test]
    fn test_scope_validation_undefined_variable() {
        let stmts: Vec<Stmt> = parse_quote! {
            let config = "foo";
            cmd("echo", confg);
        };
        let mut tracker = ScopeTracker::new();
        let res = check_stmts(&stmts, &mut tracker);
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "Undefined variable 'confg'. Did you mean 'config'?"
        );
    }

    #[test]
    fn test_scope_validation_fn_arity_mismatch() {
        let stmts: Vec<Stmt> = parse_quote! {
            fn restart_service(name: String) {
                cmd("echo", name);
            }
            restart_service();
        };
        let mut tracker = ScopeTracker::new();
        let res = check_stmts(&stmts, &mut tracker);
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "Function 'restart_service' expects 1 argument(s), but 0 were given"
        );
    }

    #[test]
    fn test_scope_validation_undefined_fn_suggestion() {
        let stmts: Vec<Stmt> = parse_quote! {
            fn restart_service(name: String) {
                cmd("echo", name);
            }
            restrt_service("nginx");
        };
        let mut tracker = ScopeTracker::new();
        let res = check_stmts(&stmts, &mut tracker);
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "Undefined function 'restrt_service'. Did you mean 'restart_service'?"
        );
    }

    #[test]
    fn test_text_tool_arity_validation() {
        let stmts: Vec<Stmt> = parse_quote! {
            grep();
        };
        let mut tracker = ScopeTracker::new();
        let error = check_stmts(&stmts, &mut tracker).unwrap_err();
        assert_eq!(
            error.to_string(),
            "grep expects at least one command argument"
        );
    }
}
