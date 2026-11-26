//! Tests for the A64 emitter and JIT cache.

mod opcode_encodings;
mod x87_tests;

use super::cache::{jset_base, MAX_LSE2_REGIONS, NOJIT};
use super::*;
use crate::emu::exec_mem::{
    bind_static_aot_module, jit_writable, register_static_aot_desc, register_static_aot_module,
    static_aot_gen, test_reset_static_lookups, test_static_lookups,
    unregister_static_aot_for_test, StaticAotEntry, StaticAotModuleDesc,
    STATIC_AOT_DESC_MAGIC, STATIC_AOT_DESC_VERSION, STATIC_AOT_FLAG_HOST_CALLABLE,
};
use crate::emu::BlockCache;

#[test]
fn timestamp_block_runs_native_scaled_and_frozen_paths() {
    use crate::emu::{jit_tests::arena_alloc, tsc};

    fn raw_count() -> u64 {
        let value: u64;
        unsafe { core::arch::asm!("isb", "mrs {}, cntvct_el0", out(reg) value, options(nostack, preserves_flags)) };
        value
    }

    let mut cache = JitCache::new();
    let mut blocks = Box::new(BlockCache::new());
    let frequency = tsc::counter_frequency();
    // Each probe reads once, then jumps to a distinct, unlinked block.
    let probes: [&[u8]; 2] = [
        &[0x0f, 0x31, 0xeb, 0x01, 0xcc, 0xcc],
        &[0x0f, 0x01, 0xf9, 0xeb, 0x01, 0xcc, 0xcc],
    ];
    for probe in probes {
        let ip = arena_alloc(32, 16);
        unsafe { core::ptr::copy_nonoverlapping(probe.as_ptr(), ip, probe.len()) };
        let code = unsafe { cache.compile_single_for_verify(ip as u64, &mut *blocks) }.unwrap();
        for mode in [
            TscMode::Native, TscMode::Scaled, TscMode::From24Mhz, TscMode::Frozen,
            TscMode::NativeEcv, TscMode::ScaledEcv, TscMode::From24MhzEcv,
        ] {
            if matches!(mode, TscMode::NativeEcv | TscMode::ScaledEcv | TscMode::From24MhzEcv) && !tsc::has_ecv() {
                continue;
            }
            cache.live_tsc_mode = mode;
            let mut cpu = Cpu::new_zeroed();
            cpu.rip = ip as u64;
            let before = raw_count();
            unsafe { cache.run_code_for_verify(&mut cpu, &mut *blocks, None, code, 16) };
            let after = raw_count();
            let stamp = cpu.gpr[2] << 32 | cpu.gpr[0];
            let expected = |count| match mode.for_run() {
                TscMode::Frozen => tsc::GUEST_TSC,
                TscMode::Scaled | TscMode::ScaledEcv => tsc::scale_counter_to_tsc(count, frequency),
                TscMode::From24Mhz | TscMode::From24MhzEcv => tsc::scale_counter_to_tsc(count, 24_000_000),
                _ => count,
            };
            assert!((expected(before)..=expected(after)).contains(&stamp), "{mode:?} timestamp {stamp}");
        }
    }
}

