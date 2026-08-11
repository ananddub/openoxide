/// AST nodes for `html! { <div>...</div> }` syntax.
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    token::{Gt, Lt, Slash},
    Expr, Ident, LitStr, Token,
};

// ── Top-level ──────────────────────────────────────────────────────────────────

pub struct Nodes(pub Vec<Node>);

impl Parse for Nodes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut nodes = Vec::new();
        while !input.is_empty() {
            // closing tag aa gaya — parent handle karega
            if input.peek(Token![<]) && input.peek2(Token![/]) {
                break;
            }
            nodes.push(input.parse()?)
        }
        Ok(Nodes(nodes))
    }
}

// ── Node ───────────────────────────────────────────────────────────────────────

pub enum Node {
    Doctype,
    Element(Element),
    Text(LitStr),
    Expr(Expr),
    ControlFlow(ControlFlow),
}

impl Parse for Node {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // `<!DOCTYPE html>`
        if input.peek(Token![<]) && input.peek2(Token![!]) {
            return parse_doctype(input);
        }
        // `<tag ...>`
        if input.peek(Token![<]) {
            return input.parse::<Element>().map(Node::Element);
        }
        // `"text"`
        if input.peek(LitStr) {
            return input.parse::<LitStr>().map(Node::Text);
        }
        // `{expr}`
        if input.peek(syn::token::Brace) {
            let content;
            syn::braced!(content in input);
            return content.parse::<Expr>().map(Node::Expr);
        }
        // `@if / @for / @while / @match`
        if input.peek(Token![@]) {
            return input.parse::<ControlFlow>().map(Node::ControlFlow);
        }
        Err(input.error("expected `<tag>`, \"text\", `{expr}`, or `@if`/`@for`/`@match`"))
    }
}

fn parse_doctype(input: ParseStream) -> syn::Result<Node> {
    input.parse::<Token![<]>()?;
    input.parse::<Token![!]>()?;
    let ident: Ident = input.parse()?;
    if ident != "DOCTYPE" {
        return Err(syn::Error::new(ident.span(), "expected `DOCTYPE`"));
    }
    let _html: Ident = input.parse()?;
    input.parse::<Token![>]>()?;
    Ok(Node::Doctype)
}

// ── Element ────────────────────────────────────────────────────────────────────

pub struct Element {
    pub name: TagName,
    pub attrs: Vec<Attr>,
    /// None = self-closing `<input />`
    pub children: Option<Nodes>,
}

impl Parse for Element {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Lt>()?;
        let name: TagName = input.parse()?;
        let mut attrs = Vec::new();

        while !input.peek(Gt) && !(input.peek(Slash) && input.peek2(Gt)) {
            attrs.push(input.parse()?)
        }

        // self-closing `<br />` or `<input />`
        if input.peek(Slash) {
            input.parse::<Slash>()?;
            input.parse::<Gt>()?;
            return Ok(Element {
                name,
                attrs,
                children: None,
            });
        }

        input.parse::<Gt>()?;

        let children: Nodes = input.parse()?;

        // `</tagname>`
        input.parse::<Token![<]>()?;
        input.parse::<Token![/]>()?;
        let _: TagName = input.parse()?;
        input.parse::<Token![>]>()?;

        Ok(Element {
            name,
            attrs,
            children: Some(children),
        })
    }
}

// ── TagName ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct TagName(pub String);

impl Parse for TagName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let first: Ident = input.call(Ident::parse_any)?;
        let mut name = first.to_string();
        loop {
            if input.peek(Token![-]) {
                input.parse::<Token![-]>()?;
                name.push('-');
            } else if input.peek(Token![:]) {
                input.parse::<Token![:]>()?;
                name.push(':');
            } else {
                break;
            }
            if input.peek(Ident::peek_any) {
                let seg: Ident = input.call(Ident::parse_any)?;
                name.push_str(&seg.to_string());
            }
        }
        Ok(TagName(name))
    }
}

