//! Register-file (`DF_REGFILE`) native execution tier and reference oracle for non-Win64-ABI
//! guest functions (such as V8's embedded CSA/Torque/Ignition/JSEntry/CEntry builtins).
//!
//! Unlike `extern "win64"` C++ lifts (`Lifted`), a [`RegfileFn`] receives `(*mut Cpu, blk)`
//! directly (`raw` typed as `*mut u8` at module boundaries, `blk = 0` at function entry and
//! `blk > 0` at call-return or exception-landing-pad continuations) and mutates all 16 GPRs
//! (including real guest `rsp`), all 16 XMMs, architectural `RFLAGS` (`cf`, `pf`, `zf`, `sf`,
//! `df`, `of`), `last_bits[0..16]`, and guest memory in place without `CallCtx` or `LiftedFrame`.

use crate::exe::{rd, wr};
use iced_x86::{Decoder, DecoderOptions, FlowControl, Mnemonic, OpKind};

/// `RegfileFn` completed with `ret` / `ret imm16`; `va` is the popped return address.
pub const RF_RET: u64 = 0;
/// `RegfileFn` exited via tail `jmp` / external `jcc` / fall-through; `va` is the target IP.
pub const RF_JUMP: u64 = 1;
/// `RegfileFn` stopped before executing instruction `va` (`int3`, or guest memory outside
/// `[0x10000, 2^47)`), leaving `Cpu` in the exact pre-instruction architectural state so the
/// interpreter/SEH path can step `va`.
pub const RF_FAULT: u64 = 2;
/// `RegfileFn` executed `call va` (having already pushed the return address onto `[rsp]`);
/// `va` is the callee IP.
pub const RF_CALL: u64 = 3;

/// Exit descriptor returned by a [`RegfileFn`] in `(x0, x1)`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct RegfileExit {
    pub kind: u64,
    pub va: u64,
}

impl RegfileExit {
    #[inline(always)]
    pub const fn ret(va: u64) -> Self {
        Self { kind: RF_RET, va }
    }

    #[inline(always)]
    pub const fn jump(va: u64) -> Self {
        Self { kind: RF_JUMP, va }
    }

    #[inline(always)]
    pub const fn fault(va: u64) -> Self {
        Self { kind: RF_FAULT, va }
    }

    #[inline(always)]
    pub const fn call(va: u64) -> Self {
        Self { kind: RF_CALL, va }
    }
}

/// Compiled register-file body (`raw` is `*mut super::Cpu`, `blk` is starting basic-block index).
pub type RegfileFn = unsafe extern "C" fn(*mut u8, u32) -> RegfileExit;

std::thread_local! {
    /// One-shot skip IP after [`RF_FAULT`]: prevents `emu_run_region` from re-entering a
    /// [`RegfileFn`] at `va` when the very first instruction of that builtin faulted/trapped.
    static SKIP_VA: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
    /// `RegfileFn` invocations executed on this thread (diagnostics / unit tests).
    static REGFILE_RUNS: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
    /// Unit-test override disabling `DF_REGFILE` dispatch on this thread.
    #[cfg(test)]
    static REGFILE_OFF: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
}

/// Count of threads with a non-zero `SKIP_VA` armed (`0` on the common path so `take_skip`
/// avoids a TLS `_tlv_get_addr` lookup on every `emu_run_region` iteration).
static SKIP_ARMED: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);

#[inline(always)]
pub(super) fn has_skip_armed() -> bool {
    SKIP_ARMED.load(core::sync::atomic::Ordering::Relaxed) != 0
}

#[inline(always)]
pub(super) fn arm_skip(va: u64) {
    SKIP_VA.with(|c| {
        let prev = c.replace(va);
        if prev == 0 && va != 0 {
            SKIP_ARMED.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
        } else if prev != 0 && va == 0 {
            SKIP_ARMED.fetch_sub(1, core::sync::atomic::Ordering::Relaxed);
        }
    });
}

#[inline(always)]
pub(super) fn take_skip(va: u64) -> bool {
    if SKIP_ARMED.load(core::sync::atomic::Ordering::Relaxed) == 0 {
        return false;
    }
    SKIP_VA.with(|c| {
        let s = c.get();
        if s == 0 {
            return false;
        }
        c.set(0);
        SKIP_ARMED.fetch_sub(1, core::sync::atomic::Ordering::Relaxed);
        s == va
    })
}

/// Number of `RegfileFn` bodies executed on this thread so far.
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn regfile_runs() -> u64 {
    REGFILE_RUNS.with(|c| c.get())
}

#[cfg(test)]
pub(crate) fn set_regfile_off_for_test(off: bool) {
    REGFILE_OFF.with(|c| c.set(off));
}

#[cfg(test)]
#[inline(always)]
pub(super) fn test_off() -> bool {
    REGFILE_OFF.with(|c| c.get())
}

/// Execute `(f, blk)` against `cpu`, chaining direct/indirect tail jumps (`RF_JUMP`),
/// builtin-to-builtin calls (`RF_CALL`), and returns (`RF_RET` to a registered continuation)
/// in a flat loop without growing the host stack.
#[inline]
pub(super) unsafe fn run_chain(cpu: &mut super::Cpu, mut f: RegfileFn, mut blk: u32) -> RegfileExit {
    let mut hops = 0u32;
    loop {
        REGFILE_RUNS.with(|c| c.set(c.get().wrapping_add(1)));
        let ex = f(cpu as *mut super::Cpu as *mut u8, blk);
        if hops < 4096
            && matches!(ex.kind, RF_JUMP | RF_CALL | RF_RET)
            && ex.va != super::RET_SENTINEL
            && ex.va != 0
        {
            if let Some((nf, nblk)) = super::lookup_routed_regfile(ex.va) {
                f = nf;
                blk = nblk;
                hops += 1;
                continue;
            }
        }
        return ex;
    }
}

