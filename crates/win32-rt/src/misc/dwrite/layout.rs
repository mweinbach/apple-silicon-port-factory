//! DirectWrite `IDWriteTextFormat`, `IDWriteTextLayout`, word-wrapping engine,
//! hit-testing, and drawable glyph-run extraction.

use super::abi::*;
use super::font::{
    font_collection_data_from_ptr, new_font_collection, new_font_face, FontCollectionData,
    FontFaceInner,
};
use crate::com::{
    com_add_ref, com_call_add_ref, com_call_release, com_from_iface, com_release, qi_helper_bytes,
    register_vtable_once, write_null_out, ComObj, ComPtr,
};
use crate::strings::read_wide_z;
use crate::types::*;
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::{Arc, Mutex, OnceLock};

// ============================================================================
// Text Format & Layout State + Word-Wrapping Engine
// ============================================================================

#[derive(Clone)]
pub(super) struct TextFormatState {
    pub(super) family_name: String,
    pub(super) collection: Arc<FontCollectionData>,
    pub(super) weight: u32,
    pub(super) style: u32,
    pub(super) stretch: u32,
    pub(super) font_size: f32,
    pub(super) locale_name: String,
    pub(super) text_alignment: u32,
    pub(super) paragraph_alignment: u32,
    pub(super) word_wrapping: u32,
    pub(super) reading_direction: u32,
    pub(super) flow_direction: u32,
    pub(super) incremental_tab_stop: f32,
    pub(super) trimming: DWRITE_TRIMMING,
    pub(super) trimming_sign: Option<ComPtr>,
    pub(super) line_spacing_method: u32,
    pub(super) line_spacing: f32,
    pub(super) baseline: f32,
}

pub(super) struct TextFormatInner {
    pub(super) state: Mutex<TextFormatState>,
}

#[derive(Clone)]
struct CharRunProps {
    family_name: String,
    collection: Arc<FontCollectionData>,
    weight: u32,
    style: u32,
    stretch: u32,
    font_size: f32,
    locale_name: String,
    underline: bool,
    strikethrough: bool,
    drawing_effect: Option<ComPtr>,
    inline_object: Option<ComPtr>,
    typography: Option<ComPtr>,
}

#[derive(Clone)]
struct LayoutCluster {
    text_pos: u32,
    utf16_len: u16,
    gid: u16,
    width: f32,
    is_whitespace: bool,
    is_newline: bool,
    can_wrap_after: bool,
    face: Arc<FontFaceInner>,
    font_size: f32,
    underline: bool,
    strikethrough: bool,
    drawing_effect: Option<ComPtr>,
    locale_name: String,
}

#[derive(Clone)]
struct LayoutLine {
    cluster_start: usize,
    cluster_end: usize,
    text_start: u32,
    utf16_len: u32,
    trailing_ws_len: u32,
    newline_len: u32,
    left: f32,
    top: f32,
    width: f32,
    width_with_trailing_ws: f32,
    height: f32,
    baseline: f32,
    is_trimmed: bool,
    cluster_lefts: Vec<f32>,
    cluster_widths: Vec<f32>,
}

struct ComputedLayout {
    clusters: Vec<LayoutCluster>,
    lines: Vec<LayoutLine>,
    metrics: DWRITE_TEXT_METRICS,
    overhang: DWRITE_OVERHANG_METRICS,
    min_width: f32,
}

pub(super) struct TextLayoutState {
    fmt: TextFormatState,
    text: Vec<u16>,
    max_width: f32,
    max_height: f32,
    char_props: Vec<CharRunProps>,
    dirty: bool,
    cached: Option<ComputedLayout>,
}

pub(super) struct TextLayoutInner {
    pub(super) state: Mutex<TextLayoutState>,
}

impl TextLayoutState {
    pub(super) fn new(text: Vec<u16>, fmt: TextFormatState, max_width: f32, max_height: f32) -> Self {
        let base = CharRunProps {
            family_name: fmt.family_name.clone(),
            collection: Arc::clone(&fmt.collection),
            weight: fmt.weight,
            style: fmt.style,
            stretch: fmt.stretch,
            font_size: fmt.font_size,
            locale_name: fmt.locale_name.clone(),
            underline: false,
            strikethrough: false,
            drawing_effect: None,
            inline_object: None,
            typography: None,
        };
        let char_props = vec![base; text.len()];
        Self {
            fmt,
            text,
            max_width: if max_width.is_finite() && max_width >= 0.0 { max_width } else { f32::MAX },
            max_height: if max_height.is_finite() && max_height >= 0.0 { max_height } else { f32::MAX },
            char_props,
            dirty: true,
            cached: None,
        }
    }

    fn props_at(&self, pos: u32) -> (CharRunProps, DWRITE_TEXT_RANGE) {
        let len = self.char_props.len();
        if len == 0 {
            let base = CharRunProps {
                family_name: self.fmt.family_name.clone(),
                collection: Arc::clone(&self.fmt.collection),
                weight: self.fmt.weight,
                style: self.fmt.style,
                stretch: self.fmt.stretch,
                font_size: self.fmt.font_size,
                locale_name: self.fmt.locale_name.clone(),
                underline: false,
                strikethrough: false,
                drawing_effect: None,
                inline_object: None,
                typography: None,
            };
            return (base, DWRITE_TEXT_RANGE { startPosition: pos, length: 0 });
        }
        let idx = (pos as usize).min(len - 1);
        let p = self.char_props[idx].clone();
        let mut end = idx + 1;
        while end < len
            && self.char_props[end].family_name == p.family_name
            && self.char_props[end].weight == p.weight
            && self.char_props[end].style == p.style
            && self.char_props[end].stretch == p.stretch
            && (self.char_props[end].font_size - p.font_size).abs() < 1e-4
            && self.char_props[end].underline == p.underline
            && self.char_props[end].strikethrough == p.strikethrough
            && self.char_props[end].locale_name == p.locale_name
        {
            end += 1;
        }
        (
            p,
            DWRITE_TEXT_RANGE {
                startPosition: idx as u32,
                length: (end - idx) as u32,
            },
        )
    }

    fn apply_range(&mut self, range: DWRITE_TEXT_RANGE, mut f: impl FnMut(&mut CharRunProps)) {
        let start = (range.startPosition as usize).min(self.char_props.len());
        let end = start.saturating_add(range.length as usize).min(self.char_props.len());
        for p in &mut self.char_props[start..end] {
            f(p);
        }
        self.dirty = true;
    }

    fn ensure_layout(&mut self) -> &ComputedLayout {
        if self.dirty || self.cached.is_none() {
            let computed = compute_text_layout(self);
            self.cached = Some(computed);
            self.dirty = false;
        }
        self.cached.as_ref().unwrap()
    }
}

pub(super) fn line_metrics_for_face(face: &FontFaceInner, em_size: f32, fmt: &TextFormatState) -> (f32, f32) {
    let m = &face.meta.metrics;
    let upem = m.designUnitsPerEm.max(1) as f32;
    let nat_ascent = (m.ascent as f32) * em_size / upem;
    let nat_descent = (m.descent as f32) * em_size / upem;
    let nat_gap = (m.lineGap.max(0) as f32) * em_size / upem;
    let nat_height = (nat_ascent + nat_descent + nat_gap).max(em_size);
    match fmt.line_spacing_method {
        DWRITE_LINE_SPACING_METHOD_UNIFORM if fmt.line_spacing > 0.0 => {
            (fmt.line_spacing, fmt.baseline.max(0.0))
        }
        DWRITE_LINE_SPACING_METHOD_PROPORTIONAL if fmt.line_spacing > 0.0 => {
            (nat_height * fmt.line_spacing, nat_ascent * fmt.baseline.max(0.5))
        }
        _ => (nat_height, nat_ascent),
    }
}

