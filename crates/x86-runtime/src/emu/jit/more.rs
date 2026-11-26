//! Native arm64/NEON code generation for `K_VAES_RR..=K_VVNNI_RM` (`115..=130`),
//! completing 100% native arm64/NEON coverage across all `FastOp` kinds.

use super::emit::{enc_logic_imm, sf, Asm};
use super::lazy::PF_NONE;
use super::translate::{stack_tso, Chk, Cond, FlagState, Src, SysCold, SysColdKind, Tr};
use super::*;
use crate::emu::fast::{self, FastOp};
use crate::emu::mem;

impl Asm {
    pub(super) fn tbz(&mut self, t: u32, bit: u32) -> usize {
        self.emit(0x3600_0000 | (bit >> 5) << 31 | (bit & 31) << 19 | t);
        self.pos - 1
    }
    pub(super) fn v_aese(&mut self, vd: u32, vn: u32) {
        self.emit(0x4E28_4800 | vn << 5 | vd);
    }
    pub(super) fn v_aesd(&mut self, vd: u32, vn: u32) {
        self.emit(0x4E28_5800 | vn << 5 | vd);
    }
    pub(super) fn v_aesmc(&mut self, vd: u32, vn: u32) {
        self.emit(0x4E28_6800 | vn << 5 | vd);
    }
    pub(super) fn v_aesimc(&mut self, vd: u32, vn: u32) {
        self.emit(0x4E28_7800 | vn << 5 | vd);
    }
    pub(super) fn v_rev64_4s(&mut self, vd: u32, vn: u32) {
        self.emit(0x4EA0_0800 | vn << 5 | vd);
    }
    pub(super) fn v_sha256h(&mut self, vd: u32, vn: u32, vm: u32) {
        self.emit(0x5E00_4000 | vm << 16 | vn << 5 | vd);
    }
    pub(super) fn v_sha256h2(&mut self, vd: u32, vn: u32, vm: u32) {
        self.emit(0x5E00_5000 | vm << 16 | vn << 5 | vd);
    }
    pub(super) fn v_sha256su0(&mut self, vd: u32, vn: u32) {
        self.emit(0x5E28_2800 | vn << 5 | vd);
    }
    pub(super) fn v_sha256su1(&mut self, vd: u32, vn: u32, vm: u32) {
        self.emit(0x5E00_6000 | vm << 16 | vn << 5 | vd);
    }
    pub(super) fn v_sha1c(&mut self, vd: u32, vn: u32, vm: u32) {
        self.emit(0x5E00_0000 | vm << 16 | vn << 5 | vd);
    }
    pub(super) fn v_sha1p(&mut self, vd: u32, vn: u32, vm: u32) {
        self.emit(0x5E00_1000 | vm << 16 | vn << 5 | vd);
    }
    pub(super) fn v_sha1m(&mut self, vd: u32, vn: u32, vm: u32) {
        self.emit(0x5E00_2000 | vm << 16 | vn << 5 | vd);
    }
    pub(super) fn v_sha1su1(&mut self, vd: u32, vn: u32) {
        self.emit(0x5E28_1800 | vn << 5 | vd);
    }
    pub(super) fn v_fcvtl_h(&mut self, q: bool, vd: u32, vn: u32) {
        self.emit(0x0E21_7800 | (q as u32) << 30 | vn << 5 | vd);
    }
    pub(super) fn v_fcvtn_h(&mut self, q: bool, vd: u32, vn: u32) {
        self.emit(0x0E21_6800 | (q as u32) << 30 | vn << 5 | vd);
    }
    pub(super) fn ldaddal(&mut self, bits: u32, rs: u32, rt: u32, rn: u32) {
        let sz = (bits / 8).trailing_zeros();
        self.emit(0x38E0_0000 | sz << 30 | rs << 16 | rn << 5 | rt);
    }
    pub(super) fn ldclral(&mut self, bits: u32, rs: u32, rt: u32, rn: u32) {
        let sz = (bits / 8).trailing_zeros();
        self.emit(0x38E0_1000 | sz << 30 | rs << 16 | rn << 5 | rt);
    }
    pub(super) fn ldeoral(&mut self, bits: u32, rs: u32, rt: u32, rn: u32) {
        let sz = (bits / 8).trailing_zeros();
        self.emit(0x38E0_2000 | sz << 30 | rs << 16 | rn << 5 | rt);
    }
    pub(super) fn ldsetal(&mut self, bits: u32, rs: u32, rt: u32, rn: u32) {
        let sz = (bits / 8).trailing_zeros();
        self.emit(0x38E0_3000 | sz << 30 | rs << 16 | rn << 5 | rt);
    }
    pub(super) fn swpal(&mut self, bits: u32, rs: u32, rt: u32, rn: u32) {
        let sz = (bits / 8).trailing_zeros();
        self.emit(0x38E0_8000 | sz << 30 | rs << 16 | rn << 5 | rt);
    }
    pub(super) fn casal(&mut self, bits: u32, rs: u32, rt: u32, rn: u32) {
        let sz = (bits / 8).trailing_zeros();
        self.emit(0x08E0_FC00 | sz << 30 | rs << 16 | rn << 5 | rt);
    }
    pub(super) fn casa(&mut self, bits: u32, rs: u32, rt: u32, rn: u32) {
        let sz = (bits / 8).trailing_zeros();
        self.emit(0x08E0_7C00 | sz << 30 | rs << 16 | rn << 5 | rt);
    }
    pub(super) fn casl(&mut self, bits: u32, rs: u32, rt: u32, rn: u32) {
        let sz = (bits / 8).trailing_zeros();
        self.emit(0x08A0_FC00 | sz << 30 | rs << 16 | rn << 5 | rt);
    }
    pub(super) fn ldar(&mut self, bits: u32, rt: u32, rn: u32) {
        let sz = (bits / 8).trailing_zeros();
        self.emit(0x08DF_FC00 | sz << 30 | rn << 5 | rt);
    }
    pub(super) fn caspal_x(&mut self, rs: u32, rt: u32, rn: u32) {
        debug_assert!(rs % 2 == 0 && rt % 2 == 0, "caspal register pair must be even");
        self.emit(0x4860_FC00 | rs << 16 | rn << 5 | rt);
    }
    pub(super) fn yield_hint(&mut self) {
        self.emit(0xD503_203F);
    }
    #[cfg(test)]
    pub(super) fn clrex(&mut self) {
        self.emit(0xD503_3F5F);
    }
    pub(super) fn ldr_reg(&mut self, bits: u32, signed: bool, rt: u32, rn: u32, rm: u32, shift: bool) {
        let sz = (bits / 8).trailing_zeros();
        let opc = if signed { 0b10 } else { 0b01 };
        let s = (shift as u32) << 12;
        self.emit(0x3820_6800 | sz << 30 | opc << 22 | rm << 16 | s | rn << 5 | rt);
    }
}

