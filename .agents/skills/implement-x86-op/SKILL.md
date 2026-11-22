---
name: implement-x86-op
description: Add, promote, or fix x86-64 to ARM64/NEON instruction translation across crates/x86-runtime (generic iced-x86 interpreter, FastOp, ARM64 JIT, relocatable .aot cache, static Mach-O .aot.o) and crates/port-factory/src/ops_db. Use when a binary hits an unimplemented or hot GENERIC_INTERP instruction, a verify-x86 mismatch, or an x86-64 flag/SIMD/MXCSR semantic bug.
---

# Implement or Fix an x86-64 → ARM64 Instruction (`crates/x86-runtime`)

`x86-runtime` executes guest x86-64 code across **four tiers** that must stay bit-for-bit
consistent (including GPRs, partial registers, RFLAGS, XMM/YMM upper halves, and `MXCSR`):

1. **Generic `iced-x86` interpreter** (`crates/x86-runtime/src/emu/{mod.rs,simd_fp.rs,simd_mov.rs}`)
   — the reference semantics inside the runtime.
2. **`FastOp` pre-decoded interpreter** (`emu/{fast.rs,fast_ext.rs,fast_more.rs}`) — compact
   decoded IR (`K_*` kinds + `VS_*`/`VI_*`/`VF_*` sub-ops).
3. **ARM64/NEON Live JIT** (`emu/{jit.rs,jit_ext.rs,jit_more.rs,jit_vec.rs,jit_xmm.rs,jit_lazy.rs}`)
   — compiles `FastOp` block prefixes into per-thread `MAP_JIT` code buffers.
4. **Relocatable AOT & Static Mach-O** (`emu/{aot.rs,jit_aot.rs,static_macho.rs}`) — persistent
   `.aot` disk caches and zero-`MAP_JIT` signed `.aot.o` objects for iOS.

## Checklist

1. **Inspect current coverage and scaffold the op.**
   ```sh
   cargo opsdb -- explain <mnemonic>
   cargo opsdb -- scaffold-op --from-pe /path/to/Game.exe
   cargo opsdb -- scaffold-op <mnemonic> --category <CAT> --arm64 "<arm64 pattern>"
   ```
   - To find hot `GENERIC_INTERP` mnemonics during a real run, set `AW2_EMU_PROF=2` (writes
     `out/hitch/emu-prof-<pid>.log`).

2. **Verify exact x86-64 architectural semantics first.**
   - Check Intel SDM (and cross-check against real x86-64 hardware via `tools/debug/x86_oracle.py`
     on Rosetta 2 for any subtle edge case).
   - Confirm:
     - **Encodings:** Legacy SSE vs `VEX.128` vs `VEX.256`, REX.W, 8/16/32/64-bit widths.
     - **VEX vs Legacy upper-lane rules:** Legacy SSE preserves `ymm_hi[dst]` bits `255:128`;
       `VEX.128` zeroes bits `255:128` and must stay consistent with `JitScratch::ymm_dirty`.
     - **Partial GPR writes & `last_bits`:** 32-bit GPR writes zero-extend to 64-bit; 8/16-bit
       writes merge into the low bits; maintain `Cpu::last_bits` for ABI registers (`rax`, `rcx`,
       `rdx`, `r8`, `r9`).
     - **Flags (`jit_lazy.rs`):** Track which RFLAGS bits are written/cleared/preserved/undefined;
       remember `PF` is maintained eagerly in `Cpu::pfv` while `ZF`/`SF`/`CF`/`OF` map onto host
       `NZCV` via lazy flag sources. Check shift-count-0 rules (flags unchanged, 32-bit dst still
       zero-extends!).
     - **FP NaN & `MXCSR` parity (`JitScratch::mxcsr`):** Honor `PORT_JIT_FAST_NAN=0` exact x86
       first-operand NaN propagation, QNaN indefinite (`0x8000_0000` / `0x8000_0000_0000_0000` on
       FP→int overflow), and `MXCSR` FTZ/DAZ/rounding-mode bits.
     - **Memory, alignment & TSO:** All guest pointers may be unaligned. Range-check guest memory
       so faults side-exit cleanly to the interpreter. Preserve `B_GS` (`%gs:`) and `B_ADDR32`
       (`0x67` 32-bit address wrap before `%gs:` base). Under TSO, use `ldapr`/`stlr` with
       unaligned fallback + `dmb`.
     - **W^X & AOT relocatability:** Never emit mutable data on RX/JIT pages (`exec_mem.rs`). Any
       absolute host pointer/helper emitted in JIT must either go through shared stubs or emit an
       `AotReloc` entry in `jit_aot.rs` / `aot.rs` (and work in `static_macho.rs` for iOS).

