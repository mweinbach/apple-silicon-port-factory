//! `d3d12-replay <trace> [--stats] [--dump] [--shaders] [--replay] [--backend null|metal] [--bench] [--golden <path>] [options]`

#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("d3d12-replay runs on macOS only");
    std::process::exit(2);
}

#[cfg(target_os = "macos")]
fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    if let Err(e) = run() {
        eprintln!("error: {e:#}");
        std::process::exit(1);
    }
}

#[cfg(target_os = "macos")]
const USAGE: &str = "\
usage: d3d12-replay <trace> [modes] [options]
       d3d12-replay --generate <name> --out <file>

modes (default: --stats):
  --stats               per-op counts, frames, draws, object churn
  --dump                one line per record
  --shaders             build every root signature, convert every shader stage (fills the
                        shader cache), translate PSO state; report failures and unsupported
                        features
  --replay              replay through --backend (default: null)
  --bench               benchmark repeatable gpu_busy and frame-time stats on Metal 4
                        (implies --backend metal)

options:
  --backend null|metal  replay backend (default: null; --bench/--golden default to metal)
  --iters N             measured benchmark iterations for --bench (default: 20)
  --warmup N            warmup iterations for --bench (default: 5)
  --golden <path>       compare rendered back buffer against golden .png or pixel-hash file
  --write-golden <path> write rendered back buffer to .png or pixel-hash file
  --max-delta N         maximum per-channel delta (0..255) for --golden PNG comparison (default: 0)
  --until N             stop after record N
  --jobs N              conversion threads for --shaders (default: all cores)
  --max-messages N      failures/warnings listed per category (default: 200)
  --generate <name>     emit a synthetic trace (many-draws, descriptor-churn, indirect-draws,
                        barrier-chains, clear-resolve) to --out <file>";

