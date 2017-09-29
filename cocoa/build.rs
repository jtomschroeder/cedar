
extern crate gcc;

fn main() {
    gcc::Build::new()
        .cpp(true)
        .flag("-std=c++14")
        .file("lib/cocoa.mm")
        // .include("??")
        .compile("libcedar-cocoa.a")
}
