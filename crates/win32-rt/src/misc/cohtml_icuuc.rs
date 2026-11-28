//! Builtin `cohtml_icuuc.dll` — ICU 69 runtime (`Locale`, `UnicodeString`,
//! `BreakIterator`, `UBiDi`, `udata_setCommonData_69`, and
//! `u_setMemoryFunctions_69`) backed by Apple's native
//! `/usr/lib/libicucore.A.dylib`.
//!
//! Apple ships the complete Unicode UAX #9 (bidirectional algorithm), UAX #14
//! (line breaking), UAX #29 (extended grapheme cluster, word, and sentence
//! breaking), and CJK dictionary tables in `/usr/lib/libicucore.A.dylib` on
//! both macOS and iOS (via the dyld shared cache). We load its unversioned C
//! `ubrk_*` / `ubidi_*` / `uloc_*` entry points lazily through `dlopen`/`dlsym`,
//! with a clean pure-Rust UTF-16 fallback if the host library is unavailable.
//!
//! Guest `icu_69::Locale` and `icu_69::UnicodeString` instances store only an
//! 8-byte tagged handle at offset 0 (written via `ptr::write_unaligned`), backed
//! by host-side tables keyed by handle and guest address. This guarantees we
//! never write past guest object storage while keeping owned UTF-16 buffers
//! alive for `ubrk_setText` even after temporary stack `UnicodeString`s are
//! destroyed.

use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::ffi::{c_char, c_void, CStr, CString};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex, Once, OnceLock};

pub static ORDINALS: &[(u16, &str)] = &[];

pub static EXTRA_EXPORTS: &[crate::Export] = &[
    // icu_69::Locale constructors, destructor, accessors, and singletons
    crate::Export {
        name: "??0Locale@icu_69@@QEAA@XZ",
        ptr: crate::Ptr(locale_ctor_default as *const ()),
    },
    crate::Export {
        name: "??0Locale@icu_69@@QEAA@PEBD000@Z",
        ptr: crate::Ptr(locale_ctor as *const ()),
    },
    crate::Export {
        name: "??0Locale@icu_69@@QEAA@AEBV01@@Z",
        ptr: crate::Ptr(locale_ctor_copy as *const ()),
    },
    crate::Export {
        name: "??1Locale@icu_69@@UEAA@XZ",
        ptr: crate::Ptr(locale_dtor as *const ()),
    },
    crate::Export {
        name: "?getName@Locale@icu_69@@QEBAPEBDXZ",
        ptr: crate::Ptr(locale_get_name as *const ()),
    },
    crate::Export {
        name: "?getLanguage@Locale@icu_69@@QEBAPEBDXZ",
        ptr: crate::Ptr(locale_get_language as *const ()),
    },
    crate::Export {
        name: "?getCountry@Locale@icu_69@@QEBAPEBDXZ",
        ptr: crate::Ptr(locale_get_country as *const ()),
    },
    crate::Export {
        name: "?getVariant@Locale@icu_69@@QEBAPEBDXZ",
        ptr: crate::Ptr(locale_get_variant as *const ()),
    },
    crate::Export {
        name: "?getDefault@Locale@icu_69@@SAAEBV12@XZ",
        ptr: crate::Ptr(locale_get_default as *const ()),
    },
    crate::Export {
        name: "?getUS@Locale@icu_69@@SAAEBV12@XZ",
        ptr: crate::Ptr(locale_get_us as *const ()),
    },
    crate::Export {
        name: "?getRoot@Locale@icu_69@@SAAEBV12@XZ",
        ptr: crate::Ptr(locale_get_root as *const ()),
    },
    // icu_69::UnicodeString constructors, fromUTF8, and destructor
    crate::Export {
        name: "??0UnicodeString@icu_69@@QEAA@XZ",
        ptr: crate::Ptr(unicode_string_ctor_empty as *const ()),
    },
    crate::Export {
        name: "??0UnicodeString@icu_69@@QEAA@PEBD@Z",
        ptr: crate::Ptr(unicode_string_ctor as *const ()),
    },
    crate::Export {
        name: "??0UnicodeString@icu_69@@QEAA@PEBDH@Z",
        ptr: crate::Ptr(unicode_string_ctor_utf8_len as *const ()),
    },
    crate::Export {
        name: "??0UnicodeString@icu_69@@QEAA@PEBDH0@Z",
        ptr: crate::Ptr(unicode_string_ctor_codepage as *const ()),
    },
    crate::Export {
        name: "??0UnicodeString@icu_69@@QEAA@PEB_S@Z",
        ptr: crate::Ptr(unicode_string_ctor_u16 as *const ()),
    },
    crate::Export {
        name: "??0UnicodeString@icu_69@@QEAA@PEB_SH@Z",
        ptr: crate::Ptr(unicode_string_ctor_u16_len as *const ()),
    },
    crate::Export {
        name: "??0UnicodeString@icu_69@@QEAA@PEBG@Z",
        ptr: crate::Ptr(unicode_string_ctor_u16 as *const ()),
    },
    crate::Export {
        name: "??0UnicodeString@icu_69@@QEAA@PEBGH@Z",
        ptr: crate::Ptr(unicode_string_ctor_u16_len as *const ()),
    },
    crate::Export {
        name: "??0UnicodeString@icu_69@@QEAA@PEB_W@Z",
        ptr: crate::Ptr(unicode_string_ctor_u16 as *const ()),
    },
    crate::Export {
        name: "??0UnicodeString@icu_69@@QEAA@PEB_WH@Z",
        ptr: crate::Ptr(unicode_string_ctor_u16_len as *const ()),
    },
    crate::Export {
        name: "??0UnicodeString@icu_69@@QEAA@VConstChar16Ptr@1@@Z",
        ptr: crate::Ptr(unicode_string_ctor_u16 as *const ()),
    },
    crate::Export {
        name: "??0UnicodeString@icu_69@@QEAA@VConstChar16Ptr@1@H@Z",
        ptr: crate::Ptr(unicode_string_ctor_u16_len as *const ()),
    },
    crate::Export {
        name: "??0UnicodeString@icu_69@@QEAA@CVConstChar16Ptr@1@H@Z",
        ptr: crate::Ptr(unicode_string_ctor_terminated_u16 as *const ()),
    },
    crate::Export {
        name: "??0UnicodeString@icu_69@@QEAA@AEBV01@@Z",
        ptr: crate::Ptr(unicode_string_ctor_copy as *const ()),
    },
    crate::Export {
        name: "??1UnicodeString@icu_69@@UEAA@XZ",
        ptr: crate::Ptr(unicode_string_dtor as *const ()),
    },
    crate::Export {
        name: "?length@UnicodeString@icu_69@@QEBAHXZ",
        ptr: crate::Ptr(unicode_string_length as *const ()),
    },
    crate::Export {
        name: "?fromUTF8@UnicodeString@icu_69@@SA?AV12@VStringPiece@2@@Z",
        ptr: crate::Ptr(unicode_string_from_utf8 as *const ()),
    },
    // icu_69::BreakIterator factories
    crate::Export {
        name: "?createCharacterInstance@BreakIterator@icu_69@@SAPEAV12@AEBVLocale@2@AEAW4UErrorCode@@@Z",
        ptr: crate::Ptr(create_character_instance as *const ()),
    },
    crate::Export {
        name: "?createLineInstance@BreakIterator@icu_69@@SAPEAV12@AEBVLocale@2@AEAW4UErrorCode@@@Z",
        ptr: crate::Ptr(create_line_instance as *const ()),
    },
    crate::Export {
        name: "?createWordInstance@BreakIterator@icu_69@@SAPEAV12@AEBVLocale@2@AEAW4UErrorCode@@@Z",
        ptr: crate::Ptr(create_word_instance as *const ()),
    },
    crate::Export {
        name: "?createSentenceInstance@BreakIterator@icu_69@@SAPEAV12@AEBVLocale@2@AEAW4UErrorCode@@@Z",
        ptr: crate::Ptr(create_sentence_instance as *const ()),
    },
];

// ---------------------------------------------------------------------------
// ICU constants & handle tags
// ---------------------------------------------------------------------------

const UBRK_DONE: i32 = -1;

const U_ZERO_ERROR: i32 = 0;
const U_ILLEGAL_ARGUMENT_ERROR: i32 = 1;
const U_MEMORY_ALLOCATION_ERROR: i32 = 7;
const U_SAFECLONE_ALLOCATED_WARNING: i32 = -126;
const U_BUFFER_OVERFLOW_ERROR: i32 = 15;
const U_UNSUPPORTED_ERROR: i32 = 16;
const U_INVALID_STATE_ERROR: i32 = 27;

const UBIDI_LTR: i32 = 0;
const UBIDI_RTL: i32 = 1;
const UBIDI_MIXED: i32 = 2;
const UBIDI_NEUTRAL: i32 = 3;

const UBIDI_MAX_EXPLICIT_LEVEL: u8 = 125;
const UBIDI_DEFAULT_LTR: u8 = 0xFE;
const UBIDI_DEFAULT_RTL: u8 = 0xFF;

const HANDLE_TAG_MASK: u64 = 0xFFFF_FFFF_0000_0000;
const LOCALE_HANDLE_TAG: u64 = 0x4C4F_4336_0000_0000; // "LOC6"
const USTR_HANDLE_TAG: u64 = 0x5553_5436_0000_0000; // "UST6"

static NEXT_HANDLE_ID: AtomicU32 = AtomicU32::new(1);
static EMPTY_U16: [u16; 1] = [0];
static BREAK_ITERATOR_CLASS_ID: u8 = 0;
static CHAR_ITER_CLASS_ID: u8 = 0;

fn next_tagged_handle(tag: u64) -> u64 {
    loop {
        let id = NEXT_HANDLE_ID.fetch_add(1, Ordering::Relaxed);
        if id != 0 {
            return tag | (id as u64);
        }
    }
}

fn empty_u16_arc() -> Arc<[u16]> {
    static EMPTY: OnceLock<Arc<[u16]>> = OnceLock::new();
    Arc::clone(EMPTY.get_or_init(|| Vec::<u16>::new().into()))
}

// ---------------------------------------------------------------------------
// Dynamic Apple libicucore.A.dylib loader (macOS + iOS)
// ---------------------------------------------------------------------------

type UBreakIteratorOpaque = c_void;
type UBiDiOpaque = c_void;

struct IcuBidiCore {
    ubidi_open_sized: unsafe extern "C" fn(
        max_length: i32,
        max_run_count: i32,
        p_error_code: *mut i32,
    ) -> *mut UBiDiOpaque,
    ubidi_close: unsafe extern "C" fn(p_bidi: *mut UBiDiOpaque),
    ubidi_set_para: unsafe extern "C" fn(
        p_bidi: *mut UBiDiOpaque,
        text: *const u16,
        length: i32,
        para_level: u8,
        embedding_levels: *mut u8,
        p_error_code: *mut i32,
    ),
    ubidi_get_direction: unsafe extern "C" fn(p_bidi: *const UBiDiOpaque) -> i32,
    ubidi_get_base_direction: unsafe extern "C" fn(text: *const u16, length: i32) -> i32,
    ubidi_count_runs: unsafe extern "C" fn(p_bidi: *mut UBiDiOpaque, p_error_code: *mut i32) -> i32,
    ubidi_get_visual_run: unsafe extern "C" fn(
        p_bidi: *mut UBiDiOpaque,
        run_index: i32,
        p_logical_start: *mut i32,
        p_length: *mut i32,
    ) -> i32,
}

struct IcuCore {
    ubrk_open: unsafe extern "C" fn(
        kind: i32,
        locale: *const c_char,
        text: *const u16,
        text_length: i32,
        status: *mut i32,
    ) -> *mut UBreakIteratorOpaque,
    ubrk_close: unsafe extern "C" fn(bi: *mut UBreakIteratorOpaque),
    ubrk_safe_clone: Option<
        unsafe extern "C" fn(
            bi: *const UBreakIteratorOpaque,
            stack_buffer: *mut c_void,
            p_buffer_size: *mut i32,
            status: *mut i32,
        ) -> *mut UBreakIteratorOpaque,
    >,
    ubrk_clone: Option<
        unsafe extern "C" fn(
            bi: *const UBreakIteratorOpaque,
            status: *mut i32,
        ) -> *mut UBreakIteratorOpaque,
    >,
    ubrk_set_text: unsafe extern "C" fn(
        bi: *mut UBreakIteratorOpaque,
        text: *const u16,
        text_length: i32,
        status: *mut i32,
    ),
    ubrk_first: unsafe extern "C" fn(bi: *mut UBreakIteratorOpaque) -> i32,
    ubrk_last: unsafe extern "C" fn(bi: *mut UBreakIteratorOpaque) -> i32,
    ubrk_next: unsafe extern "C" fn(bi: *mut UBreakIteratorOpaque) -> i32,
    ubrk_previous: unsafe extern "C" fn(bi: *mut UBreakIteratorOpaque) -> i32,
    ubrk_following: unsafe extern "C" fn(bi: *mut UBreakIteratorOpaque, offset: i32) -> i32,
    ubrk_preceding: unsafe extern "C" fn(bi: *mut UBreakIteratorOpaque, offset: i32) -> i32,
    ubrk_is_boundary: unsafe extern "C" fn(bi: *mut UBreakIteratorOpaque, offset: i32) -> i8,
    ubrk_current: unsafe extern "C" fn(bi: *const UBreakIteratorOpaque) -> i32,
    ubrk_get_rule_status: unsafe extern "C" fn(bi: *mut UBreakIteratorOpaque) -> i32,
    ubrk_get_rule_status_vec: Option<
        unsafe extern "C" fn(
            bi: *mut UBreakIteratorOpaque,
            fill_in_vec: *mut i32,
            capacity: i32,
            status: *mut i32,
        ) -> i32,
    >,
    uloc_get_default: Option<unsafe extern "C" fn() -> *const c_char>,
    ubidi: Option<IcuBidiCore>,
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
unsafe fn dlsym_opt<T>(handle: *mut c_void, name: &[u8]) -> Option<T> {
    let sym = libc::dlsym(handle, name.as_ptr().cast::<c_char>());
    if sym.is_null() {
        None
    } else {
        Some(std::mem::transmute_copy::<*mut c_void, T>(&sym))
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
fn load_icu_core() -> Option<IcuCore> {
    const CANDIDATES: &[&[u8]] = &[
        b"/usr/lib/libicucore.A.dylib\0",
        b"libicucore.A.dylib\0",
        b"libicucore.dylib\0",
    ];
    unsafe {
        let mut handle: *mut c_void = std::ptr::null_mut();
        for path in CANDIDATES {
            let h = libc::dlopen(
                path.as_ptr().cast::<c_char>(),
                libc::RTLD_LAZY | libc::RTLD_LOCAL,
            );
            if !h.is_null() {
                handle = h;
                break;
            }
        }
        if handle.is_null() {
            log::warn!(
                "cohtml_icuuc: failed to dlopen /usr/lib/libicucore.A.dylib; using Rust fallback"
            );
            return None;
        }
        let ubidi = (|| {
            Some(IcuBidiCore {
                ubidi_open_sized: dlsym_opt(handle, b"ubidi_openSized\0")?,
                ubidi_close: dlsym_opt(handle, b"ubidi_close\0")?,
                ubidi_set_para: dlsym_opt(handle, b"ubidi_setPara\0")?,
                ubidi_get_direction: dlsym_opt(handle, b"ubidi_getDirection\0")?,
                ubidi_get_base_direction: dlsym_opt(handle, b"ubidi_getBaseDirection\0")?,
                ubidi_count_runs: dlsym_opt(handle, b"ubidi_countRuns\0")?,
                ubidi_get_visual_run: dlsym_opt(handle, b"ubidi_getVisualRun\0")?,
            })
        })();
        let resolved = (|| {
            Some(IcuCore {
                ubrk_open: dlsym_opt(handle, b"ubrk_open\0")?,
                ubrk_close: dlsym_opt(handle, b"ubrk_close\0")?,
                ubrk_safe_clone: dlsym_opt(handle, b"ubrk_safeClone\0"),
                ubrk_clone: dlsym_opt(handle, b"ubrk_clone\0"),
                ubrk_set_text: dlsym_opt(handle, b"ubrk_setText\0")?,
                ubrk_first: dlsym_opt(handle, b"ubrk_first\0")?,
                ubrk_last: dlsym_opt(handle, b"ubrk_last\0")?,
                ubrk_next: dlsym_opt(handle, b"ubrk_next\0")?,
                ubrk_previous: dlsym_opt(handle, b"ubrk_previous\0")?,
                ubrk_following: dlsym_opt(handle, b"ubrk_following\0")?,
                ubrk_preceding: dlsym_opt(handle, b"ubrk_preceding\0")?,
                ubrk_is_boundary: dlsym_opt(handle, b"ubrk_isBoundary\0")?,
                ubrk_current: dlsym_opt(handle, b"ubrk_current\0")?,
                ubrk_get_rule_status: dlsym_opt(handle, b"ubrk_getRuleStatus\0")?,
                ubrk_get_rule_status_vec: dlsym_opt(handle, b"ubrk_getRuleStatusVec\0"),
                uloc_get_default: dlsym_opt(handle, b"uloc_getDefault\0"),
                ubidi,
            })
        })();
        match resolved {
            Some(core) => {
                log::debug!("cohtml_icuuc: loaded native /usr/lib/libicucore.A.dylib");
                Some(core)
            }
            None => {
                log::warn!(
                    "cohtml_icuuc: missing required ubrk_* symbol in libicucore.A.dylib; using Rust fallback"
                );
                None
            }
        }
    }
}

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
fn load_icu_core() -> Option<IcuCore> {
    None
}

fn icu_core() -> Option<&'static IcuCore> {
    static CORE: OnceLock<Option<IcuCore>> = OnceLock::new();
    CORE.get_or_init(load_icu_core).as_ref()
}

fn icu_bidi() -> Option<&'static IcuBidiCore> {
    icu_core().and_then(|c| c.ubidi.as_ref())
}

// ---------------------------------------------------------------------------
// Custom ICU memory callbacks (`u_setMemoryFunctions_69`)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy)]
struct CustomMemFuncs {
    context: usize,
    alloc_fn: usize,
    realloc_fn: usize,
    free_fn: usize,
}

impl CustomMemFuncs {
    unsafe fn alloc(self, size: usize) -> usize {
        crate::call_guest8(
            self.alloc_fn,
            self.context as u64,
            size as u64,
            0,
            0,
            0,
            0,
            0,
            0,
        ) as usize
    }

