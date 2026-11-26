//! A64 opcode encoding test cases for all `Asm` methods defined in `xmm.rs`, `more.rs`, and
//! `ext.rs`.

use super::Cases;

pub(super) fn register(cases: &mut Cases) {
    register_xmm(cases);
    register_more(cases);
    register_ext(cases);
}

fn register_xmm(cases: &mut Cases) {
    check_words!(cases, ldr_v(32, 5, 19, 108) => "ldr_imm_fpsimd", "ldr s5, [x19, #108]");
    check_words!(cases, ldr_v(64, 6, 20, 296) => "ldr_imm_fpsimd", "ldr d6, [x20, #296]");
    check_words!(cases, ldr_v(128, 7, 21, 752) => "ldr_imm_fpsimd", "ldr q7, [x21, #752]");

    check_words!(cases, str_v(32, 8, 22, 116) => "str_imm_fpsimd", "str s8, [x22, #116]");
    check_words!(cases, str_v(64, 9, 23, 312) => "str_imm_fpsimd", "str d9, [x23, #312]");
    check_words!(cases, str_v(128, 10, 24, 784) => "str_imm_fpsimd", "str q10, [x24, #784]");

    check_words!(cases, ldur_stur_v(true, 32, 4, 17, 29) => "ldur_fpsimd", "ldur s4, [x17, #29]");
    check_words!(cases, ldur_stur_v(true, 32, 5, 18, -19) => "ldur_fpsimd", "ldur s5, [x18, #-19]");
    check_words!(cases, ldur_stur_v(false, 32, 6, 19, 37) => "stur_fpsimd", "stur s6, [x19, #37]");
    check_words!(cases, ldur_stur_v(false, 32, 7, 20, -27) => "stur_fpsimd", "stur s7, [x20, #-27]");
    check_words!(cases, ldur_stur_v(true, 64, 8, 21, 45) => "ldur_fpsimd", "ldur d8, [x21, #45]");
    check_words!(cases, ldur_stur_v(true, 64, 9, 22, -35) => "ldur_fpsimd", "ldur d9, [x22, #-35]");
    check_words!(cases, ldur_stur_v(false, 64, 10, 23, 53) => "stur_fpsimd", "stur d10, [x23, #53]");
    check_words!(cases, ldur_stur_v(false, 64, 11, 24, -43) => "stur_fpsimd", "stur d11, [x24, #-43]");

    check_words!(cases, ldr_q_lit(19) => "ldr_lit_fpsimd", "ldr q19, #0");
    check_words!(cases, ldr_lit(21) => "ldr_lit_gen", "ldr x21, #0");

    cases.check(
        "patch_ldr_lit_q_fwd",
        &[
            ("clrex", "clrex"),
            ("ldr_lit_fpsimd", "ldr q19, #16"),
            ("clrex", "clrex"),
            ("clrex", "clrex"),
            ("clrex", "clrex"),
            ("clrex", "clrex"),
        ],
        |a| {
            a.clrex();
            let at = a.ldr_q_lit(19);
            a.clrex();
            a.clrex();
            a.clrex();
            let target = a.pos;
            a.clrex();
            a.patch_ldr_lit(at, target);
        },
    );
    cases.check(
        "patch_ldr_lit_q_bwd",
        &[
            ("clrex", "clrex"),
            ("clrex", "clrex"),
            ("clrex", "clrex"),
            ("clrex", "clrex"),
            ("ldr_lit_fpsimd", "ldr q20, #-12"),
        ],
        |a| {
            a.clrex();
            let target = a.pos;
            a.clrex();
            a.clrex();
            a.clrex();
            let at = a.ldr_q_lit(20);
            a.patch_ldr_lit(at, target);
        },
    );
    cases.check(
        "patch_ldr_lit_x_fwd",
        &[
            ("clrex", "clrex"),
            ("ldr_lit_gen", "ldr x21, #12"),
            ("clrex", "clrex"),
            ("clrex", "clrex"),
            ("clrex", "clrex"),
        ],
        |a| {
            a.clrex();
            let at = a.ldr_lit(21);
            a.clrex();
            a.clrex();
            let target = a.pos;
            a.clrex();
            a.patch_ldr_lit(at, target);
        },
    );
    cases.check(
        "patch_ldr_lit_x_bwd",
        &[
            ("clrex", "clrex"),
            ("clrex", "clrex"),
            ("clrex", "clrex"),
            ("ldr_lit_gen", "ldr x22, #-8"),
        ],
        |a| {
            a.clrex();
            let target = a.pos;
            a.clrex();
            a.clrex();
            let at = a.ldr_lit(22);
            a.patch_ldr_lit(at, target);
        },
    );

    check_words!(cases, fmov_s(false, 4, 19) => "fmov_float", "fmov s4, s19");
    check_words!(cases, fmov_s(true, 5, 20) => "fmov_float", "fmov d5, d20");

    check_words!(cases, v_ins_lane(0, 4, 11, 19, 5) => "ins_advsimd_elt", "ins v4.b[11], v19.b[5]");
    check_words!(cases, v_ins_lane(1, 5, 6, 20, 3) => "ins_advsimd_elt", "ins v5.h[6], v20.h[3]");
    check_words!(cases, v_ins_lane(2, 6, 3, 21, 2) => "ins_advsimd_elt", "ins v6.s[3], v21.s[2]");
    check_words!(cases, v_ins_lane(3, 7, 1, 22, 1) => "ins_advsimd_elt", "ins v7.d[1], v22.d[1]");
}

