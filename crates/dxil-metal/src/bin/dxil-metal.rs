//! `dxil-metal` CLI.
//!
//! ```text
//! cargo run -p dxil-metal --bin dxil-metal -- prewarm [--game DIR] [--jobs N] [--filter S]
//!     [--limit N] [--no-cache] [--linear] [--backend inprocess|helper] [--manifest [DIR]] [--record DIR] [--list-failures N]
//! ```
//! By default each non-library shader is compiled against a root signature synthesized from its
//! PSV0 bindings (`--linear` = no root signature, the converter's linear layout).
//! `--manifest [DIR]` instead replays the compile requests the game recorded (default
//! `<cache root>/manifest`, see `dxil_metal::manifest`) with their real root signatures;
//! `--record DIR` records this run's requests into a manifest.
//! ```text
//! cargo run -p dxil-metal --bin dxil-metal -- info <bundle.binrfx>
//! cargo run -p dxil-metal --bin dxil-metal -- stats
//! ```
//! `prewarm` converts every program of every `.binrfx` bundle through the shared cache and
//! prints a failure taxonomy by error code / shader profile / feature flags.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use aw2_data::{discover_game_dir, BinRfx};
use dxil_metal::dxbc::{self, FEATURE_NAMES};
use dxil_metal::*;

struct Job {
    bundle: String,
    stage_name: &'static str,
    entry: String,
    dxil: Vec<u8>,
    /// Manifest replay: the recorded request (root signatures as RTS0, options, stage).
    recorded: Option<Recorded>,
}

struct Recorded {
    entry_point: Option<String>,
    root_signature: Option<Vec<u8>>,
    local_root_signature: Option<Vec<u8>>,
    expected_stage: Option<ShaderStage>,
    options: CompileOptions,
}

/// Jobs from a manifest (`prewarm --manifest`); unreadable entries go to `failures`.
fn manifest_jobs(dir: &std::path::Path, failures: &mut Vec<String>) -> Vec<Job> {
    let m = manifest::Manifest::load(dir).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(2)
    });
    let mut jobs = Vec::new();
    for e in &m.entries {
        let load = || -> Result<Job, Error> {
            let dxil = m.dxil(e)?;
            let rs = e.root_signature.map(|h| m.root_signature_blob(h)).transpose()?;
            let lrs = e.local_root_signature.map(|h| m.root_signature_blob(h)).transpose()?;
            let entry = e.entry_point.clone().or_else(|| dxbc::Container::parse(&dxil).ok()?.psv_entry_point()).unwrap_or_default();
            Ok(Job {
                bundle: "manifest".into(),
                stage_name: "recorded",
                entry,
                dxil,
                recorded: Some(Recorded {
                    entry_point: e.entry_point.clone(),
                    root_signature: rs,
                    local_root_signature: lrs,
                    expected_stage: e.expected_stage,
                    options: e.options.clone(),
                }),
            })
        };
        match load() {
            Ok(j) => jobs.push(j),
            Err(err) => failures.push(format!("manifest entry {:032x}: {err}", e.key)),
        }
    }
    jobs
}

#[derive(Debug)]
struct Outcome {
    bundle: String,
    stage_name: &'static str,
    entry: String,
    profile: String,
    features: u64,
    result: Result<(ShaderStage, usize), String>,
    code: Option<ErrorCode>,
    with_root_signature: bool,
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1).cloned())
}

fn game_dir_from_args(args: &[String]) -> PathBuf {
    arg_value(args, "--game")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("AW2_GAME_DIR").map(PathBuf::from))
        .or_else(discover_game_dir)
        .unwrap_or_else(|| {
            eprintln!("game not found (pass --game DIR or set AW2_GAME_DIR)");
            std::process::exit(2)
        })
}

fn shader_dir(args: &[String]) -> PathBuf {
    game_dir_from_args(args).join("data/shaders/build/pc_dx12")
}

