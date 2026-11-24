//! Emits the standalone AOT runtime prelude, Import Address Table (IAT)
//! bindings, and function/export/section lookup directories.

use super::{AotCallbackRoot, AotFunctionMeta, AotImportBinding};
use anyhow::Result;
use std::collections::BTreeMap;
use std::fmt::Write as _;

pub(super) fn emit_aot_runtime_prelude(
    out: &mut String,
    image_base: u64,
    size_of_image: u32,
) -> Result<()> {
    writeln!(
        out,
        "pub const PREFERRED_IMAGE_BASE: u64 = 0x{image_base:016X};"
    )?;
    writeln!(out, "pub const SIZE_OF_IMAGE: u32 = 0x{size_of_image:08X};")?;
    writeln!(
        out,
        r#"pub static IMAGE_BASE: core::sync::atomic::AtomicU64 =
    core::sync::atomic::AtomicU64::new(PREFERRED_IMAGE_BASE);

#[inline(always)]
pub fn image_base() -> u64 {{
    IMAGE_BASE.load(core::sync::atomic::Ordering::Relaxed)
}}

#[derive(Clone, Copy)]
pub struct AotFunctionEntry {{
    pub begin_rva: u32,
    pub end_rva: u32,
    pub name: &'static str,
    pub func: extern "C" fn(u64, u64, u64, u64) -> u64,
    /// `true` when every instruction was lowered with exact semantics
    /// (no `[AOT_FALLBACK]`/`[AOT_MAPPED]`/approximated instruction). Does NOT
    /// account for the lossy 4-GPR ABI / private stack frame.
    pub fully_lowered: bool,
}}

#[derive(Clone, Copy)]
pub struct AotExportEntry {{
    pub name: &'static str,
    pub rva: u32,
    pub func: extern "C" fn(u64, u64, u64, u64) -> u64,
}}

#[derive(Clone, Copy)]
pub struct AotImportEntry {{
    pub iat_rva: u32,
    pub dll: &'static str,
    pub symbol: &'static str,
    pub rust_path: &'static str,
}}

#[derive(Clone, Copy)]
pub struct AotSectionEntry {{
    pub name: &'static str,
    pub rva: u32,
    pub virtual_size: u32,
    pub characteristics: u32,
}}

#[derive(Clone, Copy)]
pub struct AotCallbackRootEntry {{
    pub rva: u32,
    pub kind: &'static str,
    pub is_eh_continuation: bool,
}}

#[inline(always)]
unsafe fn rd128(addr: u64) -> u128 {{ (addr as *const u128).read_unaligned() }}
#[inline(always)]
unsafe fn wr128(addr: u64, v: u128) {{ (addr as *mut u128).write_unaligned(v); }}

#[inline(always)]
unsafe fn call_indirect(target: u64, rcx: u64, rdx: u64, r8: u64, r9: u64) -> u64 {{
    let base = image_base();
    if target >= base && target < base + SIZE_OF_IMAGE as u64 {{
        let rva = (target - base) as u32;
        if let Some(f) = lookup_aot_function(rva) {{
            return f(rcx, rdx, r8, r9);
        }}
    }}
    let f: extern "C" fn(u64, u64, u64, u64) -> u64 = core::mem::transmute(target as usize);
    f(rcx, rdx, r8, r9)
}}

/// Invoke a resolved Win64 integer/pointer IAT target with `arity` (0..=12) arguments,
/// reading stack arguments `5..=arity` from `[rsp + stack_base + 8*(i - 4)]` (e.g.
/// `stack_base = 0x20` for `call` where `ReadFileEx`'s 5th pointer `lpCompletionRoutine`
/// lives at `[rsp + 0x20]`, or `0x28` for a tail `jmp`).
#[inline(always)]
unsafe fn call_indirect_args(
    target: u64,
    rcx: u64,
    rdx: u64,
    r8: u64,
    r9: u64,
    rsp: u64,
    stack_base: u64,
    arity: u8,
) -> u64 {{
    let base = image_base();
    if target >= base && target < base + SIZE_OF_IMAGE as u64 {{
        let rva = (target - base) as u32;
        if let Some(f) = lookup_aot_function(rva) {{
            return f(rcx, rdx, r8, r9);
        }}
    }}
    if arity <= 4 || rsp == 0 {{
        let f: extern "C" fn(u64, u64, u64, u64) -> u64 = core::mem::transmute(target as usize);
        return f(rcx, rdx, r8, r9);
    }}
    let a5 = rd64(rsp.wrapping_add(stack_base));
    if arity == 5 {{
        let f: extern "C" fn(u64, u64, u64, u64, u64) -> u64 = core::mem::transmute(target as usize);
        return f(rcx, rdx, r8, r9, a5);
    }}
    let a6 = rd64(rsp.wrapping_add(stack_base + 8));
    if arity == 6 {{
        let f: extern "C" fn(u64, u64, u64, u64, u64, u64) -> u64 = core::mem::transmute(target as usize);
        return f(rcx, rdx, r8, r9, a5, a6);
    }}
    let a7 = rd64(rsp.wrapping_add(stack_base + 16));
    if arity == 7 {{
        let f: extern "C" fn(u64, u64, u64, u64, u64, u64, u64) -> u64 = core::mem::transmute(target as usize);
        return f(rcx, rdx, r8, r9, a5, a6, a7);
    }}
    let a8 = rd64(rsp.wrapping_add(stack_base + 24));
    if arity == 8 {{
        let f: extern "C" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 = core::mem::transmute(target as usize);
        return f(rcx, rdx, r8, r9, a5, a6, a7, a8);
    }}
    let a9 = rd64(rsp.wrapping_add(stack_base + 32));
    let a10 = if arity >= 10 {{ rd64(rsp.wrapping_add(stack_base + 40)) }} else {{ 0 }};
    if arity <= 10 {{
        let f: extern "C" fn(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) -> u64 = core::mem::transmute(target as usize);
        return f(rcx, rdx, r8, r9, a5, a6, a7, a8, a9, a10);
    }}
    let a11 = rd64(rsp.wrapping_add(stack_base + 48));
    let a12 = if arity >= 12 {{ rd64(rsp.wrapping_add(stack_base + 56)) }} else {{ 0 }};
    let f: extern "C" fn(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =
        core::mem::transmute(target as usize);
    f(rcx, rdx, r8, r9, a5, a6, a7, a8, a9, a10, a11, a12)
}}

static SYSCALL_HOOK: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);

static HOST_TSC_READER: std::sync::OnceLock<extern "C" fn(bool) -> u64> = std::sync::OnceLock::new();

/// Register a host Windows NT syscall dispatcher (`fn(ssn, r10, rdx, r8, r9, rsp) -> i32`).
pub fn set_syscall_hook(hook: unsafe extern "C" fn(u32, u64, u64, u64, u64, u64) -> i32) {{
    SYSCALL_HOOK.store(hook as usize, core::sync::atomic::Ordering::Release);
}}

#[inline(always)]
fn aot_read_tsc(ordered: bool) -> u64 {{
    if let Some(read) = HOST_TSC_READER.get() {{
        return read(ordered);
    }}
    static FROZEN: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    let frozen = *FROZEN.get_or_init(|| std::env::var("AW2_EMU_VERIFY").is_ok_and(|s| s.trim() == "1"));
    aot_tsc::read_tsc(ordered, frozen)
}}

#[inline(always)]
fn aot_cpuid(leaf: u32, subleaf: u32) -> (u64, u64, u64, u64) {{
    let _ = subleaf;
    match leaf {{
        0 => (7, 0x756e_6547, 0x6c65_746e, 0x4965_6e69), // GenuineIntel
        1 => (0x0009_06ea, 0x0010_0800, 0x7ed8_3223, 0x178b_fbff), // ECX bit 31 (Hypervisor) = 0
        7 => (0, 0x200c_07ab, 0, 0),
        0x8000_0000 => (0x8000_0008, 0, 0, 0),
        0x8000_0001 => (0, 0, 0x0000_0161, 0x2810_0800),
        _ => (0, 0, 0, 0),
    }}
}}

#[inline]
unsafe fn aot_nt_syscall(ssn: u32, r10: u64, rdx: u64, r8: u64, r9: u64, rsp: u64) -> u64 {{
    let hook = SYSCALL_HOOK.load(core::sync::atomic::Ordering::Acquire);
    if hook != 0 {{
        let f: unsafe extern "C" fn(u32, u64, u64, u64, u64, u64) -> i32 = core::mem::transmute(hook);
        return f(ssn, r10, rdx, r8, r9, rsp) as i64 as u64;
    }}
    match ssn {{
        0x0019 => {{
            // NtQueryInformationProcess
            let class = rdx as u32;
            let out = r8;
            let len = r9 as u32;
            match class {{
                7 if len >= 8 && out != 0 => {{ wr64(out, 0); 0 }} // ProcessDebugPort = 0
                0x1E => {{ if len >= 8 && out != 0 {{ wr64(out, 0); }} 0xC000_0353u32 as i32 as i64 as u64 }} // STATUS_PORT_NOT_SET
                0x1F if len >= 4 && out != 0 => {{ wr32(out, 1); 0 }} // ProcessDebugFlags = 1
                _ => 0,
            }}
        }}
        0x000D => {{
            // NtSetInformationThread (0x11 = ThreadHideFromDebugger)
            if rdx as u32 == 0x11 && (r8 != 0 || r9 != 0) {{
                0xC000_0004u32 as i32 as i64 as u64 // STATUS_INFO_LENGTH_MISMATCH
            }} else {{
                0
            }}
        }}
        0x0036 => {{
            // NtQuerySystemInformation
            let class = r10 as u32;
            let out = rdx;
            let len = r8 as u32;
            match class {{
                0x23 if len >= 2 && out != 0 => {{ wr8(out, 0); wr8(out + 1, 1); 0 }} // KdDebuggerNotPresent = 1
                0x67 if len >= 8 && out != 0 => {{ wr32(out, 8); wr32(out + 4, 1); 0 }} // CODEINTEGRITY_OPTION_ENABLED
                _ => 0,
            }}
        }}
        _ => 0,
    }}
}}
"#
    )?;
    out.push_str(AOT_TSO_ATOMICS_PRELUDE);
    out.push_str("\nmod aot_tsc {\n");
    out.push_str(include_str!("../../../x86-runtime/src/emu/tsc.rs"));
    out.push_str("\n}\n");
    Ok(())
}