#[cfg(target_os = "macos")]
fn run() -> anyhow::Result<()> {
    use std::path::PathBuf;

    use anyhow::{bail, Context};
    use d3d12_replay::{bench, golden, metal_backend, replay, shaders, stats, synthetic};

    let mut path = None;
    let (mut do_stats, mut do_dump, mut do_shaders, mut do_replay, mut do_bench) =
        (false, false, false, false, false);
    let mut backend_name: Option<String> = None;
    let mut golden_path: Option<PathBuf> = None;
    let mut write_golden_path: Option<PathBuf> = None;
    let mut max_delta: u8 = 0;
    let mut bench_opts = bench::BenchOptions::default();
    let mut until = None;
    let mut sopts = shaders::Options::default();
    let mut generate_name: Option<String> = None;
    let mut out_path: Option<PathBuf> = None;

    let mut args = std::env::args().skip(1);
    let num = |a: Option<String>, flag: &str| -> anyhow::Result<u64> {
        a.with_context(|| format!("{flag} needs a value"))?
            .parse()
            .with_context(|| format!("bad {flag}"))
    };
    while let Some(a) = args.next() {
        match a.as_str() {
            "--stats" => do_stats = true,
            "--dump" => do_dump = true,
            "--shaders" => do_shaders = true,
            "--replay" => do_replay = true,
            "--bench" => do_bench = true,
            "--backend" => {
                backend_name = Some(args.next().context("--backend needs a value (null|metal)")?);
            }
            "--iters" => bench_opts.iterations = num(args.next(), "--iters")?.max(1) as usize,
            "--warmup" => bench_opts.warmup = num(args.next(), "--warmup")? as usize,
            "--golden" => {
                golden_path = Some(PathBuf::from(args.next().context("--golden needs a path")?));
            }
            "--write-golden" => {
                write_golden_path = Some(PathBuf::from(args.next().context("--write-golden needs a path")?));
            }
            "--max-delta" => max_delta = num(args.next(), "--max-delta")?.min(255) as u8,
            "--until" => until = Some(num(args.next(), "--until")?),
            "--jobs" => sopts.jobs = num(args.next(), "--jobs")? as usize,
            "--max-messages" => sopts.max_messages = num(args.next(), "--max-messages")? as usize,
            "--generate" => {
                generate_name = Some(args.next().context("--generate needs a synthetic trace name")?);
            }
            "--out" => {
                out_path = Some(PathBuf::from(args.next().context("--out needs a path")?));
            }
            "-h" | "--help" => {
                println!("{USAGE}");
                return Ok(());
            }
            s if s.starts_with('-') => bail!("unknown option {s}\n\n{USAGE}"),
            _ if path.is_none() => path = Some(a),
            _ => bail!("more than one trace given\n\n{USAGE}"),
        }
    }

    if let Some(name) = generate_name {
        let out = out_path.context("--generate requires --out <file>")?;
        let bytes = synthetic::generate_by_name(&name).map_err(|e| anyhow::anyhow!(e))?;
        if let Some(dir) = out.parent() {
            if !dir.as_os_str().is_empty() {
                std::fs::create_dir_all(dir)?;
            }
        }
        std::fs::write(&out, &bytes).with_context(|| format!("writing {}", out.display()))?;
        println!("generated synthetic trace {name} ({} bytes) -> {}", bytes.len(), out.display());
        return Ok(());
    }

    let Some(path) = path else { bail!("{USAGE}") };
    let wants_golden = golden_path.is_some() || write_golden_path.is_some();
    let backend_kind = match backend_name.as_deref() {
        Some("null") => "null",
        Some("metal") => "metal",
        Some(other) => bail!("unknown backend {other:?} (expected 'null' or 'metal')"),
        None if do_bench || wants_golden => "metal",
        None => "null",
    };
    if backend_name.as_deref() == Some("metal") && !(do_bench || do_replay) {
        do_replay = true;
    }
    if wants_golden && !(do_bench || do_replay) {
        do_replay = true;
    }
    if !(do_dump || do_shaders || do_replay || do_bench) {
        do_stats = true;
    }
    if (do_bench || wants_golden) && backend_kind != "metal" {
        bail!("--bench and --golden require --backend metal");
    }

    sopts.until = until;
    bench_opts.until = until;
    let data = std::fs::read(&path).with_context(|| format!("reading {path}"))?;

    if do_dump {
        let stdout = std::io::stdout();
        let mut out = std::io::BufWriter::new(stdout.lock());
        stats::dump(&data, until, &mut out)?;
    }
    if do_stats {
        print!("{}", stats::stats(&data, until)?);
    }

    let mut captured_backend: Option<metal_backend::MetalBackend> = None;
    if do_replay {
        match backend_kind {
            "null" => {
                let mut backend = replay::NullBackend;
                print!("{}", replay::replay(&data, &mut backend, until, sopts.max_messages)?);
            }
            "metal" => {
                let mut backend = metal_backend::MetalBackend::new().map_err(|e| anyhow::anyhow!(e))?;
                let rep = replay::replay(&data, &mut backend, until, sopts.max_messages)?;
                print!("{rep}");
                if rep.failed.total > 0 {
                    bail!("metal replay had {} failure(s)", rep.failed.total);
                }
                captured_backend = Some(backend);
            }
            _ => unreachable!(),
        }
    }
    if do_bench {
        let (rep, backend) = bench::run_bench(&data, &bench_opts)?;
        print!("{rep}");
        if !rep.failures.is_empty() {
            bail!("metal bench had {} failure(s)", rep.failures.len());
        }
        captured_backend = Some(backend);
    }
    if wants_golden {
        let mut backend = captured_backend.context("no metal backend state available for golden check")?;
        let img = backend.capture_frame().map_err(|e| anyhow::anyhow!(e))?;
        if let Some(wp) = &write_golden_path {
            golden::write_golden(wp, &img).map_err(|e| anyhow::anyhow!(e))?;
            println!("wrote golden {} ({}x{}, hash={})", wp.display(), img.width, img.height, img.hash_hex());
        }
        if let Some(gp) = &golden_path {
            let check = golden::verify_golden(gp, &img, max_delta).map_err(|e| anyhow::anyhow!(e))?;
            print!("{check}");
            if !check.passed {
                bail!("golden mismatch for {}", gp.display());
            }
        }
    }
    if do_shaders {
        let p = dxil_metal::ShaderPipeline::global().map_err(|e| anyhow::anyhow!("shader pipeline: {e}"))?;
        print!("{}", shaders::validate(&data, p, &sopts)?);
    }
    Ok(())
}
