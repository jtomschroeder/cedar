
#![deny(trivial_casts, trivial_numeric_casts)]
#![deny(unused_import_braces, unused_qualifications)]
// #![deny(missing_docs)]
// #![deny(unsafe_code, unstable_features)]
// #![deny(missing_debug_implementations, missing_copy_implementations)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
pub extern crate serde_json as json;

extern crate sass_rs as sass;
extern crate web_view;

extern crate cedar_hypertext as hypertext;

mod boo;

#[macro_use]
mod tree;

mod application;
mod renderer;
mod shadow;

pub mod dom;

pub use application::{app, Application};
pub use hypertext::hypertext;

// TODO: move into own module or crate
/// build.rs helper
pub fn custom_style(path: &str) {
    let css = sass::compile_file(path, sass::Options::default()).unwrap();

    use std::fs::File;
    use std::io::prelude::*;

    let out_file = format!("{}/style.css", std::env::var("OUT_DIR").unwrap());
    let mut file = File::create(&out_file).unwrap();
    file.write_all(css.as_bytes()).unwrap();
}
