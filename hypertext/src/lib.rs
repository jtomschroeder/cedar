extern crate proc_macro as pm;
extern crate proc_macro2 as pm2;

#[macro_use]
extern crate quote;

mod parser;

#[proc_macro]
pub fn hypertext(tokens: pm::TokenStream) -> pm::TokenStream {
    let tokens = tokens.to_string();

    let dom = parser::parse(&tokens).unwrap();
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
                quote! { ::cedar::dom::text(#text) }
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
        let name: pm2::TokenStream = self.name.parse().unwrap();
        let block: pm2::TokenStream = self.block.parse().unwrap();

        // TODO?: if value is 'true' or 'false' -> add or remove element without ="..."

        match self.name.as_str() {
            "click" | "input" | "keydown" => quote! { .#name(#block) },
            _ => quote! { .attr(stringify!(#name), #block) },
        }
    }
}
