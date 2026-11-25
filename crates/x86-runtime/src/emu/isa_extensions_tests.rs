//! Known-answer vector, 4-tier differential (Generic vs FastOp vs Live JIT vs Disk AOT),
//! and static Mach-O `.aot.o` inclusion tests for Ops Slice 5 ISA extensions:
//! GFNI, AVX-VNNI, VAES-256, VPCLMULQDQ-256, SERIALIZE, RDRAND/RDSEED, `crc32 r32, hi8`,
//! and `reservednop`.

use super::cpu::{with_test_entropy, EntropyResult};
use super::static_macho::{emit_macho_object_from_aot_bytes, MachoPlatform};
use super::*;
use iced_x86::code_asm::*;
use std::sync::atomic::{AtomicU64, Ordering};

/// Run `code` (taking `data` pointer in `rcx`) across all 4 execution tiers
/// (`generic`, `FastOp`, `JIT`, `disk AOT`), checking every tier with `check` and
/// asserting all 4 tiers produce byte-identical output buffers.
fn run_all_tiers_equal(code: &[u8], init: &[u8], mut check: impl FnMut(&str, &[u8])) {
    let mut baseline: Option<Vec<u8>> = None;
    for (name, fast, compiled, cached) in [
        ("generic", false, false, false),
        ("FastOp", true, false, false),
        ("JIT", true, true, false),
        ("disk AOT", true, true, true),
    ] {
        let code_ptr = jit_tests::arena_alloc(code.len() + 16, 16);
        let data = jit_tests::arena_alloc(init.len(), 64);
        unsafe {
            std::ptr::copy_nonoverlapping(code.as_ptr(), code_ptr, code.len());
            std::ptr::write_bytes(code_ptr.add(code.len()), 0xcc, 16);
            std::ptr::copy_nonoverlapping(init.as_ptr(), data, init.len());
        }
        let fast_off = TEST_FAST_OFF.with(|s| s.replace(!fast));
        jit::force(if compiled { 1 } else { 2 });
        let before = jit::test_aot_installs();
        if cached {
            let ips = Decoder::with_ip(64, code, code_ptr as u64, DecoderOptions::NONE)
                .into_iter()
                .map(|ins| ins.ip())
                .collect::<Vec<_>>();
            assert!(unsafe { aot::test_install(&ips, || {}).0 } > 0);
        }
        let saved_mxcsr = win32_rt::ucrt::runtime::guest_mxcsr();
        win32_rt::ucrt::runtime::set_guest_mxcsr(0x1f80);
        unsafe {
            emu_run_full(code_ptr as u64, [data as u64, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]);
        }
        win32_rt::ucrt::runtime::set_guest_mxcsr(saved_mxcsr);
        if cached {
            aot::test_uninstall();
            assert!(jit::test_aot_installs() > before, "{name} must load translated AOT code");
        }
        jit::force(0);
        TEST_FAST_OFF.with(|s| s.set(fast_off));
        let out = unsafe { std::slice::from_raw_parts(data, init.len()) };
        check(name, out);
        if let Some(ref base) = baseline {
            assert_eq!(out, base.as_slice(), "tier `{name}` diverged from `generic`");
        } else {
            baseline = Some(out.to_vec());
        }
    }
}

/// Standard NIST FIPS-197 AES S-Box (256 entries) for verifying `GF2P8AFFINEINVQB`.
#[rustfmt::skip]
const AES_SBOX: [u8; 256] = [
    0x63,0x7c,0x77,0x7b,0xf2,0x6b,0x6f,0xc5,0x30,0x01,0x67,0x2b,0xfe,0xd7,0xab,0x76,
    0xca,0x82,0xc9,0x7d,0xfa,0x59,0x47,0xf0,0xad,0xd4,0xa2,0xaf,0x9c,0xa4,0x72,0xc0,
    0xb7,0xfd,0x93,0x26,0x36,0x3f,0xf7,0xcc,0x34,0xa5,0xe5,0xf1,0x71,0xd8,0x31,0x15,
    0x04,0xc7,0x23,0xc3,0x18,0x96,0x05,0x9a,0x07,0x12,0x80,0xe2,0xeb,0x27,0xb2,0x75,
    0x09,0x83,0x2c,0x1a,0x1b,0x6e,0x5a,0xa0,0x52,0x3b,0xd6,0xb3,0x29,0xe3,0x2f,0x84,
    0x53,0xd1,0x00,0xed,0x20,0xfc,0xb1,0x5b,0x6a,0xcb,0xbe,0x39,0x4a,0x4c,0x58,0xcf,
    0xd0,0xef,0xaa,0xfb,0x43,0x4d,0x33,0x85,0x45,0xf9,0x02,0x7f,0x50,0x3c,0x9f,0xa8,
    0x51,0xa3,0x40,0x8f,0x92,0x9d,0x38,0xf5,0xbc,0xb6,0xda,0x21,0x10,0xff,0xf3,0xd2,
    0xcd,0x0c,0x13,0xec,0x5f,0x97,0x44,0x17,0xc4,0xa7,0x7e,0x3d,0x64,0x5d,0x19,0x73,
    0x60,0x81,0x4f,0xdc,0x22,0x2a,0x90,0x88,0x46,0xee,0xb8,0x14,0xde,0x5e,0x0b,0xdb,
    0xe0,0x32,0x3a,0x0a,0x49,0x06,0x24,0x5c,0xc2,0xd3,0xac,0x62,0x91,0x95,0xe4,0x79,
    0xe7,0xc8,0x37,0x6d,0x8d,0xd5,0x4e,0xa9,0x6c,0x56,0xf4,0xea,0x65,0x7a,0xae,0x08,
    0xba,0x78,0x25,0x2e,0x1c,0xa6,0xb4,0xc6,0xe8,0xdd,0x74,0x1f,0x4b,0xbd,0x8b,0x8a,
    0x70,0x3e,0xb5,0x66,0x48,0x03,0xf6,0x0e,0x61,0x35,0x57,0xb9,0x86,0xc1,0x1d,0x9e,
    0xe1,0xf8,0x98,0x11,0x69,0xd9,0x8e,0x94,0x9b,0x1e,0x87,0xe9,0xce,0x55,0x28,0xdf,
    0x8c,0xa1,0x89,0x0d,0xbf,0xe6,0x42,0x68,0x41,0x99,0x2d,0x0f,0xb0,0x54,0xbb,0x16,
];

