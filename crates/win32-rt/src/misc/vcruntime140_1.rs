//! `vcruntime140_1.dll` — MSVC C++ Frame Handler 4 (`__CxxFrameHandler4`).
//!
//! Decodes `FH4::FuncInfo4`, `IPtoStateMap4`, `UWMap4`, `TryBlockMap4`, and
//! `HandlerMap4` tables in-place (matching `vcruntime140_1.dll` `0x180001000..0x180004300`)
//! and drives unwinding and catch-funclet dispatch through `win32_rt::unwind`.

// @exports: __CxxFrameHandler4, __NLG_Dispatch2, __NLG_Return2

use crate::Export;
use crate::unwind::{
    CXX_EXCEPTION, Context, DispatcherContext, EXCEPTION_EXIT_UNWIND, EXCEPTION_NONCONTINUABLE,
    EXCEPTION_TARGET_UNWIND, EXCEPTION_UNWINDING, ExceptionRecord, RtlUnwindEx,
    STATUS_UNWIND_CONSOLIDATE,
};
use std::ffi::{CStr, c_char, c_void};

pub static EXTRA_EXPORTS: &[Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

const NEG_LENGTH_TAB: [i8; 16] = [-1, -2, -1, -3, -1, -2, -1, -4, -1, -2, -1, -3, -1, -2, -1, -5];
const SHIFT_TAB: [u8; 16] = [25, 18, 25, 11, 25, 18, 25, 4, 25, 18, 25, 11, 25, 18, 25, 0];

const STATUS_LONGJUMP: u32 = 0x8000_0026;
const UNWIND_MASK: u32 = EXCEPTION_UNWINDING
    | EXCEPTION_EXIT_UNWIND
    | EXCEPTION_TARGET_UNWIND
    | crate::unwind::EXCEPTION_COLLIDED_UNWIND;

/// Decode an FH4 variable-length unsigned integer (`0x180003a41`).
#[inline]
pub unsafe fn read_unsigned(p: &mut *const u8) -> u32 {
    let idx = (**p & 0x0f) as usize;
    let neg_len = NEG_LENGTH_TAB[idx] as isize;
    let shift = SHIFT_TAB[idx];
    *p = (*p).wrapping_offset(-neg_len);
    let raw = std::ptr::read_unaligned((*p).wrapping_sub(4) as *const u32);
    raw >> shift
}

#[inline]
unsafe fn read_i32(p: &mut *const u8) -> i32 {
    let v = std::ptr::read_unaligned(*p as *const i32);
    *p = (*p).add(4);
    v
}

#[derive(Clone, Copy, Default)]
pub struct FuncInfo4 {
    pub header: u8,
    pub bbt_flags: u32,
    pub disp_unwind_map: i32,
    pub disp_try_block_map: i32,
    pub disp_ip_to_state_map: i32,
    pub disp_frame: u32,
}

impl FuncInfo4 {
    #[inline]
    pub fn is_catch(&self) -> bool {
        (self.header & 0x01) != 0
    }
    #[inline]
    pub fn is_separated(&self) -> bool {
        (self.header & 0x02) != 0
    }
    #[inline]
    pub fn is_ehs(&self) -> bool {
        (self.header & 0x20) != 0
    }
    #[inline]
    pub fn is_noexcept(&self) -> bool {
        (self.header & 0x40) != 0
    }
}

/// `FH4::DecompFuncInfo` (`0x180003a0c`).
pub unsafe fn decomp_func_info(
    pb: *const u8,
    image_base: u64,
    function_begin_rva: u32,
) -> FuncInfo4 {
    let mut fi = FuncInfo4::default();
    if pb.is_null() {
        return fi;
    }
    let header = *pb;
    fi.header = header;
    let mut p = pb.add(1);
    if (header & 0x04) != 0 {
        fi.bbt_flags = read_unsigned(&mut p);
    }
    if (header & 0x08) != 0 {
        fi.disp_unwind_map = read_i32(&mut p);
    }
    if (header & 0x10) != 0 {
        fi.disp_try_block_map = read_i32(&mut p);
    }
    let ip_map_rva = read_i32(&mut p);
    if (header & 0x02) != 0 {
        if ip_map_rva != 0 {
            let mut seg_p = (image_base as isize + ip_map_rva as isize) as *const u8;
            let num_segs = read_unsigned(&mut seg_p);
            for _ in 0..num_segs {
                let seg_begin = std::ptr::read_unaligned(seg_p as *const u32);
                let seg_map = std::ptr::read_unaligned(seg_p.add(4) as *const i32);
                seg_p = seg_p.add(8);
                if seg_begin == function_begin_rva {
                    fi.disp_ip_to_state_map = seg_map;
                    break;
                }
            }
        }
    } else {
        fi.disp_ip_to_state_map = ip_map_rva;
    }
    if (header & 0x01) != 0 {
        fi.disp_frame = read_unsigned(&mut p);
    }
    fi
}

/// `FH4::StateFromIp` (`0x1800038c8`).
pub unsafe fn state_from_ip(fi: &FuncInfo4, dc: &DispatcherContext, ip: u64) -> i32 {
    if fi.disp_ip_to_state_map == 0 || dc.function_entry.is_null() {
        return -1;
    }
    let mut p = (dc.image_base as isize + fi.disp_ip_to_state_map as isize) as *const u8;
    let count = read_unsigned(&mut p);
    if count == 0 {
        return -1;
    }
    let fn_begin = dc.image_base + (*dc.function_entry).begin_rva as u64;
    let mut cur_off = 0u32;
    let mut prev_state = -1i32;
    for i in 0..count {
        let delta = read_unsigned(&mut p);
        cur_off = cur_off.wrapping_add(delta);
        if ip < fn_begin.wrapping_add(cur_off as u64) {
            return if i == 0 { -1 } else { prev_state };
        }
        let encoded_state = read_unsigned(&mut p);
        prev_state = (encoded_state as i32) - 1;
    }
    prev_state
}

#[derive(Clone, Copy, Default)]
pub struct UnwindEntry4 {
    pub next_offset: u32,
    pub kind: u32,
    pub action: i32,
    pub object: u32,
}

unsafe fn read_unwind_map(fi: &FuncInfo4, image_base: u64) -> Vec<UnwindEntry4> {
    if fi.disp_unwind_map == 0 {
        return Vec::new();
    }
    let mut p = (image_base as isize + fi.disp_unwind_map as isize) as *const u8;
    let count = read_unsigned(&mut p) as usize;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let v = read_unsigned(&mut p);
        let next_offset = v >> 2;
        let kind = v & 3;
        let action = if kind != 0 { read_i32(&mut p) } else { 0 };
        let object = if kind == 1 || kind == 2 { read_unsigned(&mut p) } else { 0 };
        out.push(UnwindEntry4 {
            next_offset,
            kind,
            action,
            object,
        });
    }
    out
}

