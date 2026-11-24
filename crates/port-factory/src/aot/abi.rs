//! Win64 calling-convention arity and static callback registration classification for
//! the standalone PE-to-Rust AOT emitter (`crates/port-factory/src/aot`).

/// Win64 calling-convention bridge classification for an IAT import in standalone AOT code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IatCallAbi {
    /// Pure integer/pointer Win64 API taking `arity` (0..=12) arguments (4 in GPRs,
    /// args `5..=arity` on the caller's stack at `[rsp + stack_base + 8*(i - 4)]`),
    /// returning an integer/pointer in `rax` (or void).
    IntArgs(u8),
    /// Mixed int/float, XMM/SIMD, variadic (`...`), hidden `sret`, >12 args, or unknown
    /// signature: emitted with a best-effort fallback bridge and marked `fully_lowered = false`
    /// so `port-loader` defers the function to the full ARM64 JIT.
    Fallback,
}

impl IatCallAbi {
    #[inline]
    pub const fn int_arity(self) -> Option<u8> {
        match self {
            Self::IntArgs(n) => Some(n),
            Self::Fallback => None,
        }
    }
}

/// Classify a canonical `(dll, symbol)` IAT import for the standalone emitter's ABI bridge.
///
/// Only well-known pure-integer/pointer Win64 APIs with exact known arity (`0..=12`) return
/// `IatCallAbi::IntArgs(n)`. Variadic (`printf`, `sprintf`, `snprintf`, ...), floating-point
/// (`sin`, `cos`, `pow`, ...), and unknown/unverified signatures return `IatCallAbi::Fallback`
/// so the emitter never silently drops stack/XMM arguments while claiming `fully_lowered = true`.
pub fn classify_iat_abi(canonical_dll: &str, symbol: &str) -> IatCallAbi {
    let norm_sym = if (canonical_dll == "comctl32.dll" || canonical_dll == "uxtheme.dll")
        && (symbol == "#410" || symbol == "ordinal_410")
    {
        "SetWindowSubclass"
    } else if (canonical_dll == "comctl32.dll" || canonical_dll == "uxtheme.dll")
        && (symbol == "#411" || symbol == "ordinal_411")
    {
        "GetWindowSubclass"
    } else if (canonical_dll == "comctl32.dll" || canonical_dll == "uxtheme.dll")
        && (symbol == "#412" || symbol == "ordinal_412")
    {
        "RemoveWindowSubclass"
    } else if (canonical_dll == "comctl32.dll" || canonical_dll == "uxtheme.dll")
        && (symbol == "#413" || symbol == "ordinal_413")
    {
        "DefSubclassProc"
    } else {
        symbol
    };

    let arity: Option<u8> = match norm_sym {
        // 0 args
        "GetLastError"
        | "GetCurrentProcess"
        | "GetCurrentProcessId"
        | "GetCurrentThread"
        | "GetCurrentThreadId"
        | "GetTickCount"
        | "GetTickCount64"
        | "IsDebuggerPresent"
        | "SwitchToThread"
        | "TlsAlloc"
        | "CoUninitialize"
        | "OleUninitialize"
        | "RoUninitialize"
        | "InitCommonControls"
        | "abort"
        | "_cexit"
        | "_errno" => Some(0),

        // 1 arg
        "SetLastError"
        | "CloseHandle"
        | "FreeLibrary"
        | "LoadLibraryA"
        | "LoadLibraryW"
        | "GetModuleHandleA"
        | "GetModuleHandleW"
        | "SetDefaultDllDirectories"
        | "AddDllDirectory"
        | "RemoveDllDirectory"
        | "SetDllDirectoryA"
        | "SetDllDirectoryW"
        | "DisableThreadLibraryCalls"
        | "Sleep"
        | "ExitProcess"
        | "ExitThread"
        | "TlsFree"
        | "TlsGetValue"
        | "FlsAlloc"
        | "FlsFree"
        | "FlsGetValue"
        | "SetEvent"
        | "ResetEvent"
        | "InitializeCriticalSection"
        | "DeleteCriticalSection"
        | "EnterCriticalSection"
        | "LeaveCriticalSection"
        | "TryEnterCriticalSection"
        | "InitializeSRWLock"
        | "AcquireSRWLockExclusive"
        | "ReleaseSRWLockExclusive"
        | "AcquireSRWLockShared"
        | "ReleaseSRWLockShared"
        | "QueryPerformanceCounter"
        | "QueryPerformanceFrequency"
        | "GetSystemTimeAsFileTime"
        | "IsProcessorFeaturePresent"
        | "OutputDebugStringA"
        | "OutputDebugStringW"
        | "EncodePointer"
        | "DecodePointer"
        | "RtlCaptureContext"
        | "UnhandledExceptionFilter"
        | "SetUnhandledExceptionFilter"
        | "CoInitialize"
        | "OleInitialize"
        | "RoInitialize"
        | "CoRevokeClassObject"
        | "CoTaskMemAlloc"
        | "CoTaskMemFree"
        | "RegisterClassA"
        | "RegisterClassW"
        | "RegisterClassExA"
        | "RegisterClassExW"
        | "DispatchMessageA"
        | "DispatchMessageW"
        | "TranslateMessage"
        | "DestroyWindow"
        | "IsWindow"
        | "InitCommonControlsEx"
        | "malloc"
        | "free"
        | "strlen"
        | "wcslen"
        | "atexit"
        | "_onexit"
        | "_crt_atexit"
        | "_crt_at_quick_exit"
        | "exit"
        | "_amsg_exit"
        | "_lock"
        | "_unlock"
        | "_close"
        | "fclose"
        | "fflush"
        | "ferror"
        | "feof"
        | "clearerr"
        | "getc"
        | "puts"
        | "strerror" => Some(1),

        // 2 args
        "GetProcAddress"
        | "TlsSetValue"
        | "FlsSetValue"
        | "WaitForSingleObject"
        | "SleepEx"
        | "TerminateProcess"
        | "FreeLibraryAndExitThread"
        | "FreeLibraryWhenCallbackReturns"
        | "CoInitializeEx"
        | "AddVectoredExceptionHandler"
        | "AddVectoredContinueHandler"
        | "RemoveVectoredExceptionHandler"
        | "RemoveVectoredContinueHandler"
        | "EndDialog"
        | "GetDlgItem"
        | "GetDlgCtrlID"
        | "IsDialogMessageA"
        | "IsDialogMessageW"
        | "EnumWindows"
        | "KillTimer"
        | "GetWindowLongPtrA"
        | "GetWindowLongPtrW"
        | "GetClassLongPtrA"
        | "GetClassLongPtrW"
        | "ShowWindow"
        | "EnableWindow"
        | "calloc"
        | "realloc"
        | "strcmp"
        | "strcpy"
        | "strcat"
        | "strchr"
        | "strrchr"
        | "strstr"
        | "fopen"
        | "fdopen"
        | "ftell"
        | "fputc"
        | "putc"
        | "ungetc"
        | "_initterm"
        | "_initterm_e"
        | "_register_onexit_function"
        | "_fstati64"
        | "_fstat64"
        | "_stat64" => Some(2),

        // 3 args
        "LoadLibraryExA"
        | "LoadLibraryExW"
        | "GetModuleHandleExA"
        | "GetModuleHandleExW"
        | "GetModuleFileNameA"
        | "GetModuleFileNameW"
        | "VirtualFree"
        | "VirtualQuery"
        | "HeapAlloc"
        | "HeapFree"
        | "QueueUserAPC"
        | "QueueUserWorkItem"
        | "RtlQueueWorkItem"
        | "CreateThreadpoolWork"
        | "TrySubmitThreadpoolCallback"
        | "CreateThreadpoolTimer"
        | "CreateThreadpoolWait"
        | "WaitForSingleObjectEx"
        | "RtlLookupFunctionEntry"
        | "RemoveWindowSubclass"
        | "SetWindowLongPtrA"
        | "SetWindowLongPtrW"
        | "SetClassLongPtrA"
        | "SetClassLongPtrW"
        | "SetDlgItemTextA"
        | "SetDlgItemTextW"
        | "EnumChildWindows"
        | "EnumThreadWindows"
        | "RoGetActivationFactory"
        | "memcpy"
        | "memmove"
        | "memset"
        | "memcmp"
        | "memchr"
        | "strncmp"
        | "strncpy"
        | "_beginthread"
        | "_open"
        | "_wopen"
        | "_read"
        | "_write"
        | "_lseek"
        | "_lseeki64"
        | "fseek"
        | "fgets"
        | "wcstombs"
        | "__dllonexit" => Some(3),

        // 4 args
        "VirtualAlloc"
        | "VirtualProtect"
        | "HeapReAlloc"
        | "CreateEventA"
        | "CreateEventW"
        | "CreateEventExA"
        | "CreateEventExW"
        | "WaitForMultipleObjects"
        | "RaiseException"
        | "LdrLoadDll"
        | "LdrGetProcedureAddress"
        | "CreateThreadpoolIo"
        | "QueueUserAPC2"
        | "SetWindowSubclass"
        | "GetWindowSubclass"
        | "DefSubclassProc"
        | "DefWindowProcA"
        | "DefWindowProcW"
        | "DefDlgProcA"
        | "DefDlgProcW"
        | "SendMessageA"
        | "SendMessageW"
        | "PostMessageA"
        | "PostMessageW"
        | "GetMessageA"
        | "GetMessageW"
        | "SetTimer"
        | "GetDlgItemTextA"
        | "GetDlgItemTextW"
        | "SetDlgItemInt"
        | "GetDlgItemInt"
        | "EnumDisplayMonitors"
        | "SetWindowsHookExA"
        | "SetWindowsHookExW"
        | "fread"
        | "fwrite"
        | "vsnprintf"
        | "_vsnprintf" => Some(4),

        // 5 args (5th arg on stack at [rsp + 0x20]!)
        "ReadFileEx"
        | "WriteFileEx"
        | "ReadFile"
        | "WriteFile"
        | "VirtualProtectEx"
        | "MapViewOfFile"
        | "WaitForMultipleObjectsEx"
        | "CoCreateInstance"
        | "CoGetClassObject"
        | "CoRegisterClassObject"
        | "DialogBoxParamA"
        | "DialogBoxParamW"
        | "DialogBoxIndirectParamA"
        | "DialogBoxIndirectParamW"
        | "CreateDialogParamA"
        | "CreateDialogParamW"
        | "CreateDialogIndirectParamA"
        | "CreateDialogIndirectParamW"
        | "PeekMessageA"
        | "PeekMessageW"
        | "SendDlgItemMessageA"
        | "SendDlgItemMessageW"
        | "CallWindowProcA"
        | "CallWindowProcW"
        | "LdrGetProcedureAddressEx"
        | "timeSetEvent" => Some(5),

        // 6 args
        "CreateThread"
        | "_beginthreadex"
        | "RegisterWaitForSingleObject"
        | "RtlRegisterWait"
        | "RtlInstallFunctionTableCallback"
        | "SetWaitableTimer"
        | "CreateFileMappingA"
        | "CreateFileMappingW"
        | "GetQueuedCompletionStatusEx"
        | "MultiByteToWideChar" => Some(6),

        // 7 args
        "CreateFileA"
        | "CreateFileW"
        | "CreateTimerQueueTimer"
        | "SetWaitableTimerEx" => Some(7),

        // 8 args
        "WideCharToMultiByte" | "DeviceIoControl" | "RtlVirtualUnwind" => Some(8),

        // 10 args
        "CreateProcessA" | "CreateProcessW" => Some(10),

        // 12 args
        "CreateWindowExA" | "CreateWindowExW" => Some(12),

        // Everything else (including variadic printf/sprintf/snprintf, float math,
        // unknown middleware imports) uses honest Fallback.
        _ => None,
    };

    match arity {
        Some(n) => IatCallAbi::IntArgs(n),
        None => IatCallAbi::Fallback,
    }
}

