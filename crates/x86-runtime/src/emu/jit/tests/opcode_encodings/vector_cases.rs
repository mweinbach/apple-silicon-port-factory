//! Encoding verification cases for every `Asm` method in `vec.rs`
//! (both `impl Asm` blocks, including `flush_flags_and_shift_vec`).

use super::Cases;

pub(super) fn register(cases: &mut Cases) {
    register_scalar_and_misc(cases);
    register_neon_3same_and_logic(cases);
    register_lane_permute_and_widening(cases);
    register_fp_and_crypto(cases);
    register_multiword_helpers(cases);
}

fn register_scalar_and_misc(cases: &mut Cases) {
    for &(x, s, d, n, m, stem, asm) in &[
        (false, false, 27, 19, 28, "adc", "adc w27, w19, w28"),
        (true, false, 27, 19, 28, "adc", "adc x27, x19, x28"),
        (false, true, 27, 19, 28, "adcs", "adcs w27, w19, w28"),
        (true, true, 27, 19, 28, "adcs", "adcs x27, x19, x28"),
        (true, true, 31, 21, 29, "adcs", "adcs xzr, x21, x29"),
    ] {
        cases.check("adcs", &[(stem, asm)], move |a| a.adcs(x, s, d, n, m));
    }

    for &(x, s, d, n, m, stem, asm) in &[
        (false, false, 27, 19, 28, "sbc", "sbc w27, w19, w28"),
        (true, false, 27, 19, 28, "sbc", "sbc x27, x19, x28"),
        (false, true, 27, 19, 28, "sbcs", "sbcs w27, w19, w28"),
        (true, true, 27, 19, 28, "sbcs", "sbcs x27, x19, x28"),
        (false, false, 25, 31, 29, "ngc_sbc", "ngc w25, w29"),
        (true, false, 25, 31, 29, "ngc_sbc", "ngc x25, x29"),
        (false, true, 25, 31, 29, "ngcs_sbcs", "ngcs w25, w29"),
        (true, true, 25, 31, 29, "ngcs_sbcs", "ngcs x25, x29"),
    ] {
        cases.check("sbcs", &[(stem, asm)], move |a| a.sbcs(x, s, d, n, m));
    }

    for &(x, asm) in &[(false, "bic w27, w19, w28"), (true, "bic x27, x19, x28")] {
        cases.check("bic_rr", &[("bic_log_shift", asm)], move |a| {
            a.bic_rr(x, 27, 19, 28);
        });
    }

    for &(x, d, n, m, stem, asm) in &[
        (false, 27, 19, 28, "rorv", "rorv w27, w19, w28"),
        (true, 27, 19, 28, "rorv", "rorv x27, x19, x28"),
        (false, 25, 18, 29, "ror_rorv", "ror w25, w18, w29"),
        (true, 25, 18, 29, "ror_rorv", "ror x25, x18, x29"),
    ] {
        cases.check("rorv", &[(stem, asm)], move |a| a.rorv(x, d, n, m));
    }

    cases.check("v_cnt_8b", &[("cnt_advsimd", "cnt v29.8b, v17.8b")], |a| {
        a.v_cnt_8b(29, 17);
    });
    cases.check("v_addv_8b", &[("addv_advsimd", "addv b26, v19.8b")], |a| {
        a.v_addv_8b(26, 19);
    });
    cases.check("v_addv_4s", &[("addv_advsimd", "addv s24, v18.4s")], |a| {
        a.v_addv_4s(24, 18);
    });
    cases.check("rev16_w", &[("rev16_int", "rev16 w25, w19")], |a| {
        a.rev16_w(25, 19);
    });

    for &(size, asm) in &[
        (0, "abs v27.16b, v19.16b"),
        (1, "abs v27.8h, v19.8h"),
        (2, "abs v27.4s, v19.4s"),
        (3, "abs v27.2d, v19.2d"),
    ] {
        cases.check("v_abs", &[("abs_advsimd", asm)], move |a| {
            a.v_abs(size, 27, 19);
        });
    }

    for &(size, asm) in &[
        (0, "neg v28.16b, v20.16b"),
        (1, "neg v28.8h, v20.8h"),
        (2, "neg v28.4s, v20.4s"),
        (3, "neg v28.2d, v20.2d"),
    ] {
        cases.check("v_neg", &[("neg_advsimd", asm)], move |a| {
            a.v_neg(size, 28, 20);
        });
    }

    for &(size, asm) in &[
        (0, "cmlt v25.16b, v17.16b, #0"),
        (1, "cmlt v25.8h, v17.8h, #0"),
        (2, "cmlt v25.4s, v17.4s, #0"),
        (3, "cmlt v25.2d, v17.2d, #0"),
    ] {
        cases.check("v_cmlt0", &[("cmlt_advsimd", asm)], move |a| {
            a.v_cmlt0(size, 25, 17);
        });
    }

    for &(size, asm) in &[
        (0, "cmeq v26.16b, v18.16b, #0"),
        (1, "cmeq v26.8h, v18.8h, #0"),
        (2, "cmeq v26.4s, v18.4s, #0"),
        (3, "cmeq v26.2d, v18.2d, #0"),
    ] {
        cases.check("v_cmeq0", &[("cmeq_advsimd_zero", asm)], move |a| {
            a.v_cmeq0(size, 26, 18);
        });
    }

    for &(lg, vd, lane, rn, stem, asm) in &[
        (0, 27, 13, 19, "ins_advsimd_gen", "ins v27.b[13], w19"),
        (1, 27, 5, 19, "ins_advsimd_gen", "ins v27.h[5], w19"),
        (2, 27, 3, 19, "ins_advsimd_gen", "ins v27.s[3], w19"),
        (3, 27, 1, 19, "ins_advsimd_gen", "ins v27.d[1], x19"),
        (0, 28, 0, 21, "mov_ins_advsimd_gen", "mov v28.b[0], w21"),
        (3, 28, 1, 21, "mov_ins_advsimd_gen", "mov v28.d[1], x21"),
    ] {
        cases.check("v_ins_elem", &[(stem, asm)], move |a| {
            a.v_ins_elem(lg, vd, lane, rn);
        });
    }

    for &(lg, rd, vn, lane, stem, asm) in &[
        (0, 25, 19, 13, "umov_advsimd", "umov w25, v19.b[13]"),
        (1, 25, 19, 5, "umov_advsimd", "umov w25, v19.h[5]"),
        (2, 25, 19, 3, "umov_advsimd", "umov w25, v19.s[3]"),
        (3, 25, 19, 1, "umov_advsimd", "umov x25, v19.d[1]"),
        (2, 26, 20, 2, "mov_umov_advsimd", "mov w26, v20.s[2]"),
        (3, 26, 20, 0, "mov_umov_advsimd", "mov x26, v20.d[0]"),
    ] {
        cases.check("v_umov_elem", &[(stem, asm)], move |a| {
            a.v_umov_elem(lg, rd, vn, lane);
        });
    }
}

