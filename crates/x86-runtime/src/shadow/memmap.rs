//! VM map enumeration and the tracked range table.

use super::handler::page;
use win32_rt::libc;

#[repr(C, packed(4))]
#[derive(Default, Clone, Copy)]
struct SubmapInfo64 {
    protection: i32,
    max_protection: i32,
    inheritance: u32,
    offset: u64,
    user_tag: u32,
    pages_resident: u32,
    pages_shared_now_private: u32,
    pages_swapped_out: u32,
    pages_dirtied: u32,
    ref_count: u32,
    shadow_depth: u16,
    external_pager: u8,
    share_mode: u8,
    is_submap: u32,
    behavior: i32,
    object_id: u32,
    user_wired_count: u16,
    pages_reusable: u32,
    object_id_full: u64,
}

extern "C" {
    static mach_task_self_: u32;
    fn mach_vm_region_recurse(
        task: u32,
        address: *mut u64,
        size: *mut u64,
        depth: *mut u32,
        info: *mut i32,
        count: *mut u32,
    ) -> i32;
    fn _dyld_image_count() -> u32;
    fn _dyld_get_image_header(i: u32) -> *const u8;
    fn _dyld_get_image_vmaddr_slide(i: u32) -> isize;
}

const SM_COW: u8 = 1;
const SM_PRIVATE: u8 = 2;
const SM_EMPTY: u8 = 3;
const SM_PRIVATE_ALIASED: u8 = 6;

/// A VM region.
#[derive(Clone, Copy, Debug)]
pub struct Region {
    pub lo: usize,
    pub hi: usize,
    pub prot: i32,
    pub share: u8,
    pub tag: u32,
}

/// The task's VM map (leaf regions, ascending).
pub unsafe fn regions() -> Vec<Region> {
    let mut out = Vec::with_capacity(8192);
    let mut addr: u64 = 0;
    let mut depth: u32 = 0;
    loop {
        let mut size: u64 = 0;
        let mut info = SubmapInfo64::default();
        let mut count = (core::mem::size_of::<SubmapInfo64>() / 4) as u32;
        let kr = mach_vm_region_recurse(
            mach_task_self_,
            &mut addr,
            &mut size,
            &mut depth,
            &mut info as *mut SubmapInfo64 as *mut i32,
            &mut count,
        );
        if kr != 0 {
            break;
        }
        if info.is_submap != 0 {
            depth += 1;
            continue;
        }
        out.push(Region {
            lo: addr as usize,
            hi: (addr + size) as usize,
            prot: info.protection,
            share: info.share_mode,
            tag: info.user_tag,
        });
        addr += size;
    }
    out
}

/// Region containing `addr`.
pub fn region_of(regs: &[Region], addr: usize) -> Option<&Region> {
    let i = regs.partition_point(|r| r.hi <= addr);
    regs.get(i).filter(|r| r.lo <= addr)
}

/// libc malloc regions (`VM_MEMORY_MALLOC*` tags 1..=11).
pub fn is_malloc_tag(t: u32) -> bool {
    (1..=11).contains(&t)
}

const LC_SEGMENT_64: u32 = 0x19;
const S_THREAD_LOCAL_REGULAR: u32 = 0x11;
const S_THREAD_LOCAL_ZEROFILL: u32 = 0x12;
const S_THREAD_LOCAL_VARIABLES: u32 = 0x13;

#[repr(C)]
struct TlvDesc {
    thunk: unsafe extern "C" fn(*const TlvDesc) -> *mut u8,
    key: usize,
    offset: usize,
}

/// Host memory the comparison never looks at: every dyld image segment (`hard`) and this
/// thread's thread-local blocks of every image (`soft`: byte ranges inside the heap).
pub unsafe fn image_ranges(hard: &mut Vec<(usize, usize)>, soft: &mut Vec<(usize, usize)>) {
    for i in 0.._dyld_image_count() {
        let h = _dyld_get_image_header(i);
        if h.is_null() {
            continue;
        }
        let slide = _dyld_get_image_vmaddr_slide(i);
        let ncmds = *(h.add(16) as *const u32);
        let mut lc = h.add(32);
        let (mut tl_lo, mut tl_hi) = (usize::MAX, 0usize);
        let mut vars: Option<(usize, usize)> = None;
        for _ in 0..ncmds {
            let cmd = *(lc as *const u32);
            let cmdsize = *(lc.add(4) as *const u32) as usize;
            if cmd == LC_SEGMENT_64 {
                let name = core::slice::from_raw_parts(lc.add(8), 16);
                let vmaddr = *(lc.add(24) as *const u64);
                let vmsize = *(lc.add(32) as *const u64);
                if !name.starts_with(b"__PAGEZERO") && vmsize != 0 {
                    let lo = (vmaddr as isize + slide) as usize;
                    hard.push((lo, lo + vmsize as usize));
                }
                let nsects = *(lc.add(64) as *const u32);
                let mut s = lc.add(72);
                for _ in 0..nsects {
                    let addr = *(s.add(32) as *const u64);
                    let size = *(s.add(40) as *const u64) as usize;
                    let flags = *(s.add(64) as *const u32);
                    match flags & 0xff {
                        S_THREAD_LOCAL_REGULAR | S_THREAD_LOCAL_ZEROFILL => {
                            tl_lo = tl_lo.min(addr as usize);
                            tl_hi = tl_hi.max(addr as usize + size);
                        }
                        S_THREAD_LOCAL_VARIABLES => vars = Some(((addr as isize + slide) as usize, size)),
                        _ => {}
                    }
                    s = s.add(80);
                }
            }
            if cmdsize == 0 {
                break;
            }
            lc = lc.add(cmdsize);
        }
        let tlv_size = tl_hi.saturating_sub(tl_lo);
        if let Some((va, sz)) = vars {
            if sz >= core::mem::size_of::<TlvDesc>() && tlv_size > 0 {
                let d = va as *const TlvDesc;
                let th = (*d).thunk as usize;
                if th != 0 && th != tlv_bootstrap() {
                    let p = ((*d).thunk)(d) as usize;
                    // Current dyld rewrites the descriptor at load: `key` = variable offset << 32 |
                    // pthread key, `offset` = block size << 32 | (unused). Older dyld: plain
                    // `key` and `offset`.
                    let (off, size) = if (*d).offset >> 32 != 0 {
                        ((*d).key >> 32, ((*d).offset >> 32).max(tlv_size))
                    } else {
                        ((*d).offset, tlv_size)
                    };
                    if off < size {
                        let base = p.wrapping_sub(off);
                        soft.push((base, base + size));
                    }
                }
            }
        }
    }
}

