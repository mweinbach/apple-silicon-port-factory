//! Wwise Vorbis decoder.
//!
//! Wwise stores a stripped Vorbis stream: no Ogg pages, no identification/comment headers, a
//! reduced setup header whose codebooks are referenced by index into a codebook library shipped
//! with the SDK, and (for blocksize_0 != blocksize_1) audio packets without the packet-type bit
//! and the long-window prev/next flags. We rebuild a standard Vorbis I identification header,
//! setup header and audio packets exactly like vgmstream's `vorbis_custom_utils_wwise.c`
//! (ww2ogg algorithm by hcs) and decode them with our lewton fork (`codec::lewton`), whose floor1
//! synthesis defaults to Wwise's own AVX2 kernel ([`FloorSynthesis::AkAvx2`]) so the decode matches
//! the original engine; `AW2_VORBIS_FLOOR=libvorbis` (or [`FloorSynthesis::Libvorbis`]) restores
//! the spec / libvorbis / vgmstream output.
//!
//! Only the layout used by Alan Wake 2 (Wwise 2022.1, fmt extra 0x30, 2-byte packet headers,
//! "v62": aoTuV 6.03 codebook library, modified packets) plus the "v56" fallback (standard
//! codebook library) are implemented — vgmstream tries the same two in the same order.

use crate::codec::bits::{BitReader, BitWriter};
use crate::codec::lewton;
pub use crate::codec::lewton::audio::FloorSynthesis;
use crate::wem::Wem;

mod codebooks {
    include!(concat!(env!("OUT_DIR"), "/wwise_codebooks.rs"));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodebookLibrary {
    AoTuV603,
    Standard,
}

#[derive(Debug)]
pub struct VorbisError(pub String);

impl std::fmt::Display for VorbisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for VorbisError {}

/// Process default floor synthesis: `AW2_VORBIS_FLOOR=libvorbis|ak` (default `ak`).
pub fn default_floor() -> FloorSynthesis {
    static F: std::sync::OnceLock<FloorSynthesis> = std::sync::OnceLock::new();
    *F.get_or_init(|| match std::env::var("AW2_VORBIS_FLOOR").as_deref() {
        Ok("libvorbis") | Ok("vorbis") | Ok("spec") => FloorSynthesis::Libvorbis,
        _ => FloorSynthesis::AkAvx2,
    })
}

fn err<T>(m: impl Into<String>) -> Result<T, VorbisError> {
    Err(VorbisError(m.into()))
}

fn ilog(mut v: u32) -> u32 {
    let mut r = 0;
    while v != 0 {
        r += 1;
        v >>= 1;
    }
    r
}

/// `_book_maptype1_quantvals` (Tremor), as used by ww2ogg.
fn book_maptype1_quantvals(entries: u32, dimensions: u32) -> u32 {
    let bits = ilog(entries) as i64;
    let mut vals = (entries as i64) >> ((bits - 1) * (dimensions as i64 - 1) / dimensions as i64);
    loop {
        let mut acc: u64 = 1;
        let mut acc1: u64 = 1;
        for _ in 0..dimensions {
            acc = acc.saturating_mul(vals as u64);
            acc1 = acc1.saturating_mul(vals as u64 + 1);
        }
        if acc <= entries as u64 && acc1 > entries as u64 {
            return vals as u32;
        } else if acc > entries as u64 {
            vals -= 1;
        } else {
            vals += 1;
        }
    }
}

fn lookup_codebook(lib: CodebookLibrary, id: u32) -> Option<&'static [u8]> {
    let list = match lib {
        CodebookLibrary::AoTuV603 => codebooks::AOTUV603,
        CodebookLibrary::Standard => codebooks::STANDARD,
    };
    list.iter().find(|(i, _)| *i == id).map(|(_, b)| *b)
}

