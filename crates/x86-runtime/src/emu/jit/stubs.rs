//! Shared JIT/AOT trampoline stubs (`enter`, `exit`, `lookup_rip`, `lookup_jmp`, `lookup_call`,
//! `host_xfer`, `ras_miss`, `spill_xmm`, `reload_xmm`, `clear_ymm_hi`).

use super::cache::JBITS;
use super::emit::{enc_logic_imm, Asm};
use super::translate::stack_tso;
use super::xmm::xmm_pin_enabled;
use super::*;
use crate::emu::mem;

/// Explicit stub emission configuration, shared by dynamic JIT/AOT (`StubCfg::live`)
/// and offline static Mach-O `.o` emission (`StubCfg::from_aot_knobs`).
#[derive(Clone, Copy)]
pub(super) struct StubCfg {
    pub(super) ras: bool,
    pub(super) xpin: bool,
    pub(super) lb_on: bool,
    pub(super) bud_light: bool,
    pub(super) aot_on: bool,
    pub(super) bud: bool,
    pub(super) pushpop: bool,
    pub(super) stack_tso_active: bool,
    pub(super) hx: bool,
    pub(super) count_probe_hits: bool,
    #[cfg_attr(not(test), allow(dead_code))]
    pub(super) count_plain_hits: bool,
    /// When true, emit a relocatable `bl 0` (`ARM64_RELOC_BRANCH26 -> _port_static_aot_host_xfer`)
    /// instead of a baked 64-bit host pointer, and pad before `ras_miss` to match `target_ras_miss`.
    pub(super) static_pic: bool,
    pub(super) target_ras_miss: Option<usize>,
}

impl StubCfg {
    pub(super) fn live(ras: bool) -> Self {
        Self {
            ras,
            xpin: xmm_pin_enabled(),
            lb_on: last_bits_enabled(),
            bud_light: budget_light_enabled(),
            aot_on: aot_probe_enabled(),
            bud: budget_backedge_enabled(),
            pushpop: pushpop_enabled(),
            stack_tso_active: mem::tso() && stack_tso(),
            hx: hostcall_enabled(),
            count_probe_hits: cfg!(test) || crate::emu::jit_stats::enabled(),
            count_plain_hits: cfg!(test),
            static_pic: false,
            target_ras_miss: None,
        }
    }

    pub(super) fn from_aot_knobs(
        knobs: u64,
        want_ras_miss: Option<usize>,
        count_probe_hits: bool,
        count_plain_hits: bool,
    ) -> Self {
        let bit = |b: u32| ((knobs >> b) & 1) != 0;
        Self {
            ras: bit(0),
            xpin: bit(30),
            lb_on: bit(32),
            bud_light: bit(34),
            aot_on: bit(27),
            bud: bit(9),
            pushpop: bit(8),
            stack_tso_active: bit(15) && bit(16),
            hx: bit(13),
            count_probe_hits,
            count_plain_hits,
            static_pic: true,
            target_ras_miss: want_ras_miss,
        }
    }
}

/// Convert the 24 MHz count in x0 to guest ticks without division or flag changes.
pub(super) fn emit_tsc_scale_24mhz(a: &mut Asm) {
    // floor(125*x/3) = 41*x + floor(2*x/3), modulo 2^64.
    a.mov_imm(true, T2, crate::emu::tsc::TWO_THIRDS_Q64 - 1);
    a.addsub_ri(true, false, false, T2, T2, 1, false);
    a.umulh(T1, T0, T2);
    a.addsub_rr(true, false, false, T2, T0, T0, 2);
    a.addsub_rr(true, false, false, T0, T0, T2, 3);
    a.addsub_rr(true, false, false, T0, T0, T1, 0);
}

/// Timestamp fallback entries. Both return the value in x0 and preserve NZCV and all
/// guest registers. The ordered caller has already issued DSB ISHLD.
fn emit_timestamp_stubs(a: &mut Asm) -> (usize, usize) {
    let ordered = a.pos;
    let legacy = a.tbz(T1, 3);
    a.mrs_cntvctss(T0);
    let scale_ecv = a.b();
    a.bind(legacy);
    a.isb();
    let native = a.tbz(T1, 1);

    let unordered = a.pos;
    let frozen = a.tbnz(T1, 0);
    a.mrs_cntvct(T0);
    a.bind(scale_ecv);
    let generic = a.tbz(T1, 2);
    emit_tsc_scale_24mhz(a);
    a.ret();

    a.bind(generic);
    // CNTFRQ_EL0 fits 32 bits, so (counter % frequency) * 1e9 fits u64.
    a.mrs_cntfrq(T1);
    a.mov_imm(true, T2, crate::emu::tsc::TSC_NOMINAL_HZ);
    a.udiv(true, T3, T0, T1);
    a.msub(true, T0, T3, T1, T0);
    a.mul(true, T0, T0, T2);
    a.mul(true, T3, T3, T2);
    a.udiv(true, T0, T0, T1);
    a.addsub_rr(true, false, false, T0, T3, T0, 0);
    a.ret();

    a.bind(frozen);
    a.mov_imm(true, T0, crate::emu::GUEST_TSC);
    a.ret();
    a.bind(native);
    a.mrs_cntvct(T0);
    a.ret();
    (unordered, ordered)
}