pub(super) fn emit_import_table(
    out: &mut String,
    imports_by_rva: &BTreeMap<u32, AotImportBinding>,
) -> Result<()> {
    writeln!(out, "pub static AOT_IMPORTS: &[AotImportEntry] = &[")?;
    for imp in imports_by_rva.values() {
        let rp = imp.rust_path.as_deref().unwrap_or("");
        writeln!(
            out,
            "    AotImportEntry {{ iat_rva: 0x{:08X}, dll: {:?}, symbol: {:?}, rust_path: {:?} }},",
            imp.iat_rva, imp.canonical_dll, imp.symbol, rp
        )?;
    }
    writeln!(out, "];\n")?;

    writeln!(
        out,
        "#[inline(always)]\nunsafe fn call_iat(iat_rva: u32, rcx: u64, rdx: u64, r8: u64, r9: u64) -> u64 {{"
    )?;
    writeln!(
        out,
        "    call_iat_args(iat_rva, rcx, rdx, r8, r9, 0, 0, 4)"
    )?;
    writeln!(out, "}}\n")?;

    writeln!(
        out,
        "#[inline(always)]\nunsafe fn call_iat_args(iat_rva: u32, rcx: u64, rdx: u64, r8: u64, r9: u64, rsp: u64, stack_base: u64, arity: u8) -> u64 {{"
    )?;
    writeln!(
        out,
        "    let slot_va = image_base().wrapping_add(iat_rva as u64);"
    )?;
    writeln!(out, "    let target = rd64(slot_va);")?;
    writeln!(
        out,
        "    call_indirect_args(target, rcx, rdx, r8, r9, rsp, stack_base, arity)"
    )?;
    writeln!(out, "}}\n")?;
    Ok(())
}

