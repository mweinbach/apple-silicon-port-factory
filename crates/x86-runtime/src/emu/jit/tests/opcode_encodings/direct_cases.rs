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
    register_translated_instructions(cases);
    register_stubs(cases);
}

fn register_translated_instructions(cases: &mut Cases) {
    // Translate each guest instruction in isolation. Use fixed virtual addresses for its shared timestamp stubs.
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
        rdtsc_slow: 0xc00,
        rdtscp_slow: 0xb00,
        lookup_rip_ic: 0,
        lookup_jmp_ic: 0,
        lookup_call_ic: 0,
        ras_miss_ic: 0,
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
        let mut buf = [0u32; 64];
        let a = Asm::new(buf.as_mut_ptr(), 0x1000, buf.len());
        let mut tr = new_tr(a, &prep.ops[..1], &prep, &stubs, false, &mut scratch);
        tr.op(0);
        tr.emit_timestamp_fallbacks();
        tr.emit_atomic_colds();
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
        &[("mvn_orn_log_shift", "mvn w22, w22")],
    );
    check(
        "not_rbx",
        &[0x48, 0xf7, 0xd3],
        &[("mvn_orn_log_shift", "mvn x22, x22")],
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
            ("movn", "movn w2, #0"),
            ("ldeorb", "ldeoralb w2, w0, [x16]"),
            ("dmb", "dmb ish"),
        ],
    );
    let load_locks = format!("ldr x17, [x15, #{}]", crate::emu::jit::OFF_ATOMIC_LOCKS);
    check(
        "lock_not_qword_rsp",
        &[0xf0, 0x48, 0xf7, 0x14, 0x24],
        &[
            ("mov_orr_log_shift", "mov x16, x23"),
            ("and_log_imm", "and w17, w16, #15"),
            ("add_addsub_imm", "add w17, w17, #7"),
            ("lsr_ubfm", "lsr w17, w17, #4"),
            ("cbnz", "cbnz w17, #16"),
            ("movn", "movn x2, #0"),
            ("ldeor", "ldeoral x2, x0, [x16]"),
            ("dmb", "dmb ish"),
            ("ubfx_ubfm", "ubfx x5, x16, #4, #6"),
            ("ldr_imm_gen", &load_locks),
            ("add_addsub_shift", "add x17, x17, x5, lsl #7"),
            ("movz", "movz w6, #1"),
            ("swp", "swpal w6, w5, [x17]"),
            ("cbz", "cbz w5, #20"),
            ("yield", "yield"),
            ("ldr_imm_gen", "ldr w5, [x17]"),
            ("cbnz", "cbnz w5, #-8"),
            ("b_uncond", "b #-20"),
            ("ldr_imm_gen", "ldr x0, [x16]"),
            ("mvn_orn_log_shift", "mvn x2, x0"),
            ("str_imm_gen", "str x2, [x16]"),
            ("stlr", "stlr wzr, [x17]"),
            ("b_uncond", "b #-60"),
        ],
    );
    check(
        "lock_and_dword_rsp",
        &[0xf0, 0x21, 0x04, 0x24],
        &[
            ("mov_orr_log_shift", "mov x16, x23"),
            ("mov_orr_log_shift", "mov x1, x19"),
            ("and_log_imm", "and w17, w16, #15"),
            ("add_addsub_imm", "add w17, w17, #3"),
            ("lsr_ubfm", "lsr w17, w17, #4"),
            ("cbnz", "cbnz w17, #16"),
            ("mvn_orn_log_shift", "mvn w2, w1"),
            ("ldclr", "ldclral w2, w0, [x16]"),
            ("dmb", "dmb ish"),
            ("ubfx_ubfm", "ubfx x5, x16, #4, #6"),
            ("ldr_imm_gen", &load_locks),
            ("add_addsub_shift", "add x17, x17, x5, lsl #7"),
            ("movz", "movz w6, #1"),
            ("swp", "swpal w6, w5, [x17]"),
            ("cbz", "cbz w5, #20"),
            ("yield", "yield"),
            ("ldr_imm_gen", "ldr w5, [x17]"),
            ("cbnz", "cbnz w5, #-8"),
            ("b_uncond", "b #-20"),
            ("ldr_imm_gen", "ldr w0, [x16]"),
            ("and_log_shift", "and w2, w0, w1"),
            ("str_imm_gen", "str w2, [x16]"),
            ("stlr", "stlr wzr, [x17]"),
            ("b_uncond", "b #-60"),
        ],
    );
    let load_mode = format!("ldrb w1, [x15, #{}]", crate::emu::jit::OFF_TSC_MODE);
    check(
        "rdtsc",
        &[0x0f, 0x31],
        &[
            ("ldrb_imm", &load_mode),
            ("tbz", "tbz w1, #1, #8"),
            ("b_uncond", "b #16"),
            ("mrs", "mrs x0, cntvct_el0"),
            ("mov_orr_log_shift", "mov w19, w0"),
            ("lsr_ubfm", "lsr x21, x0, #32"),
            ("tbz", "tbz w1, #2, #36"),
            ("mrs", "mrs x0, cntvct_el0"),
            ("mov_orr_log_imm", "mov x2, #0xaaaaaaaaaaaaaaaa"),
            ("add_addsub_imm", "add x2, x2, #1"),
            ("umulh", "umulh x1, x0, x2"),
            ("add_addsub_shift", "add x2, x0, x0, lsl #2"),
            ("add_addsub_shift", "add x0, x0, x2, lsl #3"),
            ("add_addsub_shift", "add x0, x0, x1"),
            ("b_uncond", "b #-40"),
            ("bl", "bl #-1084"),
            ("b_uncond", "b #-48"),
        ],
    );
    check(
        "rdtscp",
        &[0x0f, 0x01, 0xf9],
        &[
            ("ldrb_imm", &load_mode),
            ("dsb", "dsb ishld"),
            ("cbnz", "cbnz w1, #20"),
            ("mrs", "mrs x0, S3_3_C14_C0_6"),
            ("mov_orr_log_shift", "mov w19, w0"),
            ("lsr_ubfm", "lsr x21, x0, #32"),
            ("movz", "movz x20, #0"),
            ("bl", "bl #-1308"),
            ("b_uncond", "b #-16"),
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
    cases.record(
        "timestamp_stubs",
        &[
            ("tbz", "tbz w1, #3, #12"),
            ("mrs", "mrs x0, S3_3_C14_C0_6"),
            ("b_uncond", "b #20"),
            ("isb", "isb"),
            ("tbz", "tbz w1, #1, #92"),
            ("tbnz", "tbnz w1, #0, #80"),
            ("mrs", "mrs x0, cntvct_el0"),
            ("tbz", "tbz w1, #2, #32"),
            ("mov_orr_log_imm", "mov x2, #0xaaaaaaaaaaaaaaaa"),
            ("add_addsub_imm", "add x2, x2, #1"),
            ("umulh", "umulh x1, x0, x2"),
            ("add_addsub_shift", "add x2, x0, x0, lsl #2"),
            ("add_addsub_shift", "add x0, x0, x2, lsl #3"),
            ("add_addsub_shift", "add x0, x0, x1"),
            ("ret", "ret"),
            ("mrs", "mrs x1, cntfrq_el0"),
            ("movz", "movz x2, #0xca00"),
            ("movk", "movk x2, #0x3b9a, lsl #16"),
            ("udiv", "udiv x3, x0, x1"),
            ("msub", "msub x0, x3, x1, x0"),
            ("mul_madd", "mul x0, x0, x2"),
            ("mul_madd", "mul x3, x3, x2"),
            ("udiv", "udiv x0, x0, x1"),
            ("add_addsub_shift", "add x0, x3, x0"),
            ("ret", "ret"),
            ("movz", "movz x0, #0x1000"),
            ("ret", "ret"),
            ("mrs", "mrs x0, cntvct_el0"),
            ("ret", "ret"),
        ],
        &bank[st.rdtscp_slow..st.lookup_rip],
    );
    let call = call.expect("host transfer BL") / 4;
    cases.record(
        "static_host_call_relocation",
        &[("bl", "bl #0")],
        &bank[call..call + 1],
    );

    let offsets = st.offsets();
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