const LC_SYMTAB: u32 = 0x2;

/// Address of the first local/global symbol of the main executable whose name contains
/// `needle` (its `LC_SYMTAB`, read from `__LINKEDIT` in memory). Rust statics are not
/// exported, but release builds keep them in the symbol table.
pub unsafe fn self_symbol(needle: &[u8]) -> Option<usize> {
    let h = _dyld_get_image_header(0);
    if h.is_null() || needle.is_empty() {
        return None;
    }
    let slide = _dyld_get_image_vmaddr_slide(0);
    let ncmds = *(h.add(16) as *const u32);
    let mut lc = h.add(32);
    let (mut le, mut tab) = (None, None);
    for _ in 0..ncmds {
        let cmd = *(lc as *const u32);
        let cmdsize = *(lc.add(4) as *const u32) as usize;
        if cmd == LC_SEGMENT_64 && core::slice::from_raw_parts(lc.add(8), 16).starts_with(b"__LINKEDIT") {
            le = Some((*(lc.add(24) as *const u64), *(lc.add(40) as *const u64)));
        } else if cmd == LC_SYMTAB {
            let r = |o: usize| *(lc.add(o) as *const u32) as usize;
            tab = Some((r(8), r(12), r(16), r(20)));
        }
        if cmdsize == 0 {
            break;
        }
        lc = lc.add(cmdsize);
    }
    let ((le_vm, le_off), (symoff, nsyms, stroff, strsize)) = (le?, tab?);
    let base = (le_vm as isize + slide) as usize - le_off as usize;
    let strs = core::slice::from_raw_parts((base + stroff) as *const u8, strsize);
    for i in 0..nsyms {
        let e = (base + symoff + i * 16) as *const u8;
        let strx = *(e as *const u32) as usize;
        let ty = *e.add(4);
        if ty & 0xe0 != 0 || ty & 0x0e != 0x0e || strx >= strsize {
            continue; // stab, or not defined in a section
        }
        let name = &strs[strx..];
        let name = &name[..name.iter().position(|&c| c == 0).unwrap_or(name.len())];
        if name.windows(needle.len()).any(|w| w == needle) {
            let v = *(e.add(8) as *const u64);
            return Some((v as isize + slide) as usize);
        }
    }
    None
}

static PL_HASHTABLE: std::sync::OnceLock<usize> = std::sync::OnceLock::new();

/// Look `parking_lot_core::parking_lot::HASHTABLE` up once, in the game/test process (the
/// orchestrator only reads the cached address).
pub fn parking_lot_sym() {
    PL_HASHTABLE.get_or_init(|| unsafe { self_symbol(b"16parking_lot_core11parking_lot9HASHTABLE").unwrap_or(0) });
}

/// `[lo, hi)` of parking_lot_core's current bucket array: `HASHTABLE` points at a
/// `HashTable { entries: Box<[Bucket]>, hash_bits, _prev }` (field order is the
/// compiler's: the `(ptr, len)` pair is found by shape: 64-aligned pointer, power-of-two
/// length). Buckets are `#[repr(align(64))]`, 64 bytes.
pub unsafe fn parking_lot_buckets() -> Option<(usize, usize)> {
    let sym = *PL_HASHTABLE.get()?;
    if sym == 0 {
        return None;
    }
    let t = *(sym as *const usize);
    if t == 0 || t & 7 != 0 {
        return None;
    }
    let w = core::slice::from_raw_parts(t as *const usize, 4);
    for i in 0..3 {
        let (p, n) = (w[i], w[i + 1]);
        if p != 0 && p & 63 == 0 && n.is_power_of_two() && n <= 1 << 24 {
            return Some((p, p + n * 64));
        }
    }
    None
}

