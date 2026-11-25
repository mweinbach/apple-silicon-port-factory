use super::*;
use iced_x86::code_asm::*;
use std::io::Write;
use std::process::{Command, Stdio};

/// Feed the very same guest text and aligned input to Rosetta. Compile it into read-only
/// Mach-O text so this control does not share the emulator's decoder or require MAP_JIT.
fn rosetta(code: &[u8], init: &[u8]) -> Vec<u8> {
    let temp = Command::new("mktemp").args(["-d", "-t", "port-cpu-oracle"]).output().unwrap();
    assert!(temp.status.success());
    let dir = std::path::PathBuf::from(std::str::from_utf8(&temp.stdout).unwrap().trim());
    struct Cleanup(std::path::PathBuf);
    impl Drop for Cleanup {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
    let _cleanup = Cleanup(dir.clone());
    let exe = dir.join("oracle");
    let mut cc = Command::new("clang")
        .args(["-arch", "x86_64", "-x", "c", "-", "-o"])
        .arg(&exe)
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    let bytes = |v: &[u8]| v.iter().map(u8::to_string).collect::<Vec<_>>().join(",");
    writeln!(
        cc.stdin.take().unwrap(),
        r#"
        #include <stdio.h>
        __asm__(".text\n.p2align 4\n_guest:\n.byte {}\n");
        extern void __attribute__((ms_abi)) guest(void *);
        static unsigned char data[{}] __attribute__((aligned(64))) = {{{}}};
        int main(void) {{ guest(data); return fwrite(data, 1, sizeof data, stdout) != sizeof data; }}
    "#,
        bytes(code),
        init.len(),
        bytes(init)
    )
    .unwrap();
    assert!(cc.wait().unwrap().success());
    let out =
        Command::new("arch").arg("-x86_64").arg(exe).env("ROSETTA_ADVERTISE_AVX", "1").output().unwrap();
    assert!(out.status.success(), "Rosetta {out:?}");
    assert_eq!(out.stdout.len(), init.len());
    out.stdout
}

fn run_tiers(code: &[u8], init: &[u8], mut check: impl FnMut(&[u8])) {
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
        let x87 = regfile_ext::x87_snapshot();
        win32_rt::ucrt::runtime::set_guest_mxcsr(0x1f80);
        regfile_ext::x87_reset();
        unsafe {
            emu_run_full(code_ptr as u64, [data as u64, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]);
        }
        win32_rt::ucrt::runtime::set_guest_mxcsr(saved_mxcsr);
        regfile_ext::x87_restore(x87);
        if cached {
            aot::test_uninstall();
            assert!(jit::test_aot_installs() > before, "{name} must actually load translated code");
        }
        jit::force(0);
        TEST_FAST_OFF.with(|s| s.set(fast_off));
        check(unsafe { std::slice::from_raw_parts(data, init.len()) });
    }
}

#[test]
fn raw_x87_nan_store_reports_invalid_only_when_consumed() {
    let mut a = CodeAssembler::new(64).unwrap();
    a.fninit().unwrap();
    a.fld(tbyte_ptr(rcx)).unwrap();
    a.fnstsw(word_ptr(rcx + 16)).unwrap();
    a.fxam().unwrap();
    a.fnstsw(word_ptr(rcx + 18)).unwrap();
    a.fst(qword_ptr(rcx + 24)).unwrap();
    a.fnstsw(word_ptr(rcx + 20)).unwrap();
    a.fnclex().unwrap();
    a.fstp(qword_ptr(rcx + 32)).unwrap();
    a.fnstsw(word_ptr(rcx + 22)).unwrap();
    a.ret().unwrap();
    let code = a.assemble(0).unwrap();
    for quiet_bit in [0, 1u64 << 62] {
        let mut init = [0u8; 40];
        init[..8].copy_from_slice(&(0x8000_1234_5678_9000 | quiet_bit).to_le_bytes());
        init[8..10].copy_from_slice(&0x7fffu16.to_le_bytes());
        let oracle = cfg!(feature = "rosetta-oracle").then(|| rosetta(&code, &init));
        let check = |out: &[u8]| {
            let sw = |i| u16::from_le_bytes([out[i], out[i + 1]]);
            assert_eq!(sw(16) & 0x3f, 0, "FLD m80 is a raw transfer");
            assert_eq!(sw(18) & 0x473f, 0x0100, "FXAM classifies either NaN without raising IE");
            let invalid = u16::from(quiet_bit == 0);
            assert_eq!(sw(20) & 0x3f, invalid, "FST m64");
            assert_eq!(sw(22) & 0x3f, invalid, "FSTP m64 after FNCLEX");
            for off in [24, 32] {
                assert_eq!(&out[off..off + 8], &0x7ff8_0002_468a_cf12u64.to_le_bytes());
            }
        };
        if let Some(control) = &oracle {
            check(control);
        }
        run_tiers(&code, &init, |out| {
            check(out);
            if let Some(control) = &oracle {
                assert_eq!(out, control);
            }
        });
    }
}

#[test]
fn fprem_raw_operand_exception_priority() {
    let mut a = CodeAssembler::new(64).unwrap();
    a.fninit().unwrap();
    a.fld(tbyte_ptr(rcx + 16)).unwrap();
    a.fld(tbyte_ptr(rcx)).unwrap();
    a.fprem().unwrap();
    a.fnstsw(word_ptr(rcx + 32)).unwrap();
    a.fninit().unwrap();
    a.ret().unwrap();
    let code = a.assemble(0).unwrap();
    let raw = |sig: u64, exponent: u16| {
        let mut slot = [0; 10];
        slot[..8].copy_from_slice(&sig.to_le_bytes());
        slot[8..].copy_from_slice(&exponent.to_le_bytes());
        slot
    };
    let one = raw(1 << 63, 0x3fff);
    let denormal = raw(1, 0);
    let snan = raw((1 << 63) | 1, 0x7fff);
    let qnan = raw((3 << 62) | 1, 0x7fff);
    for (dividend, divisor, flags) in
        [(denormal, one, 2), (snan, one, 1), (denormal, snan, 1), (qnan, denormal, 0)]
    {
        let mut init = [0; 34];
        init[..10].copy_from_slice(&dividend);
        init[16..26].copy_from_slice(&divisor);
        let oracle = cfg!(feature = "rosetta-oracle").then(|| rosetta(&code, &init));
        run_tiers(&code, &init, |out| {
            assert_eq!(out[32] & 0x3f, flags);
            if let Some(control) = &oracle {
                assert_eq!(&out[32..], &control[32..]);
            }
        });
    }
}

#[test]
fn x87_raw_denormal_operand_status() {
    let mut a = CodeAssembler::new(64).unwrap();
    a.fninit().unwrap();
    a.fld(tbyte_ptr(rcx)).unwrap();
    a.fnstsw(word_ptr(rcx + 32)).unwrap();
    a.fxam().unwrap();
    a.fnstsw(word_ptr(rcx + 34)).unwrap();
    a.fstp(tbyte_ptr(rcx + 64)).unwrap();
    a.fnstsw(word_ptr(rcx + 36)).unwrap();

    a.fld(tbyte_ptr(rcx)).unwrap();
    a.fld1().unwrap();
    a.faddp(st1, st0).unwrap();
    a.fnstsw(word_ptr(rcx + 38)).unwrap();
    a.fstp(qword_ptr(rcx + 80)).unwrap();
    a.fninit().unwrap();
    a.fld(tbyte_ptr(rcx)).unwrap();
    a.fadd(qword_ptr(rcx + 16)).unwrap();
    a.fnstsw(word_ptr(rcx + 40)).unwrap();
    a.fninit().unwrap();
    a.fld(tbyte_ptr(rcx)).unwrap();
    a.fsqrt().unwrap();
    a.fnstsw(word_ptr(rcx + 42)).unwrap();
    a.fninit().unwrap();
    a.fld(tbyte_ptr(rcx)).unwrap();
    a.fld1().unwrap();
    a.fucomip(st0, st1).unwrap();
    a.fnstsw(word_ptr(rcx + 44)).unwrap();
    a.fninit().unwrap();
    a.fld(tbyte_ptr(rcx)).unwrap();
    a.frndint().unwrap();
    a.fnstsw(word_ptr(rcx + 46)).unwrap();
    a.fninit().unwrap();
    a.ret().unwrap();
    let code = a.assemble(0).unwrap();
    for (sig, exp, other, expected) in [
        (1u64, 0u16, 1.0f64, [0, 0, 0, 2, 2, 2, 2, 2]),
        (1, 0x8000, f64::NAN, [0, 0, 0, 2, 0, 0, 2, 2]),
        (1 << 63, 0, 1.0, [0, 0, 0, 2, 2, 2, 2, 2]), // pseudo-denormal
        (0, 0x8000, 1.0, [0; 8]),                    // negative zero
    ] {
        let mut init = [0u8; 88];
        init[..8].copy_from_slice(&sig.to_le_bytes());
        init[8..10].copy_from_slice(&exp.to_le_bytes());
        init[16..24].copy_from_slice(&other.to_le_bytes());
        let check = |out: &[u8]| {
            let de: [u8; 8] = std::array::from_fn(|i| out[32 + 2 * i] & 2);
            assert_eq!(de, expected, "raw sign/exponent {exp:#x}, significand {sig:#x}");
            let class = if sig != 0 { 0x4400 } else { 0x4000 };
            let sign = (exp >> 15) << 9;
            let fxam = u16::from_le_bytes([out[34], out[35]]);
            assert_eq!(fxam & 0x4700, class | sign, "FXAM classification");
            assert_eq!(&out[64..74], &init[..10], "m80 copy");
            assert_eq!(&out[80..88], &1.0f64.to_le_bytes(), "sum with one");
        };
        if cfg!(feature = "rosetta-oracle") {
            check(&rosetta(&code, &init));
        }
        run_tiers(&code, &init, check);
    }
}

#[test]
fn x87_raw_denormal_divisor_priority() {
    let mut a = CodeAssembler::new(64).unwrap();
    a.fninit().unwrap();
    a.fld(tbyte_ptr(rcx)).unwrap();
    a.fldz().unwrap();
    a.fdivp(st1, st0).unwrap();
    a.fnstsw(word_ptr(rcx + 16)).unwrap();
    a.fninit().unwrap();
    a.fld(tbyte_ptr(rcx)).unwrap();
    a.fldz().unwrap();
    a.fdivrp(st1, st0).unwrap();
    a.fnstsw(word_ptr(rcx + 18)).unwrap();
    a.fninit().unwrap();
    a.ret().unwrap();
    let code = a.assemble(0).unwrap();
    let mut init = [0u8; 20];
    init[0] = 1;
    let check = |out: &[u8]| {
        assert_eq!(out[16] & 2, 0, "divide by zero takes priority over DE");
        assert_eq!(out[18] & 2, 2, "a raw denormal divisor is nonzero");
    };
    if cfg!(feature = "rosetta-oracle") {
        check(&rosetta(&code, &init));
    }
    run_tiers(&code, &init, check);
}

#[test]
fn xstate_roundtrip_across_tiers() {
    // Rosetta advertises XSAVE but not XSAVEOPT or XSAVEC. All three save formats are
    // independently checked against the same restored x87, XMM, YMM and MXCSR values.
    for (save, compact, legacy) in [(0, false, true), (1, false, false), (2, false, false), (3, true, false)]
    {
        for wide in [false, true] {
            let mut a = CodeAssembler::new(64).unwrap();
            a.fninit().unwrap();
            a.fld(tbyte_ptr(rcx)).unwrap();
            a.fldcw(word_ptr(rcx + 12)).unwrap();
            a.ldmxcsr(dword_ptr(rcx + 16)).unwrap();
            a.vmovups(ymm0, ymmword_ptr(rcx + 32)).unwrap();
            a.vmovups(ymm15, ymmword_ptr(rcx + 64)).unwrap();
            a.mov(eax, 7).unwrap();
            a.xor(edx, edx).unwrap();
            let area = ptr(rcx + 128);
            match (save, wide) {
                (0, false) => a.fxsave(area),
                (0, true) => a.fxsave64(area),
                (1, false) => a.xsave(area),
                (1, true) => a.xsave64(area),
                (2, false) => a.xsaveopt(area),
                (2, true) => a.xsaveopt64(area),
                (3, false) => a.xsavec(area),
                _ => a.xsavec64(area),
            }
            .unwrap();
            a.vzeroall().unwrap();
            a.fninit().unwrap();
            a.ldmxcsr(dword_ptr(rcx + 20)).unwrap();
            match (legacy, wide) {
                (true, false) => a.fxrstor(area),
                (true, true) => a.fxrstor64(area),
                (false, false) => a.xrstor(area),
                (false, true) => a.xrstor64(area),
            }
            .unwrap();
            a.fnstcw(word_ptr(rcx + 992)).unwrap();
            a.stmxcsr(dword_ptr(rcx + 996)).unwrap();
            a.fstp(tbyte_ptr(rcx + 1000)).unwrap();
            a.vmovups(ymmword_ptr(rcx + 1024), ymm0).unwrap();
            a.vmovups(ymmword_ptr(rcx + 1056), ymm15).unwrap();
            a.vzeroupper().unwrap();
            a.ret().unwrap();
            let code = a.assemble(0).unwrap();
            let mut init = vec![0; 1088];
            init[..10].copy_from_slice(&[5, 4, 3, 2, 1, 0, 0, 0x80, 0xff, 0x3f]);
            init[12..14].copy_from_slice(&0x0b7fu16.to_le_bytes());
            init[16..20].copy_from_slice(&0x9fc0u32.to_le_bytes());
            init[20..24].copy_from_slice(&0x1f80u32.to_le_bytes());
            for (i, byte) in init[32..96].iter_mut().enumerate() {
                *byte = 3 * i as u8 + 1;
            }
            init[128 + 464..128 + 512].fill(0xa5);
            let oracle = (save < 2 && cfg!(feature = "rosetta-oracle")).then(|| rosetta(&code, &init));
            run_tiers(&code, &init, |out| {
                let area = &out[128..];
                assert_eq!(&area[..2], &init[12..14]);
                assert_eq!(u16::from_le_bytes(area[2..4].try_into().unwrap()) >> 11 & 7, 7);
                assert_eq!(area[4], 0x80);
                assert_eq!(&area[32..42], &init[..10]);
                assert_eq!(&area[160..176], &init[32..48]);
                assert_eq!(&area[400..416], &init[64..80]);
                assert_eq!(&area[464..512], &[0xa5; 48]);
                if !legacy {
                    assert_eq!(area[512], 7);
                    assert_eq!(
                        u64::from_le_bytes(area[520..528].try_into().unwrap()),
                        if compact { 1 << 63 | 7 } else { 0 }
                    );
                    assert_eq!(&area[576..592], &init[48..64]);
                }
                assert_eq!(&out[992..994], &init[12..14]);
                assert_eq!(&out[996..1000], &init[16..20]);
                assert_eq!(&out[1000..1010], &init[..10]);
                for i in [0, 32] {
                    assert_eq!(&out[1024 + i..1040 + i], &init[32 + i..48 + i]);
                    assert_eq!(
                        &out[1040 + i..1056 + i],
                        if legacy { &[0; 16][..] } else { &init[48 + i..64 + i] }
                    );
                }
                if let Some(control) = &oracle {
                    assert_eq!(&out[992..], &control[992..], "Rosetta restored values");
                    assert_eq!(&area[..5], &control[128..133], "Rosetta FCW, FSW and FTW");
                }
            });
        }
    }
}

#[test]
fn independent_denorm_modes_across_tiers() {
    let mut a = CodeAssembler::new(64).unwrap();
    for mode in 0..4 {
        a.ldmxcsr(dword_ptr(rcx + 4 * mode)).unwrap();
        a.vmovups(ymm0, ymmword_ptr(rcx + 32)).unwrap();
        a.vmulps(ymm0, ymm0, ymmword_ptr(rcx + 64)).unwrap();
        a.vmovups(ymmword_ptr(rcx + 96 + 32 * mode), ymm0).unwrap();
    }
    a.vzeroupper().unwrap();
    a.ret().unwrap();
    let code = a.assemble(0).unwrap();
    let mut init = vec![0; 224];
    for (i, mxcsr) in [0x1f80u32, 0x1fc0, 0x9f80, 0x9fc0].into_iter().enumerate() {
        init[4 * i..4 * i + 4].copy_from_slice(&mxcsr.to_le_bytes());
    }
    for i in 0..8 {
        let (lhs, rhs): (u32, u32) = match i % 4 {
            0 => (1, 0x7e80_0000),
            1 => (0x0080_0000, 0x3f00_0000),
            2 => (0x0080_0000, 0x3f7f_ffff),
            _ => (0x3f80_0000, 0x3f80_0000),
        };
        init[32 + 4 * i..36 + 4 * i].copy_from_slice(&(lhs | ((i as u32 / 4) << 31)).to_le_bytes());
        init[64 + 4 * i..68 + 4 * i].copy_from_slice(&rhs.to_le_bytes());
    }
    let oracle = cfg!(feature = "rosetta-oracle").then(|| rosetta(&code, &init));
    for mask in [0, features::HostFeatures::AFP] {
        features::without_features(mask, || {
            run_tiers(&code, &init, |out| {
                for mode in 0..4 {
                    let expected = [
                        if mode & 1 == 0 { 0x3400_0000 } else { 0 },
                        if mode & 2 == 0 { 0x0040_0000 } else { 0 },
                        if mode & 2 == 0 { 0x0080_0000 } else { 0 },
                        0x3f80_0000,
                    ];
                    for i in 0..8 {
                        let off = 96 + 32 * mode + 4 * i;
                        assert_eq!(
                            u32::from_le_bytes(out[off..off + 4].try_into().unwrap()),
                            expected[i % 4] | ((i as u32 / 4) << 31),
                            "mode {mode}, lane {i}, feature mask {mask}"
                        );
                    }
                }
                if let Some(control) = &oracle {
                    assert_eq!(out, control);
                }
            })
        });
    }
}

#[test]
fn cpuid_subleaves_and_feature_fallbacks() {
    use features::HostFeatures as H;
    let leaves = [
        (0, 0),
        (1, 0),
        (4, 0),
        (4, 3),
        (4, 4),
        (7, 0),
        (7, 1),
        (0xb, 0),
        (0xb, 1),
        (0x1f, 1),
        (0x1f, 2),
        (0xd, 0),
        (0xd, 1),
        (0xd, 2),
        (0xd, 3),
        (0x8000_0001, 0),
        (0x8000_0002, 0),
        (0x8000_0003, 0),
        (0x8000_0004, 0),
    ];
    let mut a = CodeAssembler::new(64).unwrap();
    a.mov(r8, rcx).unwrap();
    for (i, (leaf, sub)) in leaves.iter().enumerate() {
        a.mov(eax, *leaf).unwrap();
        a.mov(ecx, *sub).unwrap();
        a.cpuid().unwrap();
        for (j, reg) in [eax, ebx, ecx, edx].into_iter().enumerate() {
            a.mov(dword_ptr(r8 + (i * 16 + j * 4) as i32), reg).unwrap();
        }
    }
    a.xor(ecx, ecx).unwrap();
    a.xgetbv().unwrap();
    a.mov(dword_ptr(r8 + (leaves.len() * 16) as i32), eax).unwrap();
    a.ret().unwrap();
    let code = a.assemble(0).unwrap();
    for disabled in [0, u8::MAX] {
        features::without_features(disabled, || {
            let cores = win32_rt::process::num_cpus().clamp(1, 64);
            let [_, b, c, d] = cpuid::query(1, 0);
            assert_eq!(b >> 16 & 0xff, cores);
            assert_eq!(d >> 23 & 15, 15); // MMX, FXSR, SSE, SSE2
            assert_eq!(
                c & ((1 << 12) | (1 << 22) | (1 << 23) | (1 << 26) | (1 << 27) | (1 << 28) | (1 << 29)),
                (1 << 12) | (1 << 22) | (1 << 23) | (1 << 26) | (1 << 27) | (1 << 28) | (1 << 29)
            ); // FMA, MOVBE, POPCNT, XSAVE, OSXSAVE, AVX, F16C
            assert_eq!(c & ((1 << 5) | (1 << 30)), 0); // VMX, RDRAND
            assert_eq!(c >> 25 & 1, u32::from(features::host().has(H::AES)));
            assert_eq!(c >> 1 & 1, u32::from(features::host().has(H::PMULL)));
            assert_eq!(c >> 20 & 1, u32::from(features::host().has(H::CRC32)));
            let leaf7_ebx = cpuid::query(7, 0)[1];
            assert_eq!(leaf7_ebx >> 29 & 1, u32::from(features::host().has(H::SHA1 | H::SHA256)));
            assert_eq!(
                leaf7_ebx & ((1 << 3) | (1 << 5) | (1 << 8) | (1 << 9) | (1 << 19)),
                (1 << 3) | (1 << 5) | (1 << 8) | (1 << 9) | (1 << 19)
            ); // BMI1, AVX2, BMI2, ERMS, ADX
            assert_eq!(
                leaf7_ebx
                    & ((1 << 7)
                        | (1 << 10)
                        | (1 << 16)
                        | (1 << 17)
                        | (1 << 18)
                        | (1 << 21)
                        | (1 << 26)
                        | (1 << 27)
                        | (1 << 28)
                        | (1 << 30)
                        | (1 << 31)),
                0
            ); // No SMEP, INVPCID, RDSEED, or AVX-512
            assert_eq!(cpuid::query(7, 1), [0; 4]);
            assert_eq!(cpuid::query(0x8000_0001, 0)[2] & (1 << 5), 1 << 5); // LZCNT
            assert_eq!(cpuid::query(0x8000_0001, 0)[2] & (1 << 6), 0);
            assert_eq!(unsafe { win32_rt::kernel32::sysinfo::GetEnabledXStateFeatures() }, 7);
            assert_eq!(cpuid::query(4, 0)[0] >> 26, cores - 1);
            assert_eq!(cpuid::query(0xb, 0)[1], 1);
            assert_eq!(cpuid::query(0xb, 1), cpuid::query(0x1f, 1));
            assert_eq!(cpuid::query(0x1f, 1)[1], cores);
            assert_eq!(cpuid::query(0x1f, 2)[1], 0);
            assert_eq!(cpuid::query(0xd, 0), [7, 832, 832, 0]);
            assert_eq!(cpuid::query(0xd, 1), [3, 832, 0, 0]);
            assert_eq!(cpuid::query(0xd, 2), [256, 576, 0, 0]);
            run_tiers(&code, &vec![0; leaves.len() * 16 + 4], |out| {
                for (i, (leaf, sub)) in leaves.iter().enumerate() {
                    let (a, b, c, d) = crate::simd_ops::cpuid_x64(*leaf, *sub);
                    for (j, reg) in [a, b, c, d].into_iter().enumerate() {
                        assert_eq!(&out[16 * i + 4 * j..16 * i + 4 * j + 4], &(reg as u32).to_le_bytes());
                    }
                }
                assert_eq!(&out[leaves.len() * 16..], &7u32.to_le_bytes());
            });
        });
    }

    let host_knobs = jit::aot_knob_bits();
    for feature in [H::AES, H::PMULL, H::SHA1, H::SHA256, H::CRC32, H::LSE2, H::LRCPC2, H::AFP] {
        if features::host().has(feature) {
            features::without_features(feature, || {
                assert_ne!(jit::aot_knob_bits(), host_knobs);
                assert!(!features::host().can_run(host_knobs));
            });
        }
    }

    // Compare optional instruction results on the detected host and with all accelerators
    // removed. Both the generic intrinsics and the JIT admission test must select the fallback.
    let mut a = CodeAssembler::new(64).unwrap();
    a.movups(xmm0, xmmword_ptr(rcx)).unwrap();
    a.movups(xmm1, xmmword_ptr(rcx + 16)).unwrap();
    a.aesenc(xmm0, xmm1).unwrap();
    a.pclmulqdq(xmm0, xmm1, 0x11).unwrap();
    a.sha1msg2(xmm0, xmm1).unwrap();
    a.sha256rnds2(xmm0, xmm1).unwrap();
    a.movups(xmmword_ptr(rcx + 32), xmm0).unwrap();
    a.mov(eax, 0x12345678u32).unwrap();
    a.crc32(eax, dword_ptr(rcx)).unwrap();
    a.mov(dword_ptr(rcx + 48), eax).unwrap();
    a.ret().unwrap();
    let code = a.assemble(0).unwrap();
    let init: Vec<u8> = (1..=52).collect();
    let mut reference = None;
    for disabled in [0, u8::MAX] {
        features::without_features(disabled, || {
            if disabled != 0 {
                let dec = Decoder::with_ip(64, &code, 0, DecoderOptions::NONE);
                for ins in dec.into_iter().filter(|ins| {
                    matches!(
                        ins.mnemonic(),
                        Mnemonic::Aesenc
                            | Mnemonic::Pclmulqdq
                            | Mnemonic::Sha1msg2
                            | Mnemonic::Sha256rnds2
                            | Mnemonic::Crc32
                    )
                }) {
                    assert!(!jit::supported(&fast::lower(&ins)), "{ins}");
                }
            }
            run_tiers(&code, &init, |out| {
                if let Some(expected) = &reference {
                    assert_eq!(out, expected);
                } else {
                    reference = Some(out.to_vec());
                }
            });
        });
    }
}

