//! Builtin `fmod.dll` / `fmod64.dll` / `fmodstudio.dll` / `fmodstudio64.dll`
//! (FMOD Core and FMOD Studio) audio middleware backed by `crate::audio::backend`
//! (CoreAudio AUHAL on macOS, RemoteIO on iOS, or Null when `AW2_AUDIO_BACKEND=null`).
//!
//! (OpenAL lives in `openal32.rs`.)

#![allow(non_snake_case)]

mod bank;
mod core;
mod dsp;
mod mixer;
mod studio;

pub use self::core::{
    FMOD_2D, FMOD_3D, FMOD_3D_HEADRELATIVE, FMOD_3D_INVERSEROLLOFF, FMOD_3D_LINEARROLLOFF,
    FMOD_3D_LINEARSQUAREROLLOFF, FMOD_3D_WORLDRELATIVE, FMOD_CREATESTREAM, FMOD_DEFAULT,
    FMOD_ERR_ALREADY_LOCKED, FMOD_ERR_EVENT_ALREADY_LOADED, FMOD_ERR_EVENT_NOTFOUND,
    FMOD_ERR_FILE_BAD, FMOD_ERR_FILE_EOF, FMOD_ERR_FILE_NOTFOUND, FMOD_ERR_FORMAT,
    FMOD_ERR_INITIALIZED, FMOD_ERR_INVALID_FLOAT, FMOD_ERR_INVALID_HANDLE, FMOD_ERR_INVALID_PARAM,
    FMOD_ERR_INVALID_POSITION, FMOD_ERR_INVALID_STRING, FMOD_ERR_NOT_LOCKED, FMOD_ERR_OUTPUT_INIT,
    FMOD_ERR_STUDIO_NOT_LOADED, FMOD_ERR_STUDIO_UNINITIALIZED, FMOD_ERR_TOOMANYCHANNELS,
    FMOD_ERR_TRUNCATED, FMOD_ERR_UNINITIALIZED, FMOD_LOOP_BIDI, FMOD_LOOP_NORMAL, FMOD_LOOP_OFF,
    FMOD_OK, FMOD_OPENMEMORY, FMOD_OPENMEMORY_POINT, FMOD_OPENRAW, FMOD_OPENUSER,
    FMOD_OUTPUTTYPE_AUTODETECT, FMOD_OUTPUTTYPE_COREAUDIO, FMOD_OUTPUTTYPE_NOSOUND,
    FMOD_OUTPUTTYPE_NOSOUND_NRT, FMOD_OUTPUTTYPE_UNKNOWN, FMOD_OUTPUTTYPE_WASAPI,
    FMOD_OUTPUTTYPE_WAVWRITER, FMOD_OUTPUTTYPE_WAVWRITER_NRT, FMOD_SOUND_FORMAT_NONE,
    FMOD_SOUND_FORMAT_PCM8, FMOD_SOUND_FORMAT_PCM16, FMOD_SOUND_FORMAT_PCM24,
    FMOD_SOUND_FORMAT_PCM32, FMOD_SOUND_FORMAT_PCMFLOAT, FMOD_SOUND_TYPE_FSB, FMOD_SOUND_TYPE_RAW,
    FMOD_SOUND_TYPE_UNKNOWN, FMOD_SOUND_TYPE_USER, FMOD_SOUND_TYPE_WAV, FMOD_SPEAKERMODE_5POINT1,
    FMOD_SPEAKERMODE_7POINT1, FMOD_SPEAKERMODE_DEFAULT, FMOD_SPEAKERMODE_MONO,
    FMOD_SPEAKERMODE_QUAD, FMOD_SPEAKERMODE_RAW, FMOD_SPEAKERMODE_STEREO,
    FMOD_SPEAKERMODE_SURROUND, FMOD_STUDIO_EVENT_PROPERTY_CHANNELPRIORITY,
    FMOD_STUDIO_EVENT_PROPERTY_COOLDOWN, FMOD_STUDIO_EVENT_PROPERTY_MAX,
    FMOD_STUDIO_EVENT_PROPERTY_MAXIMUM_DISTANCE, FMOD_STUDIO_EVENT_PROPERTY_MINIMUM_DISTANCE,
    FMOD_STUDIO_EVENT_PROPERTY_SCHEDULE_DELAY, FMOD_STUDIO_EVENT_PROPERTY_SCHEDULE_LOOKAHEAD,
    FMOD_STUDIO_LOAD_BANK_DECOMPRESS_SAMPLES, FMOD_STUDIO_LOAD_BANK_NONBLOCKING,
    FMOD_STUDIO_LOAD_BANK_NORMAL, FMOD_STUDIO_LOAD_BANK_UNENCRYPTED, FMOD_STUDIO_LOAD_MEMORY,
    FMOD_STUDIO_LOAD_MEMORY_POINT, FMOD_STUDIO_LOADING_STATE_ERROR,
    FMOD_STUDIO_LOADING_STATE_LOADED, FMOD_STUDIO_LOADING_STATE_LOADING,
    FMOD_STUDIO_LOADING_STATE_UNLOADED, FMOD_STUDIO_LOADING_STATE_UNLOADING,
    FMOD_STUDIO_PARAMETER_AUTOMATIC, FMOD_STUDIO_PARAMETER_AUTOMATIC_DISTANCE,
    FMOD_STUDIO_PARAMETER_DISCRETE, FMOD_STUDIO_PARAMETER_GAME_CONTROLLED,
    FMOD_STUDIO_PARAMETER_GLOBAL, FMOD_STUDIO_PARAMETER_LABELED, FMOD_STUDIO_PARAMETER_READONLY,
    FMOD_STUDIO_PLAYBACK_PLAYING, FMOD_STUDIO_PLAYBACK_STARTING, FMOD_STUDIO_PLAYBACK_STOPPED,
    FMOD_STUDIO_PLAYBACK_STOPPING, FMOD_STUDIO_PLAYBACK_SUSTAINING,
    FMOD_STUDIO_STOP_ALLOWFADEOUT, FMOD_STUDIO_STOP_IMMEDIATE, FMOD_TIMEUNIT_MS,
    FMOD_TIMEUNIT_PCM, FMOD_TIMEUNIT_PCMBYTES, FMOD_TIMEUNIT_RAWBYTES, FMOD_VERSION,
    Fmod3DAttributes, FmodCreateSoundExInfoHeader, FmodGuid, FmodStudioParameterDescription,
    FmodStudioParameterId, FmodVector, mix_system_frames,
};

