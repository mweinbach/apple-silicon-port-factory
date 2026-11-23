//! CoreAudio-backed OpenAL 1.1 runtime (`openal32.dll`).
//!
//! Implements the OpenAL 1.1 ALC (device/context management) and AL (buffers,
//! sources, streaming queues, 3D listener, distance attenuation models, stereo
//! panning, linear-interpolation resampling, and error reporting) APIs on top
//! of [`crate::audio::backend`] (CoreAudio AUHAL on macOS, RemoteIO on iOS,
//! and the real-time Null backend when `AW2_AUDIO_BACKEND=null` or headless).

#![allow(non_snake_case)]

mod mixer;
mod state;

use self::mixer::MixerRuntime;
use self::state::{
    buffer_set_invalid, ensure_mixer_running, get_buffer_prop, get_global_param_f64,
    get_source_float, get_source_int, get_source_vec3, intern_cstr, intern_cstr_list,
    listener_set_float, listener_set_orientation, listener_set_vec3, lock_state, source_batch_op,
    source_set_float_prop, source_set_int_prop, source_set_vec3_prop, BufferData, Context, Device,
    Source, ALC_ALL_ATTRIBUTES, ALC_ALL_DEVICES_SPECIFIER, ALC_ATTRIBUTES_SIZE,
    ALC_CAPTURE_DEFAULT_DEVICE_SPECIFIER, ALC_CAPTURE_DEVICE_SPECIFIER, ALC_CAPTURE_SAMPLES,
    ALC_DEFAULT_ALL_DEVICES_SPECIFIER, ALC_DEFAULT_DEVICE_SPECIFIER, ALC_DEVICE_SPECIFIER,
    ALC_EXTENSIONS, ALC_FALSE, ALC_FREQUENCY, ALC_INVALID_CONTEXT, ALC_INVALID_DEVICE,
    ALC_INVALID_ENUM, ALC_INVALID_VALUE, ALC_MAJOR_VERSION, ALC_MINOR_VERSION, ALC_MONO_SOURCES,
    ALC_NO_ERROR, ALC_OUT_OF_MEMORY, ALC_REFRESH, ALC_STEREO_SOURCES, ALC_SYNC, ALC_TRUE, AL_BITS,
    AL_BUFFER, AL_BUFFERS_PROCESSED, AL_BUFFERS_QUEUED, AL_BYTE_OFFSET, AL_CHANNELS,
    AL_CONE_INNER_ANGLE, AL_CONE_OUTER_ANGLE, AL_CONE_OUTER_GAIN, AL_DIRECTION, AL_DISTANCE_MODEL,
    AL_DOPPLER_FACTOR, AL_DOPPLER_VELOCITY, AL_EXPONENT_DISTANCE, AL_EXPONENT_DISTANCE_CLAMPED,
    AL_EXTENSIONS, AL_FALSE, AL_FORMAT_MONO16, AL_FORMAT_MONO8, AL_FORMAT_MONO_FLOAT32,
    AL_FORMAT_STEREO16, AL_FORMAT_STEREO8, AL_FORMAT_STEREO_FLOAT32, AL_FREQUENCY, AL_GAIN,
    AL_INITIAL, AL_INVALID_ENUM, AL_INVALID_NAME, AL_INVALID_OPERATION, AL_INVALID_VALUE,
    AL_INVERSE_DISTANCE, AL_INVERSE_DISTANCE_CLAMPED, AL_LINEAR_DISTANCE,
    AL_LINEAR_DISTANCE_CLAMPED, AL_LOOPING, AL_MAX_DISTANCE, AL_MAX_GAIN, AL_MIN_GAIN, AL_NONE,
    AL_NO_ERROR, AL_ORIENTATION, AL_OUT_OF_MEMORY, AL_PAUSED, AL_PITCH, AL_PLAYING, AL_POSITION,
    AL_REFERENCE_DISTANCE, AL_RENDERER, AL_ROLLOFF_FACTOR, AL_SAMPLE_OFFSET, AL_SEC_OFFSET,
    AL_SIZE, AL_SOURCE_DISTANCE_MODEL, AL_SOURCE_RELATIVE, AL_SOURCE_STATE, AL_SOURCE_TYPE,
    AL_SPEED_OF_SOUND, AL_STATIC, AL_STOPPED, AL_STREAMING, AL_TRUE, AL_UNDETERMINED, AL_VELOCITY,
    AL_VENDOR, AL_VERSION, DEFAULT_MONO_SOURCES, DEFAULT_OUTPUT_RATE, DEFAULT_REFRESH,
    DEFAULT_STEREO_SOURCES,
};
use crate::audio::backend;
use crate::audio::format::{decode_pcm_buffer_f32, SampleType};
use crate::strings::{rd, wr};
use std::ffi::c_void;
use std::sync::Arc;

// ---------------------------------------------------------------------------
// Export tables (mixed int+float Win64 ABI entries bridged for aarch64)
// ---------------------------------------------------------------------------

pub static ORDINALS: &[(u16, &str)] = &[];

pub static EXTRA_EXPORTS: &[crate::Export] = &[
    crate::Export {
        name: "alSourcef",
        ptr: crate::Ptr(crate::bridge_entry!(al_source_f)),
    },
    crate::Export {
        name: "alSource3f",
        ptr: crate::Ptr(crate::bridge_entry!(al_source_3f)),
    },
    crate::Export {
        name: "alListenerf",
        ptr: crate::Ptr(crate::bridge_entry!(al_listener_f)),
    },
    crate::Export {
        name: "alListener3f",
        ptr: crate::Ptr(crate::bridge_entry!(al_listener_3f)),
    },
    crate::Export {
        name: "alBufferf",
        ptr: crate::Ptr(crate::bridge_entry!(al_buffer_f)),
    },
    crate::Export {
        name: "alBuffer3f",
        ptr: crate::Ptr(crate::bridge_entry!(al_buffer_3f)),
    },
];

// ---------------------------------------------------------------------------
// ALC 1.1 API Exports
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn alcOpenDevice(devicename: *const u8) -> usize {
    let devs = backend::output_devices();
    let chosen = match crate::strings::from_ansi_opt(devicename) {
        None => devs.into_iter().next(),
        Some(ref s) if s.is_empty() => devs.into_iter().next(),
        Some(want) => {
            let lw = want.to_lowercase();
            let exact = devs
                .iter()
                .position(|d| d.name == want || d.uid == want)
                .or_else(|| devs.iter().position(|d| d.name.to_lowercase() == lw))
                .or_else(|| devs.iter().position(|d| d.name.to_lowercase().contains(&lw)));
            if let Some(idx) = exact {
                Some(devs.into_iter().nth(idx).unwrap())
            } else if matches!(
                lw.as_str(),
                "generic software"
                    | "generic hardware"
                    | "directsound"
                    | "directsound3d"
                    | "mmsystem"
                    | "openal soft"
                    | "openal soft on default"
            ) {
                devs.into_iter().next()
            } else {
                None
            }
        }
    };

    let Some(info) = chosen else {
        lock_state().set_alc_error(0, ALC_INVALID_VALUE);
        return 0;
    };

    log::info!(
        target: "audio",
        "openal32: alcOpenDevice -> '{}' ({} Hz, {} ch)",
        info.name,
        info.rate,
        info.channels
    );
    let mut st = lock_state();
    let handle = st.alloc_handle();
    st.devices.insert(handle, Device::new(info));
    handle
}

