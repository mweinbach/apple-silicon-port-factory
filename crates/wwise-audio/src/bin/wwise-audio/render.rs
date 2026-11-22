//! `wwise-audio render | plan | list-2d | batch | nulltest | patch-2d` — the A3 offline renderer front-end.

use wwise_audio::data::GameAudio;
use wwise_audio::engine::{GameState, Project, RenderOptions, Renderer};
use wwise_audio::wav;
use std::collections::BTreeSet;

pub const USAGE: &str = "  render --bank <bnk>... --event <id|name> --out <x.wav> [opts]   offline render of one event
  plan --bank <bnk>... --event <id|name> [opts]                   resolved voices/params/notes, no audio
  list-2d [--bank <bnk>... | --per-bank [--filter S] [--max-banks N]] [--all]
                                                                  events whose whole path the renderer models
  batch  [same selection] [--limit N] [--out-dir DIR] [--events FILE]
                                                                  render clean events + manifest.tsv + oracle_events.tsv
  nulltest <ref.wav> <ours.wav> [--align]                         residual in dBFS (peak / RMS)
  patch-2d --out-dir DIR [--bank B]... [--events FILE] [--per-bank [--filter S] [--max-banks N]]
                                                                  write 2D-only derived banks (+ Init.bnk), see bnk::patch
    bank opts: --patch-2d (derive in memory)  --bank-dir DIR (prefer DIR/<name>.bnk)
    render opts: --seed N (default 0x5eed)  --no-resampler-fma  --mixer-fma  --bus-fx  --start-ramp  --end-ramp  --max-seconds S  --i16
                 --switch G=V  --state G=V  --param ID=X  --no-init  --tail-frames N
                 --pre-event ID (repeatable: posted and rendered first with the same seed, RanSeq state kept)
    banks: VFS path, short name (Global, play_x.bnk) or file; Init.bnk is loaded first unless --no-init";

/// Notes that keep an event in the exit-test set (deterministic, 2D, no node FX).
const CLEAN_NOTES: &[&str] = &[
    "pitch",
    "initial_delay",
    "seek",
    "smpl_loop",
    "center_pct",
    // AK RNG + draw order ported (engine::rng, engine::ranseq)
    "random_container",
    "random_Volume",
    "random_MakeUpGain",
    "random_Pitch",
    "random_InitialDelay",
    "random_transition_time",
    "random_loop_count",
    "random_seek",
    "akd_gain",
];

struct Args {
    banks: Vec<String>,
    event: Option<String>,
    out: Option<String>,
    opts: RenderOptions,
    gs: GameState,
    no_init: bool,
    i16: bool,
    limit: usize,
    out_dir: String,
    events_file: Option<String>,
    all: bool,
    align: bool,
    per_bank: bool,
    filter: Option<String>,
    max_banks: usize,
    bank_dir: Option<String>,
    patch2d: bool,
    /// Events posted (and discarded) before the rendered one, RanSeq state kept.
    pre_events: Vec<u32>,
    positional: Vec<String>,
}

fn id_of(s: &str) -> u32 {
    s.parse().unwrap_or_else(|_| wwise_audio::fnv1_32_lower(s))
}