#[test]
fn xstate_masks_and_restore_faults() {
    #[repr(align(64))]
    struct Image([u8; 832]);
    let mut image = Image([0; 832]);
    let mut cpu = Cpu::new_zeroed();
    cpu.gpr[1] = image.0.as_mut_ptr() as u64;
    let ins = |bytes: &[u8]| Decoder::new(64, bytes, DecoderOptions::NONE).decode();
    let save = ins(&[0x48, 0x0f, 0xae, 0x21]);
    let compact = ins(&[0x48, 0x0f, 0xc7, 0x21]);
    let restore = ins(&[0x48, 0x0f, 0xae, 0x29]);
    let previous_fpcr = read_fpcr();
    let saved_x87 = regfile_ext::x87_snapshot();
    for mask in 0..8 {
        for compacted in [false, true] {
            image.0.fill(0xa5);
            image.0[512..576].fill(0);
            image.0[512] = 3;
            cpu.gpr[0] = mask;
            cpu.xmm = [17; 16];
            cpu.ymm_hi = [29; 16];
            cpu.jx.mxcsr = 0x1fc0;
            unsafe {
                cpu.exec_xstate(if compacted { &compact } else { &save }).unwrap();
            }
            assert_eq!(image.0[512], if compacted { mask as u8 } else { 3 | mask as u8 });
            assert_eq!(image.0[160], if mask & 2 != 0 { 17 } else { 0xa5 });
            assert_eq!(image.0[576], if mask & 4 != 0 { 29 } else { 0xa5 });
            assert_eq!(image.0[24], if mask & 2 != 0 || (!compacted && mask & 4 != 0) { 0xc0 } else { 0xa5 });
            image.0[512] = 0;
            cpu.jx.mxcsr = 0x5f80;
            unsafe {
                cpu.exec_xstate(&restore).unwrap();
            }
            assert_eq!(cpu.xmm[15], if mask & 2 != 0 { 0 } else { 17 });
            assert_eq!(cpu.ymm_hi[15], if mask & 4 != 0 { 0 } else { 29 });
            assert_eq!(
                cpu.jx.mxcsr,
                if compacted && mask & 2 != 0 {
                    0x1f80
                } else if !compacted && mask & 6 != 0 {
                    0x1fc0
                } else {
                    0x5f80
                }
            );
        }
    }
    cpu.gpr[0] = 7;
    for (offset, value) in [(512, 8u64), (520, 1), (520, (1 << 63) | 8), (528, 1), (24, 0x1_0000)] {
        image.0.fill(0);
        image.0[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
        cpu.xmm[0] = 0x5a;
        assert!(unsafe { cpu.exec_xstate(&restore) }.is_err());
        assert_eq!(cpu.xmm[0], 0x5a);
    }
    image.0.fill(0);
    image.0[512] = 4;
    image.0[527] = 0x80; // XSTATE_BV outside XCOMP_BV
    assert!(unsafe { cpu.exec_xstate(&restore) }.is_err());
    image.0[512] = 0;
    image.0[575] = 1;
    std::hint::black_box(&image);
    assert!(unsafe { cpu.exec_xstate(&restore) }.is_err());
    cpu.gpr[1] += 1;
    for instruction in
        [save, restore, ins(&[0x0f, 0xae, 0x01]), ins(&[0x0f, 0xae, 0x09]), ins(&[0x0f, 0x01, 0xd0])]
    {
        assert!(unsafe { cpu.exec_xstate(&instruction) }.is_err());
    }
    regfile_ext::x87_restore(saved_x87);
    write_fpcr(previous_fpcr);
}

#[test]
fn mmx_sse_dispatch_vectors() {
    let mut a = CodeAssembler::new(64).unwrap();
    let mut off = 64i32;
    macro_rules! packed {
        ($($name:ident),+) => {$({
            a.movq(mm0,qword_ptr(rcx)).unwrap(); a.movq(mm1,qword_ptr(rcx+8)).unwrap();
            a.$name(mm0,mm1).unwrap(); a.movq(qword_ptr(rcx+off),mm0).unwrap(); off += 8;
        })+}
    }
    packed!(
        paddb, paddw, paddd, paddq, psubb, psubw, psubd, psubq, paddsb, paddsw, paddusb, paddusw, psubsb,
        psubsw, psubusb, psubusw, pmaddwd, pmullw, pmulhw, pmulhuw, pmuludq, pminub, pmaxub, pminsw, pmaxsw,
        pavgb, pavgw, psadbw, pand, pandn, por, pxor, pcmpeqb, pcmpeqw, pcmpeqd, pcmpgtb, pcmpgtw, pcmpgtd,
        packsswb, packuswb, packssdw, punpcklbw, punpcklwd, punpckldq, punpckhbw, punpckhwd, punpckhdq,
        psllw, pslld, psllq, psrlw, psrld, psrlq, psraw, psrad, phaddw, phaddd, phaddsw, phsubw, phsubd,
        phsubsw, pmaddubsw, pmulhrsw, psignb, psignw, psignd, pabsb, pabsw, pabsd, pshufb
    );
    a.pshufw(mm0, mm1, 0x1b).unwrap();
    a.palignr(mm0, mm1, 7).unwrap();
    a.pinsrw(mm0, word_ptr(rcx + 3), 3).unwrap();
    a.pextrw(eax, mm0, 3).unwrap();
    a.mov(dword_ptr(rcx + off), eax).unwrap();
    a.pmovmskb(eax, mm0).unwrap();
    a.mov(dword_ptr(rcx + off + 4), eax).unwrap();
    off += 8;
    a.movdq2q(mm3, xmm0).unwrap();
    a.movq2dq(xmm2, mm0).unwrap();
    a.cvtpi2pd(xmm1, mm0).unwrap();
    a.cvttpd2pi(mm1, xmm1).unwrap();
    a.movq(qword_ptr(rcx + off), mm1).unwrap();
    off += 8;
    a.pxor(xmm0, xmm0).unwrap();
    a.cvtpi2ps(xmm0, qword_ptr(rcx + 1)).unwrap();
    a.cvtps2pi(mm1, xmm0).unwrap();
    a.movq(qword_ptr(rcx + off), mm1).unwrap();
    off += 8;
    a.push(rdi).unwrap();
    a.lea(rdi, ptr(rcx + off)).unwrap();
    a.maskmovq(mm0, mm1).unwrap();
    a.pop(rdi).unwrap();
    off += 8;
    a.emms().unwrap();
    a.movups(xmm0, xmmword_ptr(rcx)).unwrap();
    a.movups(xmm1, xmmword_ptr(rcx + 8)).unwrap();
    a.phaddsw(xmm0, xmm1).unwrap();
    a.phsubsw(xmm1, xmm0).unwrap();
    a.movups(xmmword_ptr(rcx + off), xmm0).unwrap();
    off += 16;
    a.mpsadbw(xmm0, xmm1, 7).unwrap();
    a.movups(xmmword_ptr(rcx + off), xmm0).unwrap();
    off += 16;
    a.phminposuw(xmm0, xmm1).unwrap();
    a.movups(xmmword_ptr(rcx + off), xmm0).unwrap();
    off += 16;
    a.movups(xmm0, xmmword_ptr(rcx + 32)).unwrap();
    a.dppd(xmm0, xmm0, 0x33).unwrap();
    a.movups(xmmword_ptr(rcx + off), xmm0).unwrap();
    off += 16;
    a.push(rdi).unwrap();
    a.lea(rdi, ptr(rcx + off)).unwrap();
    a.maskmovdqu(xmm1, xmm0).unwrap();
    a.pop(rdi).unwrap();
    off += 16;
    a.mov(r8, rcx).unwrap();
    a.mov(rax, 1u64 << 32).unwrap();
    a.mov(rdx, 4u64).unwrap();
    a.pcmpestri64(xmm0, xmm1, 0).unwrap();
    a.mov(dword_ptr(r8 + off), ecx).unwrap();
    off += 4;
    a.ret().unwrap();

    let code = a.assemble(0).unwrap();
    let mut init: Vec<u8> =
        (0..off as usize).map(|i| (i as u8).wrapping_mul(37).wrapping_add(0x80)).collect();
    init[32..40].copy_from_slice(&1.25f64.to_le_bytes());
    init[40..48].copy_from_slice(&(-2.5f64).to_le_bytes());
    let digest = |out: &[u8]| {
        out[64..].iter().fold(0xcbf29ce484222325u64, |h, &v| (h ^ u64::from(v)).wrapping_mul(0x100000001b3))
    };
    let oracle = cfg!(feature = "rosetta-oracle").then(|| rosetta(&code, &init));
    if let Some(control) = &oracle {
        eprintln!("MMX/SSE Rosetta vector digest {:#x}", digest(control));
    }
    run_tiers(&code, &init, |out| {
        if let Some(control) = &oracle {
            assert_eq!(out.iter().zip(control).position(|(a, b)| a != b), None, "first diverging byte");
        }
        assert_eq!(digest(out), 0x9075_2ad1_530b_9048); // Rosetta output for all stored lanes.
    });
}

#[test]
fn host_fpcr_survives_default_and_nondefault_guest_modes() {
    unsafe extern "win64" fn observe() -> u64 {
        read_fpcr()
    }
    const TARGET: u64 = 0x7fff_ed11_0320;
    let saved_fpcr = read_fpcr();
    let host = 1 << 22 | 1 << 24;
    crate::exe::register_native_va(TARGET, observe as *const () as usize);
    let mut a = CodeAssembler::new(64).unwrap();
    a.push(rbx).unwrap();
    a.sub(rsp, 32).unwrap();
    a.mov(rbx, rcx).unwrap();
    for i in 0..3 {
        a.ldmxcsr(dword_ptr(rbx + 4 * i)).unwrap();
        a.mov(r10, TARGET).unwrap();
        a.call(r10).unwrap();
        a.mov(qword_ptr(rbx + 16 + 8 * i), rax).unwrap();
        a.vmovss(xmm0, dword_ptr(rbx + 48)).unwrap();
        a.vmulss(xmm0, xmm0, dword_ptr(rbx + 52)).unwrap();
        a.vmovss(dword_ptr(rbx + 56 + 4 * i), xmm0).unwrap();
    }
    a.add(rsp, 32).unwrap();
    a.pop(rbx).unwrap();
    a.ret().unwrap();
    let code = a.assemble(0).unwrap();
    let mut init = [0u8; 80];
    for (i, mxcsr) in [0x1f80u32, 0x1fc0, 0x9f80].into_iter().enumerate() {
        init[4 * i..4 * i + 4].copy_from_slice(&mxcsr.to_le_bytes());
    }
    init[48..52].copy_from_slice(&1u32.to_le_bytes());
    init[52..56].copy_from_slice(&0x7e80_0000u32.to_le_bytes());
    write_fpcr(host);
    run_tiers(&code, &init, |out| {
        assert_eq!(read_fpcr(), host);
        for i in 0..3 {
            assert_eq!(u64::from_le_bytes(out[16 + 8 * i..24 + 8 * i].try_into().unwrap()), host);
            assert_eq!(
                u32::from_le_bytes(out[56 + 4 * i..60 + 4 * i].try_into().unwrap()),
                if i == 1 { 0 } else { 0x3400_0000 }
            );
        }
    });
    write_fpcr(saved_fpcr);
    crate::exe::unregister_native_va(TARGET);
}

#[test]
fn denormal_selection_estimates_and_dot_products() {
    let mut a = CodeAssembler::new(64).unwrap();
    for mode in 0..4 {
        a.ldmxcsr(dword_ptr(rcx + 4 * mode)).unwrap();
        a.movups(xmm0, xmmword_ptr(rcx + 16)).unwrap();
        a.minps(xmm0, xmmword_ptr(rcx + 32)).unwrap();
        a.movups(xmmword_ptr(rcx + 64 + 64 * mode), xmm0).unwrap();
        a.movups(xmm0, xmmword_ptr(rcx + 32)).unwrap();
        a.rcpps(xmm1, xmm0).unwrap();
        a.movups(xmmword_ptr(rcx + 80 + 64 * mode), xmm1).unwrap();
        a.rsqrtps(xmm1, xmm0).unwrap();
        a.movups(xmmword_ptr(rcx + 96 + 64 * mode), xmm1).unwrap();
        a.movups(xmm0, xmmword_ptr(rcx + 48)).unwrap();
        a.dpps(xmm0, xmm0, 0xff).unwrap();
        a.movups(xmmword_ptr(rcx + 112 + 64 * mode), xmm0).unwrap();
    }
    a.ret().unwrap();
    let code = a.assemble(0).unwrap();
    let mut init = vec![0; 320];
    let words = [
        0x1f80u32,
        0x1fc0,
        0x9f80,
        0x9fc0,
        0x3f80_0000,
        0x3f80_0000,
        0x3f80_0000,
        0x3f80_0000,
        0x807f_ffff,
        0x007f_ffff,
        0x8000_0000,
        0x0000_0000,
        0x1f80_0000,
        0x1f80_0000,
        0x1f80_0000,
        0x1f80_0000,
    ];
    for (i, v) in words.into_iter().enumerate() {
        init[4 * i..4 * i + 4].copy_from_slice(&v.to_le_bytes());
    }
    let oracle = cfg!(feature = "rosetta-oracle").then(|| rosetta(&code, &init));
    for mask in [0, features::HostFeatures::AFP] {
        features::without_features(mask, || {
            run_tiers(&code, &init, |out| {
                for mode in 0usize..4 {
                    let at = |op: usize, lane: usize| {
                        let p: usize = 64 + 64 * mode + 16 * op + 4 * lane;
                        u32::from_le_bytes(out[p..p + 4].try_into().unwrap())
                    };
                    assert_eq!(at(0, 0), if mode & 1 != 0 { 0x8000_0000 } else { 0x807f_ffff });
                    assert_eq!(at(1, 0), 0xff80_0000);
                    assert_eq!(at(2, 1), 0x7f80_0000);
                    assert_eq!(at(3, 0), if mode == 0 { 0x0080_0000 } else { 0 });
                }
                if let Some(control) = &oracle {
                    assert_eq!(&out[64..], &control[64..]);
                }
            })
        });
    }
}

#[test]
fn invalid_state_operand_resumes_from_guest_exception() {
    use std::ffi::c_void;
    use win32_rt::unwind::{Context, ExceptionRecord, STATUS_ACCESS_VIOLATION};
    unsafe extern "win64" fn resume(p: *mut [*mut c_void; 2]) -> i32 {
        let record = &*((*p)[0] as *const ExceptionRecord);
        let ctx = &mut *((*p)[1] as *mut Context);
        if record.code != STATUS_ACCESS_VIOLATION || ctx.gpr[10] != 0x3f77_0077 {
            return 0;
        }
        assert_eq!(record.address as u64, ctx.rip);
        assert_eq!(record.number_parameters, 2);
        assert_eq!(record.information[1], u64::MAX);
        ctx.rip += 3; // XGETBV
        ctx.gpr[0] = 0x5eed;
        -1
    }
    let mut a = CodeAssembler::new(64).unwrap();
    a.mov(r8, rcx).unwrap();
    a.mov(ecx, 1).unwrap();
    a.mov(r10d, 0x3f77_0077u32).unwrap();
    a.xgetbv().unwrap();
    a.mov(dword_ptr(r8), eax).unwrap();
    a.ret().unwrap();
    let code = a.assemble(0).unwrap();
    let handler = unsafe { win32_rt::unwind::AddVectoredExceptionHandler(1, resume) };
    run_tiers(&code, &[0; 8], |out| assert_eq!(&out[..4], &0x5eedu32.to_le_bytes()));
    unsafe {
        win32_rt::unwind::RemoveVectoredExceptionHandler(handler);
    }
}

#[test]
fn xstate_uses_only_requested_pages_and_preserves_raw_registers() {
    use win32_rt::kernel32::memory::{VirtualAlloc, VirtualFree, VirtualProtect};
    let saved = regfile_ext::x87_snapshot();
    let fpcr = read_fpcr();
    unsafe {
        let mapping = VirtualAlloc(std::ptr::null_mut(), 0x20000, 0x3000, 4);
        assert!(!mapping.is_null());
        let guard = mapping.cast::<u8>().add(0x10000);
        let mut old = 0;
        assert_ne!(VirtualProtect(guard.cast(), 0x10000, 1, &mut old), 0);
        let base = guard.sub(576);
        let bytes = std::slice::from_raw_parts_mut(base, 576);
        bytes[0..2].copy_from_slice(&0x037fu16.to_le_bytes());
        bytes[2..4].copy_from_slice(&0x2800u16.to_le_bytes()); // TOP = 5
        bytes[4] = 0x20; // physical slot 5, or ST(0)
        bytes[6..8].copy_from_slice(&0x123u16.to_le_bytes());
        bytes[8..16].copy_from_slice(&0x1234_5678_9000u64.to_le_bytes());
        bytes[16..24].copy_from_slice(&0x2345_6789_a000u64.to_le_bytes());
        bytes[24..28].copy_from_slice(&0x1f80u32.to_le_bytes());
        for i in 0..8 {
            let slot = 32 + 16 * i;
            bytes[slot..slot + 10].copy_from_slice(&[i as u8 + 1, 3, 4, 5, 6, 7, 8, 0x80, 0xff, 0x7f]);
            // SNaN
        }
        bytes[512] = 7;
        let ins = |b: &[u8]| Decoder::new(64, b, DecoderOptions::NONE).decode();
        let mut cpu = Cpu::new_zeroed();
        cpu.gpr[0] = 3;
        cpu.gpr[1] = base as u64;
        let restore = ins(&[0x48, 0x0f, 0xae, 0x29]);
        cpu.exec_xstate(&restore).unwrap();
        let expected = bytes[..160].to_vec();
        bytes[..160].fill(0);
        cpu.exec_xstate(&ins(&[0x48, 0x0f, 0xae, 0x01])).unwrap();
        assert_eq!(&bytes[..28], &expected[..28]);
        assert_eq!(&bytes[32..160], &expected[32..160]);
        cpu.gpr[0] = 7;
        cpu.xmm[3] = 99;
        assert!(cpu.exec_xstate(&restore).is_err()); // selected YMM bytes unreadable
        assert_eq!(cpu.xmm[3], 99);
        bytes[512] = 3;
        cpu.exec_xstate(&restore).unwrap(); // init YMM without reading its page
        assert_eq!(cpu.ymm_hi, [0; 16]);
        assert_ne!(VirtualFree(mapping, 0, 0x8000), 0);
    }
    regfile_ext::x87_restore(saved);
    write_fpcr(fpcr);
}

#[test]
fn optional_unaligned_atomics_fall_back_across_tiers() {
    let mut a = CodeAssembler::new(64).unwrap();
    a.mov(eax, 3).unwrap();
    a.lock().xadd(dword_ptr(rcx + 1), eax).unwrap();
    a.mov(dword_ptr(rcx + 32), eax).unwrap();
    a.mov(rdx, 7u64).unwrap();
    a.lock().xadd(qword_ptr(rcx + 13), rdx).unwrap();
    a.mov(qword_ptr(rcx + 40), rdx).unwrap();
    a.ret().unwrap();
    let code = a.assemble(0).unwrap();
    let mut init = [0u8; 48];
    init[1..5].copy_from_slice(&11u32.to_le_bytes());
    init[13..21].copy_from_slice(&13u64.to_le_bytes());
    for disable in [0, features::HostFeatures::LSE2 | features::HostFeatures::LRCPC2] {
        features::without_features(disable, || {
            run_tiers(&code, &init, |out| {
                assert_eq!(&out[1..5], &14u32.to_le_bytes());
                assert_eq!(&out[13..21], &20u64.to_le_bytes());
                assert_eq!(&out[32..36], &11u32.to_le_bytes());
                assert_eq!(&out[40..48], &13u64.to_le_bytes());
            })
        });
    }
}

#[test]
fn double_precision_denormals_and_scalar_merge_lanes() {
    let mut a = CodeAssembler::new(64).unwrap();
    for mode in 0..4 {
        a.ldmxcsr(dword_ptr(rcx + 4 * mode)).unwrap();
        a.vmovupd(ymm0, ymmword_ptr(rcx + 32)).unwrap();
        a.vmulpd(ymm0, ymm0, ymmword_ptr(rcx + 64)).unwrap();
        a.vmovupd(ymmword_ptr(rcx + 96 + 48 * mode), ymm0).unwrap();
        a.vmovupd(xmm1, xmmword_ptr(rcx + 32)).unwrap();
        a.vcvtsd2ss(xmm1, xmm1, xmm1).unwrap();
        a.vmovups(xmmword_ptr(rcx + 128 + 48 * mode), xmm1).unwrap();
    }
    a.vzeroupper().unwrap();
    a.ret().unwrap();
    let code = a.assemble(0).unwrap();
    let mut init = vec![0; 288];
    for (i, v) in [0x1f80u32, 0x1fc0, 0x9f80, 0x9fc0].into_iter().enumerate() {
        init[4 * i..4 * i + 4].copy_from_slice(&v.to_le_bytes());
    }
    let lhs = [0x8000_0001_0000_0000u64, 0x0010_0000_0000_0000, 0x8010_0000_0000_0000, 1];
    let rhs = [0x7fd0_0000_0000_0000u64, 0x3fe0_0000_0000_0000, 0x3fef_ffff_ffff_ffff, 0x7fd0_0000_0000_0000];
    for i in 0..4 {
        init[32 + 8 * i..40 + 8 * i].copy_from_slice(&lhs[i].to_le_bytes());
        init[64 + 8 * i..72 + 8 * i].copy_from_slice(&rhs[i].to_le_bytes());
    }
    let oracle = cfg!(feature = "rosetta-oracle").then(|| rosetta(&code, &init));
    for mask in [0, features::HostFeatures::AFP] {
        features::without_features(mask, || {
            run_tiers(&code, &init, |out| {
                for mode in 0usize..4 {
                    let base = 96 + 48 * mode;
                    let expected = [
                        if mode & 1 == 0 { 0xbeb0_0000_0000_0000 } else { 1 << 63 },
                        if mode & 2 == 0 { 0x0008_0000_0000_0000 } else { 0 },
                        if mode & 2 == 0 { 0x8010_0000_0000_0000 } else { 1 << 63 },
                        if mode & 1 == 0 { 0x3cb0_0000_0000_0000 } else { 0 },
                    ];
                    for (i, word) in expected.into_iter().enumerate() {
                        assert_eq!(
                            u64::from_le_bytes(out[base + 8 * i..base + 8 * i + 8].try_into().unwrap()),
                            word,
                            "mode {mode} lane {i}"
                        );
                    }
                    // Scalar conversion writes 32 bits; bits 127:32 come verbatim from VEX src1.
                    assert_eq!(&out[base + 36..base + 48], &init[36..48]);
                }
                if let Some(control) = &oracle {
                    assert_eq!(out, control);
                }
            })
        });
    }
}

#[test]
fn avx_vpermilpd_round_mxcsr_and_pcmpstrm_across_tiers() {
    let mut a = CodeAssembler::new(64).unwrap();
    // 1. VPERMILPD (128/256 imm8 & var, reg & mem), verifying bit-1 control and 128-bit upper zeroing.
    a.vmovupd(ymm0, ymmword_ptr(rcx)).unwrap(); // [10.0, 20.0, 30.0, 40.0]
    a.vmovdqu(ymm1, ymmword_ptr(rcx + 32)).unwrap(); // ctrl = [1, 2, 3, 0] (bit 1 -> [0, 1, 1, 0])
    a.vmovupd(ymm2, ymm0).unwrap();
    a.vpermilpd(xmm2, xmm0, 0b01).unwrap(); // lo=[20, 10], upper 128 zeroed
    a.vmovupd(ymmword_ptr(rcx + 64), ymm2).unwrap();
    a.vmovupd(ymm2, ymm0).unwrap();
    a.vpermilpd(xmm2, xmmword_ptr(rcx), 0b11).unwrap(); // lo=[20, 20], upper 128 zeroed
    a.vmovupd(ymmword_ptr(rcx + 96), ymm2).unwrap();
    a.vpermilpd(ymm2, ymm0, 0b0110).unwrap(); // lo=[10, 20], hi=[40, 30]
    a.vmovupd(ymmword_ptr(rcx + 128), ymm2).unwrap();
    a.vpermilpd(ymm2, ymmword_ptr(rcx), 0b1001).unwrap(); // lo=[20, 10], hi=[30, 40]
    a.vmovupd(ymmword_ptr(rcx + 160), ymm2).unwrap();
    a.vmovupd(ymm2, ymm0).unwrap();
    a.vpermilpd(xmm2, xmm0, xmm1).unwrap(); // bit1 of [1,2]=[0,1] -> [10,20], upper 128 zeroed
    a.vmovupd(ymmword_ptr(rcx + 192), ymm2).unwrap();
    a.vpermilpd(ymm2, ymm0, ymm1).unwrap(); // bit1 of [1,2,3,0]=[0,1,1,0] -> [10,20,40,30]
    a.vmovupd(ymmword_ptr(rcx + 224), ymm2).unwrap();
    a.vpermilpd(ymm2, ymm0, ymmword_ptr(rcx + 32)).unwrap(); // mem ctrl
    a.vmovupd(ymmword_ptr(rcx + 256), ymm2).unwrap();

    // 2. (V)ROUND{SS,SD,PS,PD} with imm8[2]=1 (use MXCSR.RC) across all 4 rounding modes.
    for mode in 0..4 {
        let base = 320 + mode * 80;
        a.ldmxcsr(dword_ptr(rcx + 288 + mode * 4)).unwrap();
        a.vmovups(ymm3, ymmword_ptr(rcx + 640)).unwrap(); // 8x f32: 1.5, -1.5, 1.25, -1.25, 2.5, -2.5, 0.25, -0.25
        a.vroundps(ymm4, ymm3, 4).unwrap();
        a.vmovups(ymmword_ptr(rcx + base), ymm4).unwrap();
        a.vmovupd(ymm5, ymmword_ptr(rcx + 672)).unwrap(); // 4x f64: 1.5, -1.5, 2.25, -2.25
        a.vroundpd(ymm6, ymm5, 4).unwrap();
        a.vmovupd(ymmword_ptr(rcx + base + 32), ymm6).unwrap();
        a.vroundss(xmm7, xmm3, xmm3, 4).unwrap();
        a.vmovss(dword_ptr(rcx + base + 64), xmm7).unwrap();
        a.vroundsd(xmm7, xmm5, xmm5, 4).unwrap();
        a.vmovsd(qword_ptr(rcx + base + 72), xmm7).unwrap();
    }
    a.ldmxcsr(dword_ptr(rcx + 288)).unwrap(); // restore RN

    // 3. Legacy PCMPISTRM preserves YMM0[255:128], whereas VPCMPISTRM zeroes YMM0[255:128].
    a.vmovdqu(ymm0, ymmword_ptr(rcx + 704)).unwrap(); // upper half non-zero sentinel
    a.vmovdqu(xmm1, xmmword_ptr(rcx + 736)).unwrap();
    a.pcmpistrm(xmm0, xmm1, 0x08).unwrap(); // Equal Each, bitmask in xmm0; ymm0[255:128] preserved!
    a.vmovdqu(ymmword_ptr(rcx + 768), ymm0).unwrap();
    a.vmovdqu(ymm0, ymmword_ptr(rcx + 704)).unwrap();
    a.vpcmpistrm(xmm0, xmm1, 0x08).unwrap(); // VEX.128: ymm0[255:128] zeroed!
    a.vmovdqu(ymmword_ptr(rcx + 800), ymm0).unwrap();

    a.vzeroupper().unwrap();
    a.ret().unwrap();
    let code = a.assemble(0).unwrap();

    let mut init = vec![0u8; 832];
    for (i, v) in [10.0f64, 20.0, 30.0, 40.0].into_iter().enumerate() {
        init[i * 8..i * 8 + 8].copy_from_slice(&v.to_bits().to_le_bytes());
    }
    for (i, v) in [1u64, 2, 3, 0].into_iter().enumerate() {
        init[32 + i * 8..40 + i * 8].copy_from_slice(&v.to_le_bytes());
    }
    for (i, rc) in [0u32, 1, 2, 3].into_iter().enumerate() {
        let mx = 0x1f80 | (rc << 13);
        init[288 + i * 4..292 + i * 4].copy_from_slice(&mx.to_le_bytes());
    }
    for (i, v) in [1.5f32, -1.5, 1.25, -1.25, 2.5, -2.5, 0.25, -0.25].into_iter().enumerate() {
        init[640 + i * 4..644 + i * 4].copy_from_slice(&v.to_bits().to_le_bytes());
    }
    for (i, v) in [1.5f64, -1.5, 2.25, -2.25].into_iter().enumerate() {
        init[672 + i * 8..680 + i * 8].copy_from_slice(&v.to_bits().to_le_bytes());
    }
    init[704..720].copy_from_slice(b"hello_world\0\0\0\0\0");
    init[720..736].copy_from_slice(&0xDEAD_BEEF_CAFE_BABEu64.to_le_bytes().repeat(2));
    init[736..752].copy_from_slice(b"hello_there\0\0\0\0\0");

    let oracle = cfg!(feature = "rosetta-oracle").then(|| rosetta(&code, &init));
    run_tiers(&code, &init, |out| {
        let f64_at = |off: usize| f64::from_bits(u64::from_le_bytes(out[off..off + 8].try_into().unwrap()));
        let f32_at = |off: usize| f32::from_bits(u32::from_le_bytes(out[off..off + 4].try_into().unwrap()));
        assert_eq!([f64_at(64), f64_at(72), f64_at(80), f64_at(88)], [20.0, 10.0, 0.0, 0.0]);
        assert_eq!([f64_at(96), f64_at(104), f64_at(112), f64_at(120)], [20.0, 20.0, 0.0, 0.0]);
        assert_eq!([f64_at(128), f64_at(136), f64_at(144), f64_at(152)], [10.0, 20.0, 40.0, 30.0]);
        assert_eq!([f64_at(160), f64_at(168), f64_at(176), f64_at(184)], [20.0, 10.0, 30.0, 40.0]);
        assert_eq!([f64_at(192), f64_at(200), f64_at(208), f64_at(216)], [10.0, 20.0, 0.0, 0.0]);
        assert_eq!([f64_at(224), f64_at(232), f64_at(240), f64_at(248)], [10.0, 20.0, 40.0, 30.0]);
        assert_eq!([f64_at(256), f64_at(264), f64_at(272), f64_at(280)], [10.0, 20.0, 40.0, 30.0]);
        // Check RD (mode=1) and RU (mode=2) and RZ (mode=3) for vroundps/pd/ss/sd imm=4
        assert_eq!([f32_at(320 + 80), f32_at(320 + 80 + 4), f32_at(320 + 80 + 8), f32_at(320 + 80 + 12)], [1.0, -2.0, 1.0, -2.0]);
        assert_eq!([f32_at(320 + 160), f32_at(320 + 160 + 4), f32_at(320 + 160 + 8), f32_at(320 + 160 + 12)], [2.0, -1.0, 2.0, -1.0]);
        assert_eq!([f32_at(320 + 240), f32_at(320 + 240 + 4), f32_at(320 + 240 + 8), f32_at(320 + 240 + 12)], [1.0, -1.0, 1.0, -1.0]);
        assert_eq!(f32_at(320 + 80 + 64), 1.0);
        assert_eq!(f64_at(320 + 160 + 72), 2.0);
        // Legacy PCMPISTRM preserves YMM0[255:128], VEX VPCMPISTRM zeroes it.
        assert_eq!(&out[784..800], &init[720..736]);
        assert_eq!(&out[816..832], &[0u8; 16]);
        if let Some(control) = &oracle {
            assert_eq!(out, control);
        }
    });
}

#[test]
fn f16c_rounding_modes_denormals_and_nans_across_tiers() {
    let mut a = CodeAssembler::new(64).unwrap();
    // 1. VCVTPH2PS (128 & 256, reg & mem): sNaN quieting, subnormals, upper zeroing.
    a.vmovdqu(ymm0, ymmword_ptr(rcx)).unwrap(); // sentinel upper half
    a.vcvtph2ps(xmm0, qword_ptr(rcx)).unwrap(); // 128-bit mem -> zeroes ymm0[255:128]
    a.vmovups(ymmword_ptr(rcx + 64), ymm0).unwrap();
    a.vmovdqu(xmm1, xmmword_ptr(rcx)).unwrap();
    a.vcvtph2ps(ymm2, xmm1).unwrap(); // 256-bit reg
    a.vmovups(ymmword_ptr(rcx + 96), ymm2).unwrap();

    // 2. VCVTPS2PH across imm=0,1,2,3,4 under MXCSR.RC=RD(1), RU(2), RZ(3), and DAZ/FTZ combos,
    //    also verifying RFLAGS preservation across imm8 override in JIT.
    a.vmovups(ymm3, ymmword_ptr(rcx + 128)).unwrap(); // fractional + overflow + min-normal + sNaN
    a.vmovups(ymm4, ymmword_ptr(rcx + 160)).unwrap(); // subnormals (f32 + f16)
    // With MXCSR.RC = RD (0x3f80): test imm=0 (RN overrides MXCSR), imm=2 (RU), imm=3 (RZ), imm=4 (MXCSR=RD).
    a.ldmxcsr(dword_ptr(rcx + 32)).unwrap();
    a.mov(eax, 1).unwrap();
    a.sub(eax, 2).unwrap(); // sets CF=1, SF=1, ZF=0, OF=0
    a.vcvtps2ph(xmm5, ymm3, 0).unwrap();
    a.setb(byte_ptr(rcx + 56)).unwrap(); // CF must survive vcvtps2ph
    a.sets(byte_ptr(rcx + 57)).unwrap(); // SF must survive vcvtps2ph
    a.vmovdqu(xmmword_ptr(rcx + 192), xmm5).unwrap();
    a.vcvtps2ph(xmmword_ptr(rcx + 208), ymm3, 1).unwrap();
    a.vcvtps2ph(xmmword_ptr(rcx + 224), ymm3, 2).unwrap();
    a.vcvtps2ph(xmmword_ptr(rcx + 240), ymm3, 3).unwrap();
    a.vcvtps2ph(xmmword_ptr(rcx + 256), ymm3, 4).unwrap();
    // 128-bit reg form zeroes upper 128 bits of dest
    a.vmovdqu(ymm6, ymmword_ptr(rcx + 128)).unwrap();
    a.vcvtps2ph(xmm6, xmm3, 2).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 272), ymm6).unwrap();

    // 3. Test DAZ vs FTZ on subnormals under RU (RC=2): 4 modes = (none, DAZ, FTZ, both).
    for m in 0..4 {
        a.ldmxcsr(dword_ptr(rcx + 36 + m * 4)).unwrap();
        a.vcvtps2ph(xmmword_ptr(rcx + 304 + m * 16), ymm4, 4).unwrap();
        a.vcvtph2ps(ymm7, xmmword_ptr(rcx)).unwrap();
        a.vmovups(ymmword_ptr(rcx + 368 + m * 32), ymm7).unwrap();
    }
    a.ldmxcsr(dword_ptr(rcx + 52)).unwrap(); // restore default 0x1f80
    a.vzeroupper().unwrap();
    a.ret().unwrap();
    let code = a.assemble(0).unwrap();

    let mut init = vec![0u8; 512];
    // [0..16]: 8x f16 inputs: +sNaN, -sNaN, +qNaN, +min_sub, -min_sub, +min_norm, +0, -0
    for (i, h) in [0x7c01u16, 0xfc01, 0x7e01, 0x0001, 0x8001, 0x0400, 0x0000, 0x8000].into_iter().enumerate() {
        init[i * 2..i * 2 + 2].copy_from_slice(&h.to_le_bytes());
    }
    init[16..32].fill(0xAA); // sentinel for upper 128 bits
    init[32..36].copy_from_slice(&(0x1f80u32 | (1 << 13)).to_le_bytes()); // RD
    for (i, mx) in [0x5f80u32, 0x5fc0, 0xdf80, 0xdfc0].into_iter().enumerate() {
        init[36 + i * 4..40 + i * 4].copy_from_slice(&mx.to_le_bytes());
    }
    init[52..56].copy_from_slice(&0x1f80u32.to_le_bytes());
    // [128..160]: 8x f32: 1.5, 1+2^-12, -(1+2^-12), 65536, -65536, 65520, 2^-14-eps, +sNaN
    for (i, f) in [
        0x3fc0_0000u32, 0x3f80_0800, 0xbf80_0800, 0x4780_0000,
        0xc780_0000, 0x477f_f000, 0x387f_f000, 0x7f80_0001,
    ]
    .into_iter()
    .enumerate()
    {
        init[128 + i * 4..132 + i * 4].copy_from_slice(&f.to_le_bytes());
    }
    // [160..192]: 8x f32 subnormals/tiny
    for (i, f) in [
        0x0000_0001u32, 0x8000_0001, 0x3800_0000, 0xb800_0000,
        0x3380_0000, 0x3300_0000, 0x387f_e000, 0x387f_c000,
    ]
    .into_iter()
    .enumerate()
    {
        init[160 + i * 4..164 + i * 4].copy_from_slice(&f.to_le_bytes());
    }

    let oracle = cfg!(feature = "rosetta-oracle").then(|| rosetta(&code, &init));
    for mask in [0, features::HostFeatures::AFP] {
        features::without_features(mask, || {
            run_tiers(&code, &init, |out| {
                let u32_at = |off: usize| u32::from_le_bytes(out[off..off + 4].try_into().unwrap());
                let u16_at = |off: usize| u16::from_le_bytes(out[off..off + 2].try_into().unwrap());
                // sNaN quieted and 128-bit upper half zeroed
                assert_eq!(u32_at(64), 0x7fc0_2000);
                assert_eq!(u32_at(68), 0xffc0_2000);
                assert_eq!(&out[80..96], &[0u8; 16]);
                // RFLAGS preserved
                assert_eq!(out[56], 1);
                assert_eq!(out[57], 1);
                // imm=0 (RN) overrides MXCSR.RC=RD: 1.5 -> 0x3e00, 1+2^-12 -> 0x3c00, 65536 -> +Inf (0x7c00)
                assert_eq!([u16_at(192), u16_at(194), u16_at(196), u16_at(198)], [0x3e00, 0x3c00, 0xbc00, 0x7c00]);
                // imm=1 (RD): -(1+2^-12) -> 0xbc01, 65536 -> +65504 (0x7bff)
                assert_eq!([u16_at(208), u16_at(210), u16_at(212), u16_at(214)], [0x3e00, 0x3c00, 0xbc01, 0x7bff]);
                // imm=2 (RU): 1+2^-12 -> 0x3c01, -65536 -> -65504 (0xfbff)
                assert_eq!([u16_at(224), u16_at(226), u16_at(228), u16_at(232)], [0x3e00, 0x3c01, 0xbc00, 0xfbff]);
                // imm=3 (RZ): 65536 -> 0x7bff, -65536 -> 0xfbff
                assert_eq!([u16_at(246), u16_at(248)], [0x7bff, 0xfbff]);
                // imm=4 (MXCSR.RC=RD) matches imm=1
                assert_eq!(&out[256..272], &out[208..224]);
                // 128-bit reg form zeroed upper 128 bits
                assert_eq!(&out[288..304], &[0u8; 16]);
                // Subnormals under RU: +2^-149 -> 0x0001 when DAZ=0 (modes 0,2), 0x0000 when DAZ=1 (modes 1,3)
                assert_eq!(u16_at(304), 0x0001);
                assert_eq!(u16_at(320), 0x0000);
                assert_eq!(u16_at(336), 0x0001);
                assert_eq!(u16_at(352), 0x0000);
                if let Some(control) = &oracle {
                    assert_eq!(out, control);
                }
            });
        });
    }
}

