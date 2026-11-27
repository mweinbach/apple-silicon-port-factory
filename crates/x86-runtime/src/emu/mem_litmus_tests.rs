//! Multi-threaded x86-64 memory-ordering litmus tests across Interpreter, JIT, and AOT.
//!
//! Exercises real concurrent execution on two Apple Silicon cores (`std::thread::scope` +
//! `std::sync::Barrier`) for:
//! - Message Passing (MP): scalar, 128-bit vector (`movdqu`), and bulk-copy (`rep movsq`)
//! - Store Buffering (SB): `mfence` and `xchg` (including inside `VolatileMetadata` covered ranges)
//! - PE `VolatileMetadata` MP preservation: non-volatile payload (`ldr`/`str`) + volatile flag (`stlr`/`ldapr`)

use crate::emu::{aot, call_va8, jit};
use iced_x86::code_asm::*;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Barrier;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ExecMode {
    Interp,
    Jit,
    Aot,
}

/// Thread-local execution-mode guard for a spawned litmus participant.
/// Ensures `ExecMode::Aot` enables AOT lookup on every worker thread and verifies at least one
/// AOT block installation actually occurred on that thread.
struct ParticipantGuard {
    mode: ExecMode,
    aot_before: u64,
    label: &'static str,
}

impl ParticipantGuard {
    fn enter(mode: ExecMode, label: &'static str) -> Self {
        let aot_before = jit::test_aot_installs();
        match mode {
            ExecMode::Interp => {
                aot::test_set_aot_enabled(false);
                jit::force(2);
            }
            ExecMode::Jit => {
                aot::test_set_aot_enabled(false);
                jit::force(1);
            }
            ExecMode::Aot => {
                aot::test_set_aot_enabled(true);
                jit::force(1);
            }
        }
        Self { mode, aot_before, label }
    }

    fn verify_and_exit(self) {
        let after = jit::test_aot_installs();
        let mode = self.mode;
        let before = self.aot_before;
        let label = self.label;
        drop(self);
        if mode == ExecMode::Aot {
            assert!(
                after > before,
                "{label}: ExecMode::Aot participant did not execute installed AOT code (installs {before} -> {after})"
            );
        } else {
            assert_eq!(
                after, before,
                "{label}: {mode:?} participant unexpectedly installed AOT code"
            );
        }
    }
}

impl Drop for ParticipantGuard {
    fn drop(&mut self) {
        jit::force(0);
        aot::test_set_aot_enabled(false);
    }
}

/// Page-aligned RW code buffer in valid guest address space for litmus tests.
#[repr(C, align(4096))]
struct CodeBuf {
    data: Box<[u8; 0x4000]>,
}

impl CodeBuf {
    fn new(_len: usize) -> Self {
        Self { data: Box::new([0u8; 0x4000]) }
    }

