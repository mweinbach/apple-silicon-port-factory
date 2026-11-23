//! Generic callback root discovery, relocation-invariant image fingerprinting, and
//! runtime-discovered callback RVA replay for cold AOT prewarming.
//!
//! Discipline:
//! - **Registration vs execution**: static discovery inspects mapped PE headers, `.pdata`
//!   unwind handlers, Load Config EH continuation tables, `.CRT*` initializer arrays, import
//!   tables, and basic-block argument/vtable patterns without ever executing guest code.
//! - **Guest vs host**: raw host/native pointers are never recorded or persisted.
//! - **Code vs data**: every static or replayed RVA is verified to lie inside an executable
//!   section of its owning PE image and pass code-entry validation.
//! - **Relocation & staleness**: runtime-discovered callbacks are persisted solely as
//!   module-relative RVAs keyed by a 128-bit relocation-invariant [`ImageFingerprint`];
//!   modified/stale images produce a different fingerprint and never replay stale RVAs.
//! - **EH continuation context**: [`CallbackKind::EhContinuation`] entries are tagged
//!   distinctly from normal callable function/callback entries because continuations resume
//!   mid-frame under `RtlRestoreContext` / `eh::land` rather than a fresh Win64 call frame.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, Once};

use super::cfg_discover::{data_dir, opt_image_base, pe_va_to_rva};
use super::format::{data_sections, exec_sections, write_atomic, H128};
use super::jumptable::is_plausible_code_entry;
use super::runtime::MAPPED;
use super::{cache_dir, fallback_cache_dirs_for};

/// Classification of a statically or dynamically discovered executable callback/root RVA.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum CallbackKind {
    /// CRT constructor/destructor array entry (`.CRT$*`, `_initterm`, `atexit`, `_onexit`).
    CrtCtorDtor = 1,
    /// Worker/thread entry (`CreateThread`, `_beginthread*`, threadpool work, `QueueUserWorkItem`).
    Worker = 2,
    /// Timer callback (`CreateThreadpoolTimer`, `CreateTimerQueueTimer`, `SetTimer`, `SetWaitableTimer*`, `timeSetEvent`).
    Timer = 3,
    /// Wait callback (`CreateThreadpoolWait`, `RegisterWaitForSingleObject`, `RtlRegisterWait`).
    Wait = 4,
    /// APC / async I/O completion (`QueueUserAPC*`, `ReadFileEx`, `WriteFileEx`, `CreateThreadpoolIo`).
    Apc = 5,
    /// Fiber-local storage destructor (`FlsAlloc`).
    Fls = 6,
    /// Window, dialog, subclass, hook, or enum procedure (`RegisterClass*`, `SetWindowLongPtr*`, `SetWindowSubclass`, `DialogBox*`, `CreateDialog*`, `Enum*`).
    WndProc = 7,
    /// COM sink / vtable / class factory method (`CoRegisterClassObject`, static COM vtable slots).
    ComSink = 8,
    /// Image-backed `.pdata` unwind/exception handler, scope filter/finally, VEH, or UEF.
    UnwindHandler = 9,
    /// EH continuation / catch landing target (`GuardEHContinuationTable`, `C_SCOPE_TABLE.JumpTarget`).
    /// Distinct context: resumed inside an existing frame, NOT a standalone callable entry.
    EhContinuation = 10,
    /// Other verified executable module-relative guest callback target.
    GenericCallback = 11,
}

impl CallbackKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CrtCtorDtor => "crt_ctor_dtor",
            Self::Worker => "worker",
            Self::Timer => "timer",
            Self::Wait => "wait",
            Self::Apc => "apc",
            Self::Fls => "fls",
            Self::WndProc => "wndproc",
            Self::ComSink => "com_sink",
            Self::UnwindHandler => "unwind_handler",
            Self::EhContinuation => "eh_continuation",
            Self::GenericCallback => "callback",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.trim() {
            "crt_ctor_dtor" => Some(Self::CrtCtorDtor),
            "worker" => Some(Self::Worker),
            "timer" => Some(Self::Timer),
            "wait" => Some(Self::Wait),
            "apc" => Some(Self::Apc),
            "fls" => Some(Self::Fls),
            "wndproc" => Some(Self::WndProc),
            "com_sink" => Some(Self::ComSink),
            "unwind_handler" => Some(Self::UnwindHandler),
            "eh_continuation" => Some(Self::EhContinuation),
            "callback" => Some(Self::GenericCallback),
            _ => None,
        }
    }

    /// `true` for EH continuation landing pads, which resume mid-frame with restored context
    /// rather than entering as a standalone Win64 callable function.
    #[inline]
    pub const fn is_eh_continuation(self) -> bool {
        matches!(self, Self::EhContinuation)
    }
}

/// One discovered module-relative executable callback/root RVA with its semantic kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CallbackRoot {
    pub rva: u32,
    pub kind: CallbackKind,
}

/// Relocation-invariant 128-bit content fingerprint of a mapped PE32+ image.
///
/// Identical across arbitrary base-address relocations of the same binary, and changes
/// whenever executable bytes, section layout, `SizeOfImage`, `TimeDateStamp`, `CheckSum`,
/// or `AddressOfEntryPoint` change (invalidating stale callback replay entries).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ImageFingerprint {
    pub hash: [u64; 2],
    pub size_of_image: u32,
    pub timestamp: u32,
    pub entry_rva: u32,
}

impl ImageFingerprint {
    pub fn hex(&self) -> String {
        format!("{:016x}{:016x}", self.hash[0], self.hash[1])
    }
}

