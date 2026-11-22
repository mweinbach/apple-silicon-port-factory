---
name: decompile-function
description: Decompile one function from AlanWake2.exe and reimplement it in Rust under games/alan-wake-2/crates/aw2-game, registered as a hook replacement and verified against the original x86-64 code. Use only for bespoke Alan Wake 2 decompilation tasks.
---

# Decompile an Alan Wake 2 Function (`games/alan-wake-2/crates/aw2-game`)

Use this workflow **only** for the bespoke Alan Wake 2 target under `games/alan-wake-2/`. New games
on the universal port factory run directly on `port-runner` + `x86-runtime`'s ARM64 block JIT and
`.aot` cache without hand-decompiling functions. Never leak AW2 virtual addresses or special cases
into `crates/` (any AW2-only guard in shared code is gated behind `exe::aw2_registry_active()`).

## Checklist

1. **Pick a target.**
   - If you don't already have a VA, run `tools/fndb.sh next --ns <ns>` or `tools/fndb.sh show <name>`.
   - Treat `fndb`'s `owner` label as heuristic context (MSVC STL `_Traits_*`, `sparsehash`,
     `rapidjson`, and `std::hash` FNV often sit inside Remedy objects). Upstream code in
     `third_party/` is helpful reference, but `AlanWake2.exe` is always ground truth.
   - To identify hot functions executed during boot/gameplay, use `AW2_PROBE=@file`,
     `AW2_HOOK_STATS_FILE=out.tsv` (one `0xVA [name]` per line), or `AW2_EMU_PROF=1`.

2. **Inspect the binary thoroughly.**
   - Pull Ghidra pseudo-C (start the persistent server if decompiling multiple functions):
     ```sh
     tools/ghidra_decompile.sh --server-start
     tools/ghidra_decompile.sh 0x14XXXXXXX   # cached in out/decomp/<va>.c
     ```
     *(Stop `--server-stop` before running `tools/ghidra_analyze.sh --names` or `--export`, because
     Ghidra locks its project to one process at a time.)*
   - Always cross-check exact widths, signedness, 32- vs 64-bit ops, struct offsets, and `setcc`
     bool returns against the annotated disassembly:
     ```sh
     tools/fndb.sh disasm 0x14XXXXXXX
     tools/fndb.sh show 0x14XXXXXXX
     tools/fndb.sh can-throw 0x14XXXXXXX
     ```
   - Inspect class/vftable layouts in `re/rtti_classes.hpp` and signatures via
     `tools/fndb.sh funcsig <substr>`.
   - Verify MSVC x64 ABI details (`this` in `rcx`, hidden `sret` pointer for non-register structs).

3. **Implement in Rust (`games/alan-wake-2/crates/aw2-game/src/ns/<namespace>/`).**
   - Write `pub unsafe extern "win64" fn` with a doc comment giving the original name, VA, and what
     you checked against disassembly.
   - **Every guest memory access must tolerate unaligned addresses:** use `exe::rd::<T>(addr)`,
     `exe::wr::<T>(addr, val)`, or `#[repr(C, packed)]` structs with compile-time size/offset
     assertions. Never do typed derefs, `ptr::read`/`write`, or wide `from_raw_parts` on raw guest
     pointers, and never take `&` of a `#[repr(C, packed)]` field.
   - Call undecompiled callees through their original VA via
     `exe::func::<unsafe extern "win64" fn(..) -> ..>(VA)(args)` — including CRT import thunks like
     `j_memcpy` (`0x14183acbc`) — so hooks and difftest mocks still apply.
   - Read static tables and globals from the mapped exe via `aw2_game::exe::ptr(va)`. Never copy
     copyrighted game data into Rust source.
   - Match original `gs:` accesses literally (`exe::teb_self()` is `gs:[0x30]`; see Hook safety
     below).

4. **Register in `games/alan-wake-2/crates/aw2-game/src/registry/`.**
   - Insert sorted by VA into the appropriate registry module (`core.rs`, `ecs.rs`,
     `r_mem_alloc.rs`, `r_mem_dtor.rs`) with `Status::Draft`, an accurate `throws:` value, and
     `diff: Some(...)`.
   - If the function returns a float, double, or `__m128` in `xmm0`, you **must** also add its VA
     to `FLOAT_RETURN` in `games/alan-wake-2/crates/aw2-game/src/registry/mod.rs` so the preserving
     shim does not clobber the return value.

