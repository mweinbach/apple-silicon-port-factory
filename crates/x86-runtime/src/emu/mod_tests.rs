//! Unit tests for `emu` core mechanics (GPR table, caller extra-args windows, `interp_burst`
//! block chaining & invalidation, `DF_LEAN` direct calls, wide stack-arg helpers, and system
//! descriptor stores) plus `test_translate` shared by `jit_tests` and `jit_bench`.

use super::*;

#[test]
fn gpr_table_matches_register_widths() {
    assert_eq!(Cpu::reg_idx(Register::RAX), Some((0, 64, false)));
    assert_eq!(Cpu::reg_idx(Register::ECX), Some((1, 32, false)));
    assert_eq!(Cpu::reg_idx(Register::DX), Some((2, 16, false)));
    assert_eq!(Cpu::reg_idx(Register::BH), Some((3, 8, true)));
    assert_eq!(Cpu::reg_idx(Register::SPL), Some((4, 8, false)));
    assert_eq!(Cpu::reg_idx(Register::R15D), Some((15, 32, false)));
    assert_eq!(Cpu::reg_idx(Register::R8L), Some((8, 8, false)));
    assert_eq!(Cpu::reg_idx(Register::XMM0), None);
    assert_eq!(Cpu::reg_idx(Register::None), None);
    assert_eq!(Cpu::reg_idx(Register::RIP), None);
}

#[test]
fn caller_extra_args_zero_full_and_partial_windows() {
    unsafe {
        assert_eq!(read_caller_extra_args(0, 0), [0; 8]);
        assert_eq!(read_caller_extra_args(0, u64::MAX), [0; 8]);
        let buf: [u64; 24] = core::array::from_fn(|i| 100 + i as u64);
        let rsp = buf.as_ptr() as u64;
        // Full window: args 9..16 at rsp + 0x40..0x80.
        assert_eq!(read_caller_extra_args(rsp, rsp + 0xc0), core::array::from_fn(|i| 108 + i as u64));
        // Window cut at rsp + 0x58: the first three args, the rest 0.
        assert_eq!(read_caller_extra_args(rsp, rsp + 0x58), [108, 109, 110, 0, 0, 0, 0, 0]);
    }
}

fn alloc_guest_code(bytes: &[u8]) -> u64 {
    let mut buf = vec![0xCCu8; bytes.len().max(64) + 32].into_boxed_slice();
    buf[..bytes.len()].copy_from_slice(bytes);
    let leaked: &'static mut [u8] = Box::leak(buf);
    leaked.as_mut_ptr() as u64
}

#[test]
fn interp_burst_self_loop_and_inter_block_chaining() {
    // Multi-block guest program exercising:
    // - entry block calling a helper (`call rel32`)
    // - helper with a tight self-loop (`add rax, rdx; dec rcx; jne self; ret`)
    // - conditional branch + unconditional jmp + fallthrough + ret
    //
    // Layout:
    //   0x00: mov eax, 0             ; B8 00 00 00 00 (5)
    //   0x05: call +0x10 (0x1A)      ; E8 10 00 00 00 (5)
    //   0x0A: add rax, 7             ; 48 83 C0 07    (4)
    //   0x0E: jmp +0x04 (0x14)       ; EB 04          (2)
    //   0x10: nop; nop; nop; nop     ; 90 90 90 90    (4)
    //   0x14: add rax, 1             ; 48 83 C0 01    (4)
    //   0x18: ret                    ; C3             (1)
    //   0x19: nop                    ; 90             (1)
    //   0x1A: add rax, rdx           ; 48 01 D0       (3)  <- loop_head
    //   0x1D: dec rcx                ; 48 FF C9       (3)
    //   0x20: jne -8 (0x1A)          ; 75 F8          (2)
    //   0x22: ret                    ; C3             (1)
    #[rustfmt::skip]
    let code: [u8; 35] = [
        0xB8, 0x00, 0x00, 0x00, 0x00,
        0xE8, 0x10, 0x00, 0x00, 0x00,
        0x48, 0x83, 0xC0, 0x07,
        0xEB, 0x04,
        0x90, 0x90, 0x90, 0x90,
        0x48, 0x83, 0xC0, 0x01,
        0xC3,
        0x90,
        0x48, 0x01, 0xD0,
        0x48, 0xFF, 0xC9,
        0x75, 0xF8,
        0xC3,
    ];
    let ip = alloc_guest_code(&code);
    jit::force(2); // interpreter mode
    TEST_INTERP_BURSTS.with(|c| c.set(0));
    TEST_INTERP_BURST_CHAINS.with(|c| c.set(0));

    // First run warms and stamps all blocks.
    let (r1, _) = unsafe { emu_run_full(ip, [100, 3, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]) };
    assert_eq!(r1, 100 * 3 + 7 + 1);
    let steps1 = test_last_steps();
    // Exact instruction count: 2 (mov+call) + 100*3 (loop) + 1 (ret) + 2 (add+jmp) + 2 (add+ret) = 307.
    assert_eq!(steps1, 307);

    // Second run: every block is stamped -> entire call+loop+ret+jmp+ret runs in ONE burst!
    TEST_INTERP_BURSTS.with(|c| c.set(0));
    TEST_INTERP_BURST_CHAINS.with(|c| c.set(0));
    let (r2, _) = unsafe { emu_run_full(ip, [500, 5, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]) };
    jit::force(0);
    assert_eq!(r2, 500 * 5 + 7 + 1);
    assert_eq!(test_last_steps(), 2 + 500 * 3 + 1 + 2 + 2);
    assert_eq!(
        TEST_INTERP_BURSTS.with(|c| c.get()),
        1,
        "warm multi-block call+self-loop+ret+jmp must finish in a single interp_burst"
    );
    // Chains: call->loop (1) + 499 self-loops + fallthrough->ret (1) + ret->caller (1) + jmp->tail (1) = 503.
    assert_eq!(TEST_INTERP_BURST_CHAINS.with(|c| c.get()), 503);
}

#[test]
fn interp_burst_generic_fallback_and_resume() {
    // Fast op -> generic op (`sidt [rcx]`, K_GENERIC) -> fast op -> ret in one block.
    #[rustfmt::skip]
    let full: [u8; 15] = [
        0x48, 0xC7, 0xC0, 0x2A, 0x00, 0x00, 0x00, // mov rax, 42
        0x0F, 0x01, 0x09,                         // sidt [rcx] (K_GENERIC)
        0x48, 0x83, 0xC0, 0x08,                   // add rax, 8
        0xC3,                                     // ret
    ];
    let ip = alloc_guest_code(&full);
    let mut out = [0u8; 16];
    jit::force(2);
    let (r, _) = unsafe {
        emu_run_full(ip, [out.as_mut_ptr() as u64, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8])
    };
    jit::force(0);
    assert_eq!(r, 50);
    assert_eq!(u16::from_le_bytes([out[0], out[1]]), 0x0fff);
    assert_eq!(test_last_steps(), 4);
}

