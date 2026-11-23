//! Function replacement hooks.
//!
//! For every entry of `aw2_game::registry::REPLACEMENTS` the first bytes of
//! the original function are overwritten with `jmp rel32` to a small thunk
//! within ±2 GB, which does an absolute `jmp [rip]` to the Rust function.
//!
//! Safety checks (iced-x86):
//! * the patch must cover whole instructions (the remainder of the last
//!   displaced instruction is filled with `int3`);
//! * none of the displaced instructions may be a `ret`/`jmp`/`int3` padding
//!   before 5 bytes are covered (function too short to patch);
//! * when the exe's `.pdata` has a RUNTIME_FUNCTION for the address it must
//!   start exactly there (catches wrong VAs).
//!
//! **Trampolines** (calling the original from Rust, e.g. for differential
//! tests): the displaced instructions are re-encoded at a new address with
//! iced's `BlockEncoder` (which fixes up RIP-relative operands and relative
//! branches), followed by an absolute jump back to `va + patch_len`. The
//! trampoline address is recorded in [`trampoline_for`]. Limitation: a branch
//! from later in the function back into the displaced bytes would land in
//! `int3` padding; we scan the function for such branches and refuse to hook.
//!
//! **Call counters**: registry hooks (and probes) go through a counting thunk
//! (`lock inc qword [rip+ctr]; jmp [rip]`). Counters are only read by the
//! reporter thread ([`start_reporter`]).
//!
//! **Register-preserving shim**: registry hooks then enter the Rust function
//! through [`preserving_shim`], which keeps rcx/rdx/r8–r11/xmm0–5 intact for
//! the caller (MSVC callers of known leaves rely on untouched volatiles).
//!
//! **Probes** (`AW2_PROBE=0x14...,0x14...` or `AW2_PROBE=@file`, one VA per
//! line, optional name after it): pass-through counted hooks whose target is
//! the original (via its trampoline). Used to find out which functions run
//! during boot before choosing what to decompile. Unhookable VAs are skipped.
//!
//! **Reporter** (enabled by `AW2_PROBE` or `AW2_HOOK_STATS=1`): a host thread
//! polls the counters every 250 ms, logs `first call: <name> @<va>` the first
//! time a hooked function runs and, with `AW2_HOOK_STATS_FILE=<path>`,
//! rewrites `va<TAB>name<TAB>calls` totals (the process ends with `_exit`, so
//! there is no exit-time dump).
//!
//! **Differential tests** ([`run_difftests`], `aw2-loader --diff-test`): for
//! every registry entry with a `diff` function, call it with the original
//! (trampoline) and the Rust replacement; it feeds both identical inputs and
//! compares results/side effects.
//!
//! **Hook-safety lint** ([`hook_lint`]): each entry's `throws` annotation is
//! checked against `db/throws.tsv` (can-throw facts from `tools/fndb.sh sync`).
//! Findings are warnings at hook time and failures under `--diff-test`.
//!
//! **Fence frames** ([`build_shim`] with `fence`): entries with `throws != No`
//! (or every entry with `AW2_FENCE_ALL=1`) enter Rust through a shim that also
//! saves rbx/rbp/rsi/rdi/r12–r15/xmm6–15 and has a registered RUNTIME_FUNCTION.
//! An exception raised below the Rust replacement unwinds the Rust frames like a
//! `longjmp` (win32-rt `unwind::cross_host_frames`), so they must not hold live
//! `Drop` values across calls that can throw.
//!
//! **`exe::try_call`** ([`install_try_thunk`]): a call thunk with `.pdata` whose
//! language handler ([`try_handler`]) claims matching MSVC C++ exceptions for
//! Rust (docs/design/ADVICE_SEH.md §3.3).

use crate::thunks;
use anyhow::{bail, Result};
use iced_x86::{BlockEncoder, BlockEncoderOptions, Decoder, DecoderOptions, FlowControl, Instruction, InstructionBlock};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

const JMP_LEN: usize = 5;
static PREFERRED_EXE_BASE: AtomicU64 = AtomicU64::new(0x1_4000_0000);
/// Bit tag on non-`x86_64` trampoline addresses so direct calls to `orig` in
/// differential tests trap into `signals::handler` and execute the original
/// guest `x86-64` instructions via `emu_run_with_xmm`.
#[cfg_attr(target_arch = "x86_64", allow(dead_code))]
pub const ORIG_TRAMPOLINE_TAG: usize = 1usize << 46;

static TRAMPOLINES: Mutex<Option<HashMap<u64, usize>>> = Mutex::new(None);

/// Counted hooks: (original VA, name, kind, counter).
static COUNTERS: Mutex<Vec<(u64, String, &'static str, &'static AtomicU64)>> = Mutex::new(Vec::new());

/// Address of the trampoline that executes the original function at `va`.
pub fn trampoline_for(va: u64) -> Option<usize> {
    TRAMPOLINES.lock().unwrap().as_ref()?.get(&va).copied()
}

fn record_trampoline(va: u64, tramp: usize) {
    TRAMPOLINES.lock().unwrap().get_or_insert_with(HashMap::new).insert(va, tramp);
}

pub struct Patch {
    #[allow(dead_code)]
    pub at: usize,
    #[allow(dead_code)]
    pub len: usize,
    pub trampoline: usize,
    /// Call counter (counted hooks only).
    pub counter: Option<&'static AtomicU64>,
}

/// Where a hook sends execution.
#[derive(Clone, Copy)]
pub enum Dest {
    /// A fixed address (the Rust replacement).
    Fixed(usize),
    /// The hook's own trampoline, i.e. the original code (probes).
    Trampoline,
}

/// Decode instructions at `addr` until at least `JMP_LEN` bytes are covered.
fn displaced(addr: usize, ip: u64) -> Result<Vec<Instruction>> {
    let bytes = unsafe { std::slice::from_raw_parts(addr as *const u8, 32) };
    let mut d = Decoder::with_ip(64, bytes, ip, DecoderOptions::NONE);
    let mut out = Vec::new();
    let mut len = 0;
    while len < JMP_LEN {
        let ins = d.decode();
        if ins.is_invalid() {
            bail!("invalid instruction at 0x{:x}", ins.ip());
        }
        match ins.flow_control() {
            FlowControl::Return | FlowControl::UnconditionalBranch | FlowControl::IndirectBranch | FlowControl::Interrupt
                if len + ins.len() < JMP_LEN =>
            {
                let end = len + ins.len();
                if matches!(ins.flow_control(), FlowControl::Return | FlowControl::UnconditionalBranch | FlowControl::IndirectBranch)
                    && bytes[end..JMP_LEN].iter().all(|&b| b == 0xCC)
                {
                    out.push(ins);
                    return Ok(out);
                }
                if matches!(ins.flow_control(), FlowControl::Interrupt)
                    && len > 0
                    && bytes[len..JMP_LEN].iter().all(|&b| b == 0xCC)
                {
                    return Ok(out);
                }
                bail!("function too short to patch ({ins} at +{len})")
            }
            _ => {}
        }
        len += ins.len();
        out.push(ins);
    }
    Ok(out)
}

/// Infer the end address of a leaf function without `.pdata` by decoding up to
/// 64 bytes until a return/unconditional-branch that is not jumped over by an
/// earlier forward branch, or `int3` padding.
fn infer_leaf_end(func: usize) -> usize {
    let bytes = unsafe { std::slice::from_raw_parts(func as *const u8, 64) };
    let mut d = Decoder::with_ip(64, bytes, func as u64, DecoderOptions::NONE);
    let mut max_target = func;
    let mut end = func;
    for ins in d.iter() {
        if ins.is_invalid() || matches!(ins.flow_control(), FlowControl::Interrupt) {
            break;
        }
        let next_ip = (ins.ip() as usize) + ins.len();
        end = next_ip;
        if matches!(ins.flow_control(), FlowControl::ConditionalBranch | FlowControl::UnconditionalBranch) {
            let t = ins.near_branch_target() as usize;
            if t > max_target && t <= func + 64 {
                max_target = t;
            }
        }
        if matches!(ins.flow_control(), FlowControl::Return | FlowControl::UnconditionalBranch | FlowControl::IndirectBranch)
            && next_ip >= max_target
        {
            break;
        }
    }
    end
}

/// Refuse if any branch in `[scan_start, func_end)` targets `(func, patch_end)`.
fn check_no_branch_into(func: usize, scan_start: usize, func_end: usize, patch_end: usize) -> Result<()> {
    if scan_start >= func_end {
        return Ok(());
    }
    let n = func_end - scan_start;
    let bytes = unsafe { std::slice::from_raw_parts(scan_start as *const u8, n) };
    let mut d = Decoder::with_ip(64, bytes, scan_start as u64, DecoderOptions::NONE);
    for ins in d.iter() {
        if matches!(ins.flow_control(), FlowControl::ConditionalBranch | FlowControl::UnconditionalBranch) {
            let t = ins.near_branch_target() as usize;
            if t > func && t < patch_end {
                bail!("branch at 0x{:x} targets displaced bytes 0x{t:x}", ins.ip());
            }
        }
    }
    Ok(())
}

/// `lock inc qword [rip+disp32]; jmp [rip]; dq target` (22 bytes) placed at
/// `at`. The counter must live in a separate non-executable page
/// ([`thunks::alloc_near_data`]): under Rosetta a store into a page that
/// holds translated code is handled as self-modifying code (exception +
/// re-translation on every call).
fn counting_thunk(target: usize, at: usize, counter: usize) -> Result<[u8; 22]> {
    let mut b = [0u8; 22];
    b[..4].copy_from_slice(&[0xF0, 0x48, 0xFF, 0x05]);
    let disp = i32::try_from(counter as i64 - (at as i64 + 8)).map_err(|_| anyhow::anyhow!("counter out of rel32 range"))?;
    b[4..8].copy_from_slice(&disp.to_le_bytes());
    b[8..22].copy_from_slice(&thunks::abs_jmp(target));
    Ok(b)
}

/// Stack argument slots (5th argument onwards) the shim forwards.
const SHIM_STACK_ARGS: usize = 12;
const SHIM_FRAME: u32 = 0x118; // 0x20 home + 0x60 args + 0x30 GPRs + 0x60 XMMs + 8 (alignment)
const SHIM_GPR_SAVE: u32 = 0x80;
const SHIM_XMM_SAVE: u32 = 0xB0;
/// Fence variant: the plain frame plus all Win64 nonvolatiles.
const FENCE_FRAME: u32 = 0x1F8; // SHIM_FRAME - 8 + 0x40 GPRs + 0xA0 XMMs + 8 (alignment)
const FENCE_NV_GPR_SAVE: u32 = 0x110;
const FENCE_NV_XMM_SAVE: u32 = 0x150;
/// Win64 nonvolatile GPRs in unwind numbering: rbx, rbp, rsi, rdi, r12-r15.
const NV_GPRS: [u8; 8] = [3, 5, 6, 7, 12, 13, 14, 15];
const _: () = assert!(FENCE_NV_GPR_SAVE == SHIM_FRAME - 8 && FENCE_NV_XMM_SAVE == FENCE_NV_GPR_SAVE + 0x40);
const _: () = assert!(FENCE_FRAME == FENCE_NV_XMM_SAVE + 0xA0 + 8 && FENCE_FRAME % 16 == 8);

/// Generated hook shim: code, the offset just after `call <target>`, and
/// (fence shims only) its UNWIND_INFO.
struct Shim {
    code: Vec<u8>,
    ret_off: usize,
    unwind_info: Option<Vec<u8>>,
}

/// Volatile-register-preserving shim in front of a Rust replacement.
///
/// MSVC's whole-program register allocation lets callers of a *known* leaf
/// keep values in volatile registers the callee does not touch — e.g.
/// 0x142409b50 keeps `r9` live across `call 0x1401df680` (`data()`, which
/// only writes rax/rcx/rdx). A Rust replacement follows the plain Win64 ABI
/// and may clobber them, so every registry hook saves rcx, rdx, r8–r11 and
/// xmm0–xmm5 and restores all but the return registers (rax; xmm0 too when
/// `float_ret`). Stack arguments 5..=12 are copied into the shim's own
/// outgoing area (a callee writing through the address of a stack argument
/// would not be seen by the caller — no current replacement does).
#[cfg(test)]
#[allow(dead_code)]
fn preserving_shim(target: usize, float_ret: bool) -> Vec<u8> {
    build_shim(target, float_ret, false).code
}