/// Inline architectural state wrapper used by generated `DF_REGFILE` Rust bodies (`rNN.rs`).
pub struct Rf<'a>(&'a mut super::Cpu);

#[inline(always)]
pub unsafe fn ctx<'a>(raw: *mut u8) -> Rf<'a> {
    Rf(&mut *(raw as *mut super::Cpu))
}

impl<'a> Rf<'a> {
    #[inline(always)]
    pub fn r64(&self, i: usize) -> u64 {
        self.0.gpr[i]
    }

    #[inline(always)]
    pub fn r32(&self, i: usize) -> u64 {
        self.0.gpr[i] as u32 as u64
    }

    #[inline(always)]
    pub fn r16(&self, i: usize) -> u64 {
        self.0.gpr[i] as u16 as u64
    }

    #[inline(always)]
    pub fn r8(&self, i: usize) -> u64 {
        self.0.gpr[i] as u8 as u64
    }

    #[inline(always)]
    pub fn rh8(&self, i: usize) -> u64 {
        (self.0.gpr[i] >> 8) & 0xFF
    }

    #[inline(always)]
    pub fn w64(&mut self, i: usize, v: u64) {
        self.0.gpr[i] = v;
        self.0.last_bits[i] = 64;
    }

    #[inline(always)]
    pub fn w32(&mut self, i: usize, v: u64) {
        self.0.gpr[i] = v as u32 as u64;
        self.0.last_bits[i] = 64;
    }

    #[inline(always)]
    pub fn w16(&mut self, i: usize, v: u64) {
        self.0.gpr[i] = (self.0.gpr[i] & !0xFFFF) | (v & 0xFFFF);
        self.0.last_bits[i] = 16;
    }

    #[inline(always)]
    pub fn w8(&mut self, i: usize, v: u64) {
        self.0.gpr[i] = (self.0.gpr[i] & !0xFF) | (v & 0xFF);
        self.0.last_bits[i] = 8;
    }

    #[inline(always)]
    pub fn wh8(&mut self, i: usize, v: u64) {
        self.0.gpr[i] = (self.0.gpr[i] & !0xFF00) | ((v & 0xFF) << 8);
        self.0.last_bits[i] = 64;
    }

    #[inline(always)]
    pub unsafe fn ld8(&self, addr: u64) -> Option<u64> {
        if !super::regfile_ext::is_valid_guest_range(addr, 1) {
            return None;
        }
        let p = addr as usize;
        Some(if super::mem::tso() { super::mem::load_acq(p, 8) } else { rd::<u8>(p) as u64 })
    }

    #[inline(always)]
    pub unsafe fn ld16(&self, addr: u64) -> Option<u64> {
        if !super::regfile_ext::is_valid_guest_range(addr, 2) {
            return None;
        }
        let p = addr as usize;
        Some(if super::mem::tso() { super::mem::load_acq(p, 16) } else { rd::<u16>(p) as u64 })
    }

    #[inline(always)]
    pub unsafe fn ld32(&self, addr: u64) -> Option<u64> {
        if !super::regfile_ext::is_valid_guest_range(addr, 4) {
            return None;
        }
        let p = addr as usize;
        Some(if super::mem::tso() { super::mem::load_acq(p, 32) } else { rd::<u32>(p) as u64 })
    }

    #[inline(always)]
    pub unsafe fn ld64(&self, addr: u64) -> Option<u64> {
        if !super::regfile_ext::is_valid_guest_range(addr, 8) {
            return None;
        }
        let p = addr as usize;
        Some(if super::mem::tso() { super::mem::load_acq(p, 64) } else { rd::<u64>(p) })
    }

    #[inline(always)]
    pub unsafe fn ld128(&self, addr: u64) -> Option<u128> {
        if !super::regfile_ext::is_valid_guest_range(addr, 16) {
            return None;
        }
        let p = addr as usize;
        Some(if super::mem::tso_vec() { super::mem::load128_acq(p) } else { rd::<u128>(p) })
    }

    #[inline(always)]
    pub unsafe fn st8(&mut self, addr: u64, v: u64) -> bool {
        if !super::regfile_ext::is_valid_guest_range(addr, 1) {
            return false;
        }
        let p = addr as usize;
        if super::mem::tso() {
            super::mem::store_rel(p, 8, v);
        } else {
            wr::<u8>(p, v as u8);
        }
        true
    }

    #[inline(always)]
    pub unsafe fn st16(&mut self, addr: u64, v: u64) -> bool {
        if !super::regfile_ext::is_valid_guest_range(addr, 2) {
            return false;
        }
        let p = addr as usize;
        if super::mem::tso() {
            super::mem::store_rel(p, 16, v);
        } else {
            wr::<u16>(p, v as u16);
        }
        true
    }

    #[inline(always)]
    pub unsafe fn st32(&mut self, addr: u64, v: u64) -> bool {
        if !super::regfile_ext::is_valid_guest_range(addr, 4) {
            return false;
        }
        let p = addr as usize;
        if super::mem::tso() {
            super::mem::store_rel(p, 32, v);
        } else {
            wr::<u32>(p, v as u32);
        }
        true
    }

