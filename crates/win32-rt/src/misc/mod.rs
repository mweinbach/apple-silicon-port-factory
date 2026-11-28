//! Smaller builtin DLLs, one file per DLL (`misc/<dll>.rs`).
//!
//! Each file defines its `extern "win64"` exports plus `EXTRA_EXPORTS`
//! (data / C++-mangled names) and `ORDINALS` (ordinal -> name).
//! DLLs without a file here (d3d12, dxgi, d3d11, ...) are unknown to the loader: their imports become trap thunks
//! and LoadLibrary of them fails with ERROR_MOD_NOT_FOUND.

use crate::{BuiltinDll, Export};

pub mod amd_ags_x64;
pub mod amd_fidelityfx_loader_dx12;
pub mod avifil32;
pub mod avrt;
pub mod bcrypt;
pub mod bink2w64;
pub(crate) mod clr;
pub mod cohtml_icuuc;
pub mod cohtml_windowsdesktop;
pub mod comdlg32;
pub mod coreclr;
pub mod crypt32;
pub mod d2d1;
pub mod d3dcompiler_47;
pub mod dbghelp;
pub mod dinput8;
pub mod directml;
pub mod dsound;
pub mod dstorage;
pub mod dwmapi;
pub mod dwrite;
pub mod faultrep;
pub mod fmod;
pub mod gameinput;
pub mod gdi32;
pub mod hid;
pub mod hostfxr;
pub mod imagehlp;
pub mod imm32;
pub mod iphlpapi;
pub mod libcurl;
pub mod libprotobuf;
pub mod libxess;
pub mod libxess_dx11;
pub mod lz4;
pub(crate) mod mediadecoders;
pub mod mediadecoders_windowsdesktop;
pub mod mfc140u;
pub mod mfplat;
pub mod mmdevapi;
pub mod mono_2_0_bdwgc;
pub mod mscoree;
pub mod msctf;
pub mod msvcp140;
pub mod msvcp140_codecvt_ids;
pub mod normaliz;
pub mod nrd;
pub mod nri;
pub mod nvapi64;
pub mod nvcloth_x64;
pub mod nvcuda;
pub mod nvngx_dlss;
pub mod ole32;
pub mod oleaut32;
pub mod openal32;
pub mod opencl;
pub mod pdh;
pub mod physx3_x64;
pub mod physx3common_x64;
pub mod physx3cooking_x64;
pub mod physxcommon_64;
pub mod physxfoundation_64;
pub mod powrprof;
pub mod pxfoundation_x64;
pub mod rpcrt4;
pub mod secur32;
pub mod setupapi;
pub mod shell32;
pub mod steam_api64;
pub mod tbbmalloc;
pub mod uxtheme;
pub mod v8_zlib;
pub mod vcruntime140;
pub mod vcruntime140_1;
pub mod version;
pub mod vulkan_1;
pub mod winhttp;
pub mod winmm;
pub mod wintrust;
pub mod wldap32;
pub mod ws2_32;
pub mod xaudio2_9redist;
pub mod xinput1_4;
pub mod zlib1;
pub mod zstd;

type Entry = (&'static str, &'static [Export], &'static [Export], &'static [(u16, &'static str)]);

static LIST: &[Entry] = include!(concat!(env!("OUT_DIR"), "/misc_dlls.rs"));

pub fn dlls() -> Vec<BuiltinDll> {
    LIST.iter()
        .map(|(name, ex, extra, ords)| {
            let mut v: Vec<Export> = ex.to_vec();
            v.extend_from_slice(extra);
            BuiltinDll { name, exports: Box::leak(v.into_boxed_slice()), ordinals: ords }
        })
        .collect()
}

/// Random (version 4) GUID bytes.
pub fn new_guid() -> [u8; 16] {
    let mut g = [0u8; 16];
    unsafe { libc::arc4random_buf(g.as_mut_ptr() as *mut libc::c_void, 16) };
    // Data3 high nibble = 4 (little-endian u16 at bytes 6..8), variant 10xx.
    g[7] = (g[7] & 0x0F) | 0x40;
    g[8] = (g[8] & 0x3F) | 0x80;
    g
}

/// `{XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX}` from a GUID in memory.
pub unsafe fn guid_string(p: *const u8) -> String {
    let b = std::slice::from_raw_parts(p, 16);
    let d1 = u32::from_le_bytes(b[0..4].try_into().unwrap());
    let d2 = u16::from_le_bytes(b[4..6].try_into().unwrap());
    let d3 = u16::from_le_bytes(b[6..8].try_into().unwrap());
    format!(
        "{{{d1:08X}-{d2:04X}-{d3:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}}}",
        b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]
    )
}
