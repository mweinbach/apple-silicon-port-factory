//! Shared game-agnostic manifest schema (`game.toml`), multi-version middleware
//! catalog, canonical DLL mapping, and DLL routing policy for Apple Silicon Port
//! Factory.
//!
//! This crate intentionally depends only on `pe-image` (pure Rust, zero OS or
//! GPU dependencies) so it compiles cleanly on `aarch64-apple-darwin`,
//! `aarch64-apple-ios`, `aarch64-apple-ios-sim`, and `x86_64-apple-darwin`.

pub mod dll_map;
pub mod manifest;
pub mod middleware;
pub mod plugins;

pub use dll_map::{canonical_dll, DllRoute, DllRoutingTable, DEFAULT_BUILTIN_OVERRIDES};
pub use manifest::{GameManifest, MiddlewareSelection, StorageManifestConfig};
pub use middleware::{
    detect_middleware_from_dlls, family_all_dlls, find_by_dll, find_family, find_version,
    suggest_game_toml_middleware, DefaultDllMode, DetectedMiddleware, MiddlewareFamily,
    MiddlewareVersionSpec, MIDDLEWARE_CATALOG,
};
pub use plugins::{
    collect_x64_pe_files, discover_bundled_plugins, has_pe_extension, has_plugin_extension,
    is_valid_x64_pe_file, probe_x64_pe_file, PeDiscoveryOptions, PeHeaderProbe,
    PE_BINARY_EXTENSIONS, PLUGIN_LIBRARY_EXTENSIONS,
};
