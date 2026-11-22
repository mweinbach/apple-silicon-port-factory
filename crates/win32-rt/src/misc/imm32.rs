//! imm32.dll: Input Method Manager (IME) with per-window and per-context state tracking.

use crate::types::*;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

// GCS_* composition string query indices
const GCS_COMPREADSTR: u32 = 0x0001;
const GCS_COMPSTR: u32 = 0x0008;
const GCS_CURSORPOS: u32 = 0x0080;
const GCS_RESULTREADSTR: u32 = 0x0200;
const GCS_RESULTSTR: u32 = 0x0800;

// ImmNotifyIME action & index constants
const NI_COMPOSITIONSTR: u32 = 0x0015;
const CPS_COMPLETE: u32 = 0x0001;
const CPS_CANCEL: u32 = 0x0004;

// ImmAssociateContextEx flags
const IACE_DEFAULT: u32 = 0x0010;

// ImmGetCompositionString error return codes
const IMM_ERROR_GENERAL: i32 = -2;

#[derive(Clone, Debug)]
struct ImeContext {
    open: bool,
    conversion_mode: u32,
    sentence_mode: u32,
    composition_utf16: Vec<u16>,
    result_utf16: Vec<u16>,
    reading_utf16: Vec<u16>,
    cursor_pos: u32,
    comp_window: [u8; 28],
    cand_window: [[u8; 32]; 4],
    #[allow(dead_code)]
    comp_font_w: [u8; 92],
    #[allow(dead_code)]
    comp_font_a: [u8; 60],
    ref_count: u32,
}

impl Default for ImeContext {
    fn default() -> Self {
        Self {
            open: false,
            conversion_mode: 0,
            sentence_mode: 0,
            composition_utf16: Vec::new(),
            result_utf16: Vec::new(),
            reading_utf16: Vec::new(),
            cursor_pos: 0,
            comp_window: [0u8; 28],
            cand_window: [[0u8; 32]; 4],
            comp_font_w: [0u8; 92],
            comp_font_a: [0u8; 60],
            ref_count: 0,
        }
    }
}

#[derive(Default)]
struct ImmState {
    contexts: HashMap<usize, ImeContext>,
    /// Explicit per-HWND association (`Some(0)` means IME explicitly disabled on that HWND).
    window_himc: HashMap<usize, usize>,
    /// Auto-created default context per HWND.
    default_himc: HashMap<usize, usize>,
    disabled_tids: HashSet<u32>,
    disabled_all: bool,
}

static NEXT_HIMC: AtomicUsize = AtomicUsize::new(0x001E_0004);

fn state() -> &'static Mutex<ImmState> {
    static S: OnceLock<Mutex<ImmState>> = OnceLock::new();
    S.get_or_init(|| Mutex::new(ImmState::default()))
}

fn alloc_context(st: &mut ImmState) -> usize {
    let h = NEXT_HIMC.fetch_add(4, Ordering::Relaxed);
    st.contexts.insert(h, ImeContext::default());
    h
}

pub unsafe extern "win64" fn ImmGetContext(hwnd: HWND) -> usize {
    let hwnd_key = hwnd as usize;
    if hwnd_key == 0 {
        return 0;
    }
    let tid = crate::kernel32::threads::GetCurrentThreadId();
    let mut st = state().lock().unwrap();
    if st.disabled_all || st.disabled_tids.contains(&tid) {
        return 0;
    }
    if let Some(&assoc) = st.window_himc.get(&hwnd_key) {
        if assoc == 0 {
            return 0;
        }
        if let Some(ctx) = st.contexts.get_mut(&assoc) {
            ctx.ref_count = ctx.ref_count.saturating_add(1);
            return assoc;
        }
    }
    let h = if let Some(&def) = st.default_himc.get(&hwnd_key) {
        if st.contexts.contains_key(&def) {
            def
        } else {
            let nh = alloc_context(&mut st);
            st.default_himc.insert(hwnd_key, nh);
            nh
        }
    } else {
        let nh = alloc_context(&mut st);
        st.default_himc.insert(hwnd_key, nh);
        nh
    };
    if let Some(ctx) = st.contexts.get_mut(&h) {
        ctx.ref_count = ctx.ref_count.saturating_add(1);
    }
    h
}