pub unsafe extern "win64" fn alcCloseDevice(device: usize) -> u8 {
    let mixers_to_stop: Vec<MixerRuntime> = {
        let mut st = lock_state();
        let Some(dev) = st.devices.remove(&device) else {
            st.set_alc_error(0, ALC_INVALID_DEVICE);
            return ALC_FALSE;
        };
        let mut mixers = Vec::new();
        for cid in dev.contexts {
            if st.current_context == Some(cid) {
                st.current_context = None;
            }
            if let Some(mut ctx) = st.contexts.remove(&cid) {
                if let Some(m) = ctx.mixer.take() {
                    mixers.push(m);
                }
            }
        }
        mixers
    };
    for m in mixers_to_stop {
        m.stop();
    }
    ALC_TRUE
}

pub unsafe extern "win64" fn alcCreateContext(device: usize, attrlist: *const i32) -> usize {
    let mut st = lock_state();
    let Some(dev) = st.devices.get_mut(&device) else {
        st.set_alc_error(0, ALC_INVALID_DEVICE);
        return 0;
    };

    let mut output_rate = DEFAULT_OUTPUT_RATE;
    let mut refresh = dev.refresh;
    let mut sync = false;
    let mut mono_sources = dev.mono_sources;
    let mut stereo_sources = dev.stereo_sources;

    if !attrlist.is_null() {
        let mut idx = 0usize;
        while idx < 128 {
            let key = rd(attrlist.add(idx));
            if key == 0 {
                break;
            }
            let val = rd(attrlist.add(idx + 1));
            idx += 2;
            match key {
                ALC_FREQUENCY => {
                    if val <= 0 {
                        dev.set_error(ALC_INVALID_VALUE);
                        return 0;
                    }
                    output_rate = (val as u32).clamp(8_000, 192_000);
                }
                ALC_REFRESH => {
                    if val <= 0 {
                        dev.set_error(ALC_INVALID_VALUE);
                        return 0;
                    }
                    refresh = val;
                }
                ALC_SYNC => {
                    sync = val != 0;
                }
                ALC_MONO_SOURCES => {
                    if val < 0 {
                        dev.set_error(ALC_INVALID_VALUE);
                        return 0;
                    }
                    mono_sources = val;
                }
                ALC_STEREO_SOURCES => {
                    if val < 0 {
                        dev.set_error(ALC_INVALID_VALUE);
                        return 0;
                    }
                    stereo_sources = val;
                }
                _ => {}
            }
        }
    }

    dev.output_rate = output_rate;
    dev.refresh = refresh;
    dev.sync = sync;
    dev.mono_sources = mono_sources;
    dev.stereo_sources = stereo_sources;

    let ctx_handle = st.alloc_handle();
    if let Some(dev) = st.devices.get_mut(&device) {
        dev.contexts.push(ctx_handle);
    }
    st.contexts.insert(
        ctx_handle,
        Context::new(
            device,
            output_rate,
            refresh,
            sync,
            mono_sources,
            stereo_sources,
        ),
    );
    ctx_handle
}

pub unsafe extern "win64" fn alcMakeContextCurrent(context: usize) -> u8 {
    let mut st = lock_state();
    if context == 0 {
        st.current_context = None;
        return ALC_TRUE;
    }
    if st.contexts.contains_key(&context) {
        st.current_context = Some(context);
        ALC_TRUE
    } else {
        st.set_alc_error(0, ALC_INVALID_CONTEXT);
        ALC_FALSE
    }
}

pub unsafe extern "win64" fn alcProcessContext(context: usize) {
    let should_start = {
        let mut st = lock_state();
        let Some(ctx) = st.contexts.get_mut(&context) else {
            st.set_alc_error(0, ALC_INVALID_CONTEXT);
            return;
        };
        ctx.suspended = false;
        !ctx.sync && ctx.has_playing_source() && ctx.mixer.is_none()
    };
    if should_start {
        ensure_mixer_running(context);
    }
}

pub unsafe extern "win64" fn alcSuspendContext(context: usize) {
    let mut st = lock_state();
    if let Some(ctx) = st.contexts.get_mut(&context) {
        ctx.suspended = true;
    } else {
        st.set_alc_error(0, ALC_INVALID_CONTEXT);
    }
}

pub unsafe extern "win64" fn alcDestroyContext(context: usize) {
    let mixer_to_stop = {
        let mut st = lock_state();
        let Some(mut ctx) = st.contexts.remove(&context) else {
            st.set_alc_error(0, ALC_INVALID_CONTEXT);
            return;
        };
        if st.current_context == Some(context) {
            st.current_context = None;
        }
        if let Some(dev) = st.devices.get_mut(&ctx.device) {
            dev.contexts.retain(|&c| c != context);
        }
        ctx.mixer.take()
    };
    if let Some(m) = mixer_to_stop {
        m.stop();
    }
}

pub unsafe extern "win64" fn alcGetCurrentContext() -> usize {
    lock_state().current_context.unwrap_or(0)
}

pub unsafe extern "win64" fn alcGetContextsDevice(context: usize) -> usize {
    let mut st = lock_state();
    if let Some(ctx) = st.contexts.get(&context) {
        ctx.device
    } else {
        st.set_alc_error(0, ALC_INVALID_CONTEXT);
        0
    }
}

pub unsafe extern "win64" fn alcGetError(device: usize) -> i32 {
    let mut st = lock_state();
    if device == 0 {
        let e = st.global_alc_error;
        st.global_alc_error = ALC_NO_ERROR;
        return e;
    }
    if let Some(dev) = st.devices.get_mut(&device) {
        let e = dev.last_error;
        dev.last_error = ALC_NO_ERROR;
        e
    } else {
        ALC_INVALID_DEVICE
    }
}

pub unsafe extern "win64" fn alcGetString(device: usize, param: i32) -> *const u8 {
    match param {
        ALC_NO_ERROR => b"No Error\0".as_ptr(),
        ALC_INVALID_DEVICE => b"Invalid Device\0".as_ptr(),
        ALC_INVALID_CONTEXT => b"Invalid Context\0".as_ptr(),
        ALC_INVALID_ENUM => b"Invalid Enum\0".as_ptr(),
        ALC_INVALID_VALUE => b"Invalid Value\0".as_ptr(),
        ALC_OUT_OF_MEMORY => b"Out of Memory\0".as_ptr(),
        ALC_DEFAULT_DEVICE_SPECIFIER | ALC_DEFAULT_ALL_DEVICES_SPECIFIER => {
            let devs = backend::output_devices();
            let name = devs
                .first()
                .map(|d| d.name.as_str())
                .unwrap_or("CoreAudio Default");
            intern_cstr(name)
        }
        ALC_DEVICE_SPECIFIER | ALC_ALL_DEVICES_SPECIFIER => {
            if device == 0 {
                let devs = backend::output_devices();
                intern_cstr_list(&devs)
            } else {
                let mut st = lock_state();
                if let Some(dev) = st.devices.get(&device) {
                    let name = dev.info.name.clone();
                    drop(st);
                    intern_cstr(&name)
                } else {
                    st.set_alc_error(0, ALC_INVALID_DEVICE);
                    std::ptr::null()
                }
            }
        }
        ALC_CAPTURE_DEFAULT_DEVICE_SPECIFIER => {
            let devs = backend::input_devices();
            let name = devs
                .first()
                .map(|d| d.name.as_str())
                .unwrap_or("Default Microphone");
            intern_cstr(name)
        }
        ALC_CAPTURE_DEVICE_SPECIFIER => {
            if device == 0 {
                let devs = backend::input_devices();
                intern_cstr_list(&devs)
            } else {
                let mut st = lock_state();
                if st.devices.contains_key(&device) {
                    drop(st);
                    let devs = backend::input_devices();
                    let name = devs
                        .first()
                        .map(|d| d.name.as_str())
                        .unwrap_or("Default Microphone");
                    intern_cstr(name)
                } else {
                    st.set_alc_error(0, ALC_INVALID_DEVICE);
                    std::ptr::null()
                }
            }
        }
        ALC_EXTENSIONS => {
            if device != 0 {
                let mut st = lock_state();
                if !st.devices.contains_key(&device) {
                    st.set_alc_error(0, ALC_INVALID_DEVICE);
                    return std::ptr::null();
                }
            }
            b"ALC_ENUMERATION_EXT ALC_ENUMERATE_ALL_EXT\0".as_ptr()
        }
        _ => {
            lock_state().set_alc_error(device, ALC_INVALID_ENUM);
            std::ptr::null()
        }
    }
}