#[test]
fn static_only_negative_cache_and_rebind_invalidation() {
    let entries_v1 = [
        StaticAotEntry { rva: 0x1000, code_offset: 0x100, flags: 0 },
        StaticAotEntry { rva: 0x2000, code_offset: 0x200, flags: STATIC_AOT_FLAG_HOST_CALLABLE },
    ];
    register_static_aot_module("jit_neg_test.dll", 0x7_0000_0000, &entries_v1);
    assert_eq!(bind_static_aot_module("jit_neg_test.dll", 0x3_0000_0000, 0x1_0000), 2);

    let mut bc = Box::new(BlockCache::new());
    let mut jc = JitCache::new();
    jc.test_set_static_only(true);
    assert!(jc.meta.is_empty(), "static_only must skip the 4 MiB meta slab");
    let m_static = jc.mode_tag();
    assert_ne!(m_static, 0);

    let hit_ip = 0x3_0000_1000u64;
    let host_ip = 0x3_0000_2000u64;
    let miss_ip = 0x3_0000_3000u64;

    unsafe {
        test_reset_static_lookups();
        // Positive hit populates index; repeat lookups do not hit registry again.
        assert_eq!(jc.lookup(hit_ip, &mut *bc), Some(0x7_0000_0100));
        assert_eq!(test_static_lookups(), 1);
        assert_eq!(jc.lookup(hit_ip, &mut *bc), Some(0x7_0000_0100));
        assert_eq!(test_static_lookups(), 1);
        assert!(!jc.is_interp_sticky(hit_ip));

        // Host-callable entry caches code != 0 with fast == 0.
        assert_eq!(jc.lookup(host_ip, &mut *bc), Some(0x7_0000_0200));
        assert_eq!(test_static_lookups(), 2);
        let h_ent = jc.index[jset_base(host_ip)];
        assert_eq!(h_ent.ip, host_ip);
        assert_eq!(h_ent.code, 0x7_0000_0200);
        assert_eq!(h_ent.fast, 0);

        // Miss populates a NOJIT negative entry; repeated lookups MUST NOT re-query registry.
        assert_eq!(jc.lookup(miss_ip, &mut *bc), None);
        assert_eq!(test_static_lookups(), 3);
        let neg = jc.index[jset_base(miss_ip)];
        assert_eq!(neg.ip, miss_ip);
        assert_eq!(neg.code, 0);
        assert_eq!(neg.fast, 0);
        assert_eq!(neg.hits, NOJIT);
        assert!(jc.is_interp_sticky(miss_ip));

        for _ in 0..16 {
            assert_eq!(jc.lookup(miss_ip, &mut *bc), None);
        }
        assert_eq!(
            test_static_lookups(),
            3,
            "negative cache must eliminate repeated lookup_static_aot calls"
        );

        // Rebinding a new module containing `miss_ip` bumps STATIC_AOT_GEN, invalidates
        // `is_interp_sticky`, flushes the negative cache, and resolves the newly bound block.
        let g_before = static_aot_gen();
        let entries_v2 = [
            StaticAotEntry { rva: 0x1000, code_offset: 0x100, flags: 0 },
            StaticAotEntry { rva: 0x3000, code_offset: 0x300, flags: 0 },
        ];
        register_static_aot_module("jit_neg_test.dll", 0x7_1000_0000, &entries_v2);
        assert_eq!(bind_static_aot_module("jit_neg_test.dll", 0x3_0000_0000, 0x1_0000), 2);
        assert_ne!(static_aot_gen(), g_before);
        assert!(!jc.is_interp_sticky(miss_ip));

        assert_eq!(jc.lookup(miss_ip, &mut *bc), Some(0x7_1000_0300));
        assert_eq!(test_static_lookups(), 4);
        assert!(!jc.is_interp_sticky(miss_ip));

        // Unbinding bumps generation and flushes cached entries back to None.
        unregister_static_aot_for_test("jit_neg_test.dll");
        assert_eq!(jc.lookup(miss_ip, &mut *bc), None);
        assert_eq!(test_static_lookups(), 5);
        assert!(jc.is_interp_sticky(miss_ip));
    }
}

#[test]
fn no_jit_transitions_to_static_only_on_late_bind_and_skips_meta() {
    let mut bc = Box::new(BlockCache::new());
    let mut jc = JitCache::new();
    // Simulate `!jit_writable()` with no static AOT module bound yet.
    jc.no_jit = true;
    jc.static_aot_gen = static_aot_gen();
    let mode_before = jc.mode_tag();

    let ip = 0x3_8000_1000u64;
    assert!(jc.is_interp_sticky(ip));
    unsafe {
        test_reset_static_lookups();
        // Steady-state no_jit without static modules must not touch static registry or init().
        for _ in 0..8 {
            assert_eq!(jc.lookup(ip, &mut *bc), None);
        }
        assert_eq!(test_static_lookups(), 0);
        assert!(jc.mem.is_null());
        assert!(!jc.static_only);
    }

    // Bind a static module with a signed enter/lookup_rip stub descriptor.
    let name = std::ffi::CString::new("late_bind.dll").unwrap();
    let entries = [StaticAotEntry { rva: 0x1000, code_offset: 0x800, flags: 0 }];
    let desc = StaticAotModuleDesc {
        magic: STATIC_AOT_DESC_MAGIC,
        version: STATIC_AOT_DESC_VERSION,
        knobs: 0,
        image_base: 0x3_8000_0000,
        image_size: 0x1_0000,
        image_hash: [1, 2],
        code_base: 0x8_0000_0000,
        code_len: 0x4000,
        enter_off: 0x40,
        lookup_rip_off: 0x100,
        flags: 0,
        entry_count: 1,
        _pad: 0,
        entries: entries.as_ptr(),
        module_name: name.as_ptr(),
    };
    assert!(unsafe { register_static_aot_desc(&desc) });
    assert_eq!(bind_static_aot_module("late_bind.dll", 0x3_8000_0000, 0x1_0000), 1);

    // Generation bumped: `is_interp_sticky` drops immediately, and `lookup` transitions
    // `JitCache` into `static_only` without allocating `self.meta`.
    assert!(!jc.is_interp_sticky(ip));
    unsafe {
        assert_eq!(jc.lookup(ip, &mut *bc), Some(0x8_0000_0800));
        assert!(jc.static_only);
        assert!(jc.meta.is_empty());
        assert_eq!(jc.enter, 0x8_0000_0040);
        assert_eq!(jc.lookup_rip, 0x8_0000_0100);
        assert_ne!(jc.mode_tag(), mode_before);
    }

    // Unbinding drops back out of `static_only` cleanly.
    unregister_static_aot_for_test("late_bind.dll");
    unsafe {
        assert_eq!(jc.lookup(ip, &mut *bc), None);
        assert!(!jc.static_only);
        assert_eq!(jc.enter, 0);
        assert!(jc.is_interp_sticky(ip));
    }
}

