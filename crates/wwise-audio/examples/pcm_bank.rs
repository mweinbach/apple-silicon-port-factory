//! Codec-isolation variant of a bank for oracle null tests: every Sound whose source is a codec
//! (plugin type 1, e.g. Vorbis) is switched to an in-bank PCM16 .wem holding *our* decode of its
//! media (rounded to 16 bits). Oracle and native renders of the result then share identical
//! PCM input, so any remaining residual is from the engine, not the decoder.
//!
//! Changes, per Sound: source plugin id -> 0x00010001 (PCM), stream type -> 0 (bank data),
//! in-memory media size -> new .wem size, source bits: prefetch bit (1) cleared. Media: DIDX /
//! DATA gain one entry per converted source id (same id; replaces any existing entry), entries
//! 16-byte aligned. The PCM .wem uses the Wwise layout (fmt size 0x18, tag 0xFFFE, block align
//! 2*ch, AkChannelConfig at fmt+0x14 = count | 1<<8 | mask<<12, mask from the original wem's
//! channel config when present). Everything else is copied.
//!
//! usage: cargo run --release -p wwise-audio --example pcm_bank -- <in.bnk> <out.bnk>
use wwise_audio::bnk::{Bank, HircBody};
use wwise_audio::data::GameAudio;
use wwise_audio::reader::Reader;
use std::collections::BTreeMap;

fn pcm_wem(samples: &[f32], ch: u16, rate: u32, mask: u32) -> Vec<u8> {
    let cfg = ch as u32 | (1 << 8) | (mask << 12);
    let mut fmt = vec![];
    fmt.extend_from_slice(&0xFFFEu16.to_le_bytes());
    fmt.extend_from_slice(&ch.to_le_bytes());
    fmt.extend_from_slice(&rate.to_le_bytes());
    fmt.extend_from_slice(&(rate * 2 * ch as u32).to_le_bytes());
    fmt.extend_from_slice(&(2 * ch).to_le_bytes());
    fmt.extend_from_slice(&16u16.to_le_bytes());
    fmt.extend_from_slice(&6u16.to_le_bytes());
    fmt.extend_from_slice(&16u16.to_le_bytes());
    fmt.extend_from_slice(&cfg.to_le_bytes());
    let data: Vec<u8> = samples
        .iter()
        .flat_map(|&s| ((s * 32768.0).round().clamp(-32768.0, 32767.0) as i16).to_le_bytes())
        .collect();
    let mut body = b"WAVEfmt ".to_vec();
    body.extend_from_slice(&(fmt.len() as u32).to_le_bytes());
    body.extend_from_slice(&fmt);
    body.extend_from_slice(b"data");
    body.extend_from_slice(&(data.len() as u32).to_le_bytes());
    body.extend_from_slice(&data);
    let mut out = b"RIFF".to_vec();
    out.extend_from_slice(&(body.len() as u32).to_le_bytes());
    out.extend_from_slice(&body);
    out
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (inp, outp) = (&args[0], &args[1]);
    let buf = std::fs::read(inp).unwrap();
    let ga = GameAudio::open().ok();
    let bank = Bank::parse(&buf).unwrap();
    // media id -> bytes (existing entries first, converted ones replace them)
    let mut media: BTreeMap<u32, Vec<u8>> = BTreeMap::new();
    for e in &bank.media_index {
        media.insert(e.id, bank.media(&buf, e.id).unwrap().to_vec());
    }
    let mut converted: BTreeMap<u32, Vec<u8>> = BTreeMap::new();
    let mut sound_edits = vec![]; // (abs offset of source, source id, stream type, bits)
    for o in &bank.hirc {
        let HircBody::Sound(s) = &o.body else { continue };
        let src = &s.source;
        if src.plugin_type() != 1 || src.plugin_id == 0x0001_0001 {
            continue;
        }
        if !converted.contains_key(&src.source_id) {
            let wem = if src.stream_type == 0 && media.contains_key(&src.source_id) {
                media[&src.source_id].clone()
            } else {
                ga.as_ref().expect("game data").read(&GameAudio::media_path(src.source_id)).unwrap()
            };
            let w = wwise_audio::wem::Wem::parse(&wem).unwrap();
            let d = wwise_audio::codec::decode_wem(&wem).unwrap();
            let mask = match w.channels {
                1 => 0x4,
                2 => 0x3,
                n => panic!("source {}: {} channels", src.source_id, n),
            };
            let pcm = pcm_wem(&d.samples, d.channels, d.sample_rate, mask);
            eprintln!(
                "source {} ({}): {} ch {} Hz {} frames -> {} bytes PCM",
                src.source_id,
                d.detail,
                d.channels,
                d.sample_rate,
                d.samples.len() / d.channels as usize,
                pcm.len()
            );
            converted.insert(src.source_id, pcm);
        }
        sound_edits.push((o.offset + 4, src.source_id, src.source_bits));
    }
    for (id, w) in &converted {
        media.insert(*id, w.clone());
    }
    let mut didx = vec![];
    let mut data = vec![];
    for (id, w) in &media {
        while data.len() % 16 != 0 {
            data.push(0);
        }
        didx.extend_from_slice(&id.to_le_bytes());
        didx.extend_from_slice(&(data.len() as u32).to_le_bytes());
        didx.extend_from_slice(&(w.len() as u32).to_le_bytes());
        data.extend_from_slice(w);
    }
    // patch the HIRC copy in place (the codec source block is a fixed 14 bytes before and after)
    let mut b2 = buf.clone();
    for (pos, id, bits) in &sound_edits {
        b2[*pos..*pos + 4].copy_from_slice(&0x0001_0001u32.to_le_bytes());
        b2[*pos + 4] = 0;
        assert_eq!(u32::from_le_bytes(b2[*pos + 5..*pos + 9].try_into().unwrap()), *id);
        b2[*pos + 9..*pos + 13].copy_from_slice(&(converted[id].len() as u32).to_le_bytes());
        b2[*pos + 13] = bits & !0x02;
    }
    let mut out = vec![];
    let mut r = Reader::new(&b2);
    let mut wrote_media = false;
    let put = |out: &mut Vec<u8>, tag: &[u8], body: &[u8]| {
        out.extend_from_slice(tag);
        out.extend_from_slice(&(body.len() as u32).to_le_bytes());
        out.extend_from_slice(body);
    };
    while !r.is_empty() {
        let tag = r.fourcc().unwrap();
        let size = r.u32().unwrap() as usize;
        let body = r.bytes(size).unwrap();
        match &tag {
            b"DIDX" | b"DATA" => {
                if !wrote_media {
                    put(&mut out, b"DIDX", &didx);
                    put(&mut out, b"DATA", &data);
                    wrote_media = true;
                }
            }
            b"BKHD" => {
                put(&mut out, &tag, body);
            }
            _ => {
                if !wrote_media && !media.is_empty() {
                    put(&mut out, b"DIDX", &didx);
                    put(&mut out, b"DATA", &data);
                    wrote_media = true;
                }
                put(&mut out, &tag, body);
            }
        }
    }
    let chk = Bank::parse(&out).unwrap();
    for o in &chk.hirc {
        if let HircBody::Sound(s) = &o.body {
            if converted.contains_key(&s.source.source_id) {
                assert_eq!(chk.media(&out, s.source.source_id).unwrap(), &converted[&s.source.source_id][..]);
            }
        }
    }
    std::fs::write(outp, &out).unwrap();
    eprintln!("{} sounds -> {} PCM sources, {} -> {} bytes", sound_edits.len(), converted.len(), buf.len(), out.len());
}