#[inline]
unsafe fn primary_frame(establisher_frame: u64, fi: &FuncInfo4) -> u64 {
    if fi.is_catch() {
        std::ptr::read_unaligned((establisher_frame + fi.disp_frame as u64) as *const u64)
    } else {
        establisher_frame
    }
}

/// `FrameHandler4::FrameUnwindToState` (`0x180002a94`).
unsafe fn frame_unwind_to_state(
    establisher_frame: u64,
    dc: &mut DispatcherContext,
    fi: &FuncInfo4,
    mut cur_state: i32,
    target_state: i32,
) {
    let entries = read_unwind_map(fi, dc.image_base);
    if entries.is_empty() {
        return;
    }
    let est = primary_frame(establisher_frame, fi);
    super::vcruntime140::inc_uncaught_exceptions();
    while cur_state > target_state && (cur_state as usize) < entries.len() {
        let entry = entries[cur_state as usize];
        if entry.next_offset == 0 {
            break;
        }
        cur_state -= entry.next_offset as i32;
        dc.scope_index = (cur_state + 2) as u32;
        if entry.kind != 0 && entry.action != 0 {
            let fn_va = (dc.image_base as isize + entry.action as isize) as usize;
            match entry.kind {
                1 => {
                    // DtorWithObj: rcx = est + object, rdx = est
                    let obj = est.wrapping_add(entry.object as u64);
                    let _ = crate::call_guest8(fn_va, obj, est, 0, 0, 0, 0, 0, 0);
                }
                2 => {
                    // DtorWithPtrToObj: rcx = *(est + object), rdx = est
                    let obj_ptr = est.wrapping_add(entry.object as u64) as *const u64;
                    let obj = std::ptr::read_unaligned(obj_ptr);
                    let _ = crate::call_guest8(fn_va, obj, est, 0, 0, 0, 0, 0, 0);
                }
                3 => {
                    // RVA funclet: rcx = fn_va, rdx = est
                    let _ = crate::call_guest8(fn_va, fn_va as u64, est, 0, 0, 0, 0, 0, 0);
                }
                _ => {}
            }
        }
    }
    super::vcruntime140::dec_uncaught_exceptions();
}

