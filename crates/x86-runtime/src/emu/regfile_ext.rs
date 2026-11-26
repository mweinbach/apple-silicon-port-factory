//! Extended register-file (`DF_REGFILE`) runtime primitives and reference oracle helpers for
//! the final 27 V8 embedded builtins (`g28`: 100% V8 embedded blob coverage):
//!
//! - **14 `v8_nonabi_x87` builtins** (`Float64Mod` / `Modulus*` / `Mod*Handler` / `StoreFastElementIC*`
//!   / `StringPrototypeSplit` / `TypedArrayPrototypeMap` / `StoreTypedElementJSAny_Uint8Clamped*`,
//!   17 sites total): exact 80-bit extended-precision x87 stack state + `fld_m64`, `fprem`,
//!   `fnstsw_ax`, `fstp_st`, `fstp_m64`, and `popfq` helpers.
//! - **8 `v8_nonabi_atomic` builtins** (`[538..545]`: `AtomicsStore`, `AtomicsExchange`,
//!   `AtomicsCompareExchange`, `AtomicsAdd`, `AtomicsSub`, `AtomicsAnd`, `AtomicsOr`, `AtomicsXor`):
//!   `lock_cmpxchg_{8,16,32,64}` and `xchg_mem_{8,16,32,64}` with `is_valid_guest_range` pre-check
//!   (returning `false` before mutating registers or memory), exact `update_flags_sub` + `rax`
//!   update on `cmpxchg`, untouched `RFLAGS` on `xchg`, and safe misaligned guest atomic fallback.
//! - **5 `v8_nonabi_trap_stub` builtins** (`117`, `119`, `581`, `582`, `627`): 1-instruction `int3`
//!   stubs returning `RegfileExit::fault(ins_va)` at entry (armed via `regfile::arm_skip` so
//!   `emu_run_full` steps `int3` once without re-entering).
//!
//! Keeping this module self-contained (with per-thread x87 register-stack state in TLS rather than
//! changing `#[repr(C)] struct Cpu`, whose field offsets are hardcoded in `emu/jit.rs`) avoids any
//! merge conflict with `g27` (`emu/regfile.rs` + `v8_regfile.py`).

use super::Cpu;
use iced_x86::{Instruction, Mnemonic, OpKind, Register};

/// Check that `[addr, addr + bytes)` lies strictly inside the valid user-space guest address window
/// `[0x1_0000, 0x8000_0000_0000)` without wrapping, including any misaligned access straddling the
/// upper canonical boundary `0x7FFF_FFFF_FFFF`.
#[inline(always)]
pub fn is_valid_guest_range(addr: u64, bytes: usize) -> bool {
    const LO: u64 = 0x1_0000;
    const SPAN: u64 = 0x8000_0000_0000 - LO; // 0x7FFF_FFFF_0000
    let len = bytes as u64;
    // Wrapping range checks: `1 <= len <= SPAN` and `LO <= addr <= LO + SPAN - len`.
    // When `bytes` is a compile-time constant (1/2/4/8/16), LLVM folds the first check away
    // and emits a single `sub` + unsigned `cmp`.
    len.wrapping_sub(1) < SPAN && addr.wrapping_sub(LO) <= SPAN - len
}

// =================================================================================================
// 80-bit extended-precision x87 FPU stack + status word (`fld m64`, `fprem`, `fnstsw ax`,
// `fstp st(i)`, `fstp m64`)
// =================================================================================================

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
enum Fp80Kind {
    #[default]
    Zero,
    Finite,
    Inf,
    Nan,
}

/// Exact 80-bit x87 extended-precision register value (`1` sign bit, unbiased exponent, `64`-bit
/// explicit-integer significand with bit 63 = integer bit `J`).
///
/// Retaining all 64 significand bits across `fprem` iterations is required for bit-exactness when
/// `exp(ST(0)) - exp(ST(1)) >= 64` (`C2 = 1` partial remainder loop): intermediate remainders can
/// carry up to 54 significant bits before the final step reduces below `|ST(1)|`, whereas rounding
/// intermediates to `f64` (53 bits) loses the low bit.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
struct Fp80 {
    kind: Fp80Kind,
    sign: u8,
    exp: i32,
    sig: u64,
}

impl Fp80 {
    /// Convert IEEE-754 `f64` bits to 80-bit extended precision, setting `SW.IE` (bit 0) and
    /// quieting on signaling NaN, or setting `SW.DE` (bit 1) and normalizing on subnormal input,
    /// matching `fld qword ptr [mem]` on x86-64 hardware.
    #[inline]
    fn from_f64_bits(bits: u64, sw: &mut u16) -> Self {
        let sign = (bits >> 63) as u8;
        let e = ((bits >> 52) & 0x7FF) as u32;
        let f = bits & 0x000F_FFFF_FFFF_FFFF;
        if e == 0x7FF {
            if f == 0 {
                Self { kind: Fp80Kind::Inf, sign, exp: 0, sig: 1u64 << 63 }
            } else {
                if (f & 0x0008_0000_0000_0000) == 0 {
                    *sw |= 0x0001; // IE: signaling NaN loaded by `fld m64`
                }
                Self {
                    kind: Fp80Kind::Nan,
                    sign,
                    exp: 0,
                    sig: (1u64 << 63) | (1u64 << 62) | (f << 11),
                }
            }
        } else if e == 0 {
            if f == 0 {
                Self { kind: Fp80Kind::Zero, sign, exp: 0, sig: 0 }
            } else {
                *sw |= 0x0002; // DE: denormal operand normalized on `fld m64`
                let clz = f.leading_zeros() as i32; // f in 1..2^52 -> clz in 12..=63
                Self {
                    kind: Fp80Kind::Finite,
                    sign,
                    exp: -1011 - clz,
                    sig: f << (clz as u32),
                }
            }
        } else {
            Self {
                kind: Fp80Kind::Finite,
                sign,
                exp: (e as i32) - 1023,
                sig: (1u64 << 63) | (f << 11),
            }
        }
    }

    fn is_signaling_nan(self) -> bool {
        self.kind == Fp80Kind::Nan && self.sig & (1 << 62) == 0
    }

    /// Round 80-bit extended precision back to IEEE-754 `f64` bits (round-to-nearest, ties-to-even),
    /// matching `fstp qword ptr [mem]` under the default x87 control word (`0x037F`).
    #[inline]
    fn to_f64_bits(self, sw: &mut u16) -> u64 {
        let s = (self.sign as u64) << 63;
        match self.kind {
            Fp80Kind::Zero => s,
            Fp80Kind::Inf => s | 0x7FF0_0000_0000_0000,
            Fp80Kind::Nan => {
                if self.is_signaling_nan() {
                    *sw |= 0x0001; // IE
                }
                let f = ((self.sig >> 11) & 0x000F_FFFF_FFFF_FFFF) | 0x0008_0000_0000_0000;
                s | 0x7FF0_0000_0000_0000 | f
            }
            Fp80Kind::Finite => {
                let mut exp = self.exp;
                let sig = self.sig;
                if exp > 1023 {
                    *sw |= 0x0028; // OE | PE
                    return s | 0x7FF0_0000_0000_0000;
                }
                if exp >= -1022 {
                    let mut hi = sig >> 11;
                    let rem = sig & 0x7FF;
                    if rem != 0 {
                        *sw |= 0x0020; // PE
                    }
                    if rem > 0x400 || (rem == 0x400 && (hi & 1) != 0) {
                        hi += 1;
                        if hi == (1u64 << 53) {
                            hi >>= 1;
                            exp += 1;
                            if exp > 1023 {
                                *sw |= 0x0008; // OE
                                return s | 0x7FF0_0000_0000_0000;
                            }
                        }
                    }
                    s | (((exp + 1023) as u64) << 52) | (hi & 0x000F_FFFF_FFFF_FFFF)
                } else {
                    let shr = -1011 - exp;
                    if shr >= 64 {
                        *sw |= 0x0030; // UE | PE
                        if shr == 64 && (sig & 0x7FFF_FFFF_FFFF_FFFF) != 0 {
                            return s | 1;
                        }
                        return s;
                    }
                    let shr_u = shr as u32;
                    let mut hi = sig >> shr_u;
                    let half = 1u64 << (shr_u - 1);
                    let rem = sig & ((1u64 << shr_u) - 1);
                    if rem != 0 {
                        *sw |= 0x0030; // UE | PE
                    }
                    if rem > half || (rem == half && (hi & 1) != 0) {
                        hi += 1;
                    }
                    s | hi
                }
            }
        }
    }

    /// Decode a raw x87 slot without quieting signaling NaNs. Each instruction applies its own exceptions.
    #[inline]
    fn from_f80_bytes(b: [u8; 10]) -> Self {
        let sig = u64::from_le_bytes(b[0..8].try_into().unwrap());
        let se = u16::from_le_bytes([b[8], b[9]]);
        let sign = (se >> 15) as u8;
        let e = (se & 0x7FFF) as i32;
        if e == 0x7FFF {
            if (sig & 0x7FFF_FFFF_FFFF_FFFF) == 0 {
                Self { kind: Fp80Kind::Inf, sign, exp: 0, sig: 1u64 << 63 }
            } else {
                Self { kind: Fp80Kind::Nan, sign, exp: 0, sig }
            }
        } else if e == 0 {
            if sig == 0 {
                Self { kind: Fp80Kind::Zero, sign, exp: 0, sig: 0 }
            } else {
                let clz = sig.leading_zeros() as i32;
                Self { kind: Fp80Kind::Finite, sign, exp: -16382 - clz, sig: sig << (clz as u32) }
            }
        } else if sig == 0 {
            Self { kind: Fp80Kind::Zero, sign, exp: 0, sig: 0 }
        } else {
            let clz = sig.leading_zeros() as i32;
            Self { kind: Fp80Kind::Finite, sign, exp: (e - 16383) - clz, sig: sig << (clz as u32) }
        }
    }

    /// Encode [`Fp80`] into 10 little-endian bytes (`m80fp`).
    #[inline]
    fn to_f80_bytes(self) -> [u8; 10] {
        let s = (self.sign as u16) << 15;
        let (sig, se) = match self.kind {
            Fp80Kind::Zero => (0u64, s),
            Fp80Kind::Inf => (1u64 << 63, s | 0x7FFF),
            Fp80Kind::Nan => (self.sig | (1u64 << 63), s | 0x7FFF),
            Fp80Kind::Finite => {
                let biased = self.exp + 16383;
                if biased >= 0x7FFF {
                    (1u64 << 63, s | 0x7FFF)
                } else if biased > 0 {
                    (self.sig, s | (biased as u16))
                } else {
                    let shr = (1 - biased) as u32;
                    let sub_sig = if shr >= 64 { 0 } else { self.sig >> shr };
                    (sub_sig, s)
                }
            }
        };
        let mut out = [0u8; 10];
        out[0..8].copy_from_slice(&sig.to_le_bytes());
        out[8..10].copy_from_slice(&se.to_le_bytes());
        out
    }

    /// Exact conversion of a signed 64-bit integer to [`Fp80`] (all 64 significand bits preserved).
    #[inline]
    fn from_i64(v: i64) -> Self {
        if v == 0 {
            return Self { kind: Fp80Kind::Zero, sign: 0, exp: 0, sig: 0 };
        }
        let sign = (v < 0) as u8;
        let mag = v.unsigned_abs();
        let clz = mag.leading_zeros() as i32;
        Self { kind: Fp80Kind::Finite, sign, exp: 63 - clz, sig: mag << (clz as u32) }
    }

