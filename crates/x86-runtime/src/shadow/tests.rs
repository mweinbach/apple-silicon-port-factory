//! Shadow probe tests: each probe forks an orchestrator from the test thread.

use super::*;
use win32_rt::libc;

/// Probes fork; keep them one at a time.
static SERIAL: Mutex<()> = Mutex::new(());

fn cfg(name: &str) -> ProbeCfg {
    let mut c = ProbeCfg::new(0x1_4000_1000, name);
    c.timeout_ms = 3000;
    c.maxpages = 1024;
    c.dump_dir = std::env::var("AW2_SHADOW_TEST_DUMP").unwrap_or_default();
    c
}

fn probe(name: &str, ret_xmm: bool, g: &dyn Fn() -> (u64, u128), n: &dyn Fn() -> (u64, u128)) -> Report {
    let _l = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    unsafe { probe_blocking(&cfg(name), &ArgRegs::default(), ret_xmm, g, n) }
}

fn leak_buf(len: usize) -> usize {
    Box::leak(vec![0u8; len].into_boxed_slice()).as_mut_ptr() as usize
}

#[test]
fn spec_parse() {
    let s = Spec::parse("va=0x140001000,0x140002000; class=draft|tpl; shard=r_more; n=3; rate=50; timeout=900; jit=1")
        .unwrap();
    assert_eq!(s.vas, vec![0x1_4000_1000, 0x1_4000_2000]);
    assert_eq!(s.classes, C_DRAFT | C_TPL);
    assert_eq!(s.shards, vec!["r_more".to_string()]);
    assert_eq!((s.n, s.rate_ms, s.timeout_ms, s.jit), (3, 50, 900, true));
    assert_eq!(Spec::parse("n=2;n_va=4").unwrap().n_va, 4);
    assert!(Spec::parse("bogus=1").is_err());
    assert!(Spec::parse("class=nope").is_err());
}

#[test]
fn selection_rules() {
    let f = |va, group, classes, throws| FnInfo { va, name: format!("f{va:x}"), group, classes, throws };
    let all = vec![
        f(1, "templates_a", C_DRAFT | C_TPL, false),
        f(2, "r_more", C_DRAFT, false),
        f(3, "base", C_VERIFIED, false),
        f(4, "r_more", C_DRAFT, true),
    ];
    let mut s = Spec::parse("class=tpl;shard=r_more").unwrap();
    let got: Vec<u64> = choose(&s, all.clone()).iter().map(|f| f.va).collect();
    assert_eq!(got, vec![1, 2]); // 4 throws
    s.throws = true;
    s.skip = vec![1];
    let got: Vec<u64> = choose(&s, all.clone()).iter().map(|f| f.va).collect();
    assert_eq!(got, vec![2, 4]);
    let s = Spec::parse("class=verified;va=4").unwrap();
    let got: Vec<u64> = choose(&s, all).iter().map(|f| f.va).collect();
    assert_eq!(got, vec![3, 4]); // explicit va overrides the throws filter
}

#[test]
fn explicit_vas_get_their_own_sample_budget() {
    let f = |va| FnInfo { va, name: format!("f{va:x}"), group: "base", classes: C_DRAFT, throws: false };
    let spec = Spec::parse("class=draft;va=2;n=2;n_va=4").unwrap();
    let st = State::new(spec, vec![f(1), f(2)], (-1, -1), None);
    let lim = |va| st.slot(st.lookup(va).unwrap()).limit;
    assert_eq!((lim(1), lim(2)), (2, 4));
    let st = State::new(Spec::parse("class=draft;va=2;n=3").unwrap(), vec![f(1), f(2)], (-1, -1), None);
    assert_eq!(st.slot(st.lookup(2).unwrap()).limit, 3);
}

#[test]
fn late_dll_registration_follows_spec() {
    let dll = |va: u64| FnInfo { va, name: format!("d{va:x}"), group: "dll", classes: C_DLL, throws: false };
    let st = |spec: &str| State::new(Spec::parse(spec).unwrap(), vec![], (-1, -1), None);

    let s = st("class=dll");
    assert!(s.lookup(0x7000).is_none());
    assert!(s.add_late(dll(0x7000)));
    assert_eq!(s.lookup(0x7000), Some(0));
    assert_eq!(s.slot(0).info.va, 0x7000);
    assert!(!s.add_late(dll(0x7000)), "duplicate va");
    assert!(s.add_late(dll(0x7100)));
    assert_eq!(s.lookup(0x7100), Some(1));
    assert_eq!(s.slot_list().len(), 2);

    assert!(!st("class=verified").add_late(dll(0x7000)));
    let s = st("va=0x5000");
    assert!(!s.add_late(dll(0x7000)));
    assert!(s.add_late(dll(0x5000)));
    assert!(!st("class=dll;skip=0x7000").add_late(dll(0x7000)));

    // Init-time slots keep their indices; late ones append.
    let s = State::new(Spec::parse("class=dll").unwrap(), vec![dll(1), dll(2)], (-1, -1), None);
    assert!(s.add_late(dll(3)));
    assert_eq!((s.lookup(1), s.lookup(2), s.lookup(3)), (Some(0), Some(1), Some(2)));
}

