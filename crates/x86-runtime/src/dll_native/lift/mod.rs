//! Whole-DLL lifts: the functions of a game-shipped DLL translated to Rust by
//! `tools/decomp_scale/gen_subsystems.py --image <dll> --base <pinned>` (one generated
//! module per DLL, see `dll_lift.py`).
//!
//! Lifted code embeds absolute VAs (data, call targets, jump tables) of the image relocated
//! to its pinned base ([`super::pinned_base`]), so a module is installed only when the DLL
//! is mapped exactly there and its `.text` is byte-identical to the generator's input
//! (FNV-1a over the relocated, mapped `.text`). Anything else (another build, a different
//! base) keeps running the original guest code. `AW2_DLL_LIFT=0` disables all lifts;
//! `AW2_DLL_LIFT=-name,-name2` disables single modules (loader names, e.g.
//! `-vcruntime140_1.dll`). `AW2_DLL_LIFT_SEL` selects single entries for A/B bisection
//! (diagnostic only, see [`selection`]).
//!
//! Each entry is registered in the native VA table like an exe replacement; VAs that
//! already dispatch natively (hand-written [`super`] natives, import-forwarder aliases) keep
//! their target. Functions that did not lift (or have SEH / unsupported handlers) stay guest
//! code; functions with a `__CxxFrameHandler4` FuncInfo lift on the normal path ([`eh`]).
//! `AW2_DLL_LIFT_COV=1` counts the calls of every installed entry ([`cov`]). `AW2_SHADOW`
//! entries selected by `va=` / `class=dll` are probed on live game state ([`shadow_hook`]).

pub mod cov;
pub mod eh;
pub mod guarded;
pub mod shadow_hook;

use std::collections::HashSet;
use std::sync::Mutex;

/// A lifted function: guest `va` (at the pinned base) and its `extern "win64"` Rust body.
pub struct Lifted {
    pub va: u64,
    pub ptr: *const (),
    /// [`LEAF`] | [`XMM_RET`] | [`DF_ALIAS`].
    pub flags: u8,
    /// When [`DF_ALIAS`] is set, RVA of the in-module target entry whose Rust body `ptr` is reused.
    pub alias_rva: u32,
}

/// The body makes no calls (no `call_va8*`): self-contained, used by the fuzz difftest.
pub const LEAF: u8 = 1;
/// The body sets an XMM return value (`set_xmm_ret`).
pub const XMM_RET: u8 = 2;
/// Pure in-module direct `E9 rel32` jump thunk reusing the lifted body at `base + alias_rva`.
pub const DF_ALIAS: u8 = 4;

// SAFETY: `ptr` is a pointer to an immutable function.
unsafe impl Sync for Lifted {}

impl Lifted {
    pub const fn new(va: u64, ptr: *const (), flags: u8) -> Self {
        Self { va, ptr, flags, alias_rva: 0 }
    }

    pub const fn alias(va: u64, ptr: *const (), flags: u8, alias_rva: u32) -> Self {
        Self { va, ptr, flags: flags | DF_ALIAS, alias_rva }
    }
}

/// A register-file lifted function (`DF_REGFILE`, e.g. V8 CSA/Torque/Ignition/JSEntry/CEntry
/// builtins): guest `[va, va + len)` at the pinned base, its [`crate::emu::regfile::RegfileFn`]
/// body (`blk = 0` at `va`), and `(rva_off_from_va, blk_idx)` call-return / unwind-handler
/// continuations inside `[va, va + len)`.
#[derive(Clone, Copy)]
pub struct LiftedRegfile {
    pub va: u64,
    pub len: u32,
    pub ptr: crate::emu::regfile::RegfileFn,
    pub conts: &'static [(u16, u16)],
}

impl LiftedRegfile {
    pub const fn new(va: u64, ptr: crate::emu::regfile::RegfileFn, len: u32) -> Self {
        Self { va, len, ptr, conts: &[] }
    }

