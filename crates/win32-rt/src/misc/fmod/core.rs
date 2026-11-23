//! FMOD 2.02 constants, ABI types, shared handle tables, and FMOD Core
//! (`fmod.dll` / `fmod64.dll`) implementation routines.

use super::bank::guid_from_uid;
use super::dsp::{decode_pcm_to_f32, format_bits_and_bytes, parse_fsb5, parse_wav};
use super::mixer::{
    Channel, ChannelGroup, FmodSystem, MixerState, Sound, group_has_ancestor,
    sound_from_decoded_sample, speakermode_for_channels, start_audio_output,
};
use super::studio::StudioSystem;
use crate::audio::backend;
use crate::audio::format::{Vec3, vec3_is_finite};
use crate::handles::HandleTable;
use crate::strings::{from_ansi, rd, wr, write_ansi_z};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

// ============================================================================
// FMOD 2.02 constants & ABI types
// ============================================================================

/// FMOD Core API version reported by `FMOD_System_GetVersion` (2.02.00).
pub const FMOD_VERSION: u32 = 0x0002_0200;

// FMOD_RESULT
pub const FMOD_OK: i32 = 0;
pub const FMOD_ERR_FILE_BAD: i32 = 13;
pub const FMOD_ERR_FILE_EOF: i32 = 16;
pub const FMOD_ERR_FILE_NOTFOUND: i32 = 18;
pub const FMOD_ERR_FORMAT: i32 = 19;
pub const FMOD_ERR_INITIALIZED: i32 = 27;
pub const FMOD_ERR_INVALID_FLOAT: i32 = 29;
pub const FMOD_ERR_INVALID_HANDLE: i32 = 30;
pub const FMOD_ERR_INVALID_PARAM: i32 = 31;
pub const FMOD_ERR_INVALID_POSITION: i32 = 32;
pub const FMOD_ERR_OUTPUT_INIT: i32 = 51;
pub const FMOD_ERR_TOOMANYCHANNELS: i32 = 64;
pub const FMOD_ERR_TRUNCATED: i32 = 65;
pub const FMOD_ERR_UNINITIALIZED: i32 = 67;
pub const FMOD_ERR_EVENT_ALREADY_LOADED: i32 = 70;
pub const FMOD_ERR_EVENT_NOTFOUND: i32 = 74;
pub const FMOD_ERR_STUDIO_UNINITIALIZED: i32 = 75;
pub const FMOD_ERR_STUDIO_NOT_LOADED: i32 = 76;
pub const FMOD_ERR_INVALID_STRING: i32 = 77;
pub const FMOD_ERR_ALREADY_LOCKED: i32 = 78;
pub const FMOD_ERR_NOT_LOCKED: i32 = 79;

// FMOD_MODE
pub const FMOD_DEFAULT: u32 = 0x0000_0000;
pub const FMOD_LOOP_OFF: u32 = 0x0000_0001;
pub const FMOD_LOOP_NORMAL: u32 = 0x0000_0002;
pub const FMOD_LOOP_BIDI: u32 = 0x0000_0004;
pub const FMOD_2D: u32 = 0x0000_0008;
pub const FMOD_3D: u32 = 0x0000_0010;
pub const FMOD_CREATESTREAM: u32 = 0x0000_0080;
pub const FMOD_OPENUSER: u32 = 0x0000_0400;
pub const FMOD_OPENMEMORY: u32 = 0x0000_0800;
pub const FMOD_OPENRAW: u32 = 0x0000_1000;
pub const FMOD_3D_HEADRELATIVE: u32 = 0x0004_0000;
pub const FMOD_3D_WORLDRELATIVE: u32 = 0x0008_0000;
pub const FMOD_3D_INVERSEROLLOFF: u32 = 0x0010_0000;
pub const FMOD_3D_LINEARROLLOFF: u32 = 0x0020_0000;
pub const FMOD_3D_LINEARSQUAREROLLOFF: u32 = 0x0040_0000;
pub const FMOD_OPENMEMORY_POINT: u32 = 0x1000_0000;

// FMOD_SOUND_TYPE
pub const FMOD_SOUND_TYPE_UNKNOWN: u32 = 0;
pub const FMOD_SOUND_TYPE_FSB: u32 = 5;
pub const FMOD_SOUND_TYPE_RAW: u32 = 12;
pub const FMOD_SOUND_TYPE_USER: u32 = 14;
pub const FMOD_SOUND_TYPE_WAV: u32 = 15;

// FMOD_SOUND_FORMAT
pub const FMOD_SOUND_FORMAT_NONE: u32 = 0;
pub const FMOD_SOUND_FORMAT_PCM8: u32 = 1;
pub const FMOD_SOUND_FORMAT_PCM16: u32 = 2;
pub const FMOD_SOUND_FORMAT_PCM24: u32 = 3;
pub const FMOD_SOUND_FORMAT_PCM32: u32 = 4;
pub const FMOD_SOUND_FORMAT_PCMFLOAT: u32 = 5;

// FMOD_SPEAKERMODE
pub const FMOD_SPEAKERMODE_DEFAULT: u32 = 0;
pub const FMOD_SPEAKERMODE_RAW: u32 = 1;
pub const FMOD_SPEAKERMODE_MONO: u32 = 2;
pub const FMOD_SPEAKERMODE_STEREO: u32 = 3;
pub const FMOD_SPEAKERMODE_QUAD: u32 = 4;
pub const FMOD_SPEAKERMODE_SURROUND: u32 = 5;
pub const FMOD_SPEAKERMODE_5POINT1: u32 = 6;
pub const FMOD_SPEAKERMODE_7POINT1: u32 = 7;

// FMOD_TIMEUNIT
pub const FMOD_TIMEUNIT_MS: u32 = 0x0000_0001;
pub const FMOD_TIMEUNIT_PCM: u32 = 0x0000_0002;
pub const FMOD_TIMEUNIT_PCMBYTES: u32 = 0x0000_0004;
pub const FMOD_TIMEUNIT_RAWBYTES: u32 = 0x0000_0008;

// FMOD_OUTPUTTYPE
pub const FMOD_OUTPUTTYPE_AUTODETECT: u32 = 0;
pub const FMOD_OUTPUTTYPE_UNKNOWN: u32 = 1;
pub const FMOD_OUTPUTTYPE_NOSOUND: u32 = 2;
pub const FMOD_OUTPUTTYPE_WAVWRITER: u32 = 3;
pub const FMOD_OUTPUTTYPE_NOSOUND_NRT: u32 = 4;
pub const FMOD_OUTPUTTYPE_WAVWRITER_NRT: u32 = 5;
pub const FMOD_OUTPUTTYPE_WASAPI: u32 = 6;
pub const FMOD_OUTPUTTYPE_COREAUDIO: u32 = 10;

// FMOD_STUDIO_LOADING_STATE
pub const FMOD_STUDIO_LOADING_STATE_UNLOADING: u32 = 0;
pub const FMOD_STUDIO_LOADING_STATE_UNLOADED: u32 = 1;
pub const FMOD_STUDIO_LOADING_STATE_LOADING: u32 = 2;
pub const FMOD_STUDIO_LOADING_STATE_LOADED: u32 = 3;
pub const FMOD_STUDIO_LOADING_STATE_ERROR: u32 = 4;

// FMOD_STUDIO_PLAYBACK_STATE
pub const FMOD_STUDIO_PLAYBACK_PLAYING: u32 = 0;
pub const FMOD_STUDIO_PLAYBACK_SUSTAINING: u32 = 1;
pub const FMOD_STUDIO_PLAYBACK_STOPPED: u32 = 2;
pub const FMOD_STUDIO_PLAYBACK_STARTING: u32 = 3;
pub const FMOD_STUDIO_PLAYBACK_STOPPING: u32 = 4;

// FMOD_STUDIO_STOP_MODE
pub const FMOD_STUDIO_STOP_ALLOWFADEOUT: u32 = 0;
pub const FMOD_STUDIO_STOP_IMMEDIATE: u32 = 1;

// FMOD_STUDIO_LOAD_MEMORY_MODE
pub const FMOD_STUDIO_LOAD_MEMORY: u32 = 0;
pub const FMOD_STUDIO_LOAD_MEMORY_POINT: u32 = 1;

