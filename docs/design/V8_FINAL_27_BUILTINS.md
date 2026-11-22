# V8 Final 27 Embedded Builtins — `g28` Integration Spec (`100%` V8 Embedded Blob Coverage)

Surveyed by `tools/decomp_scale/survey_v8_27.py` against `$AW2_GAME_DIR/v8.dll` (pinned at `0x70_006e_c000`, non-Win64-ABI embedded code range `+0x104e54..+0x24acc8`, `1,334,900` bytes, `1,643` builtins in `EmbeddedData::LayoutDescription`).

After `g27` (`1,616 / 1,643 = 98.36%` lifted via `DF_REGFILE`), **exactly 27 builtins** remain unlifted across all of `v8.dll`:
- **`14` `v8_nonabi_x87`** (`17` `Float64Mod` reduction sites)
- **`8` `v8_nonabi_atomic`** (`[538..545]`: `48` `lock cmpxchg` sites + `12` `xchg [mem], reg` sites)
- **`5` `v8_nonabi_trap_stub`** (`1`-instruction `int3` architecture/simulator trap stubs)

All runtime primitives, x87 80-bit extended-precision state, misaligned-safe guest atomics, `iced-x86` oracle extensions, and differential unit tests live in [`regfile_ext.rs`](../../crates/x86-runtime/src/emu/regfile_ext.rs).

---

## 1. Complete Inventory of the 27 Builtins

### 1.1 `v8_nonabi_x87` (14 builtins, 17 `fprem` sites)

Every single x87 instruction in `v8.dll`'s embedded blob comes from V8's TurboFan/CSA `Float64Mod` macro-assembler sequence (`sub rsp, 8; movsd [rsp], divisor; fld qword ptr [rsp]; movsd [rsp], dividend; fld qword ptr [rsp]; .loop: fprem; fnstsw ax; shr eax, 8; and eax, 0xff; push rax; popfq; jp .loop; fstp st(1); fstp qword ptr [rsp]; movsd dst, [rsp]; add rsp, 8`):

| `bid` | V8 Builtin (`Builtins::Metadata`) | VA | RVA Range | Bytes | Insns | `fprem` RVAs |
|---:|---|---|---|---:|---:|---|
| `136` | `StoreFastElementIC_Standard` | `0x70008081b4` | `+0x11c1b4..+0x11fad8` | `14,628` | `3,803` | `+0x11d687`, `+0x11ecca` |
| `138` | `StoreFastElementIC_NoTransitionIgnoreOOB` | `0x700080e1d4` | `+0x1221d4..+0x1257b4` | `13,792` | `3,545` | `+0x123612`, `+0x124c29` |
| `421` | `Modulus_Baseline` | `0x700084acb4` | `+0x15ecb4..+0x15effc` | `840` | `238` | `+0x15edfb` |
| `433` | `Modulus_WithFeedback` | `0x700084d374` | `+0x161374..+0x1616ac` | `824` | `225` | `+0x1614b4` |
| `556` | `StringPrototypeSplit` | `0x7000862694` | `+0x176694..+0x17726c` | `3,032` | `778` | `+0x176f94`, `+0x176fca` |
| `574` | `TypedArrayPrototypeMap` | `0x7000863e14` | `+0x177e14..+0x17a884` | `10,864` | `2,657` | `+0x179313` |
| `855` | `Modulus` | `0x70008ae1b4` | `+0x1c21b4..+0x1c2434` | `640` | `163` | `+0x1c231a` |
| `1136` | `StoreTypedElementJSAny_Uint8ClampedElements_0` | `0x70008efdb4` | `+0x203db4..+0x20400c` | `600` | `166` | `+0x203f46` |
| `1223` | `ModHandler` | `0x70008fa0d4` | `+0x20e0d4..+0x20e4c8` | `1,012` | `262` | `+0x20e27a` |
| `1235` | `ModSmiHandler` | `0x70008fcf94` | `+0x210f94..+0x211338` | `932` | `252` | `+0x2111e6` |
| `1390` | `ModWideHandler` | `0x7000911ff4` | `+0x225ff4..+0x2263f0` | `1,020` | `265` | `+0x22619a` |
| `1402` | `ModSmiWideHandler` | `0x7000914ed4` | `+0x228ed4..+0x22927c` | `936` | `253` | `+0x229126` |
| `1539` | `ModExtraWideHandler` | `0x7000928b94` | `+0x23cb94..+0x23cf8c` | `1,016` | `264` | `+0x23cd38` |
| `1551` | `ModSmiExtraWideHandler` | `0x700092b9f4` | `+0x23f9f4..+0x23fd98` | `932` | `253` | `+0x23fc44` |