    fn base(&self) -> u64 {
        self.data.as_ptr() as u64
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn put(&mut self, off: usize, bytes: &[u8]) {
        self.data[off..off + bytes.len()].copy_from_slice(bytes);
    }
}

struct AotGuard;
impl Drop for AotGuard {
    fn drop(&mut self) {
        aot::test_uninstall();
    }
}

struct VolMetaGuard(u64);
impl Drop for VolMetaGuard {
    fn drop(&mut self) {
        aot::test_unregister_volatile_meta(self.0);
    }
}

/// Cache-line-separated shared state for 2-thread litmus tests.
#[repr(C, align(128))]
struct SharedPair {
    a: [AtomicU64; 16],
    _pad1: [u8; 128],
    b: [AtomicU64; 16],
    _pad2: [u8; 128],
    c: [AtomicU64; 16],
}

impl SharedPair {
    fn new() -> Box<Self> {
        Box::new(Self {
            a: core::array::from_fn(|_| AtomicU64::new(0)),
            _pad1: [0; 128],
            b: core::array::from_fn(|_| AtomicU64::new(0)),
            _pad2: [0; 128],
            c: core::array::from_fn(|_| AtomicU64::new(0)),
        })
    }
}

/// Message Passing (MP) — scalar payload (`mov [data], val; mov [flag], iter` vs spin `flag == iter`, load `data`)
/// across Interpreter, JIT, and AOT.
#[test]
fn litmus_mp_scalar_interp_jit_aot() {
    let mut code = CodeBuf::new(0x4000);
    let base = code.base();
    let prod_ip = base + 0x100;
    let cons_ip = base + 0x200;

    // Producer(rcx = &data, rdx = &flag, r8 = iter):
    //   mov [rcx], r8; mov [rdx], r8; ret
    let mut a = CodeAssembler::new(64).unwrap();
    a.mov(qword_ptr(rcx), r8).unwrap();
    a.mov(qword_ptr(rdx), r8).unwrap();
    a.ret().unwrap();
    code.put(0x100, &a.assemble(prod_ip).unwrap());

    // Consumer(rcx = &data, rdx = &flag, r8 = iter) -> rax = data:
    //   spin: mov rax, [rdx]; cmp rax, r8; jne spin
    //   mov rax, [rcx]; ret
    let mut a = CodeAssembler::new(64).unwrap();
    let mut spin = a.create_label();
    a.set_label(&mut spin).unwrap();
    a.mov(rax, qword_ptr(rdx)).unwrap();
    a.cmp(rax, r8).unwrap();
    a.jne(spin).unwrap();
    a.mov(rax, qword_ptr(rcx)).unwrap();
    a.ret().unwrap();
    code.put(0x200, &a.assemble(cons_ip).unwrap());

    let shared = SharedPair::new();
    let data_ptr = shared.a.as_ptr() as u64;
    let flag_ptr = shared.b.as_ptr() as u64;

    for mode in [ExecMode::Interp, ExecMode::Jit, ExecMode::Aot] {
        let _aot = if mode == ExecMode::Aot {
            let (live, _, _) = unsafe { aot::test_install(&[prod_ip, cons_ip], || {}) };
            assert!(live >= 2, "AOT must install both producer and consumer blocks");
            Some(AotGuard)
        } else {
            None
        };

        let iters: u64 = if mode == ExecMode::Interp { 1000 } else { 4000 };
        let start_bar = Barrier::new(2);
        let end_bar = Barrier::new(2);
        let stale = AtomicU64::new(0);

        std::thread::scope(|s| {
            let sb_r = &start_bar;
            let eb_r = &end_bar;
            let stale_r = &stale;
            let shared_r = &*shared;
            let t_cons = s.spawn(move || {
                let g = ParticipantGuard::enter(mode, "mp-scalar-consumer");
                for iter in 1..=iters {
                    sb_r.wait();
                    let got = unsafe { call_va8(cons_ip, data_ptr, flag_ptr, iter, 0, 0, 0, 0, 0) };
                    if got != iter {
                        stale_r.fetch_add(1, Ordering::Relaxed);
                    }
                    eb_r.wait();
                }
                g.verify_and_exit();
            });
            let t_prod = s.spawn(move || {
                let g = ParticipantGuard::enter(mode, "mp-scalar-producer");
                for iter in 1..=iters {
                    shared_r.a[0].store(0, Ordering::SeqCst);
                    shared_r.b[0].store(0, Ordering::SeqCst);
                    sb_r.wait();
                    unsafe { call_va8(prod_ip, data_ptr, flag_ptr, iter, 0, 0, 0, 0, 0) };
                    eb_r.wait();
                }
                g.verify_and_exit();
            });
            t_prod.join().unwrap();
            t_cons.join().unwrap();
        });

        assert_eq!(
            stale.load(Ordering::Relaxed),
            0,
            "scalar MP observed stale data in {mode:?}"
        );
    }
}

/// Message Passing (MP) — 128-bit vector (`movdqu`) and bulk-copy (`rep movsq`) payloads
/// across Interpreter, JIT, and AOT.
#[test]
fn litmus_mp_vector_and_rep_movs_interp_jit_aot() {
    let mut code = CodeBuf::new(0x4000);
    let base = code.base();
    let v_prod = base + 0x100;
    let v_cons = base + 0x200;
    let m_prod = base + 0x300;
    let m_cons = base + 0x400;

    // Vector Producer(rcx = &data16, rdx = &flag, r8 = iter):
    //   movq xmm0, r8; mov r9, r8; xor r9, rdx; pinsrq xmm0, r9, 1
    //   movdqu [rcx], xmm0; mov [rdx], r8; ret
    let mut a = CodeAssembler::new(64).unwrap();
    a.movq(xmm0, r8).unwrap();
    a.mov(r9, r8).unwrap();
    a.xor(r9, rdx).unwrap();
    a.pinsrq(xmm0, r9, 1).unwrap();
    a.movdqu(xmmword_ptr(rcx), xmm0).unwrap();
    a.mov(qword_ptr(rdx), r8).unwrap();
    a.ret().unwrap();
    code.put(0x100, &a.assemble(v_prod).unwrap());

    // Vector Consumer(rcx = &data16, rdx = &flag, r8 = iter) -> rax (1 = ok, 0 = stale/torn)
    let mut a = CodeAssembler::new(64).unwrap();
    let mut spin = a.create_label();
    let mut bad = a.create_label();
    a.set_label(&mut spin).unwrap();
    a.mov(rax, qword_ptr(rdx)).unwrap();
    a.cmp(rax, r8).unwrap();
    a.jne(spin).unwrap();
    a.movdqu(xmm0, xmmword_ptr(rcx)).unwrap();
    a.movq(rax, xmm0).unwrap();
    a.pextrq(r9, xmm0, 1).unwrap();
    a.cmp(rax, r8).unwrap();
    a.jne(bad).unwrap();
    a.xor(r9, rdx).unwrap();
    a.cmp(r9, r8).unwrap();
    a.jne(bad).unwrap();
    a.mov(eax, 1).unwrap();
    a.ret().unwrap();
    a.set_label(&mut bad).unwrap();
    a.xor(eax, eax).unwrap();
    a.ret().unwrap();
    code.put(0x200, &a.assemble(v_cons).unwrap());

    // Bulk rep movsq Producer(rcx = &src64, rdx = &dst64, r8 = &flag, r9 = iter):
    //   push rsi; push rdi; mov rsi, rcx; mov rdi, rdx; mov rcx, 8; cld; rep movsq
    //   mov [r8], r9; pop rdi; pop rsi; ret
    let mut a = CodeAssembler::new(64).unwrap();
    a.push(rsi).unwrap();
    a.push(rdi).unwrap();
    a.mov(rsi, rcx).unwrap();
    a.mov(rdi, rdx).unwrap();
    a.mov(rcx, 8u64).unwrap();
    a.cld().unwrap();
    a.rep().movsq().unwrap();
    a.mov(qword_ptr(r8), r9).unwrap();
    a.pop(rdi).unwrap();
    a.pop(rsi).unwrap();
    a.ret().unwrap();
    code.put(0x300, &a.assemble(m_prod).unwrap());

    // Bulk rep movsq Consumer(rcx = &src64, rdx = &local64, r8 = &flag, r9 = iter) -> rax (1 = ok)
    let mut a = CodeAssembler::new(64).unwrap();
    let mut spin2 = a.create_label();
    let mut bad2 = a.create_label();
    a.push(rsi).unwrap();
    a.push(rdi).unwrap();
    a.set_label(&mut spin2).unwrap();
    a.mov(rax, qword_ptr(r8)).unwrap();
    a.cmp(rax, r9).unwrap();
    a.jne(spin2).unwrap();
    a.mov(rsi, rcx).unwrap();
    a.mov(rdi, rdx).unwrap();
    a.mov(rcx, 8u64).unwrap();
    a.cld().unwrap();
    a.rep().movsq().unwrap();
    // Verify first and last qword of the 64-byte copy match `r9`.
    a.cmp(qword_ptr(rdx), r9).unwrap();
    a.jne(bad2).unwrap();
    a.cmp(qword_ptr(rdx + 56), r9).unwrap();
    a.jne(bad2).unwrap();
    a.mov(eax, 1).unwrap();
    a.pop(rdi).unwrap();
    a.pop(rsi).unwrap();
    a.ret().unwrap();
    a.set_label(&mut bad2).unwrap();
    a.xor(eax, eax).unwrap();
    a.pop(rdi).unwrap();
    a.pop(rsi).unwrap();
    a.ret().unwrap();
    code.put(0x400, &a.assemble(m_cons).unwrap());

    let shared = SharedPair::new();
    let vec_data = shared.a.as_ptr() as u64;
    let flag_ptr = shared.b.as_ptr() as u64;
    let bulk_dst = shared.c.as_ptr() as u64;

    for mode in [ExecMode::Interp, ExecMode::Jit, ExecMode::Aot] {
        let _aot = if mode == ExecMode::Aot {
            let (live, _, _) = unsafe { aot::test_install(&[v_prod, v_cons, m_prod, m_cons], || {}) };
            assert!(live >= 4, "AOT must install vector and rep movsq litmus blocks");
            Some(AotGuard)
        } else {
            None
        };
        let iters: u64 = if mode == ExecMode::Interp { 500 } else { 2000 };

        // 1. 128-bit vector MP
        let start_bar = Barrier::new(2);
        let end_bar = Barrier::new(2);
        let vec_stale = AtomicU64::new(0);
        std::thread::scope(|s| {
            let sb_r = &start_bar;
            let eb_r = &end_bar;
            let stale_r = &vec_stale;
            let shared_r = &*shared;
            let t_cons = s.spawn(move || {
                let g = ParticipantGuard::enter(mode, "mp-vec-consumer");
                for iter in 1..=iters {
                    sb_r.wait();
                    let ok = unsafe { call_va8(v_cons, vec_data, flag_ptr, iter, 0, 0, 0, 0, 0) };
                    if ok != 1 {
                        stale_r.fetch_add(1, Ordering::Relaxed);
                    }
                    eb_r.wait();
                }
                g.verify_and_exit();
            });
            let t_prod = s.spawn(move || {
                let g = ParticipantGuard::enter(mode, "mp-vec-producer");
                for iter in 1..=iters {
                    shared_r.a[0].store(0, Ordering::SeqCst);
                    shared_r.a[1].store(0, Ordering::SeqCst);
                    shared_r.b[0].store(0, Ordering::SeqCst);
                    sb_r.wait();
                    unsafe { call_va8(v_prod, vec_data, flag_ptr, iter, 0, 0, 0, 0, 0) };
                    eb_r.wait();
                }
                g.verify_and_exit();
            });
            t_prod.join().unwrap();
            t_cons.join().unwrap();
        });
        assert_eq!(vec_stale.load(Ordering::Relaxed), 0, "vector MP stale/torn in {mode:?}");

        // 2. Bulk rep movsq MP
        let start_bar = Barrier::new(2);
        let end_bar = Barrier::new(2);
        let rep_stale = AtomicU64::new(0);
        std::thread::scope(|s| {
            let sb_r = &start_bar;
            let eb_r = &end_bar;
            let stale_r = &rep_stale;
            let shared_r = &*shared;
            let t_cons = s.spawn(move || {
                let g = ParticipantGuard::enter(mode, "mp-repmovs-consumer");
                let mut local = [0u64; 8];
                let local_ptr = local.as_mut_ptr() as u64;
                for iter in 1..=iters {
                    sb_r.wait();
                    let ok = unsafe { call_va8(m_cons, bulk_dst, local_ptr, flag_ptr, iter, 0, 0, 0, 0) };
                    if ok != 1 {
                        stale_r.fetch_add(1, Ordering::Relaxed);
                    }
                    eb_r.wait();
                }
                g.verify_and_exit();
            });
            let t_prod = s.spawn(move || {
                let g = ParticipantGuard::enter(mode, "mp-repmovs-producer");
                let mut src_buf = [0u64; 8];
                let src_ptr = src_buf.as_mut_ptr() as u64;
                for iter in 1..=iters {
                    for slot in &shared_r.c {
                        slot.store(0, Ordering::SeqCst);
                    }
                    shared_r.b[0].store(0, Ordering::SeqCst);
                    src_buf.fill(iter);
                    sb_r.wait();
                    unsafe { call_va8(m_prod, src_ptr, bulk_dst, flag_ptr, iter, 0, 0, 0, 0) };
                    eb_r.wait();
                }
                g.verify_and_exit();
            });
            t_prod.join().unwrap();
            t_cons.join().unwrap();
        });
        assert_eq!(rep_stale.load(Ordering::Relaxed), 0, "rep movsq MP stale in {mode:?}");
    }
}

/// Store Buffering (SB) with `mfence` and `xchg` across Interpreter, JIT, and AOT, including
/// inside a PE `VolatileMetadata` covered range (`mfence` and `xchg` must remain unconditionally
/// ordered even when surrounding plain accesses are covered).
#[test]
fn litmus_sb_mfence_and_xchg_interp_jit_aot() {
    let mut code = CodeBuf::new(0x4000);
    let base = code.base();
    let sb_mfence = base + 0x100;
    let sb_xchg = base + 0x200;

    // SB-mfence(rcx = &mine, rdx = &theirs) -> eax = *theirs:
    //   mov dword [rcx], 1; mfence; mov eax, [rdx]; ret
    let mut a = CodeAssembler::new(64).unwrap();
    a.mov(dword_ptr(rcx), 1).unwrap();
    a.mfence().unwrap();
    a.mov(eax, dword_ptr(rdx)).unwrap();
    a.ret().unwrap();
    code.put(0x100, &a.assemble(sb_mfence).unwrap());

    // SB-xchg(rcx = &mine, rdx = &theirs) -> eax = *theirs:
    //   mov eax, 1; xchg [rcx], eax; mov eax, [rdx]; ret
    let mut a = CodeAssembler::new(64).unwrap();
    a.mov(eax, 1).unwrap();
    a.xchg(dword_ptr(rcx), eax).unwrap();
    a.mov(eax, dword_ptr(rdx)).unwrap();
    a.ret().unwrap();
    code.put(0x200, &a.assemble(sb_xchg).unwrap());

    // Cover [0x100, 0x300) with empty VolatileAccessTable so plain stores/loads elide TSO
    // while `mfence` and `xchg` MUST still enforce full Store-Buffering ordering!
    let _vol = VolMetaGuard(base);
    aot::test_register_volatile_meta(base, code.len() as u64, &[(0x100, 0x300)], &[]);

    let shared = SharedPair::new();
    let x_ptr = shared.a.as_ptr() as u64;
    let y_ptr = shared.b.as_ptr() as u64;

    for (label, fn_ip) in [("mfence", sb_mfence), ("xchg", sb_xchg)] {
        for mode in [ExecMode::Interp, ExecMode::Jit, ExecMode::Aot] {
            let _aot = if mode == ExecMode::Aot {
                let (live, _, _) = unsafe { aot::test_install(&[fn_ip], || {}) };
                assert!(live >= 1);
                Some(AotGuard)
            } else {
                None
            };
            let iters = if mode == ExecMode::Interp { 600 } else { 2000 };
            let start_bar = Barrier::new(2);
            let end_bar = Barrier::new(2);
            let r1_slot = AtomicU64::new(0);
            let forbidden = AtomicU64::new(0);

            std::thread::scope(|s| {
                let sb_r = &start_bar;
                let eb_r = &end_bar;
                let r1_r = &r1_slot;
                let forb_r = &forbidden;
                let shared_r = &*shared;
                let t1 = s.spawn(move || {
                    let g = ParticipantGuard::enter(mode, "sb-t1");
                    for _ in 0..iters {
                        sb_r.wait();
                        let r1 = unsafe { call_va8(fn_ip, y_ptr, x_ptr, 0, 0, 0, 0, 0, 0) };
                        r1_r.store(r1, Ordering::Relaxed);
                        eb_r.wait();
                    }
                    g.verify_and_exit();
                });
                let t0 = s.spawn(move || {
                    let g = ParticipantGuard::enter(mode, "sb-t0");
                    for _ in 0..iters {
                        shared_r.a[0].store(0, Ordering::SeqCst);
                        shared_r.b[0].store(0, Ordering::SeqCst);
                        sb_r.wait();
                        let r0 = unsafe { call_va8(fn_ip, x_ptr, y_ptr, 0, 0, 0, 0, 0, 0) };
                        eb_r.wait();
                        let r1 = r1_r.load(Ordering::Relaxed);
                        if r0 == 0 && r1 == 0 {
                            forb_r.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                    g.verify_and_exit();
                });
                t0.join().unwrap();
                t1.join().unwrap();
            });

            assert_eq!(
                forbidden.load(Ordering::Relaxed),
                0,
                "SB-{label} observed forbidden (0, 0) outcome in {mode:?}"
            );
        }
    }
}

/// PE `VolatileMetadata` Message Passing preservation: producer and consumer reside inside a
/// covered range where the `data` accesses are non-volatile (elided to plain `ldr`/`str`) and
/// ONLY the `flag` store/load instruction RVAs are listed in `VolatileAccessTable` (`stlr`/`ldapr`).
#[test]
fn litmus_mp_volatile_metadata_preserves_synchronization() {
    let mut code = CodeBuf::new(0x4000);
    let base = code.base();
    let prod_ip = base + 0x100;
    let cons_ip = base + 0x200;

    // Producer @ 0x100:
    //   mov [rcx], r8        (non-volatile, covered -> plain str)
    //   mov [rdx], r8        (LISTED volatile -> stlr/stlur)
    //   ret
    let mut a = CodeAssembler::new(64).unwrap();
    a.mov(qword_ptr(rcx), r8).unwrap();
    let vol_store_rva = 0x100 + a.assemble(prod_ip).unwrap().len() as u32;
    a.mov(qword_ptr(rdx), r8).unwrap();
    a.ret().unwrap();
    code.put(0x100, &a.assemble(prod_ip).unwrap());

    // Consumer @ 0x200:
    //   spin: mov rax, [rdx] (LISTED volatile -> ldapr/ldapur)
    //   cmp rax, r8; jne spin
    //   mov rax, [rcx]       (non-volatile, covered -> plain ldr)
    //   ret
    let vol_load_rva = 0x200u32;
    let mut a = CodeAssembler::new(64).unwrap();
    let mut spin = a.create_label();
    a.set_label(&mut spin).unwrap();
    a.mov(rax, qword_ptr(rdx)).unwrap();
    a.cmp(rax, r8).unwrap();
    a.jne(spin).unwrap();
    let data_load_rva = 0x200 + a.assemble(cons_ip).unwrap().len() as u32;
    a.mov(rax, qword_ptr(rcx)).unwrap();
    a.ret().unwrap();
    code.put(0x200, &a.assemble(cons_ip).unwrap());

    let _vol = VolMetaGuard(base);
    aot::test_register_volatile_meta(
        base,
        code.len() as u64,
        &[(0x100, 0x300)],
        &[vol_store_rva, vol_load_rva],
    );

    // Sanity check metadata classification before running concurrent threads.
    let meta = aot::lookup_module_for_ip(prod_ip).expect("registered volatile meta");
    assert!(meta.can_elide_tso_va(prod_ip), "data store must elide TSO");
    assert!(!meta.can_elide_tso_va(base + vol_store_rva as u64), "flag store must keep TSO");
    assert!(!meta.can_elide_tso_va(base + vol_load_rva as u64), "flag load must keep TSO");
    assert!(meta.can_elide_tso_va(base + data_load_rva as u64), "data load must elide TSO");

    let shared = SharedPair::new();
    let data_ptr = shared.a.as_ptr() as u64;
    let flag_ptr = shared.b.as_ptr() as u64;

    for mode in [ExecMode::Jit, ExecMode::Aot] {
        let _aot = if mode == ExecMode::Aot {
            let (live, _, _) = unsafe { aot::test_install(&[prod_ip, cons_ip], || {}) };
            assert!(live >= 2);
            Some(AotGuard)
        } else {
            None
        };

        let iters = 3000u64;
        let start_bar = Barrier::new(2);
        let end_bar = Barrier::new(2);
        let stale = AtomicU64::new(0);
        std::thread::scope(|s| {
            let sb_r = &start_bar;
            let eb_r = &end_bar;
            let stale_r = &stale;
            let shared_r = &*shared;
            let t_cons = s.spawn(move || {
                let g = ParticipantGuard::enter(mode, "mp-volmeta-consumer");
                for iter in 1..=iters {
                    sb_r.wait();
                    let got = unsafe { call_va8(cons_ip, data_ptr, flag_ptr, iter, 0, 0, 0, 0, 0) };
                    if got != iter {
                        stale_r.fetch_add(1, Ordering::Relaxed);
                    }
                    eb_r.wait();
                }
                g.verify_and_exit();
            });
            let t_prod = s.spawn(move || {
                let g = ParticipantGuard::enter(mode, "mp-volmeta-producer");
                for iter in 1..=iters {
                    shared_r.a[0].store(0, Ordering::SeqCst);
                    shared_r.b[0].store(0, Ordering::SeqCst);
                    sb_r.wait();
                    unsafe { call_va8(prod_ip, data_ptr, flag_ptr, iter, 0, 0, 0, 0, 0) };
                    eb_r.wait();
                }
                g.verify_and_exit();
            });
            t_prod.join().unwrap();
            t_cons.join().unwrap();
        });

        assert_eq!(
            stale.load(Ordering::Relaxed),
            0,
            "VolatileMetadata MP observed stale data in {mode:?}"
        );
    }
}
