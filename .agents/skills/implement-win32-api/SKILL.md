---
name: implement-win32-api
description: Implement or fix a Windows OS, CRT, COM, graphics, audio, or middleware export in crates/win32-rt (kernel32, ntdll, ucrt, user32, d3d12, d3d11, dxgi, vulkan-1, wasapi, xinput, hid, secur32, crypt32, steam_api64, etc.). Use when port-runner or aw2-loader reports "UNIMPLEMENTED dll!name", win32-audit flags missing imports, or a Windows API behaves incorrectly.
---

# Implement or Fix a Windows API (`crates/win32-rt`)

Follow `AGENTS.md` hard rules: keep `crates/` 100% game-agnostic, never import `objc2-metal` or
`aw2-data` inside `win32-rt`, never fake success on unimplemented behavior, and assume all guest
pointers are unaligned.

For deep ABI rules (MSVC reversed overload vtables, `sret`, `win64_bridge!`, dual-interface COM,
`plain_builtin_fns`/`pure_float_math_fns`, and the full catalog of already-implemented native
middleware runtimes), read [msvc_abi_and_com.md](./references/msvc_abi_and_com.md).

## Checklist

1. **Check existing coverage and reproduce the failure.**
   - Search existing exports and aliases first:
     ```sh
     cargo opsdb -- win32-stats --search <ExportName>
     ```
   - Check [msvc_abi_and_com.md](./references/msvc_abi_and_com.md) before writing a new middleware
     runtime — Bink2, DirectStorage, DirectML, Media Foundation, D3DCompiler (`D3DReflect`/DXC),
     DirectWrite/Direct2D, TSF (`msctf`), ECMA-335 CLR/Mono (`clr/`), FMOD Core+Studio, XAudio2
     + XAPO, DirectSound, OpenAL, NvCloth 1.1.6, NRI/NRD, XeSS/DLSS/FFX/AGS, WinHTTP/libcurl,
     Schannel/Crypt32/BCrypt/WinVerifyTrust, and GameInput already exist natively.
   - Reproduce with `port-runner` (or `cargo loader` for Alan Wake 2), adding `--trace` or
     `--no-run`. Audit a game install up front with `cargo opsdb -- win32-audit /path/to/game/`.
   - To scaffold a brand-new `misc/<stem>.rs` module from a PE's exports or EXE's imports:
     ```sh
     cargo opsdb -- scaffold-dll <dll_name> /path/to/dll_or_exe [--write]
     ```

2. **Verify the exact Windows x64 ABI and contract.**
   - Verify against `third_party/directx-headers`, Microsoft docs, or the retail PE vtable /
     disassembly (never guess C++ vtable slot order — MSVC **reverses** overloaded virtual methods,
     and some WinMD/MF interfaces sort alphabetically!).
   - Use `pub unsafe extern "win64" fn Name(...) -> ...` (lowers to AAPCS64 `extern "C"` inside
     `win64_abi::win64_mods!`).
   - Exact Win64 types: `BOOL = i32`, `DWORD = u32`, `LONG = i32`, `WCHAR = u16`, `HANDLE = *mut c_void`.
     Always access guest memory with `ptr::read_unaligned` / `ptr::write_unaligned` or
     `#[repr(C, packed)]` + compile-time `size_of` assertions.
   - Set last error via `crate::teb::set_last_error(...)` on failure when documented.
   - **Mixed int/float ABI on ARM64:** Wrap ANY Win64 function mixing integer and float by-value
     args (`void Foo(this, f32)` passes `f32` in `xmm1`, not `s0`!) with `win64_bridge!` +
     `bridge_entry!`, or dummy `#[cfg(target_arch = "aarch64")]` GPR slots for `pure_float_math_fns`.
   - **Hidden `sret` returns:** Structs not of size 1/2/4/8 bytes pass hidden `sret` in `rdx`
     (member) or `rcx` (free) and **must return `sret` in `rax`**.
   - **Guest callbacks & lock safety:** Always invoke guest callbacks through `crate::call_guest8(...)`
     (installs `EhBarrier`) and **drop internal `Mutex`/`RwLock` guards before calling `call_guest8`**
     so re-entrant calls never deadlock.
   - **Dynamic native function pointers:** Register every COM vtable slot and dynamically returned
     host function pointer via `crate::register_native_va` (or `d3d12::com::build_vtable`).

