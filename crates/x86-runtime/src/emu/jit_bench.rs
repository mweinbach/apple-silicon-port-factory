//! Offline translator micro-benchmarks and code dumps (ignored tests; diagnostic only).
//!
//! ```text
//! cargo test -p aw2-game --release --lib jit_bench -- --ignored --nocapture --test-threads=1
//! ```
//! - `jit_bench_synthetic`: small guest loops (ALU, array load/store, dependent field loads,
//!   push/pop, call/ret, register- and memory-indirect calls, flag-heavy code, a branchy
//!   loop) timed interpreted and translated with TSO emulation on and off (each config on a
//!   fresh thread, so translations are not shared). Reports ns/guest instruction and, from the
//!   captured translation of each loop's blocks, host instructions per guest instruction on
//!   the main path broken down by class (`jit::test_classify`).
//! - `jit_bench_dll`: translate-only (nothing executed) dump of real code:
//!   `AW2_JIT_BENCH_DLL=/path/cohtml.WindowsDesktop.dll AW2_JIT_BENCH_RVAS=0x4ada00,...`
//!   copies the image to host memory, walks each root's blocks (fall-through, jcc/jmp
//!   targets, return addresses after calls) and translates them; reports the same breakdown,
//!   with the default knobs, the jit6 code generation (`AW2_JIT_FLAGLAZY=0`) and the jit5
//!   one (also `AW2_JIT_XMMREG=0`, `AW2_JIT_RETLIT=0`), and the main-path word savings.
//!
//! Both write `.s` files of `.inst` words (one label per block, main path then out-of-line
//! code) to `AW2_JIT_BENCH_OUT` (default `/tmp/aw2_jit_bench`) for
//! `clang -c -arch arm64 x.s -o x.o && objdump -d x.o`.

use super::jit_tests::arena_alloc;
use super::*;
use iced_x86::code_asm::*;
use std::collections::{BTreeMap, HashSet};
use std::time::Instant;

const ITER: u32 = 2_000_000;
const DATA_BYTES: usize = 0x2_0000;

fn out_dir() -> std::path::PathBuf {
    let d = std::env::var_os("AW2_JIT_BENCH_OUT").map_or_else(|| "/tmp/aw2_jit_bench".into(), std::path::PathBuf::from);
    let _ = std::fs::create_dir_all(&d);
    d
}

/// One synthetic workload: guest code taking the data pointer in rcx, `per_iter` guest
/// instructions per loop iteration on average (ITER iterations), `data` bytes of data set up
/// by `init` before every run (`None`: zeroed, at most `DATA_BYTES`).
struct Work {
    name: &'static str,
    per_iter: f64,
    build: fn(&mut CodeAssembler) -> Result<(), IcedError>,
    data: usize,
    init: Option<unsafe fn(*mut u8, usize)>,
}

fn w_alu(a: &mut CodeAssembler) -> Result<(), IcedError> {
    let mut top = a.create_label();
    a.mov(r9d, ITER)?;
    a.xor(eax, eax)?;
    a.mov(edx, 1)?;
    a.set_label(&mut top)?;
    a.add(rax, rdx)?;
    a.xor(rdx, rax)?;
    a.shl(rax, 3)?;
    a.lea(rdx, ptr(rdx + rax * 2 + 7))?;
    a.sub(r9d, 1)?;
    a.jnz(top)?;
    a.ret()
}

fn w_mem(a: &mut CodeAssembler) -> Result<(), IcedError> {
    let mut top = a.create_label();
    a.mov(rsi, rcx)?;
    a.lea(rdi, ptr(rcx + 0x8000))?;
    a.xor(edx, edx)?;
    a.mov(r9d, ITER)?;
    a.set_label(&mut top)?;
    a.mov(rax, qword_ptr(rsi + rdx * 8))?;
    a.add(rax, r9)?;
    a.mov(qword_ptr(rdi + rdx * 8), rax)?;
    a.add(edx, 1)?;
    a.and(edx, 1023)?;
    a.sub(r9d, 1)?;
    a.jnz(top)?;
    a.ret()
}