3. **Implement bottom-up across tiers.**
   - **Tier 1 (Generic):** Implement/verify in `emu/mod.rs`, `emu/simd_fp.rs`, or `emu/simd_mov.rs`
     first. Never leave Generic broken while fixing only JIT.
   - **Tier 2 (`FastOp`):** Add decoding + execution in `emu/fast.rs` / `fast_ext.rs` /
     `fast_more.rs` (reuse existing `K_*` kinds + sub-ops where possible).
   - **Tier 3 (ARM64 JIT):** Emit ARM64/NEON in `emu/jit.rs` / `jit_ext.rs` / `jit_more.rs` /
     `jit_vec.rs` / `jit_xmm.rs`, updating flag liveness (`jit_lazy.rs`) and XMM register allocation
     (`jit_xmm.rs`) as needed.
   - **Tier 4 (AOT):** Check `emu/aot.rs` and `emu/static_macho.rs` if you added new stubs,
     relocations, or block-terminator forms; bump any AOT cache/codegen version key if binary
     encoding invariants changed.

4. **Update the canonical Ops DB catalog.**
   - Update `crates/port-factory/src/ops_db/catalog_gp.rs` or `catalog_simd.rs` (keeping
     `TranslationTier` honest: `JitArm64`, `FastInterp`, `GenericInterp`, or `TrapOrBoundary`).
   - `classify_instruction` auto-indexes `OpRecord::x86_mnemonics`, so no duplicate match arm is
     needed.
   - Sync TSVs and verify zero drift:
     ```sh
     cargo opsdb -- sync
     cargo opsdb -- doctor
     ```

5. **Prove bit-for-bit parity across all tiers.**
   - Add targeted unit tests in `crates/x86-runtime/src/emu/jit_tests.rs` (or `tests.rs`) covering
     all operand forms (`RR`, `RM`, `MR`, immediates), edge values (`0`, `-0.0`, subnormals, `±Inf`,
     `SNaN`/`QNaN`, `i32::MIN`, shift-by-0, unaligned memory, `VEX.128` upper-half zeroing),
     checking Generic == FastOp == Live JIT == AOT.
   - Run the unit and multi-module AOT tests:
     ```sh
     cargo test -p x86-runtime --lib --target aarch64-apple-darwin
     cargo test -p x86-runtime --test aot_multi_module_verify --target aarch64-apple-darwin -- --test-threads=1
     ```
   - Run the offline 4-tier verifier (+ Rosetta 2 hardware oracle!) against a real PE/DLL that
     contains the instruction:
     ```sh
     cargo opsdb -- verify-x86 /path/to/target.dll --trials 4 --oracle --oracle-sample 64
     # Or focused RVA / FP-only scan:
     cargo x86-verify /path/to/target.exe --fp-only --rva 0x1000..0x200000
     ```
   - For live verification inside a game, `AW2_EMU_VERIFY=1` compares every eligible `FastOp`
     against Generic on a shadow CPU, and `PORT_AOT_VERIFY=1` re-translates and compares AOT blocks
     word-for-word on load.
   - Verify universal build & tests:
     ```sh
     cargo runner-build
     cargo factory-test -- --test-threads=1
     cargo check -p aw2-game --no-default-features --target aarch64-apple-darwin
     ```

6. **Commit cleanly.**
   - Commit `x86-runtime` changes, tests, `ops_db` updates, and `db/ops_x86_to_arm64.tsv` together:
     `x86-runtime: lower <mnemonic/family> in ARM64 JIT`.
