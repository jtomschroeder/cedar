
extern crate cc;
extern crate cmake;

use std::env;

fn main() {
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++14")
        .flag("-isystemlib/cef")
        .file("lib/app/cefsimple_mac.mm")
        .file("lib/app/simple_app.cc")
        .file("lib/app/simple_handler_mac.mm")
        .file("lib/app/simple_handler.cc")
        .compile("libcedar-cef.a");

    cc::Build::new()
        .cpp(true)
        .flag("-std=c++14")
        .flag("-isystemlib/cef")
        .file("lib/app/process_helper.cc")
        .compile("libcedar-cef-helper.a");

    println!("cargo:rustc-link-lib=framework=Cocoa");

    println!("cargo:rustc-link-search=framework=lib/cef/Release");
    println!("cargo:rustc-link-lib=framework=Chromium Embedded Framework");

    let dst = cmake::Config::new("lib/cef")
        .generator("Ninja")
        .build_target("libcef_dll_wrapper")
        .build();

    println!(
        "cargo:rustc-link-search=native={}/build/libcef_dll_wrapper",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=cef_dll_wrapper");
}
