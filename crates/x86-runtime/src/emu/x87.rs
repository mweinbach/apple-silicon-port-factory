//! Architectural x87 FPU state, exact 80-bit extended-precision soft-float engine (`Fp80`),
//! transcendental & BCD helpers, legacy environment/state serialization (`fldenv`/`fnstenv`/
//! `frstor`/`fnsave`), and C-ABI helper dispatch table (`X87Helpers`) for ARM64 JIT/AOT code.
//!
//! # Shared x87 / MMX Register-File Layout
//!
//! [`X87State`] is `#[repr(C, align(16))]` with 16-byte-aligned physical slots `regs[0..8]`
//! matching the Intel `FXSAVE` layout:
//! - `regs[p][0..8]`: 64-bit little-endian x87 significand (explicit integer bit `J` in bit 63)
//!   **or** 64-bit MMX register `MMp`.
//! - `regs[p][8..10]`: 16-bit sign (bit 15) + biased 15-bit exponent (`0xFFFF` after MMX writes).
//! - `regs[p][10..16]`: zero padding (always 0).
//! - `top`: 3-bit stack top pointer (`0..7`); logical `ST(i)` maps to physical slot `(top + i) & 7`.
//! - `valid_mask`: 8-bit abridged tag word (bit `p` = 1 iff physical slot `p` is non-empty;
//!   MMX instructions set `valid_mask = 0xFF, top = 0`; `emms`/`femms` sets `valid_mask = 0`).
//!
//! Keeping a single canonical register file in `regs` (with zero lazy mirror state) ensures
//! x87, MMX, `emms`, `fxsave`/`fxrstor`, `xsave`/`xrstor`, `fnsave`/`frstor`, and `diag`
//! are always synchronized without flushing barriers.
//!
//! # Transcendental Accuracy Notes
//!
//! `fsin`, `fcos`, `fsincos`, `fptan`, `fpatan`, `f2xm1`, `fyl2x`, and `fyl2xp1` execute via
//! native Rust helpers using host IEEE-754 `f64` (`libm`) elementary functions (`sin`, `cos`,
//! `tan`, `atan2`, `exp_m1`, `ln_1p`, `log2`). Compared against real x86-64 x87 hardware
//! (which uses a 66-bit internal approximation of `π` and achieves ~1–2 ULP at 64-bit), these
//! helpers achieve `<= 1 ULP` in 53-bit double precision (`<= 2^-52 ≈ 2.22e-16` relative error),
//! matching Win64 default `PC = 53-bit` / MSVC CRT `double` accuracy while faithfully reproducing
//! Intel SDM Vol. 2A argument-range (`|ST(0)| >= 2^63` -> `C2 = 1`), stack-fault, NaN, zero,
//! and infinity semantics.

use core::cmp::Ordering;
use core::mem::offset_of;

// =================================================================================================
// Status Word / Control Word bit masks
// =================================================================================================

pub(super) const SW_IE: u16 = 0x0001;
pub(super) const SW_DE: u16 = 0x0002;
pub(super) const SW_ZE: u16 = 0x0004;
pub(super) const SW_OE: u16 = 0x0008;
pub(super) const SW_UE: u16 = 0x0010;
pub(super) const SW_PE: u16 = 0x0020;
pub(super) const SW_SF: u16 = 0x0040;
pub(super) const SW_C0: u16 = 0x0100;
pub(super) const SW_C1: u16 = 0x0200;
pub(super) const SW_C2: u16 = 0x0400;
pub(super) const SW_C3: u16 = 0x4000;
pub(super) const SW_C_MASK: u16 = SW_C0 | SW_C1 | SW_C2 | SW_C3;

pub(super) const RC_RN: u8 = 0; // Round to nearest (ties to even)
pub(super) const RC_RM: u8 = 1; // Round down (-inf)
pub(super) const RC_RP: u8 = 2; // Round up (+inf)
pub(super) const RC_RZ: u8 = 3; // Round toward zero (truncate)

// =================================================================================================
// Exact 80-bit extended-precision floating point (`Fp80`)
// =================================================================================================

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(super) enum Fp80Kind {
    #[default]
    Zero,
    Finite,
    Inf,
    Nan,
    Unsupported,
}

/// Exact 80-bit x87 extended-precision register value (`1` sign bit, unbiased exponent, `64`-bit
/// normalized significand with bit 63 = integer bit `J`).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(super) struct Fp80 {
    pub(super) kind: Fp80Kind,
    pub(super) sign: u8,
    pub(super) exp: i32,
    pub(super) sig: u64,
}

impl Fp80 {
    pub(super) const QNAN_INDEF: Self = Self {
        kind: Fp80Kind::Nan,
        sign: 1,
        exp: 0,
        sig: 0xC000_0000_0000_0000,
    };

    pub(super) const QNAN_INDEF_BYTES: [u8; 10] = [0, 0, 0, 0, 0, 0, 0, 0xC0, 0xFF, 0xFF];

    #[inline(always)]
    pub(super) const fn zero(sign: u8) -> Self {
        Self { kind: Fp80Kind::Zero, sign: sign & 1, exp: 0, sig: 0 }
    }

    #[inline(always)]
    pub(super) const fn inf(sign: u8) -> Self {
        Self { kind: Fp80Kind::Inf, sign: sign & 1, exp: 0, sig: 1u64 << 63 }
    }

    #[inline(always)]
    pub(super) const fn one() -> Self {
        Self { kind: Fp80Kind::Finite, sign: 0, exp: 0, sig: 1u64 << 63 }
    }

    #[inline(always)]
    pub(super) fn is_signaling_nan(self) -> bool {
        self.kind == Fp80Kind::Nan && (self.sig & (1u64 << 62)) == 0
    }

    /// Convert IEEE-754 `f32` bits to 80-bit extended precision, raising `SW.IE` on sNaN and
    /// `SW.DE` on subnormal input, matching `fld dword ptr [mem]`.
    #[inline]
    pub(super) fn from_f32_bits(bits: u32, sw: &mut u16) -> Self {
        let sign = (bits >> 31) as u8;
        let e = (bits >> 23) & 0xFF;
        let f = bits & 0x007F_FFFF;
        if e == 0xFF {
            if f == 0 {
                Self::inf(sign)
            } else {
                if (f & 0x0040_0000) == 0 {
                    *sw |= SW_IE;
                }
                Self {
                    kind: Fp80Kind::Nan,
                    sign,
                    exp: 0,
                    sig: (1u64 << 63) | (1u64 << 62) | ((f as u64) << 40),
                }
            }
        } else if e == 0 {
            if f == 0 {
                Self::zero(sign)
            } else {
                *sw |= SW_DE;
                let f64_val = (f as u64) << 40;
                let clz = f64_val.leading_zeros() as i32; // f in 1..2^23 -> f64_val bit 62..40 -> clz in 1..=23
                Self {
                    kind: Fp80Kind::Finite,
                    sign,
                    exp: -126 - clz,
                    sig: f64_val << (clz as u32),
                }
            }
        } else {
            Self {
                kind: Fp80Kind::Finite,
                sign,
                exp: (e as i32) - 127,
                sig: (1u64 << 63) | ((f as u64) << 40),
            }
        }
    }

