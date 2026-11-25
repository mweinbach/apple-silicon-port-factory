//! dbghelp.dll: crash-reporting, stack-walking, minidump, and PDB line-info helpers.
//!
//! Backed by the loader's module table, the x64 `.pdata` unwinder (`crate::unwind`),
//! an MSF 7.0 / CodeView C13 PDB line-table reader, and a full Windows `MDMP` writer.

use crate::types::*;
use crate::unwind::{Context, ExceptionRecord, RBP, RSP};
use pe_image::RuntimeFunction;
use std::collections::HashMap;
use std::ffi::c_void;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, OnceLock};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

const ERROR_MOD_NOT_FOUND: u32 = 126;
const ERROR_INVALID_ADDRESS: u32 = 487;
const IMAGE_FILE_MACHINE_AMD64: u32 = 0x8664;
const ADDR_MODE_FLAT: u32 = 3;

static OPTIONS: AtomicU32 = AtomicU32::new(0);

#[derive(Clone, Debug)]
struct PdbLineEntry {
    rva: u32,
    line: u32,
    file_idx: usize,
}

struct PdbLineCache {
    /// Cached NUL-terminated ANSI & UTF-16 file paths kept at stable heap addresses
    /// for the lifetime of the `SymState` entry (`IMAGEHLP_LINE64::FileName` pointers).
    ansi_files: Vec<Box<[u8]>>,
    wide_files: Vec<Box<[u16]>>,
    lines: Vec<PdbLineEntry>,
}

#[derive(Clone, Debug)]
struct SymModule {
    base: u64,
    size: u32,
    module_name: String,
    image_name: String,
}

#[derive(Default)]
struct SymState {
    search_path: String,
    modules: Vec<SymModule>,
    /// Per-module base address -> parsed PDB line info (`None` = searched and not found).
    pdb_lines: HashMap<u64, Option<PdbLineCache>>,
}

fn sym_table() -> &'static Mutex<HashMap<usize, SymState>> {
    static T: OnceLock<Mutex<HashMap<usize, SymState>>> = OnceLock::new();
    T.get_or_init(|| Mutex::new(HashMap::new()))
}

fn proc_key(process: HANDLE) -> usize {
    process as usize
}

/// Returns the new option mask (as Windows does).
pub unsafe extern "win64" fn SymSetOptions(opts: u32) -> u32 {
    OPTIONS.store(opts, Ordering::Relaxed);
    opts
}

pub unsafe extern "win64" fn SymGetOptions() -> u32 {
    OPTIONS.load(Ordering::Relaxed)
}

pub unsafe extern "win64" fn SymFunctionTableAccess64(_process: HANDLE, addr: u64) -> *mut c_void {
    match crate::unwind::lookup_function_entry(addr) {
        Some((_, rf)) => rf as *mut c_void,
        None => std::ptr::null_mut(),
    }
}

pub unsafe extern "win64" fn SymGetModuleBase64(process: HANDLE, addr: u64) -> u64 {
    if let Some(b) = crate::try_loader().and_then(|l| l.module_from_address(addr as usize)) {
        return b as u64;
    }
    let tbl = sym_table().lock().unwrap();
    if let Some(st) = tbl.get(&proc_key(process)) {
        for m in &st.modules {
            let end = m.base.saturating_add(m.size.max(1) as u64);
            if addr >= m.base && addr < end {
                return m.base;
            }
        }
    }
    0
}

fn init_sym_state(process: HANDLE, search_path: Option<String>, invade: bool) -> BOOL {
    let mut st = SymState {
        search_path: search_path.unwrap_or_else(|| ".".to_string()),
        modules: Vec::new(),
        pdb_lines: HashMap::new(),
    };
    if invade {
        if let Some(loader) = crate::try_loader() {
            for (base, size, path) in loader.loaded_modules() {
                let mod_name = path
                    .rsplit(['\\', '/'])
                    .next()
                    .unwrap_or(&path)
                    .trim_end_matches(".dll")
                    .trim_end_matches(".exe")
                    .to_string();
                st.modules.push(SymModule {
                    base: base as u64,
                    size,
                    module_name: mod_name,
                    image_name: path,
                });
            }
        }
    }
    sym_table().lock().unwrap().insert(proc_key(process), st);
    TRUE
}

pub unsafe extern "win64" fn SymInitialize(
    process: HANDLE,
    user_search_path: LPCSTR,
    f_invade_process: BOOL,
) -> BOOL {
    init_sym_state(process, crate::strings::from_ansi_opt(user_search_path), f_invade_process != 0)
}

pub unsafe extern "win64" fn SymInitializeW(
    process: HANDLE,
    user_search_path: LPCWSTR,
    f_invade_process: BOOL,
) -> BOOL {
    init_sym_state(process, crate::strings::from_wide_opt(user_search_path), f_invade_process != 0)
}

pub unsafe extern "win64" fn SymCleanup(process: HANDLE) -> BOOL {
    sym_table().lock().unwrap().remove(&proc_key(process));
    TRUE
}

pub unsafe extern "win64" fn SymGetSearchPath(
    process: HANDLE,
    search_path: LPSTR,
    search_path_len: u32,
) -> BOOL {
    if search_path.is_null() || search_path_len == 0 {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let path = sym_table()
        .lock()
        .unwrap()
        .get(&proc_key(process))
        .map(|s| s.search_path.clone())
        .unwrap_or_else(|| ".".to_string());
    let bytes = path.as_bytes();
    let copy_n = bytes.len().min((search_path_len - 1) as usize);
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), search_path, copy_n);
    std::ptr::write_unaligned(search_path.add(copy_n), 0);
    TRUE
}

