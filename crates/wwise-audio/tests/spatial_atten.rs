//! A4: attenuation ray gains / filters against AK's `0x1415d3d00` / `0x1415d3350` (Init.bnk
//! attenuations loaded by AK's own loader, run under unicorn by `out/a4/gen_atten.py`).

use wwise_audio::bnk::hirc::{Attenuation, Cone, ConversionCurve, GraphPoint};
use wwise_audio::bnk::EnvCurve;
use wwise_audio::engine::spatial::atten::{ray_gains, AttenModel, ConeOverride, ProjectCurves, Ray, WorldTransform};
use wwise_audio::engine::spatial::atten::ray_geometry;
use std::collections::HashMap;

fn f(s: &str) -> f32 {
    s.parse().unwrap()
}

fn points(t: &[&str], n: usize) -> Vec<GraphPoint> {
    (0..n).map(|k| GraphPoint { from: f(t[3 * k]), to: f(t[3 * k + 1]), interp: t[3 * k + 2].parse().unwrap() }).collect()
}

#[test]
fn atten_matches_ak_vectors() {
    let text = include_str!("spatial_atten_vectors.txt");
    let mut envs = vec![];
    let mut atts: HashMap<u32, AttenModel> = HashMap::new();
    let mut n = 0;
    let mut worst = (0.0f32, String::new());
    let mut proj = ProjectCurves::default();
    for line in text.lines().filter(|l| !l.starts_with('#')) {
        let t: Vec<&str> = line.split_whitespace().collect();
        match t[0] {
            "envs" => {
                let np: usize = t[4].parse().unwrap();
                envs.push(EnvCurve { enabled: t[2] == "1", scaling: t[3].parse().unwrap(), points: points(&t[5..], np) });
                if envs.len() == 6 {
                    proj = ProjectCurves::from_env(&envs);
                }
            }
            "att" => {
                let id: u32 = t[1].parse().unwrap();
                let hs = t[2] == "1";
                let mut i = 4;
                let cone = if t[3] == "1" {
                    let c = Cone { inside_degrees: f(t[4]), outside_degrees: f(t[5]), outside_volume: f(t[6]), lo_pass: f(t[7]), hi_pass: f(t[8]) };
                    i = 9;
                    Some(c)
                } else {
                    None
                };
                let mut ctu = [0i8; 19];
                for k in 0..19 {
                    ctu[k] = t[i + k].parse().unwrap();
                }
                i += 19;
                let nc: usize = t[i].parse().unwrap();
                i += 1;
                let mut curves = vec![];
                for _ in 0..nc {
                    let sc: u8 = t[i].parse().unwrap();
                    let np: usize = t[i + 1].parse().unwrap();
                    curves.push(ConversionCurve { scaling: sc, points: points(&t[i + 2..], np) });
                    i += 2 + 3 * np;
                }
                let a = Attenuation { height_spread_enabled: hs, cone, curve_to_use: ctu, curves, rtpcs: vec![] };
                atts.insert(id, AttenModel::new(&a));
            }
            "ray" => {
                let att = &atts[&t[1].parse::<u32>().unwrap()];
                let mut ray: Ray = ray_geometry(&WorldTransform::default(), 1.0, &WorldTransform::default(), 1.0);
                ray.distance = f(t[2]);
                ray.emitter_angle = f(t[3]);
                ray.occlusion = f(t[4]);
                ray.obstruction = f(t[5]);
                ray.diffraction = f(t[6]);
                ray.transmission = f(t[7]);
                ray_gains(&mut ray, Some(att), &proj, ConeOverride::default(), true);
                let want: Vec<f32> = t[9..].iter().map(|s| f(s)).collect();
                let conn_lpf = ray.lpf.min(100.0);
                let conn_hpf = ray.hpf.min(100.0);
                let got = [ray.dry, ray.game_aux, ray.user_aux, ray.cone_t, ray.lpf, ray.hpf, conn_lpf, conn_hpf];
                for (k, (w, g)) in want.iter().zip(got.iter()).enumerate() {
                    // gains absolute, filter values (0..100) relative to their range
                    let d = if k >= 4 { (w - g).abs() / 100.0 } else { (w - g).abs() };
                    if d > worst.0 {
                        worst = (d, format!("{line}\n  got {got:?}"));
                    }
                }
                n += 1;
            }
            _ => {}
        }
    }
    eprintln!("{n} rays, worst error {:e}", worst.0);
    assert!(n > 100);
    assert!(worst.0 < 1e-5, "worst {:e}: {}", worst.0, worst.1);
}