#[test]
fn interp_burst_va_gen_invalidation_dispatches_new_host_hook() {
    unsafe extern "win64" fn hook_returns_222() -> u64 {
        222
    }
    // Callee at +0x10: mov eax, 111; ret
    // Caller at +0x00: call +0x0B (+0x10); ret
    #[rustfmt::skip]
    let code: [u8; 22] = [
        0xE8, 0x0B, 0x00, 0x00, 0x00,             // call +0x10
        0xC3,                                     // ret
        0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
        0xB8, 0x6F, 0x00, 0x00, 0x00,             // mov eax, 111
        0xC3,                                     // ret
    ];
    let caller_ip = alloc_guest_code(&code);
    let callee_ip = caller_ip + 0x10;

    jit::force(2);
    // Warm & chain caller -> guest callee in interp_burst.
    assert_eq!(unsafe { emu_run_full(caller_ip, [0; 8], [0; 4], [0; 8]).0 }, 111);
    TEST_INTERP_BURSTS.with(|c| c.set(0));
    TEST_INTERP_BURST_CHAINS.with(|c| c.set(0));
    assert_eq!(unsafe { emu_run_full(caller_ip, [0; 8], [0; 4], [0; 8]).0 }, 111);
    assert_eq!(TEST_INTERP_BURSTS.with(|c| c.get()), 1);
    assert_eq!(TEST_INTERP_BURST_CHAINS.with(|c| c.get()), 2); // call->callee + ret->caller

    // Register a host hook at `callee_ip` (bumps `exe::va_gen()`).
    crate::exe::register_native_va(callee_ip, hook_returns_222 as *const () as usize);
    let hooked = unsafe { emu_run_full(caller_ip, [0; 8], [0; 4], [0; 8]).0 };
    crate::exe::unregister_native_va(callee_ip);
    jit::force(0);
    assert_eq!(hooked, 222, "va_gen bump must invalidate stamped fast_gen and dispatch host hook");
}

#[test]
fn interp_burst_code_gen_invalidation() {
    // 1. Verify `note_code_region` bumps `BlockCache::fast_gen` so stamped entries become stale.
    let code: [u8; 6] = [0xB8, 0x0A, 0x00, 0x00, 0x00, 0xC3]; // mov eax, 10; ret
    let ip = alloc_guest_code(&code);
    jit::force(2);
    assert_eq!(unsafe { emu_run_full(ip, [0; 8], [0; 4], [0; 8]).0 }, 10);
    let (old_fgen, stamped_fgen) = TLS_EMU.with(|cell| unsafe {
        let st = (*cell.get()).as_ref().unwrap();
        let e = st.blocks.index[BlockCache::slot(ip)];
        (st.blocks.fast_gen, e.fast_gen)
    });
    assert_ne!(old_fgen, 0);
    assert_eq!(stamped_fgen, old_fgen);

    // 2. Allocate a dynamic-code buffer, register it with `note_code_region` (bumps code_gen),
    //    execute it (must NOT be stamped since `e.gen != 0`), rewrite it, bump code_gen again,
    //    and verify the rewritten code runs.
    let dyn_buf: &'static mut [u8] = Box::leak(vec![0xCCu8; 64].into_boxed_slice());
    dyn_buf[..6].copy_from_slice(&[0xB8, 0x15, 0x00, 0x00, 0x00, 0xC3]); // mov eax, 21; ret
    let dyn_lo = dyn_buf.as_ptr() as usize;
    mem::note_code_region(dyn_lo, dyn_lo + 64);

    let r1 = unsafe { emu_run_full(dyn_lo as u64, [0; 8], [0; 4], [0; 8]).0 };
    assert_eq!(r1, 21);
    let (new_fgen, old_entry_fgen, dyn_entry) = TLS_EMU.with(|cell| unsafe {
        let st = (*cell.get()).as_ref().unwrap();
        let old_e = st.blocks.index[BlockCache::slot(ip)];
        let dyn_e = st.blocks.index[BlockCache::slot(dyn_lo as u64)];
        (st.blocks.fast_gen, old_e.fast_gen, dyn_e)
    });
    assert_ne!(new_fgen, old_fgen, "code_gen bump must advance BlockCache::fast_gen");
    assert_ne!(old_entry_fgen, new_fgen, "previously stamped entry must be invalidated");
    assert_ne!(dyn_entry.gen, 0);
    assert_eq!(dyn_entry.fast_gen, 0, "dynamic JIT blocks (e.gen != 0) must never be fast_gen stamped");

    // Rewrite dynamic code in place and bump `code_gen` again.
    dyn_buf[..6].copy_from_slice(&[0xB8, 0x2A, 0x00, 0x00, 0x00, 0xC3]); // mov eax, 42; ret
    mem::note_code_region(dyn_lo, dyn_lo + 64);
    let r2 = unsafe { emu_run_full(dyn_lo as u64, [0; 8], [0; 4], [0; 8]).0 };
    jit::force(0);
    assert_eq!(r2, 42);
}

#[test]
fn interp_burst_plain_leaf_host_call_and_tail_call() {
    // Register a known DF_PLAIN builtin (`QueryPerformanceCounter`) at a synthetic VA and
    // call it via both guest `call rax` and guest tail `jmp rax` in interpreter mode.
    let qpc_va = 0x7_7700_0100u64;
    let qpc_fn = win32_rt::kernel32::time::QueryPerformanceCounter as *const () as usize;
    crate::exe::register_native_va(qpc_va, qpc_fn);

    // `call rax; ret` (rcx = &counter, rax = qpc_va)
    let call_code = alloc_guest_code(&[0xFF, 0xD0, 0xC3]);
    // `jmp rax` (tail call: rcx = &counter, rax = qpc_va)
    let jmp_code = alloc_guest_code(&[0xFF, 0xE0]);

    jit::force(2);
    TEST_PLAIN_HOST_CALLS.with(|c| c.set(0));
    let mut q1 = 0i64;
    let (r1, _) = unsafe {
        emu_run_full_rax(call_code, [&mut q1 as *mut i64 as u64, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8], qpc_va)
    };
    assert_eq!(r1 as u32, 1);
    assert_ne!(q1, 0);
    assert_eq!(TEST_PLAIN_HOST_CALLS.with(|c| c.get()), 1);

    let mut q2 = 0i64;
    let (r2, _) = unsafe {
        emu_run_full_rax(jmp_code, [&mut q2 as *mut i64 as u64, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8], qpc_va)
    };
    jit::force(0);
    crate::exe::unregister_native_va(qpc_va);
    assert_eq!(r2 as u32, 1);
    assert_ne!(q2, 0);
    assert_eq!(TEST_PLAIN_HOST_CALLS.with(|c| c.get()), 2);
}

/// Test-only: translate the guest block at `ip` with this thread's translator (forced on)
/// without running it; returns its host code address. The thread's emulator state must
/// exist (any earlier `emu_run_full` on this thread).
#[cfg(target_arch = "aarch64")]
pub(super) unsafe fn test_translate(ip: u64) -> Option<usize> {
    let (blocks, jit) = TLS_EMU.with(|cell| {
        let st = (*cell.get()).as_mut().expect("emulator state (run emu_run_full first)");
        (&mut st.blocks as *mut BlockCache, &mut st.jit as *mut jit::JitCache)
    });
    jit::force(1);
    let r = (*jit).lookup(ip, blocks);
    jit::force(0);
    r
}

