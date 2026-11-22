//! Histogram of RanSeq container fields (transition time and its modifiers, loop count and its
//! modifiers, modes) across banks. Used to pin units (transition time is milliseconds) and to see
//! which container features the data uses.
//! usage: cargo run --release -p wwise-audio --example ranseq_values -- <vpath|file.bnk>...
use wwise_audio::bnk::{Bank, HircBody};
use wwise_audio::data::GameAudio;
use std::collections::BTreeMap;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let ga = GameAudio::open().ok();
    let mut h: BTreeMap<String, BTreeMap<String, usize>> = BTreeMap::new();
    let mut add = |k: &str, v: String| *h.entry(k.to_string()).or_default().entry(v).or_default() += 1;
    for src in &args {
        let buf = if std::path::Path::new(src).is_file() {
            std::fs::read(src).unwrap()
        } else {
            ga.as_ref().expect("game data").read(src).unwrap()
        };
        let b = Bank::parse(&buf).unwrap();
        for o in &b.hirc {
            let HircBody::RanSeqCntr(c) = &o.body else { continue };
            add("transition_time", format!("{}", c.transition_time));
            add("transition_mod", format!("{}..{}", c.transition_time_mod_min, c.transition_time_mod_max));
            add("loop", format!("{} ({}..{})", c.loop_count, c.loop_mod_min as i16, c.loop_mod_max as i16));
            add("mode/random/transition/bits", format!("{}/{}/{}/{:#x}", c.mode, c.random_mode, c.transition_mode, c.bits));
            add("avoid_repeat", format!("{}", c.avoid_repeat_count));
            add("n_items", format!("{}", c.playlist.len()));
            let ws: Vec<i32> = c.playlist.iter().map(|p| p.1).collect();
            add("weights_all_50000", format!("{}", ws.iter().all(|&w| w == 50000)));
        }
    }
    for (k, m) in h {
        let mut v: Vec<_> = m.into_iter().collect();
        v.sort_by(|a, b| b.1.cmp(&a.1));
        let s: Vec<String> = v.iter().take(14).map(|(a, n)| format!("{a} x{n}")).collect();
        println!("{k}: {}", s.join(", "));
    }
}
