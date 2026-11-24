//! Core ARM64 instruction encoder (`Asm`) for the x86-64 -> arm64 block translator.

use super::{XS, ZR};

/// Encoding of a logical immediate (`N:immr:imms` in bits 22..10), if `v` is one.
pub(super) fn enc_logic_imm(v: u64, x: bool) -> Option<u32> {
    let v = if x { v } else { (v & 0xFFFF_FFFF) | (v << 32) };
    if v == 0 || v == u64::MAX {
        return None;
    }
    let mut size = 64u32;
    while size > 2 {
        let half = size / 2;
        let mask = (1u64 << half) - 1;
        if (v & mask) != ((v >> half) & mask) {
            break;
        }
        size = half;
    }
    let mask = if size == 64 { u64::MAX } else { (1u64 << size) - 1 };
    let elt = v & mask;
    let ones = elt.count_ones();
    let target = (1u64 << ones) - 1;
    for r in 0..size {
        let rot = if r == 0 { elt } else { ((elt >> r) | (elt << (size - r))) & mask };
        if rot == target {
            let immr = (size - r) % size;
            let imms = (((size as i64).wrapping_neg() << 1) as u64 as u32 | (ones - 1)) & 0x3F;
            let n = (size == 64) as u32;
            return Some((n << 22) | (immr << 16) | (imms << 10));
        }
    }
    None
}

pub(super) struct Asm {
    pub(super) base: *mut u32,
    /// Address word 0 executes at (`b_to`/`bl_to` targets are absolute): `base` for the
    /// JIT, the final AOT region address for AOT builds (which emit into a scratch buffer).
    pub(super) vbase: usize,
    pub(super) pos: usize,
    pub(super) cap: usize,
    pub(super) overflow: bool,
}

impl Asm {
    pub(super) fn new(base: *mut u32, vbase: usize, cap: usize) -> Self {
        Asm { base, vbase, pos: 0, cap, overflow: false }
    }
    #[inline]
    pub(super) fn emit(&mut self, w: u32) {
        if self.pos < self.cap {
            unsafe { *self.base.add(self.pos) = w };
            self.pos += 1;
        } else {
            self.overflow = true;
        }
    }
    pub(super) fn addr_of(&self, pos: usize) -> usize {
        self.vbase + pos * 4
    }
    pub(super) fn read(&self, pos: usize) -> u32 {
        unsafe { *self.base.add(pos) }
    }
    /// Point the branch at `at` to `target` (both word positions in this buffer).
    pub(super) fn patch(&mut self, at: usize, target: usize) {
        if at >= self.cap || self.overflow {
            return;
        }
        let off = target as i64 - at as i64;
        let ins = self.read(at);
        let new = patch_branch(ins, off);
        unsafe { *self.base.add(at) = new };
    }
    pub(super) fn bind(&mut self, at: usize) {
        let here = self.pos;
        self.patch(at, here);
    }