pub unsafe extern "win64" fn alcGetIntegerv(
    device: usize,
    param: i32,
    size: i32,
    values: *mut i32,
) {
    if values.is_null() || size <= 0 {
        lock_state().set_alc_error(device, ALC_INVALID_VALUE);
        return;
    }

    match param {
        ALC_MAJOR_VERSION => {
            wr(values, 1);
            return;
        }
        ALC_MINOR_VERSION => {
            wr(values, 1);
            return;
        }
        _ => {}
    }

    let mut st = lock_state();
    if device != 0 && !st.devices.contains_key(&device) {
        st.set_alc_error(0, ALC_INVALID_DEVICE);
        return;
    }

    // Resolve attribute source: current context on device -> last context on device -> device defaults.
    let (freq, refresh, sync, mono, stereo) = if let Some(dev) = st.devices.get(&device) {
        let active_ctx = st
            .current_context
            .filter(|cid| dev.contexts.contains(cid))
            .or_else(|| dev.contexts.last().copied());
        if let Some(ctx) = active_ctx.and_then(|cid| st.contexts.get(&cid)) {
            (
                ctx.output_rate as i32,
                ctx.refresh,
                if ctx.sync { 1 } else { 0 },
                ctx.mono_sources,
                ctx.stereo_sources,
            )
        } else {
            (
                dev.output_rate as i32,
                dev.refresh,
                if dev.sync { 1 } else { 0 },
                dev.mono_sources,
                dev.stereo_sources,
            )
        }
    } else {
        (
            DEFAULT_OUTPUT_RATE as i32,
            DEFAULT_REFRESH,
            0,
            DEFAULT_MONO_SOURCES,
            DEFAULT_STEREO_SOURCES,
        )
    };

    match param {
        ALC_FREQUENCY => wr(values, freq),
        ALC_REFRESH => wr(values, refresh),
        ALC_SYNC => wr(values, sync),
        ALC_MONO_SOURCES => wr(values, mono),
        ALC_STEREO_SOURCES => wr(values, stereo),
        ALC_CAPTURE_SAMPLES => wr(values, 0),
        ALC_ATTRIBUTES_SIZE => {
            if device == 0 {
                st.set_alc_error(0, ALC_INVALID_DEVICE);
                return;
            }
            wr(values, 11);
        }
        ALC_ALL_ATTRIBUTES => {
            if device == 0 {
                st.set_alc_error(0, ALC_INVALID_DEVICE);
                return;
            }
            let attrs: [i32; 11] = [
                ALC_FREQUENCY,
                freq,
                ALC_REFRESH,
                refresh,
                ALC_SYNC,
                sync,
                ALC_MONO_SOURCES,
                mono,
                ALC_STEREO_SOURCES,
                stereo,
                0,
            ];
            if (size as usize) < attrs.len() {
                st.set_alc_error(device, ALC_INVALID_VALUE);
                return;
            }
            for (i, &v) in attrs.iter().enumerate() {
                wr(values.add(i), v);
            }
        }
        _ => st.set_alc_error(device, ALC_INVALID_ENUM),
    }
}

pub unsafe extern "win64" fn alcIsExtensionPresent(device: usize, extname: *const u8) -> u8 {
    let Some(name) = crate::strings::from_ansi_opt(extname) else {
        lock_state().set_alc_error(device, ALC_INVALID_VALUE);
        return ALC_FALSE;
    };
    if device != 0 && !lock_state().devices.contains_key(&device) {
        lock_state().set_alc_error(0, ALC_INVALID_DEVICE);
        return ALC_FALSE;
    }
    if name.eq_ignore_ascii_case("ALC_ENUMERATION_EXT")
        || name.eq_ignore_ascii_case("ALC_ENUMERATE_ALL_EXT")
    {
        ALC_TRUE
    } else {
        ALC_FALSE
    }
}

pub unsafe extern "win64" fn alcGetProcAddress(
    device: usize,
    funcname: *const u8,
) -> *const c_void {
    let Some(name) = crate::strings::from_ansi_opt(funcname) else {
        lock_state().set_alc_error(device, ALC_INVALID_VALUE);
        return std::ptr::null();
    };
    if device != 0 && !lock_state().devices.contains_key(&device) {
        lock_state().set_alc_error(0, ALC_INVALID_DEVICE);
        return std::ptr::null();
    }
    if let Some(ptr) = crate::lookup("openal32.dll", &name) {
        let addr = ptr as usize;
        crate::register_native_va(addr as u64, addr);
        ptr as *const c_void
    } else {
        std::ptr::null()
    }
}

pub unsafe extern "win64" fn alcGetEnumValue(device: usize, enumname: *const u8) -> i32 {
    let Some(name) = crate::strings::from_ansi_opt(enumname) else {
        lock_state().set_alc_error(device, ALC_INVALID_VALUE);
        return 0;
    };
    if device != 0 && !lock_state().devices.contains_key(&device) {
        lock_state().set_alc_error(0, ALC_INVALID_DEVICE);
        return 0;
    }
    match name.as_str() {
        "ALC_FALSE" => ALC_FALSE as i32,
        "ALC_TRUE" => ALC_TRUE as i32,
        "ALC_NO_ERROR" => ALC_NO_ERROR,
        "ALC_INVALID_DEVICE" => ALC_INVALID_DEVICE,
        "ALC_INVALID_CONTEXT" => ALC_INVALID_CONTEXT,
        "ALC_INVALID_ENUM" => ALC_INVALID_ENUM,
        "ALC_INVALID_VALUE" => ALC_INVALID_VALUE,
        "ALC_OUT_OF_MEMORY" => ALC_OUT_OF_MEMORY,
        "ALC_MAJOR_VERSION" => ALC_MAJOR_VERSION,
        "ALC_MINOR_VERSION" => ALC_MINOR_VERSION,
        "ALC_ATTRIBUTES_SIZE" => ALC_ATTRIBUTES_SIZE,
        "ALC_ALL_ATTRIBUTES" => ALC_ALL_ATTRIBUTES,
        "ALC_DEFAULT_DEVICE_SPECIFIER" => ALC_DEFAULT_DEVICE_SPECIFIER,
        "ALC_DEVICE_SPECIFIER" => ALC_DEVICE_SPECIFIER,
        "ALC_EXTENSIONS" => ALC_EXTENSIONS,
        "ALC_FREQUENCY" => ALC_FREQUENCY,
        "ALC_REFRESH" => ALC_REFRESH,
        "ALC_SYNC" => ALC_SYNC,
        "ALC_MONO_SOURCES" => ALC_MONO_SOURCES,
        "ALC_STEREO_SOURCES" => ALC_STEREO_SOURCES,
        "ALC_DEFAULT_ALL_DEVICES_SPECIFIER" => ALC_DEFAULT_ALL_DEVICES_SPECIFIER,
        "ALC_ALL_DEVICES_SPECIFIER" => ALC_ALL_DEVICES_SPECIFIER,
        "ALC_CAPTURE_DEVICE_SPECIFIER" => ALC_CAPTURE_DEVICE_SPECIFIER,
        "ALC_CAPTURE_DEFAULT_DEVICE_SPECIFIER" => ALC_CAPTURE_DEFAULT_DEVICE_SPECIFIER,
        "ALC_CAPTURE_SAMPLES" => ALC_CAPTURE_SAMPLES,
        _ => 0,
    }
}

