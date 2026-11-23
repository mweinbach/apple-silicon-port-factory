//! FMOD Studio (`fmodstudio.dll` / `fmodstudio64.dll`) runtime state and
//! implementation routines (System, Bank, EventDescription, EventInstance, Bus, VCA).

use super::bank::{ParsedBank, StudioParameterDef, guid_from_uid, parse_studio_bank};
use super::core::{
    CHANNELS, FMOD_2D, FMOD_3D, FMOD_ERR_EVENT_ALREADY_LOADED, FMOD_ERR_EVENT_NOTFOUND,
    FMOD_ERR_FILE_BAD, FMOD_ERR_FILE_NOTFOUND, FMOD_ERR_FORMAT, FMOD_ERR_INVALID_FLOAT,
    FMOD_ERR_INVALID_HANDLE, FMOD_ERR_INVALID_PARAM, FMOD_ERR_STUDIO_UNINITIALIZED,
    FMOD_ERR_TOOMANYCHANNELS, FMOD_ERR_TRUNCATED, FMOD_LOOP_BIDI, FMOD_LOOP_NORMAL, FMOD_LOOP_OFF,
    FMOD_OK, FMOD_STUDIO_EVENT_PROPERTY_MAX, FMOD_STUDIO_EVENT_PROPERTY_MAXIMUM_DISTANCE,
    FMOD_STUDIO_EVENT_PROPERTY_MINIMUM_DISTANCE, FMOD_STUDIO_LOAD_MEMORY,
    FMOD_STUDIO_LOAD_MEMORY_POINT, FMOD_STUDIO_LOADING_STATE_LOADED,
    FMOD_STUDIO_LOADING_STATE_UNLOADED, FMOD_STUDIO_PARAMETER_AUTOMATIC_DISTANCE,
    FMOD_STUDIO_PLAYBACK_PLAYING, FMOD_STUDIO_PLAYBACK_STOPPED, FMOD_STUDIO_PLAYBACK_SUSTAINING,
    FMOD_STUDIO_STOP_ALLOWFADEOUT, FMOD_STUDIO_STOP_IMMEDIATE, Fmod3DAttributes, FmodGuid,
    FmodStudioParameterDescription, FmodStudioParameterId, FmodVector, GROUPS, STUDIO_BANKS,
    STUDIO_BUSES, STUDIO_EVENTS, STUDIO_INSTANCES, STUDIO_VCAS, STUDIOS, create_core_system_arc,
    lookup_studio, lookup_studio_bank, lookup_studio_bus, lookup_studio_event,
    lookup_studio_instance, lookup_studio_vca, next_handle, read_opt, resolve_sound_path,
    system_init_impl, system_release_impl, system_update_impl, write_opt,
};
use super::dsp::DecodedSample;
use super::mixer::{
    Channel, ChannelGroup, FmodSystem, Listener3D, MixerState, group_has_ancestor,
    resolve_group_chain,
};
use crate::audio::format::{vec3_is_finite, vec3_len, vec3_sub};
use crate::strings::{from_ansi, wr, write_ansi_z};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

// ============================================================================
// String & handle-list helpers
// ============================================================================

fn intern_cstr(s: &str) -> *const u8 {
    static INTERNED: OnceLock<Mutex<HashMap<String, usize>>> = OnceLock::new();
    let map = INTERNED.get_or_init(|| Mutex::new(HashMap::new()));
    let mut guard = map.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(&ptr) = guard.get(s) {
        return ptr as *const u8;
    }
    let mut bytes = Vec::with_capacity(s.len() + 1);
    bytes.extend_from_slice(s.as_bytes());
    bytes.push(0);
    let leaked: &'static [u8] = Box::leak(bytes.into_boxed_slice());
    let ptr = leaked.as_ptr() as usize;
    guard.insert(s.to_string(), ptr);
    ptr as *const u8
}

fn normalize_studio_path(path: &str) -> String {
    let lower = path.trim().to_ascii_lowercase();
    if lower.is_empty() || lower == "bus:/" {
        return lower;
    }
    lower.trim_end_matches('/').to_string()
}

unsafe fn write_studio_path(
    src: &str,
    path_out: *mut u8,
    size: i32,
    retrieved_out: *mut i32,
) -> i32 {
    let needed = (src.len() + 1) as i32;
    write_opt(retrieved_out, needed);

    if size < 0 {
        return FMOD_ERR_INVALID_PARAM;
    }
    if path_out.is_null() {
        if size == 0 && !retrieved_out.is_null() {
            return FMOD_OK;
        }
        return FMOD_ERR_INVALID_PARAM;
    }
    if size == 0 {
        return FMOD_ERR_TRUNCATED;
    }

    write_ansi_z(path_out, size as usize, src);

    if (size as usize) < src.len() + 1 {
        FMOD_ERR_TRUNCATED
    } else {
        FMOD_OK
    }
}

unsafe fn write_handle_list(
    items: &[usize],
    array_out: *mut usize,
    capacity: i32,
    count_out: *mut i32,
) -> i32 {
    if capacity < 0 || (capacity > 0 && array_out.is_null()) {
        return FMOD_ERR_INVALID_PARAM;
    }
    let n = items.len().min(capacity as usize);
    for (i, &h) in items.iter().take(n).enumerate() {
        wr(array_out.add(i), h);
    }
    write_opt(count_out, n as i32);
    FMOD_OK
}

fn param_def_to_abi(def: &StudioParameterDef) -> FmodStudioParameterDescription {
    FmodStudioParameterDescription {
        name: intern_cstr(&def.name),
        id: def.id,
        minimum: def.minimum,
        maximum: def.maximum,
        defaultvalue: def.default_value,
        param_type: def.param_type,
        flags: def.flags,
        guid: def.guid,
    }
}

// ============================================================================
// Runtime state structs
// ============================================================================

#[derive(Clone, Copy)]
struct StudioListener {
    attributes: Fmod3DAttributes,
    attenuation_pos: Option<FmodVector>,
    weight: f32,
}

impl Default for StudioListener {
    fn default() -> Self {
        Self {
            attributes: Fmod3DAttributes::default(),
            attenuation_pos: None,
            weight: 1.0,
        }
    }
}

struct StudioBank {
    guid: FmodGuid,
    has_explicit_guid: bool,
    path: String,
    loading_state: u32,
    sample_loading_state: u32,
    strings: Vec<(FmodGuid, String)>,
    event_handles: Vec<usize>,
    bus_handles: Vec<usize>,
    vca_handles: Vec<usize>,
}

struct StudioBus {
    guid: FmodGuid,
    path: String,
    parent_guid: Option<FmodGuid>,
    channel_group_id: usize,
    volume: f32,
    mute: bool,
    paused: bool,
    #[allow(dead_code)]
    locked_group: bool,
    is_master: bool,
    ref_banks: Vec<usize>,
}

struct StudioVca {
    guid: FmodGuid,
    path: String,
    volume: f32,
    target_bus_guids: Vec<FmodGuid>,
    target_event_guids: Vec<FmodGuid>,
    ref_banks: Vec<usize>,
}

struct StudioEventDesc {
    bank_handle: usize,
    guid: FmodGuid,
    path: String,
    bus_guid: Option<FmodGuid>,
    sample: Option<DecodedSample>,
    is_3d: bool,
    is_oneshot: bool,
    is_stream: bool,
    is_snapshot: bool,
    has_sustain_point: bool,
    default_volume: f32,
    default_pitch: f32,
    min_distance: f32,
    max_distance: f32,
    sound_size: f32,
    length_ms: i32,
    parameters: Vec<StudioParameterDef>,
    sample_loading_state: u32,
    instances: Vec<usize>,
}

struct StudioEventInstance {
    desc_handle: usize,
    event_guid: FmodGuid,
    bank_handle: usize,
    channel_group_id: usize,
    channel_id: Option<usize>,
    volume: f32,
    pitch: f32,
    paused: bool,
    state: u32,
    attributes_3d: Fmod3DAttributes,
    min_distance: f32,
    max_distance: f32,
    timeline_ms: i32,
    reverb_levels: [f32; 4],
    properties: [f32; 6],
    param_defs: HashMap<String, StudioParameterDef>,
    param_id_to_key: HashMap<FmodStudioParameterId, String>,
    param_values: HashMap<String, f32>,
    sample: Option<DecodedSample>,
    is_3d: bool,
    is_oneshot: bool,
    desc_default_volume: f32,
    desc_default_pitch: f32,
}

struct OrphanInstance {
    bank_handle: usize,
    channel_group_id: usize,
    channel_id: usize,
}

struct StudioState {
    master_bus_handle: usize,
    master_bus_guid: FmodGuid,
    num_listeners: usize,
    listeners: [StudioListener; 8],
    banks: HashMap<usize, StudioBank>,
    bank_order: Vec<usize>,
    bank_by_guid: HashMap<FmodGuid, usize>,
    events: HashMap<usize, StudioEventDesc>,
    event_by_guid: HashMap<FmodGuid, usize>,
    instances: HashMap<usize, StudioEventInstance>,
    orphans: Vec<OrphanInstance>,
    buses: HashMap<usize, StudioBus>,
    bus_by_guid: HashMap<FmodGuid, usize>,
    vcas: HashMap<usize, StudioVca>,
    vca_by_guid: HashMap<FmodGuid, usize>,
    path_to_guid: HashMap<String, FmodGuid>,
    guid_to_path: HashMap<FmodGuid, String>,
    global_param_defs: HashMap<String, StudioParameterDef>,
    global_param_id_to_key: HashMap<FmodStudioParameterId, String>,
    global_param_values: HashMap<String, f32>,
}

pub(super) struct StudioSystem {
    core_id: usize,
    core_sys: Arc<FmodSystem>,
    state: Mutex<StudioState>,
}

impl StudioState {
    fn new(master_bus_handle: usize, master_group_id: usize) -> Self {
        let master_guid = FmodGuid::from_le_bytes(guid_from_uid("bus:/"));
        let mut buses = HashMap::new();
        buses.insert(
            master_bus_handle,
            StudioBus {
                guid: master_guid,
                path: "bus:/".into(),
                parent_guid: None,
                channel_group_id: master_group_id,
                volume: 1.0,
                mute: false,
                paused: false,
                locked_group: false,
                is_master: true,
                ref_banks: Vec::new(),
            },
        );
        let mut bus_by_guid = HashMap::new();
        bus_by_guid.insert(master_guid, master_bus_handle);
        let mut path_to_guid = HashMap::new();
        path_to_guid.insert("bus:/".into(), master_guid);
        path_to_guid.insert(String::new(), master_guid);
        let mut guid_to_path = HashMap::new();
        guid_to_path.insert(master_guid, "bus:/".into());

        Self {
            master_bus_handle,
            master_bus_guid: master_guid,
            num_listeners: 1,
            listeners: [StudioListener::default(); 8],
            banks: HashMap::new(),
            bank_order: Vec::new(),
            bank_by_guid: HashMap::new(),
            events: HashMap::new(),
            event_by_guid: HashMap::new(),
            instances: HashMap::new(),
            orphans: Vec::new(),
            buses,
            bus_by_guid,
            vcas: HashMap::new(),
            vca_by_guid: HashMap::new(),
            path_to_guid,
            guid_to_path,
            global_param_defs: HashMap::new(),
            global_param_id_to_key: HashMap::new(),
            global_param_values: HashMap::new(),
        }
    }