/// Rebuilds one packed Wwise codebook into a Vorbis codebook (`ww2ogg_codebook_library_rebuild`).
fn rebuild_codebook(ow: &mut BitWriter, cb: &[u8]) -> Result<(), VorbisError> {
    let mut iw = BitReader::new(cb);
    let g = |iw: &mut BitReader, n: u32| iw.get(n).ok_or_else(|| VorbisError("codebook truncated".into()));
    ow.put(24, 0x564342); // "BCV"
    let dimensions = g(&mut iw, 4)?;
    ow.put(16, dimensions);
    let entries = g(&mut iw, 14)?;
    ow.put(24, entries);
    let ordered = g(&mut iw, 1)?;
    ow.put(1, ordered);
    if ordered != 0 {
        let initial_length = g(&mut iw, 5)?;
        ow.put(5, initial_length);
        let mut current = 0u32;
        while current < entries {
            let nbits = ilog(entries - current);
            let number = g(&mut iw, nbits)?;
            ow.put(nbits, number);
            current += number;
        }
        if current > entries {
            return err("codebook: current_entry out of range");
        }
    } else {
        let cwl_len = g(&mut iw, 3)?;
        let sparse = g(&mut iw, 1)?;
        ow.put(1, sparse);
        if cwl_len == 0 || cwl_len > 5 {
            return err("codebook: nonsense codeword length");
        }
        for _ in 0..entries {
            let mut present = true;
            if sparse != 0 {
                let p = g(&mut iw, 1)?;
                ow.put(1, p);
                present = p != 0;
            }
            if present {
                let len = g(&mut iw, cwl_len)?;
                ow.put(5, len);
            }
        }
    }
    let lookup_type = g(&mut iw, 1)?;
    ow.put(4, lookup_type);
    if lookup_type == 1 {
        let min = g(&mut iw, 32)?;
        ow.put(32, min);
        let max = g(&mut iw, 32)?;
        ow.put(32, max);
        let value_length = g(&mut iw, 4)?;
        ow.put(4, value_length);
        let seq = g(&mut iw, 1)?;
        ow.put(1, seq);
        if dimensions == 0 {
            return err("codebook: zero dimensions");
        }
        let q = book_maptype1_quantvals(entries, dimensions);
        for _ in 0..q {
            let v = g(&mut iw, value_length + 1)?;
            ow.put(value_length + 1, v);
        }
    } else if lookup_type != 0 {
        return err("codebook: invalid lookup type");
    }
    // ww2ogg: "check that we used exactly all bytes" (+1: a fully-used last byte leaves a 0 byte)
    if iw.pos() / 8 + 1 != cb.len() {
        return err(format!("codebook size mismatch ({} vs {})", iw.pos() / 8 + 1, cb.len()));
    }
    Ok(())
}

/// Rebuilt standard Vorbis headers plus the info needed to rebuild audio packets.
pub struct VorbisSetup {
    pub ident: Vec<u8>,
    pub setup: Vec<u8>,
    pub mode_blockflag: Vec<bool>,
    pub mode_bits: u32,
    pub modified_packets: bool,
}