fn w_field(a: &mut CodeAssembler) -> Result<(), IcedError> {
    // [rcx+0x10] = rcx (self pointer, set up here).
    let mut top = a.create_label();
    a.mov(qword_ptr(rcx + 0x10), rcx)?;
    a.mov(r9d, ITER)?;
    a.set_label(&mut top)?;
    a.mov(rax, qword_ptr(rcx + 0x10))?;
    a.mov(edx, dword_ptr(rax + 0x28))?;
    a.add(edx, r9d)?;
    a.mov(dword_ptr(rax + 0x28), edx)?;
    a.cmp(dword_ptr(rax + 0x30), 0)?;
    a.sub(r9d, 1)?;
    a.jnz(top)?;
    a.mov(eax, edx)?;
    a.ret()
}

fn w_stack(a: &mut CodeAssembler) -> Result<(), IcedError> {
    let mut top = a.create_label();
    a.mov(r9d, ITER)?;
    a.set_label(&mut top)?;
    a.push(rbx)?;
    a.push(rsi)?;
    a.mov(qword_ptr(rsp + 8), rax)?;
    a.mov(rax, qword_ptr(rsp))?;
    a.pop(rsi)?;
    a.pop(rbx)?;
    a.sub(r9d, 1)?;
    a.jnz(top)?;
    a.ret()
}

fn w_call(a: &mut CodeAssembler) -> Result<(), IcedError> {
    let mut top = a.create_label();
    let mut leaf = a.create_label();
    a.xor(eax, eax)?;
    a.mov(r9d, ITER)?;
    a.set_label(&mut top)?;
    a.call(leaf)?;
    a.sub(r9d, 1)?;
    a.jnz(top)?;
    a.ret()?;
    a.set_label(&mut leaf)?;
    a.lea(rax, ptr(rax + 1))?;
    a.ret()
}

fn w_icall(a: &mut CodeAssembler) -> Result<(), IcedError> {
    let mut top = a.create_label();
    let mut leaf = a.create_label();
    a.xor(eax, eax)?;
    a.lea(r11, ptr(leaf))?;
    a.mov(r9d, ITER)?;
    a.set_label(&mut top)?;
    a.call(r11)?;
    a.sub(r9d, 1)?;
    a.jnz(top)?;
    a.ret()?;
    a.set_label(&mut leaf)?;
    a.lea(rax, ptr(rax + 1))?;
    a.ret()
}

fn w_vcall(a: &mut CodeAssembler) -> Result<(), IcedError> {
    // Object at rcx: [rcx] = vtable (rcx+0x100), vtable[1] = leaf.
    let mut top = a.create_label();
    let mut leaf = a.create_label();
    a.lea(rax, ptr(leaf))?;
    a.mov(qword_ptr(rcx + 0x108), rax)?;
    a.lea(rax, ptr(rcx + 0x100))?;
    a.mov(qword_ptr(rcx), rax)?;
    a.mov(rbx, rcx)?;
    a.xor(esi, esi)?;
    a.mov(r9d, ITER)?;
    a.set_label(&mut top)?;
    a.mov(rcx, rbx)?;
    a.mov(rax, qword_ptr(rcx))?;
    a.call(qword_ptr(rax + 8))?;
    a.sub(r9d, 1)?;
    a.jnz(top)?;
    a.mov(rax, rsi)?;
    a.ret()?;
    a.set_label(&mut leaf)?;
    a.add(rsi, 1)?;
    a.ret()
}

fn w_flags(a: &mut CodeAssembler) -> Result<(), IcedError> {
    let mut top = a.create_label();
    a.xor(eax, eax)?;
    a.mov(edx, 3)?;
    a.xor(ebx, ebx)?;
    a.mov(r9d, ITER)?;
    a.set_label(&mut top)?;
    a.cmp(rax, rdx)?;
    a.setl(bl)?;
    a.adc(rax, 0)?;
    a.test(al, 1)?;
    a.cmovz(rdx, rax)?;
    a.add(rdx, rbx)?;
    a.sub(r9d, 1)?;
    a.jnz(top)?;
    a.ret()
}

fn w_branchy(a: &mut CodeAssembler) -> Result<(), IcedError> {
    let mut top = a.create_label();
    let mut skip = a.create_label();
    a.xor(eax, eax)?;
    a.mov(r9d, ITER)?;
    a.set_label(&mut top)?;
    a.test(r9d, 1)?;
    a.jz(skip)?;
    a.add(rax, 3)?;
    a.set_label(&mut skip)?;
    a.sub(r9d, 1)?;
    a.jnz(top)?;
    a.ret()
}

