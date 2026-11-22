//! `d3d12-replay <trace> [--stats] [--dump] [--shaders] [--replay] [options]`

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

modes (default: --stats):
  --stats            per-op counts, frames, draws, object churn
  --dump             one line per record
  --shaders          build every root signature, convert every shader stage (fills the
                     shader cache), translate PSO state; report failures and unsupported
                     features. AW2_SHADER_BACKEND=helper isolates converter crashes.
  --replay           replay through the validating null backend (object table, list state)

options:
  --until N          stop after record N
  --jobs N           conversion threads for --shaders (default: all cores)
  --max-messages N   failures/warnings listed per category (default 200)

Record traces with AW2_D3D12_TRACE=<file> in the game process.";

#[cfg(target_os = "macos")]
fn run() -> anyhow::Result<()> {
    use anyhow::{bail, Context};
    use d3d12_replay::{replay, shaders, stats};

    let mut path = None;
    let (mut do_stats, mut do_dump, mut do_shaders, mut do_replay) = (false, false, false, false);
    let mut until = None;
    let mut sopts = shaders::Options::default();
    let mut args = std::env::args().skip(1);
    let num = |a: Option<String>, flag: &str| -> anyhow::Result<u64> { a.with_context(|| format!("{flag} needs a value"))?.parse().with_context(|| format!("bad {flag}")) };
    while let Some(a) = args.next() {
        match a.as_str() {
            "--stats" => do_stats = true,
            "--dump" => do_dump = true,
            "--shaders" => do_shaders = true,
            "--replay" => do_replay = true,
            "--until" => until = Some(num(args.next(), "--until")?),
            "--jobs" => sopts.jobs = num(args.next(), "--jobs")? as usize,
            "--max-messages" => sopts.max_messages = num(args.next(), "--max-messages")? as usize,
            "-h" | "--help" => {
                println!("{USAGE}");
                return Ok(());
            }
            s if s.starts_with('-') => bail!("unknown option {s}\n\n{USAGE}"),
            _ if path.is_none() => path = Some(a),
            _ => bail!("more than one trace given\n\n{USAGE}"),
        }
    }
    let Some(path) = path else { bail!("{USAGE}") };
    if !(do_dump || do_shaders || do_replay) {
        do_stats = true;
    }
    sopts.until = until;
    let data = std::fs::read(&path).with_context(|| format!("reading {path}"))?;

    if do_dump {
        let stdout = std::io::stdout();
        let mut out = std::io::BufWriter::new(stdout.lock());
        stats::dump(&data, until, &mut out)?;
    }
    if do_stats {
        print!("{}", stats::stats(&data, until)?);
    }
    if do_replay {
        let mut backend = replay::NullBackend;
        print!("{}", replay::replay(&data, &mut backend, until, sopts.max_messages)?);
    }
    if do_shaders {
        let p = dxil_metal::ShaderPipeline::global().map_err(|e| anyhow::anyhow!("shader pipeline: {e}"))?;
        print!("{}", shaders::validate(&data, p, &sopts)?);
    }
    Ok(())
}