/// `ww2ogg_generate_vorbis_setup` for the external-codebook (`WWV_EXTERNAL_CODEBOOKS` /
/// `WWV_AOTUV603_CODEBOOKS`) layouts.
pub fn rebuild_setup(
    wsetup: &[u8],
    channels: u32,
    lib: CodebookLibrary,
) -> Result<(Vec<u8>, Vec<bool>, u32), VorbisError> {
    let mut iw = BitReader::new(wsetup);
    let mut ow = BitWriter::new();
    macro_rules! g {
        ($n:expr) => {
            iw.get($n).ok_or_else(|| VorbisError("setup truncated".into()))?
        };
    }
    macro_rules! copy {
        ($n:expr) => {{
            let v = g!($n);
            ow.put($n, v);
            v
        }};
    }
    ow.put(8, 0x05);
    for &c in b"vorbis" {
        ow.put(8, c as u32);
    }
    // Codebooks
    let codebook_count = copy!(8) + 1;
    for _ in 0..codebook_count {
        let id = g!(10);
        let cb = lookup_codebook(lib, id).ok_or_else(|| VorbisError(format!("codebook id {:#x} not in library", id)))?;
        rebuild_codebook(&mut ow, cb)?;
    }
    // Time domain transforms (placeholder)
    ow.put(6, 0);
    ow.put(16, 0);
    // Floors
    let floor_count = copy!(6) + 1;
    for _ in 0..floor_count {
        ow.put(16, 1); // floor type 1
        let partitions = copy!(5);
        let mut class_list = [0u32; 32];
        let mut max_class = 0;
        for j in 0..partitions as usize {
            let c = copy!(4);
            class_list[j] = c;
            max_class = max_class.max(c);
        }
        let mut class_dims = [0u32; 17];
        for j in 0..=max_class as usize {
            class_dims[j] = copy!(3) + 1;
            let subclasses = copy!(2);
            if subclasses != 0 {
                let masterbook = copy!(8);
                if masterbook >= codebook_count {
                    return err("invalid floor1 masterbook");
                }
            }
            for _ in 0..(1u32 << subclasses) {
                let book_plus1 = copy!(8) as i32;
                let book = book_plus1 - 1;
                if book >= 0 && book as u32 >= codebook_count {
                    return err("invalid floor1 subclass book");
                }
            }
        }
        let _mult = copy!(2);
        let rangebits = copy!(4);
        for j in 0..partitions as usize {
            for _ in 0..class_dims[class_list[j] as usize] {
                copy!(rangebits);
            }
        }
    }
    // Residues
    let residue_count = copy!(6) + 1;
    for _ in 0..residue_count {
        let rtype = g!(2);
        ow.put(16, rtype);
        if rtype > 2 {
            return err("invalid residue type");
        }
        copy!(24); // begin
        copy!(24); // end
        copy!(24); // partition size - 1
        let classifications = copy!(6) + 1;
        let classbook = copy!(8);
        if classbook >= codebook_count {
            return err("invalid residue classbook");
        }
        let mut cascade = [0u32; 65];
        for j in 0..classifications as usize {
            let low = copy!(3);
            let flag = copy!(1);
            let high = if flag != 0 { copy!(5) } else { 0 };
            cascade[j] = high * 8 + low;
        }
        for j in 0..classifications as usize {
            for k in 0..8 {
                if cascade[j] & (1 << k) != 0 {
                    let book = copy!(8);
                    if book >= codebook_count {
                        return err("invalid residue book");
                    }
                }
            }
        }
    }
    // Mappings
    let mapping_count = copy!(6) + 1;
    for _ in 0..mapping_count {
        ow.put(16, 0); // mapping type 0
        let submaps_flag = copy!(1);
        let submaps = if submaps_flag != 0 { copy!(4) + 1 } else { 1 };
        let square_polar = copy!(1);
        if square_polar != 0 {
            let steps = copy!(8) + 1;
            let bits = ilog(channels - 1);
            for _ in 0..steps {
                let mag = copy!(bits);
                let ang = copy!(bits);
                if ang == mag || mag >= channels || ang >= channels {
                    return err("invalid coupling");
                }
            }
        }
        let reserved = copy!(2);
        if reserved != 0 {
            return err("mapping reserved field nonzero");
        }
        if submaps > 1 {
            for _ in 0..channels {
                let mux = copy!(4);
                if mux >= submaps {
                    return err("mapping_mux >= submaps");
                }
            }
        }
        for _ in 0..submaps {
            copy!(8); // time config
            let floor = copy!(8);
            if floor >= floor_count {
                return err("invalid floor mapping");
            }
            let residue = copy!(8);
            if residue >= residue_count {
                return err("invalid residue mapping");
            }
        }
    }
    // Modes
    let mode_count = copy!(6) + 1;
    let mode_bits = ilog(mode_count - 1);
    let mut blockflags = Vec::with_capacity(mode_count as usize);
    for _ in 0..mode_count {
        let bf = copy!(1);
        blockflags.push(bf != 0);
        ow.put(16, 0); // window type
        ow.put(16, 0); // transform type
        let mapping = copy!(8);
        if mapping >= mapping_count {
            return err("invalid mode mapping");
        }
    }
    ow.put(1, 1); // framing
    Ok((ow.finish(), blockflags, mode_bits))
}