#[test]
fn lse2_regions_recycle_and_jit_cache_drop_reclaims_slots() {
    // 1. Direct register/unregister cycle well past MAX_LSE2_REGIONS (128).
    let mut first_batch = Vec::new();
    for i in 0..8usize {
        let base = 0x5_0000_0000usize + i * 0x10_0000;
        let slot = register_lse2_region(base, base + 0x8_0000, base + 0x8_0000, base + 0xC_0000)
            .expect("initial LSE2 slot available");
        first_batch.push(slot);
    }
    for &slot in &first_batch {
        unregister_lse2_region(slot);
    }
    // Repeatedly register and unregister more times than MAX_LSE2_REGIONS; every iteration must succeed.
    for i in 0..(MAX_LSE2_REGIONS + 64) {
        let base = 0x5_1000_0000usize + (i & 0xFF) * 0x10_0000;
        let slot = register_lse2_region(base, base + 0x8_0000, base + 0x8_0000, base + 0xC_0000)
            .expect("LSE2 slot must be recycled after unregister");
        unregister_lse2_region(slot);
    }
    // Re-claiming 8 slots must reuse slots within `0..MAX_LSE2_REGIONS`.
    let mut second_batch = Vec::new();
    for i in 0..8usize {
        let base = 0x5_2000_0000usize + i * 0x10_0000;
        let slot = register_lse2_region(base, base + 0x8_0000, base + 0x8_0000, base + 0xC_0000)
            .expect("recycled batch slot available");
        assert!(slot < MAX_LSE2_REGIONS);
        second_batch.push(slot);
    }
    for &slot in &second_batch {
        unregister_lse2_region(slot);
    }

    // 2. Creating, warming, and dropping > MAX_LSE2_REGIONS JitCache instances must reclaim
    //    both MAP_JIT buffers and LSE2_REGIONS slots on every Drop.
    if jit_writable() {
        for iter in 0..(MAX_LSE2_REGIONS + 16) {
            let mut jc = JitCache::new();
            unsafe { jc.warm() };
            assert!(!jc.mem.is_null(), "iter {iter}: MAP_JIT allocation failed (leak?)");
            assert!(
                jc.lse2_slot.is_some(),
                "iter {iter}: LSE2_REGIONS slot exhausted (Drop failed to unregister)"
            );
            drop(jc);
        }
    }
}

