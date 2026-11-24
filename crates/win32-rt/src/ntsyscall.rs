//! Windows 10 22H2 (Build 19045) x64 System Service Descriptor Table (`SSDT`)
//! direct `syscall` (`0F 05`) dispatcher and authentic `ntdll.dll` stub generator.
//!
//! Why this exists:
//! User-mode anti-cheat and anti-tamper systems (Overwatch/Byfron, EAC, BattlEye,
//! Denuvo, Arxan) frequently bypass imported Win32 APIs and either:
//! 1. Execute raw `x86-64` `syscall` (`0F 05`) instructions directly with the
//!    Windows NT System Service Number (`SSN`) in `EAX`, 1st arg in `R10`
//!    (per `mov r10, rcx`), 2nd in `RDX`, 3rd in `R8`, 4th in `R9`, and 5th+
//!    arguments at `[RSP + 0x28 + 8*i]`.
//! 2. Scan `ntdll.dll` exports in memory to verify they start with the canonical
//!    unhooked byte sequence `4C 8B D1 B8 <ssn:u32> 0F 05 C3` (`mov r10, rcx;
//!    mov eax, <SSN>; syscall; ret`) and dynamically read the 32-bit SSN at `+4`.

use crate::ntdll;
use crate::types::{HANDLE, NTSTATUS};
use std::ffi::c_void;

/// Canonical Windows 10 22H2 (19045) x64 System Service Numbers (SSNs).
pub mod ssn {
    pub const NT_ACCESS_CHECK: u32 = 0x0000;
    pub const NT_WORKER_FACTORY_WORKER_READY: u32 = 0x0001;
    pub const NT_WAIT_FOR_SINGLE_OBJECT: u32 = 0x0004;
    pub const NT_CALLBACK_RETURN: u32 = 0x0005;
    pub const NT_READ_FILE: u32 = 0x0006;
    pub const NT_DEVICE_IO_CONTROL_FILE: u32 = 0x0007;
    pub const NT_WRITE_FILE: u32 = 0x0008;
    pub const NT_REMOVE_IO_COMPLETION: u32 = 0x0009;
    pub const NT_RELEASE_SEMAPHORE: u32 = 0x000A;
    pub const NT_REPLY_WAIT_RECEIVE_PORT: u32 = 0x000B;
    pub const NT_SET_INFORMATION_THREAD: u32 = 0x000D;
    pub const NT_SET_EVENT: u32 = 0x000E;
    pub const NT_CLOSE: u32 = 0x000F;
    pub const NT_DUPLICATE_OBJECT_ALT: u32 = 0x0010;
    pub const NT_QUERY_ATTRIBUTES_FILE: u32 = 0x0011;
    pub const NT_OPEN_KEY: u32 = 0x0012;
    pub const NT_ALLOCATE_VIRTUAL_MEMORY: u32 = 0x0018;
    pub const NT_QUERY_INFORMATION_PROCESS: u32 = 0x0019;
    pub const NT_WAIT_FOR_MULTIPLE_OBJECTS32: u32 = 0x001A;
    pub const NT_FREE_VIRTUAL_MEMORY: u32 = 0x001E;
    pub const NT_IMPERSONATE_ANONYMOUS_TOKEN: u32 = 0x001F;
    pub const NT_QUERY_VIRTUAL_MEMORY: u32 = 0x0023;
    pub const NT_CREATE_TIMER: u32 = 0x0024;
    pub const NT_QUERY_INFORMATION_THREAD: u32 = 0x0025;
    pub const NT_OPEN_PROCESS: u32 = 0x0026;
    pub const NT_MAP_VIEW_OF_SECTION: u32 = 0x0028;
    pub const NT_UNMAP_VIEW_OF_SECTION: u32 = 0x002A;
    pub const NT_TERMINATE_PROCESS: u32 = 0x002C;
    pub const NT_CLEAR_EVENT: u32 = 0x002E;
    pub const NT_QUERY_PERFORMANCE_COUNTER: u32 = 0x0031;
    pub const NT_OPEN_FILE: u32 = 0x0033;
    pub const NT_DELAY_EXECUTION: u32 = 0x0034;
    pub const NT_QUERY_DIRECTORY_FILE: u32 = 0x0035;
    pub const NT_QUERY_SYSTEM_INFORMATION: u32 = 0x0036;
    pub const NT_OPEN_SECTION: u32 = 0x0037;
    pub const NT_WRITE_VIRTUAL_MEMORY: u32 = 0x003A;
    pub const NT_DUPLICATE_OBJECT: u32 = 0x003C;
    pub const NT_QUERY_INFORMATION_FILE: u32 = 0x003D;
    pub const NT_READ_VIRTUAL_MEMORY: u32 = 0x003F;
    pub const NT_CREATE_EVENT: u32 = 0x0048;
    pub const NT_CREATE_SECTION: u32 = 0x004A;
    pub const NT_CONTINUE: u32 = 0x0043;
    pub const NT_YIELD_EXECUTION: u32 = 0x0046;
    pub const NT_PROTECT_VIRTUAL_MEMORY: u32 = 0x0050;
    pub const NT_QUERY_SECTION: u32 = 0x0051;
    pub const NT_CREATE_FILE: u32 = 0x0055;
    pub const NT_WAIT_FOR_MULTIPLE_OBJECTS: u32 = 0x005B;
    pub const NT_SET_INFORMATION_FILE: u32 = 0x0027;
    pub const NT_SET_INFORMATION_PROCESS: u32 = 0x001C;
    pub const NT_FLUSH_INSTRUCTION_CACHE: u32 = 0x00B5;
    pub const NT_CREATE_THREAD_EX: u32 = 0x00C2;
    pub const NT_GET_CONTEXT_THREAD: u32 = 0x00F2;
    pub const NT_GET_NEXT_THREAD: u32 = 0x00F9;
    pub const NT_OPEN_THREAD: u32 = 0x012E;
    pub const NT_RAISE_EXCEPTION: u32 = 0x016E;
    pub const NT_SET_CONTEXT_THREAD: u32 = 0x018B;
    pub const NT_SET_TIMER_RESOLUTION: u32 = 0x01B2;
    pub const NT_QUERY_TIMER_RESOLUTION: u32 = 0x0158;
}

