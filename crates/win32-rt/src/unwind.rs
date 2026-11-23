//! x64 structured exception handling: CONTEXT capture/restore, the table
//! based unwinder (`RtlVirtualUnwind`, ported from Wine's algorithm), and
//! exception dispatch / unwind (`RaiseException`, `RtlUnwindEx`).
//!
//! The real `__C_specific_handler` / `__CxxFrameHandler4` live in the bundled
//! vcruntime140(_1).dll and call back into these.
//!
//! Host (Rust) frames have no `.pdata`. When a walk reaches host code, [`step`]
//! crosses it in one of two ways (docs/design/ADVICE_SEH.md):
//! * **Dispatcher frames**: a language handler (e.g. vcruntime's
//!   `__CxxFrameHandler4`) was called by [`dispatch_exception`] and calls
//!   `RtlUnwindEx`. The walk resumes at the raise-site context of that dispatch,
//!   just as Windows unwinds through `RtlpExecuteHandlerForException` back into
//!   the raising frame.
//! * **Fence frames**: a Rust replacement entered through the loader's fat hook
//!   shim. The walk follows the host frame-pointer chain (`force-frame-pointers`)
//!   to the shim's registered return address ([`register_fence`]). The shim's
//!   unwind info restores every Win64 nonvolatile. The Rust frames in between are
//!   discarded like a `longjmp`, so they must not hold live `Drop` values.
//!
//! Otherwise the walk stops and the exception is treated as unhandled.

use pe_image::RuntimeFunction;
use std::ffi::c_void;
use std::sync::Mutex;

pub const CONTEXT_SIZE: usize = 0x4D0;
pub const CONTEXT_AMD64: u32 = 0x0010_0000;
pub const CONTEXT_CONTROL: u32 = CONTEXT_AMD64 | 1;
pub const CONTEXT_INTEGER: u32 = CONTEXT_AMD64 | 2;
pub const CONTEXT_SEGMENTS: u32 = CONTEXT_AMD64 | 4;
pub const CONTEXT_FLOATING_POINT: u32 = CONTEXT_AMD64 | 8;
pub const CONTEXT_FULL: u32 = CONTEXT_CONTROL | CONTEXT_INTEGER | CONTEXT_FLOATING_POINT;

/// x64 `CONTEXT` (0x4D0 bytes, 16-byte aligned).
#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct Context {
    pub p_home: [u64; 6],      // 0x00
    pub context_flags: u32,    // 0x30
    pub mx_csr: u32,           // 0x34
    pub seg_cs: u16,           // 0x38
    pub seg_ds: u16,
    pub seg_es: u16,
    pub seg_fs: u16,
    pub seg_gs: u16,
    pub seg_ss: u16,
    pub eflags: u32,           // 0x44
    pub dr: [u64; 6],          // 0x48
    /// rax rcx rdx rbx rsp rbp rsi rdi r8..r15 (unwind register numbering)
    pub gpr: [u64; 16],        // 0x78
    pub rip: u64,              // 0xF8
    pub flt_save: [u8; 512],   // 0x100 (XMM_SAVE_AREA32; xmm0 at +0xA0)
    pub vector_register: [[u64; 2]; 26], // 0x300
    pub vector_control: u64,   // 0x4A0
    pub debug_control: u64,
    pub last_branch_to_rip: u64,
    pub last_branch_from_rip: u64,
    pub last_exception_to_rip: u64,
    pub last_exception_from_rip: u64,
}
const _: () = assert!(std::mem::size_of::<Context>() == CONTEXT_SIZE);

pub const RAX: usize = 0;
pub const RCX: usize = 1;
pub const RDX: usize = 2;
pub const RBX: usize = 3;
pub const RSP: usize = 4;
pub const RBP: usize = 5;
pub const RSI: usize = 6;
pub const RDI: usize = 7;
pub const R8: usize = 8;
pub const R9: usize = 9;
pub const R10: usize = 10;
pub const R11: usize = 11;

impl Context {
    pub fn zeroed() -> Self {
        unsafe { std::mem::zeroed() }
    }
    pub fn rsp(&self) -> u64 {
        self.gpr[RSP]
    }
    pub fn xmm_mut(&mut self, i: usize) -> &mut [u8; 16] {
        let o = 0xA0 + i * 16;
        (&mut self.flt_save[o..o + 16]).try_into().unwrap()
    }
    pub fn xmm(&self, i: usize) -> [u8; 16] {
        let o = 0xA0 + i * 16;
        self.flt_save[o..o + 16].try_into().unwrap()
    }
}

/// `EXCEPTION_RECORD` (x64).
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ExceptionRecord {
    pub code: u32,
    pub flags: u32,
    pub record: *mut ExceptionRecord,
    pub address: *mut c_void,
    pub number_parameters: u32,
    pub _pad: u32,
    pub information: [u64; 15],
}

pub const EXCEPTION_NONCONTINUABLE: u32 = 0x1;
pub const EXCEPTION_UNWINDING: u32 = 0x2;
pub const EXCEPTION_EXIT_UNWIND: u32 = 0x4;
pub const EXCEPTION_STACK_INVALID: u32 = 0x8;
pub const EXCEPTION_NESTED_CALL: u32 = 0x10;
pub const EXCEPTION_TARGET_UNWIND: u32 = 0x20;
pub const EXCEPTION_COLLIDED_UNWIND: u32 = 0x40;

pub const STATUS_UNWIND: u32 = 0xC000_0027;
/// `longjmp`'s unwind (`ExceptionInformation[0]` = the `_JUMP_BUFFER`): at the target frame
/// the context's nonvolatiles come from the jump buffer (vcruntime `longjmp` is an
/// `RtlUnwind` to the buffer's `Frame` with this code; see `do_unwind`).
pub const STATUS_LONGJMP: u32 = 0x8000_0026;
/// `Context::context_flags` bit (not a Windows flag): the context's nonvolatile registers
/// were restored from a `_JUMP_BUFFER` by a `longjmp` unwind, so a landing must apply them
/// (an exception catch keeps the frame's own values instead).
pub const CONTEXT_AW2_LONGJMP: u32 = 0x2000_0000;
pub const STATUS_NONCONTINUABLE_EXCEPTION: u32 = 0xC000_0025;
pub const STATUS_INVALID_DISPOSITION: u32 = 0xC000_0026;
pub const STATUS_ACCESS_VIOLATION: u32 = 0xC000_0005;
pub const STATUS_ILLEGAL_INSTRUCTION: u32 = 0xC000_001D;
pub const STATUS_INTEGER_DIVIDE_BY_ZERO: u32 = 0xC000_0094;
pub const STATUS_BREAKPOINT: u32 = 0x8000_0003;
pub const STATUS_STACK_BUFFER_OVERRUN: u32 = 0xC000_0409;
pub const MS_VC_EXCEPTION_SET_THREAD_NAME: u32 = 0x406D_1388;
pub const CXX_EXCEPTION: u32 = 0xE06D_7363;
pub const DBG_PRINTEXCEPTION_C: u32 = 0x4001_0006;
pub const DBG_PRINTEXCEPTION_WIDE_C: u32 = 0x4001_000A;

pub const UNW_FLAG_NHANDLER: u32 = 0;
pub const UNW_FLAG_EHANDLER: u32 = 1;
pub const UNW_FLAG_UHANDLER: u32 = 2;
pub const UNW_FLAG_CHAININFO: u32 = 4;

/// `DISPATCHER_CONTEXT` (x64).
#[repr(C)]
pub struct DispatcherContext {
    pub control_pc: u64,
    pub image_base: u64,
    pub function_entry: *const RuntimeFunction,
    pub establisher_frame: u64,
    pub target_ip: u64,
    pub context_record: *mut Context,
    pub language_handler: *const c_void,
    pub handler_data: *mut c_void,
    pub history_table: *mut c_void,
    pub scope_index: u32,
    pub fill0: u32,
}

pub type LanguageHandler =
    unsafe extern "win64" fn(*mut ExceptionRecord, u64, *mut Context, *mut DispatcherContext) -> u32;
const EXCEPTION_CONTINUE_EXECUTION_DISP: u32 = 0; // ExceptionContinueExecution
const EXCEPTION_CONTINUE_SEARCH_DISP: u32 = 1;
const EXCEPTION_NESTED_EXCEPTION_DISP: u32 = 2;
const EXCEPTION_COLLIDED_UNWIND_DISP: u32 = 3;

// ---------------------------------------------------------------------------
// Context capture / restore (naked asm)
// ---------------------------------------------------------------------------

/// `RtlCaptureContext(CONTEXT*)`: context of the caller as if this call
/// returned (rip = return address, rsp = rsp after return).
#[cfg(target_arch = "x86_64")]
#[unsafe(naked)]
pub unsafe extern "win64" fn RtlCaptureContext(_ctx: *mut Context) {
    std::arch::naked_asm!(
        "mov [rcx+0x78], rax",
        "mov [rcx+0x80], rcx",
        "mov [rcx+0x88], rdx",
        "mov [rcx+0x90], rbx",
        "mov [rcx+0xA0], rbp",
        "mov [rcx+0xA8], rsi",
        "mov [rcx+0xB0], rdi",
        "mov [rcx+0xB8], r8",
        "mov [rcx+0xC0], r9",
        "mov [rcx+0xC8], r10",
        "mov [rcx+0xD0], r11",
        "mov [rcx+0xD8], r12",
        "mov [rcx+0xE0], r13",
        "mov [rcx+0xE8], r14",
        "mov [rcx+0xF0], r15",
        "lea rax, [rsp+8]",
        "mov [rcx+0x98], rax",
        "mov rax, [rsp]",
        "mov [rcx+0xF8], rax",
        "pushfq",
        "pop rax",
        "mov [rcx+0x44], eax",
        "mov word ptr [rcx+0x38], cs",
        "mov word ptr [rcx+0x3A], ds",
        "mov word ptr [rcx+0x3C], es",
        "mov word ptr [rcx+0x3E], fs",
        "mov word ptr [rcx+0x40], gs",
        "mov word ptr [rcx+0x42], ss",
        "fxsave [rcx+0x100]",
        "stmxcsr [rcx+0x34]",
        "mov dword ptr [rcx+0x30], 0x10000F",
        "mov rax, [rcx+0x78]",
        "ret",
    )
}

#[cfg(all(target_arch = "aarch64", not(target_arch = "x86_64")))]
core::arch::global_asm!(
    ".global _capture_nv_x19_x28",
    ".p2align 2",
    "_capture_nv_x19_x28:",
    "stp x19, x20, [x0, #0x00]",
    "stp x21, x22, [x0, #0x10]",
    "stp x23, x24, [x0, #0x20]",
    "stp x25, x26, [x0, #0x30]",
    "stp x27, x28, [x0, #0x40]",
    "ret",
);

#[cfg(target_arch = "aarch64")]
unsafe extern "C" {
    fn capture_nv_x19_x28(out: *mut u64);
}

#[cfg(not(target_arch = "x86_64"))]
#[inline(never)]
pub unsafe extern "win64" fn RtlCaptureContext(ctx: *mut Context) {
    // Called directly by guest code: the calling guest frame's context.
    if guest_caller_context(RtlCaptureContext as *const () as usize, &mut *ctx).is_some() {
        return;
    }
    let mut fp: u64 = 0;
    let mut lr: u64 = 0;
    let mut sp: u64 = 0;
    #[cfg(target_arch = "aarch64")]
    std::arch::asm!(
        "mov {fp}, x29",
        "mov {lr}, x30",
        "mov {sp}, sp",
        fp = out(reg) fp,
        lr = out(reg) lr,
        sp = out(reg) sp,
        options(nomem, nostack, preserves_flags),
    );
    // Arm64 callee-saved registers of the caller. The x64 version captures
    // rbx/rbp/rsi/rdi/r12-r15 into `gpr`; the arm64 equivalents (x19..x28) have
    // no slots in the x64 CONTEXT layout, so they ride in the otherwise-unused
    // `vector_register` area. `restore_context` reloads them so a resumed
    // host frame keeps its compiler-cached pointers (spill slots, TLS cells)
    // instead of whatever the exception machinery left in those registers.
    let mut nv = [0u64; 10];
    #[cfg(target_arch = "aarch64")]
    {
        // LLVM refuses `x19` as an explicit inline-asm operand, so the reads
        // live in a `global_asm!` helper.
        capture_nv_x19_x28(nv.as_mut_ptr());
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let _ = (&mut nv, &mut fp, &mut lr, &mut sp);
    }
    *ctx = Context::zeroed();
    (*ctx).context_flags = 0x10000F;
    (*ctx).mx_csr = 0x1F80;
    for (i, v) in nv.iter().enumerate() {
        (*ctx).vector_register[i / 2][i % 2] = *v;
    }
    // Note: in this non-naked function on aarch64, x29 points at our frame
    // record `[caller_fp, caller_lr]`, so caller_fp is `[fp]` and caller_lr is `[fp + 8]`.
    if fp != 0 && (fp & 7) == 0 {
        let caller_fp = std::ptr::read_unaligned(fp as *const u64);
        let caller_lr = std::ptr::read_unaligned((fp + 8) as *const u64);
        (*ctx).gpr[RBP] = caller_fp;
        (*ctx).rip = if caller_lr != 0 { caller_lr } else { lr };
        (*ctx).gpr[RSP] = fp + 16;
    } else {
        (*ctx).gpr[RBP] = fp;
        (*ctx).gpr[RSP] = sp;
        (*ctx).rip = lr;
    }
}

/// Load every register from `ctx` and jump to `ctx.rip`. `ctx` must not live
/// on the stack region just below the target rsp (we copy it into the TEB
/// scratch area first, see [`restore_context`]).
#[cfg(target_arch = "x86_64")]
#[unsafe(naked)]
unsafe extern "win64" fn restore_context_asm(_ctx: *const Context) -> ! {
    std::arch::naked_asm!(
        "fxrstor [rcx+0x100]",
        "ldmxcsr [rcx+0x34]",
        "mov rax, [rcx+0x98]",
        "lea rax, [rax-24]",
        "mov rdx, [rcx+0x78]",
        "mov [rax], rdx",
        "mov rdx, [rcx+0x80]",
        "mov [rax+8], rdx",
        "mov rdx, [rcx+0xF8]",
        "mov [rax+16], rdx",
        "push qword ptr [rcx+0x44]",
        "popfq",
        "mov rdx, [rcx+0x88]",
        "mov rbx, [rcx+0x90]",
        "mov rbp, [rcx+0xA0]",
        "mov rsi, [rcx+0xA8]",
        "mov rdi, [rcx+0xB0]",
        "mov r8,  [rcx+0xB8]",
        "mov r9,  [rcx+0xC0]",
        "mov r10, [rcx+0xC8]",
        "mov r11, [rcx+0xD0]",
        "mov r12, [rcx+0xD8]",
        "mov r13, [rcx+0xE0]",
        "mov r14, [rcx+0xE8]",
        "mov r15, [rcx+0xF0]",
        "mov rsp, rax",
        "pop rax",
        "pop rcx",
        "ret",
    )
}

#[cfg(not(target_arch = "x86_64"))]
unsafe extern "win64" fn restore_context_asm(ctx: *const Context) -> ! {
    let c = &*ctx;
    #[cfg(target_arch = "aarch64")]
    {
        // Reload the arm64 callee-saved registers captured by
        // `RtlCaptureContext` (stashed in `vector_register`), then the
        // x64-style control state. x1..x5 are caller-saved scratch; the
        // explicit-register form keeps the compiler from assigning one of
        // our reload targets to an operand.
        let nv = (ctx as *const u8).add(std::mem::offset_of!(Context, vector_register));
        std::arch::asm!(
            "ldr x19, [x1, #0x00]",
            "ldr x20, [x1, #0x08]",
            "ldr x21, [x1, #0x10]",
            "ldr x22, [x1, #0x18]",
            "ldr x23, [x1, #0x20]",
            "ldr x24, [x1, #0x28]",
            "ldr x25, [x1, #0x30]",
            "ldr x26, [x1, #0x38]",
            "ldr x27, [x1, #0x40]",
            "ldr x28, [x1, #0x48]",
            "mov x0, x4",
            "mov x29, x3",
            "mov sp, x2",
            "br x5",
            in("x1") nv,
            in("x2") c.gpr[RSP],
            in("x3") c.gpr[RBP],
            in("x4") c.gpr[RAX],
            in("x5") c.rip,
            options(noreturn),
        );
    }
    #[cfg(not(target_arch = "aarch64"))]
    loop {
        std::hint::spin_loop();
    }
}