#[test]
fn sys_state_arm64_emitter_encodes_all_ops_with_zero_relocs() {
    use super::cache::{new_tr, Prep, StubAddrs, TrScratch};
    use super::emit::Asm;
    use super::lazy::Edges;
    use crate::emu::fast::{lower, FastOp};
    use crate::emu::features::{without_features, HostFeatures};
    use crate::emu::BLK_MAX_INSNS;
    use iced_x86::code_asm::*;
    use iced_x86::{Decoder, DecoderOptions};

    let mut a = CodeAssembler::new(64).unwrap();
    a.cpuid().unwrap();
    a.xgetbv().unwrap();
    a.fldcw(word_ptr(rcx)).unwrap();
    a.fnstcw(word_ptr(rcx)).unwrap();
    a.fstcw(word_ptr(rcx)).unwrap();
    a.ldmxcsr(dword_ptr(rcx)).unwrap();
    a.vldmxcsr(dword_ptr(rcx)).unwrap();
    a.stmxcsr(dword_ptr(rcx)).unwrap();
    a.vstmxcsr(dword_ptr(rcx)).unwrap();
    a.fxsave(ptr(rcx)).unwrap();
    a.fxsave64(ptr(rcx)).unwrap();
    a.fxrstor(ptr(rcx)).unwrap();
    a.fxrstor64(ptr(rcx)).unwrap();
    a.xsave(ptr(rcx)).unwrap();
    a.xsave64(ptr(rcx)).unwrap();
    a.xsaveopt(ptr(rcx)).unwrap();
    a.xsaveopt64(ptr(rcx)).unwrap();
    a.xsavec(ptr(rcx)).unwrap();
    a.xsavec64(ptr(rcx)).unwrap();
    a.xrstor(ptr(rcx)).unwrap();
    a.xrstor64(ptr(rcx)).unwrap();
    let bytes = a.assemble(0x1_0000_0000).unwrap();

    let stubs = StubAddrs {
        exit: 0x1000,
        lookup_rip: 0x1100,
        lookup_jmp: 0x1200,
        lookup_call: 0x1300,
        host_xfer: 0x1400,
        ras_miss: 0x1500,
        spill_xmm: 0x1600,
        reload_xmm: 0x1700,
        clear_ymm_hi: 0x1800,
        rdtsc_slow: 0x1900,
        rdtscp_slow: 0x1a00,
        lookup_rip_ic: 0x1104,
        lookup_jmp_ic: 0x1204,
        lookup_call_ic: 0x1304,
        ras_miss_ic: 0x1504,
    };

    for disable_mask in [0u8, HostFeatures::AFP] {
        without_features(disable_mask, || {
            let mut scratch = TrScratch::new();
            let mut dec = Decoder::with_ip(64, &bytes, 0x1_0000_0000, DecoderOptions::NONE);
            let mut count = 0usize;
            while dec.can_decode() {
                let ins = dec.decode();
                let op = lower(&ins);
                assert!(supported(&op), "expected JIT/AOT support for `{ins}` (mask={disable_mask})");
                let mut prep = Prep {
                    ops: [FastOp::default(); BLK_MAX_INSNS],
                    k: 1,
                    host: false,
                    wr: [0; BLK_MAX_INSNS],
                    term: Term::Fall(ins.next_ip()),
                    target_host: false,
                    lazy: false,
                    edge: Edges { live: [0; 2], kill: [0; 2] },
                    live_out: 0,
                };
                prep.ops[0] = op;
                let mut buf = vec![0u32; 512];
                let asm = Asm::new(buf.as_mut_ptr(), 0x2000, buf.len());
                let mut tr = new_tr(asm, &prep.ops[..1], &prep, &stubs, true, &mut scratch);
                let main = tr.translate(prep.term, 0);
                assert!(!tr.a.overflow, "buffer overflow translating `{ins}`");
                assert!(main > 0 && tr.a.pos > 0, "empty translation for `{ins}`");
                assert!(
                    tr.relocs.is_empty(),
                    "`{ins}` emitted runtime relocs ({:?}), breaking static Mach-O AOT",
                    tr.relocs
                );
                let words = &buf[..tr.a.pos];
                match ins.mnemonic() {
                    iced_x86::Mnemonic::Cpuid => {
                        let want_ldr = 0xF940_0000 | ((OFF_SYS_HELPERS / 8) << 10) | (XCPU << 5) | XA;
                        assert!(words.contains(&want_ldr), "cpuid must load OFF_SYS_HELPERS");
                        assert!(words.contains(&(0xD63F_0000 | (XA << 5))), "cpuid must emit blr x16");
                    }
                    iced_x86::Mnemonic::Xgetbv => {
                        assert!(
                            words.iter().any(|&w| (w & 0xFF00_001F) == (0x3500_0000 | g(1))),
                            "xgetbv must check ecx != 0 with cbnz w20"
                        );
                        let want_xcr0 = 0x5280_0000 | (crate::emu::cpuid::XCR0 << 5) | g(0);
                        assert!(words.contains(&want_xcr0), "xgetbv must load XCR0 into w19");
                    }
                    iced_x86::Mnemonic::Ldmxcsr | iced_x86::Mnemonic::Vldmxcsr => {
                        let want_str = 0xB900_0000 | ((OFF_MXCSR / 4) << 10) | (XCPU << 5) | T0;
                        assert!(words.contains(&want_str), "{ins} must store OFF_MXCSR");
                        assert!(words.contains(&(0xD53B_4400 | T2)), "{ins} must read FPCR via mrs");
                        assert!(words.contains(&(0xD51B_4400 | T1)), "{ins} must write FPCR via msr");
                    }
                    iced_x86::Mnemonic::Fldcw | iced_x86::Mnemonic::Fnstcw | iced_x86::Mnemonic::Fstcw => {
                        let want_ldr = 0xF940_0000 | ((OFF_X87_CW / 8) << 10) | (XCPU << 5) | T1;
                        assert!(words.contains(&want_ldr), "{ins} must load OFF_X87_CW");
                    }
                    iced_x86::Mnemonic::Stmxcsr | iced_x86::Mnemonic::Vstmxcsr => {
                        let want_ldr = 0xB940_0000 | ((OFF_MXCSR / 4) << 10) | (XCPU << 5) | T0;
                        assert!(words.contains(&want_ldr), "{ins} must load OFF_MXCSR");
                    }
                    iced_x86::Mnemonic::Wait => {
                        // `fstcw` assembles as `fwait` (9B) + `fnstcw`; `fwait` is a NOP on x86-64.
                    }
                    _ => {
                        let want_ldr = 0xF940_0000 | ((OFF_SYS_HELPERS / 8) << 10) | (XCPU << 5) | XA;
                        assert!(words.contains(&want_ldr), "{ins} must load OFF_SYS_HELPERS");
                        assert!(words.contains(&(0xD63F_0000 | (XA << 5))), "{ins} must emit blr x16");
                    }
                }
                tr.return_scratch(&mut scratch);
                count += 1;
            }
            // 21 assembled ops (`fstcw` emits `fwait` + `fnstcw` = 22 decoded instructions).
            assert_eq!(count, 22);
        });
    }
}