#[derive(Clone, Copy, Default)]
struct TryBlockMapEntry4 {
    try_low: i32,
    try_high: i32,
    #[allow(dead_code)]
    catch_high: i32,
    disp_handler_array: i32,
}

unsafe fn read_try_blocks(fi: &FuncInfo4, image_base: u64) -> Vec<TryBlockMapEntry4> {
    if fi.disp_try_block_map == 0 {
        return Vec::new();
    }
    let mut p = (image_base as isize + fi.disp_try_block_map as isize) as *const u8;
    let count = read_unsigned(&mut p) as usize;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let try_low = read_unsigned(&mut p) as i32;
        let try_high = read_unsigned(&mut p) as i32;
        let catch_high = read_unsigned(&mut p) as i32;
        let disp_handler_array = read_i32(&mut p);
        out.push(TryBlockMapEntry4 {
            try_low,
            try_high,
            catch_high,
            disp_handler_array,
        });
    }
    out
}

#[derive(Clone, Copy, Default)]
struct HandlerEntry4 {
    #[allow(dead_code)]
    header: u8,
    adjectives: u32,
    disp_type: i32,
    disp_catch_obj: u32,
    disp_of_handler: i32,
    cont_addr: [u64; 2],
}

unsafe fn read_handlers(
    disp_handler_array: i32,
    image_base: u64,
    fn_begin_rva: u32,
) -> Vec<HandlerEntry4> {
    if disp_handler_array == 0 {
        return Vec::new();
    }
    let mut p = (image_base as isize + disp_handler_array as isize) as *const u8;
    let count = read_unsigned(&mut p) as usize;
    let mut out = Vec::with_capacity(count);
    for _ in 0..count {
        let header = *p;
        p = p.add(1);
        let adjectives = if (header & 0x01) != 0 {
            read_unsigned(&mut p)
        } else {
            0
        };
        let disp_type = if (header & 0x02) != 0 {
            read_i32(&mut p)
        } else {
            0
        };
        let disp_catch_obj = if (header & 0x04) != 0 {
            read_unsigned(&mut p)
        } else {
            0
        };
        let disp_of_handler = read_i32(&mut p);
        let mut cont_addr = [0u64; 2];
        let cont_type = (header & 0x30) >> 4;
        let is_rva = (header & 0x08) != 0;
        if is_rva {
            if cont_type == 1 {
                cont_addr[0] = read_i32(&mut p) as i64 as u64;
            } else if cont_type == 2 {
                cont_addr[0] = read_i32(&mut p) as i64 as u64;
                cont_addr[1] = read_i32(&mut p) as i64 as u64;
            }
        } else if cont_type == 1 {
            cont_addr[0] = (fn_begin_rva.wrapping_add(read_unsigned(&mut p))) as u64;
        } else if cont_type == 2 {
            cont_addr[0] = (fn_begin_rva.wrapping_add(read_unsigned(&mut p))) as u64;
            cont_addr[1] = (fn_begin_rva.wrapping_add(read_unsigned(&mut p))) as u64;
        }
        out.push(HandlerEntry4 {
            header,
            adjectives,
            disp_type,
            disp_catch_obj,
            disp_of_handler,
            cont_addr,
        });
    }
    out
}

