---
name: decompile-function
description: Decompile one function from AlanWake2.exe and reimplement it in Rust under games/alan-wake-2/crates/aw2-game, registered as a hook replacement and verified against the original x86-64 code. Use only for bespoke Alan Wake 2 decompilation tasks.
---

# Decompile an Alan Wake 2 function (`games/alan-wake-2/crates/aw2-game`)

Use this workflow only for the Alan Wake 2 decompilation target under `games/alan-wake-2/`. New
games on the universal port factory run directly on `port-runner` + `x86-runtime`'s ARM64 block
JIT and `.aot` cache without hand-decompiling functions.

## Checklist

1. **Pick a target.**
   - If you don't already have a VA, run `tools/fndb.sh next --ns <ns>` or `tools/fndb.sh show <name>`.
   - Note the `owner` field as context (`fndb`'s ownership labels are heuristic: MSVC STL
     `_Traits_*`, `sparsehash`, `rapidjson`, and `std::hash` FNV often sit inside Remedy objects).
     Upstream code in `third_party/` is a helpful reference, but `AlanWake2.exe` is always the
     ground truth.
   - To see which functions actually execute during boot, use `AW2_PROBE=@file` and
     `AW2_HOOK_STATS_FILE=out.tsv` (one `0xVA [name]` per line).

2. **Inspect the binary thoroughly.**
   - Pull Ghidra C output: `tools/ghidra_decompile.sh 0x14XXXXXXX`.
   - Always double-check exact widths, signedness, 32- vs 64-bit ops, and `setcc` bool returns in
     `tools/fndb.sh disasm 0x14XXXXXXX`.
   - Check struct/vftable layouts in `re/rtti_classes.hpp` and signatures via `tools/fndb.sh funcsig`.
   - Note MSVC x64 calling convention (`this` in `rcx`, hidden sret pointer for large structs).

3. **Implement in Rust (`games/alan-wake-2/crates/aw2-game/src/ns/<namespace>/`).**
   - Write `pub unsafe extern "win64" fn` with a doc comment giving the original name, VA, and what
     you checked.
   - Every guest memory access must tolerate unaligned addresses: use `exe::rd::<T>(addr)`,
     `exe::wr::<T>(addr, val)`, or `#[repr(C, packed)]` structs with size/offset assertions. Never
     do a typed dereference, `ptr::read`/`write`, or wide `from_raw_parts` on raw guest pointers.
   - Call undecompiled callees through their original VA via
     `exe::func::<unsafe extern "win64" fn(..) -> ..>(VA)(args)` — including CRT import thunks like
     `j_memcpy` (`0x14183acbc`) — so hooks and difftest mocks still apply.
   - Read static tables and globals from the mapped exe via `aw2_game::exe::ptr(va)`. Never copy
     game data into Rust source.
   - Match the original `gs:` access literally (`exe::teb_self()` is `gs:[0x30]`; see "Hook safety"
     below).

4. **Register in `games/alan-wake-2/crates/aw2-game/src/registry/`.**
   - Insert sorted by VA with `Status::Draft`, an accurate `throws:` value (see below), and
     `diff: Some(...)`.
   - If the function returns a float, double, or `__m128` in `xmm0`, also add its VA to
     `FLOAT_RETURN` so the preserving shim doesn't clobber the return value.

5. **Prove parity with a differential test.**
   - Write `pub unsafe fn diff_<name>(orig, new) -> Result<u32, String>` using `aw2_game::difftest`:
     feed identical edge cases and seeded random inputs to both sides, then compare return values
     and all mutated memory.
   - Include under-aligned addresses in the test inputs (offset 4 for structs, odd byte for `u16`).
   - For non-leaf functions, use `difftest::Mock::new(callee_va, fixture)` to intercept callees on
     both sides and `difftest::Snapshot` to restore globals. Keep fixtures clearly scoped to tests.
   - For bool returns, compare only the low byte (`al`).
   - Do a quick mutation check on non-trivial logic (intentionally break a condition, confirm the
     difftest catches it, revert).
   - Fast compile check for hand-written `ns/`:
     ```sh
     cargo check -p aw2-game --no-default-features --target aarch64-apple-darwin
     ```
   - Run differential tests:
     ```sh
     cargo loader -- --game-dir "$AW2_GAME_DIR" --diff-test
     # or legacy Rosetta oracle:
     cargo hybrid-test
     ```
   - Once passing, promote `Status::Draft` → `Status::Verified`, then boot with `AW2_HOOK_STATS=1`
     to verify the hook runs cleanly in-game.

6. **Sync metadata and commit.**
   - Run `tools/fndb.sh sync` to update `db/progress.tsv` and `db/throws.tsv`.
   - Commit code + `db/progress.tsv` + `db/throws.tsv` together:
     `game/<ns>: decompile <Name> (0x14XXXXXXX)`.

---

## Hook safety rules

Read these before hooking any non-leaf or hot function — every one of these caused real crashes:

- **LTCG volatile-register liveness:** MSVC LTCG callers sometimes keep values live in volatile
  registers (`rcx`/`rdx`/`r8`–`r11`/`xmm0`–`xmm5`) across a call to a known leaf. The loader runs
  replacements through `hooks::preserving_shim` to preserve those registers and forward stack args
  5–12. If a function returns in `xmm0`, you *must* list its VA in `FLOAT_RETURN`. If a caller still
  breaks only when hooked, inspect its disassembly around the call site for implicit contracts
  (flags, upper `ymm` halves, >12 stack args, or callee-written stack slots).
- **C++ exceptions & SEH (`throws:`):** Check `tools/fndb.sh can-throw <va>` and `fndb show <va>`
  (`handler=` indicates FH4 try/unwind maps; policy in `docs/design/ADVICE_SEH.md`):
  - `Throws::No` — nothing below can throw (uses the light shim).
  - `Throws::Discard` — a callee can throw and the original has no local cleanup. Uses a fence shim
    that unwinds through Rust frames to the original caller, discarding them. Across any throwing
    call, hold **only raw pointers and integers** — no `Vec`, `String`, `Box`, mutex guards, or
    `RefCell` borrows. STL contract throws (`_Xlength_error`, `_Xbad_function_call`, OOM) count as
    `Discard`.
  - `Throws::Caught` — the original has a try map or unwind map with a throwing callee. Wrap
    throwing calls in `exe::try_call(exe::ptr::<u8>(VA) as usize, &[args..], &[...])`, run the
    original cleanup/catch logic on `Err(e)`, and `e.rethrow()` or recover identically.
  - Always run `tools/fndb.sh sync` so `db/throws.tsv` stays aligned; the loader lint catches
    mismatches at boot and fails `--diff-test`.
- **`gs:` is not the full TEB:** Under the loader, only `gs:[0x30]` (self), `gs:[0x58]`
  (`ThreadLocalStoragePointer`), and `gs:[0x60]` (PEB) are mirrored in host TSD slots. Never swap
  `gs:[0x58]` for `*(teb_self() + 0x58)`. In difftests that touch guest TLS, call a `kernel32`
  import like `GetCurrentThreadId` through the IAT first so the thread gets adopted, and return
  `SKIP` if TLS is still absent.
- **Unaligned guest structs:** Game fields frequently sit on 4-byte or odd boundaries. Never take
  `&` of a `#[repr(C, packed)]` field; use `exe::rd`/`exe::wr` or `addr_of_mut!(..).write_unaligned`.
- **Keep drafts out of `main`:** Unverified hooks break boot and `cargo hybrid-test` for everyone.
  Verify in a worktree or branch before landing on `main`.