// ---------------------------------------------------------------------------
// AL 1.1 Buffer API Exports
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn alGenBuffers(n: i32, buffers: *mut u32) {
    let mut st = lock_state();
    let Some(cid) = st.current_context else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    if n == 0 {
        return;
    }
    if n < 0 || buffers.is_null() {
        st.set_al_error(AL_INVALID_VALUE);
        return;
    }
    let Some(dev_handle) = st.contexts.get(&cid).map(|c| c.device) else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    let mut ids = Vec::with_capacity(n as usize);
    for _ in 0..(n as usize) {
        ids.push(st.alloc_al_id());
    }
    if let Some(dev) = st.devices.get_mut(&dev_handle) {
        for (i, id) in ids.into_iter().enumerate() {
            dev.buffers.insert(id, Arc::new(BufferData::default()));
            wr(buffers.add(i), id);
        }
    }
}

pub unsafe extern "win64" fn alDeleteBuffers(n: i32, buffers: *const u32) {
    let mut st = lock_state();
    let Some(cid) = st.current_context else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    if n == 0 {
        return;
    }
    if n < 0 || buffers.is_null() {
        st.set_al_error(AL_INVALID_VALUE);
        return;
    }
    let Some(dev_handle) = st.contexts.get(&cid).map(|c| c.device) else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    let mut ids = Vec::with_capacity(n as usize);
    for i in 0..(n as usize) {
        ids.push(rd(buffers.add(i)));
    }
    // Atomic validation pass (OpenAL 1.1 spec): verify every ID before removing any.
    for &bid in &ids {
        if bid == 0 {
            continue;
        }
        let exists = st
            .devices
            .get(&dev_handle)
            .is_some_and(|d| d.buffers.contains_key(&bid));
        if !exists {
            st.set_al_error(AL_INVALID_NAME);
            return;
        }
        if st.buffer_in_use_on_device(dev_handle, bid, false) {
            st.set_al_error(AL_INVALID_OPERATION);
            return;
        }
    }
    if let Some(dev) = st.devices.get_mut(&dev_handle) {
        for bid in ids {
            if bid != 0 {
                dev.buffers.remove(&bid);
            }
        }
    }
}

pub unsafe extern "win64" fn alIsBuffer(buffer: u32) -> u8 {
    let mut st = lock_state();
    let Some(cid) = st.current_context else {
        st.set_al_error(AL_INVALID_OPERATION);
        return AL_FALSE;
    };
    if buffer == 0 {
        // OpenAL 1.1 spec: alIsBuffer(0) returns AL_TRUE (NULL buffer).
        return AL_TRUE;
    }
    let Some(dev_handle) = st.contexts.get(&cid).map(|c| c.device) else {
        return AL_FALSE;
    };
    if st
        .devices
        .get(&dev_handle)
        .is_some_and(|d| d.buffers.contains_key(&buffer))
    {
        AL_TRUE
    } else {
        AL_FALSE
    }
}

pub unsafe extern "win64" fn alBufferData(
    buffer: u32,
    format: i32,
    data: *const u8,
    size: i32,
    freq: i32,
) {
    let mut st = lock_state();
    let Some(cid) = st.current_context else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    let Some(dev_handle) = st.contexts.get(&cid).map(|c| c.device) else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    if buffer == 0
        || !st
            .devices
            .get(&dev_handle)
            .is_some_and(|d| d.buffers.contains_key(&buffer))
    {
        st.set_al_error(AL_INVALID_NAME);
        return;
    }
    if st.buffer_in_use_on_device(dev_handle, buffer, true) {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    }
    let (channels, bits, frame_bytes, sample_kind): (u16, u16, usize, SampleType) = match format {
        AL_FORMAT_MONO8 => (1, 8, 1, SampleType::U8),
        AL_FORMAT_MONO16 => (1, 16, 2, SampleType::I16),
        AL_FORMAT_STEREO8 => (2, 8, 2, SampleType::U8),
        AL_FORMAT_STEREO16 => (2, 16, 4, SampleType::I16),
        AL_FORMAT_MONO_FLOAT32 => (1, 32, 4, SampleType::F32),
        AL_FORMAT_STEREO_FLOAT32 => (2, 32, 8, SampleType::F32),
        _ => {
            st.set_al_error(AL_INVALID_ENUM);
            return;
        }
    };
    if size < 0 || freq <= 0 || (size > 0 && data.is_null()) || (size as usize) % frame_bytes != 0 {
        st.set_al_error(AL_INVALID_VALUE);
        return;
    }

    let byte_len = size as usize;
    let raw: &[u8] = if byte_len == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(data, byte_len)
    };

    let mut samples = Vec::new();
    decode_pcm_buffer_f32(sample_kind, raw, &mut samples);

    let buf = Arc::new(BufferData {
        samples,
        freq: freq as u32,
        channels,
        bits,
        size_bytes: size,
    });
    if let Some(dev) = st.devices.get_mut(&dev_handle) {
        dev.buffers.insert(buffer, buf);
    }
}

pub unsafe extern "win64" fn alBufferi(buffer: u32, param: i32, _value: i32) {
    buffer_set_invalid(buffer, param, false);
}

pub unsafe extern "win64" fn alBuffer3i(
    buffer: u32,
    param: i32,
    _v1: i32,
    _v2: i32,
    _v3: i32,
) {
    buffer_set_invalid(buffer, param, false);
}

pub unsafe extern "win64" fn alBufferiv(buffer: u32, param: i32, values: *const i32) {
    buffer_set_invalid(buffer, param, values.is_null());
}

pub unsafe extern "win64" fn alBufferfv(buffer: u32, param: i32, values: *const f32) {
    buffer_set_invalid(buffer, param, values.is_null());
}

unsafe extern "win64" fn al_buffer_f(buffer: u32, param: i32, _value: f32) {
    buffer_set_invalid(buffer, param, false);
}
crate::win64_bridge!(fn al_buffer_f(u32, i32, f32) -> ());

unsafe extern "win64" fn al_buffer_3f(
    buffer: u32,
    param: i32,
    _v1: f32,
    _v2: f32,
    _v3: f32,
) {
    buffer_set_invalid(buffer, param, false);
}
crate::win64_bridge!(fn al_buffer_3f(u32, i32, f32, f32, f32) -> ());

pub unsafe extern "win64" fn alGetBufferi(buffer: u32, param: i32, value: *mut i32) {
    if value.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    match get_buffer_prop(buffer, param) {
        Ok(v) => wr(value, v),
        Err(e) => lock_state().set_al_error(e),
    }
}

