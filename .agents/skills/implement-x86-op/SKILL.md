---
name: implement-x86-op
description: Add, promote, or fix x86-64 to ARM64/NEON instruction translation across crates/x86-runtime (generic iced-x86 interpreter, FastOp, ARM64 JIT, relocatable .aot cache, static Mach-O .aot.o) and crates/port-factory/src/ops_db. Use when a binary hits an unimplemented or hot GENERIC_INTERP instruction, a verify-x86 mismatch, or an x86-64 flag/SIMD/MXCSR semantic bug.
---

# Implement or Fix an x86-64 → ARM64 Instruction (`crates/x86-runtime`)

`x86-runtime` executes guest x86-64 code across **four tiers** that must agree bit-for-bit
(GPRs, partial registers, RFLAGS, XMM/YMM upper halves, and `MXCSR`/`FPCR`):

1. **Generic `iced-x86` interpreter** (`crates/x86-runtime/src/emu/{mod.rs,simd_fp.rs,simd_mov.rs}`)
   — internal reference semantics (NEON-accelerated via `simd_mov::vops`).
2. **`FastOp` pre-decoded interpreter** (`emu/{fast.rs,fast_ext.rs,fast_more.rs}`) — compact
   **32-byte** decoded IR (`K_*` kinds + `VS_*`/`VI_*`/`VF_*` sub-ops).
3. **ARM64/NEON Live JIT** (`emu/{jit.rs,jit_ext.rs,jit_more.rs,jit_vec.rs,jit_xmm.rs,jit_lazy.rs}`)
   — compiles `FastOp` block prefixes into per-thread `MAP_JIT` code buffers.
4. **Relocatable `.aot` v6 & Static Mach-O `.aot.o`** (`emu/{aot.rs,jit_aot.rs,static_macho.rs}`)
   — multi-bank (up to 2 GiB / 112 MiB stub banks) disk caches and zero-`MAP_JIT` iOS objects.

Read [cpu_and_aot_invariants.md](./references/cpu_and_aot_invariants.md) before touching FP/NaN,
`MXCSR`/`FPCR`, AVX narrowing, FMA, stubs, or multi-bank `.aot` codegen!

## Checklist

1. **Inspect current coverage and scaffold the op.**
   ```sh
   cargo opsdb -- explain <mnemonic>
   cargo opsdb -- scaffold-op --from-pe /path/to/Game.exe
   cargo opsdb -- scaffold-op <mnemonic> --category <CAT> --arm64 "<arm64 pattern>"
   ```
   - Find hot `GENERIC_INTERP` mnemonics in a real run via `AW2_EMU_PROF=2`
     (`out/hitch/emu-prof-<pid>.log`).

2. **Verify exact x86-64 semantics and Apple Silicon invariants.**
   - Cross-check Intel SDM AND real x86-64 hardware via `tools/debug/x86_oracle.py` on Rosetta 2.
   - Check every critical invariant in [cpu_and_aot_invariants.md](./references/cpu_and_aot_invariants.md):
     - **`MXCSR` → `FPCR.FZ`/`RMode`** with strict host `FPCR` save/restore across `host_xfer`/exit.
     - **NEON FMA dual-NaN pinning:** never let LLVM commute `(lhs, m2)` in `vfmaq_f32/64`; pin
       inline `fmla` operand order (`Vd=rhs, Vn=lhs, Vm=m2`) so Generic, FastOp, and JIT match!
     - **VEX.128 vs Legacy SSE & 256→128 narrowing:** VEX.128 zeroes `ymm_hi[dst]` (`ymm_dirty`);
       narrowing ops (`K_VCVT_PD`, `K_VCVT_PH`) must read `ymm_hi[src]` BEFORE clobbering `dst`
       when `dst == src`.
     - **Partial GPR writes, `Cpu::last_bits`, lazy flags (`Cpu::pfv` + `NZCV`), shift-by-0,
       `B_GS`/`B_ADDR32`, unaligned TSO (`ldapr`/`stlr` + `dmb`), W^X (`exec_mem.rs`).**
     - **Hardware feature gating & CPUID honesty:** gate `aes`/`crc` intrinsics on `target_feature`
       with portable iOS fallbacks; never advertise a CPUID feature bit unless all tiers implement it.
     - **Stubs & AOT limits:** keep `FastOp` at **32 bytes**, keep shared stubs inside
       `STUBS_BYTES = 4096` (`static_stubs_with_veneer_fit_with_headroom`), keep static PIC /
       multi-bank `x16` GOT veneer compatibility, and never use eager `.then_some(imm + 1)` on
       unbounded integers!

3. **Implement bottom-up across tiers.**
   - **Tier 1 (Generic):** `emu/{mod.rs,simd_fp.rs,simd_mov.rs}` first (always keep Generic right).
   - **Tier 2 (`FastOp`):** `emu/{fast.rs,fast_ext.rs,fast_more.rs}` (reuse existing `K_*` kinds
     where possible; keep `size_of::<FastOp>() == 32`).
   - **Tier 3 (ARM64 JIT):** `emu/{jit.rs,jit_ext.rs,jit_more.rs,jit_vec.rs,jit_xmm.rs}` +
     flag/XMM liveness (`jit_lazy.rs`, `jit_xmm.rs`).
   - **Tier 4 (AOT / Static Mach-O):** update `emu/{aot.rs,jit_aot.rs,static_macho.rs}` if adding
     stubs, relocations, or terminators; verify `codegen_hash()` invalidates stale caches.

4. **Update the Ops DB catalog.**
   - Update `crates/port-factory/src/ops_db/catalog_gp.rs` or `catalog_simd.rs` with honest
     `TranslationTier` (`JitArm64`, `FastInterp`, `GenericInterp`, `TrapOrBoundary`).
   - Sync TSVs and verify zero drift:
     ```sh
     cargo opsdb -- sync
     cargo opsdb -- doctor
     ```

5. **Prove bit-for-bit parity across all tiers and targets.**
   - Add targeted unit/differential tests in `emu/jit_tests.rs`, `emu/fast.rs` (`fast_tests`),
     or `emu/tests.rs` covering all operand forms, edge values (`0`, `-0.0`, subnormals, `±Inf`,
     dual `SNaN`/`QNaN`, `i32::MIN`, shift-by-0, unaligned pointers, `VEX.128` upper-half zeroing),
     checking Generic == FastOp == Live JIT == AOT.
   - Run tests + iOS cross-check + offline 4-tier verifier (`--oracle` checks Rosetta 2 hardware!):
     ```sh
     cargo test -p x86-runtime --lib --target aarch64-apple-darwin -- --test-threads=1
     cargo test -p x86-runtime --test aot_multi_module_verify --target aarch64-apple-darwin -- --test-threads=1
     cargo check -p x86-runtime --lib --target aarch64-apple-ios
     cargo opsdb -- verify-x86 /path/to/target.dll --trials 4 --oracle --oracle-sample 64
     cargo runner-build
     cargo factory-test -- --test-threads=1
     ```
   - Live sanity checks: `AW2_EMU_VERIFY=1` (shadow-checks `FastOp` vs Generic) and
     `PORT_AOT_VERIFY=1` (re-translates & checks `.aot` blocks on load).
   - Check Rust 2024 lints (`dangerous_implicit_autorefs`) and `cargo check -p aw2-game --no-default-features --target aarch64-apple-darwin`.

6. **Commit cleanly.**
   - Commit `x86-runtime`, tests, `ops_db`, and `db/ops_x86_to_arm64.tsv` together:
     `x86-runtime: lower <mnemonic/family> in ARM64 JIT`.