/// Metadata entry for a Windows 10 22H2 x64 syscall.
#[derive(Debug, Clone, Copy)]
pub struct SyscallDesc {
    pub ssn: u32,
    pub name: &'static str,
    pub arg_count: u8,
}

pub static SYSCALL_TABLE: &[SyscallDesc] = &[
    SyscallDesc { ssn: ssn::NT_WAIT_FOR_SINGLE_OBJECT, name: "NtWaitForSingleObject", arg_count: 3 },
    SyscallDesc { ssn: ssn::NT_READ_FILE, name: "NtReadFile", arg_count: 9 },
    SyscallDesc { ssn: ssn::NT_WRITE_FILE, name: "NtWriteFile", arg_count: 9 },
    SyscallDesc { ssn: ssn::NT_REMOVE_IO_COMPLETION, name: "NtRemoveIoCompletion", arg_count: 5 },
    SyscallDesc { ssn: ssn::NT_RELEASE_SEMAPHORE, name: "NtReleaseSemaphore", arg_count: 3 },
    SyscallDesc { ssn: ssn::NT_SET_INFORMATION_THREAD, name: "NtSetInformationThread", arg_count: 4 },
    SyscallDesc { ssn: ssn::NT_SET_EVENT, name: "NtSetEvent", arg_count: 2 },
    SyscallDesc { ssn: ssn::NT_CLOSE, name: "NtClose", arg_count: 1 },
    SyscallDesc { ssn: ssn::NT_QUERY_ATTRIBUTES_FILE, name: "NtQueryAttributesFile", arg_count: 2 },
    SyscallDesc { ssn: ssn::NT_ALLOCATE_VIRTUAL_MEMORY, name: "NtAllocateVirtualMemory", arg_count: 6 },
    SyscallDesc { ssn: ssn::NT_QUERY_INFORMATION_PROCESS, name: "NtQueryInformationProcess", arg_count: 5 },
    SyscallDesc { ssn: ssn::NT_SET_INFORMATION_PROCESS, name: "NtSetInformationProcess", arg_count: 4 },
    SyscallDesc { ssn: ssn::NT_FREE_VIRTUAL_MEMORY, name: "NtFreeVirtualMemory", arg_count: 4 },
    SyscallDesc { ssn: ssn::NT_QUERY_VIRTUAL_MEMORY, name: "NtQueryVirtualMemory", arg_count: 6 },
    SyscallDesc { ssn: ssn::NT_QUERY_INFORMATION_THREAD, name: "NtQueryInformationThread", arg_count: 5 },
    SyscallDesc { ssn: ssn::NT_OPEN_PROCESS, name: "NtOpenProcess", arg_count: 4 },
    SyscallDesc { ssn: ssn::NT_MAP_VIEW_OF_SECTION, name: "NtMapViewOfSection", arg_count: 10 },
    SyscallDesc { ssn: ssn::NT_UNMAP_VIEW_OF_SECTION, name: "NtUnmapViewOfSection", arg_count: 2 },
    SyscallDesc { ssn: ssn::NT_TERMINATE_PROCESS, name: "NtTerminateProcess", arg_count: 2 },
    SyscallDesc { ssn: ssn::NT_CLEAR_EVENT, name: "NtClearEvent", arg_count: 1 },
    SyscallDesc { ssn: ssn::NT_QUERY_PERFORMANCE_COUNTER, name: "NtQueryPerformanceCounter", arg_count: 2 },
    SyscallDesc { ssn: ssn::NT_OPEN_FILE, name: "NtOpenFile", arg_count: 6 },
    SyscallDesc { ssn: ssn::NT_DELAY_EXECUTION, name: "NtDelayExecution", arg_count: 2 },
    SyscallDesc { ssn: ssn::NT_QUERY_SYSTEM_INFORMATION, name: "NtQuerySystemInformation", arg_count: 4 },
    SyscallDesc { ssn: ssn::NT_WRITE_VIRTUAL_MEMORY, name: "NtWriteVirtualMemory", arg_count: 5 },
    SyscallDesc { ssn: ssn::NT_DUPLICATE_OBJECT, name: "NtDuplicateObject", arg_count: 7 },
    SyscallDesc { ssn: ssn::NT_READ_VIRTUAL_MEMORY, name: "NtReadVirtualMemory", arg_count: 5 },
    SyscallDesc { ssn: ssn::NT_CONTINUE, name: "NtContinue", arg_count: 2 },
    SyscallDesc { ssn: ssn::NT_YIELD_EXECUTION, name: "NtYieldExecution", arg_count: 0 },
    SyscallDesc { ssn: ssn::NT_CREATE_EVENT, name: "NtCreateEvent", arg_count: 5 },
    SyscallDesc { ssn: ssn::NT_CREATE_SECTION, name: "NtCreateSection", arg_count: 7 },
    SyscallDesc { ssn: ssn::NT_PROTECT_VIRTUAL_MEMORY, name: "NtProtectVirtualMemory", arg_count: 5 },
    SyscallDesc { ssn: ssn::NT_CREATE_FILE, name: "NtCreateFile", arg_count: 11 },
    SyscallDesc { ssn: ssn::NT_WAIT_FOR_MULTIPLE_OBJECTS, name: "NtWaitForMultipleObjects", arg_count: 5 },
    SyscallDesc { ssn: ssn::NT_FLUSH_INSTRUCTION_CACHE, name: "NtFlushInstructionCache", arg_count: 3 },
    SyscallDesc { ssn: ssn::NT_CREATE_THREAD_EX, name: "NtCreateThreadEx", arg_count: 11 },
    SyscallDesc { ssn: ssn::NT_GET_CONTEXT_THREAD, name: "NtGetContextThread", arg_count: 2 },
    SyscallDesc { ssn: ssn::NT_GET_NEXT_THREAD, name: "NtGetNextThread", arg_count: 6 },
    SyscallDesc { ssn: ssn::NT_OPEN_THREAD, name: "NtOpenThread", arg_count: 4 },
    SyscallDesc { ssn: ssn::NT_RAISE_EXCEPTION, name: "NtRaiseException", arg_count: 3 },
    SyscallDesc { ssn: ssn::NT_SET_CONTEXT_THREAD, name: "NtSetContextThread", arg_count: 2 },
    SyscallDesc { ssn: ssn::NT_SET_TIMER_RESOLUTION, name: "NtSetTimerResolution", arg_count: 3 },
    SyscallDesc { ssn: ssn::NT_QUERY_TIMER_RESOLUTION, name: "NtQueryTimerResolution", arg_count: 3 },
];

