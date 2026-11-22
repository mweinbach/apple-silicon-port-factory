//! Link the loader with a tiny `__PAGEZERO` so the low 4 GB (0x7ffe0000 for
//! KUSER_SHARED_DATA, 0x140000000 for the exe, 0x180000000 for DLLs) are free
//! for Windows images. Only for the x86_64 macOS (Rosetta) build.
//!
//! Also embed `Info.plist` (`__TEXT,__info_plist`) so the unbundled binary
//! has a bundle identity for AppKit (Retina, Game Mode category, app name).
fn main() {
    let target = std::env::var("TARGET").unwrap_or_default();
    if target == "x86_64-apple-darwin" {
        println!("cargo:rustc-link-arg-bins=-Wl,-pagezero_size,0x10000");
    }
    if target.ends_with("apple-darwin") {
        let plist = std::path::Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("Info.plist");
        println!("cargo:rustc-link-arg-bins=-Wl,-sectcreate,__TEXT,__info_plist,{}", plist.display());
        println!("cargo:rerun-if-changed={}", plist.display());
    }
    println!("cargo:rerun-if-changed=build.rs");
}
