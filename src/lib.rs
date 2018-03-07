#![feature(proc_macro)]
// #![deny(missing_docs)]
// #![deny(unsafe_code, unstable_features)]
// #![deny(trivial_casts, trivial_numeric_casts)]
#![deny(missing_debug_implementations, missing_copy_implementations, unused_import_braces,
        unused_qualifications)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate hypertext;

// TODO: hyperlink generate rust at build-time via build.rs (a la LALRPOP)
// - (until proc-macro bug is fixed)

mod boo;

#[macro_use]
mod tree;
mod shadow;
mod renderer;
mod program;
mod processor;

pub mod dom;

pub use hypertext::hypertext;
pub use program::{program, programv, Subscription};
pub use processor::process;

pub mod browser;
pub mod memory;
