//! `AW2_PC_SAMPLE=<hz>` (diagnostic only, default off; `AW2_JIT_STATS` also turns it on at
//! 500 Hz): statistical host-PC sampler that gives *execution weight* per guest module and
//! guest block, plus the hottest host functions.
//!
//! A background thread periodically lists the task's threads (Mach), and for each thread
//! that is running suspends it, reads its PC (`ARM_THREAD_STATE64`) and resumes it at once
//! (nothing between suspend and resume takes a lock or allocates). Each PC is attributed to:
//! - a live AOT translation block (via the resolver `emu::aot` registers): guest module +
//!   block RVA;
//! - host code (`dladdr`): the enclosing symbol (Rust lifted shards, hand-written natives,
//!   win32-rt builtins, the emulator itself);
//! - otherwise "other", split by the VM region holding the PC (`other<TAB>kind` rows:
//!   per-thread JIT buffer stubs/code, other executable memory without symbols, unmapped).
//!
//! Unlike block-entry counters this needs no codegen change, so AOT caches stay valid and
//! the run is unperturbed apart from the sampling itself. Every `AW2_PC_SAMPLE_REPORT_S`
//! (default 30) seconds the cumulative report is written to
//! `$AW2_LOG_DIR/pc-sample-<pid>.txt` (else `~/Library/Logs/AW2Port/`): totals per module, then
//! `mod<TAB>module<TAB>rva<TAB>count` rows for the top blocks of every module and
//! `host<TAB>symbol<TAB>count` rows.
//!
//! A PC inside a host image that has no covering symbol (stripped system framework code)
//! counts as host by image (`host_image<TAB>file<TAB>count`); so does any PC in the dyld
//! shared cache range. "Other" PCs (no AOT block, no host image) are classified by their VM
//! region with submaps resolved (`other_region` rows carry the `VM_MEMORY_*` user tag, which
//! names the allocator) and listed individually
//! (`other_pc<TAB>kind<TAB>pc<TAB>where<TAB>count`, `where` = guest `module+0xrva` when the PC
//! lies in a mapped PE image). For `nonexec` PCs the return address at the sample is
//! recorded too (`nonexec_lr<TAB>target<TAB>caller<TAB>count`): a native caller that
//! branched to a raw guest code pointer would show up there. (A tail `br` leaves the
//! caller's caller in lr.)

#![cfg_attr(not(target_vendor = "apple"), allow(dead_code))]

use std::collections::HashMap;
use std::sync::OnceLock;
#[cfg(target_vendor = "apple")]
use std::time::{Duration, Instant};

/// Resolver for host PCs inside live AOT code: `(module, image base, guest block ip)`
/// (`ip` 0 = shared stubs / padding).
pub type Resolver = fn(usize) -> Option<(String, usize, u64)>;
/// Resolver for addresses inside mapped guest PE images: `(module, image base)`. Must not
/// block.
pub type ImageResolver = fn(usize) -> Option<(String, usize)>;

static RESOLVER: OnceLock<Resolver> = OnceLock::new();
static IMAGE_RESOLVER: OnceLock<ImageResolver> = OnceLock::new();
static STARTED: OnceLock<()> = OnceLock::new();

fn rate_hz() -> u64 {
    if let Some(v) = std::env::var("PORT_PC_SAMPLE")
        .or_else(|_| std::env::var("AW2_PC_SAMPLE"))
        .ok()
        .and_then(|s| s.trim().parse::<u64>().ok())
    {
        return v.min(10_000);
    }
    if std::env::var("AW2_JIT_STATS").ok().and_then(|s| s.trim().parse::<u64>().ok()).unwrap_or(0) != 0 {
        return 500;
    }
    0
}

/// Install the AOT resolver (first caller wins) and start the sampler if enabled.
pub fn start(resolver: Resolver, image: ImageResolver) {
    let _ = RESOLVER.set(resolver);
    let _ = IMAGE_RESOLVER.set(image);
    let hz = rate_hz();
    if hz == 0 || STARTED.set(()).is_err() {
        return;
    }
    let report = std::env::var("PORT_PC_SAMPLE_REPORT_S")
        .or_else(|_| std::env::var("AW2_PC_SAMPLE_REPORT_S"))
        .ok()
        .and_then(|s| s.trim().parse::<u64>().ok())
        .unwrap_or(30)
        .max(1);
    log::info!("pc sample: enabled at {hz} Hz, report every {report}s");
    let _ = std::thread::Builder::new().name("port-pc-sample".into()).spawn(move || unsafe { run(hz, report) });
}