#[test]
fn fma3_all_families_128_and_256_across_tiers() {
    let mut a = CodeAssembler::new(64).unwrap();
    // Load 256-bit f32 inputs: ymm0=a, ymm1=b, ymm2=c
    a.vmovups(ymm0, ymmword_ptr(rcx)).unwrap();
    a.vmovups(ymm1, ymmword_ptr(rcx + 32)).unwrap();
    a.vmovups(ymm2, ymmword_ptr(rcx + 64)).unwrap();
    // 256-bit PS: all 6 families across 132/213/231 (mix reg and mem forms)
    a.vmovups(ymm3, ymm0).unwrap();
    a.vfmadd132ps(ymm3, ymm2, ymm1).unwrap();
    a.vmovups(ymmword_ptr(rcx + 192), ymm3).unwrap();
    a.vmovups(ymm3, ymm0).unwrap();
    a.vfmsub213ps(ymm3, ymm1, ymmword_ptr(rcx + 64)).unwrap();
    a.vmovups(ymmword_ptr(rcx + 224), ymm3).unwrap();
    a.vmovups(ymm3, ymm2).unwrap();
    a.vfnmadd231ps(ymm3, ymm0, ymm1).unwrap();
    a.vmovups(ymmword_ptr(rcx + 256), ymm3).unwrap();
    a.vmovups(ymm3, ymm0).unwrap();
    a.vfnmsub132ps(ymm3, ymm2, ymmword_ptr(rcx + 32)).unwrap();
    a.vmovups(ymmword_ptr(rcx + 288), ymm3).unwrap();
    a.vmovups(ymm3, ymm0).unwrap();
    a.vfmaddsub213ps(ymm3, ymm1, ymm2).unwrap();
    a.vmovups(ymmword_ptr(rcx + 320), ymm3).unwrap();
    a.vmovups(ymm3, ymm2).unwrap();
    a.vfmsubadd231ps(ymm3, ymm0, ymm1).unwrap();
    a.vmovups(ymmword_ptr(rcx + 352), ymm3).unwrap();

    // 256-bit PD: all 6 families
    a.vmovupd(ymm4, ymmword_ptr(rcx + 96)).unwrap();
    a.vmovupd(ymm5, ymmword_ptr(rcx + 128)).unwrap();
    a.vmovupd(ymm6, ymmword_ptr(rcx + 160)).unwrap();
    a.vmovupd(ymm7, ymm4).unwrap();
    a.vfmadd213pd(ymm7, ymm5, ymm6).unwrap();
    a.vmovupd(ymmword_ptr(rcx + 384), ymm7).unwrap();
    a.vmovupd(ymm7, ymm6).unwrap();
    a.vfmsub231pd(ymm7, ymm4, ymmword_ptr(rcx + 128)).unwrap();
    a.vmovupd(ymmword_ptr(rcx + 416), ymm7).unwrap();
    a.vmovupd(ymm7, ymm4).unwrap();
    a.vfnmadd132pd(ymm7, ymm6, ymm5).unwrap();
    a.vmovupd(ymmword_ptr(rcx + 448), ymm7).unwrap();
    a.vmovupd(ymm7, ymm4).unwrap();
    a.vfnmsub213pd(ymm7, ymm5, ymm6).unwrap();
    a.vmovupd(ymmword_ptr(rcx + 480), ymm7).unwrap();
    a.vmovupd(ymm7, ymm4).unwrap();
    a.vfmaddsub132pd(ymm7, ymm6, ymm5).unwrap();
    a.vmovupd(ymmword_ptr(rcx + 512), ymm7).unwrap();
    a.vmovupd(ymm7, ymm4).unwrap();
    a.vfmsubadd213pd(ymm7, ymm5, ymm6).unwrap();
    a.vmovupd(ymmword_ptr(rcx + 544), ymm7).unwrap();

    // Scalar SS & SD: preserve dest[127:32]/[127:64] and zero dest[255:128], single-rounding exactness
    a.vmovups(ymm3, ymm0).unwrap();
    a.vfmsub213ss(xmm3, xmm1, xmm2).unwrap(); // lane 0: (1+2^-23)*(1-2^-23)-1 = -2^-46 (proves true FMA)
    a.vmovups(ymmword_ptr(rcx + 576), ymm3).unwrap();
    a.vmovupd(ymm7, ymm4).unwrap();
    a.vfnmadd231sd(xmm7, xmm5, xmm6).unwrap();
    a.vmovupd(ymmword_ptr(rcx + 608), ymm7).unwrap();

    a.vzeroupper().unwrap();
    a.ret().unwrap();
    let code = a.assemble(0).unwrap();

    let mut init = vec![0u8; 640];
    let p_a: [f32; 8] = [1.0 + f32::from_bits(0x3400_0000), -2.5, 3.25, 0.0, 4.0, -0.5, 1.75, 2.0];
    let p_b: [f32; 8] = [1.0 - f32::from_bits(0x3400_0000), 4.0, -2.0, -0.0, 0.25, 8.0, -3.0, 5.0];
    let p_c: [f32; 8] = [1.0, 10.0, 20.0, 0.0, -1.0, 2.0, 3.0, -4.0];
    for i in 0..8 {
        init[i * 4..i * 4 + 4].copy_from_slice(&p_a[i].to_bits().to_le_bytes());
        init[32 + i * 4..36 + i * 4].copy_from_slice(&p_b[i].to_bits().to_le_bytes());
        init[64 + i * 4..68 + i * 4].copy_from_slice(&p_c[i].to_bits().to_le_bytes());
    }
    let d_a: [f64; 4] = [1.0 + f64::from_bits(0x3ca0_0000_0000_0000), -3.5, 2.25, 0.0];
    let d_b: [f64; 4] = [1.0 - f64::from_bits(0x3ca0_0000_0000_0000), 2.0, -4.0, -0.0];
    let d_c: [f64; 4] = [1.0, 7.0, 11.0, 0.0];
    for i in 0..4 {
        init[96 + i * 8..104 + i * 8].copy_from_slice(&d_a[i].to_bits().to_le_bytes());
        init[128 + i * 8..136 + i * 8].copy_from_slice(&d_b[i].to_bits().to_le_bytes());
        init[160 + i * 8..168 + i * 8].copy_from_slice(&d_c[i].to_bits().to_le_bytes());
    }

    let oracle = cfg!(feature = "rosetta-oracle").then(|| rosetta(&code, &init));
    run_tiers(&code, &init, |out| {
        let f32_at = |off: usize| f32::from_bits(u32::from_le_bytes(out[off..off + 4].try_into().unwrap()));
        // True single-rounding FMA: (1 + 2^-23)*(1 - 2^-23) - 1 = -2^-46 (unfused mul+sub would give 0.0!)
        assert_eq!(f32_at(224), -f32::from_bits(0x2880_0000));
        assert_eq!(f32_at(576), -f32::from_bits(0x2880_0000));
        // Scalar SS preserves bits [127:32] and zeroes [255:128]
        assert_eq!(&out[580..592], &init[4..16]);
        assert_eq!(&out[592..608], &[0u8; 16]);
        // Scalar SD preserves bits [127:64] and zeroes [255:128]
        assert_eq!(&out[616..624], &init[104..112]);
        assert_eq!(&out[624..640], &[0u8; 16]);
        if let Some(control) = &oracle {
            assert_eq!(out, control);
        }
    });
}