/// TEB scratch area used to hold the CONTEXT during restore (beyond the
/// fields Windows defines; our TEB allocation is 0x2000 bytes).
const TEB_SCRATCH_CONTEXT: usize = 0x1900;

pub unsafe fn restore_context(ctx: &Context) -> ! {
    let teb = crate::teb::current();
    let dst = teb.add(TEB_SCRATCH_CONTEXT) as *mut Context;
    debug_assert_eq!(dst as usize % 16, 0);
    std::ptr::write(dst, *ctx);
    restore_context_asm(dst)
}

pub unsafe extern "win64" fn RtlRestoreContext(ctx: *mut Context, rec: *mut ExceptionRecord) {
    // Called by guest code: `ctx` is a guest context in the caller's emulated region.
    let mut caller = Context::zeroed();
    let cur = guest_caller_context(RtlRestoreContext as *const () as usize, &mut caller).unwrap_or(Cur::Host);
    if !rec.is_null() {
        consolidate(&mut *ctx, cur, &mut *rec);
    }
    resume_at(&*ctx, cur)
}

/// Defines a naked export that captures the caller's CONTEXT on its own
/// stack and calls `$imp(ctx: *mut Context, cur: usize) -> u64` (`cur` = the
/// caller's walk cursor, [`cur_to_raw`]); the original register
/// arguments are available as `ctx.gpr[RCX]`, `[RDX]`, `[R8]`, `[R9]` and
/// stack arguments at `ctx.rsp() + 0x28 + 8*n`.
///
/// aarch64: called directly by guest code (the innermost EH record is an emulated region
/// whose in-flight host call is this export), the CONTEXT is that guest frame's
/// ([`guest_caller_context`]); otherwise the host capture, cursor `Host`.
macro_rules! context_capturing_export {
    ($name:ident => $imp:path) => {
        #[cfg(target_arch = "x86_64")]
        #[unsafe(naked)]
        pub unsafe extern "win64" fn $name() {
            std::arch::naked_asm!(
                "sub rsp, 0x4F8",
                "mov [rsp+0x20+0x78], rax",
                "mov [rsp+0x20+0x80], rcx",
                "mov [rsp+0x20+0x88], rdx",
                "mov [rsp+0x20+0x90], rbx",
                "mov [rsp+0x20+0xA0], rbp",
                "mov [rsp+0x20+0xA8], rsi",
                "mov [rsp+0x20+0xB0], rdi",
                "mov [rsp+0x20+0xB8], r8",
                "mov [rsp+0x20+0xC0], r9",
                "mov [rsp+0x20+0xC8], r10",
                "mov [rsp+0x20+0xD0], r11",
                "mov [rsp+0x20+0xD8], r12",
                "mov [rsp+0x20+0xE0], r13",
                "mov [rsp+0x20+0xE8], r14",
                "mov [rsp+0x20+0xF0], r15",
                "lea rax, [rsp+0x500]",
                "mov [rsp+0x20+0x98], rax",
                "mov rax, [rsp+0x4F8]",
                "mov [rsp+0x20+0xF8], rax",
                "pushfq",
                "pop rax",
                "mov [rsp+0x20+0x44], eax",
                "mov word ptr [rsp+0x20+0x38], cs",
                "mov word ptr [rsp+0x20+0x42], ss",
                "fxsave [rsp+0x20+0x100]",
                "stmxcsr [rsp+0x20+0x34]",
                "mov dword ptr [rsp+0x20+0x30], 0x10000F",
                "lea rcx, [rsp+0x20]",
                "xor edx, edx",
                "call {imp}",
                "add rsp, 0x4F8",
                "ret",
                imp = sym $imp,
            )
        }
        #[cfg(not(target_arch = "x86_64"))]
        pub unsafe extern "win64" fn $name(
            rcx: u64,
            rdx: u64,
            r8: u64,
            r9: u64,
            a5: u64,
            a6: u64,
        ) -> u64 {
            let mut ctx = Context::zeroed();
            let cur = match guest_caller_context($name as *const () as usize, &mut ctx) {
                Some(c) => c,
                None => {
                    RtlCaptureContext(&mut ctx);
                    Cur::Host
                }
            };
            ctx.gpr[RCX] = rcx;
            ctx.gpr[RDX] = rdx;
            ctx.gpr[R8] = r8;
            ctx.gpr[R9] = r9;
            ctx.gpr[R10] = a5;
            ctx.gpr[R11] = a6;
            let mut stack_slots = [0u64; 8];
            stack_slots[4] = a5;
            stack_slots[5] = a6;
            if ctx.gpr[RSP] == 0 {
                ctx.gpr[RSP] = stack_slots.as_ptr() as u64;
            }
            $imp(&mut ctx, cur_to_raw(cur))
        }
    };
}

/// [`Cur`] as one register-sized value (records are 8-aligned: tag in the low 2 bits).
/// `HostFrame(r)` travels as `Visit(r)`, which settles to the same frame.
pub fn cur_to_raw(c: Cur) -> usize {
    match c {
        Cur::Host => 0,
        Cur::Region(p) => p as usize | 1,
        Cur::Lifted(p) => p as usize | 2,
        Cur::Visit(p) | Cur::HostFrame(p) => p as usize | 3,
    }
}

pub fn cur_from_raw(v: usize) -> Cur {
    let p = (v & !3) as *mut EhLink;
    match v & 3 {
        0 => Cur::Host,
        1 => Cur::Region(p),
        2 => Cur::Lifted(p),
        _ => Cur::Visit(p),
    }
}

// ---------------------------------------------------------------------------
// Function table lookup (static .pdata via the loader + dynamic tables)
// ---------------------------------------------------------------------------

struct DynTable {
    table: *const RuntimeFunction,
    count: u32,
    base: u64,
}
unsafe impl Send for DynTable {}
static DYN_TABLES: Mutex<Vec<DynTable>> = Mutex::new(Vec::new());

#[derive(Clone, Copy)]
struct DynCallbackTable {
    table_id: u64,
    base: u64,
    len: u32,
    callback: usize,
    context: usize,
    guest_image_backed: bool,
}
static DYN_CALLBACKS: Mutex<Vec<DynCallbackTable>> = Mutex::new(Vec::new());

type GetRuntimeFunctionCallback =
    unsafe extern "win64" fn(u64, *const c_void) -> *const RuntimeFunction;

#[inline]
fn is_guest_callback_addr(addr: usize) -> bool {
    #[cfg(not(target_arch = "x86_64"))]
    {
        in_pe_image(addr as u64) || crate::kernel32::memory::is_guest_vm_addr(addr)
    }
    #[cfg(target_arch = "x86_64")]
    {
        in_pe_image(addr as u64)
    }
}

pub unsafe extern "win64" fn RtlAddFunctionTable(table: *const RuntimeFunction, count: u32, base: u64) -> u8 {
    if table.is_null() || count == 0 {
        return 0;
    }
    DYN_TABLES.lock().unwrap_or_else(|e| e.into_inner()).push(DynTable { table, count, base });
    1
}

pub unsafe extern "win64" fn RtlDeleteFunctionTable(table: *const RuntimeFunction) -> u8 {
    let raw = table as u64;
    // Windows marks callback table identifiers with both low bits set (TableIdentifier & 3 == 3).
    if (raw & 3) == 3 {
        let mut cbs = DYN_CALLBACKS.lock().unwrap_or_else(|e| e.into_inner());
        let n = cbs.len();
        cbs.retain(|d| d.table_id != raw);
        return (cbs.len() != n) as u8;
    }
    let mut t = DYN_TABLES.lock().unwrap_or_else(|e| e.into_inner());
    let n = t.len();
    t.retain(|d| d.table != table);
    (t.len() != n) as u8
}

pub unsafe extern "win64" fn RtlAddGrowableFunctionTable(
    dynamic_table: *mut *mut c_void,
    table: *const RuntimeFunction,
    entry_count: u32,
    _max_entry_count: u32,
    range_base: usize,
    _range_end: usize,
) -> i32 {
    if dynamic_table.is_null() {
        return 0xC000_000Du32 as i32; // STATUS_INVALID_PARAMETER
    }
    DYN_TABLES.lock().unwrap_or_else(|e| e.into_inner()).push(DynTable {
        table,
        count: entry_count,
        base: range_base as u64,
    });
    std::ptr::write_unaligned(dynamic_table, table as *mut c_void);
    0 // STATUS_SUCCESS
}

pub unsafe extern "win64" fn RtlGrowFunctionTable(dynamic_table: *mut c_void, new_entry_count: u32) {
    let table = dynamic_table as *const RuntimeFunction;
    let mut t = DYN_TABLES.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(d) = t.iter_mut().find(|d| d.table == table) {
        d.count = new_entry_count;
    }
}

pub unsafe extern "win64" fn RtlDeleteGrowableFunctionTable(dynamic_table: *mut c_void) {
    let _ = RtlDeleteFunctionTable(dynamic_table as *const RuntimeFunction);
}

pub unsafe extern "win64" fn RtlInstallFunctionTableCallback(
    table_id: u64,
    base: u64,
    len: u32,
    cb: *const c_void,
    ctx: *const c_void,
    _dll: *const u16,
) -> u8 {
    // Windows requires the two low-order bits of TableIdentifier to be set, non-zero range
    // without overflow, and a non-null callback pointer.
    if (table_id & 3) != 3 || len == 0 || cb.is_null() || base.checked_add(len as u64).is_none() {
        return 0;
    }
    let cb_addr = cb as usize;
    let guest_image_backed = in_pe_image(cb_addr as u64);
    let mut cbs = DYN_CALLBACKS.lock().unwrap_or_else(|e| e.into_inner());
    if cbs.iter().any(|d| d.table_id == table_id) {
        return 0;
    }
    cbs.push(DynCallbackTable {
        table_id,
        base,
        len,
        callback: cb_addr,
        context: ctx as usize,
        guest_image_backed,
    });
    1
}