// FMOD_STUDIO_LOAD_BANK_FLAGS
pub const FMOD_STUDIO_LOAD_BANK_NORMAL: u32 = 0x0000_0000;
pub const FMOD_STUDIO_LOAD_BANK_NONBLOCKING: u32 = 0x0000_0001;
pub const FMOD_STUDIO_LOAD_BANK_DECOMPRESS_SAMPLES: u32 = 0x0000_0002;
pub const FMOD_STUDIO_LOAD_BANK_UNENCRYPTED: u32 = 0x0000_0004;

// FMOD_STUDIO_EVENT_PROPERTY
pub const FMOD_STUDIO_EVENT_PROPERTY_CHANNELPRIORITY: u32 = 0;
pub const FMOD_STUDIO_EVENT_PROPERTY_SCHEDULE_DELAY: u32 = 1;
pub const FMOD_STUDIO_EVENT_PROPERTY_SCHEDULE_LOOKAHEAD: u32 = 2;
pub const FMOD_STUDIO_EVENT_PROPERTY_MINIMUM_DISTANCE: u32 = 3;
pub const FMOD_STUDIO_EVENT_PROPERTY_MAXIMUM_DISTANCE: u32 = 4;
pub const FMOD_STUDIO_EVENT_PROPERTY_COOLDOWN: u32 = 5;
pub const FMOD_STUDIO_EVENT_PROPERTY_MAX: u32 = 6;

// FMOD_STUDIO_PARAMETER_TYPE
pub const FMOD_STUDIO_PARAMETER_GAME_CONTROLLED: u32 = 0;
pub const FMOD_STUDIO_PARAMETER_AUTOMATIC_DISTANCE: u32 = 1;

// FMOD_STUDIO_PARAMETER_FLAGS
pub const FMOD_STUDIO_PARAMETER_READONLY: u32 = 0x0000_0001;
pub const FMOD_STUDIO_PARAMETER_AUTOMATIC: u32 = 0x0000_0002;
pub const FMOD_STUDIO_PARAMETER_GLOBAL: u32 = 0x0000_0004;
pub const FMOD_STUDIO_PARAMETER_DISCRETE: u32 = 0x0000_0008;
pub const FMOD_STUDIO_PARAMETER_LABELED: u32 = 0x0000_0010;

