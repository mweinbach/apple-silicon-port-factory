//! shell32.dll: known folders, CommandLineToArgvW, `SHFileOperationA/W`,
//! `SHGetFileInfoA/W`, path utilities. `ShellExecute*` safely refuses host
//! process launching.

use crate::strings::{from_wide, write_wide_trunc};
use crate::types::*;
use std::ffi::c_void;
use std::path::{Path, PathBuf};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[(680, "IsUserAnAdmin")];

fn user() -> String {
    std::env::var("USER").unwrap_or_else(|_| "player".into())
}

/// Windows path for a CSIDL value (resolves via `crate::path` to `~/Translator/*`).
fn csidl_path(csidl: i32) -> Option<String> {
    let u = user();
    Some(match csidl & 0xFF {
        0x00 | 0x10 => format!("C:\\users\\{u}\\Desktop"), // CSIDL_DESKTOP / CSIDL_DESKTOPDIRECTORY
        0x02 => format!("C:\\users\\{u}\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs"),
        0x05 => format!("C:\\users\\{u}\\Documents"),
        0x06 => format!("C:\\users\\{u}\\Favorites"),
        0x07 => format!("C:\\users\\{u}\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Startup"),
        0x08 => format!("C:\\users\\{u}\\AppData\\Roaming\\Microsoft\\Windows\\Recent"),
        0x0B => format!("C:\\users\\{u}\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu"),
        0x0D => format!("C:\\users\\{u}\\Music"),
        0x0E => format!("C:\\users\\{u}\\Videos"),
        0x14 => "C:\\Windows\\Fonts".into(),
        0x15 => format!("C:\\users\\{u}\\AppData\\Roaming\\Microsoft\\Windows\\Templates"),
        0x19 => "C:\\users\\Public\\Desktop".into(),
        0x1A => format!("C:\\users\\{u}\\AppData\\Roaming"),
        0x1C => format!("C:\\users\\{u}\\AppData\\Local"),
        0x20 => format!("C:\\users\\{u}\\AppData\\Local\\Microsoft\\Windows\\INetCache"),
        0x21 => format!("C:\\users\\{u}\\AppData\\Local\\Microsoft\\Windows\\INetCookies"),
        0x23 => "C:\\ProgramData".into(),
        0x24 => "C:\\Windows".into(),
        0x25 => "C:\\Windows\\System32".into(),
        0x26 => "C:\\Program Files".into(),
        0x27 => format!("C:\\users\\{u}\\Pictures"),
        0x28 => format!("C:\\users\\{u}"),
        0x29 => "C:\\Windows\\SysWOW64".into(),
        0x2A => "C:\\Program Files (x86)".into(),
        0x2B => "C:\\Program Files\\Common Files".into(),
        0x2C => "C:\\Program Files (x86)\\Common Files".into(),
        0x2E => "C:\\users\\Public\\Documents".into(),
        0x35 => "C:\\users\\Public\\Music".into(),
        0x36 => "C:\\users\\Public\\Pictures".into(),
        0x37 => "C:\\users\\Public\\Videos".into(),
        _ => return None,
    })
}

fn known_folder_path(guid: &str) -> Option<String> {
    let u = user();
    Some(match guid {
        // AppData
        "{F1B32785-6FBA-4FCF-9D55-7B8E7F157091}" => format!("C:\\users\\{u}\\AppData\\Local"),
        "{A520A1A4-1780-4FF6-BD18-167343C5AF16}" => format!("C:\\users\\{u}\\AppData\\LocalLow"),
        "{3EB685DB-65F9-4CF6-A03A-E3EF65729F3D}" => format!("C:\\users\\{u}\\AppData\\Roaming"),
        // User folders
        "{FDD39AD0-238F-46AF-ADB4-6C85480369C7}" => format!("C:\\users\\{u}\\Documents"),
        "{B4BFCC3A-DB2C-424C-B029-7FE99A87C641}" => format!("C:\\users\\{u}\\Desktop"),
        "{374DE290-123F-4565-9164-39C4925E467B}" => format!("C:\\users\\{u}\\Downloads"),
        "{4C5C32FF-BB9D-43B0-B5B4-2D72E54EAAA4}" => format!("C:\\users\\{u}\\Saved Games"),
        "{33E28130-4E1E-4676-835A-98395C3BC3BB}" => format!("C:\\users\\{u}\\Pictures"),
        "{B7BEDE81-DF94-4682-A7D8-57A52620B86F}" => format!("C:\\users\\{u}\\Pictures\\Screenshots"),
        "{4BD8D571-6D19-48D3-BE97-422220080E43}" => format!("C:\\users\\{u}\\Music"),
        "{18989B1D-99B5-455B-841C-AB7C74E4DDFC}" => format!("C:\\users\\{u}\\Videos"),
        "{EDC0FE71-98D8-4F4A-B920-C8DC133CB165}" => format!("C:\\users\\{u}\\Videos\\Captures"),
        "{5E6C858F-0E22-4760-9AFE-EA3317B67173}" => format!("C:\\users\\{u}"),
        "{1777F761-68AD-4D8A-87BD-30B759FA33DD}" => format!("C:\\users\\{u}\\Favorites"),
        "{A63293E8-664E-48DB-A079-DF759E0509F7}" => format!("C:\\users\\{u}\\AppData\\Roaming\\Microsoft\\Windows\\Templates"),
        // Public / Shared folders
        "{62AB5D82-FDC1-4DC3-A9DD-070D1D495D97}" => "C:\\ProgramData".into(),
        "{DFDF76A2-C82A-4D63-906A-5644AC457385}" => "C:\\users\\Public".into(),
        "{ED4824AF-DCE4-45A8-81E2-FC7965083634}" => "C:\\users\\Public\\Documents".into(),
        "{C4AA340D-F20F-4863-AFEF-F87EF2E6BA25}" => "C:\\users\\Public\\Desktop".into(),
        "{3D644C9B-1FB8-4F30-9B45-F670235F79C0}" => "C:\\users\\Public\\Downloads".into(),
        "{B6EBFB86-6907-413C-9AF7-4FC2ABF07CC5}" => "C:\\users\\Public\\Pictures".into(),
        "{3214FAB5-9757-4298-BB61-92A9DEAA44FF}" => "C:\\users\\Public\\Music".into(),
        "{2400183A-6185-49FB-A2D8-4A392A602BA3}" => "C:\\users\\Public\\Videos".into(),
        // System & Program Files
        "{F38BF404-1D43-42F2-9305-67DE0B28FC23}" => "C:\\Windows".into(),
        "{1AC14E77-02E7-4E5D-B744-2EB1AE5198B7}" => "C:\\Windows\\System32".into(),
        "{D65231B0-B2F1-4857-A4CE-A8E7C6EA7D27}" => "C:\\Windows\\SysWOW64".into(),
        "{FD228CB7-AE11-4AE3-864C-16F3910AB8FE}" => "C:\\Windows\\Fonts".into(),
        "{905E63B6-C1BF-494E-B29C-65B732D3D21A}" | "{6D809377-6AF0-444B-8957-A3773F02200E}" => "C:\\Program Files".into(),
        "{7C5A40EF-A0FB-4BFC-874A-C0F2E0B9FA8E}" => "C:\\Program Files (x86)".into(),
        "{F7F1ED05-9F6D-47A2-AAAE-29D317C6F066}" | "{6365D5A7-0F0D-45E5-87F6-0DA56B6A4F7D}" => "C:\\Program Files\\Common Files".into(),
        "{DE974D24-D9C6-4D3E-BF91-F4455120B917}" => "C:\\Program Files (x86)\\Common Files".into(),
        _ => return None,
    })
}

fn ensure_dir(win: &str) {
    if let Some(h) = crate::path::to_host(win) {
        let _ = std::fs::create_dir_all(h);
    }
}

pub unsafe extern "win64" fn SHGetFolderPathW(
    _hwnd: HWND,
    csidl: i32,
    _tok: HANDLE,
    _flags: u32,
    path: *mut u16,
) -> HRESULT {
    if path.is_null() {
        return E_INVALIDARG;
    }
    match csidl_path(csidl) {
        Some(p) => {
            if csidl & 0x8000 != 0 {
                ensure_dir(&p);
            }
            write_wide_trunc(&p, path, 260);
            S_OK
        }
        None => {
            log::debug!("SHGetFolderPathW: unknown CSIDL 0x{csidl:x} -> E_INVALIDARG");
            E_INVALIDARG
        }
    }
}

pub unsafe extern "win64" fn SHGetFolderPathA(
    _hwnd: HWND,
    csidl: i32,
    _tok: HANDLE,
    _flags: u32,
    path: *mut u8,
) -> HRESULT {
    if path.is_null() {
        return E_INVALIDARG;
    }
    match csidl_path(csidl) {
        Some(p) => {
            if csidl & 0x8000 != 0 {
                ensure_dir(&p);
            }
            crate::strings::write_ansi_trunc(&p, path, 260);
            S_OK
        }
        None => E_INVALIDARG,
    }
}

pub unsafe extern "win64" fn SHGetKnownFolderPath(
    id: *const u8,
    flags: u32,
    _tok: HANDLE,
    out: *mut *mut u16,
) -> HRESULT {
    if out.is_null() || id.is_null() {
        return E_INVALIDARG;
    }
    std::ptr::write_unaligned(out, std::ptr::null_mut());
    let g = super::guid_string(id);
    match known_folder_path(&g) {
        Some(p) => {
            const KF_FLAG_CREATE: u32 = 0x8000;
            if flags & KF_FLAG_CREATE != 0 {
                ensure_dir(&p);
            }
            let w = crate::strings::to_wide_z(&p);
            let mem = crate::heap::alloc(w.len() * 2, false) as *mut u16;
            if mem.is_null() {
                return E_OUTOFMEMORY;
            }
            std::ptr::copy_nonoverlapping(w.as_ptr(), mem, w.len());
            std::ptr::write_unaligned(out, mem);
            S_OK
        }
        None => {
            log::debug!("SHGetKnownFolderPath: unknown folder {g} -> ERROR_FILE_NOT_FOUND");
            0x8007_0002u32 as i32 // HRESULT_FROM_WIN32(ERROR_FILE_NOT_FOUND)
        }
    }
}

/// Returns a single LocalAlloc block: argv pointers followed by the strings.
pub unsafe extern "win64" fn CommandLineToArgvW(cmd: *const u16, argc: *mut i32) -> *mut *mut u16 {
    let s = if cmd.is_null() || std::ptr::read_unaligned(cmd) == 0 {
        crate::process::config().cmdline.clone()
    } else {
        from_wide(cmd)
    };
    let args = crate::process::split_cmdline(&s);
    let wides: Vec<Vec<u16>> = args.iter().map(|a| crate::strings::to_wide_z(a)).collect();
    let ptrs = (args.len() + 1) * 8;
    let total = ptrs + wides.iter().map(|w| w.len() * 2).sum::<usize>();
    let mem = crate::heap::alloc(total, true) as *mut u8;
    let arr = mem as *mut *mut u16;
    let mut off = ptrs;
    for (i, w) in wides.iter().enumerate() {
        let dst = mem.add(off) as *mut u16;
        std::ptr::copy_nonoverlapping(w.as_ptr(), dst, w.len());
        std::ptr::write_unaligned(arr.add(i), dst);
        off += w.len() * 2;
    }
    if !argc.is_null() {
        std::ptr::write_unaligned(argc, args.len() as i32);
    }
    arr
}

