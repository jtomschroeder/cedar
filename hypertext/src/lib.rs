#![feature(proc_macro)]

extern crate proc_macro;

mod parser;

use proc_macro::TokenStream;

#[proc_macro]
pub fn hypertext(input: TokenStream) -> TokenStream {
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
        assert!(parser::parse_Term("22").is_ok());
        assert!(parser::parse_Term("(22)").is_ok());
        assert!(parser::parse_Term("((((22))))").is_ok());
        assert!(parser::parse_Term("((22)").is_err());
    }
}