#[test]
fn gfni_known_answers_and_4tier_difftest() {
    // Matrices:
    //   IDENTITY    = 0x0102_0408_1020_4080 (bit i -> bit i)
    //   BIT_REVERSE = 0x8040_2010_0804_0201 (bit i -> bit 7-i)
    //   AES_MAT     = 0xF1E3_C78F_1F3E_7CF8 (+ imm8=0x63 gives AES S-Box on gf2p8affineinvqb)
    const IDENTITY: u64 = 0x0102_0408_1020_4080;
    const BIT_REVERSE: u64 = 0x8040_2010_0804_0201;
    const AES_MAT: u64 = 0xF1E3_C78F_1F3E_7CF8;

    // Layout of `buf` (512 bytes):
    //   0x00..0x20: mul_a (32B)
    //   0x20..0x40: mul_b (32B)
    //   0x40..0x60: affine_mat (q0=IDENTITY, q1=BIT_REVERSE, q2=AES_MAT, q3=AES_MAT)
    //   0x60..0x80: affine_in  (32B sample bytes including 0x00, 0x01, 0x53, 0xFF)
    //   0x80..0xA0: sentinel YMM upper-half pattern
    //   0x100..0x120: out0 = legacy gf2p8mulb (YMM hi must stay intact!)
    //   0x120..0x140: out1 = vex128 vgf2p8mulb RM (YMM hi must be zeroed!)
    //   0x140..0x160: out2 = vex256 vgf2p8mulb RR
    //   0x160..0x180: out3 = vex256 vgf2p8affineqb RR (imm8=0x5A)
    //   0x180..0x1A0: out4 = vex256 vgf2p8affineinvqb RM (imm8=0x63 -> AES S-Box!)
    //   0x1A0..0x1C0: out5 = legacy gf2p8affineinvqb RM (imm8=0x63, YMM hi preserved!)
    //   0x1C0..0x1C8: saved RFLAGS across GFNI (must preserve NZCV!)
    let mut init = vec![0u8; 512];
    let mul_a: [u8; 32] = [
        0x57, 0x00, 0x01, 0xFF, 0x02, 0x80, 0x53, 0xCA, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
        0x88, 0x57, 0xA5, 0x3C, 0x0F, 0xF0, 0x81, 0x7E, 0x99, 0x13, 0x29, 0x47, 0x6B, 0x8D, 0xAF,
        0xC3, 0xE7,
    ];
    let mul_b: [u8; 32] = [
        0x83, 0xAB, 0xCD, 0xFF, 0x80, 0x02, 0xCA, 0x53, 0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54, 0x32,
        0x10, 0x13, 0x5A, 0xC3, 0xF0, 0x0F, 0x81, 0x7E, 0x66, 0x01, 0x00, 0x1B, 0x1B, 0x80, 0x80,
        0xFF, 0x57,
    ];
    init[0x00..0x20].copy_from_slice(&mul_a);
    init[0x20..0x40].copy_from_slice(&mul_b);
    init[0x40..0x48].copy_from_slice(&IDENTITY.to_le_bytes());
    init[0x48..0x50].copy_from_slice(&BIT_REVERSE.to_le_bytes());
    init[0x50..0x58].copy_from_slice(&AES_MAT.to_le_bytes());
    init[0x58..0x60].copy_from_slice(&AES_MAT.to_le_bytes());
    let aff_in: [u8; 32] = [
        0x00, 0x01, 0x02, 0x53, 0x80, 0xA5, 0xF0, 0xFF, // q0: IDENTITY ^ 0x5A
        0x00, 0x01, 0x02, 0x53, 0x80, 0xA5, 0xF0, 0xFF, // q1: BIT_REVERSE ^ 0x5A
        0x00, 0x01, 0x02, 0x53, 0x80, 0xCA, 0x7C, 0xFF, // q2: AES SBOX
        0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, // q3: AES SBOX
    ];
    init[0x60..0x80].copy_from_slice(&aff_in);
    init[0x80..0xA0].fill(0xEE);

    let mut a = CodeAssembler::new(64).unwrap();
    // 1. Legacy gf2p8mulb preserves ymm0 upper half.
    a.vmovdqu(ymm0, ymmword_ptr(rcx + 0x80)).unwrap();
    a.movdqu(xmm0, xmmword_ptr(rcx + 0x00)).unwrap();
    a.movdqu(xmm1, xmmword_ptr(rcx + 0x20)).unwrap();
    // Set known flags (CF=1, ZF=0, SF=1, OF=0) and verify GFNI preserves them.
    a.mov(eax, 1).unwrap();
    a.sub(eax, 2).unwrap(); // -1 => CF=1, SF=1, ZF=0, OF=0
    a.gf2p8mulb(xmm0, xmm1).unwrap();
    a.pushfq().unwrap();
    a.pop(qword_ptr(rcx + 0x1C0)).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 0x100), ymm0).unwrap();

    // 2. VEX.128 vgf2p8mulb RM zeroes ymm2 upper half.
    a.vmovdqu(ymm2, ymmword_ptr(rcx + 0x80)).unwrap();
    a.vgf2p8mulb(xmm2, xmm0, xmmword_ptr(rcx + 0x20)).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 0x120), ymm2).unwrap();

    // 3. VEX.256 vgf2p8mulb RR
    a.vmovdqu(ymm3, ymmword_ptr(rcx + 0x00)).unwrap();
    a.vmovdqu(ymm4, ymmword_ptr(rcx + 0x20)).unwrap();
    a.vgf2p8mulb(ymm5, ymm3, ymm4).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 0x140), ymm5).unwrap();

    // 4. VEX.256 vgf2p8affineqb RR (imm8 = 0x5A)
    a.vmovdqu(ymm6, ymmword_ptr(rcx + 0x60)).unwrap();
    a.vmovdqu(ymm7, ymmword_ptr(rcx + 0x40)).unwrap();
    a.vgf2p8affineqb(ymm8, ymm6, ymm7, 0x5A).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 0x160), ymm8).unwrap();

    // 5. VEX.256 vgf2p8affineinvqb RM (imm8 = 0x63)
    a.vgf2p8affineinvqb(ymm9, ymm6, ymmword_ptr(rcx + 0x40), 0x63).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 0x180), ymm9).unwrap();

    // 6. Legacy gf2p8affineinvqb RM (with AES_MAT at +0x50) preserves upper half of ymm10.
    a.vmovdqu(ymm10, ymmword_ptr(rcx + 0x80)).unwrap();
    a.movdqu(xmm10, xmmword_ptr(rcx + 0x70)).unwrap();
    a.gf2p8affineinvqb(xmm10, xmmword_ptr(rcx + 0x50), 0x63).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 0x1A0), ymm10).unwrap();

    a.vzeroupper().unwrap();
    a.ret().unwrap();
    let code = a.assemble(0x1000).unwrap();

    run_all_tiers_equal(&code, &init, |tier, out| {
        // FIPS-197 known vectors: 0x57 * 0x83 = 0xC1, 0 * x = 0, 1 * 0xCD = 0xCD, 0x53 * 0xCA = 0x01 (inverses!).
        assert_eq!(out[0x100], 0xC1, "{tier}: 0x57*0x83");
        assert_eq!(out[0x101], 0x00, "{tier}: 0*0xAB");
        assert_eq!(out[0x102], 0xCD, "{tier}: 1*0xCD");
        assert_eq!(out[0x106], 0x01, "{tier}: 0x53*0xCA == 1");
        assert_eq!(out[0x107], 0x01, "{tier}: 0xCA*0x53 == 1");
        // Legacy gf2p8mulb preserved ymm0[255:128] = 0xEE.
        assert_eq!(&out[0x110..0x120], &[0xEE; 16], "{tier}: legacy YMM hi preserved");
        // VEX.128 vgf2p8mulb zeroed ymm2[255:128].
        assert_eq!(&out[0x130..0x140], &[0x00; 16], "{tier}: VEX.128 YMM hi zeroed");
        // VEX.256 low lane matches legacy lane; high lane 0x57 * 0x13 = 0xFE (FIPS-197!).
        assert_eq!(&out[0x140..0x150], &out[0x100..0x110], "{tier}: VEX.256 lo lane");
        assert_eq!(out[0x150], 0xFE, "{tier}: FIPS-197 0x57*0x13 == 0xFE");
        // Affine q0 = IDENTITY ^ 0x5A, q1 = BIT_REVERSE ^ 0x5A.
        for i in 0..8 {
            assert_eq!(out[0x160 + i], aff_in[i] ^ 0x5A, "{tier}: identity affine[{i}]");
            assert_eq!(
                out[0x168 + i],
                aff_in[8 + i].reverse_bits() ^ 0x5A,
                "{tier}: bit-reverse affine[{i}]"
            );
        }
        // VEX.256 vgf2p8affineinvqb upper 16 bytes (q2, q3 use AES_MAT, imm8=0x63) == AES_SBOX!
        for i in 0..16 {
            assert_eq!(
                out[0x190 + i],
                AES_SBOX[aff_in[16 + i] as usize],
                "{tier}: VEX.256 AES S-Box[{i}]"
            );
        }
        // Legacy gf2p8affineinvqb RM matches AES_SBOX and preserves ymm10[255:128] = 0xEE.
        for i in 0..16 {
            assert_eq!(
                out[0x1A0 + i],
                AES_SBOX[aff_in[16 + i] as usize],
                "{tier}: legacy AES S-Box[{i}]"
            );
        }
        assert_eq!(&out[0x1B0..0x1C0], &[0xEE; 16], "{tier}: legacy affineinv YMM hi preserved");
        // Flags preserved: CF=1 (bit 0), SF=1 (bit 7), ZF=0 (bit 6), OF=0 (bit 11).
        let rflags = u64::from_le_bytes(out[0x1C0..0x1C8].try_into().unwrap());
        assert_eq!(rflags & ((1 << 0) | (1 << 6) | (1 << 7) | (1 << 11)), (1 << 0) | (1 << 7), "{tier}: NZCV preserved");
    });

    // Exhaustive 256-byte AES S-Box verification via VEX.256 vgf2p8affineinvqb!
    let mut sbox_init = vec![0u8; 512];
    for (i, slot) in sbox_init.iter_mut().take(256).enumerate() {
        *slot = i as u8;
    }
    for q in 0..4 {
        sbox_init[0x100 + q * 8..0x108 + q * 8].copy_from_slice(&AES_MAT.to_le_bytes());
    }
    let mut b = CodeAssembler::new(64).unwrap();
    b.vmovdqu(ymm1, ymmword_ptr(rcx + 0x100)).unwrap();
    for chunk in 0..8i32 {
        let off = chunk * 32;
        b.vmovdqu(ymm0, ymmword_ptr(rcx + off)).unwrap();
        b.vgf2p8affineinvqb(ymm0, ymm0, ymm1, 0x63).unwrap();
        b.vmovdqu(ymmword_ptr(rcx + off), ymm0).unwrap();
    }
    b.vzeroupper().unwrap();
    b.ret().unwrap();
    let sbox_code = b.assemble(0x2000).unwrap();
    run_all_tiers_equal(&sbox_code, &sbox_init, |tier, out| {
        assert_eq!(&out[0..256], &AES_SBOX, "{tier}: full 256-entry AES S-Box mismatch");
    });
}