    pub const fn with_conts(
        va: u64,
        ptr: crate::emu::regfile::RegfileFn,
        len: u32,
        conts: &'static [(u16, u16)],
    ) -> Self {
        Self { va, len, ptr, conts }
    }
}

/// One lifted DLL (generated `IMAGE`).
pub struct LiftedImage {
    /// Loader module name (lower case).
    pub name: &'static str,
    /// Base the image was relocated to for lifting (must equal the mapped base).
    pub base: usize,
    pub text_rva: usize,
    pub text_len: usize,
    /// FNV-1a 64 of the mapped `.text` (`text_len` bytes, relocated to `base`).
    pub text_fnv: u64,
    pub slices: &'static [&'static [Lifted]],
    pub regfile_slices: &'static [&'static [LiftedRegfile]],
    /// C++ EH records of the lifted functions with a `__CxxFrameHandler4` FuncInfo ([`eh`]).
    pub eh_meta: &'static [eh::EhMeta],
}

impl LiftedImage {
    pub fn regfile_entries(&self) -> Vec<LiftedRegfile> {
        self.regfile_slices.iter().copied().flatten().copied().collect()
    }
}

static IMAGES: std::sync::RwLock<Vec<&'static LiftedImage>> = std::sync::RwLock::new(Vec::new());

/// Register statically lifted DLL images for a game target (none by default).
pub fn register_lifted_images(images: &[&'static LiftedImage]) {
    let mut v = IMAGES.write().unwrap();
    for &img in images {
        if !v.iter().any(|existing| existing.name == img.name) {
            v.push(img);
        }
    }
}

/// Snapshot of all registered statically lifted DLL images.
pub fn registered_images() -> Vec<&'static LiftedImage> {
    IMAGES.read().map_or_else(|_| Vec::new(), |v| v.clone())
}

/// The lifted image for module `name` (lower-case loader name), if any.
pub fn image(name: &str) -> Option<&'static LiftedImage> {
    IMAGES
        .read()
        .ok()?
        .iter()
        .copied()
        .find(|i| i.name == name)
}

fn enabled(name: &str) -> bool {
    match std::env::var("AW2_DLL_LIFT") {
        Ok(v) => {
            let v = v.trim();
            if v == "0" {
                return false;
            }
            !v.split(',').any(|t| t.trim().strip_prefix('-').is_some_and(|n| n.eq_ignore_ascii_case(name)))
        }
        Err(_) => true,
    }
}

/// One `AW2_DLL_LIFT_SEL` item: which entries it matches (by RVA and entry index).
enum SelItem {
    Range(u64, u64),
    Set(HashSet<u64>),
    Bucket(usize, usize),
}

impl SelItem {
    fn matches(&self, rva: u64, idx: usize) -> bool {
        match self {
            SelItem::Range(lo, hi) => (*lo..*hi).contains(&rva),
            SelItem::Set(s) => s.contains(&rva),
            SelItem::Bucket(k, n) => idx % n == *k,
        }
    }
}

