extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
mod tree;
mod phantom;
mod program;
mod renderer;

pub mod dom;
pub mod facade;

pub use program::{program, process};

pub mod browser {
    mod ffi {
        extern "C" {
            pub fn alert(s: *const u8, len: u32);
            pub fn log(s: *const u8, len: u32);

            // Specific to cedar!
            pub fn command(s: *const u8, len: u32);
        }
    }

    pub fn log(msg: &str) {
        unsafe { ffi::log(msg.as_ptr(), msg.as_bytes().len() as u32) };
    }

    pub fn command(msg: &str) {
        unsafe { ffi::command(msg.as_ptr(), msg.as_bytes().len() as u32) };
    }
}