/// Compute a relocation-invariant [`ImageFingerprint`] for the mapped PE32+ image at
/// `[base, base + size)`. Returns `None` if `[base, base + size)` does not begin with valid
/// PE32+ headers.
pub unsafe fn image_fingerprint(base: usize, size: usize) -> Option<ImageFingerprint> {
    if base == 0 || size < 0x400 {
        return None;
    }
    let rd16 = |off: usize| core::ptr::read_unaligned((base + off) as *const u16);
    let rd32 = |off: usize| core::ptr::read_unaligned((base + off) as *const u32);
    let rd64 = |off: usize| core::ptr::read_unaligned((base + off) as *const u64);

    if rd16(0) != 0x5A4D {
        return None;
    }
    let nt = rd32(0x3C) as usize;
    let opt = nt.checked_add(24)?;
    if opt.checked_add(112)? > size || rd32(nt) != 0x0000_4550 || rd16(opt) != 0x20B {
        return None;
    }

    let nsec = rd16(nt + 6) as usize;
    let timestamp = rd32(nt + 8);
    let opt_sz = rd16(nt + 20) as usize;
    let entry_rva = rd32(opt + 16);
    let size_of_image = rd32(opt + 56);
    let size_of_headers = rd32(opt + 60);
    let checksum = rd32(opt + 64);

    let sec_table = opt.checked_add(opt_sz)?;
    if sec_table.checked_add(nsec.checked_mul(40)?)? > size {
        return None;
    }

    let mut h = H128::new(0xCB_F1_2026);
    h.word(size as u64);
    h.word(size_of_image as u64);
    h.word(timestamp as u64);
    h.word(entry_rva as u64);
    h.word(size_of_headers as u64);
    h.word(checksum as u64);
    h.word(nsec as u64);

    for i in 0..nsec {
        let e = sec_table + i * 40;
        let name = core::slice::from_raw_parts((base + e) as *const u8, 8);
        h.bytes(name);
        h.word(rd32(e + 8) as u64); // VirtualSize
        h.word(rd32(e + 12) as u64); // VirtualAddress
        h.word(rd32(e + 36) as u64); // Characteristics
    }

    // Collect base-relocation slots inside executable sections so we can normalize relocated
    // pointers/immediates to module-relative offsets (`val - base`) before hashing.
    // Subtracting the actual mapped `base` yields `(disk_val - preferred_base)` regardless of
    // where the image is mapped and regardless of whether the loader rewrote
    // `OptionalHeader.ImageBase` to `base` after applying relocations.
    let exec = exec_sections(base, size);
    let mut exec_relocs: Vec<(u32, u8)> = Vec::new();
    if let Some((dir_rva, dir_len)) = data_dir(base, size, 5).filter(|&(r, _)| r != 0) {
        if let Some(dir_end) = (dir_rva as u64).checked_add(dir_len as u64) {
            if dir_end <= size as u64 {
                const IMAGE_REL_BASED_HIGHLOW: u16 = 3;
                const IMAGE_REL_BASED_DIR64: u16 = 10;
                let mut off = dir_rva as u64;
                while off.checked_add(8).is_some_and(|e| e <= dir_end) {
                    let page_rva = rd32(off as usize);
                    let block_size = rd32((off + 4) as usize);
                    if block_size < 8 {
                        break;
                    }
                    let Some(next_off) = off.checked_add(block_size as u64) else {
                        break;
                    };
                    if next_off > dir_end {
                        break;
                    }
                    let n_entries = ((block_size - 8) / 2) as u64;
                    for i in 0..n_entries {
                        let entry = rd16((off + 8 + i * 2) as usize);
                        let width: u8 = match entry >> 12 {
                            IMAGE_REL_BASED_DIR64 => 8,
                            IMAGE_REL_BASED_HIGHLOW => 4,
                            _ => continue,
                        };
                        if let Some(slot_rva) = page_rva.checked_add((entry & 0x0FFF) as u32) {
                            let va = base + slot_rva as usize;
                            if exec
                                .iter()
                                .any(|&(s, e)| va >= s && va.saturating_add(width as usize) <= e)
                            {
                                exec_relocs.push((slot_rva, width));
                            }
                        }
                    }
                    off = next_off;
                }
            }
        }
    }
    exec_relocs.sort_unstable();
    exec_relocs.dedup_by_key(|&mut (rva, _)| rva);

    for (s, e) in exec {
        let sec_rva = (s - base) as u32;
        let sec_end_rva = (e - base) as u32;
        h.word(sec_rva as u64);
        h.word((sec_end_rva - sec_rva) as u64);

        let lo_idx = exec_relocs.partition_point(|&(r, _)| r < sec_rva);
        let hi_idx = exec_relocs.partition_point(|&(r, _)| r < sec_end_rva);
        if lo_idx == hi_idx {
            h.bytes(core::slice::from_raw_parts(s as *const u8, e - s));
        } else {
            let mut cur = sec_rva;
            for &(slot, width) in &exec_relocs[lo_idx..hi_idx] {
                let w = width as u32;
                if slot < cur || slot.saturating_add(w) > sec_end_rva {
                    continue;
                }
                if slot > cur {
                    h.bytes(core::slice::from_raw_parts(
                        (base + cur as usize) as *const u8,
                        (slot - cur) as usize,
                    ));
                }
                let canon_val = if width == 8 {
                    rd64(slot as usize).wrapping_sub(base as u64)
                } else {
                    rd32(slot as usize).wrapping_sub(base as u32) as u64
                };
                h.word(canon_val);
                cur = slot + w;
            }
            if cur < sec_end_rva {
                h.bytes(core::slice::from_raw_parts(
                    (base + cur as usize) as *const u8,
                    (sec_end_rva - cur) as usize,
                ));
            }
        }
    }

    Some(ImageFingerprint {
        hash: h.finish(),
        size_of_image,
        timestamp,
        entry_rva,
    })
}

// ---------------------------------------------------------------------------------------------
// Static callback / unwind / EH continuation / CRT root discovery
// ---------------------------------------------------------------------------------------------

const UNW_FLAG_EHANDLER: u8 = 1;
const UNW_FLAG_UHANDLER: u8 = 2;
const UNW_FLAG_CHAININFO: u8 = 4;

/// Image-backed `.pdata` unwind/exception handlers (`UNW_FLAG_EHANDLER` / `UNW_FLAG_UHANDLER`)
/// and valid `C_SCOPE_TABLE` filter/finally handlers (`UnwindHandler`) + catch jump targets
/// (`EhContinuation`).
unsafe fn unwind_handler_and_scope_roots(
    base: usize,
    size: usize,
    text: &[(usize, usize)],
) -> Vec<CallbackRoot> {
    let mut out = Vec::new();
    let Some((rva, len)) = data_dir(base, size, 3) else {
        return out;
    };
    let fits = |off: u64, n: u64| off.checked_add(n).is_some_and(|e| e <= size as u64);
    let rd8 = |off: u64| *((base as u64 + off) as *const u8);
    let rd32 = |off: u64| core::ptr::read_unaligned((base as u64 + off) as *const u32);

    let in_text = |r: u32| -> bool {
        let ip = base as u64 + r as u64;
        text.iter().any(|&(s, e)| (s as u64..e as u64).contains(&ip))
            && is_plausible_code_entry(ip, text)
    };

    for i in 0..len as u64 / 12 {
        let off = rva as u64 + i * 12;
        if !fits(off, 12) {
            break;
        }
        let rf_begin = rd32(off);
        let rf_end = rd32(off + 4);
        let unwind_rva = rd32(off + 8);
        if rf_begin >= rf_end || !fits(rf_begin as u64, (rf_end - rf_begin) as u64) {
            continue;
        }
        if (unwind_rva & 1) != 0 || unwind_rva == 0 {
            continue;
        }
        let u = unwind_rva as u64;
        if !fits(u, 4) {
            continue;
        }
        let flags = rd8(u) >> 3;
        if (flags & UNW_FLAG_CHAININFO) != 0
            || (flags & (UNW_FLAG_EHANDLER | UNW_FLAG_UHANDLER)) == 0
        {
            continue;
        }
        let count_of_codes = rd8(u + 2) as u64;
        let handler_off = u + 4 + ((count_of_codes + 1) & !1) * 2;
        if !fits(handler_off, 4) {
            continue;
        }
        let handler_rva = rd32(handler_off);
        if handler_rva != 0 && in_text(handler_rva) {
            out.push(CallbackRoot { rva: handler_rva, kind: CallbackKind::UnwindHandler });
        }

        // Check if handler-specific data immediately following `handler_rva` is a well-formed
        // `C_SCOPE_TABLE` (`Count: u32`, followed by `Count` 16-byte `C_SCOPE_TABLE_ENTRY`
        // records whose `[BeginAddress, EndAddress]` intervals all lie inside `[rf_begin, rf_end]`).
        let scope_off = handler_off + 4;
        if !fits(scope_off, 4) {
            continue;
        }
        let scope_count = rd32(scope_off) as u64;
        if scope_count == 0 || scope_count > 256 || !fits(scope_off + 4, scope_count * 16) {
            continue;
        }
        let mut all_valid = true;
        for si in 0..scope_count {
            let rec = scope_off + 4 + si * 16;
            let sb = rd32(rec);
            let se = rd32(rec + 4);
            if !(sb >= rf_begin && sb < se && se <= rf_end) {
                all_valid = false;
                break;
            }
        }
        if !all_valid {
            continue;
        }
        for si in 0..scope_count {
            let rec = scope_off + 4 + si * 16;
            let sh = rd32(rec + 8); // HandlerAddress (1 = EXCEPTION_EXECUTE_HANDLER)
            let sj = rd32(rec + 12); // JumpTarget (catch continuation RVA)
            if sh > 1 && in_text(sh) {
                out.push(CallbackRoot { rva: sh, kind: CallbackKind::UnwindHandler });
            }
            if sj != 0 && sj >= rf_begin && sj < rf_end && in_text(sj) {
                out.push(CallbackRoot { rva: sj, kind: CallbackKind::EhContinuation });
            }
        }
    }
    out
}