fn parse_hex(s: &str) -> Option<u64> {
    let s = s.trim();
    u64::from_str_radix(s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")).unwrap_or(s), 16).ok()
}

/// `AW2_DLL_LIFT_SEL='<module>=<items>[;<module>=<items>]'`: diagnostic per-entry selection
/// for A/B bisection of a lifted DLL (env-only, nothing persisted). Items are comma-separated,
/// each `+` (include) or `-` (exclude, the default): `0xLO-0xHI` (RVA range, half-open),
/// `0xRVA` (one entry), `@file` (hex RVAs, one per line, `#` comments) or `%k/n` (entries
/// whose index in VA order is `k` mod `n`). A leading include starts from no entries, else
/// from all; items apply left to right. Unselected entries are not registered (guest code).
/// Returns `None` when the variable has no clause for `name`.
fn selection(name: &str) -> Option<Vec<(bool, SelItem)>> {
    let v = std::env::var("AW2_DLL_LIFT_SEL").ok()?;
    let clause = v.split(';').find_map(|c| {
        let (m, items) = c.split_once('=')?;
        m.trim().eq_ignore_ascii_case(name).then(|| items.to_string())
    })?;
    let mut out = Vec::new();
    for raw in clause.split(',').map(str::trim).filter(|t| !t.is_empty()) {
        let (incl, t) = match raw.as_bytes()[0] {
            b'+' => (true, &raw[1..]),
            b'-' => (false, &raw[1..]),
            _ => (false, raw),
        };
        let item = if let Some(path) = t.strip_prefix('@') {
            match std::fs::read_to_string(path) {
                Ok(text) => SelItem::Set(
                    text.lines().filter_map(|l| parse_hex(l.split('#').next().unwrap_or(""))).collect(),
                ),
                Err(e) => {
                    log::error!("dll_lift: AW2_DLL_LIFT_SEL: cannot read {path}: {e}; ignoring the item");
                    continue;
                }
            }
        } else if let Some(b) = t.strip_prefix('%') {
            match b.split_once('/').and_then(|(k, n)| Some((k.trim().parse().ok()?, n.trim().parse().ok()?))) {
                Some((k, n)) if n > 0 && k < n => SelItem::Bucket(k, n),
                _ => {
                    log::error!("dll_lift: AW2_DLL_LIFT_SEL: bad bucket {raw:?}; ignoring the item");
                    continue;
                }
            }
        } else if let Some((lo, hi)) = t.split_once('-') {
            match (parse_hex(lo), parse_hex(hi)) {
                (Some(lo), Some(hi)) => SelItem::Range(lo, hi),
                _ => {
                    log::error!("dll_lift: AW2_DLL_LIFT_SEL: bad range {raw:?}; ignoring the item");
                    continue;
                }
            }
        } else if let Some(r) = parse_hex(t) {
            SelItem::Range(r, r + 1)
        } else {
            log::error!("dll_lift: AW2_DLL_LIFT_SEL: bad item {raw:?}; ignoring it");
            continue;
        };
        out.push((incl, item));
    }
    Some(out)
}

/// Whether entry `idx` (VA order) at `rva` survives the selection `sel`.
fn selected(sel: &[(bool, SelItem)], rva: u64, idx: usize) -> bool {
    let mut keep = !sel.first().is_some_and(|(incl, _)| *incl);
    for (incl, item) in sel {
        if item.matches(rva, idx) {
            keep = *incl;
        }
    }
    keep
}

/// Why `img` cannot be installed for the module mapped at `[base, base + size)`, or `None`.
unsafe fn gate(img: &LiftedImage, base: usize, size: usize) -> Option<String> {
    if base != img.base {
        return Some(format!("mapped at 0x{base:x}, lifted for 0x{:x}", img.base));
    }
    if img.text_rva.checked_add(img.text_len).map_or(true, |end| end > size) {
        return Some(format!("image size 0x{size:x} too small for .text 0x{:x}+0x{:x}", img.text_rva, img.text_len));
    }
    let text = core::slice::from_raw_parts((base + img.text_rva) as *const u8, img.text_len);
    let h = super::fnv1a(text);
    (h != img.text_fnv).then(|| format!(".text FNV 0x{h:016x} != lifted 0x{:016x} (different build)", img.text_fnv))
}

/// Installed module names, and entry VAs that already dispatched natively at install time.
static STATE: Mutex<(Vec<&'static str>, Vec<u64>)> = Mutex::new((Vec::new(), Vec::new()));

/// The lift of `name` passed its gate and was installed.
pub fn installed(name: &str) -> bool {
    STATE.lock().is_ok_and(|s| s.0.iter().any(|n| *n == name))
}

/// Entry `va` kept the native it already had when its image was installed.
pub fn kept_native(va: u64) -> bool {
    STATE.lock().is_ok_and(|s| s.1.contains(&va))
}

/// Entry VAs registered by [`install`] (all images).
static LIFTED_VAS: std::sync::RwLock<Option<HashSet<u64>>> = std::sync::RwLock::new(None);

/// `va` is an installed lifted DLL entry. Its body publishes `xmm0` only through
/// `simd_ops::set_xmm_ret` (every return of an [`XMM_RET`] body); any other return leaves the
/// guest's `xmm0` as it was, like the x86 it replaces (a function that never writes `xmm0`,
/// e.g. `__chkstk` / `__security_check_cookie`). The emulator's host bridge therefore treats
/// these like the exe's integer replacements (`emu::dispatch_flags`, `DF_GAME_REPL`) instead
/// of publishing the host's leftover `q0`.
pub fn is_lifted_va(va: u64) -> bool {
    LIFTED_VAS.read().is_ok_and(|s| s.as_ref().is_some_and(|s| s.contains(&va)))
}

/// Lock-free open-addressed dispatch table for `DF_REGFILE` entries and continuations
/// (`(va, ptr, blk)` triples; `CAP = 32768` gives ~31% load for V8's 1,616 builtins + ~8,500
/// continuations). Guarded by `(REGFILE_LO, REGFILE_HI)` so VAs outside the embedded builtin
/// range reject in 2 atomic loads + 1 subtraction.
const REGFILE_CAP: usize = 32768;
static REGFILE_TAB: [std::sync::atomic::AtomicU64; 2 * REGFILE_CAP] =
    [const { std::sync::atomic::AtomicU64::new(0) }; 2 * REGFILE_CAP];
static REGFILE_BLK: [std::sync::atomic::AtomicU32; REGFILE_CAP] =
    [const { std::sync::atomic::AtomicU32::new(0) }; REGFILE_CAP];
static REGFILE_LO: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(u64::MAX);
static REGFILE_HI: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static REGFILE_WRITE_LOCK: Mutex<()> = Mutex::new(());

/// `AW2_V8_REGFILE=0` disables `DF_REGFILE` builtin installation and routing (A/B knob).
#[inline]
pub fn regfile_enabled() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| {
        let on = !std::env::var("AW2_V8_REGFILE").is_ok_and(|v| v.trim() == "0");
        if !on {
            log::info!("dll_lift: DF_REGFILE builtins disabled (AW2_V8_REGFILE=0)");
        }
        on
    })
}

