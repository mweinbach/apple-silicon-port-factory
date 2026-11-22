//! FX wiring of a bank: AudioDevice slots, buses with FX slots / mixer plugins, and the plugin of
//! each referenced FX object. Used to find processing left in the dry path of oracle renders.
//! usage: cargo run --release -p wwise-audio --example fx_wiring -- <vpath|file.bnk> [bus id...]
use wwise_audio::bnk::{plugin_name, Bank, HircBody};
use wwise_audio::data::GameAudio;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let src = &args[0];
    let buf = if std::path::Path::new(src).is_file() { std::fs::read(src).unwrap() } else { GameAudio::open().unwrap().read(src).unwrap() };
    let only: Vec<u32> = args[1..].iter().map(|s| s.parse().unwrap()).collect();
    let b = Bank::parse(&buf).unwrap();
    let plug = |id: u32| -> String {
        match b.object(id).map(|o| &o.body) {
            Some(HircBody::FxShareSet(f)) | Some(HircBody::FxCustom(f)) => format!("{} ({:#x})", plugin_name(f.plugin_id), f.plugin_id),
            _ => format!("? (fx {id} not in bank)"),
        }
    };
    for o in &b.hirc {
        match &o.body {
            HircBody::AudioDevice(d) => {
                println!("AudioDevice {} plugin {} ({:#x}) slots bypass {:#x}:", o.id, plugin_name(d.fx.plugin_id), d.fx.plugin_id, d.slots.bypass_bits);
                for s in &d.slots.slots {
                    println!("   slot {} fx {} rendered {} -> {}", s.index, s.fx_id, s.is_rendered, plug(s.fx_id));
                }
            }
            HircBody::Bus(x) | HircBody::AuxBus(x) => {
                if !only.is_empty() && !only.contains(&o.id) {
                    continue;
                }
                if x.fx.slots.is_empty() && x.mixer_fx_id == 0 && only.is_empty() {
                    continue;
                }
                println!(
                    "Bus {} parent {} cfg {:#x} hdr {:#x} bypass {:#x} mixer {}",
                    o.id,
                    x.override_bus_id,
                    x.channel_config,
                    x.hdr_bits,
                    x.fx.bypass_bits,
                    if x.mixer_fx_id != 0 { plug(x.mixer_fx_id) } else { "-".into() }
                );
                for s in &x.fx.slots {
                    println!("   slot {} fx {} rendered {} -> {}", s.index, s.fx_id, s.is_rendered, plug(s.fx_id));
                }
            }
            _ => {}
        }
    }
}