fn compute_text_layout(st: &TextLayoutState) -> ComputedLayout {
    // 1. Decode UTF-16 into clusters.
    let mut clusters: Vec<LayoutCluster> = Vec::new();
    let mut face_cache: HashMap<(String, u32, u32, u32), Arc<FontFaceInner>> = HashMap::new();
    let mut resolve_face = |col: &Arc<FontCollectionData>, fam: &str, w: u32, strch: u32, sty: u32| -> Arc<FontFaceInner> {
        let key = (fam.to_ascii_lowercase(), w, strch, sty);
        if let Some(f) = face_cache.get(&key) {
            return Arc::clone(f);
        }
        let f = col.resolve_face(fam, w, strch, sty);
        face_cache.insert(key, Arc::clone(&f));
        f
    };

    let mut i = 0usize;
    let n = st.text.len();
    while i < n {
        let u0 = st.text[i];
        let (cp, ulen, is_nl) = if u0 == 0x000D {
            if i + 1 < n && st.text[i + 1] == 0x000A {
                (0x000Au32, 2u16, true)
            } else {
                (0x000Du32, 1u16, true)
            }
        } else if u0 == 0x000A || u0 == 0x0085 || u0 == 0x2028 || u0 == 0x2029 {
            (u0 as u32, 1u16, true)
        } else if (0xD800..=0xDBFF).contains(&u0)
            && i + 1 < n
            && (0xDC00..=0xDFFF).contains(&st.text[i + 1])
        {
            let hi = (u0 as u32) - 0xD800;
            let lo = (st.text[i + 1] as u32) - 0xDC00;
            (0x1_0000 + (hi << 10) + lo, 2u16, false)
        } else {
            (u0 as u32, 1u16, false)
        };

        let props = &st.char_props[i];
        let face = resolve_face(&props.collection, &props.family_name, props.weight, props.stretch, props.style);
        let ch = char::from_u32(cp).unwrap_or('\u{FFFD}');
        let is_ws = !is_nl && (ch == ' ' || ch == '\t' || ch.is_whitespace());
        let gid = if is_nl { 0 } else { face.codepoint_to_glyph(cp) };
        let width = if is_nl {
            0.0
        } else if ch == '\t' {
            if st.fmt.incremental_tab_stop > 0.0 {
                st.fmt.incremental_tab_stop
            } else {
                face.scaled_advance(face.codepoint_to_glyph(b' ' as u32), props.font_size) * 4.0
            }
        } else {
            face.scaled_advance(gid, props.font_size)
        };
        let can_wrap_after = is_ws || is_nl || ch == '-';

        clusters.push(LayoutCluster {
            text_pos: i as u32,
            utf16_len: ulen,
            gid,
            width,
            is_whitespace: is_ws,
            is_newline: is_nl,
            can_wrap_after,
            face,
            font_size: props.font_size,
            underline: props.underline,
            strikethrough: props.strikethrough,
            drawing_effect: props.drawing_effect.clone(),
            locale_name: props.locale_name.clone(),
        });
        i += ulen as usize;
    }

    // 2. Compute minimum unbreakable word width.
    let mut min_width = 0.0f32;
    let mut cur_word_w = 0.0f32;
    for c in &clusters {
        if c.is_whitespace || c.is_newline {
            if cur_word_w > min_width {
                min_width = cur_word_w;
            }
            cur_word_w = 0.0;
        } else {
            cur_word_w += c.width;
        }
    }
    if cur_word_w > min_width {
        min_width = cur_word_w;
    }

    // 3. Line breaking / word wrapping.
    let default_face = resolve_face(
        &st.fmt.collection,
        &st.fmt.family_name,
        st.fmt.weight,
        st.fmt.stretch,
        st.fmt.style,
    );
    let (default_line_h, default_baseline) = line_metrics_for_face(&default_face, st.fmt.font_size, &st.fmt);
    let max_w = st.max_width;
    let wrap_mode = st.fmt.word_wrapping;

    let mut raw_ranges: Vec<(usize, usize)> = Vec::new();
    if clusters.is_empty() {
        raw_ranges.push((0, 0));
    } else {
        let mut line_start = 0usize;
        while line_start < clusters.len() {
            let mut line_end = line_start;
            let mut cur_w = 0.0f32;
            let mut last_break: Option<usize> = None;
            let mut hit_newline = false;

            while line_end < clusters.len() {
                let c = &clusters[line_end];
                if c.is_newline {
                    line_end += 1;
                    hit_newline = true;
                    break;
                }
                let next_w = cur_w + c.width;
                let exceeds = wrap_mode != DWRITE_WORD_WRAPPING_NO_WRAP
                    && !c.is_whitespace
                    && line_end > line_start
                    && next_w > max_w + 1e-4;

                if exceeds {
                    if wrap_mode == DWRITE_WORD_WRAPPING_CHARACTER {
                        break;
                    }
                    if let Some(brk) = last_break {
                        if brk > line_start {
                            line_end = brk;
                            break;
                        }
                    }
                    if wrap_mode == DWRITE_WORD_WRAPPING_WHOLE_WORD {
                        // Continue until next wrap opportunity
                        cur_w = next_w;
                        line_end += 1;
                        if c.can_wrap_after {
                            break;
                        }
                        continue;
                    }
                    // WRAP / EMERGENCY_BREAK: break mid-word.
                    break;
                }

                cur_w = next_w;
                line_end += 1;
                if c.can_wrap_after {
                    last_break = Some(line_end);
                }
            }

            if line_end == line_start {
                line_end = (line_start + 1).min(clusters.len());
            }
            raw_ranges.push((line_start, line_end));
            line_start = line_end;

            // Trailing explicit newline creates an empty trailing line.
            if hit_newline && line_start == clusters.len() {
                raw_ranges.push((line_start, line_start));
            }
        }
    }

    // 4. Compute line geometries, vertical alignment, and horizontal alignment.
    let mut lines: Vec<LayoutLine> = Vec::with_capacity(raw_ranges.len());
    let mut y_cursor = 0.0f32;
    let total_lines = raw_ranges.len();

    for (line_idx, &(cs, ce)) in raw_ranges.iter().enumerate() {
        let mut h = default_line_h;
        let mut bl = default_baseline;
        let mut utf16_len = 0u32;
        let mut nl_len = 0u32;
        let mut w_all = 0.0f32;

        if cs < ce {
            h = 0.0;
            bl = 0.0;
            for c in &clusters[cs..ce] {
                let (ch_h, ch_bl) = line_metrics_for_face(&c.face, c.font_size, &st.fmt);
                if ch_h > h { h = ch_h; }
                if ch_bl > bl { bl = ch_bl; }
                utf16_len += c.utf16_len as u32;
                if c.is_newline {
                    nl_len += c.utf16_len as u32;
                }
                w_all += c.width;
            }
        }

        // Count trailing whitespace (before any newline).
        let mut vis_end = ce;
        while vis_end > cs && clusters[vis_end - 1].is_newline {
            vis_end -= 1;
        }
        let mut ws_end = vis_end;
        let mut tws_len = 0u32;
        while ws_end > cs && clusters[ws_end - 1].is_whitespace {
            tws_len += clusters[ws_end - 1].utf16_len as u32;
            ws_end -= 1;
        }
        let w_vis: f32 = clusters[cs..ws_end].iter().map(|c| c.width).sum();

        let is_trimmed = st.fmt.trimming.granularity != DWRITE_TRIMMING_GRANULARITY_NONE
            && w_vis > max_w + 1e-4;

        let mut cluster_widths: Vec<f32> = clusters[cs..ce].iter().map(|c| c.width).collect();
        let mut line_width_vis = w_vis;

        // Justify intermediate lines when DWRITE_TEXT_ALIGNMENT_JUSTIFIED is active.
        if st.fmt.text_alignment == DWRITE_TEXT_ALIGNMENT_JUSTIFIED
            && line_idx + 1 < total_lines
            && max_w.is_finite()
            && max_w < 1e15
            && nl_len == 0
            && w_vis > 0.0
            && w_vis < max_w
        {
            let ws_count = clusters[cs..ws_end].iter().filter(|c| c.is_whitespace).count();
            if ws_count > 0 {
                let extra = (max_w - w_vis) / (ws_count as f32);
                for (k, c) in clusters[cs..ws_end].iter().enumerate() {
                    if c.is_whitespace {
                        cluster_widths[k] += extra;
                    }
                }
                line_width_vis = max_w;
                w_all += extra * (ws_count as f32);
            }
        }

        let align_w = if max_w.is_finite() && max_w < 1e15 { max_w } else { line_width_vis };
        let line_left = match st.fmt.text_alignment {
            DWRITE_TEXT_ALIGNMENT_TRAILING => align_w - line_width_vis,
            DWRITE_TEXT_ALIGNMENT_CENTER => (align_w - line_width_vis) * 0.5,
            _ => 0.0,
        };

        let mut cluster_lefts = Vec::with_capacity(ce - cs);
        let mut x_cur = line_left;
        for &cw in &cluster_widths {
            cluster_lefts.push(x_cur);
            x_cur += cw;
        }

        let text_start = if cs < clusters.len() {
            clusters[cs].text_pos
        } else {
            st.text.len() as u32
        };

        lines.push(LayoutLine {
            cluster_start: cs,
            cluster_end: ce,
            text_start,
            utf16_len,
            trailing_ws_len: tws_len,
            newline_len: nl_len,
            left: line_left,
            top: y_cursor,
            width: line_width_vis,
            width_with_trailing_ws: w_all,
            height: h,
            baseline: bl,
            is_trimmed,
            cluster_lefts,
            cluster_widths,
        });
        y_cursor += h;
    }

    let total_height = y_cursor;
    let v_offset = if st.max_height.is_finite() && st.max_height < 1e15 {
        match st.fmt.paragraph_alignment {
            DWRITE_PARAGRAPH_ALIGNMENT_FAR => st.max_height - total_height,
            DWRITE_PARAGRAPH_ALIGNMENT_CENTER => (st.max_height - total_height) * 0.5,
            _ => 0.0,
        }
    } else {
        0.0
    };
    if v_offset != 0.0 {
        for line in &mut lines {
            line.top += v_offset;
        }
    }

    // 5. Aggregate overall bounding & overhang metrics.
    let mut min_left = f32::INFINITY;
    let mut max_vis_w = 0.0f32;
    let mut max_all_w = 0.0f32;
    let mut ink_left = f32::INFINITY;
    let mut ink_top = f32::INFINITY;
    let mut ink_right = f32::NEG_INFINITY;
    let mut ink_bottom = f32::NEG_INFINITY;

    for line in &lines {
        if line.left < min_left {
            min_left = line.left;
        }
        if line.width > max_vis_w {
            max_vis_w = line.width;
        }
        if line.width_with_trailing_ws > max_all_w {
            max_all_w = line.width_with_trailing_ws;
        }
        if line.width > 0.0 {
            ink_left = ink_left.min(line.left);
            ink_right = ink_right.max(line.left + line.width);
            ink_top = ink_top.min(line.top);
            ink_bottom = ink_bottom.max(line.top + line.height);
        }
    }
    if !min_left.is_finite() {
        min_left = 0.0;
    }
    let metrics = DWRITE_TEXT_METRICS {
        left: min_left,
        top: v_offset,
        width: max_vis_w,
        widthIncludingTrailingWhitespace: max_all_w,
        height: total_height,
        layoutWidth: st.max_width,
        layoutHeight: st.max_height,
        maxBidiReorderingDepth: 1,
        lineCount: lines.len() as u32,
    };

    let overhang = if ink_left.is_finite() {
        DWRITE_OVERHANG_METRICS {
            left: -ink_left,
            top: -ink_top,
            right: ink_right - st.max_width,
            bottom: ink_bottom - st.max_height,
        }
    } else {
        DWRITE_OVERHANG_METRICS {
            left: 0.0,
            top: 0.0,
            right: -st.max_width,
            bottom: -st.max_height,
        }
    };

    ComputedLayout { clusters, lines, metrics, overhang, min_width }
}

