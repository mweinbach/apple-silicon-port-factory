//! `AW2_SHADOW` for lifted DLL entries (exe-zero `class=dll` / `va=`).
//!
//! Lifted entries are dispatched through `register_native_vas` pointers, never through the
//! `DF_LEAN` bridge that probes exe replacements. So [`super::install`] registers every
//! entry the `AW2_SHADOW` spec would select (`va=`, `class=dll`, `shard=dll`, minus `skip=`)
//! with [`crate::shadow::register_dll`] and puts it behind a 48-byte stub in a `MAP_JIT`
//! page. The stub enters [`tramp`], which saves the argument registers (`x0..x8`,
//! `q0..q3`), calls [`enter`] and then branches to the entry's normal target with the
//! registers restored. Arguments, results and the stack pass through unchanged.
//!
//! [`enter`] returns at once unless [`crate::shadow::armed`]; otherwise it hands the call
//! to [`crate::shadow::check_with`]: guest = the original code at the entry VA (the
//! emulator starts at the VA itself, so the entry is not redispatched; its callees still
//! dispatch to their lifts, as in the `dll_lift` difftest), native = the lifted body via
//! `call_host_dispatched`. Both runs happen only in forked children.
//!
//! Without `AW2_SHADOW` (or with a spec selecting no DLL entries) nothing is installed.
//! `register_dll` works at any time, so DLLs loaded late are probed too.

#![cfg_attr(not(all(target_os = "macos", target_arch = "aarch64")), allow(dead_code))]

use std::sync::{Mutex, OnceLock};

/// What the stub's slow path needs (leaked, one per stubbed entry).
#[repr(C)]
pub struct Info {
    /// Entry VA (shadow's key).
    va: u64,
    /// Where the caller's call continues (lifted body, or its `cov` stub). Offset 8: read
    /// by [`tramp`].
    next: usize,
    /// The lifted body (the native side of a probe).
    body: usize,
    /// Result in `xmm0`.
    ret_xmm: bool,
}

const STUB_BYTES: usize = 48;
const CHUNK_STUBS: usize = 1024;

static CHUNKS: Mutex<Vec<(usize, usize)>> = Mutex::new(Vec::new());
static CUR: Mutex<(usize, usize)> = Mutex::new((0, 0));

/// The `AW2_SHADOW` spec's DLL selection (`None`: shadow off or no DLL entries selected).
struct Sel {
    vas: Vec<u64>,
    skip: Vec<u64>,
    all: bool,
}

fn sel() -> Option<&'static Sel> {
    static SEL: OnceLock<Option<Sel>> = OnceLock::new();
    SEL.get_or_init(|| {
        let raw = std::env::var("AW2_SHADOW").ok()?;
        if raw.trim().is_empty() || raw.trim() == "0" {
            return None;
        }
        let sp = crate::shadow::Spec::parse(&raw).ok()?;
        let all = sp.classes & crate::shadow::C_DLL != 0 || sp.shards.iter().any(|s| s == "dll");
        (all || !sp.vas.is_empty()).then_some(Sel { vas: sp.vas, skip: sp.skip, all })
    })
    .as_ref()
}

/// Shadow may select some lifted DLL entry.
pub fn on() -> bool {
    sel().is_some()
}

/// Entry `va` is selected by the spec (mirrors `shadow::choose` for `group = "dll"`).
pub fn wanted(va: u64) -> bool {
    sel().is_some_and(|s| !s.skip.contains(&va) && (s.all || s.vas.contains(&va)))
}

/// The pointer a shadow stub `p` continues to, or `p` itself.
pub fn target_of(p: usize) -> usize {
    let inside = CHUNKS.lock().is_ok_and(|c| c.iter().any(|&(s, e)| (s..e).contains(&p)));
    if inside { unsafe { (*core::ptr::read_unaligned((p + 16) as *const *const Info)).next } } else { p }
}

