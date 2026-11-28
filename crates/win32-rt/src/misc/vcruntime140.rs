//! `vcruntime140.dll` — MSVC C/C++ runtime primitives, RTTI (`__RTDynamicCast`,
//! `__std_type_info_*`), `std::exception` copy/destroy, `_CxxThrowException`,
//! `__C_specific_handler`, and `__CxxFrameHandler3`.

// @exports: _CxxThrowException, __AdjustPointer, __C_specific_handler
// @exports: __CxxFrameHandler3, __RTDynamicCast, __RTtypeid, __current_exception
// @exports: __current_exception_context, __intrinsic_setjmp, __processing_throw
// @exports: __std_exception_copy, __std_exception_destroy, __std_terminate
// @exports: __std_type_info_compare, __std_type_info_destroy_list, __std_type_info_name
// @exports: __uncaught_exception, __uncaught_exceptions, _purecall
// @exports: _set_purecall_handler, longjmp, memchr, memcmp, memcpy, memmove
// @exports: memset, strchr, strrchr, strstr, wcsrchr, wcsstr

use crate::Export;
use crate::unwind::{
    CXX_EXCEPTION, Context, DispatcherContext, EXCEPTION_EXIT_UNWIND, EXCEPTION_NONCONTINUABLE,
    EXCEPTION_TARGET_UNWIND, EXCEPTION_UNWINDING, ExceptionRecord, RaiseException, RtlPcToFileHeader,
    RtlUnwindEx,
};
use std::ffi::{CStr, c_char, c_void};
use std::sync::atomic::{AtomicUsize, Ordering};