#[test]
fn avx2_gathers_all_16_forms_faults_and_aliasing() {
    let mut a = CodeAssembler::new(64).unwrap();
    // Base pointer to table of 8 u64s (also read as 16 u32s) at [rcx + 64], centered at [rcx + 96] (+32) so negative indices work.
    a.lea(rdx, ptr(rcx + 96)).unwrap();
    a.xor(r8, r8).unwrap(); // null base for fault-suppression tests

    // Load common index & mask vectors:
    // ymm1 = 8x i32 indices: [-2, 0, 999999, 3, -1, 777777, 2, 1]
    // ymm2 = 4x i64 indices: [-1, 999999999, 2, 0]
    a.vmovdqu(ymm1, ymmword_ptr(rcx)).unwrap();
    a.vmovdqu(ymm2, ymmword_ptr(rcx + 32)).unwrap();

    // Helper macro-like sequence: 16 gather encodings writing (dst_ymm, mask_ymm) = 64 bytes per test starting at rcx+256.
    // 0: vpgatherdd xmm (128)
    a.vmovdqu(ymm0, ymmword_ptr(rcx + 128)).unwrap();
    a.vmovdqu(ymm3, ymmword_ptr(rcx + 160)).unwrap();
    a.vpgatherdd_3(xmm0, dword_ptr(rdx + xmm1 * 4), xmm3).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 256), ymm0).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 288), ymm3).unwrap();
    // 1: vpgatherdd ymm (256)
    a.vmovdqu(ymm0, ymmword_ptr(rcx + 128)).unwrap();
    a.vmovdqu(ymm3, ymmword_ptr(rcx + 160)).unwrap();
    a.vpgatherdd_3(ymm0, dword_ptr(rdx + ymm1 * 4), ymm3).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 320), ymm0).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 352), ymm3).unwrap();
    // 2: vgatherdps xmm (128)
    a.vmovdqu(ymm0, ymmword_ptr(rcx + 128)).unwrap();
    a.vmovdqu(ymm3, ymmword_ptr(rcx + 160)).unwrap();
    a.vgatherdps_3(xmm0, dword_ptr(rdx + xmm1 * 4), xmm3).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 384), ymm0).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 416), ymm3).unwrap();
    // 3: vgatherdps ymm (256)
    a.vmovdqu(ymm0, ymmword_ptr(rcx + 128)).unwrap();
    a.vmovdqu(ymm3, ymmword_ptr(rcx + 160)).unwrap();
    a.vgatherdps_3(ymm0, dword_ptr(rdx + ymm1 * 4), ymm3).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 448), ymm0).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 480), ymm3).unwrap();
    // 4: vpgatherdq xmm (128)
    a.vmovdqu(ymm0, ymmword_ptr(rcx + 128)).unwrap();
    a.vmovdqu(ymm3, ymmword_ptr(rcx + 192)).unwrap();
    a.vpgatherdq_3(xmm0, qword_ptr(rdx + xmm1 * 8), xmm3).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 512), ymm0).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 544), ymm3).unwrap();
    // 5: vpgatherdq ymm (256)
    a.vmovdqu(ymm0, ymmword_ptr(rcx + 128)).unwrap();
    a.vmovdqu(ymm3, ymmword_ptr(rcx + 192)).unwrap();
    a.vpgatherdq_3(ymm0, qword_ptr(rdx + xmm1 * 8), ymm3).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 576), ymm0).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 608), ymm3).unwrap();
    // 6: vgatherdpd xmm (128)
    a.vmovdqu(ymm0, ymmword_ptr(rcx + 128)).unwrap();
    a.vmovdqu(ymm3, ymmword_ptr(rcx + 192)).unwrap();
    a.vgatherdpd_3(xmm0, qword_ptr(rdx + xmm1 * 8), xmm3).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 640), ymm0).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 672), ymm3).unwrap();
    // 7: vgatherdpd ymm (256)
    a.vmovdqu(ymm0, ymmword_ptr(rcx + 128)).unwrap();
    a.vmovdqu(ymm3, ymmword_ptr(rcx + 192)).unwrap();
    a.vgatherdpd_3(ymm0, qword_ptr(rdx + xmm1 * 8), ymm3).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 704), ymm0).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 736), ymm3).unwrap();
    // 8: vpgatherqd xmm, vm64x (128): dest[127:64] & [255:128] zeroed!
    a.vmovdqu(ymm0, ymmword_ptr(rcx + 128)).unwrap();
    a.vmovdqu(ymm3, ymmword_ptr(rcx + 224)).unwrap();
    a.vpgatherqd_3(xmm0, dword_ptr(rdx + xmm2 * 4), xmm3).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 768), ymm0).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 800), ymm3).unwrap();
    // 9: vpgatherqd xmm, vm64y (256): 4 lanes into xmm0, dest[255:128] zeroed!
    a.vmovdqu(ymm0, ymmword_ptr(rcx + 128)).unwrap();
    a.vmovdqu(ymm3, ymmword_ptr(rcx + 224)).unwrap();
    a.vpgatherqd_3(xmm0, dword_ptr(rdx + ymm2 * 4), xmm3).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 832), ymm0).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 864), ymm3).unwrap();
    // 10: vgatherqps xmm, vm64x (128) with ALL-ZERO mask and NULL base (fault suppression + upper zeroing!)
    a.vmovdqu(ymm0, ymmword_ptr(rcx + 128)).unwrap();
    a.vpxor(ymm3, ymm3, ymm3).unwrap();
    a.vgatherqps_3(xmm0, dword_ptr(r8 + xmm2 * 4), xmm3).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 896), ymm0).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 928), ymm3).unwrap();
    // 11: vgatherqps xmm, vm64y (256)
    a.vmovdqu(ymm0, ymmword_ptr(rcx + 128)).unwrap();
    a.vmovdqu(ymm3, ymmword_ptr(rcx + 224)).unwrap();
    a.vgatherqps_3(xmm0, dword_ptr(rdx + ymm2 * 4), xmm3).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 960), ymm0).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 992), ymm3).unwrap();
    // 12: vpgatherqq xmm (128)
    a.vmovdqu(ymm0, ymmword_ptr(rcx + 128)).unwrap();
    a.vmovdqu(ymm3, ymmword_ptr(rcx + 192)).unwrap();
    a.vpgatherqq_3(xmm0, qword_ptr(rdx + xmm2 * 8), xmm3).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 1024), ymm0).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 1056), ymm3).unwrap();
    // 13: vpgatherqq ymm (256)
    a.vmovdqu(ymm0, ymmword_ptr(rcx + 128)).unwrap();
    a.vmovdqu(ymm3, ymmword_ptr(rcx + 192)).unwrap();
    a.vpgatherqq_3(ymm0, qword_ptr(rdx + ymm2 * 8), ymm3).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 1088), ymm0).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 1120), ymm3).unwrap();
    // 14: vgatherqpd xmm (128)
    a.vmovdqu(ymm0, ymmword_ptr(rcx + 128)).unwrap();
    a.vmovdqu(ymm3, ymmword_ptr(rcx + 192)).unwrap();
    a.vgatherqpd_3(xmm0, qword_ptr(rdx + xmm2 * 8), xmm3).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 1152), ymm0).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 1184), ymm3).unwrap();
    // 15: vgatherqpd ymm (256)
    a.vmovdqu(ymm0, ymmword_ptr(rcx + 128)).unwrap();
    a.vmovdqu(ymm3, ymmword_ptr(rcx + 192)).unwrap();
    a.vgatherqpd_3(ymm0, qword_ptr(rdx + ymm2 * 8), ymm3).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 1216), ymm0).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 1248), ymm3).unwrap();

    a.vzeroupper().unwrap();
    a.ret().unwrap();
    let code = a.assemble(0).unwrap();

    let mut init = vec![0u8; 1280];
    // [0..32]: 8x i32 indices (lanes 2 and 5 are huge/unmapped and masked off!)
    for (i, v) in [-2i32, 0, 999_999, 3, -1, 777_777, 2, 1].into_iter().enumerate() {
        init[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes());
    }
    // [32..64]: 4x i64 indices (lane 1 is huge/unmapped and masked off!)
    for (i, v) in [-1i64, 999_999_999, 2, 0].into_iter().enumerate() {
        init[32 + i * 8..40 + i * 8].copy_from_slice(&v.to_le_bytes());
    }
    // [64..128]: 16x u32 data table (rdx points at offset 96, i.e., u32 index 8 / u64 index 4)
    for i in 0u32..16 {
        let v = 0x1000_0000 + (i + 1) * 0x0101_0101;
        init[64 + (i as usize) * 4..68 + (i as usize) * 4].copy_from_slice(&v.to_le_bytes());
    }
    // [128..160]: initial dst sentinel
    for i in 0u32..8 {
        let v = 0xAAAA_0000 + i;
        init[128 + (i as usize) * 4..132 + (i as usize) * 4].copy_from_slice(&v.to_le_bytes());
    }
    // [160..192]: 32-bit mask (lanes 0,1,3,4,6,7 active; lanes 2,5 masked off)
    for (i, v) in [0x8000_0000u32, 0x8000_0001, 0x7fff_ffff, 0x8000_0000, 0x8000_0000, 0, 0x8000_0000, 0x8000_0000]
        .into_iter()
        .enumerate()
    {
        init[160 + i * 4..164 + i * 4].copy_from_slice(&v.to_le_bytes());
    }
    // [192..224]: 64-bit mask (lanes 0, 3 active; lanes 1, 2 masked off so idx32[2]=999999 & idx64[1]=999999999 suppressed!)
    for (i, v) in [0x8000_0000_0000_0000u64, 0x7fff_ffff_ffff_ffff, 0, 0x8000_0000_0000_0000]
        .into_iter()
        .enumerate()
    {
        init[192 + i * 8..200 + i * 8].copy_from_slice(&v.to_le_bytes());
    }
    // [224..256]: 32-bit mask for vm64y (lanes 0,2,3 active; lane 1 masked off!)
    for (i, v) in [0x8000_0000u32, 0x7fff_ffff, 0x8000_0000, 0x8000_0000, 0x1111_1111, 0x2222_2222, 0x3333_3333, 0x4444_4444]
        .into_iter()
        .enumerate()
    {
        init[224 + i * 4..228 + i * 4].copy_from_slice(&v.to_le_bytes());
    }

    let oracle = cfg!(feature = "rosetta-oracle").then(|| rosetta(&code, &init));
    run_tiers(&code, &init, |out| {
        // Every gather zeroes its entire mask YMM register on completion.
        for case in 0..16 {
            let m_off = 256 + case * 64 + 32;
            assert_eq!(&out[m_off..m_off + 32], &[0u8; 32], "case {case} mask must be zeroed");
        }
        // Case 10 (vgatherqps vm64x, all-zero mask, null base): lanes 0..1 preserved, lanes 2..7 zeroed.
        assert_eq!(&out[896..904], &init[128..136]);
        assert_eq!(&out[904..928], &[0u8; 24]);
        if let Some(control) = &oracle {
            assert_eq!(out, control);
        }
    });

    // Direct unit check: #UD on register aliasing (dest == mask, dest == vindex, mask == vindex)
    // and precise partial fault (lane 0 commits + clears mask; lane 1 faults with mask still set).
    let table = [0x1122_3344u32, 0x5566_7788];
    for &(dst, idx, msk) in &[(xmm0, xmm1, xmm0), (xmm0, xmm0, xmm2), (xmm0, xmm1, xmm1)] {
        let mut g = CodeAssembler::new(64).unwrap();
        g.vpgatherdd_3(dst, dword_ptr(rcx + idx * 4), msk).unwrap();
        let bytes = g.assemble(0x1000).unwrap();
        assert_eq!(
            Decoder::with_ip(64, &bytes, 0x1000, DecoderOptions::NONE).decode().mnemonic(),
            Mnemonic::INVALID,
            "decoder itself rejects aliased gather as #UD"
        );
        let ins = Decoder::with_ip(64, &bytes, 0x1000, DecoderOptions::NO_INVALID_CHECK).decode();
        let mut cpu = Cpu::new_zeroed();
        cpu.gpr[1] = table.as_ptr() as u64;
        cpu.xmm[0] = 0xDEAD_BEEF;
        cpu.xmm[1] = 0;
        cpu.xmm[2] = u128::MAX;
        FAULT_PENDING.with(|p| p.set(None));
        unsafe { assert!(cpu.exec_simd_fp_ext(ins.mnemonic(), &ins)) };
        assert_eq!(FAULT_PENDING.with(|p| p.take()), Some((u64::MAX, false)), "aliasing must fault #UD");
        assert_eq!(cpu.xmm[0], 0xDEAD_BEEF, "aliasing must not modify dest");
    }
    // Partial fault: lane 0 valid (&table[0]), lane 1 unmapped (address 0x1000).
    let mut cpu = Cpu::new_zeroed();
    cpu.gpr[1] = 0; // base = 0
    cpu.xmm[0] = 0x9999_9999_8888_8888_7777_7777_6666_6666u128;
    // Use vpgatherqq so 64-bit addresses fit cleanly: lane 0 = &table[0], lane 1 = 0x1000 (unmapped).
    let mut gq = CodeAssembler::new(64).unwrap();
    gq.vpgatherqq_3(xmm0, qword_ptr(rcx + xmm1 * 1), xmm2).unwrap();
    let bytes_q = gq.assemble(0x1000).unwrap();
    let ins_q = Decoder::with_ip(64, &bytes_q, 0x1000, DecoderOptions::NONE).decode();
    cpu.xmm[1] = (table.as_ptr() as u64 as u128) | (0x1000u128 << 64);
    cpu.xmm[2] = (1u128 << 63) | (1u128 << 127);
    FAULT_PENDING.with(|p| p.set(None));
    unsafe { assert!(cpu.exec_simd_fp_ext(ins_q.mnemonic(), &ins_q)) };
    assert_eq!(FAULT_PENDING.with(|p| p.take()), Some((0x1000, false)));
    assert_eq!(cpu.xmm[0] as u64, 0x5566_7788_1122_3344);
    assert_eq!((cpu.xmm[2] as u64), 0, "completed lane 0 mask cleared");
    assert_eq!((cpu.xmm[2] >> 64) as u64, 1u64 << 63, "faulting lane 1 mask preserved");
}