pub(super) fn emit_function_directory(
    out: &mut String,
    functions: &[AotFunctionMeta],
    sections: &[pe_image::Section],
    callback_roots: &[AotCallbackRoot],
    eh_continuations: &[u32],
    uses_rdtscp: bool,
) -> Result<()> {
    writeln!(out, "pub fn set_image_base(base: u64) {{")?;
    writeln!(out, "    IMAGE_BASE.store(base, core::sync::atomic::Ordering::Relaxed);")?;
    if uses_rdtscp {
        // Probe during module setup so the first timed guest region also uses the fast path.
        writeln!(out, r#"    #[cfg(target_arch = "aarch64")]
    if HOST_TSC_READER.get().is_none() {{ let _ = aot_tsc::has_ecv(); }}"#)?;
    }
    writeln!(out, "}}\n")?;
    writeln!(out, "pub static AOT_FUNCTIONS: &[AotFunctionEntry] = &[")?;
    for f in functions {
        writeln!(
            out,
            "    AotFunctionEntry {{ begin_rva: 0x{:08X}, end_rva: 0x{:08X}, name: {:?}, func: {}, fully_lowered: {} }},",
            f.begin_rva, f.end_rva, f.rust_ident, f.rust_ident, f.fallback_instructions == 0
        )?;
    }
    writeln!(out, "];\n")?;

    writeln!(out, "pub static AOT_EXPORTS: &[AotExportEntry] = &[")?;
    for f in functions {
        for exp in &f.export_names {
            writeln!(
                out,
                "    AotExportEntry {{ name: {:?}, rva: 0x{:08X}, func: {} }},",
                exp, f.begin_rva, f.rust_ident
            )?;
        }
    }
    writeln!(out, "];\n")?;

    writeln!(out, "pub static AOT_SECTIONS: &[AotSectionEntry] = &[")?;
    for s in sections {
        writeln!(
            out,
            "    AotSectionEntry {{ name: {:?}, rva: 0x{:08X}, virtual_size: 0x{:08X}, characteristics: 0x{:08X} }},",
            s.name, s.rva, s.mem_size(), s.characteristics
        )?;
    }
    writeln!(out, "];\n")?;

    writeln!(
        out,
        "pub static AOT_CALLBACK_ROOTS: &[AotCallbackRootEntry] = &["
    )?;
    for cb in callback_roots {
        writeln!(
            out,
            "    AotCallbackRootEntry {{ rva: 0x{:08X}, kind: {:?}, is_eh_continuation: {} }},",
            cb.rva, cb.kind, cb.is_eh_continuation
        )?;
    }
    writeln!(out, "];\n")?;

    writeln!(out, "pub static AOT_EH_CONTINUATIONS: &[u32] = &[")?;
    for &rva in eh_continuations {
        writeln!(out, "    0x{:08X},", rva)?;
    }
    writeln!(out, "];\n")?;

    writeln!(
        out,
        r#"pub fn lookup_aot_function(rva: u32) -> Option<extern "C" fn(u64, u64, u64, u64) -> u64> {{
    AOT_FUNCTIONS
        .binary_search_by_key(&rva, |e| e.begin_rva)
        .ok()
        .map(|idx| AOT_FUNCTIONS[idx].func)
}}

pub fn lookup_aot_export(name: &str) -> Option<extern "C" fn(u64, u64, u64, u64) -> u64> {{
    AOT_EXPORTS
        .iter()
        .find(|e| e.name == name)
        .map(|e| e.func)
}}

#[no_mangle]
pub extern "C" fn aw2_aot_set_image_base(base: u64) {{
    set_image_base(base);
}}

#[no_mangle]
pub extern "C" fn aw2_aot_function_count() -> usize {{
    AOT_FUNCTIONS.len()
}}

#[no_mangle]
pub extern "C" fn aw2_aot_get_entry(
    idx: usize,
    out_begin_rva: *mut u32,
    out_end_rva: *mut u32,
) -> usize {{
    if idx >= AOT_FUNCTIONS.len() {{
        return 0;
    }}
    let e = &AOT_FUNCTIONS[idx];
    unsafe {{
        if !out_begin_rva.is_null() {{
            *out_begin_rva = e.begin_rva;
        }}
        if !out_end_rva.is_null() {{
            *out_end_rva = e.end_rva;
        }}
    }}
    e.func as usize
}}

#[no_mangle]
pub extern "C" fn aw2_aot_lookup_rva(rva: u32) -> usize {{
    lookup_aot_function(rva).map(|f| f as usize).unwrap_or(0)
}}

/// `true` if entry `idx` contains no fallback/approximated instructions.
#[no_mangle]
pub extern "C" fn aw2_aot_entry_fully_lowered(idx: usize) -> bool {{
    AOT_FUNCTIONS.get(idx).is_some_and(|e| e.fully_lowered)
}}

/// Use the loader's live or verifier clock on each call, including from guest threads.
#[no_mangle]
pub extern "C" fn aw2_aot_set_tsc_reader(reader: extern "C" fn(bool) -> u64) {{
    let _ = HOST_TSC_READER.set(reader);
}}
"#
    )?;
    Ok(())
}

