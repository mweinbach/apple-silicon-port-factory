//! Derived test banks for the A3 exit test (docs/AUDIO.md §8, "A3 status").
//!
//! [`patch_2d`] rewrites a bank so every object renders as a plain 2D sound, without changing
//! anything else. Per HIRC object that carries positioning/aux blocks (every `NodeBase`-derived
//! object: Sound, RanSeq, Switch, ActorMixer, Layer, music objects; and Bus / AuxBus):
//!
//! * positioning byte `uBitsPositioning`: clear bit 1 (`bHasListenerRelativeRouting`, i.e. 3D),
//!   bits 2-3 (`ePannerType` -> 0 DirectSpeakerAssignment) and bits 5-6 (`e3DPositionType` ->
//!   0); bit 0 (override parent) and the other bits are kept. The 3D block that follows when bits
//!   0 and 1 were set (`uBits3D` and the automation path) is dropped;
//! * aux byte: clear bit 3 (`bHasAux`) and drop the four user aux-send bus ids that follow it;
//!   the reflections aux bus id is set to 0. Other aux bits are kept.
//!
//! * AudioDevice (HIRC type 0x15) objects with an FX chain: set bit 4 (bypass all) of the
//!   chain's bypass byte. The System device of AW2's Init.bnk carries a Mastering Suite
//!   (plugin 0xba0003) in slot 3 that the A2 oracle's `SetOutputDeviceEffect(..., 0)` does not
//!   remove; with the bit set the device mix is the plain master-bus output.
//!
//! Object sizes and the HIRC chunk size are recomputed; every other byte of the bank (header,
//! media index, media data, other chunks, unparsed objects) is copied unchanged. The output is
//! a deterministic function of the input.

use super::{Bank, HircBody};
use crate::reader::Reader;

#[derive(Debug, Default, Clone)]
pub struct Patch2dStats {
    /// Objects whose positioning and aux blocks were rewritten.
    pub objects: usize,
    /// Of those, objects that had 3D (listener-relative routing) enabled.
    pub had_3d: usize,
    /// Objects that had a non-zero panner type.
    pub had_panner: usize,
    /// Objects that had user aux sends.
    pub had_aux: usize,
    /// Objects that had a reflections aux bus.
    pub had_reflections: usize,
    /// AudioDevice FX chains whose bypass-all bit was set.
    pub device_fx_bypassed: usize,
    /// HIRC objects copied unchanged because they failed to parse.
    pub unparsed: usize,
    pub bytes_in: usize,
    pub bytes_out: usize,
}

fn blocks(body: &HircBody) -> Option<(&super::hirc::Positioning, &super::hirc::AuxParams)> {
    match body {
        HircBody::Bus(b) | HircBody::AuxBus(b) => Some((&b.positioning, &b.aux)),
        _ => None,
    }
}

