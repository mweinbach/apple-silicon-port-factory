//! Script-aware OpenType / AAT text shaping for `IDWriteTextAnalyzer`
//! backed by Apple CoreText on macOS and iOS.

use super::abi::{DWRITE_GLYPH_OFFSET, DWRITE_SHAPING_GLYPH_PROPERTIES, DWRITE_SHAPING_TEXT_PROPERTIES};
use super::font::FontFaceInner;
use super::sfnt::load_cached_file_bytes;
use std::sync::Arc;

#[derive(Clone, Copy, Debug)]
pub(super) struct ShapedGlyph {
    pub(super) gid: u16,
    pub(super) props: DWRITE_SHAPING_GLYPH_PROPERTIES,
    pub(super) advance: f32,
    pub(super) offset: DWRITE_GLYPH_OFFSET,
}

#[derive(Clone, Debug)]
pub(super) struct ShapedRun {
    pub(super) glyphs: Vec<ShapedGlyph>,
    pub(super) cluster_map: Vec<u16>,
    pub(super) text_props: Vec<DWRITE_SHAPING_TEXT_PROPERTIES>,
}

/// Raw OpenType feature tag (`u32` little-endian 4CC) + parameter value.
#[derive(Clone, Copy, Debug)]
pub(super) struct FeatureSetting {
    pub(super) tag: [u8; 4],
    pub(super) value: u32,
}

/// Per-range OpenType feature settings corresponding to one entry of
/// `IDWriteTextAnalyzer::GetGlyphs`'s `(features, featureRangeLengths, featureRanges)`.
#[derive(Clone, Debug)]
pub(super) struct FeatureRange {
    pub(super) start: usize,
    pub(super) len: usize,
    pub(super) settings: Vec<FeatureSetting>,
}

