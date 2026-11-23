# Alan Wake 2 Hook Safety, SEH/FH4 & `difftest` Reference

Battle-tested rules from `games/alan-wake-2/crates/aw2-game/` and `docs/design/ADVICE_SEH.md`.

## 1. LTCG Volatile-Register Liveness & `FLOAT_RETURN`

- MSVC LTCG callers sometimes keep values live in volatile registers (`rcx`/`rdx`/`r8`–`r11`/
  `xmm0`–`xmm5`) across a call to a known leaf.
- `port-loader` wraps replacements in `hooks::preserving_shim` to preserve volatile registers and
  forward stack args 5–12.
- **Mandatory `FLOAT_RETURN` rule:** If a decompiled function returns a `f32`, `f64`, or `__m128`
  in `xmm0`, you **MUST** add its preferred-base VA to `FLOAT_RETURN` in
  `games/alan-wake-2/crates/aw2-game/src/registry/mod.rs` — otherwise `preserving_shim` restores
  caller `xmm0` and silently clobbers your return value!
- If a caller still breaks *only* when hooked, inspect the call-site disassembly for implicit
  contracts (live flags, upper `ymm` halves, >12 stack args, or callee-written stack slots).

## 2. C++ Exceptions & SEH (`Throws::{No, Discard, Caught}`)

Always inspect `tools/fndb.sh can-throw <va>` and `tools/fndb.sh show <va>` (`handler=` indicates
FH4 try/unwind maps):

| `Throws` | Meaning & Mandatory Rust Safety Contract |
|---|---|
| `Throws::No` | Nothing in the transitive call graph can throw. Uses the lightweight shim. |
| `Throws::Discard` | A callee can throw and the original has no local cleanup/catch. Uses a fence shim that unwinds straight through Rust frames to the original caller, **discarding Rust stack frames without running `Drop`**. Across any throwing call, hold **ONLY raw pointers and integers** — NEVER `Vec`, `String`, `Box`, `MutexGuard`, or `RefCell` borrows! MSVC STL contract throws (`_Xlength_error`, `_Xbad_function_call`, OOM) count as `Discard`. |
| `Throws::Caught` | Original has an FH4 try map or unwind map around a throwing callee. Wrap throwing calls in `exe::try_call(exe::ptr::<u8>(VA) as usize, &[args..], &[catch_rtti_names..])` (empty catch list = `catch (...)`), execute exact original cleanup/catch logic on `Err(e)`, and `e.rethrow()` or recover identically. |

Always run `tools/fndb.sh sync` so `db/throws.tsv` stays aligned with registry entries — `aw2-loader`
lints `db/throws.tsv` at boot and fails `--diff-test` on mismatches.

## 3. `gs:` TSD Slots & Unaligned Guest Memory

- **`gs:` is not the full TEB:** Under the loader, only `gs:[0x30]` (self / `exe::teb_self()`),
  `gs:[0x58]` (`ThreadLocalStoragePointer`), and `gs:[0x60]` (PEB) are mirrored in host TSD slots.
  Never swap `gs:[0x58]` for `*(teb_self() + 0x58)`.
- In difftests that touch guest TLS, call a `kernel32` import (e.g., `GetCurrentThreadId`) through
  the IAT first so the thread gets adopted, and return `SKIP` if guest TLS is still absent.
- **Unaligned guest memory:** Always use `exe::rd::<T>(addr)`, `exe::wr::<T>(addr, val)`, or
  `#[repr(C, packed)]` with compile-time size/offset assertions (`addr_of_mut!(..).write_unaligned`).
  Never take `&` of a `#[repr(C, packed)]` field or dereference raw guest pointers directly.
- **Callee routing:** Call undecompiled callees through their original VA via
  `exe::func::<unsafe extern "win64" fn(..) -> ..>(VA)(args)` (including CRT thunks like `j_memcpy`
  at `0x14183acbc`) so hooks and `difftest::Mock` interceptions still apply.

## 4. `aw2-core` Fast Iteration vs Full `aw2` Shards

- `cargo check -p aw2-game --no-default-features --target aarch64-apple-darwin` (~12 s): type-checks
  hand-written `ns/` without compiling the 4.47M LOC shards.
- `cargo check -p port-loader --features aw2-core --target aarch64-apple-darwin` (~15 s): checks
  `aw2-loader` + hand-decompiled `ns/` + Wwise bridge without shards (backed by `.aot` caches).
- `cargo loader -- --game-dir "$AW2_GAME_DIR" --diff-test`: runs full differential tests.
- Keep `Status::Draft` off `main` — unverified hooks break boot and `cargo hybrid-test`.
