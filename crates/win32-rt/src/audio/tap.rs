//! PCM tap: `PORT_AUDIO_TAP=path.wav` (legacy alias `AW2_AUDIO_TAP`) records
//! the PCM audio submitted by render backends — WASAPI `IAudioRenderClient`,
//! `ISpatialAudioObjectRenderStream`, `IXAudio2MasteringVoice`, `BinkAudioPlayer`,
//! and `akrec` (`AUDCLNT_BUFFERFLAGS_SILENT` WASAPI buffers are written as zeros
//! because that is what the client asked to play).
//!
//! The first stream writes to `path`, later streams to `path.<n>.wav`. Next to
//! each WAV, `<wav>.tsv` logs one line per submitted block
//! (`seq frames flags padding_before frame_offset qpc_100ns`) for aligning the
//! tap against other captures (e.g. Wwise's `StartOutputCapture`).
//!
//! Writes happen on a background thread; the RIFF/data sizes are patched after
//! every batch so the file stays valid even if the process exits via
//! `ExitProcess` without running destructors.

use super::format::WaveFormat;
use std::fs::File;
use std::io::{BufWriter, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::mpsc;

enum Msg {
    Data(Vec<u8>),
    Zeros(usize),
    Line(String),
}

pub struct Tap {
    tx: Option<mpsc::Sender<Msg>>,
    thread: Option<std::thread::JoinHandle<()>>,
    pub path: PathBuf,
    seq: u64,
    frames: u64,
}

static STREAMS: AtomicU32 = AtomicU32::new(0);
static OVERRIDE: std::sync::Mutex<Option<PathBuf>> = std::sync::Mutex::new(None);

/// Tap the next initialized stream to `p`, regardless of `PORT_AUDIO_TAP` /
/// `AW2_AUDIO_TAP` (unit tests, audio harness). One-shot.
pub fn override_next(p: Option<PathBuf>) {
    *OVERRIDE.lock().unwrap() = p;
}

fn select_env_base(
    port_var: Option<std::ffi::OsString>,
    legacy_var: Option<std::ffi::OsString>,
) -> Option<PathBuf> {
    port_var
        .filter(|v| !v.is_empty())
        .or_else(|| legacy_var.filter(|v| !v.is_empty()))
        .map(PathBuf::from)
}

/// Path for the next stream, if tapping is enabled.
pub fn next_path() -> Option<PathBuf> {
    if let Some(p) = OVERRIDE.lock().unwrap().take() {
        return Some(p);
    }
    let base = select_env_base(
        std::env::var_os("PORT_AUDIO_TAP"),
        std::env::var_os("AW2_AUDIO_TAP"),
    )?;
    let n = STREAMS.fetch_add(1, Ordering::Relaxed);
    Some(if n == 0 { base } else { numbered(&base, n) })
}

fn numbered(base: &Path, n: u32) -> PathBuf {
    let stem = base.file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
    base.with_file_name(format!("{stem}.{n}.wav"))
}

pub fn sidecar_path(wav: &Path) -> PathBuf {
    let mut s = wav.as_os_str().to_owned();
    s.push(".tsv");
    PathBuf::from(s)
}

fn header(fmt: &WaveFormat, data_len: u32) -> Vec<u8> {
    let mut h = Vec::with_capacity(68);
    h.extend_from_slice(b"RIFF");
    h.extend_from_slice(&(4 + 8 + 40 + 8 + data_len).to_le_bytes());
    h.extend_from_slice(b"WAVEfmt ");
    h.extend_from_slice(&40u32.to_le_bytes());
    h.extend_from_slice(&fmt.extensible_bytes());
    h.extend_from_slice(b"data");
    h.extend_from_slice(&data_len.to_le_bytes());
    h
}

/// Offset of the PCM data in files written by [`Tap`].
pub const DATA_OFFSET: u64 = 12 + 8 + 40 + 8;

impl Tap {
    pub fn create(path: PathBuf, fmt: WaveFormat) -> std::io::Result<Tap> {
        Self::create_inner(path, fmt, None)
    }

    pub fn create_tagged(path: PathBuf, fmt: WaveFormat, tag: &str) -> std::io::Result<Tap> {
        Self::create_inner(path, fmt, Some(tag))
    }

    fn create_inner(path: PathBuf, fmt: WaveFormat, tag: Option<&str>) -> std::io::Result<Tap> {
        if let Some(d) = path.parent() {
            if !d.as_os_str().is_empty() {
                std::fs::create_dir_all(d)?;
            }
        }
        let mut f = File::create(&path)?;
        f.write_all(&header(&fmt, 0))?;
        let mut side = BufWriter::new(File::create(sidecar_path(&path))?);
        writeln!(side, "seq\tframes\tflags\tpadding_before\tframe_offset\tqpc_100ns")?;
        let (tx, rx) = mpsc::channel::<Msg>();
        let p2 = path.clone();
        let thread = std::thread::Builder::new()
            .name("wwise-audio-tap".into())
            .spawn(move || {
                let mut len: u64 = 0;
                let mut warned = false;
                let mut batch: Vec<u8> = Vec::new();
                while let Ok(first) = rx.recv() {
                    batch.clear();
                    let mut handle = |m: Msg, batch: &mut Vec<u8>| match m {
                        Msg::Data(d) => batch.extend_from_slice(&d),
                        Msg::Zeros(n) => batch.resize(batch.len() + n, 0),
                        Msg::Line(l) => {
                            let _ = side.write_all(l.as_bytes());
                        }
                    };
                    handle(first, &mut batch);
                    while let Ok(more) = rx.try_recv() {
                        handle(more, &mut batch);
                    }
                    let r = (|| -> std::io::Result<()> {
                        f.write_all(&batch)?;
                        len += batch.len() as u64;
                        let l = len.min(u32::MAX as u64 - 64) as u32;
                        f.seek(SeekFrom::Start(4))?;
                        f.write_all(&(4 + 8 + 40 + 8 + l).to_le_bytes())?;
                        f.seek(SeekFrom::Start(DATA_OFFSET - 4))?;
                        f.write_all(&l.to_le_bytes())?;
                        f.seek(SeekFrom::End(0))?;
                        side.flush()
                    })();
                    if let Err(e) = r {
                        if !warned {
                            log::error!(target: "audio", "audio tap {}: write failed: {e}", p2.display());
                            warned = true;
                        }
                    }
                }
                let _ = f.flush();
                let _ = side.flush();
            })?;
        match tag {
            Some(t) => log::info!(
                target: "audio",
                "audio tap [{t}] -> {} ({} Hz, {} ch, {} bit {:?})",
                path.display(), fmt.rate, fmt.channels, fmt.bits, fmt.sample
            ),
            None => log::info!(
                target: "audio",
                "audio tap -> {} ({} Hz, {} ch, {} bit {:?})",
                path.display(), fmt.rate, fmt.channels, fmt.bits, fmt.sample
            ),
        }
        Ok(Tap { tx: Some(tx), thread: Some(thread), path, seq: 0, frames: 0 })
    }

    /// Record one ReleaseBuffer: `bytes = None` means AUDCLNT_BUFFERFLAGS_SILENT
    /// (`zero_len` zero bytes are written).
    pub fn submit(&mut self, frames: u32, flags: u32, padding_before: u32, bytes: Option<&[u8]>, zero_len: usize) {
        let Some(tx) = &self.tx else { return };
        let qpc = crate::kernel32::time::interrupt_time_100ns();
        let _ = tx.send(Msg::Line(format!("{}\t{frames}\t{flags}\t{padding_before}\t{}\t{qpc}\n", self.seq, self.frames)));
        let _ = tx.send(match bytes {
            Some(b) => Msg::Data(b.to_vec()),
            None => Msg::Zeros(zero_len),
        });
        self.seq += 1;
        self.frames += frames as u64;
    }

    /// Record one interleaved IEEE-754 `f32` audio block.
    pub fn submit_f32(&mut self, frames: u32, padding_before: u32, samples: &[f32]) {
        if frames == 0 || samples.is_empty() {
            return;
        }
        let Some(tx) = &self.tx else { return };
        let qpc = crate::kernel32::time::interrupt_time_100ns();
        let mut bytes = Vec::with_capacity(samples.len() * 4);
        for &s in samples {
            bytes.extend_from_slice(&s.to_le_bytes());
        }
        let _ = tx.send(Msg::Line(format!("{}\t{frames}\t0\t{padding_before}\t{}\t{qpc}\n", self.seq, self.frames)));
        let _ = tx.send(Msg::Data(bytes));
        self.seq += 1;
        self.frames += frames as u64;
    }

    /// Flush and close (waits for the writer thread).
    pub fn finish(mut self) {
        self.close();
    }

    fn close(&mut self) {
        self.tx.take();
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

impl Drop for Tap {
    fn drop(&mut self) {
        self.close();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_valid_wav_and_sidecar() {
        let dir = std::env::temp_dir().join(format!("aw2-tap-test-{}", std::process::id()));
        let p = dir.join("t.wav");
        let fmt = WaveFormat::float_mix(48000, 2, 3);
        let mut t = Tap::create(p.clone(), fmt).unwrap();
        let a: Vec<u8> = [0.5f32, -0.5].iter().flat_map(|v| v.to_le_bytes()).collect();
        t.submit(1, 0, 0, Some(&a), 0);
        t.submit(1, 2, 1, None, 8);
        t.finish();
        let b = std::fs::read(&p).unwrap();
        assert_eq!(&b[0..4], b"RIFF");
        assert_eq!(u32::from_le_bytes(b[4..8].try_into().unwrap()) as usize, b.len() - 8);
        assert_eq!(u32::from_le_bytes(b[64..68].try_into().unwrap()), 16);
        assert_eq!(&b[68..76], &a[..]);
        assert_eq!(&b[76..84], &[0u8; 8]);
        let side = std::fs::read_to_string(sidecar_path(&p)).unwrap();
        let lines: Vec<&str> = side.lines().collect();
        assert_eq!(lines.len(), 3);
        assert!(lines[2].starts_with("1\t1\t2\t1\t1\t"), "{}", lines[2]);
        assert_eq!(numbered(Path::new("out/x.wav"), 2), PathBuf::from("out/x.2.wav"));
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn env_var_prefers_port_audio_tap_with_legacy_fallback_and_submit_f32() {
        use std::ffi::OsString;

        assert_eq!(
            select_env_base(Some(OsString::from("port.wav")), Some(OsString::from("legacy.wav"))),
            Some(PathBuf::from("port.wav"))
        );
        assert_eq!(
            select_env_base(Some(OsString::from("")), Some(OsString::from("legacy.wav"))),
            Some(PathBuf::from("legacy.wav"))
        );
        assert_eq!(
            select_env_base(None, Some(OsString::from("legacy.wav"))),
            Some(PathBuf::from("legacy.wav"))
        );
        assert_eq!(select_env_base(None, Some(OsString::from(""))), None);
        assert_eq!(select_env_base(None, None), None);

        let dir = std::env::temp_dir().join(format!("port-tap-f32-test-{}", std::process::id()));
        let p = dir.join("f32.wav");
        let fmt = WaveFormat::float_mix(48_000, 2, 0x3);
        let mut t = Tap::create_tagged(p.clone(), fmt, "test").unwrap();
        let samples = [0.25f32, -0.75, 0.5, -0.25];
        t.submit_f32(0, 0, &samples); // ignored
        t.submit_f32(2, 12, &samples);
        t.finish();

        let b = std::fs::read(&p).unwrap();
        assert_eq!(u32::from_le_bytes(b[64..68].try_into().unwrap()), 16);
        let pcm: Vec<f32> = b[DATA_OFFSET as usize..]
            .chunks_exact(4)
            .map(|c| f32::from_le_bytes(c.try_into().unwrap()))
            .collect();
        assert_eq!(pcm, samples);
        let side = std::fs::read_to_string(sidecar_path(&p)).unwrap();
        let lines: Vec<&str> = side.lines().collect();
        assert_eq!(lines.len(), 2);
        assert!(lines[1].starts_with("0\t2\t0\t12\t0\t"), "{}", lines[1]);
        let _ = std::fs::remove_dir_all(dir);
    }
}