/// One leaf called from 16 call sites per iteration: the return target changes on every
/// `ret` (one shared `lookup_rip` indirect branch without the RAS; host `ret` with it).
fn w_callsites(a: &mut CodeAssembler) -> Result<(), IcedError> {
    let mut top = a.create_label();
    let mut leaf = a.create_label();
    a.xor(eax, eax)?;
    a.mov(r9d, ITER)?;
    a.set_label(&mut top)?;
    for _ in 0..16 {
        a.call(leaf)?;
    }
    a.sub(r9d, 1)?;
    a.jnz(top)?;
    a.ret()?;
    a.set_label(&mut leaf)?;
    a.lea(rax, ptr(rax + 1))?;
    a.ret()
}

/// `call [rcx + idx*8]` through a 32-entry table of distinct leaves, pseudo-random index
/// (polymorphic call target; single return site).
fn w_poly(a: &mut CodeAssembler) -> Result<(), IcedError> {
    let mut top = a.create_label();
    let mut leaves: Vec<CodeLabel> = (0..32).map(|_| a.create_label()).collect();
    for (k, &l) in leaves.iter().enumerate() {
        a.lea(rax, ptr(l))?;
        a.mov(qword_ptr(rcx + 8 * k as i32), rax)?;
    }
    a.xor(esi, esi)?;
    a.mov(r9d, ITER)?;
    a.set_label(&mut top)?;
    a.imul_3(eax, r9d, 0x9E37_79B1u32 as i32)?;
    a.shr(eax, 27)?;
    a.call(qword_ptr(rcx + rax * 8))?;
    a.sub(r9d, 1)?;
    a.jnz(top)?;
    a.mov(rax, rsi)?;
    a.ret()?;
    for (k, l) in leaves.iter_mut().enumerate() {
        a.set_label(l)?;
        a.add(rsi, k as i32 + 1)?;
        a.ret()?;
    }
    Ok(())
}

/// Dependent loads through a randomly linked list far larger than the caches.
fn w_chase(a: &mut CodeAssembler) -> Result<(), IcedError> {
    let mut top = a.create_label();
    a.mov(rax, rcx)?;
    a.mov(r9d, ITER)?;
    a.set_label(&mut top)?;
    a.mov(rax, qword_ptr(rax))?;
    a.sub(r9d, 1)?;
    a.jnz(top)?;
    a.ret()
}

const CHASE_BYTES: usize = 64 << 20;

/// Single random cycle through all 64-byte nodes of `d` (Sattolo): node -> [node] = next.
unsafe fn init_chase(d: *mut u8, len: usize) {
    let n = len / 64;
    let mut perm: Vec<u32> = (0..n as u32).collect();
    let mut s = 0x9E37_79B9_7F4A_7C15u64;
    for i in (1..n).rev() {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        let j = (s % i as u64) as usize;
        perm.swap(i, j);
    }
    for i in 0..n {
        let next = perm[i] as usize;
        *(d.add(i * 64) as *mut u64) = d as u64 + (next * 64) as u64;
    }
}

/// Scalar SSE chain: load, mul, add, move, max, sub, convert, f64 add, store (values stay
/// finite and normal).
fn w_sse_scalar(a: &mut CodeAssembler) -> Result<(), IcedError> {
    let mut top = a.create_label();
    a.mov(dword_ptr(rcx + 0x10), 0x3F80_0000u32)?;
    a.mov(eax, 0x3F00_0000u32)?;
    a.movd(xmm1, eax)?;
    a.xorps(xmm0, xmm0)?;
    a.xorps(xmm4, xmm4)?;
    a.xorps(xmm6, xmm6)?;
    a.mov(r9d, ITER)?;
    a.set_label(&mut top)?;
    a.movss(xmm2, dword_ptr(rcx + 0x10))?;
    a.mulss(xmm2, xmm1)?;
    a.addss(xmm0, xmm2)?;
    a.movaps(xmm3, xmm0)?;
    a.maxss(xmm3, xmm4)?;
    a.subss(xmm0, xmm3)?;
    a.cvtss2sd(xmm5, xmm0)?;
    a.addsd(xmm6, xmm5)?;
    a.movss(dword_ptr(rcx + 0x20), xmm0)?;
    a.sub(r9d, 1)?;
    a.jnz(top)?;
    a.movq(rax, xmm6)?;
    a.ret()
}

