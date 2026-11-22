//! HID report-descriptor parser producing the Windows "preparsed data" view
//! (`HIDP_CAPS`, `HIDP_VALUE_CAPS`, `HIDP_BUTTON_CAPS` counts).
//!
//! The rules follow Windows hidparse (as documented and as reimplemented by
//! Wine's hidparse.sys):
//!
//! * `XxxReportByteLength` = the longest report of that type in bytes
//!   **plus one** for the report-ID byte, which is counted even when the
//!   device does not use report IDs. 0 when there are no reports of the type.
//! * A main item becomes **button** caps when its report size is 1 bit or it
//!   is an array (`Data,Array`); otherwise **value** caps.
//! * Constant main items without usages (padding) produce no caps.
//! * A usage range gives one `IsRange` cap. A usage list gives one cap per
//!   usage; for values, when there are fewer usages than `ReportCount`, the
//!   last usage covers the remaining fields (`ReportCount` > 1).
//! * Data indices are numbered per report type in cap order: one per usage
//!   (a value array with one usage has one data index).
//! * Link collection 0 is the top-level collection; every nested
//!   `Collection` adds a node.
//!
//! Only the first top-level collection's usage is reported as the device
//! usage; multi-TLC devices (which Windows splits into several device
//! interfaces) are treated as one.

use std::collections::HashMap;

pub const HIDP_INPUT: usize = 0;
pub const HIDP_OUTPUT: usize = 1;
pub const HIDP_FEATURE: usize = 2;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ValueCap {
    pub usage_page: u16,
    pub report_id: u8,
    pub bit_field: u16,
    pub link_collection: u16,
    pub link_usage: u16,
    pub link_usage_page: u16,
    pub is_range: bool,
    pub is_absolute: bool,
    pub has_null: bool,
    pub bit_size: u16,
    pub report_count: u16,
    pub units_exp: u32,
    pub units: u32,
    pub logical_min: i32,
    pub logical_max: i32,
    pub physical_min: i32,
    pub physical_max: i32,
    /// `NotRange.Usage` when `!is_range`.
    pub usage_min: u16,
    pub usage_max: u16,
    pub string_min: u16,
    pub string_max: u16,
    pub designator_min: u16,
    pub designator_max: u16,
    pub data_index_min: u16,
    pub data_index_max: u16,
}

/// Button caps share the value-cap fields that matter here.
pub type ButtonCap = ValueCap;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LinkNode {
    pub usage: u16,
    pub usage_page: u16,
    pub parent: u16,
    pub collection_type: u8,
}

#[derive(Clone, Debug, Default)]
pub struct Caps {
    pub usage: u16,
    pub usage_page: u16,
    /// Input / output / feature report byte lengths (including the ID byte).
    pub report_len: [u16; 3],
    pub link_nodes: Vec<LinkNode>,
    pub values: [Vec<ValueCap>; 3],
    pub buttons: [Vec<ButtonCap>; 3],
    pub data_indices: [u16; 3],
    /// True when any report carries a report ID.
    pub uses_report_ids: bool,
    /// Payload bits per (type, report id), for diagnostics and tests.
    pub report_bits: HashMap<(usize, u8), u32>,
}

#[derive(Clone, Default)]
struct Globals {
    usage_page: u16,
    logical_min: i32,
    logical_max: i32,
    /// Raw logical max and its size, for the unsigned reinterpretation.
    logical_max_raw: (u32, usize),
    physical_min: i32,
    physical_max: i32,
    unit_exp: u32,
    unit: u32,
    report_size: u32,
    report_id: u8,
    report_count: u32,
}

#[derive(Default)]
struct Locals {
    /// (page or 0 for "current page", usage)
    usages: Vec<(u16, u16)>,
    usage_min: Option<(u16, u16)>,
    usage_max: Option<(u16, u16)>,
    string_index: u16,
    string_min: u16,
    string_max: u16,
    designator_index: u16,
    designator_min: u16,
    designator_max: u16,
}

fn sext(v: u32, size: usize) -> i32 {
    match size {
        1 => v as u8 as i8 as i32,
        2 => v as u16 as i16 as i32,
        _ => v as i32,
    }
}