fn parse(args: &[String]) -> Result<Args, String> {
    let mut a = Args {
        banks: vec![],
        event: None,
        out: None,
        opts: RenderOptions::default(),
        gs: GameState::default(),
        no_init: false,
        i16: false,
        limit: 200,
        out_dir: "out/audio/render".into(),
        events_file: None,
        all: false,
        align: false,
        per_bank: false,
        filter: None,
        max_banks: usize::MAX,
        bank_dir: None,
        patch2d: false,
        pre_events: vec![],
        positional: vec![],
    };
    let mut i = 0;
    let val = |i: &mut usize| -> Result<String, String> {
        *i += 1;
        args.get(*i).cloned().ok_or_else(|| format!("{} needs a value", args[*i - 1]))
    };
    let kv = |s: String| -> Result<(u32, String), String> {
        let (k, v) = s.split_once('=').ok_or("expected K=V")?;
        Ok((id_of(k), v.to_string()))
    };
    while i < args.len() {
        match args[i].as_str() {
            "--bank" => a.banks.push(val(&mut i)?),
            "--event" => a.event = Some(val(&mut i)?),
            "--out" => a.out = Some(val(&mut i)?),
            "--seed" => a.opts.seed = parse_u32(&val(&mut i)?).ok_or("bad --seed")?,
            "--resampler-fma" => a.opts.resampler_fma = true,
            "--no-resampler-fma" => a.opts.resampler_fma = false,
            "--mixer-fma" => a.opts.mixer_fma = true,
            "--bus-fx" => a.opts.bus_fx = true,
            "--no-bus-fx" => a.opts.bus_fx = false,
            "--start-ramp" => a.opts.start_ramp = true,
            "--end-ramp" => a.opts.end_ramp = true,
            "--max-seconds" => a.opts.max_seconds = val(&mut i)?.parse().map_err(|_| "bad --max-seconds")?,
            "--tail-frames" => a.opts.tail_frames = val(&mut i)?.parse().map_err(|_| "bad --tail-frames")?,
            "--i16" => a.i16 = true,
            "--no-init" => a.no_init = true,
            "--switch" => {
                let (k, v) = kv(val(&mut i)?)?;
                a.gs.switches.insert(k, id_of(&v));
            }
            "--state" => {
                let (k, v) = kv(val(&mut i)?)?;
                a.gs.states.insert(k, id_of(&v));
            }
            "--param" => {
                let (k, v) = kv(val(&mut i)?)?;
                a.gs.params.insert(k, v.parse().map_err(|_| "bad --param value")?);
            }
            "--limit" => a.limit = val(&mut i)?.parse().map_err(|_| "bad --limit")?,
            "--out-dir" => a.out_dir = val(&mut i)?,
            "--events" => a.events_file = Some(val(&mut i)?),
            "--all" => a.all = true,
            "--align" => a.align = true,
            "--per-bank" => a.per_bank = true,
            "--filter" => a.filter = Some(val(&mut i)?),
            "--bank-dir" => a.bank_dir = Some(val(&mut i)?),
            "--patch-2d" => a.patch2d = true,
            "--pre-event" => a.pre_events.push(id_of(&val(&mut i)?)),
            "--max-banks" => a.max_banks = val(&mut i)?.parse().map_err(|_| "bad --max-banks")?,
            s if s.starts_with("--") => return Err(format!("unknown option {}\n{}", s, USAGE)),
            s => a.positional.push(s.to_string()),
        }
        i += 1;
    }
    Ok(a)
}

fn project(a: &Args) -> Result<Project, String> {
    project_with(a, GameAudio::open().ok().map(std::sync::Arc::new), &a.banks)
}

/// Bank spec, redirected to `--bank-dir DIR/<name>.bnk` when that file exists (derived banks).
fn bank_spec(a: &Args, spec: &str) -> String {
    if let Some(dir) = &a.bank_dir {
        let mut name = spec.rsplit('/').next().unwrap_or(spec).to_string();
        if !name.ends_with(".bnk") {
            name.push_str(".bnk");
        }
        let f = format!("{}/{}", dir, name);
        if std::path::Path::new(&f).is_file() {
            return f;
        }
    }
    spec.to_string()
}

