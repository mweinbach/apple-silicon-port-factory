//! Pure-Rust FMOD Studio `.bank` (RIFF `FEV `) and standalone FSB5 bank parser.

use super::core::{
    FMOD_ERR_FORMAT, FMOD_STUDIO_PARAMETER_GLOBAL, FmodGuid, FmodStudioParameterId,
};
use super::dsp::{DecodedSample, identify_container, parse_fsb5};
use crate::strings::{le_u32, le_u64};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub(super) struct StudioParameterDef {
    pub(super) name: String,
    pub(super) id: FmodStudioParameterId,
    pub(super) guid: FmodGuid,
    pub(super) minimum: f32,
    pub(super) maximum: f32,
    pub(super) default_value: f32,
    pub(super) param_type: u32,
    pub(super) flags: u32,
}

pub(super) struct ParsedBus {
    pub(super) guid: FmodGuid,
    pub(super) parent_guid: Option<FmodGuid>,
    pub(super) path: String,
    pub(super) default_volume: f32,
    pub(super) mute: bool,
    pub(super) paused: bool,
}

pub(super) struct ParsedVca {
    pub(super) guid: FmodGuid,
    pub(super) path: String,
    pub(super) default_volume: f32,
    pub(super) target_bus_guids: Vec<FmodGuid>,
    pub(super) target_event_guids: Vec<FmodGuid>,
}

pub(super) struct ParsedEvent {
    pub(super) guid: FmodGuid,
    pub(super) bus_guid: Option<FmodGuid>,
    pub(super) path: String,
    pub(super) sample_index: Option<usize>,
    pub(super) is_3d: bool,
    pub(super) is_oneshot: bool,
    pub(super) is_stream: bool,
    pub(super) is_snapshot: bool,
    pub(super) default_volume: f32,
    pub(super) default_pitch: f32,
    pub(super) min_distance: f32,
    pub(super) max_distance: f32,
    pub(super) length_ms: i32,
    pub(super) parameters: Vec<StudioParameterDef>,
}

pub(super) struct ParsedBank {
    pub(super) guid: FmodGuid,
    pub(super) has_explicit_guid: bool,
    pub(super) path: String,
    pub(super) strings: Vec<(FmodGuid, String)>,
    pub(super) samples: Vec<DecodedSample>,
    pub(super) buses: Vec<ParsedBus>,
    pub(super) vcas: Vec<ParsedVca>,
    pub(super) events: Vec<ParsedEvent>,
    pub(super) global_params: Vec<StudioParameterDef>,
}

#[inline]
fn rd_i32_le(b: &[u8], off: usize) -> Option<i32> {
    le_u32(b, off).map(|v| v as i32)
}

#[inline]
fn rd_f32_le(b: &[u8], off: usize) -> Option<f32> {
    le_u32(b, off).map(f32::from_bits)
}

#[inline]
fn rd_guid(b: &[u8], off: usize) -> Option<FmodGuid> {
    let s = b.get(off..off.checked_add(16)?)?;
    let mut arr = [0u8; 16];
    arr.copy_from_slice(s);
    Some(FmodGuid::from_le_bytes(arr))
}

fn rd_len_string(b: &[u8], off: &mut usize) -> Option<String> {
    let len = le_u32(b, *off)? as usize;
    *off += 4;
    let end = off.checked_add(len)?;
    let slice = b.get(*off..end)?;
    *off = end;
    let trimmed = slice.strip_suffix(&[0]).unwrap_or(slice);
    Some(String::from_utf8_lossy(trimmed).into_owned())
}

pub(super) fn guid_from_uid(uid: &str) -> [u8; 16] {
    let mut h1: u64 = 0xcbf2_9ce4_8422_2325;
    let mut h2: u64 = 0x8422_2325_cbf2_9ce4;
    for &b in uid.as_bytes() {
        h1 ^= b as u64;
        h1 = h1.wrapping_mul(0x0000_0100_0000_01b3);
        h2 = h2.wrapping_mul(0x0000_0100_0000_01b3);
        h2 ^= b as u64;
    }
    let mut g = [0u8; 16];
    g[0..8].copy_from_slice(&h1.to_le_bytes());
    g[8..16].copy_from_slice(&h2.to_le_bytes());
    g[7] = (g[7] & 0x0F) | 0x50;
    g[8] = (g[8] & 0x3F) | 0x80;
    g
}