/// [`preserving_shim`], optionally as a **fence frame** (docs/design/ADVICE_SEH.md):
/// the prolog also saves rbx, rbp, rsi, rdi, r12–r15 and xmm6–xmm15 with matching
/// unwind codes. When an exception raised below the Rust replacement unwinds to
/// an original frame above it, win32-rt skips the (unwind-info-less) Rust
/// frames along the frame-pointer chain to this shim's return address, and the
/// shim's unwind info restores the caller's nonvolatiles exactly.
fn build_shim(target: usize, float_ret: bool, fence: bool) -> Shim {
    let mut c = Vec::with_capacity(560);
    let rsp_d32 = |c: &mut Vec<u8>, prefix: &[u8], modrm: u8, disp: u32| {
        c.extend_from_slice(prefix);
        c.push(modrm);
        c.push(0x24); // SIB: base = rsp
        c.extend_from_slice(&disp.to_le_bytes());
    };
    let frame = if fence { FENCE_FRAME } else { SHIM_FRAME };
    // Prolog unwind codes as (end offset, op | info << 4, slot) in prolog order.
    let mut codes: Vec<(u8, u8, u16)> = Vec::new();
    // (REX prefix, ModRM reg field) for rcx, rdx, r8, r9, r10, r11.
    const GPRS: [(u8, u8); 6] = [(0x48, 1), (0x48, 2), (0x4C, 0), (0x4C, 1), (0x4C, 2), (0x4C, 3)];
    c.extend_from_slice(&[0x48, 0x81, 0xEC]); // sub rsp, imm32
    c.extend_from_slice(&frame.to_le_bytes());
    codes.push((c.len() as u8, 1, (frame / 8) as u16)); // UWOP_ALLOC_LARGE, info 0
    if fence {
        for (i, &r) in NV_GPRS.iter().enumerate() {
            let off = FENCE_NV_GPR_SAVE + 8 * i as u32;
            rsp_d32(&mut c, &[if r >= 8 { 0x4C } else { 0x48 }, 0x89], 0x84 | (r & 7) << 3, off); // mov [rsp+d], r
            codes.push((c.len() as u8, 4 | r << 4, (off / 8) as u16)); // UWOP_SAVE_NONVOL
        }
        for x in 6..16u8 {
            let off = FENCE_NV_XMM_SAVE + 16 * (x as u32 - 6);
            let pre: &[u8] = if x >= 8 { &[0x44, 0x0F, 0x11] } else { &[0x0F, 0x11] };
            rsp_d32(&mut c, pre, 0x84 | (x & 7) << 3, off); // movups [rsp+d], xmm
            codes.push((c.len() as u8, 8 | x << 4, (off / 16) as u16)); // UWOP_SAVE_XMM128
        }
    }
    let prolog_len = c.len();
    for (i, (rex, reg)) in GPRS.iter().enumerate() {
        rsp_d32(&mut c, &[*rex, 0x89], 0x84 | reg << 3, SHIM_GPR_SAVE + 8 * i as u32); // mov [rsp+d], r
    }
    for x in 0..6u8 {
        rsp_d32(&mut c, &[0x0F, 0x11], 0x84 | x << 3, SHIM_XMM_SAVE + 16 * x as u32); // movups [rsp+d], xmm
    }
    for i in 0..SHIM_STACK_ARGS as u32 {
        rsp_d32(&mut c, &[0x48, 0x8B], 0x84, frame + 0x28 + 8 * i); // mov rax, [caller arg]
        rsp_d32(&mut c, &[0x48, 0x89], 0x84, 0x20 + 8 * i); // mov [out arg], rax
    }
    c.extend_from_slice(&[0x48, 0xB8]); // mov rax, imm64
    c.extend_from_slice(&(target as u64).to_le_bytes());
    c.extend_from_slice(&[0xFF, 0xD0]); // call rax
    let ret_off = c.len();
    if fence {
        for (i, &r) in NV_GPRS.iter().enumerate() {
            let off = FENCE_NV_GPR_SAVE + 8 * i as u32;
            rsp_d32(&mut c, &[if r >= 8 { 0x4C } else { 0x48 }, 0x8B], 0x84 | (r & 7) << 3, off); // mov r, [rsp+d]
        }
        for x in 6..16u8 {
            let off = FENCE_NV_XMM_SAVE + 16 * (x as u32 - 6);
            let pre: &[u8] = if x >= 8 { &[0x44, 0x0F, 0x10] } else { &[0x0F, 0x10] };
            rsp_d32(&mut c, pre, 0x84 | (x & 7) << 3, off); // movups xmm, [rsp+d]
        }
    }
    for (i, (rex, reg)) in GPRS.iter().enumerate() {
        rsp_d32(&mut c, &[*rex, 0x8B], 0x84 | reg << 3, SHIM_GPR_SAVE + 8 * i as u32); // mov r, [rsp+d]
    }
    for x in (if float_ret { 1 } else { 0 })..6u8 {
        rsp_d32(&mut c, &[0x0F, 0x10], 0x84 | x << 3, SHIM_XMM_SAVE + 16 * x as u32); // movups xmm, [rsp+d]
    }
    c.extend_from_slice(&[0x48, 0x81, 0xC4]); // add rsp, imm32
    c.extend_from_slice(&frame.to_le_bytes());
    c.push(0xC3);
    let unwind_info = fence.then(|| {
        assert!(prolog_len < 256);
        let mut slots: Vec<u16> = Vec::new();
        for &(off, op, slot) in codes.iter().rev() {
            slots.push(off as u16 | (op as u16) << 8);
            slots.push(slot);
        }
        let mut ui = vec![1u8, prolog_len as u8, slots.len() as u8, 0]; // version 1, no flags, no frame register
        if slots.len() % 2 == 1 {
            slots.push(0);
        }
        for s in slots {
            ui.extend_from_slice(&s.to_le_bytes());
        }
        ui
    });
    Shim { code: c, ret_off, unwind_info }
}

/// Copy `shim` to executable memory. A fence shim gets its UNWIND_INFO and a
/// one-entry RUNTIME_FUNCTION table in the same allocation, right after the
/// code. They are written once here and never modified, so there is no
/// mutable data on the code page (AGENTS.md §10). The table is registered
/// with win32-rt's `RtlAddFunctionTable` and the return address with
/// `unwind::register_fence`. Returns the shim's entry point.
fn place_shim(shim: &Shim) -> usize {
    let Some(ui) = &shim.unwind_info else {
        return thunks::alloc_far(&shim.code);
    };
    let ui_off = (shim.code.len() + 3) & !3;
    let rf_off = (ui_off + ui.len() + 3) & !3;
    let mut blob = shim.code.clone();
    blob.resize(ui_off, 0xCC);
    blob.extend_from_slice(ui);
    blob.resize(rf_off, 0);
    for v in [0u32, shim.code.len() as u32, ui_off as u32] {
        blob.extend_from_slice(&v.to_le_bytes());
    }
    let base = thunks::alloc_far(&blob);
    unsafe {
        win32_rt::unwind::RtlAddFunctionTable((base + rf_off) as *const pe_image::RuntimeFunction, 1, base as u64);
    }
    win32_rt::unwind::register_fence((base + shim.ret_off) as u64);
    base
}

/// UNWIND_INFO (version 1) from prolog codes `(end offset, op, info, extra slot)` given in
/// prolog order, with an optional exception handler RVA (`UNW_FLAG_EHANDLER`, handler data 0).
fn unwind_info(prolog_len: usize, codes: &[(u8, u8, u8, Option<u16>)], handler_rva: Option<u32>) -> Vec<u8> {
    let mut slots: Vec<u16> = Vec::new();
    for &(end, op, info, extra) in codes.iter().rev() {
        slots.push(end as u16 | ((op | info << 4) as u16) << 8);
        slots.extend(extra);
    }
    let flags = if handler_rva.is_some() { 1u8 } else { 0 };
    let mut ui = vec![1 | flags << 3, prolog_len as u8, slots.len() as u8, 0];
    if slots.len() % 2 == 1 {
        slots.push(0);
    }
    for s in slots {
        ui.extend_from_slice(&s.to_le_bytes());
    }
    if let Some(h) = handler_rva {
        ui.extend_from_slice(&h.to_le_bytes());
        ui.extend_from_slice(&0u32.to_le_bytes());
    }
    ui
}

/// Place generated code that has Win64 unwind info: `[code][jmp [rip] handler][UNWIND_INFO]
/// [RUNTIME_FUNCTION]` in one code allocation (written once; nothing on it is ever modified),
/// registered with win32-rt's `RtlAddFunctionTable`. The language handler is reached through
/// the in-blob jump because handler RVAs are 32-bit. Returns the code's address.
fn place_unwound_code(code: &[u8], prolog_len: usize, codes: &[(u8, u8, u8, Option<u16>)], handler: Option<usize>) -> usize {
    let thunk_off = (code.len() + 15) & !15;
    let ui_off = (thunk_off + 16 + 3) & !3;
    let ui = unwind_info(prolog_len, codes, handler.map(|_| thunk_off as u32));
    let rf_off = (ui_off + ui.len() + 3) & !3;
    let mut blob = code.to_vec();
    blob.resize(thunk_off, 0xCC);
    blob.extend_from_slice(&thunks::abs_jmp(handler.unwrap_or(0)));
    blob.resize(ui_off, 0xCC);
    blob.extend_from_slice(&ui);
    blob.resize(rf_off, 0);
    for v in [0u32, code.len() as u32, ui_off as u32] {
        blob.extend_from_slice(&v.to_le_bytes());
    }
    let base = thunks::alloc_far(&blob);
    unsafe { win32_rt::unwind::RtlAddFunctionTable((base + rf_off) as *const pe_image::RuntimeFunction, 1, base as u64) };
    base
}

// ---------------------------------------------------------------------------------------------
// exe::try_call support (docs/design/ADVICE_SEH.md §3.3): a call thunk with .pdata whose
// language handler claims matching MSVC C++ exceptions and unwinds to the thunk's landing pad.
// ---------------------------------------------------------------------------------------------

const TRY_FRAME: u32 = 0x88;
/// Where the thunk keeps the `TryFrame*` (rsp-relative after the prolog = establisher frame).
const TRY_FRAME_SLOT: u32 = 0x80;
static TRY_LANDING: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// Build and register the try thunk once and hand it to `aw2_game::exe`.
///
/// `thunk(target: rcx, args: rdx = *const [u64; 16], nargs: r8, frame: r9) -> u64`: copies
/// args 5..16 to its outgoing stack area, loads rcx/rdx/r8/r9 (and xmm0-3 with the same bits),
/// calls `target`, stores rax/xmm0 in the frame and returns 0. When [`try_handler`] claims an
/// exception it unwinds to the landing pad, which returns 1.
#[cfg(not(target_arch = "x86_64"))]
unsafe extern "win64" fn native_try_thunk(
    target: usize,
    args: *const u64,
    _nargs: usize,
    frame: *mut x86_runtime::exe::TryFrame,
) -> u64 {
    let a = &*(args as *const [u64; x86_runtime::exe::TRY_MAX_ARGS]);
    let is_orig = (target & ORIG_TRAMPOLINE_TAG) != 0;
    let clean_va = (target & !ORIG_TRAMPOLINE_TAG) as u64;
    let xmm = [a[0] as u128, a[1] as u128, a[2] as u128, a[3] as u128];
    let gpr_args = [a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]];
    let stack_args = [a[8], a[9], a[10], a[11], a[12], a[13], a[14], a[15]];
    let (ret_gpr, ret_xmm) = if is_orig {
        x86_runtime::emu::emu_run_full(clean_va, gpr_args, xmm, stack_args)
    } else if let Some(host_fn) = x86_runtime::emu::resolve_host_callable(clean_va) {
        x86_runtime::emu::call_host_for_va(clean_va, host_fn, gpr_args, xmm, stack_args)
    } else {
        x86_runtime::emu::emu_run_full(clean_va, gpr_args, xmm, stack_args)
    };
    (*frame).rax = ret_gpr;
    (*frame).xmm0 = ret_xmm as u64;
    0
}

pub fn install_try_thunk() -> usize {
    static ONCE: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
    *ONCE.get_or_init(|| {
        #[cfg(not(target_arch = "x86_64"))]
        {
            let _ = (TRY_FRAME, TRY_FRAME_SLOT, &TRY_LANDING, place_unwound_code, try_handler);
            let base = native_try_thunk as *const () as usize;
            x86_runtime::exe::set_try_thunk(base);
            return base;
        }
        #[cfg(target_arch = "x86_64")]
        {
            let mut c: Vec<u8> = vec![0x48, 0x81, 0xEC]; // sub rsp, imm32
            c.extend_from_slice(&TRY_FRAME.to_le_bytes());
            let codes = [(c.len() as u8, 1u8, 0u8, Some((TRY_FRAME / 8) as u16))]; // UWOP_ALLOC_LARGE
            let prolog_len = c.len();
            c.extend_from_slice(&[0x4C, 0x89, 0x8C, 0x24]); // mov [rsp+slot], r9
            c.extend_from_slice(&TRY_FRAME_SLOT.to_le_bytes());
            c.extend_from_slice(&[0x49, 0x89, 0xCA, 0x49, 0x89, 0xD3]); // mov r10, rcx; mov r11, rdx
            for i in 4..x86_runtime::exe::TRY_MAX_ARGS as u8 {
                c.extend_from_slice(&[0x49, 0x8B, 0x43, 8 * i]); // mov rax, [r11+8i]
                c.extend_from_slice(&[0x48, 0x89, 0x44, 0x24, 0x20 + 8 * (i - 4)]); // mov [rsp+0x20+..], rax
            }
            c.extend_from_slice(&[0x49, 0x8B, 0x0B, 0x49, 0x8B, 0x53, 0x08, 0x4D, 0x8B, 0x43, 0x10, 0x4D, 0x8B, 0x4B, 0x18]); // rcx, rdx, r8, r9
            c.extend_from_slice(&[0x66, 0x48, 0x0F, 0x6E, 0xC1, 0x66, 0x48, 0x0F, 0x6E, 0xCA]); // movq xmm0, rcx; movq xmm1, rdx
            c.extend_from_slice(&[0x66, 0x49, 0x0F, 0x6E, 0xD0, 0x66, 0x49, 0x0F, 0x6E, 0xD9]); // movq xmm2, r8; movq xmm3, r9
            c.extend_from_slice(&[0x41, 0xFF, 0xD2]); // call r10
            c.extend_from_slice(&[0x48, 0x8B, 0x8C, 0x24]); // mov rcx, [rsp+slot]
            c.extend_from_slice(&TRY_FRAME_SLOT.to_le_bytes());
            c.extend_from_slice(&[0x48, 0x89, 0x01, 0x66, 0x0F, 0xD6, 0x41, 0x08]); // mov [rcx], rax; movq [rcx+8], xmm0
            c.extend_from_slice(&[0x31, 0xC0]); // xor eax, eax
            let epilog = |c: &mut Vec<u8>| {
                c.extend_from_slice(&[0x48, 0x81, 0xC4]); // add rsp, imm32
                c.extend_from_slice(&TRY_FRAME.to_le_bytes());
                c.push(0xC3);
            };
            epilog(&mut c);
            let landing = c.len(); // rax = 1 (RtlUnwindEx return value)
            epilog(&mut c);
            let base = place_unwound_code(&c, prolog_len, &codes, Some(try_handler as *const () as usize));
            TRY_LANDING.store(base + landing, Ordering::SeqCst);
            x86_runtime::exe::set_try_thunk(base);
            base
        }
    })
}

type RtlUnwindExFn = unsafe extern "win64" fn(u64, u64, *mut win32_rt::unwind::ExceptionRecord, u64, *mut win32_rt::unwind::Context, *mut std::ffi::c_void);

