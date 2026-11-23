//! Low-level x86-64 function patching, trampoline re-encoding, call counters, and temporary mocks.

use crate::thunks;
use anyhow::{bail, Result};
use iced_x86::{
    BlockEncoder, BlockEncoderOptions, Decoder, DecoderOptions, FlowControl, Instruction,
    InstructionBlock,
};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;

const JMP_LEN: usize = 5;
pub(super) static PREFERRED_EXE_BASE: AtomicU64 = AtomicU64::new(0x1_4000_0000);

/// Bit tag on non-`x86_64` trampoline addresses so direct calls to `orig` in
/// differential tests trap into `signals::handler` and execute the original
/// guest `x86-64` instructions via `emu_run_with_xmm`.
#[cfg_attr(target_arch = "x86_64", allow(dead_code))]
pub const ORIG_TRAMPOLINE_TAG: usize = 1usize << 46;

static TRAMPOLINES: Mutex<Option<HashMap<u64, usize>>> = Mutex::new(None);

/// Counted hooks: (original VA, name, kind, counter).
pub(super) static COUNTERS: Mutex<Vec<(u64, String, &'static str, &'static AtomicU64)>> =
    Mutex::new(Vec::new());

/// Address of the trampoline that executes the original function at `va`.
pub fn trampoline_for(va: u64) -> Option<usize> {
    TRAMPOLINES.lock().unwrap().as_ref()?.get(&va).copied()
}

pub(super) fn record_trampoline(va: u64, tramp: usize) {
    TRAMPOLINES.lock().unwrap().get_or_insert_with(HashMap::new).insert(va, tramp);
}

pub struct Patch {
    #[allow(dead_code)]
    pub at: usize,
    #[allow(dead_code)]
    pub len: usize,
    pub trampoline: usize,
    /// Call counter (counted hooks only).
    pub counter: Option<&'static AtomicU64>,
}

/// Where a hook sends execution.
#[derive(Clone, Copy)]
pub enum Dest {
    /// A fixed address (the Rust replacement).
    Fixed(usize),
    /// The hook's own trampoline, i.e. the original code (probes).
    Trampoline,
}

/// Decode instructions at `addr` until at least `JMP_LEN` bytes are covered.
fn displaced(addr: usize, ip: u64) -> Result<Vec<Instruction>> {
    let bytes = unsafe { std::slice::from_raw_parts(addr as *const u8, 32) };
    let mut d = Decoder::with_ip(64, bytes, ip, DecoderOptions::NONE);
    let mut out = Vec::new();
    let mut len = 0;
    while len < JMP_LEN {
        let ins = d.decode();
        if ins.is_invalid() {
            bail!("invalid instruction at 0x{:x}", ins.ip());
        }
        match ins.flow_control() {
            FlowControl::Return
            | FlowControl::UnconditionalBranch
            | FlowControl::IndirectBranch
            | FlowControl::Interrupt
                if len + ins.len() < JMP_LEN =>
            {
                let end = len + ins.len();
                if matches!(
                    ins.flow_control(),
                    FlowControl::Return
                        | FlowControl::UnconditionalBranch
                        | FlowControl::IndirectBranch
                ) && bytes[end..JMP_LEN].iter().all(|&b| b == 0xCC)
                {
                    out.push(ins);
                    return Ok(out);
                }
                if matches!(ins.flow_control(), FlowControl::Interrupt)
                    && len > 0
                    && bytes[len..JMP_LEN].iter().all(|&b| b == 0xCC)
                {
                    return Ok(out);
                }
                bail!("function too short to patch ({ins} at +{len})")
            }
            _ => {}
        }
        len += ins.len();
        out.push(ins);
    }
    Ok(out)
}

/// Infer the end address of a leaf function without `.pdata` by decoding up to
/// 64 bytes until a return/unconditional-branch that is not jumped over by an
/// earlier forward branch, or `int3` padding.
fn infer_leaf_end(func: usize) -> usize {
    let bytes = unsafe { std::slice::from_raw_parts(func as *const u8, 64) };
    let mut d = Decoder::with_ip(64, bytes, func as u64, DecoderOptions::NONE);
    let mut max_target = func;
    let mut end = func;
    for ins in d.iter() {
        if ins.is_invalid() || matches!(ins.flow_control(), FlowControl::Interrupt) {
            break;
        }
        let next_ip = (ins.ip() as usize) + ins.len();
        end = next_ip;
        if matches!(
            ins.flow_control(),
            FlowControl::ConditionalBranch | FlowControl::UnconditionalBranch
        ) {
            let t = ins.near_branch_target() as usize;
            if t > max_target && t <= func + 64 {
                max_target = t;
            }
        }
        if matches!(
            ins.flow_control(),
            FlowControl::Return | FlowControl::UnconditionalBranch | FlowControl::IndirectBranch
        ) && next_ip >= max_target
        {
            break;
        }
    }
    end
}