pub fn parse(desc: &[u8]) -> Result<Caps, String> {
    let mut caps = Caps::default();
    let mut g = Globals::default();
    let mut stack: Vec<Globals> = Vec::new();
    let mut l = Locals::default();
    // Link collection stack (indices into caps.link_nodes).
    let mut coll: Vec<u16> = Vec::new();
    let mut bits: HashMap<(usize, u8), u32> = HashMap::new();
    let mut seen_tlc = false;
    let mut i = 0usize;
    while i < desc.len() {
        let prefix = desc[i];
        i += 1;
        if prefix == 0xFE {
            // Long item: skip.
            if i + 2 > desc.len() {
                return Err("truncated long item".into());
            }
            let n = desc[i] as usize;
            i += 2 + n;
            continue;
        }
        let size = match prefix & 3 {
            3 => 4,
            s => s as usize,
        };
        if i + size > desc.len() {
            return Err(format!("truncated item at {}", i - 1));
        }
        let mut v = 0u32;
        for k in 0..size {
            v |= (desc[i + k] as u32) << (8 * k);
        }
        i += size;
        let kind = (prefix >> 2) & 3;
        let tag = prefix >> 4;
        match kind {
            0 => {
                // Main
                match tag {
                    0x8 | 0x9 | 0xB => {
                        let t = match tag {
                            0x8 => HIDP_INPUT,
                            0x9 => HIDP_OUTPUT,
                            _ => HIDP_FEATURE,
                        };
                        main_item(&mut caps, &g, &l, &coll, &mut bits, t, v);
                        l = Locals::default();
                    }
                    0xA => {
                        // Collection
                        let (page, usage) = l
                            .usages
                            .first()
                            .map(|&(p, u)| (if p == 0 { g.usage_page } else { p }, u))
                            .or(l.usage_min.map(|(p, u)| (if p == 0 { g.usage_page } else { p }, u)))
                            .unwrap_or((g.usage_page, 0));
                        if coll.is_empty() {
                            if !seen_tlc {
                                seen_tlc = true;
                                caps.usage = usage;
                                caps.usage_page = page;
                                caps.link_nodes.push(LinkNode { usage, usage_page: page, parent: 0, collection_type: v as u8 });
                            }
                            coll.push(0);
                        } else {
                            let parent = *coll.last().unwrap();
                            caps.link_nodes.push(LinkNode { usage, usage_page: page, parent, collection_type: v as u8 });
                            coll.push((caps.link_nodes.len() - 1) as u16);
                        }
                        l = Locals::default();
                    }
                    0xC => {
                        // End Collection
                        if coll.pop().is_none() {
                            return Err("unbalanced End Collection".into());
                        }
                        l = Locals::default();
                    }
                    _ => {}
                }
            }
            1 => {
                // Global
                match tag {
                    0x0 => g.usage_page = v as u16,
                    0x1 => g.logical_min = sext(v, size),
                    0x2 => {
                        g.logical_max = sext(v, size);
                        g.logical_max_raw = (v, size);
                    }
                    0x3 => g.physical_min = sext(v, size),
                    0x4 => g.physical_max = sext(v, size),
                    0x5 => g.unit_exp = v,
                    0x6 => g.unit = v,
                    0x7 => g.report_size = v,
                    0x8 => {
                        g.report_id = v as u8;
                        caps.uses_report_ids = true;
                    }
                    0x9 => g.report_count = v,
                    0xA => stack.push(g.clone()),
                    0xB => g = stack.pop().ok_or("Pop without Push")?,
                    _ => {}
                }
            }
            2 => {
                // Local
                let split = |v: u32| -> (u16, u16) {
                    if size == 4 {
                        ((v >> 16) as u16, v as u16)
                    } else {
                        (0, v as u16)
                    }
                };
                match tag {
                    0x0 => l.usages.push(split(v)),
                    0x1 => l.usage_min = Some(split(v)),
                    0x2 => l.usage_max = Some(split(v)),
                    0x3 => l.designator_index = v as u16,
                    0x4 => l.designator_min = v as u16,
                    0x5 => l.designator_max = v as u16,
                    0x7 => l.string_index = v as u16,
                    0x8 => l.string_min = v as u16,
                    0x9 => l.string_max = v as u16,
                    _ => {}
                }
            }
            _ => {}
        }
    }
    for t in 0..3 {
        let longest = bits.iter().filter(|((tt, _), _)| *tt == t).map(|(_, &b)| b.div_ceil(8)).max();
        caps.report_len[t] = longest.map(|b| b as u16 + 1).unwrap_or(0);
    }
    if caps.link_nodes.is_empty() {
        caps.link_nodes.push(LinkNode::default());
    }
    caps.report_bits = bits;
    Ok(caps)
}