#[inline(always)]
fn regfile_slot(va: u64) -> usize {
    ((va.wrapping_mul(0x9E37_79B9_7F4A_7C15)) >> 49) as usize & (REGFILE_CAP - 1)
}

/// Register `(va, ptr, blk)` triples in [`REGFILE_TAB`] / [`REGFILE_BLK`] and bump `va_gen`.
pub fn register_regfile_vas(entries: &[(u64, crate::emu::regfile::RegfileFn, u32)]) {
    use std::sync::atomic::Ordering;
    if entries.is_empty() {
        return;
    }
    let _g = REGFILE_WRITE_LOCK.lock();
    for &(va, ptr, blk) in entries {
        if va == 0 {
            continue;
        }
        let mut idx = regfile_slot(va);
        for _ in 0..REGFILE_CAP {
            let k = REGFILE_TAB[2 * idx].load(Ordering::Relaxed);
            if k == va {
                REGFILE_BLK[idx].store(blk, Ordering::Relaxed);
                REGFILE_TAB[2 * idx + 1].store(ptr as usize as u64, Ordering::Release);
                break;
            }
            if k == 0 {
                REGFILE_BLK[idx].store(blk, Ordering::Relaxed);
                REGFILE_TAB[2 * idx + 1].store(ptr as usize as u64, Ordering::Relaxed);
                REGFILE_TAB[2 * idx].store(va, Ordering::Release);
                break;
            }
            idx = (idx + 1) & (REGFILE_CAP - 1);
        }
        REGFILE_LO.fetch_min(va, Ordering::AcqRel);
        REGFILE_HI.fetch_max(va.saturating_add(1), Ordering::AcqRel);
    }
    crate::exe::invalidate_dispatch();
}