    unsafe fn realloc(self, mem: usize, size: usize) -> usize {
        crate::call_guest8(
            self.realloc_fn,
            self.context as u64,
            mem as u64,
            size as u64,
            0,
            0,
            0,
            0,
            0,
        ) as usize
    }

    unsafe fn free(self, mem: usize) {
        if mem != 0 {
            let _ = crate::call_guest8(
                self.free_fn,
                self.context as u64,
                mem as u64,
                0,
                0,
                0,
                0,
                0,
                0,
            );
        }
    }
}

struct MemFuncState {
    heap_in_use: bool,
    custom: Option<CustomMemFuncs>,
}

fn mem_func_state() -> &'static Mutex<MemFuncState> {
    static STATE: OnceLock<Mutex<MemFuncState>> = OnceLock::new();
    STATE.get_or_init(|| {
        Mutex::new(MemFuncState {
            heap_in_use: false,
            custom: None,
        })
    })
}

/// Latch ICU heap-in-use state and snapshot any registered custom allocator
/// callbacks before dropping the lock (so callers never hold the mutex across
/// `call_guest8`).
fn acquire_custom_mem_funcs() -> Option<CustomMemFuncs> {
    let mut st = mem_func_state().lock().unwrap_or_else(|e| e.into_inner());
    st.heap_in_use = true;
    st.custom
}

// ---------------------------------------------------------------------------
// Unaligned guest memory helpers
// ---------------------------------------------------------------------------

unsafe fn read_guest_cstr_bytes(ptr: *const u8) -> Option<Vec<u8>> {
    if ptr.is_null() {
        return None;
    }
    const MAX_LEN: usize = 64 * 1024 * 1024;
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < MAX_LEN {
        let b = std::ptr::read_unaligned(ptr.add(i));
        if b == 0 {
            break;
        }
        out.push(b);
        i += 1;
    }
    Some(out)
}

unsafe fn read_guest_cstr_string(ptr: *const u8) -> Option<String> {
    read_guest_cstr_bytes(ptr).map(|b| String::from_utf8_lossy(&b).into_owned())
}

unsafe fn read_guest_bytes_counted(ptr: *const u8, len: i32) -> Vec<u8> {
    if ptr.is_null() || len == 0 {
        return Vec::new();
    }
    if len < 0 {
        return read_guest_cstr_bytes(ptr).unwrap_or_default();
    }
    const MAX_LEN: usize = 64 * 1024 * 1024;
    let n = (len as usize).min(MAX_LEN);
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        out.push(std::ptr::read_unaligned(ptr.add(i)));
    }
    out
}

unsafe fn read_guest_utf8_to_u16(ptr: *const u8, len: i32) -> Arc<[u16]> {
    let bytes = read_guest_bytes_counted(ptr, len);
    if bytes.is_empty() {
        return empty_u16_arc();
    }
    let s = String::from_utf8_lossy(&bytes);
    let u16s: Vec<u16> = s.encode_utf16().collect();
    u16s.into()
}

unsafe fn read_guest_latin1_to_u16(ptr: *const u8, len: i32) -> Arc<[u16]> {
    let bytes = read_guest_bytes_counted(ptr, len);
    if bytes.is_empty() {
        return empty_u16_arc();
    }
    let u16s: Vec<u16> = bytes.into_iter().map(|b| b as u16).collect();
    u16s.into()
}

unsafe fn read_guest_utf16(ptr: *const u16, len: i32) -> Arc<[u16]> {
    if ptr.is_null() || len == 0 {
        return empty_u16_arc();
    }
    const MAX_U16: usize = 32 * 1024 * 1024;
    let mut u16s = Vec::new();
    if len < 0 {
        let mut i = 0usize;
        while i < MAX_U16 {
            let cu = std::ptr::read_unaligned(ptr.add(i));
            if cu == 0 {
                break;
            }
            u16s.push(cu);
            i += 1;
        }
    } else {
        let n = (len as usize).min(MAX_U16);
        u16s.reserve(n);
        for i in 0..n {
            u16s.push(std::ptr::read_unaligned(ptr.add(i)));
        }
    }
    if u16s.is_empty() {
        empty_u16_arc()
    } else {
        u16s.into()
    }
}

// ---------------------------------------------------------------------------
// Handle-backed tables for icu_69::Locale and icu_69::UnicodeString
// ---------------------------------------------------------------------------

struct HandleTable<T: ?Sized> {
    by_handle: HashMap<u64, (usize, Arc<T>)>,
    by_addr: HashMap<usize, u64>,
}

impl<T: ?Sized> HandleTable<T> {
    fn new() -> Self {
        Self {
            by_handle: HashMap::new(),
            by_addr: HashMap::new(),
        }
    }

    fn insert(&mut self, tag: u64, addr: usize, value: Arc<T>) -> u64 {
        if addr != 0 {
            if let Some(old_handle) = self.by_addr.remove(&addr) {
                self.by_handle.remove(&old_handle);
            }
        }
        let handle = next_tagged_handle(tag);
        self.by_handle.insert(handle, (addr, value));
        if addr != 0 {
            self.by_addr.insert(addr, handle);
        }
        handle
    }

    fn insert_static(&mut self, tag: u64, value: Arc<T>) -> u64 {
        let handle = next_tagged_handle(tag);
        self.by_handle.insert(handle, (0, value));
        handle
    }

    fn get_ref(&self, tag: u64, raw_handle: u64, addr: usize) -> Option<&Arc<T>> {
        if (raw_handle & HANDLE_TAG_MASK) == tag {
            if let Some((_, val)) = self.by_handle.get(&raw_handle) {
                return Some(val);
            }
        }
        if addr != 0 {
            if let Some(&h) = self.by_addr.get(&addr) {
                if let Some((_, val)) = self.by_handle.get(&h) {
                    return Some(val);
                }
            }
        }
        None
    }

    fn get(&self, tag: u64, raw_handle: u64, addr: usize) -> Option<Arc<T>> {
        self.get_ref(tag, raw_handle, addr).map(Arc::clone)
    }

    fn remove(&mut self, tag: u64, raw_handle: u64, addr: usize) {
        if (raw_handle & HANDLE_TAG_MASK) == tag {
            if let Some((owner_addr, _)) = self.by_handle.remove(&raw_handle) {
                if owner_addr != 0 && self.by_addr.get(&owner_addr) == Some(&raw_handle) {
                    self.by_addr.remove(&owner_addr);
                }
            }
        }
        if addr != 0 {
            if let Some(h) = self.by_addr.remove(&addr) {
                self.by_handle.remove(&h);
            }
        }
    }
}

struct StaticSlot(usize);

// ---------------------------------------------------------------------------
// icu_69::Locale
// ---------------------------------------------------------------------------

struct LocaleData {
    language: CString,
    country: CString,
    variant: CString,
    canonical: CString,
}

fn cstring_sanitized(s: &str) -> CString {
    if !s.as_bytes().contains(&0) {
        CString::new(s).unwrap_or_default()
    } else {
        let cleaned: String = s.chars().filter(|&c| c != '\0').collect();
        CString::new(cleaned).unwrap_or_default()
    }
}

fn titlecase_script(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for (i, ch) in s.chars().enumerate() {
        if i == 0 {
            out.push(ch.to_ascii_uppercase());
        } else {
            out.push(ch.to_ascii_lowercase());
        }
    }
    out
}

impl LocaleData {
    fn from_parts(
        language: Option<&str>,
        country: Option<&str>,
        variant: Option<&str>,
        keywords: Option<&str>,
    ) -> Self {
        let Some(raw_lang) = language else {
            return Self::default_locale();
        };

        let raw_country = country.unwrap_or("").trim();
        let raw_variant = variant.unwrap_or("").trim();
        let raw_keywords = keywords.unwrap_or("").trim().trim_start_matches('@');

        let lang_trimmed = raw_lang.trim();
        if lang_trimmed.is_empty()
            && raw_country.is_empty()
            && raw_variant.is_empty()
            && raw_keywords.is_empty()
        {
            return Self {
                language: CString::default(),
                country: CString::default(),
                variant: CString::default(),
                canonical: CString::default(),
            };
        }

        let (base_from_lang, kw_from_lang) = match lang_trimmed.split_once('@') {
            Some((b, k)) => (b, k),
            None => (lang_trimmed, ""),
        };
        let eff_keywords = if !raw_keywords.is_empty() {
            raw_keywords
        } else {
            kw_from_lang
        };

        let (lang_str, script_str, country_str, variant_str) =
            if raw_country.is_empty() && raw_variant.is_empty() {
                if base_from_lang.eq_ignore_ascii_case("C")
                    || base_from_lang.eq_ignore_ascii_case("POSIX")
                {
                    (
                        String::from("en"),
                        String::new(),
                        String::from("US"),
                        String::from("POSIX"),
                    )
                } else {
                    let normalized = base_from_lang.replace('-', "_");
                    let parts: Vec<&str> = normalized.split('_').collect();
                    let l = parts.first().copied().unwrap_or("").to_ascii_lowercase();
                    let p1 = parts.get(1).copied().unwrap_or("");
                    if p1.len() == 4 && p1.bytes().all(|b| b.is_ascii_alphabetic()) {
                        let sc = titlecase_script(p1);
                        let c = parts.get(2).copied().unwrap_or("").to_ascii_uppercase();
                        let v = if parts.len() > 3 {
                            parts[3..].join("_").to_ascii_uppercase()
                        } else {
                            String::new()
                        };
                        (l, sc, c, v)
                    } else {
                        let c = p1.to_ascii_uppercase();
                        let v = if parts.len() > 2 {
                            parts[2..].join("_").to_ascii_uppercase()
                        } else {
                            String::new()
                        };
                        (l, String::new(), c, v)
                    }
                }
            } else {
                let l = base_from_lang.replace('-', "_").to_ascii_lowercase();
                let c = raw_country.to_ascii_uppercase();
                let v = raw_variant.to_ascii_uppercase();
                (l, String::new(), c, v)
            };

        let mut canonical = String::new();
        canonical.push_str(&lang_str);
        if !script_str.is_empty() {
            if !canonical.is_empty() {
                canonical.push('_');
            }
            canonical.push_str(&script_str);
        }
        if !country_str.is_empty() {
            if !canonical.is_empty() {
                canonical.push('_');
            }
            canonical.push_str(&country_str);
            if !variant_str.is_empty() {
                canonical.push('_');
                canonical.push_str(&variant_str);
            }
        } else if !variant_str.is_empty() {
            if !canonical.is_empty() {
                canonical.push_str("__");
            }
            canonical.push_str(&variant_str);
        }
        if !eff_keywords.is_empty() {
            canonical.push('@');
            canonical.push_str(eff_keywords);
        }

        Self {
            language: cstring_sanitized(&lang_str),
            country: cstring_sanitized(&country_str),
            variant: cstring_sanitized(&variant_str),
            canonical: cstring_sanitized(&canonical),
        }
    }

    fn default_locale() -> Self {
        if let Some(get_def) = icu_core().and_then(|c| c.uloc_get_default) {
            let ptr = unsafe { get_def() };
            if !ptr.is_null() {
                let s = unsafe { CStr::from_ptr(ptr) }.to_string_lossy();
                if !s.is_empty() {
                    return Self::from_parts(Some(&s), None, None, None);
                }
            }
        }
        Self::from_parts(Some("en"), Some("US"), None, None)
    }
}

fn locale_table() -> &'static Mutex<HandleTable<LocaleData>> {
    static TABLE: OnceLock<Mutex<HandleTable<LocaleData>>> = OnceLock::new();
    TABLE.get_or_init(|| Mutex::new(HandleTable::new()))
}

fn default_locale_ref() -> &'static Arc<LocaleData> {
    static DEFAULT: OnceLock<Arc<LocaleData>> = OnceLock::new();
    DEFAULT.get_or_init(|| Arc::new(LocaleData::default_locale()))
}

fn default_locale_arc() -> Arc<LocaleData> {
    Arc::clone(default_locale_ref())
}

unsafe fn lookup_locale_cstr_ptr(
    locale_ptr: *const c_void,
    field: fn(&LocaleData) -> &CString,
) -> *const c_char {
    if !locale_ptr.is_null() {
        let raw = std::ptr::read_unaligned(locale_ptr as *const usize) as u64;
        let tbl = locale_table().lock().unwrap_or_else(|e| e.into_inner());
        if let Some(arc) = tbl.get_ref(LOCALE_HANDLE_TAG, raw, locale_ptr as usize) {
            return field(arc).as_ptr();
        }
    }
    field(default_locale_ref()).as_ptr()
}

unsafe fn store_locale(this: *mut usize, data: LocaleData) -> *mut usize {
    if !this.is_null() {
        let mut tbl = locale_table().lock().unwrap_or_else(|e| e.into_inner());
        let handle = tbl.insert(LOCALE_HANDLE_TAG, this as usize, Arc::new(data));
        std::ptr::write_unaligned(this, handle as usize);
    }
    this
}

unsafe fn lookup_locale(locale_ptr: *const c_void) -> Arc<LocaleData> {
    if locale_ptr.is_null() {
        return default_locale_arc();
    }
    let raw = std::ptr::read_unaligned(locale_ptr as *const usize) as u64;
    let tbl = locale_table().lock().unwrap_or_else(|e| e.into_inner());
    tbl.get(LOCALE_HANDLE_TAG, raw, locale_ptr as usize)
        .unwrap_or_else(default_locale_arc)
}

fn static_locale_slot(slot: &'static OnceLock<StaticSlot>, init: impl FnOnce() -> LocaleData) -> *const usize {
    let s = slot.get_or_init(|| {
        let mut tbl = locale_table().lock().unwrap_or_else(|e| e.into_inner());
        let handle = tbl.insert_static(LOCALE_HANDLE_TAG, Arc::new(init()));
        StaticSlot(handle as usize)
    });
    &s.0 as *const usize
}

// ---------------------------------------------------------------------------
// icu_69::UnicodeString
// ---------------------------------------------------------------------------

fn ustr_table() -> &'static Mutex<HandleTable<[u16]>> {
    static TABLE: OnceLock<Mutex<HandleTable<[u16]>>> = OnceLock::new();
    TABLE.get_or_init(|| Mutex::new(HandleTable::new()))
}

unsafe fn store_unicode_string(this: *mut usize, u16s: Arc<[u16]>) -> *mut usize {
    if !this.is_null() {
        let mut tbl = ustr_table().lock().unwrap_or_else(|e| e.into_inner());
        let handle = tbl.insert(USTR_HANDLE_TAG, this as usize, u16s);
        std::ptr::write_unaligned(this, handle as usize);
    }
    this
}

unsafe fn lookup_unicode_string(ustr: *const usize) -> Arc<[u16]> {
    if ustr.is_null() {
        return empty_u16_arc();
    }
    let raw = std::ptr::read_unaligned(ustr) as u64;
    let tbl = ustr_table().lock().unwrap_or_else(|e| e.into_inner());
    tbl.get(USTR_HANDLE_TAG, raw, ustr as usize)
        .unwrap_or_else(empty_u16_arc)
}

fn static_empty_ustr_slot() -> *const usize {
    static EMPTY_USTR: OnceLock<StaticSlot> = OnceLock::new();
    let s = EMPTY_USTR.get_or_init(|| {
        let mut tbl = ustr_table().lock().unwrap_or_else(|e| e.into_inner());
        let handle = tbl.insert_static(USTR_HANDLE_TAG, empty_u16_arc());
        StaticSlot(handle as usize)
    });
    &s.0 as *const usize
}

// ---------------------------------------------------------------------------
// Minimal UCharCharacterIterator-compatible view for getText() / adoptText()
// ---------------------------------------------------------------------------

#[repr(C)]
struct CharIterShim {
    vtbl: *const [crate::Ptr; 4],
    text_length: i32,
    pos: i32,
    begin: i32,
    end: i32,
    text: *const u16,
}

#[repr(C, align(16))]
struct CharIterVtable([crate::Ptr; 4]);
unsafe impl Sync for CharIterVtable {}

static CHAR_ITER_VTBL: CharIterVtable = CharIterVtable([
    crate::Ptr(char_iter_scalar_dtor as *const ()),
    crate::Ptr(char_iter_dynamic_class_id as *const ()),
    crate::Ptr(char_iter_noop as *const ()),
    crate::Ptr(char_iter_noop as *const ()),
]);

unsafe extern "win64" fn char_iter_scalar_dtor(
    this: *mut CharIterShim,
    _flags: u32,
) -> *mut CharIterShim {
    this
}

unsafe extern "win64" fn char_iter_dynamic_class_id(
    _this: *const CharIterShim,
) -> *const c_void {
    (&CHAR_ITER_CLASS_ID as *const u8).cast::<c_void>()
}

unsafe extern "win64" fn char_iter_noop(_this: *mut CharIterShim) -> i32 {
    0
}

// ---------------------------------------------------------------------------
// icu_69::BreakIterator (all 24 MSVC x64 vtable slots)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
enum BreakKind {
    Character = 0, // UBRK_CHARACTER
    Word = 1,      // UBRK_WORD
    Line = 2,      // UBRK_LINE
    Sentence = 3,  // UBRK_SENTENCE
}