fn probe_ret(name: &str, ret: RetUse, ret_xmm: bool, g: &dyn Fn() -> (u64, u128), n: &dyn Fn() -> (u64, u128)) -> Report {
    let _l = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let mut c = cfg(name);
    c.ret = ret;
    unsafe { probe_blocking(&c, &ArgRegs::default(), ret_xmm, g, n) }
}

/// Address of a local `depth` frames down (each frame holds a 256-byte array).
#[inline(never)]
fn local_addr(depth: u32) -> usize {
    let pad = [depth as u8; 256];
    core::hint::black_box(&pad);
    if depth == 0 {
        core::hint::black_box(pad.as_ptr() as usize)
    } else {
        core::hint::black_box(local_addr(depth - 1))
    }
}

#[test]
fn stack_addresses_on_both_sides_are_stack_layout_not_equal() {
    let out = leak_buf(64);
    let st = |d: u32| move || unsafe {
        let a = local_addr(d);
        *(out as *mut usize) = a;
        (a as u64, 0u128)
    };
    let r = probe("stack_layout", false, &st(1), &st(3));
    assert_eq!(r.outcome(), Outcome::StackLayout, "{r:?} diag={:?}", diag());
    assert_eq!(r.stack_layout_ranges, 1, "{r:?}");
    // Only rax (dangling frame pointer returned) differs.
    let rax_only = |d: u32| move || (local_addr(d) as u64, 0u128);
    let r = probe("stack_rax", false, &rax_only(1), &rax_only(4));
    assert_eq!(r.outcome(), Outcome::StackLayout, "{r:?} diag={:?}", diag());
    // A stack address on one side only stays a mismatch.
    let heap = leak_buf(64);
    let n = move || unsafe {
        *(out as *mut usize) = heap;
        (heap as u64, 0u128)
    };
    let r = probe("stack_one_side", false, &st(1), &n);
    assert_eq!(r.outcome(), Outcome::Mismatch, "{r:?} diag={:?}", diag());
}

/// A qword in this thread's lstack arena below the top (inside the poisoning window), with
/// stale content set by the test (the clean-arena invariant is restored afterwards).
fn stale_arena_slot() -> usize {
    let c = crate::callctx::ctx();
    unsafe { crate::lstack::shadow_warm(c) };
    let top = c.ls_top.get();
    assert!(c.ls_lo.get() != 0 && top > c.ls_lo.get() + 0x10000, "lstack arena");
    top - 0x8000
}

/// g4 renoir +0x4c650: an emulated original copies an uninitialised padding byte of its own
/// stack frame to the heap; the replacement writes 0 (zeroed frame). The frame sits right
/// below the emulator's live stack top, in the page holding it: it must be poisoned in G3, so
/// the byte scores uninit, not mismatch.
#[test]
fn emulated_callee_copying_stale_frame_padding_is_uninit() {
    // sub rsp,0x48; mov byte [rsp+0x10],5; add rsp,0x48; ret
    let dirty: &[u8] = &[0x48, 0x83, 0xEC, 0x48, 0xC6, 0x44, 0x24, 0x10, 0x05, 0x48, 0x83, 0xC4, 0x48, 0xC3];
    // sub rsp,0x48; mov rax,[rsp+0x10]; mov [rcx],rax; xor eax,eax; add rsp,0x48; ret
    let copy: &[u8] = &[
        0x48, 0x83, 0xEC, 0x48, 0x48, 0x8B, 0x44, 0x24, 0x10, 0x48, 0x89, 0x01, 0x31, 0xC0, 0x48, 0x83, 0xC4,
        0x48, 0xC3,
    ];
    let dc = leak_buf(64);
    let cc = leak_buf(64);
    let out = leak_buf(64);
    unsafe {
        core::ptr::copy_nonoverlapping(dirty.as_ptr(), dc as *mut u8, dirty.len());
        core::ptr::copy_nonoverlapping(copy.as_ptr(), cc as *mut u8, copy.len());
    }
    let _l = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    // Leave a stale 5 where the copier's frame will be (same thread, same emulator stack top).
    unsafe { crate::emu::emu_run_full(dc as u64, [0; 8], [0; 4], [0; 8]) };
    let g = move || unsafe { crate::emu::emu_run_full(cc as u64, [out as u64, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]) };
    let n = move || unsafe {
        *(out as *mut u64) = 0;
        (0u64, 0u128)
    };
    let r = unsafe { probe_blocking(&cfg("emu_uninit_padding"), &ArgRegs::default(), false, &g, &n) };
    assert_eq!(r.outcome(), Outcome::Uninit, "{r:?} diag={:?}", diag());
    assert!(r.uninit_ranges[0] >= 1 && r.uninit_ranges[1] == 0, "{r:?}");
}