    fn rebuild_string_maps(&mut self) {
        self.path_to_guid.clear();
        self.guid_to_path.clear();

        self.path_to_guid.insert("bus:/".into(), self.master_bus_guid);
        self.path_to_guid.insert(String::new(), self.master_bus_guid);
        self.guid_to_path.insert(self.master_bus_guid, "bus:/".into());

        for &bid in &self.bank_order {
            if let Some(bank) = self.banks.get(&bid) {
                for (g, p) in &bank.strings {
                    if !p.is_empty() {
                        self.path_to_guid.insert(normalize_studio_path(p), *g);
                        self.guid_to_path.insert(*g, p.clone());
                    }
                }
            }
        }
        for bus in self.buses.values() {
            if !bus.path.is_empty() {
                self.path_to_guid
                    .entry(normalize_studio_path(&bus.path))
                    .or_insert(bus.guid);
                self.guid_to_path
                    .entry(bus.guid)
                    .or_insert_with(|| bus.path.clone());
            }
        }
        for vca in self.vcas.values() {
            if !vca.path.is_empty() {
                self.path_to_guid
                    .entry(normalize_studio_path(&vca.path))
                    .or_insert(vca.guid);
                self.guid_to_path
                    .entry(vca.guid)
                    .or_insert_with(|| vca.path.clone());
            }
        }
        for ev in self.events.values() {
            if !ev.path.is_empty() {
                self.path_to_guid
                    .entry(normalize_studio_path(&ev.path))
                    .or_insert(ev.guid);
                self.guid_to_path
                    .entry(ev.guid)
                    .or_insert_with(|| ev.path.clone());
            }
        }
    }

    fn resolve_path_or_guid(&self, path_or_id: &str) -> Option<FmodGuid> {
        if let Some(g) = FmodGuid::parse_str(path_or_id) {
            return Some(g);
        }
        let norm = normalize_studio_path(path_or_id);
        self.path_to_guid.get(&norm).copied()
    }

    fn bus_vca_multiplier(&self, bus_guid: FmodGuid) -> f32 {
        let mut mult = 1.0f32;
        for vca in self.vcas.values() {
            if vca.target_bus_guids.contains(&bus_guid) {
                mult *= vca.volume;
            }
        }
        mult
    }

    fn event_vca_multiplier(&self, event_guid: FmodGuid) -> f32 {
        let mut mult = 1.0f32;
        for vca in self.vcas.values() {
            if vca.target_event_guids.contains(&event_guid) {
                mult *= vca.volume;
            }
        }
        mult
    }

    fn sync_bus_and_instance_gains(&self, mixer: &mut MixerState) {
        for bus in self.buses.values() {
            let vca_mult = self.bus_vca_multiplier(bus.guid);
            if let Some(cg) = mixer.groups.get_mut(&bus.channel_group_id) {
                cg.volume = bus.volume * vca_mult;
                cg.mute = bus.mute;
                cg.paused = bus.paused;
            }
        }
        for inst in self.instances.values() {
            let ev_vca = self.event_vca_multiplier(inst.event_guid);
            if let Some(cg) = mixer.groups.get_mut(&inst.channel_group_id) {
                cg.volume = inst.volume * inst.desc_default_volume * ev_vca;
                cg.pitch = (inst.pitch * inst.desc_default_pitch).max(0.0);
                cg.paused = inst.paused;
            }
        }
    }

    fn relink_bus_parents(&self, mixer: &mut MixerState) {
        let master_cg = mixer.master_group_id;
        for bus in self.buses.values() {
            if bus.is_master {
                continue;
            }
            let parent_from_guid = bus
                .parent_guid
                .and_then(|pg| self.bus_by_guid.get(&pg).copied())
                .and_then(|bh| self.buses.get(&bh))
                .map(|pb| pb.channel_group_id);
            let parent_from_path = if parent_from_guid.is_none() && !bus.path.is_empty() {
                let norm = normalize_studio_path(&bus.path);
                norm.rsplit_once('/').and_then(|(prefix, _)| {
                    if prefix == "bus:" || prefix.is_empty() {
                        Some(master_cg)
                    } else {
                        self.buses
                            .values()
                            .find(|b| normalize_studio_path(&b.path) == prefix)
                            .map(|b| b.channel_group_id)
                    }
                })
            } else {
                None
            };
            let parent_cg = parent_from_guid.or(parent_from_path).unwrap_or(master_cg);
            if parent_cg != bus.channel_group_id
                && !group_has_ancestor(&mixer.groups, parent_cg, bus.channel_group_id)
            {
                if let Some(cg) = mixer.groups.get_mut(&bus.channel_group_id) {
                    cg.parent = Some(parent_cg);
                }
            }
        }
    }

    fn resolve_event_bus_cg(&self, bus_guid: Option<FmodGuid>, master_cg: usize) -> usize {
        bus_guid
            .and_then(|bg| self.bus_by_guid.get(&bg).copied())
            .and_then(|bh| self.buses.get(&bh))
            .map(|b| b.channel_group_id)
            .unwrap_or(master_cg)
    }

    fn compute_listener_distance(&self, pos: FmodVector) -> f32 {
        let lp = self.listeners[0]
            .attenuation_pos
            .unwrap_or(self.listeners[0].attributes.position);
        vec3_len(vec3_sub(pos, lp))
    }
}

fn load_parsed_bank_into_studio(
    studio_arc: &Arc<StudioSystem>,
    parsed: ParsedBank,
) -> Result<usize, i32> {
    let bank_handle = next_handle();
    let mut new_event_handles = Vec::new();
    let mut new_bus_handles = Vec::new();
    let mut new_vca_handles = Vec::new();
    let mut new_group_handles = Vec::new();

    {
        let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
        let mut mixer = studio_arc
            .core_sys
            .mixer
            .lock()
            .unwrap_or_else(|e| e.into_inner());

        if parsed.has_explicit_guid && st.bank_by_guid.contains_key(&parsed.guid) {
            return Err(FMOD_ERR_EVENT_ALREADY_LOADED);
        }

        let bank_guid = if parsed.has_explicit_guid {
            parsed.guid
        } else {
            FmodGuid::from_le_bytes(guid_from_uid(&format!("bank:{bank_handle:#x}:{}", parsed.path)))
        };

        // 1) Merge this bank's string entries first so bus/VCA/event paths can resolve cross-bank.
        for (g, p) in &parsed.strings {
            if !p.is_empty() {
                st.path_to_guid.insert(normalize_studio_path(p), *g);
                st.guid_to_path.insert(*g, p.clone());
            }
        }

        // Back-fill paths for previously loaded events/buses/VCAs whose strings just arrived.
        let g2p_snapshot = st.guid_to_path.clone();
        for ev in st.events.values_mut() {
            if ev.path.is_empty() {
                if let Some(p) = g2p_snapshot.get(&ev.guid) {
                    ev.path = p.clone();
                }
            }
        }
        for bus in st.buses.values_mut() {
            if bus.path.is_empty() {
                if let Some(p) = g2p_snapshot.get(&bus.guid) {
                    bus.path = p.clone();
                }
            }
        }
        for vca in st.vcas.values_mut() {
            if vca.path.is_empty() {
                if let Some(p) = g2p_snapshot.get(&vca.guid) {
                    vca.path = p.clone();
                }
            }
        }

        // 2) Register buses from BUSS chunks.
        let mut bank_bus_handles = Vec::new();
        for pb in parsed.buses {
            let resolved_path = if !pb.path.is_empty() {
                pb.path
            } else {
                st.guid_to_path.get(&pb.guid).cloned().unwrap_or_default()
            };
            let norm = normalize_studio_path(&resolved_path);
            let is_master_path = norm == "bus:/";

            if is_master_path {
                let mh = st.master_bus_handle;
                if !pb.guid.is_zero() {
                    st.bus_by_guid.insert(pb.guid, mh);
                }
                if let Some(mb) = st.buses.get_mut(&mh) {
                    mb.volume = pb.default_volume;
                    mb.mute = pb.mute;
                    mb.paused = pb.paused;
                    if !mb.ref_banks.contains(&bank_handle) {
                        mb.ref_banks.push(bank_handle);
                    }
                }
                bank_bus_handles.push(mh);
            } else if let Some(&existing_h) = st.bus_by_guid.get(&pb.guid) {
                if let Some(b) = st.buses.get_mut(&existing_h) {
                    if b.path.is_empty() && !resolved_path.is_empty() {
                        b.path = resolved_path;
                    }
                    if b.parent_guid.is_none() {
                        b.parent_guid = pb.parent_guid;
                    }
                    if !b.ref_banks.contains(&bank_handle) {
                        b.ref_banks.push(bank_handle);
                    }
                }
                bank_bus_handles.push(existing_h);
            } else {
                let bh = next_handle();
                let cg_id = next_handle();
                let master_cg = mixer.master_group_id;
                mixer.groups.insert(
                    cg_id,
                    ChannelGroup {
                        name: if resolved_path.is_empty() {
                            format!("bus:{bh:#x}")
                        } else {
                            resolved_path.clone()
                        },
                        parent: Some(master_cg),
                        volume: pb.default_volume,
                        mute: pb.mute,
                        paused: pb.paused,
                        pitch: 1.0,
                    },
                );
                new_group_handles.push(cg_id);
                st.buses.insert(
                    bh,
                    StudioBus {
                        guid: pb.guid,
                        path: resolved_path,
                        parent_guid: pb.parent_guid,
                        channel_group_id: cg_id,
                        volume: pb.default_volume,
                        mute: pb.mute,
                        paused: pb.paused,
                        locked_group: false,
                        is_master: false,
                        ref_banks: vec![bank_handle],
                    },
                );
                st.bus_by_guid.insert(pb.guid, bh);
                new_bus_handles.push(bh);
                bank_bus_handles.push(bh);
            }
        }

        st.relink_bus_parents(&mut mixer);

        // 3) Register VCAs from VCAS chunks.
        let mut bank_vca_handles = Vec::new();
        for pv in parsed.vcas {
            let resolved_path = if !pv.path.is_empty() {
                pv.path
            } else {
                st.guid_to_path.get(&pv.guid).cloned().unwrap_or_default()
            };
            if let Some(&existing_h) = st.vca_by_guid.get(&pv.guid) {
                if let Some(v) = st.vcas.get_mut(&existing_h) {
                    if v.path.is_empty() && !resolved_path.is_empty() {
                        v.path = resolved_path;
                    }
                    for bg in pv.target_bus_guids {
                        if !v.target_bus_guids.contains(&bg) {
                            v.target_bus_guids.push(bg);
                        }
                    }
                    for eg in pv.target_event_guids {
                        if !v.target_event_guids.contains(&eg) {
                            v.target_event_guids.push(eg);
                        }
                    }
                    if !v.ref_banks.contains(&bank_handle) {
                        v.ref_banks.push(bank_handle);
                    }
                }
                bank_vca_handles.push(existing_h);
            } else {
                let vh = next_handle();
                st.vcas.insert(
                    vh,
                    StudioVca {
                        guid: pv.guid,
                        path: resolved_path,
                        volume: pv.default_volume,
                        target_bus_guids: pv.target_bus_guids,
                        target_event_guids: pv.target_event_guids,
                        ref_banks: vec![bank_handle],
                    },
                );
                st.vca_by_guid.insert(pv.guid, vh);
                new_vca_handles.push(vh);
                bank_vca_handles.push(vh);
            }
        }

        // 4) Register global parameters.
        for gp in parsed.global_params {
            let key = gp.name.to_ascii_lowercase();
            let def_val = gp.default_value;
            st.global_param_id_to_key.insert(gp.id, key.clone());
            st.global_param_values.entry(key.clone()).or_insert(def_val);
            st.global_param_defs.insert(key, gp);
        }

        // 5) Register events from EVNT chunks (or synthesized FSB5 events).
        let mut bank_event_handles = Vec::new();
        for pe in parsed.events {
            let resolved_path = if !pe.path.is_empty() {
                pe.path
            } else {
                st.guid_to_path.get(&pe.guid).cloned().unwrap_or_default()
            };
            let sample = pe
                .sample_index
                .and_then(|idx| parsed.samples.get(idx))
                .cloned();
            let length_ms = if pe.length_ms > 0 {
                pe.length_ms
            } else if let Some(ref s) = sample {
                ((s.num_frames as u64).saturating_mul(1000) / (s.sample_rate.max(1) as u64)) as i32
            } else {
                0
            };
            let eh = next_handle();
            st.events.insert(
                eh,
                StudioEventDesc {
                    bank_handle,
                    guid: pe.guid,
                    path: resolved_path,
                    bus_guid: pe.bus_guid,
                    sample,
                    is_3d: pe.is_3d,
                    is_oneshot: pe.is_oneshot,
                    is_stream: pe.is_stream,
                    is_snapshot: pe.is_snapshot,
                    has_sustain_point: false,
                    default_volume: pe.default_volume,
                    default_pitch: pe.default_pitch,
                    min_distance: pe.min_distance,
                    max_distance: pe.max_distance,
                    sound_size: 0.0,
                    length_ms,
                    parameters: pe.parameters,
                    sample_loading_state: FMOD_STUDIO_LOADING_STATE_LOADED,
                    instances: Vec::new(),
                },
            );
            st.event_by_guid.insert(pe.guid, eh);
            new_event_handles.push(eh);
            bank_event_handles.push(eh);
        }

        // 6) Record the bank itself.
        let bank_path = if !parsed.path.is_empty() {
            parsed.path
        } else {
            st.guid_to_path.get(&bank_guid).cloned().unwrap_or_default()
        };
        st.banks.insert(
            bank_handle,
            StudioBank {
                guid: bank_guid,
                has_explicit_guid: parsed.has_explicit_guid,
                path: bank_path,
                loading_state: FMOD_STUDIO_LOADING_STATE_LOADED,
                sample_loading_state: FMOD_STUDIO_LOADING_STATE_LOADED,
                strings: parsed.strings,
                event_handles: bank_event_handles,
                bus_handles: bank_bus_handles,
                vca_handles: bank_vca_handles,
            },
        );
        st.bank_order.push(bank_handle);
        st.bank_by_guid.insert(bank_guid, bank_handle);
        st.rebuild_string_maps();
        st.sync_bus_and_instance_gains(&mut mixer);
    }

    // Publish all new handles in global handle tables without holding studio/mixer locks.
    STUDIO_BANKS.insert_at(bank_handle, studio_arc.clone());
    for eh in new_event_handles {
        STUDIO_EVENTS.insert_at(eh, studio_arc.clone());
    }
    for bh in new_bus_handles {
        STUDIO_BUSES.insert_at(bh, studio_arc.clone());
    }
    for vh in new_vca_handles {
        STUDIO_VCAS.insert_at(vh, studio_arc.clone());
    }
    for cg in new_group_handles {
        GROUPS.insert_at(cg, studio_arc.core_sys.clone());
    }

    Ok(bank_handle)
}

