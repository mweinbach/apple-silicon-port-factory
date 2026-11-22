//! DXBC (Shader Model 4/5) → DXIL through Microsoft's open-source `dxilconv` (DXC's
//! `projects/dxilconv`, NCSA license), so SM5 shaders can go through the Metal Shader Converter,
//! which only reads DXIL.
//!
//! Upstream builds dxilconv only on Windows. `tools/dxilconv/build.sh` builds it for macOS
//! (`libdxilconv.dylib`) from a pinned DXC checkout plus our small portability patch. The library
//! is `dlopen`ed when first needed, and it is optional: without it, DXBC input fails with a clear
//! error, as it did before.
//!
//! Search order: `$AW2_DXILCONV`, then next to the executable (`../Frameworks`, same dir), then
//! `~/.local/lib`, `/opt/homebrew/lib` and `/usr/local/lib`.

use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use crate::Error;

#[repr(C)]
struct Guid {
    d1: u32,
    d2: u16,
    d3: u16,
    d4: [u8; 8],
}

/// CLSID_DxbcConverter {4900391E-B752-4EDD-A885-6FB76E25ADDB}
const CLSID_DXBC_CONVERTER: Guid = Guid { d1: 0x4900391e, d2: 0xb752, d3: 0x4edd, d4: [0xa8, 0x85, 0x6f, 0xb7, 0x6e, 0x25, 0xad, 0xdb] };
/// IID_IDxbcConverter {5F956ED5-78D1-4B15-8247-F7187614A041}
const IID_IDXBC_CONVERTER: Guid = Guid { d1: 0x5f956ed5, d2: 0x78d1, d3: 0x4b15, d4: [0x82, 0x47, 0xf7, 0x18, 0x76, 0x14, 0xa0, 0x41] };

type CreateInstance = unsafe extern "C" fn(clsid: *const Guid, iid: *const Guid, out: *mut *mut c_void) -> i32;

/// `IDxbcConverter` vtable. DXC's non-Windows `IUnknown` has no virtual destructor, so the
/// layout is the plain COM one, and STDMETHODCALLTYPE is the C ABI.
#[repr(C)]
struct Vtbl {
    query_interface: usize,
    add_ref: unsafe extern "C" fn(*mut c_void) -> u32,
    release: unsafe extern "C" fn(*mut c_void) -> u32,
    convert: unsafe extern "C" fn(
        this: *mut c_void,
        dxbc: *const c_void,
        size: u32,
        extra_options: *const libc_wchar,
        dxil: *mut *mut c_void,
        dxil_size: *mut u32,
        diag: *mut *mut libc_wchar,
    ) -> i32,
}

/// `wchar_t` on macOS (UTF-32).
#[allow(non_camel_case_types)]
type libc_wchar = i32;

extern "C" {
    fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlerror() -> *const c_char;
    fn free(p: *mut c_void);
}
const RTLD_NOW: c_int = 0x2;
const RTLD_LOCAL: c_int = 0x4;

struct Lib {
    create: CreateInstance,
    path: PathBuf,
}

/// Candidate locations of `libdxilconv.dylib`, in search order.
pub fn candidate_paths() -> Vec<PathBuf> {
    let mut c: Vec<PathBuf> = Vec::new();
    if let Some(p) = std::env::var_os("AW2_DXILCONV").filter(|p| !p.is_empty()) {
        c.push(p.into());
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            c.push(dir.join("../Frameworks/libdxilconv.dylib"));
            c.push(dir.join("libdxilconv.dylib"));
        }
    }
    if let Some(home) = std::env::var_os("HOME") {
        c.push(Path::new(&home).join(".local/lib/libdxilconv.dylib"));
    }
    c.push("/opt/homebrew/lib/libdxilconv.dylib".into());
    c.push("/usr/local/lib/libdxilconv.dylib".into());
    c
}

static LIB: OnceLock<Result<Lib, Error>> = OnceLock::new();
/// dxilconv keeps LLVM state per instance, but conversions are rare (a few SM5 shaders), so
/// serialize them rather than rely on the library's thread safety.
static LOCK: Mutex<()> = Mutex::new(());

fn lib() -> Result<&'static Lib, Error> {
    LIB.get_or_init(load).as_ref().map_err(Clone::clone)
}