pub static EXTRA_EXPORTS: &[Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

// ---------------------------------------------------------------------------
// Memory and string primitives (`0x180001270..0x18000cb90`)
// ---------------------------------------------------------------------------

#[inline(never)]
pub unsafe extern "win64" fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void {
    // MSVC x64 vcruntime140!memcpy aliases memmove and tolerates overlapping and unaligned ranges;
    // using `ptr::copy` avoids Rust debug-mode `ub_checks` panics on overlapping guest copies.
    if n != 0 && !dst.is_null() && !src.is_null() {
        std::ptr::copy(src.cast::<u8>(), dst.cast::<u8>(), n);
    }
    dst
}

#[inline(never)]
pub unsafe extern "win64" fn memmove(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void {
    if n != 0 && !dst.is_null() && !src.is_null() {
        std::ptr::copy(src.cast::<u8>(), dst.cast::<u8>(), n);
    }
    dst
}

#[inline(never)]
pub unsafe extern "win64" fn memset(dst: *mut c_void, c: i32, n: usize) -> *mut c_void {
    if n != 0 && !dst.is_null() {
        std::ptr::write_bytes(dst.cast::<u8>(), c as u8, n);
    }
    dst
}

#[inline(never)]
pub unsafe extern "win64" fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> i32 {
    if n == 0 || a == b {
        return 0;
    }
    libc::memcmp(a, b, n)
}

#[inline(never)]
pub unsafe extern "win64" fn memchr(s: *const c_void, c: i32, n: usize) -> *mut c_void {
    if n == 0 || s.is_null() {
        return std::ptr::null_mut();
    }
    libc::memchr(s, c, n)
}

pub unsafe extern "win64" fn strchr(s: *const c_char, c: i32) -> *mut c_char {
    if s.is_null() {
        return std::ptr::null_mut();
    }
    libc::strchr(s, c)
}

pub unsafe extern "win64" fn strrchr(s: *const c_char, c: i32) -> *mut c_char {
    if s.is_null() {
        return std::ptr::null_mut();
    }
    libc::strrchr(s, c)
}

pub unsafe extern "win64" fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char {
    if haystack.is_null() || needle.is_null() {
        return std::ptr::null_mut();
    }
    libc::strstr(haystack, needle)
}

pub unsafe extern "win64" fn wcsrchr(s: *const u16, c: u16) -> *mut u16 {
    if s.is_null() {
        return std::ptr::null_mut();
    }
    let mut last: *mut u16 = std::ptr::null_mut();
    let mut p = s;
    loop {
        let ch = std::ptr::read_unaligned(p);
        if ch == c {
            last = p as *mut u16;
        }
        if ch == 0 {
            return last;
        }
        p = p.add(1);
    }
}

pub unsafe extern "win64" fn wcschr(s: *const u16, c: u16) -> *mut u16 {
    if s.is_null() {
        return std::ptr::null_mut();
    }
    let mut p = s;
    loop {
        let ch = std::ptr::read_unaligned(p);
        if ch == c {
            return p as *mut u16;
        }
        if ch == 0 {
            return std::ptr::null_mut();
        }
        p = p.add(1);
    }
}

pub unsafe extern "win64" fn wcsstr(haystack: *const u16, needle: *const u16) -> *mut u16 {
    if haystack.is_null() || needle.is_null() {
        return std::ptr::null_mut();
    }
    if std::ptr::read_unaligned(needle) == 0 {
        return haystack as *mut u16;
    }
    let mut n_len = 0usize;
    while std::ptr::read_unaligned(needle.add(n_len)) != 0 {
        n_len += 1;
    }
    let mut h = haystack;
    while std::ptr::read_unaligned(h) != 0 {
        let mut matched = true;
        for i in 0..n_len {
            let hc = std::ptr::read_unaligned(h.add(i));
            let nc = std::ptr::read_unaligned(needle.add(i));
            if hc != nc {
                matched = false;
                break;
            }
        }
        if matched {
            return h as *mut u16;
        }
        h = h.add(1);
    }
    std::ptr::null_mut()
}

// ---------------------------------------------------------------------------
// RTTI & std::type_info (`0x1800060dc..0x18000c200`)
// ---------------------------------------------------------------------------

/// `__std_type_info_compare` (`0x18000c0b0`): pointer equality fast-path, else
/// compares the decorated names starting at `+9` (after the leading `.` of
/// `_TypeDescriptor.name` at `+8`).
pub unsafe extern "win64" fn __std_type_info_compare(lhs: *const u8, rhs: *const u8) -> i32 {
    if lhs == rhs {
        return 0;
    }
    if lhs.is_null() || rhs.is_null() {
        return if lhs < rhs { -1 } else { 1 };
    }
    libc::strcmp(lhs.add(9) as *const c_char, rhs.add(9) as *const c_char)
}

/// `__std_type_info_hash`: FNV-1a 64-bit hash of the decorated type name (`data + 9`).
pub unsafe extern "win64" fn __std_type_info_hash(data: *const u8) -> usize {
    if data.is_null() {
        return 0;
    }
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    let mut p = data.add(9);
    while *p != 0 {
        hash ^= *p as u64;
        hash = hash.wrapping_mul(0x0100_0000_01b3);
        p = p.add(1);
    }
    (hash ^ (hash >> 32)) as usize
}

/// `__std_type_info_name` (`0x18000c0d4`): returns the cached undecorated name
/// at `*data` if already populated, or the decorated name at `data + 8`.
pub unsafe extern "win64" fn __std_type_info_name(
    data: *mut *const c_char,
    _root_node: *mut c_void,
) -> *const c_char {
    if data.is_null() {
        return c"".as_ptr();
    }
    let cached = std::ptr::read_unaligned(data);
    if !cached.is_null() {
        return cached;
    }
    let decorated = (data as *const c_char).add(8);
    std::ptr::write_unaligned(data, decorated);
    decorated
}

/// `__std_type_info_destroy_list` (`0x18000c194`).
pub unsafe extern "win64" fn __std_type_info_destroy_list(root_node: *mut c_void) {
    if !root_node.is_null() {
        crate::sync::InterlockedFlushSList(root_node.cast());
    }
}

/// `__AdjustPointer` (`0x180007290`): applies a `PMD { mdisp, pdisp, vdisp }`
/// member/virtual-base displacement to `obj`.
pub unsafe extern "win64" fn __AdjustPointer(obj: *mut u8, pmd: *const [i32; 3]) -> *mut u8 {
    if obj.is_null() || pmd.is_null() {
        return obj;
    }
    let [mdisp, pdisp, vdisp] = std::ptr::read_unaligned(pmd);
    let mut ret = obj.wrapping_offset(mdisp as isize);
    if pdisp >= 0 {
        let vbptr = std::ptr::read_unaligned(obj.wrapping_offset(pdisp as isize) as *const *const u8);
        if !vbptr.is_null() {
            let voff = std::ptr::read_unaligned(vbptr.wrapping_offset(vdisp as isize) as *const i32);
            ret = ret.wrapping_offset((pdisp as isize) + (voff as isize));
        }
    }
    ret
}

#[repr(C)]
#[derive(Clone, Copy)]
struct RttiCompleteObjectLocator {
    signature: u32,
    offset: u32,
    cd_offset: u32,
    p_type_descriptor: i32,
    p_class_descriptor: i32,
    p_self: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct RttiClassHierarchyDescriptor {
    signature: u32,
    attributes: u32,
    num_base_classes: u32,
    p_base_class_array: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct RttiBaseClassDescriptor {
    p_type_descriptor: i32,
    num_contained_bases: u32,
    where_pmd: [i32; 3],
    attributes: u32,
    p_class_descriptor: i32,
}

#[inline]
unsafe fn rtti_types_equal(td_a: *const u8, td_b: *const u8) -> bool {
    if td_a == td_b {
        return true;
    }
    if td_a.is_null() || td_b.is_null() {
        return false;
    }
    let name_a = CStr::from_ptr(td_a.add(16) as *const c_char).to_bytes();
    let name_b = CStr::from_ptr(td_b.add(16) as *const c_char).to_bytes();
    name_a == name_b
}

/// `__RTDynamicCast` (`0x1800060dc`).
pub unsafe extern "win64" fn __RTDynamicCast(
    inptr: *mut u8,
    vfdelta: i32,
    src_type: *const u8,
    dst_type: *const u8,
    _is_reference: i32,
) -> *mut u8 {
    if inptr.is_null() {
        return std::ptr::null_mut();
    }
    let vtable = std::ptr::read_unaligned(inptr as *const *const usize);
    if vtable.is_null() {
        return std::ptr::null_mut();
    }
    let col_ptr = std::ptr::read_unaligned(vtable.sub(1)) as *const RttiCompleteObjectLocator;
    if col_ptr.is_null() {
        return std::ptr::null_mut();
    }
    let col = std::ptr::read_unaligned(col_ptr);
    let adj_ptr = inptr.wrapping_offset(-(vfdelta as isize));
    let complete_obj = adj_ptr
        .wrapping_sub(col.offset as usize)
        .wrapping_sub(if col.cd_offset != 0 {
            let cd_disp = std::ptr::read_unaligned(
                adj_ptr.wrapping_sub(col.cd_offset as usize) as *const i32,
            );
            cd_disp as isize as usize
        } else {
            0
        });

    let image_base = if col.signature == 0 {
        let mut b: *mut c_void = std::ptr::null_mut();
        RtlPcToFileHeader(col_ptr.cast(), &mut b);
        b as usize
    } else {
        (col_ptr as usize).wrapping_sub(col.p_self as isize as usize)
    };
    if image_base == 0 {
        return std::ptr::null_mut();
    }

    let chd_ptr = (image_base as isize + col.p_class_descriptor as isize)
        as *const RttiClassHierarchyDescriptor;
    let chd = std::ptr::read_unaligned(chd_ptr);
    let bca_ptr = (image_base as isize + chd.p_base_class_array as isize) as *const i32;
    let num_bases = chd.num_base_classes as usize;

    let _ = src_type;
    for i in 0..num_bases {
        let bcd_rva = std::ptr::read_unaligned(bca_ptr.add(i));
        let bcd_ptr =
            (image_base as isize + bcd_rva as isize) as *const RttiBaseClassDescriptor;
        let bcd = std::ptr::read_unaligned(bcd_ptr);
        if (bcd.attributes & 0x03) != 0 {
            continue;
        }
        let bcd_td = (image_base as isize + bcd.p_type_descriptor as isize) as *const u8;
        if rtti_types_equal(bcd_td, dst_type) {
            return __AdjustPointer(complete_obj, &bcd.where_pmd);
        }
    }
    std::ptr::null_mut()
}

/// `__RTtypeid`: returns the `TypeDescriptor*` (`std::type_info*`) for polymorphic object `inptr`.
pub unsafe extern "win64" fn __RTtypeid(inptr: *mut u8) -> *const u8 {
    if inptr.is_null() {
        log::error!("__RTtypeid(NULL)");
        _CxxThrowException(std::ptr::null_mut(), std::ptr::null());
        return std::ptr::null();
    }
    let vtable = std::ptr::read_unaligned(inptr as *const *const usize);
    if vtable.is_null() {
        return std::ptr::null();
    }
    let col_ptr = std::ptr::read_unaligned(vtable.sub(1)) as *const RttiCompleteObjectLocator;
    if col_ptr.is_null() {
        return std::ptr::null();
    }
    let col = std::ptr::read_unaligned(col_ptr);
    let image_base = if col.signature == 0 {
        let mut b: *mut c_void = std::ptr::null_mut();
        RtlPcToFileHeader(col_ptr.cast(), &mut b);
        b as usize
    } else {
        (col_ptr as usize).wrapping_sub(col.p_self as isize as usize)
    };
    if image_base == 0 {
        return std::ptr::null();
    }
    (image_base as isize + col.p_type_descriptor as isize) as *const u8
}

// ---------------------------------------------------------------------------
// std::exception helpers & C++ exception state (`0x180006820..0x180007290`)
// ---------------------------------------------------------------------------

#[repr(C)]
pub struct StdExceptionData {
    pub what: *const c_char,
    pub do_free: u8,
}

/// `__std_exception_copy` (`0x180006820`).
pub unsafe extern "win64" fn __std_exception_copy(
    from: *const StdExceptionData,
    to: *mut StdExceptionData,
) {
    if from.is_null() || to.is_null() {
        return;
    }
    if (*from).do_free == 0 || (*from).what.is_null() {
        (*to).what = (*from).what;
        (*to).do_free = 0;
        return;
    }
    let len = libc::strlen((*from).what) + 1;
    let copy = crate::ucrt::heap::malloc(len) as *mut c_char;
    if !copy.is_null() {
        std::ptr::copy_nonoverlapping((*from).what, copy, len);
        (*to).what = copy;
        (*to).do_free = 1;
    } else {
        (*to).what = std::ptr::null();
        (*to).do_free = 0;
    }
}

/// `__std_exception_destroy` (`0x180006880`).
pub unsafe extern "win64" fn __std_exception_destroy(data: *mut StdExceptionData) {
    if data.is_null() {
        return;
    }
    if (*data).do_free != 0 && !(*data).what.is_null() {
        crate::ucrt::heap::free((*data).what as *mut c_void);
    }
    (*data).what = std::ptr::null();
    (*data).do_free = 0;
}

#[repr(C, align(16))]
pub struct VcrtPtd {
    pub raw: [u64; 16],
}

thread_local! {
    /// Contiguous per-thread `vcruntime140` PTD (`0x180003524`):
    /// * `+0x20` (`raw[4]`): `_curexception` (`__current_exception()`)
    /// * `+0x28` (`raw[5]`): `_curcontext` (`__current_exception_context()`)
    /// * `+0x30` (`raw[6]`): `_ProcessingThrow` (`__processing_throw()` / `__uncaught_exceptions()`)
    /// * `+0x58` (`raw[11]`): `_pFrameInfoChain`
    /// * `+0x60` (`raw[12]`): `_ThrowImageBase`
    /// * `+0x68` (`raw[13]`): `_pForeignException`
    static VCRT_PTD: std::cell::UnsafeCell<VcrtPtd> = const {
        std::cell::UnsafeCell::new(VcrtPtd { raw: [0; 16] })
    };
}

#[inline]
pub(crate) fn ptd_ptr() -> *mut u8 {
    VCRT_PTD.with(|c| c.get() as *mut u8)
}

pub(crate) fn inc_uncaught_exceptions() {
    unsafe {
        let p = __processing_throw();
        *p = (*p).wrapping_add(1);
    }
}

pub(crate) fn dec_uncaught_exceptions() {
    unsafe {
        let p = __processing_throw();
        *p = (*p - 1).max(0);
    }
}

pub unsafe extern "win64" fn __current_exception() -> *mut *mut c_void {
    ptd_ptr().add(0x20) as *mut *mut c_void
}

pub unsafe extern "win64" fn __current_exception_context() -> *mut *mut c_void {
    ptd_ptr().add(0x28) as *mut *mut c_void
}

pub unsafe extern "win64" fn __processing_throw() -> *mut i32 {
    ptd_ptr().add(0x30) as *mut i32
}

pub unsafe extern "win64" fn __uncaught_exceptions() -> i32 {
    *__processing_throw()
}

pub unsafe extern "win64" fn __uncaught_exception() -> u8 {
    (*__processing_throw() > 0) as u8
}

pub unsafe extern "win64" fn __std_terminate() -> ! {
    crate::ucrt::runtime::terminate()
}

static PURECALL_HANDLER: AtomicUsize = AtomicUsize::new(0);

pub unsafe extern "win64" fn _set_purecall_handler(handler: usize) -> usize {
    PURECALL_HANDLER.swap(handler, Ordering::AcqRel)
}

pub unsafe extern "win64" fn _purecall() -> ! {
    let h = PURECALL_HANDLER.load(Ordering::Acquire);
    if h != 0 {
        let _ = crate::call_guest8(h, 0, 0, 0, 0, 0, 0, 0, 0);
    }
    crate::ucrt::runtime::abort()
}

context_capturing_export!(__intrinsic_setjmp => crate::unwind::setjmp_impl);
context_capturing_export!(longjmp => crate::unwind::longjmp_impl);

/// `__DestructExceptionObject` (`0x1800011a0`).
pub(crate) unsafe fn destruct_exception_object(rec: *const ExceptionRecord) {
    if rec.is_null() {
        return;
    }
    let r = &*rec;
    if r.code != CXX_EXCEPTION
        || r.number_parameters < 4
        || !(0x1993_0520..=0x1993_0522).contains(&r.information[0])
    {
        return;
    }
    let obj = r.information[1];
    let ti = r.information[2] as *const u8;
    let image_base = r.information[3];
    if obj == 0 || ti.is_null() {
        return;
    }
    let dtor_rva = std::ptr::read_unaligned(ti.add(4) as *const i32);
    if dtor_rva != 0 {
        let dtor_va = (image_base as isize + dtor_rva as isize) as usize;
        let _ = crate::call_guest8(dtor_va, obj, 0, 0, 0, 0, 0, 0, 0);
    }
}

/// Raise an MSVC x64 C++ exception with an explicit `throw_image_base` (used by
/// builtin runtime throw helpers whose `ThrowInfo` slab lives in `win32-rt` static data
/// rather than a loaded guest PE image).
pub(crate) unsafe fn throw_cxx_exception_with_base(
    exception_object: *mut c_void,
    throw_info: *const u8,
    image_base: usize,
) {
    let args: [u64; 4] = [
        0x1993_0520,
        exception_object as u64,
        throw_info as u64,
        image_base as u64,
    ];
    #[cfg(target_arch = "x86_64")]
    {
        type RaiseExceptionFn = unsafe extern "win64" fn(u32, u32, u32, *const u64);
        let raise: RaiseExceptionFn = std::mem::transmute(RaiseException as *const ());
        raise(CXX_EXCEPTION, EXCEPTION_NONCONTINUABLE, 4, args.as_ptr());
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = RaiseException(
            CXX_EXCEPTION as u64,
            EXCEPTION_NONCONTINUABLE as u64,
            4,
            args.as_ptr() as u64,
            0,
            0,
        );
    }
}

/// `_CxxThrowException` (`0x180006b10`).
pub unsafe extern "win64" fn _CxxThrowException(
    exception_object: *mut c_void,
    throw_info: *const u8,
) {
    let mut magic: u64 = 0x1993_0520;
    let mut image_base: *mut c_void = std::ptr::null_mut();
    if !throw_info.is_null() {
        RtlPcToFileHeader(throw_info.cast(), &mut image_base);
        let attrs = std::ptr::read_unaligned(throw_info as *const u32);
        if (attrs & 0x08) != 0 || image_base.is_null() {
            magic = 0x0199_4000;
        }
    }
    let args: [u64; 4] = [
        magic,
        exception_object as u64,
        throw_info as u64,
        image_base as u64,
    ];
    #[cfg(target_arch = "x86_64")]
    {
        type RaiseExceptionFn = unsafe extern "win64" fn(u32, u32, u32, *const u64);
        let raise: RaiseExceptionFn = std::mem::transmute(RaiseException as *const ());
        raise(CXX_EXCEPTION, EXCEPTION_NONCONTINUABLE, 4, args.as_ptr());
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = RaiseException(
            CXX_EXCEPTION as u64,
            EXCEPTION_NONCONTINUABLE as u64,
            4,
            args.as_ptr() as u64,
            0,
            0,
        );
    }
}

// ---------------------------------------------------------------------------
// `__C_specific_handler` (`0x18000f6a0`) and `__CxxFrameHandler3`
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy)]
struct ScopeRecord {
    begin_addr: u32,
    end_addr: u32,
    handler_addr: u32,
    jump_target: u32,
}

const UNWIND_MASK: u32 = EXCEPTION_UNWINDING
    | EXCEPTION_EXIT_UNWIND
    | EXCEPTION_TARGET_UNWIND
    | crate::unwind::EXCEPTION_COLLIDED_UNWIND;

/// `__C_specific_handler` (`0x18000f6a0`).
pub unsafe extern "win64" fn __C_specific_handler(
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
    let scope_table = dc.handler_data as *const u32;
    if scope_table.is_null() {
        return 1;
    }
    let count = std::ptr::read_unaligned(scope_table) as usize;
    let records = scope_table.add(1) as *const ScopeRecord;
    let image_base = dc.image_base;
    let control_rva = dc.control_pc.wrapping_sub(image_base);

    if (rec.flags & UNWIND_MASK) == 0 {
        let mut exc_ptrs: [*mut c_void; 2] = [exc_record.cast(), context_record.cast()];
        let mut idx = dc.scope_index as usize;
        while idx < count {
            let sr = std::ptr::read_unaligned(records.add(idx));
            if control_rva >= sr.begin_addr as u64
                && control_rva < sr.end_addr as u64
                && sr.jump_target != 0
            {
                let disp = if sr.handler_addr == 1 {
                    1
                } else {
                    let filter_va = (image_base + sr.handler_addr as u64) as usize;
                    crate::call_guest8(
                        filter_va,
                        &mut exc_ptrs as *mut _ as u64,
                        establisher_frame,
                        0, 0, 0, 0, 0, 0,
                    ) as i32
                };
                if disp < 0 {
                    return 0; // ExceptionContinueExecution
                }
                if disp > 0 {
                    if rec.code == CXX_EXCEPTION {
                        destruct_exception_object(exc_record);
                    }
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
                            image_base + sr.jump_target as u64,
                            exc_record,
                            rec.code as u64,
                            dc.context_record,
                            dc.history_table,
                        );
                    }
                    #[cfg(not(target_arch = "x86_64"))]
                    {
                        let _ = RtlUnwindEx(
                            establisher_frame,
                            image_base + sr.jump_target as u64,
                            exc_record as u64,
                            rec.code as u64,
                            dc.context_record as u64,
                            dc.history_table as u64,
                        );
                    }
                    unreachable!("RtlUnwindEx returned");
                }
            }
            idx += 1;
        }
    } else {
        let target_rva = dc.target_ip.wrapping_sub(image_base);
        let mut idx = dc.scope_index as usize;
        while idx < count {
            let sr = std::ptr::read_unaligned(records.add(idx));
            if control_rva >= sr.begin_addr as u64 && control_rva < sr.end_addr as u64 {
                if (rec.flags & EXCEPTION_TARGET_UNWIND) != 0 {
                    for k in 0..count {
                        let inner = std::ptr::read_unaligned(records.add(k));
                        if target_rva >= inner.begin_addr as u64
                            && target_rva < inner.end_addr as u64
                            && inner.jump_target == sr.jump_target
                            && inner.handler_addr == sr.handler_addr
                        {
                            return 1;
                        }
                    }
                }
                if sr.jump_target != 0 {
                    if target_rva == sr.jump_target as u64
                        && (rec.flags & EXCEPTION_TARGET_UNWIND) != 0
                    {
                        return 1;
                    }
                } else {
                    dc.scope_index = (idx + 1) as u32;
                    let term_va = (image_base + sr.handler_addr as u64) as usize;
                    let _ = crate::call_guest8(term_va, 1, establisher_frame, 0, 0, 0, 0, 0, 0);
                }
            }
            idx += 1;
        }
    }
    1
}