/// `lean_call` / `call_lean_in` (the `DF_LEAN` direct-call path) against the generic
/// bridge (`host_call_asm` / `call_host_dispatched_in`): same argument image at the callee
/// (x0..x7, q0..q3, stack args 9..16) and same `CallCtx` effects and results.
#[cfg(target_arch = "aarch64")]
mod lean_tests {
    use super::super::*;
    use core::arch::aarch64::uint64x2_t;
    use std::cell::Cell;

    std::thread_local! {
        static SEEN: Cell<[u64; 24]> = const { Cell::new([0; 24]) };
    }

    fn q2(v: uint64x2_t) -> [u64; 2] {
        unsafe { core::mem::transmute(v) }
    }

    #[allow(improper_ctypes_definitions, clippy::too_many_arguments)]
    unsafe extern "C" fn echo(
        a0: u64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64, a7: u64,
        v0: uint64x2_t, v1: uint64x2_t, v2: uint64x2_t, v3: uint64x2_t,
        s0: u64, s1: u64, s2: u64, s3: u64, s4: u64, s5: u64, s6: u64, s7: u64,
    ) -> u64 {
        let (v0, v1, v2, v3) = (q2(v0), q2(v1), q2(v2), q2(v3));
        let all = [
            a0, a1, a2, a3, a4, a5, a6, a7, v0[0], v0[1], v1[0], v1[1], v2[0], v2[1], v3[0], v3[1],
            s0, s1, s2, s3, s4, s5, s6, s7,
        ];
        SEEN.with(|s| s.set(all));
        all.iter().fold(0u64, |h, &x| h.rotate_left(7) ^ x)
    }

    /// A Win64-declared replacement with 10 integer args (9 and 10 come from the stack).
    #[allow(clippy::too_many_arguments)]
    unsafe extern "win64" fn ten(a: u64, b: u64, c: u64, d: u64, e: u64, f: u64, g: u64, h: u64, i: u64, j: u64) -> u64 {
        a ^ b.rotate_left(1) ^ c.rotate_left(2) ^ d.rotate_left(3) ^ e.rotate_left(4)
            ^ f.rotate_left(5) ^ g.rotate_left(6) ^ h.rotate_left(7) ^ i.rotate_left(8) ^ j.rotate_left(9)
    }

    /// A Win64-declared replacement with a float parameter (arrives in d0 = low xmm0).
    unsafe extern "win64" fn with_f64(x: f64, p: u64) -> u64 {
        x.to_bits() ^ p.rotate_left(13)
    }

    fn inputs() -> ([u64; 8], [u128; 4], [u64; 8]) {
        let gpr = core::array::from_fn(|i| 0x1111_0000_0000_0001u64.wrapping_mul(i as u64 + 3));
        let xmm = core::array::from_fn(|i| {
            ((0xA5A5_0000_0000_0000u128 | i as u128) << 64) | (0x5A5A_0000_0000_0100u128 + i as u128)
        });
        let st = core::array::from_fn(|i| 0xC0DE_0000_0000_0000u64 | (i as u64 + 9));
        (gpr, xmm, st)
    }

    #[test]
    fn lean_call_matches_host_call_asm() {
        let (gpr, xmm, st) = inputs();
        unsafe {
            let r_lean = lean_call(echo as *const () as usize, &gpr, &xmm, &st);
            let seen_lean = SEEN.with(|s| s.get());
            SEEN.with(|s| s.set([0; 24]));
            let (r_asm, _) = host_call_asm(echo as *const () as usize, &gpr, &xmm, &st);
            let seen_asm = SEEN.with(|s| s.get());
            assert_eq!(seen_lean, seen_asm);
            assert_eq!(r_lean, r_asm);
            let mut want = [0u64; 24];
            want[..8].copy_from_slice(&gpr);
            for (i, x) in xmm.iter().enumerate() {
                want[8 + 2 * i] = *x as u64;
                want[9 + 2 * i] = (*x >> 64) as u64;
            }
            want[16..].copy_from_slice(&st);
            assert_eq!(seen_lean, want);

            for f in [ten as *const () as usize, with_f64 as *const () as usize] {
                assert_eq!(lean_call(f, &gpr, &xmm, &st), host_call_asm(f, &gpr, &xmm, &st).0);
            }
            assert_eq!(lean_call(ten as *const () as usize, &gpr, &xmm, &st), ten(gpr[0], gpr[1], gpr[2], gpr[3], gpr[4], gpr[5], gpr[6], gpr[7], st[0], st[1]));
            assert_eq!(lean_call(with_f64 as *const () as usize, &gpr, &xmm, &st), with_f64(f64::from_bits(xmm[0] as u64), gpr[0]));
        }
    }

    /// Lifted-style callee: reads its XMM args and `rax` from `CallCtx`, optionally sets an
    /// XMM result.
    #[allow(clippy::too_many_arguments)]
    unsafe extern "win64" fn tls_callee(a: u64, b: u64, _c: u64, _d: u64, _e: u64, _f: u64, _g: u64, h: u64) -> u64 {
        let x = crate::simd_ops::take_xmm_args();
        let rax = crate::simd_ops::get_rax_in();
        if a & 1 == 1 {
            crate::simd_ops::set_xmm_ret(x[1] ^ x[2]);
        }
        (x[0] as u64) ^ ((x[3] >> 64) as u64) ^ rax.rotate_left(3) ^ b ^ h
    }

    #[test]
    fn call_lean_in_matches_bridge_protocol() {
        let (mut gpr, xmm, st) = inputs();
        let c = crate::callctx::ctx();
        for a in [1u64, 2] {
            gpr[0] = a;
            unsafe {
                let prev = crate::simd_ops::replace_rax_in(0x7777_0000_0000_0042);
                let lean = call_lean_in(c, tls_callee as *const () as usize, &gpr, &xmm, &st);
                let lean_state = (c.xmm_in_set.get(), c.xmm_out.get(), c.xmm_in.get());
                let full = call_host_dispatched_in(
                    c,
                    0x1_4000_1000,
                    tls_callee as *const () as usize,
                    DF_VALID | DF_HOST | DF_GAME_REPL,
                    &gpr,
                    &xmm,
                    &st,
                );
                let full_state = (c.xmm_in_set.get(), c.xmm_out.get(), c.xmm_in.get());
                crate::simd_ops::replace_rax_in(prev);
                assert_eq!(lean, full, "a={a}");
                assert_eq!(lean_state, full_state, "a={a}");
                assert_eq!(lean.1, if a == 1 { xmm[1] ^ xmm[2] } else { xmm[0] });
                assert!(!lean_state.0);
            }
        }
    }

    // --- DF_PLAIN builtins called from inside a lean-called function -------------------

