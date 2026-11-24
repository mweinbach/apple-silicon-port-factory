//! Fake PE images for builtin (Rust-implemented) DLLs.
//!
//! On Windows an `HMODULE` is the base of a mapped PE image, and some code
//! resolves exports by walking `IMAGE_EXPORT_DIRECTORY` itself instead of
//! calling `GetProcAddress` (Streamline's `sl.interposer` does this for
//! `dxgi.dll`/`d3d12.dll`, see `third_party/streamline/source/core/
//! sl.interposer/hook.cpp`, `enumerateModuleExports`). So every builtin
//! module gets a small image:
//!
//! ```text
//! +0x0000  headers   DOS + NT headers, 2 section headers, export data dir
//! +0x1000  .rdata    IMAGE_EXPORT_DIRECTORY, AddressOfFunctions,
//!                    AddressOfNames (sorted, for binary search),
//!                    AddressOfNameOrdinals, DLL name + export name strings
//! +N       .text     one 16-byte thunk per code export:
//!                    `jmp qword ptr [rip+0]` + absolute target + int3 pad
//! ```
//!
//! EAT entries are 32-bit RVAs, and our implementations live in the loader
//! binary (anywhere in the 64-bit address space), hence the thunks. They
//! also keep every guest-visible function pointer in guest-protectable
//! memory: code that `VirtualProtect`s and patches an export's first bytes
//! (detours, Streamline's `restoreOriginalCode`) touches the thunk page,
//! never host `__TEXT`. Data exports cannot go through a thunk; they get a
//! direct RVA when the data lies within 4 GB above the image base (the
//! loader places images just below the host binary to make that true).
//!
//! Ordinals: DLLs with an ordinal table (`BuiltinDll::ordinals`) keep those
//! ordinals; every other named export gets the next free ordinal after them
//! (or `Base = 1` and sorted-name order when there is no table).

use std::collections::BTreeMap;

pub const PAGE: usize = 0x1000;
pub const THUNK_SIZE: usize = 16;

/// One export to place into the image.
#[derive(Clone, Debug)]
pub struct ImageExport {
    pub name: Option<String>,
    /// Fixed ordinal (from the DLL's ordinal table), if any.
    pub ordinal: Option<u16>,
    pub target: usize,
    /// Code exports get a thunk; data exports a direct RVA.
    pub is_code: bool,
}

/// Result of laying out an image.
pub struct BuiltImage {
    pub bytes: Vec<u8>,
    /// Size in memory (page aligned).
    pub size: usize,
    /// `.text` range [start, end) as RVAs (executable pages).
    pub text: (usize, usize),
    /// Export name -> absolute address that `GetProcAddress` should return.
    pub by_name: BTreeMap<String, usize>,
    /// Ordinal -> absolute address.
    pub by_ordinal: BTreeMap<u16, usize>,
    /// Thunk address -> export name (for symbolization).
    pub thunk_names: BTreeMap<usize, String>,
    /// Data exports that could not be expressed as an RVA (outside 4 GB).
    pub unreachable_data: Vec<String>,
    /// Optional 8-byte `Wow64PrepareForException` pointer slot inside `ntdll.dll`'s
    /// `.mrdata` section (referenced by `KiUserExceptionDispatcher`).
    pub wow64_prepare_slot: Option<usize>,
}

fn align(v: usize, a: usize) -> usize {
    (v + a - 1) & !(a - 1)
}

struct W<'a>(&'a mut Vec<u8>);
impl W<'_> {
    fn u16(&mut self, o: usize, v: u16) {
        self.0[o..o + 2].copy_from_slice(&v.to_le_bytes());
    }
    fn u32(&mut self, o: usize, v: u32) {
        self.0[o..o + 4].copy_from_slice(&v.to_le_bytes());
    }
    fn u64(&mut self, o: usize, v: u64) {
        self.0[o..o + 8].copy_from_slice(&v.to_le_bytes());
    }
    fn bytes(&mut self, o: usize, b: &[u8]) {
        self.0[o..o + b.len()].copy_from_slice(b);
    }
}

