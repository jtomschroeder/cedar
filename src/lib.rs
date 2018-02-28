#![feature(proc_macro)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate hypertext;

mod boo;

#[macro_use]
mod tree;
mod shadow;
mod renderer;
mod program;
mod processor;

pub mod dom;

pub use hypertext::hypertext;
pub use program::program;
pub use processor::process;

pub mod browser;
pub mod memory;