/// Look up a syscall descriptor by its Windows 10 22H2 SSN.
pub fn lookup_by_ssn(ssn: u32) -> Option<&'static SyscallDesc> {
    SYSCALL_TABLE.iter().find(|d| d.ssn == ssn)
}

/// Look up the Windows 10 22H2 SSN for an `Nt*` or `Zw*` export name.
pub fn lookup_ssn_by_name(name: &str) -> Option<u32> {
    let norm = if let Some(rest) = name.strip_prefix("Zw") {
        format!("Nt{rest}")
    } else {
        name.to_string()
    };
    SYSCALL_TABLE.iter().find(|d| d.name == norm).map(|d| d.ssn)
}

/// Encode the canonical 16-byte Windows x64 `ntdll.dll` syscall stub:
/// ```text
/// 4C 8B D1          mov r10, rcx
/// B8 <ssn:u32>      mov eax, <ssn>
/// 0F 05             syscall
/// C3                ret
/// 0F 1F 44 00 00    nop dword ptr [rax + rax*1 + 0x00]
/// ```
/// Anti-cheats inspect `ntdll.dll` export addresses for this exact byte pattern
/// both to verify `ntdll.dll` is unhooked and to extract the 32-bit SSN at `+4`.
pub const fn encode_ntdll_syscall_stub(ssn: u32) -> [u8; 16] {
    let b = ssn.to_le_bytes();
    [
        0x4C, 0x8B, 0xD1, // mov r10, rcx
        0xB8, b[0], b[1], b[2], b[3], // mov eax, imm32
        0x0F, 0x05, // syscall
        0xC3, // ret
        0x0F, 0x1F, 0x44, 0x00, 0x00, // 5-byte nop padding
    ]
}

