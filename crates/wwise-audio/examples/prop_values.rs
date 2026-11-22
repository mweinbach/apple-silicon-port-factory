//! Histogram of raw property values (as u32 and f32) for selected AkPropIDs in one bank, split by
//! object kind (action vs node). Used to classify property value types (int vs float).
//! usage: cargo run --release -p wwise-audio --example prop_values -- <vpath|file.bnk> [propid...]
use wwise_audio::bnk::{hirc::prop, Bank, HircBody};
use wwise_audio::data::GameAudio;
use std::collections::BTreeMap;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let src = args.first().expect("bank");
    let buf = if std::path::Path::new(src).is_file() {
        std::fs::read(src).unwrap()
    } else {
        GameAudio::open().unwrap().read(src).unwrap()
    };
    let ids: Vec<u8> = if args.len() > 1 {
        args[1..].iter().map(|a| u8::from_str_radix(a.trim_start_matches("0x"), 16).unwrap()).collect()
    } else {
        vec![prop::DELAY_TIME, prop::TRANSITION_TIME, prop::PROBABILITY, prop::LOOP, prop::INITIAL_DELAY, prop::PITCH, prop::VOLUME]
    };
    let b = Bank::parse(&buf).unwrap();
    let mut hist: BTreeMap<(u8, &str, bool), BTreeMap<u32, usize>> = BTreeMap::new();
    for o in &b.hirc {
        let (kind, props, ranged) = match &o.body {
            HircBody::Action(a) => ("action", &a.props, &a.ranged),
            _ => match o.node_base() {
                Some(n) => ("node", &n.props, &n.ranged),
                None => continue,
            },
        };
        for (id, v) in &props.0 {
            if ids.contains(id) {
                *hist.entry((*id, kind, false)).or_default().entry(*v).or_default() += 1;
            }
        }
        for (id, lo, hi) in &ranged.0 {
            if ids.contains(id) {
                *hist.entry((*id, kind, true)).or_default().entry(*lo).or_default() += 1;
                *hist.entry((*id, kind, true)).or_default().entry(*hi).or_default() += 1;
            }
        }
    }
    for ((id, kind, ranged), h) in hist {
        let mut v: Vec<_> = h.into_iter().collect();
        v.sort_by(|a, b| b.1.cmp(&a.1));
        let shown: Vec<String> =
            v.iter().take(12).map(|(raw, n)| format!("{:#x}(u{} f{}) x{}", raw, raw, f32::from_bits(*raw), n)).collect();
        println!("{:#04x} {:<16} {:<6} {}: {}", id, prop::name(id), kind, if ranged { "ranged" } else { "value " }, shown.join(", "));
    }
}