fn register_neon_3same_and_logic(cases: &mut Cases) {
    // opcode 1: sqadd / uqadd
    check_words!(cases, neon_3same(false, 0, 1, 27, 19, 22) => "sqadd_advsimd", "sqadd v27.16b, v19.16b, v22.16b");
    check_words!(cases, neon_3same(false, 1, 1, 27, 19, 22) => "sqadd_advsimd", "sqadd v27.8h, v19.8h, v22.8h");
    check_words!(cases, neon_3same(false, 2, 1, 27, 19, 22) => "sqadd_advsimd", "sqadd v27.4s, v19.4s, v22.4s");
    check_words!(cases, neon_3same(false, 3, 1, 27, 19, 22) => "sqadd_advsimd", "sqadd v27.2d, v19.2d, v22.2d");
    check_words!(cases, neon_3same(true, 0, 1, 27, 19, 22) => "uqadd_advsimd", "uqadd v27.16b, v19.16b, v22.16b");
    check_words!(cases, neon_3same(true, 1, 1, 27, 19, 22) => "uqadd_advsimd", "uqadd v27.8h, v19.8h, v22.8h");
    check_words!(cases, neon_3same(true, 2, 1, 27, 19, 22) => "uqadd_advsimd", "uqadd v27.4s, v19.4s, v22.4s");
    check_words!(cases, neon_3same(true, 3, 1, 27, 19, 22) => "uqadd_advsimd", "uqadd v27.2d, v19.2d, v22.2d");
    // opcode 2: urhadd
    check_words!(cases, neon_3same(true, 0, 2, 27, 19, 22) => "urhadd_advsimd", "urhadd v27.16b, v19.16b, v22.16b");
    check_words!(cases, neon_3same(true, 1, 2, 27, 19, 22) => "urhadd_advsimd", "urhadd v27.8h, v19.8h, v22.8h");
    check_words!(cases, neon_3same(true, 2, 2, 27, 19, 22) => "urhadd_advsimd", "urhadd v27.4s, v19.4s, v22.4s");
    // opcode 3: bitwise logic (all 8 (u, size) encodings)
    check_words!(cases, neon_3same(false, 0, 3, 27, 19, 22) => "and_advsimd", "and v27.16b, v19.16b, v22.16b");
    check_words!(cases, neon_3same(false, 1, 3, 27, 19, 22) => "bic_advsimd_reg", "bic v27.16b, v19.16b, v22.16b");
    check_words!(cases, neon_3same(false, 2, 3, 27, 19, 22) => "orr_advsimd_reg", "orr v27.16b, v19.16b, v22.16b");
    check_words!(cases, neon_3same(false, 3, 3, 27, 19, 22) => "orn_advsimd", "orn v27.16b, v19.16b, v22.16b");
    check_words!(cases, neon_3same(true, 0, 3, 27, 19, 22) => "eor_advsimd", "eor v27.16b, v19.16b, v22.16b");
    check_words!(cases, neon_3same(true, 1, 3, 27, 19, 22) => "bsl_advsimd", "bsl v27.16b, v19.16b, v22.16b");
    check_words!(cases, neon_3same(true, 2, 3, 27, 19, 22) => "bit_advsimd", "bit v27.16b, v19.16b, v22.16b");
    check_words!(cases, neon_3same(true, 3, 3, 27, 19, 22) => "bif_advsimd", "bif v27.16b, v19.16b, v22.16b");
    // opcode 5: sqsub / uqsub
    check_words!(cases, neon_3same(false, 0, 5, 27, 19, 22) => "sqsub_advsimd", "sqsub v27.16b, v19.16b, v22.16b");
    check_words!(cases, neon_3same(false, 1, 5, 27, 19, 22) => "sqsub_advsimd", "sqsub v27.8h, v19.8h, v22.8h");
    check_words!(cases, neon_3same(false, 3, 5, 27, 19, 22) => "sqsub_advsimd", "sqsub v27.2d, v19.2d, v22.2d");
    check_words!(cases, neon_3same(true, 0, 5, 27, 19, 22) => "uqsub_advsimd", "uqsub v27.16b, v19.16b, v22.16b");
    check_words!(cases, neon_3same(true, 1, 5, 27, 19, 22) => "uqsub_advsimd", "uqsub v27.8h, v19.8h, v22.8h");
    check_words!(cases, neon_3same(true, 3, 5, 27, 19, 22) => "uqsub_advsimd", "uqsub v27.2d, v19.2d, v22.2d");
    // opcode 6: cmgt
    check_words!(cases, neon_3same(false, 0, 6, 27, 19, 22) => "cmgt_advsimd_reg", "cmgt v27.16b, v19.16b, v22.16b");
    check_words!(cases, neon_3same(false, 1, 6, 27, 19, 22) => "cmgt_advsimd_reg", "cmgt v27.8h, v19.8h, v22.8h");
    check_words!(cases, neon_3same(false, 2, 6, 27, 19, 22) => "cmgt_advsimd_reg", "cmgt v27.4s, v19.4s, v22.4s");
    check_words!(cases, neon_3same(false, 3, 6, 27, 19, 22) => "cmgt_advsimd_reg", "cmgt v27.2d, v19.2d, v22.2d");
    // opcode 8: sshl / ushl
    check_words!(cases, neon_3same(false, 0, 8, 27, 19, 22) => "sshl_advsimd", "sshl v27.16b, v19.16b, v22.16b");
    check_words!(cases, neon_3same(false, 1, 8, 27, 19, 22) => "sshl_advsimd", "sshl v27.8h, v19.8h, v22.8h");
    check_words!(cases, neon_3same(false, 2, 8, 27, 19, 22) => "sshl_advsimd", "sshl v27.4s, v19.4s, v22.4s");
    check_words!(cases, neon_3same(false, 3, 8, 27, 19, 22) => "sshl_advsimd", "sshl v27.2d, v19.2d, v22.2d");
    check_words!(cases, neon_3same(true, 0, 8, 27, 19, 22) => "ushl_advsimd", "ushl v27.16b, v19.16b, v22.16b");
    check_words!(cases, neon_3same(true, 1, 8, 27, 19, 22) => "ushl_advsimd", "ushl v27.8h, v19.8h, v22.8h");
    check_words!(cases, neon_3same(true, 2, 8, 27, 19, 22) => "ushl_advsimd", "ushl v27.4s, v19.4s, v22.4s");
    check_words!(cases, neon_3same(true, 3, 8, 27, 19, 22) => "ushl_advsimd", "ushl v27.2d, v19.2d, v22.2d");
    // opcode 12: smax / umax
    check_words!(cases, neon_3same(false, 0, 12, 27, 19, 22) => "smax_advsimd", "smax v27.16b, v19.16b, v22.16b");
    check_words!(cases, neon_3same(false, 1, 12, 27, 19, 22) => "smax_advsimd", "smax v27.8h, v19.8h, v22.8h");
    check_words!(cases, neon_3same(false, 2, 12, 27, 19, 22) => "smax_advsimd", "smax v27.4s, v19.4s, v22.4s");
    check_words!(cases, neon_3same(true, 0, 12, 27, 19, 22) => "umax_advsimd", "umax v27.16b, v19.16b, v22.16b");
    check_words!(cases, neon_3same(true, 1, 12, 27, 19, 22) => "umax_advsimd", "umax v27.8h, v19.8h, v22.8h");
    check_words!(cases, neon_3same(true, 2, 12, 27, 19, 22) => "umax_advsimd", "umax v27.4s, v19.4s, v22.4s");
    // opcode 13: smin / umin
    check_words!(cases, neon_3same(false, 0, 13, 27, 19, 22) => "smin_advsimd", "smin v27.16b, v19.16b, v22.16b");
    check_words!(cases, neon_3same(false, 1, 13, 27, 19, 22) => "smin_advsimd", "smin v27.8h, v19.8h, v22.8h");
    check_words!(cases, neon_3same(false, 2, 13, 27, 19, 22) => "smin_advsimd", "smin v27.4s, v19.4s, v22.4s");
    check_words!(cases, neon_3same(true, 0, 13, 27, 19, 22) => "umin_advsimd", "umin v27.16b, v19.16b, v22.16b");
    check_words!(cases, neon_3same(true, 1, 13, 27, 19, 22) => "umin_advsimd", "umin v27.8h, v19.8h, v22.8h");
    check_words!(cases, neon_3same(true, 2, 13, 27, 19, 22) => "umin_advsimd", "umin v27.4s, v19.4s, v22.4s");
    // opcode 16: add / sub
    check_words!(cases, neon_3same(false, 0, 16, 27, 19, 22) => "add_advsimd", "add v27.16b, v19.16b, v22.16b");
    check_words!(cases, neon_3same(false, 1, 16, 27, 19, 22) => "add_advsimd", "add v27.8h, v19.8h, v22.8h");
    check_words!(cases, neon_3same(false, 2, 16, 27, 19, 22) => "add_advsimd", "add v27.4s, v19.4s, v22.4s");
    check_words!(cases, neon_3same(false, 3, 16, 27, 19, 22) => "add_advsimd", "add v27.2d, v19.2d, v22.2d");
    check_words!(cases, neon_3same(true, 0, 16, 27, 19, 22) => "sub_advsimd", "sub v27.16b, v19.16b, v22.16b");
    check_words!(cases, neon_3same(true, 1, 16, 27, 19, 22) => "sub_advsimd", "sub v27.8h, v19.8h, v22.8h");
    check_words!(cases, neon_3same(true, 2, 16, 27, 19, 22) => "sub_advsimd", "sub v27.4s, v19.4s, v22.4s");
    check_words!(cases, neon_3same(true, 3, 16, 27, 19, 22) => "sub_advsimd", "sub v27.2d, v19.2d, v22.2d");
    // opcode 17: cmeq
    check_words!(cases, neon_3same(true, 0, 17, 27, 19, 22) => "cmeq_advsimd_reg", "cmeq v27.16b, v19.16b, v22.16b");
    check_words!(cases, neon_3same(true, 1, 17, 27, 19, 22) => "cmeq_advsimd_reg", "cmeq v27.8h, v19.8h, v22.8h");
    check_words!(cases, neon_3same(true, 2, 17, 27, 19, 22) => "cmeq_advsimd_reg", "cmeq v27.4s, v19.4s, v22.4s");
    check_words!(cases, neon_3same(true, 3, 17, 27, 19, 22) => "cmeq_advsimd_reg", "cmeq v27.2d, v19.2d, v22.2d");
    // opcode 19: mul
    check_words!(cases, neon_3same(false, 0, 19, 27, 19, 22) => "mul_advsimd_vec", "mul v27.16b, v19.16b, v22.16b");
    check_words!(cases, neon_3same(false, 1, 19, 27, 19, 22) => "mul_advsimd_vec", "mul v27.8h, v19.8h, v22.8h");
    check_words!(cases, neon_3same(false, 2, 19, 27, 19, 22) => "mul_advsimd_vec", "mul v27.4s, v19.4s, v22.4s");
    // opcode 23: addp
    check_words!(cases, neon_3same(false, 0, 23, 27, 19, 22) => "addp_advsimd_vec", "addp v27.16b, v19.16b, v22.16b");
    check_words!(cases, neon_3same(false, 1, 23, 27, 19, 22) => "addp_advsimd_vec", "addp v27.8h, v19.8h, v22.8h");
    check_words!(cases, neon_3same(false, 2, 23, 27, 19, 22) => "addp_advsimd_vec", "addp v27.4s, v19.4s, v22.4s");
    check_words!(cases, neon_3same(false, 3, 23, 27, 19, 22) => "addp_advsimd_vec", "addp v27.2d, v19.2d, v22.2d");

    cases.check(
        "neon_3same",
        &[("mov_orr_advsimd_reg", "mov v27.16b, v19.16b")],
        |a| {
            a.neon_3same(false, 2, 3, 27, 19, 19);
        },
    );

    cases.check(
        "v_and",
        &[("and_advsimd", "and v27.16b, v19.16b, v22.16b")],
        |a| {
            a.v_and(27, 19, 22);
        },
    );
    cases.check(
        "v_bic",
        &[("bic_advsimd_reg", "bic v27.16b, v19.16b, v22.16b")],
        |a| {
            a.v_bic(27, 19, 22);
        },
    );
    cases.check(
        "v_orr",
        &[("orr_advsimd_reg", "orr v27.16b, v19.16b, v22.16b")],
        |a| {
            a.v_orr(27, 19, 22);
        },
    );
    cases.check(
        "v_orr",
        &[("mov_orr_advsimd_reg", "mov v27.16b, v19.16b")],
        |a| {
            a.v_orr(27, 19, 19);
        },
    );
    cases.check(
        "v_eor",
        &[("eor_advsimd", "eor v27.16b, v19.16b, v22.16b")],
        |a| {
            a.v_eor(27, 19, 22);
        },
    );
    cases.check(
        "v_bsl",
        &[("bsl_advsimd", "bsl v27.16b, v19.16b, v22.16b")],
        |a| {
            a.v_bsl(27, 19, 22);
        },
    );

    cases.check(
        "v_mov",
        &[("mov_orr_advsimd_reg", "mov v27.16b, v19.16b")],
        |a| {
            a.v_mov(27, 19);
        },
    );
    // Self-move must be a no-op; emit a following move so the assertion verifies 0 words for the first call.
    cases.check(
        "v_mov",
        &[("mov_orr_advsimd_reg", "mov v29.16b, v17.16b")],
        |a| {
            a.v_mov(19, 19);
            a.v_mov(29, 17);
        },
    );
    cases.check(
        "v_zero",
        &[("eor_advsimd", "eor v29.16b, v29.16b, v29.16b")],
        |a| {
            a.v_zero(29);
        },
    );
}