/// Refuse if any branch in `[scan_start, func_end)` targets `(func, patch_end)`.
fn check_no_branch_into(
    func: usize,
    scan_start: usize,
    func_end: usize,
    patch_end: usize,
) -> Result<()> {
    if scan_start >= func_end {
        return Ok(());
    }
    let n = func_end - scan_start;
    let bytes = unsafe { std::slice::from_raw_parts(scan_start as *const u8, n) };
    let mut d = Decoder::with_ip(64, bytes, scan_start as u64, DecoderOptions::NONE);
    for ins in d.iter() {
        if matches!(
            ins.flow_control(),
            FlowControl::ConditionalBranch | FlowControl::UnconditionalBranch
        ) {
            let t = ins.near_branch_target() as usize;
            if t > func && t < patch_end {
                bail!("branch at 0x{:x} targets displaced bytes 0x{t:x}", ins.ip());
            }
        }
    }
    Ok(())
}

/// `lock inc qword [rip+disp32]; jmp [rip]; dq target` (22 bytes) placed at
/// `at`. The counter must live in a separate non-executable page
/// ([`thunks::alloc_near_data`]): under Rosetta a store into a page that
/// holds translated code is handled as self-modifying code (exception +
/// re-translation on every call).
fn counting_thunk(target: usize, at: usize, counter: usize) -> Result<[u8; 22]> {
    let mut b = [0u8; 22];
    b[..4].copy_from_slice(&[0xF0, 0x48, 0xFF, 0x05]);
    let disp = i32::try_from(counter as i64 - (at as i64 + 8))
        .map_err(|_| anyhow::anyhow!("counter out of rel32 range"))?;
    b[4..8].copy_from_slice(&disp.to_le_bytes());
    b[8..22].copy_from_slice(&thunks::abs_jmp(target));
    Ok(b)
}

/// Hook `addr` (actual address) so it jumps to `target`. `func_end` (from
/// .pdata) enables the branch-into check.
/// `restore_prot` is the host protection to put back on the patched pages.
pub fn install(
    addr: usize,
    func_end: Option<usize>,
    target: usize,
    restore_prot: i32,
) -> Result<Patch> {
    install_ex(addr, func_end, Dest::Fixed(target), false, restore_prot)
}