fn unload_bank_impl(studio_arc: &Arc<StudioSystem>, bank_handle: usize) -> i32 {
    let mut rm_events = Vec::new();
    let mut rm_instances = Vec::new();
    let mut rm_buses = Vec::new();
    let mut rm_vcas = Vec::new();
    let mut rm_groups = Vec::new();
    let mut rm_channels = Vec::new();

    {
        let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
        let Some(bank) = st.banks.remove(&bank_handle) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        let mut mixer = studio_arc
            .core_sys
            .mixer
            .lock()
            .unwrap_or_else(|e| e.into_inner());

        st.bank_order.retain(|&h| h != bank_handle);
        if bank.has_explicit_guid || st.bank_by_guid.get(&bank.guid) == Some(&bank_handle) {
            st.bank_by_guid.remove(&bank.guid);
        }

        // Stop & remove all live instances owned by this bank.
        let inst_ids: Vec<usize> = st
            .instances
            .iter()
            .filter(|(_, inst)| inst.bank_handle == bank_handle)
            .map(|(&id, _)| id)
            .collect();
        for id in inst_ids {
            if let Some(inst) = st.instances.remove(&id) {
                rm_instances.push(id);
                if let Some(cid) = inst.channel_id {
                    let _ = mixer.channels.remove(&cid);
                    rm_channels.push(cid);
                }
                mixer.groups.remove(&inst.channel_group_id);
                rm_groups.push(inst.channel_group_id);
            }
        }

        // Stop & remove any fire-and-forget orphan instances owned by this bank.
        let mut kept_orphans = Vec::new();
        for orph in st.orphans.drain(..) {
            if orph.bank_handle == bank_handle {
                mixer.channels.remove(&orph.channel_id);
                mixer.groups.remove(&orph.channel_group_id);
                rm_channels.push(orph.channel_id);
                rm_groups.push(orph.channel_group_id);
            } else {
                kept_orphans.push(orph);
            }
        }
        st.orphans = kept_orphans;

        // Remove all EventDescriptions owned by this bank.
        for eh in bank.event_handles {
            if let Some(ev) = st.events.remove(&eh) {
                if st.event_by_guid.get(&ev.guid) == Some(&eh) {
                    st.event_by_guid.remove(&ev.guid);
                }
                rm_events.push(eh);
            }
        }

        // Decrement bank refs on non-master buses and remove unreferenced ones.
        let master_cg = mixer.master_group_id;
        for bh in bank.bus_handles {
            let remove_bus = if let Some(b) = st.buses.get_mut(&bh) {
                b.ref_banks.retain(|&id| id != bank_handle);
                !b.is_master && b.ref_banks.is_empty()
            } else {
                false
            };
            if remove_bus {
                if let Some(b) = st.buses.remove(&bh) {
                    st.bus_by_guid.retain(|_, v| *v != bh);
                    mixer.groups.remove(&b.channel_group_id);
                    for g in mixer.groups.values_mut() {
                        if g.parent == Some(b.channel_group_id) {
                            g.parent = Some(master_cg);
                        }
                    }
                    rm_buses.push(bh);
                    rm_groups.push(b.channel_group_id);
                }
            }
        }

        // Decrement bank refs on VCAs and remove unreferenced ones.
        for vh in bank.vca_handles {
            let remove_vca = if let Some(v) = st.vcas.get_mut(&vh) {
                v.ref_banks.retain(|&id| id != bank_handle);
                v.ref_banks.is_empty()
            } else {
                false
            };
            if remove_vca {
                if let Some(_v) = st.vcas.remove(&vh) {
                    st.vca_by_guid.retain(|_, val| *val != vh);
                    rm_vcas.push(vh);
                }
            }
        }

        st.rebuild_string_maps();
        st.relink_bus_parents(&mut mixer);
        st.sync_bus_and_instance_gains(&mut mixer);
    }

    STUDIO_BANKS.remove(bank_handle);
    for id in rm_events {
        STUDIO_EVENTS.remove(id);
    }
    for id in rm_instances {
        STUDIO_INSTANCES.remove(id);
    }
    for id in rm_buses {
        STUDIO_BUSES.remove(id);
    }
    for id in rm_vcas {
        STUDIO_VCAS.remove(id);
    }
    for id in rm_groups {
        GROUPS.remove(id);
    }
    for id in rm_channels {
        CHANNELS.remove(id);
    }
    FMOD_OK
}

// ============================================================================
// FMOD Studio — Utility & System implementations
// ============================================================================

pub(super) unsafe fn parse_id(idstring: *const u8, id_out: *mut FmodGuid) -> i32 {
    if idstring.is_null() || id_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let s = from_ansi(idstring);
    let Some(guid) = FmodGuid::parse_str(&s) else {
        return FMOD_ERR_FORMAT;
    };
    write_opt(id_out, guid);
    FMOD_OK
}

pub(super) unsafe fn system_create(system_out: *mut usize, _header_version: u32) -> i32 {
    if system_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let (core_id, master_group_id, core_sys) = create_core_system_arc();
    let studio_id = next_handle();
    let master_bus_handle = next_handle();
    let studio_arc = Arc::new(StudioSystem {
        core_id,
        core_sys,
        state: Mutex::new(StudioState::new(master_bus_handle, master_group_id)),
    });
    STUDIOS.insert_at(studio_id, studio_arc.clone());
    STUDIO_BUSES.insert_at(master_bus_handle, studio_arc);
    write_opt(system_out, studio_id);
    FMOD_OK
}

pub(super) fn system_is_valid(system: usize) -> i32 {
    if lookup_studio(system).is_ok() { 1 } else { 0 }
}

pub(super) fn system_initialize(
    system: usize,
    maxchannels: i32,
    _studioflags: u32,
    flags: u32,
    extradriverdata: usize,
) -> i32 {
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    system_init_impl(studio_arc.core_id, maxchannels, flags, extradriverdata)
}

pub(super) fn system_update(system: usize) -> i32 {
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };

    let mut rm_groups = Vec::new();
    let mut rm_channels = Vec::new();
    {
        let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
        let mut mixer = studio_arc
            .core_sys
            .mixer
            .lock()
            .unwrap_or_else(|e| e.into_inner());

        // Sync playback state & timeline positions for active event instances.
        for inst in st.instances.values_mut() {
            if let Some(cid) = inst.channel_id {
                if let Some(ch) = mixer.channels.get(&cid) {
                    inst.timeline_ms = ((ch.play_pos.max(0.0) * 1000.0)
                        / (ch.sound_sample_rate.max(1) as f64))
                        as i32;
                    if ch.stopped && inst.state == FMOD_STUDIO_PLAYBACK_PLAYING {
                        inst.state = FMOD_STUDIO_PLAYBACK_STOPPED;
                    }
                } else if inst.state == FMOD_STUDIO_PLAYBACK_PLAYING {
                    inst.state = FMOD_STUDIO_PLAYBACK_STOPPED;
                    inst.channel_id = None;
                }
            }
        }

        // Reap released fire-and-forget instances whose Core channel has finished playing.
        let mut alive_orphans = Vec::new();
        for orph in st.orphans.drain(..) {
            let finished = mixer
                .channels
                .get(&orph.channel_id)
                .map(|c| c.stopped)
                .unwrap_or(true);
            if finished {
                mixer.channels.remove(&orph.channel_id);
                mixer.groups.remove(&orph.channel_group_id);
                rm_channels.push(orph.channel_id);
                rm_groups.push(orph.channel_group_id);
            } else {
                alive_orphans.push(orph);
            }
        }
        st.orphans = alive_orphans;
    }

    for g in rm_groups {
        GROUPS.remove(g);
    }
    for c in rm_channels {
        CHANNELS.remove(c);
    }

    system_update_impl(studio_arc.core_id)
}

pub(super) fn system_release(system: usize) -> i32 {
    let Some(studio_arc) = STUDIOS.remove(system) else {
        return FMOD_ERR_INVALID_HANDLE;
    };

    let (bank_ids, event_ids, inst_ids, bus_ids, vca_ids) = {
        let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
        let b: Vec<usize> = st.banks.keys().copied().collect();
        let e: Vec<usize> = st.events.keys().copied().collect();
        let i: Vec<usize> = st.instances.keys().copied().collect();
        let bu: Vec<usize> = st.buses.keys().copied().collect();
        let v: Vec<usize> = st.vcas.keys().copied().collect();
        st.banks.clear();
        st.bank_order.clear();
        st.bank_by_guid.clear();
        st.events.clear();
        st.event_by_guid.clear();
        st.instances.clear();
        st.orphans.clear();
        st.buses.clear();
        st.bus_by_guid.clear();
        st.vcas.clear();
        st.vca_by_guid.clear();
        (b, e, i, bu, v)
    };
    for id in bank_ids {
        STUDIO_BANKS.remove(id);
    }
    for id in event_ids {
        STUDIO_EVENTS.remove(id);
    }
    for id in inst_ids {
        STUDIO_INSTANCES.remove(id);
    }
    for id in bus_ids {
        STUDIO_BUSES.remove(id);
    }
    for id in vca_ids {
        STUDIO_VCAS.remove(id);
    }

    system_release_impl(studio_arc.core_id)
}

