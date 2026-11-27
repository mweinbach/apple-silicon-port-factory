//! Shared JIT/AOT trampoline stubs (`enter`, `exit`, `lookup_rip`, `lookup_jmp`, `lookup_call`,
//! `host_xfer`, `ras_miss`, `spill_xmm`, `reload_xmm`, `clear_ymm_hi`).

use super::cache::JSET_BITS;
use super::emit::{enc_logic_imm, Asm};
use super::translate::stack_tso;
use super::xmm::xmm_pin_enabled;
use super::*;
use crate::emu::mem;

const _: () = assert!(win32_rt::teb::off::CLIENT_ID_THREAD == 0x48);
const _: () = assert!(win32_rt::teb::off::LAST_ERROR == 0x68);
const _: () = assert!(win32_rt::teb::off::TLS_SLOTS == 0x1480);
const _: () = assert!(win32_rt::teb::off::TLS_EXPANSION_SLOTS == 0x1780);
const _: () = assert!(win32_rt::teb::TLS_MINIMUM_AVAILABLE == 64);
const _: () = assert!(win32_rt::teb::TLS_EXPANSION_SLOTS == 1024);
const _: () = assert!(win32_rt::sync::SRW_WRITER == 1);
const _: () = assert!(win32_rt::sync::SRW_PARKED == 2);
const _: () = assert!(win32_rt::sync::SRW_WAITING_WRITER == 4);
const _: () = assert!(win32_rt::sync::SRW_READER == 8);

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
    // When `xpin` is active, guest `xmm0..15` stay pinned in `v16..v31` AND `ymm_hi[0..7]`
    // stay pinned in `v8..v15`. Save callee-saved `q8..q15` (superset of AAPCS64 `d8..d15`)
    // in the frame and load pinned vector state via `reload_xmm`.
    let enter = a.pos;
    let frame_qwords: i32 = if xpin { 28 } else { 12 }; // 224B vs 96B (16B-aligned)
    a.emit(0xA980_0000 | ((-frame_qwords as u32) & 0x7F) << 15 | 30 << 10 | 31 << 5 | 29); // stp x29, x30, [sp, #-frame]!
    a.emit(0x9100_03FD); // mov x29, sp
    a.stp_x(19, 20, 31, 16);
    a.stp_x(21, 22, 31, 32);
    a.stp_x(23, 24, 31, 48);
    a.stp_x(25, 26, 31, 64);
    a.stp_x(27, 28, 31, 80);
    if xpin {
        for i in 0..4u32 {
            a.stp_q(8 + 2 * i, 9 + 2 * i, 31, (96 + 32 * i) as i32);
        }
    }
    a.mov_rr(true, XCPU, 0);
    for i in 0..8 {
        a.ldp_x(HG[2 * i], HG[2 * i + 1], XCPU, (OFF_GPR + 16 * i as u32) as i32);
    }
    if ras {
        a.ldr(64, XRAS, XCPU, OFF_RAS);
    }
    // Pack `XBUD = (budget << 1) | (ymm_dirty != 0)` so VEX-128 / `vzeroupper` / `spill_xmm` /
    // `reload_xmm` checks are a single register-only `tbz`/`tbnz x8, #0` while `sub x8, #(2*n)`
    // and `tbnz x8, #63` keep exact budget semantics without touching bit 0.
    a.ldr(64, XBUD, XCPU, OFF_BUDGET);
    a.lsl_i(true, XBUD, XBUD, 1);
    a.ldr(16, XS, XCPU, OFF_YMM_DIRTY);
    a.addsub_ri(false, true, true, ZR, XS, 0, false); // cmp w17, #0
    a.cset(XS, NE);
    a.logic_rr(true, 1, XBUD, XBUD, XS, 0, 0); // orr x8, x8, x17
    let enter_reload = xpin.then(|| a.bl());
    a.br(1);

    // exit (w0 = exit code; rip / target already stored)
    let exit = a.pos;
    for i in 0..8 {
        a.stp_x(HG[2 * i], HG[2 * i + 1], XCPU, (OFF_GPR + 16 * i as u32) as i32);
    }
    let exit_spill = xpin.then(|| a.bl());
    if ras {
        a.str(64, XRAS, XCPU, OFF_RAS);
    }
    a.asr_i(true, XA, XBUD, 1);
    a.str(64, XA, XCPU, OFF_BUDGET);
    // exit_ns: same, with the GPRs, pinned vectors, and budget already stored by the caller.
    let exit_ns = a.pos;
    if xpin {
        for i in 0..4u32 {
            a.ldp_q(8 + 2 * i, 9 + 2 * i, 31, (96 + 32 * i) as i32);
        }
    }
    a.ldp_x(19, 20, 31, 16);
    a.ldp_x(21, 22, 31, 32);
    a.ldp_x(23, 24, 31, 48);
    a.ldp_x(25, 26, 31, 64);
    a.ldp_x(27, 28, 31, 80);
    a.emit(0xA8C0_0000 | ((frame_qwords as u32) & 0x7F) << 15 | 30 << 10 | 31 << 5 | 29); // ldp x29, x30, [sp], #frame
    a.ret();

    // Shared XMM + pinned YMM_HI[0..7] spill/reload helpers (preserves all GPRs, q0..q7, and NZCV).
    // `spill_xmm` skips the 4 `stp q` of `v8..v15` when `XBUD` bit 0 == 0 (no dirty upper YMM lanes);
    // `spill_xmm_full` (`exit` / `host_xfer`) always stores `v8..v15` so `Cpu::ymm_hi[0..7]` stays
    // exact in memory after `clear_ymm_hi`.
    // `reload_xmm` skips the 4 `ldp q` of `v8..v15` when `XBUD` bit 0 == 0 and instead re-zeros
    // `v8..v15` via zero-cycle `movi #0` (AAPCS64 callees may clobber upper 64 bits of `q8..q15`).
    let (spill_xmm, spill_xmm_full, reload_xmm) = if xpin {
        let sp_pos = a.pos;
        let sp_skip_ymm = a.tbz(XBUD, 0);
        let sp_full_pos = a.pos;
        for i in 0..4u32 {
            a.stp_q(8 + 2 * i, 9 + 2 * i, XCPU, (OFF_YMM_HI + 32 * i) as i32);
        }
        a.bind(sp_skip_ymm);
        for i in 0..8u32 {
            a.stp_q(16 + 2 * i, 17 + 2 * i, XCPU, (OFF_XMM + 32 * i) as i32);
        }
        a.ret();

        let rl_pos = a.pos;
        for i in 0..8u32 {
            a.ldp_q(16 + 2 * i, 17 + 2 * i, XCPU, (OFF_XMM + 32 * i) as i32);
        }
        let rl_zero_ymm = a.tbz(XBUD, 0);
        for i in 0..4u32 {
            a.ldp_q(8 + 2 * i, 9 + 2 * i, XCPU, (OFF_YMM_HI + 32 * i) as i32);
        }
        a.ret();
        a.bind(rl_zero_ymm);
        for r in 0..8u32 {
            a.v_zero(8 + r);
        }
        a.ret();
        if let Some(f) = enter_reload {
            a.patch(f, rl_pos);
        }
        if let Some(f) = exit_spill {
            a.patch(f, sp_full_pos);
        }
        (Some(sp_pos), Some(sp_full_pos), Some(rl_pos))
    } else {
        (None, None, None)
    };

    // Shared VEX-128 / `vzeroupper` lazy `Cpu::ymm_hi` zeroing stub: clears `ymm_hi[r]`
    // (register `v(8+r)` for `r < 8` when `xpin`, else `Cpu::ymm_hi[r]` in memory) for every
    // bit `r` set in BOTH `w16` (mask) and `Cpu::jx.ymm_dirty`, updates `Cpu::jx.ymm_dirty`,
    // and clears bit 0 of `XBUD` once `ymm_dirty` reaches 0.
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
    let ym_hi_done = if xpin {
        for r in 0..8u32 {
            let skip = a.tbz(XA, r);
            a.v_zero(8 + r);
            a.bind(skip);
        }
        a.logic_ri(false, 0, XA, XA, enc_logic_imm(0xFF00, false).unwrap());
        Some(a.cbz(false, XA))
    } else {
        None
    };
    let ym_loop = a.pos;
    a.rbit(false, XS, XA);
    a.clz(false, XS, XS);
    a.addsub_rr(true, false, false, XS, XCPU, XS, 4); // x17 = XCPU + r*16
    a.stp_x(ZR, ZR, XS, OFF_YMM_HI as i32);
    a.addsub_ri(false, true, false, XS, XA, 1, false);
    a.logic_rr(false, 0, XA, XA, XS, 0, 0); // and w16, w16, w17
    let ym_back = a.cbnz(false, XA);
    a.patch(ym_back, ym_loop);
    if let Some(done) = ym_hi_done {
        a.bind(done);
    }
    a.ret();
    a.bind(ym_done);
    let ym_ret = a.cbnz(false, XS);
    a.logic_ri(true, 0, XBUD, XBUD, enc_logic_imm(!1u64, true).unwrap());
    a.bind(ym_ret);
    a.ret();

    let (rdtsc_slow, rdtscp_slow) = emit_timestamp_stubs(a);

    // 2-way set-associative hash probe of x0 -> on hit falls through with x1 = fast and
    // w17 = gen (0 for static/AOT blocks, != 0 for SMC); on miss branches to the returned
    // fixups. Clobbers x1, x2, x16, x17 and NZCV; preserves x0, x3 (T3), x4 (T4), x30.
    let probe = |a: &mut Asm| -> [usize; 3] {
        a.ldr(64, XA, XCPU, OFF_TABLE);
        a.ldr(64, XS, XCPU, OFF_TABLE + 8);
        a.mul(true, XS, 0, XS);
        a.lsr_i(true, XS, XS, 64 - JSET_BITS);
        a.addsub_rr(true, false, false, XA, XA, XS, 6); // 64B set (2 x 32B JEnt)
        a.ldp_x(XS, 1, XA, 0); // way 0: (ip, fast)
        a.addsub_rr(true, true, true, ZR, XS, 0, 0); // cmp x17, x0
        let hit0 = a.bcond(EQ);
        a.addsub_ri(true, false, false, XA, XA, 32, false); // way 1
        a.ldp_x(XS, 1, XA, 0);
        a.addsub_rr(true, true, true, ZR, XS, 0, 0); // cmp x17, x0
        let m1 = a.bcond(NE);
        a.bind(hit0);
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

    // Populate the per-site 2-way IC at `[T4]` (x4) on a static/AOT (`w17 == 0`) guest block
    // hit (`x0` = target guest IP, `x1` = host code addr). FIFO-rotates way 0 -> way 1 so
    // 2-target polymorphic sites stay read-only in steady state.
    let emit_ic_populate = |a: &mut Asm| {
        let skip_null = a.cbz(true, T4);
        let skip_smc = a.cbnz(false, XS);
        a.ldp_x(2, XS, T4, 0);
        a.stp_x(2, XS, T4, 16);
        a.stp_x(0, 1, T4, 0);
        a.bind(skip_null);
        a.bind(skip_smc);
    };

    // Shared AOT probe table (`aot_probe_enabled`; layout: `aot::probe_table`) search of x0
    // after a `probe` miss: on a live entry branches to `hit` (word position of the stub's
    // hit path, x1 = code, w17 = 0); otherwise falls through (and returns the branches to bind
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
        // Populate this thread's 2-way `table[jset_base(x0)]` inline (FIFO rotate way 0 -> way 1;
        // install `gen = 0, hits = 0, code = x1` in way 0) so repeated transfers hit in O(1).
        a.ldr(64, XA, XCPU, OFF_TABLE);
        a.ldr(64, XS, XCPU, OFF_TABLE + 8);
        a.mul(true, XS, 0, XS);
        a.lsr_i(true, XS, XS, 64 - JSET_BITS);
        a.addsub_rr(true, false, false, XA, XA, XS, 6);
        a.ldp_x(2, XS, XA, 0);
        a.stp_x(2, XS, XA, 32);
        a.ldp_x(2, XS, XA, 16);
        a.stp_x(2, XS, XA, 48);
        a.stp_x(0, 1, XA, 0);
        a.stp_x(ZR, 1, XA, 16);
        a.movz(false, XS, 0, 0); // gen = 0 so `hit` may populate `[T4]`
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

    // lookup_rip(x0 = guest rip) / lookup_rip_ic(x0 = guest rip, x4 = &ic_table[slot]):
    // continue there (populating `[x4]` on static hit), or exit with EXIT_RIP.
    let lookup_rip = a.pos;
    a.movz(true, T4, 0, 0);
    let lookup_rip_ic = a.pos;
    let miss = probe(a);
    let m_plain_rip = a.tbnz(1, 0);
    let hit = a.pos;
    emit_ic_populate(a);
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

    // lookup_jmp(x0 = target) / lookup_jmp_ic(x0 = target, x4 = &ic_table[slot])
    // Entries in `table` with bit 0 of `fast` set are cached `DF_PLAIN` / pure-float-math /
    // inline-QPC host functions dispatched directly in `is_plain_jmp` / `is_qpc_jmp`.
    let lookup_jmp = a.pos;
    a.movz(true, T4, 0, 0);
    let lookup_jmp_ic = a.pos;
    let miss = probe(a);
    let is_plain_jmp = a.tbnz(1, 0);
    let hit = a.pos;
    emit_ic_populate(a);
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

    // lookup_call(x0 = target, x3 = return address) /
    // lookup_call_ic(x0 = target, x3 = return address, x4 = &ic_table[slot]):
    // push + continue (populating `[x4]` on static hit), or EXIT_CALL.
    // With the RAS, call sites enter with `bl lookup_call[_ic]` (x30 = their landing pad).
    // Entries in `table` with bit 0 of `fast` set are cached `DF_PLAIN` / pure-float-math /
    // inline-QPC host functions dispatched directly here.
    let lookup_call = a.pos;
    a.movz(true, T4, 0, 0);
    let lookup_call_ic = a.pos;
    let miss = probe(a);
    let is_plain = a.tbnz(1, 0);
    let hit = a.pos;
    emit_ic_populate(a);
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

    // Fast path (`lookup_call` + `lookup_jmp`) for tagged `DF_PLAIN` (bit 0), pure-float-math
    // (bits 0..1), and zero-spill inline fast thunks (bit 63 | kind<<59 | bit 0) cached in `table`.
    // Bit 58 of `x1` is set transiently by `lookup_jmp` to distinguish tail jumps from calls.
    let mut to_ras_miss: Vec<usize> = Vec::new();
    let ptr_mask = enc_logic_imm(0x0000_FFFF_FFFF_FFFC, true).unwrap();
    let align7 = enc_logic_imm(7, true).unwrap();
    let jmp_bit58 = enc_logic_imm(1u64 << 58, true).unwrap();

    a.bind(is_plain);
    let is_inline_call = a.tbnz(1, 63);
    let plain_shared = a.pos;
    a.logic_ri(true, 0, XA, 1, ptr_mask); // strip bits 63..48 (incl. bit 58) and bits 1..0
    a.addsub_ri(true, true, false, 31, 31, 96, false); // sub sp, sp, #96
    a.stp_x(8, 9, 31, 0);
    a.stp_x(10, 11, 31, 16);
    a.stp_x(12, 13, 31, 32);
    a.stp_x(14, 15, 31, 48);
    a.stp_x(30, T3, 31, 64);
    a.str(64, 1, 31, 88); // preserve tagged host pointer (bit 1 = needs_xmm, bit 58 = is_jmp)
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
    // Pure-integer `DF_PLAIN` callees (`needs_xmm == 0`, bit 1 == 0) skip `mrs fpcr` /
    // `msr fpcr` completely; float-ABI & stdio callees (bit 1 == 1) use `call_with_host_fpcr`.
    a.ldr(64, 9, 31, 88);
    let fpcr_call = a.tbnz(9, 1);
    a.blr(XA);
    let plain_call_done = a.b();
    a.bind(fpcr_call);
    a.call_with_host_fpcr(80);
    a.bind(plain_call_done);
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
    let plain_ret_jmp = a.tbnz(XS, 58);
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
    a.bind(plain_ret_jmp);
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

    // Fast path (`lookup_jmp`) entry: tag bit 58 and share `is_inline_*` / `plain_shared`.
    a.bind(is_plain_jmp);
    a.logic_ri(true, 1, 1, 1, jmp_bit58); // bit 58 = lookup_jmp
    let is_inline_jmp = a.tbnz(1, 63);
    a.ldr_post(T3, g(4), 8); // pop guest return address; g(4) now points to Win64 shadow space
    let to_plain = a.b();
    a.patch(to_plain, plain_shared);

    // Shared zero-spill inline fast-thunk dispatcher for both `lookup_call` (bit 58 == 0)
    // and `lookup_jmp` (bit 58 == 1). Preserves `x0` (target VA), `x1` (tag + bit 58),
    // `T3` (return address for call), `x30` (landing pad for call), `g(1..15)`, and all XMMs.
    // Free scratch: `x2`, `XA`, `XS` (and `g(0)` as volatile Win64 return register).
    a.bind(is_inline_call);
    a.bind(is_inline_jmp);
    let mut to_inline_done: Vec<usize> = Vec::new();
    let mut to_inline_bail: Vec<usize> = Vec::new();
    a.ubfx(2, 1, 59, 4); // x2 = FAST_KIND_* (0..=10)

    // Kinds 0 (QPC) & 1 (TIME_MS): shared `cntvct_el0` + Q64 scale read.
    a.addsub_ri(false, true, true, ZR, 2, 2, false); // cmp w2, #2
    let not_time = a.bcond(HS);
    a.logic_ri(true, 0, XA, 1, ptr_mask); // XA = &QpcFastState
    a.ldp_x(2, XS, XA, 0); // x2 = mode, x17 = mult
    a.addsub_ri(true, true, true, ZR, 2, 1, false); // cmp x2, #1
    to_inline_bail.push(a.bcond(NE));
    a.mrs_cntvct(2);
    a.ldr(64, XA, XA, 16); // x16 = offset
    a.umulh(2, 2, XS);
    a.addsub_rr(true, false, false, 2, 2, XA, 0); // x2 = interrupt_time_100ns
    let is_time_ms = a.tbnz(1, 59);
    // KIND_QPC (0): *lpPerformanceCount = x2; return TRUE.
    a.str(64, 2, g(1), 0);
    a.movz(false, g(0), 1, 0);
    to_inline_done.push(a.b());
    // KIND_TIME_MS (1): x2 / 10_000 (u32 for timeGetTime/GetTickCount, u64 for GetTickCount64).
    a.bind(is_time_ms);
    a.movz(false, XS, 10_000, 0);
    a.udiv(true, 2, 2, XS);
    let is_time_ms64 = a.tbnz(1, 1);
    a.mov_rr(false, g(0), 2); // zero-extend u32 -> rax (wraps mod 2^32 ms)
    to_inline_done.push(a.b());
    a.bind(is_time_ms64);
    a.mov_rr(true, g(0), 2);
    to_inline_done.push(a.b());

    // Kind 2: EnterCriticalSection / RtlEnterCriticalSection (`g(1)` = `cs`).
    a.bind(not_time);
    a.addsub_ri(false, true, true, ZR, 2, FAST_KIND_ENTER_CS as u32, false);
    let not_enter_cs = a.bcond(NE);
    to_inline_bail.push(a.cbz(true, g(1)));
    a.logic_ri(true, 3, ZR, g(1), align7); // tst cs, #7
    to_inline_bail.push(a.bcond(NE));
    a.ldr(64, XA, XCPU, OFF_TEB);
    to_inline_bail.push(a.cbz(true, XA));
    a.ldr(32, XA, XA, 0x48); // w16 = TEB.ClientId.UniqueThread (zero-extended)
    to_inline_bail.push(a.cbz(false, XA));
    a.ldr(64, XS, g(1), 0x10); // x17 = cs.owning_thread
    a.addsub_rr(true, true, true, ZR, XS, XA, 0); // cmp owning_thread, tid
    let enter_cs_rec = a.bcond(EQ);
    a.addsub_ri(true, false, false, XS, g(1), 8, false); // x17 = &cs.lock_count
    a.movn(false, 2, 0, 0); // w2 = -1 (expected)
    a.casa(32, 2, ZR, XS); // Acquire CAS -1 -> 0
    a.addsub_ri(false, false, true, ZR, 2, 1, false); // cmn w2, #1
    to_inline_bail.push(a.bcond(NE));
    a.str(64, XA, g(1), 0x10); // cs.owning_thread = tid
    a.movz(false, 2, 1, 0);
    a.str(32, 2, g(1), 0x0c); // cs.recursion_count = 1
    a.movz(false, g(0), 0, 0); // STATUS_SUCCESS
    to_inline_done.push(a.b());
    a.bind(enter_cs_rec);
    a.ldr(32, 2, g(1), 0x0c); // w2 = cs.recursion_count
    a.addsub_ri(false, true, true, ZR, 2, 0, false); // cmp w2, #0
    to_inline_bail.push(a.bcond(LE));
    a.addsub_ri(false, false, true, 2, 2, 1, false); // adds w2, w2, #1
    to_inline_bail.push(a.bcond(VS));
    a.str(32, 2, g(1), 0x0c);
    a.movz(false, g(0), 0, 0);
    to_inline_done.push(a.b());

    // Kind 3: LeaveCriticalSection / RtlLeaveCriticalSection (`g(1)` = `cs`).
    a.bind(not_enter_cs);
    a.addsub_ri(false, true, true, ZR, 2, FAST_KIND_LEAVE_CS as u32, false);
    let not_leave_cs = a.bcond(NE);
    to_inline_bail.push(a.cbz(true, g(1)));
    a.logic_ri(true, 3, ZR, g(1), align7); // tst cs, #7
    to_inline_bail.push(a.bcond(NE));
    a.ldr(32, 2, g(1), 0x0c); // w2 = cs.recursion_count
    a.addsub_ri(false, true, true, ZR, 2, 1, false); // cmp w2, #1
    to_inline_bail.push(a.bcond(LT));
    let leave_cs_rec = a.bcond(GT);
    // Outermost leave (recursion_count == 1): clear fields, Release CAS 0 -> -1.
    // On contention (lock_count != 0), restore fields while lock stays held and bail to slow.
    a.ldr(64, XA, g(1), 0x10); // save owning_thread
    a.str(32, ZR, g(1), 0x0c); // recursion_count = 0
    a.str(64, ZR, g(1), 0x10); // owning_thread = 0
    a.addsub_ri(true, false, false, XS, g(1), 8, false); // &cs.lock_count
    a.movz(false, 2, 0, 0); // expected = 0 (uncontended)
    a.movn(false, g(0), 0, 0); // desired = -1 (free)
    a.casl(32, 2, g(0), XS); // Release CAS 0 -> -1
    let leave_cs_contended = a.cbnz(false, 2);
    a.movz(false, g(0), 0, 0); // STATUS_SUCCESS
    to_inline_done.push(a.b());
    a.bind(leave_cs_contended);
    a.movz(false, 2, 1, 0);
    a.str(32, 2, g(1), 0x0c);
    a.str(64, XA, g(1), 0x10);
    to_inline_bail.push(a.b());
    a.bind(leave_cs_rec);
    a.addsub_ri(false, true, false, 2, 2, 1, false); // w2 -= 1
    a.str(32, 2, g(1), 0x0c);
    a.movz(false, g(0), 0, 0);
    to_inline_done.push(a.b());

    // Kind 4: XInputGetState (bit 1 == 0) / XInputGetStateEx (bit 1 == 1).
    // `w_g1` = `user`, `g(2)` = `out: *mut State`.
    a.bind(not_leave_cs);
    a.addsub_ri(false, true, true, ZR, 2, FAST_KIND_XINPUT as u32, false);
    let not_xinput = a.bcond(NE);
    to_inline_bail.push(a.cbz(true, g(2)));
    a.addsub_ri(false, true, true, ZR, g(1), 4, false); // cmp w_user, #4
    to_inline_bail.push(a.bcond(HS));
    a.logic_ri(true, 0, XA, 1, ptr_mask); // XA = &XInputFastTable
    a.ldr(64, 2, XA, 0); // ready
    to_inline_bail.push(a.cbz(true, 2));
    a.addsub_ri(true, false, false, XA, XA, 32, false); // &slots[0]
    a.mov_rr(false, 2, g(1)); // zero-extend u32 user
    a.addsub_rr(true, false, false, XA, XA, 2, 5); // &slots[user] (32B stride)
    a.ldar(32, 2, XA); // w2 = seq1 (Acquire)
    to_inline_bail.push(a.tbnz(2, 0)); // odd => writer active
    a.ldr(32, XS, XA, 4); // connected
    let xinput_conn = a.cbnz(false, XS);
    a.dmb_ishld();
    a.ldr(32, XS, XA, 0); // seq2
    a.addsub_rr(false, true, true, ZR, 2, XS, 0);
    to_inline_bail.push(a.bcond(NE));
    a.movz(false, g(0), 1167, 0); // ERROR_DEVICE_NOT_CONNECTED
    to_inline_done.push(a.b());
    a.bind(xinput_conn);
    a.ldp_x(XS, g(0), XA, 8); // XS = w0, g(0) = w1
    a.dmb_ishld();
    a.ldr(32, XA, XA, 0); // XA = seq2
    a.addsub_rr(false, true, true, ZR, 2, XA, 0);
    to_inline_bail.push(a.bcond(NE));
    let xinput_keep_guide = a.tbnz(1, 1); // bit 1 == 1 -> XInputGetStateEx
    a.logic_ri(true, 0, XS, XS, enc_logic_imm(!(1u64 << 42), true).unwrap());
    a.bind(xinput_keep_guide);
    a.str(64, XS, g(2), 0);
    a.str(64, g(0), g(2), 8);
    a.movz(false, g(0), 0, 0); // ERROR_SUCCESS
    to_inline_done.push(a.b());

    // Kind 5: SwitchToThread / _Thrd_yield (63 inline `isb` yields returning FALSE, every 64th
    // bails to `SwitchToThread()` -> `libc::sched_yield()` returning TRUE).
    a.bind(not_xinput);
    a.addsub_ri(false, true, true, ZR, 2, FAST_KIND_SWITCH_TO_THREAD as u32, false);
    let not_switch = a.bcond(NE);
    a.ldr(32, 2, XCPU, OFF_SWITCH_YIELD);
    a.addsub_ri(false, false, false, 2, 2, 1, false);
    a.str(32, 2, XCPU, OFF_SWITCH_YIELD);
    a.logic_ri(false, 3, ZR, 2, enc_logic_imm(63, false).unwrap()); // tst w2, #63
    to_inline_bail.push(a.bcond(EQ));
    a.isb();
    a.movz(false, g(0), 0, 0); // FALSE (no OS context switch occurred)
    to_inline_done.push(a.b());

    // Kind 6: GetCurrentThreadId / _Thrd_id (bit 1 == 0) & GetLastError (bit 1 == 1).
    a.bind(not_switch);
    a.addsub_ri(false, true, true, ZR, 2, FAST_KIND_THREAD_ID as u32, false);
    let not_thread_id = a.bcond(NE);
    a.ldr(64, XA, XCPU, OFF_TEB);
    to_inline_bail.push(a.cbz(true, XA));
    let is_last_err = a.tbnz(1, 1);
    a.ldr(32, XS, XA, 0x48); // TEB.ClientId.UniqueThread (zero-extended)
    to_inline_bail.push(a.cbz(false, XS));
    a.mov_rr(false, g(0), XS);
    to_inline_done.push(a.b());
    a.bind(is_last_err);
    a.ldr(32, g(0), XA, 0x68); // TEB.LastErrorValue (zero-extended)
    to_inline_done.push(a.b());

    // Kind 7: TlsGetValue (bit 1 == 0) & TlsSetValue (bit 1 == 1).
    // `w_g1` = `i: u32`, `g(2)` = `v: *mut c_void` (for TlsSetValue).
    a.bind(not_thread_id);
    a.addsub_ri(false, true, true, ZR, 2, FAST_KIND_TLS as u32, false);
    let not_tls = a.bcond(NE);
    a.ldr(64, XA, XCPU, OFF_TEB);
    to_inline_bail.push(a.cbz(true, XA));
    a.mov_rr(false, 2, g(1)); // zero-extend u32 slot index into x2
    a.addsub_ri(false, true, true, ZR, 2, 64, false); // cmp w2, #64
    let tls_exp = a.bcond(HS);
    a.addsub_rr(true, false, false, XS, XA, 2, 3); // x17 = TEB + i*8
    let tls_set_low = a.tbnz(1, 1);
    a.str(32, ZR, XA, 0x68); // TEB.LastErrorValue = 0
    a.ldr(64, g(0), XS, 0x1480); // TEB.TlsSlots[i]
    to_inline_done.push(a.b());
    a.bind(tls_set_low);
    a.str(64, g(2), XS, 0x1480); // TEB.TlsSlots[i] = v
    a.movz(false, g(0), 1, 0); // TRUE
    to_inline_done.push(a.b());
    a.bind(tls_exp);
    a.addsub_ri(false, true, false, 2, 2, 64, false); // w2 = i - 64 (zero-extended)
    a.addsub_ri(false, true, true, ZR, 2, 1024, false); // cmp w2, #1024
    to_inline_bail.push(a.bcond(HS));
    a.ldr(64, XS, XA, 0x1780); // TEB.TlsExpansionSlots
    to_inline_bail.push(a.cbz(true, XS));
    a.addsub_rr(true, false, false, XS, XS, 2, 3); // &exp[i - 64]
    let tls_set_exp = a.tbnz(1, 1);
    a.str(32, ZR, XA, 0x68); // TEB.LastErrorValue = 0
    a.ldr(64, g(0), XS, 0);
    to_inline_done.push(a.b());
    a.bind(tls_set_exp);
    a.str(64, g(2), XS, 0);
    a.movz(false, g(0), 1, 0); // TRUE
    to_inline_done.push(a.b());

    // Kind 8: AcquireSRWLockExclusive (bit 1 == 0) & TryAcquireSRWLockExclusive (bit 1 == 1).
    a.bind(not_tls);
    a.addsub_ri(false, true, true, ZR, 2, FAST_KIND_SRW_ACQ_EXCL as u32, false);
    let not_srw_acq_excl = a.bcond(NE);
    to_inline_bail.push(a.cbz(true, g(1)));
    a.logic_ri(true, 3, ZR, g(1), align7);
    to_inline_bail.push(a.bcond(NE));
    a.movz(true, 2, 0, 0); // expected = 0
    a.movz(true, XS, 1, 0); // desired = SRW_WRITER (1)
    a.casa(64, 2, XS, g(1)); // Acquire CAS 0 -> 1
    let srw_excl_miss = a.cbnz(true, 2);
    a.movz(false, g(0), 1, 0); // TRUE (for TryAcquire; ignored by void Acquire)
    to_inline_done.push(a.b());
    a.bind(srw_excl_miss);
    to_inline_bail.push(a.tbz(1, 1)); // AcquireSRWLockExclusive -> slow spin/park
    a.logic_ri(true, 3, ZR, 2, enc_logic_imm(!6u64, true).unwrap()); // (s & (WRITER|READERS)) != 0?
    to_inline_bail.push(a.bcond(EQ)); // parked-only -> let slow CAS s -> s|1
    a.movz(false, g(0), 0, 0); // TryAcquireSRWLockExclusive -> FALSE
    to_inline_done.push(a.b());

    // Kind 9: AcquireSRWLockShared (bit 1 == 0) & TryAcquireSRWLockShared (bit 1 == 1).
    a.bind(not_srw_acq_excl);
    a.addsub_ri(false, true, true, ZR, 2, FAST_KIND_SRW_ACQ_SHARED as u32, false);
    let not_srw_acq_sh = a.bcond(NE);
    to_inline_bail.push(a.cbz(true, g(1)));
    a.logic_ri(true, 3, ZR, g(1), align7);
    to_inline_bail.push(a.bcond(NE));
    a.ldr(64, 2, g(1), 0); // s = load(Relaxed)
    let srw_sh_busy0 = a.tbnz(2, 0); // SRW_WRITER
    let srw_sh_busy1 = a.tbnz(2, 2); // SRW_WAITING_WRITER
    a.addsub_ri(true, false, false, XS, 2, 8, false); // desired = s + SRW_READER
    a.mov_rr(true, XA, 2); // save expected
    a.casa(64, 2, XS, g(1)); // Acquire CAS s -> s + 8
    a.addsub_rr(true, true, true, ZR, 2, XA, 0); // cmp old, expected
    to_inline_bail.push(a.bcond(NE));
    a.movz(false, g(0), 1, 0); // TRUE
    to_inline_done.push(a.b());
    a.bind(srw_sh_busy0);
    a.bind(srw_sh_busy1);
    to_inline_bail.push(a.tbz(1, 1)); // AcquireSRWLockShared -> slow spin/park
    a.movz(false, g(0), 0, 0); // TryAcquireSRWLockShared -> FALSE
    to_inline_done.push(a.b());

    // Kind 10: ReleaseSRWLockExclusive (bit 1 == 0) & ReleaseSRWLockShared (bit 1 == 1).
    a.bind(not_srw_acq_sh);
    a.addsub_ri(false, true, true, ZR, 2, FAST_KIND_SRW_REL as u32, false);
    to_inline_bail.push(a.bcond(NE));
    to_inline_bail.push(a.cbz(true, g(1)));
    a.logic_ri(true, 3, ZR, g(1), align7);
    to_inline_bail.push(a.bcond(NE));
    let srw_rel_sh = a.tbnz(1, 1);
    // ReleaseSRWLockExclusive: Release CAS 1 -> 0 (untouched when parked/waiting).
    a.movz(true, 2, 1, 0);
    a.casl(64, 2, ZR, g(1));
    a.addsub_ri(true, true, true, ZR, 2, 1, false);
    to_inline_bail.push(a.bcond(NE));
    to_inline_done.push(a.b());
    // ReleaseSRWLockShared: if (s & 7) == 0 && s != 0, Release CAS s -> s - 8.
    a.bind(srw_rel_sh);
    a.ldr(64, 2, g(1), 0);
    to_inline_bail.push(a.cbz(true, 2));
    a.logic_ri(true, 3, ZR, 2, align7);
    to_inline_bail.push(a.bcond(NE));
    a.addsub_ri(true, true, false, XS, 2, 8, false);
    a.mov_rr(true, XA, 2);
    a.casl(64, 2, XS, g(1));
    a.addsub_rr(true, true, true, ZR, 2, XA, 0);
    to_inline_bail.push(a.bcond(NE));

    // Shared fast-path return epilogue (`lookup_call` vs `lookup_jmp` via bit 58 of x1).
    let inline_fast_done = a.pos;
    for f in to_inline_done {
        a.patch(f, inline_fast_done);
    }
    if lb_on {
        a.str(8, ZR, XCPU, OFF_LB);
    }
    #[cfg(test)]
    if cfg.count_plain_hits {
        a.ldr(64, 2, XCPU, OFF_PLAINHITS);
        a.addsub_ri(true, false, false, 2, 2, 1, false);
        a.str(64, 2, XCPU, OFF_PLAINHITS);
    }
    let inline_ret_jmp = a.tbnz(1, 58);
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
    a.bind(inline_ret_jmp);
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

    // Shared bailout path to `call_slow` / `jmp_slow`.
    let inline_fast_bail = a.pos;
    for f in to_inline_bail {
        a.patch(f, inline_fast_bail);
    }
    let bail_to_jmp = a.tbnz(1, 58);
    a.patch(bail_to_jmp, jmp_slow);
    let bail_to_call = a.b();
    a.patch(bail_to_call, call_slow);

    // host_xfer(x0 = target, x2 = kind: bit 0 = jmp (else call), bit 1 = dynamic target from
    // `lookup_call`/`lookup_jmp` (else a static target from `xfer_to`); cpu.rip = return
    // address for calls).
    let mut host_xfer_bl: Option<usize> = None;
    let host_xfer = if hx {
        let p = a.pos;
        for i in 0..8 {
            a.stp_x(HG[2 * i], HG[2 * i + 1], XCPU, (OFF_GPR + 16 * i as u32) as i32);
        }
        if let Some(sp_pos) = spill_xmm_full {
            let sp_addr = a.addr_of(sp_pos);
            a.bl_to(sp_addr);
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
        if ras {
            a.ldr(64, XRAS, XCPU, OFF_RAS);
        }
        a.ldr(64, XBUD, XCPU, OFF_BUDGET);
        a.lsl_i(true, XBUD, XBUD, 1);
        a.ldr(16, XS, XCPU, OFF_YMM_DIRTY);
        a.addsub_ri(false, true, true, ZR, XS, 0, false);
        a.cset(XS, NE);
        a.logic_rr(true, 1, XBUD, XBUD, XS, 0, 0);
        if let Some(rl_pos) = reload_xmm {
            let rl_addr = a.addr_of(rl_pos);
            a.bl_to(rl_addr);
        }
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
    let (ras_miss, ras_miss_ic) = if ras {
        if let Some(want) = cfg.target_ras_miss {
            while a.pos < want && !a.overflow {
                a.emit(0xD503_201F); // nop padding to match .aot stub table offset exactly
            }
        }
        let p = a.pos;
        for f in to_ras_miss {
            a.patch(f, p);
        }
        a.movz(true, T4, 0, 0);
        let p_ic = a.pos;
        let mut found = Vec::new();
        for i in 1..=3u32 {
            a.addsub_ri(true, true, false, XA, XRAS, 16 * i, false);
            a.mov_rr(true, 1, XRAS);
            a.bfxil0(1, XA, 16);
            a.ldr(64, 2, 1, 0);
            a.logic_rr(true, 2, 2, 2, 0, 0, 0); // eor x2, x2, x0
            found.push(a.cbz(true, 2));
        }
        // Depth 1..3 missed: probe the per-site 2-way IC at `[T4]` before `lookup_rip_ic`.
        let no_ic = a.cbz(true, T4);
        a.ldp_x(1, 2, T4, 0);
        a.addsub_rr(true, true, true, ZR, 1, 0, 0);
        let try_w1 = a.bcond(NE);
        let ic_hit0 = a.cbnz(true, 2);
        a.bind(try_w1);
        a.ldp_x(1, 2, T4, 16);
        a.addsub_rr(true, true, true, ZR, 1, 0, 0);
        let miss_w1 = a.bcond(NE);
        let ic_hit1 = a.cbnz(true, 2);
        a.bind(no_ic);
        a.bind(miss_w1);
        let lr_ic = a.addr_of(lookup_rip_ic);
        a.b_to(lr_ic);
        a.bind(ic_hit0);
        a.bind(ic_hit1);
        if bud_light {
            a.addsub_ri(true, true, false, XBUD, XBUD, 2, false);
        }
        if bud {
            let f = a.tbnz(XBUD, 63);
            a.patch(f, rip_out);
        }
        a.br(2);
        for f in found {
            a.bind(f);
        }
        // x1 = matching entry: new top = the entry below it.
        a.ldr(64, 2, 1, 8);
        a.addsub_ri(true, true, false, XA, 1, 16, false);
        a.mov_rr(true, XRAS, 1);
        a.bfxil0(XRAS, XA, 16);
        a.br(2);
        (Some(p), Some(p_ic))
    } else {
        (None, None)
    };

    (
        Stubs {
            enter,
            exit,
            lookup_rip,
            lookup_jmp,
            lookup_call,
            host_xfer,
            ras_miss,
            spill_xmm,
            reload_xmm,
            clear_ymm_hi,
            rdtsc_slow,
            rdtscp_slow,
            lookup_rip_ic,
            lookup_jmp_ic,
            lookup_call_ic,
            ras_miss_ic,
        },
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
    pub(super) lookup_rip_ic: usize,
    pub(super) lookup_jmp_ic: usize,
    pub(super) lookup_call_ic: usize,
    pub(super) ras_miss_ic: Option<usize>,
}

impl Stubs {
    pub(super) fn offsets(&self) -> AotStubOffs {
        let word = |p: usize| p as u32;
        [
            word(self.enter),
            word(self.exit),
            word(self.lookup_rip),
            word(self.lookup_jmp),
            word(self.lookup_call),
            self.host_xfer.map_or(u32::MAX, word),
            self.ras_miss.map_or(u32::MAX, word),
            self.spill_xmm.map_or(u32::MAX, word),
            self.reload_xmm.map_or(u32::MAX, word),
            word(self.clear_ymm_hi),
            word(self.rdtsc_slow),
            word(self.rdtscp_slow),
            word(self.lookup_rip_ic),
            word(self.lookup_jmp_ic),
            word(self.lookup_call_ic),
            self.ras_miss_ic.map_or(u32::MAX, word),
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