    const VA_OUTER: u64 = 0x1_4000_2000;
    const VA_TRY_ACQ: u64 = 0x1_4000_3000;
    const VA_QPC: u64 = 0x1_4000_3100;
    const VA_REL: u64 = 0x1_4000_3200;

    /// (outgoing, incoming, xmm_in_set, rax_in) after a nested call. (`xmm_in` is only
    /// meaningful while `xmm_in_set`; the bridge rewrites it, the plain path doesn't.)
    type Post = ((u64, u64), (u64, u64), bool, u64);

    std::thread_local! {
        static OBS: Cell<[Option<Post>; 3]> = const { Cell::new([None; 3]) };
        static FWD_AT_ENTRY: Cell<bool> = const { Cell::new(false) };
        static XOUT: Cell<[Option<u128>; 3]> = const { Cell::new([None; 3]) };
    }

    fn post(c: &crate::callctx::CallCtx) -> Post {
        (c.outgoing_rsp.get(), c.incoming_rsp.get(), c.xmm_in_set.get(), c.rax_in.get())
    }

    /// Install `va -> (host_fn, flags)` in this thread's dispatch cache, as
    /// `resolve_dispatch_in` would after a miss.
    fn seed(va: u64, host_fn: usize, flags: u32) {
        use crate::callctx::{DispatchSlot, DCACHE_BITS, DCACHE_LEN};
        let c = crate::callctx::ctx();
        let idx = (((va >> 4) ^ (va >> (4 + DCACHE_BITS))) as usize) & (DCACHE_LEN - 1);
        unsafe {
            (*c.dcache.get())[idx] = DispatchSlot { va, host_fn, flags, gen: crate::exe::va_gen() };
        }
    }

    /// Lifted-style function reached through the lean path. Its first nested call is a
    /// plain builtin made while `xmm_in_set` is still up; the second one has stack args
    /// pending (`call_stk`).
    #[allow(clippy::too_many_arguments)]
    unsafe extern "win64" fn outer(lock: u64, qpc_out: u64, _c: u64, _d: u64, _e: u64, _f: u64, _g: u64, _h: u64) -> u64 {
        let c = crate::callctx::ctx();
        FWD_AT_ENTRY.with(|f| f.set(c.xmm_in_set.get()));
        let mut obs = [None; 3];
        let mut xout = [None; 3];
        let r1 = call_va8(VA_TRY_ACQ, lock, 0, 0, 0, 0, 0, 0, 0) as u8 as u64;
        obs[0] = Some(post(c));
        xout[0] = c.xmm_out.get();
        let stk = [0x5151u64; 24];
        let r2 = crate::simd_ops::call_stk(VA_TRY_ACQ, 0x77, lock, 0, 0, 0, stk.as_ptr() as u64) as u8 as u64;
        obs[1] = Some(post(c));
        xout[1] = c.xmm_out.get();
        let r3 = call_va8(VA_QPC, qpc_out, 0, 0, 0, 0, 0, 0, 0) as u32 as u64;
        obs[2] = Some(post(c));
        xout[2] = c.xmm_out.get();
        call_va8(VA_REL, lock, 0, 0, 0, 0, 0, 0, 0);
        OBS.with(|o| o.set(obs));
        XOUT.with(|o| o.set(xout));
        0xABC0_0000 | r1 | r2 << 4 | r3 << 8
    }

    /// Run `outer` through `call_va8` (lean path) with the builtins dispatched as `flags`;
    /// returns (rax, nested-call post-states, xmm_out after each nested call, outer post-state).
    fn run(builtin_flags: u32) -> (u64, [Option<Post>; 3], [Option<u128>; 3], Post) {
        use win32_rt::{kernel32, sync};
        seed(VA_OUTER, outer as *const () as usize, LEAN_FLAGS);
        seed(VA_TRY_ACQ, sync::TryAcquireSRWLockExclusive as *const () as usize, builtin_flags);
        seed(VA_QPC, kernel32::time::QueryPerformanceCounter as *const () as usize, builtin_flags);
        seed(VA_REL, sync::ReleaseSRWLockExclusive as *const () as usize, builtin_flags);
        let c = crate::callctx::ctx();
        let lock = Box::new(0u64);
        let mut qpc = 0i64;
        unsafe {
            let prev = crate::simd_ops::replace_rax_in(0x7777_0000_0000_0042);
            c.outgoing_rsp.set((0, 0));
            c.incoming_rsp.set((0, 0));
            let rax = call_va8(VA_OUTER, &*lock as *const u64 as u64, &mut qpc as *mut i64 as u64, 3, 4, 5, 6, 7, 8);
            let after = post(c);
            crate::simd_ops::replace_rax_in(prev);
            assert_eq!(*lock, 0, "lock released");
            assert_ne!(qpc, 0, "QPC wrote the counter");
            (rax, OBS.with(|o| o.get()), XOUT.with(|o| o.get()), after)
        }
    }

    #[test]
    fn plain_builtin_first_in_lean_callee_matches_bridge() {
        // Seeded dispatch slots: no generation bump from other tests until the end.
        let _va = crate::exe::hold_va_gen();
        let plain = run(DF_VALID | DF_HOST | DF_PLAIN);
        assert!(FWD_AT_ENTRY.with(|f| f.get()), "first nested call happens with xmm_in_set up");
        let bridge = run(DF_VALID | DF_HOST);
        // TryAcquire: 1 (fresh lock), then 0 (held); QPC: TRUE.
        assert_eq!(plain.0, 0xABC0_0000 | 1 | 0 << 4 | 1 << 8);
        assert_eq!(plain.0, bridge.0, "rax");
        assert_eq!(plain.1, bridge.1, "post-state after each nested builtin call");
        assert_eq!(plain.3, bridge.3, "post-state after the lean call");
        for (i, p) in plain.1.iter().enumerate() {
            let (out, inc, fwd, _) = p.expect("observed");
            assert_eq!((out, inc, fwd), ((0, 0), (0, 0), false), "call {i}: windows and forwarded XMM consumed");
        }
        // rax_in: the lean entry's value, then `call_stk`'s rax (kept by the builtin call).
        assert_eq!(plain.1.map(|p| p.unwrap().3), [0x7777_0000_0000_0042, 0x77, 0x77]);
        // The plain path leaves no XMM result. (The bridge may publish the builtin's leftover
        // q0 when XMM was forwarded: undefined after an integer function.)
        assert_eq!(plain.2, [None; 3]);
        assert_eq!(bridge.2[1..], [None; 2]);
        let (out, inc, fwd, rax_in) = plain.3;
        assert_eq!((out, inc, fwd, rax_in), ((0, 0), (0, 0), false, 0x7777_0000_0000_0042));
    }

    // --- stack args 17..24+ across the call helpers (guest and lifted callees) ---------

