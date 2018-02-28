#![feature(proc_macro)]

extern crate proc_macro as pm;
extern crate proc_macro2 as pm2;

#[macro_use]
extern crate quote;

mod parser;

// HACK!
// Workaround for https://github.com/rust-lang/rust/issues/46489
// hypertext! returns a closure in order to allow usage of local bindings in macro

#[proc_macro]
pub fn hypertext(input: pm::TokenStream) -> pm::TokenStream {
    let input = input.to_string();

    let args = {
        let mut count = 2;
        let args = input
            .chars()
            .take_while(move |&c| {
                if c == '|' {
                    count -= 1;
                }
                count > 0
            })
            .count();
        &input[..args + 1]
    };

    let input = &input[args.len()..];
    let dom = parser::parse(input).unwrap();

    // TODO: (HACK) someday just `dom.render().into()`

    let args: pm2::TokenStream = args.parse().unwrap();
    let dom = dom.render();
    let dom = quote! { #args { #dom } };
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