thread_local! {
    static ACTIVE_CALLBACK_LOOKUPS: std::cell::RefCell<Vec<(u64, u64)>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

struct ActiveCallbackGuard;
impl Drop for ActiveCallbackGuard {
    fn drop(&mut self) {
        ACTIVE_CALLBACK_LOOKUPS.with(|s| {
            let _ = s.borrow_mut().pop();
        });
    }
}

unsafe fn invoke_function_table_callback(entry: DynCallbackTable, pc: u64) -> *const RuntimeFunction {
    if entry.guest_image_backed || is_guest_callback_addr(entry.callback) {
        crate::call_guest8_seh(
            entry.callback,
            pc,
            entry.context as u64,
            0,
            0,
            0,
            0,
            0,
            0,
        ) as *const RuntimeFunction
    } else {
        let f: GetRuntimeFunctionCallback = std::mem::transmute(entry.callback);
        f(pc, entry.context as *const c_void)
    }
}

pub fn lookup_function_entry(pc: u64) -> Option<(u64, *const RuntimeFunction)> {
    {
        let t = DYN_TABLES.lock().unwrap_or_else(|e| e.into_inner());
        for d in t.iter() {
            if d.table.is_null() || d.count == 0 || pc < d.base || pc - d.base > u32::MAX as u64 {
                continue;
            }
            let rva = (pc - d.base) as u32;
            for i in 0..d.count as usize {
                let p = unsafe { d.table.add(i) };
                let f = unsafe { std::ptr::read_unaligned(p) };
                if rva >= f.begin_rva && rva < f.end_rva {
                    return Some((d.base, p));
                }
            }
        }
    }
    let matching_callbacks: Vec<DynCallbackTable> = {
        let cbs = DYN_CALLBACKS.lock().unwrap_or_else(|e| e.into_inner());
        cbs.iter()
            .copied()
            .filter(|d| pc >= d.base && pc < d.base.saturating_add(d.len as u64))
            .collect()
    };
    const MAX_CALLBACK_REENTRY_DEPTH: usize = 16;
    for entry in matching_callbacks {
        let can_enter = ACTIVE_CALLBACK_LOOKUPS.with(|s| {
            let mut active = s.borrow_mut();
            if active.len() >= MAX_CALLBACK_REENTRY_DEPTH
                || active.contains(&(entry.table_id, pc))
            {
                return false;
            }
            active.push((entry.table_id, pc));
            true
        });
        if !can_enter {
            continue;
        }
        let _guard = ActiveCallbackGuard;
        let rf = unsafe { invoke_function_table_callback(entry, pc) };
        if !rf.is_null() {
            return Some((entry.base, rf));
        }
    }
    crate::try_loader()?.lookup_function_entry(pc)
}

pub unsafe extern "win64" fn RtlLookupFunctionEntry(pc: u64, base: *mut u64, _history: *mut c_void) -> *const RuntimeFunction {
    match lookup_function_entry(pc) {
        Some((b, f)) => {
            if !base.is_null() {
                std::ptr::write_unaligned(base, b);
            }
            f
        }
        None => std::ptr::null(),
    }
}

pub unsafe extern "win64" fn RtlPcToFileHeader(pc: *const c_void, base: *mut *mut c_void) -> *mut c_void {
    let b = crate::try_loader().and_then(|l| l.module_from_address(pc as usize)).unwrap_or(0);
    *base = b as *mut c_void;
    b as *mut c_void
}

pub(crate) fn in_pe_image(pc: u64) -> bool {
    if crate::try_loader().and_then(|l| l.module_from_address(pc as usize)).is_some() {
        return true;
    }
    let t = TEST_IMAGES.lock().unwrap();
    !t.is_empty() && t.iter().any(|&(b, s)| pc >= b && pc < b + s)
}

/// In-memory "images" (guest code with a function table) that count as PE image code:
/// aw2-game's EH runtime tests build one per test. Not used by the game.
static TEST_IMAGES: Mutex<Vec<(u64, u64)>> = Mutex::new(Vec::new());

/// Treat `[base, base + size)` as PE image code (see [`TEST_IMAGES`]); register its function
/// table with [`RtlAddFunctionTable`] separately.
pub fn register_test_image(base: u64, size: u64) {
    TEST_IMAGES.lock().unwrap().push((base, size));
}

// ---------------------------------------------------------------------------
// RtlVirtualUnwind
// ---------------------------------------------------------------------------

const UWOP_PUSH_NONVOL: u8 = 0;
const UWOP_ALLOC_LARGE: u8 = 1;
const UWOP_ALLOC_SMALL: u8 = 2;
const UWOP_SET_FPREG: u8 = 3;
const UWOP_SAVE_NONVOL: u8 = 4;
const UWOP_SAVE_NONVOL_FAR: u8 = 5;
const UWOP_EPILOG: u8 = 6;
const UWOP_SAVE_XMM128: u8 = 8;
const UWOP_SAVE_XMM128_FAR: u8 = 9;
const UWOP_PUSH_MACHFRAME: u8 = 10;

fn opcode_size(code: u8, info: u8) -> usize {
    match code {
        UWOP_ALLOC_LARGE => 2 + (info != 0) as usize,
        UWOP_SAVE_NONVOL | UWOP_SAVE_XMM128 | UWOP_EPILOG => 2,
        UWOP_SAVE_NONVOL_FAR | UWOP_SAVE_XMM128_FAR => 3,
        _ => 1,
    }
}

unsafe fn rd<T: Copy>(a: u64) -> T {
    std::ptr::read_unaligned(a as *const T)
}

unsafe fn is_inside_epilog(mut pc: *const u8, base: u64, f: &RuntimeFunction) -> bool {
    let in_func = |p: *const u8| {
        let r = p as u64 - base;
        r >= f.begin_rva as u64 && r < f.end_rva as u64
    };
    if *pc & 0xf8 == 0x48 {
        match *pc.add(1) {
            0x81 => {
                if *pc == 0x48 && *pc.add(2) == 0xc4 {
                    pc = pc.add(7);
                } else {
                    return false;
                }
            }
            0x83 => {
                if *pc == 0x48 && *pc.add(2) == 0xc4 {
                    pc = pc.add(4);
                } else {
                    return false;
                }
            }
            0x8d => {
                let modrm = *pc.add(2);
                if *pc & 0x06 != 0 || (modrm >> 3) & 7 != 4 || modrm & 7 == 4 {
                    return false;
                }
                match modrm >> 6 {
                    1 => pc = pc.add(4),
                    2 => pc = pc.add(7),
                    _ => return false,
                }
            }
            _ => {}
        }
    }
    loop {
        if *pc & 0xf0 == 0x40 {
            pc = pc.add(1);
        }
        match *pc {
            0x58..=0x5f => {
                pc = pc.add(1);
                continue;
            }
            0xc2 | 0xc3 => return true,
            0xe9 => {
                pc = pc.offset(5 + rd::<i32>(pc as u64 + 1) as isize);
                if in_func(pc) {
                    continue;
                }
            }
            0xeb => {
                pc = pc.offset(2 + *pc.add(1) as i8 as isize);
                if in_func(pc) {
                    continue;
                }
            }
            0xf3 => return *pc.add(1) == 0xc3,
            _ => {}
        }
        return false;
    }
}

unsafe fn interpret_epilog(mut pc: *const u8, ctx: &mut Context) {
    loop {
        let mut rex = 0u8;
        if *pc & 0xf0 == 0x40 {
            rex = *pc & 0x0f;
            pc = pc.add(1);
        }
        match *pc {
            op @ 0x58..=0x5f => {
                let r = (op - 0x58) as usize + (rex as usize & 1) * 8;
                ctx.gpr[r] = rd(ctx.gpr[RSP]);
                ctx.gpr[RSP] += 8;
                pc = pc.add(1);
            }
            0x81 => {
                ctx.gpr[RSP] = ctx.gpr[RSP].wrapping_add(rd::<i32>(pc as u64 + 2) as i64 as u64);
                pc = pc.add(6);
            }
            0x83 => {
                ctx.gpr[RSP] = ctx.gpr[RSP].wrapping_add(*pc.add(2) as i8 as i64 as u64);
                pc = pc.add(3);
            }
            0x8d => {
                let modrm = *pc.add(1);
                let r = (modrm & 7) as usize + (rex as usize & 1) * 8;
                if modrm >> 6 == 1 {
                    ctx.gpr[RSP] = ctx.gpr[r].wrapping_add(*pc.add(2) as i8 as i64 as u64);
                    pc = pc.add(3);
                } else {
                    ctx.gpr[RSP] = ctx.gpr[r].wrapping_add(rd::<i32>(pc as u64 + 2) as i64 as u64);
                    pc = pc.add(6);
                }
            }
            0xc2 => {
                ctx.rip = rd(ctx.gpr[RSP]);
                ctx.gpr[RSP] += 8 + rd::<u16>(pc as u64 + 1) as u64;
                return;
            }
            0xc3 | 0xf3 => {
                ctx.rip = rd(ctx.gpr[RSP]);
                ctx.gpr[RSP] += 8;
                return;
            }
            0xe9 => pc = pc.offset(5 + rd::<i32>(pc as u64 + 1) as isize),
            0xeb => pc = pc.offset(2 + *pc.add(1) as i8 as isize),
            _ => return,
        }
    }
}

/// EstablisherFrame of the function at `fn_va` as `entry rsp - delta` (the return address
/// at `entry rsp`): the static rsp delta of the prolog up to `UWOP_SET_FPREG` (frame register:
/// EstablisherFrame = frame register - 16 * FrameOffset = rsp when it was set) or of the whole
/// prolog (no frame register: rsp after the prolog). `None` without `.pdata` / for unknown
/// opcodes.
pub fn establisher_rsp_delta(fn_va: u64) -> Option<u64> {
    let (base, mut func) = lookup_function_entry(fn_va)?;
    // (delta, is SET_FPREG) in reverse prolog order: primary info's codes, then its parents'.
    let mut ops: Vec<(u64, bool)> = Vec::new();
    unsafe {
        for _ in 0..32 {
            let info = base + (*func).unwind_rva as u64;
            let ver_flags: u8 = rd(info);
            let count: u8 = rd(info + 2);
            let codes = info + 4;
            let mut i = 0usize;
            while i < count as usize {
                let b: u8 = rd(codes + i as u64 * 2 + 1);
                let (code, inf) = (b & 0xf, b >> 4);
                let d = match code {
                    UWOP_PUSH_NONVOL => 8,
                    UWOP_ALLOC_LARGE if inf != 0 => rd::<u32>(codes + (i + 1) as u64 * 2) as u64,
                    UWOP_ALLOC_LARGE => rd::<u16>(codes + (i + 1) as u64 * 2) as u64 * 8,
                    UWOP_ALLOC_SMALL => (inf as u64 + 1) * 8,
                    UWOP_SET_FPREG | UWOP_SAVE_NONVOL | UWOP_SAVE_NONVOL_FAR | UWOP_SAVE_XMM128
                    | UWOP_SAVE_XMM128_FAR | UWOP_EPILOG => 0,
                    _ => return None,
                };
                ops.push((d, code == UWOP_SET_FPREG));
                i += opcode_size(code, inf);
            }
            if (ver_flags >> 3) as u32 & UNW_FLAG_CHAININFO == 0 {
                break;
            }
            func = (codes + (((count as u64) + 1) & !1) * 2) as *const RuntimeFunction;
        }
    }
    let from = ops.iter().position(|&(_, fp)| fp).map_or(0, |k| k + 1);
    Some(ops[from..].iter().map(|&(d, _)| d).sum())
}

/// Core of `RtlVirtualUnwind`. Returns (handler, handler_data, establisher_frame).
pub unsafe fn virtual_unwind(
    handler_type: u32,
    base: u64,
    pc: u64,
    mut func: *const RuntimeFunction,
    ctx: &mut Context,
) -> (Option<u64>, u64, u64) {
    let mut frame = ctx.gpr[RSP];
    let mut establisher = frame;
    let mut mach_frame = false;
    let mut prolog_offset: u64;
    let mut first = true;
    let mut last_info: u64;
    let mut handler_data_addr: u64;
    loop {
        let f = &*func;
        let info = base + f.unwind_rva as u64;
        last_info = info;
        let ver_flags: u8 = rd(info);
        let version = ver_flags & 7;
        let prolog: u8 = rd(info + 1);
        let count: u8 = rd(info + 2);
        let fr: u8 = rd(info + 3);
        let frame_reg = (fr & 0xf) as usize;
        let frame_off = (fr >> 4) as u64;
        let codes = info + 4;
        handler_data_addr = codes + (((count as u64) + 1) & !1) * 2;
        if version != 1 && version != 2 {
            log::error!("unknown unwind info version {version} at 0x{info:x}");
            return (None, 0, establisher);
        }
        if frame_reg != 0 {
            frame = ctx.gpr[frame_reg].wrapping_sub(frame_off * 16);
        }
        let fbegin = base + f.begin_rva as u64;
        if pc >= fbegin && pc < fbegin + prolog as u64 {
            prolog_offset = pc - fbegin;
        } else {
            prolog_offset = u64::MAX;
            if first && is_inside_epilog(pc as *const u8, base, f) {
                interpret_epilog(pc as *const u8, ctx);
                return (None, 0, frame);
            }
        }
        first = false;
        let mut i = 0usize;
        while i < count as usize {
            let off: u8 = rd(codes + i as u64 * 2);
            let b: u8 = rd(codes + i as u64 * 2 + 1);
            let code = b & 0xf;
            let inf = b >> 4;
            let sz = opcode_size(code, inf);
            if prolog_offset != u64::MAX && prolog_offset < off as u64 {
                i += sz;
                continue;
            }
            let slot16 = |k: usize| -> u64 { rd::<u16>(codes + (i + k) as u64 * 2) as u64 };
            let slot32 = || -> u64 { rd::<u32>(codes + (i + 1) as u64 * 2) as u64 };
            match code {
                UWOP_PUSH_NONVOL => {
                    ctx.gpr[inf as usize] = rd(ctx.gpr[RSP]);
                    ctx.gpr[RSP] += 8;
                }
                UWOP_ALLOC_LARGE => {
                    if inf != 0 {
                        ctx.gpr[RSP] += slot32();
                    } else {
                        ctx.gpr[RSP] += slot16(1) * 8;
                    }
                }
                UWOP_ALLOC_SMALL => ctx.gpr[RSP] += (inf as u64 + 1) * 8,
                UWOP_SET_FPREG => {
                    ctx.gpr[RSP] = frame;
                    establisher = frame;
                }
                UWOP_SAVE_NONVOL => ctx.gpr[inf as usize] = rd(frame + slot16(1) * 8),
                UWOP_SAVE_NONVOL_FAR => ctx.gpr[inf as usize] = rd(frame + slot32()),
                UWOP_SAVE_XMM128 => {
                    let v: [u8; 16] = rd(frame + slot16(1) * 16);
                    *ctx.xmm_mut(inf as usize) = v;
                }
                UWOP_SAVE_XMM128_FAR => {
                    let v: [u8; 16] = rd(frame + slot32());
                    *ctx.xmm_mut(inf as usize) = v;
                }
                UWOP_PUSH_MACHFRAME => {
                    if inf != 0 {
                        ctx.gpr[RSP] += 8;
                    }
                    ctx.rip = rd(ctx.gpr[RSP]);
                    ctx.gpr[RSP] = rd(ctx.gpr[RSP] + 24);
                    mach_frame = true;
                }
                UWOP_EPILOG => {}
                _ => log::warn!("unknown unwind opcode {code}"),
            }
            i += sz;
        }
        if (ver_flags >> 3) as u32 & UNW_FLAG_CHAININFO == 0 {
            break;
        }
        func = handler_data_addr as *const RuntimeFunction;
    }
    if !mach_frame {
        ctx.rip = rd(ctx.gpr[RSP]);
        ctx.gpr[RSP] += 8;
    }
    let flags = (rd::<u8>(last_info) >> 3) as u32;
    if flags & handler_type == 0 || prolog_offset != u64::MAX {
        return (None, 0, establisher);
    }
    let handler_rva: u32 = rd(handler_data_addr);
    (Some(base + handler_rva as u64), handler_data_addr + 4, establisher)
}

pub unsafe extern "win64" fn RtlVirtualUnwind(
    handler_type: u32,
    base: u64,
    pc: u64,
    func: *const RuntimeFunction,
    ctx: *mut Context,
    handler_data: *mut *mut c_void,
    establisher: *mut u64,
    _ctx_ptrs: *mut c_void,
) -> *const c_void {
    let (h, d, f) = virtual_unwind(handler_type, base, pc, func, &mut *ctx);
    if !establisher.is_null() {
        *establisher = f;
    }
    if !handler_data.is_null() {
        *handler_data = d as *mut c_void;
    }
    h.map_or(std::ptr::null(), |h| h as *const c_void)
}

/// One step of a stack walk: unwind `ctx` to the caller. Returns
/// `(handler, handler_data, establisher, image_base, function)` or None when
/// the walk leaves Windows code.
unsafe fn step(
    handler_type: u32,
    ctx: &Context,
    new_ctx: &mut Context,
) -> Option<(Option<u64>, u64, u64, u64, *const RuntimeFunction)> {
    let pc = ctx.rip;
    if pc == 0 {
        return None;
    }
    *new_ctx = *ctx;
    match lookup_function_entry(pc) {
        Some((base, f)) => {
            let (h, d, est) = virtual_unwind(handler_type, base, pc, f, new_ctx);
            Some((h, d, est, base, f))
        }
        None => {
            if !in_pe_image(pc) {
                return cross_host_frames(ctx, new_ctx).then(|| (None, 0, ctx.gpr[RSP], 0, std::ptr::null()));
            }
            // Leaf function: return address on top of the stack.
            new_ctx.rip = rd(ctx.gpr[RSP]);
            new_ctx.gpr[RSP] += 8;
            Some((None, 0, ctx.gpr[RSP], 0, std::ptr::null()))
        }
    }
}

// ---------------------------------------------------------------------------
// Crossing host (Rust) frames: dispatcher frames and fence frames
// ---------------------------------------------------------------------------

/// Return addresses of the loader's fat hook shims (the instruction after
/// `call <rust replacement>`), sorted. Each shim also has a RUNTIME_FUNCTION
/// registered with [`RtlAddFunctionTable`] that restores all nonvolatiles.
static FENCES: std::sync::RwLock<Vec<u64>> = std::sync::RwLock::new(Vec::new());

/// Register a fence: `ret` is the address right after the shim's call into
/// host code, and must lie in a function registered with
/// [`RtlAddFunctionTable`] whose unwind info restores every Win64 nonvolatile.
pub fn register_fence(ret: u64) {
    let mut f = FENCES.write().unwrap();
    if let Err(i) = f.binary_search(&ret) {
        f.insert(i, ret);
    }
}

/// Is `ret` a registered fence return address?
pub fn is_fence(ret: u64) -> bool {
    FENCES.read().unwrap().binary_search(&ret).is_ok()
}

/// A host (Rust) stack region of our own SEH machinery that calls back into
/// guest code, and the guest context a walk resumes at when it reaches it.
struct HostCrossing {
    /// Address of a local in the host frame that makes the guest call. The
    /// guest callee's return lands at an rsp `<= marker`, and the region ends
    /// (upward) at the innermost marker above a walk's rsp.
    marker: u64,
    resume: Box<Context>,
}

thread_local! {
    /// Host regions that call guest code (innermost last):
    /// * [`dispatch_exception`] calling a language handler: resume at the raise
    ///   site, as Windows unwinds through `RtlpExecuteHandlerForException`.
    /// * [`do_unwind`] calling a `STATUS_UNWIND_CONSOLIDATE` callback (MSVC's
    ///   `__CxxCallCatchBlock`, which runs the catch funclet): resume at the
    ///   unwind target, as Windows' `RcConsolidateFrames` frame does. A rethrow
    ///   from the catch block must not revisit the frames already unwound.
    static CROSSINGS: std::cell::RefCell<Vec<HostCrossing>> = const { std::cell::RefCell::new(Vec::new()) };
}

fn push_crossing(marker: u64, resume: &Context) {
    CROSSINGS.with(|d| d.borrow_mut().push(HostCrossing { marker, resume: Box::new(*resume) }));
}

fn pop_crossing(marker: u64) {
    CROSSINGS.with(|d| d.borrow_mut().retain(|c| c.marker != marker));
}

/// Forget crossings below `rsp`: an unwind to a frame at `rsp` abandons them.
fn abandon_crossings_below(rsp: u64) {
    CROSSINGS.with(|d| d.borrow_mut().retain(|c| c.marker >= rsp));
}

/// Stack bounds of the current thread from its TEB, as `(low, high)`.
fn stack_bounds() -> (u64, u64) {
    let teb = crate::teb::current();
    if teb.is_null() {
        return (0, u64::MAX);
    }
    unsafe {
        (
            crate::teb::field::<usize>(teb, crate::teb::off::STACK_LIMIT) as u64,
            crate::teb::field::<usize>(teb, crate::teb::off::STACK_BASE) as u64,
        )
    }
}

/// `ctx.rip` is in host code. Set `new_ctx` to the next guest-unwindable frame
/// above it (docs/design/ADVICE_SEH.md):
/// 1. A **fence**: the host frame-pointer chain from `ctx.rbp` reaches the
///    return address of a fat hook shim ([`register_fence`]) below the
///    innermost [`HostCrossing`] marker. `new_ctx` is the shim right after its
///    call into Rust; its unwind info restores the original caller's
///    nonvolatiles. The Rust frames in between are discarded.
/// 2. Otherwise the innermost **crossing** above `ctx.rsp` (our dispatcher
///    calling a language handler, or [`do_unwind`] running a consolidate
///    callback): `new_ctx` is its resume context.
///
/// Returns false if neither applies. Also usable by host-side stack walkers
/// (crash reports) to step from Rust code back into guest frames.
pub unsafe fn cross_host_frames(ctx: &Context, new_ctx: &mut Context) -> bool {
    let sp = ctx.gpr[RSP];
    let crossing = CROSSINGS.with(|d| {
        d.borrow().iter().filter(|c| c.marker >= sp).min_by_key(|c| c.marker).map(|c| (c.marker, *c.resume))
    });
    let bound = crossing.as_ref().map_or(u64::MAX, |c| c.0);
    let (lo, hi) = stack_bounds();
    let mut fp = ctx.gpr[RBP];
    let mut frames = 0;
    while frames < 512 {
        if fp < sp || fp < lo || fp.saturating_add(16) > hi || fp.saturating_add(16) > bound || fp & 7 != 0 {
            break;
        }
        let ret: u64 = rd(fp + 8);
        if is_fence(ret) {
            *new_ctx = *ctx;
            new_ctx.rip = ret;
            new_ctx.gpr[RSP] = fp + 16;
            new_ctx.gpr[RBP] = rd(fp);
            log::debug!(
                target: "win32_rt::seh",
                "fence: discarding {} host frame(s) from {} to the hook shim",
                frames + 1,
                crate::symbolize(ctx.rip as usize)
            );
            return true;
        }
        let next: u64 = rd(fp);
        if next <= fp {
            break;
        }
        fp = next;
        frames += 1;
    }
    if let Some((_, resume)) = crossing {
        log::trace!(
            target: "win32_rt::seh",
            "unwind: host SEH frames at {} -> {}",
            crate::symbolize(ctx.rip as usize),
            crate::symbolize(resume.rip as usize)
        );
        *new_ctx = resume;
        return true;
    }
    false
}

// ---------------------------------------------------------------------------
// EH record chain: emulated and lifted frames (aarch64)
// ---------------------------------------------------------------------------
//
// Guest code runs on the emulator's own stack and lifted code on the lstack arena, with
// Rust frames in between that have no unwind info. A dispatch walk cannot get from one
// such region to the next by unwinding, so every thread keeps an intrusive chain of
// records, pushed in call order (target/review/exec/g12/eh_lift_plan.md §4):
// - `EH_EMU_ENTRY` (aw2-game `emu_run_full`): one emulated region. While its guest code is
//   in a host call, the record knows the guest state of that call (the exact CONTEXT of
//   the calling guest frame) and owns the anchor a landing in this region jumps to.
// - `EH_LIFTED` (aw2-game, prologue of a lifted function with a FuncInfo): fn VA,
//   establisher frame, state IP of its in-flight call, and the anchor of its in-flight
//   catching call.
// - `EH_CROSSING` (here): the dispatcher calling a language handler, or `consolidate`
//   calling a catch block. A walk reaching it continues at the saved position, as Windows
//   unwinds through `RtlpExecuteHandlerForException` / `RcConsolidateFrames`.
// - `EH_BARRIER` (here, `call_guest8`): a Rust API calling a guest callback. Opaque: the walk
//   stops, as Windows stops at a frame without unwind info.
// - `EH_HOST_FRAME` (here, [`EhHostFrame`]): a native function's own exception-handling frame
//   (handler + data), visited like a PE frame; e.g. the native `__CxxCallCatchBlock`'s
//   `__try` / `__finally` around the catch funclet.
// A walk position is a CONTEXT plus a cursor ([`Cur`]): the record that owns the frame. The
// walk never virtual-unwinds Rust frames or the lstack arena; records give the frame order.

/// Return-address sentinel at the entry rsp of every emulator run (aw2-game emu
/// `RET_SENTINEL`): a guest walk reaching it has left its emulated region.
pub const EMU_RET_SENTINEL: u64 = 0xDEAD_BEEF_CAFE_0000;

/// Header of every EH chain record (first field, `repr(C)`).
#[repr(C)]
pub struct EhLink {
    /// Next older record (null = end of chain).
    pub next: *mut EhLink,
    /// `EH_*` kind.
    pub kind: u32,
    /// `EH_F_*` flags.
    pub flags: u32,
}

pub const EH_EMU_ENTRY: u32 = 1;
pub const EH_LIFTED: u32 = 2;
pub const EH_CROSSING: u32 = 3;
pub const EH_BARRIER: u32 = 4;
/// [`EhHostFrame`] (here): the frame of a native function with an exception handler, e.g.
/// the native `__CxxCallCatchBlock` port's `__try` around the catch funclet. The walk visits
/// it as one frame (its handler called in both phases, EstablisherFrame = the record).
pub const EH_HOST_FRAME: u32 = 5;
/// `EH_EMU_ENTRY`: entered by a Rust API holding live state (not by lifted code through
/// `call_va8*`): a walk leaving the region stops.
pub const EH_F_OPAQUE: u32 = 1;

/// Walk cursor: which record owns the frame at a walk position.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cur {
    /// Host or native-PE frames, walked as before records existed (`step`). With the chain
    /// in use, a host pc continues at the innermost record.
    Host,
    /// A guest frame of the emulated region of this `EH_EMU_ENTRY` record.
    Region(*mut EhLink),
    /// The frame of this `EH_LIFTED` record.
    Lifted(*mut EhLink),
    /// The frame of this `EH_HOST_FRAME` record.
    HostFrame(*mut EhLink),
    /// Not settled yet: the position is the frame this record describes (null = end).
    Visit(*mut EhLink),
}

