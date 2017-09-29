
extern crate gcc;

fn main() {
    gcc::Build::new()
        .cpp(true)
        .flag("-std=c++14")
        // .flag("-framework Cocoa")
        .file("lib/cocoa.mm")
        // .include("??")
        .compile("libcedar-cocoa.a");

    println!("cargo:rustc-link-lib=framework=Cocoa");
    // println!("cargo:rustc-link-lib=framework=CoreFoundation");
}