/// Load Config `GuardEHContinuationTable` entries (`DataDirectory[10]`).
/// Tagged strictly as [`CallbackKind::EhContinuation`] (distinct mid-frame resume context).
pub(super) unsafe fn load_config_eh_continuation_rvas(
    base: usize,
    size: usize,
    text: &[(usize, usize)],
) -> Vec<CallbackRoot> {
    const IMAGE_GUARD_EH_CONTINUATION_TABLE_PRESENT: u32 = 0x0040_0000;
    const MAX_EH_CONT_ENTRIES: usize = 1_000_000;
    let mut out = Vec::new();
    let Some((dir_rva, _dir_len)) = data_dir(base, size, 10) else {
        return out;
    };
    if (dir_rva as u64).saturating_add(4) > size as u64 {
        return out;
    }
    let rd32 = |off: u64| core::ptr::read_unaligned((base as u64 + off) as *const u32);
    let rd64 = |off: u64| core::ptr::read_unaligned((base as u64 + off) as *const u64);
    let avail = (size as u64).saturating_sub(dir_rva as u64);
    let lc_size = (rd32(dir_rva as u64) as u64).min(avail);
    if lc_size < 0x120 {
        return out;
    }
    let guard_flags = rd32(dir_rva as u64 + 0x90);
    if (guard_flags & IMAGE_GUARD_EH_CONTINUATION_TABLE_PRESENT) == 0 {
        return out;
    }
    let tbl_va = rd64(dir_rva as u64 + 0x110);
    let count = rd64(dir_rva as u64 + 0x118);
    if tbl_va == 0 || count == 0 {
        return out;
    }
    let opt_base = opt_image_base(base, size);
    let Some(tbl_rva) = pe_va_to_rva(base, size, opt_base, tbl_va) else {
        return out;
    };
    let extra = ((guard_flags & 0xF000_0000) >> 28) as u64;
    let stride = 4u64 + extra;
    let max_fit = (size as u64).saturating_sub(tbl_rva as u64) / stride;
    let n = (count.min(max_fit) as usize).min(MAX_EH_CONT_ENTRIES);
    out.reserve(n);
    for i in 0..n {
        let rva = rd32(tbl_rva as u64 + (i as u64) * stride);
        let ip = base as u64 + rva as u64;
        if rva != 0
            && (rva as u64) < size as u64
            && text.iter().any(|&(s, e)| (s as u64..e as u64).contains(&ip))
            && is_plausible_code_entry(ip, text)
        {
            out.push(CallbackRoot { rva, kind: CallbackKind::EhContinuation });
        }
    }
    out
}

/// CRT constructor / destructor pointer arrays in `.CRT*` sections (e.g. `.CRT$XIA..XIZ`,
/// `.CRT$XCA..XCZ`, `.CRT$XPA..XPZ`, `.CRT$XTA..XTZ`).
unsafe fn crt_section_roots(
    base: usize,
    size: usize,
    text: &[(usize, usize)],
) -> Vec<CallbackRoot> {
    let mut out = Vec::new();
    if base == 0 || size < 0x400 || text.is_empty() {
        return out;
    }
    let rd16 = |off: usize| core::ptr::read_unaligned((base + off) as *const u16);
    let rd32 = |off: usize| core::ptr::read_unaligned((base + off) as *const u32);
    let rd64 = |off: usize| core::ptr::read_unaligned((base + off) as *const u64);
    if rd16(0) != 0x5A4D {
        return out;
    }
    let nt = rd32(0x3C) as usize;
    let Some(opt) = nt.checked_add(24) else { return out };
    if opt + 24 > size || rd32(nt) != 0x0000_4550 {
        return out;
    }
    let nsec = rd16(nt + 6) as usize;
    let Some(table) = opt.checked_add(rd16(nt + 20) as usize) else { return out };
    if table.checked_add(nsec * 40).is_none_or(|e| e > size) {
        return out;
    }
    let opt_base = opt_image_base(base, size);
    for i in 0..nsec {
        let e = table + i * 40;
        let name = core::slice::from_raw_parts((base + e) as *const u8, 8);
        if !name.starts_with(b".CRT") {
            continue;
        }
        let vsize = rd32(e + 8) as usize;
        let va = rd32(e + 12) as usize;
        let end = va.saturating_add(vsize).min(size);
        let mut off = (va + 7) & !7;
        while off.checked_add(8).is_some_and(|x| x <= end) {
            let ptr = rd64(off);
            if let Some(rva) = pe_va_to_rva(base, size, opt_base, ptr) {
                let ip = base as u64 + rva as u64;
                if text.iter().any(|&(s, e)| (s as u64..e as u64).contains(&ip))
                    && is_plausible_code_entry(ip, text)
                {
                    out.push(CallbackRoot { rva, kind: CallbackKind::CrtCtorDtor });
                }
            }
            off += 8;
        }
    }
    out
}

/// Read a NUL-terminated ASCII string inside `[base, base + size)` at `rva` (capped at 128 bytes).
unsafe fn read_cstr_rva(base: usize, size: usize, rva: u32) -> Option<String> {
    let start = rva as usize;
    if start == 0 || start >= size {
        return None;
    }
    let max_len = (size - start).min(128);
    let slice = core::slice::from_raw_parts((base + start) as *const u8, max_len);
    let len = slice.iter().position(|&b| b == 0)?;
    if len == 0 {
        return None;
    }
    Some(String::from_utf8_lossy(&slice[..len]).into_owned())
}

