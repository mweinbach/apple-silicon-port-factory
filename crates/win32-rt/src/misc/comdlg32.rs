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
