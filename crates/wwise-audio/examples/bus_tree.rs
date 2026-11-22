//! Print the bus hierarchy of Init.bnk (+ optional extra banks) with props, FX and channel config.
//! usage: cargo run --release -p wwise-audio --example bus_tree -- [extra bank vpaths...]
use wwise_audio::bnk::hirc::HircBody;
use wwise_audio::bnk::Bank;
use wwise_audio::data::GameAudio;
use std::collections::BTreeMap;
fn main() {
    let ga = GameAudio::open().unwrap();
    let mut names = vec!["data_pc/audio/events/Init.bnk".to_string()];
    names.extend(std::env::args().skip(1));
    let mut buses = BTreeMap::new();
    let mut fxs: BTreeMap<u32, u32> = BTreeMap::new();
    for n in &names {
        let buf = ga.read(n).unwrap();
        let b = Bank::parse(&buf).unwrap();
        for o in &b.hirc {
            if let HircBody::FxShareSet(f) | HircBody::FxCustom(f) = &o.body { fxs.insert(o.id, f.plugin_id); }
        }
        for o in b.hirc {
            if let HircBody::Bus(x) | HircBody::AuxBus(x) = &o.body {
                buses.entry(o.id).or_insert((o.hirc_type, x.clone(), n.clone()));
            }
        }
    }
    let mut kids: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
    for (id, (_, b, _)) in &buses { kids.entry(b.override_bus_id).or_default().push(*id); }
    fn walk(id: u32, d: usize, buses: &BTreeMap<u32, (u8, wwise_audio::bnk::hirc::Bus, String)>, kids: &BTreeMap<u32, Vec<u32>>, fxs: &BTreeMap<u32, u32>) {
        let (t, b, _) = &buses[&id];
        let props: Vec<String> = b.props.0.iter().map(|(p, v)| format!("{}={}", wwise_audio::bnk::hirc::prop::name(*p), f32::from_bits(*v))).collect();
        println!("{}{} {} cfg={:#x} fx={:?} mixer={:#x} rtpcs={} states={} ducks={} props=[{}]", "  ".repeat(d), if *t == 8 { "Bus" } else { "Aux" }, id, b.channel_config,
            b.fx.slots.iter().map(|s| fxs.get(&s.fx_id).map(|p| wwise_audio::bnk::plugin_name(*p).to_string()).unwrap_or(format!("{}?", s.fx_id))).collect::<Vec<_>>(), b.mixer_fx_id, b.rtpcs.len(), b.states.groups.len(), b.ducks.len(), props.join(" "));
        if let Some(k) = kids.get(&id) { for c in k { walk(*c, d + 1, buses, kids, fxs); } }
    }
    for r in kids.get(&0).cloned().unwrap_or_default() { walk(r, 0, &buses, &kids, &fxs); }
}