const SE_ERR_NOASSOC: usize = 31;
const ERROR_NO_ASSOCIATION: u32 = 1155;

pub unsafe extern "win64" fn ShellExecuteW(
    _h: HWND,
    op: *const u16,
    file: *const u16,
    params: *const u16,
    _dir: *const u16,
    _show: i32,
) -> HINSTANCE {
    log::warn!(
        "ShellExecuteW({:?}, {:?}, {:?}) refused (host process launching disabled)",
        crate::strings::from_wide_opt(op),
        crate::strings::from_wide_opt(file),
        crate::strings::from_wide_opt(params)
    );
    crate::teb::set_last_error(ERROR_NO_ASSOCIATION);
    SE_ERR_NOASSOC as HINSTANCE
}

pub unsafe extern "win64" fn ShellExecuteA(
    _h: HWND,
    op: *const u8,
    file: *const u8,
    params: *const u8,
    _dir: *const u8,
    _show: i32,
) -> HINSTANCE {
    log::warn!(
        "ShellExecuteA({:?}, {:?}, {:?}) refused (host process launching disabled)",
        crate::strings::from_ansi_opt(op),
        crate::strings::from_ansi_opt(file),
        crate::strings::from_ansi_opt(params)
    );
    crate::teb::set_last_error(ERROR_NO_ASSOCIATION);
    SE_ERR_NOASSOC as HINSTANCE
}

pub unsafe extern "win64" fn ShellExecuteExA(exec_info: *mut u8) -> BOOL {
    if !exec_info.is_null() {
        let cb = std::ptr::read_unaligned(exec_info as *const u32);
        if cb >= 64 {
            let verb = std::ptr::read_unaligned(exec_info.add(16) as *const LPCSTR);
            let file = std::ptr::read_unaligned(exec_info.add(24) as *const LPCSTR);
            log::warn!(
                "ShellExecuteExA({:?}, {:?}) refused (host process launching disabled)",
                crate::strings::from_ansi_opt(verb),
                crate::strings::from_ansi_opt(file)
            );
            std::ptr::write_unaligned(exec_info.add(56) as *mut usize, SE_ERR_NOASSOC);
        }
    }
    crate::teb::set_last_error(ERROR_NO_ASSOCIATION);
    FALSE
}

pub unsafe extern "win64" fn ShellExecuteExW(exec_info: *mut u8) -> BOOL {
    if !exec_info.is_null() {
        let cb = std::ptr::read_unaligned(exec_info as *const u32);
        if cb >= 64 {
            let verb = std::ptr::read_unaligned(exec_info.add(16) as *const LPCWSTR);
            let file = std::ptr::read_unaligned(exec_info.add(24) as *const LPCWSTR);
            log::warn!(
                "ShellExecuteExW({:?}, {:?}) refused (host process launching disabled)",
                crate::strings::from_wide_opt(verb),
                crate::strings::from_wide_opt(file)
            );
            std::ptr::write_unaligned(exec_info.add(56) as *mut usize, SE_ERR_NOASSOC);
        }
    }
    crate::teb::set_last_error(ERROR_NO_ASSOCIATION);
    FALSE
}

pub unsafe extern "win64" fn DragQueryFileA(_h: *mut c_void, _i: u32, _buf: *mut u8, _n: u32) -> u32 {
    0
}

pub unsafe extern "win64" fn DragQueryFileW(_h: *mut c_void, _i: u32, _buf: *mut u16, _n: u32) -> u32 {
    0
}

pub unsafe extern "win64" fn DragAcceptFiles(_hwnd: HWND, _accept: BOOL) {}

pub unsafe extern "win64" fn DragFinish(_hdrop: *mut c_void) {}

pub unsafe extern "win64" fn DragQueryPoint(_hdrop: *mut c_void, pt: *mut i32) -> BOOL {
    if !pt.is_null() {
        std::ptr::write_unaligned(pt, 0);
        std::ptr::write_unaligned(pt.add(1), 0);
    }
    FALSE
}

pub unsafe extern "win64" fn SHGetSpecialFolderPathA(
    hwnd: HWND,
    path: *mut u8,
    csidl: i32,
    create: BOOL,
) -> BOOL {
    let flags = if create != 0 { csidl | 0x8000 } else { csidl };
    (SHGetFolderPathA(hwnd, flags, std::ptr::null_mut(), 0, path) == S_OK) as BOOL
}

// ---------------------------------------------------------------------------
// `SHGetFileInfoA` / `SHGetFileInfoW`
// ---------------------------------------------------------------------------

const SHGFI_USEFILEATTRIBUTES: u32 = 0x0000_0010;
const SHGFI_DISPLAYNAME: u32 = 0x0000_0200;
const SHGFI_TYPENAME: u32 = 0x0000_0400;
const SHGFI_ATTRIBUTES: u32 = 0x0000_0800;
const SHGFI_EXETYPE: u32 = 0x0000_2000;
const SHGFI_SYSICONINDEX: u32 = 0x0000_4000;
const SHGFI_ATTR_SPECIFIED: u32 = 0x0002_0000;

const SFGAO_CANCOPY: u32 = 0x0000_0001;
const SFGAO_CANMOVE: u32 = 0x0000_0002;
const SFGAO_CANRENAME: u32 = 0x0000_0010;
const SFGAO_CANDELETE: u32 = 0x0000_0020;
const SFGAO_HASPROPSHEET: u32 = 0x0000_0040;
const SFGAO_READONLY: u32 = 0x0004_0000;
const SFGAO_HIDDEN: u32 = 0x0008_0000;
const SFGAO_FILESYSANCESTOR: u32 = 0x1000_0000;
const SFGAO_FOLDER: u32 = 0x2000_0000;
const SFGAO_FILESYSTEM: u32 = 0x4000_0000;
const SFGAO_HASSUBFOLDER: u32 = 0x8000_0000;

#[repr(C)]
struct SHFILEINFOA {
    h_icon: usize,
    i_icon: i32,
    dw_attributes: u32,
    sz_display_name: [u8; 260],
    sz_type_name: [u8; 80],
}
const _: () = assert!(std::mem::size_of::<SHFILEINFOA>() == 360);

#[repr(C)]
struct SHFILEINFOW {
    h_icon: usize,
    i_icon: i32,
    dw_attributes: u32,
    sz_display_name: [u16; 260],
    sz_type_name: [u16; 80],
}
const _: () = assert!(std::mem::size_of::<SHFILEINFOW>() == 696);

struct FileInfoResult {
    sfgao: u32,
    display_name: String,
    type_name: String,
    icon_index: i32,
    ret_val: usize,
}

fn describe_file_type(is_dir: bool, name: &str) -> String {
    if is_dir {
        return "File folder".into();
    }
    if let Some((_, ext)) = name.rsplit_once('.') {
        let up = ext.to_ascii_uppercase();
        return match up.as_str() {
            "EXE" => "Application".into(),
            "DLL" => "Application extension".into(),
            "TXT" => "Text Document".into(),
            "INI" => "Configuration settings".into(),
            "BAT" | "CMD" => "Windows Batch File".into(),
            "LNK" => "Shortcut".into(),
            "" => "File".into(),
            other => format!("{other} File"),
        };
    }
    "File".into()
}

fn display_name_for_win_path(win_path: &str) -> String {
    let trimmed = win_path.trim_end_matches(['\\', '/']);
    if trimmed.len() == 2 && trimmed.as_bytes()[1] == b':' {
        return format!("Local Disk ({})", trimmed.to_ascii_uppercase());
    }
    trimmed
        .rsplit(['\\', '/'])
        .next()
        .filter(|s| !s.is_empty())
        .unwrap_or(win_path)
        .to_string()
}

fn detect_pe_exetype(host_path: &Path) -> usize {
    let Ok(bytes) = std::fs::read(host_path) else {
        return 0;
    };
    if bytes.len() < 0x40 || &bytes[0..2] != b"MZ" {
        return 0;
    }
    let pe_off = u32::from_le_bytes(bytes[0x3C..0x40].try_into().unwrap()) as usize;
    if bytes.len() >= pe_off + 4 && &bytes[pe_off..pe_off + 4] == b"PE\0\0" {
        // Low word 'PE' (0x4550); high word 0x0004 for Windows GUI, 0x0000 for console.
        let subsystem = if bytes.len() >= pe_off + 0x5E {
            u16::from_le_bytes(bytes[pe_off + 0x5C..pe_off + 0x5E].try_into().unwrap())
        } else {
            3
        };
        let hi: usize = if subsystem == 2 { 0x0004 } else { 0 };
        (hi << 16) | 0x4550
    } else {
        0
    }
}

fn query_file_info(
    win_path: &str,
    file_attrs: u32,
    flags: u32,
    requested_mask: u32,
) -> Option<FileInfoResult> {
    let use_attrs = (flags & SHGFI_USEFILEATTRIBUTES) != 0;
    let display_name = display_name_for_win_path(win_path);

    let (win_attrs, host_path_opt, has_subfolder) = if use_attrs {
        (file_attrs, None, false)
    } else {
        let host = crate::path::to_host(win_path)?;
        let meta = std::fs::metadata(&host).ok()?;
        let fname = host
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| display_name.clone());
        let attrs = crate::kernel32::file::meta_attrs(&meta, &fname);
        let sub = if meta.is_dir() {
            std::fs::read_dir(&host)
                .ok()
                .map(|rd| {
                    rd.filter_map(|e| e.ok()).any(|e| {
                        !crate::path::is_host_private_keystore_path(&e.path())
                            && e.file_type().is_ok_and(|ft| ft.is_dir())
                    })
                })
                .unwrap_or(false)
        } else {
            false
        };
        (attrs, Some(host), sub)
    };

    let is_dir = (win_attrs & crate::kernel32::file::FILE_ATTRIBUTE_DIRECTORY) != 0;
    let is_ro = (win_attrs & crate::kernel32::file::FILE_ATTRIBUTE_READONLY) != 0;
    let is_hidden = (win_attrs & crate::kernel32::file::FILE_ATTRIBUTE_HIDDEN) != 0;

    let mut sfgao = SFGAO_FILESYSTEM
        | SFGAO_CANCOPY
        | SFGAO_CANMOVE
        | SFGAO_CANRENAME
        | SFGAO_CANDELETE
        | SFGAO_HASPROPSHEET;
    if is_dir {
        sfgao |= SFGAO_FOLDER | SFGAO_FILESYSANCESTOR;
        if has_subfolder {
            sfgao |= SFGAO_HASSUBFOLDER;
        }
    }
    if is_ro {
        sfgao |= SFGAO_READONLY;
    }
    if is_hidden {
        sfgao |= SFGAO_HIDDEN;
    }
    if (flags & SHGFI_ATTR_SPECIFIED) != 0 {
        sfgao &= requested_mask;
    }

    let type_name = describe_file_type(is_dir, &display_name);
    let icon_index = if is_dir { 1 } else { 0 };
    let ret_val = if (flags & SHGFI_EXETYPE) != 0 {
        host_path_opt.as_deref().map(detect_pe_exetype).unwrap_or(0)
    } else if (flags & SHGFI_SYSICONINDEX) != 0 {
        1
    } else {
        1
    };

    Some(FileInfoResult {
        sfgao,
        display_name,
        type_name,
        icon_index,
        ret_val,
    })
}