fn register_more(cases: &mut Cases) {
    check_words!(cases, tbz(5, 19) => "tbz", "tbz w5, #19, #0");
    check_words!(cases, tbz(18, 45) => "tbz", "tbz x18, #45, #0");
    cases.check(
        "tbz_w_fwd",
        &[
            ("clrex", "clrex"),
            ("tbz", "tbz w7, #23, #12"),
            ("clrex", "clrex"),
            ("clrex", "clrex"),
            ("clrex", "clrex"),
        ],
        |a| {
            a.clrex();
            let at = a.tbz(7, 23);
            a.clrex();
            a.clrex();
            a.bind(at);
            a.clrex();
        },
    );
    cases.check(
        "tbz_w_bwd",
        &[
            ("clrex", "clrex"),
            ("clrex", "clrex"),
            ("clrex", "clrex"),
            ("tbz", "tbz w9, #29, #-8"),
        ],
        |a| {
            a.clrex();
            let target = a.pos;
            a.clrex();
            a.clrex();
            let at = a.tbz(9, 29);
            a.patch(at, target);
        },
    );
    cases.check(
        "tbz_x_fwd",
        &[
            ("clrex", "clrex"),
            ("tbz", "tbz x20, #37, #16"),
            ("clrex", "clrex"),
            ("clrex", "clrex"),
            ("clrex", "clrex"),
            ("clrex", "clrex"),
        ],
        |a| {
            a.clrex();
            let at = a.tbz(20, 37);
            a.clrex();
            a.clrex();
            a.clrex();
            a.bind(at);
            a.clrex();
        },
    );
    cases.check(
        "tbz_x_bwd",
        &[
            ("clrex", "clrex"),
            ("clrex", "clrex"),
            ("clrex", "clrex"),
            ("clrex", "clrex"),
            ("tbz", "tbz x22, #53, #-12"),
        ],
        |a| {
            a.clrex();
            let target = a.pos;
            a.clrex();
            a.clrex();
            a.clrex();
            let at = a.tbz(22, 53);
            a.patch(at, target);
        },
    );

    check_words!(cases, v_aese(5, 19) => "aese_advsimd", "aese v5.16b, v19.16b");
    check_words!(cases, v_aesd(6, 20) => "aesd_advsimd", "aesd v6.16b, v20.16b");
    check_words!(cases, v_aesmc(7, 21) => "aesmc_advsimd", "aesmc v7.16b, v21.16b");
    check_words!(cases, v_aesimc(8, 22) => "aesimc_advsimd", "aesimc v8.16b, v22.16b");
    check_words!(cases, v_rev64_4s(9, 23) => "rev64_advsimd", "rev64 v9.4s, v23.4s");

    check_words!(cases, v_sha256h(4, 17, 25) => "sha256h_advsimd", "sha256h q4, q17, v25.4s");
    check_words!(cases, v_sha256h2(5, 18, 26) => "sha256h2_advsimd", "sha256h2 q5, q18, v26.4s");
    check_words!(cases, v_sha256su0(6, 19) => "sha256su0_advsimd", "sha256su0 v6.4s, v19.4s");
    check_words!(cases, v_sha256su1(7, 20, 27) => "sha256su1_advsimd", "sha256su1 v7.4s, v20.4s, v27.4s");
    check_words!(cases, v_sha1c(8, 21, 28) => "sha1c_advsimd", "sha1c q8, s21, v28.4s");
    check_words!(cases, v_sha1p(9, 22, 29) => "sha1p_advsimd", "sha1p q9, s22, v29.4s");
    check_words!(cases, v_sha1m(10, 23, 30) => "sha1m_advsimd", "sha1m q10, s23, v30.4s");
    check_words!(cases, v_sha1su1(11, 24) => "sha1su1_advsimd", "sha1su1 v11.4s, v24.4s");

    check_words!(cases, v_fcvtl_h(false, 5, 21) => "fcvtl_advsimd", "fcvtl v5.4s, v21.4h");
    check_words!(cases, v_fcvtl_h(true, 6, 22) => "fcvtl_advsimd", "fcvtl2 v6.4s, v22.8h");

    check_words!(cases, v_fcvtn_h(false, 7, 23) => "fcvtn_advsimd", "fcvtn v7.4h, v23.4s");
    check_words!(cases, v_fcvtn_h(true, 8, 24) => "fcvtn_advsimd", "fcvtn2 v8.8h, v24.4s");

    check_words!(cases, ldaddal(8, 3, 5, 19) => "ldaddb", "ldaddalb w3, w5, [x19]");
    check_words!(cases, ldaddal(16, 4, 6, 20) => "ldaddh", "ldaddalh w4, w6, [x20]");
    check_words!(cases, ldaddal(32, 5, 7, 21) => "ldadd", "ldaddal w5, w7, [x21]");
    check_words!(cases, ldaddal(64, 6, 8, 22) => "ldadd", "ldaddal x6, x8, [x22]");

    check_words!(cases, ldclral(8, 3, 5, 19) => "ldclrb", "ldclralb w3, w5, [x19]");
    check_words!(cases, ldclral(16, 4, 6, 20) => "ldclrh", "ldclralh w4, w6, [x20]");
    check_words!(cases, ldclral(32, 5, 7, 21) => "ldclr", "ldclral w5, w7, [x21]");
    check_words!(cases, ldclral(64, 6, 8, 22) => "ldclr", "ldclral x6, x8, [x22]");

    check_words!(cases, ldeoral(8, 3, 5, 19) => "ldeorb", "ldeoralb w3, w5, [x19]");
    check_words!(cases, ldeoral(16, 4, 6, 20) => "ldeorh", "ldeoralh w4, w6, [x20]");
    check_words!(cases, ldeoral(32, 5, 7, 21) => "ldeor", "ldeoral w5, w7, [x21]");
    check_words!(cases, ldeoral(64, 6, 8, 22) => "ldeor", "ldeoral x6, x8, [x22]");

    check_words!(cases, ldsetal(8, 3, 5, 19) => "ldsetb", "ldsetalb w3, w5, [x19]");
    check_words!(cases, ldsetal(16, 4, 6, 20) => "ldseth", "ldsetalh w4, w6, [x20]");
    check_words!(cases, ldsetal(32, 5, 7, 21) => "ldset", "ldsetal w5, w7, [x21]");
    check_words!(cases, ldsetal(64, 6, 8, 22) => "ldset", "ldsetal x6, x8, [x22]");

    check_words!(cases, swpal(8, 3, 5, 19) => "swpb", "swpalb w3, w5, [x19]");
    check_words!(cases, swpal(16, 4, 6, 20) => "swph", "swpalh w4, w6, [x20]");
    check_words!(cases, swpal(32, 5, 7, 21) => "swp", "swpal w5, w7, [x21]");
    check_words!(cases, swpal(64, 6, 8, 22) => "swp", "swpal x6, x8, [x22]");

    check_words!(cases, casal(8, 3, 5, 19) => "casb", "casalb w3, w5, [x19]");
    check_words!(cases, casal(16, 4, 6, 20) => "cash", "casalh w4, w6, [x20]");
    check_words!(cases, casal(32, 5, 7, 21) => "cas", "casal w5, w7, [x21]");
    check_words!(cases, casal(64, 6, 8, 22) => "cas", "casal x6, x8, [x22]");

    check_words!(cases, caspal_x(0, 2, 16) => "casp", "caspal x0, x1, x2, x3, [x16]");
    check_words!(cases, yield_hint() => "yield", "yield");
    check_words!(cases, clrex() => "clrex", "clrex");

    check_words!(cases, ldr_reg(8, false, 5, 11, 17, false) => "ldrb_reg", "ldrb w5, [x11, x17]");
    check_words!(cases, ldr_reg(8, false, 5, 11, 17, true) => "ldrb_reg", "ldrb w5, [x11, x17, lsl #0]");
    check_words!(cases, ldr_reg(8, true, 5, 11, 17, false) => "ldrsb_reg", "ldrsb x5, [x11, x17]");
    check_words!(cases, ldr_reg(8, true, 5, 11, 17, true) => "ldrsb_reg", "ldrsb x5, [x11, x17, lsl #0]");
    check_words!(cases, ldr_reg(16, false, 6, 12, 18, false) => "ldrh_reg", "ldrh w6, [x12, x18]");
    check_words!(cases, ldr_reg(16, false, 6, 12, 18, true) => "ldrh_reg", "ldrh w6, [x12, x18, lsl #1]");
    check_words!(cases, ldr_reg(16, true, 6, 12, 18, false) => "ldrsh_reg", "ldrsh x6, [x12, x18]");
    check_words!(cases, ldr_reg(16, true, 6, 12, 18, true) => "ldrsh_reg", "ldrsh x6, [x12, x18, lsl #1]");
    check_words!(cases, ldr_reg(32, false, 7, 13, 19, false) => "ldr_reg_gen", "ldr w7, [x13, x19]");
    check_words!(cases, ldr_reg(32, false, 7, 13, 19, true) => "ldr_reg_gen", "ldr w7, [x13, x19, lsl #2]");
    check_words!(cases, ldr_reg(32, true, 7, 13, 19, false) => "ldrsw_reg", "ldrsw x7, [x13, x19]");
    check_words!(cases, ldr_reg(32, true, 7, 13, 19, true) => "ldrsw_reg", "ldrsw x7, [x13, x19, lsl #2]");
    check_words!(cases, ldr_reg(64, false, 8, 14, 20, false) => "ldr_reg_gen", "ldr x8, [x14, x20]");
    check_words!(cases, ldr_reg(64, false, 8, 14, 20, true) => "ldr_reg_gen", "ldr x8, [x14, x20, lsl #3]");
}

