//! Rust implementations of the Windows APIs used by `AlanWake2.exe` and its
//! bundled third-party DLLs, for the hybrid (Rosetta 2) runtime.
//!
//! * Every exported function is `extern "win64"` with the exact Windows ABI
//!   (`LONG`/`DWORD`/`BOOL` = 32 bit, `WCHAR` = UTF-16 `u16`, HANDLE = pointer).
//! * [`lookup`] resolves `dll!name` with API-set normalization
//!   (`api-ms-win-crt-*` -> ucrtbase, `api-ms-win-core-*` -> kernel32, ...).
//! * Things that are not (yet) implemented faithfully go through [`stub!`] so
//!   they can be grepped for (`STUB` in the log).
//! * The per-thread `TEB` lives at `gs:[0x30]` (see [`teb`]); last-error is
//!   stored in the real TEB field `LastErrorValue` (+0x68).
//!
//! This crate only builds for x86_64 (the hybrid runtime target); on other
//! architectures it is empty.
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![allow(
    clippy::missing_safety_doc,
    clippy::too_many_arguments,
)]

#[macro_use]
pub mod macros;

win64_abi::win64_mods! {
    pub mod bridge;
    pub mod types;
    pub mod strings;
    pub(crate) mod com;
    pub(crate) mod net_tls;
    pub mod teb;
    pub mod handles;
    pub mod park;
    pub mod heap;
    pub mod sync;
    pub mod path;
    pub mod process;
    pub mod thread;
    pub mod crash;
    pub mod hitch;
    pub mod unwind;
    pub mod va;

    pub mod kernel32;
    pub mod ntdll;
    pub mod ntsyscall;
    pub mod ucrt;
    pub mod msvcrt;
    pub mod user32;
    pub mod advapi32;
    pub mod misc;
    pub mod macos;
    pub mod audio;
    pub mod d3d12;
    pub mod dxgi;
    pub mod d3d11;
    pub mod slstub;
    pub mod eossdk;
}

pub mod config;
pub mod det_entropy;
pub mod middleware;
pub mod present;
pub mod shadow;
/// Cross-Apple platform backend alias (`AppKit` on macOS, `UIKit`/host-surface on iOS).
pub use macos as platform;
/// Re-exported for `aw2-game` (`shadow`), which has no direct `libc` dependency.
pub use libc;

pub(crate) use audio::com::com_iunknown;

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{OnceLock, RwLock};

static EXTRA_DLLS: RwLock<Vec<BuiltinDll>> = RwLock::new(Vec::new());
static EXTRA_ALIASES: RwLock<Vec<(String, String)>> = RwLock::new(Vec::new());
static EXPORT_OVERRIDES: RwLock<Vec<(String, String, Ptr)>> = RwLock::new(Vec::new());

/// Register an additional builtin DLL at startup (before PE loading resolves imports).
/// Allows game targets and optional middleware crates to plug in DLLs without editing `win32-rt`.
pub fn register_builtin_dll(dll: BuiltinDll) {
    EXTRA_DLLS.write().unwrap().push(dll);
}

/// Register a dynamic DLL alias (`from` -> `to`, both normalized to lowercase with `.dll`).
pub fn register_dll_alias(from: impl AsRef<str>, to: impl AsRef<str>) {
    let f = pe_image::normalize_dll_name(from.as_ref());
    let t = pe_image::normalize_dll_name(to.as_ref());
    EXTRA_ALIASES.write().unwrap().push((f, t));
}

/// Override a specific `dll!name` export at runtime (takes precedence over builtin tables).
pub fn register_export_override(dll: impl AsRef<str>, name: impl Into<String>, ptr: *const ()) {
    let d = canonical_dll(dll.as_ref());
    EXPORT_OVERRIDES.write().unwrap().push((d, name.into(), Ptr(ptr)));
}

type GuestDispatcherFn = unsafe fn(u64, u64, u64, u64, u64, u64, u64, u64, u64) -> u64;
type NativeVaRegistrarFn = fn(u64, usize);
type BoundaryNanHookFn = fn(&str, usize, usize, &str);

static GUEST_DISPATCHER: AtomicUsize = AtomicUsize::new(0);
static NATIVE_VA_REGISTRAR: AtomicUsize = AtomicUsize::new(0);
static BOUNDARY_NAN_HOOK: AtomicUsize = AtomicUsize::new(0);

/// Register a universal diagnostic hook called when a Win32/middleware boundary receives
/// non-finite floating-point inputs (`PORT_NAN_BOUNDARY=1`). Implemented by `x86-runtime::diag`
/// to unwind and log the exact guest call stack at zero cost when disabled.
pub fn set_boundary_nan_hook(f: BoundaryNanHookFn) {
    BOUNDARY_NAN_HOOK.store(f as usize, Ordering::Release);
}

/// Notify the registered diagnostic hook (if any) that non-finite floating-point data
/// arrived at a Win32/middleware boundary.
#[inline]
pub fn report_boundary_nan(tag: &str, ptr: usize, len: usize, detail: &str) {
    let h = BOUNDARY_NAN_HOOK.load(Ordering::Acquire);
    if h != 0 {
        let f: BoundaryNanHookFn = unsafe { std::mem::transmute(h) };
        f(tag, ptr, len, detail);
    }
}

/// Register the guest callback dispatcher (`aw2_game::exe::call_va8`) so `win32-rt`
/// can invoke guest function pointers (`_initterm`, `CreateThread`, `_beginthreadex`,
/// `InitOnceExecuteOnce`, `qsort`, `WNDPROC`, etc.) on both `x86_64` and `aarch64`.
pub fn set_guest_dispatcher(f: GuestDispatcherFn) {
    GUEST_DISPATCHER.store(f as usize, Ordering::Release);
}

/// Returns `true` when a guest x86-64 translator dispatcher has been installed via
/// [`set_guest_dispatcher`].
#[inline]
pub fn has_guest_dispatcher() -> bool {
    GUEST_DISPATCHER.load(Ordering::Acquire) != 0
}

/// Register the `VA_DISPATCH` registrar (`aw2_game::exe::register_native_va`) so COM
/// vtable method pointers and dynamically resolved builtins are known to `emu_run`.
pub fn set_native_va_registrar(f: NativeVaRegistrarFn) {
    NATIVE_VA_REGISTRAR.store(f as usize, Ordering::Release);
}

/// Register a native host function pointer `ptr` in `VA_DISPATCH`.
#[inline]
pub fn register_native_va(va: u64, ptr: usize) {
    let r = NATIVE_VA_REGISTRAR.load(Ordering::Acquire);
    if r != 0 {
        let f: NativeVaRegistrarFn = unsafe { std::mem::transmute(r) };
        f(va, ptr);
    }
}