pub unsafe extern "win64" fn alGetBufferiv(buffer: u32, param: i32, values: *mut i32) {
    alGetBufferi(buffer, param, values);
}

pub unsafe extern "win64" fn alGetBuffer3i(
    buffer: u32,
    param: i32,
    v1: *mut i32,
    v2: *mut i32,
    v3: *mut i32,
) {
    buffer_set_invalid(buffer, param, v1.is_null() || v2.is_null() || v3.is_null());
}

pub unsafe extern "win64" fn alGetBufferf(buffer: u32, param: i32, value: *mut f32) {
    if value.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    match get_buffer_prop(buffer, param) {
        Ok(v) => wr(value, v as f32),
        Err(e) => lock_state().set_al_error(e),
    }
}

pub unsafe extern "win64" fn alGetBufferfv(buffer: u32, param: i32, values: *mut f32) {
    alGetBufferf(buffer, param, values);
}

pub unsafe extern "win64" fn alGetBuffer3f(
    buffer: u32,
    param: i32,
    v1: *mut f32,
    v2: *mut f32,
    v3: *mut f32,
) {
    buffer_set_invalid(buffer, param, v1.is_null() || v2.is_null() || v3.is_null());
}

// ---------------------------------------------------------------------------
// AL 1.1 Source API Exports
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn alGenSources(n: i32, sources: *mut u32) {
    let mut st = lock_state();
    let Some(cid) = st.current_context else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    if n == 0 {
        return;
    }
    if n < 0 || sources.is_null() {
        st.set_al_error(AL_INVALID_VALUE);
        return;
    }
    let mut ids = Vec::with_capacity(n as usize);
    for _ in 0..(n as usize) {
        ids.push(st.alloc_al_id());
    }
    if let Some(ctx) = st.contexts.get_mut(&cid) {
        for (i, id) in ids.into_iter().enumerate() {
            ctx.sources.insert(id, Source::default());
            wr(sources.add(i), id);
        }
    }
}

pub unsafe extern "win64" fn alDeleteSources(n: i32, sources: *const u32) {
    let mut st = lock_state();
    let Some(cid) = st.current_context else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    if n == 0 {
        return;
    }
    if n < 0 || sources.is_null() {
        st.set_al_error(AL_INVALID_VALUE);
        return;
    }
    let mut ids = Vec::with_capacity(n as usize);
    for i in 0..(n as usize) {
        ids.push(rd(sources.add(i)));
    }
    let Some(ctx) = st.contexts.get_mut(&cid) else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    for &sid in &ids {
        if !ctx.sources.contains_key(&sid) {
            ctx.set_error(AL_INVALID_NAME);
            return;
        }
    }
    for sid in ids {
        ctx.sources.remove(&sid);
    }
}

pub unsafe extern "win64" fn alIsSource(source: u32) -> u8 {
    let mut st = lock_state();
    let Some(cid) = st.current_context else {
        st.set_al_error(AL_INVALID_OPERATION);
        return AL_FALSE;
    };
    if st
        .contexts
        .get(&cid)
        .is_some_and(|c| c.sources.contains_key(&source))
    {
        AL_TRUE
    } else {
        AL_FALSE
    }
}

pub unsafe extern "win64" fn alSourcePlay(source: u32) {
    let start_ctx = {
        let mut st = lock_state();
        let Some(cid) = st.current_context else {
            st.set_al_error(AL_INVALID_OPERATION);
            return;
        };
        let Some(ctx) = st.contexts.get_mut(&cid) else {
            st.set_al_error(AL_INVALID_OPERATION);
            return;
        };
        let Some(src) = ctx.sources.get_mut(&source) else {
            ctx.set_error(AL_INVALID_NAME);
            return;
        };
        src.play();
        if src.state == AL_PLAYING && !ctx.sync && !ctx.suspended && ctx.mixer.is_none() {
            Some(cid)
        } else {
            None
        }
    };
    if let Some(cid) = start_ctx {
        ensure_mixer_running(cid);
    }
}

pub unsafe extern "win64" fn alSourcePause(source: u32) {
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    if let Some(src) = ctx.sources.get_mut(&source) {
        src.pause();
    } else {
        ctx.set_error(AL_INVALID_NAME);
    }
}

pub unsafe extern "win64" fn alSourceStop(source: u32) {
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    if let Some(src) = ctx.sources.get_mut(&source) {
        src.stop();
    } else {
        ctx.set_error(AL_INVALID_NAME);
    }
}

pub unsafe extern "win64" fn alSourceRewind(source: u32) {
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    if let Some(src) = ctx.sources.get_mut(&source) {
        src.rewind();
    } else {
        ctx.set_error(AL_INVALID_NAME);
    }
}

pub unsafe extern "win64" fn alSourcePlayv(n: i32, sources: *const u32) {
    if let Some(cid) = source_batch_op(n, sources, Source::play) {
        ensure_mixer_running(cid);
    }
}

pub unsafe extern "win64" fn alSourcePausev(n: i32, sources: *const u32) {
    let _ = source_batch_op(n, sources, Source::pause);
}

pub unsafe extern "win64" fn alSourceStopv(n: i32, sources: *const u32) {
    let _ = source_batch_op(n, sources, Source::stop);
}

pub unsafe extern "win64" fn alSourceRewindv(n: i32, sources: *const u32) {
    let _ = source_batch_op(n, sources, Source::rewind);
}

pub unsafe extern "win64" fn alSourceQueueBuffers(source: u32, nb: i32, buffers: *const u32) {
    let mut st = lock_state();
    let Some(cid) = st.current_context else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    if nb == 0 {
        return;
    }
    if nb < 0 || buffers.is_null() {
        st.set_al_error(AL_INVALID_VALUE);
        return;
    }
    let Some(dev_handle) = st.contexts.get(&cid).map(|c| c.device) else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };

    let mut bids = Vec::with_capacity(nb as usize);
    for i in 0..(nb as usize) {
        bids.push(rd(buffers.add(i)));
    }

    // Validate source exists and is not AL_STATIC.
    let src_type = match st.contexts.get(&cid).and_then(|c| c.sources.get(&source)) {
        Some(s) => s.source_type,
        None => {
            st.set_al_error(AL_INVALID_NAME);
            return;
        }
    };
    if src_type == AL_STATIC {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    }

    // Validate every non-zero buffer ID exists on this device.
    for &bid in &bids {
        if bid != 0
            && !st
                .devices
                .get(&dev_handle)
                .is_some_and(|d| d.buffers.contains_key(&bid))
        {
            st.set_al_error(AL_INVALID_NAME);
            return;
        }
    }

    if let Some(src) = st
        .contexts
        .get_mut(&cid)
        .and_then(|c| c.sources.get_mut(&source))
    {
        src.source_type = AL_STREAMING;
        for bid in bids {
            src.queue.push_back(bid);
        }
    }
}

pub unsafe extern "win64" fn alSourceUnqueueBuffers(source: u32, nb: i32, buffers: *mut u32) {
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    if nb == 0 {
        return;
    }
    if nb < 0 || buffers.is_null() {
        ctx.set_error(AL_INVALID_VALUE);
        return;
    }
    let Some(src) = ctx.sources.get_mut(&source) else {
        ctx.set_error(AL_INVALID_NAME);
        return;
    };
    if src.source_type != AL_STREAMING {
        ctx.set_error(AL_INVALID_VALUE);
        return;
    }
    let count = nb as usize;
    let available = src.buffers_processed() as usize;
    if count > available || count > src.queue.len() {
        ctx.set_error(AL_INVALID_VALUE);
        return;
    }
    for i in 0..count {
        let bid = src.queue.pop_front().unwrap_or(0);
        wr(buffers.add(i), bid);
    }
    src.processed = src.processed.saturating_sub(count);
}

