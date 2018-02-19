use std::ptr;
use std::env;

use std::ffi::CString;

extern "C" {
    fn cef_helper_run(argc: i32, argv: *const *const i8) -> i32;
}

pub fn helper() {
    let args: Vec<_> = env::args().map(|arg| CString::new(arg).unwrap()).collect();
    let mut args: Vec<_> = args.iter().map(|arg| arg.as_ptr()).collect();

    let len = args.len() as i32;
    args.push(ptr::null()); // `argv` is null-terminated

    unsafe {
        cef_helper_run(len, args.as_ptr());
    }
}