#[cfg(test)]
pub(crate) fn clear_regfile_va_for_test(va: u64) {
    use std::sync::atomic::Ordering;
    let _g = REGFILE_WRITE_LOCK.lock();
    let mut idx = regfile_slot(va);
    for _ in 0..REGFILE_CAP {
        let k = REGFILE_TAB[2 * idx].load(Ordering::Relaxed);
        if k == va {
            REGFILE_TAB[2 * idx + 1].store(0, Ordering::Release);
            break;
        }
        if k == 0 {
            break;
        }
        idx = (idx + 1) & (REGFILE_CAP - 1);
    }
    crate::exe::invalidate_dispatch();
}

/// Lock-free lookup of a registered `DF_REGFILE` `(body, blk)` at `va`.
#[inline]
pub fn lookup_regfile(va: u64) -> Option<(crate::emu::regfile::RegfileFn, u32)> {
    use std::sync::atomic::Ordering;
    let hi = REGFILE_HI.load(Ordering::Acquire);
    if hi == 0 {
        return None;
    }
    let lo = REGFILE_LO.load(Ordering::Acquire);
    if hi <= lo || va.wrapping_sub(lo) >= hi - lo {
        return None;
    }
    let mut idx = regfile_slot(va);
    for _ in 0..REGFILE_CAP {
        let k = REGFILE_TAB[2 * idx].load(Ordering::Acquire);
        if k == va {
            let p = REGFILE_TAB[2 * idx + 1].load(Ordering::Acquire) as usize;
            let blk = REGFILE_BLK[idx].load(Ordering::Relaxed);
            return if p != 0 {
                Some((unsafe { core::mem::transmute::<usize, crate::emu::regfile::RegfileFn>(p) }, blk))
            } else {
                None
            };
        }
        if k == 0 {
            return None;
        }
        idx = (idx + 1) & (REGFILE_CAP - 1);
    }
    None
}