    /// Guest x86 code: for k = 5..=`kmax`, `[rcx + 8*(k-5)] = [rsp + 8*k]` (incoming arg k),
    /// then `ret`. Placed outside the exe VA window.
    fn guest_store_args_code(kmax: u32) -> u64 {
        let mut code = Vec::new();
        for k in 5u32..=kmax {
            code.extend_from_slice(&[0x48, 0x8B, 0x84, 0x24]); // mov rax, [rsp+disp32]
            code.extend_from_slice(&(8 * k).to_le_bytes());
            code.extend_from_slice(&[0x48, 0x89, 0x81]); // mov [rcx+disp32], rax
            code.extend_from_slice(&(8 * (k - 5)).to_le_bytes());
        }
        code.push(0xC3); // ret
        loop {
            let m: &'static mut [u8] = Box::leak(vec![0xCCu8; 4096].into_boxed_slice());
            let p = m.as_mut_ptr() as u64;
            if p + 4096 > 0x1_4000_0000 && p < 0x1_4500_0000 {
                continue;
            }
            m[..code.len()].copy_from_slice(&code);
            return p;
        }
    }

    /// Lifted-style callee: copies its frame slots 5..=23 (incoming args 5..23) to `out`.
    /// Slot 24 (arg 24) is above the 24-qword argument area of today's frame.
    #[allow(clippy::too_many_arguments)]
    unsafe extern "win64" fn lifted_store_args(out: u64, _d: u64, _r8: u64, _r9: u64, a5: u64, a6: u64, a7: u64, a8: u64) -> u64 {
        let mut f = crate::simd_ops::LiftedFrame::<256, 8>::new();
        let rsp = crate::simd_ops::init_lifted_stack(&mut f, a5, a6, a7, a8);
        for k in 5..24u64 {
            ((out + 8 * (k - 5)) as *mut u64).write(((rsp + 8 * k) as *const u64).read());
        }
        0x5157
    }

    const VA_LIFT_LEAN: u64 = 0x1_4000_4000;
    const VA_LIFT_HOST: u64 = 0x1_4000_4100;

    /// Caller-side outgoing area: arg k at `[rsp + 8*(k-1)]` holds `0xA000 + k`.
    fn outgoing_args() -> [u64; 32] {
        core::array::from_fn(|i| 0xA000 + i as u64 + 1)
    }

    /// `call_stk` (window `rsp..rsp+0xc0`) into an emulated callee: args 5..24 arrive
    /// (arg5..8 from the call, 9..16 `s_extra`, 17..24 `hi_args` cut at the window end).
    #[test]
    fn call_stk_passes_args_up_to_24_to_a_guest_callee() {
        let code = guest_store_args_code(24);
        let stk = outgoing_args();
        let mut out = [0u64; 20];
        let c = crate::callctx::ctx();
        unsafe {
            crate::simd_ops::call_stk(code, 0, out.as_mut_ptr() as u64, 0, 0, 0, stk.as_ptr() as u64);
        }
        let want: [u64; 20] = core::array::from_fn(|i| 0xA000 + 5 + i as u64);
        assert_eq!(out, want);
        assert_eq!((c.outgoing_rsp.get(), c.incoming_rsp.get()), ((0, 0), (0, 0)));
    }

    /// `call_stk` into a lifted callee, lean (`call_lean_in`) and generic (`DF_HOST`) dispatch:
    /// the callee frame gets args 5..23 (`init_lifted_stack` + `copy_incoming_stack_args`).
    #[test]
    fn call_stk_passes_args_up_to_23_to_a_lifted_callee() {
        // Seeded dispatch slots: no generation bump from other tests until the end.
        let _va = crate::exe::hold_va_gen();
        seed(VA_LIFT_LEAN, lifted_store_args as *const () as usize, LEAN_FLAGS);
        seed(VA_LIFT_HOST, lifted_store_args as *const () as usize, DF_VALID | DF_HOST | DF_GAME_REPL);
        let stk = outgoing_args();
        let want: [u64; 19] = core::array::from_fn(|i| 0xA000 + 5 + i as u64);
        let c = crate::callctx::ctx();
        for va in [VA_LIFT_LEAN, VA_LIFT_HOST] {
            let mut out = [0u64; 19];
            let r = unsafe { crate::simd_ops::call_stk(va, 0, out.as_mut_ptr() as u64, 0, 0, 0, stk.as_ptr() as u64) };
            assert_eq!(r, 0x5157, "va {va:#x}");
            assert_eq!(out, want, "va {va:#x}");
            assert_eq!((c.outgoing_rsp.get(), c.incoming_rsp.get()), ((0, 0), (0, 0)), "va {va:#x}");
        }
    }

    /// `read_s5_call` from an arena frame publishes the frame top as the window end, so an
    /// emulated callee gets args beyond 24 (here 5..26, the cohtml +0x481dc0 -> +0x490110
    /// shape, a `call_va8_xmm` site) as long as they lie inside the caller's frame.
    #[test]
    fn read_s5_call_passes_args_beyond_24_to_a_guest_callee() {
        let code = guest_store_args_code(26);
        let c = crate::callctx::ctx();
        let want: [u64; 22] = core::array::from_fn(|i| 0xA000 + 5 + i as u64);
        for xmm_form in [false, true] {
            let mut out = [0u64; 22];
            unsafe {
                let mut f = crate::simd_ops::LiftedFrame::<256, 40>::new();
                let base = crate::simd_ops::init_lifted_stack(&mut f, 0, 0, 0, 0);
                let rsp = base - 0x100;
                for k in 5..=26u64 {
                    ((rsp + 8 * (k - 1)) as *mut u64).write(0xA000 + k);
                }
                let s5 = crate::simd_ops::read_s5_call(rsp, &f);
                let (s6, s7, s8) = (
                    ((rsp + 0x28) as *const u64).read(),
                    ((rsp + 0x30) as *const u64).read(),
                    ((rsp + 0x38) as *const u64).read(),
                );
                let o = out.as_mut_ptr() as u64;
                if xmm_form {
                    crate::simd_ops::call_va8_xmm(code, o, 0, 0, 0, s5, s6, s7, s8, [0; 4]);
                } else {
                    crate::exe::call_va8(code, o, 0, 0, 0, s5, s6, s7, s8);
                }
            }
            assert_eq!(out, want, "xmm_form {xmm_form}");
            assert_eq!((c.outgoing_rsp.get(), c.incoming_rsp.get()), ((0, 0), (0, 0)), "xmm_form {xmm_form}");
        }
    }

    /// The loader's aarch64 `native_try_thunk` (aw2-loader hooks.rs) minus the
    /// `ORIG_TRAMPOLINE_TAG` branch: host-callable targets through `call_host_for_va`, guest
    /// code through `emu_run_full`. Neither consumes the window `call_va8_caught` publishes.
    unsafe extern "win64" fn test_try_thunk(target: usize, args: *const u64, _n: usize, frame: *mut crate::exe::TryFrame) -> u64 {
        let a = &*(args as *const [u64; crate::exe::TRY_MAX_ARGS]);
        let va = target as u64;
        let xmm = [a[0] as u128, a[1] as u128, a[2] as u128, a[3] as u128];
        let gpr: [u64; 8] = core::array::from_fn(|i| a[i]);
        let st: [u64; 8] = core::array::from_fn(|i| a[8 + i]);
        let (r, x) = match resolve_host_callable(va) {
            Some(h) => call_host_for_va(va, h, gpr, xmm, st),
            None => emu_run_full(va, gpr, xmm, st),
        };
        (*frame).rax = r;
        (*frame).xmm0 = x as u64;
        0
    }

