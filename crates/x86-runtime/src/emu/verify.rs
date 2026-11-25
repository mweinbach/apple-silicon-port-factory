//! Universal offline PE32+ binary & DLL differential verifier (`x86-verify`).
//!
//! Loads any Windows x86-64 `.exe` or `.dll` directly from disk (with PE section mapping
//! and `DIR64` base relocations, zero `DllMain`/`WinMain` execution, zero GPU/windowing),
//! discovers every reachable basic block via `aot::discover_blocks_for_verify`, and
//! executes each block (plus linked multi-block CFG traces) inside an isolated 64 KiB
//! scratch memory sandbox with deterministic + adversarial FP/SIMD/flag/unaligned inputs
//! across **all four execution tiers**:
//!
//! 1. **Generic `iced-x86` interpreter** (`Cpu::exec_generic_ins`, reference baseline)
//! 2. **FastOp interpreter** (`fast::exec`)
//! 3. **ARM64 Live JIT** (`JitCache::compile_single_for_verify`)
//! 4. **ARM64 AOT** (`aot::VerifyAotBatch`: `ChunkAssembler` + `split_and_rebase_block`
//!    + `aot_emit_stubs`, both unlinked single-block AND linked multi-block with
//!    `fallthrough nop` and `try_patch_cond_branch`)
//!
//! Optionally exports diverging blocks (and/or a sampled set of FP/SIMD blocks) as
//! self-contained JSON bundles for bit-exact hardware verification on Rosetta 2
//! (`tools/debug/x86_oracle.py`).

use super::{
    aot::{self, VerifyAotBatch},
    fast,
    jit::{self, JitCache, Term, EXIT_CALL, EXIT_JCC, EXIT_JMP, EXIT_RIP, EXIT_SIDE, FALL, FC, FO, FP, FS, FZ},
    BlockCache, Cpu,
};
use iced_x86::{ConditionCode, Decoder, DecoderOptions, FlowControl, Instruction, Mnemonic, OpAccess, OpKind, Register};
use std::collections::{BTreeMap, HashSet};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Instant;

// ---------------------------------------------------------------------------
// Low-level mmap / munmap (no libc crate dependency in x86-runtime)
// ---------------------------------------------------------------------------

const PROT_READ: i32 = 1;
const PROT_WRITE: i32 = 2;
const MAP_PRIVATE: i32 = 0x0002;
const MAP_ANON: i32 = 0x1000;
const SIGBUS: i32 = 10;
const SIGSEGV: i32 = 11;

extern "C" {
    fn mmap(addr: *mut u8, len: usize, prot: i32, flags: i32, fd: i32, offset: i64) -> *mut u8;
    fn munmap(addr: *mut u8, len: usize) -> i32;
    fn sigsetjmp(env: *mut i32, savemask: i32) -> i32;
    fn siglongjmp(env: *mut i32, val: i32) -> !;
    fn sigaction(sig: i32, act: *const SigAction, oldact: *mut SigAction) -> i32;
}

// macOS arm64 `sigjmp_buf` is 49 `i32`s (`_JBLEN + 1` where `_JBLEN = (14+8+2)*2 = 48`); use 64 for headroom.
type SigJmpBuf = [i32; 64];

#[repr(C)]
#[derive(Clone, Copy)]
struct SigAction {
    handler: usize,
    mask: u32,
    flags: i32,
}

#[repr(C)]
struct SigInfo {
    si_signo: i32,
    si_errno: i32,
    si_code: i32,
    si_pid: i32,
    si_uid: u32,
    si_status: i32,
    si_addr: usize,
}

const SA_SIGINFO: i32 = 0x0040;

std::thread_local! {
    static FAULT_JMP: core::cell::UnsafeCell<SigJmpBuf> = const { core::cell::UnsafeCell::new([0; 64]) };
    static FAULT_ARMED: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
    static FAULT_SIG: core::cell::Cell<i32> = const { core::cell::Cell::new(0) };
    static FAULT_ADDR: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
}

unsafe extern "C" fn verify_fault_handler(sig: i32, info: *const SigInfo, _ctx: *mut u8) {
    let armed = FAULT_ARMED.with(|a| a.replace(false));
    if !armed {
        // Re-raise with default action if outside a guarded verify execution.
        let dfl = SigAction { handler: 0, mask: 0, flags: 0 };
        sigaction(sig, &dfl, core::ptr::null_mut());
        return;
    }
    FAULT_SIG.with(|s| s.set(sig));
    let addr = if !info.is_null() { (*info).si_addr } else { 0 };
    FAULT_ADDR.with(|a| a.set(addr));
    let buf = FAULT_JMP.with(|b| b.get() as *mut i32);
    siglongjmp(buf, if sig != 0 { sig } else { 1 });
}

/// Run `f` under a per-thread `SIGSEGV`/`SIGBUS` guard; returns `Err((sig, fault_addr))` if it faults.
unsafe fn run_guarded<T>(f: impl FnOnce() -> T) -> Result<T, (i32, usize)> {
    static INSTALLED: std::sync::Once = std::sync::Once::new();
    INSTALLED.call_once(|| {
        let act = SigAction {
            handler: verify_fault_handler as *const () as usize,
            mask: 0,
            flags: SA_SIGINFO,
        };
        sigaction(SIGSEGV, &act, core::ptr::null_mut());
        sigaction(SIGBUS, &act, core::ptr::null_mut());
    });
    let buf = FAULT_JMP.with(|b| b.get() as *mut i32);
    if sigsetjmp(buf, 1) != 0 {
        // Ensure JIT write-protect is restored to RX after any mid-execution fault.
        super::exec_mem::jit_write_protect(true);
        let sig = FAULT_SIG.with(|s| s.get());
        let addr = FAULT_ADDR.with(|a| a.get());
        return Err((sig, addr));
    }
    FAULT_ARMED.with(|a| a.set(true));
    let r = f();
    FAULT_ARMED.with(|a| a.set(false));
    Ok(r)
}

unsafe fn alloc_rw(len: usize) -> Option<*mut u8> {
    let aligned = (len + 0x3FFF) & !0x3FFF;
    if aligned == 0 {
        return None;
    }
    let p = mmap(core::ptr::null_mut(), aligned, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANON, -1, 0);
    if p.is_null() || p as isize == -1 {
        None
    } else {
        Some(p)
    }
}

// ---------------------------------------------------------------------------
// Offline PE32+ Loader (maps sections + applies DIR64 relocations)
// ---------------------------------------------------------------------------

fn r16(b: &[u8], o: usize) -> Option<u16> {
    let s = b.get(o..o.checked_add(2)?)?;
    Some(u16::from_le_bytes([s[0], s[1]]))
}
fn r32(b: &[u8], o: usize) -> Option<u32> {
    let s = b.get(o..o.checked_add(4)?)?;
    Some(u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
}
fn r64(b: &[u8], o: usize) -> Option<u64> {
    let s = b.get(o..o.checked_add(8)?)?;
    Some(u64::from_le_bytes(s.try_into().ok()?))
}

/// Offline mapped PE32+ image (`munmap`ped on `Drop`).
pub struct MappedPe {
    pub name: String,
    pub base: usize,
    pub size: usize,
    pub alloc_len: usize,
    pub preferred_base: u64,
    /// Readable mapped address spans `[lo, hi)` inside this PE (reads allowed in sandbox).
    pub readable: Vec<(u64, u64)>,
}

impl MappedPe {
    pub fn load(path: &Path) -> Result<Self, String> {
        let data = fs::read(path).map_err(|e| format!("read {}: {e}", path.display()))?;
        let name = path
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.display().to_string());
        Self::from_bytes(&name, &data)
    }

    pub fn from_bytes(name: &str, data: &[u8]) -> Result<Self, String> {
        if data.len() < 0x100 || &data[0..2] != b"MZ" {
            return Err(format!("{name}: not a PE image (missing MZ)"));
        }
        let nt = r32(data, 0x3C).ok_or_else(|| format!("{name}: truncated e_lfanew"))? as usize;
        if data.get(nt..nt + 4) != Some(b"PE\0\0") {
            return Err(format!("{name}: missing PE signature"));
        }
        let machine = r16(data, nt + 4).ok_or_else(|| format!("{name}: truncated COFF header"))?;
        if machine != 0x8664 {
            return Err(format!("{name}: not x86-64 PE32+ (machine={machine:#x})"));
        }
        let nsec = r16(data, nt + 6).unwrap_or(0) as usize;
        let opt_sz = r16(data, nt + 20).unwrap_or(0) as usize;
        let opt = nt + 24;
        let magic = r16(data, opt).unwrap_or(0);
        if magic != 0x20B {
            return Err(format!("{name}: not PE32+ (magic={magic:#x})"));
        }
        let preferred_base = r64(data, opt + 24).unwrap_or(0);
        let size_of_image = r32(data, opt + 56).unwrap_or(0) as usize;
        let size_of_headers = r32(data, opt + 60).unwrap_or(0) as usize;
        if size_of_image < 0x1000 || size_of_image > (1024 << 20) {
            return Err(format!("{name}: invalid SizeOfImage {size_of_image:#x}"));
        }
        let alloc_len = (size_of_image + 0x3FFF) & !0x3FFF;
        let base_ptr = unsafe { alloc_rw(alloc_len) }
            .ok_or_else(|| format!("{name}: mmap({alloc_len:#x}) failed"))?;
        let base = base_ptr as usize;

        unsafe {
            let hdr_copy = size_of_headers.min(data.len()).min(size_of_image);
            core::ptr::copy_nonoverlapping(data.as_ptr(), base_ptr, hdr_copy);

            let sec0 = opt + opt_sz;
            let mut readable = Vec::with_capacity(nsec + 1);
            readable.push((base as u64, (base + hdr_copy) as u64));
            for i in 0..nsec {
                let s = sec0 + i * 40;
                let (Some(vs), Some(va), Some(rs), Some(ro), Some(_ch)) = (
                    r32(data, s + 8),
                    r32(data, s + 12),
                    r32(data, s + 16),
                    r32(data, s + 20),
                    r32(data, s + 36),
                ) else {
                    continue;
                };
                let span = (vs.max(rs) as usize).min(size_of_image.saturating_sub(va as usize));
                if span == 0 {
                    continue;
                }
                let copy_len = (rs as usize).min(data.len().saturating_sub(ro as usize)).min(span);
                if copy_len > 0 {
                    core::ptr::copy_nonoverlapping(data.as_ptr().add(ro as usize), base_ptr.add(va as usize), copy_len);
                }
                readable.push(((base + va as usize) as u64, (base + va as usize + span) as u64));
            }

            // Apply DIR64 base relocations (DataDirectory[5]) so RIP-relative pointer/switch tables work.
            let nrva = r32(data, opt + 108).unwrap_or(0).min(16) as usize;
            if nrva > 5 {
                let rel_rva = r32(data, opt + 112 + 5 * 8).unwrap_or(0) as usize;
                let rel_sz = r32(data, opt + 112 + 5 * 8 + 4).unwrap_or(0) as usize;
                let delta = (base as u64).wrapping_sub(preferred_base);
                if rel_rva != 0 && rel_sz >= 8 && delta != 0 && rel_rva + rel_sz <= size_of_image {
                    let mut p = rel_rva;
                    let end = rel_rva + rel_sz;
                    while p + 8 <= end {
                        let page_rva = core::ptr::read_unaligned(base_ptr.add(p) as *const u32) as usize;
                        let blk_sz = core::ptr::read_unaligned(base_ptr.add(p + 4) as *const u32) as usize;
                        if blk_sz < 8 || p + blk_sz > end {
                            break;
                        }
                        let count = (blk_sz - 8) / 2;
                        for j in 0..count {
                            let ent = core::ptr::read_unaligned(base_ptr.add(p + 8 + j * 2) as *const u16);
                            let ty = ent >> 12;
                            let off = (ent & 0xFFF) as usize;
                            if ty == 10 {
                                // IMAGE_REL_BASED_DIR64
                                let dst = page_rva.wrapping_add(off);
                                if dst + 8 <= size_of_image {
                                    let ptr = base_ptr.add(dst) as *mut u64;
                                    let cur = core::ptr::read_unaligned(ptr);
                                    core::ptr::write_unaligned(ptr, cur.wrapping_add(delta));
                                }
                            }
                        }
                        p += blk_sz;
                    }
                }
            }

            Ok(Self {
                name: name.to_string(),
                base,
                size: size_of_image,
                alloc_len,
                preferred_base,
                readable,
            })
        }
    }

    #[inline]
    fn can_read(&self, addr: u64, len: u64) -> bool {
        let Some(end) = addr.checked_add(len) else {
            return false;
        };
        self.readable.iter().any(|&(lo, hi)| addr >= lo && end <= hi)
    }
}