pub unsafe extern "win64" fn SHGetFileInfoA(
    path: LPCSTR,
    file_attrs: u32,
    sfi: *mut u8,
    cb_file_info: u32,
    flags: u32,
) -> usize {
    let requested_mask = if !sfi.is_null() && cb_file_info >= 16 {
        std::ptr::read_unaligned(sfi.add(12) as *const u32)
    } else {
        0
    };
    if !sfi.is_null() && cb_file_info > 0 {
        std::ptr::write_bytes(sfi, 0, cb_file_info as usize);
    }
    if path.is_null() {
        return 0;
    }
    let win_path = crate::strings::from_ansi(path);
    let Some(info) = query_file_info(&win_path, file_attrs, flags, requested_mask) else {
        return 0;
    };
    if !sfi.is_null() {
        if cb_file_info >= 12 && (flags & SHGFI_SYSICONINDEX) != 0 {
            std::ptr::write_unaligned(sfi.add(8) as *mut i32, info.icon_index);
        }
        if cb_file_info >= 16 && (flags & SHGFI_ATTRIBUTES) != 0 {
            std::ptr::write_unaligned(sfi.add(12) as *mut u32, info.sfgao);
        }
        if cb_file_info >= 276 && (flags & SHGFI_DISPLAYNAME) != 0 {
            crate::strings::write_ansi_trunc(&info.display_name, sfi.add(16), 260);
        }
        if cb_file_info >= 356 && (flags & SHGFI_TYPENAME) != 0 {
            crate::strings::write_ansi_trunc(&info.type_name, sfi.add(276), 80);
        }
    }
    info.ret_val
}

pub unsafe extern "win64" fn SHGetFileInfoW(
    path: LPCWSTR,
    file_attrs: u32,
    shfi: *mut u8,
    cb_file_info: u32,
    flags: u32,
) -> usize {
    let requested_mask = if !shfi.is_null() && cb_file_info >= 16 {
        std::ptr::read_unaligned(shfi.add(12) as *const u32)
    } else {
        0
    };
    if !shfi.is_null() && cb_file_info > 0 {
        std::ptr::write_bytes(shfi, 0, cb_file_info as usize);
    }
    if path.is_null() {
        return 0;
    }
    let win_path = from_wide(path);
    let Some(info) = query_file_info(&win_path, file_attrs, flags, requested_mask) else {
        return 0;
    };
    if !shfi.is_null() {
        if cb_file_info >= 12 && (flags & SHGFI_SYSICONINDEX) != 0 {
            std::ptr::write_unaligned(shfi.add(8) as *mut i32, info.icon_index);
        }
        if cb_file_info >= 16 && (flags & SHGFI_ATTRIBUTES) != 0 {
            std::ptr::write_unaligned(shfi.add(12) as *mut u32, info.sfgao);
        }
        if cb_file_info >= 536 && (flags & SHGFI_DISPLAYNAME) != 0 {
            write_wide_trunc(&info.display_name, shfi.add(16) as *mut u16, 260);
        }
        if cb_file_info >= 696 && (flags & SHGFI_TYPENAME) != 0 {
            write_wide_trunc(&info.type_name, shfi.add(536) as *mut u16, 80);
        }
    }
    info.ret_val
}

pub unsafe extern "win64" fn Shell_NotifyIconW(_msg: u32, _data: *mut u8) -> BOOL {
    FALSE
}

pub unsafe extern "win64" fn Shell_NotifyIconA(_msg: u32, _data: *mut u8) -> BOOL {
    FALSE
}

pub unsafe extern "win64" fn ExtractIconW(_inst: HINSTANCE, _exe: LPCWSTR, _idx: u32) -> usize {
    0
}

pub unsafe extern "win64" fn ExtractIconA(_inst: HINSTANCE, _exe: LPCSTR, _idx: u32) -> usize {
    0
}

pub unsafe extern "win64" fn ExtractIconExW(
    _file: LPCWSTR,
    _idx: i32,
    _large: *mut usize,
    _small: *mut usize,
    _n: u32,
) -> u32 {
    0
}

pub unsafe extern "win64" fn ExtractIconExA(
    _file: LPCSTR,
    _idx: i32,
    _large: *mut usize,
    _small: *mut usize,
    _n: u32,
) -> u32 {
    0
}

pub unsafe extern "win64" fn SHQueryUserNotificationState(state: *mut u32) -> HRESULT {
    if state.is_null() {
        return E_INVALIDARG;
    }
    // QUNS_ACCEPTS_NOTIFICATIONS = 5
    std::ptr::write_unaligned(state, 5);
    S_OK
}

pub unsafe extern "win64" fn SHCreateDirectoryExW(_hwnd: HWND, path: LPCWSTR, _sec: *mut c_void) -> i32 {
    if path.is_null() {
        return err::ERROR_INVALID_PARAMETER as i32;
    }
    let w = from_wide(path);
    match crate::path::to_host(&w) {
        Some(h) => match std::fs::create_dir_all(h) {
            Ok(()) => 0,
            Err(_) => err::ERROR_ACCESS_DENIED as i32,
        },
        None => err::ERROR_PATH_NOT_FOUND as i32,
    }
}

pub unsafe extern "win64" fn SHCreateDirectoryExA(_hwnd: HWND, path: LPCSTR, _sec: *mut c_void) -> i32 {
    if path.is_null() {
        return err::ERROR_INVALID_PARAMETER as i32;
    }
    let a = crate::strings::from_ansi(path);
    match crate::path::to_host(&a) {
        Some(h) => match std::fs::create_dir_all(h) {
            Ok(()) => 0,
            Err(_) => err::ERROR_ACCESS_DENIED as i32,
        },
        None => err::ERROR_PATH_NOT_FOUND as i32,
    }
}

pub unsafe extern "win64" fn SHChangeNotify(
    _event_id: i32,
    _flags: u32,
    _item1: *const c_void,
    _item2: *const c_void,
) {
}

pub unsafe extern "win64" fn SetCurrentProcessExplicitAppUserModelID(_id: LPCWSTR) -> HRESULT {
    S_OK
}

// ---------------------------------------------------------------------------
// `SHFileOperationA` / `SHFileOperationW`
// ---------------------------------------------------------------------------

const FO_MOVE: u32 = 1;
const FO_COPY: u32 = 2;
const FO_DELETE: u32 = 3;
const FO_RENAME: u32 = 4;

const FOF_MULTIDESTFILES: u16 = 0x0001;
const FOF_FILESONLY: u16 = 0x0080;
const FOF_NORECURSION: u16 = 0x1000;

#[repr(C)]
struct SHFILEOPSTRUCTW {
    hwnd: HWND,
    w_func: u32,
    p_from: LPCWSTR,
    p_to: LPCWSTR,
    f_flags: u16,
    f_any_operations_aborted: BOOL,
    h_name_mappings: *mut c_void,
    lpsz_progress_title: LPCWSTR,
}
const _: () = assert!(std::mem::size_of::<SHFILEOPSTRUCTW>() == 56);

#[repr(C)]
struct SHFILEOPSTRUCTA {
    hwnd: HWND,
    w_func: u32,
    p_from: LPCSTR,
    p_to: LPCSTR,
    f_flags: u16,
    f_any_operations_aborted: BOOL,
    h_name_mappings: *mut c_void,
    lpsz_progress_title: LPCSTR,
}
const _: () = assert!(std::mem::size_of::<SHFILEOPSTRUCTA>() == 56);

unsafe fn parse_zz_ansi(mut p: LPCSTR) -> Vec<String> {
    let mut out = Vec::new();
    if p.is_null() {
        return out;
    }
    loop {
        if std::ptr::read_unaligned(p) == 0 {
            break;
        }
        let s = crate::strings::from_ansi(p);
        let len = crate::strings::strlen(p);
        out.push(s);
        p = p.add(len + 1);
    }
    out
}

unsafe fn parse_zz_wide(mut p: LPCWSTR) -> Vec<String> {
    let mut out = Vec::new();
    if p.is_null() {
        return out;
    }
    loop {
        if std::ptr::read_unaligned(p) == 0 {
            break;
        }
        let mut units = Vec::new();
        loop {
            let u = std::ptr::read_unaligned(p);
            p = p.add(1);
            if u == 0 {
                break;
            }
            units.push(u);
        }
        out.push(String::from_utf16_lossy(&units));
    }
    out
}

fn io_err_win32(e: &std::io::Error) -> i32 {
    e.raw_os_error()
        .map(errno_to_win32)
        .unwrap_or(err::ERROR_ACCESS_DENIED) as i32
}

/// Resolve a Windows source spec (supporting `*`/`?` wildcards in the final
/// component) strictly inside `crate::path::to_host`.
fn expand_win_source(win_spec: &str, files_only: bool) -> Result<Vec<PathBuf>, i32> {
    let norm = win_spec.replace('/', "\\");
    let (dir_part, base_part) = match norm.rsplit_once('\\') {
        Some((d, b)) => (d, b),
        None => ("", norm.as_str()),
    };
    if base_part.contains('*') || base_part.contains('?') {
        let dir_win = if dir_part.is_empty() {
            crate::path::cwd()
        } else if dir_part.len() == 2 && dir_part.as_bytes()[1] == b':' {
            format!("{dir_part}\\")
        } else {
            dir_part.to_string()
        };
        let host_dir = crate::path::to_host(&dir_win).ok_or(err::ERROR_PATH_NOT_FOUND as i32)?;
        let rd = std::fs::read_dir(&host_dir).map_err(|e| io_err_win32(&e))?;
        let mut matches = Vec::new();
        for entry in rd.filter_map(|e| e.ok()) {
            let ep = entry.path();
            if crate::path::is_host_private_keystore_path(&ep) {
                continue;
            }
            let name = entry.file_name().to_string_lossy().into_owned();
            if crate::kernel32::file::wildcard_match(base_part, &name) {
                if files_only && entry.file_type().is_ok_and(|ft| ft.is_dir()) {
                    continue;
                }
                matches.push(ep);
            }
        }
        if matches.is_empty() {
            return Err(err::ERROR_FILE_NOT_FOUND as i32);
        }
        Ok(matches)
    } else {
        let host = crate::path::to_host(win_spec).ok_or(err::ERROR_PATH_NOT_FOUND as i32)?;
        if crate::path::is_host_private_keystore_path(&host) {
            return Err(err::ERROR_ACCESS_DENIED as i32);
        }
        if std::fs::symlink_metadata(&host).is_err() {
            return Err(err::ERROR_FILE_NOT_FOUND as i32);
        }
        Ok(vec![host])
    }
}

