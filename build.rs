
extern crate cc;

use std::env;

fn main() {
    cc::Build::new()
        .flag("-std=c11")
        .file("yoga/lib/YGEnums.c")
        .file("yoga/lib/YGNodeList.c")
        .file("yoga/lib/Yoga.c")
        .include("yoga/lib")
        .compile("libyoga.a");

    let target = env::var("TARGET").unwrap();
    let gtk = env::var("CARGO_FEATURE_GTK").is_ok();

    if target.contains("apple") && !gtk {
        cc::Build::new()
            .cpp(true)
            .flag("-std=c++14")
            .file("cocoa/lib/cocoa.mm")
            .include("cocoa/ext")
            .compile("libcedar-cocoa.a");

        println!("cargo:rustc-link-lib=framework=Cocoa");
    }
}