/// x86 memory-model support for generated code (emitted verbatim, so no `{{`
/// escaping). Mirrors `aw2-game/src/simd_ops.rs` (`tso_a64` +
/// `impl_lifted_atomics`), but it has to live in the generated prelude
/// because the AOT output is compiled into standalone dylibs.
///
/// * `rd8..rd64` / `wr8..wr64`: x86 is TSO, so every plain load is an acquire
///   and every plain store a release. On aarch64, naturally aligned accesses use
///   `ldapr*`/`stlr*` (FEAT_LRCPC, on by default for aarch64-apple-darwin).
///   Unaligned accesses stay plain. The asm is emitted in place at any opt level
///   and also acts as a compiler barrier. Other targets use Rust atomics.
/// * `rd64s`/`wr64s`: plain accesses for the lifter's own push/pop/ret slots.
///   Those slots are never shared.
/// * `aot_lk_*{8,16,32,64}`: x86 `lock`ed RMW (and the implicitly locked
///   `xchg mem`). They are fully ordered and return the **old** value.
///   Naturally aligned operands use Rust `SeqCst` atomics, which become LSE
///   `ld<op>al`/`swpal`/`casal` on Apple silicon. Misaligned operands take a
///   global mutex followed by a full fence.
/// * `aot_lk_cas128`: `cmpxchg16b`. Uses `caspal` on aarch64 when 16-byte
///   aligned, otherwise the mutex.
pub(super) const AOT_TSO_ATOMICS_PRELUDE: &str = r##"
#[cfg(target_arch = "aarch64")]
macro_rules! aot_tso_access_asm {
    ($rd:ident, $wr:ident, $t:ty, $chk:literal, $ldapr:literal, $ldr:literal, $stlr:literal, $str:literal, $r:literal) => {
        #[inline(always)]
        unsafe fn $rd(a: u64) -> $t {
            let v: u64;
            core::arch::asm!(
                $chk,
                concat!($ldapr, " {v:", $r, "}, [{a}]"),
                "b 3f",
                concat!("2: ", $ldr, " {v:", $r, "}, [{a}]"),
                "3:",
                a = in(reg) a, v = lateout(reg) v,
                options(nostack),
            );
            v as $t
        }
        #[inline(always)]
        unsafe fn $wr(a: u64, v: $t) {
            core::arch::asm!(
                $chk,
                concat!($stlr, " {v:", $r, "}, [{a}]"),
                "b 3f",
                concat!("2: ", $str, " {v:", $r, "}, [{a}]"),
                "3:",
                a = in(reg) a, v = in(reg) v as u64,
                options(nostack),
            );
        }
    };
}
#[cfg(target_arch = "aarch64")]
aot_tso_access_asm!(rd8, wr8, u8, "cmp xzr, xzr\nb.ne 2f", "ldaprb", "ldrb", "stlrb", "strb", "w");
#[cfg(target_arch = "aarch64")]
aot_tso_access_asm!(rd16, wr16, u16, "tst {a}, #1\nb.ne 2f", "ldaprh", "ldrh", "stlrh", "strh", "w");
#[cfg(target_arch = "aarch64")]
aot_tso_access_asm!(rd32, wr32, u32, "tst {a}, #3\nb.ne 2f", "ldapr", "ldr", "stlr", "str", "w");
#[cfg(target_arch = "aarch64")]
aot_tso_access_asm!(rd64, wr64, u64, "tst {a}, #7\nb.ne 2f", "ldapr", "ldr", "stlr", "str", "x");