fn copy_entry(src: &Path, dst: &Path, no_recursion: bool, files_only: bool) -> Result<(), i32> {
    if crate::path::is_host_private_keystore_path(src) || crate::path::is_protected_host_write(dst) {
        return Err(err::ERROR_ACCESS_DENIED as i32);
    }
    let meta = std::fs::symlink_metadata(src).map_err(|e| io_err_win32(&e))?;
    if meta.file_type().is_symlink() {
        return Err(err::ERROR_ACCESS_DENIED as i32);
    }
    if meta.is_dir() {
        if files_only {
            return Ok(());
        }
        if !no_recursion && crate::path::is_host_private_keystore_ancestor(src) {
            return Err(err::ERROR_ACCESS_DENIED as i32);
        }
        std::fs::create_dir_all(dst).map_err(|e| io_err_win32(&e))?;
        if !no_recursion {
            for entry in std::fs::read_dir(src).map_err(|e| io_err_win32(&e))? {
                let entry = entry.map_err(|e| io_err_win32(&e))?;
                let sp = entry.path();
                if crate::path::is_host_private_keystore_path(&sp) {
                    return Err(err::ERROR_ACCESS_DENIED as i32);
                }
                let dp = dst.join(entry.file_name());
                copy_entry(&sp, &dp, false, files_only)?;
            }
        }
        Ok(())
    } else {
        if let Some(parent) = dst.parent() {
            std::fs::create_dir_all(parent).map_err(|e| io_err_win32(&e))?;
        }
        std::fs::copy(src, dst).map_err(|e| io_err_win32(&e))?;
        Ok(())
    }
}

fn delete_entry(src: &Path, no_recursion: bool, files_only: bool) -> Result<(), i32> {
    if crate::path::is_protected_host_remove_or_rename(src) {
        return Err(err::ERROR_ACCESS_DENIED as i32);
    }
    let meta = std::fs::symlink_metadata(src).map_err(|e| io_err_win32(&e))?;
    if meta.is_dir() {
        if files_only {
            return Ok(());
        }
        if no_recursion {
            std::fs::remove_dir(src).map_err(|e| io_err_win32(&e))?;
        } else {
            std::fs::remove_dir_all(src).map_err(|e| io_err_win32(&e))?;
        }
    } else {
        std::fs::remove_file(src).map_err(|e| io_err_win32(&e))?;
    }
    Ok(())
}

fn move_entry(src: &Path, dst: &Path, no_recursion: bool, files_only: bool) -> Result<(), i32> {
    if crate::path::is_protected_host_remove_or_rename(src)
        || crate::path::is_protected_host_remove_or_rename(dst)
    {
        return Err(err::ERROR_ACCESS_DENIED as i32);
    }
    let meta = std::fs::symlink_metadata(src).map_err(|e| io_err_win32(&e))?;
    if meta.file_type().is_symlink() {
        return Err(err::ERROR_ACCESS_DENIED as i32);
    }
    if meta.is_dir() && files_only {
        return Ok(());
    }
    if let Some(parent) = dst.parent() {
        std::fs::create_dir_all(parent).map_err(|e| io_err_win32(&e))?;
    }
    match std::fs::rename(src, dst) {
        Ok(()) => Ok(()),
        Err(_) => {
            copy_entry(src, dst, no_recursion, files_only)?;
            delete_entry(src, no_recursion, files_only)?;
            Ok(())
        }
    }
}

fn execute_file_op(w_func: u32, from_list: &[String], to_list: &[String], flags: u16) -> i32 {
    if from_list.is_empty() {
        return err::ERROR_INVALID_PARAMETER as i32;
    }
    let multi_dest = (flags & FOF_MULTIDESTFILES) != 0;
    let files_only = (flags & FOF_FILESONLY) != 0;
    let no_recursion = (flags & FOF_NORECURSION) != 0;

    match w_func {
        FO_DELETE => {
            for spec in from_list {
                let sources = match expand_win_source(spec, files_only) {
                    Ok(v) => v,
                    Err(e) => return e,
                };
                for src in sources {
                    if let Err(e) = delete_entry(&src, no_recursion, files_only) {
                        return e;
                    }
                }
            }
            0
        }
        FO_RENAME => {
            if from_list.len() != 1 || to_list.len() != 1 {
                return err::ERROR_INVALID_PARAMETER as i32;
            }
            let Some(src) = crate::path::to_host(&from_list[0]) else {
                return err::ERROR_PATH_NOT_FOUND as i32;
            };
            let Some(dst) = crate::path::to_host(&to_list[0]) else {
                return err::ERROR_PATH_NOT_FOUND as i32;
            };
            if crate::path::is_protected_host_remove_or_rename(&src)
                || crate::path::is_protected_host_remove_or_rename(&dst)
            {
                return err::ERROR_ACCESS_DENIED as i32;
            }
            if std::fs::symlink_metadata(&src).is_err() {
                return err::ERROR_FILE_NOT_FOUND as i32;
            }
            if let Some(parent) = dst.parent() {
                if let Err(e) = std::fs::create_dir_all(parent) {
                    return io_err_win32(&e);
                }
            }
            match std::fs::rename(&src, &dst) {
                Ok(()) => 0,
                Err(e) => io_err_win32(&e),
            }
        }
        FO_COPY | FO_MOVE => {
            if to_list.is_empty() {
                return err::ERROR_INVALID_PARAMETER as i32;
            }
            let mut expanded_sources = Vec::new();
            for spec in from_list {
                match expand_win_source(spec, files_only) {
                    Ok(v) => expanded_sources.extend(v),
                    Err(e) => return e,
                }
            }
            if multi_dest && to_list.len() == expanded_sources.len() {
                for (src, to_win) in expanded_sources.iter().zip(to_list.iter()) {
                    let Some(dst) = crate::path::to_host(to_win) else {
                        return err::ERROR_PATH_NOT_FOUND as i32;
                    };
                    let res = if w_func == FO_COPY {
                        copy_entry(src, &dst, no_recursion, files_only)
                    } else {
                        move_entry(src, &dst, no_recursion, files_only)
                    };
                    if let Err(e) = res {
                        return e;
                    }
                }
                0
            } else {
                let Some(base_dst) = crate::path::to_host(&to_list[0]) else {
                    return err::ERROR_PATH_NOT_FOUND as i32;
                };
                let dst_is_dir = base_dst.is_dir() || expanded_sources.len() > 1;
                if dst_is_dir {
                    if let Err(e) = std::fs::create_dir_all(&base_dst) {
                        return io_err_win32(&e);
                    }
                }
                for src in &expanded_sources {
                    let target = if dst_is_dir {
                        let Some(name) = src.file_name() else {
                            return err::ERROR_INVALID_PARAMETER as i32;
                        };
                        base_dst.join(name)
                    } else {
                        base_dst.clone()
                    };
                    let res = if w_func == FO_COPY {
                        copy_entry(src, &target, no_recursion, files_only)
                    } else {
                        move_entry(src, &target, no_recursion, files_only)
                    };
                    if let Err(e) = res {
                        return e;
                    }
                }
                0
            }
        }
        _ => err::ERROR_INVALID_PARAMETER as i32,
    }
}

pub unsafe extern "win64" fn SHFileOperationW(op: *mut u8) -> i32 {
    if op.is_null() {
        return err::ERROR_INVALID_PARAMETER as i32;
    }
    let s = std::ptr::read_unaligned(op as *const SHFILEOPSTRUCTW);
    std::ptr::write_unaligned(op.add(36) as *mut BOOL, FALSE);
    let from_list = parse_zz_wide(s.p_from);
    let to_list = parse_zz_wide(s.p_to);
    execute_file_op(s.w_func, &from_list, &to_list, s.f_flags)
}

pub unsafe extern "win64" fn SHFileOperationA(op: *mut u8) -> i32 {
    if op.is_null() {
        return err::ERROR_INVALID_PARAMETER as i32;
    }
    let s = std::ptr::read_unaligned(op as *const SHFILEOPSTRUCTA);
    std::ptr::write_unaligned(op.add(36) as *mut BOOL, FALSE);
    let from_list = parse_zz_ansi(s.p_from);
    let to_list = parse_zz_ansi(s.p_to);
    execute_file_op(s.w_func, &from_list, &to_list, s.f_flags)
}

pub unsafe extern "win64" fn SHGetSpecialFolderPathW(
    hwnd: HWND,
    path: *mut u16,
    csidl: i32,
    create: BOOL,
) -> BOOL {
    let flags = if create != 0 { 0x8000 } else { 0 };
    (SHGetFolderPathW(hwnd, csidl | flags, std::ptr::null_mut(), 0, path) == S_OK) as BOOL
}

pub unsafe extern "win64" fn SHGetPropertyStoreFromParsingName(
    _path: LPCWSTR,
    _bind_ctx: *mut c_void,
    _flags: u32,
    _riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if !ppv.is_null() {
        std::ptr::write_unaligned(ppv, std::ptr::null_mut());
    }
    E_NOINTERFACE
}

pub unsafe extern "win64" fn IsUserAnAdmin() -> BOOL {
    TRUE
}

pub unsafe extern "win64" fn PathStripPathW(path: LPWSTR) {
    if path.is_null() {
        return;
    }
    let s = from_wide(path);
    let base = s.rsplit(['\\', '/']).next().unwrap_or(&s);
    write_wide_trunc(base, path, s.encode_utf16().count() + 1);
}

pub unsafe extern "win64" fn PathRemoveFileSpecW(path: LPWSTR) -> BOOL {
    if path.is_null() {
        return FALSE;
    }
    let s = from_wide(path);
    if let Some(pos) = s.rfind(['\\', '/']) {
        let dir = &s[..pos];
        write_wide_trunc(dir, path, s.encode_utf16().count() + 1);
        TRUE
    } else {
        FALSE
    }
}

pub unsafe extern "win64" fn PathStripPathA(path: LPSTR) {
    if path.is_null() {
        return;
    }
    let s = crate::strings::from_ansi(path);
    let base = s.rsplit(['\\', '/']).next().unwrap_or(&s);
    crate::strings::write_ansi_trunc(base, path, s.len() + 1);
}

pub unsafe extern "win64" fn PathCombineA(dest: LPSTR, dir: LPCSTR, file: LPCSTR) -> LPSTR {
    if dest.is_null() {
        return std::ptr::null_mut();
    }
    let d = crate::strings::from_ansi_opt(dir).unwrap_or_default();
    let f = crate::strings::from_ansi_opt(file).unwrap_or_default();
    let combined = if d.is_empty() {
        f
    } else if f.is_empty() {
        d
    } else {
        format!(
            "{}\\{}",
            d.trim_end_matches(['\\', '/']),
            f.trim_start_matches(['\\', '/'])
        )
    };
    crate::strings::write_ansi_trunc(&combined, dest, 260);
    dest
}

pub unsafe extern "win64" fn PathCombineW(dest: LPWSTR, dir: LPCWSTR, file: LPCWSTR) -> LPWSTR {
    if dest.is_null() {
        return std::ptr::null_mut();
    }
    let d = crate::strings::from_wide_opt(dir).unwrap_or_default();
    let f = crate::strings::from_wide_opt(file).unwrap_or_default();
    let combined = if d.is_empty() {
        f
    } else if f.is_empty() {
        d
    } else {
        format!(
            "{}\\{}",
            d.trim_end_matches(['\\', '/']),
            f.trim_start_matches(['\\', '/'])
        )
    };
    write_wide_trunc(&combined, dest, 260);
    dest
}

