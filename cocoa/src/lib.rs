
// #[link(name = "objc", kind = "dylib")]
// #[link(name = "Foundation", kind = "framework")]
// #[link(name = "AppKit", kind = "framework")]
// #[link(name = "Cocoa", kind = "framework")]
mod bindings {
    extern "C" {
        pub fn run();
    }
}

pub fn run() {
    // println!("RUN!");
    unsafe { bindings::run() }
}
