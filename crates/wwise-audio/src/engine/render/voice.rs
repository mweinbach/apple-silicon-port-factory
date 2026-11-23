//! Voice instantiation, media loading, spatialization, fade envelopes, virtual-voice limiting,
//! and sample pulling.

use super::{voice_gain, ExternalSourceDesc, Fade, Pcm, Renderer, VState, Voice};
use crate::bnk::hirc::{prop, HircBody};
use crate::codec::decode_wem;
use crate::engine::filter::ConnectionFilter;
use crate::engine::fx::FxInstance;
use crate::engine::math::{fast_pow_to_db, FRAME, RATE};
use crate::engine::music::eval_clip_automations;
use crate::engine::params::{resolve, BusLink, InstanceLimit, Rng, VoiceParams};
use crate::engine::resample::{Mode, Resampler};
use crate::engine::spatial::{
    atten::{ray_gains, ray_geometry, slot as atten_slot, AttenModel, ConeOverride, ProjectCurves},
    positioning_matrix, Cfg, PanParams, Ray3d, SpeakerSetup,
};
use crate::engine::speakers::{direct_matrix, Layout};
use std::collections::HashMap;
use std::sync::Arc as Rc;

impl<'p> Renderer<'p> {
    pub fn refresh_voices_rtpc(&mut self) {
        let saved_obj = self.state.gs.game_object_id;
        let saved_pid = self.state.gs.active_playing_id;
        let mut dummy_rng = Rng::new(0x1234);
        for i in 0..self.state.voices.len() {
            if self.state.voices[i].state == VState::Done {
                continue;
            }
            let sid = self.state.voices[i].sound_id;
            let v_obj = self.state.voices[i].game_object_id;
            let v_pid = self.state.voices[i].playing_id;
            self.state.gs.sync_for_object(v_obj, v_pid);
            if let Ok(vp) = resolve(self.project, &self.state.gs, sid, &mut dummy_rng) {
                let pcm = self.state.voices[i].pcm.clone();
                let (matrix, ray_dry, ray_user_aux, ray_game_aux, ray_lpf, ray_hpf) = self.spatial_eval(&pcm, &vp);
                let out_bus_vol_db = self
                    .state
                    .gs
                    .obj_output_bus_volume_db
                    .get(&(v_obj, 0))
                    .copied()
                    .unwrap_or(0.0);
                let (folded_out_vol, folded_out_lpf, folded_out_hpf) =
                    vp.fold_passthrough_buses(self.state.opts.bus_fx);
                let v = &mut self.state.voices[i];
                v.base_lin = voice_gain(vp.volume_db, vp.makeup_db, folded_out_vol + out_bus_vol_db)
                    * v.pcm.akd_gain
                    * ray_dry;
                v.ray_user_aux = ray_user_aux;
                v.ray_game_aux = ray_game_aux;
                v.ray_lpf = ray_lpf;
                v.ray_hpf = ray_hpf;
                v.res.set_pitch(vp.pitch_cents, true);
                v.matrix = matrix;
                let conn_lpf = (vp.lpf + folded_out_lpf).max(ray_lpf).clamp(0.0, 100.0);
                let conn_hpf = (vp.hpf + folded_out_hpf).max(ray_hpf).clamp(0.0, 100.0);
                v.filter = (conn_lpf > 0.0 || conn_hpf > 0.0)
                    .then(|| ConnectionFilter::new(conn_lpf, conn_hpf, v.pcm.data.len(), RATE));
                let mut old_fx = std::mem::take(&mut v.node_fx);
                let mut new_fx = Vec::with_capacity(vp.node_fx.len());
                for &(slot, fx_id, _) in &vp.node_fx {
                    if let Some(pos) = old_fx.iter().position(|(s, inst)| *s == slot && inst.fx_id == fx_id) {
                        new_fx.push(old_fx.remove(pos));
                    } else if let Some(inst) = FxInstance::from_project(self.project, fx_id) {
                        new_fx.push((slot, inst));
                    }
                }
                v.node_fx = new_fx;
                v.params = vp;
            }
        }
        self.state.gs.game_object_id = saved_obj;
        self.state.gs.active_playing_id = saved_pid;
        self.state.bus_voices_seen = usize::MAX;
    }