fn register_lane_permute_and_widening(cases: &mut Cases) {
    for &(idx, stem, asm) in &[
        (0, "ins_advsimd_gen", "ins v27.d[0], x19"),
        (1, "mov_ins_advsimd_gen", "mov v27.d[1], x19"),
    ] {
        cases.check("v_ins_d", &[(stem, asm)], move |a| a.v_ins_d(27, idx, 19));
    }

    for &(idx, stem, asm) in &[
        (0, "ins_advsimd_gen", "ins v27.s[0], w19"),
        (1, "ins_advsimd_gen", "ins v27.s[1], w19"),
        (2, "ins_advsimd_gen", "ins v27.s[2], w19"),
        (3, "ins_advsimd_gen", "ins v27.s[3], w19"),
        (3, "mov_ins_advsimd_gen", "mov v27.s[3], w19"),
    ] {
        cases.check("v_ins_s", &[(stem, asm)], move |a| a.v_ins_s(27, idx, 19));
    }

    for &(idx, stem, asm) in &[
        (0, "umov_advsimd", "umov x25, v19.d[0]"),
        (1, "mov_umov_advsimd", "mov x25, v19.d[1]"),
    ] {
        cases.check("v_umov_d", &[(stem, asm)], move |a| a.v_umov_d(25, 19, idx));
    }

    for &(size, asm) in &[
        (0, "dup v27.16b, w21"),
        (1, "dup v27.8h, w21"),
        (2, "dup v27.4s, w21"),
        (3, "dup v27.2d, x21"),
    ] {
        cases.check("v_dup", &[("dup_advsimd_gen", asm)], move |a| {
            a.v_dup(size, 27, 21);
        });
    }

    for &(lane, asm) in &[
        (0, "dup v28.4s, v19.s[0]"),
        (1, "dup v28.4s, v19.s[1]"),
        (2, "dup v28.4s, v19.s[2]"),
        (3, "dup v28.4s, v19.s[3]"),
    ] {
        cases.check("v_dup_lane_s", &[("dup_advsimd_elt", asm)], move |a| {
            a.v_dup_lane_s(28, 19, lane);
        });
    }

    for &(size, asm) in &[
        (0, "zip1 v27.16b, v19.16b, v22.16b"),
        (1, "zip1 v27.8h, v19.8h, v22.8h"),
        (2, "zip1 v27.4s, v19.4s, v22.4s"),
        (3, "zip1 v27.2d, v19.2d, v22.2d"),
    ] {
        cases.check("v_zip1", &[("zip1_advsimd", asm)], move |a| {
            a.v_zip1(size, 27, 19, 22);
        });
    }

    for &(size, asm) in &[
        (0, "zip2 v27.16b, v19.16b, v22.16b"),
        (1, "zip2 v27.8h, v19.8h, v22.8h"),
        (2, "zip2 v27.4s, v19.4s, v22.4s"),
        (3, "zip2 v27.2d, v19.2d, v22.2d"),
    ] {
        cases.check("v_zip2", &[("zip2_advsimd", asm)], move |a| {
            a.v_zip2(size, 27, 19, 22);
        });
    }

    for &(size, asm) in &[
        (0, "uzp1 v27.16b, v19.16b, v22.16b"),
        (1, "uzp1 v27.8h, v19.8h, v22.8h"),
        (2, "uzp1 v27.4s, v19.4s, v22.4s"),
        (3, "uzp1 v27.2d, v19.2d, v22.2d"),
    ] {
        cases.check("v_uzp1", &[("uzp1_advsimd", asm)], move |a| {
            a.v_uzp1(size, 27, 19, 22);
        });
    }

    for &(size, asm) in &[
        (0, "uzp2 v27.16b, v19.16b, v22.16b"),
        (1, "uzp2 v27.8h, v19.8h, v22.8h"),
        (2, "uzp2 v27.4s, v19.4s, v22.4s"),
        (3, "uzp2 v27.2d, v19.2d, v22.2d"),
    ] {
        cases.check("v_uzp2", &[("uzp2_advsimd", asm)], move |a| {
            a.v_uzp2(size, 27, 19, 22);
        });
    }

    cases.check(
        "v_tbl1",
        &[("tbl_advsimd", "tbl v27.16b, {v19.16b}, v22.16b")],
        |a| {
            a.v_tbl1(27, 19, 22);
        },
    );
    for &(vn_pair, asm) in &[
        (20, "tbl v27.16b, {v20.16b, v21.16b}, v22.16b"),
        (31, "tbl v27.16b, {v31.16b, v0.16b}, v22.16b"),
    ] {
        cases.check("v_tbl2", &[("tbl_advsimd", asm)], move |a| {
            a.v_tbl2(27, vn_pair, 22);
        });
    }

    for &(imm4, asm) in &[
        (1, "ext v27.16b, v19.16b, v22.16b, #1"),
        (8, "ext v27.16b, v19.16b, v22.16b, #8"),
        (15, "ext v27.16b, v19.16b, v22.16b, #15"),
    ] {
        cases.check("v_ext", &[("ext_advsimd", asm)], move |a| {
            a.v_ext(27, 19, 22, imm4);
        });
    }

    for &(q, size, asm) in &[
        (false, 0, "sqxtn v27.8b, v19.8h"),
        (true, 0, "sqxtn2 v27.16b, v19.8h"),
        (false, 1, "sqxtn v27.4h, v19.4s"),
        (true, 1, "sqxtn2 v27.8h, v19.4s"),
        (false, 2, "sqxtn v27.2s, v19.2d"),
        (true, 2, "sqxtn2 v27.4s, v19.2d"),
    ] {
        cases.check("v_sqxtn", &[("sqxtn_advsimd", asm)], move |a| {
            a.v_sqxtn(q, size, 27, 19);
        });
    }

    for &(q, size, asm) in &[
        (false, 0, "sqxtun v27.8b, v19.8h"),
        (true, 0, "sqxtun2 v27.16b, v19.8h"),
        (false, 1, "sqxtun v27.4h, v19.4s"),
        (true, 1, "sqxtun2 v27.8h, v19.4s"),
        (false, 2, "sqxtun v27.2s, v19.2d"),
        (true, 2, "sqxtun2 v27.4s, v19.2d"),
    ] {
        cases.check("v_sqxtun", &[("sqxtun_advsimd", asm)], move |a| {
            a.v_sqxtun(q, size, 27, 19);
        });
    }

    for &(ebits, shift, asm) in &[
        (8, 5, "shl v27.16b, v19.16b, #5"),
        (16, 11, "shl v27.8h, v19.8h, #11"),
        (32, 19, "shl v27.4s, v19.4s, #19"),
        (64, 37, "shl v27.2d, v19.2d, #37"),
    ] {
        cases.check("v_shl_i", &[("shl_advsimd", asm)], move |a| {
            a.v_shl_i(ebits, 27, 19, shift);
        });
    }

    for &(u, ebits, shift, stem, asm) in &[
        (false, 8, 5, "sshr_advsimd", "sshr v27.16b, v19.16b, #5"),
        (false, 16, 11, "sshr_advsimd", "sshr v27.8h, v19.8h, #11"),
        (false, 32, 19, "sshr_advsimd", "sshr v27.4s, v19.4s, #19"),
        (false, 64, 37, "sshr_advsimd", "sshr v27.2d, v19.2d, #37"),
        (true, 8, 8, "ushr_advsimd", "ushr v27.16b, v19.16b, #8"),
        (true, 16, 11, "ushr_advsimd", "ushr v27.8h, v19.8h, #11"),
        (true, 32, 19, "ushr_advsimd", "ushr v27.4s, v19.4s, #19"),
        (true, 64, 64, "ushr_advsimd", "ushr v27.2d, v19.2d, #64"),
    ] {
        cases.check("v_shr_i", &[(stem, asm)], move |a| {
            a.v_shr_i(u, ebits, 27, 19, shift);
        });
    }

    for &(q, size, asm) in &[
        (false, 0, "smull v27.8h, v19.8b, v22.8b"),
        (true, 0, "smull2 v27.8h, v19.16b, v22.16b"),
        (false, 1, "smull v27.4s, v19.4h, v22.4h"),
        (true, 1, "smull2 v27.4s, v19.8h, v22.8h"),
        (false, 2, "smull v27.2d, v19.2s, v22.2s"),
        (true, 2, "smull2 v27.2d, v19.4s, v22.4s"),
    ] {
        cases.check("v_smull", &[("smull_advsimd_vec", asm)], move |a| {
            a.v_smull(q, size, 27, 19, 22);
        });
    }

    for &(q, size, asm) in &[
        (false, 0, "umull v27.8h, v19.8b, v22.8b"),
        (true, 0, "umull2 v27.8h, v19.16b, v22.16b"),
        (false, 1, "umull v27.4s, v19.4h, v22.4h"),
        (true, 1, "umull2 v27.4s, v19.8h, v22.8h"),
        (false, 2, "umull v27.2d, v19.2s, v22.2s"),
        (true, 2, "umull2 v27.2d, v19.4s, v22.4s"),
    ] {
        cases.check("v_umull", &[("umull_advsimd_vec", asm)], move |a| {
            a.v_umull(q, size, 27, 19, 22);
        });
    }

    for &(q, asm) in &[
        (false, "rshrn v27.4h, v19.4s, #15"),
        (true, "rshrn2 v27.8h, v19.4s, #15"),
    ] {
        cases.check("v_rshrn15_h", &[("rshrn_advsimd", asm)], move |a| {
            a.v_rshrn15_h(q, 27, 19);
        });
    }

    for &(q, vd, vn, stem, asm) in &[
        (false, 27, 19, "sxtl_sshll_advsimd", "sxtl v27.8h, v19.8b"),
        (true, 27, 19, "sxtl_sshll_advsimd", "sxtl2 v27.8h, v19.16b"),
        (false, 28, 20, "sshll_advsimd", "sshll v28.8h, v20.8b, #0"),
    ] {
        cases.check("v_sxtl_b", &[(stem, asm)], move |a| a.v_sxtl_b(q, vd, vn));
    }

    for &(q, vd, vn, stem, asm) in &[
        (false, 27, 19, "uxtl_ushll_advsimd", "uxtl v27.8h, v19.8b"),
        (true, 27, 19, "uxtl_ushll_advsimd", "uxtl2 v27.8h, v19.16b"),
        (false, 28, 20, "ushll_advsimd", "ushll v28.8h, v20.8b, #0"),
    ] {
        cases.check("v_uxtl_b", &[(stem, asm)], move |a| a.v_uxtl_b(q, vd, vn));
    }
}