/// Packed SSE: load, mulps, addps, minps, store.
fn w_sse_packed(a: &mut CodeAssembler) -> Result<(), IcedError> {
    let mut top = a.create_label();
    a.mov(rax, 0x3F80_0000_3F00_0000u64)?;
    a.mov(qword_ptr(rcx), rax)?;
    a.mov(qword_ptr(rcx + 8), rax)?;
    a.movups(xmm1, xmmword_ptr(rcx))?;
    a.xorps(xmm0, xmm0)?;
    a.mov(r9d, ITER)?;
    a.set_label(&mut top)?;
    a.movups(xmm2, xmmword_ptr(rcx))?;
    a.mulps(xmm2, xmm1)?;
    a.addps(xmm0, xmm2)?;
    a.minps(xmm0, xmm1)?;
    a.movups(xmmword_ptr(rcx + 0x40), xmm0)?;
    a.sub(r9d, 1)?;
    a.jnz(top)?;
    a.ret()
}

/// Scalar SSE across a guest call (cached registers stored before every transfer).
fn w_sse_call(a: &mut CodeAssembler) -> Result<(), IcedError> {
    let mut top = a.create_label();
    let mut leaf = a.create_label();
    a.mov(eax, 0x3F80_0000u32)?;
    a.movd(xmm2, eax)?;
    a.mov(eax, 0x3E80_0000u32)?;
    a.movd(xmm1, eax)?;
    a.xorps(xmm0, xmm0)?;
    a.mov(r9d, ITER)?;
    a.set_label(&mut top)?;
    a.addss(xmm0, xmm1)?;
    a.call(leaf)?;
    a.sub(r9d, 1)?;
    a.jnz(top)?;
    a.ret()?;
    a.set_label(&mut leaf)?;
    a.mulss(xmm0, xmm2)?;
    a.ret()
}

const WORKS: &[Work] = &[
    Work { name: "alu", per_iter: 6.0, build: w_alu, data: DATA_BYTES, init: None },
    Work { name: "mem array ld/st", per_iter: 7.0, build: w_mem, data: DATA_BYTES, init: None },
    Work { name: "field ld/st", per_iter: 7.0, build: w_field, data: DATA_BYTES, init: None },
    Work { name: "push/pop+stack", per_iter: 8.0, build: w_stack, data: DATA_BYTES, init: None },
    Work { name: "call/ret", per_iter: 5.0, build: w_call, data: DATA_BYTES, init: None },
    Work { name: "call reg", per_iter: 5.0, build: w_icall, data: DATA_BYTES, init: None },
    Work { name: "call [vtbl]", per_iter: 7.0, build: w_vcall, data: DATA_BYTES, init: None },
    Work { name: "flags", per_iter: 8.0, build: w_flags, data: DATA_BYTES, init: None },
    // test, jz, (add on odd iterations), sub, jnz: 4.5 average.
    Work { name: "branchy", per_iter: 4.5, build: w_branchy, data: DATA_BYTES, init: None },
    // 16 x (call, lea, ret) + sub, jnz.
    Work { name: "call 16 sites", per_iter: 50.0, build: w_callsites, data: DATA_BYTES, init: None },
    // imul, shr, call, add, ret, sub, jnz.
    Work { name: "call 32 targets", per_iter: 7.0, build: w_poly, data: DATA_BYTES, init: None },
    Work { name: "pointer chase", per_iter: 3.0, build: w_chase, data: CHASE_BYTES, init: Some(init_chase) },
    Work { name: "sse scalar", per_iter: 11.0, build: w_sse_scalar, data: DATA_BYTES, init: None },
    Work { name: "sse packed", per_iter: 7.0, build: w_sse_packed, data: DATA_BYTES, init: None },
    // addss, call, mulss, ret, sub, jnz.
    Work { name: "sse call", per_iter: 6.0, build: w_sse_call, data: DATA_BYTES, init: None },
];