/// Language handler of the try thunk. Claims an MSVC C++ exception (`0xE06D7363`, magic
/// `0x19930520..22`) whose catchable types include a requested name (or any, for an empty
/// list), copies the object into the `TryFrame` and unwinds to the landing pad. Everything else
/// continues the search above the thunk.
unsafe extern "win64" fn try_handler(
    rec: *mut win32_rt::unwind::ExceptionRecord,
    establisher: u64,
    _ctx: *mut win32_rt::unwind::Context,
    _dc: *mut win32_rt::unwind::DispatcherContext,
) -> u32 {
    use win32_rt::unwind as u;
    const CONTINUE_SEARCH: u32 = 1;
    let r = &mut *rec;
    if r.flags & (u::EXCEPTION_UNWINDING | u::EXCEPTION_EXIT_UNWIND) != 0
        || r.code != u::CXX_EXCEPTION
        || r.number_parameters < 4
        || !(0x1993_0520..=0x1993_0522).contains(&r.information[0])
    {
        return CONTINUE_SEARCH;
    }
    let frame = x86_runtime::exe::rd::<*mut x86_runtime::exe::TryFrame>((establisher + TRY_FRAME_SLOT as u64) as usize);
    let Some(exc) = cxx_match(r, &*frame) else { return CONTINUE_SEARCH };
    (*frame).exc = exc;
    let unwind: RtlUnwindExFn = std::mem::transmute(u::RtlUnwindEx as *const ());
    let mut out = u::Context::zeroed();
    unwind(establisher, TRY_LANDING.load(Ordering::SeqCst) as u64, rec, 1, &mut out, std::ptr::null_mut());
    unreachable!("RtlUnwindEx returned");
}

/// Match a C++ exception against a `TryFrame` catch list (see [`try_handler`]). MSVC x64
/// layouts, all RVAs from `ExceptionInformation[3]`: `ThrowInfo { attributes, pmfnUnwind,
/// pForwardCompat, pCatchableTypeArray }`, `CatchableTypeArray { n, rva[n] }`, `CatchableType
/// { properties, pType, PMD { mdisp, pdisp, vdisp }, sizeOrOffset, copyFunction }`,
/// `TypeDescriptor { vftable, spare, name[] }`.
unsafe fn cxx_match(r: &win32_rt::unwind::ExceptionRecord, f: &x86_runtime::exe::TryFrame) -> Option<x86_runtime::exe::CxxException> {
    use x86_runtime::exe::rd;
    let (obj, ti, base) = (r.information[1], r.information[2], r.information[3]);
    if ti == 0 {
        return None; // `throw;` outside a catch block: nothing to match
    }
    let rva = |at: u64| base.wrapping_add(rd::<i32>(at as usize) as i64 as u64);
    let dtor = rd::<i32>(ti as usize + 4);
    let cta = rva(ti + 12);
    let wanted: &[(usize, usize)] = if f.catch_len == 0 { &[] } else { std::slice::from_raw_parts(f.catch, f.catch_len) };
    for i in 0..rd::<i32>(cta as usize).max(0) as u64 {
        let ct = rva(cta + 4 + 4 * i);
        let td = rva(ct + 4);
        let name = std::ffi::CStr::from_ptr((td + 16) as *const std::ffi::c_char).to_bytes();
        let matched = if wanted.is_empty() {
            u32::MAX
        } else {
            match wanted.iter().position(|&(p, l)| std::slice::from_raw_parts(p as *const u8, l) == name) {
                Some(m) => m as u32,
                None => continue,
            }
        };
        let props = rd::<u32>(ct as usize);
        let (mdisp, pdisp) = (rd::<i32>(ct as usize + 8), rd::<i32>(ct as usize + 12));
        let size = rd::<i32>(ct as usize + 20);
        let copy_fn = rd::<i32>(ct as usize + 24);
        let is_ptr = name.starts_with(b".PE") && size == 8;
        let copyable = (props & 1 != 0 || copy_fn == 0) && (0..=32).contains(&size) && (is_ptr || dtor == 0) && (!is_ptr || pdisp < 0);
        let mut exc = x86_runtime::exe::CxxException { throw_info: ti, image_base: base, object: obj, matched, size: 0, value: [0; 32] };
        if copyable {
            std::ptr::copy_nonoverlapping(obj as *const u8, exc.value.as_mut_ptr(), size as usize);
            exc.size = size as u32;
            if is_ptr {
                let p = exc.ptr() as u64;
                if p != 0 {
                    exc.value[..8].copy_from_slice(&p.wrapping_add(mdisp as i64 as u64).to_le_bytes());
                }
            }
        } else if wanted.is_empty() {
            // catch (...) of an object we can't copy: destroy it as MSVC does at the end of the
            // catch block (no value, no rethrow).
            if dtor != 0 {
                let d: unsafe extern "win64" fn(u64) = std::mem::transmute(base.wrapping_add(dtor as i64 as u64) as usize);
                d(obj);
            }
        } else {
            log::warn!(
                "exe::try_call: not catching {:?} (not by-pointer or trivially copyable: props 0x{props:x}, size {size}, copy fn 0x{copy_fn:x}, dtor 0x{dtor:x})",
                String::from_utf8_lossy(name)
            );
            return None;
        }
        return Some(exc);
    }
    None
}

/// Hook `addr` (actual address) so it jumps to `target`. `func_end` (from
/// .pdata) enables the branch-into check.
/// `restore_prot` is the host protection to put back on the patched pages.
pub fn install(addr: usize, func_end: Option<usize>, target: usize, restore_prot: i32) -> Result<Patch> {
    install_ex(addr, func_end, Dest::Fixed(target), false, restore_prot)
}

/// [`install`] with a choice of destination and an optional call counter.
pub fn install_ex(addr: usize, func_end: Option<usize>, dest: Dest, count: bool, restore_prot: i32) -> Result<Patch> {
    let mut ins = displaced(addr, addr as u64)?;
    let entry_len: usize = ins.iter().map(|i| i.len()).sum();
    let mut tramp_len = entry_len;
    let scan_end = func_end.unwrap_or_else(|| infer_leaf_end(addr));
    if scan_end > addr + tramp_len {
        let max_extend = scan_end.min(addr + 48);
        let rem_bytes = unsafe { std::slice::from_raw_parts((addr + tramp_len) as *const u8, max_extend - (addr + tramp_len)) };
        let mut d = Decoder::with_ip(64, rem_bytes, (addr + tramp_len) as u64, DecoderOptions::NONE);
        let mut extra = Vec::new();
        for inst in d.iter() {
            if inst.is_invalid() || matches!(inst.flow_control(), FlowControl::Interrupt) {
                break;
            }
            extra.push(inst);
            if matches!(inst.flow_control(), FlowControl::ConditionalBranch | FlowControl::UnconditionalBranch) {
                let t = inst.near_branch_target() as usize;
                if t > addr && t < addr + entry_len {
                    ins.extend(extra.drain(..));
                    tramp_len = ins.iter().map(|i| i.len()).sum();
                }
            }
        }
    }
    if scan_end > addr + tramp_len {
        check_no_branch_into(addr, addr + tramp_len, scan_end, addr + entry_len)?;
    }
    // Trampoline: re-encoded displaced instructions + jmp back.
    let tramp = thunks::alloc_near(addr, 64 + 14).ok_or_else(|| anyhow::anyhow!("no memory near 0x{addr:x}"))?;
    let block = InstructionBlock::new(&ins, tramp as u64);
    let enc = BlockEncoder::encode(64, block, BlockEncoderOptions::NONE).map_err(|e| anyhow::anyhow!("re-encode: {e}"))?;
    if enc.code_buffer.len() > 64 {
        bail!("trampoline too large");
    }
    let mut tcode = enc.code_buffer.clone();
    tcode.extend_from_slice(&thunks::abs_jmp(addr + tramp_len));
    unsafe { thunks::write_code(tramp, &tcode) };
    let target = match dest {
        Dest::Fixed(t) => t,
        Dest::Trampoline => tramp,
    };
    // Near thunk to the destination (optionally counting calls).
    let (near, counter) = if count {
        let near = thunks::alloc_near(addr, 22).ok_or_else(|| anyhow::anyhow!("no memory near 0x{addr:x}"))?;
        let ctr = thunks::alloc_near_data(near, 8).ok_or_else(|| anyhow::anyhow!("no data memory near 0x{near:x}"))?;
        unsafe { thunks::write_code(near, &counting_thunk(target, near, ctr)?) };
        // SAFETY: 16-byte aligned, zeroed, RW (non-exec), never freed.
        (near, Some(unsafe { &*(ctr as *const AtomicU64) }))
    } else {
        let near = thunks::alloc_near(addr, 14).ok_or_else(|| anyhow::anyhow!("no memory near 0x{addr:x}"))?;
        unsafe { thunks::write_code(near, &thunks::abs_jmp(target)) };
        (near, None)
    };
    let rel = near as i64 - (addr as i64 + JMP_LEN as i64);
    if rel < i32::MIN as i64 || rel > i32::MAX as i64 {
        bail!("thunk out of rel32 range");
    }
    let patch_len = entry_len.max(JMP_LEN);
    let mut patch = vec![0xCCu8; patch_len];
    patch[0] = 0xE9;
    patch[1..5].copy_from_slice(&(rel as i32).to_le_bytes());
    unsafe {
        #[cfg(target_arch = "x86_64")]
        {
            let page = addr & !0xFFF;
            let span = ((addr + patch_len + 0xFFF) & !0xFFF) - page;
            let rwx = libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC;
            if restore_prot != rwx && libc::mprotect(page as *mut libc::c_void, span, rwx) != 0 {
                bail!("mprotect failed at 0x{page:x}");
            }
            std::ptr::copy_nonoverlapping(patch.as_ptr(), addr as *mut u8, patch_len);
            if restore_prot != rwx {
                libc::mprotect(page as *mut libc::c_void, span, restore_prot);
            }
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            let _ = restore_prot;
            std::ptr::copy_nonoverlapping(patch.as_ptr(), addr as *mut u8, patch_len);
        }
    }
    Ok(Patch { at: addr, len: patch_len, trampoline: tramp, counter })
}

fn va_to_addr_with(exe_base: usize, preferred_base: u64, va: u64) -> usize {
    (exe_base as i64 + (va as i64 - preferred_base as i64)) as usize
}

fn va_to_addr(exe_base: usize, va: u64) -> usize {
    va_to_addr_with(exe_base, PREFERRED_EXE_BASE.load(Ordering::Relaxed), va)
}

/// `.pdata` lookup for an actual address: `Some((begin, end))` of the
/// containing RUNTIME_FUNCTION.
type FnLookup<'a> = &'a dyn Fn(usize) -> Option<(usize, usize)>;

fn loader_lookup(addr: usize) -> Option<(usize, usize)> {
    let l = crate::loader::get();
    let (base, f) = win32_rt::LoaderOps::lookup_function_entry(l, addr as u64)?;
    let f = unsafe { &*f };
    Some((base as usize + f.begin_rva as usize, base as usize + f.end_rva as usize))
}

/// Hook `va` (preferred-base VA) with checks against `.pdata`.
fn hook_va(exe_base: usize, va: u64, name: &str, dest: Dest, count: bool, lookup: FnLookup, restore_prot: i32) -> Result<Patch> {
    let addr = va_to_addr(exe_base, va);
    let func_end = match lookup(addr) {
        Some((begin, end)) => {
            if begin != addr {
                bail!("{name} (0x{va:x}) is not the start of a .pdata function");
            }
            Some(end)
        }
        None => None,
    };
    install_ex(addr, func_end, dest, count, restore_prot).map_err(|e| anyhow::anyhow!("hook {name} @0x{va:x}: {e}"))
}

static EXE_BASE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
static PATCH_PROT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
/// Live temporary patches: (address, original 5 bytes).
static TEMP_PATCHES: Mutex<Vec<Option<(usize, [u8; JMP_LEN])>>> = Mutex::new(Vec::new());

unsafe fn write_patch_bytes(addr: usize, bytes: &[u8]) -> Result<(), String> {
    #[cfg(target_arch = "x86_64")]
    {
        let page = addr & !0xFFF;
        let span = ((addr + bytes.len() + 0xFFF) & !0xFFF) - page;
        let rwx = libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC;
        if libc::mprotect(page as *mut libc::c_void, span, rwx) != 0 {
            return Err(format!("mprotect failed at 0x{page:x}"));
        }
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), addr as *mut u8, bytes.len());
        libc::mprotect(page as *mut libc::c_void, span, PATCH_PROT.load(Ordering::Relaxed) as i32);
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = PATCH_PROT.load(Ordering::Relaxed);
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), addr as *mut u8, bytes.len());
    }
    Ok(())
}

/// Differential-test mock: redirect the exe function at `va` to `target`
/// (a plain `jmp`, no trampoline) until [`temp_unpatch`]. Works whether or not
/// `va` is itself hooked (the first 5 bytes are saved and restored).
fn temp_patch(va: u64, target: usize) -> Result<usize, String> {
    let base = EXE_BASE.load(Ordering::Relaxed);
    if base == 0 {
        return Err("exe not mapped".into());
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let prev = x86_runtime::exe::lookup_native_va(va).unwrap_or(0);
        x86_runtime::exe::register_native_va(va, target);
        let mut prev_bytes = [0u8; JMP_LEN];
        prev_bytes.copy_from_slice(&(prev as u64).to_le_bytes()[..JMP_LEN]);
        let mut t = TEMP_PATCHES.lock().unwrap();
        t.push(Some((va as usize, prev_bytes)));
        return Ok(t.len() - 1);
    }
    #[cfg(target_arch = "x86_64")]
    {
        let addr = va_to_addr(base, va);
        let near = thunks::alloc_near(addr, 14).ok_or("no memory near target")?;
        unsafe { thunks::write_code(near, &thunks::abs_jmp(target)) };
        let rel = i32::try_from(near as i64 - (addr as i64 + JMP_LEN as i64)).map_err(|_| "thunk out of range")?;
        let mut saved = [0u8; JMP_LEN];
        unsafe { std::ptr::copy_nonoverlapping(addr as *const u8, saved.as_mut_ptr(), JMP_LEN) };
        let mut patch = [0xE9u8; JMP_LEN];
        patch[1..].copy_from_slice(&rel.to_le_bytes());
        unsafe { write_patch_bytes(addr, &patch)? };
        let mut t = TEMP_PATCHES.lock().unwrap();
        t.push(Some((addr, saved)));
        Ok(t.len() - 1)
    }
}