/// Parse an FMOD Studio radix trie (`STRR` or `STRT` type 1).
/// Layout:
/// - `guid_count: u32`, `node_count: u32`, `pool_size: u32`
/// - `guid_count` x 16-byte `FmodGuid`
/// - `node_count` x 16-byte nodes `(parent_idx: i32, guid_idx: i32, str_off: u32, str_len: u32)`
/// - `pool_size` bytes of UTF-8 edge string slices
fn parse_radix_trie_strings(payload: &[u8], out: &mut Vec<(FmodGuid, String)>) -> bool {
    let Some(guid_count) = le_u32(payload, 0).map(|v| v as usize) else {
        return false;
    };
    let Some(node_count) = le_u32(payload, 4).map(|v| v as usize) else {
        return false;
    };
    let Some(pool_size) = le_u32(payload, 8).map(|v| v as usize) else {
        return false;
    };

    let guids_off = 12usize;
    let Some(guids_bytes) = guid_count.checked_mul(16) else {
        return false;
    };
    let Some(nodes_off) = guids_off.checked_add(guids_bytes) else {
        return false;
    };
    let Some(nodes_bytes) = node_count.checked_mul(16) else {
        return false;
    };
    let Some(pool_off) = nodes_off.checked_add(nodes_bytes) else {
        return false;
    };
    let Some(pool_end) = pool_off.checked_add(pool_size) else {
        return false;
    };
    if pool_end > payload.len() {
        return false;
    }

    let mut guids = Vec::with_capacity(guid_count);
    for i in 0..guid_count {
        let Some(g) = rd_guid(payload, guids_off + i * 16) else {
            return false;
        };
        guids.push(g);
    }

    struct TrieNode {
        parent: i32,
        guid_idx: i32,
        str_off: usize,
        str_len: usize,
    }
    let mut nodes = Vec::with_capacity(node_count);
    for i in 0..node_count {
        let base = nodes_off + i * 16;
        let (Some(parent), Some(guid_idx), Some(so), Some(sl)) = (
            rd_i32_le(payload, base),
            rd_i32_le(payload, base + 4),
            le_u32(payload, base + 8),
            le_u32(payload, base + 12),
        ) else {
            return false;
        };
        nodes.push(TrieNode {
            parent,
            guid_idx,
            str_off: so as usize,
            str_len: sl as usize,
        });
    }

    let pool = &payload[pool_off..pool_end];
    for (idx, node) in nodes.iter().enumerate() {
        if node.guid_idx < 0 || (node.guid_idx as usize) >= guids.len() {
            continue;
        }
        let mut parts: Vec<&[u8]> = Vec::new();
        let mut cur = idx as i32;
        let mut hops = 0usize;
        while cur >= 0 && (cur as usize) < nodes.len() && hops < 128 {
            hops += 1;
            let n = &nodes[cur as usize];
            if n.str_len > 0 {
                let Some(end) = n.str_off.checked_add(n.str_len) else {
                    break;
                };
                if end <= pool.len() {
                    parts.push(&pool[n.str_off..end]);
                }
            }
            cur = n.parent;
        }
        parts.reverse();
        let mut full = Vec::new();
        for p in parts {
            full.extend_from_slice(p);
        }
        let path = String::from_utf8_lossy(&full).into_owned();
        if !path.is_empty() {
            out.push((guids[node.guid_idx as usize], path));
        }
    }
    true
}