/// What an `EH_LIFTED` record reports to the walk.
#[derive(Clone, Copy, Default, Debug)]
pub struct LiftedInfo {
    /// Function VA (its `.pdata` entry and UNWIND_INFO handler).
    pub fn_va: u64,
    /// EstablisherFrame (lstack arena address).
    pub establisher: u64,
    /// ControlPc: the VA of the in-flight call.
    pub state_ip: u64,
}

/// aw2-game's side of the chain (win32-rt cannot depend on aw2-game).
pub struct EhHooks {
    /// This thread's innermost record.
    pub head: fn() -> *mut EhLink,
    pub set_head: fn(*mut EhLink),
    /// `EH_EMU_ENTRY` in a host call: fill `ctx` with the guest state of that call
    /// (`rip` = return VA, `rsp`, all registers). False if the region is not in one.
    pub callout: unsafe fn(*mut EhLink, &mut Context) -> bool,
    /// `EH_EMU_ENTRY`: the host function of its in-flight host call (0 = none).
    pub callout_fn: unsafe fn(*mut EhLink) -> usize,
    /// `EH_LIFTED`: its frame.
    pub lifted: unsafe fn(*mut EhLink) -> LiftedInfo,
    /// End VA of the guest instruction at `va` (a lifted frame's in-flight call: its return
    /// address, what a stack backtrace reports for the frame).
    pub insn_end: unsafe fn(u64) -> u64,
    /// Is `frame` a valid establisher frame outside the TEB stack (emulator stack, arena)?
    pub frame_ok: fn(u64) -> bool,
    /// Resume at `ctx` in the frame of this `EH_EMU_ENTRY` (guest state) or `EH_LIFTED`
    /// (continuation `ctx.rip`) record, discarding everything newer.
    pub land: unsafe fn(*mut EhLink, &Context) -> !,
    /// One-line description for logs.
    pub describe: fn(*mut EhLink) -> String,
}

static EH_HOOKS: std::sync::OnceLock<EhHooks> = std::sync::OnceLock::new();

/// Register aw2-game's chain hooks (once per process; later calls are ignored).
pub fn set_eh_hooks(h: EhHooks) {
    let _ = EH_HOOKS.set(h);
}

#[inline]
fn eh_hooks() -> Option<&'static EhHooks> {
    EH_HOOKS.get()
}

/// A crossing record (see the section comment): the resume position of a host region of
/// the exception machinery that calls back into guest/lifted code.
#[repr(C)]
struct EhCrossing {
    link: EhLink,
    resume: Context,
    cur: Cur,
}

/// Opaque barrier record pushed by [`crate::call_guest8`] around a Rust API's guest call.
#[repr(C)]
pub struct EhBarrier {
    link: EhLink,
    /// The callback (for the log).
    target: usize,
}

/// Pops a record pushed by [`EhBarrier::push`] / the crossings.
pub struct EhPopGuard(*mut EhLink);

impl Drop for EhPopGuard {
    #[inline]
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { eh_unlink(self.0) };
        }
    }
}

impl EhBarrier {
    #[inline]
    pub fn new(target: usize) -> Self {
        Self { link: EhLink { next: std::ptr::null_mut(), kind: EH_BARRIER, flags: 0 }, target }
    }

    /// Link this barrier as the innermost record until the guard drops (no-op without hooks).
    #[inline]
    pub fn push(&mut self) -> EhPopGuard {
        unsafe { push_link(&mut self.link) }
    }
}

/// Frame record of a native function with an exception handler ([`EH_HOST_FRAME`]): a
/// dispatch or unwind walk reaching it calls `handler(rec, establisher, ctx, dc)` with
/// EstablisherFrame = the record's address and `dc.HandlerData` = `data`, exactly as for a
/// PE frame whose UNWIND_INFO names that handler, then continues at the next older record.
/// Without a landing the handler only ever continues the search. With one
/// ([`EhHostFrame::with_land`]) the record is also an unwind target: an `RtlUnwindEx` whose
/// TargetFrame is the record's address runs the newer frames' cleanup, then calls
/// `land(record, context)` instead of restoring a context (a host-side `catch`).
/// A stack backtrace ([`backtrace_chain`]) reports nothing for the frame unless it names a
/// return address ([`EhHostFrame::with_pc`]).
#[repr(C)]
pub struct EhHostFrame {
    link: EhLink,
    handler: LanguageHandler,
    data: *mut c_void,
    land: Option<unsafe fn(*mut EhLink, &Context) -> !>,
    /// Return address a backtrace reports for this frame (0 = none).
    pc: u64,
}

impl EhHostFrame {
    #[inline]
    pub fn new(handler: LanguageHandler, data: *mut c_void) -> Self {
        Self { link: EhLink { next: std::ptr::null_mut(), kind: EH_HOST_FRAME, flags: 0 }, handler, data, land: None, pc: 0 }
    }

    /// This frame as an unwind target: an unwind to it ends in `land` (which must not return;
    /// it receives the record's address).
    #[inline]
    pub fn with_land(mut self, land: unsafe fn(*mut EhLink, &Context) -> !) -> Self {
        self.land = Some(land);
        self
    }

    /// The return address a stack backtrace reports for this frame (a host-side stand-in
    /// for a guest frame, e.g. the dll_lift difftest's model of its guest entry stub).
    #[inline]
    pub fn with_pc(mut self, pc: u64) -> Self {
        self.pc = pc;
        self
    }

    /// Link as the innermost record until the guard drops (no-op without hooks).
    #[inline]
    pub fn push(&mut self) -> EhPopGuard {
        unsafe { push_link(&mut self.link) }
    }
}

#[inline]
unsafe fn push_link(link: &mut EhLink) -> EhPopGuard {
    match eh_hooks() {
        Some(h) => {
            let p = link as *mut EhLink;
            link.next = (h.head)();
            (h.set_head)(p);
            EhPopGuard(p)
        }
        None => EhPopGuard(std::ptr::null_mut()),
    }
}

/// Unlink `rec`: normally the innermost record. If a landing already discarded it, the head
/// is older and nothing is done; a non-LIFO pop (never expected) unlinks it in place.
unsafe fn eh_unlink(rec: *mut EhLink) {
    let Some(h) = eh_hooks() else { return };
    let head = (h.head)();
    if head == rec {
        (h.set_head)((*rec).next);
        return;
    }
    let mut p = head;
    while !p.is_null() {
        if (*p).next == rec {
            log::warn!(target: "win32_rt::seh", "EH chain: non-LIFO pop of a kind {} record", (*rec).kind);
            (*p).next = (*rec).next;
            return;
        }
        p = (*p).next;
    }
}

/// Is `frame` an acceptable establisher frame: the TEB stack, or (chain in use) a region
/// aw2-game vouches for (emulator stack, lstack arena). 8-byte aligned either way.
fn frame_ok(frame: u64, lo: u64, hi: u64) -> bool {
    if frame & 7 != 0 {
        return false;
    }
    if frame >= lo && frame <= hi {
        return true;
    }
    eh_hooks().is_some_and(|h| (h.frame_ok)(frame))
}

/// Settle a walk position: a host pc (chain in use) and `Visit(r)` become the frame record
/// `r` describes; a guest region at its sentinel continues after the region's record
/// (unless the region is opaque). `Err` = the walk ends here (reason for the log).
unsafe fn settle(h: &EhHooks, ctx: &mut Context, cur: &mut Cur) -> Result<(), String> {
    for _ in 0..100_000 {
        match *cur {
            Cur::Host => {
                if ctx.rip == 0 || in_pe_image(ctx.rip) || lookup_function_entry(ctx.rip).is_some() {
                    return Ok(());
                }
                let head = (h.head)();
                if head.is_null() {
                    // No records on this thread: the walk ends as it always did.
                    return Ok(());
                }
                *cur = Cur::Visit(head);
            }
            Cur::Visit(r) => {
                if r.is_null() {
                    return Err("end of the EH record chain (outermost frame)".into());
                }
                match (*r).kind {
                    EH_EMU_ENTRY => {
                        if !(h.callout)(r, ctx) {
                            return Err(format!("{} is not in a host call", (h.describe)(r)));
                        }
                        // Region arm next: a callout returning straight to the run's
                        // sentinel (tail call from the entry fn) continues past the region.
                        *cur = Cur::Region(r);
                    }
                    EH_LIFTED => {
                        let li = (h.lifted)(r);
                        ctx.rip = li.state_ip;
                        ctx.gpr[RSP] = li.establisher;
                        *cur = Cur::Lifted(r);
                        return Ok(());
                    }
                    EH_CROSSING => {
                        let x = r as *const EhCrossing;
                        *ctx = (*x).resume;
                        *cur = (*x).cur;
                    }
                    EH_BARRIER => {
                        let b = r as *const EhBarrier;
                        return Err(format!(
                            "opaque host frame: a Rust API called the guest callback {} (no unwind info past it)",
                            crate::symbolize((*b).target)
                        ));
                    }
                    EH_HOST_FRAME => {
                        *cur = Cur::HostFrame(r);
                        return Ok(());
                    }
                    k => return Err(format!("EH record of unknown kind {k}")),
                }
            }
            Cur::Region(r) => {
                if ctx.rip != EMU_RET_SENTINEL {
                    return Ok(());
                }
                if (*r).flags & EH_F_OPAQUE != 0 {
                    return Err(format!("opaque host frame: {} was entered by a Rust API", (h.describe)(r)));
                }
                *cur = Cur::Visit((*r).next);
            }
            Cur::Lifted(_) | Cur::HostFrame(_) => return Ok(()),
        }
    }
    Err("EH record chain does not end".into())
}

/// Handler and handler data of a function's UNWIND_INFO (end of its chain) if its flags
/// include `handler_type`.
unsafe fn function_handler(base: u64, mut func: *const RuntimeFunction, handler_type: u32) -> (Option<u64>, u64) {
    loop {
        let info = base + (*func).unwind_rva as u64;
        let ver_flags: u8 = rd(info);
        let count: u8 = rd(info + 2);
        let hd = info + 4 + (((count as u64) + 1) & !1) * 2;
        let flags = (ver_flags >> 3) as u32;
        if flags & UNW_FLAG_CHAININFO != 0 {
            func = hd as *const RuntimeFunction;
            continue;
        }
        if flags & handler_type == 0 {
            return (None, 0);
        }
        let handler_rva: u32 = rd(hd);
        return (Some(base + handler_rva as u64), hd + 4);
    }
}

/// One frame of a walk over the record chain.
struct ChainFrame {
    handler: Option<u64>,
    handler_data: u64,
    establisher: u64,
    image_base: u64,
    func: *const RuntimeFunction,
    /// The caller's position.
    next: Context,
    next_cur: Cur,
}

/// [`step`] over the record chain. Settles `(ctx, cur)` in place (the frame's own context,
/// e.g. `rip` = ControlPc), then returns the frame and its caller's position. Without hooks
/// this is exactly `step` (cursor stays `Host`).
unsafe fn step_chain(handler_type: u32, ctx: &mut Context, cur: &mut Cur) -> Result<ChainFrame, String> {
    let legacy = |ctx: &Context| -> Result<ChainFrame, String> {
        let mut n = Context::zeroed();
        match step(handler_type, ctx, &mut n) {
            Some((handler, handler_data, establisher, image_base, func)) => Ok(ChainFrame {
                handler,
                handler_data,
                establisher,
                image_base,
                func,
                next: n,
                next_cur: Cur::Host,
            }),
            None => Err(format!(
                "frame walk stopped at {} (no unwind info; non-PE code such as a Rust replacement?)",
                crate::symbolize(ctx.rip as usize)
            )),
        }
    };
    let Some(h) = eh_hooks() else { return legacy(ctx) };
    settle(h, ctx, cur)?;
    match *cur {
        Cur::Host => legacy(ctx),
        Cur::Lifted(r) => {
            let li = (h.lifted)(r);
            let Some((base, func)) = lookup_function_entry(li.fn_va) else {
                return Err(format!("lifted EH frame 0x{:x} has no .pdata entry", li.fn_va));
            };
            let (handler, handler_data) = function_handler(base, func, handler_type);
            Ok(ChainFrame {
                handler,
                handler_data,
                establisher: li.establisher,
                image_base: base,
                func,
                next: *ctx,
                next_cur: Cur::Visit((*r).next),
            })
        }
        Cur::Region(r) => {
            let pc = ctx.rip;
            let mut n = *ctx;
            match lookup_function_entry(pc) {
                Some((base, func)) => {
                    let (handler, handler_data, establisher) = virtual_unwind(handler_type, base, pc, func, &mut n);
                    Ok(ChainFrame { handler, handler_data, establisher, image_base: base, func, next: n, next_cur: Cur::Region(r) })
                }
                None => {
                    // Leaf function (no `.pdata`, as for any guest code outside a PE image):
                    // return address on top of the stack.
                    n.rip = rd(ctx.gpr[RSP]);
                    n.gpr[RSP] += 8;
                    Ok(ChainFrame {
                        handler: None,
                        handler_data: 0,
                        establisher: ctx.gpr[RSP],
                        image_base: 0,
                        func: std::ptr::null(),
                        next: n,
                        next_cur: Cur::Region(r),
                    })
                }
            }
        }
        Cur::HostFrame(r) => {
            let x = r as *const EhHostFrame;
            Ok(ChainFrame {
                handler: Some((*x).handler as usize as u64),
                handler_data: (*x).data as u64,
                establisher: r as u64,
                image_base: 0,
                func: std::ptr::null(),
                next: *ctx,
                next_cur: Cur::Visit((*r).next),
            })
        }
        Cur::Visit(_) => Err("unsettled walk position".into()),
    }
}