    /// Convert [`Fp80`] to a signed `bits`-bit (`16`, `32`, or `64`) integer for `fist`/`fistp`/`fisttp`.
    /// Returns the Intel integer indefinite (`1 << (bits - 1)`) and sets `SW.IE` on NaN/Inf/overflow.
    #[inline]
    fn to_int_bits(self, bits: u32, truncate: bool, sw: &mut u16) -> u64 {
        let indef = 1u64 << (bits - 1);
        let mask = if bits == 64 { u64::MAX } else { (1u64 << bits) - 1 };
        match self.kind {
            Fp80Kind::Zero => 0,
            Fp80Kind::Inf | Fp80Kind::Nan => {
                *sw |= 0x0001; // IE
                indef
            }
            Fp80Kind::Finite => {
                let mag = if self.exp < 0 {
                    *sw |= 0x0020; // PE
                    if !truncate && self.exp == -1 && self.sig > (1u64 << 63) {
                        1u64
                    } else {
                        0u64
                    }
                } else if self.exp >= 63 {
                    if bits == 64 && self.sign != 0 && self.exp == 63 && self.sig == (1u64 << 63) {
                        return indef; // exact i64::MIN, no IE
                    }
                    *sw |= 0x0001; // IE
                    return indef;
                } else {
                    let shift = (63 - self.exp) as u32;
                    let mut m = self.sig >> shift;
                    let frac = self.sig & ((1u64 << shift) - 1);
                    if frac != 0 {
                        *sw |= 0x0020; // PE
                        if !truncate {
                            let half = 1u64 << (shift - 1);
                            if frac > half || (frac == half && (m & 1) != 0) {
                                m += 1;
                            }
                        }
                    }
                    m
                };
                let max_pos = (1u64 << (bits - 1)) - 1;
                let max_neg = 1u64 << (bits - 1);
                if self.sign == 0 {
                    if mag > max_pos {
                        *sw |= 0x0001; // IE
                        indef
                    } else {
                        mag & mask
                    }
                } else if mag > max_neg {
                    *sw |= 0x0001; // IE
                    indef
                } else {
                    0u64.wrapping_sub(mag) & mask
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
enum X87Source {
    St(u8),
    Value(f64),
}

/// Architectural x87 register-stack state (`ST(0)..ST(7)` around `top`, `SW` status word, `tag`
/// mask where bit `phys` is `1` iff physical slot `phys` is non-empty).
///
/// `#[repr(C)]`: translated ARM64 MMX/x87 code addresses `regs`, `top`, and `valid_mask` by
/// compile-time `offset_of!` via `Cpu::jx.x87_state`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct X87State {
    // Store the architectural bytes so state transfers preserve SNaNs, unsupported encodings,
    // and the contents of empty slots without an arithmetic conversion.
    pub(super) regs: [[u8; 10]; 8],
    pub(super) cw: u16,
    pub(super) fop: u16,
    pub(super) fip: u64,
    pub(super) fdp: u64,
    pub(super) top: u8,
    pub(super) sw: u16,
    pub(super) valid_mask: u8,
}

impl Default for X87State {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl X87State {
    #[inline]
    pub const fn new() -> Self {
        Self {
            regs: [[0; 10]; 8],
            cw: 0x037f,
            fop: 0,
            fip: 0,
            fdp: 0,
            top: 0,
            sw: 0,
            valid_mask: 0,
        }
    }

    #[inline(always)]
    pub(super) fn phys(&self, i: u8) -> usize {
        (self.top.wrapping_add(i) & 7) as usize
    }

    fn value(&self, slot: usize) -> Fp80 {
        Fp80::from_f80_bytes(self.regs[slot])
    }

    fn raw_denormal(&self, slot: usize) -> bool {
        let raw = &self.regs[slot];
        self.valid_mask & (1 << slot) != 0 && raw[8] == 0 && raw[9] & 0x7f == 0 && raw[..8] != [0; 8]
    }

    #[inline(always)]
    fn sync_top(&mut self) {
        self.sw = (self.sw & !(7u16 << 11)) | (((self.top & 7) as u16) << 11);
    }

    #[inline]
    pub fn status_word(&self) -> u16 {
        (self.sw & !(7u16 << 11)) | (((self.top & 7) as u16) << 11)
    }

    /// Read `ST(idx)` as IEEE-754 f64 bits without mutating the x87 stack or status word.
    #[inline]
    pub fn peek_f64_bits(&self, idx: u8) -> Option<u64> {
        let p = self.phys(idx);
        if (self.valid_mask & (1 << p)) == 0 {
            return None;
        }
        let mut sw = self.sw;
        Some(self.value(p).to_f64_bits(&mut sw))
    }

    #[inline]
    pub fn push_f64_bits(&mut self, bits: u64) {
        let mut sw = self.sw & !0x0200; // C1 cleared on non-overflow push
        let val = Fp80::from_f64_bits(bits, &mut sw);
        self.push_fp80(val, sw);
    }

    #[inline]
    pub fn pop_to_st(&mut self, idx: u8) {
        let p0 = self.phys(0);
        let pi = self.phys(idx);
        let v = self.regs[p0];
        if idx != 0 {
            self.regs[pi] = v;
            self.valid_mask |= 1 << pi;
        }
        self.valid_mask &= !(1 << p0);
        self.top = self.top.wrapping_add(1) & 7;
        self.sw &= !0x0200; // C1 = 0
        self.sync_top();
    }

    #[inline]
    pub fn pop_f64_bits(&mut self) -> u64 {
        let p0 = self.phys(0);
        let mut sw = self.sw & !0x0200; // C1 = 0
        let out = if (self.valid_mask & (1 << p0)) == 0 {
            // Stack underflow (`SF | IE`): store QNaN Indefinite.
            sw |= 0x0041;
            0xFFF8_0000_0000_0000
        } else {
            let v = self.value(p0);
            self.valid_mask &= !(1 << p0);
            v.to_f64_bits(&mut sw)
        };
        self.top = self.top.wrapping_add(1) & 7;
        self.sw = sw;
        self.sync_top();
        out
    }

    /// Execute x87 `fprem` (`D9 F8`): `ST(0) <- remainder(ST(0), ST(1))` truncated toward zero,
    /// updating `C0` (`Q2`, bit 8), `C1` (`Q0`, bit 9), `C2` (incomplete flag, bit 10), and `C3`
    /// (`Q1`, bit 14) in `SW`.
    ///
    /// Matches Intel SDM Vol. 2A `FPREM` and x86-64 hardware / Rosetta 2 bit-for-bit:
    /// - `d = exp(ST(0)) - exp(ST(1)) < 64`: complete reduction in one step (`C2 = 0`,
    ///   `(C0, C3, C1) = (Q2, Q1, Q0)`, zero remainder preserves `ST(0)` sign).
    /// - `d >= 64`: partial reduction by `N = 32 + (d & 31)` (`32..=63`), setting `C2 = 1` and
    ///   clearing `C0 = C1 = C3 = 0` so `fnstsw ax; shr eax, 8; push rax; popfq; jp loop`
    ///   (`PF = C2`) repeats until `d < 64`.
    #[inline]
    pub fn fprem(&mut self) {
        const C_MASK: u16 = (1 << 8) | (1 << 9) | (1 << 10) | (1 << 14);
        self.sw &= !C_MASK;
        let p0 = self.phys(0);
        let p1 = self.phys(1);
        if (self.valid_mask & (1 << p0)) == 0 || (self.valid_mask & (1 << p1)) == 0 {
            self.sw |= 0x0041; // SF | IE (stack underflow)
            self.regs[p0] = Fp80 {
                kind: Fp80Kind::Nan,
                sign: 1,
                exp: 0,
                sig: 0xC000_0000_0000_0000,
            }.to_f80_bytes();
            self.valid_mask |= 1 << p0;
            self.sync_top();
            return;
        }
        let a = self.value(p0);
        let b = self.value(p1);

        if a.kind == Fp80Kind::Nan || b.kind == Fp80Kind::Nan {
            if a.is_signaling_nan() || b.is_signaling_nan() {
                self.sw |= 0x0001; // IE before quieting the selected NaN.
            }
            let mut res = if a.kind == Fp80Kind::Nan && b.kind == Fp80Kind::Nan {
                let pa = a.sig & 0x3FFF_FFFF_FFFF_FFFF;
                let pb = b.sig & 0x3FFF_FFFF_FFFF_FFFF;
                if pa > pb {
                    a
                } else if pb > pa {
                    b
                } else {
                    Fp80 { sign: a.sign & b.sign, ..a }
                }
            } else if a.kind == Fp80Kind::Nan {
                a
            } else {
                b
            };
            res.sig |= 1u64 << 62;
            self.regs[p0] = res.to_f80_bytes();
            self.sync_top();
            return;
        }

        if a.kind == Fp80Kind::Inf || b.kind == Fp80Kind::Zero {
            self.sw |= 0x0001; // IE -> QNaN Indefinite (`0xFFF8_0000_0000_0000` in f64)
            self.regs[p0] = Fp80 {
                kind: Fp80Kind::Nan,
                sign: 1,
                exp: 0,
                sig: 0xC000_0000_0000_0000,
            }.to_f80_bytes();
            self.sync_top();
            return;
        }

        // A NaN or invalid remainder takes priority over a denormal operand.
        if self.raw_denormal(p0) || self.raw_denormal(p1) {
            self.sw |= 0x0002;
        }
        if a.kind == Fp80Kind::Zero || b.kind == Fp80Kind::Inf {
            // Exact remainder is `a` (including `-0.0` sign); quotient Q = 0 -> C0=C1=C2=C3=0.
            self.sync_top();
            return;
        }

        let d = a.exp - b.exp;
        if d < 0 {
            // |a| < |b|: Q = 0, ST(0) unchanged (sign preserved), C0=C1=C2=C3=0.
            self.sync_top();
            return;
        }

        if d < 64 {
            let num = (a.sig as u128) << (d as u32);
            let den = b.sig as u128;
            let q = num / den;
            let rem = (num % den) as u64;
            let q3 = (q & 7) as u32;
            if (q3 & 1) != 0 {
                self.sw |= 1 << 9; // C1 = Q0
            }
            if (q3 & 2) != 0 {
                self.sw |= 1 << 14; // C3 = Q1
            }
            if (q3 & 4) != 0 {
                self.sw |= 1 << 8; // C0 = Q2
            }
            if rem == 0 {
                self.regs[p0] = Fp80 { kind: Fp80Kind::Zero, sign: a.sign, exp: 0, sig: 0 }.to_f80_bytes();
            } else {
                let clz = rem.leading_zeros() as i32;
                self.regs[p0] = Fp80 {
                    kind: Fp80Kind::Finite,
                    sign: a.sign,
                    exp: b.exp - clz,
                    sig: rem << (clz as u32),
                }.to_f80_bytes();
            }
        } else {
            let n = 32 + (d & 31);
            let num = (a.sig as u128) << (n as u32);
            let rem = (num % (b.sig as u128)) as u64;
            self.sw |= 1 << 10; // C2 = 1 (incomplete reduction; PF=1 after shr eax,8; push rax; popfq)
            if rem == 0 {
                self.regs[p0] = Fp80 { kind: Fp80Kind::Zero, sign: a.sign, exp: 0, sig: 0 }.to_f80_bytes();
            } else {
                let clz = rem.leading_zeros() as i32;
                self.regs[p0] = Fp80 {
                    kind: Fp80Kind::Finite,
                    sign: a.sign,
                    exp: (a.exp - n) - clz,
                    sig: rem << (clz as u32),
                }.to_f80_bytes();
            }
        }
        self.sync_top();
    }

    #[inline]
    fn push_fp80(&mut self, val: Fp80, extra_sw: u16) {
        self.push_bytes(val.to_f80_bytes(), extra_sw);
    }

    fn push_bytes(&mut self, val: [u8; 10], extra_sw: u16) {
        let mut sw = (self.sw & !0x0200) | extra_sw;
        let new_top = self.top.wrapping_sub(1) & 7;
        let p = new_top as usize;
        if (self.valid_mask & (1 << p)) != 0 {
            sw |= 0x0241; // SF | IE | C1
            self.regs[p] = Fp80 {
                kind: Fp80Kind::Nan,
                sign: 1,
                exp: 0,
                sig: 0xC000_0000_0000_0000,
            }.to_f80_bytes();
        } else {
            self.regs[p] = val;
            self.valid_mask |= 1 << p;
        }
        self.top = new_top;
        self.sw = sw;
        self.sync_top();
    }

    #[inline]
    pub fn pop_discard(&mut self) {
        let p0 = self.phys(0);
        self.valid_mask &= !(1 << p0);
        self.top = self.top.wrapping_add(1) & 7;
        self.sw &= !0x0200;
        self.sync_top();
    }

    #[inline]
    pub fn clear_exceptions(&mut self) {
        self.sw &= !0x80FF;
        self.sync_top();
    }

    #[inline]
    pub fn free_st(&mut self, idx: u8) {
        let pi = self.phys(idx & 7);
        self.valid_mask &= !(1 << pi);
    }

    #[inline]
    pub fn freep_st(&mut self, idx: u8) {
        let pi = self.phys(idx & 7);
        self.valid_mask &= !(1 << pi);
        self.top = self.top.wrapping_add(1) & 7;
        self.sync_top();
    }

    #[inline]
    pub fn fxam(&mut self) {
        const C_MASK: u16 = (1 << 8) | (1 << 9) | (1 << 10) | (1 << 14);
        self.sw &= !C_MASK;
        let p0 = self.phys(0);
        if (self.valid_mask & (1 << p0)) == 0 {
            // Empty: C3=1, C2=0, C0=1, C1=0
            self.sw |= (1 << 14) | (1 << 8);
            self.sync_top();
            return;
        }
        let v = self.value(p0);
        if v.sign != 0 {
            self.sw |= 1 << 9; // C1 = sign bit
        }
        match v.kind {
            Fp80Kind::Nan => self.sw |= 1 << 8,                  // C3=0, C2=0, C0=1
            Fp80Kind::Finite if self.raw_denormal(p0) => self.sw |= (1 << 14) | (1 << 10),
            Fp80Kind::Finite => self.sw |= 1 << 10,              // C3=0, C2=1, C0=0
            Fp80Kind::Inf => self.sw |= (1 << 10) | (1 << 8),    // C3=0, C2=1, C0=1
            Fp80Kind::Zero => self.sw |= 1 << 14,                // C3=1, C2=0, C0=0
        }
        self.sync_top();
    }

    #[inline]
    pub fn fxch(&mut self, idx: u8) {
        let p0 = self.phys(0);
        let pi = self.phys(idx & 7);
        self.regs.swap(p0, pi);
        let v0 = (self.valid_mask >> p0) & 1;
        let vi = (self.valid_mask >> pi) & 1;
        self.valid_mask &= !((1 << p0) | (1 << pi));
        self.valid_mask |= (vi << p0) | (v0 << pi);
        self.sw &= !0x0200;
        self.sync_top();
    }

    /// Record DE from the encoded operands before the arithmetic backend narrows them to f64.
    /// Empty slots, unsupported encodings and NaNs take priority, as do zero divisors and
    /// invalid square roots/remainders. Previously raised exceptions stay sticky.
    fn arithmetic_inputs(&mut self, mn: Mnemonic, dst: u8, src: X87Source) -> (f64, f64) {
        use Mnemonic::*;
        let left = self.phys(dst);
        let right = match src {
            X87Source::St(i) => Some(self.phys(i)),
            X87Source::Value(_) => None,
        };
        if self.raw_denormal(left) || right.is_some_and(|i| self.raw_denormal(i)) {
            let class = |slot: usize| {
                let raw = &self.regs[slot];
                let unsupported = (raw[8] != 0 || raw[9] & 0x7f != 0) && raw[7] & 0x80 == 0;
                (self.valid_mask & (1 << slot) != 0 && !unsupported).then(|| self.value(slot).kind)
            };
            let rhs = match src {
                X87Source::St(i) => class(self.phys(i)),
                X87Source::Value(v) => Some(match v.classify() {
                    std::num::FpCategory::Zero => Fp80Kind::Zero,
                    std::num::FpCategory::Infinite => Fp80Kind::Inf,
                    std::num::FpCategory::Nan => Fp80Kind::Nan,
                    _ => Fp80Kind::Finite,
                }),
            };
            if let (Some(a), Some(b)) = (class(left), rhs) {
                let suppressed = a == Fp80Kind::Nan || b == Fp80Kind::Nan || match mn {
                    Fdiv | Fdivp | Fidiv => b == Fp80Kind::Zero,
                    Fdivr | Fdivrp | Fidivr => a == Fp80Kind::Zero,
                    Fprem1 => a == Fp80Kind::Inf || b == Fp80Kind::Zero,
                    Fsqrt => self.regs[left][9] & 0x80 != 0,
                    _ => false,
                };
                if !suppressed {
                    self.sw |= 0x0002;
                }
            }
        }
        let a = self.read_st_f64(dst);
        let b = match src {
            X87Source::St(i) => self.read_st_f64(i),
            X87Source::Value(v) => v,
        };
        (a, b)
    }

    #[inline]
    fn read_st_f64(&mut self, idx: u8) -> f64 {
        let pi = self.phys(idx & 7);
        if (self.valid_mask & (1 << pi)) == 0 {
            self.sw |= 0x0041; // SF | IE
            return f64::from_bits(0xFFF8_0000_0000_0000);
        }
        let mut sw = self.sw;
        let bits = self.value(pi).to_f64_bits(&mut sw);
        self.sw = sw;
        f64::from_bits(bits)
    }

    #[inline]
    fn write_st_f64(&mut self, idx: u8, val: f64) {
        let pi = self.phys(idx & 7);
        let mut sw = self.sw;
        self.regs[pi] = Fp80::from_f64_bits(val.to_bits(), &mut sw).to_f80_bytes();
        self.valid_mask |= 1 << pi;
        self.sw = sw;
        self.sync_top();
    }

    #[inline(always)]
    fn compare_sw(&mut self, a: f64, b: f64) {
        const C_MASK: u16 = (1 << 8) | (1 << 9) | (1 << 10) | (1 << 14);
        let c0 = (!(a >= b) as u16) << 8;               // C0=1 on < or Unordered
        let c2 = ((!(a <= b) & !(a > b)) as u16) << 10; // C2=1 on Unordered
        let c3 = ((!(a < b) & !(a > b)) as u16) << 14;  // C3=1 on == or Unordered
        self.sw = (self.sw & !C_MASK) | c0 | c2 | c3;
        self.sync_top();
    }
}

std::thread_local! {
    static TLS_X87: core::cell::UnsafeCell<X87State> =
        const { core::cell::UnsafeCell::new(X87State::new()) };
}

/// Scoped immutable access to this thread's x87 state without copying `X87State` or paying
/// `RefCell` runtime borrow checks.
///
/// Safety invariant for callers: `f` must be a leaf closure that never re-enters `with_x87_mut`
/// and never invokes guest callbacks.
#[inline(always)]
fn with_x87_ref<R>(f: impl FnOnce(&X87State) -> R) -> R {
    TLS_X87.with(|cell| unsafe { f(&*cell.get()) })
}

/// Scoped exclusive mutable access to this thread's x87 state in place (avoids 136-byte copies
/// on every x87 instruction).
///
/// Safety invariant for callers: `f` must be a leaf closure that never re-enters `with_x87_ref` /
/// `with_x87_mut` and never invokes guest callbacks or memory helpers that could fault/dispatch.
#[inline(always)]
pub(super) fn with_x87_mut<R>(f: impl FnOnce(&mut X87State) -> R) -> R {
    TLS_X87.with(|cell| unsafe { f(&mut *cell.get()) })
}

/// Stable pointer to this thread's architectural [`X87State`] (`TLS_X87`), exposed to translated
/// ARM64/AOT code via `Cpu::jx.x87_state` for direct zero-relocation MMX/x87 aliasing.
#[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]
#[inline(always)]
pub(super) fn x87_state_ptr() -> usize {
    TLS_X87.with(|cell| cell.get() as usize)
}

/// Retire a decoded x87 data instruction. FOP is the low 3 bits of the escape
/// opcode followed by ModRM, independently of legacy, address-size and REX prefixes.
pub(super) unsafe fn record_x87_environment(cpu: &Cpu, ins: &Instruction) {
    use iced_x86::CpuidFeature::{CMOV, FPU, FPU287, FPU387};
    use Mnemonic::*;
    if !ins.cpuid_features().iter().any(|f| matches!(f, FPU | FPU287 | FPU387 | CMOV))
        || matches!(
            ins.mnemonic(),
            Fldcw | Fnstcw | Fstcw | Fninit | Finit | Fnclex | Fclex | Fnstsw | Fstsw | Wait
        )
        || (super::precise_faults() && super::FAULT_PENDING.with(|p| p.get().is_some()))
    {
        return;
    }
    let bytes = core::slice::from_raw_parts(super::to_mapped_addr(ins.ip()) as *const u8, ins.len());
    // Check the opcode itself: CMOV is also the CPUID requirement for integer CMOVcc.
    let mut bytes = bytes
        .iter()
        .copied()
        .skip_while(|b| matches!(b, 0x26|0x2e|0x36|0x3e|0x64..=0x67|0x40..=0x4f|0xf2|0xf3));
    let (Some(op @ 0xd8..=0xdf), Some(modrm)) = (bytes.next(), bytes.next()) else {
        return;
    };
    let data = (0..ins.op_count()).find(|&i| ins.op_kind(i) == OpKind::Memory).map(|_| cpu.eff_addr(ins));
    with_x87_mut(|st| {
        st.fip = ins.ip();
        st.fop = (u16::from(op & 7) << 8) | u16::from(modrm);
        if let Some(addr) = data {
            st.fdp = addr;
        }
    });
}

/// Snapshot this thread's x87 state (used by `run_oracle` rollback and differential tests).
#[inline]
pub fn x87_snapshot() -> X87State {
    with_x87_ref(|st| *st)
}

/// Restore this thread's x87 state.
#[inline]
pub fn x87_restore(st: X87State) {
    with_x87_mut(|cur| *cur = st);
}

/// Reset this thread's x87 state (`fninit` equivalent: empty stack, `TOP = 0`, `SW = 0`).
#[inline]
pub fn x87_reset() {
    with_x87_mut(|cur| *cur = X87State::new());
}

/// Address of this thread's `X87State::cw` field for inline `fldcw`/`fnstcw`/`fstcw`
/// in translated ARM64 code.
#[cfg(target_arch = "aarch64")]
#[inline]
pub(super) fn x87_cw_ptr() -> usize {
    TLS_X87.with(|cell| unsafe { core::ptr::addr_of_mut!((*cell.get()).cw) as usize })
}

// =================================================================================================
// Clean `#[inline]` helpers on `&mut Cpu` for `DF_REGFILE` (`Rf` delegation) and `emu`
// =================================================================================================

#[inline(always)]
unsafe fn guest_ld64(addr: u64) -> Option<u64> {
    if !is_valid_guest_range(addr, 8) {
        return None;
    }
    let p = addr as usize;
    Some(if super::mem::tso() {
        super::mem::load_acq(p, 64)
    } else {
        crate::exe::rd::<u64>(p)
    })
}

#[inline(always)]
unsafe fn guest_st64(addr: u64, v: u64) -> bool {
    if !is_valid_guest_range(addr, 8) {
        return false;
    }
    let p = addr as usize;
    if super::mem::tso() {
        super::mem::store_rel(p, 64, v);
    } else {
        crate::exe::wr::<u64>(p, v);
    }
    true
}

/// `fld qword ptr [addr]` (`DD /0`): validate `[addr, addr + 8)`, load `f64`, and push onto the
/// x87 stack as 80-bit extended precision. Returns `false` (leaving `Cpu` and x87 untouched) if
/// `[addr, addr + 8)` is outside valid guest memory.
#[inline]
pub(crate) unsafe fn fld_m64(_cpu: &mut Cpu, addr: u64) -> bool {
    let Some(bits) = guest_ld64(addr) else {
        return false;
    };
    with_x87_mut(|st| st.push_f64_bits(bits));
    true
}

/// `fprem` (`D9 F8`): compute exact IEEE-754 / Intel x87 partial remainder `ST(0) % ST(1)` (truncated
/// toward zero) and update `SW` (`C0`, `C1`, `C2`, `C3`).
#[inline]
pub(crate) fn fprem(_cpu: &mut Cpu) {
    with_x87_mut(|st| st.fprem());
}

/// `fnstsw ax` / `fstsw ax` (`DF E0`): store 16-bit x87 status word into `ax` (`last_bits[0] = 16`,
/// preserving bits `63:16` of `rax`).
#[inline]
pub(crate) fn fnstsw_ax(cpu: &mut Cpu) {
    let sw = with_x87_ref(|st| st.status_word()) as u64;
    cpu.gpr[0] = (cpu.gpr[0] & !0xFFFF) | (sw & 0xFFFF);
    cpu.last_bits[0] = 16;
}

/// `fstp st(idx)` (`DD D8+i`, e.g. `DD D9` = `fstp st(1)`): copy `ST(0)` to `ST(idx)` and pop the
/// x87 register stack.
#[inline]
pub(crate) fn fstp_st(_cpu: &mut Cpu, idx: u8) {
    with_x87_mut(|st| st.pop_to_st(idx & 7));
}

/// `fstp qword ptr [addr]` (`DD /3`): validate `[addr, addr + 8)` **before** popping the x87 stack,
/// round `ST(0)` to `f64`, store to `addr`, and pop `ST(0)`. On invalid guest address, returns
/// `false` with both `Cpu` and the x87 stack completely untouched.
#[inline]
pub(crate) unsafe fn fstp_m64(_cpu: &mut Cpu, addr: u64) -> bool {
    if !is_valid_guest_range(addr, 8) {
        return false;
    }
    let bits = with_x87_mut(|st| st.pop_f64_bits());
    guest_st64(addr, bits)
}

/// `popfq` (`9D`, used in all 17 V8 `fprem` loops via `push rax; popfq; jp loop`): pop 64-bit
/// flags image from `[rsp]`, advance `rsp` by 8, and update `CF`, `PF`, `ZF`, `SF`, `DF`, `OF`
/// identically to `Cpu::exec_simd_fp_ext(Mnemonic::Popfq)`. Returns `false` without mutating
/// `rsp` or `RFLAGS` if `[rsp, rsp + 8)` is outside valid guest memory.
///
/// Subtlety: `emu/simd_fp.rs` `Popfq` updates `cpu.gpr[4]` without touching `cpu.last_bits[4]`
/// (just like `regfile::Rf::push64` / `set_rsp`), so preserving `last_bits[4]` matches `emu::Cpu`
/// bit-for-bit.
#[inline]
pub(crate) unsafe fn popfq(cpu: &mut Cpu) -> bool {
    let sp = cpu.gpr[4];
    let Some(fl) = guest_ld64(sp) else {
        return false;
    };
    cpu.gpr[4] = sp.wrapping_add(8);
    cpu.cf = (fl & 0x001) != 0;
    cpu.set_pf((fl & 0x004) != 0);
    cpu.zf = (fl & 0x040) != 0;
    cpu.sf = (fl & 0x080) != 0;
    cpu.df = (fl & 0x400) != 0;
    cpu.of = (fl & 0x800) != 0;
    true
}

// =================================================================================================
// Guest GPR partial-write helper matching `Cpu::set_reg` / `regfile::Rf::w{8,16,32,64}`
// =================================================================================================

/// Write `val` to guest GPR `reg` (`0..16`) with `bits` (`8`, `16`, `32`, `64`) and `hi8`
/// (`true` for `ah`/`ch`/`dh`/`bh`), updating `last_bits[reg]` identically to `Cpu::set_reg`.
#[inline(always)]
pub(crate) fn write_gpr_sized(cpu: &mut Cpu, reg: usize, bits: u32, hi8: bool, val: u64) {
    match (bits, hi8) {
        (8, true) => {
            cpu.gpr[reg] = (cpu.gpr[reg] & !0xFF00) | ((val & 0xFF) << 8);
            cpu.last_bits[reg] = 64;
        }
        (8, false) => {
            cpu.gpr[reg] = (cpu.gpr[reg] & !0xFF) | (val & 0xFF);
            cpu.last_bits[reg] = 8;
        }
        (16, _) => {
            cpu.gpr[reg] = (cpu.gpr[reg] & !0xFFFF) | (val & 0xFFFF);
            cpu.last_bits[reg] = 16;
        }
        (32, _) => {
            cpu.gpr[reg] = (val as u32) as u64;
            cpu.last_bits[reg] = 64;
        }
        _ => {
            cpu.gpr[reg] = val;
            cpu.last_bits[reg] = 64;
        }
    }
}

// =================================================================================================
// Safe aligned + misaligned guest atomic primitives (`lock cmpxchg` and `xchg [mem], reg`)
// =================================================================================================

/// Shared global spin/mutex guard serializing misaligned guest atomic operations with each other
/// (never dereferencing `AtomicU16`/`AtomicU32`/`AtomicU64` on an unaligned pointer, which is UB
/// and panics in debug builds).
static UNALIGNED_GUEST_ATOMIC_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[cold]
#[inline(never)]
unsafe fn lock_cmpxchg_unaligned<T: Copy + Eq>(p: usize, expected: T, desired: T) -> (bool, T) {
    let _g = UNALIGNED_GUEST_ATOMIC_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let cur = core::ptr::read_unaligned(p as *const T);
    let hit = cur == expected;
    if hit {
        core::ptr::write_unaligned(p as *mut T, desired);
    }
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    (hit, cur)
}

#[cold]
#[inline(never)]
unsafe fn xchg_mem_unaligned<T: Copy>(p: usize, desired: T) -> T {
    let _g = UNALIGNED_GUEST_ATOMIC_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let prev = core::ptr::read_unaligned(p as *const T);
    core::ptr::write_unaligned(p as *mut T, desired);
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    prev
}

macro_rules! impl_lock_cmpxchg {
    ($fn_name:ident, $ty:ty, $atomic:ident, $bytes:expr, $bits:expr) => {
        /// `lock cmpxchg [addr], desired`:
        /// 1. Validates `is_valid_guest_range(addr, $bytes)` **before** touching memory, `rax`, or
        ///    `RFLAGS` (returning `false` so the caller can return `RegfileExit::fault(ins_va)`).
        /// 2. Compares low `$bits` of `rax` against `[addr]` atomically (aligned `SeqCst` hardware
        ///    CAS + `SeqCst` full fence, or cold mutex-serialized `read_unaligned`/`write_unaligned`
        ///    + `SeqCst` fence when `addr` is misaligned).
        /// 3. Updates `CF`/`PF`/`ZF`/`SF`/`OF` via `cpu.update_flags_sub(expected, actual, $bits)`.
        /// 4. On hit (`actual == expected`), sets `ZF = true` and leaves `rax` + `last_bits[0]`
        ///    untouched; on miss, sets `ZF = false` and writes `actual` into `al`/`ax`/`eax`/`rax`
        ///    (updating `last_bits[0]` to `8`/`16`/`64`, zero-extending `eax` into `rax` at 32-bit).
        #[inline]
        pub(crate) unsafe fn $fn_name(cpu: &mut Cpu, addr: u64, desired: u64) -> bool {
            use core::sync::atomic::{fence, $atomic, Ordering};
            if !is_valid_guest_range(addr, $bytes) {
                return false;
            }
            let expected = cpu.gpr[0] as $ty;
            let des = desired as $ty;
            let p = addr as usize;
            let (ok, actual) = if $bytes == 1 || (p & ($bytes - 1)) == 0 {
                let a = &*(p as *const $atomic);
                let res = match a.compare_exchange(expected, des, Ordering::SeqCst, Ordering::SeqCst) {
                    Ok(v) => (true, v),
                    Err(v) => (false, v),
                };
                fence(Ordering::SeqCst);
                res
            } else {
                lock_cmpxchg_unaligned::<$ty>(p, expected, des)
            };
            let _ = cpu.update_flags_sub(expected as u64, actual as u64, $bits);
            if ok {
                cpu.zf = true;
            } else {
                cpu.zf = false;
                write_gpr_sized(cpu, 0, $bits, false, actual as u64);
            }
            true
        }
    };
}

impl_lock_cmpxchg!(lock_cmpxchg_8, u8, AtomicU8, 1, 8);
impl_lock_cmpxchg!(lock_cmpxchg_16, u16, AtomicU16, 2, 16);
impl_lock_cmpxchg!(lock_cmpxchg_32, u32, AtomicU32, 4, 32);
impl_lock_cmpxchg!(lock_cmpxchg_64, u64, AtomicU64, 8, 64);

macro_rules! impl_xchg_mem {
    ($fn_name:ident, $ty:ty, $atomic:ident, $bytes:expr) => {
        /// Implicitly-locked `xchg [addr], src_val` (`86 /r` or `87 /r` with memory operand):
        /// 1. Validates `is_valid_guest_range(addr, $bytes)` **before** mutating memory or GPRs,
        ///    returning `None` on fault.
        /// 2. Atomically swaps `src_val as $ty` into `[addr]` (aligned `SeqCst` `swap` + `SeqCst`
        ///    fence, or cold mutex-serialized `read_unaligned`/`write_unaligned` + `SeqCst` fence
        ///    when `addr` is misaligned).
        /// 3. Leaves all `RFLAGS` (`cf`, `pf`, `zf`, `sf`, `df`, `of`) completely untouched and
        ///    returns `Some(old_mem as u64)` for writing back to the register operand.
        #[inline]
        pub(crate) unsafe fn $fn_name(_cpu: &mut Cpu, addr: u64, src_val: u64) -> Option<u64> {
            use core::sync::atomic::{fence, $atomic, Ordering};
            if !is_valid_guest_range(addr, $bytes) {
                return None;
            }
            let v = src_val as $ty;
            let p = addr as usize;
            let old = if $bytes == 1 || (p & ($bytes - 1)) == 0 {
                let a = &*(p as *const $atomic);
                let prev = a.swap(v, Ordering::SeqCst);
                fence(Ordering::SeqCst);
                prev
            } else {
                xchg_mem_unaligned::<$ty>(p, v)
            };
            Some(old as u64)
        }
    };
}

impl_xchg_mem!(xchg_mem_8, u8, AtomicU8, 1);
impl_xchg_mem!(xchg_mem_16, u16, AtomicU16, 2);
impl_xchg_mem!(xchg_mem_32, u32, AtomicU32, 4);
impl_xchg_mem!(xchg_mem_64, u64, AtomicU64, 8);

// =================================================================================================
// `iced-x86` reference instruction handler for x87 (`Fld`, `Fprem`, `Fnstsw`, `Fstp`) so
// `emu::Cpu` / `regfile::run_oracle` can execute the 14 V8 x87 builtins directly as the oracle
// =================================================================================================

const _: () = {
    const S0: u32 = Register::ST0 as u32;
    assert!(Register::ST1 as u32 == S0 + 1);
    assert!(Register::ST2 as u32 == S0 + 2);
    assert!(Register::ST3 as u32 == S0 + 3);
    assert!(Register::ST4 as u32 == S0 + 4);
    assert!(Register::ST5 as u32 == S0 + 5);
    assert!(Register::ST6 as u32 == S0 + 6);
    assert!(Register::ST7 as u32 == S0 + 7);
};

#[inline(always)]
fn st_reg_index(r: Register) -> Option<u8> {
    let idx = (r as u32).wrapping_sub(Register::ST0 as u32);
    if idx < 8 { Some(idx as u8) } else { None }
}

/// Execute an x87 instruction on `cpu` using the interpreter's `read_mem`/`write_mem` fault
/// pipeline (`FAULT_PENDING` in precise mode).
///
/// Covers the full practical x87 surface observed across Windows shipping DLLs and CRT runtimes:
/// `fld`/`fild`/constants, `fst`/`fstp`/`fstpnce`/`fist`/`fistp`/`fisttp`, `fadd`/`fsub`/`fsubr`/
/// `fmul`/`fdiv`/`fdivr` (+ pop & integer forms), `fprem`/`fprem1`, `fxam`/`ftst`/`fcom*`/`fucom*`/
/// `fcomi*`/`fucomi*`, `fxch`/`ffree`/`ffreep`/`fabs`/`fchs`/`fsqrt`/`frndint`/`fdecstp`/`fincstp`,
/// `fninit`/`finit`/`fnclex`/`fclex`/`fnstsw`/`fstsw`.
pub(crate) unsafe fn exec_x87_ins(cpu: &mut Cpu, mn: Mnemonic, ins: &Instruction) -> bool {
    let precise = super::precise_faults();
    let faulted = || precise && super::FAULT_PENDING.with(|p| p.get().is_some());
    let precheck_write = |cpu: &mut Cpu, addr: u64, sz: usize| -> bool {
        if precise && !is_valid_guest_range(addr, sz) {
            let bad = if addr < 0x1_0000 || addr >= 0x8000_0000_0000 {
                addr
            } else {
                0x8000_0000_0000
            };
            let _ = cpu.read_mem(bad, (sz.min(8) * 8) as u32);
            return false;
        }
        true
    };
    let read_f80 = |cpu: &mut Cpu, addr: u64| -> Option<[u8; 10]> {
        let lo = cpu.read_mem(addr, 64);
        if faulted() {
            return None;
        }
        let hi = cpu.read_mem(addr.wrapping_add(8), 16) as u16;
        if faulted() {
            return None;
        }
        let mut b = [0u8; 10];
        b[0..8].copy_from_slice(&lo.to_le_bytes());
        b[8..10].copy_from_slice(&hi.to_le_bytes());
        Some(b)
    };
    let read_int_mem = |cpu: &mut Cpu, ins: &Instruction| -> Option<i64> {
        let addr = cpu.eff_addr(ins);
        let sz = ins.memory_size().size();
        let raw = match sz {
            2 => cpu.read_mem(addr, 16) as u16 as i16 as i64,
            4 => cpu.read_mem(addr, 32) as u32 as i32 as i64,
            8 => cpu.read_mem(addr, 64) as i64,
            _ => return None,
        };
        if faulted() {
            return None;
        }
        Some(raw)
    };
    match mn {
        Mnemonic::Fninit | Mnemonic::Finit => {
            x87_reset();
            true
        }
        Mnemonic::Fnclex | Mnemonic::Fclex => {
            with_x87_mut(|st| st.clear_exceptions());
            true
        }
        Mnemonic::Fdecstp => {
            with_x87_mut(|st| {
                st.top = st.top.wrapping_sub(1) & 7;
                st.sw &= !0x0200;
                st.sync_top();
            });
            true
        }
        Mnemonic::Fincstp => {
            with_x87_mut(|st| {
                st.top = st.top.wrapping_add(1) & 7;
                st.sw &= !0x0200;
                st.sync_top();
            });
            true
        }
        Mnemonic::Ffree | Mnemonic::Ffreep => {
            let idx = if ins.op_count() > 0 && ins.op_kind(0) == OpKind::Register {
                let Some(i) = st_reg_index(ins.op_register(0)) else {
                    return false;
                };
                i
            } else {
                0
            };
            with_x87_mut(|st| {
                if mn == Mnemonic::Ffreep {
                    st.freep_st(idx);
                } else {
                    st.free_st(idx);
                }
            });
            true
        }
        Mnemonic::Fxam => {
            with_x87_mut(|st| st.fxam());
            true
        }
        Mnemonic::Fxch => {
            let idx = if ins.op_count() >= 2 && ins.op_kind(1) == OpKind::Register {
                let Some(i) = st_reg_index(ins.op_register(1)) else {
                    return false;
                };
                i
            } else if ins.op_count() == 1 && ins.op_kind(0) == OpKind::Register {
                let Some(i) = st_reg_index(ins.op_register(0)) else {
                    return false;
                };
                i
            } else {
                1
            };
            with_x87_mut(|st| st.fxch(idx));
            true
        }
        Mnemonic::Fldz
        | Mnemonic::Fld1
        | Mnemonic::Fldpi
        | Mnemonic::Fldl2e
        | Mnemonic::Fldl2t
        | Mnemonic::Fldlg2
        | Mnemonic::Fldln2 => {
            let v: f64 = match mn {
                Mnemonic::Fldz => 0.0,
                Mnemonic::Fld1 => 1.0,
                Mnemonic::Fldpi => core::f64::consts::PI,
                Mnemonic::Fldl2e => core::f64::consts::LOG2_E,
                Mnemonic::Fldl2t => core::f64::consts::LOG2_10,
                Mnemonic::Fldlg2 => core::f64::consts::LOG10_2,
                _ => core::f64::consts::LN_2,
            };
            with_x87_mut(|st| st.push_f64_bits(v.to_bits()));
            true
        }
        Mnemonic::Fld => match ins.op_kind(0) {
            OpKind::Register => {
                let Some(idx) = st_reg_index(ins.op_register(0)) else {
                    return false;
                };
                with_x87_mut(|st| {
                    let pi = st.phys(idx);
                    if (st.valid_mask & (1 << pi)) == 0 {
                        st.push_fp80(
                            Fp80 { kind: Fp80Kind::Nan, sign: 1, exp: 0, sig: 0xC000_0000_0000_0000 },
                            0x0041,
                        );
                    } else {
                        let val = st.regs[pi];
                        st.push_bytes(val, 0);
                    }
                });
                true
            }
            OpKind::Memory => {
                let addr = cpu.eff_addr(ins);
                match ins.memory_size().size() {
                    8 => {
                        let bits = cpu.read_mem(addr, 64);
                        if faulted() {
                            return true;
                        }
                        with_x87_mut(|st| st.push_f64_bits(bits));
                        true
                    }
                    4 => {
                        let bits = (f32::from_bits(cpu.read_mem(addr, 32) as u32) as f64).to_bits();
                        if faulted() {
                            return true;
                        }
                        with_x87_mut(|st| st.push_f64_bits(bits));
                        true
                    }
                    10 => {
                        let Some(bytes) = read_f80(cpu, addr) else {
                            return true;
                        };
                        with_x87_mut(|st| {
                            st.push_bytes(bytes, 0);
                        });
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        },
        Mnemonic::Fild => {
            if ins.op_kind(0) != OpKind::Memory {
                return false;
            }
            let Some(v) = read_int_mem(cpu, ins) else {
                return faulted();
            };
            with_x87_mut(|st| st.push_fp80(Fp80::from_i64(v), 0));
            true
        }
        Mnemonic::Fprem => {
            fprem(cpu);
            true
        }
        Mnemonic::Fprem1 => {
            with_x87_mut(|st| {
                let (a, b) = st.arithmetic_inputs(mn, 0, X87Source::St(1));
                const C_MASK: u16 = (1 << 8) | (1 << 9) | (1 << 10) | (1 << 14);
                st.sw &= !C_MASK;
                if a.is_nan() || b.is_nan() || a.is_infinite() || b == 0.0 {
                    st.sw |= 0x0001;
                    st.write_st_f64(0, f64::from_bits(0xFFF8_0000_0000_0000));
                } else if a == 0.0 || b.is_infinite() {
                    st.sync_top();
                } else {
                    let qf = (a / b).round_ties_even();
                    let rem = a - qf * b;
                    let q = (qf.abs() as u64) & 7;
                    if (q & 1) != 0 {
                        st.sw |= 1 << 9;
                    }
                    if (q & 2) != 0 {
                        st.sw |= 1 << 14;
                    }
                    if (q & 4) != 0 {
                        st.sw |= 1 << 8;
                    }
                    let res = if rem == 0.0 { 0.0f64.copysign(a) } else { rem };
                    st.write_st_f64(0, res);
                }
            });
            true
        }
        Mnemonic::Fnstsw | Mnemonic::Fstsw => {
            let sw = with_x87_ref(|st| st.status_word()) as u64;
            match ins.op_kind(0) {
                OpKind::Register if ins.op_register(0) == Register::AX => {
                    cpu.set_reg(Register::AX, sw);
                    true
                }
                OpKind::Memory => {
                    let addr = cpu.eff_addr(ins);
                    cpu.write_mem(addr, 16, sw);
                    true
                }
                _ => false,
            }
        }
        Mnemonic::Fst => match ins.op_kind(0) {
            OpKind::Register => {
                let Some(idx) = st_reg_index(ins.op_register(0)) else {
                    return false;
                };
                with_x87_mut(|st| {
                    let p0 = st.phys(0);
                    let pi = st.phys(idx);
                    let v = st.regs[p0];
                    st.regs[pi] = v;
                    st.valid_mask |= 1 << pi;
                    st.sw &= !0x0200;
                    st.sync_top();
                });
                true
            }
            OpKind::Memory => {
                let sz = ins.memory_size().size();
                if sz != 8 && sz != 4 {
                    return false;
                }
                let addr = cpu.eff_addr(ins);
                if !precheck_write(cpu, addr, sz) {
                    return true;
                }
                let snap = if precise { Some(x87_snapshot()) } else { None };
                let bits64 = with_x87_mut(|st| {
                    let p0 = st.phys(0);
                    let mut sw = st.sw & !0x0200;
                    let out = if (st.valid_mask & (1 << p0)) == 0 {
                        sw |= 0x0041;
                        0xFFF8_0000_0000_0000
                    } else {
                        st.value(p0).to_f64_bits(&mut sw)
                    };
                    st.sw = sw;
                    st.sync_top();
                    out
                });
                if sz == 8 {
                    cpu.write_mem(addr, 64, bits64);
                } else {
                    cpu.write_mem(addr, 32, (f64::from_bits(bits64) as f32).to_bits() as u64);
                }
                if let Some(s) = snap {
                    if faulted() {
                        x87_restore(s);
                    }
                }
                true
            }
            _ => false,
        },
        Mnemonic::Fstp | Mnemonic::Fstpnce => match ins.op_kind(0) {
            OpKind::Register => {
                let Some(idx) = st_reg_index(ins.op_register(0)) else {
                    return false;
                };
                fstp_st(cpu, idx);
                true
            }
            OpKind::Memory => {
                let sz = ins.memory_size().size();
                if sz != 8 && sz != 4 && sz != 10 {
                    return false;
                }
                let addr = cpu.eff_addr(ins);
                if !precheck_write(cpu, addr, sz) {
                    return true;
                }
                let snap = if precise { Some(x87_snapshot()) } else { None };
                if sz == 10 {
                    let bytes = with_x87_mut(|st| {
                        let p0 = st.phys(0);
                        let val = if (st.valid_mask & (1 << p0)) == 0 {
                            st.sw |= 0x0041;
                            Fp80 { kind: Fp80Kind::Nan, sign: 1, exp: 0, sig: 0xC000_0000_0000_0000 }.to_f80_bytes()
                        } else {
                            let v = st.regs[p0];
                            st.valid_mask &= !(1 << p0);
                            v
                        };
                        st.top = st.top.wrapping_add(1) & 7;
                        st.sw &= !0x0200;
                        st.sync_top();
                        val
                    });
                    cpu.write_mem(addr, 64, u64::from_le_bytes(bytes[0..8].try_into().unwrap()));
                    if !faulted() {
                        cpu.write_mem(addr.wrapping_add(8), 16, u16::from_le_bytes([bytes[8], bytes[9]]) as u64);
                    }
                } else {
                    let bits64 = with_x87_mut(|st| st.pop_f64_bits());
                    if sz == 8 {
                        cpu.write_mem(addr, 64, bits64);
                    } else {
                        let f = f64::from_bits(bits64) as f32;
                        cpu.write_mem(addr, 32, f.to_bits() as u64);
                    }
                }
                if let Some(s) = snap {
                    if faulted() {
                        x87_restore(s);
                    }
                }
                true
            }
            _ => false,
        },
        Mnemonic::Fist | Mnemonic::Fistp | Mnemonic::Fisttp => {
            if ins.op_kind(0) != OpKind::Memory {
                return false;
            }
            let sz = ins.memory_size().size();
            if sz != 2 && sz != 4 && sz != 8 {
                return false;
            }
            let addr = cpu.eff_addr(ins);
            if !precheck_write(cpu, addr, sz) {
                return true;
            }
            let pop = matches!(mn, Mnemonic::Fistp | Mnemonic::Fisttp);
            let trunc = mn == Mnemonic::Fisttp;
            let bits = (sz * 8) as u32;
            let snap = if precise { Some(x87_snapshot()) } else { None };
            let out = with_x87_mut(|st| {
                let p0 = st.phys(0);
                let mut sw = st.sw & !0x0200;
                let v = if (st.valid_mask & (1 << p0)) == 0 {
                    sw |= 0x0041; // SF | IE
                    1u64 << (bits - 1)
                } else {
                    st.value(p0).to_int_bits(bits, trunc, &mut sw)
                };
                if pop {
                    st.valid_mask &= !(1 << p0);
                    st.top = st.top.wrapping_add(1) & 7;
                }
                st.sw = sw;
                st.sync_top();
                v
            });
            cpu.write_mem(addr, bits, out);
            if let Some(s) = snap {
                if faulted() {
                    x87_restore(s);
                }
            }
            true
        }
        Mnemonic::Fadd
        | Mnemonic::Faddp
        | Mnemonic::Fiadd
        | Mnemonic::Fsub
        | Mnemonic::Fsubp
        | Mnemonic::Fisub
        | Mnemonic::Fsubr
        | Mnemonic::Fsubrp
        | Mnemonic::Fisubr
        | Mnemonic::Fmul
        | Mnemonic::Fmulp
        | Mnemonic::Fimul
        | Mnemonic::Fdiv
        | Mnemonic::Fdivp
        | Mnemonic::Fidiv
        | Mnemonic::Fdivr
        | Mnemonic::Fdivrp
        | Mnemonic::Fidivr => {
            let pop = matches!(
                mn,
                Mnemonic::Faddp
                    | Mnemonic::Fsubp
                    | Mnemonic::Fsubrp
                    | Mnemonic::Fmulp
                    | Mnemonic::Fdivp
                    | Mnemonic::Fdivrp
            );
            let is_int = matches!(
                mn,
                Mnemonic::Fiadd
                    | Mnemonic::Fisub
                    | Mnemonic::Fisubr
                    | Mnemonic::Fimul
                    | Mnemonic::Fidiv
                    | Mnemonic::Fidivr
            );
            let apply = |dst_val: f64, src_val: f64| -> f64 {
                match mn {
                    Mnemonic::Fadd | Mnemonic::Faddp | Mnemonic::Fiadd => dst_val + src_val,
                    Mnemonic::Fsub | Mnemonic::Fsubp | Mnemonic::Fisub => dst_val - src_val,
                    Mnemonic::Fsubr | Mnemonic::Fsubrp | Mnemonic::Fisubr => src_val - dst_val,
                    Mnemonic::Fmul | Mnemonic::Fmulp | Mnemonic::Fimul => dst_val * src_val,
                    Mnemonic::Fdiv | Mnemonic::Fdivp | Mnemonic::Fidiv => dst_val / src_val,
                    _ => src_val / dst_val, // Fdivr | Fdivrp | Fidivr
                }
            };
            if ins.op_count() == 0 {
                with_x87_mut(|st| {
                    let (d, s) = st.arithmetic_inputs(mn, 1, X87Source::St(0));
                    st.write_st_f64(1, apply(d, s));
                    if pop {
                        st.pop_discard();
                    }
                });
                true
            } else if ins.op_kind(0) == OpKind::Memory {
                let src_val = if is_int {
                    let Some(iv) = read_int_mem(cpu, ins) else {
                        return faulted();
                    };
                    iv as f64
                } else {
                    let addr = cpu.eff_addr(ins);
                    let v = match ins.memory_size().size() {
                        8 => f64::from_bits(cpu.read_mem(addr, 64)),
                        4 => f32::from_bits(cpu.read_mem(addr, 32) as u32) as f64,
                        _ => return false,
                    };
                    if faulted() {
                        return true;
                    }
                    v
                };
                with_x87_mut(|st| {
                    let (d, s) = st.arithmetic_inputs(mn, 0, X87Source::Value(src_val));
                    st.write_st_f64(0, apply(d, s));
                });
                true
            } else if ins.op_kind(0) == OpKind::Register {
                let Some(d_idx) = st_reg_index(ins.op_register(0)) else {
                    return false;
                };
                let s_idx = if ins.op_count() >= 2 {
                    let Some(i) = st_reg_index(ins.op_register(1)) else {
                        return false;
                    };
                    i
                } else {
                    0
                };
                with_x87_mut(|st| {
                    let (d, s) = st.arithmetic_inputs(mn, d_idx, X87Source::St(s_idx));
                    st.write_st_f64(d_idx, apply(d, s));
                    if pop {
                        st.pop_discard();
                    }
                });
                true
            } else {
                false
            }
        }
        Mnemonic::Fabs | Mnemonic::Fchs | Mnemonic::Fsqrt | Mnemonic::Frndint => {
            with_x87_mut(|st| {
                match mn {
                    Mnemonic::Fabs => {
                        let p0 = st.phys(0);
                        st.regs[p0][9] &= 0x7f;
                    }
                    Mnemonic::Fchs => {
                        let p0 = st.phys(0);
                        st.regs[p0][9] ^= 0x80;
                    }
                    Mnemonic::Fsqrt => {
                        let (v, _) = st.arithmetic_inputs(mn, 0, X87Source::Value(0.0));
                        st.write_st_f64(0, v.sqrt());
                    }
                    _ => {
                        let (v, _) = st.arithmetic_inputs(mn, 0, X87Source::Value(0.0));
                        st.write_st_f64(0, v.round_ties_even());
                    }
                }
                st.sw &= !0x0200;
                st.sync_top();
            });
            true
        }
        Mnemonic::Ftst => {
            with_x87_mut(|st| {
                let (a, zero) = st.arithmetic_inputs(mn, 0, X87Source::Value(0.0));
                st.compare_sw(a, zero);
            });
            true
        }
        Mnemonic::Fcom
        | Mnemonic::Fcomp
        | Mnemonic::Fcompp
        | Mnemonic::Fucom
        | Mnemonic::Fucomp
        | Mnemonic::Fucompp
        | Mnemonic::Ficom
        | Mnemonic::Ficomp => {
            let pops = match mn {
                Mnemonic::Fcompp | Mnemonic::Fucompp => 2,
                Mnemonic::Fcomp | Mnemonic::Fucomp | Mnemonic::Ficomp => 1,
                _ => 0,
            };
            let is_int = matches!(mn, Mnemonic::Ficom | Mnemonic::Ficomp);
            if ins.op_count() == 0 {
                with_x87_mut(|st| {
                    let (a, rhs) = st.arithmetic_inputs(mn, 0, X87Source::St(1));
                    st.compare_sw(a, rhs);
                    for _ in 0..pops {
                        st.pop_discard();
                    }
                });
                true
            } else if ins.op_kind(0) == OpKind::Memory {
                let rhs = if is_int {
                    let Some(iv) = read_int_mem(cpu, ins) else {
                        return faulted();
                    };
                    iv as f64
                } else {
                    let addr = cpu.eff_addr(ins);
                    let v = match ins.memory_size().size() {
                        8 => f64::from_bits(cpu.read_mem(addr, 64)),
                        4 => f32::from_bits(cpu.read_mem(addr, 32) as u32) as f64,
                        _ => return false,
                    };
                    if faulted() {
                        return true;
                    }
                    v
                };
                with_x87_mut(|st| {
                    let (a, rhs) = st.arithmetic_inputs(mn, 0, X87Source::Value(rhs));
                    st.compare_sw(a, rhs);
                    for _ in 0..pops {
                        st.pop_discard();
                    }
                });
                true
            } else if ins.op_kind(0) == OpKind::Register {
                let reg_op = if ins.op_count() >= 2 && ins.op_kind(1) == OpKind::Register { 1 } else { 0 };
                let Some(idx) = st_reg_index(ins.op_register(reg_op)) else {
                    return false;
                };
                with_x87_mut(|st| {
                    let (a, rhs) = st.arithmetic_inputs(mn, 0, X87Source::St(idx));
                    st.compare_sw(a, rhs);
                    for _ in 0..pops {
                        st.pop_discard();
                    }
                });
                true
            } else {
                false
            }
        }
        Mnemonic::Fcomi | Mnemonic::Fcomip | Mnemonic::Fucomi | Mnemonic::Fucomip => {
            let idx = if ins.op_count() >= 2 && ins.op_kind(1) == OpKind::Register {
                let Some(i) = st_reg_index(ins.op_register(1)) else {
                    return false;
                };
                i
            } else if ins.op_count() == 1 && ins.op_kind(0) == OpKind::Register {
                let Some(i) = st_reg_index(ins.op_register(0)) else {
                    return false;
                };
                i
            } else {
                return false;
            };
            let pop = matches!(mn, Mnemonic::Fcomip | Mnemonic::Fucomip);
            let (a, b) = with_x87_mut(|st| {
                let (a, b) = st.arithmetic_inputs(mn, 0, X87Source::St(idx));
                if pop {
                    st.pop_discard();
                }
                (a, b)
            });
            // Branchless x86 FCOMI/FUCOMI flags:
            // - ordered a > b: ZF=0, PF=0, CF=0
            // - ordered a < b: ZF=0, PF=0, CF=1
            // - ordered a == b: ZF=1, PF=0, CF=0
            // - unordered (NaN): ZF=1, PF=1, CF=1
            // Note: `cpu.pfv` encodes PF via even byte parity (`pf() == (pfv.count_ones() & 1 == 0)`),
            // so `pfv = 0` means PF=1 and `pfv = 1` means PF=0.
            let unord = !(a <= b) & !(a > b);
            cpu.sf = false;
            cpu.of = false;
            cpu.zf = !(a < b) & !(a > b);
            cpu.cf = !(a >= b);
            cpu.pfv = (!unord) as u8;
            true
        }
        _ => false,
    }
}

// =================================================================================================
// Exhaustive differential unit tests against `emu::Cpu` + real V8 builtin byte sequences
// =================================================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emu::regfile::{ctx, RegfileExit, RF_FAULT, RF_JUMP, RF_RET};
    use iced_x86::{Decoder, DecoderOptions, FlowControl, OpKind};

    /// Real 50-byte x87 `Float64Mod` reduction block extracted verbatim from `v8.dll` builtin
    /// `421` (`Modulus_Baseline`, `+0x15ede7..+0x15ee19`, identical across all 17 sites in the
    /// 14 `v8_nonabi_x87` builtins up to source/destination XMM register number) + `ret`:
    /// ```text
    /// sub   rsp, 8
    /// movsd qword ptr [rsp], xmm0      ; divisor  b
    /// fld   qword ptr [rsp]            ; ST(0) = b
    /// movsd qword ptr [rsp], xmm1      ; dividend a
    /// fld   qword ptr [rsp]            ; ST(0) = a, ST(1) = b
    /// .loop:
    /// fprem                            ; ST(0) = partial_rem(ST(0), ST(1))
    /// fnstsw ax
    /// shr   eax, 8
    /// and   eax, 0xff
    /// push  rax
    /// popfq                            ; PF <- C2 (bit 10 of SW -> bit 2 of AL)
    /// jp    .loop
    /// fstp  st(1)                      ; pop divisor, leaving remainder in ST(0)
    /// fstp  qword ptr [rsp]            ; store rounded f64 remainder and pop
    /// movsd xmm1, qword ptr [rsp]
    /// add   rsp, 8
    /// ret
    /// ```
    #[rustfmt::skip]
    const V8_BID421_FPREM_LOOP: &[u8] = &[
        0x48, 0x83, 0xec, 0x08,                         // +0x00: sub rsp, 8
        0xf2, 0x0f, 0x11, 0x04, 0x24,                   // +0x04: movsd qword ptr [rsp], xmm0
        0xdd, 0x04, 0x24,                               // +0x09: fld qword ptr [rsp]
        0xf2, 0x0f, 0x11, 0x0c, 0x24,                   // +0x0c: movsd qword ptr [rsp], xmm1
        0xdd, 0x04, 0x24,                               // +0x11: fld qword ptr [rsp]
        0xd9, 0xf8,                                     // +0x14: fprem
        0xdf, 0xe0,                                     // +0x16: fnstsw ax
        0xc1, 0xe8, 0x08,                               // +0x18: shr eax, 8
        0x25, 0xff, 0x00, 0x00, 0x00,                   // +0x1b: and eax, 0xff
        0x50,                                           // +0x20: push rax
        0x9d,                                           // +0x21: popfq
        0x7a, 0xf0,                                     // +0x22: jp +0x14
        0xdd, 0xd9,                                     // +0x24: fstp st(1)
        0xdd, 0x1c, 0x24,                               // +0x26: fstp qword ptr [rsp]
        0xf2, 0x0f, 0x10, 0x0c, 0x24,                   // +0x29: movsd xmm1, qword ptr [rsp]
        0x48, 0x83, 0xc4, 0x08,                         // +0x2e: add rsp, 8
        0xc3,                                           // +0x32: ret
    ];

    // Real byte slices from `v8.dll` atomic builtins `[538..545]` (+ trailing `ret` `0xc3` for standalone oracle execution):

    /// `bid=538` (`AtomicsStore`, `+0x171344`): `xchg byte ptr [rdx + r9], r11b; movsx r11d, r11b; ret`
    const V8_BID538_XCHG8: &[u8] = &[0x46, 0x86, 0x1c, 0x0a, 0x45, 0x0f, 0xbe, 0xdb, 0xc3];
    /// `bid=538` (`AtomicsStore`, `+0x17137f`): `xchg word ptr [rdx + rcx], r11w; movsx r11d, r11w; ret`
    const V8_BID538_XCHG16: &[u8] = &[0x66, 0x44, 0x87, 0x1c, 0x0a, 0x45, 0x0f, 0xbf, 0xdb, 0xc3];
    /// `bid=538` (`AtomicsStore`, `+0x1713a6`): `xchg dword ptr [rdx + rcx], r11d; ret`
    const V8_BID538_XCHG32: &[u8] = &[0x44, 0x87, 0x1c, 0x0a, 0xc3];
    /// `bid=538` (`AtomicsStore`, `+0x171427`): `xchg qword ptr [rdi + r8], rcx; ret`
    const V8_BID538_XCHG64: &[u8] = &[0x4a, 0x87, 0x0c, 0x07, 0xc3];

    /// `bid=539` (`AtomicsExchange`, `+0x17174b`): `xchg byte ptr [rdi + r12], cl; movsx ecx, cl; lea eax, [rcx + rcx]; ret`
    const V8_BID539_XCHG8_S: &[u8] = &[0x42, 0x86, 0x0c, 0x27, 0x0f, 0xbe, 0xc9, 0x8d, 0x04, 0x09, 0xc3];
    /// `bid=539` (`AtomicsExchange`, `+0x1717a0`): `xchg word ptr [rdi + r8], cx; movsx ecx, cx; lea eax, [rcx + rcx]; ret`
    const V8_BID539_XCHG16_S: &[u8] = &[0x66, 0x42, 0x87, 0x0c, 0x07, 0x0f, 0xbf, 0xc9, 0x8d, 0x04, 0x09, 0xc3];
    /// `bid=539` (`AtomicsExchange`, `+0x1717e5`): `xchg dword ptr [r8 + rdi], ecx; mov rdi, rcx; add edi, ecx; ret`
    const V8_BID539_XCHG32_S: &[u8] = &[0x41, 0x87, 0x0c, 0x38, 0x48, 0x8b, 0xf9, 0x03, 0xf9, 0xc3];
    /// `bid=539` (`AtomicsExchange`, `+0x1718c6`): `xchg qword ptr [rdi + r8], rcx; cmp rcx, 0; ret`
    const V8_BID539_XCHG64: &[u8] = &[0x4a, 0x87, 0x0c, 0x07, 0x48, 0x83, 0xf9, 0x00, 0xc3];

    /// `bid=540` (`AtomicsCompareExchange`, `+0x171ed7`): `lock cmpxchg byte ptr [rdi + r12], cl; movsx eax, al; shl eax, 1; ret`
    const V8_BID540_CMPXCHG8_S: &[u8] = &[0xf0, 0x42, 0x0f, 0xb0, 0x0c, 0x27, 0x0f, 0xbe, 0xc0, 0xd1, 0xe0, 0xc3];
    /// `bid=540` (`AtomicsCompareExchange`, `+0x171f11`): `lock cmpxchg byte ptr [rdi + r12], cl; movzx eax, al; shl eax, 1; ret`
    const V8_BID540_CMPXCHG8_U: &[u8] = &[0xf0, 0x42, 0x0f, 0xb0, 0x0c, 0x27, 0x0f, 0xb6, 0xc0, 0xd1, 0xe0, 0xc3];
    /// `bid=540` (`AtomicsCompareExchange`, `+0x171f36`): `lock cmpxchg word ptr [rdi + r8], cx; movsx eax, ax; shl eax, 1; ret`
    const V8_BID540_CMPXCHG16_S: &[u8] = &[0xf0, 0x66, 0x42, 0x0f, 0xb1, 0x0c, 0x07, 0x0f, 0xbf, 0xc0, 0xd1, 0xe0, 0xc3];
    /// `bid=540` (`AtomicsCompareExchange`, `+0x171f88`): `lock cmpxchg dword ptr [r8 + rdi], ecx; mov rcx, rax; add ecx, eax; ret`
    const V8_BID540_CMPXCHG32_S: &[u8] = &[0xf0, 0x41, 0x0f, 0xb1, 0x0c, 0x38, 0x48, 0x8b, 0xc8, 0x03, 0xc8, 0xc3];
    /// `bid=540` (`AtomicsCompareExchange`, `+0x172105`): `lock cmpxchg qword ptr [rdi + r8], rcx; cmp rax, 0; ret`
    const V8_BID540_CMPXCHG64: &[u8] = &[0xf0, 0x4a, 0x0f, 0xb1, 0x0c, 0x07, 0x48, 0x83, 0xf8, 0x00, 0xc3];

    /// `bid=541` (`AtomicsAdd`, `+0x172693`): `mov al, [rdi+r12]; mov r8d, eax; add r8d, ecx; lock cmpxchg [rdi+r12], r8b; jne -18; movsx eax, al; shl eax, 1; ret`
    const V8_BID541_ADD_CAS8: &[u8] = &[
        0x42, 0x8a, 0x04, 0x27, 0x44, 0x8b, 0xc0, 0x44, 0x03, 0xc1, 0xf0, 0x46, 0x0f, 0xb0, 0x04,
        0x27, 0x75, 0xee, 0x0f, 0xbe, 0xc0, 0xd1, 0xe0, 0xc3,
    ];
    /// `bid=541` (`AtomicsAdd`, `+0x172702`): 16-bit CAS retry loop
    const V8_BID541_ADD_CAS16: &[u8] = &[
        0x66, 0x42, 0x8b, 0x04, 0x07, 0x44, 0x8b, 0xc8, 0x44, 0x03, 0xc9, 0xf0, 0x66, 0x46, 0x0f,
        0xb1, 0x0c, 0x07, 0x75, 0xec, 0x0f, 0xbf, 0xc0, 0xd1, 0xe0, 0xc3,
    ];
    /// `bid=541` (`AtomicsAdd`, `+0x172766`): 32-bit CAS retry loop
    const V8_BID541_ADD_CAS32: &[u8] = &[
        0x41, 0x8b, 0x04, 0x38, 0x44, 0x8b, 0xc8, 0x44, 0x03, 0xc9, 0xf0, 0x45, 0x0f, 0xb1, 0x0c,
        0x38, 0x75, 0xee, 0x48, 0x8b, 0xc8, 0x03, 0xc8, 0xc3,
    ];
    /// `bid=541` (`AtomicsAdd`, `+0x172862`): 64-bit CAS retry loop
    const V8_BID541_ADD_CAS64: &[u8] = &[
        0x4a, 0x8b, 0x04, 0x07, 0x4c, 0x8b, 0xc8, 0x4c, 0x03, 0xc9, 0xf0, 0x4e, 0x0f, 0xb1, 0x0c,
        0x07, 0x75, 0xee, 0x48, 0x83, 0xf8, 0x00, 0xc3,
    ];

    /// `bid=542..545` (`AtomicsSub`, `AtomicsAnd`, `AtomicsOr`, `AtomicsXor`) 8-bit CAS retry loops
    const V8_BID542_SUB_CAS8: &[u8] = &[
        0x42, 0x8a, 0x04, 0x27, 0x44, 0x8b, 0xc0, 0x44, 0x2b, 0xc1, 0xf0, 0x46, 0x0f, 0xb0, 0x04,
        0x27, 0x75, 0xee, 0xc3,
    ];
    const V8_BID543_AND_CAS8: &[u8] = &[
        0x42, 0x8a, 0x04, 0x27, 0x44, 0x8b, 0xc0, 0x44, 0x23, 0xc1, 0xf0, 0x46, 0x0f, 0xb0, 0x04,
        0x27, 0x75, 0xee, 0xc3,
    ];
    const V8_BID544_OR_CAS8: &[u8] = &[
        0x42, 0x8a, 0x04, 0x27, 0x44, 0x8b, 0xc0, 0x44, 0x0b, 0xc1, 0xf0, 0x46, 0x0f, 0xb0, 0x04,
        0x27, 0x75, 0xee, 0xc3,
    ];
    const V8_BID545_XOR_CAS8: &[u8] = &[
        0x42, 0x8a, 0x04, 0x27, 0x44, 0x8b, 0xc0, 0x44, 0x33, 0xc1, 0xf0, 0x46, 0x0f, 0xb0, 0x04,
        0x27, 0x75, 0xee, 0xc3,
    ];

    /// Step `cpu` through `[lo, hi)` using `emu::Cpu` (`run_oracle` + `exec_x87_ins` fallback for
    /// x87 instructions and boundary-straddling precise-fault guard).
    unsafe fn run_oracle_with_x87(cpu: &mut Cpu, lo: u64, hi: u64) -> RegfileExit {
        let prev_prec = crate::emu::PRECISE_LOCAL.with(|p| p.replace(true));
        crate::emu::FAULT_PENDING.with(|p| p.set(None));
        cpu.rip = lo;
        let res = loop {
            let cur_ip = cpu.rip;
            if cur_ip < lo || cur_ip >= hi {
                break RegfileExit::jump(cur_ip);
            }
            let mapped = crate::emu::to_mapped_addr(cur_ip);
            if !(0x1_0000..0x8000_0000_0000).contains(&mapped) {
                break RegfileExit::fault(cur_ip);
            }
            let max_len = (0x8000_0000_0000usize - mapped).min(15);
            let bytes = core::slice::from_raw_parts(mapped as *const u8, max_len);
            let mut dec = Decoder::with_ip(64, bytes, cur_ip, DecoderOptions::NONE);
            let ins = dec.decode();
            if ins.is_invalid() {
                break RegfileExit::fault(cur_ip);
            }
            let mn = ins.mnemonic();
            if matches!(
                mn,
                Mnemonic::Int3 | Mnemonic::Ud2 | Mnemonic::Int | Mnemonic::Into | Mnemonic::Int1 | Mnemonic::Hlt
            ) {
                break RegfileExit::fault(cur_ip);
            }
            let pre = cpu.clone();
            let pre_x87 = x87_snapshot();
            let next_ip = ins.next_ip();
            cpu.rip = next_ip;
            match ins.flow_control() {
                FlowControl::Return => {
                    let imm = if ins.op_count() > 0 { cpu.read_op(&ins, 0, 16) } else { 0 };
                    if !is_valid_guest_range(cpu.gpr[4], 8) {
                        *cpu = pre;
                        cpu.rip = cur_ip;
                        break RegfileExit::fault(cur_ip);
                    }
                    let ret_va = cpu.read_mem(cpu.gpr[4], 64);
                    if crate::emu::FAULT_PENDING.with(|p| p.take().is_some()) {
                        *cpu = pre;
                        cpu.rip = cur_ip;
                        break RegfileExit::fault(cur_ip);
                    }
                    cpu.gpr[4] = cpu.gpr[4].wrapping_add(8 + imm);
                    break RegfileExit::ret(ret_va);
                }
                FlowControl::ConditionalBranch => {
                    if cpu.eval_cc(ins.condition_code()) {
                        let tgt = match ins.op_kind(0) {
                            OpKind::NearBranch64 => ins.near_branch64(),
                            OpKind::NearBranch32 => ins.near_branch32() as u64,
                            _ => ins.near_branch16() as u64,
                        };
                        cpu.rip = tgt;
                        if tgt < lo || tgt >= hi {
                            break RegfileExit::jump(tgt);
                        }
                    }
                }
                FlowControl::Next => {
                    // Check memory operand range straddling `0x8000_0000_0000` (e.g. `0x7FFF_FFFF_FFFC` u64)
                    let mut straddle_fault = false;
                    for op_i in 0..ins.op_count() {
                        if ins.op_kind(op_i) == OpKind::Memory && mn != Mnemonic::Lea {
                            let sz = ins.memory_size().size();
                            if sz > 0 && !is_valid_guest_range(cpu.eff_addr(&ins), sz) {
                                straddle_fault = true;
                            }
                        }
                    }
                    if straddle_fault {
                        *cpu = pre;
                        x87_restore(pre_x87);
                        cpu.rip = cur_ip;
                        break RegfileExit::fault(cur_ip);
                    }
                    let ok = cpu.exec_generic_ins(&ins) || exec_x87_ins(cpu, mn, &ins);
                    if !ok || crate::emu::FAULT_PENDING.with(|p| p.take().is_some()) {
                        *cpu = pre;
                        x87_restore(pre_x87);
                        cpu.rip = cur_ip;
                        break RegfileExit::fault(cur_ip);
                    }
                }
                _ => {
                    *cpu = pre;
                    x87_restore(pre_x87);
                    cpu.rip = cur_ip;
                    break RegfileExit::fault(cur_ip);
                }
            }
        };
        crate::emu::PRECISE_LOCAL.with(|p| p.set(prev_prec));
        res
    }

    fn assert_cpu_bit_exact(label: &str, ex_got: RegfileExit, ex_want: RegfileExit, got: &Cpu, want: &Cpu) {
        assert_eq!(ex_got, ex_want, "{label}: RegfileExit mismatch");
        assert_eq!(got.gpr, want.gpr, "{label}: GPR mismatch");
        assert_eq!(got.xmm, want.xmm, "{label}: XMM mismatch");
        assert_eq!(got.ymm_hi, want.ymm_hi, "{label}: YMM_HI mismatch");
        assert_eq!(got.rflags(), want.rflags(), "{label}: RFLAGS mismatch (got={:#x}, want={:#x})", got.rflags(), want.rflags());
        assert_eq!(got.last_bits, want.last_bits, "{label}: last_bits mismatch");
    }

    /// Lowered `DF_REGFILE` body for `V8_BID421_FPREM_LOOP` using `Rf` + `regfile_ext` primitives,
    /// mirroring the exact code `v8_regfile.py` will emit in `g28`.
    unsafe fn lowered_bid421_fprem_loop(cpu: &mut Cpu, code_va: u64) -> RegfileExit {
        let raw = cpu as *mut Cpu as *mut u8;
        let mut c = ctx(raw);
        let mut blk: u32 = 0;
        loop {
            match blk {
                0 => {
                    // +0x00: sub rsp, 8
                    let r0 = c.sub(c.r64(4), 8, 64);
                    c.w64(4, r0);
                    // +0x04: movsd [rsp], xmm0
                    if !c.movsd_mr(c.r64(4), 0) {
                        return RegfileExit::fault(code_va + 0x04);
                    }
                    // +0x09: fld qword ptr [rsp]
                    if !fld_m64(&mut *(raw as *mut Cpu), c.r64(4)) {
                        return RegfileExit::fault(code_va + 0x09);
                    }
                    // +0x0c: movsd [rsp], xmm1
                    if !c.movsd_mr(c.r64(4), 1) {
                        return RegfileExit::fault(code_va + 0x0c);
                    }
                    // +0x11: fld qword ptr [rsp]
                    if !fld_m64(&mut *(raw as *mut Cpu), c.r64(4)) {
                        return RegfileExit::fault(code_va + 0x11);
                    }
                    blk = 1;
                    continue;
                }
                1 => {
                    // +0x14: fprem
                    fprem(&mut *(raw as *mut Cpu));
                    // +0x16: fnstsw ax
                    fnstsw_ax(&mut *(raw as *mut Cpu));
                    // +0x18: shr eax, 8
                    let r1 = c.shr(c.r32(0), 8, 32).unwrap_or(c.r32(0));
                    c.w32(0, r1);
                    // +0x1b: and eax, 0xff
                    let r2 = c.r32(0) & 0xff;
                    c.w32(0, r2);
                    c.logic(r2, 32);
                    // +0x20: push rax
                    if !c.push64(c.r64(0)) {
                        return RegfileExit::fault(code_va + 0x20);
                    }
                    // +0x21: popfq
                    if !popfq(&mut *(raw as *mut Cpu)) {
                        return RegfileExit::fault(code_va + 0x21);
                    }
                    // +0x22: jp +0x14
                    if c.cc_p() {
                        blk = 1;
                        continue;
                    }
                    blk = 2;
                    continue;
                }
                2 => {
                    // +0x24: fstp st(1)
                    fstp_st(&mut *(raw as *mut Cpu), 1);
                    // +0x26: fstp qword ptr [rsp]
                    if !fstp_m64(&mut *(raw as *mut Cpu), c.r64(4)) {
                        return RegfileExit::fault(code_va + 0x26);
                    }
                    // +0x29: movsd xmm1, qword ptr [rsp]
                    if !c.movsd_rm(1, c.r64(4)) {
                        return RegfileExit::fault(code_va + 0x29);
                    }
                    // +0x2e: add rsp, 8
                    let r3 = c.add(c.r64(4), 8, 64);
                    c.w64(4, r3);
                    // +0x32: ret
                    return c.ret(0, code_va + 0x32);
                }
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn guest_range_checks_canonical_and_straddle_boundaries() {
        assert!(!is_valid_guest_range(0, 1));
        assert!(!is_valid_guest_range(0xFFFF, 1));
        assert!(is_valid_guest_range(0x1_0000, 1));
        assert!(is_valid_guest_range(0x1_0000, 8));
        assert!(is_valid_guest_range(0x7FFF_FFFF_FFF8, 8));
        assert!(!is_valid_guest_range(0x7FFF_FFFF_FFF9, 8));
        assert!(!is_valid_guest_range(0x7FFF_FFFF_FFFF, 2));
        assert!(is_valid_guest_range(0x7FFF_FFFF_FFFF, 1));
        assert!(!is_valid_guest_range(0x8000_0000_0000, 1));
        assert!(!is_valid_guest_range(u64::MAX - 3, 8));
        assert!(!is_valid_guest_range(0x1_0000, 0));
    }

    #[test]
    fn x87_fprem_hardware_vectors_and_v8_bid421_oracle() {
        // Exact ground-truth vectors cross-checked against x86-64 Rosetta 2 hardware (`fninit; fld b; fld a; fprem` loop):
        // (dividend_a_bits, divisor_b_bits, expected_step1_rem, expected_step1_sw, expected_final_rem, expected_final_sw, expected_iters)
        let vectors: &[(u64, u64, u64, u16, u64, u16, u32)] = &[
            // Quotient Q=1..5 -> (C0=Q2, C3=Q1, C1=Q0)
            (0x4014_0000_0000_0000, 0x4008_0000_0000_0000, 0x4000_0000_0000_0000, 0x3200, 0x4000_0000_0000_0000, 0x3200, 1), // 5.0 % 3.0 = 2.0 (Q=1 -> C1=1)
            (0x401c_0000_0000_0000, 0x4008_0000_0000_0000, 0x3ff0_0000_0000_0000, 0x7000, 0x3ff0_0000_0000_0000, 0x7000, 1), // 7.0 % 3.0 = 1.0 (Q=2 -> C3=1)
            (0x4026_0000_0000_0000, 0x4008_0000_0000_0000, 0x4000_0000_0000_0000, 0x7200, 0x4000_0000_0000_0000, 0x7200, 1), // 11.0 % 3.0 = 2.0 (Q=3 -> C3=1,C1=1)
            (0x402a_0000_0000_0000, 0x4008_0000_0000_0000, 0x3ff0_0000_0000_0000, 0x3100, 0x3ff0_0000_0000_0000, 0x3100, 1), // 13.0 % 3.0 = 1.0 (Q=4 -> C0=1)
            (0x402e_0000_0000_0000, 0x4008_0000_0000_0000, 0x0000_0000_0000_0000, 0x3300, 0x0000_0000_0000_0000, 0x3300, 1), // 15.0 % 3.0 = +0.0 (Q=5 -> C0=1,C1=1)
            // Negative dividend & -0.0 sign preservation
            (0xc02e_0000_0000_0000, 0x4008_0000_0000_0000, 0x8000_0000_0000_0000, 0x3300, 0x8000_0000_0000_0000, 0x3300, 1), // -15.0 % 3.0 = -0.0
            (0xc014_0000_0000_0000, 0xc008_0000_0000_0000, 0xc000_0000_0000_0000, 0x3200, 0xc000_0000_0000_0000, 0x3200, 1), // -5.0 % -3.0 = -2.0
            (0x0000_0000_0000_0000, 0x4008_0000_0000_0000, 0x0000_0000_0000_0000, 0x3000, 0x0000_0000_0000_0000, 0x3000, 1), // +0.0 % 3.0 = +0.0
            (0x8000_0000_0000_0000, 0x4008_0000_0000_0000, 0x8000_0000_0000_0000, 0x3000, 0x8000_0000_0000_0000, 0x3000, 1), // -0.0 % 3.0 = -0.0
            (0x8000_0000_0000_0000, 0xc008_0000_0000_0000, 0x8000_0000_0000_0000, 0x3000, 0x8000_0000_0000_0000, 0x3000, 1), // -0.0 % -3.0 = -0.0
            // ±Inf divisor & ±Inf / ±0.0 invalid operands
            (0x3ff0_0000_0000_0000, 0x7ff0_0000_0000_0000, 0x3ff0_0000_0000_0000, 0x3000, 0x3ff0_0000_0000_0000, 0x3000, 1), // 1.0 % +Inf = 1.0
            (0xbff0_0000_0000_0000, 0xfff0_0000_0000_0000, 0xbff0_0000_0000_0000, 0x3000, 0xbff0_0000_0000_0000, 0x3000, 1), // -1.0 % -Inf = -1.0
            (0x7ff0_0000_0000_0000, 0x4008_0000_0000_0000, 0xfff8_0000_0000_0000, 0x3001, 0xfff8_0000_0000_0000, 0x3001, 1), // +Inf % 3.0 = QNaN Indefinite
            (0xfff0_0000_0000_0000, 0x4008_0000_0000_0000, 0xfff8_0000_0000_0000, 0x3001, 0xfff8_0000_0000_0000, 0x3001, 1), // -Inf % 3.0 = QNaN Indefinite
            (0x4014_0000_0000_0000, 0x0000_0000_0000_0000, 0xfff8_0000_0000_0000, 0x3001, 0xfff8_0000_0000_0000, 0x3001, 1), // 5.0 % +0.0 = QNaN Indefinite
            (0x4014_0000_0000_0000, 0x8000_0000_0000_0000, 0xfff8_0000_0000_0000, 0x3001, 0xfff8_0000_0000_0000, 0x3001, 1), // 5.0 % -0.0 = QNaN Indefinite
            (0x0000_0000_0000_0000, 0x0000_0000_0000_0000, 0xfff8_0000_0000_0000, 0x3001, 0xfff8_0000_0000_0000, 0x3001, 1), // +0.0 % +0.0 = QNaN Indefinite
            // qNaN and sNaN payload propagation & quieting
            (0x7ff8_0000_0000_0001, 0x4008_0000_0000_0000, 0x7ff8_0000_0000_0001, 0x3000, 0x7ff8_0000_0000_0001, 0x3000, 1),
            (0x4008_0000_0000_0000, 0x7ff8_0000_0000_0002, 0x7ff8_0000_0000_0002, 0x3000, 0x7ff8_0000_0000_0002, 0x3000, 1),
            (0x7ff0_0000_0000_0001, 0x4008_0000_0000_0000, 0x7ff8_0000_0000_0001, 0x3001, 0x7ff8_0000_0000_0001, 0x3001, 1), // sNaN(a) quieted
            (0x4008_0000_0000_0000, 0x7ff0_0000_0000_0002, 0x7ff8_0000_0000_0002, 0x3001, 0x7ff8_0000_0000_0002, 0x3001, 1), // sNaN(b) quieted
            (0x7ff8_0000_0000_0011, 0x7ff8_0000_0000_0022, 0x7ff8_0000_0000_0022, 0x3000, 0x7ff8_0000_0000_0022, 0x3000, 1), // larger NaN payload wins
            (0xfff8_0000_0000_0005, 0x7ff8_0000_0000_0005, 0x7ff8_0000_0000_0005, 0x3000, 0x7ff8_0000_0000_0005, 0x3000, 1), // equal NaN payload -> positive wins
            // Multi-step C2=1 partial remainder (`1e308 % 1e-308`, `DBL_MAX % DBL_TRUE_MIN`, exact d=63 vs d=64 boundary)
            (0x7fee_fdcb_a987_6543, 0x0011_2345_6789_abcd, 0x7be5_e4da_95e4_f088, 0x3400, 0x0000_ade1_548a_6974, 0x3100, 33), // 33-step C2=1 80-bit reduction!
            (0x7fef_ffff_ffff_ffff, 0x0000_0000_0000_0001, 0x7ccc_0000_0000_0000, 0x3402, 0x0000_0000_0000_0000, 0x3002, 3), // DBL_MAX % 5e-324 (step 1: n=49 -> 0x7ccc000000000000, step 2: rem=0 C2=1, step 3: Zero -> C2=0)
            (0x43e0_0000_0000_0000, 0x3ff0_0000_0000_0000, 0x0000_0000_0000_0000, 0x3000, 0x0000_0000_0000_0000, 0x3000, 1),  // d=63: single step (C2=0)
            (0x43f0_0000_0000_0000, 0x3ff0_0000_0000_0000, 0x0000_0000_0000_0000, 0x3400, 0x0000_0000_0000_0000, 0x3000, 2),  // d=64: C2=1 on step 1, done on step 2
            // Subnormals
            (0x0000_0000_0000_0007, 0x0000_0000_0000_0003, 0x0000_0000_0000_0001, 0x7002, 0x0000_0000_0000_0001, 0x7002, 1),
            (0x8000_0000_0000_0006, 0x0000_0000_0000_0003, 0x8000_0000_0000_0000, 0x7002, 0x8000_0000_0000_0000, 0x7002, 1),
        ];

        let mut code_buf = vec![0u8; 256];
        code_buf[..V8_BID421_FPREM_LOOP.len()].copy_from_slice(V8_BID421_FPREM_LOOP);
        let code_va = code_buf.as_ptr() as u64;
        let lo = code_va;
        let hi = code_va + V8_BID421_FPREM_LOOP.len() as u64;

        // Single 512B-offset-paired allocation (`+0x000` vs `+0x200`) so `add rsp, 8` sees identical low-byte parity (`PF`) on both sides.
        let mut stacks = vec![0u64; 128];
        let (stack_g, stack_l) = stacks.split_at_mut(64);

        for (vi, &(a_bits, b_bits, want_r1, want_sw1, want_rf, want_swf, want_iters)) in vectors.iter().enumerate() {
            // 1. Direct X87State step-1 and multi-step verification against Rosetta 2 hardware ground truth:
            let mut st1 = X87State::new();
            st1.push_f64_bits(b_bits);
            st1.push_f64_bits(a_bits);
            st1.fprem();
            assert_eq!(st1.status_word(), want_sw1, "vec {vi}: step1 SW");
            st1.pop_to_st(1);
            let got_r1 = st1.pop_f64_bits();
            assert_eq!(got_r1, want_r1, "vec {vi}: step1 rem");

            let mut stf = X87State::new();
            stf.push_f64_bits(b_bits);
            stf.push_f64_bits(a_bits);
            let mut iters = 0u32;
            loop {
                iters += 1;
                stf.fprem();
                if (stf.status_word() & (1 << 10)) == 0 {
                    break;
                }
            }
            assert_eq!(iters, want_iters, "vec {vi}: fprem loop iteration count");
            assert_eq!(stf.status_word(), want_swf, "vec {vi}: final SW");
            stf.pop_to_st(1);
            let got_rf = stf.pop_f64_bits();
            assert_eq!(got_rf, want_rf, "vec {vi}: final rem");
            assert_eq!(stf.top, 0, "vec {vi}: stack balanced back to TOP=0");

            // 2. Full instruction-stream differential test on real V8 `bid=421` bytes (`emu::Cpu` vs `regfile_ext`):
            for (i, w) in stack_g.iter_mut().enumerate() {
                *w = 0xA5A5_0000_0000_0000u64.wrapping_add(i as u64);
            }
            stack_g[40] = 0xDEAD_BEEF_CAFE_0000;
            stack_l.copy_from_slice(&stack_g);

            let mut cpu_g = Cpu::new_zeroed();
            cpu_g.gpr[0] = 0x1122_3344_5566_7788;
            cpu_g.gpr[4] = unsafe { stack_g.as_mut_ptr().add(40) } as u64;
            cpu_g.xmm[0] = (b_bits as u128) | (0xCAFE_BABE_0000_0001u128 << 64);
            cpu_g.xmm[1] = (a_bits as u128) | (0xCAFE_BABE_0000_0002u128 << 64);
            cpu_g.set_rflags(0x8D5);

            let mut cpu_l = cpu_g.clone();
            cpu_l.gpr[4] = unsafe { stack_l.as_mut_ptr().add(40) } as u64;

            unsafe {
                x87_reset();
                let ex_g = run_oracle_with_x87(&mut cpu_g, lo, hi);
                let x87_g = x87_snapshot();

                x87_reset();
                let ex_l = lowered_bid421_fprem_loop(&mut cpu_l, code_va);
                let x87_l = x87_snapshot();

                assert_eq!(ex_g, RegfileExit::ret(0xDEAD_BEEF_CAFE_0000));
                // Compare rsp offsets relative to each side's own stack buffer, then normalize rsp for full Cpu equality.
                let sp_off_g = cpu_g.gpr[4] - (stack_g.as_ptr() as u64);
                let sp_off_l = cpu_l.gpr[4] - (stack_l.as_ptr() as u64);
                assert_eq!(sp_off_g, sp_off_l, "vec {vi}: final rsp offset");
                cpu_l.gpr[4] = cpu_g.gpr[4];
                assert_cpu_bit_exact(&format!("bid421 vec {vi}"), ex_l, ex_g, &cpu_l, &cpu_g);
                assert_eq!(x87_l, x87_g, "vec {vi}: post-builtin X87State");
                assert_eq!(stack_l, stack_g, "vec {vi}: stack memory bytes");
                assert_eq!(cpu_l.xmm[1] as u64, want_rf, "vec {vi}: xmm1 low f64 == hardware remainder");
                assert_eq!(cpu_l.xmm[1] >> 64, 0, "vec {vi}: movsd xmm1, [rsp] zeroes bits 127:64");
            }
        }
    }

    #[test]
    fn atomic_v8_builtins_538_to_545_aligned_and_misaligned_oracle() {
        // Allocate an unaligned-capable scratch buffer with guard slack on both sides.
        let mut mem_g = vec![0u8; 512];
        let mut mem_l = vec![0u8; 512];

        // Test every alignment offset `0..=7` (covering aligned AND misaligned 16/32/64-bit guest
        // accesses, including cross-qword and cross-cacheline-style odd offsets) and both CAS hit & miss!
        for align_off in 0usize..8 {
            for hit in [false, true] {
                for rflags_in in [0x202u32, 0xAC7u32] {
                    let seed_buf = |buf: &mut [u8]| {
                        for (i, b) in buf.iter_mut().enumerate() {
                            *b = (0x5Au8).wrapping_add((i as u8).wrapping_mul(37));
                        }
                    };

                    // --- 1. `lock cmpxchg` across 8/16/32/64-bit real V8 `bid=540` slices ---
                    let cmpxchg_cases: &[(&str, &[u8], u32)] = &[
                        ("bid540_cmpxchg8_s", V8_BID540_CMPXCHG8_S, 8),
                        ("bid540_cmpxchg8_u", V8_BID540_CMPXCHG8_U, 8),
                        ("bid540_cmpxchg16_s", V8_BID540_CMPXCHG16_S, 16),
                        ("bid540_cmpxchg32_s", V8_BID540_CMPXCHG32_S, 32),
                        ("bid540_cmpxchg64", V8_BID540_CMPXCHG64, 64),
                    ];

                    for &(cname, code, bits) in cmpxchg_cases {
                        seed_buf(&mut mem_g);
                        mem_l.copy_from_slice(&mem_g);
                        let target_off = 128 + align_off;
                        let addr_g = unsafe { mem_g.as_mut_ptr().add(target_off) } as u64;
                        let addr_l = unsafe { mem_l.as_mut_ptr().add(target_off) } as u64;

                        let mem_val: u64 = 0xFEDC_BA98_8765_4381;
                        unsafe {
                            core::ptr::write_unaligned(addr_g as *mut u64, mem_val);
                            core::ptr::write_unaligned(addr_l as *mut u64, mem_val);
                        }
                        let mask = if bits == 64 { u64::MAX } else { (1u64 << bits) - 1 };
                        let rax_in = if hit {
                            0x9988_7766_0000_0000 | (mem_val & mask)
                        } else {
                            0x9988_7766_0000_0000 | ((mem_val ^ 0x55) & mask)
                        };
                        let rcx_desired: u64 = 0x1357_9BDF_2468_ACE3;

                        let mut stack_g = [0xDEAD_BEEF_CAFE_0000u64; 4];
                        let mut stack_l = [0xDEAD_BEEF_CAFE_0000u64; 4];

                        let mut cpu_g = Cpu::new_zeroed();
                        cpu_g.gpr[0] = rax_in;
                        cpu_g.gpr[1] = rcx_desired;
                        cpu_g.gpr[4] = stack_g.as_mut_ptr() as u64;
                        cpu_g.gpr[7] = addr_g - (align_off as u64); // rdi
                        cpu_g.gpr[8] = align_off as u64;            // r8
                        cpu_g.gpr[12] = align_off as u64;           // r12
                        cpu_g.set_rflags(rflags_in);

                        let mut cpu_l = cpu_g.clone();
                        cpu_l.gpr[4] = stack_l.as_mut_ptr() as u64;
                        cpu_l.gpr[7] = addr_l - (align_off as u64);

                        let code_va = code.as_ptr() as u64;
                        unsafe {
                            let ex_g = run_oracle_with_x87(&mut cpu_g, code_va, code_va + code.len() as u64);

                            // Run lowered `regfile_ext` equivalent on `cpu_l`:
                            let raw = &mut cpu_l as *mut Cpu as *mut u8;
                            let mut c = ctx(raw);
                            let ea = c.r64(7).wrapping_add(if bits == 8 { c.r64(12) } else { c.r64(8) });
                            let ok = match bits {
                                8 => lock_cmpxchg_8(&mut *(raw as *mut Cpu), ea, c.r8(1)),
                                16 => lock_cmpxchg_16(&mut *(raw as *mut Cpu), ea, c.r16(1)),
                                32 => lock_cmpxchg_32(&mut *(raw as *mut Cpu), ea, c.r32(1)),
                                _ => lock_cmpxchg_64(&mut *(raw as *mut Cpu), ea, c.r64(1)),
                            };
                            assert!(ok);
                            match cname {
                                "bid540_cmpxchg8_s" => {
                                    let sx = c.r8(0) as u8 as i8 as i64 as u64;
                                    c.w32(0, sx);
                                    let sh = c.shl(c.r32(0), 1, 32).unwrap();
                                    c.w32(0, sh);
                                }
                                "bid540_cmpxchg8_u" => {
                                    let zx = c.r8(0);
                                    c.w32(0, zx);
                                    let sh = c.shl(c.r32(0), 1, 32).unwrap();
                                    c.w32(0, sh);
                                }
                                "bid540_cmpxchg16_s" => {
                                    let sx = c.r16(0) as u16 as i16 as i64 as u64;
                                    c.w32(0, sx);
                                    let sh = c.shl(c.r32(0), 1, 32).unwrap();
                                    c.w32(0, sh);
                                }
                                "bid540_cmpxchg32_s" => {
                                    c.w64(1, c.r64(0));
                                    let s = c.add(c.r32(1), c.r32(0), 32);
                                    c.w32(1, s);
                                }
                                "bid540_cmpxchg64" => {
                                    c.cmp(c.r64(0), 0, 64);
                                }
                                _ => unreachable!(),
                            }
                            let ex_l = c.ret(0, code_va + code.len() as u64 - 1);

                            cpu_l.gpr[4] = cpu_g.gpr[4];
                            cpu_l.gpr[7] = cpu_g.gpr[7];
                            assert_cpu_bit_exact(
                                &format!("{cname} align={align_off} hit={hit} fl={rflags_in:#x}"),
                                ex_l,
                                ex_g,
                                &cpu_l,
                                &cpu_g,
                            );
                            assert_eq!(mem_l, mem_g, "{cname} align={align_off} hit={hit}: memory bytes");
                        }
                    }

                    // --- 2. `xchg [mem], reg` across 8/16/32/64-bit real V8 `bid=538` & `bid=539` slices ---
                    // Verify RFLAGS stay 100% untouched on `xchg` (`bid=538` 32/64-bit have no flag-clobbering trailer!).
                    let xchg_cases: &[(&str, &[u8], u32)] = &[
                        ("bid538_xchg8", V8_BID538_XCHG8, 8),
                        ("bid538_xchg16", V8_BID538_XCHG16, 16),
                        ("bid538_xchg32", V8_BID538_XCHG32, 32),
                        ("bid538_xchg64", V8_BID538_XCHG64, 64),
                        ("bid539_xchg8_s", V8_BID539_XCHG8_S, 8),
                        ("bid539_xchg16_s", V8_BID539_XCHG16_S, 16),
                        ("bid539_xchg32_s", V8_BID539_XCHG32_S, 32),
                        ("bid539_xchg64", V8_BID539_XCHG64, 64),
                    ];

                    for &(xname, code, _bits) in xchg_cases {
                        seed_buf(&mut mem_g);
                        mem_l.copy_from_slice(&mem_g);
                        let target_off = 192 + align_off;
                        let addr_g = unsafe { mem_g.as_mut_ptr().add(target_off) } as u64;
                        let addr_l = unsafe { mem_l.as_mut_ptr().add(target_off) } as u64;

                        let mut stack_g = [0xDEAD_BEEF_CAFE_0000u64; 4];
                        let mut stack_l = [0xDEAD_BEEF_CAFE_0000u64; 4];

                        let mut cpu_g = Cpu::new_zeroed();
                        cpu_g.gpr[1] = if xname.starts_with("bid538_xchg1") || xname == "bid538_xchg32" {
                            align_off as u64
                        } else {
                            0xABCD_EF01_2345_6789
                        };
                        cpu_g.gpr[2] = addr_g - (align_off as u64); // rdx
                        cpu_g.gpr[4] = stack_g.as_mut_ptr() as u64;
                        cpu_g.gpr[7] = addr_g - (align_off as u64); // rdi
                        cpu_g.gpr[8] = align_off as u64;            // r8
                        cpu_g.gpr[9] = align_off as u64;            // r9
                        cpu_g.gpr[11] = 0x7788_99AA_BBCC_DD83;      // r11
                        cpu_g.gpr[12] = align_off as u64;           // r12
                        cpu_g.set_rflags(rflags_in);

                        let mut cpu_l = cpu_g.clone();
                        cpu_l.gpr[2] = addr_l - (align_off as u64);
                        cpu_l.gpr[4] = stack_l.as_mut_ptr() as u64;
                        cpu_l.gpr[7] = addr_l - (align_off as u64);

                        let code_va = code.as_ptr() as u64;
                        unsafe {
                            let ex_g = run_oracle_with_x87(&mut cpu_g, code_va, code_va + code.len() as u64);

                            let raw = &mut cpu_l as *mut Cpu as *mut u8;
                            let mut c = ctx(raw);
                            match xname {
                                "bid538_xchg8" => {
                                    let ea = c.r64(2).wrapping_add(c.r64(9));
                                    let old = xchg_mem_8(&mut *(raw as *mut Cpu), ea, c.r8(11)).unwrap();
                                    c.w8(11, old);
                                    c.w32(11, c.r8(11) as u8 as i8 as i64 as u64);
                                }
                                "bid538_xchg16" => {
                                    let ea = c.r64(2).wrapping_add(c.r64(1));
                                    let old = xchg_mem_16(&mut *(raw as *mut Cpu), ea, c.r16(11)).unwrap();
                                    c.w16(11, old);
                                    c.w32(11, c.r16(11) as u16 as i16 as i64 as u64);
                                }
                                "bid538_xchg32" => {
                                    let ea = c.r64(2).wrapping_add(c.r64(1));
                                    let old = xchg_mem_32(&mut *(raw as *mut Cpu), ea, c.r32(11)).unwrap();
                                    c.w32(11, old);
                                    assert_eq!(cpu_l.rflags(), rflags_in, "xchg32 must leave RFLAGS untouched");
                                }
                                "bid538_xchg64" => {
                                    let ea = c.r64(7).wrapping_add(c.r64(8));
                                    let old = xchg_mem_64(&mut *(raw as *mut Cpu), ea, c.r64(1)).unwrap();
                                    c.w64(1, old);
                                    assert_eq!(cpu_l.rflags(), rflags_in, "xchg64 must leave RFLAGS untouched");
                                }
                                "bid539_xchg8_s" => {
                                    let ea = c.r64(7).wrapping_add(c.r64(12));
                                    let old = xchg_mem_8(&mut *(raw as *mut Cpu), ea, c.r8(1)).unwrap();
                                    c.w8(1, old);
                                    c.w32(1, c.r8(1) as u8 as i8 as i64 as u64);
                                    c.w32(0, c.r64(1).wrapping_add(c.r64(1)));
                                }
                                "bid539_xchg16_s" => {
                                    let ea = c.r64(7).wrapping_add(c.r64(8));
                                    let old = xchg_mem_16(&mut *(raw as *mut Cpu), ea, c.r16(1)).unwrap();
                                    c.w16(1, old);
                                    c.w32(1, c.r16(1) as u16 as i16 as i64 as u64);
                                    c.w32(0, c.r64(1).wrapping_add(c.r64(1)));
                                }
                                "bid539_xchg32_s" => {
                                    let ea = c.r64(8).wrapping_add(c.r64(7));
                                    let old = xchg_mem_32(&mut *(raw as *mut Cpu), ea, c.r32(1)).unwrap();
                                    c.w32(1, old);
                                    c.w64(7, c.r64(1));
                                    let s = c.add(c.r32(7), c.r32(1), 32);
                                    c.w32(7, s);
                                }
                                "bid539_xchg64" => {
                                    let ea = c.r64(7).wrapping_add(c.r64(8));
                                    let old = xchg_mem_64(&mut *(raw as *mut Cpu), ea, c.r64(1)).unwrap();
                                    c.w64(1, old);
                                    c.cmp(c.r64(1), 0, 64);
                                }
                                _ => unreachable!(),
                            }
                            let ex_l = c.ret(0, code_va + code.len() as u64 - 1);

                            cpu_l.gpr[2] = cpu_g.gpr[2];
                            cpu_l.gpr[4] = cpu_g.gpr[4];
                            if xname != "bid539_xchg32_s" {
                                cpu_l.gpr[7] = cpu_g.gpr[7];
                            }
                            assert_cpu_bit_exact(
                                &format!("{xname} align={align_off} fl={rflags_in:#x}"),
                                ex_l,
                                ex_g,
                                &cpu_l,
                                &cpu_g,
                            );
                            assert_eq!(mem_l, mem_g, "{xname} align={align_off}: memory bytes");
                        }
                    }
                }
            }

            // --- 3. CAS retry loops from `bid=541..545` (`AtomicsAdd`, `Sub`, `And`, `Or`, `Xor`) ---
            let cas_loop_cases: &[(&str, &[u8], u32)] = &[
                ("bid541_add8", V8_BID541_ADD_CAS8, 8),
                ("bid541_add16", V8_BID541_ADD_CAS16, 16),
                ("bid541_add32", V8_BID541_ADD_CAS32, 32),
                ("bid541_add64", V8_BID541_ADD_CAS64, 64),
                ("bid542_sub8", V8_BID542_SUB_CAS8, 8),
                ("bid543_and8", V8_BID543_AND_CAS8, 8),
                ("bid544_or8", V8_BID544_OR_CAS8, 8),
                ("bid545_xor8", V8_BID545_XOR_CAS8, 8),
            ];

            for &(lname, code, bits) in cas_loop_cases {
                for (i, b) in mem_g.iter_mut().enumerate() {
                    *b = (0xA3u8).wrapping_add((i as u8).wrapping_mul(19));
                }
                mem_l.copy_from_slice(&mem_g);
                let target_off = 256 + align_off;
                let addr_g = unsafe { mem_g.as_mut_ptr().add(target_off) } as u64;
                let addr_l = unsafe { mem_l.as_mut_ptr().add(target_off) } as u64;

                let mut stack_g = [0xDEAD_BEEF_CAFE_0000u64; 4];
                let mut stack_l = [0xDEAD_BEEF_CAFE_0000u64; 4];

                let mut cpu_g = Cpu::new_zeroed();
                cpu_g.gpr[0] = 0xFFFF_EEEE_DDDD_CCCC;
                cpu_g.gpr[1] = 0x0123_4567_89AB_CD37;
                cpu_g.gpr[4] = stack_g.as_mut_ptr() as u64;
                cpu_g.gpr[7] = addr_g - (align_off as u64);
                cpu_g.gpr[8] = align_off as u64;
                cpu_g.gpr[12] = align_off as u64;
                cpu_g.set_rflags(0x246);

                let mut cpu_l = cpu_g.clone();
                cpu_l.gpr[4] = stack_l.as_mut_ptr() as u64;
                cpu_l.gpr[7] = addr_l - (align_off as u64);

                let code_va = code.as_ptr() as u64;
                unsafe {
                    let ex_g = run_oracle_with_x87(&mut cpu_g, code_va, code_va + code.len() as u64);

                    let raw = &mut cpu_l as *mut Cpu as *mut u8;
                    let mut c = ctx(raw);
                    loop {
                        match bits {
                            8 => {
                                let ea = c.r64(7).wrapping_add(c.r64(12));
                                let v = c.ld8(ea).unwrap();
                                c.w8(0, v);
                                c.w32(8, c.r32(0));
                                let nr = match lname {
                                    "bid541_add8" => c.add(c.r32(8), c.r32(1), 32),
                                    "bid542_sub8" => c.sub(c.r32(8), c.r32(1), 32),
                                    "bid543_and8" => {
                                        let r = c.r32(8) & c.r32(1);
                                        c.logic(r, 32);
                                        r
                                    }
                                    "bid544_or8" => {
                                        let r = c.r32(8) | c.r32(1);
                                        c.logic(r, 32);
                                        r
                                    }
                                    "bid545_xor8" => {
                                        let r = c.r32(8) ^ c.r32(1);
                                        c.logic(r, 32);
                                        r
                                    }
                                    _ => unreachable!(),
                                };
                                c.w32(8, nr);
                                assert!(lock_cmpxchg_8(&mut *(raw as *mut Cpu), ea, c.r8(8)));
                                if c.cc_ne() {
                                    continue;
                                }
                                if lname == "bid541_add8" {
                                    c.w32(0, c.r8(0) as u8 as i8 as i64 as u64);
                                    let sh = c.shl(c.r32(0), 1, 32).unwrap();
                                    c.w32(0, sh);
                                }
                                break;
                            }
                            16 => {
                                let ea = c.r64(7).wrapping_add(c.r64(8));
                                let v = c.ld16(ea).unwrap();
                                c.w16(0, v);
                                c.w32(9, c.r32(0));
                                let nr = c.add(c.r32(9), c.r32(1), 32);
                                c.w32(9, nr);
                                assert!(lock_cmpxchg_16(&mut *(raw as *mut Cpu), ea, c.r16(9)));
                                if c.cc_ne() {
                                    continue;
                                }
                                c.w32(0, c.r16(0) as u16 as i16 as i64 as u64);
                                let sh = c.shl(c.r32(0), 1, 32).unwrap();
                                c.w32(0, sh);
                                break;
                            }
                            32 => {
                                let ea = c.r64(8).wrapping_add(c.r64(7));
                                let v = c.ld32(ea).unwrap();
                                c.w32(0, v);
                                c.w32(9, c.r32(0));
                                let nr = c.add(c.r32(9), c.r32(1), 32);
                                c.w32(9, nr);
                                assert!(lock_cmpxchg_32(&mut *(raw as *mut Cpu), ea, c.r32(9)));
                                if c.cc_ne() {
                                    continue;
                                }
                                c.w64(1, c.r64(0));
                                let s = c.add(c.r32(1), c.r32(0), 32);
                                c.w32(1, s);
                                break;
                            }
                            _ => {
                                let ea = c.r64(7).wrapping_add(c.r64(8));
                                let v = c.ld64(ea).unwrap();
                                c.w64(0, v);
                                c.w64(9, c.r64(0));
                                let nr = c.add(c.r64(9), c.r64(1), 64);
                                c.w64(9, nr);
                                assert!(lock_cmpxchg_64(&mut *(raw as *mut Cpu), ea, c.r64(9)));
                                if c.cc_ne() {
                                    continue;
                                }
                                c.cmp(c.r64(0), 0, 64);
                                break;
                            }
                        }
                    }
                    let ex_l = c.ret(0, code_va + code.len() as u64 - 1);

                    cpu_l.gpr[4] = cpu_g.gpr[4];
                    cpu_l.gpr[7] = cpu_g.gpr[7];
                    assert_cpu_bit_exact(&format!("{lname} align={align_off}"), ex_l, ex_g, &cpu_l, &cpu_g);
                    assert_eq!(mem_l, mem_g, "{lname} align={align_off}: memory bytes");
                }
            }
        }
    }

    #[test]
    fn fault_rollback_leaves_cpu_flags_and_x87_stack_untouched() {
        let bad_addrs: &[u64] = &[
            0x0,
            0xDEAD,
            0xFFFF,
            0x7FFF_FFFF_FFFA, // straddles 0x8000_0000_0000 for 8-byte access
            0x7FFF_FFFF_FFFF, // straddles 0x8000_0000_0000 for 2/4/8-byte access
            0x8000_0000_0000,
            u64::MAX - 3,
        ];

        for &bad in bad_addrs {
            // 1. Atomic `lock_cmpxchg_{8,16,32,64}` & `xchg_mem_{8,16,32,64}` fault rollback vs `run_oracle_with_x87`
            for &(code, bits, is_cmpxchg) in &[
                (V8_BID540_CMPXCHG8_S, 8u32, true),
                (V8_BID540_CMPXCHG16_S, 16u32, true),
                (V8_BID540_CMPXCHG32_S, 32u32, true),
                (V8_BID540_CMPXCHG64, 64u32, true),
                (V8_BID538_XCHG8, 8u32, false),
                (V8_BID538_XCHG16, 16u32, false),
                (V8_BID538_XCHG32, 32u32, false),
                (V8_BID538_XCHG64, 64u32, false),
            ] {
                let bytes = (bits / 8) as usize;
                if is_valid_guest_range(bad, bytes) {
                    continue; // e.g. `0x7FFF_FFFF_FFFF` is valid for 1-byte access (`bytes == 1`)!
                }
                let mut cpu_g = Cpu::new_zeroed();
                cpu_g.gpr[0] = 0x1111_2222_3333_4444;
                cpu_g.gpr[1] = 0;
                cpu_g.gpr[2] = bad;
                cpu_g.gpr[7] = bad;
                cpu_g.gpr[8] = 0;
                cpu_g.gpr[9] = 0;
                cpu_g.gpr[11] = 0x5555_6666_7777_8888;
                cpu_g.gpr[12] = 0;
                cpu_g.last_bits = [8, 16, 64, 64, 64, 64, 64, 64, 64, 64, 64, 16, 64, 64, 64, 64];
                cpu_g.set_rflags(0xAD5);

                let pre_snap = cpu_g.clone();
                let mut cpu_l = cpu_g.clone();
                let code_va = code.as_ptr() as u64;

                unsafe {
                    let ex_g = run_oracle_with_x87(&mut cpu_g, code_va, code_va + code.len() as u64);
                    let ex_l = if is_cmpxchg {
                        let ok = match bits {
                            8 => lock_cmpxchg_8(&mut cpu_l, bad, 0),
                            16 => lock_cmpxchg_16(&mut cpu_l, bad, 0),
                            32 => lock_cmpxchg_32(&mut cpu_l, bad, 0),
                            _ => lock_cmpxchg_64(&mut cpu_l, bad, 0),
                        };
                        assert!(!ok);
                        RegfileExit::fault(code_va)
                    } else {
                        let res = match bits {
                            8 => xchg_mem_8(&mut cpu_l, bad, 0x88),
                            16 => xchg_mem_16(&mut cpu_l, bad, 0x7788),
                            32 => xchg_mem_32(&mut cpu_l, bad, 0x6677_8899),
                            _ => xchg_mem_64(&mut cpu_l, bad, 0x5566_7788_99AA_BBCC),
                        };
                        assert!(res.is_none());
                        RegfileExit::fault(code_va)
                    };

                    assert_eq!(ex_g.kind, RF_FAULT);
                    assert_cpu_bit_exact(
                        &format!("atomic fault bad={bad:#x} bits={bits} cas={is_cmpxchg}"),
                        ex_l,
                        ex_g,
                        &cpu_l,
                        &cpu_g,
                    );
                    assert_eq!(cpu_l.gpr, pre_snap.gpr);
                    assert_eq!(cpu_l.rflags(), pre_snap.rflags());
                    assert_eq!(cpu_l.last_bits, pre_snap.last_bits);
                }
            }

            // 2. x87 `fld_m64`, `fstp_m64`, and `popfq` fault rollback (verifies `fstp_m64` does NOT pop ST(0) on fault!)
            if !is_valid_guest_range(bad, 8) {
                unsafe {
                    x87_reset();
                    let mut st_seed = X87State::new();
                    st_seed.push_f64_bits(3.5f64.to_bits());
                    x87_restore(st_seed);

                    let mut cpu = Cpu::new_zeroed();
                    cpu.gpr[4] = bad;
                    cpu.set_rflags(0x246);
                    let pre_cpu = cpu.clone();

                    assert!(!fld_m64(&mut cpu, bad));
                    assert_eq!(x87_snapshot(), st_seed, "fld_m64 fault must not push onto x87 stack");

                    assert!(!fstp_m64(&mut cpu, bad));
                    assert_eq!(x87_snapshot(), st_seed, "fstp_m64 fault must not pop ST(0)");

                    assert!(!popfq(&mut cpu));
                    assert_eq!(cpu.gpr, pre_cpu.gpr, "popfq fault must not advance rsp");
                    assert_eq!(cpu.rflags(), pre_cpu.rflags(), "popfq fault must not mutate RFLAGS");
                }
            }
        }

        // 3. 1-instruction `int3` trap stub (representing the 5 `v8_nonabi_trap_stub` builtins:
        //    `117`, `119`, `581`, `582`, `627`, e.g. `[0xcc, 0x90, 0x66, 0x90]`)
        let trap_stub: [u8; 4] = [0xcc, 0x90, 0x66, 0x90];
        let trap_va = trap_stub.as_ptr() as u64;
        let mut cpu_g = Cpu::new_zeroed();
        cpu_g.gpr[0] = 0x42;
        let cpu_l = cpu_g.clone();
        unsafe {
            let ex_g = run_oracle_with_x87(&mut cpu_g, trap_va, trap_va + 4);
            let ex_l = RegfileExit::fault(trap_va);
            assert_eq!(ex_g, RegfileExit::fault(trap_va));
            assert_cpu_bit_exact("v8_nonabi_trap_stub", ex_l, ex_g, &cpu_l, &cpu_g);
            assert!(
                ex_l.kind != RF_RET && ex_l.kind != RF_JUMP,
                "trap stub must return RF_FAULT(ins_va)"
            );
        }
    }

    #[test]
    fn rf_wrapper_and_run_oracle_end_to_end_parity() {
        use crate::emu::regfile::{arm_skip, run_oracle, take_skip};

        // 1. `run_oracle` vs `Rf` x87/popfq methods on real `V8_BID421_FPREM_LOOP` (33-step C2=1 case)
        let mut code_buf = vec![0u8; 128];
        code_buf[..V8_BID421_FPREM_LOOP.len()].copy_from_slice(V8_BID421_FPREM_LOOP);
        let code_va = code_buf.as_ptr() as u64;
        let mut stacks = vec![0u64; 128];
        let (stack_g, stack_l) = stacks.split_at_mut(64);
        stack_g[40] = 0xDEAD_BEEF_CAFE_0000;
        stack_l.copy_from_slice(stack_g);

        let mut cpu_g = Cpu::new_zeroed();
        cpu_g.gpr[4] = unsafe { stack_g.as_mut_ptr().add(40) } as u64;
        cpu_g.xmm[0] = 0x0011_2345_6789_abcdu128;
        cpu_g.xmm[1] = 0x7fee_fdcb_a987_6543u128;
        let mut cpu_l = cpu_g.clone();
        cpu_l.gpr[4] = unsafe { stack_l.as_mut_ptr().add(40) } as u64;

        unsafe {
            let ex_g = run_oracle(&mut cpu_g, code_va, code_va, code_va + V8_BID421_FPREM_LOOP.len() as u64);
            let x87_g = x87_snapshot();

            x87_reset();
            let raw = &mut cpu_l as *mut Cpu as *mut u8;
            let mut c = ctx(raw);
            let r0 = c.sub(c.r64(4), 8, 64);
            c.w64(4, r0);
            assert!(c.movsd_mr(c.r64(4), 0));
            assert!(c.fld_m64(c.r64(4)));
            assert!(c.movsd_mr(c.r64(4), 1));
            assert!(c.fld_m64(c.r64(4)));
            loop {
                c.fprem();
                c.fnstsw_ax();
                let r1 = c.shr(c.r32(0), 8, 32).unwrap_or(c.r32(0));
                c.w32(0, r1);
                let r2 = c.r32(0) & 0xff;
                c.w32(0, r2);
                c.logic(r2, 32);
                assert!(c.push64(c.r64(0)));
                assert!(c.popfq());
                if !c.cc_p() {
                    break;
                }
            }
            c.fstp_st(1);
            assert!(c.fstp_m64(c.r64(4)));
            assert!(c.movsd_rm(1, c.r64(4)));
            let r3 = c.add(c.r64(4), 8, 64);
            c.w64(4, r3);
            let ex_l = c.ret(0, code_va + 0x32);
            let x87_l = x87_snapshot();

            cpu_l.gpr[4] = cpu_g.gpr[4];
            assert_cpu_bit_exact("rf_wrapper_bid421", ex_l, ex_g, &cpu_l, &cpu_g);
            assert_eq!(x87_l, x87_g);
        }

        // 2. `Rf::lock_cmpxchg` + `Rf::xchg_mem` + straddle fault parity with `run_oracle`
        let bad_straddle = 0x7FFF_FFFF_FFFCu64;
        let mut cpu_g2 = Cpu::new_zeroed();
        cpu_g2.gpr[7] = bad_straddle;
        let mut cpu_l2 = cpu_g2.clone();
        let cas_va = V8_BID540_CMPXCHG64.as_ptr() as u64;
        unsafe {
            let ex_g = run_oracle(&mut cpu_g2, cas_va, cas_va, cas_va + V8_BID540_CMPXCHG64.len() as u64);
            let mut c = ctx(&mut cpu_l2 as *mut Cpu as *mut u8);
            assert!(!c.lock_cmpxchg(bad_straddle, 1, 64));
            assert!(c.xchg_mem(bad_straddle, 1, 64).is_none());
            assert_cpu_bit_exact("rf_straddle_fault", RegfileExit::fault(cas_va), ex_g, &cpu_l2, &cpu_g2);
        }

        // 3. `arm_skip` / `take_skip` one-shot guard on `int3` trap stub
        let trap: [u8; 4] = [0xcc, 0x90, 0x90, 0x90];
        let t_va = trap.as_ptr() as u64;
        let mut cpu_t = Cpu::new_zeroed();
        unsafe {
            let ex = run_oracle(&mut cpu_t, t_va, t_va, t_va + 4);
            assert_eq!(ex, RegfileExit::fault(t_va));
            arm_skip(ex.va);
            assert!(take_skip(t_va));
            assert!(!take_skip(t_va));
        }
    }

    #[test]
    fn x87_extended_ops_fdivr_ffreep_fistp_fstp_fxam_fnclex() {
        use iced_x86::code_asm::*;

        // 1. Test `fdivr` (mem32/mem64/reg), `fdivrp`, `fidivr`, `ffree`, `ffreep`,
        //    `fist`/`fistp`/`fisttp` (16/32/64-bit, rounding, truncation, overflow indefinite),
        //    `fstp` (m32/m64/m80/reg), `fld` (m80/reg/const), `fild`, `fxch`, `fcomi`/`fcomip`.
        let mut a = CodeAssembler::new(64).unwrap();
        // Work buffer in rcx:
        //   [0x00]: f64 = 4.0
        //   [0x08]: f64 = 10.0
        //   [0x10]: f32 = 20.0
        //   [0x14]: i32 = 60
        //   [0x18]: f64 = 2.5
        //   [0x20]: f64 = -3.7
        //   [0x28]: f64 = 1e20 (overflow for i32)
        //   [0x30]: i64 = -9_000_000_000_000_000_000 (tests exact 64-bit fild -> fistp round-trip!)
        // Outputs:
        //   [0x40]: f64 fdivr m64 (10.0 / 4.0 = 2.5)
        //   [0x48]: f64 fdivr m32 (20.0 / 2.5 = 8.0)
        //   [0x50]: f64 fidivr m32 (60.0 / 8.0 = 7.5)
        //   [0x58]: f64 fdivrp st(1), st(0) (3.0 / 12.0 = 0.25) -> wait: push 12.0, push 3.0 -> ST0=3, ST1=12 -> fdivrp st1,st0 -> ST1=ST0/ST1 = 3/12 = 0.25!
        //   [0x60]: i16 fistp(2.5) = 2 (ties to even!)
        //   [0x62]: i16 fistp(-3.7) = -4
        //   [0x64]: i32 fisttp(-3.7) = -3 (truncate toward zero!)
        //   [0x68]: i32 fistp(1e20) = i32::MIN (0x8000_0000 indefinite!)
        //   [0x70]: i64 fild(m64) -> fistp(m64) exact 64-bit round-trip
        //   [0x78]: m80 -> m80 -> f64 round-trip
        //   [0x88]: u16 fxam SW for -0.0 (C3=1, C1=1 -> 0x4200 in mask 0x4700)
        //   [0x8A]: u16 fxam SW for empty slot after ffreep (C3=1, C0=1 -> 0x4100 in mask 0x4700)
        //   [0x8C]: u16 SW after fnclex (low 8 bits + bit 15 must be 0)
        a.fninit().unwrap();
        // fdivr m64: ST0 = 4.0; fdivr [rcx+0x08] (10.0) => 10.0 / 4.0 = 2.5
        a.fld(qword_ptr(rcx + 0x00)).unwrap();
        a.fdivr(qword_ptr(rcx + 0x08)).unwrap();
        a.fst(qword_ptr(rcx + 0x40)).unwrap();
        // fdivr m32: ST0 = 2.5; fdivr [rcx+0x10] (20.0) => 20.0 / 2.5 = 8.0
        a.fdivr(dword_ptr(rcx + 0x10)).unwrap();
        a.fst(qword_ptr(rcx + 0x48)).unwrap();
        // fidivr m32: ST0 = 8.0; fidivr [rcx+0x14] (60) => 60.0 / 8.0 = 7.5
        a.fidivr(dword_ptr(rcx + 0x14)).unwrap();
        a.fstp(qword_ptr(rcx + 0x50)).unwrap();

        // fdivrp st(1), st(0): push 12.0, push 3.0 -> ST1 = ST0/ST1 = 3.0/12.0 = 0.25
        a.fld(qword_ptr(rcx + 0x08)).unwrap(); // 10.0
        a.fld1().unwrap();
        a.faddp(st1, st0).unwrap();
        a.fld1().unwrap();
        a.faddp(st1, st0).unwrap(); // ST0 = 12.0
        a.fld1().unwrap();
        a.fld1().unwrap();
        a.faddp(st1, st0).unwrap();
        a.fld1().unwrap();
        a.faddp(st1, st0).unwrap(); // ST0 = 3.0, ST1 = 12.0
        a.fdivrp(st1, st0).unwrap(); // ST0 = 0.25
        a.fstp(qword_ptr(rcx + 0x58)).unwrap();

        // fistp / fisttp
        a.fld(qword_ptr(rcx + 0x18)).unwrap(); // 2.5
        a.fistp(word_ptr(rcx + 0x60)).unwrap();
        a.fld(qword_ptr(rcx + 0x20)).unwrap(); // -3.7
        a.fist(word_ptr(rcx + 0x62)).unwrap();
        a.fisttp(dword_ptr(rcx + 0x64)).unwrap();
        a.fld(qword_ptr(rcx + 0x28)).unwrap(); // 1e20 -> overflow IE + 0x8000_0000
        a.fistp(dword_ptr(rcx + 0x68)).unwrap();
        a.fnclex().unwrap();
        a.fnstsw(word_ptr(rcx + 0x8C)).unwrap();

        // Exact 64-bit fild -> fstp m80 -> fld m80 -> fistp m64 round-trip
        a.fild(qword_ptr(rcx + 0x30)).unwrap();
        a.fstp(tbyte_ptr(rcx + 0x78)).unwrap();
        a.fld(tbyte_ptr(rcx + 0x78)).unwrap();
        a.fistp(qword_ptr(rcx + 0x70)).unwrap();

        // fxam on -0.0, then ffreep st(0), then fxam on empty stack
        a.fldz().unwrap();
        a.fchs().unwrap();
        a.fxam().unwrap();
        a.fnstsw(word_ptr(rcx + 0x88)).unwrap();
        a.ffreep(st0).unwrap();
        a.fxam().unwrap();
        a.fnstsw(word_ptr(rcx + 0x8A)).unwrap();
        a.ret().unwrap();

        let code = a.assemble(0x1000).unwrap();
        let mut work = vec![0u8; 0x100];
        work[0x00..0x08].copy_from_slice(&4.0f64.to_le_bytes());
        work[0x08..0x10].copy_from_slice(&10.0f64.to_le_bytes());
        work[0x10..0x14].copy_from_slice(&20.0f32.to_le_bytes());
        work[0x14..0x18].copy_from_slice(&60i32.to_le_bytes());
        work[0x18..0x20].copy_from_slice(&2.5f64.to_le_bytes());
        work[0x20..0x28].copy_from_slice(&(-3.7f64).to_le_bytes());
        work[0x28..0x30].copy_from_slice(&1e20f64.to_le_bytes());
        let big_i64: i64 = -8_999_999_999_999_999_123;
        work[0x30..0x38].copy_from_slice(&big_i64.to_le_bytes());

        let mut stack = vec![0u64; 64];
        stack[40] = 0xDEAD_BEEF_CAFE_0000;
        let mut cpu = Cpu::new_zeroed();
        cpu.gpr[1] = work.as_mut_ptr() as u64;
        cpu.gpr[4] = unsafe { stack.as_mut_ptr().add(40) } as u64;
        let va = code.as_ptr() as u64;
        unsafe {
            x87_reset();
            let ex = run_oracle_with_x87(&mut cpu, va, va + code.len() as u64);
            assert_eq!(ex.kind, RF_RET, "x87 extended sequence must complete with RF_RET");
        }

        let f64_at = |o: usize| f64::from_bits(u64::from_le_bytes(work[o..o + 8].try_into().unwrap()));
        let i16_at = |o: usize| i16::from_le_bytes(work[o..o + 2].try_into().unwrap());
        let i32_at = |o: usize| i32::from_le_bytes(work[o..o + 4].try_into().unwrap());
        let i64_at = |o: usize| i64::from_le_bytes(work[o..o + 8].try_into().unwrap());
        let u16_at = |o: usize| u16::from_le_bytes(work[o..o + 2].try_into().unwrap());

        assert_eq!(f64_at(0x40), 2.5, "fdivr m64");
        assert_eq!(f64_at(0x48), 8.0, "fdivr m32");
        assert_eq!(f64_at(0x50), 7.5, "fidivr m32");
        assert_eq!(f64_at(0x58), 0.25, "fdivrp st(1), st(0)");
        assert_eq!(i16_at(0x60), 2, "fistp 2.5 ties-to-even");
        assert_eq!(i16_at(0x62), -4, "fist -3.7");
        assert_eq!(i32_at(0x64), -3, "fisttp -3.7 truncate");
        assert_eq!(i32_at(0x68), i32::MIN, "fistp overflow -> integer indefinite");
        assert_eq!(i64_at(0x70), big_i64, "fild -> fstp m80 -> fld m80 -> fistp m64 exact 64-bit");
        assert_eq!(u16_at(0x88) & 0x4700, 0x4200, "fxam(-0.0): C3=1, C1=1");
        assert_eq!(u16_at(0x8A) & 0x4700, 0x4100, "fxam(empty after ffreep): C3=1, C0=1");
        assert_eq!(u16_at(0x8C) & 0x80FF, 0, "fnclex cleared SW exception bits");

        // 2. Test real msvcr100.dll `fmod` x87 loop (`fld; fld; fxam; wait; fnstsw; fnclex; fprem; fstsw ax; fstp st(1); fstp`)
        //    plus `fxch`, `fabs`, `fsqrt`, `frndint`, `fcomip`, and `fcom`.
        let mut b = CodeAssembler::new(64).unwrap();
        let mut loop_lbl = b.create_label();
        b.fninit().unwrap();
        b.fld(qword_ptr(rcx + 0x00)).unwrap(); // divisor = 3.5
        b.fld(qword_ptr(rcx + 0x08)).unwrap(); // dividend = -12.25
        b.fxam().unwrap();
        b.wait().unwrap();
        b.fnstsw(word_ptr(rcx + 0x20)).unwrap();
        b.fnclex().unwrap();
        b.set_label(&mut loop_lbl).unwrap();
        b.fprem().unwrap();
        b.wait().unwrap();
        b.fnstsw(ax).unwrap();
        b.test(ah, 0x04).unwrap(); // C2 = bit 10 = bit 2 of AH
        b.jnz(loop_lbl).unwrap();
        b.fstp(st1).unwrap();
        b.fstp(qword_ptr(rcx + 0x10)).unwrap(); // -12.25 % 3.5 = -1.75

        // fxch + fabs + fsqrt + fcomip -> CF/ZF/PF in EFLAGS
        b.fld(qword_ptr(rcx + 0x08)).unwrap(); // -12.25
        b.fld(qword_ptr(rcx + 0x00)).unwrap(); // 3.5
        b.fxch(st0, st1).unwrap();             // ST0=-12.25, ST1=3.5
        b.fabs().unwrap();                     // ST0=12.25
        b.fsqrt().unwrap();                    // ST0=3.5
        b.fcomip(st0, st1).unwrap();           // 3.5 == 3.5 -> ZF=1, CF=0, PF=0
        b.sete(byte_ptr(rcx + 0x22)).unwrap();
        b.setb(byte_ptr(rcx + 0x23)).unwrap();
        b.setp(byte_ptr(rcx + 0x24)).unwrap();
        b.fstp(st0).unwrap();
        b.ret().unwrap();

        let code2 = b.assemble(0x2000).unwrap();
        let mut work2 = vec![0u8; 0x40];
        work2[0x00..0x08].copy_from_slice(&3.5f64.to_le_bytes());
        work2[0x08..0x10].copy_from_slice(&(-12.25f64).to_le_bytes());
        let mut cpu2 = Cpu::new_zeroed();
        cpu2.gpr[1] = work2.as_mut_ptr() as u64;
        cpu2.gpr[4] = unsafe { stack.as_mut_ptr().add(40) } as u64;
        let va2 = code2.as_ptr() as u64;
        unsafe {
            x87_reset();
            let ex = run_oracle_with_x87(&mut cpu2, va2, va2 + code2.len() as u64);
            assert_eq!(ex.kind, RF_RET);
        }
        let rem = f64::from_bits(u64::from_le_bytes(work2[0x10..0x18].try_into().unwrap()));
        let fxam_sw = u16::from_le_bytes(work2[0x20..0x22].try_into().unwrap());
        assert_eq!(rem, -1.75, "msvcr100 fmod sequence remainder");
        assert_eq!(fxam_sw & 0x4700, 0x0600, "fxam(-12.25): normal finite negative (C2=1, C1=1)");
        assert_eq!(&work2[0x22..0x25], &[1, 0, 0], "fcomip(sqrt(abs(-12.25)), 3.5): ZF=1 CF=0 PF=0");
    }

    #[test]
    fn guest_range_wrapping_overflow_and_full_span_edges() {
        const LO: u64 = 0x1_0000;
        const HI: u64 = 0x8000_0000_0000;
        const SPAN: usize = (HI - LO) as usize;

        assert!(is_valid_guest_range(LO, SPAN));
        assert!(!is_valid_guest_range(LO + 1, SPAN));
        assert!(!is_valid_guest_range(LO, SPAN + 1));
        assert!(!is_valid_guest_range(LO, usize::MAX));
        assert!(!is_valid_guest_range(u64::MAX, 1));
        assert!(!is_valid_guest_range(u64::MAX, 16));
        assert!(is_valid_guest_range(HI - 16, 16));
        assert!(!is_valid_guest_range(HI - 15, 16));
        assert!(!is_valid_guest_range(HI, 0));
    }

    #[test]
    fn st_reg_index_o1_mapping() {
        let regs = [
            Register::ST0,
            Register::ST1,
            Register::ST2,
            Register::ST3,
            Register::ST4,
            Register::ST5,
            Register::ST6,
            Register::ST7,
        ];
        for (i, &r) in regs.iter().enumerate() {
            assert_eq!(st_reg_index(r), Some(i as u8));
        }
        for r in [Register::None, Register::RAX, Register::XMM0, Register::YMM0, Register::MM0, Register::CS] {
            assert_eq!(st_reg_index(r), None);
        }
    }

    #[test]
    fn x87_fcomi_fucomi_and_fcom_all_orderings_including_unordered_nan() {
        use iced_x86::code_asm::*;

        // Test (a, b) for >, <, ==, -0.0==+0.0, qNaN/num, num/qNaN, qNaN/qNaN, sNaN/num
        // Expected (ZF, PF, CF) in RFLAGS AND (C3, C2, C0) in SW:
        //   a > b: ZF=0, PF=0, CF=0; C3=0, C2=0, C0=0
        //   a < b: ZF=0, PF=0, CF=1; C3=0, C2=0, C0=1
        //   a == b: ZF=1, PF=0, CF=0; C3=1, C2=0, C0=0
        //   Unordered: ZF=1, PF=1, CF=1; C3=1, C2=1, C0=1
        let cases: &[(u64, u64, bool, bool, bool)] = &[
            (5.0f64.to_bits(), 2.0f64.to_bits(), false, false, false),
            (2.0f64.to_bits(), 5.0f64.to_bits(), false, false, true),
            (3.25f64.to_bits(), 3.25f64.to_bits(), true, false, false),
            ((-0.0f64).to_bits(), 0.0f64.to_bits(), true, false, false),
            (f64::NEG_INFINITY.to_bits(), f64::INFINITY.to_bits(), false, false, true),
            (f64::INFINITY.to_bits(), f64::INFINITY.to_bits(), true, false, false),
            (0x7FF8_0000_0000_0001, 1.0f64.to_bits(), true, true, true), // qNaN vs finite
            (1.0f64.to_bits(), 0x7FF8_0000_0000_0002, true, true, true), // finite vs qNaN
            (0x7FF8_0000_0000_0001, 0xFFF8_0000_0000_0002, true, true, true), // NaN vs NaN
            (0x7FF0_0000_0000_0001, 0.0f64.to_bits(), true, true, true), // sNaN vs 0.0
        ];

        for &(a_bits, b_bits, want_zf, want_pf, want_cf) in cases {
            let mut work = [0u8; 64];
            work[0..8].copy_from_slice(&b_bits.to_le_bytes());
            work[8..16].copy_from_slice(&a_bits.to_le_bytes());

            // 1. Test `fucomip` / `fcomi` directly via `exec_x87_ins` in BOTH non-precise & precise modes
            for precise in [false, true] {
                crate::emu::set_precise_faults_local(precise);
                let mut asm = CodeAssembler::new(64).unwrap();
                asm.fninit().unwrap();
                asm.fld(qword_ptr(rcx + 0)).unwrap(); // ST1 = b
                asm.fld(qword_ptr(rcx + 8)).unwrap(); // ST0 = a
                asm.fcomi(st0, st1).unwrap();
                asm.fucomip(st0, st1).unwrap();
                asm.fstp(qword_ptr(rcx + 16)).unwrap(); // pops b -> tests non-precise fstp skip-snapshot path too
                let code = asm.assemble(0x3000).unwrap();

                let mut cpu = Cpu::new_zeroed();
                cpu.gpr[1] = work.as_mut_ptr() as u64;
                cpu.sf = true;
                cpu.of = true;

                let mut dec = Decoder::with_ip(64, &code, code.as_ptr() as u64, DecoderOptions::NONE);
                let mut fcomi_checked = false;
                while dec.can_decode() {
                    let ins = dec.decode();
                    let ok = unsafe { exec_x87_ins(&mut cpu, ins.mnemonic(), &ins) };
                    assert!(ok, "exec_x87_ins failed on {ins}");
                    if matches!(ins.mnemonic(), Mnemonic::Fcomi | Mnemonic::Fucomip) {
                        assert_eq!(cpu.zf, want_zf, "ZF mismatch for ({a_bits:#x}, {b_bits:#x})");
                        assert_eq!(cpu.pf(), want_pf, "PF mismatch for ({a_bits:#x}, {b_bits:#x}) (pfv={})", cpu.pfv);
                        assert_eq!(cpu.cf, want_cf, "CF mismatch for ({a_bits:#x}, {b_bits:#x})");
                        assert!(!cpu.sf && !cpu.of, "SF/OF must be cleared");
                        fcomi_checked = true;
                    }
                }
                assert!(fcomi_checked);
                // Check `fstp [rcx + 16]` stored `b` (quieted if sNaN).
                let stored = u64::from_le_bytes(work[16..24].try_into().unwrap());
                if f64::from_bits(b_bits).is_nan() {
                    assert!(f64::from_bits(stored).is_nan());
                } else {
                    assert_eq!(stored, b_bits);
                }

                // 2. Also verify `X87State::compare_sw` (C3, C2, C0) matches `(ZF, PF, CF)`.
                let mut st = X87State::new();
                st.compare_sw(f64::from_bits(a_bits), f64::from_bits(b_bits));
                let sw = st.status_word();
                assert_eq!((sw & (1 << 14)) != 0, want_zf, "SW.C3 mismatch");
                assert_eq!((sw & (1 << 10)) != 0, want_pf, "SW.C2 mismatch");
                assert_eq!((sw & (1 << 8)) != 0, want_cf, "SW.C0 mismatch");
                assert_eq!(sw & (1 << 9), 0, "SW.C1 must be cleared");
            }
            crate::emu::set_precise_faults_local(false);
        }
    }
}