/// Call a guest or native function pointer `addr` with up to 8 Win64 arguments.
///
/// The Rust caller is a frame without unwind info: a C++ exception thrown by the callback
/// and not caught inside it must not be caught by a frame beyond this call (that would
/// discard the Rust frames, as `longjmp` would). The call is therefore wrapped in an opaque
/// EH barrier record ([`unwind::EhBarrier`]); a dispatch walk reaching it stops, as Windows
/// stops at a frame without unwind info.
#[inline]
pub unsafe fn call_guest8(
    addr: usize,
    rcx: u64,
    rdx: u64,
    r8: u64,
    r9: u64,
    arg5: u64,
    arg6: u64,
    arg7: u64,
    arg8: u64,
) -> u64 {
    let mut b = crate::unwind::EhBarrier::new(addr);
    let _g = b.push();
    call_guest8_seh(addr, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8)
}

/// [`call_guest8`] without the EH barrier: for the exception machinery itself
/// (language handlers, consolidate callbacks, vectored handlers), whose crossing record
/// links the callback's frames to the raise site.
#[inline]
pub unsafe fn call_guest8_seh(
    addr: usize,
    rcx: u64,
    rdx: u64,
    r8: u64,
    r9: u64,
    arg5: u64,
    arg6: u64,
    arg7: u64,
    arg8: u64,
) -> u64 {
    let d = GUEST_DISPATCHER.load(Ordering::Acquire);
    if d != 0 {
        let f: GuestDispatcherFn = std::mem::transmute(d);
        return f(addr as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
    }
    #[cfg(target_arch = "x86_64")]
    {
        let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =
            std::mem::transmute(addr);
        f(rcx, rdx, r8, r9, arg5, arg6, arg7, arg8)
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let f: unsafe extern "C" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =
            std::mem::transmute(addr);
        f(rcx, rdx, r8, r9, arg5, arg6, arg7, arg8)
    }
}

/// Pointer wrapper so export tables can live in `static`s.
#[derive(Clone, Copy, Debug)]
pub struct Ptr(pub *const ());
unsafe impl Sync for Ptr {}
unsafe impl Send for Ptr {}

/// One exported symbol (function or data) of a builtin DLL.
#[derive(Clone, Copy, Debug)]
pub struct Export {
    pub name: &'static str,
    pub ptr: Ptr,
}

/// A builtin (Rust-implemented) DLL.
#[derive(Clone, Copy)]
pub struct BuiltinDll {
    /// Canonical lowercase name, e.g. `"kernel32.dll"`.
    pub name: &'static str,
    pub exports: &'static [Export],
    /// Ordinal -> name (for DLLs commonly imported by ordinal, e.g. ws2_32).
    pub ordinals: &'static [(u16, &'static str)],
}

/// All builtin DLLs. A DLL listed here is "known" even if it exports nothing
/// yet; imports from it that are missing become trap thunks in the loader.
pub fn builtin_dlls() -> &'static [BuiltinDll] {
    static DLLS: OnceLock<Vec<BuiltinDll>> = OnceLock::new();
    DLLS.get_or_init(|| {
        let mut v = vec![
            BuiltinDll { name: "kernel32.dll", exports: kernel32::EXPORTS, ordinals: &[] },
            BuiltinDll { name: "ntdll.dll", exports: ntdll::EXPORTS, ordinals: &[] },
            BuiltinDll { name: "ucrtbase.dll", exports: ucrt::EXPORTS, ordinals: &[] },
            BuiltinDll { name: "msvcrt.dll", exports: msvcrt::EXPORTS, ordinals: &[] },
            BuiltinDll { name: "user32.dll", exports: user32::EXPORTS, ordinals: &[] },
            BuiltinDll { name: "advapi32.dll", exports: advapi32::EXPORTS, ordinals: &[] },
            BuiltinDll { name: "dxgi.dll", exports: dxgi::EXPORTS, ordinals: dxgi::ORDINALS },
            BuiltinDll { name: "d3d12.dll", exports: d3d12::EXPORTS, ordinals: d3d12::ORDINALS },
            BuiltinDll { name: "d3d11.dll", exports: d3d11::EXPORTS, ordinals: &[] },
            BuiltinDll { name: "sl.interposer.dll", exports: slstub::EXPORTS, ordinals: &[] },
            BuiltinDll { name: "eossdk-win64-shipping.dll", exports: eossdk::EXPORTS, ordinals: &[] },
        ];
        v.extend(misc::dlls());
        v.extend(EXTRA_DLLS.read().unwrap().iter().copied());
        v
    })
}

/// Map API-set and alias DLL names to the canonical builtin module that implements them.
/// Input may be any case, with or without directory / `.dll`.
pub fn canonical_dll(dll: &str) -> String {
    let n = pe_image::normalize_dll_name(dll);
    if let Ok(aliases) = EXTRA_ALIASES.read() {
        for (from, to) in aliases.iter().rev() {
            if *from == n {
                return to.clone();
            }
        }
    }
    port_manifest::canonical_dll(&n)
}

pub fn find_builtin(dll: &str) -> Option<&'static BuiltinDll> {
    let c = canonical_dll(dll);
    builtin_dlls().iter().find(|d| d.name == c)
}

fn tables() -> &'static HashMap<&'static str, HashMap<&'static str, Ptr>> {
    static T: OnceLock<HashMap<&'static str, HashMap<&'static str, Ptr>>> = OnceLock::new();
    T.get_or_init(|| {
        builtin_dlls()
            .iter()
            .map(|d| (d.name, d.exports.iter().map(|e| (e.name, e.ptr)).collect()))
            .collect()
    })
}

/// Resolve `dll!name` to a Rust implementation.
pub fn lookup(dll: &str, name: &str) -> Option<*const ()> {
    let c = canonical_dll(dll);
    if let Ok(ov) = EXPORT_OVERRIDES.read() {
        for (od, on, op) in ov.iter().rev() {
            if od == &c && on == name {
                return Some(op.0);
            }
        }
    }
    let t = tables();
    if let Some(p) = t.get(c.as_str()).and_then(|m| m.get(name)) {
        return Some(p.0);
    }
    // kernel32 forwards a lot of its surface to ntdll (Rtl*); msvcrt falls back
    // to ucrtbase for functions with identical ABI.
    let fallback = match c.as_str() {
        "kernel32.dll" => Some("ntdll.dll"),
        "ntdll.dll" => Some("kernel32.dll"),
        "msvcrt.dll" => Some("ucrtbase.dll"),
        _ => None,
    };
    fallback.and_then(|f| t.get(f)).and_then(|m| m.get(name)).map(|p| p.0)
}

