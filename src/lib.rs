#![deny(trivial_casts, trivial_numeric_casts)]
#![deny(unused_import_braces, unused_qualifications)]
#![deny(unsafe_code, unstable_features)]
// #![deny(missing_debug_implementations, missing_copy_implementations)]
// #![deny(missing_docs)]

mod boo;
mod sml;

#[macro_use]
mod tree;

mod application;
mod renderer;
mod shadow;

pub mod dom;

pub use crate::application::{app, Application};

pub mod prelude {
    pub use crate::sml;
    // pub use crate::sml::Component;
    pub use crate::sml::Component;
}

pub mod sass {
    pub fn compile(style: &str) -> String {
        sass_rs::compile_string(style, sass_rs::Options::default()).unwrap()
    }
}