/// Resume at `ctx` in the frame owned by `cur` (guest region / lifted frame: a landing;
/// host frame with a landing: that landing; host: [`restore_context`]).
pub unsafe fn resume_at(ctx: &Context, cur: Cur) -> ! {
    match (cur, eh_hooks()) {
        (Cur::Region(r) | Cur::Lifted(r), Some(h)) => (h.land)(r, ctx),
        (Cur::HostFrame(r), _) if (*(r as *const EhHostFrame)).land.is_some() => {
            ((*(r as *const EhHostFrame)).land.unwrap_unchecked())(r, ctx)
        }
        _ => restore_context(ctx),
    }
}

/// The calling guest frame's context if the innermost record is an emulated region whose
/// in-flight host call is `host_fn` (i.e. guest code called `host_fn` directly), with its
/// cursor; else `None`.
unsafe fn guest_caller_context(host_fn: usize, ctx: &mut Context) -> Option<Cur> {
    let h = eh_hooks()?;
    let head = (h.head)();
    if head.is_null() || (*head).kind != EH_EMU_ENTRY || (h.callout_fn)(head) != host_fn {
        return None;
    }
    let mut g = Context::zeroed();
    if !(h.callout)(head, &mut g) {
        return None;
    }
    *ctx = g;
    Some(Cur::Region(head))
}

/// The walk cursor of raise position `(ctx, cur)` as of now: a host pc outside PE code with
/// records in use starts at the innermost record. Resolved before the dispatcher links its
/// own crossing (which a later `Host` settle would otherwise find first, forever), and
/// stored in that crossing so walks resuming there see the chain as of the raise.
unsafe fn walk_start(ctx: &Context, cur: Cur) -> Cur {
    if cur != Cur::Host {
        return cur;
    }
    let Some(h) = eh_hooks() else { return cur };
    if ctx.rip == 0 || in_pe_image(ctx.rip) || lookup_function_entry(ctx.rip).is_some() {
        return cur;
    }
    let head = (h.head)();
    if head.is_null() {
        cur
    } else {
        Cur::Visit(head)
    }
}

/// Push a crossing record resuming at `(resume, cur)` (chain in use) or the legacy
/// host-marker crossing; returns what [`pop_crossing_rec`] needs.
unsafe fn push_crossing_rec(x: &mut EhCrossing, marker: u64) {
    match eh_hooks() {
        Some(h) => {
            x.link.next = (h.head)();
            (h.set_head)(&mut x.link);
        }
        None => push_crossing(marker, &x.resume),
    }
}

unsafe fn pop_crossing_rec(x: &mut EhCrossing, marker: u64) {
    match eh_hooks() {
        Some(_) => eh_unlink(&mut x.link),
        None => pop_crossing(marker),
    }
}

impl EhCrossing {
    fn new(resume: &Context, cur: Cur) -> Self {
        Self { link: EhLink { next: std::ptr::null_mut(), kind: EH_CROSSING, flags: 0 }, resume: *resume, cur }
    }
}

// ---------------------------------------------------------------------------
// Dispatch
// ---------------------------------------------------------------------------

pub type VectoredHandler = unsafe extern "win64" fn(*mut [*mut c_void; 2]) -> i32;
struct VehEntry {
    id: usize,
    first: bool,
    handler: VectoredHandler,
}
static VEH: Mutex<Vec<VehEntry>> = Mutex::new(Vec::new());
static VEH_NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0x1000);

pub unsafe extern "win64" fn AddVectoredExceptionHandler(first: u32, h: VectoredHandler) -> *mut c_void {
    let id = VEH_NEXT.fetch_add(16, std::sync::atomic::Ordering::Relaxed);
    let mut v = VEH.lock().unwrap();
    let e = VehEntry { id, first: first != 0, handler: h };
    if e.first {
        v.insert(0, e);
    } else {
        v.push(e);
    }
    id as *mut c_void
}

pub unsafe extern "win64" fn RemoveVectoredExceptionHandler(h: *mut c_void) -> u32 {
    let mut v = VEH.lock().unwrap();
    let n = v.len();
    v.retain(|e| e.id != h as usize);
    (v.len() != n) as u32
}

pub static UNHANDLED_FILTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

pub fn describe_exception(rec: &ExceptionRecord) -> String {
    let name = match rec.code {
        STATUS_ACCESS_VIOLATION => "ACCESS_VIOLATION",
        CXX_EXCEPTION => "C++ exception (0xE06D7363)",
        STATUS_BREAKPOINT => "BREAKPOINT",
        STATUS_ILLEGAL_INSTRUCTION => "ILLEGAL_INSTRUCTION",
        STATUS_STACK_BUFFER_OVERRUN => "STACK_BUFFER_OVERRUN/__fastfail",
        0xC06D_007E => "DELAYLOAD module not found (VcppException ERROR_MOD_NOT_FOUND)",
        0xC06D_007F => "DELAYLOAD proc not found (VcppException ERROR_PROC_NOT_FOUND)",
        _ => "",
    };
    let mut s = format!(
        "exception 0x{:08x} {} flags=0x{:x} at {} params=[",
        rec.code,
        name,
        rec.flags,
        crate::symbolize(rec.address as usize)
    );
    for i in 0..(rec.number_parameters.min(15) as usize) {
        s += &format!("{}0x{:x}", if i > 0 { ", " } else { "" }, rec.information[i]);
    }
    s.push(']');
    if rec.code == CXX_EXCEPTION && rec.number_parameters >= 3 {
        s += &cxx_exception_type(rec).map(|t| format!(" type={t}")).unwrap_or_default();
    }
    s
}

/// Best-effort name of the thrown C++ type (first catchable type's
/// TypeDescriptor name) for logging.
fn cxx_exception_type(rec: &ExceptionRecord) -> Option<String> {
    unsafe {
        let throw_info = rec.information[2];
        let image_base = if rec.number_parameters >= 4 { rec.information[3] } else { 0 };
        if throw_info == 0 || image_base == 0 {
            return None;
        }
        let cta_rva: i32 = rd(throw_info + 12);
        let cta = image_base + cta_rva as u64;
        let n: i32 = rd(cta);
        if n <= 0 {
            return None;
        }
        let ct_rva: i32 = rd(cta + 4);
        let ct = image_base + ct_rva as u64;
        let td_rva: i32 = rd(ct + 4);
        let td = image_base + td_rva as u64;
        let name = std::ffi::CStr::from_ptr((td + 16) as *const libc::c_char);
        Some(name.to_string_lossy().into_owned())
    }
}

/// Number of C++ exceptions dispatched so far (for the throw/catch log).
static CXX_THROWS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Log a C++ exception at dispatch: sequence number, thrown type and the
/// first frames of the throw site. Used to measure how often the game throws
/// during normal flow (docs/design/ADVICE_SEH.md §5 step 0).
unsafe fn log_cxx_throw(rec: &ExceptionRecord, ctx: &Context) {
    let n = CXX_THROWS.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
    let ty = cxx_exception_type(rec).unwrap_or_else(|| "?".into());
    let frames: Vec<String> = backtrace_from(ctx, 5).iter().map(|&pc| crate::symbolize(pc as usize)).collect();
    log::info!(
        target: "win32_rt::seh",
        "C++ throw #{n} type={ty} thread=0x{:x} at {}",
        crate::teb::thread_id(),
        frames.join(" <- ")
    );
}

/// Invoke a `VectoredHandler` that may be either a guest x86-64 VA or a host Rust fn.
#[inline]
unsafe fn invoke_veh(h: VectoredHandler, ptrs: *mut [*mut c_void; 2]) -> i32 {
    let addr = h as usize;
    #[cfg(not(target_arch = "x86_64"))]
    let is_guest = in_pe_image(addr as u64) || crate::kernel32::memory::is_guest_vm_addr(addr);
    #[cfg(target_arch = "x86_64")]
    let is_guest = in_pe_image(addr as u64);
    if is_guest {
        crate::call_guest8(addr, ptrs as u64, 0, 0, 0, 0, 0, 0, 0) as i32
    } else {
        h(ptrs)
    }
}

/// Address of the 8-byte `Wow64PrepareForException` pointer slot inside
/// `ntdll.dll`'s `.mrdata` section (referenced by `KiUserExceptionDispatcher`).
static WOW64_PREPARE_SLOT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

pub fn set_wow64_prepare_for_exception_slot(slot: usize) {
    WOW64_PREPARE_SLOT.store(slot, std::sync::atomic::Ordering::Release);
}

pub fn has_guest_exception_handler() -> bool {
    wow64_prepare_handler() != 0 || !VEH.lock().unwrap().is_empty()
}

thread_local! {
    static NTCONTINUE_STACK: std::cell::RefCell<Vec<Option<Context>>> = const { std::cell::RefCell::new(Vec::new()) };
}

struct NtContinueGuard;
impl Drop for NtContinueGuard {
    fn drop(&mut self) {
        NTCONTINUE_STACK.with(|s| {
            let _ = s.borrow_mut().pop();
        });
    }
}

/// If the current thread is inside a `KiUserExceptionDispatcher` /
/// `Wow64PrepareForException` callback, capture the target `Context` for
/// `NtContinue` so the exception dispatcher can resume the emulator cleanly
/// with the modified register state.
pub fn capture_ntcontinue(ctx: &Context) -> bool {
    NTCONTINUE_STACK.with(|s| {
        let mut st = s.borrow_mut();
        if let Some(top) = st.last_mut() {
            *top = Some(*ctx);
            true
        } else {
            false
        }
    })
}

fn wow64_prepare_handler() -> usize {
    let slot = WOW64_PREPARE_SLOT.load(std::sync::atomic::Ordering::Acquire);
    if slot == 0 {
        return 0;
    }
    unsafe { std::ptr::read_unaligned(slot as *const usize) }
}

unsafe fn try_dispatch_wow64_prepare(rec: &mut ExceptionRecord, ctx: &mut Context) -> bool {
    let handler = wow64_prepare_handler();
    if handler == 0 {
        return false;
    }
    NTCONTINUE_STACK.with(|s| s.borrow_mut().push(None));
    let guard = NtContinueGuard;
    let _ = crate::call_guest8(
        handler,
        rec as *mut _ as u64,
        ctx as *mut _ as u64,
        0,
        0,
        0,
        0,
        0,
        0,
    );
    let captured = NTCONTINUE_STACK.with(|s| s.borrow_mut().pop().flatten());
    std::mem::forget(guard);
    if let Some(new_ctx) = captured {
        *ctx = new_ctx;
        return true;
    }
    false
}

struct TrapJob {
    teb: *mut u8,
    f: Box<dyn FnOnce() + Send>,
}
unsafe impl Send for TrapJob {}

/// Per-thread helper pthread with its own `x86-runtime` `TLS_EMU` stack.
///
/// When `x86-runtime`'s interpreter hits an inline trap (`int 3` / `ud2`) mid-function,
/// its saved `*sp_ptr` can lag behind the live guest `cpu.gpr[RSP]`. Invoking a guest
/// exception handler (`Wow64PrepareForException`, VEH, SEH filter/`__finally`) on the
/// same OS thread would re-enter `call_guest8` using the stale `*sp_ptr` and clobber
/// the active guest stack frames (including return addresses needed for SEH unwinding).
/// Running guest handlers on this helper thread keeps the faulting thread's `TLS_EMU`
/// stack untouched while transferring its TEB for the duration of the call.
struct TrapHelper {
    req: std::sync::Arc<(std::sync::Mutex<Option<TrapJob>>, std::sync::Condvar)>,
    done: std::sync::Arc<(std::sync::Mutex<bool>, std::sync::Condvar)>,
}

impl TrapHelper {
    fn new() -> Self {
        let req = std::sync::Arc::new((std::sync::Mutex::new(None::<TrapJob>), std::sync::Condvar::new()));
        let done = std::sync::Arc::new((std::sync::Mutex::new(false), std::sync::Condvar::new()));
        let req_w = std::sync::Arc::clone(&req);
        let done_w = std::sync::Arc::clone(&done);
        std::thread::Builder::new()
            .name("win32-trap-dispatch".into())
            .stack_size(4 * 1024 * 1024)
            .spawn(move || loop {
                let job = {
                    let mut g = req_w.0.lock().unwrap();
                    loop {
                        if let Some(j) = g.take() {
                            break j;
                        }
                        g = req_w.1.wait(g).unwrap();
                    }
                };
                unsafe {
                    if !job.teb.is_null() {
                        crate::teb::install(job.teb);
                    }
                }
                (job.f)();
                unsafe {
                    if !job.teb.is_null() {
                        crate::teb::uninstall();
                    }
                }
                let mut dg = done_w.0.lock().unwrap();
                *dg = true;
                done_w.1.notify_one();
            })
            .expect("spawn win32-trap-dispatch helper");
        Self { req, done }
    }

    unsafe fn run<R: Send>(&self, f: impl FnOnce() -> R + Send) -> R {
        let mut out: Option<R> = None;
        let out_ptr = &mut out as *mut Option<R> as usize;
        // Safety: we block synchronously on `done` until `job.f` has completed.
        let job_fn: Box<dyn FnOnce() + Send> = unsafe {
            let b: Box<dyn FnOnce() + Send + '_> = Box::new(move || {
                let r = f();
                *(out_ptr as *mut Option<R>) = Some(r);
            });
            std::mem::transmute::<Box<dyn FnOnce() + Send + '_>, Box<dyn FnOnce() + Send + 'static>>(b)
        };
        let teb = crate::teb::current_raw();
        if !teb.is_null() {
            crate::teb::uninstall();
        }
        {
            let mut g = self.req.0.lock().unwrap();
            *g = Some(TrapJob { teb, f: job_fn });
            self.req.1.notify_one();
        }
        {
            let mut dg = self.done.0.lock().unwrap();
            while !*dg {
                dg = self.done.1.wait(dg).unwrap();
            }
            *dg = false;
        }
        if !teb.is_null() {
            crate::teb::install(teb);
        }
        out.expect("TrapHelper job completed without result")
    }
}

unsafe fn run_on_isolated_emu_stack<R: Send>(f: impl FnOnce() -> R + Send) -> R {
    thread_local! {
        static IN_HELPER: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
        static HELPER: std::cell::OnceCell<TrapHelper> = const { std::cell::OnceCell::new() };
    }
    if IN_HELPER.with(|c| c.get()) {
        return f();
    }
    IN_HELPER.with(|c| c.set(true));
    let res = HELPER.with(|h| h.get_or_init(TrapHelper::new).run(f));
    IN_HELPER.with(|c| c.set(false));
    res
}