fn bundles(dir: &PathBuf, filter: Option<&str>) -> Vec<PathBuf> {
    let mut v: Vec<PathBuf> = std::fs::read_dir(dir)
        .unwrap_or_else(|e| {
            eprintln!("{}: {e}", dir.display());
            std::process::exit(2)
        })
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "binrfx"))
        .filter(|p| filter.is_none_or(|f| p.file_name().unwrap().to_string_lossy().contains(f)))
        .collect();
    v.sort();
    v
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("prewarm") => prewarm(&args[1..], true, true),
        Some("prewarm-rfx" | "scan-rfx") => prewarm(&args[1..], true, false),
        Some("prewarm-pe" | "scan-pe") => prewarm(&args[1..], false, true),
        Some("info") => info(&args[1..]),
        Some("dxbc2dxil") => dxbc2dxil(&args[1..]),
        Some("stats") => {
            let cfg = CacheConfig::from_env();
            println!("cache dir: {:?}", cfg.disk_dir);
            match ShaderPipeline::new(cfg) {
                Ok(p) => println!("converter: {}", p.converter_fingerprint()),
                Err(e) => println!("converter: {e}"),
            }
        }
        _ => {
            eprintln!("usage: dxil-metal prewarm [--game DIR] [--jobs N] [--filter S] [--limit N] [--no-cache] [--linear] [--skip-sm69] [--backend inprocess|helper] [--manifest [DIR]] [--record DIR] [--report FILE] [--list-failures N]");
            eprintln!("       dxil-metal scan-rfx [--game DIR] [--jobs N] [--filter S] [--no-cache] [--skip-sm69] [--report FILE]");
            eprintln!("       dxil-metal scan-pe [--game DIR] [--jobs N] [--filter S] [--no-cache] [--report FILE]");
            eprintln!("       dxil-metal info <bundle.binrfx | shader.dxil>");
            eprintln!("       dxil-metal dxbc2dxil <in.dxbc> <out.dxil> [--compile]   (SM4/5 -> DXIL via libdxilconv)");
            eprintln!("       dxil-metal stats");
            std::process::exit(2);
        }
    }
}

fn info(args: &[String]) {
    let path = args.first().expect("info <bundle.binrfx | shader.dxil>");
    if let Ok(bytes) = std::fs::read(path) {
        if dxbc::Container::is_container(&bytes) {
            return container_info(&bytes);
        }
    }
    let rfx = rfx::RfxBundle::open(path).expect("parse binrfx");
    println!(
        "{} v{} '{}' programs={} techniques={} permutations={}",
        rfx.file_name,
        rfx.version,
        rfx.shader_name,
        rfx.programs.len(),
        rfx.techniques.len(),
        rfx.permutations.len()
    );
    for p in &rfx.programs {
        let s = dxbc::summarize(&p.dxbc_bytes);
        match s {
            Ok(s) => println!(
                "  [{:<13}] {:<40} {:<12} {:>8}B hash={:#018x} res={} cb={} psv_entry={:?} rts0={} features={:?}",
                p.stage_slot.name(),
                p.entry_point,
                s.program.map_or("?".into(), |x| x.profile()),
                p.dxbc_bytes.len(),
                p.shader_hash,
                p.resources.len(),
                p.cbuffers.len(),
                s.entry_point,
                s.has_root_signature,
                dxbc::feature_names(s.features)
            ),
            Err(e) => println!("  [{:<13}] {:<40} {e}", p.stage_slot.name(), p.entry_point),
        }
    }
}

/// Summary of one DXBC/DXIL container, including how the entry point is resolved.
fn container_info(bytes: &[u8]) {
    let c = match dxbc::Container::parse(bytes) {
        Ok(c) => c,
        Err(e) => return println!("{e}"),
    };
    let parts: Vec<String> = c.parts.iter().map(|p| format!("{}({})", String::from_utf8_lossy(&p.fourcc), p.data.len())).collect();
    println!("{} bytes, parts: {}", bytes.len(), parts.join(" "));
    println!("program: {}", c.program().map_or("?".into(), |p| format!("{} (dxil {:#x})", p.profile(), p.dxil_version)));
    println!("psv0 entry: {:?}", c.psv_entry_point());
    println!("bitcode defined functions: {:?}", c.defined_functions());
    println!("resolved entry point: {:?}", c.entry_point());
    println!("features: {:?}", dxbc::feature_names(c.feature_flags()));
}

