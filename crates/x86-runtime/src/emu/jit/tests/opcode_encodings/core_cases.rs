//! Encoding test cases for all core `Asm` methods and helpers in `jit::emit`.

use super::Cases;
use crate::emu::jit::emit::{enc_logic_imm, patch_branch, sf, Asm};
use crate::emu::jit::{EQ, GE, GT, HI, HS, LE, LO, LS, LT, MI, NE, PL, VC, VS, XS, ZR};

const NOP_WORD: u32 = 0xD503_201F;
const SP: u32 = ZR;

pub(super) fn register(cases: &mut Cases) {
    register_helpers(cases);
    register_data_processing(cases);
    register_bitfields_and_alu(cases);
    register_scalar_fp_and_add_imm(cases);
    register_loads_stores_barriers_sysregs(cases);
    register_branches(cases);
}

fn register_helpers(cases: &mut Cases) {
    cases.check(
        "asm_new_addr_of_read_emit",
        &[("nop", "nop"), ("b_uncond", "b #-4"), ("ret", "ret")],
        |a| {
            assert_eq!(a.addr_of(0), 0x1000);
            assert_eq!(a.addr_of(3), 0x100c);
            assert_eq!(sf(false), 0);
            assert_eq!(sf(true), 1 << 31);

            // Exercise Asm::new on an independent buffer, including vbase addressing,
            // raw emit, read, and overflow / out-of-bounds patch guards.
            let mut buf = [0u32; 3];
            let mut sub = Asm::new(buf.as_mut_ptr(), 0x2000, buf.len());
            assert_eq!(sub.pos, 0);
            assert!(!sub.overflow);
            assert_eq!(sub.addr_of(0), 0x2000);
            assert_eq!(sub.addr_of(2), 0x2008);

            sub.emit(NOP_WORD);
            sub.b_to(sub.addr_of(0));
            sub.ret();
            assert_eq!(sub.pos, 3);
            assert!(!sub.overflow);

            // Out-of-bounds patch / patch_adr calls must be ignored without panicking.
            let saved_b = sub.read(1);
            sub.patch(3, 0);
            sub.patch_adr(3, 0);
            assert_eq!(sub.read(1), saved_b);

            // Emitting past capacity sets overflow and suppresses subsequent patches.
            sub.emit(NOP_WORD);
            assert!(sub.overflow);
            assert_eq!(sub.pos, 3);
            sub.patch(1, 2);
            sub.patch_adr(0, 2);
            assert_eq!(sub.read(0), NOP_WORD);
            assert_eq!(sub.read(1), saved_b);

            for i in 0..sub.pos {
                let w = sub.read(i);
                a.emit(w);
                assert_eq!(a.read(i), w);
            }
        },
    );

    cases.check(
        "enc_logic_imm",
        &[
            ("and_log_imm", "and w3, w5, #0x55555555"),
            ("orr_log_imm", "orr w4, w6, #0xaaaaaaaa"),
            ("eor_log_imm", "eor w7, w8, #0x33333333"),
            ("and_log_imm", "and w9, w10, #0x0f0f0f0f"),
            ("orr_log_imm", "orr w11, w12, #0xf0f0f0f0"),
            ("eor_log_imm", "eor w13, w14, #0x00ff00ff"),
            ("and_log_imm", "and w15, w16, #0x000ffff0"),
            ("orr_log_imm", "orr w17, w18, #0xf800000f"),
            ("and_log_imm", "and x3, x5, #0x5555555555555555"),
            ("orr_log_imm", "orr x4, x6, #0xcccccccccccccccc"),
            ("eor_log_imm", "eor x7, x8, #0x0f0f0f0f0f0f0f0f"),
            ("and_log_imm", "and x9, x10, #0x00ff00ff00ff00ff"),
            ("orr_log_imm", "orr x11, x12, #0x0000ffff0000ffff"),
            ("eor_log_imm", "eor x13, x14, #0x00000000ffffffff"),
            ("and_log_imm", "and x15, x16, #0xffff800000000007"),
        ],
        |a| {
            // Non-encodable values must return None.
            assert_eq!(enc_logic_imm(0, false), None);
            assert_eq!(enc_logic_imm(0xFFFF_FFFF, false), None);
            assert_eq!(enc_logic_imm(0x1234, false), None);
            assert_eq!(enc_logic_imm(0, true), None);
            assert_eq!(enc_logic_imm(u64::MAX, true), None);
            assert_eq!(enc_logic_imm(0x5, true), None);

            // Repeating widths 2, 4, 8, 16, 32 (32-bit), unrotated and rotated.
            a.logic_ri(false, 0, 3, 5, enc_logic_imm(0x5555_5555, false).unwrap());
            a.logic_ri(false, 1, 4, 6, enc_logic_imm(0xAAAA_AAAA, false).unwrap());
            a.logic_ri(false, 2, 7, 8, enc_logic_imm(0x3333_3333, false).unwrap());
            a.logic_ri(false, 0, 9, 10, enc_logic_imm(0x0F0F_0F0F, false).unwrap());
            a.logic_ri(false, 1, 11, 12, enc_logic_imm(0xF0F0_F0F0, false).unwrap());
            a.logic_ri(false, 2, 13, 14, enc_logic_imm(0x00FF_00FF, false).unwrap());
            a.logic_ri(false, 0, 15, 16, enc_logic_imm(0x000F_FFF0, false).unwrap());
            a.logic_ri(false, 1, 17, 18, enc_logic_imm(0xF800_000F, false).unwrap());

            // Repeating widths 2, 4, 8, 16, 32, 64 (64-bit), unrotated and rotated.
            a.logic_ri(
                true,
                0,
                3,
                5,
                enc_logic_imm(0x5555_5555_5555_5555, true).unwrap(),
            );
            a.logic_ri(
                true,
                1,
                4,
                6,
                enc_logic_imm(0xCCCC_CCCC_CCCC_CCCC, true).unwrap(),
            );
            a.logic_ri(
                true,
                2,
                7,
                8,
                enc_logic_imm(0x0F0F_0F0F_0F0F_0F0F, true).unwrap(),
            );
            a.logic_ri(
                true,
                0,
                9,
                10,
                enc_logic_imm(0x00FF_00FF_00FF_00FF, true).unwrap(),
            );
            a.logic_ri(
                true,
                1,
                11,
                12,
                enc_logic_imm(0x0000_FFFF_0000_FFFF, true).unwrap(),
            );
            a.logic_ri(
                true,
                2,
                13,
                14,
                enc_logic_imm(0x0000_0000_FFFF_FFFF, true).unwrap(),
            );
            a.logic_ri(
                true,
                0,
                15,
                16,
                enc_logic_imm(0xFFFF_8000_0000_0007, true).unwrap(),
            );
        },
    );

    cases.check(
        "patch_branch_helper",
        &[
            ("b_uncond", "b #16"),
            ("b_uncond", "b #-12"),
            ("bl", "bl #20"),
            ("bl", "bl #-16"),
            ("b_cond", "b.ne #24"),
            ("b_cond", "b.lt #-20"),
            ("cbz", "cbz w5, #28"),
            ("cbz", "cbz x6, #-24"),
            ("cbnz", "cbnz w7, #32"),
            ("cbnz", "cbnz x8, #-28"),
            ("tbnz", "tbnz w9, #7, #36"),
            ("tbnz", "tbnz x10, #45, #-32"),
            ("nop", "nop"),
        ],
        |a| {
            let mut scratch = [0u32; 12];
            let mut tmp = Asm::new(scratch.as_mut_ptr(), 0x1000, scratch.len());
            let ib = tmp.b();
            let ibl = tmp.bl();
            let ibne = tmp.bcond(NE);
            let iblt = tmp.bcond(LT);
            let icbzw = tmp.cbz(false, 5);
            let icbzx = tmp.cbz(true, 6);
            let icbnzw = tmp.cbnz(false, 7);
            let icbnzx = tmp.cbnz(true, 8);
            let itbnzw = tmp.tbnz(9, 7);
            let itbnzx = tmp.tbnz(10, 45);

            // Re-patch a previously negative branch to verify old offset bits are cleared.
            let b_neg = patch_branch(tmp.read(ib), -3);
            a.emit(patch_branch(b_neg, 4));
            a.emit(b_neg);
            a.emit(patch_branch(tmp.read(ibl), 5));
            a.emit(patch_branch(tmp.read(ibl), -4));
            a.emit(patch_branch(tmp.read(ibne), 6));
            a.emit(patch_branch(tmp.read(iblt), -5));
            a.emit(patch_branch(tmp.read(icbzw), 7));
            a.emit(patch_branch(tmp.read(icbzx), -6));
            a.emit(patch_branch(tmp.read(icbnzw), 8));
            a.emit(patch_branch(tmp.read(icbnzx), -7));
            a.emit(patch_branch(tmp.read(itbnzw), 9));
            a.emit(patch_branch(tmp.read(itbnzx), -8));
            // Non-branch instruction passes through unchanged.
            a.emit(patch_branch(NOP_WORD, 13));
        },
    );
}