Exact unhandled instruction forms across all 14 builtins (`0` other unhandled mnemonics):
- `34x` `fld qword ptr [rsp]` (`dd 04 24`)
- `17x` `fprem` (`d9 f8`)
- `17x` `fnstsw ax` (`df e0`)
- `17x` `popfq` (`9d`, used after `push rax` to load `SW[15:8]` into `RFLAGS` so `PF = C2`)
- `17x` `fstp st(1)` (`dd d9`)
- `17x` `fstp qword ptr [rsp]` (`dd 1c 24`)

### 1.2 `v8_nonabi_atomic` (8 builtins, `[538..545]`)

| `bid` | V8 Builtin | VA | RVA Range | Bytes | Insns | Atomic Forms |
|---:|---|---|---|---:|---:|---|
| `538` | `AtomicsStore` | `0x700085d0d4` | `+0x1710d4..+0x1714c4` | `1,008` | `270` | `4x` `xchg [mem], reg` (`8/16/32/64`-bit) |
| `539` | `AtomicsExchange` | `0x700085d4d4` | `+0x1714d4..+0x171b98` | `1,732` | `434` | `8x` `xchg [mem], reg` (`8/16/32/64`-bit signed/unsigned) |
| `540` | `AtomicsCompareExchange` | `0x700085dbb4` | `+0x171bb4..+0x172410` | `2,140` | `540` | `8x` `lock cmpxchg [mem], reg` (`8/16/32/64`-bit) |
| `541` | `AtomicsAdd` | `0x700085e414` | `+0x172414..+0x172b54` | `1,856` | `468` | `8x` `lock cmpxchg [mem], reg` CAS retry loops |
| `542` | `AtomicsSub` | `0x700085eb74` | `+0x172b74..+0x1732b4` | `1,856` | `468` | `8x` `lock cmpxchg [mem], reg` CAS retry loops |
| `543` | `AtomicsAnd` | `0x700085f2d4` | `+0x1732d4..+0x173a14` | `1,856` | `468` | `8x` `lock cmpxchg [mem], reg` CAS retry loops |
| `544` | `AtomicsOr` | `0x700085fa34` | `+0x173a34..+0x174174` | `1,856` | `468` | `8x` `lock cmpxchg [mem], reg` CAS retry loops |
| `545` | `AtomicsXor` | `0x7000860194` | `+0x174194..+0x1748d4` | `1,856` | `468` | `8x` `lock cmpxchg [mem], reg` CAS retry loops |

Exact unhandled instruction forms across all 8 builtins (`0` other unhandled mnemonics):
- `llvm-objdump` emits the `0xF0` prefix of every `[F0] [66] [REX] 0F B0/B1` as a standalone pseudo-instruction `(rva, "lock", "", rva + 1)` followed by `(rva + 1, "cmpxchg", "<size> ptr [ea], <reg>", nxt_rva)`. **Fault VA subtlety:** in `run_oracle`, `iced-x86` decodes `F0 ... 0F B0/B1` as a single instruction starting at `lock_rva`, so on an invalid memory address the lowered `cmpxchg` **must** return `RegfileExit::fault(lock_va)` (the address of the `0xF0` prefix byte, not `lock_va + 1`!).
- `12x` `xchg <byte|word|dword|qword> ptr [base + idx], <reg>` (`cl`, `r11b`, `cx`, `r11w`, `ecx`, `r11d`, `rcx`).

### 1.3 `v8_nonabi_trap_stub` (5 builtins)