/// Build `iat_slot_rva -> (dll_lower, symbol_or_ordinal)` from the mapped PE's Import
/// Directory (`DataDirectory[1]`) and Delay Import Directory (`DataDirectory[13]`).
unsafe fn mapped_import_map(base: usize, size: usize) -> HashMap<u32, (String, String)> {
    let mut map = HashMap::new();
    let rd32 = |off: usize| core::ptr::read_unaligned((base + off) as *const u32);
    let rd64 = |off: usize| core::ptr::read_unaligned((base + off) as *const u64);

    // 1. Standard Import Directory (`DataDirectory[1]`).
    if let Some((dir_rva, dir_len)) = data_dir(base, size, 1) {
        let mut off = dir_rva as usize;
        let end = (dir_rva as usize).saturating_add(dir_len as usize).min(size);
        while off + 20 <= end {
            let orig_ft = rd32(off);
            let name_rva = rd32(off + 12);
            let first_thunk = rd32(off + 16);
            if orig_ft == 0 && name_rva == 0 && first_thunk == 0 {
                break;
            }
            if let Some(dll_raw) = read_cstr_rva(base, size, name_rva) {
                let dll = dll_raw.to_ascii_lowercase();
                let ilt_rva = if orig_ft != 0 { orig_ft } else { first_thunk };
                let mut i = 0usize;
                while i < 16_384 {
                    let ilt_off = (ilt_rva as usize).saturating_add(i * 8);
                    let iat_rva = first_thunk.wrapping_add((i * 8) as u32);
                    if ilt_off + 8 > size || (iat_rva as usize) + 8 > size {
                        break;
                    }
                    let thunk = rd64(ilt_off);
                    if thunk == 0 {
                        break;
                    }
                    let sym = if (thunk & (1u64 << 63)) != 0 {
                        format!("#{}", (thunk & 0xFFFF) as u16)
                    } else {
                        let hint_name_rva = (thunk & 0x7FFF_FFFF) as u32;
                        match read_cstr_rva(base, size, hint_name_rva.wrapping_add(2)) {
                            Some(s) => s,
                            None => {
                                i += 1;
                                continue;
                            }
                        }
                    };
                    map.insert(iat_rva, (dll.clone(), sym));
                    i += 1;
                }
            }
            off += 20;
        }
    }

    // 2. Delay Import Directory (`DataDirectory[13]`).
    if let Some((dir_rva, dir_len)) = data_dir(base, size, 13) {
        let opt_base = opt_image_base(base, size);
        let mut off = dir_rva as usize;
        let end = (dir_rva as usize).saturating_add(dir_len as usize).min(size);
        while off + 32 <= end {
            let attrs = rd32(off);
            let name_field = rd32(off + 4);
            let iat_field = rd32(off + 12);
            let int_field = rd32(off + 16);
            if name_field == 0 && iat_field == 0 && int_field == 0 {
                break;
            }
            // Bit 0 == 1 means RVA-based (standard PE32+); 0 means legacy VA-based.
            let to_rva = |v: u32| -> Option<u32> {
                if (attrs & 1) != 0 {
                    (v != 0 && (v as usize) < size).then_some(v)
                } else {
                    pe_va_to_rva(base, size, opt_base, v as u64)
                }
            };
            if let (Some(nr), Some(iat_rva0), Some(int_rva0)) =
                (to_rva(name_field), to_rva(iat_field), to_rva(int_field))
            {
                if let Some(dll_raw) = read_cstr_rva(base, size, nr) {
                    let dll = dll_raw.to_ascii_lowercase();
                    let mut i = 0usize;
                    while i < 16_384 {
                        let int_off = (int_rva0 as usize).saturating_add(i * 8);
                        let iat_rva = iat_rva0.wrapping_add((i * 8) as u32);
                        if int_off + 8 > size || (iat_rva as usize) + 8 > size {
                            break;
                        }
                        let thunk = rd64(int_off);
                        if thunk == 0 {
                            break;
                        }
                        let sym = if (thunk & (1u64 << 63)) != 0 {
                            format!("#{}", (thunk & 0xFFFF) as u16)
                        } else {
                            let hint_rva = if (attrs & 1) != 0 {
                                (thunk & 0x7FFF_FFFF) as u32
                            } else {
                                match pe_va_to_rva(base, size, opt_base, thunk) {
                                    Some(r) => r,
                                    None => {
                                        i += 1;
                                        continue;
                                    }
                                }
                            };
                            match read_cstr_rva(base, size, hint_rva.wrapping_add(2)) {
                                Some(s) => s,
                                None => {
                                    i += 1;
                                    continue;
                                }
                            }
                        };
                        map.entry(iat_rva).or_insert_with(|| (dll.clone(), sym));
                        i += 1;
                    }
                }
            }
            off += 32;
        }
    }

    map
}

