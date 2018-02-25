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

    //    println!("DOM (parsed): {:#?}", dom);

    let dom = dom.render();

    //    println!("DOM (rendered): {:#?}", dom);

    dom.into()
}

impl parser::Element {
    fn render(self) -> quote::Tokens {
        match self {
            parser::Element::Element {
                name,
                mut attributes,
                mut children,
            } => {
                let attributes: Vec<_> = attributes.drain(..).map(|a| a.render()).collect();
                let children: Vec<_> = children.drain(..).map(Self::render).collect();
                quote! {
                    ::cedar::dom::Object::new(#name)
                        #( #attributes )*
                        .children( vec![ #( #children ),* ] )
                }
            }

            parser::Element::Text(text) => {
                quote! { ::cedar::dom::text(#text) }
            }
        }
    }
}

impl parser::Attribute {
    fn render(self) -> quote::Tokens {
        // TODO: for attrs other than 'click', use 'attr()' method

        let name: syn::Ident = syn::parse(self.name.parse().unwrap()).unwrap();
        let block: syn::Expr = syn::parse(self.block.parse().unwrap()).unwrap();

        quote! { .#name(#block) }
    }
}
