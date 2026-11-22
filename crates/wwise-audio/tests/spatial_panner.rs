//! A4: the 3D speaker panner against AK's own `0x1415dc9f0` (run under unicorn emulation by
//! `out/a4/gen_vectors.py`; vectors in `spatial_panner_vectors.txt`).

use wwise_audio::engine::spatial::{positioning_matrix, Cfg, PanParams, Ray3d, SpeakerSetup};

fn parse_f(s: &str) -> f32 {
    s.parse::<f32>().unwrap()
}

#[test]
fn speaker_setup_matches_ak() {
    let s = SpeakerSetup::aw2();
    // read back from the device after SetSpeakerAngles({60,101,145}, 54.7) under emulation
    assert_eq!(s.density, 1.397_457_957_267_761_2);
    assert_eq!(s.angles, vec![1.047_197_699_546_814, 1.762_782_692_909_240_7, 2.530_727_624_893_188_5]);
    assert_eq!(s.height, 0.954_695_224_761_962_9);
}

#[test]
fn panner_matches_ak_vectors() {
    let setup = SpeakerSetup::aw2();
    let text = include_str!("spatial_panner_vectors.txt");
    let mut n = 0;
    let mut worst = (0.0f32, String::new());
    let mut cats: std::collections::BTreeMap<String, (usize, usize)> = Default::default();
    for line in text.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()) {
        let (head, body) = line.split_once('|').unwrap();
        let h: Vec<&str> = head.split_whitespace().collect();
        let ic = Cfg::std(u32::from_str_radix(h[0], 16).unwrap());
        let oc = Cfg::std(u32::from_str_radix(h[1], 16).unwrap());
        let v: Vec<f32> = h[2..11].iter().map(|s| parse_f(s)).collect();
        let ray = Ray3d {
            front: [v[0], v[1], v[2]],
            top: [v[3], v[4], v[5]],
            pos: [v[6], v[7], v[8]],
            spread: parse_f(h[11]),
            focus: parse_f(h[12]),
            mask: u32::from_str_radix(h[13], 16).unwrap(),
        };
        let pp = PanParams {
            center: parse_f(h[14]),
            spat_mode: h[15].parse().unwrap(),
            height_spread: h[16] == "1",
            ..PanParams::default()
        };
        let want: Vec<f32> = body.split_whitespace().map(parse_f).collect();
        let got: Vec<f32> = positioning_matrix(&setup, &ray, &pp, ic, oc, None).into_iter().flatten().collect();
        assert_eq!(want.len(), got.len(), "{line}");
        let mut bad = 0.0f32;
        for (w, g) in want.iter().zip(&got) {
            let d = if w.is_nan() { 0.0 } else { (w - g).abs() };
            bad = bad.max(d);
            if d > worst.0 {
                worst = (d, format!("{line}\n  got {got:?}"));
            }
        }
        let key = format!(
            "in={} out={} spread={} focus={} center={} hs={}",
            h[0],
            h[1],
            if ray.spread == 0.0 { "0" } else if ray.spread <= 0.5 { "lo" } else { "hi" },
            ray.focus != 0.0,
            pp.center != 1.0,
            pp.height_spread
        );
        let e = cats.entry(key).or_insert((0usize, 0usize));
        e.0 += 1;
        if bad > 1e-4 {
            e.1 += 1;
        }
        n += 1;
    }
    let mut fails: Vec<_> = cats.iter().filter(|(_, v)| v.1 > 0).collect();
    fails.sort();
    for (k, v) in &fails {
        eprintln!("FAIL {k}: {}/{}", v.1, v.0);
    }
    eprintln!("{n} vectors, worst abs error {:e}", worst.0);
    // -60 dBFS budget on positional traces; the panner itself should be far below.
    assert!(worst.0 < 1e-4, "worst {:e}: {}", worst.0, worst.1);
}