// ============================================================================
// IDWriteTextFormat Vtable (28 slots)
// ============================================================================

pub(super) type PTextFormat = *mut ComObj<TextFormatInner>;

unsafe extern "win64" fn tf_qi(this: PTextFormat, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper_bytes(this, riid, ppv, &[G_IDWRITE_TEXT_FORMAT])
}
pub(super) unsafe extern "win64" fn tf_set_text_align(this: PTextFormat, align: u32) -> HRESULT {
    if this.is_null() || align > DWRITE_TEXT_ALIGNMENT_JUSTIFIED { return E_INVALIDARG; }
    ComObj::inner(this).state.lock().unwrap().text_alignment = align;
    S_OK
}
pub(super) unsafe extern "win64" fn tf_set_para_align(this: PTextFormat, align: u32) -> HRESULT {
    if this.is_null() || align > DWRITE_PARAGRAPH_ALIGNMENT_CENTER { return E_INVALIDARG; }
    ComObj::inner(this).state.lock().unwrap().paragraph_alignment = align;
    S_OK
}
unsafe extern "win64" fn tf_set_word_wrap(this: PTextFormat, wrap: u32) -> HRESULT {
    if this.is_null() || wrap > DWRITE_WORD_WRAPPING_CHARACTER { return E_INVALIDARG; }
    ComObj::inner(this).state.lock().unwrap().word_wrapping = wrap;
    S_OK
}
unsafe extern "win64" fn tf_set_reading_dir(this: PTextFormat, dir: u32) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    ComObj::inner(this).state.lock().unwrap().reading_direction = dir;
    S_OK
}
unsafe extern "win64" fn tf_set_flow_dir(this: PTextFormat, dir: u32) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    ComObj::inner(this).state.lock().unwrap().flow_direction = dir;
    S_OK
}
unsafe extern "win64" fn tf_set_tab_stop(this: PTextFormat, tab: f32) -> HRESULT {
    if this.is_null() || !tab.is_finite() || tab <= 0.0 { return E_INVALIDARG; }
    ComObj::inner(this).state.lock().unwrap().incremental_tab_stop = tab;
    S_OK
}
crate::win64_bridge!(fn tf_set_tab_stop(PTextFormat, f32) -> HRESULT);

unsafe extern "win64" fn tf_set_trimming(
    this: PTextFormat,
    opts: *const DWRITE_TRIMMING,
    sign: *mut c_void,
) -> HRESULT {
    if this.is_null() || opts.is_null() { return E_POINTER; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    st.trimming = opts.read_unaligned();
    st.trimming_sign = ComPtr::retain(sign);
    S_OK
}
unsafe extern "win64" fn tf_set_line_spacing(
    this: PTextFormat,
    method: u32,
    spacing: f32,
    baseline: f32,
) -> HRESULT {
    if this.is_null() || method > DWRITE_LINE_SPACING_METHOD_PROPORTIONAL || !spacing.is_finite() || spacing < 0.0 {
        return E_INVALIDARG;
    }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    st.line_spacing_method = method;
    st.line_spacing = spacing;
    st.baseline = baseline;
    S_OK
}
crate::win64_bridge!(fn tf_set_line_spacing(PTextFormat, u32, f32, f32) -> HRESULT);

unsafe extern "win64" fn tf_get_text_align(this: PTextFormat) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).state.lock().unwrap().text_alignment }
}
unsafe extern "win64" fn tf_get_para_align(this: PTextFormat) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).state.lock().unwrap().paragraph_alignment }
}
unsafe extern "win64" fn tf_get_word_wrap(this: PTextFormat) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).state.lock().unwrap().word_wrapping }
}
unsafe extern "win64" fn tf_get_reading_dir(this: PTextFormat) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).state.lock().unwrap().reading_direction }
}
unsafe extern "win64" fn tf_get_flow_dir(this: PTextFormat) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).state.lock().unwrap().flow_direction }
}
unsafe extern "win64" fn tf_get_tab_stop(this: PTextFormat) -> f32 {
    if this.is_null() { 0.0 } else { ComObj::inner(this).state.lock().unwrap().incremental_tab_stop }
}
unsafe extern "win64" fn tf_get_trimming(
    this: PTextFormat,
    opts: *mut DWRITE_TRIMMING,
    sign: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || opts.is_null() { return E_POINTER; }
    let st = ComObj::inner(this).state.lock().unwrap();
    opts.write_unaligned(st.trimming);
    if !sign.is_null() {
        if let Some(ref p) = st.trimming_sign {
            com_call_add_ref(p.as_ptr());
            sign.write_unaligned(p.as_ptr());
        } else {
            write_null_out(sign);
        }
    }
    S_OK
}
unsafe extern "win64" fn tf_get_line_spacing(
    this: PTextFormat,
    method: *mut u32,
    spacing: *mut f32,
    baseline: *mut f32,
) -> HRESULT {
    if this.is_null() || method.is_null() || spacing.is_null() || baseline.is_null() {
        return E_POINTER;
    }
    let st = ComObj::inner(this).state.lock().unwrap();
    method.write_unaligned(st.line_spacing_method);
    spacing.write_unaligned(st.line_spacing);
    baseline.write_unaligned(st.baseline);
    S_OK
}
unsafe extern "win64" fn tf_get_font_collection(this: PTextFormat, out: *mut *mut c_void) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let st = ComObj::inner(this).state.lock().unwrap();
    out.write_unaligned(new_font_collection(Arc::clone(&st.collection)));
    S_OK
}
unsafe extern "win64" fn tf_get_family_len(this: PTextFormat) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).state.lock().unwrap().family_name.encode_utf16().count() as u32 }
}
unsafe extern "win64" fn tf_get_family_name(this: PTextFormat, buf: *mut u16, size: u32) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    let st = ComObj::inner(this).state.lock().unwrap();
    write_wide_exact(&st.family_name, buf, size)
}
unsafe extern "win64" fn tf_get_weight(this: PTextFormat) -> u32 {
    if this.is_null() { DWRITE_FONT_WEIGHT_NORMAL } else { ComObj::inner(this).state.lock().unwrap().weight }
}
unsafe extern "win64" fn tf_get_style(this: PTextFormat) -> u32 {
    if this.is_null() { DWRITE_FONT_STYLE_NORMAL } else { ComObj::inner(this).state.lock().unwrap().style }
}
unsafe extern "win64" fn tf_get_stretch(this: PTextFormat) -> u32 {
    if this.is_null() { DWRITE_FONT_STRETCH_NORMAL } else { ComObj::inner(this).state.lock().unwrap().stretch }
}
unsafe extern "win64" fn tf_get_font_size(this: PTextFormat) -> f32 {
    if this.is_null() { 0.0 } else { ComObj::inner(this).state.lock().unwrap().font_size }
}
unsafe extern "win64" fn tf_get_locale_len(this: PTextFormat) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).state.lock().unwrap().locale_name.encode_utf16().count() as u32 }
}
unsafe extern "win64" fn tf_get_locale_name(this: PTextFormat, buf: *mut u16, size: u32) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    let st = ComObj::inner(this).state.lock().unwrap();
    write_wide_exact(&st.locale_name, buf, size)
}

pub(super) fn text_format_vtbl() -> *const usize {
    static V: OnceLock<[usize; 28]> = OnceLock::new();
    register_vtable_once(&V, || {
        [
            tf_qi as *const () as usize,
            com_add_ref::<TextFormatInner> as *const () as usize,
            com_release::<TextFormatInner> as *const () as usize,
            tf_set_text_align as *const () as usize,
            tf_set_para_align as *const () as usize,
            tf_set_word_wrap as *const () as usize,
            tf_set_reading_dir as *const () as usize,
            tf_set_flow_dir as *const () as usize,
            crate::bridge_entry!(tf_set_tab_stop) as usize,
            tf_set_trimming as *const () as usize,
            crate::bridge_entry!(tf_set_line_spacing) as usize,
            tf_get_text_align as *const () as usize,
            tf_get_para_align as *const () as usize,
            tf_get_word_wrap as *const () as usize,
            tf_get_reading_dir as *const () as usize,
            tf_get_flow_dir as *const () as usize,
            tf_get_tab_stop as *const () as usize,
            tf_get_trimming as *const () as usize,
            tf_get_line_spacing as *const () as usize,
            tf_get_font_collection as *const () as usize,
            tf_get_family_len as *const () as usize,
            tf_get_family_name as *const () as usize,
            tf_get_weight as *const () as usize,
            tf_get_style as *const () as usize,
            tf_get_stretch as *const () as usize,
            tf_get_font_size as *const () as usize,
            tf_get_locale_len as *const () as usize,
            tf_get_locale_name as *const () as usize,
        ]
    })
}

