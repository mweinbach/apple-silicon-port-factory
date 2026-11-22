---
name: implement-win32-api
description: Implement or fix a Windows OS, CRT, COM, graphics, audio, or middleware export in crates/win32-rt (kernel32, ntdll, ucrt, user32, d3d12, d3d11, dxgi, vulkan-1, wasapi, xinput, hid, secur32, crypt32, steam_api64, etc.). Use when port-runner or aw2-loader reports "UNIMPLEMENTED dll!name", win32-audit flags missing imports, or a Windows API behaves incorrectly.
---

# Implement or Fix a Windows API (`crates/win32-rt`)

Follow `AGENTS.md` hard rules: keep `crates/` 100% game-agnostic, never import `objc2-metal` or
`aw2-data` inside `win32-rt`, never fake success on unimplemented behavior, and assume all guest
pointers are unaligned.

## Checklist

1. **Check existing coverage and reproduce the failure.**
   - Search existing exports and aliases first:
     ```sh
     cargo opsdb -- win32-stats --search <ExportName>
     ```
   - Reproduce with `port-runner` (or `cargo loader` for Alan Wake 2), adding `--trace` for
     surrounding import calls or `--no-run` to list all unresolved imports at load time.
   - Audit a whole game directory up front with `cargo opsdb -- win32-audit /path/to/game/`.
   - To create a brand-new `misc/<stem>.rs` DLL module from a PE's exports or an EXE's imports:
     ```sh
     cargo opsdb -- scaffold-dll <dll_name> /path/to/dll_or_exe [--write]
     ```

2. **Verify the exact Windows x64 ABI and contract.**
   - Check `third_party/directx-headers` (for D3D/DXGI/COM) or Microsoft's Win32 documentation.
   - Use `pub unsafe extern "win64" fn Name(...) -> ...` (inside `win64_abi::win64_mods!`, this
     lowers to AAPCS64 `extern "C"` on `aarch64`).
   - Exact Win64 types: `BOOL = i32`, `DWORD = u32`, `LONG = i32`, `WCHAR = u16` (UTF-16),
     `HANDLE = *mut c_void` (or `usize`). Always read/write guest structs via `ptr::read_unaligned`
     / `ptr::write_unaligned` or `#[repr(C, packed)]`.
   - Set last error in the TEB (`crate::teb::set_last_error(...)`) on failure when documented.
   - **Mixed int/float ABI on ARM64:** If a dynamically bridged Win64 function takes mixed integer
     and float by-value arguments, wrap it with `win64_bridge!` + `bridge_entry!` (see
     `crates/win32-rt/src/macros.rs` and `src/bridge.rs`).
   - **Calling back into guest code:** Always invoke guest function pointers (`WNDPROC`,
     `CreateThread`, `qsort`, callbacks) through `crate::call_guest8(...)` so the call is wrapped
     in an `EhBarrier` (preventing C++ exceptions from unwinding across Rust frames).
   - **Dynamic native function pointers:** Any COM method or runtime-returned function pointer that
     guest x86 code can call directly on ARM64 must be registered via `crate::register_native_va`.

3. **Implement real Apple-platform behavior (or an honest failing stub).**
   - Back OS APIs with real macOS/iOS primitives: POSIX/Mach, AppKit/UIKit via `src/macos/`
     (aliased as `crate::platform`), CoreAudio/RemoteIO via `src/audio/`, Metal 4 via
     `d3d12-metal` (never direct `objc2-metal`).
   - Remember `win32-rt` also cross-compiles to `aarch64-apple-ios` / `aarch64-apple-ios-sim`.
     Gate macOS-only frameworks (`AppKit`, `IOKit`/`IOHIDManager`, CoreAudio HAL) behind
     `#[cfg(target_os = "macos")]`.
   - For COM interfaces:
     - D3D12/DXGI: use `src/d3d12/com.rs` (`Obj<T>`, `ComClass`, `build_vtable`) and regenerate
       `src/d3d12/vtables.rs` via `python3 crates/win32-rt/src/d3d12/gen_vtables.py > crates/win32-rt/src/d3d12/vtables.rs`
       when adding D3D12 interfaces.
     - D3D11: use `src/d3d11/com.rs` (`HasDeviceCore`, `child_slots`).
     - Audio/generic COM: use `src/audio/com.rs` (`com_iunknown`).
   - For physics/cloth/math middleware boundaries, call `crate::report_boundary_nan(...)` on
     non-finite float inputs so `PORT_NAN_BOUNDARY=1` catches guest NaN propagation at zero steady
     cost.
   - If a stub is genuinely unavoidable, log via `stub!(...)` / `failing!(...)` / `win64_stubs!(...)`
     and return the documented Windows failure code (`E_NOTIMPL`, `ERROR_CALL_NOT_IMPLEMENTED`,
     `FALSE`, etc.). Never silently return `S_OK` or `memcpy` past an unimplemented feature.
   - Never stub out shipping compression DLLs (`oo2core_*`, `zstd`, `lz4`) with fake passthroughs —
     let the real PE DLL run on `x86-runtime` (via negative `[runtime].builtin_overrides` if needed)
     unless you have a real, tested native implementation.

4. **Check `build.rs` export discovery and registration.**
   - `crates/win32-rt/build.rs` auto-scans each DLL's `.rs` files for **column-0** (unindented):
     - `pub [unsafe] extern "win64" fn <Name>`
     - `context_capturing_export!(<Name>, ...)` and `win64_varargs_thunk!(<Name>, ...)`
     - Top-level `failing!(...)` and `win64_stubs!(...)` blocks
     - Explicit `// @exports a b c` comments for macro-generated functions
   - **Critical gotcha:** `build.rs` splits every file at the **first `#[cfg(test)]` token** and
     ignores everything after it. Always keep `#[cfg(test)]` modules at the very bottom of the file.
   - Names starting with `dispatch_` or `aw2_` are skipped automatically.
   - Every `src/misc/<stem>.rs` file must define `pub const EXTRA_EXPORTS: &[Export]` (for
     C++-mangled/data exports) and `pub const ORDINALS: &[(u16, &str)]`, and be declared as
     `pub mod <stem>;` in `src/misc/mod.rs`.
   - API-set DLLs (`api-ms-win-*`) normalize automatically in `crates/win32-rt/src/lib.rs`. For
     runtime plugin DLLs/aliases/overrides outside `win32-rt`, use `register_builtin_dll`,
     `register_dll_alias`, or `register_export_override`.

5. **Test, sync catalogs, and verify boundaries.**
   - Add a focused unit test covering both the success path and error/unaligned/null edge cases.
   - Sync `db/win32_apis.tsv`, check drift across all catalogs, and run tests:
     ```sh
     cargo opsdb -- win32-sync
     cargo opsdb -- doctor
     cargo test -p win32-rt --lib --target aarch64-apple-darwin -- <module> --test-threads=1
     cargo runner-build
     ```
   - If you changed a public trait or signature in `win32-rt`, also check AW2 and iOS targets:
     ```sh
     cargo check -p aw2-game --no-default-features --target aarch64-apple-darwin
     IPHONEOS_DEPLOYMENT_TARGET=17.0 cargo check -p win32-rt --lib --target aarch64-apple-ios-sim
     ```
   - Re-run the failing game or PE binary to confirm it gets past the missing/broken API.

6. **Commit cleanly.**
   - Commit the Rust implementation, unit test, and updated `db/win32_apis.tsv` together:
     `win32: implement <dll>!<Name>` (or `d3d11:` / `d3d12:` / `vulkan:` / `audio:`).
