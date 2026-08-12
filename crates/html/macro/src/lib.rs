#![allow(clippy::needless_pass_by_value)]

extern crate proc_macro;

mod ast;
mod generate;

use proc_macro2::{Ident, Span};
use quote::quote;

#[proc_macro]
pub fn html(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input2 = proc_macro2::TokenStream::from(input);
    let size_hint = input2.to_string().len();

    let nodes = match syn::parse2::<ast::Nodes>(input2) {
        Ok(n) => n,
        Err(e) => return e.to_compile_error().into(),
    };

    let out = Ident::new("__html_out", Span::mixed_site());
    let stmts = generate::generate(nodes, &out);

    quote! {{
        extern crate html_rt;
        let __html_session = ::html_rt::reactive_session();
        let __html_session_id = __html_session.id.clone();
        let mut #out = ::std::string::String::with_capacity(#size_hint);
        #stmts
        if #out.contains("id=\"__slot_") {
            #out.push_str(&::std::format!(
                "<script>new EventSource('/_openoxide/html/events/{}').onmessage=e=>{{const p=JSON.parse(e.data),n=document.getElementById(p.slot);if(n)n.outerHTML=p.html}}</script>",
                __html_session_id
            ));
        }
        ::html_rt::PreEscaped(#out)
    }}
    .into()
}