/// Decode `IDWriteTextAnalyzer`'s `features`, `featureRangeLengths`, and `featureRanges`
/// preserving each range's UTF-16 character span.
pub(super) unsafe fn read_feature_ranges(
    features: *const *const std::ffi::c_void,
    feature_range_lengths: *const u32,
    feature_ranges: u32,
    text_len: usize,
) -> Vec<FeatureRange> {
    if features.is_null() || feature_ranges == 0 || text_len == 0 {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut pos = 0usize;
    for r in 0..feature_ranges as usize {
        if pos >= text_len {
            break;
        }
        let range_len = if !feature_range_lengths.is_null() {
            feature_range_lengths.add(r).read_unaligned() as usize
        } else {
            text_len.saturating_sub(pos)
        };
        let end = pos.saturating_add(range_len).min(text_len);
        let fset_ptr = features.add(r).read_unaligned() as *const u8;
        if end > pos && !fset_ptr.is_null() {
            // DWRITE_TYPOGRAPHIC_FEATURES: { DWRITE_FONT_FEATURE* features (@0); UINT32 featureCount (@8); }
            let arr = (fset_ptr as *const *const u8).read_unaligned();
            let count = (fset_ptr.add(8) as *const u32).read_unaligned() as usize;
            if !arr.is_null() && count > 0 {
                let mut settings = Vec::with_capacity(count.min(64));
                for i in 0..count.min(64) {
                    let entry = arr.add(i * 8);
                    let tag_u32 = (entry as *const u32).read_unaligned();
                    let param = (entry.add(4) as *const u32).read_unaligned();
                    settings.push(FeatureSetting {
                        tag: tag_u32.to_le_bytes(),
                        value: param,
                    });
                }
                if !settings.is_empty() {
                    out.push(FeatureRange {
                        start: pos,
                        len: end - pos,
                        settings,
                    });
                }
            }
        }
        pos = end;
    }
    out
}

/// Returns `true` when `units` contains scripts, combining marks, join controls,
/// RTL direction, vertical/sideways orientation, or explicit OpenType features that
/// cannot be shaped correctly by a 1:1 `cmap` lookup without script-aware GSUB/GPOS shaping.
pub(super) fn needs_complex_shaping(
    units: &[u16],
    is_rtl: bool,
    is_sideways: bool,
    has_features: bool,
) -> bool {
    if is_rtl || is_sideways || has_features {
        return true;
    }
    let mut i = 0usize;
    while i < units.len() {
        let u = units[i];
        let cp = if (0xD800..=0xDBFF).contains(&u) && i + 1 < units.len() {
            let lo = units[i + 1];
            if (0xDC00..=0xDFFF).contains(&lo) {
                i += 2;
                0x10000 + (((u as u32 - 0xD800) << 10) | (lo as u32 - 0xDC00))
            } else {
                i += 1;
                u as u32
            }
        } else {
            i += 1;
            u as u32
        };
        let complex = matches!(
            cp,
            // Combining diacritical marks
            0x0300..=0x036F
                | 0x0483..=0x0489
                | 0x1AB0..=0x1AFF
                | 0x1DC0..=0x1DFF
                | 0x20D0..=0x20FF
                | 0xFE20..=0xFE2F
                // Hebrew, Arabic, Syriac, Thaana, NKo, Samaritan, Mandaic
                | 0x0590..=0x08FF
                // Devanagari, Bengali, Gurmukhi, Gujarati, Oriya, Tamil, Telugu, Kannada, Malayalam, Sinhala
                | 0x0900..=0x0DFF
                // Thai, Lao, Tibetan, Myanmar
                | 0x0E00..=0x109F
                // Khmer, Mongolian
                | 0x1780..=0x18AF
                // ZWJ / ZWNJ / bidi overrides / isolates
                | 0x200C..=0x200F
                | 0x202A..=0x202E
                | 0x2066..=0x2069
                // Arabic presentation forms / variation selectors
                | 0xFB1D..=0xFDFF
                | 0xFE00..=0xFE0F
                | 0xFE70..=0xFEFF
                | 0xE0100..=0xE01EF
        );
        if complex {
            return true;
        }
    }
    false
}

fn font_bytes_for_face(face: &FontFaceInner) -> Option<Arc<Vec<u8>>> {
    face.meta
        .raw_bytes
        .clone()
        .or_else(|| face.meta.file_path.as_ref().and_then(|p| load_cached_file_bytes(p)))
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
mod coretext {
    use super::*;
    use std::ffi::c_void;

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct CFRange {
        location: isize,
        length: isize,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct CGPoint {
        x: f64,
        y: f64,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct CGSize {
        width: f64,
        height: f64,
    }

    #[repr(C)]
    struct CTParagraphStyleSetting {
        spec: u32,
        value_size: usize,
        value: *const c_void,
    }

    const CF_STRING_ENCODING_UTF8: u32 = 0x0800_0100;
    const CF_NUMBER_SINT32_TYPE: isize = 3;
    const CT_PARAGRAPH_BASE_WRITING_DIRECTION: u32 = 13;
    const CT_WRITING_DIRECTION_LTR: i8 = 0;
    const CT_WRITING_DIRECTION_RTL: i8 = 1;
    const CT_RUN_STATUS_RIGHT_TO_LEFT: u32 = 1 << 0;
    const CT_FONT_ORIENTATION_VERTICAL: i32 = 2;

    #[link(name = "CoreFoundation", kind = "framework")]
    extern "C" {
        static kCFBooleanTrue: *const c_void;
        static kCFTypeArrayCallBacks: u8;
        static kCFTypeDictionaryKeyCallBacks: u8;
        static kCFTypeDictionaryValueCallBacks: u8;

        fn CFDataCreate(allocator: *const c_void, bytes: *const u8, length: isize) -> *const c_void;
        fn CFStringCreateWithCharacters(
            allocator: *const c_void,
            chars: *const u16,
            num_chars: isize,
        ) -> *const c_void;
        fn CFStringCreateWithBytes(
            allocator: *const c_void,
            bytes: *const u8,
            num_bytes: isize,
            encoding: u32,
            is_external: bool,
        ) -> *const c_void;
        fn CFNumberCreate(
            allocator: *const c_void,
            the_type: isize,
            value_ptr: *const c_void,
        ) -> *const c_void;
        fn CFArrayCreate(
            allocator: *const c_void,
            values: *const *const c_void,
            num_values: isize,
            callbacks: *const u8,
        ) -> *const c_void;
        fn CFArrayGetCount(the_array: *const c_void) -> isize;
        fn CFArrayGetValueAtIndex(the_array: *const c_void, idx: isize) -> *const c_void;
        fn CFDictionaryCreate(
            allocator: *const c_void,
            keys: *const *const c_void,
            values: *const *const c_void,
            num_values: isize,
            key_cb: *const u8,
            val_cb: *const u8,
        ) -> *const c_void;
        fn CFAttributedStringCreate(
            allocator: *const c_void,
            str_ref: *const c_void,
            attributes: *const c_void,
        ) -> *const c_void;
        fn CFAttributedStringCreateMutableCopy(
            allocator: *const c_void,
            max_length: isize,
            a_str: *const c_void,
        ) -> *mut c_void;
        fn CFAttributedStringSetAttribute(
            a_str: *mut c_void,
            range: CFRange,
            attr_name: *const c_void,
            value: *const c_void,
        );
        fn CFRelease(cf: *const c_void);
    }

    #[link(name = "CoreText", kind = "framework")]
    extern "C" {
        static kCTFontAttributeName: *const c_void;
        static kCTLanguageAttributeName: *const c_void;
        static kCTParagraphStyleAttributeName: *const c_void;
        static kCTVerticalFormsAttributeName: *const c_void;
        static kCTFontCascadeListAttribute: *const c_void;
        static kCTFontOrientationAttribute: *const c_void;
        static kCTFontFeatureSettingsAttribute: *const c_void;
        static kCTFontOpenTypeFeatureTag: *const c_void;
        static kCTFontOpenTypeFeatureValue: *const c_void;

        fn CTFontManagerCreateFontDescriptorsFromData(data: *const c_void) -> *const c_void;
        fn CTFontDescriptorCreateCopyWithAttributes(
            original: *const c_void,
            attributes: *const c_void,
        ) -> *const c_void;
        fn CTFontCreateWithFontDescriptor(
            descriptor: *const c_void,
            size: f64,
            matrix: *const c_void,
        ) -> *const c_void;
        fn CTParagraphStyleCreate(
            settings: *const CTParagraphStyleSetting,
            setting_count: usize,
        ) -> *const c_void;
        fn CTLineCreateWithAttributedString(attr_string: *const c_void) -> *const c_void;
        fn CTLineGetGlyphRuns(line: *const c_void) -> *const c_void;
        fn CTRunGetGlyphCount(run: *const c_void) -> isize;
        fn CTRunGetStatus(run: *const c_void) -> u32;
        fn CTRunGetGlyphs(run: *const c_void, range: CFRange, buffer: *mut u16);
        fn CTRunGetStringIndices(run: *const c_void, range: CFRange, buffer: *mut isize);
        fn CTRunGetAdvances(run: *const c_void, range: CFRange, buffer: *mut CGSize);
        fn CTRunGetPositions(run: *const c_void, range: CFRange, buffer: *mut CGPoint);
        fn CTFontGetVerticalTranslationsForGlyphs(
            font: *const c_void,
            glyphs: *const u16,
            translations: *mut CGSize,
            count: isize,
        );
    }

    struct CfGuard(*const c_void);
    impl Drop for CfGuard {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe { CFRelease(self.0) };
            }
        }
    }

    unsafe fn make_ct_font(
        base_desc: *const c_void,
        empty_cascade: *const c_void,
        ct_size: f64,
        is_sideways: bool,
        settings: &[FeatureSetting],
    ) -> Option<CfGuard> {
        let mut feat_dicts: Vec<CfGuard> = Vec::new();
        let mut feat_ptrs: Vec<*const c_void> = Vec::new();
        for f in settings {
            let tag_str = CfGuard(CFStringCreateWithBytes(
                std::ptr::null(),
                f.tag.as_ptr(),
                4,
                CF_STRING_ENCODING_UTF8,
                false,
            ));
            let val_i32 = f.value as i32;
            let val_num = CfGuard(CFNumberCreate(
                std::ptr::null(),
                CF_NUMBER_SINT32_TYPE,
                &val_i32 as *const i32 as *const c_void,
            ));
            if tag_str.0.is_null() || val_num.0.is_null() {
                continue;
            }
            let fkeys = [kCTFontOpenTypeFeatureTag, kCTFontOpenTypeFeatureValue];
            let fvals = [tag_str.0, val_num.0];
            let fdict = CfGuard(CFDictionaryCreate(
                std::ptr::null(),
                fkeys.as_ptr(),
                fvals.as_ptr(),
                2,
                &kCFTypeDictionaryKeyCallBacks as *const u8,
                &kCFTypeDictionaryValueCallBacks as *const u8,
            ));
            if !fdict.0.is_null() {
                feat_ptrs.push(fdict.0);
                feat_dicts.push(fdict);
            }
        }
        let feat_arr = if !feat_ptrs.is_empty() {
            Some(CfGuard(CFArrayCreate(
                std::ptr::null(),
                feat_ptrs.as_ptr(),
                feat_ptrs.len() as isize,
                &kCFTypeArrayCallBacks as *const u8,
            )))
        } else {
            None
        };
        let orient_num = if is_sideways {
            let orient = CT_FONT_ORIENTATION_VERTICAL;
            Some(CfGuard(CFNumberCreate(
                std::ptr::null(),
                CF_NUMBER_SINT32_TYPE,
                &orient as *const i32 as *const c_void,
            )))
        } else {
            None
        };

        let mut desc_keys = vec![kCTFontCascadeListAttribute];
        let mut desc_vals = vec![empty_cascade];
        if let Some(ref onum) = orient_num {
            if !onum.0.is_null() {
                desc_keys.push(kCTFontOrientationAttribute);
                desc_vals.push(onum.0);
            }
        }
        if let Some(ref fa) = feat_arr {
            if !fa.0.is_null() {
                desc_keys.push(kCTFontFeatureSettingsAttribute);
                desc_vals.push(fa.0);
            }
        }
        let desc_attrs = CfGuard(CFDictionaryCreate(
            std::ptr::null(),
            desc_keys.as_ptr(),
            desc_vals.as_ptr(),
            desc_keys.len() as isize,
            &kCFTypeDictionaryKeyCallBacks as *const u8,
            &kCFTypeDictionaryValueCallBacks as *const u8,
        ));
        let tuned_desc = if !desc_attrs.0.is_null() {
            CfGuard(CTFontDescriptorCreateCopyWithAttributes(base_desc, desc_attrs.0))
        } else {
            CfGuard(std::ptr::null())
        };
        let use_desc = if !tuned_desc.0.is_null() { tuned_desc.0 } else { base_desc };
        let ct_font = CfGuard(CTFontCreateWithFontDescriptor(
            use_desc,
            ct_size,
            std::ptr::null(),
        ));
        (!ct_font.0.is_null()).then_some(ct_font)
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn shape_run(
        face: &FontFaceInner,
        units: &[u16],
        em_size: f32,
        is_rtl: bool,
        is_sideways: bool,
        no_visual: bool,
        locale: Option<&str>,
        feature_ranges: &[FeatureRange],
    ) -> Option<ShapedRun> {
        if units.is_empty() {
            return Some(ShapedRun {
                glyphs: Vec::new(),
                cluster_map: Vec::new(),
                text_props: Vec::new(),
            });
        }
        let bytes = font_bytes_for_face(face)?;
        let ct_size = if em_size > 0.0 && em_size.is_finite() {
            em_size as f64
        } else {
            face.meta.metrics.designUnitsPerEm.max(1) as f64
        };
        let scale_back = if em_size == 0.0 { 0.0f32 } else { 1.0f32 };

        unsafe {
            let cf_data = CfGuard(CFDataCreate(
                std::ptr::null(),
                bytes.as_ptr(),
                bytes.len() as isize,
            ));
            if cf_data.0.is_null() {
                return None;
            }
            let descs = CfGuard(CTFontManagerCreateFontDescriptorsFromData(cf_data.0));
            if descs.0.is_null() {
                return None;
            }
            let desc_count = CFArrayGetCount(descs.0);
            if desc_count <= 0 {
                return None;
            }
            let idx = (face.meta.face_index as isize).min(desc_count - 1);
            let base_desc = CFArrayGetValueAtIndex(descs.0, idx);
            if base_desc.is_null() {
                return None;
            }

            // Disable CoreText font fallback so every glyph belongs strictly to this IDWriteFontFace.
            let empty_cascade = CfGuard(CFArrayCreate(
                std::ptr::null(),
                std::ptr::null(),
                0,
                &kCFTypeArrayCallBacks as *const u8,
            ));
            if empty_cascade.0.is_null() {
                return None;
            }

            let base_ct_font = make_ct_font(base_desc, empty_cascade.0, ct_size, is_sideways, &[])?;

            let dir: i8 = if is_rtl { CT_WRITING_DIRECTION_RTL } else { CT_WRITING_DIRECTION_LTR };
            let psetting = CTParagraphStyleSetting {
                spec: CT_PARAGRAPH_BASE_WRITING_DIRECTION,
                value_size: std::mem::size_of::<i8>(),
                value: &dir as *const i8 as *const c_void,
            };
            let pstyle = CfGuard(CTParagraphStyleCreate(&psetting, 1));
            if pstyle.0.is_null() {
                return None;
            }

            let cf_str = CfGuard(CFStringCreateWithCharacters(
                std::ptr::null(),
                units.as_ptr(),
                units.len() as isize,
            ));
            if cf_str.0.is_null() {
                return None;
            }
            let cf_locale = locale
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .and_then(|loc| {
                    let g = CfGuard(CFStringCreateWithBytes(
                        std::ptr::null(),
                        loc.as_ptr(),
                        loc.len() as isize,
                        CF_STRING_ENCODING_UTF8,
                        false,
                    ));
                    (!g.0.is_null()).then_some(g)
                });
            let mut attr_keys = Vec::with_capacity(4);
            let mut attr_vals = Vec::with_capacity(4);
            attr_keys.push(kCTFontAttributeName);
            attr_vals.push(base_ct_font.0);
            attr_keys.push(kCTParagraphStyleAttributeName);
            attr_vals.push(pstyle.0);
            if is_sideways {
                attr_keys.push(kCTVerticalFormsAttributeName);
                attr_vals.push(kCFBooleanTrue);
            }
            if let Some(ref loc) = cf_locale {
                attr_keys.push(kCTLanguageAttributeName);
                attr_vals.push(loc.0);
            }
            let attrs = CfGuard(CFDictionaryCreate(
                std::ptr::null(),
                attr_keys.as_ptr(),
                attr_vals.as_ptr(),
                attr_keys.len() as isize,
                &kCFTypeDictionaryKeyCallBacks as *const u8,
                &kCFTypeDictionaryValueCallBacks as *const u8,
            ));
            if attrs.0.is_null() {
                return None;
            }
            let base_astr = CfGuard(CFAttributedStringCreate(
                std::ptr::null(),
                cf_str.0,
                attrs.0,
            ));
            if base_astr.0.is_null() {
                return None;
            }

            // Apply each DWRITE_TYPOGRAPHIC_FEATURES range strictly to its UTF-16 character span.
            let mut range_fonts: Vec<CfGuard> = Vec::new();
            let mut_astr = if !feature_ranges.is_empty() {
                let m = CfGuard(CFAttributedStringCreateMutableCopy(
                    std::ptr::null(),
                    units.len() as isize,
                    base_astr.0,
                ) as *const c_void);
                if m.0.is_null() {
                    return None;
                }
                for fr in feature_ranges {
                    if fr.start >= units.len() || fr.len == 0 || fr.settings.is_empty() {
                        continue;
                    }
                    let clamped_len = fr.len.min(units.len() - fr.start);
                    if let Some(rf) = make_ct_font(
                        base_desc,
                        empty_cascade.0,
                        ct_size,
                        is_sideways,
                        &fr.settings,
                    ) {
                        CFAttributedStringSetAttribute(
                            m.0 as *mut c_void,
                            CFRange {
                                location: fr.start as isize,
                                length: clamped_len as isize,
                            },
                            kCTFontAttributeName,
                            rf.0,
                        );
                        range_fonts.push(rf);
                    }
                }
                Some(m)
            } else {
                None
            };
            let _keep_range_fonts = &range_fonts;
            let final_astr = mut_astr.as_ref().map(|g| g.0).unwrap_or(base_astr.0);
            let line = CfGuard(CTLineCreateWithAttributedString(final_astr));
            if line.0.is_null() {
                return None;
            }

            let runs_arr = CTLineGetGlyphRuns(line.0);
            if runs_arr.is_null() {
                return None;
            }
            let run_count = CFArrayGetCount(runs_arr);

            // Extract every CTRun in visual left-to-right order, computing exact
            // DirectWrite (advance, advanceOffset, ascenderOffset) per glyph before
            // reordering runs into DirectWrite logical order.
            struct RawRun {
                min_str_idx: usize,
                items: Vec<(usize, u16, f32, DWRITE_GLYPH_OFFSET)>, // (str_idx, gid, adv, off)
            }

            let mut parsed_runs: Vec<RawRun> = Vec::with_capacity(run_count.max(0) as usize);
            let mut line_pen_x = 0.0f64;

            for ri in 0..run_count {
                let run = CFArrayGetValueAtIndex(runs_arr, ri);
                if run.is_null() {
                    continue;
                }
                let gc = CTRunGetGlyphCount(run);
                if gc <= 0 {
                    continue;
                }
                let n = gc as usize;
                let range = CFRange { location: 0, length: gc };
                let mut gids = vec![0u16; n];
                let mut indices = vec![0isize; n];
                let mut advances = vec![CGSize { width: 0.0, height: 0.0 }; n];
                let mut positions = vec![CGPoint { x: 0.0, y: 0.0 }; n];
                CTRunGetGlyphs(run, range, gids.as_mut_ptr());
                CTRunGetStringIndices(run, range, indices.as_mut_ptr());
                CTRunGetAdvances(run, range, advances.as_mut_ptr());
                CTRunGetPositions(run, range, positions.as_mut_ptr());

                let run_rtl = (CTRunGetStatus(run) & CT_RUN_STATUS_RIGHT_TO_LEFT) != 0;
                let mut items = Vec::with_capacity(n);
                let mut min_str_idx = usize::MAX;

                if is_sideways {
                    // In CoreText vertical layout, glyph positions advance downward along `-Y`
                    // and include each glyph's horizontal-to-vertical origin translation `vtr`.
                    // Subtracting `vtr` yields vertical-origin coordinates `vpos`, where
                    // `vis_adv = -vpos.y` increases along DirectWrite's advance axis and
                    // `vpos.x` lies along DirectWrite's perpendicular ascender axis.
                    // CoreText's `CTRunGetAdvances[0]` also folds `vpos[0].y` (leading vertical
                    // placement such as `vpal` top trim) into the first advance, so remove the
                    // leading offset delta from glyph 0 so `sum(adv)` matches the run's true
                    // typographic advance while `pen + advanceOffset` hits `vis_adv[v]` exactly.
                    let mut vtr = vec![CGSize { width: 0.0, height: 0.0 }; n];
                    CTFontGetVerticalTranslationsForGlyphs(
                        base_ct_font.0,
                        gids.as_ptr(),
                        vtr.as_mut_ptr(),
                        gc,
                    );
                    let vis_advs: Vec<f64> = (0..n).map(|v| -(positions[v].y - vtr[v].height)).collect();
                    let run_end = vis_advs[n - 1] + advances[n - 1].width;
                    let mut logical_pen = 0.0f64;

                    if run_rtl {
                        for v in (0..n).rev() {
                            let str_idx = (indices[v].max(0) as usize).min(units.len() - 1);
                            min_str_idx = min_str_idx.min(str_idx);
                            let raw_adv = if v == 0 {
                                advances[0].width + (vis_advs[0] - line_pen_x)
                            } else {
                                advances[v].width
                            };
                            let adv = if no_visual { 0.0 } else { raw_adv.max(0.0) };
                            let expected_vis_adv = run_end - logical_pen - adv;
                            let adv_off = if no_visual {
                                0.0
                            } else {
                                (expected_vis_adv - vis_advs[v]) as f32
                            };
                            let asc_off = if no_visual {
                                0.0
                            } else {
                                (positions[v].x - vtr[v].width) as f32
                            };
                            logical_pen += adv;
                            items.push((
                                str_idx,
                                gids[v],
                                (adv as f32) * scale_back,
                                DWRITE_GLYPH_OFFSET {
                                    advanceOffset: adv_off * scale_back,
                                    ascenderOffset: asc_off * scale_back,
                                },
                            ));
                        }
                    } else {
                        for v in 0..n {
                            let str_idx = (indices[v].max(0) as usize).min(units.len() - 1);
                            min_str_idx = min_str_idx.min(str_idx);
                            let raw_adv = if v == 0 {
                                advances[0].width + (vis_advs[0] - line_pen_x)
                            } else {
                                advances[v].width
                            };
                            let adv = if no_visual { 0.0 } else { raw_adv.max(0.0) };
                            let expected_vis_adv = line_pen_x + logical_pen;
                            let adv_off = if no_visual {
                                0.0
                            } else {
                                (vis_advs[v] - expected_vis_adv) as f32
                            };
                            let asc_off = if no_visual {
                                0.0
                            } else {
                                (positions[v].x - vtr[v].width) as f32
                            };
                            logical_pen += adv;
                            items.push((
                                str_idx,
                                gids[v],
                                (adv as f32) * scale_back,
                                DWRITE_GLYPH_OFFSET {
                                    advanceOffset: adv_off * scale_back,
                                    ascenderOffset: asc_off * scale_back,
                                },
                            ));
                        }
                    }
                    line_pen_x += logical_pen;
                } else if run_rtl {
                    // Visual order is right-to-left reversed; walk from right to left to recover
                    // DirectWrite logical order with exact RTL advanceOffset polarity.
                    let run_adv: f64 = advances.iter().map(|a| a.width).sum();
                    let run_right_x = line_pen_x + run_adv;
                    let mut logical_pen = 0.0f64;
                    for v in (0..n).rev() {
                        let str_idx = (indices[v].max(0) as usize).min(units.len() - 1);
                        min_str_idx = min_str_idx.min(str_idx);
                        let adv = if no_visual { 0.0 } else { advances[v].width };
                        let expected_vis_x = run_right_x - logical_pen - adv;
                        let adv_off = if no_visual { 0.0 } else { (expected_vis_x - positions[v].x) as f32 };
                        let asc_off = if no_visual { 0.0 } else { positions[v].y as f32 };
                        logical_pen += adv;
                        items.push((
                            str_idx,
                            gids[v],
                            (adv as f32) * scale_back,
                            DWRITE_GLYPH_OFFSET {
                                advanceOffset: adv_off * scale_back,
                                ascenderOffset: asc_off * scale_back,
                            },
                        ));
                    }
                    line_pen_x += run_adv;
                } else {
                    let run_adv: f64 = advances.iter().map(|a| a.width).sum();
                    let mut logical_pen = 0.0f64;
                    for v in 0..n {
                        let str_idx = (indices[v].max(0) as usize).min(units.len() - 1);
                        min_str_idx = min_str_idx.min(str_idx);
                        let adv = if no_visual { 0.0 } else { advances[v].width };
                        let expected_vis_x = line_pen_x + logical_pen;
                        let adv_off = if no_visual { 0.0 } else { (positions[v].x - expected_vis_x) as f32 };
                        let asc_off = if no_visual { 0.0 } else { positions[v].y as f32 };
                        logical_pen += adv;
                        items.push((
                            str_idx,
                            gids[v],
                            (adv as f32) * scale_back,
                            DWRITE_GLYPH_OFFSET {
                                advanceOffset: adv_off * scale_back,
                                ascenderOffset: asc_off * scale_back,
                            },
                        ));
                    }
                    line_pen_x += run_adv;
                }

                parsed_runs.push(RawRun { min_str_idx, items });
            }

            if parsed_runs.is_empty() {
                return None;
            }
            parsed_runs.sort_by_key(|r| r.min_str_idx);

            let mut flat: Vec<(usize, u16, f32, DWRITE_GLYPH_OFFSET)> = Vec::new();
            for r in parsed_runs {
                flat.extend(r.items);
            }
            if flat.is_empty() {
                return None;
            }

            // Build monotonic logical clusterMap and shaping properties.
            // CoreText does not expose per-character independent-shaping provenance, so keep
            // `DWRITE_SHAPING_TEXT_PROPERTIES::isShapedAlone` conservatively clear rather than
            // conflating it with cluster starts (which live in `cluster_map` and `gprops`).
            let mut cluster_map = vec![0u16; units.len()];
            let text_props = vec![DWRITE_SHAPING_TEXT_PROPERTIES { bits: 0 }; units.len()];
            let mut glyphs = Vec::with_capacity(flat.len());

            // Identify cluster boundaries (first glyph for each distinct str_idx).
            let mut clusters: Vec<(usize, usize)> = Vec::new(); // (str_idx, first_glyph_idx)
            for (gi, &(str_idx, gid, adv, offset)) in flat.iter().enumerate() {
                let is_cluster_start = clusters.last().is_none_or(|&(prev_s, _)| str_idx > prev_s);
                if is_cluster_start {
                    clusters.push((str_idx, gi));
                }
                let ch_u16 = units[str_idx];
                let is_space = ch_u16 == b' ' as u16;
                let is_zws = no_visual || ch_u16 == 0x200B;
                let is_diacritic = !is_cluster_start && adv == 0.0 && !is_zws;
                let justification: u16 = if is_space { 2 } else { 1 };
                let bits = (justification & 0xF)
                    | ((is_cluster_start as u16) << 4)
                    | ((is_diacritic as u16) << 5)
                    | ((is_zws as u16) << 6);
                glyphs.push(ShapedGlyph {
                    gid,
                    props: DWRITE_SHAPING_GLYPH_PROPERTIES { bits },
                    advance: adv,
                    offset,
                });
            }

            for ci in 0..clusters.len() {
                let (start_u16, first_gi) = clusters[ci];
                let end_u16 = if ci + 1 < clusters.len() {
                    clusters[ci + 1].0.max(start_u16)
                } else {
                    units.len()
                };
                // Cover any leading code units before cluster 0 too.
                let fill_from = if ci == 0 { 0 } else { start_u16 };
                for u in fill_from..end_u16.min(units.len()) {
                    cluster_map[u] = first_gi as u16;
                }
            }

            Some(ShapedRun {
                glyphs,
                cluster_map,
                text_props,
            })
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn shape_run_platform(
    face: &FontFaceInner,
    units: &[u16],
    em_size: f32,
    is_rtl: bool,
    is_sideways: bool,
    no_visual: bool,
    locale: Option<&str>,
    feature_ranges: &[FeatureRange],
) -> Option<ShapedRun> {
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        coretext::shape_run(
            face,
            units,
            em_size,
            is_rtl,
            is_sideways,
            no_visual,
            locale,
            feature_ranges,
        )
    }
    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
    {
        let _ = (face, units, em_size, is_rtl, is_sideways, no_visual, locale, feature_ranges);
        None
    }
}
