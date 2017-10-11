
extern crate cc;

fn main() {
    cc::Build::new()
        .flag("-std=c11")
        .file("yoga/lib/YGEnums.c")
        .file("yoga/lib/YGNodeList.c")
        .file("yoga/lib/Yoga.c")
        .include("yoga/lib")
        .compile("libyoga.a");

    cc::Build::new()
        .cpp(true)
        .flag("-std=c++14")
        .file("cocoa/lib/cocoa.mm")
        .include("cocoa/ext")
        .compile("libcedar-cocoa.a");

    println!("cargo:rustc-link-lib=framework=Cocoa");
}
