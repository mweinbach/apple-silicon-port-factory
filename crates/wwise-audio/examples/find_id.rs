//! Find which banks contain a HIRC object or source with the given ShortID (or name, hashed).
//! usage: cargo run --release -p wwise-audio --example find_id -- <id|name>...
use wwise_audio::bnk::Bank;
use wwise_audio::data::GameAudio;
fn main() {
    let ids: Vec<u32> = std::env::args().skip(1).map(|a| a.parse().unwrap_or_else(|_| wwise_audio::fnv1_32_lower(&a))).collect();
    let ga = GameAudio::open().unwrap();
    for f in ga.banks() {
        let Ok(buf) = ga.read(&f.path) else { continue };
        let Ok(b) = Bank::parse(&buf) else { continue };
        for o in &b.hirc {
            if ids.contains(&o.id) { println!("{} object {:#x} type {:#04x}", f.path, o.id, o.hirc_type); }
            for s in o.sources() { if ids.contains(&s.source_id) { println!("{} source {:#x} plugin {:#010x} in object {} type {:#04x}", f.path, s.source_id, s.plugin_id, o.id, o.hirc_type); } }
        }
    }
}
