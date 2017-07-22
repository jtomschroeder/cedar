
extern crate crossbeam;

// --- macOS ---

#[cfg(all(target_os = "macos", not(feature = "gtk3")))]
#[macro_use]
extern crate objc;
#[cfg(all(target_os = "macos", not(feature = "gtk3")))]
extern crate cocoa;
#[cfg(all(target_os = "macos", not(feature = "gtk3")))]
extern crate core_graphics;

#[cfg(all(target_os = "macos", not(feature = "gtk3")))]
#[path = "cocoa/mod.rs"]
mod cacao;

#[cfg(all(target_os = "macos", not(feature = "gtk3")))]
pub use cacao::program;

// --- gtk ---

#[cfg(any(feature = "gtk3", not(target_os = "macos")))]
extern crate gtk;

#[cfg(any(feature = "gtk3", not(target_os = "macos")))]
#[path = "gtk/mod.rs"]
mod gtk3;

#[cfg(any(feature = "gtk3", not(target_os = "macos")))]
pub use gtk3::program;

// --- common ---

#[macro_use]
mod tree;
mod atomic_box;
mod stream;

pub mod dom;
