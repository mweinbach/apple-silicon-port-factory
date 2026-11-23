//! XAPO DSP implementations, builtin COM effects (`IXAPO` + `IXAPOParameters`), and live
//! voice effect-chain attachment/execution (builtin fast-path + arbitrary guest COM `IXAPO`).

use super::mixer::{
    parse_wave_format, E_INVALIDARG, E_NOINTERFACE, E_OUTOFMEMORY, E_POINTER, S_OK,
    XAUDIO2_E_INVALID_CALL, XAUDIO2_MAX_AUDIO_CHANNELS, XAUDIO2_MAX_SAMPLE_RATE,
    XAUDIO2_MIN_SAMPLE_RATE,
};
use super::voices::{ensure_vtbls_registered, Vtable};
use crate::audio::com::{GUID, IID_IUnknown};
use crate::audio::format::{SampleType, WAVEFORMATEX, WAVE_FORMAT_IEEE_FLOAT};
use crate::com::{com_call_qi, com_call_release, com_call_slot_raw, write_null_out, Guid};
use crate::strings::{rd, wr};
use std::f32::consts::TAU;
use std::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

// ============================================================================
// XAPO Constants, CLSIDs, IIDs & ABI Structures
// ============================================================================

pub(super) const REGDB_E_CLASSNOTREG: i32 = 0x8004_0154u32 as i32;
pub(super) const XAPO_E_FORMAT_UNSUPPORTED: i32 = 0x8897_0001u32 as i32;

pub(super) const IID_IXAPO: GUID = GUID::new(
    0xA410_B984,
    0x9839,
    0x4819,
    [0xA0, 0xBE, 0x28, 0x56, 0xAE, 0x6B, 0x3A, 0xDB],
);
pub(super) const IID_IXAPO_PARAMETERS: GUID = GUID::new(
    0x26D9_5C66,
    0x80F2,
    0x499A,
    [0xAD, 0x54, 0x5A, 0xE7, 0xF0, 0x1C, 0x6D, 0x98],
);

pub(super) const CLSID_AUDIO_REVERB: GUID = GUID::new(
    0xC263_3B16,
    0x471B,
    0x4498,
    [0xB8, 0xC5, 0x4F, 0x09, 0x59, 0xE2, 0xEC, 0x09],
);
pub(super) const CLSID_AUDIO_VOLUME_METER: GUID = GUID::new(
    0x4FC3_B166,
    0x972A,
    0x40CF,
    [0xBC, 0x37, 0x7D, 0xB0, 0x3D, 0xB2, 0xFB, 0xA3],
);
pub(super) const CLSID_FX_EQ: GUID = GUID::new(
    0xF5E0_1117,
    0xD6C4,
    0x485A,
    [0xA3, 0xF5, 0x69, 0x51, 0x96, 0xF3, 0xDB, 0xFA],
);
pub(super) const CLSID_FX_MASTERING_LIMITER: GUID = GUID::new(
    0xC413_7916,
    0x2BE1,
    0x46FD,
    [0x85, 0x99, 0x44, 0x15, 0x36, 0xF4, 0x98, 0x56],
);
pub(super) const CLSID_FX_REVERB: GUID = GUID::new(
    0x7D9A_CA56,
    0xCB68,
    0x4807,
    [0xB6, 0x32, 0xB1, 0x37, 0x35, 0x2E, 0x85, 0x96],
);
pub(super) const CLSID_FX_ECHO: GUID = GUID::new(
    0x5039_D740,
    0xF736,
    0x449A,
    [0x84, 0xD3, 0xA5, 0x62, 0x02, 0x55, 0xF4, 0x87],
);
// Legacy DirectX SDK (XAudio2.7 / XAPOFX1_5) CLSIDs.
pub(super) const CLSID_FX_EQ_27: GUID = GUID::new(
    0xA90B_C001,
    0xE897,
    0xE897,
    [0x74, 0x39, 0x43, 0x55, 0x00, 0x00, 0x00, 0x00],
);
pub(super) const CLSID_FX_MASTERING_LIMITER_27: GUID = GUID::new(
    0xA90B_C001,
    0xE897,
    0xE897,
    [0x74, 0x39, 0x43, 0x55, 0x00, 0x00, 0x00, 0x01],
);
pub(super) const CLSID_FX_REVERB_27: GUID = GUID::new(
    0xA90B_C001,
    0xE897,
    0xE897,
    [0x74, 0x39, 0x43, 0x55, 0x00, 0x00, 0x00, 0x02],
);
pub(super) const CLSID_FX_ECHO_27: GUID = GUID::new(
    0xA90B_C001,
    0xE897,
    0xE897,
    [0x74, 0x39, 0x43, 0x55, 0x00, 0x00, 0x00, 0x03],
);

const XAPO_REGISTRATION_STRING_LENGTH: usize = 256;
const XAPO_FLAG_CHANNELS_MUST_MATCH: u32 = 0x0000_0001;
const XAPO_FLAG_FRAMERATE_MUST_MATCH: u32 = 0x0000_0002;
const XAPO_FLAG_BITSPERSAMPLE_MUST_MATCH: u32 = 0x0000_0004;
const XAPO_FLAG_BUFFERCOUNT_MUST_MATCH: u32 = 0x0000_0008;
const XAPO_FLAG_INPLACE_SUPPORTED: u32 = 0x0000_0010;
const XAPOBASE_DEFAULT_FLAG: u32 = XAPO_FLAG_CHANNELS_MUST_MATCH
    | XAPO_FLAG_FRAMERATE_MUST_MATCH
    | XAPO_FLAG_BITSPERSAMPLE_MUST_MATCH
    | XAPO_FLAG_BUFFERCOUNT_MUST_MATCH
    | XAPO_FLAG_INPLACE_SUPPORTED;

pub(super) const XAPO_BUFFER_SILENT: u32 = 0;
pub(super) const XAPO_BUFFER_VALID: u32 = 1;

const FXEQ_MIN_FREQUENCY_CENTER: f32 = 20.0;
const FXEQ_MAX_FREQUENCY_CENTER: f32 = 20_000.0;
const FXEQ_MIN_GAIN: f32 = 0.126;
const FXEQ_MAX_GAIN: f32 = 7.94;
const FXEQ_MIN_BANDWIDTH: f32 = 0.1;
const FXEQ_MAX_BANDWIDTH: f32 = 2.0;

const FXMASTERINGLIMITER_MIN_RELEASE: u32 = 1;
const FXMASTERINGLIMITER_MAX_RELEASE: u32 = 20;
const FXMASTERINGLIMITER_DEFAULT_RELEASE: u32 = 6;
const FXMASTERINGLIMITER_MIN_LOUDNESS: u32 = 1;
const FXMASTERINGLIMITER_MAX_LOUDNESS: u32 = 1800;
const FXMASTERINGLIMITER_DEFAULT_LOUDNESS: u32 = 1000;

const FXREVERB_MIN_DIFFUSION: f32 = 0.0;
const FXREVERB_MAX_DIFFUSION: f32 = 1.0;
const FXREVERB_DEFAULT_DIFFUSION: f32 = 0.9;
const FXREVERB_MIN_ROOMSIZE: f32 = 0.0001;
const FXREVERB_MAX_ROOMSIZE: f32 = 1.0;
const FXREVERB_DEFAULT_ROOMSIZE: f32 = 0.6;

const FXECHO_MIN_WETDRYMIX: f32 = 0.0;
const FXECHO_MAX_WETDRYMIX: f32 = 1.0;
const FXECHO_DEFAULT_WETDRYMIX: f32 = 0.5;
const FXECHO_MIN_FEEDBACK: f32 = 0.0;
const FXECHO_MAX_FEEDBACK: f32 = 1.0;
const FXECHO_DEFAULT_FEEDBACK: f32 = 0.5;
const FXECHO_MIN_DELAY: f32 = 1.0;
const FXECHO_MAX_DELAY: f32 = 2000.0;
const FXECHO_DEFAULT_DELAY: f32 = 500.0;

