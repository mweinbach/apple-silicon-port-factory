//! Compact binary trace of `AK::SoundEngine` / `AK::MusicEngine` API calls
//! (milestone A2, docs/AUDIO.md §8): written by [`super::akrec`], replayed
//! offline through the original Wwise by [`super::akreplay`].
//!
//! File layout (little endian):
//!
//! ```text
//! "AW2AKT01"                                     8-byte magic + version
//! record*  = op u8 | thread u8 | frame u32 | ret u32 | len u32 | payload[len]
//! ```
//!
//! * `thread` — small per-trace index of the calling thread (0 = first thread
//!   that called a hooked API).
//! * `frame` — number of Wwise audio frames begun when the call was made
//!   (counted by a `RegisterGlobalCallback(BeginRender)` callback the recorder
//!   installs right after `SoundEngine::Init`; 0 before Init).
//! * `ret` — the call's return value (`AKRESULT`, or the `AkPlayingID` for
//!   `PostEvent`; 0 for `void`).
//! * payload — the arguments, encoded field by field in declaration order
//!   with [`Enc`]; pointed-to data is inlined (positions, aux sends, listener
//!   arrays, external-source paths). Bank contents are not inlined: the
//!   recorder writes each distinct bank once to `<trace>.banks/` (see
//!   [`bank_file_name`]) and the record carries id, size and content hash.

use std::io::{self, Write};

pub const MAGIC: &[u8; 8] = b"AW2AKT01";
pub const HEADER_LEN: usize = 1 + 1 + 4 + 4 + 4;

/// `AkWorldTransform` size (orientation front/top as 2×3 f32, position 3×f64
/// at +0x18; SetPositionInternal 0x141589660 copies 0x30 bytes).
pub const WORLD_TRANSFORM_LEN: usize = 0x30;
/// `AkAuxSendValue` {listener u64, auxBus u32, fControlValue f32}
/// (SetGameObjectAuxSendValues 0x141588c20 strides 0x10).
pub const AUX_SEND_LEN: usize = 0x10;
/// `AkOutputSettings` {shareset u32, idDevice u32, ePanningRule u32, channelConfig u32}.
pub const OUTPUT_SETTINGS_LEN: usize = 0x10;
/// `AkInitSettings` as far as GetDefaultInitSettings (0x141580a90) writes it.
pub const INIT_SETTINGS_LEN: usize = 0xb0;
/// `AkPlatformInitSettings` (GetDefaultPlatformInitSettings 0x1415edcc0).
pub const PLATFORM_SETTINGS_LEN: usize = 0x48;

macro_rules! ops {
    ($($name:ident = $v:expr),* $(,)?) => {
        /// Recorded operation (one per hooked export / overload).
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        #[repr(u8)]
        pub enum Op { $($name = $v),* }
        impl Op {
            pub fn from_u8(v: u8) -> Option<Op> {
                match v { $($v => Some(Op::$name),)* _ => None }
            }
            pub const ALL: &'static [Op] = &[$(Op::$name),*];
        }
    };
}