/// Leaked, 64 KiB-aligned host memory outside the AW2 exe window (the arena is too small for
/// the pointer-chase list).
fn big_alloc(len: usize) -> *mut u8 {
    loop {
        let m: &'static mut [u8] = Box::leak(vec![0u8; len + 0x1_0000].into_boxed_slice());
        let p = (m.as_mut_ptr() as usize + 0xffff) & !0xffff;
        if !(p as u64 + len as u64 > 0x1_4000_0000 && (p as u64) < 0x1_4600_0000) {
            return p as *mut u8;
        }
    }
}

/// Class histogram of the main-path words of `blocks`.
fn histogram(blocks: &[jit::DumpBlock]) -> (usize, usize, BTreeMap<&'static str, usize>) {
    let mut h = BTreeMap::new();
    let (mut guest, mut host) = (0, 0);
    for b in blocks {
        guest += b.guest;
        host += b.main;
        for &w in &b.words[..b.main] {
            *h.entry(jit::test_classify(w)).or_insert(0) += 1;
        }
    }
    (guest, host, h)
}

fn fmt_hist(h: &BTreeMap<&'static str, usize>, total: usize) -> String {
    let mut v: Vec<_> = h.iter().collect();
    v.sort_by_key(|(_, &n)| core::cmp::Reverse(n));
    v.iter().map(|(k, &n)| format!("{k} {n} ({:.0}%)", n as f64 * 100.0 / total.max(1) as f64)).collect::<Vec<_>>().join(", ")
}

/// `.s` listing of `blocks` (`.inst` words; `// --- out-of-line` before the out-of-line part).
fn write_asm(name: &str, blocks: &[jit::DumpBlock]) {
    let mut s = String::from(".text\n");
    for b in blocks {
        s += &format!("// guest {:#x}: {} instrs, {} main + {} out-of-line words, host {:#x}\n", b.ip, b.guest, b.main, b.words.len() - b.main, b.code);
        s += &format!("blk_{:x}:\n", b.ip);
        for (i, w) in b.words.iter().enumerate() {
            if i == b.main {
                s += "// --- out-of-line\n";
            }
            s += &format!("  .inst 0x{w:08x}\n");
        }
    }
    let p = out_dir().join(format!("{}.s", name.replace(|c: char| !c.is_ascii_alphanumeric(), "_")));
    let _ = std::fs::write(&p, s);
}

/// Run `w` in one config on a fresh thread: (ns per guest instr, instrs in translated code
/// per run, captured blocks).
fn run_config(w: &'static Work, jit_on: bool, tso: bool, knobs: jit::TestKnobs) -> (f64, u64, Vec<jit::DumpBlock>) {
    std::thread::spawn(move || unsafe {
        mem::TEST_NO_TSO.with(|c| c.set(!tso));
        jit::test_set_knobs(knobs);
        let c = arena_alloc(0x1000, 64);
        let d = if w.data > DATA_BYTES {
            static BIG: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
            assert!(w.data <= CHASE_BYTES);
            *BIG.get_or_init(|| big_alloc(CHASE_BYTES) as usize) as *mut u8
        } else {
            arena_alloc(DATA_BYTES, 64)
        };
        let init = |d: *mut u8| match w.init {
            Some(f) => f(d, w.data),
            None => core::ptr::write_bytes(d, 0, w.data),
        };
        let mut a = CodeAssembler::new(64).unwrap();
        (w.build)(&mut a).unwrap();
        let code = a.assemble(c as u64).unwrap();
        assert!(code.len() <= 0x1000);
        core::ptr::copy_nonoverlapping(code.as_ptr(), c, code.len());
        let mode = if jit_on { 1 } else { 2 };
        // Warm-up (translates on first visit), captured.
        jit::test_dump_start();
        jit::force(mode);
        init(d);
        emu_run_full(c as u64, [d as u64, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]);
        let blocks = jit::test_dump_take();
        let mut best = f64::MAX;
        let mut in_jit = 0;
        for _ in 0..3 {
            init(d);
            let j0 = jit::test_executed();
            let t0 = Instant::now();
            emu_run_full(c as u64, [d as u64, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]);
            let ns = t0.elapsed().as_nanos() as f64;
            in_jit = jit::test_executed() - j0;
            best = best.min(ns / (ITER as f64 * w.per_iter));
        }
        jit::force(0);
        (best, in_jit, blocks)
    })
    .join()
    .unwrap()
}