/// Converts one SM4/SM5 DXBC container to (signed) DXIL; `--compile` also runs it through the
/// Metal Shader Converter without a root signature.
fn dxbc2dxil(args: &[String]) {
    let (Some(input), Some(output)) = (args.first(), args.get(1)) else {
        eprintln!("usage: dxil-metal dxbc2dxil <in.dxbc> <out.dxil> [--compile]");
        std::process::exit(2)
    };
    let dxbc = std::fs::read(input).unwrap_or_else(|e| {
        eprintln!("{input}: {e}");
        std::process::exit(2)
    });
    let t = std::time::Instant::now();
    let dxil = match dxil_metal::dxilconv::convert_signed(&dxbc) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{input}: {e}");
            std::process::exit(1)
        }
    };
    std::fs::write(output, &dxil).expect("write output");
    println!("{input} ({} B) -> {output} ({} B) in {:.1} ms", dxbc.len(), dxil.len(), t.elapsed().as_secs_f64() * 1e3);
    container_info(&dxil);
    if args.iter().any(|a| a == "--compile") {
        let opts = dxil_metal::CompileOptions::default();
        match dxil_metal::compile_uncached(&dxil_metal::CompileRequest::new(&dxil, None, &opts)) {
            Ok(s) => println!("metal: {:?} '{}' {} B metallib", s.stage, s.function_name, s.metallib.len()),
            Err(e) => {
                println!("metal: {e}");
                std::process::exit(1)
            }
        }
    }
}

