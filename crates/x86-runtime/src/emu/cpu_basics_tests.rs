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
            assert_eq!(c & ((1 << 5) | (1 << 28) | (1 << 30)), 0); // VMX, AVX, RDRAND
            assert_eq!(c >> 25 & 1, u32::from(features::host().has(H::AES)));
            assert_eq!(c >> 1 & 1, u32::from(features::host().has(H::PMULL)));
            assert_eq!(c >> 20 & 1, u32::from(features::host().has(H::CRC32)));
            assert_eq!(cpuid::query(7, 0)[1] >> 29 & 1, u32::from(features::host().has(H::SHA1 | H::SHA256)));
            assert_eq!(cpuid::query(7, 0)[1] & ((1 << 5) | (1 << 7) | (1 << 10) | (1 << 18)), 0);
            assert_eq!(cpuid::query(7, 1), [0; 4]);
            assert_eq!(cpuid::query(0x8000_0001, 0)[2] & (1 << 6), 0);
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
