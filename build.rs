#[cfg(feature = "build-native-harfbuzz")]
extern crate cc;
#[cfg(feature = "build-native-harfbuzz")]
extern crate pkg_config;

#[cfg(feature = "build-native-harfbuzz")]
fn main() {
    use std::env;

    let target = env::var("TARGET").unwrap();

    println!("cargo:rerun-if-env-changed=HARFBUZZ_SYS_NO_PKG_CONFIG");

    let mut cfg = cc::Build::new();
    cfg.cpp(true)
        .flag("-std=c++11")
        .warnings(false)
        .include("harfbuzz/src")
        .include("freetype-sys/freetype2/include/")
        .file("harfbuzz/src/harfbuzz.cc");

    cfg.define("HAVE_FREETYPE", "1");

    if !target.contains("windows") {
        cfg.define("HAVE_PTHREAD", "1");
    }

    if target.contains("apple") {
        cfg.define("HAVE_CORETEXT", "1");
    }

    if target.contains("windows-gnu") {
        cfg.flag("-Wa,-mbig-obj");
    }

    cfg.compile("embedded_harfbuzz");

    println!("cargo::rustc-link-lib=static=embedded_harfbuzz");
}

#[cfg(not(feature = "build-native-harfbuzz"))]
fn main() {}
