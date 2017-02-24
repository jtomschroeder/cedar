
#[macro_use]
#[cfg(target_os = "macos")]
extern crate objc;
#[cfg(target_os = "macos")]
extern crate cocoa;

extern crate crossbeam;

#[cfg(target_os = "macos")]
#[path = "cocoa/mod.rs"]
mod backend;

mod property;
mod atomic_box;
mod stream;
mod view;
mod application;

pub use view::View;
pub use application::Application;
