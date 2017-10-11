
extern crate cc;

fn main() {
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++14")
        .file("cocoa/lib/cocoa.mm")
        .include("cocoa/ext")
        .compile("libcedar-cocoa.a");

    println!("cargo:rustc-link-lib=framework=Cocoa");
}