unsafe fn type_match4(
    handler: &HandlerEntry4,
    catchable: *const u8,
    throw_info: *const u8,
    handler_image_base: u64,
    throw_image_base: u64,
) -> bool {
    if handler.disp_type == 0 {
        return true;
    }
    let handler_td = (handler_image_base as isize + handler.disp_type as isize) as *const u8;
    if *handler_td.add(16) == 0 {
        return true;
    }
    let ct_props = std::ptr::read_unaligned(catchable as *const u32);
    let ct_td_rva = std::ptr::read_unaligned(catchable.add(4) as *const i32);
    if (handler.adjectives & 0x80) != 0 && (ct_props & 0x10) != 0 {
        return false;
    }
    let ct_td = (throw_image_base as isize + ct_td_rva as isize) as *const u8;
    if handler_td != ct_td {
        let h_name = CStr::from_ptr(handler_td.add(16) as *const c_char).to_bytes();
        let c_name = CStr::from_ptr(ct_td.add(16) as *const c_char).to_bytes();
        if h_name != c_name {
            return false;
        }
    }
    let ti_attrs = std::ptr::read_unaligned(throw_info as *const u32);
    if (ct_props & 0x02) != 0 && (handler.adjectives & 0x08) == 0 {
        return false;
    }
    if (ti_attrs & 0x01) != 0 && (handler.adjectives & 0x01) == 0 {
        return false;
    }
    if (ti_attrs & 0x04) != 0 && (handler.adjectives & 0x04) == 0 {
        return false;
    }
    if (ti_attrs & 0x02) != 0 && (handler.adjectives & 0x02) == 0 {
        return false;
    }
    true
}

/// Build the caught object at `est + handler.disp_catch_obj` (`0x1800014b8`).
unsafe fn build_catch_object4(
    exc_record: &ExceptionRecord,
    est: u64,
    handler: &HandlerEntry4,
    catchable: *const u8,
    handler_image_base: u64,
    throw_image_base: u64,
) {
    if handler.disp_type == 0 {
        return;
    }
    let handler_td = (handler_image_base as isize + handler.disp_type as isize) as *const u8;
    if *handler_td.add(16) == 0 {
        return;
    }
    if handler.disp_catch_obj == 0 && (handler.adjectives & 0x8000_0000) == 0 {
        return;
    }
    let catch_buf = if (handler.adjectives & 0x8000_0000) != 0 {
        est as *mut u8
    } else {
        (est + handler.disp_catch_obj as u64) as *mut u8
    };
    let exc_obj = exc_record.information[1] as *mut u8;
    if exc_obj.is_null() || catch_buf.is_null() {
        return;
    }
    let ct_props = std::ptr::read_unaligned(catchable as *const u32);
    let pmd = [
        std::ptr::read_unaligned(catchable.add(8) as *const i32),
        std::ptr::read_unaligned(catchable.add(12) as *const i32),
        std::ptr::read_unaligned(catchable.add(16) as *const i32),
    ];
    let size = std::ptr::read_unaligned(catchable.add(20) as *const i32).max(0) as usize;
    let copy_fn_rva = std::ptr::read_unaligned(catchable.add(24) as *const i32);

    if (handler.adjectives & 0x08) != 0 {
        // Catch by reference: store pointer to adjusted object.
        let adj = super::vcruntime140::__AdjustPointer(exc_obj, &pmd);
        std::ptr::write_unaligned(catch_buf as *mut *mut u8, adj);
    } else if (ct_props & 0x01) != 0 {
        // Simple type / pointer copy.
        std::ptr::copy_nonoverlapping(exc_obj, catch_buf, size);
        if size == 8 {
            let p = std::ptr::read_unaligned(catch_buf as *const *mut u8);
            if !p.is_null() {
                let adj = super::vcruntime140::__AdjustPointer(p, &pmd);
                std::ptr::write_unaligned(catch_buf as *mut *mut u8, adj);
            }
        }
    } else if copy_fn_rva == 0 {
        let adj = super::vcruntime140::__AdjustPointer(exc_obj, &pmd);
        std::ptr::copy_nonoverlapping(adj, catch_buf, size);
    } else {
        let adj = super::vcruntime140::__AdjustPointer(exc_obj, &pmd);
        let copy_fn = (throw_image_base as isize + copy_fn_rva as isize) as usize;
        if (ct_props & 0x04) != 0 {
            let _ = crate::call_guest8(copy_fn, catch_buf as u64, adj as u64, 1, 0, 0, 0, 0, 0);
        } else {
            let _ = crate::call_guest8(copy_fn, catch_buf as u64, adj as u64, 0, 0, 0, 0, 0, 0);
        }
    }
}

