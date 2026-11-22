---
name: implement-win32-api
description: Implement or fix a Windows API (kernel32, ntdll, ucrt, user32, d3d12, d3d11, dxgi, vulkan-1, wasapi, xinput, hid, middleware) in crates/win32-rt. Use when port-runner or aw2-loader reports "UNIMPLEMENTED dll!name", or when adding or fixing Windows OS, CRT, COM, or graphics API behavior.
---

# Implement a Windows API (`crates/win32-rt`)

Follow the rules in `AGENTS.md` (especially: keep `crates/` game-agnostic, never import
`objc2-metal` in `win32-rt`, be honest about stubs, and keep `#[cfg(test)]` at the bottom of the
file).

## Checklist

1. **Reproduce the failure.**
   - Run the target with `port-runner` (or `cargo loader` for Alan Wake 2), adding `--trace` if you
     need surrounding import calls.
   - Record the exact DLL name, export name (or ordinal), and caller VA/module. If you have a binary
     audit, `cargo opsdb -- win32-audit /path/to/game/` shows every unresolved import up front.
   - To add a brand-new middleware/Win32 DLL module from scratch, you can scaffold it with:
     `cargo opsdb -- scaffold-dll <dll_name> /path/to/dll_or_exe [--write]`

2. **Verify the exact Windows ABI and contract.**
   - Check `third_party/directx-headers` (for D3D/DXGI/COM) or Microsoft's Win32 docs.
   - Function signature must be `pub unsafe extern "win64" fn Name(...) -> ...`.
   - Common Win64 types: `BOOL = i32`, `DWORD = u32`, `LONG = i32`, `WCHAR = u16` (UTF-16),
     `HANDLE = *mut c_void` (or `usize`). Remember guest pointers may be unaligned — use
     `ptr::read_unaligned` / `ptr::write_unaligned` when dereferencing guest structs.
   - Set last error in the TEB (`set_last_error(...)`) on failure when the Windows API specifies it.

3. **Implement real macOS behavior (or an honest stub).**
   - Build on real macOS primitives (POSIX/Mach, AppKit via `macos/`, CoreAudio via `audio/`,
     Metal 4 via `d3d12-metal` — never raw `objc2-metal` inside `win32-rt`).
   - If a stub is genuinely unavoidable, log it with `stub!(...)` / `failing!(...)` and return the
     documented Windows failure code (`E_NOTIMPL`, `ERROR_CALL_NOT_IMPLEMENTED`, `FALSE`, etc.).
     Never silently return `S_OK` or `memcpy` past an unimplemented feature.
   - Never stub out shipping compression DLLs (`oo2core_*`, `zstd`, `lz4`) — let the real PE DLL
     run on `x86-runtime` unless you have a real, tested implementation.

4. **Check export registration and `build.rs` placement.**
   - `crates/win32-rt/build.rs` scans each DLL file for `pub [unsafe] extern "win64" fn` and stops
     at the **first `#[cfg(test)]` token**. Put all `#[cfg(test)]` code at the bottom of the file,
     never above an export.
   - For COM objects or extra DLLs, wire into `DllRegistry` / `ComRegistry` or `misc/mod.rs` as
     appropriate. API-set DLLs (`api-ms-win-*`) normalize automatically in `crates/win32-rt/src/lib.rs`.

5. **Write a unit test and sync the Win32 catalog.**
   - Add a focused test covering the normal path and failure/edge cases.
   - Run:
     ```sh
     cargo opsdb -- win32-sync
     cargo test -p win32-rt --lib --target aarch64-apple-darwin -- <module> --test-threads=1
     cargo runner-build
     ```
   - Always pass `-- --test-threads=1` when running `win32-rt` tests (CoreAudio / `akrec` tests share
     process-global state and deadlock under parallel test threads).
   - Rerun the failing binary or test to confirm it gets past the missing API.

6. **Commit cleanly.**
   - Commit the code, test, and updated `db/win32_apis.tsv` together:
     `win32: implement <dll>!<Name>` (or `d3d11:` / `vulkan:` / `d3d12:` as appropriate).