/// Register the lifted functions of DLL `name` mapped at `[base, base + size)` (relocated,
/// imports bound). Returns the number of VAs registered.
pub unsafe fn install(name: &str, base: usize, size: usize) -> usize {
    let Some(img) = image(name) else { return 0 };
    if !enabled(name) {
        log::info!("dll_lift: {name}: disabled by AW2_DLL_LIFT");
        return 0;
    }
    if let Some(why) = gate(img, base, size) {
        log::warn!("dll_lift: {name}: {why}; keeping guest code");
        return 0;
    }
    let (mut regs, mut kept) = (Vec::new(), HashSet::new());
    let cov = cov::on();
    let shadow = shadow_hook::on();
    let sel = selection(name);
    let mut entries: Vec<&Lifted> = img.slices.iter().copied().flatten().collect();
    if sel.is_some() {
        entries.sort_by_key(|e| e.va);
    }
    let (mut sel_fnv, mut sel_off) = (0xcbf2_9ce4_8422_2325u64, 0usize);
    for (idx, e) in entries.iter().copied().enumerate() {
        let rva = e.va - base as u64;
        if let Some(sel) = &sel {
            if !selected(sel, rva, idx) {
                sel_off += 1;
                continue;
            }
            sel_fnv = rva.to_le_bytes().iter().fold(sel_fnv, |h, &b| (h ^ b as u64).wrapping_mul(0x100_0000_01b3));
        }
        if crate::exe::lookup_va_only(e.va).is_some()
            || super::is_dll_chkstk(e.va)
            || super::is_dll_stack_pos(e.va)
        {
            kept.insert(e.va);
            continue;
        }
        let mut p = if cov { cov::stub(img.name, base, e.va, e.ptr as usize) } else { e.ptr as usize };
        if shadow && shadow_hook::wanted(e.va) {
            p = shadow_hook::stub(img.name, base, e.va, p, e.ptr as usize, e.flags & XMM_RET != 0);
        }
        regs.push((e.va, p));
    }
    let mut rf_regs = Vec::new();
    let mut rf_n = 0usize;
    let mut rf_cont_n = 0usize;
    let mut rf_off = 0usize;
    if !img.regfile_slices.is_empty() && regfile_enabled() {
        let mut rf_entries: Vec<&LiftedRegfile> = img.regfile_slices.iter().copied().flatten().collect();
        if sel.is_some() {
            rf_entries.sort_by_key(|e| e.va);
        }
        let base_idx = entries.len();
        for (idx, e) in rf_entries.iter().copied().enumerate() {
            let rva = e.va - base as u64;
            if let Some(sel) = &sel {
                if !selected(sel, rva, base_idx + idx) {
                    rf_off += 1;
                    continue;
                }
                sel_fnv = rva
                    .to_le_bytes()
                    .iter()
                    .fold(sel_fnv, |h, &b| (h ^ b as u64).wrapping_mul(0x100_0000_01b3));
            }
            if crate::exe::lookup_va_only(e.va).is_some()
                || super::is_dll_chkstk(e.va)
                || super::is_dll_stack_pos(e.va)
            {
                kept.insert(e.va);
                continue;
            }
            let ptr = if cov {
                let s = cov::stub(img.name, base, e.va, e.ptr as usize);
                core::mem::transmute::<usize, crate::emu::regfile::RegfileFn>(s)
            } else {
                e.ptr
            };
            rf_regs.push((e.va, ptr, 0u32));
            rf_n += 1;
            for &(coff, cblk) in e.conts {
                rf_regs.push((e.va + coff as u64, e.ptr, cblk as u32));
                rf_cont_n += 1;
            }
        }
    }
    if sel.is_some() {
        let total = entries.len() + img.regfile_slices.iter().map(|s| s.len()).sum::<usize>();
        let dropped = sel_off + rf_off;
        log::warn!(
            "dll_lift: {name}: AW2_DLL_LIFT_SEL keeps {}/{} entries ({dropped} left as guest code; set fnv 0x{sel_fnv:016x}) -- diagnostic A/B selection",
            total.saturating_sub(dropped),
            total
        );
    }
    let n = regs.len();
    if let Ok(mut s) = LIFTED_VAS.write() {
        s.get_or_insert_with(HashSet::new).extend(regs.iter().map(|&(va, _)| va));
    }
    crate::exe::register_native_vas(regs);
    register_regfile_vas(&rf_regs);
    let nkept = kept.len();
    if let Ok(mut s) = STATE.lock() {
        s.0.push(img.name);
        s.1.extend(kept);
    }
    if rf_n > 0 {
        log::info!(
            "dll_lift: {name}: {n} lifted function(s) + {rf_n} regfile builtin(s) (+{rf_cont_n} continuations) installed at 0x{base:x} ({nkept} already native kept)"
        );
    } else {
        log::info!("dll_lift: {name}: {n} lifted function(s) installed at 0x{base:x} ({nkept} already native kept)");
    }
    n + rf_n
}

/// The lifted body behind a registered pointer `p` (through [`shadow_hook`] / [`cov`] stubs).
pub fn body_of(p: usize) -> usize {
    cov::target_of(shadow_hook::target_of(p))
}

/// All entries of the installed lift of `name` (for difftests).
pub fn entries(name: &str) -> impl Iterator<Item = &'static Lifted> {
    image(name).into_iter().flat_map(|i| i.slices.iter().copied().flatten())
}

/// All `DF_REGFILE` entries of the installed lift of `name` (for difftests).
pub fn regfile_entries(name: &str) -> impl Iterator<Item = &'static LiftedRegfile> {
    image(name).into_iter().flat_map(|i| i.regfile_slices.iter().copied().flatten())
}