std::thread_local! {
    /// FH4's per-thread state override (`0x180003448`; -2 = none). The catch-block frame sets
    /// it for the next FH4 frame, the catch's parent, whose ControlPc is still the in-try call
    /// site: catchHigh while a throw from the catch block is dispatched (the same try must
    /// not catch it), tryLow while one unwinds (the try's objects are already destroyed).
    /// Consumed by the next FH4 frame; reset after every `__CxxFrameHandler4` call.
    static STATE_OVERRIDE: std::cell::Cell<i32> = const { std::cell::Cell::new(-2) };
}

/// The frame's current state (`FrameUnwindToState` / `FindHandler` prologue):
/// `DispatcherContext::ScopeIndex` (collided unwind), else the override, else ControlPc.
unsafe fn current_state(fi: &FuncInfo4, dc: &DispatcherContext) -> i32 {
    if dc.scope_index != 0 {
        return (dc.scope_index as i32) - 2;
    }
    match STATE_OVERRIDE.with(|s| s.replace(-2)) {
        -2 => state_from_ip(fi, dc, dc.control_pc),
        s => s,
    }
}

/// `__CxxCallCatchBlock`'s own frame around the catch funclet (its `__try` / `__except
/// (ExFilterRethrow)` / `__finally`, scope table of `0x180002600`), as an
/// [`crate::unwind::EhHostFrame`].
#[repr(C)]
struct CatchFrame {
    /// The caught exception.
    this_exc: *mut ExceptionRecord,
    try_low: i32,
    catch_high: i32,
    /// The catch block rethrew the caught exception (its object stays alive).
    rethrow: bool,
    prev_exc: *mut c_void,
}

/// MSVC C++ exception record (`PER_IS_MSVC_PURE_OR_NATIVE_EH`).
fn is_msvc_cxx(r: &ExceptionRecord) -> bool {
    r.code == CXX_EXCEPTION && r.number_parameters == 4 && (0x1993_0520..=0x1993_0522).contains(&r.information[0])
}

unsafe extern "win64" fn catch_frame_handler(
    rec: *mut ExceptionRecord,
    _frame: u64,
    _ctx: *mut Context,
    dc: *mut DispatcherContext,
) -> u32 {
    let f = &mut *((*dc).handler_data as *mut CatchFrame);
    let r = &mut *rec;
    if (r.flags & UNWIND_MASK) != 0 {
        // `__finally` (`0x1800048f7`): an exception leaves the catch block.
        if !f.rethrow && !f.this_exc.is_null() {
            super::vcruntime140::destruct_exception_object(f.this_exc);
        }
        *super::vcruntime140::__current_exception() = f.prev_exc;
        STATE_OVERRIDE.with(|s| s.set(f.try_low));
        return 1;
    }
    // `__except (ExFilterRethrow)` (`0x1800048c8` -> `0x1800029e0`).
    if is_msvc_cxx(r) && !f.this_exc.is_null() {
        let this = &*f.this_exc;
        if r.information[1] == this.information[1] {
            f.rethrow = true;
        }
        if r.information[2] == 0 {
            // `throw;`: the filter executes the handler, which re-raises the caught
            // exception (`RaiseException(code, flags, n, params)` of it) from this frame.
            // Same effect here: the dispatch continues past this frame with that record.
            f.rethrow = true;
            r.code = this.code;
            r.flags = this.flags & EXCEPTION_NONCONTINUABLE;
            r.number_parameters = this.number_parameters;
            r.information = this.information;
        }
    }
    STATE_OVERRIDE.with(|s| s.set(f.catch_high));
    1
}