/// Try to unwind an emulated guest exception to an enclosing MSVC C-SEH
/// (`__try` / `__except` via `__C_specific_handler`) frame using `.pdata`.
///
/// Walks the emulated guest stack from `*ctx` via [`virtual_unwind`], validates
/// each frame's `SCOPE_TABLE`, evaluates matching `__except` filters (`1`, or a
/// guest filter callback), runs any intermediate `__finally` handlers, and updates
/// `*ctx` to the catching frame's `rsp`/nonvolatile registers with
/// `ctx.rip = image_base + scope.target` and `ctx.gpr[RAX] = rec.code`.
unsafe fn try_dispatch_guest_seh(rec: &mut ExceptionRecord, ctx: &mut Context) -> bool {
    let mut walk = *ctx;
    for step in 0..64usize {
        let pc = walk.rip;
        if pc < 0x10000 || pc == 0xDEAD_BEEF_CAFE_0000 {
            break;
        }
        let Some((base, func)) = lookup_function_entry(pc) else {
            // Leaf function: return address sits at [rsp].
            if walk.gpr[RSP] < 0x10000 || walk.gpr[RSP] >= 0x8000_0000_0000 {
                break;
            }
            walk.rip = rd(walk.gpr[RSP]);
            walk.gpr[RSP] = walk.gpr[RSP].wrapping_add(8);
            continue;
        };
        let frame_ctx = walk;
        let f_begin = (*func).begin_rva;
        let f_end = (*func).end_rva;
        let (handler, handler_data, establisher) =
            virtual_unwind(UNW_FLAG_EHANDLER, base, pc, func, &mut walk);
        if handler.is_none() || handler_data == 0 {
            continue;
        }
        // Validate C-SEH SCOPE_TABLE layout (distinguishes __C_specific_handler
        // from __CxxFrameHandler3/4 FuncInfo).
        let count: u32 = rd(handler_data);
        if count == 0 || count > 128 {
            continue;
        }
        let mut valid_table = true;
        for i in 0..count as u64 {
            let e = handler_data + 4 + i * 16;
            let b: u32 = rd(e);
            let en: u32 = rd(e + 4);
            let tgt: u32 = rd(e + 12);
            if b < f_begin || b >= en || en > f_end || (tgt != 0 && (tgt < f_begin || tgt >= f_end)) {
                valid_table = false;
                break;
            }
        }
        if !valid_table {
            continue;
        }
        // For unwound return addresses (step > 0), MSVC scope end is exclusive
        // of the instruction after the call (`begin < rel_pc <= end`), while at
        // the faulting instruction itself `begin <= rel_pc < end`. Check the
        // instruction inside the call (`rel_pc - 1` when step > 0).
        let rel_raw = (pc - base) as u32;
        let rel_pc = if step > 0 { rel_raw.saturating_sub(1) } else { rel_raw };
        for i in 0..count as u64 {
            let e = handler_data + 4 + i * 16;
            let b: u32 = rd(e);
            let en: u32 = rd(e + 4);
            let flt: u32 = rd(e + 8);
            let tgt: u32 = rd(e + 12);
            if tgt == 0 || rel_pc < b || rel_pc >= en {
                continue;
            }
            let verdict: i32 = if flt == 1 {
                1
            } else if flt == 0 {
                0
            } else {
                let mut ep: [*mut c_void; 2] = [rec as *mut _ as *mut c_void, ctx as *mut _ as *mut c_void];
                let ep_addr = ep.as_mut_ptr() as u64;
                let flt_addr = (base + flt as u64) as usize;
                run_on_isolated_emu_stack(move || {
                    crate::call_guest8(flt_addr, ep_addr, establisher, 0, 0, 0, 0, 0, 0) as i32
                })
            };
            if verdict == -1 {
                // EXCEPTION_CONTINUE_EXECUTION
                return true;
            }
            if verdict > 0 {
                // Run any inner __finally handlers in frames below `establisher`.
                let mut uw = *ctx;
                for ustep in 0..step {
                    let upc = uw.rip;
                    let Some((ubase, ufunc)) = lookup_function_entry(upc) else {
                        uw.rip = rd(uw.gpr[RSP]);
                        uw.gpr[RSP] = uw.gpr[RSP].wrapping_add(8);
                        continue;
                    };
                    let uf_begin = (*ufunc).begin_rva;
                    let uf_end = (*ufunc).end_rva;
                    let (uh, uhd, uest) = virtual_unwind(UNW_FLAG_UHANDLER, ubase, upc, ufunc, &mut uw);
                    if uh.is_some() && uhd != 0 {
                        let uc: u32 = rd(uhd);
                        if uc > 0 && uc <= 128 {
                            let urel = if ustep > 0 { ((upc - ubase) as u32).saturating_sub(1) } else { (upc - ubase) as u32 };
                            for j in 0..uc as u64 {
                                let ue = uhd + 4 + j * 16;
                                let ub: u32 = rd(ue);
                                let uen: u32 = rd(ue + 4);
                                let uflt: u32 = rd(ue + 8);
                                let utgt: u32 = rd(ue + 12);
                                if utgt == 0 && uflt != 0 && ub >= uf_begin && ub < uen && uen <= uf_end && urel >= ub && urel < uen {
                                    let fin_addr = (ubase + uflt as u64) as usize;
                                    run_on_isolated_emu_stack(move || {
                                        let _ = crate::call_guest8(fin_addr, 1, uest, 0, 0, 0, 0, 0, 0);
                                    });
                                }
                            }
                        }
                    }
                }
                *ctx = frame_ctx;
                ctx.rip = base + tgt as u64;
                ctx.gpr[RAX] = rec.code as u64;
                log::debug!(
                    "emu: C-SEH caught {} -> landing at {:#x} {} (rsp={:#x})",
                    describe_exception(rec),
                    ctx.rip,
                    crate::symbolize(ctx.rip as usize),
                    ctx.gpr[RSP]
                );
                return true;
            }
        }
    }
    false
}

/// Try to dispatch an emulator trap (`int 3` -> `STATUS_BREAKPOINT`,
/// `ud2` -> `STATUS_ILLEGAL_INSTRUCTION`) to `KiUserExceptionDispatcher`'s
/// `Wow64PrepareForException` hook, registered Vectored Exception Handlers, and
/// enclosing `.pdata` C-SEH (`__try` / `__except`) frames.
///
/// If handled, this updates `gpr`, `xmm`, and returns `Some((new_rip, new_eflags))`.
pub unsafe fn try_dispatch_veh_trap(
    code: u32,
    trap_rip: u64,
    gpr: &mut [u64; 16],
    xmm: &mut [u128; 16],
    eflags: u32,
) -> Option<(u64, u32)> {
    let mut rec = guest_exception_record(code, trap_rip, &[]);
    let mut ctx = guest_context(gpr, xmm, trap_rip, eflags);
    let veh_handled = if has_guest_exception_handler() {
        let rec_addr = &mut rec as *mut ExceptionRecord as usize;
        let ctx_addr = &mut ctx as *mut Context as usize;
        run_on_isolated_emu_stack(move || dispatch_veh_guest(&mut *(rec_addr as *mut ExceptionRecord), &mut *(ctx_addr as *mut Context)))
    } else {
        false
    };
    if veh_handled || try_dispatch_guest_seh(&mut rec, &mut ctx) {
        *gpr = ctx.gpr;
        for i in 0..16 {
            xmm[i] = u128::from_le_bytes(*ctx.xmm_mut(i));
        }
        return Some((ctx.rip, ctx.eflags));
    }
    None
}

/// Exception record of an emulated guest exception at `addr`. `STATUS_BREAKPOINT` gets
/// its one (zero) parameter; `info` supplies the rest (access violation: kind, address).
pub fn guest_exception_record(code: u32, addr: u64, info: &[u64]) -> ExceptionRecord {
    let mut rec = ExceptionRecord {
        code,
        flags: 0,
        record: std::ptr::null_mut(),
        address: addr as usize as *mut c_void,
        number_parameters: if code == STATUS_BREAKPOINT { 1 } else { 0 },
        _pad: 0,
        information: [0; 15],
    };
    if !info.is_empty() {
        let n = info.len().min(15);
        rec.information[..n].copy_from_slice(&info[..n]);
        rec.number_parameters = n as u32;
    }
    rec
}

/// Guest x86-64 `CONTEXT` built from interpreter registers.
pub fn guest_context(gpr: &[u64; 16], xmm: &[u128; 16], rip: u64, eflags: u32) -> Context {
    let mut ctx = Context::zeroed();
    ctx.context_flags = CONTEXT_FULL;
    ctx.mx_csr = 0x1F80;
    ctx.seg_cs = 0x33;
    ctx.seg_ss = 0x2B;
    ctx.eflags = eflags;
    ctx.dr = [0; 6]; // Clean hardware debug registers
    ctx.gpr = *gpr;
    ctx.rip = rip;
    for i in 0..16 {
        *ctx.xmm_mut(i) = xmm[i].to_le_bytes();
    }
    ctx
}

/// Offer an emulated guest exception to `Wow64PrepareForException` (if hooked)
/// and the vectored handlers. `true`: handled (`NtContinue` or
/// `EXCEPTION_CONTINUE_EXECUTION`) and `ctx` holds the state to resume.
pub unsafe fn dispatch_veh_guest(rec: &mut ExceptionRecord, ctx: &mut Context) -> bool {
    if try_dispatch_wow64_prepare(rec, ctx) {
        return true;
    }
    let vehs: Vec<VectoredHandler> = VEH.lock().unwrap().iter().map(|e| e.handler).collect();
    for h in vehs {
        let mut ptrs: [*mut c_void; 2] = [rec as *mut _ as *mut c_void, ctx as *mut _ as *mut c_void];
        if invoke_veh(h, &mut ptrs) == -1 {
            return true;
        }
    }
    false
}

/// An emulated guest exception that no vectored handler took (`int 3`, `ud2`, an access
/// outside the user address space), handled as Windows does after VEH:
/// - Frame-based (SEH) handlers are not run: the guest frames live on the interpreter's
///   stack, and neither a handler's continuation nor its unwind target can be resumed
///   inside the interpreter. This is logged.
/// - ERROR log with the guest `rip`, the guest stack walk (`.pdata`) and the host backtrace,
///   then the crash report, then the application's unhandled-exception filter.
/// - The filter returning `EXCEPTION_CONTINUE_EXECUTION` resumes with `ctx` (returns `true`)
///   if `resumable` (the interpreter can restart at `ctx.rip`); otherwise the process exits
///   with the exception code, as an unhandled exception does on Windows.
pub unsafe fn emu_unhandled_exception(rec: &mut ExceptionRecord, ctx: &mut Context, resumable: bool) -> bool {
    if resumable && try_dispatch_guest_seh(rec, ctx) {
        return true;
    }
    if crate::shadow::in_child() || crate::shadow::heap_on() {
        // Shadow-oracle probe child: no crash report, filter or exit hooks. Fault for real
        // so the run is recorded as faulted (an access violation at its own address).
        let a = if rec.code == STATUS_ACCESS_VIOLATION && rec.number_parameters >= 2 { rec.information[1] } else { 0 };
        core::ptr::read_volatile(a.max(0x10) as usize as *const u8);
        libc::_exit(134);
    }
    log::error!(
        "emu: UNHANDLED guest {} at rip {:#x} {} (no vectored handler took it; frame-based handlers are \
         not run for emulated frames)",
        describe_exception(rec),
        ctx.rip,
        crate::symbolize(ctx.rip as usize)
    );
    log::error!("emu: guest stack walk:");
    log_backtrace(ctx);
    log::error!("emu: host backtrace:\n{}", std::backtrace::Backtrace::force_capture());
    crate::crash::report("unhandled emulated guest exception", Some(rec), ctx);
    let filt = UNHANDLED_FILTER.load(std::sync::atomic::Ordering::Relaxed);
    if filt != 0 && std::env::var_os("AW2_SKIP_UNHANDLED_FILTER").is_none() {
        log::error!("calling the application's unhandled exception filter at {}", crate::symbolize(filt));
        let f: VectoredHandler = std::mem::transmute(filt);
        let mut ptrs: [*mut c_void; 2] = [rec as *mut _ as *mut c_void, ctx as *mut _ as *mut c_void];
        let r = invoke_veh(f, &mut ptrs);
        if r == -1 {
            if resumable {
                log::error!("emu: unhandled exception filter continued execution at {:#x}", ctx.rip);
                return true;
            }
            log::error!("emu: unhandled exception filter asked to continue; not resumable here, exiting");
        }
    }
    crate::process::exit_process(rec.code)
}

/// Dispatch an exception through vectored handlers and frame-based handlers.
/// Returns only if the exception is continued (in which case the caller
/// restores `ctx`). Unhandled exceptions terminate the process.
pub unsafe fn dispatch_exception(rec: &mut ExceptionRecord, ctx: &mut Context) {
    dispatch_exception_at(rec, ctx, Cur::Host)
}

/// [`dispatch_exception`] from a raise position `(ctx, cur)`. A continued exception whose
/// raise context is a guest or lifted frame (`cur` not `Host`) resumes there itself
/// ([`resume_at`]); with `Host` it returns and the caller restores `ctx`.
pub unsafe fn dispatch_exception_at(rec: &mut ExceptionRecord, ctx: &mut Context, cur: Cur) {
    log::debug!("dispatch: {}", describe_exception(rec));
    if rec.code == CXX_EXCEPTION && rec.number_parameters >= 3 {
        log_cxx_throw(rec, ctx);
    }
    // Lets an RtlUnwindEx called from a language handler walk from our (host)
    // dispatcher frames back to the raise site (see `cross_host_frames` / `settle`).
    let anchor = 0u64;
    let marker = std::hint::black_box(&anchor as *const u64 as u64);
    let walk_cur = walk_start(ctx, cur);
    let mut xr = EhCrossing::new(ctx, walk_cur);
    push_crossing_rec(&mut xr, marker);
    let continued = |xr: &mut EhCrossing, ctx: &Context| {
        pop_crossing_rec(xr, marker);
        if cur != Cur::Host {
            resume_at(ctx, cur);
        }
    };
    // Vectored handlers.
    let vehs: Vec<VectoredHandler> = VEH.lock().unwrap().iter().map(|e| e.handler).collect();
    for h in vehs {
        let mut ptrs: [*mut c_void; 2] = [rec as *mut _ as *mut c_void, ctx as *mut _ as *mut c_void];
        if invoke_veh(h, &mut ptrs) == -1 {
            // EXCEPTION_CONTINUE_EXECUTION
            continued(&mut xr, ctx);
            restore_context(ctx);
        }
    }
    let teb = crate::teb::current();
    let (stack_lo, stack_hi) = if teb.is_null() {
        (0, 0)
    } else {
        (
            crate::teb::field::<usize>(teb, crate::teb::off::STACK_LIMIT) as u64,
            crate::teb::field::<usize>(teb, crate::teb::off::STACK_BASE) as u64,
        )
    };
    let mut cur_ctx = *ctx;
    let mut cur_pos = walk_cur;
    let mut nested_frame = 0u64;
    loop {
        let f = match step_chain(UNW_FLAG_EHANDLER, &mut cur_ctx, &mut cur_pos) {
            Ok(f) => f,
            Err(why) => {
                log::warn!(target: "win32_rt::seh", "dispatch: frame walk stopped: {why}");
                break;
            }
        };
        let frame = f.establisher;
        if !matches!(cur_pos, Cur::HostFrame(_)) && !frame_ok(frame, stack_lo, stack_hi) {
            log::error!("dispatch: establisher frame 0x{frame:x} outside stack [0x{stack_lo:x},0x{stack_hi:x}]");
            rec.flags |= EXCEPTION_STACK_INVALID;
            break;
        }
        if let Some(h) = f.handler {
            let mut dc = DispatcherContext {
                control_pc: cur_ctx.rip,
                image_base: f.image_base,
                function_entry: f.func,
                establisher_frame: frame,
                target_ip: 0,
                context_record: &mut cur_ctx,
                language_handler: h as *const c_void,
                handler_data: f.handler_data as *mut c_void,
                history_table: std::ptr::null_mut(),
                scope_index: 0,
                fill0: 0,
            };
            let res = if in_pe_image(h) {
                crate::call_guest8_seh(
                    h as usize,
                    rec as *mut _ as u64,
                    frame,
                    &mut cur_ctx as *mut _ as u64,
                    &mut dc as *mut _ as u64,
                    0, 0, 0, 0,
                ) as u32
            } else {
                let hf: LanguageHandler = std::mem::transmute(h as usize);
                hf(rec, frame, &mut cur_ctx, &mut dc)
            };
            if nested_frame == frame {
                rec.flags &= !EXCEPTION_NESTED_CALL;
                nested_frame = 0;
            }
            match res {
                EXCEPTION_CONTINUE_EXECUTION_DISP => {
                    if rec.flags & EXCEPTION_NONCONTINUABLE != 0 {
                        log::error!("handler tried to continue a non-continuable exception");
                        fatal_unhandled(rec, ctx);
                    }
                    continued(&mut xr, ctx);
                    return;
                }
                EXCEPTION_CONTINUE_SEARCH_DISP => {}
                EXCEPTION_NESTED_EXCEPTION_DISP => {
                    rec.flags |= EXCEPTION_NESTED_CALL;
                    nested_frame = dc.establisher_frame;
                }
                EXCEPTION_COLLIDED_UNWIND_DISP => {
                    log::warn!("dispatch: collided unwind; continuing search from dispatcher context");
                    cur_ctx = *dc.context_record;
                    continue;
                }
                other => {
                    log::error!("handler returned invalid disposition {other}");
                    fatal_unhandled(rec, ctx);
                }
            }
        }
        if f.next.rip == 0 {
            break;
        }
        cur_ctx = f.next;
        cur_pos = f.next_cur;
    }
    pop_crossing_rec(&mut xr, marker);
    if cur == Cur::Host && !in_pe_image(ctx.rip) && rec.code != CXX_EXCEPTION {
        let mut fence_ctx = Context::zeroed();
        if cross_host_frames(ctx, &mut fence_ctx) {
            log::warn!(
                target: "win32_rt::seh",
                "caught unhandled host fault ({}) at {}; recovering to fence shim",
                describe_exception(rec),
                crate::symbolize(ctx.rip as usize)
            );
            fence_ctx.gpr[RAX] = 0;
            restore_context(&fence_ctx);
        }
    }
    fatal_unhandled(rec, ctx);
}