/// Parse an offset-pool string table (`STRI` or `STRT` type 2):
/// `count: u32`, `count` x `(guid: [u8; 16], str_offset: u32)`, then NUL-terminated string pool.
fn parse_offset_pool_strings(payload: &[u8], out: &mut Vec<(FmodGuid, String)>) -> bool {
    let Some(count) = le_u32(payload, 0).map(|v| v as usize) else {
        return false;
    };
    let Some(table_bytes) = count.checked_mul(20) else {
        return false;
    };
    let Some(pool_off) = 4usize.checked_add(table_bytes) else {
        return false;
    };
    if pool_off > payload.len() {
        return false;
    }
    let pool = &payload[pool_off..];
    for i in 0..count {
        let base = 4 + i * 20;
        let (Some(guid), Some(str_off)) = (rd_guid(payload, base), le_u32(payload, base + 16))
        else {
            return false;
        };
        let off = str_off as usize;
        if off >= pool.len() {
            return false;
        }
        let tail = &pool[off..];
        let end = tail.iter().position(|&b| b == 0).unwrap_or(tail.len());
        let path = String::from_utf8_lossy(&tail[..end]).into_owned();
        out.push((guid, path));
    }
    true
}

/// Parse a flat inline string table (`STRT` type 0 or `GUID`):
/// `count: u32`, `count` x `(guid: [u8; 16], path_len: u32, path_bytes)`.
fn parse_inline_strings(payload: &[u8], out: &mut Vec<(FmodGuid, String)>) -> bool {
    let Some(count) = le_u32(payload, 0).map(|v| v as usize) else {
        return false;
    };
    let mut off = 4usize;
    let mut entries = Vec::with_capacity(count.min(4096));
    for _ in 0..count {
        let Some(guid) = rd_guid(payload, off) else {
            return false;
        };
        off += 16;
        let Some(path) = rd_len_string(payload, &mut off) else {
            return false;
        };
        entries.push((guid, path));
    }
    out.extend(entries);
    true
}

fn parse_strt_chunk(payload: &[u8], out: &mut Vec<(FmodGuid, String)>) -> bool {
    let Some(table_type) = le_u32(payload, 0) else {
        return false;
    };
    match table_type {
        0 => parse_inline_strings(&payload[4..], out),
        1 => parse_radix_trie_strings(&payload[4..], out),
        2 => parse_offset_pool_strings(&payload[4..], out),
        _ => {
            // Fallback: try interpreting without a leading type tag.
            parse_inline_strings(payload, out) || parse_offset_pool_strings(payload, out)
        }
    }
}

