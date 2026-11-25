//! DirectWrite `IDWriteTextAnalyzer` (10 slots) and `IDWriteGlyphRunAnalysis` (6 slots).

use super::abi::*;
use super::font::{font_face_from_ptr, rendering_params_values_from_ptr, FontFaceInner};
use super::sfnt::rasterize_glyph_transformed;
use crate::com::{
    com_add_ref, com_call_release, com_release, qi_helper_bytes, register_vtable_once, ComObj,
};
use crate::types::*;
use crate::user32::RECT;
use std::ffi::c_void;
use std::sync::{Arc, OnceLock};

// ============================================================================
// IDWriteTextAnalyzer (10 slots)
// ============================================================================

pub(super) struct TextAnalyzerInner;
pub(super) type PTextAnalyzer = *mut ComObj<TextAnalyzerInner>;

unsafe extern "win64" fn ta_qi(this: PTextAnalyzer, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper_bytes(this, riid, ppv, &[G_IDWRITE_TEXT_ANALYZER])
}

/// Read `[text_pos .. text_pos + text_len)` from an `IDWriteTextAnalysisSource`
/// via slot 3 (`GetTextAtPosition(textPosition, &textString, &textLength)`).
unsafe fn read_analysis_source_text(
    source: *mut c_void,
    text_pos: u32,
    text_len: u32,
) -> Result<Vec<u16>, HRESULT> {
    if source.is_null() {
        return Err(E_INVALIDARG);
    }
    let vtbl = (source as *const *const usize).read_unaligned();
    if vtbl.is_null() {
        return Err(E_INVALIDARG);
    }
    let get_text_at = vtbl.add(3).read_unaligned();
    if get_text_at == 0 {
        return Err(E_INVALIDARG);
    }
    let mut out = Vec::with_capacity(text_len as usize);
    let mut cur = text_pos;
    let end = text_pos.saturating_add(text_len);
    while cur < end {
        let mut chunk_ptr: *const u16 = std::ptr::null();
        let mut chunk_len: u32 = 0;
        let hr = crate::call_guest8(
            get_text_at,
            source as u64,
            cur as u64,
            &mut chunk_ptr as *mut *const u16 as u64,
            &mut chunk_len as *mut u32 as u64,
            0,
            0,
            0,
            0,
        ) as HRESULT;
        if hr < 0 {
            return Err(hr);
        }
        if chunk_ptr.is_null() || chunk_len == 0 {
            break;
        }
        let take = (chunk_len as usize).min((end - cur) as usize);
        for i in 0..take {
            out.push(chunk_ptr.add(i).read_unaligned());
        }
        cur = cur.saturating_add(take as u32);
    }
    Ok(out)
}

/// Classify a Unicode scalar into a numeric script ID (ISO 15924-style grouping).
fn classify_script(ch: char) -> u16 {
    let c = ch as u32;
    match c {
        0x0041..=0x005A | 0x0061..=0x007A | 0x00C0..=0x024F | 0x1E00..=0x1EFF => 25, // Latin
        0x0370..=0x03FF | 0x1F00..=0x1FFF => 14,                                     // Greek
        0x0400..=0x052F | 0x2DE0..=0x2DFF | 0xA640..=0xA69F => 8,                    // Cyrillic
        0x0590..=0x05FF => 19,                                                       // Hebrew
        0x0600..=0x06FF | 0x0750..=0x077F | 0x08A0..=0x08FF => 2,                    // Arabic
        0x0900..=0x097F => 10,                                                       // Devanagari
        0x3040..=0x309F => 20,                                                       // Hiragana
        0x30A0..=0x30FF => 22,                                                       // Katakana
        0xAC00..=0xD7AF | 0x1100..=0x11FF => 18,                                     // Hangul
        0x4E00..=0x9FFF | 0x3400..=0x4DBF | 0xF900..=0xFAFF => 17,                   // Han
        _ => 0,                                                                      // Common / Inherited
    }
}

fn is_strong_ltr(ch: char) -> bool {
    let c = ch as u32;
    (0x0041..=0x005A).contains(&c)
        || (0x0061..=0x007A).contains(&c)
        || (0x00C0..=0x052F).contains(&c)
        || (0x3040..=0x9FFF).contains(&c)
}

fn decode_u16_at(units: &[u16], i: usize) -> (char, usize) {
    let u = units[i];
    if (0xD800..=0xDBFF).contains(&u) && i + 1 < units.len() {
        let lo = units[i + 1];
        if (0xDC00..=0xDFFF).contains(&lo) {
            let cp = 0x10000 + (((u as u32 - 0xD800) << 10) | (lo as u32 - 0xDC00));
            return (char::from_u32(cp).unwrap_or('\u{FFFD}'), 2);
        }
    }
    (char::from_u32(u as u32).unwrap_or('\u{FFFD}'), 1)
}