unsafe fn fatal_unhandled(rec: &mut ExceptionRecord, ctx: &mut Context) -> ! {
    let hook = match rec.code {
        CXX_EXCEPTION => &CXX_THROW_HOOK,
        STATUS_ACCESS_VIOLATION => &UNHANDLED_AV_HOOK,
        _ => &NO_HOOK,
    };
    let h = hook.load(std::sync::atomic::Ordering::Relaxed);
    if h != 0 {
        let f: fn(&ExceptionRecord) = std::mem::transmute(h);
        f(rec);
    }
    let filt = UNHANDLED_FILTER.load(std::sync::atomic::Ordering::Relaxed);
    log::error!("UNHANDLED {}", describe_exception(rec));
    crate::crash::report("unhandled exception", Some(rec), ctx);
    if filt != 0 && std::env::var_os("AW2_SKIP_UNHANDLED_FILTER").is_none() {
        log::error!("calling the application's unhandled exception filter at {}", crate::symbolize(filt));
        let f: VectoredHandler = std::mem::transmute(filt);
        let mut ptrs: [*mut c_void; 2] = [rec as *mut _ as *mut c_void, ctx as *mut _ as *mut c_void];
        let r = invoke_veh(f, &mut ptrs);
        if r == -1 {
            restore_context(ctx);
        }
    }
    crate::process::exit_process(rec.code)
}

pub unsafe fn log_backtrace(ctx: &Context) {
    let mut cur = *ctx;
    let mut nc = Context::zeroed();
    for i in 0..64 {
        log::error!("  #{i:<2} {}", crate::symbolize(cur.rip as usize));
        match step(UNW_FLAG_NHANDLER, &cur, &mut nc) {
            Some(_) if nc.rip != 0 && nc.gpr[RSP] > cur.gpr[RSP] => cur = nc,
            _ => break,
        }
    }
}

pub unsafe fn backtrace_from(ctx: &Context, max: usize) -> Vec<u64> {
    let mut out = Vec::new();
    let mut cur = *ctx;
    let mut nc = Context::zeroed();
    while out.len() < max {
        out.push(cur.rip);
        match step(UNW_FLAG_NHANDLER, &cur, &mut nc) {
            Some(_) if nc.rip != 0 && nc.gpr[RSP] > cur.gpr[RSP] => cur = nc,
            _ => break,
        }
    }
    out
}

/// Return addresses of the frames from walk position `(ctx, cur)` outward, at most `max`;
/// frame 0 is `ctx`'s own frame (`RtlCaptureStackBackTrace` order). With EH records in use
/// the walk follows the record chain ([`step_chain`]), as an exception dispatch does:
/// - an emulated region frame by frame; its `RET_SENTINEL` is no frame, the walk continues
///   at the next older record (the caller of that `emu_run_full`);
/// - a lifted frame with an `EH_LIFTED` record at the return address of its in-flight call
///   (end of the call instruction at its state IP, [`EhHooks::insn_end`]);
/// - an [`EhHostFrame`] at its stand-in return address if it names one, else nothing;
/// - Rust frames and lifted frames without a record are not visible (no unwind info).
///
/// Without records (or with none on this thread) this is the `.pdata` walk of
/// [`backtrace_from`].
pub unsafe fn backtrace_chain(ctx: &Context, cur: Cur, max: usize) -> Vec<u64> {
    let Some(h) = eh_hooks() else { return backtrace_from(ctx, max) };
    let mut out = Vec::new();
    let mut c = *ctx;
    let mut cur = walk_start(&c, cur);
    for _ in 0..100_000 {
        if out.len() >= max {
            break;
        }
        let Ok(f) = step_chain(UNW_FLAG_NHANDLER, &mut c, &mut cur) else { break };
        match cur {
            Cur::Host | Cur::Region(_) => {
                if c.rip == 0 {
                    break;
                }
                out.push(c.rip);
            }
            Cur::Lifted(_) => out.push((h.insn_end)(c.rip)),
            Cur::HostFrame(r) => {
                let pc = (*(r as *const EhHostFrame)).pc;
                if pc != 0 {
                    out.push(pc);
                }
            }
            Cur::Visit(_) => break,
        }
        // Frames walked by unwind info must move outward (a corrupt frame ends the walk).
        let same_stack = matches!((cur, f.next_cur), (Cur::Host, Cur::Host) | (Cur::Region(_), Cur::Region(_)));
        if same_stack && f.next.gpr[RSP] <= c.gpr[RSP] {
            break;
        }
        c = f.next;
        cur = f.next_cur;
    }
    out
}

// ---------------------------------------------------------------------------
// RaiseException
// ---------------------------------------------------------------------------

context_capturing_export!(RaiseException => raise_exception_impl);

/// Cross-architecture Rust helper to invoke [`RaiseException`] with 4 Win64 parameters.
///
/// On `x86_64`, [`RaiseException`] is a `#[naked]` asm entry point with 0 Rust parameters
/// that reads `rcx`/`rdx`/`r8`/`r9` directly from registers; on non-`x86_64`, it is a
/// regular 6-argument `extern "win64"` function.
#[inline(never)]
pub unsafe fn raise_exception_raw(code: u64, flags: u64, nargs: u64, args_ptr: u64) {
    #[cfg(target_arch = "x86_64")]
    {
        let f: unsafe extern "win64" fn(u64, u64, u64, u64) =
            std::mem::transmute(RaiseException as *const ());
        f(code, flags, nargs, args_ptr);
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        RaiseException(code, flags, nargs, args_ptr, 0, 0);
    }
}

/// `fn(&ExceptionRecord)` called for an MSVC C++ exception (`0xE06D7363`) the dispatch
/// could not deliver (the walk stopped at a frame without a record, e.g. a Rust API calling
/// a guest callback, or no frame caught it), before the unhandled-exception path; or 0. Set
/// only by the dll_lift difftest child, which records the throw ("threw_unrecorded") and
/// exits; a C++ exception that reaches its harness frame is caught there instead.
pub static CXX_THROW_HOOK: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// `fn(&ExceptionRecord)` called for a STATUS_ACCESS_VIOLATION no frame handled, before the
/// unhandled-exception path; or 0. Set only by the dll_lift difftest child (precise guest
/// faults / guarded lifted accesses, g18 seh_guard_plan.md), which records it as the fault
/// it is natively and exits.
pub static UNHANDLED_AV_HOOK: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
static NO_HOOK: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

unsafe extern "win64" fn raise_exception_impl(ctx: *mut Context, cur: usize) -> u64 {
    let ctx = &mut *ctx;
    let code = ctx.gpr[RCX] as u32;
    let flags = ctx.gpr[RDX] as u32;
    let nargs = (ctx.gpr[R8] as u32).min(15);
    let args = ctx.gpr[R9] as *const u64;
    let mut rec = ExceptionRecord {
        code,
        flags: flags & EXCEPTION_NONCONTINUABLE,
        record: std::ptr::null_mut(),
        address: ctx.rip as *mut c_void,
        number_parameters: nargs,
        _pad: 0,
        information: [0; 15],
    };
    if !args.is_null() {
        for i in 0..nargs as usize {
            rec.information[i] = *args.add(i);
        }
    }
    match code {
        MS_VC_EXCEPTION_SET_THREAD_NAME if nargs >= 3 && (rec.information[0] as u32) == 0x1000 => {
            // THREADNAME_INFO { dwType=0x1000, szName, dwThreadID, dwFlags }.
            // On Windows without a debugger the raiser's __except swallows it;
            // returning directly is equivalent and avoids a full dispatch.
            let name = crate::strings::from_ansi(rec.information[1] as *const u8);
            log::debug!("thread 0x{:x} named {:?} (MS_VC_EXCEPTION)", rec.information[2] as u32, name);
            if let Some(t) = crate::thread::current_thread_object() {
                if rec.information[2] as u32 == u32::MAX || rec.information[2] as u32 == crate::teb::thread_id() {
                    *t.name.lock().unwrap() = name;
                }
            }
            return 0;
        }
        DBG_PRINTEXCEPTION_C | DBG_PRINTEXCEPTION_WIDE_C => {
            // OutputDebugString via exception: no debugger attached -> handled.
            return 0;
        }
        _ => {}
    }
    log::debug!("RaiseException {} from {}", describe_exception(&rec), crate::symbolize(ctx.rip as usize));
    let mut orig = *ctx;
    dispatch_exception_at(&mut rec, &mut orig, cur_from_raw(cur));
    // Continued: resume at the caller with the (possibly modified) context.
    restore_context(&orig)
}

// ---------------------------------------------------------------------------
// RtlUnwindEx / RtlUnwind
// ---------------------------------------------------------------------------

context_capturing_export!(RtlUnwindEx => rtl_unwind_ex_impl);
context_capturing_export!(RtlUnwind => rtl_unwind_impl);

unsafe extern "win64" fn rtl_unwind_impl(ctx: *mut Context, cur: usize) -> u64 {
    // RtlUnwind(TargetFrame, TargetIp, ExceptionRecord, ReturnValue)
    let c = &*ctx;
    let mut scratch = Context::zeroed();
    do_unwind(c, cur_from_raw(cur), c.gpr[RCX], c.gpr[RDX], c.gpr[R8] as *mut ExceptionRecord, c.gpr[R9], &mut scratch)
}

unsafe extern "win64" fn rtl_unwind_ex_impl(ctx: *mut Context, cur: usize) -> u64 {
    // RtlUnwindEx(TargetFrame, TargetIp, ExceptionRecord, ReturnValue, OriginalContext, HistoryTable)
    let c = &*ctx;
    #[cfg(target_arch = "x86_64")]
    let orig = rd::<u64>(c.gpr[RSP] + 0x20) as *mut Context; // 5th arg
    #[cfg(not(target_arch = "x86_64"))]
    let orig = c.gpr[R10] as *mut Context;
    let mut scratch = Context::zeroed();
    let out = if orig.is_null() { &mut scratch } else { &mut *orig };
    do_unwind(c, cur_from_raw(cur), c.gpr[RCX], c.gpr[RDX], c.gpr[R8] as *mut ExceptionRecord, c.gpr[R9], out)
}

unsafe fn do_unwind(
    start: &Context,
    start_cur: Cur,
    end_frame: u64,
    target_ip: u64,
    rec_in: *mut ExceptionRecord,
    retval: u64,
    context: &mut Context,
) -> u64 {
    let mut local_rec;
    let rec: &mut ExceptionRecord = if rec_in.is_null() {
        local_rec = ExceptionRecord {
            code: STATUS_UNWIND,
            flags: 0,
            record: std::ptr::null_mut(),
            address: start.rip as *mut c_void,
            number_parameters: 0,
            _pad: 0,
            information: [0; 15],
        };
        &mut local_rec
    } else {
        &mut *rec_in
    };
    let base_flags = EXCEPTION_UNWINDING | if end_frame == 0 { EXCEPTION_EXIT_UNWIND } else { 0 };
    log::trace!("RtlUnwindEx target_frame=0x{end_frame:x} target_ip={}", crate::symbolize(target_ip as usize));
    *context = *start;
    let mut pos = start_cur;
    let teb = crate::teb::current();
    let (stack_lo, stack_hi) = if teb.is_null() {
        (0, 0)
    } else {
        (
            crate::teb::field::<usize>(teb, crate::teb::off::STACK_LIMIT) as u64,
            crate::teb::field::<usize>(teb, crate::teb::off::STACK_BASE) as u64,
        )
    };
    loop {
        rec.flags = (rec.flags & !(EXCEPTION_TARGET_UNWIND | EXCEPTION_COLLIDED_UNWIND)) | base_flags;
        let f = match step_chain(UNW_FLAG_UHANDLER, context, &mut pos) {
            Ok(f) => f,
            Err(why) => {
                log::error!("RtlUnwindEx: walked out of Windows code before reaching frame 0x{end_frame:x}: {why}");
                log_backtrace(start);
                crate::process::exit_process(0xC000_0027);
            }
        };
        let (handler, hdata, frame, base, func) = (f.handler, f.handler_data, f.establisher, f.image_base, f.func);
        // Frame addresses only grow monotonically within one stack: across emulated regions
        // and lifted frames (emulator stack, lstack arena) the order comes from the records.
        if end_frame != 0 && frame > end_frame && pos == Cur::Host {
            log::error!("RtlUnwindEx: frame 0x{frame:x} passed target 0x{end_frame:x}");
            crate::process::exit_process(0xC000_0028); // STATUS_BAD_STACK
        }
        if !matches!(pos, Cur::HostFrame(_)) && !frame_ok(frame, stack_lo, stack_hi) {
            log::error!("RtlUnwindEx: frame 0x{frame:x} outside stack");
            crate::process::exit_process(0xC000_0028);
        }
        if frame == end_frame {
            rec.flags |= EXCEPTION_TARGET_UNWIND;
            if rec.code == CXX_EXCEPTION && rec.number_parameters >= 3 {
                let fn_start = if func.is_null() { 0 } else { base + (*func).begin_rva as u64 };
                log::info!(
                    target: "win32_rt::seh",
                    "C++ catch type={} by {} (function {}) frame=0x{frame:x} continuation={}",
                    cxx_exception_type(rec).unwrap_or_else(|| "?".into()),
                    crate::symbolize(context.rip as usize),
                    crate::symbolize(fn_start as usize),
                    crate::symbolize(target_ip as usize)
                );
            }
        }
        let mut next = f.next;
        let mut next_cur = f.next_cur;
        if let Some(h) = handler {
            let mut dc = DispatcherContext {
                control_pc: context.rip,
                image_base: base,
                function_entry: func,
                establisher_frame: frame,
                target_ip,
                context_record: context,
                language_handler: h as *const c_void,
                handler_data: hdata as *mut c_void,
                history_table: std::ptr::null_mut(),
                scope_index: 0,
                fill0: 0,
            };
            let res = if in_pe_image(h) {
                crate::call_guest8_seh(
                    h as usize,
                    rec as *mut _ as u64,
                    frame,
                    context as *mut _ as u64,
                    &mut dc as *mut _ as u64,
                    0, 0, 0, 0,
                ) as u32
            } else {
                let hf: LanguageHandler = std::mem::transmute(h as usize);
                hf(rec, frame, context, &mut dc)
            };
            match res {
                EXCEPTION_CONTINUE_SEARCH_DISP => {}
                EXCEPTION_COLLIDED_UNWIND_DISP => {
                    log::warn!("RtlUnwindEx: collided unwind");
                    *context = *dc.context_record;
                    next = *context;
                    next_cur = pos;
                    let _ = step(UNW_FLAG_NHANDLER, context, &mut next);
                }
                other => {
                    log::error!("RtlUnwindEx: invalid disposition {other}");
                    crate::process::exit_process(STATUS_INVALID_DISPOSITION);
                }
            }
        }
        if frame == end_frame {
            break;
        }
        *context = next;
        pos = next_cur;
    }
    context.gpr[RAX] = retval;
    context.rip = target_ip;
    if rec.code == STATUS_LONGJMP && rec.number_parameters >= 1 && rec.information[0] != 0 {
        restore_jump_buffer(context, rec.information[0]);
    }
    consolidate(context, pos, rec);
    if pos == Cur::Host {
        abandon_crossings_below(context.gpr[RSP]);
    }
    resume_at(context, pos)
}

/// x64 `_JUMP_BUFFER`: `Frame, Rbx, Rsp, Rbp, Rsi, Rdi, R12..R15, Rip` (u64), `MxCsr` (u32),
/// `FpCsr`, `Spare` (u16), `Xmm6..Xmm15` (16 bytes each).
const JB_RIP: usize = 10;
const JB_MXCSR: usize = 88;
const JB_XMM6: usize = 96;

/// Set the nonvolatile registers, rsp, rip and mxcsr of `context` from the jump buffer at
/// `jb`, and mark it [`CONTEXT_AW2_LONGJMP`].
unsafe fn restore_jump_buffer(context: &mut Context, jb: u64) {
    let q = |i: usize| rd::<u64>(jb + 8 * i as u64);
    for (i, r) in [(1, RBX), (2, RSP), (3, RBP), (4, RSI), (5, RDI), (6, 12), (7, 13), (8, 14), (9, 15)] {
        context.gpr[r] = q(i);
    }
    context.rip = q(JB_RIP);
    context.mx_csr = rd::<u32>(jb + JB_MXCSR as u64);
    for k in 0..10 {
        let x: [u8; 16] = rd(jb + (JB_XMM6 + 16 * k) as u64);
        *context.xmm_mut(6 + k) = x;
    }
    context.context_flags |= CONTEXT_AW2_LONGJMP;
}

