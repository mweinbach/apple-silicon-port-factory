//! `AW2AKT01` binary trace replay driver for [`NativeSoundEngine`].

use super::NativeSoundEngine;
use crate::engine::render::ExternalSourceDesc;
use std::collections::HashMap;
use std::path::Path;

pub(super) const TRACE_MAGIC: &[u8; 8] = b"AW2AKT01";
const TRACE_HEADER_LEN: usize = 14;

#[derive(Debug, Clone)]
pub struct TraceReplayReport {
    pub records_total: usize,
    pub records_applied: usize,
    pub frames_rendered: u32,
    pub banks_loaded: usize,
    pub events_posted: usize,
    pub callbacks_fired: usize,
    pub samples: Vec<f32>,
}

struct TraceDec<'a> {
    b: &'a [u8],
    p: usize,
}

impl<'a> TraceDec<'a> {
    fn new(b: &'a [u8]) -> Self {
        Self { b, p: 0 }
    }
    fn take(&mut self, n: usize) -> Result<&'a [u8], String> {
        if self.p + n > self.b.len() {
            return Err(format!("trace payload truncated at {} (need {n})", self.p));
        }
        let s = &self.b[self.p..self.p + n];
        self.p += n;
        Ok(s)
    }
    fn u8(&mut self) -> Result<u8, String> {
        Ok(self.take(1)?[0])
    }
    fn bool(&mut self) -> Result<bool, String> {
        Ok(self.u8()? != 0)
    }
    fn u16(&mut self) -> Result<u16, String> {
        Ok(u16::from_le_bytes(self.take(2)?.try_into().unwrap()))
    }
    fn u32(&mut self) -> Result<u32, String> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }
    fn i32(&mut self) -> Result<i32, String> {
        Ok(self.u32()? as i32)
    }
    fn u64(&mut self) -> Result<u64, String> {
        Ok(u64::from_le_bytes(self.take(8)?.try_into().unwrap()))
    }
    fn f32(&mut self) -> Result<f32, String> {
        Ok(f32::from_bits(self.u32()?))
    }
    fn blob(&mut self) -> Result<&'a [u8], String> {
        let n = self.u32()? as usize;
        self.take(n)
    }
    fn wstr(&mut self) -> Result<String, String> {
        let n = self.u32()? as usize;
        let b = self.take(n * 2)?;
        let u16s: Vec<u16> = b.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
        Ok(String::from_utf16_lossy(&u16s))
    }
    fn ext(&mut self) -> Result<ExternalSourceDesc, String> {
        let cookie = self.u32()?;
        let codec = self.u32()?;
        let file_id = self.u32()?;
        let _mem_size = self.u32()?;
        let _has_mem = self.bool()?;
        let path_s = self.wstr()?;
        Ok(ExternalSourceDesc {
            cookie,
            codec,
            file_id,
            path: (!path_s.is_empty()).then_some(path_s),
            bytes: None,
        })
    }
}