#[cfg(target_vendor = "apple")]
mod mach {
    pub type Port = u32;
    pub const THREAD_BASIC_INFO: u32 = 3;
    pub const THREAD_BASIC_INFO_COUNT: u32 = 10;
    pub const TH_STATE_RUNNING: i32 = 1;
    #[cfg(target_arch = "aarch64")]
    pub const STATE_FLAVOR: i32 = 6; // ARM_THREAD_STATE64
    #[cfg(target_arch = "aarch64")]
    pub const STATE_COUNT: u32 = 68;
    /// Index of the PC in the state, in u64 words (x0..x28, fp, lr, sp, pc).
    #[cfg(target_arch = "aarch64")]
    pub const PC_WORD: usize = 32;
    /// Index of the link register (`None`: no link register).
    #[cfg(target_arch = "aarch64")]
    pub const LR_WORD: Option<usize> = Some(30);
    #[cfg(target_arch = "x86_64")]
    pub const LR_WORD: Option<usize> = None;
    #[cfg(target_arch = "x86_64")]
    pub const STATE_FLAVOR: i32 = 4; // x86_THREAD_STATE64
    #[cfg(target_arch = "x86_64")]
    pub const STATE_COUNT: u32 = 42;
    #[cfg(target_arch = "x86_64")]
    pub const PC_WORD: usize = 16; // rip

    #[repr(C)]
    pub struct DlInfo {
        pub fname: *const i8,
        pub fbase: *mut u8,
        pub sname: *const i8,
        pub saddr: *mut u8,
    }

    unsafe extern "C" {
        pub static mach_task_self_: Port;
        pub fn mach_thread_self() -> Port;
        pub fn task_threads(task: Port, list: *mut *mut Port, count: *mut u32) -> i32;
        pub fn thread_info(t: Port, flavor: u32, info: *mut i32, count: *mut u32) -> i32;
        pub fn thread_suspend(t: Port) -> i32;
        pub fn thread_resume(t: Port) -> i32;
        pub fn thread_get_state(t: Port, flavor: i32, state: *mut u32, count: *mut u32) -> i32;
        pub fn mach_port_deallocate(task: Port, name: Port) -> i32;
        pub fn vm_deallocate(task: Port, addr: usize, size: usize) -> i32;
        pub fn dladdr(addr: *const u8, info: *mut DlInfo) -> i32; // same signature as callprof.rs
        pub fn mach_vm_region_recurse(
            task: Port,
            addr: *mut u64,
            size: *mut u64,
            depth: *mut u32,
            info: *mut i32,
            count: *mut u32,
        ) -> i32;
        pub fn _dyld_get_shared_cache_range(length: *mut usize) -> *const u8;
    }
    /// `VM_REGION_SUBMAP_INFO_COUNT_64` (32-bit words; the struct is `#pragma pack(4)`):
    /// protection 0, max_protection 1, inheritance 2, offset 3..4, user_tag 5, ...,
    /// is_submap 12.
    pub const VM_REGION_SUBMAP_INFO_COUNT_64: u32 = 19;
    pub const SUBMAP_USER_TAG: usize = 5;
    pub const SUBMAP_IS_SUBMAP: usize = 12;
}

/// The dyld shared cache `[start, end)` (system frameworks), or `(0, 0)`.
#[cfg(target_vendor = "apple")]
fn shared_cache() -> (usize, usize) {
    static R: OnceLock<(usize, usize)> = OnceLock::new();
    *R.get_or_init(|| unsafe {
        let mut len = 0usize;
        let p = mach::_dyld_get_shared_cache_range(&mut len) as usize;
        if p == 0 { (0, 0) } else { (p, p + len) }
    })
}