#[test]
fn bytes_from_uninitialised_stack_are_uninit_not_equal() {
    let slot = stale_arena_slot();
    let out = leak_buf(64);
    unsafe { *(slot as *mut u64) = 0x1234_5678 };
    // Original copies a stale stack qword to the heap; the replacement writes 0 (zeroed frame).
    let g = move || unsafe {
        *(out as *mut u64) = *(slot as *const u64);
        (0u64, 0u128)
    };
    let n = move || unsafe {
        *(out as *mut u64) = 0;
        (0u64, 0u128)
    };
    let r = probe("uninit", false, &g, &n);
    assert_eq!(r.outcome(), Outcome::Uninit, "{r:?} diag={:?}", diag());
    assert_eq!(r.poison & types::POISON_RAN, types::POISON_RAN, "{r:?}");
    assert!(r.uninit_ranges[0] >= 1 && r.uninit_ranges[1] == 0, "{r:?}");
    // The replacement reading its own uninitialised stack shows up as N != N3.
    let n_reads = move || unsafe {
        *(out as *mut u64) = *(slot as *const u64) ^ 1;
        (0u64, 0u128)
    };
    let r = probe("uninit_native", false, &g, &n_reads);
    assert!(r.uninit_ranges[1] >= 1, "{r:?} diag={:?}", diag());
    // A difference the poisoning does not move is still a mismatch.
    let g2 = move || unsafe {
        *(out as *mut u64) = *(slot as *const u64);
        *((out + 16) as *mut u8) = 6;
        (0u64, 0u128)
    };
    let n2 = move || unsafe {
        *(out as *mut u64) = 0;
        *((out + 16) as *mut u8) = 5;
        (0u64, 0u128)
    };
    let r = probe("uninit_plus_real", false, &g2, &n2);
    assert_eq!(r.outcome(), Outcome::Mismatch, "{r:?} diag={:?}", diag());
    // Equal runs never pay for the poisoned reruns.
    let r = probe("uninit_equal", false, &n, &n);
    assert_eq!((r.outcome(), r.poison), (Outcome::Equal, 0), "{r:?}");
    unsafe { *(slot as *mut u64) = 0 };
}

#[test]
fn free_big_frame_slots_and_frame_list_are_excluded() {
    use crate::simd_ops::{init_lifted_stack, LiftedStack};
    #[inline(never)]
    fn big_frame_writes(v: u64) -> u64 {
        unsafe {
            let mut s = LiftedStack::<1>::new();
            let rsp = init_lifted_stack(&mut s, 1, 2, 3, 4);
            ((rsp - 0x100) as *mut u64).write(v);
            ((rsp + 0x18) as *mut u64).write(v);
            core::hint::black_box(&s);
            0
        }
    }
    // Create this thread's big-frame arena before the probe, as in a game thread.
    big_frame_writes(0);
    let g = || (0u64, 0u128);
    let n = || (big_frame_writes(0x5a5a), 0u128);
    let r = probe("big_frame", false, &g, &n);
    assert_eq!(r.outcome(), Outcome::Equal, "{r:?} diag={:?}", diag());
}

static PRE_TRACK_AT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

fn pre_track_write() {
    let a = PRE_TRACK_AT.load(std::sync::atomic::Ordering::Relaxed);
    unsafe { (a as *mut u8).write_volatile(0xEE) };
}

/// Writes made in a run child before tracking starts (libSystem's fork-child handlers fill
/// objc method caches, ...) are not the run's own: a run that later dirties the same page
/// must not report them (this made `free_big_frame_...` flaky, depending on heap layout).
#[test]
fn pre_tracking_child_writes_are_not_run_diffs() {
    let _l = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let pg = super::handler::page();
    let buf = leak_buf(4 * pg);
    let a = (buf + pg - 1) & !(pg - 1);
    PRE_TRACK_AT.store(a, std::sync::atomic::Ordering::Relaxed);
    let g = || (0u64, 0u128);
    // Dirties the page without changing it.
    let n = || unsafe {
        let p = (a + 100) as *mut u8;
        p.write_volatile(p.read_volatile());
        (0u64, 0u128)
    };
    super::run::TEST_PRE_TRACK.store(pre_track_write as fn() as usize, std::sync::atomic::Ordering::Relaxed);
    let r = unsafe { probe_blocking(&cfg("pre_track"), &ArgRegs::default(), false, &g, &n) };
    super::run::TEST_PRE_TRACK.store(0, std::sync::atomic::Ordering::Relaxed);
    assert!(r.pages[1] >= 1, "N must dirty the page: {r:?}");
    assert_eq!(r.outcome(), Outcome::Equal, "{r:?} diag={:?}", diag());
}

