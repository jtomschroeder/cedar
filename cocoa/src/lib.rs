
mod bindings {
    extern "C" {
        pub fn run();
    }
}

pub fn run() {
    unsafe { bindings::run() }
}