#[test]
fn indirect_branch_ic_emitter_encodes_zero_relocs_and_uses_ic_table() {
    use super::cache::{new_tr, Prep, StubAddrs, TrScratch};
    use super::emit::Asm;
    use super::lazy::Edges;
    use super::stubs::{emit_stubs_cfg, StubCfg};
    use crate::emu::fast::{lower, FastOp};
    use crate::emu::BLK_MAX_INSNS;
    use iced_x86::code_asm::*;
    use iced_x86::{Decoder, DecoderOptions};

    // 1. Verify shared stub table invariants for IC entry points.
    for ras in [false, true] {
        let mut sbuf = vec![0u32; AOT_STUBS_BYTES / 4];
        let mut sa = Asm::new(sbuf.as_mut_ptr(), 0x1000, sbuf.len());
        let mut cfg = StubCfg::live(ras);
        cfg.static_pic = false;
        let (st, _) = emit_stubs_cfg(&mut sa, cfg);
        assert!(!sa.overflow, "stubs overflowed AOT_STUBS_BYTES with ras={ras}");
        assert_eq!(st.lookup_rip_ic, st.lookup_rip + 1);
        assert_eq!(st.lookup_jmp_ic, st.lookup_jmp + 1);
        assert_eq!(st.lookup_call_ic, st.lookup_call + 1);
        if ras {
            assert_eq!(st.ras_miss_ic, st.ras_miss.map(|p| p + 1));
        } else {
            assert_eq!(st.ras_miss_ic, None);
        }
        // First word of each non-IC entry must be `movz x4, #0` (`0xD280_0004`).
        assert_eq!(sbuf[st.lookup_rip], 0xD280_0004);
        assert_eq!(sbuf[st.lookup_jmp], 0xD280_0004);
        assert_eq!(sbuf[st.lookup_call], 0xD280_0004);
        if let Some(rm) = st.ras_miss {
            assert_eq!(sbuf[rm], 0xD280_0004);
        }
    }

    // 2. Verify in-block IC codegen for CallR, CallM, JmpR, JmpM, Ret (RAS on & off, IC on & off).
    let mut a = CodeAssembler::new(64).unwrap();
    a.call(rax).unwrap();
    a.call(qword_ptr(rcx)).unwrap();
    a.jmp(rax).unwrap();
    a.jmp(qword_ptr(rcx)).unwrap();
    a.ret().unwrap();
    let bytes = a.assemble(0x1_0000_0000).unwrap();

    let stubs_ras = StubAddrs {
        exit: 0x1000,
        lookup_rip: 0x1100,
        lookup_jmp: 0x1200,
        lookup_call: 0x1300,
        lookup_rip_ic: 0x1104,
        lookup_jmp_ic: 0x1204,
        lookup_call_ic: 0x1304,
        host_xfer: 0x1400,
        ras_miss: 0x1500,
        ras_miss_ic: 0x1504,
        spill_xmm: 0x1600,
        reload_xmm: 0x1700,
        clear_ymm_hi: 0x1800,
        rdtsc_slow: 0x1900,
        rdtscp_slow: 0x1a00,
    };
    let stubs_noras = StubAddrs { ras_miss: 0, ras_miss_ic: 0, ..stubs_ras };

    let want_ldr_ic_xa = 0xF940_0000 | ((OFF_IC_TABLE / 8) << 10) | (XCPU << 5) | XA;
    let want_ldr_ic_t4 = 0xF940_0000 | ((OFF_IC_TABLE / 8) << 10) | (XCPU << 5) | T4;
    let want_blr_x2 = 0xD63F_0000 | (T2 << 5);
    let want_br_x2 = 0xD61F_0000 | (T2 << 5);

    for (ic_on, ras_on) in [(true, true), (true, false), (false, true), (false, false)] {
        test_set_knobs(TestKnobs { ic: Some(ic_on), ras: Some(ras_on), ..TestKnobs::NONE });
        let stubs = if ras_on { &stubs_ras } else { &stubs_noras };
        let mut scratch = TrScratch::new();
        let mut dec = Decoder::with_ip(64, &bytes, 0x1_0000_0000, DecoderOptions::NONE);
        let mut idx = 0usize;
        while dec.can_decode() {
            let ins = dec.decode();
            let op = lower(&ins);
            let term = match idx {
                0 => Term::CallR,
                1 => Term::CallM,
                2 => Term::JmpR,
                3 => Term::JmpM,
                _ => Term::Ret,
            };
            let mut prep = Prep {
                ops: [FastOp::default(); BLK_MAX_INSNS],
                k: 1,
                host: false,
                wr: [0; BLK_MAX_INSNS],
                term,
                target_host: false,
                lazy: false,
                edge: Edges { live: [0; 2], kill: [0; 2] },
                live_out: 0,
            };
            prep.ops[0] = op;
            let mut buf = vec![0u32; 512];
            let asm = Asm::new(buf.as_mut_ptr(), 0x2000, buf.len());
            let mut tr = new_tr(asm, &prep.ops[..1], &prep, stubs, true, &mut scratch);
            let main = tr.translate(term, 0);
            assert!(!tr.a.overflow, "overflow translating `{ins}` (ic={ic_on}, ras={ras_on})");
            assert!(
                tr.relocs.is_empty(),
                "`{ins}` emitted runtime relocs ({:?}), breaking shifted-base AOT / static Mach-O",
                tr.relocs
            );
            let hot = &buf[..main];
            let cold = &buf[main..tr.cold_code_end];
            if ic_on {
                match term {
                    Term::CallR | Term::CallM => {
                        assert!(hot.contains(&want_ldr_ic_xa), "{ins}: hot path must load OFF_IC_TABLE");
                        let want_br = if ras_on { want_blr_x2 } else { want_br_x2 };
                        assert!(hot.contains(&want_br), "{ins}: hot path must emit per-site blr/br x2");
                        assert!(!cold.is_empty(), "{ins}: IC way-1/miss tail must live in cold section");
                    }
                    Term::JmpR | Term::JmpM => {
                        assert!(hot.contains(&want_ldr_ic_xa), "{ins}: hot path must load OFF_IC_TABLE");
                        assert!(hot.contains(&want_br_x2), "{ins}: hot path must emit per-site br x2");
                        assert!(!cold.is_empty(), "{ins}: IC way-1/miss tail must live in cold section");
                    }
                    Term::Ret if ras_on => {
                        assert!(!hot.contains(&want_ldr_ic_xa), "RAS ret hot path must stay IC-load-free");
                        assert!(cold.contains(&want_ldr_ic_t4), "RAS ret cold miss must load OFF_IC_TABLE into x4");
                    }
                    Term::Ret => {
                        assert!(hot.contains(&want_ldr_ic_xa), "non-RAS ret hot path must load OFF_IC_TABLE");
                        assert!(hot.contains(&want_br_x2), "non-RAS ret hot path must emit per-site br x2");
                    }
                    _ => unreachable!(),
                }
            } else {
                assert!(!hot.contains(&want_ldr_ic_xa) && !cold.contains(&want_ldr_ic_t4),
                    "PORT_JIT_IC=0 must not emit IC table loads for `{ins}`");
            }
            tr.return_scratch(&mut scratch);
            idx += 1;
        }
        assert_eq!(idx, 5);
    }
    test_set_knobs(TestKnobs::NONE);
}