#[repr(C)]
struct IcuBreakIterator {
    vtbl: *const [crate::Ptr; 24],
    kind: BreakKind,
    locale: Arc<LocaleData>,
    /// Owned immutable UTF-16 backing buffer kept alive for `ubrk_setText`.
    text_buf: Arc<[u16]>,
    /// Native Apple `libicucore` handle (`NULL` only in pure-Rust fallback mode).
    raw: *mut UBreakIteratorOpaque,
    /// View returned by `BreakIterator::getText()`.
    char_iter: UnsafeCell<CharIterShim>,
    /// Pure-Rust fallback boundary offsets (always starts with `0`).
    fallback_breaks: Vec<i32>,
    fallback_cursor: usize,
    /// Optional guest-allocator backing block acquired via `u_setMemoryFunctions_69`.
    guest_alloc: Option<(CustomMemFuncs, usize)>,
}

impl Drop for IcuBreakIterator {
    fn drop(&mut self) {
        self.close_native();
    }
}

impl IcuBreakIterator {
    fn new(kind: BreakKind, locale: Arc<LocaleData>) -> Option<Self> {
        let guest_alloc = if let Some(mf) = acquire_custom_mem_funcs() {
            let ptr = unsafe { mf.alloc(std::mem::size_of::<IcuBreakIterator>()) };
            if ptr == 0 {
                return None;
            }
            Some((mf, ptr))
        } else {
            None
        };

        let raw = if let Some(icu) = icu_core() {
            let mut open_err: i32 = U_ZERO_ERROR;
            let handle = unsafe {
                (icu.ubrk_open)(
                    kind as i32,
                    locale.canonical.as_ptr(),
                    std::ptr::null(),
                    0,
                    &mut open_err,
                )
            };
            if !handle.is_null() && open_err <= 0 {
                let mut set_err: i32 = U_ZERO_ERROR;
                unsafe {
                    (icu.ubrk_set_text)(handle, EMPTY_U16.as_ptr(), 0, &mut set_err);
                }
                handle
            } else {
                if !handle.is_null() {
                    unsafe { (icu.ubrk_close)(handle) };
                }
                std::ptr::null_mut()
            }
        } else {
            std::ptr::null_mut()
        };

        Some(Self {
            vtbl: &BREAK_VTBL.0,
            kind,
            locale,
            text_buf: empty_u16_arc(),
            raw,
            char_iter: UnsafeCell::new(CharIterShim {
                vtbl: &CHAR_ITER_VTBL.0,
                text_length: 0,
                pos: 0,
                begin: 0,
                end: 0,
                text: EMPTY_U16.as_ptr(),
            }),
            fallback_breaks: vec![0],
            fallback_cursor: 0,
            guest_alloc,
        })
    }

    fn close_native(&mut self) {
        if !self.raw.is_null() {
            if let Some(icu) = icu_core() {
                unsafe { (icu.ubrk_close)(self.raw) };
            }
            self.raw = std::ptr::null_mut();
        }
        // Release text buffer strictly AFTER `ubrk_close` has completed.
        self.text_buf = empty_u16_arc();
        self.fallback_breaks.clear();
        self.fallback_breaks.push(0);
        self.fallback_cursor = 0;
        let ci = self.char_iter.get();
        unsafe {
            (*ci).text_length = 0;
            (*ci).pos = 0;
            (*ci).begin = 0;
            (*ci).end = 0;
            (*ci).text = EMPTY_U16.as_ptr();
        }
        if let Some((mf, ptr)) = self.guest_alloc.take() {
            unsafe { mf.free(ptr) };
        }
    }

    fn set_text_u16(&mut self, buf: Arc<[u16]>) {
        let len = buf.len().min(i32::MAX as usize) as i32;
        let ptr = if len == 0 {
            EMPTY_U16.as_ptr()
        } else {
            buf.as_ptr()
        };
        if !self.raw.is_null() {
            if let Some(icu) = icu_core() {
                // Keep previous text buffer alive until after `ubrk_setText` returns.
                let _old_buf = std::mem::replace(&mut self.text_buf, buf);
                let mut err: i32 = U_ZERO_ERROR;
                unsafe {
                    (icu.ubrk_set_text)(self.raw, ptr, len, &mut err);
                }
                self.sync_char_iter(0);
                return;
            }
        }
        self.text_buf = buf;
        self.fallback_breaks.clear();
        self.fallback_cursor = 0;
        compute_fallback_breaks(&self.text_buf, self.kind, &mut self.fallback_breaks);
        self.sync_char_iter(0);
    }

    fn sync_char_iter(&self, pos: i32) {
        let len = self.text_buf.len().min(i32::MAX as usize) as i32;
        let ptr = if len == 0 {
            EMPTY_U16.as_ptr()
        } else {
            self.text_buf.as_ptr()
        };
        let ci = self.char_iter.get();
        unsafe {
            (*ci).text_length = len;
            (*ci).pos = pos;
            (*ci).begin = 0;
            (*ci).end = len;
            (*ci).text = ptr;
        }
    }

    fn current_pos(&self) -> i32 {
        if !self.raw.is_null() {
            if let Some(icu) = icu_core() {
                return unsafe { (icu.ubrk_current)(self.raw) };
            }
        }
        self.fallback_breaks
            .get(self.fallback_cursor)
            .copied()
            .unwrap_or(0)
    }

    fn first(&mut self) -> i32 {
        if !self.raw.is_null() {
            if let Some(icu) = icu_core() {
                return unsafe { (icu.ubrk_first)(self.raw) };
            }
        }
        self.fallback_cursor = 0;
        0
    }

    fn last(&mut self) -> i32 {
        if !self.raw.is_null() {
            if let Some(icu) = icu_core() {
                return unsafe { (icu.ubrk_last)(self.raw) };
            }
        }
        self.fallback_cursor = self.fallback_breaks.len().saturating_sub(1);
        self.fallback_breaks
            .get(self.fallback_cursor)
            .copied()
            .unwrap_or(0)
    }

    fn next(&mut self) -> i32 {
        if !self.raw.is_null() {
            if let Some(icu) = icu_core() {
                return unsafe { (icu.ubrk_next)(self.raw) };
            }
        }
        if self.fallback_cursor + 1 < self.fallback_breaks.len() {
            self.fallback_cursor += 1;
            self.fallback_breaks[self.fallback_cursor]
        } else {
            UBRK_DONE
        }
    }

    fn previous(&mut self) -> i32 {
        if !self.raw.is_null() {
            if let Some(icu) = icu_core() {
                return unsafe { (icu.ubrk_previous)(self.raw) };
            }
        }
        if self.fallback_cursor > 0 {
            self.fallback_cursor -= 1;
            self.fallback_breaks[self.fallback_cursor]
        } else {
            UBRK_DONE
        }
    }

    fn following(&mut self, offset: i32) -> i32 {
        if !self.raw.is_null() {
            if let Some(icu) = icu_core() {
                return unsafe { (icu.ubrk_following)(self.raw, offset) };
            }
        }
        if let Some((idx, &val)) = self
            .fallback_breaks
            .iter()
            .enumerate()
            .find(|(_, &b)| b > offset)
        {
            self.fallback_cursor = idx;
            val
        } else {
            self.fallback_cursor = self.fallback_breaks.len().saturating_sub(1);
            UBRK_DONE
        }
    }

    fn preceding(&mut self, offset: i32) -> i32 {
        if !self.raw.is_null() {
            if let Some(icu) = icu_core() {
                return unsafe { (icu.ubrk_preceding)(self.raw, offset) };
            }
        }
        if let Some((idx, &val)) = self
            .fallback_breaks
            .iter()
            .enumerate()
            .rev()
            .find(|(_, &b)| b < offset)
        {
            self.fallback_cursor = idx;
            val
        } else {
            self.fallback_cursor = 0;
            UBRK_DONE
        }
    }

    fn is_boundary(&mut self, offset: i32) -> i8 {
        if !self.raw.is_null() {
            if let Some(icu) = icu_core() {
                return if unsafe { (icu.ubrk_is_boundary)(self.raw, offset) } != 0 {
                    1
                } else {
                    0
                };
            }
        }
        if offset < 0 {
            self.fallback_cursor = 0;
            return 0;
        }
        if let Some((idx, &val)) = self
            .fallback_breaks
            .iter()
            .enumerate()
            .find(|(_, &b)| b >= offset)
        {
            self.fallback_cursor = idx;
            if val == offset {
                1
            } else {
                0
            }
        } else {
            self.fallback_cursor = self.fallback_breaks.len().saturating_sub(1);
            0
        }
    }

    fn rule_status(&self) -> i32 {
        if !self.raw.is_null() {
            if let Some(icu) = icu_core() {
                return unsafe { (icu.ubrk_get_rule_status)(self.raw) };
            }
        }
        0
    }

    fn clone_boxed(&self) -> Option<Box<Self>> {
        let guest_alloc = if let Some(mf) = acquire_custom_mem_funcs() {
            let ptr = unsafe { mf.alloc(std::mem::size_of::<IcuBreakIterator>()) };
            if ptr == 0 {
                return None;
            }
            Some((mf, ptr))
        } else {
            None
        };

        let cloned_raw = if !self.raw.is_null() {
            if let Some(icu) = icu_core() {
                let mut err: i32 = U_ZERO_ERROR;
                let h = unsafe {
                    if let Some(clone_fn) = icu.ubrk_clone {
                        clone_fn(self.raw, &mut err)
                    } else if let Some(safe_clone_fn) = icu.ubrk_safe_clone {
                        safe_clone_fn(
                            self.raw,
                            std::ptr::null_mut(),
                            std::ptr::null_mut(),
                            &mut err,
                        )
                    } else {
                        std::ptr::null_mut()
                    }
                };
                if !h.is_null() && err <= 0 {
                    h
                } else {
                    if !h.is_null() {
                        unsafe { (icu.ubrk_close)(h) };
                    }
                    let mut open_err: i32 = U_ZERO_ERROR;
                    let reopened = unsafe {
                        (icu.ubrk_open)(
                            self.kind as i32,
                            self.locale.canonical.as_ptr(),
                            std::ptr::null(),
                            0,
                            &mut open_err,
                        )
                    };
                    if !reopened.is_null() && open_err <= 0 {
                        let len = self.text_buf.len().min(i32::MAX as usize) as i32;
                        let ptr = if len == 0 {
                            EMPTY_U16.as_ptr()
                        } else {
                            self.text_buf.as_ptr()
                        };
                        let mut set_err: i32 = U_ZERO_ERROR;
                        unsafe {
                            (icu.ubrk_set_text)(reopened, ptr, len, &mut set_err);
                            let pos = (icu.ubrk_current)(self.raw);
                            if pos > 0 {
                                (icu.ubrk_is_boundary)(reopened, pos);
                            }
                        }
                        reopened
                    } else {
                        if !reopened.is_null() {
                            unsafe { (icu.ubrk_close)(reopened) };
                        }
                        std::ptr::null_mut()
                    }
                }
            } else {
                std::ptr::null_mut()
            }
        } else {
            std::ptr::null_mut()
        };

        let text_buf = Arc::clone(&self.text_buf);
        let len = text_buf.len().min(i32::MAX as usize) as i32;
        let ptr = if len == 0 {
            EMPTY_U16.as_ptr()
        } else {
            text_buf.as_ptr()
        };
        let (fallback_breaks, fallback_cursor) = if cloned_raw.is_null() {
            if !self.raw.is_null() {
                let mut fb = Vec::new();
                compute_fallback_breaks(&text_buf, self.kind, &mut fb);
                let cur_pos = self.current_pos();
                let cur_idx = fb
                    .iter()
                    .position(|&b| b >= cur_pos)
                    .unwrap_or_else(|| fb.len().saturating_sub(1));
                (fb, cur_idx)
            } else {
                (self.fallback_breaks.clone(), self.fallback_cursor)
            }
        } else {
            (vec![0], 0)
        };

        Some(Box::new(Self {
            vtbl: &BREAK_VTBL.0,
            kind: self.kind,
            locale: Arc::clone(&self.locale),
            text_buf,
            raw: cloned_raw,
            char_iter: UnsafeCell::new(CharIterShim {
                vtbl: &CHAR_ITER_VTBL.0,
                text_length: len,
                pos: self.current_pos(),
                begin: 0,
                end: len,
                text: ptr,
            }),
            fallback_breaks,
            fallback_cursor,
            guest_alloc,
        }))
    }
}

#[repr(C, align(16))]
struct Vtable([crate::Ptr; 24]);
unsafe impl Sync for Vtable {}

/// Exact MSVC x64 virtual function table for `icu_69::BreakIterator`
/// (with `RuleBasedBreakIterator` extensions at slots 22-23).
/// Note that MSVC groups overloaded virtual methods (`setText`, `next`) at the
/// slot of their first declaration in reverse declaration order.
static BREAK_VTBL: Vtable = Vtable([
    crate::Ptr(brk_scalar_deleting_dtor as *const ()), // 0  (+0x00): ~BreakIterator()
    crate::Ptr(brk_get_dynamic_class_id as *const ()), // 1  (+0x08): getDynamicClassID() const
    crate::Ptr(brk_equals as *const ()),               // 2  (+0x10): operator==(const BreakIterator&) const
    crate::Ptr(brk_clone as *const ()),                // 3  (+0x18): clone() const
    crate::Ptr(brk_get_text as *const ()),             // 4  (+0x20): getText() const
    crate::Ptr(brk_get_utext as *const ()),            // 5  (+0x28): getUText(UText*, UErrorCode&) const
    crate::Ptr(brk_set_text_utext as *const ()),       // 6  (+0x30): setText(UText*, UErrorCode&)
    crate::Ptr(brk_set_text as *const ()),             // 7  (+0x38): setText(const UnicodeString&)
    crate::Ptr(brk_adopt_text as *const ()),           // 8  (+0x40): adoptText(CharacterIterator*)
    crate::Ptr(brk_first as *const ()),                // 9  (+0x48): first()
    crate::Ptr(brk_last as *const ()),                 // 10 (+0x50): last()
    crate::Ptr(brk_previous as *const ()),             // 11 (+0x58): previous()
    crate::Ptr(brk_next_n as *const ()),               // 12 (+0x60): next(int32_t)
    crate::Ptr(brk_next as *const ()),                 // 13 (+0x68): next()
    crate::Ptr(brk_current as *const ()),              // 14 (+0x70): current() const
    crate::Ptr(brk_following as *const ()),            // 15 (+0x78): following(int32_t)
    crate::Ptr(brk_preceding as *const ()),            // 16 (+0x80): preceding(int32_t)
    crate::Ptr(brk_is_boundary as *const ()),          // 17 (+0x88): isBoundary(int32_t)
    crate::Ptr(brk_get_rule_status as *const ()),      // 18 (+0x90): getRuleStatus() const
    crate::Ptr(brk_get_rule_status_vec as *const ()),  // 19 (+0x98): getRuleStatusVec(int32_t*, int32_t, UErrorCode&)
    crate::Ptr(brk_create_buffer_clone as *const ()),  // 20 (+0xa0): createBufferClone(void*, int32_t&, UErrorCode&)
    crate::Ptr(brk_refresh_input_text as *const ()),   // 21 (+0xa8): refreshInputText(UText*, UErrorCode&)
    crate::Ptr(brk_hash_code as *const ()),            // 22 (+0xb0): RuleBasedBreakIterator::hashCode() const
    crate::Ptr(brk_get_rules as *const ()),            // 23 (+0xb8): RuleBasedBreakIterator::getRules() const
]);

fn ensure_vtbl_registered() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        for fn_ptr in &BREAK_VTBL.0 {
            let p = fn_ptr.0 as usize;
            crate::register_native_va(p as u64, p);
        }
        for fn_ptr in &CHAR_ITER_VTBL.0 {
            let p = fn_ptr.0 as usize;
            crate::register_native_va(p as u64, p);
        }
        for e in EXTRA_EXPORTS {
            let p = e.ptr.0 as usize;
            crate::register_native_va(p as u64, p);
        }
    });
}

// Slot 0 (+0x00): ~BreakIterator() [scalar deleting destructor]
unsafe extern "win64" fn brk_scalar_deleting_dtor(
    this: *mut IcuBreakIterator,
    flags: u32,
) -> *mut IcuBreakIterator {
    if this.is_null() {
        return std::ptr::null_mut();
    }
    if (flags & 1) != 0 {
        drop(Box::from_raw(this));
    } else {
        (*this).close_native();
    }
    this
}

// Slot 1 (+0x08): getDynamicClassID() const
unsafe extern "win64" fn brk_get_dynamic_class_id(
    _this: *const IcuBreakIterator,
) -> *const c_void {
    (&BREAK_ITERATOR_CLASS_ID as *const u8).cast::<c_void>()
}

// Slot 2 (+0x10): operator==(const BreakIterator&) const
unsafe extern "win64" fn brk_equals(
    this: *const IcuBreakIterator,
    other: *const IcuBreakIterator,
) -> u8 {
    if this == other {
        return 1;
    }
    if this.is_null() || other.is_null() {
        return 0;
    }
    let a = &*this;
    let b = &*other;
    let eq = a.kind == b.kind
        && a.locale.canonical == b.locale.canonical
        && *a.text_buf == *b.text_buf
        && a.current_pos() == b.current_pos();
    if eq {
        1
    } else {
        0
    }
}

// Slot 3 (+0x18): clone() const
unsafe extern "win64" fn brk_clone(this: *const IcuBreakIterator) -> *mut IcuBreakIterator {
    if this.is_null() {
        return std::ptr::null_mut();
    }
    match (*this).clone_boxed() {
        Some(b) => Box::into_raw(b),
        None => std::ptr::null_mut(),
    }
}

// Slot 4 (+0x20): getText() const
unsafe extern "win64" fn brk_get_text(this: *const IcuBreakIterator) -> *mut c_void {
    if this.is_null() {
        return std::ptr::null_mut();
    }
    let it = &*this;
    it.sync_char_iter(it.current_pos());
    it.char_iter.get().cast::<c_void>()
}