    // --- data processing ---
    pub(super) fn addsub_rr(&mut self, x: bool, sub: bool, s: bool, d: u32, n: u32, m: u32, lsl: u32) {
        self.emit(0x0B00_0000 | sf(x) | (sub as u32) << 30 | (s as u32) << 29 | m << 16 | lsl << 10 | n << 5 | d);
    }
    pub(super) fn addsub_ri(&mut self, x: bool, sub: bool, s: bool, d: u32, n: u32, imm12: u32, sh12: bool) {
        self.emit(0x1100_0000 | sf(x) | (sub as u32) << 30 | (s as u32) << 29 | (sh12 as u32) << 22 | (imm12 & 0xFFF) << 10 | n << 5 | d);
    }
    /// `opc`: 0 and, 1 orr, 2 eor, 3 ands; `shift`: 0 lsl, 1 lsr, 2 asr, 3 ror.
    pub(super) fn logic_rr(&mut self, x: bool, opc: u32, d: u32, n: u32, m: u32, shift: u32, amt: u32) {
        self.emit(0x0A00_0000 | sf(x) | opc << 29 | shift << 22 | m << 16 | amt << 10 | n << 5 | d);
    }
    pub(super) fn logic_ri(&mut self, x: bool, opc: u32, d: u32, n: u32, enc: u32) {
        self.emit(0x1200_0000 | sf(x) | opc << 29 | enc | n << 5 | d);
    }
    pub(super) fn mov_rr(&mut self, x: bool, d: u32, m: u32) {
        self.logic_rr(x, 1, d, ZR, m, 0, 0);
    }
    pub(super) fn movz(&mut self, x: bool, d: u32, imm16: u32, hw: u32) {
        self.emit(0x5280_0000 | sf(x) | hw << 21 | (imm16 & 0xFFFF) << 5 | d);
    }
    pub(super) fn movk(&mut self, x: bool, d: u32, imm16: u32, hw: u32) {
        self.emit(0x7280_0000 | sf(x) | hw << 21 | (imm16 & 0xFFFF) << 5 | d);
    }
    pub(super) fn movn(&mut self, x: bool, d: u32, imm16: u32, hw: u32) {
        self.emit(0x1280_0000 | sf(x) | hw << 21 | (imm16 & 0xFFFF) << 5 | d);
    }
    pub(super) fn mov_imm(&mut self, x: bool, d: u32, v: u64) {
        let nch = if x { 4 } else { 2 };
        let v = if x { v } else { v & 0xFFFF_FFFF };
        let ch = |i: u32| ((v >> (16 * i)) & 0xFFFF) as u32;
        let zeros = (0..nch).filter(|&i| ch(i) == 0).count() as u32;
        let ones = (0..nch).filter(|&i| ch(i) == 0xFFFF).count() as u32;
        if nch - zeros > 1 && nch - ones > 1 {
            if let Some(e) = enc_logic_imm(v, x) {
                self.logic_ri(x, 1, d, ZR, e);
                return;
            }
        }
        if ones > zeros {
            let mut first = true;
            for i in 0..nch {
                if ch(i) != 0xFFFF {
                    if first {
                        self.movn(x, d, !ch(i) & 0xFFFF, i);
                        first = false;
                    } else {
                        self.movk(x, d, ch(i), i);
                    }
                }
            }
            if first {
                self.movn(x, d, 0, 0);
            }
        } else {
            let mut first = true;
            for i in 0..nch {
                if ch(i) != 0 {
                    if first {
                        self.movz(x, d, ch(i), i);
                        first = false;
                    } else {
                        self.movk(x, d, ch(i), i);
                    }
                }
            }
            if first {
                self.movz(x, d, 0, 0);
            }
        }
    }
    pub(super) fn ubfm(&mut self, x: bool, d: u32, n: u32, immr: u32, imms: u32) {
        self.emit(0x5300_0000 | sf(x) | (x as u32) << 22 | immr << 16 | imms << 10 | n << 5 | d);
    }
    pub(super) fn sbfm(&mut self, x: bool, d: u32, n: u32, immr: u32, imms: u32) {
        self.emit(0x1300_0000 | sf(x) | (x as u32) << 22 | immr << 16 | imms << 10 | n << 5 | d);
    }
    pub(super) fn bfm(&mut self, x: bool, d: u32, n: u32, immr: u32, imms: u32) {
        self.emit(0x3300_0000 | sf(x) | (x as u32) << 22 | immr << 16 | imms << 10 | n << 5 | d);
    }
    /// Copy the low `w` bits of `n` into `d` (other bits of `d` kept).
    pub(super) fn bfxil0(&mut self, d: u32, n: u32, w: u32) {
        self.bfm(true, d, n, 0, w - 1);
    }
    /// `ubfx d, n, #lsb, #width` (64-bit).
    pub(super) fn ubfx(&mut self, d: u32, n: u32, lsb: u32, width: u32) {
        self.ubfm(true, d, n, lsb, lsb + width - 1);
    }
    pub(super) fn lsl_i(&mut self, x: bool, d: u32, n: u32, s: u32) {
        let bits = if x { 64 } else { 32 };
        self.ubfm(x, d, n, (bits - s) % bits, bits - 1 - s);
    }
    pub(super) fn lsr_i(&mut self, x: bool, d: u32, n: u32, s: u32) {
        let bits = if x { 64 } else { 32 };
        self.ubfm(x, d, n, s, bits - 1);
    }
    pub(super) fn asr_i(&mut self, x: bool, d: u32, n: u32, s: u32) {
        let bits = if x { 64 } else { 32 };
        self.sbfm(x, d, n, s, bits - 1);
    }
    pub(super) fn ror_i(&mut self, x: bool, d: u32, n: u32, s: u32) {
        self.emit(0x1380_0000 | sf(x) | (x as u32) << 22 | n << 16 | s << 10 | n << 5 | d);
    }
    /// `op2`: 8 lslv, 9 lsrv, 10 asrv.
    pub(super) fn shiftv(&mut self, x: bool, op2: u32, d: u32, n: u32, m: u32) {
        self.emit(0x1AC0_0000 | sf(x) | m << 16 | op2 << 10 | n << 5 | d);
    }
    pub(super) fn csel(&mut self, x: bool, d: u32, n: u32, m: u32, cond: u32) {
        self.emit(0x1A80_0000 | sf(x) | m << 16 | cond << 12 | n << 5 | d);
    }
    pub(super) fn csinc(&mut self, x: bool, d: u32, n: u32, m: u32, cond: u32) {
        self.emit(0x1A80_0400 | sf(x) | m << 16 | cond << 12 | n << 5 | d);
    }
    pub(super) fn cset(&mut self, d: u32, cond: u32) {
        // csinc wd, wzr, wzr, !cond
        self.emit(0x1A80_0400 | ZR << 16 | (cond ^ 1) << 12 | ZR << 5 | d);
    }
    pub(super) fn clz(&mut self, x: bool, d: u32, n: u32) {
        self.emit(0x5AC0_1000 | sf(x) | n << 5 | d);
    }
    pub(super) fn rbit(&mut self, x: bool, d: u32, n: u32) {
        self.emit(0x5AC0_0000 | sf(x) | n << 5 | d);
    }
    pub(super) fn rev(&mut self, x: bool, d: u32, n: u32) {
        self.emit(if x { 0xDAC0_0C00 } else { 0x5AC0_0800 } | n << 5 | d);
    }
    pub(super) fn mul(&mut self, x: bool, d: u32, n: u32, m: u32) {
        self.emit(0x1B00_0000 | sf(x) | m << 16 | ZR << 10 | n << 5 | d);
    }
    pub(super) fn smulh(&mut self, d: u32, n: u32, m: u32) {
        self.emit(0x9B40_7C00 | m << 16 | n << 5 | d);
    }
    pub(super) fn umulh(&mut self, d: u32, n: u32, m: u32) {
        self.emit(0x9BC0_7C00 | m << 16 | n << 5 | d);
    }
    pub(super) fn udiv(&mut self, x: bool, d: u32, n: u32, m: u32) {
        self.emit(0x1AC0_0800 | sf(x) | m << 16 | n << 5 | d);
    }
    pub(super) fn sdiv(&mut self, x: bool, d: u32, n: u32, m: u32) {
        self.emit(0x1AC0_0C00 | sf(x) | m << 16 | n << 5 | d);
    }
    pub(super) fn msub(&mut self, x: bool, d: u32, n: u32, m: u32, a: u32) {
        self.emit(0x1B00_8000 | sf(x) | m << 16 | a << 10 | n << 5 | d);
    }
    // --- scalar FP / conversions ---
    pub(super) fn fmov_gf(&mut self, is_f64: bool, vd: u32, n: u32) {
        self.emit(0x1E27_0000 | sf(is_f64) | (is_f64 as u32) << 22 | n << 5 | vd);
    }
    pub(super) fn fmov_fg(&mut self, is_f64: bool, d: u32, vn: u32) {
        self.emit(0x1E26_0000 | sf(is_f64) | (is_f64 as u32) << 22 | vn << 5 | d);
    }
    pub(super) fn scvtf_gp(&mut self, x: bool, is_f64: bool, vd: u32, n: u32) {
        self.emit(0x1E22_0000 | sf(x) | (is_f64 as u32) << 22 | n << 5 | vd);
    }
    pub(super) fn fcmp(&mut self, is_f64: bool, vn: u32, vm: u32) {
        self.emit(0x1E20_2000 | (is_f64 as u32) << 22 | vm << 16 | vn << 5);
    }
    pub(super) fn fcvt_d_s(&mut self, vd: u32, vn: u32) {
        self.emit(0x1E22_C000 | vn << 5 | vd);
    }
    pub(super) fn fcvt_s_d(&mut self, vd: u32, vn: u32) {
        self.emit(0x1E62_4000 | vn << 5 | vd);
    }
    pub(super) fn frintn_d(&mut self, vd: u32, vn: u32) {
        self.emit(0x1E64_4000 | vn << 5 | vd);
    }
    pub(super) fn frintz_d(&mut self, vd: u32, vn: u32) {
        self.emit(0x1E65_C000 | vn << 5 | vd);
    }
    pub(super) fn fcvtzs_d(&mut self, x: bool, d: u32, vn: u32) {
        self.emit(0x1E78_0000 | sf(x) | vn << 5 | d);
    }
    /// Flag-free `d = n + v`.
    pub(super) fn add_imm(&mut self, x: bool, d: u32, n: u32, v: i64) {
        let (sub, a) = if v < 0 { (true, v.unsigned_abs()) } else { (false, v as u64) };
        if a < 4096 {
            self.addsub_ri(x, sub, false, d, n, a as u32, false);
        } else if a & 0xFFF == 0 && a < (1 << 24) {
            self.addsub_ri(x, sub, false, d, n, (a >> 12) as u32, true);
        } else {
            self.mov_imm(x, XS, v as u64);
            self.addsub_rr(x, false, false, d, n, XS, 0);
        }
    }