#[cfg(not(target_arch = "aarch64"))]
macro_rules! aot_tso_access_portable {
    ($rd:ident, $wr:ident, $t:ty, $at:ty, $align:expr) => {
        #[inline(always)]
        unsafe fn $rd(a: u64) -> $t {
            if a & ($align - 1) == 0 {
                (*(a as *const $at)).load(core::sync::atomic::Ordering::Acquire)
            } else {
                core::ptr::read_unaligned(a as *const $t)
            }
        }
        #[inline(always)]
        unsafe fn $wr(a: u64, v: $t) {
            if a & ($align - 1) == 0 {
                (*(a as *const $at)).store(v, core::sync::atomic::Ordering::Release);
            } else {
                core::ptr::write_unaligned(a as *mut $t, v);
            }
        }
    };
}
#[cfg(not(target_arch = "aarch64"))]
aot_tso_access_portable!(rd8, wr8, u8, core::sync::atomic::AtomicU8, 1);
#[cfg(not(target_arch = "aarch64"))]
aot_tso_access_portable!(rd16, wr16, u16, core::sync::atomic::AtomicU16, 2);
#[cfg(not(target_arch = "aarch64"))]
aot_tso_access_portable!(rd32, wr32, u32, core::sync::atomic::AtomicU32, 4);
#[cfg(not(target_arch = "aarch64"))]
aot_tso_access_portable!(rd64, wr64, u64, core::sync::atomic::AtomicU64, 8);