fn main_item(
    caps: &mut Caps,
    g: &Globals,
    l: &Locals,
    coll: &[u16],
    bits: &mut HashMap<(usize, u8), u32>,
    t: usize,
    flags: u32,
) {
    let total = g.report_size.saturating_mul(g.report_count);
    *bits.entry((t, g.report_id)).or_insert(0) += total;
    let is_const = flags & 0x01 != 0;
    let is_array = flags & 0x02 == 0;
    let has_usages = !l.usages.is_empty() || (l.usage_min.is_some() && l.usage_max.is_some());
    if (is_const && !has_usages) || g.report_count == 0 {
        return;
    }
    let is_button = g.report_size == 1 || is_array;
    let link = coll.last().copied().unwrap_or(0);
    let node = caps.link_nodes.get(link as usize).cloned().unwrap_or_default();
    // Logical max: reinterpret as unsigned when the minimum is non-negative.
    let mut lmax = g.logical_max;
    if g.logical_min >= 0 && lmax < 0 {
        lmax = g.logical_max_raw.0 as i32;
    }
    let base = ValueCap {
        usage_page: g.usage_page,
        report_id: g.report_id,
        bit_field: flags as u16,
        link_collection: link,
        link_usage: node.usage,
        link_usage_page: node.usage_page,
        is_range: false,
        is_absolute: flags & 0x04 == 0,
        has_null: flags & 0x40 != 0,
        bit_size: g.report_size as u16,
        report_count: g.report_count as u16,
        units_exp: g.unit_exp,
        units: g.unit,
        logical_min: g.logical_min,
        logical_max: lmax,
        physical_min: g.physical_min,
        physical_max: g.physical_max,
        string_min: l.string_index,
        string_max: l.string_index,
        designator_min: l.designator_index,
        designator_max: l.designator_index,
        ..Default::default()
    };
    let page_of = |p: u16| if p == 0 { g.usage_page } else { p };
    let mut out: Vec<ValueCap> = Vec::new();
    if let (Some(min), Some(max)) = (l.usage_min, l.usage_max) {
        let mut c = base.clone();
        c.is_range = true;
        c.usage_page = page_of(min.0);
        c.usage_min = min.1;
        c.usage_max = max.1.max(min.1);
        c.string_min = l.string_min;
        c.string_max = l.string_max;
        c.designator_min = l.designator_min;
        c.designator_max = l.designator_max;
        out.push(c);
    } else if l.usages.is_empty() {
        out.push(base.clone());
    } else if is_button {
        for &(p, u) in &l.usages {
            let mut c = base.clone();
            c.usage_page = page_of(p);
            c.usage_min = u;
            c.usage_max = u;
            out.push(c);
        }
    } else {
        let n = l.usages.len().min(g.report_count as usize);
        for (k, &(p, u)) in l.usages.iter().take(n).enumerate() {
            let mut c = base.clone();
            c.usage_page = page_of(p);
            c.usage_min = u;
            c.usage_max = u;
            c.report_count = if k + 1 == n { (g.report_count as usize - (n - 1)) as u16 } else { 1 };
            out.push(c);
        }
    }
    for mut c in out {
        let n_idx = if c.is_range { c.usage_max - c.usage_min + 1 } else { 1 };
        c.data_index_min = caps.data_indices[t];
        c.data_index_max = caps.data_indices[t] + n_idx - 1;
        caps.data_indices[t] += n_idx;
        if is_button {
            caps.buttons[t].push(c);
        } else {
            caps.values[t].push(c);
        }
    }
}