use super::{AotCallbackRoot, AotImportBinding};
use iced_x86::{Decoder, DecoderOptions, FlowControl, Instruction, Mnemonic, OpKind, Register};
use pe_image::Pe;
use std::collections::{BTreeMap, BTreeSet, HashMap};

/// Discover executable module-relative callback roots, CRT ctor/dtor arrays, `.pdata` unwind
/// handlers, Load Config CFG/EH continuation entries, DIR64 vtables/COM sinks, and static
/// registration-site callbacks (including 5th-arg stack slots like `ReadFileEx`) from a
/// parsed `Pe` image without ever executing guest code.
pub(super) fn discover_pe_callback_roots(
    pe: &Pe<'_>,
    exec_ranges: &[(u32, u32)],
    imports_by_rva: &BTreeMap<u32, AotImportBinding>,
    iat_thunks: &HashMap<u32, u32>,
) -> Vec<AotCallbackRoot> {
    let image_base = pe.headers.image_base;
    let size_of_image = pe.headers.size_of_image;
    let is_exec_rva = |rva: u32| -> bool {
        exec_ranges.iter().any(|&(lo, hi)| rva >= lo && rva < hi)
    };
    let is_plausible_code = |rva: u32| -> bool {
        if !is_exec_rva(rva) {
            return false;
        }
        let Ok(b) = pe.bytes_at(rva, 1) else { return false };
        if b[0] == 0xCC {
            return false;
        }
        let avail = exec_ranges
            .iter()
            .find(|&&(lo, hi)| rva >= lo && rva < hi)
            .map(|&(_, hi)| (hi - rva).min(15) as usize)
            .unwrap_or(1);
        let Ok(slice) = pe.bytes_at(rva, avail) else { return false };
        if slice.len() >= 2 && slice[0] == 0x00 && slice[1] == 0x00 {
            return false;
        }
        let mut d = Decoder::with_ip(64, slice, image_base + rva as u64, DecoderOptions::NONE);
        !d.decode().is_invalid()
    };
    let va_to_rva = |va: u64| -> Option<u32> {
        let rel = va.checked_sub(image_base)?;
        if rel < size_of_image as u64 {
            u32::try_from(rel).ok()
        } else {
            None
        }
    };
    let data_ranges: Vec<(u32, u32)> = pe
        .sections
        .iter()
        .filter(|s| {
            !s.is_exec()
                && (s.characteristics & pe_image::consts::IMAGE_SCN_CNT_CODE == 0)
                && s.raw_size > 0
        })
        .map(|s| (s.rva, s.rva.saturating_add(s.raw_size.min(s.mem_size()))))
        .collect();
    let is_data_rva = |rva: u32| -> bool {
        data_ranges.iter().any(|&(lo, hi)| rva >= lo && rva < hi)
    };
    let rd32 = |rva: u32| -> Option<u32> {
        let b = pe.bytes_at(rva, 4).ok()?;
        Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    };
    let rd64 = |rva: u32| -> Option<u64> {
        let b = pe.bytes_at(rva, 8).ok()?;
        Some(u64::from_le_bytes(b.try_into().ok()?))
    };

    let mut roots: BTreeSet<AotCallbackRoot> = BTreeSet::new();

    // 1. TLS callbacks (`DataDirectory[9]`).
    if let Ok(Some(tls)) = pe.tls() {
        if let Some(tbl_rva) = va_to_rva(tls.callbacks_va) {
            for i in 0..1024u32 {
                let Some(va) = rd64(tbl_rva.saturating_add(i * 8)) else { break };
                if va == 0 {
                    break;
                }
                if let Some(rva) = va_to_rva(va) {
                    if is_plausible_code(rva) {
                        roots.insert(AotCallbackRoot {
                            rva,
                            kind: "tls_callback",
                            is_eh_continuation: false,
                        });
                    }
                }
            }
        }
    }

    // 2. `.pdata` image-backed unwind handlers + `C_SCOPE_TABLE` handlers & catch continuations.
    if let Ok(rfs) = pe.runtime_functions() {
        for rf in rfs {
            if rf.begin_rva >= rf.end_rva || rf.unwind_rva == 0 || (rf.unwind_rva & 1) != 0 {
                continue;
            }
            let Ok(hdr) = pe.bytes_at(rf.unwind_rva, 4) else { continue };
            let flags = hdr[0] >> 3;
            if (flags & 4) != 0 || (flags & (1 | 2)) == 0 {
                continue;
            }
            let count_codes = hdr[2] as u32;
            let handler_off = rf.unwind_rva.saturating_add(4 + ((count_codes + 1) & !1) * 2);
            if let Some(handler_rva) = rd32(handler_off) {
                if handler_rva != 0 && is_plausible_code(handler_rva) {
                    roots.insert(AotCallbackRoot {
                        rva: handler_rva,
                        kind: "unwind_handler",
                        is_eh_continuation: false,
                    });
                }
            }
            let scope_off = handler_off.saturating_add(4);
            if let Some(count) = rd32(scope_off) {
                if count > 0 && count <= 256 {
                    let mut all_valid = true;
                    for si in 0..count {
                        let rec = scope_off.saturating_add(4 + si * 16);
                        let Some(sb) = rd32(rec) else {
                            all_valid = false;
                            break;
                        };
                        let Some(se) = rd32(rec + 4) else {
                            all_valid = false;
                            break;
                        };
                        if !(sb >= rf.begin_rva && sb < se && se <= rf.end_rva) {
                            all_valid = false;
                            break;
                        }
                    }
                    if all_valid {
                        for si in 0..count {
                            let rec = scope_off.saturating_add(4 + si * 16);
                            if let Some(sh) = rd32(rec + 8) {
                                if sh > 1 && is_plausible_code(sh) {
                                    roots.insert(AotCallbackRoot {
                                        rva: sh,
                                        kind: "unwind_handler",
                                        is_eh_continuation: false,
                                    });
                                }
                            }
                            if let Some(sj) = rd32(rec + 12) {
                                if sj != 0
                                    && sj >= rf.begin_rva
                                    && sj < rf.end_rva
                                    && is_plausible_code(sj)
                                {
                                    roots.insert(AotCallbackRoot {
                                        rva: sj,
                                        kind: "eh_continuation",
                                        is_eh_continuation: true,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // 3. Load Config Directory (`DataDirectory[10]`): CFG functions, LongJump targets,
    //    and EH Continuation Table (distinct context: `is_eh_continuation = true`).
    let lc_dir = pe.headers.data_dirs[10];
    if lc_dir.rva != 0 && lc_dir.size >= 4 {
        if let Some(lc_size) = rd32(lc_dir.rva) {
            if lc_size >= 0x94 {
                let guard_flags = rd32(lc_dir.rva + 0x90).unwrap_or(0);
                let extra = ((guard_flags & 0xF000_0000) >> 28) as u32;
                let stride = 4 + extra;
                if (guard_flags & 0x0000_0400) != 0 {
                    if let (Some(tbl_va), Some(count)) =
                        (rd64(lc_dir.rva + 0x80), rd64(lc_dir.rva + 0x88))
                    {
                        if let Some(tbl_rva) = va_to_rva(tbl_va) {
                            for i in 0..count.min(262_144) as u32 {
                                if let Some(rva) = rd32(tbl_rva.saturating_add(i * stride)) {
                                    if is_plausible_code(rva) {
                                        roots.insert(AotCallbackRoot {
                                            rva,
                                            kind: "cfg_target",
                                            is_eh_continuation: false,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
                if lc_size >= 0x120 && (guard_flags & 0x0040_0000) != 0 {
                    if let (Some(tbl_va), Some(count)) =
                        (rd64(lc_dir.rva + 0x110), rd64(lc_dir.rva + 0x118))
                    {
                        if let Some(tbl_rva) = va_to_rva(tbl_va) {
                            for i in 0..count.min(262_144) as u32 {
                                if let Some(rva) = rd32(tbl_rva.saturating_add(i * stride)) {
                                    if is_plausible_code(rva) {
                                        roots.insert(AotCallbackRoot {
                                            rva,
                                            kind: "eh_continuation",
                                            is_eh_continuation: true,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // 4. `.CRT*` constructor / destructor pointer arrays.
    for sec in &pe.sections {
        if sec.name.starts_with(".CRT") && sec.raw_size >= 8 {
            let end = sec.rva.saturating_add(sec.raw_size.min(sec.mem_size()));
            let mut off = (sec.rva + 7) & !7;
            while off.saturating_add(8) <= end {
                if let Some(va) = rd64(off) {
                    if let Some(rva) = va_to_rva(va) {
                        if is_plausible_code(rva) {
                            roots.insert(AotCallbackRoot {
                                rva,
                                kind: "crt_ctor_dtor",
                                is_eh_continuation: false,
                            });
                        }
                    }
                }
                off += 8;
            }
        }
    }

    // 5. `DIR64` base relocations pointing into executable code (vtables / callback tables).
    let reloc_dir = pe.headers.data_dirs[5];
    if reloc_dir.rva != 0 && reloc_dir.size >= 8 {
        if let Ok(rb) = pe.bytes_at(reloc_dir.rva, reloc_dir.size as usize) {
            let mut pos = 0usize;
            while pos + 8 <= rb.len() {
                let page_rva = u32::from_le_bytes([rb[pos], rb[pos + 1], rb[pos + 2], rb[pos + 3]]);
                let blk_sz =
                    u32::from_le_bytes([rb[pos + 4], rb[pos + 5], rb[pos + 6], rb[pos + 7]])
                        as usize;
                if blk_sz < 8 || pos + blk_sz > rb.len() {
                    break;
                }
                let mut ep = pos + 8;
                while ep + 2 <= pos + blk_sz {
                    let ent = u16::from_le_bytes([rb[ep], rb[ep + 1]]);
                    if (ent >> 12) == 10 {
                        let slot_rva = page_rva.wrapping_add((ent & 0x0FFF) as u32);
                        if let Some(va) = rd64(slot_rva) {
                            if let Some(rva) = va_to_rva(va) {
                                if is_plausible_code(rva) {
                                    roots.insert(AotCallbackRoot {
                                        rva,
                                        kind: "reloc_code_ptr",
                                        is_eh_continuation: false,
                                    });
                                }
                            }
                        }
                    }
                    ep += 2;
                }
                pos += blk_sz;
            }
        }
    }

    // 6. Static callback registration call sites & COM vtable stores.
    #[derive(Clone, Copy)]
    enum Track {
        Unknown,
        Va(u64),
        StackOff(i64),
        Scalar(i64),
    }
    let gpr_i = |r: Register| -> Option<usize> {
        if r == Register::None {
            return None;
        }
        let full = r.full_register();
        full.is_gpr64().then(|| full.number())
    };
    let imm_val = |ins: &Instruction, op: u32| -> Option<i64> {
        match ins.op_kind(op) {
            OpKind::Immediate8 => Some(ins.immediate8() as i8 as i64),
            OpKind::Immediate16 => Some(ins.immediate16() as i16 as i64),
            OpKind::Immediate32 => Some(ins.immediate32() as i32 as i64),
            OpKind::Immediate64 => Some(ins.immediate64() as i64),
            OpKind::Immediate8to16 => Some(ins.immediate8to16() as i64),
            OpKind::Immediate8to32 => Some(ins.immediate8to32() as i64),
            OpKind::Immediate8to64 => Some(ins.immediate8to64()),
            OpKind::Immediate32to64 => Some(ins.immediate32to64()),
            _ => None,
        }
    };
    let mut scanned_vtables: BTreeSet<u32> = BTreeSet::new();
    let mut scan_com_vtable = |vt_rva: u32, roots: &mut BTreeSet<AotCallbackRoot>| {
        if !scanned_vtables.insert(vt_rva) {
            return;
        }
        let mut slots = Vec::new();
        for i in 0..32u32 {
            let Some(va) = rd64(vt_rva.saturating_add(i * 8)) else { break };
            let Some(rva) = va_to_rva(va) else { break };
            if !is_plausible_code(rva) {
                break;
            }
            slots.push(rva);
        }
        if slots.len() >= 3 {
            for rva in slots {
                roots.insert(AotCallbackRoot {
                    rva,
                    kind: "com_sink",
                    is_eh_continuation: false,
                });
            }
        }
    };

    for &(sec_lo, sec_hi) in exec_ranges {
        let len = (sec_hi - sec_lo) as usize;
        let Ok(code) = pe.bytes_at(sec_lo, len) else { continue };
        let mut dec = Decoder::with_ip(64, code, image_base + sec_lo as u64, DecoderOptions::NONE);
        let mut ins = Instruction::default();
        let mut regs = [Track::Unknown; 16];
        let mut stack: BTreeMap<i64, Track> = BTreeMap::new();

        while dec.can_decode() {
            dec.decode_out(&mut ins);
            if ins.is_invalid() {
                regs = [Track::Unknown; 16];
                stack.clear();
                continue;
            }
            match ins.mnemonic() {
                Mnemonic::Lea => {
                    if ins.op0_kind() == OpKind::Register {
                        if let Some(d) = gpr_i(ins.op0_register()) {
                            regs[d] = if ins.is_ip_rel_memory_operand() {
                                Track::Va(ins.ip_rel_memory_address())
                            } else if ins.memory_base() == Register::RSP
                                && ins.memory_index() == Register::None
                            {
                                Track::StackOff(ins.memory_displacement64() as i64)
                            } else {
                                Track::Unknown
                            };
                        }
                    }
                }
                Mnemonic::Mov => match (ins.op0_kind(), ins.op1_kind()) {
                    (OpKind::Register, OpKind::Register) => {
                        if let (Some(d), Some(s)) =
                            (gpr_i(ins.op0_register()), gpr_i(ins.op1_register()))
                        {
                            regs[d] = regs[s];
                        }
                    }
                    (OpKind::Register, _) if imm_val(&ins, 1).is_some() => {
                        if let Some(d) = gpr_i(ins.op0_register()) {
                            let imm = imm_val(&ins, 1).unwrap();
                            regs[d] = if va_to_rva(imm as u64).is_some() {
                                Track::Va(imm as u64)
                            } else {
                                Track::Scalar(imm)
                            };
                        }
                    }
                    (OpKind::Memory, OpKind::Register) => {
                        if let Some(s) = gpr_i(ins.op1_register()) {
                            let val = regs[s];
                            if ins.memory_base() == Register::RSP
                                && ins.memory_index() == Register::None
                            {
                                let off = ins.memory_displacement64() as i64;
                                if (0..=0x200).contains(&off) {
                                    stack.insert(off, val);
                                }
                            } else if let Some(b) = gpr_i(ins.memory_base()) {
                                if ins.memory_index() == Register::None {
                                    if let Track::StackOff(so) = regs[b] {
                                        let off =
                                            so.wrapping_add(ins.memory_displacement64() as i64);
                                        if (0..=0x200).contains(&off) {
                                            stack.insert(off, val);
                                        }
                                    }
                                    if ins.memory_displacement64() == 0 {
                                        if let Track::Va(vt_va) = val {
                                            if let Some(vt_rva) = va_to_rva(vt_va) {
                                                if is_data_rva(vt_rva) {
                                                    scan_com_vtable(vt_rva, &mut roots);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                },
                Mnemonic::Xor => {
                    if ins.op0_kind() == OpKind::Register && ins.op1_kind() == OpKind::Register {
                        if let (Some(d), Some(s)) =
                            (gpr_i(ins.op0_register()), gpr_i(ins.op1_register()))
                        {
                            regs[d] = if d == s { Track::Scalar(0) } else { Track::Unknown };
                        }
                    }
                }
                Mnemonic::Call => {
                    let iat_rva = if ins.op0_kind() == OpKind::Memory
                        && ins.is_ip_rel_memory_operand()
                    {
                        va_to_rva(ins.ip_rel_memory_address())
                    } else if matches!(
                        ins.op0_kind(),
                        OpKind::NearBranch64 | OpKind::NearBranch32 | OpKind::NearBranch16
                    ) {
                        va_to_rva(ins.near_branch_target())
                            .and_then(|tr| iat_thunks.get(&tr).copied())
                    } else {
                        None
                    };
                    if let Some(imp) = iat_rva.and_then(|r| imports_by_rva.get(&r)) {
                        let norm_sym = if (imp.canonical_dll == "comctl32.dll"
                            || imp.canonical_dll == "uxtheme.dll")
                            && (imp.symbol == "#410" || imp.symbol == "ordinal_410")
                        {
                            "SetWindowSubclass"
                        } else {
                            imp.symbol.as_str()
                        };
                        let arg = |idx: u8| -> Track {
                            match idx {
                                0 => regs[1],
                                1 => regs[2],
                                2 => regs[8],
                                3 => regs[9],
                                n => stack
                                    .get(&(0x20 + ((n - 4) as i64) * 8))
                                    .copied()
                                    .unwrap_or(Track::Unknown),
                            }
                        };
                        let direct: Option<(u8, &'static str)> = match norm_sym {
                            "CreateThread" | "_beginthreadex" => Some((2, "worker")),
                            "_beginthread"
                            | "CreateThreadpoolWork"
                            | "TrySubmitThreadpoolCallback"
                            | "QueueUserWorkItem"
                            | "RtlQueueWorkItem" => Some((0, "worker")),
                            "CreateThreadpoolTimer" => Some((0, "timer")),
                            "CreateTimerQueueTimer" | "timeSetEvent" => Some((2, "timer")),
                            "SetTimer" | "SetWaitableTimer" | "SetWaitableTimerEx" => {
                                Some((3, "timer"))
                            }
                            "CreateThreadpoolWait" => Some((0, "wait")),
                            "RegisterWaitForSingleObject" | "RtlRegisterWait" => Some((2, "wait")),
                            "QueueUserAPC" | "QueueUserAPC2" => Some((0, "apc")),
                            "CreateThreadpoolIo" => Some((1, "apc")),
                            "ReadFileEx" | "WriteFileEx" => Some((4, "apc")),
                            "FlsAlloc" => Some((0, "fls")),
                            "SetWindowSubclass"
                            | "SetWindowsHookExW"
                            | "SetWindowsHookExA"
                            | "EnumChildWindows" => Some((1, "wndproc")),
                            "EnumWindows" | "EnumThreadWindows" => Some((0, "wndproc")),
                            "EnumDisplayMonitors" => Some((2, "wndproc")),
                            "DialogBoxParamW"
                            | "DialogBoxParamA"
                            | "DialogBoxIndirectParamW"
                            | "DialogBoxIndirectParamA"
                            | "CreateDialogParamW"
                            | "CreateDialogParamA"
                            | "CreateDialogIndirectParamW"
                            | "CreateDialogIndirectParamA" => Some((3, "wndproc")),
                            "atexit" | "_onexit" | "_crt_atexit" | "_crt_at_quick_exit" => {
                                Some((0, "crt_ctor_dtor"))
                            }
                            "_register_onexit_function" => Some((1, "crt_ctor_dtor")),
                            "AddVectoredExceptionHandler" | "AddVectoredContinueHandler" => {
                                Some((1, "unwind_handler"))
                            }
                            "SetUnhandledExceptionFilter" => Some((0, "unwind_handler")),
                            "RtlInstallFunctionTableCallback" => Some((3, "unwind_handler")),
                            _ => None,
                        };
                        if let Some((arg_idx, kind)) = direct {
                            if let Track::Va(va) = arg(arg_idx) {
                                if let Some(rva) = va_to_rva(va) {
                                    if is_plausible_code(rva) {
                                        roots.insert(AotCallbackRoot {
                                            rva,
                                            kind,
                                            is_eh_continuation: false,
                                        });
                                    }
                                }
                            }
                        } else if matches!(
                            norm_sym,
                            "RegisterClassW"
                                | "RegisterClassA"
                                | "RegisterClassExW"
                                | "RegisterClassExA"
                        ) {
                            match arg(0) {
                                Track::StackOff(so) => {
                                    if let Some(Track::Va(va)) = stack.get(&(so + 8)).copied() {
                                        if let Some(rva) = va_to_rva(va) {
                                            if is_plausible_code(rva) {
                                                roots.insert(AotCallbackRoot {
                                                    rva,
                                                    kind: "wndproc",
                                                    is_eh_continuation: false,
                                                });
                                            }
                                        }
                                    }
                                }
                                Track::Va(va) => {
                                    if let Some(srva) = va_to_rva(va) {
                                        if let Some(wp_va) = rd64(srva.saturating_add(8)) {
                                            if let Some(rva) = va_to_rva(wp_va) {
                                                if is_plausible_code(rva) {
                                                    roots.insert(AotCallbackRoot {
                                                        rva,
                                                        kind: "wndproc",
                                                        is_eh_continuation: false,
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        } else if matches!(norm_sym, "SetWindowLongPtrW" | "SetWindowLongPtrA") {
                            if matches!(arg(1), Track::Scalar(-4) | Track::Scalar(8)) {
                                if let Track::Va(va) = arg(2) {
                                    if let Some(rva) = va_to_rva(va) {
                                        if is_plausible_code(rva) {
                                            roots.insert(AotCallbackRoot {
                                                rva,
                                                kind: "wndproc",
                                                is_eh_continuation: false,
                                            });
                                        }
                                    }
                                }
                            }
                        } else if matches!(norm_sym, "_initterm" | "_initterm_e") {
                            if let (Track::Va(lo_va), Track::Va(hi_va)) = (arg(0), arg(1)) {
                                if let (Some(r_lo), Some(r_hi)) =
                                    (va_to_rva(lo_va), va_to_rva(hi_va))
                                {
                                    if r_lo < r_hi && (r_hi - r_lo) <= 0x10_000 {
                                        let mut p = r_lo;
                                        while p.saturating_add(8) <= r_hi {
                                            if let Some(cb_va) = rd64(p) {
                                                if let Some(rva) = va_to_rva(cb_va) {
                                                    if is_plausible_code(rva) {
                                                        roots.insert(AotCallbackRoot {
                                                            rva,
                                                            kind: "crt_ctor_dtor",
                                                            is_eh_continuation: false,
                                                        });
                                                    }
                                                }
                                            }
                                            p += 8;
                                        }
                                    }
                                }
                            }
                        } else if norm_sym == "CoRegisterClassObject" {
                            if let Track::Va(obj_va) = arg(1) {
                                if let Some(obj_rva) = va_to_rva(obj_va) {
                                    if let Some(vt_va) = rd64(obj_rva) {
                                        if let Some(vt_rva) = va_to_rva(vt_va) {
                                            if is_data_rva(vt_rva) {
                                                scan_com_vtable(vt_rva, &mut roots);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    for r in [0usize, 1, 2, 8, 9, 10, 11] {
                        regs[r] = Track::Unknown;
                    }
                }
                Mnemonic::Ret | Mnemonic::Int3 | Mnemonic::Ud2 | Mnemonic::Jmp => {
                    regs = [Track::Unknown; 16];
                    stack.clear();
                }
                _ => {
                    if ins.flow_control() != FlowControl::ConditionalBranch
                        && ins.op_count() > 0
                        && ins.op0_kind() == OpKind::Register
                    {
                        if let Some(d) = gpr_i(ins.op0_register()) {
                            regs[d] = Track::Unknown;
                        }
                    }
                }
            }
        }
    }

    roots.into_iter().collect()
}