    fn load_pcm(&mut self, sound_id: u32) -> Result<Rc<Pcm>, String> {
        let o = self.project.node(sound_id).unwrap();
        let HircBody::Sound(s) = &o.body else { unreachable!() };
        let src = s.source.clone();
        self.load_source_pcm(&src)
    }

    fn decode_wem_to_pcm(buf: &[u8]) -> Result<Rc<Pcm>, String> {
        let wem = crate::wem::Wem::parse(buf).map_err(|e| e.to_string())?;
        let d = decode_wem(buf)?;
        let ch = d.channels as usize;
        let frames = d.samples.len() / ch.max(1);
        let mut data = vec![Vec::with_capacity(frames); ch];
        for f in 0..frames {
            for c in 0..ch {
                data[c].push(d.samples[f * ch + c]);
            }
        }
        let layout = Layout::from_wem(wem.channels, wem.channel_config_type, wem.channel_mask);
        Ok(Rc::new(Pcm {
            rate: d.sample_rate,
            layout,
            data,
            loop_points: wem.loop_points,
            akd_gain: wem.akd_gain(),
        }))
    }

    fn resolve_external_wem_bytes(&self, ext: &ExternalSourceDesc) -> Result<Vec<u8>, String> {
        if let Some(b) = &ext.bytes {
            return Ok(b.clone());
        }
        if let Some(raw_path) = &ext.path {
            let trimmed = raw_path.trim().trim_matches('\0');
            if !trimmed.is_empty() {
                if std::path::Path::new(trimmed).is_file() {
                    if let Ok(b) = std::fs::read(trimmed) {
                        return Ok(b);
                    }
                }
                let mut norm = trimmed.replace('\\', "/").to_ascii_lowercase();
                while norm.starts_with('/') {
                    norm.remove(0);
                }
                if !norm.ends_with(".wem") {
                    norm.push_str(".wem");
                }
                if std::path::Path::new(&norm).is_file() {
                    if let Ok(b) = std::fs::read(&norm) {
                        return Ok(b);
                    }
                }
                if let Some(ga) = self.project.game.as_ref() {
                    for vpath in ga.external_candidates(&norm) {
                        if let Ok(b) = ga.read(&vpath) {
                            return Ok(b);
                        }
                    }
                }
            }
        }
        if ext.file_id != 0 {
            if let Some(ga) = self.project.game.as_ref() {
                let vpath = ga.media_vpath(ext.file_id);
                if let Ok(b) = ga.read(&vpath) {
                    return Ok(b);
                }
            }
        }
        Err(format!(
            "external_source cookie={:#x} path={:?} file_id={} not found",
            ext.cookie, ext.path, ext.file_id
        ))
    }

    pub(super) fn load_source_pcm(&mut self, src: &crate::bnk::hirc::Source) -> Result<Rc<Pcm>, String> {
        if src.plugin_id == 0x0008_0001 {
            let ext_opt = self
                .state
                .cur_ext_sources
                .iter()
                .find(|e| e.cookie == src.source_id || e.cookie == 0)
                .or_else(|| self.state.cur_ext_sources.first())
                .cloned();
            if let Some(ext) = ext_opt {
                let buf = self.resolve_external_wem_bytes(&ext)?;
                return Self::decode_wem_to_pcm(&buf);
            }
            if let Some(p) = self.state.pcm_cache.get(&src.source_id) {
                return Ok(p.clone());
            }
            return Err("external_source (dialogue, path supplied by the game at PostEvent)".into());
        }
        if let Some(p) = self.state.pcm_cache.get(&src.source_id) {
            return Ok(p.clone());
        }
        if src.plugin_type() != 1 {
            return Err(format!(
                "source plugin {} ({:#x})",
                crate::bnk::plugin_name(src.plugin_id),
                src.plugin_id
            ));
        }
        let loc = self
            .project
            .locate_media(src)
            .ok_or_else(|| format!("media {} not found", src.source_id))?;
        let buf = self.project.read_media(&loc)?;
        let p = Self::decode_wem_to_pcm(&buf)?;
        self.state.pcm_cache.insert(src.source_id, p.clone());
        Ok(p)
    }