#[test]
fn bmi_lzcnt_movbe_and_xsave_ymm_across_tiers() {
    let mut a = CodeAssembler::new(64).unwrap();
    // BMI1: andn, bextr, blsi, blsmsk, blsr, tzcnt
    a.mov(rax, 0x0F0F_F0F0_1234_5678u64 as i64).unwrap();
    a.mov(rdx, 0x00FF_00FF_AAAA_5555u64 as i64).unwrap();
    a.andn(r8, rax, rdx).unwrap();
    a.mov(qword_ptr(rcx), r8).unwrap();
    a.mov(r9, 0x0000_0000_0000_0804u64 as i64).unwrap(); // start=4, len=8
    a.bextr(r8, rax, r9).unwrap();
    a.mov(qword_ptr(rcx + 8), r8).unwrap();
    a.blsi(r8, rax).unwrap();
    a.mov(qword_ptr(rcx + 16), r8).unwrap();
    a.blsmsk(r8, rax).unwrap();
    a.mov(qword_ptr(rcx + 24), r8).unwrap();
    a.blsr(r8, rax).unwrap();
    a.mov(qword_ptr(rcx + 32), r8).unwrap();
    a.tzcnt(r8, rax).unwrap();
    a.mov(qword_ptr(rcx + 40), r8).unwrap();
    // BMI2: bzhi, mulx, pdep, pext, rorx, sarx, shlx, shrx
    a.mov(r9, 12u64).unwrap();
    a.bzhi(r8, rax, r9).unwrap();
    a.mov(qword_ptr(rcx + 48), r8).unwrap();
    a.mulx(r8, r9, rax).unwrap(); // rdx * rax -> hi=r8, lo=r9
    a.mov(qword_ptr(rcx + 56), r8).unwrap();
    a.mov(qword_ptr(rcx + 64), r9).unwrap();
    a.pdep(r8, rax, rdx).unwrap();
    a.mov(qword_ptr(rcx + 72), r8).unwrap();
    a.pext(r8, rax, rdx).unwrap();
    a.mov(qword_ptr(rcx + 80), r8).unwrap();
    a.rorx(r8, rax, 13).unwrap();
    a.mov(qword_ptr(rcx + 88), r8).unwrap();
    a.mov(r9, 5u64).unwrap();
    a.sarx(r8, rax, r9).unwrap();
    a.mov(qword_ptr(rcx + 96), r8).unwrap();
    a.shlx(r8, rax, r9).unwrap();
    a.mov(qword_ptr(rcx + 104), r8).unwrap();
    a.shrx(r8, rax, r9).unwrap();
    a.mov(qword_ptr(rcx + 112), r8).unwrap();
    // LZCNT & MOVBE (16/32/64-bit)
    a.lzcnt(r8, rax).unwrap();
    a.mov(qword_ptr(rcx + 120), r8).unwrap();
    a.movbe(r8, qword_ptr(rcx + 128)).unwrap();
    a.movbe(qword_ptr(rcx + 136), r8).unwrap();
    a.movbe(r8d, dword_ptr(rcx + 128)).unwrap();
    a.mov(dword_ptr(rcx + 144), r8d).unwrap();
    a.movbe(r8w, word_ptr(rcx + 128)).unwrap();
    a.mov(word_ptr(rcx + 148), r8w).unwrap();
    // XSAVE/XRSTOR YMM upper-half roundtrip with AVX2 ymm state
    a.vmovdqu(ymm0, ymmword_ptr(rcx + 160)).unwrap();
    a.vpaddd(ymm0, ymm0, ymm0).unwrap();
    a.mov(eax, 7).unwrap();
    a.xor(edx, edx).unwrap();
    a.xsave64(ptr(rcx + 256)).unwrap();
    a.vzeroall().unwrap();
    a.xrstor64(ptr(rcx + 256)).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + 192), ymm0).unwrap();
    a.vzeroupper().unwrap();
    a.ret().unwrap();
    let code = a.assemble(0).unwrap();

    let mut init = vec![0u8; 256 + 832];
    init[128..136].copy_from_slice(&0x0123_4567_89AB_CDEFu64.to_le_bytes());
    for i in 0u32..8 {
        init[160 + (i as usize) * 4..164 + (i as usize) * 4].copy_from_slice(&(i + 10).to_le_bytes());
    }
    let oracle = cfg!(feature = "rosetta-oracle").then(|| rosetta(&code, &init));
    run_tiers(&code, &init, |out| {
        // YMM0 doubled and preserved across XSAVE64 -> VZEROALL -> XRSTOR64
        for i in 0u32..8 {
            let v = u32::from_le_bytes(out[192 + (i as usize) * 4..196 + (i as usize) * 4].try_into().unwrap());
            assert_eq!(v, (i + 10) * 2);
        }
        // MOVBE round-trip and 32/16-bit byte-swapped values
        assert_eq!(&out[136..144], &init[128..136]);
        assert_eq!(u32::from_le_bytes(out[144..148].try_into().unwrap()), 0xEFCD_AB89);
        assert_eq!(u16::from_le_bytes(out[148..150].try_into().unwrap()), 0xEFCD);
        if let Some(control) = &oracle {
            // Compare all deterministic outputs (skip XSAVE legacy x87/FIP/FDP header bytes 256..416
            // which may differ in host x87 tags, and check YMM save area 256+576..256+592 directly).
            assert_eq!(&out[..256], &control[..256]);
            assert_eq!(&out[256 + 576..256 + 592], &control[256 + 576..256 + 592]);
        }
    });
}

/// Every single one of the 113 mnemonics across `MMX_ALIASED` (89), `SIMD_LEGACY_FALLBACK` (16),
/// and `AVX2_GATHER` (8) across all covered operand forms (reg/reg, reg/mem, mem/reg, reg/imm,
/// Legacy/VEX.128/VEX.256, VSIB base/nobase/scale/a32/gs) must report `InstructionLowering::Arm64`.
#[test]
fn slice4_instruction_lowering_all_113_mnemonics_arm64() {
    use std::collections::BTreeSet;

    let mut a = CodeAssembler::new(64).unwrap();
    // --- 1. MMX_ALIASED (89 mnemonics) ---
    a.emms().unwrap();
    a.db(&[0x0f, 0x0e]).unwrap(); // femms
    a.pshufw(mm0, mm1, 0x1B).unwrap();
    a.pshufw(mm0, qword_ptr(rcx), 0xE4).unwrap();
    a.db(&[0x0f, 0xf7, 0xc1]).unwrap(); // maskmovq mm0, mm1 ([rdi])
    a.db(&[0x67, 0x0f, 0xf7, 0xc1]).unwrap(); // maskmovq mm0, mm1 ([edi])
    a.movntq(qword_ptr(rcx), mm0).unwrap();
    a.cvtpi2ps(xmm0, mm1).unwrap();
    a.cvtpi2ps(xmm0, qword_ptr(rcx)).unwrap();
    a.cvtpi2pd(xmm0, mm1).unwrap();
    a.cvtpi2pd(xmm0, qword_ptr(rcx)).unwrap();
    a.cvtps2pi(mm0, xmm1).unwrap();
    a.cvtps2pi(mm0, qword_ptr(rcx)).unwrap();
    a.cvttps2pi(mm0, xmm1).unwrap();
    a.cvttps2pi(mm0, qword_ptr(rcx)).unwrap();
    a.cvtpd2pi(mm0, xmm1).unwrap();
    a.cvtpd2pi(mm0, xmmword_ptr(rcx)).unwrap();
    a.cvttpd2pi(mm0, xmm1).unwrap();
    a.cvttpd2pi(mm0, xmmword_ptr(rcx)).unwrap();
    a.movdq2q(mm0, xmm1).unwrap();
    a.movq2dq(xmm0, mm1).unwrap();
    a.movd(mm0, eax).unwrap();
    a.movd(mm0, dword_ptr(rcx)).unwrap();
    a.movd(eax, mm0).unwrap();
    a.movd(dword_ptr(rcx), mm0).unwrap();
    a.movq(mm0, mm1).unwrap();
    a.movq(mm0, rax).unwrap();
    a.movq(mm0, qword_ptr(rcx)).unwrap();
    a.movq(rax, mm0).unwrap();
    a.movq(qword_ptr(rcx), mm0).unwrap();
    a.pinsrw(mm0, eax, 2).unwrap();
    a.pinsrw(mm0, rax, 3).unwrap();
    a.pinsrw(mm0, word_ptr(rcx), 1).unwrap();
    a.pextrw(eax, mm0, 2).unwrap();
    a.pextrw(rax, mm0, 3).unwrap();
    a.pmovmskb(eax, mm0).unwrap();
    a.pmovmskb(rax, mm0).unwrap();

    macro_rules! mm_rm {
        ($($m:ident),+ $(,)?) => {
            $(
                a.$m(mm0, mm1).unwrap();
                a.$m(mm0, qword_ptr(rcx)).unwrap();
            )+
        };
    }
    mm_rm!(
        paddb, paddw, paddd, paddq, psubb, psubw, psubd, psubq, paddsb, paddsw, paddusb, paddusw,
        psubsb, psubsw, psubusb, psubusw, pmaddwd, pmullw, pmulhw, pmulhuw, pmuludq, pminub,
        pmaxub, pminsw, pmaxsw, pavgb, pavgw, psadbw, pand, pandn, por, pxor, pcmpeqb, pcmpeqw,
        pcmpeqd, pcmpgtb, pcmpgtw, pcmpgtd, packsswb, packuswb, packssdw, punpcklbw, punpcklwd,
        punpckldq, punpckhbw, punpckhwd, punpckhdq, phaddw, phaddd, phaddsw, phsubw, phsubd,
        phsubsw, pmaddubsw, pmulhrsw, psignb, psignw, psignd, pabsb, pabsw, pabsd, pshufb
    );
    macro_rules! mm_shift {
        ($($m:ident),+ $(,)?) => {
            $(
                a.$m(mm0, mm1).unwrap();
                a.$m(mm0, qword_ptr(rcx)).unwrap();
                a.$m(mm0, 3).unwrap();
            )+
        };
    }
    mm_shift!(psllw, pslld, psllq, psrlw, psrld, psrlq, psraw, psrad);
    a.palignr(mm0, mm1, 3).unwrap();
    a.palignr(mm0, qword_ptr(rcx), 5).unwrap();

    // --- 2. SIMD_LEGACY_FALLBACK (16 mnemonics) ---
    a.phaddsw(xmm0, xmm1).unwrap();
    a.phaddsw(xmm0, xmmword_ptr(rcx)).unwrap();
    a.vphaddsw(xmm0, xmm1, xmm2).unwrap();
    a.vphaddsw(xmm0, xmm1, xmmword_ptr(rcx)).unwrap();
    a.vphaddsw(ymm0, ymm1, ymm2).unwrap();
    a.vphaddsw(ymm0, ymm1, ymmword_ptr(rcx)).unwrap();
    a.phsubsw(xmm0, xmm1).unwrap();
    a.phsubsw(xmm0, xmmword_ptr(rcx)).unwrap();
    a.vphsubsw(xmm0, xmm1, xmm2).unwrap();
    a.vphsubsw(xmm0, xmm1, xmmword_ptr(rcx)).unwrap();
    a.vphsubsw(ymm0, ymm1, ymm2).unwrap();
    a.vphsubsw(ymm0, ymm1, ymmword_ptr(rcx)).unwrap();
    a.dppd(xmm0, xmm1, 0x33).unwrap();
    a.dppd(xmm0, xmmword_ptr(rcx), 0x12).unwrap();
    a.vdppd(xmm0, xmm1, xmm2, 0x31).unwrap();
    a.vdppd(xmm0, xmm1, xmmword_ptr(rcx), 0x23).unwrap();
    a.phminposuw(xmm0, xmm1).unwrap();
    a.phminposuw(xmm0, xmmword_ptr(rcx)).unwrap();
    a.vphminposuw(xmm0, xmm1).unwrap();
    a.vphminposuw(xmm0, xmmword_ptr(rcx)).unwrap();
    a.mpsadbw(xmm0, xmm1, 0x05).unwrap();
    a.mpsadbw(xmm0, xmmword_ptr(rcx), 0x02).unwrap();
    a.vmpsadbw(xmm0, xmm1, xmm2, 0x05).unwrap();
    a.vmpsadbw(xmm0, xmm1, xmmword_ptr(rcx), 0x02).unwrap();
    a.vmpsadbw(ymm0, ymm1, ymm2, 0x2D).unwrap();
    a.vmpsadbw(ymm0, ymm1, ymmword_ptr(rcx), 0x1B).unwrap();
    // REX.W pcmpestri64 / vpcmpestri64 / pcmpestrm64 / vpcmpestrm64 (reg & mem)
    a.db(&[0x66, 0x48, 0x0f, 0x3a, 0x61, 0xc1, 0x08]).unwrap(); // pcmpestri64 xmm0, xmm1, 8
    a.db(&[0x66, 0x48, 0x0f, 0x3a, 0x61, 0x01, 0x08]).unwrap(); // pcmpestri64 xmm0, [rcx], 8
    a.db(&[0xc4, 0xe3, 0xf9, 0x61, 0xc1, 0x08]).unwrap(); // vpcmpestri64 xmm0, xmm1, 8
    a.db(&[0xc4, 0xe3, 0xf9, 0x61, 0x01, 0x08]).unwrap(); // vpcmpestri64 xmm0, [rcx], 8
    a.db(&[0x66, 0x48, 0x0f, 0x3a, 0x60, 0xc1, 0x08]).unwrap(); // pcmpestrm64 xmm0, xmm1, 8
    a.db(&[0x66, 0x48, 0x0f, 0x3a, 0x60, 0x01, 0x08]).unwrap(); // pcmpestrm64 xmm0, [rcx], 8
    a.db(&[0xc4, 0xe3, 0xf9, 0x60, 0xc1, 0x08]).unwrap(); // vpcmpestrm64 xmm0, xmm1, 8
    a.db(&[0xc4, 0xe3, 0xf9, 0x60, 0x01, 0x08]).unwrap(); // vpcmpestrm64 xmm0, [rcx], 8
    // maskmovdqu / vmaskmovdqu ([rdi] and [edi])
    a.db(&[0x66, 0x0f, 0xf7, 0xc1]).unwrap();
    a.db(&[0x67, 0x66, 0x0f, 0xf7, 0xc1]).unwrap();
    a.db(&[0xc5, 0xf9, 0xf7, 0xc1]).unwrap();
    a.db(&[0x67, 0xc5, 0xf9, 0xf7, 0xc1]).unwrap();

    // --- 3. AVX2_GATHER (8 mnemonics × 128/256 + VSIB variants) ---
    a.vpgatherdd_3(xmm0, dword_ptr(rdx + xmm1 * 4), xmm2).unwrap();
    a.vpgatherdd_3(ymm0, dword_ptr(rdx + ymm1 * 4 + 16), ymm2).unwrap();
    a.vpgatherdd_3(xmm0, dword_ptr(xmm1 * 4 + 0x1000), xmm2).unwrap(); // no base
    a.vpgatherdd_3(xmm0, dword_ptr(edx + xmm1 * 2 - 4), xmm2).unwrap(); // addr32 + neg disp
    a.vpgatherdd_3(xmm0, dword_ptr(rdx + xmm1 * 1).gs(), xmm2).unwrap(); // gs segment
    a.vpgatherdq_3(xmm0, qword_ptr(rdx + xmm1 * 8), xmm2).unwrap();
    a.vpgatherdq_3(ymm0, qword_ptr(rdx + xmm1 * 8), ymm2).unwrap();
    a.vpgatherqd_3(xmm0, dword_ptr(rdx + xmm1 * 4), xmm2).unwrap();
    a.vpgatherqd_3(xmm0, dword_ptr(rdx + ymm1 * 4), xmm2).unwrap();
    a.vpgatherqq_3(xmm0, qword_ptr(rdx + xmm1 * 8), xmm2).unwrap();
    a.vpgatherqq_3(ymm0, qword_ptr(rdx + ymm1 * 8), ymm2).unwrap();
    a.vgatherdps_3(xmm0, dword_ptr(rdx + xmm1 * 4), xmm2).unwrap();
    a.vgatherdps_3(ymm0, dword_ptr(rdx + ymm1 * 4), ymm2).unwrap();
    a.vgatherdpd_3(xmm0, qword_ptr(rdx + xmm1 * 8), xmm2).unwrap();
    a.vgatherdpd_3(ymm0, qword_ptr(rdx + xmm1 * 8), ymm2).unwrap();
    a.vgatherqps_3(xmm0, dword_ptr(rdx + xmm1 * 4), xmm2).unwrap();
    a.vgatherqps_3(xmm0, dword_ptr(rdx + ymm1 * 4), xmm2).unwrap();
    a.vgatherqpd_3(xmm0, qword_ptr(rdx + xmm1 * 8), xmm2).unwrap();
    a.vgatherqpd_3(ymm0, qword_ptr(rdx + ymm1 * 8), ymm2).unwrap();

    let bytes = a.assemble(0x1000_0000).unwrap();
    let mut seen = BTreeSet::new();
    let mut total = 0usize;
    let mut dec = Decoder::with_ip(64, &bytes, 0x1000_0000, DecoderOptions::NONE);
    while dec.can_decode() {
        let ins = dec.decode();
        assert_ne!(ins.mnemonic(), Mnemonic::INVALID, "bad encoding at {:#x}", ins.ip());
        assert_eq!(
            instruction_lowering(&ins),
            InstructionLowering::Arm64,
            "expected Arm64 lowering for `{ins}` ({:?})",
            ins.mnemonic()
        );
        seen.insert(format!("{:?}", ins.mnemonic()).to_ascii_lowercase());
        total += 1;
    }
    assert!(total >= 210, "expected full operand-form sweep, got {total}");
    let expected: &[&str] = &[
        // 89 MMX_ALIASED
        "emms", "femms", "pshufw", "maskmovq", "movntq", "cvtpi2ps", "cvtpi2pd", "cvtps2pi",
        "cvttps2pi", "cvtpd2pi", "cvttpd2pi", "movdq2q", "movq2dq", "movd", "movq", "pinsrw",
        "pextrw", "pmovmskb", "paddb", "paddw", "paddd", "paddq", "psubb", "psubw", "psubd",
        "psubq", "paddsb", "paddsw", "paddusb", "paddusw", "psubsb", "psubsw", "psubusb",
        "psubusw", "pmaddwd", "pmullw", "pmulhw", "pmulhuw", "pmuludq", "pminub", "pmaxub",
        "pminsw", "pmaxsw", "pavgb", "pavgw", "psadbw", "pand", "pandn", "por", "pxor", "pcmpeqb",
        "pcmpeqw", "pcmpeqd", "pcmpgtb", "pcmpgtw", "pcmpgtd", "packsswb", "packuswb", "packssdw",
        "punpcklbw", "punpcklwd", "punpckldq", "punpckhbw", "punpckhwd", "punpckhdq", "psllw",
        "pslld", "psllq", "psrlw", "psrld", "psrlq", "psraw", "psrad", "phaddw", "phaddd",
        "phaddsw", "phsubw", "phsubd", "phsubsw", "pmaddubsw", "pmulhrsw", "psignb", "psignw",
        "psignd", "pabsb", "pabsw", "pabsd", "pshufb", "palignr",
        // 16 SIMD_LEGACY_FALLBACK (phaddsw/phsubsw share mnemonic with MMX above -> 14 new + 2 shared = 16)
        "vphaddsw", "vphsubsw", "dppd", "vdppd", "phminposuw", "vphminposuw", "mpsadbw",
        "vmpsadbw", "pcmpestri64", "vpcmpestri64", "pcmpestrm64", "vpcmpestrm64", "maskmovdqu",
        "vmaskmovdqu",
        // 8 AVX2_GATHER
        "vpgatherdd", "vpgatherdq", "vpgatherqd", "vpgatherqq", "vgatherdps", "vgatherdpd",
        "vgatherqps", "vgatherqpd",
    ];
    for &mn in expected {
        assert!(seen.contains(mn), "missing mnemonic `{mn}` in slice-4 lowering coverage");
    }
}