5. **Prove parity with a differential test.**
   - Write `pub unsafe fn diff_<name>(orig, new) -> Result<u32, String>` using `aw2_game::difftest`:
     feed edge cases and seeded random inputs to both sides, comparing return values and all
     mutated memory.
   - Include under-aligned addresses in inputs (offset +4 for structs, odd byte for `u16`).
   - For non-leaf functions, intercept callees on both sides with `difftest::Mock::new(callee_va,
     fixture)` and restore mutated globals with `difftest::Snapshot`.
   - For `bool` returns, compare only the low byte (`al`).
   - Do a quick mutation check on non-trivial logic (break a condition, verify `--diff-test`
     catches it, revert).
   - Fast compile check for hand-written `ns/` (~12 s, skips mass shards):
     ```sh
     cargo check -p aw2-game --no-default-features --target aarch64-apple-darwin
     ```
   - Run differential tests:
     ```sh
     cargo loader -- --game-dir "$AW2_GAME_DIR" --diff-test
     # or legacy Rosetta x86-64 oracle:
     cargo hybrid-test
     ```
   - Once passing, promote `Status::Draft` → `Status::Verified`, then boot with `AW2_HOOK_STATS=1`
     (and `PORT_CONTROL_DIR` / `AW2_CAMERA_TRACE=1` if gameplay-visible) to verify it runs cleanly
     in-game.

6. **Sync metadata and commit.**
   - Sync both progress and exception tables:
     ```sh
     tools/fndb.sh sync
     ```
   - Commit Rust code + `db/progress.tsv` + `db/throws.tsv` together:
     `game/<ns>: decompile <Name> (0x14XXXXXXX)`.

---

## Hook Safety Rules

Every rule below prevented real crashes:

- **LTCG volatile-register liveness:** MSVC LTCG callers sometimes keep values live in volatile
  registers (`rcx`/`rdx`/`r8`–`r11`/`xmm0`–`xmm5`) across a call to a known leaf. The loader runs
  replacements through `hooks::preserving_shim` to preserve those registers and forward stack args
  5–12. Always list `xmm0`-returning VAs in `FLOAT_RETURN`. If a caller breaks only when hooked,
  inspect its call-site disassembly for implicit contracts (flags, upper `ymm` halves, >12 stack
  args, or callee-written stack slots).
- **C++ exceptions & SEH (`throws:`):** Check `tools/fndb.sh can-throw <va>` and `fndb show <va>`
  (`handler=` indicates FH4 try/unwind maps; policy in `docs/design/ADVICE_SEH.md`):
  - `Throws::No` — nothing below can throw (light shim).
  - `Throws::Discard` — a callee can throw and the original has no local cleanup. Unwinds through
    Rust frames to the original caller, discarding them. Across any throwing call, hold **only raw
    pointers and integers** — zero `Vec`, `String`, `Box`, mutex guards, or `RefCell` borrows. STL
    contract throws (`_Xlength_error`, `_Xbad_function_call`, OOM) count as `Discard`.
  - `Throws::Caught` — original has a try map or unwind map with a throwing callee. Wrap throwing
    calls in `exe::try_call(exe::ptr::<u8>(VA) as usize, &[args..], &[catch_rtti..])`, run the
    original cleanup/catch logic on `Err(e)`, and `e.rethrow()` or recover identically.
  - Always run `tools/fndb.sh sync` so `db/throws.tsv` stays aligned; the loader lint catches
    mismatches at boot and fails `--diff-test`.
- **`gs:` is not the full TEB:** Only `gs:[0x30]` (self), `gs:[0x58]` (`ThreadLocalStoragePointer`),
  and `gs:[0x60]` (PEB) are mirrored in host TSD slots. Never swap `gs:[0x58]` for
  `*(teb_self() + 0x58)`. In difftests touching guest TLS, call a `kernel32` import like
  `GetCurrentThreadId` through the IAT first so the thread is adopted, and return `SKIP` if TLS is
  still absent.
- **Keep drafts out of `main`:** Unverified hooks break boot and `cargo hybrid-test`. Verify in a
  branch or worktree before landing on `main`.
