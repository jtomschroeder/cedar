
extern crate crossbeam;

use std::os::raw::{c_void, c_char};
use std::ffi::{CStr, CString};
use std::sync::Arc;

use crossbeam::sync::MsQueue;

mod bindings {
    use super::*;
    extern "C" {
        pub fn run(renderer: *mut c_void);
    }
}

pub struct Renderer {
    pub incoming: Arc<MsQueue<String>>,
    pub outgoing: Arc<MsQueue<String>>,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            incoming: Arc::new(MsQueue::new()),
            outgoing: Arc::new(MsQueue::new()),
        }
    }

    // fn send(&self) {
    //     eprintln!("HEY!");
    // }
}

// TODO: handling dropping of interconnect instance

#[no_mangle]
pub extern "C" fn renderer_send(renderer: *mut Renderer, s: *const c_char) {
    let renderer: &Renderer = unsafe { &*renderer };

    let s = unsafe { CStr::from_ptr(s) };
    let s = s.to_str().unwrap();

    renderer.outgoing.push(s.into());
}

#[no_mangle]
pub extern "C" fn renderer_recv(renderer: *mut Renderer) -> *mut c_char {
    let renderer: &Renderer = unsafe { &*renderer };

    let input = renderer.incoming.pop(); // blocking!

    let string = CString::new(input.into_bytes()).unwrap();
    CString::into_raw(string)
}

#[no_mangle]
pub extern "C" fn renderer_string_drop(s: *mut c_char) {
    let _ = unsafe { CString::from_raw(s) };
}

pub fn run(interconnect: Renderer) {
    let interconnect = Box::new(interconnect);
    unsafe { bindings::run(Box::into_raw(interconnect) as *mut c_void) }
}
