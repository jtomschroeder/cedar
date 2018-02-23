#![feature(proc_macro)]

extern crate proc_macro as pm;

mod element;
mod parser;

#[proc_macro]
pub fn hypertext(input: pm::TokenStream) -> pm::TokenStream {
    let input = input.to_string();
    format!("::browser::log(\"Hello, world: '{}'\")", input)
        .parse()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use parser;

    #[test]
    fn parser() {
        assert!(parser::parse_Element("--").is_err());
        assert!(parser::parse_Element("<div></div>").is_ok());
        assert!(parser::parse_Element("<div>Hello, world!</div>").is_ok());

        println!("{:?}", parser::parse_Element("<div></div>"));
    }
}