/// [`install`] with a choice of destination and an optional call counter.
pub fn install_ex(
    addr: usize,
    func_end: Option<usize>,
    dest: Dest,
    count: bool,
    restore_prot: i32,
) -> Result<Patch> {
    let mut ins = displaced(addr, addr as u64)?;
    let entry_len: usize = ins.iter().map(|i| i.len()).sum();
    let mut tramp_len = entry_len;
    let scan_end = func_end.unwrap_or_else(|| infer_leaf_end(addr));
    if scan_end > addr + tramp_len {
        let max_extend = scan_end.min(addr + 48);
        let rem_bytes = unsafe {
            std::slice::from_raw_parts((addr + tramp_len) as *const u8, max_extend - (addr + tramp_len))
        };
        let mut d = Decoder::with_ip(64, rem_bytes, (addr + tramp_len) as u64, DecoderOptions::NONE);
        let mut extra = Vec::new();
        for inst in d.iter() {
            if inst.is_invalid() || matches!(inst.flow_control(), FlowControl::Interrupt) {
                break;
            }
            extra.push(inst);
            if matches!(
                inst.flow_control(),
                FlowControl::ConditionalBranch | FlowControl::UnconditionalBranch
            ) {
                let t = inst.near_branch_target() as usize;
                if t > addr && t < addr + entry_len {
                    ins.extend(extra.drain(..));
                    tramp_len = ins.iter().map(|i| i.len()).sum();
                }
            }
        }
    }
    if scan_end > addr + tramp_len {
        check_no_branch_into(addr, addr + tramp_len, scan_end, addr + entry_len)?;
    }
    // Trampoline: re-encoded displaced instructions + jmp back.
    let tramp = thunks::alloc_near(addr, 64 + 14)
        .ok_or_else(|| anyhow::anyhow!("no memory near 0x{addr:x}"))?;
    let block = InstructionBlock::new(&ins, tramp as u64);
    let enc = BlockEncoder::encode(64, block, BlockEncoderOptions::NONE)
        .map_err(|e| anyhow::anyhow!("re-encode: {e}"))?;
    if enc.code_buffer.len() > 64 {
        bail!("trampoline too large");
    }
    let mut tcode = enc.code_buffer.clone();
    tcode.extend_from_slice(&thunks::abs_jmp(addr + tramp_len));
    unsafe { thunks::write_code(tramp, &tcode) };
    let target = match dest {
        Dest::Fixed(t) => t,
        Dest::Trampoline => tramp,
    };
    // Near thunk to the destination (optionally counting calls).
    let (near, counter) = if count {
        let near = thunks::alloc_near(addr, 22)
            .ok_or_else(|| anyhow::anyhow!("no memory near 0x{addr:x}"))?;
        let ctr = thunks::alloc_near_data(near, 8)
            .ok_or_else(|| anyhow::anyhow!("no data memory near 0x{near:x}"))?;
        unsafe { thunks::write_code(near, &counting_thunk(target, near, ctr)?) };
        // SAFETY: 16-byte aligned, zeroed, RW (non-exec), never freed.
        (near, Some(unsafe { &*(ctr as *const AtomicU64) }))
    } else {
        let near = thunks::alloc_near(addr, 14)
            .ok_or_else(|| anyhow::anyhow!("no memory near 0x{addr:x}"))?;
        unsafe { thunks::write_code(near, &thunks::abs_jmp(target)) };
        (near, None)
    };
    let rel = near as i64 - (addr as i64 + JMP_LEN as i64);
    if rel < i32::MIN as i64 || rel > i32::MAX as i64 {
        bail!("thunk out of rel32 range");
    }
    let patch_len = entry_len.max(JMP_LEN);
    let mut patch = vec![0xCCu8; patch_len];
    patch[0] = 0xE9;
    patch[1..5].copy_from_slice(&(rel as i32).to_le_bytes());
    unsafe {
        #[cfg(target_arch = "x86_64")]
        {
            let page = addr & !0xFFF;
            let span = ((addr + patch_len + 0xFFF) & !0xFFF) - page;
            let rwx = libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC;
            if restore_prot != rwx && libc::mprotect(page as *mut libc::c_void, span, rwx) != 0 {
                bail!("mprotect failed at 0x{page:x}");
            }
            std::ptr::copy_nonoverlapping(patch.as_ptr(), addr as *mut u8, patch_len);
            if restore_prot != rwx {
                libc::mprotect(page as *mut libc::c_void, span, restore_prot);
            }
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            let _ = restore_prot;
            std::ptr::copy_nonoverlapping(patch.as_ptr(), addr as *mut u8, patch_len);
        }
    }
    Ok(Patch { at: addr, len: patch_len, trampoline: tramp, counter })
}

pub(super) fn va_to_addr_with(exe_base: usize, preferred_base: u64, va: u64) -> usize {
    (exe_base as i64 + (va as i64 - preferred_base as i64)) as usize
}

fn va_to_addr(exe_base: usize, va: u64) -> usize {
    va_to_addr_with(exe_base, PREFERRED_EXE_BASE.load(Ordering::Relaxed), va)
}

/// `.pdata` lookup for an actual address: `Some((begin, end))` of the
/// containing RUNTIME_FUNCTION.
pub(super) type FnLookup<'a> = &'a dyn Fn(usize) -> Option<(usize, usize)>;

pub(super) fn loader_lookup(addr: usize) -> Option<(usize, usize)> {
    let l = crate::loader::get();
    let (base, f) = win32_rt::LoaderOps::lookup_function_entry(l, addr as u64)?;
    let f = unsafe { &*f };
    Some((base as usize + f.begin_rva as usize, base as usize + f.end_rva as usize))
}