fn project_with(a: &Args, ga: Option<std::sync::Arc<GameAudio>>, banks: &[String]) -> Result<Project, String> {
    let mut p = Project::with_source(ga);
    let load = |p: &mut Project, spec: &str| -> Result<usize, String> {
        let spec = bank_spec(a, spec);
        if a.patch2d {
            // derived 2D bank built in memory (same bytes `patch-2d` writes)
            let (name, bytes) = p.read_bank(&spec)?;
            let (out, _) = wwise_audio::bnk::patch::patch_2d(&bytes).map_err(|e| format!("{}: {}", name, e))?;
            p.load_bank_bytes(name, out)
        } else {
            p.load_bank(&spec)
        }
    };
    if !a.no_init {
        load(&mut p, "Init.bnk")?;
    }
    let banks = if banks.is_empty() { vec!["Global.bnk".to_string()] } else { banks.to_vec() };
    for b in &banks {
        load(&mut p, b)?;
    }
    Ok(p)
}

/// `patch-2d`: write 2D-only derived copies of Init.bnk and the selected banks to --out-dir.
fn patch_2d_cmd(a: &Args) -> Result<(), String> {
    let ga = GameAudio::open()?;
    let mut names: Vec<String> = vec!["Init.bnk".into()];
    if let Some(f) = &a.events_file {
        for l in std::fs::read_to_string(f).map_err(|e| e.to_string())?.lines() {
            if l.starts_with('#') || l.trim().is_empty() {
                continue;
            }
            let b = l.split('\t').next().unwrap().trim().to_string();
            names.push(b);
        }
    }
    names.extend(a.banks.iter().cloned());
    if a.per_bank {
        let mut n = 0;
        for f in ga.banks() {
            let name = f.path.rsplit('/').next().unwrap().to_string();
            if name == "Init.bnk" || f.size < 16 || a.filter.as_ref().is_some_and(|flt| !name.contains(flt.as_str())) {
                continue;
            }
            if n >= a.max_banks {
                break;
            }
            names.push(name);
            n += 1;
        }
    }
    let mut seen = BTreeSet::new();
    std::fs::create_dir_all(&a.out_dir).map_err(|e| e.to_string())?;
    let mut tot = wwise_audio::bnk::patch::Patch2dStats::default();
    let mut nb = 0;
    let mut streams = 0;
    for mut n in names {
        if !n.ends_with(".bnk") {
            n.push_str(".bnk");
        }
        let base = n.rsplit('/').next().unwrap().to_string();
        if !seen.insert(base.clone()) {
            continue;
        }
        let vpath = if n.contains('/') { n.clone() } else { format!("{}{}", wwise_audio::data::EVENTS_DIR, n) };
        let buf = ga.read(&vpath)?;
        let (out, st) = wwise_audio::bnk::patch::patch_2d(&buf).map_err(|e| format!("{}: {}", base, e))?;
        std::fs::write(format!("{}/{}", a.out_dir, base), &out).map_err(|e| e.to_string())?;
        // streamed / prefetch sources: the A2 harness IO hook opens them as `<dir>/<id>.wem`
        // (OpenByID, crates/win32-rt/src/audio/harness.rs), so the oracle needs a copy
        if let Ok(bank) = wwise_audio::bnk::Bank::parse(&out) {
            for o in &bank.hirc {
                let wwise_audio::bnk::HircBody::Sound(s) = &o.body else { continue };
                let id = s.source.source_id;
                let dst = format!("{}/{}.wem", a.out_dir, id);
                if s.source.stream_type != 0 && !std::path::Path::new(&dst).exists() {
                    if let Ok(w) = ga.read(&GameAudio::media_path(id)) {
                        std::fs::write(&dst, &w).map_err(|e| e.to_string())?;
                        streams += 1;
                    }
                }
            }
        }
        nb += 1;
        tot.objects += st.objects;
        tot.had_3d += st.had_3d;
        tot.had_panner += st.had_panner;
        tot.had_aux += st.had_aux;
        tot.had_reflections += st.had_reflections;
        tot.unparsed += st.unparsed;
        tot.device_fx_bypassed += st.device_fx_bypassed;
        tot.bytes_in += st.bytes_in;
        tot.bytes_out += st.bytes_out;
    }
    eprintln!(
        "patch-2d: {} banks -> {}: {} objects rewritten ({} had 3D, {} a panner type, {} user aux sends, {} reflections bus), {} device FX chains bypassed, {} unparsed objects copied, {} -> {} bytes, {} streamed .wem copied",
        nb, a.out_dir, tot.objects, tot.had_3d, tot.had_panner, tot.had_aux, tot.had_reflections, tot.device_fx_bypassed, tot.unparsed, tot.bytes_in, tot.bytes_out, streams
    );
    Ok(())
}

