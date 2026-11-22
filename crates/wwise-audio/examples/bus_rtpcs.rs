//! RTPCs and state props of selected buses in Init.bnk (or a given bank).
//! usage: cargo run --release -p wwise-audio --example bus_rtpcs -- <vpath|file.bnk> <bus id>...
use wwise_audio::bnk::{Bank, HircBody};
use wwise_audio::data::GameAudio;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let src = &args[0];
    let buf = if std::path::Path::new(src).is_file() { std::fs::read(src).unwrap() } else { GameAudio::open().unwrap().read(src).unwrap() };
    let ids: Vec<u32> = args[1..].iter().map(|s| s.parse().unwrap()).collect();
    let b = Bank::parse(&buf).unwrap();
    for o in &b.hirc {
        let (HircBody::Bus(x) | HircBody::AuxBus(x)) = &o.body else { continue };
        if !ids.contains(&o.id) {
            continue;
        }
        println!("bus {} cfg {:#x} props {:?}", o.id, x.channel_config, x.props);
        for r in &x.rtpcs {
            let pts: Vec<String> = r.points.iter().map(|p| format!("({},{},{})", p.from, p.to, p.interp)).collect();
            println!(
                "  rtpc {} type {} accum {} param {:#x} scaling {} default {:?} pts {}",
                r.rtpc_id,
                r.rtpc_type,
                r.accum,
                r.param_id,
                r.scaling,
                b.global_settings.as_ref().and_then(|g| g.rtpc_ramping.iter().find(|q| q.rtpc_id == r.rtpc_id)).map(|q| q.default_value),
                pts.join(" ")
            );
        }
        println!("  states {:?}", x.states);
    }
}
