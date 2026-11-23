//! `bink2w64.dll` — native Rust replacement for RAD Game Tools Bink 2 (`1.999a` / `2022.05`).
//!
//! Supports full native ARM64 decoding of Bink 2 (`KB2f`..`KB2n`, integer path) frames
//! AND hardware Apple `VideoToolbox` HEVC (`KB2v`) frames into caller-registered planar
//! YCrCb (+ optional Alpha / HDR) buffers via the standard `BinkOpenWithOptions` /
//! `BinkGetFrameBuffersInfo` / `BinkRegisterFrameBuffers` / `BinkDoFrame*` / `BinkNextFrame` /
//! `BinkWait` / `BinkGoto` / `BinkGetKeyFrame` API.

#[path = "bink2/mod.rs"]
mod bink2;

use bink2::{async_threads, blit, playback, player, subtitles};
use crate::Export;
use std::ffi::c_void;
use std::sync::atomic::Ordering;

pub use bink2::player::BinkInstance;
pub use bink2::types::{
    BinkFrameBuffers, BinkFramePlaneSet, BinkOpenOptions, BinkPlane, BinkRealtime, BinkSummary,
    BinkTrack, BINKBGIORESUME, BINKBGIOSUSPEND, BINKBGIOTRYWAIT, BINKBGIOWAIT, BINKSURFACE24,
    BINKSURFACE24R, BINKSURFACE32, BINKSURFACE32A, BINKSURFACE32R, BINKSURFACE32RA,
    BINKSURFACE4444, BINKSURFACE555, BINKSURFACE5551, BINKSURFACE555R, BINKSURFACE565,
    BINKSURFACE565R, BINKSURFACE655, BINKSURFACE664, BINKSURFACE8P, BINKSURFACEMASK,
    BINKSURFACEUYVY, BINKSURFACEYUY2, BINKSURFACEYV12, DEFAULT_BINK_COLORSPACE,
};

pub static EXTRA_EXPORTS: &[Export] = &[];
pub static ORDINALS: &[(u16, &str)] = &[];

// ============================================================================
// Global Configuration, Allocators, Sound Setup, Errors & Timers
// ============================================================================

pub unsafe extern "win64" fn BinkSetMemory(alloc_fn: usize, free_fn: usize) {
    async_threads::set_memory(alloc_fn, free_fn);
}

pub unsafe extern "win64" fn BinkSetOSFileCallbacks(
    open_fn: usize,
    read_fn: usize,
    seek_fn: usize,
    close_fn: usize,
) {
    async_threads::set_os_file_callbacks(open_fn, read_fn, seek_fn, close_fn);
}

pub unsafe extern "win64" fn BinkOpenXAudio2(_param1: u64, _param2: u64) -> *mut c_void {
    async_threads::mark_sound_enabled();
    1usize as *mut c_void
}

pub unsafe extern "win64" fn BinkOpenXAudio27(_param1: u64, _param2: u64) -> *mut c_void {
    async_threads::mark_sound_enabled();
    1usize as *mut c_void
}

pub unsafe extern "win64" fn BinkOpenXAudio28(_param1: u64, _param2: u64) -> *mut c_void {
    async_threads::mark_sound_enabled();
    1usize as *mut c_void
}

pub unsafe extern "win64" fn BinkOpenXAudio29(_param1: u64, _param2: u64) -> *mut c_void {
    async_threads::mark_sound_enabled();
    1usize as *mut c_void
}

pub unsafe extern "win64" fn BinkOpenDirectSound(_param: u64) -> *mut c_void {
    async_threads::mark_sound_enabled();
    1usize as *mut c_void
}

pub unsafe extern "win64" fn BinkOpenWaveOut(_param: u64) -> *mut c_void {
    async_threads::mark_sound_enabled();
    1usize as *mut c_void
}

pub unsafe extern "win64" fn BinkFindXAudio2WinDevice(
    param1: u64,
    param2: u64,
) -> *mut c_void {
    log::debug!(
        "bink2w64: BinkFindXAudio2WinDevice({param1:#x}, {param2:#x}) -> NULL (default device)"
    );
    // NULL directs XAudio2 / Bink to use the default system audio endpoint.
    std::ptr::null_mut()
}

pub unsafe extern "win64" fn BinkSetSoundSystem(open_fn: usize, param: u64) -> i32 {
    async_threads::set_sound_system4(open_fn, param, 0, 0, 0)
}