fn tlv_bootstrap() -> usize {
    static P: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
    *P.get_or_init(|| unsafe { libc::dlsym(libc::RTLD_DEFAULT, c"_tlv_bootstrap".as_ptr()) as usize })
}

/// Per-image thread-local sections and descriptors (test diagnostics).
#[cfg(test)]
pub unsafe fn tlv_debug() -> String {
    extern "C" {
        fn _dyld_get_image_name(i: u32) -> *const libc::c_char;
    }
    let mut out = String::new();
    for i in 0.._dyld_image_count() {
        let h = _dyld_get_image_header(i);
        if h.is_null() {
            continue;
        }
        let slide = _dyld_get_image_vmaddr_slide(i);
        let ncmds = *(h.add(16) as *const u32);
        let mut lc = h.add(32);
        let mut secs = String::new();
        let mut vars: Option<(usize, usize)> = None;
        for _ in 0..ncmds {
            let cmd = *(lc as *const u32);
            let cmdsize = *(lc.add(4) as *const u32) as usize;
            if cmd == LC_SEGMENT_64 {
                let nsects = *(lc.add(64) as *const u32);
                let mut s = lc.add(72);
                for _ in 0..nsects {
                    let addr = *(s.add(32) as *const u64);
                    let size = *(s.add(40) as *const u64) as usize;
                    let flags = *(s.add(64) as *const u32);
                    let t = flags & 0xff;
                    if (S_THREAD_LOCAL_REGULAR..=S_THREAD_LOCAL_VARIABLES).contains(&t) {
                        let n = core::slice::from_raw_parts(s, 16);
                        let n = String::from_utf8_lossy(&n[..n.iter().position(|&b| b == 0).unwrap_or(16)]).into_owned();
                        secs += &format!(" {n}(t{t:#x} addr={addr:#x} size={size:#x})");
                        if t == S_THREAD_LOCAL_VARIABLES {
                            vars = Some(((addr as isize + slide) as usize, size));
                        }
                    }
                    s = s.add(80);
                }
            }
            if cmdsize == 0 {
                break;
            }
            lc = lc.add(cmdsize);
        }
        if secs.is_empty() {
            continue;
        }
        let name = std::ffi::CStr::from_ptr(_dyld_get_image_name(i)).to_string_lossy().into_owned();
        out += &format!("[{i}] {name}:{secs}\n");
        if let Some((va, sz)) = vars {
            let n = sz / core::mem::size_of::<TlvDesc>();
            let (mut lo, mut hi) = (usize::MAX, 0usize);
            for k in 0..n {
                let d = (va as *const TlvDesc).add(k);
                let th = (*d).thunk as usize;
                if th == 0 || th == tlv_bootstrap() {
                    out += &format!("   desc {k}: thunk {th:#x} (unusable)\n");
                    break;
                }
                let p = ((*d).thunk)(d) as usize;
                lo = lo.min(p);
                hi = hi.max(p);
                if k < 3 {
                    out += &format!("   desc {k}: key {:#x} offset {:#x} -> {p:#x}\n", (*d).key, (*d).offset);
                }
            }
            out += &format!("   {n} descs, addresses [{lo:#x}, {hi:#x}]\n");
        }
    }
    out
}

/// Tracked range table (`[lo, hi, flags]`, sorted): writable non-executable regions minus
/// `hard` (rounded outward to pages). Flags: bit 0 shared, bits 8..11 protection.
pub unsafe fn tracked_ranges(regs: &[Region], hard: &[(usize, usize)]) -> Vec<[usize; 3]> {
    let pg = page();
    let mut hard: Vec<(usize, usize)> =
        hard.iter().map(|&(lo, hi)| (lo & !(pg - 1), (hi + pg - 1) & !(pg - 1))).collect();
    hard.sort_unstable();
    let mut out = Vec::with_capacity(regs.len());
    for r in regs {
        if r.prot & libc::PROT_WRITE == 0 || r.prot & libc::PROT_EXEC != 0 {
            continue;
        }
        let shared = !matches!(r.share, SM_COW | SM_PRIVATE | SM_EMPTY | SM_PRIVATE_ALIASED);
        let flags = (shared as usize) | ((r.prot as usize & 7) << 8);
        let mut cur = r.lo;
        for &(xlo, xhi) in hard.iter() {
            if xhi <= cur || xlo >= r.hi {
                continue;
            }
            if xlo > cur {
                out.push([cur, xlo, flags]);
            }
            cur = cur.max(xhi);
            if cur >= r.hi {
                break;
            }
        }
        if cur < r.hi {
            out.push([cur, r.hi, flags]);
        }
    }
    out.sort_unstable();
    out
}

/// Write-protect every tracked range (keeps read access). Returns the tracked bytes.
pub unsafe fn protect(ranges: &[[usize; 3]]) -> usize {
    let mut total = 0;
    for r in ranges {
        if libc::mprotect(r[0] as *mut libc::c_void, r[1] - r[0], libc::PROT_READ) == 0 {
            total += r[1] - r[0];
        }
    }
    total
}