    pub(super) fn start_sound(&mut self, id: u32, at: u64, fade: Option<Fade>) -> Result<Option<u64>, String> {
        if self.state.voices.len() >= 20_000 {
            return Err("more than 20000 voices scheduled".into());
        }
        let params = resolve(self.project, &self.state.gs, id, &mut self.state.rng)?;
        for n in &params.notes {
            self.state.notes.insert(n.clone());
        }
        let pcm = match self.load_pcm(id) {
            Ok(p) => p,
            Err(e) => {
                self.note(format!("unsupported_source: {}", e));
                return Ok(Some(at));
            }
        };
        let o = self.project.node(id).unwrap();
        let HircBody::Sound(s) = &o.body else { unreachable!() };
        // Loop prop (int): absent = play once, 0 = infinite, N = N plays [V: data scan].
        let loops = s.node.props.get_raw(prop::LOOP).unwrap_or(1);
        if s.node.ranged.0.iter().any(|(i, _, _)| *i == prop::LOOP) {
            self.note("random_loop");
        }
        let n = pcm.frames();
        let (ls, le) = match pcm.loop_points {
            Some((a, b)) if loops != 1 => (a as usize, (b as usize).min(n)),
            _ => (0, n),
        };
        let mut segments = vec![(0usize, le)];
        if loops == 0 {
            segments.push((ls, le)); // repeated forever
            self.note("infinite_loop");
        } else {
            for _ in 1..loops {
                segments.push((ls, le));
            }
            if le < n {
                segments.push((le, n));
            }
        }
        if pcm.loop_points.is_some() && loops != 1 {
            self.note("smpl_loop");
        }
        let ch = pcm.data.len();
        let mut res = Resampler::new(ch, pcm.rate, self.state.opts.resampler_fma);
        res.set_pitch(params.pitch_cents, false);
        if params.pitch_cents != 0.0 {
            self.note("pitch");
        }
        let total_in: u64 = segments.iter().map(|(a, b)| (b - a) as u64).sum();
        // initial delays were applied per played node in play_node
        let start = at;
        let out_len = if res.mode == Mode::Bypass {
            total_in
        } else {
            Resampler::output_len(total_in, res.step)
        };
        let end = if loops == 0 { None } else { Some(start + out_len) };
        let (matrix, ray_dry, ray_user_aux, ray_game_aux, ray_lpf, ray_hpf) = self.spatial_eval(&pcm, &params);
        let out_bus_vol_db = self
            .state
            .gs
            .obj_output_bus_volume_db
            .get(&(self.state.cur_game_obj, 0))
            .copied()
            .unwrap_or(0.0);
        let (folded_out_vol, folded_out_lpf, folded_out_hpf) =
            params.fold_passthrough_buses(self.state.opts.bus_fx);
        // the media's `akd ` gain and 3D ray dry attenuation multiply the voice gain (Wem::akd_gain)
        let base_lin =
            voice_gain(params.volume_db, params.makeup_db, folded_out_vol + out_bus_vol_db) * pcm.akd_gain * ray_dry;
        if pcm.akd_gain != 1.0 {
            self.note("akd_gain");
        }
        let conn_lpf = (params.lpf + folded_out_lpf).max(ray_lpf).clamp(0.0, 100.0);
        let conn_hpf = (params.hpf + folded_out_hpf).max(ray_hpf).clamp(0.0, 100.0);
        let filter = (conn_lpf > 0.0 || conn_hpf > 0.0).then(|| ConnectionFilter::new(conn_lpf, conn_hpf, ch, RATE));
        let mut chain = vec![];
        let mut nid = id;
        while nid != 0 && chain.len() < 64 {
            chain.push(nid);
            nid = self
                .project
                .node(nid)
                .and_then(|o| o.node_base())
                .map_or(0, |n| n.direct_parent_id);
        }
        let mut f_in = fade;
        if let Some(f) = f_in.as_mut() {
            if f.len == 0 {
                f_in = None;
            }
        }
        self.state.log.push(format!(
            "t={} sound {} src {} {}ch {}Hz {} frames loops={} vol={:.3}dB makeup={:.3}dB pitch={:.1}c step={} -> end {:?} bus {:?}",
            start,
            id,
            s.source.source_id,
            ch,
            pcm.rate,
            n,
            loops,
            params.volume_db,
            params.makeup_db,
            params.pitch_cents,
            res.step,
            end,
            params.buses.iter().map(|b| (b.id, b.bus_volume_db)).collect::<Vec<_>>()
        ));
        let node_fx: Vec<(u8, FxInstance)> = params
            .node_fx
            .iter()
            .filter_map(|&(slot, fx_id, _)| FxInstance::from_project(self.project, fx_id).map(|inst| (slot, inst)))
            .collect();
        self.state.voices.push(Voice {
            sound_id: id,
            playing_id: self.state.cur_playing_id,
            event_id: self.state.cur_event_id,
            game_object_id: self.state.cur_game_obj,
            chain,
            params,
            pcm,
            start,
            state: VState::Scheduled,
            evicted_by_limit: false,
            segments,
            seg: 0,
            seg_pos: 0,
            res,
            base_lin,
            ray_user_aux,
            ray_game_aux,
            ray_lpf,
            ray_hpf,
            prev_gain: None,
            aux_prev_gain: HashMap::new(),
            aux_filters: HashMap::new(),
            node_fx,
            fade_in: f_in,
            fade_out: None,
            stop_at: None,
            end,
            matrix,
            filter,
            pause_fade: None,
            clip_origin_sample: 0,
            clip_automations: Vec::new(),
        });
        Ok(end)
    }