3. **Implement real Apple-platform behavior (or an honest failing stub).**
   - Back OS APIs with real macOS/iOS primitives (`src/macos/` aliased as `crate::platform`,
     `src/audio/`, `d3d12-metal` safe Rust wrappers — never direct `objc2-metal`).
   - Gate macOS-only frameworks (`AppKit`, `IOKit`/`IOHIDManager`, CoreAudio HAL) behind
     `#[cfg(target_os = "macos")]` so `aarch64-apple-ios` stays green.
   - For D3D12 COM interfaces, regenerate `src/d3d12/vtables.rs` via
     `python3 crates/win32-rt/src/d3d12/gen_vtables.py > crates/win32-rt/src/d3d12/vtables.rs`.
   - For physics/cloth/math boundaries, call `crate::report_boundary_nan(...)` on non-finite floats
     so `PORT_NAN_BOUNDARY=1` catches guest NaNs at zero steady cost.
   - If a hot CRT/math/RTTI leaf function never invokes `call_guest8`, consider adding it to
     `plain_builtin_fns()` or `pure_float_math_fns()` in `crates/win32-rt/src/lib.rs` (kept
     strictly disjoint; verified by `plain_and_pure_math_tables_cover_crt_exports`).
   - If a stub is unavoidable, log via `stub!` / `failing!` / `win64_stubs!` and return the
     documented failure code (`E_NOTIMPL`, `ERROR_CALL_NOT_IMPLEMENTED`, `FALSE`). Never stub
     shipping compression DLLs (`oo2core_*`, `zstd`, `lz4`) with fake `memcpy`s.

4. **Check `build.rs` export rules & run the module guardrail script.**
   - `crates/win32-rt/build.rs` scans **column-0** (unindented) `pub [unsafe] extern "win64" fn`,
     `context_capturing_export!`, `win64_varargs_thunk!`, `failing!(...)`, `win64_stubs!(...)`, and
     `// @exports ...`, and **stops at the FIRST `#[cfg(test)]` token**. Keep a single
     `#[cfg(test)]` module strictly at the very bottom!
   - Every `src/misc/<stem>.rs` needs `EXTRA_EXPORTS`, `ORDINALS`, and `pub mod <stem>;` in
     `src/misc/mod.rs`. Avoid `static mut` and Rust 2024 `dangerous_implicit_autorefs`.
   - Run the bundled static guardrail script ([audit_win32_module.py](./scripts/audit_win32_module.py)):
     ```sh
     python3 .agents/skills/implement-win32-api/scripts/audit_win32_module.py crates/win32-rt/src/misc/<file>.rs
     ```

5. **Test, sync catalogs, and cross-check targets.**
   - Add unit tests covering normal, error, null, and unaligned paths.
   - Run:
     ```sh
     cargo opsdb -- win32-sync
     cargo opsdb -- doctor
     cargo test -p win32-rt --lib --target aarch64-apple-darwin -- <module> --test-threads=1
     cargo check -p win32-rt --lib --target aarch64-apple-ios
     cargo runner-build
     ```
   - If you changed a public signature, also check `cargo check -p aw2-game --no-default-features --target aarch64-apple-darwin`
     and `cargo check -p win32-rt --lib --target x86_64-apple-darwin`.

6. **Commit cleanly.**
   - Commit Rust code, tests, and updated `db/win32_apis.tsv` together:
     `win32: implement <dll>!<Name>` (or `d3d11:` / `d3d12:` / `vulkan:` / `audio:`).