ops! {
    // lifecycle (recorded for reference; the replayer does its own init)
    Init = 1,                    // init[INIT_SETTINGS_LEN] plat[PLATFORM_SETTINGS_LEN]
    Term = 2,
    MusicInit = 3,               // raw AkMusicSettings bytes (u32 len + data)
    MusicTerm = 4,
    RegisterGlobalCallback = 5,  // fn u64, location u32, cookie u64, type u32, company u32, plugin u32
    RegisterBusVolumeCallback = 6, // bus u32, fn u64, cookie u64
    // frame boundary
    RenderAudio = 10,            // allow_sync u8
    // game objects / listeners
    RegisterGameObj = 11,        // obj u64
    UnregisterGameObj = 12,      // obj u64
    UnregisterAllGameObj = 13,
    SetPosition = 14,            // obj u64, transform[0x30], flags i32
    SetMultiplePositions = 15,   // obj u64, n u16, type i32, flags i32, n×transform
    SetDefaultListeners = 16,    // n u32, n×u64
    SetDistanceProbe = 17,       // listener u64, probe u64
    SetScalingFactor = 18,       // obj u64, f32
    SetObjectObstructionAndOcclusion = 19, // obj u64, listener u64, obstruction f32, occlusion f32
    SetGameObjectAuxSendValues = 20, // obj u64, n u32, n×AkAuxSendValue
    SetGameObjectOutputBusVolume = 21, // emitter u64, listener u64, f32
    // game syncs
    SetRTPCValue = 30,           // rtpc u32, value f32, obj u64, ms i32, curve i32, bypass u8
    SetRTPCValueByPlayingID = 31, // rtpc u32, value f32, playing u32, ms i32, curve i32, bypass u8
    ResetRTPCValue = 32,         // rtpc u32, obj u64, ms i32, curve i32, bypass u8
    SetSwitch = 33,              // group u32, state u32, obj u64
    SetState = 34,               // group u32, state u32
    PostTrigger = 35,            // trigger u32, obj u64
    // events
    PostEvent = 40,              // event u32, obj u64, flags u32, callback u64, cookie u64, playing u32, n u32, n×ExtSrc
    ExecuteActionOnEvent = 41,   // event u32, action i32, obj u64, ms i32, curve i32, playing u32
    ExecuteActionOnEventName = 42, // name str, action i32, obj u64, ms i32, curve i32, playing u32
    SeekOnEvent = 43,            // event u32, obj u64, ms i32, nearest_marker u8, playing u32
    SeekOnEventPercent = 44,     // event u32, obj u64, percent f32, nearest_marker u8, playing u32
    StopAll = 45,                // obj u64
    StopPlayingID = 46,          // playing u32, ms i32, curve i32
    PinEventInStreamCache = 47,  // event u32, prio i8, active_prio i8
    UnpinEventInStreamCache = 48, // event u32
    // banks
    LoadBankMemoryCopy = 50,     // ptr u64, size u32, bank_id u32 (out), hash u64
    LoadBankMemoryView = 51,     // ptr u64, size u32, bank_id u32 (out), hash u64
    UnloadBank = 52,             // bank_id u32, ptr u64, bank_type u32
    // outputs
    AddOutput = 60,              // settings[0x10], n u32, n×u64 listeners, device_id u64 (out)
    RemoveOutput = 61,           // device_id u64
    SetSpeakerAngles = 62,       // n u32, n×f32, height f32, device_id u64
}

impl Op {
    pub fn name(self) -> String {
        format!("{self:?}")
    }
}

/// One external source of a `PostEvent` (`AkExternalSourceInfo`, 0x20 bytes:
/// cookie u32 @0, idCodec u32 @4, szFile wchar* @8, pInMemory @0x10,
/// uiMemorySize u32 @0x18, idFile u32 @0x1c; copied by 0x14157f8e0).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ExtSrc {
    pub cookie: u32,
    pub codec: u32,
    pub file_id: u32,
    pub mem_size: u32,
    pub has_mem: bool,
    pub path: Vec<u16>,
}

/// Payload encoder.
#[derive(Default)]
pub struct Enc(pub Vec<u8>);

impl Enc {
    pub fn u8(&mut self, v: u8) -> &mut Self {
        self.0.push(v);
        self
    }
    pub fn u16(&mut self, v: u16) -> &mut Self {
        self.0.extend_from_slice(&v.to_le_bytes());
        self
    }
    pub fn u32(&mut self, v: u32) -> &mut Self {
        self.0.extend_from_slice(&v.to_le_bytes());
        self
    }
    pub fn i32(&mut self, v: i32) -> &mut Self {
        self.u32(v as u32)
    }
    pub fn u64(&mut self, v: u64) -> &mut Self {
        self.0.extend_from_slice(&v.to_le_bytes());
        self
    }
    pub fn f32(&mut self, v: f32) -> &mut Self {
        self.u32(v.to_bits())
    }
    pub fn bytes(&mut self, b: &[u8]) -> &mut Self {
        self.0.extend_from_slice(b);
        self
    }
    /// Length-prefixed (u32) byte string.
    pub fn blob(&mut self, b: &[u8]) -> &mut Self {
        self.u32(b.len() as u32).bytes(b)
    }
    /// Length-prefixed (u32, in units) UTF-16 string, no terminator.
    pub fn wstr(&mut self, s: &[u16]) -> &mut Self {
        self.u32(s.len() as u32);
        for c in s {
            self.u16(*c);
        }
        self
    }
    pub fn ext(&mut self, e: &ExtSrc) -> &mut Self {
        self.u32(e.cookie).u32(e.codec).u32(e.file_id).u32(e.mem_size).u8(e.has_mem as u8).wstr(&e.path)
    }
}

/// Payload decoder. Reading past the end yields an error (corrupt trace).
pub struct Dec<'a> {
    b: &'a [u8],
    p: usize,
}

pub type DResult<T> = Result<T, String>;

