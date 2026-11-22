//! The `win64_abi::win64_mods!` proc macro mirrors `src/` into
//! `target/win64-abi-aarch64/aw2-game/src/` for native builds, but it does not
//! register the (huge) generated shard files in rustc's dep-info. Without this
//! build script, a build where only files under `src/shards/` changed was a
//! silent no-op. Cargo scans a `rerun-if-changed` directory recursively, so any
//! shard edit reruns this script, which recompiles the crate and resyncs the mirror.

fn main() {
    println!("cargo:rerun-if-changed=src/shards");
    println!("cargo:rerun-if-changed=build.rs");
}