impl NativeSoundEngine {
    /// Replay a compact `AW2AKT01` binary trace against this `NativeSoundEngine`.
    pub fn replay_trace(
        &self,
        trace_bytes: &[u8],
        banks_dir: &Path,
        tail_frames: usize,
    ) -> Result<TraceReplayReport, String> {
        if trace_bytes.len() < TRACE_MAGIC.len() || &trace_bytes[..TRACE_MAGIC.len()] != TRACE_MAGIC {
            return Err("not an AW2AKT01 trace (bad magic)".into());
        }
        let mut p = TRACE_MAGIC.len();
        let mut records_total = 0usize;
        let mut records_applied = 0usize;
        let mut banks_loaded = 0usize;
        let mut events_posted = 0usize;
        let mut samples = Vec::new();
        let mut pid_map: HashMap<u32, u32> = HashMap::new();

        while p + TRACE_HEADER_LEN <= trace_bytes.len() {
            let h = &trace_bytes[p..p + TRACE_HEADER_LEN];
            let op = h[0];
            let ret = u32::from_le_bytes(h[6..10].try_into().unwrap());
            let len = u32::from_le_bytes(h[10..14].try_into().unwrap()) as usize;
            if p + TRACE_HEADER_LEN + len > trace_bytes.len() {
                break;
            }
            let payload = &trace_bytes[p + TRACE_HEADER_LEN..p + TRACE_HEADER_LEN + len];
            p += TRACE_HEADER_LEN + len;
            records_total += 1;

            let mut d = TraceDec::new(payload);
            match op {
                // Init, Term, MusicInit, MusicTerm, RegisterGlobalCallback, RegisterBusVolumeCallback
                1..=6 | 47 | 48 | 60..=62 => {}
                10 => {
                    // RenderAudio
                    let allow_sync = d.bool().unwrap_or(true);
                    let f = self.render_audio(allow_sync)?;
                    samples.extend_from_slice(&f);
                    records_applied += 1;
                }
                11 => {
                    self.register_game_obj(d.u64()?, None);
                    records_applied += 1;
                }
                12 => {
                    self.unregister_game_obj(d.u64()?);
                    records_applied += 1;
                }
                13 => {
                    self.unregister_all_game_obj();
                    records_applied += 1;
                }
                14 => {
                    let obj = d.u64()?;
                    let raw = d.take(0x30)?;
                    let _flags = d.i32()?;
                    self.set_position_bytes(obj, raw);
                    records_applied += 1;
                }
                15 => {
                    let obj = d.u64()?;
                    let n = d.u16()? as usize;
                    let ty = d.i32()?;
                    let _flags = d.i32()?;
                    let raw = d.take(n * 0x30)?;
                    self.set_multiple_positions_bytes(obj, raw, n, ty);
                    records_applied += 1;
                }
                16 => {
                    let n = d.u32()? as usize;
                    let mut l = Vec::with_capacity(n);
                    for _ in 0..n {
                        l.push(d.u64()?);
                    }
                    self.set_default_listeners(&l);
                    records_applied += 1;
                }
                17 => {
                    self.set_distance_probe(d.u64()?, d.u64()?);
                    records_applied += 1;
                }
                18 => {
                    self.set_scaling_factor(d.u64()?, d.f32()?);
                    records_applied += 1;
                }
                19 => {
                    self.set_object_obstruction_and_occlusion(d.u64()?, d.u64()?, d.f32()?, d.f32()?);
                    records_applied += 1;
                }
                20 => {
                    let obj = d.u64()?;
                    let n = d.u32()? as usize;
                    let raw = d.take(n * 0x10)?;
                    self.set_game_object_aux_send_bytes(obj, raw, n);
                    records_applied += 1;
                }
                21 => {
                    self.set_game_object_output_bus_volume(d.u64()?, d.u64()?, d.f32()?);
                    records_applied += 1;
                }
                30 => {
                    self.set_rtpc_value(d.u32()?, d.f32()?, d.u64()?, d.i32()?, d.i32()?, d.bool()?);
                    records_applied += 1;
                }
                31 => {
                    let (id, val, pid, ms, curve, bypass) =
                        (d.u32()?, d.f32()?, d.u32()?, d.i32()?, d.i32()?, d.bool()?);
                    let mapped_pid = pid_map.get(&pid).copied().unwrap_or(pid);
                    self.set_rtpc_value_by_playing_id(id, val, mapped_pid, ms, curve, bypass);
                    records_applied += 1;
                }
                32 => {
                    self.reset_rtpc_value(d.u32()?, d.u64()?, d.i32()?, d.i32()?, d.bool()?);
                    records_applied += 1;
                }
                33 => {
                    self.set_switch(d.u32()?, d.u32()?, d.u64()?);
                    records_applied += 1;
                }
                34 => {
                    self.set_state(d.u32()?, d.u32()?);
                    records_applied += 1;
                }
                35 => {
                    self.post_trigger(d.u32()?, d.u64()?);
                    records_applied += 1;
                }
                40 => {
                    let (ev, obj, flags, _cb, cookie, custom_pid, n) = (
                        d.u32()?,
                        d.u64()?,
                        d.u32()?,
                        d.u64()?,
                        d.u64()?,
                        d.u32()?,
                        d.u32()? as usize,
                    );
                    let mut exts = Vec::with_capacity(n);
                    for _ in 0..n {
                        exts.push(d.ext()?);
                    }
                    let assigned = self.post_event(
                        ev,
                        obj,
                        flags,
                        None,
                        cookie,
                        exts,
                        if ret != 0 { ret } else { custom_pid },
                    );
                    if ret != 0 && assigned != 0 {
                        pid_map.insert(ret, assigned);
                    }
                    events_posted += 1;
                    records_applied += 1;
                }
                41 => {
                    let (ev, act, obj, ms, curve, pid) =
                        (d.u32()?, d.i32()?, d.u64()?, d.i32()?, d.i32()?, d.u32()?);
                    let mapped_pid = pid_map.get(&pid).copied().unwrap_or(pid);
                    self.execute_action_on_event(ev, act, obj, ms, curve, mapped_pid);
                    records_applied += 1;
                }
                42 => {
                    let name = String::from_utf8_lossy(d.blob()?).into_owned();
                    let (act, obj, ms, curve, pid) = (d.i32()?, d.u64()?, d.i32()?, d.i32()?, d.u32()?);
                    let mapped_pid = pid_map.get(&pid).copied().unwrap_or(pid);
                    self.execute_action_on_event_name(&name, act, obj, ms, curve, mapped_pid);
                    records_applied += 1;
                }
                43 => {
                    let (ev, obj, ms, near, pid) = (d.u32()?, d.u64()?, d.i32()?, d.bool()?, d.u32()?);
                    let mapped_pid = pid_map.get(&pid).copied().unwrap_or(pid);
                    self.seek_on_event(ev, obj, ms, near, mapped_pid);
                    records_applied += 1;
                }
                44 => {
                    let (ev, obj, pct, near, pid) = (d.u32()?, d.u64()?, d.f32()?, d.bool()?, d.u32()?);
                    let mapped_pid = pid_map.get(&pid).copied().unwrap_or(pid);
                    self.seek_on_event_percent(ev, obj, pct, near, mapped_pid);
                    records_applied += 1;
                }
                45 => {
                    self.stop_all(d.u64()?);
                    records_applied += 1;
                }
                46 => {
                    let (pid, ms, curve) = (d.u32()?, d.i32()?, d.i32()?);
                    let mapped_pid = pid_map.get(&pid).copied().unwrap_or(pid);
                    self.stop_playing_id(mapped_pid, ms, curve);
                    records_applied += 1;
                }
                50 | 51 => {
                    let (_ptr, size, id, hash) = (d.u64()?, d.u32()?, d.u32()?, d.u64()?);
                    let fname = format!("bank_{id:08x}_{size}_{hash:016x}.bnk");
                    let bank_path = banks_dir.join(&fname);
                    let bytes = std::fs::read(&bank_path)
                        .map_err(|e| format!("failed to read trace bank {}: {e}", bank_path.display()))?;
                    self.load_bank_bytes(fname, bytes)?;
                    banks_loaded += 1;
                    records_applied += 1;
                }
                52 => {
                    let (id, _ptr, _ty) = (d.u32()?, d.u64()?, d.u32()?);
                    self.unload_bank(id);
                    records_applied += 1;
                }
                _ => {}
            }
        }

        if tail_frames > 0 {
            let tail = self.render_frames(tail_frames)?;
            samples.extend_from_slice(&tail);
        }

        Ok(TraceReplayReport {
            records_total,
            records_applied,
            frames_rendered: self.frames_rendered(),
            banks_loaded,
            events_posted,
            callbacks_fired: self.callback_history().len(),
            samples,
        })
    }
}