/// Standard Vorbis identification header (vgmstream `build_header_identification`).
pub fn build_ident(channels: u8, sample_rate: u32, bs0_exp: u8, bs1_exp: u8) -> Vec<u8> {
    let mut b = vec![0x01];
    b.extend_from_slice(b"vorbis");
    b.extend_from_slice(&0u32.to_le_bytes());
    b.push(channels);
    b.extend_from_slice(&sample_rate.to_le_bytes());
    b.extend_from_slice(&[0; 12]);
    b.push((bs1_exp << 4) | bs0_exp);
    b.push(1);
    b
}

/// Streaming Wwise Vorbis decoder over a complete (or prefetch-truncated) .wem buffer.
pub struct WwiseVorbisDecoder<'a> {
    buf: &'a [u8],
    pub channels: u16,
    pub sample_rate: u32,
    pub num_samples: u32,
    pub library: CodebookLibrary,
    ident: lewton::header::IdentHeader,
    setup: lewton::header::SetupHeader,
    pwr: lewton::audio::PreviousWindowRight,
    pub floor: FloorSynthesis,
    /// Rebuilt standard Vorbis identification / setup headers.
    pub ident_header: Vec<u8>,
    pub setup_header: Vec<u8>,
    blockflags: Vec<bool>,
    mode_bits: u32,
    modified: bool,
    prev_blockflag: bool,
    pos: usize,
    end: usize,
    emitted: u64,
}

impl<'a> WwiseVorbisDecoder<'a> {
    pub fn new(buf: &'a [u8], wem: &Wem) -> Result<Self, VorbisError> {
        if !codebooks::AVAILABLE {
            return err("Wwise codebooks unavailable: third_party/vgmstream missing at build time (run ./scripts/third_party.sh vgmstream)");
        }
        let v = wem.vorbis.as_ref().ok_or_else(|| VorbisError("not a Wwise Vorbis wem".into()))?;
        let data_start = wem.data.offset;
        let end = (data_start + wem.data.size).min(buf.len());
        let setup_off = data_start + v.seek_table_size as usize;
        if setup_off + 2 > end {
            return err("setup packet out of range");
        }
        let setup_size = u16::from_le_bytes([buf[setup_off], buf[setup_off + 1]]) as usize;
        let wsetup = buf
            .get(setup_off + 2..setup_off + 2 + setup_size)
            .ok_or_else(|| VorbisError("setup packet truncated".into()))?;
        let ident_bytes = build_ident(wem.channels as u8, wem.sample_rate, v.blocksize_0_exp, v.blocksize_1_exp);
        let ident = lewton::header::read_header_ident(&ident_bytes)
            .map_err(|e| VorbisError(format!("ident: {:?}", e)))?;
        // vgmstream: .wem defaults to aoTuV 6.03 codebooks, retrying with the standard library.
        let mut last_err = String::new();
        for lib in [CodebookLibrary::AoTuV603, CodebookLibrary::Standard] {
            let (setup_bytes, blockflags, mode_bits) = match rebuild_setup(wsetup, wem.channels as u32, lib) {
                Ok(x) => x,
                Err(e) => {
                    last_err = e.0;
                    continue;
                }
            };
            let setup = match lewton::header::read_header_setup(
                &setup_bytes,
                ident.audio_channels,
                (ident.blocksize_0, ident.blocksize_1),
            ) {
                Ok(s) => s,
                Err(e) => {
                    last_err = format!("setup: {:?}", e);
                    continue;
                }
            };
            return Ok(Self {
                buf,
                channels: wem.channels,
                sample_rate: wem.sample_rate,
                num_samples: v.num_samples,
                library: lib,
                ident,
                setup,
                pwr: lewton::audio::PreviousWindowRight::new(),
                floor: default_floor(),
                ident_header: ident_bytes.clone(),
                setup_header: setup_bytes,
                blockflags,
                mode_bits,
                // vgmstream setup_version_config: equal blocksizes keep standard packets.
                modified: v.blocksize_0_exp != v.blocksize_1_exp,
                prev_blockflag: false,
                pos: data_start + v.audio_offset as usize,
                end,
                emitted: 0,
            });
        }
        err(format!("could not rebuild Vorbis setup with either codebook library: {}", last_err))
    }