impl Drop for MappedPe {
    fn drop(&mut self) {
        unsafe {
            munmap(self.base as *mut u8, self.alloc_len);
        }
    }
}

// ---------------------------------------------------------------------------
// Configuration & Report types
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct VerifyConfig {
    /// Maximum blocks to execute per PE binary (0 = all discovered blocks).
    pub max_blocks: usize,
    /// Number of adversarial input trials per block (1..=4, default 3).
    pub trials: usize,
    /// Include FastOp vs Generic comparison.
    pub check_fast: bool,
    /// Include Live ARM64 JIT vs Generic comparison.
    pub check_jit: bool,
    /// Include AOT (split + rebased) single-block comparison.
    pub check_aot: bool,
    /// Include AOT linked multi-block CFG trace comparison (`fallthrough nop` + `cond_patch`).
    pub check_linked_aot: bool,
    /// Optional RVA range filter `[lo, hi)` (e.g. narrow to one subsystem).
    pub rva_filter: Option<(u32, u32)>,
    /// Only test blocks containing at least one FP/SIMD instruction.
    pub fp_only: bool,
    /// Maximum mismatches recorded per binary before stopping verbose collection (still counts all).
    pub max_mismatches: usize,
    /// Threads for discovery & AOT image building.
    pub threads: usize,
    /// Optional directory to dump mismatch (and sampled) snapshots for `tools/debug/x86_oracle.py`.
    pub oracle_dir: Option<PathBuf>,
    /// Also export up to `N` passing FP/SIMD register-only blocks per binary to `oracle_dir`
    /// so Rosetta 2 (`arch -x86_64`) can cross-check interpreter+JIT shared semantics too.
    pub oracle_sample: usize,
    /// Also flag NaN-payload-only diffs (both NaN, different mantissa bits).
    pub strict_nan_payload: bool,
}