pub unsafe extern "win64" fn PathFindFileNameW(path: LPCWSTR) -> LPWSTR {
    if path.is_null() {
        return std::ptr::null_mut();
    }
    let mut last = path;
    let mut p = path;
    loop {
        let ch = std::ptr::read_unaligned(p);
        if ch == 0 {
            break;
        }
        if (ch == b'\\' as u16 || ch == b'/' as u16 || ch == b':' as u16)
            && std::ptr::read_unaligned(p.add(1)) != 0
        {
            last = p.add(1);
        }
        p = p.add(1);
    }
    last as LPWSTR
}

pub unsafe extern "win64" fn PathFindFileNameA(path: LPCSTR) -> LPSTR {
    if path.is_null() {
        return std::ptr::null_mut();
    }
    let mut last = path;
    let mut p = path;
    loop {
        let ch = std::ptr::read_unaligned(p);
        if ch == 0 {
            break;
        }
        if (ch == b'\\' || ch == b'/' || ch == b':') && std::ptr::read_unaligned(p.add(1)) != 0 {
            last = p.add(1);
        }
        p = p.add(1);
    }
    last as LPSTR
}

pub unsafe extern "win64" fn PathFindExtensionW(path: LPCWSTR) -> LPWSTR {
    if path.is_null() {
        return std::ptr::null_mut();
    }
    let mut dot: LPCWSTR = std::ptr::null();
    let mut p = path;
    loop {
        let ch = std::ptr::read_unaligned(p);
        if ch == 0 {
            break;
        }
        if ch == b'.' as u16 {
            dot = p;
        } else if ch == b'\\' as u16 || ch == b'/' as u16 || ch == b' ' as u16 {
            dot = std::ptr::null();
        }
        p = p.add(1);
    }
    (if dot.is_null() { p } else { dot }) as LPWSTR
}

pub unsafe extern "win64" fn PathFindExtensionA(path: LPCSTR) -> LPSTR {
    if path.is_null() {
        return std::ptr::null_mut();
    }
    let mut dot: LPCSTR = std::ptr::null();
    let mut p = path;
    loop {
        let ch = std::ptr::read_unaligned(p);
        if ch == 0 {
            break;
        }
        if ch == b'.' {
            dot = p;
        } else if ch == b'\\' || ch == b'/' || ch == b' ' {
            dot = std::ptr::null();
        }
        p = p.add(1);
    }
    (if dot.is_null() { p } else { dot }) as LPSTR
}

pub unsafe extern "win64" fn PathRemoveExtensionW(path: LPWSTR) {
    if path.is_null() {
        return;
    }
    let ext = PathFindExtensionW(path);
    if !ext.is_null() {
        std::ptr::write_unaligned(ext, 0);
    }
}

pub unsafe extern "win64" fn PathRemoveExtensionA(path: LPSTR) {
    if path.is_null() {
        return;
    }
    let ext = PathFindExtensionA(path);
    if !ext.is_null() {
        std::ptr::write_unaligned(ext, 0);
    }
}

pub unsafe extern "win64" fn PathRemoveFileSpecA(path: LPSTR) -> BOOL {
    if path.is_null() {
        return FALSE;
    }
    let s = crate::strings::from_ansi(path);
    if let Some(pos) = s.rfind(['\\', '/']) {
        crate::strings::write_ansi_trunc(&s[..pos], path, s.len() + 1);
        TRUE
    } else {
        FALSE
    }
}

pub unsafe extern "win64" fn PathAppendW(path: LPWSTR, more: LPCWSTR) -> BOOL {
    if path.is_null() || more.is_null() {
        return FALSE;
    }
    let cur = from_wide(path);
    let add = from_wide(more);
    let combined = format!(
        "{}\\{}",
        cur.trim_end_matches(['\\', '/']),
        add.trim_start_matches(['\\', '/'])
    );
    write_wide_trunc(&combined, path, 260);
    TRUE
}

pub unsafe extern "win64" fn PathAppendA(path: LPSTR, more: LPCSTR) -> BOOL {
    if path.is_null() || more.is_null() {
        return FALSE;
    }
    let cur = crate::strings::from_ansi(path);
    let add = crate::strings::from_ansi(more);
    let combined = format!(
        "{}\\{}",
        cur.trim_end_matches(['\\', '/']),
        add.trim_start_matches(['\\', '/'])
    );
    crate::strings::write_ansi_trunc(&combined, path, 260);
    TRUE
}

pub unsafe extern "win64" fn PathFileExistsW(path: LPCWSTR) -> BOOL {
    if path.is_null() {
        return FALSE;
    }
    let a = crate::kernel32::file::GetFileAttributesW(path);
    (a != u32::MAX) as BOOL
}

pub unsafe extern "win64" fn PathFileExistsA(path: LPCSTR) -> BOOL {
    if path.is_null() {
        return FALSE;
    }
    let a = crate::kernel32::file::GetFileAttributesA(path);
    (a != u32::MAX) as BOOL
}

pub unsafe extern "win64" fn PathIsDirectoryW(path: LPCWSTR) -> BOOL {
    if path.is_null() {
        return FALSE;
    }
    let a = crate::kernel32::file::GetFileAttributesW(path);
    (a != u32::MAX && (a & 0x10) != 0) as BOOL
}

pub unsafe extern "win64" fn PathIsDirectoryA(path: LPCSTR) -> BOOL {
    if path.is_null() {
        return FALSE;
    }
    let a = crate::kernel32::file::GetFileAttributesA(path);
    (a != u32::MAX && (a & 0x10) != 0) as BOOL
}

pub unsafe extern "win64" fn PathIsRelativeW(path: LPCWSTR) -> BOOL {
    if path.is_null() {
        return TRUE;
    }
    let s = from_wide(path);
    let b = s.as_bytes();
    let abs = b.starts_with(b"\\") || b.starts_with(b"/") || (b.len() >= 2 && b[1] == b':');
    (!abs) as BOOL
}

pub unsafe extern "win64" fn PathIsRelativeA(path: LPCSTR) -> BOOL {
    if path.is_null() {
        return TRUE;
    }
    let s = crate::strings::from_ansi(path);
    let b = s.as_bytes();
    let abs = b.starts_with(b"\\") || b.starts_with(b"/") || (b.len() >= 2 && b[1] == b':');
    (!abs) as BOOL
}

pub unsafe extern "win64" fn PathAddBackslashW(path: LPWSTR) -> LPWSTR {
    if path.is_null() {
        return std::ptr::null_mut();
    }
    let mut p = path;
    while std::ptr::read_unaligned(p) != 0 {
        p = p.add(1);
    }
    if p > path && std::ptr::read_unaligned(p.sub(1)) != b'\\' as u16 {
        std::ptr::write_unaligned(p, b'\\' as u16);
        std::ptr::write_unaligned(p.add(1), 0);
        p = p.add(1);
    }
    p
}

pub unsafe extern "win64" fn PathAddBackslashA(path: LPSTR) -> LPSTR {
    if path.is_null() {
        return std::ptr::null_mut();
    }
    let mut p = path;
    while std::ptr::read_unaligned(p) != 0 {
        p = p.add(1);
    }
    if p > path && std::ptr::read_unaligned(p.sub(1)) != b'\\' {
        std::ptr::write_unaligned(p, b'\\');
        std::ptr::write_unaligned(p.add(1), 0);
        p = p.add(1);
    }
    p
}

pub unsafe extern "win64" fn PathRemoveBackslashW(path: LPWSTR) -> LPWSTR {
    if path.is_null() {
        return std::ptr::null_mut();
    }
    let mut p = path;
    while std::ptr::read_unaligned(p) != 0 {
        p = p.add(1);
    }
    if p > path && std::ptr::read_unaligned(p.sub(1)) == b'\\' as u16 {
        p = p.sub(1);
        std::ptr::write_unaligned(p, 0);
    }
    p
}

pub unsafe extern "win64" fn PathRemoveBackslashA(path: LPSTR) -> LPSTR {
    if path.is_null() {
        return std::ptr::null_mut();
    }
    let mut p = path;
    while std::ptr::read_unaligned(p) != 0 {
        p = p.add(1);
    }
    if p > path && std::ptr::read_unaligned(p.sub(1)) == b'\\' {
        p = p.sub(1);
        std::ptr::write_unaligned(p, 0);
    }
    p
}

pub unsafe extern "win64" fn PathMatchSpecW(file: LPCWSTR, spec: LPCWSTR) -> BOOL {
    if file.is_null() || spec.is_null() {
        return FALSE;
    }
    let f = from_wide(file);
    let sp = from_wide(spec);
    sp.split(';')
        .any(|pat| crate::kernel32::file::wildcard_match(pat.trim(), &f)) as BOOL
}

pub unsafe extern "win64" fn PathMatchSpecA(file: LPCSTR, spec: LPCSTR) -> BOOL {
    if file.is_null() || spec.is_null() {
        return FALSE;
    }
    let f = crate::strings::from_ansi(file);
    let sp = crate::strings::from_ansi(spec);
    sp.split(';')
        .any(|pat| crate::kernel32::file::wildcard_match(pat.trim(), &f)) as BOOL
}

pub unsafe extern "win64" fn StrStrIW(first: LPCWSTR, srch: LPCWSTR) -> LPWSTR {
    if first.is_null() || srch.is_null() {
        return std::ptr::null_mut();
    }
    let hay = from_wide(first);
    let ndl = from_wide(srch);
    if ndl.is_empty() {
        return first as LPWSTR;
    }
    let h_lo = hay.to_ascii_lowercase();
    let n_lo = ndl.to_ascii_lowercase();
    if let Some(byte_idx) = h_lo.find(&n_lo) {
        let utf16_idx = hay[..byte_idx].encode_utf16().count();
        first.add(utf16_idx) as LPWSTR
    } else {
        std::ptr::null_mut()
    }
}

pub unsafe extern "win64" fn StrStrIA(first: LPCSTR, srch: LPCSTR) -> LPSTR {
    if first.is_null() || srch.is_null() {
        return std::ptr::null_mut();
    }
    let hay = crate::strings::from_ansi(first);
    let ndl = crate::strings::from_ansi(srch);
    if ndl.is_empty() {
        return first as LPSTR;
    }
    let h_lo = hay.to_ascii_lowercase();
    let n_lo = ndl.to_ascii_lowercase();
    if let Some(idx) = h_lo.find(&n_lo) {
        first.add(idx) as LPSTR
    } else {
        std::ptr::null_mut()
    }
}

// ---------------------------------------------------------------------------
// `SHCreateItemFromParsingName` & `IShellItem` (filesystem items)
// ---------------------------------------------------------------------------

const SIGDN_NORMALDISPLAY: u32 = 0x0000_0000;
const SIGDN_PARENTRELATIVEPARSING: u32 = 0x8001_8001;
const SIGDN_DESKTOPABSOLUTEPARSING: u32 = 0x8002_8000;
const SIGDN_PARENTRELATIVEEDITING: u32 = 0x8003_1001;
const SIGDN_DESKTOPABSOLUTEEDITING: u32 = 0x8004_C000;
const SIGDN_FILESYSPATH: u32 = 0x8005_8000;
const SIGDN_URL: u32 = 0x8006_8000;
const SIGDN_PARENTRELATIVEFORADDRESSBAR: u32 = 0x8007_C001;
const SIGDN_PARENTRELATIVE: u32 = 0x8008_0001;
const SIGDN_PARENTRELATIVEFORUI: u32 = 0x8009_4001;