    /// Rebuilds the next Wwise packet into a standard Vorbis audio packet
    /// (`ww2ogg_generate_vorbis_packet`, WWV_MODIFIED branch).
    pub fn next_packet(&mut self) -> Option<Result<Vec<u8>, VorbisError>> {
        if self.pos + 2 > self.end {
            return None;
        }
        let size = u16::from_le_bytes([self.buf[self.pos], self.buf[self.pos + 1]]) as usize;
        let start = self.pos + 2;
        if size == 0 || start + size > self.end {
            return None; // EOF or truncated (prefetch) tail
        }
        let payload = &self.buf[start..start + size];
        self.pos = start + size;
        if !self.modified {
            return Some(Ok(payload.to_vec()));
        }
        // Next packet's first byte (for the long-window next flag), unless at EOF.
        // vgmstream reads `header + 1` bytes past the packet from the file (not clamped to the
        // data chunk) and uses that byte whenever the read succeeds.
        let next_first = self.buf.get(self.pos + 2).copied();
        let mut iw = BitReader::new(payload);
        let mut ow = BitWriter::new();
        ow.put(1, 0); // packet type: audio
        let mode = iw.get(self.mode_bits).unwrap_or(0);
        ow.put(self.mode_bits, mode);
        let remainder = iw.get(8 - self.mode_bits).unwrap_or(0);
        let Some(&bf) = self.blockflags.get(mode as usize) else {
            return Some(err(format!("invalid mode {}", mode)));
        };
        if bf {
            let next_bf = match next_first {
                Some(b) => {
                    let mut nw = BitReader::new(std::slice::from_ref(&b));
                    let nm = nw.get(self.mode_bits).unwrap_or(0);
                    self.blockflags.get(nm as usize).copied().unwrap_or(false)
                }
                None => false,
            };
            ow.put(1, self.prev_blockflag as u32);
            ow.put(1, next_bf as u32);
        }
        self.prev_blockflag = bf;
        ow.put(8 - self.mode_bits, remainder);
        for &b in &payload[1..] {
            ow.put(8, b as u32);
        }
        Some(Ok(ow.finish()))
    }

    /// Decodes the next packet; returns planar f32 samples (empty for the first packet).
    /// `None` at end of stream. Output is clipped to the header's `num_samples`.
    pub fn decode_packet(&mut self) -> Option<Result<Vec<Vec<f32>>, VorbisError>> {
        loop {
            let pkt = match self.next_packet()? {
                Ok(p) => p,
                Err(e) => return Some(Err(e)),
            };
            let res: Result<Vec<Vec<f32>>, _> =
                lewton::audio::read_audio_packet_generic_with(&self.ident, &self.setup, &pkt, &mut self.pwr, self.floor);
            let mut pcm = match res {
                Ok(p) => p,
                Err(lewton::audio::AudioReadError::AudioIsHeader) => continue,
                Err(e) => return Some(Err(VorbisError(format!("audio packet: {:?}", e)))),
            };
            let n = pcm.first().map(|c| c.len()).unwrap_or(0) as u64;
            let left = (self.num_samples as u64).saturating_sub(self.emitted);
            if n > left {
                for c in &mut pcm {
                    c.truncate(left as usize);
                }
            }
            self.emitted += n.min(left);
            return Some(Ok(pcm));
        }
    }

    /// Decodes the whole stream to interleaved f32.
    pub fn decode_all(mut self) -> Result<Vec<f32>, VorbisError> {
        let ch = self.channels as usize;
        let mut out = Vec::with_capacity(self.num_samples as usize * ch);
        while let Some(p) = self.decode_packet() {
            let p = p?;
            let n = p.first().map(|c| c.len()).unwrap_or(0);
            for i in 0..n {
                for c in 0..ch {
                    out.push(p[c][i]);
                }
            }
        }
        Ok(out)
    }

    pub fn samples_emitted(&self) -> u64 {
        self.emitted
    }
}
