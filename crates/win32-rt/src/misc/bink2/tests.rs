//! Unit tests for `bink2w64.dll`.

use super::bink2::async_threads::{
    async_pool, AsyncDecodeGuard, AsyncThreadState, MAX_ASYNC_THREADS, SOUND_DISABLED,
    SOUND_TRACK_SELECTION,
};
use super::bink2::audio as bink2_audio;
use super::*;
use bink2_codec::stream::{has_extended_header, Bk2FileHeader};
use std::ffi::CStr;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Duration;

#[test]
fn bink_lifecycle_and_offsets() {
    unsafe {
        let mut bk2 = [0u8; 1024];
        let hdr = Bk2FileHeader {
            magic: *b"KB2n",
            file_size_minus_8: 100_000 - 8,
            frames: 120,
            largest_frame_size: 8192,
            frames_dup: 120,
            width: 1920,
            height: 1080,
            fps_num: 30,
            fps_den: 1,
            video_flags: 0,
            num_tracks: 0,
        };
        std::ptr::write_unaligned(bk2.as_mut_ptr() as *mut Bk2FileHeader, hdr);

        // Open from memory (BINKFROMMEMORY = 0x0400_0000, BINKNOFRAMEBUFFERS = 0x0400)
        let hbink = BinkOpenWithOptions(bk2.as_ptr(), std::ptr::null(), 0x0400_0400);
        assert!(!hbink.is_null());
        let base = hbink as *const u8;
        assert_eq!(std::ptr::read_unaligned(base.add(0x00) as *const u32), 1920);
        assert_eq!(std::ptr::read_unaligned(base.add(0x04) as *const u32), 1080);
        assert_eq!(std::ptr::read_unaligned(base.add(0x08) as *const u32), 120);
        assert_eq!(std::ptr::read_unaligned(base.add(0x0c) as *const u32), 1);
        assert_eq!(std::ptr::read_unaligned(base.add(0x14) as *const u32), 30);
        assert_eq!(std::ptr::read_unaligned(base.add(0x18) as *const u32), 1);
        assert_eq!(std::ptr::read_unaligned(base.add(0x134) as *const u32), 2);

        // GPU buffers info returns 0 to select StandardDecompressor
        let mut gpu = [0u8; 0xe0];
        assert_eq!(
            BinkGetGPUDataBuffersInfo(hbink, gpu.as_mut_ptr() as *mut c_void),
            0
        );

        // FrameBuffers info & registration
        let mut fb: BinkFrameBuffers = std::mem::zeroed();
        BinkGetFrameBuffersInfo(hbink, &mut fb);
        assert_eq!(fb.total_frames, 2);
        assert_eq!(fb.ya_buffer_width, 1920);
        assert_eq!(fb.ya_buffer_height, 1088);
        assert_eq!(fb.crcb_buffer_width, 960);
        assert_eq!(fb.crcb_buffer_height, 544);
        assert_eq!(fb.frames[0].y_plane.allocate, 1);

        BinkRegisterFrameBuffers(hbink, &mut fb);
        assert_eq!(
            std::ptr::read_unaligned(base.add(0x50) as *const usize),
            (&mut fb as *mut BinkFrameBuffers) as usize
        );
        assert_eq!(
            std::ptr::read_unaligned(base.add(0xb0) as *const usize),
            (&mut fb as *mut BinkFrameBuffers) as usize
        );

        assert_eq!(BinkWait(hbink), 0);
        assert_eq!(BinkDoFrame(hbink), 0);
        let threads = [2u32, 3u32];
        assert_eq!(BinkDoFrameAsyncMulti(hbink, threads.as_ptr(), 2), 1);
        assert_eq!(BinkDoFrameAsync(hbink, 2, 3), 1);
        assert_eq!(BinkDoFrameAsyncWait(hbink, -1), 1);
        BinkNextFrame(hbink);
        assert_eq!(std::ptr::read_unaligned(base.add(0x0c) as *const u32), 2);
        assert_eq!(std::ptr::read_unaligned(base.add(0x10) as *const u32), 1);

        let mut sum = BinkSummary::default();
        BinkGetSummary(hbink, &mut sum);
        assert_eq!(sum.total_frames, 120);
        assert!(sum.total_played_frames >= 2);

        // Non-looping video reaches FrameNum == Frames cleanly without stalling
        BinkSetWillLoop(hbink, 0);
        BinkGoto(hbink, 119, 1);
        assert_eq!(std::ptr::read_unaligned(base.add(0x0c) as *const u32), 119);
        BinkNextFrame(hbink);
        assert_eq!(std::ptr::read_unaligned(base.add(0x0c) as *const u32), 120);
        assert_eq!(std::ptr::read_unaligned(base.add(0x10) as *const u32), 119);
        BinkNextFrame(hbink);
        assert_eq!(std::ptr::read_unaligned(base.add(0x0c) as *const u32), 120);
        assert_eq!(std::ptr::read_unaligned(base.add(0x10) as *const u32), 120);

        BinkPause(hbink, 1);
        assert_eq!(std::ptr::read_unaligned(base.add(0x11c) as *const u32), 1);
        assert_eq!(BinkWait(hbink), 1);
        BinkPause(hbink, 0);
        assert_eq!(BinkWait(hbink), 0);
        BinkSetWillLoop(hbink, 1);
        assert_eq!(std::ptr::read_unaligned(base.add(0x3b8) as *const u32), 1);
        BinkNextFrame(hbink);
        assert_eq!(std::ptr::read_unaligned(base.add(0x0c) as *const u32), 1);
        assert_eq!(std::ptr::read_unaligned(base.add(0x10) as *const u32), 120);

        BinkClose(hbink);
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[test]
fn kb2v_videotoolbox_hevc_end_to_end_pixel_accuracy() {
    use bink2_codec::hevc_container::{build_kb2v, parse_annex_b_hevc};
    use std::io::Write;
    use std::process::{Command, Stdio};

    let w = 320usize;
    let h = 192usize;
    // Distinct (Y, Cb, Cr) frames so any Y/Cb/Cr plane swap or range bug fails immediately.
    let colors: [(u8, u8, u8); 4] = [
        (81, 90, 240),   // red
        (41, 240, 110),  // blue
        (145, 54, 34),   // green
        (220, 128, 128), // near-white
    ];
    let mut raw_yuv = Vec::with_capacity(colors.len() * (w * h * 3 / 2));
    for &(y, cb, cr) in &colors {
        raw_yuv.extend(std::iter::repeat_n(y, w * h));
        raw_yuv.extend(std::iter::repeat_n(cb, (w / 2) * (h / 2)));
        raw_yuv.extend(std::iter::repeat_n(cr, (w / 2) * (h / 2)));
    }

    let Ok(mut child) = Command::new("ffmpeg")
        .args([
            "-y",
            "-v",
            "error",
            "-f",
            "rawvideo",
            "-pix_fmt",
            "yuv420p",
            "-s",
            "320x192",
            "-r",
            "30",
            "-color_range",
            "pc",
            "-i",
            "pipe:0",
            "-c:v",
            "hevc_videotoolbox",
            "-b:v",
            "1500k",
            "-bf",
            "0",
            "-g",
            "30",
            "-allow_sw",
            "1",
            "-color_range",
            "pc",
            "-f",
            "hevc",
            "pipe:1",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    else {
        // Skip gracefully if ffmpeg is absent in a minimal CI environment.
        return;
    };
    child.stdin.take().unwrap().write_all(&raw_yuv).unwrap();
    let out = child.wait_with_output().unwrap();
    assert!(
        out.status.success(),
        "ffmpeg hevc_videotoolbox failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    let parsed = parse_annex_b_hevc(&out.stdout).expect("parse annex-b HEVC");
    assert_eq!(parsed.frames.len(), colors.len());
    let kb2v = build_kb2v(w as u32, h as u32, 30, 1, true, &parsed).expect("build KB2v");

    unsafe {
        // Open from memory with auto-allocated frame buffers (BINKFROMMEMORY = 0x0400_0000).
        let hbink = BinkOpenWithOptions(kb2v.as_ptr(), std::ptr::null(), 0x0400_0000);
        assert!(!hbink.is_null(), "BinkOpenWithOptions failed on KB2v");

        let mut fb: BinkFrameBuffers = std::mem::zeroed();
        BinkGetFrameBuffersInfo(hbink, &mut fb);
        assert_eq!(fb.ya_buffer_width, 320);
        assert_eq!(fb.ya_buffer_height, 192);
        assert_eq!(fb.crcb_buffer_width, 160);
        assert_eq!(fb.crcb_buffer_height, 96);

        let inst = &*(hbink as *const BinkInstance);
        for (idx, &(exp_y, exp_cb, exp_cr)) in colors.iter().enumerate() {
            assert_eq!(BinkDoFrame(hbink), 0);
            let live_fb = &*(inst.rd_u64(0x0b0) as *const BinkFrameBuffers);
            let ps = &live_fb.frames[live_fb.frame_num as usize];
            let y_pitch = ps.y_plane.buffer_pitch as usize;
            let c_pitch = ps.cb_plane.buffer_pitch as usize;
            let got_y = *(ps.y_plane.buffer as *const u8).add((h / 2) * y_pitch + (w / 2));
            let got_cb = *(ps.cb_plane.buffer as *const u8).add((h / 4) * c_pitch + (w / 4));
            let got_cr = *(ps.cr_plane.buffer as *const u8).add((h / 4) * c_pitch + (w / 4));
            assert!(
                got_y.abs_diff(exp_y) <= 14
                    && got_cb.abs_diff(exp_cb) <= 14
                    && got_cr.abs_diff(exp_cr) <= 14,
                "frame {} mismatch: got YCbCr=({},{},{}) expected ({},{},{})",
                idx + 1,
                got_y,
                got_cb,
                got_cr,
                exp_y,
                exp_cb,
                exp_cr
            );
            BinkNextFrame(hbink);
        }

        // Seek back to frame 1 (IDR) and verify decode still succeeds cleanly.
        BinkGoto(hbink, 1, 0);
        assert_eq!(BinkDoFrame(hbink), 0);
        let live_fb = &*(inst.rd_u64(0x0b0) as *const BinkFrameBuffers);
        let ps = &live_fb.frames[live_fb.frame_num as usize];
        let got_cr = *(ps.cr_plane.buffer as *const u8)
            .add((h / 4) * (ps.cr_plane.buffer_pitch as usize) + (w / 4));
        assert!(got_cr.abs_diff(colors[0].2) <= 14, "seeked frame 1 Cr={got_cr}");

        BinkClose(hbink);
    }
}

fn build_synthetic_bink_with_audio(
    magic: [u8; 4],
    tracks: &[(u32, u32, u16, u32)], // (max_size, sample_rate, flags, track_id)
    frame_track_packets: &[Vec<Vec<u8>>],
) -> Vec<u8> {
    let num_frames = frame_track_packets.len() as u32;
    let num_tracks = tracks.len();
    let fourcc = u32::from_le_bytes(magic);
    let extra_hdr_bytes = if has_extended_header(fourcc) { 4 } else { 0 };
    let tables_off = 0x2c + extra_hdr_bytes;
    let tables_bytes = (num_tracks * 3 + (num_frames as usize) + 1) * 4;
    let data_start = (tables_off + tables_bytes + 3) & !3;

    let mut frame_blobs: Vec<Vec<u8>> = Vec::with_capacity(frame_track_packets.len());
    let mut frame_offsets: Vec<u32> = Vec::with_capacity(frame_track_packets.len() + 1);
    let mut cur_off = data_start;
    let mut largest_frame = 0u32;

    for (f_idx, track_pkts) in frame_track_packets.iter().enumerate() {
        let mut blob = Vec::new();
        for pkt in track_pkts {
            blob.extend_from_slice(&(pkt.len() as u32).to_le_bytes());
            blob.extend_from_slice(pkt);
        }
        // Pad frame blob to 4-byte boundary.
        while (blob.len() & 3) != 0 {
            blob.push(0);
        }
        largest_frame = largest_frame.max(blob.len() as u32);
        let is_key = if f_idx == 0 { 1u32 } else { 0u32 };
        frame_offsets.push(((cur_off as u32) & !3) | is_key);
        cur_off += blob.len();
        frame_blobs.push(blob);
    }
    frame_offsets.push((cur_off as u32) & !3);

    let total_len = cur_off;
    let hdr = Bk2FileHeader {
        magic,
        file_size_minus_8: (total_len.saturating_sub(8)) as u32,
        frames: num_frames,
        largest_frame_size: largest_frame.max(64),
        frames_dup: num_frames,
        width: 320,
        height: 180,
        fps_num: 30,
        fps_den: 1,
        video_flags: 0,
        num_tracks: num_tracks as u32,
    };

    let mut out = Vec::with_capacity(total_len);
    out.extend_from_slice(&hdr.to_bytes());
    out.resize(tables_off, 0);
    for &(max_size, _, _, _) in tracks {
        out.extend_from_slice(&max_size.to_le_bytes());
    }
    for &(_, sr, fl, _) in tracks {
        let raw_type = (sr & 0xFFFF) | ((fl as u32) << 16);
        out.extend_from_slice(&raw_type.to_le_bytes());
    }
    for &(_, _, _, tid) in tracks {
        out.extend_from_slice(&tid.to_le_bytes());
    }
    for &fo in &frame_offsets {
        out.extend_from_slice(&fo.to_le_bytes());
    }
    while out.len() < data_start {
        out.push(0);
    }
    for blob in frame_blobs {
        out.extend_from_slice(&blob);
    }
    assert_eq!(out.len(), total_len);
    out
}

#[test]
fn failed_open_returns_null_without_synthetic_header() {
    unsafe {
        let missing = b"/nonexistent_dir_bink_test/missing_movie.bk2\0";
        let h = BinkOpenWithOptions(missing.as_ptr(), std::ptr::null(), 0);
        assert!(h.is_null(), "missing file must return NULL, not a fake header");

        let bad_magic = [0xDEu8; 256];
        let h2 = BinkOpenWithOptions(bad_magic.as_ptr(), std::ptr::null(), 0x0400_0000);
        assert!(h2.is_null(), "invalid magic in memory buffer must return NULL");
    }
}

#[test]
fn bink_audio_tracks_playback_and_track_data_api() {
    use bink2_audio::test_util::{encode_simple_block, wrap_audio_packet, BitWriter};
    use bink2_audio::{BINK_AUD_16BITS, BINK_AUD_STEREO, BINK_AUD_USEDCT};

    let _audio_guard = crate::audio::audio_test_lock();
    crate::audio::backend::force_backend(Some(crate::audio::backend::Backend::Null));

    // Track 0: 44100 Hz stereo 16-bit DCT, id=10
    // Track 1: 22050 Hz mono 16-bit RDFT, id=20
    let t0_flags = BINK_AUD_16BITS | BINK_AUD_STEREO | BINK_AUD_USEDCT;
    let t1_flags = BINK_AUD_16BITS;

    // Frame 1 audio packets (non-zero DCT tone on track 0, non-zero RDFT sine on track 1)
    let mut bw_t0_f1 = BitWriter::new();
    encode_simple_block(
        &mut bw_t0_f1,
        44100,
        2,
        true,
        (0, 0, false),
        (0, 0, false),
        65,
        Some((10, 12, false)),
    );
    let pkt_t0_f1 = wrap_audio_packet((1920 * 2 * 2) as u32, &bw_t0_f1.into_bytes());

    let mut bw_t1_f1 = BitWriter::new();
    encode_simple_block(
        &mut bw_t1_f1,
        22050,
        1,
        false,
        (0, 0, false),
        (0, 0, false),
        62,
        Some((9, 14, false)),
    );
    let pkt_t1_f1 = wrap_audio_packet((960 * 2) as u32, &bw_t1_f1.into_bytes());

    // Frame 2 audio packets (silence on track 0, truncated corrupt packet on track 1)
    let mut bw_t0_f2 = BitWriter::new();
    encode_simple_block(
        &mut bw_t0_f2,
        44100,
        2,
        true,
        (0, 0, false),
        (0, 0, false),
        0,
        None,
    );
    let pkt_t0_f2 = wrap_audio_packet((1920 * 2 * 2) as u32, &bw_t0_f2.into_bytes());
    let pkt_t1_f2_truncated = vec![0x80, 0x07, 0x00, 0x00, 0x12, 0x34];

    let tracks = [
        ((1920 * 2 * 2) as u32, 44100u32, t0_flags, 10u32),
        ((960 * 2) as u32, 22050u32, t1_flags, 20u32),
    ];

    // Test BOTH Bink 2 (`KB2i`) and Bink 1 (`BIKi`) headers.
    for magic in [*b"KB2i", *b"BIKi"] {
        let bink_bytes = build_synthetic_bink_with_audio(
            magic,
            &tracks,
            &[
                vec![pkt_t0_f1.clone(), pkt_t1_f1.clone()],
                vec![pkt_t0_f2.clone(), pkt_t1_f2_truncated.clone()],
            ],
        );

        unsafe {
            assert!(!BinkOpenXAudio2(0, 0).is_null());
            assert_eq!(BinkSetSoundSystem2(BinkOpenXAudio2 as *const () as usize, 0, 0), 1);

            // Select track_id = 20 via BinkSetSoundTrack + BINKSNDTRACK (0x4000)
            let sel_id = 20u32;
            BinkSetSoundTrack(1, &sel_id);
            let hbink = BinkOpenWithOptions(
                bink_bytes.as_ptr(),
                std::ptr::null(),
                0x0400_0400 | 0x4000, // BINKFROMMEMORY | BINKNOFRAMEBUFFERS | BINKSNDTRACK
            );
            assert!(!hbink.is_null());

            // Verify track header metadata queries.
            assert_eq!(BinkGetTrackID(hbink, 0), 10);
            assert_eq!(BinkGetTrackID(hbink, 1), 20);
            assert_eq!(BinkGetTrackID(hbink, 2), 0);
            assert_eq!(BinkGetTrackMaxSize(hbink, 0), 1920 * 4);
            assert_eq!(BinkGetTrackMaxSize(hbink, 1), 960 * 2);
            assert_eq!(
                BinkGetTrackType(hbink, 0),
                44100u32 | ((t0_flags as u32) << 16)
            );
            assert_eq!(
                BinkGetTrackType(hbink, 1),
                22050u32 | ((t1_flags as u32) << 16)
            );

            // Verify selected playback track and volume/pan/pause/sound-on-off controls.
            assert_eq!(
                (*(hbink as *const BinkInstance)).audio_player.track_count(),
                1
            );
            BinkSetVolume(hbink, 20, 16384);
            BinkSetPan(hbink, 20, 49152);
            assert_eq!(
                (*(hbink as *const BinkInstance))
                    .audio_player
                    .track_volume_pan(0),
                Some((16384, 49152))
            );

            // Pause before decoding frame 1 so the null clock does not drain the ring immediately.
            assert_eq!(BinkPause(hbink, 1), 1);
            assert_eq!(BinkDoFrame(hbink), 0);
            assert_eq!(
                (*(hbink as *const BinkInstance))
                    .audio_player
                    .ring_queued_frames(),
                960
            );

            // BinkGoto must flush queued audio frames and reset decoder overlap state.
            BinkGoto(hbink, 1, 1);
            assert_eq!(
                (*(hbink as *const BinkInstance))
                    .audio_player
                    .ring_queued_frames(),
                0
            );
            assert_eq!(BinkPause(hbink, 0), 0);

            // Turning sound off mutes and clears; turning back on re-enables.
            assert_eq!(BinkSetSoundOnOff(hbink, 0), 0);
            assert_eq!(BinkSetSoundOnOff(hbink, 1), 1);

            // Pull decoded 16-bit PCM directly via BinkOpenTrack / BinkGetTrackData.
            let htr0 = BinkOpenTrack(hbink, 0);
            let htr1 = BinkOpenTrack(hbink, 1);
            assert!(!htr0.is_null() && !htr1.is_null());
            assert!(BinkOpenTrack(hbink, 2).is_null());
            assert_eq!((*htr0).frequency, 44100);
            assert_eq!((*htr0).bits, 16);
            assert_eq!((*htr0).channels, 2);
            assert_eq!((*htr0).max_size, 1920 * 4);
            assert_eq!((*htr1).frequency, 22050);
            assert_eq!((*htr1).channels, 1);

            let mut pcm0 = vec![0u8; 1920 * 4];
            let got0 = BinkGetTrackData(htr0, pcm0.as_mut_ptr() as *mut c_void);
            assert_eq!(got0, 1920 * 4);
            let samples0: Vec<i16> = pcm0
                .chunks_exact(2)
                .map(|c| i16::from_le_bytes([c[0], c[1]]))
                .collect();
            let peak0 = samples0.iter().map(|s| s.unsigned_abs()).max().unwrap_or(0);
            assert!(peak0 > 1000, "DCT track 0 should have non-zero PCM, peak={peak0}");
            // Second read on same frame without BinkNextFrame returns 0.
            assert_eq!(BinkGetTrackData(htr0, pcm0.as_mut_ptr() as *mut c_void), 0);

            let mut pcm1 = vec![0u8; 960 * 2];
            let got1 = BinkGetTrackData(htr1, pcm1.as_mut_ptr() as *mut c_void);
            assert_eq!(got1, 960 * 2);
            let samples1: Vec<i16> = pcm1
                .chunks_exact(2)
                .map(|c| i16::from_le_bytes([c[0], c[1]]))
                .collect();
            let peak1 = samples1.iter().map(|s| s.unsigned_abs()).max().unwrap_or(0);
            assert!(peak1 > 1000, "RDFT track 1 should have non-zero PCM, peak={peak1}");

            // Advance to frame 2: track 1 has truncated packet — must fail gracefully (0 bytes, no panic).
            BinkNextFrame(hbink);
            assert_eq!(BinkDoFrame(hbink), 0);
            let got1_f2 = BinkGetTrackData(htr1, pcm1.as_mut_ptr() as *mut c_void);
            assert_eq!(got1_f2, 0);

            BinkCloseTrack(htr0);
            BinkCloseTrack(htr1);
            BinkClose(hbink);

            // BinkSetSoundTrack(0, NULL) + BINKSNDTRACK opens 0 playback tracks cleanly.
            BinkSetSoundTrack(0, std::ptr::null());
            let hbink_muted = BinkOpenWithOptions(
                bink_bytes.as_ptr(),
                std::ptr::null(),
                0x0400_0400 | 0x4000,
            );
            assert!(!hbink_muted.is_null());
            assert_eq!(
                (*(hbink_muted as *const BinkInstance))
                    .audio_player
                    .track_count(),
                0
            );
            BinkClose(hbink_muted);
        }
    }

    // Restore global track selection state for subsequent tests.
    *SOUND_TRACK_SELECTION.lock().unwrap_or_else(|e| e.into_inner()) = None;
    SOUND_DISABLED.store(false, Ordering::Release);
}

#[test]
fn bink_copy_to_buffer_and_rect_formats() {
    unsafe {
        // Minimal 32x16 full-range KB2n movie with alpha flag (0x0010_0000 | 0x10)
        let mut bk2 = [0u8; 256];
        let hdr = Bk2FileHeader {
            magic: *b"KB2n",
            file_size_minus_8: 256 - 8,
            frames: 2,
            largest_frame_size: 64,
            frames_dup: 2,
            width: 32,
            height: 16,
            fps_num: 30,
            fps_den: 1,
            video_flags: 0x0010_0010, // alpha + full range
            num_tracks: 0,
        };
        std::ptr::write_unaligned(bk2.as_mut_ptr() as *mut Bk2FileHeader, hdr);

        // BINKFROMMEMORY (0x0400_0000) + auto-allocated frame buffers + alpha
        let hbink = BinkOpenWithOptions(
            bk2.as_ptr(),
            std::ptr::null(),
            0x0400_0000 | 0x0010_0000,
        );
        assert!(!hbink.is_null());

        // Populate the active decoded YCrCbA planes with known full-range values:
        // Left half (x < 16): pure red (Y=76, Cr=255, Cb=85, A=0x80)
        // Right half (x >= 16): pure white (Y=255, Cr=128, Cb=128, A=0xCC)
        let inst = &*(hbink as *const BinkInstance);
        let fb_ptr = inst.rd_u64(0x0b0) as *const BinkFrameBuffers;
        let fb = std::ptr::read_unaligned(fb_ptr);
        let ps = fb.frames[fb.frame_num as usize];
        let y_pitch = ps.y_plane.buffer_pitch as usize;
        let c_pitch = ps.cr_plane.buffer_pitch as usize;
        let a_pitch = ps.a_plane.buffer_pitch as usize;
        for y in 0..16usize {
            for x in 0..32usize {
                let (yv, av) = if x < 16 { (76u8, 0x80u8) } else { (255u8, 0xCCu8) };
                *(ps.y_plane.buffer as *mut u8).add(y * y_pitch + x) = yv;
                *(ps.a_plane.buffer as *mut u8).add(y * a_pitch + x) = av;
            }
        }
        for cy in 0..8usize {
            for cx in 0..16usize {
                let (cr, cb) = if cx < 8 { (255u8, 85u8) } else { (128u8, 128u8) };
                *(ps.cr_plane.buffer as *mut u8).add(cy * c_pitch + cx) = cr;
                *(ps.cb_plane.buffer as *mut u8).add(cy * c_pitch + cx) = cb;
            }
        }

        // 1. BINKSURFACE32 (BGRA opaque: [B, G, R, 0xFF])
        let mut bgra = vec![0u8; 32 * 16 * 4];
        assert_eq!(
            BinkCopyToBuffer(
                hbink,
                bgra.as_mut_ptr() as *mut c_void,
                32 * 4,
                16,
                0,
                0,
                BINKSURFACE32
            ),
            0
        );
        // Red pixel at (0, 0)
        assert!(bgra[0] <= 4 && bgra[1] <= 4 && bgra[2] >= 250 && bgra[3] == 0xFF);
        // White pixel at (20, 0)
        let woff = 20 * 4;
        assert!(
            bgra[woff] >= 252
                && bgra[woff + 1] >= 252
                && bgra[woff + 2] >= 252
                && bgra[woff + 3] == 0xFF
        );

        // 2. BINKSURFACE32RA (RGBA with alpha plane: [R, G, B, A])
        let mut rgba = vec![0u8; 32 * 16 * 4];
        assert_eq!(
            BinkCopyToBuffer(
                hbink,
                rgba.as_mut_ptr() as *mut c_void,
                32 * 4,
                16,
                0,
                0,
                BINKSURFACE32RA
            ),
            0
        );
        assert!(rgba[0] >= 250 && rgba[1] <= 4 && rgba[2] <= 4 && rgba[3] == 0x80);
        assert!(
            rgba[woff] >= 252
                && rgba[woff + 1] >= 252
                && rgba[woff + 2] >= 252
                && rgba[woff + 3] == 0xCC
        );

        // 3. BINKSURFACE24R (RGB24: [R, G, B])
        let mut rgb24 = vec![0u8; 32 * 16 * 3];
        assert_eq!(
            BinkCopyToBuffer(
                hbink,
                rgb24.as_mut_ptr() as *mut c_void,
                32 * 3,
                16,
                0,
                0,
                BINKSURFACE24R
            ),
            0
        );
        assert!(rgb24[0] >= 250 && rgb24[1] <= 4 && rgb24[2] <= 4);

        // 4. BINKSURFACE565 (16-bit RGB565)
        let mut rgb565 = vec![0u8; 32 * 16 * 2];
        assert_eq!(
            BinkCopyToBuffer(
                hbink,
                rgb565.as_mut_ptr() as *mut c_void,
                32 * 2,
                16,
                0,
                0,
                BINKSURFACE565
            ),
            0
        );
        let px_red = u16::from_le_bytes([rgb565[0], rgb565[1]]);
        assert_eq!(px_red >> 11, 31); // full red
        assert_eq!((px_red >> 5) & 0x3f, 0);
        assert_eq!(px_red & 0x1f, 0);
        let px_white = u16::from_le_bytes([rgb565[20 * 2], rgb565[20 * 2 + 1]]);
        assert_eq!(px_white, 0xFFFF);

        // 5. BinkCopyToBufferRect sub-rectangle into offset destination
        let mut rect_dst = vec![0x11u8; 32 * 16 * 4];
        assert_eq!(
            BinkCopyToBufferRect(
                hbink,
                rect_dst.as_mut_ptr() as *mut c_void,
                32 * 4,
                16,
                4,
                2,
                16,
                0,
                8,
                4,
                BINKSURFACE32A
            ),
            0
        );
        // Untouched pixel at (0, 0) stays 0x11
        assert_eq!(&rect_dst[0..4], &[0x11, 0x11, 0x11, 0x11]);
        // Copied white pixel with alpha 0xCC at (4, 2)
        let roff = (2 * 32 + 4) * 4;
        assert!(
            rect_dst[roff] >= 252
                && rect_dst[roff + 1] >= 252
                && rect_dst[roff + 2] >= 252
                && rect_dst[roff + 3] == 0xCC
        );

        // 6. BinkCopyToBuffer with YUY2 & YV12
        let mut yuy2 = vec![0u8; 32 * 16 * 2];
        assert_eq!(
            BinkCopyToBuffer(
                hbink,
                yuy2.as_mut_ptr() as *mut c_void,
                32 * 2,
                16,
                0,
                0,
                BINKSURFACEYUY2
            ),
            0
        );
        assert_eq!(&yuy2[0..4], &[76, 85, 76, 255]);

        let mut yv12 = vec![0u8; 32 * 16 + 16 * 8 * 2];
        assert_eq!(
            BinkCopyToBuffer(
                hbink,
                yv12.as_mut_ptr() as *mut c_void,
                32,
                16,
                0,
                0,
                BINKSURFACEYV12
            ),
            0
        );
        assert_eq!(yv12[0], 76);
        assert_eq!(yv12[20], 255);
        assert_eq!(yv12[32 * 16], 255); // V = Cr
        assert_eq!(yv12[32 * 16 + 16 * 8], 85); // U = Cb

        // 7. Turning video off makes BinkCopyToBuffer return 1 (skipped) and BinkGetRects return 0
        assert_eq!(BinkGetRects(hbink, 0), 1);
        assert_eq!(BinkSetVideoOnOff(hbink, 0), 0);
        assert_eq!(BinkGetRects(hbink, 0), 0);
        assert_eq!(
            BinkCopyToBuffer(
                hbink,
                bgra.as_mut_ptr() as *mut c_void,
                32 * 4,
                16,
                0,
                0,
                BINKSURFACE32
            ),
            1
        );
        assert_eq!(BinkSetVideoOnOff(hbink, 1), 1);
        assert_eq!(BinkGetRects(hbink, 0), 1);

        BinkClose(hbink);
    }
}

#[test]
fn bink_async_threads_multi_coordination() {
    unsafe {
        assert_eq!(BinkStartAsyncThread(0, std::ptr::null()), 1);
        assert_eq!(BinkStartAsyncThread(1, std::ptr::null()), 1);
        assert_eq!(BinkStartAsyncThread(2, std::ptr::null()), 1);
        assert_eq!(BinkStartAsyncThread(-1, std::ptr::null()), 0);
        assert_eq!(BinkStartAsyncThread(MAX_ASYNC_THREADS as i32, std::ptr::null()), 0);

        let hbink = BinkOpen(BinkLogoAddress() as *const u8, 0x0400_0000);
        assert!(!hbink.is_null());

        let tids = [0u32, 1u32, 2u32];
        assert_eq!(BinkDoFrameAsyncMulti(hbink, tids.as_ptr(), 3), 1);
        assert_eq!(BinkDoFrameAsyncWait(hbink, -1), 1);

        // Simulate an in-flight decode on thread 1 and verify BinkWaitStopAsyncThreadsMulti
        // blocks until the in-flight decode guard drops, while an unrelated handle's Wait(0)
        // does not falsely stall.
        let guard = AsyncDecodeGuard::acquire(&[1]);
        assert_eq!(BinkRequestStopAsyncThreadsMulti(tids.as_ptr(), 3), 1);
        assert_eq!(BinkDoFrameAsyncWait(hbink, 0), 1);

        let finished = Arc::new(AtomicBool::new(false));
        let finished_bg = finished.clone();
        let waiter = std::thread::spawn(move || {
            let ids = [0u32, 1u32, 2u32];
            let rc = BinkWaitStopAsyncThreadsMulti(ids.as_ptr(), 3);
            finished_bg.store(true, Ordering::Release);
            rc
        });

        std::thread::sleep(Duration::from_millis(15));
        assert!(
            !finished.load(Ordering::Acquire),
            "wait must not return while decode is in flight"
        );
        drop(guard);
        assert_eq!(waiter.join().unwrap(), 1);
        assert!(finished.load(Ordering::Acquire));

        {
            let (lock, _) = async_pool();
            let pool = lock.lock().unwrap_or_else(|e| e.into_inner());
            assert_eq!(pool.states[0], AsyncThreadState::Stopped);
            assert_eq!(pool.states[1], AsyncThreadState::Stopped);
            assert_eq!(pool.states[2], AsyncThreadState::Stopped);
        }

        // Null pointer with positive count fails; null pointer with 0 stops all cleanly.
        assert_eq!(BinkRequestStopAsyncThreadsMulti(std::ptr::null(), 2), 0);
        assert_eq!(BinkWaitStopAsyncThreadsMulti(std::ptr::null(), 2), 0);
        assert_eq!(BinkRequestStopAsyncThreadsMulti(std::ptr::null(), 0), 1);
        assert_eq!(BinkWaitStopAsyncThreadsMulti(std::ptr::null(), 0), 1);

        BinkClose(hbink);
    }
}

#[test]
fn bink_error_get_set_and_open_failure() {
    unsafe {
        BinkSetError(b"Custom Bink error 42\0".as_ptr());
        let got = CStr::from_ptr(BinkGetError() as *const std::ffi::c_char)
            .to_str()
            .unwrap();
        assert_eq!(got, "Custom Bink error 42");

        // NULL clears error string to empty.
        BinkSetError(std::ptr::null());
        let cleared = CStr::from_ptr(BinkGetError() as *const std::ffi::c_char)
            .to_str()
            .unwrap();
        assert_eq!(cleared, "");

        // Failed BinkOpen populates a descriptive error.
        let h = BinkOpen(b"/nonexistent_bink_err_test/nope.bk2\0".as_ptr(), 0);
        assert!(h.is_null());
        let open_err = CStr::from_ptr(BinkGetError() as *const std::ffi::c_char)
            .to_str()
            .unwrap();
        assert!(!open_err.is_empty());

        // Successful open clears error.
        let ok = BinkOpen(BinkLogoAddress() as *const u8, 0x0400_0000);
        assert!(!ok.is_null());
        let ok_err = CStr::from_ptr(BinkGetError() as *const std::ffi::c_char)
            .to_str()
            .unwrap();
        assert_eq!(ok_err, "");
        BinkClose(ok);
    }
}

#[test]
fn bink_util_mutex_and_helpers() {
    unsafe {
        assert!(BinkUtilCPUs() >= 1);

        // BinkUtilMalloc / BinkUtilFree (32-byte aligned)
        let p = BinkUtilMalloc(128);
        assert!(!p.is_null());
        assert_eq!((p as usize) % 32, 0);
        std::ptr::write_bytes(p as *mut u8, 0xA5, 128);
        BinkUtilFree(p);
        BinkUtilFree(std::ptr::null_mut());

        // BinkUtilMutex* with cross-thread timeout and unlock wakeup
        let m = BinkUtilMutexCreate();
        assert!(!m.is_null());
        BinkUtilMutexLock(m);
        // Re-entrant lock on same thread succeeds immediately
        assert_eq!(BinkUtilMutexLockTimeOut(m, 0), 1);
        BinkUtilMutexUnlock(m);

        // Another thread attempting lock with short timeout must time out (return 0)
        let m_addr = m as usize;
        let t1 = std::thread::spawn(move || BinkUtilMutexLockTimeOut(m_addr as *mut c_void, 20));
        assert_eq!(t1.join().unwrap(), 0);

        // Release lock while another thread waits with generous timeout -> succeeds (1)
        let t2 = std::thread::spawn(move || {
            let got = BinkUtilMutexLockTimeOut(m_addr as *mut c_void, 500);
            if got != 0 {
                BinkUtilMutexUnlock(m_addr as *mut c_void);
            }
            got
        });
        std::thread::sleep(Duration::from_millis(15));
        BinkUtilMutexUnlock(m);
        assert_eq!(t2.join().unwrap(), 1);

        BinkUtilMutexDestroy(m);
        // Operations on destroyed/null mutex are safe no-ops
        assert_eq!(BinkUtilMutexLockTimeOut(m, 0), 0);
        assert_eq!(BinkUtilMutexLockTimeOut(std::ptr::null_mut(), 0), 0);
        BinkUtilMutexDestroy(std::ptr::null_mut());

        // Global sound lock/unlock
        BinkUtilSoundGlobalLock();
        BinkUtilSoundGlobalLock(); // re-entrant safe
        BinkUtilSoundGlobalUnlock();
        BinkUtilSoundGlobalUnlock();
    }
}

#[test]
fn bink_realtime_allocate_plane_bgio_and_timer() {
    unsafe {
        let t0 = RADTimerRead();
        std::thread::sleep(Duration::from_millis(12));
        let t1 = RADTimerRead();
        assert!(
            t1.wrapping_sub(t0) >= 5,
            "RADTimerRead should advance monotonically: t0={t0}, t1={t1}"
        );

        let hbink = BinkOpen(BinkLogoAddress() as *const u8, 0x0400_0400);
        assert!(!hbink.is_null());

        // BinkAllocateFrameBuffers with 256-byte minimum alignment
        let mut fb: BinkFrameBuffers = std::mem::zeroed();
        assert_eq!(BinkAllocateFrameBuffers(hbink, &mut fb, 256), 1);
        assert!(!fb.frames[0].y_plane.buffer.is_null());
        assert_eq!((fb.frames[0].y_plane.buffer as usize) % 256, 0);
        assert!(!fb.frames[0].cr_plane.buffer.is_null());
        assert_eq!((fb.frames[0].cr_plane.buffer as usize) % 256, 0);
        BinkRegisterFrameBuffers(hbink, &mut fb);

        // BinkDoFramePlane + BinkService + BinkControlBackgroundIO
        assert_eq!(BinkDoFramePlane(hbink, 0x1f), 0);
        BinkService(hbink);
        assert_eq!(BinkControlBackgroundIO(hbink, BINKBGIOSUSPEND | BINKBGIOWAIT), 1);
        assert_eq!(BinkControlBackgroundIO(hbink, BINKBGIORESUME), 0);

        // BinkGetRealtime
        let mut rt = BinkRealtime::default();
        BinkGetRealtime(hbink, &mut rt, 30);
        assert_eq!(rt.frame_num, 1);
        assert_eq!(rt.frame_rate, 30);
        assert_eq!(rt.frame_rate_div, 1);
        assert_eq!(rt.frames, 1);
        assert!(rt.frames_time >= 33);
        assert!(rt.read_buffer_size > 0);

        // BinkGetPlatformInfo returns 0 cleanly
        assert_eq!(BinkGetPlatformInfo(0, std::ptr::null_mut()), 0);

        BinkClose(hbink);
        BinkFreeGlobals();
    }
}

#[test]
fn bink_sound_open_variants_sound_system4_and_speaker_volumes() {
    let _audio_guard = crate::audio::audio_test_lock();
    crate::audio::backend::force_backend(Some(crate::audio::backend::Backend::Null));

    unsafe {
        assert!(!BinkOpenXAudio27(0, 0).is_null());
        assert!(!BinkOpenXAudio28(0, 0).is_null());
        assert!(!BinkOpenXAudio29(0, 0).is_null());
        assert!(!BinkOpenDirectSound(0).is_null());
        assert!(!BinkOpenWaveOut(0).is_null());
        assert!(BinkFindXAudio2WinDevice(0, 0).is_null());

        assert_eq!(
            BinkSetSoundSystem4(BinkOpenXAudio29 as *const () as usize, 1, 2, 3, 4),
            1
        );
        assert!(!SOUND_DISABLED.load(Ordering::Acquire));

        // Open a synthetic 1-track video and verify BinkSetSpeakerVolumes updates volume & pan
        let tracks = [(1920u32 * 4, 44100u32, 0x7000u16, 7u32)];
        let bink_bytes = build_synthetic_bink_with_audio(*b"KB2i", &tracks, &[vec![vec![]]]);
        let hbink = BinkOpen(bink_bytes.as_ptr(), 0x0400_0400);
        assert!(!hbink.is_null());

        let spk_idx = [0u32, 1u32]; // FL, FR
        let vols = [16384i32, 8192i32]; // Left 0.5, Right 0.25 -> vol=16384, pan=16384
        BinkSetSpeakerVolumes(hbink, 7, spk_idx.as_ptr(), vols.as_ptr(), 2);
        assert_eq!(
            (*(hbink as *const BinkInstance))
                .audio_player
                .track_volume_pan(0),
            Some((16384, 16384))
        );

        // Right-only speaker volume -> pan = 65536
        let vols_r = [0i32, 32768i32];
        BinkSetSpeakerVolumes(hbink, 7, spk_idx.as_ptr(), vols_r.as_ptr(), 2);
        assert_eq!(
            (*(hbink as *const BinkInstance))
                .audio_player
                .track_volume_pan(0),
            Some((32768, 65536))
        );

        BinkClose(hbink);

        // Null sound system disables audio cleanly
        assert_eq!(BinkSetSoundSystem4(0, 0, 0, 0, 0), 1);
        assert!(SOUND_DISABLED.load(Ordering::Acquire));
        SOUND_DISABLED.store(false, Ordering::Release);
    }
}

#[test]
fn bink_subtitles_and_preopen_setters() {
    unsafe {
        // Pre-open setters: BinkSetFrameRate + BINKFRAMERATE (0x1000)
        BinkSetFrameRate(60, 1);
        BinkSetIOSize(131_072);
        BinkSetSimulate(500_000);
        BinkSetFileOffset(0);
        BinkSetIO(0);

        let mut bk2 = [0u8; 256];
        let hdr = Bk2FileHeader {
            magic: *b"KB2n",
            file_size_minus_8: 256 - 8,
            frames: 300,
            largest_frame_size: 64,
            frames_dup: 300,
            width: 320,
            height: 180,
            fps_num: 30,
            fps_den: 1,
            video_flags: 0,
            num_tracks: 0,
        };
        std::ptr::write_unaligned(bk2.as_mut_ptr() as *mut Bk2FileHeader, hdr);

        let hbink = BinkOpen(
            bk2.as_ptr(),
            0x0400_0400 | 0x0000_1000 | 0x0100_0000 | 0x0040_0000,
        );
        assert!(!hbink.is_null());
        let base = hbink as *const u8;
        assert_eq!(std::ptr::read_unaligned(base.add(0x14) as *const u32), 60);
        assert_eq!(std::ptr::read_unaligned(base.add(0x18) as *const u32), 1);

        // Write a temp .srt file and test BinkLoadSubtitles / BinkCurrentSubtitle / BinkGetSubtitleByIndex
        let tmp_srt = std::env::temp_dir().join(format!(
            "bink2w64_sub_test_{}.srt",
            std::process::id()
        ));
        let srt_body =
            "1\n00:00:00,000 --> 00:00:01,000\nHello Overwatch\n\n2\n00:00:02,000 --> 00:00:03,000\nSecond Line\n";
        std::fs::write(&tmp_srt, srt_body).unwrap();
        let mut path_nul = tmp_srt.to_string_lossy().as_bytes().to_vec();
        path_nul.push(0);

        assert_eq!(BinkLoadSubtitles(hbink, path_nul.as_ptr()), 2);
        let sub0 = BinkGetSubtitleByIndex(hbink, 0);
        assert!(!sub0.is_null());
        assert_eq!(
            CStr::from_ptr(sub0 as *const std::ffi::c_char)
                .to_str()
                .unwrap(),
            "Hello Overwatch"
        );
        let sub1 = BinkGetSubtitleByIndex(hbink, 1);
        assert!(!sub1.is_null());
        assert_eq!(
            CStr::from_ptr(sub1 as *const std::ffi::c_char)
                .to_str()
                .unwrap(),
            "Second Line"
        );
        assert!(BinkGetSubtitleByIndex(hbink, 2).is_null());
        assert!(BinkGetSubtitleByIndex(hbink, -1).is_null());

        // At frame 1 (t = 0 ms), subtitle 0 is active
        let cur0 = BinkCurrentSubtitle(hbink);
        assert!(!cur0.is_null());
        assert_eq!(
            CStr::from_ptr(cur0 as *const std::ffi::c_char)
                .to_str()
                .unwrap(),
            "Hello Overwatch"
        );

        // At frame 151 at 60fps (t = 2500 ms), subtitle 1 is active
        BinkGoto(hbink, 151, 1);
        let cur1 = BinkCurrentSubtitle(hbink);
        assert!(!cur1.is_null());
        assert_eq!(
            CStr::from_ptr(cur1 as *const std::ffi::c_char)
                .to_str()
                .unwrap(),
            "Second Line"
        );

        // At frame 91 at 60fps (t = 1500 ms), no subtitle is active
        BinkGoto(hbink, 91, 1);
        assert!(BinkCurrentSubtitle(hbink).is_null());

        let _ = std::fs::remove_file(&tmp_srt);
        BinkClose(hbink);
        BinkFreeGlobals();
    }
}

/// Build a minimal valid multi-frame in-memory `KB2n` buffer for async API ordering tests.
fn make_test_kb2n_multiframe(frames: u32, width: u32, height: u32) -> Vec<u8> {
    let hdr_size = 0x2cusize;
    let extra_hdr = 4usize; // KB2n extended header
    let table_bytes = ((frames as usize) + 1) * 4;
    let payload_start = hdr_size + extra_hdr + table_bytes;
    let frame_pkt_size = 16usize;
    let total_size = payload_start + (frames as usize) * frame_pkt_size;
    let mut buf = vec![0u8; total_size];

    let hdr = Bk2FileHeader {
        magic: *b"KB2n",
        file_size_minus_8: (total_size - 8) as u32,
        frames,
        largest_frame_size: frame_pkt_size as u32,
        frames_dup: frames,
        width,
        height,
        fps_num: 30,
        fps_den: 1,
        video_flags: 0,
        num_tracks: 0,
    };
    unsafe {
        std::ptr::write_unaligned(buf.as_mut_ptr() as *mut Bk2FileHeader, hdr);
    }
    let fo_base = hdr_size + extra_hdr;
    for f in 0..=(frames as usize) {
        let mut off = (payload_start + f * frame_pkt_size) as u32;
        if f == 0 {
            off |= 1; // frame 1 is a keyframe
        }
        buf[fo_base + f * 4..fo_base + f * 4 + 4].copy_from_slice(&off.to_le_bytes());
    }
    buf
}

#[test]
fn bink_async_wait_semantics_multi_handle_and_close_in_flight() {
    use std::sync::{Condvar, Mutex};

    unsafe {
        let stream1 = make_test_kb2n_multiframe(4, 64, 32);
        let stream2 = make_test_kb2n_multiframe(4, 64, 32);
        let h1 = BinkOpen(stream1.as_ptr(), 0x0400_0000);
        let h2 = BinkOpen(stream2.as_ptr(), 0x0400_0000);
        assert!(!h1.is_null() && !h2.is_null());

        // 1. Wait semantics: poll (us=0) and finite timeout (us>0) return 0 while in flight;
        //    independent handle h2 returns 1 immediately; once released, wait returns 1 and
        //    publishes fb.frame_num.
        let gate1 = Arc::new((Mutex::new(false), Condvar::new()));
        let done1 = Arc::new(AtomicBool::new(false));
        let inst1 = &*(h1 as *const BinkInstance);
        let fb1_before = inst1.default_frame_buffers.frame_num;

        super::bink2::player::start_gated_async_job_for_test(
            h1,
            &[0, 1],
            gate1.clone(),
            done1.clone(),
        );

        // Non-blocking poll and short timeout both see in-flight status.
        assert_eq!(BinkDoFrameAsyncWait(h1, 0), 0);
        assert_eq!(BinkDoFrameAsyncWait(h1, 5_000), 0);
        // Frame buffer index must NOT flip until completion is committed.
        assert_eq!((&*(h1 as *const BinkInstance)).default_frame_buffers.frame_num, fb1_before);

        // Handle 2 is completely independent: poll returns 1 immediately and can decode concurrently.
        assert_eq!(BinkDoFrameAsyncWait(h2, 0), 1);
        assert_eq!(BinkDoFrameAsync(h2, 2, 3), 1);
        assert_eq!(BinkDoFrameAsyncWait(h2, -1), 1);
        assert_eq!((&*(h2 as *const BinkInstance)).played_frames, 1);

        // Release handle 1's worker on a background thread while main thread blocks in Wait(-1).
        let gate1_rel = gate1.clone();
        let releaser = std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(15));
            let (lock, cvar) = &*gate1_rel;
            *lock.lock().unwrap() = true;
            cvar.notify_all();
        });
        assert_eq!(BinkDoFrameAsyncWait(h1, -1), 1);
        releaser.join().unwrap();
        assert!(done1.load(Ordering::Acquire));
        assert_ne!(
            (&*(h1 as *const BinkInstance)).default_frame_buffers.frame_num,
            fb1_before
        );
        assert_eq!((&*(h1 as *const BinkInstance)).played_frames, 1);

        // 2. BinkNextFrame while in flight must drain the worker before advancing FrameNum.
        BinkNextFrame(h1);
        let gate_nf = Arc::new((Mutex::new(false), Condvar::new()));
        let done_nf = Arc::new(AtomicBool::new(false));
        super::bink2::player::start_gated_async_job_for_test(
            h1,
            &[0],
            gate_nf.clone(),
            done_nf.clone(),
        );
        let gate_nf_rel = gate_nf.clone();
        let nf_rel = std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(15));
            let (lock, cvar) = &*gate_nf_rel;
            *lock.lock().unwrap() = true;
            cvar.notify_all();
        });
        BinkNextFrame(h1);
        nf_rel.join().unwrap();
        assert!(done_nf.load(Ordering::Acquire));
        assert_eq!(std::ptr::read_unaligned((h1 as *const u8).add(0x0c) as *const u32), 3);

        // 3. BinkClose while an async decode is still in flight must wait for completion
        //    before freeing the BinkInstance (no use-after-free).
        let gate_close = Arc::new((Mutex::new(false), Condvar::new()));
        let done_close = Arc::new(AtomicBool::new(false));
        super::bink2::player::start_gated_async_job_for_test(
            h1,
            &[0, 1],
            gate_close.clone(),
            done_close.clone(),
        );
        let gate_close_rel = gate_close.clone();
        let close_rel = std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(15));
            let (lock, cvar) = &*gate_close_rel;
            *lock.lock().unwrap() = true;
            cvar.notify_all();
        });
        BinkClose(h1);
        assert!(
            done_close.load(Ordering::Acquire),
            "BinkClose must not return until in-flight worker completes"
        );
        close_rel.join().unwrap();

        // 4. Multi-frame real async decode loop on handle 2.
        for expected_frame in 2..=4u32 {
            BinkNextFrame(h2);
            assert_eq!(BinkDoFrameAsync(h2, 0, 1), 1);
            assert_eq!(BinkDoFrameAsyncWait(h2, -1), 1);
            assert_eq!(
                std::ptr::read_unaligned((h2 as *const u8).add(0x0c) as *const u32),
                expected_frame
            );
        }
        BinkClose(h2);
    }
}

#[cfg(target_arch = "aarch64")]
#[test]
fn bink_blit_neon_yuv_to_rgb32_bit_exact() {
    unsafe {
        super::bink2::blit::verify_neon_yuv_to_rgb32_matches_scalar();
    }
}
