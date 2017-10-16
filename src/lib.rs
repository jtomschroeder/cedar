
extern crate crossbeam;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

// macOS

#[cfg(all(target_os = "macos", not(feature = "gtk")))]
#[path = "cocoa/mod.rs"]
pub mod facade;

// GTK

#[cfg(any(feature = "gtk", not(target_os = "macos")))]
#[path = "gtk/mod.rs"]
mod facade;

// ---

#[macro_use]
mod tree;
mod phantom;
mod program;
mod renderer;

// mod yoga;

pub mod dom;

pub use program::program;