/// Comprehensive translator-vs-interpreter difftest and known-answer vectors for all 89
/// `MMX_ALIASED` mnemonics (both register and memory forms), x87/MMX physical aliasing,
/// `emms`/`femms`, `cvtpi2ps/pd` reg-vs-mem mode transitions, and `cvt(t)ps/pd2pi` across all
/// 4 MXCSR rounding modes.
#[test]
fn mmx_all_89_mnemonics_reg_mem_and_x87_aliasing_across_tiers() {
    let mut a = CodeAssembler::new(64).unwrap();
    // Inputs at [rcx + 0..128]:
    //   +0:  mm_a = 0x8001_7FFF_FFFE_0180
    //   +8:  mm_b = 0x7F80_8000_0003_FF7F
    //   +16: shift_cnt_small = 3
    //   +24: shift_cnt_big   = 65
    //   +32: shift_cnt_hi32  = 0x0000_0001_0000_0001 (> 2^32, must clamp to >= width!)
    //   +40: pshufb_ctrl     = 0x80_87_0F_0A_07_04_01_00 (tests bit-7 zeroing & 3-bit index mask)
    //   +48: mm_m32768       = 0x8000_8000_7FFF_8000 (for pmulhrsw(-32768,-32768) == 0x8000)
    //   +56: mask_q          = 0x80_00_80_7F_00_80_01_80 (for maskmovq)
    //   +64: f32[4]          = [1.5, -2.5, 3.0, -4.0]
    //   +80: f64[2]          = [2.5, -3.5]
    //   +96: i32[2]          = [-123456, 789012]
    //   +112: ymm_sentinel (32 bytes at +112..144)
    // Outputs written sequentially starting at [rcx + 160].
    let mut out_off: i32 = 160;
    let store_mm = |a: &mut CodeAssembler, out_off: &mut i32, r: AsmRegisterMm| {
        a.movq(qword_ptr(rcx + *out_off), r).unwrap();
        *out_off += 8;
    };

    // Load baseline inputs
    a.movq(mm6, qword_ptr(rcx)).unwrap();
    a.movq(mm7, qword_ptr(rcx + 8)).unwrap();

    // movd (mm<-r32, mm<-m32, r32<-mm, m32<-mm) + movq (mm<-r64, r64<-mm) + movntq
    a.mov(eax, 0xCAFE_BABEu32 as i32).unwrap();
    a.movd(mm0, eax).unwrap();
    store_mm(&mut a, &mut out_off, mm0);
    a.movd(mm1, dword_ptr(rcx + 4)).unwrap();
    store_mm(&mut a, &mut out_off, mm1);
    a.movd(edx, mm6).unwrap();
    a.mov(qword_ptr(rcx + out_off), rdx).unwrap();
    out_off += 8;
    a.movd(dword_ptr(rcx + out_off), mm7).unwrap();
    out_off += 8;
    a.mov(rax, 0x0123_4567_89AB_CDEFu64 as i64).unwrap();
    a.movq(mm2, rax).unwrap();
    a.movq(rdx, mm2).unwrap();
    a.mov(qword_ptr(rcx + out_off), rdx).unwrap();
    out_off += 8;
    a.movntq(qword_ptr(rcx + out_off), mm2).unwrap();
    out_off += 8;

    // movdq2q & movq2dq (verify movq2dq preserves ymm_hi!)
    a.vmovdqu(ymm5, ymmword_ptr(rcx + 112)).unwrap();
    a.movq2dq(xmm5, mm6).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + out_off), ymm5).unwrap();
    out_off += 32;
    a.movdq2q(mm3, xmm5).unwrap();
    store_mm(&mut a, &mut out_off, mm3);

    // pinsrw (r32, r64, m16), pextrw (r32, r64), pmovmskb (r32, r64)
    a.movq(mm0, mm6).unwrap();
    a.mov(eax, 0xDEAD_1234u32 as i32).unwrap();
    a.pinsrw(mm0, eax, 1).unwrap();
    a.mov(rax, 0xBEEF_0000_0000_5678u64 as i64).unwrap();
    a.pinsrw(mm0, rax, 3).unwrap();
    a.pinsrw(mm0, word_ptr(rcx + 10), 2).unwrap();
    store_mm(&mut a, &mut out_off, mm0);
    a.pextrw(eax, mm0, 3).unwrap();
    a.mov(qword_ptr(rcx + out_off), rax).unwrap();
    out_off += 8;
    a.pextrw(rdx, mm0, 1).unwrap();
    a.mov(qword_ptr(rcx + out_off), rdx).unwrap();
    out_off += 8;
    a.pmovmskb(eax, mm6).unwrap();
    a.mov(qword_ptr(rcx + out_off), rax).unwrap();
    out_off += 8;
    a.pmovmskb(rdx, mm7).unwrap();
    a.mov(qword_ptr(rcx + out_off), rdx).unwrap();
    out_off += 8;

    // pshufw (reg & mem)
    a.pshufw(mm0, mm6, 0x1B).unwrap();
    store_mm(&mut a, &mut out_off, mm0);
    a.pshufw(mm0, qword_ptr(rcx + 8), 0xB1).unwrap();
    store_mm(&mut a, &mut out_off, mm0);

    // Binary MMX ALU, logic, compare, pack, unpack, SSSE3 — BOTH reg/reg AND reg/mem!
    macro_rules! run_mm_bin {
        ($($m:ident),+ $(,)?) => {
            $(
                a.movq(mm0, mm6).unwrap();
                a.$m(mm0, mm7).unwrap();
                store_mm(&mut a, &mut out_off, mm0);
                a.movq(mm0, mm6).unwrap();
                a.$m(mm0, qword_ptr(rcx + 8)).unwrap();
                store_mm(&mut a, &mut out_off, mm0);
            )+
        };
    }
    run_mm_bin!(
        paddb, paddw, paddd, paddq, psubb, psubw, psubd, psubq, paddsb, paddsw, paddusb, paddusw,
        psubsb, psubsw, psubusb, psubusw, pmaddwd, pmullw, pmulhw, pmulhuw, pmuludq, pminub,
        pmaxub, pminsw, pmaxsw, pavgb, pavgw, psadbw, pand, pandn, por, pxor, pcmpeqb, pcmpeqw,
        pcmpeqd, pcmpgtb, pcmpgtw, pcmpgtd, packsswb, packuswb, packssdw, punpcklbw, punpcklwd,
        punpckldq, punpckhbw, punpckhwd, punpckhdq, phaddw, phaddd, phaddsw, phsubw, phsubd,
        phsubsw, pmaddubsw, pmulhrsw, psignb, psignw, psignd
    );

    // pmulhrsw(-32768, -32768) special rounding overflow edge case -> 0x8000
    a.movq(mm0, qword_ptr(rcx + 48)).unwrap();
    a.pmulhrsw(mm0, qword_ptr(rcx + 48)).unwrap();
    store_mm(&mut a, &mut out_off, mm0);

    // Unary SSSE3 pabsb/pabsw/pabsd (reg & mem)
    macro_rules! run_mm_un {
        ($($m:ident),+ $(,)?) => {
            $(
                a.$m(mm0, mm6).unwrap();
                store_mm(&mut a, &mut out_off, mm0);
                a.$m(mm0, qword_ptr(rcx + 8)).unwrap();
                store_mm(&mut a, &mut out_off, mm0);
            )+
        };
    }
    run_mm_un!(pabsb, pabsw, pabsd);

    // pshufb (reg & mem) with 3-bit index masking + bit-7 zeroing
    a.movq(mm4, qword_ptr(rcx + 40)).unwrap();
    a.movq(mm0, mm6).unwrap();
    a.pshufb(mm0, mm4).unwrap();
    store_mm(&mut a, &mut out_off, mm0);
    a.movq(mm0, mm7).unwrap();
    a.pshufb(mm0, qword_ptr(rcx + 40)).unwrap();
    store_mm(&mut a, &mut out_off, mm0);

    // palignr across all edge immediates (0, 3, 8, 12, 16, 20), reg & mem
    for &imm in &[0i32, 3, 8, 12, 16, 20] {
        a.movq(mm0, mm6).unwrap();
        a.palignr(mm0, mm7, imm).unwrap();
        store_mm(&mut a, &mut out_off, mm0);
    }
    a.movq(mm0, mm6).unwrap();
    a.palignr(mm0, qword_ptr(rcx + 8), 5).unwrap();
    store_mm(&mut a, &mut out_off, mm0);

    // Shifts: imm (0, 3, 65), reg (3, 65, >2^32), mem (3, >2^32)
    a.movq(mm3, qword_ptr(rcx + 16)).unwrap(); // 3
    a.movq(mm4, qword_ptr(rcx + 24)).unwrap(); // 65
    a.movq(mm5, qword_ptr(rcx + 32)).unwrap(); // 2^32 + 1
    macro_rules! run_mm_shift {
        ($($m:ident),+ $(,)?) => {
            $(
                for &imm in &[0i32, 3, 65] {
                    a.movq(mm0, mm6).unwrap();
                    a.$m(mm0, imm).unwrap();
                    store_mm(&mut a, &mut out_off, mm0);
                }
                for cnt in [mm3, mm4, mm5] {
                    a.movq(mm0, mm6).unwrap();
                    a.$m(mm0, cnt).unwrap();
                    store_mm(&mut a, &mut out_off, mm0);
                }
                a.movq(mm0, mm6).unwrap();
                a.$m(mm0, qword_ptr(rcx + 16)).unwrap();
                store_mm(&mut a, &mut out_off, mm0);
                a.movq(mm0, mm6).unwrap();
                a.$m(mm0, qword_ptr(rcx + 32)).unwrap();
                store_mm(&mut a, &mut out_off, mm0);
            )+
        };
    }
    run_mm_shift!(psllw, pslld, psllq, psrlw, psrld, psrlq, psraw, psrad);

    // maskmovq ([rdi] partial mask, and all-zero mask over unmapped 0x100 must NOT fault)
    let maskmov_off = out_off;
    a.movq(qword_ptr(rcx + out_off), mm7).unwrap(); // pre-fill sentinel
    a.lea(rdi, ptr(rcx + out_off)).unwrap();
    a.movq(mm0, mm6).unwrap();
    a.movq(mm1, qword_ptr(rcx + 56)).unwrap();
    a.db(&[0x0f, 0xf7, 0xc1]).unwrap(); // maskmovq mm0, mm1
    out_off += 8;
    a.mov(rdi, 0x100u64).unwrap();
    a.pxor(mm1, mm1).unwrap();
    a.db(&[0x0f, 0xf7, 0xc1]).unwrap(); // all-zero mask -> no fault

    // cvtpi2ps/pd (reg & mem) — verify cvtpi2ps preserves xmm[127:64] and ymm_hi!
    let cvt_off = out_off;
    a.vmovdqu(ymm2, ymmword_ptr(rcx + 112)).unwrap();
    a.movq(mm0, qword_ptr(rcx + 96)).unwrap();
    a.cvtpi2ps(xmm2, mm0).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + out_off), ymm2).unwrap();
    out_off += 32;
    a.vmovdqu(ymm2, ymmword_ptr(rcx + 112)).unwrap();
    a.cvtpi2ps(xmm2, qword_ptr(rcx + 96)).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + out_off), ymm2).unwrap();
    out_off += 32;
    a.vmovdqu(ymm2, ymmword_ptr(rcx + 112)).unwrap();
    a.cvtpi2pd(xmm2, mm0).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + out_off), ymm2).unwrap();
    out_off += 32;
    a.vmovdqu(ymm2, ymmword_ptr(rcx + 112)).unwrap();
    a.cvtpi2pd(xmm2, qword_ptr(rcx + 96)).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + out_off), ymm2).unwrap();
    out_off += 32;

    // cvtps2pi / cvttps2pi / cvtpd2pi / cvttpd2pi (reg & mem)
    a.movups(xmm3, xmmword_ptr(rcx + 64)).unwrap();
    a.movupd(xmm4, xmmword_ptr(rcx + 80)).unwrap();
    a.cvtps2pi(mm0, xmm3).unwrap();
    store_mm(&mut a, &mut out_off, mm0);
    a.cvtps2pi(mm0, qword_ptr(rcx + 64)).unwrap();
    store_mm(&mut a, &mut out_off, mm0);
    a.cvttps2pi(mm0, xmm3).unwrap();
    store_mm(&mut a, &mut out_off, mm0);
    a.cvttps2pi(mm0, qword_ptr(rcx + 64)).unwrap();
    store_mm(&mut a, &mut out_off, mm0);
    a.cvtpd2pi(mm0, xmm4).unwrap();
    store_mm(&mut a, &mut out_off, mm0);
    a.cvtpd2pi(mm0, xmmword_ptr(rcx + 80)).unwrap();
    store_mm(&mut a, &mut out_off, mm0);
    a.cvttpd2pi(mm0, xmm4).unwrap();
    store_mm(&mut a, &mut out_off, mm0);
    a.cvttpd2pi(mm0, xmmword_ptr(rcx + 80)).unwrap();
    store_mm(&mut a, &mut out_off, mm0);

    a.emms().unwrap();
    a.db(&[0x0f, 0x0e]).unwrap(); // femms
    a.vzeroupper().unwrap();
    a.ret().unwrap();
    let code = a.assemble(0).unwrap();

    let buf_len = (out_off as usize + 64).next_multiple_of(64);
    let mut init = vec![0u8; buf_len];
    init[0..8].copy_from_slice(&0x8001_7FFF_FFFE_0180u64.to_le_bytes());
    init[8..16].copy_from_slice(&0x7F80_8000_0003_FF7Fu64.to_le_bytes());
    init[16..24].copy_from_slice(&3u64.to_le_bytes());
    init[24..32].copy_from_slice(&65u64.to_le_bytes());
    init[32..40].copy_from_slice(&0x0000_0001_0000_0001u64.to_le_bytes());
    init[40..48].copy_from_slice(&0x8087_0F0A_0704_0100u64.to_le_bytes());
    init[48..56].copy_from_slice(&0x8000_8000_7FFF_8000u64.to_le_bytes());
    init[56..64].copy_from_slice(&0x8000_807F_0080_0180u64.to_le_bytes());
    for (i, v) in [1.5f32, -2.5, 3.0, -4.0].into_iter().enumerate() {
        init[64 + i * 4..68 + i * 4].copy_from_slice(&v.to_bits().to_le_bytes());
    }
    for (i, v) in [2.5f64, -3.5].into_iter().enumerate() {
        init[80 + i * 8..88 + i * 8].copy_from_slice(&v.to_bits().to_le_bytes());
    }
    init[96..100].copy_from_slice(&(-123456i32).to_le_bytes());
    init[100..104].copy_from_slice(&789012i32.to_le_bytes());
    for i in 0..32usize {
        init[112 + i] = 0xA0 + (i as u8);
    }

    let mut generic_ref: Option<Vec<u8>> = None;
    run_tiers(&code, &init, |out| {
        if let Some(ref g) = generic_ref {
            assert_eq!(out, g.as_slice(), "MMX tier mismatch against generic interpreter");
        } else {
            generic_ref = Some(out.to_vec());
        }
        // Spot-check key known-answer invariants:
        // 1. movq2dq preserved ymm_hi (bytes 16..32 of the 32-byte slot at 160 + 48 = 208)
        assert_eq!(&out[208..216], &init[0..8]);
        assert_eq!(&out[216..224], &[0u8; 8]);
        assert_eq!(&out[224..240], &init[128..144]);
        // 2. maskmovq wrote only bytes with mask MSB == 1 (bits 7 of 0x80,0x01,0x80,0x00,0x7F,0x80,0x00,0x80)
        let mm_out = &out[maskmov_off as usize..maskmov_off as usize + 8];
        for i in 0..8 {
            let expected = if init[56 + i] & 0x80 != 0 { init[i] } else { init[8 + i] };
            assert_eq!(mm_out[i], expected, "maskmovq byte {i}");
        }
        // 3. cvtpi2ps preserved xmm[127:64] and ymm_hi
        let c0 = cvt_off as usize;
        assert_eq!(&out[c0 + 8..c0 + 32], &init[120..144]);
        assert_eq!(&out[c0 + 32..c0 + 64], &out[c0..c0 + 32]);
    });

    // --- Architectural X87State physical aliasing & EMMS/FEMMS/CVTPI2P* mode transition checks ---
    let run_x87_check = |code: &[u8], init: &[u8], init_x87: Option<regfile_ext::X87State>, check: &dyn Fn(regfile_ext::X87State)| {
        for (name, fast, compiled, cached) in [
            ("generic", false, false, false),
            ("FastOp", true, false, false),
            ("JIT", true, true, false),
            ("disk AOT", true, true, true),
        ] {
            let code_ptr = jit_tests::arena_alloc(code.len() + 16, 16);
            let data = jit_tests::arena_alloc(init.len().max(64), 64);
            unsafe {
                std::ptr::copy_nonoverlapping(code.as_ptr(), code_ptr, code.len());
                std::ptr::write_bytes(code_ptr.add(code.len()), 0xcc, 16);
                std::ptr::copy_nonoverlapping(init.as_ptr(), data, init.len());
            }
            let fast_off = TEST_FAST_OFF.with(|s| s.replace(!fast));
            jit::force(if compiled { 1 } else { 2 });
            if cached {
                let ips = Decoder::with_ip(64, code, code_ptr as u64, DecoderOptions::NONE)
                    .into_iter()
                    .map(|ins| ins.ip())
                    .collect::<Vec<_>>();
                assert!(unsafe { aot::test_install(&ips, || {}).0 } > 0, "{name} aot install");
            }
            let saved = regfile_ext::x87_snapshot();
            if let Some(st) = init_x87 {
                regfile_ext::x87_restore(st);
            } else {
                regfile_ext::x87_reset();
            }
            unsafe {
                emu_run_full(code_ptr as u64, [data as u64, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]);
            }
            let after = regfile_ext::x87_snapshot();
            regfile_ext::x87_restore(saved);
            if cached {
                aot::test_uninstall();
            }
            jit::force(0);
            TEST_FAST_OFF.with(|s| s.set(fast_off));
            check(after);
        }
    };

    // Case A: writing mm0..mm7 populates X87State::regs[i][0..8], sets exponent 0xFFFF, top=0, valid_mask=0xFF.
    let mut a_write = CodeAssembler::new(64).unwrap();
    let mms = [mm0, mm1, mm2, mm3, mm4, mm5, mm6, mm7];
    for (i, &r) in mms.iter().enumerate() {
        a_write.movq(r, qword_ptr(rcx + (i as i32) * 8)).unwrap();
    }
    a_write.ret().unwrap();
    let mut w_init = [0u8; 64];
    for i in 0..8u64 {
        let v = 0x1111_2222_3333_4440 + i;
        w_init[(i as usize) * 8..(i as usize) * 8 + 8].copy_from_slice(&v.to_le_bytes());
    }
    run_x87_check(&a_write.assemble(0).unwrap(), &w_init, None, &|st| {
        assert_eq!(st.top, 0);
        assert_eq!(st.valid_mask, 0xFF);
        for i in 0..8usize {
            let expected = 0x1111_2222_3333_4440u64 + i as u64;
            assert_eq!(&st.regs[i][0..8], &expected.to_le_bytes());
            assert_eq!(&st.regs[i][8..10], &[0xFF, 0xFF]);
        }
    });

    // Case B: emms and femms clear valid_mask=0 while preserving regs[0..8].
    for is_femms in [false, true] {
        let mut a_emms = CodeAssembler::new(64).unwrap();
        a_emms.movq(mm3, qword_ptr(rcx)).unwrap();
        if is_femms {
            a_emms.db(&[0x0f, 0x0e]).unwrap();
        } else {
            a_emms.emms().unwrap();
        }
        a_emms.ret().unwrap();
        run_x87_check(&a_emms.assemble(0).unwrap(), &w_init, None, &|st| {
            assert_eq!(st.valid_mask, 0);
            assert_eq!(&st.regs[3][0..8], &0x1111_2222_3333_4440u64.to_le_bytes());
            assert_eq!(&st.regs[3][8..10], &[0xFF, 0xFF]);
        });
    }

    // Case C: cvtpi2ps/cvtpi2pd MEMORY form does NOT transition x87 to MMX mode (preserves top & valid_mask),
    // whereas REGISTER form DOES set top=0, valid_mask=0xFF.
    let mut seed_x87 = regfile_ext::X87State::new();
    seed_x87.top = 5;
    seed_x87.valid_mask = 0x18;
    for is_pd in [false, true] {
        let mut a_mem = CodeAssembler::new(64).unwrap();
        if is_pd {
            a_mem.cvtpi2pd(xmm0, qword_ptr(rcx)).unwrap();
        } else {
            a_mem.cvtpi2ps(xmm0, qword_ptr(rcx)).unwrap();
        }
        a_mem.ret().unwrap();
        run_x87_check(&a_mem.assemble(0).unwrap(), &w_init, Some(seed_x87), &|st| {
            assert_eq!(st.top, 5, "cvtpi2p* mem must not alter x87 top");
            assert_eq!(st.valid_mask, 0x18, "cvtpi2p* mem must not set MMX valid_mask");
        });

        let mut a_reg = CodeAssembler::new(64).unwrap();
        if is_pd {
            a_reg.cvtpi2pd(xmm0, mm2).unwrap();
        } else {
            a_reg.cvtpi2ps(xmm0, mm2).unwrap();
        }
        a_reg.ret().unwrap();
        run_x87_check(&a_reg.assemble(0).unwrap(), &w_init, Some(seed_x87), &|st| {
            assert_eq!(st.top, 0, "cvtpi2p* reg must transition to MMX top=0");
            assert_eq!(st.valid_mask, 0xFF, "cvtpi2p* reg must set MMX valid_mask=0xFF");
        });
    }

    // --- cvt(t)ps/pd2pi across all 4 MXCSR rounding modes & overflow/NaN/Inf edge values ---
    let mut a_rc = CodeAssembler::new(64).unwrap();
    a_rc.ldmxcsr(dword_ptr(rcx)).unwrap();
    // 6 pairs of f32 at [rcx + 16..64], 6 pairs of f64 at [rcx + 64..160]
    let mut o = 160i32;
    for i in 0..6i32 {
        a_rc.movq(xmm0, qword_ptr(rcx + 16 + i * 8)).unwrap();
        a_rc.cvtps2pi(mm0, xmm0).unwrap();
        a_rc.movq(qword_ptr(rcx + o), mm0).unwrap();
        o += 8;
        a_rc.cvttps2pi(mm1, qword_ptr(rcx + 16 + i * 8)).unwrap();
        a_rc.movq(qword_ptr(rcx + o), mm1).unwrap();
        o += 8;
        a_rc.movupd(xmm1, xmmword_ptr(rcx + 64 + i * 16)).unwrap();
        a_rc.cvtpd2pi(mm2, xmm1).unwrap();
        a_rc.movq(qword_ptr(rcx + o), mm2).unwrap();
        o += 8;
        a_rc.cvttpd2pi(mm3, xmmword_ptr(rcx + 64 + i * 16)).unwrap();
        a_rc.movq(qword_ptr(rcx + o), mm3).unwrap();
        o += 8;
    }
    a_rc.emms().unwrap();
    a_rc.ret().unwrap();
    let rc_code = a_rc.assemble(0).unwrap();
    let f32_pairs: [[f32; 2]; 6] = [
        [1.5, -1.5],
        [2.5, -2.5],
        [f32::NAN, f32::INFINITY],
        [f32::NEG_INFINITY, 2147483648.0],
        [-2147483648.0, -2147483904.0],
        [2147483520.0, -0.5],
    ];
    let f64_pairs: [[f64; 2]; 6] = [
        [1.5, -1.5],
        [2.5, -2.5],
        [f64::NAN, f64::INFINITY],
        [f64::NEG_INFINITY, 2147483648.0],
        [2147483647.9, -2147483648.9],
        [-2147483648.0, -0.5],
    ];
    for rc_bits in [0x0000u32, 0x2000, 0x4000, 0x6000] {
        let mut rc_init = vec![0u8; (o as usize + 32).next_multiple_of(64)];
        rc_init[0..4].copy_from_slice(&(0x1F80 | rc_bits).to_le_bytes());
        for (i, p) in f32_pairs.iter().enumerate() {
            rc_init[16 + i * 8..20 + i * 8].copy_from_slice(&p[0].to_bits().to_le_bytes());
            rc_init[20 + i * 8..24 + i * 8].copy_from_slice(&p[1].to_bits().to_le_bytes());
        }
        for (i, p) in f64_pairs.iter().enumerate() {
            rc_init[64 + i * 16..72 + i * 16].copy_from_slice(&p[0].to_bits().to_le_bytes());
            rc_init[72 + i * 16..80 + i * 16].copy_from_slice(&p[1].to_bits().to_le_bytes());
        }
        let mut g_ref: Option<Vec<u8>> = None;
        run_tiers(&rc_code, &rc_init, |out| {
            if let Some(ref g) = g_ref {
                assert_eq!(out, g.as_slice(), "cvt*2pi mismatch for MXCSR rc={rc_bits:#x}");
            } else {
                g_ref = Some(out.to_vec());
            }
            // NaN & +Inf must produce x86 integer indefinite 0x8000_0000 (pair 2 at off = 160 + 2*32)
            let p2 = 160 + 2 * 32;
            for sub in [0, 8, 16, 24] {
                let lo = i32::from_le_bytes(out[p2 + sub..p2 + sub + 4].try_into().unwrap());
                let hi = i32::from_le_bytes(out[p2 + sub + 4..p2 + sub + 8].try_into().unwrap());
                assert_eq!((lo, hi), (i32::MIN, i32::MIN), "NaN/Inf indefinite rc={rc_bits:#x} sub={sub}");
            }
        });
    }
}