pub unsafe extern "win64" fn ImmReleaseContext(_hwnd: HWND, himc: usize) -> BOOL {
    if himc == 0 {
        return FALSE;
    }
    let mut st = state().lock().unwrap();
    if let Some(ctx) = st.contexts.get_mut(&himc) {
        ctx.ref_count = ctx.ref_count.saturating_sub(1);
        TRUE
    } else {
        FALSE
    }
}

pub unsafe extern "win64" fn ImmAssociateContext(hwnd: HWND, himc: usize) -> usize {
    let hwnd_key = hwnd as usize;
    if hwnd_key == 0 {
        return 0;
    }
    let mut st = state().lock().unwrap();
    if himc != 0 && !st.contexts.contains_key(&himc) {
        return 0;
    }
    let prev = st
        .window_himc
        .get(&hwnd_key)
        .copied()
        .or_else(|| st.default_himc.get(&hwnd_key).copied())
        .unwrap_or(0);
    st.window_himc.insert(hwnd_key, himc);
    prev
}

pub unsafe extern "win64" fn ImmAssociateContextEx(hwnd: HWND, himc: usize, flags: u32) -> BOOL {
    let hwnd_key = hwnd as usize;
    if hwnd_key == 0 {
        return FALSE;
    }
    let mut st = state().lock().unwrap();
    if (flags & IACE_DEFAULT) != 0 {
        st.window_himc.remove(&hwnd_key);
        return TRUE;
    }
    if himc != 0 && !st.contexts.contains_key(&himc) {
        return FALSE;
    }
    st.window_himc.insert(hwnd_key, himc);
    TRUE
}

pub unsafe extern "win64" fn ImmCreateContext() -> usize {
    let mut st = state().lock().unwrap();
    alloc_context(&mut st)
}

pub unsafe extern "win64" fn ImmDestroyContext(himc: usize) -> BOOL {
    if himc == 0 {
        return FALSE;
    }
    let mut st = state().lock().unwrap();
    if st.contexts.remove(&himc).is_none() {
        return FALSE;
    }
    st.window_himc.retain(|_, v| *v != himc);
    st.default_himc.retain(|_, v| *v != himc);
    TRUE
}

pub unsafe extern "win64" fn ImmSetCompositionWindow(himc: usize, form: *const u8) -> BOOL {
    if himc == 0 || form.is_null() {
        return FALSE;
    }
    let mut st = state().lock().unwrap();
    let Some(ctx) = st.contexts.get_mut(&himc) else {
        return FALSE;
    };
    std::ptr::copy_nonoverlapping(form, ctx.comp_window.as_mut_ptr(), 28);
    TRUE
}

pub unsafe extern "win64" fn ImmSetCandidateWindow(himc: usize, form: *const u8) -> BOOL {
    if himc == 0 || form.is_null() {
        return FALSE;
    }
    let idx = (form as *const u32).read_unaligned() as usize;
    if idx >= 4 {
        return FALSE;
    }
    let mut st = state().lock().unwrap();
    let Some(ctx) = st.contexts.get_mut(&himc) else {
        return FALSE;
    };
    std::ptr::copy_nonoverlapping(form, ctx.cand_window[idx].as_mut_ptr(), 32);
    TRUE
}

pub unsafe extern "win64" fn ImmGetCompositionStringW(
    himc: usize,
    index: u32,
    buf: *mut u8,
    buf_len: u32,
) -> i32 {
    if himc == 0 {
        return IMM_ERROR_GENERAL;
    }
    let st = state().lock().unwrap();
    let Some(ctx) = st.contexts.get(&himc) else {
        return IMM_ERROR_GENERAL;
    };
    if index == GCS_CURSORPOS {
        return ctx.cursor_pos as i32;
    }
    let units: &[u16] = match index {
        GCS_COMPSTR => &ctx.composition_utf16,
        GCS_RESULTSTR => &ctx.result_utf16,
        GCS_COMPREADSTR | GCS_RESULTREADSTR => &ctx.reading_utf16,
        _ => &[],
    };
    let byte_len = units.len() * 2;
    if buf.is_null() || buf_len == 0 {
        return byte_len as i32;
    }
    let copy_units = (byte_len.min(buf_len as usize)) / 2;
    for (i, &u) in units.iter().take(copy_units).enumerate() {
        (buf.add(i * 2) as *mut u16).write_unaligned(u);
    }
    (copy_units * 2) as i32
}