pub static EXTRA_EXPORTS: &[crate::Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

/// Emit paired Win64 (`not(aarch64)`) and AAPCS64-register-padded (`aarch64`)
/// `pub unsafe extern "win64" fn` shims for FMOD exports taking `f32` arguments.
///
/// Every generated symbol MUST be listed in a `// @exports ...` comment so
/// `win32-rt/build.rs` and `port-factory/src/win32_db.rs` discover it identically.
macro_rules! fmod_float_export {
    (
        $name:ident -> $impl_fn:path ;
        win64( $($w_arg:ident : $w_ty:ty),* $(,)? ) ;
        aarch64( $($a_arg:ident : $a_ty:ty),* $(,)? ) ;
        call( $($c_arg:ident),* $(,)? )
    ) => {
        #[cfg(not(target_arch = "aarch64"))]
        pub unsafe extern "win64" fn $name($($w_arg: $w_ty),*) -> i32 {
            $impl_fn($($c_arg),*)
        }

        #[cfg(target_arch = "aarch64")]
        pub unsafe extern "win64" fn $name($($a_arg: $a_ty),*) -> i32 {
            $impl_fn($($c_arg),*)
        }
    };
}

// ============================================================================
// FMOD Core (`fmod.dll` / `fmod64.dll`) — System
// ============================================================================

pub unsafe extern "win64" fn FMOD_System_Create(
    system_out: *mut usize,
    header_version: u32,
) -> i32 {
    self::core::system_create(system_out, header_version)
}

pub unsafe extern "win64" fn FMOD_System_Init(
    system: usize,
    maxchannels: i32,
    flags: u32,
    extradriverdata: usize,
) -> i32 {
    self::core::system_init_impl(system, maxchannels, flags, extradriverdata)
}

pub unsafe extern "win64" fn FMOD_System_Close(system: usize) -> i32 {
    self::core::system_close_impl(system)
}

pub unsafe extern "win64" fn FMOD_System_Release(system: usize) -> i32 {
    self::core::system_release_impl(system)
}

pub unsafe extern "win64" fn FMOD_System_Update(system: usize) -> i32 {
    self::core::system_update_impl(system)
}

pub unsafe extern "win64" fn FMOD_System_GetVersion(
    system: usize,
    version_out: *mut u32,
) -> i32 {
    self::core::system_get_version(system, version_out)
}

pub unsafe extern "win64" fn FMOD_System_SetOutput(system: usize, output: u32) -> i32 {
    self::core::system_set_output(system, output)
}

pub unsafe extern "win64" fn FMOD_System_GetOutput(
    system: usize,
    output_out: *mut u32,
) -> i32 {
    self::core::system_get_output(system, output_out)
}

pub unsafe extern "win64" fn FMOD_System_GetNumDrivers(
    system: usize,
    numdrivers_out: *mut i32,
) -> i32 {
    self::core::system_get_num_drivers(system, numdrivers_out)
}

pub unsafe extern "win64" fn FMOD_System_GetDriverInfo(
    system: usize,
    id: i32,
    name_out: *mut u8,
    namelen: i32,
    guid: usize,
    systemrate: *mut i32,
    speakermode: *mut u32,
    speakermodechannels: *mut i32,
) -> i32 {
    self::core::system_get_driver_info(
        system,
        id,
        name_out,
        namelen,
        guid,
        systemrate,
        speakermode,
        speakermodechannels,
    )
}

pub unsafe extern "win64" fn FMOD_System_SetDriver(system: usize, driver: i32) -> i32 {
    self::core::system_set_driver(system, driver)
}

pub unsafe extern "win64" fn FMOD_System_GetDriver(
    system: usize,
    driver_out: *mut i32,
) -> i32 {
    self::core::system_get_driver(system, driver_out)
}

pub unsafe extern "win64" fn FMOD_System_GetMasterChannelGroup(
    system: usize,
    group_out: *mut usize,
) -> i32 {
    self::core::system_get_master_channel_group(system, group_out)
}

pub unsafe extern "win64" fn FMOD_System_CreateChannelGroup(
    system: usize,
    name: *const u8,
    group_out: *mut usize,
) -> i32 {
    self::core::system_create_channel_group(system, name, group_out)
}

pub unsafe extern "win64" fn FMOD_System_Set3DListenerAttributes(
    system: usize,
    listener: i32,
    pos: *const FmodVector,
    vel: *const FmodVector,
    forward: *const FmodVector,
    up: *const FmodVector,
) -> i32 {
    self::core::system_set_3d_listener_attributes(system, listener, pos, vel, forward, up)
}

pub unsafe extern "win64" fn FMOD_System_Get3DListenerAttributes(
    system: usize,
    listener: i32,
    pos: *mut FmodVector,
    vel: *mut FmodVector,
    forward: *mut FmodVector,
    up: *mut FmodVector,
) -> i32 {
    self::core::system_get_3d_listener_attributes(system, listener, pos, vel, forward, up)
}

// @exports FMOD_System_Set3DSettings
fmod_float_export!(
    FMOD_System_Set3DSettings -> self::core::system_set_3d_settings_impl;
    win64(system: usize, dopplerscale: f32, distancefactor: f32, rolloffscale: f32);
    aarch64(system: usize, _xmm0: f32, dopplerscale: f32, distancefactor: f32, rolloffscale: f32);
    call(system, dopplerscale, distancefactor, rolloffscale)
);

pub unsafe extern "win64" fn FMOD_System_Get3DSettings(
    system: usize,
    dopplerscale: *mut f32,
    distancefactor: *mut f32,
    rolloffscale: *mut f32,
) -> i32 {
    self::core::system_get_3d_settings(system, dopplerscale, distancefactor, rolloffscale)
}

// ============================================================================
// FMOD Core — Sound creation & Sound methods
// ============================================================================

pub unsafe extern "win64" fn FMOD_System_CreateSound(
    system: usize,
    name_or_data: *const u8,
    mode: u32,
    exinfo: usize,
    sound_out: *mut usize,
) -> i32 {
    self::core::system_create_sound(system, name_or_data, mode, exinfo, sound_out)
}

pub unsafe extern "win64" fn FMOD_System_CreateStream(
    system: usize,
    name_or_data: *const u8,
    mode: u32,
    exinfo: usize,
    sound_out: *mut usize,
) -> i32 {
    self::core::system_create_sound(system, name_or_data, mode | FMOD_CREATESTREAM, exinfo, sound_out)
}

pub unsafe extern "win64" fn FMOD_Sound_Release(sound: usize) -> i32 {
    self::core::sound_release(sound)
}

pub unsafe extern "win64" fn FMOD_Sound_GetNumSubSounds(
    sound: usize,
    numsubsounds_out: *mut i32,
) -> i32 {
    self::core::sound_get_num_sub_sounds(sound, numsubsounds_out)
}

pub unsafe extern "win64" fn FMOD_Sound_GetSubSound(
    sound: usize,
    index: i32,
    subsound_out: *mut usize,
) -> i32 {
    self::core::sound_get_sub_sound(sound, index, subsound_out)
}

pub unsafe extern "win64" fn FMOD_Sound_GetName(
    sound: usize,
    name_out: *mut u8,
    namelen: i32,
) -> i32 {
    self::core::sound_get_name(sound, name_out, namelen)
}

pub unsafe extern "win64" fn FMOD_Sound_GetLength(
    sound: usize,
    length_out: *mut u32,
    lengthtype: u32,
) -> i32 {
    self::core::sound_get_length(sound, length_out, lengthtype)
}

pub unsafe extern "win64" fn FMOD_Sound_GetFormat(
    sound: usize,
    sound_type_out: *mut u32,
    format_out: *mut u32,
    channels_out: *mut i32,
    bits_out: *mut i32,
) -> i32 {
    self::core::sound_get_format(sound, sound_type_out, format_out, channels_out, bits_out)
}

pub unsafe extern "win64" fn FMOD_Sound_GetDefaults(
    sound: usize,
    frequency_out: *mut f32,
    priority_out: *mut i32,
) -> i32 {
    self::core::sound_get_defaults(sound, frequency_out, priority_out)
}

// @exports FMOD_Sound_SetDefaults
fmod_float_export!(
    FMOD_Sound_SetDefaults -> self::core::sound_set_defaults_impl;
    win64(sound: usize, frequency: f32, priority: i32);
    aarch64(sound: usize, _rdx: u64, priority: i32, _xmm0: f32, frequency: f32);
    call(sound, frequency, priority)
);

pub unsafe extern "win64" fn FMOD_Sound_SetMode(sound: usize, mode: u32) -> i32 {
    self::core::sound_set_mode(sound, mode)
}

pub unsafe extern "win64" fn FMOD_Sound_GetMode(sound: usize, mode_out: *mut u32) -> i32 {
    self::core::sound_get_mode(sound, mode_out)
}

pub unsafe extern "win64" fn FMOD_Sound_SetLoopCount(sound: usize, loopcount: i32) -> i32 {
    self::core::sound_set_loop_count(sound, loopcount)
}

pub unsafe extern "win64" fn FMOD_Sound_GetLoopCount(
    sound: usize,
    loopcount_out: *mut i32,
) -> i32 {
    self::core::sound_get_loop_count(sound, loopcount_out)
}

// @exports FMOD_Sound_Set3DMinMaxDistance
fmod_float_export!(
    FMOD_Sound_Set3DMinMaxDistance -> self::core::sound_set_3d_min_max_distance_impl;
    win64(sound: usize, min: f32, max: f32);
    aarch64(sound: usize, _xmm0: f32, min: f32, max: f32);
    call(sound, min, max)
);

pub unsafe extern "win64" fn FMOD_Sound_Get3DMinMaxDistance(
    sound: usize,
    min_out: *mut f32,
    max_out: *mut f32,
) -> i32 {
    self::core::sound_get_3d_min_max_distance(sound, min_out, max_out)
}

pub unsafe extern "win64" fn FMOD_Sound_Lock(
    sound: usize,
    offset: u32,
    length: u32,
    ptr1: *mut *mut u8,
    ptr2: *mut *mut u8,
    len1: *mut u32,
    len2: *mut u32,
) -> i32 {
    self::core::sound_lock(sound, offset, length, ptr1, ptr2, len1, len2)
}

pub unsafe extern "win64" fn FMOD_Sound_Unlock(
    sound: usize,
    ptr1: *mut u8,
    ptr2: *mut u8,
    len1: u32,
    len2: u32,
) -> i32 {
    self::core::sound_unlock(sound, ptr1, ptr2, len1, len2)
}

// ============================================================================
// FMOD Core — Playback & Channel methods
// ============================================================================

pub unsafe extern "win64" fn FMOD_System_PlaySound(
    system: usize,
    sound: usize,
    channelgroup: usize,
    paused: i32,
    channel_out: *mut usize,
) -> i32 {
    self::core::system_play_sound(system, sound, channelgroup, paused, channel_out)
}

pub unsafe extern "win64" fn FMOD_Channel_SetPaused(channel: usize, paused: i32) -> i32 {
    self::core::channel_set_paused(channel, paused)
}

pub unsafe extern "win64" fn FMOD_Channel_GetPaused(
    channel: usize,
    paused_out: *mut i32,
) -> i32 {
    self::core::channel_get_paused(channel, paused_out)
}

// @exports FMOD_Channel_SetVolume
fmod_float_export!(
    FMOD_Channel_SetVolume -> self::core::channel_set_volume_impl;
    win64(channel: usize, volume: f32);
    aarch64(channel: usize, _xmm0: f32, volume: f32);
    call(channel, volume)
);

pub unsafe extern "win64" fn FMOD_Channel_GetVolume(
    channel: usize,
    volume_out: *mut f32,
) -> i32 {
    self::core::channel_get_volume(channel, volume_out)
}

// @exports FMOD_Channel_SetPitch
fmod_float_export!(
    FMOD_Channel_SetPitch -> self::core::channel_set_pitch_impl;
    win64(channel: usize, pitch: f32);
    aarch64(channel: usize, _xmm0: f32, pitch: f32);
    call(channel, pitch)
);

pub unsafe extern "win64" fn FMOD_Channel_GetPitch(
    channel: usize,
    pitch_out: *mut f32,
) -> i32 {
    self::core::channel_get_pitch(channel, pitch_out)
}

// @exports FMOD_Channel_SetPan
fmod_float_export!(
    FMOD_Channel_SetPan -> self::core::channel_set_pan_impl;
    win64(channel: usize, pan: f32);
    aarch64(channel: usize, _xmm0: f32, pan: f32);
    call(channel, pan)
);

pub unsafe extern "win64" fn FMOD_Channel_SetMute(channel: usize, mute: i32) -> i32 {
    self::core::channel_set_mute(channel, mute)
}

pub unsafe extern "win64" fn FMOD_Channel_GetMute(channel: usize, mute_out: *mut i32) -> i32 {
    self::core::channel_get_mute(channel, mute_out)
}

pub unsafe extern "win64" fn FMOD_Channel_SetMode(channel: usize, mode: u32) -> i32 {
    self::core::channel_set_mode(channel, mode)
}

pub unsafe extern "win64" fn FMOD_Channel_GetMode(channel: usize, mode_out: *mut u32) -> i32 {
    self::core::channel_get_mode(channel, mode_out)
}

pub unsafe extern "win64" fn FMOD_Channel_Set3DAttributes(
    channel: usize,
    pos: *const FmodVector,
    vel: *const FmodVector,
) -> i32 {
    self::core::channel_set_3d_attributes(channel, pos, vel)
}

pub unsafe extern "win64" fn FMOD_Channel_Get3DAttributes(
    channel: usize,
    pos_out: *mut FmodVector,
    vel_out: *mut FmodVector,
) -> i32 {
    self::core::channel_get_3d_attributes(channel, pos_out, vel_out)
}

// @exports FMOD_Channel_Set3DMinMaxDistance
fmod_float_export!(
    FMOD_Channel_Set3DMinMaxDistance -> self::core::channel_set_3d_min_max_distance_impl;
    win64(channel: usize, mindistance: f32, maxdistance: f32);
    aarch64(channel: usize, _xmm0: f32, mindistance: f32, maxdistance: f32);
    call(channel, mindistance, maxdistance)
);

pub unsafe extern "win64" fn FMOD_Channel_Get3DMinMaxDistance(
    channel: usize,
    mindistance_out: *mut f32,
    maxdistance_out: *mut f32,
) -> i32 {
    self::core::channel_get_3d_min_max_distance(channel, mindistance_out, maxdistance_out)
}

// @exports FMOD_Channel_SetFrequency
fmod_float_export!(
    FMOD_Channel_SetFrequency -> self::core::channel_set_frequency_impl;
    win64(channel: usize, frequency: f32);
    aarch64(channel: usize, _xmm0: f32, frequency: f32);
    call(channel, frequency)
);

pub unsafe extern "win64" fn FMOD_Channel_GetFrequency(
    channel: usize,
    frequency_out: *mut f32,
) -> i32 {
    self::core::channel_get_frequency(channel, frequency_out)
}

pub unsafe extern "win64" fn FMOD_Channel_SetLoopCount(channel: usize, loopcount: i32) -> i32 {
    self::core::channel_set_loop_count(channel, loopcount)
}

pub unsafe extern "win64" fn FMOD_Channel_GetLoopCount(
    channel: usize,
    loopcount_out: *mut i32,
) -> i32 {
    self::core::channel_get_loop_count(channel, loopcount_out)
}

pub unsafe extern "win64" fn FMOD_Channel_SetPosition(
    channel: usize,
    position: u32,
    postype: u32,
) -> i32 {
    self::core::channel_set_position(channel, position, postype)
}

pub unsafe extern "win64" fn FMOD_Channel_GetPosition(
    channel: usize,
    position_out: *mut u32,
    postype: u32,
) -> i32 {
    self::core::channel_get_position(channel, position_out, postype)
}

pub unsafe extern "win64" fn FMOD_Channel_IsPlaying(
    channel: usize,
    isplaying_out: *mut i32,
) -> i32 {
    self::core::channel_is_playing(channel, isplaying_out)
}

pub unsafe extern "win64" fn FMOD_Channel_Stop(channel: usize) -> i32 {
    self::core::channel_stop(channel)
}

pub unsafe extern "win64" fn FMOD_Channel_SetChannelGroup(
    channel: usize,
    channelgroup: usize,
) -> i32 {
    self::core::channel_set_channel_group(channel, channelgroup)
}

pub unsafe extern "win64" fn FMOD_Channel_GetChannelGroup(
    channel: usize,
    channelgroup_out: *mut usize,
) -> i32 {
    self::core::channel_get_channel_group(channel, channelgroup_out)
}

// ============================================================================
// FMOD Core — ChannelGroup methods
// ============================================================================

// @exports FMOD_ChannelGroup_SetVolume
fmod_float_export!(
    FMOD_ChannelGroup_SetVolume -> self::core::channel_group_set_volume_impl;
    win64(channelgroup: usize, volume: f32);
    aarch64(channelgroup: usize, _xmm0: f32, volume: f32);
    call(channelgroup, volume)
);

pub unsafe extern "win64" fn FMOD_ChannelGroup_GetVolume(
    channelgroup: usize,
    volume_out: *mut f32,
) -> i32 {
    self::core::channel_group_get_volume(channelgroup, volume_out)
}

pub unsafe extern "win64" fn FMOD_ChannelGroup_SetMute(channelgroup: usize, mute: i32) -> i32 {
    self::core::channel_group_set_mute(channelgroup, mute)
}

pub unsafe extern "win64" fn FMOD_ChannelGroup_GetMute(
    channelgroup: usize,
    mute_out: *mut i32,
) -> i32 {
    self::core::channel_group_get_mute(channelgroup, mute_out)
}

pub unsafe extern "win64" fn FMOD_ChannelGroup_SetPaused(
    channelgroup: usize,
    paused: i32,
) -> i32 {
    self::core::channel_group_set_paused(channelgroup, paused)
}

pub unsafe extern "win64" fn FMOD_ChannelGroup_GetPaused(
    channelgroup: usize,
    paused_out: *mut i32,
) -> i32 {
    self::core::channel_group_get_paused(channelgroup, paused_out)
}

// @exports FMOD_ChannelGroup_SetPitch
fmod_float_export!(
    FMOD_ChannelGroup_SetPitch -> self::core::channel_group_set_pitch_impl;
    win64(channelgroup: usize, pitch: f32);
    aarch64(channelgroup: usize, _xmm0: f32, pitch: f32);
    call(channelgroup, pitch)
);

pub unsafe extern "win64" fn FMOD_ChannelGroup_GetPitch(
    channelgroup: usize,
    pitch_out: *mut f32,
) -> i32 {
    self::core::channel_group_get_pitch(channelgroup, pitch_out)
}

pub unsafe extern "win64" fn FMOD_ChannelGroup_Stop(channelgroup: usize) -> i32 {
    self::core::channel_group_stop(channelgroup)
}

pub unsafe extern "win64" fn FMOD_ChannelGroup_AddGroup(
    channelgroup: usize,
    group: usize,
    propagatedspclock: i32,
    connection: *mut usize,
) -> i32 {
    self::core::channel_group_add_group(channelgroup, group, propagatedspclock, connection)
}

pub unsafe extern "win64" fn FMOD_ChannelGroup_Release(channelgroup: usize) -> i32 {
    self::core::channel_group_release(channelgroup)
}

pub unsafe extern "win64" fn FMOD_ChannelGroup_GetParentGroup(
    channelgroup: usize,
    group_out: *mut usize,
) -> i32 {
    self::core::channel_group_get_parent_group(channelgroup, group_out)
}

pub unsafe extern "win64" fn FMOD_ChannelGroup_GetName(
    channelgroup: usize,
    name_out: *mut u8,
    namelen: i32,
) -> i32 {
    self::core::channel_group_get_name(channelgroup, name_out, namelen)
}

pub unsafe extern "win64" fn FMOD_ChannelGroup_IsPlaying(
    channelgroup: usize,
    isplaying_out: *mut i32,
) -> i32 {
    self::core::channel_group_is_playing(channelgroup, isplaying_out)
}

// ============================================================================
// FMOD Studio (`fmodstudio.dll` / `fmodstudio64.dll`) — Utility & System
// ============================================================================

pub unsafe extern "win64" fn FMOD_Studio_ParseID(
    idstring: *const u8,
    id_out: *mut FmodGuid,
) -> i32 {
    studio::parse_id(idstring, id_out)
}

pub unsafe extern "win64" fn FMOD_Studio_System_Create(
    system_out: *mut usize,
    header_version: u32,
) -> i32 {
    studio::system_create(system_out, header_version)
}

pub unsafe extern "win64" fn FMOD_Studio_System_IsValid(system: usize) -> i32 {
    studio::system_is_valid(system)
}

pub unsafe extern "win64" fn FMOD_Studio_System_Initialize(
    system: usize,
    maxchannels: i32,
    studioflags: u32,
    flags: u32,
    extradriverdata: usize,
) -> i32 {
    studio::system_initialize(system, maxchannels, studioflags, flags, extradriverdata)
}

pub unsafe extern "win64" fn FMOD_Studio_System_Update(system: usize) -> i32 {
    studio::system_update(system)
}

pub unsafe extern "win64" fn FMOD_Studio_System_Release(system: usize) -> i32 {
    studio::system_release(system)
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetCoreSystem(
    system: usize,
    coresystem_out: *mut usize,
) -> i32 {
    studio::system_get_core_system(system, coresystem_out)
}

pub unsafe extern "win64" fn FMOD_Studio_System_LoadBankFile(
    system: usize,
    filename: *const u8,
    flags: u32,
    bank_out: *mut usize,
) -> i32 {
    studio::system_load_bank_file(system, filename, flags, bank_out)
}

pub unsafe extern "win64" fn FMOD_Studio_System_LoadBankMemory(
    system: usize,
    buffer: *const u8,
    length: i32,
    mode: u32,
    flags: u32,
    bank_out: *mut usize,
) -> i32 {
    studio::system_load_bank_memory(system, buffer, length, mode, flags, bank_out)
}

pub unsafe extern "win64" fn FMOD_Studio_System_UnloadAll(system: usize) -> i32 {
    studio::system_unload_all(system)
}

pub unsafe extern "win64" fn FMOD_Studio_System_FlushCommands(system: usize) -> i32 {
    studio::system_flush_commands(system)
}

pub unsafe extern "win64" fn FMOD_Studio_System_FlushSampleLoading(system: usize) -> i32 {
    studio::system_flush_sample_loading(system)
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetBank(
    system: usize,
    path: *const u8,
    bank_out: *mut usize,
) -> i32 {
    studio::system_get_bank(system, path, bank_out)
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetBankByID(
    system: usize,
    id: *const FmodGuid,
    bank_out: *mut usize,
) -> i32 {
    studio::system_get_bank_by_id(system, id, bank_out)
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetBankCount(
    system: usize,
    count_out: *mut i32,
) -> i32 {
    studio::system_get_bank_count(system, count_out)
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetBankList(
    system: usize,
    array: *mut usize,
    capacity: i32,
    count_out: *mut i32,
) -> i32 {
    studio::system_get_bank_list(system, array, capacity, count_out)
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetEvent(
    system: usize,
    path: *const u8,
    event_out: *mut usize,
) -> i32 {
    studio::system_get_event(system, path, event_out)
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetEventByID(
    system: usize,
    id: *const FmodGuid,
    event_out: *mut usize,
) -> i32 {
    studio::system_get_event_by_id(system, id, event_out)
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetBus(
    system: usize,
    path: *const u8,
    bus_out: *mut usize,
) -> i32 {
    studio::system_get_bus(system, path, bus_out)
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetBusByID(
    system: usize,
    id: *const FmodGuid,
    bus_out: *mut usize,
) -> i32 {
    studio::system_get_bus_by_id(system, id, bus_out)
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetVCA(
    system: usize,
    path: *const u8,
    vca_out: *mut usize,
) -> i32 {
    studio::system_get_vca(system, path, vca_out)
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetVCAByID(
    system: usize,
    id: *const FmodGuid,
    vca_out: *mut usize,
) -> i32 {
    studio::system_get_vca_by_id(system, id, vca_out)
}

pub unsafe extern "win64" fn FMOD_Studio_System_LookupID(
    system: usize,
    path: *const u8,
    id_out: *mut FmodGuid,
) -> i32 {
    studio::system_lookup_id(system, path, id_out)
}

pub unsafe extern "win64" fn FMOD_Studio_System_LookupPath(
    system: usize,
    id: *const FmodGuid,
    path_out: *mut u8,
    size: i32,
    retrieved_out: *mut i32,
) -> i32 {
    studio::system_lookup_path(system, id, path_out, size, retrieved_out)
}

pub unsafe extern "win64" fn FMOD_Studio_System_SetNumListeners(
    system: usize,
    numlisteners: i32,
) -> i32 {
    studio::system_set_num_listeners(system, numlisteners)
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetNumListeners(
    system: usize,
    numlisteners_out: *mut i32,
) -> i32 {
    studio::system_get_num_listeners(system, numlisteners_out)
}

pub unsafe extern "win64" fn FMOD_Studio_System_SetListenerAttributes(
    system: usize,
    listener: i32,
    attributes: *const Fmod3DAttributes,
    attenuationposition: *const FmodVector,
) -> i32 {
    studio::system_set_listener_attributes(system, listener, attributes, attenuationposition)
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetListenerAttributes(
    system: usize,
    listener: i32,
    attributes_out: *mut Fmod3DAttributes,
    attenuationposition_out: *mut FmodVector,
) -> i32 {
    studio::system_get_listener_attributes(
        system,
        listener,
        attributes_out,
        attenuationposition_out,
    )
}

// @exports FMOD_Studio_System_SetListenerWeight
fmod_float_export!(
    FMOD_Studio_System_SetListenerWeight -> studio::system_set_listener_weight_impl;
    win64(system: usize, listener: i32, weight: f32);
    aarch64(system: usize, listener: i32, _xmm0: f32, _xmm1: f32, weight: f32);
    call(system, listener, weight)
);

pub unsafe extern "win64" fn FMOD_Studio_System_GetListenerWeight(
    system: usize,
    listener: i32,
    weight_out: *mut f32,
) -> i32 {
    studio::system_get_listener_weight(system, listener, weight_out)
}

// @exports FMOD_Studio_System_SetParameterByName
fmod_float_export!(
    FMOD_Studio_System_SetParameterByName -> studio::system_set_param_by_name_impl;
    win64(system: usize, name: *const u8, value: f32, ignoreseekspeed: i32);
    aarch64(
        system: usize,
        name: *const u8,
        _r8: u64,
        ignoreseekspeed: i32,
        _xmm0: f32,
        _xmm1: f32,
        value: f32
    );
    call(system, name, value, ignoreseekspeed)
);

pub unsafe extern "win64" fn FMOD_Studio_System_GetParameterByName(
    system: usize,
    name: *const u8,
    value_out: *mut f32,
    finalvalue_out: *mut f32,
) -> i32 {
    studio::system_get_param_by_name(system, name, value_out, finalvalue_out)
}

// @exports FMOD_Studio_System_SetParameterByID
fmod_float_export!(
    FMOD_Studio_System_SetParameterByID -> studio::system_set_param_by_id_impl;
    win64(system: usize, id: u64, value: f32, ignoreseekspeed: i32);
    aarch64(
        system: usize,
        id: u64,
        _r8: u64,
        ignoreseekspeed: i32,
        _xmm0: f32,
        _xmm1: f32,
        value: f32
    );
    call(system, id, value, ignoreseekspeed)
);

pub unsafe extern "win64" fn FMOD_Studio_System_GetParameterByID(
    system: usize,
    id: u64,
    value_out: *mut f32,
    finalvalue_out: *mut f32,
) -> i32 {
    studio::system_get_param_by_id(system, id, value_out, finalvalue_out)
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetParameterDescriptionByName(
    system: usize,
    name: *const u8,
    parameter_out: *mut FmodStudioParameterDescription,
) -> i32 {
    studio::system_get_param_desc_by_name(system, name, parameter_out)
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetParameterDescriptionByID(
    system: usize,
    id: u64,
    parameter_out: *mut FmodStudioParameterDescription,
) -> i32 {
    studio::system_get_param_desc_by_id(system, id, parameter_out)
}

pub unsafe extern "win64" fn FMOD_Studio_System_GetParameterDescriptionCount(
    system: usize,
    count_out: *mut i32,
) -> i32 {
    studio::system_get_param_desc_count(system, count_out)
}

// ============================================================================
// FMOD Studio — Bank
// ============================================================================

pub unsafe extern "win64" fn FMOD_Studio_Bank_IsValid(bank: usize) -> i32 {
    studio::bank_is_valid(bank)
}

pub unsafe extern "win64" fn FMOD_Studio_Bank_GetID(bank: usize, id_out: *mut FmodGuid) -> i32 {
    studio::bank_get_id(bank, id_out)
}

pub unsafe extern "win64" fn FMOD_Studio_Bank_GetPath(
    bank: usize,
    path_out: *mut u8,
    size: i32,
    retrieved_out: *mut i32,
) -> i32 {
    studio::bank_get_path(bank, path_out, size, retrieved_out)
}

pub unsafe extern "win64" fn FMOD_Studio_Bank_Unload(bank: usize) -> i32 {
    studio::bank_unload(bank)
}

pub unsafe extern "win64" fn FMOD_Studio_Bank_LoadSampleData(bank: usize) -> i32 {
    studio::bank_load_sample_data(bank)
}

pub unsafe extern "win64" fn FMOD_Studio_Bank_UnloadSampleData(bank: usize) -> i32 {
    studio::bank_unload_sample_data(bank)
}

pub unsafe extern "win64" fn FMOD_Studio_Bank_GetLoadingState(
    bank: usize,
    state_out: *mut u32,
) -> i32 {
    studio::bank_get_loading_state(bank, state_out)
}

pub unsafe extern "win64" fn FMOD_Studio_Bank_GetSampleLoadingState(
    bank: usize,
    state_out: *mut u32,
) -> i32 {
    studio::bank_get_sample_loading_state(bank, state_out)
}

pub unsafe extern "win64" fn FMOD_Studio_Bank_GetStringCount(
    bank: usize,
    count_out: *mut i32,
) -> i32 {
    studio::bank_get_string_count(bank, count_out)
}

pub unsafe extern "win64" fn FMOD_Studio_Bank_GetStringInfo(
    bank: usize,
    index: i32,
    id_out: *mut FmodGuid,
    path_out: *mut u8,
    size: i32,
    retrieved_out: *mut i32,
) -> i32 {
    studio::bank_get_string_info(bank, index, id_out, path_out, size, retrieved_out)
}

pub unsafe extern "win64" fn FMOD_Studio_Bank_GetEventCount(
    bank: usize,
    count_out: *mut i32,
) -> i32 {
    studio::bank_get_event_count(bank, count_out)
}

pub unsafe extern "win64" fn FMOD_Studio_Bank_GetEventList(
    bank: usize,
    array: *mut usize,
    capacity: i32,
    count_out: *mut i32,
) -> i32 {
    studio::bank_get_event_list(bank, array, capacity, count_out)
}

pub unsafe extern "win64" fn FMOD_Studio_Bank_GetBusCount(
    bank: usize,
    count_out: *mut i32,
) -> i32 {
    studio::bank_get_bus_count(bank, count_out)
}

pub unsafe extern "win64" fn FMOD_Studio_Bank_GetBusList(
    bank: usize,
    array: *mut usize,
    capacity: i32,
    count_out: *mut i32,
) -> i32 {
    studio::bank_get_bus_list(bank, array, capacity, count_out)
}

pub unsafe extern "win64" fn FMOD_Studio_Bank_GetVCACount(
    bank: usize,
    count_out: *mut i32,
) -> i32 {
    studio::bank_get_vca_count(bank, count_out)
}

pub unsafe extern "win64" fn FMOD_Studio_Bank_GetVCAList(
    bank: usize,
    array: *mut usize,
    capacity: i32,
    count_out: *mut i32,
) -> i32 {
    studio::bank_get_vca_list(bank, array, capacity, count_out)
}

// ============================================================================
// FMOD Studio — EventDescription
// ============================================================================

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_IsValid(event_desc: usize) -> i32 {
    studio::event_desc_is_valid(event_desc)
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_GetID(
    event_desc: usize,
    id_out: *mut FmodGuid,
) -> i32 {
    studio::event_desc_get_id(event_desc, id_out)
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_GetPath(
    event_desc: usize,
    path_out: *mut u8,
    size: i32,
    retrieved_out: *mut i32,
) -> i32 {
    studio::event_desc_get_path(event_desc, path_out, size, retrieved_out)
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_GetParameterDescriptionCount(
    event_desc: usize,
    count_out: *mut i32,
) -> i32 {
    studio::event_desc_get_param_desc_count(event_desc, count_out)
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_GetParameterDescriptionByIndex(
    event_desc: usize,
    index: i32,
    parameter_out: *mut FmodStudioParameterDescription,
) -> i32 {
    studio::event_desc_get_param_desc_by_index(event_desc, index, parameter_out)
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_GetParameterDescriptionByName(
    event_desc: usize,
    name: *const u8,
    parameter_out: *mut FmodStudioParameterDescription,
) -> i32 {
    studio::event_desc_get_param_desc_by_name(event_desc, name, parameter_out)
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_GetParameterDescriptionByID(
    event_desc: usize,
    id: u64,
    parameter_out: *mut FmodStudioParameterDescription,
) -> i32 {
    studio::event_desc_get_param_desc_by_id(event_desc, id, parameter_out)
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_GetLength(
    event_desc: usize,
    length_out: *mut i32,
) -> i32 {
    studio::event_desc_get_length(event_desc, length_out)
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_GetMinMaxDistance(
    event_desc: usize,
    min_out: *mut f32,
    max_out: *mut f32,
) -> i32 {
    studio::event_desc_get_min_max_distance(event_desc, min_out, max_out)
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_GetSoundSize(
    event_desc: usize,
    size_out: *mut f32,
) -> i32 {
    studio::event_desc_get_sound_size(event_desc, size_out)
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_IsSnapshot(
    event_desc: usize,
    snapshot_out: *mut i32,
) -> i32 {
    studio::event_desc_is_snapshot(event_desc, snapshot_out)
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_IsOneshot(
    event_desc: usize,
    oneshot_out: *mut i32,
) -> i32 {
    studio::event_desc_is_oneshot(event_desc, oneshot_out)
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_IsStream(
    event_desc: usize,
    isstream_out: *mut i32,
) -> i32 {
    studio::event_desc_is_stream(event_desc, isstream_out)
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_Is3D(
    event_desc: usize,
    is3d_out: *mut i32,
) -> i32 {
    studio::event_desc_is_3d(event_desc, is3d_out)
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_HasSustainPoint(
    event_desc: usize,
    sustainpoint_out: *mut i32,
) -> i32 {
    studio::event_desc_has_sustain_point(event_desc, sustainpoint_out)
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_CreateInstance(
    event_desc: usize,
    instance_out: *mut usize,
) -> i32 {
    studio::event_desc_create_instance(event_desc, instance_out)
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_GetInstanceCount(
    event_desc: usize,
    count_out: *mut i32,
) -> i32 {
    studio::event_desc_get_instance_count(event_desc, count_out)
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_GetInstanceList(
    event_desc: usize,
    array: *mut usize,
    capacity: i32,
    count_out: *mut i32,
) -> i32 {
    studio::event_desc_get_instance_list(event_desc, array, capacity, count_out)
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_LoadSampleData(
    event_desc: usize,
) -> i32 {
    studio::event_desc_load_sample_data(event_desc)
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_UnloadSampleData(
    event_desc: usize,
) -> i32 {
    studio::event_desc_unload_sample_data(event_desc)
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_GetSampleLoadingState(
    event_desc: usize,
    state_out: *mut u32,
) -> i32 {
    studio::event_desc_get_sample_loading_state(event_desc, state_out)
}

pub unsafe extern "win64" fn FMOD_Studio_EventDescription_ReleaseAllInstances(
    event_desc: usize,
) -> i32 {
    studio::event_desc_release_all_instances(event_desc)
}

// ============================================================================
// FMOD Studio — EventInstance
// ============================================================================

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_IsValid(instance: usize) -> i32 {
    studio::event_instance_is_valid(instance)
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_GetDescription(
    instance: usize,
    description_out: *mut usize,
) -> i32 {
    studio::event_instance_get_description(instance, description_out)
}

// @exports FMOD_Studio_EventInstance_SetVolume
fmod_float_export!(
    FMOD_Studio_EventInstance_SetVolume -> studio::event_instance_set_volume_impl;
    win64(instance: usize, volume: f32);
    aarch64(instance: usize, _xmm0: f32, volume: f32);
    call(instance, volume)
);

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_GetVolume(
    instance: usize,
    volume_out: *mut f32,
    finalvolume_out: *mut f32,
) -> i32 {
    studio::event_instance_get_volume(instance, volume_out, finalvolume_out)
}

// @exports FMOD_Studio_EventInstance_SetPitch
fmod_float_export!(
    FMOD_Studio_EventInstance_SetPitch -> studio::event_instance_set_pitch_impl;
    win64(instance: usize, pitch: f32);
    aarch64(instance: usize, _xmm0: f32, pitch: f32);
    call(instance, pitch)
);

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_GetPitch(
    instance: usize,
    pitch_out: *mut f32,
    finalpitch_out: *mut f32,
) -> i32 {
    studio::event_instance_get_pitch(instance, pitch_out, finalpitch_out)
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_Set3DAttributes(
    instance: usize,
    attributes: *const Fmod3DAttributes,
) -> i32 {
    studio::event_instance_set_3d_attributes(instance, attributes)
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_Get3DAttributes(
    instance: usize,
    attributes_out: *mut Fmod3DAttributes,
) -> i32 {
    studio::event_instance_get_3d_attributes(instance, attributes_out)
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_SetPaused(
    instance: usize,
    paused: i32,
) -> i32 {
    studio::event_instance_set_paused(instance, paused)
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_GetPaused(
    instance: usize,
    paused_out: *mut i32,
) -> i32 {
    studio::event_instance_get_paused(instance, paused_out)
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_Start(instance: usize) -> i32 {
    studio::event_instance_start(instance)
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_Stop(
    instance: usize,
    mode: u32,
) -> i32 {
    studio::event_instance_stop(instance, mode)
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_GetTimelinePosition(
    instance: usize,
    position_out: *mut i32,
) -> i32 {
    studio::event_instance_get_timeline_position(instance, position_out)
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_SetTimelinePosition(
    instance: usize,
    position: i32,
) -> i32 {
    studio::event_instance_set_timeline_position(instance, position)
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_GetPlaybackState(
    instance: usize,
    state_out: *mut u32,
) -> i32 {
    studio::event_instance_get_playback_state(instance, state_out)
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_GetChannelGroup(
    instance: usize,
    group_out: *mut usize,
) -> i32 {
    studio::event_instance_get_channel_group(instance, group_out)
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_Release(instance: usize) -> i32 {
    studio::event_instance_release(instance)
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_IsVirtual(
    instance: usize,
    virtualstate_out: *mut i32,
) -> i32 {
    studio::event_instance_is_virtual(instance, virtualstate_out)
}

// @exports FMOD_Studio_EventInstance_SetParameterByName
fmod_float_export!(
    FMOD_Studio_EventInstance_SetParameterByName -> studio::event_instance_set_param_by_name_impl;
    win64(instance: usize, name: *const u8, value: f32, ignoreseekspeed: i32);
    aarch64(
        instance: usize,
        name: *const u8,
        _r8: u64,
        ignoreseekspeed: i32,
        _xmm0: f32,
        _xmm1: f32,
        value: f32
    );
    call(instance, name, value, ignoreseekspeed)
);

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_GetParameterByName(
    instance: usize,
    name: *const u8,
    value_out: *mut f32,
    finalvalue_out: *mut f32,
) -> i32 {
    studio::event_instance_get_param_by_name(instance, name, value_out, finalvalue_out)
}

// @exports FMOD_Studio_EventInstance_SetParameterByID
fmod_float_export!(
    FMOD_Studio_EventInstance_SetParameterByID -> studio::event_instance_set_param_by_id_impl;
    win64(instance: usize, id: u64, value: f32, ignoreseekspeed: i32);
    aarch64(
        instance: usize,
        id: u64,
        _r8: u64,
        ignoreseekspeed: i32,
        _xmm0: f32,
        _xmm1: f32,
        value: f32
    );
    call(instance, id, value, ignoreseekspeed)
);

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_GetParameterByID(
    instance: usize,
    id: u64,
    value_out: *mut f32,
    finalvalue_out: *mut f32,
) -> i32 {
    studio::event_instance_get_param_by_id(instance, id, value_out, finalvalue_out)
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_GetMinMaxDistance(
    instance: usize,
    min_out: *mut f32,
    max_out: *mut f32,
) -> i32 {
    studio::event_instance_get_min_max_distance(instance, min_out, max_out)
}

// @exports FMOD_Studio_EventInstance_SetProperty
fmod_float_export!(
    FMOD_Studio_EventInstance_SetProperty -> studio::event_instance_set_property_impl;
    win64(instance: usize, index: u32, value: f32);
    aarch64(instance: usize, index: u32, _xmm0: f32, _xmm1: f32, value: f32);
    call(instance, index, value)
);

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_GetProperty(
    instance: usize,
    index: u32,
    value_out: *mut f32,
) -> i32 {
    studio::event_instance_get_property(instance, index, value_out)
}

// @exports FMOD_Studio_EventInstance_SetReverbLevel
fmod_float_export!(
    FMOD_Studio_EventInstance_SetReverbLevel -> studio::event_instance_set_reverb_level_impl;
    win64(instance: usize, index: i32, reverb: f32);
    aarch64(instance: usize, index: i32, _xmm0: f32, _xmm1: f32, reverb: f32);
    call(instance, index, reverb)
);

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_GetReverbLevel(
    instance: usize,
    index: i32,
    reverb_out: *mut f32,
) -> i32 {
    studio::event_instance_get_reverb_level(instance, index, reverb_out)
}

pub unsafe extern "win64" fn FMOD_Studio_EventInstance_KeyOff(instance: usize) -> i32 {
    studio::event_instance_key_off(instance)
}

// ============================================================================
// FMOD Studio — Bus
// ============================================================================

pub unsafe extern "win64" fn FMOD_Studio_Bus_IsValid(bus: usize) -> i32 {
    studio::bus_is_valid(bus)
}

pub unsafe extern "win64" fn FMOD_Studio_Bus_GetID(bus: usize, id_out: *mut FmodGuid) -> i32 {
    studio::bus_get_id(bus, id_out)
}

pub unsafe extern "win64" fn FMOD_Studio_Bus_GetPath(
    bus: usize,
    path_out: *mut u8,
    size: i32,
    retrieved_out: *mut i32,
) -> i32 {
    studio::bus_get_path(bus, path_out, size, retrieved_out)
}

// @exports FMOD_Studio_Bus_SetVolume
fmod_float_export!(
    FMOD_Studio_Bus_SetVolume -> studio::bus_set_volume_impl;
    win64(bus: usize, volume: f32);
    aarch64(bus: usize, _xmm0: f32, volume: f32);
    call(bus, volume)
);

pub unsafe extern "win64" fn FMOD_Studio_Bus_GetVolume(
    bus: usize,
    volume_out: *mut f32,
    finalvolume_out: *mut f32,
) -> i32 {
    studio::bus_get_volume(bus, volume_out, finalvolume_out)
}

pub unsafe extern "win64" fn FMOD_Studio_Bus_SetPaused(bus: usize, paused: i32) -> i32 {
    studio::bus_set_paused(bus, paused)
}

pub unsafe extern "win64" fn FMOD_Studio_Bus_GetPaused(
    bus: usize,
    paused_out: *mut i32,
) -> i32 {
    studio::bus_get_paused(bus, paused_out)
}

pub unsafe extern "win64" fn FMOD_Studio_Bus_SetMute(bus: usize, mute: i32) -> i32 {
    studio::bus_set_mute(bus, mute)
}

pub unsafe extern "win64" fn FMOD_Studio_Bus_GetMute(bus: usize, mute_out: *mut i32) -> i32 {
    studio::bus_get_mute(bus, mute_out)
}

pub unsafe extern "win64" fn FMOD_Studio_Bus_StopAllEvents(bus: usize, mode: u32) -> i32 {
    studio::bus_stop_all_events(bus, mode)
}

pub unsafe extern "win64" fn FMOD_Studio_Bus_LockChannelGroup(bus: usize) -> i32 {
    studio::bus_lock_channel_group(bus)
}

pub unsafe extern "win64" fn FMOD_Studio_Bus_UnlockChannelGroup(bus: usize) -> i32 {
    studio::bus_unlock_channel_group(bus)
}

pub unsafe extern "win64" fn FMOD_Studio_Bus_GetChannelGroup(
    bus: usize,
    group_out: *mut usize,
) -> i32 {
    studio::bus_get_channel_group(bus, group_out)
}

// ============================================================================
// FMOD Studio — VCA
// ============================================================================

pub unsafe extern "win64" fn FMOD_Studio_VCA_IsValid(vca: usize) -> i32 {
    studio::vca_is_valid(vca)
}

pub unsafe extern "win64" fn FMOD_Studio_VCA_GetID(vca: usize, id_out: *mut FmodGuid) -> i32 {
    studio::vca_get_id(vca, id_out)
}

pub unsafe extern "win64" fn FMOD_Studio_VCA_GetPath(
    vca: usize,
    path_out: *mut u8,
    size: i32,
    retrieved_out: *mut i32,
) -> i32 {
    studio::vca_get_path(vca, path_out, size, retrieved_out)
}

// @exports FMOD_Studio_VCA_SetVolume
fmod_float_export!(
    FMOD_Studio_VCA_SetVolume -> studio::vca_set_volume_impl;
    win64(vca: usize, volume: f32);
    aarch64(vca: usize, _xmm0: f32, volume: f32);
    call(vca, volume)
);

pub unsafe extern "win64" fn FMOD_Studio_VCA_GetVolume(
    vca: usize,
    volume_out: *mut f32,
    finalvolume_out: *mut f32,
) -> i32 {
    studio::vca_get_volume(vca, volume_out, finalvolume_out)
}

#[cfg(test)]
mod tests;