pub type FmodVector = Vec3;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct FmodGuid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl FmodGuid {
    pub const ZERO: Self = Self {
        data1: 0,
        data2: 0,
        data3: 0,
        data4: [0; 8],
    };

    pub fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }

    pub fn from_le_bytes(b: [u8; 16]) -> Self {
        Self {
            data1: u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
            data2: u16::from_le_bytes([b[4], b[5]]),
            data3: u16::from_le_bytes([b[6], b[7]]),
            data4: [b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]],
        }
    }

    pub fn to_le_bytes(self) -> [u8; 16] {
        let mut out = [0u8; 16];
        out[0..4].copy_from_slice(&self.data1.to_le_bytes());
        out[4..6].copy_from_slice(&self.data2.to_le_bytes());
        out[6..8].copy_from_slice(&self.data3.to_le_bytes());
        out[8..16].copy_from_slice(&self.data4);
        out
    }

    pub fn parse_str(s: &str) -> Option<Self> {
        let trimmed = s.trim();
        let inner = trimmed
            .strip_prefix('{')
            .and_then(|v| v.strip_suffix('}'))?;
        let parts: Vec<&str> = inner.split('-').collect();
        if parts.len() != 5
            || parts[0].len() != 8
            || parts[1].len() != 4
            || parts[2].len() != 4
            || parts[3].len() != 4
            || parts[4].len() != 12
        {
            return None;
        }
        let data1 = u32::from_str_radix(parts[0], 16).ok()?;
        let data2 = u16::from_str_radix(parts[1], 16).ok()?;
        let data3 = u16::from_str_radix(parts[2], 16).ok()?;
        let mut data4 = [0u8; 8];
        data4[0] = u8::from_str_radix(&parts[3][0..2], 16).ok()?;
        data4[1] = u8::from_str_radix(&parts[3][2..4], 16).ok()?;
        for i in 0..6 {
            data4[2 + i] = u8::from_str_radix(&parts[4][i * 2..i * 2 + 2], 16).ok()?;
        }
        Some(Self {
            data1,
            data2,
            data3,
            data4,
        })
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fmod3DAttributes {
    pub position: FmodVector,
    pub velocity: FmodVector,
    pub forward: FmodVector,
    pub up: FmodVector,
}

impl Default for Fmod3DAttributes {
    fn default() -> Self {
        Self {
            position: FmodVector::new(0.0, 0.0, 0.0),
            velocity: FmodVector::new(0.0, 0.0, 0.0),
            forward: FmodVector::new(0.0, 0.0, 1.0),
            up: FmodVector::new(0.0, 1.0, 0.0),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct FmodStudioParameterId {
    pub data1: u32,
    pub data2: u32,
}

impl FmodStudioParameterId {
    #[inline]
    pub fn from_u64(raw: u64) -> Self {
        Self {
            data1: raw as u32,
            data2: (raw >> 32) as u32,
        }
    }

    #[inline]
    pub fn to_u64(self) -> u64 {
        (self.data1 as u64) | ((self.data2 as u64) << 32)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct FmodStudioParameterDescription {
    pub name: *const u8,
    pub id: FmodStudioParameterId,
    pub minimum: f32,
    pub maximum: f32,
    pub defaultvalue: f32,
    pub param_type: u32,
    pub flags: u32,
    pub guid: FmodGuid,
}

/// First 24 bytes of `FMOD_CREATESOUNDEXINFO` (identical layout across FMOD 1.x / 2.x Win64).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct FmodCreateSoundExInfoHeader {
    pub cbsize: i32,
    pub length: u32,
    pub fileoffset: u32,
    pub numchannels: i32,
    pub defaultfrequency: i32,
    pub format: u32,
}

// ============================================================================
// Guest-memory unaligned pointer helpers & path resolution
// ============================================================================

#[inline]
pub(super) unsafe fn write_opt<T>(ptr: *mut T, val: T) {
    if !ptr.is_null() {
        wr(ptr, val);
    }
}

#[inline]
pub(super) unsafe fn read_opt<T: Copy>(ptr: *const T) -> Option<T> {
    if ptr.is_null() {
        None
    } else {
        Some(rd(ptr))
    }
}

unsafe fn read_exinfo(exinfo: usize) -> Result<Option<FmodCreateSoundExInfoHeader>, i32> {
    if exinfo == 0 {
        return Ok(None);
    }
    let p = exinfo as *const u8;
    let cbsize = rd(p as *const i32);
    if cbsize < 4 {
        return Err(FMOD_ERR_INVALID_PARAM);
    }
    let mut info = FmodCreateSoundExInfoHeader {
        cbsize,
        ..Default::default()
    };
    if cbsize >= 8 {
        info.length = rd(p.add(4) as *const u32);
    }
    if cbsize >= 12 {
        info.fileoffset = rd(p.add(8) as *const u32);
    }
    if cbsize >= 16 {
        info.numchannels = rd(p.add(12) as *const i32);
    }
    if cbsize >= 20 {
        info.defaultfrequency = rd(p.add(16) as *const i32);
    }
    if cbsize >= 24 {
        info.format = rd(p.add(20) as *const u32);
    }
    Ok(Some(info))
}

pub(super) fn resolve_sound_path(raw_path: &str) -> Option<PathBuf> {
    if let Some(host) = crate::path::to_host(raw_path) {
        if host.is_file() {
            return Some(host);
        }
    }
    let direct = Path::new(raw_path);
    if direct.is_file() {
        return Some(direct.to_path_buf());
    }
    None
}

// ============================================================================
// Global handle tables (backed by `crate::handles::HandleTable`)
// ============================================================================

static NEXT_FMOD_HANDLE: AtomicUsize = AtomicUsize::new(0xF60D_0001);

pub(super) static SYSTEMS: HandleTable<Arc<FmodSystem>> = HandleTable::new(0xF60D_0001, 1);
pub(super) static SOUNDS: HandleTable<Arc<FmodSystem>> = HandleTable::new(0xF60D_0001, 1);
pub(super) static GROUPS: HandleTable<Arc<FmodSystem>> = HandleTable::new(0xF60D_0001, 1);
pub(super) static CHANNELS: HandleTable<Arc<FmodSystem>> = HandleTable::new(0xF60D_0001, 1);

pub(super) static STUDIOS: HandleTable<Arc<StudioSystem>> = HandleTable::new(0xF60D_0001, 1);
pub(super) static STUDIO_BANKS: HandleTable<Arc<StudioSystem>> = HandleTable::new(0xF60D_0001, 1);
pub(super) static STUDIO_EVENTS: HandleTable<Arc<StudioSystem>> = HandleTable::new(0xF60D_0001, 1);
pub(super) static STUDIO_INSTANCES: HandleTable<Arc<StudioSystem>> =
    HandleTable::new(0xF60D_0001, 1);
pub(super) static STUDIO_BUSES: HandleTable<Arc<StudioSystem>> = HandleTable::new(0xF60D_0001, 1);
pub(super) static STUDIO_VCAS: HandleTable<Arc<StudioSystem>> = HandleTable::new(0xF60D_0001, 1);

#[inline]
pub(super) fn next_handle() -> usize {
    NEXT_FMOD_HANDLE.fetch_add(1, Ordering::Relaxed)
}

#[inline]
pub(super) fn lookup_system(handle: usize) -> Result<Arc<FmodSystem>, i32> {
    SYSTEMS.get_cloned(handle).ok_or(FMOD_ERR_INVALID_HANDLE)
}

#[inline]
fn lookup_sound_system(handle: usize) -> Result<Arc<FmodSystem>, i32> {
    SOUNDS.get_cloned(handle).ok_or(FMOD_ERR_INVALID_HANDLE)
}

#[inline]
fn lookup_group_system(handle: usize) -> Result<Arc<FmodSystem>, i32> {
    GROUPS.get_cloned(handle).ok_or(FMOD_ERR_INVALID_HANDLE)
}

#[inline]
fn lookup_channel_system(handle: usize) -> Result<Arc<FmodSystem>, i32> {
    CHANNELS.get_cloned(handle).ok_or(FMOD_ERR_INVALID_HANDLE)
}

#[inline]
pub(super) fn lookup_studio(studio: usize) -> Result<Arc<StudioSystem>, i32> {
    STUDIOS.get_cloned(studio).ok_or(FMOD_ERR_INVALID_HANDLE)
}

#[inline]
pub(super) fn lookup_studio_bank(bank: usize) -> Result<Arc<StudioSystem>, i32> {
    STUDIO_BANKS.get_cloned(bank).ok_or(FMOD_ERR_INVALID_HANDLE)
}

#[inline]
pub(super) fn lookup_studio_event(event: usize) -> Result<Arc<StudioSystem>, i32> {
    STUDIO_EVENTS
        .get_cloned(event)
        .ok_or(FMOD_ERR_INVALID_HANDLE)
}

#[inline]
pub(super) fn lookup_studio_instance(instance: usize) -> Result<Arc<StudioSystem>, i32> {
    STUDIO_INSTANCES
        .get_cloned(instance)
        .ok_or(FMOD_ERR_INVALID_HANDLE)
}

#[inline]
pub(super) fn lookup_studio_bus(bus: usize) -> Result<Arc<StudioSystem>, i32> {
    STUDIO_BUSES.get_cloned(bus).ok_or(FMOD_ERR_INVALID_HANDLE)
}

#[inline]
pub(super) fn lookup_studio_vca(vca: usize) -> Result<Arc<StudioSystem>, i32> {
    STUDIO_VCAS.get_cloned(vca).ok_or(FMOD_ERR_INVALID_HANDLE)
}

pub(super) fn create_core_system_arc() -> (usize, usize, Arc<FmodSystem>) {
    let sys_id = next_handle();
    let master_id = next_handle();
    let sys = Arc::new(FmodSystem {
        mixer: Arc::new(Mutex::new(MixerState::new(master_id))),
        audio: Mutex::new(None),
        mixed_frames: Arc::new(AtomicU64::new(0)),
    });
    SYSTEMS.insert_at(sys_id, sys.clone());
    GROUPS.insert_at(master_id, sys.clone());
    (sys_id, master_id, sys)
}

fn create_core_system() -> usize {
    create_core_system_arc().0
}

/// Pull mixed interleaved 48 kHz stereo float32 frames directly from `system`.
/// Primarily used for non-realtime (`FMOD_OUTPUTTYPE_NOSOUND_NRT`) mixing and tests.
pub fn mix_system_frames(system: usize, out_interleaved_stereo: &mut [f32]) -> Result<usize, i32> {
    let sys = lookup_system(system)?;
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let n = m.mix_frames(out_interleaved_stereo);
    sys.mixed_frames.fetch_add(n as u64, Ordering::Relaxed);
    Ok(n)
}

// ============================================================================
// FMOD Core — System implementations
// ============================================================================

pub(super) unsafe fn system_create(system_out: *mut usize, _header_version: u32) -> i32 {
    if system_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let id = create_core_system();
    write_opt(system_out, id);
    FMOD_OK
}

pub(super) fn system_init_impl(
    system: usize,
    maxchannels: i32,
    _flags: u32,
    _extradriverdata: usize,
) -> i32 {
    if maxchannels <= 0 {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_system(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let (output_type, driver_id) = {
        let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
        if m.initialized {
            return FMOD_ERR_INITIALIZED;
        }
        m.max_channels = maxchannels as usize;
        m.initialized = true;
        (m.output_type, m.driver_id)
    };

    match start_audio_output(&sys, output_type, driver_id) {
        Ok(audio_opt) => {
            *sys.audio.lock().unwrap_or_else(|e| e.into_inner()) = audio_opt;
            FMOD_OK
        }
        Err(e) => {
            let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
            m.initialized = false;
            e
        }
    }
}

pub(super) fn system_close_impl(system: usize) -> i32 {
    let sys = match lookup_system(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let audio = sys.audio.lock().unwrap_or_else(|e| e.into_inner()).take();
    drop(audio);

    let channel_ids: Vec<usize> = {
        let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
        let ids = m.channels.keys().copied().collect();
        m.channels.clear();
        m.initialized = false;
        ids
    };
    for cid in channel_ids {
        CHANNELS.remove(cid);
    }
    log::debug!(
        target: "audio",
        "fmod: closed system {system:#x} (mixed {} frames)",
        sys.mixed_frames.load(Ordering::Relaxed)
    );
    FMOD_OK
}

pub(super) fn system_release_impl(system: usize) -> i32 {
    let Some(sys) = SYSTEMS.remove(system) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    let audio = sys.audio.lock().unwrap_or_else(|e| e.into_inner()).take();
    drop(audio);

    let (sound_ids, group_ids, channel_ids): (Vec<usize>, Vec<usize>, Vec<usize>) = {
        let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
        let s = m.sounds.keys().copied().collect();
        let g = m.groups.keys().copied().collect();
        let c = m.channels.keys().copied().collect();
        m.sounds.clear();
        m.groups.clear();
        m.channels.clear();
        m.initialized = false;
        (s, g, c)
    };
    for id in sound_ids {
        SOUNDS.remove(id);
    }
    for id in group_ids {
        GROUPS.remove(id);
    }
    for id in channel_ids {
        CHANNELS.remove(id);
    }
    log::debug!(
        target: "audio",
        "fmod: released system {system:#x} (mixed {} frames)",
        sys.mixed_frames.load(Ordering::Relaxed)
    );
    FMOD_OK
}

pub(super) fn system_update_impl(system: usize) -> i32 {
    let sys = match lookup_system(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let pruned: Vec<usize> = {
        let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
        if m.channels.len() > 4096 {
            let dead: Vec<usize> = m
                .channels
                .iter()
                .filter(|(_, ch)| ch.stopped)
                .map(|(&id, _)| id)
                .collect();
            for id in &dead {
                m.channels.remove(id);
            }
            dead
        } else {
            Vec::new()
        }
    };
    for id in pruned {
        CHANNELS.remove(id);
    }
    FMOD_OK
}

pub(super) unsafe fn system_get_version(system: usize, version_out: *mut u32) -> i32 {
    if lookup_system(system).is_err() {
        return FMOD_ERR_INVALID_HANDLE;
    }
    if version_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let version = crate::middleware::active_version_tag("fmod").unwrap_or(FMOD_VERSION);
    write_opt(version_out, version);
    FMOD_OK
}

pub(super) fn system_set_output(system: usize, output: u32) -> i32 {
    let sys = match lookup_system(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    m.output_type = output;
    FMOD_OK
}

pub(super) unsafe fn system_get_output(system: usize, output_out: *mut u32) -> i32 {
    if output_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_system(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let out = sys
        .mixer
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .output_type;
    write_opt(output_out, out);
    FMOD_OK
}

pub(super) unsafe fn system_get_num_drivers(system: usize, numdrivers_out: *mut i32) -> i32 {
    if lookup_system(system).is_err() {
        return FMOD_ERR_INVALID_HANDLE;
    }
    if numdrivers_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let devs = backend::output_devices();
    write_opt(numdrivers_out, devs.len() as i32);
    FMOD_OK
}

pub(super) unsafe fn system_get_driver_info(
    system: usize,
    id: i32,
    name_out: *mut u8,
    namelen: i32,
    guid: usize,
    systemrate: *mut i32,
    speakermode: *mut u32,
    speakermodechannels: *mut i32,
) -> i32 {
    if lookup_system(system).is_err() {
        return FMOD_ERR_INVALID_HANDLE;
    }
    if id < 0 {
        return FMOD_ERR_INVALID_PARAM;
    }
    let devs = backend::output_devices();
    let Some(dev) = devs.get(id as usize) else {
        return FMOD_ERR_INVALID_PARAM;
    };

    if !name_out.is_null() && namelen > 0 {
        write_ansi_z(name_out, namelen as usize, &dev.name);
    }
    if guid != 0 {
        let g = guid_from_uid(&dev.uid);
        std::ptr::copy_nonoverlapping(g.as_ptr(), guid as *mut u8, 16);
    }
    write_opt(systemrate, dev.rate.max(8000) as i32);
    let ch = dev.channels.max(1);
    write_opt(speakermode, speakermode_for_channels(ch));
    write_opt(speakermodechannels, ch as i32);
    FMOD_OK
}

pub(super) fn system_set_driver(system: usize, driver: i32) -> i32 {
    if driver < 0 {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_system(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let devs = backend::output_devices();
    if (driver as usize) >= devs.len() {
        return FMOD_ERR_INVALID_PARAM;
    }
    sys.mixer
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .driver_id = driver;
    FMOD_OK
}

pub(super) unsafe fn system_get_driver(system: usize, driver_out: *mut i32) -> i32 {
    if driver_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_system(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let d = sys
        .mixer
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .driver_id;
    write_opt(driver_out, d);
    FMOD_OK
}

pub(super) unsafe fn system_get_master_channel_group(
    system: usize,
    group_out: *mut usize,
) -> i32 {
    if group_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_system(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let master_id = sys
        .mixer
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .master_group_id;
    write_opt(group_out, master_id);
    FMOD_OK
}

pub(super) unsafe fn system_create_channel_group(
    system: usize,
    name: *const u8,
    group_out: *mut usize,
) -> i32 {
    if group_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_system(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let group_name = if name.is_null() {
        String::new()
    } else {
        from_ansi(name)
    };
    let group_id = next_handle();
    {
        let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
        let master_id = m.master_group_id;
        m.groups.insert(
            group_id,
            ChannelGroup {
                name: group_name,
                parent: Some(master_id),
                volume: 1.0,
                mute: false,
                paused: false,
                pitch: 1.0,
            },
        );
    }
    GROUPS.insert_at(group_id, sys);
    write_opt(group_out, group_id);
    FMOD_OK
}

pub(super) unsafe fn system_set_3d_listener_attributes(
    system: usize,
    listener: i32,
    pos: *const FmodVector,
    vel: *const FmodVector,
    forward: *const FmodVector,
    up: *const FmodVector,
) -> i32 {
    if listener < 0 {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_system(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let p_opt = read_opt(pos);
    let v_opt = read_opt(vel);
    let f_opt = read_opt(forward);
    let u_opt = read_opt(up);
    for vec in [p_opt, v_opt, f_opt, u_opt].into_iter().flatten() {
        if !vec3_is_finite(vec) {
            return FMOD_ERR_INVALID_FLOAT;
        }
    }
    if listener == 0 {
        let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(p) = p_opt {
            m.listener.pos = p;
        }
        if let Some(v) = v_opt {
            m.listener.vel = v;
        }
        if let Some(f) = f_opt {
            m.listener.forward = f;
        }
        if let Some(u) = u_opt {
            m.listener.up = u;
        }
    }
    FMOD_OK
}

pub(super) unsafe fn system_get_3d_listener_attributes(
    system: usize,
    listener: i32,
    pos: *mut FmodVector,
    vel: *mut FmodVector,
    forward: *mut FmodVector,
    up: *mut FmodVector,
) -> i32 {
    if listener < 0 {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_system(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let l = sys.mixer.lock().unwrap_or_else(|e| e.into_inner()).listener;
    write_opt(pos, l.pos);
    write_opt(vel, l.vel);
    write_opt(forward, l.forward);
    write_opt(up, l.up);
    FMOD_OK
}

pub(super) fn system_set_3d_settings_impl(
    system: usize,
    dopplerscale: f32,
    distancefactor: f32,
    rolloffscale: f32,
) -> i32 {
    if !dopplerscale.is_finite()
        || !distancefactor.is_finite()
        || !rolloffscale.is_finite()
        || dopplerscale < 0.0
        || distancefactor <= 0.0
        || rolloffscale < 0.0
    {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let sys = match lookup_system(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    m.doppler_scale = dopplerscale;
    m.distance_factor = distancefactor;
    m.rolloff_scale = rolloffscale;
    FMOD_OK
}

pub(super) unsafe fn system_get_3d_settings(
    system: usize,
    dopplerscale: *mut f32,
    distancefactor: *mut f32,
    rolloffscale: *mut f32,
) -> i32 {
    let sys = match lookup_system(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    write_opt(dopplerscale, m.doppler_scale);
    write_opt(distancefactor, m.distance_factor);
    write_opt(rolloffscale, m.rolloff_scale);
    FMOD_OK
}

// ============================================================================
// FMOD Core — Sound implementations
// ============================================================================

pub(super) unsafe fn system_create_sound(
    system: usize,
    name_or_data: *const u8,
    mode: u32,
    exinfo: usize,
    sound_out: *mut usize,
) -> i32 {
    write_opt(sound_out, 0);
    if sound_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_system(system) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let ex = match read_exinfo(exinfo) {
        Ok(v) => v,
        Err(e) => return e,
    };

    let open_user = (mode & FMOD_OPENUSER) != 0;
    let open_mem = (mode & (FMOD_OPENMEMORY | FMOD_OPENMEMORY_POINT)) != 0;
    let open_raw = (mode & FMOD_OPENRAW) != 0;

    let sound = if open_user {
        let Some(info) = ex else {
            return FMOD_ERR_INVALID_PARAM;
        };
        if info.numchannels <= 0
            || info.numchannels > 32
            || info.defaultfrequency <= 0
            || info.length == 0
        {
            return FMOD_ERR_INVALID_PARAM;
        }
        let Some((bits, bps)) = format_bits_and_bytes(info.format) else {
            log::warn!(target: "audio", "FMOD_System_CreateSound(OPENUSER): unsupported PCM format {}", info.format);
            return FMOD_ERR_FORMAT;
        };
        let frame_bytes = (info.numchannels as usize) * bps;
        let aligned_len = ((info.length as usize) / frame_bytes) * frame_bytes;
        let raw = if info.format == FMOD_SOUND_FORMAT_PCM8 {
            vec![128u8; aligned_len]
        } else {
            vec![0u8; aligned_len]
        };
        let Some(decoded) = decode_pcm_to_f32(&raw, info.format) else {
            return FMOD_ERR_FORMAT;
        };
        let num_frames = decoded.len() / (info.numchannels as usize).max(1);
        let loop_count = if (mode & (FMOD_LOOP_NORMAL | FMOD_LOOP_BIDI)) != 0 {
            -1
        } else {
            0
        };
        Sound {
            name: String::new(),
            sound_type: FMOD_SOUND_TYPE_USER,
            format: info.format,
            channels: info.numchannels as u32,
            sample_rate: info.defaultfrequency as u32,
            bits_per_sample: bits,
            default_frequency: info.defaultfrequency as f32,
            default_priority: 128,
            mode,
            loop_count,
            loop_start: 0,
            loop_end: num_frames,
            raw_pcm: raw.into_boxed_slice(),
            samples: Arc::new(decoded),
            num_frames,
            min_distance: 1.0,
            max_distance: 10_000.0,
            locked: false,
            fsb_subsounds: Vec::new(),
            subsound_handles: Vec::new(),
        }
    } else {
        let mut sound_name = String::new();
        let source_bytes: Vec<u8> = if open_mem {
            let Some(info) = ex else {
                return FMOD_ERR_INVALID_PARAM;
            };
            if name_or_data.is_null() || info.length == 0 {
                return FMOD_ERR_INVALID_PARAM;
            }
            let start = name_or_data.add(info.fileoffset as usize);
            std::slice::from_raw_parts(start, info.length as usize).to_vec()
        } else {
            if name_or_data.is_null() {
                return FMOD_ERR_INVALID_PARAM;
            }
            let path_str = from_ansi(name_or_data);
            sound_name = path_str.clone();
            let Some(host_path) = resolve_sound_path(&path_str) else {
                return FMOD_ERR_FILE_NOTFOUND;
            };
            let data = match std::fs::read(&host_path) {
                Ok(d) => d,
                Err(_) => return FMOD_ERR_FILE_BAD,
            };
            if let Some(info) = ex {
                let off = info.fileoffset as usize;
                if off > data.len() {
                    return FMOD_ERR_FILE_EOF;
                }
                let rem = &data[off..];
                if info.length > 0 {
                    rem[..(info.length as usize).min(rem.len())].to_vec()
                } else {
                    rem.to_vec()
                }
            } else {
                data
            }
        };

        if open_raw {
            let Some(info) = ex else {
                return FMOD_ERR_INVALID_PARAM;
            };
            if info.numchannels <= 0 || info.numchannels > 32 || info.defaultfrequency <= 0 {
                return FMOD_ERR_INVALID_PARAM;
            }
            let Some((bits, bps)) = format_bits_and_bytes(info.format) else {
                log::warn!(target: "audio", "FMOD_System_CreateSound(OPENRAW): unsupported PCM format {}", info.format);
                return FMOD_ERR_FORMAT;
            };
            let frame_bytes = (info.numchannels as usize) * bps;
            let aligned_len = (source_bytes.len() / frame_bytes) * frame_bytes;
            let raw = source_bytes[..aligned_len].to_vec();
            let Some(decoded) = decode_pcm_to_f32(&raw, info.format) else {
                return FMOD_ERR_FORMAT;
            };
            let num_frames = decoded.len() / (info.numchannels as usize).max(1);
            let loop_count = if (mode & (FMOD_LOOP_NORMAL | FMOD_LOOP_BIDI)) != 0 {
                -1
            } else {
                0
            };
            Sound {
                name: sound_name,
                sound_type: FMOD_SOUND_TYPE_RAW,
                format: info.format,
                channels: info.numchannels as u32,
                sample_rate: info.defaultfrequency as u32,
                bits_per_sample: bits,
                default_frequency: info.defaultfrequency as f32,
                default_priority: 128,
                mode,
                loop_count,
                loop_start: 0,
                loop_end: num_frames,
                raw_pcm: raw.into_boxed_slice(),
                samples: Arc::new(decoded),
                num_frames,
                min_distance: 1.0,
                max_distance: 10_000.0,
                locked: false,
                fsb_subsounds: Vec::new(),
                subsound_handles: Vec::new(),
            }
        } else if source_bytes.len() >= 4 && &source_bytes[0..4] == b"FSB5" {
            let fsb = match parse_fsb5(&source_bytes) {
                Ok(f) => f,
                Err(e) => return e,
            };
            if fsb.samples.is_empty() {
                return FMOD_ERR_FORMAT;
            }
            let mut snd = sound_from_decoded_sample(&fsb.samples[0], mode);
            if snd.name.is_empty() {
                snd.name = sound_name;
            }
            let nsub = fsb.samples.len();
            snd.fsb_subsounds = fsb.samples;
            snd.subsound_handles = vec![None; nsub];
            snd
        } else {
            let wav = match parse_wav(&source_bytes) {
                Ok(w) => w,
                Err(e) => return e,
            };
            let Some(decoded) = decode_pcm_to_f32(&wav.raw_pcm, wav.format) else {
                return FMOD_ERR_FORMAT;
            };
            let num_frames = decoded.len() / (wav.channels as usize).max(1);
            let loop_count = if (mode & (FMOD_LOOP_NORMAL | FMOD_LOOP_BIDI)) != 0 {
                -1
            } else {
                0
            };
            Sound {
                name: sound_name,
                sound_type: FMOD_SOUND_TYPE_WAV,
                format: wav.format,
                channels: wav.channels,
                sample_rate: wav.sample_rate,
                bits_per_sample: wav.bits_per_sample,
                default_frequency: wav.sample_rate as f32,
                default_priority: 128,
                mode,
                loop_count,
                loop_start: 0,
                loop_end: num_frames,
                raw_pcm: wav.raw_pcm.into_boxed_slice(),
                samples: Arc::new(decoded),
                num_frames,
                min_distance: 1.0,
                max_distance: 10_000.0,
                locked: false,
                fsb_subsounds: Vec::new(),
                subsound_handles: Vec::new(),
            }
        }
    };

    let sound_id = next_handle();
    {
        let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
        m.sounds.insert(sound_id, sound);
    }
    SOUNDS.insert_at(sound_id, sys);
    write_opt(sound_out, sound_id);
    FMOD_OK
}

pub(super) fn sound_release(sound: usize) -> i32 {
    let Some(sys) = SOUNDS.remove(sound) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    let sub_ids: Vec<usize> = {
        let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
        let subs: Vec<usize> = m
            .sounds
            .remove(&sound)
            .map(|s| s.subsound_handles.into_iter().flatten().collect())
            .unwrap_or_default();
        for &sub in &subs {
            m.sounds.remove(&sub);
        }
        for ch in m.channels.values_mut() {
            if ch.sound_id == sound || subs.contains(&ch.sound_id) {
                ch.stopped = true;
            }
        }
        subs
    };
    for id in sub_ids {
        SOUNDS.remove(id);
    }
    FMOD_OK
}

pub(super) unsafe fn sound_get_num_sub_sounds(sound: usize, numsubsounds_out: *mut i32) -> i32 {
    if numsubsounds_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_sound_system(sound) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(s) = m.sounds.get(&sound) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(numsubsounds_out, s.fsb_subsounds.len() as i32);
    FMOD_OK
}

pub(super) unsafe fn sound_get_sub_sound(
    sound: usize,
    index: i32,
    subsound_out: *mut usize,
) -> i32 {
    write_opt(subsound_out, 0);
    if subsound_out.is_null() || index < 0 {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_sound_system(sound) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let idx = index as usize;
    let (sub_id, newly_created) = {
        let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
        let Some(parent) = m.sounds.get(&sound) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        if idx >= parent.fsb_subsounds.len() {
            return FMOD_ERR_INVALID_PARAM;
        }
        if let Some(existing) = parent.subsound_handles.get(idx).copied().flatten() {
            if m.sounds.contains_key(&existing) {
                (existing, false)
            } else {
                let sample = parent.fsb_subsounds[idx].clone();
                let mode = parent.mode;
                let sub_snd = sound_from_decoded_sample(&sample, mode);
                let new_id = next_handle();
                m.sounds.insert(new_id, sub_snd);
                if let Some(p_mut) = m.sounds.get_mut(&sound) {
                    p_mut.subsound_handles[idx] = Some(new_id);
                }
                (new_id, true)
            }
        } else {
            let sample = parent.fsb_subsounds[idx].clone();
            let mode = parent.mode;
            let sub_snd = sound_from_decoded_sample(&sample, mode);
            let new_id = next_handle();
            m.sounds.insert(new_id, sub_snd);
            if let Some(p_mut) = m.sounds.get_mut(&sound) {
                p_mut.subsound_handles[idx] = Some(new_id);
            }
            (new_id, true)
        }
    };
    if newly_created {
        SOUNDS.insert_at(sub_id, sys);
    }
    write_opt(subsound_out, sub_id);
    FMOD_OK
}

pub(super) unsafe fn sound_get_name(sound: usize, name_out: *mut u8, namelen: i32) -> i32 {
    if name_out.is_null() || namelen <= 0 {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_sound_system(sound) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(s) = m.sounds.get(&sound) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_ansi_z(name_out, namelen as usize, &s.name);
    FMOD_OK
}

pub(super) unsafe fn sound_get_length(
    sound: usize,
    length_out: *mut u32,
    lengthtype: u32,
) -> i32 {
    if length_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_sound_system(sound) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(s) = m.sounds.get(&sound) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    let val = match lengthtype {
        FMOD_TIMEUNIT_MS => {
            ((s.num_frames as u64).saturating_mul(1000) / (s.sample_rate.max(1) as u64)) as u32
        }
        FMOD_TIMEUNIT_PCM => s.num_frames as u32,
        FMOD_TIMEUNIT_PCMBYTES => {
            let bps = (s.bits_per_sample as usize) / 8;
            (s.num_frames * (s.channels as usize) * bps) as u32
        }
        FMOD_TIMEUNIT_RAWBYTES => s.raw_pcm.len() as u32,
        _ => return FMOD_ERR_INVALID_PARAM,
    };
    write_opt(length_out, val);
    FMOD_OK
}

pub(super) unsafe fn sound_get_format(
    sound: usize,
    sound_type_out: *mut u32,
    format_out: *mut u32,
    channels_out: *mut i32,
    bits_out: *mut i32,
) -> i32 {
    let sys = match lookup_sound_system(sound) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(s) = m.sounds.get(&sound) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(sound_type_out, s.sound_type);
    write_opt(format_out, s.format);
    write_opt(channels_out, s.channels as i32);
    write_opt(bits_out, s.bits_per_sample as i32);
    FMOD_OK
}

pub(super) unsafe fn sound_get_defaults(
    sound: usize,
    frequency_out: *mut f32,
    priority_out: *mut i32,
) -> i32 {
    let sys = match lookup_sound_system(sound) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(s) = m.sounds.get(&sound) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(frequency_out, s.default_frequency);
    write_opt(priority_out, s.default_priority);
    FMOD_OK
}

pub(super) fn sound_set_defaults_impl(sound: usize, frequency: f32, priority: i32) -> i32 {
    if !frequency.is_finite() {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let sys = match lookup_sound_system(sound) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(s) = m.sounds.get_mut(&sound) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    s.default_frequency = frequency;
    s.default_priority = priority.clamp(0, 256);
    FMOD_OK
}

pub(super) fn sound_set_mode(sound: usize, mode: u32) -> i32 {
    let sys = match lookup_sound_system(sound) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(s) = m.sounds.get_mut(&sound) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    s.mode = mode;
    if (mode & FMOD_LOOP_OFF) != 0 {
        s.loop_count = 0;
    } else if (mode & (FMOD_LOOP_NORMAL | FMOD_LOOP_BIDI)) != 0 && s.loop_count == 0 {
        s.loop_count = -1;
    }
    FMOD_OK
}

pub(super) unsafe fn sound_get_mode(sound: usize, mode_out: *mut u32) -> i32 {
    if mode_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_sound_system(sound) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(s) = m.sounds.get(&sound) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(mode_out, s.mode);
    FMOD_OK
}

pub(super) fn sound_set_loop_count(sound: usize, loopcount: i32) -> i32 {
    if loopcount < -1 {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_sound_system(sound) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(s) = m.sounds.get_mut(&sound) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    s.loop_count = loopcount;
    if loopcount == 0 {
        s.mode = (s.mode & !(FMOD_LOOP_NORMAL | FMOD_LOOP_BIDI)) | FMOD_LOOP_OFF;
    } else if (s.mode & FMOD_LOOP_BIDI) == 0 {
        s.mode = (s.mode & !FMOD_LOOP_OFF) | FMOD_LOOP_NORMAL;
    }
    FMOD_OK
}

pub(super) unsafe fn sound_get_loop_count(sound: usize, loopcount_out: *mut i32) -> i32 {
    if loopcount_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_sound_system(sound) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(s) = m.sounds.get(&sound) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(loopcount_out, s.loop_count);
    FMOD_OK
}

pub(super) fn sound_set_3d_min_max_distance_impl(sound: usize, min: f32, max: f32) -> i32 {
    if !min.is_finite() || !max.is_finite() || min < 0.0 || max < min {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let sys = match lookup_sound_system(sound) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(s) = m.sounds.get_mut(&sound) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    s.min_distance = min;
    s.max_distance = max;
    FMOD_OK
}

pub(super) unsafe fn sound_get_3d_min_max_distance(
    sound: usize,
    min_out: *mut f32,
    max_out: *mut f32,
) -> i32 {
    let sys = match lookup_sound_system(sound) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(s) = m.sounds.get(&sound) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(min_out, s.min_distance);
    write_opt(max_out, s.max_distance);
    FMOD_OK
}

pub(super) unsafe fn sound_lock(
    sound: usize,
    offset: u32,
    length: u32,
    ptr1: *mut *mut u8,
    ptr2: *mut *mut u8,
    len1: *mut u32,
    len2: *mut u32,
) -> i32 {
    if ptr1.is_null() || len1.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_sound_system(sound) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(s) = m.sounds.get_mut(&sound) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    if s.locked {
        return FMOD_ERR_ALREADY_LOCKED;
    }
    let total = s.raw_pcm.len();
    let off = offset as usize;
    let req = length as usize;
    if total == 0 || off >= total || req == 0 || req > total {
        return FMOD_ERR_INVALID_PARAM;
    }

    let first = req.min(total - off);
    let second = req - first;
    let base = s.raw_pcm.as_mut_ptr();
    s.locked = true;

    write_opt(ptr1, base.add(off));
    write_opt(len1, first as u32);
    if second > 0 {
        write_opt(ptr2, base);
        write_opt(len2, second as u32);
    } else {
        write_opt(ptr2, std::ptr::null_mut());
        write_opt(len2, 0);
    }
    FMOD_OK
}

pub(super) fn sound_unlock(
    sound: usize,
    _ptr1: *mut u8,
    _ptr2: *mut u8,
    len1: u32,
    len2: u32,
) -> i32 {
    let sys = match lookup_sound_system(sound) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let (new_samples, new_frames, new_loop_start, new_loop_end) = {
        let Some(s) = m.sounds.get_mut(&sound) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        if !s.locked {
            return FMOD_ERR_NOT_LOCKED;
        }
        if (len1 as usize).saturating_add(len2 as usize) > s.raw_pcm.len() {
            return FMOD_ERR_INVALID_PARAM;
        }
        s.locked = false;
        let Some(decoded) = decode_pcm_to_f32(&s.raw_pcm, s.format) else {
            return FMOD_ERR_FORMAT;
        };
        let nframes = decoded.len() / (s.channels as usize).max(1);
        let arc = Arc::new(decoded);
        s.samples = arc.clone();
        if s.loop_end == s.num_frames {
            s.loop_end = nframes;
        } else {
            s.loop_end = s.loop_end.min(nframes);
            s.loop_start = s.loop_start.min(s.loop_end);
        }
        s.num_frames = nframes;
        (arc, nframes, s.loop_start, s.loop_end)
    };

    for ch in m.channels.values_mut() {
        if ch.sound_id == sound {
            ch.samples = new_samples.clone();
            ch.num_frames = new_frames;
            ch.loop_start = new_loop_start;
            ch.loop_end = new_loop_end;
        }
    }
    FMOD_OK
}

// ============================================================================
// FMOD Core — Playback & Channel implementations
// ============================================================================

pub(super) unsafe fn system_play_sound(
    system: usize,
    sound: usize,
    channelgroup: usize,
    paused: i32,
    channel_out: *mut usize,
) -> i32 {
    write_opt(channel_out, 0);
    let sys = match lookup_system(system) {
        Ok(s) => s,
        Err(e) => return e,
    };

    let channel_id = next_handle();
    {
        let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
        if !m.initialized {
            return FMOD_ERR_UNINITIALIZED;
        }
        let target_group = if channelgroup == 0 {
            m.master_group_id
        } else if m.groups.contains_key(&channelgroup) {
            channelgroup
        } else {
            return FMOD_ERR_INVALID_HANDLE;
        };

        let active_count = m.channels.values().filter(|c| !c.stopped).count();
        if active_count >= m.max_channels {
            return FMOD_ERR_TOOMANYCHANNELS;
        }

        let Some(s) = m.sounds.get(&sound) else {
            return FMOD_ERR_INVALID_HANDLE;
        };
        let frame_bytes = (s.channels as usize) * ((s.bits_per_sample as usize) / 8).max(1);
        let ch = Channel {
            sound_id: sound,
            group_id: target_group,
            samples: s.samples.clone(),
            sound_channels: s.channels as usize,
            sound_sample_rate: s.sample_rate,
            sound_frame_bytes: frame_bytes,
            num_frames: s.num_frames,
            loop_start: s.loop_start,
            loop_end: s.loop_end,
            play_pos: 0.0,
            frequency: s.default_frequency,
            pitch: 1.0,
            volume: 1.0,
            pan: 0.0,
            mute: false,
            paused: paused != 0,
            stopped: false,
            mode: s.mode,
            loop_count: s.loop_count,
            loops_remaining: s.loop_count,
            pos_3d: FmodVector::default(),
            vel_3d: FmodVector::default(),
            min_distance: s.min_distance,
            max_distance: s.max_distance,
        };
        m.channels.insert(channel_id, ch);
    }
    CHANNELS.insert_at(channel_id, sys);
    write_opt(channel_out, channel_id);
    FMOD_OK
}

pub(super) fn channel_set_paused(channel: usize, paused: i32) -> i32 {
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get_mut(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    ch.paused = paused != 0;
    FMOD_OK
}

pub(super) unsafe fn channel_get_paused(channel: usize, paused_out: *mut i32) -> i32 {
    if paused_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(paused_out, if ch.paused { 1 } else { 0 });
    FMOD_OK
}

pub(super) fn channel_set_volume_impl(channel: usize, volume: f32) -> i32 {
    if !volume.is_finite() {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get_mut(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    ch.volume = volume;
    FMOD_OK
}

pub(super) unsafe fn channel_get_volume(channel: usize, volume_out: *mut f32) -> i32 {
    if volume_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(volume_out, ch.volume);
    FMOD_OK
}

pub(super) fn channel_set_pitch_impl(channel: usize, pitch: f32) -> i32 {
    if !pitch.is_finite() || pitch < 0.0 {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get_mut(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    ch.pitch = pitch;
    FMOD_OK
}

pub(super) unsafe fn channel_get_pitch(channel: usize, pitch_out: *mut f32) -> i32 {
    if pitch_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(pitch_out, ch.pitch);
    FMOD_OK
}

pub(super) fn channel_set_pan_impl(channel: usize, pan: f32) -> i32 {
    if !pan.is_finite() {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get_mut(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    ch.pan = pan.clamp(-1.0, 1.0);
    FMOD_OK
}

pub(super) fn channel_set_mute(channel: usize, mute: i32) -> i32 {
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get_mut(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    ch.mute = mute != 0;
    FMOD_OK
}

pub(super) unsafe fn channel_get_mute(channel: usize, mute_out: *mut i32) -> i32 {
    if mute_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(mute_out, if ch.mute { 1 } else { 0 });
    FMOD_OK
}

pub(super) fn channel_set_mode(channel: usize, mode: u32) -> i32 {
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get_mut(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    ch.mode = mode;
    if (mode & FMOD_LOOP_OFF) != 0 {
        ch.loop_count = 0;
        ch.loops_remaining = 0;
    } else if (mode & (FMOD_LOOP_NORMAL | FMOD_LOOP_BIDI)) != 0 && ch.loops_remaining == 0 {
        ch.loop_count = -1;
        ch.loops_remaining = -1;
    }
    FMOD_OK
}

pub(super) unsafe fn channel_get_mode(channel: usize, mode_out: *mut u32) -> i32 {
    if mode_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(mode_out, ch.mode);
    FMOD_OK
}

pub(super) unsafe fn channel_set_3d_attributes(
    channel: usize,
    pos: *const FmodVector,
    vel: *const FmodVector,
) -> i32 {
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let p_opt = read_opt(pos);
    let v_opt = read_opt(vel);
    for vec in [p_opt, v_opt].into_iter().flatten() {
        if !vec3_is_finite(vec) {
            return FMOD_ERR_INVALID_FLOAT;
        }
    }
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get_mut(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    if let Some(p) = p_opt {
        ch.pos_3d = p;
    }
    if let Some(v) = v_opt {
        ch.vel_3d = v;
    }
    FMOD_OK
}

pub(super) unsafe fn channel_get_3d_attributes(
    channel: usize,
    pos_out: *mut FmodVector,
    vel_out: *mut FmodVector,
) -> i32 {
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(pos_out, ch.pos_3d);
    write_opt(vel_out, ch.vel_3d);
    FMOD_OK
}

pub(super) fn channel_set_3d_min_max_distance_impl(
    channel: usize,
    mindistance: f32,
    maxdistance: f32,
) -> i32 {
    if !mindistance.is_finite()
        || !maxdistance.is_finite()
        || mindistance < 0.0
        || maxdistance < mindistance
    {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get_mut(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    ch.min_distance = mindistance;
    ch.max_distance = maxdistance;
    FMOD_OK
}

pub(super) unsafe fn channel_get_3d_min_max_distance(
    channel: usize,
    mindistance_out: *mut f32,
    maxdistance_out: *mut f32,
) -> i32 {
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(mindistance_out, ch.min_distance);
    write_opt(maxdistance_out, ch.max_distance);
    FMOD_OK
}

pub(super) fn channel_set_frequency_impl(channel: usize, frequency: f32) -> i32 {
    if !frequency.is_finite() {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get_mut(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    ch.frequency = frequency;
    FMOD_OK
}

pub(super) unsafe fn channel_get_frequency(channel: usize, frequency_out: *mut f32) -> i32 {
    if frequency_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(frequency_out, ch.frequency);
    FMOD_OK
}

pub(super) fn channel_set_loop_count(channel: usize, loopcount: i32) -> i32 {
    if loopcount < -1 {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get_mut(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    ch.loop_count = loopcount;
    ch.loops_remaining = loopcount;
    if loopcount == 0 {
        ch.mode = (ch.mode & !(FMOD_LOOP_NORMAL | FMOD_LOOP_BIDI)) | FMOD_LOOP_OFF;
    } else if (ch.mode & FMOD_LOOP_BIDI) == 0 {
        ch.mode = (ch.mode & !FMOD_LOOP_OFF) | FMOD_LOOP_NORMAL;
    }
    FMOD_OK
}

pub(super) unsafe fn channel_get_loop_count(channel: usize, loopcount_out: *mut i32) -> i32 {
    if loopcount_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(loopcount_out, ch.loop_count);
    FMOD_OK
}

pub(super) fn channel_set_position(channel: usize, position: u32, postype: u32) -> i32 {
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get_mut(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    let frame = match postype {
        FMOD_TIMEUNIT_PCM => position as usize,
        FMOD_TIMEUNIT_MS => {
            ((position as u64).saturating_mul(ch.sound_sample_rate as u64) / 1000) as usize
        }
        FMOD_TIMEUNIT_PCMBYTES => (position as usize) / ch.sound_frame_bytes.max(1),
        _ => return FMOD_ERR_INVALID_PARAM,
    };
    if frame > ch.num_frames {
        return FMOD_ERR_INVALID_POSITION;
    }
    ch.play_pos = frame as f64;
    FMOD_OK
}

pub(super) unsafe fn channel_get_position(
    channel: usize,
    position_out: *mut u32,
    postype: u32,
) -> i32 {
    if position_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    let cur_frame = (ch.play_pos.max(0.0).floor() as usize).min(ch.num_frames);
    let val = match postype {
        FMOD_TIMEUNIT_PCM => cur_frame as u32,
        FMOD_TIMEUNIT_MS => {
            ((cur_frame as u64).saturating_mul(1000) / (ch.sound_sample_rate.max(1) as u64)) as u32
        }
        FMOD_TIMEUNIT_PCMBYTES => (cur_frame * ch.sound_frame_bytes) as u32,
        _ => return FMOD_ERR_INVALID_PARAM,
    };
    write_opt(position_out, val);
    FMOD_OK
}

pub(super) unsafe fn channel_is_playing(channel: usize, isplaying_out: *mut i32) -> i32 {
    if isplaying_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => {
            write_opt(isplaying_out, 0);
            return e;
        }
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get(&channel) else {
        write_opt(isplaying_out, 0);
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(isplaying_out, if ch.stopped { 0 } else { 1 });
    FMOD_OK
}

pub(super) fn channel_stop(channel: usize) -> i32 {
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get_mut(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    ch.stopped = true;
    FMOD_OK
}

pub(super) fn channel_set_channel_group(channel: usize, channelgroup: usize) -> i32 {
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let target_group = if channelgroup == 0 {
        m.master_group_id
    } else if m.groups.contains_key(&channelgroup) {
        channelgroup
    } else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    let Some(ch) = m.channels.get_mut(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    ch.group_id = target_group;
    FMOD_OK
}

pub(super) unsafe fn channel_get_channel_group(
    channel: usize,
    channelgroup_out: *mut usize,
) -> i32 {
    if channelgroup_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_channel_system(channel) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(ch) = m.channels.get(&channel) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(channelgroup_out, ch.group_id);
    FMOD_OK
}

// ============================================================================
// FMOD Core — ChannelGroup implementations
// ============================================================================

pub(super) fn channel_group_set_volume_impl(channelgroup: usize, volume: f32) -> i32 {
    if !volume.is_finite() {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let sys = match lookup_group_system(channelgroup) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(g) = m.groups.get_mut(&channelgroup) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    g.volume = volume;
    FMOD_OK
}

pub(super) unsafe fn channel_group_get_volume(channelgroup: usize, volume_out: *mut f32) -> i32 {
    if volume_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_group_system(channelgroup) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(g) = m.groups.get(&channelgroup) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(volume_out, g.volume);
    FMOD_OK
}

pub(super) fn channel_group_set_mute(channelgroup: usize, mute: i32) -> i32 {
    let sys = match lookup_group_system(channelgroup) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(g) = m.groups.get_mut(&channelgroup) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    g.mute = mute != 0;
    FMOD_OK
}

pub(super) unsafe fn channel_group_get_mute(channelgroup: usize, mute_out: *mut i32) -> i32 {
    if mute_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_group_system(channelgroup) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(g) = m.groups.get(&channelgroup) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(mute_out, if g.mute { 1 } else { 0 });
    FMOD_OK
}

pub(super) fn channel_group_set_paused(channelgroup: usize, paused: i32) -> i32 {
    let sys = match lookup_group_system(channelgroup) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(g) = m.groups.get_mut(&channelgroup) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    g.paused = paused != 0;
    FMOD_OK
}

pub(super) unsafe fn channel_group_get_paused(channelgroup: usize, paused_out: *mut i32) -> i32 {
    if paused_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_group_system(channelgroup) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(g) = m.groups.get(&channelgroup) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(paused_out, if g.paused { 1 } else { 0 });
    FMOD_OK
}

pub(super) fn channel_group_set_pitch_impl(channelgroup: usize, pitch: f32) -> i32 {
    if !pitch.is_finite() || pitch < 0.0 {
        return FMOD_ERR_INVALID_FLOAT;
    }
    let sys = match lookup_group_system(channelgroup) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(g) = m.groups.get_mut(&channelgroup) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    g.pitch = pitch;
    FMOD_OK
}

pub(super) unsafe fn channel_group_get_pitch(channelgroup: usize, pitch_out: *mut f32) -> i32 {
    if pitch_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_group_system(channelgroup) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(g) = m.groups.get(&channelgroup) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(pitch_out, g.pitch);
    FMOD_OK
}

pub(super) fn channel_group_stop(channelgroup: usize) -> i32 {
    let sys = match lookup_group_system(channelgroup) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    if !m.groups.contains_key(&channelgroup) {
        return FMOD_ERR_INVALID_HANDLE;
    }
    let affected: Vec<usize> = m
        .groups
        .keys()
        .copied()
        .filter(|&gid| group_has_ancestor(&m.groups, gid, channelgroup))
        .collect();
    for ch in m.channels.values_mut() {
        if affected.contains(&ch.group_id) {
            ch.stopped = true;
        }
    }
    FMOD_OK
}

pub(super) unsafe fn channel_group_add_group(
    channelgroup: usize,
    group: usize,
    _propagatedspclock: i32,
    connection: *mut usize,
) -> i32 {
    write_opt(connection, 0);
    let parent_sys = match lookup_group_system(channelgroup) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let child_sys = match lookup_group_system(group) {
        Ok(s) => s,
        Err(e) => return e,
    };
    if !Arc::ptr_eq(&parent_sys, &child_sys) {
        return FMOD_ERR_INVALID_PARAM;
    }
    let mut m = parent_sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    if channelgroup == group || group == m.master_group_id {
        return FMOD_ERR_INVALID_PARAM;
    }
    if !m.groups.contains_key(&channelgroup) || !m.groups.contains_key(&group) {
        return FMOD_ERR_INVALID_HANDLE;
    }
    if group_has_ancestor(&m.groups, channelgroup, group) {
        return FMOD_ERR_INVALID_PARAM;
    }
    if let Some(child) = m.groups.get_mut(&group) {
        child.parent = Some(channelgroup);
    }
    FMOD_OK
}

pub(super) fn channel_group_release(channelgroup: usize) -> i32 {
    let sys = match lookup_group_system(channelgroup) {
        Ok(s) => s,
        Err(e) => return e,
    };
    {
        let mut m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
        if channelgroup == m.master_group_id {
            return FMOD_ERR_INVALID_PARAM;
        }
        if m.groups.remove(&channelgroup).is_none() {
            return FMOD_ERR_INVALID_HANDLE;
        }
        let master_id = m.master_group_id;
        for g in m.groups.values_mut() {
            if g.parent == Some(channelgroup) {
                g.parent = Some(master_id);
            }
        }
        for ch in m.channels.values_mut() {
            if ch.group_id == channelgroup {
                ch.group_id = master_id;
            }
        }
    }
    GROUPS.remove(channelgroup);
    FMOD_OK
}

pub(super) unsafe fn channel_group_get_parent_group(
    channelgroup: usize,
    group_out: *mut usize,
) -> i32 {
    if group_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_group_system(channelgroup) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(g) = m.groups.get(&channelgroup) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_opt(group_out, g.parent.unwrap_or(0));
    FMOD_OK
}

pub(super) unsafe fn channel_group_get_name(
    channelgroup: usize,
    name_out: *mut u8,
    namelen: i32,
) -> i32 {
    if name_out.is_null() || namelen <= 0 {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_group_system(channelgroup) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    let Some(g) = m.groups.get(&channelgroup) else {
        return FMOD_ERR_INVALID_HANDLE;
    };
    write_ansi_z(name_out, namelen as usize, &g.name);
    FMOD_OK
}

pub(super) unsafe fn channel_group_is_playing(
    channelgroup: usize,
    isplaying_out: *mut i32,
) -> i32 {
    if isplaying_out.is_null() {
        return FMOD_ERR_INVALID_PARAM;
    }
    let sys = match lookup_group_system(channelgroup) {
        Ok(s) => s,
        Err(e) => {
            write_opt(isplaying_out, 0);
            return e;
        }
    };
    let m = sys.mixer.lock().unwrap_or_else(|e| e.into_inner());
    if !m.groups.contains_key(&channelgroup) {
        write_opt(isplaying_out, 0);
        return FMOD_ERR_INVALID_HANDLE;
    }
    let playing = m
        .channels
        .values()
        .any(|ch| !ch.stopped && group_has_ancestor(&m.groups, ch.group_id, channelgroup));
    write_opt(isplaying_out, if playing { 1 } else { 0 });
    FMOD_OK
}
