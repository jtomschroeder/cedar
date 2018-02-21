extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod boo;

#[macro_use]
mod tree;
mod phantom;
mod program;
mod renderer;

pub mod dom;

pub use program::{process, program};

pub mod browser {
    mod ffi {
        extern "C" {
            pub fn log(s: *const u8, len: u32);
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

pub mod memory {
    use std::mem;
    use std::ffi::CString;
    use std::os::raw::{c_char, c_void};

    #[no_mangle]
    pub extern "C" fn alloc(size: usize) -> *mut c_void {
        let mut buf = Vec::with_capacity(size);
        let ptr = buf.as_mut_ptr();
        mem::forget(buf);
        ptr as *mut c_void
    }

    #[no_mangle]
    pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
        let _ = unsafe { Vec::from_raw_parts(ptr, 0, cap) };
    }

    #[no_mangle]
    pub extern "C" fn dealloc_str(ptr: *mut c_char) {
        let _ = unsafe { CString::from_raw(ptr) };
    }
}