/// Shared stubs (never overwritten by translation-cache flushes). `ras`: emit the shadow
/// return-address stack push in `lookup_call` and the `ras_miss` stub.
pub(super) fn emit_stubs(a: &mut Asm, ras: bool) -> Stubs {
    emit_stubs_cfg(a, StubCfg::live(ras)).0
}

pub(super) fn emit_stubs_cfg(a: &mut Asm, cfg: StubCfg) -> (Stubs, Option<usize>) {
    let ras = cfg.ras;
    let xpin = cfg.xpin;
    let lb_on = cfg.lb_on;
    let bud_light = cfg.bud_light;
    // enter(cpu: x0, code: x1)
    let enter = a.pos;
    a.emit(0xA980_0000 | ((-12i32 as u32) & 0x7F) << 15 | 30 << 10 | 31 << 5 | 29); // stp x29, x30, [sp, #-96]!
    a.emit(0x9100_03FD); // mov x29, sp
    a.stp_x(19, 20, 31, 16);
    a.stp_x(21, 22, 31, 32);
    a.stp_x(23, 24, 31, 48);
    a.stp_x(25, 26, 31, 64);
    a.stp_x(27, 28, 31, 80);
    a.mov_rr(true, XCPU, 0);
    for i in 0..8 {
        a.ldp_x(HG[2 * i], HG[2 * i + 1], XCPU, (OFF_GPR + 16 * i as u32) as i32);
    }
    if xpin {
        for i in 0..8u32 {
            a.ldp_q(16 + 2 * i, 17 + 2 * i, XCPU, (OFF_XMM + 32 * i) as i32);
        }
    }
    if ras {
        a.ldr(64, XRAS, XCPU, OFF_RAS);
    }
    // Pack `XBUD = (budget << 1) | (ymm_dirty != 0)` so VEX-128 / `vzeroupper` checks are a
    // single register-only `tbnz x8, #0` while `sub x8, #(2*n)` and `tbnz x8, #63` keep exact
    // budget semantics without touching bit 0.
    a.ldr(64, XBUD, XCPU, OFF_BUDGET);
    a.lsl_i(true, XBUD, XBUD, 1);
    a.ldr(16, XS, XCPU, OFF_YMM_DIRTY);
    a.addsub_ri(false, true, true, ZR, XS, 0, false); // cmp w17, #0
    a.cset(XS, NE);
    a.logic_rr(true, 1, XBUD, XBUD, XS, 0, 0); // orr x8, x8, x17
    a.br(1);

    // exit (w0 = exit code; rip / target already stored)
    let exit = a.pos;
    for i in 0..8 {
        a.stp_x(HG[2 * i], HG[2 * i + 1], XCPU, (OFF_GPR + 16 * i as u32) as i32);
    }
    if xpin {
        for i in 0..8u32 {
            a.stp_q(16 + 2 * i, 17 + 2 * i, XCPU, (OFF_XMM + 32 * i) as i32);
        }
    }
    if ras {
        a.str(64, XRAS, XCPU, OFF_RAS);
    }
    a.asr_i(true, XA, XBUD, 1);
    a.str(64, XA, XCPU, OFF_BUDGET);
    // exit_ns: same, with the GPRs and budget already stored by the caller.
    let exit_ns = a.pos;
    a.ldp_x(19, 20, 31, 16);
    a.ldp_x(21, 22, 31, 32);
    a.ldp_x(23, 24, 31, 48);
    a.ldp_x(25, 26, 31, 64);
    a.ldp_x(27, 28, 31, 80);
    a.emit(0xA8C0_0000 | (12u32 & 0x7F) << 15 | 30 << 10 | 31 << 5 | 29); // ldp x29, x30, [sp], #96
    a.ret();

    // Shared XMM spill/reload helpers (preserves all GPRs, q0..q15, and NZCV).
    let (spill_xmm, reload_xmm) = if xpin {
        let sp_pos = a.pos;
        for i in 0..8u32 {
            a.stp_q(16 + 2 * i, 17 + 2 * i, XCPU, (OFF_XMM + 32 * i) as i32);
        }
        a.ret();

        let rl_pos = a.pos;
        for i in 0..8u32 {
            a.ldp_q(16 + 2 * i, 17 + 2 * i, XCPU, (OFF_XMM + 32 * i) as i32);
        }
        a.ret();
        (Some(sp_pos), Some(rl_pos))
    } else {
        (None, None)
    };

    // Shared VEX-128 / `vzeroupper` lazy `Cpu::ymm_hi` zeroing stub: clears `Cpu::ymm_hi[r]`
    // for every bit `r` set in BOTH `w16` (mask) and `Cpu::jx.ymm_dirty`, updates
    // `Cpu::jx.ymm_dirty`, and clears bit 0 of `XBUD` once `ymm_dirty` reaches 0.
    // Clobbers only x16, x17 (and bit 0 of x8); preserves all other GPRs and NZCV.
    let clear_ymm_hi = a.pos;
    a.ldr(16, XS, XCPU, OFF_YMM_DIRTY);
    a.logic_rr(false, 0, XA, XA, XS, 0, 0); // and w16, w16, w17 (preserves NZCV)
    let ym_done = a.cbz(false, XA);
    a.bic_rr(false, XS, XS, XA);
    a.str(16, XS, XCPU, OFF_YMM_DIRTY);
    let ym_keep_bit = a.cbnz(false, XS);
    a.logic_ri(true, 0, XBUD, XBUD, enc_logic_imm(!1u64, true).unwrap());
    a.bind(ym_keep_bit);
    let ym_loop = a.pos;
    a.rbit(false, XS, XA);
    a.clz(false, XS, XS);
    a.addsub_rr(true, false, false, XS, XCPU, XS, 4); // x17 = XCPU + r*16
    a.stp_x(ZR, ZR, XS, OFF_YMM_HI as i32);
    a.addsub_ri(false, true, false, XS, XA, 1, false);
    a.logic_rr(false, 0, XA, XA, XS, 0, 0); // and w16, w16, w17
    let ym_back = a.cbnz(false, XA);
    a.patch(ym_back, ym_loop);
    a.ret();
    a.bind(ym_done);
    let ym_ret = a.cbnz(false, XS);
    a.logic_ri(true, 0, XBUD, XBUD, enc_logic_imm(!1u64, true).unwrap());
    a.bind(ym_ret);
    a.ret();

    let (rdtsc_slow, rdtscp_slow) = emit_timestamp_stubs(a);

    // Hash probe of x0 -> on hit branches to the code (x1); on miss falls through.
    // Clobbers x1, x2, x16, x17 and NZCV.
    let probe = |a: &mut Asm| -> [usize; 3] {
        a.ldr(64, XA, XCPU, OFF_TABLE);
        a.ldr(64, XS, XCPU, OFF_TABLE + 8);
        a.mul(true, XS, 0, XS);
        a.lsr_i(true, XS, XS, 64 - JBITS);
        a.addsub_rr(true, false, false, XA, XA, XS, 5);
        a.ldp_x(XS, 1, XA, 0);
        a.addsub_rr(true, true, true, ZR, XS, 0, 0); // cmp x17, x0
        let m1 = a.bcond(NE);
        let m2 = a.cbz(true, 1);
        a.ldr(32, XS, XA, 16);
        let ok = a.cbz(false, XS);
        a.ldr(64, 2, XCPU, OFF_CODEGEN);
        a.ldr(32, 2, 2, 0);
        a.addsub_rr(false, true, true, ZR, XS, 2, 0); // cmp w17, w2
        let m3 = a.bcond(NE);
        a.bind(ok);
        [m1, m2, m3]
    };

    // Shared AOT probe table (`aot_probe_enabled`; layout: `aot::probe_table`) search of x0
    // after a `probe` miss: on a live entry branches to `hit` (word position of the stub's
    // hit path, x1 = code); otherwise falls through (and returns the branches to bind
    // there). Same clobbers as `probe`. Emits nothing when the knob is off.
    let aot_on = cfg.aot_on;
    let count_probe_hits = cfg.count_probe_hits;
    let aot_probe = |a: &mut Asm, hit: usize| -> Vec<usize> {
        if !aot_on {
            return Vec::new();
        }
        a.ldr(64, XA, XCPU, OFF_AOTDIR);
        let m0 = a.cbz(true, XA);
        a.ldr(64, XS, XCPU, OFF_TABLE + 8);
        a.mul(true, XS, 0, XS);
        a.ldr(64, 2, XA, 0); // shift
        a.shiftv(true, 9, XS, XS, 2); // lsrv
        let lp = a.pos;
        a.addsub_rr(true, false, false, 1, XA, XS, 4);
        a.ldp_x(2, 1, 1, 16); // (ip, fast)
        a.addsub_rr(true, true, true, ZR, 2, 0, 0); // cmp x2, x0
        let found = a.bcond(EQ);
        let m1 = a.cbz(true, 2);
        a.addsub_ri(true, false, false, XS, XS, 1, false);
        a.ldr(64, 2, XA, 8); // mask
        a.logic_rr(true, 0, XS, XS, 2, 0, 0);
        let back = a.b();
        a.patch(back, lp);
        a.bind(found);
        let m2 = a.cbz(true, 1);
        // Populate this thread's direct-mapped `table[jslot(x0)]` inline (`gen = 0, hits = 0,
        // code = x1`) so repeated indirect calls/jmps/rets hit in O(1) without probing again.
        a.ldr(64, XA, XCPU, OFF_TABLE);
        a.ldr(64, XS, XCPU, OFF_TABLE + 8);
        a.mul(true, XS, 0, XS);
        a.lsr_i(true, XS, XS, 64 - JBITS);
        a.addsub_rr(true, false, false, XA, XA, XS, 5);
        a.stp_x(0, 1, XA, 0);
        a.stp_x(ZR, 1, XA, 16);
        if count_probe_hits {
            a.ldr(64, 2, XCPU, OFF_PROBEHITS);
            a.addsub_ri(true, false, false, 2, 2, 1, false);
            a.str(64, 2, XCPU, OFF_PROBEHITS);
        }
        let h = a.b();
        a.patch(h, hit);
        vec![m0, m1, m2]
    };

    // With AW2_JIT_BUDGET_BACKEDGE, blocks only test the budget before backward/RAS
    // transfers; every dynamic transfer that reaches translated code tests it here (hit
    // paths: x0 is a translated guest IP, so leaving with EXIT_RIP there is exact).
    let bud = cfg.bud;

    // lookup_rip(x0 = guest rip): continue there, or exit with EXIT_RIP.
    let lookup_rip = a.pos;
    let miss = probe(a);
    let m_plain_rip = a.tbnz(1, 0);
    let hit = a.pos;
    if bud_light {
        a.addsub_ri(true, true, false, XBUD, XBUD, 2, false);
    }
    let bud_rip = bud.then(|| a.tbnz(XBUD, 63));
    a.br(1);
    for m in miss {
        a.bind(m);
    }
    a.bind(m_plain_rip);
    for m in aot_probe(a, hit) {
        a.bind(m);
    }
    if let Some(f) = bud_rip {
        a.bind(f);
    }
    let rip_out = a.pos;
    a.str(64, 0, XCPU, OFF_RIP);
    a.movz(false, 0, EXIT_RIP as u32, 0);
    let e = a.addr_of(exit);
    a.b_to(e);

    // lookup_jmp(x0 = target)
    // Entries in `table` with bit 0 of `fast` set are cached `DF_PLAIN` / pure-float-math /
    // inline-QPC host functions dispatched directly in `is_plain_jmp` / `is_qpc_jmp`.
    let lookup_jmp = a.pos;
    let miss = probe(a);
    let is_plain_jmp = a.tbnz(1, 0);
    let hit = a.pos;
    if bud {
        if bud_light {
            a.addsub_ri(true, true, false, XBUD, XBUD, 2, false);
        }
        let f = a.tbnz(XBUD, 63);
        a.patch(f, rip_out);
    }
    a.br(1);
    for m in miss {
        a.bind(m);
    }
    for m in aot_probe(a, hit) {
        a.bind(m);
    }
    let hx = cfg.hx;
    let mut to_xfer: Vec<usize> = Vec::new();
    let jmp_slow = a.pos;
    if hx {
        // Host tail call (or EXIT_JMP from the helper for guest targets); kind 1 | 2 (dynamic).
        a.movz(true, 2, 3, 0);
        to_xfer.push(a.b());
    } else {
        a.str(64, 0, XCPU, OFF_TARGET);
        a.str(64, 0, XCPU, OFF_RIP);
        a.movz(false, 0, EXIT_JMP as u32, 0);
        a.b_to(e);
    }

    // lookup_call(x0 = target, x3 = return address): push + continue, or EXIT_CALL.
    // With the RAS, call sites enter with `bl lookup_call` (x30 = their landing pad).
    // Entries in `table` with bit 0 of `fast` set are cached `DF_PLAIN` / pure-float-math /
    // inline-QPC host functions dispatched directly here.
    let lookup_call = a.pos;
    let miss = probe(a);
    let is_plain = a.tbnz(1, 0);
    let hit = a.pos;
    let unal = if cfg.pushpop {
        // Plain pre-indexed push (see `pushpop_enabled`): no alignment check.
        a.str_pre(T3, HG[4], -8);
        None
    } else {
        a.addsub_ri(true, true, false, XA, HG[4], 8, false); // x16 = rsp - 8
        a.logic_ri(true, 0, XS, XA, enc_logic_imm(7, true).unwrap());
        let unal = a.cbnz(true, XS);
        if cfg.stack_tso_active {
            a.stlr(64, T3, XA);
        } else {
            a.str(64, T3, XA, 0);
        }
        a.mov_rr(true, HG[4], XA);
        Some(unal)
    };
    if ras {
        ras_push(a, T3, 30);
    }
    if bud {
        if bud_light {
            a.addsub_ri(true, true, false, XBUD, XBUD, 2, false);
        }
        // The call itself is done (return address pushed): continue at the callee.
        let f = a.tbnz(XBUD, 63);
        a.patch(f, rip_out);
    }
    a.br(1);
    for m in miss {
        a.bind(m);
    }
    for m in aot_probe(a, hit) {
        a.bind(m);
    }
    let call_slow = a.pos;
    if hx {
        // Host call (or EXIT_CALL from the helper for guest targets); nothing pushed yet.
        // kind 0 | 2 (dynamic).
        a.str(64, T3, XCPU, OFF_RIP);
        a.movz(true, 2, 2, 0);
        to_xfer.push(a.b());
    }
    if let Some(u) = unal {
        a.bind(u);
    }
    a.str(64, 0, XCPU, OFF_TARGET);
    a.str(64, T3, XCPU, OFF_RIP);
    a.movz(false, 0, EXIT_CALL as u32, 0);
    a.b_to(e);

    // Fast path (`lookup_call`) for tagged `DF_PLAIN` (bit 0), pure-float-math (bits 0..1),
    // and inline zero-spill QPC (bit 63 | bit 0) host entries cached in `table`.
    a.bind(is_plain);
    let is_qpc = a.tbnz(1, 63);
    a.logic_ri(true, 0, XA, 1, enc_logic_imm(!3u64, true).unwrap());
    a.addsub_ri(true, true, false, 31, 31, 96, false); // sub sp, sp, #96
    a.stp_x(8, 9, 31, 0);
    a.stp_x(10, 11, 31, 16);
    a.stp_x(12, 13, 31, 32);
    a.stp_x(14, 15, 31, 48);
    a.stp_x(30, T3, 31, 64);
    a.str(64, 1, 31, 88); // preserve tagged host pointer (bit 1 = needs_xmm) across blr
    let no_xmm_in = a.tbz(1, 1);
    if xpin {
        a.v_mov(0, 16);
        a.v_mov(1, 17);
        a.v_mov(2, 18);
        a.v_mov(3, 19);
    } else {
        a.ldp_q(0, 1, XCPU, OFF_XMM as i32);
        a.ldp_q(2, 3, XCPU, (OFF_XMM + 32) as i32);
    }
    a.bind(no_xmm_in);
    // AAPCS64 designates `v16..v31` as caller-saved; even integer-only `DF_PLAIN` builtins
    // (e.g. `memcpy`/`memset` via NEON or LLVM auto-vectorized Rust) can clobber `v16..v31`,
    // so pinned XMM registers must always be spilled before `blr` and reloaded afterward.
    if let Some(sp_pos) = spill_xmm {
        let sp_addr = a.addr_of(sp_pos);
        a.bl_to(sp_addr);
    }
    if ras {
        a.str(64, XRAS, XCPU, OFF_RAS);
    }
    for (j, gi) in [1u8, 2, 8, 9].into_iter().enumerate() {
        a.mov_rr(true, j as u32, g(gi));
    }
    a.ldp_x(4, 5, g(4), 0x20);
    a.ldp_x(6, 7, g(4), 0x30);
    a.call_with_host_fpcr(80);
    a.ldp_x(8, 9, 31, 0);
    a.ldp_x(10, 11, 31, 16);
    a.ldp_x(12, 13, 31, 32);
    a.ldp_x(14, 15, 31, 48);
    if let Some(rl_pos) = reload_xmm {
        let rl_addr = a.addr_of(rl_pos);
        a.bl_to(rl_addr);
    }
    // Only write `q0` back to guest `xmm0` for float-ABI (`needs_xmm`, bit 1 == 1) callees;
    // integer-only `DF_PLAIN` builtins preserve guest `xmm0` rather than overwriting it with leftover `q0`.
    a.ldr(64, XS, 31, 88);
    let no_xmm_out = a.tbz(XS, 1);
    if xpin {
        a.v_mov(16, 0);
    } else {
        a.str(128, 0, XCPU, OFF_XMM);
    }
    a.bind(no_xmm_out);
    if ras {
        a.ldr(64, XRAS, XCPU, OFF_RAS);
    }
    a.ldp_x(30, T3, 31, 64);
    a.addsub_ri(true, false, false, 31, 31, 96, false); // add sp, sp, #96
    a.mov_rr(true, g(0), 0);
    if lb_on {
        a.str(8, ZR, XCPU, OFF_LB);
    }
    #[cfg(test)]
    if cfg.count_plain_hits {
        a.ldr(64, 2, XCPU, OFF_PLAINHITS);
        a.addsub_ri(true, false, false, 2, 2, 1, false);
        a.str(64, 2, XCPU, OFF_PLAINHITS);
    }
    let call_bud_out = if ras {
        let bud_plain = bud.then(|| a.tbnz(XBUD, 63));
        a.ret();
        if let Some(f) = bud_plain {
            a.bind(f);
            let out = a.pos;
            a.mov_rr(true, 0, T3);
            let ro = a.addr_of(rip_out);
            a.b_to(ro);
            Some(out)
        } else {
            None
        }
    } else {
        a.mov_rr(true, 0, T3);
        let lr = a.addr_of(lookup_rip);
        a.b_to(lr);
        None
    };

    // Zero-spill inline QPC (`lookup_call`): `x0` = target, `T3` = return address, `x30` = landing pad.
    a.bind(is_qpc);
    a.logic_ri(true, 0, XA, 1, enc_logic_imm(!((1u64 << 63) | 3), true).unwrap());
    a.ldp_x(2, XS, XA, 0); // x2 = mode, x17 = mult
    a.addsub_ri(true, true, true, ZR, 2, 1, false); // cmp x2, #1
    let qpc_slow = a.bcond(NE);
    a.patch(qpc_slow, call_slow);
    a.mrs_cntvct(2);
    a.ldr(64, XA, XA, 16); // x16 = offset
    a.umulh(2, 2, XS);
    a.addsub_rr(true, false, false, 2, 2, XA, 0);
    a.str(64, 2, g(1), 0); // *lpPerformanceCount = result
    a.movz(false, g(0), 1, 0); // eax = 1 (TRUE, zero-extended to rax)
    if lb_on {
        a.str(8, ZR, XCPU, OFF_LB);
    }
    #[cfg(test)]
    if cfg.count_plain_hits {
        a.ldr(64, 2, XCPU, OFF_PLAINHITS);
        a.addsub_ri(true, false, false, 2, 2, 1, false);
        a.str(64, 2, XCPU, OFF_PLAINHITS);
    }
    if ras {
        if let Some(out) = call_bud_out {
            let f = a.tbnz(XBUD, 63);
            a.patch(f, out);
        }
        a.ret();
    } else {
        a.mov_rr(true, 0, T3);
        let lr = a.addr_of(lookup_rip);
        a.b_to(lr);
    }

    // Fast path (`lookup_jmp`) for tail jumps to tagged `DF_PLAIN` / pure-float-math / inline-QPC
    // host entries cached in `table`.
    let mut to_ras_miss: Vec<usize> = Vec::new();
    a.bind(is_plain_jmp);
    let is_qpc_jmp = a.tbnz(1, 63);
    a.ldr_post(T3, g(4), 8); // pop guest return address; g(4) now points to Win64 shadow space
    a.logic_ri(true, 0, XA, 1, enc_logic_imm(!3u64, true).unwrap());
    a.addsub_ri(true, true, false, 31, 31, 96, false); // sub sp, sp, #96
    a.stp_x(8, 9, 31, 0);
    a.stp_x(10, 11, 31, 16);
    a.stp_x(12, 13, 31, 32);
    a.stp_x(14, 15, 31, 48);
    a.stp_x(30, T3, 31, 64);
    a.str(64, 1, 31, 88); // preserve tagged host pointer (bit 1 = needs_xmm) across blr
    let no_xmm_in_jmp = a.tbz(1, 1);
    if xpin {
        a.v_mov(0, 16);
        a.v_mov(1, 17);
        a.v_mov(2, 18);
        a.v_mov(3, 19);
    } else {
        a.ldp_q(0, 1, XCPU, OFF_XMM as i32);
        a.ldp_q(2, 3, XCPU, (OFF_XMM + 32) as i32);
    }
    a.bind(no_xmm_in_jmp);
    // AAPCS64 designates `v16..v31` as caller-saved (clobbered by NEON `memcpy`/`memset`/Rust),
    // so pinned XMM registers must always be spilled before `blr` and reloaded afterward.
    if let Some(sp_pos) = spill_xmm {
        let sp_addr = a.addr_of(sp_pos);
        a.bl_to(sp_addr);
    }
    if ras {
        a.str(64, XRAS, XCPU, OFF_RAS);
    }
    for (j, gi) in [1u8, 2, 8, 9].into_iter().enumerate() {
        a.mov_rr(true, j as u32, g(gi));
    }
    a.ldp_x(4, 5, g(4), 0x20);
    a.ldp_x(6, 7, g(4), 0x30);
    a.call_with_host_fpcr(80);
    a.ldp_x(8, 9, 31, 0);
    a.ldp_x(10, 11, 31, 16);
    a.ldp_x(12, 13, 31, 32);
    a.ldp_x(14, 15, 31, 48);
    if let Some(rl_pos) = reload_xmm {
        let rl_addr = a.addr_of(rl_pos);
        a.bl_to(rl_addr);
    }
    // Only write `q0` back to guest `xmm0` for float-ABI (`needs_xmm`, bit 1 == 1) callees.
    a.ldr(64, XS, 31, 88);
    let no_xmm_out_jmp = a.tbz(XS, 1);
    if xpin {
        a.v_mov(16, 0);
    } else {
        a.str(128, 0, XCPU, OFF_XMM);
    }
    a.bind(no_xmm_out_jmp);
    if ras {
        a.ldr(64, XRAS, XCPU, OFF_RAS);
    }
    a.ldp_x(30, T3, 31, 64);
    a.addsub_ri(true, false, false, 31, 31, 96, false); // add sp, sp, #96
    a.mov_rr(true, g(0), 0);
    if lb_on {
        a.str(8, ZR, XCPU, OFF_LB);
    }
    #[cfg(test)]
    if cfg.count_plain_hits {
        a.ldr(64, 2, XCPU, OFF_PLAINHITS);
        a.addsub_ri(true, false, false, 2, 2, 1, false);
        a.str(64, 2, XCPU, OFF_PLAINHITS);
    }
    a.mov_rr(true, 0, T3);
    if bud {
        let f = a.tbnz(XBUD, 63);
        a.patch(f, rip_out);
    }
    if ras {
        a.ldp_x(1, 2, XRAS, 0);
        a.addsub_ri(true, true, false, XA, XRAS, 16, false);
        a.logic_rr(true, 2, 1, 1, 0, 0, 0); // eor x1, x1, x0
        to_ras_miss.push(a.cbnz(true, 1));
        a.bfxil0(XRAS, XA, 16);
        a.ret_r(2);
    } else {
        let lr = a.addr_of(lookup_rip);
        a.b_to(lr);
    }

    // Zero-spill inline QPC (`lookup_jmp`): `x0` = target, `[g(4)]` = guest return address.
    a.bind(is_qpc_jmp);
    a.logic_ri(true, 0, XA, 1, enc_logic_imm(!((1u64 << 63) | 3), true).unwrap());
    a.ldp_x(2, XS, XA, 0); // x2 = mode, x17 = mult
    a.addsub_ri(true, true, true, ZR, 2, 1, false); // cmp x2, #1
    let qpc_jmp_slow = a.bcond(NE);
    a.patch(qpc_jmp_slow, jmp_slow);
    a.mrs_cntvct(2);
    a.ldr(64, XA, XA, 16); // x16 = offset
    a.umulh(2, 2, XS);
    a.addsub_rr(true, false, false, 2, 2, XA, 0);
    a.str(64, 2, g(1), 0); // *lpPerformanceCount = result
    a.movz(false, g(0), 1, 0); // eax = 1 (TRUE, zero-extended to rax)
    if lb_on {
        a.str(8, ZR, XCPU, OFF_LB);
    }
    #[cfg(test)]
    if cfg.count_plain_hits {
        a.ldr(64, 2, XCPU, OFF_PLAINHITS);
        a.addsub_ri(true, false, false, 2, 2, 1, false);
        a.str(64, 2, XCPU, OFF_PLAINHITS);
    }
    a.ldr_post(0, g(4), 8); // pop guest return address into x0
    if bud {
        let f = a.tbnz(XBUD, 63);
        a.patch(f, rip_out);
    }
    if ras {
        a.ldp_x(1, 2, XRAS, 0);
        a.addsub_ri(true, true, false, XA, XRAS, 16, false);
        a.logic_rr(true, 2, 1, 1, 0, 0, 0); // eor x1, x1, x0
        to_ras_miss.push(a.cbnz(true, 1));
        a.bfxil0(XRAS, XA, 16);
        a.ret_r(2);
    } else {
        let lr = a.addr_of(lookup_rip);
        a.b_to(lr);
    }

    // host_xfer(x0 = target, x2 = kind: bit 0 = jmp (else call), bit 1 = dynamic target from
    // `lookup_call`/`lookup_jmp` (else a static target from `xfer_to`); cpu.rip = return
    // address for calls).
    let mut host_xfer_bl: Option<usize> = None;
    let host_xfer = if hx {
        let p = a.pos;
        for i in 0..8 {
            a.stp_x(HG[2 * i], HG[2 * i + 1], XCPU, (OFF_GPR + 16 * i as u32) as i32);
        }
        if xpin {
            for i in 0..8u32 {
                a.stp_q(16 + 2 * i, 17 + 2 * i, XCPU, (OFF_XMM + 32 * i) as i32);
            }
        }
        if ras {
            a.str(64, XRAS, XCPU, OFF_RAS);
        }
        a.asr_i(true, XA, XBUD, 1);
        a.str(64, XA, XCPU, OFF_BUDGET);
        a.mov_rr(true, 19, XCPU);
        a.mov_rr(true, 1, 0);
        a.mov_rr(true, 0, XCPU);
        if cfg.static_pic {
            // Resolved permanently at link time by `ARM64_RELOC_BRANCH26` to
            // `_port_static_aot_host_xfer` (zero baked host addresses in __TEXT).
            let bl_pos = a.bl();
            host_xfer_bl = Some(bl_pos * 4);
        } else {
            a.mov_imm(true, XA, crate::emu::jit_host_xfer as *const () as u64);
            a.blr(XA);
        }
        a.mov_rr(true, XCPU, 19);
        let out = a.cbnz(true, 0);
        for i in 0..8 {
            a.ldp_x(HG[2 * i], HG[2 * i + 1], XCPU, (OFF_GPR + 16 * i as u32) as i32);
        }
        if xpin {
            for i in 0..8u32 {
                a.ldp_q(16 + 2 * i, 17 + 2 * i, XCPU, (OFF_XMM + 32 * i) as i32);
            }
        }
        if ras {
            a.ldr(64, XRAS, XCPU, OFF_RAS);
        }
        a.ldr(64, XBUD, XCPU, OFF_BUDGET);
        a.lsl_i(true, XBUD, XBUD, 1);
        a.ldr(16, XS, XCPU, OFF_YMM_DIRTY);
        a.addsub_ri(false, true, true, ZR, XS, 0, false);
        a.cset(XS, NE);
        a.logic_rr(true, 1, XBUD, XBUD, XS, 0, 0);
        a.ldr(64, 0, XCPU, OFF_RIP);
        let lr = a.addr_of(lookup_rip);
        a.b_to(lr);
        a.bind(out);
        // GPRs and budget are already stored (x19 now holds the Cpu pointer).
        let ens = a.addr_of(exit_ns);
        a.b_to(ens);
        for f in to_xfer {
            a.patch(f, p);
        }
        Some(p)
    } else {
        None
    };

    // ras_miss(x0 = guest return target, XRAS = RAS top whose entry did not match): the
    // entry for this return may sit up to 3 entries deeper (frames left without a
    // translated `ret`: longjmp/SEH unwinds, rets run by the interpreter, host-called guest
    // code). Pop through the first match and return to its landing pad; otherwise keep the
    // stack as is (the return has no entry, e.g. its call was interpreted) and probe.
    // Any entry whose guest address equals the target is a correct continuation: its
    // landing pad transfers to exactly that guest IP.
    let ras_miss = if ras {
        if let Some(want) = cfg.target_ras_miss {
            while a.pos < want && !a.overflow {
                a.emit(0xD503_201F); // nop padding to match .aot stub table offset exactly
            }
        }
        let p = a.pos;
        for f in to_ras_miss {
            a.patch(f, p);
        }
        let mut found = Vec::new();
        for i in 1..=3u32 {
            a.addsub_ri(true, true, false, XA, XRAS, 16 * i, false);
            a.mov_rr(true, 1, XRAS);
            a.bfxil0(1, XA, 16);
            a.ldr(64, 2, 1, 0);
            a.logic_rr(true, 2, 2, 2, 0, 0, 0); // eor x2, x2, x0
            found.push(a.cbz(true, 2));
        }
        let lr = a.addr_of(lookup_rip);
        a.b_to(lr);
        for f in found {
            a.bind(f);
        }
        // x1 = matching entry: new top = the entry below it.
        a.ldr(64, 2, 1, 8);
        a.addsub_ri(true, true, false, XA, 1, 16, false);
        a.mov_rr(true, XRAS, 1);
        a.bfxil0(XRAS, XA, 16);
        a.br(2);
        Some(p)
    } else {
        None
    };

    (
        Stubs { enter, exit, lookup_rip, lookup_jmp, lookup_call, host_xfer, ras_miss, spill_xmm, reload_xmm, clear_ymm_hi, rdtsc_slow, rdtscp_slow },
        host_xfer_bl,
    )
}