pub(super) unsafe fn text_format_state_from_ptr(ptr: *mut c_void) -> Option<TextFormatState> {
    if let Some(tf) = com_from_iface::<TextFormatInner>(ptr, text_format_vtbl()) {
        Some(tf.inner.state.lock().unwrap().clone())
    } else if let Some(tl) = com_from_iface::<TextLayoutInner>(ptr, text_layout_vtbl()) {
        Some(tl.inner.state.lock().unwrap().fmt.clone())
    } else {
        None
    }
}

// ============================================================================
// IDWriteTextLayout Vtable (67 slots: 28 inherited from IDWriteTextFormat + 39)
// ============================================================================

pub(super) type PTextLayout = *mut ComObj<TextLayoutInner>;

unsafe extern "win64" fn tl_qi(this: PTextLayout, riid: *const u8, ppv: *mut *mut c_void) -> HRESULT {
    qi_helper_bytes(this, riid, ppv, &[G_IDWRITE_TEXT_LAYOUT, G_IDWRITE_TEXT_FORMAT])
}
// Inherited IDWriteTextFormat methods (slots 3..27) forwarding to `st.fmt` + marking dirty on write.
unsafe extern "win64" fn tl_set_text_align(this: PTextLayout, align: u32) -> HRESULT {
    if this.is_null() || align > DWRITE_TEXT_ALIGNMENT_JUSTIFIED { return E_INVALIDARG; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    st.fmt.text_alignment = align;
    st.dirty = true;
    S_OK
}
unsafe extern "win64" fn tl_set_para_align(this: PTextLayout, align: u32) -> HRESULT {
    if this.is_null() || align > DWRITE_PARAGRAPH_ALIGNMENT_CENTER { return E_INVALIDARG; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    st.fmt.paragraph_alignment = align;
    st.dirty = true;
    S_OK
}
pub(super) unsafe extern "win64" fn tl_set_word_wrap(this: PTextLayout, wrap: u32) -> HRESULT {
    if this.is_null() || wrap > DWRITE_WORD_WRAPPING_CHARACTER { return E_INVALIDARG; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    st.fmt.word_wrapping = wrap;
    st.dirty = true;
    S_OK
}
unsafe extern "win64" fn tl_set_reading_dir(this: PTextLayout, dir: u32) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    st.fmt.reading_direction = dir;
    st.dirty = true;
    S_OK
}
unsafe extern "win64" fn tl_set_flow_dir(this: PTextLayout, dir: u32) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    st.fmt.flow_direction = dir;
    st.dirty = true;
    S_OK
}
unsafe extern "win64" fn tl_set_tab_stop(this: PTextLayout, tab: f32) -> HRESULT {
    if this.is_null() || !tab.is_finite() || tab <= 0.0 { return E_INVALIDARG; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    st.fmt.incremental_tab_stop = tab;
    st.dirty = true;
    S_OK
}
crate::win64_bridge!(fn tl_set_tab_stop(PTextLayout, f32) -> HRESULT);

unsafe extern "win64" fn tl_set_trimming(
    this: PTextLayout,
    opts: *const DWRITE_TRIMMING,
    sign: *mut c_void,
) -> HRESULT {
    if this.is_null() || opts.is_null() { return E_POINTER; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    st.fmt.trimming = opts.read_unaligned();
    st.fmt.trimming_sign = ComPtr::retain(sign);
    st.dirty = true;
    S_OK
}
unsafe extern "win64" fn tl_set_line_spacing(
    this: PTextLayout,
    method: u32,
    spacing: f32,
    baseline: f32,
) -> HRESULT {
    if this.is_null() || method > DWRITE_LINE_SPACING_METHOD_PROPORTIONAL || !spacing.is_finite() || spacing < 0.0 {
        return E_INVALIDARG;
    }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    st.fmt.line_spacing_method = method;
    st.fmt.line_spacing = spacing;
    st.fmt.baseline = baseline;
    st.dirty = true;
    S_OK
}
crate::win64_bridge!(fn tl_set_line_spacing(PTextLayout, u32, f32, f32) -> HRESULT);

unsafe extern "win64" fn tl_get_text_align(this: PTextLayout) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).state.lock().unwrap().fmt.text_alignment }
}
unsafe extern "win64" fn tl_get_para_align(this: PTextLayout) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).state.lock().unwrap().fmt.paragraph_alignment }
}
unsafe extern "win64" fn tl_get_word_wrap(this: PTextLayout) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).state.lock().unwrap().fmt.word_wrapping }
}
unsafe extern "win64" fn tl_get_reading_dir(this: PTextLayout) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).state.lock().unwrap().fmt.reading_direction }
}
unsafe extern "win64" fn tl_get_flow_dir(this: PTextLayout) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).state.lock().unwrap().fmt.flow_direction }
}
unsafe extern "win64" fn tl_get_tab_stop(this: PTextLayout) -> f32 {
    if this.is_null() { 0.0 } else { ComObj::inner(this).state.lock().unwrap().fmt.incremental_tab_stop }
}
unsafe extern "win64" fn tl_get_trimming(
    this: PTextLayout,
    opts: *mut DWRITE_TRIMMING,
    sign: *mut *mut c_void,
) -> HRESULT {
    if this.is_null() || opts.is_null() { return E_POINTER; }
    let st = ComObj::inner(this).state.lock().unwrap();
    opts.write_unaligned(st.fmt.trimming);
    if !sign.is_null() {
        if let Some(ref p) = st.fmt.trimming_sign {
            com_call_add_ref(p.as_ptr());
            sign.write_unaligned(p.as_ptr());
        } else {
            write_null_out(sign);
        }
    }
    S_OK
}
unsafe extern "win64" fn tl_get_line_spacing(
    this: PTextLayout,
    method: *mut u32,
    spacing: *mut f32,
    baseline: *mut f32,
) -> HRESULT {
    if this.is_null() || method.is_null() || spacing.is_null() || baseline.is_null() { return E_POINTER; }
    let st = ComObj::inner(this).state.lock().unwrap();
    method.write_unaligned(st.fmt.line_spacing_method);
    spacing.write_unaligned(st.fmt.line_spacing);
    baseline.write_unaligned(st.fmt.baseline);
    S_OK
}
unsafe extern "win64" fn tl_fmt_get_collection(this: PTextLayout, out: *mut *mut c_void) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    out.write_unaligned(new_font_collection(Arc::clone(&ComObj::inner(this).state.lock().unwrap().fmt.collection)));
    S_OK
}
unsafe extern "win64" fn tl_fmt_get_family_len(this: PTextLayout) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).state.lock().unwrap().fmt.family_name.encode_utf16().count() as u32 }
}
unsafe extern "win64" fn tl_fmt_get_family_name(this: PTextLayout, buf: *mut u16, size: u32) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    write_wide_exact(&ComObj::inner(this).state.lock().unwrap().fmt.family_name, buf, size)
}
unsafe extern "win64" fn tl_fmt_get_weight(this: PTextLayout) -> u32 {
    if this.is_null() { DWRITE_FONT_WEIGHT_NORMAL } else { ComObj::inner(this).state.lock().unwrap().fmt.weight }
}
unsafe extern "win64" fn tl_fmt_get_style(this: PTextLayout) -> u32 {
    if this.is_null() { DWRITE_FONT_STYLE_NORMAL } else { ComObj::inner(this).state.lock().unwrap().fmt.style }
}
unsafe extern "win64" fn tl_fmt_get_stretch(this: PTextLayout) -> u32 {
    if this.is_null() { DWRITE_FONT_STRETCH_NORMAL } else { ComObj::inner(this).state.lock().unwrap().fmt.stretch }
}
unsafe extern "win64" fn tl_fmt_get_font_size(this: PTextLayout) -> f32 {
    if this.is_null() { 0.0 } else { ComObj::inner(this).state.lock().unwrap().fmt.font_size }
}
unsafe extern "win64" fn tl_fmt_get_locale_len(this: PTextLayout) -> u32 {
    if this.is_null() { 0 } else { ComObj::inner(this).state.lock().unwrap().fmt.locale_name.encode_utf16().count() as u32 }
}
unsafe extern "win64" fn tl_fmt_get_locale_name(this: PTextLayout, buf: *mut u16, size: u32) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    write_wide_exact(&ComObj::inner(this).state.lock().unwrap().fmt.locale_name, buf, size)
}

