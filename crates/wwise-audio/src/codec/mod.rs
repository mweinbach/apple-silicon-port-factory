//! Decoders for the .wem codecs Alan Wake 2 ships (see docs/AUDIO.md inventory).

pub mod bits;
pub mod lewton;
pub mod vorbis;
pub mod pcm;

use crate::wem::{Codec, Wem};

/// Decoded PCM, interleaved f32 in [-1, 1].
pub struct Decoded {
    pub channels: u16,
    pub sample_rate: u32,
    pub samples: Vec<f32>,
    pub detail: String,
}

/// Decodes a complete .wem buffer (Vorbis floor: [`vorbis::default_floor`]).
pub fn decode_wem(buf: &[u8]) -> Result<Decoded, String> {
    decode_wem_with(buf, vorbis::default_floor())
}

/// Decodes a complete .wem buffer with an explicit Vorbis floor1 synthesis.
pub fn decode_wem_with(buf: &[u8], floor: vorbis::FloorSynthesis) -> Result<Decoded, String> {
    let wem = Wem::parse(buf).map_err(|e| e.to_string())?;
    match wem.codec {
        Codec::Vorbis => {
            let mut dec = vorbis::WwiseVorbisDecoder::new(buf, &wem).map_err(|e| e.to_string())?;
            dec.floor = floor;
            let lib = dec.library;
            let samples = dec.decode_all().map_err(|e| e.to_string())?;
            Ok(Decoded {
                channels: wem.channels,
                sample_rate: wem.sample_rate,
                samples,
                detail: format!("Wwise Vorbis, codebooks {:?}, floor {:?}", lib, floor),
            })
        }
        Codec::Pcm => {
            let samples = pcm::decode_pcm16(buf, &wem)?;
            Ok(Decoded { channels: wem.channels, sample_rate: wem.sample_rate, samples, detail: "PCM16".into() })
        }
        other => Err(format!("codec {} not implemented (not present in AW2 data per inventory)", other.name())),
    }
}