pub(super) unsafe fn system_get_core_system(system: usize, coresystem_out: *mut usize) -> i32 {
    if coresystem_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => {
            write_opt(coresystem_out, 0);
            return e;
        }
    };
    write_opt(coresystem_out, studio_arc.core_id);
    FMOD_OK
}

pub(super) unsafe fn system_load_bank_file(
    system: usize,
    filename: *const u8,
    _flags: u32,
    bank_out: *mut usize,
) -> i32 {
    write_opt(bank_out, 0);
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    if filename.is_null() || bank_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let raw_path = from_ansi(filename);
    let Some(host_path) = resolve_sound_path(&raw_path) else {
        return FMOD_ERR_FILE_NOTFOUND;
    };
    let bytes = match std::fs::read(&host_path) {
        Ok(b) => b,
        Err(_) => return FMOD_ERR_FILE_BAD,
    };
    let parsed = match parse_studio_bank(&bytes, &raw_path) {
        Ok(p) => p,
        Err(e) => return e,
    };
    match load_parsed_bank_into_studio(&studio_arc, parsed) {
        Ok(h) => {
            write_opt(bank_out, h);
            FMOD_OK
        }
        Err(e) => e,
    }
}

pub(super) unsafe fn system_load_bank_memory(
    system: usize,
    buffer: *const u8,
    length: i32,
    mode: u32,
    _flags: u32,
    bank_out: *mut usize,
) -> i32 {
    write_opt(bank_out, 0);
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    if buffer.is_null() || length <= 0 || bank_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    if mode != FMOD_STUDIO_LOAD_MEMORY && mode != FMOD_STUDIO_LOAD_MEMORY_POINT {
        return FMOD_ERR_INVALID_PARAM;
    }
    let slice = std::slice::from_raw_parts(buffer, length as usize);
    let parsed = match parse_studio_bank(slice, "") {
        Ok(p) => p,
        Err(e) => return e,
    };
    match load_parsed_bank_into_studio(&studio_arc, parsed) {
        Ok(h) => {
            write_opt(bank_out, h);
            FMOD_OK
        }
        Err(e) => e,
    }
}

pub(super) fn system_unload_all(system: usize) -> i32 {
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let bank_ids: Vec<usize> = {
        let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
        st.bank_order.clone()
    };
    for bid in bank_ids {
        let _ = unload_bank_impl(&studio_arc, bid);
    }
    FMOD_OK
}

pub(super) fn system_flush_commands(system: usize) -> i32 {
    match lookup_studio(system) {
        Ok(_) => FMOD_OK,
        Err(e) => e,
    }
}

pub(super) fn system_flush_sample_loading(system: usize) -> i32 {
    match lookup_studio(system) {
        Ok(_) => FMOD_OK,
        Err(e) => e,
    }
}

pub(super) unsafe fn system_get_bank(
    system: usize,
    path: *const u8,
    bank_out: *mut usize,
) -> i32 {
    write_opt(bank_out, 0);
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    if path.is_null() || bank_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let raw = from_ansi(path);
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(g) = st.resolve_path_or_guid(&raw) {
        if let Some(&h) = st.bank_by_guid.get(&g) {
            write_opt(bank_out, h);
            return FMOD_OK;
        }
    }
    let norm = normalize_studio_path(&raw);
    for (&h, bank) in &st.banks {
        if normalize_studio_path(&bank.path) == norm {
            write_opt(bank_out, h);
            return FMOD_OK;
        }
    }
    FMOD_ERR_EVENT_NOTFOUND
}

pub(super) unsafe fn system_get_bank_by_id(
    system: usize,
    id: *const FmodGuid,
    bank_out: *mut usize,
) -> i32 {
    write_opt(bank_out, 0);
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let Some(guid) = read_opt(id) else {
        return FMOD_ERR_INVALID_PARAM;
    };
    if bank_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(&h) = st.bank_by_guid.get(&guid) else {
        return FMOD_ERR_EVENT_NOTFOUND;
    };
    write_opt(bank_out, h);
    FMOD_OK
}

pub(super) unsafe fn system_get_bank_count(system: usize, count_out: *mut i32) -> i32 {
    if count_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    write_opt(count_out, st.banks.len() as i32);
    FMOD_OK
}

pub(super) unsafe fn system_get_bank_list(
    system: usize,
    array: *mut usize,
    capacity: i32,
    count_out: *mut i32,
) -> i32 {
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    write_handle_list(&st.bank_order, array, capacity, count_out)
}

pub(super) unsafe fn system_get_event(
    system: usize,
    path: *const u8,
    event_out: *mut usize,
) -> i32 {
    write_opt(event_out, 0);
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    if path.is_null() || event_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let raw = from_ansi(path);
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(g) = st.resolve_path_or_guid(&raw) {
        if let Some(&h) = st.event_by_guid.get(&g) {
            write_opt(event_out, h);
            return FMOD_OK;
        }
    }
    let norm = normalize_studio_path(&raw);
    for (&h, ev) in &st.events {
        if !ev.path.is_empty() && normalize_studio_path(&ev.path) == norm {
            write_opt(event_out, h);
            return FMOD_OK;
        }
    }
    FMOD_ERR_EVENT_NOTFOUND
}

pub(super) unsafe fn system_get_event_by_id(
    system: usize,
    id: *const FmodGuid,
    event_out: *mut usize,
) -> i32 {
    write_opt(event_out, 0);
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let Some(guid) = read_opt(id) else {
        return FMOD_ERR_INVALID_PARAM;
    };
    if event_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(&h) = st.event_by_guid.get(&guid) else {
        return FMOD_ERR_EVENT_NOTFOUND;
    };
    write_opt(event_out, h);
    FMOD_OK
}

pub(super) unsafe fn system_get_bus(system: usize, path: *const u8, bus_out: *mut usize) -> i32 {
    write_opt(bus_out, 0);
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    if path.is_null() || bus_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let raw = from_ansi(path);
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let norm = normalize_studio_path(&raw);
    if norm.is_empty() || norm == "bus:/" {
        write_opt(bus_out, st.master_bus_handle);
        return FMOD_OK;
    }
    if let Some(g) = st.resolve_path_or_guid(&raw) {
        if let Some(&h) = st.bus_by_guid.get(&g) {
            write_opt(bus_out, h);
            return FMOD_OK;
        }
    }
    for (&h, b) in &st.buses {
        if !b.path.is_empty() && normalize_studio_path(&b.path) == norm {
            write_opt(bus_out, h);
            return FMOD_OK;
        }
    }
    FMOD_ERR_EVENT_NOTFOUND
}

pub(super) unsafe fn system_get_bus_by_id(
    system: usize,
    id: *const FmodGuid,
    bus_out: *mut usize,
) -> i32 {
    write_opt(bus_out, 0);
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let Some(guid) = read_opt(id) else {
        return FMOD_ERR_INVALID_PARAM;
    };
    if bus_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(&h) = st.bus_by_guid.get(&guid) else {
        return FMOD_ERR_EVENT_NOTFOUND;
    };
    write_opt(bus_out, h);
    FMOD_OK
}

pub(super) unsafe fn system_get_vca(system: usize, path: *const u8, vca_out: *mut usize) -> i32 {
    write_opt(vca_out, 0);
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    if path.is_null() || vca_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let raw = from_ansi(path);
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(g) = st.resolve_path_or_guid(&raw) {
        if let Some(&h) = st.vca_by_guid.get(&g) {
            write_opt(vca_out, h);
            return FMOD_OK;
        }
    }
    let norm = normalize_studio_path(&raw);
    for (&h, v) in &st.vcas {
        if !v.path.is_empty() && normalize_studio_path(&v.path) == norm {
            write_opt(vca_out, h);
            return FMOD_OK;
        }
    }
    FMOD_ERR_EVENT_NOTFOUND
}

pub(super) unsafe fn system_get_vca_by_id(
    system: usize,
    id: *const FmodGuid,
    vca_out: *mut usize,
) -> i32 {
    write_opt(vca_out, 0);
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let Some(guid) = read_opt(id) else {
        return FMOD_ERR_INVALID_PARAM;
    };
    if vca_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(&h) = st.vca_by_guid.get(&guid) else {
        return FMOD_ERR_EVENT_NOTFOUND;
    };
    write_opt(vca_out, h);
    FMOD_OK
}

pub(super) unsafe fn system_lookup_id(
    system: usize,
    path: *const u8,
    id_out: *mut FmodGuid,
) -> i32 {
    if path.is_null() || id_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let raw = from_ansi(path);
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(g) = st.resolve_path_or_guid(&raw) else {
        return FMOD_ERR_EVENT_NOTFOUND;
    };
    write_opt(id_out, g);
    FMOD_OK
}

pub(super) unsafe fn system_lookup_path(
    system: usize,
    id: *const FmodGuid,
    path_out: *mut u8,
    size: i32,
    retrieved_out: *mut i32,
) -> i32 {
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let Some(guid) = read_opt(id) else {
        return FMOD_ERR_INVALID_PARAM;
    };
    let found = {
        let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
        st.guid_to_path.get(&guid).cloned()
    };
    let Some(p) = found else {
        return FMOD_ERR_EVENT_NOTFOUND;
    };
    write_studio_path(&p, path_out, size, retrieved_out)
}

