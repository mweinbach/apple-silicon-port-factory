//! faultrep.dll: legacy Windows Error Reporting (`ReportFault`).
//!
//! No Dr. Watson / WER client exists on macOS; `ReportFault` logs the fault
//! record and honestly returns `frrvErrNoDW` (4) per `errorrep.h`.

use std::ffi::c_void;

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

/// `EFaultRepRetVal::frrvErrNoDW` — error reporting client could not be launched.
pub const FRRV_ERR_NO_DW: u32 = 4;

pub unsafe extern "win64" fn ReportFault(pep: *const c_void, dw_opt: u32) -> u32 {
    let exc_code = if !pep.is_null() {
        let rec = std::ptr::read_unaligned(pep as *const *const u32);
        if !rec.is_null() {
            Some(std::ptr::read_unaligned(rec))
        } else {
            None
        }
    } else {
        None
    };
    log::warn!(
        "faultrep!ReportFault(pep={pep:p}, opt={dw_opt:#x}, code={exc_code:#x?}) -> frrvErrNoDW ({FRRV_ERR_NO_DW})"
    );
    FRRV_ERR_NO_DW
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_fault_returns_documented_no_dw_status() {
        unsafe {
            assert_eq!(ReportFault(std::ptr::null(), 0), FRRV_ERR_NO_DW);
            let rec_code: u32 = 0xC000_0005;
            let ep = [&rec_code as *const u32 as *const c_void, std::ptr::null()];
            assert_eq!(ReportFault(ep.as_ptr().cast(), 0), FRRV_ERR_NO_DW);
            assert!(crate::lookup("faultrep.dll", "ReportFault").is_some());
        }
    }
}
