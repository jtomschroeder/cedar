#![deny(trivial_casts, trivial_numeric_casts)]
#![deny(unused_import_braces, unused_qualifications)]
#![deny(unsafe_code, unstable_features)]
// #![deny(missing_debug_implementations, missing_copy_implementations)]
// #![deny(missing_docs)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
pub extern crate serde_json as json;

extern crate sass_rs as sass;

mod boo;
mod sml;

#[macro_use]
mod tree;

mod application;
mod renderer;
mod shadow;

pub mod dom;

pub use crate::application::{app, Application};

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