unsafe extern "win64" fn ta_analyze_script(
    this: PTextAnalyzer,
    source: *mut c_void,
    text_pos: u32,
    text_len: u32,
    sink: *mut c_void,
) -> HRESULT {
    if this.is_null() || source.is_null() || sink.is_null() {
        return E_INVALIDARG;
    }
    if text_len == 0 {
        return S_OK;
    }
    let units = match read_analysis_source_text(source, text_pos, text_len) {
        Ok(u) => u,
        Err(hr) => return hr,
    };
    if units.is_empty() {
        return S_OK;
    }
    let sink_vtbl = (sink as *const *const usize).read_unaligned();
    if sink_vtbl.is_null() {
        return E_INVALIDARG;
    }
    let set_script = sink_vtbl.add(3).read_unaligned();

    // Assign per-UTF-16-unit script, inheriting common/punctuation from neighbors.
    let mut scripts = vec![0u16; units.len()];
    let mut i = 0usize;
    while i < units.len() {
        let (ch, step) = decode_u16_at(&units, i);
        let sc = classify_script(ch);
        for k in 0..step {
            scripts[i + k] = sc;
        }
        i += step;
    }
    let mut active = scripts.iter().copied().find(|&s| s != 0).unwrap_or(25);
    for s in &mut scripts {
        if *s == 0 {
            *s = active;
        } else {
            active = *s;
        }
    }

    let mut run_start = 0usize;
    while run_start < scripts.len() {
        let sc = scripts[run_start];
        let mut run_end = run_start + 1;
        while run_end < scripts.len() && scripts[run_end] == sc {
            run_end += 1;
        }
        let analysis = DWRITE_SCRIPT_ANALYSIS { script: sc, shapes: 0 };
        let hr = crate::call_guest8(
            set_script,
            sink as u64,
            (text_pos + run_start as u32) as u64,
            (run_end - run_start) as u64,
            &analysis as *const DWRITE_SCRIPT_ANALYSIS as u64,
            0,
            0,
            0,
            0,
        ) as HRESULT;
        if hr < 0 {
            return hr;
        }
        run_start = run_end;
    }
    S_OK
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BidiClass {
    L,
    R,
    AL,
    EN,
    ES,
    ET,
    AN,
    CS,
    NSM,
    BN,
    WS,
    ON,
}

fn classify_bidi(ch: char) -> BidiClass {
    let cp = ch as u32;
    match cp {
        // European digits (ASCII + superscripts + Extended Arabic-Indic digits)
        0x0030..=0x0039 | 0x00B2..=0x00B3 | 0x00B9 | 0x06F0..=0x06F9 => BidiClass::EN,
        // Arabic-Indic digits & separators
        0x0660..=0x0669 | 0x066B..=0x066C => BidiClass::AN,
        // European separators
        0x002B | 0x002D => BidiClass::ES,
        // European terminators
        0x0023..=0x0025 | 0x00A2..=0x00A5 | 0x00B0..=0x00B1 | 0x0609..=0x060A | 0x066A | 0x20AC => {
            BidiClass::ET
        }
        // Common separators
        0x002C | 0x002E..=0x002F | 0x003A | 0x00A0 | 0x060C => BidiClass::CS,
        // Nonspacing combining marks
        0x0300..=0x036F
        | 0x0483..=0x0489
        | 0x0591..=0x05BD
        | 0x05BF
        | 0x05C1..=0x05C2
        | 0x05C4..=0x05C5
        | 0x05C7
        | 0x0610..=0x061A
        | 0x064B..=0x065F
        | 0x0670
        | 0x06D6..=0x06DC
        | 0x06DF..=0x06E4
        | 0x06E7..=0x06E8
        | 0x06EA..=0x06ED
        | 0x1AB0..=0x1AFF
        | 0x1DC0..=0x1DFF
        | 0x20D0..=0x20FF
        | 0xFE20..=0xFE2F => BidiClass::NSM,
        // Boundary neutrals
        0x0000..=0x0008 | 0x000E..=0x001B | 0x007F..=0x0084 | 0x0086..=0x009F | 0x200B..=0x200D => {
            BidiClass::BN
        }
        // Whitespace
        0x0009 | 0x000C | 0x0020 | 0x1680 | 0x2000..=0x200A | 0x2028 | 0x205F | 0x3000 => {
            BidiClass::WS
        }
        // Arabic letters (AL)
        0x0600..=0x06FF | 0x0750..=0x077F | 0x08A0..=0x08FF | 0xFB50..=0xFDFF | 0xFE70..=0xFEFF => {
            BidiClass::AL
        }
        // Strong RTL (Hebrew, Syriac, Thaana, NKo, etc.)
        0x0590..=0x05FF | 0x0700..=0x074F | 0x0780..=0x089F | 0xFB1D..=0xFB4F => BidiClass::R,
        _ if is_strong_ltr(ch) => BidiClass::L,
        _ => BidiClass::ON,
    }
}

/// Resolve per-UTF-16-code-unit bidi embedding levels following UAX #9 weak (W1-W7),
/// neutral (N1-N2), and implicit level (I1-I2) rules for a single paragraph run.
fn resolve_bidi_levels(units: &[u16], explicit: u8) -> Vec<u8> {
    let mut spans: Vec<(usize, usize, BidiClass)> = Vec::with_capacity(units.len());
    let mut i = 0usize;
    while i < units.len() {
        let (ch, step) = decode_u16_at(units, i);
        spans.push((i, step, classify_bidi(ch)));
        i += step;
    }
    let n = spans.len();
    let mut types: Vec<BidiClass> = spans.iter().map(|s| s.2).collect();
    let base_dir = if (explicit & 1) == 1 { BidiClass::R } else { BidiClass::L };

    // W1: Examine each nonspacing mark (NSM) and change its type to the previous type (or sos).
    let mut prev = base_dir;
    for t in &mut types {
        if *t == BidiClass::NSM {
            *t = prev;
        } else if *t != BidiClass::BN {
            prev = *t;
        }
    }

    // W2: Search backward from each EN until the first strong type (R, L, AL, or sos).
    //     If AL is found, change EN to AN.
    let mut last_strong = base_dir;
    for t in &mut types {
        match *t {
            BidiClass::L | BidiClass::R | BidiClass::AL => last_strong = *t,
            BidiClass::EN if last_strong == BidiClass::AL => *t = BidiClass::AN,
            _ => {}
        }
    }

    // W3: Change all ALs to R.
    for t in &mut types {
        if *t == BidiClass::AL {
            *t = BidiClass::R;
        }
    }

    // W4: A single ES between two ENs changes to EN; a single CS between two ENs or two ANs
    //     changes to that type.
    for idx in 1..n.saturating_sub(1) {
        match types[idx] {
            BidiClass::ES if types[idx - 1] == BidiClass::EN && types[idx + 1] == BidiClass::EN => {
                types[idx] = BidiClass::EN;
            }
            BidiClass::CS if types[idx - 1] == BidiClass::EN && types[idx + 1] == BidiClass::EN => {
                types[idx] = BidiClass::EN;
            }
            BidiClass::CS if types[idx - 1] == BidiClass::AN && types[idx + 1] == BidiClass::AN => {
                types[idx] = BidiClass::AN;
            }
            _ => {}
        }
    }

    // W5: Sequences of ET adjacent to EN change to EN.
    let mut k = 0usize;
    while k < n {
        if types[k] == BidiClass::ET {
            let start = k;
            while k < n && types[k] == BidiClass::ET {
                k += 1;
            }
            let left_en = start > 0 && types[start - 1] == BidiClass::EN;
            let right_en = k < n && types[k] == BidiClass::EN;
            if left_en || right_en {
                for t in &mut types[start..k] {
                    *t = BidiClass::EN;
                }
            }
        } else {
            k += 1;
        }
    }

    // W6: Remaining separators and terminators change to ON.
    for t in &mut types {
        if matches!(*t, BidiClass::ES | BidiClass::ET | BidiClass::CS) {
            *t = BidiClass::ON;
        }
    }

    // W7: Search backward from each EN until the first strong type (L, R, or sos).
    //     If L is found, change EN to L.
    let mut last_strong = base_dir;
    for t in &mut types {
        match *t {
            BidiClass::L | BidiClass::R => last_strong = *t,
            BidiClass::EN if last_strong == BidiClass::L => *t = BidiClass::L,
            _ => {}
        }
    }

    // N1 / N2: Resolve sequences of neutrals (ON, WS, BN). For N1, AN and EN act like R.
    let is_neutral = |t: BidiClass| matches!(t, BidiClass::ON | BidiClass::WS | BidiClass::BN);
    let strong_for_n1 = |t: BidiClass| match t {
        BidiClass::L => BidiClass::L,
        BidiClass::R | BidiClass::EN | BidiClass::AN => BidiClass::R,
        _ => base_dir,
    };
    let mut k = 0usize;
    while k < n {
        if is_neutral(types[k]) {
            let start = k;
            while k < n && is_neutral(types[k]) {
                k += 1;
            }
            let left = if start > 0 { strong_for_n1(types[start - 1]) } else { base_dir };
            let right = if k < n { strong_for_n1(types[k]) } else { base_dir };
            let resolved = if left == right { left } else { base_dir };
            for t in &mut types[start..k] {
                *t = resolved;
            }
        } else {
            k += 1;
        }
    }

    // I1 / I2: Resolve implicit embedding levels.
    let mut levels = vec![explicit; units.len()];
    for (idx, &(start, step, _)) in spans.iter().enumerate() {
        let lv = if (explicit & 1) == 0 {
            // I1 (even base level)
            match types[idx] {
                BidiClass::R => explicit + 1,
                BidiClass::AN | BidiClass::EN => explicit + 2,
                _ => explicit,
            }
        } else {
            // I2 (odd base level)
            match types[idx] {
                BidiClass::L | BidiClass::EN | BidiClass::AN => explicit + 1,
                _ => explicit,
            }
        };
        for off in 0..step {
            levels[start + off] = lv;
        }
    }
    levels
}

unsafe extern "win64" fn ta_analyze_bidi(
    this: PTextAnalyzer,
    source: *mut c_void,
    text_pos: u32,
    text_len: u32,
    sink: *mut c_void,
) -> HRESULT {
    if this.is_null() || source.is_null() || sink.is_null() {
        return E_INVALIDARG;
    }
    if text_len == 0 {
        return S_OK;
    }
    let units = match read_analysis_source_text(source, text_pos, text_len) {
        Ok(u) => u,
        Err(hr) => return hr,
    };
    if units.is_empty() {
        return S_OK;
    }
    let src_vtbl = (source as *const *const usize).read_unaligned();
    let get_dir = src_vtbl.add(5).read_unaligned();
    let dir = if get_dir != 0 {
        crate::call_guest8(get_dir, source as u64, 0, 0, 0, 0, 0, 0, 0) as u32
    } else {
        0
    };
    let explicit: u8 = if dir == 1 { 1 } else { 0 };

    let levels = resolve_bidi_levels(&units, explicit);

    let sink_vtbl = (sink as *const *const usize).read_unaligned();
    if sink_vtbl.is_null() {
        return E_INVALIDARG;
    }
    let set_bidi = sink_vtbl.add(5).read_unaligned();
    let mut run_start = 0usize;
    while run_start < levels.len() {
        let lv = levels[run_start];
        let mut run_end = run_start + 1;
        while run_end < levels.len() && levels[run_end] == lv {
            run_end += 1;
        }
        let hr = crate::call_guest8(
            set_bidi,
            sink as u64,
            (text_pos + run_start as u32) as u64,
            (run_end - run_start) as u64,
            explicit as u64,
            lv as u64,
            0,
            0,
            0,
        ) as HRESULT;
        if hr < 0 {
            return hr;
        }
        run_start = run_end;
    }
    S_OK
}

unsafe extern "win64" fn ta_analyze_number_substitution(
    this: PTextAnalyzer,
    source: *mut c_void,
    text_pos: u32,
    text_len: u32,
    sink: *mut c_void,
) -> HRESULT {
    if this.is_null() || source.is_null() || sink.is_null() {
        return E_INVALIDARG;
    }
    if text_len == 0 {
        return S_OK;
    }
    let src_vtbl = (source as *const *const usize).read_unaligned();
    let sink_vtbl = (sink as *const *const usize).read_unaligned();
    if src_vtbl.is_null() || sink_vtbl.is_null() {
        return E_INVALIDARG;
    }
    let get_num_sub = src_vtbl.add(7).read_unaligned();
    let set_num_sub = sink_vtbl.add(6).read_unaligned();
    if get_num_sub == 0 || set_num_sub == 0 {
        return S_OK;
    }
    let mut cur = text_pos;
    let end = text_pos.saturating_add(text_len);
    while cur < end {
        let mut run_len = 0u32;
        let mut num_sub: *mut c_void = std::ptr::null_mut();
        let hr = crate::call_guest8(
            get_num_sub,
            source as u64,
            cur as u64,
            &mut run_len as *mut u32 as u64,
            &mut num_sub as *mut *mut c_void as u64,
            0,
            0,
            0,
            0,
        ) as HRESULT;
        if hr < 0 {
            return hr;
        }
        let step = if run_len == 0 { end - cur } else { run_len.min(end - cur) };
        let hr2 = crate::call_guest8(
            set_num_sub,
            sink as u64,
            cur as u64,
            step as u64,
            num_sub as u64,
            0,
            0,
            0,
            0,
        ) as HRESULT;
        if !num_sub.is_null() {
            com_call_release(num_sub);
        }
        if hr2 < 0 {
            return hr2;
        }
        cur = cur.saturating_add(step);
    }
    S_OK
}

unsafe extern "win64" fn ta_analyze_line_breakpoints(
    this: PTextAnalyzer,
    source: *mut c_void,
    text_pos: u32,
    text_len: u32,
    sink: *mut c_void,
) -> HRESULT {
    if this.is_null() || source.is_null() || sink.is_null() {
        return E_INVALIDARG;
    }
    if text_len == 0 {
        return S_OK;
    }
    let units = match read_analysis_source_text(source, text_pos, text_len) {
        Ok(u) => u,
        Err(hr) => return hr,
    };
    if units.is_empty() {
        return S_OK;
    }
    // DWRITE_BREAK_CONDITION: 0=NEUTRAL, 1=CAN_BREAK, 2=MAY_NOT_BREAK, 3=MUST_BREAK
    let mut bps = Vec::with_capacity(units.len());
    for (idx, &u) in units.iter().enumerate() {
        let mut before: u8 = 0; // NEUTRAL
        let mut after: u8 = 0;
        let mut is_ws = false;
        let mut is_soft_hyphen = false;
        match u {
            0x000A => {
                // LF: must break after; do not break between CR and LF.
                if idx > 0 && units[idx - 1] == 0x000D {
                    before = 2;
                }
                after = 3;
                is_ws = true;
            }
            0x000D => {
                after = if idx + 1 < units.len() && units[idx + 1] == 0x000A { 2 } else { 3 };
                is_ws = true;
            }
            0x0020 | 0x0009 => {
                after = 1; // CAN_BREAK
                is_ws = true;
            }
            0x002D => {
                after = 1; // hyphen allows break after
            }
            0x00AD => {
                after = 1;
                is_soft_hyphen = true;
            }
            0x00A0 => {
                // NBSP: may not break before or after
                before = 2;
                after = 2;
                is_ws = true;
            }
            _ => {}
        }
        let bits = (before & 0x3)
            | ((after & 0x3) << 2)
            | ((is_ws as u8) << 4)
            | ((is_soft_hyphen as u8) << 5);
        bps.push(DWRITE_LINE_BREAKPOINT { bits });
    }

    let sink_vtbl = (sink as *const *const usize).read_unaligned();
    if sink_vtbl.is_null() {
        return E_INVALIDARG;
    }
    let set_bps = sink_vtbl.add(4).read_unaligned();
    crate::call_guest8(
        set_bps,
        sink as u64,
        text_pos as u64,
        bps.len() as u64,
        bps.as_ptr() as u64,
        0,
        0,
        0,
        0,
    ) as HRESULT
}

#[allow(clippy::too_many_arguments)]
pub(super) unsafe extern "win64" fn ta_get_glyphs(
    this: PTextAnalyzer,
    text_string: *const u16,
    text_len: u32,
    font_face: *mut c_void,
    _is_sideways: BOOL,
    is_rtl: BOOL,
    script_analysis: *const DWRITE_SCRIPT_ANALYSIS,
    _locale_name: *const u16,
    _number_substitution: *mut c_void,
    features: *const *const c_void,
    feature_range_lengths: *const u32,
    feature_ranges: u32,
    max_glyph_count: u32,
    cluster_map: *mut u16,
    text_props: *mut DWRITE_SHAPING_TEXT_PROPERTIES,
    glyph_indices: *mut u16,
    glyph_props: *mut DWRITE_SHAPING_GLYPH_PROPERTIES,
    actual_glyph_count: *mut u32,
) -> HRESULT {
    if this.is_null()
        || font_face.is_null()
        || actual_glyph_count.is_null()
        || (text_len > 0
            && (text_string.is_null()
                || cluster_map.is_null()
                || text_props.is_null()
                || glyph_indices.is_null()
                || glyph_props.is_null()))
    {
        return E_INVALIDARG;
    }
    let Some(face) = font_face_from_ptr(font_face) else {
        return E_INVALIDARG;
    };
    let mut units = Vec::with_capacity(text_len as usize);
    for i in 0..text_len as usize {
        units.push(text_string.add(i).read_unaligned());
    }
    let no_visual = !script_analysis.is_null() && (script_analysis.read_unaligned().shapes & 1) != 0;
    let feat_ranges = super::shaping::read_feature_ranges(
        features,
        feature_range_lengths,
        feature_ranges,
        units.len(),
    );
    let complex = super::shaping::needs_complex_shaping(&units, is_rtl != 0, !feat_ranges.is_empty());

    if let Some(shaped) = super::shaping::shape_run_platform(
        &face,
        &units,
        face.meta.metrics.designUnitsPerEm.max(1) as f32,
        is_rtl != 0,
        no_visual,
        &feat_ranges,
    ) {
        actual_glyph_count.write_unaligned(shaped.glyphs.len() as u32);
        if shaped.glyphs.len() > max_glyph_count as usize {
            return E_NOT_SUFFICIENT_BUFFER;
        }
        for (i, &cm) in shaped.cluster_map.iter().enumerate() {
            cluster_map.add(i).write_unaligned(cm);
        }
        for (i, &tp) in shaped.text_props.iter().enumerate() {
            text_props.add(i).write_unaligned(tp);
        }
        for (gi, g) in shaped.glyphs.iter().enumerate() {
            glyph_indices.add(gi).write_unaligned(g.gid);
            glyph_props.add(gi).write_unaligned(g.props);
        }
        return S_OK;
    }

    if complex {
        log::warn!(
            "dwrite: IDWriteTextAnalyzer::GetGlyphs cannot shape complex/RTL/featured run (len={}, family={:?}) without backing SFNT shaper",
            units.len(),
            face.meta.family_name,
        );
        actual_glyph_count.write_unaligned(0);
        return E_NOTIMPL;
    }

    // Simple LTR ASCII/Latin fallback when no backing SFNT bytes exist (e.g. built-in 8x8
    // fallback face or minimal synthetic unit-test TTF rejected by CoreText).
    let mut glyphs: Vec<(u16, u16, usize, usize)> = Vec::with_capacity(units.len()); // (gid, props_bits, start_u16, step)
    let mut i = 0usize;
    while i < units.len() {
        let (ch, step) = decode_u16_at(&units, i);
        let gid = face.codepoint_to_glyph(ch as u32);
        let is_zws = no_visual || ch == '\u{200B}';
        let justification: u16 = if ch == ' ' { 2 } else { 1 }; // BLANK vs CHARACTER
        let gprops = (justification & 0xF) | (1 << 4) /* isClusterStart */ | ((is_zws as u16) << 6);
        glyphs.push((gid, gprops, i, step));
        i += step;
    }

    actual_glyph_count.write_unaligned(glyphs.len() as u32);
    if glyphs.len() > max_glyph_count as usize {
        return E_NOT_SUFFICIENT_BUFFER;
    }

    for (gi, &(gid, gprops, start_u16, step)) in glyphs.iter().enumerate() {
        for k in 0..step {
            cluster_map.add(start_u16 + k).write_unaligned(gi as u16);
            text_props
                .add(start_u16 + k)
                .write_unaligned(DWRITE_SHAPING_TEXT_PROPERTIES { bits: 0 });
        }
        glyph_indices.add(gi).write_unaligned(gid);
        glyph_props
            .add(gi)
            .write_unaligned(DWRITE_SHAPING_GLYPH_PROPERTIES { bits: gprops });
    }
    S_OK
}

/// `IDWriteTextAnalyzer::GetGlyphPlacements` (19 parameters).
/// `font_em_size_bits` is argument #9 (always passed in a Win64 stack slot, never XMM0..3).
#[allow(clippy::too_many_arguments)]
pub(super) unsafe extern "win64" fn ta_get_glyph_placements(
    this: PTextAnalyzer,
    text_string: *const u16,
    _cluster_map: *const u16,
    _text_props: *const DWRITE_SHAPING_TEXT_PROPERTIES,
    text_len: u32,
    glyph_indices: *const u16,
    _glyph_props: *const DWRITE_SHAPING_GLYPH_PROPERTIES,
    glyph_count: u32,
    font_face: *mut c_void,
    font_em_size_bits: u32,
    _is_sideways: BOOL,
    is_rtl: BOOL,
    script_analysis: *const DWRITE_SCRIPT_ANALYSIS,
    _locale_name: *const u16,
    features: *const *const c_void,
    feature_range_lengths: *const u32,
    feature_ranges: u32,
    glyph_advances: *mut f32,
    glyph_offsets: *mut DWRITE_GLYPH_OFFSET,
) -> HRESULT {
    if this.is_null() || font_face.is_null() {
        return E_INVALIDARG;
    }
    if glyph_count > 0 && (glyph_indices.is_null() || glyph_advances.is_null() || glyph_offsets.is_null()) {
        return E_INVALIDARG;
    }
    let Some(face) = font_face_from_ptr(font_face) else {
        return E_INVALIDARG;
    };
    let em_size = f32::from_bits(font_em_size_bits);
    if !em_size.is_finite() || em_size < 0.0 {
        return E_INVALIDARG;
    }
    let no_visual = !script_analysis.is_null() && (script_analysis.read_unaligned().shapes & 1) != 0;
    if text_len > 0 && !text_string.is_null() && glyph_count > 0 {
        let mut units = Vec::with_capacity(text_len as usize);
        for i in 0..text_len as usize {
            units.push(text_string.add(i).read_unaligned());
        }
        let feat_ranges = super::shaping::read_feature_ranges(
            features,
            feature_range_lengths,
            feature_ranges,
            units.len(),
        );
        if let Some(shaped) = super::shaping::shape_run_platform(
            &face,
            &units,
            em_size,
            is_rtl != 0,
            no_visual,
            &feat_ranges,
        ) {
            if shaped.glyphs.len() == glyph_count as usize
                && (0..glyph_count as usize)
                    .all(|i| shaped.glyphs[i].gid == glyph_indices.add(i).read_unaligned())
            {
                for (i, g) in shaped.glyphs.iter().enumerate() {
                    glyph_advances.add(i).write_unaligned(g.advance);
                    glyph_offsets.add(i).write_unaligned(g.offset);
                }
                return S_OK;
            }
        } else if super::shaping::needs_complex_shaping(&units, is_rtl != 0, !feat_ranges.is_empty()) {
            log::warn!(
                "dwrite: IDWriteTextAnalyzer::GetGlyphPlacements cannot position complex run (len={}, family={:?})",
                units.len(),
                face.meta.family_name,
            );
            return E_NOTIMPL;
        }
    }
    for i in 0..glyph_count as usize {
        let gid = glyph_indices.add(i).read_unaligned();
        let adv = if no_visual { 0.0 } else { face.scaled_advance(gid, em_size) };
        glyph_advances.add(i).write_unaligned(adv);
        glyph_offsets.add(i).write_unaligned(DWRITE_GLYPH_OFFSET::default());
    }
    S_OK
}

/// `IDWriteTextAnalyzer::GetGdiCompatibleGlyphPlacements` (22 parameters).
/// `font_em_size_bits` (#9) and `pixels_per_dip_bits` (#10) are always Win64 stack slots.
#[allow(clippy::too_many_arguments)]
pub(super) unsafe extern "win64" fn ta_get_gdi_compatible_glyph_placements(
    this: PTextAnalyzer,
    text_string: *const u16,
    _cluster_map: *const u16,
    _text_props: *const DWRITE_SHAPING_TEXT_PROPERTIES,
    text_len: u32,
    glyph_indices: *const u16,
    _glyph_props: *const DWRITE_SHAPING_GLYPH_PROPERTIES,
    glyph_count: u32,
    font_face: *mut c_void,
    font_em_size_bits: u32,
    pixels_per_dip_bits: u32,
    _transform: *const DWRITE_MATRIX,
    use_gdi_natural: BOOL,
    _is_sideways: BOOL,
    is_rtl: BOOL,
    script_analysis: *const DWRITE_SCRIPT_ANALYSIS,
    _locale_name: *const u16,
    features: *const *const c_void,
    feature_range_lengths: *const u32,
    feature_ranges: u32,
    glyph_advances: *mut f32,
    glyph_offsets: *mut DWRITE_GLYPH_OFFSET,
) -> HRESULT {
    if this.is_null() || font_face.is_null() {
        return E_INVALIDARG;
    }
    if glyph_count > 0 && (glyph_indices.is_null() || glyph_advances.is_null() || glyph_offsets.is_null()) {
        return E_INVALIDARG;
    }
    let Some(face) = font_face_from_ptr(font_face) else {
        return E_INVALIDARG;
    };
    let em_size = f32::from_bits(font_em_size_bits);
    let ppd = f32::from_bits(pixels_per_dip_bits);
    if !em_size.is_finite() || em_size < 0.0 || !ppd.is_finite() || ppd <= 0.0 {
        return E_INVALIDARG;
    }
    let no_visual = !script_analysis.is_null() && (script_analysis.read_unaligned().shapes & 1) != 0;
    if text_len > 0 && !text_string.is_null() && glyph_count > 0 {
        let mut units = Vec::with_capacity(text_len as usize);
        for i in 0..text_len as usize {
            units.push(text_string.add(i).read_unaligned());
        }
        let feat_ranges = super::shaping::read_feature_ranges(
            features,
            feature_range_lengths,
            feature_ranges,
            units.len(),
        );
        if let Some(shaped) = super::shaping::shape_run_platform(
            &face,
            &units,
            em_size,
            is_rtl != 0,
            no_visual,
            &feat_ranges,
        ) {
            if shaped.glyphs.len() == glyph_count as usize
                && (0..glyph_count as usize)
                    .all(|i| shaped.glyphs[i].gid == glyph_indices.add(i).read_unaligned())
            {
                for (i, g) in shaped.glyphs.iter().enumerate() {
                    let mut adv = g.advance;
                    if use_gdi_natural == FALSE {
                        adv = (adv * ppd).round() / ppd;
                    }
                    glyph_advances.add(i).write_unaligned(adv);
                    glyph_offsets.add(i).write_unaligned(g.offset);
                }
                return S_OK;
            }
        } else if super::shaping::needs_complex_shaping(&units, is_rtl != 0, !feat_ranges.is_empty()) {
            log::warn!(
                "dwrite: IDWriteTextAnalyzer::GetGdiCompatibleGlyphPlacements cannot position complex run (len={}, family={:?})",
                units.len(),
                face.meta.family_name,
            );
            return E_NOTIMPL;
        }
    }
    for i in 0..glyph_count as usize {
        let gid = glyph_indices.add(i).read_unaligned();
        let mut adv = if no_visual { 0.0 } else { face.scaled_advance(gid, em_size) };
        if use_gdi_natural == FALSE {
            adv = (adv * ppd).round() / ppd;
        }
        glyph_advances.add(i).write_unaligned(adv);
        glyph_offsets.add(i).write_unaligned(DWRITE_GLYPH_OFFSET::default());
    }
    S_OK
}

#[cfg(target_arch = "aarch64")]
#[inline]
unsafe fn read_win64_stack_slot(arg_index: usize) -> Option<u64> {
    debug_assert!(arg_index >= 4);
    let (rsp_lo, rsp_hi) = crate::incoming_rsp();
    if rsp_lo == 0 {
        return None;
    }
    // Win64 stack layout at host_call/host_tail_call entry:
    // `[rsp + 0x00..0x20]` = shadow space, `[rsp + 0x20]` = 0-indexed arg 4,
    // `[rsp + 0x80]` = arg 16, ..., `[rsp + 0xA8]` = arg 21.
    let off = 0x20u64 + ((arg_index - 4) as u64) * 8;
    let addr = rsp_lo.checked_add(off)?;
    let end = addr.checked_add(8)?;
    if end > rsp_hi {
        return None;
    }
    Some(std::ptr::read_unaligned(addr as *const u64))
}

/// Guest-ABI entry thunk for `IDWriteTextAnalyzer::GetGlyphs` (18 parameters).
/// Receives all 16 forwarded 8-byte Win64 slots (`x0..x7` + `[sp+0x00..0x38]`) as `u64`
/// (avoiding Apple AAPCS64 4-byte packing of stack `u32`/`BOOL` args) and reads slots
/// 16..=17 directly from the active guest Win64 stack frame via [`crate::incoming_rsp`].
#[cfg(target_arch = "aarch64")]
#[allow(clippy::too_many_arguments)]
unsafe extern "C" fn ta_get_glyphs_entry(
    a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
    a7: u64,
    a8: u64,
    a9: u64,
    a10: u64,
    a11: u64,
    a12: u64,
    a13: u64,
    a14: u64,
    a15: u64,
) -> HRESULT {
    let (Some(a16), Some(a17)) = (read_win64_stack_slot(16), read_win64_stack_slot(17)) else {
        return E_INVALIDARG;
    };
    ta_get_glyphs(
        a0 as PTextAnalyzer,
        a1 as *const u16,
        a2 as u32,
        a3 as *mut c_void,
        a4 as BOOL,
        a5 as BOOL,
        a6 as *const DWRITE_SCRIPT_ANALYSIS,
        a7 as *const u16,
        a8 as *mut c_void,
        a9 as *const *const c_void,
        a10 as *const u32,
        a11 as u32,
        a12 as u32,
        a13 as *mut u16,
        a14 as *mut DWRITE_SHAPING_TEXT_PROPERTIES,
        a15 as *mut u16,
        a16 as *mut DWRITE_SHAPING_GLYPH_PROPERTIES,
        a17 as *mut u32,
    )
}

/// Guest-ABI entry thunk for `IDWriteTextAnalyzer::GetGlyphPlacements` (19 parameters).
#[cfg(target_arch = "aarch64")]
#[allow(clippy::too_many_arguments)]
unsafe extern "C" fn ta_get_glyph_placements_entry(
    a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
    a7: u64,
    a8: u64,
    a9: u64,
    a10: u64,
    a11: u64,
    a12: u64,
    a13: u64,
    a14: u64,
    a15: u64,
) -> HRESULT {
    let (Some(a16), Some(a17), Some(a18)) = (
        read_win64_stack_slot(16),
        read_win64_stack_slot(17),
        read_win64_stack_slot(18),
    ) else {
        return E_INVALIDARG;
    };
    ta_get_glyph_placements(
        a0 as PTextAnalyzer,
        a1 as *const u16,
        a2 as *const u16,
        a3 as *const DWRITE_SHAPING_TEXT_PROPERTIES,
        a4 as u32,
        a5 as *const u16,
        a6 as *const DWRITE_SHAPING_GLYPH_PROPERTIES,
        a7 as u32,
        a8 as *mut c_void,
        a9 as u32,
        a10 as BOOL,
        a11 as BOOL,
        a12 as *const DWRITE_SCRIPT_ANALYSIS,
        a13 as *const u16,
        a14 as *const *const c_void,
        a15 as *const u32,
        a16 as u32,
        a17 as *mut f32,
        a18 as *mut DWRITE_GLYPH_OFFSET,
    )
}

/// Guest-ABI entry thunk for `IDWriteTextAnalyzer::GetGdiCompatibleGlyphPlacements` (22 parameters).
#[cfg(target_arch = "aarch64")]
#[allow(clippy::too_many_arguments)]
unsafe extern "C" fn ta_get_gdi_compatible_glyph_placements_entry(
    a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
    a7: u64,
    a8: u64,
    a9: u64,
    a10: u64,
    a11: u64,
    a12: u64,
    a13: u64,
    a14: u64,
    a15: u64,
) -> HRESULT {
    let (Some(a16), Some(a17), Some(a18), Some(a19), Some(a20), Some(a21)) = (
        read_win64_stack_slot(16),
        read_win64_stack_slot(17),
        read_win64_stack_slot(18),
        read_win64_stack_slot(19),
        read_win64_stack_slot(20),
        read_win64_stack_slot(21),
    ) else {
        return E_INVALIDARG;
    };
    ta_get_gdi_compatible_glyph_placements(
        a0 as PTextAnalyzer,
        a1 as *const u16,
        a2 as *const u16,
        a3 as *const DWRITE_SHAPING_TEXT_PROPERTIES,
        a4 as u32,
        a5 as *const u16,
        a6 as *const DWRITE_SHAPING_GLYPH_PROPERTIES,
        a7 as u32,
        a8 as *mut c_void,
        a9 as u32,
        a10 as u32,
        a11 as *const DWRITE_MATRIX,
        a12 as BOOL,
        a13 as BOOL,
        a14 as BOOL,
        a15 as *const DWRITE_SCRIPT_ANALYSIS,
        a16 as *const u16,
        a17 as *const *const c_void,
        a18 as *const u32,
        a19 as u32,
        a20 as *mut f32,
        a21 as *mut DWRITE_GLYPH_OFFSET,
    )
}

fn text_analyzer_vtbl() -> *const usize {
    static V: OnceLock<[usize; 10]> = OnceLock::new();
    register_vtable_once(&V, || {
        #[cfg(target_arch = "aarch64")]
        let (get_glyphs, get_placements, get_gdi_placements) = (
            ta_get_glyphs_entry as *const () as usize,
            ta_get_glyph_placements_entry as *const () as usize,
            ta_get_gdi_compatible_glyph_placements_entry as *const () as usize,
        );
        #[cfg(not(target_arch = "aarch64"))]
        let (get_glyphs, get_placements, get_gdi_placements) = (
            ta_get_glyphs as *const () as usize,
            ta_get_glyph_placements as *const () as usize,
            ta_get_gdi_compatible_glyph_placements as *const () as usize,
        );
        [
            ta_qi as *const () as usize,                                    // 0
            com_add_ref::<TextAnalyzerInner> as *const () as usize,         // 1
            com_release::<TextAnalyzerInner> as *const () as usize,         // 2
            ta_analyze_script as *const () as usize,                        // 3
            ta_analyze_bidi as *const () as usize,                          // 4
            ta_analyze_number_substitution as *const () as usize,           // 5
            ta_analyze_line_breakpoints as *const () as usize,              // 6
            get_glyphs,                                                     // 7
            get_placements,                                                 // 8
            get_gdi_placements,                                             // 9
        ]
    })
}

pub(super) fn new_text_analyzer() -> *mut c_void {
    ComObj::new(text_analyzer_vtbl(), TextAnalyzerInner) as *mut c_void
}

// ============================================================================
// IDWriteGlyphRunAnalysis (6 slots)
// ============================================================================

struct PlacedGlyphBitmap {
    left: i32,
    top: i32,
    width: u32,
    height: u32,
    alpha: Vec<u8>,
}

pub(super) struct GlyphRunAnalysisInner {
    rendering_mode: u32,
    bounds: RECT,
    bitmaps: Vec<PlacedGlyphBitmap>,
}

pub(super) type PGlyphRunAnalysis = *mut ComObj<GlyphRunAnalysisInner>;

impl GlyphRunAnalysisInner {
    pub(super) unsafe fn build(
        run: &DWRITE_GLYPH_RUN,
        pixels_per_dip: f32,
        transform: Option<&DWRITE_MATRIX>,
        rendering_mode: u32,
        baseline_x: f32,
        baseline_y: f32,
    ) -> Result<Self, HRESULT> {
        let Some(face): Option<Arc<FontFaceInner>> = font_face_from_ptr(run.fontFace) else {
            return Err(E_INVALIDARG);
        };
        if !run.fontEmSize.is_finite() || run.fontEmSize < 0.0 || !pixels_per_dip.is_finite() || pixels_per_dip <= 0.0 {
            return Err(E_INVALIDARG);
        }
        if run.glyphCount > 0 && run.glyphIndices.is_null() {
            return Err(E_INVALIDARG);
        }

        let m = transform.copied().unwrap_or_default();
        let is_rtl = (run.bidiLevel & 1) != 0;
        let rtl_dir = if is_rtl { -1.0f32 } else { 1.0f32 };

        let mut pen_x = baseline_x;
        let pen_y = baseline_y;
        let mut bitmaps = Vec::with_capacity(run.glyphCount as usize);
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;

        for i in 0..run.glyphCount as usize {
            let gid = run.glyphIndices.add(i).read_unaligned();
            let adv = if !run.glyphAdvances.is_null() {
                run.glyphAdvances.add(i).read_unaligned()
            } else {
                face.scaled_advance(gid, run.fontEmSize)
            };
            let off = if !run.glyphOffsets.is_null() {
                run.glyphOffsets.add(i).read_unaligned()
            } else {
                DWRITE_GLYPH_OFFSET::default()
            };

            // Per DWRITE_GLYPH_RUN: for RTL runs (odd bidiLevel), the baseline origin is
            // the right edge of the run, so subtract each glyph's advance before placing
            // its left-origin outline; LTR runs post-increment the pen after placement.
            if is_rtl {
                pen_x -= adv;
            }

            let gx_dip = pen_x + off.advanceOffset * rtl_dir;
            let gy_dip = pen_y - off.ascenderOffset;
            let gx_px = (gx_dip * m.m11 + gy_dip * m.m21 + m.dx) * pixels_per_dip;
            let gy_px = (gx_dip * m.m12 + gy_dip * m.m22 + m.dy) * pixels_per_dip;

            let rast = rasterize_glyph_transformed(&face, gid, run.fontEmSize, pixels_per_dip, &m);
            if rast.width > 0 && rast.height > 0 {
                let left = gx_px.round() as i32 + rast.offset_x;
                let top = gy_px.round() as i32 + rast.offset_y;
                let right = left.saturating_add(rast.width as i32);
                let bottom = top.saturating_add(rast.height as i32);
                min_x = min_x.min(left);
                min_y = min_y.min(top);
                max_x = max_x.max(right);
                max_y = max_y.max(bottom);
                bitmaps.push(PlacedGlyphBitmap {
                    left,
                    top,
                    width: rast.width,
                    height: rast.height,
                    alpha: rast.alpha,
                });
            }
            if !is_rtl {
                pen_x += adv;
            }
        }

        let bounds = if min_x < max_x && min_y < max_y {
            RECT { left: min_x, top: min_y, right: max_x, bottom: max_y }
        } else {
            RECT::default()
        };
        Ok(Self { rendering_mode, bounds, bitmaps })
    }
}

unsafe extern "win64" fn gra_qi(this: PGlyphRunAnalysis, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper_bytes(this, riid, ppv, &[G_IDWRITE_GLYPH_RUN_ANALYSIS])
}

pub(super) unsafe extern "win64" fn gra_get_alpha_texture_bounds(
    this: PGlyphRunAnalysis,
    texture_type: u32,
    texture_bounds: *mut RECT,
) -> HRESULT {
    if this.is_null() || texture_bounds.is_null() {
        return E_POINTER;
    }
    if texture_type != DWRITE_TEXTURE_ALIASED_1X1 && texture_type != DWRITE_TEXTURE_CLEARTYPE_3X1 {
        return E_INVALIDARG;
    }
    texture_bounds.write_unaligned(ComObj::inner(this).bounds);
    S_OK
}

pub(super) unsafe extern "win64" fn gra_create_alpha_texture(
    this: PGlyphRunAnalysis,
    texture_type: u32,
    texture_bounds: *const RECT,
    alpha_values: *mut u8,
    buffer_size: u32,
) -> HRESULT {
    if this.is_null() || texture_bounds.is_null() {
        return E_POINTER;
    }
    let bpp: usize = match texture_type {
        DWRITE_TEXTURE_ALIASED_1X1 => 1,
        DWRITE_TEXTURE_CLEARTYPE_3X1 => 3,
        _ => return E_INVALIDARG,
    };
    let rc = texture_bounds.read_unaligned();
    if rc.right < rc.left || rc.bottom < rc.top {
        return E_INVALIDARG;
    }
    let w = (rc.right - rc.left) as usize;
    let h = (rc.bottom - rc.top) as usize;
    let needed = match w.checked_mul(h).and_then(|n| n.checked_mul(bpp)) {
        Some(n) => n,
        None => return E_INVALIDARG,
    };
    if needed == 0 {
        return S_OK;
    }
    if alpha_values.is_null() {
        return E_POINTER;
    }
    if (buffer_size as usize) < needed {
        return E_INVALIDARG;
    }
    std::ptr::write_bytes(alpha_values, 0, needed);

    let inner = ComObj::inner(this);
    let aliased_binary =
        texture_type == DWRITE_TEXTURE_ALIASED_1X1 && inner.rendering_mode == DWRITE_RENDERING_MODE_ALIASED;

    for bm in &inner.bitmaps {
        for sy in 0..bm.height as i32 {
            let dy = bm.top + sy - rc.top;
            if dy < 0 || dy as usize >= h {
                continue;
            }
            for sx in 0..bm.width as i32 {
                let dx = bm.left + sx - rc.left;
                if dx < 0 || dx as usize >= w {
                    continue;
                }
                let mut cov = bm.alpha[(sy as usize) * (bm.width as usize) + (sx as usize)];
                if cov == 0 {
                    continue;
                }
                if aliased_binary {
                    cov = if cov >= 128 { 255 } else { 0 };
                }
                let dst_idx = (dy as usize * w + dx as usize) * bpp;
                if bpp == 1 {
                    let prev = alpha_values.add(dst_idx).read_unaligned();
                    alpha_values.add(dst_idx).write_unaligned(prev.saturating_add(cov));
                } else {
                    for c in 0..3 {
                        let prev = alpha_values.add(dst_idx + c).read_unaligned();
                        alpha_values.add(dst_idx + c).write_unaligned(prev.saturating_add(cov));
                    }
                }
            }
        }
    }
    S_OK
}

pub(super) unsafe extern "win64" fn gra_get_alpha_blend_params(
    this: PGlyphRunAnalysis,
    rendering_params: *mut c_void,
    blend_gamma: *mut f32,
    blend_enhanced_contrast: *mut f32,
    blend_cleartype_level: *mut f32,
) -> HRESULT {
    if this.is_null() || blend_gamma.is_null() || blend_enhanced_contrast.is_null() || blend_cleartype_level.is_null() {
        return E_POINTER;
    }
    let (gamma, contrast, ct) = rendering_params_values_from_ptr(rendering_params)
        .unwrap_or((1.8, 0.5, 0.5));
    blend_gamma.write_unaligned(gamma);
    blend_enhanced_contrast.write_unaligned(contrast);
    blend_cleartype_level.write_unaligned(ct);
    S_OK
}

fn glyph_run_analysis_vtbl() -> *const usize {
    static V: OnceLock<[usize; 6]> = OnceLock::new();
    register_vtable_once(&V, || {
        [
            gra_qi as *const () as usize,                                   // 0
            com_add_ref::<GlyphRunAnalysisInner> as *const () as usize,     // 1
            com_release::<GlyphRunAnalysisInner> as *const () as usize,     // 2
            gra_get_alpha_texture_bounds as *const () as usize,             // 3
            gra_create_alpha_texture as *const () as usize,                 // 4
            gra_get_alpha_blend_params as *const () as usize,               // 5
        ]
    })
}

pub(super) fn new_glyph_run_analysis(inner: GlyphRunAnalysisInner) -> *mut c_void {
    ComObj::new(glyph_run_analysis_vtbl(), inner) as *mut c_void
}