/// Lay out a builtin image for `dll_name` mapped at `base`.
pub fn build(dll_name: &str, base: usize, exports: &[ImageExport]) -> BuiltImage {
    // ---- ordinal assignment ---------------------------------------------
    let mut exps: Vec<ImageExport> = Vec::new();
    {
        let mut seen = std::collections::HashSet::new();
        for e in exports {
            if let Some(n) = &e.name {
                if !seen.insert(n.clone()) {
                    continue;
                }
            }
            exps.push(e.clone());
        }
    }
    exps.sort_by(|a, b| a.name.cmp(&b.name));
    let fixed: Vec<u16> = exps.iter().filter_map(|e| e.ordinal).collect();
    let ord_base: u32 = fixed.iter().copied().min().map(u32::from).unwrap_or(1).max(1);
    let mut next: u32 = fixed.iter().copied().max().map(|m| m as u32 + 1).unwrap_or(ord_base);
    let mut used: std::collections::HashSet<u32> = fixed.iter().map(|&o| o as u32).collect();
    let mut ords: Vec<u32> = Vec::with_capacity(exps.len());
    for e in &exps {
        let o = match e.ordinal {
            Some(o) => o as u32,
            None => {
                while used.contains(&next) {
                    next += 1;
                }
                used.insert(next);
                next
            }
        };
        ords.push(o);
    }
    let n_funcs = ords.iter().map(|&o| o - ord_base + 1).max().unwrap_or(0) as usize;
    let named: Vec<usize> = (0..exps.len()).filter(|&i| exps[i].name.is_some()).collect(); // already sorted by name

    // ---- .rdata layout ----------------------------------------------------
    let rdata = PAGE;
    let dir = rdata;
    let funcs = dir + 40;
    let names = funcs + 4 * n_funcs;
    let name_ords = names + 4 * named.len();
    let mut strings = align(name_ords + 2 * named.len(), 4);
    let dll_name_rva = strings;
    strings += dll_name.len() + 1;
    let mut name_rvas = Vec::with_capacity(named.len());
    for &i in &named {
        name_rvas.push(strings);
        strings += exps[i].name.as_ref().unwrap().len() + 1;
    }
    let export_end = strings;
    let text = align(export_end, PAGE);
    let n_code = exps.iter().filter(|e| e.is_code).count();
    let text_end = text + align((n_code * THUNK_SIZE).max(1), PAGE);
    let is_ntdll = dll_name.eq_ignore_ascii_case("ntdll.dll");
    let mrdata = text_end;
    let size = if is_ntdll { mrdata + PAGE } else { text_end };

    let mut img = vec![0u8; size];
    let mut by_name = BTreeMap::new();
    let mut by_ordinal = BTreeMap::new();
    let mut thunk_names = BTreeMap::new();
    let mut unreachable_data = Vec::new();
    {
        let mut w = W(&mut img);
        // ---- headers ------------------------------------------------------
        w.u16(0, 0x5A4D); // MZ
        w.u32(0x3C, 0x80);
        w.u32(0x80, 0x0000_4550); // PE\0\0
        let fh = 0x84;
        w.u16(fh, 0x8664); // AMD64
        w.u16(fh + 2, if is_ntdll { 3 } else { 2 }); // sections
        w.u16(fh + 16, 0xF0); // SizeOfOptionalHeader
        w.u16(fh + 18, 0x2022); // EXECUTABLE | LARGE_ADDRESS_AWARE | DLL
        let oh = fh + 20;
        w.u16(oh, 0x20B);
        w.u32(oh + 4, (text_end - text) as u32); // SizeOfCode
        w.u32(oh + 8, (text - rdata + if is_ntdll { PAGE } else { 0 }) as u32); // SizeOfInitializedData
        w.u32(oh + 20, text as u32); // BaseOfCode
        w.u64(oh + 24, base as u64);
        w.u32(oh + 32, PAGE as u32); // SectionAlignment
        w.u32(oh + 36, PAGE as u32); // FileAlignment (= section alignment: file layout == memory layout)
        w.u16(oh + 40, 10); // OS version
        w.u16(oh + 48, 10); // subsystem version
        w.u32(oh + 56, size as u32); // SizeOfImage
        w.u32(oh + 60, PAGE as u32); // SizeOfHeaders
        w.u16(oh + 68, 3); // subsystem: console
        w.u16(oh + 70, 0x0160); // DllCharacteristics: HIGH_ENTROPY_VA | DYNAMIC_BASE | NX_COMPAT
        w.u32(oh + 108, 16); // NumberOfRvaAndSizes
        // DataDirectory[0] = export
        w.u32(oh + 112, dir as u32);
        w.u32(oh + 116, (export_end - dir) as u32);
        // Section headers.
        let sh = oh + 0xF0;
        let mut sect = |i: usize, name: &[u8], rva: usize, len: usize, ch: u32| {
            let s = sh + i * 40;
            w.bytes(s, name);
            w.u32(s + 8, len as u32); // VirtualSize
            w.u32(s + 12, rva as u32);
            w.u32(s + 16, align(len, PAGE) as u32); // SizeOfRawData
            w.u32(s + 20, rva as u32); // PointerToRawData
            w.u32(s + 36, ch);
        };
        sect(0, b".rdata", rdata, export_end - rdata, 0x4000_0040); // INITIALIZED_DATA | READ
        sect(1, b".text", text, n_code * THUNK_SIZE, 0x6000_0020); // CODE | EXECUTE | READ
        if is_ntdll {
            sect(2, b".mrdata", mrdata, PAGE, 0x4000_0040); // INITIALIZED_DATA | READ (Wow64PrepareForException slot)
        }

        // ---- export directory ---------------------------------------------
        w.u32(dir + 12, dll_name_rva as u32);
        w.u32(dir + 16, ord_base);
        w.u32(dir + 20, n_funcs as u32);
        w.u32(dir + 24, named.len() as u32);
        w.u32(dir + 28, funcs as u32);
        w.u32(dir + 32, names as u32);
        w.u32(dir + 36, name_ords as u32);
        w.bytes(dll_name_rva, dll_name.as_bytes());
        let mut thunk = text;
        for (i, e) in exps.iter().enumerate() {
            let idx = (ords[i] - ord_base) as usize;
            let addr = if e.is_code {
                // On native ARM64 (`aarch64`), emit authentic Windows 10 x64 stubs in
                // `ntdll.dll`:
                // - Syscall exports get `mov r10, rcx; mov eax, <SSN>; syscall; ret`
                //   so anti-tamper / SSN-extraction checks see unhooked ntdll bytes.
                // - `KiUserExceptionDispatcher` gets the canonical Windows 10 x64
                //   `cld; mov rax, [rip + Wow64PrepareForException]; test rax, rax; jz +2; call rax; ret`
                //   pointing into `.mrdata` so user-mode exception dispatch hooks work.
                // Direct calls still fast-path into Rust via `register_native_va`.
                #[cfg(not(target_arch = "x86_64"))]
                let is_ki_user_exc = is_ntdll && e.name.as_deref() == Some("KiUserExceptionDispatcher");
                #[cfg(target_arch = "x86_64")]
                let is_ki_user_exc = false;
                #[cfg(not(target_arch = "x86_64"))]
                let ntdll_ssn = if is_ntdll {
                    e.name.as_deref().and_then(win32_rt::ntsyscall::lookup_ssn_by_name)
                } else {
                    None
                };
                #[cfg(target_arch = "x86_64")]
                let ntdll_ssn: Option<u32> = None;
                if is_ki_user_exc {
                    let disp = (mrdata as isize - (thunk as isize + 8)) as i32;
                    let d = disp.to_le_bytes();
                    w.bytes(
                        thunk,
                        &[
                            0xFC, // cld
                            0x48, 0x8B, 0x05, d[0], d[1], d[2], d[3], // mov rax, [rip + disp32]
                            0x48, 0x85, 0xC0, // test rax, rax
                            0x74, 0x02, // jz +2
                            0xFF, 0xD0, // call rax
                            0xC3, // ret
                        ],
                    );
                } else if let Some(ssn) = ntdll_ssn {
                    let stub = win32_rt::ntsyscall::encode_ntdll_syscall_stub(ssn);
                    w.bytes(thunk, &stub);
                } else {
                    #[cfg(target_arch = "x86_64")]
                    {
                        // jmp qword ptr [rip+0]; dq target; int3 int3
                        w.bytes(thunk, &[0xFF, 0x25, 0, 0, 0, 0]);
                        w.u64(thunk + 6, e.target as u64);
                        w.bytes(thunk + 14, &[0xCC, 0xCC]);
                    }
                    #[cfg(not(target_arch = "x86_64"))]
                    {
                        // On ARM64, direct calls dispatch via `register_native_va(thunk_va, host_fn)`.
                        // Avoid `FF 25 <rip+0> <host_arm64_ptr>` here: guest import-forwarder /
                        // hook scanners follow `jmp [rip+disp32]` thunks and try to resolve the
                        // target inside PEB->Ldr, crashing on the non-PE Mach-O host pointer.
                        w.bytes(
                            thunk,
                            &[
                                0x48, 0x83, 0xEC, 0x28, // sub rsp, 0x28
                                0x48, 0x83, 0xC4, 0x28, // add rsp, 0x28
                                0xC3, // ret
                                0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC,
                            ],
                        );
                    }
                }
                w.u32(funcs + 4 * idx, thunk as u32);
                let a = base + thunk;
                thunk += THUNK_SIZE;
                if let Some(n) = &e.name {
                    thunk_names.insert(a, n.clone());
                }
                #[cfg(not(target_arch = "x86_64"))]
                if base != 0 {
                    x86_runtime::exe::register_native_va(a as u64, e.target);
                }
                a
            } else {
                match e.target.checked_sub(base) {
                    Some(r) if r < u32::MAX as usize && r != 0 => w.u32(funcs + 4 * idx, r as u32),
                    _ => unreachable_data.push(e.name.clone().unwrap_or_else(|| format!("#{}", ords[i]))),
                }
                e.target
            };
            if let Some(n) = &e.name {
                by_name.insert(n.clone(), addr);
            }
            // Only declared ordinals are resolvable: auto-assigned ones exist
            // in the EAT but do not match real Windows numbering, so an
            // ordinal import of something we lack must still trap.
            if let (Some(o), true) = (u16::try_from(ords[i]).ok(), e.ordinal.is_some()) {
                by_ordinal.insert(o, addr);
            }
        }
        for (k, &i) in named.iter().enumerate() {
            let n = exps[i].name.as_ref().unwrap();
            w.u32(names + 4 * k, name_rvas[k] as u32);
            w.u16(name_ords + 2 * k, (ords[i] - ord_base) as u16);
            w.bytes(name_rvas[k], n.as_bytes());
        }
    }
    BuiltImage {
        bytes: img,
        size,
        text: (text, text_end),
        by_name,
        by_ordinal,
        thunk_names,
        unreachable_data,
        wow64_prepare_slot: (is_ntdll && base != 0).then_some(base + mrdata),
    }
}