pub(super) fn system_set_num_listeners(system: usize, numlisteners: i32) -> i32 {
    if !(1..=8).contains(&numlisteners) {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    studio_arc
        .state
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .num_listeners = numlisteners as usize;
    FMOD_OK
}

pub(super) unsafe fn system_get_num_listeners(system: usize, numlisteners_out: *mut i32) -> i32 {
    if numlisteners_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let n = studio_arc
        .state
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .num_listeners;
    write_opt(numlisteners_out, n as i32);
    FMOD_OK
}

pub(super) unsafe fn system_set_listener_attributes(
    system: usize,
    listener: i32,
    attributes: *const Fmod3DAttributes,
    attenuationposition: *const FmodVector,
) -> i32 {
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let Some(attrs) = read_opt(attributes) else {
        return FMOD_ERR_INVALID_PARAM;
    };
    let atten_opt = read_opt(attenuationposition);
    for v in [attrs.position, attrs.velocity, attrs.forward, attrs.up]
        .into_iter()
        .chain(atten_opt)
    {
        if !vec3_is_finite(v) {
            return FMOD_ERR_INVALID_FLOAT;
        }
    }
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    if listener < 0 || (listener as usize) >= st.num_listeners {
        return FMOD_ERR_INVALID_PARAM;
    }
    st.listeners[listener as usize].attributes = attrs;
    st.listeners[listener as usize].attenuation_pos = atten_opt;
    if listener == 0 {
        let mut mixer = studio_arc
            .core_sys
            .mixer
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        mixer.listener = Listener3D {
            pos: atten_opt.unwrap_or(attrs.position),
            vel: attrs.velocity,
            forward: attrs.forward,
            up: attrs.up,
        };
    }
    FMOD_OK
}

pub(super) unsafe fn system_get_listener_attributes(
    system: usize,
    listener: i32,
    attributes_out: *mut Fmod3DAttributes,
    attenuationposition_out: *mut FmodVector,
) -> i32 {
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    if listener < 0 || (listener as usize) >= st.num_listeners {
        return FMOD_ERR_INVALID_PARAM;
    }
    let l = st.listeners[listener as usize];
    write_opt(attributes_out, l.attributes);
    if !attenuationposition_out.is_null() {
        write_opt(
            attenuationposition_out,
            l.attenuation_pos.unwrap_or(l.attributes.position),
        );
    }
    FMOD_OK
}

pub(super) fn system_set_listener_weight_impl(system: usize, listener: i32, weight: f32) -> i32 {
    if !weight.is_finite() || weight < 0.0 {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    if listener < 0 || (listener as usize) >= st.num_listeners {
        return FMOD_ERR_INVALID_PARAM;
    }
    st.listeners[listener as usize].weight = weight;
    FMOD_OK
}

pub(super) unsafe fn system_get_listener_weight(
    system: usize,
    listener: i32,
    weight_out: *mut f32,
) -> i32 {
    if weight_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    if listener < 0 || (listener as usize) >= st.num_listeners {
        return FMOD_ERR_INVALID_PARAM;
    }
    write_opt(weight_out, st.listeners[listener as usize].weight);
    FMOD_OK
}

pub(super) fn system_set_param_by_name_impl(
    system: usize,
    name: *const u8,
    value: f32,
    _ignoreseekspeed: i32,
) -> i32 {
    if name.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    if !value.is_finite() {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let key = unsafe { from_ansi(name) }.to_ascii_lowercase();
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(def) = st.global_param_defs.get(&key) else {
        return FMOD_ERR_EVENT_NOTFOUND;
    };
    let clamped = if def.maximum > def.minimum {
        value.clamp(def.minimum, def.maximum)
    } else {
        value
    };
    st.global_param_values.insert(key, clamped);
    FMOD_OK
}

pub(super) unsafe fn system_get_param_by_name(
    system: usize,
    name: *const u8,
    value_out: *mut f32,
    finalvalue_out: *mut f32,
) -> i32 {
    if name.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let key = from_ansi(name).to_ascii_lowercase();
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(&val) = st.global_param_values.get(&key) else {
        return FMOD_ERR_EVENT_NOTFOUND;
    };
    write_opt(value_out, val);
    write_opt(finalvalue_out, val);
    FMOD_OK
}

pub(super) fn system_set_param_by_id_impl(
    system: usize,
    id: u64,
    value: f32,
    _ignoreseekspeed: i32,
) -> i32 {
    if !value.is_finite() {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let pid = FmodStudioParameterId::from_u64(id);
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(key) = st.global_param_id_to_key.get(&pid).cloned() else {
        return FMOD_ERR_EVENT_NOTFOUND;
    };
    let Some(def) = st.global_param_defs.get(&key) else {
        return FMOD_ERR_EVENT_NOTFOUND;
    };
    let clamped = if def.maximum > def.minimum {
        value.clamp(def.minimum, def.maximum)
    } else {
        value
    };
    st.global_param_values.insert(key, clamped);
    FMOD_OK
}

pub(super) unsafe fn system_get_param_by_id(
    system: usize,
    id: u64,
    value_out: *mut f32,
    finalvalue_out: *mut f32,
) -> i32 {
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let pid = FmodStudioParameterId::from_u64(id);
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(key) = st.global_param_id_to_key.get(&pid) else {
        return FMOD_ERR_EVENT_NOTFOUND;
    };
    let Some(&val) = st.global_param_values.get(key) else {
        return FMOD_ERR_EVENT_NOTFOUND;
    };
    write_opt(value_out, val);
    write_opt(finalvalue_out, val);
    FMOD_OK
}

pub(super) unsafe fn system_get_param_desc_by_name(
    system: usize,
    name: *const u8,
    parameter_out: *mut FmodStudioParameterDescription,
) -> i32 {
    if name.is_null() || parameter_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let key = from_ansi(name).to_ascii_lowercase();
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(def) = st.global_param_defs.get(&key) else {
        return FMOD_ERR_EVENT_NOTFOUND;
    };
    write_opt(parameter_out, param_def_to_abi(def));
    FMOD_OK
}

pub(super) unsafe fn system_get_param_desc_by_id(
    system: usize,
    id: u64,
    parameter_out: *mut FmodStudioParameterDescription,
) -> i32 {
    if parameter_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let pid = FmodStudioParameterId::from_u64(id);
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(key) = st.global_param_id_to_key.get(&pid) else {
        return FMOD_ERR_EVENT_NOTFOUND;
    };
    let Some(def) = st.global_param_defs.get(key) else {
        return FMOD_ERR_EVENT_NOTFOUND;
    };
    write_opt(parameter_out, param_def_to_abi(def));
    FMOD_OK
}

pub(super) unsafe fn system_get_param_desc_count(system: usize, count_out: *mut i32) -> i32 {
    if count_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    write_opt(count_out, st.global_param_defs.len() as i32);
    FMOD_OK
}

// ============================================================================
// FMOD Studio — Bank implementations
// ============================================================================

pub(super) fn bank_is_valid(bank: usize) -> i32 {
    let Ok(studio_arc) = lookup_studio_bank(bank) else {
        return 0;
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    if st.banks.contains_key(&bank) { 1 } else { 0 }
}

pub(super) unsafe fn bank_get_id(bank: usize, id_out: *mut FmodGuid) -> i32 {
    if id_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_bank(bank) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.banks.get(&bank) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(id_out, b.guid);
    FMOD_OK
}

pub(super) unsafe fn bank_get_path(
    bank: usize,
    path_out: *mut u8,
    size: i32,
    retrieved_out: *mut i32,
) -> i32 {
    let studio_arc = match lookup_studio_bank(bank) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let path = {
        let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
        let Some(b) = st.banks.get(&bank) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        if !b.path.is_empty() {
            b.path.clone()
        } else {
            st.guid_to_path.get(&b.guid).cloned().unwrap_or_default()
        }
    };
    write_studio_path(&path, path_out, size, retrieved_out)
}

pub(super) fn bank_unload(bank: usize) -> i32 {
    let studio_arc = match lookup_studio_bank(bank) {
        Ok(s) => s,
        Err(e) => return e,
    };
    unload_bank_impl(&studio_arc, bank)
}

pub(super) fn bank_load_sample_data(bank: usize) -> i32 {
    let studio_arc = match lookup_studio_bank(bank) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let evs = {
        let Some(b) = st.banks.get_mut(&bank) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        b.sample_loading_state = FMOD_STUDIO_LOADING_STATE_LOADED;
        b.event_handles.clone()
    };
    for eh in evs {
        if let Some(ev) = st.events.get_mut(&eh) {
            ev.sample_loading_state = FMOD_STUDIO_LOADING_STATE_LOADED;
        }
    }
    FMOD_OK
}

pub(super) fn bank_unload_sample_data(bank: usize) -> i32 {
    let studio_arc = match lookup_studio_bank(bank) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let evs = {
        let Some(b) = st.banks.get_mut(&bank) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        b.sample_loading_state = FMOD_STUDIO_LOADING_STATE_UNLOADED;
        b.event_handles.clone()
    };
    for eh in evs {
        if let Some(ev) = st.events.get_mut(&eh) {
            ev.sample_loading_state = FMOD_STUDIO_LOADING_STATE_UNLOADED;
        }
    }
    FMOD_OK
}

pub(super) unsafe fn bank_get_loading_state(bank: usize, state_out: *mut u32) -> i32 {
    if state_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_bank(bank) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.banks.get(&bank) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(state_out, b.loading_state);
    FMOD_OK
}

pub(super) unsafe fn bank_get_sample_loading_state(bank: usize, state_out: *mut u32) -> i32 {
    if state_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_bank(bank) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.banks.get(&bank) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(state_out, b.sample_loading_state);
    FMOD_OK
}

pub(super) unsafe fn bank_get_string_count(bank: usize, count_out: *mut i32) -> i32 {
    if count_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_bank(bank) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.banks.get(&bank) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(count_out, b.strings.len() as i32);
    FMOD_OK
}

pub(super) unsafe fn bank_get_string_info(
    bank: usize,
    index: i32,
    id_out: *mut FmodGuid,
    path_out: *mut u8,
    size: i32,
    retrieved_out: *mut i32,
) -> i32 {
    if index < 0 {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_bank(bank) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let entry = {
        let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
        let Some(b) = st.banks.get(&bank) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        b.strings.get(index as usize).cloned()
    };
    let Some((guid, path)) = entry else {
        return FMOD_ERR_INVALID_PARAM;
    };
    write_opt(id_out, guid);
    if path_out.is_null() && size == 0 && retrieved_out.is_null() {
        return FMOD_OK;
    }
    write_studio_path(&path, path_out, size, retrieved_out)
}

pub(super) unsafe fn bank_get_event_count(bank: usize, count_out: *mut i32) -> i32 {
    if count_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_bank(bank) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.banks.get(&bank) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(count_out, b.event_handles.len() as i32);
    FMOD_OK
}

pub(super) unsafe fn bank_get_event_list(
    bank: usize,
    array: *mut usize,
    capacity: i32,
    count_out: *mut i32,
) -> i32 {
    let studio_arc = match lookup_studio_bank(bank) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.banks.get(&bank) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_handle_list(&b.event_handles, array, capacity, count_out)
}

pub(super) unsafe fn bank_get_bus_count(bank: usize, count_out: *mut i32) -> i32 {
    if count_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_bank(bank) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.banks.get(&bank) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(count_out, b.bus_handles.len() as i32);
    FMOD_OK
}

pub(super) unsafe fn bank_get_bus_list(
    bank: usize,
    array: *mut usize,
    capacity: i32,
    count_out: *mut i32,
) -> i32 {
    let studio_arc = match lookup_studio_bank(bank) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.banks.get(&bank) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_handle_list(&b.bus_handles, array, capacity, count_out)
}

pub(super) unsafe fn bank_get_vca_count(bank: usize, count_out: *mut i32) -> i32 {
    if count_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_bank(bank) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.banks.get(&bank) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(count_out, b.vca_handles.len() as i32);
    FMOD_OK
}

pub(super) unsafe fn bank_get_vca_list(
    bank: usize,
    array: *mut usize,
    capacity: i32,
    count_out: *mut i32,
) -> i32 {
    let studio_arc = match lookup_studio_bank(bank) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.banks.get(&bank) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_handle_list(&b.vca_handles, array, capacity, count_out)
}

// ============================================================================
// FMOD Studio — EventDescription implementations
// ============================================================================

pub(super) fn event_desc_is_valid(event_desc: usize) -> i32 {
    let Ok(studio_arc) = lookup_studio_event(event_desc) else {
        return 0;
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    if st.events.contains_key(&event_desc) { 1 } else { 0 }
}

pub(super) unsafe fn event_desc_get_id(event_desc: usize, id_out: *mut FmodGuid) -> i32 {
    if id_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(id_out, ev.guid);
    FMOD_OK
}

pub(super) unsafe fn event_desc_get_path(
    event_desc: usize,
    path_out: *mut u8,
    size: i32,
    retrieved_out: *mut i32,
) -> i32 {
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let path = {
        let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
        let Some(ev) = st.events.get(&event_desc) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        if !ev.path.is_empty() {
            ev.path.clone()
        } else {
            st.guid_to_path.get(&ev.guid).cloned().unwrap_or_default()
        }
    };
    write_studio_path(&path, path_out, size, retrieved_out)
}

pub(super) unsafe fn event_desc_get_param_desc_count(
    event_desc: usize,
    count_out: *mut i32,
) -> i32 {
    if count_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(count_out, ev.parameters.len() as i32);
    FMOD_OK
}

pub(super) unsafe fn event_desc_get_param_desc_by_index(
    event_desc: usize,
    index: i32,
    parameter_out: *mut FmodStudioParameterDescription,
) -> i32 {
    if index < 0 || parameter_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    let Some(def) = ev.parameters.get(index as usize) else {
        return FMOD_ERR_INVALID_PARAM;
    };
    write_opt(parameter_out, param_def_to_abi(def));
    FMOD_OK
}

pub(super) unsafe fn event_desc_get_param_desc_by_name(
    event_desc: usize,
    name: *const u8,
    parameter_out: *mut FmodStudioParameterDescription,
) -> i32 {
    if name.is_null() || parameter_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let target = from_ansi(name);
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    if let Some(def) = ev
        .parameters
        .iter()
        .find(|p| p.name.eq_ignore_ascii_case(&target))
    {
        write_opt(parameter_out, param_def_to_abi(def));
        return FMOD_OK;
    }
    if let Some(def) = st.global_param_defs.get(&target.to_ascii_lowercase()) {
        write_opt(parameter_out, param_def_to_abi(def));
        return FMOD_OK;
    }
    FMOD_ERR_EVENT_NOTFOUND
}

pub(super) unsafe fn event_desc_get_param_desc_by_id(
    event_desc: usize,
    id: u64,
    parameter_out: *mut FmodStudioParameterDescription,
) -> i32 {
    if parameter_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let pid = FmodStudioParameterId::from_u64(id);
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    if let Some(def) = ev.parameters.iter().find(|p| p.id == pid) {
        write_opt(parameter_out, param_def_to_abi(def));
        return FMOD_OK;
    }
    if let Some(key) = st.global_param_id_to_key.get(&pid) {
        if let Some(def) = st.global_param_defs.get(key) {
            write_opt(parameter_out, param_def_to_abi(def));
            return FMOD_OK;
        }
    }
    FMOD_ERR_EVENT_NOTFOUND
}

pub(super) unsafe fn event_desc_get_length(event_desc: usize, length_out: *mut i32) -> i32 {
    if length_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(length_out, ev.length_ms);
    FMOD_OK
}

pub(super) unsafe fn event_desc_get_min_max_distance(
    event_desc: usize,
    min_out: *mut f32,
    max_out: *mut f32,
) -> i32 {
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(min_out, ev.min_distance);
    write_opt(max_out, ev.max_distance);
    FMOD_OK
}

pub(super) unsafe fn event_desc_get_sound_size(event_desc: usize, size_out: *mut f32) -> i32 {
    if size_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(size_out, ev.sound_size);
    FMOD_OK
}

pub(super) unsafe fn event_desc_is_snapshot(event_desc: usize, snapshot_out: *mut i32) -> i32 {
    if snapshot_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(snapshot_out, if ev.is_snapshot { 1 } else { 0 });
    FMOD_OK
}

pub(super) unsafe fn event_desc_is_oneshot(event_desc: usize, oneshot_out: *mut i32) -> i32 {
    if oneshot_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(oneshot_out, if ev.is_oneshot { 1 } else { 0 });
    FMOD_OK
}

pub(super) unsafe fn event_desc_is_stream(event_desc: usize, isstream_out: *mut i32) -> i32 {
    if isstream_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(isstream_out, if ev.is_stream { 1 } else { 0 });
    FMOD_OK
}

pub(super) unsafe fn event_desc_is_3d(event_desc: usize, is3d_out: *mut i32) -> i32 {
    if is3d_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(is3d_out, if ev.is_3d { 1 } else { 0 });
    FMOD_OK
}

pub(super) unsafe fn event_desc_has_sustain_point(
    event_desc: usize,
    sustainpoint_out: *mut i32,
) -> i32 {
    if sustainpoint_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(sustainpoint_out, if ev.has_sustain_point { 1 } else { 0 });
    FMOD_OK
}

pub(super) unsafe fn event_desc_create_instance(
    event_desc: usize,
    instance_out: *mut usize,
) -> i32 {
    write_opt(instance_out, 0);
    if instance_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };

    let inst_handle = next_handle();
    let cg_id = next_handle();
    {
        let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
        let mut mixer = studio_arc
            .core_sys
            .mixer
            .lock()
            .unwrap_or_else(|e| e.into_inner());

        let Some(ev) = st.events.get(&event_desc) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        let parent_cg = st.resolve_event_bus_cg(ev.bus_guid, mixer.master_group_id);
        let ev_vca = st.event_vca_multiplier(ev.guid);

        let mut param_defs = HashMap::new();
        let mut param_id_to_key = HashMap::new();
        let mut param_values = HashMap::new();
        for p in &ev.parameters {
            let key = p.name.to_ascii_lowercase();
            param_id_to_key.insert(p.id, key.clone());
            param_values.insert(key.clone(), p.default_value);
            param_defs.insert(key, p.clone());
        }

        let mut properties = [-1.0f32; 6];
        properties[FMOD_STUDIO_EVENT_PROPERTY_MINIMUM_DISTANCE as usize] = ev.min_distance;
        properties[FMOD_STUDIO_EVENT_PROPERTY_MAXIMUM_DISTANCE as usize] = ev.max_distance;

        let cg_name = if ev.path.is_empty() {
            format!("event:{inst_handle:#x}")
        } else {
            ev.path.clone()
        };
        mixer.groups.insert(
            cg_id,
            ChannelGroup {
                name: cg_name,
                parent: Some(parent_cg),
                volume: ev.default_volume * ev_vca,
                mute: false,
                paused: false,
                pitch: ev.default_pitch,
            },
        );

        let inst = StudioEventInstance {
            desc_handle: event_desc,
            event_guid: ev.guid,
            bank_handle: ev.bank_handle,
            channel_group_id: cg_id,
            channel_id: None,
            volume: 1.0,
            pitch: 1.0,
            paused: false,
            state: FMOD_STUDIO_PLAYBACK_STOPPED,
            attributes_3d: Fmod3DAttributes::default(),
            min_distance: ev.min_distance,
            max_distance: ev.max_distance,
            timeline_ms: 0,
            reverb_levels: [0.0; 4],
            properties,
            param_defs,
            param_id_to_key,
            param_values,
            sample: ev.sample.clone(),
            is_3d: ev.is_3d,
            is_oneshot: ev.is_oneshot,
            desc_default_volume: ev.default_volume,
            desc_default_pitch: ev.default_pitch,
        };
        st.instances.insert(inst_handle, inst);
        if let Some(ev_mut) = st.events.get_mut(&event_desc) {
            ev_mut.instances.push(inst_handle);
        }
    }
    STUDIO_INSTANCES.insert_at(inst_handle, studio_arc.clone());
    GROUPS.insert_at(cg_id, studio_arc.core_sys.clone());
    write_opt(instance_out, inst_handle);
    FMOD_OK
}

pub(super) unsafe fn event_desc_get_instance_count(
    event_desc: usize,
    count_out: *mut i32,
) -> i32 {
    if count_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(count_out, ev.instances.len() as i32);
    FMOD_OK
}

pub(super) unsafe fn event_desc_get_instance_list(
    event_desc: usize,
    array: *mut usize,
    capacity: i32,
    count_out: *mut i32,
) -> i32 {
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_handle_list(&ev.instances, array, capacity, count_out)
}

pub(super) fn event_desc_load_sample_data(event_desc: usize) -> i32 {
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get_mut(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    ev.sample_loading_state = FMOD_STUDIO_LOADING_STATE_LOADED;
    FMOD_OK
}

pub(super) fn event_desc_unload_sample_data(event_desc: usize) -> i32 {
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get_mut(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    ev.sample_loading_state = FMOD_STUDIO_LOADING_STATE_UNLOADED;
    FMOD_OK
}

pub(super) unsafe fn event_desc_get_sample_loading_state(
    event_desc: usize,
    state_out: *mut u32,
) -> i32 {
    if state_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ev) = st.events.get(&event_desc) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(state_out, ev.sample_loading_state);
    FMOD_OK
}

pub(super) fn event_desc_release_all_instances(event_desc: usize) -> i32 {
    let studio_arc = match lookup_studio_event(event_desc) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let inst_ids: Vec<usize> = {
        let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
        let Some(ev) = st.events.get(&event_desc) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        ev.instances.clone()
    };
    for ih in inst_ids {
        let _ = event_instance_release(ih);
    }
    FMOD_OK
}

// ============================================================================
// FMOD Studio — EventInstance implementations
// ============================================================================

pub(super) fn event_instance_is_valid(instance: usize) -> i32 {
    let Ok(studio_arc) = lookup_studio_instance(instance) else {
        return 0;
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    if st.instances.contains_key(&instance) { 1 } else { 0 }
}

pub(super) unsafe fn event_instance_get_description(
    instance: usize,
    description_out: *mut usize,
) -> i32 {
    if description_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(description_out, inst.desc_handle);
    FMOD_OK
}

pub(super) fn event_instance_set_volume_impl(instance: usize, volume: f32) -> i32 {
    if !volume.is_finite() || volume < 0.0 {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let (cg_id, ev_guid, desc_vol) = {
        let Some(inst) = st.instances.get_mut(&instance) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        inst.volume = volume;
        (inst.channel_group_id, inst.event_guid, inst.desc_default_volume)
    };
    let ev_vca = st.event_vca_multiplier(ev_guid);
    let mut mixer = studio_arc
        .core_sys
        .mixer
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    if let Some(cg) = mixer.groups.get_mut(&cg_id) {
        cg.volume = volume * desc_vol * ev_vca;
    }
    FMOD_OK
}

pub(super) unsafe fn event_instance_get_volume(
    instance: usize,
    volume_out: *mut f32,
    finalvolume_out: *mut f32,
) -> i32 {
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    let vol = inst.volume;
    let final_vol = {
        let mixer = studio_arc
            .core_sys
            .mixer
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        resolve_group_chain(&mixer.groups, inst.channel_group_id).volume
    };
    write_opt(volume_out, vol);
    write_opt(finalvolume_out, final_vol);
    FMOD_OK
}

pub(super) fn event_instance_set_pitch_impl(instance: usize, pitch: f32) -> i32 {
    if !pitch.is_finite() || pitch < 0.0 {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let (cg_id, desc_pitch) = {
        let Some(inst) = st.instances.get_mut(&instance) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        inst.pitch = pitch;
        (inst.channel_group_id, inst.desc_default_pitch)
    };
    let mut mixer = studio_arc
        .core_sys
        .mixer
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    if let Some(cg) = mixer.groups.get_mut(&cg_id) {
        cg.pitch = (pitch * desc_pitch).max(0.0);
    }
    FMOD_OK
}

pub(super) unsafe fn event_instance_get_pitch(
    instance: usize,
    pitch_out: *mut f32,
    finalpitch_out: *mut f32,
) -> i32 {
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    let pitch = inst.pitch;
    let final_pitch = {
        let mixer = studio_arc
            .core_sys
            .mixer
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        resolve_group_chain(&mixer.groups, inst.channel_group_id).pitch
    };
    write_opt(pitch_out, pitch);
    write_opt(finalpitch_out, final_pitch);
    FMOD_OK
}

pub(super) unsafe fn event_instance_set_3d_attributes(
    instance: usize,
    attributes: *const Fmod3DAttributes,
) -> i32 {
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let Some(attrs) = read_opt(attributes) else {
        return FMOD_ERR_INVALID_PARAM;
    };
    for v in [attrs.position, attrs.velocity, attrs.forward, attrs.up] {
        if !vec3_is_finite(v) {
            return FMOD_ERR_INVALID_FLOAT;
        }
    }
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get_mut(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    inst.attributes_3d = attrs;
    if let Some(cid) = inst.channel_id {
        let mut mixer = studio_arc
            .core_sys
            .mixer
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        if let Some(ch) = mixer.channels.get_mut(&cid) {
            ch.pos_3d = attrs.position;
            ch.vel_3d = attrs.velocity;
        }
    }
    FMOD_OK
}

pub(super) unsafe fn event_instance_get_3d_attributes(
    instance: usize,
    attributes_out: *mut Fmod3DAttributes,
) -> i32 {
    if attributes_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(attributes_out, inst.attributes_3d);
    FMOD_OK
}

pub(super) fn event_instance_set_paused(instance: usize, paused: i32) -> i32 {
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get_mut(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    inst.paused = paused != 0;
    let cg_id = inst.channel_group_id;
    let mut mixer = studio_arc
        .core_sys
        .mixer
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    if let Some(cg) = mixer.groups.get_mut(&cg_id) {
        cg.paused = paused != 0;
    }
    FMOD_OK
}

pub(super) unsafe fn event_instance_get_paused(instance: usize, paused_out: *mut i32) -> i32 {
    if paused_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(paused_out, if inst.paused { 1 } else { 0 });
    FMOD_OK
}

pub(super) fn event_instance_start(instance: usize) -> i32 {
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };

    let mut old_channel = None;
    let mut new_channel = None;

    {
        let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
        let mut mixer = studio_arc
            .core_sys
            .mixer
            .lock()
            .unwrap_or_else(|e| e.into_inner());

        if !mixer.initialized {
            return FMOD_ERR_STUDIO_UNINITIALIZED;
        }
        let Some(inst) = st.instances.get_mut(&instance) else {
            return FMOD_ERR_INVALID_HANDLE;
        };

        if let Some(prev_cid) = inst.channel_id.take() {
            mixer.channels.remove(&prev_cid);
            old_channel = Some(prev_cid);
        }

        if let Some(ref smp) = inst.sample {
            if smp.num_frames > 0 && smp.channels > 0 {
                let active_count = mixer.channels.values().filter(|c| !c.stopped).count();
                if active_count >= mixer.max_channels {
                    return FMOD_ERR_TOOMANYCHANNELS;
                }
                let looping = !inst.is_oneshot || smp.has_loop;
                let mut mode = if inst.is_3d { FMOD_3D } else { FMOD_2D };
                mode |= if looping {
                    FMOD_LOOP_NORMAL
                } else {
                    FMOD_LOOP_OFF
                };
                let loop_count = if looping { -1 } else { 0 };
                let frame_bytes =
                    (smp.channels as usize) * ((smp.bits_per_sample as usize) / 8).max(1);
                let start_frame = if inst.timeline_ms > 0 {
                    ((inst.timeline_ms as f64) * (smp.sample_rate as f64) / 1000.0)
                        .clamp(0.0, smp.num_frames as f64)
                } else {
                    0.0
                };

                let cid = next_handle();
                mixer.channels.insert(
                    cid,
                    Channel {
                        sound_id: 0,
                        group_id: inst.channel_group_id,
                        samples: smp.samples.clone(),
                        sound_channels: smp.channels as usize,
                        sound_sample_rate: smp.sample_rate,
                        sound_frame_bytes: frame_bytes,
                        num_frames: smp.num_frames,
                        loop_start: if smp.has_loop { smp.loop_start } else { 0 },
                        loop_end: if smp.has_loop {
                            smp.loop_end
                        } else {
                            smp.num_frames
                        },
                        play_pos: start_frame,
                        frequency: smp.sample_rate as f32,
                        pitch: 1.0,
                        volume: 1.0,
                        pan: 0.0,
                        mute: false,
                        paused: false,
                        stopped: false,
                        mode,
                        loop_count,
                        loops_remaining: loop_count,
                        pos_3d: inst.attributes_3d.position,
                        vel_3d: inst.attributes_3d.velocity,
                        min_distance: inst.min_distance,
                        max_distance: inst.max_distance,
                    },
                );
                inst.channel_id = Some(cid);
                new_channel = Some(cid);
            }
        }
        inst.state = FMOD_STUDIO_PLAYBACK_PLAYING;
    }

    if let Some(old_cid) = old_channel {
        CHANNELS.remove(old_cid);
    }
    if let Some(new_cid) = new_channel {
        CHANNELS.insert_at(new_cid, studio_arc.core_sys.clone());
    }
    FMOD_OK
}

pub(super) fn event_instance_stop(instance: usize, mode: u32) -> i32 {
    if mode != FMOD_STUDIO_STOP_ALLOWFADEOUT && mode != FMOD_STUDIO_STOP_IMMEDIATE {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get_mut(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    inst.state = FMOD_STUDIO_PLAYBACK_STOPPED;
    if let Some(cid) = inst.channel_id {
        let mut mixer = studio_arc
            .core_sys
            .mixer
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        if let Some(ch) = mixer.channels.get_mut(&cid) {
            ch.stopped = true;
        }
    }
    FMOD_OK
}

pub(super) unsafe fn event_instance_get_timeline_position(
    instance: usize,
    position_out: *mut i32,
) -> i32 {
    if position_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get_mut(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    if let Some(cid) = inst.channel_id {
        let mixer = studio_arc
            .core_sys
            .mixer
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        if let Some(ch) = mixer.channels.get(&cid) {
            inst.timeline_ms = ((ch.play_pos.max(0.0) * 1000.0)
                / (ch.sound_sample_rate.max(1) as f64)) as i32;
        }
    }
    write_opt(position_out, inst.timeline_ms);
    FMOD_OK
}

pub(super) fn event_instance_set_timeline_position(instance: usize, position: i32) -> i32 {
    if position < 0 {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get_mut(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    inst.timeline_ms = position;
    if let Some(cid) = inst.channel_id {
        let mut mixer = studio_arc
            .core_sys
            .mixer
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        if let Some(ch) = mixer.channels.get_mut(&cid) {
            let frame = ((position as f64) * (ch.sound_sample_rate as f64) / 1000.0)
                .clamp(0.0, ch.num_frames as f64);
            ch.play_pos = frame;
        }
    }
    FMOD_OK
}

pub(super) unsafe fn event_instance_get_playback_state(
    instance: usize,
    state_out: *mut u32,
) -> i32 {
    if state_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get_mut(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    if inst.state == FMOD_STUDIO_PLAYBACK_PLAYING {
        if let Some(cid) = inst.channel_id {
            let mixer = studio_arc
                .core_sys
                .mixer
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            let finished = mixer
                .channels
                .get(&cid)
                .map(|c| c.stopped)
                .unwrap_or(true);
            if finished {
                inst.state = FMOD_STUDIO_PLAYBACK_STOPPED;
            }
        }
    }
    write_opt(state_out, inst.state);
    FMOD_OK
}

pub(super) unsafe fn event_instance_get_channel_group(
    instance: usize,
    group_out: *mut usize,
) -> i32 {
    if group_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(group_out, inst.channel_group_id);
    FMOD_OK
}

pub(super) fn event_instance_release(instance: usize) -> i32 {
    let Some(studio_arc) = STUDIO_INSTANCES.remove(instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };

    let mut rm_group = None;
    let mut rm_channel = None;
    {
        let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
        let Some(inst) = st.instances.remove(&instance) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        if let Some(ev) = st.events.get_mut(&inst.desc_handle) {
            ev.instances.retain(|&h| h != instance);
        }

        let mut mixer = studio_arc
            .core_sys
            .mixer
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let still_playing = inst
            .channel_id
            .and_then(|cid| mixer.channels.get(&cid))
            .is_some_and(|ch| !ch.stopped);

        if still_playing {
            // Fire-and-forget semantics (`Start` -> `Release`): let the live mixer finish
            // playing the active channel and reap it on `Studio_System_Update` / `Bank_Unload`.
            if let Some(cid) = inst.channel_id {
                st.orphans.push(OrphanInstance {
                    bank_handle: inst.bank_handle,
                    channel_group_id: inst.channel_group_id,
                    channel_id: cid,
                });
            }
        } else {
            if let Some(cid) = inst.channel_id {
                mixer.channels.remove(&cid);
                rm_channel = Some(cid);
            }
            mixer.groups.remove(&inst.channel_group_id);
            rm_group = Some(inst.channel_group_id);
        }
    }

    if let Some(cg) = rm_group {
        GROUPS.remove(cg);
    }
    if let Some(cid) = rm_channel {
        CHANNELS.remove(cid);
    }
    FMOD_OK
}

pub(super) unsafe fn event_instance_is_virtual(
    instance: usize,
    virtualstate_out: *mut i32,
) -> i32 {
    if virtualstate_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    if !st.instances.contains_key(&instance) {
        return FMOD_ERR_INVALID_HANDLE;
    }
    write_opt(virtualstate_out, 0);
    FMOD_OK
}

pub(super) fn event_instance_set_param_by_name_impl(
    instance: usize,
    name: *const u8,
    value: f32,
    _ignoreseekspeed: i32,
) -> i32 {
    if name.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    if !value.is_finite() {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let key = unsafe { from_ansi(name) }.to_ascii_lowercase();
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(inst) = st.instances.get_mut(&instance) {
        if let Some(def) = inst.param_defs.get(&key) {
            let clamped = if def.maximum > def.minimum {
                value.clamp(def.minimum, def.maximum)
            } else {
                value
            };
            inst.param_values.insert(key, clamped);
            return FMOD_OK;
        }
    } else {
        return FMOD_ERR_INVALID_HANDLE;
    }
    if let Some(def) = st.global_param_defs.get(&key) {
        let clamped = if def.maximum > def.minimum {
            value.clamp(def.minimum, def.maximum)
        } else {
            value
        };
        st.global_param_values.insert(key, clamped);
        return FMOD_OK;
    }
    FMOD_ERR_EVENT_NOTFOUND
}

pub(super) unsafe fn event_instance_get_param_by_name(
    instance: usize,
    name: *const u8,
    value_out: *mut f32,
    finalvalue_out: *mut f32,
) -> i32 {
    if name.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let key = from_ansi(name).to_ascii_lowercase();
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    if let Some(def) = inst.param_defs.get(&key) {
        let set_val = inst
            .param_values
            .get(&key)
            .copied()
            .unwrap_or(def.default_value);
        let final_val = if def.param_type == FMOD_STUDIO_PARAMETER_AUTOMATIC_DISTANCE {
            let d = st.compute_listener_distance(inst.attributes_3d.position);
            if def.maximum > def.minimum {
                d.clamp(def.minimum, def.maximum)
            } else {
                d
            }
        } else {
            set_val
        };
        write_opt(value_out, set_val);
        write_opt(finalvalue_out, final_val);
        return FMOD_OK;
    }
    if let Some(&val) = st.global_param_values.get(&key) {
        write_opt(value_out, val);
        write_opt(finalvalue_out, val);
        return FMOD_OK;
    }
    FMOD_ERR_EVENT_NOTFOUND
}

pub(super) fn event_instance_set_param_by_id_impl(
    instance: usize,
    id: u64,
    value: f32,
    _ignoreseekspeed: i32,
) -> i32 {
    if !value.is_finite() {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let pid = FmodStudioParameterId::from_u64(id);
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(inst) = st.instances.get_mut(&instance) {
        if let Some(key) = inst.param_id_to_key.get(&pid).cloned() {
            if let Some(def) = inst.param_defs.get(&key) {
                let clamped = if def.maximum > def.minimum {
                    value.clamp(def.minimum, def.maximum)
                } else {
                    value
                };
                inst.param_values.insert(key, clamped);
                return FMOD_OK;
            }
        }
    } else {
        return FMOD_ERR_INVALID_HANDLE;
    }
    if let Some(key) = st.global_param_id_to_key.get(&pid).cloned() {
        if let Some(def) = st.global_param_defs.get(&key) {
            let clamped = if def.maximum > def.minimum {
                value.clamp(def.minimum, def.maximum)
            } else {
                value
            };
            st.global_param_values.insert(key, clamped);
            return FMOD_OK;
        }
    }
    FMOD_ERR_EVENT_NOTFOUND
}

pub(super) unsafe fn event_instance_get_param_by_id(
    instance: usize,
    id: u64,
    value_out: *mut f32,
    finalvalue_out: *mut f32,
) -> i32 {
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let pid = FmodStudioParameterId::from_u64(id);
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    if let Some(key) = inst.param_id_to_key.get(&pid) {
        if let Some(def) = inst.param_defs.get(key) {
            let set_val = inst
                .param_values
                .get(key)
                .copied()
                .unwrap_or(def.default_value);
            let final_val = if def.param_type == FMOD_STUDIO_PARAMETER_AUTOMATIC_DISTANCE {
                let d = st.compute_listener_distance(inst.attributes_3d.position);
                if def.maximum > def.minimum {
                    d.clamp(def.minimum, def.maximum)
                } else {
                    d
                }
            } else {
                set_val
            };
            write_opt(value_out, set_val);
            write_opt(finalvalue_out, final_val);
            return FMOD_OK;
        }
    }
    if let Some(key) = st.global_param_id_to_key.get(&pid) {
        if let Some(&val) = st.global_param_values.get(key) {
            write_opt(value_out, val);
            write_opt(finalvalue_out, val);
            return FMOD_OK;
        }
    }
    FMOD_ERR_EVENT_NOTFOUND
}

pub(super) unsafe fn event_instance_get_min_max_distance(
    instance: usize,
    min_out: *mut f32,
    max_out: *mut f32,
) -> i32 {
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(min_out, inst.min_distance);
    write_opt(max_out, inst.max_distance);
    FMOD_OK
}

pub(super) fn event_instance_set_property_impl(instance: usize, index: u32, value: f32) -> i32 {
    if index >= FMOD_STUDIO_EVENT_PROPERTY_MAX {
        return FMOD_ERR_INVALID_PARAM;
    }
    if !value.is_finite() {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get_mut(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    inst.properties[index as usize] = value;
    let mut update_dist = false;
    if index == FMOD_STUDIO_EVENT_PROPERTY_MINIMUM_DISTANCE && value >= 0.0 {
        inst.min_distance = value;
        if inst.max_distance < inst.min_distance {
            inst.max_distance = inst.min_distance;
        }
        update_dist = true;
    } else if index == FMOD_STUDIO_EVENT_PROPERTY_MAXIMUM_DISTANCE && value >= 0.0 {
        inst.max_distance = value.max(inst.min_distance);
        update_dist = true;
    }
    if update_dist {
        if let Some(cid) = inst.channel_id {
            let min_d = inst.min_distance;
            let max_d = inst.max_distance;
            let mut mixer = studio_arc
                .core_sys
                .mixer
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            if let Some(ch) = mixer.channels.get_mut(&cid) {
                ch.min_distance = min_d;
                ch.max_distance = max_d;
            }
        }
    }
    FMOD_OK
}

pub(super) unsafe fn event_instance_get_property(
    instance: usize,
    index: u32,
    value_out: *mut f32,
) -> i32 {
    if index >= FMOD_STUDIO_EVENT_PROPERTY_MAX || value_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(value_out, inst.properties[index as usize]);
    FMOD_OK
}

pub(super) fn event_instance_set_reverb_level_impl(
    instance: usize,
    index: i32,
    reverb: f32,
) -> i32 {
    if !(0..4).contains(&index) {
        return FMOD_ERR_INVALID_PARAM;
    }
    if !reverb.is_finite() {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get_mut(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    inst.reverb_levels[index as usize] = reverb;
    FMOD_OK
}

pub(super) unsafe fn event_instance_get_reverb_level(
    instance: usize,
    index: i32,
    reverb_out: *mut f32,
) -> i32 {
    if !(0..4).contains(&index) || reverb_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(reverb_out, inst.reverb_levels[index as usize]);
    FMOD_OK
}

pub(super) fn event_instance_key_off(instance: usize) -> i32 {
    let studio_arc = match lookup_studio_instance(instance) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(inst) = st.instances.get_mut(&instance) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    if inst.state == FMOD_STUDIO_PLAYBACK_SUSTAINING {
        inst.state = FMOD_STUDIO_PLAYBACK_PLAYING;
    }
    if let Some(cid) = inst.channel_id {
        let mut mixer = studio_arc
            .core_sys
            .mixer
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        if let Some(ch) = mixer.channels.get_mut(&cid) {
            ch.loop_count = 0;
            ch.loops_remaining = 0;
            ch.mode = (ch.mode & !(FMOD_LOOP_NORMAL | FMOD_LOOP_BIDI)) | FMOD_LOOP_OFF;
        }
    }
    FMOD_OK
}

// ============================================================================
// FMOD Studio — Bus implementations
// ============================================================================

pub(super) fn bus_is_valid(bus: usize) -> i32 {
    let Ok(studio_arc) = lookup_studio_bus(bus) else {
        return 0;
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    if st.buses.contains_key(&bus) { 1 } else { 0 }
}

pub(super) unsafe fn bus_get_id(bus: usize, id_out: *mut FmodGuid) -> i32 {
    if id_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_bus(bus) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.buses.get(&bus) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(id_out, b.guid);
    FMOD_OK
}

pub(super) unsafe fn bus_get_path(
    bus: usize,
    path_out: *mut u8,
    size: i32,
    retrieved_out: *mut i32,
) -> i32 {
    let studio_arc = match lookup_studio_bus(bus) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let path = {
        let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
        let Some(b) = st.buses.get(&bus) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        if !b.path.is_empty() {
            b.path.clone()
        } else {
            st.guid_to_path.get(&b.guid).cloned().unwrap_or_default()
        }
    };
    write_studio_path(&path, path_out, size, retrieved_out)
}

pub(super) fn bus_set_volume_impl(bus: usize, volume: f32) -> i32 {
    if !volume.is_finite() || volume < 0.0 {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let studio_arc = match lookup_studio_bus(bus) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let (cg_id, bus_guid) = {
        let Some(b) = st.buses.get_mut(&bus) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        b.volume = volume;
        (b.channel_group_id, b.guid)
    };
    let vca_mult = st.bus_vca_multiplier(bus_guid);
    let mut mixer = studio_arc
        .core_sys
        .mixer
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    if let Some(cg) = mixer.groups.get_mut(&cg_id) {
        cg.volume = volume * vca_mult;
    }
    FMOD_OK
}

pub(super) unsafe fn bus_get_volume(
    bus: usize,
    volume_out: *mut f32,
    finalvolume_out: *mut f32,
) -> i32 {
    let studio_arc = match lookup_studio_bus(bus) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.buses.get(&bus) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    let vol = b.volume;
    let final_vol = {
        let mixer = studio_arc
            .core_sys
            .mixer
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        resolve_group_chain(&mixer.groups, b.channel_group_id).volume
    };
    write_opt(volume_out, vol);
    write_opt(finalvolume_out, final_vol);
    FMOD_OK
}

pub(super) fn bus_set_paused(bus: usize, paused: i32) -> i32 {
    let studio_arc = match lookup_studio_bus(bus) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.buses.get_mut(&bus) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    b.paused = paused != 0;
    let cg_id = b.channel_group_id;
    let mut mixer = studio_arc
        .core_sys
        .mixer
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    if let Some(cg) = mixer.groups.get_mut(&cg_id) {
        cg.paused = paused != 0;
    }
    FMOD_OK
}

pub(super) unsafe fn bus_get_paused(bus: usize, paused_out: *mut i32) -> i32 {
    if paused_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_bus(bus) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.buses.get(&bus) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(paused_out, if b.paused { 1 } else { 0 });
    FMOD_OK
}

pub(super) fn bus_set_mute(bus: usize, mute: i32) -> i32 {
    let studio_arc = match lookup_studio_bus(bus) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.buses.get_mut(&bus) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    b.mute = mute != 0;
    let cg_id = b.channel_group_id;
    let mut mixer = studio_arc
        .core_sys
        .mixer
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    if let Some(cg) = mixer.groups.get_mut(&cg_id) {
        cg.mute = mute != 0;
    }
    FMOD_OK
}

pub(super) unsafe fn bus_get_mute(bus: usize, mute_out: *mut i32) -> i32 {
    if mute_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_bus(bus) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.buses.get(&bus) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(mute_out, if b.mute { 1 } else { 0 });
    FMOD_OK
}

pub(super) fn bus_stop_all_events(bus: usize, mode: u32) -> i32 {
    if mode != FMOD_STUDIO_STOP_ALLOWFADEOUT && mode != FMOD_STUDIO_STOP_IMMEDIATE {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_bus(bus) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(bus_cg) = st.buses.get(&bus).map(|b| b.channel_group_id) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    let mut mixer = studio_arc
        .core_sys
        .mixer
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let affected_groups: Vec<usize> = mixer
        .groups
        .keys()
        .copied()
        .filter(|&gid| group_has_ancestor(&mixer.groups, gid, bus_cg))
        .collect();
    for ch in mixer.channels.values_mut() {
        if affected_groups.contains(&ch.group_id) {
            ch.stopped = true;
        }
    }
    for inst in st.instances.values_mut() {
        if affected_groups.contains(&inst.channel_group_id) {
            inst.state = FMOD_STUDIO_PLAYBACK_STOPPED;
        }
    }
    FMOD_OK
}

pub(super) fn bus_lock_channel_group(bus: usize) -> i32 {
    let studio_arc = match lookup_studio_bus(bus) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.buses.get_mut(&bus) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    b.locked_group = true;
    FMOD_OK
}

pub(super) fn bus_unlock_channel_group(bus: usize) -> i32 {
    let studio_arc = match lookup_studio_bus(bus) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.buses.get_mut(&bus) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    b.locked_group = false;
    FMOD_OK
}

pub(super) unsafe fn bus_get_channel_group(bus: usize, group_out: *mut usize) -> i32 {
    if group_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_bus(bus) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(b) = st.buses.get(&bus) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(group_out, b.channel_group_id);
    FMOD_OK
}

// ============================================================================
// FMOD Studio — VCA implementations
// ============================================================================

pub(super) fn vca_is_valid(vca: usize) -> i32 {
    let Ok(studio_arc) = lookup_studio_vca(vca) else {
        return 0;
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    if st.vcas.contains_key(&vca) { 1 } else { 0 }
}

pub(super) unsafe fn vca_get_id(vca: usize, id_out: *mut FmodGuid) -> i32 {
    if id_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let studio_arc = match lookup_studio_vca(vca) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(v) = st.vcas.get(&vca) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(id_out, v.guid);
    FMOD_OK
}

pub(super) unsafe fn vca_get_path(
    vca: usize,
    path_out: *mut u8,
    size: i32,
    retrieved_out: *mut i32,
) -> i32 {
    let studio_arc = match lookup_studio_vca(vca) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let path = {
        let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
        let Some(v) = st.vcas.get(&vca) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        if !v.path.is_empty() {
            v.path.clone()
        } else {
            st.guid_to_path.get(&v.guid).cloned().unwrap_or_default()
        }
    };
    write_studio_path(&path, path_out, size, retrieved_out)
}

pub(super) fn vca_set_volume_impl(vca: usize, volume: f32) -> i32 {
    if !volume.is_finite() || volume < 0.0 {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let studio_arc = match lookup_studio_vca(vca) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(v) = st.vcas.get_mut(&vca) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    v.volume = volume;
    let mut mixer = studio_arc
        .core_sys
        .mixer
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    st.sync_bus_and_instance_gains(&mut mixer);
    FMOD_OK
}

pub(super) unsafe fn vca_get_volume(
    vca: usize,
    volume_out: *mut f32,
    finalvolume_out: *mut f32,
) -> i32 {
    let studio_arc = match lookup_studio_vca(vca) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let st = studio_arc.state.lock().unwrap_or_else(|e| e.into_inner());
    let Some(v) = st.vcas.get(&vca) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(volume_out, v.volume);
    write_opt(finalvolume_out, v.volume);
    FMOD_OK
}