fn prewarm(args: &[String], include_rfx: bool, include_pe: bool) {
    let dir = if args.iter().any(|a| a == "--manifest") { PathBuf::new() } else { shader_dir(args) };
    let jobs_n: usize = arg_value(args, "--jobs")
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| std::thread::available_parallelism().map_or(8, |n| n.get()));
    let limit: usize = arg_value(args, "--limit").and_then(|s| s.parse().ok()).unwrap_or(usize::MAX);
    let list_failures: usize = arg_value(args, "--list-failures").and_then(|s| s.parse().ok()).unwrap_or(40);
    let no_psv_rs = args.iter().any(|a| a == "--linear");
    let skip_sm69 = args.iter().any(|a| a == "--skip-sm69");
    let report_path = arg_value(args, "--report").map(PathBuf::from);
    let trace = std::env::var_os("AW2_PREWARM_TRACE").is_some();
    let filter = arg_value(args, "--filter");
    let cfg = if args.iter().any(|a| a == "--no-cache") { CacheConfig::memory_only() } else { CacheConfig::from_env() };
    let backend = match arg_value(args, "--backend").as_deref() {
        Some("helper") => Some(BackendKind::Helper),
        Some("inprocess") => Some(BackendKind::InProcess),
        Some("cache-only" | "cache") => Some(BackendKind::CacheOnly),
        Some(other) => {
            eprintln!("unknown backend {other} (inprocess | helper | cache-only)");
            std::process::exit(2)
        }
        None => None,
    };
    let pipeline = match backend {
        Some(kind) => ShaderPipeline::with_backend(cfg.clone(), kind),
        None => ShaderPipeline::new(cfg.clone()),
    }
    .unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(2)
    });

    if let Some(d) = arg_value(args, "--record") {
        match manifest::ManifestWriter::open(&d) {
            Ok(m) => pipeline.set_manifest(Some(std::sync::Arc::new(m))),
            Err(e) => {
                eprintln!("--record: {e}");
                std::process::exit(2)
            }
        }
    }
    let t0 = Instant::now();
    let manifest_dir = args.iter().position(|a| a == "--manifest").map(|i| match args.get(i + 1) {
        Some(d) if !d.starts_with("--") => PathBuf::from(d),
        _ => CacheConfig::from_env().disk_dir.map(|d| d.join("manifest")).unwrap_or_else(|| {
            eprintln!("--manifest: no directory given and the disk cache is disabled");
            std::process::exit(2)
        }),
    });
    if manifest_dir.is_none() {
        let mut combined_text = String::new();
        if include_rfx {
            let corpus_opts = rfx::CorpusOptions {
                shader_dir: dir.clone(),
                filter: filter.clone(),
                limit,
                threads: jobs_n,
                linear_layout: no_psv_rs,
                skip_sm69,
            };
            let corpus_report = rfx::scan_and_prewarm_corpus(&corpus_opts, &pipeline).unwrap_or_else(|e| {
                eprintln!("scan_and_prewarm_corpus: {e}");
                std::process::exit(2);
            });
            let text = corpus_report.format_report(list_failures);
            print!("{text}");
            combined_text.push_str(&text);
        }
        if include_pe {
            if !combined_text.is_empty() {
                println!();
                combined_text.push('\n');
            }
            let pe_opts = pe::PeCorpusOptions {
                game_dir: game_dir_from_args(args),
                filter: filter.clone(),
                limit,
                threads: jobs_n,
                linear_layout: no_psv_rs,
            };
            let pe_report = pe::scan_and_prewarm_pe_corpus(&pe_opts, &pipeline).unwrap_or_else(|e| {
                eprintln!("scan_and_prewarm_pe_corpus: {e}");
                std::process::exit(2);
            });
            let text = pe_report.format_report(list_failures);
            print!("{text}");
            combined_text.push_str(&text);
        }
        if let Some(rp) = report_path {
            if let Some(parent) = rp.parent() {
                if !parent.as_os_str().is_empty() {
                    std::fs::create_dir_all(parent).expect("create report parent dir");
                }
            }
            std::fs::write(&rp, &combined_text).unwrap_or_else(|e| {
                eprintln!("write {}: {e}", rp.display());
                std::process::exit(2);
            });
            println!("\nReport written to {}", rp.display());
        }
        return;
    }
    let paths = if manifest_dir.is_some() { Vec::new() } else { bundles(&dir, filter.as_deref()) };
    let mut jobs = Vec::new();
    let mut parse_failures = Vec::new();
    if let Some(d) = &manifest_dir {
        jobs = manifest_jobs(d, &mut parse_failures);
        println!("manifest {}: {} recorded requests", d.display(), jobs.len() + parse_failures.len());
    }
    for p in &paths {
        match BinRfx::open(p) {
            Ok(rfx) => {
                for prog in rfx.programs {
                    // DXIL libraries (DXR) have no entry name in the bundle: compile every
                    // export listed in RDAT, as a state object would.
                    let exports: Vec<String> = if prog.entry_point.is_empty() {
                        dxbc::Container::parse(&prog.dxbc_bytes)
                            .map(|c| c.library_functions().into_iter().map(|f| f.unmangled_name).collect())
                            .unwrap_or_default()
                    } else {
                        vec![prog.entry_point.clone()]
                    };
                    if exports.is_empty() {
                        parse_failures.push(format!("{} [{}]: no entry point / exports", rfx.file_name, prog.stage_name));
                    }
                    for entry in exports {
                        jobs.push(Job {
                            bundle: rfx.file_name.clone(),
                            stage_name: prog.stage_name,
                            entry,
                            dxil: prog.dxbc_bytes.clone(),
                            recorded: None,
                        });
                    }
                }
            }
            Err(e) => parse_failures.push(format!("{}: {e}", p.display())),
        }
    }
    jobs.truncate(limit);
    let total_bytes: usize = jobs.iter().map(|j| j.dxil.len()).sum();
    println!(
        "prewarm: {} bundles, {} jobs ({:.1} MB DXIL), {} threads, cache {:?}, converter {}, backend {}",
        paths.len(),
        jobs.len(),
        total_bytes as f64 / 1e6,
        jobs_n,
        cfg.disk_dir,
        pipeline.converter_fingerprint(),
        match pipeline.helper() {
            Some(h) => format!("helper {}", h.executable().display()),
            None => "in-process".into(),
        }
    );

    let next = AtomicUsize::new(0);
    let done = AtomicUsize::new(0);
    let outcomes = Mutex::new(Vec::with_capacity(jobs.len()));
    let opts = CompileOptions::default();
    std::thread::scope(|s| {
        for _ in 0..jobs_n {
            s.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::Relaxed);
                let Some(job) = jobs.get(i) else { break };
                if trace {
                    eprintln!("compile {} {} {}", i, job.bundle, job.entry);
                }
                let summary = dxbc::summarize(&job.dxil).ok();
                let mut profile = summary.as_ref().and_then(|s| s.program).map_or("unknown".into(), |p| p.profile());
                let mut features = summary.as_ref().map_or(0, |s| s.features);
                // For library exports, report the export kind and its own feature bits.
                if let Some(f) = summary.as_ref().and_then(|s| s.library_functions.iter().find(|f| f.unmangled_name == job.entry)) {
                    profile = format!("{}:{}", profile, f.kind.short_name());
                    features |= f.features;
                }
                // Game shaders carry no root signature (the game creates them at runtime), so
                // compile against one covering the shader's PSV0/RDAT bindings (closest to
                // runtime), else the converter's linear layout.
                let (rs, lrs, r);
                if let Some(rec) = &job.recorded {
                    // Manifest replay: exactly the request the game made.
                    let load = |b: &Option<Vec<u8>>| b.as_ref().map(|b| pipeline.root_signature(b)).transpose();
                    r = match (load(&rec.root_signature), load(&rec.local_root_signature)) {
                        (Ok(g), Ok(l)) => {
                            rs = g;
                            lrs = l;
                            let mut req = CompileRequest::new(&job.dxil, rs.as_deref(), &rec.options);
                            req.entry_point = rec.entry_point.as_deref();
                            req.expected_stage = rec.expected_stage;
                            req.local_root_signature = lrs.as_deref();
                            pipeline.compile(&req)
                        }
                        (Err(e), _) | (_, Err(e)) => {
                            rs = None;
                            Err(e)
                        }
                    };
                } else {
                    let covering = if no_psv_rs { None } else { RootSignatureDesc::covering_dxil(&job.dxil) };
                    rs = match covering.map(|d| pipeline.root_signature(&d.to_blob())) {
                        Some(Ok(rs)) => Some(rs),
                        Some(Err(e)) => {
                            eprintln!("{} {}: covering root signature rejected: {e}", job.bundle, job.entry);
                            None
                        }
                        None => None,
                    };
                    let req = CompileRequest::new(&job.dxil, rs.as_deref(), &opts).entry_point(&job.entry);
                    r = pipeline.compile(&req);
                }
                let code = match &r {
                    Err(Error::Convert { code, .. }) => Some(*code),
                    _ => None,
                };
                outcomes.lock().unwrap().push(Outcome {
                    bundle: job.bundle.clone(),
                    stage_name: job.stage_name,
                    entry: job.entry.clone(),
                    profile,
                    features,
                    result: r.map(|c| (c.stage, c.metallib.len())).map_err(|e| e.to_string()),
                    code,
                    with_root_signature: rs.is_some(),
                });
                let d = done.fetch_add(1, Ordering::Relaxed) + 1;
                if d.is_multiple_of(250) {
                    eprintln!("  {d}/{} ({:.1}s)", jobs.len(), t0.elapsed().as_secs_f64());
                }
            });
        }
    });
    let mut outcomes = outcomes.into_inner().unwrap();
    outcomes.sort_by(|a, b| (&a.bundle, &a.entry).cmp(&(&b.bundle, &b.entry)));
    report(&outcomes, &parse_failures, &pipeline, t0.elapsed().as_secs_f64(), list_failures);
}