pub unsafe extern "win64" fn alSourcei(source: u32, param: i32, value: i32) {
    source_set_int_prop(source, param, value);
}

pub unsafe extern "win64" fn alSource3i(source: u32, param: i32, v1: i32, v2: i32, v3: i32) {
    source_set_vec3_prop(source, param, v1 as f32, v2 as f32, v3 as f32);
}

pub unsafe extern "win64" fn alSourceiv(source: u32, param: i32, values: *const i32) {
    if values.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    match param {
        AL_POSITION | AL_VELOCITY | AL_DIRECTION => {
            let v1 = rd(values) as f32;
            let v2 = rd(values.add(1)) as f32;
            let v3 = rd(values.add(2)) as f32;
            source_set_vec3_prop(source, param, v1, v2, v3);
        }
        _ => source_set_int_prop(source, param, rd(values)),
    }
}

pub unsafe extern "win64" fn alSourcefv(source: u32, param: i32, values: *const f32) {
    if values.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    match param {
        AL_POSITION | AL_VELOCITY | AL_DIRECTION => {
            let v1 = rd(values);
            let v2 = rd(values.add(1));
            let v3 = rd(values.add(2));
            source_set_vec3_prop(source, param, v1, v2, v3);
        }
        _ => source_set_float_prop(source, param, rd(values)),
    }
}

unsafe extern "win64" fn al_source_f(source: u32, param: i32, value: f32) {
    source_set_float_prop(source, param, value);
}
crate::win64_bridge!(fn al_source_f(u32, i32, f32) -> ());

unsafe extern "win64" fn al_source_3f(source: u32, param: i32, v1: f32, v2: f32, v3: f32) {
    source_set_vec3_prop(source, param, v1, v2, v3);
}
crate::win64_bridge!(fn al_source_3f(u32, i32, f32, f32, f32) -> ());

pub unsafe extern "win64" fn alGetSourcei(source: u32, param: i32, value: *mut i32) {
    if value.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    match get_source_int(source, param) {
        Ok(v) => wr(value, v),
        Err(e) => lock_state().set_al_error(e),
    }
}

pub unsafe extern "win64" fn alGetSource3i(
    source: u32,
    param: i32,
    v1: *mut i32,
    v2: *mut i32,
    v3: *mut i32,
) {
    if v1.is_null() || v2.is_null() || v3.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    match get_source_vec3(source, param) {
        Ok(v) => {
            wr(v1, v[0].round() as i32);
            wr(v2, v[1].round() as i32);
            wr(v3, v[2].round() as i32);
        }
        Err(e) => lock_state().set_al_error(e),
    }
}

pub unsafe extern "win64" fn alGetSourceiv(source: u32, param: i32, values: *mut i32) {
    if values.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    match param {
        AL_POSITION | AL_VELOCITY | AL_DIRECTION => match get_source_vec3(source, param) {
            Ok(v) => {
                wr(values, v[0].round() as i32);
                wr(values.add(1), v[1].round() as i32);
                wr(values.add(2), v[2].round() as i32);
            }
            Err(e) => lock_state().set_al_error(e),
        },
        _ => alGetSourcei(source, param, values),
    }
}

pub unsafe extern "win64" fn alGetSourcef(source: u32, param: i32, value: *mut f32) {
    if value.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    match get_source_float(source, param) {
        Ok(v) => wr(value, v),
        Err(e) => lock_state().set_al_error(e),
    }
}

pub unsafe extern "win64" fn alGetSource3f(
    source: u32,
    param: i32,
    v1: *mut f32,
    v2: *mut f32,
    v3: *mut f32,
) {
    if v1.is_null() || v2.is_null() || v3.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    match get_source_vec3(source, param) {
        Ok(v) => {
            wr(v1, v[0]);
            wr(v2, v[1]);
            wr(v3, v[2]);
        }
        Err(e) => lock_state().set_al_error(e),
    }
}

pub unsafe extern "win64" fn alGetSourcefv(source: u32, param: i32, values: *mut f32) {
    if values.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    match param {
        AL_POSITION | AL_VELOCITY | AL_DIRECTION => match get_source_vec3(source, param) {
            Ok(v) => {
                wr(values, v[0]);
                wr(values.add(1), v[1]);
                wr(values.add(2), v[2]);
            }
            Err(e) => lock_state().set_al_error(e),
        },
        _ => alGetSourcef(source, param, values),
    }
}

// ---------------------------------------------------------------------------
// AL 1.1 Listener API Exports
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn alListeneri(param: i32, value: i32) {
    listener_set_float(param, value as f32);
}

pub unsafe extern "win64" fn alListener3i(param: i32, v1: i32, v2: i32, v3: i32) {
    listener_set_vec3(param, v1 as f32, v2 as f32, v3 as f32);
}

pub unsafe extern "win64" fn alListeneriv(param: i32, values: *const i32) {
    if values.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    match param {
        AL_POSITION | AL_VELOCITY => {
            listener_set_vec3(
                param,
                rd(values) as f32,
                rd(values.add(1)) as f32,
                rd(values.add(2)) as f32,
            );
        }
        AL_ORIENTATION => {
            listener_set_orientation([
                rd(values) as f32,
                rd(values.add(1)) as f32,
                rd(values.add(2)) as f32,
                rd(values.add(3)) as f32,
                rd(values.add(4)) as f32,
                rd(values.add(5)) as f32,
            ]);
        }
        _ => listener_set_float(param, rd(values) as f32),
    }
}

pub unsafe extern "win64" fn alListenerfv(param: i32, values: *const f32) {
    if values.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    match param {
        AL_GAIN => listener_set_float(param, rd(values)),
        AL_POSITION | AL_VELOCITY => {
            listener_set_vec3(param, rd(values), rd(values.add(1)), rd(values.add(2)));
        }
        AL_ORIENTATION => {
            listener_set_orientation([
                rd(values),
                rd(values.add(1)),
                rd(values.add(2)),
                rd(values.add(3)),
                rd(values.add(4)),
                rd(values.add(5)),
            ]);
        }
        _ => {
            let mut st = lock_state();
            if let Some(ctx) = st.current_ctx_mut() {
                ctx.set_error(AL_INVALID_ENUM);
            } else {
                st.set_al_error(AL_INVALID_OPERATION);
            }
        }
    }
}

unsafe extern "win64" fn al_listener_f(param: i32, value: f32) {
    listener_set_float(param, value);
}
crate::win64_bridge!(fn al_listener_f(i32, f32) -> ());

unsafe extern "win64" fn al_listener_3f(param: i32, v1: f32, v2: f32, v3: f32) {
    listener_set_vec3(param, v1, v2, v3);
}
crate::win64_bridge!(fn al_listener_3f(i32, f32, f32, f32) -> ());

pub unsafe extern "win64" fn alGetListenerf(param: i32, value: *mut f32) {
    if value.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    match param {
        AL_GAIN => wr(value, ctx.listener.gain),
        _ => ctx.set_error(AL_INVALID_ENUM),
    }
}