fn register_data_processing(cases: &mut Cases) {
    check_words!(cases, "addsub_rr";
        addsub_rr(false, false, false, 3, 5, 7, 0) => "add_addsub_shift", "add w3, w5, w7";
        addsub_rr(false, false, false, 4, 6, 8, 5) => "add_addsub_shift", "add w4, w6, w8, lsl #5";
        addsub_rr(false, false, true, 9, 11, 13, 31) => "adds_addsub_shift", "adds w9, w11, w13, lsl #31";
        addsub_rr(false, true, false, 14, 16, 18, 0) => "sub_addsub_shift", "sub w14, w16, w18";
        addsub_rr(false, true, true, 19, 21, 23, 3) => "subs_addsub_shift", "subs w19, w21, w23, lsl #3";
        addsub_rr(true, false, false, 3, 5, 7, 0) => "add_addsub_shift", "add x3, x5, x7";
        addsub_rr(true, false, false, 4, 6, 8, 11) => "add_addsub_shift", "add x4, x6, x8, lsl #11";
        addsub_rr(true, false, true, 9, 11, 13, 63) => "adds_addsub_shift", "adds x9, x11, x13, lsl #63";
        addsub_rr(true, true, false, 14, 16, 18, 0) => "sub_addsub_shift", "sub x14, x16, x18";
        addsub_rr(true, true, true, 19, 21, 23, 7) => "subs_addsub_shift", "subs x19, x21, x23, lsl #7";
    );

    check_words!(cases, "addsub_rr_aliases";
        addsub_rr(false, false, true, ZR, 5, 7, 0) => "cmn_adds_addsub_shift", "cmn w5, w7";
        addsub_rr(true, false, true, ZR, 5, 7, 3) => "cmn_adds_addsub_shift", "cmn x5, x7, lsl #3";
        addsub_rr(false, true, true, ZR, 6, 8, 4) => "cmp_subs_addsub_shift", "cmp w6, w8, lsl #4";
        addsub_rr(true, true, true, ZR, 6, 8, 0) => "cmp_subs_addsub_shift", "cmp x6, x8";
        addsub_rr(false, true, false, 3, ZR, 9, 0) => "neg_sub_addsub_shift", "neg w3, w9";
        addsub_rr(true, true, false, 3, ZR, 9, 5) => "neg_sub_addsub_shift", "neg x3, x9, lsl #5";
        addsub_rr(false, true, true, 4, ZR, 10, 2) => "negs_subs_addsub_shift", "negs w4, w10, lsl #2";
        addsub_rr(true, true, true, 4, ZR, 10, 0) => "negs_subs_addsub_shift", "negs x4, x10";
    );

    check_words!(cases, "addsub_ri";
        addsub_ri(false, false, false, 3, 5, 0x123, false) => "add_addsub_imm", "add w3, w5, #0x123";
        addsub_ri(false, false, false, 4, 6, 0x456, true) => "add_addsub_imm", "add w4, w6, #0x456, lsl #12";
        addsub_ri(false, false, true, 7, 9, 0xFFF, false) => "adds_addsub_imm", "adds w7, w9, #0xfff";
        addsub_ri(false, true, false, 10, 12, 0x234, false) => "sub_addsub_imm", "sub w10, w12, #0x234";
        addsub_ri(false, true, true, 13, 15, 0x345, true) => "subs_addsub_imm", "subs w13, w15, #0x345, lsl #12";
        addsub_ri(true, false, false, 3, 5, 0x123, false) => "add_addsub_imm", "add x3, x5, #0x123";
        addsub_ri(true, false, false, 4, SP, 0x456, true) => "add_addsub_imm", "add x4, sp, #0x456, lsl #12";
        addsub_ri(true, false, true, 7, 9, 0xFFF, true) => "adds_addsub_imm", "adds x7, x9, #0xfff, lsl #12";
        addsub_ri(true, true, false, SP, SP, 0x80, false) => "sub_addsub_imm", "sub sp, sp, #0x80";
        addsub_ri(true, true, true, 19, 21, 0x789, false) => "subs_addsub_imm", "subs x19, x21, #0x789";
    );

    check_words!(cases, "addsub_ri_aliases";
        addsub_ri(false, false, true, ZR, 5, 0x123, false) => "cmn_adds_addsub_imm", "cmn w5, #0x123";
        addsub_ri(true, false, true, ZR, SP, 0x40, true) => "cmn_adds_addsub_imm", "cmn sp, #0x40, lsl #12";
        addsub_ri(false, true, true, ZR, 6, 0x234, true) => "cmp_subs_addsub_imm", "cmp w6, #0x234, lsl #12";
        addsub_ri(true, true, true, ZR, 19, 0x345, false) => "cmp_subs_addsub_imm", "cmp x19, #0x345";
        addsub_ri(false, false, false, SP, 5, 0, false) => "mov_add_addsub_imm", "mov wsp, w5";
        addsub_ri(false, false, false, 3, SP, 0, false) => "mov_add_addsub_imm", "mov w3, wsp";
        addsub_ri(true, false, false, SP, 21, 0, false) => "mov_add_addsub_imm", "mov sp, x21";
        addsub_ri(true, false, false, 22, SP, 0, false) => "mov_add_addsub_imm", "mov x22, sp";
    );

    check_words!(cases, "logic_rr";
        logic_rr(false, 0, 3, 5, 7, 0, 0) => "and_log_shift", "and w3, w5, w7";
        logic_rr(false, 0, 4, 6, 8, 0, 5) => "and_log_shift", "and w4, w6, w8, lsl #5";
        logic_rr(false, 1, 9, 11, 13, 1, 7) => "orr_log_shift", "orr w9, w11, w13, lsr #7";
        logic_rr(false, 2, 14, 16, 18, 2, 11) => "eor_log_shift", "eor w14, w16, w18, asr #11";
        logic_rr(false, 3, 19, 21, 23, 3, 31) => "ands_log_shift", "ands w19, w21, w23, ror #31";
        logic_rr(true, 0, 3, 5, 7, 0, 0) => "and_log_shift", "and x3, x5, x7";
        logic_rr(true, 0, 4, 6, 8, 0, 13) => "and_log_shift", "and x4, x6, x8, lsl #13";
        logic_rr(true, 1, 9, 11, 13, 1, 29) => "orr_log_shift", "orr x9, x11, x13, lsr #29";
        logic_rr(true, 2, 14, 16, 18, 2, 41) => "eor_log_shift", "eor x14, x16, x18, asr #41";
        logic_rr(true, 3, 19, 21, 23, 3, 57) => "ands_log_shift", "ands x19, x21, x23, ror #57";
    );

    check_words!(cases, "logic_rr_aliases";
        logic_rr(false, 1, 3, ZR, 19, 0, 0) => "mov_orr_log_shift", "mov w3, w19";
        logic_rr(true, 1, 4, ZR, 22, 0, 0) => "mov_orr_log_shift", "mov x4, x22";
        logic_rr(false, 3, ZR, 5, 7, 0, 0) => "tst_ands_log_shift", "tst w5, w7";
        logic_rr(false, 3, ZR, 6, 8, 1, 3) => "tst_ands_log_shift", "tst w6, w8, lsr #3";
        logic_rr(true, 3, ZR, 9, 11, 0, 0) => "tst_ands_log_shift", "tst x9, x11";
        logic_rr(true, 3, ZR, 12, 14, 3, 17) => "tst_ands_log_shift", "tst x12, x14, ror #17";
    );

    check_words!(cases, "logic_ri";
        logic_ri(false, 0, 3, 5, enc_logic_imm(0xFF, false).unwrap()) => "and_log_imm", "and w3, w5, #0xff";
        logic_ri(false, 1, 4, 6, enc_logic_imm(0x00FF_00FF, false).unwrap()) => "orr_log_imm", "orr w4, w6, #0x00ff00ff";
        logic_ri(false, 2, SP, 8, enc_logic_imm(0x5555_5555, false).unwrap()) => "eor_log_imm", "eor wsp, w8, #0x55555555";
        logic_ri(false, 3, 9, 11, enc_logic_imm(0xFFFF_0000, false).unwrap()) => "ands_log_imm", "ands w9, w11, #0xffff0000";
        logic_ri(true, 0, 3, 5, enc_logic_imm(0xFF, true).unwrap()) => "and_log_imm", "and x3, x5, #0x00000000000000ff";
        logic_ri(true, 1, 4, 6, enc_logic_imm(0x0F0F_0F0F_0F0F_0F0F, true).unwrap()) => "orr_log_imm", "orr x4, x6, #0x0f0f0f0f0f0f0f0f";
        logic_ri(true, 2, SP, 8, enc_logic_imm(!0xFu64, true).unwrap()) => "eor_log_imm", "eor sp, x8, #0xfffffffffffffff0";
        logic_ri(true, 3, 19, 21, enc_logic_imm(0xFFFF_8000_0000_0007, true).unwrap()) => "ands_log_imm", "ands x19, x21, #0xffff800000000007";
    );

    check_words!(cases, "logic_ri_aliases";
        logic_ri(false, 1, 3, ZR, enc_logic_imm(0x5555_5555, false).unwrap()) => "mov_orr_log_imm", "mov w3, #0x55555555";
        logic_ri(true, 1, 4, ZR, enc_logic_imm(0x0F0F_0F0F_0F0F_0F0F, true).unwrap()) => "mov_orr_log_imm", "mov x4, #0x0f0f0f0f0f0f0f0f";
        logic_ri(true, 1, SP, ZR, enc_logic_imm(0x5555_5555_5555_5555, true).unwrap()) => "mov_orr_log_imm", "mov sp, #0x5555555555555555";
        logic_ri(false, 3, ZR, 5, enc_logic_imm(0x00FF_00FF, false).unwrap()) => "tst_ands_log_imm", "tst w5, #0x00ff00ff";
        logic_ri(true, 3, ZR, 19, enc_logic_imm(0xFFFF_8000_0000_0007, true).unwrap()) => "tst_ands_log_imm", "tst x19, #0xffff800000000007";
    );

    check_words!(cases, "mov_rr";
        mov_rr(false, 3, 19) => "mov_orr_log_shift", "mov w3, w19";
        mov_rr(false, 27, 5) => "mov_orr_log_shift", "mov w27, w5";
        mov_rr(true, 4, 22) => "mov_orr_log_shift", "mov x4, x22";
        mov_rr(true, 28, 6) => "mov_orr_log_shift", "mov x28, x6";
    );

    check_words!(cases, "movz";
        movz(false, 3, 0x1234, 0) => "mov_movz", "mov w3, #0x1234";
        movz(false, 4, 0, 1) => "movz", "movz w4, #0, lsl #16";
        movz(false, 5, 0x5678, 1) => "mov_movz", "mov w5, #0x56780000";
        movz(true, 6, 0x9ABC, 0) => "mov_movz", "mov x6, #0x9abc";
        movz(true, 7, 0, 2) => "movz", "movz x7, #0, lsl #32";
        movz(true, 8, 0x1234, 1) => "movz", "movz x8, #0x1234, lsl #16";
        movz(true, 9, 0x5678, 2) => "movz", "movz x9, #0x5678, lsl #32";
        movz(true, 19, 0xDEF0, 3) => "movz", "movz x19, #0xdef0, lsl #48";
    );

    check_words!(cases, "movk";
        movk(false, 3, 0x1234, 0) => "movk", "movk w3, #0x1234";
        movk(false, 4, 0x5678, 1) => "movk", "movk w4, #0x5678, lsl #16";
        movk(true, 5, 0x9ABC, 0) => "movk", "movk x5, #0x9abc";
        movk(true, 6, 0x1234, 1) => "movk", "movk x6, #0x1234, lsl #16";
        movk(true, 7, 0x5678, 2) => "movk", "movk x7, #0x5678, lsl #32";
        movk(true, 21, 0xDEF0, 3) => "movk", "movk x21, #0xdef0, lsl #48";
    );

    check_words!(cases, "movn";
        movn(false, 3, 0, 0) => "mov_movn", "mov w3, #-1";
        movn(false, 4, 0, 1) => "movn", "movn w4, #0, lsl #16";
        movn(false, 5, 0xFFFF, 0) => "movn", "movn w5, #0xffff";
        movn(false, 6, 0x1234, 1) => "movn", "movn w6, #0x1234, lsl #16";
        movn(true, 7, 0, 0) => "mov_movn", "mov x7, #-1";
        movn(true, 8, 0, 3) => "movn", "movn x8, #0, lsl #48";
        movn(true, 9, 0x1234, 1) => "movn", "movn x9, #0x1234, lsl #16";
        movn(true, 10, 0x5678, 2) => "movn", "movn x10, #0x5678, lsl #32";
        movn(true, 22, 0x9ABC, 3) => "movn", "movn x22, #0x9abc, lsl #48";
    );

    cases.check(
        "mov_imm_32",
        &[
            // Zero
            ("mov_movz", "mov w3, #0"),
            // All-ones
            ("mov_movn", "mov w4, #-1"),
            // Single non-zero chunk at hw=1
            ("mov_movz", "mov w5, #0x12340000"),
            // Single non-0xffff chunk (ones > zeros)
            ("movn", "movn w6, #0xedcb"),
            // Equal ones and zeros (0xffff_0000 -> movz hw=1)
            ("movz", "movz w7, #0xffff, lsl #16"),
            // Logical immediate shortcut
            ("mov_orr_log_imm", "mov w8, #0x55555555"),
            // 2-word movz + movk (upper 32 bits ignored when x == false)
            ("movz", "movz w9, #0x5678"),
            ("movk", "movk w9, #0x1234, lsl #16"),
        ],
        |a| {
            a.mov_imm(false, 3, 0);
            a.mov_imm(false, 4, 0xFFFF_FFFF);
            a.mov_imm(false, 5, 0x1234_0000);
            a.mov_imm(false, 6, 0xFFFF_1234);
            a.mov_imm(false, 7, 0xFFFF_0000);
            a.mov_imm(false, 8, 0x5555_5555);
            a.mov_imm(false, 9, 0xDEAD_BEEF_1234_5678);
        },
    );

    cases.check(
        "mov_imm_64",
        &[
            // Zero
            ("mov_movz", "mov x3, #0"),
            // All-ones
            ("mov_movn", "mov x4, #-1"),
            // Single non-zero chunk at hw=2
            ("movz", "movz x5, #0x1234, lsl #32"),
            // Single non-0xffff chunk at hw=2 (ones > zeros)
            ("movn", "movn x6, #0xedcb, lsl #32"),
            // Logical immediate shortcut
            ("mov_orr_log_imm", "mov x7, #0x00ff00ff00ff00ff"),
            // 2-word movz + movk (hw=1, hw=3)
            ("movz", "movz x8, #0x5678, lsl #16"),
            ("movk", "movk x8, #0x1234, lsl #48"),
            // 2-word movn + movk (ones=2, zeros=0)
            ("movn", "movn x9, #0xa987"),
            ("movk", "movk x9, #0x1234, lsl #32"),
            // 2-word movn + movk with zero chunk (ones=2, zeros=1)
            ("movn", "movn x10, #0xa987"),
            ("movk", "movk x10, #0x0000, lsl #32"),
            // 3-word movz + 2*movk (ones=1, zeros=1 tie-break into movz path)
            ("movz", "movz x11, #0x5678"),
            ("movk", "movk x11, #0x1234, lsl #32"),
            ("movk", "movk x11, #0xffff, lsl #48"),
            // 4-word full 64-bit constant
            ("movz", "movz x19, #0xdef0"),
            ("movk", "movk x19, #0x9abc, lsl #16"),
            ("movk", "movk x19, #0x5678, lsl #32"),
            ("movk", "movk x19, #0x1234, lsl #48"),
        ],
        |a| {
            a.mov_imm(true, 3, 0);
            a.mov_imm(true, 4, u64::MAX);
            a.mov_imm(true, 5, 0x0000_1234_0000_0000);
            a.mov_imm(true, 6, 0xFFFF_1234_FFFF_FFFF);
            a.mov_imm(true, 7, 0x00FF_00FF_00FF_00FF);
            a.mov_imm(true, 8, 0x1234_0000_5678_0000);
            a.mov_imm(true, 9, 0xFFFF_1234_FFFF_5678);
            a.mov_imm(true, 10, 0xFFFF_0000_FFFF_5678);
            a.mov_imm(true, 11, 0xFFFF_1234_0000_5678);
            a.mov_imm(true, 19, 0x1234_5678_9ABC_DEF0);
        },
    );
}