    /// `call_stk_caught` -> `call_va8_caught` -> `exe::try_call` passes only 16 args, but it
    /// publishes the caller's window as `incoming_rsp` first, and `emu_run_full` takes args
    /// 17..24 from there (`hi_args`): an emulated callee still gets args 5..24.
    #[test]
    fn call_stk_caught_passes_args_up_to_24_to_a_guest_callee() {
        crate::exe::set_try_thunk(test_try_thunk as *const () as usize);
        let code = guest_store_args_code(24);
        let stk = outgoing_args();
        let mut out = [0u64; 20];
        let c = crate::callctx::ctx();
        unsafe {
            crate::simd_ops::call_stk_caught(code, 0, out.as_mut_ptr() as u64, 0, 0, 0, stk.as_ptr() as u64);
        }
        let want: [u64; 20] = core::array::from_fn(|i| 0xA000 + 5 + i as u64);
        assert_eq!(out, want);
        assert_eq!((c.outgoing_rsp.get(), c.incoming_rsp.get()), ((0, 0), (0, 0)));
    }

    /// Wide lifted callee (`LiftedFrame<256, 8, 26>`: reads args up to 25): copies its frame
    /// slots 5..=25 to `out`.
    #[allow(clippy::too_many_arguments)]
    unsafe extern "win64" fn lifted_store_args_wide(out: u64, _d: u64, _r8: u64, _r9: u64, a5: u64, a6: u64, a7: u64, a8: u64) -> u64 {
        let mut f = crate::simd_ops::LiftedFrame::<256, 8, 26>::new();
        let rsp = crate::simd_ops::init_lifted_stack(&mut f, a5, a6, a7, a8);
        for k in 5..26u64 {
            ((out + 8 * (k - 5)) as *mut u64).write(((rsp + 8 * k) as *const u64).read());
        }
        0x5158
    }

    const VA_LIFT_LEAN_W: u64 = 0x1_4000_4200;
    const VA_LIFT_HOST_W: u64 = 0x1_4000_4300;

    /// `call_stk_w::<0xd0>` (a call site writing its outgoing area up to `[rsp+0xc8]`, the
    /// cohtml +0x481dc0 shape) passes args 5..26 to emulated callees through every form, and
    /// args 5..25 to a wide lifted callee (lean and `DF_HOST` dispatch). Today's `call_stk`
    /// cuts the same lifted callee at arg24.
    #[test]
    fn call_stk_w_passes_args_up_to_its_limit() {
        // Seeded dispatch slots: no generation bump from other tests until the end.
        let _va = crate::exe::hold_va_gen();
        let code = guest_store_args_code(26);
        let stk = outgoing_args();
        let s = stk.as_ptr() as u64;
        let c = crate::callctx::ctx();
        let want: [u64; 22] = core::array::from_fn(|i| 0xA000 + 5 + i as u64);
        crate::exe::set_try_thunk(test_try_thunk as *const () as usize);
        for form in 0..3 {
            let mut out = [0u64; 22];
            let o = out.as_mut_ptr() as u64;
            unsafe {
                match form {
                    0 => {
                        crate::simd_ops::call_stk_w::<0xd0>(code, 0, o, 0, 0, 0, s);
                    }
                    1 => {
                        crate::simd_ops::call_stk_caught_w::<0xd0>(code, 0, o, 0, 0, 0, s);
                    }
                    _ => {
                        crate::simd_ops::call_stk_xmm_w::<0xd0>(code, 0, o, 0, 0, 0, s, [0; 4]);
                    }
                }
            }
            assert_eq!(out, want, "guest form {form}");
            assert_eq!((c.outgoing_rsp.get(), c.incoming_rsp.get()), ((0, 0), (0, 0)), "form {form}");
        }
        seed(VA_LIFT_LEAN_W, lifted_store_args_wide as *const () as usize, LEAN_FLAGS);
        seed(VA_LIFT_HOST_W, lifted_store_args_wide as *const () as usize, DF_VALID | DF_HOST | DF_GAME_REPL);
        let want: [u64; 21] = core::array::from_fn(|i| 0xA000 + 5 + i as u64);
        for va in [VA_LIFT_LEAN_W, VA_LIFT_HOST_W] {
            let mut out = [0u64; 21];
            let r = unsafe { crate::simd_ops::call_stk_w::<0xd0>(va, 0, out.as_mut_ptr() as u64, 0, 0, 0, s) };
            assert_eq!(r, 0x5158, "va {va:#x}");
            assert_eq!(out, want, "va {va:#x}");
            assert_eq!((c.outgoing_rsp.get(), c.incoming_rsp.get()), ((0, 0), (0, 0)), "va {va:#x}");
            // Today's window: arg24 still arrives, arg25 reads the clean frame's 0.
            let mut out = [0u64; 21];
            unsafe { crate::simd_ops::call_stk(va, 0, out.as_mut_ptr() as u64, 0, 0, 0, s) };
            assert_eq!(out[..20], want[..20], "va {va:#x} call_stk");
            assert_eq!(out[20], 0, "va {va:#x} call_stk arg25");
        }
    }

    /// `tail_stk_w::<0xd8>` (window `[rsp+8, rsp+0xd8)`): the emulated tail callee gets args
    /// 5..26 of the tail site's own incoming area (arg k at `[rsp + 8*k]`), in every form.
    #[test]
    fn tail_stk_w_passes_args_up_to_its_limit() {
        let code = guest_store_args_code(26);
        let stk = outgoing_args();
        let s = stk.as_ptr() as u64;
        let c = crate::callctx::ctx();
        // `stk[k]` (at `s + 8k`) = 0xA000 + k + 1.
        let want: [u64; 22] = core::array::from_fn(|i| 0xA000 + 5 + i as u64 + 1);
        crate::exe::set_try_thunk(test_try_thunk as *const () as usize);
        for form in 0..3 {
            let mut out = [0u64; 22];
            let o = out.as_mut_ptr() as u64;
            unsafe {
                match form {
                    0 => {
                        crate::simd_ops::tail_stk_w::<0xd8>(code, 0, o, 0, 0, 0, s);
                    }
                    1 => {
                        crate::simd_ops::tail_stk_caught_w::<0xd8>(code, 0, o, 0, 0, 0, s);
                    }
                    _ => {
                        crate::simd_ops::tail_stk_xmm_w::<0xd8>(code, 0, o, 0, 0, 0, s, [0; 4]);
                    }
                }
            }
            assert_eq!(out, want, "tail form {form}");
            assert_eq!((c.outgoing_rsp.get(), c.incoming_rsp.get()), ((0, 0), (0, 0)), "form {form}");
        }
    }