impl Caps {
    /// `HIDP_CAPS` (64 bytes).
    pub fn to_hidp_caps(&self) -> [u8; 64] {
        let mut o = [0u8; 64];
        let mut put = |off: usize, v: u16| o[off..off + 2].copy_from_slice(&v.to_le_bytes());
        put(0, self.usage);
        put(2, self.usage_page);
        put(4, self.report_len[HIDP_INPUT]);
        put(6, self.report_len[HIDP_OUTPUT]);
        put(8, self.report_len[HIDP_FEATURE]);
        put(44, self.link_nodes.len() as u16);
        for t in 0..3 {
            let b = 46 + t * 6;
            put(b, self.buttons[t].len() as u16);
            put(b + 2, self.values[t].len() as u16);
            put(b + 4, self.data_indices[t]);
        }
        o
    }
}

impl ValueCap {
    /// `HIDP_VALUE_CAPS` (72 bytes).
    pub fn to_hidp_value_caps(&self) -> [u8; 72] {
        let mut o = [0u8; 72];
        self.write_common(&mut o);
        o[16] = self.has_null as u8;
        o[18..20].copy_from_slice(&self.bit_size.to_le_bytes());
        o[20..22].copy_from_slice(&self.report_count.to_le_bytes());
        o[32..36].copy_from_slice(&self.units_exp.to_le_bytes());
        o[36..40].copy_from_slice(&self.units.to_le_bytes());
        o[40..44].copy_from_slice(&self.logical_min.to_le_bytes());
        o[44..48].copy_from_slice(&self.logical_max.to_le_bytes());
        o[48..52].copy_from_slice(&self.physical_min.to_le_bytes());
        o[52..56].copy_from_slice(&self.physical_max.to_le_bytes());
        self.write_union(&mut o);
        o
    }

    /// `HIDP_BUTTON_CAPS` (72 bytes).
    pub fn to_hidp_button_caps(&self) -> [u8; 72] {
        let mut o = [0u8; 72];
        self.write_common(&mut o);
        o[16..18].copy_from_slice(&self.report_count.to_le_bytes());
        self.write_union(&mut o);
        o
    }

    fn write_common(&self, o: &mut [u8; 72]) {
        o[0..2].copy_from_slice(&self.usage_page.to_le_bytes());
        o[2] = self.report_id;
        o[4..6].copy_from_slice(&self.bit_field.to_le_bytes());
        o[6..8].copy_from_slice(&self.link_collection.to_le_bytes());
        o[8..10].copy_from_slice(&self.link_usage.to_le_bytes());
        o[10..12].copy_from_slice(&self.link_usage_page.to_le_bytes());
        o[12] = self.is_range as u8;
        o[13] = (self.string_min != self.string_max) as u8;
        o[14] = (self.designator_min != self.designator_max) as u8;
        o[15] = self.is_absolute as u8;
    }

