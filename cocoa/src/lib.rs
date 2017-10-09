
use std::io;
use std::os::raw::c_char;
use std::ffi::{CStr, CString};

mod bindings {
    use super::*;

    extern "C" {
        pub fn run(ic: *mut Interconnect);
    }
}

pub struct Interconnect;

// impl Interconnect {
//     fn send(&self) {
//         eprintln!("HEY!");
//     }
// }

#[no_mangle]
pub extern "C" fn ic_send(ic: *mut Interconnect, s: *const c_char) {
    let ic: &Interconnect = unsafe { &*ic };

    let s = unsafe { CStr::from_ptr(s) };
    let s = s.to_str().unwrap();

    println!("{}", s);
}

#[no_mangle]
pub extern "C" fn ic_recv(ic: *mut Interconnect) -> *mut c_char {
    let ic: &Interconnect = unsafe { &*ic };

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let string = CString::new(input.into_bytes()).unwrap();
    CString::into_raw(string)
}

#[no_mangle]
pub extern "C" fn ic_string_drop(s: *mut c_char) {
    let _ = unsafe { CString::from_raw(s) };
}

pub fn run() {
    let interconnect = Box::new(Interconnect);
    unsafe { bindings::run(Box::into_raw(interconnect)) }
}
