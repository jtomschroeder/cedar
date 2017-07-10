
extern crate crossbeam;

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
    pub use cacao::View;
    pub use cacao::Application;
}

// --- gtk ---

#[cfg(any(feature = "gtk3", not(target_os = "macos")))]
extern crate gtk;

#[cfg(any(feature = "gtk3", not(target_os = "macos")))]
#[path = "gtk/mod.rs"]
pub mod gtk3;

#[cfg(any(feature = "gtk3", not(target_os = "macos")))]
pub mod backend {
    pub use gtk3::View;
    pub use gtk3::Application;
}

// --- common ---

pub use backend::*;

mod property;
mod atomic_box;
mod stream;
mod update;
mod program;

pub use self::update::Update;
pub use self::program::Program;
