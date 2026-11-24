//! Generalized `x86-64 -> aarch64` operation translation database, Wine-style
//! core Windows API mapping database, PE32+ binary analyzer, static AOT
//! (`x86-64 PE -> Native ARM64 Rust`) compiler, scaffolding generator, and
//! Windows Virtual Filesystem (`vfs`).
//!
//! This crate decouples the `x86-64 -> ARM64` translation catalog, Win32 API
//! mapping database, static AOT compiler, and Windows path/mount router from
//! any single game so we can:
//! 1. Maintain a structured, queryable database (`db/ops_x86_to_arm64.tsv`) of
//!    every `x86-64` instruction family, `FastOp` kind (`K_1..=K_124`), operand
//!    form, width, `ARM64` instruction emission pattern, flag/vector semantics,
//!    and verification vector count.
//! 2. Maintain a Wine-style core Windows API mapping database (`db/win32_apis.tsv`)
//!    covering `kernel32.dll`, `ntdll.dll`, `ucrtbase.dll`, `msvcrt.dll`,
//!    `user32.dll`, `advapi32.dll`, `ws2_32.dll`, `ole32.dll`, `api-ms-win-*`
//!    sets, and more, mapping each Win32 symbol to its Rust implementation.
//! 3. Statically AOT-compile any Windows `x86-64` `.exe` or `.dll` into native
//!    `aarch64` Rust functions (`aot.rs`), or package it into a self-contained,
//!    zero-dependency embeddable Rust crate (`scaffold::generate_embeddable_aot_crate`).
//! 4. Scaffold new `x86-64 -> ARM64` ops (`scaffold::generate_op_scaffold`) and
//!    new Windows/middleware DLL builtins (`scaffold::generate_dll_scaffold`).
//! 5. Route Windows drives, special folders (`%APPDATA%`, `%LOCALAPPDATA%`,
//!    `%TEMP%`), and custom `WIN=HOST` mounts (`vfs::WinFs`).

pub mod anticheat;
pub mod aot;
pub mod bundle_common;
pub mod diag_toolkit;
pub mod doctor;
pub mod dotnet;
pub mod game_profiles;
pub mod gpu_shaders;
pub mod ios_bundle;
pub mod json_output;
pub mod launcher;
pub mod ops_db;
pub mod scaffold;
pub mod scanner;
pub mod steam_library;
pub mod storage_opt;
pub mod vfs;
pub mod win32_db;