/// Consolidate callback (`__CxxCallCatchBlock`, `0x180002600`) invoked by `RtlUnwindEx`
/// for `STATUS_UNWIND_CONSOLIDATE`.
unsafe extern "win64" fn call_catch_block4(rec: *mut ExceptionRecord) -> u64 {
    let r = &*rec;
    let est_ptr = r.information[1] as *const u64;
    let est = if est_ptr.is_null() { 0 } else { std::ptr::read_unaligned(est_ptr) };
    let handler_va = r.information[2] as usize;
    let cont_addr0 = r.information[5];
    let orig_rec = r.information[6] as *mut ExceptionRecord;
    let cont_addr1 = r.information[9];

    let prev_exc = *super::vcruntime140::__current_exception();
    *super::vcruntime140::__current_exception() = orig_rec as *mut c_void;

    let mut cf = CatchFrame {
        this_exc: orig_rec,
        try_low: r.information[3] as i32,
        catch_high: r.information[11] as i32,
        rethrow: false,
        prev_exc,
    };
    let mut hf = crate::unwind::EhHostFrame::new(catch_frame_handler, &mut cf as *mut CatchFrame as *mut c_void);
    let ret = {
        let _frame = hf.push();
        crate::call_guest8_seh(handler_va, handler_va as u64, est, 0x100, 0, 0, 0, 0, 0)
    };

    let cont = if ret < 2 {
        if ret == 0 { cont_addr0 } else { cont_addr1 }
    } else {
        ret
    };

    if !orig_rec.is_null() {
        super::vcruntime140::destruct_exception_object(orig_rec);
    }
    *super::vcruntime140::__current_exception() = prev_exc;
    STATE_OVERRIDE.with(|s| s.set(-2));
    cont
}

pub unsafe extern "win64" fn __NLG_Dispatch2() {}
pub unsafe extern "win64" fn __NLG_Return2() {}

pub unsafe extern "win64" fn __CxxFrameHandler4(
    exc_record: *mut ExceptionRecord,
    establisher_frame: u64,
    context_record: *mut Context,
    dispatcher_context: *mut DispatcherContext,
) -> u32 {
    // `0x180002354`: the state override never outlives one frame handler call.
    let r = frame_handler4(exc_record, establisher_frame, context_record, dispatcher_context);
    STATE_OVERRIDE.with(|s| s.set(-2));
    r
}

