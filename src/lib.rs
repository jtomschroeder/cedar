
extern crate crossbeam;

#[macro_use]
extern crate tree;

// --- macOS ---

#[macro_use]
#[cfg(target_os = "macos")]
extern crate objc;
#[cfg(target_os = "macos")]
extern crate cocoa;
#[cfg(target_os = "macos")]
extern crate core_graphics;

#[cfg(all(target_os = "macos", not(feature = "gtk3")))]
#[path = "cocoa/mod.rs"]
mod cacao;

#[cfg(all(target_os = "macos", not(feature = "gtk3")))]
pub mod backend {
    pub use cacao::*;

    // pub use cacao::View;
    // pub use cacao::Program;
}

// --- gtk ---

#[cfg(any(feature = "gtk3", not(target_os = "macos")))]
extern crate gtk;

#[cfg(any(feature = "gtk3", not(target_os = "macos")))]
#[path = "gtk/mod.rs"]
pub mod gtk3;

#[cfg(any(feature = "gtk3", not(target_os = "macos")))]
pub mod backend {
    pub use gtk3::*;

    // pub use gtk3::View;
    // pub use gtk3::Program;
}

// --- common ---

pub use backend::*;

pub mod dom;

// TODO: remove property!

mod property;
mod atomic_box;
mod stream;