fn register_ext(cases: &mut Cases) {
    check_words!(cases, v_xtl(false, 0, false, 4, 19) => "sxtl_sshll_advsimd", "sxtl v4.8h, v19.8b");
    check_words!(cases, v_xtl(false, 0, true, 4, 19) => "sxtl_sshll_advsimd", "sxtl2 v4.8h, v19.16b");
    check_words!(cases, v_xtl(false, 1, false, 5, 20) => "sxtl_sshll_advsimd", "sxtl v5.4s, v20.4h");
    check_words!(cases, v_xtl(false, 1, true, 5, 20) => "sxtl_sshll_advsimd", "sxtl2 v5.4s, v20.8h");
    check_words!(cases, v_xtl(false, 2, false, 6, 21) => "sxtl_sshll_advsimd", "sxtl v6.2d, v21.2s");
    check_words!(cases, v_xtl(false, 2, true, 6, 21) => "sxtl_sshll_advsimd", "sxtl2 v6.2d, v21.4s");
    check_words!(cases, v_xtl(true, 0, false, 7, 22) => "uxtl_ushll_advsimd", "uxtl v7.8h, v22.8b");
    check_words!(cases, v_xtl(true, 0, true, 7, 22) => "uxtl_ushll_advsimd", "uxtl2 v7.8h, v22.16b");
    check_words!(cases, v_xtl(true, 1, false, 8, 23) => "uxtl_ushll_advsimd", "uxtl v8.4s, v23.4h");
    check_words!(cases, v_xtl(true, 1, true, 8, 23) => "uxtl_ushll_advsimd", "uxtl2 v8.4s, v23.8h");
    check_words!(cases, v_xtl(true, 2, false, 9, 24) => "uxtl_ushll_advsimd", "uxtl v9.2d, v24.2s");
    check_words!(cases, v_xtl(true, 2, true, 9, 24) => "uxtl_ushll_advsimd", "uxtl2 v9.2d, v24.4s");

    check_words!(cases, v_fcvtl(false, 5, 21) => "fcvtl_advsimd", "fcvtl v5.2d, v21.2s");
    check_words!(cases, v_fcvtl(true, 6, 22) => "fcvtl_advsimd", "fcvtl2 v6.2d, v22.4s");

    check_words!(cases, v_fcvtn(false, 7, 23) => "fcvtn_advsimd", "fcvtn v7.2s, v23.2d");
    check_words!(cases, v_fcvtn(true, 8, 24) => "fcvtn_advsimd", "fcvtn2 v8.4s, v24.2d");

    check_words!(cases, v_scvtf_2d(9, 24) => "scvtf_advsimd_int", "scvtf v9.2d, v24.2d");
    check_words!(cases, v_fcvtzs_2d(10, 25) => "fcvtzs_advsimd_int", "fcvtzs v10.2d, v25.2d");

    check_words!(cases, v_xtn_2s(false, 11, 26) => "xtn_advsimd", "xtn v11.2s, v26.2d");
    check_words!(cases, v_xtn_2s(true, 12, 27) => "xtn_advsimd", "xtn2 v12.4s, v27.2d");

    check_words!(cases, s_fneg(false, 4, 19) => "fneg_float", "fneg s4, s19");
    check_words!(cases, s_fneg(true, 5, 20) => "fneg_float", "fneg d5, d20");

    check_words!(cases, s_fmadd(false, 3, 11, 19, 27) => "fmadd_float", "fmadd s3, s11, s19, s27");
    check_words!(cases, s_fmadd(true, 4, 12, 20, 28) => "fmadd_float", "fmadd d4, d12, d20, d28");

    check_words!(cases, v_fneg(false, 6, 21) => "fneg_advsimd", "fneg v6.4s, v21.4s");
    check_words!(cases, v_fneg(true, 7, 22) => "fneg_advsimd", "fneg v7.2d, v22.2d");

    check_words!(cases, v_fmla(false, 5, 13, 23) => "fmla_advsimd_vec", "fmla v5.4s, v13.4s, v23.4s");
    check_words!(cases, v_fmla(true, 6, 14, 24) => "fmla_advsimd_vec", "fmla v6.2d, v14.2d, v24.2d");

    check_words!(cases, bics_rr(false, false, 4, 11, 18) => "bic_log_shift", "bic w4, w11, w18");
    check_words!(cases, bics_rr(true, false, 5, 12, 19) => "bic_log_shift", "bic x5, x12, x19");
    check_words!(cases, bics_rr(false, true, 6, 13, 20) => "bics", "bics w6, w13, w20");
    check_words!(cases, bics_rr(true, true, 7, 14, 21) => "bics", "bics x7, x14, x21");

    check_words!(cases, crc32c(8, 4, 11, 18) => "crc32c", "crc32cb w4, w11, w18");
    check_words!(cases, crc32c(16, 5, 12, 19) => "crc32c", "crc32ch w5, w12, w19");
    check_words!(cases, crc32c(32, 6, 13, 20) => "crc32c", "crc32cw w6, w13, w20");
    check_words!(cases, crc32c(64, 7, 14, 21) => "crc32c", "crc32cx w7, w14, x21");

    check_words!(cases, casa(8, 3, 5, 19) => "casb", "casab w3, w5, [x19]");
    check_words!(cases, casa(16, 4, 6, 20) => "cash", "casah w4, w6, [x20]");
    check_words!(cases, casa(32, 2, 31, 17) => "cas", "casa w2, wzr, [x17]");
    check_words!(cases, casa(64, 6, 8, 22) => "cas", "casa x6, x8, [x22]");

    check_words!(cases, casl(8, 3, 5, 19) => "casb", "caslb w3, w5, [x19]");
    check_words!(cases, casl(16, 4, 6, 20) => "cash", "caslh w4, w6, [x20]");
    check_words!(cases, casl(32, 2, 0, 17) => "cas", "casl w2, w0, [x17]");
    check_words!(cases, casl(64, 6, 8, 22) => "cas", "casl x6, x8, [x22]");

    check_words!(cases, ldar(8, 2, 17) => "ldarb", "ldarb w2, [x17]");
    check_words!(cases, ldar(16, 2, 17) => "ldarh", "ldarh w2, [x17]");
    check_words!(cases, ldar(32, 2, 17) => "ldar", "ldar w2, [x17]");
    check_words!(cases, ldar(64, 2, 17) => "ldar", "ldar x2, [x17]");
}
