#![feature(proc_macro)]

extern crate proc_macro;

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
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
