extern crate cc;
extern crate pkg_config;

use std::env;

fn main() {
    let mut build = cc::Build::new();

    build
        .include("webview.h")
        .flag_if_supported("-std=c11")
        .flag_if_supported("-w");

    if env::var("DEBUG").is_err() {
        build.define("NDEBUG", None);
    } else {
        build.define("DEBUG", None);
    }

    build
        .file("webview.c")
        .define("OBJC_OLD_DISPATCH_PROTOTYPES", "1")
        .flag("-x")
        .flag("objective-c");
    println!("cargo:rustc-link-lib=framework=Cocoa");
    println!("cargo:rustc-link-lib=framework=WebKit");

    build.compile("webview");
}