    /// Convert IEEE-754 `f64` bits to 80-bit extended precision, raising `SW.IE` on sNaN and
    /// `SW.DE` on subnormal input, matching `fld qword ptr [mem]`.
    #[inline]
    pub(super) fn from_f64_bits(bits: u64, sw: &mut u16) -> Self {
        let sign = (bits >> 63) as u8;
        let e = ((bits >> 52) & 0x7FF) as u32;
        let f = bits & 0x000F_FFFF_FFFF_FFFF;
        if e == 0x7FF {
            if f == 0 {
                Self::inf(sign)
            } else {
                if (f & 0x0008_0000_0000_0000) == 0 {
                    *sw |= SW_IE;
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
                Self::zero(sign)
            } else {
                *sw |= SW_DE;
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

    /// Convert IEEE-754 `f64` bits without mutating `sw` (for internal transcendental results).
    #[inline]
    pub(super) fn from_f64_raw(v: f64) -> Self {
        let mut dummy = 0u16;
        Self::from_f64_bits(v.to_bits(), &mut dummy)
    }

    /// Decode a raw 10-byte x87 slot without quieting sNaNs or raising exceptions.
    #[inline]
    pub(super) fn from_f80_bytes(b: [u8; 10]) -> Self {
        let sig = u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]);
        let se = u16::from_le_bytes([b[8], b[9]]);
        let sign = (se >> 15) as u8;
        let e = (se & 0x7FFF) as i32;
        if e == 0x7FFF {
            if (sig >> 63) == 0 {
                Self { kind: Fp80Kind::Unsupported, sign, exp: 0, sig }
            } else if (sig & 0x7FFF_FFFF_FFFF_FFFF) == 0 {
                Self::inf(sign)
            } else {
                Self { kind: Fp80Kind::Nan, sign, exp: 0, sig }
            }
        } else if e == 0 {
            if sig == 0 {
                Self::zero(sign)
            } else {
                // Denormal (J=0) or pseudo-denormal (J=1): both represent sig * 2^(-16382 - 63).
                let clz = sig.leading_zeros() as i32;
                Self { kind: Fp80Kind::Finite, sign, exp: -16382 - clz, sig: sig << (clz as u32) }
            }
        } else if (sig >> 63) == 0 {
            // Unnormal: non-zero exponent with J-bit clear -> unsupported encoding on 387+.
            Self { kind: Fp80Kind::Unsupported, sign, exp: e - 16383, sig }
        } else {
            Self { kind: Fp80Kind::Finite, sign, exp: e - 16383, sig }
        }
    }

    /// Encode [`Fp80`] into 10 little-endian bytes (`m80fp`).
    #[inline]
    pub(super) fn to_f80_bytes(self) -> [u8; 10] {
        let s = (self.sign as u16) << 15;
        let (sig, se) = match self.kind {
            Fp80Kind::Zero => (0u64, s),
            Fp80Kind::Inf => (1u64 << 63, s | 0x7FFF),
            Fp80Kind::Nan => (self.sig | (1u64 << 63), s | 0x7FFF),
            Fp80Kind::Unsupported => (self.sig, s | 0x7FFF),
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
    pub(super) fn from_i64(v: i64) -> Self {
        if v == 0 {
            return Self::zero(0);
        }
        let sign = (v < 0) as u8;
        let mag = v.unsigned_abs();
        let clz = mag.leading_zeros() as i32;
        Self { kind: Fp80Kind::Finite, sign, exp: 63 - clz, sig: mag << (clz as u32) }
    }

    /// Decide whether to increment magnitude given `(sign, lsb_odd, round_bit, sticky, rc)`.
    #[inline(always)]
    fn should_round_up(sign: u8, lsb_odd: bool, round_bit: bool, sticky: bool, rc: u8) -> bool {
        match rc & 3 {
            RC_RN => round_bit && (sticky || lsb_odd),
            RC_RM => sign != 0 && (round_bit || sticky),
            RC_RP => sign == 0 && (round_bit || sticky),
            _ => false, // RC_RZ
        }
    }

    /// Round 80-bit extended precision to IEEE-754 `f64` bits under rounding mode `rc`.
    #[inline]
    pub(super) fn to_f64_bits(self, rc: u8, sw: &mut u16) -> u64 {
        let s = (self.sign as u64) << 63;
        match self.kind {
            Fp80Kind::Zero => s,
            Fp80Kind::Inf => s | 0x7FF0_0000_0000_0000,
            Fp80Kind::Unsupported => {
                *sw |= SW_IE;
                0xFFF8_0000_0000_0000
            }
            Fp80Kind::Nan => {
                if self.is_signaling_nan() {
                    *sw |= SW_IE;
                }
                let f = ((self.sig >> 11) & 0x000F_FFFF_FFFF_FFFF) | 0x0008_0000_0000_0000;
                s | 0x7FF0_0000_0000_0000 | f
            }
            Fp80Kind::Finite => {
                let mut exp = self.exp;
                let sig = self.sig;
                if exp > 1023 {
                    *sw |= SW_OE | SW_PE;
                    return match rc & 3 {
                        RC_RZ => s | 0x7FEF_FFFF_FFFF_FFFF,
                        RC_RM => if self.sign != 0 { s | 0x7FF0_0000_0000_0000 } else { 0x7FEF_FFFF_FFFF_FFFF },
                        RC_RP => if self.sign == 0 { s | 0x7FF0_0000_0000_0000 } else { s | 0x7FEF_FFFF_FFFF_FFFF },
                        _ => s | 0x7FF0_0000_0000_0000,
                    };
                }
                if exp >= -1022 {
                    let mut hi = sig >> 11;
                    let rem = sig & 0x7FF;
                    if rem != 0 {
                        *sw |= SW_PE;
                        let rbit = (rem & 0x400) != 0;
                        let sticky = (rem & 0x3FF) != 0;
                        if Self::should_round_up(self.sign, (hi & 1) != 0, rbit, sticky, rc) {
                            hi += 1;
                            if hi == (1u64 << 53) {
                                hi >>= 1;
                                exp += 1;
                                if exp > 1023 {
                                    *sw |= SW_OE;
                                    return s | 0x7FF0_0000_0000_0000;
                                }
                            }
                        }
                    }
                    s | (((exp + 1023) as u64) << 52) | (hi & 0x000F_FFFF_FFFF_FFFF)
                } else {
                    let shr = -1011 - exp;
                    if shr >= 64 {
                        *sw |= SW_UE | SW_PE;
                        let rbit = shr == 64 && (sig >> 63) != 0;
                        let sticky = if shr == 64 { (sig & 0x7FFF_FFFF_FFFF_FFFF) != 0 } else { sig != 0 };
                        if Self::should_round_up(self.sign, false, rbit, sticky, rc) {
                            return s | 1;
                        }
                        return s;
                    }
                    let shr_u = shr as u32;
                    let mut hi = sig >> shr_u;
                    let half = 1u64 << (shr_u - 1);
                    let rem = sig & ((1u64 << shr_u) - 1);
                    if rem != 0 {
                        *sw |= SW_UE | SW_PE;
                        let rbit = (rem & half) != 0;
                        let sticky = (rem & (half - 1)) != 0;
                        if Self::should_round_up(self.sign, (hi & 1) != 0, rbit, sticky, rc) {
                            hi += 1;
                        }
                    }
                    s | hi
                }
            }
        }
    }

    /// Round 80-bit extended precision directly to IEEE-754 `f32` bits under rounding mode `rc`
    /// (avoiding double rounding through `f64`).
    #[inline]
    pub(super) fn to_f32_bits(self, rc: u8, sw: &mut u16) -> u32 {
        let s = (self.sign as u32) << 31;
        match self.kind {
            Fp80Kind::Zero => s,
            Fp80Kind::Inf => s | 0x7F80_0000,
            Fp80Kind::Unsupported => {
                *sw |= SW_IE;
                0xFFC0_0000
            }
            Fp80Kind::Nan => {
                if self.is_signaling_nan() {
                    *sw |= SW_IE;
                }
                let f = (((self.sig >> 40) as u32) & 0x007F_FFFF) | 0x0040_0000;
                s | 0x7F80_0000 | f
            }
            Fp80Kind::Finite => {
                let mut exp = self.exp;
                let sig = self.sig;
                if exp > 127 {
                    *sw |= SW_OE | SW_PE;
                    return match rc & 3 {
                        RC_RZ => s | 0x7F7F_FFFF,
                        RC_RM => if self.sign != 0 { s | 0x7F80_0000 } else { 0x7F7F_FFFF },
                        RC_RP => if self.sign == 0 { s | 0x7F80_0000 } else { s | 0x7F7F_FFFF },
                        _ => s | 0x7F80_0000,
                    };
                }
                if exp >= -126 {
                    let mut hi = (sig >> 40) as u32;
                    let rem = sig & ((1u64 << 40) - 1);
                    if rem != 0 {
                        *sw |= SW_PE;
                        let rbit = (rem & (1u64 << 39)) != 0;
                        let sticky = (rem & ((1u64 << 39) - 1)) != 0;
                        if Self::should_round_up(self.sign, (hi & 1) != 0, rbit, sticky, rc) {
                            hi += 1;
                            if hi == (1u32 << 24) {
                                hi >>= 1;
                                exp += 1;
                                if exp > 127 {
                                    *sw |= SW_OE;
                                    return s | 0x7F80_0000;
                                }
                            }
                        }
                    }
                    s | (((exp + 127) as u32) << 23) | (hi & 0x007F_FFFF)
                } else {
                    let shr = -86 - exp; // -126 -> 40, -127 -> 41, ..., -149 -> 63, -150 -> 64
                    if shr >= 64 {
                        *sw |= SW_UE | SW_PE;
                        let rbit = shr == 64 && (sig >> 63) != 0;
                        let sticky = if shr == 64 { (sig & 0x7FFF_FFFF_FFFF_FFFF) != 0 } else { sig != 0 };
                        if Self::should_round_up(self.sign, false, rbit, sticky, rc) {
                            return s | 1;
                        }
                        return s;
                    }
                    let shr_u = shr as u32;
                    let mut hi = (sig >> shr_u) as u32;
                    let half = 1u64 << (shr_u - 1);
                    let rem = sig & ((1u64 << shr_u) - 1);
                    if rem != 0 {
                        *sw |= SW_UE | SW_PE;
                        let rbit = (rem & half) != 0;
                        let sticky = (rem & (half - 1)) != 0;
                        if Self::should_round_up(self.sign, (hi & 1) != 0, rbit, sticky, rc) {
                            hi += 1;
                        }
                    }
                    s | hi
                }
            }
        }
    }

    /// Round a normalized 128-bit significand `(hi, lo)` (where `hi` has bit 63 set) to
    /// `prec_bits` (`24`, `53`, or `64`) under rounding control `rc`, applying 80-bit exponent
    /// range (`-16382..=16383`) and setting `OE`/`UE`/`PE`.
    #[inline]
    pub(super) fn round_from_parts(
        sign: u8,
        mut exp: i32,
        mut hi: u64,
        mut lo: u64,
        prec_bits: u32,
        rc: u8,
        sw: &mut u16,
    ) -> Self {
        debug_assert!(hi & (1u64 << 63) != 0);
        // Handle 80-bit subnormal / underflow (`exp < -16382`).
        if exp < -16382 {
            let shift = (-16382 - exp) as u32;
            let (sub_hi, sub_lo) = if shift >= 128 {
                (0u64, 1u64)
            } else if shift >= 64 {
                let s = shift - 64;
                let sticky = ((lo != 0) as u64) | if s > 0 { ((hi & ((1u64 << s) - 1)) != 0) as u64 } else { 0 };
                (0u64, (hi >> s) | sticky)
            } else {
                let new_hi = hi >> shift;
                let new_lo = (hi << (64 - shift)) | (lo >> shift) | (((lo & ((1u64 << shift) - 1)) != 0) as u64);
                (new_hi, new_lo)
            };
            // Round at the subnormal 64-bit boundary (or narrower prec_bits if requested).
            let rshift = 64 - prec_bits;
            let (mut q, rbit, sticky) = if rshift == 0 {
                (sub_hi, (sub_lo >> 63) != 0, (sub_lo & 0x7FFF_FFFF_FFFF_FFFF) != 0)
            } else {
                let low_mask = (1u64 << rshift) - 1;
                let rem = sub_hi & low_mask;
                let half = 1u64 << (rshift - 1);
                let rb = (rem & half) != 0;
                let st = (rem & (half - 1)) != 0 || sub_lo != 0;
                (sub_hi >> rshift, rb, st)
            };
            if rbit || sticky {
                *sw |= SW_UE | SW_PE;
                if Self::should_round_up(sign, (q & 1) != 0, rbit, sticky, rc) {
                    q += 1;
                }
            }
            let full_sig = q << rshift;
            if full_sig == 0 {
                return Self::zero(sign);
            }
            let clz = full_sig.leading_zeros() as i32;
            if clz == 0 {
                // Rounded back up to normal exponent -16382.
                return Self { kind: Fp80Kind::Finite, sign, exp: -16382, sig: full_sig };
            }
            return Self { kind: Fp80Kind::Finite, sign, exp: -16382 - clz, sig: full_sig << (clz as u32) };
        }

        // Normal rounding to `prec_bits` (24, 53, or 64).
        let rshift = 64 - prec_bits;
        let (rbit, sticky) = if rshift == 0 {
            ((lo >> 63) != 0, (lo & 0x7FFF_FFFF_FFFF_FFFF) != 0)
        } else {
            let low_mask = (1u64 << rshift) - 1;
            let rem = hi & low_mask;
            let half = 1u64 << (rshift - 1);
            let rb = (rem & half) != 0;
            let st = (rem & (half - 1)) != 0 || lo != 0;
            hi >>= rshift;
            lo = 0;
            let _ = lo;
            (rb, st)
        };
        if rbit || sticky {
            *sw |= SW_PE;
            if Self::should_round_up(sign, (hi & 1) != 0, rbit, sticky, rc) {
                let (next, ov) = hi.overflowing_add(1);
                let limit_ov = if rshift == 0 { ov } else { next == (1u64 << prec_bits) };
                if limit_ov {
                    hi = 1u64 << (prec_bits - 1);
                    exp += 1;
                } else {
                    hi = next;
                }
            }
        }
        let final_sig = hi << rshift;
        if exp > 16383 {
            *sw |= SW_OE | SW_PE;
            let max_sig = if rshift == 0 { u64::MAX } else { !((1u64 << rshift) - 1) };
            let max_fin = Self { kind: Fp80Kind::Finite, sign, exp: 16383, sig: max_sig };
            return match rc & 3 {
                RC_RZ => max_fin,
                RC_RM => if sign != 0 { Self::inf(1) } else { max_fin },
                RC_RP => if sign == 0 { Self::inf(0) } else { max_fin },
                _ => Self::inf(sign),
            };
        }
        Self { kind: Fp80Kind::Finite, sign, exp, sig: final_sig }
    }

    /// Intel x87 dual-NaN selection rule: raises `SW.IE` on any sNaN, picks the larger 62-bit
    /// payload (or positive sign on equal payload), and quiets the result.
    #[inline]
    pub(super) fn select_nan(a: Self, b: Self, sw: &mut u16) -> Self {
        if a.is_signaling_nan() || b.is_signaling_nan() {
            *sw |= SW_IE;
        }
        let mut res = if a.kind == Fp80Kind::Nan && b.kind == Fp80Kind::Nan {
            let pa = a.sig & 0x3FFF_FFFF_FFFF_FFFF;
            let pb = b.sig & 0x3FFF_FFFF_FFFF_FFFF;
            if pa > pb {
                a
            } else if pb > pa {
                b
            } else {
                Self { sign: a.sign & b.sign, ..a }
            }
        } else if a.kind == Fp80Kind::Nan {
            a
        } else {
            b
        };
        res.sig |= (1u64 << 63) | (1u64 << 62);
        res
    }

    /// Exact 80-bit addition `a + b` honoring `cw` precision control (`24`/`53`/`64`) and
    /// rounding control (`RN`/`RM`/`RP`/`RZ`).
    pub(super) fn add(a: Self, b: Self, cw: u16, sw: &mut u16) -> Self {
        if a.kind == Fp80Kind::Unsupported || b.kind == Fp80Kind::Unsupported {
            *sw |= SW_IE;
            return Self::QNAN_INDEF;
        }
        if a.kind == Fp80Kind::Nan || b.kind == Fp80Kind::Nan {
            return Self::select_nan(a, b, sw);
        }
        let rc = rc_of(cw);
        let prec = prec_of(cw);
        match (a.kind, b.kind) {
            (Fp80Kind::Inf, Fp80Kind::Inf) => {
                if a.sign != b.sign {
                    *sw |= SW_IE;
                    Self::QNAN_INDEF
                } else {
                    a
                }
            }
            (Fp80Kind::Inf, _) => a,
            (_, Fp80Kind::Inf) => b,
            (Fp80Kind::Zero, Fp80Kind::Zero) => {
                let s = if a.sign == b.sign { a.sign } else { u8::from(rc == RC_RM) };
                Self::zero(s)
            }
            (Fp80Kind::Zero, _) => b,
            (_, Fp80Kind::Zero) => a,
            (Fp80Kind::Finite, Fp80Kind::Finite) => {
                // Ensure |big| >= |small|.
                let (big, small) = if a.exp > b.exp || (a.exp == b.exp && a.sig >= b.sig) {
                    (a, b)
                } else {
                    (b, a)
                };
                let diff = (big.exp - small.exp) as u32;
                let (s_hi, s_lo) = if diff == 0 {
                    (small.sig, 0u64)
                } else if diff < 64 {
                    (small.sig >> diff, small.sig << (64 - diff))
                } else if diff < 128 {
                    let d = diff - 64;
                    let lo = if d == 0 {
                        small.sig
                    } else {
                        (small.sig >> d) | (((small.sig & ((1u64 << d) - 1)) != 0) as u64)
                    };
                    (0u64, lo)
                } else {
                    (0u64, 1u64)
                };

                if big.sign == small.sign {
                    let (sum_hi, carry) = big.sig.overflowing_add(s_hi);
                    if carry {
                        let out_hi = (1u64 << 63) | (sum_hi >> 1);
                        let out_lo = ((sum_hi & 1) << 63) | (s_lo >> 1) | (s_lo & 1);
                        Self::round_from_parts(big.sign, big.exp + 1, out_hi, out_lo, prec, rc, sw)
                    } else {
                        Self::round_from_parts(big.sign, big.exp, sum_hi, s_lo, prec, rc, sw)
                    }
                } else {
                    let (lo_sub, borrow_lo) = 0u64.overflowing_sub(s_lo);
                    let hi_sub = big.sig.wrapping_sub(s_hi).wrapping_sub(borrow_lo as u64);
                    if hi_sub == 0 && lo_sub == 0 {
                        return Self::zero(u8::from(rc == RC_RM));
                    }
                    let (norm_hi, norm_lo, exp_delta) = if hi_sub != 0 {
                        let clz = hi_sub.leading_zeros();
                        if clz == 0 {
                            (hi_sub, lo_sub, 0i32)
                        } else {
                            ((hi_sub << clz) | (lo_sub >> (64 - clz)), lo_sub << clz, -(clz as i32))
                        }
                    } else {
                        let clz = lo_sub.leading_zeros();
                        (lo_sub << clz, 0u64, -64 - (clz as i32))
                    };
                    Self::round_from_parts(big.sign, big.exp + exp_delta, norm_hi, norm_lo, prec, rc, sw)
                }
            }
            _ => Self::QNAN_INDEF,
        }
    }

    #[inline]
    pub(super) fn sub(a: Self, mut b: Self, cw: u16, sw: &mut u16) -> Self {
        if b.kind != Fp80Kind::Nan && b.kind != Fp80Kind::Unsupported {
            b.sign ^= 1;
        }
        Self::add(a, b, cw, sw)
    }

    /// Exact 80-bit multiplication `a * b`.
    pub(super) fn mul(a: Self, b: Self, cw: u16, sw: &mut u16) -> Self {
        if a.kind == Fp80Kind::Unsupported || b.kind == Fp80Kind::Unsupported {
            *sw |= SW_IE;
            return Self::QNAN_INDEF;
        }
        if a.kind == Fp80Kind::Nan || b.kind == Fp80Kind::Nan {
            return Self::select_nan(a, b, sw);
        }
        let sign = a.sign ^ b.sign;
        match (a.kind, b.kind) {
            (Fp80Kind::Inf, Fp80Kind::Zero) | (Fp80Kind::Zero, Fp80Kind::Inf) => {
                *sw |= SW_IE;
                Self::QNAN_INDEF
            }
            (Fp80Kind::Inf, _) | (_, Fp80Kind::Inf) => Self::inf(sign),
            (Fp80Kind::Zero, _) | (_, Fp80Kind::Zero) => Self::zero(sign),
            (Fp80Kind::Finite, Fp80Kind::Finite) => {
                let prod = (a.sig as u128) * (b.sig as u128);
                let (exp, hi, lo) = if (prod >> 127) != 0 {
                    (a.exp + b.exp + 1, (prod >> 64) as u64, prod as u64)
                } else {
                    (a.exp + b.exp, (prod >> 63) as u64, (prod << 1) as u64)
                };
                Self::round_from_parts(sign, exp, hi, lo, prec_of(cw), rc_of(cw), sw)
            }
            _ => Self::QNAN_INDEF,
        }
    }

    /// Exact 80-bit division `a / b`.
    pub(super) fn div(a: Self, b: Self, cw: u16, sw: &mut u16) -> Self {
        if a.kind == Fp80Kind::Unsupported || b.kind == Fp80Kind::Unsupported {
            *sw |= SW_IE;
            return Self::QNAN_INDEF;
        }
        if a.kind == Fp80Kind::Nan || b.kind == Fp80Kind::Nan {
            return Self::select_nan(a, b, sw);
        }
        let sign = a.sign ^ b.sign;
        match (a.kind, b.kind) {
            (Fp80Kind::Zero, Fp80Kind::Zero) | (Fp80Kind::Inf, Fp80Kind::Inf) => {
                *sw |= SW_IE;
                Self::QNAN_INDEF
            }
            (Fp80Kind::Finite, Fp80Kind::Zero) => {
                *sw |= SW_ZE;
                Self::inf(sign)
            }
            (Fp80Kind::Inf, _) => Self::inf(sign),
            (Fp80Kind::Zero, _) | (_, Fp80Kind::Inf) => Self::zero(sign),
            (Fp80Kind::Finite, Fp80Kind::Finite) => {
                let den = b.sig as u128;
                let (exp, num) = if a.sig >= b.sig {
                    (a.exp - b.exp, (a.sig as u128) << 63)
                } else {
                    (a.exp - b.exp - 1, (a.sig as u128) << 64)
                };
                let q = (num / den) as u64;
                let rem = num % den;
                let rem2 = rem << 1;
                let rbit = (rem2 >= den) as u64;
                let rem_after = if rbit != 0 { rem2 - den } else { rem2 };
                let lo = (rbit << 63) | ((rem_after != 0) as u64);
                Self::round_from_parts(sign, exp, q, lo, prec_of(cw), rc_of(cw), sw)
            }
            _ => Self::QNAN_INDEF,
        }
    }

    /// Exact 80-bit square root `sqrt(a)`.
    pub(super) fn sqrt(a: Self, cw: u16, sw: &mut u16) -> Self {
        match a.kind {
            Fp80Kind::Unsupported => {
                *sw |= SW_IE;
                Self::QNAN_INDEF
            }
            Fp80Kind::Nan => {
                if a.is_signaling_nan() {
                    *sw |= SW_IE;
                }
                Self { sig: a.sig | (1u64 << 63) | (1u64 << 62), ..a }
            }
            Fp80Kind::Zero => a, // preserves -0.0
            Fp80Kind::Inf => {
                if a.sign != 0 {
                    *sw |= SW_IE;
                    Self::QNAN_INDEF
                } else {
                    a
                }
            }
            Fp80Kind::Finite => {
                if a.sign != 0 {
                    *sw |= SW_IE;
                    return Self::QNAN_INDEF;
                }
                let (exp_even, m) = if (a.exp & 1) != 0 {
                    (a.exp - 1, (a.sig as u128) << 64)
                } else {
                    (a.exp, (a.sig as u128) << 63)
                };
                // Exact integer square root of `m` (in `[2^126, 2^128)` -> `q` in `[2^63, 2^64)`).
                let q = isqrt_u128(m) as u64;
                let rem = m - (q as u128) * (q as u128);
                // `sqrt(m) > q + 0.5` iff `rem > q` (exact tie is impossible for integers).
                let lo = if rem == 0 {
                    0u64
                } else if rem > (q as u128) {
                    (1u64 << 63) | 1
                } else {
                    1u64
                };
                Self::round_from_parts(0, exp_even >> 1, q, lo, prec_of(cw), rc_of(cw), sw)
            }
        }
    }

    /// Exact 80-bit round-to-integer (`frndint`) under rounding control `rc`.
    pub(super) fn frndint(self, rc: u8, sw: &mut u16) -> Self {
        match self.kind {
            Fp80Kind::Unsupported => {
                *sw |= SW_IE;
                Self::QNAN_INDEF
            }
            Fp80Kind::Nan => {
                if self.is_signaling_nan() {
                    *sw |= SW_IE;
                }
                Self { sig: self.sig | (1u64 << 63) | (1u64 << 62), ..self }
            }
            Fp80Kind::Zero | Fp80Kind::Inf => self,
            Fp80Kind::Finite => {
                if self.exp >= 63 {
                    return self; // already an exact integer
                }
                if self.exp < 0 {
                    *sw |= SW_PE;
                    let rbit = self.exp == -1 && (self.sig >> 63) != 0;
                    let sticky = if self.exp == -1 {
                        (self.sig & 0x7FFF_FFFF_FFFF_FFFF) != 0
                    } else {
                        true
                    };
                    if Self::should_round_up(self.sign, false, rbit, sticky, rc) {
                        return Self { kind: Fp80Kind::Finite, sign: self.sign, exp: 0, sig: 1u64 << 63 };
                    }
                    return Self::zero(self.sign);
                }
                let shift = (63 - self.exp) as u32; // in 1..=63
                let mask = (1u64 << shift) - 1;
                let frac = self.sig & mask;
                if frac == 0 {
                    return self;
                }
                *sw |= SW_PE;
                let mut int_part = self.sig >> shift;
                let half = 1u64 << (shift - 1);
                let rbit = (frac & half) != 0;
                let sticky = (frac & (half - 1)) != 0;
                let mut exp = self.exp;
                if Self::should_round_up(self.sign, (int_part & 1) != 0, rbit, sticky, rc) {
                    int_part += 1;
                    if int_part == (1u64 << (self.exp as u32 + 1)) {
                        int_part >>= 1;
                        exp += 1;
                    }
                }
                Self {
                    kind: Fp80Kind::Finite,
                    sign: self.sign,
                    exp,
                    sig: int_part << ((63 - exp) as u32),
                }
            }
        }
    }

    /// Exact conversion to a signed `bits`-bit (`16`, `32`, or `64`) integer under `rc`.
    #[inline]
    pub(super) fn to_int_bits(self, bits: u32, rc: u8, sw: &mut u16) -> u64 {
        let indef = 1u64 << (bits - 1);
        let mask = if bits == 64 { u64::MAX } else { (1u64 << bits) - 1 };
        match self.kind {
            Fp80Kind::Zero => 0,
            Fp80Kind::Inf | Fp80Kind::Nan | Fp80Kind::Unsupported => {
                *sw |= SW_IE;
                indef
            }
            Fp80Kind::Finite => {
                let mut pe = false;
                let mag = if self.exp < 0 {
                    pe = true;
                    let rbit = self.exp == -1 && (self.sig >> 63) != 0;
                    let sticky = if self.exp == -1 {
                        (self.sig & 0x7FFF_FFFF_FFFF_FFFF) != 0
                    } else {
                        true
                    };
                    u64::from(Self::should_round_up(self.sign, false, rbit, sticky, rc))
                } else if self.exp >= 64 {
                    *sw |= SW_IE;
                    return indef;
                } else if self.exp == 63 {
                    // Magnitude is in [2^63, 2^64): only valid for bits==64, sign==1, sig==1<<63 (i64::MIN).
                    if bits == 64 && self.sign != 0 && self.sig == (1u64 << 63) {
                        return indef; // exact i64::MIN, no IE/PE
                    }
                    *sw |= SW_IE;
                    return indef;
                } else {
                    let shift = (63 - self.exp) as u32;
                    let mut m = self.sig >> shift;
                    let frac = self.sig & ((1u64 << shift) - 1);
                    if frac != 0 {
                        pe = true;
                        let half = 1u64 << (shift - 1);
                        let rbit = (frac & half) != 0;
                        let sticky = (frac & (half - 1)) != 0;
                        if Self::should_round_up(self.sign, (m & 1) != 0, rbit, sticky, rc) {
                            m += 1;
                        }
                    }
                    m
                };
                let max_pos = (1u64 << (bits - 1)) - 1;
                let max_neg = 1u64 << (bits - 1);
                if self.sign == 0 {
                    if mag > max_pos {
                        *sw |= SW_IE;
                        indef
                    } else {
                        if pe {
                            *sw |= SW_PE;
                        }
                        mag & mask
                    }
                } else if mag > max_neg {
                    *sw |= SW_IE;
                    indef
                } else {
                    if pe {
                        *sw |= SW_PE;
                    }
                    0u64.wrapping_sub(mag) & mask
                }
            }
        }
    }

    /// Exact 80-bit comparison returning `None` if unordered (NaN/Unsupported), else `Some(Ordering)`.
    pub(super) fn compare(a: Self, b: Self, signaling: bool, sw: &mut u16) -> Option<Ordering> {
        if a.kind == Fp80Kind::Unsupported || b.kind == Fp80Kind::Unsupported {
            *sw |= SW_IE;
            return None;
        }
        if a.kind == Fp80Kind::Nan || b.kind == Fp80Kind::Nan {
            if signaling || a.is_signaling_nan() || b.is_signaling_nan() {
                *sw |= SW_IE;
            }
            return None;
        }
        if a.kind == Fp80Kind::Zero && b.kind == Fp80Kind::Zero {
            return Some(Ordering::Equal);
        }
        if a.sign != b.sign {
            return Some(if a.sign != 0 { Ordering::Less } else { Ordering::Greater });
        }
        let mag_ord = match (a.kind, b.kind) {
            (Fp80Kind::Zero, Fp80Kind::Zero) => Ordering::Equal,
            (Fp80Kind::Zero, _) => Ordering::Less,
            (_, Fp80Kind::Zero) => Ordering::Greater,
            (Fp80Kind::Inf, Fp80Kind::Inf) => Ordering::Equal,
            (Fp80Kind::Inf, _) => Ordering::Greater,
            (_, Fp80Kind::Inf) => Ordering::Less,
            (Fp80Kind::Finite, Fp80Kind::Finite) => a.exp.cmp(&b.exp).then_with(|| a.sig.cmp(&b.sig)),
            _ => Ordering::Equal,
        };
        Some(if a.sign != 0 { mag_ord.reverse() } else { mag_ord })
    }
}

/// Exact floor integer square root of a `u128`.
#[inline]
fn isqrt_u128(n: u128) -> u128 {
    if n < 2 {
        return n;
    }
    // Initial guess from highest bit: 2^ceil(bits/2).
    let shift = (128 - n.leading_zeros()).div_ceil(2);
    let mut x = 1u128 << shift;
    loop {
        let next = (x + n / x) >> 1;
        if next >= x {
            return x;
        }
        x = next;
    }
}

#[inline(always)]
pub(super) const fn prec_of(cw: u16) -> u32 {
    match (cw >> 8) & 3 {
        0 => 24,
        2 => 53,
        _ => 64,
    }
}

#[inline(always)]
pub(super) const fn rc_of(cw: u16) -> u8 {
    ((cw >> 10) & 3) as u8
}

// =================================================================================================
// Shared `#[repr(C, align(16))]` x87 / MMX Register-File State
// =================================================================================================

/// Architectural x87 / MMX register-stack state (`ST(0)..ST(7)` / `MM0..MM7`, `CW`, `SW`, `TOP`,
/// abridged tag mask `valid_mask`, and FPU environment pointers `FOP`/`FIP`/`FDP`).
///
/// Layout is fixed (`#[repr(C, align(16))]`) and directly addressed by ARM64 JIT/AOT code.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C, align(16))]
pub struct X87State {
    /// Physical register slots `R0..R7`, 16 bytes each (matching `FXSAVE`):
    /// bytes `0..8` = 64-bit significand / MMX register, `8..10` = sign+exp (`0xFFFF` for MMX),
    /// `10..16` = zero padding.
    pub(super) regs: [[u8; 16]; 8],
    pub(super) cw: u16,
    pub(super) sw: u16,
    pub(super) fop: u16,
    pub(super) top: u8,
    pub(super) valid_mask: u8,
    pub(super) fip: u64,
    pub(super) fdp: u64,
}

pub(super) const OFF_X87_REGS: u32 = offset_of!(X87State, regs) as u32;
pub(super) const OFF_X87_CW: u32 = offset_of!(X87State, cw) as u32;
pub(super) const OFF_X87_SW: u32 = offset_of!(X87State, sw) as u32;
pub(super) const OFF_X87_FOP: u32 = offset_of!(X87State, fop) as u32;
pub(super) const OFF_X87_TOP: u32 = offset_of!(X87State, top) as u32;
pub(super) const OFF_X87_VALID: u32 = offset_of!(X87State, valid_mask) as u32;
pub(super) const OFF_X87_FIP: u32 = offset_of!(X87State, fip) as u32;
pub(super) const OFF_X87_FDP: u32 = offset_of!(X87State, fdp) as u32;

const _: () = {
    assert!(OFF_X87_REGS == 0x00);
    assert!(OFF_X87_CW == 0x80);
    assert!(OFF_X87_SW == 0x82);
    assert!(OFF_X87_FOP == 0x84);
    assert!(OFF_X87_TOP == 0x86);
    assert!(OFF_X87_VALID == 0x87);
    assert!(OFF_X87_FIP == 0x88);
    assert!(OFF_X87_FDP == 0x90);
    assert!(core::mem::size_of::<X87State>() == 0xA0);
};

impl Default for X87State {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

/// Binary arithmetic operation code shared across `X87State`, `FastOp`, and ARM64 JIT/AOT helpers.
pub(super) const X87_OP_ADD: u8 = 0;
pub(super) const X87_OP_SUB: u8 = 1;
pub(super) const X87_OP_SUBR: u8 = 2;
pub(super) const X87_OP_MUL: u8 = 3;
pub(super) const X87_OP_DIV: u8 = 4;
pub(super) const X87_OP_DIVR: u8 = 5;

/// Transcendental / unary helper opcode.
pub(super) const X87_UNARY_SQRT: u8 = 0;
pub(super) const X87_UNARY_RNDINT: u8 = 1;
pub(super) const X87_UNARY_SIN: u8 = 2;
pub(super) const X87_UNARY_COS: u8 = 3;
pub(super) const X87_UNARY_SINCOS: u8 = 4;
pub(super) const X87_UNARY_PTAN: u8 = 5;
pub(super) const X87_UNARY_PATAN: u8 = 6;
pub(super) const X87_UNARY_2XM1: u8 = 7;
pub(super) const X87_UNARY_YL2X: u8 = 8;
pub(super) const X87_UNARY_YL2XP1: u8 = 9;
pub(super) const X87_UNARY_SCALE: u8 = 10;
pub(super) const X87_UNARY_XTRACT: u8 = 11;
pub(super) const X87_UNARY_PREM: u8 = 12;
pub(super) const X87_UNARY_PREM1: u8 = 13;

/// Exact 80-bit x87 constant IDs (`fldz..fldln2`).
pub(super) const X87_CONST_ZERO: u8 = 0;
pub(super) const X87_CONST_ONE: u8 = 1;
pub(super) const X87_CONST_PI: u8 = 2;
pub(super) const X87_CONST_L2E: u8 = 3;
pub(super) const X87_CONST_L2T: u8 = 4;
pub(super) const X87_CONST_LG2: u8 = 5;
pub(super) const X87_CONST_LN2: u8 = 6;

impl X87State {
    #[inline]
    pub const fn new() -> Self {
        Self {
            regs: [[0; 16]; 8],
            cw: 0x037F,
            sw: 0,
            fop: 0,
            top: 0,
            valid_mask: 0,
            fip: 0,
            fdp: 0,
        }
    }

    #[inline(always)]
    pub(super) fn phys(&self, i: u8) -> usize {
        (self.top.wrapping_add(i) & 7) as usize
    }

    #[inline(always)]
    pub(super) fn slot_bytes(&self, slot: usize) -> [u8; 10] {
        let r = &self.regs[slot];
        [r[0], r[1], r[2], r[3], r[4], r[5], r[6], r[7], r[8], r[9]]
    }

    #[inline(always)]
    pub(super) fn set_slot_bytes(&mut self, slot: usize, val: [u8; 10]) {
        self.regs[slot][..10].copy_from_slice(&val);
        self.regs[slot][10..16].fill(0);
    }

    #[inline(always)]
    pub(super) fn value(&self, slot: usize) -> Fp80 {
        Fp80::from_f80_bytes(self.slot_bytes(slot))
    }

    #[inline(always)]
    pub(super) fn raw_denormal(&self, slot: usize) -> bool {
        let raw = &self.regs[slot];
        self.valid_mask & (1 << slot) != 0 && raw[8] == 0 && (raw[9] & 0x7f) == 0 && raw[..8] != [0; 8]
    }

    #[inline(always)]
    pub(super) fn sync_top(&mut self) {
        self.sw = (self.sw & !(7u16 << 11)) | (((self.top & 7) as u16) << 11);
    }

    #[inline]
    pub fn status_word(&self) -> u16 {
        (self.sw & !(7u16 << 11)) | (((self.top & 7) as u16) << 11)
    }

    /// Compute the full 16-bit x87 tag word (2 bits per physical slot R0..R7) per Intel SDM Vol. 1 §8.1.7.
    pub(super) fn full_tag_word(&self) -> u16 {
        let mut tw = 0u16;
        for p in 0..8usize {
            let tag: u16 = if (self.valid_mask & (1 << p)) == 0 {
                0b11 // Empty
            } else {
                let raw = &self.regs[p];
                let sig = u64::from_le_bytes(raw[0..8].try_into().unwrap());
                let exp = u16::from_le_bytes([raw[8], raw[9]]) & 0x7FFF;
                if exp == 0 && sig == 0 {
                    0b01 // Zero
                } else if exp == 0x7FFF || exp == 0 || (sig >> 63) == 0 {
                    0b10 // Special (Inf, NaN, denormal, unsupported)
                } else {
                    0b00 // Valid normal
                }
            };
            tw |= tag << (2 * p);
        }
        tw
    }

    /// Read `ST(idx)` as IEEE-754 f64 bits without mutating the x87 stack or status word.
    #[inline]
    pub fn peek_f64_bits(&self, idx: u8) -> Option<u64> {
        let p = self.phys(idx);
        if (self.valid_mask & (1 << p)) == 0 {
            return None;
        }
        let mut sw = self.sw;
        Some(self.value(p).to_f64_bits(rc_of(self.cw), &mut sw))
    }

    /// Push raw 10-byte value onto the stack (detecting stack overflow -> `SF | IE | C1` + QNaN Indef).
    #[inline]
    pub(super) fn push_bytes(&mut self, val: [u8; 10], extra_sw: u16) {
        let mut sw = (self.sw & !SW_C1) | extra_sw;
        let new_top = self.top.wrapping_sub(1) & 7;
        let p = new_top as usize;
        if (self.valid_mask & (1 << p)) != 0 {
            sw |= SW_SF | SW_IE | SW_C1;
            self.set_slot_bytes(p, Fp80::QNAN_INDEF_BYTES);
        } else {
            self.set_slot_bytes(p, val);
            self.valid_mask |= 1 << p;
        }
        self.top = new_top;
        self.sw = sw;
        self.sync_top();
    }

    #[inline]
    pub(super) fn push_fp80(&mut self, val: Fp80, extra_sw: u16) {
        self.push_bytes(val.to_f80_bytes(), extra_sw);
    }

    #[inline]
    pub fn push_f64_bits(&mut self, bits: u64) {
        let mut extra = 0u16;
        let val = Fp80::from_f64_bits(bits, &mut extra);
        self.push_fp80(val, extra);
    }

    #[inline]
    pub(super) fn push_f32_bits(&mut self, bits: u32) {
        let mut extra = 0u16;
        let val = Fp80::from_f32_bits(bits, &mut extra);
        self.push_fp80(val, extra);
    }

    /// Push an x87 architectural constant (`X87_CONST_*`), honoring `CW.RC` for irrational constants.
    pub(super) fn push_const(&mut self, id: u8) {
        let rc = rc_of(self.cw);
        let (se, sig): (u16, u64) = match id {
            X87_CONST_ZERO => (0x0000, 0),
            X87_CONST_ONE => (0x3FFF, 1u64 << 63),
            X87_CONST_PI => (
                0x4000,
                if matches!(rc, RC_RM | RC_RZ) { 0xC90F_DAA2_2168_C234 } else { 0xC90F_DAA2_2168_C235 },
            ),
            X87_CONST_L2E => (
                0x3FFF,
                if matches!(rc, RC_RM | RC_RZ) { 0xB8AA_3B29_5C17_F0BB } else { 0xB8AA_3B29_5C17_F0BC },
            ),
            X87_CONST_L2T => (
                0x4000,
                if rc == RC_RP { 0xD49A_784B_CD1B_8AFF } else { 0xD49A_784B_CD1B_8AFE },
            ),
            X87_CONST_LG2 => (
                0x3FFD,
                if matches!(rc, RC_RM | RC_RZ) { 0x9A20_9A84_FBCF_F798 } else { 0x9A20_9A84_FBCF_F799 },
            ),
            _ => (
                // X87_CONST_LN2
                0x3FFE,
                if matches!(rc, RC_RM | RC_RZ) { 0xB172_17F7_D1CF_79AB } else { 0xB172_17F7_D1CF_79AC },
            ),
        };
        let mut bytes = [0u8; 10];
        bytes[0..8].copy_from_slice(&sig.to_le_bytes());
        bytes[8..10].copy_from_slice(&se.to_le_bytes());
        self.push_bytes(bytes, 0);
    }

    /// `fld st(idx)`: copy `ST(idx)` and push onto stack.
    #[inline]
    pub(super) fn fld_st(&mut self, idx: u8) {
        let pi = self.phys(idx & 7);
        if (self.valid_mask & (1 << pi)) == 0 {
            self.push_bytes(Fp80::QNAN_INDEF_BYTES, SW_SF | SW_IE);
        } else {
            let val = self.slot_bytes(pi);
            self.push_bytes(val, 0);
        }
    }

    #[inline]
    pub(super) fn fst_st(&mut self, idx: u8) {
        let p0 = self.phys(0);
        let pi = self.phys(idx & 7);
        let v = self.slot_bytes(p0);
        if (self.valid_mask & (1 << p0)) == 0 {
            self.sw = (self.sw & !SW_C1) | SW_SF | SW_IE;
            self.set_slot_bytes(pi, Fp80::QNAN_INDEF_BYTES);
            self.valid_mask |= 1 << pi;
        } else {
            self.set_slot_bytes(pi, v);
            self.valid_mask |= 1 << pi;
            self.sw &= !SW_C1;
        }
        self.sync_top();
    }

    #[inline]
    pub fn pop_to_st(&mut self, idx: u8) {
        let idx = idx & 7;
        let p0 = self.phys(0);
        let pi = self.phys(idx);
        let v = self.slot_bytes(p0);
        if idx != 0 {
            if (self.valid_mask & (1 << p0)) == 0 {
                self.sw |= SW_SF | SW_IE;
                self.set_slot_bytes(pi, Fp80::QNAN_INDEF_BYTES);
            } else {
                self.set_slot_bytes(pi, v);
            }
            self.valid_mask |= 1 << pi;
        }
        self.valid_mask &= !(1 << p0);
        self.top = self.top.wrapping_add(1) & 7;
        self.sw &= !SW_C1;
        self.sync_top();
    }

    #[inline]
    pub fn pop_discard(&mut self) {
        let p0 = self.phys(0);
        self.valid_mask &= !(1 << p0);
        self.top = self.top.wrapping_add(1) & 7;
        self.sw &= !SW_C1;
        self.sync_top();
    }

    #[inline]
    pub(super) fn store_f64_bits(&mut self, pop: bool) -> u64 {
        let p0 = self.phys(0);
        let rc = rc_of(self.cw);
        let mut sw = self.sw & !SW_C1;
        let out = if (self.valid_mask & (1 << p0)) == 0 {
            sw |= SW_SF | SW_IE;
            0xFFF8_0000_0000_0000
        } else {
            let v = self.value(p0);
            if pop {
                self.valid_mask &= !(1 << p0);
            }
            v.to_f64_bits(rc, &mut sw)
        };
        if pop {
            self.top = self.top.wrapping_add(1) & 7;
        }
        self.sw = sw;
        self.sync_top();
        out
    }

    #[inline]
    pub fn pop_f64_bits(&mut self) -> u64 {
        self.store_f64_bits(true)
    }

    #[inline]
    pub(super) fn store_f32_bits(&mut self, pop: bool) -> u32 {
        let p0 = self.phys(0);
        let rc = rc_of(self.cw);
        let mut sw = self.sw & !SW_C1;
        let out = if (self.valid_mask & (1 << p0)) == 0 {
            sw |= SW_SF | SW_IE;
            0xFFC0_0000
        } else {
            let v = self.value(p0);
            if pop {
                self.valid_mask &= !(1 << p0);
            }
            v.to_f32_bits(rc, &mut sw)
        };
        if pop {
            self.top = self.top.wrapping_add(1) & 7;
        }
        self.sw = sw;
        self.sync_top();
        out
    }

    #[inline]
    pub(super) fn pop_f80_bytes(&mut self) -> [u8; 10] {
        let p0 = self.phys(0);
        let val = if (self.valid_mask & (1 << p0)) == 0 {
            self.sw |= SW_SF | SW_IE;
            Fp80::QNAN_INDEF_BYTES
        } else {
            let v = self.slot_bytes(p0);
            self.valid_mask &= !(1 << p0);
            v
        };
        self.top = self.top.wrapping_add(1) & 7;
        self.sw &= !SW_C1;
        self.sync_top();
        val
    }

    #[inline]
    pub(super) fn store_int_bits(&mut self, bits: u32, pop: bool, trunc: bool) -> u64 {
        let p0 = self.phys(0);
        let rc = if trunc { RC_RZ } else { rc_of(self.cw) };
        let mut sw = self.sw & !SW_C1;
        let v = if (self.valid_mask & (1 << p0)) == 0 {
            sw |= SW_SF | SW_IE;
            1u64 << (bits - 1)
        } else {
            self.value(p0).to_int_bits(bits, rc, &mut sw)
        };
        if pop {
            self.valid_mask &= !(1 << p0);
            self.top = self.top.wrapping_add(1) & 7;
        }
        self.sw = sw;
        self.sync_top();
        v
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
        self.sw &= !SW_C_MASK;
        let p0 = self.phys(0);
        if (self.valid_mask & (1 << p0)) == 0 {
            // Empty: C3=1, C2=0, C0=1, C1=sign of slot? Intel SDM: C1=sign bit of ST(0) or 0; keep 0.
            self.sw |= SW_C3 | SW_C0;
            self.sync_top();
            return;
        }
        let v = self.value(p0);
        if v.sign != 0 {
            self.sw |= SW_C1;
        }
        match v.kind {
            Fp80Kind::Unsupported => {} // C3=0, C2=0, C0=0
            Fp80Kind::Nan => self.sw |= SW_C0,
            Fp80Kind::Finite if self.raw_denormal(p0) => self.sw |= SW_C3 | SW_C2,
            Fp80Kind::Finite => self.sw |= SW_C2,
            Fp80Kind::Inf => self.sw |= SW_C2 | SW_C0,
            Fp80Kind::Zero => self.sw |= SW_C3,
        }
        self.sync_top();
    }

    #[inline]
    pub fn fxch(&mut self, idx: u8) {
        let p0 = self.phys(0);
        let pi = self.phys(idx & 7);
        self.sw &= !SW_C1;
        let v0 = (self.valid_mask & (1 << p0)) != 0;
        let vi = (self.valid_mask & (1 << pi)) != 0;
        if !v0 || !vi {
            self.sw |= SW_SF | SW_IE;
            if !v0 {
                self.set_slot_bytes(p0, Fp80::QNAN_INDEF_BYTES);
                self.valid_mask |= 1 << p0;
            }
            if !vi {
                self.set_slot_bytes(pi, Fp80::QNAN_INDEF_BYTES);
                self.valid_mask |= 1 << pi;
            }
        }
        if p0 != pi {
            self.regs.swap(p0, pi);
        }
        self.sync_top();
    }

    /// `fcmov*` when condition is `cond_met`: copies `ST(idx)` to `ST(0)` if `cond_met`.
    #[inline]
    pub(super) fn fcmov(&mut self, idx: u8, cond_met: bool) {
        let p0 = self.phys(0);
        let pi = self.phys(idx & 7);
        self.sw &= !SW_C1;
        if (self.valid_mask & (1 << pi)) == 0 {
            if cond_met {
                self.sw |= SW_SF | SW_IE;
                self.set_slot_bytes(p0, Fp80::QNAN_INDEF_BYTES);
                self.valid_mask |= 1 << p0;
            }
        } else if cond_met {
            let v = self.slot_bytes(pi);
            self.set_slot_bytes(p0, v);
            self.valid_mask |= 1 << p0;
        }
        self.sync_top();
    }

    /// Execute binary arithmetic (`X87_OP_*`) between `ST(dst)` and either `ST(src)` or an immediate `Fp80`.
    pub(super) fn binop_st(&mut self, op: u8, dst_idx: u8, src_idx: u8, pop: bool) {
        let pd = self.phys(dst_idx & 7);
        let ps = self.phys(src_idx & 7);
        let mut sw = self.sw & !SW_C1;
        if (self.valid_mask & (1 << pd)) == 0 || (self.valid_mask & (1 << ps)) == 0 {
            sw |= SW_SF | SW_IE;
            self.set_slot_bytes(pd, Fp80::QNAN_INDEF_BYTES);
            self.valid_mask |= 1 << pd;
            self.sw = sw;
            if pop {
                self.pop_discard();
            } else {
                self.sync_top();
            }
            return;
        }
        let a = self.value(pd);
        let b = self.value(ps);
        let raw_de = self.raw_denormal(pd) || self.raw_denormal(ps);
        let res = Self::apply_binop(op, a, b, raw_de, self.cw, &mut sw);
        self.set_slot_bytes(pd, res.to_f80_bytes());
        self.valid_mask |= 1 << pd;
        self.sw = sw;
        if pop {
            self.pop_discard();
        } else {
            self.sync_top();
        }
    }

    /// Execute binary arithmetic (`X87_OP_*`) between `ST(0)` and a decoded memory operand `rhs`
    /// (with `rhs_extra_sw` carrying any `IE`/`DE` from loading `m32`/`m64`).
    pub(super) fn binop_val(&mut self, op: u8, rhs: Fp80, rhs_extra_sw: u16) {
        let p0 = self.phys(0);
        let mut sw = (self.sw & !SW_C1) | rhs_extra_sw;
        if (self.valid_mask & (1 << p0)) == 0 {
            sw |= SW_SF | SW_IE;
            self.set_slot_bytes(p0, Fp80::QNAN_INDEF_BYTES);
            self.valid_mask |= 1 << p0;
            self.sw = sw;
            self.sync_top();
            return;
        }
        let a = self.value(p0);
        let raw_de = self.raw_denormal(p0);
        let res = Self::apply_binop(op, a, rhs, raw_de, self.cw, &mut sw);
        self.set_slot_bytes(p0, res.to_f80_bytes());
        self.valid_mask |= 1 << p0;
        self.sw = sw;
        self.sync_top();
    }

    #[inline]
    fn apply_binop(op: u8, a: Fp80, b: Fp80, raw_de: bool, cw: u16, sw: &mut u16) -> Fp80 {
        if raw_de {
            let suppressed = a.kind == Fp80Kind::Nan
                || b.kind == Fp80Kind::Nan
                || a.kind == Fp80Kind::Unsupported
                || b.kind == Fp80Kind::Unsupported
                || (op == X87_OP_DIV && b.kind == Fp80Kind::Zero)
                || (op == X87_OP_DIVR && a.kind == Fp80Kind::Zero);
            if !suppressed {
                *sw |= SW_DE;
            }
        }
        match op {
            X87_OP_ADD => Fp80::add(a, b, cw, sw),
            X87_OP_SUB => Fp80::sub(a, b, cw, sw),
            X87_OP_SUBR => Fp80::sub(b, a, cw, sw),
            X87_OP_MUL => Fp80::mul(a, b, cw, sw),
            X87_OP_DIV => Fp80::div(a, b, cw, sw),
            _ => Fp80::div(b, a, cw, sw),
        }
    }

    /// Compare `ST(0)` against `ST(idx)` or a value, writing `C0/C2/C3` into `SW` (or returning
    /// EFLAGS code `0=GT, 1=LT, 2=EQ, 3=UNORD` for `fcomi*`/`fucomi*`).
    pub(super) fn compare_st(&mut self, idx: u8, signaling: bool, write_sw: bool, pops: u8) -> u8 {
        let p0 = self.phys(0);
        let pi = self.phys(idx & 7);
        let mut sw = self.sw;
        if write_sw {
            sw &= !SW_C_MASK;
        } else {
            sw &= !SW_C1;
        }
        let code = if (self.valid_mask & (1 << p0)) == 0 || (self.valid_mask & (1 << pi)) == 0 {
            sw |= SW_SF | SW_IE;
            3u8 // unordered
        } else {
            let a = self.value(p0);
            let b = self.value(pi);
            if (self.raw_denormal(p0) || self.raw_denormal(pi))
                && a.kind != Fp80Kind::Nan
                && b.kind != Fp80Kind::Nan
                && a.kind != Fp80Kind::Unsupported
                && b.kind != Fp80Kind::Unsupported
            {
                sw |= SW_DE;
            }
            match Fp80::compare(a, b, signaling, &mut sw) {
                Some(Ordering::Greater) => 0,
                Some(Ordering::Less) => 1,
                Some(Ordering::Equal) => 2,
                None => 3,
            }
        };
        if write_sw {
            sw |= match code {
                0 => 0,
                1 => SW_C0,
                2 => SW_C3,
                _ => SW_C0 | SW_C2 | SW_C3,
            };
        }
        self.sw = sw;
        for _ in 0..pops {
            self.pop_discard();
        }
        self.sync_top();
        code
    }

    pub(super) fn compare_val(&mut self, rhs: Fp80, rhs_extra_sw: u16, signaling: bool, pops: u8) {
        let p0 = self.phys(0);
        let mut sw = (self.sw & !SW_C_MASK) | rhs_extra_sw;
        let code = if (self.valid_mask & (1 << p0)) == 0 {
            sw |= SW_SF | SW_IE;
            3u8
        } else {
            let a = self.value(p0);
            if self.raw_denormal(p0)
                && a.kind != Fp80Kind::Nan
                && rhs.kind != Fp80Kind::Nan
                && a.kind != Fp80Kind::Unsupported
                && rhs.kind != Fp80Kind::Unsupported
            {
                sw |= SW_DE;
            }
            match Fp80::compare(a, rhs, signaling, &mut sw) {
                Some(Ordering::Greater) => 0,
                Some(Ordering::Less) => 1,
                Some(Ordering::Equal) => 2,
                None => 3,
            }
        };
        sw |= match code {
            0 => 0,
            1 => SW_C0,
            2 => SW_C3,
            _ => SW_C0 | SW_C2 | SW_C3,
        };
        self.sw = sw;
        for _ in 0..pops {
            self.pop_discard();
        }
        self.sync_top();
    }

    /// Execute x87 `fprem` (`ieee == false`) or `fprem1` (`ieee == true`).
    pub(super) fn fprem_impl(&mut self, ieee: bool) {
        self.sw &= !SW_C_MASK;
        let p0 = self.phys(0);
        let p1 = self.phys(1);
        if (self.valid_mask & (1 << p0)) == 0 || (self.valid_mask & (1 << p1)) == 0 {
            self.sw |= SW_SF | SW_IE;
            self.set_slot_bytes(p0, Fp80::QNAN_INDEF_BYTES);
            self.valid_mask |= 1 << p0;
            self.sync_top();
            return;
        }
        let a = self.value(p0);
        let b = self.value(p1);
        if a.kind == Fp80Kind::Unsupported || b.kind == Fp80Kind::Unsupported {
            self.sw |= SW_IE;
            self.set_slot_bytes(p0, Fp80::QNAN_INDEF_BYTES);
            self.sync_top();
            return;
        }
        if a.kind == Fp80Kind::Nan || b.kind == Fp80Kind::Nan {
            let mut sw = self.sw;
            let res = Fp80::select_nan(a, b, &mut sw);
            self.sw = sw;
            self.set_slot_bytes(p0, res.to_f80_bytes());
            self.sync_top();
            return;
        }
        if a.kind == Fp80Kind::Inf || b.kind == Fp80Kind::Zero {
            self.sw |= SW_IE;
            self.set_slot_bytes(p0, Fp80::QNAN_INDEF_BYTES);
            self.sync_top();
            return;
        }
        if self.raw_denormal(p0) || self.raw_denormal(p1) {
            self.sw |= SW_DE;
        }
        if a.kind == Fp80Kind::Zero || b.kind == Fp80Kind::Inf {
            self.sync_top();
            return;
        }

        let d = a.exp - b.exp;
        if d < 0 {
            if ieee && d == -1 {
                // |a| in [0.5 * 2^b.exp, 2^b.exp): check if 2*|a| > |b|.
                // Specifically |a| = a.sig * 2^(b.exp - 1 - 63), |b| = b.sig * 2^(b.exp - 63).
                // So 2*|a| = a.sig * 2^(b.exp - 63) vs |b| = b.sig * 2^(b.exp - 63).
                // Round-to-nearest-even with q=0: round up to q=1 iff 2*|a| > |b|, i.e. a.sig > b.sig!
                if a.sig > b.sig {
                    let rem = (b.sig as u128 * 2) - (a.sig as u128);
                    // Wait: |b| - |a| = (2*b.sig - a.sig) * 2^(b.exp - 1 - 63).
                    let clz = rem.leading_zeros() as i32; // rem in (b.sig, 2*b.sig) -> 64..65 bits
                    let shift = 64 - clz; // wait: rem < 2*b.sig < 2^65
                    let (sig, exp_adj) = if shift > 0 {
                        ((rem >> shift) as u64, shift)
                    } else {
                        ((rem << (-shift)) as u64, shift)
                    };
                    self.sw |= SW_C1; // Q=1 -> Q0=1
                    self.set_slot_bytes(
                        p0,
                        Fp80 {
                            kind: Fp80Kind::Finite,
                            sign: a.sign ^ 1,
                            exp: b.exp - 1 + exp_adj,
                            sig,
                        }
                        .to_f80_bytes(),
                    );
                }
            }
            self.sync_top();
            return;
        }

        if d < 64 {
            let num = (a.sig as u128) << (d as u32);
            let den = b.sig as u128;
            let mut q = num / den;
            let mut rem = num % den;
            let mut res_sign = a.sign;
            if ieee && rem != 0 {
                let rem2 = rem << 1;
                if rem2 > den || (rem2 == den && (q & 1) != 0) {
                    q += 1;
                    rem = den - rem;
                    res_sign ^= 1;
                }
            }
            let q3 = (q & 7) as u32;
            if (q3 & 1) != 0 {
                self.sw |= SW_C1;
            }
            if (q3 & 2) != 0 {
                self.sw |= SW_C3;
            }
            if (q3 & 4) != 0 {
                self.sw |= SW_C0;
            }
            let rem64 = rem as u64;
            if rem64 == 0 {
                self.set_slot_bytes(p0, Fp80::zero(a.sign).to_f80_bytes());
            } else {
                let clz = rem64.leading_zeros() as i32;
                self.set_slot_bytes(
                    p0,
                    Fp80 {
                        kind: Fp80Kind::Finite,
                        sign: res_sign,
                        exp: b.exp - clz,
                        sig: rem64 << (clz as u32),
                    }
                    .to_f80_bytes(),
                );
            }
        } else {
            let n = 32 + (d & 31);
            let num = (a.sig as u128) << (n as u32);
            let rem = (num % (b.sig as u128)) as u64;
            self.sw |= SW_C2;
            if rem == 0 {
                self.set_slot_bytes(p0, Fp80::zero(a.sign).to_f80_bytes());
            } else {
                let clz = rem.leading_zeros() as i32;
                self.set_slot_bytes(
                    p0,
                    Fp80 {
                        kind: Fp80Kind::Finite,
                        sign: a.sign,
                        exp: (a.exp - n) - clz,
                        sig: rem << (clz as u32),
                    }
                    .to_f80_bytes(),
                );
            }
        }
        self.sync_top();
    }

    #[inline]
    pub fn fprem(&mut self) {
        self.fprem_impl(false);
    }

    /// Execute unary & transcendental x87 operations (`X87_UNARY_*`).
    pub(super) fn exec_unary(&mut self, op: u8) {
        match op {
            X87_UNARY_SQRT => {
                let p0 = self.phys(0);
                let mut sw = self.sw & !SW_C1;
                if (self.valid_mask & (1 << p0)) == 0 {
                    sw |= SW_SF | SW_IE;
                    self.set_slot_bytes(p0, Fp80::QNAN_INDEF_BYTES);
                    self.valid_mask |= 1 << p0;
                } else {
                    let a = self.value(p0);
                    if self.raw_denormal(p0) && a.sign == 0 && a.kind == Fp80Kind::Finite {
                        sw |= SW_DE;
                    }
                    let r = Fp80::sqrt(a, self.cw, &mut sw);
                    self.set_slot_bytes(p0, r.to_f80_bytes());
                }
                self.sw = sw;
                self.sync_top();
            }
            X87_UNARY_RNDINT => {
                let p0 = self.phys(0);
                let mut sw = self.sw & !SW_C1;
                if (self.valid_mask & (1 << p0)) == 0 {
                    sw |= SW_SF | SW_IE;
                    self.set_slot_bytes(p0, Fp80::QNAN_INDEF_BYTES);
                    self.valid_mask |= 1 << p0;
                } else {
                    let a = self.value(p0);
                    if self.raw_denormal(p0) && a.kind == Fp80Kind::Finite {
                        sw |= SW_DE;
                    }
                    let r = a.frndint(rc_of(self.cw), &mut sw);
                    self.set_slot_bytes(p0, r.to_f80_bytes());
                }
                self.sw = sw;
                self.sync_top();
            }
            X87_UNARY_PREM => self.fprem_impl(false),
            X87_UNARY_PREM1 => self.fprem_impl(true),
            X87_UNARY_SCALE => {
                let p0 = self.phys(0);
                let p1 = self.phys(1);
                let mut sw = self.sw & !SW_C1;
                if (self.valid_mask & (1 << p0)) == 0 || (self.valid_mask & (1 << p1)) == 0 {
                    sw |= SW_SF | SW_IE;
                    self.set_slot_bytes(p0, Fp80::QNAN_INDEF_BYTES);
                    self.valid_mask |= 1 << p0;
                    self.sw = sw;
                    self.sync_top();
                    return;
                }
                let a = self.value(p0);
                let b = self.value(p1);
                let res = if a.kind == Fp80Kind::Unsupported || b.kind == Fp80Kind::Unsupported {
                    sw |= SW_IE;
                    Fp80::QNAN_INDEF
                } else if a.kind == Fp80Kind::Nan || b.kind == Fp80Kind::Nan {
                    Fp80::select_nan(a, b, &mut sw)
                } else {
                    if self.raw_denormal(p0) || self.raw_denormal(p1) {
                        sw |= SW_DE;
                    }
                    match (a.kind, b.kind) {
                        (Fp80Kind::Zero, Fp80Kind::Inf) if b.sign == 0 => {
                            sw |= SW_IE;
                            Fp80::QNAN_INDEF
                        }
                        (Fp80Kind::Inf, Fp80Kind::Inf) if b.sign != 0 => {
                            sw |= SW_IE;
                            Fp80::QNAN_INDEF
                        }
                        (Fp80Kind::Zero, _) => a,
                        (Fp80Kind::Inf, _) => a,
                        (Fp80Kind::Finite, Fp80Kind::Inf) => {
                            if b.sign == 0 {
                                sw |= SW_OE | SW_PE;
                                Fp80::inf(a.sign)
                            } else {
                                sw |= SW_UE | SW_PE;
                                Fp80::zero(a.sign)
                            }
                        }
                        (Fp80Kind::Finite, Fp80Kind::Zero) => a,
                        (Fp80Kind::Finite, Fp80Kind::Finite) => {
                            let scale: i32 = if b.exp < 0 {
                                0
                            } else if b.exp >= 30 {
                                if b.sign != 0 { -1_000_000 } else { 1_000_000 }
                            } else {
                                let shift = (63 - b.exp) as u32;
                                let mag = (b.sig >> shift) as i32;
                                if b.sign != 0 { -mag } else { mag }
                            };
                            let new_exp = a.exp.saturating_add(scale);
                            Fp80::round_from_parts(a.sign, new_exp, a.sig, 0, 64, rc_of(self.cw), &mut sw)
                        }
                        _ => Fp80::QNAN_INDEF,
                    }
                };
                self.set_slot_bytes(p0, res.to_f80_bytes());
                self.sw = sw;
                self.sync_top();
            }
            X87_UNARY_XTRACT => {
                let p0 = self.phys(0);
                let mut sw = self.sw & !SW_C1;
                if (self.valid_mask & (1 << p0)) == 0 {
                    sw |= SW_SF | SW_IE;
                    self.set_slot_bytes(p0, Fp80::QNAN_INDEF_BYTES);
                    self.valid_mask |= 1 << p0;
                    self.sw = sw;
                    self.push_bytes(Fp80::QNAN_INDEF_BYTES, 0);
                    return;
                }
                let a = self.value(p0);
                let (exp_val, sig_val) = match a.kind {
                    Fp80Kind::Unsupported => {
                        sw |= SW_IE;
                        (Fp80::QNAN_INDEF, Fp80::QNAN_INDEF)
                    }
                    Fp80Kind::Nan => {
                        if a.is_signaling_nan() {
                            sw |= SW_IE;
                        }
                        let q = Fp80 { sig: a.sig | (1u64 << 63) | (1u64 << 62), ..a };
                        (q, q)
                    }
                    Fp80Kind::Zero => {
                        sw |= SW_ZE;
                        (Fp80::inf(1), a)
                    }
                    Fp80Kind::Inf => (Fp80::inf(0), a),
                    Fp80Kind::Finite => {
                        if self.raw_denormal(p0) {
                            sw |= SW_DE;
                        }
                        (
                            Fp80::from_i64(a.exp as i64),
                            Fp80 { kind: Fp80Kind::Finite, sign: a.sign, exp: 0, sig: a.sig },
                        )
                    }
                };
                self.set_slot_bytes(p0, exp_val.to_f80_bytes());
                self.sw = sw;
                self.push_fp80(sig_val, 0);
            }
            X87_UNARY_SIN | X87_UNARY_COS | X87_UNARY_SINCOS | X87_UNARY_PTAN => {
                let p0 = self.phys(0);
                let mut sw = self.sw & !(SW_C1 | SW_C2);
                if (self.valid_mask & (1 << p0)) == 0 {
                    sw |= SW_SF | SW_IE;
                    self.set_slot_bytes(p0, Fp80::QNAN_INDEF_BYTES);
                    self.valid_mask |= 1 << p0;
                    self.sw = sw;
                    if matches!(op, X87_UNARY_SINCOS | X87_UNARY_PTAN) {
                        self.push_bytes(Fp80::QNAN_INDEF_BYTES, 0);
                    } else {
                        self.sync_top();
                    }
                    return;
                }
                let a = self.value(p0);
                if a.kind == Fp80Kind::Unsupported {
                    sw |= SW_IE;
                    self.set_slot_bytes(p0, Fp80::QNAN_INDEF_BYTES);
                    self.sw = sw;
                    if matches!(op, X87_UNARY_SINCOS | X87_UNARY_PTAN) {
                        self.push_bytes(Fp80::QNAN_INDEF_BYTES, 0);
                    } else {
                        self.sync_top();
                    }
                    return;
                }
                if a.kind == Fp80Kind::Nan {
                    if a.is_signaling_nan() {
                        sw |= SW_IE;
                    }
                    let q = Fp80 { sig: a.sig | (1u64 << 63) | (1u64 << 62), ..a };
                    self.set_slot_bytes(p0, q.to_f80_bytes());
                    self.sw = sw;
                    if matches!(op, X87_UNARY_SINCOS | X87_UNARY_PTAN) {
                        self.push_fp80(q, 0);
                    } else {
                        self.sync_top();
                    }
                    return;
                }
                if a.kind == Fp80Kind::Inf {
                    sw |= SW_IE;
                    self.set_slot_bytes(p0, Fp80::QNAN_INDEF_BYTES);
                    self.sw = sw;
                    if matches!(op, X87_UNARY_SINCOS | X87_UNARY_PTAN) {
                        self.push_bytes(Fp80::QNAN_INDEF_BYTES, 0);
                    } else {
                        self.sync_top();
                    }
                    return;
                }
                if self.raw_denormal(p0) {
                    sw |= SW_DE;
                }
                if a.kind == Fp80Kind::Finite && a.exp >= 63 {
                    // Out of range: |ST(0)| >= 2^63 -> set C2=1, leave ST(0) unchanged, no push.
                    sw |= SW_C2;
                    self.sw = sw;
                    self.sync_top();
                    return;
                }
                let mut dummy = 0u16;
                let x = f64::from_bits(a.to_f64_bits(RC_RN, &mut dummy));
                match op {
                    X87_UNARY_SIN => {
                        let r = if a.kind == Fp80Kind::Zero { a } else { sw |= SW_PE; Fp80::from_f64_raw(x.sin()) };
                        self.set_slot_bytes(p0, r.to_f80_bytes());
                        self.sw = sw;
                        self.sync_top();
                    }
                    X87_UNARY_COS => {
                        let r = if a.kind == Fp80Kind::Zero { Fp80::one() } else { sw |= SW_PE; Fp80::from_f64_raw(x.cos()) };
                        self.set_slot_bytes(p0, r.to_f80_bytes());
                        self.sw = sw;
                        self.sync_top();
                    }
                    X87_UNARY_SINCOS => {
                        let (s_val, c_val) = if a.kind == Fp80Kind::Zero {
                            (a, Fp80::one())
                        } else {
                            sw |= SW_PE;
                            let (s, c) = x.sin_cos();
                            (Fp80::from_f64_raw(s), Fp80::from_f64_raw(c))
                        };
                        self.set_slot_bytes(p0, s_val.to_f80_bytes());
                        self.sw = sw;
                        self.push_fp80(c_val, 0);
                    }
                    _ => {
                        // X87_UNARY_PTAN: ST(0) = tan(x), then push +1.0.
                        let t_val = if a.kind == Fp80Kind::Zero {
                            a
                        } else {
                            sw |= SW_PE;
                            Fp80::from_f64_raw(x.tan())
                        };
                        self.set_slot_bytes(p0, t_val.to_f80_bytes());
                        self.sw = sw;
                        self.push_fp80(Fp80::one(), 0);
                    }
                }
            }
            X87_UNARY_PATAN => {
                // ST(1) = atan2(ST(1), ST(0)), pop ST(0).
                let p0 = self.phys(0);
                let p1 = self.phys(1);
                let mut sw = self.sw & !SW_C1;
                if (self.valid_mask & (1 << p0)) == 0 || (self.valid_mask & (1 << p1)) == 0 {
                    sw |= SW_SF | SW_IE;
                    self.set_slot_bytes(p1, Fp80::QNAN_INDEF_BYTES);
                    self.valid_mask |= 1 << p1;
                    self.sw = sw;
                    self.pop_discard();
                    return;
                }
                let x = self.value(p0);
                let y = self.value(p1);
                let res = if x.kind == Fp80Kind::Unsupported || y.kind == Fp80Kind::Unsupported {
                    sw |= SW_IE;
                    Fp80::QNAN_INDEF
                } else if x.kind == Fp80Kind::Nan || y.kind == Fp80Kind::Nan {
                    Fp80::select_nan(x, y, &mut sw)
                } else {
                    if self.raw_denormal(p0) || self.raw_denormal(p1) {
                        sw |= SW_DE;
                    }
                    let mut dummy = 0u16;
                    let xf = f64::from_bits(x.to_f64_bits(RC_RN, &mut dummy));
                    let yf = f64::from_bits(y.to_f64_bits(RC_RN, &mut dummy));
                    let r = yf.atan2(xf);
                    if r != 0.0 {
                        sw |= SW_PE;
                    }
                    Fp80::from_f64_raw(r)
                };
                self.set_slot_bytes(p1, res.to_f80_bytes());
                self.valid_mask |= 1 << p1;
                self.sw = sw;
                self.pop_discard();
            }
            X87_UNARY_2XM1 => {
                // ST(0) = 2^(ST(0)) - 1 (valid for -1.0 <= ST(0) <= +1.0).
                let p0 = self.phys(0);
                let mut sw = self.sw & !SW_C1;
                if (self.valid_mask & (1 << p0)) == 0 {
                    sw |= SW_SF | SW_IE;
                    self.set_slot_bytes(p0, Fp80::QNAN_INDEF_BYTES);
                    self.valid_mask |= 1 << p0;
                } else {
                    let a = self.value(p0);
                    let res = match a.kind {
                        Fp80Kind::Unsupported => {
                            sw |= SW_IE;
                            Fp80::QNAN_INDEF
                        }
                        Fp80Kind::Nan => {
                            if a.is_signaling_nan() {
                                sw |= SW_IE;
                            }
                            Fp80 { sig: a.sig | (1u64 << 63) | (1u64 << 62), ..a }
                        }
                        Fp80Kind::Zero => a, // preserves ±0.0
                        Fp80Kind::Inf => {
                            if a.sign != 0 {
                                Fp80 { kind: Fp80Kind::Finite, sign: 1, exp: 0, sig: 1u64 << 63 } // -1.0
                            } else {
                                a
                            }
                        }
                        Fp80Kind::Finite => {
                            if self.raw_denormal(p0) {
                                sw |= SW_DE;
                            }
                            let mut dummy = 0u16;
                            let x = f64::from_bits(a.to_f64_bits(RC_RN, &mut dummy));
                            let r = (x * core::f64::consts::LN_2).exp_m1();
                            sw |= SW_PE;
                            Fp80::from_f64_raw(r)
                        }
                    };
                    self.set_slot_bytes(p0, res.to_f80_bytes());
                }
                self.sw = sw;
                self.sync_top();
            }
            X87_UNARY_YL2X | X87_UNARY_YL2XP1 => {
                // fyl2x: ST(1) = ST(1) * log2(ST(0)), pop.
                // fyl2xp1: ST(1) = ST(1) * log2(ST(0) + 1.0), pop.
                let is_p1 = op == X87_UNARY_YL2XP1;
                let p0 = self.phys(0);
                let p1 = self.phys(1);
                let mut sw = self.sw & !SW_C1;
                if (self.valid_mask & (1 << p0)) == 0 || (self.valid_mask & (1 << p1)) == 0 {
                    sw |= SW_SF | SW_IE;
                    self.set_slot_bytes(p1, Fp80::QNAN_INDEF_BYTES);
                    self.valid_mask |= 1 << p1;
                    self.sw = sw;
                    self.pop_discard();
                    return;
                }
                let x = self.value(p0);
                let y = self.value(p1);
                let res = if x.kind == Fp80Kind::Unsupported || y.kind == Fp80Kind::Unsupported {
                    sw |= SW_IE;
                    Fp80::QNAN_INDEF
                } else if x.kind == Fp80Kind::Nan || y.kind == Fp80Kind::Nan {
                    Fp80::select_nan(x, y, &mut sw)
                } else {
                    if self.raw_denormal(p0) || self.raw_denormal(p1) {
                        sw |= SW_DE;
                    }
                    let mut dummy = 0u16;
                    let xf = f64::from_bits(x.to_f64_bits(RC_RN, &mut dummy));
                    let yf = f64::from_bits(y.to_f64_bits(RC_RN, &mut dummy));
                    if !is_p1 {
                        if x.kind == Fp80Kind::Zero {
                            if y.kind == Fp80Kind::Zero {
                                sw |= SW_IE;
                                Fp80::QNAN_INDEF
                            } else {
                                sw |= SW_ZE;
                                Fp80::inf(y.sign ^ 1)
                            }
                        } else if x.sign != 0 {
                            sw |= SW_IE;
                            Fp80::QNAN_INDEF
                        } else if x.kind == Fp80Kind::Inf {
                            if y.kind == Fp80Kind::Zero {
                                sw |= SW_IE;
                                Fp80::QNAN_INDEF
                            } else {
                                Fp80::inf(y.sign)
                            }
                        } else {
                            let r = yf * xf.log2();
                            if r.is_nan() {
                                sw |= SW_IE;
                                Fp80::QNAN_INDEF
                            } else {
                                if r != 0.0 && r.is_finite() {
                                    sw |= SW_PE;
                                }
                                Fp80::from_f64_raw(r)
                            }
                        }
                    } else {
                        // fyl2xp1: y * log2(x + 1.0)
                        if x.kind == Fp80Kind::Zero {
                            if y.kind == Fp80Kind::Inf {
                                sw |= SW_IE;
                                Fp80::QNAN_INDEF
                            } else {
                                Fp80::zero(x.sign ^ y.sign)
                            }
                        } else if xf < -1.0 {
                            sw |= SW_IE;
                            Fp80::QNAN_INDEF
                        } else if xf == -1.0 {
                            if y.kind == Fp80Kind::Zero {
                                sw |= SW_IE;
                                Fp80::QNAN_INDEF
                            } else {
                                sw |= SW_ZE;
                                Fp80::inf(y.sign ^ 1)
                            }
                        } else {
                            let r = yf * (xf.ln_1p() * core::f64::consts::LOG2_E);
                            if r.is_nan() {
                                sw |= SW_IE;
                                Fp80::QNAN_INDEF
                            } else {
                                if r != 0.0 && r.is_finite() {
                                    sw |= SW_PE;
                                }
                                Fp80::from_f64_raw(r)
                            }
                        }
                    }
                };
                self.set_slot_bytes(p1, res.to_f80_bytes());
                self.valid_mask |= 1 << p1;
                self.sw = sw;
                self.pop_discard();
            }
            _ => {}
        }
    }

    /// `fbld m80bcd`: decode 10-byte packed BCD (18 decimal digits + sign bit in byte 9 bit 7)
    /// and push exact [`Fp80`] onto stack.
    pub(super) fn push_bcd(&mut self, bytes: [u8; 10]) {
        let sign = bytes[9] >> 7;
        let mut mag: u64 = 0;
        let mut place: u64 = 1;
        for &b in &bytes[0..9] {
            let lo = (b & 0x0F) as u64;
            let hi = (b >> 4) as u64;
            mag = mag.wrapping_add(lo.wrapping_mul(place));
            place = place.wrapping_mul(10);
            mag = mag.wrapping_add(hi.wrapping_mul(place));
            place = place.wrapping_mul(10);
        }
        let val = if mag == 0 {
            Fp80::zero(sign)
        } else {
            let clz = mag.leading_zeros() as i32;
            Fp80 { kind: Fp80Kind::Finite, sign, exp: 63 - clz, sig: mag << (clz as u32) }
        };
        self.push_fp80(val, 0);
    }

    /// `fbstp m80bcd`: round `ST(0)` to integer per `CW.RC`, encode 18-digit packed BCD, pop `ST(0)`.
    pub(super) fn pop_bcd(&mut self) -> [u8; 10] {
        // Intel SDM Table 4-3: packed BCD integer indefinite = FFFF C000 0000 0000 0000H.
        const BCD_INDEF: [u8; 10] = [0, 0, 0, 0, 0, 0, 0, 0xC0, 0xFF, 0xFF];
        const MAX_BCD: u64 = 999_999_999_999_999_999;

        let p0 = self.phys(0);
        let rc = rc_of(self.cw);
        let mut sw = self.sw & !SW_C1;
        let out = if (self.valid_mask & (1 << p0)) == 0 {
            sw |= SW_SF | SW_IE;
            BCD_INDEF
        } else {
            let v = self.value(p0);
            self.valid_mask &= !(1 << p0);
            match v.kind {
                Fp80Kind::Zero => {
                    let mut b = [0u8; 10];
                    b[9] = v.sign << 7;
                    b
                }
                Fp80Kind::Inf | Fp80Kind::Nan | Fp80Kind::Unsupported => {
                    sw |= SW_IE;
                    BCD_INDEF
                }
                Fp80Kind::Finite => {
                    let mut pe = false;
                    let mag_opt: Option<u64> = if v.exp < 0 {
                        pe = true;
                        let rbit = v.exp == -1 && (v.sig >> 63) != 0;
                        let sticky = if v.exp == -1 {
                            (v.sig & 0x7FFF_FFFF_FFFF_FFFF) != 0
                        } else {
                            true
                        };
                        Some(u64::from(Fp80::should_round_up(v.sign, false, rbit, sticky, rc)))
                    } else if v.exp >= 60 {
                        // 10^18 - 1 < 2^60, so exp >= 60 always overflows 18 BCD digits!
                        None
                    } else {
                        let shift = (63 - v.exp) as u32;
                        let mut m = v.sig >> shift;
                        let frac = v.sig & ((1u64 << shift) - 1);
                        if frac != 0 {
                            pe = true;
                            let half = 1u64 << (shift - 1);
                            let rbit = (frac & half) != 0;
                            let sticky = (frac & (half - 1)) != 0;
                            if Fp80::should_round_up(v.sign, (m & 1) != 0, rbit, sticky, rc) {
                                m += 1;
                            }
                        }
                        (m <= MAX_BCD).then_some(m)
                    };
                    match mag_opt {
                        Some(mut mag) => {
                            if pe {
                                sw |= SW_PE;
                            }
                            let mut b = [0u8; 10];
                            for byte in &mut b[0..9] {
                                let d0 = (mag % 10) as u8;
                                mag /= 10;
                                let d1 = (mag % 10) as u8;
                                mag /= 10;
                                *byte = d0 | (d1 << 4);
                            }
                            b[9] = v.sign << 7;
                            b
                        }
                        None => {
                            sw |= SW_IE;
                            BCD_INDEF
                        }
                    }
                }
            }
        };
        self.top = self.top.wrapping_add(1) & 7;
        self.sw = sw;
        self.sync_top();
        out
    }

    /// Serialize 28-byte (32-bit protected mode) or 14-byte (16-bit operand-size) x87 environment
    /// for `fnstenv`/`fstenv`/`fnsave`/`fsave`.
    pub(super) fn encode_env(&self, out: &mut [u8], size16: bool) {
        let cw = self.cw;
        let sw = self.status_word();
        let tw = self.full_tag_word();
        if size16 {
            out[0..2].copy_from_slice(&cw.to_le_bytes());
            out[2..4].copy_from_slice(&sw.to_le_bytes());
            out[4..6].copy_from_slice(&tw.to_le_bytes());
            out[6..8].copy_from_slice(&(self.fip as u16).to_le_bytes());
            out[8..10].copy_from_slice(&(self.fop & 0x7FF).to_le_bytes());
            out[10..12].copy_from_slice(&(self.fdp as u16).to_le_bytes());
            out[12..14].copy_from_slice(&0u16.to_le_bytes());
        } else {
            out[0..2].copy_from_slice(&cw.to_le_bytes());
            out[2..4].copy_from_slice(&0xFFFFu16.to_le_bytes());
            out[4..6].copy_from_slice(&sw.to_le_bytes());
            out[6..8].copy_from_slice(&0xFFFFu16.to_le_bytes());
            out[8..10].copy_from_slice(&tw.to_le_bytes());
            out[10..12].copy_from_slice(&0xFFFFu16.to_le_bytes());
            out[12..16].copy_from_slice(&(self.fip as u32).to_le_bytes());
            out[16..18].copy_from_slice(&0u16.to_le_bytes()); // FCS
            out[18..20].copy_from_slice(&(self.fop & 0x7FF).to_le_bytes());
            out[20..24].copy_from_slice(&(self.fdp as u32).to_le_bytes());
            out[24..26].copy_from_slice(&0u16.to_le_bytes()); // FDS
            out[26..28].copy_from_slice(&0xFFFFu16.to_le_bytes());
        }
    }

    /// Load 28-byte (32-bit) or 14-byte (16-bit) x87 environment (`fldenv`/`frstor`).
    pub(super) fn decode_env(&mut self, data: &[u8], size16: bool) {
        let (cw, sw, tw, fip, fop, fdp) = if size16 {
            (
                u16::from_le_bytes([data[0], data[1]]),
                u16::from_le_bytes([data[2], data[3]]),
                u16::from_le_bytes([data[4], data[5]]),
                u16::from_le_bytes([data[6], data[7]]) as u64,
                u16::from_le_bytes([data[8], data[9]]) & 0x7FF,
                u16::from_le_bytes([data[10], data[11]]) as u64,
            )
        } else {
            (
                u16::from_le_bytes([data[0], data[1]]),
                u16::from_le_bytes([data[4], data[5]]),
                u16::from_le_bytes([data[8], data[9]]),
                u32::from_le_bytes([data[12], data[13], data[14], data[15]]) as u64,
                u16::from_le_bytes([data[18], data[19]]) & 0x7FF,
                u32::from_le_bytes([data[20], data[21], data[22], data[23]]) as u64,
            )
        };
        self.cw = super::xstate::control_word(cw);
        self.sw = sw;
        self.top = ((sw >> 11) & 7) as u8;
        self.fip = fip;
        self.fop = fop;
        self.fdp = fdp;
        let mut vm = 0u8;
        for p in 0..8usize {
            if ((tw >> (2 * p)) & 3) != 0b11 {
                vm |= 1 << p;
            }
        }
        self.valid_mask = vm;
    }

    /// `fnstenv`/`fstenv`: serialize env into `out` (14 or 28 bytes), then mask all FPU exceptions.
    pub(super) fn stenv(&mut self, out: &mut [u8], size16: bool) {
        self.encode_env(out, size16);
        self.cw |= 0x003F; // mask all 6 x87 exceptions per Intel SDM
    }

    /// `fnsave`/`fsave`: serialize env + logical `ST(0)..ST(7)` (94 or 108 bytes), then re-init FPU.
    pub(super) fn fsave(&mut self, out: &mut [u8], size16: bool) {
        let env_len = if size16 { 14 } else { 28 };
        self.encode_env(&mut out[..env_len], size16);
        for i in 0..8usize {
            let phys = self.phys(i as u8);
            let off = env_len + i * 10;
            out[off..off + 10].copy_from_slice(&self.regs[phys][..10]);
        }
        // Re-initialize FPU state (equivalent to FNINIT, leaving register contents untouched or cleared).
        self.cw = 0x037F;
        self.sw = 0;
        self.top = 0;
        self.valid_mask = 0;
        self.fop = 0;
        self.fip = 0;
        self.fdp = 0;
    }

    /// `frstor`: load env + logical `ST(0)..ST(7)` from `data` (94 or 108 bytes).
    pub(super) fn frstor(&mut self, data: &[u8], size16: bool) {
        let env_len = if size16 { 14 } else { 28 };
        self.decode_env(&data[..env_len], size16);
        for i in 0..8usize {
            let phys = self.phys(i as u8);
            let off = env_len + i * 10;
            let mut slot = [0u8; 10];
            slot.copy_from_slice(&data[off..off + 10]);
            self.set_slot_bytes(phys, slot);
        }
    }
    #[cfg(test)]
    #[inline(always)]
    pub(super) fn compare_sw(&mut self, a: f64, b: f64) {
        let c0 = (!(a >= b) as u16) << 8;
        let c2 = ((!(a <= b) & !(a > b)) as u16) << 10;
        let c3 = ((!(a < b) & !(a > b)) as u16) << 14;
        self.sw = (self.sw & !SW_C_MASK) | c0 | c2 | c3;
        self.sync_top();
    }
}

// =================================================================================================
// Per-thread x87 state storage (`TLS_X87`)
// =================================================================================================

std::thread_local! {
    static TLS_X87: core::cell::UnsafeCell<X87State> =
        const { core::cell::UnsafeCell::new(X87State::new()) };
}

#[inline(always)]
pub(super) fn x87_state_ptr() -> usize {
    TLS_X87.with(|cell| cell.get() as usize)
}

#[inline(always)]
pub(super) fn with_x87_ref<R>(f: impl FnOnce(&X87State) -> R) -> R {
    TLS_X87.with(|cell| unsafe { f(&*cell.get()) })
}

#[inline(always)]
pub(super) fn with_x87_mut<R>(f: impl FnOnce(&mut X87State) -> R) -> R {
    TLS_X87.with(|cell| unsafe { f(&mut *cell.get()) })
}

#[inline]
pub fn x87_snapshot() -> X87State {
    with_x87_ref(|st| *st)
}

#[inline]
pub fn x87_restore(st: X87State) {
    with_x87_mut(|cur| *cur = st);
}

#[inline]
pub fn x87_reset() {
    with_x87_mut(|cur| *cur = X87State::new());
}

// =================================================================================================
// C-ABI Native Rust Helper Table for ARM64 JIT / AOT Code (Zero Relocations!)
// =================================================================================================

/// Pair of `u64` returned in `(x0, x1)` under AAPCS64.
#[repr(C)]
pub(super) struct X87Pair {
    pub(super) lo: u64,
    pub(super) hi: u64,
}

/// Static function-pointer table exposed to translated ARM64 JIT/AOT blocks via
/// `Cpu::jx.x87_helpers`. Because blocks load helper pointers indirectly through `XCPU`,
/// AOT images require zero runtime literal relocations and link cleanly into signed static
/// Mach-O `.aot.o` objects on macOS and iOS.
#[repr(C)]
pub(super) struct X87Helpers {
    pub(super) fld_f32: unsafe extern "C" fn(*mut X87State, u32),
    pub(super) fld_f64: unsafe extern "C" fn(*mut X87State, u64),
    pub(super) fld_f80: unsafe extern "C" fn(*mut X87State, u64, u64),
    pub(super) fld_st: unsafe extern "C" fn(*mut X87State, u32),
    pub(super) fild_i64: unsafe extern "C" fn(*mut X87State, i64),
    pub(super) fld_const: unsafe extern "C" fn(*mut X87State, u32),
    pub(super) fst_f32: unsafe extern "C" fn(*mut X87State, u32) -> u32,
    pub(super) fst_f64: unsafe extern "C" fn(*mut X87State, u32) -> u64,
    pub(super) fstp_f80: unsafe extern "C" fn(*mut X87State) -> X87Pair,
    pub(super) fst_st: unsafe extern "C" fn(*mut X87State, u32, u32),
    pub(super) fist: unsafe extern "C" fn(*mut X87State, u32, u32) -> u64,
    pub(super) binop_st: unsafe extern "C" fn(*mut X87State, u32, u32, u32),
    pub(super) binop_f32: unsafe extern "C" fn(*mut X87State, u32, u32),
    pub(super) binop_f64: unsafe extern "C" fn(*mut X87State, u32, u64),
    pub(super) binop_i64: unsafe extern "C" fn(*mut X87State, u32, i64),
    pub(super) unary: unsafe extern "C" fn(*mut X87State, u32),
    pub(super) compare_st: unsafe extern "C" fn(*mut X87State, u32, u32) -> u64,
    pub(super) compare_f32: unsafe extern "C" fn(*mut X87State, u32, u32),
    pub(super) compare_f64: unsafe extern "C" fn(*mut X87State, u64, u32),
    pub(super) compare_i64: unsafe extern "C" fn(*mut X87State, i64, u32),
    pub(super) fxam: unsafe extern "C" fn(*mut X87State),
    pub(super) fcmov: unsafe extern "C" fn(*mut X87State, u32, u32),
    pub(super) fbld: unsafe extern "C" fn(*mut X87State, u64, u64),
    pub(super) fbstp: unsafe extern "C" fn(*mut X87State) -> X87Pair,
    pub(super) fldenv: unsafe extern "C" fn(*mut X87State, u64, u32),
    pub(super) fnstenv: unsafe extern "C" fn(*mut X87State, u64, u32),
    pub(super) frstor: unsafe extern "C" fn(*mut X87State, u64, u32),
    pub(super) fnsave: unsafe extern "C" fn(*mut X87State, u64, u32),
}

unsafe extern "C" fn h_fld_f32(st: *mut X87State, bits: u32) {
    (*st).push_f32_bits(bits);
}

unsafe extern "C" fn h_fld_f64(st: *mut X87State, bits: u64) {
    (*st).push_f64_bits(bits);
}

unsafe extern "C" fn h_fld_f80(st: *mut X87State, lo: u64, hi: u64) {
    let mut b = [0u8; 10];
    b[0..8].copy_from_slice(&lo.to_le_bytes());
    b[8..10].copy_from_slice(&(hi as u16).to_le_bytes());
    (*st).push_bytes(b, 0);
}

unsafe extern "C" fn h_fld_st(st: *mut X87State, idx: u32) {
    (*st).fld_st(idx as u8);
}

unsafe extern "C" fn h_fild_i64(st: *mut X87State, v: i64) {
    (*st).push_fp80(Fp80::from_i64(v), 0);
}

unsafe extern "C" fn h_fld_const(st: *mut X87State, id: u32) {
    (*st).push_const(id as u8);
}

unsafe extern "C" fn h_fst_f32(st: *mut X87State, pop: u32) -> u32 {
    (*st).store_f32_bits(pop != 0)
}

unsafe extern "C" fn h_fst_f64(st: *mut X87State, pop: u32) -> u64 {
    (*st).store_f64_bits(pop != 0)
}

unsafe extern "C" fn h_fstp_f80(st: *mut X87State) -> X87Pair {
    let b = (*st).pop_f80_bytes();
    X87Pair {
        lo: u64::from_le_bytes(b[0..8].try_into().unwrap()),
        hi: u16::from_le_bytes([b[8], b[9]]) as u64,
    }
}

unsafe extern "C" fn h_fst_st(st: *mut X87State, idx: u32, pop: u32) {
    if pop != 0 {
        (*st).pop_to_st(idx as u8);
    } else {
        (*st).fst_st(idx as u8);
    }
}

unsafe extern "C" fn h_fist(st: *mut X87State, bits: u32, flags: u32) -> u64 {
    (*st).store_int_bits(bits, (flags & 1) != 0, (flags & 2) != 0)
}

unsafe extern "C" fn h_binop_st(st: *mut X87State, op_pop: u32, dst: u32, src: u32) {
    (*st).binop_st((op_pop & 7) as u8, dst as u8, src as u8, (op_pop & 8) != 0);
}

unsafe extern "C" fn h_binop_f32(st: *mut X87State, op: u32, bits: u32) {
    let mut extra = 0u16;
    let rhs = Fp80::from_f32_bits(bits, &mut extra);
    (*st).binop_val(op as u8, rhs, extra);
}

unsafe extern "C" fn h_binop_f64(st: *mut X87State, op: u32, bits: u64) {
    let mut extra = 0u16;
    let rhs = Fp80::from_f64_bits(bits, &mut extra);
    (*st).binop_val(op as u8, rhs, extra);
}

unsafe extern "C" fn h_binop_i64(st: *mut X87State, op: u32, v: i64) {
    (*st).binop_val(op as u8, Fp80::from_i64(v), 0);
}

unsafe extern "C" fn h_unary(st: *mut X87State, op: u32) {
    (*st).exec_unary(op as u8);
}

/// `flags`: bit 0 = signaling, bit 1 = write_sw, bits 3:2 = pops (0, 1, or 2).
/// Returns comparison code `0=GT, 1=LT, 2=EQ, 3=UNORD`.
unsafe extern "C" fn h_compare_st(st: *mut X87State, idx: u32, flags: u32) -> u64 {
    (*st).compare_st(idx as u8, (flags & 1) != 0, (flags & 2) != 0, ((flags >> 2) & 3) as u8) as u64
}

unsafe extern "C" fn h_compare_f32(st: *mut X87State, bits: u32, flags: u32) {
    let mut extra = 0u16;
    let rhs = Fp80::from_f32_bits(bits, &mut extra);
    (*st).compare_val(rhs, extra, (flags & 1) != 0, ((flags >> 2) & 3) as u8);
}

unsafe extern "C" fn h_compare_f64(st: *mut X87State, bits: u64, flags: u32) {
    let mut extra = 0u16;
    let rhs = Fp80::from_f64_bits(bits, &mut extra);
    (*st).compare_val(rhs, extra, (flags & 1) != 0, ((flags >> 2) & 3) as u8);
}

unsafe extern "C" fn h_compare_i64(st: *mut X87State, v: i64, flags: u32) {
    (*st).compare_val(Fp80::from_i64(v), 0, (flags & 1) != 0, ((flags >> 2) & 3) as u8);
}

unsafe extern "C" fn h_fxam(st: *mut X87State) {
    (*st).fxam();
}

unsafe extern "C" fn h_fcmov(st: *mut X87State, idx: u32, cond: u32) {
    (*st).fcmov(idx as u8, cond != 0);
}

unsafe extern "C" fn h_fbld(st: *mut X87State, lo: u64, hi: u64) {
    let mut b = [0u8; 10];
    b[0..8].copy_from_slice(&lo.to_le_bytes());
    b[8..10].copy_from_slice(&(hi as u16).to_le_bytes());
    (*st).push_bcd(b);
}

unsafe extern "C" fn h_fbstp(st: *mut X87State) -> X87Pair {
    let b = (*st).pop_bcd();
    X87Pair {
        lo: u64::from_le_bytes(b[0..8].try_into().unwrap()),
        hi: u16::from_le_bytes([b[8], b[9]]) as u64,
    }
}

unsafe extern "C" fn h_fldenv(st: *mut X87State, addr: u64, size16: u32) {
    let len = if size16 != 0 { 14 } else { 28 };
    let mut buf = [0u8; 28];
    if super::mem::tso() {
        core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
    }
    core::ptr::copy_nonoverlapping(addr as *const u8, buf.as_mut_ptr(), len);
    (*st).decode_env(&buf[..len], size16 != 0);
}

unsafe extern "C" fn h_fnstenv(st: *mut X87State, addr: u64, size16: u32) {
    let len = if size16 != 0 { 14 } else { 28 };
    let mut buf = [0u8; 28];
    (*st).stenv(&mut buf[..len], size16 != 0);
    if super::mem::tso() {
        core::sync::atomic::fence(core::sync::atomic::Ordering::Release);
    }
    core::ptr::copy_nonoverlapping(buf.as_ptr(), addr as *mut u8, len);
}

unsafe extern "C" fn h_frstor(st: *mut X87State, addr: u64, size16: u32) {
    let len = if size16 != 0 { 94 } else { 108 };
    let mut buf = [0u8; 108];
    if super::mem::tso() {
        core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
    }
    core::ptr::copy_nonoverlapping(addr as *const u8, buf.as_mut_ptr(), len);
    (*st).frstor(&buf[..len], size16 != 0);
}

unsafe extern "C" fn h_fnsave(st: *mut X87State, addr: u64, size16: u32) {
    let len = if size16 != 0 { 94 } else { 108 };
    let mut buf = [0u8; 108];
    (*st).fsave(&mut buf[..len], size16 != 0);
    if super::mem::tso() {
        core::sync::atomic::fence(core::sync::atomic::Ordering::Release);
    }
    core::ptr::copy_nonoverlapping(buf.as_ptr(), addr as *mut u8, len);
}

static X87_HELPERS: X87Helpers = X87Helpers {
    fld_f32: h_fld_f32,
    fld_f64: h_fld_f64,
    fld_f80: h_fld_f80,
    fld_st: h_fld_st,
    fild_i64: h_fild_i64,
    fld_const: h_fld_const,
    fst_f32: h_fst_f32,
    fst_f64: h_fst_f64,
    fstp_f80: h_fstp_f80,
    fst_st: h_fst_st,
    fist: h_fist,
    binop_st: h_binop_st,
    binop_f32: h_binop_f32,
    binop_f64: h_binop_f64,
    binop_i64: h_binop_i64,
    unary: h_unary,
    compare_st: h_compare_st,
    compare_f32: h_compare_f32,
    compare_f64: h_compare_f64,
    compare_i64: h_compare_i64,
    fxam: h_fxam,
    fcmov: h_fcmov,
    fbld: h_fbld,
    fbstp: h_fbstp,
    fldenv: h_fldenv,
    fnstenv: h_fnstenv,
    frstor: h_frstor,
    fnsave: h_fnsave,
};

#[inline(always)]
pub(super) fn x87_helpers_ptr() -> usize {
    (&raw const X87_HELPERS) as usize
}