// ---------------------------------------------------------------------------
// Host introspection: is an address code, and where is the host binary?
// ---------------------------------------------------------------------------

/// `__TEXT` ranges of host Mach-O images, cached per image base.
fn text_range_of(fbase: usize) -> Option<(usize, usize)> {
    use std::sync::Mutex;
    static CACHE: Mutex<BTreeMap<usize, Option<(usize, usize)>>> = Mutex::new(BTreeMap::new());
    let mut c = CACHE.lock().unwrap();
    *c.entry(fbase).or_insert_with(|| unsafe { parse_text_segment(fbase) })
}

/// Walk the Mach-O load commands of the image at `fbase` for the
/// `__TEXT,__text` section (immutable statics live in `__TEXT,__const`, so
/// the whole segment would misclassify data as code).
unsafe fn parse_text_segment(fbase: usize) -> Option<(usize, usize)> {
    const MH_MAGIC_64: u32 = 0xfeed_facf;
    const LC_SEGMENT_64: u32 = 0x19;
    let rd32 = |a: usize| std::ptr::read_unaligned(a as *const u32);
    let rd64 = |a: usize| std::ptr::read_unaligned(a as *const u64);
    if rd32(fbase) != MH_MAGIC_64 {
        return None;
    }
    let ncmds = rd32(fbase + 16);
    let mut lc = fbase + 32;
    let mut seg_vmaddr = None;
    let mut text = None;
    for _ in 0..ncmds {
        let cmd = rd32(lc);
        let size = rd32(lc + 4) as usize;
        if cmd == LC_SEGMENT_64 {
            let name = std::slice::from_raw_parts((lc + 8) as *const u8, 16);
            if name.starts_with(b"__TEXT\0") {
                seg_vmaddr = Some(rd64(lc + 24));
                let nsects = rd32(lc + 64) as usize;
                for i in 0..nsects {
                    let sect = lc + 72 + i * 80;
                    let sname = std::slice::from_raw_parts(sect as *const u8, 16);
                    if sname.starts_with(b"__text\0") {
                        text = Some((rd64(sect + 32), rd64(sect + 40)));
                    }
                }
            }
        }
        if size == 0 {
            break;
        }
        lc += size;
    }
    let (addr, len) = text?;
    let slide = fbase as u64 - seg_vmaddr?;
    Some(((addr + slide) as usize, (addr + slide + len) as usize))
}