/// Register `va` (`module+0xrva`) with shadow and return a stub that probes it and then
/// continues to `next`; `next` itself if no stub can be made.
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub unsafe fn stub(module: &str, base: usize, va: u64, next: usize, body: usize, ret_xmm: bool) -> usize {
    use crate::emu::exec_mem::{alloc_jit, icache_invalidate, jit_write_protect};
    let Ok(mut cur) = CUR.lock() else { return next };
    if cur.0 + STUB_BYTES > cur.1 {
        let len = CHUNK_STUBS * STUB_BYTES;
        let Some(p) = alloc_jit(len) else {
            return next;
        };
        *cur = (p as usize, p as usize + len);
        if let Ok(mut c) = CHUNKS.lock() {
            c.push(*cur);
        }
    }
    let s = cur.0;
    cur.0 += STUB_BYTES;
    crate::shadow::register_dll(va, &format!("{module}+0x{:x}", va as usize - base));
    let info: &'static Info = Box::leak(Box::new(Info { va, next, body, ret_xmm }));
    let words: [u32; 4] = [
        0x5800_0090, // ldr x16, #16 -> +16 (info)
        0x5800_00B1, // ldr x17, #20 -> +24 (tramp)
        0xD61F_0220, // br  x17
        0xD503_201F, // nop
    ];
    jit_write_protect(false);
    for (i, w) in words.iter().enumerate() {
        core::ptr::write((s + 4 * i) as *mut u32, *w);
    }
    core::ptr::write((s + 16) as *mut u64, info as *const Info as u64);
    core::ptr::write((s + 24) as *mut u64, tramp as *const () as u64);
    core::ptr::write((s + 32) as *mut u64, next as u64);
    core::ptr::write((s + 40) as *mut u64, 0);
    jit_write_protect(true);
    icache_invalidate(s as *mut u8, STUB_BYTES);
    s
}

#[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
pub unsafe fn stub(_module: &str, _base: usize, _va: u64, next: usize, _body: usize, _ret_xmm: bool) -> usize {
    next
}

/// Stub slow path: `x16` = `&Info`, argument registers as the caller set them, stack
/// arguments 9..16 at `[sp]`. Saves `x0..x8` / `q0..q3`, calls [`enter`], restores them
/// and branches to `Info::next` (so `lr` and `sp` are the caller's again).
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
#[unsafe(naked)]
unsafe extern "C" fn tramp() {
    core::arch::naked_asm!(
        "stp x29, x30, [sp, #-16]!",
        "mov x29, sp",
        "sub sp, sp, #144",
        "stp x0, x1, [sp, #0]",
        "stp x2, x3, [sp, #16]",
        "stp x4, x5, [sp, #32]",
        "stp x6, x7, [sp, #48]",
        "stp x8, x16, [sp, #64]",
        "stp q0, q1, [sp, #80]",
        "stp q2, q3, [sp, #112]",
        "mov x0, x16",
        "mov x1, sp",
        "add x2, x29, #16",
        "bl {enter}",
        "ldp q2, q3, [sp, #112]",
        "ldp q0, q1, [sp, #80]",
        "ldp x8, x16, [sp, #64]",
        "ldp x6, x7, [sp, #48]",
        "ldp x4, x5, [sp, #32]",
        "ldp x2, x3, [sp, #16]",
        "ldp x0, x1, [sp, #0]",
        "add sp, sp, #144",
        "ldp x29, x30, [sp], #16",
        "ldr x16, [x16, #8]",
        "br x16",
        enter = sym enter,
    );
}

#[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
unsafe extern "C" fn tramp() {}

/// Saved register block of [`tramp`]: `x0..x7`, `x8`, `x16`, then `q0..q3`.
#[repr(C)]
struct Saved {
    gpr: [u64; 8],
    _x8_x16: [u64; 2],
    xmm: [u128; 4],
}

/// Probe this call if shadow is armed and the entry's budget allows (see the module docs).
unsafe extern "C" fn enter(info: &'static Info, saved: &Saved, stack: *const [u64; 8]) {
    if !crate::shadow::armed() {
        return;
    }
    let regs = crate::shadow::ArgRegs { gpr: saved.gpr, xmm: saved.xmm, stack: core::ptr::read_unaligned(stack) };
    let (va, body) = (info.va, info.body);
    let guest = || crate::emu::emu_run_full(va, regs.gpr, regs.xmm, regs.stack);
    let native = || {
        crate::emu::call_host_dispatched(va, body, crate::emu::DF_VALID | crate::emu::DF_HOST, regs.gpr, regs.xmm, regs.stack)
    };
    crate::shadow::check_with(crate::shadow::Kind::Dll, va, &regs, info.ret_xmm, &guest, &native);
}