pub unsafe extern "win64" fn BinkSetSoundSystem2(open_fn: usize, param1: u64, param2: u64) -> i32 {
    async_threads::set_sound_system4(open_fn, param1, param2, 0, 0)
}

pub unsafe extern "win64" fn BinkSetSoundSystem4(
    open_fn: usize,
    param1: u64,
    param2: u64,
    param3: u64,
    param4: u64,
) -> i32 {
    async_threads::set_sound_system4(open_fn, param1, param2, param3, param4)
}

pub unsafe extern "win64" fn BinkSetSoundTrack(total_tracks: u32, tracks: *const u32) {
    async_threads::set_sound_track(total_tracks, tracks);
}

pub unsafe extern "win64" fn BinkSetFileOffset(offset: u64) {
    async_threads::PREOPEN_FILE_OFFSET.store(offset, Ordering::Release);
}

pub unsafe extern "win64" fn BinkSetFrameRate(forcerate: u32, forceratediv: u32) {
    async_threads::PREOPEN_FRAME_RATE_NUM.store(forcerate, Ordering::Release);
    async_threads::PREOPEN_FRAME_RATE_DEN.store(forceratediv, Ordering::Release);
}

pub unsafe extern "win64" fn BinkSetIO(io_open_fn: usize) {
    async_threads::PREOPEN_USER_IO.store(io_open_fn, Ordering::Release);
}

pub unsafe extern "win64" fn BinkSetIOSize(iosize: u32) {
    async_threads::PREOPEN_IO_SIZE.store(iosize, Ordering::Release);
}

pub unsafe extern "win64" fn BinkSetSimulate(sim: u32) {
    async_threads::PREOPEN_SIMULATE.store(sim, Ordering::Release);
}

pub unsafe extern "win64" fn BinkGetError() -> *const u8 {
    async_threads::get_bink_error_ptr()
}

pub unsafe extern "win64" fn BinkSetError(err: *const u8) {
    async_threads::set_bink_error_ptr(err);
}

pub unsafe extern "win64" fn RADTimerRead() -> u32 {
    async_threads::rad_timer_read()
}

pub unsafe extern "win64" fn BinkLogoAddress() -> *mut c_void {
    bink2::types::BINK_LOGO_DATA.as_ptr() as *mut c_void
}

pub unsafe extern "win64" fn BinkGetPlatformInfo(
    bink_platform_enum: i32,
    output: *mut c_void,
) -> i32 {
    log::debug!(
        "bink2w64: BinkGetPlatformInfo(enum={bink_platform_enum}, output={output:?}) -> 0"
    );
    0
}

pub unsafe extern "win64" fn BinkFreeGlobals() {
    async_threads::free_globals();
}

// ============================================================================
// Utility CPU, Memory, Mutex & Async Thread Coordination
// ============================================================================

pub unsafe extern "win64" fn BinkUtilCPUs() -> i32 {
    async_threads::util_cpus()
}

pub unsafe extern "win64" fn BinkUtilMalloc(bytes: u64) -> *mut c_void {
    async_threads::util_malloc(bytes)
}

pub unsafe extern "win64" fn BinkUtilFree(ptr: *mut c_void) {
    async_threads::util_free(ptr);
}

pub unsafe extern "win64" fn BinkUtilMutexCreate() -> *mut c_void {
    async_threads::util_mutex_create()
}

pub unsafe extern "win64" fn BinkUtilMutexDestroy(mutex: *mut c_void) {
    async_threads::util_mutex_destroy(mutex);
}

pub unsafe extern "win64" fn BinkUtilMutexLock(mutex: *mut c_void) {
    async_threads::util_mutex_lock(mutex);
}

pub unsafe extern "win64" fn BinkUtilMutexLockTimeOut(mutex: *mut c_void, ms: i32) -> i32 {
    async_threads::util_mutex_lock_timeout(mutex, ms)
}

pub unsafe extern "win64" fn BinkUtilMutexUnlock(mutex: *mut c_void) {
    async_threads::util_mutex_unlock(mutex);
}

pub unsafe extern "win64" fn BinkUtilSoundGlobalLock() {
    async_threads::sound_global_lock();
}

pub unsafe extern "win64" fn BinkUtilSoundGlobalUnlock() {
    async_threads::sound_global_unlock();
}

pub unsafe extern "win64" fn BinkStartAsyncThread(thread_num: i32, _param: *const c_void) -> i32 {
    async_threads::start_async_thread(thread_num)
}

pub unsafe extern "win64" fn BinkRequestStopAsyncThread(thread_num: i32) -> i32 {
    async_threads::request_stop_async_thread(thread_num)
}