#[test]
fn avx_vnni_saturation_and_4tier_difftest() {
    // Tests all 4 AVX-VNNI mnemonics (vpdpbusd, vpdpbusds, vpdpwssd, vpdpwssds), 128 & 256,
    // RR & RM, aliasing, YMM-hi zeroing on 128-bit, and the critical i32 saturation corner case
    // `(-32768)^2 + (-32768)^2 = +2^31` with dst = -1 and dst = 0.
    let mut init = vec![0u8; 512];
    let put_i32s = |buf: &mut [u8], off: usize, vals: &[i32]| {
        for (i, &v) in vals.iter().enumerate() {
            buf[off + i * 4..off + i * 4 + 4].copy_from_slice(&v.to_le_bytes());
        }
    };
    let put_i16s = |buf: &mut [u8], off: usize, vals: &[i16]| {
        for (i, &v) in vals.iter().enumerate() {
            buf[off + i * 2..off + i * 2 + 2].copy_from_slice(&v.to_le_bytes());
        }
    };

    // 0x00..0x20: dst_init (8x i32)
    put_i32s(&mut init, 0x00, &[
        i32::MAX - 10, // pos overflow test
        i32::MIN + 10, // neg overflow test
        -1,            // (-32768)^2 + (-32768)^2 + (-1) = +2^31 - 1 = i32::MAX!
        0,             // (-32768)^2 + (-32768)^2 + 0 = +2^31 -> saturates to i32::MAX!
        1000,
        -1000,
        i32::MAX,
        i32::MIN,
    ]);
    // 0x20..0x40: busd src1 (u8)
    init[0x20..0x40].copy_from_slice(&[
        255, 255, 255, 255, // lane 0: 4 * 255 * 127 = +129540
        255, 255, 255, 255, // lane 1: 4 * 255 * -128 = -130560
        1, 2, 3, 4,         // lane 2
        200, 100, 50, 25,   // lane 3
        10, 20, 30, 40,     // lane 4
        255, 0, 128, 64,    // lane 5
        1, 1, 1, 1,         // lane 6: +4
        1, 1, 1, 1,         // lane 7: -4
    ]);
    // 0x40..0x60: busd src2 (i8 as u8)
    let busd_b: [i8; 32] = [
        127, 127, 127, 127,
        -128, -128, -128, -128,
        10, -20, 30, -40,
        -1, 2, -3, 4,
        1, 2, 3, 4,
        -1, 127, -2, 3,
        1, 1, 1, 1,
        -1, -1, -1, -1,
    ];
    for (i, &v) in busd_b.iter().enumerate() {
        init[0x40 + i] = v as u8;
    }
    // 0x60..0x80: wssd src1 (16x i16)
    put_i16s(&mut init, 0x60, &[
        30000, 30000,       // lane 0: +1.8e9 (overflows i32::MAX-10)
        -30000, -30000,     // lane 1: with +30000 -> -1.8e9 (underflows i32::MIN+10)
        -32768, -32768,     // lane 2: (-32768)^2 + (-32768)^2 = +2^31!
        -32768, -32768,     // lane 3: (-32768)^2 + (-32768)^2 = +2^31!
        100, -200,          // lane 4
        32767, -32768,      // lane 5
        10, 20,             // lane 6
        -10, -20,           // lane 7
    ]);
    // 0x80..0xA0: wssd src2 (16x i16)
    put_i16s(&mut init, 0x80, &[
        30000, 30000,
        30000, 30000,
        -32768, -32768,
        -32768, -32768,
        3, 4,
        1, 1,
        5, 6,
        5, 6,
    ]);
    init[0xA0..0xC0].fill(0xDD); // sentinel

    let mut a = CodeAssembler::new(64).unwrap();
    // out0 (0x100): vpdpbusd ymm (RR)
    a.vmovdqu(ymm0, ymmword_ptr(rcx + 0x00)).unwrap();
    a.vmovdqu(ymm1, ymmword_ptr(rcx + 0x20)).unwrap();
    a.vmovdqu(ymm2, ymmword_ptr(rcx + 0x40)).unwrap();
    a.vpdpbusd(ymm0, ymm1, ymm2).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 0x100), ymm0).unwrap();

    // out1 (0x120): vpdpbusds ymm (RM)
    a.vmovdqu(ymm0, ymmword_ptr(rcx + 0x00)).unwrap();
    a.vpdpbusds(ymm0, ymm1, ymmword_ptr(rcx + 0x40)).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 0x120), ymm0).unwrap();

    // out2 (0x140): vpdpwssd ymm (RR)
    a.vmovdqu(ymm3, ymmword_ptr(rcx + 0x00)).unwrap();
    a.vmovdqu(ymm4, ymmword_ptr(rcx + 0x60)).unwrap();
    a.vmovdqu(ymm5, ymmword_ptr(rcx + 0x80)).unwrap();
    a.vpdpwssd(ymm3, ymm4, ymm5).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 0x140), ymm3).unwrap();

    // out3 (0x160): vpdpwssds ymm (RM) — tests (-32768)^2 + (-32768)^2 saturation!
    a.vmovdqu(ymm3, ymmword_ptr(rcx + 0x00)).unwrap();
    a.vpdpwssds(ymm3, ymm4, ymmword_ptr(rcx + 0x80)).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 0x160), ymm3).unwrap();

    // out4 (0x180): VEX.128 vpdpwssds RR zeroes upper half of ymm6.
    a.vmovdqu(ymm6, ymmword_ptr(rcx + 0xA0)).unwrap();
    a.movdqu(xmm6, xmmword_ptr(rcx + 0x00)).unwrap();
    a.vpdpwssds(xmm6, xmm4, xmm5).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 0x180), ymm6).unwrap();

    // out5 (0x1A0): aliasing d == s1 == s2 (`vpdpbusd xmm7, xmm7, xmm7`)
    a.vmovdqu(ymm7, ymmword_ptr(rcx + 0x20)).unwrap();
    a.vpdpbusd(xmm7, xmm7, xmm7).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 0x1A0), ymm7).unwrap();

    a.vzeroupper().unwrap();
    a.ret().unwrap();
    let code = a.assemble(0x3000).unwrap();

    let rd_i32 = |buf: &[u8], off: usize| -> i32 {
        i32::from_le_bytes(buf[off..off + 4].try_into().unwrap())
    };
    run_all_tiers_equal(&code, &init, |tier, out| {
        // vpdpbusd (wrapping)
        assert_eq!(rd_i32(out, 0x100), (i32::MAX - 10).wrapping_add(129_540), "{tier}: busd[0]");
        assert_eq!(rd_i32(out, 0x104), (i32::MIN + 10).wrapping_add(-130_560), "{tier}: busd[1]");
        assert_eq!(rd_i32(out, 0x108), -1 + (10 - 40 + 90 - 160), "{tier}: busd[2]");
        // vpdpbusds (saturating)
        assert_eq!(rd_i32(out, 0x120), i32::MAX, "{tier}: busds[0] pos sat");
        assert_eq!(rd_i32(out, 0x124), i32::MIN, "{tier}: busds[1] neg sat");
        assert_eq!(rd_i32(out, 0x138), i32::MAX, "{tier}: busds[6] pos sat");
        assert_eq!(rd_i32(out, 0x13C), i32::MIN, "{tier}: busds[7] neg sat");
        // vpdpwssd (wrapping): (-32768)^2 + (-32768)^2 = 0x8000_0000 (wrapping i32::MIN!)
        assert_eq!(rd_i32(out, 0x148), (-1i32).wrapping_add(i32::MIN), "{tier}: wssd[2] wrap");
        assert_eq!(rd_i32(out, 0x14C), i32::MIN, "{tier}: wssd[3] wrap");
        // vpdpwssds (saturating): MUST be i32::MAX for BOTH lane 2 (dst=-1) and lane 3 (dst=0)!
        assert_eq!(rd_i32(out, 0x160), i32::MAX, "{tier}: wssds[0] pos sat");
        assert_eq!(rd_i32(out, 0x164), i32::MIN, "{tier}: wssds[1] neg sat");
        assert_eq!(rd_i32(out, 0x168), i32::MAX, "{tier}: wssds[2] (-32768)^2*2 + (-1)");
        assert_eq!(rd_i32(out, 0x16C), i32::MAX, "{tier}: wssds[3] (-32768)^2*2 + 0");
        assert_eq!(rd_i32(out, 0x170), 1000 + 300 - 800, "{tier}: wssds[4]");
        assert_eq!(rd_i32(out, 0x174), -1000 + 32767 - 32768, "{tier}: wssds[5]");
        // VEX.128 upper 16 bytes must be zeroed.
        assert_eq!(&out[0x180..0x190], &out[0x160..0x170], "{tier}: wssds 128 lo");
        assert_eq!(&out[0x190..0x1A0], &[0u8; 16], "{tier}: wssds 128 YMM hi zeroed");
        assert_eq!(&out[0x1B0..0x1C0], &[0u8; 16], "{tier}: aliased busd 128 YMM hi zeroed");
    });
}

