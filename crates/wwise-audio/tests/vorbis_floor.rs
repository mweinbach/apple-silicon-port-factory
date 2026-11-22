//! Vorbis floor1 synthesis: the lewton fork in `codec::lewton` must stay
//! bit-identical to upstream lewton in `FloorSynthesis::Libvorbis` mode, and
//! the AK (Wwise AVX2) floor must differ from it only by a small, floor-shaped
//! relative amount. Real game data; skips without the install.

use wwise_audio::bnk::Bank;
use wwise_audio::codec::vorbis::{FloorSynthesis, WwiseVorbisDecoder};
use wwise_audio::data::{game_dir, GameAudio};
use wwise_audio::wem::Wem;

fn game() -> Option<GameAudio> {
    if game_dir().is_none() {
        eprintln!("skipping: Alan Wake 2 install not found (set AW2_GAME_DIR)");
        return None;
    }
    Some(GameAudio::open().expect("mount game VFS"))
}

fn media(ga: &GameAudio) -> Vec<(String, Vec<u8>)> {
    let buf = ga.read("data_pc/audio/events/Global.bnk").expect("read bank");
    let b = Bank::parse(&buf).expect("parse bank");
    let m = b.media(&buf, 39952857).expect("DIDX entry 39952857").to_vec();
    let s = ga.read("data_pc/audio/media/999877740.wem").unwrap();
    vec![("39952857 (1 ch, embedded)".into(), m), ("999877740 (2 ch, streamed)".into(), s)]
}

fn decode(buf: &[u8], floor: FloorSynthesis) -> Vec<Vec<f32>> {
    let wem = Wem::parse(buf).unwrap();
    let mut d = WwiseVorbisDecoder::new(buf, &wem).unwrap();
    d.floor = floor;
    let mut out = vec![Vec::new(); wem.channels as usize];
    while let Some(p) = d.decode_packet() {
        for (o, c) in out.iter_mut().zip(p.unwrap()) {
            o.extend(c);
        }
    }
    out
}

/// Upstream lewton over the same rebuilt packets.
fn decode_upstream(buf: &[u8]) -> Vec<Vec<f32>> {
    let wem = Wem::parse(buf).unwrap();
    let mut d = WwiseVorbisDecoder::new(buf, &wem).unwrap();
    let ident = lewton::header::read_header_ident(&d.ident_header).unwrap();
    let setup = lewton::header::read_header_setup(&d.setup_header, ident.audio_channels, (ident.blocksize_0, ident.blocksize_1)).unwrap();
    let mut pwr = lewton::audio::PreviousWindowRight::new();
    let mut out = vec![Vec::new(); wem.channels as usize];
    let total = d.num_samples as usize;
    while let Some(p) = d.next_packet() {
        let pcm: Vec<Vec<f32>> = match lewton::audio::read_audio_packet_generic(&ident, &setup, &p.unwrap(), &mut pwr) {
            Ok(x) => x,
            Err(lewton::audio::AudioReadError::AudioIsHeader) => continue,
            Err(e) => panic!("{e:?}"),
        };
        for (o, c) in out.iter_mut().zip(pcm) {
            o.extend(c);
        }
    }
    for o in &mut out {
        o.truncate(total);
    }
    out
}

#[test]
fn fork_libvorbis_mode_is_bit_identical_to_upstream_lewton() {
    let Some(ga) = game() else { return };
    for (name, buf) in media(&ga) {
        let a = decode(&buf, FloorSynthesis::Libvorbis);
        let b = decode_upstream(&buf);
        assert_eq!(a.len(), b.len());
        for (ca, cb) in a.iter().zip(&b) {
            assert_eq!(ca.len(), cb.len(), "{name}");
            let diff = ca.iter().zip(cb).filter(|(x, y)| x.to_bits() != y.to_bits()).count();
            assert_eq!(diff, 0, "{name}: {diff} samples differ");
        }
    }
}

#[test]
fn ak_floor_differs_from_libvorbis_by_a_small_relative_amount() {
    let Some(ga) = game() else { return };
    for (name, buf) in media(&ga) {
        let a = decode(&buf, FloorSynthesis::Libvorbis);
        let k = decode(&buf, FloorSynthesis::AkAvx2);
        let (mut e_sig, mut e_diff) = (0f64, 0f64);
        for (ca, ck) in a.iter().zip(&k) {
            assert_eq!(ca.len(), ck.len());
            for (x, y) in ca.iter().zip(ck) {
                e_sig += (*x as f64).powi(2);
                e_diff += (*x as f64 - *y as f64).powi(2);
            }
        }
        let rel_db = 10.0 * (e_diff / e_sig.max(1e-30)).log10();
        eprintln!("{name}: AK floor vs libvorbis floor: {rel_db:.1} dB relative");
        // A few-percent per-bin gain difference: well below the signal, well above float noise.
        assert!(rel_db < -20.0 && rel_db > -80.0, "{name}: {rel_db:.1} dB");
    }
}
