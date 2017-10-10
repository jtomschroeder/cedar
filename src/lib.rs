
extern crate crossbeam;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate yoga;
extern crate cocoa;

#[macro_use]
mod tree;
mod phantom;
mod program;

pub mod dom;
pub use program::program;

// TODO: handle window 'resize' event
// TODO: implement 'field' rendering
// TODO: implement 'change' event