#[test]
fn rets_tables_are_merged_and_keep_every_row() {
    let dir = std::env::temp_dir().join(format!("aw2_rets_{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let a = dir.join("a.tsv");
    let b = dir.join("b.tsv");
    std::fs::write(&a, "# c\nva\tsites\trax\txmm\tind\tsig\n0x140001000\t1\t0\t0\t0\t?\n").unwrap();
    std::fs::write(&b, "0x7001f13df0\t2\t8\t?\t0\t?\n").unwrap();
    let m = load_rets(&format!("{},{}", a.display(), b.display()));
    assert_eq!(m.len(), 2);
    assert_eq!(m[&0x1_4000_1000].rax_mask, 0);
    assert_eq!(m[&0x70_01f1_3df0].rax_mask, 0xff);
    // Late entries (not in the init-time selection) see their row.
    let mut st = State::new(Spec::parse("class=dll").unwrap(), vec![], (-1, -1), None);
    st.rets = m;
    assert_eq!(st.ret_use(0x70_01f1_3df0).rax_mask, 0xff);
    assert_eq!(st.ret_use(0x70_0000_0000), RetUse::default());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn ret_use_rows() {
    let full = RetUse::default();
    assert_eq!(RetUse::from_row("0", "0", true, "void"), full, "unknown callers: full compare");
    let r = RetUse::from_row("0", "?", false, "?");
    assert_eq!((r.rax_mask, r.xmm, r.src), (0, true, types::RET_X86));
    let r = RetUse::from_row("8", "?", false, "?");
    assert_eq!((r.rax_mask, r.xmm), (0xff, false), "rax live -> xmm0 is not the return register");
    let r = RetUse::from_row("?", "?", false, "void");
    assert_eq!((r.rax_mask, r.xmm, r.src), (0, false, types::RET_SIG | (types::RET_SIG << 4)));
    let r = RetUse::from_row("?", "?", false, "float");
    assert_eq!((r.rax_mask, r.xmm), (0, true));
    let r = RetUse::from_row("?", "?", false, "int");
    assert_eq!((r.rax_mask, r.xmm), (u64::MAX, false));
    assert_eq!(RetUse::from_row("?", "?", false, "?"), full);
    let r = RetUse::from_row("64", "1", false, "void");
    assert_eq!((r.rax_mask, r.xmm), (u64::MAX, true), "x86 evidence wins over the declared type");
}

#[test]
fn dead_return_bits_are_ret_dead_not_equal() {
    let dead = RetUse { rax_mask: 0, xmm: false, src: types::RET_X86 };
    let r = probe_ret("rax_dead", dead, false, &|| (5, 0), &|| (6, 0));
    assert_eq!(r.outcome(), Outcome::RetDead, "{r:?} diag={:?}", diag());
    assert_eq!((r.ret_mask, r.ret_xmm), (0, 0));
    let r = probe_ret("xmm_dead", dead, true, &|| (5, 1), &|| (5, 2));
    assert_eq!(r.outcome(), Outcome::RetDead, "{r:?} diag={:?}", diag());
    let r = probe_ret("all_equal", dead, true, &|| (5, 1), &|| (5, 1));
    assert_eq!(r.outcome(), Outcome::Equal, "{r:?} diag={:?}", diag());
    let al = RetUse { rax_mask: 0xff, xmm: false, src: types::RET_X86 };
    let r = probe_ret("al_live_upper_dead", al, false, &|| (0x1234_0001, 0), &|| (0x9999_0001, 0));
    assert_eq!(r.outcome(), Outcome::RetDead, "{r:?} diag={:?}", diag());
    let r = probe_ret("al_live_differs", al, false, &|| (1, 0), &|| (0, 0));
    assert_eq!(r.outcome(), Outcome::Mismatch, "{r:?} diag={:?}", diag());
    // Dead bits that differ between the two guest runs are not nondeterminism.
    let flip = std::sync::atomic::AtomicU64::new(0);
    let g = || (0x100 + flip.fetch_add(1, std::sync::atomic::Ordering::Relaxed) * 0x100, 0u128);
    let r = probe_ret("dead_noise", al, false, &g, &|| (0, 0));
    assert_ne!(r.outcome(), Outcome::Nondet, "{r:?} diag={:?}", diag());
    // Memory diffs still count.
    let b = leak_buf(64);
    let w = move |v: u8| unsafe { *(b as *mut u8) = v };
    let r = probe_ret("mem_still_counts", dead, false, &|| { w(1); (0, 0) }, &|| { w(2); (0, 0) });
    assert_eq!(r.outcome(), Outcome::Mismatch, "{r:?} diag={:?}", diag());
}

#[test]
fn parking_lot_bucket_table_is_found() {
    let k = leak_buf(8);
    // WakeConditionVariable unparks through parking_lot_core (creates the table).
    unsafe { win32_rt::sync::WakeConditionVariable(k as *mut core::ffi::c_void) };
    memmap::parking_lot_sym();
    let r = unsafe { memmap::parking_lot_buckets() };
    let (lo, hi) = r.expect("parking_lot_core HASHTABLE symbol / table");
    assert!(hi > lo && (hi - lo) % 64 == 0 && lo % 64 == 0, "{lo:#x}..{hi:#x}");
}

#[test]
fn probe_children_cannot_spawn_threads() {
    unsafe extern "win64" fn never_runs(_p: *mut core::ffi::c_void) -> u32 {
        0
    }
    // CREATE_SUSPENDED: if NO_SPAWN were not set, the thread would at least not run.
    let w = || unsafe {
        let start: win32_rt::thread::ThreadProc = never_runs;
        let h = win32_rt::kernel32::threads::CreateThread(
            core::ptr::null_mut(),
            0,
            Some(start),
            core::ptr::null_mut(),
            0x4,
            core::ptr::null_mut(),
        );
        (h as u64 | (win32_rt::thread::NO_SPAWN.load(Ordering::Relaxed) as u64) << 63, 0u128)
    };
    let r = probe("no_spawn", false, &w, &w);
    assert_eq!(r.outcome(), Outcome::Equal, "{r:?} diag={:?}", diag());
    // NULL handle, NO_SPAWN set, in both runs.
    assert_eq!((r.rax[0], r.rax[1]), (1 << 63, 1 << 63), "{r:?}");
    assert!(!win32_rt::thread::NO_SPAWN.load(Ordering::Relaxed), "parent must keep spawning");
}

#[test]
fn equal_heap_writes() {
    let b = leak_buf(256);
    let w = move || unsafe {
        for i in 0..64 {
            *((b + i) as *mut u8) = i as u8 ^ 0x5a;
        }
        (7u64, 0u128)
    };
    let r = probe("equal", false, &w, &w);
    assert_eq!(r.outcome(), Outcome::Equal, "{r:?} diag={:?}", diag());
    assert!(r.pages[0] >= 1 && r.pages[0] == r.pages[1], "{r:?} diag={:?}", diag());
}

#[test]
fn off_by_one_write_is_a_mismatch() {
    let b = leak_buf(256);
    let g = move || unsafe {
        core::ptr::write_bytes(b as *mut u8, 0xab, 16);
        (0u64, 0u128)
    };
    let n = move || unsafe {
        core::ptr::write_bytes(b as *mut u8, 0xab, 17);
        (0u64, 0u128)
    };
    let r = probe("off_by_one", false, &g, &n);
    assert_eq!(r.outcome(), Outcome::Mismatch, "{r:?} diag={:?}", diag());
    assert_eq!((r.first_diff, r.diff_bytes, r.diff_ranges), ((b + 16) as u64, 1, 1), "{r:?} diag={:?}", diag());
}

#[test]
fn return_value_mismatch_and_abi_narrow() {
    let r = probe("rax", false, &|| (5, 0), &|| (6, 0));
    assert_eq!(r.outcome(), Outcome::Mismatch, "{r:?} diag={:?}", diag());
    let r = probe("narrow", false, &|| (1, 0), &|| (0xdead_0001, 0));
    assert_eq!(r.outcome(), Outcome::AbiNarrow, "{r:?} diag={:?}", diag());
    let r = probe("xmm", true, &|| (0, 1), &|| (0, 2));
    assert_eq!(r.outcome(), Outcome::Mismatch, "{r:?} diag={:?}", diag());
    let r = probe("xmm_ignored", false, &|| (0, 1), &|| (0, 2));
    assert_eq!(r.outcome(), Outcome::Equal, "{r:?} diag={:?}", diag());
}

#[test]
fn guest_nondeterminism_is_nondet() {
    let r = probe("nondet", false, &|| (unsafe { libc::getpid() } as u64, 0), &|| (1, 0));
    assert_eq!(r.outcome(), Outcome::Nondet, "{r:?} diag={:?}", diag());
}

/// Host thread-locals (const-initialized and lazy with Drop) are inside the TLV ranges the
/// comparison ignores.
#[test]
fn tlv_blocks_are_ignored() {
    std::thread_local! {
        static TL_CONST: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
        static TL_LAZY: core::cell::RefCell<Vec<u8>> = core::cell::RefCell::new(Vec::with_capacity(4));
    }
    let a = TL_CONST.with(|c| c as *const _ as usize);
    let b = TL_LAZY.with(|c| c as *const _ as usize);
    let (mut hard, mut soft) = (Vec::new(), Vec::new());
    unsafe { super::memmap::image_ranges(&mut hard, &mut soft) };
    for x in [a, b] {
        assert!(
            soft.iter().any(|&(lo, hi)| x >= lo && x < hi),
            "{x:#x} not in {soft:x?}\n{}",
            unsafe { super::memmap::tlv_debug() }
        );
    }
}

/// Host-internal heap traffic (emulator decode buffers, Rust temporaries) lands in the
/// run's private host arena and is never compared.
#[test]
fn host_allocations_in_runs_are_ignored() {
    let g = || {
        let v: Vec<u64> = (0..10_000).collect();
        let s: u64 = v.iter().sum();
        let leaked = Box::leak(Box::new([1u8; 4096]));
        core::hint::black_box(leaked);
        (s & 1, 0u128)
    };
    let n = || (0u64, 0u128);
    let r = probe("host_alloc", false, &g, &n);
    assert_eq!(r.outcome(), Outcome::Equal, "{r:?} diag={:?}", diag());
}

#[test]
fn frozen_clock_makes_qpc_deterministic() {
    let qpc = || {
        let mut v = 0i64;
        unsafe { win32_rt::kernel32::time::QueryPerformanceCounter(&mut v) };
        std::thread::sleep(std::time::Duration::from_millis(2));
        (v as u64, 0u128)
    };
    let r = probe("qpc", false, &qpc, &qpc);
    assert_eq!(r.outcome(), Outcome::Equal, "{r:?} diag={:?}", diag());
}

#[test]
fn guest_heap_is_deterministic_and_layout_diffs_are_tagged() {
    let out = leak_buf(64);
    let g = move || unsafe {
        let p = win32_rt::heap::alloc(32, false) as usize;
        *(p as *mut u64) = 0x1122;
        *(out as *mut usize) = p;
        (p as u64, 0u128)
    };
    let r = probe("heap_same", false, &g, &g);
    assert_eq!(r.outcome(), Outcome::Equal, "{r:?} diag={:?}", diag());
    let n = move || unsafe {
        let _extra = win32_rt::heap::alloc(16, false);
        let p = win32_rt::heap::alloc(32, false) as usize;
        *(p as *mut u64) = 0x1122;
        *(out as *mut usize) = p;
        (p as u64, 0u128)
    };
    let r = probe("heap_layout", false, &g, &n);
    assert_eq!(r.outcome(), Outcome::AllocLayout, "{r:?} diag={:?}", diag());
}

#[test]
fn timeouts_and_faults() {
    let spin = || loop {
        std::hint::spin_loop();
    };
    let _l = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let mut c = cfg("spin");
    c.timeout_ms = 200;
    let ok = || (0u64, 0u128);
    let r = unsafe { probe_blocking(&c, &ArgRegs::default(), false, &ok, &spin) };
    assert_eq!((r.outcome(), r.reason()), (Outcome::Mismatch, Reason::NativeTimeout), "{r:?} diag={:?}", diag());
    let r = unsafe { probe_blocking(&c, &ArgRegs::default(), false, &spin, &ok) };
    assert_eq!((r.outcome(), r.reason()), (Outcome::Skipped, Reason::GuestTimeout), "{r:?} diag={:?}", diag());
    let r = unsafe { probe_blocking(&c, &ArgRegs::default(), false, &spin, &spin) };
    assert_eq!(r.reason(), Reason::DoubleTimeout, "{r:?} diag={:?}", diag());
    // A fault inside the valid address range is a mismatch; a wild one is inconclusive.
    let guard = unsafe {
        libc::mmap(core::ptr::null_mut(), 16384, libc::PROT_NONE, libc::MAP_ANON | libc::MAP_PRIVATE, -1, 0)
    } as usize;
    let bad = move || unsafe {
        core::ptr::write_volatile(guard as *mut u64, 1);
        (0u64, 0u128)
    };
    let r = unsafe { probe_blocking(&c, &ArgRegs::default(), false, &ok, &bad) };
    assert_eq!((r.outcome(), r.reason()), (Outcome::Mismatch, Reason::NativeFault), "{r:?} diag={:?}", diag());
    assert_eq!(r.fault_addr[1], guard as u64);
    let wild = || unsafe {
        core::ptr::write_volatile(0x8usize as *mut u64, 1);
        (0u64, 0u128)
    };
    let r = unsafe { probe_blocking(&c, &ArgRegs::default(), false, &ok, &wild) };
    assert_eq!((r.outcome(), r.reason()), (Outcome::Skipped, Reason::NativeWildFault), "{r:?} diag={:?}", diag());
}

#[test]
fn shared_mapping_write_is_skipped_and_never_lands() {
    let sh = unsafe {
        libc::mmap(
            core::ptr::null_mut(),
            16384,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_ANON | libc::MAP_SHARED,
            -1,
            0,
        )
    } as usize;
    unsafe { *(sh as *mut u64) = 42 };
    let w = move || unsafe {
        core::ptr::write_volatile(sh as *mut u64, 43);
        (0u64, 0u128)
    };
    let ok = || (0u64, 0u128);
    let r = probe("shared", false, &ok, &w);
    assert_eq!((r.outcome(), r.reason()), (Outcome::Skipped, Reason::SharedWrite), "{r:?} diag={:?}", diag());
    assert_eq!(unsafe { *(sh as *const u64) }, 42);
}

/// g3br: a run reached CoreAudio (Wwise device enumeration) and crashed in libdispatch.
/// Fork-unsafe host APIs end the run with `host_unsafe` naming the API; the game process
/// itself is unaffected.
#[test]
fn host_unsafe_api_in_a_run_is_skipped_and_named() {
    let ok = || (0u64, 0u128);
    let host = || {
        win32_rt::shadow::guard_host("test_coreaudio");
        (1u64, 0u128)
    };
    for (g, n) in [(&host as &dyn Fn() -> (u64, u128), &ok as &dyn Fn() -> (u64, u128)), (&ok, &host)] {
        let r = probe("host_unsafe", false, g, n);
        assert_eq!((r.outcome(), r.reason()), (Outcome::Skipped, Reason::HostUnsafe), "{r:?} diag={:?}", diag());
        assert_eq!(r.host_api_name(), "test_coreaudio");
    }
    assert!(!win32_rt::shadow::in_child(), "the test process is not a shadow child");
    win32_rt::shadow::guard_host("must not exit the parent");
}

/// Aborts, traps and illegal instructions in a run are recorded run faults: they never reach
/// the loader's handler (in a fork copy it hung/recursed on the symbolizer mutex) and never
/// produce a crash report.
#[test]
fn traps_and_aborts_in_runs_are_recorded_faults() {
    let ok = || (0u64, 0u128);
    let abort = || -> (u64, u128) { unsafe { libc::abort() } };
    let r = probe("abort", false, &abort, &ok);
    assert_eq!((r.outcome(), r.reason()), (Outcome::Skipped, Reason::GuestFault), "{r:?} diag={:?}", diag());
    #[cfg(target_arch = "aarch64")]
    {
        let trap = || unsafe {
            core::arch::asm!("brk #0x77");
            (0u64, 0u128)
        };
        let ill = || unsafe {
            core::arch::asm!("udf #0x77");
            (0u64, 0u128)
        };
        for (name, f) in [("brk", &trap as &dyn Fn() -> (u64, u128)), ("udf", &ill)] {
            let r = probe(name, false, &ok, f);
            assert_eq!(r.outcome(), Outcome::Mismatch, "{name}: {r:?} diag={:?}", diag());
            assert!(matches!(r.reason(), Reason::NativeFault | Reason::NativeWildFault), "{name}: {r:?}");
            let r = probe(name, false, f, &ok);
            assert_eq!((r.outcome(), r.reason()), (Outcome::Skipped, Reason::GuestFault), "{name}: {r:?} diag={:?}", diag());
        }
    }
}

/// An orchestrator whose game process is gone stops (and kills its run) instead of
/// carrying on: simulated by recording a parent pid that is not ours.
#[test]
fn orchestrator_exits_when_its_parent_is_gone() {
    let _l = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        unsafe {
            // Child: pretend the game is some other process, then wait on a spinning run.
            super::run::PARENT.store(libc::getppid() + 1_000_000, Ordering::Relaxed);
            let Some(bufs) = super::run::Bufs::new(16) else { libc::_exit(2) };
            let spin = || loop {
                std::hint::spin_loop();
            };
            let _ = super::run::run_one(&bufs, 0, 60_000, &spin);
            // Not reached: run_one kills the run and exits with EXIT_PARENT_GONE.
            libc::_exit(0);
        }
    }
    let mut status = 0i32;
    let t0 = super::run::now_us();
    unsafe { libc::waitpid(pid, &mut status, 0) };
    let dt = super::run::now_us() - t0;
    assert!(libc::WIFEXITED(status), "status {status:#x}");
    assert_eq!(libc::WEXITSTATUS(status), super::run::EXIT_PARENT_GONE);
    assert!(dt < 10_000_000, "took {dt}us");
}

/// Guest x86 `*rdx = *rcx + 1` run by the emulator vs a native replacement: the correct
/// one is equal, an off-by-one one is caught as a mismatch at the output address.
#[test]
fn emulated_original_vs_wrong_replacement() {
    // mov eax,[rcx] ; add eax,1 ; mov [rdx],eax ; mov eax,1 ; ret
    let code: &'static [u8] = Box::leak(Box::new([0x8b, 0x01, 0x83, 0xc0, 0x01, 0x89, 0x02, 0xb8, 1, 0, 0, 0, 0xc3]));
    let input = leak_buf(64);
    let out = leak_buf(64);
    unsafe { *(input as *mut u32) = 41 };
    let regs = ArgRegs { gpr: [input as u64, out as u64, 0, 0, 0, 0, 0, 0], ..Default::default() };
    let g = move || unsafe { crate::emu::emu_run_full(code.as_ptr() as u64, regs.gpr, regs.xmm, regs.stack) };
    let good = move || unsafe {
        *(out as *mut u32) = *(input as *const u32) + 1;
        (1u64, 0u128)
    };
    let wrong = move || unsafe {
        *(out as *mut u32) = *(input as *const u32) + 2;
        (1u64, 0u128)
    };
    let _l = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    let c = cfg("emu_good");
    let r = unsafe { probe_blocking(&c, &regs, false, &g, &good) };
    assert_eq!(r.outcome(), Outcome::Equal, "{r:?} diag={:?}", diag());
    let r = unsafe { probe_blocking(&c, &regs, false, &g, &wrong) };
    assert_eq!(r.outcome(), Outcome::Mismatch, "{r:?} diag={:?}", diag());
    assert_eq!((r.first_diff, r.diff_bytes), (out as u64, 1), "{r:?} diag={:?}", diag());
    assert_eq!(unsafe { *(out as *const u32) }, 0, "probe runs must not touch the caller's memory");
}

/// Native side of [`dll_probe_replays_the_callers_stack_window`]: reads its stack args the way
/// a lifted frame does (`take_incoming_rsp`, arg 9 at `caller_rsp + 0x40`), then
/// `*arg11 + arg20` (arg11 is a pointer: no window = null read, as in cohtml +0x430eb0).
#[cfg(target_arch = "aarch64")]
unsafe extern "win64" fn native_reads_window() -> u64 {
    let (r, l) = crate::emu::take_incoming_rsp();
    let at = |o: u64| if r != 0 && r + o + 8 <= l { core::ptr::read_unaligned((r + o) as *const u64) } else { 0 };
    core::ptr::read_volatile(at(0x50) as *const u64) + at(0x98)
}

/// g5s cohtml +0x430eb0: the DLL shadow hook runs the lifted body via
/// `call_host_dispatched`, so its stack args 9.. come only from the caller's published window
/// (`CallCtx::incoming_rsp`). The orchestrator's warm-up reset it, N read arg11 = 0 and
/// faulted while G (args 9..16 from `ArgRegs::stack`) was fine. Every run must see the
/// caller's window (G reads arg 20 from it too), and the probe must not consume it: the real
/// call still needs it.
#[cfg(target_arch = "aarch64")]
#[test]
fn dll_probe_replays_the_callers_stack_window() {
    // mov rax,[rsp+0x58] (arg11) ; mov rax,[rax] ; add rax,[rsp+0xa0] (arg20) ; ret
    let code: &'static [u8] = Box::leak(Box::new([
        0x48, 0x8B, 0x44, 0x24, 0x58, 0x48, 0x8B, 0x00, 0x48, 0x03, 0x84, 0x24, 0xA0, 0x00, 0x00, 0x00, 0xC3,
    ]));
    let cell = leak_buf(64);
    let w = leak_buf(0x100) as u64;
    unsafe {
        *(cell as *mut u64) = 7;
        *((w + 0x50) as *mut u64) = cell as u64; // arg11
        *((w + 0x98) as *mut u64) = 35; // arg20
    }
    let mut regs = ArgRegs::default();
    regs.stack[2] = cell as u64; // arg11 in the 9..16 image
    let g = move || unsafe { crate::emu::emu_run_full(code.as_ptr() as u64, regs.gpr, regs.xmm, regs.stack) };
    let n = move || unsafe {
        crate::emu::call_host_dispatched(
            0x7000_0000,
            native_reads_window as *const () as usize,
            crate::emu::DF_VALID | crate::emu::DF_HOST,
            regs.gpr,
            regs.xmm,
            regs.stack,
        )
    };
    let _l = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
    crate::emu::set_incoming_rsp(w, w + 0x100);
    let r = unsafe { probe_blocking(&cfg("dll_window"), &regs, false, &g, &n) };
    let left = crate::emu::take_incoming_rsp();
    assert_eq!(left, (w, w + 0x100), "the probe consumed the caller's window");
    assert_eq!(r.outcome(), Outcome::Equal, "{r:?} diag={:?}", diag());
    assert_eq!(r.rax[..3], [42, 42, 42], "{r:?}");
}

