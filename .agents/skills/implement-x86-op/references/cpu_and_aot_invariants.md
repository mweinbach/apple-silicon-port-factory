# x86-64 → ARM64 CPU, SIMD, FPCR & Multi-Bank AOT Invariants

Hard-won architectural invariants from `crates/x86-runtime/src/emu/` and real game bring-up/NaN hunts.

## 1. Floating-Point, `MXCSR` → `FPCR` & Dual-NaN Parity

- **`MXCSR` → ARM64 `FPCR` (`FZ` + `RMode`) with strict host isolation:**
  - Real Windows engines (`AlanWake2.exe`, `PhysX3_x64.dll`) load `MXCSR = 0x9FC0` (`FTZ | DAZ`).
    Ignoring `ldmxcsr` leaves ARM64 `FPCR.FZ = 0`, letting tiny subnormals (`2^-136`) survive
    zero-guards and explode `1.0 / x` into `±Inf` → `-NaN`!
  - On Apple Silicon, `FPCR.FZ` (`1 << 24`) flushes **both** subnormal inputs and outputs to
    sign-preserving zero, matching `MXCSR FTZ | DAZ = 0x8040`.
  - Per-CPU `JitScratch::mxcsr` (default `0x1F80`) drives guest `FPCR` on entry to JIT/interpreter
    and **MUST restore host `FPCR` across `host_xfer` and exit** so CoreAudio/Metal/AppKit keep
    standard macOS IEEE-754 state.
- **NEON FMA dual-NaN operand commutation trap:**
  - Rust/LLVM's `vfmaq_f32(rhs, lhs, m2)` allows LLVM to commute `(lhs, m2)`, swapping dual-NaN
    priority relative to x86-64 hardware and `jit_ext.rs` (`v_fmla`)!
  - Pin inline `fmla` operand order (`Vd = rhs, Vn = lhs, Vm = m2`) so Generic (`vops::fma_ps/pd`),
    `FastOp` (`fast_ext.rs`), and Live JIT/AOT agree bit-for-bit on dual-NaN inputs.
- **Exact NaN vs Fast NaN:**
  - `PORT_JIT_FAST_NAN=0` enforces exact x86-64 first-operand NaN propagation & indefinite `-qNaN`
    (`0xffc0_0000` / `0xfff8_0000_0000_0000`). `x86-verify` defaults to `PORT_JIT_FAST_NAN=0`.
  - FP→int overflow produces `0x8000_0000` / `0x8000_0000_0000_0000` (`cvtt*2si`, `cvt*2dq`).
  - `minps`/`maxps`/`minpd`/`maxpd` use `vcltq`/`vcgtq` + `vbslq` (second operand wins on NaN or `-0.0/+0.0`).
  - `pmulhrsw(-32768, -32768)` wraps via widening `vmull_s16` + `vrshrn_n_s32::<15>`.

## 2. Register, Vector & Flag Semantics

- **VEX.128 vs Legacy SSE upper-lane rules (`JitScratch::ymm_dirty`):**
  - Legacy SSE preserves `ymm_hi[dst]` bits `255:128`.
  - `VEX.128` zeroes `ymm_hi[dst]` bits `255:128` (including `vblendps`/`vblendpd`/`vinsertps` and
    `K_VAES_*`!).
  - **Narrowing 256→128 trap (`d == s`):** Instructions like `K_VCVT_PD` (`vcvtpd2ps`,
    `vcvttpd2dq`, `vcvtpd2dq`) and `K_VCVT_PH` (`vcvtps2ph`) read `ymm_hi[s]` AND zero `ymm_hi[d]`.
    Never mark `d` as clobbered before reading `s` when `d == s`!
- **Partial GPR writes & `Cpu::last_bits`:**
  - 32-bit GPR writes zero-extend to 64-bit; 8/16-bit writes merge into low bits; `hi8` (`ah`/`ch`/
    `dh`/`bh`) sets `last_bits = 64`. Maintain `Cpu::last_bits` for ABI registers (`rax`, `rcx`,
    `rdx`, `r8`, `r9`).
