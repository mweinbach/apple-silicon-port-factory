//! `wwise-audio validate`: pick N .wem per distinct stream class (source location, channels, rate,
//! Vorbis blocksizes, looped, prefetch-truncated) from the real game data, decode each with our
//! decoder and with an external `vgmstream-cli`, and compare sample-by-sample.
//!
//! Everything extracted/decoded goes to `--out` (default `out/audio/validate`, gitignored);
//! decoded WAVs are deleted after comparison unless `--keep`.

use crate::cli::compare;
use wwise_audio::bnk::Bank;
use wwise_audio::codec::decode_wem;
use wwise_audio::data::GameAudio;
use wwise_audio::wav;
use wwise_audio::wem::Wem;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

#[derive(Clone)]
enum Loc {
    File(String),
    Bank(String, u32),
}

fn class_of(src: &str, w: &Wem) -> String {
    let bs = w.vorbis.as_ref().map(|v| format!(" bs{},{}", v.blocksize_0_exp, v.blocksize_1_exp)).unwrap_or_default();
    format!(
        "{:<8} {} {:>2}ch {:>5}Hz{}{}{}",
        src,
        w.codec.name(),
        w.channels,
        w.sample_rate,
        bs,
        if w.loop_points.is_some() { " loop" } else { "" },
        if w.truncated { " prefetch" } else { "" }
    )
}

fn arg<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).map(String::as_str)
}