fn report(outcomes: &[Outcome], parse_failures: &[String], pipeline: &ShaderPipeline, secs: f64, list: usize) {
    let ok = outcomes.iter().filter(|o| o.result.is_ok()).count();
    let failed: Vec<&Outcome> = outcomes.iter().filter(|o| o.result.is_err()).collect();
    let metallib_bytes: usize = outcomes.iter().filter_map(|o| o.result.as_ref().ok()).map(|r| r.1).sum();
    let with_rs = outcomes.iter().filter(|o| o.with_root_signature).count();
    println!("\n== result: {ok}/{} converted ({with_rs} against a PSV0/RDAT-derived root signature, rest linear), {} failed, {:.1}s, {:.1} MB metallib", outcomes.len(), failed.len(), secs, metallib_bytes as f64 / 1e6);
    let st = pipeline.stats();
    println!(
        "== cache: memory_hits={} disk_hits={} compiles={} failures={} dedup_waits={} read={:.1}MB written={:.1}MB disk_errors={} compile_cpu={:.1}s",
        st.memory_hits,
        st.disk_hits,
        st.compiles,
        st.compile_failures,
        st.dedup_waits,
        st.disk_bytes_read as f64 / 1e6,
        st.disk_bytes_written as f64 / 1e6,
        st.disk_errors,
        st.compile_nanos as f64 / 1e9
    );
    if let Some(h) = pipeline.helper() {
        println!("== helper: {} worker processes alive", h.workers());
    }
    if !parse_failures.is_empty() {
        println!("== binrfx parse failures: {}", parse_failures.len());
        for p in parse_failures {
            println!("   {p}");
        }
    }

    // by profile
    let mut by_profile: BTreeMap<&str, (usize, usize)> = BTreeMap::new();
    for o in outcomes {
        let e = by_profile.entry(&o.profile).or_default();
        e.0 += 1;
        if o.result.is_err() {
            e.1 += 1;
        }
    }
    println!("\n== by profile (total / failed)");
    for (p, (t, f)) in &by_profile {
        println!("   {p:<12} {t:>6} {f:>6}");
    }
    let mut by_stage: BTreeMap<&str, (usize, usize)> = BTreeMap::new();
    for o in outcomes {
        let e = by_stage.entry(o.stage_name).or_default();
        e.0 += 1;
        if o.result.is_err() {
            e.1 += 1;
        }
    }
    println!("\n== by binrfx stage slot (total / failed)");
    for (p, (t, f)) in &by_stage {
        println!("   {p:<14} {t:>6} {f:>6}");
    }
    let mut by_metal: BTreeMap<String, usize> = BTreeMap::new();
    for o in outcomes {
        if let Ok((s, _)) = &o.result {
            *by_metal.entry(format!("{s:?}")).or_default() += 1;
        }
    }
    println!("\n== converted Metal stages: {by_metal:?}");

    // by error code
    let mut by_code: BTreeMap<String, usize> = BTreeMap::new();
    for o in &failed {
        *by_code.entry(o.code.map_or_else(|| "other".to_string(), |c| format!("{c:?}"))).or_default() += 1;
    }
    println!("\n== failures by error code");
    for (c, n) in &by_code {
        println!("   {c:<44} {n:>6}");
    }

    // features: share among all vs failed
    println!("\n== feature flags (programs using / failed among them)");
    for bit in 0..64u32 {
        let using = outcomes.iter().filter(|o| o.features >> bit & 1 == 1).count();
        if using == 0 {
            continue;
        }
        let f = failed.iter().filter(|o| o.features >> bit & 1 == 1).count();
        let name = FEATURE_NAMES.get(bit as usize).copied().unwrap_or("?");
        println!("   {name:<38} {using:>6} {f:>6}");
    }

    if !failed.is_empty() {
        println!("\n== first {} failures", list.min(failed.len()));
        for o in failed.iter().take(list) {
            println!(
                "   {} {} [{} {}] {:?}: {}",
                o.bundle,
                o.entry,
                o.stage_name,
                o.profile,
                dxbc::feature_names(o.features),
                o.result.as_ref().unwrap_err()
            );
        }
    }
}