    /// Layout of a bus: its own channel config, else its parent's ("same as parent"), the
    /// master bus falling back to the device layout (`out_channels`, stereo) [A].
    pub(super) fn bus_layout(&self, buses: &[BusLink], i: usize) -> Layout {
        for b in &buses[i..] {
            if let Some(l) = Layout::from_config(b.channel_config) {
                return l;
            }
        }
        self.device_layout()
    }

    pub(super) fn device_layout(&self) -> Layout {
        match self.state.opts.out_channels {
            1 => Layout::MONO,
            _ => Layout::STEREO,
        }
    }

    /// Voice -> output bus matrix [in][out], plus 3D ray dry/user_aux/game_aux gains and LPF/HPF.
    pub(super) fn spatial_eval(&mut self, pcm: &Pcm, vp: &VoiceParams) -> (Vec<Vec<f32>>, f32, f32, f32, f32, f32) {
        if vp.positioning.panner_type() != 0 {
            // balance-fade at centre equals direct assignment only for stereo outputs [V §1.2]
            self.note(format!("panner_type_{}", vp.positioning.panner_type()));
        }
        if self.state.opts.out_channels != 2 {
            self.note("non_stereo_output");
        }
        let out = self.bus_layout(&vp.buses, 0);
        if pcm.layout.channels > 2 {
            self.note(format!(
                "multichannel_{}ch_{:#x}",
                pcm.layout.channels, pcm.layout.mask
            ));
        }
        let (m2d, notes) = direct_matrix(pcm.layout, out, vp.pan.2 / 100.0);
        for n in notes {
            self.note(n);
        }
        let Some(emitter) = self.state.gs.emitter.as_ref() else {
            return (m2d, 1.0, 1.0, 1.0, 0.0, 0.0);
        };
        if !vp.is_3d() {
            return (m2d, 1.0, 1.0, 1.0, 0.0, 0.0);
        }
        let mut ray = ray_geometry(
            &self.state.gs.listener,
            self.state.gs.listener_scaling,
            emitter,
            self.state.gs.emitter_scaling,
        );
        ray.occlusion = self.state.gs.occlusion;
        ray.obstruction = self.state.gs.obstruction;
        ray.diffraction = self.state.gs.diffraction;
        ray.transmission = self.state.gs.transmission;
        let att = if vp.positioning.enable_attenuation() && vp.attenuation_id != 0 {
            self.project.attenuation(vp.attenuation_id).and_then(|o| match &o.body {
                HircBody::Attenuation(a) => Some(AttenModel::new(a)),
                _ => None,
            })
        } else {
            None
        };
        let proj = ProjectCurves::from_env(self.project.env_curves());
        let cone_ovr = ConeOverride {
            volume: vp.cone_volume,
            lpf: vp.cone_lpf,
            hpf: vp.cone_hpf,
        };
        ray_gains(&mut ray, att.as_ref(), &proj, cone_ovr, true);
        let spread = att
            .as_ref()
            .and_then(|a| a.eval_slot(atten_slot::SPREAD, ray.distance))
            .unwrap_or(0.0)
            * 0.01;
        let focus = att
            .as_ref()
            .and_then(|a| a.eval_slot(atten_slot::FOCUS, ray.distance))
            .unwrap_or(0.0)
            * 0.01;
        let ray3d = Ray3d {
            front: ray.front_l,
            top: ray.top_l,
            pos: ray.pos_l,
            spread: spread.clamp(0.0, 1.0),
            focus: focus.clamp(0.0, 1.0),
            mask: !0,
        };
        let pp = PanParams {
            center: vp.pan.2 * 0.01,
            pan_lr: vp.pan.0,
            pan_bf: vp.pan.1,
            pan_du: 0.0,
            spat_mix: (vp.spat_mix * 0.01).clamp(0.0, 1.0),
            spat_mode: vp.positioning.spatialization_mode(),
            panner_type: vp.positioning.panner_type(),
            height_spread: att.as_ref().is_some_and(|a| a.height_spread),
        };
        let setup = SpeakerSetup::aw2();
        let in_cfg = Cfg {
            n: pcm.layout.channels as u32,
            kind: pcm.layout.kind as u32,
            mask: pcm.layout.mask,
        };
        let out_cfg = Cfg {
            n: out.channels as u32,
            kind: out.kind as u32,
            mask: out.mask,
        };
        let m3d = positioning_matrix(&setup, &ray3d, &pp, in_cfg, out_cfg, Some(&m2d));
        (m3d, ray.dry, ray.user_aux, ray.game_aux, ray.lpf, ray.hpf)
    }

