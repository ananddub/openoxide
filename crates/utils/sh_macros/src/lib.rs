extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod convert;
mod parser;

use convert::convert_sh_stmt;
use parser::ShInput;

std::thread_local! {
    pub(crate) static SH_VARS: std::cell::RefCell<std::collections::HashSet<String>> =
        std::cell::RefCell::new(std::collections::HashSet::new());
}

#[proc_macro]
pub fn sh(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ShInput);
    let mut ir_constructions = Vec::new();

    // 1. Scope-check pass — collects all declared variable names
    let mut tracker = convert::scope::ScopeTracker::new();
    if let Err(err) = convert::scope::check_sh_stmts(&input.stmts, &mut tracker) {
        return err.to_compile_error().into();
    }

    // 2. Store full variable set in thread-local for substitute_sh_vars
    SH_VARS.with(|cell| {
        *cell.borrow_mut() = tracker.all_vars();
    });

    // 3. Code-generation pass
    for stmt in &input.stmts {
        match convert_sh_stmt(stmt) {
            Ok(tokens) => ir_constructions.push(tokens),
            Err(err) => return err.to_compile_error().into(),
        }
    }

    // 4. Clear thread-local
    SH_VARS.with(|cell| cell.borrow_mut().clear());

    let expanded = quote! {
        ({
            use crate::utils::exec::script::dsl::IntoShellIRGeneric;
            let mut steps: Vec<crate::utils::exec::script::dsl::ShellIR> = Vec::new();
            #( steps.push(crate::utils::exec::script::dsl::ShellIRWrapper(#ir_constructions).into_shell_ir()); )*
            steps
        })
    };

    TokenStream::from(expanded)
}