// Slot 5 (+0x28): getUText(UText* fillIn, UErrorCode& status) const
unsafe extern "win64" fn brk_get_utext(
    _this: *const IcuBreakIterator,
    _fill_in: *mut c_void,
    err: *mut i32,
) -> *mut c_void {
    if !err.is_null() && std::ptr::read_unaligned(err) <= 0 {
        std::ptr::write_unaligned(err, U_UNSUPPORTED_ERROR);
    }
    std::ptr::null_mut()
}

// Slot 6 (+0x30): setText(UText* text, UErrorCode& status)
unsafe extern "win64" fn brk_set_text_utext(
    this: *mut IcuBreakIterator,
    text: *mut c_void,
    err: *mut i32,
) {
    if this.is_null() {
        return;
    }
    if !err.is_null() && std::ptr::read_unaligned(err) > 0 {
        return;
    }
    if text.is_null() {
        (*this).set_text_u16(empty_u16_arc());
        if !err.is_null() {
            std::ptr::write_unaligned(err, U_ZERO_ERROR);
        }
        return;
    }
    log::warn!("cohtml_icuuc: BreakIterator::setText(UText*) is unsupported across guest/host ABI");
    if !err.is_null() {
        std::ptr::write_unaligned(err, U_UNSUPPORTED_ERROR);
    }
}

// Slot 7 (+0x38): setText(const UnicodeString& text)
unsafe extern "win64" fn brk_set_text(this: *mut IcuBreakIterator, ustr: *const usize) {
    if this.is_null() {
        return;
    }
    let buf = lookup_unicode_string(ustr);
    (*this).set_text_u16(buf);
}

// Slot 8 (+0x40): adoptText(CharacterIterator* it)
unsafe extern "win64" fn brk_adopt_text(this: *mut IcuBreakIterator, char_iter: *mut c_void) {
    if this.is_null() {
        return;
    }
    if char_iter.is_null() {
        (*this).set_text_u16(empty_u16_arc());
        return;
    }
    let ci = std::ptr::read_unaligned(char_iter as *const CharIterShim);
    let buf = if ci.begin >= 0
        && ci.end >= ci.begin
        && ci.end <= ci.text_length
        && (ci.end - ci.begin) <= 32 * 1024 * 1024
        && !ci.text.is_null()
    {
        read_guest_utf16(ci.text.add(ci.begin as usize), ci.end - ci.begin)
    } else {
        empty_u16_arc()
    };
    (*this).set_text_u16(buf);

    // Per ICU semantics, `adoptText` assumes ownership of the passed CharacterIterator.
    // Invoke its guest virtual scalar deleting destructor only when running under the loader
    // and the pointer is not our own embedded `CharIterShim`.
    let self_ci = (*this).char_iter.get().cast::<c_void>();
    if char_iter != self_ci
        && ci.vtbl != &CHAR_ITER_VTBL.0 as *const _
        && !ci.vtbl.is_null()
        && crate::try_loader().is_some()
    {
        let dtor = std::ptr::read_unaligned(ci.vtbl as *const usize);
        if dtor != 0 {
            let _ = crate::call_guest8(dtor, char_iter as u64, 1, 0, 0, 0, 0, 0, 0);
        }
    }
}

// Slot 9 (+0x48): first()
unsafe extern "win64" fn brk_first(this: *mut IcuBreakIterator) -> i32 {
    if this.is_null() {
        return 0;
    }
    (*this).first()
}

// Slot 10 (+0x50): last()
unsafe extern "win64" fn brk_last(this: *mut IcuBreakIterator) -> i32 {
    if this.is_null() {
        return 0;
    }
    (*this).last()
}

// Slot 11 (+0x58): previous()
unsafe extern "win64" fn brk_previous(this: *mut IcuBreakIterator) -> i32 {
    if this.is_null() {
        return UBRK_DONE;
    }
    (*this).previous()
}

// Slot 12 (+0x60): next(int32_t n)
unsafe extern "win64" fn brk_next_n(this: *mut IcuBreakIterator, n: i32) -> i32 {
    if this.is_null() {
        return UBRK_DONE;
    }
    let it = &mut *this;
    if n == 0 {
        return it.current_pos();
    }
    let mut res = it.current_pos();
    if n > 0 {
        for _ in 0..n {
            res = it.next();
            if res == UBRK_DONE {
                break;
            }
        }
    } else {
        for _ in 0..n.unsigned_abs() {
            res = it.previous();
            if res == UBRK_DONE {
                break;
            }
        }
    }
    res
}

// Slot 13 (+0x68): next()
unsafe extern "win64" fn brk_next(this: *mut IcuBreakIterator) -> i32 {
    if this.is_null() {
        return UBRK_DONE;
    }
    (*this).next()
}

// Slot 14 (+0x70): current() const
unsafe extern "win64" fn brk_current(this: *const IcuBreakIterator) -> i32 {
    if this.is_null() {
        return 0;
    }
    (*this).current_pos()
}

// Slot 15 (+0x78): following(int32_t offset)
unsafe extern "win64" fn brk_following(this: *mut IcuBreakIterator, offset: i32) -> i32 {
    if this.is_null() {
        return UBRK_DONE;
    }
    (*this).following(offset)
}

// Slot 16 (+0x80): preceding(int32_t offset)
unsafe extern "win64" fn brk_preceding(this: *mut IcuBreakIterator, offset: i32) -> i32 {
    if this.is_null() {
        return UBRK_DONE;
    }
    (*this).preceding(offset)
}

// Slot 17 (+0x88): isBoundary(int32_t offset)
unsafe extern "win64" fn brk_is_boundary(this: *mut IcuBreakIterator, offset: i32) -> i8 {
    if this.is_null() {
        return 0;
    }
    (*this).is_boundary(offset)
}

// Slot 18 (+0x90): getRuleStatus() const
unsafe extern "win64" fn brk_get_rule_status(this: *const IcuBreakIterator) -> i32 {
    if this.is_null() {
        return 0;
    }
    (*this).rule_status()
}

// Slot 19 (+0x98): getRuleStatusVec(int32_t* fillInVec, int32_t capacity, UErrorCode& status)
unsafe extern "win64" fn brk_get_rule_status_vec(
    this: *mut IcuBreakIterator,
    fill_in_vec: *mut i32,
    capacity: i32,
    err: *mut i32,
) -> i32 {
    if this.is_null() {
        return 0;
    }
    if !err.is_null() && std::ptr::read_unaligned(err) > 0 {
        return 0;
    }
    let it = &*this;
    let cap = capacity.max(0) as usize;
    if !it.raw.is_null() {
        if let Some(vec_fn) = icu_core().and_then(|c| c.ubrk_get_rule_status_vec) {
            let mut tmp = vec![0i32; cap];
            let dst_ptr = if cap == 0 || fill_in_vec.is_null() {
                std::ptr::null_mut()
            } else {
                tmp.as_mut_ptr()
            };
            let mut icu_err: i32 = U_ZERO_ERROR;
            let count = vec_fn(it.raw, dst_ptr, capacity, &mut icu_err);
            if !fill_in_vec.is_null() && cap > 0 && count > 0 {
                let to_copy = (count as usize).min(cap);
                for (i, &val) in tmp.iter().take(to_copy).enumerate() {
                    std::ptr::write_unaligned(fill_in_vec.add(i), val);
                }
            }
            if !err.is_null() {
                std::ptr::write_unaligned(
                    err,
                    if icu_err > 0 { icu_err } else { U_ZERO_ERROR },
                );
            }
            return count;
        }
    }
    let status_val = it.rule_status();
    if capacity < 1 || fill_in_vec.is_null() {
        if !err.is_null() {
            std::ptr::write_unaligned(err, U_BUFFER_OVERFLOW_ERROR);
        }
        return 1;
    }
    std::ptr::write_unaligned(fill_in_vec, status_val);
    if !err.is_null() {
        std::ptr::write_unaligned(err, U_ZERO_ERROR);
    }
    1
}

// Slot 20 (+0xa0): createBufferClone(void* stackBuffer, int32_t& bufferSize, UErrorCode& status)
unsafe extern "win64" fn brk_create_buffer_clone(
    this: *const IcuBreakIterator,
    stack_buffer: *mut c_void,
    buffer_size: *mut i32,
    err: *mut i32,
) -> *mut IcuBreakIterator {
    if this.is_null() {
        return std::ptr::null_mut();
    }
    if !err.is_null() && std::ptr::read_unaligned(err) > 0 {
        return std::ptr::null_mut();
    }
    if !buffer_size.is_null() && std::ptr::read_unaligned(buffer_size) == 0 {
        std::ptr::write_unaligned(buffer_size, std::mem::size_of::<IcuBreakIterator>() as i32);
        return std::ptr::null_mut();
    }
    let cloned = brk_clone(this);
    if !err.is_null() {
        let code = if cloned.is_null() {
            U_MEMORY_ALLOCATION_ERROR
        } else if !stack_buffer.is_null() {
            U_SAFECLONE_ALLOCATED_WARNING
        } else {
            U_ZERO_ERROR
        };
        std::ptr::write_unaligned(err, code);
    }
    cloned
}

// Slot 21 (+0xa8): refreshInputText(UText* input, UErrorCode& status)
unsafe extern "win64" fn brk_refresh_input_text(
    this: *mut IcuBreakIterator,
    input: *mut c_void,
    err: *mut i32,
) -> *mut IcuBreakIterator {
    if !err.is_null() && std::ptr::read_unaligned(err) <= 0 {
        if input.is_null() {
            std::ptr::write_unaligned(err, U_ZERO_ERROR);
        } else {
            std::ptr::write_unaligned(err, U_UNSUPPORTED_ERROR);
        }
    }
    this
}

// Slot 22 (+0xb0): RuleBasedBreakIterator::hashCode() const
unsafe extern "win64" fn brk_hash_code(this: *const IcuBreakIterator) -> i32 {
    if this.is_null() {
        return 0;
    }
    let it = &*this;
    let mut h: u32 = 2166136261u32 ^ (it.kind as u32);
    for &b in it.locale.canonical.as_bytes() {
        h ^= b as u32;
        h = h.wrapping_mul(16777619);
    }
    h as i32
}

// Slot 23 (+0xb8): RuleBasedBreakIterator::getRules() const
unsafe extern "win64" fn brk_get_rules(_this: *const IcuBreakIterator) -> *const usize {
    static_empty_ustr_slot()
}

// ---------------------------------------------------------------------------
// Pure-Rust UTF-16 fallback (used only when libicucore is unavailable)
// ---------------------------------------------------------------------------

fn is_combining_or_extend(ch: char) -> bool {
    let cp = ch as u32;
    matches!(
        cp,
        0x0300..=0x036F
            | 0x1AB0..=0x1AFF
            | 0x1DC0..=0x1DFF
            | 0x20D0..=0x20FF
            | 0xFE00..=0xFE0F
            | 0xFE20..=0xFE2F
            | 0x1F3FB..=0x1F3FF
    )
}

fn is_cjk_ideograph(ch: char) -> bool {
    let cp = ch as u32;
    matches!(
        cp,
        0x3040..=0x309F
            | 0x30A0..=0x30FF
            | 0x3400..=0x4DBF
            | 0x4E00..=0x9FFF
            | 0xF900..=0xFAFF
            | 0x20000..=0x2A6DF
    )
}