    /// Transition fade ratio (0..1) for the frame ending at `t`, modeling Wwise's single active
    /// `CAkTransition` handoff (`FUN_1415ada30` / `CAkTransitionManager::ChangeParameter`) [V].
    ///
    /// Precedence when a newer fade begins while an earlier fade is still running:
    /// - Once `fade_out` starts (`fo.elapsed(t) > 0`), it drives the envelope from its captured
    ///   starting ratio to `0.0` (superseding `fade_in` and `pause_fade`).
    /// - Otherwise, once `pause_fade` starts (`pf.elapsed(t) > 0`), it drives the envelope from
    ///   its captured starting ratio to `0.0` (pause) or `1.0` (resume), superseding `fade_in`.
    /// - Otherwise, `fade_in` drives `0.0 -> 1.0`.
    pub(super) fn fade_ratio(v: &Voice, t: u64) -> f32 {
        if let Some((fo, from)) = v.fade_out {
            if fo.elapsed(t) > 0 {
                return fo.value(from, 0.0, t);
            }
        }
        if let Some((pf, from, is_pause)) = v.pause_fade {
            if pf.elapsed(t) > 0 {
                return pf.value(from, if is_pause { 0.0 } else { 1.0 }, t);
            }
        }
        if let Some(fi) = v.fade_in {
            return fi.value(0.0, 1.0, t);
        }
        1.0
    }