// IDWriteTextLayout-specific setters (slots 28..41)
pub(super) unsafe extern "win64" fn tl_set_max_width(this: PTextLayout, max_w: f32) -> HRESULT {
    if this.is_null() || max_w.is_nan() || max_w < 0.0 { return E_INVALIDARG; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    st.max_width = max_w;
    st.dirty = true;
    S_OK
}
crate::win64_bridge!(fn tl_set_max_width(PTextLayout, f32) -> HRESULT);

unsafe extern "win64" fn tl_set_max_height(this: PTextLayout, max_h: f32) -> HRESULT {
    if this.is_null() || max_h.is_nan() || max_h < 0.0 { return E_INVALIDARG; }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    st.max_height = max_h;
    st.dirty = true;
    S_OK
}
crate::win64_bridge!(fn tl_set_max_height(PTextLayout, f32) -> HRESULT);

unsafe extern "win64" fn tl_set_font_collection(this: PTextLayout, col: *mut c_void, range: u64) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    let data = font_collection_data_from_ptr(col);
    ComObj::inner(this).state.lock().unwrap().apply_range(DWRITE_TEXT_RANGE::from_u64(range), |p| {
        p.collection = Arc::clone(&data);
    });
    S_OK
}
unsafe extern "win64" fn tl_set_font_family(this: PTextLayout, name: *const u16, range: u64) -> HRESULT {
    if this.is_null() || name.is_null() { return E_POINTER; }
    let fam = read_wide_z(name, DW_MAX_STR);
    ComObj::inner(this).state.lock().unwrap().apply_range(DWRITE_TEXT_RANGE::from_u64(range), |p| {
        p.family_name = fam.clone();
    });
    S_OK
}
unsafe extern "win64" fn tl_set_font_weight(this: PTextLayout, weight: u32, range: u64) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    ComObj::inner(this).state.lock().unwrap().apply_range(DWRITE_TEXT_RANGE::from_u64(range), |p| {
        p.weight = weight;
    });
    S_OK
}
unsafe extern "win64" fn tl_set_font_style(this: PTextLayout, style: u32, range: u64) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    ComObj::inner(this).state.lock().unwrap().apply_range(DWRITE_TEXT_RANGE::from_u64(range), |p| {
        p.style = style;
    });
    S_OK
}
unsafe extern "win64" fn tl_set_font_stretch(this: PTextLayout, stretch: u32, range: u64) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    ComObj::inner(this).state.lock().unwrap().apply_range(DWRITE_TEXT_RANGE::from_u64(range), |p| {
        p.stretch = stretch;
    });
    S_OK
}
unsafe extern "win64" fn tl_set_font_size(this: PTextLayout, size: f32, range: u64) -> HRESULT {
    if this.is_null() || !size.is_finite() || size <= 0.0 { return E_INVALIDARG; }
    ComObj::inner(this).state.lock().unwrap().apply_range(DWRITE_TEXT_RANGE::from_u64(range), |p| {
        p.font_size = size;
    });
    S_OK
}
crate::win64_bridge!(fn tl_set_font_size(PTextLayout, f32, u64) -> HRESULT);

pub(super) unsafe extern "win64" fn tl_set_underline(this: PTextLayout, has: BOOL, range: u64) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    let v = has != 0;
    ComObj::inner(this).state.lock().unwrap().apply_range(DWRITE_TEXT_RANGE::from_u64(range), |p| {
        p.underline = v;
    });
    S_OK
}
unsafe extern "win64" fn tl_set_strikethrough(this: PTextLayout, has: BOOL, range: u64) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    let v = has != 0;
    ComObj::inner(this).state.lock().unwrap().apply_range(DWRITE_TEXT_RANGE::from_u64(range), |p| {
        p.strikethrough = v;
    });
    S_OK
}
unsafe extern "win64" fn tl_set_drawing_effect(this: PTextLayout, effect: *mut c_void, range: u64) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    let retained = ComPtr::retain(effect);
    ComObj::inner(this).state.lock().unwrap().apply_range(DWRITE_TEXT_RANGE::from_u64(range), |p| {
        p.drawing_effect = retained.clone();
    });
    S_OK
}
unsafe extern "win64" fn tl_set_inline_object(this: PTextLayout, obj: *mut c_void, range: u64) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    let retained = ComPtr::retain(obj);
    ComObj::inner(this).state.lock().unwrap().apply_range(DWRITE_TEXT_RANGE::from_u64(range), |p| {
        p.inline_object = retained.clone();
    });
    S_OK
}
unsafe extern "win64" fn tl_set_typography(this: PTextLayout, typo: *mut c_void, range: u64) -> HRESULT {
    if this.is_null() { return E_INVALIDARG; }
    let retained = ComPtr::retain(typo);
    ComObj::inner(this).state.lock().unwrap().apply_range(DWRITE_TEXT_RANGE::from_u64(range), |p| {
        p.typography = retained.clone();
    });
    S_OK
}
unsafe extern "win64" fn tl_set_locale_name(this: PTextLayout, loc: *const u16, range: u64) -> HRESULT {
    if this.is_null() || loc.is_null() { return E_POINTER; }
    let s = read_wide_z(loc, DW_MAX_STR);
    ComObj::inner(this).state.lock().unwrap().apply_range(DWRITE_TEXT_RANGE::from_u64(range), |p| {
        p.locale_name = s.clone();
    });
    S_OK
}

// IDWriteTextLayout-specific getters (slots 42..57)
unsafe extern "win64" fn tl_get_max_width(this: PTextLayout) -> f32 {
    if this.is_null() { 0.0 } else { ComObj::inner(this).state.lock().unwrap().max_width }
}
unsafe extern "win64" fn tl_get_max_height(this: PTextLayout) -> f32 {
    if this.is_null() { 0.0 } else { ComObj::inner(this).state.lock().unwrap().max_height }
}
unsafe extern "win64" fn tl_get_font_collection(
    this: PTextLayout,
    pos: u32,
    out: *mut *mut c_void,
    range: *mut DWRITE_TEXT_RANGE,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let (p, r) = ComObj::inner(this).state.lock().unwrap().props_at(pos);
    out.write_unaligned(new_font_collection(p.collection));
    if !range.is_null() { range.write_unaligned(r); }
    S_OK
}
unsafe extern "win64" fn tl_get_family_len(
    this: PTextLayout,
    pos: u32,
    len: *mut u32,
    range: *mut DWRITE_TEXT_RANGE,
) -> HRESULT {
    if this.is_null() || len.is_null() { return E_POINTER; }
    let (p, r) = ComObj::inner(this).state.lock().unwrap().props_at(pos);
    len.write_unaligned(p.family_name.encode_utf16().count() as u32);
    if !range.is_null() { range.write_unaligned(r); }
    S_OK
}
unsafe extern "win64" fn tl_get_family_name(
    this: PTextLayout,
    pos: u32,
    buf: *mut u16,
    size: u32,
    range: *mut DWRITE_TEXT_RANGE,
) -> HRESULT {
    if this.is_null() || buf.is_null() { return E_POINTER; }
    let (p, r) = ComObj::inner(this).state.lock().unwrap().props_at(pos);
    let hr = write_wide_exact(&p.family_name, buf, size);
    if hr == S_OK && !range.is_null() { range.write_unaligned(r); }
    hr
}
unsafe extern "win64" fn tl_get_weight(
    this: PTextLayout,
    pos: u32,
    out: *mut u32,
    range: *mut DWRITE_TEXT_RANGE,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let (p, r) = ComObj::inner(this).state.lock().unwrap().props_at(pos);
    out.write_unaligned(p.weight);
    if !range.is_null() { range.write_unaligned(r); }
    S_OK
}
unsafe extern "win64" fn tl_get_style(
    this: PTextLayout,
    pos: u32,
    out: *mut u32,
    range: *mut DWRITE_TEXT_RANGE,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let (p, r) = ComObj::inner(this).state.lock().unwrap().props_at(pos);
    out.write_unaligned(p.style);
    if !range.is_null() { range.write_unaligned(r); }
    S_OK
}
unsafe extern "win64" fn tl_get_stretch(
    this: PTextLayout,
    pos: u32,
    out: *mut u32,
    range: *mut DWRITE_TEXT_RANGE,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let (p, r) = ComObj::inner(this).state.lock().unwrap().props_at(pos);
    out.write_unaligned(p.stretch);
    if !range.is_null() { range.write_unaligned(r); }
    S_OK
}
unsafe extern "win64" fn tl_get_font_size(
    this: PTextLayout,
    pos: u32,
    out: *mut f32,
    range: *mut DWRITE_TEXT_RANGE,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let (p, r) = ComObj::inner(this).state.lock().unwrap().props_at(pos);
    out.write_unaligned(p.font_size);
    if !range.is_null() { range.write_unaligned(r); }
    S_OK
}
pub(super) unsafe extern "win64" fn tl_get_underline(
    this: PTextLayout,
    pos: u32,
    out: *mut BOOL,
    range: *mut DWRITE_TEXT_RANGE,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let (p, r) = ComObj::inner(this).state.lock().unwrap().props_at(pos);
    out.write_unaligned(if p.underline { TRUE } else { FALSE });
    if !range.is_null() { range.write_unaligned(r); }
    S_OK
}
unsafe extern "win64" fn tl_get_strikethrough(
    this: PTextLayout,
    pos: u32,
    out: *mut BOOL,
    range: *mut DWRITE_TEXT_RANGE,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let (p, r) = ComObj::inner(this).state.lock().unwrap().props_at(pos);
    out.write_unaligned(if p.strikethrough { TRUE } else { FALSE });
    if !range.is_null() { range.write_unaligned(r); }
    S_OK
}
unsafe extern "win64" fn tl_get_drawing_effect(
    this: PTextLayout,
    pos: u32,
    out: *mut *mut c_void,
    range: *mut DWRITE_TEXT_RANGE,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let (p, r) = ComObj::inner(this).state.lock().unwrap().props_at(pos);
    if let Some(ref e) = p.drawing_effect {
        com_call_add_ref(e.as_ptr());
        out.write_unaligned(e.as_ptr());
    } else {
        write_null_out(out);
    }
    if !range.is_null() { range.write_unaligned(r); }
    S_OK
}
unsafe extern "win64" fn tl_get_inline_object(
    this: PTextLayout,
    pos: u32,
    out: *mut *mut c_void,
    range: *mut DWRITE_TEXT_RANGE,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let (p, r) = ComObj::inner(this).state.lock().unwrap().props_at(pos);
    if let Some(ref o) = p.inline_object {
        com_call_add_ref(o.as_ptr());
        out.write_unaligned(o.as_ptr());
    } else {
        write_null_out(out);
    }
    if !range.is_null() { range.write_unaligned(r); }
    S_OK
}
unsafe extern "win64" fn tl_get_typography(
    this: PTextLayout,
    pos: u32,
    out: *mut *mut c_void,
    range: *mut DWRITE_TEXT_RANGE,
) -> HRESULT {
    if this.is_null() || out.is_null() { return E_POINTER; }
    let (p, r) = ComObj::inner(this).state.lock().unwrap().props_at(pos);
    if let Some(ref t) = p.typography {
        com_call_add_ref(t.as_ptr());
        out.write_unaligned(t.as_ptr());
    } else {
        write_null_out(out);
    }
    if !range.is_null() { range.write_unaligned(r); }
    S_OK
}
unsafe extern "win64" fn tl_get_locale_len(
    this: PTextLayout,
    pos: u32,
    len: *mut u32,
    range: *mut DWRITE_TEXT_RANGE,
) -> HRESULT {
    if this.is_null() || len.is_null() { return E_POINTER; }
    let (p, r) = ComObj::inner(this).state.lock().unwrap().props_at(pos);
    len.write_unaligned(p.locale_name.encode_utf16().count() as u32);
    if !range.is_null() { range.write_unaligned(r); }
    S_OK
}
unsafe extern "win64" fn tl_get_locale_name(
    this: PTextLayout,
    pos: u32,
    buf: *mut u16,
    size: u32,
    range: *mut DWRITE_TEXT_RANGE,
) -> HRESULT {
    if this.is_null() || buf.is_null() { return E_POINTER; }
    let (p, r) = ComObj::inner(this).state.lock().unwrap().props_at(pos);
    let hr = write_wide_exact(&p.locale_name, buf, size);
    if hr == S_OK && !range.is_null() { range.write_unaligned(r); }
    hr
}