    #[inline(always)]
    pub unsafe fn st64(&mut self, addr: u64, v: u64) -> bool {
        if !super::regfile_ext::is_valid_guest_range(addr, 8) {
            return false;
        }
        let p = addr as usize;
        if super::mem::tso() {
            super::mem::store_rel(p, 64, v);
        } else {
            wr::<u64>(p, v);
        }
        true
    }

    #[inline(always)]
    pub unsafe fn st128(&mut self, addr: u64, v: u128) -> bool {
        if !super::regfile_ext::is_valid_guest_range(addr, 16) {
            return false;
        }
        let p = addr as usize;
        if super::mem::tso_vec() {
            super::mem::store128_rel(p, v);
        } else {
            wr::<u128>(p, v);
        }
        true
    }

    #[inline(always)]
    pub unsafe fn push64(&mut self, v: u64) -> bool {
        let sp = self.0.gpr[4].wrapping_sub(8);
        if !self.st64(sp, v) {
            return false;
        }
        self.0.gpr[4] = sp;
        true
    }

    #[inline(always)]
    pub unsafe fn pop64_reg(&mut self, dst: usize) -> bool {
        let sp = self.0.gpr[4];
        let Some(v) = self.ld64(sp) else { return false };
        self.0.gpr[4] = sp.wrapping_add(8);
        self.w64(dst, v);
        true
    }

    #[inline(always)]
    pub fn set_rsp(&mut self, sp: u64) {
        self.0.gpr[4] = sp;
    }

    #[inline(always)]
    pub unsafe fn leave(&mut self) -> bool {
        let bp = self.0.gpr[5];
        let Some(v) = self.ld64(bp) else { return false };
        self.0.gpr[4] = bp.wrapping_add(8);
        self.0.gpr[5] = v;
        self.0.last_bits[4] = 64;
        self.0.last_bits[5] = 64;
        true
    }

    #[inline(always)]
    pub unsafe fn call(&mut self, callee: u64, ret_va: u64, ip: u64) -> RegfileExit {
        if !self.push64(ret_va) {
            return RegfileExit::fault(ip);
        }
        RegfileExit::call(callee)
    }

    #[inline(always)]
    pub unsafe fn ret(&mut self, imm16: u64, ip: u64) -> RegfileExit {
        let sp = self.0.gpr[4];
        let Some(ra) = self.ld64(sp) else { return RegfileExit::fault(ip) };
        self.0.gpr[4] = sp.wrapping_add(8 + imm16);
        RegfileExit::ret(ra)
    }

    #[inline(always)]
    pub fn cdqe(&mut self) {
        self.0.gpr[0] = self.0.gpr[0] as i32 as i64 as u64;
    }

    #[inline(always)]
    pub fn cdq(&mut self) {
        self.0.gpr[2] = if (self.0.gpr[0] as i32) < 0 { 0xFFFF_FFFF } else { 0 };
    }

    #[inline(always)]
    pub fn cqo(&mut self) {
        self.0.gpr[2] = if (self.0.gpr[0] as i64) < 0 { u64::MAX } else { 0 };
    }

    #[inline(always)]
    pub fn idiv(&mut self, d: u64, bits: u32) {
        if bits == 64 && d != 0 {
            let n = (((self.0.gpr[2] as u128) << 64) | (self.0.gpr[0] as u128)) as i128;
            let d64 = d as i64 as i128;
            self.0.gpr[0] = n.wrapping_div(d64) as i64 as u64;
            self.0.gpr[2] = n.wrapping_rem(d64) as i64 as u64;
        } else if bits == 32 && (d as u32) != 0 {
            let n = (((self.0.gpr[2] as u32 as u64) << 32) | (self.0.gpr[0] as u32 as u64)) as i64;
            let d32 = d as u32 as i32 as i64;
            self.0.gpr[0] = n.wrapping_div(d32) as i32 as u32 as u64;
            self.0.gpr[2] = n.wrapping_rem(d32) as i32 as u32 as u64;
        }
    }

    #[inline(always)]
    pub fn bsr(&mut self, dst: usize, src: u64, bits: u32) {
        let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
        let v = src & mask;
        if v == 0 {
            self.0.zf = true;
        } else {
            self.0.zf = false;
            let idx = 63u64.wrapping_sub(v.leading_zeros() as u64);
            match bits {
                16 => self.w16(dst, idx),
                32 => self.w32(dst, idx),
                _ => self.w64(dst, idx),
            }
        }
    }

    #[inline(always)]
    pub fn add(&mut self, a: u64, b: u64, bits: u32) -> u64 {
        self.0.update_flags_add(a, b, bits)
    }

    #[inline(always)]
    pub fn sub(&mut self, a: u64, b: u64, bits: u32) -> u64 {
        self.0.update_flags_sub(a, b, bits)
    }

    #[inline(always)]
    pub fn cmp(&mut self, a: u64, b: u64, bits: u32) {
        let _ = self.0.update_flags_sub(a, b, bits);
    }

    #[inline(always)]
    pub fn logic(&mut self, res: u64, bits: u32) {
        self.0.update_flags_logic(res, bits);
    }

    #[inline(always)]
    pub fn inc(&mut self, a: u64, bits: u32) -> u64 {
        let saved = self.0.cf;
        let r = self.0.update_flags_add(a, 1, bits);
        self.0.cf = saved;
        r
    }

