//! Bus graph construction, auto-ducking, auxiliary routing, and frame mixing.

use super::voice::{ms_to_samples, pull};
use super::{bus_gain, BusNode, DuckState, Fade, Pending, Renderer, VState};
use crate::bnk::hirc::{ActionKind, HircBody};
use crate::engine::filter::ConnectionFilter;
use crate::engine::fx::FxInstance;
use crate::engine::math::{db_to_lin, FRAME, RATE};
use crate::engine::mixer::mix_channel;
use crate::engine::music::eval_clip_automations;
use crate::engine::params::{resolve_bus_chain, BusLink};
use crate::engine::speakers::{direct_matrix, LFE};
use std::collections::{BTreeSet, HashMap};

impl<'p> Renderer<'p> {
    fn compute_bus_order(&mut self) {
        if self.state.voices.len() == self.state.bus_voices_seen {
            return;
        }
        self.state.bus_voices_seen = self.state.voices.len();
        let mut seen: HashMap<u32, BusNode> = HashMap::new();
        let device = self.device_layout();
        let mut notes = vec![];
        let mut chains: Vec<Vec<BusLink>> = Vec::new();
        for v in &self.state.voices {
            chains.push(v.params.buses.clone());
            for &(aux_id, _, _, _) in &v.params.user_aux_sends {
                if let Ok(ch) = resolve_bus_chain(self.project, &self.state.gs, aux_id) {
                    chains.push(ch);
                }
            }
            if v.params.use_game_aux {
                for &(aux_id, _) in &self.state.gs.game_aux_sends {
                    if let Ok(ch) = resolve_bus_chain(self.project, &self.state.gs, aux_id) {
                        chains.push(ch);
                    }
                }
            }
        }
        for buses in &chains {
            let n = buses.len();
            for (i, b) in buses.iter().enumerate() {
                if seen.contains_key(&b.id) {
                    continue;
                }
                let parent = buses.get(i + 1).map_or(0, |p| p.id);
                let layout = self.bus_layout(buses, i);
                let parent_layout = if parent == 0 {
                    device
                } else {
                    self.bus_layout(buses, i + 1)
                };
                if b.panner_type != 0 && (b.pan.0 != 0.0 || b.pan.1 != 0.0) {
                    notes.push(format!("bus_panner_type_{}", b.panner_type));
                }
                let (m, ns) = direct_matrix(layout, parent_layout, b.center_pct / 100.0);
                notes.extend(ns.iter().map(|s| format!("bus_{}", s)));
                let mix_node = b.mix_node(self.state.opts.bus_fx);
                // Per CAkBus::IsMixingBus + sub_1415ca360 / sub_1415c9610:
                // - Mixing nodes absorb `BusVolume`, `OutputBusVolume`, and `OutputBusLPF`/`HPF`
                //   of any immediately following pass-through ancestor buses up to the next mixing bus.
                // - Pass-through buses themselves do NOT apply gain or connection filters (their
                //   params were folded into their child voice or child mixing bus).
                let (out_vol_db, out_lpf, out_hpf) = if mix_node {
                    let mut ov = b.output_bus_volume_db;
                    let mut ol = b.out_lpf;
                    let mut oh = b.out_hpf;
                    for anc in &buses[i + 1..] {
                        if anc.mix_node(self.state.opts.bus_fx) {
                            break;
                        }
                        ov += anc.bus_volume_db + anc.output_bus_volume_db;
                        ol += anc.out_lpf;
                        oh += anc.out_hpf;
                    }
                    (ov, ol, oh)
                } else {
                    (0.0, 0.0, 0.0)
                };
                seen.insert(
                    b.id,
                    BusNode {
                        id: b.id,
                        parent,
                        depth: n - 1 - i,
                        layout,
                        to_parent: m,
                        gain_db: b.bus_volume_db,
                        out_vol_db,
                        mix_node,
                        out_lpf,
                        out_hpf,
                        fx: b.fx.clone(),
                        ducks: b.ducks.clone(),
                        recovery_time_ms: b.recovery_time_ms,
                        max_duck_volume: b.max_duck_volume,
                    },
                );
            }
        }
        for n in notes {
            self.note(n);
        }
        let mut order: Vec<BusNode> = seen.into_values().collect();
        order.sort_by(|a, b| b.depth.cmp(&a.depth).then(a.id.cmp(&b.id)));
        self.state.bus_order = order;
    }

