//! Register-preserving hook shims, SEH fence frames, and `exe::try_call` C++ exception catching.

#[cfg(not(target_arch = "x86_64"))]
use super::patch::ORIG_TRAMPOLINE_TAG;
use crate::thunks;
use std::sync::atomic::{AtomicUsize, Ordering};

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
const _: () =
    assert!(FENCE_NV_GPR_SAVE == SHIM_FRAME - 8 && FENCE_NV_XMM_SAVE == FENCE_NV_GPR_SAVE + 0x40);
const _: () = assert!(FENCE_FRAME == FENCE_NV_XMM_SAVE + 0xA0 + 8 && FENCE_FRAME % 16 == 8);

/// Generated hook shim: code, the offset just after `call <target>`, and
/// (fence shims only) its UNWIND_INFO.
pub(super) struct Shim {
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
pub(super) fn preserving_shim(target: usize, float_ret: bool) -> Vec<u8> {
    build_shim(target, float_ret, false).code
}

/// Register-preserving shim, optionally as a **fence frame** (docs/design/ADVICE_SEH.md):
/// the prolog also saves rbx, rbp, rsi, rdi, r12–r15 and xmm6–xmm15 with matching
/// unwind codes. When an exception raised below the Rust replacement unwinds to
/// an original frame above it, win32-rt skips the (unwind-info-less) Rust
/// frames along the frame-pointer chain to this shim's return address, and the
/// shim's unwind info restores the caller's nonvolatiles exactly.
pub(super) fn build_shim(target: usize, float_ret: bool, fence: bool) -> Shim {
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
            rsp_d32(
                &mut c,
                &[if r >= 8 { 0x4C } else { 0x48 }, 0x89],
                0x84 | (r & 7) << 3,
                off,
            ); // mov [rsp+d], r
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
            rsp_d32(
                &mut c,
                &[if r >= 8 { 0x4C } else { 0x48 }, 0x8B],
                0x84 | (r & 7) << 3,
                off,
            ); // mov r, [rsp+d]
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
pub(super) fn place_shim(shim: &Shim) -> usize {
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
        win32_rt::unwind::RtlAddFunctionTable(
            (base + rf_off) as *const pe_image::RuntimeFunction,
            1,
            base as u64,
        );
    }
    win32_rt::unwind::register_fence((base + shim.ret_off) as u64);
    base
}

/// UNWIND_INFO (version 1) from prolog codes `(end offset, op, info, extra slot)` given in
/// prolog order, with an optional exception handler RVA (`UNW_FLAG_EHANDLER`, handler data 0).
fn unwind_info(
    prolog_len: usize,
    codes: &[(u8, u8, u8, Option<u16>)],
    handler_rva: Option<u32>,
) -> Vec<u8> {
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
pub(super) fn place_unwound_code(
    code: &[u8],
    prolog_len: usize,
    codes: &[(u8, u8, u8, Option<u16>)],
    handler: Option<usize>,
) -> usize {
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
    unsafe {
        win32_rt::unwind::RtlAddFunctionTable(
            (base + rf_off) as *const pe_image::RuntimeFunction,
            1,
            base as u64,
        );
    }
    base
}

// ---------------------------------------------------------------------------------------------
// exe::try_call support (docs/design/ADVICE_SEH.md §3.3): a call thunk with .pdata whose
// language handler claims matching MSVC C++ exceptions and unwinds to the thunk's landing pad.
// ---------------------------------------------------------------------------------------------

const TRY_FRAME: u32 = 0x88;
/// Where the thunk keeps the `TryFrame*` (rsp-relative after the prolog = establisher frame).
const TRY_FRAME_SLOT: u32 = 0x80;
static TRY_LANDING: AtomicUsize = AtomicUsize::new(0);

/// Build and register the try thunk once and hand it to `x86_runtime::exe`.
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
            c.extend_from_slice(&[
                0x49, 0x8B, 0x0B, 0x49, 0x8B, 0x53, 0x08, 0x4D, 0x8B, 0x43, 0x10, 0x4D, 0x8B,
                0x4B, 0x18,
            ]); // rcx, rdx, r8, r9
            c.extend_from_slice(&[
                0x66, 0x48, 0x0F, 0x6E, 0xC1, 0x66, 0x48, 0x0F, 0x6E, 0xCA,
            ]); // movq xmm0, rcx; movq xmm1, rdx
            c.extend_from_slice(&[
                0x66, 0x49, 0x0F, 0x6E, 0xD0, 0x66, 0x49, 0x0F, 0x6E, 0xD9,
            ]); // movq xmm2, r8; movq xmm3, r9
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
            let base =
                place_unwound_code(&c, prolog_len, &codes, Some(try_handler as *const () as usize));
            TRY_LANDING.store(base + landing, Ordering::SeqCst);
            x86_runtime::exe::set_try_thunk(base);
            base
        }
    })
}