/// `__CxxFrameHandler3` (`0x1800010f0`).
pub unsafe extern "win64" fn __CxxFrameHandler3(
    exc_record: *mut ExceptionRecord,
    establisher_frame: u64,
    _context_record: *mut Context,
    dispatcher_context: *mut DispatcherContext,
) -> u32 {
    if exc_record.is_null() || dispatcher_context.is_null() {
        return 1;
    }
    let rec = &*exc_record;
    let dc = &mut *dispatcher_context;
    if (rec.flags & UNWIND_MASK) != 0 && !dc.handler_data.is_null() {
        // FH3 FuncInfo { magicNumber, maxState, dispUnwindMap, nTryBlocks, dispTryBlockMap, nIPMapEntries, dispIPtoStateMap, dispUwindHelp }
        let fi_rva = std::ptr::read_unaligned(dc.handler_data as *const i32);
        let fi = (dc.image_base as isize + fi_rva as isize) as *const i32;
        let max_state = std::ptr::read_unaligned(fi.add(1));
        let disp_unwind_map = std::ptr::read_unaligned(fi.add(2));
        let n_ip = std::ptr::read_unaligned(fi.add(5));
        let disp_ip_map = std::ptr::read_unaligned(fi.add(6));
        if max_state > 0 && disp_unwind_map != 0 && n_ip > 0 && disp_ip_map != 0 {
            let control_rva = dc.control_pc.wrapping_sub(dc.image_base) as i32;
            let ip_map = (dc.image_base as isize + disp_ip_map as isize) as *const [i32; 2];
            let mut cur_state = -1i32;
            for i in 0..n_ip as usize {
                let [ip_rva, st] = std::ptr::read_unaligned(ip_map.add(i));
                if control_rva < ip_rva {
                    break;
                }
                cur_state = st;
            }
            let uw_map = (dc.image_base as isize + disp_unwind_map as isize) as *const [i32; 2];
            while cur_state >= 0 && cur_state < max_state {
                let [to_state, action_rva] = std::ptr::read_unaligned(uw_map.add(cur_state as usize));
                cur_state = to_state;
                if action_rva != 0 {
                    let action_va = (dc.image_base as isize + action_rva as isize) as usize;
                    let _ = crate::call_guest8(action_va, action_va as u64, establisher_frame, 0, 0, 0, 0, 0, 0);
                }
            }
        }
    }
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vcruntime_strrchr_purecall_rtti_and_setjmp_exports() {
        for sym in [
            "strrchr",
            "_set_purecall_handler",
            "__RTtypeid",
            "__intrinsic_setjmp",
            "longjmp",
        ] {
            assert!(
                crate::lookup("vcruntime140.dll", sym).is_some(),
                "missing vcruntime140!{sym}"
            );
        }

        unsafe {
            // strrchr
            let s = c"ab/cd/ef";
            let hit = strrchr(s.as_ptr(), b'/' as i32);
            assert_eq!(hit, s.as_ptr().add(5) as *mut c_char);
            assert!(strrchr(s.as_ptr(), b'z' as i32).is_null());
            assert!(strrchr(std::ptr::null(), b'a' as i32).is_null());

            // _set_purecall_handler swaps atomically
            let prev = _set_purecall_handler(0x1234_5678);
            assert_eq!(_set_purecall_handler(prev), 0x1234_5678);

            // __RTtypeid resolves TypeDescriptor from vtable[-1] CompleteObjectLocator (signature=1)
            #[repr(C)]
            struct FakeRttiModule {
                td: [u8; 32],
                col: RttiCompleteObjectLocator,
                vtable_with_meta: [usize; 2],
            }
            let mut m = FakeRttiModule {
                td: [0; 32],
                col: RttiCompleteObjectLocator {
                    signature: 1,
                    offset: 0,
                    cd_offset: 0,
                    p_type_descriptor: 0,
                    p_class_descriptor: 0,
                    p_self: 32,
                },
                vtable_with_meta: [0, 0],
            };
            m.vtable_with_meta[0] = &m.col as *const RttiCompleteObjectLocator as usize;
            let mut obj_vptr = &m.vtable_with_meta[1] as *const usize;
            let got = __RTtypeid(&mut obj_vptr as *mut *const usize as *mut u8);
            assert_eq!(got, m.td.as_ptr());

            // setjmp_impl populates _JUMP_BUFFER fields faithfully from Context
            let mut ctx = Context::zeroed();
            let mut jb = [0u8; 256];
            ctx.gpr[crate::unwind::RCX] = jb.as_mut_ptr() as u64;
            ctx.gpr[crate::unwind::RDX] = 0x7000_0000;
            ctx.gpr[crate::unwind::RBX] = 0x1111;
            ctx.gpr[crate::unwind::RSP] = 0x2222;
            ctx.gpr[crate::unwind::RBP] = 0x3333;
            ctx.gpr[crate::unwind::RSI] = 0x4444;
            ctx.gpr[crate::unwind::RDI] = 0x5555;
            ctx.gpr[12] = 0x6666;
            ctx.gpr[13] = 0x7777;
            ctx.gpr[14] = 0x8888;
            ctx.gpr[15] = 0x9999;
            ctx.rip = 0x1400_1234;
            ctx.mx_csr = 0x1F80;
            *ctx.xmm_mut(6) = [6u8; 16];
            *ctx.xmm_mut(15) = [15u8; 16];
            assert_eq!(crate::unwind::setjmp_impl(&mut ctx, 0), 0);
            assert_eq!(std::ptr::read_unaligned(jb.as_ptr() as *const u64), 0x7000_0000);
            assert_eq!(std::ptr::read_unaligned(jb.as_ptr().add(0x08) as *const u64), 0x1111);
            assert_eq!(std::ptr::read_unaligned(jb.as_ptr().add(0x10) as *const u64), 0x2222);
            assert_eq!(std::ptr::read_unaligned(jb.as_ptr().add(0x50) as *const u64), 0x1400_1234);
            assert_eq!(std::ptr::read_unaligned(jb.as_ptr().add(0x58) as *const u32), 0x1F80);
            assert_eq!(&jb[0x60..0x70], &[6u8; 16]);
            assert_eq!(&jb[0xF0..0x100], &[15u8; 16]);

            // memcpy must tolerate overlapping (forward, backward, self) and unaligned guest pointers.
            let mut ov = *b"0123456789ABCDEF";
            assert_eq!(memcpy(ov.as_mut_ptr().add(3).cast(), ov.as_ptr().add(1).cast(), 8), ov.as_mut_ptr().add(3).cast());
            assert_eq!(&ov, b"01212345678BCDEF");
            assert_eq!(memcpy(ov.as_mut_ptr().add(1).cast(), ov.as_ptr().add(3).cast(), 8), ov.as_mut_ptr().add(1).cast());
            assert_eq!(&ov, b"01234567878BCDEF");
            assert_eq!(memcpy(ov.as_mut_ptr().add(1).cast(), ov.as_ptr().add(1).cast(), 6), ov.as_mut_ptr().add(1).cast());
            assert_eq!(&ov, b"01234567878BCDEF");
        }
    }
}
