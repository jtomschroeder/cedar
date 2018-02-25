#![allow(dead_code)]
#![feature(proc_macro)]

extern crate proc_macro;

use proc_macro::TokenStream;

#[macro_use]
extern crate quote;
extern crate syn;

mod parser;

#[proc_macro]
pub fn hypertext(input: TokenStream) -> TokenStream {
    //    let input: syn::Expr = syn::parse(input).unwrap();

    let input = input.to_string();
    let dom = parser::parse(&input).unwrap();
    //    format!(r##"cedar::browser::log(r#"Hello, world: {:?}"#)"##, dom)
    //        .parse()
    //        .unwrap()

    println!("DOM (parsed): {:#?}", dom);

    let dom = dom.render();

    println!("DOM (rendered): {:#?}", dom);

    dom.into()
}

impl parser::Element {
    fn render(self) -> quote::Tokens {
        match self {
            parser::Element::Element {
                name,
                attributes,
                mut children,
            } => {
                let children: Vec<_> = children.drain(..).map(Self::render).collect();
                quote! { ::cedar::dom::Object::new(#name).children( vec![ #(#children),* ] ) }
            }

            parser::Element::Text(text) => {
                quote! { ::cedar::dom::text(#text) }
            }
        }
    }
}