// ── Attr ───────────────────────────────────────────────────────────────────────

pub enum AttrKind {
    Normal { name: AttrName, value: AttrValue },
    On { event: String, method: syn::Expr },
    Bind(String),
    Toggle(String),
    Show(String),
    Hide(String),
    Signals(Vec<(String, syn::Expr)>),
}

pub struct Attr {
    pub kind: AttrKind,
}

impl Parse for Attr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: AttrName = input.parse()?;
        let name_str = name.0.as_str();

        if name_str == "signals" {
            if input.peek(Token![=]) {
                input.parse::<Token![=]>()?;
                let content;
                syn::braced!(content in input);
                let mut pairs = Vec::new();
                while !content.is_empty() {
                    let k: Ident = content.parse()?;
                    content.parse::<Token![:]>()?;
                    let v: Expr = content.parse()?;
                    pairs.push((k.to_string(), v));
                    if content.peek(Token![,]) {
                        content.parse::<Token![,]>()?;
                    }
                }
                return Ok(Attr {
                    kind: AttrKind::Signals(pairs),
                });
            }
            return Err(input.error("expected `={k: expr, ...}` for signals"));
        }

        let mut value = AttrValue::Empty;
        if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;

            if input.peek(LitStr) {
                value = AttrValue::Lit(input.parse()?);
            } else if input.peek(syn::token::Brace) {
                let content;
                syn::braced!(content in input);
                value = AttrValue::Expr(content.parse()?);
            } else {
                return Err(input.error("expected string literal or `{expr}`"));
            }
        }

        if let Some(event) = name_str.strip_prefix("on:") {
            if let AttrValue::Expr(method) = value {
                return Ok(Attr {
                    kind: AttrKind::On {
                        event: event.to_string(),
                        method,
                    },
                });
            } else {
                return Err(input.error("on: event requires `{expr}` value"));
            }
        }
        if let Some(field) = name_str.strip_prefix("bind:") {
            return Ok(Attr {
                kind: AttrKind::Bind(field.to_string()),
            });
        }
        if let Some(signal) = name_str.strip_prefix("toggle:") {
            return Ok(Attr {
                kind: AttrKind::Toggle(signal.to_string()),
            });
        }
        if let Some(signal) = name_str.strip_prefix("show:") {
            return Ok(Attr {
                kind: AttrKind::Show(signal.to_string()),
            });
        }
        if let Some(signal) = name_str.strip_prefix("hide:") {
            return Ok(Attr {
                kind: AttrKind::Hide(signal.to_string()),
            });
        }

        Ok(Attr {
            kind: AttrKind::Normal { name, value },
        })
    }
}

// ── AttrName ───────────────────────────────────────────────────────────────────
// Supports: class, data-on:click, data-bind:input, data-on:keydown.enter

#[derive(Debug, Clone)]
pub struct AttrName(pub String);

impl Parse for AttrName {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let first: Ident = input.call(Ident::parse_any)?;
        let mut name = first.to_string();
        loop {
            if input.peek(Token![-]) {
                input.parse::<Token![-]>()?;
                name.push('-');
            } else if input.peek(Token![:]) {
                input.parse::<Token![:]>()?;
                name.push(':');
            } else if input.peek(Token![.]) {
                input.parse::<Token![.]>()?;
                name.push('.');
            } else {
                break;
            }
            if input.peek(Ident::peek_any) {
                let seg: Ident = input.call(Ident::parse_any)?;
                name.push_str(&seg.to_string());
            }
        }
        Ok(AttrName(name))
    }
}

pub enum AttrValue {
    Lit(LitStr),
    Expr(Expr),
    Empty,
}

// ── ControlFlow ────────────────────────────────────────────────────────────────

pub enum ControlFlow {
    If(IfNode),
    For(ForNode),
    While(WhileNode),
    Match(MatchNode),
}