| `bid` | V8 Builtin | VA | RVA | Bytes | Disassembly |
|---:|---|---|---|---:|---|
| `117` | `BigIntToI32Pair` | `0x70007ff994` | `+0x113994` | `4` | `cc 90 66 90` (`int3; nop; nop`) |
| `119` | `I32PairToBigInt` | `0x70007ffa94` | `+0x113a94` | `4` | `cc 90 66 90` (`int3; nop; nop`) |
| `581` | `WasmI32AtomicWait32` | `0x7000866eb4` | `+0x17aeb4` | `4` | `cc 90 66 90` (`int3; nop; nop`) |
| `582` | `WasmI64AtomicWait32` | `0x7000866ed4` | `+0x17aed4` | `4` | `cc 90 66 90` (`int3; nop; nop`) |
| `627` | `DirectCEntry` | `0x700086f9b4` | `+0x1839b4` | `4` | `cc cc cc cc` (`int3 x4`) |

`_lower_builtin` already lowers `int3` to `return RegfileExit::fault(0x{ins_va:x});`. At runtime, `emu_run_full` handles `RF_FAULT(va)` by calling `regfile::arm_skip(va)` and re-running the loop at `va`, where `take_skip(va)` bypasses `lookup_routed_regfile` for one step and executes the `int3` trap handler cleanly without infinite re-entry. Removing the `if insns[0][1] in ("int3", "ud2"): rejected.append(...)` filter lifts all 5 stubs to 100% bit-exact `RegfileFn`s (`RF_FAULT` parity in `diff_regfile`).

---

## 2. Whole-Port `.pdata` Scan (`AlanWake2.exe` + All 8 Guest DLLs)

| Image | `.pdata` Entries | `fprem` / `fprem1` | `cmpxchg` | `xchg [mem], reg` |
|---|---:|---:|---:|---:|
| `AlanWake2.exe` | `142,429` | **`0`** | `769` (`500` fns) | `5,536` (`2,633` fns) |
| `cohtml.windowsdesktop.dll` | `24,558` | **`0`** | `524` (`253` fns) | `58` (`35` fns) |
| `v8.dll` | `61,463` | **`17` (`14` fns — all in embedded blob)** | `348` (`198` fns) | `263` (`124` fns) |
| `v8_libbase.dll` | `591` | **`0`** | `3` (`3` fns) | `2` (`2` fns) |
| `msvcp140.dll` | `1,237` | **`0`** | `1` (`1` fn) | `1` (`1` fn) |
| `vcruntime140.dll` | `253` | **`0`** | `1` (`1` fn) | `4` (`1` fn) |
| `vcruntime140_1.dll` | `87` | **`0`** | `0` | `4` (`1` fn) |
| `v8_libplatform.dll` | `539` | **`0`** | `8` (`8` fns) | `2` (`2` fns) |
| `renoircore.windowsdesktop.dll` | `5,512` | **`0`** | `248` (`114` fns) | `76` (`40` fns) |

**Key takeaway:** Those 17 sites inside the 14 V8 embedded builtins are the **only** `fprem`/`fprem1` instructions in the entire port (`AlanWake2.exe` + all 8 DLLs).

---

## 3. `emu` / x87 / Misaligned-Atomic Subtleties Found & Verified

1. **`emu` lacked x87 stack execution (`fld`/`fprem`/`fnstsw`/`fstp`):** `emu/simd_fp.rs` previously only stubbed `Fnstcw`/`Fldcw` and returned `false` (`RF_FAULT`) on `Fld`, `Fprem`, `Fnstsw`, and `Fstp`. `regfile_ext::exec_x87_ins` implements all four instructions against per-thread `TLS_X87` (`X87State`), without altering `#[repr(C)] struct Cpu` (whose field offsets are hardcoded via `offset_of!` in `emu/jit.rs`).
2. **80-bit internal significand required across `C2 = 1` `fprem` iterations:** When `d = exp(ST(0)) - exp(ST(1)) >= 64`, x87 reduces the exponent by **`N = 32 + (d & 31)`** (`32..=63`), sets `C2 = 1` (clearing `C0 = C1 = C3 = 0`), and keeps the 64-bit significand in `ST(0)`. Intermediate remainders can carry 54 significant bits before the final step (`d < 64`) reduces below `|ST(1)|` (e.g. `0x7feefdcba9876543 % 0x001123456789abcd` takes 34 steps). Rounding intermediate `ST(0)` values to 53-bit `f64` loses 1 ULP; `Fp80` (`u128` exact significand modulo) matches x86-64 Rosetta 2 hardware across **784 edge pairs + 100,000 random `f64` pairs with `0` mismatches** on `ST(0)`, `SW`, and iteration counts.
3. **NaN / `±0.0` / `±Inf` / `sNaN` / `SW` bits:**
   - `a = -15.0, b = 3.0` → `-0.0` (`0x8000_0000_0000_0000`, dividend sign preserved on zero remainder).
   - `a = ±Inf` or `b = ±0.0` → x87 QNaN Indefinite (`0xFFF8_0000_0000_0000`), `SW.IE = 1`.
   - `sNaN` on `fld m64` sets `SW.IE = 1` and quiets bit 62 of `Fp80`; two NaN operands propagate the larger 62-bit payload (ties broken toward `+NaN` `sign = a.sign & b.sign`, verified on Rosetta 2).
   - `fnstsw ax` writes 16-bit `SW` (including `TOP << 11` and sticky `IE`/`DE`/`UE`/`PE`) to `ax` with `last_bits[0] = 16`.