// Helper to build positioned drawable glyph runs from a computed layout.
pub(crate) struct DrawableGlyphRun {
    pub font_face: Arc<FontFaceInner>,
    pub em_size: f32,
    pub baseline_x: f32,
    pub baseline_y: f32,
    pub line_height: f32,
    pub glyph_indices: Vec<u16>,
    pub glyph_advances: Vec<f32>,
    pub glyph_offsets: Vec<DWRITE_GLYPH_OFFSET>,
    pub utf16_slice: Vec<u16>,
    pub cluster_map: Vec<u16>,
    pub text_position: u32,
    pub locale_wide: Vec<u16>,
    pub drawing_effect: *mut c_void,
    pub underline: Option<(f32, f32, f32)>,     // (width, offset, thickness)
    pub strikethrough: Option<(f32, f32, f32)>, // (width, offset, thickness)
}

fn build_drawable_runs(st: &mut TextLayoutState) -> Vec<DrawableGlyphRun> {
    st.ensure_layout();
    let layout = st.cached.as_ref().unwrap();
    let mut out = Vec::new();

    for line in &layout.lines {
        let cs = line.cluster_start;
        let ce = line.cluster_end;
        let mut idx = cs;
        while idx < ce {
            let c0 = &layout.clusters[idx];
            if c0.is_newline {
                idx += 1;
                continue;
            }
            let run_start = idx;
            let eff0 = c0.drawing_effect.as_ref().map_or(std::ptr::null_mut(), |p| p.as_ptr());
            let mut run_end = idx + 1;
            while run_end < ce {
                let cn = &layout.clusters[run_end];
                if cn.is_newline {
                    break;
                }
                let effn = cn.drawing_effect.as_ref().map_or(std::ptr::null_mut(), |p| p.as_ptr());
                if !Arc::ptr_eq(&cn.face, &c0.face)
                    || (cn.font_size - c0.font_size).abs() > 1e-4
                    || cn.underline != c0.underline
                    || cn.strikethrough != c0.strikethrough
                    || effn != eff0
                {
                    break;
                }
                run_end += 1;
            }

            let count = run_end - run_start;
            let mut gids = Vec::with_capacity(count);
            let mut advs = Vec::with_capacity(count);
            let mut offs = Vec::with_capacity(count);
            let mut cmap = Vec::new();
            let mut run_w = 0.0f32;
            let text_pos_start = layout.clusters[run_start].text_pos;
            let last_c = &layout.clusters[run_end - 1];
            let text_pos_end = (last_c.text_pos + last_c.utf16_len as u32) as usize;

            for (gi, ci) in (run_start..run_end).enumerate() {
                let c = &layout.clusters[ci];
                let w = line.cluster_widths[ci - cs];
                gids.push(c.gid);
                advs.push(w);
                offs.push(DWRITE_GLYPH_OFFSET::default());
                run_w += w;
                for _ in 0..c.utf16_len {
                    cmap.push(gi as u16);
                }
            }

            let u_start = (text_pos_start as usize).min(st.text.len());
            let u_end = text_pos_end.min(st.text.len());
            let utf16_slice = st.text[u_start..u_end].to_vec();
            let locale_wide: Vec<u16> = c0.locale_name.encode_utf16().chain(std::iter::once(0)).collect();

            let m = &c0.face.meta.metrics;
            let upem = m.designUnitsPerEm.max(1) as f32;
            let scale = c0.font_size / upem;
            let underline = if c0.underline {
                Some((run_w, -(m.underlinePosition as f32) * scale, (m.underlineThickness as f32 * scale).max(1.0)))
            } else {
                None
            };
            let strikethrough = if c0.strikethrough {
                Some((
                    run_w,
                    -(m.strikethroughPosition as f32) * scale,
                    (m.strikethroughThickness as f32 * scale).max(1.0),
                ))
            } else {
                None
            };

            out.push(DrawableGlyphRun {
                font_face: Arc::clone(&c0.face),
                em_size: c0.font_size,
                baseline_x: line.cluster_lefts[run_start - cs],
                baseline_y: line.top + line.baseline,
                line_height: line.height,
                glyph_indices: gids,
                glyph_advances: advs,
                glyph_offsets: offs,
                utf16_slice,
                cluster_map: cmap,
                text_position: text_pos_start,
                locale_wide,
                drawing_effect: eff0,
                underline,
                strikethrough,
            });
            idx = run_end;
        }
    }
    out
}

pub(crate) unsafe fn extract_layout_runs(layout_ptr: *mut c_void) -> Option<Vec<DrawableGlyphRun>> {
    let tl = com_from_iface::<TextLayoutInner>(layout_ptr, text_layout_vtbl())?;
    let mut st = tl.inner.state.lock().unwrap();
    Some(build_drawable_runs(&mut st))
}

pub(crate) unsafe fn layout_text_from_format(
    text: &[u16],
    format_ptr: *mut c_void,
    max_w: f32,
    max_h: f32,
) -> Option<Vec<DrawableGlyphRun>> {
    let fmt = text_format_state_from_ptr(format_ptr)?;
    let mut st = TextLayoutState::new(text.to_vec(), fmt, max_w, max_h);
    Some(build_drawable_runs(&mut st))
}

// Layout / Draw / Metrics / HitTesting (slots 58..66)
pub(super) unsafe extern "win64" fn tl_draw(
    this: PTextLayout,
    client_ctx: *mut c_void,
    renderer: *mut c_void,
    origin_x: f32,
    origin_y: f32,
) -> HRESULT {
    if this.is_null() || renderer.is_null() {
        return E_POINTER;
    }
    let vtbl = (renderer as *const *const usize).read_unaligned();
    if vtbl.is_null() {
        return E_INVALIDARG;
    }
    let fn_draw_glyph_run = vtbl.add(6).read_unaligned();
    let fn_draw_underline = vtbl.add(7).read_unaligned();
    let fn_draw_strikethrough = vtbl.add(8).read_unaligned();

    let (runs, reading_dir, flow_dir) = {
        let mut st = ComObj::inner(this).state.lock().unwrap();
        let rd = st.fmt.reading_direction;
        let fd = st.fmt.flow_direction;
        (build_drawable_runs(&mut st), rd, fd)
    };

    type DrawGlyphRunFn = unsafe extern "win64" fn(
        *mut c_void,
        *mut c_void,
        f32,
        f32,
        u32,
        *const DWRITE_GLYPH_RUN,
        *const DWRITE_GLYPH_RUN_DESCRIPTION,
        *mut c_void,
    ) -> HRESULT;
    type DrawUnderlineFn = unsafe extern "win64" fn(
        *mut c_void,
        *mut c_void,
        f32,
        f32,
        *const DWRITE_UNDERLINE,
        *mut c_void,
    ) -> HRESULT;
    type DrawStrikeFn = unsafe extern "win64" fn(
        *mut c_void,
        *mut c_void,
        f32,
        f32,
        *const DWRITE_STRIKETHROUGH,
        *mut c_void,
    ) -> HRESULT;

    for run in &runs {
        let face_com = new_font_face(Arc::clone(&run.font_face));
        let glyph_run = DWRITE_GLYPH_RUN {
            fontFace: face_com,
            fontEmSize: run.em_size,
            glyphCount: run.glyph_indices.len() as u32,
            glyphIndices: run.glyph_indices.as_ptr(),
            glyphAdvances: run.glyph_advances.as_ptr(),
            glyphOffsets: run.glyph_offsets.as_ptr(),
            isSideways: FALSE,
            bidiLevel: 0,
        };
        let desc = DWRITE_GLYPH_RUN_DESCRIPTION {
            localeName: run.locale_wide.as_ptr(),
            string: run.utf16_slice.as_ptr(),
            stringLength: run.utf16_slice.len() as u32,
            _pad0: 0,
            clusterMap: run.cluster_map.as_ptr(),
            textPosition: run.text_position,
            _pad1: 0,
        };
        let bx = origin_x + run.baseline_x;
        let by = origin_y + run.baseline_y;

        let hr = if fn_draw_glyph_run != 0 {
            let f: DrawGlyphRunFn = std::mem::transmute(fn_draw_glyph_run);
            f(
                renderer,
                client_ctx,
                bx,
                by,
                DWRITE_MEASURING_MODE_NATURAL,
                &glyph_run,
                &desc,
                run.drawing_effect,
            )
        } else {
            S_OK
        };
        com_call_release(face_com);
        if hr < 0 {
            return hr;
        }

        if let Some((w, off, thick)) = run.underline {
            if fn_draw_underline != 0 {
                let ul = DWRITE_UNDERLINE {
                    width: w,
                    thickness: thick,
                    offset: off,
                    runHeight: run.line_height,
                    readingDirection: reading_dir,
                    flowDirection: flow_dir,
                    localeName: run.locale_wide.as_ptr(),
                    measuringMode: DWRITE_MEASURING_MODE_NATURAL,
                    _pad: 0,
                };
                let f: DrawUnderlineFn = std::mem::transmute(fn_draw_underline);
                let hr2 = f(renderer, client_ctx, bx, by, &ul, run.drawing_effect);
                if hr2 < 0 {
                    return hr2;
                }
            }
        }
        if let Some((w, off, thick)) = run.strikethrough {
            if fn_draw_strikethrough != 0 {
                let sk = DWRITE_STRIKETHROUGH {
                    width: w,
                    thickness: thick,
                    offset: off,
                    runHeight: run.line_height,
                    readingDirection: reading_dir,
                    flowDirection: flow_dir,
                    localeName: run.locale_wide.as_ptr(),
                    measuringMode: DWRITE_MEASURING_MODE_NATURAL,
                    _pad: 0,
                };
                let f: DrawStrikeFn = std::mem::transmute(fn_draw_strikethrough);
                let hr2 = f(renderer, client_ctx, bx, by, &sk, run.drawing_effect);
                if hr2 < 0 {
                    return hr2;
                }
            }
        }
    }
    S_OK
}
crate::win64_bridge!(fn tl_draw(PTextLayout, *mut c_void, *mut c_void, f32, f32) -> HRESULT);

