---
name: decompile-function
description: Decompile one function from AlanWake2.exe and reimplement it in Rust under games/alan-wake-2/crates/aw2-game, registered as a hook replacement and verified against the original x86-64 code. Use only for bespoke Alan Wake 2 decompilation tasks.
---

# Decompile an Alan Wake 2 Function (`games/alan-wake-2/crates/aw2-game`)

Use this workflow **only** for the bespoke Alan Wake 2 target under `games/alan-wake-2/`. New games
on the universal port factory run directly on `port-runner` + `x86-runtime`'s ARM64 block JIT and
`.aot` cache without hand-decompiling functions. Never leak AW2 virtual addresses or special cases
into `crates/` (any AW2-only guard in shared code is gated behind `exe::aw2_registry_active()`).

Read [hook_safety_and_seh.md](./references/hook_safety_and_seh.md) before hooking any non-leaf,
`xmm0`-returning, TLS-touching, or throwing function — every rule there prevented real crashes.

## Checklist

1. **Pick a target.**
   - Run `tools/fndb.sh next --ns <ns>` or `tools/fndb.sh show <name>`.
   - Treat `fndb`'s `owner` label as heuristic context (MSVC STL `_Traits_*`, `sparsehash`,
     `rapidjson`, and `std::hash` FNV often sit inside Remedy objects). Upstream code in
     `third_party/` is helpful reference, but `AlanWake2.exe` is always ground truth.
   - To identify hot functions executed during boot/gameplay, use `AW2_PROBE=@file`,
     `AW2_HOOK_STATS_FILE=out.tsv`, `AW2_EMU_PROF=1`, or `AW2_PC_SAMPLE=500`.

2. **Inspect the binary thoroughly.**
   - Pull Ghidra pseudo-C (start the persistent server when decompiling multiple functions):
     ```sh
     tools/ghidra_decompile.sh --server-start
     tools/ghidra_decompile.sh 0x14XXXXXXX   # cached in out/decomp/<va>.c
     ```
     *(Stop `--server-stop` before running `tools/ghidra_analyze.sh --names` or `--export`, because
     Ghidra locks its project to one process at a time.)*
   - Cross-check exact widths, signedness, 32- vs 64-bit ops, struct offsets, and `setcc` bool
     returns against annotated disassembly:
     ```sh
     tools/fndb.sh disasm 0x14XXXXXXX
     tools/fndb.sh show 0x14XXXXXXX
     tools/fndb.sh can-throw 0x14XXXXXXX
     ```
   - Inspect class/vftable layouts in `re/rtti_classes.hpp` and signatures via
     `tools/fndb.sh funcsig <substr>`. Verify MSVC x64 ABI (`this` in `rcx`, hidden `sret` for
     structs not of size 1/2/4/8 bytes, reversed overloaded virtual methods).

3. **Implement in Rust (`games/alan-wake-2/crates/aw2-game/src/ns/<namespace>/`).**
   - Write `pub unsafe extern "win64" fn` with a doc comment giving the original name, VA, and what
     you verified against disassembly.
   - **Tolerate unaligned guest memory everywhere:** use `exe::rd::<T>(addr)`, `exe::wr::<T>(addr, val)`,
     or `#[repr(C, packed)]` structs with compile-time size/offset assertions. Never take `&` of a
     `#[repr(C, packed)]` field or dereference raw guest pointers directly.
   - Route undecompiled callees through their original VA via
     `exe::func::<unsafe extern "win64" fn(..) -> ..>(VA)(args)` (including CRT thunks like
     `j_memcpy` at `0x14183acbc`) so hooks and `difftest::Mock` interceptions still apply.
   - Read static tables/globals via `aw2_game::exe::ptr(va)` (never copy copyrighted game data into
     Rust source). Match `gs:` accesses literally (`exe::teb_self()` is `gs:[0x30]`).

4. **Register in `games/alan-wake-2/crates/aw2-game/src/registry/`.**
   - Insert sorted by VA into `core.rs`, `ecs.rs`, `r_mem_alloc.rs`, or `r_mem_dtor.rs` with
     `Status::Draft`, an accurate `throws:` (`No`, `Discard`, or `Caught` — see
     [hook_safety_and_seh.md](./references/hook_safety_and_seh.md)), and `diff: Some(...)`.
   - If the function returns `f32`, `f64`, or `__m128` in `xmm0`, you **MUST** add its VA to
     `FLOAT_RETURN` in `games/alan-wake-2/crates/aw2-game/src/registry/mod.rs`.

5. **Prove parity with a differential test.**
   - Write `pub unsafe fn diff_<name>(orig, new) -> Result<u32, String>` using `aw2_game::difftest`:
     feed edge cases, under-aligned addresses (+4 struct / odd `u16`), and seeded random inputs to
     both sides; compare return values (low byte `al` only for `bool`) and all mutated memory.
   - Use `difftest::Mock::new(callee_va, fixture)` for non-leaf callees and `difftest::Snapshot`
     to restore mutated globals. Do a quick mutation check (break logic, confirm `--diff-test`
     catches it, revert).
   - Fast compile checks (~12–15 s, skipping 4.47M LOC shards):
     ```sh
     cargo check -p aw2-game --no-default-features --target aarch64-apple-darwin
     cargo check -p port-loader --features aw2-core --target aarch64-apple-darwin
     ```
   - Run differential tests:
     ```sh
     cargo loader -- --game-dir "$AW2_GAME_DIR" --diff-test
     # or legacy Rosetta x86-64 oracle:
     cargo hybrid-test
     ```
   - Promote `Status::Draft` → `Status::Verified`, then boot with `AW2_HOOK_STATS=1` (and
     `PORT_CONTROL_DIR` / `AW2_CAMERA_TRACE=1` if gameplay-visible). Keep unverified drafts off `main`.

6. **Sync metadata and commit.**
   - Run `tools/fndb.sh sync` to update `db/progress.tsv` and `db/throws.tsv` (verified by the
     boot-time `throws` linter).
   - Commit Rust code + `db/progress.tsv` + `db/throws.tsv` together:
     `game/<ns>: decompile <Name> (0x14XXXXXXX)`.