pub unsafe extern "win64" fn alGetListener3f(
    param: i32,
    v1: *mut f32,
    v2: *mut f32,
    v3: *mut f32,
) {
    if v1.is_null() || v2.is_null() || v3.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    let v = match param {
        AL_POSITION => ctx.listener.position,
        AL_VELOCITY => ctx.listener.velocity,
        _ => {
            ctx.set_error(AL_INVALID_ENUM);
            return;
        }
    };
    wr(v1, v[0]);
    wr(v2, v[1]);
    wr(v3, v[2]);
}

pub unsafe extern "win64" fn alGetListenerfv(param: i32, values: *mut f32) {
    if values.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    match param {
        AL_GAIN => wr(values, ctx.listener.gain),
        AL_POSITION => {
            for (i, &x) in ctx.listener.position.iter().enumerate() {
                wr(values.add(i), x);
            }
        }
        AL_VELOCITY => {
            for (i, &x) in ctx.listener.velocity.iter().enumerate() {
                wr(values.add(i), x);
            }
        }
        AL_ORIENTATION => {
            for (i, &x) in ctx.listener.orientation.iter().enumerate() {
                wr(values.add(i), x);
            }
        }
        _ => ctx.set_error(AL_INVALID_ENUM),
    }
}

pub unsafe extern "win64" fn alGetListeneri(param: i32, value: *mut i32) {
    if value.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    match param {
        AL_GAIN => wr(value, ctx.listener.gain.round() as i32),
        _ => ctx.set_error(AL_INVALID_ENUM),
    }
}

pub unsafe extern "win64" fn alGetListener3i(
    param: i32,
    v1: *mut i32,
    v2: *mut i32,
    v3: *mut i32,
) {
    if v1.is_null() || v2.is_null() || v3.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    let v = match param {
        AL_POSITION => ctx.listener.position,
        AL_VELOCITY => ctx.listener.velocity,
        _ => {
            ctx.set_error(AL_INVALID_ENUM);
            return;
        }
    };
    wr(v1, v[0].round() as i32);
    wr(v2, v[1].round() as i32);
    wr(v3, v[2].round() as i32);
}

pub unsafe extern "win64" fn alGetListeneriv(param: i32, values: *mut i32) {
    if values.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    match param {
        AL_GAIN => wr(values, ctx.listener.gain.round() as i32),
        AL_POSITION => {
            for (i, &x) in ctx.listener.position.iter().enumerate() {
                wr(values.add(i), x.round() as i32);
            }
        }
        AL_VELOCITY => {
            for (i, &x) in ctx.listener.velocity.iter().enumerate() {
                wr(values.add(i), x.round() as i32);
            }
        }
        AL_ORIENTATION => {
            for (i, &x) in ctx.listener.orientation.iter().enumerate() {
                wr(values.add(i), x.round() as i32);
            }
        }
        _ => ctx.set_error(AL_INVALID_ENUM),
    }
}

// ---------------------------------------------------------------------------
// AL 1.1 Global State, Distance Model, Errors & Introspection Exports
// ---------------------------------------------------------------------------

pub unsafe extern "win64" fn alDistanceModel(value: i32) {
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    match value {
        AL_NONE
        | AL_INVERSE_DISTANCE
        | AL_INVERSE_DISTANCE_CLAMPED
        | AL_LINEAR_DISTANCE
        | AL_LINEAR_DISTANCE_CLAMPED
        | AL_EXPONENT_DISTANCE
        | AL_EXPONENT_DISTANCE_CLAMPED => ctx.distance_model = value,
        _ => ctx.set_error(AL_INVALID_VALUE),
    }
}

pub unsafe extern "win64" fn alDopplerFactor(value: f32) {
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    if !value.is_finite() || value < 0.0 {
        ctx.set_error(AL_INVALID_VALUE);
    } else {
        ctx.doppler_factor = value;
    }
}

pub unsafe extern "win64" fn alDopplerVelocity(value: f32) {
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    if !value.is_finite() || value <= 0.0 {
        ctx.set_error(AL_INVALID_VALUE);
    } else {
        ctx.doppler_velocity = value;
    }
}

pub unsafe extern "win64" fn alSpeedOfSound(value: f32) {
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    if !value.is_finite() || value <= 0.0 {
        ctx.set_error(AL_INVALID_VALUE);
    } else {
        ctx.speed_of_sound = value;
    }
}

pub unsafe extern "win64" fn alGetError() -> i32 {
    let mut st = lock_state();
    if let Some(cid) = st.current_context {
        if let Some(ctx) = st.contexts.get_mut(&cid) {
            let e = ctx.last_error;
            ctx.last_error = AL_NO_ERROR;
            return e;
        }
    }
    let e = st.global_al_error;
    st.global_al_error = AL_NO_ERROR;
    e
}

pub unsafe extern "win64" fn alGetString(param: i32) -> *const u8 {
    match param {
        AL_NO_ERROR => b"No Error\0".as_ptr(),
        AL_INVALID_NAME => b"Invalid Name\0".as_ptr(),
        AL_INVALID_ENUM => b"Invalid Enum\0".as_ptr(),
        AL_INVALID_VALUE => b"Invalid Value\0".as_ptr(),
        AL_INVALID_OPERATION => b"Invalid Operation\0".as_ptr(),
        AL_OUT_OF_MEMORY => b"Out of Memory\0".as_ptr(),
        AL_VENDOR => b"PortFactory OpenAL\0".as_ptr(),
        AL_VERSION => b"1.1 CoreAudio\0".as_ptr(),
        AL_RENDERER => b"CoreAudio Software Mixer\0".as_ptr(),
        AL_EXTENSIONS => b"AL_EXT_FLOAT32\0".as_ptr(),
        _ => {
            lock_state().set_al_error(AL_INVALID_ENUM);
            std::ptr::null()
        }
    }
}

pub unsafe extern "win64" fn alGetBoolean(param: i32) -> u8 {
    match get_global_param_f64(param) {
        Ok(v) => {
            if v != 0.0 {
                AL_TRUE
            } else {
                AL_FALSE
            }
        }
        Err(e) => {
            lock_state().set_al_error(e);
            AL_FALSE
        }
    }
}

pub unsafe extern "win64" fn alGetInteger(param: i32) -> i32 {
    match get_global_param_f64(param) {
        Ok(v) => v as i32,
        Err(e) => {
            lock_state().set_al_error(e);
            0
        }
    }
}

pub unsafe extern "win64" fn alGetFloat(param: i32) -> f32 {
    match get_global_param_f64(param) {
        Ok(v) => v as f32,
        Err(e) => {
            lock_state().set_al_error(e);
            0.0
        }
    }
}

pub unsafe extern "win64" fn alGetDouble(param: i32) -> f64 {
    match get_global_param_f64(param) {
        Ok(v) => v,
        Err(e) => {
            lock_state().set_al_error(e);
            0.0
        }
    }
}

pub unsafe extern "win64" fn alGetBooleanv(param: i32, values: *mut u8) {
    if values.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    match get_global_param_f64(param) {
        Ok(v) => wr(values, if v != 0.0 { AL_TRUE } else { AL_FALSE }),
        Err(e) => lock_state().set_al_error(e),
    }
}

pub unsafe extern "win64" fn alGetIntegerv(param: i32, values: *mut i32) {
    if values.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    match get_global_param_f64(param) {
        Ok(v) => wr(values, v as i32),
        Err(e) => lock_state().set_al_error(e),
    }
}

pub unsafe extern "win64" fn alGetFloatv(param: i32, values: *mut f32) {
    if values.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    match get_global_param_f64(param) {
        Ok(v) => wr(values, v as f32),
        Err(e) => lock_state().set_al_error(e),
    }
}

