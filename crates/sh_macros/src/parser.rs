use syn::Stmt;
use syn::parse::Parse;

/// A statement inside `sh! { ... }`.
/// Either a regular Rust statement, or a shell function definition
/// with untyped parameters: `fn name(p1, p2) { ... }`.
pub enum ShStmt {
    Syn(Stmt),
    ShFn {
        name: String,
        params: Vec<String>,
        body: Vec<ShStmt>,
    },
}

pub struct ShInput {
    pub stmts: Vec<ShStmt>,
}

impl Parse for ShInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut stmts = Vec::new();
        while !input.is_empty() {
            stmts.push(input.parse()?);
        }
        Ok(ShInput { stmts })
    }
}

impl Parse for ShStmt {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Peek ahead: is it `fn <ident> (`?
        if input.peek(syn::Token![fn]) {
            // consume `fn`
            let fn_token: syn::Token![fn] = input.parse()?;
            let name: syn::Ident = input.parse()?;

            // Parse parameter list: `(p1, p2, ...)` — identifiers only, no types
            let content;
            syn::parenthesized!(content in input);
            let mut params = Vec::new();
            while !content.is_empty() {
                let param: syn::Ident = content.parse()?;
                params.push(param.to_string());
                if content.peek(syn::Token![,]) {
                    let _: syn::Token![,] = content.parse()?;
                }
            }

            // Parse body block
            let body_content;
            syn::braced!(body_content in input);
            let mut body = Vec::new();
            while !body_content.is_empty() {
                body.push(body_content.parse::<ShStmt>()?);
            }

            let _ = fn_token; // suppress unused warning
            return Ok(ShStmt::ShFn {
                name: name.to_string(),
                params,
                body,
            });
        }

        // Regular Rust statement
        Ok(ShStmt::Syn(input.parse()?))
    }
}