fn write_out(path: &str, samples: &[f32], channels: u16, i16_trunc: bool) -> Result<(), String> {
    if let Some(dir) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let f = std::fs::File::create(path).map_err(|e| format!("{}: {}", path, e))?;
    if i16_trunc {
        // Wwise's capture conversion `(i16)(x * 32767.0f)` (truncating), as observed by the A1 harness
        let q: Vec<f32> = samples.iter().map(|&x| ((x * 32767.0) as i16) as f32 / 32767.0).collect();
        wav::write_wav(std::io::BufWriter::new(f), channels, wwise_audio::engine::math::RATE, &q, false)
    } else {
        wav::write_wav(std::io::BufWriter::new(f), channels, wwise_audio::engine::math::RATE, samples, true)
    }
    .map_err(|e| e.to_string())
}

pub struct Rendered {
    pub samples: Vec<f32>,
    pub notes: BTreeSet<String>,
    pub voices: usize,
    pub planned_end: Option<u64>,
    pub log: Vec<String>,
}

pub fn render_event(p: &Project, gs: &GameState, opts: &RenderOptions, event: u32) -> Result<Rendered, String> {
    render_event_after(p, gs, opts, &[], event)
}

/// [`render_event`] after posting and rendering `pre` (discarded) on the same nodes.
pub fn render_event_after(p: &Project, gs: &GameState, opts: &RenderOptions, pre: &[u32], event: u32) -> Result<Rendered, String> {
    let mut r = Renderer::new(p, gs.clone(), opts.clone());
    for &e in pre {
        r.post_event(e, 0)?;
        r.render()?;
        r.next_event();
    }
    r.post_event(event, 0)?;
    let samples = r.render()?;
    if r.voice_count() == 0 {
        r.notes.insert("no_voices".into());
    }
    Ok(Rendered { samples, notes: r.notes.clone(), voices: r.voice_count(), planned_end: r.planned_end(), log: r.log.clone() })
}

fn is_clean(notes: &BTreeSet<String>) -> bool {
    notes.iter().all(|n| CLEAN_NOTES.contains(&n.as_str()))
}