impl Parse for ControlFlow {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![@]>()?;
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![if]) {
            input.parse().map(ControlFlow::If)
        } else if lookahead.peek(Token![for]) {
            input.parse().map(ControlFlow::For)
        } else if lookahead.peek(Token![while]) {
            input.parse().map(ControlFlow::While)
        } else if lookahead.peek(Token![match]) {
            input.parse().map(ControlFlow::Match)
        } else {
            Err(lookahead.error())
        }
    }
}

// ── @if ────────────────────────────────────────────────────────────────────────

pub struct IfNode {
    pub cond: Expr,
    pub then_nodes: Nodes,
    pub else_nodes: Option<ElseBranch>,
    pub is_async: bool,
}

impl Parse for IfNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![if]>()?;
        let cond = Expr::parse_without_eager_brace(input)?;
        let content;
        syn::braced!(content in input);
        let then_nodes: Nodes = content.parse()?;

        let else_nodes = if input.peek(Token![@]) {
            let fork = input.fork();
            let _ = fork.parse::<Token![@]>();
            if fork.peek(Token![else]) {
                input.parse::<Token![@]>()?;
                Some(input.parse()?)
            } else {
                None
            }
        } else {
            None
        };

        let is_async = quote::quote!(#cond).to_string().contains(".await");

        Ok(IfNode {
            cond,
            then_nodes,
            else_nodes,
            is_async,
        })
    }
}

pub enum ElseBranch {
    ElseIf(Box<IfNode>),
    Else(Nodes),
}

impl Parse for ElseBranch {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![else]>()?;
        if input.peek(Token![if]) {
            input.parse().map(|n| ElseBranch::ElseIf(Box::new(n)))
        } else {
            let content;
            syn::braced!(content in input);
            Ok(ElseBranch::Else(content.parse()?))
        }
    }
}

// ── @for ───────────────────────────────────────────────────────────────────────

pub struct ForNode {
    pub pat: syn::Pat,
    pub expr: Expr,
    pub body: Nodes,
    pub is_async: bool,
}

impl Parse for ForNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![for]>()?;
        let pat = syn::Pat::parse_multi_with_leading_vert(input)?;
        input.parse::<Token![in]>()?;
        let expr = Expr::parse_without_eager_brace(input)?;
        let content;
        syn::braced!(content in input);
        let is_async = quote::quote!(#expr).to_string().contains(".await");
        Ok(ForNode {
            pat,
            expr,
            body: content.parse()?,
            is_async,
        })
    }
}

// ── @while ─────────────────────────────────────────────────────────────────────

pub struct WhileNode {
    pub cond: Expr,
    pub body: Nodes,
}

impl Parse for WhileNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![while]>()?;
        let cond = Expr::parse_without_eager_brace(input)?;
        let content;
        syn::braced!(content in input);
        Ok(WhileNode {
            cond,
            body: content.parse()?,
        })
    }
}

// ── @match ─────────────────────────────────────────────────────────────────────

pub struct MatchArm {
    pub pat: syn::Pat,
    pub guard: Option<Expr>,
    pub body: Nodes,
}

pub struct MatchNode {
    pub expr: Expr,
    pub arms: Vec<MatchArm>,
}

impl Parse for MatchNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![match]>()?;
        let expr = Expr::parse_without_eager_brace(input)?;
        let arms_content;
        syn::braced!(arms_content in input);
        let mut arms = Vec::new();
        while !arms_content.is_empty() {
            let pat = syn::Pat::parse_multi_with_leading_vert(&arms_content)?;
            let guard = if arms_content.peek(Token![if]) {
                arms_content.parse::<Token![if]>()?;
                Some(arms_content.parse::<Expr>()?)
            } else {
                None
            };
            arms_content.parse::<Token![=>]>()?;
            let body_content;
            syn::braced!(body_content in arms_content);
            let body: Nodes = body_content.parse()?;
            if arms_content.peek(Token![,]) {
                arms_content.parse::<Token![,]>()?;
            }
            arms.push(MatchArm { pat, guard, body });
        }
        Ok(MatchNode { expr, arms })
    }
}
