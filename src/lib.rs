#![feature(proc_macro)]
// #![deny(missing_docs)]
// #![deny(unsafe_code, unstable_features)]
#![deny(trivial_casts, trivial_numeric_casts)]
#![deny(
    missing_debug_implementations, missing_copy_implementations, unused_import_braces,
    unused_qualifications
)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
pub extern crate serde_json as json;

extern crate sass_rs as sass;
extern crate web_view;

extern crate hypertext;

// TODO: hyperlink generate rust at build-time via build.rs (a la LALRPOP)
// - (until proc-macro bug is fixed)

mod boo;

#[macro_use]
mod tree;

mod program;
mod renderer;
mod shadow;

pub mod dom;

pub use hypertext::hypertext;
pub use program::program;