impl<'a> Dec<'a> {
    pub fn new(b: &'a [u8]) -> Self {
        Dec { b, p: 0 }
    }
    pub fn take(&mut self, n: usize) -> DResult<&'a [u8]> {
        if self.p + n > self.b.len() {
            return Err(format!("payload truncated (need {n} at {} of {})", self.p, self.b.len()));
        }
        let s = &self.b[self.p..self.p + n];
        self.p += n;
        Ok(s)
    }
    pub fn u8(&mut self) -> DResult<u8> {
        Ok(self.take(1)?[0])
    }
    pub fn i8(&mut self) -> DResult<i8> {
        Ok(self.u8()? as i8)
    }
    pub fn bool(&mut self) -> DResult<bool> {
        Ok(self.u8()? != 0)
    }
    pub fn u16(&mut self) -> DResult<u16> {
        Ok(u16::from_le_bytes(self.take(2)?.try_into().unwrap()))
    }
    pub fn u32(&mut self) -> DResult<u32> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }
    pub fn i32(&mut self) -> DResult<i32> {
        Ok(self.u32()? as i32)
    }
    pub fn u64(&mut self) -> DResult<u64> {
        Ok(u64::from_le_bytes(self.take(8)?.try_into().unwrap()))
    }
    pub fn f32(&mut self) -> DResult<f32> {
        Ok(f32::from_bits(self.u32()?))
    }
    pub fn blob(&mut self) -> DResult<&'a [u8]> {
        let n = self.u32()? as usize;
        self.take(n)
    }
    pub fn wstr(&mut self) -> DResult<Vec<u16>> {
        let n = self.u32()? as usize;
        let b = self.take(n * 2)?;
        Ok(b.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect())
    }
    pub fn ext(&mut self) -> DResult<ExtSrc> {
        Ok(ExtSrc {
            cookie: self.u32()?,
            codec: self.u32()?,
            file_id: self.u32()?,
            mem_size: self.u32()?,
            has_mem: self.bool()?,
            path: self.wstr()?,
        })
    }
    pub fn remaining(&self) -> usize {
        self.b.len() - self.p
    }
}

/// One decoded record (payload still encoded).
#[derive(Debug, Clone, PartialEq)]
pub struct Record<'a> {
    pub op: Op,
    pub thread: u8,
    pub frame: u32,
    pub ret: u32,
    pub payload: &'a [u8],
}

impl Record<'_> {
    pub fn dec(&self) -> Dec<'_> {
        Dec::new(self.payload)
    }
}

/// Serialize one record.
pub fn write_record(w: &mut impl Write, op: Op, thread: u8, frame: u32, ret: u32, payload: &[u8]) -> io::Result<()> {
    let mut h = [0u8; HEADER_LEN];
    h[0] = op as u8;
    h[1] = thread;
    h[2..6].copy_from_slice(&frame.to_le_bytes());
    h[6..10].copy_from_slice(&ret.to_le_bytes());
    h[10..14].copy_from_slice(&(payload.len() as u32).to_le_bytes());
    w.write_all(&h)?;
    w.write_all(payload)
}

/// Parse a whole trace. Unknown op codes are an error; a truncated final
/// record (recorder killed mid-write) is dropped with a warning count.
pub fn parse(data: &[u8]) -> Result<(Vec<Record<'_>>, usize), String> {
    if data.len() < MAGIC.len() || &data[..MAGIC.len()] != MAGIC {
        return Err("not an AW2 AK trace (bad magic)".into());
    }
    let mut p = MAGIC.len();
    let mut out = Vec::new();
    let mut truncated = 0;
    while p < data.len() {
        if p + HEADER_LEN > data.len() {
            truncated += 1;
            break;
        }
        let h = &data[p..p + HEADER_LEN];
        let op = Op::from_u8(h[0]).ok_or_else(|| format!("unknown op {} at offset {p}", h[0]))?;
        let len = u32::from_le_bytes(h[10..14].try_into().unwrap()) as usize;
        if p + HEADER_LEN + len > data.len() {
            truncated += 1;
            break;
        }
        out.push(Record {
            op,
            thread: h[1],
            frame: u32::from_le_bytes(h[2..6].try_into().unwrap()),
            ret: u32::from_le_bytes(h[6..10].try_into().unwrap()),
            payload: &data[p + HEADER_LEN..p + HEADER_LEN + len],
        });
        p += HEADER_LEN + len;
    }
    Ok((out, truncated))
}

