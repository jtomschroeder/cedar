
extern crate cc;
extern crate cmake;

use std::env;
use std::path::Path;

fn main() {
    let home = env::var("HOME").unwrap();
    let target = env::var("TARGET").unwrap();

    if !target.contains("apple") || !target.contains("x86_64") {
        panic!("Only macOS supported! (currently)");
    }

    if !Path::new(&format!("{}/.cedar", home)).exists() {
        panic!("cedar is not setup! Please run `cedar setup`");
    }

    cc::Build::new()
        .cpp(true)
        .flag("-std=c++14")
        .flag("-isystemlib/cef")
        .file("lib/app/cefsimple_mac.mm")
        .file("lib/app/simple_app.cc")
        .file("lib/app/simple_handler_mac.mm")
        .file("lib/app/simple_handler.cc")
        .file("lib/app/process_helper.cc")
        .compile("libcedar-cef.a");

    println!("cargo:rustc-link-lib=framework=Cocoa");

    println!("cargo:rustc-link-search=framework={}/.cedar/lib", home);
    println!("cargo:rustc-link-lib=framework=Chromium Embedded Framework");

    let cef = cmake::Config::new("lib/cef")
        .build_target("libcef_dll_wrapper")
        .build();

    println!(
        "cargo:rustc-link-search=native={}/build/libcef_dll_wrapper",
        cef.display()
    );
    println!("cargo:rustc-link-lib=static=cef_dll_wrapper");
}