/// Comprehensive translator-vs-interpreter difftest and known-answer vectors for all 16
/// `SIMD_LEGACY_FALLBACK` mnemonics: `(v)phaddsw`, `(v)phsubsw`, `(v)phminposuw`,
/// `(v)mpsadbw` (128 & 256), `(v)dppd` (all masks, rounding modes, NaN suppression & tiebreak),
/// `(v)pcmpestri64`/`(v)pcmpestrm64` (REX.W 64-bit lengths & flags), and `(v)maskmovdqu`.
#[test]
fn simd_legacy_stragglers_all_16_mnemonics_across_tiers() {
    let mut a = CodeAssembler::new(64).unwrap();
    // Inputs at [rcx + 0..256]:
    //   +0..32:   y_a (16x i16 incl. saturation edges i16::MAX/MIN)
    //   +32..64:  y_b (16x i16)
    //   +64..80:  minpos_a (ties at lanes 2 and 5)
    //   +80..96:  minpos_b (all 0xFFFF except lane 7)
    //   +96..128: dppd_a (f64 pairs)
    //   +128..160: dppd_b (f64 pairs, lane 1 has SNaN/Inf for mask-off test)
    //   +160..176: str_a (for pcmpestr*64)
    //   +176..192: str_b (for pcmpestr*64)
    //   +192..208: maskmov_mask (16B)
    //   +208..240: ymm_sentinel (32B)
    let mut o = 256i32;

    // 1. (v)phaddsw / (v)phsubsw (Legacy, VEX.128, VEX.256, reg & mem)
    a.vmovdqu(ymm4, ymmword_ptr(rcx)).unwrap();
    a.vmovdqu(ymm5, ymmword_ptr(rcx + 32)).unwrap();

    // Legacy phaddsw preserves ymm_hi
    a.vmovdqu(ymm0, ymmword_ptr(rcx + 208)).unwrap();
    a.movdqu(xmm0, xmm4).unwrap();
    a.phaddsw(xmm0, xmm5).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + o), ymm0).unwrap();
    let phaddsw_leg_off = o;
    o += 32;
    a.movdqu(xmm0, xmm4).unwrap();
    a.phaddsw(xmm0, xmmword_ptr(rcx + 32)).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + o), ymm0).unwrap();
    o += 32;
    // VEX.128 zeroes ymm_hi
    a.vmovdqu(ymm0, ymmword_ptr(rcx + 208)).unwrap();
    a.vphaddsw(xmm0, xmm4, xmm5).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + o), ymm0).unwrap();
    let phaddsw_v128_off = o;
    o += 32;
    a.vphaddsw(xmm0, xmm4, xmmword_ptr(rcx + 32)).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + o), ymm0).unwrap();
    o += 32;
    // VEX.256
    a.vphaddsw(ymm0, ymm4, ymm5).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + o), ymm0).unwrap();
    o += 32;
    a.vphaddsw(ymm0, ymm4, ymmword_ptr(rcx + 32)).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + o), ymm0).unwrap();
    o += 32;

    // phsubsw / vphsubsw
    a.vmovdqu(ymm0, ymmword_ptr(rcx + 208)).unwrap();
    a.movdqu(xmm0, xmm4).unwrap();
    a.phsubsw(xmm0, xmm5).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + o), ymm0).unwrap();
    o += 32;
    a.movdqu(xmm0, xmm4).unwrap();
    a.phsubsw(xmm0, xmmword_ptr(rcx + 32)).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + o), ymm0).unwrap();
    o += 32;
    a.vphsubsw(xmm0, xmm4, xmm5).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + o), ymm0).unwrap();
    o += 32;
    a.vphsubsw(xmm0, xmm4, xmmword_ptr(rcx + 32)).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + o), ymm0).unwrap();
    o += 32;
    a.vphsubsw(ymm0, ymm4, ymm5).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + o), ymm0).unwrap();
    o += 32;
    a.vphsubsw(ymm0, ymm4, ymmword_ptr(rcx + 32)).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + o), ymm0).unwrap();
    o += 32;

    // 2. (v)phminposuw (Legacy & VEX.128, reg & mem, ties & lane 7)
    let minpos_off = o;
    a.vmovdqu(ymm0, ymmword_ptr(rcx + 208)).unwrap();
    a.movdqu(xmm1, xmmword_ptr(rcx + 64)).unwrap();
    a.phminposuw(xmm0, xmm1).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + o), ymm0).unwrap();
    o += 32;
    a.phminposuw(xmm0, xmmword_ptr(rcx + 80)).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + o), ymm0).unwrap();
    o += 32;
    a.vmovdqu(ymm0, ymmword_ptr(rcx + 208)).unwrap();
    a.vphminposuw(xmm0, xmm1).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + o), ymm0).unwrap();
    o += 32;
    a.vphminposuw(xmm0, xmmword_ptr(rcx + 80)).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + o), ymm0).unwrap();
    o += 32;

    // 3. (v)mpsadbw (Legacy, VEX.128, VEX.256 with distinct lo imm[2:0] and hi imm[5:3], reg & mem)
    a.vmovdqu(ymm0, ymmword_ptr(rcx + 208)).unwrap();
    a.movdqu(xmm0, xmm4).unwrap();
    a.mpsadbw(xmm0, xmm5, 0x05).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + o), ymm0).unwrap();
    o += 32;
    a.movdqu(xmm0, xmm4).unwrap();
    a.mpsadbw(xmm0, xmmword_ptr(rcx + 32), 0x06).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + o), ymm0).unwrap();
    o += 32;
    a.vmpsadbw(xmm0, xmm4, xmm5, 0x05).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + o), ymm0).unwrap();
    o += 32;
    a.vmpsadbw(xmm0, xmm4, xmmword_ptr(rcx + 32), 0x06).unwrap();
    a.vmovdqu(ymmword_ptr(rcx + o), ymm0).unwrap();
    o += 32;
    a.vmpsadbw(ymm0, ymm4, ymm5, 0x2D).unwrap(); // lo=5 (a+4, b+4), hi=5 (a+4, b+4)
    a.vmovdqu(ymmword_ptr(rcx + o), ymm0).unwrap();
    o += 32;
    a.vmpsadbw(ymm0, ymm4, ymmword_ptr(rcx + 32), 0x1B).unwrap(); // lo=3, hi=3
    a.vmovdqu(ymmword_ptr(rcx + o), ymm0).unwrap();
    o += 32;
    a.vmpsadbw(ymm0, ymm4, ymm5, 0x3A).unwrap(); // distinct lo=2 vs hi=7!
    a.vmovdqu(ymmword_ptr(rcx + o), ymm0).unwrap();
    o += 32;

    // 4. (v)dppd across all mask combinations + masked-off SNaN/Inf suppression + dual-NaN tiebreak
    let dppd_off = o;
    a.movupd(xmm2, xmmword_ptr(rcx + 96)).unwrap();
    a.movupd(xmm3, xmmword_ptr(rcx + 128)).unwrap();
    for &imm in &[0x00i32, 0x03, 0x11, 0x12, 0x21, 0x23, 0x31, 0x32, 0x33] {
        a.movupd(xmm0, xmm2).unwrap();
        a.dppd(xmm0, xmm3, imm).unwrap();
        a.movupd(xmmword_ptr(rcx + o), xmm0).unwrap();
        o += 16;
        a.vdppd(xmm0, xmm2, xmmword_ptr(rcx + 128), imm).unwrap();
        a.movupd(xmmword_ptr(rcx + o), xmm0).unwrap();
        o += 16;
    }
    // Masked-off lane 1 has SNaN * Inf; imm=0x11 must NOT produce NaN!
    let dppd_nan_off = o;
    a.movupd(xmm2, xmmword_ptr(rcx + 112)).unwrap();
    a.movupd(xmm3, xmmword_ptr(rcx + 144)).unwrap();
    a.vdppd(xmm0, xmm2, xmm3, 0x11).unwrap();
    a.movupd(xmmword_ptr(rcx + o), xmm0).unwrap();
    o += 16;
    // Both lanes active with dual NaN in lane 1 -> first-operand quieted NaN priority
    a.vdppd(xmm0, xmm2, xmm3, 0x33).unwrap();
    a.movupd(xmmword_ptr(rcx + o), xmm0).unwrap();
    o += 16;

    // 5. REX.W pcmpestri64 / vpcmpestri64 / pcmpestrm64 / vpcmpestrm64 (64-bit RAX/RDX lengths)
    // Use r11 = rcx as buffer base because pcmpestri64 writes its index result into ECX/RCX.
    a.mov(r11, rcx).unwrap();
    a.movdqu(xmm1, xmmword_ptr(r11 + 160)).unwrap();
    a.movdqu(xmm2, xmmword_ptr(r11 + 176)).unwrap();
    // Use RAX/RDX with non-zero upper 32 bits (0x1_0000_0004 -> clamped to max_len=16, NOT 4!),
    // negative i64 (-5 -> abs=5), and i64::MIN (clamped to max_len).
    let pcmp_off = o;
    for &(len_a, len_d, imm8) in &[
        (0x0000_0001_0000_0004u64, 0x0000_0000_0000_0008u64, 0x08u8),
        ((-5i64) as u64, 10u64, 0x0Cu8),
        (i64::MIN as u64, (-3i64) as u64, 0x01u8),
        (4u64, 6u64, 0x14u8),
        (4u64, 6u64, 0x60u8),
    ] {
        // Legacy pcmpestri64 (reg)
        a.mov(rax, len_a as i64).unwrap();
        a.mov(rdx, len_d as i64).unwrap();
        a.movdqu(xmm0, xmm1).unwrap();
        a.db(&[0x66, 0x48, 0x0f, 0x3a, 0x61, 0xc2, imm8]).unwrap(); // pcmpestri64 xmm0, xmm2, imm8
        a.pushfq().unwrap();
        a.pop(r8).unwrap();
        a.and(r8, 0x8D5).unwrap();
        a.mov(qword_ptr(r11 + o), rcx).unwrap();
        a.mov(qword_ptr(r11 + o + 8), r8).unwrap();
        o += 16;

        // VEX vpcmpestri64 (mem: [r11 + 176] -> ModRM with r11 needs REX.B=1 / VEX.B=0)
        a.mov(rax, len_a as i64).unwrap();
        a.mov(rdx, len_d as i64).unwrap();
        a.lea(r9, ptr(r11 + 176)).unwrap();
        a.movdqu(xmm0, xmm1).unwrap();
        // vpcmpestri64 xmm0, [rsi], imm8 (use rsi so VEX.B=1 cleanly)
        a.mov(rsi, r9).unwrap();
        a.db(&[0xc4, 0xe3, 0xf9, 0x61, 0x06, imm8]).unwrap();
        a.pushfq().unwrap();
        a.pop(r8).unwrap();
        a.and(r8, 0x8D5).unwrap();
        a.mov(qword_ptr(r11 + o), rcx).unwrap();
        a.mov(qword_ptr(r11 + o + 8), r8).unwrap();
        o += 16;

        // Legacy pcmpestrm64 (reg) — preserves ymm0[255:128]
        a.mov(rax, len_a as i64).unwrap();
        a.mov(rdx, len_d as i64).unwrap();
        a.vmovdqu(ymm0, ymmword_ptr(r11 + 208)).unwrap();
        a.movdqu(xmm0, xmm1).unwrap();
        a.db(&[0x66, 0x48, 0x0f, 0x3a, 0x60, 0xc2, imm8]).unwrap();
        a.pushfq().unwrap();
        a.pop(r8).unwrap();
        a.and(r8, 0x8D5).unwrap();
        a.vmovdqu(ymmword_ptr(r11 + o), ymm0).unwrap();
        a.mov(qword_ptr(r11 + o + 32), r8).unwrap();
        o += 48;

        // VEX vpcmpestrm64 (mem: [rsi]) — zeroes ymm0[255:128]
        a.mov(rax, len_a as i64).unwrap();
        a.mov(rdx, len_d as i64).unwrap();
        a.vmovdqu(ymm0, ymmword_ptr(r11 + 208)).unwrap();
        a.movdqu(xmm0, xmm1).unwrap();
        a.db(&[0xc4, 0xe3, 0xf9, 0x60, 0x06, imm8]).unwrap();
        a.pushfq().unwrap();
        a.pop(r8).unwrap();
        a.and(r8, 0x8D5).unwrap();
        a.vmovdqu(ymmword_ptr(r11 + o), ymm0).unwrap();
        a.mov(qword_ptr(r11 + o + 32), r8).unwrap();
        o += 48;
    }

    // 6. (v)maskmovdqu (Legacy & VEX, partial mask + all-zero mask over unmapped 0x100)
    let maskmovdqu_off = o;
    a.movdqu(xmm0, xmmword_ptr(r11)).unwrap();
    a.movdqu(xmm1, xmmword_ptr(r11 + 192)).unwrap();
    a.movdqu(xmm2, xmmword_ptr(r11 + 32)).unwrap();
    a.movdqu(xmmword_ptr(r11 + o), xmm2).unwrap();
    a.lea(rdi, ptr(r11 + o)).unwrap();
    a.db(&[0x66, 0x0f, 0xf7, 0xc1]).unwrap(); // maskmovdqu xmm0, xmm1
    o += 16;
    a.movdqu(xmmword_ptr(r11 + o), xmm2).unwrap();
    a.lea(rdi, ptr(r11 + o)).unwrap();
    a.db(&[0xc5, 0xf9, 0xf7, 0xc1]).unwrap(); // vmaskmovdqu xmm0, xmm1
    o += 16;
    // All-zero mask over unmapped 0x100 must NOT fault
    a.mov(rdi, 0x100u64).unwrap();
    a.vpxor(xmm1, xmm1, xmm1).unwrap();
    a.db(&[0x66, 0x0f, 0xf7, 0xc1]).unwrap();
    a.db(&[0xc5, 0xf9, 0xf7, 0xc1]).unwrap();

    a.vzeroupper().unwrap();
    a.ret().unwrap();
    let code = a.assemble(0).unwrap();

    let buf_len = (o as usize + 64).next_multiple_of(64);
    let mut init = vec![0u8; buf_len];
    let ya: [i16; 16] = [
        30000, 10000, -30000, -10000, i16::MIN, i16::MAX, 100, -200, 32767, 1, -32768, -1, 500,
        600, -700, 800,
    ];
    let yb: [i16; 16] = [
        -32768, 100, 32767, -100, 20000, 20000, -20000, -20000, 10, 20, 30, 40, -10, -20, -30, -40,
    ];
    for i in 0..16 {
        init[i * 2..i * 2 + 2].copy_from_slice(&ya[i].to_le_bytes());
        init[32 + i * 2..34 + i * 2].copy_from_slice(&yb[i].to_le_bytes());
    }
    let mp_a: [u16; 8] = [500, 200, 42, 999, 100, 42, 77, 1000];
    let mp_b: [u16; 8] = [0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFE];
    for i in 0..8 {
        init[64 + i * 2..66 + i * 2].copy_from_slice(&mp_a[i].to_le_bytes());
        init[80 + i * 2..82 + i * 2].copy_from_slice(&mp_b[i].to_le_bytes());
    }
    // dppd_a (+96..112) & dppd_b (+128..144): normal values + signed-zero test
    init[96..104].copy_from_slice(&1.5f64.to_bits().to_le_bytes());
    init[104..112].copy_from_slice(&(-2.25f64).to_bits().to_le_bytes());
    init[128..136].copy_from_slice(&4.0f64.to_bits().to_le_bytes());
    init[136..144].copy_from_slice(&(-3.0f64).to_bits().to_le_bytes());
    // dppd_nan_a (+112..128) & dppd_nan_b (+144..160): lane 0 = 2.0 * 3.0, lane 1 = SNaN1 * QNaN2
    init[112..120].copy_from_slice(&2.0f64.to_bits().to_le_bytes());
    init[120..128].copy_from_slice(&0x7FF0_0000_0000_1234u64.to_le_bytes()); // SNaN1
    init[144..152].copy_from_slice(&3.0f64.to_bits().to_le_bytes());
    init[152..160].copy_from_slice(&0xFFF8_0000_0000_5678u64.to_le_bytes()); // QNaN2
    // str_a / str_b for pcmpestr*64
    init[160..176].copy_from_slice(b"hello_world_1234");
    init[176..192].copy_from_slice(b"helo!_world_5678");
    // maskmov_mask (16B)
    init[192..208].copy_from_slice(&[
        0x80, 0x00, 0xFF, 0x7F, 0x01, 0x81, 0x00, 0x80, 0x7E, 0x80, 0x00, 0x00, 0xFE, 0x7F, 0x80,
        0x00,
    ]);
    for i in 0..32usize {
        init[208 + i] = 0xC0 + (i as u8);
    }

    let mut generic_ref: Option<Vec<u8>> = None;
    run_tiers(&code, &init, |out| {
        if let Some(ref g) = generic_ref {
            assert_eq!(out, g.as_slice(), "legacy SIMD straggler tier mismatch");
        } else {
            generic_ref = Some(out.to_vec());
        }
        // Legacy phaddsw preserves ymm_hi; VEX.128 zeroes ymm_hi
        assert_eq!(&out[phaddsw_leg_off as usize + 16..phaddsw_leg_off as usize + 32], &init[224..240]);
        assert_eq!(&out[phaddsw_v128_off as usize + 16..phaddsw_v128_off as usize + 32], &[0u8; 16]);
        // phminposuw tiebreak: min=42 at lowest index 2; upper 96b zero
        let mp0 = minpos_off as usize;
        assert_eq!(u16::from_le_bytes(out[mp0..mp0 + 2].try_into().unwrap()), 42);
        assert_eq!(u16::from_le_bytes(out[mp0 + 2..mp0 + 4].try_into().unwrap()), 2);
        assert_eq!(&out[mp0 + 4..mp0 + 16], &[0u8; 12]);
        // phminposuw lane 7: min=0xFFFE at index 7
        let mp1 = mp0 + 32;
        assert_eq!(u16::from_le_bytes(out[mp1..mp1 + 2].try_into().unwrap()), 0xFFFE);
        assert_eq!(u16::from_le_bytes(out[mp1 + 2..mp1 + 4].try_into().unwrap()), 7);
        // dppd masked-off SNaN does NOT leak NaN: lane 0 = 2.0 * 3.0 = 6.0, lane 1 = 0.0
        let dn = dppd_nan_off as usize;
        assert_eq!(u64::from_le_bytes(out[dn..dn + 8].try_into().unwrap()), 6.0f64.to_bits());
        assert_eq!(u64::from_le_bytes(out[dn + 8..dn + 16].try_into().unwrap()), 0u64);
        // dppd dual-NaN: first operand SNaN1 quieted (bit 51 set -> 0x7FF8_0000_0000_1234)
        assert_eq!(
            u64::from_le_bytes(out[dn + 16..dn + 24].try_into().unwrap()),
            0x7FF8_0000_0000_1234
        );
        // pcmpestrm64 Legacy preserves ymm_hi vs VEX zeroes ymm_hi (first case at pcmp_off + 32 / +80)
        let pm_leg = pcmp_off as usize + 32;
        let pm_vex = pcmp_off as usize + 80;
        assert_eq!(&out[pm_leg + 16..pm_leg + 32], &init[224..240]);
        assert_eq!(&out[pm_vex + 16..pm_vex + 32], &[0u8; 16]);
        // (v)maskmovdqu wrote exact masked bytes
        for base in [maskmovdqu_off as usize, maskmovdqu_off as usize + 16] {
            for i in 0..16 {
                let exp = if init[192 + i] & 0x80 != 0 { init[i] } else { init[32 + i] };
                assert_eq!(out[base + i], exp, "maskmovdqu byte {i}");
            }
        }
        let _ = dppd_off;
    });

    // DPPD intermediate rounding & signed-zero test under all 4 MXCSR rounding modes
    let mut a_dp = CodeAssembler::new(64).unwrap();
    a_dp.ldmxcsr(dword_ptr(rcx)).unwrap();
    a_dp.movupd(xmm0, xmmword_ptr(rcx + 16)).unwrap();
    a_dp.movupd(xmm1, xmmword_ptr(rcx + 32)).unwrap();
    a_dp.vdppd(xmm2, xmm0, xmm1, 0x33).unwrap();
    a_dp.movupd(xmmword_ptr(rcx + 48), xmm2).unwrap();
    a_dp.vdppd(xmm2, xmm0, xmm1, 0x11).unwrap();
    a_dp.movupd(xmmword_ptr(rcx + 64), xmm2).unwrap();
    a_dp.vdppd(xmm2, xmm0, xmm1, 0x22).unwrap();
    a_dp.movupd(xmmword_ptr(rcx + 80), xmm2).unwrap();
    a_dp.ret().unwrap();
    let dp_code = a_dp.assemble(0).unwrap();
    for rc_bits in [0x0000u32, 0x2000, 0x4000, 0x6000] {
        let mut dp_init = [0u8; 96];
        dp_init[0..4].copy_from_slice(&(0x1F80 | rc_bits).to_le_bytes());
        // (-0.0 * 1.0) + (+0.0 * 1.0) -> tests signed-zero under RM (-0.0) vs RN/RP/RZ (+0.0)
        // Plus lane 0 = (1.0 + 2^-52) * (1.0 + 2^-52) when swapped in a second pass
        dp_init[16..24].copy_from_slice(&(-0.0f64).to_bits().to_le_bytes());
        dp_init[24..32].copy_from_slice(&(1.0f64 + f64::EPSILON).to_bits().to_le_bytes());
        dp_init[32..40].copy_from_slice(&1.0f64.to_bits().to_le_bytes());
        dp_init[40..48].copy_from_slice(&(-1.0f64 - f64::EPSILON).to_bits().to_le_bytes());
        let mut g_ref: Option<Vec<u8>> = None;
        run_tiers(&dp_code, &dp_init, |out| {
            if let Some(ref g) = g_ref {
                assert_eq!(out, g.as_slice(), "dppd rounding mismatch rc={rc_bits:#x}");
            } else {
                g_ref = Some(out.to_vec());
            }
        });
    }
}

