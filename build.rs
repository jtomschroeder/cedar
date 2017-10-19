
extern crate cc;

use std::env;

fn main() {
    cc::Build::new()
        .flag("-std=c11")
        .file("lib/yoga/YGEnums.c")
        .file("lib/yoga/YGNodeList.c")
        .file("lib/yoga/Yoga.c")
        .compile("libyoga.a");

    let target = env::var("TARGET").unwrap();
    let gtk = env::var("CARGO_FEATURE_GTK").is_ok();

    if target.contains("apple") && !gtk {
        let out_dir = env::var("OUT_DIR").unwrap();

        cc::Build::new()
            .cpp(true)
            .flag("-std=c++14")

            // Yoga
            .include("lib")
            .object(format!("{}/libyoga.a", out_dir))

            .file("lib/cocoa/cocoa.mm")
            .compile("libcedar-cocoa.a");

        println!("cargo:rustc-link-lib=framework=Cocoa");
    }
}