fn register_bitfields_and_alu(cases: &mut Cases) {
    check_words!(cases, "ubfm";
        ubfm(false, 3, 5, 4, 11) => "ubfm", "ubfm w3, w5, #4, #11";
        ubfm(true, 4, 6, 7, 19) => "ubfm", "ubfm x4, x6, #7, #19";
        ubfm(false, 7, 9, 28, 11) => "ubfiz_ubfm", "ubfiz w7, w9, #4, #12";
        ubfm(true, 8, 10, 59, 13) => "ubfiz_ubfm", "ubfiz x8, x10, #5, #14";
        ubfm(false, 11, 13, 3, 11) => "ubfx_ubfm", "ubfx w11, w13, #3, #9";
        ubfm(true, 12, 14, 6, 22) => "ubfx_ubfm", "ubfx x12, x14, #6, #17";
        ubfm(false, 15, 17, 0, 7) => "uxtb_ubfm", "uxtb w15, w17";
        ubfm(false, 16, 18, 0, 15) => "uxth_ubfm", "uxth w16, w18";
        ubfm(false, 19, 21, 27, 26) => "lsl_ubfm", "lsl w19, w21, #5";
        ubfm(true, 20, 22, 13, 63) => "lsr_ubfm", "lsr x20, x22, #13";
    );

    check_words!(cases, "sbfm";
        sbfm(false, 3, 5, 4, 11) => "sbfm", "sbfm w3, w5, #4, #11";
        sbfm(true, 4, 6, 7, 19) => "sbfm", "sbfm x4, x6, #7, #19";
        sbfm(false, 7, 9, 28, 11) => "sbfiz_sbfm", "sbfiz w7, w9, #4, #12";
        sbfm(true, 8, 10, 59, 13) => "sbfiz_sbfm", "sbfiz x8, x10, #5, #14";
        sbfm(false, 11, 13, 3, 11) => "sbfx_sbfm", "sbfx w11, w13, #3, #9";
        sbfm(true, 12, 14, 6, 22) => "sbfx_sbfm", "sbfx x12, x14, #6, #17";
        sbfm(false, 15, 17, 0, 7) => "sxtb_sbfm", "sxtb w15, w17";
        sbfm(true, 15, 17, 0, 7) => "sxtb_sbfm", "sxtb x15, w17";
        sbfm(false, 16, 18, 0, 15) => "sxth_sbfm", "sxth w16, w18";
        sbfm(true, 16, 18, 0, 15) => "sxth_sbfm", "sxth x16, w18";
        sbfm(true, 19, 21, 0, 31) => "sxtw_sbfm", "sxtw x19, w21";
        sbfm(true, 20, 22, 11, 63) => "asr_sbfm", "asr x20, x22, #11";
    );

    check_words!(cases, "bfm";
        bfm(false, 3, 5, 4, 11) => "bfm", "bfm w3, w5, #4, #11";
        bfm(true, 4, 6, 7, 19) => "bfm", "bfm x4, x6, #7, #19";
        bfm(false, 7, 9, 28, 11) => "bfi_bfm", "bfi w7, w9, #4, #12";
        bfm(true, 8, 10, 59, 13) => "bfi_bfm", "bfi x8, x10, #5, #14";
        bfm(false, 11, 13, 3, 11) => "bfxil_bfm", "bfxil w11, w13, #3, #9";
        bfm(true, 12, 14, 6, 22) => "bfxil_bfm", "bfxil x12, x14, #6, #17";
        bfm(false, 15, ZR, 28, 11) => "bfc_bfm", "bfc w15, #4, #12";
        bfm(true, 19, ZR, 59, 13) => "bfc_bfm", "bfc x19, #5, #14";
    );

    check_words!(cases, "bfxil0";
        bfxil0(3, 5, 8) => "bfxil_bfm", "bfxil x3, x5, #0, #8";
        bfxil0(4, 6, 16) => "bfxil_bfm", "bfxil x4, x6, #0, #16";
        bfxil0(19, 21, 32) => "bfxil_bfm", "bfxil x19, x21, #0, #32";
        bfxil0(22, 24, 47) => "bfxil_bfm", "bfxil x22, x24, #0, #47";
    );

    check_words!(cases, "ubfx";
        ubfx(3, 5, 0, 8) => "ubfx_ubfm", "ubfx x3, x5, #0, #8";
        ubfx(4, 6, 5, 12) => "ubfx_ubfm", "ubfx x4, x6, #5, #12";
        ubfx(19, 21, 32, 16) => "ubfx_ubfm", "ubfx x19, x21, #32, #16";
        ubfx(22, 24, 48, 15) => "ubfx_ubfm", "ubfx x22, x24, #48, #15";
    );

    check_words!(cases, "lsl_i";
        lsl_i(false, 3, 5, 0) => "lsr_ubfm", "lsr w3, w5, #0";
        lsl_i(false, 4, 6, 1) => "lsl_ubfm", "lsl w4, w6, #1";
        lsl_i(false, 7, 9, 13) => "lsl_ubfm", "lsl w7, w9, #13";
        lsl_i(false, 11, 13, 31) => "lsl_ubfm", "lsl w11, w13, #31";
        lsl_i(true, 3, 5, 0) => "lsr_ubfm", "lsr x3, x5, #0";
        lsl_i(true, 4, 6, 1) => "lsl_ubfm", "lsl x4, x6, #1";
        lsl_i(true, 19, 21, 29) => "lsl_ubfm", "lsl x19, x21, #29";
        lsl_i(true, 22, 24, 63) => "lsl_ubfm", "lsl x22, x24, #63";
    );

    check_words!(cases, "lsr_i";
        lsr_i(false, 3, 5, 0) => "lsr_ubfm", "lsr w3, w5, #0";
        lsr_i(false, 4, 6, 7) => "lsr_ubfm", "lsr w4, w6, #7";
        lsr_i(false, 7, 9, 31) => "lsr_ubfm", "lsr w7, w9, #31";
        lsr_i(true, 11, 13, 0) => "lsr_ubfm", "lsr x11, x13, #0";
        lsr_i(true, 19, 21, 19) => "lsr_ubfm", "lsr x19, x21, #19";
        lsr_i(true, 22, 24, 63) => "lsr_ubfm", "lsr x22, x24, #63";
    );

    check_words!(cases, "asr_i";
        asr_i(false, 3, 5, 0) => "asr_sbfm", "asr w3, w5, #0";
        asr_i(false, 4, 6, 7) => "asr_sbfm", "asr w4, w6, #7";
        asr_i(false, 7, 9, 31) => "asr_sbfm", "asr w7, w9, #31";
        asr_i(true, 11, 13, 0) => "asr_sbfm", "asr x11, x13, #0";
        asr_i(true, 19, 21, 19) => "asr_sbfm", "asr x19, x21, #19";
        asr_i(true, 22, 24, 63) => "asr_sbfm", "asr x22, x24, #63";
    );

    check_words!(cases, "ror_i";
        ror_i(false, 3, 5, 1) => "ror_extr", "ror w3, w5, #1";
        ror_i(false, 4, 6, 11) => "ror_extr", "ror w4, w6, #11";
        ror_i(false, 7, 9, 31) => "ror_extr", "ror w7, w9, #31";
        ror_i(true, 11, 13, 1) => "ror_extr", "ror x11, x13, #1";
        ror_i(true, 19, 21, 29) => "ror_extr", "ror x19, x21, #29";
        ror_i(true, 22, 24, 63) => "ror_extr", "ror x22, x24, #63";
    );

    cases.check(
        "shiftv",
        &[
            // Canonical variable shift pages at 32-bit and 64-bit
            ("lslv", "lslv w3, w5, w7"),
            ("lsrv", "lsrv w4, w6, w8"),
            ("asrv", "asrv w9, w11, w13"),
            ("rorv", "rorv w14, w16, w18"),
            ("lslv", "lslv x3, x5, x7"),
            ("lsrv", "lsrv x4, x6, x8"),
            ("asrv", "asrv x9, x11, x13"),
            ("rorv", "rorv x14, x16, x18"),
            // Standard register-shift aliases at both widths
            ("lsl_lslv", "lsl w19, w21, w23"),
            ("lsr_lsrv", "lsr w20, w22, w24"),
            ("asr_asrv", "asr x19, x21, x23"),
            ("ror_rorv", "ror x20, x22, x24"),
        ],
        |a| {
            a.shiftv(false, 8, 3, 5, 7);
            a.shiftv(false, 9, 4, 6, 8);
            a.shiftv(false, 10, 9, 11, 13);
            a.shiftv(false, 11, 14, 16, 18);
            a.shiftv(true, 8, 3, 5, 7);
            a.shiftv(true, 9, 4, 6, 8);
            a.shiftv(true, 10, 9, 11, 13);
            a.shiftv(true, 11, 14, 16, 18);
            a.shiftv(false, 8, 19, 21, 23);
            a.shiftv(false, 9, 20, 22, 24);
            a.shiftv(true, 10, 19, 21, 23);
            a.shiftv(true, 11, 20, 22, 24);
        },
    );

    check_words!(cases, "csel";
        csel(false, 3, 5, 7, EQ) => "csel", "csel w3, w5, w7, eq";
        csel(false, 4, 6, 8, HS) => "csel", "csel w4, w6, w8, hs";
        csel(false, 9, 11, 13, LT) => "csel", "csel w9, w11, w13, lt";
        csel(true, 3, 5, 7, NE) => "csel", "csel x3, x5, x7, ne";
        csel(true, 4, 6, 8, LO) => "csel", "csel x4, x6, x8, lo";
        csel(true, 19, 21, 23, GT) => "csel", "csel x19, x21, x23, gt";
        csel(true, 20, 22, 24, LE) => "csel", "csel x20, x22, x24, le";
    );

    check_words!(cases, "csinc";
        csinc(false, 3, 5, 7, MI) => "csinc", "csinc w3, w5, w7, mi";
        csinc(true, 4, 6, 8, VC) => "csinc", "csinc x4, x6, x8, vc";
        csinc(false, 9, 11, 11, NE) => "cinc_csinc", "cinc w9, w11, eq";
        csinc(true, 12, 14, 14, LS) => "cinc_csinc", "cinc x12, x14, hi";
        csinc(false, 15, ZR, ZR, LT) => "cset_csinc", "cset w15, ge";
        csinc(true, 19, ZR, ZR, VC) => "cset_csinc", "cset x19, vs";
    );

    check_words!(cases, "cset";
        cset(3, EQ) => "cset_csinc", "cset w3, eq";
        cset(4, NE) => "cset_csinc", "cset w4, ne";
        cset(5, HS) => "cset_csinc", "cset w5, hs";
        cset(6, LO) => "cset_csinc", "cset w6, lo";
        cset(7, MI) => "cset_csinc", "cset w7, mi";
        cset(8, PL) => "cset_csinc", "cset w8, pl";
        cset(9, VS) => "cset_csinc", "cset w9, vs";
        cset(10, VC) => "cset_csinc", "cset w10, vc";
        cset(11, HI) => "cset_csinc", "cset w11, hi";
        cset(12, LS) => "cset_csinc", "cset w12, ls";
        cset(13, GE) => "cset_csinc", "cset w13, ge";
        cset(14, LT) => "cset_csinc", "cset w14, lt";
        cset(19, GT) => "cset_csinc", "cset w19, gt";
        cset(22, LE) => "cset_csinc", "cset w22, le";
    );

    check_words!(cases, "cneg";
        cneg(false, 3, 5, EQ) => "cneg_csneg", "cneg w3, w5, eq";
        cneg(false, 4, 6, NE) => "cneg_csneg", "cneg w4, w6, ne";
        cneg(true, 19, 21, LT) => "cneg_csneg", "cneg x19, x21, lt";
        cneg(true, 20, 22, MI) => "cneg_csneg", "cneg x20, x22, mi";
    );

    check_words!(cases, "clz";
        clz(false, 3, 19) => "clz_int", "clz w3, w19";
        clz(true, 4, 22) => "clz_int", "clz x4, x22";
    );

    check_words!(cases, "rbit";
        rbit(false, 3, 19) => "rbit_int", "rbit w3, w19";
        rbit(true, 4, 22) => "rbit_int", "rbit x4, x22";
    );

    check_words!(cases, "rev";
        rev(false, 3, 19) => "rev", "rev w3, w19";
        rev(true, 4, 22) => "rev", "rev x4, x22";
    );

    check_words!(cases, "mul";
        mul(false, 3, 5, 19) => "mul_madd", "mul w3, w5, w19";
        mul(true, 4, 6, 22) => "mul_madd", "mul x4, x6, x22";
    );

    check_words!(cases, "smulh";
        smulh(3, 5, 7) => "smulh", "smulh x3, x5, x7";
        smulh(19, 21, 23) => "smulh", "smulh x19, x21, x23";
    );

    check_words!(cases, "umulh";
        umulh(3, 5, 7) => "umulh", "umulh x3, x5, x7";
        umulh(19, 21, 23) => "umulh", "umulh x19, x21, x23";
    );

    check_words!(cases, "udiv";
        udiv(false, 3, 5, 19) => "udiv", "udiv w3, w5, w19";
        udiv(true, 4, 6, 22) => "udiv", "udiv x4, x6, x22";
    );

    check_words!(cases, "sdiv";
        sdiv(false, 3, 5, 19) => "sdiv", "sdiv w3, w5, w19";
        sdiv(true, 4, 6, 22) => "sdiv", "sdiv x4, x6, x22";
    );

    check_words!(cases, "msub";
        msub(false, 3, 5, 7, 9) => "msub", "msub w3, w5, w7, w9";
        msub(true, 4, 6, 8, 19) => "msub", "msub x4, x6, x8, x19";
        msub(false, 11, 13, 15, ZR) => "mneg_msub", "mneg w11, w13, w15";
        msub(true, 20, 22, 24, ZR) => "mneg_msub", "mneg x20, x22, x24";
    );
}