#[test]
fn vaes256_and_vpclmulqdq256_known_answers_and_4tier_difftest() {
    // NIST FIPS-197 Appendix B AES-128 round 1 & round 10 test vectors across BOTH 128-bit lanes!
    let state0: [u8; 16] = [
        0x19, 0x3d, 0xe3, 0xbe, 0xa0, 0xf4, 0xe2, 0x2b,
        0x9a, 0xc6, 0x8d, 0x2a, 0xe9, 0xf8, 0x48, 0x08,
    ];
    let rk1: [u8; 16] = [
        0xa0, 0xfa, 0xfe, 0x17, 0x88, 0x54, 0x2c, 0xb1,
        0x23, 0xa3, 0x39, 0x39, 0x2a, 0x6c, 0x76, 0x05,
    ];
    let exp_r1: [u8; 16] = [
        0xa4, 0x9c, 0x7f, 0xf2, 0x68, 0x9f, 0x35, 0x2b,
        0x6b, 0x5b, 0xea, 0x43, 0x02, 0x6a, 0x50, 0x49,
    ];
    let state9: [u8; 16] = [
        0xeb, 0x40, 0xf2, 0x1e, 0x59, 0x2e, 0x38, 0x84,
        0x8b, 0xa1, 0x13, 0xe7, 0x1b, 0xc3, 0x42, 0xd2,
    ];
    let rk10: [u8; 16] = [
        0xd0, 0x14, 0xf9, 0xa8, 0xc9, 0xee, 0x25, 0x89,
        0xe1, 0x3f, 0x0c, 0xc8, 0xb6, 0x63, 0x0c, 0xa6,
    ];
    let exp_r10: [u8; 16] = [
        0x39, 0x25, 0x84, 0x1d, 0x02, 0xdc, 0x09, 0xfb,
        0xdc, 0x11, 0x85, 0x97, 0x19, 0x6a, 0x0b, 0x32,
    ];

    let mut init = vec![0u8; 512];
    init[0x00..0x10].copy_from_slice(&state0);
    init[0x10..0x20].copy_from_slice(&state9);
    init[0x20..0x30].copy_from_slice(&rk1);
    init[0x30..0x40].copy_from_slice(&rk10);
    // CLMUL 256-bit operands at 0x40..0x60 and 0x60..0x80:
    // lo lane: a = (0x0000_0000_0000_0002, 0x8000_0000_0000_0000), b = (0x0000_0000_0000_0003, 0x8000_0000_0000_0001)
    // hi lane: a = (0x1234_5678_9ABC_DEF0, 0xFEDC_BA98_7654_3210), b = (0x0F0F_0F0F_0F0F_0F0F, 0x5555_5555_5555_5555)
    let cl_a: [u64; 4] = [0x2, 1u64 << 63, 0x1234_5678_9ABC_DEF0, 0xFEDC_BA98_7654_3210];
    let cl_b: [u64; 4] = [0x3, (1u64 << 63) | 1, 0x0F0F_0F0F_0F0F_0F0F, 0x5555_5555_5555_5555];
    for i in 0..4 {
        init[0x40 + i * 8..0x48 + i * 8].copy_from_slice(&cl_a[i].to_le_bytes());
        init[0x60 + i * 8..0x68 + i * 8].copy_from_slice(&cl_b[i].to_le_bytes());
    }

    let mut a = CodeAssembler::new(64).unwrap();
    a.vmovdqu(ymm0, ymmword_ptr(rcx + 0x00)).unwrap();
    a.vmovdqu(ymm1, ymmword_ptr(rcx + 0x20)).unwrap();
    // vaesenc RR -> 0x100
    a.vaesenc(ymm2, ymm0, ymm1).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 0x100), ymm2).unwrap();
    // vaesenclast RM -> 0x120
    a.vaesenclast(ymm3, ymm0, ymmword_ptr(rcx + 0x20)).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 0x120), ymm3).unwrap();
    // vaesdec RR & vaesdeclast RM -> 0x140, 0x160
    a.vaesdec(ymm4, ymm2, ymm1).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 0x140), ymm4).unwrap();
    a.vaesdeclast(ymm5, ymm3, ymmword_ptr(rcx + 0x20)).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 0x160), ymm5).unwrap();
    // vpclmulqdq 256-bit across all 4 imm8 selectors (0x00 RR, 0x01 RM, 0x10 RR, 0x11 RM)
    a.vmovdqu(ymm6, ymmword_ptr(rcx + 0x40)).unwrap();
    a.vmovdqu(ymm7, ymmword_ptr(rcx + 0x60)).unwrap();
    a.vpclmulqdq(ymm8, ymm6, ymm7, 0x00).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 0x180), ymm8).unwrap();
    a.vpclmulqdq(ymm8, ymm6, ymmword_ptr(rcx + 0x60), 0x01).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 0x1A0), ymm8).unwrap();
    a.vpclmulqdq(ymm8, ymm6, ymm7, 0x10).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 0x1C0), ymm8).unwrap();
    a.vpclmulqdq(ymm8, ymm6, ymmword_ptr(rcx + 0x60), 0x11).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 0x1E0), ymm8).unwrap();

    a.vzeroupper().unwrap();
    a.ret().unwrap();
    let code = a.assemble(0x4000).unwrap();

    run_all_tiers_equal(&code, &init, |tier, out| {
        assert_eq!(&out[0x100..0x110], &exp_r1, "{tier}: vaesenc lo lane (FIPS-197 R1)");
        assert_eq!(&out[0x130..0x140], &exp_r10, "{tier}: vaesenclast hi lane (FIPS-197 R10)");
        // vpclmulqdq imm8=0x00 lo lane: 0x2 * 0x3 = 0x6 (carryless)
        assert_eq!(u128::from_le_bytes(out[0x180..0x190].try_into().unwrap()), 6u128, "{tier}: clmul 0x00");
        // vpclmulqdq imm8=0x11 lo lane: (1<<63) * ((1<<63)|1) = (1<<126) ^ (1<<63)
        let exp_11 = (1u128 << 126) ^ (1u128 << 63);
        assert_eq!(u128::from_le_bytes(out[0x1E0..0x1F0].try_into().unwrap()), exp_11, "{tier}: clmul 0x11");
    });
}

