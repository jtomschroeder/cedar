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