pub(super) unsafe extern "win64" fn tl_get_line_metrics(
    this: PTextLayout,
    line_metrics: *mut DWRITE_LINE_METRICS,
    max_count: u32,
    actual_count: *mut u32,
) -> HRESULT {
    if this.is_null() || actual_count.is_null() {
        return E_POINTER;
    }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    let layout = st.ensure_layout();
    let n = layout.lines.len() as u32;
    actual_count.write_unaligned(n);
    if max_count < n || line_metrics.is_null() {
        return E_NOT_SUFFICIENT_BUFFER;
    }
    for (i, line) in layout.lines.iter().enumerate() {
        line_metrics.add(i).write_unaligned(DWRITE_LINE_METRICS {
            length: line.utf16_len,
            trailingWhitespaceLength: line.trailing_ws_len,
            newlineLength: line.newline_len,
            height: line.height,
            baseline: line.baseline,
            isTrimmed: if line.is_trimmed { TRUE } else { FALSE },
        });
    }
    S_OK
}
pub(super) unsafe extern "win64" fn tl_get_metrics(this: PTextLayout, out: *mut DWRITE_TEXT_METRICS) -> HRESULT {
    if this.is_null() || out.is_null() {
        return E_POINTER;
    }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    let layout = st.ensure_layout();
    out.write_unaligned(layout.metrics);
    S_OK
}
pub(super) unsafe extern "win64" fn tl_get_overhang_metrics(
    this: PTextLayout,
    out: *mut DWRITE_OVERHANG_METRICS,
) -> HRESULT {
    if this.is_null() || out.is_null() {
        return E_POINTER;
    }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    let layout = st.ensure_layout();
    out.write_unaligned(layout.overhang);
    S_OK
}
pub(super) unsafe extern "win64" fn tl_get_cluster_metrics(
    this: PTextLayout,
    out: *mut DWRITE_CLUSTER_METRICS,
    max_count: u32,
    actual_count: *mut u32,
) -> HRESULT {
    if this.is_null() || actual_count.is_null() {
        return E_POINTER;
    }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    let layout = st.ensure_layout();
    let n = layout.clusters.len() as u32;
    actual_count.write_unaligned(n);
    if max_count < n || (n > 0 && out.is_null()) {
        return E_NOT_SUFFICIENT_BUFFER;
    }
    for (i, c) in layout.clusters.iter().enumerate() {
        let mut flags = 0u16;
        if c.can_wrap_after { flags |= 1 << 0; }
        if c.is_whitespace { flags |= 1 << 1; }
        if c.is_newline { flags |= 1 << 2; }
        out.add(i).write_unaligned(DWRITE_CLUSTER_METRICS {
            width: c.width,
            length: c.utf16_len,
            flags,
        });
    }
    S_OK
}
pub(super) unsafe extern "win64" fn tl_determine_min_width(this: PTextLayout, min_width: *mut f32) -> HRESULT {
    if this.is_null() || min_width.is_null() {
        return E_POINTER;
    }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    let layout = st.ensure_layout();
    min_width.write_unaligned(layout.min_width);
    S_OK
}
pub(super) unsafe extern "win64" fn tl_hit_test_point(
    this: PTextLayout,
    point_x: f32,
    point_y: f32,
    is_trailing_hit: *mut BOOL,
    is_inside: *mut BOOL,
    metrics: *mut DWRITE_HIT_TEST_METRICS,
) -> HRESULT {
    if this.is_null() || is_trailing_hit.is_null() || is_inside.is_null() || metrics.is_null() {
        return E_POINTER;
    }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    let layout = st.ensure_layout();

    let first_line = &layout.lines[0];
    let last_line = layout.lines.last().unwrap();
    let y_inside = point_y >= first_line.top && point_y < last_line.top + last_line.height;

    let line = if point_y < first_line.top {
        first_line
    } else if point_y >= last_line.top + last_line.height {
        last_line
    } else {
        layout
            .lines
            .iter()
            .find(|l| point_y >= l.top && point_y < l.top + l.height)
            .unwrap_or(last_line)
    };

    let cs = line.cluster_start;
    let ce = line.cluster_end;
    let mut vis_end = ce;
    while vis_end > cs && layout.clusters[vis_end - 1].is_newline {
        vis_end -= 1;
    }

    if cs == vis_end {
        is_trailing_hit.write_unaligned(FALSE);
        is_inside.write_unaligned(FALSE);
        metrics.write_unaligned(DWRITE_HIT_TEST_METRICS {
            textPosition: line.text_start,
            length: if cs < ce { layout.clusters[cs].utf16_len as u32 } else { 0 },
            left: line.left,
            top: line.top,
            width: 0.0,
            height: line.height,
            bidiLevel: 0,
            isText: TRUE,
            isTrimmed: if line.is_trimmed { TRUE } else { FALSE },
        });
        return S_OK;
    }

    let line_right = line.cluster_lefts[vis_end - 1 - cs] + line.cluster_widths[vis_end - 1 - cs];
    let x_inside = point_x >= line.left && point_x < line_right;
    is_inside.write_unaligned(if y_inside && x_inside { TRUE } else { FALSE });

    let (chosen_ci, trailing) = if point_x < line.left {
        (cs, false)
    } else if point_x >= line_right {
        (vis_end - 1, true)
    } else {
        let mut found = (vis_end - 1, true);
        for ci in cs..vis_end {
            let cl = line.cluster_lefts[ci - cs];
            let cw = line.cluster_widths[ci - cs];
            if point_x < cl + cw {
                found = (ci, point_x >= cl + cw * 0.5);
                break;
            }
        }
        found
    };

    let c = &layout.clusters[chosen_ci];
    is_trailing_hit.write_unaligned(if trailing { TRUE } else { FALSE });
    metrics.write_unaligned(DWRITE_HIT_TEST_METRICS {
        textPosition: c.text_pos,
        length: c.utf16_len as u32,
        left: line.cluster_lefts[chosen_ci - cs],
        top: line.top,
        width: line.cluster_widths[chosen_ci - cs],
        height: line.height,
        bidiLevel: 0,
        isText: TRUE,
        isTrimmed: if line.is_trimmed { TRUE } else { FALSE },
    });
    S_OK
}
crate::win64_bridge!(fn tl_hit_test_point(
    PTextLayout, f32, f32, *mut BOOL, *mut BOOL, *mut DWRITE_HIT_TEST_METRICS
) -> HRESULT);