fn register_fp_and_crypto(cases: &mut Cases) {
    for &(is_f64, op, stem, asm) in &[
        (false, 0, "fadd_float", "fadd s27, s19, s22"),
        (false, 1, "fsub_float", "fsub s27, s19, s22"),
        (false, 2, "fmul_float", "fmul s27, s19, s22"),
        (false, 3, "fdiv_float", "fdiv s27, s19, s22"),
        (true, 0, "fadd_float", "fadd d27, d19, d22"),
        (true, 1, "fsub_float", "fsub d27, d19, d22"),
        (true, 2, "fmul_float", "fmul d27, d19, d22"),
        (true, 3, "fdiv_float", "fdiv d27, d19, d22"),
    ] {
        cases.check("s_farith", &[(stem, asm)], move |a| {
            a.s_farith(is_f64, op, 27, 19, 22);
        });
    }

    for &(is_f64, asm) in &[(false, "fsqrt s27, s19"), (true, "fsqrt d27, d19")] {
        cases.check("s_fsqrt", &[("fsqrt_float", asm)], move |a| {
            a.s_fsqrt(is_f64, 27, 19);
        });
    }

    for &(is_f64, cond, asm) in &[
        (false, 4, "fcsel s27, s19, s22, mi"),
        (false, 12, "fcsel s27, s19, s22, gt"),
        (true, 13, "fcsel d27, d19, d22, le"),
    ] {
        cases.check("s_fcsel", &[("fcsel_float", asm)], move |a| {
            a.s_fcsel(is_f64, 27, 19, 22, cond);
        });
    }

    for &(is_f64, op, stem, asm) in &[
        (false, 0, "fadd_advsimd", "fadd v27.4s, v19.4s, v22.4s"),
        (false, 1, "fsub_advsimd", "fsub v27.4s, v19.4s, v22.4s"),
        (false, 2, "fmul_advsimd_vec", "fmul v27.4s, v19.4s, v22.4s"),
        (false, 3, "fdiv_advsimd", "fdiv v27.4s, v19.4s, v22.4s"),
        (true, 0, "fadd_advsimd", "fadd v27.2d, v19.2d, v22.2d"),
        (true, 1, "fsub_advsimd", "fsub v27.2d, v19.2d, v22.2d"),
        (true, 2, "fmul_advsimd_vec", "fmul v27.2d, v19.2d, v22.2d"),
        (true, 3, "fdiv_advsimd", "fdiv v27.2d, v19.2d, v22.2d"),
    ] {
        cases.check("v_farith", &[(stem, asm)], move |a| {
            a.v_farith(is_f64, op, 27, 19, 22);
        });
    }

    for &(is_f64, asm) in &[
        (false, "fcmeq v27.4s, v19.4s, v22.4s"),
        (true, "fcmeq v27.2d, v19.2d, v22.2d"),
    ] {
        cases.check("v_fcmeq", &[("fcmeq_advsimd_reg", asm)], move |a| {
            a.v_fcmeq(is_f64, 27, 19, 22);
        });
    }

    for &(is_f64, asm) in &[
        (false, "fcmge v27.4s, v19.4s, v22.4s"),
        (true, "fcmge v27.2d, v19.2d, v22.2d"),
    ] {
        cases.check("v_fcmge", &[("fcmge_advsimd_reg", asm)], move |a| {
            a.v_fcmge(is_f64, 27, 19, 22);
        });
    }

    for &(is_f64, asm) in &[
        (false, "fcmgt v27.4s, v19.4s, v22.4s"),
        (true, "fcmgt v27.2d, v19.2d, v22.2d"),
    ] {
        cases.check("v_fcmgt", &[("fcmgt_advsimd_reg", asm)], move |a| {
            a.v_fcmgt(is_f64, 27, 19, 22);
        });
    }

    for &(is_f64, asm) in &[
        (false, "faddp v27.4s, v19.4s, v22.4s"),
        (true, "faddp v27.2d, v19.2d, v22.2d"),
    ] {
        cases.check("v_faddp", &[("faddp_advsimd_vec", asm)], move |a| {
            a.v_faddp(is_f64, 27, 19, 22);
        });
    }

    for &(is_f64, asm) in &[
        (false, "fsqrt v27.4s, v19.4s"),
        (true, "fsqrt v27.2d, v19.2d"),
    ] {
        cases.check("v_fsqrt", &[("fsqrt_advsimd", asm)], move |a| {
            a.v_fsqrt(is_f64, 27, 19);
        });
    }

    cases.check("v_not", &[("not_advsimd", "not v27.16b, v19.16b")], |a| {
        a.v_not(27, 19);
    });
    cases.check(
        "v_not",
        &[("mvn_not_advsimd", "mvn v28.16b, v21.16b")],
        |a| {
            a.v_not(28, 21);
        },
    );

    for &(is_f64, rmode, vd, vn, stem, asm) in &[
        (false, 0, 27, 19, "frintn_float", "frintn s27, s19"),
        (false, 1, 27, 19, "frintm_float", "frintm s27, s19"),
        (false, 2, 27, 19, "frintp_float", "frintp s27, s19"),
        (false, 3, 27, 19, "frintz_float", "frintz s27, s19"),
        (true, 0, 27, 19, "frintn_float", "frintn d27, d19"),
        (true, 1, 27, 19, "frintm_float", "frintm d27, d19"),
        (true, 2, 27, 19, "frintp_float", "frintp d27, d19"),
        (true, 3, 27, 19, "frintz_float", "frintz d27, d19"),
        (true, 9, 28, 20, "frintm_float", "frintm d28, d20"),
    ] {
        cases.check("s_frint", &[(stem, asm)], move |a| {
            a.s_frint(is_f64, rmode, vd, vn);
        });
    }

    for &(is_f64, rmode, vd, vn, stem, asm) in &[
        (false, 0, 27, 19, "frintn_advsimd", "frintn v27.4s, v19.4s"),
        (false, 1, 27, 19, "frintm_advsimd", "frintm v27.4s, v19.4s"),
        (false, 2, 27, 19, "frintp_advsimd", "frintp v27.4s, v19.4s"),
        (false, 3, 27, 19, "frintz_advsimd", "frintz v27.4s, v19.4s"),
        (true, 0, 27, 19, "frintn_advsimd", "frintn v27.2d, v19.2d"),
        (true, 1, 27, 19, "frintm_advsimd", "frintm v27.2d, v19.2d"),
        (true, 2, 27, 19, "frintp_advsimd", "frintp v27.2d, v19.2d"),
        (true, 3, 27, 19, "frintz_advsimd", "frintz v27.2d, v19.2d"),
        (false, 10, 28, 20, "frintp_advsimd", "frintp v28.4s, v20.4s"),
    ] {
        cases.check("v_frint", &[(stem, asm)], move |a| {
            a.v_frint(is_f64, rmode, vd, vn);
        });
    }

    for &(q, asm) in &[
        (false, "uabdl v27.8h, v19.8b, v22.8b"),
        (true, "uabdl2 v27.8h, v19.16b, v22.16b"),
    ] {
        cases.check("v_uabdl_b", &[("uabdl_advsimd", asm)], move |a| {
            a.v_uabdl_b(q, 27, 19, 22);
        });
    }

    cases.check(
        "v_uaddlv_h",
        &[("uaddlv_advsimd", "uaddlv s27, v19.8h")],
        |a| {
            a.v_uaddlv_h(27, 19);
        },
    );
    cases.check(
        "v_pmull_1q",
        &[("pmull_advsimd", "pmull v27.1q, v19.1d, v22.1d")],
        |a| {
            a.v_pmull_1q(27, 19, 22);
        },
    );
    cases.check(
        "v_uminv_s",
        &[("uminv_advsimd", "uminv s26, v18.4s")],
        |a| {
            a.v_uminv_s(26, 18);
        },
    );
    cases.check(
        "v_scvtf_4s",
        &[("scvtf_advsimd_int", "scvtf v27.4s, v19.4s")],
        |a| {
            a.v_scvtf_4s(27, 19);
        },
    );
    cases.check(
        "v_frintn_4s",
        &[("frintn_advsimd", "frintn v27.4s, v19.4s")],
        |a| {
            a.v_frintn_4s(27, 19);
        },
    );
    cases.check(
        "v_fcvtzs_4s",
        &[("fcvtzs_advsimd_int", "fcvtzs v27.4s, v19.4s")],
        |a| {
            a.v_fcvtzs_4s(27, 19);
        },
    );
}