fn load() -> Result<Lib, Error> {
    let mut tried = Vec::new();
    for cand in candidate_paths() {
        if !cand.exists() {
            continue;
        }
        let Ok(cpath) = CString::new(cand.to_string_lossy().as_bytes()) else { continue };
        // SAFETY: plain dlopen/dlsym of an existing file; the handle is never closed.
        unsafe {
            let h = dlopen(cpath.as_ptr(), RTLD_NOW | RTLD_LOCAL);
            if h.is_null() {
                let m = dlerror();
                tried.push(format!("{}: {}", cand.display(), if m.is_null() { String::new() } else { CStr::from_ptr(m).to_string_lossy().into_owned() }));
                continue;
            }
            let sym = dlsym(h, c"DxcCreateInstance".as_ptr());
            if sym.is_null() {
                tried.push(format!("{}: no DxcCreateInstance export", cand.display()));
                continue;
            }
            return Ok(Lib { create: std::mem::transmute::<*mut c_void, CreateInstance>(sym), path: cand });
        }
    }
    Err(Error::Load(if tried.is_empty() {
        "libdxilconv.dylib not found (build it with crates/dxil-metal/tools/dxilconv/build.sh, or set AW2_DXILCONV)".into()
    } else {
        format!("failed to load libdxilconv.dylib: {}", tried.join("; "))
    }))
}

/// True when `libdxilconv.dylib` can be loaded.
pub fn available() -> bool {
    lib().is_ok()
}

/// Path of the loaded library (for diagnostics and cache fingerprints).
pub fn library_path() -> Option<&'static Path> {
    lib().ok().map(|l| l.path.as_path())
}

/// Converts a DXBC container with SM4/SM5 bytecode into a DXIL container. The output is
/// unsigned, as dxilconv runs no validator; the caller signs it if needed.
pub fn convert(dxbc: &[u8]) -> Result<Vec<u8>, Error> {
    let lib = lib()?;
    let size = u32::try_from(dxbc.len()).map_err(|_| Error::InvalidInput("DXBC too large".into()))?;
    let _g = LOCK.lock().unwrap_or_else(|p| p.into_inner());
    // SAFETY: COM calls through the documented IDxbcConverter vtable. Output buffers are
    // malloc'ed by the library (CoTaskMemAlloc = malloc off Windows) and freed here.
    unsafe {
        let mut obj: *mut c_void = std::ptr::null_mut();
        let hr = (lib.create)(&CLSID_DXBC_CONVERTER, &IID_IDXBC_CONVERTER, &mut obj);
        if hr < 0 || obj.is_null() {
            return Err(Error::Load(format!("DxcCreateInstance(DxbcConverter) failed: {hr:#x}")));
        }
        let vtbl = &**(obj as *mut *const Vtbl);
        let mut out: *mut c_void = std::ptr::null_mut();
        let mut out_size = 0u32;
        let mut diag: *mut libc_wchar = std::ptr::null_mut();
        let hr = (vtbl.convert)(obj, dxbc.as_ptr().cast(), size, std::ptr::null(), &mut out, &mut out_size, &mut diag);
        let message = take_wide(diag);
        let result = if hr >= 0 && !out.is_null() {
            Ok(std::slice::from_raw_parts(out as *const u8, out_size as usize).to_vec())
        } else {
            Err(Error::Convert {
                code: crate::ErrorCode::Unknown,
                message: format!("dxilconv: DXBC->DXIL failed ({hr:#x}){}", if message.is_empty() { String::new() } else { format!(": {message}") }),
            })
        };
        if !out.is_null() {
            free(out);
        }
        (vtbl.release)(obj);
        result
    }
}

/// [`convert`], then signs the container (the Metal Shader Converter doesn't need the
/// checksum, but D3D tooling and our cache dumps expect a valid one).
pub fn convert_signed(dxbc: &[u8]) -> Result<Vec<u8>, Error> {
    let mut dxil = convert(dxbc)?;
    crate::dxbc::sign_container(&mut dxil)?;
    Ok(dxil)
}

/// Copies and frees a malloc'ed UTF-32 string from the library.
unsafe fn take_wide(p: *mut libc_wchar) -> String {
    if p.is_null() {
        return String::new();
    }
    let mut s = String::new();
    let mut q = p;
    while *q != 0 {
        s.push(char::from_u32(*q as u32).unwrap_or('?'));
        q = q.add(1);
    }
    free(p.cast());
    s.trim_end().to_string()
}