pub unsafe extern "win64" fn BinkWaitStopAsyncThread(thread_num: i32) -> i32 {
    async_threads::wait_stop_async_thread(thread_num)
}

pub unsafe extern "win64" fn BinkRequestStopAsyncThreadsMulti(
    threads: *const u32,
    thread_count: i32,
) -> i32 {
    async_threads::request_stop_async_threads_multi(threads, thread_count)
}

pub unsafe extern "win64" fn BinkWaitStopAsyncThreadsMulti(
    threads: *const u32,
    thread_count: i32,
) -> i32 {
    async_threads::wait_stop_async_threads_multi(threads, thread_count)
}

// ============================================================================
// Lifecycle, Frame Buffers, Decode & Blit
// ============================================================================

pub unsafe extern "win64" fn BinkOpen(name: *const u8, flags: u32) -> *mut c_void {
    player::open_with_options(name, std::ptr::null(), flags)
}

pub unsafe extern "win64" fn BinkOpenWithOptions(
    name: *const u8,
    boo: *const BinkOpenOptions,
    flags: u32,
) -> *mut c_void {
    player::open_with_options(name, boo, flags)
}

pub unsafe extern "win64" fn BinkClose(bink: *mut c_void) {
    player::close(bink);
}

pub unsafe extern "win64" fn BinkGetSummary(bink: *mut c_void, sum: *mut BinkSummary) {
    playback::get_summary(bink, sum);
}

pub unsafe extern "win64" fn BinkGetRealtime(
    bink: *mut c_void,
    run: *mut BinkRealtime,
    frame_window: u32,
) {
    playback::get_realtime(bink, run, frame_window);
}

pub unsafe extern "win64" fn BinkGetRects(bink: *mut c_void, _flags: u32) -> i32 {
    playback::get_rects(bink)
}

pub unsafe extern "win64" fn BinkGetGPUDataBuffersInfo(
    _bink: *mut c_void,
    gpu: *mut c_void,
) -> i32 {
    player::get_gpu_data_buffers_info(gpu)
}

pub unsafe extern "win64" fn BinkRegisterGPUDataBuffers(bink: *mut c_void, gpu: *mut c_void) {
    player::register_gpu_data_buffers(bink, gpu);
}

pub unsafe extern "win64" fn BinkGetFrameBuffersInfo(
    bink: *mut c_void,
    fbset: *mut BinkFrameBuffers,
) {
    player::get_frame_buffers_info(bink, fbset);
}

pub unsafe extern "win64" fn BinkRegisterFrameBuffers(
    bink: *mut c_void,
    fbset: *mut BinkFrameBuffers,
) {
    player::register_frame_buffers(bink, fbset);
}

pub unsafe extern "win64" fn BinkAllocateFrameBuffers(
    bink: *mut c_void,
    set: *mut BinkFrameBuffers,
    minimum_alignment: u32,
) -> i32 {
    player::allocate_frame_buffers(bink, set, minimum_alignment)
}

pub unsafe extern "win64" fn BinkDoFrame(bink: *mut c_void) -> i32 {
    player::do_frame(bink)
}

pub unsafe extern "win64" fn BinkDoFramePlane(bink: *mut c_void, which_planes: u32) -> i32 {
    player::do_frame_plane(bink, which_planes)
}

pub unsafe extern "win64" fn BinkDoFrameAsync(
    bink: *mut c_void,
    thread_one: u32,
    thread_two: u32,
) -> i32 {
    player::do_frame_async(bink, thread_one, thread_two)
}

pub unsafe extern "win64" fn BinkDoFrameAsyncMulti(
    bink: *mut c_void,
    threads: *const u32,
    thread_count: i32,
) -> i32 {
    player::do_frame_async_multi(bink, threads, thread_count)
}

pub unsafe extern "win64" fn BinkDoFrameAsyncWait(bink: *mut c_void, us: i32) -> i32 {
    async_threads::wait_async_decode(bink, us)
}

pub unsafe extern "win64" fn BinkCopyToBuffer(
    bink: *mut c_void,
    dest: *mut c_void,
    destpitch: i32,
    destheight: u32,
    destx: u32,
    desty: u32,
    flags: u32,
) -> i32 {
    blit::copy_to_buffer(bink, dest, destpitch, destheight, destx, desty, flags)
}

