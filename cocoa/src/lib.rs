
extern crate crossbeam;

use std::io;
use std::os::raw::{c_void, c_char};
use std::ffi::{CStr, CString};
use std::sync::Arc;

use crossbeam::sync::MsQueue;

mod bindings {
    use super::*;
    extern "C" {
        pub fn run(ic: *mut c_void);
    }
}

pub struct Interconnect {
    pub incoming: Arc<MsQueue<String>>,
    pub outgoing: Arc<MsQueue<String>>,
}

impl Interconnect {
    pub fn new() -> Self {
        Interconnect {
            incoming: Arc::new(MsQueue::new()),
            outgoing: Arc::new(MsQueue::new()),
        }
    }

    // fn send(&self) {
    //     eprintln!("HEY!");
    // }
}

#[no_mangle]
pub extern "C" fn ic_send(ic: *mut Interconnect, s: *const c_char) {
    let ic: &Interconnect = unsafe { &*ic };

    let s = unsafe { CStr::from_ptr(s) };
    let s = s.to_str().unwrap();

    ic.outgoing.push(s.into());
}

#[no_mangle]
pub extern "C" fn ic_recv(ic: *mut Interconnect) -> *mut c_char {
    let ic: &Interconnect = unsafe { &*ic };

    let mut input = ic.incoming.pop();

    let string = CString::new(input.into_bytes()).unwrap();
    CString::into_raw(string)
}

#[no_mangle]
pub extern "C" fn ic_string_drop(s: *mut c_char) {
    let _ = unsafe { CString::from_raw(s) };
}

pub fn run(interconnect: Interconnect) {
    let interconnect = Box::new(interconnect);
    unsafe { bindings::run(Box::into_raw(interconnect) as *mut c_void) }
}
