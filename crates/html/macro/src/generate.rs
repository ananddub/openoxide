/// HTML AST → `String::push_str` calls.
/// Inspired by Maud's generator but for `<tag>` style syntax.
use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::ast::*;
use std::sync::atomic::{AtomicU32, Ordering};

static SLOT_COUNTER: AtomicU32 = AtomicU32::new(0);

fn extract_table_and_inner_expr(expr: &syn::Expr) -> (Option<String>, syn::Expr) {
    if let syn::Expr::Index(index_expr) = expr {
        if let syn::Expr::Await(await_expr) = &*index_expr.expr {
            if let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(s),
                ..
            }) = &*index_expr.index
            {
                return (Some(s.value()), (*await_expr.base).clone());
            }
        }
    }
    if let syn::Expr::Await(await_expr) = expr {
        return (None, (*await_expr.base).clone());
    }
    (None, expr.clone())
}

fn extract_let_await(cond: &syn::Expr) -> (syn::Pat, syn::Expr, Option<String>) {
    if let syn::Expr::Let(let_expr) = cond {
        let (table_opt, inner_expr) = extract_table_and_inner_expr(&let_expr.expr);
        let full_pat = &*let_expr.pat;
        return (full_pat.clone(), inner_expr, table_opt);
    }
    panic!("async @if must be: @if let Some(x) = expr.await or expr.await[\"table\"]");
}

/// Extract the inner binding from `Some(x)` pattern → returns `x` pattern
fn unwrap_some_pat(pat: &syn::Pat) -> syn::Pat {
    if let syn::Pat::TupleStruct(ts) = pat {
        // Check if it's Some(...)
        let seg = ts.path.segments.last().map(|s| s.ident.to_string());
        if seg.as_deref() == Some("Some") {
            if let Some(inner) = ts.elems.first() {
                return inner.clone();
            }
        }
    }
    pat.clone()
}

