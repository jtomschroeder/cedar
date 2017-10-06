
extern crate cc;

fn main() {
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++14")
        .file("lib/cocoa.mm")
        .include("ext")
        .compile("libcedar-cocoa.a");

    println!("cargo:rustc-link-lib=framework=Cocoa");
}