/// AVX2 gather fault, partial-completion, restartable-state, `#UD` alias, and VSIB addressing
/// tests across FastOp and Live JIT ARM64 translation.
#[test]
fn avx2_gather_jit_and_aot_partial_fault_restart_and_ud() {
    use super::jit::JitCache;

    // Helper: step one gather block through either FastOp (`use_jit = false`) or Live JIT
    // (`use_jit = true`). In JIT mode, a faulting lane side-exits back to `cpu.rip` with all
    // earlier lanes already committed & masked off; we then run the FastOp at `cpu.rip` to
    // verify the fault address is delivered via `FAULT_PENDING` without re-reading completed lanes.
    let mut g = CodeAssembler::new(64).unwrap();
    g.vpgatherqq_3(xmm0, qword_ptr(rcx + xmm1 * 1), xmm2).unwrap();
    g.ret().unwrap();
    let code_qq = g.assemble(0).unwrap();

    let mut gqq256 = CodeAssembler::new(64).unwrap();
    gqq256.vpgatherqq_3(ymm0, qword_ptr(rcx + ymm1 * 1), ymm2).unwrap();
    gqq256.ret().unwrap();
    let code_qq256 = gqq256.assemble(0).unwrap();

    let mut jc = Box::new(JitCache::new());
    let mut bc = Box::new(BlockCache::new());

    for (tier_name, use_jit) in [("FastOp", false), ("JIT", true)] {
        // --- 1. 128-bit vpgatherqq: lane 0 valid, lane 1 unmapped (0x1000) ---
        let code_ptr = jit_tests::arena_alloc(code_qq.len() + 16, 16);
        let tbl_ptr = jit_tests::arena_alloc(64, 16) as *mut u64;
        unsafe {
            std::ptr::copy_nonoverlapping(code_qq.as_ptr(), code_ptr, code_qq.len());
            std::ptr::write_bytes(code_ptr.add(code_qq.len()), 0xcc, 16);
            *tbl_ptr.add(0) = 0x1122_3344_5566_7788;
            *tbl_ptr.add(1) = 0xAABB_CCDD_EEFF_0011;
        }
        let ins_qq = Decoder::with_ip(64, &code_qq, code_ptr as u64, DecoderOptions::NONE).decode();
        let fop_qq = fast::lower(&ins_qq);
        assert_eq!(fop_qq.kind, fast::K_VGATHER);
        let jit_code_qq = if use_jit {
            Some(unsafe { jc.compile_single_for_verify(code_ptr as u64, &mut *bc) }.expect("compile vpgatherqq"))
        } else {
            None
        };
        let mut cpu = Cpu::new_zeroed();
        let stack = jit_tests::arena_alloc(4096, 16);
        let guest_sp = stack as u64 + 4088;
        unsafe { (guest_sp as *mut u64).write(RET_SENTINEL) };
        cpu.gpr[4] = guest_sp;
        cpu.gpr[1] = 0; // rcx = 0 (indices hold full 64-bit addresses)
        cpu.rip = code_ptr as u64;
        cpu.xmm[0] = 0x9999_9999_8888_8888_7777_7777_6666_6666u128;
        cpu.xmm[1] = (tbl_ptr as u64 as u128) | (0x1000u128 << 64);
        cpu.xmm[2] = (1u128 << 63) | (1u128 << 127);
        FAULT_PENDING.with(|p| p.set(None));
        if let Some(jc_addr) = jit_code_qq {
            let (ex, _) = unsafe { jc.run_code_for_verify(&mut cpu, &mut *bc, None, jc_addr, 64) };
            assert_eq!(ex, jit::EXIT_SIDE, "{tier_name}: lane 1 fault must side-exit");
            assert_eq!(cpu.rip, code_ptr as u64, "{tier_name}: RIP must point at faulting gather");
            // Prove the ARM64 JIT itself already committed lane 0 & cleared lane 0's mask BEFORE side-exit!
            assert_eq!(cpu.xmm[0] as u64, 0x1122_3344_5566_7788, "{tier_name}: JIT lane 0 committed");
            assert_eq!((cpu.xmm[0] >> 64) as u64, 0x9999_9999_8888_8888, "{tier_name}: JIT lane 1 untouched");
            assert_eq!(cpu.xmm[2] as u64, 0, "{tier_name}: JIT lane 0 mask cleared");
            assert_eq!((cpu.xmm[2] >> 64) as u64, 1u64 << 63, "{tier_name}: JIT lane 1 mask preserved");
            // Poison tbl[0] BEFORE the interpreter handles the side-exited op: lane 0 must NOT be re-read!
            unsafe { *tbl_ptr.add(0) = 0xDEAD_DEAD_DEAD_DEAD };
            cpu.jx.ctx_sp = 0; // direct FastOp fault check via FAULT_PENDING
        }
        unsafe { fast::exec(&mut cpu, &fop_qq) };
        assert_eq!(
            FAULT_PENDING.with(|p| p.take()),
            Some((0x1000, false)),
            "{tier_name}: vpgatherqq lane 1 must fault at 0x1000"
        );
        assert_eq!(cpu.xmm[0] as u64, 0x1122_3344_5566_7788, "{tier_name}: lane 0 committed");
        assert_eq!((cpu.xmm[0] >> 64) as u64, 0x9999_9999_8888_8888, "{tier_name}: lane 1 untouched");
        assert_eq!(cpu.xmm[2] as u64, 0, "{tier_name}: lane 0 mask cleared");
        assert_eq!((cpu.xmm[2] >> 64) as u64, 1u64 << 63, "{tier_name}: lane 1 mask preserved");

        // Poison tbl[0], repair lane 1 index -> &tbl[1], and restart the exact same gather!
        unsafe { *tbl_ptr.add(0) = 0xDEAD_DEAD_DEAD_DEAD };
        cpu.xmm[1] = (tbl_ptr as u64 as u128) | ((unsafe { tbl_ptr.add(1) } as u64 as u128) << 64);
        if let Some(jc_addr) = jit_code_qq {
            let (ex, _) = unsafe { jc.run_code_for_verify(&mut cpu, &mut *bc, None, jc_addr, 64) };
            assert_ne!(ex, jit::EXIT_SIDE, "{tier_name}: restarted gather must complete cleanly");
        } else {
            unsafe { fast::exec(&mut cpu, &fop_qq) };
        }
        assert_eq!(FAULT_PENDING.with(|p| p.take()), None, "{tier_name}: restart must not fault");
        assert_eq!(
            cpu.xmm[0] as u64, 0x1122_3344_5566_7788,
            "{tier_name}: restarted gather must NOT re-read already-completed lane 0"
        );
        assert_eq!((cpu.xmm[0] >> 64) as u64, 0xAABB_CCDD_EEFF_0011, "{tier_name}: lane 1 loaded");
        assert_eq!(cpu.xmm[2], 0, "{tier_name}: mask zeroed on completion");
        assert_eq!(cpu.ymm_hi[2], 0, "{tier_name}: mask ymm_hi zeroed on completion");

        // --- 2. 256-bit vpgatherqq faulting in upper 128-bit half (lane 2 of 4) ---
        let code_ptr2 = jit_tests::arena_alloc(code_qq256.len() + 16, 16);
        let tbl = jit_tests::arena_alloc(64, 16) as *mut u64;
        unsafe {
            std::ptr::copy_nonoverlapping(code_qq256.as_ptr(), code_ptr2, code_qq256.len());
            std::ptr::write_bytes(code_ptr2.add(code_qq256.len()), 0xcc, 16);
            for i in 0..4u64 {
                *tbl.add(i as usize) = 0xCAFE_0000_0000_0000 + i;
            }
        }
        let ins_qq256 = Decoder::with_ip(64, &code_qq256, code_ptr2 as u64, DecoderOptions::NONE).decode();
        let fop_qq256 = fast::lower(&ins_qq256);
        assert_eq!(fop_qq256.kind, fast::K_VGATHER);
        let jit_code_qq256 = if use_jit {
            Some(unsafe { jc.compile_single_for_verify(code_ptr2 as u64, &mut *bc) }.expect("compile vpgatherqq256"))
        } else {
            None
        };
        let mut cpu = Cpu::new_zeroed();
        let guest_sp2 = stack as u64 + 4088;
        unsafe { (guest_sp2 as *mut u64).write(RET_SENTINEL) };
        cpu.gpr[4] = guest_sp2;
        cpu.gpr[1] = 0;
        cpu.rip = code_ptr2 as u64;
        cpu.xmm[0] = 0x1111_1111_1111_1111_2222_2222_2222_2222u128;
        cpu.ymm_hi[0] = 0x3333_3333_3333_3333_4444_4444_4444_4444u128;
        // Lane 0: valid, Lane 1: masked off, Lane 2 (ymm_hi lo): 0x1000 FAULT, Lane 3: valid
        cpu.xmm[1] = (tbl as u64 as u128) | (0x2000u128 << 64);
        cpu.ymm_hi[1] = 0x1000u128 | ((unsafe { tbl.add(3) } as u64 as u128) << 64);
        cpu.xmm[2] = 1u128 << 63; // lane 0 active, lane 1 masked off
        cpu.ymm_hi[2] = (1u128 << 63) | (1u128 << 127); // lanes 2 & 3 active
        FAULT_PENDING.with(|p| p.set(None));
        if let Some(jc_addr) = jit_code_qq256 {
            let (ex, _) = unsafe { jc.run_code_for_verify(&mut cpu, &mut *bc, None, jc_addr, 64) };
            assert_eq!(ex, jit::EXIT_SIDE, "{tier_name}: ymm lane 2 must side-exit");
            assert_eq!(cpu.rip, code_ptr2 as u64);
            assert_eq!(cpu.xmm[0] as u64, 0xCAFE_0000_0000_0000, "{tier_name}: JIT ymm lane 0 committed");
            assert_eq!((cpu.xmm[0] >> 64) as u64, 0x1111_1111_1111_1111, "{tier_name}: JIT masked lane 1 preserved");
            assert_eq!(cpu.ymm_hi[0] as u64, 0x4444_4444_4444_4444, "{tier_name}: JIT faulting lane 2 untouched");
            assert_eq!(cpu.xmm[2], 0, "{tier_name}: JIT lanes 0..1 masks cleared");
            assert_eq!(cpu.ymm_hi[2], (1u128 << 63) | (1u128 << 127), "{tier_name}: JIT lanes 2..3 masks preserved");
            unsafe { *tbl.add(0) = 0xBAD0_BAD0_BAD0_BAD0 };
            cpu.jx.ctx_sp = 0; // direct FastOp fault check via FAULT_PENDING
        }
        unsafe { fast::exec(&mut cpu, &fop_qq256) };
        assert_eq!(FAULT_PENDING.with(|p| p.take()), Some((0x1000, false)), "{tier_name}: ymm lane 2 fault");
        // Fix lane 2 and poison lane 0, then restart!
        unsafe { *tbl.add(0) = 0xBAD0_BAD0_BAD0_BAD0 };
        cpu.ymm_hi[1] = (unsafe { tbl.add(2) } as u64 as u128) | ((unsafe { tbl.add(3) } as u64 as u128) << 64);
        if let Some(jc_addr) = jit_code_qq256 {
            let (ex, _) = unsafe { jc.run_code_for_verify(&mut cpu, &mut *bc, None, jc_addr, 64) };
            assert_ne!(ex, jit::EXIT_SIDE);
        } else {
            unsafe { fast::exec(&mut cpu, &fop_qq256) };
        }
        assert_eq!(FAULT_PENDING.with(|p| p.take()), None);
        assert_eq!(cpu.xmm[0] as u64, 0xCAFE_0000_0000_0000);
        assert_eq!(cpu.ymm_hi[0] as u64, 0xCAFE_0000_0000_0002);
        assert_eq!((cpu.ymm_hi[0] >> 64) as u64, 0xCAFE_0000_0000_0003);
        assert_eq!(cpu.xmm[2], 0);
        assert_eq!(cpu.ymm_hi[2], 0);
    }

    // Part 2: #UD alias check directly in FastOp (`d==m`, `d==v`, `m==v`)
    let table32 = [0x1122_3344u32, 0x5566_7788];
    for &(dst, idx, msk) in &[(xmm0, xmm1, xmm0), (xmm0, xmm0, xmm2), (xmm0, xmm1, xmm1)] {
        let mut ga = CodeAssembler::new(64).unwrap();
        ga.vpgatherdd_3(dst, dword_ptr(rcx + idx * 4), msk).unwrap();
        let bytes = ga.assemble(0x2000).unwrap();
        let ins = Decoder::with_ip(64, &bytes, 0x2000, DecoderOptions::NO_INVALID_CHECK).decode();
        let fop = fast::lower(&ins);
        assert_eq!(fop.kind, fast::K_VGATHER);
        let mut cpu = Cpu::new_zeroed();
        cpu.gpr[1] = table32.as_ptr() as u64;
        cpu.xmm[0] = 0xDEAD_BEEF;
        cpu.xmm[1] = 0;
        cpu.xmm[2] = u128::MAX;
        FAULT_PENDING.with(|p| p.set(None));
        unsafe { fast::exec(&mut cpu, &fop) };
        assert_eq!(FAULT_PENDING.with(|p| p.take()), Some((u64::MAX, false)), "FastOp aliasing must fault #UD");
        assert_eq!(cpu.xmm[0], 0xDEAD_BEEF, "FastOp aliasing must not modify dest");
    }

    // Part 3: VSIB scale (1/2/4/8), negative disp, negative 32-bit sign-extended index across all 4 tiers.
    let mut vs = CodeAssembler::new(64).unwrap();
    vs.lea(rdx, ptr(rcx + 128)).unwrap();
    vs.vmovdqu(ymm1, ymmword_ptr(rcx)).unwrap(); // i32 indices: [-4, -1, 0, 2, -2, 1, 3, -3]
    for (idx, scale) in [(0, 1u32), (1, 2), (2, 4), (3, 8)] {
        vs.vmovdqu(ymm0, ymmword_ptr(rcx + 32)).unwrap();
        vs.vmovdqu(ymm3, ymmword_ptr(rcx + 64)).unwrap();
        match scale {
            1 => vs.vpgatherdd_3(ymm0, dword_ptr(rdx + ymm1 * 1 - 4), ymm3).unwrap(),
            2 => vs.vpgatherdd_3(ymm0, dword_ptr(rdx + ymm1 * 2 - 4), ymm3).unwrap(),
            4 => vs.vpgatherdd_3(ymm0, dword_ptr(rdx + ymm1 * 4 - 4), ymm3).unwrap(),
            _ => vs.vpgatherdd_3(ymm0, dword_ptr(rdx + ymm1 * 8 - 4), ymm3).unwrap(),
        }
        vs.vmovdqu(ymmword_ptr(rcx + 256 + idx * 32), ymm0).unwrap();
    }
    vs.vzeroupper().unwrap();
    vs.ret().unwrap();
    let vs_code = vs.assemble(0).unwrap();
    let mut vs_init = vec![0u8; 512];
    for (i, v) in [-4i32, -1, 0, 2, -2, 1, 3, -3].into_iter().enumerate() {
        vs_init[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes());
    }
    for i in 0..32usize {
        vs_init[32 + i] = 0x5A;
        vs_init[64 + i] = 0x80; // all MSBs set -> all lanes active
    }
    for (i, b) in vs_init.iter_mut().enumerate().take(192).skip(96) {
        *b = ((i.wrapping_mul(37)) ^ 0xA5) as u8;
    }
    let mut vs_ref: Option<Vec<u8>> = None;
    run_tiers(&vs_code, &vs_init, |out| {
        if let Some(ref g) = vs_ref {
            assert_eq!(out, g.as_slice(), "VSIB scale/disp/neg-idx tier mismatch");
        } else {
            vs_ref = Some(out.to_vec());
        }
    });
}

/// Regression test for `XA` (`x16`) address-cache invalidation when MMX helpers (`movq mm, [b+i]`,
/// `movq [b+i], mm`, `paddd mm, [b+i]`, `emms`, `maskmovq`) or `vpgather*` overwrite `XA` in the
/// same translated block as indexed `[rcx + rdx]` loads/stores.
#[test]
fn mmx_and_gather_invalidate_xa_address_cache_in_same_block() {
    let mut a = CodeAssembler::new(64).unwrap();
    a.mov(rdx, 16u64).unwrap();
    // 1. movq mm0, [rcx+rdx] populates xa_key=(rcx, rdx, 0, 0) in load_op, then overwrites XA
    //    with X87State in store_mmx_d_and_tag. The immediately following indexed load/store MUST
    //    NOT reuse stale XA (which points at X87State instead of [rcx+rdx]!).
    a.movq(mm0, qword_ptr(rcx + rdx)).unwrap();
    a.mov(rax, qword_ptr(rcx + rdx)).unwrap();
    a.mov(qword_ptr(rcx + 64), rax).unwrap();
    // 2. movq [rcx+rdx], mm1 populates xa_key in store_op, then emit_mmx_tag_valid overwrites XA.
    a.movq(mm1, qword_ptr(rcx + 8)).unwrap();
    a.movq(qword_ptr(rcx + rdx), mm1).unwrap();
    a.mov(r8, 0x1234_5678_9ABC_DEF0u64 as i64).unwrap();
    a.mov(qword_ptr(rcx + rdx), r8).unwrap(); // must store to guest [rcx+16], NOT corrupt X87State!
    a.movq(qword_ptr(rcx + 72), mm0).unwrap(); // verify X87State::regs[0] was NOT corrupted!
    // 3. Indexed load followed by emms + indexed load
    a.mov(r9, qword_ptr(rcx + rdx)).unwrap();
    a.emms().unwrap();
    a.mov(r10, qword_ptr(rcx + rdx)).unwrap();
    a.mov(qword_ptr(rcx + 80), r9).unwrap();
    a.mov(qword_ptr(rcx + 88), r10).unwrap();
    // 4. Indexed load followed by vpgatherdd + indexed load
    a.vpxor(xmm1, xmm1, xmm1).unwrap();
    a.vpcmpeqd(xmm2, xmm2, xmm2).unwrap();
    a.mov(rax, qword_ptr(rcx + rdx)).unwrap();
    a.vpgatherdd_3(xmm0, dword_ptr(rcx + xmm1 * 4), xmm2).unwrap();
    a.mov(r11, qword_ptr(rcx + rdx)).unwrap();
    a.mov(qword_ptr(rcx + 96), rax).unwrap();
    a.mov(qword_ptr(rcx + 104), r11).unwrap();
    a.vzeroupper().unwrap();
    a.ret().unwrap();
    let code = a.assemble(0).unwrap();

    let mut init = vec![0u8; 128];
    init[0..8].copy_from_slice(&0x1111_2222_3333_4444u64.to_le_bytes());
    init[8..16].copy_from_slice(&0x5555_6666_7777_8888u64.to_le_bytes());
    init[16..24].copy_from_slice(&0xAAAA_BBBB_CCCC_DDDDu64.to_le_bytes());

    struct KnobsReset;
    impl Drop for KnobsReset {
        fn drop(&mut self) {
            jit::test_set_knobs(jit::TestKnobs::NONE);
        }
    }
    let _reset = KnobsReset;
    jit::test_set_knobs(jit::TestKnobs {
        hw_range: Some(true),
        lse2_trap: Some(true),
        ..jit::TestKnobs::NONE
    });

    let mut generic_ref: Option<Vec<u8>> = None;
    run_tiers(&code, &init, |out| {
        if let Some(ref g) = generic_ref {
            assert_eq!(out, g.as_slice(), "XA address cache stale after MMX/gather");
        } else {
            generic_ref = Some(out.to_vec());
        }
        let rd64 = |off: usize| u64::from_le_bytes(out[off..off + 8].try_into().unwrap());
        assert_eq!(rd64(64), 0xAAAA_BBBB_CCCC_DDDD);
        assert_eq!(rd64(72), 0xAAAA_BBBB_CCCC_DDDD);
        assert_eq!(rd64(16), 0x1234_5678_9ABC_DEF0);
        assert_eq!(rd64(80), 0x1234_5678_9ABC_DEF0);
        assert_eq!(rd64(88), 0x1234_5678_9ABC_DEF0);
        assert_eq!(rd64(96), 0x1234_5678_9ABC_DEF0);
        assert_eq!(rd64(104), 0x1234_5678_9ABC_DEF0);
    });
}
