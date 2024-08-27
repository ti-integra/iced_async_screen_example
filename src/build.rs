use std::env;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(build_release)");
    if env::var("PROFILE").unwrap() == "release" {
        println!("cargo:rustc-cfg=build_release");
    }
}