    #[test]
    fn unprivileged_descriptor_table_and_msw_stores() {
        // sidt [rcx]; sgdt [rcx+10]; sldt word [rcx+20]; str word [rcx+22];
        // smsw word [rcx+24]; smsw eax; ret
        static CODE: [u8; 22] = [
            0x0F, 0x01, 0x09, // sidt [rcx]
            0x0F, 0x01, 0x41, 0x0A, // sgdt [rcx+10]
            0x0F, 0x00, 0x41, 0x14, // sldt word ptr [rcx+20]
            0x0F, 0x00, 0x49, 0x16, // str word ptr [rcx+22]
            0x0F, 0x01, 0x61, 0x18, // smsw word ptr [rcx+24]
            0x0F, 0x01, 0xE0, // smsw eax
        ];
        let mut full = CODE.to_vec();
        full.push(0xC3); // ret
        let mut out = [0xAAu8; 32];
        let (rax, _) = unsafe {
            emu_run_full(
                full.as_ptr() as u64,
                [out.as_mut_ptr() as u64, 0, 0, 0, 0, 0, 0, 0],
                [0; 4],
                [0; 8],
            )
        };
        assert_eq!(u16::from_le_bytes([out[0], out[1]]), 0x0fff);
        assert_eq!(u64::from_le_bytes(out[2..10].try_into().unwrap()), 0xffff_f800_0000_0000);
        assert_eq!(u16::from_le_bytes([out[10], out[11]]), 0x0057);
        assert_eq!(u64::from_le_bytes(out[12..20].try_into().unwrap()), 0xffff_f800_0000_1000);
        assert_eq!(u16::from_le_bytes([out[20], out[21]]), 0x0000);
        assert_eq!(u16::from_le_bytes([out[22], out[23]]), 0x0040);
        assert_eq!(u16::from_le_bytes([out[24], out[25]]), 0x0033);
        assert_eq!(rax, 0x8005_0033);
    }

    static HOST_TEXT_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    struct HostTextRangeGuard {
        _lock: std::sync::MutexGuard<'static, ()>,
    }

    impl HostTextRangeGuard {
        fn enter(start: usize, end: usize) -> Self {
            let lock = HOST_TEXT_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
            crate::exe::set_host_text_range(start, end);
            win32_rt::set_unregistered_host_text_checker(crate::exe::is_unregistered_host_text);
            Self { _lock: lock }
        }

        /// Record the exact Mach-O `__TEXT,__text` section of the current test binary so
        /// heap-allocated guest code buffers never collide with `in_host_text`.
        fn enter_current_image() -> Self {
            extern "C" {
                fn getsectiondata(
                    mhp: *const core::ffi::c_void,
                    segname: *const core::ffi::c_char,
                    sectname: *const core::ffi::c_char,
                    size: *mut usize,
                ) -> *const u8;
            }
            let (start, end) = unsafe {
                let mut info: win32_rt::libc::Dl_info = core::mem::zeroed();
                let anchor = Self::enter_current_image as *const () as *const core::ffi::c_void;
                assert_ne!(win32_rt::libc::dladdr(anchor, &mut info), 0, "dladdr failed");
                assert!(!info.dli_fbase.is_null(), "dli_fbase null");
                let mut size = 0usize;
                let ptr = getsectiondata(
                    info.dli_fbase,
                    c"__TEXT".as_ptr(),
                    c"__text".as_ptr(),
                    &mut size,
                );
                assert!(!ptr.is_null() && size > 0, "getsectiondata(__TEXT, __text) failed");
                (ptr as usize, ptr as usize + size)
            };
            Self::enter(start, end)
        }
    }

    impl Drop for HostTextRangeGuard {
        fn drop(&mut self) {
            win32_rt::clear_unregistered_host_text_checker();
            crate::exe::set_host_text_range(0, 0);
        }
    }

    #[test]
    fn unregistered_host_text_is_never_authorized_in_lookup_or_cached_dispatch() {
        unsafe extern "win64" fn reg_thunk_target(a: u64) -> u64 {
            a.wrapping_add(0x42)
        }

        // Record a neutral synthetic host-text range for negative-exclusion checks only.
        let sentinel_base: usize = 0x0000_0007_8000_0000;
        let sentinel_addr: usize = sentinel_base + 0x140;
        let _guard = HostTextRangeGuard::enter(sentinel_base, sentinel_base + 0x2000);
        assert!(crate::exe::in_host_text(sentinel_addr));

        // 1. Direct lookup, host-callable predicate, and memoized dispatch (cold + warm dcache)
        //    must all reject the unregistered host-text address without authorizing DF_HOST.
        assert_eq!(crate::exe::lookup_native_va(sentinel_addr as u64), None);
        assert_eq!(resolve_host_callable(sentinel_addr as u64), None);
        for _ in 0..3 {
            let (h, flags) = resolve_dispatch(sentinel_addr as u64);
            assert_eq!(h, 0, "unregistered host text must never populate host_fn");
            assert_eq!(flags & DF_HOST, 0, "unregistered host text must never set DF_HOST");
        }

        // 2. An explicitly registered synthetic thunk VA (bit 46 clear) dispatches cleanly
        //    to its approved target, and stops resolving once unregistered.
        let thunk_va: u64 = 0x0000_0007_7700_0200;
        let target_ptr = reg_thunk_target as *const () as usize;
        crate::exe::register_native_va(thunk_va, target_ptr);
        assert_eq!(crate::exe::lookup_native_va(thunk_va), Some(target_ptr));
        assert_eq!(resolve_host_callable(thunk_va), Some(target_ptr));
        assert_eq!(unsafe { call_va8(thunk_va, 10, 0, 0, 0, 0, 0, 0, 0) }, 10 + 0x42);
        crate::exe::unregister_native_va(thunk_va);
        assert_eq!(resolve_host_callable(thunk_va), None);
    }

    #[test]
    fn beginthread_dispatches_host_trampoline_through_call_va8_without_refusal() {
        // Real x86-64 guest thread entry:
        //   mov rax, [rcx]
        //   add rax, 0x2a
        //   mov [rcx], rax
        //   ret
        let guest_fn_va = super::alloc_guest_code(&[
            0x48, 0x8B, 0x01,
            0x48, 0x83, 0xC0, 0x2A,
            0x48, 0x89, 0x01,
            0xC3,
        ]);
        let _guard = HostTextRangeGuard::enter_current_image();
        assert!(
            !crate::exe::in_host_text(guest_fn_va as usize),
            "guest code buffer must lie outside host __TEXT"
        );
        win32_rt::set_guest_dispatcher(crate::exe::call_va8);
        win32_rt::set_native_va_registrar(crate::exe::register_native_va);

        let refusals_before = unregistered_host_refusal_count();
        let shared = std::sync::atomic::AtomicU64::new(0x1000);
        unsafe {
            let entry: unsafe extern "win64" fn(*mut core::ffi::c_void) =
                core::mem::transmute(guest_fn_va as usize);
            let h = win32_rt::ucrt::runtime::_beginthread(
                entry,
                0,
                (&shared as *const std::sync::atomic::AtomicU64) as *mut core::ffi::c_void,
            );
            assert_ne!(h, usize::MAX, "_beginthread must succeed");
            assert_ne!(h, 0, "_beginthread handle must be non-null");
            let wait = win32_rt::kernel32::threads::WaitForSingleObject(
                h as win32_rt::types::HANDLE,
                5_000,
            );
            assert_eq!(wait, 0, "WaitForSingleObject must return WAIT_OBJECT_0");
            win32_rt::kernel32::threads::CloseHandle(h as win32_rt::types::HANDLE);
        }
        assert_eq!(
            shared.load(std::sync::atomic::Ordering::Acquire),
            0x102A,
            "guest thread body launched via _beginthread must execute through begin_thread_tramp"
        );
        assert_eq!(
            unregistered_host_refusal_count(),
            refusals_before,
            "_beginthread must not trigger any unregistered host __TEXT refusal"
        );
    }