pub unsafe extern "win64" fn ImmGetCompositionStringA(
    himc: usize,
    index: u32,
    buf: *mut u8,
    buf_len: u32,
) -> i32 {
    if himc == 0 {
        return IMM_ERROR_GENERAL;
    }
    let st = state().lock().unwrap();
    let Some(ctx) = st.contexts.get(&himc) else {
        return IMM_ERROR_GENERAL;
    };
    if index == GCS_CURSORPOS {
        return ctx.cursor_pos as i32;
    }
    let units: &[u16] = match index {
        GCS_COMPSTR => &ctx.composition_utf16,
        GCS_RESULTSTR => &ctx.result_utf16,
        GCS_COMPREADSTR | GCS_RESULTREADSTR => &ctx.reading_utf16,
        _ => &[],
    };
    let bytes = String::from_utf16_lossy(units).into_bytes();
    if buf.is_null() || buf_len == 0 {
        return bytes.len() as i32;
    }
    let copy_len = bytes.len().min(buf_len as usize);
    if copy_len > 0 {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), buf, copy_len);
    }
    copy_len as i32
}

pub unsafe extern "win64" fn ImmNotifyIME(himc: usize, action: u32, index: u32, _value: u32) -> BOOL {
    if himc == 0 {
        return FALSE;
    }
    let mut st = state().lock().unwrap();
    let Some(ctx) = st.contexts.get_mut(&himc) else {
        return FALSE;
    };
    if action == NI_COMPOSITIONSTR {
        match index {
            CPS_COMPLETE => {
                ctx.result_utf16 = std::mem::take(&mut ctx.composition_utf16);
                ctx.cursor_pos = 0;
            }
            CPS_CANCEL => {
                ctx.composition_utf16.clear();
                ctx.cursor_pos = 0;
            }
            _ => {}
        }
    }
    TRUE
}

pub unsafe extern "win64" fn ImmGetOpenStatus(himc: usize) -> BOOL {
    if himc == 0 {
        return FALSE;
    }
    let st = state().lock().unwrap();
    st.contexts.get(&himc).map_or(FALSE, |c| if c.open { TRUE } else { FALSE })
}

pub unsafe extern "win64" fn ImmSetOpenStatus(himc: usize, open: BOOL) -> BOOL {
    if himc == 0 {
        return FALSE;
    }
    let mut st = state().lock().unwrap();
    let Some(ctx) = st.contexts.get_mut(&himc) else {
        return FALSE;
    };
    ctx.open = open != FALSE;
    TRUE
}

pub unsafe extern "win64" fn ImmGetProperty(_kl: usize, _index: u32) -> u32 {
    0x0009_0000 // IME_PROP_UNICODE | IME_PROP_AT_CARET
}

pub unsafe extern "win64" fn ImmGetCandidateListW(
    _himc: usize,
    _index: u32,
    _buf: *mut u8,
    _buf_len: u32,
) -> u32 {
    0
}

pub unsafe extern "win64" fn ImmGetCandidateListA(
    _himc: usize,
    _index: u32,
    _buf: *mut u8,
    _buf_len: u32,
) -> u32 {
    0
}

pub unsafe extern "win64" fn ImmGetCandidateListCountW(_himc: usize, list_count: *mut u32) -> u32 {
    if !list_count.is_null() {
        std::ptr::write_unaligned(list_count, 0);
    }
    0
}

pub unsafe extern "win64" fn ImmGetCandidateListCountA(_himc: usize, list_count: *mut u32) -> u32 {
    if !list_count.is_null() {
        std::ptr::write_unaligned(list_count, 0);
    }
    0
}

pub unsafe extern "win64" fn ImmGetConversionStatus(
    himc: usize,
    conv: *mut u32,
    sent: *mut u32,
) -> BOOL {
    if himc == 0 {
        return FALSE;
    }
    let st = state().lock().unwrap();
    let Some(ctx) = st.contexts.get(&himc) else {
        return FALSE;
    };
    if !conv.is_null() {
        std::ptr::write_unaligned(conv, ctx.conversion_mode);
    }
    if !sent.is_null() {
        std::ptr::write_unaligned(sent, ctx.sentence_mode);
    }
    TRUE
}