    #[inline(always)]
    pub fn dec(&mut self, a: u64, bits: u32) -> u64 {
        let saved = self.0.cf;
        let r = self.0.update_flags_sub(a, 1, bits);
        self.0.cf = saved;
        r
    }

    #[inline(always)]
    pub fn neg(&mut self, a: u64, bits: u32) -> u64 {
        let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
        let ua = a & mask;
        let r = self.0.update_flags_sub(0, ua, bits);
        self.0.cf = ua != 0;
        r
    }

    #[inline(always)]
    pub fn shl(&mut self, a: u64, cnt: u64, bits: u32) -> Option<u64> {
        let m = if bits == 64 { 63 } else { 31 };
        let c = (cnt as u32) & m;
        (c > 0).then(|| self.0.shift(super::fast::SHIFT_SHL, a, c, bits as u8))
    }

    #[inline(always)]
    pub fn shr(&mut self, a: u64, cnt: u64, bits: u32) -> Option<u64> {
        let m = if bits == 64 { 63 } else { 31 };
        let c = (cnt as u32) & m;
        (c > 0).then(|| self.0.shift(super::fast::SHIFT_SHR, a, c, bits as u8))
    }

    #[inline(always)]
    pub fn sar(&mut self, a: u64, cnt: u64, bits: u32) -> Option<u64> {
        let m = if bits == 64 { 63 } else { 31 };
        let c = (cnt as u32) & m;
        (c > 0).then(|| self.0.shift(super::fast::SHIFT_SAR, a, c, bits as u8))
    }

    #[inline(always)]
    pub fn imul2(&mut self, a: u64, b: u64, bits: u32) -> u64 {
        let sext = |v: u64| -> i128 {
            match bits {
                8 => v as u8 as i8 as i128,
                16 => v as u16 as i16 as i128,
                32 => v as u32 as i32 as i128,
                _ => v as i64 as i128,
            }
        };
        let prod = sext(a).wrapping_mul(sext(b));
        let fits = match bits {
            16 => prod == (prod as i16 as i128),
            32 => prod == (prod as i32 as i128),
            _ => prod == (prod as i64 as i128),
        };
        self.0.cf = !fits;
        self.0.of = !fits;
        prod as u64
    }

    #[inline(always)]
    pub fn imul1(&mut self, src: u64, bits: u32) {
        if bits == 64 {
            let prod = (self.0.gpr[0] as i64 as i128).wrapping_mul(src as i64 as i128);
            self.0.gpr[0] = prod as i64 as u64;
            self.0.gpr[2] = (prod >> 64) as i64 as u64;
            self.0.cf = prod != (prod as i64 as i128);
            self.0.of = self.0.cf;
        } else if bits == 32 {
            let prod = (self.0.gpr[0] as u32 as i32 as i64).wrapping_mul(src as u32 as i32 as i64);
            self.0.gpr[0] = prod as i32 as u32 as u64;
            self.0.gpr[2] = ((prod >> 32) as i32 as u32) as u64;
            self.0.cf = prod != (prod as i32 as i64);
            self.0.of = self.0.cf;
        }
    }

    #[inline(always)]
    pub fn cc_o(&self) -> bool {
        self.0.of
    }
    #[inline(always)]
    pub fn cc_no(&self) -> bool {
        !self.0.of
    }
    #[inline(always)]
    pub fn cc_b(&self) -> bool {
        self.0.cf
    }
    #[inline(always)]
    pub fn cc_ae(&self) -> bool {
        !self.0.cf
    }
    #[inline(always)]
    pub fn cc_e(&self) -> bool {
        self.0.zf
    }
    #[inline(always)]
    pub fn cc_ne(&self) -> bool {
        !self.0.zf
    }
    #[inline(always)]
    pub fn cc_be(&self) -> bool {
        self.0.cf || self.0.zf
    }
    #[inline(always)]
    pub fn cc_a(&self) -> bool {
        !self.0.cf && !self.0.zf
    }
    #[inline(always)]
    pub fn cc_s(&self) -> bool {
        self.0.sf
    }
    #[inline(always)]
    pub fn cc_ns(&self) -> bool {
        !self.0.sf
    }
    #[inline(always)]
    pub fn cc_p(&self) -> bool {
        self.0.pf()
    }
    #[inline(always)]
    pub fn cc_np(&self) -> bool {
        !self.0.pf()
    }
    #[inline(always)]
    pub fn cc_l(&self) -> bool {
        self.0.sf != self.0.of
    }
    #[inline(always)]
    pub fn cc_ge(&self) -> bool {
        self.0.sf == self.0.of
    }
    #[inline(always)]
    pub fn cc_le(&self) -> bool {
        self.0.zf || (self.0.sf != self.0.of)
    }
    #[inline(always)]
    pub fn cc_g(&self) -> bool {
        !self.0.zf && (self.0.sf == self.0.of)
    }

    #[inline(always)]
    pub fn xmm_u64(&self, i: usize) -> u64 {
        self.0.xmm[i] as u64
    }

    #[inline(always)]
    pub fn xmm_u32(&self, i: usize) -> u32 {
        self.0.xmm[i] as u32
    }

    #[inline(always)]
    pub fn movsd_rr(&mut self, d: usize, s: usize) {
        self.0.xmm[d] = (self.0.xmm[d] & !0xFFFF_FFFF_FFFF_FFFF) | (self.0.xmm[s] & 0xFFFF_FFFF_FFFF_FFFF);
    }