/// `__InternalCxxFrameHandler` (`0x1800020c8`).
unsafe fn frame_handler4(
    exc_record: *mut ExceptionRecord,
    establisher_frame: u64,
    context_record: *mut Context,
    dispatcher_context: *mut DispatcherContext,
) -> u32 {
    if exc_record.is_null() || dispatcher_context.is_null() {
        return 1;
    }
    let rec = &mut *exc_record;
    let dc = &mut *dispatcher_context;
    let handler_data = dc.handler_data as *const i32;
    if handler_data.is_null() {
        return 1;
    }
    let fi_rva = std::ptr::read_unaligned(handler_data);
    let fi_ptr = (dc.image_base as isize + fi_rva as isize) as *const u8;
    let fn_begin_rva = if dc.function_entry.is_null() {
        0
    } else {
        (*dc.function_entry).begin_rva
    };
    let fi = decomp_func_info(fi_ptr, dc.image_base, fn_begin_rva);

    // Unwind pass (`(rec.flags & 0x66) != 0`).
    if (rec.flags & UNWIND_MASK) != 0 {
        if fi.disp_unwind_map == 0 {
            return 1;
        }
        let cur_state = current_state(&fi, dc);
        let target_state = if (rec.flags & EXCEPTION_TARGET_UNWIND) != 0 {
            if rec.code == STATUS_UNWIND_CONSOLIDATE {
                rec.information[3] as i32
            } else if rec.code == STATUS_LONGJUMP {
                state_from_ip(&fi, dc, dc.target_ip)
            } else {
                return 1;
            }
        } else {
            -1
        };
        if cur_state >= 0 && cur_state > target_state {
            frame_unwind_to_state(establisher_frame, dc, &fi, cur_state, target_state);
        }
        return 1;
    }

    // Exception search pass.
    let cur_state = current_state(&fi, dc);
    let try_blocks = read_try_blocks(&fi, dc.image_base);
    let est = primary_frame(establisher_frame, &fi);

    for tb in &try_blocks {
        if cur_state < tb.try_low || cur_state > tb.try_high {
            continue;
        }
        let handlers = read_handlers(tb.disp_handler_array, dc.image_base, fn_begin_rva);
        for h in &handlers {
            let matched_ct = if rec.code == CXX_EXCEPTION
                && rec.number_parameters >= 4
                && (0x1993_0520..=0x1993_0522).contains(&rec.information[0])
            {
                let mut active_rec = rec as *const ExceptionRecord;
                if (*active_rec).information[2] == 0 {
                    let prev = *super::vcruntime140::__current_exception() as *const ExceptionRecord;
                    if !prev.is_null() {
                        active_rec = prev;
                    }
                }
                let ti = (*active_rec).information[2] as *const u8;
                let throw_base = (*active_rec).information[3];
                if ti.is_null() {
                    None
                } else if h.disp_type == 0 {
                    Some((std::ptr::null(), active_rec))
                } else {
                    let cta_rva = std::ptr::read_unaligned(ti.add(12) as *const i32);
                    let cta = (throw_base as isize + cta_rva as isize) as *const i32;
                    let n = std::ptr::read_unaligned(cta).max(0) as usize;
                    let mut found = None;
                    for i in 0..n {
                        let ct_rva = std::ptr::read_unaligned(cta.add(1 + i));
                        let ct = (throw_base as isize + ct_rva as isize) as *const u8;
                        if type_match4(h, ct, ti, dc.image_base, throw_base) {
                            found = Some((ct, active_rec));
                            break;
                        }
                    }
                    found
                }
            } else if !fi.is_ehs() && h.disp_type == 0 {
                Some((std::ptr::null(), rec as *const ExceptionRecord))
            } else {
                None
            };

            if let Some((ct, active_rec)) = matched_ct {
                if !ct.is_null() {
                    build_catch_object4(
                        &*active_rec,
                        est,
                        h,
                        ct,
                        dc.image_base,
                        (*active_rec).information[3],
                    );
                }
                let handler_va = (dc.image_base as isize + h.disp_of_handler as isize) as u64;
                let cont0 = if h.cont_addr[0] != 0 {
                    dc.image_base.wrapping_add(h.cont_addr[0])
                } else {
                    0
                };
                let cont1 = if h.cont_addr[1] != 0 {
                    dc.image_base.wrapping_add(h.cont_addr[1])
                } else {
                    0
                };
                let mut est_cell = est;
                let mut cons = ExceptionRecord {
                    code: STATUS_UNWIND_CONSOLIDATE,
                    flags: EXCEPTION_NONCONTINUABLE,
                    record: std::ptr::null_mut(),
                    address: std::ptr::null_mut(),
                    number_parameters: 15,
                    _pad: 0,
                    information: [
                        call_catch_block4 as *const () as usize as u64,
                        &mut est_cell as *mut u64 as u64,
                        handler_va,
                        tb.try_low as u64,
                        context_record as u64,
                        cont0,
                        active_rec as u64,
                        0,
                        0x1993_0520,
                        cont1,
                        dc as *mut DispatcherContext as u64,
                        tb.catch_high as u64,
                        0,
                        0,
                        0,
                    ],
                };
                #[cfg(target_arch = "x86_64")]
                {
                    type RtlUnwindExFn = unsafe extern "win64" fn(
                        u64,
                        u64,
                        *mut ExceptionRecord,
                        u64,
                        *mut Context,
                        *mut c_void,
                    );
                    let unwind: RtlUnwindExFn = std::mem::transmute(RtlUnwindEx as *const ());
                    unwind(
                        establisher_frame,
                        dc.control_pc,
                        &mut cons,
                        0,
                        dc.context_record,
                        dc.history_table,
                    );
                }
                #[cfg(not(target_arch = "x86_64"))]
                {
                    let _ = RtlUnwindEx(
                        establisher_frame,
                        dc.control_pc,
                        &mut cons as *mut ExceptionRecord as u64,
                        0,
                        dc.context_record as u64,
                        dc.history_table as u64,
                    );
                }
                unreachable!("RtlUnwindEx returned");
            }
        }
    }

    if fi.is_noexcept() && !fi.is_catch() && rec.code == CXX_EXCEPTION {
        super::vcruntime140::__std_terminate();
    }

    1
}
