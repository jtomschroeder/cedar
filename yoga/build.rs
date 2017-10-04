
extern crate gcc;

fn main() {
    gcc::Build::new()
        .flag("-std=c11")
        .file("yoga/YGEnums.c")
        .file("yoga/YGNodeList.c")
        .file("yoga/Yoga.c")
        .include("yoga")
        .compile("libyoga.a")
}