pub fn validate(args: &[String]) -> Result<(), String> {
    let per_class: usize = arg(args, "--per-class").and_then(|s| s.parse().ok()).unwrap_or(1);
    let out = PathBuf::from(arg(args, "--out").unwrap_or("out/audio/validate"));
    let keep = args.iter().any(|a| a == "--keep");
    let vgm = arg(args, "--vgmstream")
        .map(String::from)
        .or_else(|| std::env::var("VGMSTREAM_CLI").ok())
        .unwrap_or_else(|| "out/vgmstream/build/cli/vgmstream-cli".into());
    let have_vgm = Path::new(&vgm).is_file();
    if !have_vgm {
        eprintln!("warning: vgmstream-cli not found at {vgm}; only decoding with wwise-audio");
    }
    std::fs::create_dir_all(&out).map_err(|e| e.to_string())?;

    let ga = GameAudio::open()?;
    let nthreads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4).min(12);
    let found: Mutex<Vec<(String, String, Loc)>> = Mutex::new(Vec::new());

    // loose .wem
    let wems = ga.wems();
    std::thread::scope(|s| {
        for chunk in wems.chunks(wems.len().div_ceil(nthreads).max(1)) {
            let (ga, found) = (&ga, &found);
            s.spawn(move || {
                let mut l = Vec::new();
                for f in chunk {
                    let Ok(buf) = ga.read(&f.path) else { continue };
                    let Ok(w) = Wem::parse(&buf) else { continue };
                    let src = if f.path.contains("/dialogue/") { "dialogue" } else if f.path.contains("/media/") { "media" } else { "loose" };
                    l.push((class_of(src, &w), f.path.clone(), Loc::File(f.path.clone())));
                }
                found.lock().unwrap().extend(l);
            });
        }
    });
    // bank-embedded media
    let banks = ga.banks();
    std::thread::scope(|s| {
        for chunk in banks.chunks(banks.len().div_ceil(nthreads).max(1)) {
            let (ga, found) = (&ga, &found);
            s.spawn(move || {
                let mut l = Vec::new();
                for f in chunk {
                    let Ok(buf) = ga.read(&f.path) else { continue };
                    let Ok(b) = Bank::parse(&buf) else { continue };
                    for m in &b.media_index {
                        let Some(mb) = b.media(&buf, m.id) else { continue };
                        let Ok(w) = Wem::parse(mb) else { continue };
                        l.push((class_of("bank", &w), format!("{}#{}", f.path, m.id), Loc::Bank(f.path.clone(), m.id)));
                    }
                }
                found.lock().unwrap().extend(l);
            });
        }
    });
    let mut all = found.into_inner().unwrap();
    all.sort_by(|a, b| (&a.0, &a.1).cmp(&(&b.0, &b.1)));
    let mut classes: BTreeMap<String, (usize, Vec<(String, Loc)>)> = BTreeMap::new();
    for (k, name, loc) in all {
        let e = classes.entry(k).or_default();
        e.0 += 1;
        // spread picks: take the first and then every 97th to avoid always testing neighbours
        if e.1.len() < per_class && (e.0 == 1 || e.0 % 97 == 0) {
            e.1.push((name, loc));
        }
    }
    println!("{} stream classes; vgmstream: {}", classes.len(), if have_vgm { vgm.as_str() } else { "absent" });
    println!(
        "{:<52} {:>6} {:>10} {:>10} {:>10} {:>10} {:>7}  {}",
        "class", "files", "frames", "vgm frames", "max|d|", "rms", "xRT", "sample"
    );
    let (mut pass, mut fail) = (0, 0);
    let mut idx = 0;
    for (k, (count, picks)) in &classes {
        for (name, loc) in picks {
            idx += 1;
            let buf = match loc {
                Loc::File(p) => ga.read(p)?,
                Loc::Bank(p, id) => {
                    let bb = ga.read(p)?;
                    let b = Bank::parse(&bb).map_err(|e| e.to_string())?;
                    b.media(&bb, *id).ok_or("media vanished")?.to_vec()
                }
            };
            let wem_path = out.join(format!("{idx:03}.wem"));
            if let (Loc::Bank(_, id), Ok(w)) = (loc, Wem::parse(&buf)) {
                if w.truncated {
                    // Prefetch copy: Wwise plays these bytes while the full stream is opened, then
                    // continues from the streamed file. Check it is an exact prefix of that file.
                    let full = ga.read(&GameAudio::media_path(*id));
                    let ok = matches!(&full, Ok(f) if f.len() > buf.len() && f[..buf.len()] == buf[..]);
                    if ok {
                        pass += 1
                    } else {
                        fail += 1
                    }
                    println!(
                        "{:<52} {:>6} prefetch {} bytes is {}a prefix of {}  {}",
                        k,
                        count,
                        buf.len(),
                        if ok { "" } else { "NOT " },
                        GameAudio::media_path(*id),
                        name
                    );
                    continue;
                }
            }
            std::fs::write(&wem_path, &buf).map_err(|e| e.to_string())?;
            let t = std::time::Instant::now();
            let ours = match decode_wem(&buf) {
                Ok(d) => d,
                Err(e) => {
                    fail += 1;
                    println!("{:<52} {:>6} DECODE ERROR {e}  {name}", k, count);
                    continue;
                }
            };
            let el = t.elapsed().as_secs_f64();
            let frames = ours.samples.len() / ours.channels.max(1) as usize;
            let xrt = frames as f64 / ours.sample_rate as f64 / el.max(1e-9);
            if !have_vgm {
                println!("{:<52} {:>6} {:>10} {:>10} {:>10} {:>10} {:>7.0}  {}", k, count, frames, "-", "-", "-", xrt, name);
                continue;
            }
            let vgm_wav = out.join(format!("{idx:03}.vgm.wav"));
            let st = std::process::Command::new(&vgm)
                .args(["-i", "-W", "4", "-o"]) // ignore looping, 32-bit float output
                .arg(&vgm_wav)
                .arg(&wem_path)
                .output()
                .map_err(|e| e.to_string())?;
            if !st.status.success() {
                println!("{:<52} {:>6} {:>10} vgmstream failed: {}  {name}", k, count, frames, String::from_utf8_lossy(&st.stderr).trim());
                fail += 1;
                continue;
            }
            let vb = std::fs::read(&vgm_wav).map_err(|e| e.to_string())?;
            let (vc, vr, vs) = wav::read_wav(&vb)?;
            let r = compare(&ours.samples, &vs);
            let vframes = vs.len() / vc.max(1) as usize;
            let ok = vc == ours.channels && vr == ours.sample_rate && vframes == frames && r.max_abs < 1e-4;
            if ok {
                pass += 1
            } else {
                fail += 1
            }
            println!(
                "{:<52} {:>6} {:>10} {:>10} {:>10.2e} {:>10.2e} {:>7.0}  {}{}",
                k,
                count,
                frames,
                vframes,
                r.max_abs,
                r.rms,
                xrt,
                name,
                if ok { "" } else { "  <-- MISMATCH" }
            );
            if !keep {
                let _ = std::fs::remove_file(&vgm_wav);
                let _ = std::fs::remove_file(&wem_path);
            } else {
                let f = std::fs::File::create(out.join(format!("{idx:03}.ours.wav"))).map_err(|e| e.to_string())?;
                wav::write_wav(std::io::BufWriter::new(f), ours.channels, ours.sample_rate, &ours.samples, true)
                    .map_err(|e| e.to_string())?;
            }
        }
    }
    println!("validate: {pass} match, {fail} mismatch/error (threshold: equal length/ch/rate and max|d| < 1e-4)");
    Ok(())
}
