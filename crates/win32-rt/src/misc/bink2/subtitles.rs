//! SRT subtitle loading and frame-time lookup for `bink2w64.dll`.

use super::player::BinkInstance;
use crate::strings::{read_ansi_bytes_z, DEFAULT_MAX_STR_LEN};
use std::ffi::c_void;

pub(crate) struct SubtitleEntry {
    pub(crate) start_ms: u64,
    pub(crate) end_ms: u64,
    pub(crate) text_nul: Vec<u8>,
}

fn parse_srt_timestamp_ms(s: &str) -> Option<u64> {
    let s = s.trim();
    let mut parts = s.split(':');
    let h: u64 = parts.next()?.trim().parse().ok()?;
    let m: u64 = parts.next()?.trim().parse().ok()?;
    let sec_ms = parts.next()?.trim();
    let (sec_str, ms_str) = sec_ms
        .split_once(',')
        .or_else(|| sec_ms.split_once('.'))
        .unwrap_or((sec_ms, "0"));
    let sec: u64 = sec_str.trim().parse().ok()?;
    let ms_digits: String = ms_str.chars().take_while(|c| c.is_ascii_digit()).collect();
    let ms: u64 = if ms_digits.is_empty() {
        0
    } else {
        ms_digits.parse().ok()?
    };
    Some(
        h.saturating_mul(3_600_000)
            .saturating_add(m.saturating_mul(60_000))
            .saturating_add(sec.saturating_mul(1_000))
            .saturating_add(ms.min(999)),
    )
}

fn parse_srt_subtitles(content: &str) -> Vec<SubtitleEntry> {
    let mut entries = Vec::new();
    let clean = content.strip_prefix('\u{feff}').unwrap_or(content);
    let mut lines = clean.lines().peekable();
    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let time_line = if trimmed.contains("-->") {
            trimmed
        } else if let Some(next_line) = lines.peek() {
            if next_line.contains("-->") {
                lines.next().unwrap().trim()
            } else {
                continue;
            }
        } else {
            continue;
        };
        let Some((start_s, end_s)) = time_line.split_once("-->") else {
            continue;
        };
        let (Some(start_ms), Some(end_ms)) = (
            parse_srt_timestamp_ms(start_s),
            parse_srt_timestamp_ms(end_s.split_whitespace().next().unwrap_or("")),
        ) else {
            continue;
        };
        let mut text = String::new();
        for text_line in lines.by_ref() {
            if text_line.trim().is_empty() {
                break;
            }
            if !text.is_empty() {
                text.push('\n');
            }
            text.push_str(text_line.trim_end_matches('\r'));
        }
        if !text.is_empty() {
            let mut text_nul = text.into_bytes();
            text_nul.push(0);
            entries.push(SubtitleEntry {
                start_ms,
                end_ms,
                text_nul,
            });
        }
    }
    entries
}

pub(crate) unsafe fn load_subtitles(bink: *mut c_void, name: *const u8) -> i32 {
    if bink.is_null() || name.is_null() {
        return 0;
    }
    let raw_name = read_ansi_bytes_z(name, DEFAULT_MAX_STR_LEN);
    let Ok(path_str) = std::str::from_utf8(&raw_name) else {
        return 0;
    };
    let content = crate::path::to_host(path_str)
        .and_then(|p| std::fs::read_to_string(p).ok())
        .or_else(|| std::fs::read_to_string(path_str).ok());
    let Some(text) = content else {
        return 0;
    };
    let parsed = parse_srt_subtitles(&text);
    let count = parsed.len() as i32;
    let inst = &mut *(bink as *mut BinkInstance);
    inst.subtitles = parsed;
    count
}

pub(crate) unsafe fn current_subtitle(bink: *mut c_void) -> *const u8 {
    if bink.is_null() {
        return std::ptr::null();
    }
    let inst = &*(bink as *const BinkInstance);
    if inst.subtitles.is_empty() {
        return std::ptr::null();
    }
    let cur_frame = inst.rd_u32(0x00c).saturating_sub(1) as u64;
    let fps_num = inst.rd_u32(0x014).max(1) as u64;
    let fps_den = inst.rd_u32(0x018).max(1) as u64;
    let time_ms = cur_frame
        .saturating_mul(fps_den)
        .saturating_mul(1000)
        / fps_num;
    for entry in &inst.subtitles {
        if time_ms >= entry.start_ms && time_ms <= entry.end_ms {
            return entry.text_nul.as_ptr();
        }
    }
    std::ptr::null()
}

pub(crate) unsafe fn subtitle_by_index(bink: *mut c_void, index: i32) -> *const u8 {
    if bink.is_null() || index < 0 {
        return std::ptr::null();
    }
    let inst = &*(bink as *const BinkInstance);
    inst.subtitles
        .get(index as usize)
        .map_or(std::ptr::null(), |e| e.text_nul.as_ptr())
}