    #[test]
    fn nvapi_query_interface_and_static_vtables_dispatch_via_call_va8() {
        let cout_vtbl_slot0 = unsafe { *win32_rt::misc::msvcp140::COUT.ios.vtable }.0 as usize;
        let _guard = HostTextRangeGuard::enter_current_image();

        win32_rt::set_guest_dispatcher(crate::exe::call_va8);
        win32_rt::set_native_va_registrar(crate::exe::register_native_va);

        let refusals_before = unregistered_host_refusal_count();

        // 1. Public API path: nvapi_QueryInterface returns registered host-callable pointers.
        let p_ver = win32_rt::misc::nvapi64::nvapi_QueryInterface(
            win32_rt::misc::nvapi64::ID_NVAPI_GET_INTERFACE_VERSION_STRING,
        );
        assert_ne!(p_ver, 0);
        assert!(crate::exe::in_host_text(p_ver));
        assert!(win32_rt::audit_guest_callable(p_ver, "test_nvapi").is_ok());

        let mut buf = [0u8; 64];
        let rc = unsafe { call_va8(p_ver as u64, buf.as_mut_ptr() as u64, 0, 0, 0, 0, 0, 0, 0) } as i32;
        assert_eq!(rc, 0, "NvAPI_GetInterfaceVersionString must return NVAPI_OK via call_va8");
        let s = std::ffi::CStr::from_bytes_until_nul(&buf)
            .expect("NUL-terminated NVAPI version string")
            .to_str()
            .unwrap();
        assert!(s.contains("NVAPI"), "unexpected version string: {s:?}");

        // 2. Static data-exported vtable slots (msvcp140::COUT) are registered automatically
        //    when set_native_va_registrar runs, even before any msvcp140 function is called.
        assert!(crate::exe::in_host_text(cout_vtbl_slot0));
        assert_eq!(resolve_host_callable(cout_vtbl_slot0 as u64), Some(cout_vtbl_slot0));
        assert!(win32_rt::audit_guest_callable(cout_vtbl_slot0, "test_cout_vtbl").is_ok());

        assert_eq!(
            unregistered_host_refusal_count(),
            refusals_before,
            "registered host thunks must not increment refusal count"
        );
    }

    #[test]
    fn unregistered_host_text_refusal_symbolizes_and_audit_helper_catches_escape() {
        #[inline(never)]
        unsafe extern "win64" fn unregistered_escape_sentinel(x: u64) -> u64 {
            x.wrapping_add(1)
        }

        let addr = unregistered_escape_sentinel as *const () as usize;
        let _guard = HostTextRangeGuard::enter_current_image();
        assert!(crate::exe::in_host_text(addr));
        assert_eq!(resolve_host_callable(addr as u64), None);

        // 1. Audit helper fails cleanly when an unregistered host __TEXT pointer escapes.
        let err = win32_rt::audit_guest_callable(addr, "regression_test_site")
            .expect_err("audit_guest_callable must reject unregistered host __TEXT");
        assert!(err.contains("unregistered host __TEXT"), "unexpected audit err: {err}");
        assert!(err.contains("regression_test_site"), "missing site in audit err: {err}");

        // 2. dladdr symbolization resolves the sentinel function name.
        let sym = describe_host_addr(addr);
        assert!(
            sym.contains("unregistered_escape_sentinel"),
            "describe_host_addr should resolve sentinel symbol via dladdr, got: {sym}"
        );

        // 3. Both call_va8 and call_va8_xmm refuse the unregistered target safely and count it.
        let before = unregistered_host_refusal_count();
        assert_eq!(unsafe { call_va8(addr as u64, 41, 0, 0, 0, 0, 0, 0, 0) }, 0);
        assert_eq!(unregistered_host_refusal_count(), before + 1);
        assert_eq!(
            unsafe { crate::simd_ops::call_va8_xmm(addr as u64, 41, 0, 0, 0, 0, 0, 0, 0, [0; 4]) },
            (0, 0)
        );
        assert_eq!(unregistered_host_refusal_count(), before + 2);

        // 4. StrictHostThunkGuard turns a refusal into a panic naming the symbol.
        let panic_res = std::panic::catch_unwind(|| {
            let _strict = StrictHostThunkGuard::enter();
            unsafe { call_va8(addr as u64, 41, 0, 0, 0, 0, 0, 0, 0) }
        });
        assert!(panic_res.is_err(), "StrictHostThunkGuard must panic on unregistered host __TEXT");

        // 5. Once registered, audit passes and both call_va8 and call_va8_xmm dispatch natively.
        crate::exe::register_native_va(addr as u64, addr);
        assert!(win32_rt::audit_guest_callable(addr, "regression_test_site").is_ok());
        let after_reg = unregistered_host_refusal_count();
        assert_eq!(unsafe { call_va8(addr as u64, 41, 0, 0, 0, 0, 0, 0, 0) }, 42);
        assert_eq!(
            unsafe { crate::simd_ops::call_va8_xmm(addr as u64, 41, 0, 0, 0, 0, 0, 0, 0, [0; 4]).0 },
            42
        );
        assert_eq!(unregistered_host_refusal_count(), after_reg);
        crate::exe::unregister_native_va(addr as u64);
    }
}

#[test]
fn guest_timestamp_scaling_matches_wide_division_through_wraparound() {
    use super::tsc::{scale_counter_to_tsc, TSC_NOMINAL_HZ};
    for hz in [1, 19_200_000, 24_000_000, TSC_NOMINAL_HZ, u32::MAX as u64] {
        let wrap = ((1u128 << 64) * hz as u128 / TSC_NOMINAL_HZ as u128) as u64;
        for counter in [
            0, 1, 2, 3, hz, 1 << 56, wrap.wrapping_sub(1), wrap, wrap.wrapping_add(1),
            u64::MAX - 2, u64::MAX - 1, u64::MAX,
        ] {
            let expected = (counter as u128 * 1_000_000_000 / hz as u128) as u64;
            assert_eq!(scale_counter_to_tsc(counter, hz), expected, "counter={counter} hz={hz}");
        }
    }
    assert_eq!(scale_counter_to_tsc(123, 0), 123);
}