pub(super) unsafe extern "win64" fn tl_hit_test_text_position(
    this: PTextLayout,
    text_pos: u32,
    is_trailing_hit: BOOL,
    point_x: *mut f32,
    point_y: *mut f32,
    metrics: *mut DWRITE_HIT_TEST_METRICS,
) -> HRESULT {
    if this.is_null() || point_x.is_null() || point_y.is_null() || metrics.is_null() {
        return E_POINTER;
    }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    let text_len = st.text.len() as u32;
    let layout = st.ensure_layout();

    for line in &layout.lines {
        let cs = line.cluster_start;
        let ce = line.cluster_end;
        for ci in cs..ce {
            let c = &layout.clusters[ci];
            if text_pos >= c.text_pos && text_pos < c.text_pos + c.utf16_len as u32 {
                let cl = line.cluster_lefts[ci - cs];
                let cw = line.cluster_widths[ci - cs];
                let px = if is_trailing_hit != 0 { cl + cw } else { cl };
                point_x.write_unaligned(px);
                point_y.write_unaligned(line.top);
                metrics.write_unaligned(DWRITE_HIT_TEST_METRICS {
                    textPosition: c.text_pos,
                    length: c.utf16_len as u32,
                    left: cl,
                    top: line.top,
                    width: cw,
                    height: line.height,
                    bidiLevel: 0,
                    isText: TRUE,
                    isTrimmed: if line.is_trimmed { TRUE } else { FALSE },
                });
                return S_OK;
            }
        }
    }

    // Past end of text (or empty text): return caret at end of last line.
    let last = layout.lines.last().unwrap();
    let end_x = last.left + last.width_with_trailing_ws;
    point_x.write_unaligned(end_x);
    point_y.write_unaligned(last.top);
    metrics.write_unaligned(DWRITE_HIT_TEST_METRICS {
        textPosition: text_len,
        length: 0,
        left: end_x,
        top: last.top,
        width: 0.0,
        height: last.height,
        bidiLevel: 0,
        isText: TRUE,
        isTrimmed: if last.is_trimmed { TRUE } else { FALSE },
    });
    S_OK
}

pub(super) unsafe extern "win64" fn tl_hit_test_text_range(
    this: PTextLayout,
    text_pos: u32,
    text_len: u32,
    origin_x: f32,
    origin_y: f32,
    out_metrics: *mut DWRITE_HIT_TEST_METRICS,
    max_count: u32,
    actual_count: *mut u32,
) -> HRESULT {
    if this.is_null() || actual_count.is_null() {
        return E_POINTER;
    }
    let mut st = ComObj::inner(this).state.lock().unwrap();
    let layout = st.ensure_layout();
    let range_end = text_pos.saturating_add(text_len);
    let mut rects: Vec<DWRITE_HIT_TEST_METRICS> = Vec::new();

    if text_len == 0 {
        // Zero-length range returns a single zero-width caret metric on the containing line.
        let last = layout.lines.last().unwrap();
        let mut found = DWRITE_HIT_TEST_METRICS {
            textPosition: text_pos,
            length: 0,
            left: origin_x + last.left + last.width_with_trailing_ws,
            top: origin_y + last.top,
            width: 0.0,
            height: last.height,
            bidiLevel: 0,
            isText: TRUE,
            isTrimmed: FALSE,
        };
        'outer: for line in &layout.lines {
            for ci in line.cluster_start..line.cluster_end {
                let c = &layout.clusters[ci];
                if text_pos >= c.text_pos && text_pos < c.text_pos + c.utf16_len as u32 {
                    found = DWRITE_HIT_TEST_METRICS {
                        textPosition: c.text_pos,
                        length: 0,
                        left: origin_x + line.cluster_lefts[ci - line.cluster_start],
                        top: origin_y + line.top,
                        width: 0.0,
                        height: line.height,
                        bidiLevel: 0,
                        isText: TRUE,
                        isTrimmed: if line.is_trimmed { TRUE } else { FALSE },
                    };
                    break 'outer;
                }
            }
        }
        rects.push(found);
    } else {
        for line in &layout.lines {
            let cs = line.cluster_start;
            let ce = line.cluster_end;
            let mut first_ci: Option<usize> = None;
            let mut last_ci: Option<usize> = None;
            for ci in cs..ce {
                let c = &layout.clusters[ci];
                let c_end = c.text_pos + c.utf16_len as u32;
                if c_end > text_pos && c.text_pos < range_end {
                    if first_ci.is_none() {
                        first_ci = Some(ci);
                    }
                    last_ci = Some(ci);
                }
            }
            if let (Some(fci), Some(lci)) = (first_ci, last_ci) {
                let left = line.cluster_lefts[fci - cs];
                let right = line.cluster_lefts[lci - cs] + line.cluster_widths[lci - cs];
                let start_pos = layout.clusters[fci].text_pos;
                let end_pos = layout.clusters[lci].text_pos + layout.clusters[lci].utf16_len as u32;
                rects.push(DWRITE_HIT_TEST_METRICS {
                    textPosition: start_pos,
                    length: end_pos - start_pos,
                    left: origin_x + left,
                    top: origin_y + line.top,
                    width: right - left,
                    height: line.height,
                    bidiLevel: 0,
                    isText: TRUE,
                    isTrimmed: if line.is_trimmed { TRUE } else { FALSE },
                });
            }
        }
    }

    let n = rects.len() as u32;
    actual_count.write_unaligned(n);
    if max_count < n || (n > 0 && out_metrics.is_null()) {
        return E_NOT_SUFFICIENT_BUFFER;
    }
    for (i, r) in rects.iter().enumerate() {
        out_metrics.add(i).write_unaligned(*r);
    }
    S_OK
}
crate::win64_bridge!(fn tl_hit_test_text_range(
    PTextLayout, u32, u32, f32, f32, *mut DWRITE_HIT_TEST_METRICS, u32, *mut u32
) -> HRESULT);

pub(super) fn text_layout_vtbl() -> *const usize {
    static V: OnceLock<[usize; 67]> = OnceLock::new();
    register_vtable_once(&V, || {
        [
            tl_qi as *const () as usize,                                // 0
            com_add_ref::<TextLayoutInner> as *const () as usize,       // 1
            com_release::<TextLayoutInner> as *const () as usize,       // 2
            tl_set_text_align as *const () as usize,                    // 3
            tl_set_para_align as *const () as usize,                    // 4
            tl_set_word_wrap as *const () as usize,                     // 5
            tl_set_reading_dir as *const () as usize,                   // 6
            tl_set_flow_dir as *const () as usize,                      // 7
            crate::bridge_entry!(tl_set_tab_stop) as usize,             // 8
            tl_set_trimming as *const () as usize,                      // 9
            crate::bridge_entry!(tl_set_line_spacing) as usize,         // 10
            tl_get_text_align as *const () as usize,                    // 11
            tl_get_para_align as *const () as usize,                    // 12
            tl_get_word_wrap as *const () as usize,                     // 13
            tl_get_reading_dir as *const () as usize,                   // 14
            tl_get_flow_dir as *const () as usize,                      // 15
            tl_get_tab_stop as *const () as usize,                      // 16
            tl_get_trimming as *const () as usize,                      // 17
            tl_get_line_spacing as *const () as usize,                  // 18
            tl_fmt_get_collection as *const () as usize,                // 19
            tl_fmt_get_family_len as *const () as usize,                // 20
            tl_fmt_get_family_name as *const () as usize,               // 21
            tl_fmt_get_weight as *const () as usize,                    // 22
            tl_fmt_get_style as *const () as usize,                     // 23
            tl_fmt_get_stretch as *const () as usize,                   // 24
            tl_fmt_get_font_size as *const () as usize,                 // 25
            tl_fmt_get_locale_len as *const () as usize,                // 26
            tl_fmt_get_locale_name as *const () as usize,               // 27
            crate::bridge_entry!(tl_set_max_width) as usize,            // 28
            crate::bridge_entry!(tl_set_max_height) as usize,           // 29
            tl_set_font_collection as *const () as usize,               // 30
            tl_set_font_family as *const () as usize,                   // 31
            tl_set_font_weight as *const () as usize,                   // 32
            tl_set_font_style as *const () as usize,                    // 33
            tl_set_font_stretch as *const () as usize,                  // 34
            crate::bridge_entry!(tl_set_font_size) as usize,            // 35
            tl_set_underline as *const () as usize,                     // 36
            tl_set_strikethrough as *const () as usize,                 // 37
            tl_set_drawing_effect as *const () as usize,                // 38
            tl_set_inline_object as *const () as usize,                 // 39
            tl_set_typography as *const () as usize,                    // 40
            tl_set_locale_name as *const () as usize,                   // 41
            tl_get_max_width as *const () as usize,                     // 42
            tl_get_max_height as *const () as usize,                    // 43
            tl_get_font_collection as *const () as usize,               // 44
            tl_get_family_len as *const () as usize,                    // 45
            tl_get_family_name as *const () as usize,                   // 46
            tl_get_weight as *const () as usize,                        // 47
            tl_get_style as *const () as usize,                         // 48
            tl_get_stretch as *const () as usize,                       // 49
            tl_get_font_size as *const () as usize,                     // 50
            tl_get_underline as *const () as usize,                     // 51
            tl_get_strikethrough as *const () as usize,                 // 52
            tl_get_drawing_effect as *const () as usize,                // 53
            tl_get_inline_object as *const () as usize,                 // 54
            tl_get_typography as *const () as usize,                    // 55
            tl_get_locale_len as *const () as usize,                    // 56
            tl_get_locale_name as *const () as usize,                   // 57
            crate::bridge_entry!(tl_draw) as usize,                     // 58
            tl_get_line_metrics as *const () as usize,                  // 59
            tl_get_metrics as *const () as usize,                       // 60
            tl_get_overhang_metrics as *const () as usize,              // 61
            tl_get_cluster_metrics as *const () as usize,               // 62
            tl_determine_min_width as *const () as usize,               // 63
            crate::bridge_entry!(tl_hit_test_point) as usize,           // 64
            tl_hit_test_text_position as *const () as usize,            // 65
            crate::bridge_entry!(tl_hit_test_text_range) as usize,      // 66
        ]
    })
}