type RtlUnwindExFn = unsafe extern "win64" fn(
    u64,
    u64,
    *mut win32_rt::unwind::ExceptionRecord,
    u64,
    *mut win32_rt::unwind::Context,
    *mut std::ffi::c_void,
);

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
    let frame = x86_runtime::exe::rd::<*mut x86_runtime::exe::TryFrame>(
        (establisher + TRY_FRAME_SLOT as u64) as usize,
    );
    let Some(exc) = cxx_match(r, &*frame) else {
        return CONTINUE_SEARCH;
    };
    (*frame).exc = exc;
    let unwind: RtlUnwindExFn = std::mem::transmute(u::RtlUnwindEx as *const ());
    let mut out = u::Context::zeroed();
    unwind(
        establisher,
        TRY_LANDING.load(Ordering::SeqCst) as u64,
        rec,
        1,
        &mut out,
        std::ptr::null_mut(),
    );
    unreachable!("RtlUnwindEx returned");
}

/// Match a C++ exception against a `TryFrame` catch list (see [`try_handler`]). MSVC x64
/// layouts, all RVAs from `ExceptionInformation[3]`: `ThrowInfo { attributes, pmfnUnwind,
/// pForwardCompat, pCatchableTypeArray }`, `CatchableTypeArray { n, rva[n] }`, `CatchableType
/// { properties, pType, PMD { mdisp, pdisp, vdisp }, sizeOrOffset, copyFunction }`,
/// `TypeDescriptor { vftable, spare, name[] }`.
unsafe fn cxx_match(
    r: &win32_rt::unwind::ExceptionRecord,
    f: &x86_runtime::exe::TryFrame,
) -> Option<x86_runtime::exe::CxxException> {
    use x86_runtime::exe::rd;
    let (obj, ti, base) = (r.information[1], r.information[2], r.information[3]);
    if ti == 0 {
        return None; // `throw;` outside a catch block: nothing to match
    }
    let rva = |at: u64| base.wrapping_add(rd::<i32>(at as usize) as i64 as u64);
    let dtor = rd::<i32>(ti as usize + 4);
    let cta = rva(ti + 12);
    let wanted: &[(usize, usize)] = if f.catch_len == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(f.catch, f.catch_len)
    };
    for i in 0..rd::<i32>(cta as usize).max(0) as u64 {
        let ct = rva(cta + 4 + 4 * i);
        let td = rva(ct + 4);
        let name = std::ffi::CStr::from_ptr((td + 16) as *const std::ffi::c_char).to_bytes();
        let matched = if wanted.is_empty() {
            u32::MAX
        } else {
            match wanted
                .iter()
                .position(|&(p, l)| std::slice::from_raw_parts(p as *const u8, l) == name)
            {
                Some(m) => m as u32,
                None => continue,
            }
        };
        let props = rd::<u32>(ct as usize);
        let (mdisp, pdisp) = (rd::<i32>(ct as usize + 8), rd::<i32>(ct as usize + 12));
        let size = rd::<i32>(ct as usize + 20);
        let copy_fn = rd::<i32>(ct as usize + 24);
        let is_ptr = name.starts_with(b".PE") && size == 8;
        let copyable = (props & 1 != 0 || copy_fn == 0)
            && (0..=32).contains(&size)
            && (is_ptr || dtor == 0)
            && (!is_ptr || pdisp < 0);
        let mut exc = x86_runtime::exe::CxxException {
            throw_info: ti,
            image_base: base,
            object: obj,
            matched,
            size: 0,
            value: [0; 32],
        };
        if copyable {
            std::ptr::copy_nonoverlapping(obj as *const u8, exc.value.as_mut_ptr(), size as usize);
            exc.size = size as u32;
            if is_ptr {
                let p = exc.ptr() as u64;
                if p != 0 {
                    exc.value[..8]
                        .copy_from_slice(&p.wrapping_add(mdisp as i64 as u64).to_le_bytes());
                }
            }
        } else if wanted.is_empty() {
            // catch (...) of an object we can't copy: destroy it as MSVC does at the end of the
            // catch block (no value, no rethrow).
            if dtor != 0 {
                let d: unsafe extern "win64" fn(u64) =
                    std::mem::transmute(base.wrapping_add(dtor as i64 as u64) as usize);
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