impl Default for VerifyConfig {
    fn default() -> Self {
        Self {
            max_blocks: 0,
            trials: 3,
            check_fast: true,
            check_jit: true,
            check_aot: true,
            check_linked_aot: true,
            rva_filter: None,
            fp_only: false,
            max_mismatches: 64,
            threads: std::thread::available_parallelism().map_or(4, |n| n.get().min(8)),
            oracle_dir: None,
            oracle_sample: 0,
            strict_nan_payload: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MismatchRecord {
    pub module: String,
    pub rva: u32,
    pub engine: &'static str,
    pub trial: usize,
    pub kind: &'static str,
    pub detail: String,
    pub disasm: String,
}

#[derive(Clone, Debug, Default)]
pub struct BinaryVerifyReport {
    pub module: String,
    pub roots: usize,
    pub discovered_blocks: usize,
    pub translatable_blocks: usize,
    pub blocks_tested: usize,
    pub trials_executed: usize,
    pub trials_skipped_mem: usize,
    pub trials_skipped_unsup: usize,
    pub fast_mismatches: usize,
    pub jit_mismatches: usize,
    pub aot_mismatches: usize,
    pub linked_traces_tested: usize,
    pub linked_aot_mismatches: usize,
    pub aot_links_patched: usize,
    pub aot_fallthrough_nops: usize,
    pub aot_cond_patched: usize,
    pub nan_payload_diffs: usize,
    pub oracle_exported: usize,
    pub unhandled_mnemonics: BTreeMap<String, usize>,
    pub mismatches: Vec<MismatchRecord>,
    pub elapsed_ms: u64,
}

impl BinaryVerifyReport {
    pub fn total_critical_mismatches(&self) -> usize {
        self.fast_mismatches + self.jit_mismatches + self.aot_mismatches + self.linked_aot_mismatches
    }
}

// ---------------------------------------------------------------------------
// Isolated 64 KiB Scratch Sandbox & Adversarial Trial Seeding
// ---------------------------------------------------------------------------

const SANDBOX_SIZE: usize = 64 * 1024;
const SANDBOX_MID: u64 = 0x8000;

struct Sandbox {
    ptr: *mut u8,
    base: u64,
    snapshot: Vec<u8>,
}

impl Sandbox {
    fn new() -> Result<Self, String> {
        let ptr = unsafe { alloc_rw(SANDBOX_SIZE) }.ok_or_else(|| "mmap sandbox failed".to_string())?;
        Ok(Self {
            ptr,
            base: ptr as u64,
            snapshot: vec![0u8; SANDBOX_SIZE],
        })
    }

    #[inline]
    fn contains(&self, addr: u64, len: u64) -> bool {
        let Some(end) = addr.checked_add(len) else {
            return false;
        };
        addr >= self.base && end <= self.base + SANDBOX_SIZE as u64
    }

    unsafe fn slice(&self) -> &[u8] {
        core::slice::from_raw_parts(self.ptr, SANDBOX_SIZE)
    }

    unsafe fn load_snapshot(&mut self) {
        core::ptr::copy_nonoverlapping(self.snapshot.as_ptr(), self.ptr, SANDBOX_SIZE);
    }

    unsafe fn capture(&self) -> Vec<u8> {
        self.slice().to_vec()
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        unsafe {
            munmap(self.ptr, SANDBOX_SIZE);
        }
    }
}

struct XorRng(u64);

impl XorRng {
    #[inline]
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
}

const ADVERSARIAL_F32: [u32; 20] = [
    0x0000_0000, // +0.0
    0x8000_0000, // -0.0
    0x3F80_0000, // +1.0
    0xBF80_0000, // -1.0
    0x3F00_0000, // +0.5
    0xBF00_0000, // -0.5
    0x3F35_04F3, // +0.70710677
    0x4049_0FDB, // +pi
    0x3586_37BD, // 1e-6
    0x0000_0001, // smallest positive subnormal
    0x8000_0001, // smallest negative subnormal
    0x7F7F_FFFF, // f32::MAX
    0xFF7F_FFFF, // f32::MIN
    0x7F80_0000, // +Inf
    0xFF80_0000, // -Inf
    0x7FC0_0000, // +qNaN
    0xFFC0_0000, // -qNaN (x86 indefinite)
    0x7FC0_1234, // payload qNaN
    0x7F80_0001, // sNaN
    0xFFFF_FFFF, // all-ones SIMD mask / -qNaN
];

const CLEAN_F32: [u32; 12] = [
    0x0000_0000, // 0.0
    0x3F80_0000, // 1.0
    0xBF80_0000, // -1.0
    0x3F00_0000, // 0.5
    0xBF00_0000, // -0.5
    0x4000_0000, // 2.0
    0x3E80_0000, // 0.25
    0x3F35_04F3, // 0.70710677
    0x4049_0FDB, // pi
    0x4228_0000, // 42.0
    0x3C23_D70A, // 0.01
    0xC120_0000, // -10.0
];

fn pack_f32x4(a: u32, b: u32, c: u32, d: u32) -> u128 {
    (a as u128) | ((b as u128) << 32) | ((c as u128) << 64) | ((d as u128) << 96)
}

fn seed_trial(sb: &mut Sandbox, ip: u64, trial: usize) -> Cpu {
    let mut rng = XorRng(ip.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ ((trial as u64 + 1).wrapping_mul(0xBF58_476D_1CE4_E5B9)) | 1);
    // Fill sandbox snapshot with deterministic data appropriate for the trial.
    match trial % 4 {
        0 => {
            for chunk in sb.snapshot.chunks_exact_mut(4) {
                let v = CLEAN_F32[(rng.next() as usize) % CLEAN_F32.len()];
                chunk.copy_from_slice(&v.to_le_bytes());
            }
        }
        1 => {
            for chunk in sb.snapshot.chunks_exact_mut(4) {
                let v = ADVERSARIAL_F32[(rng.next() as usize) % ADVERSARIAL_F32.len()];
                chunk.copy_from_slice(&v.to_le_bytes());
            }
        }
        2 => {
            for chunk in sb.snapshot.chunks_exact_mut(8) {
                let v = rng.next();
                chunk.copy_from_slice(&v.to_le_bytes());
            }
        }
        _ => {
            let pat: [u32; 4] = [0x0000_0000, 0x8000_0000, 0x3F80_0000, 0xBF80_0000];
            for (i, chunk) in sb.snapshot.chunks_exact_mut(4).enumerate() {
                chunk.copy_from_slice(&pat[i & 3].to_le_bytes());
            }
        }
    }

    let mut cpu = Cpu::new_zeroed();
    cpu.rip = ip;
    let misalign = if trial % 4 == 2 { 3u64 } else { 0u64 };
    for r in 0..16 {
        if r == 4 {
            // RSP in the middle of sandbox (16B aligned for trials 0/1/3, unaligned+8 for trial 2)
            cpu.gpr[4] = sb.base + SANDBOX_MID + if trial % 4 == 2 { 8 } else { 0 };
        } else {
            // Spread base pointers across the middle of the sandbox so [reg + disp] usually stays in-bounds.
            let off = 0x2000 + ((r as u64) * 0x400) + if r % 3 == 0 { misalign } else { 0 };
            cpu.gpr[r] = sb.base + off;
        }
    }

    match trial % 4 {
        0 => {
            for i in 0..16 {
                let a = CLEAN_F32[(rng.next() as usize) % CLEAN_F32.len()];
                let b = CLEAN_F32[(rng.next() as usize) % CLEAN_F32.len()];
                let c = CLEAN_F32[(rng.next() as usize) % CLEAN_F32.len()];
                let d = CLEAN_F32[(rng.next() as usize) % CLEAN_F32.len()];
                cpu.xmm[i] = pack_f32x4(a, b, c, d);
                cpu.ymm_hi[i] = 0;
            }
            cpu.cf = false;
            cpu.zf = false;
            cpu.sf = false;
            cpu.of = false;
            cpu.pfv = 0;
        }
        1 => {
            for i in 0..16 {
                let a = ADVERSARIAL_F32[(rng.next() as usize) % ADVERSARIAL_F32.len()];
                let b = ADVERSARIAL_F32[(rng.next() as usize) % ADVERSARIAL_F32.len()];
                let c = ADVERSARIAL_F32[(rng.next() as usize) % ADVERSARIAL_F32.len()];
                let d = ADVERSARIAL_F32[(rng.next() as usize) % ADVERSARIAL_F32.len()];
                cpu.xmm[i] = pack_f32x4(a, b, c, d);
                // Populate non-zero YMM upper halves to catch missing VEX zeroing or SSE upper-half clobbering!
                let ya = ADVERSARIAL_F32[(rng.next() as usize) % ADVERSARIAL_F32.len()];
                let yb = ADVERSARIAL_F32[(rng.next() as usize) % ADVERSARIAL_F32.len()];
                let yc = ADVERSARIAL_F32[(rng.next() as usize) % ADVERSARIAL_F32.len()];
                let yd = ADVERSARIAL_F32[(rng.next() as usize) % ADVERSARIAL_F32.len()];
                cpu.ymm_hi[i] = pack_f32x4(ya, yb, yc, yd);
            }
            let f = rng.next();
            cpu.cf = (f & 1) != 0;
            cpu.zf = (f & 2) != 0;
            cpu.sf = (f & 4) != 0;
            cpu.of = (f & 8) != 0;
            cpu.pfv = if (f & 16) != 0 { 0 } else { 1 };
        }
        2 => {
            for i in 0..16 {
                let lo = rng.next();
                let hi = rng.next();
                cpu.xmm[i] = (lo as u128) | ((hi as u128) << 64);
                let ylo = rng.next();
                let yhi = rng.next();
                cpu.ymm_hi[i] = (ylo as u128) | ((yhi as u128) << 64);
            }
            let f = rng.next();
            cpu.cf = (f & 1) != 0;
            cpu.zf = (f & 2) != 0;
            cpu.sf = (f & 4) != 0;
            cpu.of = (f & 8) != 0;
            cpu.pfv = if (f & 16) != 0 { 0 } else { 1 };
        }
        _ => {
            // Identical edge-case vectors (`0.0`, `-0.0`, `1.0`, `-1.0`) to trigger `0/0`, `x-x`, `min(+0,-0)`, `rsqrt(0)`.
            let v0 = pack_f32x4(0x0000_0000, 0x8000_0000, 0x3F80_0000, 0xBF80_0000);
            let v1 = pack_f32x4(0x8000_0000, 0x0000_0000, 0x7F80_0000, 0xFF80_0000);
            for i in 0..16 {
                cpu.xmm[i] = if i % 2 == 0 { v0 } else { v1 };
                cpu.ymm_hi[i] = if i % 3 == 0 { v0 } else { 0 };
            }
            cpu.cf = true;
            cpu.zf = true;
            cpu.sf = false;
            cpu.of = true;
            cpu.pfv = 0;
        }
    }
    cpu
}

// ---------------------------------------------------------------------------
// Instruction Safety Inspection & Generic / FastOp Stepping
// ---------------------------------------------------------------------------

fn is_forbidden_ins(ins: &Instruction) -> bool {
    if matches!(ins.segment_prefix(), Register::FS | Register::GS) {
        return true;
    }
    use Mnemonic::*;
    matches!(
        ins.mnemonic(),
        Syscall
            | Sysenter
            | Sysexit
            | Sysret
            | Int
            | Int1
            | Int3
            | Into
            | Ud0
            | Ud1
            | Ud2
            | Hlt
            | Cpuid
            | Xgetbv
            | Xsetbv
            | In
            | Insb
            | Insw
            | Insd
            | Out
            | Outsb
            | Outsw
            | Outsd
            | Ldmxcsr
            | Vldmxcsr
            | Stmxcsr
            | Vstmxcsr
            | Fxsave
            | Fxsave64
            | Fxrstor
            | Fxrstor64
            | Xsave
            | Xsave64
            | Xsaveopt
            | Xsaveopt64
            | Xsavec
            | Xsavec64
            | Xrstor
            | Xrstor64
            | Cli
            | Sti
            | Invlpg
            | Wrmsr
            | Rdmsr
            | Swapgs
            | Rdrand
            | Rdseed
            | Pause
            | Monitor
            | Mwait
            | Fldcw
            | Fnstcw
            | Fstcw
            | Fnstsw
            | Fstsw
            | Fninit
            | Finit
            | Fnclex
            | Fclex
            | Fldenv
            | Fnstenv
            | Fstenv
            | Fnsave
            | Fsave
            | Frstor
    )
}

fn is_fp_or_simd_mnemonic(mn: Mnemonic) -> bool {
    let s = format!("{mn:?}");
    let u = s.as_bytes();
    if u.is_empty() {
        return false;
    }
    // Check common SSE/AVX/FMA/x87 prefixes/suffixes
    s.starts_with('V')
        || s.starts_with("Addp")
        || s.starts_with("Adds")
        || s.starts_with("Subp")
        || s.starts_with("Subs")
        || s.starts_with("Mulp")
        || s.starts_with("Muls")
        || s.starts_with("Divp")
        || s.starts_with("Divs")
        || s.starts_with("Sqrt")
        || s.starts_with("Rsqrt")
        || s.starts_with("Rcp")
        || s.starts_with("Minp")
        || s.starts_with("Mins")
        || s.starts_with("Maxp")
        || s.starts_with("Maxs")
        || s.starts_with("Cvt")
        || s.starts_with("Comi")
        || s.starts_with("Ucomi")
        || s.starts_with("Cmpp")
        || s.starts_with("Cmps")
        || s.starts_with("Shuf")
        || s.starts_with("Unpck")
        || s.starts_with("Blend")
        || s.starts_with("Dp")
        || s.starts_with("Hadd")
        || s.starts_with("Hsub")
        || s.starts_with("Round")
        || s.starts_with("Movap")
        || s.starts_with("Movup")
        || s.starts_with("Movss")
        || s.starts_with("Movsd")
        || s.starts_with("Movdq")
        || s.starts_with("P")
            && (s.starts_with("Padd")
                || s.starts_with("Psub")
                || s.starts_with("Pmul")
                || s.starts_with("Pshuf")
                || s.starts_with("Pcmp")
                || s.starts_with("Pmin")
                || s.starts_with("Pmax")
                || s.starts_with("Pand")
                || s.starts_with("Por")
                || s.starts_with("Pxor")
                || s.starts_with("Pmov")
                || s.starts_with("Pack")
                || s.starts_with("Punpck"))
}

fn is_approx_fp_mnemonic(mn: Mnemonic) -> bool {
    use Mnemonic::*;
    matches!(mn, Rsqrtps | Rsqrtss | Vrsqrtps | Vrsqrtss | Rcpps | Rcpss | Vrcpps | Vrcpss)
}

/// Verify that every memory access `ins` would perform under `cpu` stays strictly inside
/// `sb` (for writes) or `sb | pe.readable` (for reads), and avoid `#DE` integer divide traps.
fn check_ins_safe(
    cpu: &Cpu,
    ins: &Instruction,
    sb: &Sandbox,
    pe: &MappedPe,
    factory: &mut iced_x86::InstructionInfoFactory,
) -> bool {
    if is_forbidden_ins(ins) {
        return false;
    }
    let mn = ins.mnemonic();
    // Avoid integer #DE (divide by zero / signed overflow) and bit-test mem ops (whose effective
    // address shifts by `bit_index / 8` outside `eff_addr(ins)`).
    if matches!(mn, Mnemonic::Div | Mnemonic::Idiv) {
        return false;
    }
    if matches!(mn, Mnemonic::Bt | Mnemonic::Bts | Mnemonic::Btr | Mnemonic::Btc)
        && ins.op_count() >= 1
        && ins.op_kind(0) == OpKind::Memory
    {
        return false;
    }
    // Check string instructions (both REP-prefixed and single-iteration non-REP!)
    let is_str_op = matches!(
        mn,
        Mnemonic::Movsb
            | Mnemonic::Movsw
            | Mnemonic::Movsd
            | Mnemonic::Movsq
            | Mnemonic::Stosb
            | Mnemonic::Stosw
            | Mnemonic::Stosd
            | Mnemonic::Stosq
            | Mnemonic::Lodsb
            | Mnemonic::Lodsw
            | Mnemonic::Lodsd
            | Mnemonic::Lodsq
            | Mnemonic::Cmpsb
            | Mnemonic::Cmpsw
            | Mnemonic::Cmpsd
            | Mnemonic::Cmpsq
            | Mnemonic::Scasb
            | Mnemonic::Scasw
            | Mnemonic::Scasd
            | Mnemonic::Scasq
    ) && ins.op_count() > 0
        && ins.op_kind(0) == OpKind::Memory;
    if ins.has_rep_prefix() || ins.has_repne_prefix() || is_str_op {
        let count = if ins.has_rep_prefix() || ins.has_repne_prefix() {
            if cpu.gpr[1] > 64 {
                return false;
            }
            cpu.gpr[1]
        } else {
            1
        };
        let elem = ins.memory_size().size().max(1) as u64;
        let bytes = count.saturating_mul(elem).max(elem);
        if matches!(mn, Mnemonic::Movsb | Mnemonic::Movsw | Mnemonic::Movsd | Mnemonic::Movsq | Mnemonic::Cmpsb | Mnemonic::Cmpsw | Mnemonic::Cmpsd | Mnemonic::Cmpsq | Mnemonic::Lodsb | Mnemonic::Lodsw | Mnemonic::Lodsd | Mnemonic::Lodsq) {
            let lo = if cpu.df { cpu.gpr[6].wrapping_sub(bytes) } else { cpu.gpr[6] };
            if !sb.contains(lo, bytes) && !pe.can_read(lo, bytes) {
                return false;
            }
        }
        if matches!(mn, Mnemonic::Movsb | Mnemonic::Movsw | Mnemonic::Movsd | Mnemonic::Movsq | Mnemonic::Stosb | Mnemonic::Stosw | Mnemonic::Stosd | Mnemonic::Stosq) {
            let lo = if cpu.df { cpu.gpr[7].wrapping_sub(bytes) } else { cpu.gpr[7] };
            if !sb.contains(lo, bytes) {
                return false;
            }
        }
        if matches!(mn, Mnemonic::Scasb | Mnemonic::Scasw | Mnemonic::Scasd | Mnemonic::Scasq | Mnemonic::Cmpsb | Mnemonic::Cmpsw | Mnemonic::Cmpsd | Mnemonic::Cmpsq) {
            let lo = if cpu.df { cpu.gpr[7].wrapping_sub(bytes) } else { cpu.gpr[7] };
            if !sb.contains(lo, bytes) && !pe.can_read(lo, bytes) {
                return false;
            }
        }
    }

    // Stack operations (Push / Pop / Call / Ret / Leave / Enter)
    let rsp = cpu.gpr[4];
    match mn {
        Mnemonic::Push | Mnemonic::Pushfq => {
            if !sb.contains(rsp.wrapping_sub(8), 8) {
                return false;
            }
        }
        Mnemonic::Pop | Mnemonic::Popfq => {
            if !sb.contains(rsp, 8) && !pe.can_read(rsp, 8) {
                return false;
            }
        }
        Mnemonic::Call => {
            if !sb.contains(rsp.wrapping_sub(8), 8) {
                return false;
            }
        }
        Mnemonic::Ret => {
            if !sb.contains(rsp, 8) && !pe.can_read(rsp, 8) {
                return false;
            }
        }
        Mnemonic::Leave => {
            if !sb.contains(cpu.gpr[5], 8) && !pe.can_read(cpu.gpr[5], 8) {
                return false;
            }
        }
        Mnemonic::Enter => return false,
        // Gather / scatter instructions use vector indices we don't pre-validate
        Mnemonic::Vgatherdps | Mnemonic::Vgatherdpd | Mnemonic::Vgatherqps | Mnemonic::Vgatherqpd
        | Mnemonic::Vpgatherdd | Mnemonic::Vpgatherdq | Mnemonic::Vpgatherqd | Mnemonic::Vpgatherqq => {
            return false;
        }
        _ => {}
    }

    // Explicit memory operand check
    let has_mem_op = (0..ins.op_count()).any(|i| ins.op_kind(i) == OpKind::Memory);
    if has_mem_op && mn != Mnemonic::Lea && !matches!(mn, Mnemonic::Prefetcht0 | Mnemonic::Prefetcht1 | Mnemonic::Prefetcht2 | Mnemonic::Prefetchnta | Mnemonic::Prefetchw | Mnemonic::Nop) {
        let addr = eff_addr_at_next_ip(cpu, ins);
        let sz = (ins.memory_size().size() as u64).max(1);
        let info = factory.info(ins);
        let mut writes = false;
        for i in 0..ins.op_count() {
            if ins.op_kind(i) == OpKind::Memory {
                match info.op_access(i) {
                    OpAccess::Write | OpAccess::CondWrite | OpAccess::ReadWrite | OpAccess::ReadCondWrite => {
                        writes = true;
                    }
                    _ => {}
                }
            }
        }
        if ins.has_lock_prefix() || matches!(mn, Mnemonic::Xchg | Mnemonic::Cmpxchg | Mnemonic::Cmpxchg8b | Mnemonic::Cmpxchg16b | Mnemonic::Xadd) {
            writes = true;
        }
        // Special-case `cmpxchg16b` size = 16
        let sz = if mn == Mnemonic::Cmpxchg16b { 16 } else { sz };
        if writes {
            if !sb.contains(addr, sz) {
                return false;
            }
        } else if !sb.contains(addr, sz) && !pe.can_read(addr, sz) {
            return false;
        }
    }
    true
}

/// Compute effective address with `RIP = ins.next_ip()` (without mutating `cpu`).
fn eff_addr_at_next_ip(cpu: &Cpu, ins: &Instruction) -> u64 {
    let mut c = Cpu::new_zeroed();
    c.gpr = cpu.gpr;
    c.rip = ins.next_ip();
    unsafe { c.eff_addr(ins) }
}

/// Copy architectural guest CPU state into a fresh `Cpu`.
#[inline]
fn clone_cpu(src: &Cpu) -> Cpu {
    let mut c = Cpu::new_zeroed();
    c.gpr = src.gpr;
    c.xmm = src.xmm;
    c.ymm_hi = src.ymm_hi;
    c.rip = src.rip;
    c.cf = src.cf;
    c.zf = src.zf;
    c.sf = src.sf;
    c.of = src.of;
    c.pfv = src.pfv;
    c.df = src.df;
    c.last_bits = src.last_bits;
    c.jx.mxcsr = src.jx.mxcsr;
    c
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum StepStatus {
    Ok,
    UnsafeMem,
    Unhandled(Mnemonic),
}

/// Execute one instruction in the Generic `iced-x86` interpreter (matching `emu_run_region`).
unsafe fn step_generic_one(
    cpu: &mut Cpu,
    ins: &Instruction,
    sb: &Sandbox,
    pe: &MappedPe,
    factory: &mut iced_x86::InstructionInfoFactory,
) -> StepStatus {
    if !check_ins_safe(cpu, ins, sb, pe, factory) {
        return StepStatus::UnsafeMem;
    }
    cpu.rip = ins.next_ip();
    match ins.mnemonic() {
        Mnemonic::Ret => {
            let rsp = cpu.gpr[4];
            let ret_addr = core::ptr::read_unaligned(rsp as *const u64);
            let imm = if ins.op_count() > 0 { ins.immediate16() as u64 } else { 0 };
            cpu.gpr[4] = rsp.wrapping_add(8 + imm);
            cpu.rip = ret_addr;
            StepStatus::Ok
        }
        Mnemonic::Call => {
            let target = match ins.op_kind(0) {
                OpKind::NearBranch64 | OpKind::NearBranch32 | OpKind::NearBranch16 => ins.near_branch_target(),
                _ => cpu.read_op(ins, 0, 64),
            };
            let new_rsp = cpu.gpr[4].wrapping_sub(8);
            core::ptr::write_unaligned(new_rsp as *mut u64, cpu.rip);
            cpu.gpr[4] = new_rsp;
            cpu.rip = target;
            StepStatus::Ok
        }
        Mnemonic::Jmp => {
            let target = match ins.op_kind(0) {
                OpKind::NearBranch64 | OpKind::NearBranch32 | OpKind::NearBranch16 => ins.near_branch_target(),
                _ => cpu.read_op(ins, 0, 64),
            };
            cpu.rip = target;
            StepStatus::Ok
        }
        _ => {
            let cc = ins.condition_code();
            let mn = ins.mnemonic();
            if cc != ConditionCode::None
                && ins.flow_control() == FlowControl::ConditionalBranch
                && !matches!(mn, Mnemonic::Loop | Mnemonic::Loope | Mnemonic::Loopne | Mnemonic::Jrcxz | Mnemonic::Jecxz)
            {
                if cpu.eval_cc(cc) {
                    cpu.rip = ins.near_branch_target();
                }
                StepStatus::Ok
            } else if cpu.exec_generic_ins(ins) {
                StepStatus::Ok
            } else {
                StepStatus::Unhandled(ins.mnemonic())
            }
        }
    }
}

/// Execute the exact `FastOp` slice `ops[..k]` (plus optional fused `JccPair` second branch)
/// through the `fast::exec` interpreter.
unsafe fn run_fast_ops(cpu: &mut Cpu, ops: &[fast::FastOp], jcc_pair_f1: Option<fast::FastOp>) {
    let mut i = 0usize;
    while i < ops.len() {
        let op = &ops[i];
        cpu.rip = op.ip.wrapping_add(op.len as u64);
        let fx = fast::exec(cpu, op);
        match fx {
            fast::Fx::Done => {
                i += 1;
            }
            fast::Fx::FuseJcc => {
                i += 1;
                if i < ops.len() {
                    let jop = &ops[i];
                    cpu.rip = jop.ip.wrapping_add(jop.len as u64);
                    match fast::exec(cpu, jop) {
                        fast::Fx::Done => {}
                        fast::Fx::JccNear(t) => cpu.rip = t,
                        _ => {}
                    }
                    i += 1;
                }
            }
            fast::Fx::JccNear(t) => {
                cpu.rip = t;
                break;
            }
            fast::Fx::Jmp(t) => {
                cpu.rip = t;
                break;
            }
            fast::Fx::Call(t) => {
                let new_rsp = cpu.gpr[4].wrapping_sub(8);
                core::ptr::write_unaligned(new_rsp as *mut u64, cpu.rip);
                cpu.gpr[4] = new_rsp;
                cpu.rip = t;
                break;
            }
        }
    }
    if let Some(op2) = jcc_pair_f1 {
        if cpu.rip == op2.ip {
            cpu.rip = op2.ip.wrapping_add(op2.len as u64);
            match fast::exec(cpu, &op2) {
                fast::Fx::Done => {}
                fast::Fx::JccNear(t) => cpu.rip = t,
                _ => {}
            }
        }
    }
}

// ---------------------------------------------------------------------------
// State Comparison (with approx-rsqrt tolerance & NaN-payload classification)
// ---------------------------------------------------------------------------

#[derive(Debug)]
enum DiffResult {
    Match,
    NanPayloadOnly,
    Critical(&'static str, String),
}

#[inline]
fn f32_is_nan(bits: u32) -> bool {
    (bits & 0x7F80_0000) == 0x7F80_0000 && (bits & 0x007F_FFFF) != 0
}

#[inline]
fn f64_is_nan(bits: u64) -> bool {
    (bits & 0x7FF0_0000_0000_0000) == 0x7FF0_0000_0000_0000 && (bits & 0x000F_FFFF_FFFF_FFFF) != 0
}

/// Compare two 128-bit vector values lane-by-lane.
/// Returns `Ok(nan_payload_only)` or `Err(description)`.
fn cmp_vec128(got: u128, want: u128, has_approx: bool) -> Result<bool, String> {
    if got == want {
        return Ok(false);
    }
    // Check if difference is strictly NaN-payload bits across all 4 f32 lanes (or 2 f64 lanes)
    let mut all_nan_payload_f32 = true;
    let mut all_approx_f32 = has_approx;
    for l in 0..4 {
        let g = (got >> (l * 32)) as u32;
        let w = (want >> (l * 32)) as u32;
        if g == w {
            continue;
        }
        if !(f32_is_nan(g) && f32_is_nan(w)) {
            all_nan_payload_f32 = false;
        }
        if all_approx_f32 {
            let gf = f32::from_bits(g);
            let wf = f32::from_bits(w);
            if !(gf.is_finite() && wf.is_finite() && gf.is_sign_positive() == wf.is_sign_positive()) {
                all_approx_f32 = false;
            } else {
                // Hardware rsqrt/rcp (~12-bit precision = ~0.05% relative tolerance)
                let denom = wf.abs().max(1e-30);
                if (gf - wf).abs() / denom > 2e-3 {
                    all_approx_f32 = false;
                }
            }
        }
    }
    if all_approx_f32 {
        return Ok(false);
    }
    let mut all_nan_payload_f64 = true;
    for l in 0..2 {
        let g = (got >> (l * 64)) as u64;
        let w = (want >> (l * 64)) as u64;
        if g == w {
            continue;
        }
        if !(f64_is_nan(g) && f64_is_nan(w)) {
            all_nan_payload_f64 = false;
        }
    }
    if all_nan_payload_f32 || all_nan_payload_f64 {
        return Ok(true);
    }
    Err(format!("got=0x{got:032x} want=0x{want:032x}"))
}

fn compare_states(
    got: &Cpu,
    want: &Cpu,
    mem_got: &[u8],
    mem_want: &[u8],
    flag_mask: u8,
    has_approx: bool,
    strict_nan_payload: bool,
) -> DiffResult {
    if got.rip != want.rip {
        return DiffResult::Critical("rip", format!("got={:#x} want={:#x}", got.rip, want.rip));
    }
    if got.jx.mxcsr != want.jx.mxcsr {
        return DiffResult::Critical(
            "mxcsr",
            format!("got={:#06x} want={:#06x}", got.jx.mxcsr, want.jx.mxcsr),
        );
    }
    for i in 0..16 {
        if got.gpr[i] != want.gpr[i] {
            return DiffResult::Critical(
                "gpr",
                format!("gpr[{i}] got={:#018x} want={:#018x}", got.gpr[i], want.gpr[i]),
            );
        }
    }
    let mut nan_note: Option<String> = None;
    for i in 0..16 {
        match cmp_vec128(got.xmm[i], want.xmm[i], has_approx) {
            Ok(false) => {}
            Ok(true) => {
                let d = format!("xmm{i} got=0x{:032x} want=0x{:032x}", got.xmm[i], want.xmm[i]);
                if strict_nan_payload {
                    return DiffResult::Critical("xmm_nan_payload", d);
                }
                nan_note.get_or_insert(d);
            }
            Err(d) => return DiffResult::Critical("xmm", format!("xmm{i} {d}")),
        }
        match cmp_vec128(got.ymm_hi[i], want.ymm_hi[i], has_approx) {
            Ok(false) => {}
            Ok(true) => {
                let d = format!("ymm{i}_hi got=0x{:032x} want=0x{:032x}", got.ymm_hi[i], want.ymm_hi[i]);
                if strict_nan_payload {
                    return DiffResult::Critical("ymm_nan_payload", d);
                }
                nan_note.get_or_insert(d);
            }
            Err(d) => return DiffResult::Critical("ymm_hi", format!("ymm{i}_hi {d}")),
        }
    }
    let fl = |c: &Cpu| {
        ((c.cf as u8) * FC)
            | ((c.zf as u8) * FZ)
            | ((c.sf as u8) * FS)
            | ((c.of as u8) * FO)
            | ((c.pf() as u8) * FP)
    };
    let fg = fl(got) & flag_mask;
    let fw = fl(want) & flag_mask;
    if fg != fw {
        return DiffResult::Critical(
            "flags",
            format!("flags(mask={flag_mask:#x}) got={fg:#x} want={fw:#x}"),
        );
    }
    if mem_got != mem_want {
        if let Some(off) = mem_got.iter().zip(mem_want).position(|(a, b)| a != b) {
            let align = off & !7;
            let end = (align + 8).min(mem_got.len());
            let mut g_bytes = [0u8; 8];
            let mut w_bytes = [0u8; 8];
            g_bytes[..end - align].copy_from_slice(&mem_got[align..end]);
            w_bytes[..end - align].copy_from_slice(&mem_want[align..end]);
            let gv = u64::from_le_bytes(g_bytes);
            let wv = u64::from_le_bytes(w_bytes);
            // Check if memory write was a NaN-payload-only f32/f64 store
            let g32 = u32::from_le_bytes([g_bytes[off - align & !3], g_bytes[(off - align & !3) + 1], g_bytes[(off - align & !3) + 2], g_bytes[(off - align & !3) + 3]]);
            let w32 = u32::from_le_bytes([w_bytes[off - align & !3], w_bytes[(off - align & !3) + 1], w_bytes[(off - align & !3) + 2], w_bytes[(off - align & !3) + 3]]);
            if !strict_nan_payload && f32_is_nan(g32) && f32_is_nan(w32) {
                nan_note.get_or_insert(format!("mem+{off:#x} got={g32:#010x} want={w32:#010x}"));
            } else if !has_approx {
                return DiffResult::Critical(
                    "mem",
                    format!("sandbox+{off:#x} got={gv:#018x} want={wv:#018x}"),
                );
            } else {
                // Allow approx float tolerance in memory writes too
                let gf = f32::from_bits(g32);
                let wf = f32::from_bits(w32);
                let ok_approx = gf.is_finite()
                    && wf.is_finite()
                    && gf.is_sign_positive() == wf.is_sign_positive()
                    && (gf - wf).abs() / wf.abs().max(1e-30) <= 2e-3;
                if !ok_approx {
                    return DiffResult::Critical(
                        "mem",
                        format!("sandbox+{off:#x} got={gv:#018x} want={wv:#018x}"),
                    );
                }
            }
        }
    }
    if nan_note.is_some() {
        DiffResult::NanPayloadOnly
    } else {
        DiffResult::Match
    }
}

fn compare_x87_states(got: &super::x87::X87State, want: &super::x87::X87State) -> DiffResult {
    if got.cw != want.cw {
        return DiffResult::Critical("x87_cw", format!("got={:#06x} want={:#06x}", got.cw, want.cw));
    }
    if got.status_word() != want.status_word() {
        return DiffResult::Critical(
            "x87_sw",
            format!("got={:#06x} want={:#06x}", got.status_word(), want.status_word()),
        );
    }
    if got.top != want.top || got.valid_mask != want.valid_mask {
        return DiffResult::Critical(
            "x87_tag",
            format!(
                "top got={} want={}, valid got={:#04x} want={:#04x}",
                got.top, want.top, got.valid_mask, want.valid_mask
            ),
        );
    }
    for p in 0..8usize {
        if (want.valid_mask & (1 << p)) != 0 && got.regs[p][..10] != want.regs[p][..10] {
            return DiffResult::Critical(
                "x87_reg",
                format!(
                    "slot[{p}] got={:02x?} want={:02x?}",
                    &got.regs[p][..10],
                    &want.regs[p][..10]
                ),
            );
        }
    }
    DiffResult::Match
}

fn merge_diff(a: DiffResult, b: DiffResult) -> DiffResult {
    match (a, b) {
        (c @ DiffResult::Critical(..), _) | (_, c @ DiffResult::Critical(..)) => c,
        (DiffResult::NanPayloadOnly, _) | (_, DiffResult::NanPayloadOnly) => DiffResult::NanPayloadOnly,
        _ => DiffResult::Match,
    }
}

/// Normalize JIT/AOT `EXIT_CALL` (where `lookup_call` exited before pushing `fall` onto `rsp`)
/// so its architectural post-state matches `step_generic_one`'s `Call` post-state.
unsafe fn normalize_jit_exit(cpu: &mut Cpu, exit_code: u64, sb: &Sandbox) -> bool {
    match exit_code {
        EXIT_RIP | EXIT_JMP | EXIT_JCC => true,
        EXIT_CALL => {
            let new_rsp = cpu.gpr[4].wrapping_sub(8);
            if !sb.contains(new_rsp, 8) {
                return false;
            }
            core::ptr::write_unaligned(new_rsp as *mut u64, cpu.rip);
            cpu.gpr[4] = new_rsp;
            cpu.rip = cpu.jx.target;
            true
        }
        EXIT_SIDE => false,
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// Oracle Export (compatible with `tools/debug/x86_oracle.py`)
// ---------------------------------------------------------------------------

fn hex_slice(b: &[u8]) -> String {
    let mut s = String::with_capacity(b.len() * 2);
    for &x in b {
        use core::fmt::Write;
        let _ = write!(s, "{x:02x}");
    }
    s
}

fn export_oracle_ins(
    dir: &Path,
    seq: usize,
    module: &str,
    rva: u32,
    ins: &Instruction,
    raw_bytes: &[u8],
    pre: &Cpu,
    post: &Cpu,
) -> bool {
    // Only export register-only FP/SIMD instructions directly supported by `x86_oracle.py`
    if (0..ins.op_count()).any(|i| ins.op_kind(i) == OpKind::Memory) {
        return false;
    }
    if ins.op_count() == 0 || ins.op_kind(0) != OpKind::Register {
        return false;
    }
    let r0 = ins.op_register(0);
    let dst_reg = if (Register::XMM0..=Register::XMM15).contains(&r0) {
        (r0 as u8) - (Register::XMM0 as u8)
    } else if (Register::YMM0..=Register::YMM15).contains(&r0) {
        (r0 as u8) - (Register::YMM0 as u8)
    } else {
        return false;
    };
    let _ = fs::create_dir_all(dir);
    let path = dir.join(format!("snap_{seq:05}_{rva:x}.json"));
    let Ok(mut f) = File::create(&path) else {
        return false;
    };
    let fmt_arr128 = |a: &[u128; 16]| -> String {
        let parts: Vec<String> = a.iter().map(|v| format!("\"0x{v:032x}\"")).collect();
        format!("[{}]", parts.join(","))
    };
    let fmt_gpr = |a: &[u64; 16]| -> String {
        let parts: Vec<String> = a.iter().map(|v| format!("\"0x{v:016x}\"")).collect();
        format!("[{}]", parts.join(","))
    };
    let dis = format!("{ins}").replace('"', "\\\"");
    let json = format!(
        "{{\n  \"where\": \"{module}+0x{rva:x}\",\n  \"ip\": \"0x{:x}\",\n  \"disasm\": \"{dis}\",\n  \"bytes\": \"{}\",\n  \"dst_reg\": {dst_reg},\n  \"lane\": 0,\n  \"pre_mxcsr\": \"0x{:x}\",\n  \"post_mxcsr\": \"0x{:x}\",\n  \"pre_gpr\": {},\n  \"pre_xmm\": {},\n  \"pre_ymm_hi\": {},\n  \"post_xmm\": {},\n  \"post_ymm_hi\": {}\n}}\n",
        ins.ip(),
        hex_slice(raw_bytes),
        pre.mxcsr(),
        post.mxcsr(),
        fmt_gpr(&pre.gpr),
        fmt_arr128(&pre.xmm),
        fmt_arr128(&pre.ymm_hi),
        fmt_arr128(&post.xmm),
        fmt_arr128(&post.ymm_hi),
    );
    f.write_all(json.as_bytes()).is_ok()
}

// ---------------------------------------------------------------------------
// Core Binary Verifier
// ---------------------------------------------------------------------------

/// Decode the instructions of one `Prep` block (`ops[..k]`) into concrete `iced_x86::Instruction`s.
unsafe fn decode_block_insns(ops: &[fast::FastOp]) -> Option<Vec<Instruction>> {
    let mut out = Vec::with_capacity(ops.len());
    for op in ops {
        let len = (op.len as usize).clamp(1, 15);
        let bytes = core::slice::from_raw_parts(op.ip as *const u8, len);
        let mut dec = Decoder::with_ip(64, bytes, op.ip, DecoderOptions::NONE);
        let ins = dec.decode();
        if ins.is_invalid() {
            return None;
        }
        out.push(ins);
    }
    Some(out)
}

fn format_block_disasm(insns: &[Instruction]) -> String {
    insns
        .iter()
        .map(|i| format!("{i}"))
        .collect::<Vec<_>>()
        .join("; ")
}

/// Verify a single PE32+ binary (`.exe` or `.dll`) offline across all 4 execution tiers.
pub fn verify_pe_file(path: &Path, cfg: &VerifyConfig) -> Result<BinaryVerifyReport, String> {
    let pe = MappedPe::load(path)?;
    verify_mapped_pe(&pe, cfg)
}

/// Verify an already-mapped [`MappedPe`] image offline across Generic, FastOp, Live JIT,
/// AOT (unlinked split+rebased), and AOT Linked (multi-block CFG traces with `fallthrough nop`
/// & `cond_patch`).
pub fn verify_mapped_pe(pe: &MappedPe, cfg: &VerifyConfig) -> Result<BinaryVerifyReport, String> {
    super::with_frozen_tsc(|| verify_with_frozen_clock(pe, cfg))
}

fn verify_with_frozen_clock(pe: &MappedPe, cfg: &VerifyConfig) -> Result<BinaryVerifyReport, String> {
    let t0 = Instant::now();
    let mut rep = BinaryVerifyReport {
        module: pe.name.clone(),
        ..BinaryVerifyReport::default()
    };

    let (roots, mut ips) = unsafe { aot::discover_blocks_for_verify(pe.base, pe.size, 0, 128, cfg.threads) };
    rep.roots = roots.len();
    rep.discovered_blocks = ips.len();
    if let Some((rlo, rhi)) = cfg.rva_filter {
        ips.retain(|&ip| {
            let rva = ip.wrapping_sub(pe.base as u64) as u32;
            rva >= rlo && rva < rhi
        });
    }
    if cfg.max_blocks > 0 && ips.len() > cfg.max_blocks {
        ips.truncate(cfg.max_blocks);
    }
    if ips.is_empty() {
        rep.elapsed_ms = t0.elapsed().as_millis() as u64;
        return Ok(rep);
    }

    let saved_host_fpcr = super::read_fpcr();
    struct VerifyFpcrRestore(u64);
    impl Drop for VerifyFpcrRestore {
        fn drop(&mut self) {
            if super::read_fpcr() != self.0 {
                super::write_fpcr(self.0);
            }
        }
    }
    let _fpcr_restore = VerifyFpcrRestore(saved_host_fpcr);
    struct VerifyX87Restore(super::x87::X87State);
    impl Drop for VerifyX87Restore {
        fn drop(&mut self) {
            super::x87::x87_restore(self.0);
        }
    }
    let _x87_restore = VerifyX87Restore(super::x87::x87_snapshot());

    let mut sb = Sandbox::new()?;
    let mut bc = Box::new(BlockCache::new());
    let bcp: *mut BlockCache = &mut *bc;
    let mut jit_cache = JitCache::new();
    let mut factory = iced_x86::InstructionInfoFactory::new();
    let mut seen_mismatch_keys: HashSet<(u32, &'static str, &'static str)> = HashSet::new();
    let mut oracle_seq = 0usize;
    let mut oracle_samples = 0usize;

    // Process in AOT chunks of up to 4096 blocks so we bound MAP_JIT memory even for 100 MiB EXEs
    // while exercising real multi-block `ChunkAssembler` hot/cold splitting and intra-chunk linking!
    const CHUNK_BLOCKS: usize = 4096;
    for chunk in ips.chunks(CHUNK_BLOCKS) {
        // Build unlinked AOT region for this chunk (if AOT checking requested).
        let mut aot_batch = if cfg.check_aot || cfg.check_linked_aot {
            unsafe { VerifyAotBatch::build(chunk, cfg.threads) }
        } else {
            None
        };

        // --- Pass 1: Single-block 4-way verification (Generic vs FastOp vs Live JIT vs Unlinked AOT) ---
        for &ip in chunk {
            let rva = ip.wrapping_sub(pe.base as u64) as u32;
            let Some(prep) = (unsafe { jit::prep_block(bcp, ip) }) else {
                continue;
            };
            let ops = &prep.ops[..prep.k];
            let Some(insns) = (unsafe { decode_block_insns(ops) }) else {
                continue;
            };
            let has_fp = insns.iter().any(|ins| is_fp_or_simd_mnemonic(ins.mnemonic()));
            if cfg.fp_only && !has_fp {
                continue;
            }
            let has_approx = insns.iter().any(|ins| is_approx_fp_mnemonic(ins.mnemonic()));
            rep.translatable_blocks += 1;

            // Optional fused second instruction for `Term::JccPair`
            let jcc_pair_extra: Option<(fast::FastOp, Instruction)> = match prep.term {
                Term::JccPair { .. } => {
                    let f1 = ops[prep.k - 1].ip.wrapping_add(ops[prep.k - 1].len as u64);
                    let mut dummy = 0u64;
                    unsafe { BlockCache::get(bcp, f1, &mut dummy) }
                        .ok()
                        .and_then(|(off2, n2)| {
                            if n2 == 0 {
                                return None;
                            }
                            let op2 = unsafe { (&(*bcp).ops)[off2] };
                            let ins2 = unsafe { decode_block_insns(&[op2]) }?.pop()?;
                            Some((op2, ins2))
                        })
                }
                _ => None,
            };

            let jit_code = if cfg.check_jit {
                unsafe { jit_cache.compile_single_for_verify(ip, bcp) }
            } else {
                None
            };
            let aot_entry = if cfg.check_aot {
                aot_batch.as_ref().and_then(|b| b.block_entry(ip))
            } else {
                None
            };

            let mut block_ran_any = false;
            for trial in 0..cfg.trials.max(1) {
                let init_cpu = seed_trial(&mut sb, ip, trial);

                // 1) Generic iced-x86 interpreter reference run (also validates every memory access!)
                super::apply_guest_mxcsr_hw(init_cpu.jx.mxcsr);
                super::x87::x87_reset();
                unsafe { sb.load_snapshot() };
                let mut gen_cpu = clone_cpu(&init_cpu);
                let mut ok = true;
                let mut oracle_cand: Option<(Instruction, Vec<u8>, Cpu, Cpu)> = None;
                for ins in &insns {
                    let want_oracle = cfg.oracle_dir.is_some()
                        && oracle_samples < cfg.oracle_sample
                        && oracle_cand.is_none()
                        && is_fp_or_simd_mnemonic(ins.mnemonic())
                        && !is_approx_fp_mnemonic(ins.mnemonic())
                        && (0..ins.op_count()).all(|oi| ins.op_kind(oi) != OpKind::Memory);
                    let pre_one = want_oracle.then(|| clone_cpu(&gen_cpu));
                    match unsafe { step_generic_one(&mut gen_cpu, ins, &sb, pe, &mut factory) } {
                        StepStatus::Ok => {
                            if let Some(pre) = pre_one {
                                let raw = unsafe {
                                    core::slice::from_raw_parts(ins.ip() as *const u8, ins.len()).to_vec()
                                };
                                oracle_cand = Some((*ins, raw, pre, clone_cpu(&gen_cpu)));
                            }
                        }
                        StepStatus::UnsafeMem => {
                            rep.trials_skipped_mem += 1;
                            ok = false;
                            break;
                        }
                        StepStatus::Unhandled(mn) => {
                            *rep.unhandled_mnemonics.entry(format!("{mn:?}")).or_default() += 1;
                            rep.trials_skipped_unsup += 1;
                            ok = false;
                            break;
                        }
                    }
                }
                if ok {
                    if let Some((_, ref ins2)) = jcc_pair_extra {
                        if gen_cpu.rip == ins2.ip() {
                            match unsafe { step_generic_one(&mut gen_cpu, ins2, &sb, pe, &mut factory) } {
                                StepStatus::Ok => {}
                                StepStatus::UnsafeMem => {
                                    rep.trials_skipped_mem += 1;
                                    ok = false;
                                }
                                StepStatus::Unhandled(mn) => {
                                    *rep.unhandled_mnemonics.entry(format!("{mn:?}")).or_default() += 1;
                                    rep.trials_skipped_unsup += 1;
                                    ok = false;
                                }
                            }
                        }
                    }
                }
                if !ok {
                    continue;
                }
                let gen_mem = unsafe { sb.capture() };
                let gen_x87 = super::x87::x87_snapshot();
                block_ran_any = true;
                rep.trials_executed += 1;

                if let (Some(odir), Some((oins, oraw, opre, opost))) = (cfg.oracle_dir.as_ref(), oracle_cand) {
                    oracle_seq += 1;
                    let orva = oins.ip().wrapping_sub(pe.base as u64) as u32;
                    if export_oracle_ins(odir, oracle_seq, &pe.name, orva, &oins, &oraw, &opre, &opost) {
                        oracle_samples += 1;
                        rep.oracle_exported += 1;
                    }
                }

                // Compute exact live flag mask for JIT/AOT based on the taken/not-taken edge.
                let jit_flag_mask = match prep.term {
                    Term::Jcc { target, .. } => {
                        if gen_cpu.rip == target {
                            prep.edge.live[0]
                        } else {
                            prep.edge.live[1]
                        }
                    }
                    _ => prep.live_out,
                };

                let mut record_diff = |engine: &'static str, diff: DiffResult, count_slot: &mut usize| {
                    match diff {
                        DiffResult::Match => {}
                        DiffResult::NanPayloadOnly => {
                            rep.nan_payload_diffs += 1;
                        }
                        DiffResult::Critical(kind, detail) => {
                            *count_slot += 1;
                            if seen_mismatch_keys.insert((rva, engine, kind))
                                && rep.mismatches.len() < cfg.max_mismatches
                            {
                                rep.mismatches.push(MismatchRecord {
                                    module: pe.name.clone(),
                                    rva,
                                    engine,
                                    trial,
                                    kind,
                                    detail,
                                    disasm: format_block_disasm(&insns),
                                });
                            }
                        }
                    }
                };

                // 2) FastOp vs Generic
                if cfg.check_fast {
                    super::apply_guest_mxcsr_hw(init_cpu.jx.mxcsr);
                    super::x87::x87_reset();
                    unsafe { sb.load_snapshot() };
                    let mut fast_cpu = clone_cpu(&init_cpu);
                    unsafe { run_fast_ops(&mut fast_cpu, ops, jcc_pair_extra.as_ref().map(|x| x.0)) };
                    let fast_mem = unsafe { sb.capture() };
                    let fast_x87 = super::x87::x87_snapshot();
                    let d = merge_diff(
                        compare_states(
                            &fast_cpu,
                            &gen_cpu,
                            &fast_mem,
                            &gen_mem,
                            FALL,
                            has_approx,
                            cfg.strict_nan_payload,
                        ),
                        compare_x87_states(&fast_x87, &gen_x87),
                    );
                    let mut c = 0;
                    record_diff("fast", d, &mut c);
                    rep.fast_mismatches += c;
                }

                // Indirect calls (`call r/m64`) to target `0` intentionally hit `jit_host_xfer`'s
                // graceful null-call guard (`rax = 0; continue at fallthrough rip`) instead of
                // crashing at `rip = 0`. Skip JIT/AOT comparison on such null-target trials.
                let null_indirect_call = matches!(prep.term, Term::CallR | Term::CallM) && gen_cpu.rip == 0;
                // Live JIT emits direct in-block back-edges for self-looping blocks (`target == ip`),
                // so a taken self-loop executes multiple iterations instead of stopping after 1 block.
                let taken_self_loop = gen_cpu.rip == ip;

                // 3) Live ARM64 JIT vs Generic (fault-guarded)
                if let Some(code) = jit_code {
                    if !null_indirect_call && !taken_self_loop {
                        super::apply_guest_mxcsr_hw(init_cpu.jx.mxcsr);
                        super::x87::x87_reset();
                        unsafe { sb.load_snapshot() };
                        let mut jcpu = clone_cpu(&init_cpu);
                        let run_res = unsafe { run_guarded(|| jit_cache.run_code_for_verify(&mut jcpu, bcp, None, code, 256)) };
                        match run_res {
                            Err((sig, addr)) => {
                                let mut c = 0;
                                record_diff("jit", DiffResult::Critical("fault", format!("sig={sig} addr={addr:#x}")), &mut c);
                                rep.jit_mismatches += c;
                            }
                            Ok((ex, _)) => {
                                if unsafe { normalize_jit_exit(&mut jcpu, ex, &sb) } {
                                    let jmem = unsafe { sb.capture() };
                                    let jx87 = super::x87::x87_snapshot();
                                    let d = merge_diff(
                                        compare_states(
                                            &jcpu,
                                            &gen_cpu,
                                            &jmem,
                                            &gen_mem,
                                            jit_flag_mask,
                                            has_approx,
                                            cfg.strict_nan_payload,
                                        ),
                                        compare_x87_states(&jx87, &gen_x87),
                                    );
                                    let mut c = 0;
                                    record_diff("jit", d, &mut c);
                                    rep.jit_mismatches += c;
                                }
                            }
                        }
                    }
                }

                // 4) Unlinked AOT (split + rebased + AOT stubs) vs Generic (fault-guarded)
                if let Some((enter, code)) = aot_entry {
                    if !null_indirect_call {
                        super::apply_guest_mxcsr_hw(init_cpu.jx.mxcsr);
                        super::x87::x87_reset();
                        unsafe { sb.load_snapshot() };
                        let mut acpu = clone_cpu(&init_cpu);
                        let run_res = unsafe { run_guarded(|| jit_cache.run_code_for_verify(&mut acpu, bcp, Some(enter), code, 256)) };
                        match run_res {
                            Err((sig, addr)) => {
                                let mut c = 0;
                                record_diff("aot", DiffResult::Critical("fault", format!("sig={sig} addr={addr:#x}")), &mut c);
                                rep.aot_mismatches += c;
                            }
                            Ok((ex, _)) => {
                                if unsafe { normalize_jit_exit(&mut acpu, ex, &sb) } {
                                    let amem = unsafe { sb.capture() };
                                    let ax87 = super::x87::x87_snapshot();
                                    let d = merge_diff(
                                        compare_states(
                                            &acpu,
                                            &gen_cpu,
                                            &amem,
                                            &gen_mem,
                                            jit_flag_mask,
                                            has_approx,
                                            cfg.strict_nan_payload,
                                        ),
                                        compare_x87_states(&ax87, &gen_x87),
                                    );
                                    let mut c = 0;
                                    record_diff("aot", d, &mut c);
                                    rep.aot_mismatches += c;
                                }
                            }
                        }
                    }
                }
            }
            if block_ran_any {
                rep.blocks_tested += 1;
            }
        }

        // --- Pass 2: Linked AOT multi-block CFG trace verification (`link_pass`: fallthrough-nop + cond_patch + b/bl) ---
        if cfg.check_linked_aot {
            if let Some(ref mut batch) = aot_batch {
                let (patched, nops, conds) = unsafe { batch.apply_intra_links() };
                rep.aot_links_patched += patched;
                rep.aot_fallthrough_nops += nops;
                rep.aot_cond_patched += conds;

                for &start_ip in chunk {
                    let Some((enter, code)) = batch.block_entry(start_ip) else {
                        continue;
                    };
                    let rva = start_ip.wrapping_sub(pe.base as u64) as u32;
                    let init_cpu = seed_trial(&mut sb, start_ip, 1);

                    // Walk up to 6 linked blocks in Generic Interpreter FIRST to ensure the entire
                    // linked CFG trace exits the batch cleanly with zero unsafe memory accesses or loops.
                    super::apply_guest_mxcsr_hw(init_cpu.jx.mxcsr);
                    super::x87::x87_reset();
                    unsafe { sb.load_snapshot() };
                    let mut gen_cpu = clone_cpu(&init_cpu);
                    let mut cur_ip = start_ip;
                    let mut visited: HashSet<u64> = HashSet::new();
                    let mut trace_blocks = 0usize;
                    let mut trace_ok = true;
                    let mut trace_approx = false;
                    let mut crossed_dead_flags = false;
                    let mut last_flag_mask = FALL;
                    let mut trace_disasm = Vec::new();

                    while batch.contains(cur_ip) && trace_blocks < 6 {
                        if !visited.insert(cur_ip) {
                            // Loop inside batch: skip so linked AOT doesn't spin past our step cap.
                            trace_ok = false;
                            break;
                        }
                        let Some(prep) = (unsafe { jit::prep_block(bcp, cur_ip) }) else {
                            trace_ok = false;
                            break;
                        };
                        let ops = &prep.ops[..prep.k];
                        let Some(insns) = (unsafe { decode_block_insns(ops) }) else {
                            trace_ok = false;
                            break;
                        };
                        if insns.iter().any(|ins| is_approx_fp_mnemonic(ins.mnemonic())) {
                            trace_approx = true;
                        }
                        if trace_disasm.len() < 12 {
                            trace_disasm.push(format_block_disasm(&insns));
                        }
                        for ins in &insns {
                            match unsafe { step_generic_one(&mut gen_cpu, ins, &sb, pe, &mut factory) } {
                                StepStatus::Ok => {}
                                _ => {
                                    trace_ok = false;
                                    break;
                                }
                            }
                        }
                        if !trace_ok {
                            break;
                        }
                        if let Term::JccPair { .. } = prep.term {
                            let f1 = ops[prep.k - 1].ip.wrapping_add(ops[prep.k - 1].len as u64);
                            if gen_cpu.rip == f1 {
                                let mut d2 = 0u64;
                                if let Ok((off2, n2)) = unsafe { BlockCache::get(bcp, f1, &mut d2) } {
                                    if n2 > 0 {
                                        let op2 = unsafe { (&(*bcp).ops)[off2] };
                                        if let Some(mut v2) = unsafe { decode_block_insns(&[op2]) } {
                                            if let Some(ins2) = v2.pop() {
                                                if unsafe { step_generic_one(&mut gen_cpu, &ins2, &sb, pe, &mut factory) } != StepStatus::Ok {
                                                    trace_ok = false;
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        let block_flag_mask = match prep.term {
                            Term::Jcc { target, .. } => {
                                if gen_cpu.rip == target {
                                    prep.edge.live[0]
                                } else {
                                    prep.edge.live[1]
                                }
                            }
                            _ => prep.live_out,
                        };
                        if block_flag_mask == 0 {
                            crossed_dead_flags = true;
                        }
                        last_flag_mask = if crossed_dead_flags { 0 } else { block_flag_mask };
                        trace_blocks += 1;
                        // Stop following if the block ends with an indirect transfer (`ret`/`call r/m`/`jmp r/m`)
                        // because an empty `JitCache::index` exits on dynamic transfers even if the target is in-batch.
                        if matches!(prep.term, Term::Ret | Term::CallR | Term::CallM | Term::JmpR | Term::JmpM) {
                            if matches!(prep.term, Term::CallR | Term::CallM) && gen_cpu.rip == 0 {
                                trace_ok = false;
                            }
                            break;
                        }
                        cur_ip = gen_cpu.rip;
                    }

                    // Only compare traces that actually traversed >= 2 linked blocks and cleanly exited the batch!
                    if !trace_ok || trace_blocks < 2 || batch.contains(gen_cpu.rip) {
                        continue;
                    }
                    let gen_mem = unsafe { sb.capture() };
                    let gen_x87 = super::x87::x87_snapshot();
                    super::apply_guest_mxcsr_hw(init_cpu.jx.mxcsr);
                    super::x87::x87_reset();
                    unsafe { sb.load_snapshot() };
                    let mut acpu = clone_cpu(&init_cpu);
                    let run_res = unsafe { run_guarded(|| jit_cache.run_code_for_verify(&mut acpu, bcp, Some(enter), code, 1024)) };
                    let (ex, _) = match run_res {
                        Ok(v) => v,
                        Err((sig, addr)) => {
                            rep.linked_aot_mismatches += 1;
                            if seen_mismatch_keys.insert((rva, "aot_linked", "fault"))
                                && rep.mismatches.len() < cfg.max_mismatches
                            {
                                rep.mismatches.push(MismatchRecord {
                                    module: pe.name.clone(),
                                    rva,
                                    engine: "aot_linked",
                                    trial: 1,
                                    kind: "fault",
                                    detail: format!("[{trace_blocks} linked blocks] sig={sig} addr={addr:#x}"),
                                    disasm: trace_disasm.join(" | "),
                                });
                            }
                            continue;
                        }
                    };
                    if !unsafe { normalize_jit_exit(&mut acpu, ex, &sb) } {
                        continue;
                    }
                    let amem = unsafe { sb.capture() };
                    let ax87 = super::x87::x87_snapshot();
                    rep.linked_traces_tested += 1;
                    match merge_diff(
                        compare_states(
                            &acpu,
                            &gen_cpu,
                            &amem,
                            &gen_mem,
                            last_flag_mask,
                            trace_approx,
                            cfg.strict_nan_payload,
                        ),
                        compare_x87_states(&ax87, &gen_x87),
                    ) {
                        DiffResult::Match => {}
                        DiffResult::NanPayloadOnly => {
                            rep.nan_payload_diffs += 1;
                        }
                        DiffResult::Critical(kind, detail) => {
                            rep.linked_aot_mismatches += 1;
                            if seen_mismatch_keys.insert((rva, "aot_linked", kind))
                                && rep.mismatches.len() < cfg.max_mismatches
                            {
                                rep.mismatches.push(MismatchRecord {
                                    module: pe.name.clone(),
                                    rva,
                                    engine: "aot_linked",
                                    trial: 1,
                                    kind,
                                    detail: format!("[{trace_blocks} linked blocks] {detail}"),
                                    disasm: trace_disasm.join(" | "),
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    rep.elapsed_ms = t0.elapsed().as_millis() as u64;
    Ok(rep)
}

/// Discover all PE32+ x86-64 `.exe` and `.dll` files in a path (single file or directory).
pub fn collect_pe_targets(input: &Path) -> Result<Vec<PathBuf>, String> {
    if input.is_file() {
        return Ok(vec![input.to_path_buf()]);
    }
    if !input.is_dir() {
        return Err(format!("not a file or directory: {}", input.display()));
    }
    let mut out = Vec::new();
    let entries = fs::read_dir(input).map_err(|e| format!("read_dir {}: {e}", input.display()))?;
    for ent in entries.flatten() {
        let p = ent.path();
        if !p.is_file() {
            continue;
        }
        let ext = p.extension().and_then(|s| s.to_str()).unwrap_or("").to_ascii_lowercase();
        if ext == "exe" || ext == "dll" {
            out.push(p);
        }
    }
    out.sort();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a minimal valid PE32+ image in memory containing `code` at RVA `0x1000`
    /// (with `.pdata` covering `0x1000..0x1000+code.len()`).
    fn synth_pe(code: &[u8]) -> Vec<u8> {
        let mut pe = vec![0u8; 0x3000];
        pe[0..2].copy_from_slice(b"MZ");
        pe[0x3C..0x40].copy_from_slice(&0x80u32.to_le_bytes());
        let nt = 0x80;
        pe[nt..nt + 4].copy_from_slice(b"PE\0\0");
        pe[nt + 4..nt + 6].copy_from_slice(&0x8664u16.to_le_bytes()); // AMD64
        pe[nt + 6..nt + 8].copy_from_slice(&2u16.to_le_bytes()); // 2 sections (.text, .pdata)
        pe[nt + 20..nt + 22].copy_from_slice(&0xF0u16.to_le_bytes()); // SizeOfOptionalHeader
        let opt = nt + 24;
        pe[opt..opt + 2].copy_from_slice(&0x20Bu16.to_le_bytes()); // PE32+
        pe[opt + 16..opt + 20].copy_from_slice(&0x1000u32.to_le_bytes()); // EntryPoint
        pe[opt + 24..opt + 32].copy_from_slice(&0x1_4000_0000u64.to_le_bytes()); // ImageBase
        pe[opt + 56..opt + 60].copy_from_slice(&0x3000u32.to_le_bytes()); // SizeOfImage
        pe[opt + 60..opt + 64].copy_from_slice(&0x400u32.to_le_bytes()); // SizeOfHeaders
        pe[opt + 108..opt + 112].copy_from_slice(&16u32.to_le_bytes()); // NumberOfRvaAndSizes
        // DataDirectory[3] = Exception Table (.pdata @ 0x2000, size 12)
        pe[opt + 112 + 3 * 8..opt + 112 + 3 * 8 + 4].copy_from_slice(&0x2000u32.to_le_bytes());
        pe[opt + 112 + 3 * 8 + 4..opt + 112 + 3 * 8 + 8].copy_from_slice(&12u32.to_le_bytes());
        // Section 0: .text (RVA 0x1000, Raw 0x1000, Size 0x1000, EXEC|READ|CODE)
        let s0 = opt + 0xF0;
        pe[s0..s0 + 5].copy_from_slice(b".text");
        pe[s0 + 8..s0 + 12].copy_from_slice(&0x1000u32.to_le_bytes());
        pe[s0 + 12..s0 + 16].copy_from_slice(&0x1000u32.to_le_bytes());
        pe[s0 + 16..s0 + 20].copy_from_slice(&0x1000u32.to_le_bytes());
        pe[s0 + 20..s0 + 24].copy_from_slice(&0x1000u32.to_le_bytes());
        pe[s0 + 36..s0 + 40].copy_from_slice(&0x6000_0020u32.to_le_bytes());
        // Section 1: .pdata (RVA 0x2000, Raw 0x2000, Size 0x1000, READ|DATA)
        let s1 = s0 + 40;
        pe[s1..s1 + 6].copy_from_slice(b".pdata");
        pe[s1 + 8..s1 + 12].copy_from_slice(&0x1000u32.to_le_bytes());
        pe[s1 + 12..s1 + 16].copy_from_slice(&0x2000u32.to_le_bytes());
        pe[s1 + 16..s1 + 20].copy_from_slice(&0x1000u32.to_le_bytes());
        pe[s1 + 20..s1 + 24].copy_from_slice(&0x2000u32.to_le_bytes());
        pe[s1 + 36..s1 + 40].copy_from_slice(&0x4000_0040u32.to_le_bytes());
        // Code + .pdata entry
        pe[0x1000..0x1000 + code.len()].copy_from_slice(code);
        pe[0x2000..0x2004].copy_from_slice(&0x1000u32.to_le_bytes());
        pe[0x2004..0x2008].copy_from_slice(&(0x1000u32 + code.len() as u32).to_le_bytes());
        pe
    }

    #[test]
    fn verify_timestamp_blocks_and_linked_aot() {
        let code = [
            0x0f, 0x31,             // rdtsc
            0x41, 0x89, 0xc0,       // mov r8d, eax
            0x74, 0x04,             // jz plain
            0x0f, 0x01, 0xf9, 0xc3, // rdtscp; ret
            0x0f, 0x31, 0xc3,       // plain: rdtsc; ret
        ];
        let mapped = MappedPe::from_bytes("clock.dll", &synth_pe(&code)).expect("map clock PE");
        let cfg = VerifyConfig {
            trials: 4,
            threads: 2,
            rva_filter: Some((0x1000, 0x1000 + code.len() as u32)),
            ..VerifyConfig::default()
        };
        let rep = verify_mapped_pe(&mapped, &cfg).expect("verify clock PE");
        assert_eq!(rep.blocks_tested, 3, "{rep:?}");
        assert_eq!(rep.trials_executed, 12, "{rep:?}");
        assert!(rep.linked_traces_tested > 0, "{rep:?}");
        assert_eq!(rep.total_critical_mismatches(), 0, "{:?}", rep.mismatches);
    }

    #[test]
    fn verify_synthetic_pe_simd_and_linked_aot_clean() {
        // Two-block FP/SIMD + branch sequence:
        //   vaddps xmm0, xmm1, xmm2     (c5 f0 58 c2)
        //   vmulps xmm0, xmm0, xmm3     (c5 f8 59 c3)
        //   vsubss xmm4, xmm0, xmm1     (c5 fa 5c e1)
        //   ucomiss xmm4, xmm2          (0f 2e e2)
        //   ja +4                       (77 04)
        //   vaddps xmm5, xmm0, xmm4     (c5 f8 58 ec)
        //   ret                         (c3)
        // target:
        //   vsubps xmm5, xmm0, xmm4     (c5 f8 5c ec)
        //   ret                         (c3)
        let code: [u8; 25] = [
            0xC5, 0xF0, 0x58, 0xC2,
            0xC5, 0xF8, 0x59, 0xC3,
            0xC5, 0xFA, 0x5C, 0xE1,
            0x0F, 0x2E, 0xE2,
            0x77, 0x05,
            0xC5, 0xF8, 0x58, 0xEC,
            0xC3,
            0xC5, 0xF8, 0x5C,
        ];
        let mut full = code.to_vec();
        full.push(0xEC);
        full.push(0xC3);
        let pe_bytes = synth_pe(&full);
        let mapped = MappedPe::from_bytes("synth_simd.dll", &pe_bytes).expect("map synth PE");
        let cfg = VerifyConfig {
            trials: 4,
            threads: 1,
            ..VerifyConfig::default()
        };
        let rep = verify_mapped_pe(&mapped, &cfg).expect("verify synth PE");
        assert!(rep.blocks_tested >= 2, "expected >=2 blocks tested, got {rep:?}");
        assert!(rep.trials_executed >= 8, "expected >=8 trials executed, got {rep:?}");
        assert_eq!(
            rep.total_critical_mismatches(),
            0,
            "unexpected mismatches on synth SIMD PE: {:?}",
            rep.mismatches
        );
    }
}