    #[inline(always)]
    pub unsafe fn movsd_rm(&mut self, d: usize, addr: u64) -> bool {
        let Some(v) = self.ld64(addr) else { return false };
        self.0.xmm[d] = v as u128;
        true
    }

    #[inline(always)]
    pub unsafe fn movsd_mr(&mut self, addr: u64, s: usize) -> bool {
        self.st64(addr, self.0.xmm[s] as u64)
    }

    #[inline(always)]
    pub fn movss_rr(&mut self, d: usize, s: usize) {
        self.0.xmm[d] = (self.0.xmm[d] & !0xFFFF_FFFF) | (self.0.xmm[s] & 0xFFFF_FFFF);
    }

    #[inline(always)]
    pub unsafe fn movss_rm(&mut self, d: usize, addr: u64) -> bool {
        let Some(v) = self.ld32(addr) else { return false };
        self.0.xmm[d] = v as u128;
        true
    }

    #[inline(always)]
    pub unsafe fn movss_mr(&mut self, addr: u64, s: usize) -> bool {
        self.st32(addr, self.0.xmm[s] as u32 as u64)
    }

    #[inline(always)]
    pub fn mov128_rr(&mut self, d: usize, s: usize) {
        self.0.xmm[d] = self.0.xmm[s];
    }

    #[inline(always)]
    pub unsafe fn mov128_rm(&mut self, d: usize, addr: u64) -> bool {
        let Some(v) = self.ld128(addr) else { return false };
        self.0.xmm[d] = v;
        true
    }

    #[inline(always)]
    pub unsafe fn mov128_mr(&mut self, addr: u64, s: usize) -> bool {
        self.st128(addr, self.0.xmm[s])
    }

    #[inline(always)]
    pub fn movq_xx(&mut self, d: usize, s: usize) {
        self.0.xmm[d] = self.0.xmm[s] & 0xFFFF_FFFF_FFFF_FFFF;
    }

    #[inline(always)]
    pub fn movq_xr(&mut self, d: usize, s_gpr: usize) {
        self.0.xmm[d] = self.r64(s_gpr) as u128;
    }

    #[inline(always)]
    pub fn movq_rx(&mut self, d_gpr: usize, s: usize) {
        self.w64(d_gpr, self.0.xmm[s] as u64);
    }

    #[inline(always)]
    pub fn movd_xr(&mut self, d: usize, s_gpr: usize) {
        self.0.xmm[d] = (self.r32(s_gpr) as u32) as u128;
    }

    #[inline(always)]
    pub fn movd_rx(&mut self, d_gpr: usize, s: usize) {
        self.w32(d_gpr, self.0.xmm[s] as u32 as u64);
    }

    #[inline(always)]
    pub fn xorpd_rr(&mut self, d: usize, s: usize) {
        self.0.xmm[d] ^= self.0.xmm[s];
    }

    #[inline(always)]
    pub unsafe fn xorpd_rm(&mut self, d: usize, addr: u64) -> bool {
        let Some(v) = self.ld128(addr) else { return false };
        self.0.xmm[d] ^= v;
        true
    }

    #[inline(always)]
    pub fn and128_rr(&mut self, d: usize, s: usize) {
        self.0.xmm[d] &= self.0.xmm[s];
    }

    #[inline(always)]
    pub unsafe fn and128_rm(&mut self, d: usize, addr: u64) -> bool {
        let Some(v) = self.ld128(addr) else { return false };
        self.0.xmm[d] &= v;
        true
    }

    #[inline(always)]
    fn pcmpeqd_u128(x: u128, y: u128) -> u128 {
        #[cfg(target_arch = "aarch64")]
        unsafe {
            use core::arch::aarch64::*;
            let vx: uint32x4_t = core::mem::transmute(x);
            let vy: uint32x4_t = core::mem::transmute(y);
            core::mem::transmute(vceqq_u32(vx, vy))
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            let mut out = 0u128;
            for i in 0..4 {
                let vx = ((x >> (i * 32)) & 0xFFFF_FFFF) as u32;
                let vy = ((y >> (i * 32)) & 0xFFFF_FFFF) as u32;
                if vx == vy {
                    out |= 0xFFFF_FFFFu128 << (i * 32);
                }
            }
            out
        }
    }

    #[inline(always)]
    pub fn pcmpeqd_rr(&mut self, d: usize, s: usize) {
        self.0.xmm[d] = Self::pcmpeqd_u128(self.0.xmm[d], self.0.xmm[s]);
    }

    #[inline(always)]
    pub unsafe fn pcmpeqd_rm(&mut self, d: usize, addr: u64) -> bool {
        let Some(v) = self.ld128(addr) else { return false };
        self.0.xmm[d] = Self::pcmpeqd_u128(self.0.xmm[d], v);
        true
    }

    #[inline(always)]
    pub fn psllq(&mut self, d: usize, cnt: u64) {
        let v = self.0.xmm[d];
        let mut out = 0u128;
        if cnt < 64 {
            let sh = cnt as u32;
            let l0 = (v as u64) << sh;
            let l1 = ((v >> 64) as u64) << sh;
            out = (l0 as u128) | ((l1 as u128) << 64);
        }
        self.0.xmm[d] = out;
    }

    #[inline(always)]
    pub fn psrlq(&mut self, d: usize, cnt: u64) {
        let v = self.0.xmm[d];
        let mut out = 0u128;
        if cnt < 64 {
            let sh = cnt as u32;
            let l0 = (v as u64) >> sh;
            let l1 = ((v >> 64) as u64) >> sh;
            out = (l0 as u128) | ((l1 as u128) << 64);
        }
        self.0.xmm[d] = out;
    }