pub unsafe extern "win64" fn ImmSetConversionStatus(himc: usize, conv: u32, sent: u32) -> BOOL {
    if himc == 0 {
        return FALSE;
    }
    let mut st = state().lock().unwrap();
    let Some(ctx) = st.contexts.get_mut(&himc) else {
        return FALSE;
    };
    ctx.conversion_mode = conv;
    ctx.sentence_mode = sent;
    TRUE
}

pub unsafe extern "win64" fn ImmGetDefaultIMEWnd(_hwnd: HWND) -> HWND {
    std::ptr::null_mut()
}

pub unsafe extern "win64" fn ImmGetDescriptionW(_hkl: usize, _buf: *mut u16, _len: u32) -> u32 {
    0
}

pub unsafe extern "win64" fn ImmGetDescriptionA(_hkl: usize, _buf: *mut u8, _len: u32) -> u32 {
    0
}

pub unsafe extern "win64" fn ImmGetIMEFileNameW(_hkl: usize, _buf: *mut u16, _len: u32) -> u32 {
    0
}

pub unsafe extern "win64" fn ImmGetIMEFileNameA(_hkl: usize, _buf: *mut u8, _len: u32) -> u32 {
    0
}

pub unsafe extern "win64" fn ImmIsIME(_hkl: usize) -> BOOL {
    FALSE
}

pub unsafe extern "win64" fn ImmSetCompositionFontW(himc: usize, lf: *const u8) -> BOOL {
    if himc == 0 || lf.is_null() {
        return FALSE;
    }
    let mut st = state().lock().unwrap();
    let Some(ctx) = st.contexts.get_mut(&himc) else {
        return FALSE;
    };
    std::ptr::copy_nonoverlapping(lf, ctx.comp_font_w.as_mut_ptr(), 92);
    TRUE
}

pub unsafe extern "win64" fn ImmSetCompositionFontA(himc: usize, lf: *const u8) -> BOOL {
    if himc == 0 || lf.is_null() {
        return FALSE;
    }
    let mut st = state().lock().unwrap();
    let Some(ctx) = st.contexts.get_mut(&himc) else {
        return FALSE;
    };
    std::ptr::copy_nonoverlapping(lf, ctx.comp_font_a.as_mut_ptr(), 60);
    TRUE
}

pub unsafe extern "win64" fn ImmGetCompositionWindow(himc: usize, form: *mut u8) -> BOOL {
    if himc == 0 || form.is_null() {
        return FALSE;
    }
    let st = state().lock().unwrap();
    let Some(ctx) = st.contexts.get(&himc) else {
        return FALSE;
    };
    std::ptr::copy_nonoverlapping(ctx.comp_window.as_ptr(), form, 28);
    TRUE
}

pub unsafe extern "win64" fn ImmGetCandidateWindow(himc: usize, idx: u32, form: *mut u8) -> BOOL {
    if himc == 0 || form.is_null() || idx >= 4 {
        return FALSE;
    }
    let st = state().lock().unwrap();
    let Some(ctx) = st.contexts.get(&himc) else {
        return FALSE;
    };
    std::ptr::copy_nonoverlapping(ctx.cand_window[idx as usize].as_ptr(), form, 32);
    TRUE
}

pub unsafe extern "win64" fn ImmDisableIME(tid: u32) -> BOOL {
    let mut st = state().lock().unwrap();
    if tid == u32::MAX {
        st.disabled_all = true;
    } else if tid == 0 {
        let cur = crate::kernel32::threads::GetCurrentThreadId();
        st.disabled_tids.insert(cur);
    } else {
        st.disabled_tids.insert(tid);
    }
    TRUE
}

pub unsafe extern "win64" fn ImmGetVirtualKey(_hwnd: HWND) -> u32 {
    0xFF // VK_PROCESSKEY not pending
}

