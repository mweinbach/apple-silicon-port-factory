//! Instructions written directly by the translator and the static AOT stub writer.

use super::Cases;
use crate::emu::fast::{lower, FastOp};
use crate::emu::jit::cache::{new_tr, Prep, StubAddrs, TrScratch};
use crate::emu::jit::emit::Asm;
use crate::emu::jit::lazy::Edges;
use crate::emu::jit::stubs::{emit_stubs_cfg, StubCfg};
use crate::emu::jit::{emit_static_stubs_with_veneer, Term, AOT_STUBS_BYTES};
use crate::emu::BLK_MAX_INSNS;
use iced_x86::{Decoder, DecoderOptions};

pub(super) fn register(cases: &mut Cases) {
    register_not(cases);
    register_stubs(cases);
}

fn register_not(cases: &mut Cases) {
    // Translate each guest instruction in isolation. These forms never branch to a shared stub.
    let stubs = StubAddrs {
        exit: 0,
        lookup_rip: 0,
        lookup_jmp: 0,
        lookup_call: 0,
        host_xfer: 0,
        ras_miss: 0,
        spill_xmm: 0,
        reload_xmm: 0,
        clear_ymm_hi: 0,
    };
    let mut prep = Prep {
        ops: [FastOp::default(); BLK_MAX_INSNS],
        k: 1,
        host: false,
        wr: [0; BLK_MAX_INSNS],
        term: Term::Fall(0),
        target_host: false,
        lazy: false,
        edge: Edges {
            live: [0; 2],
            kill: [0; 2],
        },
        live_out: 0,
    };
    let mut scratch = TrScratch::new();
    let mut check = |name: &str, guest: &[u8], expected: &[(&str, &str)]| {
        let mut decoder = Decoder::new(64, guest, DecoderOptions::NONE);
        let ins = decoder.decode();
        assert!(!ins.is_invalid() && !decoder.can_decode());
        prep.ops[0] = lower(&ins);
        let mut buf = [0u32; 32];
        let a = Asm::new(buf.as_mut_ptr(), 0x1000, buf.len());
        let mut tr = new_tr(a, &prep.ops[..1], &prep, &stubs, false, &mut scratch);
        tr.op(0);
        assert!(!tr.a.overflow);
        cases.record(name, expected, &buf[..tr.a.pos]);
        tr.return_scratch(&mut scratch);
    };
    check(
        "not_bl",
        &[0xf6, 0xd3],
        &[
            ("mvn_orn_log_shift", "mvn w0, w22"),
            ("bfxil_bfm", "bfxil x22, x0, #0, #8"),
        ],
    );
    check(
        "not_bx",
        &[0x66, 0xf7, 0xd3],
        &[
            ("mvn_orn_log_shift", "mvn w0, w22"),
            ("bfxil_bfm", "bfxil x22, x0, #0, #16"),
        ],
    );
    check(
        "not_ebx",
        &[0xf7, 0xd3],
        &[
            ("mvn_orn_log_shift", "mvn w0, w22"),
            ("mov_orr_log_shift", "mov w22, w0"),
        ],
    );
    check(
        "not_rbx",
        &[0x48, 0xf7, 0xd3],
        &[
            ("mvn_orn_log_shift", "mvn x0, x22"),
            ("mov_orr_log_shift", "mov x22, x0"),
        ],
    );
    check(
        "not_bh",
        &[0xf6, 0xd7],
        &[
            ("ubfx_ubfm", "ubfx x0, x22, #8, #8"),
            ("mvn_orn_log_shift", "mvn w0, w0"),
            ("bfi_bfm", "bfi x22, x0, #8, #8"),
        ],
    );
    check(
        "lock_not_byte_rsp",
        &[0xf0, 0xf6, 0x14, 0x24],
        &[
            ("mov_orr_log_shift", "mov x16, x23"),
            ("ldaxrb", "ldaxrb w0, [x16]"),
            ("mvn_orn_log_shift", "mvn w2, w0"),
            ("stlxrb", "stlxrb w3, w2, [x16]"),
            ("cbnz", "cbnz w3, #-12"),
        ],
    );
    check(
        "lock_not_qword_rsp",
        &[0xf0, 0x48, 0xf7, 0x14, 0x24],
        &[
            ("mov_orr_log_shift", "mov x16, x23"),
            ("and_log_imm", "and x17, x16, #7"),
            ("cbnz", "cbnz x17, #24"),
            ("ldaxr", "ldaxr x0, [x16]"),
            ("mvn_orn_log_shift", "mvn x2, x0"),
            ("stlxr", "stlxr w3, x2, [x16]"),
            ("cbnz", "cbnz w3, #-12"),
            ("b_uncond", "b #24"),
            ("ldr_imm_gen", "ldr x0, [x16]"),
            ("dmb", "dmb ishld"),
            ("mvn_orn_log_shift", "mvn x2, x0"),
            ("dmb", "dmb ish"),
            ("str_imm_gen", "str x2, [x16]"),
        ],
    );
}

fn register_stubs(cases: &mut Cases) {
    const VBASE: usize = 0x1000;
    const HOST_CALLS: u64 = 1 << 13;
    let mut bank = vec![0u32; AOT_STUBS_BYTES / 4];
    let mut a = Asm::new(bank.as_mut_ptr(), VBASE, bank.len());
    let cfg = StubCfg::from_aot_knobs(HOST_CALLS, None, false, false);
    let (st, call) = emit_stubs_cfg(&mut a, cfg);
    assert!(!a.overflow);
    cases.record(
        "stub_enter_frame",
        &[
            ("stp_gen", "stp x29, x30, [sp, #-96]!"),
            ("mov_add_addsub_imm", "mov x29, sp"),
        ],
        &bank[st.enter..st.enter + 2],
    );
    // With XMM pinning disabled, clear_ymm_hi immediately follows the exit stub.
    cases.record(
        "stub_exit_frame",
        &[("ldp_gen", "ldp x29, x30, [sp], #96"), ("ret", "ret")],
        &bank[st.clear_ymm_hi - 2..st.clear_ymm_hi],
    );
    let call = call.expect("host transfer BL") / 4;
    cases.record(
        "static_host_call_relocation",
        &[("bl", "bl #0")],
        &bank[call..call + 1],
    );

    let offsets = [
        Some(st.enter),
        Some(st.exit),
        Some(st.lookup_rip),
        Some(st.lookup_jmp),
        Some(st.lookup_call),
        st.host_xfer,
        st.ras_miss,
        st.spill_xmm,
        st.reload_xmm,
        Some(st.clear_ymm_hi),
    ]
    .map(|off| off.map_or(u32::MAX, |i| i as u32));
    // The production writer takes an aligned, writable stub bank of AOT_STUBS_BYTES.
    let (_, veneer) =
        unsafe { emit_static_stubs_with_veneer(bank.as_mut_ptr(), VBASE, HOST_CALLS, &offsets) }
            .unwrap();
    let veneer = veneer.expect("host transfer veneer") / 4;
    let branch = format!("bl #{}", (veneer as isize - call as isize) * 4);
    cases.record(
        "static_host_call_to_veneer",
        &[("bl", &branch)],
        &bank[call..call + 1],
    );
    cases.record(
        "static_host_call_veneer",
        &[
            ("adrp", "adrp x16, #0"),
            ("ldr_imm_gen", "ldr x16, [x16]"),
            ("br", "br x16"),
            ("nop", "nop"),
        ],
        &bank[veneer..veneer + 4],
    );
}
