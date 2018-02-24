#![feature(proc_macro)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate hypertext;

mod boo;

#[macro_use]
mod tree;
mod phantom;
mod program;
mod renderer;

pub mod dom;

pub use hypertext::hypertext;
pub use program::{process, program};

pub mod browser;
pub mod memory;