/// Hook `va` (preferred-base VA) with checks against `.pdata`.
pub(super) fn hook_va(
    exe_base: usize,
    va: u64,
    name: &str,
    dest: Dest,
    count: bool,
    lookup: FnLookup,
    restore_prot: i32,
) -> Result<Patch> {
    let addr = va_to_addr(exe_base, va);
    let func_end = match lookup(addr) {
        Some((begin, end)) => {
            if begin != addr {
                bail!("{name} (0x{va:x}) is not the start of a .pdata function");
            }
            Some(end)
        }
        None => None,
    };
    install_ex(addr, func_end, dest, count, restore_prot)
        .map_err(|e| anyhow::anyhow!("hook {name} @0x{va:x}: {e}"))
}

pub(super) static EXE_BASE: AtomicUsize = AtomicUsize::new(0);
pub(super) static PATCH_PROT: AtomicUsize = AtomicUsize::new(0);
/// Live temporary patches: (address, original 5 bytes).
static TEMP_PATCHES: Mutex<Vec<Option<(usize, [u8; JMP_LEN])>>> = Mutex::new(Vec::new());

unsafe fn write_patch_bytes(addr: usize, bytes: &[u8]) -> Result<(), String> {
    #[cfg(target_arch = "x86_64")]
    {
        let page = addr & !0xFFF;
        let span = ((addr + bytes.len() + 0xFFF) & !0xFFF) - page;
        let rwx = libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC;
        if libc::mprotect(page as *mut libc::c_void, span, rwx) != 0 {
            return Err(format!("mprotect failed at 0x{page:x}"));
        }
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), addr as *mut u8, bytes.len());
        libc::mprotect(page as *mut libc::c_void, span, PATCH_PROT.load(Ordering::Relaxed) as i32);
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = PATCH_PROT.load(Ordering::Relaxed);
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), addr as *mut u8, bytes.len());
    }
    Ok(())
}

/// Differential-test mock: redirect the exe function at `va` to `target`
/// (a plain `jmp`, no trampoline) until [`temp_unpatch`]. Works whether or not
/// `va` is itself hooked (the first 5 bytes are saved and restored).
pub(super) fn temp_patch(va: u64, target: usize) -> Result<usize, String> {
    let base = EXE_BASE.load(Ordering::Relaxed);
    if base == 0 {
        return Err("exe not mapped".into());
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let prev = x86_runtime::exe::lookup_native_va(va).unwrap_or(0);
        x86_runtime::exe::register_native_va(va, target);
        let mut prev_bytes = [0u8; JMP_LEN];
        prev_bytes.copy_from_slice(&(prev as u64).to_le_bytes()[..JMP_LEN]);
        let mut t = TEMP_PATCHES.lock().unwrap();
        t.push(Some((va as usize, prev_bytes)));
        return Ok(t.len() - 1);
    }
    #[cfg(target_arch = "x86_64")]
    {
        let addr = va_to_addr(base, va);
        let near = thunks::alloc_near(addr, 14).ok_or("no memory near target")?;
        unsafe { thunks::write_code(near, &thunks::abs_jmp(target)) };
        let rel = i32::try_from(near as i64 - (addr as i64 + JMP_LEN as i64))
            .map_err(|_| "thunk out of range")?;
        let mut saved = [0u8; JMP_LEN];
        unsafe { std::ptr::copy_nonoverlapping(addr as *const u8, saved.as_mut_ptr(), JMP_LEN) };
        let mut patch = [0xE9u8; JMP_LEN];
        patch[1..].copy_from_slice(&rel.to_le_bytes());
        unsafe { write_patch_bytes(addr, &patch)? };
        let mut t = TEMP_PATCHES.lock().unwrap();
        t.push(Some((addr, saved)));
        Ok(t.len() - 1)
    }
}

pub(super) fn temp_unpatch(token: usize) {
    let e = TEMP_PATCHES.lock().unwrap().get_mut(token).and_then(Option::take);
    if let Some((addr, saved)) = e {
        #[cfg(not(target_arch = "x86_64"))]
        {
            let _ = write_patch_bytes;
            let mut b8 = [0u8; 8];
            b8[..JMP_LEN].copy_from_slice(&saved);
            let prev = u64::from_le_bytes(b8) as usize;
            if prev != 0 {
                x86_runtime::exe::register_native_va(addr as u64, prev);
            } else {
                x86_runtime::exe::unregister_native_va(addr as u64);
            }
            return;
        }
        #[cfg(target_arch = "x86_64")]
        if let Err(e) = unsafe { write_patch_bytes(addr, &saved) } {
            log::error!("temp_unpatch 0x{addr:x}: {e}");
        }
    }
}