/// Knobs of the default code generation (environment / defaults).
const KNOBS_DEFAULT: jit::TestKnobs = jit::TestKnobs::NONE;
/// Shadow RAS off, everything else default.
const KNOBS_NO_RAS: jit::TestKnobs = jit::TestKnobs { ras: Some(false), ..jit::TestKnobs::NONE };
/// jit6 code generation: lazy flag / last_bits materialisation off.
const KNOBS_JIT6: jit::TestKnobs = jit::TestKnobs { flaglazy: Some(false), ..jit::TestKnobs::NONE };
/// jit5 code generation: XMM register cache and return-address literals off (and jit7's).
const KNOBS_JIT5: jit::TestKnobs = jit::TestKnobs { xmmreg: Some(false), retlit: Some(false), ..KNOBS_JIT6 };
/// jit4 code generation: memory-check / push-pop / back-edge budget changes off (and later ones).
const KNOBS_JIT4: jit::TestKnobs = jit::TestKnobs { memchk: Some(0), pushpop: Some(false), budget_be: Some(false), ..KNOBS_JIT5 };
/// The code generation before the RAS / flags-dead-at-call / read-only plain loads (and
/// everything after them).
const KNOBS_OLD: jit::TestKnobs =
    jit::TestKnobs { ras: Some(false), flags_dead: Some(0), ro_plain: Some(false), ..KNOBS_JIT4 };

#[test]
#[ignore]
fn jit_bench_synthetic() {
    println!("jit bench: {ITER} iterations per workload; ns per guest instruction (best of 3)");
    println!(
        "{:<18} {:>8} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9} {:>9}  {:>6} {:>6} {:>6} {:>6}  breakdown of the loop's translated main path (TSO on)",
        "workload", "interp", "jit tso", "jit plain", "no RAS", "jit6", "jit5", "jit4", "old cg", "h/g", "jit6", "jit5", "jit4"
    );
    for w in WORKS {
        let (ni, _, _) = run_config(w, false, true, KNOBS_DEFAULT);
        let (nt, in_jit, blocks) = run_config(w, true, true, KNOBS_DEFAULT);
        let (np, _, plain_blocks) = run_config(w, true, false, KNOBS_DEFAULT);
        let (nr, _, _) = run_config(w, true, true, KNOBS_NO_RAS);
        let (n6, _, blocks6) = run_config(w, true, true, KNOBS_JIT6);
        let (n5, _, blocks5) = run_config(w, true, true, KNOBS_JIT5);
        let (n4, _, blocks4) = run_config(w, true, true, KNOBS_JIT4);
        let (no, _, _) = run_config(w, true, true, KNOBS_OLD);
        let (g6, h6, _) = histogram(&blocks6);
        let (g5, h5, _) = histogram(&blocks5);
        let (g4, h4, _) = histogram(&blocks4);
        // The loop's blocks: those executed per iteration are all blocks except the entry
        // prologue/epilogue; report every captured block (they are tiny) and the totals.
        let (g, h, hist) = histogram(&blocks);
        let (_, hp, _) = histogram(&plain_blocks);
        let total_guest = (ITER as f64 * w.per_iter) as u64;
        println!(
            "{:<18} {:>8.2} {:>9.2} {:>9.2} {:>9.2} {:>9.2} {:>9.2} {:>9.2} {:>9.2}  {:>6.2} {:>6.2} {:>6.2} {:>6.2}  [{} blocks, {g} guest / {h} host (plain {hp}), {:.0}% in jit] {}",
            w.name,
            ni,
            nt,
            np,
            nr,
            n6,
            n5,
            n4,
            no,
            h as f64 / g.max(1) as f64,
            h6 as f64 / g6.max(1) as f64,
            h5 as f64 / g5.max(1) as f64,
            h4 as f64 / g4.max(1) as f64,
            blocks.len(),
            in_jit as f64 * 100.0 / total_guest as f64,
            fmt_hist(&hist, h)
        );
        write_asm(&format!("syn_{}", w.name), &blocks);
        write_asm(&format!("syn_{}_plain", w.name), &plain_blocks);
        write_asm(&format!("syn_{}_jit6", w.name), &blocks6);
        write_asm(&format!("syn_{}_jit5", w.name), &blocks5);
    }
    println!("asm listings in {}", out_dir().display());
}