#[inline(never)]
unsafe fn store_marker(p: usize, v: u8) {
    core::ptr::write_volatile(p as *mut u8, v);
}

/// Mismatch dumps name each diff page's first writer per run (pc/lr of the faulting store)
/// and the image base, so the code that wrote a diff can be symbolized offline.
#[test]
fn dumps_name_the_first_writer_of_each_diff_page() {
    // A page of its own: a heap buffer shares its page with harness data, whose first
    // write (not `store_marker`'s) would then be the one recorded.
    let b = unsafe {
        libc::mmap(core::ptr::null_mut(), 16384, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_ANON | libc::MAP_PRIVATE, -1, 0)
    };
    assert_ne!(b, libc::MAP_FAILED);
    let b = b as usize;
    let g = move || unsafe {
        store_marker(b, 1);
        (0u64, 0u128)
    };
    let n = move || unsafe {
        store_marker(b, 2);
        (0u64, 0u128)
    };
    let dir = std::env::temp_dir().join(format!("aw2_shadow_fw_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let mut c = cfg("first_writer");
    c.dump_dir = dir.to_string_lossy().into_owned();
    let r = {
        let _l = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        unsafe { probe_blocking(&c, &ArgRegs::default(), false, &g, &n) }
    };
    assert_eq!(r.outcome(), Outcome::Mismatch, "{r:?} diag={:?}", diag());
    let dump = std::fs::read_dir(&dir).unwrap().filter_map(|e| e.ok()).next().expect("dump file").path();
    let text = std::fs::read_to_string(&dump).unwrap();
    let _ = std::fs::remove_dir_all(&dir);
    assert!(text.lines().any(|l| l.starts_with("image=0x")), "{text}");
    let lines: Vec<&str> = text.lines().collect();
    let holds_b = |l: &str| {
        let Some(rest) = l.strip_prefix('[') else { return false };
        let Some((lo, rest)) = rest.split_once(',') else { return false };
        let Some((hi, _)) = rest.split_once(')') else { return false };
        let p = |s: &str| u64::from_str_radix(s.trim_start_matches("0x"), 16).ok();
        matches!((p(lo), p(hi)), (Some(lo), Some(hi)) if lo <= b as u64 && (b as u64) < hi)
    };
    let i = lines.iter().position(|l| holds_b(l)).unwrap_or_else(|| panic!("no diff range holds {b:#x}\n{text}"));
    let line = lines.get(i + 1).copied().filter(|l| l.contains("first write to the page:")).unwrap_or_else(|| panic!("{text}"));
    // Both runs wrote the page through `store_marker`: the faulting pc lies inside it, or
    // (unoptimized builds) inside the outlined `core::ptr::write_volatile::<u8>` it calls.
    let f = store_marker as *const () as usize as u64;
    let field = |key: &str| -> Vec<u64> {
        line.split(key)
            .skip(1)
            .map(|s| u64::from_str_radix(s.split_whitespace().next().unwrap(), 16).unwrap())
            .collect()
    };
    let (pcs, lrs) = (field(" pc=0x"), field(" lr=0x"));
    assert_eq!((pcs.len(), lrs.len()), (2, 2), "{line}");
    let sym = |pc: u64| unsafe {
        let mut i: libc::Dl_info = core::mem::zeroed();
        if libc::dladdr(pc as *const libc::c_void, &mut i) != 0 && !i.dli_sname.is_null() {
            format!("{}+{:#x}", std::ffi::CStr::from_ptr(i.dli_sname).to_string_lossy(), pc - i.dli_saddr as u64)
        } else {
            "?".to_string()
        }
    };
    let inside = |a: u64| (a >= f && a < f + 64) || sym(a).contains("write_volatile");
    assert!(
        pcs.iter().all(|&pc| inside(pc)),
        "{line} store_marker={f:#x} writers={:?}\n{text}",
        pcs.iter().map(|&pc| sym(pc)).collect::<Vec<_>>()
    );
}