#[inline]
const fn hresult_from_win32(code: u32) -> HRESULT {
    if code == 0 {
        S_OK
    } else {
        ((code & 0x0000_FFFF) | 0x8007_0000) as HRESULT
    }
}

/// Reject virtual shell namespaces, CLSID parsing paths, URI schemes, and UNC network shares
/// before falling through to the local Windows VFS.
fn is_unsupported_shell_namespace(raw: &str) -> bool {
    let s = raw.trim_start();
    if s.starts_with("::{") || s.starts_with(":::{") {
        return true;
    }
    if s.starts_with("\\\\") || s.starts_with("//") {
        // Allow extended-length local/device prefixes handled by `crate::path::normalize`.
        let up = s.replace('/', "\\").to_ascii_uppercase();
        if !(up.starts_with("\\\\?\\") || up.starts_with("\\\\.\\")) {
            return true;
        }
    }
    if s.contains("://") {
        return true;
    }
    if let Some(colon) = s.find(':') {
        // A single ASCII letter before ':' is a Windows drive spec (e.g. `C:\...`);
        // multi-character prefixes before ':' (e.g. `shell:`, `ms-settings:`, `knownfolder:`)
        // are namespace/URI schemes we do not implement.
        if colon >= 2 {
            let scheme = &s[..colon];
            if !scheme.contains('\\')
                && !scheme.contains('/')
                && scheme
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'+' | b'-' | b'.'))
            {
                return true;
            }
        }
    }
    false
}

fn shell_item_leaf_name(win_path: &str) -> String {
    let trimmed = win_path.trim_end_matches(['\\', '/']);
    if trimmed.len() == 2 && trimmed.as_bytes()[1] == b':' {
        return format!("{}\\", trimmed.to_ascii_uppercase());
    }
    trimmed
        .rsplit(['\\', '/'])
        .next()
        .filter(|s| !s.is_empty())
        .unwrap_or(win_path)
        .to_string()
}

fn shell_item_parent_win_path(win_path: &str) -> Option<String> {
    let trimmed = win_path.trim_end_matches('\\');
    // Drive root (`C:` / `C:\`) has no filesystem parent (its parent is virtual "This PC").
    if trimmed.len() <= 2 && trimmed.as_bytes().get(1) == Some(&b':') {
        return None;
    }
    let pos = trimmed.rfind('\\')?;
    let parent = &trimmed[..pos];
    if parent.len() == 2 && parent.as_bytes()[1] == b':' {
        Some(format!("{parent}\\"))
    } else if parent.is_empty() {
        None
    } else {
        Some(parent.to_string())
    }
}

fn format_file_url(win_path: &str) -> String {
    let mut out = String::from("file:///");
    for ch in win_path.chars() {
        match ch {
            '\\' => out.push('/'),
            ' ' => out.push_str("%20"),
            '%' => out.push_str("%25"),
            '#' => out.push_str("%23"),
            '?' => out.push_str("%3F"),
            c => out.push(c),
        }
    }
    out
}

unsafe fn alloc_cotask_wide(s: &str) -> *mut u16 {
    let wide = crate::strings::to_wide_z(s);
    let bytes = wide.len().saturating_mul(2);
    let p = crate::misc::ole32::CoTaskMemAlloc(bytes) as *mut u16;
    if p.is_null() {
        return std::ptr::null_mut();
    }
    std::ptr::copy_nonoverlapping(wide.as_ptr(), p, wide.len());
    p
}

struct ShellItem {
    win_path: String,
}

unsafe extern "win64" fn shell_item_qi(
    this: *mut crate::com::ComObj<ShellItem>,
    riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    crate::com::qi_helper_bytes(this, riid, ppv, &[crate::com::IID_IShellItem])
}

unsafe extern "win64" fn shell_item_bind_to_handler(
    _this: *mut crate::com::ComObj<ShellItem>,
    _pbc: *mut c_void,
    _bhid: *const u8,
    _riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    crate::com::write_null_out(ppv);
    E_NOINTERFACE
}

unsafe extern "win64" fn shell_item_get_parent(
    this: *mut crate::com::ComObj<ShellItem>,
    ppsi: *mut *mut c_void,
) -> HRESULT {
    if ppsi.is_null() {
        return E_POINTER;
    }
    crate::com::write_null_out(ppsi);
    if this.is_null() {
        return E_INVALIDARG;
    }
    let Some(parent_win) = shell_item_parent_win_path(&(*this).inner.win_path) else {
        return E_NOTIMPL;
    };
    let Some(parent_host) = crate::path::to_host(&parent_win) else {
        return hresult_from_win32(err::ERROR_PATH_NOT_FOUND);
    };
    if std::fs::metadata(&parent_host).is_err() {
        return hresult_from_win32(err::ERROR_PATH_NOT_FOUND);
    }
    let parent = crate::com::ComObj::new(shell_item_vtbl(), ShellItem { win_path: parent_win });
    std::ptr::write_unaligned(ppsi, parent as *mut c_void);
    S_OK
}

unsafe extern "win64" fn shell_item_get_display_name(
    this: *mut crate::com::ComObj<ShellItem>,
    sigdn_name: u32,
    ppsz_name: *mut LPWSTR,
) -> HRESULT {
    if ppsz_name.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(ppsz_name, std::ptr::null_mut());
    if this.is_null() {
        return E_INVALIDARG;
    }
    let win_path = &(*this).inner.win_path;
    let text = match sigdn_name {
        SIGDN_FILESYSPATH | SIGDN_DESKTOPABSOLUTEPARSING | SIGDN_DESKTOPABSOLUTEEDITING => {
            win_path.clone()
        }
        SIGDN_NORMALDISPLAY
        | SIGDN_PARENTRELATIVEPARSING
        | SIGDN_PARENTRELATIVEEDITING
        | SIGDN_PARENTRELATIVEFORADDRESSBAR
        | SIGDN_PARENTRELATIVE
        | SIGDN_PARENTRELATIVEFORUI => shell_item_leaf_name(win_path),
        SIGDN_URL => format_file_url(win_path),
        _ => return E_INVALIDARG,
    };
    let ptr = alloc_cotask_wide(&text);
    if ptr.is_null() {
        return E_OUTOFMEMORY;
    }
    std::ptr::write_unaligned(ppsz_name, ptr);
    S_OK
}

unsafe extern "win64" fn shell_item_get_attributes(
    this: *mut crate::com::ComObj<ShellItem>,
    sfgao_mask: u32,
    psfgao_attribs: *mut u32,
) -> HRESULT {
    if psfgao_attribs.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(psfgao_attribs, 0);
    if this.is_null() {
        return E_INVALIDARG;
    }
    let Some(info) = query_file_info(&(*this).inner.win_path, 0, SHGFI_ATTRIBUTES, 0) else {
        return hresult_from_win32(err::ERROR_FILE_NOT_FOUND);
    };
    let matched = info.sfgao & sfgao_mask;
    std::ptr::write_unaligned(psfgao_attribs, matched);
    if matched == sfgao_mask { S_OK } else { S_FALSE }
}

unsafe extern "win64" fn shell_item_compare(
    this: *mut crate::com::ComObj<ShellItem>,
    psi: *mut c_void,
    _hint: u32,
    pi_order: *mut i32,
) -> HRESULT {
    if pi_order.is_null() {
        return E_POINTER;
    }
    std::ptr::write_unaligned(pi_order, 0);
    if this.is_null() || psi.is_null() {
        return E_INVALIDARG;
    }
    if psi == this as *mut c_void {
        return S_OK;
    }
    let other_path = if let Some(other) =
        crate::com::com_from_iface::<ShellItem>(psi, shell_item_vtbl())
    {
        other.inner.win_path.clone()
    } else {
        let mut name_ptr: LPWSTR = std::ptr::null_mut();
        let hr = crate::com::com_call_slot(
            psi,
            5,
            SIGDN_FILESYSPATH as u64,
            &mut name_ptr as *mut LPWSTR as u64,
            0,
        );
        if hr < 0 || name_ptr.is_null() {
            return if hr < 0 { hr } else { E_FAIL };
        }
        let s = crate::path::normalize(&from_wide(name_ptr));
        crate::misc::ole32::CoTaskMemFree(name_ptr as *mut c_void);
        s
    };
    let a = (*this).inner.win_path.to_ascii_lowercase();
    let b = other_path.to_ascii_lowercase();
    match a.cmp(&b) {
        std::cmp::Ordering::Equal => {
            std::ptr::write_unaligned(pi_order, 0);
            S_OK
        }
        std::cmp::Ordering::Less => {
            std::ptr::write_unaligned(pi_order, -1);
            S_FALSE
        }
        std::cmp::Ordering::Greater => {
            std::ptr::write_unaligned(pi_order, 1);
            S_FALSE
        }
    }
}

fn shell_item_vtbl() -> *const usize {
    static VTBL: std::sync::OnceLock<[usize; 8]> = std::sync::OnceLock::new();
    let vtbl = VTBL.get_or_init(|| {
        [
            shell_item_qi as *const () as usize,
            crate::com::com_add_ref::<ShellItem> as *const () as usize,
            crate::com::com_release::<ShellItem> as *const () as usize,
            shell_item_bind_to_handler as *const () as usize,
            shell_item_get_parent as *const () as usize,
            shell_item_get_display_name as *const () as usize,
            shell_item_get_attributes as *const () as usize,
            shell_item_compare as *const () as usize,
        ]
    });
    static REG: std::sync::Once = std::sync::Once::new();
    REG.call_once(|| unsafe {
        crate::com::register_host_vtable(crate::com::IID_IShellItem, vtbl.as_ptr(), vtbl.len());
    });
    vtbl.as_ptr()
}