- **Lazy flags (`jit_lazy.rs`) & speculative decoding:**
  - `PF` is maintained eagerly in `Cpu::pfv`; `ZF`/`SF`/`CF`/`OF` map onto host `NZCV`.
  - `AW2_JIT_FLAGLAZY=1` (`succ_need`) speculatively decodes conditional branch successors — obfuscated
    PEs place invalid junk bytes on dead opaque-predicate fall-throughs, so successor decoding must
    handle invalid/unmapped bytes cleanly without panicking.
  - Shift-by-0 leaves flags unchanged, yet a 32-bit destination STILL zero-extends to 64-bit!
- **Address-size & segment overrides:**
  - Preserve `B_ADDR32` (`0x67`: wrap EA mod `2^32` and zero-extend BEFORE adding `%gs:` base) and
    `B_GS` (`%gs:` segment base from `JitScratch::teb`).
- **Hardware Crypto/CRC feature gating:**
  - Gate ARMv8 crypto (`vaeseq`/`vaesdq`/`vmull_p64`) on `target_feature = "aes"` and `__crc32c*`
    on `target_feature = "crc"` with portable fallbacks so `aarch64-apple-ios` builds cleanly.
- **`CPUID` honesty (`emu/mod.rs`):**
  - Never advertise a CPUID feature bit (e.g., SHA-NI leaf 7 EBX bit 29) unless **every** execution
    tier implements all instructions in that extension.

## 3. `FastOp`, Tagged Dispatch & Multi-Bank `.aot` v6 / Static Mach-O

- **`FastOp` size invariant:** `assert_eq!(core::mem::size_of::<FastOp>(), 32)` (`fast_struct_size_is_32`).
- **`JEnt::fast` tagged host dispatch (`jit.rs`):**
  - Pure-integer `DF_PLAIN`: `(host_fn as u64) | 1`
  - Pure-float-math (`pure_float_math_fns`, `needs_xmm`): `(host_fn as u64) | 3`
  - Zero-spill inline QPC (`QueryPerformanceCounter`/`RtlQueryPerformanceCounter`):
    `(qpc_fast_state_ptr() as u64) | (1u64 << 63) | 1` (uses `mrs cntvct_el0` directly!).
  - **`STUBS_BYTES = 4096` (1,024 ARM64 words):** Always verify `static_stubs_with_veneer_fit_with_headroom`
    after modifying shared JIT/AOT stubs.
- **Multi-bank `.aot` v6 (`aot.rs`, `jit_aot.rs`, `static_macho.rs`):**
  - Supports up to **2 GiB** (`MAX_CODE = 2048 MiB`, `EVICT_CAP = 4 GiB`) split into **112 MiB**
    stub banks (`AOT_BANK_SPAN`, safely inside ARM64's ±128 MiB `B`/`BL` reach), with per-bank
    256 KiB `AOT_LSE2_SLAB` in `MAP_JIT` mode.
  - Multi-bank static Mach-O `.aot.o` uses `x16` GOT veneers (`ARM64_RELOC_GOT_LOAD_PAGE21/12`,
    **zero `ARM64_RELOC_BRANCH26`**) so `ld64` links 300–500+ MiB `__TEXT,__text` objects cleanly.
  - Default CFG closure depth (`DEFAULT_AOT_CLOSURE`) is `128`, discovering `.pdata`, entry,
    exports, PE32+ TLS callbacks, Load Config CFG `GuardCFFunctionTable` / `GuardLongJumpTargetTable`,
    `.pdata`-less leaf callees, `.reloc`/LEA leaf roots, and switch/jump tables.
- **Rust debug-overflow trap in CFG/jump-table discovery:**
  - Never write `(imm < MAX).then_some(imm + 1)` — `.then_some` eagerly evaluates `imm + 1` and
    panics when `imm == u64::MAX`! Always use `.then(|| imm + 1)`.