fn temp_unpatch(token: usize) {
    let e = TEMP_PATCHES.lock().unwrap().get_mut(token).and_then(Option::take);
    if let Some((addr, saved)) = e {
        #[cfg(not(target_arch = "x86_64"))]
        {
            let _ = write_patch_bytes;
            let mut b8 = [0u8; 8];
            b8[..JMP_LEN].copy_from_slice(&saved);
            let prev = u64::from_le_bytes(b8) as usize;
            if prev != 0 {
                x86_runtime::exe::register_native_va(addr as u64, prev);
            } else {
                x86_runtime::exe::unregister_native_va(addr as u64);
            }
            return;
        }
        #[cfg(target_arch = "x86_64")]
        if let Err(e) = unsafe { write_patch_bytes(addr, &saved) } {
            log::error!("temp_unpatch 0x{addr:x}: {e}");
        }
    }
}

/// Apply every replacement from a [`x86_runtime::game_plugin::ReplacementSet`] to the mapped exe.
pub fn apply_registry(
    exe_base: usize,
    set: &x86_runtime::game_plugin::ReplacementSet,
) -> Result<usize> {
    apply_registry_with(exe_base, set, &loader_lookup, libc::PROT_READ | libc::PROT_EXEC)
}

fn apply_registry_with(
    exe_base: usize,
    set: &x86_runtime::game_plugin::ReplacementSet,
    lookup: FnLookup,
    restore_prot: i32,
) -> Result<usize> {
    let default_hook_shards = cfg!(not(target_arch = "x86_64"));
    let hook_shards = std::env::var_os("AW2_HOOK_SHARDS")
        .map_or(default_hook_shards, |v| v != "0")
        || std::env::var_os("AW2_DIFF_SHARDS").is_some_and(|v| v != "0");
    let min_va = std::env::var("AW2_HOOK_MIN_VA")
        .ok()
        .and_then(|s| u64::from_str_radix(s.trim_start_matches("0x"), 16).ok())
        .unwrap_or(0);
    let max_va = std::env::var("AW2_HOOK_MAX_VA")
        .ok()
        .and_then(|s| u64::from_str_radix(s.trim_start_matches("0x"), 16).ok())
        .unwrap_or(u64::MAX);
    let has_va_filter = std::env::var_os("AW2_HOOK_MIN_VA").is_some()
        || std::env::var_os("AW2_HOOK_MAX_VA").is_some();
    let is_special_abi = |va: u64| set.special_abi_vas.contains(&va);
    let mut reps: Vec<&'static x86_runtime::registry_types::Replacement> = if hook_shards {
        set.all
            .iter()
            .copied()
            .filter(|r| !is_special_abi(r.va))
            .filter(|r| (!has_va_filter && r.diff.is_some()) || (r.va >= min_va && r.va <= max_va))
            .collect()
    } else {
        set.base
            .iter()
            .copied()
            .filter(|r| !is_special_abi(r.va) && r.va >= min_va && r.va <= max_va)
            .collect()
    };
    reps.sort_by_key(|r| r.va);
    PREFERRED_EXE_BASE.store(set.preferred_exe_base, Ordering::Relaxed);
    x86_runtime::exe::set_aw2_registry_active(true);
    x86_runtime::exe::set_base(exe_base);
    EXE_BASE.store(exe_base, Ordering::Relaxed);
    PATCH_PROT.store(restore_prot as usize, Ordering::Relaxed);
    x86_runtime::difftest::set_patcher(temp_patch, temp_unpatch);
    for w in reps.windows(2) {
        if w[0].va == w[1].va {
            bail!("duplicate replacement VA 0x{:x} ({} and {})", w[0].va, w[0].name, w[1].name);
        }
    }
    if std::env::var_os("AW2_HOOK_LINT").is_some_and(|v| v != "0") {
        hook_lint(set);
    }
    install_try_thunk();
    x86_runtime::exe::set_trampoline_lookup(trampoline_for);
    // On non-x86_64 hosts (`aarch64-apple-darwin`), register the active replacements
    // in the native VA dispatch table so `exe::call_va!` and `emu_run` resolve
    // verified replacement VAs directly to their native `aarch64` Rust functions
    // while executing unverified shard functions accurately via `emu.rs`.
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = (lookup, restore_prot, build_shim, place_shim, hook_va);
        let pref = set.preferred_exe_base;
        let iat_window_end = pref.saturating_add(0x0300_0000);
        let is_unverified_iat_thunk = |r: &x86_runtime::registry_types::Replacement| -> bool {
            r.diff.is_none()
                && (pref..iat_window_end).contains(&r.va)
                && unsafe { x86_runtime::exe::rd::<u16>(va_to_addr_with(exe_base, pref, r.va)) == 0x25ff }
        };
        let native_count = reps
            .iter()
            .filter(|r| !x86_runtime::simd_ops::is_aot_va_stub(r.ptr) && !is_unverified_iat_thunk(r))
            .count();
        x86_runtime::exe::register_native_vas(
            reps.iter()
                .filter(|r| !x86_runtime::simd_ops::is_aot_va_stub(r.ptr) && !is_unverified_iat_thunk(r))
                .map(|r| (r.va, r.ptr as usize)),
        );
        for r in &reps {
            let addr = va_to_addr_with(exe_base, pref, r.va);
            record_trampoline(r.va, addr | ORIG_TRAMPOLINE_TAG);
        }
        let guest_jit = reps.iter().filter(|r| x86_runtime::simd_ops::is_aot_va_stub(r.ptr)).count();
        let (mut thunks_host, mut thunks_guest) = (0usize, 0usize);
        for r in reps.iter().filter(|r| !x86_runtime::simd_ops::is_aot_va_stub(r.ptr) && is_unverified_iat_thunk(r)) {
            if x86_runtime::emu::resolve_host_callable(r.va).is_some() {
                thunks_host += 1;
            } else {
                thunks_guest += 1;
            }
        }
        log::info!(
            "registered {} compiled native replacement functions in VA dispatch table ({} total in shards, {} guest-JIT, {} import thunks routed to their targets: {} host, {} guest DLL)",
            native_count,
            set.total_count,
            guest_jit,
            thunks_host + thunks_guest,
            thunks_host,
            thunks_guest,
        );
        return Ok(reps.len());
    }
    #[cfg(target_arch = "x86_64")]
    {
        let fence_all = std::env::var_os("AW2_FENCE_ALL").is_some_and(|v| v != "0");
        let count_hooks = std::env::var_os("AW2_HOOK_STATS").is_some_and(|v| v != "0")
            || std::env::var_os("AW2_PROBE").is_some();
        let rwx = libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC;
        let batched_span = if !reps.is_empty() && restore_prot != rwx {
            let first_page = va_to_addr_with(exe_base, set.preferred_exe_base, reps.first().unwrap().va) & !0xFFF;
            let last_end = (va_to_addr_with(exe_base, set.preferred_exe_base, reps.last().unwrap().va) + 64 + 0xFFF) & !0xFFF;
            let span = last_end.saturating_sub(first_page);
            if span > 0 && unsafe { libc::mprotect(first_page as *mut libc::c_void, span, rwx) } == 0 {
                Some((first_page, span))
            } else {
                None
            }
        } else {
            None
        };
        let loop_prot = if batched_span.is_some() { rwx } else { restore_prot };
        let mut fenced_count = 0usize;
        let mut hooked_count = 0usize;
        let mut hook_err = None;
        for r in &reps {
            let float_ret = set.float_return_vas.contains(&r.va);
            let fence = FENCES && (fence_all || r.throws != x86_runtime::registry_types::Throws::No);
            let shim = place_shim(&build_shim(r.ptr as usize, float_ret, fence));
            let p = match hook_va(exe_base, r.va, r.name, Dest::Fixed(shim), count_hooks, lookup, loop_prot) {
                Ok(p) => p,
                Err(e) if hook_shards && r.diff.is_none() => {
                    log::debug!("skipped unhookable shard entry {} @0x{:x}: {e}", r.name, r.va);
                    continue;
                }
                Err(e) => {
                    hook_err = Some(e);
                    break;
                }
            };
            if fence {
                fenced_count += 1;
            }
            hooked_count += 1;
            log::debug!(
                "hooked {} @0x{:x} ({:?}{}) -> {:p}, trampoline 0x{:x}",
                r.name,
                r.va,
                r.status,
                if fence { ", fenced" } else { "" },
                r.ptr,
                p.trampoline
            );
            record_trampoline(r.va, p.trampoline);
            if let Some(c) = p.counter {
                COUNTERS.lock().unwrap().push((r.va, r.name.to_string(), "replacement", c));
            }
        }
        if let Some((first_page, span)) = batched_span {
            unsafe {
                libc::mprotect(first_page as *mut libc::c_void, span, restore_prot);
            }
        }
        if let Some(e) = hook_err {
            return Err(e);
        }
        log::info!(
            "hooked {} functions from game replacement registry ({} fenced, {} total registered in shards)",
            hooked_count,
            fenced_count,
            set.total_count
        );
        Ok(hooked_count)
    }
}

/// Set up generic exe hook state (when no plugin [`ReplacementSet`] is active):
/// preferred base + patcher + try thunk + trampoline lookup.
pub fn init_generic_hooks(exe_base: usize) {
    let pref = x86_runtime::exe::preferred_base();
    if pref != 0 {
        PREFERRED_EXE_BASE.store(pref, Ordering::Relaxed);
    }
    EXE_BASE.store(exe_base, Ordering::Relaxed);
    x86_runtime::difftest::set_patcher(temp_patch, temp_unpatch);
    install_try_thunk();
    x86_runtime::exe::set_trampoline_lookup(trampoline_for);
}

/// Parse `AW2_PROBE`: `0xVA[,0xVA...]` or `@file` (one `0xVA [name]` per line,
/// `#` comments).
fn probe_list(spec: &str) -> Result<Vec<(u64, String)>> {
    let text;
    let body = if let Some(path) = spec.strip_prefix('@') {
        text = std::fs::read_to_string(path).map_err(|e| anyhow::anyhow!("AW2_PROBE file {path}: {e}"))?;
        text.as_str()
    } else {
        spec
    };
    let mut out = Vec::new();
    for line in body.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        if spec.starts_with('@') {
            let mut it = line.splitn(2, char::is_whitespace);
            let va = it.next().unwrap();
            let name = it.next().unwrap_or("").trim();
            let v = u64::from_str_radix(va.trim_start_matches("0x"), 16).map_err(|_| anyhow::anyhow!("bad probe VA {va:?}"))?;
            out.push((v, if name.is_empty() { format!("sub_{v:x}") } else { name.to_string() }));
        } else {
            for tok in line.split([',', ' ']).filter(|t| !t.is_empty()) {
                let v = u64::from_str_radix(tok.trim_start_matches("0x"), 16).map_err(|_| anyhow::anyhow!("bad probe VA {tok:?}"))?;
                out.push((v, format!("sub_{v:x}")));
            }
        }
    }
    Ok(out)
}

/// Install pass-through counted probes from `AW2_PROBE` (see module docs).
/// Returns (installed, skipped).
pub fn apply_probes(exe_base: usize) -> Result<(usize, usize)> {
    let Ok(spec) = std::env::var("AW2_PROBE") else { return Ok((0, 0)) };
    let list = probe_list(&spec)?;
    let (mut ok, mut skipped) = (0, 0);
    let mut seen: std::collections::HashSet<u64> = COUNTERS.lock().unwrap().iter().map(|c| c.0).collect();
    for (va, name) in list {
        if !seen.insert(va) {
            continue; // already hooked (registry replacement or duplicate)
        }
        match hook_va(exe_base, va, &name, Dest::Trampoline, true, &loader_lookup, libc::PROT_READ | libc::PROT_EXEC) {
            Ok(p) => {
                ok += 1;
                if let Some(c) = p.counter {
                    COUNTERS.lock().unwrap().push((va, name, "probe", c));
                }
            }
            Err(e) => {
                skipped += 1;
                log::debug!("probe skipped: {e}");
            }
        }
    }
    log::info!("AW2_PROBE: {ok} probes installed, {skipped} skipped (unhookable)");
    Ok((ok, skipped))
}

/// Start the counter reporter thread if `AW2_PROBE` or `AW2_HOOK_STATS` is set.
pub fn start_reporter() {
    let enabled = std::env::var_os("AW2_PROBE").is_some() || std::env::var_os("AW2_HOOK_STATS").is_some_and(|v| v != "0");
    if !enabled || COUNTERS.lock().unwrap().is_empty() {
        return;
    }
    let file = std::env::var_os("AW2_HOOK_STATS_FILE").map(std::path::PathBuf::from);
    let spawn = std::thread::Builder::new().name("aw2-hook-stats".into()).spawn(move || {
        let mut seen: Vec<u64> = Vec::new();
        let mut last_total = u64::MAX;
        loop {
            std::thread::sleep(std::time::Duration::from_millis(250));
            let counters = COUNTERS.lock().unwrap();
            let snap: Vec<u64> = counters.iter().map(|c| c.3.load(Ordering::Relaxed)).collect();
            if seen.len() < snap.len() {
                seen.resize(snap.len(), 0);
            }
            for (i, n) in snap.iter().enumerate() {
                if seen[i] == 0 && *n != 0 {
                    let (va, name, kind, _) = &counters[i];
                    log::info!(target: "hooks", "first call: {name} @0x{va:x} ({kind}, {n} calls so far)");
                }
                seen[i] = *n;
            }
            let total: u64 = snap.iter().sum();
            if let Some(path) = &file {
                if total != last_total {
                    let mut rows: Vec<_> = (0..snap.len()).filter(|&i| snap[i] != 0).collect();
                    rows.sort_by(|&a, &b| snap[b].cmp(&snap[a]));
                    let mut s = String::from("va\tname\tkind\tcalls\n");
                    for i in rows {
                        let (va, name, kind, n) = (counters[i].0, &counters[i].1, counters[i].2, snap[i]);
                        s.push_str(&format!("0x{va:x}\t{name}\t{kind}\t{n}\n"));
                    }
                    let tmp = path.with_extension("tmp");
                    if std::fs::write(&tmp, s).is_ok() {
                        let _ = std::fs::rename(&tmp, path);
                    }
                }
            }
            drop(counters);
            last_total = total;
        }
    });
    if let Err(e) = spawn {
        log::warn!("hook stats reporter: {e}");
    }
}