4. **`popfq` `last_bits[4]` parity:** `emu/simd_fp.rs` `Mnemonic::Popfq` advances `cpu.gpr[4]` by 8 without overwriting `cpu.last_bits[4]`. `regfile_ext::popfq` preserves `last_bits[4]` identically.
5. **Misaligned & boundary-straddling guest atomics:** Difftest fuzzing (`seed_regfile_trial`) seeds arbitrary unaligned guest addresses in `rdi`/`rdx`/`r8`/`r12` and near-canonical-boundary addresses (`0x7FFF_FFFF_FFFA`). `regfile_ext::{lock_cmpxchg_*, xchg_mem_*}`:
   - Check `is_valid_guest_range(addr, bytes)` (`addr >= 0x1_0000 && addr + bytes <= 0x8000_0000_0000`) **before** mutating memory, `rax`, or `RFLAGS`.
   - Use `AtomicU{8,16,32,64}` + `fence(SeqCst)` only when `(addr & (bytes - 1)) == 0`, falling back to `UNALIGNED_GUEST_ATOMIC_LOCK` + `read_unaligned`/`write_unaligned` + `fence(SeqCst)` when misaligned (never panicking on `AtomicU32` alignment checks).

---

## 4. Exact `g28` Hookup (~30 Lines Total)

### 4.1 `crates/aw2-game/src/emu/simd_fp.rs` (1-line oracle delegation)

At the `_ => return false` arm of `Cpu::exec_simd_fp_ext`:
```rust
_ => return super::regfile_ext::exec_x87_ins(cpu, mn, &ins),
```

### 4.2 `crates/aw2-game/src/emu/regfile.rs` (`Rf` methods + `run_oracle` x87/straddle guard)

Add thin `#[inline(always)]` delegations on `impl<'a> Rf<'a>`:
```rust
#[inline(always)]
pub unsafe fn fld_m64(&mut self, addr: u64) -> bool { super::regfile_ext::fld_m64(self.0, addr) }
#[inline(always)]
pub fn fprem(&mut self) { super::regfile_ext::fprem(self.0); }
#[inline(always)]
pub fn fnstsw_ax(&mut self) { super::regfile_ext::fnstsw_ax(self.0); }
#[inline(always)]
pub fn fstp_st(&mut self, idx: u8) { super::regfile_ext::fstp_st(self.0, idx); }
#[inline(always)]
pub unsafe fn fstp_m64(&mut self, addr: u64) -> bool { super::regfile_ext::fstp_m64(self.0, addr) }
#[inline(always)]
pub unsafe fn popfq(&mut self) -> bool { super::regfile_ext::popfq(self.0) }
#[inline(always)]
pub unsafe fn lock_cmpxchg(&mut self, addr: u64, src: u64, bits: u32) -> bool {
    match bits {
        8 => super::regfile_ext::lock_cmpxchg_8(self.0, addr, src),
        16 => super::regfile_ext::lock_cmpxchg_16(self.0, addr, src),
        32 => super::regfile_ext::lock_cmpxchg_32(self.0, addr, src),
        _ => super::regfile_ext::lock_cmpxchg_64(self.0, addr, src),
    }
}
#[inline(always)]
pub unsafe fn xchg_mem(&mut self, addr: u64, src: u64, bits: u32) -> Option<u64> {
    match bits {
        8 => super::regfile_ext::xchg_mem_8(self.0, addr, src),
        16 => super::regfile_ext::xchg_mem_16(self.0, addr, src),
        32 => super::regfile_ext::xchg_mem_32(self.0, addr, src),
        _ => super::regfile_ext::xchg_mem_64(self.0, addr, src),
    }
}
```
And in `run_oracle`:
- Call `super::regfile_ext::x87_reset();` at entry (and before each `RegfileFn` call in `diff_regfile`), snapshot `let pre_x87 = super::regfile_ext::x87_snapshot();` alongside `let pre = cpu.clone();`, and `super::regfile_ext::x87_restore(pre_x87);` on fault rollback.
- Use `super::regfile_ext::is_valid_guest_range(addr, bytes)` in `Rf::ld{8,16,32,64,128}` / `st{8,16,32,64,128}` (or check memory operand range in `run_oracle`) so multi-byte accesses straddling `0x8000_0000_0000` agree on `RF_FAULT`.