/// Callback parameter rule for a Win32/CRT/COM registration API.
#[derive(Clone, Copy)]
enum RegApiSpec {
    /// Direct callback pointer in Win64 argument slot `arg_idx` (0=rcx, 1=rdx, 2=r8, 3=r9, 4=[rsp+0x20], ...).
    DirectArg { arg_idx: u8, kind: CallbackKind },
    /// `SetWindowLongPtrW/A` or `SetClassLongPtrW/A`: callback in `r8` (arg 2) when `rdx` (arg 1) matches an index.
    WindowLongPtr { expected_indices: &'static [i64] },
    /// `RegisterClass(Ex)A/W`: struct pointer in `rcx` with `lpfnWndProc` at `+8`.
    WndClassStruct,
    /// `_initterm` / `_initterm_e`: pointer range `[rcx, rdx)`.
    InittermRange,
    /// `CoRegisterClassObject`: `IUnknown*` in `rdx` whose `[rdx]` vtable points to in-image COM methods.
    ComClassObject,
}

fn match_registration_api(dll: &str, sym: &str) -> Option<RegApiSpec> {
    // Handle well-known comctl32 ordinal 410 -> SetWindowSubclass.
    let norm_sym = if (dll == "comctl32.dll" || dll == "uxtheme.dll") && sym == "#410" {
        "SetWindowSubclass"
    } else {
        sym
    };
    match norm_sym {
        // Worker / thread
        "CreateThread" | "_beginthreadex" => {
            Some(RegApiSpec::DirectArg { arg_idx: 2, kind: CallbackKind::Worker })
        }
        "_beginthread"
        | "CreateThreadpoolWork"
        | "TrySubmitThreadpoolCallback"
        | "QueueUserWorkItem"
        | "RtlQueueWorkItem" => {
            Some(RegApiSpec::DirectArg { arg_idx: 0, kind: CallbackKind::Worker })
        }
        // Timer
        "CreateThreadpoolTimer" => {
            Some(RegApiSpec::DirectArg { arg_idx: 0, kind: CallbackKind::Timer })
        }
        "CreateTimerQueueTimer" | "timeSetEvent" => {
            Some(RegApiSpec::DirectArg { arg_idx: 2, kind: CallbackKind::Timer })
        }
        "SetTimer" | "SetWaitableTimer" | "SetWaitableTimerEx" => {
            Some(RegApiSpec::DirectArg { arg_idx: 3, kind: CallbackKind::Timer })
        }
        // Wait
        "CreateThreadpoolWait" => {
            Some(RegApiSpec::DirectArg { arg_idx: 0, kind: CallbackKind::Wait })
        }
        "RegisterWaitForSingleObject" | "RtlRegisterWait" => {
            Some(RegApiSpec::DirectArg { arg_idx: 2, kind: CallbackKind::Wait })
        }
        // APC / async I/O
        "QueueUserAPC" | "QueueUserAPC2" => {
            Some(RegApiSpec::DirectArg { arg_idx: 0, kind: CallbackKind::Apc })
        }
        "CreateThreadpoolIo" => {
            Some(RegApiSpec::DirectArg { arg_idx: 1, kind: CallbackKind::Apc })
        }
        "ReadFileEx" | "WriteFileEx" => {
            // 5th Win64 argument (`lpCompletionRoutine`) sits on the stack at `[rsp + 0x20]`.
            Some(RegApiSpec::DirectArg { arg_idx: 4, kind: CallbackKind::Apc })
        }
        // FLS
        "FlsAlloc" => Some(RegApiSpec::DirectArg { arg_idx: 0, kind: CallbackKind::Fls }),
        // Window / dialog / subclass / hook / enum procedures
        "SetWindowSubclass" | "SetWindowsHookExW" | "SetWindowsHookExA" | "EnumChildWindows" => {
            Some(RegApiSpec::DirectArg { arg_idx: 1, kind: CallbackKind::WndProc })
        }
        "EnumWindows" | "EnumThreadWindows" => {
            Some(RegApiSpec::DirectArg { arg_idx: 0, kind: CallbackKind::WndProc })
        }
        "EnumDisplayMonitors" => {
            Some(RegApiSpec::DirectArg { arg_idx: 2, kind: CallbackKind::WndProc })
        }
        "DialogBoxParamW"
        | "DialogBoxParamA"
        | "DialogBoxIndirectParamW"
        | "DialogBoxIndirectParamA"
        | "CreateDialogParamW"
        | "CreateDialogParamA"
        | "CreateDialogIndirectParamW"
        | "CreateDialogIndirectParamA" => {
            Some(RegApiSpec::DirectArg { arg_idx: 3, kind: CallbackKind::WndProc })
        }
        "SetWindowLongPtrW" | "SetWindowLongPtrA" => Some(RegApiSpec::WindowLongPtr {
            expected_indices: &[-4, 8], // GWLP_WNDPROC (-4), DWLP_DLGPROC (8)
        }),
        "SetClassLongPtrW" | "SetClassLongPtrA" => Some(RegApiSpec::WindowLongPtr {
            expected_indices: &[-24], // GCLP_WNDPROC (-24)
        }),
        "RegisterClassW" | "RegisterClassA" | "RegisterClassExW" | "RegisterClassExA" => {
            Some(RegApiSpec::WndClassStruct)
        }
        // CRT ctor / dtor
        "atexit" | "_onexit" | "_crt_atexit" | "_crt_at_quick_exit" => {
            Some(RegApiSpec::DirectArg { arg_idx: 0, kind: CallbackKind::CrtCtorDtor })
        }
        "_register_onexit_function" => {
            Some(RegApiSpec::DirectArg { arg_idx: 1, kind: CallbackKind::CrtCtorDtor })
        }
        "_initterm" | "_initterm_e" => Some(RegApiSpec::InittermRange),
        // Exception handlers
        "AddVectoredExceptionHandler" | "AddVectoredContinueHandler" => {
            Some(RegApiSpec::DirectArg { arg_idx: 1, kind: CallbackKind::UnwindHandler })
        }
        "SetUnhandledExceptionFilter" => {
            Some(RegApiSpec::DirectArg { arg_idx: 0, kind: CallbackKind::UnwindHandler })
        }
        "RtlInstallFunctionTableCallback" => {
            Some(RegApiSpec::DirectArg { arg_idx: 3, kind: CallbackKind::UnwindHandler })
        }
        // COM class factory registration
        "CoRegisterClassObject" => Some(RegApiSpec::ComClassObject),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RegTrack {
    Unknown,
    /// Provable absolute VA (from `lea reg, [rip + disp32]` or `mov reg, imm64`).
    Va(u64),
    /// Stack pointer + signed offset (`lea reg, [rsp + disp]`).
    StackOff(i64),
    /// Known scalar constant (e.g. `GWLP_WNDPROC = -4`).
    Scalar(i64),
}

fn gpr64_idx(r: iced_x86::Register) -> Option<usize> {
    if r == iced_x86::Register::None {
        return None;
    }
    let full = r.full_register();
    full.is_gpr64().then(|| full.number())
}

fn imm_i64(ins: &iced_x86::Instruction, op: u32) -> Option<i64> {
    use iced_x86::OpKind as K;
    match ins.op_kind(op) {
        K::Immediate8 => Some(ins.immediate8() as i8 as i64),
        K::Immediate16 => Some(ins.immediate16() as i16 as i64),
        K::Immediate32 => Some(ins.immediate32() as i32 as i64),
        K::Immediate64 => Some(ins.immediate64() as i64),
        K::Immediate8to16 => Some(ins.immediate8to16() as i64),
        K::Immediate8to32 => Some(ins.immediate8to32() as i64),
        K::Immediate8to64 => Some(ins.immediate8to64()),
        K::Immediate32to64 => Some(ins.immediate32to64()),
        _ => None,
    }
}

/// Scan executable sections for static callback registration call sites and static COM vtable
/// stores. Caps linear scan budget on huge binaries (`> 16 MiB` total `.text`) so offline
/// prewarm stays fast while still covering all plugins, DLLs, and typical executables.
unsafe fn static_registration_site_roots(
    base: usize,
    size: usize,
    text: &[(usize, usize)],
) -> Vec<CallbackRoot> {
    const MAX_SCAN_TEXT_BYTES: usize = 16 << 20;
    let total_text: usize = text.iter().map(|&(s, e)| e.saturating_sub(s)).sum();
    if total_text == 0 || total_text > MAX_SCAN_TEXT_BYTES {
        return Vec::new();
    }

    let iat_map = mapped_import_map(base, size);
    let data_secs = data_sections(base, size);
    let opt_base = opt_image_base(base, size);

    // Map 6-byte `jmp [rip + disp32]` IAT thunks (`thunk_rva -> iat_rva`).
    let mut thunk_to_iat: HashMap<u32, u32> = HashMap::new();
    if !iat_map.is_empty() {
        for &(s, e) in text {
            let slice = core::slice::from_raw_parts(s as *const u8, e - s);
            let sec_rva = (s - base) as u32;
            let mut pos = 0usize;
            while pos + 6 <= slice.len() {
                if slice[pos] == 0xFF && slice[pos + 1] == 0x25 {
                    let disp = i32::from_le_bytes([
                        slice[pos + 2],
                        slice[pos + 3],
                        slice[pos + 4],
                        slice[pos + 5],
                    ]);
                    let thunk_rva = sec_rva.wrapping_add(pos as u32);
                    let target_rva = (thunk_rva as i64 + 6 + disp as i64) as u32;
                    if iat_map.contains_key(&target_rva) {
                        thunk_to_iat.insert(thunk_rva, target_rva);
                    }
                    pos += 6;
                } else {
                    pos += 1;
                }
            }
        }
    }

    let in_text_code = |va: u64| -> Option<u32> {
        let rva = pe_va_to_rva(base, size, opt_base, va)?;
        let ip = base as u64 + rva as u64;
        if text.iter().any(|&(s, e)| (s as u64..e as u64).contains(&ip))
            && is_plausible_code_entry(ip, text)
        {
            Some(rva)
        } else {
            None
        }
    };

    let in_data = |va: u64| -> Option<u32> {
        let rva = pe_va_to_rva(base, size, opt_base, va)?;
        let addr = base + rva as usize;
        data_secs.iter().any(|&(s, e)| addr >= s && addr < e).then_some(rva)
    };

    let mut out = Vec::new();
    let mut scanned_vtables: BTreeSet<u32> = BTreeSet::new();

    // Helper: if `vtable_rva` in `.rdata`/`.data` looks like a COM vtable (>= 3 consecutive
    // executable code pointers for IUnknown::{QI, AddRef, Release}), emit all consecutive code
    // slots (up to 32) as `CallbackKind::ComSink`.
    let mut try_scan_com_vtable = |vtable_rva: u32, out: &mut Vec<CallbackRoot>| {
        if !scanned_vtables.insert(vtable_rva) {
            return;
        }
        let mut slots = Vec::new();
        for i in 0..32u32 {
            let off = (vtable_rva as usize).saturating_add((i as usize) * 8);
            if off + 8 > size {
                break;
            }
            let ptr = core::ptr::read_unaligned((base + off) as *const u64);
            match in_text_code(ptr) {
                Some(cb_rva) => slots.push(cb_rva),
                None => break,
            }
        }
        if slots.len() >= 3 {
            for cb_rva in slots {
                out.push(CallbackRoot { rva: cb_rva, kind: CallbackKind::ComSink });
            }
        }
    };

    use iced_x86::{Decoder, DecoderOptions, FlowControl as F, Instruction, Mnemonic as M, OpKind as K, Register as R};

    for &(s, e) in text {
        let slice = core::slice::from_raw_parts(s as *const u8, e - s);
        let mut dec = Decoder::with_ip(64, slice, s as u64, DecoderOptions::NONE);
        let mut ins = Instruction::default();

        let mut regs = [RegTrack::Unknown; 16];
        let mut stack_slots: BTreeMap<i64, RegTrack> = BTreeMap::new();

        while dec.can_decode() {
            dec.decode_out(&mut ins);
            if ins.is_invalid() {
                regs = [RegTrack::Unknown; 16];
                stack_slots.clear();
                continue;
            }
            let mn = ins.mnemonic();
            match mn {
                M::Lea => {
                    if ins.op0_kind() == K::Register {
                        if let Some(dst) = gpr64_idx(ins.op0_register()) {
                            regs[dst] = if ins.is_ip_rel_memory_operand() {
                                RegTrack::Va(ins.ip_rel_memory_address())
                            } else if ins.memory_base() == R::RSP
                                && ins.memory_index() == R::None
                            {
                                RegTrack::StackOff(ins.memory_displacement64() as i64)
                            } else if let Some(b) = gpr64_idx(ins.memory_base()) {
                                if ins.memory_index() == R::None {
                                    match regs[b] {
                                        RegTrack::StackOff(o) => RegTrack::StackOff(
                                            o.wrapping_add(ins.memory_displacement64() as i64),
                                        ),
                                        RegTrack::Va(v) => RegTrack::Va(
                                            v.wrapping_add(ins.memory_displacement64()),
                                        ),
                                        _ => RegTrack::Unknown,
                                    }
                                } else {
                                    RegTrack::Unknown
                                }
                            } else {
                                RegTrack::Unknown
                            };
                        }
                    }
                }
                M::Mov => {
                    match (ins.op0_kind(), ins.op1_kind()) {
                        (K::Register, K::Register) => {
                            if let (Some(d), Some(s)) =
                                (gpr64_idx(ins.op0_register()), gpr64_idx(ins.op1_register()))
                            {
                                regs[d] = regs[s];
                            }
                        }
                        (K::Register, _) if imm_i64(&ins, 1).is_some() => {
                            if let Some(d) = gpr64_idx(ins.op0_register()) {
                                let imm = imm_i64(&ins, 1).unwrap();
                                let u = imm as u64;
                                regs[d] = if pe_va_to_rva(base, size, opt_base, u).is_some() {
                                    RegTrack::Va(u)
                                } else {
                                    RegTrack::Scalar(imm)
                                };
                            }
                        }
                        (K::Register, K::Memory) => {
                            if let Some(d) = gpr64_idx(ins.op0_register()) {
                                if ins.memory_base() == R::RSP && ins.memory_index() == R::None {
                                    let off = ins.memory_displacement64() as i64;
                                    regs[d] = stack_slots
                                        .get(&off)
                                        .copied()
                                        .unwrap_or(RegTrack::Unknown);
                                } else {
                                    regs[d] = RegTrack::Unknown;
                                }
                            }
                        }
                        (K::Memory, K::Register) => {
                            if let Some(src) = gpr64_idx(ins.op1_register()) {
                                let val = regs[src];
                                if ins.memory_base() == R::RSP && ins.memory_index() == R::None {
                                    let off = ins.memory_displacement64() as i64;
                                    if (0..=0x200).contains(&off) {
                                        stack_slots.insert(off, val);
                                    }
                                } else if let Some(b) = gpr64_idx(ins.memory_base()) {
                                    if ins.memory_index() == R::None {
                                        if let RegTrack::StackOff(so) = regs[b] {
                                            let off =
                                                so.wrapping_add(ins.memory_displacement64() as i64);
                                            if (0..=0x200).contains(&off) {
                                                stack_slots.insert(off, val);
                                            }
                                        }
                                        // Pattern: storing an in-image `.rdata` vtable pointer into `[obj + 0]`.
                                        if ins.memory_displacement64() == 0 {
                                            if let RegTrack::Va(vt_va) = val {
                                                if let Some(vt_rva) = in_data(vt_va) {
                                                    try_scan_com_vtable(vt_rva, &mut out);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        (K::Memory, _) if imm_i64(&ins, 1).is_some() => {
                            let imm = imm_i64(&ins, 1).unwrap();
                            let u = imm as u64;
                            let val = if pe_va_to_rva(base, size, opt_base, u).is_some() {
                                RegTrack::Va(u)
                            } else {
                                RegTrack::Scalar(imm)
                            };
                            if ins.memory_base() == R::RSP && ins.memory_index() == R::None {
                                let off = ins.memory_displacement64() as i64;
                                if (0..=0x200).contains(&off) {
                                    stack_slots.insert(off, val);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                M::Xor => {
                    if ins.op0_kind() == K::Register && ins.op1_kind() == K::Register {
                        if let (Some(d), Some(s)) =
                            (gpr64_idx(ins.op0_register()), gpr64_idx(ins.op1_register()))
                        {
                            regs[d] = if d == s { RegTrack::Scalar(0) } else { RegTrack::Unknown };
                        }
                    }
                }
                M::Call => {
                    // Resolve IAT entry for `call [rip + iat]` or `call thunk`.
                    let iat_rva = if ins.op0_kind() == K::Memory && ins.is_ip_rel_memory_operand() {
                        ins.ip_rel_memory_address()
                            .checked_sub(base as u64)
                            .and_then(|r| u32::try_from(r).ok())
                    } else if matches!(
                        ins.op0_kind(),
                        K::NearBranch64 | K::NearBranch32 | K::NearBranch16
                    ) {
                        ins.near_branch_target()
                            .checked_sub(base as u64)
                            .and_then(|r| u32::try_from(r).ok())
                            .and_then(|tr| thunk_to_iat.get(&tr).copied())
                    } else {
                        None
                    };

                    if let Some((dll, sym)) = iat_rva.and_then(|r| iat_map.get(&r)) {
                        if let Some(spec) = match_registration_api(dll, sym) {
                            let arg_val = |idx: u8| -> RegTrack {
                                match idx {
                                    0 => regs[1], // rcx
                                    1 => regs[2], // rdx
                                    2 => regs[8], // r8
                                    3 => regs[9], // r9
                                    n => stack_slots
                                        .get(&(0x20 + ((n - 4) as i64) * 8))
                                        .copied()
                                        .unwrap_or(RegTrack::Unknown),
                                }
                            };
                            match spec {
                                RegApiSpec::DirectArg { arg_idx, kind } => {
                                    if let RegTrack::Va(va) = arg_val(arg_idx) {
                                        if let Some(rva) = in_text_code(va) {
                                            out.push(CallbackRoot { rva, kind });
                                        }
                                    }
                                }
                                RegApiSpec::WindowLongPtr { expected_indices } => {
                                    if let RegTrack::Scalar(idx) = arg_val(1) {
                                        if expected_indices.contains(&idx) {
                                            if let RegTrack::Va(va) = arg_val(2) {
                                                if let Some(rva) = in_text_code(va) {
                                                    out.push(CallbackRoot {
                                                        rva,
                                                        kind: CallbackKind::WndProc,
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
                                RegApiSpec::WndClassStruct => match arg_val(0) {
                                    RegTrack::StackOff(so) => {
                                        if let Some(RegTrack::Va(va)) =
                                            stack_slots.get(&(so + 8)).copied()
                                        {
                                            if let Some(rva) = in_text_code(va) {
                                                out.push(CallbackRoot {
                                                    rva,
                                                    kind: CallbackKind::WndProc,
                                                });
                                            }
                                        }
                                    }
                                    RegTrack::Va(va) => {
                                        if let Some(srva) = in_data(va) {
                                            let off = srva as usize + 8;
                                            if off + 8 <= size {
                                                let wndproc_va = core::ptr::read_unaligned(
                                                    (base + off) as *const u64,
                                                );
                                                if let Some(rva) = in_text_code(wndproc_va) {
                                                    out.push(CallbackRoot {
                                                        rva,
                                                        kind: CallbackKind::WndProc,
                                                    });
                                                }
                                            }
                                        }
                                    }
                                    _ => {}
                                },
                                RegApiSpec::InittermRange => {
                                    if let (RegTrack::Va(va_lo), RegTrack::Va(va_hi)) =
                                        (arg_val(0), arg_val(1))
                                    {
                                        if let (Some(r_lo), Some(r_hi)) =
                                            (in_data(va_lo), in_data(va_hi))
                                        {
                                            if r_lo < r_hi
                                                && (r_hi - r_lo) <= 0x10_000
                                                && (r_hi as usize) <= size
                                            {
                                                let mut p = r_lo as usize;
                                                while p + 8 <= r_hi as usize {
                                                    let cb_va = core::ptr::read_unaligned(
                                                        (base + p) as *const u64,
                                                    );
                                                    if let Some(rva) = in_text_code(cb_va) {
                                                        out.push(CallbackRoot {
                                                            rva,
                                                            kind: CallbackKind::CrtCtorDtor,
                                                        });
                                                    }
                                                    p += 8;
                                                }
                                            }
                                        }
                                    }
                                }
                                RegApiSpec::ComClassObject => {
                                    if let RegTrack::Va(obj_va) = arg_val(1) {
                                        if let Some(obj_rva) = in_data(obj_va) {
                                            if (obj_rva as usize) + 8 <= size {
                                                let vt_va = core::ptr::read_unaligned(
                                                    (base + obj_rva as usize) as *const u64,
                                                );
                                                if let Some(vt_rva) = in_data(vt_va) {
                                                    try_scan_com_vtable(vt_rva, &mut out);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Clobber Win64 volatile registers after call.
                    for r in [0usize, 1, 2, 8, 9, 10, 11] {
                        regs[r] = RegTrack::Unknown;
                    }
                }
                M::Ret | M::Int3 | M::Ud2 | M::Jmp => {
                    regs = [RegTrack::Unknown; 16];
                    stack_slots.clear();
                }
                _ => {
                    if ins.flow_control() != F::ConditionalBranch
                        && ins.op_count() > 0
                        && ins.op0_kind() == K::Register
                    {
                        if let Some(d) = gpr64_idx(ins.op0_register()) {
                            regs[d] = RegTrack::Unknown;
                        }
                    }
                }
            }
        }
    }

    out
}

/// Discover all static callback/unwind/EH-continuation/CRT roots from a mapped PE32+ image.
/// Sorted and deduplicated by `(rva, kind)`.
pub unsafe fn discover_static_callback_roots(
    base: usize,
    size: usize,
    text: &[(usize, usize)],
) -> Vec<CallbackRoot> {
    let mut out = unwind_handler_and_scope_roots(base, size, text);
    out.extend(load_config_eh_continuation_rvas(base, size, text));
    out.extend(crt_section_roots(base, size, text));
    out.extend(static_registration_site_roots(base, size, text));
    out.sort_unstable();
    out.dedup();
    out
}

// ---------------------------------------------------------------------------------------------
// Runtime-discovered callback RVA recording & fingerprint-keyed cold-prewarm replay
// ---------------------------------------------------------------------------------------------

struct RuntimeModuleCallbacks {
    fingerprint: ImageFingerprint,
    entries: BTreeSet<CallbackRoot>,
}

static RUNTIME_CALLBACKS: Mutex<Option<HashMap<String, RuntimeModuleCallbacks>>> =
    Mutex::new(None);

fn callback_file_in(dir: &Path, name: &str, fp: &ImageFingerprint) -> PathBuf {
    dir.join("callbacks").join(format!("{name}-{}.tsv", fp.hex()))
}

fn callback_file(name: &str, fp: &ImageFingerprint) -> PathBuf {
    callback_file_in(&cache_dir(), name, fp)
}

/// Record a runtime-discovered guest callback VA under its owning mapped PE image's
/// relocation-invariant [`ImageFingerprint`] and module-relative RVA.
///
/// Strictly enforces:
/// - **Guest vs host**: ignores null, native/host-callable VAs, and host text pointers.
/// - **Code vs data**: ignores any VA outside the owning PE image's executable sections.
/// - **No raw host pointers**: only `(rva, kind)` keyed by `(module_name, ImageFingerprint)`
///   is retained and persisted.
///
/// Returns `true` if a valid module-relative guest callback RVA was recorded.
pub fn record_runtime_callback(guest_va: u64, kind: CallbackKind) -> bool {
    if guest_va == 0 {
        return false;
    }
    if crate::emu::resolve_host_callable(guest_va).is_some() {
        return false;
    }
    if crate::exe::in_host_text(guest_va as usize) {
        return false;
    }

    let addr = guest_va as usize;
    let found = {
        let Ok(v) = MAPPED.try_lock() else {
            return false;
        };
        v.iter().rev().find_map(|m| {
            if addr < m.base || addr - m.base >= m.size {
                return None;
            }
            let fp = m.fingerprint?;
            if !m.exec.iter().any(|&(s, e)| addr >= s && addr < e) {
                return None;
            }
            let rva = u32::try_from(addr - m.base).ok()?;
            Some((m.name.clone(), fp, rva))
        })
    };

    let Some((name, fp, rva)) = found else {
        return false;
    };

    static HOOK: Once = Once::new();
    HOOK.call_once(|| {
        win32_rt::process::register_exit_hook(flush_runtime_callbacks);
    });

    if let Ok(mut g) = RUNTIME_CALLBACKS.try_lock() {
        let map = g.get_or_insert_with(HashMap::new);
        let entry = map.entry(name).or_insert_with(|| RuntimeModuleCallbacks {
            fingerprint: fp,
            entries: BTreeSet::new(),
        });
        if entry.fingerprint != fp {
            // Module was replaced by a newer image with a different fingerprint: drop stale RVAs.
            entry.fingerprint = fp;
            entry.entries.clear();
        }
        entry.entries.insert(CallbackRoot { rva, kind });
        true
    } else {
        false
    }
}

/// Remove stale `<name>-*.tsv` callback replay files for other image fingerprints.
fn prune_other_callback_fingerprints(dir: &Path, name: &str, keep: &Path) -> usize {
    let cb_dir = dir.join("callbacks");
    let pre = format!("{name}-");
    let Ok(rd) = std::fs::read_dir(&cb_dir) else {
        return 0;
    };
    let mut removed = 0usize;
    for e in rd.flatten() {
        let f = e.file_name();
        let Some(hex) = f
            .to_str()
            .and_then(|s| s.strip_prefix(pre.as_str()))
            .and_then(|s| s.strip_suffix(".tsv"))
        else {
            continue;
        };
        if hex.len() == 32
            && hex.bytes().all(|b| b.is_ascii_hexdigit())
            && e.path() != keep
            && std::fs::remove_file(e.path()).is_ok()
        {
            removed += 1;
        }
    }
    removed
}

fn parse_callback_tsv(
    content: &str,
    expected_name: &str,
    expected_fp: &ImageFingerprint,
) -> Option<Vec<CallbackRoot>> {
    let mut ver_ok = false;
    let mut mod_ok = false;
    let mut fp_ok = false;
    let mut size_ok = false;
    let mut ts_ok = false;
    let mut entry_ok = false;
    let want_fp = expected_fp.hex();
    let mut out = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(hdr) = line.strip_prefix('#') {
            let hdr = hdr.trim();
            if hdr == "port-factory-callbacks v1" {
                ver_ok = true;
            } else if let Some(v) = hdr.strip_prefix("module=") {
                mod_ok = v.trim() == expected_name;
            } else if let Some(v) = hdr.strip_prefix("fp=") {
                fp_ok = v.trim().eq_ignore_ascii_case(&want_fp);
            } else if let Some(v) = hdr.strip_prefix("size=") {
                size_ok = u32::from_str_radix(v.trim().trim_start_matches("0x"), 16)
                    .is_ok_and(|x| x == expected_fp.size_of_image);
            } else if let Some(v) = hdr.strip_prefix("timestamp=") {
                ts_ok = u32::from_str_radix(v.trim().trim_start_matches("0x"), 16)
                    .is_ok_and(|x| x == expected_fp.timestamp);
            } else if let Some(v) = hdr.strip_prefix("entry=") {
                entry_ok = u32::from_str_radix(v.trim().trim_start_matches("0x"), 16)
                    .is_ok_and(|x| x == expected_fp.entry_rva);
            }
            continue;
        }
        let mut it = line.split('\t');
        let (Some(rva_s), Some(kind_s)) = (it.next(), it.next()) else {
            continue;
        };
        let Ok(rva) = u32::from_str_radix(rva_s.trim().trim_start_matches("0x"), 16) else {
            continue;
        };
        let Some(kind) = CallbackKind::parse(kind_s) else {
            continue;
        };
        out.push(CallbackRoot { rva, kind });
    }

    if !(ver_ok && mod_ok && fp_ok && size_ok && ts_ok && entry_ok) {
        return None;
    }
    out.sort_unstable();
    out.dedup();
    Some(out)
}

fn format_callback_tsv(
    name: &str,
    fp: &ImageFingerprint,
    entries: &BTreeSet<CallbackRoot>,
) -> String {
    let mut s = String::with_capacity(160 + entries.len() * 28);
    s.push_str("# port-factory-callbacks v1\n");
    s.push_str(&format!("# module={name}\n"));
    s.push_str(&format!("# fp={}\n", fp.hex()));
    s.push_str(&format!("# size={:#x}\n", fp.size_of_image));
    s.push_str(&format!("# timestamp={:#x}\n", fp.timestamp));
    s.push_str(&format!("# entry={:#x}\n", fp.entry_rva));
    for e in entries {
        s.push_str(&format!("{:#010x}\t{}\n", e.rva, e.kind.as_str()));
    }
    s
}

/// Flush all runtime-discovered callback RVAs to `<cache>/callbacks/<module>-<fp>.tsv`,
/// merging with any existing entries for the same fingerprint and pruning stale fingerprints.
pub fn flush_runtime_callbacks() {
    let taken = match RUNTIME_CALLBACKS.try_lock() {
        Ok(mut g) => g.take(),
        Err(_) => return,
    };
    let Some(map) = taken else { return };
    let primary = cache_dir();
    for (name, mut rec) in map {
        if rec.entries.is_empty() {
            continue;
        }
        let path = callback_file(&name, &rec.fingerprint);
        if let Ok(existing) = std::fs::read_to_string(&path) {
            if let Some(prev) = parse_callback_tsv(&existing, &name, &rec.fingerprint) {
                rec.entries.extend(prev);
            }
        }
        let data = format_callback_tsv(&name, &rec.fingerprint, &rec.entries);
        match write_atomic(&path, data.as_bytes()) {
            Ok(()) => {
                let pruned = prune_other_callback_fingerprints(&primary, &name, &path);
                log::info!(
                    "emu aot: callbacks {name}: {} root(s) -> {}{}",
                    rec.entries.len(),
                    path.display(),
                    if pruned != 0 {
                        format!(" ({pruned} stale fingerprint file(s) removed)")
                    } else {
                        String::new()
                    }
                );
            }
            Err(e) => {
                log::warn!("emu aot: callbacks {name}: write {}: {e}", path.display());
            }
        }
    }
}

/// Load and validate persisted runtime-discovered callback RVAs matching `(name, base, size)`.
/// Rejects stale/mismatched fingerprints and filters every RVA against the live image's
/// executable sections and code-entry plausibility.
pub unsafe fn load_replayed_callback_roots(
    name: &str,
    base: usize,
    size: usize,
) -> Vec<CallbackRoot> {
    let Some(fp) = image_fingerprint(base, size) else {
        return Vec::new();
    };
    let text = exec_sections(base, size);
    if text.is_empty() {
        return Vec::new();
    }
    let norm = name.to_ascii_lowercase();
    let primary = cache_dir();
    let fallbacks = fallback_cache_dirs_for(&primary);
    let mut merged: BTreeSet<CallbackRoot> = BTreeSet::new();

    for dir in core::iter::once(primary.as_path()).chain(fallbacks.iter().map(PathBuf::as_path)) {
        let p = callback_file_in(dir, &norm, &fp);
        let Ok(content) = std::fs::read_to_string(&p) else {
            continue;
        };
        let Some(entries) = parse_callback_tsv(&content, &norm, &fp) else {
            log::warn!(
                "emu aot: callbacks {norm}: {} failed fingerprint validation; ignored",
                p.display()
            );
            continue;
        };
        for cb in entries {
            let ip = base as u64 + cb.rva as u64;
            if (cb.rva as usize) < size
                && text.iter().any(|&(s, e)| (s as u64..e as u64).contains(&ip))
                && is_plausible_code_entry(ip, &text)
            {
                merged.insert(cb);
            }
        }
    }

    // Also include any in-process runtime-discovered callbacks not yet flushed to disk.
    if let Ok(g) = RUNTIME_CALLBACKS.try_lock() {
        if let Some(map) = g.as_ref() {
            if let Some(rec) = map.get(&norm) {
                if rec.fingerprint == fp {
                    for &cb in &rec.entries {
                        let ip = base as u64 + cb.rva as u64;
                        if (cb.rva as usize) < size
                            && text.iter().any(|&(s, e)| (s as u64..e as u64).contains(&ip))
                            && is_plausible_code_entry(ip, &text)
                        {
                            merged.insert(cb);
                        }
                    }
                }
            }
        }
    }

    merged.into_iter().collect()
}

/// Combined static + fingerprint-matched runtime-replayed callback roots for a mapped PE image.
pub unsafe fn discover_all_callback_roots(
    name: &str,
    base: usize,
    size: usize,
) -> Vec<CallbackRoot> {
    let text = exec_sections(base, size);
    if text.is_empty() {
        return Vec::new();
    }
    let mut out = discover_static_callback_roots(base, size, &text);
    out.extend(load_replayed_callback_roots(name, base, size));
    out.sort_unstable();
    out.dedup();
    out
}

/// Test-only: clear in-memory runtime callback buffer for deterministic test isolation.
#[cfg(test)]
pub(super) fn clear_runtime_callbacks_for_test() {
    if let Ok(mut g) = RUNTIME_CALLBACKS.lock() {
        *g = None;
    }
}