/// Resolve `Self::method` and `Self::method(arg)` into the generated route path.
fn transform_route(method: &syn::Expr) -> TokenStream {
    let (method, args) = match method {
        syn::Expr::Call(call) => (&*call.func, call.args.iter().collect::<Vec<_>>()),
        method => (method, Vec::new()),
    };

    if let syn::Expr::Path(expr_path) = method {
        if expr_path.path.segments.len() > 1 {
            let mut new_path = expr_path.clone();
            if let Some(last_seg) = new_path.path.segments.last_mut() {
                let ident_str = last_seg.ident.to_string();
                if !ident_str.starts_with("__PATH_") {
                    last_seg.ident =
                        syn::Ident::new(&format!("__PATH_{}", ident_str), last_seg.ident.span());
                }
            }
            if args.is_empty() {
                return quote! { #new_path };
            }
            return quote! {{
                let mut __route = (#new_path).to_owned();
                #(
                    if let (::std::option::Option::Some(__start), ::std::option::Option::Some(__end)) =
                        (__route.find('{'), __route.find('}'))
                    {
                        __route.replace_range(__start..=__end, &::std::format!("{}", #args));
                    }
                )*
                __route
            }};
        }
    }
    quote! { #method }
}

pub fn generate(nodes: Nodes, out: &Ident) -> TokenStream {
    let mut b = Builder::new(out.clone());
    gen_nodes(nodes, &mut b, out);
    b.finish()
}

fn gen_nodes(nodes: Nodes, b: &mut Builder, out: &Ident) {
    for node in nodes.0 {
        gen_node(node, b, out);
    }
}

fn gen_node(node: Node, b: &mut Builder, out: &Ident) {
    match node {
        Node::Doctype => b.push_raw("<!DOCTYPE html>"),

        Node::Text(lit) => {
            let s = lit.value();
            b.push_escaped(&s);
        }

        Node::Expr(expr) => {
            b.flush();
            b.tokens.extend(quote! {
                ::html_rt::macro_private::render_to!(&(#expr), &mut #out);
            });
        }

        Node::Element(el) => gen_element(el, b, out),

        Node::ControlFlow(cf) => gen_control_flow(cf, b, out),
    }
}

fn gen_element(el: Element, b: &mut Builder, out: &Ident) {
    let name = &el.name.0;

    b.push_raw("<");
    b.push_raw(name);

    for attr in el.attrs {
        gen_attr(attr, b, out);
    }

    match el.children {
        None => {
            b.push_raw(" />");
        }
        Some(children) => {
            b.push_raw(">");
            gen_nodes(children, b, out);
            b.push_raw("</");
            b.push_raw(name);
            b.push_raw(">");
        }
    }
}

fn gen_attr(attr: Attr, b: &mut Builder, out: &Ident) {
    match attr.kind {
        AttrKind::Normal { name, value } => {
            let name_str = &name.0;
            match value {
                AttrValue::Empty => {
                    b.push_raw(" ");
                    b.push_raw(name_str);
                }
                AttrValue::Lit(lit) => {
                    let val = lit.value();
                    b.push_raw(" ");
                    b.push_raw(name_str);
                    b.push_raw("=\"");
                    b.push_escaped(&val);
                    b.push_raw("\"");
                }
                AttrValue::Expr(expr) => {
                    b.push_raw(" ");
                    b.push_raw(name_str);
                    b.push_raw("=\"");
                    b.flush();
                    if name_str == "action" {
                        let route = transform_route(&expr);
                        b.tokens.extend(quote! {
                            ::html_rt::macro_private::render_to!(&(#route), &mut #out);
                        });
                    } else {
                        b.tokens.extend(quote! {
                            ::html_rt::macro_private::render_to!(&(#expr), &mut #out);
                        });
                    }
                    b.push_raw("\"");
                }
            }
        }
        AttrKind::On { event, method } => {
            let resolved_method = transform_route(&method);
            b.flush();
            b.tokens.extend(quote! {
                #out.push_str(&format!(" data-on:{}=\"@post('", #event));
                #out.push_str(&#resolved_method);
                #out.push_str("')\"");
            });
        }
        AttrKind::Bind(field) => {
            b.push_raw(&format!(" data-bind:{}", field));
        }
        AttrKind::Toggle(signal) => {
            b.push_raw(&format!(" data-on:click=\"${s} = !${s}\"", s = signal));
        }
        AttrKind::Show(signal) => {
            b.push_raw(&format!(" data-show=\"${}\"", signal));
        }
        AttrKind::Hide(signal) => {
            b.push_raw(&format!(" data-show=\"!${}\"", signal));
        }
        AttrKind::Signals(pairs) => {
            b.flush();
            let mut keys = Vec::new();
            let mut vals = Vec::new();
            for (k, v) in pairs {
                keys.push(format!("\\\"{}\\\":{{}}", k));
                vals.push(v);
            }
            let inner = keys.join(",");
            let fmt_str = format!(" data-signals=\"{{{{{}}}}}\"", inner);

            b.tokens.extend(quote! {
                #out.push_str(&format!(#fmt_str, #(#vals),*));
            });
        }
    }
}

fn gen_control_flow(cf: ControlFlow, b: &mut Builder, out: &Ident) {
    match cf {
        ControlFlow::If(if_node) => gen_if(if_node, b, out),
        ControlFlow::For(for_node) => gen_for(for_node, b, out),
        ControlFlow::While(wh) => gen_while(wh, b, out),
        ControlFlow::Match(m) => gen_match(m, b, out),
    }
}

fn gen_if(node: IfNode, b: &mut Builder, out: &Ident) {
    if node.is_async {
        return gen_if_async(node, b, out);
    }

    let cond = node.cond;
    let then_ts = {
        let mut inner = Builder::new(out.clone());
        gen_nodes(node.then_nodes, &mut inner, out);
        inner.finish()
    };

    b.flush();

    match node.else_nodes {
        None => {
            b.tokens.extend(quote! { if #cond { #then_ts } });
        }
        Some(ElseBranch::Else(else_nodes)) => {
            let else_ts = {
                let mut inner = Builder::new(out.clone());
                gen_nodes(else_nodes, &mut inner, out);
                inner.finish()
            };
            b.tokens
                .extend(quote! { if #cond { #then_ts } else { #else_ts } });
        }
        Some(ElseBranch::ElseIf(else_if)) => {
            let else_ts = {
                let mut inner = Builder::new(out.clone());
                gen_if(*else_if, &mut inner, out);
                inner.finish()
            };
            b.tokens
                .extend(quote! { if #cond { #then_ts } else { #else_ts } });
        }
    }
}

fn gen_if_async(node: IfNode, b: &mut Builder, out: &Ident) {
    let slot_n = SLOT_COUNTER.fetch_add(1, Ordering::Relaxed);
    let slot_id_str = format!("__slot_{}", slot_n);

    let (full_pat, inner_expr, table_opt) = extract_let_await(&node.cond);
    let inner_pat = unwrap_some_pat(&full_pat);

    let table_tokens = match table_opt {
        Some(tbl) => quote! { ::std::option::Option::Some(#tbl) },
        None => quote! { ::std::option::Option::None },
    };

    let mut else_b = Builder::new(out.clone());
    if let Some(else_nodes) = node.else_nodes {
        match else_nodes {
            ElseBranch::Else(nodes) => gen_nodes(nodes, &mut else_b, out),
            ElseBranch::ElseIf(else_if) => gen_if(*else_if, &mut else_b, out),
        }
    }
    let else_tokens = else_b.finish();

    let task_out = quote::format_ident!("__task_out");
    let mut then_b = Builder::new(task_out.clone());
    gen_nodes(node.then_nodes, &mut then_b, &task_out);
    let then_tokens = then_b.finish();

    b.flush();
    b.tokens.extend(quote! {
        #out.push_str(&::std::format!("<div id=\"{}\">", #slot_id_str));
        { #else_tokens }
        #out.push_str("</div>");

        {
            let __slot_id = #slot_id_str;
            let __session = __html_session.clone();
            let __target_table: ::std::option::Option<&'static str> = #table_tokens;
            ::tokio::spawn(async move {
                let mut __changes = ::html_rt::subscribe_table_changes();
                loop {
                    if let Some(#inner_pat) = (#inner_expr).await {
                        let mut __task_out = ::std::string::String::new();
                        { #then_tokens }
                        let patch = ::std::format!("<div id=\"{}\">{}</div>", __slot_id, __task_out);
                        if !__session.send(__slot_id, patch).await { break; }
                    }
                    let Some(table) = __target_table else { break; };
                    loop {
                        match __changes.recv().await {
                            Ok(tables) if tables.iter().any(|changed| changed == table) => break,
                            Ok(_) => continue,
                            Err(_) => return,
                        }
                    }
                }
            });
        }
    });
}

fn gen_for(node: ForNode, b: &mut Builder, out: &Ident) {
    if node.is_async {
        return gen_for_async(node, b, out);
    }

    let pat = node.pat;
    let expr = node.expr;
    let body_ts = {
        let mut inner = Builder::new(out.clone());
        gen_nodes(node.body, &mut inner, out);
        inner.finish()
    };
    b.flush();
    b.tokens.extend(quote! { for #pat in (#expr) { #body_ts } });
}

fn gen_for_async(node: ForNode, b: &mut Builder, out: &Ident) {
    let slot_n = SLOT_COUNTER.fetch_add(1, Ordering::Relaxed);
    let slot_id_str = format!("__slot_{}", slot_n);

    let (table_opt, inner_expr) = extract_table_and_inner_expr(&node.expr);
    let pat = &node.pat;

    let table_tokens = match table_opt {
        Some(tbl) => quote! { ::std::option::Option::Some(#tbl) },
        None => quote! { ::std::option::Option::None },
    };

    let task_out = Ident::new("__task_out", proc_macro2::Span::call_site());
    let mut body_b = Builder::new(task_out.clone());
    gen_nodes(node.body, &mut body_b, &task_out);
    let body_tokens = body_b.finish();

    b.flush();
    b.tokens.extend(quote! {
        #out.push_str(&::std::format!("<div id=\"{}\">", #slot_id_str));
        #out.push_str("</div>");

        {
            let __slot_id = #slot_id_str;
            let __session = __html_session.clone();
            let __target_table: ::std::option::Option<&'static str> = #table_tokens;
            ::tokio::spawn(async move {
                let mut __changes = ::html_rt::subscribe_table_changes();
                loop {
                    let __items = (#inner_expr).await;
                    let mut __patch = ::std::string::String::new();
                    for #pat in __items {
                        let mut #task_out = ::std::string::String::new();
                        { #body_tokens }
                        __patch.push_str(&#task_out);
                    }
                    let patch = ::std::format!("<div id=\"{}\">{}</div>", __slot_id, __patch);
                    if !__session.send(__slot_id, patch).await { break; }
                    let Some(table) = __target_table else { break; };
                    loop {
                        match __changes.recv().await {
                            Ok(tables) if tables.iter().any(|changed| changed == table) => break,
                            Ok(_) => continue,
                            Err(_) => return,
                        }
                    }
                }
            });
        }
    });
}

fn gen_while(node: WhileNode, b: &mut Builder, out: &Ident) {
    let cond = node.cond;
    let body_ts = {
        let mut inner = Builder::new(out.clone());
        gen_nodes(node.body, &mut inner, out);
        inner.finish()
    };
    b.flush();
    b.tokens.extend(quote! { while #cond { #body_ts } });
}

fn gen_match(node: MatchNode, b: &mut Builder, out: &Ident) {
    let expr = node.expr;
    b.flush();

    let mut arms_ts = TokenStream::new();
    for arm in node.arms {
        let pat = arm.pat;
        let body_ts = {
            let mut inner = Builder::new(out.clone());
            gen_nodes(arm.body, &mut inner, out);
            inner.finish()
        };
        if let Some(guard) = arm.guard {
            arms_ts.extend(quote! { #pat if #guard => { #body_ts }, });
        } else {
            arms_ts.extend(quote! { #pat => { #body_ts }, });
        }
    }

    b.tokens.extend(quote! { match #expr { #arms_ts } });
}

// ── Builder ───────────────────────────────────────────────────────────────────

pub struct Builder {
    out: Ident,
    pub tokens: TokenStream,
    tail: String,
}

impl Builder {
    pub fn new(out: Ident) -> Self {
        Self {
            out,
            tokens: TokenStream::new(),
            tail: String::new(),
        }
    }

    pub fn push_raw(&mut self, s: &str) {
        self.tail.push_str(s);
    }

    pub fn push_escaped(&mut self, s: &str) {
        for b in s.bytes() {
            match b {
                b'&' => self.tail.push_str("&amp;"),
                b'<' => self.tail.push_str("&lt;"),
                b'>' => self.tail.push_str("&gt;"),
                b'"' => self.tail.push_str("&quot;"),
                _ => self.tail.push(b as char),
            }
        }
    }

    pub fn flush(&mut self) {
        if self.tail.is_empty() {
            return;
        }
        let s = &self.tail;
        let out = &self.out;
        self.tokens.extend(quote! { #out.push_str(#s); });
        self.tail.clear();
    }

    pub fn finish(mut self) -> TokenStream {
        self.flush();
        self.tokens
    }
}