    pub(super) fn active(&self) -> bool {
        self.state.pending.iter().any(|p| self.pending_keeps_alive(p))
            || self.state.voices.iter().any(|v| v.state != VState::Done)
    }

    /// AK launches a delayed action one buffer tick before the frame that contains its due
    /// sample (the same one-frame lead as the delayed play fade in `play_node`), and an event
    /// whose last voice has ended completes in the frame that runs its last pending action.
    /// So a pending Stop/Pause/Seek/... due in frame `k` keeps the render alive only through
    /// frame `k - 1` [V: oracle lengths of 1716566570 (Stop at 168000 -> 328 frames),
    /// 786641639 (67200 -> 131) and 3084055277 (110400 -> 215)]. Plays always keep it alive
    /// (they start voices). Their effect on playing voices is unchanged: the transition math
    /// (`Fade`) already gives 0 elapsed ticks in that lead frame.
    pub(super) fn pending_keeps_alive(&self, p: &Pending) -> bool {
        let is_play = self
            .project
            .action(p.action)
            .is_some_and(|o| matches!(&o.body, HircBody::Action(a) if a.kind == ActionKind::Play));
        is_play || p.at / FRAME as u64 > self.state.now / FRAME as u64
    }

    /// Render one frame into `out` (interleaved, `out_channels`).
    pub fn render_frame(&mut self, out: &mut Vec<f32>) -> Result<(), String> {
        let f0 = self.state.now;
        let f1 = f0 + FRAME as u64;
        // actions due in this frame (sample-accurate scheduling inside the frame [A])
        while let Some(p) = self.state.pending.first().cloned() {
            if p.at >= f1 {
                break;
            }
            self.state.pending.remove(0);
            self.run_action(&p)?;
        }
        self.enforce_voice_limits_and_thresholds(f0, f1);
        self.compute_bus_order();
        let oc = self.state.opts.out_channels;
        let mut bus_buf: HashMap<u32, Vec<Vec<f32>>> = HashMap::new();
        for bn in &self.state.bus_order {
            bus_buf.insert(bn.id, vec![vec![0.0; FRAME]; bn.layout.channels]);
        }
        let mut active_buses: BTreeSet<u32> = BTreeSet::new();
        let fma = self.state.opts.mixer_fma;
        let start_ramp = self.state.opts.start_ramp;
        let end_ramp = self.state.opts.end_ramp;
        let mut meter_updates: Vec<(u32, f32)> = Vec::new();
        for v in self.state.voices.iter_mut() {
            if v.state == VState::Done || v.state == VState::Paused || v.start >= f1 {
                continue;
            }
            if let Some(s) = v.stop_at {
                if s <= f0 {
                    v.state = VState::Done;
                    continue;
                }
            }
            let offset = v.start.saturating_sub(f0) as usize;
            let ch = v.pcm.data.len();
            if v.state == VState::Virtual {
                match v.params.virtual_queue_behavior {
                    1 => {
                        // FromElapsedTime: advance cursor silently without mixing
                        let mut scratch = vec![vec![0.0f32; FRAME]; ch];
                        let produced = pull(v, &mut scratch, offset, FRAME - offset);
                        if produced < FRAME - offset || v.end.is_some_and(|e| e <= f1) {
                            v.state = VState::Done;
                        }
                    }
                    2 => {
                        // Resume: freeze cursor while virtual and extend planned end by elapsed virtual frame
                        if let Some(e) = v.end.as_mut() {
                            *e += (FRAME - offset) as u64;
                        }
                    }
                    _ => {
                        // FromBeginning: cursor will rewind to 0 on wake; extend planned end while virtual
                        if let Some(e) = v.end.as_mut() {
                            *e += (FRAME - offset) as u64;
                        }
                    }
                }
                continue;
            }
            v.state = VState::Playing;
            for b in &v.params.buses {
                active_buses.insert(b.id);
            }
            let mut vbuf_raw = vec![vec![0.0f32; FRAME]; ch];
            let produced = pull(v, &mut vbuf_raw, offset, FRAME - offset);
            if !v.node_fx.is_empty() {
                let lfe_ch = (v.pcm.layout.mask & LFE != 0).then(|| ch.saturating_sub(1));
                for (_, fx_inst) in v.node_fx.iter_mut() {
                    if let Some(upd) = fx_inst.process(self.project, &self.state.gs, &mut vbuf_raw, lfe_ch) {
                        meter_updates.push(upd);
                    }
                }
            }
            if !v.clip_automations.is_empty() {
                let (_, auto_lpf, auto_hpf) = eval_clip_automations(&v.clip_automations, v.clip_origin_sample, f1);
                if auto_lpf > 0.0 || auto_hpf > 0.0 {
                    let eff_lpf = (v.params.lpf + auto_lpf).max(v.ray_lpf).clamp(0.0, 100.0);
                    let eff_hpf = (v.params.hpf + auto_hpf).max(v.ray_hpf).clamp(0.0, 100.0);
                    v.filter = Some(ConnectionFilter::new(eff_lpf, eff_hpf, ch, RATE));
                }
            }
            let has_aux = !v.params.user_aux_sends.is_empty()
                || (v.params.use_game_aux && !self.state.gs.game_aux_sends.is_empty());
            let mut vbuf = if has_aux && v.filter.is_some() {
                vbuf_raw.clone()
            } else {
                std::mem::take(&mut vbuf_raw)
            };
            if let Some(flt) = v.filter.as_mut() {
                for (c, ch_buf) in vbuf.iter_mut().enumerate() {
                    flt.process(c, ch_buf);
                }
            }
            if has_aux && vbuf_raw.is_empty() {
                vbuf_raw = vbuf.clone();
            }
            // the source also ends when its last sample lands exactly on the frame boundary: AK
            // flags NoMoreData with that full frame and releases the voice (no extra frame) [V:
            // oracle lengths, e.g. event 1664992110 ends at 2525*512]
            let finished = produced < FRAME - offset || v.end.is_some_and(|e| e <= f1);
            let env_f0 = Renderer::envelope(v, f0);
            let env_f1 = Renderer::envelope(v, f1);
            // natural end inside the frame: no ramp in the oracle (see `RenderOptions::end_ramp`)
            let nominal = v.base_lin * env_f1;
            let target = if finished && end_ramp { 0.0 } else { nominal };
            let prev = match v.prev_gain {
                Some(p) => p,
                None if start_ramp => 0.0,
                // first frame: the fade ratio's previous value (0 for a fade-in) [V: 3586369275
                // frame 0 ramps 0 -> interp(1/N)]
                None => v.base_lin * env_f0,
            };
            // pause: once the pause fade completed, hold
            if let Some((f, _, true)) = v.pause_fade {
                if f1 >= f.end() {
                    v.state = VState::Paused;
                    v.prev_gain = Some(0.0);
                }
            }
            let bus = v.params.buses.first().map_or(0, |b| b.id);
            if let Some(bb) = bus_buf.get_mut(&bus) {
                // AkMixer: per (in, out) pair, gain ramps from prevMx*prevGain to nextMx*nextGain;
                // the matrix itself does not ramp (prev == next, ADVICE_A3 §2.1)
                for (ic, row) in v.matrix.iter().enumerate() {
                    for (o, &m) in row.iter().enumerate() {
                        if o < bb.len() {
                            mix_channel(&vbuf[ic], &mut bb[o], m * prev, m * target, fma);
                        }
                    }
                }
            }
            // Route auxiliary sends (user-defined 0..3 and game-defined) to their AuxBus buffers
            if has_aux {
                let voice_pre_dry = db_to_lin(v.params.volume_db) * db_to_lin(v.params.makeup_db) * v.pcm.akd_gain;
                let mut sends: Vec<(u32, f32, f32, f32, bool)> = v
                    .params
                    .user_aux_sends
                    .iter()
                    .map(|&(aid, vol, lpf, hpf)| (aid, vol, lpf, hpf, false))
                    .collect();
                if v.params.use_game_aux {
                    for &(aid, env_vol) in &self.state.gs.game_aux_sends {
                        sends.push((
                            aid,
                            env_vol + v.params.game_aux_volume_db,
                            v.params.game_aux_lpf,
                            v.params.game_aux_hpf,
                            true,
                        ));
                    }
                }
                for (aux_id, send_vol_db, send_lpf, send_hpf, is_game) in sends {
                    let Some(abuf) = bus_buf.get_mut(&aux_id) else { continue };
                    active_buses.insert(aux_id);
                    let ray_aux = if is_game { v.ray_game_aux } else { v.ray_user_aux };
                    let send_base = voice_pre_dry * db_to_lin(send_vol_db) * ray_aux;
                    let send_nom = send_base * env_f1;
                    let send_tgt = if finished && end_ramp { 0.0 } else { send_nom };
                    let send_prv = match v.aux_prev_gain.get(&aux_id).copied() {
                        Some(p) => p,
                        None if start_ramp => 0.0,
                        None => send_base * env_f0,
                    };
                    let eff_lpf = send_lpf.max(v.ray_lpf).clamp(0.0, 100.0);
                    let eff_hpf = send_hpf.max(v.ray_hpf).clamp(0.0, 100.0);
                    let mut sbuf;
                    let src_buf = if eff_lpf > 0.0 || eff_hpf > 0.0 {
                        sbuf = vbuf_raw.clone();
                        let flt = v
                            .aux_filters
                            .entry(aux_id)
                            .or_insert_with(|| ConnectionFilter::new(eff_lpf, eff_hpf, ch, RATE));
                        for (c, ch_buf) in sbuf.iter_mut().enumerate() {
                            flt.process(c, ch_buf);
                        }
                        &sbuf
                    } else {
                        &vbuf_raw
                    };
                    for (ic, row) in v.matrix.iter().enumerate() {
                        for (o, &m) in row.iter().enumerate() {
                            if o < abuf.len() {
                                mix_channel(&src_buf[ic], &mut abuf[o], m * send_prv, m * send_tgt, fma);
                            }
                        }
                    }
                    if v.state != VState::Paused {
                        v.aux_prev_gain.insert(aux_id, send_tgt);
                    }
                }
            }
            if v.state != VState::Paused {
                v.prev_gain = Some(target);
            }
            if finished || target == 0.0 && v.stop_at.is_some_and(|s| s <= f1) {
                v.state = VState::Done;
            }
            if v.fade_out.is_some_and(|(f, _)| f1 >= f.end()) {
                v.state = VState::Done;
            }
        }
        // Update bus auto-ducking states (`DuckInfo` on buses)
        for bn in &self.state.bus_order {
            if bn.ducks.is_empty() {
                continue;
            }
            let is_active = active_buses.contains(&bn.id);
            for d in &bn.ducks {
                let st = self.state.duck_states.entry((bn.id, d.bus_id)).or_insert(DuckState {
                    active: false,
                    current_db: 0.0,
                    from_db: 0.0,
                    to_db: 0.0,
                    trans_start: 0,
                    trans_len: 0,
                    curve: d.fade_curve as u32,
                });
                if is_active && !st.active {
                    st.active = true;
                    st.from_db = st.current_db;
                    st.to_db = d.duck_volume;
                    st.trans_start = f0;
                    st.trans_len = ms_to_samples(d.fade_out_ms.max(0) as f32);
                    st.curve = d.fade_curve as u32;
                } else if !is_active && st.active {
                    st.active = false;
                    st.from_db = st.current_db;
                    st.to_db = 0.0;
                    st.trans_start = f0 + ms_to_samples(bn.recovery_time_ms.max(0) as f32);
                    st.trans_len = ms_to_samples(d.fade_in_ms.max(0) as f32);
                    st.curve = d.fade_curve as u32;
                }
                if f1 <= st.trans_start {
                    st.current_db = st.from_db;
                } else if st.trans_len == 0 {
                    st.current_db = st.to_db;
                } else {
                    let fade = Fade {
                        start: st.trans_start,
                        len: st.trans_len,
                        curve: st.curve,
                    };
                    st.current_db = fade.value(st.from_db, st.to_db, f1);
                }
            }
        }
        // bus mix, children before parents: bus -> parent with direct-assignment matrix x bus volume
        let mut master = vec![vec![0.0f32; FRAME]; oc];
        for bn in &self.state.bus_order {
            let mut buf = bus_buf.remove(&bn.id).unwrap();
            if self.state.opts.bus_fx && !bn.fx.is_empty() {
                let lfe_ch = (bn.layout.mask & LFE != 0).then(|| buf.len().saturating_sub(1));
                for &(slot, fx_id, _) in &bn.fx {
                    let key = (bn.id, slot);
                    let needs_init = self
                        .state
                        .bus_fx_instances
                        .get(&key)
                        .map_or(true, |inst| inst.fx_id != fx_id);
                    if needs_init {
                        if let Some(inst) = FxInstance::from_project(self.project, fx_id) {
                            self.state.bus_fx_instances.insert(key, inst);
                        }
                    }
                    if let Some(inst) = self.state.bus_fx_instances.get_mut(&key) {
                        if let Some(upd) = inst.process(self.project, &self.state.gs, &mut buf, lfe_ch) {
                            meter_updates.push(upd);
                        }
                    }
                }
            }
            if bn.out_lpf != 0.0 || bn.out_hpf != 0.0 {
                // the connection filters each bus channel before the gain/matrix mix
                // (`FUN_14163dbc0`: LPF, HPF, then MixOneInNChannels) [V]
                let f = self
                    .state
                    .bus_filters
                    .entry(bn.id)
                    .or_insert_with(|| ConnectionFilter::new(bn.out_lpf, bn.out_hpf, buf.len(), RATE));
                for (c, ch) in buf.iter_mut().enumerate() {
                    f.process(c, ch);
                }
            }
            let mut duck_db: f32 = self
                .state
                .duck_states
                .iter()
                .filter(|((_, target_id), _)| *target_id == bn.id)
                .map(|(_, st)| st.current_db)
                .sum();
            if duck_db != 0.0 && bn.max_duck_volume < 0.0 {
                duck_db = duck_db.max(bn.max_duck_volume);
            }
            let mut g = bus_gain(bn.gain_db, bn.out_vol_db, bn.mix_node, bn.parent == 0);
            if duck_db != 0.0 {
                g *= db_to_lin(duck_db);
            }
            let gp = *self.state.bus_prev_gain.entry(bn.id).or_insert(g);
            self.state.bus_prev_gain.insert(bn.id, g);
            let dst = if bn.parent == 0 {
                &mut master
            } else {
                bus_buf.get_mut(&bn.parent).unwrap()
            };
            for (ic, row) in bn.to_parent.iter().enumerate() {
                for (o, &m) in row.iter().enumerate() {
                    if o < dst.len() {
                        mix_channel(&buf[ic], &mut dst[o], m * gp, m * g, fma);
                    }
                }
            }
        }
        if !meter_updates.is_empty() {
            let mut changed = false;
            for (param_id, val) in meter_updates {
                let prev = self.state.gs.params.get(&param_id).copied();
                if prev.map_or(true, |p| (p - val).abs() > 1e-4) {
                    self.state.gs.set_rtpc(param_id, val);
                    changed = true;
                }
            }
            if changed {
                self.refresh_voices_rtpc();
            }
        }
        for n in 0..FRAME {
            for c in 0..oc {
                out.push(master[c][n]);
            }
        }
        self.state.now = f1;
        Ok(())
    }

    /// Render until every voice has finished (or `max_seconds`).
    pub fn render(&mut self) -> Result<Vec<f32>, String> {
        let mut out = vec![];
        let horizon = self.horizon();
        while self.active() {
            if self.state.now >= horizon {
                self.note("truncated_at_max_seconds");
                break;
            }
            self.render_frame(&mut out)?;
        }
        for _ in 0..self.state.opts.tail_frames {
            self.render_frame(&mut out)?;
        }
        Ok(out)
    }
}