const BUILTIN_XAPO_MAGIC: u64 = 0x5841_504F_4255_494C; // "XAPOBUIL"
const XAPO_DEFAULT_MAX_FRAMES: u32 = 4096;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub(super) struct XAudio2EffectDescriptor {
    pub(super) effect: *mut c_void,
    pub(super) initial_state: i32,
    pub(super) output_channels: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub(super) struct XAudio2EffectChain {
    pub(super) effect_count: u32,
    pub(super) effects: *const XAudio2EffectDescriptor,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub(super) struct XapoRegistrationProperties {
    pub(super) clsid: GUID,
    pub(super) friendly_name: [u16; XAPO_REGISTRATION_STRING_LENGTH],
    pub(super) copyright_info: [u16; XAPO_REGISTRATION_STRING_LENGTH],
    pub(super) major_version: u32,
    pub(super) minor_version: u32,
    pub(super) flags: u32,
    pub(super) min_input_buffer_count: u32,
    pub(super) max_input_buffer_count: u32,
    pub(super) min_output_buffer_count: u32,
    pub(super) max_output_buffer_count: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub(super) struct XapoLockForProcessBufferParameters {
    pub(super) format: *const WAVEFORMATEX,
    pub(super) max_frame_count: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub(super) struct XapoProcessBufferParameters {
    pub(super) buffer: *mut c_void,
    pub(super) buffer_flags: u32,
    pub(super) valid_frame_count: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub(super) struct XAudio2FxReverbParameters {
    pub(super) wet_dry_mix: f32,
    pub(super) reflections_delay: u32,
    pub(super) reverb_delay: u8,
    pub(super) rear_delay: u8,
    pub(super) side_delay: u8,
    pub(super) position_left: u8,
    pub(super) position_right: u8,
    pub(super) position_matrix_left: u8,
    pub(super) position_matrix_right: u8,
    pub(super) early_diffusion: u8,
    pub(super) late_diffusion: u8,
    pub(super) low_eq_gain: u8,
    pub(super) low_eq_cutoff: u8,
    pub(super) high_eq_gain: u8,
    pub(super) high_eq_cutoff: u8,
    pub(super) room_filter_freq: f32,
    pub(super) room_filter_main: f32,
    pub(super) room_filter_hf: f32,
    pub(super) reflections_gain: f32,
    pub(super) reverb_gain: f32,
    pub(super) decay_time: f32,
    pub(super) density: f32,
    pub(super) room_size: f32,
    pub(super) disable_late_field: i32,
}

impl Default for XAudio2FxReverbParameters {
    fn default() -> Self {
        Self {
            wet_dry_mix: 100.0,
            reflections_delay: 5,
            reverb_delay: 5,
            rear_delay: 5,
            side_delay: 5,
            position_left: 6,
            position_right: 6,
            position_matrix_left: 27,
            position_matrix_right: 27,
            early_diffusion: 8,
            late_diffusion: 8,
            low_eq_gain: 8,
            low_eq_cutoff: 4,
            high_eq_gain: 8,
            high_eq_cutoff: 4,
            room_filter_freq: 5000.0,
            room_filter_main: 0.0,
            room_filter_hf: 0.0,
            reflections_gain: 0.0,
            reverb_gain: 0.0,
            decay_time: 1.0,
            density: 100.0,
            room_size: 100.0,
            disable_late_field: 0,
        }
    }
}

/// XAudio 2.8 layout (no `side_delay`, has `disable_late_field`).
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub(super) struct XAudio2FxReverbParameters28 {
    pub(super) wet_dry_mix: f32,
    pub(super) reflections_delay: u32,
    pub(super) reverb_delay: u8,
    pub(super) rear_delay: u8,
    pub(super) position_left: u8,
    pub(super) position_right: u8,
    pub(super) position_matrix_left: u8,
    pub(super) position_matrix_right: u8,
    pub(super) early_diffusion: u8,
    pub(super) late_diffusion: u8,
    pub(super) low_eq_gain: u8,
    pub(super) low_eq_cutoff: u8,
    pub(super) high_eq_gain: u8,
    pub(super) high_eq_cutoff: u8,
    pub(super) room_filter_freq: f32,
    pub(super) room_filter_main: f32,
    pub(super) room_filter_hf: f32,
    pub(super) reflections_gain: f32,
    pub(super) reverb_gain: f32,
    pub(super) decay_time: f32,
    pub(super) density: f32,
    pub(super) room_size: f32,
    pub(super) disable_late_field: i32,
}

/// XAudio 2.7 layout (no `side_delay` and no `disable_late_field`).
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub(super) struct XAudio2FxReverbParameters27 {
    pub(super) wet_dry_mix: f32,
    pub(super) reflections_delay: u32,
    pub(super) reverb_delay: u8,
    pub(super) rear_delay: u8,
    pub(super) position_left: u8,
    pub(super) position_right: u8,
    pub(super) position_matrix_left: u8,
    pub(super) position_matrix_right: u8,
    pub(super) early_diffusion: u8,
    pub(super) late_diffusion: u8,
    pub(super) low_eq_gain: u8,
    pub(super) low_eq_cutoff: u8,
    pub(super) high_eq_gain: u8,
    pub(super) high_eq_cutoff: u8,
    pub(super) room_filter_freq: f32,
    pub(super) room_filter_main: f32,
    pub(super) room_filter_hf: f32,
    pub(super) reflections_gain: f32,
    pub(super) reverb_gain: f32,
    pub(super) decay_time: f32,
    pub(super) density: f32,
    pub(super) room_size: f32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub(super) struct XAudio2FxVolumeMeterLevels {
    pub(super) peak_levels: *mut f32,
    pub(super) rms_levels: *mut f32,
    pub(super) channel_count: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub(super) struct FxEqParameters {
    pub(super) frequency_center0: f32,
    pub(super) gain0: f32,
    pub(super) bandwidth0: f32,
    pub(super) frequency_center1: f32,
    pub(super) gain1: f32,
    pub(super) bandwidth1: f32,
    pub(super) frequency_center2: f32,
    pub(super) gain2: f32,
    pub(super) bandwidth2: f32,
    pub(super) frequency_center3: f32,
    pub(super) gain3: f32,
    pub(super) bandwidth3: f32,
}

impl Default for FxEqParameters {
    fn default() -> Self {
        Self {
            frequency_center0: 100.0,
            gain0: 1.0,
            bandwidth0: 1.0,
            frequency_center1: 800.0,
            gain1: 1.0,
            bandwidth1: 1.0,
            frequency_center2: 2000.0,
            gain2: 1.0,
            bandwidth2: 1.0,
            frequency_center3: 10_000.0,
            gain3: 1.0,
            bandwidth3: 1.0,
        }
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub(super) struct FxMasteringLimiterParameters {
    pub(super) release: u32,
    pub(super) loudness: u32,
}

impl Default for FxMasteringLimiterParameters {
    fn default() -> Self {
        Self {
            release: FXMASTERINGLIMITER_DEFAULT_RELEASE,
            loudness: FXMASTERINGLIMITER_DEFAULT_LOUDNESS,
        }
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub(super) struct FxReverbParameters {
    pub(super) diffusion: f32,
    pub(super) room_size: f32,
}

impl Default for FxReverbParameters {
    fn default() -> Self {
        Self {
            diffusion: FXREVERB_DEFAULT_DIFFUSION,
            room_size: FXREVERB_DEFAULT_ROOMSIZE,
        }
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub(super) struct FxEchoParameters {
    pub(super) wet_dry_mix: f32,
    pub(super) feedback: f32,
    pub(super) delay: f32,
}

impl Default for FxEchoParameters {
    fn default() -> Self {
        Self {
            wet_dry_mix: FXECHO_DEFAULT_WETDRYMIX,
            feedback: FXECHO_DEFAULT_FEEDBACK,
            delay: FXECHO_DEFAULT_DELAY,
        }
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub(super) struct FxEchoInitData {
    pub(super) max_delay: f32,
}

const _: () = assert!(std::mem::size_of::<XAudio2EffectDescriptor>() == 16);
const _: () = assert!(std::mem::size_of::<XAudio2EffectChain>() == 12);
const _: () = assert!(std::mem::size_of::<XapoRegistrationProperties>() == 1068);
const _: () = assert!(std::mem::size_of::<XapoLockForProcessBufferParameters>() == 12);
const _: () = assert!(std::mem::size_of::<XapoProcessBufferParameters>() == 16);
const _: () = assert!(std::mem::size_of::<XAudio2FxReverbParameters>() == 57);
const _: () = assert!(std::mem::size_of::<XAudio2FxReverbParameters28>() == 56);
const _: () = assert!(std::mem::size_of::<XAudio2FxReverbParameters27>() == 52);
const _: () = assert!(std::mem::size_of::<XAudio2FxVolumeMeterLevels>() == 20);
const _: () = assert!(std::mem::size_of::<FxEqParameters>() == 48);
const _: () = assert!(std::mem::size_of::<FxMasteringLimiterParameters>() == 8);
const _: () = assert!(std::mem::size_of::<FxReverbParameters>() == 8);
const _: () = assert!(std::mem::size_of::<FxEchoParameters>() == 12);
const _: () = assert!(std::mem::size_of::<FxEchoInitData>() == 4);

// ============================================================================
// XAPO COM Vtables
// ============================================================================

pub(super) static XAPO_VTBL: Vtable<13> = Vtable([
    crate::Ptr(xapo_query_interface as *const ()),             // 0 (+0x00)
    crate::Ptr(xapo_add_ref as *const ()),                     // 1 (+0x08)
    crate::Ptr(xapo_release as *const ()),                     // 2 (+0x10)
    crate::Ptr(xapo_get_registration_properties as *const ()), // 3 (+0x18)
    crate::Ptr(xapo_is_input_format_supported as *const ()),   // 4 (+0x20)
    crate::Ptr(xapo_is_output_format_supported as *const ()),  // 5 (+0x28)
    crate::Ptr(xapo_initialize as *const ()),                  // 6 (+0x30)
    crate::Ptr(xapo_reset as *const ()),                       // 7 (+0x38)
    crate::Ptr(xapo_lock_for_process as *const ()),            // 8 (+0x40)
    crate::Ptr(xapo_unlock_for_process as *const ()),          // 9 (+0x48)
    crate::Ptr(xapo_process as *const ()),                     // 10 (+0x50)
    crate::Ptr(xapo_calc_input_frames as *const ()),           // 11 (+0x58)
    crate::Ptr(xapo_calc_output_frames as *const ()),          // 12 (+0x60)
]);

pub(super) static XAPO_PARAMS_VTBL: Vtable<5> = Vtable([
    crate::Ptr(xapo_params_query_interface as *const ()), // 0 (+0x00)
    crate::Ptr(xapo_params_add_ref as *const ()),         // 1 (+0x08)
    crate::Ptr(xapo_params_release as *const ()),         // 2 (+0x10)
    crate::Ptr(xapo_params_set_parameters as *const ()),  // 3 (+0x18)
    crate::Ptr(xapo_params_get_parameters as *const ()),  // 4 (+0x20)
]);

// ============================================================================
// Helpers & 16-Byte Aligned XAPO Scratch Buffer
// ============================================================================

#[inline]
fn flush_denormal(x: f32) -> f32 {
    if !x.is_finite() || x.abs() < 1e-20 {
        0.0
    } else {
        x
    }
}

#[inline]
fn db_to_lin(db: f32) -> f32 {
    if !db.is_finite() || db <= -100.0 {
        0.0
    } else {
        10.0_f32.powf(db * 0.05)
    }
}

/// 16-byte aligned reusable `f32` buffer for XAPO `Process` calls (XAPO requires
/// 16-byte aligned `pBuffer` pointers for SIMD/SSE guest effects).
#[repr(C, align(16))]
#[derive(Clone, Copy, Default)]
struct AlignedF32Chunk([f32; 4]);

#[derive(Default)]
pub(super) struct AlignedF32Buf {
    chunks: Vec<AlignedF32Chunk>,
    len: usize,
}

impl AlignedF32Buf {
    fn resize_zero(&mut self, len: usize) {
        let needed = len.div_ceil(4);
        self.chunks.clear();
        self.chunks.resize(needed, AlignedF32Chunk([0.0; 4]));
        self.len = len;
    }

    fn copy_from_slice(&mut self, src: &[f32]) {
        let needed = src.len().div_ceil(4);
        if self.chunks.len() < needed {
            self.chunks.resize(needed, AlignedF32Chunk([0.0; 4]));
        } else if needed > 0 {
            self.chunks[needed - 1] = AlignedF32Chunk([0.0; 4]);
        }
        self.len = src.len();
        if !src.is_empty() {
            self.as_mut_slice().copy_from_slice(src);
        }
    }

    fn as_slice(&self) -> &[f32] {
        if self.len == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(self.chunks.as_ptr() as *const f32, self.len) }
        }
    }

    fn as_mut_slice(&mut self) -> &mut [f32] {
        if self.len == 0 {
            &mut []
        } else {
            unsafe {
                std::slice::from_raw_parts_mut(self.chunks.as_mut_ptr() as *mut f32, self.len)
            }
        }
    }

    fn as_mut_ptr(&mut self) -> *mut f32 {
        if self.chunks.is_empty() {
            self.chunks.push(AlignedF32Chunk([0.0; 4]));
        }
        self.chunks.as_mut_ptr() as *mut f32
    }
}

fn adapt_channels(src: &[f32], src_ch: usize, dst: &mut [f32], dst_ch: usize, frames: usize) {
    if frames == 0 || src_ch == 0 || dst_ch == 0 {
        return;
    }
    if src_ch == dst_ch {
        let n = frames * src_ch;
        dst[..n].copy_from_slice(&src[..n]);
        return;
    }
    if src_ch == 1 {
        for f in 0..frames {
            let v = src[f];
            let d = &mut dst[f * dst_ch..(f + 1) * dst_ch];
            d.fill(v);
        }
        return;
    }
    if dst_ch == 1 {
        let inv = 1.0 / (src_ch as f32);
        for f in 0..frames {
            let s = &src[f * src_ch..(f + 1) * src_ch];
            let mut acc = 0.0f32;
            for &v in s {
                acc += v;
            }
            dst[f] = acc * inv;
        }
        return;
    }
    let copy_ch = src_ch.min(dst_ch);
    for f in 0..frames {
        let s = &src[f * src_ch..(f + 1) * src_ch];
        let d = &mut dst[f * dst_ch..(f + 1) * dst_ch];
        d[..copy_ch].copy_from_slice(&s[..copy_ch]);
        for v in &mut d[copy_ch..] {
            *v = 0.0;
        }
    }
}

fn write_wide_fixed<const N: usize>(dst: &mut [u16; N], s: &str) {
    dst.fill(0);
    for (slot, unit) in dst.iter_mut().take(N.saturating_sub(1)).zip(s.encode_utf16()) {
        *slot = unit;
    }
}

fn make_float_wfx(channels: usize, sample_rate: u32) -> WAVEFORMATEX {
    let ch = channels.clamp(1, XAUDIO2_MAX_AUDIO_CHANNELS as usize) as u16;
    let block_align = ch * 4;
    WAVEFORMATEX {
        wFormatTag: WAVE_FORMAT_IEEE_FLOAT,
        nChannels: ch,
        nSamplesPerSec: sample_rate,
        nAvgBytesPerSec: sample_rate.saturating_mul(block_align as u32),
        nBlockAlign: block_align,
        wBitsPerSample: 32,
        cbSize: 0,
    }
}

unsafe fn parse_xapo_float_format(ptr: *const WAVEFORMATEX) -> Result<(usize, u32), i32> {
    let fmt = parse_wave_format(ptr).map_err(|_| XAPO_E_FORMAT_UNSUPPORTED)?;
    if fmt.sample_kind != SampleType::F32 {
        return Err(XAPO_E_FORMAT_UNSUPPORTED);
    }
    Ok((fmt.channels, fmt.sample_rate))
}

// ============================================================================
// Delay Line, Comb & Allpass Primitives
// ============================================================================

#[derive(Clone)]
struct DelayLine {
    buf: Vec<f32>,
    write_idx: usize,
}

impl DelayLine {
    fn new(max_samples: usize) -> Self {
        Self {
            buf: vec![0.0; max_samples.max(2)],
            write_idx: 0,
        }
    }

    fn reset(&mut self) {
        self.buf.fill(0.0);
        self.write_idx = 0;
    }

    #[inline]
    fn write(&mut self, sample: f32) {
        self.buf[self.write_idx] = flush_denormal(sample);
        self.write_idx += 1;
        if self.write_idx >= self.buf.len() {
            self.write_idx = 0;
        }
    }

    #[inline]
    fn read_int(&self, delay_samples: usize) -> f32 {
        let len = self.buf.len();
        let d = delay_samples.min(len - 1);
        let idx = if self.write_idx >= d {
            self.write_idx - d
        } else {
            len + self.write_idx - d
        };
        self.buf[idx]
    }

    #[inline]
    fn read_frac(&self, delay_samples: f32) -> f32 {
        let len = self.buf.len();
        let max_d = (len - 2) as f32;
        let d = delay_samples.clamp(0.0, max_d);
        let d0 = d.floor() as usize;
        let frac = d - d0 as f32;
        let d1 = d0 + 1;
        let s0 = self.read_int(d0);
        let s1 = self.read_int(d1);
        s0 + (s1 - s0) * frac
    }
}

#[derive(Clone)]
struct CombFilter {
    buf: Vec<f32>,
    active_len: usize,
    idx: usize,
    filter_store: f32,
}

impl CombFilter {
    fn new(max_len: usize, active_len: usize) -> Self {
        let cap = max_len.max(16);
        Self {
            buf: vec![0.0; cap],
            active_len: active_len.clamp(8, cap),
            idx: 0,
            filter_store: 0.0,
        }
    }

    fn set_active_len(&mut self, new_len: usize) {
        self.active_len = new_len.clamp(8, self.buf.len());
        if self.idx >= self.active_len {
            self.idx = 0;
        }
    }

    fn reset(&mut self) {
        self.buf.fill(0.0);
        self.idx = 0;
        self.filter_store = 0.0;
    }

    #[inline]
    fn process(&mut self, input: f32, feedback: f32, damp1: f32, damp2: f32) -> f32 {
        let out = self.buf[self.idx];
        self.filter_store = flush_denormal(out * damp2 + self.filter_store * damp1);
        self.buf[self.idx] = flush_denormal(input + self.filter_store * feedback);
        self.idx += 1;
        if self.idx >= self.active_len {
            self.idx = 0;
        }
        out
    }
}

#[derive(Clone)]
struct AllpassFilter {
    buf: Vec<f32>,
    idx: usize,
}

impl AllpassFilter {
    fn new(len: usize) -> Self {
        Self {
            buf: vec![0.0; len.max(8)],
            idx: 0,
        }
    }

    fn reset(&mut self) {
        self.buf.fill(0.0);
        self.idx = 0;
    }

    #[inline]
    fn process(&mut self, input: f32, feedback: f32) -> f32 {
        let buf_out = self.buf[self.idx];
        let out = buf_out - input * feedback;
        self.buf[self.idx] = flush_denormal(input + buf_out * feedback);
        self.idx += 1;
        if self.idx >= self.buf.len() {
            self.idx = 0;
        }
        out
    }
}

// ============================================================================
// Schroeder/Freeverb Multichannel Reverb Engine (AudioReverb & FXReverb)
// ============================================================================

const FREEVERB_COMB_TUNINGS: [usize; 8] = [1116, 1188, 1277, 1356, 1422, 1491, 1557, 1617];
const FREEVERB_ALLPASS_TUNINGS: [usize; 4] = [556, 441, 341, 225];
const EARLY_TAP_MS: [f32; 4] = [0.0, 7.3, 17.1, 31.7];
const EARLY_TAP_GAINS: [f32; 4] = [0.55, -0.40, 0.30, -0.22];

struct ReverbChannel {
    predelay: DelayLine,
    room_lp: f32,
    low_shelf_lp: f32,
    combs: Vec<CombFilter>,
    allpasses: Vec<AllpassFilter>,
    out_delay: DelayLine,
}

impl ReverbChannel {
    fn new(sample_rate: u32, ch_idx: usize) -> Self {
        let sr_scale = (sample_rate as f32) / 44_100.0;
        let predelay_samples = ((sample_rate as f32) * 0.45).ceil() as usize + 64;
        let spread = (ch_idx % 8) * 23;
        let ap_spread = (ch_idx % 8) * 13;
        let combs = FREEVERB_COMB_TUNINGS
            .iter()
            .map(|&base| {
                let full_len = (((base + spread) as f32) * sr_scale).round() as usize;
                CombFilter::new(full_len.max(16), full_len.max(16))
            })
            .collect();
        let allpasses = FREEVERB_ALLPASS_TUNINGS
            .iter()
            .map(|&base| {
                let len = (((base + ap_spread) as f32) * sr_scale).round() as usize;
                AllpassFilter::new(len.max(8))
            })
            .collect();
        let out_delay_cap = ((sample_rate as f32) * 0.01).ceil() as usize + 8;
        Self {
            predelay: DelayLine::new(predelay_samples),
            room_lp: 0.0,
            low_shelf_lp: 0.0,
            combs,
            allpasses,
            out_delay: DelayLine::new(out_delay_cap),
        }
    }

    fn reset(&mut self) {
        self.predelay.reset();
        self.room_lp = 0.0;
        self.low_shelf_lp = 0.0;
        for c in &mut self.combs {
            c.reset();
        }
        for a in &mut self.allpasses {
            a.reset();
        }
        self.out_delay.reset();
    }
}

pub(super) struct ReverbConfig {
    wet: f32,
    dry: f32,
    late_delay_samples: usize,
    early_tap_offsets: [usize; 4],
    early_gain: f32,
    late_gain: f32,
    room_main: f32,
    room_hf_gain: f32,
    room_lp_alpha: f32,
    low_eq_alpha: f32,
    low_eq_scale: f32,
    decay_time: f32,
    damp1: f32,
    allpass_feedback: f32,
    size_scale: f32,
    rear_delay_samples: usize,
    side_delay_samples: usize,
    disable_late_field: bool,
}

pub(super) struct SchroederReverbDsp {
    sample_rate: u32,
    channels: Vec<ReverbChannel>,
    cfg: ReverbConfig,
    tail_frames_remaining: usize,
}

impl SchroederReverbDsp {
    pub(super) fn new(cfg: ReverbConfig) -> Self {
        Self {
            sample_rate: 48_000,
            channels: Vec::new(),
            cfg,
            tail_frames_remaining: 0,
        }
    }

    fn configure(&mut self, sample_rate: u32, out_channels: usize) {
        self.sample_rate = sample_rate.max(XAUDIO2_MIN_SAMPLE_RATE);
        self.channels = (0..out_channels.max(1))
            .map(|c| ReverbChannel::new(self.sample_rate, c))
            .collect();
        self.tail_frames_remaining = 0;
        self.apply_comb_lengths();
    }

    fn reset(&mut self) {
        for ch in &mut self.channels {
            ch.reset();
        }
        self.tail_frames_remaining = 0;
    }

    fn set_config(&mut self, cfg: ReverbConfig) {
        self.cfg = cfg;
        self.apply_comb_lengths();
    }

    fn apply_comb_lengths(&mut self) {
        let sr_scale = (self.sample_rate as f32) / 44_100.0;
        let size_scale = self.cfg.size_scale.clamp(0.25, 1.0);
        for (ch_idx, ch) in self.channels.iter_mut().enumerate() {
            let spread = (ch_idx % 8) * 23;
            for (c, &base) in ch.combs.iter_mut().zip(FREEVERB_COMB_TUNINGS.iter()) {
                let target = (((base + spread) as f32) * sr_scale * size_scale).round() as usize;
                c.set_active_len(target.max(12));
            }
        }
    }

    /// Process reverb from `src` (or in-place from `dst` when `src` is `None` and `in_ch == out_ch`)
    /// into `dst` (`frames * out_ch`). Zero-allocation on the steady-state audio thread.
    fn process(
        &mut self,
        src: Option<&[f32]>,
        in_ch: usize,
        dst: &mut [f32],
        out_ch: usize,
        frames: usize,
        in_silent: bool,
    ) -> bool {
        if frames == 0 || out_ch == 0 {
            return false;
        }
        if self.channels.len() != out_ch {
            self.configure(self.sample_rate, out_ch);
        }
        if in_silent && self.tail_frames_remaining == 0 {
            dst[..frames * out_ch].fill(0.0);
            return false;
        }

        let sr = self.sample_rate as f32;
        let wet = self.cfg.wet;
        let dry = self.cfg.dry;
        let damp1 = self.cfg.damp1.clamp(0.0, 0.85);
        let damp2 = 1.0 - damp1;
        let ap_fb = self.cfg.allpass_feedback.clamp(0.0, 0.75);
        let decay = self.cfg.decay_time.max(0.05);
        let early_gain = self.cfg.early_gain;
        let late_gain = self.cfg.late_gain;
        let room_main = self.cfg.room_main;
        let room_hf_gain = self.cfg.room_hf_gain;
        let room_lp_alpha = self.cfg.room_lp_alpha;
        let low_eq_alpha = self.cfg.low_eq_alpha;
        let low_eq_scale = self.cfg.low_eq_scale;
        let disable_late = self.cfg.disable_late_field;
        let early_offsets = self.cfg.early_tap_offsets;
        let late_delay = self.cfg.late_delay_samples;
        let rear_delay = self.cfg.rear_delay_samples;
        let side_delay = self.cfg.side_delay_samples;

        let mut input_energy = 0.0f32;
        let mut wet_peak = 0.0f32;

        for f in 0..frames {
            for c in 0..out_ch {
                let x = if in_silent || in_ch == 0 {
                    0.0
                } else {
                    let s = match src {
                        Some(s_buf) => s_buf[f * in_ch + (c % in_ch)],
                        None => dst[f * out_ch + c],
                    };
                    let s = if s.is_finite() { s } else { 0.0 };
                    input_energy = input_energy.max(s.abs());
                    s
                };

                let ch = &mut self.channels[c];
                // Input room tone filter (high-shelf / LPF).
                ch.room_lp = flush_denormal(ch.room_lp + room_lp_alpha * (x - ch.room_lp));
                let filtered = room_main * (room_hf_gain * x + (1.0 - room_hf_gain) * ch.room_lp);

                ch.predelay.write(filtered);

                // Early reflections multi-tap sum (`+ 1` accounts for post-write index advance).
                let mut early = 0.0f32;
                for i in 0..4 {
                    early += ch.predelay.read_int(early_offsets[i] + 1) * EARLY_TAP_GAINS[i];
                }
                early *= early_gain;

                // Late field: 8 parallel damped Schroeder/Freeverb combs + 4 series unitary allpasses.
                let late = if disable_late {
                    0.0
                } else {
                    let late_in = ch.predelay.read_int(late_delay + 1);
                    let mut comb_sum = 0.0f32;
                    for comb in &mut ch.combs {
                        let fb = 10.0_f32
                            .powf(-3.0 * (comb.active_len as f32) / (decay * sr))
                            .clamp(0.0, 0.995);
                        comb_sum += comb.process(late_in, fb, damp1, damp2);
                    }
                    let mut diffused = comb_sum * 0.15;
                    for ap in &mut ch.allpasses {
                        diffused = ap.process(diffused, ap_fb);
                    }
                    // Low-frequency EQ shelf on late tail.
                    ch.low_shelf_lp =
                        flush_denormal(ch.low_shelf_lp + low_eq_alpha * (diffused - ch.low_shelf_lp));
                    let shaped = diffused + (low_eq_scale - 1.0) * ch.low_shelf_lp;
                    shaped * late_gain
                };

                let mut wet_sample = early + late;
                // Surround rear (channels 4, 5) and side (channels 6, 7) output delay.
                let extra_delay = match c {
                    4 | 5 => rear_delay,
                    6 | 7 => side_delay,
                    _ => 0,
                };
                if extra_delay > 0 {
                    ch.out_delay.write(wet_sample);
                    wet_sample = ch.out_delay.read_int(extra_delay + 1);
                }

                wet_peak = wet_peak.max(wet_sample.abs());
                let y = dry * x + wet * wet_sample;
                dst[f * out_ch + c] = if y.is_finite() { y } else { 0.0 };
            }
        }

        if input_energy > 1e-7 {
            let max_early = early_offsets[3];
            let pre_total = late_delay.max(max_early) + rear_delay.max(side_delay);
            let decay_frames = if disable_late {
                (sr * 0.05) as usize
            } else {
                (decay * 1.5 * sr) as usize + 4096
            };
            self.tail_frames_remaining = pre_total + decay_frames;
            true
        } else if wet_peak > 1e-7 {
            self.tail_frames_remaining = self.tail_frames_remaining.saturating_sub(frames);
            true
        } else {
            self.tail_frames_remaining = self.tail_frames_remaining.saturating_sub(frames);
            self.tail_frames_remaining > 0
        }
    }
}

pub(super) fn reverb_config_from_xaudio2(
    p: &XAudio2FxReverbParameters,
    sample_rate: u32,
) -> ReverbConfig {
    let sr = sample_rate.max(XAUDIO2_MIN_SAMPLE_RATE) as f32;
    let wet = (p.wet_dry_mix * 0.01).clamp(0.0, 1.0);
    let dry = 1.0 - wet;
    let room_norm = (p.room_size * 0.01).clamp(0.0, 1.0);
    let density_norm = (p.density * 0.01).clamp(0.0, 1.0);
    let size_scale = (0.45 + 0.35 * room_norm + 0.20 * density_norm).clamp(0.35, 1.0);

    let refl_delay = ((p.reflections_delay as f32) * 0.001 * sr).round() as usize;
    let rev_delay = ((p.reverb_delay as f32) * 0.001 * sr).round() as usize;
    let rear_delay = ((p.rear_delay as f32) * 0.001 * sr).round() as usize;
    let side_delay = ((p.side_delay as f32) * 0.001 * sr).round() as usize;

    let mut early_tap_offsets = [0usize; 4];
    let early_scale = 0.4 + 0.6 * room_norm;
    for i in 0..4 {
        let ms = EARLY_TAP_MS[i] * early_scale;
        early_tap_offsets[i] = refl_delay + ((ms * 0.001 * sr).round() as usize);
    }
    let late_delay_samples = refl_delay + rev_delay;

    let room_freq = p.room_filter_freq.clamp(20.0, 0.45 * sr);
    let room_lp_alpha = (1.0 - (-TAU * room_freq / sr).exp()).clamp(0.001, 1.0);
    let room_main = db_to_lin(p.room_filter_main);
    let room_hf_gain = db_to_lin(p.room_filter_hf);

    // HighEQGain is 0..=8 where 8 = 0 dB (no extra HF damping), 0 = -8 dB (strong damping).
    let hf_eq_cut = (8u8.saturating_sub(p.high_eq_gain) as f32) / 8.0;
    let hf_room_cut = (1.0 - room_hf_gain).clamp(0.0, 1.0);
    let damp1 = (0.12 + 0.55 * hf_eq_cut + 0.25 * hf_room_cut).clamp(0.05, 0.85);

    // LowEQGain is 0..=12 where 8 = 0 dB (-8..+4 dB), LowEQCutoff is 0..=9 (50..500 Hz).
    let low_db = (p.low_eq_gain as f32) - 8.0;
    let low_eq_scale = db_to_lin(low_db).clamp(0.25, 2.0);
    let low_freq = (50.0 + 50.0 * (p.low_eq_cutoff as f32)).clamp(50.0, 500.0);
    let low_eq_alpha = (1.0 - (-TAU * low_freq / sr).exp()).clamp(0.001, 1.0);

    let early_diff_scale = 0.6 + 0.4 * ((p.early_diffusion as f32) / 15.0);
    let allpass_feedback = 0.20 + 0.45 * ((p.late_diffusion as f32) / 15.0);

    ReverbConfig {
        wet,
        dry,
        late_delay_samples,
        early_tap_offsets,
        early_gain: db_to_lin(p.reflections_gain) * early_diff_scale,
        late_gain: db_to_lin(p.reverb_gain),
        room_main,
        room_hf_gain,
        room_lp_alpha,
        low_eq_alpha,
        low_eq_scale,
        decay_time: p.decay_time.max(0.1),
        damp1,
        allpass_feedback,
        size_scale,
        rear_delay_samples: rear_delay,
        side_delay_samples: side_delay,
        disable_late_field: p.disable_late_field != 0,
    }
}

pub(super) fn reverb_config_from_fxreverb(
    p: &FxReverbParameters,
    sample_rate: u32,
) -> ReverbConfig {
    let sr = sample_rate.max(XAUDIO2_MIN_SAMPLE_RATE) as f32;
    let diff = p.diffusion.clamp(FXREVERB_MIN_DIFFUSION, FXREVERB_MAX_DIFFUSION);
    let room = p.room_size.clamp(FXREVERB_MIN_ROOMSIZE, FXREVERB_MAX_ROOMSIZE);
    let size_scale = (0.35 + 0.65 * room).clamp(0.35, 1.0);
    let decay_time = 0.25 + 2.5 * room;
    let refl_delay = (0.004 * sr).round() as usize;
    let mut early_tap_offsets = [0usize; 4];
    for i in 0..4 {
        let ms = EARLY_TAP_MS[i] * (0.35 + 0.65 * room);
        early_tap_offsets[i] = refl_delay + ((ms * 0.001 * sr).round() as usize);
    }
    let late_delay_samples = refl_delay + ((0.008 * sr).round() as usize);
    let room_lp_alpha = (1.0 - (-TAU * 6000.0 / sr).exp()).clamp(0.001, 1.0);
    let low_eq_alpha = (1.0 - (-TAU * 250.0 / sr).exp()).clamp(0.001, 1.0);

    ReverbConfig {
        wet: 0.55,
        dry: 0.65,
        late_delay_samples,
        early_tap_offsets,
        early_gain: 0.55,
        late_gain: 0.95,
        room_main: 1.0,
        room_hf_gain: 0.85,
        room_lp_alpha,
        low_eq_alpha,
        low_eq_scale: 1.0,
        decay_time,
        damp1: 0.25,
        allpass_feedback: 0.15 + 0.55 * diff,
        size_scale,
        rear_delay_samples: 0,
        side_delay_samples: 0,
        disable_late_field: false,
    }
}

fn validate_xaudio2_reverb_params(p: &XAudio2FxReverbParameters) -> bool {
    let wet = p.wet_dry_mix;
    let refl_d = p.reflections_delay;
    let rev_d = p.reverb_delay;
    let rear_d = p.rear_delay;
    let side_d = p.side_delay;
    let rff = p.room_filter_freq;
    let rfm = p.room_filter_main;
    let rfh = p.room_filter_hf;
    let rg = p.reflections_gain;
    let rvg = p.reverb_gain;
    let dt = p.decay_time;
    let dens = p.density;
    let rsz = p.room_size;
    wet.is_finite()
        && (0.0..=100.0).contains(&wet)
        && refl_d <= 300
        && rev_d <= 85
        && rear_d <= 5
        && side_d <= 5
        && p.position_left <= 30
        && p.position_right <= 30
        && p.position_matrix_left <= 30
        && p.position_matrix_right <= 30
        && p.early_diffusion <= 15
        && p.late_diffusion <= 15
        && p.low_eq_gain <= 12
        && p.low_eq_cutoff <= 9
        && p.high_eq_gain <= 8
        && p.high_eq_cutoff <= 14
        && rff.is_finite()
        && (20.0..=20_000.0).contains(&rff)
        && rfm.is_finite()
        && (-100.0..=0.0).contains(&rfm)
        && rfh.is_finite()
        && (-100.0..=0.0).contains(&rfh)
        && rg.is_finite()
        && (-100.0..=20.0).contains(&rg)
        && rvg.is_finite()
        && (-100.0..=20.0).contains(&rvg)
        && dt.is_finite()
        && dt >= 0.1
        && dens.is_finite()
        && (0.0..=100.0).contains(&dens)
        && rsz.is_finite()
        && (0.0..=100.0).contains(&rsz)
}

// ============================================================================
// Volume Meter DSP
// ============================================================================

pub(super) struct VolumeMeterDsp {
    channels: usize,
    peak: Vec<f32>,
    rms: Vec<f32>,
}

impl VolumeMeterDsp {
    pub(super) fn new() -> Self {
        Self {
            channels: 0,
            peak: Vec::new(),
            rms: Vec::new(),
        }
    }

    fn configure(&mut self, channels: usize) {
        self.channels = channels;
        self.peak = vec![0.0; channels];
        self.rms = vec![0.0; channels];
    }

    fn reset(&mut self) {
        self.peak.fill(0.0);
        self.rms.fill(0.0);
    }

    fn process(&mut self, src: &[f32], ch: usize, frames: usize, in_silent: bool) {
        if ch == 0 {
            return;
        }
        if self.channels != ch {
            self.configure(ch);
        }
        if in_silent || frames == 0 {
            self.peak.fill(0.0);
            self.rms.fill(0.0);
            return;
        }
        self.peak.fill(0.0);
        self.rms.fill(0.0);
        for f in 0..frames {
            let base = f * ch;
            for c in 0..ch {
                let s = src[base + c];
                let a = if s.is_finite() { s.abs() } else { 0.0 };
                if a > self.peak[c] {
                    self.peak[c] = a;
                }
                self.rms[c] += a * a;
            }
        }
        let inv_n = 1.0 / (frames as f32);
        for c in 0..ch {
            self.rms[c] = (self.rms[c] * inv_n).sqrt();
        }
    }
}

// ============================================================================
// FXEQ 4-Band Parametric Biquad EQ
// ============================================================================

#[derive(Clone, Copy)]
struct BiquadCoeffs {
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,
    bypass: bool,
}

impl Default for BiquadCoeffs {
    fn default() -> Self {
        Self {
            b0: 1.0,
            b1: 0.0,
            b2: 0.0,
            a1: 0.0,
            a2: 0.0,
            bypass: true,
        }
    }
}

#[derive(Clone, Copy, Default)]
struct BiquadState {
    z1: f32,
    z2: f32,
}

fn design_peaking_eq(freq_hz: f32, linear_gain: f32, bandwidth_oct: f32, sr: f32) -> BiquadCoeffs {
    if (linear_gain - 1.0).abs() < 1e-5 {
        return BiquadCoeffs::default();
    }
    let fc = freq_hz.clamp(20.0, (sr * 0.48).max(20.0));
    let bw = bandwidth_oct.clamp(FXEQ_MIN_BANDWIDTH, FXEQ_MAX_BANDWIDTH);
    let a = linear_gain.clamp(FXEQ_MIN_GAIN, FXEQ_MAX_GAIN).sqrt();
    let w0 = TAU * fc / sr;
    let sin_w0 = w0.sin();
    let cos_w0 = w0.cos();
    let alpha = sin_w0 * ((std::f32::consts::LN_2 * 0.5) * bw * (w0 / sin_w0.max(1e-6))).sinh();
    let a0 = 1.0 + alpha / a;
    if !a0.is_finite() || a0.abs() < 1e-12 {
        return BiquadCoeffs::default();
    }
    let inv_a0 = 1.0 / a0;
    BiquadCoeffs {
        b0: (1.0 + alpha * a) * inv_a0,
        b1: (-2.0 * cos_w0) * inv_a0,
        b2: (1.0 - alpha * a) * inv_a0,
        a1: (-2.0 * cos_w0) * inv_a0,
        a2: (1.0 - alpha / a) * inv_a0,
        bypass: false,
    }
}

pub(super) struct FxEqDsp {
    params: FxEqParameters,
    sample_rate: u32,
    coeffs: [BiquadCoeffs; 4],
    states: Vec<[BiquadState; 4]>,
}

impl FxEqDsp {
    pub(super) fn new() -> Self {
        let mut s = Self {
            params: FxEqParameters::default(),
            sample_rate: 48_000,
            coeffs: [BiquadCoeffs::default(); 4],
            states: Vec::new(),
        };
        s.update_coeffs();
        s
    }

    fn configure(&mut self, sample_rate: u32, channels: usize) {
        self.sample_rate = sample_rate.max(XAUDIO2_MIN_SAMPLE_RATE);
        self.states = vec![[BiquadState::default(); 4]; channels.max(1)];
        self.update_coeffs();
    }

    fn reset(&mut self) {
        for ch in &mut self.states {
            *ch = [BiquadState::default(); 4];
        }
    }

    fn validate(p: &FxEqParameters) -> bool {
        let bands = [
            (p.frequency_center0, p.gain0, p.bandwidth0),
            (p.frequency_center1, p.gain1, p.bandwidth1),
            (p.frequency_center2, p.gain2, p.bandwidth2),
            (p.frequency_center3, p.gain3, p.bandwidth3),
        ];
        bands.iter().all(|&(f, g, b)| {
            f.is_finite()
                && (FXEQ_MIN_FREQUENCY_CENTER..=FXEQ_MAX_FREQUENCY_CENTER).contains(&f)
                && g.is_finite()
                && (FXEQ_MIN_GAIN..=FXEQ_MAX_GAIN).contains(&g)
                && b.is_finite()
                && (FXEQ_MIN_BANDWIDTH..=FXEQ_MAX_BANDWIDTH).contains(&b)
        })
    }

    fn update_coeffs(&mut self) {
        let sr = self.sample_rate as f32;
        let p = self.params;
        self.coeffs[0] = design_peaking_eq(p.frequency_center0, p.gain0, p.bandwidth0, sr);
        self.coeffs[1] = design_peaking_eq(p.frequency_center1, p.gain1, p.bandwidth1, sr);
        self.coeffs[2] = design_peaking_eq(p.frequency_center2, p.gain2, p.bandwidth2, sr);
        self.coeffs[3] = design_peaking_eq(p.frequency_center3, p.gain3, p.bandwidth3, sr);
    }

    fn has_active_state(&self) -> bool {
        self.states
            .iter()
            .any(|ch| ch.iter().any(|s| s.z1.abs() > 1e-7 || s.z2.abs() > 1e-7))
    }

    fn process(&mut self, buf: &mut [f32], ch: usize, frames: usize, in_silent: bool) -> bool {
        if frames == 0 || ch == 0 {
            return false;
        }
        if self.states.len() != ch {
            self.states.resize(ch, [BiquadState::default(); 4]);
        }
        if in_silent && !self.has_active_state() {
            buf[..frames * ch].fill(0.0);
            return false;
        }
        if self.coeffs.iter().all(|c| c.bypass) {
            return !in_silent;
        }
        let mut out_peak = 0.0f32;
        for f in 0..frames {
            let base = f * ch;
            for c in 0..ch {
                let mut x = if in_silent { 0.0 } else { buf[base + c] };
                if !x.is_finite() {
                    x = 0.0;
                }
                let st_arr = &mut self.states[c];
                for b in 0..4 {
                    let co = self.coeffs[b];
                    if co.bypass {
                        continue;
                    }
                    let st = &mut st_arr[b];
                    let y = co.b0 * x + st.z1;
                    st.z1 = flush_denormal(co.b1 * x - co.a1 * y + st.z2);
                    st.z2 = flush_denormal(co.b2 * x - co.a2 * y);
                    x = y;
                }
                out_peak = out_peak.max(x.abs());
                buf[base + c] = x;
            }
        }
        !in_silent || out_peak > 1e-7
    }
}

// ============================================================================
// FXMasteringLimiter Linked Multichannel Peak/Envelope Limiter
// ============================================================================

pub(super) struct FxMasteringLimiterDsp {
    params: FxMasteringLimiterParameters,
    sample_rate: u32,
    envelope: f32,
}

impl FxMasteringLimiterDsp {
    pub(super) fn new() -> Self {
        Self {
            params: FxMasteringLimiterParameters::default(),
            sample_rate: 48_000,
            envelope: 0.0,
        }
    }

    fn configure(&mut self, sample_rate: u32) {
        self.sample_rate = sample_rate.max(XAUDIO2_MIN_SAMPLE_RATE);
    }

    fn reset(&mut self) {
        self.envelope = 0.0;
    }

    fn validate(p: &FxMasteringLimiterParameters) -> bool {
        let rel = p.release;
        let loud = p.loudness;
        (FXMASTERINGLIMITER_MIN_RELEASE..=FXMASTERINGLIMITER_MAX_RELEASE).contains(&rel)
            && (FXMASTERINGLIMITER_MIN_LOUDNESS..=FXMASTERINGLIMITER_MAX_LOUDNESS).contains(&loud)
    }

    fn process(&mut self, buf: &mut [f32], ch: usize, frames: usize, in_silent: bool) -> bool {
        if frames == 0 || ch == 0 {
            return false;
        }
        let rel_ms = self.params.release.clamp(
            FXMASTERINGLIMITER_MIN_RELEASE,
            FXMASTERINGLIMITER_MAX_RELEASE,
        ) as f32;
        let loud = self.params.loudness.clamp(
            FXMASTERINGLIMITER_MIN_LOUDNESS,
            FXMASTERINGLIMITER_MAX_LOUDNESS,
        ) as f32;
        let release_samples = (rel_ms * 0.001 * (self.sample_rate as f32)).max(1.0);
        let release_coeff = (-1.0 / release_samples).exp();

        if in_silent {
            // Advance envelope release during silence so subsequent bursts recover properly.
            self.envelope = flush_denormal(self.envelope * release_coeff.powi(frames as i32));
            buf[..frames * ch].fill(0.0);
            return false;
        }

        // Loudness <= 1000 lowers ceiling proportionally (1000 = 1.0, 500 = 0.5);
        // Loudness > 1000 applies makeup drive while maintaining the 1.0 brickwall ceiling.
        let (makeup, threshold) = if loud <= 1000.0 {
            (1.0, (loud * 0.001).max(0.001))
        } else {
            (loud * 0.001, 1.0)
        };

        for f in 0..frames {
            let base = f * ch;
            let mut frame_peak = 0.0f32;
            for c in 0..ch {
                let s = buf[base + c];
                let driven = if s.is_finite() { s * makeup } else { 0.0 };
                buf[base + c] = driven;
                frame_peak = frame_peak.max(driven.abs());
            }
            if frame_peak > self.envelope {
                self.envelope = frame_peak;
            } else {
                self.envelope =
                    flush_denormal(frame_peak + release_coeff * (self.envelope - frame_peak));
            }
            let gain = if self.envelope > threshold {
                threshold / self.envelope
            } else {
                1.0
            };
            for c in 0..ch {
                buf[base + c] *= gain;
            }
        }
        true
    }
}

// ============================================================================
// FXEcho Multichannel Delay + Feedback
// ============================================================================

pub(super) struct FxEchoDsp {
    max_delay_ms: f32,
    params: FxEchoParameters,
    sample_rate: u32,
    lines: Vec<DelayLine>,
    tail_frames_remaining: usize,
}

impl FxEchoDsp {
    pub(super) fn new() -> Self {
        Self {
            max_delay_ms: FXECHO_MAX_DELAY,
            params: FxEchoParameters::default(),
            sample_rate: 48_000,
            lines: Vec::new(),
            tail_frames_remaining: 0,
        }
    }

    fn set_max_delay(&mut self, max_delay_ms: f32) -> Result<(), i32> {
        if !max_delay_ms.is_finite()
            || !(FXECHO_MIN_DELAY..=FXECHO_MAX_DELAY).contains(&max_delay_ms)
        {
            return Err(E_INVALIDARG);
        }
        self.max_delay_ms = max_delay_ms;
        if self.params.delay > max_delay_ms {
            self.params.delay = max_delay_ms;
        }
        if !self.lines.is_empty() {
            let ch = self.lines.len();
            self.configure(self.sample_rate, ch);
        }
        Ok(())
    }

    fn configure(&mut self, sample_rate: u32, channels: usize) {
        self.sample_rate = sample_rate.max(XAUDIO2_MIN_SAMPLE_RATE);
        let max_samples =
            ((self.max_delay_ms * 0.001 * (self.sample_rate as f32)).ceil() as usize) + 4;
        self.lines = (0..channels.max(1))
            .map(|_| DelayLine::new(max_samples))
            .collect();
        self.tail_frames_remaining = 0;
    }

    fn reset(&mut self) {
        for line in &mut self.lines {
            line.reset();
        }
        self.tail_frames_remaining = 0;
    }

    fn validate(&self, p: &FxEchoParameters) -> bool {
        let wet = p.wet_dry_mix;
        let fb = p.feedback;
        let del = p.delay;
        wet.is_finite()
            && (FXECHO_MIN_WETDRYMIX..=FXECHO_MAX_WETDRYMIX).contains(&wet)
            && fb.is_finite()
            && (FXECHO_MIN_FEEDBACK..=FXECHO_MAX_FEEDBACK).contains(&fb)
            && del.is_finite()
            && (FXECHO_MIN_DELAY..=self.max_delay_ms).contains(&del)
    }

    fn process(&mut self, buf: &mut [f32], ch: usize, frames: usize, in_silent: bool) -> bool {
        if frames == 0 || ch == 0 {
            return false;
        }
        if self.lines.len() != ch {
            self.configure(self.sample_rate, ch);
        }
        if in_silent && self.tail_frames_remaining == 0 {
            buf[..frames * ch].fill(0.0);
            return false;
        }

        let wet = self
            .params
            .wet_dry_mix
            .clamp(FXECHO_MIN_WETDRYMIX, FXECHO_MAX_WETDRYMIX);
        let dry = 1.0 - wet;
        let fb = self
            .params
            .feedback
            .clamp(FXECHO_MIN_FEEDBACK, FXECHO_MAX_FEEDBACK);
        let delay_ms = self.params.delay.clamp(FXECHO_MIN_DELAY, self.max_delay_ms);
        let delay_samples = (delay_ms * 0.001 * (self.sample_rate as f32)).max(1.0);

        let mut input_peak = 0.0f32;
        let mut out_peak = 0.0f32;

        for f in 0..frames {
            let base = f * ch;
            for c in 0..ch {
                let x = if in_silent {
                    0.0
                } else {
                    let v = buf[base + c];
                    let v = if v.is_finite() { v } else { 0.0 };
                    input_peak = input_peak.max(v.abs());
                    v
                };
                let line = &mut self.lines[c];
                let delayed = line.read_frac(delay_samples);
                let out = dry * x + wet * delayed;
                let store = (x + delayed * fb).clamp(-16.0, 16.0);
                line.write(store);
                out_peak = out_peak.max(out.abs());
                buf[base + c] = out;
            }
        }

        let d_int = delay_samples.ceil() as usize;
        if input_peak > 1e-7 {
            let repeats = if fb <= 0.001 {
                1usize
            } else if fb >= 0.999 {
                32usize
            } else {
                ((-6.5 / fb.log10()).ceil() as usize).clamp(1, 32)
            };
            self.tail_frames_remaining = d_int.saturating_mul(repeats).saturating_add(64);
            true
        } else if out_peak > 1e-7 {
            self.tail_frames_remaining = self.tail_frames_remaining.saturating_sub(frames);
            true
        } else {
            self.tail_frames_remaining = self.tail_frames_remaining.saturating_sub(frames);
            self.tail_frames_remaining > 0
        }
    }
}

// ============================================================================
// Builtin XAPO State & Dual-Interface COM Object
// ============================================================================

pub(super) enum BuiltinXapoKind {
    AudioReverb {
        params: XAudio2FxReverbParameters,
        dsp: SchroederReverbDsp,
    },
    VolumeMeter(VolumeMeterDsp),
    FxEq(FxEqDsp),
    FxMasteringLimiter(FxMasteringLimiterDsp),
    FxReverb {
        params: FxReverbParameters,
        dsp: SchroederReverbDsp,
    },
    FxEcho(FxEchoDsp),
}

pub(super) struct BuiltinXapoState {
    kind: BuiltinXapoKind,
    locked: bool,
    sample_rate: u32,
    in_channels: usize,
    out_channels: usize,
}

impl BuiltinXapoState {
    fn new(kind: BuiltinXapoKind) -> Self {
        Self {
            kind,
            locked: false,
            sample_rate: 48_000,
            in_channels: 2,
            out_channels: 2,
        }
    }

    fn clsid(&self) -> GUID {
        match self.kind {
            BuiltinXapoKind::AudioReverb { .. } => CLSID_AUDIO_REVERB,
            BuiltinXapoKind::VolumeMeter(_) => CLSID_AUDIO_VOLUME_METER,
            BuiltinXapoKind::FxEq(_) => CLSID_FX_EQ,
            BuiltinXapoKind::FxMasteringLimiter(_) => CLSID_FX_MASTERING_LIMITER,
            BuiltinXapoKind::FxReverb { .. } => CLSID_FX_REVERB,
            BuiltinXapoKind::FxEcho(_) => CLSID_FX_ECHO,
        }
    }

    fn friendly_name(&self) -> &'static str {
        match self.kind {
            BuiltinXapoKind::AudioReverb { .. } => "XAudio2 Reverb",
            BuiltinXapoKind::VolumeMeter(_) => "XAudio2 Volume Meter",
            BuiltinXapoKind::FxEq(_) => "FXEQ",
            BuiltinXapoKind::FxMasteringLimiter(_) => "FXMasteringLimiter",
            BuiltinXapoKind::FxReverb { .. } => "FXReverb",
            BuiltinXapoKind::FxEcho(_) => "FXEcho",
        }
    }

    fn reg_flags(&self) -> u32 {
        match self.kind {
            BuiltinXapoKind::AudioReverb { .. } | BuiltinXapoKind::FxReverb { .. } => {
                XAPO_FLAG_FRAMERATE_MUST_MATCH
                    | XAPO_FLAG_BITSPERSAMPLE_MUST_MATCH
                    | XAPO_FLAG_BUFFERCOUNT_MUST_MATCH
                    | XAPO_FLAG_INPLACE_SUPPORTED
            }
            BuiltinXapoKind::VolumeMeter(_)
            | BuiltinXapoKind::FxEq(_)
            | BuiltinXapoKind::FxMasteringLimiter(_)
            | BuiltinXapoKind::FxEcho(_) => XAPOBASE_DEFAULT_FLAG,
        }
    }

    fn channels_must_match(&self) -> bool {
        (self.reg_flags() & XAPO_FLAG_CHANNELS_MUST_MATCH) != 0
    }

    fn registration_properties(&self) -> XapoRegistrationProperties {
        let mut friendly_name = [0u16; XAPO_REGISTRATION_STRING_LENGTH];
        let mut copyright_info = [0u16; XAPO_REGISTRATION_STRING_LENGTH];
        write_wide_fixed(&mut friendly_name, self.friendly_name());
        write_wide_fixed(&mut copyright_info, "Builtin XAudio2 XAPO");
        XapoRegistrationProperties {
            clsid: self.clsid(),
            friendly_name,
            copyright_info,
            major_version: 1,
            minor_version: 0,
            flags: self.reg_flags(),
            min_input_buffer_count: 1,
            max_input_buffer_count: 1,
            min_output_buffer_count: 1,
            max_output_buffer_count: 1,
        }
    }

    unsafe fn initialize(&mut self, p_data: *const c_void, byte_size: u32) -> i32 {
        match &mut self.kind {
            BuiltinXapoKind::FxEcho(echo) => {
                if p_data.is_null() && byte_size == 0 {
                    return S_OK;
                }
                if p_data.is_null() || (byte_size as usize) < std::mem::size_of::<FxEchoInitData>()
                {
                    return E_INVALIDARG;
                }
                let init = rd(p_data as *const FxEchoInitData);
                match echo.set_max_delay(init.max_delay) {
                    Ok(()) => S_OK,
                    Err(hr) => hr,
                }
            }
            _ => {
                if !p_data.is_null() && byte_size > 0 {
                    // Builtin effects without init data accept optional init cleanly.
                    S_OK
                } else {
                    S_OK
                }
            }
        }
    }

    fn reset(&mut self) {
        match &mut self.kind {
            BuiltinXapoKind::AudioReverb { dsp, .. } => dsp.reset(),
            BuiltinXapoKind::VolumeMeter(vm) => vm.reset(),
            BuiltinXapoKind::FxEq(eq) => eq.reset(),
            BuiltinXapoKind::FxMasteringLimiter(lim) => lim.reset(),
            BuiltinXapoKind::FxReverb { dsp, .. } => dsp.reset(),
            BuiltinXapoKind::FxEcho(echo) => echo.reset(),
        }
    }

    fn lock_for_process(&mut self, sample_rate: u32, in_ch: usize, out_ch: usize) -> i32 {
        if in_ch == 0
            || out_ch == 0
            || in_ch > XAUDIO2_MAX_AUDIO_CHANNELS as usize
            || out_ch > XAUDIO2_MAX_AUDIO_CHANNELS as usize
            || !(XAUDIO2_MIN_SAMPLE_RATE..=XAUDIO2_MAX_SAMPLE_RATE).contains(&sample_rate)
        {
            return XAPO_E_FORMAT_UNSUPPORTED;
        }
        if self.channels_must_match() && in_ch != out_ch {
            return XAPO_E_FORMAT_UNSUPPORTED;
        }
        self.sample_rate = sample_rate;
        self.in_channels = in_ch;
        self.out_channels = out_ch;
        self.locked = true;
        match &mut self.kind {
            BuiltinXapoKind::AudioReverb { params, dsp } => {
                dsp.set_config(reverb_config_from_xaudio2(params, sample_rate));
                dsp.configure(sample_rate, out_ch);
            }
            BuiltinXapoKind::VolumeMeter(vm) => vm.configure(in_ch),
            BuiltinXapoKind::FxEq(eq) => eq.configure(sample_rate, in_ch),
            BuiltinXapoKind::FxMasteringLimiter(lim) => lim.configure(sample_rate),
            BuiltinXapoKind::FxReverb { params, dsp } => {
                dsp.set_config(reverb_config_from_fxreverb(params, sample_rate));
                dsp.configure(sample_rate, out_ch);
            }
            BuiltinXapoKind::FxEcho(echo) => echo.configure(sample_rate, in_ch),
        }
        S_OK
    }

    fn unlock_for_process(&mut self) {
        self.locked = false;
    }

    fn has_active_tail(&self) -> bool {
        match &self.kind {
            BuiltinXapoKind::AudioReverb { dsp, .. } => dsp.tail_frames_remaining > 0,
            BuiltinXapoKind::FxReverb { dsp, .. } => dsp.tail_frames_remaining > 0,
            BuiltinXapoKind::FxEcho(echo) => echo.tail_frames_remaining > 0,
            BuiltinXapoKind::FxEq(eq) => eq.has_active_state(),
            BuiltinXapoKind::VolumeMeter(_) | BuiltinXapoKind::FxMasteringLimiter(_) => false,
        }
    }

    /// Process audio in-place (`in_ch == out_ch`). Returns `true` if output is valid (non-silent).
    fn process_in_place(
        &mut self,
        buf: &mut [f32],
        ch: usize,
        frames: usize,
        in_silent: bool,
        is_enabled: bool,
    ) -> bool {
        if frames == 0 || ch == 0 {
            return false;
        }
        if !is_enabled {
            if in_silent {
                buf[..frames * ch].fill(0.0);
                return false;
            }
            return true;
        }
        match &mut self.kind {
            BuiltinXapoKind::AudioReverb { dsp, .. } | BuiltinXapoKind::FxReverb { dsp, .. } => {
                dsp.process(None, ch, buf, ch, frames, in_silent)
            }
            BuiltinXapoKind::VolumeMeter(vm) => {
                vm.process(buf, ch, frames, in_silent);
                if in_silent {
                    buf[..frames * ch].fill(0.0);
                    false
                } else {
                    true
                }
            }
            BuiltinXapoKind::FxEq(eq) => eq.process(buf, ch, frames, in_silent),
            BuiltinXapoKind::FxMasteringLimiter(lim) => lim.process(buf, ch, frames, in_silent),
            BuiltinXapoKind::FxEcho(echo) => echo.process(buf, ch, frames, in_silent),
        }
    }

    /// Process audio from `src` (`frames * in_ch`) into `dst` (`frames * out_ch`).
    /// Returns `true` if `dst` contains valid (non-silent) audio.
    fn process_out_of_place(
        &mut self,
        src: &[f32],
        in_ch: usize,
        dst: &mut [f32],
        out_ch: usize,
        frames: usize,
        in_silent: bool,
        is_enabled: bool,
    ) -> bool {
        if frames == 0 || out_ch == 0 {
            return false;
        }
        if !is_enabled {
            if in_silent {
                dst[..frames * out_ch].fill(0.0);
                return false;
            }
            adapt_channels(src, in_ch, dst, out_ch, frames);
            return true;
        }
        match &mut self.kind {
            BuiltinXapoKind::AudioReverb { dsp, .. } | BuiltinXapoKind::FxReverb { dsp, .. } => {
                dsp.process(Some(src), in_ch, dst, out_ch, frames, in_silent)
            }
            BuiltinXapoKind::VolumeMeter(vm) => {
                if in_silent {
                    vm.process(src, in_ch, frames, true);
                    dst[..frames * out_ch].fill(0.0);
                    false
                } else {
                    vm.process(src, in_ch, frames, false);
                    adapt_channels(src, in_ch, dst, out_ch, frames);
                    true
                }
            }
            BuiltinXapoKind::FxEq(eq) => {
                if !in_silent {
                    adapt_channels(src, in_ch, dst, out_ch, frames);
                }
                eq.process(dst, out_ch, frames, in_silent)
            }
            BuiltinXapoKind::FxMasteringLimiter(lim) => {
                if !in_silent {
                    adapt_channels(src, in_ch, dst, out_ch, frames);
                }
                lim.process(dst, out_ch, frames, in_silent)
            }
            BuiltinXapoKind::FxEcho(echo) => {
                if !in_silent {
                    adapt_channels(src, in_ch, dst, out_ch, frames);
                }
                echo.process(dst, out_ch, frames, in_silent)
            }
        }
    }

    pub(super) unsafe fn set_parameters(&mut self, ptr: *const c_void, byte_size: u32) -> i32 {
        match &mut self.kind {
            BuiltinXapoKind::AudioReverb { params, dsp } => {
                if ptr.is_null() {
                    return E_POINTER;
                }
                let sz = byte_size as usize;
                let parsed = if sz == std::mem::size_of::<XAudio2FxReverbParameters>() {
                    rd(ptr as *const XAudio2FxReverbParameters)
                } else if sz == std::mem::size_of::<XAudio2FxReverbParameters28>() {
                    let p28 = rd(ptr as *const XAudio2FxReverbParameters28);
                    XAudio2FxReverbParameters {
                        wet_dry_mix: p28.wet_dry_mix,
                        reflections_delay: p28.reflections_delay,
                        reverb_delay: p28.reverb_delay,
                        rear_delay: p28.rear_delay,
                        side_delay: 5,
                        position_left: p28.position_left,
                        position_right: p28.position_right,
                        position_matrix_left: p28.position_matrix_left,
                        position_matrix_right: p28.position_matrix_right,
                        early_diffusion: p28.early_diffusion,
                        late_diffusion: p28.late_diffusion,
                        low_eq_gain: p28.low_eq_gain,
                        low_eq_cutoff: p28.low_eq_cutoff,
                        high_eq_gain: p28.high_eq_gain,
                        high_eq_cutoff: p28.high_eq_cutoff,
                        room_filter_freq: p28.room_filter_freq,
                        room_filter_main: p28.room_filter_main,
                        room_filter_hf: p28.room_filter_hf,
                        reflections_gain: p28.reflections_gain,
                        reverb_gain: p28.reverb_gain,
                        decay_time: p28.decay_time,
                        density: p28.density,
                        room_size: p28.room_size,
                        disable_late_field: p28.disable_late_field,
                    }
                } else if sz == std::mem::size_of::<XAudio2FxReverbParameters27>() {
                    let p27 = rd(ptr as *const XAudio2FxReverbParameters27);
                    XAudio2FxReverbParameters {
                        wet_dry_mix: p27.wet_dry_mix,
                        reflections_delay: p27.reflections_delay,
                        reverb_delay: p27.reverb_delay,
                        rear_delay: p27.rear_delay,
                        side_delay: 5,
                        position_left: p27.position_left,
                        position_right: p27.position_right,
                        position_matrix_left: p27.position_matrix_left,
                        position_matrix_right: p27.position_matrix_right,
                        early_diffusion: p27.early_diffusion,
                        late_diffusion: p27.late_diffusion,
                        low_eq_gain: p27.low_eq_gain,
                        low_eq_cutoff: p27.low_eq_cutoff,
                        high_eq_gain: p27.high_eq_gain,
                        high_eq_cutoff: p27.high_eq_cutoff,
                        room_filter_freq: p27.room_filter_freq,
                        room_filter_main: p27.room_filter_main,
                        room_filter_hf: p27.room_filter_hf,
                        reflections_gain: p27.reflections_gain,
                        reverb_gain: p27.reverb_gain,
                        decay_time: p27.decay_time,
                        density: p27.density,
                        room_size: p27.room_size,
                        disable_late_field: 0,
                    }
                } else {
                    return E_INVALIDARG;
                };
                if !validate_xaudio2_reverb_params(&parsed) {
                    return E_INVALIDARG;
                }
                *params = parsed;
                dsp.set_config(reverb_config_from_xaudio2(params, self.sample_rate));
                S_OK
            }
            BuiltinXapoKind::VolumeMeter(_) => {
                // VolumeMeter has no writable parameters; SetParameters is a no-op.
                S_OK
            }
            BuiltinXapoKind::FxEq(eq) => {
                if ptr.is_null() {
                    return E_POINTER;
                }
                if (byte_size as usize) != std::mem::size_of::<FxEqParameters>() {
                    return E_INVALIDARG;
                }
                let p = rd(ptr as *const FxEqParameters);
                if !FxEqDsp::validate(&p) {
                    return E_INVALIDARG;
                }
                eq.params = p;
                eq.update_coeffs();
                S_OK
            }
            BuiltinXapoKind::FxMasteringLimiter(lim) => {
                if ptr.is_null() {
                    return E_POINTER;
                }
                if (byte_size as usize) != std::mem::size_of::<FxMasteringLimiterParameters>() {
                    return E_INVALIDARG;
                }
                let p = rd(ptr as *const FxMasteringLimiterParameters);
                if !FxMasteringLimiterDsp::validate(&p) {
                    return E_INVALIDARG;
                }
                lim.params = p;
                S_OK
            }
            BuiltinXapoKind::FxReverb { params, dsp } => {
                if ptr.is_null() {
                    return E_POINTER;
                }
                if (byte_size as usize) != std::mem::size_of::<FxReverbParameters>() {
                    return E_INVALIDARG;
                }
                let p = rd(ptr as *const FxReverbParameters);
                let diff = p.diffusion;
                let room = p.room_size;
                if !diff.is_finite()
                    || !(FXREVERB_MIN_DIFFUSION..=FXREVERB_MAX_DIFFUSION).contains(&diff)
                    || !room.is_finite()
                    || !(FXREVERB_MIN_ROOMSIZE..=FXREVERB_MAX_ROOMSIZE).contains(&room)
                {
                    return E_INVALIDARG;
                }
                *params = p;
                dsp.set_config(reverb_config_from_fxreverb(params, self.sample_rate));
                S_OK
            }
            BuiltinXapoKind::FxEcho(echo) => {
                if ptr.is_null() {
                    return E_POINTER;
                }
                if (byte_size as usize) != std::mem::size_of::<FxEchoParameters>() {
                    return E_INVALIDARG;
                }
                let p = rd(ptr as *const FxEchoParameters);
                if !echo.validate(&p) {
                    return E_INVALIDARG;
                }
                echo.params = p;
                S_OK
            }
        }
    }

    pub(super) unsafe fn get_parameters(&self, ptr: *mut c_void, byte_size: u32) -> i32 {
        if ptr.is_null() {
            return E_POINTER;
        }
        match &self.kind {
            BuiltinXapoKind::AudioReverb { params, .. } => {
                let sz = byte_size as usize;
                if sz == std::mem::size_of::<XAudio2FxReverbParameters>() {
                    wr(ptr as *mut XAudio2FxReverbParameters, *params);
                    S_OK
                } else if sz == std::mem::size_of::<XAudio2FxReverbParameters28>() {
                    let p28 = XAudio2FxReverbParameters28 {
                        wet_dry_mix: params.wet_dry_mix,
                        reflections_delay: params.reflections_delay,
                        reverb_delay: params.reverb_delay,
                        rear_delay: params.rear_delay,
                        position_left: params.position_left,
                        position_right: params.position_right,
                        position_matrix_left: params.position_matrix_left,
                        position_matrix_right: params.position_matrix_right,
                        early_diffusion: params.early_diffusion,
                        late_diffusion: params.late_diffusion,
                        low_eq_gain: params.low_eq_gain,
                        low_eq_cutoff: params.low_eq_cutoff,
                        high_eq_gain: params.high_eq_gain,
                        high_eq_cutoff: params.high_eq_cutoff,
                        room_filter_freq: params.room_filter_freq,
                        room_filter_main: params.room_filter_main,
                        room_filter_hf: params.room_filter_hf,
                        reflections_gain: params.reflections_gain,
                        reverb_gain: params.reverb_gain,
                        decay_time: params.decay_time,
                        density: params.density,
                        room_size: params.room_size,
                        disable_late_field: params.disable_late_field,
                    };
                    wr(ptr as *mut XAudio2FxReverbParameters28, p28);
                    S_OK
                } else if sz == std::mem::size_of::<XAudio2FxReverbParameters27>() {
                    let p27 = XAudio2FxReverbParameters27 {
                        wet_dry_mix: params.wet_dry_mix,
                        reflections_delay: params.reflections_delay,
                        reverb_delay: params.reverb_delay,
                        rear_delay: params.rear_delay,
                        position_left: params.position_left,
                        position_right: params.position_right,
                        position_matrix_left: params.position_matrix_left,
                        position_matrix_right: params.position_matrix_right,
                        early_diffusion: params.early_diffusion,
                        late_diffusion: params.late_diffusion,
                        low_eq_gain: params.low_eq_gain,
                        low_eq_cutoff: params.low_eq_cutoff,
                        high_eq_gain: params.high_eq_gain,
                        high_eq_cutoff: params.high_eq_cutoff,
                        room_filter_freq: params.room_filter_freq,
                        room_filter_main: params.room_filter_main,
                        room_filter_hf: params.room_filter_hf,
                        reflections_gain: params.reflections_gain,
                        reverb_gain: params.reverb_gain,
                        decay_time: params.decay_time,
                        density: params.density,
                        room_size: params.room_size,
                    };
                    wr(ptr as *mut XAudio2FxReverbParameters27, p27);
                    S_OK
                } else {
                    E_INVALIDARG
                }
            }
            BuiltinXapoKind::VolumeMeter(vm) => {
                if (byte_size as usize) < std::mem::size_of::<XAudio2FxVolumeMeterLevels>() {
                    return E_INVALIDARG;
                }
                let levels = rd(ptr as *const XAudio2FxVolumeMeterLevels);
                let count = levels.channel_count as usize;
                if count == 0 || count > XAUDIO2_MAX_AUDIO_CHANNELS as usize {
                    return E_INVALIDARG;
                }
                if self.locked && count != vm.channels {
                    return E_INVALIDARG;
                }
                if !levels.peak_levels.is_null() {
                    for i in 0..count {
                        let v = vm.peak.get(i).copied().unwrap_or(0.0);
                        wr(levels.peak_levels.add(i), v);
                    }
                }
                if !levels.rms_levels.is_null() {
                    for i in 0..count {
                        let v = vm.rms.get(i).copied().unwrap_or(0.0);
                        wr(levels.rms_levels.add(i), v);
                    }
                }
                S_OK
            }
            BuiltinXapoKind::FxEq(eq) => {
                if (byte_size as usize) != std::mem::size_of::<FxEqParameters>() {
                    return E_INVALIDARG;
                }
                wr(ptr as *mut FxEqParameters, eq.params);
                S_OK
            }
            BuiltinXapoKind::FxMasteringLimiter(lim) => {
                if (byte_size as usize) != std::mem::size_of::<FxMasteringLimiterParameters>() {
                    return E_INVALIDARG;
                }
                wr(ptr as *mut FxMasteringLimiterParameters, lim.params);
                S_OK
            }
            BuiltinXapoKind::FxReverb { params, .. } => {
                if (byte_size as usize) != std::mem::size_of::<FxReverbParameters>() {
                    return E_INVALIDARG;
                }
                wr(ptr as *mut FxReverbParameters, *params);
                S_OK
            }
            BuiltinXapoKind::FxEcho(echo) => {
                if (byte_size as usize) != std::mem::size_of::<FxEchoParameters>() {
                    return E_INVALIDARG;
                }
                wr(ptr as *mut FxEchoParameters, echo.params);
                S_OK
            }
        }
    }
}

/// Dual-interface COM object implementing both `IXAPO` (at offset 0) and
/// `IXAPOParameters` (at `offset_of!(BuiltinXapoCom, params_vtbl)`).
#[repr(C)]
pub(super) struct BuiltinXapoCom {
    #[allow(dead_code)]
    xapo_vtbl: *const [crate::Ptr; 13],
    #[allow(dead_code)]
    params_vtbl: *const [crate::Ptr; 5],
    magic: u64,
    refs: AtomicU32,
    inner: Arc<Mutex<BuiltinXapoState>>,
}

impl BuiltinXapoCom {
    pub(super) fn alloc(kind: BuiltinXapoKind) -> *mut Self {
        ensure_vtbls_registered();
        Box::into_raw(Box::new(Self {
            xapo_vtbl: &XAPO_VTBL.0,
            params_vtbl: &XAPO_PARAMS_VTBL.0,
            magic: BUILTIN_XAPO_MAGIC,
            refs: AtomicU32::new(1),
            inner: Arc::new(Mutex::new(BuiltinXapoState::new(kind))),
        }))
    }
}

#[inline]
unsafe fn xapo_params_interface_ptr(base: *mut BuiltinXapoCom) -> *mut c_void {
    (base as *mut u8).add(std::mem::offset_of!(BuiltinXapoCom, params_vtbl)) as *mut c_void
}

#[inline]
unsafe fn xapo_from_params_ptr(this_params: *mut c_void) -> *mut BuiltinXapoCom {
    if this_params.is_null() {
        return std::ptr::null_mut();
    }
    (this_params as *mut u8).sub(std::mem::offset_of!(BuiltinXapoCom, params_vtbl))
        as *mut BuiltinXapoCom
}

unsafe fn try_identify_builtin_xapo(p_unknown: *mut c_void) -> Option<*mut BuiltinXapoCom> {
    if p_unknown.is_null() {
        return None;
    }
    let vtbl = rd(p_unknown as *const usize);
    if vtbl == XAPO_VTBL.0.as_ptr() as usize {
        let base = p_unknown as *mut BuiltinXapoCom;
        if rd(std::ptr::addr_of!((*base).magic)) == BUILTIN_XAPO_MAGIC {
            return Some(base);
        }
    } else if vtbl == XAPO_PARAMS_VTBL.0.as_ptr() as usize {
        let base = xapo_from_params_ptr(p_unknown);
        if rd(std::ptr::addr_of!((*base).magic)) == BUILTIN_XAPO_MAGIC {
            return Some(base);
        }
    }
    None
}

unsafe fn xapo_qi_common(
    base: *mut BuiltinXapoCom,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> i32 {
    if ppv.is_null() {
        return E_POINTER;
    }
    write_null_out(ppv);
    if base.is_null() {
        return E_NOINTERFACE;
    }
    let Some(iid) = GUID::read(riid) else {
        return E_POINTER;
    };
    let obj = &*base;
    if iid == IID_IUnknown || iid == IID_IXAPO {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        wr(ppv, base as *mut c_void);
        return S_OK;
    }
    if iid == IID_IXAPO_PARAMETERS {
        obj.refs.fetch_add(1, Ordering::AcqRel);
        wr(ppv, xapo_params_interface_ptr(base));
        return S_OK;
    }
    E_NOINTERFACE
}

pub(super) unsafe extern "win64" fn xapo_query_interface(
    this: *mut BuiltinXapoCom,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> i32 {
    xapo_qi_common(this, riid, ppv)
}

unsafe extern "win64" fn xapo_add_ref(this: *mut BuiltinXapoCom) -> u32 {
    if this.is_null() {
        return 0;
    }
    let obj = &*this;
    obj.refs.fetch_add(1, Ordering::AcqRel) + 1
}

pub(super) unsafe extern "win64" fn xapo_release(this: *mut BuiltinXapoCom) -> u32 {
    if this.is_null() {
        return 0;
    }
    let prev = {
        let obj = &*this;
        obj.refs.fetch_sub(1, Ordering::AcqRel)
    };
    if prev == 1 {
        drop(Box::from_raw(this));
        0
    } else {
        prev - 1
    }
}

pub(super) unsafe extern "win64" fn xapo_get_registration_properties(
    this: *mut BuiltinXapoCom,
    pp_props: *mut *mut XapoRegistrationProperties,
) -> i32 {
    if this.is_null() || pp_props.is_null() {
        return E_POINTER;
    }
    write_null_out(pp_props);
    let obj = &*this;
    let props = obj.inner.lock().unwrap().registration_properties();
    let mem = crate::audio::com::co_alloc(std::mem::size_of::<XapoRegistrationProperties>())
        as *mut XapoRegistrationProperties;
    if mem.is_null() {
        return E_OUTOFMEMORY;
    }
    wr(mem, props);
    wr(pp_props, mem);
    S_OK
}

unsafe fn xapo_check_format_supported(
    this: *mut BuiltinXapoCom,
    other_format: *const WAVEFORMATEX,
    requested_format: *const WAVEFORMATEX,
    pp_supported: *mut *mut WAVEFORMATEX,
) -> i32 {
    write_null_out(pp_supported);
    if this.is_null() || requested_format.is_null() {
        return E_POINTER;
    }
    let obj = &*this;
    let must_match_ch = obj.inner.lock().unwrap().channels_must_match();
    let other_parsed = if other_format.is_null() {
        None
    } else {
        parse_xapo_float_format(other_format).ok()
    };
    let req_ok = match parse_xapo_float_format(requested_format) {
        Ok((req_ch, req_sr)) => match other_parsed {
            Some((other_ch, other_sr)) => {
                req_sr == other_sr && (!must_match_ch || req_ch == other_ch)
            }
            None => true,
        },
        Err(_) => false,
    };
    if req_ok {
        return S_OK;
    }
    if !pp_supported.is_null() {
        let req_raw = rd(requested_format);
        let fallback_ch = other_parsed.map(|(ch, _)| ch).unwrap_or_else(|| {
            (req_raw.nChannels as usize).clamp(1, XAUDIO2_MAX_AUDIO_CHANNELS as usize)
        });
        let fallback_sr = other_parsed.map(|(_, sr)| sr).unwrap_or_else(|| {
            req_raw
                .nSamplesPerSec
                .clamp(XAUDIO2_MIN_SAMPLE_RATE, XAUDIO2_MAX_SAMPLE_RATE)
        });
        let mem = crate::audio::com::co_alloc(std::mem::size_of::<WAVEFORMATEX>())
            as *mut WAVEFORMATEX;
        if !mem.is_null() {
            wr(mem, make_float_wfx(fallback_ch, fallback_sr));
            wr(pp_supported, mem);
        }
    }
    XAPO_E_FORMAT_UNSUPPORTED
}

pub(super) unsafe extern "win64" fn xapo_is_input_format_supported(
    this: *mut BuiltinXapoCom,
    p_output_format: *const WAVEFORMATEX,
    p_requested_input_format: *const WAVEFORMATEX,
    pp_supported_input_format: *mut *mut WAVEFORMATEX,
) -> i32 {
    xapo_check_format_supported(
        this,
        p_output_format,
        p_requested_input_format,
        pp_supported_input_format,
    )
}

pub(super) unsafe extern "win64" fn xapo_is_output_format_supported(
    this: *mut BuiltinXapoCom,
    p_input_format: *const WAVEFORMATEX,
    p_requested_output_format: *const WAVEFORMATEX,
    pp_supported_output_format: *mut *mut WAVEFORMATEX,
) -> i32 {
    xapo_check_format_supported(
        this,
        p_input_format,
        p_requested_output_format,
        pp_supported_output_format,
    )
}

pub(super) unsafe extern "win64" fn xapo_initialize(
    this: *mut BuiltinXapoCom,
    p_data: *const c_void,
    data_byte_size: u32,
) -> i32 {
    if this.is_null() {
        return E_POINTER;
    }
    let obj = &*this;
    obj.inner.lock().unwrap().initialize(p_data, data_byte_size)
}

unsafe extern "win64" fn xapo_reset(this: *mut BuiltinXapoCom) {
    if this.is_null() {
        return;
    }
    let obj = &*this;
    obj.inner.lock().unwrap().reset();
}

unsafe extern "win64" fn xapo_lock_for_process(
    this: *mut BuiltinXapoCom,
    in_count: u32,
    p_in_params: *const XapoLockForProcessBufferParameters,
    out_count: u32,
    p_out_params: *const XapoLockForProcessBufferParameters,
) -> i32 {
    if this.is_null() || p_in_params.is_null() || p_out_params.is_null() {
        return E_POINTER;
    }
    if in_count != 1 || out_count != 1 {
        return E_INVALIDARG;
    }
    let in_p = rd(p_in_params);
    let out_p = rd(p_out_params);
    let Ok((in_ch, in_sr)) = parse_xapo_float_format(in_p.format) else {
        return XAPO_E_FORMAT_UNSUPPORTED;
    };
    let Ok((out_ch, out_sr)) = parse_xapo_float_format(out_p.format) else {
        return XAPO_E_FORMAT_UNSUPPORTED;
    };
    if in_sr != out_sr {
        return XAPO_E_FORMAT_UNSUPPORTED;
    }
    let obj = &*this;
    obj.inner
        .lock()
        .unwrap()
        .lock_for_process(in_sr, in_ch, out_ch)
}

unsafe extern "win64" fn xapo_unlock_for_process(this: *mut BuiltinXapoCom) {
    if this.is_null() {
        return;
    }
    let obj = &*this;
    obj.inner.lock().unwrap().unlock_for_process();
}

unsafe extern "win64" fn xapo_process(
    this: *mut BuiltinXapoCom,
    in_count: u32,
    p_in_params: *const XapoProcessBufferParameters,
    out_count: u32,
    p_out_params: *mut XapoProcessBufferParameters,
    is_enabled: i32,
) {
    if this.is_null()
        || in_count == 0
        || out_count == 0
        || p_in_params.is_null()
        || p_out_params.is_null()
    {
        return;
    }
    let in_p = rd(p_in_params);
    let mut out_p = rd(p_out_params);
    let frames = in_p.valid_frame_count as usize;
    let in_silent = in_p.buffer_flags == XAPO_BUFFER_SILENT;
    let enabled = is_enabled != 0;

    let obj = &*this;
    let mut st = obj.inner.lock().unwrap();
    let in_ch = st.in_channels;
    let out_ch = st.out_channels;

    let out_valid =
        if frames == 0 || out_p.buffer.is_null() || (!in_silent && in_p.buffer.is_null()) {
            false
        } else if in_p.buffer == out_p.buffer && in_ch == out_ch {
            let buf = std::slice::from_raw_parts_mut(out_p.buffer as *mut f32, frames * out_ch);
            st.process_in_place(buf, out_ch, frames, in_silent, enabled)
        } else {
            let src_slice = if in_silent || in_p.buffer.is_null() {
                &[]
            } else {
                std::slice::from_raw_parts(in_p.buffer as *const f32, frames * in_ch)
            };
            let dst_slice =
                std::slice::from_raw_parts_mut(out_p.buffer as *mut f32, frames * out_ch);
            st.process_out_of_place(
                src_slice, in_ch, dst_slice, out_ch, frames, in_silent, enabled,
            )
        };

    out_p.valid_frame_count = in_p.valid_frame_count;
    out_p.buffer_flags = if out_valid {
        XAPO_BUFFER_VALID
    } else {
        XAPO_BUFFER_SILENT
    };
    wr(p_out_params, out_p);
}

pub(super) unsafe extern "win64" fn xapo_calc_input_frames(
    _this: *mut BuiltinXapoCom,
    output_frame_count: u32,
) -> u32 {
    output_frame_count
}

pub(super) unsafe extern "win64" fn xapo_calc_output_frames(
    _this: *mut BuiltinXapoCom,
    input_frame_count: u32,
) -> u32 {
    input_frame_count
}

pub(super) unsafe extern "win64" fn xapo_params_query_interface(
    this_params: *mut c_void,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> i32 {
    xapo_qi_common(xapo_from_params_ptr(this_params), riid, ppv)
}

unsafe extern "win64" fn xapo_params_add_ref(this_params: *mut c_void) -> u32 {
    xapo_add_ref(xapo_from_params_ptr(this_params))
}

pub(super) unsafe extern "win64" fn xapo_params_release(this_params: *mut c_void) -> u32 {
    xapo_release(xapo_from_params_ptr(this_params))
}

unsafe extern "win64" fn xapo_params_set_parameters(
    this_params: *mut c_void,
    p_params: *const c_void,
    byte_size: u32,
) {
    let base = xapo_from_params_ptr(this_params);
    if base.is_null() {
        return;
    }
    let obj = &*base;
    let _ = obj.inner.lock().unwrap().set_parameters(p_params, byte_size);
}

unsafe extern "win64" fn xapo_params_get_parameters(
    this_params: *mut c_void,
    p_params: *mut c_void,
    byte_size: u32,
) {
    let base = xapo_from_params_ptr(this_params);
    if base.is_null() {
        return;
    }
    let obj = &*base;
    let _ = obj.inner.lock().unwrap().get_parameters(p_params, byte_size);
}

// ============================================================================
// Voice Effect Chain Attachments (Builtin Fast-Path + Arbitrary Guest COM IXAPO)
// ============================================================================

pub(super) struct BuiltinXapoAttachment {
    com_ptr: usize,
    pub(super) state: Arc<Mutex<BuiltinXapoState>>,
}

impl Drop for BuiltinXapoAttachment {
    fn drop(&mut self) {
        self.state.lock().unwrap().unlock_for_process();
        if self.com_ptr != 0 {
            unsafe {
                xapo_release(self.com_ptr as *mut BuiltinXapoCom);
            }
        }
    }
}

pub(super) struct GuestXapoHandle {
    pub(super) xapo_ptr: usize,
    pub(super) params_ptr: usize,
    pub(super) process_lock: Mutex<()>,
}

impl Drop for GuestXapoHandle {
    fn drop(&mut self) {
        let _g = self.process_lock.lock().unwrap();
        unsafe {
            if self.xapo_ptr != 0 {
                let _ = com_call_slot_raw(self.xapo_ptr as *mut c_void, 9, &[]);
            }
            if self.params_ptr != 0 {
                com_call_release(self.params_ptr as *mut c_void);
            }
            if self.xapo_ptr != 0 {
                com_call_release(self.xapo_ptr as *mut c_void);
            }
        }
    }
}

#[derive(Clone)]
pub(super) enum EffectBackend {
    Builtin(Arc<BuiltinXapoAttachment>),
    Guest(Arc<GuestXapoHandle>),
}

pub(super) struct VoiceEffectSlot {
    pub(super) backend: EffectBackend,
    pub(super) enabled: bool,
    in_channels: usize,
    pub(super) output_channels: usize,
}

impl VoiceEffectSlot {
    pub(super) fn has_active_tail(&self) -> bool {
        if !self.enabled {
            return false;
        }
        match &self.backend {
            EffectBackend::Builtin(b) => b.state.lock().unwrap().has_active_tail(),
            EffectBackend::Guest(_) => true,
        }
    }
}

/// Build and lock an entire XAudio2 effect chain strictly OUTSIDE `EngineCore.state` lock so
/// guest `QueryInterface`/`LockForProcess` calls can never deadlock against XAudio2 state.
pub(super) unsafe fn build_effect_chain(
    effect_chain: *const XAudio2EffectChain,
    voice_in_channels: usize,
    sample_rate: u32,
    is_master: bool,
) -> Result<Vec<VoiceEffectSlot>, i32> {
    if effect_chain.is_null() {
        return Ok(Vec::new());
    }
    let chain = rd(effect_chain);
    let count = chain.effect_count as usize;
    if count == 0 {
        return Ok(Vec::new());
    }
    if chain.effects.is_null() {
        return Err(E_POINTER);
    }

    let mut descriptors = Vec::with_capacity(count);
    for i in 0..count {
        let desc = rd(chain.effects.add(i));
        let eff = desc.effect;
        let out_ch = desc.output_channels as usize;
        if eff.is_null() {
            return Err(E_POINTER);
        }
        if out_ch == 0 || out_ch > XAUDIO2_MAX_AUDIO_CHANNELS as usize {
            return Err(XAUDIO2_E_INVALID_CALL);
        }
        descriptors.push((eff, desc.initial_state != 0, out_ch));
    }

    if is_master {
        if let Some(&(_, _, last_out_ch)) = descriptors.last() {
            if last_out_ch != voice_in_channels {
                return Err(XAUDIO2_E_INVALID_CALL);
            }
        }
    }

    let mut slots = Vec::with_capacity(count);
    let mut cur_ch = voice_in_channels;

    for (eff_ptr, initial_enabled, out_ch) in descriptors {
        if let Some(base) = try_identify_builtin_xapo(eff_ptr) {
            let state_arc = {
                let obj = &*base;
                obj.refs.fetch_add(1, Ordering::AcqRel);
                obj.inner.clone()
            };
            let hr = state_arc
                .lock()
                .unwrap()
                .lock_for_process(sample_rate, cur_ch, out_ch);
            if hr < 0 {
                xapo_release(base);
                return Err(hr);
            }
            let att = Arc::new(BuiltinXapoAttachment {
                com_ptr: base as usize,
                state: state_arc,
            });
            slots.push(VoiceEffectSlot {
                backend: EffectBackend::Builtin(att),
                enabled: initial_enabled,
                in_channels: cur_ch,
                output_channels: out_ch,
            });
        } else {
            // Arbitrary guest COM object: QI for IXAPO (required) and IXAPOParameters (optional).
            let iid_xapo = Guid::from_win_guid(IID_IXAPO);
            let mut xapo_raw: *mut c_void = std::ptr::null_mut();
            let hr_xapo = com_call_qi(eff_ptr, &iid_xapo, &mut xapo_raw);
            if hr_xapo < 0 || xapo_raw.is_null() {
                return Err(if hr_xapo < 0 {
                    hr_xapo
                } else {
                    XAUDIO2_E_INVALID_CALL
                });
            }
            let xapo_ptr = xapo_raw as usize;

            let iid_params = Guid::from_win_guid(IID_IXAPO_PARAMETERS);
            let mut params_raw: *mut c_void = std::ptr::null_mut();
            let hr_params = com_call_qi(eff_ptr, &iid_params, &mut params_raw);
            let params_ptr = if hr_params < 0 {
                0
            } else {
                params_raw as usize
            };

            let in_wfx = make_float_wfx(cur_ch, sample_rate);
            let out_wfx = make_float_wfx(out_ch, sample_rate);
            let in_lock = XapoLockForProcessBufferParameters {
                format: &in_wfx,
                max_frame_count: XAPO_DEFAULT_MAX_FRAMES,
            };
            let out_lock = XapoLockForProcessBufferParameters {
                format: &out_wfx,
                max_frame_count: XAPO_DEFAULT_MAX_FRAMES,
            };
            let hr_lock = com_call_slot_raw(
                xapo_raw,
                8,
                &[
                    1,
                    &in_lock as *const XapoLockForProcessBufferParameters as u64,
                    1,
                    &out_lock as *const XapoLockForProcessBufferParameters as u64,
                ],
            )
            .map(|r| r as i32)
            .unwrap_or(XAUDIO2_E_INVALID_CALL);
            if hr_lock < 0 {
                if params_ptr != 0 {
                    com_call_release(params_ptr as *mut c_void);
                }
                com_call_release(xapo_raw);
                return Err(hr_lock);
            }

            let handle = Arc::new(GuestXapoHandle {
                xapo_ptr,
                params_ptr,
                process_lock: Mutex::new(()),
            });
            slots.push(VoiceEffectSlot {
                backend: EffectBackend::Guest(handle),
                enabled: initial_enabled,
                in_channels: cur_ch,
                output_channels: out_ch,
            });
        }
        cur_ch = out_ch;
    }

    Ok(slots)
}

/// Execute a voice's XAPO effect chain in order on `buf` (`frames * initial_ch`).
///
/// Updates `buf` in place (resizing if an effect changes the channel count) and returns
/// `(final_channels, output_is_valid)`.
pub(super) fn process_effect_chain(
    effects: &[VoiceEffectSlot],
    buf: &mut Vec<f32>,
    initial_ch: usize,
    frames: usize,
    in_silent: bool,
    fx_a: &mut AlignedF32Buf,
    fx_b: &mut AlignedF32Buf,
) -> (usize, bool) {
    if effects.is_empty() || frames == 0 {
        return (initial_ch, !in_silent);
    }

    let mut cur_ch = initial_ch;
    let mut cur_valid = !in_silent;
    // Use `fx_a` as current input and `fx_b` as output, ping-ponging across slots.
    let mut active_is_a = true;
    if cur_valid {
        fx_a.copy_from_slice(&buf[..frames * cur_ch]);
    } else {
        fx_a.resize_zero(frames * cur_ch);
    }

    for slot in effects {
        let in_ch = slot.in_channels;
        let out_ch = slot.output_channels;
        if in_ch != cur_ch {
            // Re-adapt defensively if channel count ever diverged.
            let (src_buf, dst_buf) = if active_is_a {
                (&*fx_a, &mut *fx_b)
            } else {
                (&*fx_b, &mut *fx_a)
            };
            dst_buf.resize_zero(frames * in_ch);
            if cur_valid {
                adapt_channels(
                    src_buf.as_slice(),
                    cur_ch,
                    dst_buf.as_mut_slice(),
                    in_ch,
                    frames,
                );
            }
            active_is_a = !active_is_a;
            cur_ch = in_ch;
        }

        match &slot.backend {
            EffectBackend::Builtin(att) => {
                if !slot.enabled && in_ch == out_ch {
                    // Disabled in-place builtin effect: pure pass-through.
                    continue;
                }
                let mut st = att.state.lock().unwrap();
                if in_ch == out_ch {
                    let cur_buf = if active_is_a {
                        fx_a.as_mut_slice()
                    } else {
                        fx_b.as_mut_slice()
                    };
                    cur_valid =
                        st.process_in_place(cur_buf, out_ch, frames, !cur_valid, slot.enabled);
                } else {
                    let (src_buf, dst_buf) = if active_is_a {
                        (&*fx_a, &mut *fx_b)
                    } else {
                        (&*fx_b, &mut *fx_a)
                    };
                    dst_buf.resize_zero(frames * out_ch);
                    cur_valid = st.process_out_of_place(
                        src_buf.as_slice(),
                        in_ch,
                        dst_buf.as_mut_slice(),
                        out_ch,
                        frames,
                        !cur_valid,
                        slot.enabled,
                    );
                    active_is_a = !active_is_a;
                    cur_ch = out_ch;
                }
            }
            EffectBackend::Guest(guest) => {
                let _g = guest.process_lock.lock().unwrap();
                let (in_ptr, out_ptr, switched) = if in_ch == out_ch {
                    let cur = if active_is_a { &mut *fx_a } else { &mut *fx_b };
                    let p = cur.as_mut_ptr() as *mut c_void;
                    (p, p, false)
                } else {
                    let (src_buf, dst_buf) = if active_is_a {
                        (&mut *fx_a, &mut *fx_b)
                    } else {
                        (&mut *fx_b, &mut *fx_a)
                    };
                    dst_buf.resize_zero(frames * out_ch);
                    (
                        src_buf.as_mut_ptr() as *mut c_void,
                        dst_buf.as_mut_ptr() as *mut c_void,
                        true,
                    )
                };
                let in_param = XapoProcessBufferParameters {
                    buffer: in_ptr,
                    buffer_flags: if cur_valid {
                        XAPO_BUFFER_VALID
                    } else {
                        XAPO_BUFFER_SILENT
                    },
                    valid_frame_count: frames as u32,
                };
                let mut out_param = XapoProcessBufferParameters {
                    buffer: out_ptr,
                    buffer_flags: if cur_valid {
                        XAPO_BUFFER_VALID
                    } else {
                        XAPO_BUFFER_SILENT
                    },
                    valid_frame_count: frames as u32,
                };
                unsafe {
                    let _ = com_call_slot_raw(
                        guest.xapo_ptr as *mut c_void,
                        10,
                        &[
                            1,
                            &in_param as *const XapoProcessBufferParameters as u64,
                            1,
                            &mut out_param as *mut XapoProcessBufferParameters as u64,
                            if slot.enabled { 1 } else { 0 },
                        ],
                    );
                }
                cur_valid = out_param.buffer_flags != XAPO_BUFFER_SILENT;
                if switched {
                    active_is_a = !active_is_a;
                    cur_ch = out_ch;
                }
                if !cur_valid {
                    let cur = if active_is_a { &mut *fx_a } else { &mut *fx_b };
                    cur.as_mut_slice().fill(0.0);
                }
            }
        }
    }

    let final_slice = if active_is_a {
        fx_a.as_slice()
    } else {
        fx_b.as_slice()
    };
    let final_len = frames * cur_ch;
    buf.clear();
    if cur_valid {
        buf.extend_from_slice(&final_slice[..final_len]);
    } else {
        buf.resize(final_len, 0.0);
    }
    (cur_ch, cur_valid)
}
