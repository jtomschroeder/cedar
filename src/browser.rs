#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod ffi {
    extern "C" {
        pub fn log(s: *const u8, len: u32);
        pub fn command(s: *const u8, len: u32);
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod ffi {
    pub unsafe fn log(_: *const u8, _: u32) {}
    pub unsafe fn command(_: *const u8, _: u32) {}
}

pub fn log(msg: &str) {
    unsafe { ffi::log(msg.as_ptr(), msg.as_bytes().len() as u32) };
}

pub fn command(msg: &str) {
    unsafe { ffi::command(msg.as_ptr(), msg.as_bytes().len() as u32) };
}
