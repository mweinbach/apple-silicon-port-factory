//! Wwise RIFF (.wem) container parsing.
//!
//! Format knowledge from vgmstream `src/meta/wwise.c` (`parse_wwise`) and hcs's ww2ogg; every
//! field is cross-checked by `wwise-audio inventory` against all 93k .wem files of the game.

use crate::reader::{ParseError, Reader, Result};

/// Wwise codec tags found in `fmt .wFormatTag` (vgmstream `parse_wwise`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Codec {
    Pcm,         // 0x0001 / 0xFFFE
    WwiseIma,    // 0x0002 (PC: Wwise IMA ADPCM)
    Vorbis,      // 0xFFFF
    Opus,        // 0x3040
    OpusWem,     // 0x3041 ("OPUS_WEM", 2019.2.3+)
    PtAdpcm,     // 0x8311 (2019.1+)
    Other(u16),
}

impl Codec {
    pub fn from_tag(tag: u16) -> Self {
        match tag {
            0x0001 | 0xFFFE => Codec::Pcm,
            0x0002 => Codec::WwiseIma,
            0xFFFF => Codec::Vorbis,
            0x3040 => Codec::Opus,
            0x3041 => Codec::OpusWem,
            0x8311 => Codec::PtAdpcm,
            t => Codec::Other(t),
        }
    }
    pub fn name(&self) -> String {
        match self {
            Codec::Pcm => "PCM".into(),
            Codec::WwiseIma => "Wwise IMA ADPCM".into(),
            Codec::Vorbis => "Wwise Vorbis".into(),
            Codec::Opus => "Wwise Opus (0x3040)".into(),
            Codec::OpusWem => "Wwise Opus WEM (0x3041)".into(),
            Codec::PtAdpcm => "PTADPCM".into(),
            Codec::Other(t) => format!("unknown 0x{:04x}", t),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Chunk {
    pub fourcc: [u8; 4],
    pub offset: usize,
    pub size: usize,
}

/// Wwise Vorbis header extension (`fmt ` extra data, extra_size 0x30 = Wwise >= 2012 / "v62").
#[derive(Debug, Clone, Default)]
pub struct VorbisInfo {
    pub num_samples: u32,
    /// Size of the seek table at the start of `data` (= offset of the setup packet).
    pub seek_table_size: u32,
    /// Offset of the first audio packet within `data`.
    pub audio_offset: u32,
    /// Vorbis blocksize exponents (spec blocksize_0 = short, blocksize_1 = long).
    pub blocksize_0_exp: u8,
    pub blocksize_1_exp: u8,
    /// Raw 0x30-byte extension (fields at +0x04..+0x0f and +0x18..+0x27 not yet identified;
    /// candidates per ww2ogg/vgmstream comments: mod signal, loop packet offsets, uid).
    pub raw: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Wem {
    pub format_tag: u16,
    pub codec: Codec,
    pub channels: u16,
    pub sample_rate: u32,
    pub avg_bytes_per_sec: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
    pub extra_size: u16,
    /// Raw channel layout word (fmt+0x14). For Wwise >= 2018: bits 0-7 channels, 8-11 config
    /// type (0 none, 1 standard, 2 ambisonic), 12-31 speaker mask.
    pub channel_layout_raw: u32,
    pub channel_config_type: u8,
    pub channel_mask: u32,
    pub fmt: Chunk,
    pub data: Chunk,
    pub chunks: Vec<Chunk>,
    pub loop_points: Option<(u32, u32)>,
    pub vorbis: Option<VorbisInfo>,
    /// `data` extends past the end of the buffer (in-bank prefetch copy of a streamed file).
    pub truncated: bool,
    /// `akd ` chunk as four f32 (located by AK's RIFF parser `FUN_1415cf650`, which hands its
    /// size/pointer to the source; `FUN_1415cea00` keeps it at source+0x10). The first two are
    /// linear gains the oracle applies to the voice (see `Wem::akd_gain`).
    pub akd: Option<[f32; 4]>,
}

impl Wem {
    pub fn parse(buf: &[u8]) -> Result<Wem> {
        let mut r = Reader::new(buf);
        let riff = r.fourcc()?;
        if &riff != b"RIFF" {
            return r.err(format!("not a RIFF wem ({:?})", String::from_utf8_lossy(&riff)));
        }
        let _riff_size = r.u32()?; // ignored by AK's parser (vgmstream note)
        let wave = r.fourcc()?;
        if &wave != b"WAVE" {
            return r.err("RIFF type is not WAVE");
        }
        let mut chunks = Vec::new();
        // Wwise chunks are even-aligned and don't add RIFF padding bytes (vgmstream note).
        while r.remaining() >= 8 {
            let fourcc = r.fourcc()?;
            let size = r.u32()? as usize;
            let offset = r.pos();
            chunks.push(Chunk { fourcc, offset, size });
            if size > r.remaining() {
                // Only legal for `data` (prefetch truncation); stop scanning.
                break;
            }
            r.skip(size)?;
        }
        let find = |cc: &[u8; 4]| chunks.iter().find(|c| &c.fourcc == cc).cloned();
        let fmt = find(b"fmt ").ok_or(ParseError { offset: 0, msg: "no fmt chunk".into() })?;
        let data = find(b"data").ok_or(ParseError { offset: 0, msg: "no data chunk".into() })?;
        if fmt.size < 0x10 {
            return Err(ParseError { offset: fmt.offset, msg: "fmt too small".into() });
        }
        let mut f = Reader::with_base(&buf[fmt.offset..(fmt.offset + fmt.size).min(buf.len())], fmt.offset);
        let format_tag = f.u16()?;
        let channels = f.u16()?;
        let sample_rate = f.u32()?;
        let avg_bytes_per_sec = f.u32()?;
        let block_align = f.u16()?;
        let bits_per_sample = f.u16()?;
        let extra_size = if fmt.size > 0x10 { f.u16()? } else { 0 };
        let mut channel_layout_raw = 0;
        let mut channel_config_type = 0;
        let mut channel_mask = 0;
        if extra_size >= 6 && fmt.size >= 0x18 {
            f.skip(2)?;
            channel_layout_raw = f.u32()?;
            if (channel_layout_raw & 0xFF) as u16 == channels {
                channel_config_type = ((channel_layout_raw >> 8) & 0xF) as u8;
                channel_mask = channel_layout_raw >> 12;
            } else {
                channel_mask = channel_layout_raw;
            }
        }
        let codec = Codec::from_tag(format_tag);

        let mut loop_points = None;
        if let Some(smpl) = find(b"smpl") {
            if smpl.size >= 0x34 && smpl.offset + smpl.size <= buf.len() {
                let mut s = Reader::with_base(&buf[smpl.offset..smpl.offset + smpl.size], smpl.offset);
                s.skip(0x1c)?;
                let loop_count = s.u32()?;
                s.skip(4 + 4)?; // sampler data, cue point id
                let loop_type = s.u32()?;
                let start = s.u32()?;
                let end = s.u32()?;
                if loop_count == 1 && loop_type == 0 {
                    loop_points = Some((start, end + 1));
                }
            }
        }

        let mut vorbis = None;
        if codec == Codec::Vorbis {
            // vgmstream: extra_offset = fmt + 0x18 for >= v56 layouts. Only the 0x30 layout
            // (Wwise 2012+, "v62"/"v56") is supported - that's the only one AW2 ships.
            if extra_size != 0x30 || fmt.size < 0x42 {
                return Err(ParseError {
                    offset: fmt.offset,
                    msg: format!("unsupported Wwise Vorbis fmt (extra_size {:#x}, fmt size {:#x})", extra_size, fmt.size),
                });
            }
            let raw = &buf[fmt.offset + 0x12..fmt.offset + 0x42];
            let mut e = Reader::with_base(&buf[fmt.offset + 0x18..fmt.offset + 0x42], fmt.offset + 0x18);
            let num_samples = e.u32()?; // +0x00
            e.skip(0x0c)?;
            let seek_table_size = e.u32()?; // +0x10
            let audio_offset = e.u32()?; // +0x14
            e.skip(0x10)?;
            let bs0 = e.u8()?; // +0x28 (vgmstream: "blocksize_1_exp", low nibble of the ident byte)
            let bs1 = e.u8()?; // +0x29
            vorbis = Some(VorbisInfo {
                num_samples,
                seek_table_size,
                audio_offset,
                blocksize_0_exp: bs0,
                blocksize_1_exp: bs1,
                raw: raw.to_vec(),
            });
        }

        let mut akd = None;
        if let Some(c) = find(b"akd ") {
            if c.size >= 16 && c.offset + 16 <= buf.len() {
                let f = |i: usize| f32::from_le_bytes(buf[c.offset + 4 * i..c.offset + 4 * i + 4].try_into().unwrap());
                akd = Some([f(0), f(1), f(2), f(3)]);
            }
        }

        let truncated = data.offset + data.size > buf.len();
        Ok(Wem {
            format_tag,
            codec,
            channels,
            sample_rate,
            avg_bytes_per_sec,
            block_align,
            bits_per_sample,
            extra_size,
            channel_layout_raw,
            channel_config_type,
            channel_mask,
            fmt,
            data,
            chunks,
            loop_points,
            vorbis,
            truncated,
            akd,
        })
    }

    /// Linear gain the original applies for the `akd ` chunk: the product of its first two
    /// floats [V data: oracle/ours ratio = akd[0] on 3313869370 (2.2273) and = akd[1] on
    /// 361435055 (1.4142), with the other field 1.0; A: which AK stage multiplies it in, the
    /// consumer of source+0x10 is not traced yet]. 1.0 without the chunk.
    pub fn akd_gain(&self) -> f32 {
        self.akd.map_or(1.0, |a| a[0] * a[1])
    }

    pub fn chunk_names(&self) -> String {
        self.chunks
            .iter()
            .map(|c| String::from_utf8_lossy(&c.fourcc).trim().to_string())
            .collect::<Vec<_>>()
            .join(",")
    }

    pub fn num_samples(&self) -> Option<u32> {
        match self.codec {
            Codec::Vorbis => self.vorbis.as_ref().map(|v| v.num_samples),
            Codec::Pcm if self.bits_per_sample == 16 && self.channels > 0 => {
                Some((self.data.size / (2 * self.channels as usize)) as u32)
            }
            _ => None,
        }
    }
}
