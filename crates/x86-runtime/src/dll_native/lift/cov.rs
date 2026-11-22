//! `AW2_DLL_LIFT_COV=1` (diagnostic): which lifted DLL functions run, and how often.
//!
//! Each installed entry is registered behind a 48-byte aarch64 stub in a `MAP_JIT` page
//! that atomically bumps the entry's counter (`stadd`) and branches to the lifted body. Only
//! x16/x17 (intra-procedure-call scratch) are touched, so arguments, results and the stack
//! pass through unchanged, whichever path dispatches (guest `call`, `call_va8` from lifted
//! code). Every 60 s and at exit, `$AW2_LOG_DIR/dll-lift-cov-<pid>.txt` (else
//! `~/Library/Logs/AW2Port/`) lists per image the entries that ran / installed and every
//! entry's count (`cov<TAB>module<TAB>+rva<TAB>calls`), and the summary is logged.

#![cfg_attr(not(all(target_os = "macos", target_arch = "aarch64")), allow(dead_code))]

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};

const STUB_BYTES: usize = 48;
const CHUNK_STUBS: usize = 4096;

/// Counters and identities of every stub: (module, image base, va, counter).
static ENTRIES: Mutex<Vec<(&'static str, usize, u64, &'static AtomicU64)>> = Mutex::new(Vec::new());
/// Stub chunks `[start, end)` (for [`target_of`]).
static CHUNKS: Mutex<Vec<(usize, usize)>> = Mutex::new(Vec::new());
static CUR: Mutex<(usize, usize)> = Mutex::new((0, 0)); // (next stub, chunk end)

pub fn on() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| std::env::var("AW2_DLL_LIFT_COV").is_ok_and(|v| v.trim() != "0" && !v.trim().is_empty()))
}

/// The lifted body behind coverage stub `p`, or `p` itself.
pub fn target_of(p: usize) -> usize {
    let inside = CHUNKS.lock().is_ok_and(|c| c.iter().any(|&(s, e)| (s..e).contains(&p)));
    if inside { unsafe { core::ptr::read_unaligned((p + 32) as *const usize) } } else { p }
}

/// A counting stub for entry `va` of `module` (mapped at `base`) that jumps to `target`;
/// `target` itself if no stub can be made.
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub unsafe fn stub(module: &'static str, base: usize, va: u64, target: usize) -> usize {
    use crate::emu::exec_mem::{alloc_jit, icache_invalidate, jit_write_protect};
    let Ok(mut cur) = CUR.lock() else { return target };
    if cur.0 + STUB_BYTES > cur.1 {
        let len = CHUNK_STUBS * STUB_BYTES;
        let Some(p) = alloc_jit(len) else {
            return target;
        };
        *cur = (p as usize, p as usize + len);
        if let Ok(mut c) = CHUNKS.lock() {
            c.push(*cur);
        }
    }
    let s = cur.0;
    cur.0 += STUB_BYTES;
    let counter: &'static AtomicU64 = Box::leak(Box::new(AtomicU64::new(0)));
    let words: [u32; 6] = [
        0x5800_00D0, // ldr  x16, #24      (counter address)
        0xD280_0031, // mov  x17, #1
        0xF831_021F, // stadd x17, [x16]
        0x5800_00B0, // ldr  x16, #20 -> +32 (target)
        0xD61F_0200, // br   x16
        0xD503_201F, // nop
    ];
    jit_write_protect(false);
    for (i, w) in words.iter().enumerate() {
        core::ptr::write((s + 4 * i) as *mut u32, *w);
    }
    core::ptr::write((s + 24) as *mut u64, counter as *const AtomicU64 as u64);
    core::ptr::write((s + 32) as *mut u64, target as u64);
    core::ptr::write((s + 40) as *mut u64, 0);
    jit_write_protect(true);
    icache_invalidate(s as *mut u8, STUB_BYTES);
    if let Ok(mut e) = ENTRIES.lock() {
        e.push((module, base, va, counter));
    }
    start_reporter();
    s
}

#[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
pub unsafe fn stub(_module: &'static str, _base: usize, _va: u64, target: usize) -> usize {
    target
}

fn start_reporter() {
    static STARTED: AtomicBool = AtomicBool::new(false);
    if STARTED.swap(true, Ordering::Relaxed) {
        return;
    }
    win32_rt::process::register_exit_hook(report);
    let _ = std::thread::Builder::new().name("aw2-lift-cov".into()).spawn(|| loop {
        std::thread::sleep(std::time::Duration::from_secs(60));
        report();
    });
}

fn report() {
    use std::fmt::Write;
    let Ok(e) = ENTRIES.lock() else { return };
    let mut per: Vec<(&str, usize, usize, u64)> = Vec::new(); // (module, ran, installed, calls)
    let mut rows = String::new();
    for &(m, base, va, c) in e.iter() {
        let n = c.load(Ordering::Relaxed);
        let k = match per.iter().position(|p| p.0 == m) {
            Some(k) => k,
            None => {
                per.push((m, 0, 0, 0));
                per.len() - 1
            }
        };
        per[k].1 += (n != 0) as usize;
        per[k].2 += 1;
        per[k].3 += n;
        let _ = writeln!(rows, "cov\t{m}\t+0x{:x}\t{n}", va as usize - base);
    }
    drop(e);
    let mut o = String::new();
    for (m, ran, inst, calls) in &per {
        let _ = writeln!(o, "# {m}: {ran}/{inst} lifted entries ran, {calls} calls");
        log::info!("dll_lift cov: {m}: {ran}/{inst} lifted entries ran, {calls} calls");
    }
    o.push_str(&rows);
    let dir = std::env::var("AW2_LOG_DIR").ok().filter(|d| !d.is_empty()).unwrap_or_else(|| {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        format!("{home}/Library/Logs/AW2Port")
    });
    let _ = std::fs::create_dir_all(&dir);
    let path = std::path::Path::new(&dir).join(format!("dll-lift-cov-{}.txt", std::process::id()));
    let tmp = path.with_extension("tmp");
    if std::fs::write(&tmp, o.as_bytes()).is_ok() {
        let _ = std::fs::rename(&tmp, &path);
    }
}