### 4.3 `tools/decomp_scale/v8_regfile.py` (~25 lines)

1. In `_parse_op`, recognize x87 stack registers before `_parse_int`:
   ```python
   if s.startswith("st(") and s.endswith(")"):
       return ("st", int(s[3:-1]))
   if s == "st":
       return ("st", 0)
   ```
2. Track `lock_va: int | None = None` across instructions in a basic block:
   ```python
   if mn == "lock" and not raw_ops:
       lock_va = ins_va
       continue
   fault_va = lock_va if lock_va is not None else ins_va
   lock_va = None
   ```
3. Add lowering arms in `_lower_builtin`:
   ```python
   elif mn in ("popfq", "popf"):
       lines.append(f"if !c.popfq() {{ return RegfileExit::fault(0x{fault_va:x}); }}")
   elif mn == "fld":
       op0 = parsed[0]
       if op0[0] != "mem" or op0[1] not in (0, 64):
           raise ValueError(f"+0x{rva:x}: unsupported fld {ops}")
       lines.append(f"if !c.fld_m64({op0[2]}) {{ return RegfileExit::fault(0x{fault_va:x}); }}")
   elif mn == "fprem":
       lines.append("c.fprem();")
   elif mn in ("fnstsw", "fstsw") and raw_ops == ["ax"]:
       lines.append("c.fnstsw_ax();")
   elif mn == "fstp":
       op0 = parsed[0]
       if op0[0] == "st":
           lines.append(f"c.fstp_st({op0[1]});")
       elif op0[0] == "mem" and op0[1] in (0, 64):
           lines.append(f"if !c.fstp_m64({op0[2]}) {{ return RegfileExit::fault(0x{fault_va:x}); }}")
       else:
           raise ValueError(f"+0x{rva:x}: unsupported fstp {ops}")
   elif mn == "cmpxchg":
       op0, op1 = parsed
       if op0[0] != "mem" or op1[0] != "gpr":
           raise ValueError(f"+0x{rva:x}: unsupported cmpxchg {ops}")
       bits = _op_width(op1)
       v1, tmp_id = _emit_read_scalar(op1, bits, fault_va, lines, tmp_id)
       lines.append(f"if !c.lock_cmpxchg({op0[2]}, {v1}, {bits}) {{ return RegfileExit::fault(0x{fault_va:x}); }}")
   elif mn == "xchg" and (parsed[0][0] == "mem" or parsed[1][0] == "mem"):
       mem_op, reg_op = (parsed[0], parsed[1]) if parsed[0][0] == "mem" else (parsed[1], parsed[0])
       bits = _op_width(reg_op)
       vr, tmp_id = _emit_read_scalar(reg_op, bits, fault_va, lines, tmp_id)
       old_t = f"xo{tmp_id}"; tmp_id += 1
       lines.append(f"let Some({old_t}) = c.xchg_mem({mem_op[2]}, {vr}, {bits}) else {{ return RegfileExit::fault(0x{fault_va:x}); }};")
       _emit_write_scalar(reg_op, old_t, bits, fault_va, lines)
   ```
4. Remove the three `v8_nonabi_trap_stub` / `v8_nonabi_x87` / `v8_nonabi_atomic` rejection checks in `lift_nonabi` → **`1,643 / 1,643` (`100.0%`) V8 embedded builtins lift cleanly**.