#[test]
fn serialize_crc32_hi8_and_reservednop_4tier_difftest() {
    let init = vec![0u8; 128];
    let mut a = CodeAssembler::new(64).unwrap();
    // Set known flags & GPRs, execute SERIALIZE, verify GPRs and flags survive intact.
    a.mov(rax, 0x1122_3344_5566_7788u64).unwrap();
    a.mov(rdx, 0x99AA_BBCC_DDEE_FF00u64).unwrap();
    a.add(rax, 1).unwrap(); // CF=0, ZF=0, SF=0, OF=0
    a.serialize().unwrap();
    a.pushfq().unwrap();
    a.pop(qword_ptr(rcx + 0x00)).unwrap();
    a.mov(qword_ptr(rcx + 0x08), rax).unwrap();
    a.mov(qword_ptr(rcx + 0x10), rdx).unwrap();

    // `crc32 r32, ah/ch/dh/bh` (legacy dest reg so no REX prefix conflicts with high-8 reg).
    a.mov(esi, 0x1234_5678u32).unwrap();
    a.mov(eax, 0x0000_AB00u32).unwrap();
    a.crc32(esi, ah).unwrap();
    a.mov(edx, 0x0000_CD00u32).unwrap();
    a.crc32(esi, dh).unwrap();
    a.mov(ebx, 0x0000_EF00u32).unwrap();
    a.crc32(esi, bh).unwrap();
    a.mov(dword_ptr(rcx + 0x18), esi).unwrap();

    // `reservednop` multi-byte forms (`0F 1E C0`, `0F 1C 01`) must be clean no-ops.
    a.db(&[0x0F, 0x1E, 0xC0]).unwrap();
    a.db(&[0x0F, 0x1C, 0x01]).unwrap();
    a.mov(qword_ptr(rcx + 0x20), rax).unwrap();
    a.ret().unwrap();
    let code = a.assemble(0x5000).unwrap();

    // Compute reference Castagnoli CRC32C for 0x1234_5678 + [0xAB, 0xCD, 0xEF].
    let mut ref_crc = 0x1234_5678u32;
    for byte in [0xABu8, 0xCD, 0xEF] {
        ref_crc ^= byte as u32;
        for _ in 0..8 {
            ref_crc = if ref_crc & 1 != 0 { (ref_crc >> 1) ^ 0x82F6_3B78 } else { ref_crc >> 1 };
        }
    }

    run_all_tiers_equal(&code, &init, |tier, out| {
        assert_eq!(u64::from_le_bytes(out[0x08..0x10].try_into().unwrap()), 0x1122_3344_5566_7789, "{tier}: rax");
        assert_eq!(u64::from_le_bytes(out[0x10..0x18].try_into().unwrap()), 0x99AA_BBCC_DDEE_FF00, "{tier}: rdx");
        assert_eq!(u32::from_le_bytes(out[0x18..0x1C].try_into().unwrap()), ref_crc, "{tier}: crc32 hi8");
        assert_eq!(u64::from_le_bytes(out[0x20..0x28].try_into().unwrap()), 0x0000_AB00, "{tier}: reservednop");
    });
}