    /// Play/stop/pause fade ratio plus MusicTrack `ClipAutomation` gain for the frame ending at `t`.
    pub(super) fn envelope(v: &Voice, t: u64) -> f32 {
        let mut g = Self::fade_ratio(v, t);
        if !v.clip_automations.is_empty() {
            let (auto_gain, _, _) = eval_clip_automations(&v.clip_automations, v.clip_origin_sample, t);
            g *= auto_gain;
        }
        if let Some(s) = v.stop_at {
            if t >= s {
                g = 0.0;
            }
        }
        g
    }

    /// Enforce `project.volume_threshold_db` (`BelowThresholdBehavior`) and `MaxNumInstance`
    /// voice limiting (`AdvSettings` / `Bus::max_num_instance`) for voices active in `[f0, f1)`.
    pub(super) fn enforce_voice_limits_and_thresholds(&mut self, f0: u64, f1: u64) {
        let thresh_db = self.project.volume_threshold_db;
        // 1. Activate scheduled voices starting in this frame and check volume threshold
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
            if v.state == VState::Scheduled {
                v.state = VState::Playing;
            }
            let btb = v.params.below_threshold_behavior;
            if btb != 0 {
                let eff_db = if v.base_lin <= 0.0 {
                    -96.3
                } else {
                    fast_pow_to_db(v.base_lin * v.base_lin)
                };
                if eff_db < thresh_db {
                    match btb {
                        1 => v.state = VState::Done, // KillVoice
                        2 => {
                            // SetAsVirtualVoice
                            if v.state != VState::Virtual {
                                v.state = VState::Virtual;
                                v.prev_gain = Some(0.0);
                                if v.params.virtual_queue_behavior == 0 {
                                    v.seg = 0;
                                    v.seg_pos = 0;
                                    v.res.reset();
                                }
                            }
                        }
                        3 => {
                            // KillIfOneShotElseVirtual
                            if v.end.is_some() {
                                v.state = VState::Done;
                            } else if v.state != VState::Virtual {
                                v.state = VState::Virtual;
                                v.prev_gain = Some(0.0);
                                if v.params.virtual_queue_behavior == 0 {
                                    v.seg = 0;
                                    v.seg_pos = 0;
                                    v.res.reset();
                                }
                            }
                        }
                        _ => {}
                    }
                } else if v.state == VState::Virtual && !v.evicted_by_limit {
                    // Volume rose back above threshold: wake from virtual
                    v.state = VState::Playing;
                    if v.params.virtual_queue_behavior == 0 {
                        v.seg = 0;
                        v.seg_pos = 0;
                        v.res.reset();
                    }
                }
            }
        }

        // 2. Collect unique InstanceLimits across active voices
        let mut limits: Vec<InstanceLimit> = Vec::new();
        for v in &self.state.voices {
            if v.state == VState::Done || v.state == VState::Paused || v.start >= f1 {
                continue;
            }
            for &lim in &v.params.instance_limits {
                if !limits
                    .iter()
                    .any(|l| l.scope_id == lim.scope_id && l.is_bus == lim.is_bus)
                {
                    limits.push(lim);
                }
            }
        }
        if limits.is_empty() {
            return;
        }