pub unsafe extern "win64" fn ImmSetCompositionStringW(
    himc: usize,
    _index: u32,
    comp: *const u8,
    comp_len: u32,
    read: *const u8,
    read_len: u32,
) -> BOOL {
    if himc == 0 {
        return FALSE;
    }
    let mut st = state().lock().unwrap();
    let Some(ctx) = st.contexts.get_mut(&himc) else {
        return FALSE;
    };
    if !comp.is_null() {
        let n = (comp_len as usize) / 2;
        let mut units = Vec::with_capacity(n);
        for i in 0..n {
            units.push((comp.add(i * 2) as *const u16).read_unaligned());
        }
        while units.last() == Some(&0) {
            units.pop();
        }
        ctx.cursor_pos = units.len() as u32;
        ctx.composition_utf16 = units;
    }
    if !read.is_null() {
        let n = (read_len as usize) / 2;
        let mut units = Vec::with_capacity(n);
        for i in 0..n {
            units.push((read.add(i * 2) as *const u16).read_unaligned());
        }
        while units.last() == Some(&0) {
            units.pop();
        }
        ctx.reading_utf16 = units;
    }
    TRUE
}

pub unsafe extern "win64" fn ImmSetCompositionStringA(
    himc: usize,
    _index: u32,
    comp: *const u8,
    comp_len: u32,
    read: *const u8,
    read_len: u32,
) -> BOOL {
    if himc == 0 {
        return FALSE;
    }
    let mut st = state().lock().unwrap();
    let Some(ctx) = st.contexts.get_mut(&himc) else {
        return FALSE;
    };
    if !comp.is_null() {
        let raw = std::slice::from_raw_parts(comp, comp_len as usize);
        let trimmed = match raw.iter().rposition(|&b| b != 0) {
            Some(end) => &raw[..=end],
            None => &[],
        };
        let s = String::from_utf8_lossy(trimmed);
        let units: Vec<u16> = s.encode_utf16().collect();
        ctx.cursor_pos = units.len() as u32;
        ctx.composition_utf16 = units;
    }
    if !read.is_null() {
        let raw = std::slice::from_raw_parts(read, read_len as usize);
        let trimmed = match raw.iter().rposition(|&b| b != 0) {
            Some(end) => &raw[..=end],
            None => &[],
        };
        let s = String::from_utf8_lossy(trimmed);
        ctx.reading_utf16 = s.encode_utf16().collect();
    }
    TRUE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imm_distinct_contexts_window_association_and_state() {
        unsafe {
            let hwnd1 = 0x0011_0001usize as HWND;
            let hwnd2 = 0x0011_0002usize as HWND;

            let c1 = ImmCreateContext();
            let c2 = ImmCreateContext();
            assert_ne!(c1, 0);
            assert_ne!(c2, 0);
            assert_ne!(c1, c2);

            // Per-window default contexts are distinct.
            let d1 = ImmGetContext(hwnd1);
            let d2 = ImmGetContext(hwnd2);
            assert_ne!(d1, 0);
            assert_ne!(d2, 0);
            assert_ne!(d1, d2);
            assert_eq!(ImmReleaseContext(hwnd1, d1), TRUE);
            assert_eq!(ImmReleaseContext(hwnd2, d2), TRUE);

            // Associate c1 with hwnd1, then disable with 0, then restore default.
            let prev = ImmAssociateContext(hwnd1, c1);
            assert_eq!(prev, d1);
            assert_eq!(ImmGetContext(hwnd1), c1);
            assert_eq!(ImmReleaseContext(hwnd1, c1), TRUE);

            assert_eq!(ImmAssociateContext(hwnd1, 0), c1);
            assert_eq!(ImmGetContext(hwnd1), 0);
            assert_eq!(ImmAssociateContextEx(hwnd1, 0, IACE_DEFAULT), TRUE);
            assert_eq!(ImmGetContext(hwnd1), d1);
            assert_eq!(ImmReleaseContext(hwnd1, d1), TRUE);

            // Open & conversion/sentence status are isolated per context.
            assert_eq!(ImmGetOpenStatus(c1), FALSE);
            assert_eq!(ImmSetOpenStatus(c1, TRUE), TRUE);
            assert_eq!(ImmGetOpenStatus(c1), TRUE);
            assert_eq!(ImmGetOpenStatus(c2), FALSE);

            assert_eq!(ImmSetConversionStatus(c1, 0x0001, 0x0008), TRUE);
            let (mut conv, mut sent) = (0u32, 0u32);
            assert_eq!(ImmGetConversionStatus(c1, &mut conv, &mut sent), TRUE);
            assert_eq!((conv, sent), (0x0001, 0x0008));

            // Composition and candidate window structs roundtrip.
            let mut comp_in = [0u8; 28];
            comp_in[0..4].copy_from_slice(&0x0002u32.to_le_bytes()); // CFS_POINT
            comp_in[4..8].copy_from_slice(&120i32.to_le_bytes());
            comp_in[8..12].copy_from_slice(&240i32.to_le_bytes());
            assert_eq!(ImmSetCompositionWindow(c1, comp_in.as_ptr()), TRUE);
            let mut comp_out = [0u8; 28];
            assert_eq!(ImmGetCompositionWindow(c1, comp_out.as_mut_ptr()), TRUE);
            assert_eq!(comp_out, comp_in);

            let mut cand_in = [0u8; 32];
            cand_in[0..4].copy_from_slice(&2u32.to_le_bytes()); // dwIndex = 2
            cand_in[4..8].copy_from_slice(&0x0040u32.to_le_bytes()); // CFS_CANDIDATEPOS
            cand_in[8..12].copy_from_slice(&300i32.to_le_bytes());
            assert_eq!(ImmSetCandidateWindow(c1, cand_in.as_ptr()), TRUE);
            let mut cand_out = [0u8; 32];
            assert_eq!(ImmGetCandidateWindow(c1, 2, cand_out.as_mut_ptr()), TRUE);
            assert_eq!(cand_out, cand_in);

            assert_eq!(ImmDestroyContext(c1), TRUE);
            assert_eq!(ImmDestroyContext(c1), FALSE);
            assert_eq!(ImmDestroyContext(c2), TRUE);
        }
    }

    #[test]
    fn imm_composition_string_w_and_a_and_complete() {
        unsafe {
            let ctx = ImmCreateContext();
            let text_w: Vec<u16> = "東京".encode_utf16().collect();
            let text_bytes: Vec<u8> = text_w.iter().flat_map(|u| u.to_le_bytes()).collect();
            assert_eq!(
                ImmSetCompositionStringW(
                    ctx,
                    0x0009,
                    text_bytes.as_ptr(),
                    text_bytes.len() as u32,
                    std::ptr::null(),
                    0,
                ),
                TRUE
            );

            // Query byte length in W and A.
            assert_eq!(ImmGetCompositionStringW(ctx, GCS_COMPSTR, std::ptr::null_mut(), 0), 4);
            assert_eq!(ImmGetCompositionStringW(ctx, GCS_CURSORPOS, std::ptr::null_mut(), 0), 2);
            let mut out_w = [0u8; 4];
            assert_eq!(ImmGetCompositionStringW(ctx, GCS_COMPSTR, out_w.as_mut_ptr(), 4), 4);
            assert_eq!(out_w, text_bytes.as_slice());

            let utf8_len = "東京".len() as i32;
            assert_eq!(ImmGetCompositionStringA(ctx, GCS_COMPSTR, std::ptr::null_mut(), 0), utf8_len);
            let mut out_a = vec![0u8; utf8_len as usize];
            assert_eq!(
                ImmGetCompositionStringA(ctx, GCS_COMPSTR, out_a.as_mut_ptr(), utf8_len as u32),
                utf8_len
            );
            assert_eq!(out_a, "東京".as_bytes());

            // CPS_COMPLETE commits composition string into GCS_RESULTSTR and clears GCS_COMPSTR.
            assert_eq!(ImmNotifyIME(ctx, NI_COMPOSITIONSTR, CPS_COMPLETE, 0), TRUE);
            assert_eq!(ImmGetCompositionStringW(ctx, GCS_COMPSTR, std::ptr::null_mut(), 0), 0);
            assert_eq!(ImmGetCompositionStringW(ctx, GCS_RESULTSTR, std::ptr::null_mut(), 0), 4);

            assert_eq!(ImmDestroyContext(ctx), TRUE);
            assert_eq!(ImmGetCompositionStringW(ctx, GCS_RESULTSTR, std::ptr::null_mut(), 0), IMM_ERROR_GENERAL);
        }
    }
}