/// Push the shadow RAS entry (guest return address in register `g`, host landing pad in
/// register `h`). Updates pinned `XRAS` in place; clobbers x16.
pub(super) fn ras_push(a: &mut Asm, g: u32, h: u32) {
    a.addsub_ri(true, false, false, XA, XRAS, 16, false);
    a.bfxil0(XRAS, XA, 16);
    a.stp_x(g, h, XRAS, 0);
}

/// Word positions of the shared stubs.
pub(super) struct Stubs {
    pub(super) enter: usize,
    pub(super) exit: usize,
    pub(super) lookup_rip: usize,
    pub(super) lookup_jmp: usize,
    pub(super) lookup_call: usize,
    pub(super) ras_miss: Option<usize>,
    pub(super) host_xfer: Option<usize>,
    pub(super) spill_xmm: Option<usize>,
    pub(super) reload_xmm: Option<usize>,
    pub(super) clear_ymm_hi: usize,
    pub(super) rdtsc_slow: usize,
    pub(super) rdtscp_slow: usize,
}

impl Stubs {
    pub(super) fn offsets(&self) -> AotStubOffs {
        let word = |p: usize| p as u32;
        [
            word(self.enter), word(self.exit), word(self.lookup_rip), word(self.lookup_jmp),
            word(self.lookup_call), self.host_xfer.map_or(u32::MAX, word),
            self.ras_miss.map_or(u32::MAX, word), self.spill_xmm.map_or(u32::MAX, word),
            self.reload_xmm.map_or(u32::MAX, word), word(self.clear_ymm_hi),
            word(self.rdtsc_slow), word(self.rdtscp_slow),
        ]
    }
}

/// `AW2_JIT_HOSTCALL=0` makes host calls/jmps from translated code leave translated code
/// (`EXIT_CALL`/`EXIT_JMP`, dispatched by the interpreter loop) instead of calling the host
/// function from the `host_xfer` stub / inline. Default on.
pub(super) fn hostcall_enabled() -> bool {
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| {
        let on = std::env::var("AW2_JIT_HOSTCALL").map_or(true, |s| s.trim() != "0");
        log::info!("emu jit: in-translation host calls {}", if on { "on" } else { "off (AW2_JIT_HOSTCALL=0)" });
        on
    })
}

/// `AW2_JIT_PLAINCALL=0` disables the inline call sequence for `DF_PLAIN` builtins
/// (memcpy/memset/SRW locks/...) in translated code; they then go through `host_xfer`.
/// Default on; implied off by `AW2_JIT_HOSTCALL=0` and while the interpreter profiler is on.
pub(in crate::emu) fn plaincall_enabled() -> bool {
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| {
        hostcall_enabled()
            && std::env::var("AW2_JIT_PLAINCALL").map_or(true, |s| s.trim() != "0")
            && crate::emu_prof::level() == 0
            && !crate::diag::nan_trace_installed()
    })
}