fn register_scalar_fp_and_add_imm(cases: &mut Cases) {
    check_words!(cases, "fmov_gf";
        fmov_gf(false, 3, 5) => "fmov_float_gen", "fmov s3, w5";
        fmov_gf(true, 19, 21) => "fmov_float_gen", "fmov d19, x21";
    );

    check_words!(cases, "fmov_fg";
        fmov_fg(false, 3, 5) => "fmov_float_gen", "fmov w3, s5";
        fmov_fg(true, 19, 21) => "fmov_float_gen", "fmov x19, d21";
    );

    check_words!(cases, "scvtf_gp";
        scvtf_gp(false, false, 3, 5) => "scvtf_float_int", "scvtf s3, w5";
        scvtf_gp(false, true, 4, 6) => "scvtf_float_int", "scvtf d4, w6";
        scvtf_gp(true, false, 19, 21) => "scvtf_float_int", "scvtf s19, x21";
        scvtf_gp(true, true, 20, 22) => "scvtf_float_int", "scvtf d20, x22";
    );

    check_words!(cases, "fcmp";
        fcmp(false, 5, 7) => "fcmp_float", "fcmp s5, s7";
        fcmp(true, 19, 23) => "fcmp_float", "fcmp d19, d23";
    );

    check_words!(cases, "fcmp_zero";
        fcmp_zero(false, 5) => "fcmp_float", "fcmp s5, #0.0";
        fcmp_zero(true, 19) => "fcmp_float", "fcmp d19, #0.0";
    );

    check_words!(cases, "s_fadd";
        s_fadd(false, 3, 5, 7) => "fadd_float", "fadd s3, s5, s7";
        s_fadd(true, 19, 21, 23) => "fadd_float", "fadd d19, d21, d23";
    );

    check_words!(cases, "s_fsub";
        s_fsub(false, 3, 5, 7) => "fsub_float", "fsub s3, s5, s7";
        s_fsub(true, 19, 21, 23) => "fsub_float", "fsub d19, d21, d23";
    );

    check_words!(cases, "s_fmul";
        s_fmul(false, 3, 5, 7) => "fmul_float", "fmul s3, s5, s7";
        s_fmul(true, 19, 21, 23) => "fmul_float", "fmul d19, d21, d23";
    );

    check_words!(cases, "s_fdiv";
        s_fdiv(false, 3, 5, 7) => "fdiv_float", "fdiv s3, s5, s7";
        s_fdiv(true, 19, 21, 23) => "fdiv_float", "fdiv d19, d21, d23";
    );

    check_words!(cases, "fcvt_d_s";
        fcvt_d_s(3, 5) => "fcvt_float", "fcvt d3, s5";
        fcvt_d_s(21, 27) => "fcvt_float", "fcvt d21, s27";
    );

    check_words!(cases, "fcvt_s_d";
        fcvt_s_d(3, 5) => "fcvt_float", "fcvt s3, d5";
        fcvt_s_d(21, 27) => "fcvt_float", "fcvt s21, d27";
    );

    check_words!(cases, "frintn_d";
        frintn_d(3, 5) => "frintn_float", "frintn d3, d5";
        frintn_d(21, 27) => "frintn_float", "frintn d21, d27";
    );

    check_words!(cases, "frintz_d";
        frintz_d(3, 5) => "frintz_float", "frintz d3, d5";
        frintz_d(21, 27) => "frintz_float", "frintz d21, d27";
    );

    check_words!(cases, "fcvtzs_d";
        fcvtzs_d(false, 3, 19) => "fcvtzs_float_int", "fcvtzs w3, d19";
        fcvtzs_d(true, 4, 21) => "fcvtzs_float_int", "fcvtzs x4, d21";
    );

    cases.check(
        "add_imm_32",
        &[
            ("add_addsub_imm", "add w3, w5, #0"),
            ("add_addsub_imm", "add w3, w5, #0x123"),
            ("sub_addsub_imm", "sub w4, w6, #0x123"),
            ("add_addsub_imm", "add w7, w9, #0x25, lsl #12"),
            ("sub_addsub_imm", "sub w8, w10, #0x25, lsl #12"),
            // Large immediate -> mov_imm into XS (w17) + add
            ("movz", "movz w17, #0x5678"),
            ("movk", "movk w17, #0x1234, lsl #16"),
            ("add_addsub_shift", "add w11, w13, w17"),
            // Large negative immediate (32-bit two's complement 0xedcb_a988)
            ("movz", "movz w17, #0xa988"),
            ("movk", "movk w17, #0xedcb, lsl #16"),
            ("add_addsub_shift", "add w12, w14, w17"),
            // Logical-immediate fallback into XS (w17) + add
            ("mov_orr_log_imm", "mov w17, #0x55555555"),
            ("add_addsub_shift", "add w15, w16, w17"),
        ],
        |a| {
            assert_eq!(XS, 17);
            a.add_imm(false, 3, 5, 0);
            a.add_imm(false, 3, 5, 0x123);
            a.add_imm(false, 4, 6, -0x123);
            a.add_imm(false, 7, 9, 0x25_000);
            a.add_imm(false, 8, 10, -0x25_000);
            a.add_imm(false, 11, 13, 0x1234_5678);
            a.add_imm(false, 12, 14, -0x1234_5678);
            a.add_imm(false, 15, 16, 0x5555_5555);
        },
    );

    cases.check(
        "add_imm_64",
        &[
            ("add_addsub_imm", "add x3, x5, #0x123"),
            ("sub_addsub_imm", "sub x4, x6, #0x123"),
            ("add_addsub_imm", "add x7, x9, #0x25, lsl #12"),
            ("sub_addsub_imm", "sub x8, x10, #0x25, lsl #12"),
            // Large positive immediate -> movz + movk into XS (x17) + add
            ("movz", "movz x17, #0x5678"),
            ("movk", "movk x17, #0x1234, lsl #16"),
            ("add_addsub_shift", "add x11, x13, x17"),
            // Large negative immediate -> movn + movk into XS (x17) + add
            ("movn", "movn x17, #0x5677"),
            ("movk", "movk x17, #0xedcb, lsl #16"),
            ("add_addsub_shift", "add x12, x14, x17"),
            // Logical-immediate fallback into XS (x17) + add
            ("mov_orr_log_imm", "mov x17, #0x5555555555555555"),
            ("add_addsub_shift", "add x19, x21, x17"),
        ],
        |a| {
            a.add_imm(true, 3, 5, 0x123);
            a.add_imm(true, 4, 6, -0x123);
            a.add_imm(true, 7, 9, 0x25_000);
            a.add_imm(true, 8, 10, -0x25_000);
            a.add_imm(true, 11, 13, 0x1234_5678);
            a.add_imm(true, 12, 14, -0x1234_5678);
            a.add_imm(true, 19, 21, 0x5555_5555_5555_5555);
        },
    );
}