        // 3. Enforce each InstanceLimit (evicting lowest-priority or oldest/newest voices, or waking limit-evicted virtual voices)
        for lim in limits {
            let max_inst = lim.max_instances as usize;
            let in_scope = |v: &Voice| -> bool {
                if lim.is_bus {
                    v.params.buses.iter().any(|b| b.id == lim.scope_id)
                } else {
                    v.chain.contains(&lim.scope_id)
                }
            };
            loop {
                let playing_indices: Vec<usize> = self
                    .state
                    .voices
                    .iter()
                    .enumerate()
                    .filter(|(_, v)| v.state == VState::Playing && v.start < f1 && in_scope(v))
                    .map(|(i, _)| i)
                    .collect();
                if playing_indices.len() <= max_inst {
                    break;
                }
                // Pick victim: lowest effective priority, breaking ties by kill_newest (newest vs oldest)
                let mut victim = playing_indices[0];
                let mut victim_prio = self.state.voices[victim]
                    .params
                    .effective_priority(self.project, &self.state.gs);
                for &idx in &playing_indices[1..] {
                    let prio = self.state.voices[idx]
                        .params
                        .effective_priority(self.project, &self.state.gs);
                    let is_worse = if (prio - victim_prio).abs() > 1e-4 {
                        prio < victim_prio
                    } else if lim.kill_newest {
                        (self.state.voices[idx].start, idx) > (self.state.voices[victim].start, victim)
                    } else {
                        (self.state.voices[idx].start, idx) < (self.state.voices[victim].start, victim)
                    };
                    if is_worse {
                        victim = idx;
                        victim_prio = prio;
                    }
                }
                let v = &mut self.state.voices[victim];
                let btb = v.params.below_threshold_behavior;
                if lim.use_virtual_behavior && (btb == 0 || btb == 2 || (btb == 3 && v.end.is_none())) {
                    v.state = VState::Virtual;
                    v.evicted_by_limit = true;
                    v.prev_gain = Some(0.0);
                    if v.params.virtual_queue_behavior == 0 {
                        v.seg = 0;
                        v.seg_pos = 0;
                        v.res.reset();
                    }
                } else {
                    v.state = VState::Done;
                }
            }
            // Wake limit-evicted virtual voices if a slot opened up
            loop {
                let playing_count = self
                    .state
                    .voices
                    .iter()
                    .filter(|v| v.state == VState::Playing && v.start < f1 && in_scope(v))
                    .count();
                if playing_count >= max_inst {
                    break;
                }
                let candidate = self
                    .state
                    .voices
                    .iter()
                    .enumerate()
                    .filter(|(_, v)| v.state == VState::Virtual && v.evicted_by_limit && v.start < f1 && in_scope(v))
                    .max_by(|(ia, va), (ib, vb)| {
                        let pa = va.params.effective_priority(self.project, &self.state.gs);
                        let pb = vb.params.effective_priority(self.project, &self.state.gs);
                        pa.partial_cmp(&pb)
                            .unwrap_or(std::cmp::Ordering::Equal)
                            .then_with(|| ib.cmp(ia))
                    })
                    .map(|(i, _)| i);
                if let Some(idx) = candidate {
                    let v = &mut self.state.voices[idx];
                    v.state = VState::Playing;
                    v.evicted_by_limit = false;
                    if v.params.virtual_queue_behavior == 0 {
                        v.seg = 0;
                        v.seg_pos = 0;
                        v.res.reset();
                    }
                } else {
                    break;
                }
            }
        }
    }
}

pub(super) fn ms_to_samples(ms: f32) -> u64 {
    // AK converts transition/delay times to samples at the engine rate [A: rounding]
    ((ms.max(0.0) as f64) * RATE as f64 / 1000.0).round() as u64
}