fn u16le(b: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([b[o], b[o + 1]])
}
fn u32le(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes(b[o..o + 4].try_into().unwrap())
}

#[test]
#[ignore]
fn jit_bench_dll() {
    let Ok(path) = std::env::var("AW2_JIT_BENCH_DLL") else {
        eprintln!("AW2_JIT_BENCH_DLL not set");
        return;
    };
    let roots: Vec<u32> = std::env::var("AW2_JIT_BENCH_RVAS")
        .expect("AW2_JIT_BENCH_RVAS")
        .split(',')
        .map(|t| u32::from_str_radix(t.trim().trim_start_matches("0x"), 16).expect("hex rva"))
        .collect();
    let tso = std::env::var("AW2_JIT_BENCH_TSO").map_or(true, |s| s.trim() != "0");
    let max_blocks: usize = std::env::var("AW2_JIT_BENCH_MAX").ok().and_then(|s| s.parse().ok()).unwrap_or(400);
    let data = std::fs::read(&path).expect("read dll");
    let nt = u32le(&data, 0x3c) as usize;
    let opt = nt + 24;
    let size_of_image = u32le(&data, opt + 56) as usize;
    let size_of_headers = u32le(&data, opt + 60) as usize;
    let nsec = u16le(&data, nt + 6) as usize;
    let sec0 = opt + u16le(&data, nt + 20) as usize;
    // Image copy in host memory (never in the AW2 exe window: the registry treats those VAs).
    let base = loop {
        let m: &'static mut [u8] = Box::leak(vec![0u8; size_of_image + 0x1_0000].into_boxed_slice());
        let p = (m.as_mut_ptr() as usize + 0xffff) & !0xffff;
        if !(p as u64 + size_of_image as u64 > 0x1_4000_0000 && (p as u64) < 0x1_4600_0000) {
            break p;
        }
    };
    unsafe {
        core::ptr::copy_nonoverlapping(data.as_ptr(), base as *mut u8, size_of_headers.min(data.len()));
        for i in 0..nsec {
            let s = sec0 + i * 40;
            let (va, raw_size, raw_off) = (u32le(&data, s + 12) as usize, u32le(&data, s + 16) as usize, u32le(&data, s + 20) as usize);
            let n = raw_size.min(data.len().saturating_sub(raw_off)).min(size_of_image.saturating_sub(va));
            core::ptr::copy_nonoverlapping(data.as_ptr().add(raw_off), (base + va) as *mut u8, n);
        }
    }
    let base = base as u64;
    let n_roots = roots.len();
    let first_root = roots.first().copied().unwrap_or(0);
    // Same walk on a fresh thread per knob set (translation caches are per thread).
    let walk = |knobs: jit::TestKnobs| -> Vec<jit::DumpBlock> {
        let roots = roots.clone();
        let (base, tso, max_blocks, size_of_image) = (base, tso, max_blocks, size_of_image);
        std::thread::spawn(move || unsafe {
            mem::TEST_NO_TSO.with(|c| c.set(!tso));
            jit::test_set_knobs(knobs);
            // Emulator + translator state for this thread (a `ret` run under the forced translator).
            let r = arena_alloc(16, 16);
            *r = 0xC3;
            jit::force(1);
            emu_run_full(r as u64, [0; 8], [0; 4], [0; 8]);
            jit::force(0);
            jit::test_dump_start();
            let mut seen: HashSet<u64> = HashSet::new();
            let mut work: Vec<u64> = roots.iter().rev().map(|&r| base + r as u64).collect();
            let mut starts: Vec<u64> = Vec::new();
            while let Some(start) = work.pop() {
                if starts.len() >= max_blocks || !seen.insert(start) {
                    continue;
                }
                starts.push(start);
                // Successor block starts: decode to the end of this block.
                let mut ip = start;
                for _ in 0..BLK_MAX_INSNS {
                    let bytes = core::slice::from_raw_parts(ip as *const u8, 16);
                    let mut dec = Decoder::with_ip(64, bytes, ip, DecoderOptions::NONE);
                    let ins = dec.decode();
                    if ins.is_invalid() {
                        break;
                    }
                    let next = ip + ins.len() as u64;
                    let inside = |t: u64| t >= base && t < base + size_of_image as u64;
                    match ins.flow_control() {
                        FlowControl::Return | FlowControl::IndirectBranch | FlowControl::Interrupt | FlowControl::Exception => break,
                        FlowControl::UnconditionalBranch => {
                            let t = ins.near_branch_target();
                            if inside(t) {
                                work.push(t);
                            }
                            break;
                        }
                        FlowControl::ConditionalBranch => {
                            let t = ins.near_branch_target();
                            if inside(t) {
                                work.push(t);
                            }
                            work.push(next);
                            break;
                        }
                        FlowControl::Call | FlowControl::IndirectCall => {
                            work.push(next);
                            break;
                        }
                        _ => {}
                    }
                    ip = next;
                }
                let _ = test_translate(start);
            }
            jit::test_dump_take()
        })
        .join()
        .unwrap()
    };
    let blocks = walk(KNOBS_DEFAULT);
    let blocks6 = walk(KNOBS_JIT6);
    let blocks5 = walk(KNOBS_JIT5);
    let (g, h, hist) = histogram(&blocks);
    println!(
        "jit bench dll {path} (TSO {}): {} roots, {} blocks translated, {g} guest instrs -> {h} host main-path words ({:.2} host/guest)",
        if tso { "on" } else { "off" },
        n_roots,
        blocks.len(),
        h as f64 / g.max(1) as f64
    );
    println!("  breakdown: {}", fmt_hist(&hist, h));
    let total = |bs: &[jit::DumpBlock]| bs.iter().map(|b| b.words.len()).sum::<usize>();
    let (g6, h6, hist6) = histogram(&blocks6);
    println!(
        "  jit6 codegen (FLAGLAZY=0): {} blocks, {g6} guest instrs -> {h6} main-path words ({:.2} host/guest); main path {:+} words ({:+.1}%), all code {} -> {} words",
        blocks6.len(),
        h6 as f64 / g6.max(1) as f64,
        h as i64 - h6 as i64,
        (h as f64 - h6 as f64) * 100.0 / h6.max(1) as f64,
        total(&blocks6[..]),
        total(&blocks[..])
    );
    println!("  jit6 breakdown: {}", fmt_hist(&hist6, h6));
    let (g5, h5, _) = histogram(&blocks5);
    println!(
        "  jit5 codegen (XMMREG=0 RETLIT=0 FLAGLAZY=0): {} blocks, {g5} guest instrs -> {h5} main-path words ({:.2} host/guest); main path {:+} words ({:+.1}%), all code {} -> {} words",
        blocks5.len(),
        h5 as f64 / g5.max(1) as f64,
        h as i64 - h5 as i64,
        (h as f64 - h5 as f64) * 100.0 / h5.max(1) as f64,
        total(&blocks5[..]),
        total(&blocks[..])
    );
    let old: BTreeMap<u64, usize> = blocks6.iter().map(|b| (b.ip, b.main)).collect();
    let mut d: Vec<(i64, &jit::DumpBlock)> =
        blocks.iter().filter_map(|b| old.get(&b.ip).map(|&m| (m as i64 - b.main as i64, b))).collect();
    d.sort_by_key(|&(s, _)| core::cmp::Reverse(s));
    println!("  largest main-path savings vs jit6:");
    for (s, b) in d.iter().take(10) {
        println!("    +{:#x}: {} guest, {} -> {} main words ({s} saved)", b.ip - base, b.guest, b.main as i64 + s, b.main);
    }
    let mut v: Vec<&jit::DumpBlock> = blocks.iter().collect();
    v.sort_by_key(|b| core::cmp::Reverse(b.main * 1000 / b.guest.max(1)));
    println!("  highest host/guest blocks:");
    for b in v.iter().take(15) {
        println!("    +{:#x}: {} guest, {} main ({:.1}/instr), {} out-of-line", b.ip - base, b.guest, b.main, b.main as f64 / b.guest.max(1) as f64, b.words.len() - b.main);
    }
    write_asm(&format!("dll_{:x}{}", first_root, if tso { "" } else { "_plain" }), &blocks);
    write_asm(&format!("dll_{:x}{}_jit6", first_root, if tso { "" } else { "_plain" }), &blocks6);
    write_asm(&format!("dll_{:x}{}_jit5", first_root, if tso { "" } else { "_plain" }), &blocks5);
    println!("asm listing in {}", out_dir().display());
}