#[inline(always)]
unsafe fn rd64s(a: u64) -> u64 { core::ptr::read_unaligned(a as *const u64) }
#[inline(always)]
unsafe fn wr64s(a: u64, v: u64) { core::ptr::write_unaligned(a as *mut u64, v) }

static AOT_ATOMIC_FALLBACK_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[inline(never)]
fn aot_atomic_fallback_guard() -> std::sync::MutexGuard<'static, ()> {
    AOT_ATOMIC_FALLBACK_LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

macro_rules! aot_lk_width {
    ($t:ty, $at:ty, $align:expr, $rmw:ident, $add:ident, $sub:ident, $and:ident, $or:ident, $xor:ident, $xchg:ident, $cas:ident) => {
        /// Locked generic RMW; returns the old value (zero-extended).
        #[inline(always)]
        unsafe fn $rmw(a: u64, f: impl Fn($t) -> $t) -> u64 {
            use core::sync::atomic::Ordering::{Relaxed, SeqCst};
            if a & ($align - 1) == 0 {
                let p = &*(a as *const $at);
                let mut cur = p.load(Relaxed);
                loop {
                    match p.compare_exchange_weak(cur, f(cur), SeqCst, Relaxed) {
                        Ok(o) => return o as u64,
                        Err(o) => cur = o,
                    }
                }
            } else {
                let _g = aot_atomic_fallback_guard();
                core::sync::atomic::fence(SeqCst);
                let o = core::ptr::read_unaligned(a as *const $t);
                core::ptr::write_unaligned(a as *mut $t, f(o));
                core::sync::atomic::fence(SeqCst);
                o as u64
            }
        }
        #[inline(always)]
        unsafe fn $add(a: u64, v: u64) -> u64 {
            if a & ($align - 1) == 0 {
                (*(a as *const $at)).fetch_add(v as $t, core::sync::atomic::Ordering::SeqCst) as u64
            } else {
                $rmw(a, |o| o.wrapping_add(v as $t))
            }
        }
        #[inline(always)]
        unsafe fn $sub(a: u64, v: u64) -> u64 {
            if a & ($align - 1) == 0 {
                (*(a as *const $at)).fetch_sub(v as $t, core::sync::atomic::Ordering::SeqCst) as u64
            } else {
                $rmw(a, |o| o.wrapping_sub(v as $t))
            }
        }
        #[inline(always)]
        unsafe fn $and(a: u64, v: u64) -> u64 {
            if a & ($align - 1) == 0 {
                (*(a as *const $at)).fetch_and(v as $t, core::sync::atomic::Ordering::SeqCst) as u64
            } else {
                $rmw(a, |o| o & (v as $t))
            }
        }
        #[inline(always)]
        unsafe fn $or(a: u64, v: u64) -> u64 {
            if a & ($align - 1) == 0 {
                (*(a as *const $at)).fetch_or(v as $t, core::sync::atomic::Ordering::SeqCst) as u64
            } else {
                $rmw(a, |o| o | (v as $t))
            }
        }
        #[inline(always)]
        unsafe fn $xor(a: u64, v: u64) -> u64 {
            if a & ($align - 1) == 0 {
                (*(a as *const $at)).fetch_xor(v as $t, core::sync::atomic::Ordering::SeqCst) as u64
            } else {
                $rmw(a, |o| o ^ (v as $t))
            }
        }
        #[inline(always)]
        unsafe fn $xchg(a: u64, v: u64) -> u64 {
            if a & ($align - 1) == 0 {
                (*(a as *const $at)).swap(v as $t, core::sync::atomic::Ordering::SeqCst) as u64
            } else {
                $rmw(a, |_| v as $t)
            }
        }
        /// Locked compare-and-exchange; returns the old value (success iff old == expected).
        #[inline(always)]
        unsafe fn $cas(a: u64, expected: u64, desired: u64) -> u64 {
            use core::sync::atomic::Ordering::SeqCst;
            if a & ($align - 1) == 0 {
                match (*(a as *const $at)).compare_exchange(expected as $t, desired as $t, SeqCst, SeqCst) {
                    Ok(o) | Err(o) => o as u64,
                }
            } else {
                $rmw(a, |o| if o == expected as $t { desired as $t } else { o })
            }
        }
    };
}
aot_lk_width!(u8, core::sync::atomic::AtomicU8, 1, aot_lk_rmw8, aot_lk_add8, aot_lk_sub8, aot_lk_and8, aot_lk_or8, aot_lk_xor8, aot_lk_xchg8, aot_lk_cas8);
aot_lk_width!(u16, core::sync::atomic::AtomicU16, 2, aot_lk_rmw16, aot_lk_add16, aot_lk_sub16, aot_lk_and16, aot_lk_or16, aot_lk_xor16, aot_lk_xchg16, aot_lk_cas16);
aot_lk_width!(u32, core::sync::atomic::AtomicU32, 4, aot_lk_rmw32, aot_lk_add32, aot_lk_sub32, aot_lk_and32, aot_lk_or32, aot_lk_xor32, aot_lk_xchg32, aot_lk_cas32);
aot_lk_width!(u64, core::sync::atomic::AtomicU64, 8, aot_lk_rmw64, aot_lk_add64, aot_lk_sub64, aot_lk_and64, aot_lk_or64, aot_lk_xor64, aot_lk_xchg64, aot_lk_cas64);