fn register_loads_stores_barriers_sysregs(cases: &mut Cases) {
    check_words!(cases, "ldst_ui";
        ldst_ui(true, 8, 3, 5, 7) => "ldrb_imm", "ldrb w3, [x5, #7]";
        ldst_ui(false, 8, 4, 6, 4095) => "strb_imm", "strb w4, [x6, #4095]";
        ldst_ui(true, 16, 7, 9, 14) => "ldrh_imm", "ldrh w7, [x9, #14]";
        ldst_ui(false, 16, 8, 10, 8190) => "strh_imm", "strh w8, [x10, #8190]";
        ldst_ui(true, 32, 11, 13, 28) => "ldr_imm_gen", "ldr w11, [x13, #28]";
        ldst_ui(false, 32, 12, SP, 16380) => "str_imm_gen", "str w12, [sp, #16380]";
        ldst_ui(true, 64, 14, 16, 56) => "ldr_imm_gen", "ldr x14, [x16, #56]";
        ldst_ui(false, 64, 15, 18, 32760) => "str_imm_gen", "str x15, [x18, #32760]";
        ldst_ui(true, 128, 19, 21, 112) => "ldr_imm_fpsimd", "ldr q19, [x21, #112]";
        ldst_ui(false, 128, 20, SP, 65520) => "str_imm_fpsimd", "str q20, [sp, #65520]";
    );

    check_words!(cases, "ldr";
        ldr(8, 3, 5, 0) => "ldrb_imm", "ldrb w3, [x5]";
        ldr(16, 4, 6, 18) => "ldrh_imm", "ldrh w4, [x6, #18]";
        ldr(32, 7, 9, 36) => "ldr_imm_gen", "ldr w7, [x9, #36]";
        ldr(64, 11, SP, 72) => "ldr_imm_gen", "ldr x11, [sp, #72]";
        ldr(128, 19, 21, 144) => "ldr_imm_fpsimd", "ldr q19, [x21, #144]";
    );

    check_words!(cases, "str";
        str(8, 3, 5, 0) => "strb_imm", "strb w3, [x5]";
        str(16, 4, 6, 18) => "strh_imm", "strh w4, [x6, #18]";
        str(32, 7, 9, 36) => "str_imm_gen", "str w7, [x9, #36]";
        str(64, 11, SP, 72) => "str_imm_gen", "str x11, [sp, #72]";
        str(128, 19, 21, 144) => "str_imm_fpsimd", "str q19, [x21, #144]";
    );

    check_words!(cases, "stp_x";
        stp_x(3, 4, 5, 0) => "stp_gen", "stp x3, x4, [x5]";
        stp_x(6, 7, 8, 24) => "stp_gen", "stp x6, x7, [x8, #24]";
        stp_x(9, 10, 11, -48) => "stp_gen", "stp x9, x10, [x11, #-48]";
        stp_x(12, 13, SP, -512) => "stp_gen", "stp x12, x13, [sp, #-512]";
        stp_x(19, 20, 21, 504) => "stp_gen", "stp x19, x20, [x21, #504]";
    );

    check_words!(cases, "ldp_x";
        ldp_x(3, 4, 5, 0) => "ldp_gen", "ldp x3, x4, [x5]";
        ldp_x(6, 7, 8, 24) => "ldp_gen", "ldp x6, x7, [x8, #24]";
        ldp_x(9, 10, 11, -48) => "ldp_gen", "ldp x9, x10, [x11, #-48]";
        ldp_x(12, 13, SP, -512) => "ldp_gen", "ldp x12, x13, [sp, #-512]";
        ldp_x(19, 20, 21, 504) => "ldp_gen", "ldp x19, x20, [x21, #504]";
    );

    check_words!(cases, "stp_q";
        stp_q(3, 4, 5, 0) => "stp_fpsimd", "stp q3, q4, [x5]";
        stp_q(6, 7, 8, 48) => "stp_fpsimd", "stp q6, q7, [x8, #48]";
        stp_q(9, 10, 11, -96) => "stp_fpsimd", "stp q9, q10, [x11, #-96]";
        stp_q(12, 13, SP, -1024) => "stp_fpsimd", "stp q12, q13, [sp, #-1024]";
        stp_q(19, 20, 21, 1008) => "stp_fpsimd", "stp q19, q20, [x21, #1008]";
    );

    check_words!(cases, "ldp_q";
        ldp_q(3, 4, 5, 0) => "ldp_fpsimd", "ldp q3, q4, [x5]";
        ldp_q(6, 7, 8, 48) => "ldp_fpsimd", "ldp q6, q7, [x8, #48]";
        ldp_q(9, 10, 11, -96) => "ldp_fpsimd", "ldp q9, q10, [x11, #-96]";
        ldp_q(12, 13, SP, -1024) => "ldp_fpsimd", "ldp q12, q13, [sp, #-1024]";
        ldp_q(19, 20, 21, 1008) => "ldp_fpsimd", "ldp q19, q20, [x21, #1008]";
    );

    check_words!(cases, "ldur_q";
        ldur_q(3, 5, 0) => "ldur_fpsimd", "ldur q3, [x5]";
        ldur_q(4, 6, 29) => "ldur_fpsimd", "ldur q4, [x6, #29]";
        ldur_q(7, 9, -37) => "ldur_fpsimd", "ldur q7, [x9, #-37]";
        ldur_q(11, SP, -256) => "ldur_fpsimd", "ldur q11, [sp, #-256]";
        ldur_q(19, 21, 255) => "ldur_fpsimd", "ldur q19, [x21, #255]";
    );

    check_words!(cases, "stur_q";
        stur_q(3, 5, 0) => "stur_fpsimd", "stur q3, [x5]";
        stur_q(4, 6, 29) => "stur_fpsimd", "stur q4, [x6, #29]";
        stur_q(7, 9, -37) => "stur_fpsimd", "stur q7, [x9, #-37]";
        stur_q(11, SP, -256) => "stur_fpsimd", "stur q11, [sp, #-256]";
        stur_q(19, 21, 255) => "stur_fpsimd", "stur q19, [x21, #255]";
    );

    check_words!(cases, "ldapr";
        ldapr(8, 3, 5) => "ldaprb", "ldaprb w3, [x5]";
        ldapr(16, 4, 6) => "ldaprh", "ldaprh w4, [x6]";
        ldapr(32, 7, SP) => "ldapr", "ldapr w7, [sp]";
        ldapr(64, 19, 21) => "ldapr", "ldapr x19, [x21]";
    );

    check_words!(cases, "stlr";
        stlr(8, 3, 5) => "stlrb", "stlrb w3, [x5]";
        stlr(16, 4, 6) => "stlrh", "stlrh w4, [x6]";
        stlr(32, 7, SP) => "stlr", "stlr w7, [sp]";
        stlr(64, 19, 21) => "stlr", "stlr x19, [x21]";
    );

    check_words!(cases, "ldur_stur";
        ldur_stur(true, 8, 3, 5, -13) => "ldurb", "ldurb w3, [x5, #-13]";
        ldur_stur(false, 8, 4, 6, 19) => "sturb", "sturb w4, [x6, #19]";
        ldur_stur(true, 16, 7, 9, -256) => "ldurh", "ldurh w7, [x9, #-256]";
        ldur_stur(false, 16, 8, 10, 255) => "sturh", "sturh w8, [x10, #255]";
        ldur_stur(true, 32, 11, 13, 0) => "ldur_gen", "ldur w11, [x13]";
        ldur_stur(false, 32, 12, SP, -21) => "stur_gen", "stur w12, [sp, #-21]";
        ldur_stur(true, 64, 19, 21, 37) => "ldur_gen", "ldur x19, [x21, #37]";
        ldur_stur(false, 64, 20, 22, -43) => "stur_gen", "stur x20, [x22, #-43]";
    );

    check_words!(cases, "ldst_reg";
        ldst_reg(true, 8, 3, 5, 6, false) => "ldrb_reg", "ldrb w3, [x5, x6]";
        ldst_reg(false, 8, 4, 5, 6, false) => "strb_reg", "strb w4, [x5, x6]";
        ldst_reg(true, 16, 7, 9, 10, false) => "ldrh_reg", "ldrh w7, [x9, x10]";
        ldst_reg(true, 16, 7, 9, 10, true) => "ldrh_reg", "ldrh w7, [x9, x10, lsl #1]";
        ldst_reg(false, 16, 8, 9, 10, true) => "strh_reg", "strh w8, [x9, x10, lsl #1]";
        ldst_reg(true, 32, 11, 13, 14, false) => "ldr_reg_gen", "ldr w11, [x13, x14]";
        ldst_reg(true, 32, 11, 13, 14, true) => "ldr_reg_gen", "ldr w11, [x13, x14, lsl #2]";
        ldst_reg(false, 32, 12, 13, 14, true) => "str_reg_gen", "str w12, [x13, x14, lsl #2]";
        ldst_reg(true, 64, 19, 21, 22, false) => "ldr_reg_gen", "ldr x19, [x21, x22]";
        ldst_reg(true, 64, 19, 21, 22, true) => "ldr_reg_gen", "ldr x19, [x21, x22, lsl #3]";
        ldst_reg(false, 64, 20, 21, 22, true) => "str_reg_gen", "str x20, [x21, x22, lsl #3]";
    );

    check_words!(cases, "ldapur_stlur";
        ldapur_stlur(true, 8, 3, 5, -13) => "ldapurb", "ldapurb w3, [x5, #-13]";
        ldapur_stlur(false, 8, 4, 6, 19) => "stlurb", "stlurb w4, [x6, #19]";
        ldapur_stlur(true, 16, 7, 9, -256) => "ldapurh", "ldapurh w7, [x9, #-256]";
        ldapur_stlur(false, 16, 8, 10, 255) => "stlurh", "stlurh w8, [x10, #255]";
        ldapur_stlur(true, 32, 11, 13, 0) => "ldapur_gen", "ldapur w11, [x13]";
        ldapur_stlur(false, 32, 12, SP, -21) => "stlur_gen", "stlur w12, [sp, #-21]";
        ldapur_stlur(true, 64, 19, 21, 37) => "ldapur_gen", "ldapur x19, [x21, #37]";
        ldapur_stlur(false, 64, 20, 22, -43) => "stlur_gen", "stlur x20, [x22, #-43]";
    );

    check_words!(cases, "str_pre";
        str_pre(3, 5, -16) => "str_imm_gen", "str x3, [x5, #-16]!";
        str_pre(4, SP, -256) => "str_imm_gen", "str x4, [sp, #-256]!";
        str_pre(19, 21, 255) => "str_imm_gen", "str x19, [x21, #255]!";
    );

    check_words!(cases, "ldr_post";
        ldr_post(3, 5, 16) => "ldr_imm_gen", "ldr x3, [x5], #16";
        ldr_post(4, SP, -256) => "ldr_imm_gen", "ldr x4, [sp], #-256";
        ldr_post(19, 21, 255) => "ldr_imm_gen", "ldr x19, [x21], #255";
    );

    check_words!(cases, "stp_pre";
        stp_pre(3, 4, SP, -32) => "stp_gen", "stp x3, x4, [sp, #-32]!";
        stp_pre(6, 7, 8, -512) => "stp_gen", "stp x6, x7, [x8, #-512]!";
        stp_pre(19, 20, 21, 504) => "stp_gen", "stp x19, x20, [x21, #504]!";
    );

    check_words!(cases, "ldp_post";
        ldp_post(3, 4, SP, 32) => "ldp_gen", "ldp x3, x4, [sp], #32";
        ldp_post(6, 7, 8, -512) => "ldp_gen", "ldp x6, x7, [x8], #-512";
        ldp_post(19, 20, 21, 504) => "ldp_gen", "ldp x19, x20, [x21], #504";
    );

    cases.check("dmb_ish", &[("dmb", "dmb ish")], |a| {
        a.dmb_ish();
    });

    cases.check("dmb_ishld", &[("dmb", "dmb ishld")], |a| {
        a.dmb_ishld();
    });

    cases.check("dmb_ishst", &[("dmb", "dmb ishst")], |a| {
        a.dmb_ishst();
    });

    cases.check("dsb_ish", &[("dsb", "dsb ish")], |a| {
        a.dsb_ish();
    });

    cases.check("dsb_ishld", &[("dsb", "dsb ishld")], |a| {
        a.dsb_ishld();
    });

    cases.check("isb", &[("isb", "isb")], |a| {
        a.isb();
    });

    // CNTVCTSS_EL0 in generic system-register syntax for the baseline apple-m1 assembler.
    check_words!(cases, "mrs_cntvct";
        mrs_cntvct(3) => "mrs", "mrs x3, cntvct_el0";
        mrs_cntvct(22) => "mrs", "mrs x22, cntvct_el0";
        mrs_cntvctss(3) => "mrs", "mrs x3, S3_3_C14_C0_6";
        mrs_cntvctss(22) => "mrs", "mrs x22, S3_3_C14_C0_6";
    );

    check_words!(cases, "mrs_cntfrq";
        mrs_cntfrq(3) => "mrs", "mrs x3, cntfrq_el0";
        mrs_cntfrq(22) => "mrs", "mrs x22, cntfrq_el0";
    );

    check_words!(cases, "mrs_fpcr";
        mrs_fpcr(5) => "mrs", "mrs x5, fpcr";
        mrs_fpcr(24) => "mrs", "mrs x24, fpcr";
    );

    check_words!(cases, "msr_fpcr";
        msr_fpcr(7) => "msr_reg", "msr fpcr, x7";
        msr_fpcr(26) => "msr_reg", "msr fpcr, x26";
    );

    check_words!(cases, "mrs_fpsr";
        mrs_fpsr(5) => "mrs", "mrs x5, fpsr";
        mrs_fpsr(24) => "mrs", "mrs x24, fpsr";
    );

    check_words!(cases, "msr_fpsr";
        msr_fpsr(7) => "msr_reg", "msr fpsr, x7";
        msr_fpsr(26) => "msr_reg", "msr fpsr, x26";
    );

    check_words!(cases, "mrs_nzcv";
        mrs_nzcv(5) => "mrs", "mrs x5, nzcv";
        mrs_nzcv(24) => "mrs", "mrs x24, nzcv";
    );

    check_words!(cases, "msr_nzcv";
        msr_nzcv(7) => "msr_reg", "msr nzcv, x7";
        msr_nzcv(26) => "msr_reg", "msr nzcv, x26";
    );
}