    #[inline(always)]
    pub fn movmskpd(&self, s: usize) -> u64 {
        let lo = self.0.xmm[s];
        let b0 = ((lo >> 63) & 1) as u64;
        let b1 = ((lo >> 127) & 1) as u64;
        b0 | (b1 << 1)
    }

    #[inline(always)]
    fn unpckldq_u128(va: u128, vb: u128) -> u128 {
        #[cfg(target_arch = "aarch64")]
        unsafe {
            use core::arch::aarch64::*;
            let a: uint32x4_t = core::mem::transmute(va);
            let b: uint32x4_t = core::mem::transmute(vb);
            core::mem::transmute(vzip1q_u32(a, b))
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            let a0 = (va as u32) as u128;
            let a1 = ((va >> 32) as u32) as u128;
            let b0 = (vb as u32) as u128;
            let b1 = ((vb >> 32) as u32) as u128;
            a0 | (b0 << 32) | (a1 << 64) | (b1 << 96)
        }
    }

    #[inline(always)]
    pub fn punpckldq_rr(&mut self, d: usize, s: usize) {
        self.0.xmm[d] = Self::unpckldq_u128(self.0.xmm[d], self.0.xmm[s]);
    }

    #[inline(always)]
    pub unsafe fn punpckldq_rm(&mut self, d: usize, addr: u64) -> bool {
        let Some(v) = self.ld128(addr) else { return false };
        self.0.xmm[d] = Self::unpckldq_u128(self.0.xmm[d], v);
        true
    }

    #[inline(always)]
    pub fn arith_sd(&mut self, op: u8, d: usize, b_bits: u64) {
        let a = self.0.xmm[d];
        let r = super::sse_arith64(op, f64::from_bits(a as u64), f64::from_bits(b_bits));
        self.0.xmm[d] = (a & !0xFFFF_FFFF_FFFF_FFFF) | (r.to_bits() as u128);
    }

    #[inline(always)]
    pub fn sqrtsd(&mut self, d: usize, b_bits: u64) {
        let a = self.0.xmm[d];
        let r = super::sse_sqrt64(f64::from_bits(b_bits));
        self.0.xmm[d] = (a & !0xFFFF_FFFF_FFFF_FFFF) | (r.to_bits() as u128);
    }

    #[inline(always)]
    pub fn ucomisd(&mut self, a_bits: u64, b_bits: u64) {
        let fa = f64::from_bits(a_bits);
        let fb = f64::from_bits(b_bits);
        let unord = !(fa <= fb) & !(fa > fb);
        self.0.sf = false;
        self.0.of = false;
        self.0.zf = !(fa < fb) & !(fa > fb);
        self.0.cf = !(fa >= fb);
        self.0.pfv = (!unord) as u8;
    }

    #[inline(always)]
    pub fn ucomiss(&mut self, a_bits: u32, b_bits: u32) {
        let fa = f32::from_bits(a_bits);
        let fb = f32::from_bits(b_bits);
        let unord = !(fa <= fb) & !(fa > fb);
        self.0.sf = false;
        self.0.of = false;
        self.0.zf = !(fa < fb) & !(fa > fb);
        self.0.cf = !(fa >= fb);
        self.0.pfv = (!unord) as u8;
    }

    #[inline(always)]
    pub fn cvtsi2sd(&mut self, d: usize, iv: i64) {
        self.0.xmm[d] = (self.0.xmm[d] & !0xFFFF_FFFF_FFFF_FFFF) | ((iv as f64).to_bits() as u128);
    }

    #[inline(always)]
    pub fn cvtsi2ss(&mut self, d: usize, iv: i64) {
        self.0.xmm[d] = (self.0.xmm[d] & !0xFFFF_FFFF) | ((iv as f32).to_bits() as u128);
    }

    #[inline(always)]
    pub fn cvttsd2si32(&self, b_bits: u64) -> u64 {
        super::sse_cvt_i32(f64::from_bits(b_bits), true) as u64
    }

    #[inline(always)]
    pub fn cvttsd2si64(&self, b_bits: u64) -> u64 {
        super::sse_cvt_i64(f64::from_bits(b_bits), true)
    }

    #[inline(always)]
    pub fn cvttss2si32(&self, b_bits: u32) -> u64 {
        super::sse_cvt_i32(f32::from_bits(b_bits) as f64, true) as u64
    }

    #[inline(always)]
    pub fn cvttss2si64(&self, b_bits: u32) -> u64 {
        super::sse_cvt_i64(f32::from_bits(b_bits) as f64, true)
    }

    #[inline(always)]
    pub fn cvtsd2ss(&mut self, d: usize, b_bits: u64) {
        let f = f64::from_bits(b_bits) as f32;
        self.0.xmm[d] = (self.0.xmm[d] & !0xFFFF_FFFF) | (f.to_bits() as u128);
    }

    #[inline(always)]
    pub fn cvtss2sd(&mut self, d: usize, b_bits: u32) {
        let f = f32::from_bits(b_bits) as f64;
        self.0.xmm[d] = (self.0.xmm[d] & !0xFFFF_FFFF_FFFF_FFFF) | (f.to_bits() as u128);
    }

    #[inline(always)]
    pub unsafe fn fld_m64(&mut self, addr: u64) -> bool {
        super::regfile_ext::fld_m64(self.0, addr)
    }

