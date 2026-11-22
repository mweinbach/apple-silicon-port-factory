//! Minimal RIFF WAVE writer (PCM16 or IEEE float32).

use std::io::Write;

pub fn write_wav<W: Write>(mut w: W, channels: u16, rate: u32, samples: &[f32], float: bool) -> std::io::Result<()> {
    let (tag, bits): (u16, u16) = if float { (3, 32) } else { (1, 16) };
    let block = channels * bits / 8;
    let data_len = (samples.len() * (bits as usize / 8)) as u32;
    w.write_all(b"RIFF")?;
    w.write_all(&(36 + data_len).to_le_bytes())?;
    w.write_all(b"WAVEfmt ")?;
    w.write_all(&16u32.to_le_bytes())?;
    w.write_all(&tag.to_le_bytes())?;
    w.write_all(&channels.to_le_bytes())?;
    w.write_all(&rate.to_le_bytes())?;
    w.write_all(&(rate * block as u32).to_le_bytes())?;
    w.write_all(&block.to_le_bytes())?;
    w.write_all(&bits.to_le_bytes())?;
    w.write_all(b"data")?;
    w.write_all(&data_len.to_le_bytes())?;
    let mut buf = Vec::with_capacity(data_len as usize);
    for &s in samples {
        if float {
            buf.extend_from_slice(&s.to_le_bytes());
        } else {
            buf.extend_from_slice(&f32_to_i16(s).to_le_bytes());
        }
    }
    w.write_all(&buf)
}

/// Float to PCM16 the way vgmstream's `sbuf` does for float codecs
/// (`CONV_FLT_S16`: `clamp16(lrintf(x * 32767.0f))`, round-half-even) — vgmstream `src/base/sbuf.c`.
pub fn f32_to_i16(s: f32) -> i16 {
    let v = (s * 32767.0).round_ties_even();
    v.clamp(-32768.0, 32767.0) as i16
}

/// Parses a PCM16 or float32 WAV produced by vgmstream-cli / this crate into f32 samples.
pub fn read_wav(buf: &[u8]) -> Result<(u16, u32, Vec<f32>), String> {
    if buf.len() < 12 || &buf[0..4] != b"RIFF" || &buf[8..12] != b"WAVE" {
        return Err("not a WAV".into());
    }
    let mut p = 12;
    let (mut ch, mut rate, mut tag, mut bits) = (0u16, 0u32, 0u16, 0u16);
    while p + 8 <= buf.len() {
        let id = &buf[p..p + 4];
        let sz = u32::from_le_bytes(buf[p + 4..p + 8].try_into().unwrap()) as usize;
        let body = &buf[p + 8..(p + 8 + sz).min(buf.len())];
        if id == b"fmt " {
            tag = u16::from_le_bytes([body[0], body[1]]);
            ch = u16::from_le_bytes([body[2], body[3]]);
            rate = u32::from_le_bytes(body[4..8].try_into().unwrap());
            bits = u16::from_le_bytes([body[14], body[15]]);
            if tag == 0xFFFE && body.len() >= 26 {
                tag = u16::from_le_bytes([body[24], body[25]]);
            }
        } else if id == b"data" {
            let s: Vec<f32> = match (tag, bits) {
                (1, 16) => body.chunks_exact(2).map(|c| i16::from_le_bytes([c[0], c[1]]) as f32 / 32768.0).collect(),
                (3, 32) => body.chunks_exact(4).map(|c| f32::from_le_bytes(c.try_into().unwrap())).collect(),
                _ => return Err(format!("unsupported WAV format tag {} bits {}", tag, bits)),
            };
            return Ok((ch, rate, s));
        }
        p += 8 + sz + (sz & 1);
    }
    Err("no data chunk".into())
}