/// True if `addr` lies in the `__TEXT` segment of a loaded host image.
pub fn is_host_code(addr: usize) -> bool {
    unsafe {
        let mut info: libc::Dl_info = std::mem::zeroed();
        if libc::dladdr(addr as *const libc::c_void, &mut info) == 0 || info.dli_fbase.is_null() {
            return false;
        }
        match text_range_of(info.dli_fbase as usize) {
            Some((s, e)) => addr >= s && addr < e,
            None => false,
        }
    }
}

/// Base address of the host loader binary (its Mach-O header).
pub fn host_binary_base() -> usize {
    unsafe {
        let mut info: libc::Dl_info = std::mem::zeroed();
        let f = host_binary_base as *const () as usize;
        if libc::dladdr(f as *const libc::c_void, &mut info) != 0 {
            info.dli_fbase as usize
        } else {
            0
        }
    }
}

/// `[start, end)` address range of the host loader binary's `__TEXT,__text` section.
#[cfg_attr(target_arch = "x86_64", allow(dead_code))]
pub fn host_text_range() -> Option<(usize, usize)> {
    text_range_of(host_binary_base())
}

/// Immutable, W^X-separated guest-visible thunk page for a fixed, closed table of internal
/// host entry points (e.g. verified Winsock2 extension functions).
pub struct FixedThunkPage {
    pub base: usize,
    pub thunks: Vec<(usize, &'static str)>,
}

impl FixedThunkPage {
    pub fn symbol_for(&self, addr: usize) -> Option<&'static str> {
        if addr < self.base || addr >= self.base + PAGE {
            return None;
        }
        let aligned = addr & !(THUNK_SIZE - 1);
        self.thunks.iter().find_map(|&(va, name)| (va == aligned).then_some(name))
    }
}

