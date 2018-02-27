#![feature(proc_macro)]

extern crate proc_macro2 as pm2;
extern crate proc_macro as pm;

#[macro_use]
extern crate quote;

mod parser;

#[proc_macro]
pub fn hypertext(input: pm::TokenStream) -> pm::TokenStream {
    let input = input.to_string();
    let dom = parser::parse(&input).unwrap();
    dom.render().into()
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
                        #( .push( #children ) )*
                }
            }

            parser::Element::Text(text) => {
                quote! { ::cedar::dom::object(#text) }
            }

            parser::Element::Block(block) => {
                let block: pm2::TokenStream = block.parse().unwrap();
                quote! { #block }
            }
        }
    }
}

impl parser::Attribute {
    fn render(self) -> quote::Tokens {
        // TODO: for attrs other than 'click', use 'attr()' method

        let name: pm2::TokenStream = self.name.parse().unwrap();
        let block: pm2::TokenStream = self.block.parse().unwrap();

        quote! { .#name(#block) }
    }
}