    // --- loads / stores ---
    /// Unsigned-offset load/store of `bits` (8/16/32/64/128); `ld` selects load.
    pub(super) fn ldst_ui(&mut self, ld: bool, bits: u32, t: u32, n: u32, off: u32) {
        let (base, scale) = match bits {
            8 => (0x3900_0000, 0),
            16 => (0x7900_0000, 1),
            32 => (0xB900_0000, 2),
            64 => (0xF900_0000, 3),
            _ => (0x3D80_0000, 4),
        };
        let opc = if ld { 0x0040_0000 } else { 0 };
        assert!(off % (1 << scale) == 0 && (off >> scale) < 4096, "ldst offset {off}");
        self.emit(base | opc | (off >> scale) << 10 | n << 5 | t);
    }
    pub(super) fn ldr(&mut self, bits: u32, t: u32, n: u32, off: u32) {
        self.ldst_ui(true, bits, t, n, off);
    }
    pub(super) fn str(&mut self, bits: u32, t: u32, n: u32, off: u32) {
        self.ldst_ui(false, bits, t, n, off);
    }
    pub(super) fn stp_x(&mut self, t1: u32, t2: u32, n: u32, off: i32) {
        assert!(off % 8 == 0 && (-512..=504).contains(&off), "stp offset {off}");
        self.emit(0xA900_0000 | (((off / 8) as u32) & 0x7F) << 15 | t2 << 10 | n << 5 | t1);
    }
    pub(super) fn ldp_x(&mut self, t1: u32, t2: u32, n: u32, off: i32) {
        assert!(off % 8 == 0 && (-512..=504).contains(&off), "ldp offset {off}");
        self.emit(0xA940_0000 | (((off / 8) as u32) & 0x7F) << 15 | t2 << 10 | n << 5 | t1);
    }
    pub(super) fn stp_q(&mut self, t1: u32, t2: u32, n: u32, off: i32) {
        assert!(off % 16 == 0 && (-1024..=1008).contains(&off), "stp q offset {off}");
        self.emit(0xAD00_0000 | (((off / 16) as u32) & 0x7F) << 15 | t2 << 10 | n << 5 | t1);
    }
    pub(super) fn ldp_q(&mut self, t1: u32, t2: u32, n: u32, off: i32) {
        assert!(off % 16 == 0 && (-1024..=1008).contains(&off), "ldp q offset {off}");
        self.emit(0xAD40_0000 | (((off / 16) as u32) & 0x7F) << 15 | t2 << 10 | n << 5 | t1);
    }
    pub(super) fn ldur_q(&mut self, t: u32, n: u32, off: i32) {
        assert!((-256..=255).contains(&off), "ldur q offset {off}");
        self.emit(0x3CC0_0000 | ((off as u32) & 0x1FF) << 12 | n << 5 | t);
    }
    pub(super) fn stur_q(&mut self, t: u32, n: u32, off: i32) {
        assert!((-256..=255).contains(&off), "stur q offset {off}");
        self.emit(0x3C80_0000 | ((off as u32) & 0x1FF) << 12 | n << 5 | t);
    }
    pub(super) fn ldapr(&mut self, bits: u32, t: u32, n: u32) {
        let base = match bits {
            8 => 0x38BF_C000,
            16 => 0x78BF_C000,
            32 => 0xB8BF_C000,
            _ => 0xF8BF_C000,
        };
        self.emit(base | n << 5 | t);
    }
    pub(super) fn stlr(&mut self, bits: u32, t: u32, n: u32) {
        let base = match bits {
            8 => 0x089F_FC00,
            16 => 0x489F_FC00,
            32 => 0x889F_FC00,
            _ => 0xC89F_FC00,
        };
        self.emit(base | n << 5 | t);
    }
    /// `ldur{b,h,}`/`stur{b,h,}` (unscaled signed 9-bit offset, plain non-TSO).
    pub(super) fn ldur_stur(&mut self, ld: bool, bits: u32, t: u32, n: u32, off: i32) {
        assert!((-256..=255).contains(&off), "ldur/stur offset {off}");
        let size = match bits {
            8 => 0u32,
            16 => 1,
            32 => 2,
            _ => 3,
        };
        let opc = if ld { 0x0040_0000 } else { 0 };
        self.emit(0x3800_0000 | size << 30 | opc | ((off as u32) & 0x1FF) << 12 | n << 5 | t);
    }
    /// `ldapur{b,h,}`/`stlur{b,h,}` (FEAT_LRCPC2): RCpc load-acquire / store-release with a
    /// signed 9-bit unscaled offset.
    pub(super) fn ldapur_stlur(&mut self, ld: bool, bits: u32, t: u32, n: u32, off: i32) {
        assert!((-256..=255).contains(&off), "ldapur/stlur offset {off}");
        let size = match bits {
            8 => 0u32,
            16 => 1,
            32 => 2,
            _ => 3,
        };
        let opc = if ld { 0x0040_0000 } else { 0 };
        self.emit(0x1900_0000 | size << 30 | opc | ((off as u32) & 0x1FF) << 12 | n << 5 | t);
    }
    /// `str xt, [xn, #off]!` (pre-index).
    pub(super) fn str_pre(&mut self, t: u32, n: u32, off: i32) {
        assert!((-256..=255).contains(&off), "str pre offset {off}");
        self.emit(0xF800_0C00 | ((off as u32) & 0x1FF) << 12 | n << 5 | t);
    }
    /// `ldr xt, [xn], #off` (post-index).
    pub(super) fn ldr_post(&mut self, t: u32, n: u32, off: i32) {
        assert!((-256..=255).contains(&off), "ldr post offset {off}");
        self.emit(0xF840_0400 | ((off as u32) & 0x1FF) << 12 | n << 5 | t);
    }
    /// `stp xt1, xt2, [xn, #off]!` (pre-index).
    pub(super) fn stp_pre(&mut self, t1: u32, t2: u32, n: u32, off: i32) {
        assert!(off % 8 == 0 && (-512..=504).contains(&off), "stp pre offset {off}");
        self.emit(0xA980_0000 | (((off / 8) as u32) & 0x7F) << 15 | t2 << 10 | n << 5 | t1);
    }
    /// `ldp xt1, xt2, [xn], #off` (post-index; t1 != t2, neither == n).
    pub(super) fn ldp_post(&mut self, t1: u32, t2: u32, n: u32, off: i32) {
        assert!(off % 8 == 0 && (-512..=504).contains(&off) && t1 != t2 && t1 != n && t2 != n);
        self.emit(0xA8C0_0000 | (((off / 8) as u32) & 0x7F) << 15 | t2 << 10 | n << 5 | t1);
    }
    pub(super) fn dmb_ish(&mut self) {
        self.emit(0xD503_3BBF);
    }
    pub(super) fn dmb_ishld(&mut self) {
        self.emit(0xD503_39BF);
    }
    pub(super) fn dmb_ishst(&mut self) {
        self.emit(0xD503_3ABF);
    }
    pub(super) fn dsb_ishld(&mut self) {
        self.emit(0xD503_399F);
    }
    pub(super) fn isb(&mut self) {
        self.emit(0xD503_3FDF);
    }
    /// `mrs xt, cntvct_el0` (reads the virtual timer count directly without ISB).
    pub(super) fn mrs_cntvct(&mut self, t: u32) {
        self.emit(0xD53B_E040 | t);
    }
    /// `mrs xt, cntvctss_el0` (FEAT_ECV self-synchronizing virtual counter).
    pub(super) fn mrs_cntvctss(&mut self, t: u32) {
        self.emit(0xD53B_E0C0 | t);
    }
    /// `mrs xt, cntfrq_el0` (reads the execution host's counter frequency in Hz).
    pub(super) fn mrs_cntfrq(&mut self, t: u32) {
        self.emit(0xD53B_E000 | t);
    }
    /// `mrs xt, fpcr` (reads ARM64 floating-point control register).
    pub(super) fn mrs_fpcr(&mut self, t: u32) {
        self.emit(0xD53B_4400 | t);
    }
    /// `msr fpcr, xt` (writes ARM64 floating-point control register).
    pub(super) fn msr_fpcr(&mut self, t: u32) {
        self.emit(0xD51B_4400 | t);
    }