pub unsafe extern "win64" fn SymGetSearchPathW(
    process: HANDLE,
    search_path: LPWSTR,
    search_path_len: u32,
) -> BOOL {
    if search_path.is_null() || search_path_len == 0 {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let path = sym_table()
        .lock()
        .unwrap()
        .get(&proc_key(process))
        .map(|s| s.search_path.clone())
        .unwrap_or_else(|| ".".to_string());
    crate::strings::write_wide_trunc(&path, search_path, search_path_len as usize);
    TRUE
}

pub unsafe extern "win64" fn SymSetSearchPath(process: HANDLE, search_path: LPCSTR) -> BOOL {
    let s = crate::strings::from_ansi_opt(search_path).unwrap_or_default();
    let mut tbl = sym_table().lock().unwrap();
    let st = tbl.entry(proc_key(process)).or_default();
    st.search_path = s;
    // Invalidate failed PDB lookups so callers that add a symbol directory can retry.
    st.pdb_lines.retain(|_, v| v.is_some());
    TRUE
}

pub unsafe extern "win64" fn SymSetSearchPathW(process: HANDLE, search_path: LPCWSTR) -> BOOL {
    let s = crate::strings::from_wide_opt(search_path).unwrap_or_default();
    let mut tbl = sym_table().lock().unwrap();
    let st = tbl.entry(proc_key(process)).or_default();
    st.search_path = s;
    st.pdb_lines.retain(|_, v| v.is_some());
    TRUE
}

pub unsafe extern "win64" fn SymLoadModule64(
    process: HANDLE,
    _file: HANDLE,
    image_name: LPCSTR,
    module_name: LPCSTR,
    base_of_dll: u64,
    size_of_dll: u32,
) -> u64 {
    let img = crate::strings::from_ansi_opt(image_name).unwrap_or_default();
    let mname = crate::strings::from_ansi_opt(module_name).unwrap_or_else(|| {
        img.rsplit(['\\', '/']).next().unwrap_or(&img).to_string()
    });
    let base = if base_of_dll != 0 {
        base_of_dll
    } else {
        crate::try_loader()
            .and_then(|l| l.get_module_handle(if img.is_empty() { None } else { Some(&img) }))
            .unwrap_or(0) as u64
    };
    if base == 0 {
        crate::teb::set_last_error(ERROR_MOD_NOT_FOUND);
        return 0;
    }
    let mut tbl = sym_table().lock().unwrap();
    let st = tbl.entry(proc_key(process)).or_default();
    st.modules.retain(|m| m.base != base);
    st.modules.push(SymModule {
        base,
        size: size_of_dll,
        module_name: mname,
        image_name: img,
    });
    base
}

fn find_sym_module(process: HANDLE, addr: u64) -> Option<SymModule> {
    if let Some(loader) = crate::try_loader() {
        for (base, size, path) in loader.loaded_modules() {
            let b = base as u64;
            let end = b.saturating_add(size as u64);
            if addr >= b && addr < end {
                let mod_name = path
                    .rsplit(['\\', '/'])
                    .next()
                    .unwrap_or(&path)
                    .to_string();
                return Some(SymModule {
                    base: b,
                    size,
                    module_name: mod_name,
                    image_name: path,
                });
            }
        }
    }
    let tbl = sym_table().lock().unwrap();
    tbl.get(&proc_key(process)).and_then(|st| {
        st.modules
            .iter()
            .find(|m| addr >= m.base && addr < m.base.saturating_add(m.size.max(1) as u64))
            .cloned()
    })
}

pub unsafe extern "win64" fn SymGetModuleInfo64(
    process: HANDLE,
    qw_addr: u64,
    module_info: *mut u8,
) -> BOOL {
    if module_info.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let Some(m) = find_sym_module(process, qw_addr) else {
        crate::teb::set_last_error(ERROR_MOD_NOT_FOUND);
        return FALSE;
    };
    let cb = std::ptr::read_unaligned(module_info as *const u32) as usize;
    if cb < 584 {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    std::ptr::write_bytes(module_info.add(4), 0, cb - 4);
    std::ptr::write_unaligned(module_info.add(8) as *mut u64, m.base);
    std::ptr::write_unaligned(module_info.add(16) as *mut u32, m.size);
    std::ptr::write_unaligned(module_info.add(32) as *mut u32, 3); // SymExport
    let copy_ascii = |dst: *mut u8, cap: usize, s: &str| {
        let n = s.len().min(cap - 1);
        std::ptr::copy_nonoverlapping(s.as_bytes().as_ptr(), dst, n);
        std::ptr::write_unaligned(dst.add(n), 0);
    };
    copy_ascii(module_info.add(36), 32, &m.module_name);
    copy_ascii(module_info.add(68), 256, &m.image_name);
    copy_ascii(module_info.add(324), 256, &m.image_name);
    TRUE
}

pub unsafe extern "win64" fn SymGetModuleInfoW64(
    process: HANDLE,
    qw_addr: u64,
    module_info: *mut u8,
) -> BOOL {
    if module_info.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let Some(m) = find_sym_module(process, qw_addr) else {
        crate::teb::set_last_error(ERROR_MOD_NOT_FOUND);
        return FALSE;
    };
    let cb = std::ptr::read_unaligned(module_info as *const u32) as usize;
    if cb < 1124 {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    std::ptr::write_bytes(module_info.add(4), 0, cb - 4);
    std::ptr::write_unaligned(module_info.add(8) as *mut u64, m.base);
    std::ptr::write_unaligned(module_info.add(16) as *mut u32, m.size);
    std::ptr::write_unaligned(module_info.add(32) as *mut u32, 3); // SymExport
    crate::strings::write_wide_trunc(&m.module_name, module_info.add(36) as *mut u16, 32);
    crate::strings::write_wide_trunc(&m.image_name, module_info.add(100) as *mut u16, 256);
    crate::strings::write_wide_trunc(&m.image_name, module_info.add(612) as *mut u16, 256);
    TRUE
}

fn resolve_symbol_name(process: HANDLE, addr: u64) -> (String, u64, u64) {
    let mod_base = unsafe { SymGetModuleBase64(process, addr) };
    let sym = crate::symbolize(addr as usize);
    let disp = if mod_base != 0 && addr >= mod_base {
        addr - mod_base
    } else {
        0
    };
    (sym, mod_base, disp)
}

pub unsafe extern "win64" fn SymFromAddr(
    process: HANDLE,
    addr: u64,
    displacement: *mut u64,
    symbol: *mut u8,
) -> BOOL {
    if symbol.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let (name, mod_base, disp) = resolve_symbol_name(process, addr);
    if !displacement.is_null() {
        std::ptr::write_unaligned(displacement, disp);
    }
    let max_name = std::ptr::read_unaligned(symbol.add(80) as *const u32) as usize;
    std::ptr::write_unaligned(symbol.add(32) as *mut u64, mod_base);
    std::ptr::write_unaligned(symbol.add(56) as *mut u64, addr);
    let copy_n = if max_name > 0 {
        name.len().min(max_name - 1)
    } else {
        0
    };
    std::ptr::write_unaligned(symbol.add(76) as *mut u32, copy_n as u32);
    if max_name > 0 {
        std::ptr::copy_nonoverlapping(name.as_bytes().as_ptr(), symbol.add(84), copy_n);
        std::ptr::write_unaligned(symbol.add(84 + copy_n), 0);
    }
    TRUE
}

pub unsafe extern "win64" fn SymFromAddrW(
    process: HANDLE,
    addr: u64,
    displacement: *mut u64,
    symbol: *mut u8,
) -> BOOL {
    if symbol.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let (name, mod_base, disp) = resolve_symbol_name(process, addr);
    if !displacement.is_null() {
        std::ptr::write_unaligned(displacement, disp);
    }
    let max_name = std::ptr::read_unaligned(symbol.add(80) as *const u32) as usize;
    std::ptr::write_unaligned(symbol.add(32) as *mut u64, mod_base);
    std::ptr::write_unaligned(symbol.add(56) as *mut u64, addr);
    let units: Vec<u16> = name.encode_utf16().collect();
    let copy_n = if max_name > 0 {
        units.len().min(max_name - 1)
    } else {
        0
    };
    std::ptr::write_unaligned(symbol.add(76) as *mut u32, copy_n as u32);
    if max_name > 0 {
        let dst = symbol.add(84) as *mut u16;
        for (i, &u) in units[..copy_n].iter().enumerate() {
            std::ptr::write_unaligned(dst.add(i), u);
        }
        std::ptr::write_unaligned(dst.add(copy_n), 0);
    }
    TRUE
}

pub unsafe extern "win64" fn SymGetSymFromAddr64(
    process: HANDLE,
    addr: u64,
    displacement: *mut u64,
    symbol: *mut u8,
) -> BOOL {
    if symbol.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    // IMAGEHLP_SYMBOL64: +0 SizeOfStruct(u32), +8 Address(u64), +16 Size(u32),
    // +20 Flags(u32), +24 MaxNameLength(u32), +28 Name[1].
    let (name, _mod_base, disp) = resolve_symbol_name(process, addr);
    if !displacement.is_null() {
        std::ptr::write_unaligned(displacement, disp);
    }
    let max_name = std::ptr::read_unaligned(symbol.add(24) as *const u32) as usize;
    std::ptr::write_unaligned(symbol.add(8) as *mut u64, addr);
    if max_name > 0 {
        let copy_n = name.len().min(max_name - 1);
        std::ptr::copy_nonoverlapping(name.as_bytes().as_ptr(), symbol.add(28), copy_n);
        std::ptr::write_unaligned(symbol.add(28 + copy_n), 0);
    }
    TRUE
}

pub unsafe extern "win64" fn EnumerateLoadedModules64(
    _process: HANDLE,
    callback: *const c_void,
    user_context: *mut c_void,
) -> BOOL {
    if callback.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let mods = crate::try_loader().map(|l| l.loaded_modules()).unwrap_or_default();
    for (base, size, path) in mods {
        let mut cstr = path.into_bytes();
        cstr.push(0);
        let cont = crate::call_guest8(
            callback as usize,
            cstr.as_ptr() as u64,
            base as u64,
            size as u64,
            user_context as u64,
            0,
            0,
            0,
            0,
        );
        if cont == 0 {
            break;
        }
    }
    TRUE
}

pub unsafe extern "win64" fn EnumerateLoadedModulesW64(
    _process: HANDLE,
    callback: *const c_void,
    user_context: *mut c_void,
) -> BOOL {
    if callback.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let mods = crate::try_loader().map(|l| l.loaded_modules()).unwrap_or_default();
    for (base, size, path) in mods {
        let wide = crate::strings::to_wide_z(&path);
        let cont = crate::call_guest8(
            callback as usize,
            wide.as_ptr() as u64,
            base as u64,
            size as u64,
            user_context as u64,
            0,
            0,
            0,
            0,
        );
        if cont == 0 {
            break;
        }
    }
    TRUE
}

pub unsafe extern "win64" fn SymEnumerateModulesW64(
    process: HANDLE,
    callback: *const c_void,
    user_context: *mut c_void,
) -> BOOL {
    if callback.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let _ = process;
    let mods = crate::try_loader().map(|l| l.loaded_modules()).unwrap_or_default();
    for (base, _size, path) in mods {
        let mod_name = path.rsplit(['\\', '/']).next().unwrap_or(&path);
        let wide = crate::strings::to_wide_z(mod_name);
        // PSYM_ENUMMODULES_CALLBACKW64(PCWSTR ModuleName, DWORD64 BaseOfDll, PVOID UserContext)
        let cont = crate::call_guest8(
            callback as usize,
            wide.as_ptr() as u64,
            base as u64,
            user_context as u64,
            0,
            0,
            0,
            0,
            0,
        );
        if cont == 0 {
            break;
        }
    }
    TRUE
}

// ============================================================================
// StackWalk64 (backed by `crate::unwind` x64 `.pdata` unwinder)
// ============================================================================

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct Address64 {
    offset: u64,
    segment: u16,
    _pad: u16,
    mode: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct KdHelp64 {
    thread: u64,
    th_callback_stack: u32,
    th_callback_bstore: u32,
    next_callback: u32,
    frame_pointer: u32,
    ki_call_user_mode: u64,
    ke_user_callback_dispatcher: u64,
    system_range_start: u64,
    ki_user_exception_dispatcher: u64,
    stack_base: u64,
    stack_limit: u64,
    build_version: u32,
    retpoline_stub_function_table_size: u32,
    retpoline_stub_function_table: u64,
    retpoline_stub_offset: u32,
    retpoline_stub_size: u32,
    reserved0: [u64; 2],
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct StackFrame64 {
    addr_pc: Address64,
    addr_return: Address64,
    addr_frame: Address64,
    addr_stack: Address64,
    addr_bstore: Address64,
    func_table_entry: u64,
    params: [u64; 4],
    far: BOOL,
    virtual_: BOOL,
    reserved: [u64; 3],
    kd_help: KdHelp64,
}

const _: () = assert!(std::mem::size_of::<Address64>() == 16);
const _: () = assert!(std::mem::size_of::<KdHelp64>() == 112);
const _: () = assert!(std::mem::size_of::<StackFrame64>() == 264);

const STACKWALK_INIT_MAGIC: u64 = 0x5357_3634_0000_0001;
const STACKWALK_MAX_DEPTH: u64 = 256;

fn flat_addr(offset: u64) -> Address64 {
    Address64 {
        offset,
        segment: 0,
        _pad: 0,
        mode: ADDR_MODE_FLAT,
    }
}

unsafe fn sw_read_mem(
    h_process: HANDLE,
    read_mem_cb: *const c_void,
    addr: u64,
    buf: &mut [u8],
) -> bool {
    if addr < 0x1000 || buf.is_empty() {
        return false;
    }
    if !read_mem_cb.is_null() {
        let mut n_read: u32 = 0;
        let ok = crate::call_guest8(
            read_mem_cb as usize,
            h_process as u64,
            addr,
            buf.as_mut_ptr() as u64,
            buf.len() as u64,
            &mut n_read as *mut u32 as u64,
            0,
            0,
            0,
        );
        return ok != 0 && (n_read as usize) == buf.len();
    }
    crate::crash::safe_read(addr, buf)
}

unsafe fn sw_read_u64(h_process: HANDLE, read_mem_cb: *const c_void, addr: u64) -> Option<u64> {
    let mut b = [0u8; 8];
    sw_read_mem(h_process, read_mem_cb, addr, &mut b).then(|| u64::from_le_bytes(b))
}

unsafe fn sw_lookup_rf(
    h_process: HANDLE,
    pc: u64,
    fn_table_cb: *const c_void,
    mod_base_cb: *const c_void,
) -> Option<(u64, RuntimeFunction, u64)> {
    if pc < 0x1000 {
        return None;
    }
    let default_ft = SymFunctionTableAccess64 as *const () as usize;
    let default_mb = SymGetModuleBase64 as *const () as usize;

    let use_custom_ft = !fn_table_cb.is_null() && (fn_table_cb as usize) != default_ft;
    let use_custom_mb = !mod_base_cb.is_null() && (mod_base_cb as usize) != default_mb;

    // Per PFunctionTableAccessRoutine64, both the default lookup and custom
    // FunctionTableAccessRoutine64 return a PVOID to a RUNTIME_FUNCTION in the
    // caller's own address space (not a target-process address).
    if !use_custom_ft && !use_custom_mb {
        let (base, rf_ptr) = crate::unwind::lookup_function_entry(pc)?;
        let mut raw = [0u8; 12];
        if !crate::crash::safe_read(rf_ptr as u64, &mut raw) {
            return None;
        }
        let rf = std::ptr::read_unaligned(raw.as_ptr() as *const RuntimeFunction);
        return Some((base, rf, rf_ptr as u64));
    }

    let rf_addr = if use_custom_ft {
        crate::call_guest8(fn_table_cb as usize, h_process as u64, pc, 0, 0, 0, 0, 0, 0)
    } else {
        SymFunctionTableAccess64(h_process, pc) as u64
    };
    if rf_addr == 0 {
        return None;
    }
    let base = if use_custom_mb {
        crate::call_guest8(mod_base_cb as usize, h_process as u64, pc, 0, 0, 0, 0, 0, 0)
    } else {
        SymGetModuleBase64(h_process, pc)
    };
    if base == 0 {
        return None;
    }
    let mut raw = [0u8; 12];
    if !crate::crash::safe_read(rf_addr, &mut raw) {
        return None;
    }
    let rf = std::ptr::read_unaligned(raw.as_ptr() as *const RuntimeFunction);
    Some((base, rf, rf_addr))
}

/// Unwind `ctx` by one x86-64 frame using `.pdata` (via `safe_virtual_unwind`)
/// or the x64 leaf-function convention (`[rsp]`) when no `.pdata` entry exists.
unsafe fn sw_unwind_one(
    h_process: HANDLE,
    ctx: &mut Context,
    is_first_frame: bool,
    fn_table_cb: *const c_void,
    mod_base_cb: *const c_void,
    read_mem_cb: *const c_void,
) -> bool {
    if ctx.rip == 0 {
        return false;
    }
    let old_rip = ctx.rip;
    let old_rsp = ctx.gpr[RSP];
    let lookup_pc = if is_first_frame {
        ctx.rip
    } else {
        ctx.rip.saturating_sub(1)
    };
    let allow_epilog = is_first_frame;

    let unwound = if let Some((base, rf, _)) =
        sw_lookup_rf(h_process, lookup_pc, fn_table_cb, mod_base_cb)
    {
        crate::unwind::safe_virtual_unwind_with(base, lookup_pc, rf, ctx, allow_epilog, &mut |addr, buf| {
            sw_read_mem(h_process, read_mem_cb, addr, buf)
        })
    } else {
        // Leaf function (no `.pdata` entry): return address sits directly at [RSP].
        match sw_read_u64(h_process, read_mem_cb, ctx.gpr[RSP]) {
            Some(ret) if ret != 0 => {
                ctx.rip = ret;
                ctx.gpr[RSP] = ctx.gpr[RSP].wrapping_add(8);
                true
            }
            _ => false,
        }
    };
    if !unwound || ctx.rip == 0 {
        return false;
    }
    if ctx.gpr[RSP] < old_rsp || (ctx.gpr[RSP] == old_rsp && ctx.rip == old_rip) {
        return false;
    }
    true
}

unsafe fn sw_fill_params(
    h_process: HANDLE,
    read_mem_cb: *const c_void,
    rsp: u64,
    sf: &mut StackFrame64,
) {
    for i in 0..4 {
        let addr = rsp.wrapping_add(8 + (i as u64) * 8);
        sf.params[i] = sw_read_u64(h_process, read_mem_cb, addr).unwrap_or(0);
    }
}

pub unsafe extern "win64" fn StackWalk64(
    machine_type: u32,
    h_process: HANDLE,
    _h_thread: HANDLE,
    stack_frame: *mut u8,
    context_record: *mut u8,
    read_memory_routine: *const c_void,
    function_table_access_routine: *const c_void,
    get_module_base_routine: *const c_void,
    _translate_address: *const c_void,
) -> BOOL {
    if stack_frame.is_null() || machine_type != IMAGE_FILE_MACHINE_AMD64 {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }

    let mut sf = std::ptr::read_unaligned(stack_frame as *const StackFrame64);
    let mut ctx = if !context_record.is_null() {
        std::ptr::read_unaligned(context_record as *const Context)
    } else {
        let mut c = Context::zeroed();
        c.rip = sf.addr_pc.offset;
        c.gpr[RSP] = sf.addr_stack.offset;
        c.gpr[RBP] = sf.addr_frame.offset;
        c
    };

    if sf.reserved[0] != STACKWALK_INIT_MAGIC {
        // First call: report the starting frame, then peek one frame ahead for AddrReturn.
        if sf.addr_pc.offset != 0 {
            ctx.rip = sf.addr_pc.offset;
        }
        if sf.addr_stack.offset != 0 {
            ctx.gpr[RSP] = sf.addr_stack.offset;
        }
        if sf.addr_frame.offset != 0 {
            ctx.gpr[RBP] = sf.addr_frame.offset;
        }
        if ctx.rip == 0 {
            return FALSE;
        }
        sf.addr_pc = flat_addr(ctx.rip);
        sf.addr_stack = flat_addr(ctx.gpr[RSP]);
        sf.addr_frame = flat_addr(ctx.gpr[RBP]);
        sf.func_table_entry = sw_lookup_rf(
            h_process,
            ctx.rip,
            function_table_access_routine,
            get_module_base_routine,
        )
        .map(|(_, _, ptr)| ptr)
        .unwrap_or(0);
        sw_fill_params(h_process, read_memory_routine, ctx.gpr[RSP], &mut sf);

        let mut peek = ctx;
        let next_rip = if sw_unwind_one(
            h_process,
            &mut peek,
            true,
            function_table_access_routine,
            get_module_base_routine,
            read_memory_routine,
        ) {
            peek.rip
        } else {
            0
        };
        sf.addr_return = flat_addr(next_rip);
        sf.far = FALSE;
        sf.virtual_ = TRUE;
        sf.reserved[0] = STACKWALK_INIT_MAGIC;
        sf.reserved[1] = 1;
        std::ptr::write_unaligned(stack_frame as *mut StackFrame64, sf);
        return TRUE;
    }

    // Subsequent call: advance one frame from the current context.
    let depth = sf.reserved[1];
    if sf.addr_return.offset == 0 || depth >= STACKWALK_MAX_DEPTH {
        sf.addr_pc = flat_addr(0);
        sf.addr_return = flat_addr(0);
        std::ptr::write_unaligned(stack_frame as *mut StackFrame64, sf);
        return FALSE;
    }

    let is_first_step = depth == 1;
    if !sw_unwind_one(
        h_process,
        &mut ctx,
        is_first_step,
        function_table_access_routine,
        get_module_base_routine,
        read_memory_routine,
    ) {
        sf.addr_pc = flat_addr(0);
        sf.addr_return = flat_addr(0);
        std::ptr::write_unaligned(stack_frame as *mut StackFrame64, sf);
        return FALSE;
    }

    if !context_record.is_null() {
        std::ptr::write_unaligned(context_record as *mut Context, ctx);
    }

    sf.addr_pc = flat_addr(ctx.rip);
    sf.addr_stack = flat_addr(ctx.gpr[RSP]);
    sf.addr_frame = flat_addr(ctx.gpr[RBP]);
    sf.func_table_entry = sw_lookup_rf(
        h_process,
        ctx.rip.saturating_sub(1),
        function_table_access_routine,
        get_module_base_routine,
    )
    .map(|(_, _, ptr)| ptr)
    .unwrap_or(0);
    sw_fill_params(h_process, read_memory_routine, ctx.gpr[RSP], &mut sf);

    let mut peek = ctx;
    let next_rip = if sw_unwind_one(
        h_process,
        &mut peek,
        false,
        function_table_access_routine,
        get_module_base_routine,
        read_memory_routine,
    ) {
        peek.rip
    } else {
        0
    };
    sf.addr_return = flat_addr(next_rip);
    sf.reserved[1] = depth + 1;
    std::ptr::write_unaligned(stack_frame as *mut StackFrame64, sf);
    TRUE
}

// ============================================================================
// MiniDumpWriteDump (valid Windows `MDMP` writer)
// ============================================================================

const MINIDUMP_SIGNATURE: u32 = 0x504D_444D; // "MDMP"
const MINIDUMP_VERSION: u32 = 0x0000_A793; // 42899
const MINIDUMP_THREAD_LIST_STREAM: u32 = 3;
const MINIDUMP_MODULE_LIST_STREAM: u32 = 4;
const MINIDUMP_MEMORY_LIST_STREAM: u32 = 5;
const MINIDUMP_EXCEPTION_STREAM: u32 = 6;
const MINIDUMP_SYSTEM_INFO_STREAM: u32 = 7;
const MINIDUMP_MISC_INFO_STREAM: u32 = 15;
const MINIDUMP_MISC1_PROCESS_ID: u32 = 0x0000_0001;

// MINIDUMP_TYPE flags requiring full/private process-wide memory enumeration or memory
// scrubbing that this writer does not implement; reject honestly rather than emitting an
// unfiltered or partial dump under those flags.
const MINIDUMP_WITH_FULL_MEMORY: u32 = 0x0000_0002;
const MINIDUMP_FILTER_MEMORY: u32 = 0x0000_0008;
const MINIDUMP_FILTER_MODULE_PATHS: u32 = 0x0000_0080;
const MINIDUMP_WITH_PRIVATE_RW_MEMORY: u32 = 0x0000_0200;
const MINIDUMP_WITH_FULL_MEMORY_INFO: u32 = 0x0000_0800;
const MINIDUMP_WITH_PRIVATE_WC_MEMORY: u32 = 0x0001_0000;
const MINIDUMP_FILTER_TRIAGE: u32 = 0x0010_0000;
const MINIDUMP_UNSUPPORTED_FLAGS: u32 = MINIDUMP_WITH_FULL_MEMORY
    | MINIDUMP_FILTER_MEMORY
    | MINIDUMP_WITH_PRIVATE_RW_MEMORY
    | MINIDUMP_WITH_FULL_MEMORY_INFO
    | MINIDUMP_WITH_PRIVATE_WC_MEMORY
    | MINIDUMP_FILTER_TRIAGE;

/// Strip directory components from a module or PDB path for `MiniDumpFilterModulePaths` (`0x80`),
/// keeping only the filename so usernames and directory layouts are not serialized into dumps.
fn filter_module_path(path: &str) -> &str {
    let trimmed = path.trim_end_matches(['\\', '/']);
    trimmed.rsplit(['\\', '/']).next().unwrap_or(trimmed)
}

/// Strip directory components from the embedded PDB path of a CodeView `RSDS` or `NB10` record
/// when `MiniDumpFilterModulePaths` (`0x80`) is requested.
fn filter_codeview_pdb_path(cv: Vec<u8>) -> Vec<u8> {
    let hdr_len = if cv.len() > 24 && &cv[0..4] == b"RSDS" {
        24usize
    } else if cv.len() > 16 && &cv[0..4] == b"NB10" {
        16usize
    } else {
        return cv;
    };
    let tail = &cv[hdr_len..];
    let nul_pos = tail.iter().position(|&b| b == 0).unwrap_or(tail.len());
    let pdb_path = String::from_utf8_lossy(&tail[..nul_pos]);
    let filtered = filter_module_path(&pdb_path);
    let mut out = Vec::with_capacity(hdr_len + filtered.len() + 1);
    out.extend_from_slice(&cv[..hdr_len]);
    out.extend_from_slice(filtered.as_bytes());
    out.push(0);
    out
}

struct MdmpBuilder {
    buf: Vec<u8>,
}

impl MdmpBuilder {
    fn new() -> Self {
        Self { buf: Vec::with_capacity(64 * 1024) }
    }

    fn rva(&self) -> u32 {
        self.buf.len() as u32
    }

    fn align4(&mut self) {
        while (self.buf.len() & 3) != 0 {
            self.buf.push(0);
        }
    }

    fn push_u8(&mut self, v: u8) {
        self.buf.push(v);
    }

    fn push_u16(&mut self, v: u16) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    fn push_u32(&mut self, v: u32) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    fn push_u64(&mut self, v: u64) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    fn push_bytes(&mut self, b: &[u8]) -> (u32, u32) {
        let rva = self.rva();
        self.buf.extend_from_slice(b);
        (b.len() as u32, rva)
    }

    fn push_minidump_string(&mut self, s: &str) -> u32 {
        self.align4();
        let rva = self.rva();
        let units: Vec<u16> = s.encode_utf16().collect();
        self.push_u32((units.len() * 2) as u32);
        for u in units {
            self.push_u16(u);
        }
        self.push_u16(0); // NUL terminator (not counted in byte length)
        rva
    }

    fn patch_u32(&mut self, off: usize, v: u32) {
        self.buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
    }
}

/// Extract PE `TimeDateStamp`, `CheckSum`, and optional CodeView `RSDS` record bytes
/// from a mapped PE image in guest memory using fault-safe reads.
fn read_pe_debug_metadata(base: u64) -> (u32, u32, Option<Vec<u8>>) {
    let mut dos = [0u8; 64];
    if !crate::crash::safe_read(base, &mut dos) || &dos[0..2] != b"MZ" {
        return (0, 0, None);
    }
    let e_lfanew = u32::from_le_bytes([dos[60], dos[61], dos[62], dos[63]]) as u64;
    if e_lfanew < 64 || e_lfanew > 0x10_0000 {
        return (0, 0, None);
    }
    let mut nt = [0u8; 264];
    if !crate::crash::safe_read(base.wrapping_add(e_lfanew), &mut nt) || &nt[0..4] != b"PE\0\0" {
        return (0, 0, None);
    }
    let time_date_stamp = u32::from_le_bytes([nt[8], nt[9], nt[10], nt[11]]);
    let opt_magic = u16::from_le_bytes([nt[24], nt[25]]);
    let checksum = u32::from_le_bytes([nt[24 + 64], nt[24 + 65], nt[24 + 66], nt[24 + 67]]);

    // DataDirectory[6] = IMAGE_DIRECTORY_ENTRY_DEBUG.
    // PE32+: data directories start at opt+112 (nt+136); entry 6 at nt+136+48 = nt+184.
    // PE32:  data directories start at opt+96  (nt+120); entry 6 at nt+120+48 = nt+168.
    let dd_off = match opt_magic {
        0x20B => 24 + 112 + 6 * 8,
        0x10B => 24 + 96 + 6 * 8,
        _ => return (time_date_stamp, checksum, None),
    };
    let num_rva = u32::from_le_bytes([
        nt[dd_off - 6 * 8 - 4],
        nt[dd_off - 6 * 8 - 3],
        nt[dd_off - 6 * 8 - 2],
        nt[dd_off - 6 * 8 - 1],
    ]);
    if num_rva <= 6 || dd_off + 8 > nt.len() {
        return (time_date_stamp, checksum, None);
    }
    let dbg_rva = u32::from_le_bytes([nt[dd_off], nt[dd_off + 1], nt[dd_off + 2], nt[dd_off + 3]]);
    let dbg_sz =
        u32::from_le_bytes([nt[dd_off + 4], nt[dd_off + 5], nt[dd_off + 6], nt[dd_off + 7]]);
    if dbg_rva == 0 || dbg_sz < 28 {
        return (time_date_stamp, checksum, None);
    }
    let entries = (dbg_sz / 28).min(32);
    for i in 0..entries {
        let mut dir = [0u8; 28];
        if !crate::crash::safe_read(
            base.wrapping_add(dbg_rva as u64).wrapping_add(i as u64 * 28),
            &mut dir,
        ) {
            break;
        }
        let dbg_type = u32::from_le_bytes([dir[12], dir[13], dir[14], dir[15]]);
        if dbg_type == 2 {
            // IMAGE_DEBUG_TYPE_CODEVIEW
            let data_sz = u32::from_le_bytes([dir[16], dir[17], dir[18], dir[19]]) as usize;
            let addr_rva = u32::from_le_bytes([dir[20], dir[21], dir[22], dir[23]]) as u64;
            if addr_rva != 0 && (24..=4096).contains(&data_sz) {
                let mut cv = vec![0u8; data_sz];
                if crate::crash::safe_read(base.wrapping_add(addr_rva), &mut cv) {
                    return (time_date_stamp, checksum, Some(cv));
                }
            }
        }
    }
    (time_date_stamp, checksum, None)
}

/// Capture up to `max_bytes` starting at `start` using fault-safe page-by-page reads.
fn safe_read_range(start: u64, max_bytes: usize) -> Vec<u8> {
    if start < 0x1000 || max_bytes == 0 {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut cur = start;
    while out.len() < max_bytes {
        let page_rem = (0x1000 - (cur & 0xFFF)) as usize;
        let step = page_rem.min(max_bytes - out.len());
        let mut chunk = vec![0u8; step];
        if !crate::crash::safe_read(cur, &mut chunk) {
            break;
        }
        out.extend_from_slice(&chunk);
        cur = match cur.checked_add(step as u64) {
            Some(next) => next,
            None => break,
        };
    }
    out
}

pub unsafe extern "win64" fn MiniDumpWriteDump(
    h_process: HANDLE,
    process_id: u32,
    h_file: HANDLE,
    dump_type: u32,
    exception_param: *const u8,
    user_stream_param: *const u8,
    callback_param: *const u8,
) -> BOOL {
    if h_file.is_null() || h_file == INVALID_HANDLE_VALUE {
        crate::teb::set_last_error(err::ERROR_INVALID_HANDLE);
        return FALSE;
    }
    if (dump_type & MINIDUMP_UNSUPPORTED_FLAGS) != 0 {
        crate::teb::set_last_error(err::ERROR_NOT_SUPPORTED);
        return FALSE;
    }
    if !callback_param.is_null() {
        let mut raw = [0u8; 8];
        if !crate::crash::safe_read(callback_param as u64, &mut raw) {
            crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
            return FALSE;
        }
        let cb_routine = usize::from_le_bytes(raw);
        if cb_routine != 0 {
            log::debug!("dbghelp: MiniDumpWriteDump with MINIDUMP_CALLBACK_INFORMATION is not supported");
            crate::teb::set_last_error(err::ERROR_NOT_SUPPORTED);
            return FALSE;
        }
    }
    let my_pid = std::process::id();
    let is_self_handle = h_process.is_null()
        || (h_process as isize) == -1
        || (h_process as usize) == (my_pid as usize)
        || crate::kernel32::process::GetProcessId(h_process) == my_pid;
    if !is_self_handle && process_id != 0 && process_id != my_pid {
        crate::teb::set_last_error(err::ERROR_ACCESS_DENIED);
        return FALSE;
    }

    // Read optional MINIDUMP_EXCEPTION_INFORMATION { ThreadId: u32, ExceptionPointers: ptr, ClientPointers: BOOL }
    // Note: minidumpapiset.h uses `#pragma pack(push, 4)` so offsets are +0 ThreadId(4), +4 ExceptionPointers(8), +12 ClientPointers(4).
    let mut exc_thread_id: Option<u32> = None;
    let mut exc_record: Option<ExceptionRecord> = None;
    let mut exc_context: Option<Context> = None;
    if !exception_param.is_null() {
        let tid = std::ptr::read_unaligned(exception_param as *const u32);
        let ep_ptr = std::ptr::read_unaligned(exception_param.add(4) as *const usize);
        if ep_ptr != 0 {
            let er_ptr = std::ptr::read_unaligned(ep_ptr as *const *const ExceptionRecord);
            let ctx_ptr =
                std::ptr::read_unaligned((ep_ptr + 8) as *const *const Context);
            if !er_ptr.is_null() {
                exc_record = Some(std::ptr::read_unaligned(er_ptr));
            }
            if !ctx_ptr.is_null() {
                exc_context = Some(std::ptr::read_unaligned(ctx_ptr));
            }
            exc_thread_id = Some(tid);
        }
    }

    // Collect caller user streams (`MINIDUMP_USER_STREAM_INFORMATION`, packed(4): +0 count(4), +4 array_ptr(8)).
    // Never silently truncate a stream or drop streams past the cap; fail honestly instead.
    const MAX_USER_STREAMS: usize = 64;
    const MAX_USER_STREAM_BYTES: usize = 16 * 1024 * 1024;
    const MAX_TOTAL_USER_STREAM_BYTES: usize = 64 * 1024 * 1024;
    let mut user_streams: Vec<(u32, Vec<u8>)> = Vec::new();
    if !user_stream_param.is_null() {
        let mut info_raw = [0u8; 12];
        if !crate::crash::safe_read(user_stream_param as u64, &mut info_raw) {
            crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
            return FALSE;
        }
        let count = u32::from_le_bytes([info_raw[0], info_raw[1], info_raw[2], info_raw[3]]) as usize;
        let arr_ptr = u64::from_le_bytes([
            info_raw[4],
            info_raw[5],
            info_raw[6],
            info_raw[7],
            info_raw[8],
            info_raw[9],
            info_raw[10],
            info_raw[11],
        ]);
        if count > 0 {
            if arr_ptr == 0 {
                crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
                return FALSE;
            }
            if count > MAX_USER_STREAMS {
                log::debug!(
                    "dbghelp: MiniDumpWriteDump user stream count {} exceeds limit {}",
                    count,
                    MAX_USER_STREAMS
                );
                crate::teb::set_last_error(err::ERROR_NOT_SUPPORTED);
                return FALSE;
            }
            let mut total_user_bytes = 0usize;
            // MINIDUMP_USER_STREAM (packed(4)): +0 Type(4), +4 BufferSize(4), +8 Buffer(8) = 16 bytes.
            for i in 0..count {
                let Some(elem_addr) = arr_ptr.checked_add((i as u64) * 16) else {
                    crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
                    return FALSE;
                };
                let mut elem = [0u8; 16];
                if !crate::crash::safe_read(elem_addr, &mut elem) {
                    crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
                    return FALSE;
                }
                let stype = u32::from_le_bytes([elem[0], elem[1], elem[2], elem[3]]);
                let sz = u32::from_le_bytes([elem[4], elem[5], elem[6], elem[7]]) as usize;
                let ptr = u64::from_le_bytes([
                    elem[8], elem[9], elem[10], elem[11], elem[12], elem[13], elem[14], elem[15],
                ]);
                if sz > MAX_USER_STREAM_BYTES {
                    log::debug!(
                        "dbghelp: MiniDumpWriteDump user stream {} size {} exceeds limit {}",
                        i,
                        sz,
                        MAX_USER_STREAM_BYTES
                    );
                    crate::teb::set_last_error(err::ERROR_NOT_SUPPORTED);
                    return FALSE;
                }
                total_user_bytes = match total_user_bytes.checked_add(sz) {
                    Some(t) if t <= MAX_TOTAL_USER_STREAM_BYTES => t,
                    _ => {
                        log::debug!(
                            "dbghelp: MiniDumpWriteDump total user stream size exceeds limit {}",
                            MAX_TOTAL_USER_STREAM_BYTES
                        );
                        crate::teb::set_last_error(err::ERROR_NOT_SUPPORTED);
                        return FALSE;
                    }
                };
                let mut data = Vec::new();
                if sz > 0 {
                    if ptr == 0 {
                        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
                        return FALSE;
                    }
                    data.resize(sz, 0);
                    if !crate::crash::safe_read(ptr, &mut data) {
                        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
                        return FALSE;
                    }
                }
                user_streams.push((stype, data));
            }
        }
    }

    // Capture current thread context if no exception context was supplied.
    let current_tid = crate::kernel32::threads::GetCurrentThreadId();
    let active_ctx = exc_context.unwrap_or_else(|| {
        if let Some(gc) = crate::crash::capture_guest_context() {
            gc
        } else {
            let mut c = Context::zeroed();
            crate::unwind::RtlCaptureContext(&mut c);
            c
        }
    });

    // Gather module list (loader modules + manually SymLoadModule64-registered modules).
    let mut modules: Vec<SymModule> = Vec::new();
    if let Some(loader) = crate::try_loader() {
        for (base, size, path) in loader.loaded_modules() {
            let mod_name = path.rsplit(['\\', '/']).next().unwrap_or(&path).to_string();
            modules.push(SymModule {
                base: base as u64,
                size,
                module_name: mod_name,
                image_name: path,
            });
        }
    }
    {
        let tbl = sym_table().lock().unwrap();
        if let Some(st) = tbl.get(&proc_key(h_process)) {
            for m in &st.modules {
                if !modules.iter().any(|existing| existing.base == m.base) {
                    modules.push(m.clone());
                }
            }
        }
    }

    // Gather thread entries: primary/crashing thread + any other live tracked Win32 threads.
    // For additional threads, capture real CPU context + stack via SIGUSR2 / Mach thread_get_state;
    // never fabricate a zeroed CONTEXT_FULL if a thread's state cannot be captured.
    let all_threads = crate::thread::all_threads();
    let mut thread_entries: Vec<(u32, u64, Option<(Context, Vec<u8>)>)> = Vec::new();
    let main_tid = exc_thread_id.unwrap_or(current_tid);
    let main_teb = if main_tid == current_tid {
        crate::teb::current() as u64
    } else {
        all_threads
            .iter()
            .find(|t| t.tid.load(Ordering::Relaxed) as u32 == main_tid)
            .map(|t| t.teb.load(Ordering::Relaxed) as u64)
            .unwrap_or(0)
    };
    let main_stack = safe_read_range(active_ctx.gpr[RSP], 64 * 1024);
    thread_entries.push((main_tid, main_teb, Some((active_ctx, main_stack))));
    for t in &all_threads {
        let tid = t.tid.load(Ordering::Relaxed) as u32;
        if tid != 0 && tid != main_tid && t.exit_code().is_none() {
            let teb = t.teb.load(Ordering::Relaxed) as u64;
            let pthread = t.pthread.load(Ordering::Relaxed);
            let mach_port = t.mach_port();
            let kernel_suspended = t.is_kernel_suspended();
            let captured = crate::crash::capture_thread_context_and_stack(
                pthread,
                mach_port,
                kernel_suspended,
                64 * 1024,
            );
            thread_entries.push((tid, teb, captured));
        }
    }

    // Stream directory count: ThreadList, ModuleList, MemoryList, SystemInfo, MiscInfo
    // + optional ExceptionStream + user streams.
    let has_exc = exc_record.is_some();
    let num_streams = 5u32 + u32::from(has_exc) + (user_streams.len() as u32);
    let now_unix = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as u32)
        .unwrap_or(0);

    let mut md = MdmpBuilder::new();
    // MINIDUMP_HEADER (32 bytes)
    md.push_u32(MINIDUMP_SIGNATURE);
    md.push_u32(MINIDUMP_VERSION);
    md.push_u32(num_streams);
    md.push_u32(32); // StreamDirectoryRva
    md.push_u32(0); // CheckSum
    md.push_u32(now_unix);
    md.push_u64(dump_type as u64);

    // Reserve MINIDUMP_DIRECTORY array (12 bytes per stream).
    let dir_base = md.buf.len();
    for _ in 0..num_streams {
        md.push_u32(0);
        md.push_u32(0);
        md.push_u32(0);
    }
    let mut dir_idx = 0usize;
    let mut write_dir = |md: &mut MdmpBuilder, stype: u32, size: u32, rva: u32| {
        let off = dir_base + dir_idx * 12;
        md.patch_u32(off, stype);
        md.patch_u32(off + 4, size);
        md.patch_u32(off + 8, rva);
        dir_idx += 1;
    };

    // 1. SystemInfoStream (7): 56 bytes + CSDVersion MINIDUMP_STRING
    let csd_rva = md.push_minidump_string("");
    md.align4();
    let sys_rva = md.rva();
    let ncpu = std::thread::available_parallelism()
        .map(|n| n.get().min(255) as u8)
        .unwrap_or(1);
    md.push_u16(9); // PROCESSOR_ARCHITECTURE_AMD64
    md.push_u16(6); // ProcessorLevel
    md.push_u16(0); // ProcessorRevision
    md.push_u8(ncpu);
    md.push_u8(1); // VER_NT_WORKSTATION
    md.push_u32(10); // MajorVersion
    md.push_u32(0); // MinorVersion
    md.push_u32(19045); // BuildNumber
    md.push_u32(2); // VER_PLATFORM_WIN32_NT
    md.push_u32(csd_rva);
    md.push_u16(0x0100); // VER_SUITE_SINGLEUSERTS
    md.push_u16(0); // Reserved2
    // CPU_INFORMATION union (24 bytes): OtherCpuInfo.ProcessorFeatures[2]
    md.push_u64(0);
    md.push_u64(0);
    md.push_u64(0);
    write_dir(&mut md, MINIDUMP_SYSTEM_INFO_STREAM, 56, sys_rva);

    // 2. MiscInfoStream (15): 24 bytes
    md.align4();
    let misc_rva = md.rva();
    md.push_u32(24); // SizeOfInfo
    md.push_u32(MINIDUMP_MISC1_PROCESS_ID);
    md.push_u32(if process_id != 0 { process_id } else { my_pid });
    md.push_u32(0);
    md.push_u32(0);
    md.push_u32(0);
    write_dir(&mut md, MINIDUMP_MISC_INFO_STREAM, 24, misc_rva);

    // 3. Pre-write module names and CodeView records, then ModuleListStream (4)
    let filter_paths = (dump_type & MINIDUMP_FILTER_MODULE_PATHS) != 0;
    let mut mod_meta: Vec<(u32, u32, u32, (u32, u32))> = Vec::with_capacity(modules.len());
    for m in &modules {
        let dumped_name = if filter_paths {
            filter_module_path(&m.image_name)
        } else {
            m.image_name.as_str()
        };
        let name_rva = md.push_minidump_string(dumped_name);
        let (ts, csum, cv) = read_pe_debug_metadata(m.base);
        let cv = cv.map(|bytes| {
            if filter_paths {
                filter_codeview_pdb_path(bytes)
            } else {
                bytes
            }
        });
        let cv_loc = match cv {
            Some(bytes) => {
                md.align4();
                md.push_bytes(&bytes)
            }
            None => (0, 0),
        };
        mod_meta.push((name_rva, ts, csum, cv_loc));
    }
    md.align4();
    let mod_list_rva = md.rva();
    md.push_u32(modules.len() as u32);
    for (m, &(name_rva, ts, csum, (cv_sz, cv_rva))) in modules.iter().zip(mod_meta.iter()) {
        // MINIDUMP_MODULE (108 bytes)
        md.push_u64(m.base);
        md.push_u32(m.size);
        md.push_u32(csum);
        md.push_u32(ts);
        md.push_u32(name_rva);
        // VS_FIXEDFILEINFO (52 bytes)
        md.push_u32(0xFEEF_04BD); // dwSignature
        md.push_u32(0x0001_0000); // dwStrucVersion
        for _ in 0..11 {
            md.push_u32(0);
        }
        // CvRecord (8 bytes)
        md.push_u32(cv_sz);
        md.push_u32(cv_rva);
        // MiscRecord (8 bytes)
        md.push_u32(0);
        md.push_u32(0);
        // Reserved0, Reserved1 (16 bytes)
        md.push_u64(0);
        md.push_u64(0);
    }
    let mod_list_sz = 4 + (modules.len() as u32) * 108;
    write_dir(&mut md, MINIDUMP_MODULE_LIST_STREAM, mod_list_sz, mod_list_rva);

    // 4. Write thread contexts + stack captures, then ThreadListStream (3)
    let mut mem_ranges: Vec<(u64, u32, u32)> = Vec::new();
    let mut thread_locs: Vec<(u32, u64, (u64, u32, u32), (u32, u32))> =
        Vec::with_capacity(thread_entries.len());
    let mut primary_ctx_loc = (0u32, 0u32);
    for (idx, (tid, teb, captured)) in thread_entries.iter().enumerate() {
        let (stack_loc, ctx_loc) = match captured {
            Some((ctx, stack_bytes)) => {
                md.align4();
                let ctx_bytes = std::slice::from_raw_parts(
                    ctx as *const Context as *const u8,
                    std::mem::size_of::<Context>(),
                );
                let ctx_loc = md.push_bytes(ctx_bytes);
                if idx == 0 {
                    primary_ctx_loc = ctx_loc;
                }
                let stack_start = ctx.gpr[RSP];
                let stack_loc = if !stack_bytes.is_empty() {
                    md.align4();
                    let (sz, rva) = md.push_bytes(stack_bytes);
                    mem_ranges.push((stack_start, sz, rva));
                    (stack_start, sz, rva)
                } else {
                    (0, 0, 0)
                };
                (stack_loc, ctx_loc)
            }
            None => ((0, 0, 0), (0, 0)),
        };
        thread_locs.push((*tid, *teb, stack_loc, ctx_loc));
    }
    md.align4();
    let thr_list_rva = md.rva();
    md.push_u32(thread_locs.len() as u32);
    for &(tid, teb, (st_start, st_sz, st_rva), (ctx_sz, ctx_rva)) in &thread_locs {
        // MINIDUMP_THREAD (48 bytes)
        md.push_u32(tid);
        md.push_u32(0); // SuspendCount
        md.push_u32(0x20); // PriorityClass (NORMAL_PRIORITY_CLASS)
        md.push_u32(0); // Priority
        md.push_u64(teb);
        // Stack: MINIDUMP_MEMORY_DESCRIPTOR (16 bytes)
        md.push_u64(st_start);
        md.push_u32(st_sz);
        md.push_u32(st_rva);
        // ThreadContext: MINIDUMP_LOCATION_DESCRIPTOR (8 bytes)
        md.push_u32(ctx_sz);
        md.push_u32(ctx_rva);
    }
    let thr_list_sz = 4 + (thread_locs.len() as u32) * 48;
    write_dir(&mut md, MINIDUMP_THREAD_LIST_STREAM, thr_list_sz, thr_list_rva);

    // 5. Optional code window around RIP + MemoryListStream (5)
    if active_ctx.rip >= 0x1000 {
        let code_start = active_ctx.rip.saturating_sub(64);
        let code_bytes = safe_read_range(code_start, 256);
        if !code_bytes.is_empty()
            && !mem_ranges
                .iter()
                .any(|&(s, sz, _)| code_start >= s && code_start < s.saturating_add(sz as u64))
        {
            md.align4();
            let (sz, rva) = md.push_bytes(&code_bytes);
            mem_ranges.push((code_start, sz, rva));
        }
    }
    md.align4();
    let mem_list_rva = md.rva();
    md.push_u32(mem_ranges.len() as u32);
    for &(start, sz, rva) in &mem_ranges {
        md.push_u64(start);
        md.push_u32(sz);
        md.push_u32(rva);
    }
    let mem_list_sz = 4 + (mem_ranges.len() as u32) * 16;
    write_dir(&mut md, MINIDUMP_MEMORY_LIST_STREAM, mem_list_sz, mem_list_rva);

    // 6. Optional ExceptionStream (6): 168 bytes
    if let Some(er) = exc_record {
        md.align4();
        let exc_rva = md.rva();
        md.push_u32(main_tid);
        md.push_u32(0); // __alignment
        md.push_u32(er.code);
        md.push_u32(er.flags);
        md.push_u64(er.record as u64);
        md.push_u64(er.address as u64);
        md.push_u32(er.number_parameters.min(15));
        md.push_u32(0); // __unusedAlignment
        for &info in &er.information {
            md.push_u64(info);
        }
        md.push_u32(primary_ctx_loc.0);
        md.push_u32(primary_ctx_loc.1);
        write_dir(&mut md, MINIDUMP_EXCEPTION_STREAM, 168, exc_rva);
    }

    // 7. Caller-provided UserStreams
    for (stype, data) in &user_streams {
        md.align4();
        let (sz, rva) = md.push_bytes(data);
        write_dir(&mut md, *stype, sz, rva);
    }

    // Write the entire MDMP file to `h_file`.
    let mut offset = 0usize;
    while offset < md.buf.len() {
        let chunk = (md.buf.len() - offset).min(u32::MAX as usize) as u32;
        let mut written = 0u32;
        let ok = crate::kernel32::file::WriteFile(
            h_file,
            md.buf.as_ptr().add(offset) as *const c_void,
            chunk,
            &mut written,
            std::ptr::null_mut(),
        );
        if ok == FALSE || written == 0 {
            return FALSE;
        }
        offset += written as usize;
    }
    TRUE
}

// ============================================================================
// MSF 7.0 / CodeView C13 PDB Line-Table Reader + SymGetLineFromAddr{64,W64}
// ============================================================================

const MSF7_MAGIC: &[u8; 32] = b"Microsoft C/C++ MSF 7.00\r\n\x1aDS\0\0\0";
const DEBUG_S_LINES: u32 = 0xF2;
const DEBUG_S_FILECHKSMS: u32 = 0xF4;

fn read_le_u16(b: &[u8], off: usize) -> Option<u16> {
    Some(u16::from_le_bytes(b.get(off..off + 2)?.try_into().ok()?))
}

fn read_le_u32(b: &[u8], off: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(off..off + 4)?.try_into().ok()?))
}

struct MsfFile<'a> {
    data: &'a [u8],
    block_size: usize,
    streams: Vec<(usize, Vec<u32>)>,
}

impl<'a> MsfFile<'a> {
    fn parse(data: &'a [u8]) -> Option<Self> {
        if data.len() < 56 || &data[0..32] != MSF7_MAGIC {
            return None;
        }
        let block_size = read_le_u32(data, 32)? as usize;
        if !(512..=65536).contains(&block_size) || !block_size.is_power_of_two() {
            return None;
        }
        let dir_bytes = read_le_u32(data, 44)? as usize;
        let block_map_addr = read_le_u32(data, 52)? as usize;
        let dir_blocks = dir_bytes.div_ceil(block_size);
        let map_off = block_map_addr.checked_mul(block_size)?;
        let mut dir_buf = Vec::with_capacity(dir_blocks * block_size);
        for i in 0..dir_blocks {
            let blk = read_le_u32(data, map_off + i * 4)? as usize;
            let start = blk.checked_mul(block_size)?;
            dir_buf.extend_from_slice(data.get(start..start + block_size)?);
        }
        dir_buf.truncate(dir_bytes);

        let num_streams = read_le_u32(&dir_buf, 0)? as usize;
        if num_streams > 65536 {
            return None;
        }
        let mut sizes = Vec::with_capacity(num_streams);
        let mut pos = 4usize;
        for _ in 0..num_streams {
            sizes.push(read_le_u32(&dir_buf, pos)?);
            pos += 4;
        }
        let mut streams = Vec::with_capacity(num_streams);
        for raw_sz in sizes {
            if raw_sz == u32::MAX || raw_sz == 0 {
                streams.push((0, Vec::new()));
                continue;
            }
            let sz = raw_sz as usize;
            let nblocks = sz.div_ceil(block_size);
            let mut blocks = Vec::with_capacity(nblocks);
            for _ in 0..nblocks {
                blocks.push(read_le_u32(&dir_buf, pos)?);
                pos += 4;
            }
            streams.push((sz, blocks));
        }
        Some(Self { data, block_size, streams })
    }

    fn read_stream(&self, idx: usize) -> Option<Vec<u8>> {
        let (sz, blocks) = self.streams.get(idx)?;
        if *sz == 0 {
            return Some(Vec::new());
        }
        let mut out = Vec::with_capacity(blocks.len() * self.block_size);
        for &blk in blocks {
            let start = (blk as usize).checked_mul(self.block_size)?;
            out.extend_from_slice(self.data.get(start..start + self.block_size)?);
        }
        out.truncate(*sz);
        Some(out)
    }
}

/// Find the `/names` string-table stream index in PDB stream 1 (PDB Info Stream).
fn parse_pdb_names_stream_index(pdb_info: &[u8]) -> Option<usize> {
    // Header: Version(4), Signature(4), Age(4), Guid(16) = 28 bytes.
    if pdb_info.len() < 32 {
        return None;
    }
    let names_buf_len = read_le_u32(pdb_info, 28)? as usize;
    let names_buf_start = 32usize;
    let names_buf_end = names_buf_start.checked_add(names_buf_len)?;
    let names_buf = pdb_info.get(names_buf_start..names_buf_end)?;

    // Serialized hash table: Size(4), Capacity(4), PresentBitVector, DeletedBitVector, (k, v) pairs.
    let mut pos = names_buf_end;
    let _size = read_le_u32(pdb_info, pos)?;
    pos += 4;
    let _cap = read_le_u32(pdb_info, pos)?;
    pos += 4;
    let present_words = read_le_u32(pdb_info, pos)? as usize;
    pos = pos.checked_add(4)?;
    if present_words > pdb_info.len().saturating_sub(pos) / 4 {
        return None;
    }
    let mut present_bits = Vec::with_capacity(present_words);
    for _ in 0..present_words {
        present_bits.push(read_le_u32(pdb_info, pos)?);
        pos = pos.checked_add(4)?;
    }
    let deleted_words = read_le_u32(pdb_info, pos)? as usize;
    pos = pos.checked_add(4)?.checked_add(deleted_words.checked_mul(4)?)?;

    let present_count: usize = present_bits.iter().map(|w| w.count_ones() as usize).sum();
    for _ in 0..present_count {
        let name_off = read_le_u32(pdb_info, pos)? as usize;
        let stream_idx = read_le_u32(pdb_info, pos + 4)? as usize;
        pos += 8;
        if let Some(tail) = names_buf.get(name_off..) {
            let end = tail.iter().position(|&b| b == 0).unwrap_or(tail.len());
            if &tail[..end] == b"/names" {
                return Some(stream_idx);
            }
        }
    }
    None
}

/// Read section virtual addresses (1-based `seg` -> RVA) from a mapped PE image.
fn read_pe_section_rvas(base: u64) -> Vec<u32> {
    let mut dos = [0u8; 64];
    if !crate::crash::safe_read(base, &mut dos) || &dos[0..2] != b"MZ" {
        return Vec::new();
    }
    let e_lfanew = u32::from_le_bytes([dos[60], dos[61], dos[62], dos[63]]) as u64;
    let mut coff = [0u8; 24];
    if !crate::crash::safe_read(base.wrapping_add(e_lfanew), &mut coff) || &coff[0..4] != b"PE\0\0" {
        return Vec::new();
    }
    let num_sec = u16::from_le_bytes([coff[6], coff[7]]) as usize;
    let opt_sz = u16::from_le_bytes([coff[20], coff[21]]) as u64;
    let sec_start = base.wrapping_add(e_lfanew).wrapping_add(24).wrapping_add(opt_sz);
    let mut rvas = Vec::with_capacity(num_sec.min(128));
    for i in 0..num_sec.min(128) {
        let mut hdr = [0u8; 40];
        if !crate::crash::safe_read(sec_start.wrapping_add(i as u64 * 40), &mut hdr) {
            break;
        }
        rvas.push(u32::from_le_bytes([hdr[12], hdr[13], hdr[14], hdr[15]]));
    }
    rvas
}

/// Parse an MSF 7.0 PDB file into a sorted RVA -> (file, line) cache.
/// When `expected_rsds_id` is `Some((guid, age))`, verifies that the PDB Info Stream (Stream 1)
/// GUID and age match before accepting the file.
fn parse_pdb_line_cache(
    pdb_bytes: &[u8],
    fallback_sec_rvas: &[u32],
    expected_rsds_id: Option<([u8; 16], u32)>,
) -> Option<PdbLineCache> {
    let msf = MsfFile::parse(pdb_bytes)?;
    let pdb_info = msf.read_stream(1)?;
    if let Some((expected_guid, expected_age)) = expected_rsds_id {
        if pdb_info.len() < 28 {
            return None;
        }
        let pdb_age = read_le_u32(&pdb_info, 8)?;
        let pdb_guid = &pdb_info[12..28];
        if pdb_guid != expected_guid || pdb_age != expected_age {
            return None;
        }
    }
    let names_stream_idx = parse_pdb_names_stream_index(&pdb_info)?;
    let names_stream = msf.read_stream(names_stream_idx)?;
    // `/names` header: Signature(0xEFFEEFFE), HashVersion(4), ByteSize(4), then NUL-terminated strings.
    if names_stream.len() < 12 || read_le_u32(&names_stream, 0)? != 0xEFFE_EFFE {
        return None;
    }
    let str_byte_len = read_le_u32(&names_stream, 8)? as usize;
    let str_table = names_stream.get(12..12usize.checked_add(str_byte_len)?)?;

    // Stream 3: DBI stream.
    let dbi = msf.read_stream(3)?;
    if dbi.len() < 64 {
        return None;
    }
    let mod_info_sz = read_le_u32(&dbi, 24)? as usize;
    let sec_contrib_sz = read_le_u32(&dbi, 28)? as usize;
    let sec_map_sz = read_le_u32(&dbi, 32)? as usize;
    let src_info_sz = read_le_u32(&dbi, 36)? as usize;
    let type_server_sz = read_le_u32(&dbi, 40)? as usize;
    let opt_dbg_sz = read_le_u32(&dbi, 48)? as usize;
    let ec_sz = read_le_u32(&dbi, 52)? as usize;

    // Optional Debug Header Substream: entry 5 = Section Header stream index (u16).
    let opt_dbg_off = 64usize
        .checked_add(mod_info_sz)?
        .checked_add(sec_contrib_sz)?
        .checked_add(sec_map_sz)?
        .checked_add(src_info_sz)?
        .checked_add(type_server_sz)?
        .checked_add(ec_sz)?;
    let mut sec_rvas: Vec<u32> = Vec::new();
    if opt_dbg_sz >= 12 {
        if let Some(off) = opt_dbg_off.checked_add(10) {
            if let Some(sec_hdr_idx) = read_le_u16(&dbi, off) {
                if sec_hdr_idx != 0xFFFF {
                    if let Some(sec_stream) = msf.read_stream(sec_hdr_idx as usize) {
                        for chunk in sec_stream.chunks_exact(40) {
                            sec_rvas.push(u32::from_le_bytes([chunk[12], chunk[13], chunk[14], chunk[15]]));
                        }
                    }
                }
            }
        }
    }
    if sec_rvas.is_empty() {
        sec_rvas.extend_from_slice(fallback_sec_rvas);
    }

    let mut file_map: HashMap<String, usize> = HashMap::new();
    let mut ansi_files: Vec<Box<[u8]>> = Vec::new();
    let mut wide_files: Vec<Box<[u16]>> = Vec::new();
    let mut lines: Vec<PdbLineEntry> = Vec::new();

    let mut intern_file = |name: String| -> usize {
        if let Some(&idx) = file_map.get(&name) {
            return idx;
        }
        let idx = ansi_files.len();
        let mut a = name.as_bytes().to_vec();
        a.push(0);
        ansi_files.push(a.into_boxed_slice());
        wide_files.push(crate::strings::to_wide_z(&name).into_boxed_slice());
        file_map.insert(name, idx);
        idx
    };

    // Walk Module Info Substream starting at DBI offset 64.
    // Validate the full declared substream against `dbi.len()` so malformed/truncated PDBs
    // are cleanly rejected instead of panicking during the NUL-terminated name scan.
    let mod_end = 64usize.checked_add(mod_info_sz)?;
    let mod_info = dbi.get(64..mod_end)?;
    let mut mpos = 0usize;
    while mpos + 64 <= mod_info.len() {
        let mod_stream = read_le_u16(mod_info, mpos + 34)?;
        let sym_sz = read_le_u32(mod_info, mpos + 36)? as usize;
        let c11_sz = read_le_u32(mod_info, mpos + 40)? as usize;
        let c13_sz = read_le_u32(mod_info, mpos + 44)? as usize;
        // Skip ModuleName and ObjFileName (two NUL-terminated strings after fixed 64-byte header), align to 4.
        let mut npos = mpos + 64;
        for _ in 0..2 {
            while let Some(&b) = mod_info.get(npos) {
                npos += 1;
                if b == 0 {
                    break;
                }
            }
        }
        mpos = npos.checked_add(3)? & !3;

        if mod_stream == 0xFFFF || c13_sz == 0 {
            continue;
        }
        let Some(ms) = msf.read_stream(mod_stream as usize) else {
            continue;
        };
        let c13_start = sym_sz.saturating_add(c11_sz);
        let Some(c13) = ms.get(c13_start..c13_start.saturating_add(c13_sz).min(ms.len())) else {
            continue;
        };

        // Pass 1: find DEBUG_S_FILECHKSMS subsection (`file_id` offset -> filename string).
        let mut chksms_slice: Option<&[u8]> = None;
        let mut spos = 0usize;
        while spos + 8 <= c13.len() {
            let stype = read_le_u32(c13, spos)?;
            let slen = read_le_u32(c13, spos + 4)? as usize;
            let body_start = spos + 8;
            let body_end = body_start.saturating_add(slen).min(c13.len());
            if stype == DEBUG_S_FILECHKSMS {
                chksms_slice = Some(&c13[body_start..body_end]);
                break;
            }
            spos = (body_end + 3) & !3;
        }
        let Some(chksms) = chksms_slice else {
            continue;
        };

        // Pass 2: parse all DEBUG_S_LINES subsections.
        spos = 0;
        while spos + 8 <= c13.len() {
            let stype = read_le_u32(c13, spos)?;
            let slen = read_le_u32(c13, spos + 4)? as usize;
            let body_start = spos + 8;
            let body_end = body_start.saturating_add(slen).min(c13.len());
            spos = (body_end + 3) & !3;
            if stype != DEBUG_S_LINES || body_end - body_start < 12 {
                continue;
            }
            let body = &c13[body_start..body_end];
            let off_con = read_le_u32(body, 0)?;
            let seg_con = read_le_u16(body, 4)?;
            let flags = read_le_u16(body, 6)?;
            let has_cols = (flags & 1) != 0;
            let sec_rva = if seg_con >= 1 {
                sec_rvas.get((seg_con - 1) as usize).copied().unwrap_or(0)
            } else {
                0
            };
            let mut bpos = 12usize;
            while bpos + 12 <= body.len() {
                let file_id = read_le_u32(body, bpos)? as usize;
                let n_lines = read_le_u32(body, bpos + 4)? as usize;
                let cb_block = read_le_u32(body, bpos + 8)? as usize;
                if cb_block < 12 || bpos + cb_block > body.len() {
                    break;
                }
                let file_name = read_le_u32(chksms, file_id)
                    .and_then(|name_off| {
                        let tail = str_table.get(name_off as usize..)?;
                        let nul = tail.iter().position(|&b| b == 0).unwrap_or(tail.len());
                        Some(String::from_utf8_lossy(&tail[..nul]).into_owned())
                    })
                    .unwrap_or_default();
                let file_idx = intern_file(file_name);
                let line_rec_start = bpos + 12;
                let max_lines = (cb_block - 12) / if has_cols { 12 } else { 8 };
                for li in 0..n_lines.min(max_lines) {
                    let lpos = line_rec_start + li * 8;
                    let off = read_le_u32(body, lpos)?;
                    let raw_line = read_le_u32(body, lpos + 4)?;
                    let line_num = raw_line & 0x00FF_FFFF;
                    let rva = sec_rva.wrapping_add(off_con).wrapping_add(off);
                    lines.push(PdbLineEntry { rva, line: line_num, file_idx });
                }
                bpos += cb_block;
            }
        }
    }

    if lines.is_empty() {
        return None;
    }
    lines.sort_by_key(|e| e.rva);
    Some(PdbLineCache { ansi_files, wide_files, lines })
}

fn push_unique_path(out: &mut Vec<PathBuf>, p: PathBuf) {
    if !p.as_os_str().is_empty() && !out.contains(&p) {
        out.push(p);
    }
}

fn looks_like_windows_path(p: &str) -> bool {
    let b = p.as_bytes();
    (b.len() >= 2 && b[1] == b':') || p.contains('\\')
}

/// Resolve a guest Windows path (via `crate::path::to_host`) and/or native host path into
/// candidate host `PathBuf`s.
fn push_guest_or_host_file(out: &mut Vec<PathBuf>, raw: &str) {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return;
    }
    let is_win = looks_like_windows_path(trimmed);
    if !is_win {
        push_unique_path(out, PathBuf::from(trimmed));
    }
    if let Some(host) = crate::path::to_host(trimmed) {
        push_unique_path(out, host);
    }
    if is_win {
        push_unique_path(out, PathBuf::from(trimmed));
    }
}

/// Resolve `dir + fname` where `dir` may be either a guest Windows directory (e.g. `C:\Symbols`
/// or a guest-CWD-relative directory) or a native host directory (e.g. `/tmp/syms`).
fn push_dir_file_candidates(out: &mut Vec<PathBuf>, dir: &str, fname: &str) {
    let dir = dir.trim();
    if dir.is_empty() || fname.is_empty() {
        return;
    }
    let is_win = looks_like_windows_path(dir);
    if !is_win {
        push_unique_path(out, PathBuf::from(dir).join(fname));
    }
    let dir_clean = dir.trim_end_matches(['\\', '/']);
    let joined_win = if dir_clean.len() == 2 && dir_clean.as_bytes()[1] == b':' {
        format!("{dir_clean}\\{fname}")
    } else if dir_clean.is_empty() {
        format!("\\{fname}")
    } else {
        format!("{dir_clean}\\{fname}")
    };
    if let Some(host_file) = crate::path::to_host(&joined_win) {
        push_unique_path(out, host_file);
    }
    if let Some(host_dir) = crate::path::to_host(dir) {
        push_unique_path(out, host_dir.join(fname));
    }
    if is_win {
        push_unique_path(out, PathBuf::from(dir).join(fname));
    }
}

/// Extract the parent directory of a Windows or POSIX path string.
fn parent_dir_str(p: &str) -> Option<&str> {
    let trimmed = p.trim();
    let idx = trimmed.rfind(['\\', '/'])?;
    if idx == 0 {
        Some(&trimmed[..1])
    } else if idx == 2 && trimmed.as_bytes()[1] == b':' {
        Some(&trimmed[..=idx])
    } else {
        Some(&trimmed[..idx])
    }
}

/// Locate a PDB file on disk for `m` using the PE's CodeView `RSDS` record and `search_path`.
fn load_pdb_for_module(m: &SymModule, search_path: &str) -> Option<PdbLineCache> {
    let mut candidates: Vec<PathBuf> = Vec::new();
    let mut expected_rsds_id: Option<([u8; 16], u32)> = None;
    let img_parent = parent_dir_str(&m.image_name);
    let (_, _, cv) = read_pe_debug_metadata(m.base);
    if let Some(ref cv_bytes) = cv {
        // RSDS: 'R','S','D','S'(4), GUID(16), Age(4), NUL-terminated UTF-8 path.
        if cv_bytes.len() > 24 && &cv_bytes[0..4] == b"RSDS" {
            let mut guid = [0u8; 16];
            guid.copy_from_slice(&cv_bytes[4..20]);
            let age = u32::from_le_bytes([cv_bytes[20], cv_bytes[21], cv_bytes[22], cv_bytes[23]]);
            expected_rsds_id = Some((guid, age));
            let tail = &cv_bytes[24..];
            let nul = tail.iter().position(|&b| b == 0).unwrap_or(tail.len());
            let pdb_raw = String::from_utf8_lossy(&tail[..nul]);
            if !pdb_raw.is_empty() {
                push_guest_or_host_file(&mut candidates, pdb_raw.as_ref());
                if let Some(fname) = pdb_raw.rsplit(['\\', '/']).next().filter(|s| !s.is_empty()) {
                    if let Some(parent) = img_parent {
                        push_dir_file_candidates(&mut candidates, parent, fname);
                    }
                    for dir in search_path.split(';').filter(|d| !d.trim().is_empty()) {
                        push_dir_file_candidates(&mut candidates, dir, fname);
                    }
                }
            }
        }
    }
    // Also check `<module_name>.pdb` beside the image and in every search-path directory.
    let stem = if m.module_name.len() > 4
        && (m.module_name[m.module_name.len() - 4..].eq_ignore_ascii_case(".dll")
            || m.module_name[m.module_name.len() - 4..].eq_ignore_ascii_case(".exe"))
    {
        &m.module_name[..m.module_name.len() - 4]
    } else {
        &m.module_name
    };
    let pdb_fname = format!("{stem}.pdb");
    if let Some(parent) = img_parent {
        push_dir_file_candidates(&mut candidates, parent, &pdb_fname);
    }
    for dir in search_path.split(';').filter(|d| !d.trim().is_empty()) {
        push_dir_file_candidates(&mut candidates, dir, &pdb_fname);
    }

    let sec_rvas = read_pe_section_rvas(m.base);
    for path in candidates {
        if let Ok(bytes) = std::fs::read(&path) {
            if let Some(cache) = parse_pdb_line_cache(&bytes, &sec_rvas, expected_rsds_id) {
                return Some(cache);
            }
        }
    }
    None
}

unsafe fn sym_get_line_impl(
    process: HANDLE,
    qw_addr: u64,
    displacement: *mut u32,
    line_out: *mut u8,
    wide: bool,
) -> BOOL {
    if line_out.is_null() {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let cb = std::ptr::read_unaligned(line_out as *const u32);
    if cb < 40 {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let Some(m) = find_sym_module(process, qw_addr) else {
        crate::teb::set_last_error(ERROR_MOD_NOT_FOUND);
        return FALSE;
    };
    let rva = qw_addr.saturating_sub(m.base) as u32;
    let key = proc_key(process);

    let mut tbl = sym_table().lock().unwrap();
    let st = tbl.entry(key).or_default();
    let search_path = st.search_path.clone();
    let cache = st
        .pdb_lines
        .entry(m.base)
        .or_insert_with(|| load_pdb_for_module(&m, &search_path));
    let Some(cache) = cache.as_ref() else {
        crate::teb::set_last_error(ERROR_INVALID_ADDRESS);
        return FALSE;
    };

    let idx = match cache.lines.binary_search_by_key(&rva, |e| e.rva) {
        Ok(i) => i,
        Err(0) => {
            crate::teb::set_last_error(ERROR_INVALID_ADDRESS);
            return FALSE;
        }
        Err(i) => i - 1,
    };
    let entry = &cache.lines[idx];
    let line_addr = m.base.saturating_add(entry.rva as u64);
    let disp = qw_addr.saturating_sub(line_addr) as u32;
    if !displacement.is_null() {
        std::ptr::write_unaligned(displacement, disp);
    }
    // IMAGEHLP_LINE64 / IMAGEHLP_LINEW64 (40 bytes on x64):
    // +0 SizeOfStruct(u32), +8 Key(ptr), +16 LineNumber(u32), +24 FileName(ptr), +32 Address(u64).
    std::ptr::write_unaligned(line_out.add(8) as *mut usize, 0);
    std::ptr::write_unaligned(line_out.add(16) as *mut u32, entry.line);
    let name_ptr = if wide {
        cache.wide_files[entry.file_idx].as_ptr() as usize
    } else {
        cache.ansi_files[entry.file_idx].as_ptr() as usize
    };
    std::ptr::write_unaligned(line_out.add(24) as *mut usize, name_ptr);
    std::ptr::write_unaligned(line_out.add(32) as *mut u64, line_addr);
    TRUE
}

pub unsafe extern "win64" fn SymGetLineFromAddr64(
    process: HANDLE,
    qw_addr: u64,
    pdw_displacement: *mut u32,
    line64: *mut u8,
) -> BOOL {
    sym_get_line_impl(process, qw_addr, pdw_displacement, line64, false)
}

pub unsafe extern "win64" fn SymGetLineFromAddrW64(
    process: HANDLE,
    qw_addr: u64,
    pdw_displacement: *mut u32,
    line64: *mut u8,
) -> BOOL {
    sym_get_line_impl(process, qw_addr, pdw_displacement, line64, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sym_lifecycle_search_path_module_and_addr_work() {
        unsafe {
            let proc = 0x1234usize as HANDLE;
            assert_eq!(SymInitialize(proc, c"C:\\Symbols".as_ptr() as *const u8, FALSE), TRUE);

            let mut path_buf = [0u8; 64];
            assert_eq!(SymGetSearchPath(proc, path_buf.as_mut_ptr(), 64), TRUE);
            assert_eq!(crate::strings::from_ansi(path_buf.as_ptr()), "C:\\Symbols");

            let new_w = crate::strings::to_wide_z("C:\\AltSymbols");
            assert_eq!(SymSetSearchPathW(proc, new_w.as_ptr()), TRUE);
            let mut wbuf = [0u16; 64];
            assert_eq!(SymGetSearchPathW(proc, wbuf.as_mut_ptr(), 64), TRUE);
            assert_eq!(crate::strings::from_wide(wbuf.as_ptr()), "C:\\AltSymbols");

            let base = SymLoadModule64(
                proc,
                std::ptr::null_mut(),
                c"C:\\Game\\app.exe".as_ptr() as *const u8,
                c"app".as_ptr() as *const u8,
                0x14000_0000,
                0x20000,
            );
            assert_eq!(base, 0x14000_0000);
            assert_eq!(SymGetModuleBase64(proc, 0x14000_1234), 0x14000_0000);

            let mut mod_info = vec![0u8; 600];
            (mod_info.as_mut_ptr() as *mut u32).write_unaligned(600);
            assert_eq!(SymGetModuleInfo64(proc, 0x14000_1234, mod_info.as_mut_ptr()), TRUE);
            assert_eq!((mod_info.as_ptr().add(8) as *const u64).read_unaligned(), 0x14000_0000);
            assert_eq!(crate::strings::from_ansi(mod_info.as_ptr().add(36)), "app");

            let mut sym = vec![0u8; 88 + 128];
            (sym.as_mut_ptr() as *mut u32).write_unaligned(88);
            (sym.as_mut_ptr().add(80) as *mut u32).write_unaligned(128);
            let mut disp = 0u64;
            assert_eq!(SymFromAddr(proc, 0x14000_1234, &mut disp, sym.as_mut_ptr()), TRUE);
            assert_eq!(disp, 0x1234);
            assert_eq!((sym.as_ptr().add(32) as *const u64).read_unaligned(), 0x14000_0000);

            assert_eq!(SymCleanup(proc), TRUE);
        }
    }

    #[test]
    fn stack_walk64_unwinds_pdata_and_leaf_frames() {
        unsafe {
            // Build a synthetic image buffer containing a leaf frame + two .pdata frames.
            // Image layout (base = img.as_ptr()):
            //   +0x100..+0x140: fn_outer (UWOP_ALLOC_SMALL 0x28 -> inf=4)
            //   +0x200..+0x240: fn_mid   (UWOP_ALLOC_SMALL 0x28 -> inf=4, UWOP_PUSH_NONVOL rbx)
            //   +0x300..+0x340: fn_leaf  (no .pdata; return address at [rsp])
            //   +0x400: UNWIND_INFO for fn_outer
            //   +0x420: UNWIND_INFO for fn_mid
            let mut img = vec![0x90u8; 0x1000];
            let base = img.as_ptr() as u64;

            // UNWIND_INFO for fn_outer at +0x400: version=1, prolog=4, count=1, fr=0
            //   code[0]: off=4, op=UWOP_ALLOC_SMALL(2) | (4 << 4) => allocates (4+1)*8 = 40 bytes
            img[0x400] = 1;
            img[0x401] = 4;
            img[0x402] = 1;
            img[0x403] = 0;
            img[0x404] = 4;
            img[0x405] = 2 | (4 << 4);

            // UNWIND_INFO for fn_mid at +0x420: version=1, prolog=6, count=2, fr=0
            //   code[0]: off=6, op=UWOP_ALLOC_SMALL(2) | (4 << 4) => +40 bytes
            //   code[1]: off=2, op=UWOP_PUSH_NONVOL(0) | (3 << 4) => pop rbx (+8 bytes)
            img[0x420] = 1;
            img[0x421] = 6;
            img[0x422] = 2;
            img[0x423] = 0;
            img[0x424] = 6;
            img[0x425] = 2 | (4 << 4);
            img[0x426] = 2;
            img[0x427] = 0 | (3 << 4);

            let rfs = [
                RuntimeFunction { begin_rva: 0x100, end_rva: 0x140, unwind_rva: 0x400 },
                RuntimeFunction { begin_rva: 0x200, end_rva: 0x240, unwind_rva: 0x420 },
            ];
            crate::unwind::RtlAddFunctionTable(rfs.as_ptr(), rfs.len() as u32, base);

            // Build synthetic x64 stack (grows downward; stack[idx] at sp + idx*8):
            //   idx 0: return address from fn_leaf -> fn_mid + 0x20
            //   idx 1..=5: 40-byte frame of fn_mid
            //   idx 6: saved rbx = 0xCAFE_BABE_1234_5678
            //   idx 7: return address from fn_mid -> fn_outer + 0x18
            //   idx 8..=12: 40-byte frame of fn_outer
            //   idx 13: 0 (end of call stack)
            let mut stack = [0u64; 16];
            stack[0] = base + 0x220;
            stack[6] = 0xCAFE_BABE_1234_5678;
            stack[7] = base + 0x118;
            stack[13] = 0;

            let mut ctx = Context::zeroed();
            ctx.rip = base + 0x310; // inside fn_leaf
            ctx.gpr[RSP] = stack.as_ptr() as u64;
            ctx.gpr[3] = 0x1111; // rbx before unwind

            let mut sf = StackFrame64::default();
            sf.addr_pc = flat_addr(ctx.rip);
            sf.addr_stack = flat_addr(ctx.gpr[RSP]);

            let mut pcs = Vec::new();
            while StackWalk64(
                IMAGE_FILE_MACHINE_AMD64,
                (-1isize) as HANDLE,
                (-2isize) as HANDLE,
                &mut sf as *mut StackFrame64 as *mut u8,
                &mut ctx as *mut Context as *mut u8,
                std::ptr::null(),
                SymFunctionTableAccess64 as *const () as *const c_void,
                SymGetModuleBase64 as *const () as *const c_void,
                std::ptr::null(),
            ) == TRUE
            {
                pcs.push(sf.addr_pc.offset);
            }

            crate::unwind::RtlDeleteFunctionTable(rfs.as_ptr());

            assert_eq!(pcs, vec![base + 0x310, base + 0x220, base + 0x118]);
            assert_eq!(ctx.gpr[3], 0xCAFE_BABE_1234_5678, "UWOP_PUSH_NONVOL must restore RBX");
        }
    }

    #[test]
    fn minidump_write_dump_emits_valid_mdmp_with_exception_and_user_stream() {
        unsafe {
            let tmp_dir = std::env::temp_dir().join(format!(
                "port_factory_mdmp_{}_{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
            std::fs::create_dir_all(&tmp_dir).unwrap();
            crate::path::add_mount("M:\\", &tmp_dir);
            let wpath = crate::strings::to_wide_z("M:\\crash.dmp");
            let h_file = crate::kernel32::file::CreateFileW(
                wpath.as_ptr(),
                0xC000_0000, // GENERIC_READ | GENERIC_WRITE
                0,
                std::ptr::null_mut(),
                2, // CREATE_ALWAYS
                0x80,
                std::ptr::null_mut(),
            );
            assert_ne!(h_file, INVALID_HANDLE_VALUE);

            let proc = (-1isize) as HANDLE;
            let _ = SymInitialize(proc, std::ptr::null(), FALSE);
            let _ = SymLoadModule64(
                proc,
                std::ptr::null_mut(),
                c"C:\\Test\\demo.exe".as_ptr() as *const u8,
                c"demo".as_ptr() as *const u8,
                0x14000_0000,
                0x10000,
            );

            // Build exception info and a custom user stream.
            let mut er: ExceptionRecord = std::mem::zeroed();
            er.code = 0xC000_0005;
            er.address = 0x14000_1234usize as *mut c_void;
            let mut ctx = Context::zeroed();
            ctx.context_flags = crate::unwind::CONTEXT_FULL;
            ctx.rip = 0x14000_1234;
            let stack_probe = [0xAAu8; 64];
            ctx.gpr[RSP] = stack_probe.as_ptr() as u64;

            let ep = [&er as *const ExceptionRecord as usize, &ctx as *const Context as usize];
            #[repr(C, packed(4))]
            struct ExcInfo {
                tid: u32,
                ep: usize,
                client_ptrs: i32,
            }
            let exc_info = ExcInfo { tid: 42, ep: ep.as_ptr() as usize, client_ptrs: 0 };

            let payload = b"hello-minidump-user-stream";
            #[repr(C, packed(4))]
            struct UserStream {
                stype: u32,
                sz: u32,
                buf: usize,
            }
            #[repr(C, packed(4))]
            struct UserStreamInfo {
                count: u32,
                arr: usize,
            }
            let us = UserStream {
                stype: 0x0001_0042,
                sz: payload.len() as u32,
                buf: payload.as_ptr() as usize,
            };
            let usi = UserStreamInfo { count: 1, arr: &us as *const UserStream as usize };

            assert_eq!(
                MiniDumpWriteDump(
                    proc,
                    std::process::id(),
                    h_file,
                    0, // MiniDumpNormal
                    &exc_info as *const ExcInfo as *const u8,
                    &usi as *const UserStreamInfo as *const u8,
                    std::ptr::null(),
                ),
                TRUE
            );
            let mut fsize: i64 = 0;
            assert_eq!(crate::kernel32::file::GetFileSizeEx(h_file, &mut fsize), TRUE);
            assert!(fsize > 32);
            let mut new_pos: i64 = 0;
            assert_eq!(crate::kernel32::file::SetFilePointerEx(h_file, 0, &mut new_pos, 0), TRUE);
            let mut bytes = vec![0u8; fsize as usize];
            let mut got = 0u32;
            assert_eq!(
                crate::kernel32::file::ReadFile(
                    h_file,
                    bytes.as_mut_ptr() as *mut c_void,
                    bytes.len() as u32,
                    &mut got,
                    std::ptr::null_mut(),
                ),
                TRUE
            );
            bytes.truncate(got as usize);
            crate::kernel32::threads::CloseHandle(h_file);
            let _ = crate::kernel32::file::DeleteFileW(wpath.as_ptr());
            let _ = std::fs::remove_dir_all(&tmp_dir);
            let _ = SymCleanup(proc);

            assert!(bytes.len() >= 32 + 7 * 12);
            assert_eq!(&bytes[0..4], b"MDMP");
            assert_eq!(read_le_u32(&bytes, 4).unwrap() & 0xFFFF, 0xA793);
            let n_streams = read_le_u32(&bytes, 8).unwrap();
            assert_eq!(n_streams, 7); // SysInfo, Misc, ModList, ThrList, MemList, Exception, UserStream
            let dir_rva = read_le_u32(&bytes, 12).unwrap() as usize;
            let mut seen = HashMap::new();
            for i in 0..n_streams as usize {
                let st = read_le_u32(&bytes, dir_rva + i * 12).unwrap();
                let sz = read_le_u32(&bytes, dir_rva + i * 12 + 4).unwrap();
                let rva = read_le_u32(&bytes, dir_rva + i * 12 + 8).unwrap();
                seen.insert(st, (sz, rva));
            }
            // Verify SystemInfo = AMD64 (9), Exception = 0xC0000005 @ 0x140001234, UserStream matches.
            let (_, sys_rva) = seen[&MINIDUMP_SYSTEM_INFO_STREAM];
            assert_eq!(read_le_u16(&bytes, sys_rva as usize).unwrap(), 9);
            let (_, exc_rva) = seen[&MINIDUMP_EXCEPTION_STREAM];
            assert_eq!(read_le_u32(&bytes, exc_rva as usize).unwrap(), 42);
            assert_eq!(read_le_u32(&bytes, exc_rva as usize + 8).unwrap(), 0xC000_0005);
            let (us_sz, us_rva) = seen[&0x0001_0042];
            assert_eq!(&bytes[us_rva as usize..(us_rva + us_sz) as usize], payload);
            assert!(seen.contains_key(&MINIDUMP_MODULE_LIST_STREAM));
            assert!(seen.contains_key(&MINIDUMP_THREAD_LIST_STREAM));
            assert!(seen.contains_key(&MINIDUMP_MEMORY_LIST_STREAM));
            assert!(seen.contains_key(&MINIDUMP_MISC_INFO_STREAM));
        }
    }

    /// Build a minimal valid MSF 7.0 PDB containing `/names`, DBI (with 1 module & 1 section
    /// header at RVA 0x1000), and C13 `DEBUG_S_FILECHKSMS` + `DEBUG_S_LINES` entries.
    fn build_minimal_pdb(src_file: &str, seg_offset: u32, line_num: u32) -> Vec<u8> {
        build_minimal_pdb_with_id(src_file, seg_offset, line_num, [0u8; 16], 1)
    }

    fn build_minimal_pdb_with_id(
        src_file: &str,
        seg_offset: u32,
        line_num: u32,
        guid: [u8; 16],
        age: u32,
    ) -> Vec<u8> {
        let block_size = 4096usize;

        // Stream 1: PDB info stream mapping "/names" -> stream 4
        let mut s1 = Vec::new();
        s1.extend_from_slice(&20000404u32.to_le_bytes()); // Version
        s1.extend_from_slice(&0u32.to_le_bytes()); // Sig
        s1.extend_from_slice(&age.to_le_bytes()); // Age
        s1.extend_from_slice(&guid); // Guid
        let key_str = b"/names\0";
        s1.extend_from_slice(&(key_str.len() as u32).to_le_bytes());
        s1.extend_from_slice(key_str);
        s1.extend_from_slice(&1u32.to_le_bytes()); // Size
        s1.extend_from_slice(&2u32.to_le_bytes()); // Capacity
        s1.extend_from_slice(&1u32.to_le_bytes()); // Present words = 1
        s1.extend_from_slice(&1u32.to_le_bytes()); // bit 0 set
        s1.extend_from_slice(&0u32.to_le_bytes()); // Deleted words = 0
        s1.extend_from_slice(&0u32.to_le_bytes()); // key offset = 0
        s1.extend_from_slice(&4u32.to_le_bytes()); // stream idx = 4

        // Stream 4: `/names` string table
        let mut s4 = Vec::new();
        s4.extend_from_slice(&0xEFFE_EFFEu32.to_le_bytes());
        s4.extend_from_slice(&1u32.to_le_bytes());
        let mut stbl = vec![0u8]; // offset 0 = ""
        let file_name_off = stbl.len() as u32;
        stbl.extend_from_slice(src_file.as_bytes());
        stbl.push(0);
        s4.extend_from_slice(&(stbl.len() as u32).to_le_bytes());
        s4.extend_from_slice(&stbl);

        // Stream 5: Section Header stream (1 IMAGE_SECTION_HEADER with VirtualAddress = 0x1000)
        let mut s5 = vec![0u8; 40];
        s5[0..5].copy_from_slice(b".text");
        s5[12..16].copy_from_slice(&0x1000u32.to_le_bytes());

        // Stream 6: Module stream with C13 DEBUG_S_FILECHKSMS + DEBUG_S_LINES
        let mut s6 = Vec::new();
        // Subsection 1: DEBUG_S_FILECHKSMS (0xF4): name_off(4), hash_len(1)=0, hash_type(1)=0, pad(2)
        s6.extend_from_slice(&DEBUG_S_FILECHKSMS.to_le_bytes());
        s6.extend_from_slice(&8u32.to_le_bytes());
        s6.extend_from_slice(&file_name_off.to_le_bytes());
        s6.extend_from_slice(&[0u8; 4]);
        // Subsection 2: DEBUG_S_LINES (0xF2):
        //   header(12): off_con(4), seg_con(2)=1, flags(2)=0, cb_con(4)=0x100
        //   file_block(12+8=20): file_id(4)=0, n_lines(4)=1, cb_block(4)=20, off(4)=0, line(4)
        s6.extend_from_slice(&DEBUG_S_LINES.to_le_bytes());
        s6.extend_from_slice(&32u32.to_le_bytes());
        s6.extend_from_slice(&seg_offset.to_le_bytes());
        s6.extend_from_slice(&1u16.to_le_bytes());
        s6.extend_from_slice(&0u16.to_le_bytes());
        s6.extend_from_slice(&0x100u32.to_le_bytes());
        s6.extend_from_slice(&0u32.to_le_bytes()); // file_id
        s6.extend_from_slice(&1u32.to_le_bytes()); // n_lines
        s6.extend_from_slice(&20u32.to_le_bytes()); // cb_block
        s6.extend_from_slice(&0u32.to_le_bytes()); // line offset
        s6.extend_from_slice(&line_num.to_le_bytes());

        // Stream 3: DBI stream
        let mut mod_info = vec![0u8; 64];
        mod_info[34..36].copy_from_slice(&6u16.to_le_bytes()); // module stream = 6
        mod_info[44..48].copy_from_slice(&(s6.len() as u32).to_le_bytes()); // c13_byte_size
        mod_info.extend_from_slice(b"test.obj\0test.obj\0");
        while (mod_info.len() & 3) != 0 {
            mod_info.push(0);
        }
        let mut opt_dbg = vec![0xFFu8; 12];
        opt_dbg[10..12].copy_from_slice(&5u16.to_le_bytes()); // section hdr stream = 5

        let mut s3 = vec![0u8; 64];
        s3[24..28].copy_from_slice(&(mod_info.len() as u32).to_le_bytes());
        s3[48..52].copy_from_slice(&(opt_dbg.len() as u32).to_le_bytes());
        s3.extend_from_slice(&mod_info);
        s3.extend_from_slice(&opt_dbg);

        let streams: [&[u8]; 7] = [&[], &s1, &[], &s3, &s4, &s5, &s6];

        // Allocate blocks: block 0 = superblock, 1 = block map, 2 = stream dir, 3.. = stream payloads.
        let mut stream_blocks: Vec<Vec<u32>> = Vec::new();
        let mut next_blk = 3u32;
        for st in &streams {
            let n = st.len().div_ceil(block_size);
            let mut blks = Vec::new();
            for _ in 0..n {
                blks.push(next_blk);
                next_blk += 1;
            }
            stream_blocks.push(blks);
        }

        let mut dir = Vec::new();
        dir.extend_from_slice(&(streams.len() as u32).to_le_bytes());
        for st in &streams {
            dir.extend_from_slice(&(st.len() as u32).to_le_bytes());
        }
        for blks in &stream_blocks {
            for &b in blks {
                dir.extend_from_slice(&b.to_le_bytes());
            }
        }
        assert!(dir.len() <= block_size);

        let total_blocks = next_blk as usize;
        let mut out = vec![0u8; total_blocks * block_size];
        out[0..32].copy_from_slice(MSF7_MAGIC);
        out[32..36].copy_from_slice(&(block_size as u32).to_le_bytes());
        out[40..44].copy_from_slice(&(total_blocks as u32).to_le_bytes());
        out[44..48].copy_from_slice(&(dir.len() as u32).to_le_bytes());
        out[52..56].copy_from_slice(&1u32.to_le_bytes()); // block_map_addr = block 1

        // Block 1: block map pointing to directory block 2
        out[block_size..block_size + 4].copy_from_slice(&2u32.to_le_bytes());
        // Block 2: stream directory
        out[2 * block_size..2 * block_size + dir.len()].copy_from_slice(&dir);
        // Stream payloads
        for (st, blks) in streams.iter().zip(stream_blocks.iter()) {
            for (i, &b) in blks.iter().enumerate() {
                let src = &st[i * block_size..((i + 1) * block_size).min(st.len())];
                let dst = (b as usize) * block_size;
                out[dst..dst + src.len()].copy_from_slice(src);
            }
        }
        out
    }

    #[test]
    fn sym_get_line_from_addr_resolves_pdb_and_reports_windows_errors() {
        unsafe {
            let dir = std::env::temp_dir().join(format!(
                "port_factory_pdb_{}",
                std::process::id()
            ));
            let _ = std::fs::create_dir_all(&dir);
            let pdb_path = dir.join("samplemod.pdb");
            let pdb_bytes = build_minimal_pdb("src\\engine.cpp", 0x240, 137);
            std::fs::write(&pdb_path, &pdb_bytes).unwrap();

            let proc = 0x5678usize as HANDLE;
            let sp = std::ffi::CString::new(dir.to_str().unwrap()).unwrap();
            assert_eq!(SymInitialize(proc, sp.as_ptr() as *const u8, FALSE), TRUE);
            let base = SymLoadModule64(
                proc,
                std::ptr::null_mut(),
                c"samplemod.dll".as_ptr() as *const u8,
                c"samplemod".as_ptr() as *const u8,
                0x2000_0000,
                0x10000,
            );
            assert_eq!(base, 0x2000_0000);

            // Exact hit + 0x10 byte displacement inside .text (RVA 0x1000 + 0x240 + 0x10).
            let mut line_a = [0u8; 40];
            (line_a.as_mut_ptr() as *mut u32).write_unaligned(40);
            let mut disp = 0u32;
            assert_eq!(
                SymGetLineFromAddr64(proc, base + 0x1250, &mut disp, line_a.as_mut_ptr()),
                TRUE
            );
            assert_eq!(disp, 0x10);
            assert_eq!((line_a.as_ptr().add(16) as *const u32).read_unaligned(), 137);
            let fname_a = (line_a.as_ptr().add(24) as *const *const u8).read_unaligned();
            assert_eq!(crate::strings::from_ansi(fname_a), "src\\engine.cpp");
            assert_eq!((line_a.as_ptr().add(32) as *const u64).read_unaligned(), base + 0x1240);

            // Wide variant returns UTF-16 filename and same metadata.
            let mut line_w = [0u8; 40];
            (line_w.as_mut_ptr() as *mut u32).write_unaligned(40);
            assert_eq!(
                SymGetLineFromAddrW64(proc, base + 0x1240, &mut disp, line_w.as_mut_ptr()),
                TRUE
            );
            assert_eq!(disp, 0);
            let fname_w = (line_w.as_ptr().add(24) as *const *const u16).read_unaligned();
            assert_eq!(crate::strings::from_wide(fname_w), "src\\engine.cpp");

            // Address outside any loaded module -> FALSE, ERROR_MOD_NOT_FOUND.
            assert_eq!(
                SymGetLineFromAddr64(proc, 0x9999_0000, &mut disp, line_a.as_mut_ptr()),
                FALSE
            );
            assert_eq!(crate::teb::last_error(), ERROR_MOD_NOT_FOUND);

            // Module with no PDB on disk -> FALSE, ERROR_INVALID_ADDRESS.
            let base2 = SymLoadModule64(
                proc,
                std::ptr::null_mut(),
                c"nopdb.dll".as_ptr() as *const u8,
                c"nopdb".as_ptr() as *const u8,
                0x3000_0000,
                0x10000,
            );
            assert_eq!(
                SymGetLineFromAddr64(proc, base2 + 0x100, &mut disp, line_a.as_mut_ptr()),
                FALSE
            );
            assert_eq!(crate::teb::last_error(), ERROR_INVALID_ADDRESS);

            assert_eq!(SymCleanup(proc), TRUE);
            let _ = std::fs::remove_file(&pdb_path);
            let _ = std::fs::remove_dir(&dir);
        }
    }

    #[test]
    fn minidump_write_dump_captures_real_context_and_stack_for_worker_thread() {
        unsafe extern "win64" fn worker_proc(param: *mut c_void) -> u32 {
            let pair = &*(param as *const (HANDLE, HANDLE));
            // Signal ready, then block on release event while MiniDumpWriteDump inspects us.
            crate::kernel32::threads::SetEvent(pair.0);
            crate::kernel32::threads::WaitForSingleObject(pair.1, 10_000);
            0
        }

        unsafe {
            let ev_ready = crate::kernel32::threads::CreateEventW(
                std::ptr::null_mut(),
                TRUE,
                FALSE,
                std::ptr::null(),
            );
            let ev_done = crate::kernel32::threads::CreateEventW(
                std::ptr::null_mut(),
                TRUE,
                FALSE,
                std::ptr::null(),
            );
            assert!(!ev_ready.is_null() && !ev_done.is_null());

            let pair = Box::new((ev_ready, ev_done));
            let mut worker_tid = 0u32;
            let h_worker = crate::kernel32::threads::CreateThread(
                std::ptr::null_mut(),
                0,
                Some(worker_proc),
                &*pair as *const (HANDLE, HANDLE) as *mut c_void,
                0,
                &mut worker_tid,
            );
            assert!(!h_worker.is_null());
            assert_ne!(worker_tid, 0);
            assert_eq!(crate::kernel32::threads::WaitForSingleObject(ev_ready, 5_000), 0);

            let tmp_dir = std::env::temp_dir().join(format!(
                "port_factory_mdmp_worker_{}_{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
            std::fs::create_dir_all(&tmp_dir).unwrap();
            crate::path::add_mount("N:\\", &tmp_dir);
            let wpath = crate::strings::to_wide_z("N:\\worker.dmp");
            let h_file = crate::kernel32::file::CreateFileW(
                wpath.as_ptr(),
                0xC000_0000,
                0,
                std::ptr::null_mut(),
                2,
                0x80,
                std::ptr::null_mut(),
            );
            assert_ne!(h_file, INVALID_HANDLE_VALUE);

            let ok = MiniDumpWriteDump(
                (-1isize) as HANDLE,
                std::process::id(),
                h_file,
                0,
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
            );
            assert_eq!(ok, TRUE);

            // Release worker thread cleanly before parsing dump.
            crate::kernel32::threads::SetEvent(ev_done);
            assert_eq!(crate::kernel32::threads::WaitForSingleObject(h_worker, 5_000), 0);
            crate::kernel32::threads::CloseHandle(h_worker);
            crate::kernel32::threads::CloseHandle(ev_ready);
            crate::kernel32::threads::CloseHandle(ev_done);
            drop(pair);

            let mut fsize: i64 = 0;
            assert_eq!(crate::kernel32::file::GetFileSizeEx(h_file, &mut fsize), TRUE);
            let mut new_pos: i64 = 0;
            assert_eq!(crate::kernel32::file::SetFilePointerEx(h_file, 0, &mut new_pos, 0), TRUE);
            let mut bytes = vec![0u8; fsize as usize];
            let mut got = 0u32;
            assert_eq!(
                crate::kernel32::file::ReadFile(
                    h_file,
                    bytes.as_mut_ptr() as *mut c_void,
                    bytes.len() as u32,
                    &mut got,
                    std::ptr::null_mut(),
                ),
                TRUE
            );
            bytes.truncate(got as usize);
            crate::kernel32::threads::CloseHandle(h_file);
            let _ = crate::kernel32::file::DeleteFileW(wpath.as_ptr());
            let _ = std::fs::remove_dir_all(&tmp_dir);

            // Locate ThreadListStream (3) and locate our worker_tid entry.
            let num_streams = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
            let dir_rva = u32::from_le_bytes(bytes[12..16].try_into().unwrap()) as usize;
            let mut thr_rva = None;
            for i in 0..num_streams {
                let off = dir_rva + i * 12;
                let stype = u32::from_le_bytes(bytes[off..off + 4].try_into().unwrap());
                if stype == MINIDUMP_THREAD_LIST_STREAM {
                    thr_rva = Some(u32::from_le_bytes(bytes[off + 8..off + 12].try_into().unwrap()) as usize);
                }
            }
            let rva = thr_rva.expect("ThreadListStream missing");
            let count = u32::from_le_bytes(bytes[rva..rva + 4].try_into().unwrap()) as usize;
            let mut found = false;
            for i in 0..count {
                let off = rva + 4 + i * 48;
                let tid = u32::from_le_bytes(bytes[off..off + 4].try_into().unwrap());
                if tid == worker_tid {
                    let stack_start = u64::from_le_bytes(bytes[off + 24..off + 32].try_into().unwrap());
                    let stack_sz = u32::from_le_bytes(bytes[off + 32..off + 36].try_into().unwrap());
                    let ctx_sz = u32::from_le_bytes(bytes[off + 40..off + 44].try_into().unwrap()) as usize;
                    let ctx_rva = u32::from_le_bytes(bytes[off + 44..off + 48].try_into().unwrap()) as usize;
                    assert_eq!(ctx_sz, std::mem::size_of::<Context>());
                    let ctx: Context = std::ptr::read_unaligned(bytes[ctx_rva..ctx_rva + ctx_sz].as_ptr() as *const Context);
                    assert_ne!(ctx.rip, 0, "worker thread RIP must be real, not zero");
                    assert_ne!(ctx.gpr[RSP], 0, "worker thread RSP must be real, not zero");
                    assert_eq!(stack_start, ctx.gpr[RSP]);
                    assert!(stack_sz > 0, "worker thread stack must be captured");
                    found = true;
                    break;
                }
            }
            assert!(found, "worker thread {worker_tid} missing from ThreadListStream");
        }
    }

    #[test]
    fn stack_walk64_uses_custom_read_memory_callback_for_non_leaf_unwind() {
        use std::sync::OnceLock;

        // Synthetic remote address space (unmapped in our own process) served ONLY via custom callbacks.
        // FunctionTableAccessRoutine returns a caller-local PVOID to a cached RUNTIME_FUNCTION, while
        // ReadProcessMemoryRoutine only knows target-process addresses (UNWIND_INFO + stack).
        const REMOTE_BASE: u64 = 0x7FFF_1234_0000;
        const REMOTE_STACK: u64 = 0x7FFE_5678_1000;
        const REMOTE_UNWIND_RVA: u32 = 0x5000;

        struct RemoteImage {
            rf: RuntimeFunction,
            unwind_info: [u8; 8],
            stack: [u64; 8],
        }
        static REMOTE: OnceLock<RemoteImage> = OnceLock::new();
        let _ = REMOTE.get_or_init(|| {
            // Non-leaf function [0x100..0x200): prolog pushes RBX (reg 3) then allocates 0x20 bytes.
            // Unwind codes (in reverse prolog order):
            //   code 0: off=8, UWOP_ALLOC_SMALL (2), info=3 ((3+1)*8 = 0x20) -> 0x32
            //   code 1: off=4, UWOP_PUSH_NONVOL (0), info=3 (RBX)             -> 0x30
            let rf = RuntimeFunction {
                begin_rva: 0x100,
                end_rva: 0x200,
                unwind_rva: REMOTE_UNWIND_RVA,
            };
            let unwind_info = [
                1,    // Version=1, Flags=0
                0x10, // SizeOfProlog
                2,    // CountOfCodes
                0,    // FrameRegister=0
                0x08, 0x32, // UWOP_ALLOC_SMALL 0x20
                0x04, 0x30, // UWOP_PUSH_NONVOL RBX
            ];
            let mut stack = [0u64; 8];
            // [RSP + 0x00..0x20] = local alloc; [RSP + 0x20] = saved RBX; [RSP + 0x28] = caller RIP.
            stack[4] = 0x1122_3344_5566_7788; // saved RBX
            stack[5] = REMOTE_BASE + 0x800;   // caller return RIP
            stack[6] = 0;                     // terminator
            RemoteImage { rf, unwind_info, stack }
        });

        unsafe extern "win64" fn remote_read_mem(
            _h_proc: HANDLE,
            base_addr: u64,
            buf: *mut c_void,
            size: u32,
            out_read: *mut u32,
        ) -> BOOL {
            let img = REMOTE.get().unwrap();
            let size_usize = size as usize;
            let dst = std::slice::from_raw_parts_mut(buf as *mut u8, size_usize);
            let stack_bytes = std::slice::from_raw_parts(
                img.stack.as_ptr() as *const u8,
                std::mem::size_of_val(&img.stack),
            );
            // Serve ONLY target-process regions; caller-local pointers (such as &img.rf)
            // must NOT be routed through ReadProcessMemoryRoutine.
            let regions: [(u64, &[u8]); 2] = [
                (REMOTE_BASE + REMOTE_UNWIND_RVA as u64, &img.unwind_info),
                (REMOTE_STACK, stack_bytes),
            ];
            for (start, data) in regions {
                let end = start + data.len() as u64;
                if base_addr >= start && base_addr.saturating_add(size as u64) <= end {
                    let off = (base_addr - start) as usize;
                    dst.copy_from_slice(&data[off..off + size_usize]);
                    if !out_read.is_null() {
                        std::ptr::write_unaligned(out_read, size);
                    }
                    return TRUE;
                }
            }
            if !out_read.is_null() {
                std::ptr::write_unaligned(out_read, 0);
            }
            FALSE
        }

        unsafe extern "win64" fn remote_fn_table(_h_proc: HANDLE, addr: u64) -> *mut c_void {
            if (REMOTE_BASE + 0x100..REMOTE_BASE + 0x200).contains(&addr) {
                let img = REMOTE.get().unwrap();
                &img.rf as *const RuntimeFunction as *mut c_void
            } else {
                std::ptr::null_mut()
            }
        }

        unsafe extern "win64" fn remote_mod_base(_h_proc: HANDLE, addr: u64) -> u64 {
            if (REMOTE_BASE..REMOTE_BASE + 0x10000).contains(&addr) {
                REMOTE_BASE
            } else {
                0
            }
        }

        unsafe {
            crate::register_native_va(remote_read_mem as *const () as u64, remote_read_mem as *const () as usize);
            crate::register_native_va(remote_fn_table as *const () as u64, remote_fn_table as *const () as usize);
            crate::register_native_va(remote_mod_base as *const () as u64, remote_mod_base as *const () as usize);

            let mut ctx = Context::zeroed();
            ctx.rip = REMOTE_BASE + 0x140;
            ctx.gpr[RSP] = REMOTE_STACK;
            ctx.gpr[3] = 0; // RBX before unwind

            let mut sf: StackFrame64 = std::mem::zeroed();
            sf.addr_pc = flat_addr(ctx.rip);
            sf.addr_stack = flat_addr(ctx.gpr[RSP]);
            sf.addr_frame = flat_addr(ctx.gpr[RSP]);

            // First call reports the starting non-leaf frame and peeks the caller RIP via remote reader.
            assert_eq!(
                StackWalk64(
                    IMAGE_FILE_MACHINE_AMD64,
                    0x1234usize as HANDLE,
                    std::ptr::null_mut(),
                    &mut sf as *mut StackFrame64 as *mut u8,
                    &mut ctx as *mut Context as *mut u8,
                    remote_read_mem as *const () as *const c_void,
                    remote_fn_table as *const () as *const c_void,
                    remote_mod_base as *const () as *const c_void,
                    std::ptr::null(),
                ),
                TRUE
            );
            assert_eq!(sf.addr_pc.offset, REMOTE_BASE + 0x140);
            assert_eq!(sf.addr_return.offset, REMOTE_BASE + 0x800);

            // Second call advances to the caller frame and restores RBX from the remote stack.
            assert_eq!(
                StackWalk64(
                    IMAGE_FILE_MACHINE_AMD64,
                    0x1234usize as HANDLE,
                    std::ptr::null_mut(),
                    &mut sf as *mut StackFrame64 as *mut u8,
                    &mut ctx as *mut Context as *mut u8,
                    remote_read_mem as *const () as *const c_void,
                    remote_fn_table as *const () as *const c_void,
                    remote_mod_base as *const () as *const c_void,
                    std::ptr::null(),
                ),
                TRUE
            );
            assert_eq!(sf.addr_pc.offset, REMOTE_BASE + 0x800);
            assert_eq!(ctx.gpr[3], 0x1122_3344_5566_7788, "custom reader must restore remote RBX");
        }
    }

    #[test]
    fn sym_get_line_from_addr_verifies_pdb_guid_and_age_against_rsds() {
        unsafe {
            let good_guid = [
                0x10, 0x21, 0x32, 0x43, 0x54, 0x65, 0x76, 0x87, 0x98, 0xA9, 0xBA, 0xCB, 0xDC,
                0xED, 0xFE, 0x0F,
            ];
            let stale_guid = [0xAA; 16];
            let good_age = 3u32;

            // Build a minimal in-memory PE32+ header with an IMAGE_DIRECTORY_ENTRY_DEBUG RSDS record.
            let mut pe = vec![0u8; 0x600];
            pe[0..2].copy_from_slice(b"MZ");
            pe[60..64].copy_from_slice(&0x80u32.to_le_bytes()); // e_lfanew = 0x80
            let nt = 0x80usize;
            pe[nt..nt + 4].copy_from_slice(b"PE\0\0");
            pe[nt + 24..nt + 26].copy_from_slice(&0x20Bu16.to_le_bytes()); // PE32+
            // NumberOfRvaAndSizes at opt+108 = nt+132
            pe[nt + 132..nt + 136].copy_from_slice(&16u32.to_le_bytes());
            // DataDirectory[6] (Debug) at nt + 24 + 112 + 48 = nt + 184
            pe[nt + 184..nt + 188].copy_from_slice(&0x300u32.to_le_bytes()); // DebugDir RVA
            pe[nt + 188..nt + 192].copy_from_slice(&28u32.to_le_bytes()); // DebugDir Size
            // IMAGE_DEBUG_DIRECTORY at RVA 0x300
            let dbg = 0x300usize;
            pe[dbg + 12..dbg + 16].copy_from_slice(&2u32.to_le_bytes()); // IMAGE_DEBUG_TYPE_CODEVIEW
            let rsds_name = b"matchmod.pdb\0";
            let cv_len = (24 + rsds_name.len()) as u32;
            pe[dbg + 16..dbg + 20].copy_from_slice(&cv_len.to_le_bytes()); // SizeOfData
            pe[dbg + 20..dbg + 24].copy_from_slice(&0x400u32.to_le_bytes()); // AddressOfRawData
            // RSDS record at RVA 0x400
            let rsds = 0x400usize;
            pe[rsds..rsds + 4].copy_from_slice(b"RSDS");
            pe[rsds + 4..rsds + 20].copy_from_slice(&good_guid);
            pe[rsds + 20..rsds + 24].copy_from_slice(&good_age.to_le_bytes());
            pe[rsds + 24..rsds + 24 + rsds_name.len()].copy_from_slice(rsds_name);

            let uniq = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let dir_stale = std::env::temp_dir().join(format!("pf_pdb_stale_{uniq}"));
            let dir_wrong_age = std::env::temp_dir().join(format!("pf_pdb_age_{uniq}"));
            let dir_good = std::env::temp_dir().join(format!("pf_pdb_good_{uniq}"));
            std::fs::create_dir_all(&dir_stale).unwrap();
            std::fs::create_dir_all(&dir_wrong_age).unwrap();
            std::fs::create_dir_all(&dir_good).unwrap();

            // dir_stale has wrong GUID, dir_wrong_age has right GUID but wrong age, dir_good matches both.
            std::fs::write(
                dir_stale.join("matchmod.pdb"),
                build_minimal_pdb_with_id("src\\stale.cpp", 0x100, 999, stale_guid, good_age),
            )
            .unwrap();
            std::fs::write(
                dir_wrong_age.join("matchmod.pdb"),
                build_minimal_pdb_with_id("src\\oldage.cpp", 0x100, 888, good_guid, good_age + 1),
            )
            .unwrap();
            std::fs::write(
                dir_good.join("matchmod.pdb"),
                build_minimal_pdb_with_id("src\\matched.cpp", 0x100, 42, good_guid, good_age),
            )
            .unwrap();

            let pe_base = pe.as_ptr() as u64;

            // 1. Search path with stale -> wrong-age -> good must skip mismatched PDBs and find line 42.
            let proc_ok = 0x7771usize as HANDLE;
            let sp_ok = std::ffi::CString::new(format!(
                "{};{};{}",
                dir_stale.display(),
                dir_wrong_age.display(),
                dir_good.display()
            ))
            .unwrap();
            assert_eq!(SymInitialize(proc_ok, sp_ok.as_ptr() as *const u8, FALSE), TRUE);
            assert_eq!(
                SymLoadModule64(
                    proc_ok,
                    std::ptr::null_mut(),
                    c"matchmod.dll".as_ptr() as *const u8,
                    c"matchmod".as_ptr() as *const u8,
                    pe_base,
                    0x10000,
                ),
                pe_base
            );
            let mut line = [0u8; 40];
            (line.as_mut_ptr() as *mut u32).write_unaligned(40);
            let mut disp = 0u32;
            assert_eq!(
                SymGetLineFromAddr64(proc_ok, pe_base + 0x1100, &mut disp, line.as_mut_ptr()),
                TRUE
            );
            assert_eq!((line.as_ptr().add(16) as *const u32).read_unaligned(), 42);
            let fname = (line.as_ptr().add(24) as *const *const u8).read_unaligned();
            assert_eq!(crate::strings::from_ansi(fname), "src\\matched.cpp");
            assert_eq!(SymCleanup(proc_ok), TRUE);

            // 2. Search path with ONLY mismatched PDBs must reject them cleanly (ERROR_INVALID_ADDRESS).
            let proc_bad = 0x7772usize as HANDLE;
            let sp_bad = std::ffi::CString::new(format!(
                "{};{}",
                dir_stale.display(),
                dir_wrong_age.display()
            ))
            .unwrap();
            assert_eq!(SymInitialize(proc_bad, sp_bad.as_ptr() as *const u8, FALSE), TRUE);
            assert_eq!(
                SymLoadModule64(
                    proc_bad,
                    std::ptr::null_mut(),
                    c"matchmod.dll".as_ptr() as *const u8,
                    c"matchmod".as_ptr() as *const u8,
                    pe_base,
                    0x10000,
                ),
                pe_base
            );
            assert_eq!(
                SymGetLineFromAddr64(proc_bad, pe_base + 0x1100, &mut disp, line.as_mut_ptr()),
                FALSE
            );
            assert_eq!(crate::teb::last_error(), ERROR_INVALID_ADDRESS);

            // Updating the search path via SymSetSearchPath must invalidate the negative cache
            // so retrying SymGetLineFromAddr64 on the same session resolves the matching PDB.
            let sp_fixed = std::ffi::CString::new(dir_good.to_str().unwrap()).unwrap();
            assert_eq!(SymSetSearchPath(proc_bad, sp_fixed.as_ptr() as *const u8), TRUE);
            assert_eq!(
                SymGetLineFromAddr64(proc_bad, pe_base + 0x1100, &mut disp, line.as_mut_ptr()),
                TRUE
            );
            assert_eq!((line.as_ptr().add(16) as *const u32).read_unaligned(), 42);
            assert_eq!(SymCleanup(proc_bad), TRUE);

            let _ = std::fs::remove_dir_all(&dir_stale);
            let _ = std::fs::remove_dir_all(&dir_wrong_age);
            let _ = std::fs::remove_dir_all(&dir_good);
        }
    }

    #[test]
    fn minidump_rejects_full_memory_and_uses_exception_thread_teb() {
        use std::sync::atomic::AtomicU64;

        unsafe extern "win64" fn worker_proc(param: *mut c_void) -> u32 {
            let state = &*(param as *const (HANDLE, HANDLE, AtomicU64));
            state.2.store(crate::teb::current() as u64, Ordering::Release);
            crate::kernel32::threads::SetEvent(state.0);
            crate::kernel32::threads::WaitForSingleObject(state.1, 10_000);
            0
        }

        unsafe {
            let ev_ready = crate::kernel32::threads::CreateEventW(
                std::ptr::null_mut(),
                TRUE,
                FALSE,
                std::ptr::null(),
            );
            let ev_done = crate::kernel32::threads::CreateEventW(
                std::ptr::null_mut(),
                TRUE,
                FALSE,
                std::ptr::null(),
            );
            let state = Box::new((ev_ready, ev_done, AtomicU64::new(0)));
            let mut worker_tid = 0u32;
            let h_worker = crate::kernel32::threads::CreateThread(
                std::ptr::null_mut(),
                0,
                Some(worker_proc),
                &*state as *const (HANDLE, HANDLE, AtomicU64) as *mut c_void,
                0,
                &mut worker_tid,
            );
            assert!(!h_worker.is_null());
            assert_eq!(crate::kernel32::threads::WaitForSingleObject(ev_ready, 5_000), 0);
            let worker_teb = state.2.load(Ordering::Acquire);
            let caller_teb = crate::teb::current() as u64;
            assert_ne!(worker_teb, 0);
            assert_ne!(worker_teb, caller_teb);

            let uniq = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let tmp_dir = std::env::temp_dir().join(format!("pf_mdmp_teb_{uniq}"));
            std::fs::create_dir_all(&tmp_dir).unwrap();
            crate::path::add_mount("O:\\", &tmp_dir);
            let wpath = crate::strings::to_wide_z("O:\\exc.dmp");
            let h_file = crate::kernel32::file::CreateFileW(
                wpath.as_ptr(),
                0xC000_0000,
                0,
                std::ptr::null_mut(),
                2,
                0x80,
                std::ptr::null_mut(),
            );
            assert_ne!(h_file, INVALID_HANDLE_VALUE);

            // 1. MiniDumpWithFullMemory must fail honestly with ERROR_NOT_SUPPORTED.
            assert_eq!(
                MiniDumpWriteDump(
                    (-1isize) as HANDLE,
                    std::process::id(),
                    h_file,
                    MINIDUMP_WITH_FULL_MEMORY,
                    std::ptr::null(),
                    std::ptr::null(),
                    std::ptr::null(),
                ),
                FALSE
            );
            assert_eq!(crate::teb::last_error(), err::ERROR_NOT_SUPPORTED);

            // 1b. Non-null MINIDUMP_CALLBACK_INFORMATION with CallbackRoutine != NULL must
            //     fail honestly with ERROR_NOT_SUPPORTED instead of silently ignoring callbacks.
            #[repr(C)]
            struct CallbackInfo {
                callback_routine: usize,
                callback_param: usize,
            }
            let cb_info = CallbackInfo {
                callback_routine: 0x14000_1234,
                callback_param: 0,
            };
            assert_eq!(
                MiniDumpWriteDump(
                    (-1isize) as HANDLE,
                    std::process::id(),
                    h_file,
                    0,
                    std::ptr::null(),
                    std::ptr::null(),
                    &cb_info as *const CallbackInfo as *const u8,
                ),
                FALSE
            );
            assert_eq!(crate::teb::last_error(), err::ERROR_NOT_SUPPORTED);

            // 1c. Oversized user stream (>16 MiB), too many user streams (>64), or
            //     non-empty user stream with NULL buffer must fail honestly instead of
            //     silently truncating or dropping caller diagnostic data.
            #[repr(C, packed(4))]
            struct UserStream {
                stype: u32,
                size: u32,
                buf: u64,
            }
            #[repr(C, packed(4))]
            struct UserStreamInfo {
                count: u32,
                array: u64,
            }
            let tiny = [0xABu8; 4];
            let oversized_stream = UserStream {
                stype: 0x0001_0001,
                size: (16 * 1024 * 1024) + 1,
                buf: tiny.as_ptr() as u64,
            };
            let oversized_info = UserStreamInfo {
                count: 1,
                array: &oversized_stream as *const UserStream as u64,
            };
            assert_eq!(
                MiniDumpWriteDump(
                    (-1isize) as HANDLE,
                    std::process::id(),
                    h_file,
                    0,
                    std::ptr::null(),
                    &oversized_info as *const UserStreamInfo as *const u8,
                    std::ptr::null(),
                ),
                FALSE
            );
            assert_eq!(crate::teb::last_error(), err::ERROR_NOT_SUPPORTED);

            let many_streams: Vec<UserStream> = (0..65)
                .map(|i| UserStream {
                    stype: 0x0001_0000 + i,
                    size: tiny.len() as u32,
                    buf: tiny.as_ptr() as u64,
                })
                .collect();
            let many_info = UserStreamInfo {
                count: many_streams.len() as u32,
                array: many_streams.as_ptr() as u64,
            };
            assert_eq!(
                MiniDumpWriteDump(
                    (-1isize) as HANDLE,
                    std::process::id(),
                    h_file,
                    0,
                    std::ptr::null(),
                    &many_info as *const UserStreamInfo as *const u8,
                    std::ptr::null(),
                ),
                FALSE
            );
            assert_eq!(crate::teb::last_error(), err::ERROR_NOT_SUPPORTED);

            let null_buf_stream = UserStream {
                stype: 0x0001_0002,
                size: 16,
                buf: 0,
            };
            let null_buf_info = UserStreamInfo {
                count: 1,
                array: &null_buf_stream as *const UserStream as u64,
            };
            assert_eq!(
                MiniDumpWriteDump(
                    (-1isize) as HANDLE,
                    std::process::id(),
                    h_file,
                    0,
                    std::ptr::null(),
                    &null_buf_info as *const UserStreamInfo as *const u8,
                    std::ptr::null(),
                ),
                FALSE
            );
            assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_PARAMETER);

            // 2. Dumping an exception for `worker_tid` from the caller thread must record
            //    `worker_teb` (not `caller_teb`) in the primary MINIDUMP_THREAD entry.
            let mut er: ExceptionRecord = std::mem::zeroed();
            er.code = 0xC000_0005;
            let mut ctx = Context::zeroed();
            ctx.context_flags = crate::unwind::CONTEXT_FULL;
            ctx.rip = 0x14000_2000;
            let ep = [&er as *const ExceptionRecord as usize, &ctx as *const Context as usize];
            #[repr(C, packed(4))]
            struct ExcInfo {
                tid: u32,
                ep: usize,
                client_ptrs: i32,
            }
            let exc_info = ExcInfo { tid: worker_tid, ep: ep.as_ptr() as usize, client_ptrs: 0 };
            assert_eq!(
                MiniDumpWriteDump(
                    (-1isize) as HANDLE,
                    std::process::id(),
                    h_file,
                    0,
                    &exc_info as *const ExcInfo as *const u8,
                    std::ptr::null(),
                    std::ptr::null(),
                ),
                TRUE
            );

            crate::kernel32::threads::SetEvent(ev_done);
            assert_eq!(crate::kernel32::threads::WaitForSingleObject(h_worker, 5_000), 0);
            crate::kernel32::threads::CloseHandle(h_worker);
            crate::kernel32::threads::CloseHandle(ev_ready);
            crate::kernel32::threads::CloseHandle(ev_done);
            drop(state);

            let mut fsize: i64 = 0;
            assert_eq!(crate::kernel32::file::GetFileSizeEx(h_file, &mut fsize), TRUE);
            let mut new_pos: i64 = 0;
            assert_eq!(crate::kernel32::file::SetFilePointerEx(h_file, 0, &mut new_pos, 0), TRUE);
            let mut bytes = vec![0u8; fsize as usize];
            let mut got = 0u32;
            assert_eq!(
                crate::kernel32::file::ReadFile(
                    h_file,
                    bytes.as_mut_ptr() as *mut c_void,
                    bytes.len() as u32,
                    &mut got,
                    std::ptr::null_mut(),
                ),
                TRUE
            );
            bytes.truncate(got as usize);
            crate::kernel32::threads::CloseHandle(h_file);
            let _ = crate::kernel32::file::DeleteFileW(wpath.as_ptr());
            let _ = std::fs::remove_dir_all(&tmp_dir);

            let num_streams = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
            let dir_rva = u32::from_le_bytes(bytes[12..16].try_into().unwrap()) as usize;
            let thr_rva = (0..num_streams)
                .find_map(|i| {
                    let off = dir_rva + i * 12;
                    let stype = u32::from_le_bytes(bytes[off..off + 4].try_into().unwrap());
                    (stype == MINIDUMP_THREAD_LIST_STREAM)
                        .then(|| u32::from_le_bytes(bytes[off + 8..off + 12].try_into().unwrap()) as usize)
                })
                .expect("ThreadListStream missing");
            let primary_tid = u32::from_le_bytes(bytes[thr_rva + 4..thr_rva + 8].try_into().unwrap());
            let primary_teb = u64::from_le_bytes(bytes[thr_rva + 20..thr_rva + 28].try_into().unwrap());
            assert_eq!(primary_tid, worker_tid);
            assert_eq!(primary_teb, worker_teb, "primary MINIDUMP_THREAD must carry exception thread's TEB");
        }
    }

    #[test]
    fn sym_get_line_from_addr_resolves_guest_windows_search_and_image_paths() {
        let uniq = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let sym_root = std::env::temp_dir().join(format!("pf_pdb_win_sym_{uniq}"));
        let sym_dir = sym_root.join("Symbols");
        let img_root = std::env::temp_dir().join(format!("pf_pdb_win_img_{uniq}"));
        let img_dir = img_root.join("Bin");
        std::fs::create_dir_all(&sym_dir).unwrap();
        std::fs::create_dir_all(&img_dir).unwrap();

        let guid: [u8; 16] = [
            0x21, 0x32, 0x43, 0x54, 0x65, 0x76, 0x87, 0x98,
            0xA9, 0xBA, 0xCB, 0xDC, 0xED, 0xFE, 0x0F, 0x10,
        ];
        let age: u32 = 1;
        let pdb_sym = build_minimal_pdb_with_id("src\\win_search.cpp", 0x100, 77, guid, age);
        let pdb_img = build_minimal_pdb_with_id("src\\win_adjacent.cpp", 0x200, 88, guid, age);
        std::fs::write(sym_dir.join("winsearch.pdb"), &pdb_sym).unwrap();
        std::fs::write(img_dir.join("winadj.pdb"), &pdb_img).unwrap();

        crate::path::add_mount("S:\\", &sym_root);
        crate::path::add_mount("G:\\", &img_root);

        let make_pe = |rsds_name: &[u8]| -> Vec<u8> {
            let mut pe = vec![0u8; 0x600];
            pe[0..2].copy_from_slice(b"MZ");
            pe[60..64].copy_from_slice(&0x80u32.to_le_bytes());
            let nt = 0x80usize;
            pe[nt..nt + 4].copy_from_slice(b"PE\0\0");
            pe[nt + 24..nt + 26].copy_from_slice(&0x20Bu16.to_le_bytes());
            pe[nt + 132..nt + 136].copy_from_slice(&16u32.to_le_bytes());
            pe[nt + 184..nt + 188].copy_from_slice(&0x300u32.to_le_bytes());
            pe[nt + 188..nt + 192].copy_from_slice(&28u32.to_le_bytes());
            let dbg = 0x300usize;
            pe[dbg + 12..dbg + 16].copy_from_slice(&2u32.to_le_bytes());
            let cv_len = (24 + rsds_name.len()) as u32;
            pe[dbg + 16..dbg + 20].copy_from_slice(&cv_len.to_le_bytes());
            pe[dbg + 20..dbg + 24].copy_from_slice(&0x400u32.to_le_bytes());
            let rsds = 0x400usize;
            pe[rsds..rsds + 4].copy_from_slice(b"RSDS");
            pe[rsds + 4..rsds + 20].copy_from_slice(&guid);
            pe[rsds + 20..rsds + 24].copy_from_slice(&age.to_le_bytes());
            pe[rsds + 24..rsds + 24 + rsds_name.len()].copy_from_slice(rsds_name);
            pe
        };
        let pe_search = make_pe(b"X:\\missing\\winsearch.pdb\0");
        let pe_adj = make_pe(b"X:\\missing\\winadj.pdb\0");
        let base_search = pe_search.as_ptr() as u64;
        let base_adj = pe_adj.as_ptr() as u64;

        unsafe {
            // 1. Guest Windows search path (`S:\Symbols`) via SymInitialize resolves the PDB.
            let proc1 = 0x5555usize as HANDLE;
            assert_eq!(SymInitialize(proc1, c"S:\\Symbols".as_ptr() as *const u8, FALSE), TRUE);
            assert_eq!(
                SymLoadModule64(
                    proc1,
                    std::ptr::null_mut(),
                    c"C:\\Nowhere\\winsearch.dll".as_ptr() as *const u8,
                    c"winsearch".as_ptr() as *const u8,
                    base_search,
                    0x10000,
                ),
                base_search
            );
            let mut disp = 0u32;
            let mut line = [0u8; 40];
            (line.as_mut_ptr() as *mut u32).write_unaligned(40);
            assert_eq!(
                SymGetLineFromAddr64(proc1, base_search + 0x1100, &mut disp, line.as_mut_ptr()),
                TRUE
            );
            assert_eq!((line.as_ptr().add(16) as *const u32).read_unaligned(), 77);
            assert_eq!(SymCleanup(proc1), TRUE);

            // 2. Guest Windows image path (`G:\Bin\winadj.dll`) resolves adjacent `G:\Bin\winadj.pdb`
            //    even when no search path is configured.
            let proc2 = 0x5556usize as HANDLE;
            assert_eq!(SymInitialize(proc2, std::ptr::null(), FALSE), TRUE);
            assert_eq!(
                SymLoadModule64(
                    proc2,
                    std::ptr::null_mut(),
                    c"G:\\Bin\\winadj.dll".as_ptr() as *const u8,
                    c"winadj".as_ptr() as *const u8,
                    base_adj,
                    0x10000,
                ),
                base_adj
            );
            let mut line2 = [0u8; 40];
            (line2.as_mut_ptr() as *mut u32).write_unaligned(40);
            assert_eq!(
                SymGetLineFromAddr64(proc2, base_adj + 0x1200, &mut disp, line2.as_mut_ptr()),
                TRUE
            );
            assert_eq!((line2.as_ptr().add(16) as *const u32).read_unaligned(), 88);
            assert_eq!(SymCleanup(proc2), TRUE);
        }

        let _ = std::fs::remove_dir_all(&sym_root);
        let _ = std::fs::remove_dir_all(&img_root);
    }

    #[test]
    fn parse_pdb_line_cache_rejects_truncated_dbi_mod_info_without_panicking() {
        // Case 1: DBI ModInfoSize exceeds the actual DBI stream length (64 + ModInfoSize > dbi.len()).
        // Must return None cleanly rather than indexing past `dbi` during module-name scanning.
        let mut pdb = build_minimal_pdb_with_id("src\\ok.cpp", 0x100, 10, [0x42; 16], 1);
        let (dbi_sz, dbi_off) = {
            let msf = MsfFile::parse(&pdb).expect("valid minimal MSF");
            let (sz, blocks) = &msf.streams[3];
            (*sz, blocks[0] as usize * msf.block_size)
        };
        let oversized = (dbi_sz + 64) as u32;
        pdb[dbi_off + 24..dbi_off + 28].copy_from_slice(&oversized.to_le_bytes());
        assert!(
            parse_pdb_line_cache(&pdb, &[0x1000], None).is_none(),
            "truncated DBI ModInfoSize must return None without panicking"
        );

        // Case 2: ModInfoSize fits inside dbi.len(), but ModuleName / ObjFileName lack NUL terminators
        // before the end of the substream. Must complete without panicking.
        let mut pdb2 = build_minimal_pdb_with_id("src\\ok.cpp", 0x100, 10, [0x42; 16], 1);
        let (dbi_sz2, dbi_off2) = {
            let msf = MsfFile::parse(&pdb2).expect("valid minimal MSF");
            let (sz, blocks) = &msf.streams[3];
            (*sz, blocks[0] as usize * msf.block_size)
        };
        let valid_mod_sz = dbi_sz2.saturating_sub(64);
        assert!(valid_mod_sz >= 68);
        pdb2[dbi_off2 + 24..dbi_off2 + 28].copy_from_slice(&(valid_mod_sz as u32).to_le_bytes());
        for b in &mut pdb2[dbi_off2 + 64 + 64..dbi_off2 + 64 + valid_mod_sz] {
            *b = b'A';
        }
        let _ = parse_pdb_line_cache(&pdb2, &[0x1000], None);
    }

    #[test]
    fn minidump_filter_module_paths_strips_directories_from_module_and_codeview_pdb_paths() {
        // Construct a synthetic PE32+ image in memory with an RSDS CodeView record containing a
        // full user build path (`C:\Users\secret_dev\src\secret_game.pdb`).
        let mut pe = vec![0u8; 0x1000];
        pe[0..2].copy_from_slice(b"MZ");
        pe[60..64].copy_from_slice(&0x80u32.to_le_bytes()); // e_lfanew
        let nt = 0x80usize;
        pe[nt..nt + 4].copy_from_slice(b"PE\0\0");
        pe[nt + 24..nt + 26].copy_from_slice(&0x020Bu16.to_le_bytes()); // PE32+
        let num_rva_off = nt + 24 + 108;
        pe[num_rva_off..num_rva_off + 4].copy_from_slice(&16u32.to_le_bytes());
        let dbg_dd_off = nt + 24 + 112 + 6 * 8;
        let dbg_dir_rva = 0x300u32;
        pe[dbg_dd_off..dbg_dd_off + 4].copy_from_slice(&dbg_dir_rva.to_le_bytes());
        pe[dbg_dd_off + 4..dbg_dd_off + 8].copy_from_slice(&28u32.to_le_bytes());

        let rsds_rva = 0x400u32;
        let mut rsds = Vec::new();
        rsds.extend_from_slice(b"RSDS");
        let expected_guid = [0x11u8; 16];
        let expected_age = 7u32;
        rsds.extend_from_slice(&expected_guid);
        rsds.extend_from_slice(&expected_age.to_le_bytes());
        rsds.extend_from_slice(b"C:\\Users\\secret_dev\\src\\secret_game.pdb\0");
        pe[rsds_rva as usize..rsds_rva as usize + rsds.len()].copy_from_slice(&rsds);

        // IMAGE_DEBUG_DIRECTORY at dbg_dir_rva (28 bytes): +12 Type=2 (CodeView), +16 SizeOfData, +20 AddressOfRawData.
        let d = dbg_dir_rva as usize;
        pe[d + 12..d + 16].copy_from_slice(&2u32.to_le_bytes());
        pe[d + 16..d + 20].copy_from_slice(&(rsds.len() as u32).to_le_bytes());
        pe[d + 20..d + 24].copy_from_slice(&rsds_rva.to_le_bytes());

        let uniq = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let tmp_dir = std::env::temp_dir().join(format!("pf_mdmp_filter_{uniq}"));
        std::fs::create_dir_all(&tmp_dir).unwrap();
        crate::path::add_mount("R:\\", &tmp_dir);

        unsafe {
            let proc_h = (-1isize) as HANDLE;
            assert_eq!(SymInitialize(proc_h, std::ptr::null(), FALSE), TRUE);
            let full_img = b"C:\\Users\\alice\\AppData\\Local\\SecretGame\\secret_game.dll\0";
            let mod_name = b"secret_game.dll\0";
            let base = pe.as_ptr() as u64;
            assert_eq!(
                SymLoadModule64(
                    proc_h,
                    std::ptr::null_mut(),
                    full_img.as_ptr(),
                    mod_name.as_ptr(),
                    base,
                    pe.len() as u32,
                ),
                base
            );

            let wpath = crate::strings::to_wide_z("R:\\filtered.dmp");
            let h_file = crate::kernel32::file::CreateFileW(
                wpath.as_ptr(),
                0xC000_0000,
                0,
                std::ptr::null_mut(),
                2,
                0x80,
                std::ptr::null_mut(),
            );
            assert_ne!(h_file, INVALID_HANDLE_VALUE);

            // Unsupported memory-filtering flags must still fail honestly with ERROR_NOT_SUPPORTED.
            for unsupported in [MINIDUMP_FILTER_MEMORY, MINIDUMP_FILTER_TRIAGE] {
                assert_eq!(
                    MiniDumpWriteDump(
                        proc_h,
                        std::process::id(),
                        h_file,
                        unsupported,
                        std::ptr::null(),
                        std::ptr::null(),
                        std::ptr::null(),
                    ),
                    FALSE
                );
                assert_eq!(crate::teb::last_error(), err::ERROR_NOT_SUPPORTED);
            }

            // MiniDumpFilterModulePaths (0x80) succeeds and strips directory prefixes from both
            // module names and CodeView RSDS PDB paths.
            assert_eq!(
                MiniDumpWriteDump(
                    proc_h,
                    std::process::id(),
                    h_file,
                    MINIDUMP_FILTER_MODULE_PATHS,
                    std::ptr::null(),
                    std::ptr::null(),
                    std::ptr::null(),
                ),
                TRUE
            );

            let mut fsize: i64 = 0;
            assert_eq!(crate::kernel32::file::GetFileSizeEx(h_file, &mut fsize), TRUE);
            let mut new_pos: i64 = 0;
            assert_eq!(crate::kernel32::file::SetFilePointerEx(h_file, 0, &mut new_pos, 0), TRUE);
            let mut bytes = vec![0u8; fsize as usize];
            let mut got = 0u32;
            assert_eq!(
                crate::kernel32::file::ReadFile(
                    h_file,
                    bytes.as_mut_ptr() as *mut c_void,
                    bytes.len() as u32,
                    &mut got,
                    std::ptr::null_mut(),
                ),
                TRUE
            );
            crate::kernel32::threads::CloseHandle(h_file);
            assert_eq!(SymCleanup(proc_h), TRUE);

            // Locate ModuleListStream (4) and inspect our module's name + CodeView record.
            let num_streams = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
            let dir_rva = u32::from_le_bytes(bytes[12..16].try_into().unwrap()) as usize;
            let mut mod_stream_rva = None;
            for i in 0..num_streams {
                let off = dir_rva + i * 12;
                let stype = u32::from_le_bytes(bytes[off..off + 4].try_into().unwrap());
                if stype == MINIDUMP_MODULE_LIST_STREAM {
                    mod_stream_rva = Some(u32::from_le_bytes(bytes[off + 8..off + 12].try_into().unwrap()) as usize);
                }
            }
            let mrva = mod_stream_rva.expect("ModuleListStream must exist");
            let mod_count = u32::from_le_bytes(bytes[mrva..mrva + 4].try_into().unwrap()) as usize;
            let mut found = false;
            for i in 0..mod_count {
                let moff = mrva + 4 + i * 108;
                let mbase = u64::from_le_bytes(bytes[moff..moff + 8].try_into().unwrap());
                let name_rva = u32::from_le_bytes(bytes[moff + 20..moff + 24].try_into().unwrap()) as usize;
                let name_bytes = u32::from_le_bytes(bytes[name_rva..name_rva + 4].try_into().unwrap()) as usize;
                let units: Vec<u16> = bytes[name_rva + 4..name_rva + 4 + name_bytes]
                    .chunks_exact(2)
                    .map(|c| u16::from_le_bytes([c[0], c[1]]))
                    .collect();
                let name_str = String::from_utf16(&units).unwrap();
                assert!(
                    !name_str.contains('\\') && !name_str.contains('/'),
                    "MiniDumpFilterModulePaths must strip directories from all module names: {name_str:?}"
                );
                if mbase == base {
                    found = true;
                    assert_eq!(name_str, "secret_game.dll");
                    let cv_sz = u32::from_le_bytes(bytes[moff + 76..moff + 80].try_into().unwrap()) as usize;
                    let cv_rva = u32::from_le_bytes(bytes[moff + 80..moff + 84].try_into().unwrap()) as usize;
                    let cv = &bytes[cv_rva..cv_rva + cv_sz];
                    assert_eq!(&cv[0..4], b"RSDS");
                    assert_eq!(&cv[4..20], &expected_guid);
                    assert_eq!(u32::from_le_bytes(cv[20..24].try_into().unwrap()), expected_age);
                    let pdb_str = std::str::from_utf8(&cv[24..cv.len() - 1]).unwrap();
                    assert_eq!(pdb_str, "secret_game.pdb");
                }
            }
            assert!(found, "registered module must appear in ModuleListStream");
        }

        let _ = std::fs::remove_dir_all(&tmp_dir);
    }
}