/// Rewrite `buf` as a 2D-only bank (see module docs).
pub fn patch_2d(buf: &[u8]) -> Result<(Vec<u8>, Patch2dStats), String> {
    let bank = Bank::parse(buf).map_err(|e| e.to_string())?;
    let mut st = Patch2dStats { bytes_in: buf.len(), unparsed: bank.hirc_errors.len(), ..Default::default() };
    // body offset -> (positioning, aux)
    let mut edits = std::collections::HashMap::new();
    // body offset -> absolute offset of the AudioDevice FX chain bypass byte
    let mut dev_bypass = std::collections::HashMap::new();
    for o in &bank.hirc {
        if let HircBody::AudioDevice(d) = &o.body {
            if let Some(pos) = d.slots.bypass_pos {
                dev_bypass.insert(o.offset, pos);
            }
        }
        let pa = match o.node_base() {
            Some(n) => Some((&n.positioning, &n.aux)),
            None => blocks(&o.body),
        };
        if let Some((p, a)) = pa {
            if p.span.1 != a.span.0 || p.span.0 < o.offset || a.span.1 > o.offset + o.size {
                return Err(format!("object {} (type {:#x}): positioning/aux blocks not contiguous", o.id, o.hirc_type));
            }
            edits.insert(o.offset, (p, a));
        }
    }
    let mut out = Vec::with_capacity(buf.len());
    let mut r = Reader::new(buf);
    while !r.is_empty() {
        let fourcc = r.fourcc().map_err(|e| e.to_string())?;
        let size = r.u32().map_err(|e| e.to_string())? as usize;
        let start = r.pos();
        let body = r.bytes(size).map_err(|e| e.to_string())?;
        out.extend_from_slice(&fourcc);
        if &fourcc != b"HIRC" {
            out.extend_from_slice(&(size as u32).to_le_bytes());
            out.extend_from_slice(body);
            continue;
        }
        let mut h = Vec::with_capacity(size);
        let mut c = Reader::new(body);
        let n = c.u32().map_err(|e| e.to_string())?;
        h.extend_from_slice(&n.to_le_bytes());
        for _ in 0..n {
            let t = c.u8().map_err(|e| e.to_string())?;
            let osize = c.u32().map_err(|e| e.to_string())? as usize;
            let abs = start + c.pos();
            let ob = c.bytes(osize).map_err(|e| e.to_string())?;
            let new_body = match edits.get(&abs) {
                None => {
                    let mut nb = ob.to_vec();
                    if let Some(&pos) = dev_bypass.get(&abs) {
                        st.device_fx_bypassed += 1;
                        nb[pos - abs] |= 0x10;
                    }
                    nb
                }
                Some((p, a)) => {
                    st.objects += 1;
                    if p.bits & 0x02 != 0 {
                        st.had_3d += 1;
                    }
                    if p.panner_type() != 0 {
                        st.had_panner += 1;
                    }
                    if a.aux_ids.is_some() {
                        st.had_aux += 1;
                    }
                    if a.reflections_aux_bus != 0 {
                        st.had_reflections += 1;
                    }
                    let (p0, p1) = (p.span.0 - abs, p.span.1 - abs);
                    let a1 = a.span.1 - abs;
                    let mut nb = Vec::with_capacity(osize);
                    nb.extend_from_slice(&ob[..p0]);
                    nb.push(p.bits & !(0x02 | 0x0C | 0x60));
                    nb.push(a.bits & !0x08);
                    nb.extend_from_slice(&0u32.to_le_bytes());
                    debug_assert!(p1 <= a1);
                    nb.extend_from_slice(&ob[a1..]);
                    nb
                }
            };
            h.push(t);
            h.extend_from_slice(&(new_body.len() as u32).to_le_bytes());
            h.extend_from_slice(&new_body);
        }
        if !c.is_empty() {
            h.extend_from_slice(&body[c.pos()..]);
        }
        out.extend_from_slice(&(h.len() as u32).to_le_bytes());
        out.extend_from_slice(&h);
    }
    st.bytes_out = out.len();
    // the result must parse, with the same objects and no 3D / aux left
    let b2 = Bank::parse(&out).map_err(|e| format!("patched bank does not parse: {}", e))?;
    if b2.hirc.len() != bank.hirc.len() {
        return Err(format!("patched bank: {} objects parsed, expected {}", b2.hirc.len(), bank.hirc.len()));
    }
    for o in &b2.hirc {
        if let HircBody::AudioDevice(d) = &o.body {
            if d.slots.bypass_pos.is_some() && d.slots.bypass_bits & 0x10 == 0 {
                return Err(format!("patched bank: AudioDevice {} FX not bypassed", o.id));
            }
        }
        if let Some(n) = o.node_base() {
            if n.positioning.bits & 0x6E != 0 || n.aux.aux_ids.is_some() {
                return Err(format!("patched bank: object {} still has 3D/panner/aux", o.id));
            }
        }
    }
    Ok((out, st))
}