    #[inline(always)]
    pub fn fprem(&mut self) {
        super::regfile_ext::fprem(self.0);
    }

    #[inline(always)]
    pub fn fnstsw_ax(&mut self) {
        super::regfile_ext::fnstsw_ax(self.0);
    }

    #[inline(always)]
    pub fn fstp_st(&mut self, idx: u8) {
        super::regfile_ext::fstp_st(self.0, idx);
    }

    #[inline(always)]
    pub unsafe fn fstp_m64(&mut self, addr: u64) -> bool {
        super::regfile_ext::fstp_m64(self.0, addr)
    }

    #[inline(always)]
    pub unsafe fn popfq(&mut self) -> bool {
        super::regfile_ext::popfq(self.0)
    }

    #[inline(always)]
    pub unsafe fn lock_cmpxchg(&mut self, addr: u64, desired: u64, bits: u32) -> bool {
        match bits {
            8 => super::regfile_ext::lock_cmpxchg_8(self.0, addr, desired),
            16 => super::regfile_ext::lock_cmpxchg_16(self.0, addr, desired),
            32 => super::regfile_ext::lock_cmpxchg_32(self.0, addr, desired),
            _ => super::regfile_ext::lock_cmpxchg_64(self.0, addr, desired),
        }
    }

    #[inline(always)]
    pub unsafe fn xchg_mem(&mut self, addr: u64, src_val: u64, bits: u32) -> Option<u64> {
        match bits {
            8 => super::regfile_ext::xchg_mem_8(self.0, addr, src_val),
            16 => super::regfile_ext::xchg_mem_16(self.0, addr, src_val),
            32 => super::regfile_ext::xchg_mem_32(self.0, addr, src_val),
            _ => super::regfile_ext::xchg_mem_64(self.0, addr, src_val),
        }
    }
}