pub unsafe extern "win64" fn SHCreateItemFromParsingName(
    psz_path: LPCWSTR,
    _pbc: *mut c_void,
    riid: *const u8,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    crate::com::write_null_out(ppv);
    if psz_path.is_null() {
        return E_INVALIDARG;
    }
    let Some(req_iid) = crate::com::read_guid_from_bytes(riid) else {
        return E_POINTER;
    };
    let raw = from_wide(psz_path);
    if raw.is_empty() {
        return E_INVALIDARG;
    }
    if is_unsupported_shell_namespace(&raw) {
        stub!("SHCreateItemFromParsingName: unsupported shell namespace {raw:?}");
        return E_NOTIMPL;
    }
    let win_path = crate::path::absolute(&raw);
    let Some(host_path) = crate::path::to_host(&win_path) else {
        return hresult_from_win32(err::ERROR_PATH_NOT_FOUND);
    };
    if let Err(e) = std::fs::metadata(&host_path) {
        return if e.kind() == std::io::ErrorKind::NotFound {
            hresult_from_win32(err::ERROR_FILE_NOT_FOUND)
        } else {
            hresult_from_win32(io_err_win32(&e) as u32)
        };
    }
    if req_iid != crate::com::IID_IUnknown && req_iid != crate::com::IID_IShellItem {
        return E_NOINTERFACE;
    }
    let item = crate::com::ComObj::new(shell_item_vtbl(), ShellItem { win_path });
    std::ptr::write_unaligned(ppv, item as *mut c_void);
    S_OK
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_zz_w(items: &[&str]) -> Vec<u16> {
        let mut out = Vec::new();
        for s in items {
            out.extend(s.encode_utf16());
            out.push(0);
        }
        out.push(0);
        out
    }

    fn to_zz_a(items: &[&str]) -> Vec<u8> {
        let mut out = Vec::new();
        for s in items {
            out.extend_from_slice(s.as_bytes());
            out.push(0);
        }
        out.push(0);
        out
    }

    #[test]
    fn sh_file_operation_copy_move_rename_delete() {
        let root = std::env::temp_dir().join(format!("win32_shell32_test_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("src_dir")).unwrap();
        std::fs::write(root.join("src_dir/hello.txt"), b"shell32 payload").unwrap();
        crate::path::add_mount("S:\\ShellTest", &root);

        unsafe {
            // 1) FO_COPY via SHFileOperationW (recursive directory copy)
            let from_w = to_zz_w(&["S:\\ShellTest\\src_dir"]);
            let to_w = to_zz_w(&["S:\\ShellTest\\copied_dir"]);
            let mut op_w = SHFILEOPSTRUCTW {
                hwnd: std::ptr::null_mut(),
                w_func: FO_COPY,
                p_from: from_w.as_ptr(),
                p_to: to_w.as_ptr(),
                f_flags: 0x0014, // FOF_SILENT | FOF_NOCONFIRMATION
                f_any_operations_aborted: 1,
                h_name_mappings: std::ptr::null_mut(),
                lpsz_progress_title: std::ptr::null(),
            };
            assert_eq!(SHFileOperationW(&mut op_w as *mut _ as *mut u8), 0);
            assert_eq!(op_w.f_any_operations_aborted, FALSE);
            assert_eq!(
                std::fs::read(root.join("copied_dir/hello.txt")).unwrap(),
                b"shell32 payload"
            );

            // 2) FO_RENAME via SHFileOperationA
            let from_a = to_zz_a(&["S:\\ShellTest\\copied_dir\\hello.txt"]);
            let to_a = to_zz_a(&["S:\\ShellTest\\copied_dir\\renamed.txt"]);
            let mut op_a = SHFILEOPSTRUCTA {
                hwnd: std::ptr::null_mut(),
                w_func: FO_RENAME,
                p_from: from_a.as_ptr(),
                p_to: to_a.as_ptr(),
                f_flags: 0,
                f_any_operations_aborted: 0,
                h_name_mappings: std::ptr::null_mut(),
                lpsz_progress_title: std::ptr::null(),
            };
            assert_eq!(SHFileOperationA(&mut op_a as *mut _ as *mut u8), 0);
            assert!(!root.join("copied_dir/hello.txt").exists());
            assert!(root.join("copied_dir/renamed.txt").exists());

            // 3) FO_MOVE via SHFileOperationW
            let from_mv = to_zz_w(&["S:\\ShellTest\\copied_dir\\renamed.txt"]);
            let to_mv = to_zz_w(&["S:\\ShellTest\\moved.txt"]);
            op_w.w_func = FO_MOVE;
            op_w.p_from = from_mv.as_ptr();
            op_w.p_to = to_mv.as_ptr();
            assert_eq!(SHFileOperationW(&mut op_w as *mut _ as *mut u8), 0);
            assert!(!root.join("copied_dir/renamed.txt").exists());
            assert_eq!(std::fs::read(root.join("moved.txt")).unwrap(), b"shell32 payload");

            // 4) FO_DELETE via SHFileOperationA (multiple entries)
            let from_del = to_zz_a(&["S:\\ShellTest\\moved.txt", "S:\\ShellTest\\copied_dir"]);
            op_a.w_func = FO_DELETE;
            op_a.p_from = from_del.as_ptr();
            op_a.p_to = std::ptr::null();
            assert_eq!(SHFileOperationA(&mut op_a as *mut _ as *mut u8), 0);
            assert!(!root.join("moved.txt").exists());
            assert!(!root.join("copied_dir").exists());

            // 4b) Wildcard FO_DELETE via SHFileOperationW
            std::fs::write(root.join("a.tmp"), b"1").unwrap();
            std::fs::write(root.join("b.tmp"), b"2").unwrap();
            std::fs::write(root.join("keep.dat"), b"3").unwrap();
            let wild_del = to_zz_w(&["S:\\ShellTest\\*.tmp"]);
            op_w.w_func = FO_DELETE;
            op_w.p_from = wild_del.as_ptr();
            op_w.p_to = std::ptr::null();
            assert_eq!(SHFileOperationW(&mut op_w as *mut _ as *mut u8), 0);
            assert!(!root.join("a.tmp").exists());
            assert!(!root.join("b.tmp").exists());
            assert!(root.join("keep.dat").exists());

            // 5) Unmounted drive fails cleanly
            let bad = to_zz_w(&["X:\\NotMounted\\nope.txt"]);
            op_w.w_func = FO_DELETE;
            op_w.p_from = bad.as_ptr();
            assert_ne!(SHFileOperationW(&mut op_w as *mut _ as *mut u8), 0);
        }

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn sh_get_file_info_and_safe_shell_execute() {
        let root = std::env::temp_dir().join(format!("win32_shell32_info_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("sub")).unwrap();
        std::fs::write(root.join("notes.txt"), b"abc").unwrap();
        crate::path::add_mount("S:\\InfoTest", &root);

        unsafe {
            // SHGetFileInfoA on real file
            let mut sfi_a: SHFILEINFOA = std::mem::zeroed();
            let ret_a = SHGetFileInfoA(
                b"S:\\InfoTest\\notes.txt\0".as_ptr(),
                0,
                &mut sfi_a as *mut _ as *mut u8,
                std::mem::size_of::<SHFILEINFOA>() as u32,
                SHGFI_DISPLAYNAME | SHGFI_TYPENAME | SHGFI_ATTRIBUTES,
            );
            assert_ne!(ret_a, 0);
            assert_eq!(crate::strings::from_ansi(sfi_a.sz_display_name.as_ptr()), "notes.txt");
            assert_eq!(crate::strings::from_ansi(sfi_a.sz_type_name.as_ptr()), "Text Document");
            assert_ne!(sfi_a.dw_attributes & SFGAO_FILESYSTEM, 0);
            assert_eq!(sfi_a.dw_attributes & SFGAO_FOLDER, 0);

            // SHGetFileInfoW on real directory with subfolder
            let mut sfi_w: SHFILEINFOW = std::mem::zeroed();
            let dir_w = crate::strings::to_wide_z("S:\\InfoTest");
            let ret_w = SHGetFileInfoW(
                dir_w.as_ptr(),
                0,
                &mut sfi_w as *mut _ as *mut u8,
                std::mem::size_of::<SHFILEINFOW>() as u32,
                SHGFI_DISPLAYNAME | SHGFI_TYPENAME | SHGFI_ATTRIBUTES,
            );
            assert_ne!(ret_w, 0);
            assert_eq!(from_wide(sfi_w.sz_display_name.as_ptr()), "InfoTest");
            assert_eq!(from_wide(sfi_w.sz_type_name.as_ptr()), "File folder");
            assert_ne!(sfi_w.dw_attributes & SFGAO_FOLDER, 0);
            assert_ne!(sfi_w.dw_attributes & SFGAO_HASSUBFOLDER, 0);

            // Missing file returns 0 without SHGFI_USEFILEATTRIBUTES
            let missing_w = crate::strings::to_wide_z("S:\\InfoTest\\does_not_exist.bin");
            assert_eq!(
                SHGetFileInfoW(
                    missing_w.as_ptr(),
                    0,
                    &mut sfi_w as *mut _ as *mut u8,
                    std::mem::size_of::<SHFILEINFOW>() as u32,
                    SHGFI_TYPENAME,
                ),
                0
            );
            // With SHGFI_USEFILEATTRIBUTES it succeeds synthetically
            assert_ne!(
                SHGetFileInfoW(
                    missing_w.as_ptr(),
                    0x80,
                    &mut sfi_w as *mut _ as *mut u8,
                    std::mem::size_of::<SHFILEINFOW>() as u32,
                    SHGFI_USEFILEATTRIBUTES | SHGFI_TYPENAME,
                ),
                0
            );
            assert_eq!(from_wide(sfi_w.sz_type_name.as_ptr()), "BIN File");

            // ShellExecute* refuses host process execution safely
            assert_eq!(
                ShellExecuteA(
                    std::ptr::null_mut(),
                    b"open\0".as_ptr(),
                    b"cmd.exe\0".as_ptr(),
                    std::ptr::null(),
                    std::ptr::null(),
                    1
                ) as usize,
                SE_ERR_NOASSOC
            );
            let mut sei = [0u8; 112];
            std::ptr::write_unaligned(sei.as_mut_ptr() as *mut u32, 112u32);
            assert_eq!(ShellExecuteExA(sei.as_mut_ptr()), FALSE);
            assert_eq!(std::ptr::read_unaligned(sei.as_ptr().add(56) as *const usize), SE_ERR_NOASSOC);
        }

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn sh_create_item_from_parsing_name_and_ishellitem() {
        let root = std::env::temp_dir().join(format!("win32_shellitem_test_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("sub/nested")).unwrap();
        std::fs::write(root.join("sub/report.txt"), b"shell item contents").unwrap();
        crate::path::add_mount("S:\\ItemTest", &root);

        unsafe {
            let iid_item = crate::com::IID_IShellItem.to_bytes();
            let iid_unk = crate::com::IID_IUnknown.to_bytes();
            let iid_bogus = crate::com::guid("11111111-2222-3333-4444-555555555555").to_bytes();

            // 1. Existing file item via translated VFS path (forward slashes + extended prefix normalize cleanly).
            let file_w = crate::strings::to_wide_z("S:/ItemTest/sub/report.txt");
            let mut item_ptr: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                SHCreateItemFromParsingName(
                    file_w.as_ptr(),
                    std::ptr::null_mut(),
                    iid_item.as_ptr(),
                    &mut item_ptr
                ),
                S_OK
            );
            assert!(!item_ptr.is_null());

            // Interface & host vtable registration check in `crate::com`.
            let desc = crate::com::lookup_interface(&crate::com::IID_IShellItem)
                .expect("IShellItem registered in COM interface registry");
            assert_eq!(desc.name, "IShellItem");
            assert!(desc.host_vtbl.is_some());

            // 2. Identity & refcounting via IUnknown / IShellItem QI.
            let mut unk_ptr: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                crate::com::com_call_qi(item_ptr, &crate::com::IID_IUnknown, &mut unk_ptr),
                S_OK
            );
            assert_eq!(unk_ptr, item_ptr);
            let mut item_ptr2: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                crate::com::com_call_qi(unk_ptr, &crate::com::IID_IShellItem, &mut item_ptr2),
                S_OK
            );
            assert_eq!(item_ptr2, item_ptr);
            let mut bogus_ptr: *mut c_void = 0x1usize as *mut c_void;
            assert_eq!(
                crate::com::com_call_qi_bytes(item_ptr, iid_bogus.as_ptr(), &mut bogus_ptr),
                E_NOINTERFACE
            );
            assert!(bogus_ptr.is_null());
            assert_eq!(crate::com::com_call_release(item_ptr2), 2);
            assert_eq!(crate::com::com_call_release(unk_ptr), 1);

            // 3. GetDisplayName ownership (CoTaskMemAlloc -> CoTaskMemFree) across SIGDN variants.
            let read_and_free_name = |obj: *mut c_void, sigdn: u32| -> String {
                let mut name_ptr: LPWSTR = std::ptr::null_mut();
                let hr = crate::com::com_call_slot(
                    obj,
                    5,
                    sigdn as u64,
                    &mut name_ptr as *mut LPWSTR as u64,
                    0,
                );
                assert_eq!(hr, S_OK);
                assert!(!name_ptr.is_null());
                let s = from_wide(name_ptr);
                crate::misc::ole32::CoTaskMemFree(name_ptr as *mut c_void);
                s
            };
            assert_eq!(read_and_free_name(item_ptr, SIGDN_FILESYSPATH), "S:\\ItemTest\\sub\\report.txt");
            assert_eq!(read_and_free_name(item_ptr, SIGDN_DESKTOPABSOLUTEPARSING), "S:\\ItemTest\\sub\\report.txt");
            assert_eq!(read_and_free_name(item_ptr, SIGDN_NORMALDISPLAY), "report.txt");
            assert_eq!(read_and_free_name(item_ptr, SIGDN_PARENTRELATIVEPARSING), "report.txt");
            assert_eq!(read_and_free_name(item_ptr, SIGDN_URL), "file:///S:/ItemTest/sub/report.txt");

            // Unaligned (odd-address) Unicode parsing-name buffer through SHCreateItemFromParsingName.
            std::fs::write(root.join("sub/odd-\u{1F680}.txt"), b"odd unicode").unwrap();
            let odd_path = "S:\\ItemTest\\sub\\odd-\u{1F680}.txt";
            let odd_units = odd_path.encode_utf16().count() + 1;
            let mut odd_backing = vec![0u16; odd_units + 2];
            let odd_ptr = (odd_backing.as_mut_ptr() as *mut u8).add(1) as *mut u16;
            assert!(!odd_ptr.is_aligned());
            assert_eq!(crate::strings::write_wide_z(odd_ptr, odd_units, odd_path), odd_units - 1);
            let mut odd_item: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                SHCreateItemFromParsingName(
                    odd_ptr,
                    std::ptr::null_mut(),
                    iid_item.as_ptr(),
                    &mut odd_item,
                ),
                S_OK
            );
            assert!(!odd_item.is_null());
            assert_eq!(read_and_free_name(odd_item, SIGDN_FILESYSPATH), odd_path);
            assert_eq!(crate::com::com_call_release(odd_item), 0);

            let mut bad_name: LPWSTR = 0x1usize as LPWSTR;
            assert_eq!(
                crate::com::com_call_slot(
                    item_ptr,
                    5,
                    0xDEAD_BEEFu64,
                    &mut bad_name as *mut LPWSTR as u64,
                    0,
                ),
                E_INVALIDARG
            );
            assert!(bad_name.is_null());

            // 4. GetAttributes on file vs folder.
            let mut file_attrs: u32 = 0;
            assert_eq!(
                crate::com::com_call_slot(
                    item_ptr,
                    6,
                    (SFGAO_FILESYSTEM | SFGAO_FOLDER) as u64,
                    &mut file_attrs as *mut u32 as u64,
                    0,
                ),
                S_FALSE
            );
            assert_eq!(file_attrs, SFGAO_FILESYSTEM);

            let mut fs_only: u32 = 0;
            assert_eq!(
                crate::com::com_call_slot(
                    item_ptr,
                    6,
                    SFGAO_FILESYSTEM as u64,
                    &mut fs_only as *mut u32 as u64,
                    0,
                ),
                S_OK
            );
            assert_eq!(fs_only, SFGAO_FILESYSTEM);

            // 5. GetParent + Compare against a directly created directory item (case-insensitive).
            let mut parent_ptr: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                crate::com::com_call_slot(item_ptr, 4, &mut parent_ptr as *mut *mut c_void as u64, 0, 0),
                S_OK
            );
            assert!(!parent_ptr.is_null());
            assert_eq!(read_and_free_name(parent_ptr, SIGDN_FILESYSPATH), "S:\\ItemTest\\sub");
            assert_eq!(read_and_free_name(parent_ptr, SIGDN_NORMALDISPLAY), "sub");

            let mut dir_attrs: u32 = 0;
            assert_eq!(
                crate::com::com_call_slot(
                    parent_ptr,
                    6,
                    (SFGAO_FILESYSTEM | SFGAO_FOLDER | SFGAO_HASSUBFOLDER) as u64,
                    &mut dir_attrs as *mut u32 as u64,
                    0,
                ),
                S_OK
            );
            assert_eq!(dir_attrs, SFGAO_FILESYSTEM | SFGAO_FOLDER | SFGAO_HASSUBFOLDER);

            let dir_w = crate::strings::to_wide_z("\\\\?\\s:\\itemtest\\SUB");
            let mut direct_dir_ptr: *mut c_void = std::ptr::null_mut();
            assert_eq!(
                SHCreateItemFromParsingName(
                    dir_w.as_ptr(),
                    std::ptr::null_mut(),
                    iid_unk.as_ptr(),
                    &mut direct_dir_ptr
                ),
                S_OK
            );
            let mut order: i32 = 99;
            assert_eq!(
                crate::com::com_call_slot(
                    parent_ptr,
                    7,
                    direct_dir_ptr as u64,
                    0,
                    &mut order as *mut i32 as u64,
                ),
                S_OK
            );
            assert_eq!(order, 0);

            assert_eq!(
                crate::com::com_call_slot(
                    parent_ptr,
                    7,
                    item_ptr as u64,
                    0,
                    &mut order as *mut i32 as u64,
                ),
                S_FALSE
            );
            assert_ne!(order, 0);

            // 6. BindToHandler returns honest E_NOINTERFACE.
            let mut bound: *mut c_void = 0x1usize as *mut c_void;
            assert_eq!(
                crate::com::com_call_slot_raw(
                    item_ptr,
                    3,
                    &[
                        0,
                        iid_bogus.as_ptr() as u64,
                        iid_bogus.as_ptr() as u64,
                        &mut bound as *mut *mut c_void as u64
                    ]
                )
                .unwrap() as HRESULT,
                E_NOINTERFACE
            );
            assert!(bound.is_null());

            assert_eq!(crate::com::com_call_release(direct_dir_ptr), 0);
            assert_eq!(crate::com::com_call_release(parent_ptr), 0);
            assert_eq!(crate::com::com_call_release(item_ptr), 0);

            // 7. Missing filesystem items, unmounted drives, unsupported IIDs, and unsupported namespaces fail honestly.
            let missing_w = crate::strings::to_wide_z("S:\\ItemTest\\sub\\missing.txt");
            let mut out: *mut c_void = 0x1usize as *mut c_void;
            assert_eq!(
                SHCreateItemFromParsingName(
                    missing_w.as_ptr(),
                    std::ptr::null_mut(),
                    iid_item.as_ptr(),
                    &mut out
                ),
                hresult_from_win32(err::ERROR_FILE_NOT_FOUND)
            );
            assert!(out.is_null());

            let unmounted_w = crate::strings::to_wide_z("Q:\\NoSuchDrive\\file.txt");
            assert_eq!(
                SHCreateItemFromParsingName(
                    unmounted_w.as_ptr(),
                    std::ptr::null_mut(),
                    iid_item.as_ptr(),
                    &mut out
                ),
                hresult_from_win32(err::ERROR_PATH_NOT_FOUND)
            );
            assert!(out.is_null());

            assert_eq!(
                SHCreateItemFromParsingName(
                    file_w.as_ptr(),
                    std::ptr::null_mut(),
                    iid_bogus.as_ptr(),
                    &mut out
                ),
                E_NOINTERFACE
            );
            assert!(out.is_null());

            for unsupported in [
                "shell:::{20D04FE0-3AEA-1069-A2D8-08002B30309D}",
                "::{645FF040-5081-101B-9F08-00AA002F954E}",
                "http://example.com/file.txt",
                "ms-settings:display",
                "\\\\server\\share\\file.txt",
            ] {
                let w = crate::strings::to_wide_z(unsupported);
                out = 0x1usize as *mut c_void;
                assert_eq!(
                    SHCreateItemFromParsingName(
                        w.as_ptr(),
                        std::ptr::null_mut(),
                        iid_item.as_ptr(),
                        &mut out
                    ),
                    E_NOTIMPL,
                    "expected E_NOTIMPL for unsupported namespace {unsupported}"
                );
                assert!(out.is_null());
            }
        }

        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn sh_file_operation_and_find_block_private_capi_keystore() {
        let root = std::env::temp_dir().join(format!("pf_shell_ks_test_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let ks = root.join(".capi-keystore");
        let dst = root.join("exfil");
        std::fs::create_dir_all(&ks).unwrap();
        std::fs::create_dir_all(&dst).unwrap();
        std::fs::write(ks.join("user_test.key"), [0xAAu8; 32]).unwrap();
        std::fs::write(ks.join("user_test_containers.bin"), b"PFCAPIE1secret").unwrap();

        let prev_home = std::env::var_os("TRANSLATOR_HOME");
        let prev_drive = std::env::var_os("AW2_DRIVE_C");
        std::env::set_var("TRANSLATOR_HOME", &root);
        std::env::remove_var("AW2_DRIVE_C");
        crate::path::add_mount("V:\\", &root);

        // 1. Wildcard SHFileOperationW(FO_COPY) on V:\.capi-* must not match or copy .capi-keystore
        let rc_wild = execute_file_op(
            FO_COPY,
            &["V:\\.capi-*".to_string()],
            &["V:\\exfil".to_string()],
            0,
        );
        assert_eq!(rc_wild, err::ERROR_FILE_NOT_FOUND as i32);
        assert!(!dst.join(".capi-keystore").exists());

        // 2. Recursive SHFileOperationW(FO_COPY) on parent V:\ must be rejected before copying .capi-keystore
        let rc_rec = execute_file_op(
            FO_COPY,
            &["V:\\".to_string()],
            &["V:\\exfil\\copy".to_string()],
            0,
        );
        assert_eq!(rc_rec, err::ERROR_ACCESS_DENIED as i32);
        assert!(!dst.join("copy").join(".capi-keystore").exists());

        // 3. FindFirstFileW wildcard must not reveal .capi-keystore
        unsafe {
            let pat = crate::strings::to_wide_z("V:\\.capi-*");
            let mut fdata = [0u8; 592];
            let h = crate::kernel32::file::FindFirstFileW(pat.as_ptr(), fdata.as_mut_ptr());
            assert_eq!(h, INVALID_HANDLE_VALUE);
            assert_eq!(crate::teb::last_error(), err::ERROR_FILE_NOT_FOUND);
        }

        match prev_home {
            Some(v) => std::env::set_var("TRANSLATOR_HOME", v),
            None => std::env::remove_var("TRANSLATOR_HOME"),
        }
        match prev_drive {
            Some(v) => std::env::set_var("AW2_DRIVE_C", v),
            None => std::env::remove_var("AW2_DRIVE_C"),
        }
        let _ = std::fs::remove_dir_all(&root);
    }
}