    fn write_union(&self, o: &mut [u8; 72]) {
        let w = |o: &mut [u8; 72], k: usize, v: u16| o[56 + 2 * k..58 + 2 * k].copy_from_slice(&v.to_le_bytes());
        if self.is_range {
            w(o, 0, self.usage_min);
            w(o, 1, self.usage_max);
            w(o, 2, self.string_min);
            w(o, 3, self.string_max);
            w(o, 4, self.designator_min);
            w(o, 5, self.designator_max);
            w(o, 6, self.data_index_min);
            w(o, 7, self.data_index_max);
        } else {
            w(o, 0, self.usage_min);
            w(o, 2, self.string_min);
            w(o, 4, self.designator_min);
            w(o, 6, self.data_index_min);
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    fn hex(s: &str) -> Vec<u8> {
        s.split_whitespace().map(|b| u8::from_str_radix(b, 16).unwrap()).collect()
    }

    /// DualSense (054C:0CE6) USB report descriptor, 273 bytes
    /// (github.com/nondebug/dualsense).
    pub(crate) fn dualsense_usb() -> Vec<u8> {
        hex("05 01 09 05 a1 01 85 01 09 30 09 31 09 32 09 35 09 33 09 34 15 00 26 ff 00 75 08 95 06 81 02 06 00 ff 09 20 95 01 81 02 05 01 09 39 15 00 25 07 35 00 46 3b 01 65 14 75 04 95 01 81 42 65 00 05 09 19 01 29 0f 15 00 25 01 75 01 95 0f 81 02 06 00 ff 09 21 95 0d 81 02 06 00 ff 09 22 15 00 26 ff 00 75 08 95 34 81 02 85 02 09 23 95 2f 91 02 85 05 09 33 95 28 b1 02 85 08 09 34 95 2f b1 02 85 09 09 24 95 13 b1 02 85 0a 09 25 95 1a b1 02 85 20 09 26 95 3f b1 02 85 21 09 27 95 04 b1 02 85 22 09 40 95 3f b1 02 85 80 09 28 95 3f b1 02 85 81 09 29 95 3f b1 02 85 82 09 2a 95 09 b1 02 85 83 09 2b 95 3f b1 02 85 84 09 2c 95 3f b1 02 85 85 09 2d 95 02 b1 02 85 a0 09 2e 95 01 b1 02 85 e0 09 2f 95 3f b1 02 85 f0 09 30 95 3f b1 02 85 f1 09 31 95 3f b1 02 85 f2 09 32 95 0f b1 02 85 f4 09 35 95 3f b1 02 85 f5 09 36 95 03 b1 02 c0")
    }

    /// DualSense Bluetooth report descriptor, 280 bytes (same source).
    pub(crate) fn dualsense_bt() -> Vec<u8> {
        hex("05 01 09 05 a1 01 85 01 09 30 09 31 09 32 09 35 15 00 26 ff 00 75 08 95 04 81 02 09 39 15 00 25 07 35 00 46 3b 01 65 14 75 04 95 01 81 42 65 00 05 09 19 01 29 0e 15 00 25 01 75 01 95 0e 81 02 75 06 95 01 81 01 05 01 09 33 09 34 15 00 26 ff 00 75 08 95 02 81 02 06 00 ff 15 00 26 ff 00 75 08 95 4d 85 31 09 31 91 02 09 3b 81 02 85 32 09 32 95 8d 91 02 85 33 09 33 95 cd 91 02 85 34 09 34 96 0d 01 91 02 85 35 09 35 96 4d 01 91 02 85 36 09 36 96 8d 01 91 02 85 37 09 37 96 cd 01 91 02 85 38 09 38 96 0d 02 91 02 85 39 09 39 96 22 02 91 02 06 80 ff 85 05 09 33 95 28 b1 02 85 08 09 34 95 2f b1 02 85 09 09 24 95 13 b1 02 85 20 09 26 95 3f b1 02 85 22 09 40 95 3f b1 02 85 80 09 28 95 3f b1 02 85 81 09 29 95 3f b1 02 85 82 09 2a 95 09 b1 02 85 83 09 2b 95 3f b1 02 85 f1 09 31 95 3f b1 02 85 f2 09 32 95 0f b1 02 85 f0 09 30 95 3f b1 02 c0 00")
    }

    #[test]
    fn dualsense_usb_caps_match_windows() {
        let c = parse(&dualsense_usb()).unwrap();
        assert_eq!((c.usage_page, c.usage), (1, 5));
        // Windows: Input 64, Output 48, Feature 64 (the game sizes its
        // output report from these: 48 = USB report 0x02).
        assert_eq!(c.report_len, [64, 48, 64]);
        assert!(c.uses_report_ids);
        assert_eq!(c.link_nodes.len(), 1);
        // Input values: X Y Z Rz Rx Ry, vendor 0x20, hat, vendor 0x22 (x52).
        let iv: Vec<(u16, u16, u16)> = c.values[HIDP_INPUT].iter().map(|v| (v.usage_page, v.usage_min, v.report_count)).collect();
        assert_eq!(
            iv,
            vec![(1, 0x30, 1), (1, 0x31, 1), (1, 0x32, 1), (1, 0x35, 1), (1, 0x33, 1), (1, 0x34, 1), (0xff00, 0x20, 1), (1, 0x39, 1), (0xff00, 0x22, 52)]
        );
        let hat = &c.values[HIDP_INPUT][7];
        assert!(hat.has_null && hat.is_absolute);
        assert_eq!((hat.logical_min, hat.logical_max, hat.physical_max, hat.units), (0, 7, 315, 0x14));
        // Input buttons: Button 1..15 (range), vendor 0x21 (13 one-bit fields).
        assert_eq!(c.buttons[HIDP_INPUT].len(), 2);
        assert!(c.buttons[HIDP_INPUT][0].is_range);
        assert_eq!((c.buttons[HIDP_INPUT][0].usage_min, c.buttons[HIDP_INPUT][0].usage_max), (1, 15));
        // Feature: 20 vendor reports, including the 0x85 / usage 0x2D /
        // count 2 cap the game's pad library requires before it accepts a
        // DualSense (sub_1417a7c00).
        assert_eq!(c.values[HIDP_FEATURE].len(), 20);
        let f85 = c.values[HIDP_FEATURE].iter().find(|v| v.report_id == 0x85).unwrap();
        assert_eq!((f85.usage_page, f85.usage_min, f85.report_count, f85.bit_size), (0xff00, 0x2d, 2, 8));
        let raw = f85.to_hidp_value_caps();
        assert_eq!(u16::from_le_bytes([raw[0], raw[1]]), 0xff00);
        assert_eq!(raw[2], 0x85);
        assert_eq!(u16::from_le_bytes([raw[20], raw[21]]), 2);
        assert_eq!(u16::from_le_bytes([raw[56], raw[57]]), 0x2d);
        // Feature 0x09 (pairing info) is 19 + 1 bytes: the game reads 20.
        assert_eq!(c.report_bits[&(HIDP_FEATURE, 0x09)], 19 * 8);
        let hc = c.to_hidp_caps();
        assert_eq!(u16::from_le_bytes([hc[60], hc[61]]), 20, "NumberFeatureValueCaps");
        assert_eq!(u16::from_le_bytes([hc[4], hc[5]]), 64);
        assert_eq!(u16::from_le_bytes([hc[6], hc[7]]), 48);
    }

    #[test]
    fn dualsense_bt_caps_match_windows() {
        let c = parse(&dualsense_bt()).unwrap();
        assert_eq!((c.usage_page, c.usage), (1, 5));
        // Input: report 0x31 is 77 bytes -> 78. Output: report 0x39 is 546
        // bytes -> 547 (> 48, so the game picks its 78-byte BT path).
        assert_eq!(c.report_len, [78, 547, 64]);
        // The BT descriptor has no feature report 0x85.
        assert!(c.values[HIDP_FEATURE].iter().all(|v| v.report_id != 0x85));
        assert!(c.values[HIDP_FEATURE].iter().all(|v| v.usage_page == 0xff80));
        // Constant padding (75 06 95 01 81 01) produces no caps.
        assert!(c.values[HIDP_INPUT].iter().chain(&c.buttons[HIDP_INPUT]).all(|v| v.bit_field & 1 == 0));
    }

    #[test]
    fn no_report_ids_still_count_the_id_byte_and_nesting_adds_link_nodes() {
        // Gamepad, 2 x 8-bit axes inside a physical collection, 8 buttons.
        let d = hex("05 01 09 05 a1 01 09 01 a1 00 09 30 09 31 15 81 25 7f 75 08 95 02 81 02 c0 05 09 19 01 29 08 15 00 25 01 75 01 95 08 81 02 c0");
        let c = parse(&d).unwrap();
        assert!(!c.uses_report_ids);
        assert_eq!(c.report_len, [4, 0, 0]);
        assert_eq!(c.link_nodes.len(), 2);
        assert_eq!(c.values[HIDP_INPUT].len(), 2);
        assert_eq!(c.values[HIDP_INPUT][0].link_collection, 1);
        assert_eq!(c.values[HIDP_INPUT][0].link_usage, 1);
        assert_eq!((c.values[HIDP_INPUT][0].logical_min, c.values[HIDP_INPUT][0].logical_max), (-127, 127));
        assert_eq!(c.data_indices[HIDP_INPUT], 2 + 8);
        assert_eq!((c.buttons[HIDP_INPUT][0].data_index_min, c.buttons[HIDP_INPUT][0].data_index_max), (2, 9));
    }
}