fn register_multiword_helpers(cases: &mut Cases) {
    cases.check("zero128", &[("stp_gen", "stp xzr, xzr, [x23]")], |a| {
        a.zero128(23, 0);
    });
    cases.check("zero128", &[("stp_gen", "stp xzr, xzr, [x23, #48]")], |a| {
        a.zero128(23, 48);
    });
    cases.check("zero128", &[("stp_gen", "stp xzr, xzr, [sp, #504]")], |a| {
        a.zero128(31, 504);
    });
    cases.check(
        "zero128",
        &[
            ("str_imm_gen", "str xzr, [x23, #512]"),
            ("str_imm_gen", "str xzr, [x23, #520]"),
        ],
        |a| a.zero128(23, 512),
    );

    // Arithmetic right shift (arith = true): 7 words per width.
    for &(size, ebits, lsr_asm, movz_asm, dup_asm, sshl_asm) in &[
        (
            1u32,
            16u64,
            "lsr x1, x0, #4",
            "movz x0, #15",
            "dup v2.8h, w0",
            "sshl v0.8h, v0.8h, v2.8h",
        ),
        (
            2,
            32,
            "lsr x1, x0, #5",
            "movz x0, #31",
            "dup v2.4s, w0",
            "sshl v0.4s, v0.4s, v2.4s",
        ),
    ] {
        cases.check(
            "flush_flags_and_shift_vec",
            &[
                ("fmov_float_gen", "fmov x0, d1"),
                ("lsr_ubfm", lsr_asm),
                ("cbz", "cbz x1, #8"),
                ("movz", movz_asm),
                ("neg_sub_addsub_shift", "neg w0, w0"),
                ("dup_advsimd_gen", dup_asm),
                ("sshl_advsimd", sshl_asm),
            ],
            move |a| a.flush_flags_and_shift_vec(size, ebits, false, true),
        );
    }

    // Logical right shift (arith = false, left = false): 8 words per width.
    for &(size, ebits, lsr_asm, dup_asm, ushl_asm) in &[
        (
            1u32,
            16u64,
            "lsr x1, x0, #4",
            "dup v2.8h, w0",
            "ushl v0.8h, v0.8h, v2.8h",
        ),
        (
            2,
            32,
            "lsr x1, x0, #5",
            "dup v2.4s, w0",
            "ushl v0.4s, v0.4s, v2.4s",
        ),
        (
            3,
            64,
            "lsr x1, x0, #6",
            "dup v2.2d, x0",
            "ushl v0.2d, v0.2d, v2.2d",
        ),
    ] {
        cases.check(
            "flush_flags_and_shift_vec",
            &[
                ("fmov_float_gen", "fmov x0, d1"),
                ("lsr_ubfm", lsr_asm),
                ("cbnz", "cbnz x1, #20"),
                ("neg_sub_addsub_shift", "neg w0, w0"),
                ("dup_advsimd_gen", dup_asm),
                ("ushl_advsimd", ushl_asm),
                ("b_uncond", "b #8"),
                ("eor_advsimd", "eor v0.16b, v0.16b, v0.16b"),
            ],
            move |a| a.flush_flags_and_shift_vec(size, ebits, false, false),
        );
    }

    // Logical left shift (arith = false, left = true): 7 words per width.
    for &(size, ebits, lsr_asm, dup_asm, ushl_asm) in &[
        (
            1u32,
            16u64,
            "lsr x1, x0, #4",
            "dup v2.8h, w0",
            "ushl v0.8h, v0.8h, v2.8h",
        ),
        (
            2,
            32,
            "lsr x1, x0, #5",
            "dup v2.4s, w0",
            "ushl v0.4s, v0.4s, v2.4s",
        ),
        (
            3,
            64,
            "lsr x1, x0, #6",
            "dup v2.2d, x0",
            "ushl v0.2d, v0.2d, v2.2d",
        ),
    ] {
        cases.check(
            "flush_flags_and_shift_vec",
            &[
                ("fmov_float_gen", "fmov x0, d1"),
                ("lsr_ubfm", lsr_asm),
                ("cbnz", "cbnz x1, #16"),
                ("dup_advsimd_gen", dup_asm),
                ("ushl_advsimd", ushl_asm),
                ("b_uncond", "b #8"),
                ("eor_advsimd", "eor v0.16b, v0.16b, v0.16b"),
            ],
            move |a| a.flush_flags_and_shift_vec(size, ebits, true, false),
        );
    }
}
