//! Little-endian PCM16 (`fmt` tag 0x0001/0xFFFE). Present in Wwise generally; the AW2
//! inventory decides whether it is reachable (see docs/AUDIO.md).

use crate::wem::Wem;

pub fn decode_pcm16(buf: &[u8], wem: &Wem) -> Result<Vec<f32>, String> {
    if wem.bits_per_sample != 16 {
        return Err(format!("PCM with {} bits not supported", wem.bits_per_sample));
    }
    let end = (wem.data.offset + wem.data.size).min(buf.len());
    let data = &buf[wem.data.offset..end];
    Ok(data.chunks_exact(2).map(|c| i16::from_le_bytes([c[0], c[1]]) as f32 / 32768.0).collect())
}