/// Schedule a fade-out on `v`, capturing the voice's active fade ratio at the frame boundary
/// where `f` starts so mid-fade-in, mid-pause, or retargeted fade-out stops hand off continuously
/// (`CAkTransitionManager::ChangeParameter`, `FUN_1415ada30` [V]).
pub(super) fn start_fade_out(v: &mut Voice, f: Fade) {
    let prev_end = (f.start / FRAME as u64) * FRAME as u64;
    let from = if v.state == VState::Paused {
        0.0
    } else {
        Renderer::fade_ratio(v, prev_end)
    };
    v.fade_out = Some((f, from.clamp(0.0, 1.0)));
}

pub(super) fn stop_voice(v: &mut Voice, at: u64, len: u64, curve: u32) {
    if (v.state == VState::Scheduled && v.start >= at) || v.state == VState::Paused {
        v.state = VState::Done;
        return;
    }
    // 0 ms stop: the transition (0 ticks) is done at its first update, so the voice ramps to 0
    // over the action's frame via the mixer [V: FUN_1415ad0e0; ADVICE_A3 §2.2]
    let f = Fade { start: at, len, curve };
    start_fade_out(v, f);
    v.stop_at = Some(f.end());
}

/// Schedule a pause fade-out on `v`, capturing its active fade ratio at the action's frame
/// boundary so pausing mid-fade-in or mid-resume hands off smoothly without jumping to 1.0.
pub(super) fn pause_voice(v: &mut Voice, at: u64, len: u64, curve: u32) {
    let prev_end = (at / FRAME as u64) * FRAME as u64;
    let from = Renderer::fade_ratio(v, prev_end).clamp(0.0, 1.0);
    v.pause_fade = Some((Fade { start: at, len: len.max(FRAME as u64), curve }, from, true));
}

/// Schedule a resume fade-in on `v`, handling both fully paused voices (`from = 0.0`) and voices
/// resumed while a pause fade-out is still running (`from = current pause fade ratio`).
pub(super) fn resume_voice(v: &mut Voice, at: u64, len: u64, curve: u32) {
    if v.state == VState::Paused {
        v.state = VState::Playing;
        v.pause_fade = Some((Fade { start: at, len, curve }, 0.0, false));
    } else if v.state == VState::Playing && v.pause_fade.is_some_and(|(_, _, is_pause)| is_pause) {
        let prev_end = (at / FRAME as u64) * FRAME as u64;
        let from = Renderer::fade_ratio(v, prev_end).clamp(0.0, 1.0);
        v.pause_fade = Some((Fade { start: at, len, curve }, from, false));
    }
}

pub(super) fn seek_voice(v: &mut Voice, pos: usize) {
    // place the cursor in the first segment that contains `pos` (source frames from the start)
    let mut acc = 0usize;
    for (i, (a, b)) in v.segments.iter().enumerate() {
        let len = b - a;
        if pos < acc + len {
            v.seg = i;
            v.seg_pos = pos - acc;
            return;
        }
        acc += len;
    }
    v.seg = v.segments.len();
}

/// Pull `n` output frames of the voice into `buf[c][offset..]`, returns frames produced.
pub(super) fn pull(v: &mut Voice, buf: &mut [Vec<f32>], offset: usize, n: usize) -> usize {
    let mut produced = 0;
    let infinite_last = v.end.is_none();
    while produced < n {
        if v.seg >= v.segments.len() {
            if infinite_last && !v.segments.is_empty() {
                v.seg = v.segments.len() - 1; // loop the last (loop) segment forever
                v.seg_pos = 0;
            } else {
                break;
            }
        }
        let (a, b) = v.segments[v.seg];
        let s = a + v.seg_pos;
        if s >= b {
            v.seg += 1;
            v.seg_pos = 0;
            continue;
        }
        let input: Vec<&[f32]> = v.pcm.data.iter().map(|c| &c[s..b]).collect();
        let (consumed, p) = v.res.process(&input, buf, offset + produced, n - produced);
        v.seg_pos += consumed;
        produced += p;
        if consumed == 0 && p == 0 {
            break;
        }
    }
    produced
}
