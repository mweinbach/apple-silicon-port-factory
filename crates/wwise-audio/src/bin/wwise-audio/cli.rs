use wwise_audio::codec::decode_wem;
use wwise_audio::data::GameAudio;
use wwise_audio::wav;

const USAGE: &str = "usage: wwise-audio <command>
  bank-info <vpath|file.bnk>          chunks + HIRC object summary of one bank
  decode <vpath|file.wem> <out.wav> [--float]   decode one .wem to WAV
  wem-info <vpath|file.wem>           .wem header
  inventory [--wem-sample N]          scan all banks (+ .wem headers) and print the inventory
  compare <a.wav> <b.wav>             sample-level diff of two WAVs (e.g. vs vgmstream-cli)
  validate [--per-class N] [--vgmstream PATH] [--out DIR] [--keep]
                                      decode N .wem per stream class, A/B against vgmstream-cli
  replay <trace.akt> [<out.wav>]      replay an AW2AKT01 trace through NativeSoundEngine
";

fn load(arg: &str, ga: &mut Option<GameAudio>) -> Result<Vec<u8>, String> {
    if std::path::Path::new(arg).is_file() {
        return std::fs::read(arg).map_err(|e| e.to_string());
    }
    if ga.is_none() {
        *ga = Some(GameAudio::open()?);
    }
    ga.as_ref().unwrap().read(arg)
}

pub fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut ga = None;
    match args.first().map(String::as_str) {
        Some("decode") => {
            let src = args.get(1).ok_or(USAGE)?;
            let out = args.get(2).ok_or(USAGE)?;
            let float = args.iter().any(|a| a == "--float");
            let buf = load(src, &mut ga)?;
            let t = std::time::Instant::now();
            let d = decode_wem(&buf)?;
            let el = t.elapsed();
            let f = std::fs::File::create(out).map_err(|e| e.to_string())?;
            wav::write_wav(std::io::BufWriter::new(f), d.channels, d.sample_rate, &d.samples, float)
                .map_err(|e| e.to_string())?;
            let frames = d.samples.len() / d.channels.max(1) as usize;
            println!(
                "{}: {} ch, {} Hz, {} frames ({:.3} s), {} — decoded in {:.1} ms",
                src,
                d.channels,
                d.sample_rate,
                frames,
                frames as f64 / d.sample_rate as f64,
                d.detail,
                el.as_secs_f64() * 1e3
            );
        }
        Some("wem-info") => {
            let src = args.get(1).ok_or(USAGE)?;
            let buf = load(src, &mut ga)?;
            let w = wwise_audio::wem::Wem::parse(&buf).map_err(|e| e.to_string())?;
            println!("{:#?}", w);
        }
        Some("compare") => {
            let a = std::fs::read(args.get(1).ok_or(USAGE)?).map_err(|e| e.to_string())?;
            let b = std::fs::read(args.get(2).ok_or(USAGE)?).map_err(|e| e.to_string())?;
            let (ca, ra, sa) = wav::read_wav(&a)?;
            let (cb, rb, sb) = wav::read_wav(&b)?;
            let r = compare(&sa, &sb);
            println!(
                "a: {} ch {} Hz {} samples | b: {} ch {} Hz {} samples\n{}",
                ca, ra, sa.len(), cb, rb, sb.len(), r
            );
        }
        Some("bank-info") => bank_info(args.get(1).ok_or(USAGE)?, &mut ga)?,
        Some("validate") => crate::validate::validate(&args[1..])?,
        Some("inventory") => {
            let sample = args
                .iter()
                .position(|a| a == "--wem-sample")
                .and_then(|i| args.get(i + 1))
                .and_then(|s| s.parse().ok())
                .unwrap_or(usize::MAX);
            inventory(sample)?
        }
        Some("replay") => {
            let trace_path = args.get(1).ok_or(USAGE)?;
            let out_wav = args.get(2);
            let bytes = std::fs::read(trace_path).map_err(|e| e.to_string())?;
            let mut banks_os = std::path::Path::new(trace_path).as_os_str().to_owned();
            banks_os.push(".banks");
            let banks_dir = std::path::PathBuf::from(banks_os);
            let engine = wwise_audio::runtime::NativeSoundEngine::default();
            if let Some(wav_path) = out_wav {
                let _ = engine.start_output_capture(wav_path);
            }
            let report = engine.replay_trace(&bytes, &banks_dir, 4)?;
            if out_wav.is_some() {
                engine.stop_output_capture()?;
            }
            println!(
                "replayed {}: {}/{} records, {} banks, {} events, {} render frames ({} samples)",
                trace_path,
                report.records_applied,
                report.records_total,
                report.banks_loaded,
                report.events_posted,
                report.frames_rendered,
                report.samples.len()
            );
        }
        Some(c @ ("render" | "plan" | "list-2d" | "batch" | "nulltest" | "patch-2d")) => crate::render::run(c, &args[1..])?,
        _ => return Err(format!("{}{}", USAGE, crate::render::USAGE)),
    }
    Ok(())
}