fn compute_fallback_breaks(text: &[u16], kind: BreakKind, out: &mut Vec<i32>) {
    out.push(0);
    if text.is_empty() {
        return;
    }
    let chars: Vec<(char, i32)> = char::decode_utf16(text.iter().copied())
        .map(|r| match r {
            Ok(c) => (c, c.len_utf16() as i32),
            Err(_) => ('\u{FFFD}', 1),
        })
        .collect();

    let mut u16_pos: i32 = 0;
    match kind {
        BreakKind::Character => {
            let mut i = 0usize;
            while i < chars.len() {
                let (ch, len) = chars[i];
                u16_pos += len;
                i += 1;
                if ch == '\r' && i < chars.len() && chars[i].0 == '\n' {
                    u16_pos += chars[i].1;
                    i += 1;
                } else {
                    while i < chars.len() {
                        let next_ch = chars[i].0;
                        if next_ch == '\u{200D}' {
                            u16_pos += chars[i].1;
                            i += 1;
                            if i < chars.len() {
                                u16_pos += chars[i].1;
                                i += 1;
                            }
                        } else if is_combining_or_extend(next_ch) {
                            u16_pos += chars[i].1;
                            i += 1;
                        } else {
                            break;
                        }
                    }
                }
                out.push(u16_pos);
            }
        }
        BreakKind::Word => {
            #[derive(PartialEq, Eq, Clone, Copy)]
            enum Class {
                Word,
                Space,
                Other,
            }
            let classify = |c: char| {
                if c.is_alphanumeric() || c == '_' {
                    Class::Word
                } else if c.is_whitespace() {
                    Class::Space
                } else {
                    Class::Other
                }
            };
            let mut prev: Option<Class> = None;
            for &(ch, len) in &chars {
                let cls = classify(ch);
                if let Some(p) = prev {
                    if p != cls || cls == Class::Other {
                        out.push(u16_pos);
                    }
                }
                u16_pos += len;
                prev = Some(cls);
            }
            if out.last().copied() != Some(u16_pos) {
                out.push(u16_pos);
            }
        }
        BreakKind::Line => {
            let mut prev_ws = false;
            let mut prev_cjk = false;
            for &(ch, len) in &chars {
                let ws = ch.is_whitespace() || ch == '-';
                let cjk = is_cjk_ideograph(ch);
                let is_closing_punct = matches!(ch, '。' | '、' | '，' | '．' | '」' | '』' | ')' | ']' | '.' | ',' | '!' | '?');
                if !is_closing_punct && ((prev_ws && !ws) || (prev_cjk && cjk)) && u16_pos > 0 {
                    out.push(u16_pos);
                }
                u16_pos += len;
                if ch == '\n' {
                    out.push(u16_pos);
                    prev_ws = false;
                    prev_cjk = false;
                } else {
                    prev_ws = ws;
                    prev_cjk = cjk;
                }
            }
            if out.last().copied() != Some(u16_pos) {
                out.push(u16_pos);
            }
        }
        BreakKind::Sentence => {
            let mut after_term = false;
            for &(ch, len) in &chars {
                if after_term && !ch.is_whitespace() && !matches!(ch, '.' | '!' | '?' | '"' | '\'' | ')') {
                    out.push(u16_pos);
                    after_term = false;
                }
                u16_pos += len;
                if matches!(ch, '.' | '!' | '?' | '。' | '！' | '？' | '\n') {
                    after_term = true;
                }
            }
            if out.last().copied() != Some(u16_pos) {
                out.push(u16_pos);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// ICU 69 Bidirectional Algorithm (`UBiDi`, backed by Apple libicucore.A.dylib)
// ---------------------------------------------------------------------------

enum BidiHandleBacking {
    HostBox(usize),
    Custom(CustomMemFuncs, usize),
}

impl BidiHandleBacking {
    unsafe fn release(self) {
        match self {
            BidiHandleBacking::HostBox(ptr) => {
                if ptr != 0 {
                    drop(Box::from_raw(ptr as *mut u8));
                }
            }
            BidiHandleBacking::Custom(mf, ptr) => {
                mf.free(ptr);
            }
        }
    }
}

#[derive(Clone, Copy)]
struct FallbackBidiRun {
    logical_start: i32,
    length: i32,
    direction: i32,
}

struct IcuBiDi {
    raw: *mut UBiDiOpaque,
    /// Owned immutable UTF-16 buffer kept alive for native `ubidi_*` queries.
    text_buf: Arc<[u16]>,
    /// Owned `embeddingLevels` buffer kept alive for native `ubidi_*` queries
    /// (`ubidi_setPara` retains the pointer and mutates levels in place).
    emb_buf: Option<Box<[u8]>>,
    max_length: i32,
    max_run_count: i32,
    has_para: bool,
    closed: bool,
    handle_backing: Option<BidiHandleBacking>,
    custom_mem: Option<CustomMemFuncs>,
    guest_buf_ptr: usize,
    guest_buf_cap: usize,
    fallback_dir: i32,
    fallback_runs: Vec<FallbackBidiRun>,
}

unsafe impl Send for IcuBiDi {}

impl Drop for IcuBiDi {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            if let Some(ubidi) = icu_bidi() {
                unsafe { (ubidi.ubidi_close)(self.raw) };
            }
            self.raw = std::ptr::null_mut();
        }
        self.text_buf = empty_u16_arc();
        self.emb_buf = None;
    }
}

fn bidi_table() -> &'static Mutex<HashMap<usize, Arc<Mutex<IcuBiDi>>>> {
    static TABLE: OnceLock<Mutex<HashMap<usize, Arc<Mutex<IcuBiDi>>>>> = OnceLock::new();
    TABLE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn lookup_bidi(p_bidi: *const c_void) -> Option<Arc<Mutex<IcuBiDi>>> {
    if p_bidi.is_null() {
        return None;
    }
    let tbl = bidi_table().lock().unwrap_or_else(|e| e.into_inner());
    tbl.get(&(p_bidi as usize)).map(Arc::clone)
}

fn classify_strong_bidi_dir(ch: char) -> Option<i32> {
    let cp = ch as u32;
    if cp == 0x200E {
        return Some(UBIDI_LTR);
    }
    if cp == 0x200F {
        return Some(UBIDI_RTL);
    }
    if matches!(
        cp,
        0x0590..=0x08FF
            | 0xFB1D..=0xFDFF
            | 0xFE70..=0xFEFF
            | 0x10800..=0x10FFF
            | 0x1E800..=0x1EFFF
    ) {
        return Some(UBIDI_RTL);
    }
    if ch.is_alphabetic() {
        return Some(UBIDI_LTR);
    }
    None
}

fn fallback_base_direction(units: &[u16]) -> i32 {
    let mut isolate_depth = 0usize;
    for res in char::decode_utf16(units.iter().copied()) {
        let Ok(ch) = res else { continue };
        match ch as u32 {
            0x2066..=0x2068 => isolate_depth += 1,
            0x2069 => isolate_depth = isolate_depth.saturating_sub(1),
            _ if isolate_depth == 0 => {
                if let Some(dir) = classify_strong_bidi_dir(ch) {
                    return dir;
                }
            }
            _ => {}
        }
    }
    UBIDI_NEUTRAL
}

fn fallback_compute_bidi(
    units: &[u16],
    para_level: u8,
    embedding_levels: Option<&[u8]>,
    max_length: i32,
    max_run_count: i32,
) -> Result<(i32, Vec<FallbackBidiRun>), i32> {
    if (para_level & 0x7F) > UBIDI_MAX_EXPLICIT_LEVEL
        && para_level != UBIDI_DEFAULT_LTR
        && para_level != UBIDI_DEFAULT_RTL
    {
        return Err(U_ILLEGAL_ARGUMENT_ERROR);
    }
    let len = units.len().min(i32::MAX as usize) as i32;
    if max_length > 0 && len > max_length {
        return Err(U_MEMORY_ALLOCATION_ERROR);
    }
    let base_level: u8 = match para_level {
        UBIDI_DEFAULT_LTR => {
            if fallback_base_direction(units) == UBIDI_RTL {
                1
            } else {
                0
            }
        }
        UBIDI_DEFAULT_RTL => {
            if fallback_base_direction(units) == UBIDI_LTR {
                0
            } else {
                1
            }
        }
        lv => lv & 0x7F,
    };
    let base_dir = if (base_level & 1) != 0 {
        UBIDI_RTL
    } else {
        UBIDI_LTR
    };
    if units.is_empty() {
        return Ok((base_dir, Vec::new()));
    }

    let mut levels: Vec<u8> = if let Some(emb) = embedding_levels {
        let mut out = Vec::with_capacity(units.len());
        let mut all_overridden = true;
        for &lv in emb.iter().take(units.len()) {
            let eff = lv & 0x7F;
            if eff > UBIDI_MAX_EXPLICIT_LEVEL {
                return Err(U_ILLEGAL_ARGUMENT_ERROR);
            }
            if (lv & 0x80) == 0 {
                all_overridden = false;
            }
            out.push(eff);
        }
        if !all_overridden {
            // Non-override per-character embeddingLevels require full UAX #9
            // explicit-level run resolution; fail honestly without libicucore.
            return Err(U_UNSUPPORTED_ERROR);
        }
        out
    } else if para_level < UBIDI_DEFAULT_LTR && (para_level & 0x80) != 0 {
        vec![base_level; units.len()]
    } else {
        if super::dwrite::analysis::has_explicit_bidi_controls(units) {
            return Err(U_UNSUPPORTED_ERROR);
        }
        super::dwrite::analysis::resolve_bidi_levels_fallback(units, base_level)
    };

    // UAX #9 L1: reset segment/paragraph separators and any immediately preceding
    // whitespace/BN (plus trailing whitespace/BN at end of paragraph) to base_level.
    if embedding_levels.is_none() && (para_level >= UBIDI_DEFAULT_LTR || (para_level & 0x80) == 0) {
        let is_seg_or_para_sep = |u: u16| {
            matches!(u, 0x0009..=0x000D | 0x001C..=0x001F | 0x0085 | 0x2029)
        };
        let is_ws_or_bn = |u: u16| {
            matches!(
                u,
                0x0009 | 0x000C | 0x0020 | 0x1680 | 0x2000..=0x200A | 0x2028 | 0x205F | 0x3000
                    | 0x0000..=0x0008
                    | 0x000E..=0x001B
                    | 0x007F..=0x0084
                    | 0x0086..=0x009F
                    | 0x200B..=0x200D
            )
        };
        let mut reset_run = true;
        for idx in (0..units.len()).rev() {
            let u = units[idx];
            if is_seg_or_para_sep(u) {
                levels[idx] = base_level;
                reset_run = true;
            } else if reset_run && is_ws_or_bn(u) {
                levels[idx] = base_level;
            } else {
                reset_run = false;
            }
        }
    }

    // Group contiguous equal-level spans into logical runs, then reorder per UAX #9 L2.
    let mut level_runs: Vec<(i32, i32, u8)> = Vec::new();
    let mut start = 0usize;
    while start < levels.len() {
        let lv = levels[start];
        let mut end = start + 1;
        while end < levels.len() && levels[end] == lv {
            end += 1;
        }
        level_runs.push((start as i32, (end - start) as i32, lv));
        start = end;
    }
    if max_run_count > 0 && (level_runs.len() as i32) > max_run_count {
        return Err(U_MEMORY_ALLOCATION_ERROR);
    }

    let max_lv = level_runs.iter().map(|r| r.2).max().unwrap_or(0);
    let min_odd_lv = level_runs
        .iter()
        .map(|r| r.2)
        .filter(|&lv| (lv & 1) != 0)
        .min();
    if let Some(min_odd) = min_odd_lv {
        for lv in (min_odd..=max_lv).rev() {
            let mut i = 0usize;
            while i < level_runs.len() {
                if level_runs[i].2 >= lv {
                    let s = i;
                    while i < level_runs.len() && level_runs[i].2 >= lv {
                        i += 1;
                    }
                    level_runs[s..i].reverse();
                } else {
                    i += 1;
                }
            }
        }
    }

    let runs: Vec<FallbackBidiRun> = level_runs
        .into_iter()
        .map(|(logical_start, length, lv)| FallbackBidiRun {
            logical_start,
            length,
            direction: if (lv & 1) != 0 { UBIDI_RTL } else { UBIDI_LTR },
        })
        .collect();
    let dir = if runs.iter().all(|r| r.direction == base_dir) {
        base_dir
    } else {
        UBIDI_MIXED
    };
    Ok((dir, runs))
}

impl IcuBiDi {
    fn count_runs(&mut self, status: &mut i32) -> i32 {
        if !self.raw.is_null() {
            if let Some(ubidi) = icu_bidi() {
                return unsafe { (ubidi.ubidi_count_runs)(self.raw, status) };
            }
        }
        if !self.has_para {
            *status = U_INVALID_STATE_ERROR;
            return -1;
        }
        self.fallback_runs.len() as i32
    }

    fn direction(&self) -> i32 {
        if !self.raw.is_null() {
            if let Some(ubidi) = icu_bidi() {
                return unsafe { (ubidi.ubidi_get_direction)(self.raw) };
            }
        }
        self.fallback_dir
    }

    fn visual_run_unchecked(&mut self, run_index: i32) -> (i32, i32, i32) {
        if !self.raw.is_null() {
            if let Some(ubidi) = icu_bidi() {
                let mut logical_start: i32 = 0;
                let mut run_len: i32 = 0;
                let dir = unsafe {
                    (ubidi.ubidi_get_visual_run)(
                        self.raw,
                        run_index,
                        &mut logical_start,
                        &mut run_len,
                    )
                };
                return (dir, logical_start, run_len);
            }
        }
        let run = self.fallback_runs[run_index as usize];
        (run.direction, run.logical_start, run.length)
    }
}

// ---------------------------------------------------------------------------
// Exported ICU 69 C / C++ entry points
// ---------------------------------------------------------------------------

fn new_break_iterator(kind: BreakKind, locale_ptr: *const c_void, err: *mut i32) -> *mut c_void {
    ensure_vtbl_registered();
    if !err.is_null() {
        unsafe {
            if std::ptr::read_unaligned(err) > 0 {
                return std::ptr::null_mut();
            }
            std::ptr::write_unaligned(err, U_ZERO_ERROR);
        }
    }
    let locale = unsafe { lookup_locale(locale_ptr) };
    let Some(iter) = IcuBreakIterator::new(kind, locale) else {
        if !err.is_null() {
            unsafe { std::ptr::write_unaligned(err, U_MEMORY_ALLOCATION_ERROR) };
        }
        return std::ptr::null_mut();
    };
    let b = Box::new(iter);
    Box::into_raw(b) as *mut c_void
}

pub unsafe extern "win64" fn udata_setCommonData_69(_data: *const c_void, err: *mut i32) {
    ensure_vtbl_registered();
    if !err.is_null() && std::ptr::read_unaligned(err) <= 0 {
        std::ptr::write_unaligned(err, U_ZERO_ERROR);
    }
}

pub unsafe extern "win64" fn u_setMemoryFunctions_69(
    context: *const c_void,
    a: *const c_void,
    r: *const c_void,
    f: *const c_void,
    status: *mut i32,
) {
    ensure_vtbl_registered();
    if status.is_null() || std::ptr::read_unaligned(status) > 0 {
        return;
    }
    if a.is_null() || r.is_null() || f.is_null() {
        std::ptr::write_unaligned(status, U_ILLEGAL_ARGUMENT_ERROR);
        return;
    }
    let mut st = mem_func_state().lock().unwrap_or_else(|e| e.into_inner());
    if st.heap_in_use {
        drop(st);
        std::ptr::write_unaligned(status, U_INVALID_STATE_ERROR);
        return;
    }
    st.custom = Some(CustomMemFuncs {
        context: context as usize,
        alloc_fn: a as usize,
        realloc_fn: r as usize,
        free_fn: f as usize,
    });
    drop(st);
    std::ptr::write_unaligned(status, U_ZERO_ERROR);
}

pub unsafe extern "win64" fn ubidi_openSized_69(
    max_length: i32,
    max_run_count: i32,
    p_error_code: *mut i32,
) -> *mut c_void {
    ensure_vtbl_registered();
    if p_error_code.is_null() || std::ptr::read_unaligned(p_error_code) > 0 {
        return std::ptr::null_mut();
    }
    if max_length < 0 || max_run_count < 0 {
        std::ptr::write_unaligned(p_error_code, U_ILLEGAL_ARGUMENT_ERROR);
        return std::ptr::null_mut();
    }

    let mut icu_err: i32 = U_ZERO_ERROR;
    let raw = if let Some(ubidi) = icu_bidi() {
        let h = (ubidi.ubidi_open_sized)(max_length, max_run_count, &mut icu_err);
        if h.is_null() || icu_err > 0 {
            if !h.is_null() {
                (ubidi.ubidi_close)(h);
            }
            let code = if icu_err > 0 {
                icu_err
            } else {
                U_MEMORY_ALLOCATION_ERROR
            };
            std::ptr::write_unaligned(p_error_code, code);
            return std::ptr::null_mut();
        }
        h
    } else {
        std::ptr::null_mut()
    };

    let custom_mem = acquire_custom_mem_funcs();
    let (handle_addr, backing) = if let Some(mf) = custom_mem {
        let initial_bytes = 16usize
            .saturating_add((max_length as usize).saturating_mul(2))
            .saturating_add((max_run_count as usize).saturating_mul(8))
            .min(64 * 1024 * 1024);
        let ptr = mf.alloc(initial_bytes);
        if ptr == 0 {
            if !raw.is_null() {
                if let Some(ubidi) = icu_bidi() {
                    (ubidi.ubidi_close)(raw);
                }
            }
            std::ptr::write_unaligned(p_error_code, U_MEMORY_ALLOCATION_ERROR);
            return std::ptr::null_mut();
        }
        (ptr, BidiHandleBacking::Custom(mf, ptr))
    } else {
        let ptr = Box::into_raw(Box::new(0u8)) as usize;
        (ptr, BidiHandleBacking::HostBox(ptr))
    };

    let entry = Arc::new(Mutex::new(IcuBiDi {
        raw,
        text_buf: empty_u16_arc(),
        emb_buf: None,
        max_length,
        max_run_count,
        has_para: false,
        closed: false,
        handle_backing: Some(backing),
        custom_mem,
        guest_buf_ptr: 0,
        guest_buf_cap: 0,
        fallback_dir: UBIDI_LTR,
        fallback_runs: Vec::new(),
    }));

    let mut tbl = bidi_table().lock().unwrap_or_else(|e| e.into_inner());
    if tbl.contains_key(&handle_addr) {
        drop(tbl);
        // Defend against a buggy custom allocator returning a live address without corrupting
        // the existing handle entry.
        let mut guard = entry.lock().unwrap_or_else(|e| e.into_inner());
        let _ = guard.handle_backing.take();
        drop(guard);
        std::ptr::write_unaligned(p_error_code, U_MEMORY_ALLOCATION_ERROR);
        return std::ptr::null_mut();
    }
    tbl.insert(handle_addr, entry);
    drop(tbl);

    std::ptr::write_unaligned(p_error_code, icu_err);
    handle_addr as *mut c_void
}

pub unsafe extern "win64" fn ubidi_close_69(p_bidi: *mut c_void) {
    if p_bidi.is_null() {
        return;
    }
    let removed = {
        let mut tbl = bidi_table().lock().unwrap_or_else(|e| e.into_inner());
        tbl.remove(&(p_bidi as usize))
    };
    let Some(entry) = removed else {
        log::warn!("cohtml_icuuc: ubidi_close_69 called with unknown handle {:p}", p_bidi);
        return;
    };

    let (guest_buf_to_free, handle_backing) = {
        let mut bidi = entry.lock().unwrap_or_else(|e| e.into_inner());
        bidi.closed = true;
        if !bidi.raw.is_null() {
            if let Some(ubidi) = icu_bidi() {
                (ubidi.ubidi_close)(bidi.raw);
            }
            bidi.raw = std::ptr::null_mut();
        }
        // Release text and embedding buffers strictly AFTER `ubidi_close` completes.
        bidi.text_buf = empty_u16_arc();
        bidi.emb_buf = None;
        bidi.fallback_runs.clear();
        let buf_free = if bidi.guest_buf_ptr != 0 {
            let ptr = bidi.guest_buf_ptr;
            bidi.guest_buf_ptr = 0;
            bidi.guest_buf_cap = 0;
            bidi.custom_mem.map(|mf| (mf, ptr))
        } else {
            None
        };
        (buf_free, bidi.handle_backing.take())
    };

    if let Some((mf, ptr)) = guest_buf_to_free {
        mf.free(ptr);
    }
    if let Some(backing) = handle_backing {
        backing.release();
    }
}

pub unsafe extern "win64" fn ubidi_setPara_69(
    p_bidi: *mut c_void,
    text: *const u16,
    length: i32,
    para_level: u8,
    embedding_levels: *mut u8,
    p_error_code: *mut i32,
) {
    ensure_vtbl_registered();
    if p_error_code.is_null() || std::ptr::read_unaligned(p_error_code) > 0 {
        return;
    }
    let Some(entry) = lookup_bidi(p_bidi) else {
        std::ptr::write_unaligned(p_error_code, U_ILLEGAL_ARGUMENT_ERROR);
        return;
    };
    if text.is_null() || length < -1 {
        std::ptr::write_unaligned(p_error_code, U_ILLEGAL_ARGUMENT_ERROR);
        return;
    }

    let text_buf = read_guest_utf16(text, length);
    let actual_len = text_buf.len().min(i32::MAX as usize) as i32;
    let emb_buf: Option<Box<[u8]>> = if !embedding_levels.is_null() && actual_len > 0 {
        let n = actual_len as usize;
        let mut v = Vec::with_capacity(n);
        for i in 0..n {
            v.push(std::ptr::read_unaligned(embedding_levels.add(i)));
        }
        Some(v.into_boxed_slice())
    } else {
        None
    };

    // If custom ICU memory callbacks are installed on a dynamically-sized UBiDi
    // (`max_length == 0`), grow its guest staging buffer without holding any lock.
    let text_bytes = text_buf.len().saturating_mul(std::mem::size_of::<u16>());
    let grow_req = {
        let mut bidi = entry.lock().unwrap_or_else(|e| e.into_inner());
        if bidi.closed {
            std::ptr::write_unaligned(p_error_code, U_INVALID_STATE_ERROR);
            return;
        }
        if bidi.max_length == 0 && text_bytes > bidi.guest_buf_cap {
            bidi.custom_mem.map(|mf| {
                let old_ptr = bidi.guest_buf_ptr;
                let old_cap = bidi.guest_buf_cap;
                // Take ownership across `call_guest8` so a concurrent `ubidi_close_69`
                // cannot double-free `old_ptr`.
                bidi.guest_buf_ptr = 0;
                bidi.guest_buf_cap = 0;
                (mf, old_ptr, old_cap)
            })
        } else {
            None
        }
    };

    if let Some((mf, old_ptr, old_cap)) = grow_req {
        let new_ptr = if old_ptr == 0 {
            mf.alloc(text_bytes)
        } else {
            mf.realloc(old_ptr, text_bytes)
        };
        if new_ptr == 0 {
            let free_old_after_unlock = {
                let mut bidi = entry.lock().unwrap_or_else(|e| e.into_inner());
                if old_ptr != 0 {
                    if !bidi.closed {
                        bidi.guest_buf_ptr = old_ptr;
                        bidi.guest_buf_cap = old_cap;
                        false
                    } else {
                        true
                    }
                } else {
                    false
                }
            };
            if free_old_after_unlock {
                mf.free(old_ptr);
            }
            std::ptr::write_unaligned(p_error_code, U_MEMORY_ALLOCATION_ERROR);
            return;
        }
        let closed_during_alloc = {
            let mut bidi = entry.lock().unwrap_or_else(|e| e.into_inner());
            if bidi.closed {
                true
            } else {
                bidi.guest_buf_ptr = new_ptr;
                bidi.guest_buf_cap = text_bytes;
                false
            }
        };
        if closed_during_alloc {
            mf.free(new_ptr);
            std::ptr::write_unaligned(p_error_code, U_INVALID_STATE_ERROR);
            return;
        }
    }

    let mut bidi = entry.lock().unwrap_or_else(|e| e.into_inner());
    if bidi.closed {
        std::ptr::write_unaligned(p_error_code, U_INVALID_STATE_ERROR);
        return;
    }

    if !bidi.raw.is_null() {
        if let Some(ubidi) = icu_bidi() {
            let old_buf = std::mem::replace(&mut bidi.text_buf, text_buf);
            let old_emb = std::mem::replace(&mut bidi.emb_buf, emb_buf);
            let text_ptr = if actual_len == 0 {
                EMPTY_U16.as_ptr()
            } else {
                bidi.text_buf.as_ptr()
            };
            let emb_ptr = bidi
                .emb_buf
                .as_deref_mut()
                .map_or(std::ptr::null_mut(), <[u8]>::as_mut_ptr);
            let mut icu_err: i32 = U_ZERO_ERROR;
            (ubidi.ubidi_set_para)(
                bidi.raw,
                text_ptr,
                actual_len,
                para_level,
                emb_ptr,
                &mut icu_err,
            );
            if icu_err > 0 {
                // Keep the previously referenced buffers alive if libicucore retained
                // the prior paragraph on error.
                bidi.text_buf = old_buf;
                bidi.emb_buf = old_emb;
            } else {
                bidi.has_para = true;
                if let Some(ref resolved) = bidi.emb_buf {
                    if !embedding_levels.is_null() {
                        for (i, &lv) in resolved.iter().enumerate() {
                            std::ptr::write_unaligned(embedding_levels.add(i), lv);
                        }
                    }
                }
            }
            std::ptr::write_unaligned(p_error_code, icu_err);
            return;
        }
    }

    match fallback_compute_bidi(
        &text_buf,
        para_level,
        emb_buf.as_deref(),
        bidi.max_length,
        bidi.max_run_count,
    ) {
        Ok((dir, runs)) => {
            bidi.text_buf = text_buf;
            bidi.emb_buf = emb_buf.map(|mut b| {
                for lv in b.iter_mut() {
                    *lv &= 0x7F;
                }
                if !embedding_levels.is_null() {
                    for (i, &lv) in b.iter().enumerate() {
                        std::ptr::write_unaligned(embedding_levels.add(i), lv);
                    }
                }
                b
            });
            bidi.fallback_dir = dir;
            bidi.fallback_runs = runs;
            bidi.has_para = true;
            std::ptr::write_unaligned(p_error_code, U_ZERO_ERROR);
        }
        Err(code) => {
            std::ptr::write_unaligned(p_error_code, code);
        }
    }
}

pub unsafe extern "win64" fn ubidi_getDirection_69(p_bidi: *const c_void) -> i32 {
    let Some(entry) = lookup_bidi(p_bidi) else {
        return UBIDI_LTR;
    };
    let bidi = entry.lock().unwrap_or_else(|e| e.into_inner());
    if bidi.closed {
        return UBIDI_LTR;
    }
    bidi.direction()
}

pub unsafe extern "win64" fn ubidi_getBaseDirection_69(text: *const u16, length: i32) -> i32 {
    ensure_vtbl_registered();
    if text.is_null() || length == 0 || length < -1 {
        return UBIDI_NEUTRAL;
    }
    let buf = read_guest_utf16(text, length);
    if buf.is_empty() {
        return UBIDI_NEUTRAL;
    }
    if let Some(ubidi) = icu_bidi() {
        return (ubidi.ubidi_get_base_direction)(
            buf.as_ptr(),
            buf.len().min(i32::MAX as usize) as i32,
        );
    }
    fallback_base_direction(&buf)
}

pub unsafe extern "win64" fn ubidi_countRuns_69(
    p_bidi: *mut c_void,
    p_error_code: *mut i32,
) -> i32 {
    if p_error_code.is_null() || std::ptr::read_unaligned(p_error_code) > 0 {
        return -1;
    }
    let Some(entry) = lookup_bidi(p_bidi) else {
        std::ptr::write_unaligned(p_error_code, U_INVALID_STATE_ERROR);
        return -1;
    };
    let mut bidi = entry.lock().unwrap_or_else(|e| e.into_inner());
    if bidi.closed {
        std::ptr::write_unaligned(p_error_code, U_INVALID_STATE_ERROR);
        return -1;
    }
    let mut icu_err: i32 = U_ZERO_ERROR;
    let count = bidi.count_runs(&mut icu_err);
    std::ptr::write_unaligned(p_error_code, icu_err);
    count
}

pub unsafe extern "win64" fn ubidi_getVisualRun_69(
    p_bidi: *mut c_void,
    run_index: i32,
    p_logical_start: *mut i32,
    p_length: *mut i32,
) -> i32 {
    if p_bidi.is_null() || run_index < 0 {
        return UBIDI_LTR;
    }
    let Some(entry) = lookup_bidi(p_bidi) else {
        return UBIDI_LTR;
    };
    let mut bidi = entry.lock().unwrap_or_else(|e| e.into_inner());
    if bidi.closed {
        return UBIDI_LTR;
    }
    let mut err: i32 = U_ZERO_ERROR;
    let count = bidi.count_runs(&mut err);
    if err > 0 || run_index >= count {
        return UBIDI_LTR;
    }
    let (dir, logical_start, run_len) = bidi.visual_run_unchecked(run_index);
    if !p_logical_start.is_null() {
        std::ptr::write_unaligned(p_logical_start, logical_start);
    }
    if !p_length.is_null() {
        std::ptr::write_unaligned(p_length, run_len);
    }
    dir
}

// --- icu_69::Locale exports ---

unsafe extern "win64" fn locale_ctor_default(this: *mut usize) -> *mut usize {
    ensure_vtbl_registered();
    store_locale(this, LocaleData::default_locale())
}

unsafe extern "win64" fn locale_ctor(
    this: *mut usize,
    language: *const u8,
    country: *const u8,
    variant: *const u8,
    keywords: *const u8,
) -> *mut usize {
    ensure_vtbl_registered();
    let lang_s = read_guest_cstr_string(language);
    let country_s = read_guest_cstr_string(country);
    let variant_s = read_guest_cstr_string(variant);
    let kw_s = read_guest_cstr_string(keywords);
    let data = LocaleData::from_parts(
        lang_s.as_deref(),
        country_s.as_deref(),
        variant_s.as_deref(),
        kw_s.as_deref(),
    );
    store_locale(this, data)
}

unsafe extern "win64" fn locale_ctor_copy(this: *mut usize, other: *const c_void) -> *mut usize {
    ensure_vtbl_registered();
    let src = lookup_locale(other);
    let cloned = LocaleData {
        language: src.language.clone(),
        country: src.country.clone(),
        variant: src.variant.clone(),
        canonical: src.canonical.clone(),
    };
    store_locale(this, cloned)
}

unsafe extern "win64" fn locale_dtor(this: *mut usize) {
    if this.is_null() {
        return;
    }
    let raw = std::ptr::read_unaligned(this) as u64;
    let mut tbl = locale_table().lock().unwrap_or_else(|e| e.into_inner());
    tbl.remove(LOCALE_HANDLE_TAG, raw, this as usize);
    std::ptr::write_unaligned(this, 0);
}

unsafe extern "win64" fn locale_get_name(this: *const c_void) -> *const c_char {
    lookup_locale_cstr_ptr(this, |d| &d.canonical)
}

unsafe extern "win64" fn locale_get_language(this: *const c_void) -> *const c_char {
    lookup_locale_cstr_ptr(this, |d| &d.language)
}

unsafe extern "win64" fn locale_get_country(this: *const c_void) -> *const c_char {
    lookup_locale_cstr_ptr(this, |d| &d.country)
}

unsafe extern "win64" fn locale_get_variant(this: *const c_void) -> *const c_char {
    lookup_locale_cstr_ptr(this, |d| &d.variant)
}

unsafe extern "win64" fn locale_get_default() -> *const usize {
    static SLOT: OnceLock<StaticSlot> = OnceLock::new();
    static_locale_slot(&SLOT, LocaleData::default_locale)
}

unsafe extern "win64" fn locale_get_us() -> *const usize {
    static SLOT: OnceLock<StaticSlot> = OnceLock::new();
    static_locale_slot(&SLOT, || {
        LocaleData::from_parts(Some("en"), Some("US"), None, None)
    })
}

unsafe extern "win64" fn locale_get_root() -> *const usize {
    static SLOT: OnceLock<StaticSlot> = OnceLock::new();
    static_locale_slot(&SLOT, || LocaleData::from_parts(Some(""), None, None, None))
}

// --- icu_69::UnicodeString exports ---

unsafe extern "win64" fn unicode_string_ctor_empty(this: *mut usize) -> *mut usize {
    ensure_vtbl_registered();
    store_unicode_string(this, empty_u16_arc())
}

unsafe extern "win64" fn unicode_string_ctor(
    this: *mut usize,
    codepage_data: *const u8,
) -> *mut usize {
    ensure_vtbl_registered();
    let u16s = read_guest_utf8_to_u16(codepage_data, -1);
    store_unicode_string(this, u16s)
}

unsafe extern "win64" fn unicode_string_ctor_utf8_len(
    this: *mut usize,
    codepage_data: *const u8,
    data_length: i32,
) -> *mut usize {
    ensure_vtbl_registered();
    let u16s = read_guest_utf8_to_u16(codepage_data, data_length);
    store_unicode_string(this, u16s)
}

unsafe extern "win64" fn unicode_string_ctor_codepage(
    this: *mut usize,
    src: *const u8,
    length: i32,
    codepage: *const u8,
) -> *mut usize {
    ensure_vtbl_registered();
    let cp = read_guest_cstr_string(codepage).unwrap_or_default();
    let u16s = if cp.eq_ignore_ascii_case("ISO-8859-1")
        || cp.eq_ignore_ascii_case("ISO8859-1")
        || cp.eq_ignore_ascii_case("latin1")
    {
        read_guest_latin1_to_u16(src, length)
    } else {
        read_guest_utf8_to_u16(src, length)
    };
    store_unicode_string(this, u16s)
}

unsafe extern "win64" fn unicode_string_ctor_u16(
    this: *mut usize,
    text: *const u16,
) -> *mut usize {
    ensure_vtbl_registered();
    let u16s = read_guest_utf16(text, -1);
    store_unicode_string(this, u16s)
}

unsafe extern "win64" fn unicode_string_ctor_u16_len(
    this: *mut usize,
    text: *const u16,
    text_length: i32,
) -> *mut usize {
    ensure_vtbl_registered();
    let u16s = read_guest_utf16(text, text_length);
    store_unicode_string(this, u16s)
}

unsafe extern "win64" fn unicode_string_ctor_terminated_u16(
    this: *mut usize,
    _is_terminated: i8,
    text: *const u16,
    text_length: i32,
) -> *mut usize {
    ensure_vtbl_registered();
    let u16s = read_guest_utf16(text, text_length);
    store_unicode_string(this, u16s)
}

unsafe extern "win64" fn unicode_string_ctor_copy(
    this: *mut usize,
    other: *const usize,
) -> *mut usize {
    ensure_vtbl_registered();
    let u16s = lookup_unicode_string(other);
    store_unicode_string(this, u16s)
}

unsafe extern "win64" fn unicode_string_dtor(this: *mut usize) {
    if this.is_null() {
        return;
    }
    let raw = std::ptr::read_unaligned(this) as u64;
    let mut tbl = ustr_table().lock().unwrap_or_else(|e| e.into_inner());
    tbl.remove(USTR_HANDLE_TAG, raw, this as usize);
    std::ptr::write_unaligned(this, 0);
}

/// `int32_t icu_69::UnicodeString::length() const`
unsafe extern "win64" fn unicode_string_length(this: *const usize) -> i32 {
    lookup_unicode_string(this).len().min(i32::MAX as usize) as i32
}

/// `static UnicodeString icu_69::UnicodeString::fromUTF8(StringPiece utf8)`
/// On MSVC x64: `rcx` = hidden sret buffer (`*mut UnicodeString`), `rdx` = `*const StringPiece`
/// (`{ ptr: *const u8, length: i32 }`), returns `out` in `rax`.
unsafe extern "win64" fn unicode_string_from_utf8(
    out: *mut usize,
    string_piece: *const u8,
) -> *mut usize {
    ensure_vtbl_registered();
    let u16s = if string_piece.is_null() {
        empty_u16_arc()
    } else {
        let data_ptr = std::ptr::read_unaligned(string_piece as *const *const u8);
        let data_len = std::ptr::read_unaligned(string_piece.add(8) as *const i32);
        if data_len <= 0 {
            empty_u16_arc()
        } else {
            read_guest_utf8_to_u16(data_ptr, data_len)
        }
    };
    store_unicode_string(out, u16s)
}

// --- icu_69::BreakIterator factories ---

unsafe extern "win64" fn create_character_instance(
    locale: *const c_void,
    err: *mut i32,
) -> *mut c_void {
    new_break_iterator(BreakKind::Character, locale, err)
}

unsafe extern "win64" fn create_line_instance(
    locale: *const c_void,
    err: *mut i32,
) -> *mut c_void {
    new_break_iterator(BreakKind::Line, locale, err)
}

unsafe extern "win64" fn create_word_instance(
    locale: *const c_void,
    err: *mut i32,
) -> *mut c_void {
    new_break_iterator(BreakKind::Word, locale, err)
}

unsafe extern "win64" fn create_sentence_instance(
    locale: *const c_void,
    err: *mut i32,
) -> *mut c_void {
    new_break_iterator(BreakKind::Sentence, locale, err)
}

#[cfg(test)]
mod tests {
    use super::*;

    unsafe fn collect_breaks(bi: *mut IcuBreakIterator) -> Vec<i32> {
        let mut out = vec![brk_first(bi)];
        loop {
            let pos = brk_next(bi);
            if pos == UBRK_DONE {
                break;
            }
            out.push(pos);
        }
        out
    }

    #[test]
    fn apple_libicucore_loads() {
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        assert!(
            icu_core().is_some(),
            "expected /usr/lib/libicucore.A.dylib to load on Apple platforms"
        );
    }

    #[test]
    fn locale_fields_canonicalization_and_cleanup() {
        unsafe {
            let mut loc_en: usize = 0;
            locale_ctor(
                &mut loc_en,
                b"en\0".as_ptr(),
                b"US\0".as_ptr(),
                std::ptr::null(),
                std::ptr::null(),
            );
            assert_eq!(
                CStr::from_ptr(locale_get_name(&loc_en as *const usize as *const c_void))
                    .to_str()
                    .unwrap(),
                "en_US"
            );
            assert_eq!(
                CStr::from_ptr(locale_get_language(&loc_en as *const usize as *const c_void))
                    .to_str()
                    .unwrap(),
                "en"
            );
            assert_eq!(
                CStr::from_ptr(locale_get_country(&loc_en as *const usize as *const c_void))
                    .to_str()
                    .unwrap(),
                "US"
            );
            assert_eq!(
                CStr::from_ptr(locale_get_variant(&loc_en as *const usize as *const c_void))
                    .to_str()
                    .unwrap(),
                ""
            );

            let mut loc_ja: usize = 0;
            locale_ctor(
                &mut loc_ja,
                b"ja-JP@lb=strict\0".as_ptr(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
            );
            assert_eq!(
                CStr::from_ptr(locale_get_name(&loc_ja as *const usize as *const c_void))
                    .to_str()
                    .unwrap(),
                "ja_JP@lb=strict"
            );

            let mut loc_copy: usize = 0;
            locale_ctor_copy(&mut loc_copy, &loc_ja as *const usize as *const c_void);
            locale_dtor(&mut loc_ja);
            assert_eq!(loc_ja, 0);
            assert_eq!(
                CStr::from_ptr(locale_get_name(&loc_copy as *const usize as *const c_void))
                    .to_str()
                    .unwrap(),
                "ja_JP@lb=strict"
            );
            locale_dtor(&mut loc_copy);
            locale_dtor(&mut loc_en);
        }
    }

    #[test]
    fn unicode_string_utf8_utf16_from_utf8_and_lifetime() {
        unsafe {
            // Counted UTF-8 (not NUL-terminated)
            let raw = b"Hello world EXTRA";
            let mut u8_str: usize = 0;
            unicode_string_ctor_utf8_len(&mut u8_str, raw.as_ptr(), 5);
            let got = lookup_unicode_string(&u8_str);
            let expected_hello: Vec<u16> = "Hello".encode_utf16().collect();
            assert_eq!(&*got, &expected_hello[..]);
            unicode_string_dtor(&mut u8_str);
            assert_eq!(u8_str, 0);

            // Unaligned UTF-16 input (explicit 1-byte misalignment)
            let utf16_units: Vec<u16> = "Café🇯🇵".encode_utf16().collect();
            let mut unaligned = vec![0xAAu8; 1 + utf16_units.len() * 2];
            for (i, cu) in utf16_units.iter().enumerate() {
                let le = cu.to_le_bytes();
                unaligned[1 + i * 2] = le[0];
                unaligned[1 + i * 2 + 1] = le[1];
            }
            let mut u16_str: usize = 0;
            unicode_string_ctor_u16_len(
                &mut u16_str,
                unaligned.as_ptr().add(1) as *const u16,
                utf16_units.len() as i32,
            );
            assert_eq!(&*lookup_unicode_string(&u16_str), &utf16_units[..]);
            unicode_string_dtor(&mut u16_str);

            // UnicodeString::fromUTF8(StringPiece)
            #[repr(C)]
            struct TestStringPiece {
                ptr: *const u8,
                length: i32,
                _pad: i32,
            }
            let src_bytes = "日本語".as_bytes();
            let sp = TestStringPiece {
                ptr: src_bytes.as_ptr(),
                length: src_bytes.len() as i32,
                _pad: 0,
            };
            let mut from_u8: usize = 0;
            let ret = unicode_string_from_utf8(&mut from_u8, &sp as *const TestStringPiece as *const u8);
            assert_eq!(ret, &mut from_u8 as *mut usize);
            let expected_ja: Vec<u16> = "日本語".encode_utf16().collect();
            assert_eq!(&*lookup_unicode_string(&from_u8), &expected_ja[..]);
            unicode_string_dtor(&mut from_u8);
        }
    }

    #[test]
    fn break_iterator_uax29_grapheme_clusters_emoji_and_combining() {
        unsafe {
            let mut err: i32 = 0;
            let bi = create_character_instance(locale_get_us() as *const c_void, &mut err)
                as *mut IcuBreakIterator;
            assert!(!bi.is_null());
            assert_eq!(err, 0);

            // "e" + U+0301 (2 UTF-16 units) + ZWJ family emoji (11 UTF-16 units) + "a" (1 UTF-16 unit) = 14 UTF-16 units
            let text = "e\u{0301}\u{1F468}\u{200D}\u{1F469}\u{200D}\u{1F467}\u{200D}\u{1F466}a\0";
            let mut ustr: usize = 0;
            unicode_string_ctor(&mut ustr, text.as_ptr());
            brk_set_text(bi, &ustr);
            // Destroy temporary UnicodeString immediately; BreakIterator must retain its own UTF-16 buffer.
            unicode_string_dtor(&mut ustr);

            let breaks = collect_breaks(bi);
            assert_eq!(breaks, vec![0, 2, 13, 14]);

            brk_scalar_deleting_dtor(bi, 1);
        }
    }

    #[test]
    fn break_iterator_word_clone_and_rule_status() {
        unsafe {
            let mut err: i32 = 0;
            let proto = create_word_instance(std::ptr::null(), &mut err) as *mut IcuBreakIterator;
            assert!(!proto.is_null());
            assert_eq!(err, 0);

            let cloned = brk_clone(proto);
            assert!(!cloned.is_null());

            let mut ustr: usize = 0;
            unicode_string_ctor(&mut ustr, b"Hello World 2!\0".as_ptr());
            brk_set_text(cloned, &ustr);
            unicode_string_dtor(&mut ustr);

            assert_eq!(brk_first(cloned), 0);
            assert_eq!(brk_next(cloned), 5); // "Hello"
            if icu_core().is_some() {
                assert!(brk_get_rule_status(cloned) >= 200); // UBRK_WORD_LETTER
                let mut vec_buf = [0i32; 4];
                let mut vec_err = 0i32;
                let count = brk_get_rule_status_vec(cloned, vec_buf.as_mut_ptr(), 4, &mut vec_err);
                assert!(count >= 1);
                assert_eq!(vec_err, 0);
                assert!(vec_buf[0] >= 200);
            }

            // Clone mid-iteration and verify independent progress from the cloned position
            let mid_clone = brk_clone(cloned);
            assert!(!mid_clone.is_null());
            assert_eq!(brk_current(mid_clone), 5);
            assert_eq!(brk_equals(cloned, mid_clone), 1);

            assert_eq!(brk_next(cloned), 6); // " "
            assert_eq!(brk_next(cloned), 11); // "World"
            assert_eq!(brk_next(cloned), 12); // " "
            assert_eq!(brk_next(cloned), 13); // "2"
            if icu_core().is_some() {
                let st = brk_get_rule_status(cloned);
                assert!((100..200).contains(&st)); // UBRK_WORD_NUMBER
            }
            assert_eq!(brk_next(cloned), 14); // "!"
            assert_eq!(brk_next(cloned), UBRK_DONE);

            // `mid_clone` must be unaffected by advancing `cloned`
            assert_eq!(brk_current(mid_clone), 5);
            assert_eq!(brk_equals(cloned, mid_clone), 0);
            assert_eq!(brk_next(mid_clone), 6);

            brk_scalar_deleting_dtor(mid_clone, 1);
            brk_scalar_deleting_dtor(cloned, 1);
            brk_scalar_deleting_dtor(proto, 1);
        }
    }

    #[test]
    fn break_iterator_cjk_line_and_word_breaking() {
        unsafe {
            // Japanese UAX #14 line breaking (no spaces, Kinsoku rule forbids break before "。")
            let mut loc_ja: usize = 0;
            locale_ctor(
                &mut loc_ja,
                b"ja\0".as_ptr(),
                b"JP\0".as_ptr(),
                std::ptr::null(),
                std::ptr::null(),
            );
            let mut err: i32 = 0;
            let bi_ja = create_line_instance(&loc_ja as *const usize as *const c_void, &mut err)
                as *mut IcuBreakIterator;
            assert!(!bi_ja.is_null());
            assert_eq!(err, 0);

            let ja_text = "日本語のテキストを折り返します。\0";
            let mut ustr_ja: usize = 0;
            unicode_string_ctor(&mut ustr_ja, ja_text.as_ptr());
            brk_set_text(bi_ja, &ustr_ja);
            unicode_string_dtor(&mut ustr_ja);

            let ja_breaks = collect_breaks(bi_ja);
            assert_eq!(ja_breaks.first().copied(), Some(0));
            assert_eq!(ja_breaks.last().copied(), Some(16));
            assert!(
                ja_breaks.len() > 5,
                "expected CJK line break opportunities without spaces, got {ja_breaks:?}"
            );
            assert!(
                !ja_breaks.contains(&15),
                "UAX #14 must not break immediately before ideographic full stop '。', got {ja_breaks:?}"
            );
            brk_scalar_deleting_dtor(bi_ja, 1);
            locale_dtor(&mut loc_ja);

            // Chinese UAX #14 line breaking and dictionary word breaking
            let mut loc_zh: usize = 0;
            locale_ctor(
                &mut loc_zh,
                b"zh\0".as_ptr(),
                b"CN\0".as_ptr(),
                std::ptr::null(),
                std::ptr::null(),
            );
            let mut err_zh: i32 = 0;
            let bi_zh_line =
                create_line_instance(&loc_zh as *const usize as *const c_void, &mut err_zh)
                    as *mut IcuBreakIterator;
            assert!(!bi_zh_line.is_null());
            let mut ustr_zh: usize = 0;
            unicode_string_ctor(&mut ustr_zh, "今天天气非常好。\0".as_ptr());
            brk_set_text(bi_zh_line, &ustr_zh);
            let zh_line_breaks = collect_breaks(bi_zh_line);
            assert_eq!(zh_line_breaks.first().copied(), Some(0));
            assert_eq!(zh_line_breaks.last().copied(), Some(8));
            assert!(zh_line_breaks.len() > 3);
            assert!(!zh_line_breaks.contains(&7));
            brk_scalar_deleting_dtor(bi_zh_line, 1);
            unicode_string_dtor(&mut ustr_zh);

            if icu_core().is_some() {
                let mut err_w: i32 = 0;
                let bi_zh_word =
                    create_word_instance(&loc_zh as *const usize as *const c_void, &mut err_w)
                        as *mut IcuBreakIterator;
                assert!(!bi_zh_word.is_null());
                let mut ustr_dict: usize = 0;
                unicode_string_ctor(&mut ustr_dict, "中华人民共和国\0".as_ptr());
                brk_set_text(bi_zh_word, &ustr_dict);
                unicode_string_dtor(&mut ustr_dict);
                let zh_words = collect_breaks(bi_zh_word);
                assert_eq!(zh_words.first().copied(), Some(0));
                assert_eq!(zh_words.last().copied(), Some(7));
                assert!(
                    zh_words.len() > 2,
                    "expected CJK dictionary word segmentation, got {zh_words:?}"
                );
                brk_scalar_deleting_dtor(bi_zh_word, 1);
            }
            locale_dtor(&mut loc_zh);
        }
    }

    #[test]
    fn break_iterator_sentence_and_hard_line_break() {
        unsafe {
            let mut err: i32 = 0;
            let bi_sent = create_sentence_instance(locale_get_us() as *const c_void, &mut err)
                as *mut IcuBreakIterator;
            assert!(!bi_sent.is_null());
            assert_eq!(err, 0);

            let mut ustr: usize = 0;
            unicode_string_ctor(&mut ustr, b"Hello world. How are you? Fine!\0".as_ptr());
            brk_set_text(bi_sent, &ustr);
            unicode_string_dtor(&mut ustr);
            assert_eq!(collect_breaks(bi_sent), vec![0, 13, 26, 31]);
            brk_scalar_deleting_dtor(bi_sent, 1);

            if icu_core().is_some() {
                let mut err_l: i32 = 0;
                let bi_line = create_line_instance(locale_get_us() as *const c_void, &mut err_l)
                    as *mut IcuBreakIterator;
                let mut ustr_l: usize = 0;
                unicode_string_ctor(&mut ustr_l, b"Hello world-wide\nnext line\0".as_ptr());
                brk_set_text(bi_line, &ustr_l);
                unicode_string_dtor(&mut ustr_l);
                assert_eq!(brk_first(bi_line), 0);
                let mut saw_hard = false;
                loop {
                    let pos = brk_next(bi_line);
                    if pos == UBRK_DONE {
                        break;
                    }
                    if pos == 17 {
                        assert!(brk_get_rule_status(bi_line) >= 100); // UBRK_LINE_HARD
                        saw_hard = true;
                    }
                }
                assert!(saw_hard);
                brk_scalar_deleting_dtor(bi_line, 1);
            }
        }
    }

    #[test]
    fn break_iterator_navigation_adopt_and_dtor_cleanup() {
        unsafe {
            let mut err: i32 = 0;
            udata_setCommonData_69(std::ptr::null(), &mut err);
            assert_eq!(err, 0);

            // Pre-existing failure status must short-circuit factory creation.
            let mut fail_err: i32 = 1;
            let null_bi = create_word_instance(std::ptr::null(), &mut fail_err);
            assert!(null_bi.is_null());
            assert_eq!(fail_err, 1);

            let bi = create_word_instance(locale_get_us() as *const c_void, &mut err)
                as *mut IcuBreakIterator;
            assert!(!bi.is_null());
            assert!(!brk_get_dynamic_class_id(bi).is_null());

            let mut ustr: usize = 0;
            unicode_string_ctor(&mut ustr, b"one two three\0".as_ptr());
            brk_set_text(bi, &ustr);
            unicode_string_dtor(&mut ustr);

            // last() & previous()
            assert_eq!(brk_last(bi), 13);
            assert_eq!(brk_current(bi), 13);
            assert_eq!(brk_previous(bi), 8); // before "three"
            assert_eq!(brk_previous(bi), 7); // before " "
            assert_eq!(brk_previous(bi), 4); // before "two"

            // following() & preceding()
            assert_eq!(brk_following(bi, 0), 3);
            assert_eq!(brk_following(bi, 13), UBRK_DONE);
            assert_eq!(brk_preceding(bi, 10), 8);
            assert_eq!(brk_preceding(bi, 0), UBRK_DONE);

            // isBoundary() side effect sets current() to first boundary >= offset
            assert_eq!(brk_is_boundary(bi, 4), 1);
            assert_eq!(brk_current(bi), 4);
            assert_eq!(brk_is_boundary(bi, 1), 0);
            assert_eq!(brk_current(bi), 3);

            // next(n): 0, positive, negative, past end
            assert_eq!(brk_first(bi), 0);
            assert_eq!(brk_next_n(bi, 0), 0);
            assert_eq!(brk_next_n(bi, 2), 4); // 0 -> 3 -> 4
            assert_eq!(brk_next_n(bi, -1), 3); // 4 -> 3
            assert_eq!(brk_next_n(bi, 100), UBRK_DONE);

            // createBufferClone preflight + allocate
            let mut buf_sz: i32 = 0;
            let mut clone_err: i32 = 0;
            assert!(brk_create_buffer_clone(bi, std::ptr::null_mut(), &mut buf_sz, &mut clone_err).is_null());
            assert!(buf_sz > 0);
            let mut stack_buf = [0u8; 256];
            let mut stack_sz: i32 = stack_buf.len() as i32;
            let buf_clone = brk_create_buffer_clone(
                bi,
                stack_buf.as_mut_ptr().cast::<c_void>(),
                &mut stack_sz,
                &mut clone_err,
            );
            assert!(!buf_clone.is_null());
            assert_eq!(clone_err, U_SAFECLONE_ALLOCATED_WARNING);
            brk_scalar_deleting_dtor(buf_clone, 1);

            // adoptText via UCharCharacterIterator-compatible layout
            let adopted_u16: Vec<u16> = "alpha beta".encode_utf16().collect();
            let mut ci = CharIterShim {
                vtbl: &CHAR_ITER_VTBL.0,
                text_length: adopted_u16.len() as i32,
                pos: 0,
                begin: 0,
                end: adopted_u16.len() as i32,
                text: adopted_u16.as_ptr(),
            };
            brk_adopt_text(bi, (&mut ci as *mut CharIterShim).cast::<c_void>());
            assert_eq!(collect_breaks(bi), vec![0, 5, 6, 10]);

            // getText() returns valid CharacterIterator view over current text
            let view = brk_get_text(bi) as *const CharIterShim;
            assert!(!view.is_null());
            assert_eq!((*view).text_length, 10);

            // Non-deleting dtor (flags=0) closes native handle cleanly, then deleting dtor frees Box
            brk_scalar_deleting_dtor(bi, 0);
            assert!((*bi).raw.is_null());
            brk_scalar_deleting_dtor(bi, 1);
        }
    }

    #[test]
    fn unicode_string_length_handles_surrogates_unaligned_null_and_dtor() {
        unsafe {
            assert_eq!(unicode_string_length(std::ptr::null()), 0);

            let mut empty: usize = 0;
            unicode_string_ctor_empty(&mut empty);
            assert_eq!(unicode_string_length(&empty), 0);
            unicode_string_dtor(&mut empty);

            // "A😀" -> 1 BMP unit + 2 UTF-16 surrogate units = 3 code units.
            let mut ustr: usize = 0;
            unicode_string_ctor(&mut ustr, "A\u{1F600}\0".as_ptr());
            assert_eq!(unicode_string_length(&ustr), 3);
            unicode_string_dtor(&mut ustr);
            assert_eq!(unicode_string_length(&ustr), 0);

            // Explicit 1-byte misaligned guest UnicodeString slot
            let mut raw_slot = [0xCCu8; 16];
            let unaligned_this = raw_slot.as_mut_ptr().add(1) as *mut usize;
            unicode_string_ctor_utf8_len(unaligned_this, b"Spider-Man".as_ptr(), 10);
            assert_eq!(unicode_string_length(unaligned_this), 10);
            unicode_string_dtor(unaligned_this);
            assert_eq!(unicode_string_length(unaligned_this), 0);
        }
    }

    #[test]
    fn ubidi_base_direction_ltr_rtl_neutral_unaligned_and_null() {
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        assert!(
            icu_bidi().is_some(),
            "expected ubidi_* symbols in /usr/lib/libicucore.A.dylib"
        );

        unsafe {
            assert_eq!(ubidi_getBaseDirection_69(std::ptr::null(), 0), UBIDI_NEUTRAL);
            assert_eq!(ubidi_getBaseDirection_69(std::ptr::null(), -1), UBIDI_NEUTRAL);

            let ltr: Vec<u16> = "123 Hello \u{05E9}\u{05DC}\u{05D5}\u{05DD}\0".encode_utf16().collect();
            assert_eq!(ubidi_getBaseDirection_69(ltr.as_ptr(), -1), UBIDI_LTR);
            assert_eq!(ubidi_getBaseDirection_69(ltr.as_ptr(), 0), UBIDI_NEUTRAL);
            assert_eq!(ubidi_getBaseDirection_69(ltr.as_ptr(), -2), UBIDI_NEUTRAL);

            // Explicit 1-byte misaligned RTL UTF-16 input (Arabic + Latin)
            let rtl_units: Vec<u16> = "99 \u{0645}\u{0631}\u{062D}\u{0628}\u{0627} Hello".encode_utf16().collect();
            let mut unaligned = vec![0xEEu8; 1 + rtl_units.len() * 2];
            for (i, cu) in rtl_units.iter().enumerate() {
                let le = cu.to_le_bytes();
                unaligned[1 + i * 2] = le[0];
                unaligned[1 + i * 2 + 1] = le[1];
            }
            assert_eq!(
                ubidi_getBaseDirection_69(
                    unaligned.as_ptr().add(1) as *const u16,
                    rtl_units.len() as i32
                ),
                UBIDI_RTL
            );

            let neutral: Vec<u16> = "12345 !@#".encode_utf16().collect();
            assert_eq!(
                ubidi_getBaseDirection_69(neutral.as_ptr(), neutral.len() as i32),
                UBIDI_NEUTRAL
            );

            // Exercise pure-Rust fallback classifier directly too.
            assert_eq!(fallback_base_direction(&ltr[..ltr.len() - 1]), UBIDI_LTR);
            assert_eq!(fallback_base_direction(&rtl_units), UBIDI_RTL);
            assert_eq!(fallback_base_direction(&neutral), UBIDI_NEUTRAL);
        }
    }

    #[test]
    fn ubidi_runs_unaligned_io_fixed_size_and_error_handling() {
        unsafe {
            // Invalid openSized arguments
            let mut err: i32 = U_ZERO_ERROR;
            assert!(ubidi_openSized_69(-1, 0, &mut err).is_null());
            assert_eq!(err, U_ILLEGAL_ARGUMENT_ERROR);

            err = U_ZERO_ERROR;
            assert!(ubidi_openSized_69(0, -1, &mut err).is_null());
            assert_eq!(err, U_ILLEGAL_ARGUMENT_ERROR);

            // Pre-existing error short-circuits openSized
            err = U_INVALID_STATE_ERROR;
            assert!(ubidi_openSized_69(0, 0, &mut err).is_null());
            assert_eq!(err, U_INVALID_STATE_ERROR);

            // Null/uninitialized handle behavior
            assert_eq!(ubidi_getDirection_69(std::ptr::null()), UBIDI_LTR);
            err = U_ZERO_ERROR;
            assert_eq!(ubidi_countRuns_69(std::ptr::null_mut(), &mut err), -1);
            assert_eq!(err, U_INVALID_STATE_ERROR);

            // Open dynamically sized UBiDi via unaligned error code pointer
            let mut err_buf = [0u8; 8];
            let unaligned_err = err_buf.as_mut_ptr().add(1) as *mut i32;
            std::ptr::write_unaligned(unaligned_err, U_ZERO_ERROR);
            let bidi = ubidi_openSized_69(0, 0, unaligned_err);
            assert!(!bidi.is_null());
            assert_eq!(std::ptr::read_unaligned(unaligned_err), U_ZERO_ERROR);

            // Before setPara: countRuns returns -1 + U_INVALID_STATE_ERROR
            assert_eq!(ubidi_countRuns_69(bidi, unaligned_err), -1);
            assert_eq!(
                std::ptr::read_unaligned(unaligned_err),
                U_INVALID_STATE_ERROR
            );

            // setPara with NULL text fails with U_ILLEGAL_ARGUMENT_ERROR
            std::ptr::write_unaligned(unaligned_err, U_ZERO_ERROR);
            ubidi_setPara_69(
                bidi,
                std::ptr::null(),
                0,
                UBIDI_DEFAULT_LTR,
                std::ptr::null_mut(),
                unaligned_err,
            );
            assert_eq!(
                std::ptr::read_unaligned(unaligned_err),
                U_ILLEGAL_ARGUMENT_ERROR
            );

            // Mixed LTR + Hebrew RTL with misaligned UTF-16 input and misaligned out pointers
            let mixed_str = "Hello \u{05E9}\u{05DC}\u{05D5}\u{05DD} world";
            let mixed_u16: Vec<u16> = mixed_str.encode_utf16().collect();
            let mut unaligned_text = vec![0xAAu8; 1 + mixed_u16.len() * 2];
            for (i, cu) in mixed_u16.iter().enumerate() {
                let le = cu.to_le_bytes();
                unaligned_text[1 + i * 2] = le[0];
                unaligned_text[1 + i * 2 + 1] = le[1];
            }
            std::ptr::write_unaligned(unaligned_err, U_ZERO_ERROR);
            ubidi_setPara_69(
                bidi,
                unaligned_text.as_ptr().add(1) as *const u16,
                mixed_u16.len() as i32,
                UBIDI_DEFAULT_LTR,
                std::ptr::null_mut(),
                unaligned_err,
            );
            assert_eq!(std::ptr::read_unaligned(unaligned_err), U_ZERO_ERROR);
            assert_eq!(ubidi_getDirection_69(bidi), UBIDI_MIXED);

            let run_count = ubidi_countRuns_69(bidi, unaligned_err);
            assert_eq!(std::ptr::read_unaligned(unaligned_err), U_ZERO_ERROR);
            assert_eq!(run_count, 3);

            let mut start_buf = [0u8; 8];
            let mut len_buf = [0u8; 8];
            let p_start = start_buf.as_mut_ptr().add(1) as *mut i32;
            let p_len = len_buf.as_mut_ptr().add(3) as *mut i32;

            let mut runs = Vec::new();
            for i in 0..run_count {
                let dir = ubidi_getVisualRun_69(bidi, i, p_start, p_len);
                runs.push((
                    dir,
                    std::ptr::read_unaligned(p_start),
                    std::ptr::read_unaligned(p_len),
                ));
            }
            assert_eq!(
                runs,
                vec![(UBIDI_LTR, 0, 6), (UBIDI_RTL, 6, 4), (UBIDI_LTR, 10, 6)]
            );

            // NULL out pointers are accepted cleanly
            assert_eq!(
                ubidi_getVisualRun_69(bidi, 1, std::ptr::null_mut(), std::ptr::null_mut()),
                UBIDI_RTL
            );

            // Out-of-range runIndex returns UBIDI_LTR and leaves caller out-params untouched
            std::ptr::write_unaligned(p_start, 777);
            std::ptr::write_unaligned(p_len, 888);
            assert_eq!(ubidi_getVisualRun_69(bidi, -1, p_start, p_len), UBIDI_LTR);
            assert_eq!(std::ptr::read_unaligned(p_start), 777);
            assert_eq!(std::ptr::read_unaligned(p_len), 888);
            assert_eq!(
                ubidi_getVisualRun_69(bidi, run_count, p_start, p_len),
                UBIDI_LTR
            );
            assert_eq!(std::ptr::read_unaligned(p_start), 777);
            assert_eq!(std::ptr::read_unaligned(p_len), 888);

            // RTL base paragraph with embedded Arabic + digits + Latin reverses visual order
            let rtl_mix = "\u{0645}\u{0631}\u{062D}\u{0628}\u{0627} 123 Hello";
            let rtl_u16: Vec<u16> = rtl_mix.encode_utf16().collect();
            std::ptr::write_unaligned(unaligned_err, U_ZERO_ERROR);
            ubidi_setPara_69(
                bidi,
                rtl_u16.as_ptr(),
                rtl_u16.len() as i32,
                UBIDI_DEFAULT_RTL,
                std::ptr::null_mut(),
                unaligned_err,
            );
            assert_eq!(std::ptr::read_unaligned(unaligned_err), U_ZERO_ERROR);
            assert_eq!(ubidi_getDirection_69(bidi), UBIDI_MIXED);
            let rtl_runs = ubidi_countRuns_69(bidi, unaligned_err);
            assert!(rtl_runs >= 3);
            let mut first_start = -1i32;
            let mut first_len = -1i32;
            let first_dir = ubidi_getVisualRun_69(bidi, 0, &mut first_start, &mut first_len);
            assert_eq!(first_dir, UBIDI_LTR);
            assert_eq!(first_start, 10);
            assert_eq!(first_len, 5);

            // Empty non-null string yields 0 runs
            let empty_u16 = [0u16];
            std::ptr::write_unaligned(unaligned_err, U_ZERO_ERROR);
            ubidi_setPara_69(
                bidi,
                empty_u16.as_ptr(),
                0,
                0,
                std::ptr::null_mut(),
                unaligned_err,
            );
            assert_eq!(std::ptr::read_unaligned(unaligned_err), U_ZERO_ERROR);
            assert_eq!(ubidi_countRuns_69(bidi, unaligned_err), 0);

            // Explicit embeddingLevels override
            let ascii_u16: Vec<u16> = "Hello".encode_utf16().collect();
            let mut emb_levels = [0x81u8; 5]; // level 1 + UBIDI_LEVEL_OVERRIDE
            std::ptr::write_unaligned(unaligned_err, U_ZERO_ERROR);
            ubidi_setPara_69(
                bidi,
                ascii_u16.as_ptr(),
                ascii_u16.len() as i32,
                0,
                emb_levels.as_mut_ptr(),
                unaligned_err,
            );
            assert_eq!(std::ptr::read_unaligned(unaligned_err), U_ZERO_ERROR);
            assert_eq!(emb_levels, [1u8; 5]);
            assert_eq!(ubidi_countRuns_69(bidi, unaligned_err), 1);
            let mut ov_start = -1i32;
            let mut ov_len = -1i32;
            assert_eq!(
                ubidi_getVisualRun_69(bidi, 0, &mut ov_start, &mut ov_len),
                UBIDI_RTL
            );
            assert_eq!((ov_start, ov_len), (0, 5));

            // Close + idempotent double close + NULL close
            ubidi_close_69(bidi);
            ubidi_close_69(bidi);
            ubidi_close_69(std::ptr::null_mut());

            // Pre-sized UBiDi rejects text longer than maxLength with U_MEMORY_ALLOCATION_ERROR
            err = U_ZERO_ERROR;
            let fixed = ubidi_openSized_69(2, 0, &mut err);
            assert!(!fixed.is_null());
            assert_eq!(err, U_ZERO_ERROR);
            ubidi_setPara_69(
                fixed,
                ascii_u16.as_ptr(),
                ascii_u16.len() as i32,
                0,
                std::ptr::null_mut(),
                &mut err,
            );
            assert_eq!(err, U_MEMORY_ALLOCATION_ERROR);
            ubidi_close_69(fixed);

            // Pure-Rust UAX #9 fallback matches native libicucore on mixed LTR/RTL and Arabic + digits
            let (fb_dir, fb_runs) =
                fallback_compute_bidi(&mixed_u16, UBIDI_DEFAULT_LTR, None, 0, 0).unwrap();
            assert_eq!(fb_dir, UBIDI_MIXED);
            assert_eq!(
                fb_runs
                    .iter()
                    .map(|r| (r.direction, r.logical_start, r.length))
                    .collect::<Vec<_>>(),
                vec![(UBIDI_LTR, 0, 6), (UBIDI_RTL, 6, 4), (UBIDI_LTR, 10, 6)]
            );

            let (fb_rtl_dir, fb_rtl_runs) =
                fallback_compute_bidi(&rtl_u16, UBIDI_DEFAULT_RTL, None, 0, 0).unwrap();
            assert_eq!(fb_rtl_dir, UBIDI_MIXED);
            assert_eq!(
                fb_rtl_runs
                    .iter()
                    .map(|r| (r.direction, r.logical_start, r.length))
                    .collect::<Vec<_>>(),
                vec![
                    (UBIDI_LTR, 10, 5),
                    (UBIDI_RTL, 9, 1),
                    (UBIDI_LTR, 6, 3),
                    (UBIDI_RTL, 0, 6),
                ]
            );

            // Explicit bidi controls fail honestly in fallback mode instead of returning wrong runs
            let explicit_u16: Vec<u16> = "\u{202E}abc\u{202C}".encode_utf16().collect();
            assert_eq!(
                fallback_compute_bidi(&explicit_u16, UBIDI_DEFAULT_LTR, None, 0, 0).err(),
                Some(U_UNSUPPORTED_ERROR)
            );
            assert_eq!(
                fallback_compute_bidi(&ascii_u16, 0, None, 2, 0).err(),
                Some(U_MEMORY_ALLOCATION_ERROR)
            );
        }
    }

    #[test]
    fn u_set_memory_functions_routes_alloc_realloc_free_and_enforces_state() {
        use std::sync::atomic::{AtomicBool, AtomicUsize};

        struct AllocTracker {
            alloc_calls: AtomicUsize,
            realloc_calls: AtomicUsize,
            free_calls: AtomicUsize,
            live_ptrs: Mutex<HashMap<usize, Vec<u8>>>,
            fail_alloc: AtomicBool,
        }

        unsafe extern "win64" fn test_alloc(
            ctx: u64,
            size: u64,
            _: u64,
            _: u64,
            _: u64,
            _: u64,
            _: u64,
            _: u64,
        ) -> u64 {
            let tracker = &*(ctx as *const AllocTracker);
            tracker.alloc_calls.fetch_add(1, Ordering::AcqRel);
            if tracker.fail_alloc.load(Ordering::Acquire) {
                return 0;
            }
            let mut buf = vec![0u8; (size as usize).max(1)];
            let ptr = buf.as_mut_ptr() as usize;
            tracker
                .live_ptrs
                .lock()
                .unwrap()
                .insert(ptr, buf);
            ptr as u64
        }

        unsafe extern "win64" fn test_realloc(
            ctx: u64,
            mem: u64,
            size: u64,
            _: u64,
            _: u64,
            _: u64,
            _: u64,
            _: u64,
        ) -> u64 {
            let tracker = &*(ctx as *const AllocTracker);
            tracker.realloc_calls.fetch_add(1, Ordering::AcqRel);
            if tracker.fail_alloc.load(Ordering::Acquire) {
                return 0;
            }
            let mut map = tracker.live_ptrs.lock().unwrap();
            let mut buf = map.remove(&(mem as usize)).unwrap_or_default();
            buf.resize((size as usize).max(1), 0);
            let ptr = buf.as_mut_ptr() as usize;
            map.insert(ptr, buf);
            ptr as u64
        }

        unsafe extern "win64" fn test_free(
            ctx: u64,
            mem: u64,
            _: u64,
            _: u64,
            _: u64,
            _: u64,
            _: u64,
            _: u64,
        ) -> u64 {
            let tracker = &*(ctx as *const AllocTracker);
            tracker.free_calls.fetch_add(1, Ordering::AcqRel);
            let removed = tracker.live_ptrs.lock().unwrap().remove(&(mem as usize));
            assert!(
                removed.is_some(),
                "free called for unknown/double-freed pointer {mem:#x}"
            );
            0
        }

        struct ResetMemGuard;
        impl Drop for ResetMemGuard {
            fn drop(&mut self) {
                let mut st = mem_func_state().lock().unwrap_or_else(|e| e.into_inner());
                st.heap_in_use = false;
                st.custom = None;
            }
        }

        // Reset before running so earlier tests in this process don't pre-latch `heap_in_use`.
        let _guard = {
            let mut st = mem_func_state().lock().unwrap_or_else(|e| e.into_inner());
            st.heap_in_use = false;
            st.custom = None;
            ResetMemGuard
        };

        let tracker = AllocTracker {
            alloc_calls: AtomicUsize::new(0),
            realloc_calls: AtomicUsize::new(0),
            free_calls: AtomicUsize::new(0),
            live_ptrs: Mutex::new(HashMap::new()),
            fail_alloc: AtomicBool::new(false),
        };
        let ctx_ptr = (&tracker as *const AllocTracker).cast::<c_void>();

        unsafe {
            // Missing callback pointer -> U_ILLEGAL_ARGUMENT_ERROR
            let mut err: i32 = U_ZERO_ERROR;
            u_setMemoryFunctions_69(
                ctx_ptr,
                std::ptr::null(),
                test_realloc as *const c_void,
                test_free as *const c_void,
                &mut err,
            );
            assert_eq!(err, U_ILLEGAL_ARGUMENT_ERROR);

            // Pre-existing failure status -> short-circuit without altering status
            err = U_BUFFER_OVERFLOW_ERROR;
            u_setMemoryFunctions_69(
                ctx_ptr,
                test_alloc as *const c_void,
                test_realloc as *const c_void,
                test_free as *const c_void,
                &mut err,
            );
            assert_eq!(err, U_BUFFER_OVERFLOW_ERROR);

            // Register custom allocator callbacks via unaligned status pointer
            let mut err_buf = [0u8; 8];
            let unaligned_err = err_buf.as_mut_ptr().add(3) as *mut i32;
            std::ptr::write_unaligned(unaligned_err, U_ZERO_ERROR);
            u_setMemoryFunctions_69(
                ctx_ptr,
                test_alloc as *const c_void,
                test_realloc as *const c_void,
                test_free as *const c_void,
                unaligned_err,
            );
            assert_eq!(std::ptr::read_unaligned(unaligned_err), U_ZERO_ERROR);

            // UBiDi open + growing setPara + close exercises alloc, realloc, and free
            let bidi = ubidi_openSized_69(0, 0, unaligned_err);
            assert!(!bidi.is_null());
            assert_eq!(std::ptr::read_unaligned(unaligned_err), U_ZERO_ERROR);
            assert_eq!(tracker.alloc_calls.load(Ordering::Acquire), 1);

            let short_u16: Vec<u16> = "Hi".encode_utf16().collect();
            ubidi_setPara_69(
                bidi,
                short_u16.as_ptr(),
                short_u16.len() as i32,
                0,
                std::ptr::null_mut(),
                unaligned_err,
            );
            assert_eq!(std::ptr::read_unaligned(unaligned_err), U_ZERO_ERROR);
            assert_eq!(tracker.alloc_calls.load(Ordering::Acquire), 2);
            assert_eq!(tracker.realloc_calls.load(Ordering::Acquire), 0);

            let longer_u16: Vec<u16> = "Hello longer bidi paragraph".encode_utf16().collect();
            ubidi_setPara_69(
                bidi,
                longer_u16.as_ptr(),
                longer_u16.len() as i32,
                0,
                std::ptr::null_mut(),
                unaligned_err,
            );
            assert_eq!(std::ptr::read_unaligned(unaligned_err), U_ZERO_ERROR);
            assert_eq!(tracker.realloc_calls.load(Ordering::Acquire), 1);

            ubidi_close_69(bidi);
            assert_eq!(tracker.free_calls.load(Ordering::Acquire), 2);
            assert!(tracker.live_ptrs.lock().unwrap().is_empty());

            // BreakIterator create + delete also routes through custom alloc/free
            err = U_ZERO_ERROR;
            let bi = create_word_instance(locale_get_us() as *const c_void, &mut err)
                as *mut IcuBreakIterator;
            assert!(!bi.is_null());
            assert_eq!(err, U_ZERO_ERROR);
            assert_eq!(tracker.alloc_calls.load(Ordering::Acquire), 3);
            brk_scalar_deleting_dtor(bi, 1);
            assert_eq!(tracker.free_calls.load(Ordering::Acquire), 3);
            assert!(tracker.live_ptrs.lock().unwrap().is_empty());

            // Calling u_setMemoryFunctions_69 once heap is in use returns U_INVALID_STATE_ERROR
            err = U_ZERO_ERROR;
            u_setMemoryFunctions_69(
                ctx_ptr,
                test_alloc as *const c_void,
                test_realloc as *const c_void,
                test_free as *const c_void,
                &mut err,
            );
            assert_eq!(err, U_INVALID_STATE_ERROR);

            // Simulated OOM from custom allocator propagates U_MEMORY_ALLOCATION_ERROR cleanly
            tracker.fail_alloc.store(true, Ordering::Release);
            err = U_ZERO_ERROR;
            assert!(ubidi_openSized_69(0, 0, &mut err).is_null());
            assert_eq!(err, U_MEMORY_ALLOCATION_ERROR);
            err = U_ZERO_ERROR;
            assert!(create_word_instance(locale_get_us() as *const c_void, &mut err).is_null());
            assert_eq!(err, U_MEMORY_ALLOCATION_ERROR);
            assert!(tracker.live_ptrs.lock().unwrap().is_empty());
        }
    }
}