/// FNV-1a 64 of a bank's bytes (content key for `<trace>.banks/`).
pub fn fnv1a64(b: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &x in b {
        h ^= x as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

/// Side-file name of a recorded bank.
pub fn bank_file_name(bank_id: u32, size: u32, hash: u64) -> String {
    format!("bank_{bank_id:08x}_{size}_{hash:016x}.bnk")
}

/// `<trace>.banks` directory next to a trace file.
pub fn banks_dir(trace: &std::path::Path) -> std::path::PathBuf {
    let mut s = trace.as_os_str().to_owned();
    s.push(".banks");
    s.into()
}

/// Bank ID from a bank's `BKHD` chunk (u32 at +12 of the file: tag, size,
/// version, id). `None` if the buffer is not a bank.
pub fn bank_id_from_header(b: &[u8]) -> Option<u32> {
    if b.len() >= 16 && &b[0..4] == b"BKHD" {
        Some(u32::from_le_bytes(b[12..16].try_into().unwrap()))
    } else {
        None
    }
}

/// Per-op counts, for logs and reports.
pub fn summarize(recs: &[Record<'_>]) -> String {
    let mut counts = std::collections::BTreeMap::<Op, usize>::new();
    for r in recs {
        *counts.entry(r.op).or_default() += 1;
    }
    let frames = recs.iter().map(|r| r.frame).max().unwrap_or(0);
    let threads = recs.iter().map(|r| r.thread).max().map_or(0, |t| t as usize + 1);
    let mut s = format!("{} records, {} threads, last frame {frames}:", recs.len(), threads);
    for (op, n) in counts {
        s.push_str(&format!(" {op:?}={n}"));
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        let mut buf = MAGIC.to_vec();
        let mut e = Enc::default();
        e.u32(0xdead).f32(1.5).u64(7).i32(-3).i32(4).u8(1);
        write_record(&mut buf, Op::SetRTPCValue, 0, 12, 1, &e.0).unwrap();
        let ext = ExtSrc { cookie: 9, codec: 4, file_id: 0, mem_size: 0, has_mem: false, path: "a\\b.wem".encode_utf16().collect() };
        let mut e = Enc::default();
        e.u32(1).u64(2).u32(0x100001).u64(0x1234).u64(0).u32(0).u32(1).ext(&ext);
        write_record(&mut buf, Op::PostEvent, 1, 13, 77, &e.0).unwrap();
        write_record(&mut buf, Op::RenderAudio, 0, 14, 1, &[1]).unwrap();
        // truncated tail
        buf.extend_from_slice(&[Op::StopAll as u8, 0, 0]);
        let (recs, trunc) = parse(&buf).unwrap();
        assert_eq!(trunc, 1);
        assert_eq!(recs.len(), 3);
        assert_eq!(recs[0].op, Op::SetRTPCValue);
        let mut d = recs[0].dec();
        assert_eq!((d.u32().unwrap(), d.f32().unwrap(), d.u64().unwrap(), d.i32().unwrap()), (0xdead, 1.5, 7, -3));
        assert_eq!(recs[1].ret, 77);
        assert_eq!(recs[1].thread, 1);
        let mut d = recs[1].dec();
        for _ in 0..1 {
            d.u32().unwrap();
            d.u64().unwrap();
            d.u32().unwrap();
            d.u64().unwrap();
            d.u64().unwrap();
            d.u32().unwrap();
        }
        assert_eq!(d.u32().unwrap(), 1);
        assert_eq!(d.ext().unwrap(), ext);
        assert_eq!(d.remaining(), 0);
        assert_eq!(recs[2].frame, 14);
        assert!(summarize(&recs).contains("PostEvent=1"));
    }

    #[test]
    fn ops_are_unique_and_decodable() {
        for &op in Op::ALL {
            assert_eq!(Op::from_u8(op as u8), Some(op));
        }
        assert_eq!(Op::from_u8(0), None);
    }

    #[test]
    fn bank_helpers() {
        let mut b = b"BKHD".to_vec();
        b.extend_from_slice(&[0x18, 0, 0, 0, 0x8c, 0, 0, 0, 0x78, 0x56, 0x34, 0x12]);
        assert_eq!(bank_id_from_header(&b), Some(0x12345678));
        assert_eq!(bank_id_from_header(b"RIFF...."), None);
        assert_eq!(fnv1a64(b""), 0xcbf2_9ce4_8422_2325);
        assert_eq!(bank_file_name(1, 2, 3), "bank_00000001_2_0000000000000003.bnk");
        assert_eq!(banks_dir(std::path::Path::new("/x/t.akt")), std::path::PathBuf::from("/x/t.akt.banks"));
    }
}