pub const STATUS_UNWIND_CONSOLIDATE: u32 = 0x8000_0029;

/// `STATUS_UNWIND_CONSOLIDATE` (Windows `RtlRestoreContext`): before resuming at
/// `context`, call `rec.ExceptionInformation[0](rec)` and resume at the address
/// it returns. MSVC's FH4 uses this for every catch: `UnwindNestedFrames` calls
/// `RtlUnwindEx` with `__CxxCallCatchBlock` as the callback, which runs the catch
/// funclet and returns the continuation address.
///
/// The callback runs on the current (deep) stack, so the thrower's frames, and the
/// exception object in them, stay intact while the catch block runs. Windows
/// gives the callback a fake frame (`RcConsolidateFrames`) whose unwind leads to
/// the target frame. Here the same is a crossing record that resumes at
/// `(context, cur)`, so a rethrow from the catch block skips the frames already unwound.
unsafe fn consolidate(context: &mut Context, cur: Cur, rec: &mut ExceptionRecord) {
    if rec.code != STATUS_UNWIND_CONSOLIDATE || rec.number_parameters < 1 || rec.information[0] == 0 {
        return;
    }
    let cb_addr = rec.information[0];
    let anchor = 0u64;
    let marker = std::hint::black_box(&anchor as *const u64 as u64);
    let mut xr = EhCrossing::new(context, cur);
    push_crossing_rec(&mut xr, marker);
    log::trace!(target: "win32_rt::seh", "consolidate: calling {}", crate::symbolize(cb_addr as usize));
    let ip = if in_pe_image(cb_addr) {
        crate::call_guest8_seh(cb_addr as usize, rec as *mut _ as u64, 0, 0, 0, 0, 0, 0, 0)
    } else {
        let cb: unsafe extern "win64" fn(*mut ExceptionRecord) -> u64 = std::mem::transmute(cb_addr as usize);
        cb(rec)
    };
    pop_crossing_rec(&mut xr, marker);
    context.rip = ip;
}

// ---------------------------------------------------------------------------
// Stack backtrace
// ---------------------------------------------------------------------------

context_capturing_export!(RtlCaptureStackBackTrace => capture_backtrace_impl);

unsafe extern "win64" fn capture_backtrace_impl(ctx: *mut Context, cur: usize) -> u64 {
    // RtlCaptureStackBackTrace(FramesToSkip, FramesToCapture, BackTrace, BackTraceHash)
    let c = &*ctx;
    let skip = c.gpr[RCX] as u32 as usize;
    let count = c.gpr[RDX] as u32 as usize;
    let out = c.gpr[R8] as *mut u64;
    let hash = c.gpr[R9] as *mut u32;
    // Frame 0 is the caller of RtlCaptureStackBackTrace (guest caller: its context, cursor
    // its region; lifted / host caller: the host capture, settled onto the record chain).
    let frames = backtrace_chain(c, cur_from_raw(cur), skip + count);
    let mut n = 0;
    let mut h = 0u32;
    for (i, pc) in frames.into_iter().skip(skip).take(count).enumerate() {
        *out.add(i) = pc;
        h = h.wrapping_add(pc as u32);
        n += 1;
    }
    if !hash.is_null() {
        *hash = h;
    }
    n as u64
}

/// Build an EXCEPTION_RECORD + CONTEXT for a hardware fault and dispatch it.
/// Used by the loader's signal handler after switching off the signal stack.
pub unsafe extern "win64" fn dispatch_hardware_exception(rec: *mut ExceptionRecord, ctx: *mut Context) {
    let mut c = *ctx;
    dispatch_exception(&mut *rec, &mut c);
    restore_context(&c)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn rtl_install_function_table_callback_lookup_delete_out_of_range_and_reentry() {
        static CALL_COUNT: AtomicUsize = AtomicUsize::new(0);
        static LAST_PC: AtomicUsize = AtomicUsize::new(0);
        static LAST_CTX: AtomicUsize = AtomicUsize::new(0);
        static NESTED_BASE_SEEN: AtomicUsize = AtomicUsize::new(0);
        static NESTED_RF_SEEN: AtomicUsize = AtomicUsize::new(0);
        static SELF_REENTRY_RF: AtomicUsize = AtomicUsize::new(usize::MAX);
        static DYNAMIC_NESTED_OK: AtomicUsize = AtomicUsize::new(0);

        static RF_PRIMARY: RuntimeFunction = RuntimeFunction {
            begin_rva: 0x100,
            end_rva: 0x200,
            unwind_rva: 0x1000,
        };
        static RF_SECONDARY: RuntimeFunction = RuntimeFunction {
            begin_rva: 0x040,
            end_rva: 0x080,
            unwind_rva: 0x2000,
        };
        static RF_TEMP: RuntimeFunction = RuntimeFunction {
            begin_rva: 0x010,
            end_rva: 0x050,
            unwind_rva: 0x3000,
        };

        const BASE_PRIMARY: u64 = 0x7000_0000;
        const LEN_PRIMARY: u32 = 0x4000;
        const TID_PRIMARY: u64 = 0x7000_0000 | 3;

        const BASE_SECONDARY: u64 = 0x7001_0000;
        const LEN_SECONDARY: u32 = 0x2000;
        const TID_SECONDARY: u64 = 0x7001_0000 | 3;

        const BASE_TEMP: u64 = 0x7002_0000;
        const LEN_TEMP: u32 = 0x1000;
        const TID_TEMP: u64 = 0x7002_0000 | 3;

        const BASE_GUEST_IMG: u64 = 0x7003_0000;
        const LEN_GUEST_IMG: u32 = 0x1000;
        const TID_GUEST_IMG: u64 = 0x7003_0000 | 3;

        unsafe extern "win64" fn temp_cb(pc: u64, ctx: *const c_void) -> *const RuntimeFunction {
            if pc == BASE_TEMP + 0x20 && ctx as usize == 0x55AA {
                &RF_TEMP
            } else {
                std::ptr::null()
            }
        }

        unsafe extern "win64" fn secondary_cb(pc: u64, ctx: *const c_void) -> *const RuntimeFunction {
            if pc >= BASE_SECONDARY + 0x40 && pc < BASE_SECONDARY + 0x80 && ctx as usize == 0x2222 {
                &RF_SECONDARY
            } else {
                std::ptr::null()
            }
        }

        unsafe extern "win64" fn primary_cb(pc: u64, ctx: *const c_void) -> *const RuntimeFunction {
            CALL_COUNT.fetch_add(1, Ordering::SeqCst);
            LAST_PC.store(pc as usize, Ordering::SeqCst);
            LAST_CTX.store(ctx as usize, Ordering::SeqCst);

            // Reentry test: while inside callback (locks MUST be dropped!), perform:
            // 1) lookup into another callback table,
            // 2) install + lookup + delete of a temporary callback table,
            // 3) self-reentry lookup on our own exact (table_id, pc) without deadlock or recursion blowup.
            if ctx as usize == 0xCAFE_BABE {
                let mut nested_base = 0u64;
                let nested_rf = RtlLookupFunctionEntry(
                    BASE_SECONDARY + 0x50,
                    &mut nested_base,
                    std::ptr::null_mut(),
                );
                NESTED_BASE_SEEN.store(nested_base as usize, Ordering::SeqCst);
                NESTED_RF_SEEN.store(nested_rf as usize, Ordering::SeqCst);

                let installed = RtlInstallFunctionTableCallback(
                    TID_TEMP,
                    BASE_TEMP,
                    LEN_TEMP,
                    temp_cb as *const c_void,
                    0x55AAusize as *const c_void,
                    std::ptr::null(),
                );
                let mut tb = 0u64;
                let trf = RtlLookupFunctionEntry(BASE_TEMP + 0x20, &mut tb, std::ptr::null_mut());
                let deleted = RtlDeleteFunctionTable(TID_TEMP as *const RuntimeFunction);
                if installed == 1
                    && tb == BASE_TEMP
                    && trf == (&RF_TEMP as *const RuntimeFunction)
                    && deleted == 1
                {
                    DYNAMIC_NESTED_OK.store(1, Ordering::SeqCst);
                }

                let self_rf = RtlLookupFunctionEntry(pc, std::ptr::null_mut(), std::ptr::null_mut());
                SELF_REENTRY_RF.store(self_rf as usize, Ordering::SeqCst);
            }

            if pc >= BASE_PRIMARY + 0x100 && pc < BASE_PRIMARY + 0x200 {
                &RF_PRIMARY
            } else {
                std::ptr::null()
            }
        }

        unsafe extern "win64" fn guest_img_cb(pc: u64, ctx: *const c_void) -> *const RuntimeFunction {
            if pc == BASE_GUEST_IMG + 0x150 && ctx as usize == 0x7777 {
                &RF_PRIMARY
            } else {
                std::ptr::null()
            }
        }

        unsafe {
            CALL_COUNT.store(0, Ordering::SeqCst);
            LAST_PC.store(0, Ordering::SeqCst);
            LAST_CTX.store(0, Ordering::SeqCst);
            NESTED_BASE_SEEN.store(0, Ordering::SeqCst);
            NESTED_RF_SEEN.store(0, Ordering::SeqCst);
            SELF_REENTRY_RF.store(usize::MAX, Ordering::SeqCst);
            DYNAMIC_NESTED_OK.store(0, Ordering::SeqCst);

            // 1. Validation: TableIdentifier must have low 2 bits == 3, len > 0, cb != null, no overflow.
            assert_eq!(
                RtlInstallFunctionTableCallback(
                    BASE_PRIMARY, // low 2 bits == 0 -> rejected
                    BASE_PRIMARY,
                    LEN_PRIMARY,
                    primary_cb as *const c_void,
                    std::ptr::null(),
                    std::ptr::null(),
                ),
                0
            );
            assert_eq!(
                RtlInstallFunctionTableCallback(
                    TID_PRIMARY,
                    BASE_PRIMARY,
                    0, // zero length -> rejected
                    primary_cb as *const c_void,
                    std::ptr::null(),
                    std::ptr::null(),
                ),
                0
            );
            assert_eq!(
                RtlInstallFunctionTableCallback(
                    TID_PRIMARY,
                    BASE_PRIMARY,
                    LEN_PRIMARY,
                    std::ptr::null(), // null callback -> rejected
                    std::ptr::null(),
                    std::ptr::null(),
                ),
                0
            );
            assert_eq!(
                RtlInstallFunctionTableCallback(
                    TID_PRIMARY,
                    u64::MAX - 4,
                    0x100, // base + len overflows -> rejected
                    primary_cb as *const c_void,
                    std::ptr::null(),
                    std::ptr::null(),
                ),
                0
            );

            // 2. Install primary & secondary callback ranges; duplicate TableIdentifier is rejected.
            assert_eq!(
                RtlInstallFunctionTableCallback(
                    TID_PRIMARY,
                    BASE_PRIMARY,
                    LEN_PRIMARY,
                    primary_cb as *const c_void,
                    0x1111usize as *const c_void,
                    std::ptr::null(),
                ),
                1
            );
            assert_eq!(
                RtlInstallFunctionTableCallback(
                    TID_PRIMARY,
                    BASE_PRIMARY,
                    LEN_PRIMARY,
                    primary_cb as *const c_void,
                    0x1111usize as *const c_void,
                    std::ptr::null(),
                ),
                0
            );
            assert_eq!(
                RtlInstallFunctionTableCallback(
                    TID_SECONDARY,
                    BASE_SECONDARY,
                    LEN_SECONDARY,
                    secondary_cb as *const c_void,
                    0x2222usize as *const c_void,
                    std::ptr::null(),
                ),
                1
            );

            // 3. Out-of-range lookups return NULL and must NOT invoke the callback.
            let mut out_base = 0xDEAD_BEEFu64;
            assert!(
                RtlLookupFunctionEntry(BASE_PRIMARY - 1, &mut out_base, std::ptr::null_mut())
                    .is_null()
            );
            assert!(
                RtlLookupFunctionEntry(
                    BASE_PRIMARY + LEN_PRIMARY as u64,
                    &mut out_base,
                    std::ptr::null_mut()
                )
                .is_null()
            );
            assert_eq!(CALL_COUNT.load(Ordering::SeqCst), 0);

            // 4. In-range lookup invokes callback with exact (ControlPc, Context) and returns (entry, base).
            let rf = RtlLookupFunctionEntry(
                BASE_PRIMARY + 0x120,
                &mut out_base,
                std::ptr::null_mut(),
            );
            assert_eq!(rf, &RF_PRIMARY as *const RuntimeFunction);
            assert_eq!(out_base, BASE_PRIMARY);
            assert_eq!(CALL_COUNT.load(Ordering::SeqCst), 1);
            assert_eq!(LAST_PC.load(Ordering::SeqCst), (BASE_PRIMARY + 0x120) as usize);
            assert_eq!(LAST_CTX.load(Ordering::SeqCst), 0x1111);

            // In-range PC where callback returns NULL yields NULL.
            let rf_none = RtlLookupFunctionEntry(
                BASE_PRIMARY + 0x010,
                &mut out_base,
                std::ptr::null_mut(),
            );
            assert!(rf_none.is_null());
            assert_eq!(CALL_COUNT.load(Ordering::SeqCst), 2);

            // 5. Delete primary and re-install with reentry context to verify lock-free reentry.
            assert_eq!(RtlDeleteFunctionTable(TID_PRIMARY as *const RuntimeFunction), 1);
            assert_eq!(RtlDeleteFunctionTable(TID_PRIMARY as *const RuntimeFunction), 0);
            assert!(
                RtlLookupFunctionEntry(BASE_PRIMARY + 0x120, &mut out_base, std::ptr::null_mut())
                    .is_null()
            );
            assert_eq!(CALL_COUNT.load(Ordering::SeqCst), 2);

            assert_eq!(
                RtlInstallFunctionTableCallback(
                    TID_PRIMARY,
                    BASE_PRIMARY,
                    LEN_PRIMARY,
                    primary_cb as *const c_void,
                    0xCAFE_BABEusize as *const c_void,
                    std::ptr::null(),
                ),
                1
            );
            let rf_re = RtlLookupFunctionEntry(
                BASE_PRIMARY + 0x140,
                &mut out_base,
                std::ptr::null_mut(),
            );
            assert_eq!(rf_re, &RF_PRIMARY as *const RuntimeFunction);
            assert_eq!(out_base, BASE_PRIMARY);
            assert_eq!(NESTED_BASE_SEEN.load(Ordering::SeqCst), BASE_SECONDARY as usize);
            assert_eq!(
                NESTED_RF_SEEN.load(Ordering::SeqCst),
                &RF_SECONDARY as *const RuntimeFunction as usize
            );
            assert_eq!(DYNAMIC_NESTED_OK.load(Ordering::SeqCst), 1);
            assert_eq!(SELF_REENTRY_RF.load(Ordering::SeqCst), 0);

            // 6. Guest image-backed callback tracking + EH-aware dispatch (`call_guest8_seh`).
            let cb_va = guest_img_cb as *const () as u64;
            register_test_image(cb_va, 0x100);
            assert_eq!(
                RtlInstallFunctionTableCallback(
                    TID_GUEST_IMG,
                    BASE_GUEST_IMG,
                    LEN_GUEST_IMG,
                    guest_img_cb as *const c_void,
                    0x7777usize as *const c_void,
                    std::ptr::null(),
                ),
                1
            );
            {
                let cbs = DYN_CALLBACKS.lock().unwrap_or_else(|e| e.into_inner());
                let entry = cbs.iter().find(|d| d.table_id == TID_GUEST_IMG).unwrap();
                assert!(entry.guest_image_backed);
            }
            let mut img_base = 0u64;
            let img_rf = RtlLookupFunctionEntry(
                BASE_GUEST_IMG + 0x150,
                &mut img_base,
                std::ptr::null_mut(),
            );
            assert_eq!(img_rf, &RF_PRIMARY as *const RuntimeFunction);
            assert_eq!(img_base, BASE_GUEST_IMG);

            // Clean up all installed test tables.
            assert_eq!(RtlDeleteFunctionTable(TID_PRIMARY as *const RuntimeFunction), 1);
            assert_eq!(RtlDeleteFunctionTable(TID_SECONDARY as *const RuntimeFunction), 1);
            assert_eq!(RtlDeleteFunctionTable(TID_GUEST_IMG as *const RuntimeFunction), 1);
        }
    }
}