#[test]
fn rdrand_and_rdseed_real_entropy_deterministic_and_failure_semantics() {
    // Build a snippet that tests rdrand/rdseed at 16/32/64-bit, preserves caller-saved guest
    // GPRs (r10..r15) and XMM0 across the OS entropy call, and captures RFLAGS after each op.
    let mut a = CodeAssembler::new(64).unwrap();
    a.mov(r10, 0xA0A1_A2A3_A4A5_A6A7u64).unwrap();
    a.mov(r11, 0xB0B1_B2B3_B4B5_B6B7u64).unwrap();
    a.mov(r12, 0xC0C1_C2C3_C4C5_C6C7u64).unwrap();
    a.mov(r13, 0xD0D1_D2D3_D4D5_D6D7u64).unwrap();
    a.mov(r14, 0xE0E1_E2E3_E4E5_E6E7u64).unwrap();
    a.mov(r15, 0xF0F1_F2F3_F4F5_F6F7u64).unwrap();
    a.movdqu(xmm0, xmmword_ptr(rcx + 0x80)).unwrap();

    // Pre-seed flags to ZF=1, SF=1, OF=1, PF=1 so we verify rdrand/rdseed clear them all!
    a.mov(rax, 0xDEAD_BEEF_CAFE_0000u64).unwrap();
    a.xor(edx, edx).unwrap(); // ZF=1, PF=1
    a.rdrand(ax).unwrap();
    a.pushfq().unwrap();
    a.pop(qword_ptr(rcx + 0x00)).unwrap();
    a.mov(qword_ptr(rcx + 0x08), rax).unwrap();

    a.mov(rbx, 0xFFFF_FFFF_FFFF_FFFFu64).unwrap();
    a.rdrand(ebx).unwrap();
    a.pushfq().unwrap();
    a.pop(qword_ptr(rcx + 0x10)).unwrap();
    a.mov(qword_ptr(rcx + 0x18), rbx).unwrap();

    a.rdseed(r8).unwrap();
    a.pushfq().unwrap();
    a.pop(qword_ptr(rcx + 0x20)).unwrap();
    a.mov(qword_ptr(rcx + 0x28), r8).unwrap();

    a.mov(r9, 0x1111_2222_3333_4444u64).unwrap();
    a.rdseed(r9w).unwrap();
    a.pushfq().unwrap();
    a.pop(qword_ptr(rcx + 0x30)).unwrap();
    a.mov(qword_ptr(rcx + 0x38), r9).unwrap();

    // Store caller-saved GPRs and XMM0 to prove zero clobbering across host call.
    a.mov(qword_ptr(rcx + 0x40), r10).unwrap();
    a.mov(qword_ptr(rcx + 0x48), r11).unwrap();
    a.mov(qword_ptr(rcx + 0x50), r12).unwrap();
    a.mov(qword_ptr(rcx + 0x58), r13).unwrap();
    a.mov(qword_ptr(rcx + 0x60), r14).unwrap();
    a.mov(qword_ptr(rcx + 0x68), r15).unwrap();
    a.movdqu(xmmword_ptr(rcx + 0x90), xmm0).unwrap();
    a.ret().unwrap();
    let code = a.assemble(0x6000).unwrap();

    let mut init = vec![0u8; 256];
    init[0x80..0x90].copy_from_slice(&0x0123_4567_89AB_CDEF_FEDC_BA98_7654_3210u128.to_le_bytes());

    let rd64 = |buf: &[u8], off: usize| -> u64 {
        u64::from_le_bytes(buf[off..off + 8].try_into().unwrap())
    };
    const FLAG_MASK: u64 = (1 << 0) | (1 << 2) | (1 << 6) | (1 << 7) | (1 << 11); // CF,PF,ZF,SF,OF

    // 1. Real OS `getentropy` path: verify CF=1, OF=SF=ZF=PF=0, distinct non-constant 64-bit values!
    let mut real_samples = Vec::new();
    for _ in 0..4 {
        let code_ptr = jit_tests::arena_alloc(code.len() + 16, 16);
        let data = jit_tests::arena_alloc(init.len(), 64);
        unsafe {
            std::ptr::copy_nonoverlapping(code.as_ptr(), code_ptr, code.len());
            std::ptr::write_bytes(code_ptr.add(code.len()), 0xcc, 16);
            std::ptr::copy_nonoverlapping(init.as_ptr(), data, init.len());
        }
        jit::force(1);
        unsafe {
            emu_run_full(code_ptr as u64, [data as u64, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]);
        }
        jit::force(0);
        let out = unsafe { std::slice::from_raw_parts(data, init.len()) };
        for flag_off in [0x00, 0x10, 0x20, 0x30] {
            assert_eq!(rd64(out, flag_off) & FLAG_MASK, 1, "real entropy must set CF=1 and clear PF/ZF/SF/OF");
        }
        assert_eq!(rd64(out, 0x08) >> 16, 0xDEAD_BEEF_CAFE, "rdrand r16 must preserve upper 48 bits");
        assert_eq!(rd64(out, 0x18) >> 32, 0, "rdrand r32 must zero-extend to 64 bits");
        assert_eq!(rd64(out, 0x38) >> 16, 0x1111_2222_3333, "rdseed r16 must preserve upper 48 bits");
        assert_eq!(rd64(out, 0x40), 0xA0A1_A2A3_A4A5_A6A7);
        assert_eq!(rd64(out, 0x48), 0xB0B1_B2B3_B4B5_B6B7);
        assert_eq!(rd64(out, 0x50), 0xC0C1_C2C3_C4C5_C6C7);
        assert_eq!(rd64(out, 0x58), 0xD0D1_D2D3_D4D5_D6D7);
        assert_eq!(rd64(out, 0x60), 0xE0E1_E2E3_E4E5_E6E7);
        assert_eq!(rd64(out, 0x68), 0xF0F1_F2F3_F4F5_F6F7);
        assert_eq!(&out[0x90..0xA0], &init[0x80..0x90], "XMM0 must survive entropy call");
        let r8_val = rd64(out, 0x28);
        assert_ne!(r8_val, 0x9E37_79B9_7F4A_7C15, "must not return old hardcoded constant");
        real_samples.push(r8_val);
    }
    real_samples.sort_unstable();
    real_samples.dedup();
    assert_eq!(real_samples.len(), 4, "4 consecutive 64-bit getentropy samples must be distinct");

    // 2. Deterministic success path across all 4 tiers (Generic == FastOp == Live JIT == Disk AOT).
    static SEQ: AtomicU64 = AtomicU64::new(0);
    fn det_ok() -> EntropyResult {
        let i = SEQ.fetch_add(1, Ordering::Relaxed);
        EntropyResult { val: 0x1020_3040_5060_7080u64.wrapping_add(i * 0x1111_2222_3333_4444), ok: 1 }
    }
    with_test_entropy(det_ok, || {
        let mut baseline: Option<Vec<u8>> = None;
        for (name, fast, compiled, cached) in [
            ("generic", false, false, false),
            ("FastOp", true, false, false),
            ("JIT", true, true, false),
            ("disk AOT", true, true, true),
        ] {
            SEQ.store(0, Ordering::Relaxed);
            let code_ptr = jit_tests::arena_alloc(code.len() + 16, 16);
            let data = jit_tests::arena_alloc(init.len(), 64);
            unsafe {
                std::ptr::copy_nonoverlapping(code.as_ptr(), code_ptr, code.len());
                std::ptr::write_bytes(code_ptr.add(code.len()), 0xcc, 16);
                std::ptr::copy_nonoverlapping(init.as_ptr(), data, init.len());
            }
            let fast_off = TEST_FAST_OFF.with(|s| s.replace(!fast));
            jit::force(if compiled { 1 } else { 2 });
            if cached {
                let ips = Decoder::with_ip(64, &code, code_ptr as u64, DecoderOptions::NONE)
                    .into_iter()
                    .map(|ins| ins.ip())
                    .collect::<Vec<_>>();
                assert!(unsafe { aot::test_install(&ips, || {}).0 } > 0);
            }
            unsafe {
                emu_run_full(code_ptr as u64, [data as u64, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]);
            }
            if cached {
                aot::test_uninstall();
            }
            jit::force(0);
            TEST_FAST_OFF.with(|s| s.set(fast_off));
            let out = unsafe { std::slice::from_raw_parts(data, init.len()) };
            assert_eq!(rd64(out, 0x08), 0xDEAD_BEEF_CAFE_7080, "{name}: rdrand r16");
            assert_eq!(rd64(out, 0x18), 0x8393_B4C4, "{name}: rdrand r32");
            assert_eq!(rd64(out, 0x28), 0x3242_7484_B6C6_F908, "{name}: rdseed r64");
            assert_eq!(rd64(out, 0x38), 0x1111_2222_3333_3D4C, "{name}: rdseed r16");
            for flag_off in [0x00, 0x10, 0x20, 0x30] {
                assert_eq!(rd64(out, flag_off) & FLAG_MASK, 1, "{name}: CF=1 PF=ZF=SF=OF=0");
            }
            if let Some(ref b) = baseline {
                assert_eq!(out, b.as_slice(), "{name} diverged from generic");
            } else {
                baseline = Some(out.to_vec());
            }
        }
    });

    // 3. Failure path (`ok = 0`): Intel SDM requires dst = 0 (r16 preserves upper 48 bits), CF = 0,
    //    OF = SF = ZF = PF = 0 across all 4 tiers.
    fn det_fail() -> EntropyResult {
        EntropyResult { val: 0xBAD0_BAD0_BAD0_BAD0, ok: 0 }
    }
    with_test_entropy(det_fail, || {
        run_all_tiers_equal(&code, &init, |tier, out| {
            assert_eq!(rd64(out, 0x08), 0xDEAD_BEEF_CAFE_0000, "{tier}: fail rdrand r16 zeroes low 16");
            assert_eq!(rd64(out, 0x18), 0, "{tier}: fail rdrand r32 zeroes all 64");
            assert_eq!(rd64(out, 0x28), 0, "{tier}: fail rdseed r64 zeroes all 64");
            assert_eq!(rd64(out, 0x38), 0x1111_2222_3333_0000, "{tier}: fail rdseed r16 zeroes low 16");
            for flag_off in [0x00, 0x10, 0x20, 0x30] {
                assert_eq!(rd64(out, flag_off) & FLAG_MASK, 0, "{tier}: fail clears CF/PF/ZF/SF/OF");
            }
        });
    });
}