    // --- branches (offsets patched later unless given) ---
    pub(super) fn b(&mut self) -> usize {
        self.emit(0x1400_0000);
        self.pos - 1
    }
    pub(super) fn b_to(&mut self, target_addr: usize) {
        let here = self.addr_of(self.pos);
        let off = (target_addr as i64 - here as i64) / 4;
        self.emit(0x1400_0000 | (off as u32 & 0x03FF_FFFF));
    }
    pub(super) fn bcond(&mut self, cond: u32) -> usize {
        self.emit(0x5400_0000 | cond);
        self.pos - 1
    }
    pub(super) fn cbz(&mut self, x: bool, t: u32) -> usize {
        self.emit(0x3400_0000 | sf(x) | t);
        self.pos - 1
    }
    pub(super) fn cbnz(&mut self, x: bool, t: u32) -> usize {
        self.emit(0x3500_0000 | sf(x) | t);
        self.pos - 1
    }
    pub(super) fn tbnz(&mut self, t: u32, bit: u32) -> usize {
        self.emit(0x3700_0000 | (bit >> 5) << 31 | (bit & 31) << 19 | t);
        self.pos - 1
    }
    pub(super) fn br(&mut self, n: u32) {
        self.emit(0xD61F_0000 | n << 5);
    }
    pub(super) fn ret(&mut self) {
        self.emit(0xD65F_03C0);
    }
    /// `ret xn` (return-predicted indirect branch).
    pub(super) fn ret_r(&mut self, n: u32) {
        self.emit(0xD65F_0000 | n << 5);
    }
    /// `bl` placeholder (patched later like [`Self::b`]; the link bit is preserved).
    pub(super) fn bl(&mut self) -> usize {
        self.emit(0x9400_0000);
        self.pos - 1
    }
    pub(super) fn bl_to(&mut self, target_addr: usize) {
        let here = self.addr_of(self.pos);
        let off = (target_addr as i64 - here as i64) / 4;
        self.emit(0x9400_0000 | (off as u32 & 0x03FF_FFFF));
    }
    /// `adr xd, .+off` placeholder; set the offset with [`Self::patch_adr`].
    pub(super) fn adr(&mut self, d: u32) -> usize {
        self.emit(0x1000_0000 | d);
        self.pos - 1
    }
    /// Point the `adr` at word `at` to word `target` of this buffer.
    pub(super) fn patch_adr(&mut self, at: usize, target: usize) {
        if at >= self.cap || self.overflow {
            return;
        }
        let off = ((target as i64 - at as i64) * 4) as u32;
        let ins = self.read(at) & 0x9F00_001F;
        let new = ins | (off & 3) << 29 | ((off >> 2) & 0x7_FFFF) << 5;
        unsafe { *self.base.add(at) = new };
    }
    pub(super) fn blr(&mut self, n: u32) {
        self.emit(0xD63F_0000 | n << 5);
    }
}

#[inline(always)]
pub(super) fn sf(x: bool) -> u32 {
    (x as u32) << 31
}

/// Re-target branch instruction `ins` by `off` words.
pub(super) fn patch_branch(ins: u32, off: i64) -> u32 {
    let o = off as u32;
    if ins & 0x7C00_0000 == 0x1400_0000 {
        (ins & 0xFC00_0000) | (o & 0x03FF_FFFF)
    } else if ins & 0xFF00_0010 == 0x5400_0000 || ins & 0x7E00_0000 == 0x3400_0000 {
        (ins & 0xFF00_001F) | ((o & 0x7FFFF) << 5)
    } else if ins & 0x7E00_0000 == 0x3600_0000 {
        (ins & 0xFFF8_001F) | ((o & 0x3FFF) << 5)
    } else {
        ins
    }
}