fn parse_buss_chunk(payload: &[u8], out: &mut Vec<ParsedBus>) -> Result<(), i32> {
    let count = le_u32(payload, 0).ok_or(FMOD_ERR_FORMAT)? as usize;
    let mut off = 4usize;
    for _ in 0..count {
        let guid = rd_guid(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 16;
        let parent_raw = rd_guid(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 16;
        let default_volume = rd_f32_le(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 4;
        let flags = le_u32(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 4;
        let path = rd_len_string(payload, &mut off).ok_or(FMOD_ERR_FORMAT)?;
        out.push(ParsedBus {
            guid,
            parent_guid: (!parent_raw.is_zero()).then_some(parent_raw),
            path,
            default_volume: if default_volume.is_finite() {
                default_volume
            } else {
                1.0
            },
            mute: (flags & 1) != 0,
            paused: (flags & 2) != 0,
        });
    }
    Ok(())
}

fn parse_vcas_chunk(payload: &[u8], out: &mut Vec<ParsedVca>) -> Result<(), i32> {
    let count = le_u32(payload, 0).ok_or(FMOD_ERR_FORMAT)? as usize;
    let mut off = 4usize;
    for _ in 0..count {
        let guid = rd_guid(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 16;
        let default_volume = rd_f32_le(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 4;
        let path = rd_len_string(payload, &mut off).ok_or(FMOD_ERR_FORMAT)?;
        let bus_count = le_u32(payload, off).ok_or(FMOD_ERR_FORMAT)? as usize;
        off += 4;
        let mut target_bus_guids = Vec::with_capacity(bus_count.min(256));
        for _ in 0..bus_count {
            let g = rd_guid(payload, off).ok_or(FMOD_ERR_FORMAT)?;
            off += 16;
            target_bus_guids.push(g);
        }
        let ev_count = le_u32(payload, off).ok_or(FMOD_ERR_FORMAT)? as usize;
        off += 4;
        let mut target_event_guids = Vec::with_capacity(ev_count.min(256));
        for _ in 0..ev_count {
            let g = rd_guid(payload, off).ok_or(FMOD_ERR_FORMAT)?;
            off += 16;
            target_event_guids.push(g);
        }
        out.push(ParsedVca {
            guid,
            path,
            default_volume: if default_volume.is_finite() {
                default_volume
            } else {
                1.0
            },
            target_bus_guids,
            target_event_guids,
        });
    }
    Ok(())
}

fn parse_evnt_chunk(payload: &[u8], out: &mut Vec<ParsedEvent>) -> Result<(), i32> {
    let count = le_u32(payload, 0).ok_or(FMOD_ERR_FORMAT)? as usize;
    let mut off = 4usize;
    for _ in 0..count {
        let guid = rd_guid(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 16;
        let bus_raw = rd_guid(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 16;
        let sample_idx = le_u32(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 4;
        let flags = le_u32(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 4;
        let default_volume = rd_f32_le(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 4;
        let default_pitch = rd_f32_le(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 4;
        let min_distance = rd_f32_le(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 4;
        let max_distance = rd_f32_le(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 4;
        let length_ms = le_u32(payload, off).ok_or(FMOD_ERR_FORMAT)? as i32;
        off += 4;
        let path = rd_len_string(payload, &mut off).ok_or(FMOD_ERR_FORMAT)?;

        out.push(ParsedEvent {
            guid,
            bus_guid: (!bus_raw.is_zero()).then_some(bus_raw),
            path,
            sample_index: (sample_idx != u32::MAX).then_some(sample_idx as usize),
            is_3d: (flags & 0x01) != 0,
            is_oneshot: (flags & 0x02) != 0,
            is_stream: (flags & 0x04) != 0,
            is_snapshot: (flags & 0x08) != 0,
            default_volume: if default_volume.is_finite() {
                default_volume
            } else {
                1.0
            },
            default_pitch: if default_pitch.is_finite() && default_pitch > 0.0 {
                default_pitch
            } else {
                1.0
            },
            min_distance: if min_distance.is_finite() && min_distance >= 0.0 {
                min_distance
            } else {
                1.0
            },
            max_distance: if max_distance.is_finite() && max_distance >= min_distance.max(0.0) {
                max_distance
            } else {
                20.0
            },
            length_ms,
            parameters: Vec::new(),
        });
    }
    Ok(())
}

fn parse_parm_chunk(
    payload: &[u8],
    out: &mut Vec<(FmodGuid, StudioParameterDef)>,
) -> Result<(), i32> {
    let count = le_u32(payload, 0).ok_or(FMOD_ERR_FORMAT)? as usize;
    let mut off = 4usize;
    for _ in 0..count {
        let owner_guid = rd_guid(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 16;
        let param_id = FmodStudioParameterId::from_u64(le_u64(payload, off).ok_or(FMOD_ERR_FORMAT)?);
        off += 8;
        let minimum = rd_f32_le(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 4;
        let maximum = rd_f32_le(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 4;
        let default_value = rd_f32_le(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 4;
        let flags = le_u32(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 4;
        let param_type = le_u32(payload, off).ok_or(FMOD_ERR_FORMAT)?;
        off += 4;
        let name = rd_len_string(payload, &mut off).ok_or(FMOD_ERR_FORMAT)?;
        let pguid = FmodGuid::from_le_bytes(guid_from_uid(&format!(
            "param:{name}:{:x}:{:x}",
            param_id.data1, param_id.data2
        )));
        out.push((
            owner_guid,
            StudioParameterDef {
                name,
                id: param_id,
                guid: pguid,
                minimum: if minimum.is_finite() { minimum } else { 0.0 },
                maximum: if maximum.is_finite() { maximum } else { 1.0 },
                default_value: if default_value.is_finite() {
                    default_value
                } else {
                    0.0
                },
                param_type,
                flags,
            },
        ));
    }
    Ok(())
}

fn parse_embedded_fsb5_chunk(
    payload: &[u8],
    samples_out: &mut Vec<DecodedSample>,
) -> Result<(), i32> {
    let mut off = 0usize;
    let mut found_any = false;
    while off + 4 <= payload.len() {
        if &payload[off..off + 4] == b"FSB5" {
            let fsb = parse_fsb5(&payload[off..])?;
            samples_out.extend(fsb.samples);
            found_any = true;
            let step = fsb.total_bytes.max(4);
            off = off.saturating_add(step);
        } else {
            off += 1;
        }
    }
    if !found_any && payload.len() >= 4 {
        return Err(FMOD_ERR_FORMAT);
    }
    Ok(())
}

struct FevWalkState {
    recognized_chunks: usize,
    bank_guid: FmodGuid,
    has_explicit_guid: bool,
    strings: Vec<(FmodGuid, String)>,
    samples: Vec<DecodedSample>,
    buses: Vec<ParsedBus>,
    vcas: Vec<ParsedVca>,
    events: Vec<ParsedEvent>,
    raw_params: Vec<(FmodGuid, StudioParameterDef)>,
}

fn walk_fev_chunks(slice: &[u8], st: &mut FevWalkState, depth: usize) -> Result<(), i32> {
    if depth > 16 {
        return Err(FMOD_ERR_FORMAT);
    }
    let mut pos = 0usize;
    while pos < slice.len() {
        if pos + 8 > slice.len() {
            return Err(FMOD_ERR_FORMAT);
        }
        let chunk_id = &slice[pos..pos + 4];
        let chunk_size = le_u32(slice, pos + 4).ok_or(FMOD_ERR_FORMAT)? as usize;
        pos += 8;
        let payload_end = pos.checked_add(chunk_size).ok_or(FMOD_ERR_FORMAT)?;
        if payload_end > slice.len() {
            return Err(FMOD_ERR_FORMAT);
        }
        let payload = &slice[pos..payload_end];

        match chunk_id {
            b"FMT " => {
                if payload.len() < 4 {
                    return Err(FMOD_ERR_FORMAT);
                }
                st.recognized_chunks += 1;
            }
            b"LIST" => {
                if payload.len() < 4 {
                    return Err(FMOD_ERR_FORMAT);
                }
                st.recognized_chunks += 1;
                walk_fev_chunks(&payload[4..], st, depth + 1)?;
            }
            b"BNKI" => {
                if payload.len() < 16 {
                    return Err(FMOD_ERR_FORMAT);
                }
                if let Some(g) = rd_guid(payload, 0) {
                    if !g.is_zero() {
                        st.bank_guid = g;
                        st.has_explicit_guid = true;
                    }
                }
                st.recognized_chunks += 1;
            }
            b"STRT" => {
                if !parse_strt_chunk(payload, &mut st.strings) {
                    return Err(FMOD_ERR_FORMAT);
                }
                st.recognized_chunks += 1;
            }
            b"STRR" => {
                if !parse_radix_trie_strings(payload, &mut st.strings) {
                    return Err(FMOD_ERR_FORMAT);
                }
                st.recognized_chunks += 1;
            }
            b"STRI" => {
                if !parse_offset_pool_strings(payload, &mut st.strings) {
                    return Err(FMOD_ERR_FORMAT);
                }
                st.recognized_chunks += 1;
            }
            b"GUID" => {
                if !parse_inline_strings(payload, &mut st.strings) {
                    return Err(FMOD_ERR_FORMAT);
                }
                st.recognized_chunks += 1;
            }
            b"BUSS" => {
                parse_buss_chunk(payload, &mut st.buses)?;
                st.recognized_chunks += 1;
            }
            b"VCAS" => {
                parse_vcas_chunk(payload, &mut st.vcas)?;
                st.recognized_chunks += 1;
            }
            b"EVNT" => {
                parse_evnt_chunk(payload, &mut st.events)?;
                st.recognized_chunks += 1;
            }
            b"PARM" => {
                parse_parm_chunk(payload, &mut st.raw_params)?;
                st.recognized_chunks += 1;
            }
            b"SNDH" => {
                if payload.len() < 4 {
                    return Err(FMOD_ERR_FORMAT);
                }
                st.recognized_chunks += 1;
            }
            b"SND " | b"SNDA" | b"FSB5" => {
                parse_embedded_fsb5_chunk(payload, &mut st.samples)?;
                st.recognized_chunks += 1;
            }
            _ => {
                // Ignore unrecognized auxiliary RIFF chunks gracefully.
            }
        }

        let padded = chunk_size.saturating_add(chunk_size & 1);
        pos = pos.saturating_add(padded).min(slice.len());
    }
    Ok(())
}

pub(super) fn parse_studio_bank(bytes: &[u8], source_path: &str) -> Result<ParsedBank, i32> {
    if bytes.len() >= 4 && &bytes[0..4] == b"FSB5" {
        let fsb = parse_fsb5(bytes)?;
        if fsb.samples.is_empty() {
            return Err(FMOD_ERR_FORMAT);
        }
        let bank_path = if source_path.is_empty() {
            "bank:/FSB5".to_string()
        } else {
            source_path.to_string()
        };
        let mut strings = Vec::with_capacity(fsb.samples.len());
        let mut events = Vec::with_capacity(fsb.samples.len());
        for (i, smp) in fsb.samples.iter().enumerate() {
            let ev_path = if smp.name.is_empty() {
                format!("event:/sample_{i}")
            } else if smp.name.starts_with("event:/") {
                smp.name.clone()
            } else {
                format!("event:/{}", smp.name.trim_start_matches('/'))
            };
            let ev_guid = FmodGuid::from_le_bytes(guid_from_uid(&ev_path));
            let length_ms = ((smp.num_frames as u64).saturating_mul(1000)
                / (smp.sample_rate.max(1) as u64)) as i32;
            strings.push((ev_guid, ev_path.clone()));
            events.push(ParsedEvent {
                guid: ev_guid,
                bus_guid: None,
                path: ev_path,
                sample_index: Some(i),
                is_3d: false,
                is_oneshot: !smp.has_loop,
                is_stream: false,
                is_snapshot: false,
                default_volume: 1.0,
                default_pitch: 1.0,
                min_distance: 1.0,
                max_distance: 20.0,
                length_ms,
                parameters: Vec::new(),
            });
        }
        return Ok(ParsedBank {
            guid: FmodGuid::ZERO,
            has_explicit_guid: false,
            path: bank_path,
            strings,
            samples: fsb.samples,
            buses: Vec::new(),
            vcas: Vec::new(),
            events,
            global_params: Vec::new(),
        });
    }

    if bytes.len() < 12 || &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"FEV " {
        log::warn!(
            target: "audio",
            "FMOD Studio LoadBank: invalid bank container ({}, {} bytes) -> FMOD_ERR_FORMAT",
            identify_container(bytes),
            bytes.len()
        );
        return Err(FMOD_ERR_FORMAT);
    }

    let riff_size = le_u32(bytes, 4).ok_or(FMOD_ERR_FORMAT)? as usize;
    let container_end = 8usize.checked_add(riff_size).ok_or(FMOD_ERR_FORMAT)?;
    if riff_size < 4 || container_end > bytes.len() {
        return Err(FMOD_ERR_FORMAT);
    }

    let mut st = FevWalkState {
        recognized_chunks: 0,
        bank_guid: FmodGuid::ZERO,
        has_explicit_guid: false,
        strings: Vec::new(),
        samples: Vec::new(),
        buses: Vec::new(),
        vcas: Vec::new(),
        events: Vec::new(),
        raw_params: Vec::new(),
    };
    walk_fev_chunks(&bytes[12..container_end], &mut st, 0)?;

    if st.recognized_chunks == 0 {
        return Err(FMOD_ERR_FORMAT);
    }

    // Build local GUID -> path map from this bank's string entries.
    let mut local_g2p: HashMap<FmodGuid, String> = HashMap::new();
    for (g, p) in &st.strings {
        local_g2p.insert(*g, p.clone());
    }

    let bank_path = if st.has_explicit_guid {
        local_g2p
            .get(&st.bank_guid)
            .cloned()
            .unwrap_or_else(|| source_path.to_string())
    } else {
        st.strings
            .iter()
            .find(|(_, p)| p.to_ascii_lowercase().starts_with("bank:/"))
            .map(|(_, p)| p.clone())
            .unwrap_or_else(|| source_path.to_string())
    };

    for bus in &mut st.buses {
        if bus.path.is_empty() {
            if let Some(p) = local_g2p.get(&bus.guid) {
                bus.path = p.clone();
            }
        }
    }
    for vca in &mut st.vcas {
        if vca.path.is_empty() {
            if let Some(p) = local_g2p.get(&vca.guid) {
                vca.path = p.clone();
            }
        }
    }
    for ev in &mut st.events {
        if ev.path.is_empty() {
            if let Some(p) = local_g2p.get(&ev.guid) {
                ev.path = p.clone();
            }
        }
    }

    // Attach parsed parameters either to matching events or global parameter list.
    let mut global_params = Vec::new();
    for (owner, pdef) in st.raw_params {
        if owner.is_zero() || (pdef.flags & FMOD_STUDIO_PARAMETER_GLOBAL) != 0 {
            global_params.push(pdef);
        } else if let Some(ev) = st.events.iter_mut().find(|e| e.guid == owner) {
            ev.parameters.push(pdef);
        } else {
            global_params.push(pdef);
        }
    }

    // If the bank carries embedded FSB5 samples but no explicit EVNT chunk, synthesize
    // events from any event:/ string entries or FSB5 sample names so real audio banks
    // with stripped/opaque MODL metadata still resolve and mix audio.
    if st.events.is_empty() && !st.samples.is_empty() {
        let mut matched_event_strings: Vec<(FmodGuid, String)> = st
            .strings
            .iter()
            .filter(|(_, p)| p.to_ascii_lowercase().starts_with("event:/"))
            .cloned()
            .collect();
        if !matched_event_strings.is_empty() {
            for (idx, (guid, path)) in matched_event_strings.drain(..).enumerate() {
                let leaf = path.rsplit('/').next().unwrap_or("");
                let sample_index = st
                    .samples
                    .iter()
                    .position(|s| !s.name.is_empty() && s.name.eq_ignore_ascii_case(leaf))
                    .or_else(|| (idx < st.samples.len()).then_some(idx));
                let length_ms = sample_index
                    .and_then(|si| st.samples.get(si))
                    .map(|s| {
                        ((s.num_frames as u64).saturating_mul(1000) / (s.sample_rate.max(1) as u64))
                            as i32
                    })
                    .unwrap_or(0);
                st.events.push(ParsedEvent {
                    guid,
                    bus_guid: None,
                    path,
                    sample_index,
                    is_3d: false,
                    is_oneshot: true,
                    is_stream: false,
                    is_snapshot: false,
                    default_volume: 1.0,
                    default_pitch: 1.0,
                    min_distance: 1.0,
                    max_distance: 20.0,
                    length_ms,
                    parameters: Vec::new(),
                });
            }
        } else {
            for (i, smp) in st.samples.iter().enumerate() {
                let path = if smp.name.is_empty() {
                    format!("event:/sample_{i}")
                } else if smp.name.starts_with("event:/") {
                    smp.name.clone()
                } else {
                    format!("event:/{}", smp.name.trim_start_matches('/'))
                };
                let guid = FmodGuid::from_le_bytes(guid_from_uid(&path));
                let length_ms = ((smp.num_frames as u64).saturating_mul(1000)
                    / (smp.sample_rate.max(1) as u64)) as i32;
                st.strings.push((guid, path.clone()));
                st.events.push(ParsedEvent {
                    guid,
                    bus_guid: None,
                    path,
                    sample_index: Some(i),
                    is_3d: false,
                    is_oneshot: !smp.has_loop,
                    is_stream: false,
                    is_snapshot: false,
                    default_volume: 1.0,
                    default_pitch: 1.0,
                    min_distance: 1.0,
                    max_distance: 20.0,
                    length_ms,
                    parameters: Vec::new(),
                });
            }
        }
    }

    Ok(ParsedBank {
        guid: st.bank_guid,
        has_explicit_guid: st.has_explicit_guid,
        path: bank_path,
        strings: st.strings,
        samples: st.samples,
        buses: st.buses,
        vcas: st.vcas,
        events: st.events,
        global_params,
    })
}