pub struct CompareResult {
    pub len_a: usize,
    pub len_b: usize,
    pub max_abs: f32,
    pub rms: f64,
    pub exact: usize,
}

impl std::fmt::Display for CompareResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = self.len_a.min(self.len_b);
        write!(
            f,
            "compared {} samples (len a={} b={}): max |diff| = {:.3e} ({:.2} LSB16), rms diff = {:.3e}, exactly equal: {} ({:.2}%)",
            n,
            self.len_a,
            self.len_b,
            self.max_abs,
            self.max_abs * 32767.0,
            self.rms,
            self.exact,
            100.0 * self.exact as f64 / n.max(1) as f64
        )
    }
}

pub fn compare(a: &[f32], b: &[f32]) -> CompareResult {
    let n = a.len().min(b.len());
    let (mut max_abs, mut sq, mut exact) = (0f32, 0f64, 0usize);
    for i in 0..n {
        let d = (a[i] - b[i]).abs();
        if d == 0.0 {
            exact += 1;
        }
        max_abs = max_abs.max(d);
        sq += (d as f64) * (d as f64);
    }
    CompareResult { len_a: a.len(), len_b: b.len(), max_abs, rms: (sq / n.max(1) as f64).sqrt(), exact }
}

pub fn bank_info(src: &str, ga: &mut Option<GameAudio>) -> Result<(), String> {
    use wwise_audio::bnk::{hirc::hirc_type_name, plugin_name, Bank};
    let buf = load(src, ga)?;
    let b = Bank::parse(&buf).map_err(|e| e.to_string())?;
    let h = &b.header;
    println!(
        "{}: version {} bank_id {} ({:#010x}) language {:#010x} project {} type {} align {}",
        src, h.version, h.bank_id, h.bank_id, h.language_id, h.project_id, h.bank_type, h.alt_values & 0xFFFF
    );
    for c in &b.chunks {
        println!("  chunk {} @ {:#x} size {}", c.name(), c.offset, c.size);
    }
    if !b.media_index.is_empty() {
        let total: u64 = b.media_index.iter().map(|m| m.size as u64).sum();
        println!("  DIDX: {} embedded media, {} bytes", b.media_index.len(), total);
    }
    let mut counts = std::collections::BTreeMap::new();
    for o in &b.hirc {
        *counts.entry(o.hirc_type).or_insert(0usize) += 1;
    }
    println!("  HIRC: {} objects parsed, {} errors", b.hirc.len(), b.hirc_errors.len());
    for (t, n) in &counts {
        println!("    {:#04x} {:<18} {}", t, hirc_type_name(*t), n);
    }
    for (t, id, off, e) in b.hirc_errors.iter().take(20) {
        println!("    ERROR type {:#04x} id {} @ {:#x}: {}", t, id, off, e);
    }
    if !b.plugins.is_empty() {
        println!("  INIT plugins:");
        for (id, dll) in &b.plugins {
            println!("    {:#010x} {:<40} {}", id, plugin_name(*id), dll);
        }
    }
    if let Some(g) = &b.global_settings {
        println!(
            "  STMG: volume threshold {} dB, max voices {}, max dangerous virt voices {}, {} state groups, {} switch groups, {} game parameters, {} acoustic textures",
            g.volume_threshold, g.max_num_voices, g.max_num_dangerous_virt_voices, g.state_groups.len(),
            g.switch_groups.len(), g.rtpc_ramping.len(), g.acoustic_textures.len()
        );
    }
    if let Some(p) = &b.custom_platform {
        println!("  PLAT: {:?}", p);
    }
    if std::env::var("AW2_AUDIO_VERBOSE").is_ok() {
        for o in &b.hirc {
            println!("{:#?}", o);
        }
    }
    Ok(())
}

pub use crate::inventory::inventory;