/// Provision an immutable single-page W^X thunk table for a fixed, closed list of
/// `(symbol_name, host_target)` entries.
///
/// - On ARM64 (macOS & iOS), writes harmless x86-64 stub bytes containing **no** host pointers,
///   switches the page to `PROT_READ`, registers guest `PAGE_EXECUTE_READ` metadata, and binds
///   each thunk VA to its fixed `host_target` via `x86_runtime::exe::register_native_va`.
/// - On `x86_64`, emits standard 16-byte `jmp [rip+0]` thunks and switches the page to
///   `PROT_READ | PROT_EXEC` (strict W^X).
pub fn build_fixed_thunk_page(entries: &[(&'static str, usize)]) -> FixedThunkPage {
    assert!(
        entries.len() * THUNK_SIZE <= PAGE,
        "fixed thunk table exceeds one page"
    );
    let base = unsafe { map_near_host(PAGE) };
    let mut thunks = Vec::with_capacity(entries.len());
    unsafe {
        std::ptr::write_bytes(base as *mut u8, 0xCC, PAGE);
        for (i, &(name, target)) in entries.iter().enumerate() {
            let off = i * THUNK_SIZE;
            let thunk_va = base + off;
            #[cfg(target_arch = "x86_64")]
            {
                let mut stub = [0xCCu8; THUNK_SIZE];
                stub[0..6].copy_from_slice(&[0xFF, 0x25, 0, 0, 0, 0]);
                stub[6..14].copy_from_slice(&(target as u64).to_le_bytes());
                std::ptr::copy_nonoverlapping(stub.as_ptr(), thunk_va as *mut u8, THUNK_SIZE);
            }
            #[cfg(not(target_arch = "x86_64"))]
            {
                let stub: [u8; THUNK_SIZE] = [
                    0x48, 0x83, 0xEC, 0x28, // sub rsp, 0x28
                    0x48, 0x83, 0xC4, 0x28, // add rsp, 0x28
                    0xC3, // ret
                    0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC, 0xCC,
                ];
                std::ptr::copy_nonoverlapping(stub.as_ptr(), thunk_va as *mut u8, THUNK_SIZE);
                x86_runtime::exe::register_native_va(thunk_va as u64, target);
            }
            thunks.push((thunk_va, name));
        }
        #[cfg(target_arch = "x86_64")]
        let prot = libc::PROT_READ | libc::PROT_EXEC;
        #[cfg(not(target_arch = "x86_64"))]
        let prot = libc::PROT_READ;
        let _ = libc::mprotect(base as *mut libc::c_void, PAGE, prot);
    }
    win32_rt::kernel32::memory::register_image(
        base,
        PAGE,
        &[(base, base + PAGE, win32_rt::kernel32::memory::PAGE_EXECUTE_READ)],
    );
    FixedThunkPage { base, thunks }
}

/// Map `size` bytes (RW) for a builtin image, preferring a spot just below
/// the host binary so its statics are within 4 GB above the image base
/// (data exports need 32-bit RVAs).
pub unsafe fn map_near_host(size: usize) -> usize {
    let rw = libc::PROT_READ | libc::PROT_WRITE;
    let flags = libc::MAP_PRIVATE | libc::MAP_ANON;
    // With the loader's 64 KB __PAGEZERO the host binary itself sits low
    // (e.g. 0x4ca6000), so search downwards from just below it.
    let host = host_binary_base();
    if host > size + 0x2_0000 {
        let mut hint = (host - size - 0x1_0000) & !0xFFFF;
        let lowest = host.saturating_sub(0xC000_0000).max(0x1_0000);
        while hint >= lowest {
            let p = libc::mmap(hint as *mut libc::c_void, size, rw, flags, -1, 0);
            if p != libc::MAP_FAILED {
                let p = p as usize;
                if p < host && host - p < 0xD000_0000 {
                    return p;
                }
                libc::munmap(p as *mut libc::c_void, size);
            }
            if hint < lowest + 0x1_0000 {
                break;
            }
            hint -= 0x1_0000;
        }
    }
    let p = libc::mmap(std::ptr::null_mut(), size, rw, flags, -1, 0);
    assert!(p != libc::MAP_FAILED, "builtin image mmap failed");
    p as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    unsafe extern "win64" fn f1() -> u32 {
        1
    }
    unsafe extern "win64" fn f2() -> u32 {
        2
    }
    static DATA: u32 = 0xC0FFEE;

    #[test]
    fn export_directory_parses_and_thunks_jump() {
        let ex = vec![
            ImageExport { name: Some("Zeta".into()), ordinal: None, target: f2 as *const () as usize, is_code: true },
            ImageExport { name: Some("Alpha".into()), ordinal: Some(101), target: f1 as *const () as usize, is_code: true },
            ImageExport { name: Some("Data".into()), ordinal: None, target: &DATA as *const u32 as usize, is_code: false },
            ImageExport { name: Some("Alpha".into()), ordinal: None, target: f2 as *const () as usize, is_code: true }, // dup ignored
        ];
        let size_guess = 0x4000;
        let base = unsafe { map_near_host(size_guess) };
        let img = build("test.dll", base, &ex);
        assert!(img.size <= size_guess);
        // Parse with pe-image as a file (file layout == memory layout).
        let pe = pe_image::Pe::parse(&img.bytes).unwrap();
        let e = pe.exports().unwrap();
        assert_eq!(e.dll_name, "test.dll");
        assert_eq!(e.ordinal_base, 101);
        let a = e.by_name("Alpha").unwrap();
        assert_eq!(a.ordinal, 101);
        let z = e.by_name("Zeta").unwrap();
        // Test binaries keep the default __PAGEZERO, so memory below the host
        // binary may be unavailable here; the loader binary (64 KB pagezero) is
        // where data RVAs matter.
        let data_ok = img.unreachable_data.is_empty();
        assert!(z.ordinal > 101);
        // Names are sorted (binary-search contract).
        let dir = pe.headers.dir(0).rva;
        let n = pe.u32_at(dir + 24).unwrap();
        let names_rva = pe.u32_at(dir + 32).unwrap();
        let mut prev = String::new();
        for i in 0..n {
            let s = pe.cstr_at(pe.u32_at(names_rva + 4 * i).unwrap()).unwrap();
            assert!(s > prev);
            prev = s;
        }
        // Map it executable and call through the EAT like sl.interposer does.
        unsafe {
            std::ptr::copy_nonoverlapping(img.bytes.as_ptr(), base as *mut u8, img.bytes.len());
            #[cfg(target_arch = "x86_64")]
            {
                libc::mprotect((base + img.text.0) as *mut libc::c_void, img.text.1 - img.text.0, libc::PROT_READ | libc::PROT_EXEC);
                let pe_image::ExportTarget::Rva(r) = a.target else { panic!() };
                let f: unsafe extern "win64" fn() -> u32 = std::mem::transmute(base + r as usize);
                assert_eq!(f(), 1);
                assert_eq!(img.by_name["Alpha"], base + r as usize);
                let pe_image::ExportTarget::Rva(r) = z.target else { panic!() };
                let f: unsafe extern "win64" fn() -> u32 = std::mem::transmute(base + r as usize);
                assert_eq!(f(), 2);
            }
            #[cfg(not(target_arch = "x86_64"))]
            {
                let pe_image::ExportTarget::Rva(r) = a.target else { panic!() };
                assert_eq!(img.by_name["Alpha"], base + r as usize);
                assert_eq!(x86_runtime::exe::call_va8((base + r as usize) as u64, 0, 0, 0, 0, 0, 0, 0, 0) as u32, 1);
                let pe_image::ExportTarget::Rva(r) = z.target else { panic!() };
                assert_eq!(x86_runtime::exe::call_va8((base + r as usize) as u64, 0, 0, 0, 0, 0, 0, 0, 0) as u32, 2);
            }
            if data_ok {
                let d = e.by_name("Data").unwrap();
                assert!(d.ordinal > 101 && d.ordinal != z.ordinal);
                let pe_image::ExportTarget::Rva(r) = d.target else { panic!() };
                assert_eq!(*((base + r as usize) as *const u32), 0xC0FFEE);
            }
        }
        assert!(is_host_code(f1 as *const () as usize));
        assert!(!is_host_code(&DATA as *const u32 as usize));
    }

    #[cfg(not(target_arch = "x86_64"))]
    #[test]
    fn ntdll_exports_emit_canonical_syscall_stubs() {
        let ex = vec![
            ImageExport { name: Some("KiUserExceptionDispatcher".into()), ordinal: None, target: f2 as *const () as usize, is_code: true },
            ImageExport { name: Some("NtCreateSection".into()), ordinal: None, target: f1 as *const () as usize, is_code: true },
            ImageExport { name: Some("ZwCreateSection".into()), ordinal: None, target: f1 as *const () as usize, is_code: true },
            ImageExport { name: Some("RtlGetVersion".into()), ordinal: None, target: f2 as *const () as usize, is_code: true },
        ];
        let base = 0x1000_0000usize;
        let img = build("ntdll.dll", base, &ex);
        let slot = img.wow64_prepare_slot.expect("ntdll must expose Wow64PrepareForException slot");
        let pe = pe_image::Pe::parse(&img.bytes).unwrap();
        let e = pe.exports().unwrap();
        // Verify KiUserExceptionDispatcher starts with `cld; mov rax, [rip + disp32]` pointing at `slot`.
        let pe_image::ExportTarget::Rva(ki_rva) = e.by_name("KiUserExceptionDispatcher").unwrap().target else { panic!() };
        let ki_bytes = &img.bytes[ki_rva as usize..ki_rva as usize + 16];
        assert_eq!(&ki_bytes[0..4], &[0xFC, 0x48, 0x8B, 0x05]);
        let disp = i32::from_le_bytes(ki_bytes[4..8].try_into().unwrap()) as isize;
        assert_eq!((base as isize + ki_rva as isize + 8 + disp) as usize, slot);

        for sym in ["NtCreateSection", "ZwCreateSection"] {
            let pe_image::ExportTarget::Rva(rva) = e.by_name(sym).unwrap().target else { panic!() };
            let stub = &img.bytes[rva as usize..rva as usize + 16];
            let expected = win32_rt::ntsyscall::encode_ntdll_syscall_stub(win32_rt::ntsyscall::ssn::NT_CREATE_SECTION);
            assert_eq!(stub, &expected, "{sym} stub mismatch");
        }
        let pe_image::ExportTarget::Rva(rva) = e.by_name("RtlGetVersion").unwrap().target else { panic!() };
        assert_eq!(&img.bytes[rva as usize..rva as usize + 9], &[0x48, 0x83, 0xEC, 0x28, 0x48, 0x83, 0xC4, 0x28, 0xC3]);
    }
}