/// Resolve by ordinal (only for DLLs with an ordinal table).
pub fn lookup_ordinal(dll: &str, ordinal: u16) -> Option<*const ()> {
    let b = find_builtin(dll)?;
    let name = b.ordinals.iter().find(|(o, _)| *o == ordinal)?.1;
    lookup(dll, name)
}

/// Name of the builtin export at exactly `addr`, if any (for symbolization).
pub fn builtin_name_for(addr: usize) -> Option<(&'static str, &'static str)> {
    for d in builtin_dlls() {
        for e in d.exports {
            if e.ptr.0 as usize == addr {
                return Some((d.name, e.name));
            }
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Loader interface (implemented by aw2-loader, used by LoadLibrary & co.)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub enum ProcName<'a> {
    Name(&'a str),
    Ordinal(u16),
}

impl std::fmt::Display for ProcName<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcName::Name(n) => f.write_str(n),
            ProcName::Ordinal(o) => write!(f, "#{o}"),
        }
    }
}

/// Closed set of verified Winsock2 `WSAIoctl(SIO_GET_EXTENSION_FUNCTION_POINTER)` extensions.
///
/// Only these fixed extensions can be provisioned as guest-visible, guest-callable thunks;
/// this intentionally never forms a generic arbitrary-target host trampoline.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum WsaExtension {
    AcceptEx = 0,
    ConnectEx = 1,
    DisconnectEx = 2,
    TransmitFile = 3,
    GetAcceptExSockaddrs = 4,
}

impl WsaExtension {
    pub const ALL: [WsaExtension; 5] = [
        WsaExtension::AcceptEx,
        WsaExtension::ConnectEx,
        WsaExtension::DisconnectEx,
        WsaExtension::TransmitFile,
        WsaExtension::GetAcceptExSockaddrs,
    ];

    pub const fn symbol_name(self) -> &'static str {
        match self {
            WsaExtension::AcceptEx => "ws2_32!AcceptEx",
            WsaExtension::ConnectEx => "ws2_32!ConnectEx",
            WsaExtension::DisconnectEx => "ws2_32!DisconnectEx",
            WsaExtension::TransmitFile => "ws2_32!TransmitFile",
            WsaExtension::GetAcceptExSockaddrs => "ws2_32!GetAcceptExSockaddrs",
        }
    }
}

/// Services the PE loader provides to the Win32 layer.
pub trait LoaderOps: Send + Sync {
    /// `LoadLibraryExW` semantics. Returns the HMODULE or a Win32 error code.
    fn load_library(&self, name: &str, flags: u32) -> Result<usize, u32>;
    /// `LdrLoadDll` semantics with an optional semicolon-separated search path hint (`PathToFile`).
    fn load_library_with_search_path(
        &self,
        name: &str,
        flags: u32,
        search_path: Option<&str>,
    ) -> Result<usize, u32> {
        let _ = search_path;
        self.load_library(name, flags)
    }
    fn free_library(&self, hmod: usize) -> bool;
    /// `None` = the main executable.
    fn get_module_handle(&self, name: Option<&str>) -> Option<usize>;
    fn get_proc_address(&self, hmod: usize, name: ProcName<'_>) -> Option<usize>;
    /// Provision and return a stable, guest-visible, guest-callable thunk VA for one of the
    /// fixed verified Winsock2 extensions (never a raw host code pointer). Returns `None` if
    /// thunk provisioning is unavailable.
    fn wsa_extension_thunk(&self, _ext: WsaExtension) -> Option<usize> {
        None
    }
    /// Windows-style full path of a module (`None` = main exe).
    fn module_path(&self, hmod: Option<usize>) -> Option<String>;
    /// HMODULE of the image containing `addr`.
    fn module_from_address(&self, addr: usize) -> Option<usize>;
    /// Returns `true` strictly if `addr` lies within an executable code section of a real
    /// mapped guest PE image (excluding synthetic builtin DLLs, PE headers, and data sections).
    fn is_guest_pe_code(&self, _addr: usize) -> bool {
        false
    }
    /// All currently loaded modules as `(image_base, image_size, win_path)`.
    fn loaded_modules(&self) -> Vec<(usize, u32, String)> {
        Vec::new()
    }
    /// Allocate static TLS for a new thread (fills TEB+0x58) and, unless
    /// `is_initial`, run TLS callbacks / DllMain(DLL_THREAD_ATTACH).
    fn thread_attach(&self, teb: *mut u8, is_initial: bool);
    /// Run DLL_THREAD_DETACH notifications and free static TLS.
    fn thread_detach(&self, teb: *mut u8);
    /// `RtlLookupFunctionEntry`: (image base, RUNTIME_FUNCTION*).
    fn lookup_function_entry(&self, pc: u64) -> Option<(u64, *const pe_image::RuntimeFunction)>;
    fn disable_thread_library_calls(&self, hmod: usize) -> bool;
    /// Human-readable symbolization (`module+0xrva (nearest export)`).
    fn symbolize(&self, addr: usize) -> String;
    /// Block until symbol tables (fndb `.syms` files) are loaded, so that
    /// [`symbolize`](Self::symbolize) names exe functions (crash reports).
    fn wait_symbols(&self) {}
    /// Disassemble up to `max` instructions of `bytes` (located at `addr`),
    /// one formatted line each (crash reports).
    fn disassemble(&self, _addr: u64, _bytes: &[u8], _max: usize) -> Vec<String> {
        Vec::new()
    }
}

static LOADER: OnceLock<&'static dyn LoaderOps> = OnceLock::new();

pub fn set_loader(l: &'static dyn LoaderOps) {
    if LOADER.set(l).is_err() {
        panic!("win32_rt::set_loader called twice");
    }
}

pub fn loader() -> &'static dyn LoaderOps {
    *LOADER.get().expect("win32-rt used before the loader registered itself")
}

pub fn try_loader() -> Option<&'static dyn LoaderOps> {
    LOADER.get().copied()
}

/// Resolve a stable guest-visible thunk VA for `ext` via the registered loader (or a
/// `#[cfg(test)]`-only scoped test provider). Never returns 0 or a raw host code pointer.
pub(crate) fn resolve_wsa_extension_thunk(ext: WsaExtension, raw_host: usize) -> Option<usize> {
    #[cfg(test)]
    let thunk = tests::test_wsa_extension_thunk(ext)
        .or_else(|| try_loader().and_then(|l| l.wsa_extension_thunk(ext)));
    #[cfg(not(test))]
    let thunk = try_loader().and_then(|l| l.wsa_extension_thunk(ext));
    thunk.filter(|&va| va != 0 && va != raw_host)
}