/// Run every registry differential test (original via trampoline vs Rust).
/// Returns (passed, failed). Entries without a `diff` function are skipped.
pub fn run_difftests(set: Option<&x86_runtime::game_plugin::ReplacementSet>) -> (usize, usize) {
    // AOT translation caches for the mapped images (AW2_DLL_AOT / AW2_DLL_AOT_BUILD).
    x86_runtime::emu::aot_activate();
    let _ = win32_rt::teb::map_kuser_shared_data();
    let teb = win32_rt::teb::current();
    unsafe {
        let mut tls_arr = x86_runtime::exe::tls_pointer() as usize;
        if tls_arr == 0 {
            tls_arr = Box::leak(vec![0usize; 64].into_boxed_slice()).as_mut_ptr() as usize;
            win32_rt::teb::set_field(teb, win32_rt::teb::off::TLS_POINTER, tls_arr);
            win32_rt::teb::gs_write(win32_rt::teb::off::TLS_POINTER, tls_arr);
        }
        if x86_runtime::exe::rd::<usize>(tls_arr) == 0 {
            let slot0 = Box::leak(vec![0u8; 0x80000].into_boxed_slice()).as_mut_ptr() as usize;
            x86_runtime::exe::wr::<usize>(tls_arr, slot0);
        }
    }
    let (mut pass, mut fail) = (0, 0);
    let diff_shards = std::env::var_os("AW2_DIFF_SHARDS").is_some_and(|v| v != "0");
    let diff_limit: usize = std::env::var("AW2_DIFF_LIMIT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(usize::MAX);
    let only: Vec<String> = std::env::var("AW2_DIFF_ONLY")
        .ok()
        .map(|s| s.split(',').map(|t| t.trim().to_ascii_lowercase()).filter(|t| !t.is_empty()).collect())
        .unwrap_or_default();
    if let Some(set) = set {
        let selected = |r: &x86_runtime::registry_types::Replacement| {
            only.is_empty()
                || only.iter().any(|t| {
                    let tv = t.trim_start_matches("0x").replace('_', "");
                    u64::from_str_radix(&tv, 16).is_ok_and(|v| v == r.va) || r.name.to_ascii_lowercase().contains(t.as_str())
                })
        };
        for &r in &set.all {
            if pass + fail >= diff_limit {
                break;
            }
            if !selected(r) {
                continue;
            }
            if r.diff.is_none() && !diff_shards {
                continue;
            }
            let Some(orig) = trampoline_for(r.va) else {
                log::error!("difftest FAIL {} @0x{:x}: not hooked (no trampoline)", r.name, r.va);
                fail += 1;
                continue;
            };
            let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
                if let Some(diff) = r.diff {
                    diff(orig, r.ptr as usize)
                } else {
                    x86_runtime::difftest::shard_leaf_difftest(r.va, orig, r.ptr as usize)
                }
            }));
            match res {
                Ok(Ok(cases)) => {
                    if r.diff.is_some() || pass < 25 || pass % 1000 == 0 {
                        log::info!("difftest ok   {} @0x{:x} ({:?}): {cases} cases (pass={})", r.name, r.va, r.status, pass + 1);
                    }
                    pass += 1;
                }
                Ok(Err(e)) if e.starts_with(x86_runtime::difftest::SKIP) => {
                    if r.diff.is_some() {
                        log::warn!("difftest skip {} @0x{:x}: {e}", r.name, r.va);
                    }
                }
                Ok(Err(e)) => {
                    log::error!("difftest FAIL {} @0x{:x}: {e}", r.name, r.va);
                    fail += 1;
                }
                Err(payload) => {
                    let msg = payload
                        .downcast_ref::<&str>()
                        .copied()
                        .or_else(|| payload.downcast_ref::<String>().map(|s| s.as_str()))
                        .unwrap_or("panic");
                    log::error!("difftest PANIC {} @0x{:x}: {msg}", r.name, r.va);
                    fail += 1;
                }
            }
        }
    }
    // Native replacements inside mapped DLLs (`x86_runtime::dll_native`): the original is the
    // guest code at the mapped address, entered through the emulator tag.
    #[cfg(not(target_arch = "x86_64"))]
    for d in x86_runtime::dll_native::difftests() {
        if pass + fail >= diff_limit {
            break;
        }
        let name = d.name.to_ascii_lowercase();
        if !only.is_empty() && !only.iter().any(|t| name.contains(t.as_str())) {
            continue;
        }
        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { (d.diff)(d.addr | ORIG_TRAMPOLINE_TAG, d.new) }));
        match res {
            Ok(Ok(cases)) => {
                log::info!("difftest ok   {} @0x{:x} (dll_native): {cases} cases (pass={})", d.name, d.addr, pass + 1);
                pass += 1;
            }
            Ok(Err(e)) if e.starts_with(x86_runtime::difftest::SKIP) => log::warn!("difftest skip {} @0x{:x}: {e}", d.name, d.addr),
            Ok(Err(e)) => {
                log::error!("difftest FAIL {} @0x{:x}: {e}", d.name, d.addr);
                fail += 1;
            }
            Err(_) => {
                log::error!("difftest PANIC {} @0x{:x}", d.name, d.addr);
                fail += 1;
            }
        }
    }
    // Exe guest-entry routing (`aw2_game::exe_route_diff`): import thunks and special-ABI CRT
    // helpers routed natively vs the exe's original x86 under the emulator.
    #[cfg(all(feature = "aw2-core", not(target_arch = "x86_64")))]
    if set.is_some() {
        for d in aw2_game::exe_route_diff::difftests() {
            if pass + fail >= diff_limit {
                break;
            }
            if !only.is_empty() && !only.iter().any(|t| d.name.contains(t.as_str())) {
                continue;
            }
            let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe { (d.run)() }));
            match res {
                Ok(Ok(cases)) => {
                    log::info!("difftest ok   {} (exe route): {cases} cases (pass={})", d.name, pass + 1);
                    pass += 1;
                }
                Ok(Err(e)) if e.starts_with(x86_runtime::difftest::SKIP) => log::warn!("difftest skip {}: {e}", d.name),
                Ok(Err(e)) => {
                    log::error!("difftest FAIL {}: {e}", d.name);
                    fail += 1;
                }
                Err(_) => {
                    log::error!("difftest PANIC {}", d.name);
                    fail += 1;
                }
            }
        }
    }
    if let Some(set) = set {
        let lint_errors = hook_lint(set).errors;
        if lint_errors > 0 {
            log::error!("difftest: hook-lint reported {lint_errors} error(s) (logged at hook time), counted as failures");
            fail += lint_errors;
        }
    }
    // `--diff-test` exits without the exit hooks: dump the AW2_JIT_PROFILE profile here.
    x86_runtime::emu::aot_profile_dump();
    (pass, fail)
}

// ---------------------------------------------------------------------------------------------
// Hook-safety lint (docs/design/ADVICE_SEH.md §5 step 1).
//
// Every registry entry declares `throws: Throws::{No, Discard, Caught}`. `tools/fndb.sh sync`
// writes `db/throws.tsv` with the original's can-throw masks (`fndb can-throw`) and its own EH
// shape (FH4 try map / unwind map). The table is embedded at compile time so the lint needs no
// files at run time. At hook time findings are logged as warnings; `--diff-test` (and the
// `registry_difftests_against_exe` test) count errors as failures.

/// Fence frames (hook shim with unwind info + unwinder skip over Rust frames) are implemented:
/// replacements annotated `Discard`/`Caught` are entered through a fence shim ([`build_shim`]).
const FENCES: bool = true;
/// `exe::try_call` (catch an MSVC exception from an original callee in Rust) is implemented
/// ([`install_try_thunk`]).
const TRY_CALL: bool = true;

#[cfg(feature = "aw2-core")]
const THROWS_TSV: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../db/throws.tsv"));
#[cfg(not(feature = "aw2-core"))]
const THROWS_TSV: &str = "";

