//! comdlg32.dll: Common Dialog Box Library (Open/Save File, Color, Font dialogs)
//! for non-gaming desktop applications.

use crate::types::*;
use std::sync::atomic::{AtomicU32, Ordering};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

static LAST_COMMDLG_ERR: AtomicU32 = AtomicU32::new(0);

pub unsafe extern "win64" fn CommDlgExtendedError() -> u32 {
    LAST_COMMDLG_ERR.load(Ordering::Relaxed)
}

pub unsafe extern "win64" fn GetOpenFileNameW(ofn: *mut u8) -> BOOL {
    if ofn.is_null() {
        LAST_COMMDLG_ERR.store(0xFFFF, Ordering::Relaxed); // CDERR_STRUCTSIZE
        return FALSE;
    }
    // Zero return (= user cancelled) with CommDlgExtendedError() == 0 is standard Win32 cancel behavior.
    LAST_COMMDLG_ERR.store(0, Ordering::Relaxed);
    FALSE
}

pub unsafe extern "win64" fn GetOpenFileNameA(ofn: *mut u8) -> BOOL {
    GetOpenFileNameW(ofn)
}

pub unsafe extern "win64" fn GetSaveFileNameW(ofn: *mut u8) -> BOOL {
    GetOpenFileNameW(ofn)
}

pub unsafe extern "win64" fn GetSaveFileNameA(ofn: *mut u8) -> BOOL {
    GetOpenFileNameW(ofn)
}

pub unsafe extern "win64" fn ChooseColorW(_cc: *mut u8) -> BOOL {
    LAST_COMMDLG_ERR.store(0, Ordering::Relaxed);
    FALSE
}

pub unsafe extern "win64" fn ChooseColorA(cc: *mut u8) -> BOOL {
    ChooseColorW(cc)
}

pub unsafe extern "win64" fn ChooseFontW(_cf: *mut u8) -> BOOL {
    LAST_COMMDLG_ERR.store(0, Ordering::Relaxed);
    FALSE
}

pub unsafe extern "win64" fn ChooseFontA(cf: *mut u8) -> BOOL {
    ChooseFontW(cf)
}

pub unsafe extern "win64" fn GetFileTitleA(
    lpsz_file: LPCSTR,
    lpsz_title: LPSTR,
    cch_size: u16,
) -> i16 {
    if lpsz_file.is_null() {
        return -1;
    }
    let full = crate::strings::from_ansi(lpsz_file);
    if full.is_empty() {
        return -1;
    }
    let title = full
        .rsplit(['\\', '/', ':'])
        .next()
        .unwrap_or(&full);
    let needed = title.len() + 1;
    if lpsz_title.is_null() || (cch_size as usize) < needed {
        return needed as i16;
    }
    std::ptr::copy_nonoverlapping(title.as_ptr(), lpsz_title, title.len());
    *lpsz_title.add(title.len()) = 0;
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_file_title_a_extracts_basename_and_sizes() {
        unsafe {
            assert_eq!(GetFileTitleA(std::ptr::null(), std::ptr::null_mut(), 0), -1);
            let p = b"C:\\Games\\Example\\Spider.exe\0";
            let need = GetFileTitleA(p.as_ptr(), std::ptr::null_mut(), 0);
            assert_eq!(need, 11); // "Spider.exe\0"
            let mut buf = [0u8; 32];
            assert_eq!(GetFileTitleA(p.as_ptr(), buf.as_mut_ptr(), 4), 11);
            assert_eq!(GetFileTitleA(p.as_ptr(), buf.as_mut_ptr(), 32), 0);
            assert_eq!(&buf[..11], b"Spider.exe\0");
        }
    }
}