#[inline(always)]
unsafe fn stack_arg(rsp: u64, index_from_5th: usize) -> u64 {
    if rsp < 0x1000 {
        return 0;
    }
    let addr = rsp.wrapping_add(0x28 + (index_from_5th * 8) as u64);
    std::ptr::read_unaligned(addr as *const u64)
}

/// Dispatch a direct `x86-64` `syscall` (`0F 05`) instruction using the
/// Windows x64 syscall ABI:
/// - `ssn`: `EAX` (System Service Number)
/// - `r10`: 1st argument (`R10`, copied from `RCX` by `mov r10, rcx`)
/// - `rdx`: 2nd argument
/// - `r8`:  3rd argument
/// - `r9`:  4th argument
/// - `rsp`: Guest stack pointer (`[rsp + 0x28]` = 5th arg, `[rsp + 0x30]` = 6th arg, ...)
pub unsafe extern "win64" fn dispatch_nt_syscall(
    ssn: u32,
    r10: u64,
    rdx: u64,
    r8: u64,
    r9: u64,
    rsp: u64,
) -> NTSTATUS {
    let a5 = stack_arg(rsp, 0);
    let a6 = stack_arg(rsp, 1);

    let sym_name = lookup_by_ssn(ssn).map_or("unknown", |d| d.name);
    log::debug!(
        "ntsyscall: {sym_name} (SSN=0x{ssn:04x}, r10=0x{r10:x}, rdx=0x{rdx:x}, r8=0x{r8:x}, r9=0x{r9:x}, a5=0x{a5:x}, a6=0x{a6:x})"
    );

    match ssn {
        ssn::NT_CLOSE => ntdll::NtClose(r10 as HANDLE),
        ssn::NT_OPEN_FILE => ntdll::NtOpenFile(
            r10 as *mut HANDLE,
            rdx as u32,
            r8 as *const c_void,
            r9 as *mut u8,
            a5 as u32,
            a6 as u32,
        ),
        ssn::NT_CREATE_FILE => ntdll::NtCreateFile(
            r10 as *mut HANDLE,
            rdx as u32,
            r8 as *const c_void,
            r9 as *mut u8,
            a5 as *const i64,
            a6 as u32,
            stack_arg(rsp, 2) as u32,
            stack_arg(rsp, 3) as u32,
            stack_arg(rsp, 4) as u32,
            stack_arg(rsp, 5) as *mut c_void,
            stack_arg(rsp, 6) as u32,
        ),
        ssn::NT_YIELD_EXECUTION => ntdll::NtYieldExecution(),
        ssn::NT_DELAY_EXECUTION => ntdll::NtDelayExecution(r10 as u8, rdx as *const i64),
        ssn::NT_QUERY_PERFORMANCE_COUNTER => {
            ntdll::NtQueryPerformanceCounter(r10 as *mut i64, rdx as *mut i64)
        }
        ssn::NT_SET_TIMER_RESOLUTION => {
            ntdll::NtSetTimerResolution(r10 as u32, rdx as u8, r8 as *mut u32)
        }
        ssn::NT_QUERY_TIMER_RESOLUTION => {
            ntdll::NtQueryTimerResolution(r10 as *mut u32, rdx as *mut u32, r8 as *mut u32)
        }
        ssn::NT_QUERY_INFORMATION_PROCESS => ntdll::NtQueryInformationProcess(
            r10 as HANDLE,
            rdx as u32,
            r8 as *mut u8,
            r9 as u32,
            a5 as *mut u32,
        ),
        ssn::NT_SET_INFORMATION_PROCESS => ntdll::NtSetInformationProcess(
            r10 as HANDLE,
            rdx as u32,
            r8 as *const c_void,
            r9 as u32,
        ),
        ssn::NT_SET_INFORMATION_THREAD => ntdll::NtSetInformationThread(
            r10 as HANDLE,
            rdx as u32,
            r8 as *const c_void,
            r9 as u32,
        ),
        ssn::NT_QUERY_INFORMATION_THREAD => ntdll::NtQueryInformationThread(
            r10 as HANDLE,
            rdx as u32,
            r8 as *mut u8,
            r9 as u32,
            a5 as *mut u32,
        ),
        ssn::NT_QUERY_SYSTEM_INFORMATION => ntdll::NtQuerySystemInformation(
            r10 as u32,
            rdx as *mut u8,
            r8 as u32,
            r9 as *mut u32,
        ),
        ssn::NT_ALLOCATE_VIRTUAL_MEMORY => ntdll::NtAllocateVirtualMemory(
            r10 as HANDLE,
            rdx as *mut *mut c_void,
            r8 as usize,
            r9 as *mut usize,
            a5 as u32,
            a6 as u32,
        ),
        ssn::NT_FREE_VIRTUAL_MEMORY => ntdll::NtFreeVirtualMemory(
            r10 as HANDLE,
            rdx as *mut *mut c_void,
            r8 as *mut usize,
            r9 as u32,
        ),
        ssn::NT_PROTECT_VIRTUAL_MEMORY => ntdll::NtProtectVirtualMemory(
            r10 as HANDLE,
            rdx as *mut *mut c_void,
            r8 as *mut usize,
            r9 as u32,
            a5 as *mut u32,
        ),
        ssn::NT_QUERY_VIRTUAL_MEMORY => ntdll::NtQueryVirtualMemory(
            r10 as HANDLE,
            rdx as *const c_void,
            r8 as u32,
            r9 as *mut c_void,
            a5 as usize,
            a6 as *mut usize,
        ),
        ssn::NT_READ_VIRTUAL_MEMORY => ntdll::NtReadVirtualMemory(
            r10 as HANDLE,
            rdx as *const c_void,
            r8 as *mut c_void,
            r9 as usize,
            a5 as *mut usize,
        ),
        ssn::NT_WRITE_VIRTUAL_MEMORY => ntdll::NtWriteVirtualMemory(
            r10 as HANDLE,
            rdx as *mut c_void,
            r8 as *const c_void,
            r9 as usize,
            a5 as *mut usize,
        ),
        ssn::NT_FLUSH_INSTRUCTION_CACHE => ntdll::NtFlushInstructionCache(
            r10 as HANDLE,
            rdx as *const c_void,
            r8 as usize,
        ),
        ssn::NT_CREATE_SECTION => ntdll::NtCreateSection(
            r10 as *mut HANDLE,
            rdx as u32,
            r8 as *const c_void,
            r9 as *const i64,
            a5 as u32,
            a6 as u32,
            stack_arg(rsp, 2) as HANDLE,
        ),
        ssn::NT_MAP_VIEW_OF_SECTION => ntdll::NtMapViewOfSection(
            r10 as HANDLE,
            rdx as HANDLE,
            r8 as *mut *mut c_void,
            r9 as usize,
            a5 as usize,
            a6 as *mut i64,
            stack_arg(rsp, 2) as *mut usize,
            stack_arg(rsp, 3) as u32,
            stack_arg(rsp, 4) as u32,
            stack_arg(rsp, 5) as u32,
        ),
        ssn::NT_UNMAP_VIEW_OF_SECTION => {
            ntdll::NtUnmapViewOfSection(r10 as HANDLE, rdx as *mut c_void)
        }
        ssn::NT_CREATE_THREAD_EX => ntdll::NtCreateThreadEx(
            r10 as *mut HANDLE,
            rdx as u32,
            r8 as *const c_void,
            r9 as HANDLE,
            a5 as *const c_void,
            a6 as *mut c_void,
            stack_arg(rsp, 2) as u32,
            stack_arg(rsp, 3) as usize,
            stack_arg(rsp, 4) as usize,
            stack_arg(rsp, 5) as usize,
            stack_arg(rsp, 6) as *mut c_void,
        ),
        ssn::NT_GET_NEXT_THREAD => ntdll::NtGetNextThread(
            r10 as HANDLE,
            rdx as HANDLE,
            r8 as u32,
            r9 as u32,
            a5 as u32,
            a6 as *mut HANDLE,
        ),
        ssn::NT_GET_CONTEXT_THREAD => {
            ntdll::NtGetContextThread(r10 as HANDLE, rdx as *mut crate::unwind::Context)
        }
        ssn::NT_SET_CONTEXT_THREAD => {
            ntdll::NtSetContextThread(r10 as HANDLE, rdx as *const crate::unwind::Context)
        }
        ssn::NT_OPEN_PROCESS => ntdll::NtOpenProcess(
            r10 as *mut HANDLE,
            rdx as u32,
            r8 as *const c_void,
            r9 as *const c_void,
        ),
        ssn::NT_OPEN_THREAD => ntdll::NtOpenThread(
            r10 as *mut HANDLE,
            rdx as u32,
            r8 as *const c_void,
            r9 as *const c_void,
        ),
        ssn::NT_WAIT_FOR_SINGLE_OBJECT => {
            ntdll::NtWaitForSingleObject(r10 as HANDLE, rdx as u8, r8 as *const i64)
        }
        ssn::NT_WAIT_FOR_MULTIPLE_OBJECTS => ntdll::NtWaitForMultipleObjects(
            r10 as u32,
            rdx as *const HANDLE,
            r8 as u32,
            r9 as u8,
            a5 as *const i64,
        ),
        ssn::NT_RELEASE_SEMAPHORE => {
            ntdll::NtReleaseSemaphore(r10 as HANDLE, rdx as i32, r8 as *mut i32)
        }
        ssn::NT_REMOVE_IO_COMPLETION => ntdll::NtRemoveIoCompletion(
            r10 as HANDLE,
            rdx as *mut usize,
            r8 as *mut *mut c_void,
            r9 as *mut u8,
            a5 as *const i64,
        ),
        ssn::NT_READ_FILE => ntdll::NtReadFile(
            r10 as HANDLE,
            rdx as HANDLE,
            r8 as *const c_void,
            r9 as *mut c_void,
            a5 as *mut u8,
            a6 as *mut c_void,
            stack_arg(rsp, 2) as u32,
            stack_arg(rsp, 3) as *const i64,
            stack_arg(rsp, 4) as *const u32,
        ),
        ssn::NT_WRITE_FILE => ntdll::NtWriteFile(
            r10 as HANDLE,
            rdx as HANDLE,
            r8 as *const c_void,
            r9 as *mut c_void,
            a5 as *mut u8,
            a6 as *const c_void,
            stack_arg(rsp, 2) as u32,
            stack_arg(rsp, 3) as *const i64,
            stack_arg(rsp, 4) as *const u32,
        ),
        ssn::NT_QUERY_ATTRIBUTES_FILE => {
            ntdll::NtQueryAttributesFile(r10 as *const c_void, rdx as *mut u8)
        }
        ssn::NT_DUPLICATE_OBJECT => ntdll::NtDuplicateObject(
            r10 as HANDLE,
            rdx as HANDLE,
            r8 as HANDLE,
            r9 as *mut HANDLE,
            a5 as u32,
            a6 as u32,
            stack_arg(rsp, 2) as u32,
        ),
        ssn::NT_CONTINUE => {
            ntdll::NtContinue(r10 as *const crate::unwind::Context, rdx as u8)
        }
        ssn::NT_RAISE_EXCEPTION => ntdll::NtRaiseException(
            r10 as *mut crate::unwind::ExceptionRecord,
            rdx as *mut crate::unwind::Context,
            r8 as u8,
        ),
        ssn::NT_CREATE_EVENT => ntdll::NtCreateEvent(
            r10 as *mut HANDLE,
            rdx as u32,
            r8 as *const c_void,
            r9 as u32,
            a5 as u8,
        ),
        ssn::NT_SET_EVENT => ntdll::NtSetEvent(r10 as HANDLE, rdx as *mut i32),
        ssn::NT_CLEAR_EVENT => ntdll::NtClearEvent(r10 as HANDLE),
        ssn::NT_TERMINATE_PROCESS => {
            if r10 == 0 || r10 == usize::MAX as u64 {
                crate::process::exit_process(rdx as u32);
            }
            ntdll::STATUS_SUCCESS
        }
        _ => {
            warn_unhandled_ssn_once(ssn, sym_name, r10, rdx, r8, r9);
            ntdll::STATUS_NOT_IMPLEMENTED
        }
    }
}

fn warn_unhandled_ssn_once(ssn: u32, sym_name: &str, r10: u64, rdx: u64, r8: u64, r9: u64) {
    static SEEN: std::sync::Mutex<Option<std::collections::HashSet<u32>>> =
        std::sync::Mutex::new(None);
    let first = {
        let mut g = SEEN.lock().unwrap_or_else(|e| e.into_inner());
        g.get_or_insert_with(std::collections::HashSet::new).insert(ssn)
    };
    if first {
        log::warn!(
            "ntsyscall: unhandled direct syscall {sym_name} (SSN=0x{ssn:04x}, r10=0x{r10:x}, rdx=0x{rdx:x}, r8=0x{r8:x}, r9=0x{r9:x}) -> STATUS_NOT_IMPLEMENTED"
        );
    } else {
        log::debug!(
            "ntsyscall: unhandled direct syscall {sym_name} (SSN=0x{ssn:04x}, r10=0x{r10:x}, rdx=0x{rdx:x}) -> STATUS_NOT_IMPLEMENTED"
        );
    }
}
