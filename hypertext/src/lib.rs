#![allow(dead_code)]
#![feature(proc_macro)]

extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;

#[macro_use]
extern crate failure;

#[macro_use]
extern crate quote;
extern crate syn;

mod parser;

#[derive(Fail, Debug)]
enum Error {
    #[fail(display = "Failed to lex.")] Lex(proc_macro::LexError),
    #[fail(display = "Failed to parse.")] Parse(syn::synom::ParseError),
}

#[proc_macro]
pub fn hypertext(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let dom = parser::parse(&input).unwrap();

    dom.render().into()
}

fn parse<T: syn::synom::Synom>(input: &str) -> Result<T, Error> {
//    println!("input: {}", input);
    input
        .parse()
        .map_err(Error::Lex)
        .and_then(|input| syn::parse(input).map_err(Error::Parse))
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
                quote! { ::cedar::dom::object(#text) }
            }

            parser::Element::Block(block) => {
//                let block: syn::Expr = parse(&block).unwrap();
                let block: proc_macro2::TokenStream = block.parse().unwrap();

                quote! { ::cedar::dom::object(#block) }
            }
        }
    }
}

impl parser::Attribute {
    fn render(self) -> quote::Tokens {
        // TODO: for attrs other than 'click', use 'attr()' method

        let name: syn::Ident = parse(&self.name).unwrap();
//        let block: syn::ExprCall = parse(&self.block).unwrap();
        let block: proc_macro2::TokenStream = self.block.parse().unwrap();

        quote! { .#name(#block) }
    }
}