#[test]
fn slice5_ops_aot_and_static_macho_inclusion() {
    // Emit a single guest function containing all new Slice 5 ops and verify that:
    //   1. Every instruction lowers to `InstructionLowering::Arm64`;
    //   2. `aot::build_image_for_test` compiles it into a single native AOT block;
    //   3. `emit_macho_object_from_aot_bytes` emits clean static Mach-O `.aot.o` for both
    //      macOS and iOS with `skipped_reloc_blocks == 0` (zero runtime relocations!).
    let mut code_buf = vec![0x90u8; 4096].into_boxed_slice();
    let base = code_buf.as_ptr() as u64;
    let entry_ip = base + 0x100;

    let mut a = CodeAssembler::new(64).unwrap();
    a.gf2p8mulb(xmm0, xmm1).unwrap();
    a.vgf2p8affineqb(ymm0, ymm1, ymm2, 0x5A).unwrap();
    a.vgf2p8affineinvqb(ymm0, ymm1, ymm2, 0x63).unwrap();
    a.vpdpbusd(ymm0, ymm1, ymm2).unwrap();
    a.vpdpbusds(ymm0, ymm1, ymm2).unwrap();
    a.vpdpwssd(ymm0, ymm1, ymm2).unwrap();
    a.vpdpwssds(ymm0, ymm1, ymm2).unwrap();
    a.vaesenc(ymm0, ymm1, ymm2).unwrap();
    a.vaesenclast(ymm0, ymm1, ymm2).unwrap();
    a.vaesdec(ymm0, ymm1, ymm2).unwrap();
    a.vaesdeclast(ymm0, ymm1, ymm2).unwrap();
    a.vpclmulqdq(ymm0, ymm1, ymm2, 0x11).unwrap();
    a.serialize().unwrap();
    a.rdrand(rax).unwrap();
    a.rdseed(edx).unwrap();
    a.crc32(eax, ah).unwrap();
    a.db(&[0x0F, 0x1E, 0xC0]).unwrap(); // reservednop
    a.vzeroupper().unwrap();
    a.ret().unwrap();
    let bytes = a.assemble(entry_ip).unwrap();
    code_buf[0x100..0x100 + bytes.len()].copy_from_slice(&bytes);

    let mut dec = Decoder::with_ip(64, &bytes, entry_ip, DecoderOptions::NONE);
    while dec.can_decode() {
        let ins = dec.decode();
        assert_eq!(
            instruction_lowering(&ins),
            InstructionLowering::Arm64,
            "Slice 5 op `{ins}` must lower to Arm64"
        );
    }

    let mut bc = Box::new(block_cache::BlockCache::new());
    let img = unsafe { aot::build_image_for_test(&mut *bc, &[entry_ip]) }
        .expect("build AOT image for Slice 5 ops");
    assert_eq!(img.blocks.len(), 1, "entire Slice 5 sequence must compile as 1 native AOT block");
    let key = aot::AotKey {
        image_hash: [0x51CE_5A07_1111_2222, 0x3333_4444_5555_6666],
        image_base: base,
        image_size: code_buf.len() as u64,
        codegen_hash: aot::codegen_hash(),
        knobs: jit::aot_knob_bits(),
    };
    let aot_bytes = aot::encode_for_test("slice5_isa.dll", &key, &img);
    for plat in [MachoPlatform::MacOs, MachoPlatform::Ios] {
        let (obj, rep) = emit_macho_object_from_aot_bytes(&aot_bytes, plat)
            .unwrap_or_else(|e| panic!("emit_macho_object_from_aot_bytes({plat}) failed: {e}"));
        assert_eq!(rep.emitted_blocks, 1);
        assert_eq!(rep.skipped_reloc_blocks, 0, "{plat}: Slice 5 ops must emit zero skipped reloc blocks");
        assert!(!obj.is_empty());
    }
    drop(code_buf);
}
