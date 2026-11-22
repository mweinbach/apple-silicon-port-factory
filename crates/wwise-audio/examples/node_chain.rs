//! Print the actor-mixer chain (props, ranged props, RTPCs, states) and bus chain of a sound.
//! usage: cargo run --release -p wwise-audio --example node_chain -- <bank dir> <bank> <sound id>
use wwise_audio::bnk::hirc::{prop, HircBody};
use wwise_audio::data::GameAudio;
use wwise_audio::engine::project::Project;

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let (dir, bank, sid): (&str, &str, u32) = (&a[0], &a[1], a[2].parse().unwrap());
    let mut p = Project::with_source(GameAudio::open().ok().map(std::sync::Arc::new));
    p.load_bank(&format!("{dir}/Init.bnk")).unwrap();
    p.load_bank(&format!("{dir}/{bank}.bnk")).unwrap();
    let mut id = sid;
    let mut bus = 0;
    while id != 0 {
        let o = p.node(id).expect("node");
        let n = o.node_base().unwrap();
        let props: Vec<String> = n.props.0.iter().map(|(k, v)| format!("{}={}", prop::name(*k), f32::from_bits(*v))).collect();
        let ranged: Vec<String> =
            n.ranged.0.iter().map(|(k, lo, hi)| format!("{}=[{},{}]", prop::name(*k), f32::from_bits(*lo), f32::from_bits(*hi))).collect();
        println!("node {} type {:#x} bus {} props {:?} ranged {:?}", id, o.hirc_type, n.override_bus_id, props, ranged);
        for r in &n.rtpcs {
            println!("   rtpc {} param {:#x} scaling {} accum {} pts {:?}", r.rtpc_id, r.param_id, r.scaling, r.accum, r.points.iter().map(|q| (q.from, q.to, q.interp)).collect::<Vec<_>>());
        }
        if !n.states.groups.is_empty() {
            println!("   states props {:?} groups {}", n.states.props.iter().map(|s| s.property_id).collect::<Vec<_>>(), n.states.groups.len());
        }
        if bus == 0 {
            bus = n.override_bus_id;
        }
        id = n.direct_parent_id;
    }
    while bus != 0 {
        let o = p.bus(bus).unwrap();
        let (HircBody::Bus(b) | HircBody::AuxBus(b)) = &o.body else { break };
        let props: Vec<String> = b.props.0.iter().map(|(k, v)| format!("{}={}", prop::name(*k), f32::from_bits(*v))).collect();
        println!("bus {} cfg {:#x} props {:?} rtpcs {}", bus, b.channel_config, props, b.rtpcs.len());
        for r in &b.rtpcs {
            println!("   rtpc {} param {:#x} scaling {} default {} pts {:?}", r.rtpc_id, r.param_id, r.scaling, p.param_default(r.rtpc_id), r.points.iter().map(|q| (q.from, q.to, q.interp)).collect::<Vec<_>>());
        }
        bus = b.override_bus_id;
    }
}
