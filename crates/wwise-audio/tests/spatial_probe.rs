//! Scratch probe (temporary).
use wwise_audio::engine::spatial::{positioning_matrix, Cfg, PanParams, Ray3d, SpeakerSetup};

fn show(ic: u32, oc: u32, pos: [f32; 3], spread: f32) {
    let s = SpeakerSetup::aw2();
    let ray = Ray3d { front: [0.0, 0.0, 1.0], top: [0.0, 1.0, 0.0], pos, spread, focus: 0.0, mask: !0 };
    let m = positioning_matrix(&s, &ray, &PanParams::default(), Cfg::std(ic), Cfg::std(oc), None);
    eprintln!("{ic:x}->{oc:x} pos={pos:?} spread={spread}");
    for r in m {
        eprintln!("    {:?}", r);
    }
}

#[test]
fn probe() {
    show(3, 3, [0.0, 0.0, 1.0], 0.0);
    show(3, 3, [1.0, 0.0, 0.0], 0.0);
    show(3, 3, [0.0, 0.0, 1.0], 0.3);
    show(4, 3, [0.0, 0.0, 1.0], 0.3);
    show(4, 3, [0.0, 0.0, 1.0], 0.7);
    show(3, 3, [0.0, 0.0, 1.0], 1.0);
    show(0x603, 3, [0.0, 0.0, 1.0], 0.0);
}

#[test]
#[ignore]
fn dump_curves() {
    use wwise_audio::bnk::{hirc::HircBody, Bank};
    let dir = std::env::var("AW2_BANKS").unwrap_or_else(|_| {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../out/audio_harness")
            .to_string_lossy()
            .into_owned()
    });
    let init = Bank::parse(&std::fs::read(format!("{dir}/Init.bnk")).unwrap()).unwrap();
    for (i, c) in init.env_curves.iter().enumerate() {
        eprintln!("ENVS[{i}] en={} scaling={} {:?}", c.enabled, c.scaling, c.points);
    }
    let mut hist = std::collections::BTreeMap::new();
    for o in &init.hirc {
        if let HircBody::Attenuation(a) = &o.body {
            for (slot, &ci) in a.curve_to_use.iter().enumerate() {
                if ci >= 0 {
                    let c = &a.curves[ci as usize];
                    *hist.entry((slot, c.scaling)).or_insert(0) += 1;
                } else if ci != -1 {
                    *hist.entry((slot, 200 + (ci as i32 + 2).unsigned_abs() as u8)).or_insert(0) += 1;
                }
            }
        }
    }
    eprintln!("attenuation (slot, scaling) histogram in Init.bnk: {hist:?}");
}