pub unsafe extern "win64" fn BinkCopyToBufferRect(
    bink: *mut c_void,
    dest: *mut c_void,
    destpitch: i32,
    destheight: u32,
    destx: u32,
    desty: u32,
    srcx: u32,
    srcy: u32,
    srcw: u32,
    srch: u32,
    flags: u32,
) -> i32 {
    blit::copy_to_buffer_rect(
        bink, dest, destpitch, destheight, destx, desty, srcx, srcy, srcw, srch, flags,
    )
}

// ============================================================================
// Playback Transport, Audio Track Controls & Subtitles
// ============================================================================

pub unsafe extern "win64" fn BinkService(bink: *mut c_void) {
    playback::service(bink);
}

pub unsafe extern "win64" fn BinkControlBackgroundIO(bink: *mut c_void, control: u32) -> i32 {
    playback::control_background_io(bink, control)
}

pub unsafe extern "win64" fn BinkSetVideoOnOff(bink: *mut c_void, onoff: i32) -> i32 {
    playback::set_video_on_off(bink, onoff)
}

pub unsafe extern "win64" fn BinkNextFrame(bink: *mut c_void) {
    playback::next_frame(bink);
}

pub unsafe extern "win64" fn BinkWait(bink: *mut c_void) -> i32 {
    playback::wait(bink)
}

pub unsafe extern "win64" fn BinkShouldSkip(_bink: *mut c_void) -> i32 {
    0
}

pub unsafe extern "win64" fn BinkPause(bink: *mut c_void, pause: i32) -> i32 {
    playback::pause(bink, pause)
}

pub unsafe extern "win64" fn BinkSetWillLoop(bink: *mut c_void, onoff: i32) {
    playback::set_will_loop(bink, onoff);
}

pub unsafe extern "win64" fn BinkGetKeyFrame(bink: *mut c_void, frame: u32, flags: i32) -> u32 {
    playback::get_key_frame(bink, frame, flags)
}

pub unsafe extern "win64" fn BinkGoto(bink: *mut c_void, frame: u32, flags: i32) {
    playback::goto(bink, frame, flags);
}

pub unsafe extern "win64" fn BinkSetVolume(bink: *mut c_void, track_id: u32, volume: i32) {
    playback::set_volume(bink, track_id, volume);
}

pub unsafe extern "win64" fn BinkSetPan(bink: *mut c_void, track_id: u32, pan: i32) {
    playback::set_pan(bink, track_id, pan);
}

pub unsafe extern "win64" fn BinkSetSpeakerVolumes(
    bink: *mut c_void,
    track_id: u32,
    speaker_indexes: *const u32,
    volumes: *const i32,
    total: u32,
) {
    playback::set_speaker_volumes(bink, track_id, speaker_indexes, volumes, total);
}

pub unsafe extern "win64" fn BinkSetSoundOnOff(bink: *mut c_void, onoff: i32) -> i32 {
    playback::set_sound_on_off(bink, onoff)
}

pub unsafe extern "win64" fn BinkGetTrackMaxSize(bink: *mut c_void, track_index: u32) -> u32 {
    playback::get_track_max_size(bink, track_index)
}

pub unsafe extern "win64" fn BinkGetTrackID(bink: *mut c_void, track_index: u32) -> u32 {
    playback::get_track_id(bink, track_index)
}

pub unsafe extern "win64" fn BinkGetTrackType(bink: *mut c_void, track_index: u32) -> u32 {
    playback::get_track_type(bink, track_index)
}

pub unsafe extern "win64" fn BinkOpenTrack(bink: *mut c_void, track_index: u32) -> *mut BinkTrack {
    playback::open_track(bink, track_index)
}

pub unsafe extern "win64" fn BinkCloseTrack(track: *mut BinkTrack) {
    playback::close_track(track);
}

pub unsafe extern "win64" fn BinkGetTrackData(track: *mut BinkTrack, dest: *mut c_void) -> u32 {
    playback::get_track_data(track, dest)
}

pub unsafe extern "win64" fn BinkLoadSubtitles(bink: *mut c_void, name: *const u8) -> i32 {
    subtitles::load_subtitles(bink, name)
}

pub unsafe extern "win64" fn BinkCurrentSubtitle(bink: *mut c_void) -> *const u8 {
    subtitles::current_subtitle(bink)
}

pub unsafe extern "win64" fn BinkGetSubtitleByIndex(bink: *mut c_void, index: i32) -> *const u8 {
    subtitles::subtitle_by_index(bink, index)
}

// ============================================================================
// Unit Tests (must remain at the bottom for win32-rt/build.rs export scanner)
// ============================================================================

#[cfg(test)]
#[path = "bink2/tests.rs"]
mod tests;