#[derive(Default)]
struct Stats {
    samples: u64,
    sweeps: u64,
    /// module -> samples
    by_module: HashMap<String, u64>,
    /// (module, rva) -> samples
    by_block: HashMap<(String, u64), u64>,
    /// host symbol start -> samples
    by_host: HashMap<usize, u64>,
    /// host image base (no symbol for the PC; 0 = dyld shared cache, image unknown) -> samples
    by_host_image: HashMap<usize, u64>,
    other: u64,
    /// unresolved PCs by kind (see [`classify_other`])
    other_kind: HashMap<&'static str, u64>,
    /// unresolved PCs by VM region (start, size, protection, VM user tag)
    other_region: HashMap<(u64, u64, i32, i32), u64>,
    /// unresolved PCs by (kind, pc)
    other_pc: HashMap<(&'static str, usize), u64>,
    /// `nonexec` PCs by (pc, lr at the sample)
    nonexec_lr: HashMap<(usize, usize), u64>,
}

/// Per-thread JIT buffers (`emu::jit`): one `MAP_JIT` mapping each, shared stubs first.
const JIT_CODE_SIZE: u64 = 128 << 20;
const JIT_STUBS_BYTES: u64 = 4096;

/// Kind of a PC that is neither live AOT code nor covered by a host image, from the VM
/// region that contains it (submaps resolved): `jit-stubs` / `jit-code` (an executable
/// mapping of the per-thread JIT buffer size: the emulator's runtime translations),
/// `exec-nosym` (other executable memory without an image), `nonexec` (readable but not
/// executable: should not happen), `unmapped`. The region carries its VM user tag
/// (`VM_MEMORY_*`, names the allocator).
#[cfg(target_vendor = "apple")]
unsafe fn classify_other(pc: usize) -> (&'static str, (u64, u64, i32, i32)) {
    use mach::*;
    // `mach_vm_region_recurse` descends into submaps: the dyld shared cache is one nested
    // submap whose top-level entry reads as non-executable.
    let (mut a, mut sz) = (pc as u64, 0u64);
    let mut depth: u32 = 0;
    let mut info = [0i32; VM_REGION_SUBMAP_INFO_COUNT_64 as usize];
    loop {
        let mut cnt = VM_REGION_SUBMAP_INFO_COUNT_64;
        if mach_vm_region_recurse(mach_task_self_, &mut a, &mut sz, &mut depth, info.as_mut_ptr(), &mut cnt) != 0
            || a > pc as u64
        {
            return ("unmapped", (0, 0, 0, 0));
        }
        if info[SUBMAP_IS_SUBMAP] != 0 && depth < 8 {
            depth += 1;
            continue;
        }
        break;
    }
    let (prot, tag) = (info[0], info[SUBMAP_USER_TAG]);
    let kind = if prot & 4 == 0 {
        "nonexec"
    } else if sz == JIT_CODE_SIZE {
        if pc as u64 - a < JIT_STUBS_BYTES { "jit-stubs" } else { "jit-code" }
    } else {
        "exec-nosym"
    };
    (kind, (a, sz, prot, tag))
}

#[cfg(not(target_vendor = "apple"))]
unsafe fn run(_hz: u64, _report: u64) {}

#[cfg(target_vendor = "apple")]
unsafe fn run(hz: u64, report: u64) {
    use mach::*;
    let period = Duration::from_nanos(1_000_000_000 / hz.max(1));
    let task = mach_task_self_;
    let me = mach_thread_self();
    let mut st = Stats::default();
    let mut pcs: Vec<(usize, usize)> = Vec::with_capacity(512);
    let mut last = Instant::now();
    let start = last;
    loop {
        std::thread::sleep(period);
        pcs.clear();
        let mut list: *mut Port = core::ptr::null_mut();
        let mut n: u32 = 0;
        if task_threads(task, &mut list, &mut n) != 0 || list.is_null() {
            continue;
        }
        for i in 0..n as usize {
            let t = *list.add(i);
            if t != me {
                let mut info = [0i32; 10];
                let mut cnt = THREAD_BASIC_INFO_COUNT;
                if thread_info(t, THREAD_BASIC_INFO, info.as_mut_ptr(), &mut cnt) == 0 && info[6] == TH_STATE_RUNNING {
                    let mut state = [0u64; 40];
                    let mut sc = STATE_COUNT;
                    if thread_suspend(t) == 0 {
                        let ok = thread_get_state(t, STATE_FLAVOR, state.as_mut_ptr() as *mut u32, &mut sc) == 0;
                        thread_resume(t);
                        if ok && pcs.len() < pcs.capacity() {
                            pcs.push((state[PC_WORD] as usize, LR_WORD.map_or(0, |w| state[w] as usize)));
                        }
                    }
                }
            }
            mach_port_deallocate(task, t);
        }
        vm_deallocate(task, list as usize, n as usize * core::mem::size_of::<Port>());
        st.sweeps += 1;
        let resolve = RESOLVER.get().copied();
        for &(pc, lr) in &pcs {
            let pc = pc & ((1usize << 48) - 1);
            st.samples += 1;
            if let Some((m, base, ip)) = resolve.and_then(|r| r(pc)) {
                let rva = if ip == 0 { u64::MAX } else { ip.wrapping_sub(base as u64) };
                *st.by_block.entry((m.clone(), rva)).or_insert(0) += 1;
                *st.by_module.entry(m).or_insert(0) += 1;
                continue;
            }
            let mut di = DlInfo { fname: core::ptr::null(), fbase: core::ptr::null_mut(), sname: core::ptr::null(), saddr: core::ptr::null_mut() };
            let found = dladdr(pc as *const _, &mut di) != 0;
            let (sc0, sc1) = shared_cache();
            if found && !di.saddr.is_null() {
                *st.by_host.entry(di.saddr as usize).or_insert(0) += 1;
            } else if found && !di.fbase.is_null() {
                *st.by_host_image.entry(di.fbase as usize).or_insert(0) += 1;
            } else if pc >= sc0 && pc < sc1 {
                *st.by_host_image.entry(0).or_insert(0) += 1;
            } else {
                st.other += 1;
                let (kind, region) = classify_other(pc);
                *st.other_kind.entry(kind).or_insert(0) += 1;
                *st.other_region.entry(region).or_insert(0) += 1;
                *st.other_pc.entry((kind, pc)).or_insert(0) += 1;
                if kind == "nonexec" {
                    // lr may carry a PAC signature; strip to the 48-bit VA.
                    *st.nonexec_lr.entry((pc, lr & ((1usize << 48) - 1))).or_insert(0) += 1;
                }
            }
        }
        if last.elapsed().as_secs() >= report {
            last = Instant::now();
            write_report(&st, start.elapsed().as_secs_f64(), hz);
        }
    }
}

/// Crude v0 demangling: the path's `<len><ident>` segments joined with `::` (crate hashes
/// `Cs<base62>_` and `<digits>_` disambiguators skipped; generic args come out flattened).
fn demangle_v0(r: &str) -> String {
    let b = r.as_bytes();
    let (mut i, mut parts) = (0usize, Vec::new());
    while i < b.len() {
        if b[i] == b'C' && b.get(i + 1) == Some(&b's') {
            i += 2;
            while i < b.len() && b[i] != b'_' {
                i += 1;
            }
            i += 1;
            continue;
        }
        if b[i].is_ascii_digit() && (i == 0 || !b[i - 1].is_ascii_digit()) {
            let j = (i..b.len()).find(|&k| !b[k].is_ascii_digit()).unwrap_or(b.len());
            let len: usize = r[i..j].parse().unwrap_or(0);
            if j < b.len() && b[j] != b'_' && len > 0 && j + len <= b.len() {
                parts.push(&r[j..j + len]);
                i = j + len;
                continue;
            }
            i = j;
            continue;
        }
        i += 1;
    }
    if parts.is_empty() { r.to_string() } else { parts.join("::") }
}

/// Crude Rust (legacy + v0) / Itanium demangling for the report (hashes dropped).
pub(crate) fn demangle(s: &str) -> String {
    let t = s.strip_prefix('_').unwrap_or(s);
    if let Some(r) = t.strip_prefix("_R").or_else(|| t.strip_prefix('R').filter(|r| r.starts_with('N'))) {
        return demangle_v0(r);
    }
    let Some(mut r) = t.strip_prefix("_ZN").or_else(|| t.strip_prefix("ZN")) else { return s.to_string() };
    let mut parts: Vec<String> = Vec::new();
    while let Some(c) = r.chars().next() {
        if !c.is_ascii_digit() {
            break;
        }
        let nd = r.chars().take_while(|c| c.is_ascii_digit()).count();
        let Ok(len) = r[..nd].parse::<usize>() else { break };
        if nd + len > r.len() {
            break;
        }
        let id = &r[nd..nd + len];
        r = &r[nd + len..];
        if !(id.len() == 17 && id.starts_with('h') && id[1..].chars().all(|c| c.is_ascii_hexdigit())) {
            parts.push(id.replace("$LT$", "<").replace("$GT$", ">").replace("$u20$", " ").replace("..", "::").replace("$C$", ","));
        }
    }
    if parts.is_empty() { s.to_string() } else { parts.join("::") }
}

/// Report name of address `a`: live AOT block (`aot:<module>+0x<rva>`), mapped guest image
/// (`<module>+0x<rva>`), host symbol (`<symbol>+0x<off>`), else `?`.
#[cfg(target_vendor = "apple")]
fn name_addr(a: usize) -> String {
    if a == 0 {
        return "-".to_string();
    }
    if let Some((m, base, ip)) = RESOLVER.get().and_then(|r| r(a)) {
        return if ip == 0 { format!("aot:{m}:stubs") } else { format!("aot:{m}+0x{:x}", ip.wrapping_sub(base as u64)) };
    }
    if let Some((m, base)) = IMAGE_RESOLVER.get().and_then(|r| r(a)) {
        return format!("{m}+0x{:x}", a.wrapping_sub(base));
    }
    let mut di = mach::DlInfo { fname: core::ptr::null(), fbase: core::ptr::null_mut(), sname: core::ptr::null(), saddr: core::ptr::null_mut() };
    unsafe {
        if mach::dladdr(a as *const _, &mut di) != 0 {
            if !di.sname.is_null() {
                let s = demangle(&std::ffi::CStr::from_ptr(di.sname).to_string_lossy());
                return format!("{s}+0x{:x}", a.wrapping_sub(di.saddr as usize));
            }
            if !di.fbase.is_null() {
                return format!("{}+0x{:x}", image_name(di.fbase as usize), a.wrapping_sub(di.fbase as usize));
            }
        }
    }
    let (sc0, sc1) = shared_cache();
    if a >= sc0 && a < sc1 {
        return format!("[dyld shared cache]+0x{:x}", a - sc0);
    }
    "?".to_string()
}

/// File name of the host image at `fbase` (0: the dyld shared cache, image unknown).
#[cfg(target_vendor = "apple")]
fn image_name(fbase: usize) -> String {
    if fbase == 0 {
        return "[dyld shared cache]".to_string();
    }
    let mut di = mach::DlInfo { fname: core::ptr::null(), fbase: core::ptr::null_mut(), sname: core::ptr::null(), saddr: core::ptr::null_mut() };
    unsafe {
        if mach::dladdr(fbase as *const _, &mut di) != 0 && !di.fname.is_null() {
            let f = std::ffi::CStr::from_ptr(di.fname).to_string_lossy();
            return f.rsplit('/').next().unwrap_or(&f).to_string();
        }
    }
    format!("0x{fbase:x}")
}

#[cfg(target_vendor = "apple")]
fn write_report(st: &Stats, secs: f64, hz: u64) {
    use std::fmt::Write;
    let mut o = String::new();
    let pct = |a: u64| if st.samples == 0 { 0.0 } else { a as f64 * 100.0 / st.samples as f64 };
    let aot: u64 = st.by_module.values().sum();
    let host: u64 = st.by_host.values().sum::<u64>() + st.by_host_image.values().sum::<u64>();
    let _ = writeln!(o, "# pc sample: {secs:.0}s at {hz} Hz, {} sweeps, {} running-thread samples", st.sweeps, st.samples);
    let _ = writeln!(o, "# aot {aot} ({:.2}%), host {host} ({:.2}%), other/jit {} ({:.2}%)", pct(aot), pct(host), st.other, pct(st.other));
    let mut mods: Vec<_> = st.by_module.iter().collect();
    mods.sort_by(|a, b| b.1.cmp(a.1));
    for (m, c) in &mods {
        let _ = writeln!(o, "total\t{m}\t{c}\t{:.3}%", pct(**c));
    }
    let mut kinds: Vec<_> = st.other_kind.iter().collect();
    kinds.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    for (k, c) in kinds {
        let _ = writeln!(o, "other\t{k}\t{c}\t{:.3}%", pct(*c));
    }
    let mut regions: Vec<_> = st.other_region.iter().collect();
    regions.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    for ((a, sz, prot, tag), c) in regions.into_iter().take(16) {
        let _ = writeln!(o, "other_region\t0x{a:x}\t0x{sz:x}\tprot={prot}\ttag={tag}\t{c}");
    }
    let mut imgs: Vec<_> = st.by_host_image.iter().collect();
    imgs.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    for (fbase, c) in imgs.into_iter().take(40) {
        let _ = writeln!(o, "host_image\t{}\t{c}\t{:.3}%", image_name(*fbase), pct(*c));
    }
    let mut opcs: Vec<_> = st.other_pc.iter().collect();
    opcs.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    for ((kind, pc), c) in opcs.into_iter().take(60) {
        let _ = writeln!(o, "other_pc\t{kind}\t0x{pc:x}\t{}\t{c}", name_addr(*pc));
    }
    let mut lrs: Vec<_> = st.nonexec_lr.iter().collect();
    lrs.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    for ((pc, lr), c) in lrs.into_iter().take(80) {
        let _ = writeln!(o, "nonexec_lr\t{}\t{}\t0x{lr:x}\t{c}", name_addr(*pc), name_addr(*lr));
    }
    let mut blocks: Vec<_> = st.by_block.iter().collect();
    blocks.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    let mut per: HashMap<&str, usize> = HashMap::new();
    for ((m, rva), c) in blocks {
        let k = per.entry(m.as_str()).or_insert(0);
        *k += 1;
        if *k > 400 {
            continue;
        }
        if *rva == u64::MAX {
            let _ = writeln!(o, "mod\t{m}\tstubs\t{c}");
        } else {
            let _ = writeln!(o, "mod\t{m}\t0x{rva:x}\t{c}");
        }
    }
    let mut hosts: Vec<_> = st.by_host.iter().collect();
    hosts.sort_by(|a, b| b.1.cmp(a.1));
    for (addr, c) in hosts.into_iter().take(300) {
        let mut di = mach::DlInfo { fname: core::ptr::null(), fbase: core::ptr::null_mut(), sname: core::ptr::null(), saddr: core::ptr::null_mut() };
        let name = unsafe {
            if mach::dladdr(*addr as *const _, &mut di) != 0 && !di.sname.is_null() {
                demangle(&std::ffi::CStr::from_ptr(di.sname).to_string_lossy())
            } else {
                format!("0x{addr:x}")
            }
        };
        let _ = writeln!(o, "host\t{name}\t{c}\t{:.3}%", pct(*c));
    }
    let dir = std::env::var("PORT_LOG_DIR")
        .or_else(|_| std::env::var("AW2_LOG_DIR"))
        .ok()
        .filter(|d| !d.is_empty())
        .unwrap_or_else(|| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            let sub = win32_rt::config::app_config().log_dir_name;
            format!("{home}/Library/Logs/{sub}")
        });
    let _ = std::fs::create_dir_all(&dir);
    let path = std::path::Path::new(&dir).join(format!("pc-sample-{}.txt", std::process::id()));
    let tmp = path.with_extension("tmp");
    if std::fs::write(&tmp, o.as_bytes()).is_ok() {
        let _ = std::fs::rename(&tmp, &path);
    }
    log::info!("pc sample: {} samples ({:.1}% aot, {:.1}% host) -> {}", st.samples, pct(aot), pct(host), path.display());
}

#[cfg(test)]
mod tests {
    #[test]
    fn demangle_rust_symbols() {
        assert_eq!(super::demangle("__ZN8aw2_game3emu8call_va817h0123456789abcdefE"), "aw2_game::emu::call_va8");
        assert_eq!(super::demangle("_memcpy"), "_memcpy");
        assert_eq!(
            super::demangle("_RNvMs2_NtNtNtNtCsjWSbCF3BefW_8aw2_game2ns1r3mem9difftestsNtB5_10AllocArena5setup"),
            "aw2_game::ns::r::mem::difftests::AllocArena::setup"
        );
    }
}