/// Authoritative single-instruction `iced-x86` reference oracle over `[lo, hi)` starting at
/// `start_va` for `dll_regfile` difftests and unit tests: steps `cpu` inside `[lo, hi)` under
/// precise-fault semantics without calling out into downstream runtime targets (stopping at
/// `call` after pushing `next_ip` with `RegfileExit::call(tgt)`).
pub unsafe fn run_oracle(cpu: &mut super::Cpu, start_va: u64, lo: u64, hi: u64) -> RegfileExit {
    #[cfg(test)]
    let prev_prec = super::PRECISE_LOCAL.with(|p| p.replace(true));
    #[cfg(not(test))]
    let _prec_guard = {
        super::enter_precise_oracle();
        struct OraclePreciseGuard;
        impl Drop for OraclePreciseGuard {
            fn drop(&mut self) {
                super::leave_precise_oracle();
            }
        }
        OraclePreciseGuard
    };
    super::FAULT_PENDING.with(|p| p.set(None));
    super::regfile_ext::x87_reset();
    cpu.rip = start_va;
    let mut steps = 0usize;
    let res = loop {
        let cur_ip = cpu.rip;
        if cur_ip < lo || cur_ip >= hi {
            break RegfileExit::jump(cur_ip);
        }
        let mapped = super::to_mapped_addr(cur_ip);
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
        let pre_x87 = super::regfile_ext::x87_snapshot();
        let next_ip = ins.next_ip();
        cpu.rip = next_ip;
        match ins.flow_control() {
            FlowControl::Call | FlowControl::IndirectCall => {
                let tgt = match ins.op_kind(0) {
                    OpKind::NearBranch64 => ins.near_branch64(),
                    OpKind::NearBranch32 => ins.near_branch32() as u64,
                    OpKind::NearBranch16 => ins.near_branch16() as u64,
                    _ => cpu.read_op(&ins, 0, 64),
                };
                if super::FAULT_PENDING.with(|p| p.take().is_some()) {
                    *cpu = pre;
                    super::regfile_ext::x87_restore(pre_x87);
                    cpu.rip = cur_ip;
                    break RegfileExit::fault(cur_ip);
                }
                let new_sp = cpu.gpr[4].wrapping_sub(8);
                cpu.write_mem(new_sp, 64, next_ip);
                if super::FAULT_PENDING.with(|p| p.take().is_some()) {
                    *cpu = pre;
                    super::regfile_ext::x87_restore(pre_x87);
                    cpu.rip = cur_ip;
                    break RegfileExit::fault(cur_ip);
                }
                cpu.gpr[4] = new_sp;
                cpu.rip = tgt;
                break RegfileExit::call(tgt);
            }
            FlowControl::Return => {
                let imm = if ins.op_count() > 0 { cpu.read_op(&ins, 0, 16) } else { 0 };
                let ret_va = cpu.read_mem(cpu.gpr[4], 64);
                if super::FAULT_PENDING.with(|p| p.take().is_some()) {
                    *cpu = pre;
                    super::regfile_ext::x87_restore(pre_x87);
                    cpu.rip = cur_ip;
                    break RegfileExit::fault(cur_ip);
                }
                cpu.gpr[4] = cpu.gpr[4].wrapping_add(8 + imm);
                break RegfileExit::ret(ret_va);
            }
            FlowControl::UnconditionalBranch => {
                let tgt = match ins.op_kind(0) {
                    OpKind::NearBranch64 => ins.near_branch64(),
                    OpKind::NearBranch32 => ins.near_branch32() as u64,
                    OpKind::NearBranch16 => ins.near_branch16() as u64,
                    _ => cpu.read_op(&ins, 0, 64),
                };
                if super::FAULT_PENDING.with(|p| p.take().is_some()) {
                    *cpu = pre;
                    super::regfile_ext::x87_restore(pre_x87);
                    cpu.rip = cur_ip;
                    break RegfileExit::fault(cur_ip);
                }
                cpu.rip = tgt;
                if tgt < lo || tgt >= hi {
                    break RegfileExit::jump(tgt);
                }
            }
            FlowControl::IndirectBranch => {
                let tgt = cpu.read_op(&ins, 0, 64);
                if super::FAULT_PENDING.with(|p| p.take().is_some()) {
                    *cpu = pre;
                    super::regfile_ext::x87_restore(pre_x87);
                    cpu.rip = cur_ip;
                    break RegfileExit::fault(cur_ip);
                }
                cpu.rip = tgt;
                break RegfileExit::jump(tgt);
            }
            FlowControl::ConditionalBranch => {
                let cc = ins.condition_code();
                if cpu.eval_cc(cc) {
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
                let ok = cpu.exec_generic_ins(&ins);
                if !ok || super::FAULT_PENDING.with(|p| p.take().is_some()) {
                    *cpu = pre;
                    super::regfile_ext::x87_restore(pre_x87);
                    cpu.rip = cur_ip;
                    break RegfileExit::fault(cur_ip);
                }
            }
            _ => {
                *cpu = pre;
                super::regfile_ext::x87_restore(pre_x87);
                cpu.rip = cur_ip;
                break RegfileExit::fault(cur_ip);
            }
        }
        steps += 1;
        if steps >= usize::MAX / 2 {
            break RegfileExit::fault(cpu.rip);
        }
    };
    #[cfg(test)]
    super::PRECISE_LOCAL.with(|p| p.set(prev_prec));
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emu::Cpu;

    #[test]
    fn regfile_ucomiss_ucomisd_branchless_flags() {
        let mut cpu = Cpu::new_zeroed();
        let mut rf = unsafe { ctx(&mut cpu as *mut Cpu as *mut u8) };

        let cases_f64: &[(f64, f64, bool, bool, bool)] = &[
            (5.0, 3.0, false, false, false),
            (3.0, 5.0, false, false, true),
            (4.0, 4.0, true, false, false),
            (-0.0, 0.0, true, false, false),
            (0.0, -0.0, true, false, false),
            (f64::INFINITY, f64::INFINITY, true, false, false),
            (f64::NEG_INFINITY, f64::INFINITY, false, false, true),
            (f64::NAN, 1.0, true, true, true),
            (1.0, f64::NAN, true, true, true),
            (f64::NAN, f64::NAN, true, true, true),
            (f64::from_bits(0x7FF0_0000_0000_0001), 0.0, true, true, true), // sNaN
        ];
        for &(a, b, zf, pf, cf) in cases_f64 {
            rf.0.sf = true;
            rf.0.of = true;
            rf.ucomisd(a.to_bits(), b.to_bits());
            assert_eq!(rf.0.zf, zf, "ucomisd({a}, {b}) ZF");
            assert_eq!(rf.cc_p(), pf, "ucomisd({a}, {b}) PF (pfv={})", rf.0.pfv);
            assert_eq!(rf.0.cf, cf, "ucomisd({a}, {b}) CF");
            assert!(!rf.0.sf && !rf.0.of, "ucomisd({a}, {b}) clears SF/OF");
        }

        let cases_f32: &[(f32, f32, bool, bool, bool)] = &[
            (2.0, -1.0, false, false, false),
            (-1.0, 2.0, false, false, true),
            (-0.0, 0.0, true, false, false),
            (f32::NAN, 0.0, true, true, true),
            (0.0, f32::from_bits(0x7F80_0001), true, true, true), // sNaN
        ];
        for &(a, b, zf, pf, cf) in cases_f32 {
            rf.0.sf = true;
            rf.0.of = true;
            rf.ucomiss(a.to_bits(), b.to_bits());
            assert_eq!(rf.0.zf, zf, "ucomiss({a}, {b}) ZF");
            assert_eq!(rf.cc_p(), pf, "ucomiss({a}, {b}) PF");
            assert_eq!(rf.0.cf, cf, "ucomiss({a}, {b}) CF");
            assert!(!rf.0.sf && !rf.0.of);
        }
    }

    #[test]
    fn regfile_pcmpeqd_and_punpckldq_neon() {
        let mut cpu = Cpu::new_zeroed();
        cpu.xmm[0] = 0x0000_0004_0000_0003_FFFF_FFFF_0000_0001;
        cpu.xmm[1] = 0x0000_0009_0000_0003_FFFF_FFFF_0000_0002;
        let mut rf = unsafe { ctx(&mut cpu as *mut Cpu as *mut u8) };
        rf.pcmpeqd_rr(0, 1);
        assert_eq!(rf.0.xmm[0], 0x0000_0000_FFFF_FFFF_FFFF_FFFF_0000_0000);

        rf.0.xmm[2] = 0x4444_4444_3333_3333_2222_2222_1111_1111;
        rf.0.xmm[3] = 0xDDDD_DDDD_CCCC_CCCC_BBBB_BBBB_AAAA_AAAA;
        rf.punpckldq_rr(2, 3);
        assert_eq!(rf.0.xmm[2], 0xBBBB_BBBB_2222_2222_AAAA_AAAA_1111_1111);
    }
}