pub unsafe extern "win64" fn alGetDoublev(param: i32, values: *mut f64) {
    if values.is_null() {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return;
    }
    match get_global_param_f64(param) {
        Ok(v) => wr(values, v),
        Err(e) => lock_state().set_al_error(e),
    }
}

pub unsafe extern "win64" fn alEnable(capability: i32) {
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    match capability {
        AL_SOURCE_DISTANCE_MODEL => ctx.source_distance_model_enabled = true,
        _ => ctx.set_error(AL_INVALID_ENUM),
    }
}

pub unsafe extern "win64" fn alDisable(capability: i32) {
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return;
    };
    match capability {
        AL_SOURCE_DISTANCE_MODEL => ctx.source_distance_model_enabled = false,
        _ => ctx.set_error(AL_INVALID_ENUM),
    }
}

pub unsafe extern "win64" fn alIsEnabled(capability: i32) -> u8 {
    let mut st = lock_state();
    let Some(ctx) = st.current_ctx_mut() else {
        st.set_al_error(AL_INVALID_OPERATION);
        return AL_FALSE;
    };
    match capability {
        AL_SOURCE_DISTANCE_MODEL => {
            if ctx.source_distance_model_enabled {
                AL_TRUE
            } else {
                AL_FALSE
            }
        }
        _ => {
            ctx.set_error(AL_INVALID_ENUM);
            AL_FALSE
        }
    }
}

pub unsafe extern "win64" fn alIsExtensionPresent(extname: *const u8) -> u8 {
    let Some(name) = crate::strings::from_ansi_opt(extname) else {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return AL_FALSE;
    };
    if name.eq_ignore_ascii_case("AL_EXT_FLOAT32") {
        AL_TRUE
    } else {
        AL_FALSE
    }
}

pub unsafe extern "win64" fn alGetProcAddress(fname: *const u8) -> *const c_void {
    let Some(name) = crate::strings::from_ansi_opt(fname) else {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return std::ptr::null();
    };
    if let Some(ptr) = crate::lookup("openal32.dll", &name) {
        let addr = ptr as usize;
        crate::register_native_va(addr as u64, addr);
        ptr as *const c_void
    } else {
        std::ptr::null()
    }
}

pub unsafe extern "win64" fn alGetEnumValue(ename: *const u8) -> i32 {
    let Some(name) = crate::strings::from_ansi_opt(ename) else {
        lock_state().set_al_error(AL_INVALID_VALUE);
        return AL_NONE;
    };
    match name.as_str() {
        "AL_NONE" => AL_NONE,
        "AL_FALSE" => AL_FALSE as i32,
        "AL_TRUE" => AL_TRUE as i32,
        "AL_SOURCE_RELATIVE" => AL_SOURCE_RELATIVE,
        "AL_CONE_INNER_ANGLE" => AL_CONE_INNER_ANGLE,
        "AL_CONE_OUTER_ANGLE" => AL_CONE_OUTER_ANGLE,
        "AL_PITCH" => AL_PITCH,
        "AL_POSITION" => AL_POSITION,
        "AL_DIRECTION" => AL_DIRECTION,
        "AL_VELOCITY" => AL_VELOCITY,
        "AL_LOOPING" => AL_LOOPING,
        "AL_BUFFER" => AL_BUFFER,
        "AL_GAIN" => AL_GAIN,
        "AL_MIN_GAIN" => AL_MIN_GAIN,
        "AL_MAX_GAIN" => AL_MAX_GAIN,
        "AL_ORIENTATION" => AL_ORIENTATION,
        "AL_SOURCE_STATE" => AL_SOURCE_STATE,
        "AL_INITIAL" => AL_INITIAL,
        "AL_PLAYING" => AL_PLAYING,
        "AL_PAUSED" => AL_PAUSED,
        "AL_STOPPED" => AL_STOPPED,
        "AL_BUFFERS_QUEUED" => AL_BUFFERS_QUEUED,
        "AL_BUFFERS_PROCESSED" => AL_BUFFERS_PROCESSED,
        "AL_REFERENCE_DISTANCE" => AL_REFERENCE_DISTANCE,
        "AL_ROLLOFF_FACTOR" => AL_ROLLOFF_FACTOR,
        "AL_CONE_OUTER_GAIN" => AL_CONE_OUTER_GAIN,
        "AL_MAX_DISTANCE" => AL_MAX_DISTANCE,
        "AL_SEC_OFFSET" => AL_SEC_OFFSET,
        "AL_SAMPLE_OFFSET" => AL_SAMPLE_OFFSET,
        "AL_BYTE_OFFSET" => AL_BYTE_OFFSET,
        "AL_SOURCE_TYPE" => AL_SOURCE_TYPE,
        "AL_STATIC" => AL_STATIC,
        "AL_STREAMING" => AL_STREAMING,
        "AL_UNDETERMINED" => AL_UNDETERMINED,
        "AL_FORMAT_MONO8" => AL_FORMAT_MONO8,
        "AL_FORMAT_MONO16" => AL_FORMAT_MONO16,
        "AL_FORMAT_STEREO8" => AL_FORMAT_STEREO8,
        "AL_FORMAT_STEREO16" => AL_FORMAT_STEREO16,
        "AL_FORMAT_MONO_FLOAT32" => AL_FORMAT_MONO_FLOAT32,
        "AL_FORMAT_STEREO_FLOAT32" => AL_FORMAT_STEREO_FLOAT32,
        "AL_FREQUENCY" => AL_FREQUENCY,
        "AL_BITS" => AL_BITS,
        "AL_CHANNELS" => AL_CHANNELS,
        "AL_SIZE" => AL_SIZE,
        "AL_NO_ERROR" => AL_NO_ERROR,
        "AL_INVALID_NAME" => AL_INVALID_NAME,
        "AL_INVALID_ENUM" => AL_INVALID_ENUM,
        "AL_INVALID_VALUE" => AL_INVALID_VALUE,
        "AL_INVALID_OPERATION" => AL_INVALID_OPERATION,
        "AL_OUT_OF_MEMORY" => AL_OUT_OF_MEMORY,
        "AL_VENDOR" => AL_VENDOR,
        "AL_VERSION" => AL_VERSION,
        "AL_RENDERER" => AL_RENDERER,
        "AL_EXTENSIONS" => AL_EXTENSIONS,
        "AL_DOPPLER_FACTOR" => AL_DOPPLER_FACTOR,
        "AL_DOPPLER_VELOCITY" => AL_DOPPLER_VELOCITY,
        "AL_SPEED_OF_SOUND" => AL_SPEED_OF_SOUND,
        "AL_DISTANCE_MODEL" => AL_DISTANCE_MODEL,
        "AL_INVERSE_DISTANCE" => AL_INVERSE_DISTANCE,
        "AL_INVERSE_DISTANCE_CLAMPED" => AL_INVERSE_DISTANCE_CLAMPED,
        "AL_LINEAR_DISTANCE" => AL_LINEAR_DISTANCE,
        "AL_LINEAR_DISTANCE_CLAMPED" => AL_LINEAR_DISTANCE_CLAMPED,
        "AL_EXPONENT_DISTANCE" => AL_EXPONENT_DISTANCE,
        "AL_EXPONENT_DISTANCE_CLAMPED" => AL_EXPONENT_DISTANCE_CLAMPED,
        "AL_SOURCE_DISTANCE_MODEL" => AL_SOURCE_DISTANCE_MODEL,
        _ => AL_NONE,
    }
}

#[cfg(test)]
mod tests;