/// Symbolize an address via the loader if available.
pub fn symbolize(addr: usize) -> String {
    match try_loader() {
        Some(l) => l.symbolize(addr),
        None => format!("0x{addr:x}"),
    }
}

/// Pure-integer leaf Win32/CRT builtins (<= 8 integer args, no XMM args/ret, never call
/// back into guest code) eligible for `DF_PLAIN` direct calls. Built inside `win32-rt` so
/// every function pointer is guaranteed identical to the export-table pointer.
pub fn plain_builtin_fns() -> &'static [usize] {
    static FNS: OnceLock<Box<[usize]>> = OnceLock::new();
    FNS.get_or_init(|| {
        let mut v = vec![
            msvcrt::memcmp as *const () as usize,
            misc::vcruntime140::memcmp as *const () as usize,
            msvcrt::memcpy as *const () as usize,
            msvcrt::memmove as *const () as usize,
            msvcrt::memset as *const () as usize,
            msvcrt::memchr as *const () as usize,
            misc::vcruntime140::memcpy as *const () as usize,
            misc::vcruntime140::memmove as *const () as usize,
            misc::vcruntime140::memset as *const () as usize,
            misc::vcruntime140::memchr as *const () as usize,
            misc::vcruntime140::strchr as *const () as usize,
            misc::vcruntime140::strstr as *const () as usize,
            misc::vcruntime140::wcschr as *const () as usize,
            misc::vcruntime140::wcsrchr as *const () as usize,
            misc::vcruntime140::wcsstr as *const () as usize,
            misc::vcruntime140::__std_type_info_compare as *const () as usize,
            misc::vcruntime140::__std_type_info_hash as *const () as usize,
            misc::vcruntime140::__std_type_info_name as *const () as usize,
            misc::vcruntime140::__AdjustPointer as *const () as usize,
            // Kernel32 / sync / ntdll pure-host leaf primitives
            kernel32::threads::GetCurrentThread as *const () as usize,
            kernel32::threads::GetCurrentThreadId as *const () as usize,
            kernel32::threads::GetCurrentProcessorNumber as *const () as usize,
            kernel32::threads::TlsGetValue as *const () as usize,
            kernel32::threads::TlsSetValue as *const () as usize,
            kernel32::threads::SetEvent as *const () as usize,
            kernel32::threads::ResetEvent as *const () as usize,
            kernel32::threads::ReleaseSemaphore as *const () as usize,
            kernel32::process::GetCurrentProcess as *const () as usize,
            kernel32::process::GetCurrentProcessId as *const () as usize,
            kernel32::process::GetLastError as *const () as usize,
            kernel32::process::SetLastError as *const () as usize,
            kernel32::time::SwitchToThread as *const () as usize,
            kernel32::time::QueryPerformanceCounter as *const () as usize,
            kernel32::time::QueryPerformanceFrequency as *const () as usize,
            kernel32::time::GetTickCount as *const () as usize,
            kernel32::time::GetTickCount64 as *const () as usize,
            kernel32::time::QueryInterruptTime as *const () as usize,
            kernel32::time::QueryUnbiasedInterruptTime as *const () as usize,
            misc::winmm::timeGetTime as *const () as usize,
            sync::AcquireSRWLockExclusive as *const () as usize,
            sync::ReleaseSRWLockExclusive as *const () as usize,
            sync::TryAcquireSRWLockExclusive as *const () as usize,
            sync::AcquireSRWLockShared as *const () as usize,
            sync::ReleaseSRWLockShared as *const () as usize,
            sync::TryAcquireSRWLockShared as *const () as usize,
            sync::InitializeSRWLock as *const () as usize,
            sync::EnterCriticalSection as *const () as usize,
            sync::TryEnterCriticalSection as *const () as usize,
            sync::LeaveCriticalSection as *const () as usize,
            sync::DeleteCriticalSection as *const () as usize,
            sync::WakeConditionVariable as *const () as usize,
            sync::WakeAllConditionVariable as *const () as usize,
            sync::SleepConditionVariableCS as *const () as usize,
            sync::SleepConditionVariableSRW as *const () as usize,
            sync::WaitOnAddress as *const () as usize,
            sync::WakeByAddressSingle as *const () as usize,
            sync::WakeByAddressAll as *const () as usize,
            ntdll::RtlZeroMemory as *const () as usize,
            ntdll::RtlFillMemory as *const () as usize,
            ntdll::RtlMoveMemory as *const () as usize,
            ntdll::RtlCopyMemory as *const () as usize,
            ntdll::RtlCompareMemory as *const () as usize,
            ntdll::RtlInitializeCriticalSection as *const () as usize,
            ntdll::RtlInitializeCriticalSectionAndSpinCount as *const () as usize,
            ntdll::RtlEnterCriticalSection as *const () as usize,
            ntdll::RtlTryEnterCriticalSection as *const () as usize,
            ntdll::RtlLeaveCriticalSection as *const () as usize,
            ntdll::RtlDeleteCriticalSection as *const () as usize,
            ntdll::RtlIsCriticalSectionLockedByThread as *const () as usize,
            ntdll::RtlAcquireSRWLockExclusive as *const () as usize,
            ntdll::RtlReleaseSRWLockExclusive as *const () as usize,
            ntdll::RtlAcquireSRWLockShared as *const () as usize,
            ntdll::RtlReleaseSRWLockShared as *const () as usize,
            ntdll::RtlInitializeSRWLock as *const () as usize,
            ntdll::RtlQueryPerformanceCounter as *const () as usize,
            ntdll::RtlQueryPerformanceFrequency as *const () as usize,
            // UCRT pure-host leaf heap/locale/ctype/string functions (never invoke guest callbacks)
            ucrt::heap::free as *const () as usize,
            ucrt::heap::_free_base as *const () as usize,
            ucrt::locale::_lock_locales as *const () as usize,
            ucrt::locale::_unlock_locales as *const () as usize,
            ucrt::string::__pctype_func as *const () as usize,
            ucrt::string::__pwctype_func as *const () as usize,
            ucrt::string::tolower as *const () as usize,
            ucrt::string::toupper as *const () as usize,
            ucrt::string::_tolower as *const () as usize,
            ucrt::string::_toupper as *const () as usize,
            ucrt::string::_tolower_l as *const () as usize,
            ucrt::string::_toupper_l as *const () as usize,
            ucrt::string::towlower as *const () as usize,
            ucrt::string::towupper as *const () as usize,
            ucrt::string::_towlower_l as *const () as usize,
            ucrt::string::_towupper_l as *const () as usize,
            ucrt::string::isalpha as *const () as usize,
            ucrt::string::_isalpha_l as *const () as usize,
            ucrt::string::isupper as *const () as usize,
            ucrt::string::_isupper_l as *const () as usize,
            ucrt::string::islower as *const () as usize,
            ucrt::string::_islower_l as *const () as usize,
            ucrt::string::isdigit as *const () as usize,
            ucrt::string::_isdigit_l as *const () as usize,
            ucrt::string::isxdigit as *const () as usize,
            ucrt::string::_isxdigit_l as *const () as usize,
            ucrt::string::isspace as *const () as usize,
            ucrt::string::_isspace_l as *const () as usize,
            ucrt::string::ispunct as *const () as usize,
            ucrt::string::_ispunct_l as *const () as usize,
            ucrt::string::isalnum as *const () as usize,
            ucrt::string::_isalnum_l as *const () as usize,
            ucrt::string::isprint as *const () as usize,
            ucrt::string::_isprint_l as *const () as usize,
            ucrt::string::isgraph as *const () as usize,
            ucrt::string::_isgraph_l as *const () as usize,
            ucrt::string::iscntrl as *const () as usize,
            ucrt::string::_iscntrl_l as *const () as usize,
            ucrt::string::isblank as *const () as usize,
            ucrt::string::__isascii as *const () as usize,
            ucrt::string::__toascii as *const () as usize,
            ucrt::string::__iscsym as *const () as usize,
            ucrt::string::__iscsymf as *const () as usize,
            ucrt::string::_isctype as *const () as usize,
            ucrt::string::_isctype_l as *const () as usize,
            ucrt::string::iswalpha as *const () as usize,
            ucrt::string::iswupper as *const () as usize,
            ucrt::string::iswlower as *const () as usize,
            ucrt::string::iswdigit as *const () as usize,
            ucrt::string::iswxdigit as *const () as usize,
            ucrt::string::iswspace as *const () as usize,
            ucrt::string::iswpunct as *const () as usize,
            ucrt::string::iswalnum as *const () as usize,
            ucrt::string::iswprint as *const () as usize,
            ucrt::string::iswgraph as *const () as usize,
            ucrt::string::iswcntrl as *const () as usize,
            ucrt::string::iswblank as *const () as usize,
            ucrt::string::iswctype as *const () as usize,
            ucrt::string::iswascii as *const () as usize,
            ucrt::string::_iswctype_l as *const () as usize,
            ucrt::string::strlen as *const () as usize,
            ucrt::string::strnlen as *const () as usize,
            ucrt::string::strcmp as *const () as usize,
            ucrt::string::strncmp as *const () as usize,
            ucrt::string::_stricmp as *const () as usize,
            ucrt::string::_strnicmp as *const () as usize,
            ucrt::string::_stricmp_l as *const () as usize,
            ucrt::string::_strnicmp_l as *const () as usize,
            ucrt::string::_strcmpi as *const () as usize,
            ucrt::string::strcoll as *const () as usize,
            ucrt::string::_stricoll as *const () as usize,
            ucrt::string::_strncoll as *const () as usize,
            ucrt::string::_strnicoll as *const () as usize,
            ucrt::string::strxfrm as *const () as usize,
            ucrt::string::strcpy as *const () as usize,
            ucrt::string::strncpy as *const () as usize,
            ucrt::string::strcat as *const () as usize,
            ucrt::string::strncat as *const () as usize,
            ucrt::string::strcpy_s as *const () as usize,
            ucrt::string::strncpy_s as *const () as usize,
            ucrt::string::strcat_s as *const () as usize,
            ucrt::string::strncat_s as *const () as usize,
            ucrt::string::_strdup as *const () as usize,
            ucrt::string::strspn as *const () as usize,
            ucrt::string::strcspn as *const () as usize,
            ucrt::string::strpbrk as *const () as usize,
            ucrt::string::strtok_s as *const () as usize,
            ucrt::string::strtok as *const () as usize,
            ucrt::string::__strncnt as *const () as usize,
            ucrt::string::_strlwr as *const () as usize,
            ucrt::string::_strupr as *const () as usize,
            ucrt::string::_strlwr_s as *const () as usize,
            ucrt::string::_strupr_s as *const () as usize,
            ucrt::string::_strrev as *const () as usize,
            ucrt::string::_strset as *const () as usize,
            ucrt::string::_strnset as *const () as usize,
            ucrt::string::memcpy_s as *const () as usize,
            ucrt::string::memmove_s as *const () as usize,
            ucrt::string::wmemcpy_s as *const () as usize,
            ucrt::string::wmemmove_s as *const () as usize,
            ucrt::string::_memicmp as *const () as usize,
            ucrt::string::_memccpy as *const () as usize,
            ucrt::string::wcslen as *const () as usize,
            ucrt::string::wcsnlen as *const () as usize,
            ucrt::string::wcscmp as *const () as usize,
            ucrt::string::wcsncmp as *const () as usize,
            ucrt::string::_wcsicmp as *const () as usize,
            ucrt::string::_wcsnicmp as *const () as usize,
            ucrt::string::_wcsicmp_l as *const () as usize,
            ucrt::string::_wcsnicmp_l as *const () as usize,
            ucrt::string::wcscoll as *const () as usize,
            ucrt::string::_wcsicoll as *const () as usize,
            ucrt::string::_wcsncoll as *const () as usize,
            ucrt::string::_wcsnicoll as *const () as usize,
            ucrt::string::wcsxfrm as *const () as usize,
            ucrt::string::wcscpy as *const () as usize,
            ucrt::string::wcsncpy as *const () as usize,
            ucrt::string::wcscat as *const () as usize,
            ucrt::string::wcsncat as *const () as usize,
            ucrt::string::wcscpy_s as *const () as usize,
            ucrt::string::wcsncpy_s as *const () as usize,
            ucrt::string::wcscat_s as *const () as usize,
            ucrt::string::wcsncat_s as *const () as usize,
            ucrt::string::_wcsdup as *const () as usize,
            ucrt::string::wcsspn as *const () as usize,
            ucrt::string::wcscspn as *const () as usize,
            ucrt::string::wcspbrk as *const () as usize,
            ucrt::string::wcstok_s as *const () as usize,
            ucrt::string::wcstok as *const () as usize,
            ucrt::string::_wcstok as *const () as usize,
            ucrt::string::_wcslwr as *const () as usize,
            ucrt::string::_wcsupr as *const () as usize,
            ucrt::string::_wcslwr_s as *const () as usize,
            ucrt::string::_wcsupr_s as *const () as usize,
            ucrt::string::_wcsrev as *const () as usize,
            ucrt::string::_wcsset as *const () as usize,
            ucrt::string::_wcsnset as *const () as usize,
            ucrt::string::wmemset as *const () as usize,
            // Pointer-taking float classification helpers (1 pointer arg, i16 return, no XMM args)
            ucrt::math::_dtest as *const () as usize,
            ucrt::math::_ldtest as *const () as usize,
            ucrt::math::_fdtest as *const () as usize,
        ];
        v.extend_from_slice(d3d12::plain_com_fns());
        v.into_boxed_slice()
    })
}