pub fn run(cmd: &str, rest: &[String]) -> Result<(), String> {
    let a = parse(rest)?;
    match cmd {
        "render" | "plan" => {
            let p = project(&a)?;
            let ev = id_of(a.event.as_deref().ok_or("--event required")?);
            let t = std::time::Instant::now();
            let r = render_event_after(&p, &a.gs, &a.opts, &a.pre_events, ev)?;
            for l in &r.log {
                println!("  {}", l);
            }
            let frames = r.samples.len() / a.opts.out_channels;
            println!(
                "event {}: {} voices, {} frames ({:.3} s), planned end {:?}, peak {:.4}, {:.0} ms",
                ev,
                r.voices,
                frames,
                frames as f64 / 48000.0,
                r.planned_end,
                r.samples.iter().fold(0f32, |m, &x| m.max(x.abs())),
                t.elapsed().as_secs_f64() * 1e3
            );
            println!("notes: {}", if r.notes.is_empty() { "(none)".into() } else { r.notes.iter().cloned().collect::<Vec<_>>().join(", ") });
            if cmd == "render" {
                let out = a.out.as_deref().ok_or("--out required")?;
                write_out(out, &r.samples, a.opts.out_channels as u16, a.i16)?;
                println!("wrote {}", out);
            }
        }
        "list-2d" | "batch" => {
            // Targets: (bank label, project, events). Default: one project over --bank (Global).
            // --per-bank: every event bank in data_pc/audio/events (optionally --filter S,
            // --max-banks N), each loaded alone with Init.bnk, its own events only.
            let ga = GameAudio::open().ok().map(std::sync::Arc::new);
            let mut targets: Vec<(String, Vec<u32>)> = vec![];
            if a.per_bank {
                let g = ga.as_ref().ok_or("--per-bank needs the game data")?;
                for f in g.banks() {
                    let name = f.path.rsplit('/').next().unwrap().to_string();
                    if name == "Init.bnk" || name == "Global.bnk" || f.size < 16 {
                        continue;
                    }
                    if let Some(flt) = &a.filter {
                        if !name.contains(flt.as_str()) {
                            continue;
                        }
                    }
                    targets.push((name, vec![]));
                }
                targets.truncate(a.max_banks);
            } else {
                targets.push((String::new(), vec![]));
            }
            let mut manifest = String::from("# bank\tevent\tframes\tvoices\tclean\tnotes\n");
            let mut oracle_list = String::new();
            let (mut n_clean, mut n_done, mut n_err, mut n_bank_err) = (0, 0, 0, 0);
            let mut tally: std::collections::BTreeMap<String, usize> = Default::default();
            'banks: for (bank, _) in targets {
                if cmd == "batch" && n_clean >= a.limit {
                    break;
                }
                let p = if bank.is_empty() {
                    project_with(&a, ga.clone(), &a.banks)?
                } else {
                    match project_with(&a, ga.clone(), std::slice::from_ref(&bank)) {
                        Ok(p) => p,
                        Err(e) => {
                            n_bank_err += 1;
                            manifest.push_str(&format!("{}\t-\t0\t0\t0\tbank error: {}\n", bank, e));
                            continue;
                        }
                    }
                };
                let events: Vec<u32> = match &a.events_file {
                    Some(f) => std::fs::read_to_string(f)
                        .map_err(|e| e.to_string())?
                        .lines()
                        .filter(|l| !l.starts_with('#'))
                        .filter_map(|l| l.split_whitespace().last())
                        .map(id_of)
                        .collect(),
                    None if bank.is_empty() => p.events(),
                    None => p.banks.last().unwrap().bank.hirc.iter().filter(|o| o.hirc_type == 4).map(|o| o.id).collect(),
                };
                let label = if bank.is_empty() { a.banks.join("+") } else { bank.clone() };
                for ev in events {
                    if cmd == "batch" && n_clean >= a.limit {
                        break 'banks;
                    }
                    match render_event(&p, &a.gs, &a.opts, ev) {
                        Ok(r) => {
                            n_done += 1;
                            for n in &r.notes {
                                *tally.entry(n.split(':').next().unwrap().to_string()).or_default() += 1;
                            }
                            let clean = is_clean(&r.notes) && r.samples.iter().any(|&x| x != 0.0);
                            if clean {
                                n_clean += 1;
                            }
                            let frames = r.samples.len() / a.opts.out_channels;
                            let notes = r.notes.iter().cloned().collect::<Vec<_>>().join(",");
                            if cmd == "list-2d" && (clean || a.all) {
                                println!("{}\t{}\t{}\t{}\t{}\t{}", label, ev, frames, r.voices, if clean { "clean" } else { "-" }, notes);
                            }
                            if cmd == "batch" && clean {
                                let path = format!("{}/{}.wav", a.out_dir, ev);
                                write_out(&path, &r.samples, a.opts.out_channels as u16, a.i16)?;
                                oracle_list.push_str(&format!("{}\t{}\n", label.trim_end_matches(".bnk"), ev));
                            }
                            manifest.push_str(&format!("{}\t{}\t{}\t{}\t{}\t{}\n", label, ev, frames, r.voices, clean as u8, notes));
                        }
                        Err(e) => {
                            n_err += 1;
                            manifest.push_str(&format!("{}\t{}\t0\t0\t0\terror: {}\n", label, ev, e));
                            if a.all {
                                println!("{}\t{}\terror: {}", label, ev, e);
                            }
                        }
                    }
                }
            }
            eprintln!(
                "{} events rendered, {} clean (2D, deterministic, no node FX, non-silent), {} errors, {} banks failed to load",
                n_done, n_clean, n_err, n_bank_err
            );
            let mut t: Vec<_> = tally.into_iter().collect();
            t.sort_by(|a, b| b.1.cmp(&a.1));
            eprintln!("note tally: {}", t.iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<_>>().join(" "));
            if cmd == "batch" {
                std::fs::create_dir_all(&a.out_dir).map_err(|e| e.to_string())?;
                let m = format!("{}/manifest.tsv", a.out_dir);
                std::fs::write(&m, manifest).map_err(|e| e.to_string())?;
                // bank<TAB>event list for the A2 oracle (AW2_AUDIO_HARNESS_BANKS / _EVENT_IDS)
                std::fs::write(format!("{}/oracle_events.tsv", a.out_dir), oracle_list).map_err(|e| e.to_string())?;
                eprintln!("wrote {}, oracle_events.tsv and {} WAVs to {}", m, n_clean, a.out_dir);
            }
        }
        "patch-2d" => patch_2d_cmd(&a)?,
        "nulltest" => {
            let ra = std::fs::read(a.positional.first().ok_or(USAGE)?).map_err(|e| e.to_string())?;
            let rb = std::fs::read(a.positional.get(1).ok_or(USAGE)?).map_err(|e| e.to_string())?;
            let (ca, _, sa) = wav::read_wav(&ra)?;
            let (cb, _, sb) = wav::read_wav(&rb)?;
            if ca != cb {
                return Err(format!("channel count differs: {} vs {}", ca, cb));
            }
            let c = ca as usize;
            let (oa, ob) = if a.align {
                let first = |s: &[f32]| s.iter().position(|&x| x != 0.0).map_or(0, |p| p / c * c);
                (first(&sa), first(&sb))
            } else {
                (0, 0)
            };
            let (xa, xb) = (&sa[oa..], &sb[ob..]);
            let n = xa.len().min(xb.len());
            let db = |x: f64| if x > 0.0 { 20.0 * x.log10() } else { f64::NEG_INFINITY };
            let (mut peak, mut sq, mut ref_peak) = (0f64, 0f64, 0f64);
            for i in 0..n {
                let d = (xa[i] - xb[i]) as f64;
                peak = peak.max(d.abs());
                sq += d * d;
                ref_peak = ref_peak.max(xa[i].abs() as f64);
            }
            // residual beyond the common length counts as error too
            let tail_a: f64 = xa[n..].iter().map(|&x| (x as f64).abs()).fold(0.0, f64::max);
            let tail_b: f64 = xb[n..].iter().map(|&x| (x as f64).abs()).fold(0.0, f64::max);
            println!(
                "aligned offsets a={} b={} frames | len a={} b={} frames (diff {}) | ref peak {:.2} dBFS | residual peak {:.2} dBFS, rms {:.2} dBFS | tail peak a {:.2} b {:.2} dBFS",
                oa / c,
                ob / c,
                xa.len() / c,
                xb.len() / c,
                (xa.len() as i64 - xb.len() as i64) / c as i64,
                db(ref_peak),
                db(peak),
                db((sq / n.max(1) as f64).sqrt()),
                db(tail_a),
                db(tail_b)
            );
        }
        _ => return Err(USAGE.into()),
    }
    Ok(())
}

fn parse_u32(s: &str) -> Option<u32> {
    match s.strip_prefix("0x") {
        Some(h) => u32::from_str_radix(h, 16).ok(),
        None => s.parse().ok(),
    }
}