#[test]
fn rotated_smc_entry_revalidates_with_authoritative_meta() {
    use crate::emu::jit::cache::JMeta;
    use crate::emu::jit_tests::arena_alloc;
    use crate::emu::mem;

    let mut jc = JitCache::new();
    let mut bc = Box::new(BlockCache::new());
    let ip_ptr = arena_alloc(64, 16);
    let ip_smc = ip_ptr as u64;
    // `mov eax, 0x11223344; ret`
    let body_v1: [u8; 6] = [0xb8, 0x44, 0x33, 0x22, 0x11, 0xc3];
    let body_v2: [u8; 6] = [0xb8, 0x88, 0x77, 0x66, 0x55, 0xc3];
    unsafe {
        core::ptr::copy_nonoverlapping(body_v1.as_ptr(), ip_ptr, body_v1.len());
    }
    let lo = ip_ptr as usize;
    let hi = lo + 64;
    mem::note_code_region(lo, hi);

    unsafe {
        // Trigger translation into way 0 with `gen != 0`.
        let mut code1 = None;
        for _ in 0..8 {
            if let Some(c) = jc.lookup(ip_smc, &mut *bc) {
                code1 = Some(c);
                break;
            }
        }
        let code1 = code1.expect("SMC block must compile");
        let b = jset_base(ip_smc);
        assert_eq!(jc.index[b].ip, ip_smc);
        assert_ne!(jc.index[b].gen, 0, "dynamic region block must carry non-zero gen");

        // Poison way 1's `meta` with an unrelated oversized length/hash (simulating another block
        // previously occupying way 1) and rotate `ip_smc` from way 0 -> way 1 via `cache_plain_host`
        // (same meta-untouched rotation performed by `aot_probe` and `cache_qpc_host`).
        jc.meta[b | 1] = JMeta { hash: 0xDEAD_BEEF_CAFE_BABE, len: 0x7FFF_FFFF };
        let mut peer = ip_smc + 0x1000;
        while jset_base(peer) != b {
            peer += 16;
        }
        let tbl = jc.test_table_ptr();
        JitCache::cache_plain_host(tbl, peer, 0x1000_0000, false);
        assert_eq!(jc.index[b].ip, peer);
        assert_eq!(jc.index[b | 1].ip, ip_smc);

        // Bump CODE_GEN without growing JIT_REGIONS_GEN and without modifying `ip_smc`'s bytes:
        // `revalidate()` must recover the authoritative `JMeta` from `compiled`, keep `code1`
        // (no spurious retranslation), and avoid reading the poisoned 2 GiB length.
        mem::note_code_region(lo, hi);
        let code2 = jc.lookup(ip_smc, &mut *bc).expect("unchanged rotated SMC block must revalidate");
        assert_eq!(code2, code1, "unchanged rotated SMC block must not spuriously retranslate");
        assert_eq!(jc.index[b | 1].gen, mem::code_gen());
        assert_eq!(jc.meta[b | 1].len, body_v1.len() as u32);

        // Now actually overwrite `ip_smc`'s bytes and bump CODE_GEN: `revalidate()` must detect
        // the real code change and produce a fresh translation.
        core::ptr::copy_nonoverlapping(body_v2.as_ptr(), ip_ptr, body_v2.len());
        mem::note_code_region(lo, hi);
        let code3 = jc.lookup(ip_smc, &mut *bc).expect("modified rotated SMC block must retranslate");
        assert_ne!(code3, code1, "modified SMC block must retranslate to new code");
    }
}