// Mask bits from tools/fndb/canthrow.py.
const T_CXX: u32 = 1;
const T_CONTRACT: u32 = 2;
const T_SEH: u32 = 4;
const T_UNKNOWN: u32 = 8;
const T_STD: u32 = 16;
/// Exceptions a game `catch` may handle (contract throws are treated as abort-equivalent).
const T_CATCHABLE: u32 = T_CXX | T_SEH | T_STD;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ThrowFacts {
    /// Bits reachable through direct calls only.
    direct: u32,
    /// Bits reachable when vcalls are followed (over-approximated by vftable slot).
    all: u32,
    /// The original's own EH shape: `none`, `cleanup`, `catch`, `noexcept`.
    eh: &'static str,
    witness: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum LintLevel {
    Debug,
    Warn,
    Error,
}

fn parse_throws_tsv(text: &'static str) -> HashMap<u64, ThrowFacts> {
    let mut out = HashMap::new();
    for line in text.lines() {
        if line.starts_with('#') || line.starts_with("va\t") || line.trim().is_empty() {
            continue;
        }
        let f: Vec<&'static str> = line.split('\t').collect();
        let (Some(va), Some(direct), Some(all)) = (
            f.first().and_then(|s| u64::from_str_radix(s.trim_start_matches("0x"), 16).ok()),
            f.get(1).and_then(|s| s.parse().ok()),
            f.get(2).and_then(|s| s.parse().ok()),
        ) else {
            continue;
        };
        out.insert(va, ThrowFacts { direct, all, eh: f.get(3).copied().unwrap_or("none"), witness: f.get(4).copied().unwrap_or("") });
    }
    out
}

/// Findings for one entry (pure; see the module comment for the policy).
fn lint_rule(throws: x86_runtime::registry_types::Throws, f: &ThrowFacts) -> Vec<(LintLevel, String)> {
    use x86_runtime::registry_types::Throws;
    use LintLevel::*;
    let mut out = Vec::new();
    let catchable = f.direct & T_CATCHABLE != 0;
    let contract = f.direct & T_CONTRACT != 0;
    let wit = if f.witness.is_empty() { String::new() } else { format!(" [via {}]", f.witness) };
    // What the original requires.
    if f.eh == "catch" && catchable && throws != Throws::Caught {
        out.push((Error, format!("original has a try map and a throwing callee: needs Throws::Caught (exe::try_call){wit}")));
    } else if f.eh == "catch" && !catchable && throws != Throws::Caught {
        out.push((Warn, "original has a try map but no direct throwing callee: check what it catches (vcall/indirect callee, /EHa fault?)".into()));
    } else if f.eh == "cleanup" && catchable && throws != Throws::Caught {
        out.push((Error, format!("original runs destructors on unwind and a callee can throw: needs Throws::Caught (cleanup on Err, rethrow){wit}")));
    } else if (catchable || contract) && throws == Throws::No {
        out.push((Error, format!("a callee can throw ({}): annotate Throws::Discard (no live Drop values across it) or Caught{wit}", mask_names(f.direct))));
    }
    // What the annotation needs from the runtime.
    if throws == Throws::Discard && catchable && !FENCES {
        out.push((Error, "Throws::Discard with catchable throws needs fence frames (disabled): an exception would hit the Rust frame and be fatal".into()));
    }
    if throws == Throws::Caught && !TRY_CALL {
        out.push((Error, "Throws::Caught needs exe::try_call (not implemented yet)".into()));
    }
    // Over-annotation and weak evidence.
    if throws != Throws::No && f.direct & (T_CATCHABLE | T_CONTRACT) == 0 {
        let lvl = if f.all & T_CATCHABLE != 0 { Debug } else { Warn };
        out.push((lvl, format!("annotated {throws:?} but no direct callee can throw (with vcalls: {})", mask_names(f.all))));
    }
    if throws == Throws::No && f.direct & (T_CATCHABLE | T_CONTRACT) == 0 && f.all != 0 {
        out.push((Debug, format!("may throw only via vcalls/indirect calls (over-approximation): {}", mask_names(f.all))));
    }
    out
}

fn mask_names(m: u32) -> String {
    let names: Vec<&str> = [(T_CXX, "cxx"), (T_CONTRACT, "contract"), (T_SEH, "seh"), (T_UNKNOWN, "unknown-indirect"), (T_STD, "std")]
        .iter()
        .filter(|(b, _)| m & b != 0)
        .map(|(_, n)| *n)
        .collect();
    if names.is_empty() { "none".into() } else { names.join(",") }
}

#[derive(Debug, Clone, Copy, Default)]
struct LintSummary {
    errors: usize,
    warnings: usize,
}

fn hook_lint_with_tsv(
    set: &x86_runtime::game_plugin::ReplacementSet,
    tsv: &'static str,
) -> LintSummary {
    if tsv.is_empty() {
        log::info!("hook-lint: no throws table embedded; skipping");
        return LintSummary::default();
    }
    let facts = parse_throws_tsv(tsv);
    let mut s = LintSummary::default();
    for &r in &set.all {
        let Some(f) = facts.get(&r.va) else {
            log::warn!("hook-lint {} @0x{:x}: no row in db/throws.tsv (run tools/fndb.sh sync)", r.name, r.va);
            s.warnings += 1;
            continue;
        };
        for (lvl, msg) in lint_rule(r.throws, f) {
            match lvl {
                LintLevel::Error => {
                    log::warn!("hook-lint ERROR {} @0x{:x}: {msg} (fails --diff-test)", r.name, r.va);
                    s.errors += 1;
                }
                LintLevel::Warn => {
                    if r.diff.is_some() {
                        log::warn!("hook-lint {} @0x{:x}: {msg}", r.name, r.va);
                    } else {
                        log::debug!("hook-lint {} @0x{:x}: {msg}", r.name, r.va);
                    }
                    s.warnings += 1;
                }
                LintLevel::Debug => log::debug!("hook-lint {} @0x{:x}: {msg}", r.name, r.va),
            }
        }
    }
    log::info!(
        "hook-lint: {} replacements, {} error(s), {} warning(s) (fences: {FENCES}, try_call: {TRY_CALL})",
        set.total_count,
        s.errors,
        s.warnings
    );
    s
}

/// Lint the registry once (cached) and log the findings. Errors are logged as warnings here
/// (dev boots keep going); [`run_difftests`] turns them into failures.
fn hook_lint(set: &x86_runtime::game_plugin::ReplacementSet) -> LintSummary {
    static ONCE: std::sync::OnceLock<LintSummary> = std::sync::OnceLock::new();
    *ONCE.get_or_init(|| hook_lint_with_tsv(set, THROWS_TSV))
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    const RWX: i32 = libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC;

    unsafe extern "win64" fn replacement(a: u64, b: u64) -> u64 {
        a * 1000 + b
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn hook_and_trampoline() {
        // A small "original": lea rax,[rcx+rdx]; add rax, [rip+data]; ret ; data: dq 7
        let mut code: Vec<u8> = vec![
            0x48, 0x8D, 0x04, 0x11, // lea rax, [rcx+rdx]   (4 bytes; forces a 2-instruction patch)
            0x48, 0x03, 0x05, 0x01, 0, 0, 0, // add rax, [rip+1]
            0xC3, // ret
        ];
        code.extend_from_slice(&7u64.to_le_bytes());
        let anchor = hook_and_trampoline as *const () as usize;
        let at = thunks::alloc_near(anchor, 64).unwrap();
        unsafe { thunks::write_code(at, &code) };
        type F = unsafe extern "win64" fn(u64, u64) -> u64;
        let orig: F = unsafe { std::mem::transmute(at) };
        assert_eq!(unsafe { orig(2, 3) }, 12);
        let p = install(at, Some(at + 12), replacement as *const () as usize, RWX).unwrap();
        assert_eq!(p.len, 11);
        assert_eq!(unsafe { orig(2, 3) }, 2003);
        let tr: F = unsafe { std::mem::transmute(p.trampoline) };
        assert_eq!(unsafe { tr(2, 3) }, 12, "trampoline must run the original (RIP-relative fixed up)");
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn counted_probe_passes_through() {
        let mut code: Vec<u8> = vec![0x48, 0x8D, 0x04, 0x11, 0x48, 0x03, 0x05, 0x01, 0, 0, 0, 0xC3];
        code.extend_from_slice(&7u64.to_le_bytes());
        let at = thunks::alloc_near(counted_probe_passes_through as *const () as usize, 64).unwrap();
        unsafe { thunks::write_code(at, &code) };
        type F = unsafe extern "win64" fn(u64, u64) -> u64;
        let f: F = unsafe { std::mem::transmute(at) };
        let p = install_ex(at, Some(at + 12), Dest::Trampoline, true, RWX).unwrap();
        let c = p.counter.unwrap();
        assert_eq!(c.load(Ordering::Relaxed), 0);
        // Counters must not share a page with code (Rosetta treats such
        // stores as self-modifying code): the page is RW without execute.
        let mbi = win32_rt::kernel32::memory::query(c as *const AtomicU64 as usize).unwrap();
        assert_eq!(mbi.protect & 0xF0, 0, "counter on an executable page (protect {:#x})", mbi.protect);
        for i in 0..5 {
            assert_eq!(unsafe { f(i, 1) }, i + 8);
        }
        assert_eq!(c.load(Ordering::Relaxed), 5);
        // Counted hook to a replacement.
        let at2 = thunks::alloc_near(counted_probe_passes_through as *const () as usize, 64).unwrap();
        unsafe { thunks::write_code(at2, &code) };
        let g: F = unsafe { std::mem::transmute(at2) };
        let p2 = install_ex(at2, Some(at2 + 12), Dest::Fixed(replacement as *const () as usize), true, RWX).unwrap();
        assert_eq!(unsafe { g(2, 3) }, 2003);
        assert_eq!(p2.counter.unwrap().load(Ordering::Relaxed), 1);
    }

    #[test]
    fn refuses_short_function() {
        let code = [0xC3u8, 0xCC, 0xCC, 0xCC, 0x90, 0xCC];
        let at = thunks::alloc_near(refuses_short_function as *const () as usize, 16).unwrap();
        unsafe { thunks::write_code(at, &code) };
        assert!(install(at, None, replacement as *const () as usize, RWX).is_err());
    }

    #[test]
    fn native_emu_executes_x86_and_dispatches_va() {
        // x86-64 guest snippet:
        //   lea rax, [rcx + rdx]
        //   add rax, [rip + 1]
        //   ret
        //   dq 7
        let mut code: Vec<u8> = vec![
            0x48, 0x8D, 0x04, 0x11,
            0x48, 0x03, 0x05, 0x01, 0, 0, 0,
            0xC3,
        ];
        code.extend_from_slice(&7u64.to_le_bytes());
        let at = thunks::alloc_near(native_emu_executes_x86_and_dispatches_va as *const () as usize, 64).unwrap();
        unsafe { thunks::write_code(at, &code) };
        let res = unsafe { x86_runtime::emu::emu_run(at as u64, [2, 3, 0, 0, 0, 0, 0, 0]) };
        assert_eq!(res, 12);

        // Register a native replacement VA and verify `call_va8` dispatches to the native function.
        let fake_va = 0x1_43F0_0000u64;
        x86_runtime::exe::register_native_va(fake_va, replacement as *const () as usize);
        let hooked_res = unsafe { x86_runtime::exe::call_va8(fake_va, 2, 3, 0, 0, 0, 0, 0, 0) };
        assert_eq!(hooked_res, 2003);
    }

    /// Target that clobbers every volatile register and sums its 6 args.
    #[cfg(target_arch = "x86_64")]
    #[rustfmt::skip]
    const CLOBBER_SUM6: &[u8] = &[
        0x48, 0x8D, 0x04, 0x11,             // lea rax, [rcx+rdx]
        0x4C, 0x01, 0xC0,                   // add rax, r8
        0x4C, 0x01, 0xC8,                   // add rax, r9
        0x48, 0x03, 0x44, 0x24, 0x28,       // add rax, [rsp+0x28]  (arg 5)
        0x48, 0x03, 0x44, 0x24, 0x30,       // add rax, [rsp+0x30]  (arg 6)
        0x31, 0xC9, 0x31, 0xD2,             // xor ecx,ecx; xor edx,edx
        0x45, 0x31, 0xC0, 0x45, 0x31, 0xC9, // xor r8d,r8d; xor r9d,r9d
        0x45, 0x31, 0xD2, 0x45, 0x31, 0xDB, // xor r10d,r10d; xor r11d,r11d
        0x0F, 0x57, 0xC0, 0x0F, 0x57, 0xED, // xorps xmm0,xmm0; xorps xmm5,xmm5
        0xC3,
    ];

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn preserving_shim_keeps_volatiles() {
        let target = thunks::alloc_far(CLOBBER_SUM6);
        let shim = thunks::alloc_far(&preserving_shim(target, false));
        // Caller: set r9/r10/r11/xmm5 sentinels, call shim(1..6) and return
        // rax + r9 + r10 + r11 + (xmm5 low qword) + rcx.
        let mut c: Vec<u8> = vec![
            0x48, 0x83, 0xEC, 0x38, // sub rsp, 0x38
            0x48, 0xC7, 0x44, 0x24, 0x20, 5, 0, 0, 0, // mov qword [rsp+0x20], 5
            0x48, 0xC7, 0x44, 0x24, 0x28, 6, 0, 0, 0, // mov qword [rsp+0x28], 6
            0x49, 0xC7, 0xC2, 0x00, 0x01, 0, 0, // mov r10, 0x100
            0x49, 0xC7, 0xC3, 0x00, 0x10, 0, 0, // mov r11, 0x1000
            0xB8, 0x00, 0x00, 0x01, 0x00, // mov eax, 0x10000
            0x66, 0x48, 0x0F, 0x6E, 0xE8, // movq xmm5, rax
            0xB9, 1, 0, 0, 0, // mov ecx, 1
            0xBA, 2, 0, 0, 0, // mov edx, 2
            0x41, 0xB8, 3, 0, 0, 0, // mov r8d, 3
            0x41, 0xB9, 4, 0, 0, 0, // mov r9d, 4
            0x48, 0xB8, // mov rax, imm64 (shim)
        ];
        c.extend_from_slice(&(shim as u64).to_le_bytes());
        c.extend_from_slice(&[
            0xFF, 0xD0, // call rax           -> rax = 21
            0x4C, 0x01, 0xC8, // add rax, r9  (4)
            0x4C, 0x01, 0xD0, // add rax, r10 (0x100)
            0x4C, 0x01, 0xD8, // add rax, r11 (0x1000)
            0x48, 0x01, 0xC8, // add rax, rcx (1)
            0x66, 0x48, 0x0F, 0x7E, 0xE9, // movq rcx, xmm5
            0x48, 0x01, 0xC8, // add rax, rcx (0x10000)
            0x48, 0x83, 0xC4, 0x38, // add rsp, 0x38
            0xC3,
        ]);
        let f: unsafe extern "win64" fn() -> u64 = unsafe { std::mem::transmute(thunks::alloc_far(&c)) };
        assert_eq!(unsafe { f() }, 21 + 4 + 0x100 + 0x1000 + 1 + 0x10000);
    }

    #[test]
    fn probe_spec_parsing() {
        let v = probe_list("0x140204cc0, 1401fa730").unwrap();
        assert_eq!(v.iter().map(|x| x.0).collect::<Vec<_>>(), [0x140204cc0, 0x1401fa730]);
    }

    /// Minimal image mapping for differential tests of pure leaf functions:
    /// headers + sections at the preferred base (relocated if unavailable),
    /// no imports, no TLS, no DllMain. Everything stays RWX.
    fn map_exe_for_difftest(path: &std::path::Path) -> Result<(usize, Vec<pe_image::RuntimeFunction>)> {
        let data = std::fs::read(path)?;
        let pe = pe_image::Pe::parse(&data)?;
        let h = &pe.headers;
        let size = (h.size_of_image as usize + 0xFFF) & !0xFFF;
        let base = unsafe {
            libc::mmap(h.image_base as *mut libc::c_void, size, RWX, libc::MAP_PRIVATE | libc::MAP_ANON, -1, 0)
        };
        if base == libc::MAP_FAILED {
            bail!("mmap image failed");
        }
        let base = base as usize;
        let img = unsafe { std::slice::from_raw_parts_mut(base as *mut u8, size) };
        img[..h.size_of_headers as usize].copy_from_slice(&data[..h.size_of_headers as usize]);
        for s in &pe.sections {
            let n = (s.raw_size.min(s.mem_size()) as usize).min(data.len().saturating_sub(s.raw_offset as usize));
            img[s.rva as usize..s.rva as usize + n].copy_from_slice(&data[s.raw_offset as usize..s.raw_offset as usize + n]);
        }
        let delta = base as i64 - h.image_base as i64;
        if delta != 0 {
            pe_image::apply_relocations(img, &pe.relocations()?, delta)?;
        }
        // Imports: win32-rt implementations where available (strlen, ...),
        // host mem* for vcruntime140 (the real loader maps the game's own
        // vcruntime140.dll), trap thunks otherwise. Delay imports are left alone.
        unsafe extern "win64" fn t_memmove(d: *mut u8, s: *const u8, n: usize) -> *mut u8 {
            std::ptr::copy(s, d, n);
            d
        }
        unsafe extern "win64" fn t_memset(d: *mut u8, c: i32, n: usize) -> *mut u8 {
            std::ptr::write_bytes(d, c as u8, n);
            d
        }
        unsafe extern "win64" fn t_memcmp(a: *const u8, b: *const u8, n: usize) -> i32 {
            let (a, b) = (std::slice::from_raw_parts(a, n), std::slice::from_raw_parts(b, n));
            a.cmp(b) as i32
        }
        let vcruntime = |name: &str| -> Option<usize> {
            match name {
                "memcpy" | "memmove" => Some(t_memmove as *const () as usize),
                "memset" => Some(t_memset as *const () as usize),
                "memcmp" => Some(t_memcmp as *const () as usize),
                _ => None,
            }
        };
        for dll in pe.imports()? {
            for e in &dll.entries {
                let p = match &e.name {
                    pe_image::ImportName::Name { name, .. } => win32_rt::lookup(&dll.dll, name).map(|p| p as usize).or_else(|| {
                        dll.dll.eq_ignore_ascii_case("vcruntime140.dll").then(|| vcruntime(name)).flatten()
                    }),
                    pe_image::ImportName::Ordinal(o) => win32_rt::lookup_ordinal(&dll.dll, *o).map(|p| p as usize),
                };
                let p = p.unwrap_or_else(|| thunks::trap_thunk(&dll.dll, &e.name.to_string(), "AlanWake2.exe"));
                img[e.iat_rva as usize..e.iat_rva as usize + 8].copy_from_slice(&(p as u64).to_le_bytes());
            }
        }
        Ok((base, pe.runtime_functions()?))
    }

    /// Hooks every registry replacement in a minimally mapped AlanWake2.exe
    /// and runs the differential tests (original trampoline vs Rust).
    /// Skipped when the game is not installed.
    #[cfg(all(feature = "aw2-core", target_arch = "x86_64"))]
    #[test]
    fn registry_difftests_against_exe() {
        let Some(dir) = std::env::var_os("PORT_GAME_DIR")
            .or_else(|| std::env::var_os("AW2_GAME_DIR"))
            .filter(|v| !v.is_empty())
            .map(std::path::PathBuf::from)
        else {
            eprintln!("skipping: set PORT_GAME_DIR or AW2_GAME_DIR to run registry difftests");
            return;
        };
        let exe = dir.join("AlanWake2.exe");
        if !exe.exists() {
            eprintln!("skipping: {} not found (set AW2_GAME_DIR)", exe.display());
            return;
        }
        let _ = env_logger::builder().is_test(true).filter_level(log::LevelFilter::Info).try_init();
        let (base, pdata) = map_exe_for_difftest(&exe).unwrap();
        let lookup = move |addr: usize| -> Option<(usize, usize)> {
            let rva = u32::try_from(addr.checked_sub(base)?).ok()?;
            let f = pe_image::find_runtime_function(&pdata, rva)?;
            Some((base + f.begin_rva as usize, base + f.end_rva as usize))
        };
        aw2_game::register_plugin();
        let reps = x86_runtime::game_plugin::find_plugin(Some("alan-wake-2"), "alanwake2.exe")
            .and_then(|p| p.replacements())
            .expect("aw2 replacements");
        let n = apply_registry_with(base, &reps, &lookup, RWX).unwrap();
        let (pass, fail) = run_difftests(Some(&reps));
        eprintln!("difftests: {n} hooks, {pass} passed, {fail} failed");
        assert_eq!(fail, 0, "differential tests failed (see log)");
    }

    // -----------------------------------------------------------------------------------------
    // Fence frames (docs/design/ADVICE_SEH.md §5 step 2).
    //
    // outer (guest code with .pdata + an __except-style language handler)
    //   -> fat hook shim (fence) -> fence_mid (Rust) -> inner (guest code with .pdata)
    //   -> RaiseException
    // `inner` clobbers every nonvolatile except rbp before raising, standing in for whatever the
    // discarded Rust frames left in registers. The landing pad in `outer` stores its registers,
    // which must equal the sentinels `outer` set before the call (restored by the shim's unwind
    // info alone).

    use win32_rt::unwind::{Context, DispatcherContext, ExceptionRecord};
    use std::sync::atomic::AtomicUsize;

    const TEST_CODE: u32 = 0xE000_0001;
    const TEST_CODE_NESTED: u32 = 0xE000_0002;
    /// Nonvolatile GPRs in the order the landing pad stores them.
    const LAND_GPRS: [u8; 8] = [3, 5, 6, 7, 12, 13, 14, 15];

    static FENCE_LANDING: AtomicUsize = AtomicUsize::new(0);
    static FENCE_CONTROL_PC: AtomicUsize = AtomicUsize::new(0);
    /// 0 = plain RtlUnwindEx to the landing pad, 1 = STATUS_UNWIND_CONSOLIDATE callback that
    /// returns the landing pad, 2 = the callback raises TEST_CODE_NESTED (a rethrow from a catch).
    static FENCE_MODE: AtomicUsize = AtomicUsize::new(0);
    static CALLBACK_RAN: AtomicUsize = AtomicUsize::new(0);
    static INNER_HANDLER_NESTED: AtomicUsize = AtomicUsize::new(0);

    fn gpr_sentinel(i: usize) -> u64 {
        0xA5A5_0000_0000_005A | (i as u64 + 1) << 16
    }
    fn xmm_sentinel(i: usize) -> u64 {
        0xC3C3_0000_0000_0000 | (i as u64 + 1) << 8
    }

    /// `[rex] op modrm(mod=10, reg, rm) [sib] disp32`.
    fn mem_d32(c: &mut Vec<u8>, prefix: &[u8], reg: u8, rm_rsp: bool, disp: u32) {
        c.extend_from_slice(prefix);
        if rm_rsp {
            c.push(0x84 | (reg & 7) << 3);
            c.push(0x24);
        } else {
            c.push(0x81 | (reg & 7) << 3); // [rcx + disp32]
        }
        c.extend_from_slice(&disp.to_le_bytes());
    }

    use super::place_unwound_code as place_guest;

    /// `outer(shim: rcx, inner: rdx, buf: r8) -> u64`. Returns 1 if the call returned normally;
    /// from the landing pad it returns RtlUnwindEx's return value. Records the landing pad and
    /// the return address of its call in FENCE_LANDING / FENCE_CONTROL_PC.
    fn build_outer(handler: usize) -> usize {
        let mut c: Vec<u8> = Vec::new();
        let mut codes = Vec::new();
        for &r in &LAND_GPRS {
            if r >= 8 {
                c.push(0x41);
            }
            c.push(0x50 | (r & 7)); // push r
            codes.push((c.len() as u8, 0u8, r, None)); // UWOP_PUSH_NONVOL
        }
        const FRAME: u32 = 0xD8;
        c.extend_from_slice(&[0x48, 0x81, 0xEC]); // sub rsp, imm32
        c.extend_from_slice(&FRAME.to_le_bytes());
        codes.push((c.len() as u8, 1, 0, Some((FRAME / 8) as u16))); // UWOP_ALLOC_LARGE
        for x in 6..16u8 {
            let off = 0x30 + 16 * (x as u32 - 6);
            mem_d32(&mut c, if x >= 8 { &[0x44, 0x0F, 0x29] } else { &[0x0F, 0x29] }, x, true, off); // movaps [rsp+d], xmm
            codes.push((c.len() as u8, 8, x, Some((off / 16) as u16))); // UWOP_SAVE_XMM128
        }
        let prolog_len = c.len();
        mem_d32(&mut c, &[0x4C, 0x89], 0, true, 0x28); // mov [rsp+0x28], r8
        for (i, &r) in LAND_GPRS.iter().enumerate() {
            c.extend_from_slice(&[0x48 | (r >= 8) as u8, 0xB8 | (r & 7)]); // mov r, imm64
            c.extend_from_slice(&gpr_sentinel(i).to_le_bytes());
        }
        for x in 6..16u8 {
            c.extend_from_slice(&[0x48, 0xB8]); // mov rax, imm64
            c.extend_from_slice(&xmm_sentinel(x as usize - 6).to_le_bytes());
            c.extend_from_slice(&[0x66, if x >= 8 { 0x4C } else { 0x48 }, 0x0F, 0x6E, 0xC0 | (x & 7) << 3]); // movq xmm, rax
        }
        c.extend_from_slice(&[0x48, 0x89, 0xC8]); // mov rax, rcx
        c.extend_from_slice(&[0x48, 0x89, 0xD1]); // mov rcx, rdx
        c.extend_from_slice(&[0xFF, 0xD0]); // call rax
        let control_pc = c.len();
        c.extend_from_slice(&[0xB8, 1, 0, 0, 0]); // mov eax, 1
        c.extend_from_slice(&[0xE9, 0, 0, 0, 0]); // jmp epilog (patched below)
        let jmp_end = c.len();
        let landing = c.len();
        mem_d32(&mut c, &[0x48, 0x8B], 1, true, 0x28); // mov rcx, [rsp+0x28]
        mem_d32(&mut c, &[0x48, 0x89], 0, false, 0xE0); // mov [rcx+0xE0], rax
        for (i, &r) in LAND_GPRS.iter().enumerate() {
            mem_d32(&mut c, &[0x48 | ((r >= 8) as u8) << 2, 0x89], r, false, 8 * i as u32); // mov [rcx+d], r
        }
        for x in 6..16u8 {
            mem_d32(&mut c, if x >= 8 { &[0x44, 0x0F, 0x11] } else { &[0x0F, 0x11] }, x, false, 0x40 + 16 * (x as u32 - 6)); // movups [rcx+d], xmm
        }
        let epilog = c.len();
        c[jmp_end - 4..jmp_end].copy_from_slice(&((epilog - jmp_end) as i32).to_le_bytes());
        for x in 6..16u8 {
            mem_d32(&mut c, if x >= 8 { &[0x44, 0x0F, 0x28] } else { &[0x0F, 0x28] }, x, true, 0x30 + 16 * (x as u32 - 6)); // movaps xmm, [rsp+d]
        }
        c.extend_from_slice(&[0x48, 0x81, 0xC4]); // add rsp, imm32
        c.extend_from_slice(&FRAME.to_le_bytes());
        for &r in LAND_GPRS.iter().rev() {
            if r >= 8 {
                c.push(0x41);
            }
            c.push(0x58 | (r & 7)); // pop r
        }
        c.push(0xC3);
        let base = place_guest(&c, prolog_len, &codes, Some(handler));
        FENCE_LANDING.store(base + landing, Ordering::SeqCst);
        FENCE_CONTROL_PC.store(base + control_pc, Ordering::SeqCst);
        base
    }

    /// `inner(raise: rcx)`: clobbers rbx, rsi, rdi, r12-r15 and xmm6-15 (all ones), then
    /// `RaiseException(TEST_CODE, 0, 0, NULL)`. Its unwind info only records `sub rsp, 0x28`.
    fn build_inner(handler: usize) -> usize {
        let mut c: Vec<u8> = vec![0x48, 0x83, 0xEC, 0x28]; // sub rsp, 0x28
        let codes = [(4u8, 2u8, 4u8, None)]; // UWOP_ALLOC_SMALL (0x28 - 8) / 8
        c.extend_from_slice(&[0x48, 0x89, 0xC8]); // mov rax, rcx
        for &r in LAND_GPRS.iter().filter(|&&r| r != 5) {
            c.extend_from_slice(&[0x48 | (r >= 8) as u8, 0xC7, 0xC0 | (r & 7), 0xFF, 0xFF, 0xFF, 0xFF]); // mov r, -1
        }
        for x in 6..16u8 {
            if x >= 8 {
                c.extend_from_slice(&[0x66, 0x45, 0x0F, 0x76, 0xC0 | (x & 7) << 3 | (x & 7)]);
            } else {
                c.extend_from_slice(&[0x66, 0x0F, 0x76, 0xC0 | x << 3 | x]); // pcmpeqd xmm, xmm
            }
        }
        c.push(0xB9);
        c.extend_from_slice(&TEST_CODE.to_le_bytes()); // mov ecx, TEST_CODE
        c.extend_from_slice(&[0x31, 0xD2, 0x45, 0x31, 0xC0, 0x45, 0x31, 0xC9]); // xor edx/r8d/r9d
        c.extend_from_slice(&[0xFF, 0xD0]); // call rax
        c.extend_from_slice(&[0x48, 0x83, 0xC4, 0x28, 0xC3]); // add rsp, 0x28; ret
        place_guest(&c, 4, &codes, Some(handler))
    }

    type RtlUnwindExFn = unsafe extern "win64" fn(u64, u64, *mut ExceptionRecord, u64, *mut Context, *mut std::ffi::c_void);
    type RaiseFn = unsafe extern "win64" fn(u32, u32, u32, *const u64);

    fn raise_fn() -> RaiseFn {
        unsafe { std::mem::transmute(win32_rt::unwind::RaiseException as *const ()) }
    }

    /// The "Rust replacement" between the fence and the raising original.
    #[inline(never)]
    unsafe extern "win64" fn fence_mid(inner: usize) -> u64 {
        let f: unsafe extern "win64" fn(usize) -> u64 = std::mem::transmute(inner);
        std::hint::black_box(f(raise_fn() as usize)) + 1
    }

    /// STATUS_UNWIND_CONSOLIDATE callback (stands in for `__CxxCallCatchBlock`).
    unsafe extern "win64" fn fence_consolidate_cb(rec: *mut ExceptionRecord) -> u64 {
        CALLBACK_RAN.fetch_add(1, Ordering::SeqCst);
        if FENCE_MODE.load(Ordering::SeqCst) == 2 {
            FENCE_MODE.store(0, Ordering::SeqCst); // the nested exception is caught plainly
            raise_fn()(TEST_CODE_NESTED, 0, 0, std::ptr::null());
            unreachable!("nested exception returned");
        }
        (*rec).information[1]
    }

    /// `outer`'s language handler: an `__except(EXCEPTION_EXECUTE_HANDLER)` for TEST_CODE and
    /// TEST_CODE_NESTED.
    unsafe extern "win64" fn fence_outer_handler(rec: *mut ExceptionRecord, frame: u64, _ctx: *mut Context, _dc: *mut DispatcherContext) -> u32 {
        let r = &mut *rec;
        let ours = r.code == TEST_CODE || r.code == TEST_CODE_NESTED || r.code == win32_rt::unwind::CXX_EXCEPTION;
        if r.flags & win32_rt::unwind::EXCEPTION_UNWINDING != 0 || !ours {
            return 1; // ExceptionContinueSearch
        }
        let unwind: RtlUnwindExFn = std::mem::transmute(win32_rt::unwind::RtlUnwindEx as *const ());
        let landing = FENCE_LANDING.load(Ordering::SeqCst) as u64;
        let mut out = Context::zeroed();
        let retval = if r.code == TEST_CODE_NESTED { 0x78 } else { 0x77 };
        if FENCE_MODE.load(Ordering::SeqCst) == 0 {
            unwind(frame, landing, rec, retval, &mut out, std::ptr::null_mut());
        } else {
            let mut cons = ExceptionRecord {
                code: win32_rt::unwind::STATUS_UNWIND_CONSOLIDATE,
                flags: win32_rt::unwind::EXCEPTION_NONCONTINUABLE,
                record: rec,
                address: std::ptr::null_mut(),
                number_parameters: 2,
                _pad: 0,
                information: [0; 15],
            };
            cons.information[0] = fence_consolidate_cb as *const () as usize as u64;
            cons.information[1] = landing;
            let control_pc = FENCE_CONTROL_PC.load(Ordering::SeqCst) as u64;
            unwind(frame, control_pc, &mut cons, retval, &mut out, std::ptr::null_mut());
        }
        unreachable!("RtlUnwindEx returned");
    }

    /// `inner`'s language handler: never handles, counts nested-exception visits (a rethrow
    /// from the catch block must not revisit frames that were already unwound).
    unsafe extern "win64" fn fence_inner_handler(rec: *mut ExceptionRecord, _f: u64, _c: *mut Context, _d: *mut DispatcherContext) -> u32 {
        if (*rec).code == TEST_CODE_NESTED {
            INNER_HANDLER_NESTED.fetch_add(1, Ordering::SeqCst);
        }
        1
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn fence_frame_catches_through_rust() {
        let _ = env_logger::builder().is_test(true).try_init();
        let outer = build_outer(fence_outer_handler as *const () as usize);
        let inner = build_inner(fence_inner_handler as *const () as usize);
        let shim = place_shim(&build_shim(fence_mid as *const () as usize, false, true));
        // Mode 3: a C++ exception that exe::try_call does not catch passes through the try thunk
        // and the Rust frames to `outer`.
        install_try_thunk();
        let (thrower, cxx_args) = build_cxx_thrower();
        TRY_ARGS.store(cxx_args, Ordering::SeqCst);
        let shim_try = place_shim(&build_shim(fence_mid_try as *const () as usize, false, true));
        type Outer = unsafe extern "win64" fn(usize, usize, *mut u64) -> u64;
        let f: Outer = unsafe { std::mem::transmute(outer) };
        for (mode, expect_ret, expect_cb) in [(0usize, 0x77u64, 0usize), (1, 0x77, 1), (2, 0x78, 1), (3, 0x77, 0)] {
            FENCE_MODE.store(mode, Ordering::SeqCst);
            CALLBACK_RAN.store(0, Ordering::SeqCst);
            INNER_HANDLER_NESTED.store(0, Ordering::SeqCst);
            let mut buf = [0u64; 30];
            let ret = if mode == 3 {
                FENCE_MODE.store(0, Ordering::SeqCst);
                unsafe { f(shim_try, thrower, buf.as_mut_ptr()) }
            } else {
                unsafe { f(shim, inner, buf.as_mut_ptr()) }
            };
            assert_eq!(ret, expect_ret, "mode {mode}: landing pad not reached via RtlUnwindEx");
            assert_eq!(buf[28], expect_ret, "mode {mode}: rax at the landing pad");
            for i in 0..LAND_GPRS.len() {
                assert_eq!(buf[i], gpr_sentinel(i), "mode {mode}: nonvolatile GPR #{} not restored by the fence", LAND_GPRS[i]);
            }
            for i in 0..10 {
                assert_eq!((buf[8 + 2 * i], buf[9 + 2 * i]), (xmm_sentinel(i), 0), "mode {mode}: xmm{} not restored", i + 6);
            }
            assert_eq!(CALLBACK_RAN.load(Ordering::SeqCst), expect_cb, "mode {mode}: consolidate callback runs");
            assert_eq!(INNER_HANDLER_NESTED.load(Ordering::SeqCst), 0, "mode {mode}: rethrow revisited an unwound frame");
        }
    }

    static TRY_ARGS: AtomicUsize = AtomicUsize::new(0);

    /// A replacement that calls a throwing original through exe::try_call with a catch list that
    /// does not match (the exception must continue to the frames above).
    #[inline(never)]
    unsafe extern "win64" fn fence_mid_try(thrower: usize) -> u64 {
        let args = [raise_fn() as usize as u64, TRY_ARGS.load(Ordering::SeqCst) as u64];
        match x86_runtime::exe::try_call(thrower, &args, &[b".PEAVOther@@"]) {
            Ok(r) => r.rax + 1,
            Err(_) => 0xBAD,
        }
    }

    /// A synthetic MSVC throw: `thrower(raise: rcx, args: rdx)` (guest code with .pdata) does
    /// `RaiseException(0xE06D7363, EXCEPTION_NONCONTINUABLE, 4, args)` with args
    /// `{0x19930520, &object, &ThrowInfo, image_base}`. The "image" is a heap block holding the
    /// ThrowInfo, a CatchableTypeArray of `Derived*` (exact) and `Base*` (mdisp 0x10) and their
    /// TypeDescriptors; the object is the pointer 0x1234_5000. Returns (thrower, args).
    fn build_cxx_thrower() -> (usize, usize) {
        let img: &'static mut [u8] = Box::leak(vec![0u8; 0x200].into_boxed_slice());
        let base = img.as_ptr() as u64;
        let mut put = |off: usize, v: &[u8]| img[off..off + v.len()].copy_from_slice(v);
        let le = |v: i32| v.to_le_bytes();
        put(0x0C, &le(0x20)); // ThrowInfo.pCatchableTypeArray
        put(0x20, &le(2));
        put(0x24, &le(0x40));
        put(0x28, &le(0x60));
        for (ct, td, mdisp, name) in [(0x40, 0x100, 0, &b".PEAVDerived@@\0"[..]), (0x60, 0x140, 0x10, &b".PEAVBase@@\0"[..])] {
            put(ct, &le(1)); // CT_IsSimpleType
            put(ct + 4, &le(td as i32));
            put(ct + 8, &le(mdisp));
            put(ct + 12, &le(-1)); // pdisp: no virtual base
            put(ct + 20, &le(8)); // sizeOrOffset
            put(td + 16, name);
        }
        put(0x180, &0x1234_5000u64.to_le_bytes());
        for (i, v) in [0x1993_0520u64, base + 0x180, base, base].iter().enumerate() {
            put(0x1A0 + 8 * i, &v.to_le_bytes());
        }
        #[rustfmt::skip]
        let code = [
            0x48, 0x83, 0xEC, 0x28,             // sub rsp, 0x28
            0x48, 0x89, 0xC8,                   // mov rax, rcx
            0x49, 0x89, 0xD1,                   // mov r9, rdx
            0xB9, 0x63, 0x73, 0x6D, 0xE0,       // mov ecx, 0xE06D7363
            0xBA, 0x01, 0x00, 0x00, 0x00,       // mov edx, EXCEPTION_NONCONTINUABLE
            0x41, 0xB8, 0x04, 0x00, 0x00, 0x00, // mov r8d, 4
            0xFF, 0xD0,                         // call rax
            0x48, 0x83, 0xC4, 0x28, 0xC3,       // add rsp, 0x28; ret
        ];
        (place_guest(&code, 4, &[(4, 2, 4, None)], None), (base + 0x1A0) as usize)
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn try_call_catches_cxx_exceptions() {
        use x86_runtime::exe::try_call;
        let _ = env_logger::builder().is_test(true).try_init();
        install_try_thunk();
        let (thrower, args) = build_cxx_thrower();
        let a = [raise_fn() as usize as u64, args as u64];
        unsafe {
            let e = try_call(thrower, &a, &[b".PEAVOther@@", b".PEAVBase@@"]).unwrap_err();
            assert_eq!((e.matched, e.size, e.ptr()), (1, 8, 0x1234_5010), "Base*: pointer adjusted by mdisp");
            assert_eq!(e.type_name().as_deref(), Some(".PEAVDerived@@"));
            let e = try_call(thrower, &a, &[b".PEAVDerived@@"]).unwrap_err();
            assert_eq!((e.matched, e.ptr()), (0, 0x1234_5000));
            let e = try_call(thrower, &a, &[]).unwrap_err();
            assert_eq!((e.matched, e.ptr()), (u32::MAX, 0x1234_5000), "catch (...)");
        }
        unsafe extern "win64" fn sum6(a: u64, b: u64, c: u64, d: u64, e: u64, f: u64) -> u64 {
            a + 2 * b + 3 * c + 4 * d + 5 * e + 6 * f
        }
        unsafe extern "win64" fn half(x: f64) -> f64 {
            x / 2.0
        }
        unsafe {
            let r = try_call(sum6 as *const () as usize, &[1, 2, 3, 4, 5, 6], &[]).unwrap();
            assert_eq!(r.rax, 1 + 4 + 9 + 16 + 25 + 36, "register and stack args");
            let r = try_call(half as *const () as usize, &[3.0f64.to_bits()], &[]).unwrap();
            assert_eq!(f64::from_bits(r.xmm0), 1.5, "double arg/return via xmm0");
        }
    }

    #[test]
    fn host_frame_without_fence_is_not_crossed() {
        #[inline(never)]
        fn check() -> bool {
            let mut ctx = Context::zeroed();
            unsafe { win32_rt::unwind::RtlCaptureContext(&mut ctx) };
            let mut out = Context::zeroed();
            unsafe { win32_rt::unwind::cross_host_frames(&ctx, &mut out) }
        }
        assert!(!check(), "a Rust frame that was not entered through a fence shim must stay a stop");
    }

    #[test]
    fn hook_lint_rules() {
        use x86_runtime::registry_types::Throws;
        let f = |direct, all, eh| ThrowFacts { direct, all, eh, witness: "" };
        let worst = |t, facts: &ThrowFacts| lint_rule(t, facts).iter().map(|(l, _)| *l).max();
        // Nothing below throws.
        assert_eq!(worst(Throws::No, &f(0, 0, "none")), None);
        assert_eq!(worst(Throws::No, &f(T_UNKNOWN, T_UNKNOWN, "none")), Some(LintLevel::Debug));
        assert_eq!(worst(Throws::No, &f(0, T_CXX, "cleanup")), Some(LintLevel::Debug));
        // Contract throws (abort-equivalent): Discard is fine without fences.
        assert_eq!(worst(Throws::No, &f(T_CONTRACT, T_CONTRACT, "none")), Some(LintLevel::Error));
        assert_eq!(worst(Throws::Discard, &f(T_CONTRACT | T_UNKNOWN, 27, "cleanup")), None);
        // Catchable throws: Discard needs fences, Caught needs try_call.
        assert_eq!(worst(Throws::No, &f(T_CXX, T_CXX, "none")), Some(LintLevel::Error));
        assert_eq!(worst(Throws::Discard, &f(T_CXX, T_CXX, "none")).is_some(), !FENCES);
        assert_eq!(worst(Throws::Discard, &f(T_STD, T_STD, "cleanup")), Some(LintLevel::Error));
        assert_eq!(worst(Throws::Discard, &f(T_CXX, T_CXX, "catch")), Some(LintLevel::Error));
        assert_eq!(worst(Throws::Caught, &f(T_CXX, T_CXX, "catch")).is_some(), !TRY_CALL);
        // Try map without a visible thrower, and over-annotation.
        assert_eq!(worst(Throws::No, &f(0, 0, "catch")), Some(LintLevel::Warn));
        assert_eq!(worst(Throws::Discard, &f(0, 0, "none")), Some(LintLevel::Warn));
    }

    #[test]
    fn hook_lint_generic_replacement_set() {
        use x86_runtime::game_plugin::ReplacementSet;
        use x86_runtime::registry_types::Replacement;

        static OK_FN: Replacement = Replacement::draft(0x1_4000_1000, "demo::ok_fn", std::ptr::null());
        static BAD_FN: Replacement = Replacement::draft(0x1_4000_2000, "demo::bad_fn", std::ptr::null());
        let set = ReplacementSet {
            base: vec![&OK_FN, &BAD_FN],
            all: vec![&OK_FN, &BAD_FN],
            total_count: 2,
            float_return_vas: &[],
            special_abi_vas: &[],
            preferred_exe_base: 0x1_4000_0000,
        };
        let tsv = "va\tmask_direct\tmask_all\teh\twitness\n0x140001000\t0\t0\tnone\t\n0x140002000\t1\t1\tnone\tthrower\n";
        let summary = hook_lint_with_tsv(&set, tsv);
        assert_eq!(summary.errors, 1);
        assert_eq!(summary.warnings, 0);
        assert_eq!(hook_lint_with_tsv(&set, "").errors, 0);
    }

    #[cfg(feature = "aw2-core")]
    #[test]
    fn hook_lint_registry_clean() {
        aw2_game::register_plugin();
        let reps = x86_runtime::game_plugin::find_plugin(Some("alan-wake-2"), "alanwake2.exe")
            .and_then(|p| p.replacements())
            .expect("aw2 replacements");
        let facts = parse_throws_tsv(THROWS_TSV);
        for &r in &reps.all {
            let f = facts.get(&r.va).unwrap_or_else(|| panic!("0x{:x} {} missing from db/throws.tsv (tools/fndb.sh sync)", r.va, r.name));
            let errs: Vec<_> = lint_rule(r.throws, f).into_iter().filter(|(l, _)| *l == LintLevel::Error).collect();
            assert!(errs.is_empty(), "0x{:x} {}: {:?}", r.va, r.name, errs);
        }
    }
}

/// End-to-end check of `win32_rt::win64_bridge!` through the interpreter's
/// real host-call path (`aw2_game::emu::call_host_with_xmm`), with arguments
/// laid out the way an MSVC Win64 caller leaves them.
#[cfg(all(test, target_arch = "aarch64"))]
mod bridge_abi_tests {
    use std::sync::Mutex;

    static SEEN: Mutex<Vec<String>> = Mutex::new(Vec::new());

    /// `SetRTPCValue(const char*, float, u64, i32, i32, bool)` shape.
    unsafe extern "win64" fn rtpc(name: *const u8, v: f32, obj: u64, ms: i32, curve: i32, bypass: bool) -> i32 {
        SEEN.lock().unwrap().push(format!("{:#x} {v} {obj:#x} {ms} {curve} {bypass}", name as usize));
        7
    }
    win32_rt::win64_bridge!(fn rtpc(*const u8, f32, u64, i32, i32, bool) -> i32);

    /// `ClearDepthStencilView` shape, plus an f64 in a stack slot (slot 8).
    unsafe extern "win64" fn mixed(a: u64, b: u64, c: u32, d: f32, e: u8, n: u32, p: u64, f: f64) -> i32 {
        SEEN.lock().unwrap().push(format!("{a:#x} {b:#x} {c} {d} {e} {n} {p:#x} {f}"));
        9
    }
    win32_rt::win64_bridge!(fn mixed(u64, u64, u32, f32, u8, u32, u64, f64) -> i32);

    /// An f32 in an xmm register with junk above the low 32 bits.
    fn x(f: f32) -> u128 {
        f.to_bits() as u128 | 0x1111_2222_3333_4444_dead_beef_0000_0000
    }

    #[test]
    fn bridge_entries_read_win64_slots() {
        unsafe {
            // rcx=name, xmm1=v (rdx junk), r8=obj, r9=ms, [5]=curve, [6]=bypass (junk above the low byte).
            let gpr = [0x1234, 0xdead, 0x55, 250, 3, 0xabcd_ef00_0000_0001, 0, 0];
            let f = win32_rt::bridge_entry!(rtpc) as usize;
            let (r, _) = x86_runtime::emu::call_host_with_xmm(f, gpr, [x(9.0), x(0.75), 0, 0], [0; 8]);
            assert_eq!(r as i32, 7);
            // xmm3 = depth (r9 junk), slot 8 = f64 bits from the stack.
            let gpr = [0x10, 0x20, 3, 0xdead, 0x1_ff, 2, 0x7d_0000_0000, 2.5f64.to_bits()];
            let f = win32_rt::bridge_entry!(mixed) as usize;
            let (r, _) = x86_runtime::emu::call_host_with_xmm(f, gpr, [x(1.0), x(2.0), x(3.0), x(0.5)], [0; 8]);
            assert_eq!(r as i32, 9);
        }
        let seen = SEEN.lock().unwrap().clone();
        assert_eq!(seen, ["0x1234 0.75 0x55 250 3 true", "0x10 0x20 3 0.5 255 2 0x7d00000000 2.5"]);
    }
}