/// `lock cmpxchg16b`: compares `[a]` with `exp_hi:exp_lo` and stores `new_hi:new_lo` on match.
/// Returns `(success, old_lo, old_hi)`.
#[inline(always)]
unsafe fn aot_lk_cas128(a: u64, exp_lo: u64, exp_hi: u64, new_lo: u64, new_hi: u64) -> (bool, u64, u64) {
    #[cfg(target_arch = "aarch64")]
    {
        if a & 15 == 0 {
            let mut lo = exp_lo;
            let mut hi = exp_hi;
            core::arch::asm!(
                "caspal x0, x1, x2, x3, [{a}]",
                a = in(reg) a,
                inout("x0") lo,
                inout("x1") hi,
                in("x2") new_lo,
                in("x3") new_hi,
                options(nostack),
            );
            return (lo == exp_lo && hi == exp_hi, lo, hi);
        }
    }
    let _g = aot_atomic_fallback_guard();
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    let lo = core::ptr::read_unaligned(a as *const u64);
    let hi = core::ptr::read_unaligned((a + 8) as *const u64);
    let ok = lo == exp_lo && hi == exp_hi;
    if ok {
        core::ptr::write_unaligned(a as *mut u64, new_lo);
        core::ptr::write_unaligned((a + 8) as *mut u64, new_hi);
    }
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    (ok, lo, hi)
}
"##;