/// Pure float math builtins (float args in `xmm0..2`, float or integer result, no guest
/// callbacks) eligible for inline `plain_call` with XMM arg/ret marshalling.
pub fn pure_float_math_fns() -> &'static [usize] {
    static FNS: OnceLock<Box<[usize]>> = OnceLock::new();
    FNS.get_or_init(|| {
        use ucrt::math;
        vec![
            math::acos as *const () as usize,
            math::asin as *const () as usize,
            math::atan as *const () as usize,
            math::cos as *const () as usize,
            math::sin as *const () as usize,
            math::tan as *const () as usize,
            math::cosh as *const () as usize,
            math::sinh as *const () as usize,
            math::tanh as *const () as usize,
            math::acosh as *const () as usize,
            math::asinh as *const () as usize,
            math::atanh as *const () as usize,
            math::exp as *const () as usize,
            math::exp2 as *const () as usize,
            math::expm1 as *const () as usize,
            math::log as *const () as usize,
            math::log10 as *const () as usize,
            math::log2 as *const () as usize,
            math::log1p as *const () as usize,
            math::sqrt as *const () as usize,
            math::cbrt as *const () as usize,
            math::ceil as *const () as usize,
            math::floor as *const () as usize,
            math::round as *const () as usize,
            math::trunc as *const () as usize,
            math::fabs as *const () as usize,
            math::atan2 as *const () as usize,
            math::pow as *const () as usize,
            math::fmod as *const () as usize,
            math::copysign as *const () as usize,
            math::hypot as *const () as usize,
            math::fmax as *const () as usize,
            math::fmin as *const () as usize,
            math::fdim as *const () as usize,
            math::_hypot as *const () as usize,
            math::_copysign as *const () as usize,
            math::fma as *const () as usize,
            math::frexp as *const () as usize,
            math::ldexp as *const () as usize,
            math::scalbn as *const () as usize,
            math::_scalb as *const () as usize,
            math::modf as *const () as usize,
            math::nearbyint as *const () as usize,
            math::rint as *const () as usize,
            math::lrint as *const () as usize,
            math::llrint as *const () as usize,
            math::lround as *const () as usize,
            math::llround as *const () as usize,
            math::ilogb as *const () as usize,
            math::logb as *const () as usize,
            math::_logb as *const () as usize,
            math::remainder as *const () as usize,
            math::erf as *const () as usize,
            math::erfc as *const () as usize,
            math::tgamma as *const () as usize,
            math::lgamma as *const () as usize,
            math::nan as *const () as usize,
            math::_dclass as *const () as usize,
            math::_ldclass as *const () as usize,
            math::_dsign as *const () as usize,
            math::_ldsign as *const () as usize,
            math::_dpcomp as *const () as usize,
            math::_ldpcomp as *const () as usize,
            math::_finite as *const () as usize,
            math::_isnan as *const () as usize,
            math::isnan as *const () as usize,
            math::_fpclass as *const () as usize,
            math::_chgsign as *const () as usize,
            math::acosf as *const () as usize,
            math::asinf as *const () as usize,
            math::atanf as *const () as usize,
            math::cosf as *const () as usize,
            math::sinf as *const () as usize,
            math::tanf as *const () as usize,
            math::coshf as *const () as usize,
            math::sinhf as *const () as usize,
            math::tanhf as *const () as usize,
            math::expf as *const () as usize,
            math::exp2f as *const () as usize,
            math::expm1f as *const () as usize,
            math::logf as *const () as usize,
            math::log10f as *const () as usize,
            math::log2f as *const () as usize,
            math::log1pf as *const () as usize,
            math::sqrtf as *const () as usize,
            math::cbrtf as *const () as usize,
            math::ceilf as *const () as usize,
            math::floorf as *const () as usize,
            math::roundf as *const () as usize,
            math::truncf as *const () as usize,
            math::fabsf as *const () as usize,
            math::atan2f as *const () as usize,
            math::powf as *const () as usize,
            math::fmodf as *const () as usize,
            math::copysignf as *const () as usize,
            math::hypotf as *const () as usize,
            math::fmaxf as *const () as usize,
            math::fminf as *const () as usize,
            math::_hypotf as *const () as usize,
            math::_copysignf as *const () as usize,
            math::fmaf as *const () as usize,
            math::frexpf as *const () as usize,
            math::ldexpf as *const () as usize,
            math::scalbnf as *const () as usize,
            math::modff as *const () as usize,
            math::nearbyintf as *const () as usize,
            math::rintf as *const () as usize,
            math::lrintf as *const () as usize,
            math::llrintf as *const () as usize,
            math::lroundf as *const () as usize,
            math::llroundf as *const () as usize,
            math::ilogbf as *const () as usize,
            math::remainderf as *const () as usize,
            math::nanf as *const () as usize,
            math::_fdclass as *const () as usize,
            math::_fdsign as *const () as usize,
            math::_fdpcomp as *const () as usize,
            math::_finitef as *const () as usize,
            math::_isnanf as *const () as usize,
            math::_fpclassf as *const () as usize,
        ]
        .into_boxed_slice()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apiset_normalization() {
        assert_eq!(canonical_dll("api-ms-win-crt-runtime-l1-1-0.dll"), "ucrtbase.dll");
        assert_eq!(canonical_dll("API-MS-WIN-CORE-SYNCH-L1-2-0.DLL"), "kernel32.dll");
        assert_eq!(canonical_dll("KERNEL32.dll"), "kernel32.dll");
        assert_eq!(canonical_dll("KernelBase"), "kernel32.dll");
    }

    #[test]
    fn lookup_case_insensitive_dll() {
        assert!(lookup("KERNEL32.DLL", "GetLastError").is_some());
        assert!(lookup("api-ms-win-crt-heap-l1-1-0.dll", "malloc").is_some());
        assert!(lookup("kernel32.dll", "DefinitelyNotAFunction").is_none());
        // Names are case-sensitive, like GetProcAddress.
        assert!(lookup("kernel32.dll", "getlasterror").is_none());
    }

    #[test]
    fn vcruntime_exports_and_primitives() {
        assert!(lookup("VCRUNTIME140_1.dll", "__CxxFrameHandler4").is_some());
        for sym in [
            "memcpy",
            "memmove",
            "memset",
            "memcmp",
            "memchr",
            "strchr",
            "strstr",
            "wcsrchr",
            "wcsstr",
            "__std_type_info_compare",
            "__std_type_info_name",
            "__std_type_info_destroy_list",
            "__RTDynamicCast",
            "__AdjustPointer",
            "__current_exception",
            "__current_exception_context",
            "_CxxThrowException",
            "__C_specific_handler",
            "__CxxFrameHandler3",
            "__std_exception_copy",
            "__std_exception_destroy",
            "__std_terminate",
            "_purecall",
            "__uncaught_exception",
            "__uncaught_exceptions",
        ] {
            assert!(
                lookup("VCRUNTIME140.dll", sym).is_some(),
                "missing export {sym} in vcruntime140.dll"
            );
        }

        unsafe {
            // Test wcsrchr and wcsstr.
            let s: [u16; 6] = [b'a' as u16, b'b' as u16, b'c' as u16, b'b' as u16, b'd' as u16, 0];
            let needle: [u16; 3] = [b'b' as u16, b'd' as u16, 0];
            let rchr = misc::vcruntime140::wcsrchr(s.as_ptr(), b'b' as u16);
            assert_eq!(rchr, s.as_ptr().add(3) as *mut u16);
            let sstr = misc::vcruntime140::wcsstr(s.as_ptr(), needle.as_ptr());
            assert_eq!(sstr, s.as_ptr().add(3) as *mut u16);

            // Test __std_exception_copy and __std_exception_destroy.
            let msg = c"test error";
            let src = misc::vcruntime140::StdExceptionData {
                what: msg.as_ptr(),
                do_free: 1,
            };
            let mut dst = misc::vcruntime140::StdExceptionData {
                what: std::ptr::null(),
                do_free: 0,
            };
            misc::vcruntime140::__std_exception_copy(&src, &mut dst);
            assert!(!dst.what.is_null());
            assert_ne!(dst.what, src.what);
            assert_eq!(dst.do_free, 1);
            assert_eq!(std::ffi::CStr::from_ptr(dst.what), msg);
            misc::vcruntime140::__std_exception_destroy(&mut dst);
            assert!(dst.what.is_null());
            assert_eq!(dst.do_free, 0);
        }
    }

    #[test]
    fn msvcp140_exports_and_primitives() {
        for sym in [
            "_Mtx_init_in_situ",
            "_Mtx_lock",
            "_Mtx_unlock",
            "_Cnd_init_in_situ",
            "_Cnd_signal",
            "_Cnd_broadcast",
            "_Query_perf_counter",
            "_Query_perf_frequency",
            "_Xtime_get_ticks",
            "?_Syserror_map@std@@YAPEBDH@Z",
            "?_Winerror_map@std@@YAHH@Z",
            "?cout@std@@3V?$basic_ostream@DU?$char_traits@D@std@@@1@A",
            "?cerr@std@@3V?$basic_ostream@DU?$char_traits@D@std@@@1@A",
            "??0?$basic_streambuf@DU?$char_traits@D@std@@@std@@IEAA@XZ",
            "??1?$basic_streambuf@DU?$char_traits@D@std@@@std@@UEAA@XZ",
        ] {
            assert!(
                lookup("MSVCP140.dll", sym).is_some(),
                "missing export {sym} in msvcp140.dll"
            );
        }

        unsafe {
            let mut mtx: misc::msvcp140::MtxInternal = std::mem::zeroed();
            misc::msvcp140::_Mtx_init_in_situ(&mut mtx, 0x101); // recursive
            assert_eq!(misc::msvcp140::_Mtx_current_owns(&mtx), 0);
            assert_eq!(misc::msvcp140::_Mtx_lock(&mut mtx), 0);
            assert_eq!(misc::msvcp140::_Mtx_current_owns(&mtx), 1);
            assert_eq!(misc::msvcp140::_Mtx_trylock(&mut mtx), 0);
            assert_eq!(mtx.count, 2);
            assert_eq!(misc::msvcp140::_Mtx_unlock(&mut mtx), 0);
            assert_eq!(misc::msvcp140::_Mtx_unlock(&mut mtx), 0);
            assert_eq!(misc::msvcp140::_Mtx_current_owns(&mtx), 0);
            misc::msvcp140::_Mtx_destroy_in_situ(&mut mtx);

            assert!(_Query_perf_Check());
        }
    }

    unsafe fn _Query_perf_Check() -> bool {
        let f = misc::msvcp140::_Query_perf_frequency();
        let c = misc::msvcp140::_Query_perf_counter();
        let t = misc::msvcp140::_Xtime_get_ticks();
        f > 0 && c > 0 && t > 0
    }

    #[test]
    fn dotnet_and_mono_runtime_bridges() {
        assert!(lookup("mscoree.dll", "_CorExeMain").is_some());
        assert!(lookup("mscoree.dll", "_CorDllMain").is_some());
        assert!(lookup("mscoree.dll", "CLRCreateInstance").is_some());
        assert!(lookup("mscoree.dll", "CorBindToRuntimeEx").is_some());
        assert!(lookup("hostfxr.dll", "hostfxr_initialize_for_runtime_config").is_some());
        assert!(lookup("hostfxr.dll", "hostfxr_get_runtime_delegate").is_some());
        assert!(lookup("coreclr.dll", "coreclr_initialize").is_some());
        assert!(lookup("coreclr.dll", "coreclr_create_delegate").is_some());
        assert!(lookup("mono-2.0-bdwgc.dll", "mono_jit_init").is_some());
        assert!(lookup("mono.dll", "mono_domain_assembly_open").is_some());
        assert!(lookup("mono-2.0-bdwgc.dll", "mono_gchandle_new").is_some());

        unsafe {
            // Both native host CoreCLR and our embedded CLR provide real ICLRMetaHost.
            let mut metahost: *mut std::ffi::c_void = std::ptr::null_mut();
            let clr_hr =
                misc::mscoree::CLRCreateInstance(std::ptr::null(), std::ptr::null(), &mut metahost);
            assert_eq!(clr_hr, 0);
            assert!(!metahost.is_null());

            // Nonexistent runtimeconfig.json must still fail honestly.
            let cfg = strings::to_wide_z("C:\\App\\NonExistentGame.runtimeconfig.json");
            let mut ctx: *mut std::ffi::c_void = 0xdeadusize as *mut std::ffi::c_void;
            let hfx_rc = misc::hostfxr::hostfxr_initialize_for_runtime_config(
                cfg.as_ptr(),
                std::ptr::null(),
                &mut ctx,
            );
            assert_ne!(hfx_rc, 0);
            assert!(ctx.is_null());

            // Mono works natively (via host libmono or embedded CLR) with real UTF-16/UTF-8 strings.
            let dom = misc::mono_2_0_bdwgc::mono_jit_init(b"TestDomain\0".as_ptr());
            assert!(!dom.is_null());
            let s = misc::mono_2_0_bdwgc::mono_string_new(dom, b"HelloMono\0".as_ptr());
            assert!(!s.is_null());
            let utf8 = misc::mono_2_0_bdwgc::mono_string_to_utf8(s);
            assert!(!utf8.is_null());
            assert_eq!(std::ffi::CStr::from_ptr(utf8), c"HelloMono");
            misc::mono_2_0_bdwgc::mono_free(utf8 as *mut std::ffi::c_void);
        }
    }

    #[test]
    fn plain_and_pure_math_tables_cover_crt_exports() {
        let plain: std::collections::HashSet<usize> = plain_builtin_fns().iter().copied().collect();
        let math: std::collections::HashSet<usize> = pure_float_math_fns().iter().copied().collect();

        for (dll, sym) in [
            ("ucrtbase.dll", "strcpy_s"),
            ("ucrtbase.dll", "strncpy_s"),
            ("ucrtbase.dll", "strcat_s"),
            ("ucrtbase.dll", "strncat_s"),
            ("ucrtbase.dll", "memcpy_s"),
            ("ucrtbase.dll", "memmove_s"),
            ("ucrtbase.dll", "wcscpy_s"),
            ("ucrtbase.dll", "wcsncpy_s"),
            ("ucrtbase.dll", "wcscat_s"),
            ("ucrtbase.dll", "wcsncat_s"),
            ("ucrtbase.dll", "_wcsicmp"),
            ("ucrtbase.dll", "_wcsnicmp"),
            ("msvcrt.dll", "strlen"),
            ("msvcrt.dll", "strcmp"),
            ("msvcrt.dll", "strncmp"),
            ("msvcrt.dll", "strcpy"),
            ("msvcrt.dll", "strncpy"),
            ("msvcrt.dll", "wcslen"),
            ("msvcrt.dll", "memcpy"),
            ("msvcrt.dll", "memmove"),
            ("msvcrt.dll", "memset"),
            ("msvcrt.dll", "memcmp"),
            ("msvcrt.dll", "memchr"),
            ("kernel32.dll", "WaitOnAddress"),
            ("kernel32.dll", "WakeByAddressSingle"),
            ("kernel32.dll", "WakeByAddressAll"),
            ("kernel32.dll", "SleepConditionVariableCS"),
            ("kernel32.dll", "SleepConditionVariableSRW"),
        ] {
            let p = lookup(dll, sym).unwrap_or_else(|| panic!("missing {dll}!{sym}")) as usize;
            assert!(plain.contains(&p), "{dll}!{sym} missing from plain_builtin_fns");
        }

        for (dll, sym) in [
            ("ucrtbase.dll", "sinf"),
            ("ucrtbase.dll", "cosf"),
            ("ucrtbase.dll", "tanf"),
            ("ucrtbase.dll", "sqrtf"),
            ("ucrtbase.dll", "log2f"),
            ("ucrtbase.dll", "acosf"),
            ("ucrtbase.dll", "ceilf"),
            ("ucrtbase.dll", "floorf"),
            ("ucrtbase.dll", "powf"),
            ("ucrtbase.dll", "fmodf"),
            ("ucrtbase.dll", "fmaxf"),
            ("ucrtbase.dll", "fminf"),
            ("ucrtbase.dll", "_isnan"),
            ("ucrtbase.dll", "_isnanf"),
            ("ucrtbase.dll", "ldexp"),
            ("ucrtbase.dll", "ldexpf"),
            ("ucrtbase.dll", "lrintf"),
            ("ucrtbase.dll", "lroundf"),
            ("ucrtbase.dll", "_finite"),
            ("ucrtbase.dll", "_finitef"),
            ("ucrtbase.dll", "_dclass"),
            ("ucrtbase.dll", "_fdclass"),
            ("msvcrt.dll", "sinf"),
            ("msvcrt.dll", "cosf"),
            ("msvcrt.dll", "sqrtf"),
            ("msvcrt.dll", "powf"),
            ("msvcrt.dll", "ldexp"),
        ] {
            let p = lookup(dll, sym).unwrap_or_else(|| panic!("missing {dll}!{sym}")) as usize;
            assert!(math.contains(&p), "{dll}!{sym} missing from pure_float_math_fns");
        }

        for p in &math {
            assert!(!plain.contains(p), "function {p:#x} present in both plain and pure_float_math tables");
        }
    }

    pub(crate) struct ScopedGuestDispatcherGuard {
        prev: usize,
    }

    impl Drop for ScopedGuestDispatcherGuard {
        fn drop(&mut self) {
            GUEST_DISPATCHER.store(self.prev, Ordering::Release);
        }
    }

    /// Test-only harness to install a scoped guest callback dispatcher and restore the prior
    /// value on drop. Never available in production builds.
    pub(crate) fn install_test_guest_dispatcher(
        f: Option<GuestDispatcherFn>,
    ) -> ScopedGuestDispatcherGuard {
        let next = f.map_or(0, |func| func as usize);
        let prev = GUEST_DISPATCHER.swap(next, Ordering::AcqRel);
        ScopedGuestDispatcherGuard { prev }
    }

    pub(crate) const TEST_WSA_EXT_THUNK_BASE: usize = 0x7FFF_E000;
    pub(crate) const TEST_WSA_EXT_THUNK_STRIDE: usize = 16;
    static TEST_WSA_EXT_ENABLED: std::sync::atomic::AtomicBool =
        std::sync::atomic::AtomicBool::new(false);

    pub(crate) fn test_wsa_extension_thunk(ext: WsaExtension) -> Option<usize> {
        if !TEST_WSA_EXT_ENABLED.load(Ordering::Acquire) {
            return None;
        }
        Some(TEST_WSA_EXT_THUNK_BASE + (ext as usize) * TEST_WSA_EXT_THUNK_STRIDE)
    }

    pub(crate) struct ScopedTestWsaExtGuard {
        prev: bool,
    }

    impl Drop for ScopedTestWsaExtGuard {
        fn drop(&mut self) {
            TEST_WSA_EXT_ENABLED.store(self.prev, Ordering::Release);
        }
    }

    pub(crate) fn enable_test_wsa_ext_thunks() -> ScopedTestWsaExtGuard {
        let prev = TEST_WSA_EXT_ENABLED.swap(true, Ordering::AcqRel);
        ScopedTestWsaExtGuard { prev }
    }
}