impl Tr<'_> {
    /// Read the guest clock into EDX:EAX with runtime-selected timing and verifier modes.
    fn emit_timestamp(&mut self, ordered: bool) {
        self.a.ldr(8, T1, XCPU, OFF_TSC_MODE);
        // Keep the direct hardware sample inline. The bank supplies shared leaf routines
        // for other timer rates and verifier reads, including in signed static images.
        let slow = if ordered {
            self.a.dsb_ishld();
            let slow = self.a.cbnz(false, T1);
            self.a.mrs_cntvctss(T0);
            slow
        } else {
            // A full block can exceed the 32 KiB range of TBZ. B reaches its cold fragment.
            let native = self.a.tbz(T1, 1);
            let slow = self.a.b();
            self.a.bind(native);
            self.a.mrs_cntvct(T0);
            slow
        };
        self.tsc_colds.push((slow, self.a.pos, ordered));
        self.a.mov_rr(false, g(0), T0);
        self.a.lsr_i(true, g(2), T0, 32);
        self.set_lb(0, 64);
        self.set_lb(2, 64);
        if ordered {
            self.a.movz(true, g(1), 0, 0);
            self.set_lb(1, 64);
        }
    }

    pub(super) fn emit_timestamp_fallbacks(&mut self) {
        for &(site, resume, ordered) in &self.tsc_colds {
            self.a.bind(site);
            let back = self.a.addr_of(resume);
            if !ordered {
                let uncommon = self.a.tbz(T1, 2);
                self.a.mrs_cntvct(T0);
                super::stubs::emit_tsc_scale_24mhz(&mut self.a);
                self.a.b_to(back);
                self.a.bind(uncommon);
            }
            self.a.bl_to(if ordered { self.rdtscp_slow } else { self.rdtsc_slow });
            self.a.b_to(back);
        }
    }

    /// Inline `ldmxcsr`/`vldmxcsr`: validate reserved bits (`m >> 16 != 0` -> `#GP(0)`), store
    /// `Cpu::jx.mxcsr`, and update hardware `FPCR` (`RMode`, `FZ`, plus `FEAT_AFP` `AH`/`FIZ`
    /// when present) without clobbering `NZCV` or exiting to the interpreter.
    fn emit_ldmxcsr(&mut self, k: usize, op: &FastOp) {
        self.flush_x87_top();
        self.load_op(k, T0, op, 32);
        // Reserved bits [31:16] must be zero; check flag-free via `lsr` + `cbnz`.
        self.a.lsr_i(false, T1, T0, 16);
        let gp_fix = self.a.cbnz(false, T1);
        self.sys_colds.push(SysCold {
            fix: gp_fix,
            k,
            kind: SysColdKind::FaultGp,
            fs: self.fs,
            lb: self.lb,
            xd: self.xd,
            pf: self.pf,
        });
        self.a.str(32, T0, XCPU, OFF_MXCSR);
        // RMode: MXCSR RC bits 14:13 map 00->00 (RN), 01->10 (RM), 10->01 (RP), 11->11 (RZ).
        self.a.ubfx(T1, T0, 13, 1);
        self.a.ubfx(T2, T0, 14, 1);
        self.a.lsl_i(true, T1, T1, 23);
        self.a.logic_rr(true, 1, T1, T1, T2, 0, 22);
        self.a.ubfx(T2, T0, 15, 1); // ftz
        self.a.ubfx(T3, T0, 6, 1); // daz
        self.a.logic_rr(true, 2, T4, T2, T3, 0, 0); // mixed = ftz ^ daz
        if crate::emu::features::host().has(crate::emu::features::HostFeatures::AFP) {
            // Exact match for `cpu::mxcsr_to_fpcr`: FZ = ftz; if mixed, AH = ftz and FIZ = daz.
            self.a.logic_rr(true, 1, T1, T1, T2, 0, 24);
            self.a.addsub_rr(true, true, false, T4, ZR, T4, 0); // 0 if equal, !0 if mixed
            self.a.logic_rr(true, 1, T5, T3, T2, 0, 1); // (ftz << 1) | daz
            self.a.logic_rr(true, 0, T5, T5, T4, 0, 0);
            self.a.logic_rr(true, 1, T1, T1, T5, 0, 0);
        } else {
            // Without FEAT_AFP, ARM64 FPCR only has combined FZ (both input and output flush).
            // Equal FTZ==DAZ stays 100% native; mixed FTZ!=DAZ exits cleanly to soft_flush.
            let soft_fix = self.a.cbnz(true, T4);
            self.sys_colds.push(SysCold {
                fix: soft_fix,
                k,
                kind: SysColdKind::SoftFlushExit,
                fs: self.fs,
                lb: self.lb,
                xd: self.xd,
                pf: self.pf,
            });
            self.a.logic_rr(true, 1, T1, T1, T2, 0, 24);
        }
        self.a.mrs_fpcr(T2);
        self.a.logic_rr(true, 2, T2, T2, T1, 0, 0);
        let skip = self.a.cbz(true, T2);
        self.a.msr_fpcr(T1);
        self.a.bind(skip);
    }

    /// Inline `fnstcw`/`fstcw`: read this thread's `X87State::cw` via `Cpu::jx.x87_cw`.
    fn emit_fnstcw(&mut self, k: usize, op: &FastOp) {
        let a = self.addr_into(op, XA);
        self.a.ldr(64, T1, XCPU, OFF_X87_CW);
        self.a.ldr(16, T0, T1, 0);
        self.store(k, T0, a, 16, Self::chk_of(op));
    }

    /// Inline `fldcw`: load m16, apply architectural mask `(v & 0x1f3f) | 0x40`, store to `X87State::cw`.
    fn emit_fldcw(&mut self, k: usize, op: &FastOp) {
        self.load_op(k, T0, op, 16);
        self.a.movz(false, T1, 0x1F3F, 0);
        self.a.logic_rr(false, 0, T0, T0, T1, 0, 0);
        self.a.logic_ri(false, 1, T0, T0, enc_logic_imm(0x40, false).unwrap());
        self.a.ldr(64, T1, XCPU, OFF_X87_CW);
        self.a.str(16, T0, T1, 0);
    }

    /// Native `cpuid` via `Cpu::jx.sys_helpers.cpuid`: queries the single shared `cpuid::query`
    /// table on the executing thread with zero baked addresses and full FPCR/XMM preservation.
    fn emit_cpuid(&mut self, k: usize) {
        self.flush_flags(self.live[k]);
        self.fs.host = 0;
        if self.xpin {
            self.a.bl_to(self.spill_xmm);
        }
        // Frame (96B, 16B-aligned): [0..72) x7..x15, [72..80) saved FPCR, [80..96) [u32; 4] out.
        self.a.addsub_ri(true, true, false, 31, 31, 96, false);
        self.a.stp_x(7, 8, 31, 0);
        self.a.stp_x(9, 10, 31, 16);
        self.a.stp_x(11, 12, 31, 32);
        self.a.stp_x(13, 14, 31, 48);
        self.a.str(64, 15, 31, 64);
        self.a.mov_rr(false, 0, g(0)); // w0 = leaf (eax)
        self.a.mov_rr(false, 1, g(1)); // w1 = subleaf (ecx)
        self.a.addsub_ri(true, false, false, 2, 31, 80, false); // x2 = &mut [u32; 4]
        self.a.ldr(64, XA, XCPU, OFF_SYS_HELPERS);
        self.a.ldr(64, XA, XA, SYS_CPUID_OFF);
        self.a.call_with_host_fpcr(72);
        self.a.ldp_x(7, 8, 31, 0);
        self.a.ldp_x(9, 10, 31, 16);
        self.a.ldp_x(11, 12, 31, 32);
        self.a.ldp_x(13, 14, 31, 48);
        self.a.ldr(64, 15, 31, 64);
        if self.xpin {
            self.a.bl_to(self.reload_xmm);
        }
        // 32-bit loads zero-extend bits 63:32 of RAX/RBX/RCX/RDX.
        self.a.ldr(32, g(0), 31, 80); // eax
        self.a.ldr(32, g(3), 31, 84); // ebx
        self.a.ldr(32, g(1), 31, 88); // ecx
        self.a.ldr(32, g(2), 31, 92); // edx
        self.a.addsub_ri(true, false, false, 31, 31, 96, false);
        self.set_lb(0, 64);
        self.set_lb(1, 64);
        self.set_lb(2, 64);
        self.set_lb(3, 64);
    }

    /// Inline `xgetbv`: `#GP(0)` if `ecx != 0`; otherwise `eax = XCR0 (7)`, `edx = 0`.
    fn emit_xgetbv(&mut self, k: usize) {
        self.flush_x87_top();
        let gp_fix = self.a.cbnz(false, g(1));
        self.sys_colds.push(SysCold {
            fix: gp_fix,
            k,
            kind: SysColdKind::FaultGp,
            fs: self.fs,
            lb: self.lb,
            xd: self.xd,
            pf: self.pf,
        });
        self.a.movz(false, g(0), crate::emu::cpuid::XCR0, 0);
        self.a.movz(true, g(2), 0, 0);
        self.set_lb(0, 64);
        self.set_lb(2, 64);
    }

    /// Native `FXSAVE*`/`FXRSTOR*`/`XSAVE*`/`XRSTOR*` via `Cpu::jx.sys_helpers.xstate`.
    fn emit_xstate(&mut self, k: usize, op: &FastOp) {
        self.flush_x87_top();
        self.flush_flags(FALL);
        self.fs.host = 0;
        self.flush_lb();
        let a = self.addr_into(op, XA);
        self.a.mov_rr(true, T2, a); // x2 = guest area address
        if self.xpin {
            self.a.bl_to(self.spill_xmm);
        }
        for i in 0..8 {
            self.a.stp_x(HG[2 * i], HG[2 * i + 1], XCPU, (OFF_GPR + 16 * i as u32) as i32);
        }
        if self.ras_miss != 0 {
            self.a.str(64, XRAS, XCPU, OFF_RAS);
        }
        self.a.mov_imm(true, T3, op.ip);
        self.a.str(64, T3, XCPU, OFF_RIP);
        self.a.addsub_ri(true, true, false, 31, 31, 16, false);
        self.a.stp_x(XBUD, XCPU, 31, 0);
        self.a.mov_rr(true, 0, XCPU); // x0 = cpu
        self.a.movz(false, 1, op.w as u32, 0); // w1 = xstate flags
        // x2 = addr, x3 = op.ip
        self.a.ldr(64, XS, XCPU, OFF_HOST_FPCR);
        self.a.msr_fpcr(XS);
        self.a.ldr(64, XA, XCPU, OFF_SYS_HELPERS);
        self.a.ldr(64, XA, XA, SYS_XSTATE_OFF);
        self.a.blr(XA);
        self.a.ldp_x(XBUD, XCPU, 31, 0);
        self.a.addsub_ri(true, false, false, 31, 31, 16, false);
        let abn = self.a.cbnz(false, T0);
        self.sys_colds.push(SysCold {
            fix: abn,
            k,
            kind: SysColdKind::XstateAbnormal,
            fs: self.fs,
            lb: self.lb,
            xd: 0,
            pf: PF_NONE,
        });
        // Normal completion: reload caller-saved guest GPRs r10..r15 (`HG[10..16]`), XRAS, XMMs.
        for i in 5..8 {
            self.a.ldp_x(HG[2 * i], HG[2 * i + 1], XCPU, (OFF_GPR + 16 * i as u32) as i32);
        }
        if self.ras_miss != 0 {
            self.a.ldr(64, XRAS, XCPU, OFF_RAS);
        }
        if self.xpin {
            self.a.bl_to(self.reload_xmm);
        }
        if op.w & crate::emu::xstate::XS_FLAG_RESTORE != 0 {
            self.a.ldr(16, XS, XCPU, OFF_YMM_DIRTY);
            self.a.addsub_ri(false, true, true, ZR, XS, 0, false);
            self.a.cset(XS, NE);
            self.a.bfxil0(XBUD, XS, 1);
            self.ymm_clean = 0;
            self.x87_top_known = None;
            self.x87_top_dirty = false;
            self.x87_sw_top_dirty = false;
        }
    }

    /// Out-of-line cold exits for `#GP(0)` faults, non-AFP mixed MXCSR, and abnormal XSTATE exits.
    pub(super) fn emit_sys_colds(&mut self) {
        let n = self.ops.len();
        let mut colds = core::mem::take(&mut self.sys_colds);
        for c in &colds {
            self.a.bind(c.fix);
            let saved = (self.fs, self.lb, self.pf);
            self.fs = c.fs;
            self.lb = c.lb;
            self.pf = c.pf;
            match c.kind {
                SysColdKind::FaultGp => {
                    self.flush_flags(FALL);
                    self.flush_lb();
                    self.xstore(c.xd);
                    if self.xpin {
                        self.a.bl_to(self.spill_xmm);
                    }
                    for i in 0..8 {
                        self.a.stp_x(HG[2 * i], HG[2 * i + 1], XCPU, (OFF_GPR + 16 * i as u32) as i32);
                    }
                    if self.ras_miss != 0 {
                        self.a.str(64, XRAS, XCPU, OFF_RAS);
                    }
                    if self.bud_dec {
                        self.a.addsub_ri(true, false, false, XBUD, XBUD, (2 * (n - c.k)) as u32, false);
                    }
                    self.a.addsub_ri(true, true, false, 31, 31, 16, false);
                    self.a.stp_x(XBUD, XCPU, 31, 0);
                    self.a.mov_imm(true, T1, self.ops[c.k].ip);
                    self.a.str(64, T1, XCPU, OFF_RIP);
                    self.a.mov_rr(true, 0, XCPU);
                    self.a.ldr(64, XS, XCPU, OFF_HOST_FPCR);
                    self.a.msr_fpcr(XS);
                    self.a.ldr(64, XA, XCPU, OFF_SYS_HELPERS);
                    self.a.ldr(64, XA, XA, SYS_FAULT_GP_OFF);
                    self.a.blr(XA);
                    self.a.ldp_x(XBUD, XCPU, 31, 0);
                    self.a.addsub_ri(true, false, false, 31, 31, 16, false);
                    for i in 0..8 {
                        self.a.ldp_x(HG[2 * i], HG[2 * i + 1], XCPU, (OFF_GPR + 16 * i as u32) as i32);
                    }
                    if self.xpin {
                        self.a.bl_to(self.reload_xmm);
                    }
                    if self.ras_miss != 0 {
                        self.a.ldr(64, XRAS, XCPU, OFF_RAS);
                    }
                    self.a.movz(false, 0, EXIT_RIP as u32, 0);
                    self.a.b_to(self.exit);
                }
                SysColdKind::SoftFlushExit => {
                    self.a.mrs_fpcr(T2);
                    self.a.logic_rr(true, 2, T2, T2, T1, 0, 0);
                    let skip = self.a.cbz(true, T2);
                    self.a.msr_fpcr(T1);
                    self.a.bind(skip);
                    self.flush_flags(self.live[c.k + 1]);
                    self.flush_lb();
                    self.xstore(c.xd);
                    if self.bud_dec && n > c.k + 1 {
                        self.a.addsub_ri(true, false, false, XBUD, XBUD, (2 * (n - (c.k + 1))) as u32, false);
                    }
                    let next_ip = self.ops[c.k].ip.wrapping_add(self.ops[c.k].len as u64);
                    self.a.mov_imm(true, T0, next_ip);
                    self.a.str(64, T0, XCPU, OFF_RIP);
                    self.a.movz(false, 0, EXIT_RIP as u32, 0);
                    self.a.b_to(self.exit);
                }
                SysColdKind::XstateAbnormal => {
                    self.a.addsub_ri(false, true, false, T1, T0, 1, false);
                    let is_fault = self.a.cbnz(false, T1);
                    let next_ip = self.ops[c.k].ip.wrapping_add(self.ops[c.k].len as u64);
                    self.a.mov_imm(true, T1, next_ip);
                    self.a.str(64, T1, XCPU, OFF_RIP);
                    if self.bud_dec && n > c.k + 1 {
                        self.a.addsub_ri(true, false, false, XBUD, XBUD, (2 * (n - (c.k + 1))) as u32, false);
                    }
                    let done = self.a.b();
                    self.a.bind(is_fault);
                    if self.bud_dec {
                        self.a.addsub_ri(true, false, false, XBUD, XBUD, (2 * (n - c.k)) as u32, false);
                    }
                    self.a.bind(done);
                    for i in 0..8 {
                        self.a.ldp_x(HG[2 * i], HG[2 * i + 1], XCPU, (OFF_GPR + 16 * i as u32) as i32);
                    }
                    if self.xpin {
                        self.a.bl_to(self.reload_xmm);
                    }
                    if self.ras_miss != 0 {
                        self.a.ldr(64, XRAS, XCPU, OFF_RAS);
                    }
                    self.a.movz(false, 0, EXIT_RIP as u32, 0);
                    self.a.b_to(self.exit);
                }
            }
            (self.fs, self.lb, self.pf) = saved;
        }
        colds.clear();
        self.sys_colds = colds;
    }

    /// Load an 8-bit source operand for `K_HI8` into `dst` (zero-extended to 64 bits).
    pub(super) fn rd_hi8_src(&mut self, k: usize, op: &FastOp, spec: u8, dst: u32) -> Option<u32> {
        if spec & 0x40 != 0 {
            let a = self.addr_into(op, XA);
            self.load(k, dst, a, 8, Self::chk_of(op));
            Some(a)
        } else if spec & 0x20 != 0 {
            self.a.mov_imm(false, dst, op.imm & 0xFF);
            None
        } else {
            let idx = spec & 0x0F;
            if spec & 0x80 != 0 {
                self.a.ubfx(dst, g(idx), 8, 8);
            } else {
                self.a.ubfm(false, dst, g(idx), 0, 7);
            }
            None
        }
    }

    /// Write an 8/16/32/64-bit result for `K_HI8` according to `spec`.
    pub(super) fn wr_hi8_dst(&mut self, spec: u8, w: u8, val: u32, mem_addr: Option<u32>) {
        if spec & 0x40 != 0 {
            self.store_checked_addr(val, mem_addr.unwrap(), 8);
        } else if spec & 0x80 != 0 {
            let idx = spec & 0x0F;
            self.a.bfm(true, g(idx), val, 56, 7);
            self.set_lb(idx, 64);
        } else {
            self.wr_gpr(spec & 0x0F, w, val);
        }
    }

    /// Emit one 128-bit lane of AES / VAES (`sub 0..=5`) on `(v0 = state, v1 = round_key) -> v0`.
    fn emit_vaes_lane(&mut self, sub: u8, imm8: u8) {
        match sub {
            0 => {
                self.a.v_zero(2);
                self.a.v_aese(0, 2);
                self.a.v_aesmc(0, 0);
                self.a.v_eor(0, 0, 1);
            }
            1 => {
                self.a.v_zero(2);
                self.a.v_aese(0, 2);
                self.a.v_eor(0, 0, 1);
            }
            2 => {
                self.a.v_zero(2);
                self.a.v_aesd(0, 2);
                self.a.v_aesimc(0, 0);
                self.a.v_eor(0, 0, 1);
            }
            3 => {
                self.a.v_zero(2);
                self.a.v_aesd(0, 2);
                self.a.v_eor(0, 0, 1);
            }
            4 => {
                self.a.v_aesimc(0, 1);
            }
            _ => {
                self.a.v_zero(2);
                self.a.v_aese(1, 2);
                self.a.mov_imm(true, T0, 0x040B_0E01_0B0E_0104);
                self.a.mov_imm(true, T1, 0x0C03_0609_0306_090C);
                self.a.fmov_gf(true, 2, T0);
                self.a.v_ins_d(2, 1, T1);
                self.a.v_tbl1(0, 1, 2);
                self.a.mov_imm(true, T0, (imm8 as u64) << 32);
                self.a.v_dup(3, 2, T0);
                self.a.v_eor(0, 0, 2);
            }
        }
    }

    /// Multiply 16 bytes of `a` and `b` in GF(2^8) mod `0x11B` into `dst` (baseline ARMv8.0 NEON).
    /// `acc`, `cur_b`, `tmp`, and `c1b` (preloaded with `0x1B` in every byte) must be distinct from
    /// each other and from `a`/`b`; `dst` may alias `a` or `b`. Preserves NZCV.
    fn emit_gfmul_regs(&mut self, dst: u32, a: u32, b: u32, acc: u32, cur_b: u32, tmp: u32, c1b: u32) {
        self.a.v_zero(acc);
        self.a.v_mov(cur_b, b);
        self.a.movz(false, T1, 8, 0);
        let lp = self.a.pos;
        // acc = (acc << 1) ^ ((acc & 0x80) ? 0x1B : 0)
        self.a.v_cmlt0(0, tmp, acc);
        self.a.v_shl_i(8, acc, acc, 1);
        self.a.v_and(tmp, tmp, c1b);
        self.a.v_eor(acc, acc, tmp);
        // acc ^= (cur_b & 0x80) ? a : 0
        self.a.v_cmlt0(0, tmp, cur_b);
        self.a.v_and(tmp, tmp, a);
        self.a.v_eor(acc, acc, tmp);
        self.a.v_shl_i(8, cur_b, cur_b, 1);
        self.a.addsub_ri(false, true, false, T1, T1, 1, false);
        let br = self.a.cbnz(false, T1);
        self.a.patch(br, lp);
        self.a.v_mov(dst, acc);
    }

    /// Emit one 128-bit lane of GFNI (`0=gf2p8mulb`, `1=gf2p8affineqb`, `2=gf2p8affineinvqb`)
    /// on `(v0 = src1, v1 = src2) -> v0`. `c1b` is a free scratch vector (`6` on lo lane, `7` on hi).
    /// Preserves NZCV and the opposite 256-bit lane register.
    fn emit_gfni_lane(&mut self, sub: u8, imm8: u8, c1b: u32) {
        if sub == 0 {
            self.a.movz(false, T0, 0x1B, 0);
            self.a.v_dup(0, c1b, T0);
            self.emit_gfmul_regs(0, 0, 1, 2, 3, 4, c1b);
            return;
        }
        if sub == 2 {
            // Invert each byte of v0 in GF(2^8) mod 0x11B via x^254 = x^2 * x^4 * ... * x^128
            // (maps 0 -> 0 with zero branches). Preserves v1 (the 8x8 bit-matrices).
            self.a.movz(false, T0, 0x1B, 0);
            self.a.v_dup(0, c1b, T0);
            self.emit_gfmul_regs(5, 0, 0, 2, 3, 4, c1b); // v5 = x^2
            self.a.v_mov(0, 5); // v0 = acc = x^2
            self.a.movz(false, T2, 6, 0);
            let lp = self.a.pos;
            self.emit_gfmul_regs(5, 5, 5, 2, 3, 4, c1b); // v5 = v5^2
            self.emit_gfmul_regs(0, 0, 5, 2, 3, 4, c1b); // v0 = v0 * v5
            self.a.addsub_ri(false, true, false, T2, T2, 1, false);
            let br = self.a.cbnz(false, T2);
            self.a.patch(br, lp);
        }
        // Affine transform: for each qword, row k (byte k of qword in v1) yields output bit (7 - k).
        self.a.v_zero(2); // v2 = result bytes
        self.a.movz(true, T0, 0, 0);
        self.a.fmov_gf(true, 4, T0);
        self.a.mov_imm(true, T0, 0x0808_0808_0808_0808);
        self.a.v_ins_d(4, 1, T0); // v4 = [0;8, 8;8]
        self.a.movz(false, T0, 1, 0);
        self.a.v_dup(0, 5, T0); // v5 = [1;16]
        self.a.movz(false, T2, 8, 0);
        let lp = self.a.pos;
        self.a.v_tbl1(3, 1, 4); // v3 = matrix row k broadcast per qword
        self.a.v_and(3, 3, 0); // v3 = x & row
        self.a.v_cnt_16b(3, 3); // v3 = popcount(x & row)
        self.a.v_and(3, 3, 5); // v3 = parity bit (0 or 1)
        self.a.v_shl_i(8, 2, 2, 1); // v2 <<= 1
        self.a.v_orr(2, 2, 3); // v2 |= bit (7 - k)
        self.a.neon_3same(false, 0, 16, 4, 4, 5); // add v4.16b, v4.16b, v5.16b
        self.a.addsub_ri(false, true, false, T2, T2, 1, false);
        let br = self.a.cbnz(false, T2);
        self.a.patch(br, lp);
        if imm8 != 0 {
            self.a.movz(false, T0, imm8 as u32, 0);
            self.a.v_dup(0, 3, T0);
            self.a.v_eor(2, 2, 3);
        }
        self.a.v_mov(0, 2);
    }

    /// Emit one 128-bit lane of AVX-VNNI (`0=vpdpbusd`, `1=vpdpbusds`, `2=vpdpwssd`, `3=vpdpwssds`)
    /// on `(v5 = dst, v0 = src1, v1 = src2) -> v0`. Preserves `v6`, `v7`, and NZCV.
    fn emit_vnni_lane(&mut self, sub: u8) {
        match sub {
            0 | 1 => {
                // src1: u8, src2: i8 -> each product in [-32640, 32385] fits i16;
                // sum of 4 products in [-130560, 129540] fits i32 without overflow.
                self.a.v_uxtl_b(false, 2, 0);
                self.a.v_sxtl_b(false, 3, 1);
                self.a.neon_3same(false, 1, 19, 2, 2, 3); // mul v2.8h, v2.8h, v3.8h
                self.a.v_saddlp(1, 2, 2); // saddlp v2.4s, v2.8h
                self.a.v_uxtl_b(true, 3, 0);
                self.a.v_sxtl_b(true, 4, 1);
                self.a.neon_3same(false, 1, 19, 3, 3, 4); // mul v3.8h, v3.8h, v4.8h
                self.a.v_saddlp(1, 3, 3); // saddlp v3.4s, v3.8h
                self.a.neon_3same(false, 2, 23, 2, 2, 3); // addp v2.4s, v2.4s, v3.4s
                let opc = if sub == 0 { 16 } else { 1 }; // add vs sqadd
                self.a.neon_3same(false, 2, opc, 0, 5, 2);
            }
            2 => {
                // vpdpwssd: wrapping 32-bit accumulation of pairs of i16*i16 products.
                self.a.v_smull(false, 1, 2, 0, 1);
                self.a.v_smull(true, 1, 3, 0, 1);
                self.a.neon_3same(false, 2, 23, 2, 2, 3); // addp v2.4s, v2.4s, v3.4s
                self.a.neon_3same(false, 2, 16, 0, 5, 2); // add v0.4s, v5.4s, v2.4s
            }
            _ => {
                // vpdpwssds: accumulate in i64 before saturating to i32 so
                // (-32768)^2 + (-32768)^2 = +2^31 saturates instead of wrapping negative.
                self.a.v_smull(false, 1, 2, 0, 1); // v2.4s = p0..p3
                self.a.v_smull(true, 1, 3, 0, 1); // v3.4s = p4..p7
                self.a.v_saddlp(2, 2, 2); // v2.2d = [p0+p1, p2+p3] (i64)
                self.a.v_saddlp(2, 3, 3); // v3.2d = [p4+p5, p6+p7] (i64)
                self.a.v_sxtl_s(false, 4, 5); // v4.2d = [dst0, dst1] (i64)
                self.a.neon_3same(false, 3, 16, 2, 2, 4); // add v2.2d, v2.2d, v4.2d
                self.a.v_sxtl_s(true, 4, 5); // v4.2d = [dst2, dst3] (i64)
                self.a.neon_3same(false, 3, 16, 3, 3, 4); // add v3.2d, v3.2d, v4.2d
                self.a.v_sqxtn(false, 2, 0, 2); // sqxtn v0.2s, v2.2d
                self.a.v_sqxtn(true, 2, 0, 3); // sqxtn2 v0.4s, v3.2d
            }
        }
    }

    pub(super) fn emit_more_op(&mut self, k: usize, op: &FastOp) {
        use fast::*;
        let w = op.w;
        let wb = w as u32;
        let la = self.live[k + 1];
        match op.kind {
            K_VAES_RR | K_VAES_RM => {
                let sub = op.x & 0x3F;
                let imm8 = op.imm as u8;
                let is_256 = (op.w & 3) == VM_V256;
                self.vload_b(k, op);
                if sub <= 3 {
                    self.xsrc(0, (op.imm >> 8) as u8);
                }
                self.emit_vaes_lane(sub, imm8);
                if is_256 {
                    let s1 = ((op.imm >> 8) as u32 & 15) * 16;
                    self.a.v_mov(6, 0);
                    self.a.ldr(128, 0, XCPU, OFF_YMM_HI + s1);
                    self.a.v_mov(1, 7);
                    self.emit_vaes_lane(sub, imm8);
                }
                self.vstore_dst(op);
            }
            K_VCVT_PH => {
                let sub = op.x & 0x1F;
                let mem = (op.x & 0x20) != 0;
                if sub == 0 {
                    // Vcvtph2ps
                    let is_256 = (op.w & 3) == VM_V256;
                    if mem {
                        let a = self.addr_into(op, XA);
                        if is_256 {
                            if Self::chk_of(op) == Chk::Full {
                                let se = self.side_exit_here(k);
                                self.range_check_to(a, se);
                            }
                            self.a.ldr(64, T0, a, 0);
                            self.a.ldr(64, T1, a, 8);
                            if mem::tso() {
                                self.a.dmb_ishld();
                            }
                            self.a.fmov_gf(true, 0, T0);
                            self.a.v_ins_d(0, 1, T1);
                        } else {
                            self.load(k, T0, a, 64, Self::chk_of(op));
                            self.a.fmov_gf(true, 0, T0);
                        }
                    } else {
                        self.xsrc(0, op.r1);
                    }
                    if is_256 {
                        self.a.v_fcvtl_h(true, 1, 0);
                    }
                    self.a.v_fcvtl_h(false, 0, 0);
                    self.xwrite(op.r0, 0);
                    if is_256 {
                        self.write_ymm_hi(op.r0, 1);
                    } else {
                        self.clear_ymm_hi(op.r0);
                    }
                } else {
                    // Vcvtps2ph
                    let src_256 = op.w != 0;
                    let s = (op.r1 as u32 & 15) * 16;
                    self.xsrc(0, op.r1);
                    if src_256 {
                        self.a.ldr(128, 1, XCPU, OFF_YMM_HI + s);
                    }
                    let imm = op.imm as u8;
                    if (imm & 4) != 0 {
                        // imm8[2]=1: use MXCSR.RC (already installed in FPCR.RMode).
                        self.a.v_fcvtn_h(false, 0, 0);
                        if src_256 {
                            self.a.v_fcvtn_h(true, 0, 1);
                        }
                    } else {
                        // imm8[2]=0: override FPCR.RMode from imm8[1:0] without touching NZCV or flush bits.
                        let arm_rm: u64 = match imm & 3 {
                            0 => 0, // RN
                            1 => 2, // RM (-inf)
                            2 => 1, // RP (+inf)
                            _ => 3, // RZ (0)
                        };
                        self.a.mrs_fpcr(T0);
                        self.a.mov_imm(true, T2, !(3u64 << 22));
                        self.a.logic_rr(true, 0, T1, T0, T2, 0, 0);
                        if arm_rm != 0 {
                            self.a.mov_imm(true, T2, arm_rm << 22);
                            self.a.logic_rr(true, 1, T1, T1, T2, 0, 0);
                        }
                        self.a.logic_rr(true, 2, T2, T0, T1, 0, 0);
                        let same_pre = self.a.cbz(true, T2);
                        self.a.msr_fpcr(T1);
                        self.a.bind(same_pre);
                        self.a.v_fcvtn_h(false, 0, 0);
                        if src_256 {
                            self.a.v_fcvtn_h(true, 0, 1);
                        }
                        let same_post = self.a.cbz(true, T2);
                        self.a.msr_fpcr(T0);
                        self.a.bind(same_post);
                    }
                    if mem {
                        let a = self.addr_into(op, XA);
                        if src_256 {
                            let chk = Self::chk_of(op);
                            if chk == Chk::Full {
                                let se = self.side_exit_here(k);
                                self.range_check_to(a, se);
                            }
                            self.a.fmov_fg(true, T0, 0);
                            self.a.v_umov_d(T1, 0, 1);
                            if mem::tso() && (chk != Chk::Stack || stack_tso()) {
                                self.a.dmb_ishst();
                            }
                            self.a.str(64, T0, a, 0);
                            self.a.str(64, T1, a, 8);
                        } else {
                            self.a.fmov_fg(true, T0, 0);
                            self.store(k, T0, a, 64, Self::chk_of(op));
                        }
                    } else {
                        self.xwrite(op.r0, 0);
                        self.clear_ymm_hi(op.r0);
                    }
                }
            }
            K_VMASKMOV => {
                let is_store = (op.x & 0x20) != 0;
                let is_256 = (op.w & 3) == VM_V256;
                let is_q = (op.w & 4) != 0;
                let (lg, ebits, esz, per_lane) = if is_q {
                    (3u32, 64u32, 8u32, 2u32)
                } else {
                    (2, 32, 4, 4)
                };
                let d_or_s = (op.r0 as u32 & 15) * 16;
                let m_off = (op.r1 as u32 & 15) * 16;
                let base_a = self.addr_into(op, T5);
                if base_a != T5 {
                    self.a.mov_rr(true, T5, base_a);
                }
                self.xsrc(2, op.r1);
                if is_256 {
                    self.a.ldr(128, 3, XCPU, OFF_YMM_HI + m_off);
                }
                let halves = if is_256 { 2u32 } else { 1u32 };
                if !is_store {
                    self.a.v_zero(0);
                    self.a.v_zero(1);
                    for h in 0..halves {
                        for lane in 0..per_lane {
                            self.a.v_umov_elem(lg, T1, 2 + h, lane);
                            let skip = self.a.tbz(T1, ebits - 1);
                            let off_bytes = h * 16 + lane * esz;
                            self.a.add_imm(true, XA, T5, off_bytes as i64);
                            self.load(k, T0, XA, ebits, Chk::Full);
                            self.a.v_ins_elem(lg, h, lane, T0);
                            self.a.bind(skip);
                        }
                    }
                    self.xwrite(op.r0, 0);
                    if is_256 {
                        self.write_ymm_hi(op.r0, 1);
                    } else {
                        self.clear_ymm_hi(op.r0);
                    }
                } else {
                    self.xsrc(0, op.r0);
                    if is_256 {
                        self.a.ldr(128, 1, XCPU, OFF_YMM_HI + d_or_s);
                    }
                    for h in 0..halves {
                        for lane in 0..per_lane {
                            self.a.v_umov_elem(lg, T1, 2 + h, lane);
                            let skip = self.a.tbz(T1, ebits - 1);
                            self.a.v_umov_elem(lg, T0, h, lane);
                            let off_bytes = h * 16 + lane * esz;
                            self.a.add_imm(true, XA, T5, off_bytes as i64);
                            self.store(k, T0, XA, ebits, Chk::Full);
                            self.a.bind(skip);
                        }
                    }
                }
            }
            K_SYS_MISC => {
                let sub = op.x & 0x3F;
                match sub {
                    0 => {
                        // Lahf
                        self.flush_flags(FC | FZ | FS | FP);
                        self.a.ldr(8, T0, XCPU, OFF_SF);
                        self.a.lsl_i(false, T0, T0, 7);
                        self.a.ldr(8, T1, XCPU, OFF_ZF);
                        self.a.logic_rr(false, 1, T0, T0, T1, 0, 6);
                        self.a.ldr(8, T1, XCPU, OFF_PFV);
                        self.a.fmov_gf(false, 0, T1);
                        self.a.v_cnt_8b(0, 0);
                        self.a.fmov_fg(false, T1, 0);
                        self.a.logic_ri(false, 2, T1, T1, enc_logic_imm(1, false).unwrap());
                        self.a.ubfm(false, T1, T1, 0, 0);
                        self.a.logic_rr(false, 1, T0, T0, T1, 0, 2);
                        self.a.ldr(8, T1, XCPU, OFF_CF);
                        self.a.logic_rr(false, 1, T0, T0, T1, 0, 0);
                        self.a.movz(false, T1, 2, 0);
                        self.a.logic_rr(false, 1, T0, T0, T1, 0, 0);
                        self.a.bfm(true, g(0), T0, 56, 7);
                        self.set_lb(0, 64);
                    }
                    1 => {
                        // Sahf
                        self.a.ubfx(T0, g(0), 8, 8);
                        if la & FS != 0 {
                            self.a.ubfx(T1, T0, 7, 1);
                            self.a.str(8, T1, XCPU, OFF_SF);
                        }
                        if la & FZ != 0 {
                            self.a.ubfx(T1, T0, 6, 1);
                            self.a.str(8, T1, XCPU, OFF_ZF);
                        }
                        if la & FC != 0 {
                            self.a.ubfx(T1, T0, 0, 1);
                            self.a.str(8, T1, XCPU, OFF_CF);
                        }
                        if la & FP != 0 {
                            self.a.ubfx(T1, T0, 2, 1);
                            self.a.logic_ri(false, 2, T1, T1, enc_logic_imm(1, false).unwrap());
                            self.a.str(8, T1, XCPU, OFF_PFV);
                        }
                        self.fs.host &= !(FC | FZ | FS | FP);
                        self.fs.mem = (self.fs.mem & !(FC | FZ | FS | FP)) | (la & (FC | FZ | FS | FP));
                    }
                    2 => {
                        // Pushf / Pushfq
                        self.flush_flags(FALL);
                        self.a.movz(false, T0, 0x202, 0);
                        self.a.ldr(8, T1, XCPU, OFF_CF);
                        self.a.logic_rr(false, 1, T0, T0, T1, 0, 0);
                        self.a.ldr(8, T1, XCPU, OFF_PFV);
                        self.a.fmov_gf(false, 0, T1);
                        self.a.v_cnt_8b(0, 0);
                        self.a.fmov_fg(false, T1, 0);
                        self.a.logic_ri(false, 2, T1, T1, enc_logic_imm(1, false).unwrap());
                        self.a.ubfm(false, T1, T1, 0, 0);
                        self.a.logic_rr(false, 1, T0, T0, T1, 0, 2);
                        self.a.ldr(8, T1, XCPU, OFF_ZF);
                        self.a.logic_rr(false, 1, T0, T0, T1, 0, 6);
                        self.a.ldr(8, T1, XCPU, OFF_SF);
                        self.a.logic_rr(false, 1, T0, T0, T1, 0, 7);
                        self.a.ldr(8, T1, XCPU, OFF_DF);
                        self.a.logic_rr(false, 1, T0, T0, T1, 0, 10);
                        self.a.ldr(8, T1, XCPU, OFF_OF);
                        self.a.logic_rr(false, 1, T0, T0, T1, 0, 11);
                        self.a.addsub_ri(true, true, false, XA, g(4), wb / 8, false);
                        self.store(k, T0, XA, wb, if wb == 64 { Chk::Stack } else { Chk::Full });
                        self.a.mov_rr(true, g(4), XA);
                    }
                    3 => {
                        // Popf / Popfq
                        self.load(k, T0, g(4), wb, if wb == 64 { Chk::Stack } else { Chk::Full });
                        self.a.addsub_ri(true, false, false, g(4), g(4), wb / 8, false);
                        self.a.ubfx(T1, T0, 0, 1);
                        self.a.str(8, T1, XCPU, OFF_CF);
                        self.a.ubfx(T1, T0, 2, 1);
                        self.a.logic_ri(false, 2, T1, T1, enc_logic_imm(1, false).unwrap());
                        self.a.str(8, T1, XCPU, OFF_PFV);
                        self.a.ubfx(T1, T0, 6, 1);
                        self.a.str(8, T1, XCPU, OFF_ZF);
                        self.a.ubfx(T1, T0, 7, 1);
                        self.a.str(8, T1, XCPU, OFF_SF);
                        self.a.ubfx(T1, T0, 10, 1);
                        self.a.str(8, T1, XCPU, OFF_DF);
                        self.a.ubfx(T1, T0, 11, 1);
                        self.a.str(8, T1, XCPU, OFF_OF);
                        self.fs = FlagState { src: Src::None, host: 0, mem: FALL };
                    }
                    4 => {
                        // Stmxcsr / Vstmxcsr
                        let a = self.addr_into(op, XA);
                        self.a.ldr(32, T0, XCPU, OFF_MXCSR);
                        self.store(k, T0, a, 32, Self::chk_of(op));
                    }
                    5 => self.emit_ldmxcsr(k, op),
                    6 => {
                        // Xlatb
                        self.a.ubfm(true, T0, g(0), 0, 7);
                        self.a.addsub_rr(true, false, false, XA, g(3), T0, 0);
                        self.load(k, T1, XA, 8, Chk::Full);
                        self.wr_gpr(0, 8, T1);
                    }
                    7 => self.emit_fnstcw(k, op),
                    8 => self.emit_timestamp(w != 0),
                    9 => self.emit_cpuid(k),
                    10 => {
                        // 16-bit push
                        match w {
                            0 => self.a.ubfm(false, T0, g(op.r0), 0, 15),
                            1 => {
                                let a = self.addr_into(op, XA);
                                self.load(k, T0, a, 16, Self::chk_of(op));
                            }
                            _ => self.a.mov_imm(false, T0, op.imm & 0xFFFF),
                        }
                        self.a.addsub_ri(true, true, false, XA, g(4), 2, false);
                        self.store(k, T0, XA, 16, Chk::Full);
                        self.a.mov_rr(true, g(4), XA);
                    }
                    11 => {
                        // 16-bit pop
                        self.load(k, T0, g(4), 16, Chk::Full);
                        if w == 0 {
                            self.a.addsub_ri(true, false, false, g(4), g(4), 2, false);
                            self.wr_gpr(op.r0, 16, T0);
                        } else {
                            self.a.addsub_ri(true, false, false, g(4), g(4), 2, false);
                            let a = self.addr_into(op, XA);
                            self.a.addsub_ri(true, true, false, g(4), g(4), 2, false);
                            self.store(k, T0, a, 16, Self::chk_of(op));
                            self.a.addsub_ri(true, false, false, g(4), g(4), 2, false);
                        }
                    }
                    12 => {
                        // Enter imm16, 0: push rbp; mov rbp, rsp; sub rsp, imm16
                        self.a.addsub_ri(true, true, false, XA, g(4), 8, false);
                        self.store(k, g(5), XA, 64, Chk::Stack);
                        self.a.mov_rr(true, g(5), XA);
                        if op.imm != 0 {
                            self.a.mov_imm(true, T0, op.imm & 0xFFFF);
                            self.a.addsub_rr(true, true, false, g(4), XA, T0, 0);
                        } else {
                            self.a.mov_rr(true, g(4), XA);
                        }
                        self.set_lb(4, 64);
                        self.set_lb(5, 64);
                    }
                    13 => {
                        // Rdrand / Rdseed: real OS entropy via `JitScratch::entropy_fn`
                        // (`guest_entropy_raw` -> `EntropyResult { val: x0, ok: x1 }`), zero
                        // static AOT relocations. Intel SDM: on success `dst = val`, `CF = 1`;
                        // on failure `dst = 0`, `CF = 0`; `OF = SF = ZF = AF = PF = 0` always.
                        self.a.addsub_ri(true, true, false, 31, 31, 96, false);
                        self.a.stp_x(XRAS, XBUD, 31, 0);
                        self.a.stp_x(9, 10, 31, 16);
                        self.a.stp_x(11, 12, 31, 32);
                        self.a.stp_x(13, 14, 31, 48);
                        self.a.stp_x(XCPU, 30, 31, 64);
                        if self.spill_xmm != 0 {
                            self.a.bl_to(self.spill_xmm);
                        }
                        self.a.ldr(64, XA, XCPU, OFF_ENTROPY_FN);
                        self.a.call_with_host_fpcr(80);
                        self.a.ldr(64, XCPU, 31, 64);
                        if self.reload_xmm != 0 {
                            self.a.bl_to(self.reload_xmm);
                        }
                        self.a.ldp_x(XCPU, 30, 31, 64);
                        self.a.ldp_x(13, 14, 31, 48);
                        self.a.ldp_x(11, 12, 31, 32);
                        self.a.ldp_x(9, 10, 31, 16);
                        self.a.ldp_x(XRAS, XBUD, 31, 0);
                        self.a.addsub_ri(true, false, false, 31, 31, 96, false);
                        self.xa_key = None;
                        self.wr_gpr(op.r0, op.w, T0);
                        self.a.str(8, T1, XCPU, OFF_CF);
                        self.a.str(8, ZR, XCPU, OFF_ZF);
                        self.a.str(8, ZR, XCPU, OFF_SF);
                        self.a.str(8, ZR, XCPU, OFF_OF);
                        self.a.movz(false, T2, 1, 0);
                        self.a.str(8, T2, XCPU, OFF_PFV); // odd popcount => PF = 0
                        self.pf = super::lazy::PF_NONE;
                        self.fs = FlagState { src: Src::None, host: 0, mem: FALL };
                    }
                    14 => self.emit_fldcw(k, op),
                    15 => self.emit_xgetbv(k),
                    16 => self.emit_xstate(k, op),
                    _ => {}
                }
            }
            K_HI8 => self.emit_hi8_op(k, op),
            K_STRING => self.emit_string_op(k, op),
            K_BIT_EXT => self.emit_bit_ext_op(k, op),
            K_ATOMIC => self.emit_atomic_op(k, op),
            K_PCMPSTR => self.emit_pcmpstr_op(k, op),
            K_VSHA_RR | K_VSHA_RM => {
                let sub = op.x & 0x3F;
                if op.kind == K_VSHA_RM {
                    let a = self.addr_into(op, XA);
                    if Self::chk_of(op) == Chk::Full {
                        let se = self.side_exit_here(k);
                        self.range_check_to(a, se);
                    }
                    self.a.ldr(64, T0, a, 0);
                    self.a.ldr(64, T1, a, 8);
                    if mem::tso() {
                        self.a.dmb_ishld();
                    }
                    self.a.fmov_gf(true, 1, T0);
                    self.a.v_ins_d(1, 1, T1);
                } else {
                    self.xsrc(1, op.r1);
                }
                self.xsrc(0, op.r0);
                match sub {
                    0 => {
                        // Sha1rnds4 xmm1, xmm2/m128, imm8
                        let func = (op.imm as u8) & 3;
                        let k_val: u32 = match func {
                            0 => 0x5A82_7999,
                            1 => 0x6ED9_EBA1,
                            2 => 0x8F1B_BCDC,
                            _ => 0xCA62_C1D6,
                        };
                        self.a.v_rev64_4s(0, 0);
                        self.a.v_ext(0, 0, 0, 8);
                        self.a.v_rev64_4s(1, 1);
                        self.a.v_ext(1, 1, 1, 8);
                        self.a.mov_imm(false, T0, k_val as u64);
                        self.a.v_dup(2, 2, T0);
                        self.a.neon_3same(false, 2, 16, 1, 1, 2); // add v1.4s, v1.4s, v2.4s
                        self.a.v_zero(2);
                        match func {
                            0 => self.a.v_sha1c(0, 2, 1),
                            2 => self.a.v_sha1m(0, 2, 1),
                            _ => self.a.v_sha1p(0, 2, 1),
                        }
                        self.a.v_rev64_4s(0, 0);
                        self.a.v_ext(0, 0, 0, 8);
                    }
                    1 => {
                        // Sha1nexte xmm1, xmm2/m128
                        self.a.v_umov_elem(2, T0, 0, 3);
                        self.a.ror_i(false, T0, T0, 2);
                        self.a.v_umov_elem(2, T1, 1, 3);
                        self.a.addsub_rr(false, false, false, T0, T0, T1, 0);
                        self.a.v_mov(0, 1);
                        self.a.v_ins_s(0, 3, T0);
                    }
                    2 => {
                        // Sha1msg1 xmm1, xmm2/m128
                        self.a.v_ext(2, 1, 0, 8);
                        self.a.v_eor(0, 0, 2);
                    }
                    3 => {
                        // Sha1msg2 xmm1, xmm2/m128
                        self.a.v_rev64_4s(0, 0);
                        self.a.v_ext(0, 0, 0, 8);
                        self.a.v_rev64_4s(1, 1);
                        self.a.v_ext(1, 1, 1, 8);
                        self.a.v_sha1su1(0, 1);
                        self.a.v_rev64_4s(0, 0);
                        self.a.v_ext(0, 0, 0, 8);
                    }
                    4 => {
                        // Sha256rnds2 xmm1, xmm2/m128, <XMM0>
                        self.xsrc(6, 0);
                        self.a.v_rev64_4s(2, 0);
                        self.a.v_rev64_4s(3, 1);
                        self.a.v_zip1(3, 4, 3, 2);
                        self.a.v_zip2(3, 5, 3, 2);
                        self.a.v_mov(3, 5);
                        self.a.v_sha256h(5, 4, 6);
                        self.a.v_sha256h2(4, 3, 6);
                        self.a.v_zip2(3, 0, 4, 5);
                        self.a.v_rev64_4s(0, 0);
                    }
                    5 => {
                        // Sha256msg1 xmm1, xmm2/m128
                        self.a.v_sha256su0(0, 1);
                    }
                    _ => {
                        // Sha256msg2 xmm1, xmm2/m128
                        self.a.v_zero(2);
                        self.a.v_zip2(3, 3, 2, 1);
                        self.a.v_sha256su1(0, 2, 3);
                    }
                }
                self.xwrite(op.r0, 0);
            }
            K_VGFNI_RR | K_VGFNI_RM => {
                let sub = op.x & 0x3F;
                let imm8 = op.imm as u8;
                let is_256 = (op.w & 3) == VM_V256;
                self.vload_b(k, op);
                self.xsrc(0, (op.imm >> 8) as u8);
                self.emit_gfni_lane(sub, imm8, 6);
                if is_256 {
                    let s1 = ((op.imm >> 8) as u32 & 15) * 16;
                    self.a.v_mov(6, 0);
                    self.a.ldr(128, 0, XCPU, OFF_YMM_HI + s1);
                    self.a.v_mov(1, 7);
                    self.emit_gfni_lane(sub, imm8, 7);
                }
                self.vstore_dst(op);
            }
            K_VVNNI_RR | K_VVNNI_RM => {
                let sub = op.x & 0x3F;
                let is_256 = (op.w & 3) == VM_V256;
                self.vload_b(k, op);
                self.xsrc(0, (op.imm >> 8) as u8);
                self.xsrc(5, op.r0);
                self.emit_vnni_lane(sub);
                if is_256 {
                    let d_off = (op.r0 as u32 & 15) * 16;
                    let s1_off = ((op.imm >> 8) as u32 & 15) * 16;
                    self.a.v_mov(6, 0);
                    self.a.ldr(128, 5, XCPU, OFF_YMM_HI + d_off);
                    self.a.ldr(128, 0, XCPU, OFF_YMM_HI + s1_off);
                    self.a.v_mov(1, 7);
                    self.emit_vnni_lane(sub);
                }
                self.vstore_dst(op);
            }
            K_VEXT_RR..=K_MMX_MISC => self.emit_mmx_gather_op(k, op),
            _ => unreachable!("unsupported op kind {}", op.kind),
        }
    }

    pub(super) fn emit_hi8_op(&mut self, k: usize, op: &FastOp) {
        let sub = op.x & 0x3F;
        let la = self.live[k + 1];
        match sub {
            0 => {
                // mov
                let _ = self.rd_hi8_src(k, op, op.r1, T0);
                let dst_a = if op.r0 & 0x40 != 0 {
                    let a = self.addr_into(op, XA);
                    if Self::chk_of(op) == Chk::Full {
                        let se = self.side_exit_here(k);
                        self.range_check_to(a, se);
                    }
                    Some(a)
                } else {
                    None
                };
                self.wr_hi8_dst(op.r0, op.w, T0, dst_a);
            }
            1 => {
                // movzx
                let _ = self.rd_hi8_src(k, op, op.r1, T0);
                self.wr_hi8_dst(op.r0, op.w, T0, None);
            }
            2 => {
                // movsx
                let _ = self.rd_hi8_src(k, op, op.r1, T0);
                self.a.sbfm(true, T0, T0, 0, 7);
                self.wr_hi8_dst(op.r0, op.w, T0, None);
            }
            3..=11 => {
                // add(3), sub(4), and(5), or(6), xor(7), cmp(8), test(9), adc(10), sbb(11)
                let a_sub = sub - 3;
                if matches!(a_sub, 7 | 8) {
                    self.flag_reg(FC, T6);
                }
                let dst_a = self.rd_hi8_src(k, op, op.r0, T3);
                let _ = self.rd_hi8_src(k, op, op.r1, T4);
                let writes = !matches!(a_sub, 5 | 6);
                let need_nzcv = la & FNZCV != 0;
                let need_pf = la & FP != 0;
                if matches!(a_sub, 7 | 8) {
                    let is_sub = a_sub == 8;
                    self.a.addsub_rr(false, is_sub, false, T5, T3, T4, 0);
                    self.a.addsub_rr(false, is_sub, false, T5, T5, T6, 0);
                    self.a.ubfm(false, T2, T5, 0, 7);
                    if need_nzcv {
                        if la & FC != 0 {
                            self.a.ubfx(XS, T5, 8, 1);
                            self.a.str(8, XS, XCPU, OFF_CF);
                        }
                        if la & FZ != 0 {
                            self.a.clz(true, XS, T2);
                            self.a.lsr_i(true, XS, XS, 6);
                            self.a.str(8, XS, XCPU, OFF_ZF);
                        }
                        if la & FS != 0 {
                            self.a.ubfx(XS, T2, 7, 1);
                            self.a.str(8, XS, XCPU, OFF_SF);
                        }
                        if la & FO != 0 {
                            if is_sub {
                                self.a.logic_rr(false, 2, T4, T3, T4, 0, 0);
                                self.a.logic_rr(false, 2, T3, T3, T2, 0, 0);
                            } else {
                                self.a.logic_rr(false, 2, T3, T3, T2, 0, 0);
                                self.a.logic_rr(false, 2, T4, T4, T2, 0, 0);
                            }
                            self.a.logic_rr(false, 0, T3, T3, T4, 0, 0);
                            self.a.ubfx(XS, T3, 7, 1);
                            self.a.str(8, XS, XCPU, OFF_OF);
                        }
                    }
                    if writes {
                        self.wr_hi8_dst(op.r0, 8, T2, dst_a);
                    }
                    if need_pf {
                        self.a.str(8, T2, XCPU, OFF_PFV);
                    }
                    self.fs = FlagState {
                        src: Src::None,
                        host: 0,
                        mem: (self.fs.mem & !FALL) | (la & FALL),
                    };
                } else {
                    self.a.lsl_i(true, T3, T3, 56);
                    self.a.lsl_i(true, T4, T4, 56);
                    match a_sub {
                        0 => self.a.addsub_rr(true, false, true, T5, T3, T4, 0),
                        1 | 5 => self.a.addsub_rr(true, true, true, T5, T3, T4, 0),
                        2 | 6 => self.a.logic_rr(true, 3, T5, T3, T4, 0, 0),
                        3 => {
                            self.a.logic_rr(true, 1, T5, T3, T4, 0, 0);
                            self.a.logic_rr(true, 3, ZR, T5, T5, 0, 0);
                        }
                        _ => {
                            self.a.logic_rr(true, 2, T5, T3, T4, 0, 0);
                            self.a.logic_rr(true, 3, ZR, T5, T5, 0, 0);
                        }
                    }
                    self.a.lsr_i(true, T2, T5, 56);
                    if writes {
                        self.wr_hi8_dst(op.r0, 8, T2, dst_a);
                    }
                    if need_pf {
                        self.a.str(8, T2, XCPU, OFF_PFV);
                    }
                    self.fs.mem &= !FALL;
                    if need_pf {
                        self.fs.mem |= FP;
                    }
                    self.fs.host = if need_nzcv { la & FNZCV } else { 0 };
                    self.fs.src = match a_sub {
                        0 => Src::Add,
                        1 | 5 => Src::Sub,
                        _ => Src::Logic,
                    };
                }
            }
            12 | 13 => {
                // inc / dec
                let dec = sub == 13;
                if la & FC != 0 && self.fs.host & FC != 0 && self.fs.mem & FC == 0 {
                    self.flush_flags(FC);
                }
                let _ = self.rd_hi8_src(k, op, op.r0, T3);
                self.a.lsl_i(true, T3, T3, 56);
                self.a.mov_imm(true, T4, 1u64 << 56);
                self.a.addsub_rr(true, dec, true, T5, T3, T4, 0);
                self.a.lsr_i(true, T2, T5, 56);
                self.wr_hi8_dst(op.r0, 8, T2, None);
                if la & FP != 0 {
                    self.a.str(8, T2, XCPU, OFF_PFV);
                }
                self.fs.mem &= !(FZ | FS | FO | FP);
                if la & FP != 0 {
                    self.fs.mem |= FP;
                }
                self.fs.host = la & (FZ | FS | FO);
                self.fs.src = Src::IncDec;
            }
            14 => {
                // not
                let _ = self.rd_hi8_src(k, op, op.r0, T0);
                self.a.emit(0x2A20_03E0 | T0 << 16 | 31 << 5 | T0);
                self.wr_hi8_dst(op.r0, 8, T0, None);
            }
            15 => {
                // neg
                let _ = self.rd_hi8_src(k, op, op.r0, T3);
                self.a.lsl_i(true, T3, T3, 56);
                self.a.addsub_rr(true, true, true, T5, ZR, T3, 0);
                self.a.lsr_i(true, T2, T5, 56);
                self.wr_hi8_dst(op.r0, 8, T2, None);
                if la & FP != 0 {
                    self.a.str(8, T2, XCPU, OFF_PFV);
                }
                self.fs.mem &= !FALL;
                if la & FP != 0 {
                    self.fs.mem |= FP;
                }
                self.fs.host = la & FNZCV;
                self.fs.src = Src::Sub;
            }
            16..=20 => {
                // shl(16), shr(17), sar(18), rol(19), ror(20)
                self.flush_flags(la);
                let _ = self.rd_hi8_src(k, op, op.r0, T0);
                if op.r1 & 0x20 != 0 {
                    self.a.mov_imm(false, T6, op.imm & 31);
                } else {
                    self.a.logic_ri(false, 0, T6, g(1), enc_logic_imm(31, false).unwrap());
                }
                let skip = self.a.cbz(false, T6);
                if sub <= 18 {
                    let mut of_zero = false;
                    match sub {
                        16 => {
                            self.a.shiftv(true, 8, T4, T0, T6);
                            self.a.ubfx(T2, T4, 8, 1);
                            self.a.ubfm(true, T1, T4, 0, 7);
                            self.a.ubfx(T3, T1, 7, 1);
                            self.a.logic_rr(true, 2, T3, T3, T2, 0, 0);
                        }
                        17 => {
                            self.a.shiftv(true, 9, T1, T0, T6);
                            self.a.addsub_ri(true, true, false, T4, T6, 1, false);
                            self.a.shiftv(true, 9, T2, T0, T4);
                            self.a.ubfx(T2, T2, 0, 1);
                            self.a.ubfx(T3, T0, 7, 1);
                        }
                        _ => {
                            self.a.sbfm(true, XS, T0, 0, 7);
                            self.a.shiftv(true, 10, T1, XS, T6);
                            self.a.ubfm(true, T1, T1, 0, 7);
                            self.a.addsub_ri(true, true, false, T4, T6, 1, false);
                            self.a.shiftv(true, 10, T2, XS, T4);
                            self.a.ubfx(T2, T2, 0, 1);
                            of_zero = true;
                        }
                    }
                    self.shift_flags(la, 8, of_zero);
                } else {
                    let ror = sub == 20;
                    if ror {
                        self.a.logic_ri(false, 0, T4, T6, enc_logic_imm(7, false).unwrap());
                    } else {
                        self.a.addsub_rr(false, true, false, T4, ZR, T6, 0);
                        self.a.logic_ri(false, 0, T4, T4, enc_logic_imm(7, false).unwrap());
                    }
                    self.a.lsl_i(false, T2, T0, 8);
                    self.a.logic_rr(false, 1, T2, T0, T2, 0, 0);
                    self.a.shiftv(false, 9, T1, T2, T4);
                    self.a.ubfm(false, T1, T1, 0, 7);
                    self.a.ubfx(T2, T1, 7, 1);
                    if ror {
                        self.a.ubfx(T3, T1, 6, 1);
                        self.a.logic_rr(false, 2, T3, T2, T3, 0, 0);
                        self.a.str(8, T2, XCPU, OFF_CF);
                        self.a.str(8, T3, XCPU, OFF_OF);
                    } else {
                        self.a.ubfx(T4, T1, 0, 1);
                        self.a.logic_rr(false, 2, T3, T2, T4, 0, 0);
                        self.a.str(8, T4, XCPU, OFF_CF);
                        self.a.str(8, T3, XCPU, OFF_OF);
                    }
                }
                self.wr_hi8_dst(op.r0, 8, T1, None);
                self.a.bind(skip);
                self.fs = FlagState {
                    src: Src::None,
                    host: 0,
                    mem: (self.fs.mem & !FALL) | (la & FALL),
                };
            }
            21 => {
                // setcc
                let c = self.cond((op.imm as u8) & 0x3F);
                match c {
                    Cond::Host(cnd) => self.a.cset(T0, cnd),
                    Cond::Reg(r, inv) => {
                        if inv {
                            self.a.logic_ri(false, 2, T0, r, enc_logic_imm(1, false).unwrap());
                        } else {
                            self.a.mov_rr(false, T0, r);
                        }
                    }
                    Cond::Const(b) => self.a.movz(false, T0, b as u32, 0),
                }
                self.wr_hi8_dst(op.r0, 8, T0, None);
            }
            22 => {
                // xchg
                let _ = self.rd_hi8_src(k, op, op.r0, T0);
                let _ = self.rd_hi8_src(k, op, op.r1, T1);
                self.wr_hi8_dst(op.r0, 8, T1, None);
                self.wr_hi8_dst(op.r1, 8, T0, None);
            }
            23 => {
                // xadd
                let _ = self.rd_hi8_src(k, op, op.r0, T0);
                let _ = self.rd_hi8_src(k, op, op.r1, T1);
                self.a.lsl_i(true, T3, T0, 56);
                self.a.lsl_i(true, T4, T1, 56);
                self.a.addsub_rr(true, false, true, T5, T3, T4, 0);
                self.a.lsr_i(true, T2, T5, 56);
                self.wr_hi8_dst(op.r0, 8, T2, None);
                self.wr_hi8_dst(op.r1, 8, T0, None);
                if la & FP != 0 {
                    self.a.str(8, T2, XCPU, OFF_PFV);
                }
                self.fs.mem &= !FALL;
                if la & FP != 0 {
                    self.fs.mem |= FP;
                }
                self.fs.host = la & FNZCV;
                self.fs.src = Src::Add;
            }
            _ => {
                // cmpxchg
                self.a.ubfm(false, T0, g(0), 0, 7); // AL
                let _ = self.rd_hi8_src(k, op, op.r0, T1);
                let _ = self.rd_hi8_src(k, op, op.r1, T6);
                self.a.lsl_i(true, T3, T0, 56);
                self.a.lsl_i(true, T4, T1, 56);
                self.a.addsub_rr(true, true, true, T5, T3, T4, 0);
                self.a.lsr_i(true, T2, T5, 56);
                let neq = self.a.bcond(NE);
                self.wr_hi8_dst(op.r0, 8, T6, None);
                let done = self.a.b();
                self.a.bind(neq);
                self.wr_gpr(0, 8, T1);
                self.a.bind(done);
                if la & FP != 0 {
                    self.a.str(8, T2, XCPU, OFF_PFV);
                }
                self.fs.mem &= !FALL;
                if la & FP != 0 {
                    self.fs.mem |= FP;
                }
                self.fs.host = la & FNZCV;
                self.fs.src = Src::Sub;
            }
        }
    }

    pub(super) fn emit_string_op(&mut self, k: usize, op: &FastOp) {
        let wb = op.w as u32;
        let step_bytes = wb / 8;
        let op_k = op.x & 7;
        let rep_mode = (op.x >> 3) & 3;
        let la = self.live[k + 1];
        if op_k >= 3 && rep_mode != 0 {
            self.flush_flags(la);
        }
        // T4 = signed step (+step_bytes if DF==0 else -step_bytes)
        self.a.movz(true, T4, step_bytes, 0);
        self.a.ldr(8, T5, XCPU, OFF_DF);
        let df_clear = self.a.cbz(false, T5);
        self.a.addsub_rr(true, true, false, T4, ZR, T4, 0);
        self.a.bind(df_clear);

        let skip_rep = if rep_mode != 0 {
            Some(self.a.cbz(true, g(1)))
        } else {
            None
        };
        let loop_top = self.a.pos;
        match op_k {
            0 => {
                // movs
                self.load(k, T0, g(6), wb, Chk::Full);
                self.store(k, T0, g(7), wb, Chk::Full);
                self.a.addsub_rr(true, false, false, g(6), g(6), T4, 0);
                self.a.addsub_rr(true, false, false, g(7), g(7), T4, 0);
            }
            1 => {
                // stos
                self.store(k, g(0), g(7), wb, Chk::Full);
                self.a.addsub_rr(true, false, false, g(7), g(7), T4, 0);
            }
            2 => {
                // lods
                self.load(k, T0, g(6), wb, Chk::Full);
                self.wr_gpr(0, op.w, T0);
                self.a.addsub_rr(true, false, false, g(6), g(6), T4, 0);
            }
            3 => {
                // scas: acc - [rdi]
                self.load(k, T1, g(7), wb, Chk::Full);
                self.a.addsub_rr(true, false, false, g(7), g(7), T4, 0);
                let sh = 64 - wb;
                if wb == 64 {
                    self.a.addsub_rr(true, true, true, T2, g(0), T1, 0);
                } else if wb == 32 {
                    self.a.addsub_rr(false, true, true, T2, g(0), T1, 0);
                } else {
                    self.a.lsl_i(true, T0, g(0), sh);
                    self.a.lsl_i(true, T1, T1, sh);
                    self.a.addsub_rr(true, true, true, T3, T0, T1, 0);
                    self.a.lsr_i(true, T2, T3, sh);
                }
                if la & FP != 0 {
                    self.a.str(8, T2, XCPU, OFF_PFV);
                }
                self.fs.mem &= !FALL;
                if la & FP != 0 {
                    self.fs.mem |= FP;
                }
                self.fs.host = la & FNZCV;
                self.fs.src = Src::Sub;
                if rep_mode != 0 {
                    self.flush_flags(la);
                }
            }
            _ => {
                // cmps: [rsi] - [rdi]
                self.load(k, T0, g(6), wb, Chk::Full);
                self.load(k, T1, g(7), wb, Chk::Full);
                self.a.addsub_rr(true, false, false, g(6), g(6), T4, 0);
                self.a.addsub_rr(true, false, false, g(7), g(7), T4, 0);
                let sh = 64 - wb;
                if wb == 64 {
                    self.a.addsub_rr(true, true, true, T2, T0, T1, 0);
                } else if wb == 32 {
                    self.a.addsub_rr(false, true, true, T2, T0, T1, 0);
                } else {
                    self.a.lsl_i(true, T0, T0, sh);
                    self.a.lsl_i(true, T1, T1, sh);
                    self.a.addsub_rr(true, true, true, T3, T0, T1, 0);
                    self.a.lsr_i(true, T2, T3, sh);
                }
                if la & FP != 0 {
                    self.a.str(8, T2, XCPU, OFF_PFV);
                }
                self.fs.mem &= !FALL;
                if la & FP != 0 {
                    self.fs.mem |= FP;
                }
                self.fs.host = la & FNZCV;
                self.fs.src = Src::Sub;
                if rep_mode != 0 {
                    self.flush_flags(la);
                }
            }
        }
        if rep_mode != 0 {
            self.a.addsub_ri(true, true, false, g(1), g(1), 1, false);
            let done_rcx = self.a.cbz(true, g(1));
            if op_k >= 3 {
                // repe (rep_mode==1) continues while EQ; repne (rep_mode==2) continues while NE.
                let cont_cond = if rep_mode == 1 { EQ } else { NE };
                let br = self.a.bcond(cont_cond);
                self.a.patch(br, loop_top);
            } else {
                let br = self.a.b();
                self.a.patch(br, loop_top);
            }
            self.a.bind(done_rcx);
            self.set_lb(1, 64);
        }
        if let Some(s) = skip_rep {
            self.a.bind(s);
        }
    }

    pub(super) fn emit_bit_ext_op(&mut self, k: usize, op: &FastOp) {
        let wb = op.w as u32;
        let sub = op.x & 0x0F;
        let mem = (op.x & 0x20) != 0;
        let imm_cnt = (op.x & 0x10) != 0;
        let la = self.live[k + 1];
        if sub <= 1 {
            // Pdep (0) / Pext (1)
            let x = wb == 64;
            self.a.mov_rr(x, T0, g(op.imm as u8));
            if mem {
                let a = self.addr_into(op, XA);
                self.load(k, T1, a, wb, Self::chk_of(op));
            } else {
                self.a.mov_rr(x, T1, g(op.r1));
            }
            self.a.movz(true, T2, 0, 0); // res = 0
            if sub == 0 {
                let done = self.a.cbz(true, T1);
                let loop_top = self.a.pos;
                self.a.addsub_rr(true, true, false, T3, ZR, T1, 0);
                self.a.logic_rr(true, 0, T3, T1, T3, 0, 0); // T3 = m & -m
                self.a.ubfx(T4, T0, 0, 1);
                self.a.addsub_rr(true, true, false, T4, ZR, T4, 0);
                self.a.logic_rr(true, 0, T4, T3, T4, 0, 0);
                self.a.logic_rr(true, 1, T2, T2, T4, 0, 0);
                self.a.lsr_i(true, T0, T0, 1);
                self.a.bic_rr(true, T1, T1, T3);
                let back = self.a.cbnz(true, T1);
                self.a.patch(back, loop_top);
                self.a.bind(done);
            } else {
                self.a.movz(true, T5, 1, 0); // out_bit = 1
                let done = self.a.cbz(true, T1);
                let loop_top = self.a.pos;
                self.a.addsub_rr(true, true, false, T3, ZR, T1, 0);
                self.a.logic_rr(true, 0, T3, T1, T3, 0, 0);
                self.a.logic_rr(true, 0, T4, T0, T3, 0, 0);
                self.a.clz(true, T4, T4);
                self.a.lsr_i(true, T4, T4, 6);
                self.a.logic_ri(false, 2, T4, T4, enc_logic_imm(1, false).unwrap());
                self.a.addsub_rr(true, true, false, T4, ZR, T4, 0);
                self.a.logic_rr(true, 0, T4, T5, T4, 0, 0);
                self.a.logic_rr(true, 1, T2, T2, T4, 0, 0);
                self.a.lsl_i(true, T5, T5, 1);
                self.a.bic_rr(true, T1, T1, T3);
                let back = self.a.cbnz(true, T1);
                self.a.patch(back, loop_top);
                self.a.bind(done);
            }
            self.wr_gpr(op.r0, op.w, T2);
        } else {
            // Rcl (2) / Rcr (3)
            self.flush_flags(la | FC);
            let rcl = sub == 2;
            let addr = if mem {
                let a = self.addr_into(op, XA);
                self.load(k, T0, a, wb, Self::chk_of(op));
                Some(a)
            } else {
                let u = self.ua(op.r0, wb);
                if u != T0 {
                    self.a.mov_rr(true, T0, u);
                }
                None
            };
            let mask = if wb == 64 { 63u64 } else { 31u64 };
            if imm_cnt {
                self.a.mov_imm(false, T6, op.imm & mask);
            } else {
                self.a.logic_ri(false, 0, T6, g(1), enc_logic_imm(mask, false).unwrap());
            }
            let skip = self.a.cbz(false, T6);
            if wb == 8 || wb == 16 {
                let m = wb + 1;
                self.a.movz(false, T2, m, 0);
                self.a.udiv(false, T3, T6, T2);
                self.a.msub(false, T6, T3, T2, T6);
            }
            self.a.ldr(8, T1, XCPU, OFF_CF);
            let done_loop = self.a.cbz(false, T6);
            let loop_top = self.a.pos;
            if rcl {
                self.a.ubfx(T2, T0, wb - 1, 1);
                self.a.lsl_i(true, T0, T0, 1);
                self.a.logic_rr(true, 1, T0, T0, T1, 0, 0);
                if wb < 64 {
                    self.a.ubfm(true, T0, T0, 0, wb - 1);
                }
                self.a.mov_rr(false, T1, T2);
            } else {
                self.a.ubfx(T2, T0, 0, 1);
                self.a.lsr_i(true, T0, T0, 1);
                self.a.lsl_i(true, T3, T1, wb - 1);
                self.a.logic_rr(true, 1, T0, T0, T3, 0, 0);
                self.a.mov_rr(false, T1, T2);
            }
            self.a.addsub_ri(false, true, false, T6, T6, 1, false);
            let back = self.a.cbnz(false, T6);
            self.a.patch(back, loop_top);
            self.a.bind(done_loop);
            if let Some(a) = addr {
                self.store_checked_addr(T0, a, wb);
            } else {
                self.wr_gpr(op.r0, op.w, T0);
            }
            self.a.str(8, T1, XCPU, OFF_CF);
            if la & FO != 0 {
                self.a.ubfx(T2, T0, wb - 1, 1);
                if rcl {
                    self.a.logic_rr(false, 2, T3, T2, T1, 0, 0);
                } else {
                    self.a.ubfx(T4, T0, wb - 2, 1);
                    self.a.logic_rr(false, 2, T3, T2, T4, 0, 0);
                }
                self.a.str(8, T3, XCPU, OFF_OF);
            }
            self.a.bind(skip);
            self.fs = FlagState {
                src: Src::None,
                host: 0,
                mem: (self.fs.mem & !FALL) | (la & FALL),
            };
        }
    }

    /// Branch to the 16-byte-boundary-crossing slow path iff `[XA .. XA + bytes)` crosses a
    /// 16-byte granule boundary. Preserves NZCV; clobbers only `XS`. Returns `None` for 1-byte
    /// accesses, which can never cross a 16-byte boundary.
    fn emit_cross16_branch(&mut self, bytes: u32) -> Option<usize> {
        match bytes {
            1 => None,
            n if n == 16 || !crate::emu::features::host().has(crate::emu::features::HostFeatures::LSE2) => {
                self.a.logic_ri(false, 0, XS, XA, enc_logic_imm((n - 1) as u64, false).unwrap());
                Some(self.a.cbnz(false, XS))
            }
            _ => {
                self.a.logic_ri(false, 0, XS, XA, enc_logic_imm(15, false).unwrap());
                self.a.addsub_ri(false, false, false, XS, XS, bytes - 1, false);
                self.a.lsr_i(false, XS, XS, 4);
                Some(self.a.cbnz(false, XS))
            }
        }
    }

    /// Acquire the sharded spinlock for 16-byte-crossing LOCK ops starting in granule `XA >> 4`.
    /// Writes the lock pointer into `lock_reg` (caller unlocks via `self.a.stlr(32, ZR, lock_reg)`).
    /// Clobbers only `lock_reg`, `tmp1`, `tmp2`; preserves NZCV.
    fn emit_shard_lock_acquire(&mut self, lock_reg: u32, tmp1: u32, tmp2: u32) {
        self.a.ubfx(tmp1, XA, 4, crate::emu::cpu::ATOMIC_LOCK_SHARD_BITS);
        self.a.ldr(64, lock_reg, XCPU, OFF_ATOMIC_LOCKS);
        self.a.addsub_rr(
            true,
            false,
            false,
            lock_reg,
            lock_reg,
            tmp1,
            crate::emu::cpu::ATOMIC_LOCK_STRIDE_SHIFT,
        );
        self.a.movz(false, tmp2, 1, 0);
        let try_lock = self.a.pos;
        self.a.swpal(32, tmp2, tmp1, lock_reg);
        let got = self.a.cbz(false, tmp1);
        let spin = self.a.pos;
        self.a.yield_hint();
        self.a.ldr(32, tmp1, lock_reg, 0);
        let still_locked = self.a.cbnz(false, tmp1);
        self.a.patch(still_locked, spin);
        let retry = self.a.b();
        self.a.patch(retry, try_lock);
        self.a.bind(got);
    }

    pub(super) fn emit_atomic_op(&mut self, k: usize, op: &FastOp) {
        let sub = op.x & 0x3F;
        let wb = op.w as u32;
        let la = self.live[k + 1];
        let a = self.addr_into(op, XA);
        if a != XA {
            self.a.mov_rr(true, XA, a);
        }
        if Self::chk_of(op) == Chk::Full {
            let se = self.side_exit_here(k);
            self.range_check_to(XA, se);
        }
        match sub {
            0..=8 | 13 | 14 => {
                // 0..4: add/sub/and/or/xor, 5..6: inc/dec, 7..8: not/neg, 13: xchg, 14: xadd
                if matches!(sub, 5 | 6) && la & FC != 0 && self.fs.host & FC != 0 && self.fs.mem & FC == 0 {
                    self.flush_flags(FC);
                }
                if op.r0 != 0 {
                    self.a.mov_imm(true, T1, op.imm);
                } else if matches!(sub, 0..=4 | 13 | 14) {
                    self.a.mov_rr(true, T1, g(op.r1));
                }
                let x = wb == 64;
                let cross = self.emit_cross16_branch(wb / 8);
                // Fast path: single-instruction LSE atomics (FEAT_LSE2 supports misaligned
                // addresses inside a 16-byte granule with single-copy atomicity).
                match sub {
                    0 | 14 => self.a.ldaddal(wb, T1, T0, XA),
                    1 => {
                        self.a.addsub_rr(x, true, false, T2, ZR, T1, 0);
                        self.a.ldaddal(wb, T2, T0, XA);
                    }
                    2 => {
                        self.a.emit(sf(x) | 0x2A20_03E0 | T1 << 16 | 31 << 5 | T2);
                        self.a.ldclral(wb, T2, T0, XA);
                    }
                    3 => self.a.ldsetal(wb, T1, T0, XA),
                    4 => self.a.ldeoral(wb, T1, T0, XA),
                    5 => {
                        self.a.movz(x, T2, 1, 0);
                        self.a.ldaddal(wb, T2, T0, XA);
                    }
                    6 => {
                        self.a.movn(x, T2, 0, 0);
                        self.a.ldaddal(wb, T2, T0, XA);
                    }
                    7 => {
                        self.a.movn(x, T2, 0, 0);
                        self.a.ldeoral(wb, T2, T0, XA);
                    }
                    8 => {
                        self.a.ldr(wb, T0, XA, 0);
                        let retry = self.a.pos;
                        self.a.mov_rr(x, T3, T0);
                        self.a.addsub_rr(x, true, false, T2, ZR, T0, 0);
                        self.a.casal(wb, T0, T2, XA);
                        self.a.addsub_rr(x, true, true, ZR, T0, T3, 0);
                        let fail = self.a.bcond(NE);
                        self.a.patch(fail, retry);
                    }
                    _ => self.a.swpal(wb, T1, T0, XA), // 13: xchg
                }
                if let Some(cross) = cross {
                    let done = self.a.b();
                    self.a.bind(cross);
                    self.emit_shard_lock_acquire(XS, T5, T6);
                    self.a.ldr(wb, T0, XA, 0);
                    match sub {
                        0 | 14 => self.a.addsub_rr(x, false, false, T2, T0, T1, 0),
                        1 => self.a.addsub_rr(x, true, false, T2, T0, T1, 0),
                        2 => self.a.logic_rr(x, 0, T2, T0, T1, 0, 0),
                        3 => self.a.logic_rr(x, 1, T2, T0, T1, 0, 0),
                        4 => self.a.logic_rr(x, 2, T2, T0, T1, 0, 0),
                        5 => self.a.addsub_ri(x, false, false, T2, T0, 1, false),
                        6 => self.a.addsub_ri(x, true, false, T2, T0, 1, false),
                        7 => self.a.emit(sf(x) | 0x2A20_03E0 | T0 << 16 | 31 << 5 | T2),
                        8 => self.a.addsub_rr(x, true, false, T2, ZR, T0, 0),
                        _ => self.a.mov_rr(x, T2, T1),
                    }
                    self.a.str(wb, T2, XA, 0);
                    self.a.stlr(32, ZR, XS);
                    self.a.bind(done);
                }
                self.a.dmb_ish();
                if matches!(sub, 13 | 14) {
                    self.wr_gpr(op.r1, op.w, T0);
                }
                if !matches!(sub, 7 | 13) {
                    if sub == 3 {
                        self.a.logic_rr(x, 1, T2, T0, T1, 0, 0);
                    } else if sub == 4 {
                        self.a.logic_rr(x, 2, T2, T0, T1, 0, 0);
                    }
                    let sh = 64 - wb;
                    if wb >= 32 {
                        match sub {
                            0 | 14 => self.a.addsub_rr(x, false, true, T5, T0, T1, 0),
                            1 => self.a.addsub_rr(x, true, true, T5, T0, T1, 0),
                            2 => self.a.logic_rr(x, 3, T5, T0, T1, 0, 0),
                            3 | 4 => self.a.logic_rr(x, 3, ZR, T2, T2, 0, 0),
                            5 => self.a.addsub_ri(x, false, true, T5, T0, 1, false),
                            6 => self.a.addsub_ri(x, true, true, T5, T0, 1, false),
                            _ => self.a.addsub_rr(x, true, true, T5, ZR, T0, 0),
                        }
                    } else {
                        self.a.lsl_i(true, T3, T0, sh);
                        match sub {
                            0 | 1 | 2 | 14 => {
                                self.a.lsl_i(true, T4, T1, sh);
                                if sub == 0 || sub == 14 {
                                    self.a.addsub_rr(true, false, true, T5, T3, T4, 0);
                                } else if sub == 1 {
                                    self.a.addsub_rr(true, true, true, T5, T3, T4, 0);
                                } else {
                                    self.a.logic_rr(true, 3, T5, T3, T4, 0, 0);
                                }
                            }
                            3 | 4 => {
                                self.a.lsl_i(true, T5, T2, sh);
                                self.a.logic_rr(true, 3, ZR, T5, T5, 0, 0);
                            }
                            5 | 6 => {
                                self.a.mov_imm(true, T4, 1u64 << sh);
                                self.a.addsub_rr(true, sub == 6, true, T5, T3, T4, 0);
                            }
                            _ => self.a.addsub_rr(true, true, true, T5, ZR, T3, 0),
                        }
                    }
                    if la & FP != 0 {
                        if matches!(sub, 3 | 4) {
                            self.a.str(8, T2, XCPU, OFF_PFV);
                        } else if wb >= 32 {
                            self.a.str(8, T5, XCPU, OFF_PFV);
                        } else {
                            self.a.lsr_i(true, T2, T5, sh);
                            self.a.str(8, T2, XCPU, OFF_PFV);
                        }
                    }
                    if matches!(sub, 5 | 6) {
                        self.fs.mem &= !(FZ | FS | FO | FP);
                        if la & FP != 0 {
                            self.fs.mem |= FP;
                        }
                        self.fs.host = la & (FZ | FS | FO);
                        self.fs.src = Src::IncDec;
                    } else {
                        self.fs.mem &= !FALL;
                        if la & FP != 0 {
                            self.fs.mem |= FP;
                        }
                        self.fs.host = la & FNZCV;
                        self.fs.src = match sub {
                            0 | 14 => Src::Add,
                            1 | 8 => Src::Sub,
                            _ => Src::Logic,
                        };
                    }
                }
            }
            9..=12 => {
                // Memory bt(9), bts(10), btr(11), btc(12)
                if op.r0 != 0 {
                    // Immediate bit index
                    let b = (op.imm as u32) & (wb - 1);
                    self.a.movz(false, T1, b, 0);
                } else {
                    // Register bit offset: signed offset across memory elements
                    self.a.sbfm(true, T2, g(op.r1), 0, wb - 1);
                    self.a.logic_ri(false, 0, T1, T2, enc_logic_imm((wb - 1) as u64, false).unwrap());
                    let sh = wb.trailing_zeros();
                    self.a.asr_i(true, T2, T2, sh);
                    let elem_sh = (wb / 8).trailing_zeros();
                    self.a.addsub_rr(true, false, false, XA, XA, T2, elem_sh);
                    if Self::chk_of(op) == Chk::Full {
                        let se = self.side_exit_here(k);
                        self.range_check_to(XA, se);
                    }
                }
                if sub == 9 {
                    self.load(k, T0, XA, wb, Chk::Align);
                    self.a.shiftv(true, 9, T3, T0, T1);
                    self.a.ubfx(T3, T3, 0, 1);
                    self.a.str(8, T3, XCPU, OFF_CF);
                } else {
                    self.a.movz(true, T4, 1, 0);
                    self.a.shiftv(true, 8, T4, T4, T1);
                    let cross = self.emit_cross16_branch(wb / 8);
                    match sub {
                        10 => self.a.ldsetal(wb, T4, T0, XA),
                        11 => self.a.ldclral(wb, T4, T0, XA),
                        _ => self.a.ldeoral(wb, T4, T0, XA),
                    }
                    if let Some(cross) = cross {
                        let done = self.a.b();
                        self.a.bind(cross);
                        self.emit_shard_lock_acquire(XS, T5, T6);
                        self.a.ldr(wb, T0, XA, 0);
                        match sub {
                            10 => self.a.logic_rr(true, 1, T2, T0, T4, 0, 0),
                            11 => self.a.bic_rr(true, T2, T0, T4),
                            _ => self.a.logic_rr(true, 2, T2, T0, T4, 0, 0),
                        }
                        self.a.str(wb, T2, XA, 0);
                        self.a.stlr(32, ZR, XS);
                        self.a.bind(done);
                    }
                    self.a.dmb_ish();
                    self.a.shiftv(true, 9, T3, T0, T1);
                    self.a.ubfx(T3, T3, 0, 1);
                    self.a.str(8, T3, XCPU, OFF_CF);
                }
                self.fs.host &= !FC;
                self.fs.mem |= FC;
            }
            15 => {
                // cmpxchg [mem], r1
                self.a.mov_rr(true, T1, g(op.r1));
                let cross = self.emit_cross16_branch(wb / 8);
                self.a.mov_rr(wb == 64, T0, g(0));
                self.a.casal(wb, T0, T1, XA);
                if let Some(cross) = cross {
                    let done = self.a.b();
                    self.a.bind(cross);
                    self.emit_shard_lock_acquire(XS, T5, T6);
                    self.a.ldr(wb, T0, XA, 0);
                    if wb == 64 {
                        self.a.addsub_rr(true, true, true, ZR, g(0), T0, 0);
                    } else if wb == 32 {
                        self.a.addsub_rr(false, true, true, ZR, g(0), T0, 0);
                    } else {
                        self.a.ubfm(false, T3, g(0), 0, wb - 1);
                        self.a.addsub_rr(false, true, true, ZR, T3, T0, 0);
                    }
                    let no_store = self.a.bcond(NE);
                    self.a.str(wb, T1, XA, 0);
                    self.a.bind(no_store);
                    self.a.stlr(32, ZR, XS);
                    self.a.bind(done);
                }
                self.a.dmb_ish();
                let sh = 64 - wb;
                if wb == 64 {
                    self.a.addsub_rr(true, true, true, T2, g(0), T0, 0);
                } else if wb == 32 {
                    self.a.addsub_rr(false, true, true, T2, g(0), T0, 0);
                } else {
                    self.a.lsl_i(true, T3, g(0), sh);
                    self.a.lsl_i(true, T4, T0, sh);
                    self.a.addsub_rr(true, true, true, T5, T3, T4, 0);
                    self.a.lsr_i(true, T2, T5, sh);
                }
                let matched = self.a.bcond(EQ);
                self.wr_gpr(0, op.w, T0);
                self.a.bind(matched);
                if la & FP != 0 {
                    self.a.str(8, T2, XCPU, OFF_PFV);
                }
                self.fs.mem &= !FALL;
                if la & FP != 0 {
                    self.fs.mem |= FP;
                }
                self.fs.host = la & FNZCV;
                self.fs.src = Src::Sub;
            }
            16 => {
                // cmpxchg8b [mem]
                self.a.ubfm(true, T0, g(0), 0, 31);
                self.a.lsl_i(true, T1, g(2), 32);
                self.a.logic_rr(true, 1, T0, T0, T1, 0, 0); // expected = edx:eax
                self.a.ubfm(true, T1, g(3), 0, 31);
                self.a.lsl_i(true, T2, g(1), 32);
                self.a.logic_rr(true, 1, T1, T1, T2, 0, 0); // desired = ecx:ebx
                let cross = self.emit_cross16_branch(8).unwrap();
                self.a.mov_rr(true, T2, T0);
                self.a.casal(64, T2, T1, XA);
                let done = self.a.b();
                self.a.bind(cross);
                self.emit_shard_lock_acquire(XS, T5, T6);
                self.a.ldr(64, T2, XA, 0);
                self.a.logic_rr(true, 2, T3, T2, T0, 0, 0);
                let no_store = self.a.cbnz(true, T3);
                self.a.str(64, T1, XA, 0);
                self.a.bind(no_store);
                self.a.stlr(32, ZR, XS);
                self.a.bind(done);
                self.a.dmb_ish();
                self.a.logic_rr(true, 2, T3, T2, T0, 0, 0);
                let mismatch = self.a.cbnz(true, T3);
                self.a.movz(false, T4, 1, 0);
                self.a.str(8, T4, XCPU, OFF_ZF);
                let after = self.a.b();
                self.a.bind(mismatch);
                self.a.mov_rr(false, g(0), T2);
                self.a.lsr_i(true, g(2), T2, 32);
                self.set_lb(0, 32);
                self.set_lb(2, 32);
                self.a.str(8, ZR, XCPU, OFF_ZF);
                self.a.bind(after);
                self.fs.host &= !FZ;
                self.fs.mem |= FZ;
            }
            _ => {
                // cmpxchg16b [mem]
                let cross = self.emit_cross16_branch(16).unwrap();
                self.a.mov_rr(true, T0, g(0));
                self.a.mov_rr(true, T1, g(2));
                self.a.mov_rr(true, T2, g(3));
                self.a.mov_rr(true, T3, g(1));
                self.a.caspal_x(T0, T2, XA);
                let done = self.a.b();
                self.a.bind(cross);
                self.emit_shard_lock_acquire(XS, T5, T6);
                self.a.ldr(64, T0, XA, 0);
                self.a.ldr(64, T1, XA, 8);
                self.a.logic_rr(true, 2, T2, T0, g(0), 0, 0);
                self.a.logic_rr(true, 2, T3, T1, g(2), 0, 0);
                self.a.logic_rr(true, 1, T2, T2, T3, 0, 0);
                let no_store = self.a.cbnz(true, T2);
                self.a.str(64, g(3), XA, 0);
                self.a.str(64, g(1), XA, 8);
                self.a.bind(no_store);
                self.a.stlr(32, ZR, XS);
                self.a.bind(done);
                self.a.dmb_ish();
                self.a.logic_rr(true, 2, T2, T0, g(0), 0, 0);
                self.a.logic_rr(true, 2, T3, T1, g(2), 0, 0);
                self.a.logic_rr(true, 1, T2, T2, T3, 0, 0);
                let mismatch = self.a.cbnz(true, T2);
                self.a.movz(false, T4, 1, 0);
                self.a.str(8, T4, XCPU, OFF_ZF);
                let after = self.a.b();
                self.a.bind(mismatch);
                self.a.mov_rr(true, g(0), T0);
                self.a.mov_rr(true, g(2), T1);
                self.set_lb(0, 64);
                self.set_lb(2, 64);
                self.a.str(8, ZR, XCPU, OFF_ZF);
                self.a.bind(after);
                self.fs.host &= !FZ;
                self.fs.mem |= FZ;
            }
        }
    }

    pub(super) fn emit_pcmpstr_op(&mut self, k: usize, op: &FastOp) {
        let is_exp = (op.x & 1) != 0;
        let is_msk = (op.x & 2) != 0;
        let is_vex = (op.x & 4) != 0;
        let is_long = (op.x & 8) != 0;
        let mem = (op.x & 0x20) != 0;
        let imm = op.imm as u8;
        let is_words = (imm & 1) != 0;
        let is_signed = (imm & 2) != 0;
        let n_elems: u32 = if is_words { 8 } else { 16 };
        let ebits: u32 = if is_words { 16 } else { 8 };
        let sh_elem = is_words;

        if mem {
            let a = self.addr_into(op, XA);
            if Self::chk_of(op) == Chk::Full {
                let se = self.side_exit_here(k);
                self.range_check_to(a, se);
            }
            self.a.ldr(64, T0, a, 0);
            self.a.ldr(64, T1, a, 8);
            if mem::tso() {
                self.a.dmb_ishld();
            }
            self.a.fmov_gf(true, 1, T0);
            self.a.v_ins_d(1, 1, T1);
        } else {
            self.xsrc(1, op.r1);
        }
        self.xsrc(0, op.r0);
        // Spill v0 (a) at [sp] and v1 (b) at [sp + 16]; borrow x7 (XRAS) as temp inside the loop.
        const P7: u32 = 7;
        if self.ras_miss != 0 {
            self.a.str(64, XRAS, XCPU, OFF_RAS);
        }
        self.a.addsub_ri(true, true, false, 31, 31, 32, false);
        self.a.str(128, 0, 31, 0);
        self.a.str(128, 1, 31, 16);
        self.a.addsub_ri(true, false, false, XA, 31, 0, false); // XA = &a[0]
        self.a.addsub_ri(true, false, false, XS, 31, 16, false); // XS = &b[0]

        // Compute T0 = a_len, T1 = b_len.
        if is_exp {
            for (dst, src_g) in [(T0, g(0)), (T1, g(2))] {
                if is_long {
                    self.a.mov_rr(true, dst, src_g);
                    let pos = self.a.tbz(dst, 63);
                    self.a.addsub_rr(true, true, false, dst, ZR, dst, 0);
                    self.a.bind(pos);
                    self.a.movz(true, T2, n_elems, 0);
                    self.a.addsub_rr(true, true, true, ZR, dst, T2, 0);
                    self.a.csel(true, dst, T2, dst, HI);
                } else {
                    self.a.sbfm(false, dst, src_g, 0, 31);
                    let pos = self.a.tbz(dst, 31);
                    self.a.addsub_rr(false, true, false, dst, ZR, dst, 0);
                    self.a.bind(pos);
                    self.a.movz(false, T2, n_elems, 0);
                    self.a.addsub_rr(false, true, true, ZR, dst, T2, 0);
                    self.a.csel(false, dst, T2, dst, HI);
                }
            }
        } else {
            for (dst, base_ptr) in [(T0, XA), (T1, XS)] {
                self.a.movz(false, dst, 0, 0);
                let top = self.a.pos;
                self.a.addsub_ri(false, true, true, ZR, dst, n_elems, false);
                let end = self.a.bcond(HS);
                self.a.ldr_reg(ebits, false, T2, base_ptr, dst, sh_elem);
                let found_zero = self.a.cbz(false, T2);
                self.a.addsub_ri(false, false, false, dst, dst, 1, false);
                let back = self.a.b();
                self.a.patch(back, top);
                self.a.bind(end);
                self.a.bind(found_zero);
            }
        }

        // Compute T2 = int_res1 across j in 0..n_elems.
        self.a.movz(false, T2, 0, 0); // int_res1 = 0
        self.a.movz(false, T3, 0, 0); // j = 0
        let j_top = self.a.pos;
        let j_limit = if matches!((imm >> 2) & 3, 0 | 1) { T1 } else { ZR };
        if j_limit == T1 {
            self.a.addsub_rr(false, true, true, ZR, T3, T1, 0);
        } else {
            self.a.addsub_ri(false, true, true, ZR, T3, n_elems, false);
        }
        let j_done = self.a.bcond(HS);

        match (imm >> 2) & 3 {
            0 => {
                // Equal Any
                self.a.ldr_reg(ebits, is_signed, T5, XS, T3, sh_elem); // bj
                self.a.movz(false, T4, 0, 0); // i = 0
                let i_top = self.a.pos;
                self.a.addsub_rr(false, true, true, ZR, T4, T0, 0);
                let i_done = self.a.bcond(HS);
                self.a.ldr_reg(ebits, is_signed, T6, XA, T4, sh_elem); // ai
                self.a.addsub_rr(false, true, true, ZR, T5, T6, 0);
                let neq = self.a.bcond(NE);
                self.a.movz(false, P7, 1, 0);
                self.a.shiftv(false, 8, P7, P7, T3);
                self.a.logic_rr(false, 1, T2, T2, P7, 0, 0);
                let brk = self.a.b();
                self.a.bind(neq);
                self.a.addsub_ri(false, false, false, T4, T4, 1, false);
                let back = self.a.b();
                self.a.patch(back, i_top);
                self.a.bind(i_done);
                self.a.bind(brk);
            }
            1 => {
                // Ranges
                self.a.ldr_reg(ebits, is_signed, T5, XS, T3, sh_elem); // bj
                self.a.movz(false, T4, 0, 0); // i = 0
                let i_top = self.a.pos;
                self.a.addsub_ri(false, false, false, T6, T4, 1, false);
                self.a.addsub_rr(false, true, true, ZR, T6, T0, 0);
                let i_done = self.a.bcond(HS);
                self.a.ldr_reg(ebits, is_signed, P7, XA, T4, sh_elem); // a[i]
                self.a.addsub_rr(false, true, true, ZR, T5, P7, 0);
                let lt_lo = self.a.bcond(LT);
                self.a.ldr_reg(ebits, is_signed, P7, XA, T6, sh_elem); // a[i+1]
                self.a.addsub_rr(false, true, true, ZR, T5, P7, 0);
                let gt_hi = self.a.bcond(GT);
                self.a.movz(false, P7, 1, 0);
                self.a.shiftv(false, 8, P7, P7, T3);
                self.a.logic_rr(false, 1, T2, T2, P7, 0, 0);
                let brk = self.a.b();
                self.a.bind(lt_lo);
                self.a.bind(gt_hi);
                self.a.addsub_ri(false, false, false, T4, T4, 2, false);
                let back = self.a.b();
                self.a.patch(back, i_top);
                self.a.bind(i_done);
                self.a.bind(brk);
            }
            2 => {
                // Equal Each
                self.a.addsub_rr(false, true, true, ZR, T3, T0, 0);
                let a_in = self.a.bcond(LO);
                // j >= a_len: match iff j >= b_len
                self.a.addsub_rr(false, true, true, ZR, T3, T1, 0);
                let no_match1 = self.a.bcond(LO);
                self.a.movz(false, P7, 1, 0);
                self.a.shiftv(false, 8, P7, P7, T3);
                self.a.logic_rr(false, 1, T2, T2, P7, 0, 0);
                let done1 = self.a.b();
                self.a.bind(a_in);
                // j < a_len: match iff j < b_len && a[j] == b[j]
                self.a.addsub_rr(false, true, true, ZR, T3, T1, 0);
                let no_match2 = self.a.bcond(HS);
                self.a.ldr_reg(ebits, is_signed, T5, XA, T3, sh_elem);
                self.a.ldr_reg(ebits, is_signed, T6, XS, T3, sh_elem);
                self.a.addsub_rr(false, true, true, ZR, T5, T6, 0);
                let no_match3 = self.a.bcond(NE);
                self.a.movz(false, P7, 1, 0);
                self.a.shiftv(false, 8, P7, P7, T3);
                self.a.logic_rr(false, 1, T2, T2, P7, 0, 0);
                self.a.bind(no_match1);
                self.a.bind(no_match2);
                self.a.bind(no_match3);
                self.a.bind(done1);
            }
            _ => {
                // Equal Ordered
                self.a.movz(false, T4, 0, 0); // k = 0
                let k_top = self.a.pos;
                self.a.addsub_rr(false, false, false, T5, T3, T4, 0); // j + k
                self.a.addsub_ri(false, true, true, ZR, T5, n_elems, false);
                let k_matched = self.a.bcond(HS);
                self.a.addsub_rr(false, true, true, ZR, T4, T0, 0); // k >= a_len
                let k_matched2 = self.a.bcond(HS);
                self.a.addsub_rr(false, true, true, ZR, T5, T1, 0); // j + k >= b_len
                let k_failed = self.a.bcond(HS);
                self.a.ldr_reg(ebits, is_signed, T6, XA, T4, sh_elem);
                self.a.ldr_reg(ebits, is_signed, P7, XS, T5, sh_elem);
                self.a.addsub_rr(false, true, true, ZR, T6, P7, 0);
                let k_failed2 = self.a.bcond(NE);
                self.a.addsub_ri(false, false, false, T4, T4, 1, false);
                let back = self.a.b();
                self.a.patch(back, k_top);
                self.a.bind(k_matched);
                self.a.bind(k_matched2);
                self.a.movz(false, P7, 1, 0);
                self.a.shiftv(false, 8, P7, P7, T3);
                self.a.logic_rr(false, 1, T2, T2, P7, 0, 0);
                self.a.bind(k_failed);
                self.a.bind(k_failed2);
            }
        }
        self.a.addsub_ri(false, false, false, T3, T3, 1, false);
        let j_back = self.a.b();
        self.a.patch(j_back, j_top);
        self.a.bind(j_done);

        // Restore sp (+32) and XRAS.
        self.a.addsub_ri(true, false, false, 31, 31, 32, false);
        if self.ras_miss != 0 {
            self.a.ldr(64, XRAS, XCPU, OFF_RAS);
        }

        // Polarity -> int_res2 in T2.
        let mask_all = if is_words { 0x00FFu32 } else { 0xFFFFu32 };
        self.a.movz(false, T3, mask_all, 0);
        match (imm >> 4) & 3 {
            1 => {
                self.a.bic_rr(false, T2, T3, T2);
            }
            3 => {
                self.a.movz(false, T4, 1, 0);
                self.a.shiftv(false, 8, T4, T4, T1);
                self.a.addsub_ri(false, true, false, T4, T4, 1, false); // (1 << b_len) - 1
                self.a.logic_rr(false, 2, T2, T2, T4, 0, 0);
                self.a.logic_rr(false, 0, T2, T2, T3, 0, 0);
            }
            _ => {
                self.a.logic_rr(false, 0, T2, T2, T3, 0, 0);
            }
        }

        if is_msk {
            if (imm & 0x40) == 0 {
                self.a.fmov_gf(true, 0, T2);
            } else {
                self.a.v_zero(0);
                let lg = if is_words { 1u32 } else { 0u32 };
                self.a.mov_imm(false, T4, 0xFFFF);
                for lane in 0..n_elems {
                    let skip = self.a.tbz(T2, lane);
                    self.a.v_ins_elem(lg, 0, lane, T4);
                    self.a.bind(skip);
                }
            }
            self.xwrite(0, 0);
            if is_vex {
                self.clear_ymm_hi(0);
            }
        } else {
            let zero_res = self.a.cbz(false, T2);
            if (imm & 0x40) != 0 {
                self.a.clz(false, T4, T2);
                self.a.movz(false, T5, 31, 0);
                self.a.addsub_rr(false, true, false, g(1), T5, T4, 0);
            } else {
                self.a.rbit(false, T4, T2);
                self.a.clz(false, g(1), T4);
            }
            let idx_done = self.a.b();
            self.a.bind(zero_res);
            self.a.movz(false, g(1), n_elems, 0);
            self.a.bind(idx_done);
            self.set_lb(1, 64);
        }

        // Flags: CF = (int_res2 != 0), ZF = (b_len < n), SF = (a_len < n), OF = (int_res2 & 1), PF = 0.
        self.a.clz(true, T4, T2);
        self.a.lsr_i(true, T4, T4, 6);
        self.a.logic_ri(false, 2, T4, T4, enc_logic_imm(1, false).unwrap());
        self.a.str(8, T4, XCPU, OFF_CF);
        self.a.addsub_ri(false, true, true, ZR, T1, n_elems, false);
        self.a.cset(T4, LO);
        self.a.str(8, T4, XCPU, OFF_ZF);
        self.a.addsub_ri(false, true, true, ZR, T0, n_elems, false);
        self.a.cset(T4, LO);
        self.a.str(8, T4, XCPU, OFF_SF);
        self.a.ubfx(T4, T2, 0, 1);
        self.a.str(8, T4, XCPU, OFF_OF);
        self.a.movz(false, T4, 1, 0);
        self.a.str(8, T4, XCPU, OFF_PFV);
        self.fs = FlagState { src: Src::None, host: 0, mem: FALL };
    }
}