fn register_branches(cases: &mut Cases) {
    cases.check(
        "b_patch_bind",
        &[
            ("b_uncond", "b #8"),
            ("nop", "nop"),
            ("b_uncond", "b #-8"),
            ("b_uncond", "b #0"),
        ],
        |a| {
            let fwd = a.b();
            let raw = a.read(fwd);
            a.emit(NOP_WORD);
            a.bind(fwd);
            assert_ne!(a.read(fwd), raw);

            let bwd = a.b();
            // Patch forward first, then re-patch backward to confirm bits overwrite cleanly.
            a.patch(bwd, 4);
            a.patch(bwd, 0);

            let _unpatched = a.b();
        },
    );

    cases.check(
        "b_to",
        &[
            ("b_uncond", "b #12"),
            ("b_uncond", "b #0"),
            ("b_uncond", "b #-8"),
        ],
        |a| {
            a.b_to(0x100c);
            let self_addr = a.addr_of(a.pos);
            a.b_to(self_addr);
            a.b_to(0x1000);
        },
    );

    cases.check(
        "bcond",
        &[
            ("b_cond", "b.eq #8"),
            ("nop", "nop"),
            ("b_cond", "b.ne #-8"),
            ("b_cond", "b.hs #8"),
            ("b_cond", "b.lo #8"),
            ("b_cond", "b.mi #8"),
            ("b_cond", "b.pl #8"),
            ("b_cond", "b.vs #8"),
            ("b_cond", "b.vc #8"),
            ("b_cond", "b.hi #8"),
            ("b_cond", "b.ls #8"),
            ("b_cond", "b.ge #8"),
            ("b_cond", "b.lt #-8"),
            ("b_cond", "b.gt #8"),
            ("b_cond", "b.le #-8"),
        ],
        |a| {
            let eq = a.bcond(EQ);
            a.emit(NOP_WORD);
            a.bind(eq);

            let ne = a.bcond(NE);
            a.patch(ne, 0);

            for &(cond, delta) in &[
                (HS, 2isize),
                (LO, 2),
                (MI, 2),
                (PL, 2),
                (VS, 2),
                (VC, 2),
                (HI, 2),
                (LS, 2),
                (GE, 2),
                (LT, -2),
                (GT, 2),
                (LE, -2),
            ] {
                let at = a.bcond(cond);
                let target = (at as isize + delta) as usize;
                a.patch(at, target);
            }
        },
    );

    cases.check(
        "cbz",
        &[
            ("cbz", "cbz w5, #8"),
            ("nop", "nop"),
            ("cbz", "cbz w6, #-8"),
            ("cbz", "cbz x19, #8"),
            ("nop", "nop"),
            ("cbz", "cbz x23, #-12"),
        ],
        |a| {
            let fw = a.cbz(false, 5);
            a.emit(NOP_WORD);
            a.bind(fw);

            let bw = a.cbz(false, 6);
            a.patch(bw, 0);

            let fx = a.cbz(true, 19);
            a.emit(NOP_WORD);
            a.bind(fx);

            let bx = a.cbz(true, 23);
            a.patch(bx, 2);
        },
    );

    cases.check(
        "cbnz",
        &[
            ("cbnz", "cbnz w5, #8"),
            ("nop", "nop"),
            ("cbnz", "cbnz w6, #-8"),
            ("cbnz", "cbnz x19, #8"),
            ("nop", "nop"),
            ("cbnz", "cbnz x23, #-12"),
        ],
        |a| {
            let fw = a.cbnz(false, 5);
            a.emit(NOP_WORD);
            a.bind(fw);

            let bw = a.cbnz(false, 6);
            a.patch(bw, 0);

            let fx = a.cbnz(true, 19);
            a.emit(NOP_WORD);
            a.bind(fx);

            let bx = a.cbnz(true, 23);
            a.patch(bx, 2);
        },
    );

    cases.check(
        "tbnz",
        &[
            ("tbnz", "tbnz w5, #0, #8"),
            ("nop", "nop"),
            ("tbnz", "tbnz w6, #7, #-8"),
            ("tbnz", "tbnz w19, #31, #8"),
            ("nop", "nop"),
            ("tbnz", "tbnz x7, #32, #-8"),
            ("tbnz", "tbnz x21, #45, #8"),
            ("nop", "nop"),
            ("tbnz", "tbnz x24, #63, #-12"),
        ],
        |a| {
            let b0 = a.tbnz(5, 0);
            a.emit(NOP_WORD);
            a.bind(b0);

            let b7 = a.tbnz(6, 7);
            a.patch(b7, 0);

            let b31 = a.tbnz(19, 31);
            a.emit(NOP_WORD);
            a.bind(b31);

            let b32 = a.tbnz(7, 32);
            a.patch(b32, 3);

            let b45 = a.tbnz(21, 45);
            a.emit(NOP_WORD);
            a.bind(b45);

            let b63 = a.tbnz(24, 63);
            a.patch(b63, 5);
        },
    );

    cases.check("br", &[("br", "br x9"), ("br", "br x30")], |a| {
        a.br(9);
        a.br(30);
    });

    cases.check("ret", &[("ret", "ret")], |a| {
        a.ret();
    });

    cases.check("ret_r", &[("ret", "ret x9"), ("ret", "ret x30")], |a| {
        a.ret_r(9);
        a.ret_r(30);
    });

    cases.check(
        "bl",
        &[("bl", "bl #8"), ("nop", "nop"), ("bl", "bl #-8")],
        |a| {
            let fwd = a.bl();
            a.emit(NOP_WORD);
            a.bind(fwd);

            let bwd = a.bl();
            a.patch(bwd, 0);
        },
    );

    cases.check(
        "bl_to",
        &[("bl", "bl #16"), ("bl", "bl #0"), ("bl", "bl #-8")],
        |a| {
            a.bl_to(0x1010);
            let self_addr = a.addr_of(a.pos);
            a.bl_to(self_addr);
            a.bl_to(0x1000);
        },
    );

    cases.check(
        "adr_and_patch_adr",
        &[
            ("adr", "adr x3, #0"),
            ("adr", "adr x5, #12"),
            ("nop", "nop"),
            ("adr", "adr x19, #-12"),
            ("adr", "adr x22, #-8"),
        ],
        |a| {
            let unpatched = a.adr(3);
            let unpatched_word = a.read(unpatched);

            let fwd = a.adr(5);
            a.emit(NOP_WORD);
            let bwd = a.adr(19);
            let repatch = a.adr(22);

            a.patch_adr(fwd, 4);
            a.patch_adr(bwd, 0);
            // Re-patch an already-patched adr to verify old offset bits are cleared.
            a.patch_adr(repatch, 0);
            a.patch_adr(repatch, 2);

            assert_eq!(a.read(unpatched), unpatched_word);
        },
    );

    cases.check("blr", &[("blr", "blr x9"), ("blr", "blr x22")], |a| {
        a.blr(9);
        a.blr(22);
    });
}
