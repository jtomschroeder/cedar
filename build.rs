
extern crate cc;

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    if !target.contains("apple") || !target.contains("x86_64") {
        panic!("Only macOS supported! (currently)");
    }

    cc::Build::new()
        .cpp(true)
        .flag("-std=c++14")
        .file("lib/cocoa/app.mm")
        .compile("libcedar-cocoa.a");

    println!("cargo:rustc-link-lib=framework=Cocoa");
    println!("cargo:rustc-link-lib=framework=WebKit");
}
