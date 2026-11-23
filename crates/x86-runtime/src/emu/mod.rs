//! Portable `x86-64` instruction execution engine (`iced-x86` decoder + Win64 ABI bridge)
//! for calling unlifted PE functions (`DllMain`, CRT init, third-party DLLs, or fallback
//! `AlanWake2.exe` VAs) on non-`x86_64` hosts (`aarch64-apple-darwin`) and from `tpl_lifted_cfg`.
//!
//! Whenever a `call` or `jmp` targets an address registered in [`crate::exe::lookup_native_va`]
//! (or any address inside the host Mach-O `__TEXT` segment), execution dispatches directly
//! to the compiled native `aarch64` Rust function (`aw2-game` replacement or `win32-rt`
//! builtin export). Otherwise, it decodes and executes the guest `x86-64` instructions in
//! place against guest memory and the thread's TEB (`gs:[offset]`).

use crate::exe::{base, lookup_native_va, rd, teb_self, wr};
use iced_x86::{
    ConditionCode, Decoder, DecoderOptions, EncodingKind, FlowControl, Instruction, Mnemonic, OpKind,
    Register,
};

pub mod eh;
pub mod exec_mem;
mod fast;
mod mem;
pub mod regfile;
pub mod regfile_ext;
mod simd_kernels;
mod simd_mov;
mod simd_fp;
#[cfg(target_arch = "aarch64")]
mod jit;
#[cfg(not(target_arch = "aarch64"))]
#[path = "jit_stub.rs"]
mod jit;
mod jit_stats;
#[cfg(target_arch = "aarch64")]
pub(crate) mod aot;
#[cfg(target_arch = "aarch64")]
pub mod static_macho;
#[cfg(target_arch = "aarch64")]
pub mod verify;

/// Difftest hook (`dll_lift` fuzz children only; 0 = none): called on every guest `call` to
/// address 0, which the emulator turns into `rax = 0` and goes on (natively a fault), on a run
/// that ends at rip 0 (a `jmp`/`ret` to 0) and on a run started at 0 (`call_va*(0)`), so the
/// harness can tell runs that went on past one (see `dll_native::lift::diff`, "null_call").
pub static NULL_CALL_HOOK: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

#[cold]
#[inline(never)]
fn note_null_call() {
    let h = NULL_CALL_HOOK.load(std::sync::atomic::Ordering::Relaxed);
    if h != 0 {
        let f: fn() = unsafe { core::mem::transmute(h) };
        f();
    }
}
pub use mem::note_code_region;

/// AOT translation cache (`AW2_DLL_AOT`, `aot.rs`): PE image `name` was mapped, relocated
/// and import-bound at `[base, base + size)` (also feeds the `AW2_JIT_PROFILE` dump).
pub fn aot_module_mapped(name: &str, base: usize, size: usize) {
    #[cfg(target_arch = "aarch64")]
    aot::module_mapped(name, base, size);
    #[cfg(not(target_arch = "aarch64"))]
    let _ = (name, base, size);
}

/// Enable hardware `FEAT_LSE2` `SIGBUS` alignment-trap recovery in translated code
/// (`PORT_JIT_LSE2_TRAP`): called once by `port-loader::signals::install_handlers()` after
/// installing the `SIGBUS` handler that emulates 16-byte-boundary-crossing `ldapr`/`ldapur`/
/// `stlr`/`stlur` faults.
pub fn enable_lse2_trap() {
    #[cfg(target_arch = "aarch64")]
    jit::enable_lse2_trap();
}

/// Enable hardware MMU range checking in translated code (`PORT_JIT_HW_RANGE`): called once
/// by `port-loader::signals::install_handlers()`. Eliminates software `[0x10000, 2^47)`
/// range checks (`sub`/`lsr`/`cbnz`) by relying on Darwin's 4 GiB `__PAGEZERO` + 47-bit
/// user VA space + signal handler.
pub fn enable_hw_range() {
    #[cfg(target_arch = "aarch64")]
    jit::enable_hw_range();
}

/// Async-signal-safe, lock-free self-patching of a faulting unaligned `ldapr`/`ldapur`/
/// `stlr`/`stlur` inside an executing `MAP_JIT` AOT/JIT region to a 16-byte exact-TSO island.
/// Returns `true` if the site was patched (or already patched by another thread).
#[cfg(target_arch = "aarch64")]
pub unsafe fn patch_lse2_unaligned(pc: usize, insn: u32) -> bool {
    jit::patch_lse2_unaligned(pc, insn)
}

/// AOT translation cache activation point (loader: after the AW2 registry is applied, before
/// DllMains / difftests run): load the caches of the images mapped since the last call, or
/// build them and exit (`AW2_DLL_AOT_BUILD`).
pub fn aot_activate() {
    #[cfg(target_arch = "aarch64")]
    aot::activate();
}

/// Merge the `PORT_JIT_PROFILE` / `AW2_JIT_PROFILE` block profile recorded so far into the cache directory.
pub fn aot_profile_dump() {
    #[cfg(target_arch = "aarch64")]
    aot::profile_dump();
}

#[cfg(target_arch = "aarch64")]
pub use aot::{aot_cached_preferred_base, aot_prewarm_all_mapped, AotPrewarmReport};

/// Look up the compiled `image_base` of a matching dynamic `.aot` cache (no-op on non-aarch64 hosts).
#[cfg(not(target_arch = "aarch64"))]
pub fn aot_cached_preferred_base(_name: &str, _image_size: usize) -> Option<usize> {
    None
}

/// Outcome of profile-free offline `.aot` block-cache precompilation for one mapped PE module.
#[cfg(not(target_arch = "aarch64"))]
#[derive(Debug, Clone)]
pub struct AotPrewarmReport {
    pub module: String,
    pub roots: usize,
    pub blocks: usize,
    pub code_bytes: usize,
    pub cache_path: Option<std::path::PathBuf>,
    pub skipped_reason: Option<String>,
}

/// Profile-free offline `.aot` block-cache precompiler (no-op on non-aarch64 hosts).
#[cfg(not(target_arch = "aarch64"))]
pub fn aot_prewarm_all_mapped(_max_roots_per_module: usize) -> Vec<AotPrewarmReport> {
    Vec::new()
}

#[inline(always)]
pub fn set_incoming_rsp(rsp: u64, rsp_limit: u64) {
    crate::callctx::ctx().incoming_rsp.set((rsp, rsp_limit));
}

#[inline(always)]
pub fn take_incoming_rsp() -> (u64, u64) {
    crate::callctx::ctx().incoming_rsp.replace((0, 0))
}

/// The published caller window, left in place (shadow probes replay it in every run while
/// the real call still consumes it).
#[inline(always)]
pub fn peek_incoming_rsp() -> (u64, u64) {
    crate::callctx::ctx().incoming_rsp.get()
}

#[inline(always)]
pub fn set_outgoing_rsp(rsp: u64, rsp_limit: u64) {
    crate::callctx::ctx().outgoing_rsp.set((rsp, rsp_limit));
}

#[inline(always)]
pub fn take_outgoing_rsp() -> (u64, u64) {
    crate::callctx::ctx().outgoing_rsp.replace((0, 0))
}

#[inline(always)]
pub unsafe fn read_caller_extra_args(caller_rsp: u64, rsp_limit: u64) -> [u64; 8] {
    // The window is the caller's own outgoing-argument area (its lifted stack or a host
    // buffer), written by this thread just before the call: plain loads, no TSO ordering.
    // No caller rsp (no stack args published): 74% of gameplay calls (cvP,
    // `AW2_CALLVA_PROF`). Answered inline; this used to call the cold partial reader.
    if caller_rsp == 0 {
        return [0; 8];
    }
    let base = caller_rsp.wrapping_add(0x40);
    if base <= base.wrapping_add(0x40) && base.wrapping_add(0x40) <= rsp_limit {
        return core::ptr::read_unaligned(base as *const [u64; 8]);
    }
    read_caller_extra_args_partial(caller_rsp, rsp_limit)
}

#[cold]
#[inline(never)]
unsafe fn read_caller_extra_args_partial(caller_rsp: u64, rsp_limit: u64) -> [u64; 8] {
    let mut out = [0u64; 8];
    if caller_rsp != 0 {
        for i in 0..8 {
            let addr = caller_rsp.wrapping_add(0x40 + (i as u64) * 8);
            if addr.wrapping_add(8) <= rsp_limit {
                out[i] = core::ptr::read_unaligned(addr as *const u64);
            }
        }
    }
    out
}

/// Check whether `va_or_addr` is a registered native host function pointer in `VA_DISPATCH`
/// (including 6-byte `FF 25 <disp32>` IAT jump thunks in the primary PE module).
#[inline]
pub fn resolve_host_callable(va_or_addr: u64) -> Option<usize> {
    // Special-ABI CRT helpers (`__security_check_cookie` preserves RAX; `__chkstk` takes size
    // in RAX; `_guard_dispatch_icall` takes target in RAX) must never be dispatched as standard
    // Win64 host functions. These are AW2 VAs: only consulted when the AW2 registry is active.
    if matches!(
        va_or_addr,
        0x1_4183_53d0 | 0x1_4183_5480 | 0x1_4254_7700 | 0x1_4254_7720
    ) && crate::exe::aw2_registry_active()
    {
        return None;
    }
    if crate::dll_native::is_dll_chkstk(va_or_addr)
        || crate::dll_native::is_dll_stack_pos(va_or_addr)
    {
        return None;
    }
    if let Some(f) = lookup_native_va(va_or_addr) {
        return Some(f);
    }
    let pref = crate::exe::preferred_base();
    let sz = crate::exe::image_size();
    let b = base() as u64;
    if b != 0
        && ((pref != 0 && (pref..pref.wrapping_add(sz)).contains(&va_or_addr))
            || (b..b.wrapping_add(sz)).contains(&va_or_addr))
    {
        let m = to_mapped_addr(va_or_addr);
        let img_lo = b as usize;
        let img_hi = b.wrapping_add(sz) as usize;
        if m >= img_lo
            && m.saturating_add(6) <= img_hi
            && win32_rt::kernel32::memory::is_speculative_read_safe(m, 6)
        {
            unsafe {
                if rd::<u16>(m) == 0x25ff {
                    let disp = rd::<i32>(m + 2) as i64;
                    let slot = (m as i64).wrapping_add(6).wrapping_add(disp) as usize;
                    if slot >= img_lo
                        && slot.saturating_add(8) <= img_hi
                        && win32_rt::kernel32::memory::is_speculative_read_safe(slot, 8)
                    {
                        let target = rd::<u64>(slot);
                        return lookup_native_va(target);
                    }
                }
            }
        }
    }
    None
}

// ---------------------------------------------------------------------------------------------
// AW2 exe guest entries (exe x86 execution -> 0).
//
// Every `AlanWake2.exe` function is a native replacement in `VA_DISPATCH`, except:
// - the 270 6-byte import thunks `jmp qword ptr [rip+disp32]` (`FF 25`). They are kept out of the
//   VA table on purpose (see `hooks.rs` `is_unverified_iat_thunk`) so that
//   `resolve_host_callable` dispatches host-builtin imports directly;
// - the 4 special-ABI CRT helpers (`__security_check_cookie`, `__chkstk`,
//   `_guard_dispatch_icall` x2), which lifted call sites lower inline.
// Guest code still reaches the thunks whose slot is a *guest* PE DLL function (msvcp140,
// vcruntime140, cohtml, ...). Lifted code does `call_va8(thunk)` -> `emu_run_full(entry = thunk)`,
// and cohtml calls exe vtable slots that are thunks back into cohtml. Before this change those
// paths translated/interpreted the exe thunk block. Now the run loop and `jit_host_xfer` route such
// entries natively (`ExeRoute`): the thunk becomes a direct transfer to its slot target, and the
// special helpers run as Rust with the original's register effects. Not routed:
// - a non-thunk `entry_ip`: a difftest oracle runs the original exe function as x86 on purpose;
// - any other exe ip (e.g. `AW2_JIT_THUNK_ELIDE`). `AW2_EXE_GUEST_LOG=1` logs those (first 64
//   distinct ips) to confirm that gameplay executes no exe x86.
// ---------------------------------------------------------------------------------------------

/// `_guard_dispatch_icall` (0x142547700): `jmp rax`.
const AW2_GUARD_DISPATCH: u64 = 0x1_4254_7700;
/// `__chkstk` (0x141835480).
const AW2_CHKSTK: u64 = 0x1_4183_5480;
/// `__security_check_cookie` (0x1418353d0).
const AW2_CHECK_COOKIE: u64 = 0x1_4183_53d0;
/// `__security_cookie` (the qword `__security_check_cookie` compares against).
const AW2_SECURITY_COOKIE: u64 = 0x1_4382_2fc0;
/// `__report_gsfailure` (0x141835cc8): `__security_check_cookie`'s failure tail jump.
const AW2_REPORT_GSFAILURE: u64 = 0x1_4183_5cc8;

/// How [`exe_route`] handles a guest block start inside the AW2 exe.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ExeRoute {
    /// Not routed: execute the guest instructions normally.
    None,
    /// Transfer to this target as a tail jump (stack unchanged): import thunk slot target, or
    /// `rax` for `_guard_dispatch_icall`.
    Jmp(u64),
    /// Return to the caller (pop the return address): `__chkstk` (stacks are committed,
    /// see `apply_fx`) and the `__security_check_cookie` success path.
    Ret,
}

/// Gate for [`exe_route`]: `(preferred base, mapped base, image size)` of the AW2 exe, or a zero
/// span when the AW2 registry isn't applied (other exes, unit tests). One run computes it once.
#[inline]
fn exe_route_gate() -> (u64, u64, u64) {
    if !crate::exe::aw2_registry_active() {
        return (0, 0, 0);
    }
    (crate::exe::preferred_base(), base() as u64, crate::exe::image_size())
}

/// `ip` lies in the AW2 exe image (`gate` from [`exe_route_gate`]).
#[inline(always)]
fn in_exe_gate(gate: (u64, u64, u64), ip: u64) -> bool {
    ip.wrapping_sub(gate.0) < gate.2 || ip.wrapping_sub(gate.1) < gate.2
}

/// Slot target of the exe import thunk `jmp qword ptr [rip+disp32]` at `ip`, if `ip` is one.
#[inline]
unsafe fn exe_import_thunk_target(ip: u64) -> Option<u64> {
    let m = to_mapped_addr(ip);
    let img_lo = base();
    let img_hi = img_lo.wrapping_add(crate::exe::image_size() as usize);
    if m < img_lo || m.saturating_add(6) > img_hi || rd::<u16>(m) != 0x25ff {
        return None;
    }
    let slot = (m as i64).wrapping_add(6).wrapping_add(rd::<i32>(m + 2) as i64) as usize;
    if slot < img_lo || slot.saturating_add(8) > img_hi {
        return None;
    }
    Some(rd::<u64>(slot))
}

std::thread_local! {
    /// Difftest override (`exe_route_diff`): route nothing on this thread, so the exe's x86
    /// runs (the oracle side).
    pub static EXE_ROUTE_OFF: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
    /// Exe entries routed natively on this thread (read by the difftests).
    pub static EXE_ROUTED: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
}

#[inline]
fn exe_route_off() -> bool {
    EXE_ROUTE_OFF.with(|c| c.get())
}

#[inline]
fn exe_routed_inc() {
    EXE_ROUTED.with(|c| c.set(c.get().wrapping_add(1)));
}

/// Route a guest block start `ip` inside the AW2 exe (see the section comment). Applies the
/// special helpers' register effects to `cpu`; the caller performs the transfer.
#[cold]
#[inline(never)]
unsafe fn exe_route(cpu: &mut Cpu, ip: u64, entry_ip: u64) -> ExeRoute {
    if exe_route_off() {
        return ExeRoute::None;
    }
    if let Some(t) = exe_import_thunk_target(ip) {
        return ExeRoute::Jmp(t);
    }
    if ip != entry_ip {
        match ip {
            AW2_GUARD_DISPATCH => return ExeRoute::Jmp(cpu.gpr[0]),
            AW2_CHKSTK => return ExeRoute::Ret,
            AW2_CHECK_COOKIE => return check_cookie(cpu),
            _ => {}
        }
    }
    exe_guest_note(ip, entry_ip);
    ExeRoute::None
}

/// `__security_check_cookie` 0x1418353d0, native:
/// ```text
/// cmp rcx, [__security_cookie] ; jne fail
/// rol rcx, 16 ; test cx, 0xffff ; jne restore
/// ret
/// restore: ror rcx, 16
/// fail:    jmp __report_gsfailure      ; rcx = the checked value
/// ```
/// The success path leaves `rcx` rotated and the `test` flags (ZF=1, SF=CF=OF=0, PF from a
/// zero low byte). The failure path keeps the original `rcx` and jumps to
/// `__report_gsfailure` (a native replacement if registered, else guest). The `cmp`/`test`
/// flags of that path aren't reproduced: `__report_gsfailure` never returns and doesn't read
/// them.
unsafe fn check_cookie(cpu: &mut Cpu) -> ExeRoute {
    let rcx = cpu.gpr[1];
    let cookie = rd::<u64>(to_mapped_addr(AW2_SECURITY_COOKIE));
    let rot = rcx.rotate_left(16);
    if rcx == cookie && rot as u16 == 0 {
        cpu.gpr[1] = rot;
        cpu.last_bits[1] = 64;
        cpu.zf = true;
        cpu.sf = false;
        cpu.cf = false;
        cpu.of = false;
        cpu.pfv = 0;
        ExeRoute::Ret
    } else {
        ExeRoute::Jmp(AW2_REPORT_GSFAILURE)
    }
}

/// `AW2_EXE_GUEST_LOG=1`: warn once per distinct exe ip that guest code executes (first 64).
#[cold]
fn exe_guest_note(ip: u64, entry_ip: u64) {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if !*ON.get_or_init(|| std::env::var("AW2_EXE_GUEST_LOG").is_ok_and(|v| v != "0")) {
        return;
    }
    static SEEN: std::sync::Mutex<Vec<u64>> = std::sync::Mutex::new(Vec::new());
    if let Ok(mut s) = SEEN.lock() {
        if s.len() < 64 && !s.contains(&ip) {
            s.push(ip);
            log::warn!("emu: guest x86 executing in the exe at {ip:#x} (run entry {entry_ip:#x})");
        }
    }
}

/// Apply a non-`None` [`ExeRoute`] in the run loop: `Some(result)` when the run is finished
/// (returned to the sentinel), `None` to continue at `cpu.rip`.
unsafe fn apply_exe_route(
    cpu: &mut Cpu,
    route: ExeRoute,
    entry_ip: u64,
    blocks_ptr: *mut BlockCache,
    sp_ptr: *mut u64,
    prof: Option<&'static crate::emu_prof::ThreadProf>,
) -> Option<(u64, u128)> {
    if route != ExeRoute::None {
        exe_routed_inc();
    }
    match route {
        ExeRoute::None => None,
        // A host target becomes a host tail call, a guest target a plain jump.
        ExeRoute::Jmp(t) => apply_fx(cpu, fast::Fx::Jmp(t), entry_ip, blocks_ptr, sp_ptr, prof),
        ExeRoute::Ret => {
            let ret_addr = cpu.read_mem(cpu.gpr[4], 64);
            cpu.gpr[4] = cpu.gpr[4].wrapping_add(8);
            if ret_addr == RET_SENTINEL || ret_addr == 0 {
                return Some((cpu.abi_gpr(0), cpu.xmm[0]));
            }
            cpu.rip = ret_addr;
            None
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Mapped-DLL import stubs (g23).
//
// cohtml / RenoirCore / v8 call their imports through 6-byte stubs `jmp qword ptr [rip+slot]`
// (`dll_native::iat_stub_slot`: slot inside the image's IAT directory). Lifted code reaches
// them with `call_va8(stub)`; [`resolve_dispatch_in`] sees through them ([`DF_IAT`], the slot
// read at call time). A stub the emulator still reaches (guest code calling it, or a
// `call_va8` whose slot target is guest code) is routed here at its block start: the
// transfer goes to the slot target read *now*, exactly as the `jmp` would (host target: host
// tail call, guest target: plain jump). Refused (the guest executes the stub), each named:
// - `slot_outside_iat`: the slot is outside the IAT directory (delay-load / data slot);
// - `slot_unbound`: the slot holds 0;
// - `no_iat_dir`: the image has no IAT directory (logged once by `dll_native::install`; its
//   span is outside the route gate).
// `AW2_DLL_ROUTE=0` turns all of it off. `AW2_DLL_ROUTE_LOG=1` logs each stub's outcome
// (`dispatched` / `routed` / the refusal; first 256 distinct).
// ---------------------------------------------------------------------------------------------

std::thread_local! {
    /// DLL import stubs routed natively on this thread (read by the tests).
    static DLL_ROUTED: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
}

/// DLL import stubs routed by the run loop on this thread so far.
#[cfg_attr(not(all(test, target_arch = "aarch64")), allow(dead_code))]
pub(crate) fn dll_routed() -> u64 {
    DLL_ROUTED.with(|c| c.get())
}

/// Gate for [`dll_stub_route`]: `(lowest base, span)` of the DLL images with an IAT
/// directory, or a zero span when routing is off. One run computes it once.
#[inline]
fn dll_route_gate() -> (u64, u64) {
    if !dll_route_enabled() {
        return (0, 0);
    }
    crate::dll_native::iat_gate()
}

/// Slot target of the import stub at block start `ip` (inside the [`dll_route_gate`] span),
/// read now; `None` when `ip` is not a routable stub. The 2-byte opcode check keeps the
/// per-block cost to one load for ordinary DLL code.
#[inline(always)]
unsafe fn dll_stub_route(ip: u64) -> Option<u64> {
    let Some((b, end)) = crate::dll_native::iat_image_bounds(ip) else {
        return None;
    };
    // SAFETY: `cur` is bounds-checked inside `[b, end)` before each unaligned load.
    let mut cur = ip as usize;
    let mut w = core::ptr::read_unaligned(cur as *const u16);
    if (w & 0xff) == 0xe9 {
        for _ in 0..4 {
            if cur + 5 > end {
                return None;
            }
            let rel = core::ptr::read_unaligned((cur + 1) as *const i32) as i64;
            let nxt = (cur as i64).wrapping_add(5).wrapping_add(rel);
            if nxt < b as i64 || (nxt as usize) + 2 > end || (nxt as usize) == cur {
                return None;
            }
            cur = nxt as usize;
            w = core::ptr::read_unaligned(cur as *const u16);
            if (w & 0xff) != 0xe9 {
                break;
            }
        }
    }
    if w != 0x25ff && w != 0xff48 {
        return None;
    }
    dll_stub_route_slow(ip)
}

#[cold]
#[inline(never)]
unsafe fn dll_stub_route_slow(ip: u64) -> Option<u64> {
    match crate::dll_native::iat_stub_slot_checked(ip) {
        Ok(Some(slot)) => {
            let t = core::ptr::read_volatile(slot as *const u64);
            if t == 0 {
                dll_route_note(ip, "left on the guest (slot_unbound)");
                return None;
            }
            DLL_ROUTED.with(|c| c.set(c.get().wrapping_add(1)));
            dll_route_note(ip, "routed");
            Some(t)
        }
        Ok(None) => None,
        Err(crate::dll_native::StubRefusal::SlotOutsideIat) => {
            dll_route_note(ip, "left on the guest (slot_outside_iat)");
            None
        }
    }
}

/// `va` is a mapped DLL's IAT import stub and DLL routing is enabled: never translate it into
/// a guest JIT/AOT block, and send static JIT transfers to it through `host_xfer`.
#[inline]
pub(super) fn is_routed_dll_stub(va: u64) -> bool {
    let g = dll_route_gate();
    va.wrapping_sub(g.0) < g.1 && crate::dll_native::iat_stub_slot(va).is_some()
}

/// Follow a mapped DLL's IAT import stub (and any chained IAT stub) to its current non-zero
/// slot target when DLL routing is enabled.
#[inline(always)]
unsafe fn resolve_dll_stub_target(mut target: u64) -> Option<u64> {
    let g = dll_route_gate();
    if target.wrapping_sub(g.0) >= g.1 {
        return None;
    }
    let mut resolved = None;
    for _ in 0..4 {
        if target.wrapping_sub(g.0) >= g.1 {
            break;
        }
        let Some(t) = dll_stub_route(target) else { break };
        target = t;
        resolved = Some(t);
    }
    resolved
}

std::thread_local! {
    /// Byte-detected DLL `__chkstk` calls routed natively on this thread (tests).
    static DLL_CHKSTK_ROUTED: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
    /// Test/oracle override: disable native DLL `__chkstk` routing on this thread.
    #[cfg(test)]
    static DLL_CHKSTK_OFF: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
}

/// DLL `__chkstk` invocations routed natively on this thread so far.
#[cfg_attr(not(all(test, target_arch = "aarch64")), allow(dead_code))]
pub(crate) fn dll_chkstk_routed() -> u64 {
    DLL_CHKSTK_ROUTED.with(|c| c.get())
}

#[inline]
fn dll_chkstk_enabled() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| !std::env::var("AW2_DLL_CHKSTK").is_ok_and(|v| v.trim() == "0"))
}

/// `va` is a byte-detected DLL `__chkstk` helper routed natively by the emulator.
#[inline(always)]
pub(super) fn is_routed_dll_chkstk(va: u64) -> bool {
    #[cfg(test)]
    if DLL_CHKSTK_OFF.with(|c| c.get()) {
        return false;
    }
    crate::dll_native::is_dll_chkstk(va) && dll_chkstk_enabled()
}

/// Active stack limit for a guest `__chkstk` probe with caller stack pointer `caller_rsp`:
/// uses the thread's emulator-stack floor (`stack_min + GUEST_FLOOR_MARGIN`, shared with
/// `lstack`) when `caller_rsp` is on the emulator stack (unless `TEB.StackBase`/`StackLimit`
/// were explicitly placed inside that stack), otherwise `TEB.StackLimit` (`gs:[0x10]`).
#[inline]
unsafe fn dll_chkstk_limit(caller_rsp: u64) -> u64 {
    let (floor, gtop) = crate::callctx::ctx().ls_gstack.get();
    let teb = teb_self() as u64;
    let (teb_hi, teb_lo) = if teb != 0 {
        (rd::<u64>((teb + 0x08) as usize), rd::<u64>((teb + 0x10) as usize))
    } else {
        (0, 0)
    };
    if floor != 0 {
        let alloc_base = floor.saturating_sub(GUEST_FLOOR_MARGIN + GUEST_STACK_MIN_OFF);
        let alloc_end = gtop.saturating_add(256);
        if (alloc_base..=alloc_end).contains(&caller_rsp) {
            if (alloc_base..=alloc_end).contains(&teb_hi) && teb_lo <= teb_hi {
                return teb_lo;
            }
            return floor;
        }
    }
    if teb != 0 && teb_lo <= teb_hi && caller_rsp <= teb_hi && teb_hi.wrapping_sub(caller_rsp) <= (64 << 20) {
        return teb_lo;
    }
    floor.max(0x10000)
}

/// Native MSVC x64 `__chkstk` semantics for `call chkstk` (`cpu.rip` = return address,
/// `cpu.gpr[4]` = caller `rsp` before pushing the return address).
#[inline]
unsafe fn dll_chkstk_call(cpu: &mut Cpu, sp_ptr: *mut u64) -> Option<(u64, u128)> {
    let caller_rsp = cpu.gpr[4];
    let ret_addr = cpu.rip;
    dll_chkstk_exec(cpu, sp_ptr, caller_rsp, ret_addr)
}

/// Native MSVC x64 `__chkstk` semantics for `jmp chkstk` or block-start entry (`[cpu.gpr[4]]`
/// holds the return address).
#[inline]
unsafe fn dll_chkstk_tail(cpu: &mut Cpu, sp_ptr: *mut u64) -> Option<(u64, u128)> {
    let entry_rsp = cpu.gpr[4];
    let ret_addr = cpu.read_mem(entry_rsp, 64);
    let caller_rsp = entry_rsp.wrapping_add(8);
    dll_chkstk_exec(cpu, sp_ptr, caller_rsp, ret_addr)
}

#[cold]
#[inline(never)]
unsafe fn dll_chkstk_exec(
    cpu: &mut Cpu,
    sp_ptr: *mut u64,
    caller_rsp: u64,
    ret_addr: u64,
) -> Option<(u64, u128)> {
    DLL_CHKSTK_ROUTED.with(|c| c.set(c.get().wrapping_add(1)));
    let rax = cpu.gpr[0];
    let r10 = caller_rsp.checked_sub(rax).unwrap_or(0);
    let limit = dll_chkstk_limit(caller_rsp);
    if r10 >= limit {
        // Exact register/flag state after `sub rsp, 0x10; ...; add rsp, 0x10; ret`:
        // all 16 GPRs (with `rsp = caller_rsp`) and all 16 XMMs preserved, flags from
        // `add rsp, 0x10` restoring `entry_rsp = caller_rsp - 8`.
        let entry_rsp = caller_rsp.wrapping_sub(8);
        cpu.gpr[4] = caller_rsp;
        cpu.last_bits[4] = 64;
        cpu.update_flags_add(entry_rsp.wrapping_sub(0x10), 0x10, 64);
        if ret_addr == RET_SENTINEL || ret_addr == 0 {
            return Some((cpu.abi_gpr(0), cpu.xmm[0]));
        }
        cpu.rip = ret_addr;
        return None;
    }
    // Stack overflow (`r10 < limit`): raise noncontinuable `STATUS_STACK_OVERFLOW` from the
    // caller's call site on the EH chain, as the guard-page touch in `chkstk.asm` would.
    cpu.gpr[4] = caller_rsp;
    cpu.last_bits[4] = 64;
    cpu.rip = ret_addr;
    if (0x1_0008..0x8000_0000_0000).contains(&caller_rsp) {
        wr::<u64>((caller_rsp - 8) as usize, ret_addr);
    }
    let saved_sp = *sp_ptr;
    *sp_ptr = (caller_rsp.wrapping_sub(256)) & !0xFu64;
    let raise = win32_rt::unwind::RaiseException as *const () as usize;
    let lsa = crate::lstack::anchor_enter(crate::callctx::ctx(), *sp_ptr);
    let _ = eh::callout(cpu, raise, *sp_ptr, move || {
        win32_rt::unwind::raise_exception_raw(
            crate::lstack::STATUS_STACK_OVERFLOW as u64,
            win32_rt::unwind::EXCEPTION_NONCONTINUABLE as u64,
            0,
            0,
        );
        (0, 0)
    });
    crate::lstack::anchor_leave(crate::callctx::ctx(), lsa);
    *sp_ptr = saved_sp;
    if cpu.rip == RET_SENTINEL || cpu.rip == 0 {
        return Some((cpu.abi_gpr(0), cpu.xmm[0]));
    }
    None
}

std::thread_local! {
    /// Byte-detected DLL `lea rax, [rsp]; ret` (`stack_pos`) calls routed natively on this thread.
    static DLL_STACK_POS_ROUTED: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
    /// Test/oracle override: disable native DLL `stack_pos` routing on this thread.
    #[cfg(test)]
    static DLL_STACK_POS_OFF: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
}

/// DLL `stack_pos` invocations routed natively on this thread so far.
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn dll_stack_pos_routed() -> u64 {
    DLL_STACK_POS_ROUTED.with(|c| c.get())
}

#[inline]
fn dll_stack_pos_enabled() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| {
        let on = !std::env::var("AW2_DLL_STACK_POS").is_ok_and(|v| v.trim() == "0");
        if !on {
            log::info!("emu: DLL stack_pos routing off (AW2_DLL_STACK_POS=0)");
        }
        on
    })
}

/// `va` is a byte-detected DLL `lea rax, [rsp]; ret` helper (or standalone `E9` thunk to one)
/// routed natively in 0 guest instructions.
#[inline(always)]
pub(super) fn is_routed_dll_stack_pos(va: u64) -> bool {
    #[cfg(test)]
    if DLL_STACK_POS_OFF.with(|c| c.get()) {
        return false;
    }
    crate::dll_native::is_dll_stack_pos(va) && dll_stack_pos_enabled()
}

/// Host `call_va8` fast path for [`is_routed_dll_stack_pos`]: returns the exact `sp_top` guest
/// stack coordinate that [`emu_run_region`] would establish (`state.sp - lifted_depth - 328`
/// aligned to `16n + 8`) without entering the interpreter loop.
#[cfg_attr(target_arch = "x86_64", allow(dead_code))]
#[inline]
unsafe fn dll_stack_pos_va8(c: &crate::callctx::CallCtx) -> u64 {
    let lifted = crate::lstack::lifted_depth(c) as u64;
    let (top, overflow) = TLS_EMU.with(|cell| {
        let opt = &mut *cell.get();
        let state = opt.get_or_insert_with(ThreadEmuState::new);
        if state.sp < state.stack_min || state.sp > state.stack_top {
            state.sp = state.stack_top;
        }
        let start = state.sp.saturating_sub(lifted);
        let overflow = lifted != 0 && start < state.stack_min + GUEST_FLOOR_MARGIN;
        let top = (start.wrapping_sub(320) & !0xFu64).wrapping_sub(8);
        (top, overflow)
    });
    if overflow {
        crate::lstack::raise_stack_overflow(crate::lstack::Overflow::Region);
    }
    DLL_STACK_POS_ROUTED.with(|r| r.set(r.get().wrapping_add(1)));
    top
}

/// Native `call stack_pos` (`cpu.rip` = return address, `cpu.gpr[4]` = caller `rsp`): writes
/// `ret_addr` to `[caller_rsp - 8]`, sets `rax = caller_rsp - 8`, `rsp = caller_rsp`, and
/// preserves all other GPRs, XMMs, and `RFLAGS`.
#[inline]
unsafe fn dll_stack_pos_call(cpu: &mut Cpu) -> Option<(u64, u128)> {
    let caller_rsp = cpu.gpr[4];
    let ret_addr = cpu.rip;
    let entry_rsp = caller_rsp.wrapping_sub(8);
    cpu.write_mem(entry_rsp, 64, ret_addr);
    if precise_faults() && FAULT_PENDING.with(|p| p.get().is_some()) {
        return None;
    }
    DLL_STACK_POS_ROUTED.with(|c| c.set(c.get().wrapping_add(1)));
    cpu.gpr[0] = entry_rsp;
    cpu.last_bits[0] = 64;
    cpu.gpr[4] = caller_rsp;
    if ret_addr == RET_SENTINEL || ret_addr == 0 {
        if ret_addr == 0 {
            note_null_call();
        }
        return Some((cpu.abi_gpr(0), cpu.xmm[0]));
    }
    cpu.rip = ret_addr;
    None
}

/// Native `jmp stack_pos` / block-start entry (`[cpu.gpr[4]]` holds `ret_addr`): reads
/// `ret_addr`, sets `rax = entry_rsp`, `rsp = entry_rsp + 8`, and preserves all other state.
#[inline]
unsafe fn dll_stack_pos_tail(cpu: &mut Cpu) -> Option<(u64, u128)> {
    let entry_rsp = cpu.gpr[4];
    let ret_addr = cpu.read_mem(entry_rsp, 64);
    if precise_faults() && FAULT_PENDING.with(|p| p.get().is_some()) {
        return None;
    }
    DLL_STACK_POS_ROUTED.with(|c| c.set(c.get().wrapping_add(1)));
    cpu.gpr[0] = entry_rsp;
    cpu.last_bits[0] = 64;
    cpu.gpr[4] = entry_rsp.wrapping_add(8);
    if ret_addr == RET_SENTINEL || ret_addr == 0 {
        if ret_addr == 0 {
            note_null_call();
        }
        return Some((cpu.abi_gpr(0), cpu.xmm[0]));
    }
    cpu.rip = ret_addr;
    None
}

/// Look up a routed `DF_REGFILE` `(body, blk)` at `va` (`None` when disabled).
#[inline(always)]
pub(super) fn lookup_routed_regfile(va: u64) -> Option<(regfile::RegfileFn, u32)> {
    #[cfg(test)]
    if regfile::test_off() {
        return None;
    }
    crate::dll_native::lift::lookup_regfile(va)
}

/// `va` is a registered and enabled `DF_REGFILE` entry.
#[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]
#[inline(always)]
pub(super) fn is_routed_regfile(va: u64) -> bool {
    lookup_routed_regfile(va).is_some()
}

/// `AW2_DLL_ROUTE_LOG=1`: log what happened to a DLL import stub (`routed` by the run loop,
/// `dispatched` = resolved through its slot by `resolve_dispatch` (`DF_IAT`), or the named
/// refusal), first 256 distinct (stub, outcome) pairs.
#[cold]
fn dll_route_note(ip: u64, what: &'static str) {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if !*ON.get_or_init(|| std::env::var("AW2_DLL_ROUTE_LOG").is_ok_and(|v| v != "0")) {
        return;
    }
    static SEEN: std::sync::Mutex<Vec<(u64, &'static str)>> = std::sync::Mutex::new(Vec::new());
    if let Ok(mut s) = SEEN.lock() {
        if s.len() < 256 && !s.contains(&(ip, what)) {
            s.push((ip, what));
            // Difftest trial children lower the log level to errors: stderr then.
            if log::log_enabled!(log::Level::Info) {
                log::info!("emu: DLL import stub {ip:#x}: {what}");
            } else {
                eprintln!("emu: DLL import stub {ip:#x}: {what}");
            }
        }
    }
}

/// Slot was filled (distinguishes "not a host function" from an empty slot).
pub const DF_VALID: u32 = 1;
/// `host_fn` is a native host callable ([`resolve_host_callable`] returned it).
pub const DF_HOST: u32 = 2;
/// AW2 integer replacement in the game window (preserves `xmm0` unless set explicitly).
pub const DF_GAME_REPL: u32 = 4;
/// AW2 `FLOAT_RETURN` function (result in `q0`; `rax` preserved for replacements).
pub const DF_FLOAT_RET: u32 = 8;
/// `AW2_TRACE_EAT=1` logging for non-replacement host calls.
pub const DF_TRACE: u32 = 16;

/// ABI fixup flags of a call target (independent of which host fn implements it).
#[cold]
#[inline(never)]
fn dispatch_flags(target_va: u64) -> u32 {
    static TRACE_EAT: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    let trace_eat = *TRACE_EAT.get_or_init(|| {
        std::env::var("AW2_TRACE_EAT").is_ok_and(|s| s == "1")
    });
    // AW2-only ABI fixups: the replacement window and `FLOAT_RETURN` are AW2 VAs, so they are
    // consulted only when the AW2 registry is applied (other exes may occupy the same VAs).
    let aw2 = crate::exe::aw2_registry_active();
    let is_game_repl = aw2
        && (0x1_4000_0000..0x1_4600_0000).contains(&target_va)
        && !((0x1_4000_0000..0x1_4300_0000).contains(&target_va)
            && unsafe { rd::<u16>(to_mapped_addr(target_va)) } == 0x25ff);
    // Lifted DLL entries follow the same xmm0 protocol (see `dll_native::lift::is_lifted_va`).
    let is_game_repl = is_game_repl || crate::dll_native::lift::is_lifted_va(target_va);
    let is_float_ret = aw2 && crate::hooks::is_float_return_va(target_va);
    let mut f = DF_VALID;
    if is_game_repl {
        f |= DF_GAME_REPL;
    }
    if is_float_ret {
        f |= DF_FLOAT_RET;
    }
    if trace_eat {
        f |= DF_TRACE;
    }
    if crate::jobprof::marked(target_va) {
        f |= DF_MARK;
    }
    f
}

/// `AW2_JOB_MARK` target: the bridges record the call as a `jobprof::K_MARK` span.
pub const DF_MARK: u32 = 64;

#[cold]
#[inline(never)]
fn resolve_dispatch_slow(va: u64) -> (usize, u32) {
    let mut f = dispatch_flags(va);
    let h = match resolve_host_callable(va) {
        Some(h) => {
            f |= DF_HOST;
            h
        }
        None => 0,
    };
    if h == 0 && dll_route_enabled() {
        if let Some(slot) = crate::dll_native::iat_stub_slot(va) {
            // Memoized as the slot address; `resolve_dispatch_in` reads the slot per call.
            dll_route_note(va, "dispatched (DF_IAT)");
            return (slot, f | DF_IAT);
        }
    }
    if f == DF_VALID | DF_HOST && (is_plain_builtin(h) || crate::hooks::is_extra_plain_native(h)) {
        f |= DF_PLAIN;
    }
    #[cfg(target_arch = "aarch64")]
    if f == DF_VALID | DF_HOST | DF_GAME_REPL && lean_target(h) {
        f |= DF_LEAN;
        if crate::shadow::selected(va) {
            f |= DF_SHADOW;
        }
    }
    (h, f)
}

/// `AW2_SHADOW`-selected [`DF_LEAN`] target: [`call_va8`] / `simd_ops::call_va8_xmm` miss
/// their `LEAN_FLAGS` fast path and the bridges hand the call to [`shadow_lean_va8`] /
/// [`shadow_lean_xmm`], which may start a [`crate::shadow`] probe before the real call.
pub const DF_SHADOW: u32 = 256;

/// `host_fn` is a compiled `aw2-game` replacement (a [`crate::registry`] function pointer)
/// and the call needs no other ABI fixup (no `DF_FLOAT_RET` / `DF_MARK` / `DF_TRACE`; see
/// [`LEAN_FLAGS`]). [`call_va8`] and `simd_ops::call_va8_xmm` enter it with a plain AAPCS
/// call ([`lean_call`]) instead of [`host_call_asm`], keeping the `CallCtx` protocol.
pub const DF_LEAN: u32 = 128;

/// The complete flags of a [`DF_LEAN`] target as returned by [`resolve_dispatch`].
pub const LEAN_FLAGS: u32 = DF_VALID | DF_HOST | DF_GAME_REPL | DF_LEAN;

/// Whether `h` is a registered replacement function (`registry::all()`, base + shards).
///
/// Those are all rustc-compiled Rust (no `naked`/`global_asm` functions are registered), so
/// they keep the AAPCS64 callee-saved registers. [`host_call_asm`]'s x19..x28 fence exists
/// for the guest-emulation stubs below other host functions; for these callees a typed
/// call leaves the callee exactly the register/stack image the asm builds. Exceptions are
/// unaffected: on aarch64 no fence shims are registered (`unwind::register_fence` is only
/// called from the x86_64 hook path) and `exe::try_call`'s thunk does not catch, so no
/// unwind ever resumes through the bridge's saved registers.
///
/// Difftest `Mock`s and `temp_patch` re-point a VA to a fixture (not in the set) and bump
/// `va_gen`, so mocked calls take the full bridge. `AW2_DIRECT_CALLS=0` (A/B knob, read
/// once) disables the classification; the hot path has no extra check.
#[cfg(target_arch = "aarch64")]
#[cold]
#[inline(never)]
fn lean_target(h: usize) -> bool {
    static SET: std::sync::OnceLock<Option<Vec<usize>>> = std::sync::OnceLock::new();
    let set = SET.get_or_init(|| {
        if std::env::var("AW2_DIRECT_CALLS").is_ok_and(|s| s.trim() == "0") {
            log::info!("emu: direct calls off (AW2_DIRECT_CALLS=0): replacements use the full bridge");
            return None;
        }
        let mut v: Vec<usize> = crate::hooks::collect_lean_targets();
        v.retain(|&p| p != 0);
        v.sort_unstable();
        v.dedup();
        log::info!(
            "emu: direct calls to {} compiled replacement functions (AW2_DIRECT_CALLS=0 disables)",
            v.len()
        );
        Some(v)
    });
    h != 0 && set.as_ref().is_some_and(|v| v.binary_search(&h).is_ok())
}

/// [`host_call_asm`]'s argument image (x0..x7 = `gpr`, q0..q3 = `xmm`, stack arguments
/// 9..16 = `stack_args` at `[sp + 0x00..0x40]`) built by an ordinary AAPCS64 call, without
/// the callee-saved fence. Returns `x0`. Only for [`DF_LEAN`] targets.
#[cfg(target_arch = "aarch64")]
#[inline(always)]
#[allow(improper_ctypes, improper_ctypes_definitions)]
pub(crate) unsafe fn lean_call(host_fn: usize, gpr: &[u64; 8], xmm: &[u128; 4], stack_args: &[u64; 8]) -> u64 {
    use core::arch::aarch64::uint64x2_t;
    // AAPCS64: the 8 integer args take x0..x7, the 4 short vectors q0..q3 (independent
    // register counter), and the integer args after the 8th go to the stack in order.
    type Lean = unsafe extern "C" fn(
        u64, u64, u64, u64, u64, u64, u64, u64,
        uint64x2_t, uint64x2_t, uint64x2_t, uint64x2_t,
        u64, u64, u64, u64, u64, u64, u64, u64,
    ) -> u64;
    let f: Lean = core::mem::transmute(core::hint::black_box(host_fn));
    let q = |v: u128| -> uint64x2_t { core::mem::transmute(v) };
    f(
        gpr[0], gpr[1], gpr[2], gpr[3], gpr[4], gpr[5], gpr[6], gpr[7],
        q(xmm[0]), q(xmm[1]), q(xmm[2]), q(xmm[3]),
        stack_args[0], stack_args[1], stack_args[2], stack_args[3],
        stack_args[4], stack_args[5], stack_args[6], stack_args[7],
    )
}

/// [`call_host_dispatched_in`] for a [`LEAN_FLAGS`] target (`DF_GAME_REPL`, no
/// `DF_FLOAT_RET` / `DF_MARK` / `DF_TRACE`) through [`lean_call`]: same `CallCtx` effects
/// (publishes `xmm`, consumes `xmm_out`) and result `(x0, set_xmm_ret value or xmm[0])`.
#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub(crate) unsafe fn call_lean_in(
    c: &crate::callctx::CallCtx,
    host_fn: usize,
    gpr: &[u64; 8],
    xmm: &[u128; 4],
    stack_args: &[u64; 8],
) -> (u64, u128) {
    c.xmm_in.set(*xmm);
    c.xmm_in_set.set(true);
    c.xmm_out.set(None);
    let r_gpr = lean_call(host_fn, gpr, xmm, stack_args);
    c.xmm_in_set.set(false);
    let r_xmm = match c.xmm_out.take() {
        Some(v) => v,
        None => xmm[0],
    };
    (r_gpr, r_xmm)
}

/// `host_fn` is an integer-only win32-rt builtin (at most 8 args, no XMM or stack
/// args, never calls back into guest code): [`call_va8`] calls it directly.
pub const DF_PLAIN: u32 = 32;

/// Hot builtins safe to call without the Win64 bridge (see [`DF_PLAIN`]).
fn is_plain_builtin(h: usize) -> bool {
    if h == 0 {
        return false;
    }
    static SORTED: std::sync::OnceLock<Box<[usize]>> = std::sync::OnceLock::new();
    let table = SORTED.get_or_init(|| {
        let mut v = win32_rt::plain_builtin_fns().to_vec();
        v.retain(|&p| p != 0);
        v.sort_unstable();
        v.dedup();
        v.into_boxed_slice()
    });
    table.binary_search(&h).is_ok()
}

/// Host function the translator may call inline (no bridge, no TLS context) in place of
/// the non-`DF_PLAIN` host function `h`, with `host_call_asm`'s register setup (Win64 args
/// 1..8 in x0..x7, xmm0..3 in q0..q3; result rax = x0, xmm0 = q0), and whether a 0 result
/// means "not handled, make the real call" (then the variant had no side effects).
/// Every variant provably never runs guest code.
#[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]
pub(super) fn inline_variant(h: usize) -> Option<(usize, bool)> {
    use win32_rt::ucrt::heap;
    if h == 0 {
        return None;
    }
    static PURE_MATH: std::sync::OnceLock<Box<[usize]>> = std::sync::OnceLock::new();
    let pure = PURE_MATH.get_or_init(|| {
        let mut v = win32_rt::pure_float_math_fns().to_vec();
        v.retain(|&p| p != 0);
        v.sort_unstable();
        v.dedup();
        v.into_boxed_slice()
    });
    if pure.binary_search(&h).is_ok() {
        return Some((h, false));
    }
    if h == heap::malloc as *const () as usize || h == heap::_malloc_base as *const () as usize {
        return Some((malloc_try as *const () as usize, true));
    }
    if h == heap::calloc as *const () as usize || h == heap::_calloc_base as *const () as usize {
        return Some((calloc_try as *const () as usize, true));
    }
    if let Some(hit) = crate::hooks::lookup_extra_inline_variant(h) {
        return Some(hit);
    }
    None
}

/// `malloc` without the out-of-memory path (`_callnewh` may run a guest new handler):
/// the block, or 0 to make the real call.
#[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]
unsafe extern "win64" fn malloc_try(size: usize) -> *mut core::ffi::c_void {
    win32_rt::heap::alloc(size.max(1), false)
}

/// `calloc` without the overflow (`errno`) and out-of-memory paths: the zeroed block, or 0
/// to make the real call.
#[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]
unsafe extern "win64" fn calloc_try(n: usize, size: usize) -> *mut core::ffi::c_void {
    match n.checked_mul(size) {
        Some(t) => win32_rt::heap::alloc(t.max(1), true),
        None => core::ptr::null_mut(),
    }
}

/// [`resolve_host_callable`] plus the call's ABI flags (`DF_*`), memoized in a per-thread
/// direct-mapped cache tagged with [`crate::exe::va_gen`]. Avoids the global VA hash-table
/// probe and the guest code-byte read (both usually cache misses) on every lifted call.
#[inline(always)]
pub fn resolve_dispatch(va: u64) -> (usize, u32) {
    resolve_dispatch_in(crate::callctx::ctx(), va)
}

/// [`resolve_dispatch`] with the caller's per-thread context already in hand.
///
/// A [`DF_IAT`] entry (a mapped DLL's import stub `jmp [rip+slot]`) is never answered from
/// the memo: the slot is read now and its current target resolved ([`iat_dispatch`]), so a
/// rewritten slot takes effect on the next call.
#[inline(always)]
pub(crate) fn resolve_dispatch_in(c: &crate::callctx::CallCtx, va: u64) -> (usize, u32) {
    let (h, f) = resolve_dispatch_memo(c, va);
    if f & DF_IAT != 0 {
        return iat_dispatch(c, h, f);
    }
    (h, f)
}

/// [`resolve_dispatch`] without following DLL import stubs: a [`DF_IAT`] entry comes back
/// as-is (`host_fn` = the slot address, never a callable). For callers that bake the result
/// into translated code, where a later slot rewrite would not be seen.
#[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]
#[inline]
pub(crate) fn resolve_dispatch_static(va: u64) -> (usize, u32) {
    resolve_dispatch_memo(crate::callctx::ctx(), va)
}

/// The per-thread memo behind [`resolve_dispatch_in`] (entries tagged with
/// [`crate::exe::va_gen`]).
#[inline(always)]
fn resolve_dispatch_memo(c: &crate::callctx::CallCtx, va: u64) -> (usize, u32) {
    use crate::callctx::{DCACHE_BITS, DCACHE_LEN};
    let gen = crate::exe::va_gen();
    let idx = (((va >> 4) ^ (va >> (4 + DCACHE_BITS))) as usize) & (DCACHE_LEN - 1);
    unsafe {
        let e = (*c.dcache.get()).get_unchecked(idx);
        if e.va == va && e.gen == gen {
            return (e.host_fn, e.flags);
        }
    }
    let (h, f) = resolve_dispatch_slow(va);
    unsafe {
        *(*c.dcache.get()).get_unchecked_mut(idx) =
            crate::callctx::DispatchSlot { va, host_fn: h, flags: f, gen };
    }
    (h, f)
}

/// Target is a mapped DLL's import stub `jmp qword ptr [rip+disp32]` whose slot lies in the
/// image's IAT directory (`dll_native::iat_stub_slot`); the memoized `host_fn` is the slot
/// address. Only [`resolve_dispatch_memo`] / [`resolve_dispatch_static`] return it;
/// [`resolve_dispatch_in`] replaces it by the slot target's resolution at every call.
pub const DF_IAT: u32 = 512;

/// [`DF_IAT`] entry `(slot, stub_flags)` at call time: read the slot now and return its
/// target's own `(host_fn, flags)` when that is a host callable. Otherwise (guest target,
/// unbound slot, a target that is itself a stub) `(0, stub flags)`: the caller runs the
/// emulator at the stub and the run loop's DLL stub route jumps to the slot target. The slot
/// value is never memoized; only the target's resolution is, under its own VA.
/// [`DF_SHADOW`] is dropped: an `AW2_SHADOW` probe would be keyed by the stub VA.
#[inline(never)]
fn iat_dispatch(c: &crate::callctx::CallCtx, slot: usize, stub_flags: u32) -> (usize, u32) {
    // SAFETY: `slot` is an aligned qword inside a mapped image's IAT directory.
    let t = unsafe { core::ptr::read_volatile(slot as *const u64) };
    if t != 0 {
        let (h, f) = resolve_dispatch_memo(c, t);
        if h != 0 && f & DF_HOST != 0 && f & DF_IAT == 0 {
            return (h, f & !DF_SHADOW);
        }
    }
    (0, stub_flags & !DF_IAT)
}

/// `AW2_DLL_ROUTE=0` turns off DLL import-stub dispatch ([`DF_IAT`] and the run loop's DLL
/// stub route): stubs then run as guest code (the pre-g23 behaviour). Read once.
#[inline]
fn dll_route_enabled() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| {
        let on = !std::env::var("AW2_DLL_ROUTE").is_ok_and(|v| v.trim() == "0");
        if !on {
            log::info!("emu: DLL import-stub dispatch off (AW2_DLL_ROUTE=0)");
        }
        on
    })
}

/// Map a preferred-base exe VA to its actual mapped address.
#[inline]
pub fn to_mapped_addr(va_or_addr: u64) -> usize {
    let b = base() as u64;
    let pref = crate::exe::preferred_base();
    let sz = crate::exe::image_size();
    if b != pref && (pref..pref.wrapping_add(sz)).contains(&va_or_addr) {
        (b.wrapping_add(va_or_addr - pref)) as usize
    } else {
        va_or_addr as usize
    }
}

/// Call a guest or native function at `va_or_addr` with up to 8 Win64 integer/pointer arguments.
///
/// - On `x86_64`, calls the target directly via `extern "win64"`.
/// - On `aarch64`, checks `resolve_host_callable` first; if it is a compiled native Rust
///   function (`aw2-game` or `win32-rt`), invokes it directly; otherwise runs the `iced-x86`
///   interpreter starting at `to_mapped_addr(va_or_addr)`.
#[inline]
pub unsafe fn call_va8(
    va_or_addr: u64,
    rcx: u64,
    rdx: u64,
    r8: u64,
    r9: u64,
    arg5: u64,
    arg6: u64,
    arg7: u64,
    arg8: u64,
) -> u64 {
    #[cfg(target_arch = "x86_64")]
    {
        let addr = resolve_host_callable(va_or_addr).unwrap_or_else(|| to_mapped_addr(va_or_addr));
        let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =
            core::mem::transmute(addr);
        f(rcx, rdx, r8, r9, arg5, arg6, arg7, arg8)
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        // One per-thread context lookup for the whole call (each `ctx()` is a TSD
        // load + null checks that LLVM cannot merge across the stores in between).
        let c = crate::callctx::ctx();
        if crate::callprof::on() {
            let va = va_or_addr & !(1u64 << 46);
            crate::callprof::hit(c, va_or_addr, resolve_dispatch_in(c, va), crate::callprof::ENTRY_VA8);
        }
        let is_orig = (va_or_addr & (1u64 << 46)) != 0;
        let clean_va = va_or_addr & !(1u64 << 46);
        let (host_fn, flags) = if is_orig { (0, 0) } else { resolve_dispatch_in(c, clean_va) };
        #[cfg(target_arch = "aarch64")]
        if flags == LEAN_FLAGS {
            // Compiled replacement: the `DF_HOST` protocol of `call_va8_bridge` (rsp
            // windows, stack args 9..16, forwarded XMM, `rax_in`), host call via `lean_call`.
            let (mut out_rsp, mut out_limit) = c.outgoing_rsp.replace((0, 0));
            if out_rsp == 0 {
                (out_rsp, out_limit) = c.incoming_rsp.get();
            }
            let s_extra = read_caller_extra_args(out_rsp, out_limit);
            let fwd = c.xmm_in_set.get();
            let xmm = if fwd { c.xmm_in.get() } else { [rcx as u128, rdx as u128, r8 as u128, r9 as u128] };
            let prev_rax = c.rax_in.get();
            c.incoming_rsp.set((out_rsp, out_limit));
            let (r_gpr, r_xmm) =
                call_lean_in(c, host_fn, &[rcx, rdx, r8, r9, arg5, arg6, arg7, arg8], &xmm, &s_extra);
            c.rax_in.set(prev_rax);
            c.incoming_rsp.set((0, 0));
            if fwd && r_xmm != xmm[0] {
                crate::simd_ops::set_xmm_ret(r_xmm);
            }
            return r_gpr;
        }
        if flags == DF_VALID | DF_HOST | DF_PLAIN {
            // Integer-only builtin (<= 8 args, no XMM or stack args, never runs guest code):
            // the end state `call_va8_bridge` + `call_host_dispatched_in` leave, minus the
            // ABI shuffle:
            // - outgoing and incoming rsp windows consumed;
            // - forwarded XMM consumed (`xmm_in_set` false);
            // - no XMM result.
            // The bridge would also publish the builtin's leftover q0 as an XMM result when
            // XMM was forwarded; that's undefined after an integer function in Win64, and
            // callers of `call_va8` don't read it.
            // Taken even when stack args or forwarded XMM are pending: `call_lean_in`
            // leaves `xmm_in_set` up until a callee's first nested call, so the first
            // builtin call inside a lean-called function used to take the full bridge.
            c.outgoing_rsp.set((0, 0));
            c.incoming_rsp.set((0, 0));
            c.xmm_in_set.set(false);
            c.xmm_out.set(None);
            let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =
                core::mem::transmute(host_fn);
            return f(rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
        }
        call_va8_bridge(c, is_orig, clean_va, host_fn, flags, [rcx, rdx, r8, r9, arg5, arg6, arg7, arg8])
    }
}

/// The generic part of [`call_va8`] (host functions through [`host_call_asm`], guest code
/// through [`emu_run_full`]). Out of line so the fast paths keep a small frame.
#[cfg(not(target_arch = "x86_64"))]
#[inline(never)]
unsafe fn call_va8_bridge(
    c: &crate::callctx::CallCtx,
    is_orig: bool,
    clean_va: u64,
    host_fn: usize,
    flags: u32,
    gpr: [u64; 8],
) -> u64 {
    #[cfg(target_arch = "aarch64")]
    if flags & DF_SHADOW != 0 {
        return shadow_lean_va8(c, clean_va, host_fn, gpr);
    }
    let [rcx, rdx, r8, r9, ..] = gpr;
    let (mut out_rsp, mut out_limit) = c.outgoing_rsp.replace((0, 0));
    if out_rsp == 0 {
        let (in_rsp, in_limit) = c.incoming_rsp.replace((0, 0));
        out_rsp = in_rsp;
        out_limit = in_limit;
    }
    let s_extra = read_caller_extra_args(out_rsp, out_limit);
    let fwd_xmm = if c.xmm_in_set.replace(false) { Some(c.xmm_in.get()) } else { None };
    let xmm = fwd_xmm.unwrap_or([rcx as u128, rdx as u128, r8 as u128, r9 as u128]);
    let prev_rax = c.rax_in.get();
    if !is_orig {
        if flags & DF_HOST != 0 {
            c.incoming_rsp.set((out_rsp, out_limit));
            let (r_gpr, r_xmm) = call_host_dispatched_in(c, clean_va, host_fn, flags, &gpr, &xmm, &s_extra);
            c.rax_in.set(prev_rax);
            c.incoming_rsp.set((0, 0));
            if fwd_xmm.is_some() && r_xmm != xmm[0] {
                crate::simd_ops::set_xmm_ret(r_xmm);
            }
            return r_gpr;
        }
        if is_routed_dll_stack_pos(clean_va) {
            let r_gpr = dll_stack_pos_va8(c);
            c.rax_in.set(prev_rax);
            if fwd_xmm.is_some() {
                crate::simd_ops::set_xmm_ret(xmm[0]);
            }
            return r_gpr;
        }
    }
    set_incoming_rsp(out_rsp, out_limit);
    let (r_gpr, r_xmm) = emu_run_full_rax(to_mapped_addr(clean_va) as u64, gpr, xmm, s_extra, prev_rax);
    crate::simd_ops::replace_rax_in(prev_rax);
    set_incoming_rsp(0, 0);
    if fwd_xmm.is_some() {
        crate::simd_ops::set_xmm_ret(r_xmm);
    }
    r_gpr
}

/// [`call_va8`]'s `LEAN_FLAGS` branch for a [`DF_SHADOW`] target: identical `CallCtx`
/// protocol and result, preceded by [`crate::shadow::probe_lean`] (which forks a probe
/// only when armed and the fn's sampling budget allows; otherwise it returns at once).
#[cfg(target_arch = "aarch64")]
#[inline(never)]
unsafe fn shadow_lean_va8(c: &crate::callctx::CallCtx, va: u64, host_fn: usize, gpr: [u64; 8]) -> u64 {
    let (mut out_rsp, mut out_limit) = c.outgoing_rsp.replace((0, 0));
    if out_rsp == 0 {
        (out_rsp, out_limit) = c.incoming_rsp.get();
    }
    let s_extra = read_caller_extra_args(out_rsp, out_limit);
    let fwd = c.xmm_in_set.get();
    let xmm = if fwd { c.xmm_in.get() } else { [gpr[0] as u128, gpr[1] as u128, gpr[2] as u128, gpr[3] as u128] };
    crate::shadow::probe_lean(c, va, host_fn, &gpr, &xmm, &s_extra, (out_rsp, out_limit), false);
    let prev_rax = c.rax_in.get();
    c.incoming_rsp.set((out_rsp, out_limit));
    let (r_gpr, r_xmm) = call_lean_in(c, host_fn, &gpr, &xmm, &s_extra);
    c.rax_in.set(prev_rax);
    c.incoming_rsp.set((0, 0));
    if fwd && r_xmm != xmm[0] {
        crate::simd_ops::set_xmm_ret(r_xmm);
    }
    r_gpr
}

/// `simd_ops::call_va8_xmm`'s `LEAN_FLAGS` branch for a [`DF_SHADOW`] target (see
/// [`shadow_lean_va8`]).
#[cfg(target_arch = "aarch64")]
#[inline(never)]
pub(crate) unsafe fn shadow_lean_xmm(
    c: &crate::callctx::CallCtx,
    va: u64,
    host_fn: usize,
    gpr: [u64; 8],
    xmm: [u128; 4],
) -> (u64, u128) {
    let (out_rsp, out_limit) = c.outgoing_rsp.replace((0, 0));
    let s_extra = read_caller_extra_args(out_rsp, out_limit);
    crate::shadow::probe_lean(c, va, host_fn, &gpr, &xmm, &s_extra, (out_rsp, out_limit), true);
    let prev_rax = c.rax_in.get();
    c.incoming_rsp.set((out_rsp, out_limit));
    let r = call_lean_in(c, host_fn, &gpr, &xmm, &s_extra);
    c.rax_in.set(prev_rax);
    c.incoming_rsp.set((0, 0));
    r
}

/// Shadow probe runs (forked children only): the guest original of `va` the way
/// [`call_va8_bridge`] runs an ORIG (bit 46) call, after the caller's rsp window is restored.
#[cfg_attr(target_arch = "x86_64", allow(dead_code))]
pub(crate) unsafe fn shadow_run_guest(
    entry: u64,
    rsp_window: (u64, u64),
    gpr: &[u64; 8],
    xmm: &[u128; 4],
    s_extra: &[u64; 8],
) -> (u64, u128) {
    set_incoming_rsp(rsp_window.0, rsp_window.1);
    let r = emu_run_full(to_mapped_addr(entry) as u64, *gpr, *xmm, *s_extra);
    set_incoming_rsp(0, 0);
    r
}

/// Shadow probe runs (forked children only): the lean native call, as in [`shadow_lean_va8`].
#[cfg(target_arch = "aarch64")]
pub(crate) unsafe fn shadow_run_lean(
    host_fn: usize,
    rsp_window: (u64, u64),
    gpr: &[u64; 8],
    xmm: &[u128; 4],
    s_extra: &[u64; 8],
) -> (u64, u128) {
    let c = crate::callctx::ctx();
    c.incoming_rsp.set(rsp_window);
    // Same incoming rax as the guest runs (`emu_run_full` starts with rax = 0): lifted
    // bodies return `get_rax_in()` on paths where the original leaves rax untouched.
    c.rax_in.set(0);
    let r = call_lean_in(c, host_fn, gpr, xmm, s_extra);
    c.incoming_rsp.set((0, 0));
    r
}

/// Forking harnesses (dll_lift difftest): create this thread's emulator state and set the
/// translator up (MAP_JIT code region + stubs) before the forks, so every child inherits it
/// instead of mapping and writing a fresh MAP_JIT region itself. g14c: 4 children whose
/// first guest run did that faulted on the region's first write (3 at its base, which mmap
/// had placed right after the lstack arena the parent set up just before; 1 at +0x4000),
/// and the child's later guest run died; none reproduced.
pub unsafe fn jit_warm() {
    TLS_EMU.with(|cell| {
        let opt = &mut *cell.get();
        let state = opt.get_or_insert_with(ThreadEmuState::new);
        if jit::enabled() {
            state.jit.warm();
        }
    });
}

/// Shadow orchestrator setup on the probing thread: make sure this thread's emulator state
/// exists (so the probe runs never allocate it), switch the translator off unless `jit`,
/// and append the emulator's scratch memory to `out` as `[lo, hi)`: the guest stack below
/// its current sp and the block-cache tables. Callers' frames above the guest sp stay
/// visible to the comparison. Returns the guest-stack scratch `[base, sp)`.
pub(crate) unsafe fn shadow_prepare(jit: bool, hard: &mut Vec<(usize, usize)>, soft: &mut Vec<(usize, usize)>) -> (usize, usize) {
    if !jit {
        jit::force(2);
    }
    TLS_EMU.with(|cell| {
        let opt = &mut *cell.get();
        let state = opt.get_or_insert_with(ThreadEmuState::new);
        let base = state._stack.as_ptr() as usize;
        let sp = if state.sp < state.stack_min || state.sp > state.stack_top { state.stack_top } else { state.sp };
        out_split(hard, soft, base, sp as usize);
        let sp_ = state as *const ThreadEmuState as usize;
        soft.push((sp_, sp_ + core::mem::size_of::<ThreadEmuState>()));
        let b = &state.blocks;
        for (p, n) in [
            (b.arena.as_ptr() as usize, core::mem::size_of_val(&*b.arena)),
            (b.ops.as_ptr() as usize, core::mem::size_of_val(&*b.ops)),
            (b.index.as_ptr() as usize, core::mem::size_of_val(&*b.index)),
            (b.rcache.as_ptr() as usize, core::mem::size_of_val(&*b.rcache)),
        ] {
            hard.push((p, p + n));
        }
        (base, sp as usize)
    })
}

/// Scratch `[lo, hi)` below a live stack top: untracked up to the page holding `hi`
/// (rounded down, so callers' frames in that page stay tracked), byte-ignored up to `hi`.
pub(crate) fn out_split(hard: &mut Vec<(usize, usize)>, soft: &mut Vec<(usize, usize)>, lo: usize, hi: usize) {
    let pg = crate::shadow::page_size();
    hard.push((lo, hi & !(pg - 1)));
    soft.push((lo, hi));
}

/// CPU state for the `iced-x86` interpreter.
///
/// GPR indexing:
/// `0: rax, 1: rcx, 2: rdx, 3: rbx, 4: rsp, 5: rbp, 6: rsi, 7: rdi, 8..16: r8..r15`.
///
/// `repr(C)`: the block translator ([`jit`]) addresses fields by offset.
#[derive(Clone)]
#[repr(C)]
pub struct Cpu {
    pub gpr: [u64; 16],
    pub xmm: [u128; 16],
    pub ymm_hi: [u128; 16],
    pub rip: u64,
    pub cf: bool,
    pub zf: bool,
    pub sf: bool,
    pub of: bool,
    /// Parity-flag source byte: PF is set iff this byte has an even number of 1 bits
    /// (stored lazily as the low result byte; see [`Cpu::pf`]).
    pub pfv: u8,
    /// Direction flag (STD/CLD); string instructions step backwards when set.
    pub df: bool,
    /// Width of the last write per GPR: 8 or 16 for partial writes, anything else (64, or
    /// 0 from translated code) means a full-width write. Only read by [`Cpu::abi_gpr`].
    pub last_bits: [u8; 16],
    /// Scratch area shared with translated code.
    jx: jit::JitScratch,
}

impl Cpu {
    /// Current guest x86-64 `MXCSR` register value.
    #[inline(always)]
    pub(crate) fn mxcsr(&self) -> u32 {
        self.jx.mxcsr
    }

    /// Write `self.ymm_hi[d] = v` and maintain the `self.jx.ymm_dirty` bitmask incrementally.
    #[inline(always)]
    pub(super) fn set_ymm_hi(&mut self, d: usize, v: u128) {
        self.ymm_hi[d] = v;
        if v != 0 {
            self.jx.ymm_dirty |= 1u16 << d;
        } else {
            self.jx.ymm_dirty &= !(1u16 << d);
        }
    }

    /// Zero `self.ymm_hi[d]` and clear bit `d` in `self.jx.ymm_dirty`.
    #[inline(always)]
    pub(super) fn clear_ymm_hi_reg(&mut self, d: usize) {
        self.ymm_hi[d] = 0;
        self.jx.ymm_dirty &= !(1u16 << d);
    }

    /// Zero all 16 `self.ymm_hi` upper halves (`vzeroupper`/`vzeroall`) and clear `self.jx.ymm_dirty`.
    #[inline(always)]
    pub(super) fn clear_all_ymm_hi(&mut self) {
        self.ymm_hi = [0; 16];
        self.jx.ymm_dirty = 0;
    }

    pub fn new_zeroed() -> Self {
        Self {
            gpr: [0; 16],
            xmm: [0; 16],
            ymm_hi: [0; 16],
            rip: 0,
            cf: false,
            zf: false,
            sf: false,
            of: false,
            pfv: 1,
            df: false,
            last_bits: [64; 16],
            jx: jit::JitScratch { teb: unsafe { teb_self() } as u64, ..jit::JitScratch::default() },
        }
    }
}

const RET_SENTINEL: u64 = 0xDEAD_BEEF_CAFE_0000;

/// Bits of ARM64 `FPCR` controlled by guest x86 `MXCSR`: `FZ` (bit 24) and `RMode` (bits 23:22).
#[cfg(target_arch = "aarch64")]
pub(super) const FPCR_GUEST_MASK: u64 = 7u64 << 22;

/// Bits of guest x86 `MXCSR` that affect ARM64 `FPCR`: `FTZ` (bit 15), `RC` (bits 14:13), `DAZ` (bit 6).
/// When `(mxcsr & MXCSR_FPCR_MASK) == 0` (including the `0x1F80` default), `mxcsr_to_fpcr(mxcsr) == 0`.
pub(super) const MXCSR_FPCR_MASK: u32 = 0xE040;

/// Convert x86-64 `MXCSR` control bits (`FTZ`=15, `DAZ`=6, `RC`=14:13) to ARM64 `FPCR`
/// (`FZ`=24, `RMode`=23:22). Note `RMode` swaps `RM`/`RP` relative to `MXCSR.RC`:
/// `RC 00->00 (RN), 01->10 (RM), 10->01 (RP), 11->11 (RZ)`.
#[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]
#[inline(always)]
pub(super) fn mxcsr_to_fpcr(mxcsr: u32) -> u64 {
    let rc_lo = ((mxcsr >> 13) & 1) as u64;
    let rc_hi = ((mxcsr >> 14) & 1) as u64;
    let fz = (((mxcsr >> 15) | (mxcsr >> 6)) & 1) as u64;
    (fz << 24) | (rc_lo << 23) | (rc_hi << 22)
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub(super) fn read_fpcr() -> u64 {
    let v: u64;
    unsafe {
        core::arch::asm!("mrs {}, fpcr", out(reg) v, options(nomem, nostack, preserves_flags));
    }
    v
}

#[cfg(not(target_arch = "aarch64"))]
#[allow(dead_code)]
#[inline(always)]
pub(super) fn read_fpcr() -> u64 {
    0
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
pub(super) fn write_fpcr(v: u64) {
    unsafe {
        core::arch::asm!("msr fpcr, {}", in(reg) v, options(nomem, nostack, preserves_flags));
    }
}

#[cfg(not(target_arch = "aarch64"))]
#[allow(dead_code)]
#[inline(always)]
pub(super) fn write_fpcr(_v: u64) {}

/// Apply a guest `MXCSR` value to ARM64 hardware `FPCR`, skipping the serializing `msr`
/// when `FPCR` already matches.
#[inline(always)]
pub(super) fn apply_guest_mxcsr_hw(mxcsr: u32) {
    #[cfg(target_arch = "aarch64")]
    {
        let cur = read_fpcr();
        let target = (cur & !FPCR_GUEST_MASK) | mxcsr_to_fpcr(mxcsr);
        if target != cur {
            write_fpcr(target);
        }
    }
    #[cfg(not(target_arch = "aarch64"))]
    let _ = mxcsr;
}

/// RAII guard for one guest execution region (`emu_run_region`): installs the guest thread's
/// `MXCSR` into hardware `FPCR` on entry and writes back `Cpu::jx.mxcsr` + restores the host's
/// saved `FPCR` on exit (including EH unwinds). Skips `mrs`/`msr fpcr` completely on the
/// common path where `MXCSR` has default rounding/FTZ/DAZ bits (`(mxcsr & 0xE040) == 0`).
struct GuestFpcrGuard {
    mxcsr_ptr: *const u32,
    entry_mxcsr: u32,
    #[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]
    saved_host_fpcr: Option<u64>,
}

impl GuestFpcrGuard {
    #[inline(always)]
    unsafe fn enter(mxcsr_ptr: *const u32) -> Self {
        let entry_mxcsr = *mxcsr_ptr;
        #[cfg(target_arch = "aarch64")]
        let saved_host_fpcr = if (entry_mxcsr & MXCSR_FPCR_MASK) != 0 {
            let saved = read_fpcr();
            let target = (saved & !FPCR_GUEST_MASK) | mxcsr_to_fpcr(entry_mxcsr);
            if target != saved {
                write_fpcr(target);
            }
            Some(saved)
        } else {
            None
        };
        #[cfg(not(target_arch = "aarch64"))]
        let saved_host_fpcr = None;
        Self { mxcsr_ptr, entry_mxcsr, saved_host_fpcr }
    }
}

impl Drop for GuestFpcrGuard {
    #[inline(always)]
    fn drop(&mut self) {
        let cur_m = unsafe { *self.mxcsr_ptr };
        if cur_m != self.entry_mxcsr {
            win32_rt::ucrt::runtime::set_guest_mxcsr(cur_m);
        }
        #[cfg(target_arch = "aarch64")]
        {
            if let Some(saved) = self.saved_host_fpcr {
                if read_fpcr() != saved {
                    write_fpcr(saved);
                }
            } else if (cur_m & MXCSR_FPCR_MASK) != 0 {
                let cur = read_fpcr();
                if (cur & FPCR_GUEST_MASK) != 0 {
                    write_fpcr(cur & !FPCR_GUEST_MASK);
                }
            }
        }
    }
}

/// RAII guard around full host calls (`host_call` / `host_tail_call`): syncs `Cpu::jx.mxcsr`
/// to `win32_rt`'s thread-local `MXCSR` (for `_controlfp`/`_control87` and nested guest
/// callbacks), resets hardware `FPCR` to IEEE-754 defaults while host code runs, and pulls
/// any updated `MXCSR` back into `Cpu::jx.mxcsr` + hardware `FPCR` upon return.
struct HostFpcrGuard {
    mxcsr_ptr: *mut u32,
}

impl HostFpcrGuard {
    #[inline(always)]
    unsafe fn enter(mxcsr_ptr: *mut u32) -> Self {
        let m = *mxcsr_ptr;
        win32_rt::ucrt::runtime::set_guest_mxcsr(m);
        #[cfg(target_arch = "aarch64")]
        if (m & MXCSR_FPCR_MASK) != 0 {
            let cur = read_fpcr();
            if (cur & FPCR_GUEST_MASK) != 0 {
                write_fpcr(cur & !FPCR_GUEST_MASK);
            }
        }
        Self { mxcsr_ptr }
    }
}

impl Drop for HostFpcrGuard {
    #[inline(always)]
    fn drop(&mut self) {
        let m = win32_rt::ucrt::runtime::guest_mxcsr() & 0xFFFF;
        unsafe {
            *self.mxcsr_ptr = m;
        }
        if (m & MXCSR_FPCR_MASK) != 0 {
            apply_guest_mxcsr_hw(m);
        }
    }
}

/// Lightweight RAII guard for direct leaf host calls (`plain_host_call_asm`/`xfer_plain`/public
/// host dispatch): clears guest `FPCR` bits (`FZ`/`RMode`) while host code runs (no-op if
/// already 0) and restores on return.
struct ClearFpcrGuard(#[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))] Option<u64>);

impl ClearFpcrGuard {
    #[inline(always)]
    fn enter() -> Self {
        #[cfg(target_arch = "aarch64")]
        {
            let cur = read_fpcr();
            if (cur & FPCR_GUEST_MASK) != 0 {
                write_fpcr(cur & !FPCR_GUEST_MASK);
                return Self(Some(cur));
            }
        }
        Self(None)
    }

    /// Fast path when guest `mxcsr` is known: skips `mrs fpcr` when `(mxcsr & MXCSR_FPCR_MASK) == 0`.
    #[inline(always)]
    fn enter_for_mxcsr(mxcsr: u32) -> Self {
        if (mxcsr & MXCSR_FPCR_MASK) != 0 {
            Self::enter()
        } else {
            Self(None)
        }
    }
}

impl Drop for ClearFpcrGuard {
    #[inline(always)]
    fn drop(&mut self) {
        #[cfg(target_arch = "aarch64")]
        if let Some(prev) = self.0 {
            if read_fpcr() != prev {
                write_fpcr(prev);
            }
        }
    }
}

#[cfg(test)]
std::thread_local! {
    static STRICT_FAULTS: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
}

/// Unit tests only: the interpreter/JIT fuzzers probe invalid addresses on purpose and compare
/// the lenient results, so tests keep the old behaviour (int3 returns, invalid accesses read 0)
/// unless the thread opted in with [`strict_faults_for_test`]. Always `false` outside tests.
#[inline(always)]
fn lenient_in_tests() -> bool {
    #[cfg(test)]
    {
        !STRICT_FAULTS.with(|c| c.get())
    }
    #[cfg(not(test))]
    {
        false
    }
}

/// Unit tests: use the production int3 / invalid-address behaviour on this thread.
#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn strict_faults_for_test(on: bool) {
    STRICT_FAULTS.with(|c| c.set(on));
}

/// `AW2_EMU_INT3=return` (A/B only): an `int 3` / `ud2` no vectored handler takes makes the
/// guest function return (the old behaviour). Default: unhandled exception (crash report,
/// the application's filter, exit), as on Windows.
fn int3_returns() -> bool {
    if lenient_in_tests() {
        return true;
    }
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| {
        let r = std::env::var("AW2_EMU_INT3").is_ok_and(|v| v.trim() == "return");
        if r {
            log::warn!("emu: AW2_EMU_INT3=return: unhandled int3/ud2 return from the guest function (A/B only)");
        }
        r
    })
}

/// `AW2_EMU_BADADDR=ignore` (A/B only): accesses outside `[0x10000, 2^47)` read 0 / drop
/// the write forever (the old behaviour). Default: see [`cold_bad_addr`].
fn bad_addr_ignored() -> bool {
    if lenient_in_tests() {
        return true;
    }
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| {
        let r = std::env::var("AW2_EMU_BADADDR").is_ok_and(|v| v.trim() == "ignore");
        if r {
            log::warn!("emu: AW2_EMU_BADADDR=ignore: invalid guest accesses read 0 (A/B only)");
        }
        r
    })
}

/// Consecutive invalid accesses at one `rip` that raise an access violation (a pointer walk
/// such as `rbx = [rbx+0xd8]` that left the address space spins forever on reads of 0).
const BAD_ADDR_RUN_LIMIT: u32 = 100_000;

std::thread_local! {
    /// (`rip`, consecutive invalid accesses there).
    static BAD_ADDR_RUN: core::cell::Cell<(u64, u32)> = const { core::cell::Cell::new((0, 0)) };
    /// Precise guest faults: (address, write) of the current instruction's first invalid access.
    static FAULT_PENDING: core::cell::Cell<Option<(u64, bool)>> = const { core::cell::Cell::new(None) };
}

/// Precise guest faults (target/review/exec/g18/seh_guard_plan.md): set only by the dll_lift
/// difftest child (`AW2_DLL_DIFF_PRECISE=1`) or `regfile::run_oracle`, default off.
/// Bit 0 = global [`set_precise_faults`] flag; bits 1..31 = active non-test oracle depth
/// ([`enter_precise_oracle`]/[`leave_precise_oracle`]) so production builds never pay a TLS
/// `_tlv_get_addr` call on every guest memory store.
static PRECISE_FAULTS: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);

/// The guest time-stamp counter: `rdtsc` / `rdtscp` return this constant (edx:eax; rdtscp
/// ecx = TSC_AUX = 0) on every emulator path (interpreter, the fast path's generic fallback,
/// and the JIT's inline `movz`, `jit_more.rs` kind 8, which must stay equal to it), and the
/// lifted code's `rdtsc` ([`guest_rdtsc`], dll_lift). A real monotonic counter is a deferred,
/// gameplay-visible option (g19 frontier_plan.md).
pub const GUEST_TSC: u64 = 0x1000;

/// [`GUEST_TSC`] for lifted code (`rdtsc` -> rdx:rax).
#[inline(always)]
pub fn guest_rdtsc() -> u64 {
    GUEST_TSC
}

/// Turn precise guest faults on / off for runs that start afterwards (difftest child only).
pub fn set_precise_faults(on: bool) {
    if on {
        PRECISE_FAULTS.fetch_or(1, core::sync::atomic::Ordering::Relaxed);
    } else {
        PRECISE_FAULTS.fetch_and(!1, core::sync::atomic::Ordering::Relaxed);
    }
}

/// Increment active non-test oracle precise-fault depth (`regfile::run_oracle`).
#[cfg(not(test))]
#[inline]
pub(super) fn enter_precise_oracle() {
    PRECISE_FAULTS.fetch_add(2, core::sync::atomic::Ordering::Relaxed);
}

/// Decrement active non-test oracle precise-fault depth (`regfile::run_oracle`).
#[cfg(not(test))]
#[inline]
pub(super) fn leave_precise_oracle() {
    PRECISE_FAULTS.fetch_sub(2, core::sync::atomic::Ordering::Relaxed);
}

#[cfg(test)]
std::thread_local! {
    /// Lib tests: precise guest faults on this thread only (tests run in parallel).
    static PRECISE_LOCAL: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
}

/// Lib tests: [`set_precise_faults`] for this thread only.
#[cfg(test)]
pub(crate) fn set_precise_faults_local(on: bool) {
    PRECISE_LOCAL.with(|p| p.set(on));
}

/// Whether runs starting now use precise guest faults.
#[inline(always)]
pub fn precise_faults() -> bool {
    #[cfg(test)]
    {
        PRECISE_FAULTS.load(core::sync::atomic::Ordering::Relaxed) != 0 || PRECISE_LOCAL.with(|p| p.get())
    }
    #[cfg(not(test))]
    {
        PRECISE_FAULTS.load(core::sync::atomic::Ordering::Relaxed) != 0
    }
}

/// Access violations dispatched from guest instruction boundaries (precise mode), process-wide.
pub static PRECISE_FAULT_DISPATCHES: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);

/// Precise mode: dispatch STATUS_ACCESS_VIOLATION (Information = [write, address]) from the
/// guest state in `cpu` (rip = the faulting instruction, registers as before it). The raise is
/// an [`eh::callout`] of `RaiseException` itself, so the dispatcher takes this region's guest
/// context as the raise context (ExceptionAddress / ControlPc = the instruction). A landing in
/// this region loads its state into `cpu`; a continued exception leaves `cpu` as it was (the
/// instruction runs again, as on Windows). Nested runs (filters) start below the guest rsp.
#[cold]
#[inline(never)]
unsafe fn guest_fault_dispatch(cpu: &mut Cpu, sp_ptr: *mut u64, addr: u64, write: bool) {
    PRECISE_FAULT_DISPATCHES.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    crate::dll_native::lift::eh::note_seh_fault(addr);
    let saved_sp = *sp_ptr;
    *sp_ptr = (cpu.gpr[4].wrapping_sub(256)) & !0xFu64;
    let raise = win32_rt::unwind::RaiseException as *const () as usize;
    let info = [write as u64, addr];
    let lsa = crate::lstack::anchor_enter(crate::callctx::ctx(), *sp_ptr);
    let _ = eh::callout(cpu, raise, *sp_ptr, move || {
        win32_rt::unwind::raise_exception_raw(
            win32_rt::unwind::STATUS_ACCESS_VIOLATION as u64,
            0,
            2,
            info.as_ptr() as u64,
        );
        (0, 0)
    });
    crate::lstack::anchor_leave(crate::callctx::ctx(), lsa);
    *sp_ptr = saved_sp;
    cpu.last_bits = [64; 16];
}

/// A guest access outside `[0x10000, 2^47)`. The access reads 0 / is dropped, unless:
/// - the address is in the null page (`< 0x10000`), or
/// - this is the `BAD_ADDR_RUN_LIMIT`th invalid access in a row at the same `rip`;
/// then it raises `STATUS_ACCESS_VIOLATION` (vectored handlers, then the unhandled path;
/// an access violation is not resumable mid-instruction, so a handler asking to continue
/// is logged and the process exits). The WARN is rate-limited (first 32, then powers of 2).
#[cold]
#[inline(never)]
fn cold_bad_addr(cpu: &Cpu, what: &str, addr: u64, bits: u32, write: bool) {
    use core::sync::atomic::{AtomicU64, Ordering};
    static SEEN: AtomicU64 = AtomicU64::new(0);
    let n = SEEN.fetch_add(1, Ordering::Relaxed) + 1;
    let rip = cpu.rip;
    if n <= 32 || n.is_power_of_two() {
        log::warn!("emu: {what} invalid addr={addr:#x} (bits={bits}) around rip={rip:#x} [{n} invalid accesses so far]");
    }
    if precise_faults() {
        // Raised at the end of the instruction (see `PRECISE_FAULTS`); the first access counts.
        FAULT_PENDING.with(|p| {
            if p.get().is_none() {
                p.set(Some((addr, write)));
            }
        });
        return;
    }
    if bad_addr_ignored() {
        return;
    }
    let run = BAD_ADDR_RUN.with(|r| {
        let (last, k) = r.get();
        let k = if last == rip { k.saturating_add(1) } else { 1 };
        r.set((rip, k));
        k
    });
    if addr >= 0x1_0000 && run < BAD_ADDR_RUN_LIMIT {
        return;
    }
    log::error!(
        "emu: raising an access violation: {what} addr={addr:#x} (bits={bits}) at rip={rip:#x} (next instruction){}",
        if addr < 0x1_0000 { " (null page)".to_string() } else { format!(" after {run} consecutive invalid accesses there") }
    );
    unsafe {
        let mut rec = win32_rt::unwind::guest_exception_record(
            win32_rt::unwind::STATUS_ACCESS_VIOLATION,
            rip,
            &[write as u64, addr],
        );
        let mut ctx = win32_rt::unwind::guest_context(&cpu.gpr, &cpu.xmm, rip, cpu.rflags());
        if win32_rt::unwind::dispatch_veh_guest(&mut rec, &mut ctx) {
            log::error!("emu: a vectored handler asked to continue an emulated access violation; not resumable mid-instruction");
        }
        win32_rt::unwind::emu_unhandled_exception(&mut rec, &mut ctx, false);
    }
}

/// `Register as u8` -> packed `(gpr index, width, high-byte)` (0 = not a GPR). See [`Cpu::reg_idx`].
static GPR_TABLE: [u8; 256] = build_gpr_table();

const fn build_gpr_table() -> [u8; 256] {
    const fn pack(idx: u8, w: u8, hi: bool) -> u8 {
        0x80 | idx | (w << 4) | if hi { 0x40 } else { 0 }
    }
    let mut t = [0u8; 256];
    let r64 = [
        Register::RAX, Register::RCX, Register::RDX, Register::RBX, Register::RSP, Register::RBP, Register::RSI, Register::RDI,
        Register::R8, Register::R9, Register::R10, Register::R11, Register::R12, Register::R13, Register::R14, Register::R15,
    ];
    let r32 = [
        Register::EAX, Register::ECX, Register::EDX, Register::EBX, Register::ESP, Register::EBP, Register::ESI, Register::EDI,
        Register::R8D, Register::R9D, Register::R10D, Register::R11D, Register::R12D, Register::R13D, Register::R14D, Register::R15D,
    ];
    let r16 = [
        Register::AX, Register::CX, Register::DX, Register::BX, Register::SP, Register::BP, Register::SI, Register::DI,
        Register::R8W, Register::R9W, Register::R10W, Register::R11W, Register::R12W, Register::R13W, Register::R14W, Register::R15W,
    ];
    let r8 = [
        Register::AL, Register::CL, Register::DL, Register::BL, Register::SPL, Register::BPL, Register::SIL, Register::DIL,
        Register::R8L, Register::R9L, Register::R10L, Register::R11L, Register::R12L, Register::R13L, Register::R14L, Register::R15L,
    ];
    let mut i = 0;
    while i < 16 {
        t[r64[i] as usize] = pack(i as u8, 3, false);
        t[r32[i] as usize] = pack(i as u8, 2, false);
        t[r16[i] as usize] = pack(i as u8, 1, false);
        t[r8[i] as usize] = pack(i as u8, 0, false);
        i += 1;
    }
    t[Register::AH as usize] = pack(0, 0, true);
    t[Register::CH as usize] = pack(1, 0, true);
    t[Register::DH as usize] = pack(2, 0, true);
    t[Register::BH as usize] = pack(3, 0, true);
    t
}

impl Cpu {
    /// x86 PF: even parity of the low result byte.
    #[inline(always)]
    fn pf(&self) -> bool {
        self.pfv.count_ones() & 1 == 0
    }

    #[inline(always)]
    fn set_pf(&mut self, pf: bool) {
        self.pfv = (!pf) as u8;
    }

    #[inline(always)]
    fn reg_idx(r: Register) -> Option<(usize, u32, bool)> {
        let e = GPR_TABLE[r as usize];
        if e == 0 {
            return None;
        }
        // Packed: bits 0..4 = gpr index, bits 4..6 = width code (0:8,1:16,2:32,3:64), bit 6 = high-byte, bit 7 = valid.
        let idx = (e & 0x0F) as usize;
        let bits = 8u32 << ((e >> 4) & 0x3);
        Some((idx, bits, (e & 0x40) != 0))
    }

    #[inline(always)]
    fn xmm_idx(r: Register) -> Option<usize> {
        let idx = match r {
            Register::XMM0 | Register::YMM0 => 0, Register::XMM1 | Register::YMM1 => 1,
            Register::XMM2 | Register::YMM2 => 2, Register::XMM3 | Register::YMM3 => 3,
            Register::XMM4 | Register::YMM4 => 4, Register::XMM5 | Register::YMM5 => 5,
            Register::XMM6 | Register::YMM6 => 6, Register::XMM7 | Register::YMM7 => 7,
            Register::XMM8 | Register::YMM8 => 8, Register::XMM9 | Register::YMM9 => 9,
            Register::XMM10 | Register::YMM10 => 10, Register::XMM11 | Register::YMM11 => 11,
            Register::XMM12 | Register::YMM12 => 12, Register::XMM13 | Register::YMM13 => 13,
            Register::XMM14 | Register::YMM14 => 14, Register::XMM15 | Register::YMM15 => 15,
            _ => return Option::None,
        };
        Some(idx)
    }

    #[inline(always)]
    fn is_ymm(r: Register) -> bool {
        matches!(
            r,
            Register::YMM0
                | Register::YMM1
                | Register::YMM2
                | Register::YMM3
                | Register::YMM4
                | Register::YMM5
                | Register::YMM6
                | Register::YMM7
                | Register::YMM8
                | Register::YMM9
                | Register::YMM10
                | Register::YMM11
                | Register::YMM12
                | Register::YMM13
                | Register::YMM14
                | Register::YMM15
        )
    }

    #[inline(always)]
    fn get_reg(&self, r: Register) -> u64 {
        let Some((idx, bits, hi8)) = Self::reg_idx(r) else { return 0 };
        let v = self.gpr[idx];
        match (bits, hi8) {
            (64, _) => v,
            (32, _) => v as u32 as u64,
            (16, _) => v as u16 as u64,
            (8, false) => v as u8 as u64,
            (8, true) => ((v >> 8) & 0xFF) as u64,
            _ => v,
        }
    }

    #[inline(always)]
    fn set_reg(&mut self, r: Register, val: u64) {
        let Some((idx, bits, hi8)) = Self::reg_idx(r) else { return };
        self.last_bits[idx] = match (bits, hi8) {
            (8, false) => 8,
            (16, _) => 16,
            _ => 64,
        };
        match (bits, hi8) {
            (64, _) => self.gpr[idx] = val,
            (32, _) => self.gpr[idx] = val as u32 as u64,
            (16, _) => self.gpr[idx] = (self.gpr[idx] & !0xFFFF) | (val & 0xFFFF),
            (8, false) => self.gpr[idx] = (self.gpr[idx] & !0xFF) | (val & 0xFF),
            (8, true) => self.gpr[idx] = (self.gpr[idx] & !0xFF00) | ((val & 0xFF) << 8),
            _ => {}
        }
    }

    #[inline(always)]
    fn abi_gpr(&self, idx: usize) -> u64 {
        match self.last_bits[idx] {
            8 => self.gpr[idx] & 0xFF,
            16 => self.gpr[idx] & 0xFFFF,
            _ => self.gpr[idx],
        }
    }

    #[inline(always)]
    pub(crate) unsafe fn eff_addr(&self, ins: &Instruction) -> u64 {
        if ins.is_ip_rel_memory_operand() {
            return ins.ip_rel_memory_address();
        }
        let mut addr: u64 = ins.memory_displacement64();
        let base_reg = ins.memory_base();
        let mut is_a32 = false;
        if base_reg != Register::None {
            if base_reg == Register::RIP {
                addr = addr.wrapping_add(ins.next_ip());
            } else {
                if matches!(Self::reg_idx(base_reg), Some((_, 32, _))) {
                    is_a32 = true;
                }
                addr = addr.wrapping_add(self.get_reg(base_reg));
            }
        }
        let index = ins.memory_index();
        if index != Register::None {
            if matches!(Self::reg_idx(index), Some((_, 32, _))) {
                is_a32 = true;
            }
            addr = addr.wrapping_add(self.get_reg(index).wrapping_mul(ins.memory_index_scale() as u64));
        }
        if is_a32 {
            addr = (addr as u32) as u64;
        }
        if ins.segment_prefix() == Register::GS {
            let teb = teb_self() as u64;
            addr = teb.wrapping_add(addr);
        }
        addr
    }

    #[inline(always)]
    unsafe fn read_mem(&self, addr: u64, bits: u32) -> u64 {
        let bytes = (bits as usize / 8).clamp(1, 8);
        if !regfile_ext::is_valid_guest_range(addr, bytes) {
            cold_bad_addr(self, "read_mem", addr, bits, false);
            return 0;
        }
        let p = addr as usize;
        if mem::tso() {
            return mem::load_acq(p, if bits > 64 { 64 } else { bits });
        }
        match bits {
            8 => rd::<u8>(p) as u64,
            16 => rd::<u16>(p) as u64,
            32 => rd::<u32>(p) as u64,
            64 => rd::<u64>(p),
            _ => rd::<u64>(p),
        }
    }

    #[inline(always)]
    unsafe fn write_mem(&self, addr: u64, bits: u32, val: u64) {
        if precise_faults() && FAULT_PENDING.with(|p| p.get().is_some()) {
            return;
        }
        let bytes = (bits as usize / 8).clamp(1, 8);
        if !regfile_ext::is_valid_guest_range(addr, bytes) {
            cold_bad_addr(self, "write_mem", addr, bits, true);
            return;
        }
        let p = addr as usize;
        if mem::tso() {
            return mem::store_rel(p, if bits > 64 { 64 } else { bits }, val);
        }
        match bits {
            8 => wr::<u8>(p, val as u8),
            16 => wr::<u16>(p, val as u16),
            32 => wr::<u32>(p, val as u32),
            64 => wr::<u64>(p, val),
            _ => wr::<u64>(p, val),
        }
    }

    #[inline]
    unsafe fn read_u128(&self, addr: u64) -> u128 {
        if !regfile_ext::is_valid_guest_range(addr, 16) {
            cold_bad_addr(self, "read_u128", addr, 128, false);
            return 0;
        }
        let p = addr as usize;
        if mem::tso() {
            return mem::load128_acq(p);
        }
        rd::<u128>(p)
    }

    #[inline]
    unsafe fn write_u128(&self, addr: u64, val: u128) {
        if precise_faults() && FAULT_PENDING.with(|p| p.get().is_some()) {
            return;
        }
        if !regfile_ext::is_valid_guest_range(addr, 16) {
            cold_bad_addr(self, "write_u128", addr, 128, true);
            return;
        }
        let p = addr as usize;
        if mem::tso() {
            return mem::store128_rel(p, val);
        }
        wr::<u128>(p, val);
    }

    /// `true` for VEX-encoded instructions (a VEX.128 write zeroes bits 255:128 of the
    /// destination; a legacy SSE write preserves them).
    #[inline(always)]
    fn is_vex(ins: &Instruction) -> bool {
        ins.encoding() == EncodingKind::VEX
    }

    /// Read vector source operand `op` (XMM/YMM register or memory) as `(bits 127:0, bits 255:128)`.
    /// Memory operands read exactly `memory_size()` bytes (so `m32`/`m64` sources never over-read);
    /// the upper half is only produced when `ymm` is set (register) / the operand is 256-bit (memory).
    #[inline]
    unsafe fn vsrc(&self, ins: &Instruction, op: u32, ymm: bool) -> (u128, u128) {
        match ins.op_kind(op) {
            OpKind::Register => match Self::xmm_idx(ins.op_register(op)) {
                Some(i) => (self.xmm[i], if ymm { self.ymm_hi[i] } else { 0 }),
                None => (0, 0),
            },
            OpKind::Memory => {
                let a = self.eff_addr(ins);
                let n = ins.memory_size().size();
                let lo = match n {
                    1 => self.read_mem(a, 8) as u128,
                    2 => self.read_mem(a, 16) as u128,
                    4 => self.read_mem(a, 32) as u128,
                    8 => self.read_mem(a, 64) as u128,
                    _ => self.read_u128(a),
                };
                let hi = if ymm && n >= 32 { self.read_u128(a.wrapping_add(16)) } else { 0 };
                (lo, hi)
            }
            _ => (0, 0),
        }
    }

    /// Write a full-width vector result to XMM/YMM register `d` (operand 0): YMM destinations
    /// get `hi`, VEX.128 destinations get their upper half zeroed, legacy SSE preserves it.
    #[inline]
    fn vdst(&mut self, ins: &Instruction, d: usize, lo: u128, hi: u128) {
        self.xmm[d] = lo;
        if Self::is_ymm(ins.op_register(0)) {
            self.set_ymm_hi(d, hi);
        } else if Self::is_vex(ins) {
            self.clear_ymm_hi_reg(d);
        }
    }

    /// VEX.128 forms zero bits 255:128 of the destination register.
    #[inline(always)]
    fn vex128_zero_upper(&mut self, ins: &Instruction, d: usize) {
        if Self::is_vex(ins) && !Self::is_ymm(ins.op_register(0)) {
            self.clear_ymm_hi_reg(d);
        }
    }

    /// x86 `lock`-prefixed read-modify-write: atomic and a full memory barrier. An aarch64
    /// `casal` alone is not a full barrier (a later load may be satisfied before its store is
    /// visible), which breaks StoreLoad-dependent guest code such as work-stealing deque pops
    /// (`lock xadd [bottom]` then load `top`), so follow it with a `dmb ish`.
    #[inline]
    unsafe fn atomic_rmw(&self, addr: u64, bits: u32, f: impl FnMut(u64) -> u64) -> (u64, u64) {
        let r = self.atomic_rmw_raw(addr, bits, f);
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        r
    }

    /// `lock cmpxchg`: atomic compare-exchange plus a full barrier (see [`Self::atomic_rmw`]).
    #[inline]
    unsafe fn atomic_cmpxchg(&self, addr: u64, bits: u32, expected: u64, desired: u64) -> (bool, u64) {
        let r = self.atomic_cmpxchg_raw(addr, bits, expected, desired);
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        r
    }

    #[inline]
    unsafe fn atomic_rmw_raw(&self, addr: u64, bits: u32, mut f: impl FnMut(u64) -> u64) -> (u64, u64) {
        use core::sync::atomic::{AtomicU16, AtomicU32, AtomicU64, AtomicU8, Ordering};
        let align = (bits as usize / 8).clamp(1, 8);
        if !regfile_ext::is_valid_guest_range(addr, align) {
            cold_bad_addr(self, "atomic_rmw", addr, bits, true);
            return (0, 0);
        }
        let p = addr as usize;
        if (p & (align - 1)) == 0 {
            match bits {
                8 => {
                    let a = &*(p as *const AtomicU8);
                    let mut cur = a.load(Ordering::Relaxed);
                    loop {
                        let next = f(cur as u64) as u8;
                        match a.compare_exchange_weak(cur, next, Ordering::SeqCst, Ordering::Relaxed) {
                            Ok(_) => return (cur as u64, next as u64),
                            Err(actual) => cur = actual,
                        }
                    }
                }
                16 => {
                    let a = &*(p as *const AtomicU16);
                    let mut cur = a.load(Ordering::Relaxed);
                    loop {
                        let next = f(cur as u64) as u16;
                        match a.compare_exchange_weak(cur, next, Ordering::SeqCst, Ordering::Relaxed) {
                            Ok(_) => return (cur as u64, next as u64),
                            Err(actual) => cur = actual,
                        }
                    }
                }
                32 => {
                    let a = &*(p as *const AtomicU32);
                    let mut cur = a.load(Ordering::Relaxed);
                    loop {
                        let next = f(cur as u64) as u32;
                        match a.compare_exchange_weak(cur, next, Ordering::SeqCst, Ordering::Relaxed) {
                            Ok(_) => return (cur as u64, next as u64),
                            Err(actual) => cur = actual,
                        }
                    }
                }
                _ => {
                    let a = &*(p as *const AtomicU64);
                    let mut cur = a.load(Ordering::Relaxed);
                    loop {
                        let next = f(cur);
                        match a.compare_exchange_weak(cur, next, Ordering::SeqCst, Ordering::Relaxed) {
                            Ok(_) => return (cur, next),
                            Err(actual) => cur = actual,
                        }
                    }
                }
            }
        } else {
            static UNALIGNED_ATOMIC_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
            let _g = UNALIGNED_ATOMIC_LOCK.lock().unwrap();
            let old = self.read_mem(addr, bits);
            let new = f(old);
            self.write_mem(addr, bits, new);
            (old, new)
        }
    }

    #[inline]
    unsafe fn atomic_cmpxchg_raw(&self, addr: u64, bits: u32, expected: u64, desired: u64) -> (bool, u64) {
        use core::sync::atomic::{AtomicU16, AtomicU32, AtomicU64, AtomicU8, Ordering};
        let align = (bits as usize / 8).clamp(1, 8);
        if !regfile_ext::is_valid_guest_range(addr, align) {
            cold_bad_addr(self, "atomic_cmpxchg", addr, bits, true);
            return (false, 0);
        }
        let p = addr as usize;
        if (p & (align - 1)) == 0 {
            match bits {
                8 => {
                    let a = &*(p as *const AtomicU8);
                    match a.compare_exchange(expected as u8, desired as u8, Ordering::SeqCst, Ordering::SeqCst) {
                        Ok(v) => (true, v as u64),
                        Err(v) => (false, v as u64),
                    }
                }
                16 => {
                    let a = &*(p as *const AtomicU16);
                    match a.compare_exchange(expected as u16, desired as u16, Ordering::SeqCst, Ordering::SeqCst) {
                        Ok(v) => (true, v as u64),
                        Err(v) => (false, v as u64),
                    }
                }
                32 => {
                    let a = &*(p as *const AtomicU32);
                    match a.compare_exchange(expected as u32, desired as u32, Ordering::SeqCst, Ordering::SeqCst) {
                        Ok(v) => (true, v as u64),
                        Err(v) => (false, v as u64),
                    }
                }
                _ => {
                    let a = &*(p as *const AtomicU64);
                    match a.compare_exchange(expected, desired, Ordering::SeqCst, Ordering::SeqCst) {
                        Ok(v) => (true, v),
                        Err(v) => (false, v),
                    }
                }
            }
        } else {
            static UNALIGNED_CAS_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
            let _g = UNALIGNED_CAS_LOCK.lock().unwrap();
            let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
            let cur = self.read_mem(addr, bits) & mask;
            if cur == (expected & mask) {
                self.write_mem(addr, bits, desired);
                (true, cur)
            } else {
                (false, cur)
            }
        }
    }

    #[inline(always)]
    fn op_bits(ins: &Instruction, op: u32) -> u32 {
        match ins.op_kind(op) {
            OpKind::Register => {
                let r = ins.op_register(op);
                if let Some((_, bits, _)) = Self::reg_idx(r) {
                    bits
                } else {
                    128
                }
            }
            OpKind::Memory => (ins.memory_size().size() as u32 * 8).max(8),
            OpKind::Immediate8 => 8,
            OpKind::Immediate16 => 16,
            OpKind::Immediate32 => 32,
            OpKind::Immediate64 | OpKind::Immediate8to64 | OpKind::Immediate32to64 => 64,
            OpKind::Immediate8to16 => 16,
            OpKind::Immediate8to32 => 32,
            _ => 64,
        }
    }

    #[inline(always)]
    unsafe fn read_op(&self, ins: &Instruction, op: u32, default_bits: u32) -> u64 {
        let raw = match ins.op_kind(op) {
            OpKind::Register => return self.get_reg(ins.op_register(op)),
            OpKind::Memory => {
                let a = self.eff_addr(ins);
                let b = (ins.memory_size().size() as u32 * 8).max(default_bits);
                return self.read_mem(a, b);
            }
            OpKind::Immediate8 => ins.immediate8() as u64,
            OpKind::Immediate16 => ins.immediate16() as u64,
            OpKind::Immediate32 => ins.immediate32() as u64,
            OpKind::Immediate64 => ins.immediate64(),
            OpKind::Immediate8to16 => ins.immediate8to16() as u16 as u64,
            OpKind::Immediate8to32 => ins.immediate8to32() as u32 as u64,
            OpKind::Immediate8to64 => ins.immediate8to64() as u64,
            OpKind::Immediate32to64 => ins.immediate32to64() as u64,
            _ => 0,
        };
        if default_bits < 64 {
            raw & ((1u64 << default_bits) - 1)
        } else {
            raw
        }
    }

    #[inline(always)]
    unsafe fn write_op(&mut self, ins: &Instruction, op: u32, bits: u32, val: u64) {
        match ins.op_kind(op) {
            OpKind::Register => self.set_reg(ins.op_register(op), val),
            OpKind::Memory => {
                let a = self.eff_addr(ins);
                self.write_mem(a, bits, val);
            }
            _ => {}
        }
    }

    #[inline(always)]
    fn update_flags_logic(&mut self, res: u64, bits: u32) {
        let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
        let r = res & mask;
        let sign_bit = 1u64 << (bits - 1);
        self.cf = false;
        self.of = false;
        self.zf = r == 0;
        self.sf = (r & sign_bit) != 0;
        self.pfv = r as u8;
    }

    #[inline(always)]
    fn update_flags_add(&mut self, a: u64, b: u64, bits: u32) -> u64 {
        let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
        let ua = a & mask;
        let ub = b & mask;
        let sign_bit = 1u64 << (bits - 1);
        let res = ua.wrapping_add(ub) & mask;
        self.cf = res < ua;
        self.zf = res == 0;
        self.sf = (res & sign_bit) != 0;
        self.of = ((ua ^ res) & (ub ^ res) & sign_bit) != 0;
        self.pfv = res as u8;
        res
    }

    #[inline(always)]
    fn update_flags_sub(&mut self, a: u64, b: u64, bits: u32) -> u64 {
        let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
        let ua = a & mask;
        let ub = b & mask;
        let sign_bit = 1u64 << (bits - 1);
        let res = ua.wrapping_sub(ub) & mask;
        self.cf = ua < ub;
        self.zf = res == 0;
        self.sf = (res & sign_bit) != 0;
        self.of = ((ua ^ ub) & (ua ^ res) & sign_bit) != 0;
        self.pfv = res as u8;
        res
    }

    #[inline(always)]
    fn eval_cc(&self, cc: ConditionCode) -> bool {
        match cc {
            ConditionCode::o => self.of,
            ConditionCode::no => !self.of,
            ConditionCode::b => self.cf,
            ConditionCode::ae => !self.cf,
            ConditionCode::e => self.zf,
            ConditionCode::ne => !self.zf,
            ConditionCode::be => self.cf || self.zf,
            ConditionCode::a => !self.cf && !self.zf,
            ConditionCode::s => self.sf,
            ConditionCode::ns => !self.sf,
            ConditionCode::p => self.pf(),
            ConditionCode::np => !self.pf(),
            ConditionCode::l => self.sf != self.of,
            ConditionCode::ge => self.sf == self.of,
            ConditionCode::le => self.zf || (self.sf != self.of),
            ConditionCode::g => !self.zf && (self.sf == self.of),
            ConditionCode::None => false,
        }
    }

    #[inline(always)]
    pub fn rflags(&self) -> u32 {
        0x202
            | (self.cf as u32)
            | ((self.pf() as u32) << 2)
            | ((self.zf as u32) << 6)
            | ((self.sf as u32) << 7)
            | ((self.df as u32) << 10)
            | ((self.of as u32) << 11)
    }

    #[inline(always)]
    pub fn set_rflags(&mut self, f: u32) {
        self.cf = (f & 1) != 0;
        self.set_pf(((f >> 2) & 1) != 0);
        self.zf = ((f >> 6) & 1) != 0;
        self.sf = ((f >> 7) & 1) != 0;
        self.df = ((f >> 10) & 1) != 0;
        self.of = ((f >> 11) & 1) != 0;
    }
}

const GUEST_STACK_QWORDS: usize = 1024 * 1024; // 8 MB per thread

// ---------------------------------------------------------------------------------------------
// x86 SSE numeric semantics (MXCSR default: all exceptions masked, round-to-nearest-even).
// ---------------------------------------------------------------------------------------------

/// x86 NaN result rules for SSE arithmetic: a NaN source is propagated quieted (the first source
/// wins when both are NaN); an invalid operation on non-NaN inputs yields the negative "real
/// indefinite" QNaN (`0xFFC00000`), whereas aarch64 would produce the positive default NaN.
#[inline(always)]
fn x86_nan32(a: f32, b: f32, r: f32) -> f32 {
    if !r.is_nan() {
        r
    } else if a.is_nan() {
        f32::from_bits(a.to_bits() | 0x0040_0000)
    } else if b.is_nan() {
        f32::from_bits(b.to_bits() | 0x0040_0000)
    } else {
        f32::from_bits(0xFFC0_0000)
    }
}

/// See [`x86_nan32`].
#[inline(always)]
fn x86_nan64(a: f64, b: f64, r: f64) -> f64 {
    if !r.is_nan() {
        r
    } else if a.is_nan() {
        f64::from_bits(a.to_bits() | 0x0008_0000_0000_0000)
    } else if b.is_nan() {
        f64::from_bits(b.to_bits() | 0x0008_0000_0000_0000)
    } else {
        f64::from_bits(0xFFF8_0000_0000_0000)
    }
}

/// `addss/subss/mulss/divss/minss/maxss` (and the packed lanes) with x86 semantics.
/// `op`: 0 add, 1 sub, 2 mul, 3 div, 4 min, 5 max. MIN/MAX return the second operand when
/// either input is NaN or both are zero (of either sign).
#[inline(always)]
fn sse_arith32(op: u8, a: f32, b: f32) -> f32 {
    match op {
        0 => x86_nan32(a, b, a + b),
        1 => x86_nan32(a, b, a - b),
        2 => x86_nan32(a, b, a * b),
        3 => x86_nan32(a, b, a / b),
        4 => if a < b { a } else { b },
        _ => if a > b { a } else { b },
    }
}

/// See [`sse_arith32`].
#[inline(always)]
fn sse_arith64(op: u8, a: f64, b: f64) -> f64 {
    match op {
        0 => x86_nan64(a, b, a + b),
        1 => x86_nan64(a, b, a - b),
        2 => x86_nan64(a, b, a * b),
        3 => x86_nan64(a, b, a / b),
        4 => if a < b { a } else { b },
        _ => if a > b { a } else { b },
    }
}

#[inline(always)]
fn sse_sqrt32(a: f32) -> f32 {
    x86_nan32(a, a, a.sqrt())
}

#[inline(always)]
fn sse_sqrt64(a: f64) -> f64 {
    x86_nan64(a, a, a.sqrt())
}

/// `cvt(t)ss2si` / `cvt(t)sd2si` to a 32-bit integer: NaN and out-of-range inputs produce the
/// "integer indefinite" value `0x80000000`. Non-truncating forms round to nearest-even (MXCSR).
#[inline(always)]
fn sse_cvt_i32(f: f64, trunc: bool) -> u32 {
    let r = if trunc { f.trunc() } else { f.round_ties_even() };
    if r.is_nan() || r >= 2147483648.0 || r < -2147483648.0 {
        0x8000_0000
    } else {
        r as i32 as u32
    }
}

/// 64-bit counterpart of [`sse_cvt_i32`] (indefinite = `0x8000000000000000`).
#[inline(always)]
fn sse_cvt_i64(f: f64, trunc: bool) -> u64 {
    let r = if trunc { f.trunc() } else { f.round_ties_even() };
    if r.is_nan() || r >= 9223372036854775808.0 || r < -9223372036854775808.0 {
        0x8000_0000_0000_0000
    } else {
        r as i64 as u64
    }
}

/// `cmpps`/`cmpss`/`vcmpXX` predicate evaluation (imm8[3:0]; imm8[4] only selects signalling
/// vs quiet behaviour, which does not affect the result with exceptions masked).
#[inline(always)]
fn sse_cmp_pred(fa: f64, fb: f64, pred: u8) -> bool {
    let unord = fa.is_nan() || fb.is_nan();
    match pred & 0x0F {
        0x00 => !unord && fa == fb, // EQ_OQ
        0x01 => !unord && fa < fb,  // LT_OS
        0x02 => !unord && fa <= fb, // LE_OS
        0x03 => unord,              // UNORD_Q
        0x04 => unord || fa != fb,  // NEQ_UQ
        0x05 => unord || fa >= fb,  // NLT_US
        0x06 => unord || fa > fb,   // NLE_US
        0x07 => !unord,             // ORD_Q
        0x08 => unord || fa == fb,  // EQ_UQ
        0x09 => unord || fa < fb,   // NGE_US
        0x0A => unord || fa <= fb,  // NGT_US
        0x0B => false,              // FALSE_OQ
        0x0C => !unord && fa != fb, // NEQ_OQ
        0x0D => !unord && fa >= fb, // GE_OS
        0x0E => !unord && fa > fb,  // GT_OS
        _ => true,                  // TRUE_UQ
    }
}

/// Decoded-instruction arena size per thread (instructions): 72 bytes each (iced
/// `Instruction` + [`fast::FastOp`]). On macOS this is 1M instructions (72 MB virtual,
/// lazily committed); on iOS (6 GiB total Jetsam cap with 30+ worker threads running
/// static AOT), cap the fallback interpreter cache at 64K instructions (~4.5 MiB max)
/// and 16K index entries (512 KiB max).
#[cfg(target_os = "ios")]
const BLK_ARENA_LEN: usize = 1 << 16;
#[cfg(not(target_os = "ios"))]
const BLK_ARENA_LEN: usize = 1 << 20;
#[cfg(target_os = "ios")]
const BLK_INDEX_BITS: usize = 14;
#[cfg(not(target_os = "ios"))]
const BLK_INDEX_BITS: usize = 18;
const BLK_INDEX_LEN: usize = 1 << BLK_INDEX_BITS;
/// Maximum straight-line instructions decoded into one block.
const BLK_MAX_INSNS: usize = 32;

#[derive(Clone, Copy, Default)]
#[repr(C, align(32))]
struct BlkEntry {
    ip: u64,
    off: u32,
    len: u32,
    /// 0 for blocks outside run-time generated code (never revalidated); otherwise the
    /// [`mem::code_gen`] the block was decoded or last revalidated in.
    gen: u32,
    /// Non-zero epoch matching [`BlockCache::fast_gen`] when this block is provably safe to
    /// chain directly inside [`interp_burst`] without re-checking host/JIT/route tables.
    fast_gen: u32,
    /// Direct block-chaining cache: `index` slot + 1 for the last taken branch/call/ret target
    /// (`0` = unset). Always validated against `(ip, fast_gen)` before use.
    taken_slot: u32,
    /// Direct block-chaining cache: `index` slot + 1 for the fallthrough target (`0` = unset).
    /// Always validated against `(ip, fast_gen)` before use.
    fall_slot: u32,
}
const _: () = assert!(core::mem::size_of::<BlkEntry>() == 32);

/// Per-thread decoded basic-block cache.
///
/// Blocks are runs of instructions starting at a guest IP and ending at the first
/// control-flow instruction (or [`BLK_MAX_INSNS`]), stored contiguously in an arena so the
/// interpreter walks decoded instructions linearly. The arena is never freed or reallocated;
/// when full it is reset and refilled from the start. Readers (including outer interpreter
/// frames on the same thread that re-entered via a host callback) always check
/// `Instruction::ip()` against the current RIP before using a cached instruction, so a
/// reused arena slot can never be executed at the wrong address.
struct BlockCache {
    arena: Box<[Instruction]>,
    ops: Box<[fast::FastOp]>,
    index: Box<[BlkEntry]>,
    cursor: usize,
    rcache: Box<[RcEntry]>,
    /// Current fast-chain epoch (never 0 once synchronized); bumped whenever `fast_key` changes.
    fast_gen: u32,
    /// Epoch key `(va_gen, static_aot_gen, code_gen, aot_epoch, mode_flags)` for `fast_gen`.
    fast_key: (u32, u32, u32, u32, u32),
}

/// Host-callable resolution cache entry (see [`BlockCache::resolve`]).
#[derive(Clone, Copy, Default)]
struct RcEntry {
    target: u64,
    host: usize,
    gen: u32,
}
const RC_BITS: usize = 12;
const RC_LEN: usize = 1 << RC_BITS;

/// `AW2_JIT_THUNK_ELIDE` (bit mask): guest callers (interpreter and translator, through
/// [`BlockCache::resolve`]) run the guest original of `va` instead of its registered host
/// replacement `h`. Host callers (`call_va!`) still use the replacement.
/// - bit `2`: game-registered re-entering allocator replacements (`hooks::register_reenter_alloc_vas`);
/// - unset / `0` (default): off. Other bits are ignored.
fn thunk_elided(va: u64, h: usize) -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| {
        let mode = std::env::var("AW2_JIT_THUNK_ELIDE").ok().and_then(|s| s.trim().parse::<u32>().ok()).unwrap_or(0);
        let on = mode & 2 != 0;
        if on {
            log::info!("emu: thunk elision on (AW2_JIT_THUNK_ELIDE={mode}): {} re-entering allocator VAs", crate::hooks::reenter_alloc_len());
        }
        on
    }) && crate::hooks::is_reenter_alloc_va(va, h)
}

/// Allocate a zero-filled boxed slice via `alloc_zeroed` (large sizes come straight from
/// fresh, lazily-committed zero pages).
///
/// # Safety
/// All-zero bytes must be a valid `T`.
unsafe fn zeroed_slice<T>(n: usize) -> Box<[T]> {
    let layout = std::alloc::Layout::array::<T>(n).expect("zeroed_slice layout");
    let p = std::alloc::alloc_zeroed(layout) as *mut T;
    if p.is_null() {
        std::alloc::handle_alloc_error(layout);
    }
    Box::from_raw(core::ptr::slice_from_raw_parts_mut(p, n))
}

enum BlkErr {
    BadRip(usize),
    Invalid([u8; 8]),
}

impl BlockCache {
    fn new() -> Self {
        // SAFETY: `Instruction`, `FastOp`, `BlkEntry` and `RcEntry` are plain data for which
        // all-zero bytes are valid (an empty/never-matching slot).
        unsafe {
            Self {
                arena: zeroed_slice::<Instruction>(BLK_ARENA_LEN),
                ops: zeroed_slice::<fast::FastOp>(BLK_ARENA_LEN),
                index: zeroed_slice::<BlkEntry>(BLK_INDEX_LEN),
                cursor: 0,
                rcache: zeroed_slice::<RcEntry>(RC_LEN),
                fast_gen: 0,
                fast_key: (0, 0, 0, 0, 0),
            }
        }
    }

    /// Synchronize the per-thread fast-chain epoch against
    /// `(exe::va_gen(), exec_mem::static_aot_gen(), mem::code_gen(), aot::epoch(), mode_flags)`.
    /// Returns a non-zero epoch token; any change to the 5-tuple bumps `fast_gen`, invalidating
    /// all previously stamped [`BlkEntry::fast_gen`] entries in O(1).
    #[inline(always)]
    fn sync_fast_gen(&mut self, mode_flags: u32) -> u32 {
        #[cfg(target_arch = "aarch64")]
        let aot_ep = aot::epoch();
        #[cfg(not(target_arch = "aarch64"))]
        let aot_ep = 0u32;
        let key = (
            crate::exe::va_gen(),
            exec_mem::static_aot_gen(),
            mem::code_gen(),
            aot_ep,
            mode_flags,
        );
        if key != self.fast_key {
            self.bump_fast_gen(key);
        }
        self.fast_gen
    }

    #[cold]
    #[inline(never)]
    fn bump_fast_gen(&mut self, key: (u32, u32, u32, u32, u32)) {
        self.fast_key = key;
        let next = self.fast_gen.wrapping_add(1);
        if next == 0 {
            for e in self.index.iter_mut() {
                e.fast_gen = 0;
            }
            self.fast_gen = 1;
        } else {
            self.fast_gen = next;
        }
    }

    /// `resolve_host_callable(target)` memoized per thread; entries are valid only for the
    /// dispatch generation they were filled in ([`crate::exe::va_gen`]), so any
    /// (un)registration invalidates them.
    #[inline(always)]
    unsafe fn resolve(this: *mut BlockCache, target: u64) -> Option<usize> {
        let gen = crate::exe::va_gen();
        let slot = (((target >> 4) ^ (target >> 16)) as usize) & (RC_LEN - 1);
        let e = (&mut *this).rcache.get_unchecked_mut(slot);
        if e.target == target && e.gen == gen {
            return if e.host != 0 { Some(e.host) } else { None };
        }
        let r = resolve_host_callable(target).filter(|&h| !thunk_elided(target, h));
        *e = RcEntry { target, host: r.unwrap_or(0), gen };
        r
    }

    #[inline(always)]
    fn slot(ip: u64) -> usize {
        ((ip as usize).wrapping_mul(0x9E37_79B9_7F4A_7C15) >> (64 - BLK_INDEX_BITS)) & (BLK_INDEX_LEN - 1)
    }

    /// Return the block starting at `ip` as `(arena offset, length >= 1)`.
    #[inline(always)]
    unsafe fn get(this: *mut BlockCache, ip: u64, misses: &mut u64) -> Result<(usize, usize), BlkErr> {
        let bc = &mut *this;
        let e = *bc.index.get_unchecked(Self::slot(ip));
        if e.ip == ip && e.len != 0 && bc.ops.get_unchecked(e.off as usize).ip == ip {
            if e.gen == 0 || e.gen == mem::code_gen() {
                return Ok((e.off as usize, e.len as usize));
            }
            return Self::revalidate(this, ip, e, misses);
        }
        Self::fill(this, ip, misses)
    }

    /// A cached block of run-time generated code whose executable memory changed since it
    /// was decoded: keep it if its bytes are unchanged, otherwise decode it again.
    #[cold]
    #[inline(never)]
    unsafe fn revalidate(this: *mut BlockCache, ip: u64, e: BlkEntry, misses: &mut u64) -> Result<(usize, usize), BlkErr> {
        let bc = &mut *this;
        let gen = mem::code_gen();
        let end = bc.arena.get_unchecked(e.off as usize + e.len as usize - 1).next_ip();
        if mem::jit_unchanged(ip, to_mapped_addr(ip), end.wrapping_sub(ip) as usize) {
            bc.index.get_unchecked_mut(Self::slot(ip)).gen = gen;
            return Ok((e.off as usize, e.len as usize));
        }
        Self::fill(this, ip, misses)
    }

    #[cold]
    #[inline(never)]
    unsafe fn fill(this: *mut BlockCache, ip: u64, misses: &mut u64) -> Result<(usize, usize), BlkErr> {
        let bc = &mut *this;
        if bc.cursor + BLK_MAX_INSNS > BLK_ARENA_LEN {
            bc.cursor = 0;
            bc.index.fill(BlkEntry::default());
        }
        let start = bc.cursor;
        let mapped = to_mapped_addr(ip);
        if !(0x1_0000..0x8000_0000_0000).contains(&mapped) {
            return Err(BlkErr::BadRip(mapped));
        }
        // Read the generation before the code bytes: a rewrite racing with this decode bumps
        // it afterwards, which forces a revalidation on the next lookup.
        let gen = if mem::in_jit(mapped) { mem::code_gen() } else { 0 };
        // One decoder for the whole block; it only reads the bytes of the instructions it
        // actually decodes (blocks are straight-line code within one image).
        let bytes = core::slice::from_raw_parts(mapped as *const u8, BLK_MAX_INSNS * 15);
        let mut dec = Decoder::with_ip(64, bytes, ip, DecoderOptions::NONE);
        let fast_on = fast_enabled();
        let mut n = 0usize;
        while n < BLK_MAX_INSNS {
            let slot = bc.arena.get_unchecked_mut(start + n);
            dec.decode_out(slot);
            if slot.is_invalid() {
                if n == 0 {
                    let mut b = [0u8; 8];
                    b.copy_from_slice(&bytes[..8]);
                    return Err(BlkErr::Invalid(b));
                }
                break;
            }
            let decoded = &*slot;
            let mut fop = if fast_on {
                fast::lower(decoded)
            } else {
                fast::FastOp { ip: decoded.ip(), len: decoded.len() as u8, ..Default::default() }
            };
            if fop.kind == fast::K_INSN {
                fop.imm = decoded as *const Instruction as u64;
            }
            *bc.ops.get_unchecked_mut(start + n) = fop;
            n += 1;
            *misses += 1;
            if decoded.flow_control() != FlowControl::Next {
                break;
            }
        }
        if gen != 0 || mem::codecheck() {
            let end = bc.arena.get_unchecked(start + n - 1).next_ip();
            let len = end.wrapping_sub(ip) as usize;
            if gen != 0 {
                mem::jit_record(ip, mapped, len);
            }
            if mem::codecheck() {
                mem::on_fill(ip, mapped, len);
            }
        }
        if fast_on {
            fast::fuse(&mut bc.ops[start..start + n]);
        }
        bc.cursor = start + n;
        *bc.index.get_unchecked_mut(Self::slot(ip)) = BlkEntry {
            ip,
            off: start as u32,
            len: n as u32,
            gen,
            fast_gen: 0,
            taken_slot: 0,
            fall_slot: 0,
        };
        Ok((start, n))
    }
}

/// `AW2_EMU_FAST=0` disables the pre-decoded fast path (everything runs generic).
fn fast_enabled() -> bool {
    #[cfg(test)]
    if TEST_FAST_OFF.with(|c| c.get()) {
        return false;
    }
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| std::env::var("AW2_EMU_FAST").map_or(true, |s| s.trim() != "0"))
}

#[cfg(test)]
std::thread_local! {
    /// Unit tests: decode blocks on this thread without fast ops (generic reference run).
    static TEST_FAST_OFF: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
}

/// `AW2_EMU_VERIFY=1` cross-checks every eligible fast op against the generic handler.
fn verify_enabled() -> bool {
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| std::env::var("AW2_EMU_VERIFY").is_ok_and(|s| s.trim() == "1"))
}

/// `lock cmpxchg16b` on a 16-byte-aligned guest address: a real 128-bit CAS (LSE `caspal`)
/// so it is atomic against concurrent 64-bit locked ops / stores to either half, followed by
/// x86's full barrier. Returns the previous value. `None` on hosts without it (callers fall
/// back to a mutex).
#[cfg(target_arch = "aarch64")]
#[inline]
unsafe fn cas128_full(addr: u64, expected: u128, desired: u128) -> Option<u128> {
    let (mut lo, mut hi) = (expected as u64, (expected >> 64) as u64);
    core::arch::asm!(
        "caspal x0, x1, x2, x3, [{p}]",
        p = in(reg) addr,
        inout("x0") lo,
        inout("x1") hi,
        in("x2") desired as u64,
        in("x3") (desired >> 64) as u64,
        options(nostack),
    );
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    Some(((hi as u128) << 64) | lo as u128)
}

#[cfg(not(target_arch = "aarch64"))]
#[inline]
unsafe fn cas128_full(_addr: u64, _expected: u128, _desired: u128) -> Option<u128> {
    None
}

#[cold]
#[inline(never)]
fn report_fast_mismatch(ip: u64, ins: &Instruction, what: &str, fast_v: u64, gen_v: u64) {
    static SEEN: std::sync::Mutex<Option<std::collections::HashSet<u64>>> = std::sync::Mutex::new(None);
    let mut g = SEEN.lock().unwrap_or_else(|e| e.into_inner());
    if g.get_or_insert_with(Default::default).insert(ip) {
        log::error!("emu fast-path MISMATCH at {ip:#x} `{ins}`: {what} fast={fast_v:#x} generic={gen_v:#x}");
    }
}

/// Log an instruction the generic interpreter has no handler for (once per guest IP). Execution
/// continues past it as a no-op, so this is usually the first clue for a later divergence.
#[cold]
#[inline(never)]
fn report_unimplemented(ip: u64, mn: Mnemonic, ins: &Instruction) {
    static SEEN: std::sync::Mutex<Option<std::collections::HashSet<u64>>> = std::sync::Mutex::new(None);
    let mut g = SEEN.lock().unwrap_or_else(|e| e.into_inner());
    if g.get_or_insert_with(Default::default).insert(ip) {
        log::error!("emu: UNIMPLEMENTED mnemonic {mn:?} at {ip:#x} `{ins}`");
    }
}

/// Compare a shadow CPU (fast path) against the real CPU (generic path).
#[cold]
#[inline(never)]
fn verify_fast(s: &Cpu, cpu: &Cpu, ip: u64, ins: &Instruction) {
    for i in 0..16 {
        if s.gpr[i] != cpu.gpr[i] {
            report_fast_mismatch(ip, ins, &format!("gpr[{i}]"), s.gpr[i], cpu.gpr[i]);
            return;
        }
        if s.last_bits[i] != cpu.last_bits[i] {
            report_fast_mismatch(ip, ins, &format!("last_bits[{i}]"), s.last_bits[i] as u64, cpu.last_bits[i] as u64);
            return;
        }
    }
    let fl = |c: &Cpu| (c.cf as u64) | (c.zf as u64) << 1 | (c.sf as u64) << 2 | (c.of as u64) << 3 | (c.pf() as u64) << 4;
    if fl(s) != fl(cpu) {
        report_fast_mismatch(ip, ins, "flags(cf,zf,sf,of,pf)", fl(s), fl(cpu));
        return;
    }
    if s.rip != cpu.rip {
        report_fast_mismatch(ip, ins, "rip", s.rip, cpu.rip);
        return;
    }
    for i in 0..16 {
        for (what, a, b) in [("xmm", s.xmm[i], cpu.xmm[i]), ("ymm_hi", s.ymm_hi[i], cpu.ymm_hi[i])] {
            if a != b {
                let half = if a as u64 != b as u64 { "lo" } else { "hi" };
                let sh = if half == "lo" { 0 } else { 64 };
                report_fast_mismatch(ip, ins, &format!("{what}[{i}].{half}"), (a >> sh) as u64, (b >> sh) as u64);
                return;
            }
        }
    }
}

struct ThreadEmuState {
    _stack: Box<[u64]>,
    stack_min: u64,
    stack_top: u64,
    sp: u64,
    blocks: BlockCache,
    jit: jit::JitCache,
}

/// Bytes kept free above `stack_min` (the guest stack's floor for the lifted stack account):
/// a lifted budget never reaches below `stack_min + GUEST_FLOOR_MARGIN`, and a nested region
/// whose start (charged with the live lifted depth) would lie below it raises
/// `STATUS_STACK_OVERFLOW` instead (`crate::lstack` "Stack account").
pub(crate) const GUEST_FLOOR_MARGIN: u64 = 64 << 10;
/// `stack_min` offset from the guest stack base.
const GUEST_STACK_MIN_OFF: u64 = 256 * 1024 * 8;
/// Lower bound of `stack_top - floor` on any thread (the lifted budget of a thread with no
/// emulator state yet).
pub(crate) const GUEST_ROOM_MIN: u64 = (GUEST_STACK_QWORDS as u64) * 8 - 256 - 16 - GUEST_STACK_MIN_OFF - GUEST_FLOOR_MARGIN;

impl ThreadEmuState {
    fn new() -> Self {
        let stack = vec![0u64; GUEST_STACK_QWORDS].into_boxed_slice();
        let base_addr = stack.as_ptr() as u64;
        let stack_top = (base_addr + (GUEST_STACK_QWORDS as u64) * 8 - 256) & !0xFu64;
        let stack_min = base_addr + GUEST_STACK_MIN_OFF;
        crate::lstack::note_guest_stack(crate::callctx::ctx(), stack_min + GUEST_FLOOR_MARGIN, stack_top);
        ThreadEmuState { _stack: stack, stack_min, stack_top, sp: stack_top, blocks: BlockCache::new(), jit: jit::JitCache::new() }
    }
}

std::thread_local! {
    static TLS_EMU: std::cell::UnsafeCell<Option<ThreadEmuState>> = const { std::cell::UnsafeCell::new(None) };
}

struct SpGuard {
    sp_ptr: *mut u64,
    saved_sp: u64,
    saved_teb_stack: Option<(usize, u64, u64, u64)>,
}

impl Drop for SpGuard {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            *self.sp_ptr = self.saved_sp;
            if let Some((teb, base, limit, dealloc)) = self.saved_teb_stack {
                wr::<u64>(teb + 0x08, base);
                wr::<u64>(teb + 0x10, limit);
                wr::<u64>(teb + 0x1478, dealloc);
            }
        }
    }
}

std::thread_local! {
    /// Innermost running interpreter frame's CPU on this thread (null when none).
    static CUR_CPU: core::cell::Cell<*const Cpu> = const { core::cell::Cell::new(core::ptr::null()) };
}

/// Restores the outer frame's [`CUR_CPU`] on every exit path.
struct CurCpuGuard(*const Cpu);
impl Drop for CurCpuGuard {
    #[inline(always)]
    fn drop(&mut self) {
        CUR_CPU.with(|c| c.set(self.0));
    }
}

/// `(gpr[16], rip)` of this thread's innermost running interpreter frame, for crash
/// reports. `rip` is the address after the instruction being executed. Register values
/// may be slightly stale if the compiler kept them in host registers.
pub fn current_guest_state() -> Option<([u64; 16], u64)> {
    let p = CUR_CPU.try_with(|c| c.get()).ok()?;
    if p.is_null() {
        return None;
    }
    unsafe { Some((core::ptr::read_volatile(&(*p).gpr), core::ptr::read_volatile(&(*p).rip))) }
}

/// `(CUR_CPU, guest stack allocation pointer)` of this thread, for `eh_rt::anchor::Snapshot`
/// (sp 0 = no emulator state yet).
pub(crate) fn eh_emu_state() -> (usize, u64) {
    let cpu = CUR_CPU.with(|c| c.get()) as usize;
    let sp = TLS_EMU.with(|cell| unsafe { (*cell.get()).as_ref().map_or(0, |s| s.sp) });
    (cpu, sp)
}

/// Restore what [`eh_emu_state`] returned after a landing skipped the frames whose
/// `CurCpuGuard` / `SpGuard` would have. The stack pointer is only set if the state exists.
pub(crate) unsafe fn eh_set_emu_state(cpu: usize, sp: u64) {
    CUR_CPU.with(|c| c.set(cpu as *const Cpu));
    TLS_EMU.with(|cell| {
        if let Some(s) = unsafe { (*cell.get()).as_mut() } {
            if sp != 0 {
                s.sp = sp;
            }
        }
    });
}

/// Owns one interpreter invocation's counters and records them in [`crate::emu_prof`] on every exit path.
struct ProfGuard {
    prof: Option<&'static crate::emu_prof::ThreadProf>,
    entry_ip: u64,
    steps: usize,
    misses: u64,
    flushed_steps: usize,
    flushed_misses: u64,
    sample_left: u32,
}

#[cfg(test)]
std::thread_local! {
    /// Guest instructions executed by the most recently finished `emu_run_region` on this thread.
    static TEST_LAST_STEPS: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
    /// Cumulative guest instructions executed across `emu_run_region` runs on this thread.
    static TEST_TOTAL_STEPS: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
}

#[cfg(test)]
pub(crate) fn test_last_steps() -> u64 {
    TEST_LAST_STEPS.with(|c| c.get())
}

#[cfg(test)]
pub(crate) fn test_total_steps() -> u64 {
    TEST_TOTAL_STEPS.with(|c| c.get())
}

impl Drop for ProfGuard {
    #[inline(always)]
    fn drop(&mut self) {
        #[cfg(test)]
        {
            let s = self.steps as u64;
            TEST_LAST_STEPS.with(|c| c.set(s));
            TEST_TOTAL_STEPS.with(|c| c.set(c.get().wrapping_add(s)));
        }
        if let Some(p) = self.prof {
            if self.steps > 0 {
                p.func_exit(self.entry_ip, (self.steps - self.flushed_steps) as u64, self.misses - self.flushed_misses);
            }
            p.sample_ctr.set(self.sample_left);
        }
    }
}

/// Call a native host function from guest code, timing it for the profiler when enabled.
#[inline(always)]
unsafe fn call_host_profiled(
    prof: Option<&'static crate::emu_prof::ThreadProf>,
    ret_ip: u64,
    target_va: u64,
    host_fn: usize,
    gpr: [u64; 8],
    xmm: [u128; 4],
    stack_args: [u64; 8],
) -> (u64, u128) {
    let r = match prof {
        None => call_host_for_va(target_va, host_fn, gpr, xmm, stack_args),
        Some(p) => {
            let t0 = crate::emu_prof::now_ns();
            let r = call_host_for_va(target_va, host_fn, gpr, xmm, stack_args);
            p.host_call(host_fn, ret_ip, crate::emu_prof::now_ns().saturating_sub(t0));
            r
        }
    };
    if crate::diag::nan_trace_installed() && crate::diag::nan_trace_active() {
        let (_, flags) = resolve_dispatch(target_va);
        crate::diag::check_host_call(target_va, ret_ip, flags, &gpr, &xmm, r.0, r.1);
    }
    r
}

/// Tail-call native `host_fn` from guest code whose stack is at function-entry state
/// (return address on top): pops the return address, calls the host function with the
/// Win64 arguments, stores the result in `rax`/`xmm0`, and returns the popped return address.
#[inline(never)]
unsafe fn host_tail_call(
    cpu: &mut Cpu,
    sp_ptr: *mut u64,
    prof: Option<&'static crate::emu_prof::ThreadProf>,
    target_va: u64,
    host_fn: usize,
) -> u64 {
    let ret_addr = cpu.read_mem(cpu.gpr[4], 64);
    cpu.gpr[4] = cpu.gpr[4].wrapping_add(8);
    let rsp = cpu.gpr[4];
    let s5 = rd::<u64>((rsp + 0x20) as usize);
    let s6 = rd::<u64>((rsp + 0x28) as usize);
    let s7 = rd::<u64>((rsp + 0x30) as usize);
    let s8 = rd::<u64>((rsp + 0x38) as usize);
    let s_extra = [
        rd::<u64>((rsp + 0x40) as usize),
        rd::<u64>((rsp + 0x48) as usize),
        rd::<u64>((rsp + 0x50) as usize),
        rd::<u64>((rsp + 0x58) as usize),
        rd::<u64>((rsp + 0x60) as usize),
        rd::<u64>((rsp + 0x68) as usize),
        rd::<u64>((rsp + 0x70) as usize),
        rd::<u64>((rsp + 0x78) as usize),
    ];
    *sp_ptr = (rsp.wrapping_sub(256)) & !0xFu64;
    set_incoming_rsp(rsp, rsp.wrapping_add(0x100));
    let prev_rax = crate::simd_ops::replace_rax_in(cpu.gpr[0]);
    // The calling guest frame's state for an EH walk during the call (`eh::callout`).
    cpu.rip = ret_addr;
    // Only lifted `DF_GAME_REPL` functions (which take raw `u64` params without `win64-abi`
    // narrow-int guards) rely on `last_bits` argument masking; real Win32/CRT `extern "win64"`
    // functions already mask `u8`/`u16`/`bool` params via `win64-abi` and must see full 64-bit
    // guest registers even after partial-register ops (`mov cl, cl`, `add cl, imm8`).
    let mask_args = resolve_dispatch(target_va).1 & DF_GAME_REPL != 0;
    let gpr = if mask_args {
        [cpu.abi_gpr(1), cpu.abi_gpr(2), cpu.abi_gpr(8), cpu.abi_gpr(9), s5, s6, s7, s8]
    } else {
        [cpu.gpr[1], cpu.gpr[2], cpu.gpr[8], cpu.gpr[9], s5, s6, s7, s8]
    };
    let xmm = [cpu.xmm[0], cpu.xmm[1], cpu.xmm[2], cpu.xmm[3]];
    let lsa = crate::lstack::anchor_enter(crate::callctx::ctx(), *sp_ptr);
    let mxcsr_ptr = &mut cpu.jx.mxcsr as *mut u32;
    let r = {
        let _fpcr = HostFpcrGuard::enter(mxcsr_ptr);
        eh::callout(cpu, host_fn, *sp_ptr, move || {
            call_host_profiled(prof, ret_addr, target_va, host_fn, gpr, xmm, s_extra)
        })
    };
    crate::lstack::anchor_leave(crate::callctx::ctx(), lsa);
    crate::simd_ops::replace_rax_in(prev_rax);
    set_incoming_rsp(0, 0);
    match r {
        Some((r_gpr, r_xmm)) => {
            cpu.gpr[0] = r_gpr;
            cpu.xmm[0] = r_xmm;
            cpu.last_bits[0] = 64;
            ret_addr
        }
        // An EH landing in this region: continue at the landing's guest state.
        None => cpu.rip,
    }
}

/// Guest `call` into a native host function: passes the Win64 arguments (4 registers +
/// stack slots at `[rsp+0x20..]`), stores the result in `rax`/`xmm0`. `cpu.rip` must be
/// the return address. Nothing is pushed (`rsp` is unchanged), but the return address is
/// stored at `[rsp-8]`, where a real `call` would have put it: guest code that walks its own
/// frames from inside the callee reads it there (V8's `ExitFrame` takes its pc from
/// `[saved rsp - 8]`; a stale slot made `GcSafeFindCodeForInnerPointer` abort). The slot is
/// below the guest `rsp` (free under Win64, which has no red zone) and nested runs start at
/// `(rsp - 256) & !0xF`, so nothing live is overwritten. The `xfer_plain` fast path and the
/// translator's inline `plain_call`s do not store it: `DF_PLAIN` builtins never run guest code.
#[inline(never)]
unsafe fn host_call(
    cpu: &mut Cpu,
    sp_ptr: *mut u64,
    prof: Option<&'static crate::emu_prof::ThreadProf>,
    target_va: u64,
    host_fn: usize,
) {
    let rsp = cpu.gpr[4];
    if (0x1_0008..0x8000_0000_0000).contains(&rsp) {
        wr::<u64>((rsp - 8) as usize, cpu.rip);
    }
    let s5 = rd::<u64>((rsp + 0x20) as usize);
    let s6 = rd::<u64>((rsp + 0x28) as usize);
    let s7 = rd::<u64>((rsp + 0x30) as usize);
    let s8 = rd::<u64>((rsp + 0x38) as usize);
    let s_extra = [
        rd::<u64>((rsp + 0x40) as usize),
        rd::<u64>((rsp + 0x48) as usize),
        rd::<u64>((rsp + 0x50) as usize),
        rd::<u64>((rsp + 0x58) as usize),
        rd::<u64>((rsp + 0x60) as usize),
        rd::<u64>((rsp + 0x68) as usize),
        rd::<u64>((rsp + 0x70) as usize),
        rd::<u64>((rsp + 0x78) as usize),
    ];
    *sp_ptr = (rsp.wrapping_sub(256)) & !0xFu64;
    set_incoming_rsp(rsp, rsp.wrapping_add(0x100));
    let prev_rax = crate::simd_ops::replace_rax_in(cpu.gpr[0]);
    let ret_ip = cpu.rip;
    let mask_args = resolve_dispatch(target_va).1 & DF_GAME_REPL != 0;
    let gpr = if mask_args {
        [cpu.abi_gpr(1), cpu.abi_gpr(2), cpu.abi_gpr(8), cpu.abi_gpr(9), s5, s6, s7, s8]
    } else {
        [cpu.gpr[1], cpu.gpr[2], cpu.gpr[8], cpu.gpr[9], s5, s6, s7, s8]
    };
    let xmm = [cpu.xmm[0], cpu.xmm[1], cpu.xmm[2], cpu.xmm[3]];
    let lsa = crate::lstack::anchor_enter(crate::callctx::ctx(), *sp_ptr);
    let mxcsr_ptr = &mut cpu.jx.mxcsr as *mut u32;
    let r = {
        let _fpcr = HostFpcrGuard::enter(mxcsr_ptr);
        eh::callout(cpu, host_fn, *sp_ptr, move || {
            call_host_profiled(prof, ret_ip, target_va, host_fn, gpr, xmm, s_extra)
        })
    };
    crate::lstack::anchor_leave(crate::callctx::ctx(), lsa);
    crate::simd_ops::replace_rax_in(prev_rax);
    set_incoming_rsp(0, 0);
    // `None`: an EH landing in this region; `cpu` holds the landing's guest state.
    if let Some((r_gpr, r_xmm)) = r {
        cpu.gpr[0] = r_gpr;
        cpu.xmm[0] = r_xmm;
        cpu.last_bits[0] = 64;
    }
}

/// Like [`call_host_with_xmm`], but preserves `xmm[0]` across integer `aw2-game` replacements
/// (`0x1_4000_0000..0x1_4600_0000`, excluding 6-byte `FF 25` IAT jump thunks to builtin DLLs)
/// unless the replacement explicitly wrote an XMM result via `simd_ops::set_xmm_ret`, and
/// preserves `rax` (`cpu.gpr[0]`) across float-returning replacements that do not write `rax`.
#[inline]
pub unsafe fn call_host_for_va(
    target_va: u64,
    host_fn: usize,
    gpr: [u64; 8],
    xmm: [u128; 4],
    stack_args: [u64; 8],
) -> (u64, u128) {
    let (_, flags) = resolve_dispatch(target_va);
    call_host_dispatched(target_va, host_fn, flags, gpr, xmm, stack_args)
}

/// [`call_host_for_va`] with the target's `DF_*` flags already resolved
/// (see [`resolve_dispatch`]).
#[inline]
pub unsafe fn call_host_dispatched(
    target_va: u64,
    host_fn: usize,
    flags: u32,
    gpr: [u64; 8],
    xmm: [u128; 4],
    stack_args: [u64; 8],
) -> (u64, u128) {
    let _fpcr = ClearFpcrGuard::enter();
    call_host_dispatched_in(crate::callctx::ctx(), target_va, host_fn, flags, &gpr, &xmm, &stack_args)
}

/// [`call_host_dispatched`] with the per-thread context already looked up.
#[inline(always)]
pub(crate) unsafe fn call_host_dispatched_in(
    c: &crate::callctx::CallCtx,
    target_va: u64,
    host_fn: usize,
    flags: u32,
    gpr: &[u64; 8],
    xmm: &[u128; 4],
    stack_args: &[u64; 8],
) -> (u64, u128) {
    let is_game_repl = flags & DF_GAME_REPL != 0;
    let is_float_ret = flags & DF_FLOAT_RET != 0;
    // == simd_ops::set_tls_xmm_in(xmm)
    c.xmm_in.set(*xmm);
    c.xmm_in_set.set(true);
    c.xmm_out.set(None);
    let jp = if flags & DF_MARK != 0 { crate::jobprof::begin() } else { None };
    let (mut r_gpr, raw_q0) = host_call_asm(host_fn, gpr, xmm, stack_args);
    if flags & DF_MARK != 0 {
        crate::jobprof::end(jp, crate::jobprof::K_MARK, target_va, 0xffff, 0);
    }
    if flags & DF_TRACE != 0 && !is_game_repl {
        cold_trace_eat(target_va, gpr, r_gpr);
    }
    // == simd_ops::take_tls_xmm_out_opt()
    c.xmm_in_set.set(false);
    let tls_xmm = c.xmm_out.take();
    if is_game_repl && is_float_ret {
        r_gpr = c.rax_in.get();
    }
    let r_xmm = if is_float_ret {
        raw_q0
    } else {
        match tls_xmm {
            Some(v) => v,
            None if is_game_repl || (flags & DF_PLAIN != 0) => xmm[0],
            None => raw_q0,
        }
    };
    (r_gpr, r_xmm)
}

/// Invoke a native host function pointer `host_fn` passing GPR (`x0..x7` / Win64 args 1..8),
/// XMM/NEON (`q0..q3`), and stack arguments 9..16 (`stack_args[0..8]` at `[sp+0x00..sp+0x40]`),
/// returning `(rax/x0, xmm0/q0)`.
#[inline]
pub unsafe fn call_host_with_xmm(
    host_fn: usize,
    gpr: [u64; 8],
    xmm: [u128; 4],
    stack_args: [u64; 8],
) -> (u64, u128) {
    let (r_gpr, raw_q0) = call_host_with_xmm_raw(host_fn, gpr, xmm, stack_args);
    (r_gpr, crate::simd_ops::take_tls_xmm_out(raw_q0))
}

#[cold]
#[inline(never)]
fn cold_trace_eat(target_va: u64, gpr: &[u64; 8], r_gpr: u64) {
    let sym = win32_rt::symbolize(target_va as usize);
    log::info!(
        "[eat] {}({:#x}, {:#x}, {:#x}, {:#x}) -> {:#x}",
        sym, gpr[0], gpr[1], gpr[2], gpr[3], r_gpr
    );
}

#[inline]
unsafe fn call_host_with_xmm_raw(
    host_fn: usize,
    gpr: [u64; 8],
    xmm: [u128; 4],
    stack_args: [u64; 8],
) -> (u64, u128) {
    let _fpcr = ClearFpcrGuard::enter();
    crate::simd_ops::set_tls_xmm_in(xmm);
    host_call_asm(host_fn, &gpr, &xmm, &stack_args)
}

/// The register-level host call: `x0..x7` = `gpr`, `q0..q3` = `xmm`, stack args 9..16 at
/// `[sp]`, returning `(x0, q0)`. Does not touch the per-thread XMM forwarding state or
/// hardware `FPCR` (callers must hold [`HostFpcrGuard`] or [`ClearFpcrGuard`]).
#[inline(always)]
unsafe fn host_call_asm(
    host_fn: usize,
    gpr: &[u64; 8],
    xmm: &[u128; 4],
    stack_args: &[u64; 8],
) -> (u64, u128) {
    #[cfg(target_arch = "aarch64")]
    {
        use core::arch::aarch64::uint64x2_t;
        #[inline(always)]
        fn q(v: u128) -> uint64x2_t {
            unsafe { core::mem::transmute(v) }
        }
        // x19..x28 cannot be named as inline-asm operands (LLVM reserves
        // x19), so the fence for the arm64 callee-saved registers lives in
        // the asm string itself: everything below `blr x16` is guest-emulation
        // machinery (JIT-generated code, hand-written stubs, the interpreter)
        // that must not be assumed AAPCS64-clean. Without this fence a cached
        // spill pointer or TLS cell address in the calling Rust frame can come
        // back as a guest register value (observed: x24 = 0xffff..ffff).
        //
        // Arguments 1..8 and the four vector arguments are passed straight in
        // x0..x7 / q0..q3; only stack arguments 9..16 are copied from `x17` to
        // `[sp, #0x00..#0x40]` of the call frame.
        let r_x0: u64;
        let r_q0: uint64x2_t;
        core::arch::asm!(
            "sub sp, sp, #0xA0",
            "stp x29, x30, [sp, #0x90]",
            "add x29, sp, #0x90",
            "stp x19, x20, [sp, #0x40]",
            "stp x21, x22, [sp, #0x50]",
            "stp x23, x24, [sp, #0x60]",
            "stp x25, x26, [sp, #0x70]",
            "stp x27, x28, [sp, #0x80]",
            "ldp x9, x10, [x17, #0x00]",
            "stp x9, x10, [sp, #0x00]",
            "ldp x9, x10, [x17, #0x10]",
            "stp x9, x10, [sp, #0x10]",
            "ldp x9, x10, [x17, #0x20]",
            "stp x9, x10, [sp, #0x20]",
            "ldp x9, x10, [x17, #0x30]",
            "stp x9, x10, [sp, #0x30]",
            "blr x16",
            "ldp x19, x20, [sp, #0x40]",
            "ldp x21, x22, [sp, #0x50]",
            "ldp x23, x24, [sp, #0x60]",
            "ldp x25, x26, [sp, #0x70]",
            "ldp x27, x28, [sp, #0x80]",
            "ldp x29, x30, [sp, #0x90]",
            "add sp, sp, #0xA0",
            inout("x16") host_fn => _,
            inout("x17") stack_args.as_ptr() => _,
            inout("x0") gpr[0] => r_x0,
            inout("x1") gpr[1] => _, inout("x2") gpr[2] => _, inout("x3") gpr[3] => _,
            inout("x4") gpr[4] => _, inout("x5") gpr[5] => _, inout("x6") gpr[6] => _,
            inout("x7") gpr[7] => _,
            inout("v0") q(xmm[0]) => r_q0,
            inout("v1") q(xmm[1]) => _, inout("v2") q(xmm[2]) => _, inout("v3") q(xmm[3]) => _,
            out("x8") _, out("x9") _, out("x10") _, out("x11") _,
            out("x12") _, out("x13") _, out("x14") _, out("x15") _,
            out("v4") _, out("v5") _, out("v6") _, out("v7") _,
            out("v16") _, out("v17") _, out("v18") _, out("v19") _,
            out("v20") _, out("v21") _, out("v22") _, out("v23") _,
            out("v24") _, out("v25") _, out("v26") _, out("v27") _,
            out("v28") _, out("v29") _, out("v30") _, out("v31") _,
        );
        (r_x0, core::mem::transmute::<uint64x2_t, u128>(r_q0))
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let (gpr, stack_args) = (*gpr, *stack_args);
        let _ = xmm;
        let f: unsafe extern "win64" fn(
            u64, u64, u64, u64, u64, u64, u64, u64,
            u64, u64, u64, u64, u64, u64, u64, u64,
        ) -> u64 = core::mem::transmute(host_fn);
        let r = f(
            gpr[0], gpr[1], gpr[2], gpr[3], gpr[4], gpr[5], gpr[6], gpr[7],
            stack_args[0], stack_args[1], stack_args[2], stack_args[3],
            stack_args[4], stack_args[5], stack_args[6], stack_args[7],
        );
        (r, r as u128)
    }
}

#[cfg(test)]
std::thread_local! {
    /// Test-only counter of `DF_VALID | DF_HOST | DF_PLAIN` leaf host calls taken on the
    /// direct `host_call_asm` fast path in [`apply_fx`] / [`apply_xfer_tail`].
    static TEST_PLAIN_HOST_CALLS: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
}

/// Direct leaf call into a `DF_VALID | DF_HOST | DF_PLAIN` integer builtin via
/// [`host_call_asm`], skipping `eh::callout`, `lstack` anchoring, `CallCtx` XMM/RSP window
/// setup, and extra stack-arg loads (`s_extra`). Only used when `prof.is_none() && !precise`.
#[inline(always)]
unsafe fn plain_host_call_asm(cpu: &mut Cpu, host_fn: usize, args_rsp: u64) {
    let r = args_rsp as usize;
    let gpr = [
        cpu.gpr[1],
        cpu.gpr[2],
        cpu.gpr[8],
        cpu.gpr[9],
        rd::<u64>(r + 0x20),
        rd::<u64>(r + 0x28),
        rd::<u64>(r + 0x30),
        rd::<u64>(r + 0x38),
    ];
    let _fpcr = ClearFpcrGuard::enter_for_mxcsr(cpu.jx.mxcsr);
    let (rax, _q0) = host_call_asm(host_fn, &gpr, &[0u128; 4], &[0u64; 8]);
    cpu.gpr[0] = rax;
    cpu.last_bits[0] = 64;
    #[cfg(test)]
    TEST_PLAIN_HOST_CALLS.with(|c| c.set(c.get().wrapping_add(1)));
}

/// Check if `target` resolves to a pure-integer `DF_VALID | DF_HOST | DF_PLAIN` leaf builtin
/// eligible for [`plain_host_call_asm`] (when profiling and precise-fault mode are both off).
#[inline(always)]
fn plain_leaf_fn(
    target: u64,
    host_fn: usize,
    prof: Option<&'static crate::emu_prof::ThreadProf>,
) -> Option<usize> {
    if prof.is_some() || precise_faults() || crate::diag::nan_trace_installed() {
        return None;
    }
    let (h, flags) = resolve_dispatch(target);
    if flags == DF_VALID | DF_HOST | DF_PLAIN && h == host_fn && h != 0 {
        Some(h)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn apply_tail_fx(
    cpu: &mut Cpu,
    target: u64,
    entry_ip: u64,
    blocks_ptr: *mut BlockCache,
    sp_ptr: *mut u64,
    prof: Option<&'static crate::emu_prof::ThreadProf>,
) -> Option<(u64, u128)> {
    apply_xfer_tail(cpu, target, false, entry_ip, blocks_ptr, sp_ptr, prof)
}

#[inline]
unsafe fn apply_xfer_tail(
    cpu: &mut Cpu,
    mut target: u64,
    mut from_call: bool,
    entry_ip: u64,
    blocks_ptr: *mut BlockCache,
    sp_ptr: *mut u64,
    prof: Option<&'static crate::emu_prof::ThreadProf>,
) -> Option<(u64, u128)> {
    let mut hops = 0u32;
    loop {
        if from_call && target == 0 {
            let ret_addr = cpu.read_mem(cpu.gpr[4], 64);
            cpu.gpr[4] = cpu.gpr[4].wrapping_add(8);
            note_null_call();
            cpu.gpr[0] = 0;
            cpu.last_bits[0] = 64;
            if ret_addr == RET_SENTINEL || ret_addr == 0 {
                return Some((cpu.abi_gpr(0), cpu.xmm[0]));
            }
            if hops < 4096 && lookup_routed_regfile(ret_addr).is_some() {
                hops += 1;
                target = ret_addr;
                from_call = false;
                continue;
            }
            cpu.rip = ret_addr;
            return None;
        }
        let routed_stub = if let Some(t) = resolve_dll_stub_target(target) {
            target = t;
            true
        } else {
            false
        };
        if target == 0x141835480 && crate::exe::aw2_registry_active() {
            // Windows `__chkstk` tail call: return to the caller.
            let ret_addr = cpu.read_mem(cpu.gpr[4], 64);
            cpu.gpr[4] = cpu.gpr[4].wrapping_add(8);
            if ret_addr == RET_SENTINEL || ret_addr == 0 {
                return Some((cpu.abi_gpr(0), cpu.xmm[0]));
            }
            if hops < 4096 && lookup_routed_regfile(ret_addr).is_some() {
                hops += 1;
                target = ret_addr;
                from_call = false;
                continue;
            }
            cpu.rip = ret_addr;
            return None;
        }
        if is_routed_dll_chkstk(target) {
            if let Some(r) = dll_chkstk_tail(cpu, sp_ptr) {
                return Some(r);
            }
            if hops < 4096 && lookup_routed_regfile(cpu.rip).is_some() {
                hops += 1;
                target = cpu.rip;
                from_call = false;
                continue;
            }
            return None;
        }
        if is_routed_dll_stack_pos(target) {
            if let Some(r) = dll_stack_pos_tail(cpu) {
                return Some(r);
            }
            if hops < 4096 && lookup_routed_regfile(cpu.rip).is_some() {
                hops += 1;
                target = cpu.rip;
                from_call = false;
                continue;
            }
            return None;
        }
        if let Some((rf, blk)) = lookup_routed_regfile(target) {
            if hops >= 4096 {
                cpu.rip = target;
                return None;
            }
            let ex = regfile::run_chain(cpu, rf, blk);
            match ex.kind {
                regfile::RF_RET => {
                    if ex.va == RET_SENTINEL || ex.va == 0 {
                        if ex.va == 0 && entry_ip != 0 {
                            note_null_call();
                        }
                        return Some((cpu.abi_gpr(0), cpu.xmm[0]));
                    }
                    cpu.rip = ex.va;
                    return None;
                }
                regfile::RF_JUMP => {
                    if ex.va == RET_SENTINEL || ex.va == 0 {
                        if ex.va == 0 && entry_ip != 0 {
                            note_null_call();
                        }
                        return Some((cpu.abi_gpr(0), cpu.xmm[0]));
                    }
                    hops += 1;
                    target = ex.va;
                    from_call = false;
                    continue;
                }
                regfile::RF_CALL => {
                    hops += 1;
                    target = ex.va;
                    from_call = true;
                    continue;
                }
                _ => {
                    cpu.rip = ex.va;
                    regfile::arm_skip(ex.va);
                    return None;
                }
            }
        }
        if routed_stub || from_call || target != entry_ip {
            if let Some(host_fn) = BlockCache::resolve(blocks_ptr, target) {
                let ret_addr = if let Some(h) = plain_leaf_fn(target, host_fn, prof) {
                    let ret_addr = cpu.read_mem(cpu.gpr[4], 64);
                    cpu.gpr[4] = cpu.gpr[4].wrapping_add(8);
                    plain_host_call_asm(cpu, h, cpu.gpr[4]);
                    ret_addr
                } else {
                    host_tail_call(cpu, sp_ptr, prof, target, host_fn)
                };
                if ret_addr == RET_SENTINEL || ret_addr == 0 {
                    return Some((cpu.gpr[0], cpu.xmm[0]));
                }
                if hops < 4096 && lookup_routed_regfile(ret_addr).is_some() {
                    hops += 1;
                    target = ret_addr;
                    from_call = false;
                    continue;
                }
                cpu.rip = ret_addr;
                return None;
            }
        }
        break;
    }
    cpu.rip = target;
    None
}

/// Apply a fast op's / translated block's control transfer (`cpu.rip` = next instruction):
/// host-function dispatch for `call`/`jmp`/near `jcc` targets, Windows `__chkstk`, and the
/// guest `call` push. Returns `Some((rax, xmm0))` when the guest function returned to the
/// interpreter's sentinel through a host tail call.
#[inline(always)]
unsafe fn apply_fx(
    cpu: &mut Cpu,
    fx: fast::Fx,
    entry_ip: u64,
    blocks_ptr: *mut BlockCache,
    sp_ptr: *mut u64,
    prof: Option<&'static crate::emu_prof::ThreadProf>,
) -> Option<(u64, u128)> {
    match fx {
        fast::Fx::Done | fast::Fx::FuseJcc => {}
        fast::Fx::JccNear(target) | fast::Fx::Jmp(target) => {
            return apply_tail_fx(cpu, target, entry_ip, blocks_ptr, sp_ptr, prof);
        }
        fast::Fx::Call(mut target) => {
            if let Some(t) = resolve_dll_stub_target(target) {
                target = t;
            }
            if target == 0 {
                note_null_call();
                cpu.gpr[0] = 0;
                cpu.last_bits[0] = 64;
            } else if target == 0x141835480 && crate::exe::aw2_registry_active() {
                // Windows `__chkstk`: no-op.
            } else if is_routed_dll_chkstk(target) {
                return dll_chkstk_call(cpu, sp_ptr);
            } else if is_routed_dll_stack_pos(target) {
                return dll_stack_pos_call(cpu);
            } else if let Some((rf, blk)) = lookup_routed_regfile(target) {
                let entry_rsp = cpu.gpr[4].wrapping_sub(8);
                cpu.write_mem(entry_rsp, 64, cpu.rip);
                if precise_faults() && FAULT_PENDING.with(|p| p.get().is_some()) {
                    return None;
                }
                cpu.gpr[4] = entry_rsp;
                let ex = regfile::run_chain(cpu, rf, blk);
                match ex.kind {
                    regfile::RF_RET => {
                        if ex.va == RET_SENTINEL || ex.va == 0 {
                            if ex.va == 0 && entry_ip != 0 {
                                note_null_call();
                            }
                            return Some((cpu.abi_gpr(0), cpu.xmm[0]));
                        }
                        cpu.rip = ex.va;
                    }
                    regfile::RF_JUMP => {
                        if ex.va == RET_SENTINEL || ex.va == 0 {
                            if ex.va == 0 && entry_ip != 0 {
                                note_null_call();
                            }
                            return Some((cpu.abi_gpr(0), cpu.xmm[0]));
                        }
                        return apply_xfer_tail(cpu, ex.va, false, entry_ip, blocks_ptr, sp_ptr, prof);
                    }
                    regfile::RF_CALL => {
                        return apply_xfer_tail(cpu, ex.va, true, entry_ip, blocks_ptr, sp_ptr, prof);
                    }
                    _ => {
                        cpu.rip = ex.va;
                        regfile::arm_skip(ex.va);
                    }
                }
            } else if let Some(host_fn) = BlockCache::resolve(blocks_ptr, target) {
                if let Some(h) = plain_leaf_fn(target, host_fn, prof) {
                    plain_host_call_asm(cpu, h, cpu.gpr[4]);
                } else {
                    host_call(cpu, sp_ptr, prof, target, host_fn);
                }
            } else {
                cpu.gpr[4] = cpu.gpr[4].wrapping_sub(8);
                cpu.write_mem(cpu.gpr[4], 64, cpu.rip);
                cpu.rip = target;
            }
        }
    }
    None
}

/// Before a host call from translated code that may run guest code again on this thread
/// (nested run of the same translation cache): store the live shadow-RAS top into the
/// cache (`jx.ctx_ras_cell`) so the nested run pushes above the caller's live entries. The
/// caller keeps its own top (`jx.ras`) afterwards; entries the nested run left are only
/// ever used after their guest address matched.
#[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]
#[inline(always)]
unsafe fn jit_ras_publish(cpu: &Cpu) {
    let cell = cpu.jx.ctx_ras_cell as *mut u64;
    if !cell.is_null() {
        *cell = cpu.jx.ras;
    }
}

/// Host call (`kind & 1 == 0`, `cpu.rip` = return address) or tail jump (`kind & 1 == 1`)
/// to `target`, made from translated code through the translator's `host_xfer` stub
/// (`kind & 2`: dynamic target from `lookup_call`/`lookup_jmp`, used for `AW2_JIT_STATS`
/// only): exactly [`apply_fx`]'s `Fx::Call` / `Fx::Jmp` handling, with the run context from
/// `cpu.jx`. Returns 0 to continue in translated code at `cpu.rip` (after the host call, or
/// at the pushed/jumped-to guest target for targets that are not host functions), or
/// `jit::EXIT_RIP` to leave translated code (the guest function returned to the sentinel, or
/// the host dispatch table changed during the call).
/// C-ABI trampoline exported for statically linked, Apple-signed ARM64 Mach-O `.o`
/// AOT modules (`bl _port_static_aot_host_xfer` in `static_macho.rs`).
#[cfg(target_arch = "aarch64")]
#[no_mangle]
pub unsafe extern "C" fn port_static_aot_host_xfer(cpu: *mut Cpu, target: u64, kind: u64) -> u64 {
    jit_host_xfer(cpu, target, kind)
}

#[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]
unsafe extern "C" fn jit_host_xfer(cpu: *mut Cpu, mut target: u64, kind: u64) -> u64 {
    let cpu = &mut *cpu;
    let dynamic = kind & 2 != 0;
    let kind = kind & 1;
    let blocks = cpu.jx.ctx_blocks as *mut BlockCache;
    let sp_ptr = cpu.jx.ctx_sp as *mut u64;
    let prof = (cpu.jx.ctx_prof as *const crate::emu_prof::ThreadProf).as_ref();
    let mut from_call = false;
    if kind == 0 {
        if let Some(t) = resolve_dll_stub_target(target) {
            target = t;
        }
        let chkstk = target == 0x141835480 && crate::exe::aw2_registry_active();
        let dll_chkstk = is_routed_dll_chkstk(target);
        if target == 0 {
            note_null_call();
            cpu.gpr[0] = 0;
            cpu.last_bits[0] = 64;
            return 0;
        }
        if chkstk {
            return 0;
        }
        if dll_chkstk {
            jit_ras_publish(cpu);
            if let Some((r_gpr, r_xmm)) = dll_chkstk_call(cpu, sp_ptr) {
                cpu.gpr[0] = r_gpr;
                cpu.xmm[0] = r_xmm;
                cpu.last_bits[0] = 64;
                cpu.rip = RET_SENTINEL;
                return jit::EXIT_RIP;
            }
            return jit_xfer_done(cpu);
        }
        if is_routed_dll_stack_pos(target) {
            let entry_rsp = cpu.gpr[4].wrapping_sub(8);
            if !(0x1_0000..0x8000_0000_0000).contains(&(entry_rsp as usize)) {
                cpu.jx.target = target;
                return jit::EXIT_CALL;
            }
            if let Some((r_gpr, r_xmm)) = dll_stack_pos_call(cpu) {
                cpu.gpr[0] = r_gpr;
                cpu.xmm[0] = r_xmm;
                cpu.last_bits[0] = 64;
                cpu.rip = RET_SENTINEL;
                return jit::EXIT_RIP;
            }
            return jit_xfer_done(cpu);
        }
        if let Some((rf, blk)) = lookup_routed_regfile(target) {
            let entry_rsp = cpu.gpr[4].wrapping_sub(8);
            if !(0x1_0000..0x8000_0000_0000).contains(&(entry_rsp as usize)) {
                cpu.jx.target = target;
                return jit::EXIT_CALL;
            }
            cpu.write_mem(entry_rsp, 64, cpu.rip);
            cpu.gpr[4] = entry_rsp;
            let ex = regfile::run_chain(cpu, rf, blk);
            match ex.kind {
                regfile::RF_RET => {
                    cpu.rip = ex.va;
                    if ex.va == RET_SENTINEL || ex.va == 0 {
                        if ex.va == 0 && cpu.jx.ctx_entry != 0 {
                            note_null_call();
                        }
                        return jit::EXIT_RIP;
                    }
                    if lookup_routed_regfile(ex.va).is_some() {
                        return jit::EXIT_RIP;
                    }
                    return jit_xfer_done(cpu);
                }
                regfile::RF_JUMP => {
                    if ex.va == RET_SENTINEL || ex.va == 0 {
                        cpu.rip = ex.va;
                        if ex.va == 0 && cpu.jx.ctx_entry != 0 {
                            note_null_call();
                        }
                        return jit::EXIT_RIP;
                    }
                    target = ex.va;
                    // Fall through into tail-jump dispatch below (`rsp` already holds the
                    // pushed return address).
                }
                regfile::RF_CALL => {
                    target = ex.va;
                    from_call = true;
                    // Fall through into tail-jump dispatch below (`rsp` already holds the
                    // pushed return address of the `DF_REGFILE` call site).
                }
                _ => {
                    cpu.rip = ex.va;
                    regfile::arm_skip(ex.va);
                    return jit::EXIT_SIDE;
                }
            }
        } else if let Some(host_fn) = BlockCache::resolve(blocks, target) {
            if jit_stats::enabled() {
                jit_stats::host_xfer(target, false, dynamic, cpu.rip);
            }
            if xfer_plain(cpu, target, cpu.gpr[4]) {
                return jit_xfer_done(cpu);
            }
            jit_ras_publish(cpu);
            host_call(cpu, sp_ptr, prof, target, host_fn);
            return jit_xfer_done(cpu);
        } else {
            // Guest target (lookup_call probe miss): push + jump as `apply_fx`, then continue
            // through `lookup_rip` (which exits with EXIT_RIP if the target is not translated).
            cpu.gpr[4] = cpu.gpr[4].wrapping_sub(8);
            cpu.write_mem(cpu.gpr[4], 64, cpu.rip);
            cpu.rip = jit_guest_target(blocks, target);
            return 0;
        }
    }
    let mut hops = 0u32;
    loop {
        if from_call && target == 0 {
            let ret_addr = cpu.read_mem(cpu.gpr[4], 64);
            cpu.gpr[4] = cpu.gpr[4].wrapping_add(8);
            note_null_call();
            cpu.gpr[0] = 0;
            cpu.last_bits[0] = 64;
            cpu.rip = ret_addr;
            if ret_addr == RET_SENTINEL || ret_addr == 0 {
                return jit::EXIT_RIP;
            }
            if hops < 4096 && lookup_routed_regfile(ret_addr).is_some() {
                hops += 1;
                target = ret_addr;
                from_call = false;
                continue;
            }
            return if lookup_routed_regfile(ret_addr).is_some() {
                jit::EXIT_RIP
            } else {
                jit_xfer_done(cpu)
            };
        }
        let routed_stub = if let Some(t) = resolve_dll_stub_target(target) {
            target = t;
            true
        } else {
            false
        };
        if target == 0x141835480 && crate::exe::aw2_registry_active() {
            let ret_addr = cpu.read_mem(cpu.gpr[4], 64);
            cpu.gpr[4] = cpu.gpr[4].wrapping_add(8);
            cpu.rip = ret_addr;
            if ret_addr == RET_SENTINEL || ret_addr == 0 {
                return jit::EXIT_RIP;
            }
            if hops < 4096 && lookup_routed_regfile(ret_addr).is_some() {
                hops += 1;
                target = ret_addr;
                from_call = false;
                continue;
            }
            if lookup_routed_regfile(ret_addr).is_some() {
                return jit::EXIT_RIP;
            }
            return 0;
        }
        if is_routed_dll_chkstk(target) {
            jit_ras_publish(cpu);
            if let Some((r_gpr, r_xmm)) = dll_chkstk_tail(cpu, sp_ptr) {
                cpu.gpr[0] = r_gpr;
                cpu.xmm[0] = r_xmm;
                cpu.last_bits[0] = 64;
                cpu.rip = RET_SENTINEL;
                return jit::EXIT_RIP;
            }
            if hops < 4096 && lookup_routed_regfile(cpu.rip).is_some() {
                hops += 1;
                target = cpu.rip;
                from_call = false;
                continue;
            }
            if lookup_routed_regfile(cpu.rip).is_some() {
                return jit::EXIT_RIP;
            }
            return jit_xfer_done(cpu);
        }
        if is_routed_dll_stack_pos(target) {
            if !(0x1_0000..0x8000_0000_0000).contains(&(cpu.gpr[4] as usize)) {
                cpu.jx.target = target;
                return jit::EXIT_JMP;
            }
            if let Some((r_gpr, r_xmm)) = dll_stack_pos_tail(cpu) {
                cpu.gpr[0] = r_gpr;
                cpu.xmm[0] = r_xmm;
                cpu.last_bits[0] = 64;
                cpu.rip = RET_SENTINEL;
                return jit::EXIT_RIP;
            }
            if hops < 4096 && lookup_routed_regfile(cpu.rip).is_some() {
                hops += 1;
                target = cpu.rip;
                from_call = false;
                continue;
            }
            if lookup_routed_regfile(cpu.rip).is_some() {
                return jit::EXIT_RIP;
            }
            return jit_xfer_done(cpu);
        }
        if let Some((rf, blk)) = lookup_routed_regfile(target) {
            if hops >= 4096 {
                cpu.rip = target;
                return jit::EXIT_RIP;
            }
            let ex = regfile::run_chain(cpu, rf, blk);
            match ex.kind {
                regfile::RF_RET => {
                    cpu.rip = ex.va;
                    if ex.va == RET_SENTINEL || ex.va == 0 {
                        if ex.va == 0 && cpu.jx.ctx_entry != 0 {
                            note_null_call();
                        }
                        return jit::EXIT_RIP;
                    }
                    if lookup_routed_regfile(ex.va).is_some() {
                        return jit::EXIT_RIP;
                    }
                    return jit_xfer_done(cpu);
                }
                regfile::RF_JUMP => {
                    if ex.va == RET_SENTINEL || ex.va == 0 {
                        cpu.rip = ex.va;
                        if ex.va == 0 && cpu.jx.ctx_entry != 0 {
                            note_null_call();
                        }
                        return jit::EXIT_RIP;
                    }
                    hops += 1;
                    target = ex.va;
                    from_call = false;
                    continue;
                }
                regfile::RF_CALL => {
                    hops += 1;
                    target = ex.va;
                    from_call = true;
                    continue;
                }
                _ => {
                    cpu.rip = ex.va;
                    regfile::arm_skip(ex.va);
                    return jit::EXIT_SIDE;
                }
            }
        }
        if routed_stub || from_call || target != cpu.jx.ctx_entry {
            if let Some(host_fn) = BlockCache::resolve(blocks, target) {
                if jit_stats::enabled() {
                    jit_stats::host_xfer(target, true, dynamic, cpu.read_mem(cpu.gpr[4], 64));
                }
                let ret_addr = cpu.read_mem(cpu.gpr[4], 64);
                let ret_addr = if xfer_plain(cpu, target, cpu.gpr[4].wrapping_add(8)) {
                    cpu.gpr[4] = cpu.gpr[4].wrapping_add(8);
                    ret_addr
                } else {
                    jit_ras_publish(cpu);
                    host_tail_call(cpu, sp_ptr, prof, target, host_fn)
                };
                cpu.rip = ret_addr;
                if ret_addr == RET_SENTINEL || ret_addr == 0 {
                    return jit::EXIT_RIP;
                }
                if hops < 4096 && lookup_routed_regfile(ret_addr).is_some() {
                    hops += 1;
                    target = ret_addr;
                    from_call = false;
                    continue;
                }
                if lookup_routed_regfile(ret_addr).is_some() {
                    return jit::EXIT_RIP;
                }
                return jit_xfer_done(cpu);
            }
        }
        break;
    }
    cpu.rip = jit_guest_target(blocks, target);
    0
}

/// Guest target of a call/jmp from translated code (`jit_host_xfer`, not host-callable). An
/// AW2 exe import thunk whose slot holds guest code continues at the slot target directly,
/// as the run loop's [`exe_route`] would after an `EXIT_RIP`, so the exe thunk is never
/// translated. Any other target (including a host slot target, which the run loop handles)
/// is returned unchanged.
#[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]
#[inline(always)]
unsafe fn jit_guest_target(blocks: *mut BlockCache, target: u64) -> u64 {
    if in_exe_gate(exe_route_gate(), target) {
        return jit_guest_thunk_target(blocks, target);
    }
    target
}

#[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]
#[cold]
#[inline(never)]
unsafe fn jit_guest_thunk_target(blocks: *mut BlockCache, target: u64) -> u64 {
    if exe_route_off() {
        return target;
    }
    match exe_import_thunk_target(target) {
        Some(t) if t != 0 && BlockCache::resolve(blocks, t).is_none() => {
            exe_routed_inc();
            t
        }
        _ => target,
    }
}

#[cfg(test)]
std::thread_local! {
    /// Test-only: host transfers served by [`xfer_plain`] on this thread.
    static TEST_XFER_PLAIN: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
}

/// Host transfers served by [`xfer_plain`] on this thread (tests).
#[cfg(test)]
pub(super) fn test_xfer_plain() -> u64 {
    TEST_XFER_PLAIN.with(|c| c.get())
}

#[cfg(test)]
pub(super) fn note_test_xfer_plain(n: u64) {
    TEST_XFER_PLAIN.with(|c| c.set(c.get() + n));
}

#[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]
#[inline(always)]
fn is_qpc_host_fn(h: usize) -> bool {
    static QPC_FNS: std::sync::OnceLock<[usize; 4]> = std::sync::OnceLock::new();
    let fns = QPC_FNS.get_or_init(|| [
        win32_rt::kernel32::time::QueryPerformanceCounter as *const () as usize,
        win32_rt::ntdll::RtlQueryPerformanceCounter as *const () as usize,
        win32_rt::lookup("kernel32.dll", "QueryPerformanceCounter").map_or(0, |p| p as usize),
        win32_rt::lookup("ntdll.dll", "RtlQueryPerformanceCounter").map_or(0, |p| p as usize),
    ]);
    h != 0 && (h == fns[0] || h == fns[1] || h == fns[2] || h == fns[3])
}

/// `host_xfer` fast path for the targets the translator calls inline when they are static
/// ([`jit::inline_host_fn`]: `DF_PLAIN` builtins and [`inline_variant`]s) but that arrive
/// here through a register/memory-indirect call or jmp (`call rax`, `call [rbx+8]`, CFG
/// dispatch, ...): calls the function directly with the translator's inline `plain_call`
/// register contract (args = `abi_gpr` rcx/rdx/r8/r9 + `[rsp+0x20..0x40]` (`rsp` = `args_rsp`,
/// i.e. after popping the return address for tail jumps), xmm0..3; rax = x0, xmm0 = q0,
/// `last_bits[rax]` = full), without the bridge's per-thread setup, which these functions
/// never use (they never run guest code). Returns false (nothing done) when `target` has no
/// such function or a bail-out variant returned 0 (then make the real call).
#[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]
#[inline(always)]
unsafe fn xfer_plain(cpu: &mut Cpu, target: u64, args_rsp: u64) -> bool {
    let Some((h, bail, needs_xmm)) = jit::inline_host_fn_kind(target) else { return false };
    let r = args_rsp as usize;
    let gpr = [
        cpu.gpr[1],
        cpu.gpr[2],
        cpu.gpr[8],
        cpu.gpr[9],
        rd::<u64>(r + 0x20),
        rd::<u64>(r + 0x28),
        rd::<u64>(r + 0x30),
        rd::<u64>(r + 0x38),
    ];
    let xmm = if needs_xmm {
        [cpu.xmm[0], cpu.xmm[1], cpu.xmm[2], cpu.xmm[3]]
    } else {
        [0u128; 4]
    };
    let _fpcr = ClearFpcrGuard::enter_for_mxcsr(cpu.jx.mxcsr);
    let (rax, q0) = host_call_asm(h, &gpr, &xmm, &[0; 8]);
    if bail && rax == 0 {
        return false;
    }
    if !bail {
        if is_qpc_host_fn(h) {
            jit::JitCache::cache_qpc_host(cpu.jx.table, target, win32_rt::kernel32::time::qpc_fast_state_ptr());
        } else {
            jit::JitCache::cache_plain_host(cpu.jx.table, target, h, needs_xmm);
        }
    }
    if jit_stats::enabled() {
        jit_stats::xfer_plain();
    }
    #[cfg(test)]
    TEST_XFER_PLAIN.with(|c| c.set(c.get() + 1));
    cpu.gpr[0] = rax;
    if needs_xmm {
        cpu.xmm[0] = q0;
    }
    cpu.last_bits[0] = 64;
    true
}

/// After a host call from translated code: keep running translated code unless the host
/// dispatch table changed meanwhile (translations embed host-callable decisions).
/// `Cpu::jx.ymm_dirty` is maintained incrementally across interpreter and translated writes
/// (nested `emu_run_region` runs use their own `Cpu`), so no 256-byte `ymm_hi` scan is needed.
#[inline]
fn jit_xfer_done(cpu: &mut Cpu) -> u64 {
    #[cfg(debug_assertions)]
    for i in 0..16 {
        debug_assert!(
            cpu.ymm_hi[i] == 0 || (cpu.jx.ymm_dirty & (1u16 << i)) != 0,
            "ymm_dirty stale at reg {i} after jit_host_xfer: ymm_hi={:#x}, ymm_dirty={:#x}",
            cpu.ymm_hi[i],
            cpu.jx.ymm_dirty
        );
    }
    if crate::exe::va_gen() as u64 != cpu.jx.ctx_vagen {
        jit::EXIT_RIP
    } else {
        0
    }
}

/// Execute guest `x86-64` instructions starting at `entry_ip` with `args`.
pub unsafe fn emu_run(entry_ip: u64, args: [u64; 8]) -> u64 {
    let xmm = [
        args[0] as u128,
        args[1] as u128,
        args[2] as u128,
        args[3] as u128,
    ];
    let (gpr0, _xmm0) = emu_run_full(entry_ip, args, xmm, [0; 8]);
    gpr0
}

/// Execute guest `x86-64` instructions starting at `entry_ip` with explicit GPR and XMM arguments,
/// returning `(rax, xmm0)`.
pub unsafe fn emu_run_with_xmm(entry_ip: u64, args: [u64; 8], xmm_args: [u128; 4]) -> (u64, u128) {
    emu_run_full(entry_ip, args, xmm_args, [0; 8])
}

/// Execute guest `x86-64` instructions starting at `entry_ip` with explicit GPR, XMM, and
/// stack arguments 9..16 (`stack_args[0..8]`), returning `(rax, xmm0)`. The guest starts with
/// rax = 0 (see [`emu_run_full_rax`]).
///
/// For Rust callers holding live state: the region's EH record is opaque (an exception
/// dispatch leaving its guest frames stops there, as at a Windows frame without unwind info).
#[inline]
pub unsafe fn emu_run_full(
    entry_ip: u64,
    args: [u64; 8],
    xmm_args: [u128; 4],
    stack_args: [u64; 8],
) -> (u64, u128) {
    emu_run_region(entry_ip, args, xmm_args, stack_args, 0, true)
}

/// [`emu_run_full`] whose guest starts with `rax = rax_in`: a native (lifted) caller's rax at
/// its `call`, as a real x86 call leaves it. Lifted bodies return `get_rax_in()` on paths where
/// the original leaves rax untouched, and so does guest code: e.g. a lifted msvcp140
/// `std::exception` dtor calls `__std_exception_destroy` through its `jmp [IAT]` thunk (guest,
/// bound to guest vcruntime140) and returns the callee's untouched rax (the dtor's vftable
/// `lea`); with rax = 0 at the emulator entry the lifted dtor returned 0 where the original
/// returns the vftable (g15a difftest msvcp140 +0x1b6c / +0x1bec / +0x1c10).
/// A null `entry_ip` (`note_null_call`) still yields rax = 0, as the guest's `call 0` does.
///
/// For lifted callers (`call_va8*` bridges): the region's EH record is transparent, so an
/// exception dispatch continues from its outermost guest frame into the lifted caller.
/// Zero up to `bytes` immediately below the thread's current guest `sp` (clamped to `stack_min`),
/// so difftest's `unseeded_frame_read` confirmation re-run sees 0 in unwritten local stack slots
/// instead of `DllMain` startup residue, matching `LiftedFrame`'s clean-arena invariant.
pub unsafe fn zero_guest_stack_unused(bytes: usize) {
    TLS_EMU.with(|cell| {
        let opt = &mut *cell.get();
        let state = opt.get_or_insert_with(ThreadEmuState::new);
        if state.sp < state.stack_min || state.sp > state.stack_top {
            state.sp = state.stack_top;
        }
        let hi = state.sp as usize;
        let lo = hi.saturating_sub(bytes).max(state.stack_min as usize);
        if lo < hi {
            core::ptr::write_bytes(lo as *mut u8, 0, hi - lo);
        }
    });
}

#[inline]
pub unsafe fn emu_run_full_rax(
    entry_ip: u64,
    args: [u64; 8],
    xmm_args: [u128; 4],
    stack_args: [u64; 8],
    rax_in: u64,
) -> (u64, u128) {
    emu_run_region(entry_ip, args, xmm_args, stack_args, rax_in, false)
}

/// Maximum fast ops executed in one [`interp_burst`] invocation before returning to
/// `emu_run_region` to re-check outer budgets, profiler sampling, and epoch counters.
const INTERP_BURST_BUDGET: u32 = 8192;

#[cfg(test)]
std::thread_local! {
    /// Test-only: total [`interp_burst`] entries on this thread.
    static TEST_INTERP_BURSTS: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
    /// Test-only: total inter-block and self-loop block transitions performed inside [`interp_burst`].
    static TEST_INTERP_BURST_CHAINS: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
}

/// Outcome of an [`interp_burst`] run.
struct BurstExit {
    fx: fast::Fx,
    steps: u32,
    last_ip: u64,
    next_off: usize,
    next_left: usize,
}

/// Whether `fast::Cpu::rset` must maintain `Cpu::last_bits` (needed only when `DF_GAME_REPL`
/// hooks or differential verification / tests are active).
#[inline(always)]
fn fast_last_bits_on(verify: bool) -> bool {
    if cfg!(test) || verify {
        return true;
    }
    #[cfg(target_arch = "aarch64")]
    {
        jit::last_bits_enabled()
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        crate::exe::aw2_registry_active()
    }
}

/// Compute the loop-invariant portion of the mode-flags word for [`BlockCache::sync_fast_gen`].
#[inline(always)]
fn burst_base_mode_flags(jit_on: bool, dll_gate: (u64, u64)) -> u32 {
    let gate_hash = (((dll_gate.0 >> 12) ^ (dll_gate.1 >> 12)) as u32) << 16;
    #[cfg(not(test))]
    let fast_bit = u32::from(fast_enabled()) << 11;
    #[cfg(test)]
    let fast_bit = 0u32;
    (u32::from(jit_on) << 4)
        | (u32::from(crate::exe::aw2_registry_active()) << 5)
        | (u32::from(exe_route_off()) << 6)
        | (u32::from(dll_route_enabled()) << 7)
        | (u32::from(dll_chkstk_enabled()) << 8)
        | (u32::from(dll_stack_pos_enabled()) << 9)
        | fast_bit
        | gate_hash
}

/// Combine [`burst_base_mode_flags`] with per-burst dynamic bits (`jit_tag`, `aot`, test overrides).
#[inline(always)]
fn burst_mode_flags_from_base(base: u32, jit_ptr: *const jit::JitCache) -> u32 {
    #[cfg(target_arch = "aarch64")]
    let jit_tag = unsafe { (*jit_ptr).mode_tag() as u32 };
    #[cfg(not(target_arch = "aarch64"))]
    let jit_tag = {
        let _ = jit_ptr;
        0u32
    };
    #[cfg(target_arch = "aarch64")]
    let aot_on = u32::from(aot::active());
    #[cfg(not(target_arch = "aarch64"))]
    let aot_on = 0u32;
    #[cfg(test)]
    let test_bits = (u32::from(fast_enabled()) << 11)
        | (u32::from(DLL_CHKSTK_OFF.with(|c| c.get())) << 12)
        | (u32::from(DLL_STACK_POS_OFF.with(|c| c.get())) << 13)
        | (u32::from(regfile::test_off()) << 14);
    #[cfg(not(test))]
    let test_bits = 0u32;
    base | (jit_tag & 0xF) | (aot_on << 10) | test_bits
}

/// Compact out-of-line interpreter burst loop: executes pre-decoded [`fast::FastOp`] blocks
/// and chains directly across 0-hash self-loops, cached direct `(taken_slot, fall_slot)` edges,
/// and stamped 32-byte [`BlkEntry`] slots for `jcc`, `jmp`, `call`, `ret`, and fallthrough up
/// to `budget` ops (`<= INTERP_BURST_BUDGET`).
///
/// Only entered on blocks stamped with `fast_gen` (guaranteed `e.gen == 0`, non-generic start,
/// sticky-interpreted, non-host, non-hook/chkstk/IAT/DLL-stub). Straight-line non-branch runs
/// omit per-op `cpu.rip` stores and `(*nx).ip == cpu.rip` checks, materializing `cpu.rip` at
/// block ends, branches, calls, and exits. Exits cleanly back to `emu_run_region` on index
/// miss, `K_GENERIC`, host/special transfer, budget expiry, or fault.
#[inline(never)]
unsafe fn interp_burst(
    cpu: &mut Cpu,
    blocks_ptr: *mut BlockCache,
    mut cur_slot: usize,
    cur_off: usize,
    cur_len: usize,
    fast_gen: u32,
    budget: u32,
) -> BurstExit {
    #[cfg(test)]
    TEST_INTERP_BURSTS.with(|c| c.set(c.get().wrapping_add(1)));

    let ops_base = (*blocks_ptr).ops.as_ptr();
    let index_base = (*blocks_ptr).index.as_mut_ptr();
    let mut blk_start = ops_base.add(cur_off);
    let mut blk_ip = (*blk_start).ip;
    let mut blk_len = cur_len;
    let mut op = blk_start;
    let mut left = cur_len - 1;
    let mut n = 0u32;

    macro_rules! resolve_chain {
        ($target:expr, $is_fall:expr) => {{
            let tgt: u64 = $target;
            if tgt == blk_ip {
                Some((cur_slot, blk_start, blk_len))
            } else {
                let cur_ent = index_base.add(cur_slot);
                let cached = if $is_fall { (*cur_ent).fall_slot } else { (*cur_ent).taken_slot };
                let mut hit: Option<(usize, *const fast::FastOp, usize)> = None;
                if cached != 0 {
                    let s = (cached - 1) as usize;
                    let e = *index_base.add(s);
                    if e.ip == tgt && e.fast_gen == fast_gen {
                        let ptr = ops_base.add(e.off as usize);
                        if (*ptr).ip == tgt {
                            hit = Some((s, ptr, e.len as usize));
                        }
                    }
                }
                if hit.is_none() {
                    let s = BlockCache::slot(tgt);
                    let e = *index_base.add(s);
                    if e.ip == tgt && e.fast_gen == fast_gen {
                        let ptr = ops_base.add(e.off as usize);
                        if (*ptr).ip == tgt {
                            let enc = (s + 1) as u32;
                            if $is_fall {
                                (*cur_ent).fall_slot = enc;
                            } else {
                                (*cur_ent).taken_slot = enc;
                            }
                            hit = Some((s, ptr, e.len as usize));
                        }
                    }
                }
                hit
            }
        }};
    }

    loop {
        if left == 0 {
            cpu.rip = (*op).ip.wrapping_add((*op).len as u64);
        }
        let mut fx = fast::exec(cpu, &*op);
        n += 1;
        if matches!(fx, fast::Fx::FuseJcc) {
            // `fast::fuse` guarantees the next slot of this block is the paired `K_JCC` (end of block).
            op = op.add(1);
            left -= 1;
            cpu.rip = (*op).ip.wrapping_add((*op).len as u64);
            fx = fast::exec(cpu, &*op);
            n += 1;
        }
        match fx {
            fast::Fx::Done => {
                if left != 0 {
                    let nx = op.add(1);
                    debug_assert_eq!((*nx).ip, (*op).ip.wrapping_add((*op).len as u64));
                    if n < budget && (*nx).kind != fast::K_GENERIC {
                        op = nx;
                        left -= 1;
                        continue;
                    }
                    cpu.rip = (*nx).ip;
                    return BurstExit {
                        fx: fast::Fx::Done,
                        steps: n,
                        last_ip: (*op).ip,
                        next_off: nx.offset_from(ops_base) as usize,
                        next_left: left,
                    };
                }
                if n >= budget {
                    return BurstExit {
                        fx: fast::Fx::Done,
                        steps: n,
                        last_ip: (*op).ip,
                        next_off: 0,
                        next_left: 0,
                    };
                }
                let target = cpu.rip;
                let is_fall = target == (*op).ip.wrapping_add((*op).len as u64);
                if let Some((slot, ptr, len)) = resolve_chain!(target, is_fall) {
                    #[cfg(test)]
                    TEST_INTERP_BURST_CHAINS.with(|c| c.set(c.get().wrapping_add(1)));
                    cur_slot = slot;
                    blk_ip = target;
                    blk_start = ptr;
                    blk_len = len;
                    op = ptr;
                    left = len - 1;
                    continue;
                }
                return BurstExit {
                    fx: fast::Fx::Done,
                    steps: n,
                    last_ip: (*op).ip,
                    next_off: 0,
                    next_left: 0,
                };
            }
            fast::Fx::JccNear(target) | fast::Fx::Jmp(target) => {
                if n < budget {
                    if let Some((slot, ptr, len)) = resolve_chain!(target, false) {
                        #[cfg(test)]
                        TEST_INTERP_BURST_CHAINS.with(|c| c.set(c.get().wrapping_add(1)));
                        cpu.rip = target;
                        cur_slot = slot;
                        blk_ip = target;
                        blk_start = ptr;
                        blk_len = len;
                        op = ptr;
                        left = len - 1;
                        continue;
                    }
                }
                return BurstExit {
                    fx,
                    steps: n,
                    last_ip: (*op).ip,
                    next_off: 0,
                    next_left: 0,
                };
            }
            fast::Fx::Call(target) => {
                if n < budget {
                    let new_rsp = cpu.gpr[4].wrapping_sub(8);
                    if regfile_ext::is_valid_guest_range(new_rsp, 8) {
                        if let Some((slot, ptr, len)) = resolve_chain!(target, false) {
                            #[cfg(test)]
                            TEST_INTERP_BURST_CHAINS.with(|c| c.set(c.get().wrapping_add(1)));
                            cpu.gpr[4] = new_rsp;
                            cpu.write_mem(new_rsp, 64, cpu.rip);
                            cpu.rip = target;
                            cur_slot = slot;
                            blk_ip = target;
                            blk_start = ptr;
                            blk_len = len;
                            op = ptr;
                            left = len - 1;
                            continue;
                        }
                    }
                }
                return BurstExit {
                    fx,
                    steps: n,
                    last_ip: (*op).ip,
                    next_off: 0,
                    next_left: 0,
                };
            }
            fast::Fx::FuseJcc => unreachable!(),
        }
    }
}

/// One emulator run (see [`emu_run_full_rax`]); `eh_opaque` = its EH record's opacity.
#[inline(never)]
unsafe fn emu_run_region(
    entry_ip: u64,
    args: [u64; 8],
    xmm_args: [u128; 4],
    stack_args: [u64; 8],
    rax_in: u64,
    eh_opaque: bool,
) -> (u64, u128) {
    if entry_ip == 0 {
        // A host/lifted call through a null pointer (`call_va*(0)`): the loop below does not
        // run and the caller goes on with rax as is (natively a fault).
        note_null_call();
    }
    // Lifted bytes live below the innermost guest -> host transition (`crate::lstack` "Stack
    // account"): the region starts that much lower, where a native callee would sit.
    let lifted = crate::lstack::lifted_depth(crate::callctx::ctx()) as u64;
    let (sp_ptr, saved_sp, sp_top, stack_lo, stack_floor, stack_hi, blocks_ptr, jit_ptr, overflow) = TLS_EMU.with(|cell| {
        let opt = &mut *cell.get();
        let state = opt.get_or_insert_with(ThreadEmuState::new);
        if state.sp < state.stack_min || state.sp > state.stack_top {
            state.sp = state.stack_top;
        }
        let saved = state.sp;
        let start = saved.saturating_sub(lifted);
        let overflow = lifted != 0 && start < state.stack_min + GUEST_FLOOR_MARGIN;
        // Win64 ABI: at function entry (right after CALL pushes 8-byte return address),
        // RSP % 16 == 8, with 32-byte shadow space at [RSP+8..RSP+0x28] and stack args at [RSP+0x28..].
        let top = (start.wrapping_sub(320) & !0xFu64).wrapping_sub(8);
        if !overflow {
            state.sp = (top - 256) & !0xFu64;
        }
        let alloc_base = state.stack_min.saturating_sub(GUEST_STACK_MIN_OFF);
        let stack_floor = state.stack_min + GUEST_FLOOR_MARGIN;
        (
            &mut state.sp as *mut u64,
            saved,
            top,
            alloc_base,
            stack_floor,
            state.stack_top,
            &mut state.blocks as *mut BlockCache,
            &mut state.jit as *mut jit::JitCache,
            overflow,
        )
    });
    if overflow {
        // Nothing changed yet (guest sp, EH chain): as a guard-page hit at the callee's entry.
        crate::lstack::raise_stack_overflow(crate::lstack::Overflow::Region);
    }
    // Keep `TEB.StackBase` (`gs:[0x08]`) / `TEB.StackLimit` (`gs:[0x10]`) matching the active
    // heap guest stack when the TEB currently points at the host pthread's C stack, so unhooked
    // `__chkstk` helpers (e.g. inside packed binaries) never probe downward from the host stack
    // limit into the host guard page. Leave custom pivoted stacks (fibers / tests) untouched.
    let saved_teb_stack = {
        let teb = teb_self();
        if teb != 0 {
            let old_base = rd::<u64>(teb + 0x08);
            let old_limit = rd::<u64>(teb + 0x10);
            let host_sp = &overflow as *const bool as u64;
            if old_limit <= host_sp && host_sp <= old_base {
                let old_dealloc = rd::<u64>(teb + 0x1478);
                wr::<u64>(teb + 0x08, stack_hi);
                wr::<u64>(teb + 0x10, stack_floor);
                wr::<u64>(teb + 0x1478, stack_lo);
                Some((teb, old_base, old_limit, old_dealloc))
            } else {
                None
            }
        } else {
            None
        }
    };
    let _sp_guard = SpGuard { sp_ptr, saved_sp, saved_teb_stack };
    let (caller_rsp, rsp_limit) = take_incoming_rsp();

    wr::<u64>(sp_top as usize, RET_SENTINEL);
    wr::<u64>((sp_top + 0x08) as usize, 0);
    wr::<u64>((sp_top + 0x10) as usize, 0);
    wr::<u64>((sp_top + 0x18) as usize, 0);
    wr::<u64>((sp_top + 0x20) as usize, 0);
    wr::<u64>((sp_top + 0x28) as usize, args[4]);
    wr::<u64>((sp_top + 0x30) as usize, args[5]);
    wr::<u64>((sp_top + 0x38) as usize, args[6]);
    wr::<u64>((sp_top + 0x40) as usize, args[7]);
    for i in 0..8 {
        wr::<u64>((sp_top + 0x48 + (i as u64) * 8) as usize, stack_args[i]);
    }
    // Only populate high stack args [sp_top+0x88..sp_top+0x108) when forwarding from a guest/lifted
    // caller frame (`caller_rsp != 0`); direct `emu_run*` callers pass at most 16 args.
    if caller_rsp != 0 {
        for i in 0..16 {
            let addr = caller_rsp.wrapping_add(0x80 + (i as u64) * 8);
            let v = if addr.wrapping_add(8) <= rsp_limit { rd::<u64>(addr as usize) } else { 0 };
            wr::<u64>((sp_top + 0x88 + (i as u64) * 8) as usize, v);
        }
    }

    let mut cpu = Cpu {
        gpr: [0; 16],
        xmm: [0; 16],
        ymm_hi: [0; 16],
        rip: entry_ip,
        cf: false,
        zf: false,
        sf: false,
        of: false,
        pfv: 1,
        df: false,
        last_bits: [64; 16],
        jx: jit::JitScratch {
            teb: teb_self() as u64,
            mxcsr: win32_rt::ucrt::runtime::guest_mxcsr() & 0xFFFF,
            ..jit::JitScratch::default()
        },
    };
    let _fpcr_guard = GuestFpcrGuard::enter(&cpu.jx.mxcsr as *const u32);
    if entry_ip != 0 {
        cpu.gpr[0] = rax_in; // rax
    }
    cpu.gpr[1] = args[0]; // rcx
    cpu.gpr[2] = args[1]; // rdx
    cpu.gpr[8] = args[2]; // r8
    cpu.gpr[9] = args[3]; // r9
    cpu.gpr[4] = sp_top;  // rsp
    cpu.xmm[0] = xmm_args[0];
    cpu.xmm[1] = xmm_args[1];
    cpu.xmm[2] = xmm_args[2];
    cpu.xmm[3] = xmm_args[3];
    let _cur_cpu = CurCpuGuard(CUR_CPU.with(|c| c.replace(&cpu as *const Cpu)));
    let mut eh_rec = eh::EmuEntry::new(&cpu as *const Cpu, entry_ip, eh_opaque);
    let _eh_rec = eh_rec.push();

        let mut prev_ip = entry_ip;
    const MAX_STEPS: usize = usize::MAX / 2;

    // Interpreter profiler (see `emu_prof`). With profiling off `g.sample_left` starts at
    // u32::MAX, so the per-instruction cost is a single decrement + never-taken branch.
    let prof = crate::emu_prof::thread_prof();
    let prof_mnem = prof.is_some() && crate::emu_prof::level() >= 2;
    // Level 2 counts only generic-path mnemonics (fast-path candidates); level 3 counts all.
    let prof_mnem_fast = prof.is_some() && crate::emu_prof::level() >= 3;
    // Context for host calls made from translated code (`jit_host_xfer`).
    cpu.jx.ctx_sp = sp_ptr as usize;
    cpu.jx.ctx_blocks = blocks_ptr as usize;
    cpu.jx.ctx_prof = prof.map_or(0, |p| p as *const crate::emu_prof::ThreadProf as usize);
    cpu.jx.ctx_entry = entry_ip;
    let mut g = ProfGuard {
        prof,
        entry_ip,
        steps: 0,
        misses: 0,
        flushed_steps: 0,
        flushed_misses: 0,
        sample_left: prof.map_or(u32::MAX, |p| p.sample_ctr.get().max(1)),
    };

    let mut blk_off: usize = 0;
    let mut blk_left: usize = 0;
    let (ops_base, ins_base): (*const fast::FastOp, *const Instruction) =
        ((*blocks_ptr).ops.as_ptr(), (*blocks_ptr).arena.as_ptr());
    let verify = verify_enabled();
    cpu.jx.lb_on = u8::from(fast_last_bits_on(verify));
    // Translated code bypasses per-instruction verification and mnemonic counting.
    // Precise guest faults (difftest child only): interpreted, one instruction at a time.
    let precise = precise_faults();
    let nan_inst = crate::diag::nan_trace_installed();
    let mut pre: Option<Cpu> = None;
    let jit_on = jit::enabled() && !verify && !prof_mnem && !precise;
    // Diagnostic translator coverage counters (`AW2_JIT_STATS`, default off).
    let jstats = jit_on && jit_stats::enabled();
    let mut jstats_rip = false;
    // Set after a translator side exit: interpret the next instruction before re-entering.
    let mut jit_skip = false;
    let mut shadow: Option<Cpu> = None;
    // AW2 exe image span: guest block starts in it go through `exe_route` (import thunks,
    // special-ABI helpers). A zero span when the AW2 registry isn't applied.
    let exe_gate = exe_route_gate();
    // Mapped-DLL span with IAT directories: guest block starts in it that are import stubs
    // go to their slot target (`dll_stub_route`). A zero span when routing is off.
    let dll_gate = dll_route_gate();
    let burst_base_flags = burst_base_mode_flags(jit_on, dll_gate);
    let mut route_hops: usize = 0;

    while cpu.rip != RET_SENTINEL && cpu.rip != 0 && g.steps < MAX_STEPS && route_hops < MAX_STEPS {
        if precise {
            if let Some((addr, write)) = FAULT_PENDING.with(|p| p.take()) {
                if let Some(p) = pre.as_ref() {
                    cpu.clone_from(p);
                }
                guest_fault_dispatch(&mut cpu, sp_ptr, addr, write);
                blk_left = 0;
                continue;
            }
            match pre.as_mut() {
                Some(p) => p.clone_from(&cpu),
                None => pre = Some(cpu.clone()),
            }
        }
        let cur_ip = cpu.rip;
        let nan_step = nan_inst && crate::diag::nan_trace_covers(cur_ip);
        // Hot-path bypass: a valid compiled block in `JitCache` (not `static_only`, no bound
        // static AOT, matching `va_gen`/`aot_epoch`/`static_aot_gen`) already passed `prep_block`
        // / `aot::check_block`, which rejects `dll_chkstk`, `dll_stack_pos`, `regfile`, and
        // `dll_stub` VAs. Skip the 5 routing probes when outside `exe_gate` and no `SKIP_VA` is armed.
        let fast_jit_code = if jit_on
            && !jit_skip
            && !nan_step
            && !in_exe_gate(exe_gate, cur_ip)
            && !regfile::has_skip_armed()
        {
            #[cfg(test)]
            let test_routed = is_routed_dll_chkstk(cur_ip)
                || is_routed_dll_stack_pos(cur_ip)
                || lookup_routed_regfile(cur_ip).is_some();
            #[cfg(not(test))]
            let test_routed = false;
            if test_routed {
                None
            } else {
                (*jit_ptr).lookup_fast_hit(cur_ip)
            }
        } else {
            None
        };
        if fast_jit_code.is_none() {
            if in_exe_gate(exe_gate, cur_ip) {
                let route = exe_route(&mut cpu, cur_ip, entry_ip);
                if route != ExeRoute::None {
                    if let Some(r) = apply_exe_route(&mut cpu, route, entry_ip, blocks_ptr, sp_ptr, prof) {
                        return r;
                    }
                    route_hops += 1;
                    continue;
                }
            }
            if is_routed_dll_chkstk(cur_ip) {
                if let Some(r) = dll_chkstk_tail(&mut cpu, sp_ptr) {
                    return r;
                }
                route_hops += 1;
                continue;
            }
            if is_routed_dll_stack_pos(cur_ip) {
                if let Some(r) = dll_stack_pos_tail(&mut cpu) {
                    return r;
                }
                route_hops += 1;
                continue;
            }
            if regfile::take_skip(cur_ip) {
                jit_skip = true;
            } else if let Some((rf, blk)) = lookup_routed_regfile(cur_ip) {
                let ex = regfile::run_chain(&mut cpu, rf, blk);
                match ex.kind {
                    regfile::RF_RET => {
                        if ex.va == RET_SENTINEL || ex.va == 0 {
                            if ex.va == 0 && entry_ip != 0 {
                                note_null_call();
                            }
                            return (cpu.abi_gpr(0), cpu.xmm[0]);
                        }
                        cpu.rip = ex.va;
                        route_hops += 1;
                        continue;
                    }
                    regfile::RF_JUMP => {
                        if ex.va == RET_SENTINEL || ex.va == 0 {
                            if ex.va == 0 && entry_ip != 0 {
                                note_null_call();
                            }
                            return (cpu.abi_gpr(0), cpu.xmm[0]);
                        }
                        if let Some(r) = apply_xfer_tail(&mut cpu, ex.va, false, entry_ip, blocks_ptr, sp_ptr, prof) {
                            return r;
                        }
                        route_hops += 1;
                        continue;
                    }
                    regfile::RF_CALL => {
                        if let Some(r) = apply_xfer_tail(&mut cpu, ex.va, true, entry_ip, blocks_ptr, sp_ptr, prof) {
                            return r;
                        }
                        route_hops += 1;
                        continue;
                    }
                    _ => {
                        cpu.rip = ex.va;
                        regfile::arm_skip(ex.va);
                        jit_skip = true;
                        route_hops += 1;
                        continue;
                    }
                }
            }
            if cur_ip.wrapping_sub(dll_gate.0) < dll_gate.1 {
                if let Some(t) = dll_stub_route(cur_ip) {
                    // The stub's `jmp [slot]`: host target = host tail call, guest = plain jump.
                    if let Some(r) = apply_fx(&mut cpu, fast::Fx::Jmp(t), entry_ip, blocks_ptr, sp_ptr, prof) {
                        return r;
                    }
                    route_hops += 1;
                    continue;
                }
            }
        }
        g.steps += 1;
        if verify {
            shadow = None;
        }
        g.sample_left = g.sample_left.wrapping_sub(1);
        if g.sample_left == 0 {
            g.sample_left = if prof.is_some() { crate::emu_prof::SAMPLE_PERIOD } else { u32::MAX };
            if let Some(p) = prof {
                p.sample_ip(cur_ip);
                p.func_progress(entry_ip, (g.steps - g.flushed_steps) as u64, g.misses - g.flushed_misses);
                g.flushed_steps = g.steps;
                g.flushed_misses = g.misses;
            }
        }
        let skipped_jit = core::mem::take(&mut jit_skip);
        if jit_on && !skipped_jit && !nan_step {
            if g.sample_left <= BLK_MAX_INSNS as u32 {
                // Take the profiler sample now rather than interpreting up to a block's
                // worth of instructions until the countdown reaches 0.
                g.sample_left = if prof.is_some() { crate::emu_prof::SAMPLE_PERIOD } else { u32::MAX };
                if let Some(p) = prof {
                    p.sample_ip(cur_ip);
                    p.func_progress(entry_ip, (g.steps - g.flushed_steps) as u64, g.misses - g.flushed_misses);
                    g.flushed_steps = g.steps;
                    g.flushed_misses = g.misses;
                }
            }
            let after_rip_exit = core::mem::take(&mut jstats_rip);
            if let Some(code) = fast_jit_code.or_else(|| (*jit_ptr).lookup(cur_ip, blocks_ptr)) {
                let mut budget = g.sample_left as i64 - 1;
                if nan_inst {
                    budget = budget.min(4096);
                }
                let (ex, executed) = (*jit_ptr).run(&mut cpu, code, budget);
                if jstats {
                    if after_rip_exit {
                        jit_stats::rip_rehit();
                    }
                    jit_stats::jit_run(ex, executed, cpu.jx.target, cpu.rip);
                    jstats_rip = ex == jit::EXIT_RIP;
                }
                g.steps = g.steps - 1 + executed as usize;
                g.sample_left = (g.sample_left as i64 - executed).max(1) as u32;
                blk_left = 0;
                let fx = match ex {
                    jit::EXIT_CALL => fast::Fx::Call(cpu.jx.target),
                    jit::EXIT_JMP => fast::Fx::Jmp(cpu.jx.target),
                    jit::EXIT_JCC => fast::Fx::JccNear(cpu.jx.target),
                    jit::EXIT_SIDE => {
                        jit_skip = true;
                        continue;
                    }
                    _ => continue,
                };
                if let Some(r) = apply_fx(&mut cpu, fx, entry_ip, blocks_ptr, sp_ptr, prof) {
                    return r;
                }
                continue;
            }
        }
        let mut head_slot: Option<(usize, usize)> = None;
        let cur_off = if blk_left != 0 && (*ops_base.add(blk_off)).ip == cur_ip {
            blk_left -= 1;
            blk_off += 1;
            blk_off - 1
        } else {
            match BlockCache::get(blocks_ptr, cur_ip, &mut g.misses) {
                Ok((off, n)) => {
                    blk_off = off + 1;
                    blk_left = n - 1;
                    head_slot = Some((BlockCache::slot(cur_ip), n));
                    off
                }
                Err(BlkErr::BadRip(mapped_ip)) => {
                    log::warn!(
                        "emu: invalid rip={:#x} (mapped={:#x}) from prev_ip={:#x} entry={:#x} step={}",
                        cur_ip,
                        mapped_ip,
                        prev_ip,
                        entry_ip,
                        g.steps
                    );
                    break;
                }
                Err(BlkErr::Invalid(bytes)) => {
                    log::warn!("emu: invalid instruction at rip={:#x} bytes={:02x?}", cur_ip, &bytes[..]);
                    break;
                }
            }
        };
        let op = &*ops_base.add(cur_off);
        prev_ip = cur_ip;
        cpu.rip = cur_ip.wrapping_add(op.len as u64);
        if jstats {
            jit_stats::interp(cur_ip, (*ins_base.add(cur_off)).mnemonic(), op.kind == fast::K_GENERIC);
        }

        if op.kind != fast::K_GENERIC {
            if prof_mnem_fast {
                if let Some(p) = prof {
                    p.mnemonic((*ins_base.add(cur_off)).mnemonic());
                }
            }
            if !verify || !fast::verifiable(op.kind) {
                let fx;
                if !verify && !prof_mnem_fast && !precise && !nan_step {
                    let burst_ok = if !prof_mnem && !jstats && !skipped_jit {
                        if let Some((slot, blk_len)) = head_slot {
                            let mflags = burst_mode_flags_from_base(burst_base_flags, jit_ptr);
                            let fast_gen = (*blocks_ptr).sync_fast_gen(mflags);
                            let ent = &mut *(*blocks_ptr).index.as_mut_ptr().add(slot);
                            if ent.ip == cur_ip
                                && ent.off as usize == cur_off
                                && ent.len as usize == blk_len
                                && ent.gen == 0
                            {
                                let stamped = if ent.fast_gen == fast_gen {
                                    true
                                } else {
                                    #[cfg(target_arch = "aarch64")]
                                    let sticky = !jit_on || (*jit_ptr).is_interp_sticky(cur_ip);
                                    #[cfg(not(target_arch = "aarch64"))]
                                    let sticky = true;
                                    let safe = sticky
                                        && !in_exe_gate(exe_gate, cur_ip)
                                        && !(cur_ip == AW2_CHKSTK && crate::exe::aw2_registry_active())
                                        && !is_routed_dll_chkstk(cur_ip)
                                        && !is_routed_dll_stack_pos(cur_ip)
                                        && lookup_routed_regfile(cur_ip).is_none()
                                        && !is_routed_dll_stub(cur_ip)
                                        && BlockCache::resolve(blocks_ptr, cur_ip).is_none();
                                    if safe {
                                        ent.fast_gen = fast_gen;
                                    }
                                    safe
                                };
                                if stamped {
                                    Some((slot, blk_len, fast_gen))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    if let Some((slot, blk_len, fast_gen)) = burst_ok {
                        let step_cap = (MAX_STEPS - g.steps + 1).min(u32::MAX as usize) as u32;
                        let burst_budget = INTERP_BURST_BUDGET
                            .min(g.sample_left.max(1))
                            .min(step_cap)
                            .max(1);
                        let exit = interp_burst(
                            &mut cpu,
                            blocks_ptr,
                            slot,
                            cur_off,
                            blk_len,
                            fast_gen,
                            burst_budget,
                        );
                        let extra = (exit.steps - 1) as usize;
                        blk_off = exit.next_off;
                        blk_left = exit.next_left;
                        g.steps += extra;
                        g.sample_left = g.sample_left.saturating_sub(extra as u32).max(1);
                        prev_ip = exit.last_ip;
                        fx = exit.fx;
                    } else {
                        let (f, n, last) = fast::run_chain(&mut cpu, op, blk_left, g.sample_left.max(1));
                        let extra = (n - 1) as usize;
                        if jstats && extra != 0 {
                            jit_stats::interp_chain(extra as u64);
                        }
                        blk_off += extra;
                        blk_left -= extra;
                        g.steps += extra;
                        g.sample_left = g.sample_left.saturating_sub(extra as u32).max(1);
                        prev_ip = (*last).ip;
                        fx = f;
                    }
                } else {
                    let pre_fp = if nan_step {
                        Some(crate::diag::pre_ins(&cpu, &*ins_base.add(cur_off)))
                    } else {
                        None
                    };
                    let mut f = fast::exec(&mut cpu, op);
                    if let Some(ref pf) = pre_fp {
                        crate::diag::post_ins(pf, &cpu, cur_ip, &*ins_base.add(cur_off));
                    }
                    if let fast::Fx::FuseJcc = f {
                        // `fuse` guarantees the next slot of this block is the jcc.
                        let j = &*ops_base.add(blk_off);
                        blk_off += 1;
                        blk_left -= 1;
                        g.steps += 1;
                        prev_ip = j.ip;
                        cpu.rip = j.ip.wrapping_add(j.len as u64);
                        f = fast::exec(&mut cpu, j);
                    }
                    fx = f;
                }
                if let Some(r) = apply_fx(&mut cpu, fx, entry_ip, blocks_ptr, sp_ptr, prof) {
                    return r;
                }
                continue;
            }
            // Verify mode: run the fast op on a shadow copy, then let the generic
            // handler below execute for real and compare at the end of the iteration.
            let mut s = cpu.clone();
            if let fast::Fx::JccNear(t) = fast::exec(&mut s, op) {
                s.rip = t;
            }
            shadow = Some(s);
        }

        let ins = *ins_base.add(cur_off);

        let mn = ins.mnemonic();
        if prof_mnem {
            if let Some(p) = prof {
                p.mnemonic(mn);
            }
        }
        match mn {
            Mnemonic::Int3 | Mnemonic::Ud2 => {
                let code = if mn == Mnemonic::Int3 {
                    win32_rt::unwind::STATUS_BREAKPOINT
                } else {
                    win32_rt::unwind::STATUS_ILLEGAL_INSTRUCTION
                };
                let rflags = cpu.rflags();
                if let Some((new_rip, new_eflags)) = win32_rt::unwind::try_dispatch_veh_trap(
                    code,
                    cur_ip,
                    &mut cpu.gpr,
                    &mut cpu.xmm,
                    rflags,
                ) {
                    cpu.rip = new_rip;
                    cpu.set_rflags(new_eflags);
                    for lb in &mut cpu.last_bits {
                        *lb = 64;
                    }
                    continue;
                }
                if int3_returns() {
                    // A/B only (`AW2_EMU_INT3=return`): the old behaviour, the guest function
                    // "returns" whatever is in rax/xmm0.
                    break;
                }
                // Unhandled, as on Windows: no silent return (V8_Fatal -> OS::Abort -> int3
                // used to "return" and leave cohtml running on a corrupt heap).
                let mut rec = win32_rt::unwind::guest_exception_record(code, cur_ip, &[]);
                let mut ctx = win32_rt::unwind::guest_context(&cpu.gpr, &cpu.xmm, cur_ip, rflags);
                if win32_rt::unwind::emu_unhandled_exception(&mut rec, &mut ctx, true) {
                    cpu.gpr = ctx.gpr;
                    for i in 0..16 {
                        cpu.xmm[i] = u128::from_le_bytes(*ctx.xmm_mut(i));
                    }
                    cpu.rip = ctx.rip;
                    cpu.set_rflags(ctx.eflags);
                    for lb in &mut cpu.last_bits {
                        *lb = 64;
                    }
                    continue;
                }
                break;
            }
            // `int n` (`CD ib`): as on Windows (and as `lift_ops::trap` raises it): 0x29 is
            // `__fastfail` (STATUS_STACK_BUFFER_OVERRUN, information[0] = rcx; skips the vectored
            // handlers and is never resumed), 0x2c an assertion failure, 3 a breakpoint, every
            // other vector a #GP in user mode (STATUS_ACCESS_VIOLATION). It used to fall through
            // to the unimplemented no-op, so a fast-fail went on past the `int`.
            Mnemonic::Int => {
                let n = ins.immediate8();
                let code = match n {
                    0x29 => win32_rt::unwind::STATUS_STACK_BUFFER_OVERRUN,
                    0x2c => 0xC000_0420,
                    3 => win32_rt::unwind::STATUS_BREAKPOINT,
                    _ => win32_rt::unwind::STATUS_ACCESS_VIOLATION,
                };
                let fastfail = n == 0x29;
                let rflags = cpu.rflags();
                if !fastfail {
                    if let Some((new_rip, new_eflags)) =
                        win32_rt::unwind::try_dispatch_veh_trap(code, cur_ip, &mut cpu.gpr, &mut cpu.xmm, rflags)
                    {
                        cpu.rip = new_rip;
                        cpu.set_rflags(new_eflags);
                        for lb in &mut cpu.last_bits {
                            *lb = 64;
                        }
                        continue;
                    }
                }
                if int3_returns() {
                    break;
                }
                let info = [cpu.gpr[1]];
                let mut rec = win32_rt::unwind::guest_exception_record(code, cur_ip, if fastfail { &info[..] } else { &[] });
                let mut ctx = win32_rt::unwind::guest_context(&cpu.gpr, &cpu.xmm, cur_ip, rflags);
                if win32_rt::unwind::emu_unhandled_exception(&mut rec, &mut ctx, !fastfail) && !fastfail {
                    cpu.gpr = ctx.gpr;
                    for i in 0..16 {
                        cpu.xmm[i] = u128::from_le_bytes(*ctx.xmm_mut(i));
                    }
                    cpu.rip = ctx.rip;
                    cpu.set_rflags(ctx.eflags);
                    for lb in &mut cpu.last_bits {
                        *lb = 64;
                    }
                    continue;
                }
                break;
            }
            Mnemonic::Syscall | Mnemonic::Sysenter => {
                let ssn = cpu.gpr[0] as u32;
                let r10 = if cpu.gpr[10] != 0 { cpu.gpr[10] } else { cpu.gpr[1] };
                let rdx = cpu.gpr[2];
                let r8 = cpu.gpr[8];
                let r9 = cpu.gpr[9];
                let rsp = cpu.gpr[4];
                // Architectural x86-64 SYSCALL saves next RIP in RCX and RFLAGS in R11
                cpu.gpr[1] = cpu.rip;
                cpu.gpr[11] = cpu.rflags() as u64;
                let status = win32_rt::ntsyscall::dispatch_nt_syscall(ssn, r10, rdx, r8, r9, rsp);
                cpu.gpr[0] = status as i64 as u64;
                cpu.last_bits[0] = 64;
                cpu.last_bits[1] = 64;
                cpu.last_bits[11] = 64;
            }
            Mnemonic::Ret => {
                let rsp = cpu.gpr[4];
                let ret_addr = cpu.read_mem(rsp, 64);
                let imm = if ins.op_count() > 0 { ins.immediate16() as u64 } else { 0 };
                cpu.gpr[4] = rsp.wrapping_add(8 + imm);
                cpu.rip = ret_addr;
            }
            Mnemonic::Call => {
                let target = match ins.op_kind(0) {
                    OpKind::NearBranch64 | OpKind::NearBranch32 | OpKind::NearBranch16 => ins.near_branch_target(),
                    _ => cpu.read_op(&ins, 0, 64),
                };
                if let Some(r) = apply_fx(&mut cpu, fast::Fx::Call(target), entry_ip, blocks_ptr, sp_ptr, prof) {
                    return r;
                }
            }
            Mnemonic::Jmp => {
                let target = match ins.op_kind(0) {
                    OpKind::NearBranch64 | OpKind::NearBranch32 | OpKind::NearBranch16 => ins.near_branch_target(),
                    _ => cpu.read_op(&ins, 0, 64),
                };
                if let Some(r) = apply_fx(&mut cpu, fast::Fx::Jmp(target), entry_ip, blocks_ptr, sp_ptr, prof) {
                    return r;
                }
            }
            _ => {
                let cc = ins.condition_code();
                if cc != ConditionCode::None
                    && ins.flow_control() == FlowControl::ConditionalBranch
                    && !matches!(mn, Mnemonic::Loop | Mnemonic::Loope | Mnemonic::Loopne | Mnemonic::Jrcxz | Mnemonic::Jecxz)
                {
                    if cpu.eval_cc(cc) {
                        let target = ins.near_branch_target();
                        if ins.len() > 2 {
                            if let Some(r) = apply_fx(&mut cpu, fast::Fx::JccNear(target), entry_ip, blocks_ptr, sp_ptr, prof) {
                                return r;
                            }
                        } else {
                            cpu.rip = target;
                        }
                    }
                } else {
                    let pre_fp = if nan_step {
                        Some(crate::diag::pre_ins(&cpu, &ins))
                    } else {
                        None
                    };
                    if !cpu.exec_generic_ins(&ins) {
                        if let Some(p) = prof {
                            p.unhandled(cur_ip, mn);
                        }
                        report_unimplemented(cur_ip, mn, &ins);
                    } else if let Some(ref pf) = pre_fp {
                        crate::diag::post_ins(pf, &cpu, cur_ip, &ins);
                    }
                }
            }
        }
        if let Some(s) = shadow.take() {
            verify_fast(&s, &cpu, cur_ip, &ins);
        }
    }
    if precise {
        FAULT_PENDING.with(|p| p.set(None));
    }
    if g.steps >= MAX_STEPS {
        log::warn!("emu: hit MAX_STEPS ({}) at rip={:#x} (entry={:#x})", MAX_STEPS, cpu.rip, entry_ip);
    }
    if cpu.rip == 0 && entry_ip != 0 {
        // A guest `jmp`/`ret` to 0 ended the run (natively a fault).
        note_null_call();
    }
    (cpu.abi_gpr(0), cpu.xmm[0])
}

impl Cpu {
    pub(super) unsafe fn exec_generic_ins(&mut self, ins_ref: &Instruction) -> bool {
        let ins = *ins_ref;
        let mn = ins.mnemonic();
        let cpu = self;
        match mn {
            Mnemonic::Mfence | Mnemonic::Sfence | Mnemonic::Lfence => {
                core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
            }
            Mnemonic::Nop | Mnemonic::Pause | Mnemonic::Fnop | Mnemonic::Endbr64 | Mnemonic::Endbr32
            | Mnemonic::Clflush | Mnemonic::Prefetch | Mnemonic::Prefetcht0 | Mnemonic::Prefetcht1
            | Mnemonic::Prefetcht2 | Mnemonic::Prefetchnta | Mnemonic::Prefetchw | Mnemonic::Prefetchwt1
            | Mnemonic::Wait | Mnemonic::Rdsspd | Mnemonic::Rdsspq | Mnemonic::Emms | Mnemonic::Femms => {}
            Mnemonic::Clc => cpu.cf = false,
            Mnemonic::Stc => cpu.cf = true,
            Mnemonic::Cmc => cpu.cf = !cpu.cf,
            Mnemonic::Cld => cpu.df = false,
            Mnemonic::Std => cpu.df = true,
            Mnemonic::Leave => {
                cpu.gpr[4] = cpu.gpr[5];
                let v = cpu.read_mem(cpu.gpr[4], 64);
                cpu.gpr[4] = cpu.gpr[4].wrapping_add(8);
                cpu.gpr[5] = v;
                cpu.last_bits[4] = 64;
                cpu.last_bits[5] = 64;
            }
            Mnemonic::Push => {
                // 64-bit mode: 8 bytes, or 2 with a 66h operand-size prefix (push r16/m16/imm16).
                let bits = if Cpu::op_bits(&ins, 0) == 16 { 16 } else { 64 };
                let v = cpu.read_op(&ins, 0, bits);
                cpu.gpr[4] = cpu.gpr[4].wrapping_sub((bits / 8) as u64);
                cpu.write_mem(cpu.gpr[4], bits, v);
            }
            Mnemonic::Pop => {
                let bits = if Cpu::op_bits(&ins, 0) == 16 { 16 } else { 64 };
                let v = cpu.read_mem(cpu.gpr[4], bits);
                cpu.gpr[4] = cpu.gpr[4].wrapping_add((bits / 8) as u64);
                cpu.write_op(&ins, 0, bits, v);
            }
            Mnemonic::Lea => {
                let a = cpu.eff_addr(&ins);
                let bits = Cpu::op_bits(&ins, 0);
                cpu.write_op(&ins, 0, bits, a);
            }
            Mnemonic::Mov | Mnemonic::Movnti => {
                let bits = Cpu::op_bits(&ins, 0);
                let v = cpu.read_op(&ins, 1, bits);
                cpu.write_op(&ins, 0, bits, v);
            }
            Mnemonic::Movzx => {
                let dst_bits = Cpu::op_bits(&ins, 0);
                let src_bits = Cpu::op_bits(&ins, 1);
                let v = cpu.read_op(&ins, 1, src_bits);
                let mask = if src_bits >= 64 { u64::MAX } else { (1u64 << src_bits) - 1 };
                cpu.write_op(&ins, 0, dst_bits, v & mask);
            }
            Mnemonic::Movsx | Mnemonic::Movsxd => {
                let dst_bits = Cpu::op_bits(&ins, 0);
                let src_bits = Cpu::op_bits(&ins, 1);
                let v = cpu.read_op(&ins, 1, src_bits);
                let sext = match src_bits {
                    8 => v as i8 as i64 as u64,
                    16 => v as i16 as i64 as u64,
                    32 => v as i32 as i64 as u64,
                    _ => v,
                };
                cpu.write_op(&ins, 0, dst_bits, sext);
            }
            Mnemonic::Cbw => {
                cpu.gpr[0] = (cpu.gpr[0] & !0xFFFF) | ((cpu.gpr[0] as u8 as i8 as i16 as u16) as u64);
            }
            Mnemonic::Cwde => {
                cpu.gpr[0] = (cpu.gpr[0] as u16 as i16 as i32 as u32) as u64;
            }
            Mnemonic::Cdqe => {
                cpu.gpr[0] = cpu.gpr[0] as i32 as i64 as u64;
            }
            Mnemonic::Cwd => {
                let hi = if (cpu.gpr[0] as i16) < 0 { 0xFFFFu64 } else { 0 };
                cpu.gpr[2] = (cpu.gpr[2] & !0xFFFF) | hi;
            }
            Mnemonic::Cdq => {
                cpu.gpr[2] = if (cpu.gpr[0] as i32) < 0 { 0xFFFF_FFFF } else { 0 };
            }
            Mnemonic::Cqo => {
                cpu.gpr[2] = if (cpu.gpr[0] as i64) < 0 { u64::MAX } else { 0 };
            }
            Mnemonic::Xor => {
                let bits = Cpu::op_bits(&ins, 0);
                let b = cpu.read_op(&ins, 1, bits);
                if ins.has_lock_prefix() && ins.op_kind(0) == OpKind::Memory {
                    let addr = cpu.eff_addr(&ins);
                    let (_, res) = cpu.atomic_rmw(addr, bits, |a| a ^ b);
                    cpu.update_flags_logic(res, bits);
                } else {
                    let a = cpu.read_op(&ins, 0, bits);
                    let res = a ^ b;
                    cpu.write_op(&ins, 0, bits, res);
                    cpu.update_flags_logic(res, bits);
                }
            }
            Mnemonic::Or => {
                let bits = Cpu::op_bits(&ins, 0);
                let b = cpu.read_op(&ins, 1, bits);
                if ins.has_lock_prefix() && ins.op_kind(0) == OpKind::Memory {
                    let addr = cpu.eff_addr(&ins);
                    let (_, res) = cpu.atomic_rmw(addr, bits, |a| a | b);
                    cpu.update_flags_logic(res, bits);
                } else {
                    let a = cpu.read_op(&ins, 0, bits);
                    let res = a | b;
                    cpu.write_op(&ins, 0, bits, res);
                    cpu.update_flags_logic(res, bits);
                }
            }
            Mnemonic::And => {
                let bits = Cpu::op_bits(&ins, 0);
                let b = cpu.read_op(&ins, 1, bits);
                if ins.has_lock_prefix() && ins.op_kind(0) == OpKind::Memory {
                    let addr = cpu.eff_addr(&ins);
                    let (_, res) = cpu.atomic_rmw(addr, bits, |a| a & b);
                    cpu.update_flags_logic(res, bits);
                } else {
                    let a = cpu.read_op(&ins, 0, bits);
                    let res = a & b;
                    cpu.write_op(&ins, 0, bits, res);
                    cpu.update_flags_logic(res, bits);
                }
            }
            Mnemonic::Test => {
                let bits = Cpu::op_bits(&ins, 0);
                let a = cpu.read_op(&ins, 0, bits);
                let b = cpu.read_op(&ins, 1, bits);
                cpu.update_flags_logic(a & b, bits);
            }
            Mnemonic::Add => {
                let bits = Cpu::op_bits(&ins, 0);
                let b = cpu.read_op(&ins, 1, bits);
                if ins.has_lock_prefix() && ins.op_kind(0) == OpKind::Memory {
                    let addr = cpu.eff_addr(&ins);
                    let (a, _) = cpu.atomic_rmw(addr, bits, |old| old.wrapping_add(b));
                    let _ = cpu.update_flags_add(a, b, bits);
                } else {
                    let a = cpu.read_op(&ins, 0, bits);
                    let res = cpu.update_flags_add(a, b, bits);
                    cpu.write_op(&ins, 0, bits, res);
                }
            }
            Mnemonic::Sub => {
                let bits = Cpu::op_bits(&ins, 0);
                let b = cpu.read_op(&ins, 1, bits);
                if ins.has_lock_prefix() && ins.op_kind(0) == OpKind::Memory {
                    let addr = cpu.eff_addr(&ins);
                    let (a, _) = cpu.atomic_rmw(addr, bits, |old| old.wrapping_sub(b));
                    let _ = cpu.update_flags_sub(a, b, bits);
                } else {
                    let a = cpu.read_op(&ins, 0, bits);
                    let res = cpu.update_flags_sub(a, b, bits);
                    cpu.write_op(&ins, 0, bits, res);
                }
            }
            Mnemonic::Cmp => {
                let bits = Cpu::op_bits(&ins, 0);
                let a = cpu.read_op(&ins, 0, bits);
                let b = cpu.read_op(&ins, 1, bits);
                let _ = cpu.update_flags_sub(a, b, bits);
            }
            Mnemonic::Inc => {
                let bits = Cpu::op_bits(&ins, 0);
                let saved_cf = cpu.cf;
                if ins.has_lock_prefix() && ins.op_kind(0) == OpKind::Memory {
                    let addr = cpu.eff_addr(&ins);
                    let (a, _) = cpu.atomic_rmw(addr, bits, |old| old.wrapping_add(1));
                    let _ = cpu.update_flags_add(a, 1, bits);
                } else {
                    let a = cpu.read_op(&ins, 0, bits);
                    let res = cpu.update_flags_add(a, 1, bits);
                    cpu.write_op(&ins, 0, bits, res);
                }
                cpu.cf = saved_cf;
            }
            Mnemonic::Dec => {
                let bits = Cpu::op_bits(&ins, 0);
                let saved_cf = cpu.cf;
                if ins.has_lock_prefix() && ins.op_kind(0) == OpKind::Memory {
                    let addr = cpu.eff_addr(&ins);
                    let (a, _) = cpu.atomic_rmw(addr, bits, |old| old.wrapping_sub(1));
                    let _ = cpu.update_flags_sub(a, 1, bits);
                } else {
                    let a = cpu.read_op(&ins, 0, bits);
                    let res = cpu.update_flags_sub(a, 1, bits);
                    cpu.write_op(&ins, 0, bits, res);
                }
                cpu.cf = saved_cf;
            }
            Mnemonic::Not => {
                let bits = Cpu::op_bits(&ins, 0);
                if ins.has_lock_prefix() && ins.op_kind(0) == OpKind::Memory {
                    let addr = cpu.eff_addr(&ins);
                    let _ = cpu.atomic_rmw(addr, bits, |old| !old);
                } else {
                    let a = cpu.read_op(&ins, 0, bits);
                    cpu.write_op(&ins, 0, bits, !a);
                }
            }
            Mnemonic::Neg => {
                let bits = Cpu::op_bits(&ins, 0);
                if ins.has_lock_prefix() && ins.op_kind(0) == OpKind::Memory {
                    let addr = cpu.eff_addr(&ins);
                    let (a, _) = cpu.atomic_rmw(addr, bits, |old| 0u64.wrapping_sub(old));
                    let _ = cpu.update_flags_sub(0, a, bits);
                    cpu.cf = a != 0;
                } else {
                    let a = cpu.read_op(&ins, 0, bits);
                    let res = cpu.update_flags_sub(0, a, bits);
                    cpu.cf = a != 0;
                    cpu.write_op(&ins, 0, bits, res);
                }
            }
            Mnemonic::Shl | Mnemonic::Sal | Mnemonic::Shr | Mnemonic::Sar => {
                let bits = Cpu::op_bits(&ins, 0);
                let a = cpu.read_op(&ins, 0, bits);
                let mask = if bits == 64 { 63 } else { 31 };
                let cnt = (cpu.read_op(&ins, 1, 8) as u32) & mask;
                if cnt > 0 {
                    let sub = match mn {
                        Mnemonic::Shr => fast::SHIFT_SHR,
                        Mnemonic::Sar => fast::SHIFT_SAR,
                        _ => fast::SHIFT_SHL,
                    };
                    let res = cpu.shift(sub, a, cnt, bits as u8);
                    cpu.write_op(&ins, 0, bits, res);
                } else if bits == 32 && ins.op_kind(0) == OpKind::Register {
                    // Count 0: flags unchanged, but a 32-bit register destination is still
                    // written (upper half zeroed), as on hardware.
                    cpu.write_op(&ins, 0, 32, a);
                }
            }
            Mnemonic::Rol => {
                let bits = Cpu::op_bits(&ins, 0);
                let a = cpu.read_op(&ins, 0, bits);
                let mask = if bits == 64 { 63 } else { 31 };
                let cnt = (cpu.read_op(&ins, 1, 8) as u32) & mask;
                if cnt > 0 {
                    let res = match bits {
                        8 => (a as u8).rotate_left(cnt) as u64,
                        16 => (a as u16).rotate_left(cnt) as u64,
                        32 => (a as u32).rotate_left(cnt) as u64,
                        _ => a.rotate_left(cnt),
                    };
                    cpu.write_op(&ins, 0, bits, res);
                    cpu.cf = (res & 1) != 0;
                    cpu.of = ((res >> (bits - 1)) & 1 != 0) ^ cpu.cf;
                } else if bits == 32 && ins.op_kind(0) == OpKind::Register {
                    cpu.write_op(&ins, 0, 32, a); // count 0 still zero-extends a 32-bit register
                }
            }
            Mnemonic::Ror => {
                let bits = Cpu::op_bits(&ins, 0);
                let a = cpu.read_op(&ins, 0, bits);
                let mask = if bits == 64 { 63 } else { 31 };
                let cnt = (cpu.read_op(&ins, 1, 8) as u32) & mask;
                if cnt > 0 {
                    let res = match bits {
                        8 => (a as u8).rotate_right(cnt) as u64,
                        16 => (a as u16).rotate_right(cnt) as u64,
                        32 => (a as u32).rotate_right(cnt) as u64,
                        _ => a.rotate_right(cnt),
                    };
                    cpu.write_op(&ins, 0, bits, res);
                    cpu.cf = ((res >> (bits - 1)) & 1) != 0;
                    cpu.of = cpu.cf ^ ((res >> (bits - 2)) & 1 != 0);
                } else if bits == 32 && ins.op_kind(0) == OpKind::Register {
                    cpu.write_op(&ins, 0, 32, a); // count 0 still zero-extends a 32-bit register
                }
            }
            Mnemonic::Bsr => {
                let bits = Cpu::op_bits(&ins, 0);
                let src = cpu.read_op(&ins, 1, bits);
                let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
                let v = src & mask;
                if v == 0 {
                    cpu.zf = true;
                } else {
                    cpu.zf = false;
                    let idx = 63u64.wrapping_sub(v.leading_zeros() as u64);
                    cpu.write_op(&ins, 0, bits, idx);
                }
            }
            Mnemonic::Bsf | Mnemonic::Tzcnt => {
                let bits = Cpu::op_bits(&ins, 0);
                let src = cpu.read_op(&ins, 1, bits);
                let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
                let v = src & mask;
                let is_tzcnt = ins.mnemonic() == Mnemonic::Tzcnt;
                if v == 0 {
                    // BSF: ZF=1, dest unchanged. TZCNT: CF=1, dest = operand size, ZF = (dest == 0) = 0.
                    cpu.zf = !is_tzcnt;
                    cpu.cf = true;
                    if is_tzcnt {
                        cpu.write_op(&ins, 0, bits, bits as u64);
                    }
                } else {
                    let idx = v.trailing_zeros() as u64;
                    cpu.zf = is_tzcnt && idx == 0;
                    cpu.cf = false;
                    cpu.write_op(&ins, 0, bits, idx);
                }
            }
            Mnemonic::Lzcnt => {
                let bits = Cpu::op_bits(&ins, 0);
                let src = cpu.read_op(&ins, 1, bits);
                let res = match bits {
                    16 => (src as u16).leading_zeros() as u64,
                    32 => (src as u32).leading_zeros() as u64,
                    _ => src.leading_zeros() as u64,
                };
                cpu.write_op(&ins, 0, bits, res);
                cpu.zf = res == 0;
                cpu.cf = res == bits as u64;
            }
            Mnemonic::Popcnt => {
                let bits = Cpu::op_bits(&ins, 0);
                let src = cpu.read_op(&ins, 1, bits);
                let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
                let res = (src & mask).count_ones() as u64;
                cpu.write_op(&ins, 0, bits, res);
                cpu.zf = res == 0;
                cpu.cf = false;
                cpu.sf = false;
                cpu.of = false;
                cpu.set_pf(false);
            }
            Mnemonic::Bswap => {
                let bits = Cpu::op_bits(&ins, 0);
                let src = cpu.read_op(&ins, 0, bits);
                let res = match bits {
                    32 => (src as u32).swap_bytes() as u64,
                    _ => src.swap_bytes(),
                };
                cpu.write_op(&ins, 0, bits, res);
            }
            Mnemonic::Bt | Mnemonic::Bts | Mnemonic::Btr | Mnemonic::Btc => {
                let bits = Cpu::op_bits(&ins, 0);
                if ins.op_kind(0) == OpKind::Memory {
                    let base_addr = cpu.eff_addr(&ins);
                    let (addr, b) = if ins.op_kind(1) == OpKind::Register {
                        let bit_off = match bits {
                            16 => cpu.read_op(&ins, 1, 16) as i16 as i64,
                            32 => cpu.read_op(&ins, 1, 32) as i32 as i64,
                            _ => cpu.read_op(&ins, 1, 64) as i64,
                        };
                        let elem_bytes = (bits / 8) as i64;
                        let elem_idx = bit_off.div_euclid(bits as i64);
                        let rem = bit_off.rem_euclid(bits as i64) as u32;
                        ((base_addr as i64).wrapping_add(elem_idx.wrapping_mul(elem_bytes)) as u64, rem)
                    } else {
                        (base_addr, (cpu.read_op(&ins, 1, bits) as u32) & (bits - 1))
                    };
                    match ins.mnemonic() {
                        Mnemonic::Bts => {
                            let (a, _) = cpu.atomic_rmw(addr, bits, |old| old | (1u64 << b));
                            cpu.cf = ((a >> b) & 1) != 0;
                        }
                        Mnemonic::Btr => {
                            let (a, _) = cpu.atomic_rmw(addr, bits, |old| old & !(1u64 << b));
                            cpu.cf = ((a >> b) & 1) != 0;
                        }
                        Mnemonic::Btc => {
                            let (a, _) = cpu.atomic_rmw(addr, bits, |old| old ^ (1u64 << b));
                            cpu.cf = ((a >> b) & 1) != 0;
                        }
                        _ => {
                            let a = cpu.read_mem(addr, bits);
                            cpu.cf = ((a >> b) & 1) != 0;
                        }
                    }
                } else {
                    let a = cpu.read_op(&ins, 0, bits);
                    let b = (cpu.read_op(&ins, 1, bits) as u32) & (bits - 1);
                    cpu.cf = ((a >> b) & 1) != 0;
                    match ins.mnemonic() {
                        Mnemonic::Bts => cpu.write_op(&ins, 0, bits, a | (1u64 << b)),
                        Mnemonic::Btr => cpu.write_op(&ins, 0, bits, a & !(1u64 << b)),
                        Mnemonic::Btc => cpu.write_op(&ins, 0, bits, a ^ (1u64 << b)),
                        _ => {}
                    }
                }
            }
            Mnemonic::Adc => {
                let bits = Cpu::op_bits(&ins, 0);
                let a = cpu.read_op(&ins, 0, bits);
                let b = cpu.read_op(&ins, 1, bits);
                let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
                let (ua, ub) = (a & mask, b & mask);
                let full = (ua as u128) + (ub as u128) + (cpu.cf as u128);
                let res = (full as u64) & mask;
                let sign_bit = 1u64 << (bits - 1);
                cpu.cf = full > mask as u128;
                cpu.of = ((ua ^ res) & (ub ^ res) & sign_bit) != 0;
                cpu.zf = res == 0;
                cpu.sf = (res & sign_bit) != 0;
                cpu.pfv = res as u8;
                cpu.write_op(&ins, 0, bits, res);
            }
            Mnemonic::Sbb => {
                let bits = Cpu::op_bits(&ins, 0);
                let a = cpu.read_op(&ins, 0, bits);
                let b = cpu.read_op(&ins, 1, bits);
                let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
                let (ua, ub) = (a & mask, b & mask);
                let c = cpu.cf as u64;
                let res = ua.wrapping_sub(ub).wrapping_sub(c) & mask;
                let sign_bit = 1u64 << (bits - 1);
                cpu.cf = (ua as u128) < (ub as u128) + (c as u128);
                cpu.of = ((ua ^ ub) & (ua ^ res) & sign_bit) != 0;
                cpu.zf = res == 0;
                cpu.sf = (res & sign_bit) != 0;
                cpu.pfv = res as u8;
                cpu.write_op(&ins, 0, bits, res);
            }
            Mnemonic::Mul => {
                let bits = Cpu::op_bits(&ins, 0);
                let src = cpu.read_op(&ins, 0, bits);
                if bits == 64 {
                    let prod = (cpu.gpr[0] as u128).wrapping_mul(src as u128);
                    cpu.gpr[0] = prod as u64;
                    cpu.gpr[2] = (prod >> 64) as u64;
                    cpu.cf = cpu.gpr[2] != 0;
                    cpu.of = cpu.cf;
                } else if bits == 32 {
                    let prod = (cpu.gpr[0] as u32 as u64).wrapping_mul(src as u32 as u64);
                    cpu.gpr[0] = prod as u32 as u64;
                    cpu.gpr[2] = (prod >> 32) as u32 as u64;
                    cpu.cf = cpu.gpr[2] != 0;
                    cpu.of = cpu.cf;
                } else if bits == 16 {
                    let prod = (cpu.gpr[0] as u16 as u32) * (src as u16 as u32);
                    cpu.set_reg(Register::AX, prod as u64);
                    cpu.set_reg(Register::DX, (prod >> 16) as u64);
                    cpu.cf = (prod >> 16) != 0;
                    cpu.of = cpu.cf;
                } else {
                    let prod = (cpu.gpr[0] as u8 as u16) * (src as u8 as u16);
                    cpu.set_reg(Register::AX, prod as u64);
                    cpu.cf = (prod >> 8) != 0;
                    cpu.of = cpu.cf;
                }
            }
            Mnemonic::Div => {
                let bits = Cpu::op_bits(&ins, 0);
                let d = cpu.read_op(&ins, 0, bits);
                if bits == 64 && d != 0 {
                    let n = ((cpu.gpr[2] as u128) << 64) | (cpu.gpr[0] as u128);
                    cpu.gpr[0] = (n / (d as u128)) as u64;
                    cpu.gpr[2] = (n % (d as u128)) as u64;
                } else if bits == 32 && (d as u32) != 0 {
                    let n = ((cpu.gpr[2] as u32 as u64) << 32) | (cpu.gpr[0] as u32 as u64);
                    let d32 = d as u32 as u64;
                    cpu.gpr[0] = (n / d32) as u32 as u64;
                    cpu.gpr[2] = (n % d32) as u32 as u64;
                } else if bits == 16 && (d as u16) != 0 {
                    let n = ((cpu.gpr[2] as u16 as u32) << 16) | (cpu.gpr[0] as u16 as u32);
                    let d16 = d as u16 as u32;
                    cpu.set_reg(Register::AX, (n / d16) as u64);
                    cpu.set_reg(Register::DX, (n % d16) as u64);
                } else if bits == 8 && (d as u8) != 0 {
                    let n = cpu.gpr[0] as u16;
                    let d8 = d as u8 as u16;
                    cpu.set_reg(Register::AL, (n / d8) as u64);
                    cpu.set_reg(Register::AH, (n % d8) as u64);
                }
            }
            Mnemonic::Idiv => {
                let bits = Cpu::op_bits(&ins, 0);
                let d = cpu.read_op(&ins, 0, bits);
                if bits == 64 && d != 0 {
                    let n = (((cpu.gpr[2] as u128) << 64) | (cpu.gpr[0] as u128)) as i128;
                    let d64 = d as i64 as i128;
                    cpu.gpr[0] = n.wrapping_div(d64) as i64 as u64;
                    cpu.gpr[2] = n.wrapping_rem(d64) as i64 as u64;
                } else if bits == 32 && (d as u32) != 0 {
                    let n = (((cpu.gpr[2] as u32 as u64) << 32) | (cpu.gpr[0] as u32 as u64)) as i64;
                    let d32 = d as u32 as i32 as i64;
                    cpu.gpr[0] = n.wrapping_div(d32) as i32 as u32 as u64;
                    cpu.gpr[2] = n.wrapping_rem(d32) as i32 as u32 as u64;
                } else if bits == 16 && (d as u16) != 0 {
                    let n = (((cpu.gpr[2] as u16 as u32) << 16) | (cpu.gpr[0] as u16 as u32)) as i32;
                    let d16 = d as u16 as i16 as i32;
                    cpu.set_reg(Register::AX, n.wrapping_div(d16) as u64);
                    cpu.set_reg(Register::DX, n.wrapping_rem(d16) as u64);
                } else if bits == 8 && (d as u8) != 0 {
                    let n = cpu.gpr[0] as u16 as i16;
                    let d8 = d as u8 as i8 as i16;
                    cpu.set_reg(Register::AL, n.wrapping_div(d8) as u64);
                    cpu.set_reg(Register::AH, n.wrapping_rem(d8) as u64);
                }
            }
            Mnemonic::Imul => {
                let bits = Cpu::op_bits(&ins, 0);
                let sext = |v: u64, b: u32| -> i128 {
                    match b {
                        8 => v as u8 as i8 as i128,
                        16 => v as u16 as i16 as i128,
                        32 => v as u32 as i32 as i128,
                        _ => v as i64 as i128,
                    }
                };
                match ins.op_count() {
                    1 => {
                        let src = cpu.read_op(&ins, 0, bits);
                        if bits == 64 {
                            let prod = (cpu.gpr[0] as i64 as i128).wrapping_mul(src as i64 as i128);
                            cpu.gpr[0] = prod as i64 as u64;
                            cpu.gpr[2] = (prod >> 64) as i64 as u64;
                            cpu.cf = prod != (prod as i64 as i128);
                            cpu.of = cpu.cf;
                        } else if bits == 32 {
                            let prod = (cpu.gpr[0] as u32 as i32 as i64).wrapping_mul(src as u32 as i32 as i64);
                            cpu.gpr[0] = prod as i32 as u32 as u64;
                            cpu.gpr[2] = ((prod >> 32) as i32 as u32) as u64;
                            cpu.cf = prod != (prod as i32 as i64);
                            cpu.of = cpu.cf;
                        } else if bits == 16 {
                            let prod = (cpu.gpr[0] as u16 as i16 as i32) * (src as u16 as i16 as i32);
                            cpu.set_reg(Register::AX, prod as u64);
                            cpu.set_reg(Register::DX, (prod >> 16) as u64);
                            cpu.cf = prod != (prod as i16 as i32);
                            cpu.of = cpu.cf;
                        } else {
                            let prod = (cpu.gpr[0] as u8 as i8 as i16) * (src as u8 as i8 as i16);
                            cpu.set_reg(Register::AX, prod as u64);
                            cpu.cf = prod != (prod as i8 as i16);
                            cpu.of = cpu.cf;
                        }
                    }
                    2 => {
                        let a = sext(cpu.read_op(&ins, 0, bits), bits);
                        let b = sext(cpu.read_op(&ins, 1, bits), bits);
                        let prod = a.wrapping_mul(b);
                        let res = prod as u64;
                        cpu.write_op(&ins, 0, bits, res);
                        let fits = match bits {
                            16 => prod == (prod as i16 as i128),
                            32 => prod == (prod as i32 as i128),
                            _ => prod == (prod as i64 as i128),
                        };
                        cpu.cf = !fits;
                        cpu.of = !fits;
                    }
                    3 => {
                        let a = sext(cpu.read_op(&ins, 1, bits), bits);
                        let b = sext(cpu.read_op(&ins, 2, bits), bits);
                        let prod = a.wrapping_mul(b);
                        let res = prod as u64;
                        cpu.write_op(&ins, 0, bits, res);
                        let fits = match bits {
                            16 => prod == (prod as i16 as i128),
                            32 => prod == (prod as i32 as i128),
                            _ => prod == (prod as i64 as i128),
                        };
                        cpu.cf = !fits;
                        cpu.of = !fits;
                    }
                    _ => {}
                }
            }
            Mnemonic::Xchg => {
                let bits = Cpu::op_bits(&ins, 0);
                if ins.op_kind(0) == OpKind::Memory {
                    let b = cpu.read_op(&ins, 1, bits);
                    let addr = cpu.eff_addr(&ins);
                    let (a, _) = cpu.atomic_rmw(addr, bits, |_| b);
                    cpu.write_op(&ins, 1, bits, a);
                } else if ins.op_kind(1) == OpKind::Memory {
                    let a = cpu.read_op(&ins, 0, bits);
                    let addr = cpu.eff_addr(&ins);
                    let (b, _) = cpu.atomic_rmw(addr, bits, |_| a);
                    cpu.write_op(&ins, 0, bits, b);
                } else {
                    let a = cpu.read_op(&ins, 0, bits);
                    let b = cpu.read_op(&ins, 1, bits);
                    cpu.write_op(&ins, 0, bits, b);
                    cpu.write_op(&ins, 1, bits, a);
                }
            }
            Mnemonic::Cmpxchg => {
                let bits = Cpu::op_bits(&ins, 0);
                let src = cpu.read_op(&ins, 1, bits);
                let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
                let acc = cpu.gpr[0] & mask;
                if ins.op_kind(0) == OpKind::Memory {
                    let addr = cpu.eff_addr(&ins);
                    let (ok, dst) = cpu.atomic_cmpxchg(addr, bits, acc, src & mask);
                    let _ = cpu.update_flags_sub(acc, dst, bits);
                    if ok {
                        cpu.zf = true;
                    } else {
                        cpu.zf = false;
                        let acc_reg = match bits {
                            8 => Register::AL,
                            16 => Register::AX,
                            32 => Register::EAX,
                            _ => Register::RAX,
                        };
                        cpu.set_reg(acc_reg, dst);
                    }
                } else {
                    let dst = cpu.read_op(&ins, 0, bits);
                    let _ = cpu.update_flags_sub(acc, dst, bits);
                    if (acc & mask) == (dst & mask) {
                        cpu.zf = true;
                        cpu.write_op(&ins, 0, bits, src);
                    } else {
                        cpu.zf = false;
                        let acc_reg = match bits {
                            8 => Register::AL,
                            16 => Register::AX,
                            32 => Register::EAX,
                            _ => Register::RAX,
                        };
                        cpu.set_reg(acc_reg, dst);
                    }
                }
            }
            Mnemonic::Cmpxchg8b | Mnemonic::Cmpxchg16b => {
                let addr = cpu.eff_addr(&ins);
                if mn == Mnemonic::Cmpxchg8b {
                    let expected = ((cpu.gpr[2] as u32 as u64) << 32) | (cpu.gpr[0] as u32 as u64);
                    let desired = ((cpu.gpr[1] as u32 as u64) << 32) | (cpu.gpr[3] as u32 as u64);
                    let (ok, actual) = cpu.atomic_cmpxchg(addr, 64, expected, desired);
                    cpu.zf = ok;
                    if !ok {
                        cpu.set_reg(Register::EAX, actual as u32 as u64);
                        cpu.set_reg(Register::EDX, (actual >> 32) as u32 as u64);
                    }
                } else {
                    let expected = ((cpu.gpr[2] as u128) << 64) | (cpu.gpr[0] as u128);
                    let desired = ((cpu.gpr[1] as u128) << 64) | (cpu.gpr[3] as u128);
                    let fast = if addr & 15 == 0 && (0x1_0000..0x8000_0000_0000).contains(&addr) {
                        cas128_full(addr, expected, desired)
                    } else {
                        None
                    };
                    let actual = if let Some(a) = fast {
                        a
                    } else {
                        static CMPXCHG16B_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
                        let _g = CMPXCHG16B_LOCK.lock().unwrap();
                        let actual = cpu.read_u128(addr);
                        if actual == expected {
                            cpu.write_u128(addr, desired);
                        }
                        actual
                    };
                    if actual == expected {
                        cpu.zf = true;
                    } else {
                        cpu.zf = false;
                        cpu.set_reg(Register::RAX, actual as u64);
                        cpu.set_reg(Register::RDX, (actual >> 64) as u64);
                    }
                }
            }
            Mnemonic::Xadd => {
                let bits = Cpu::op_bits(&ins, 0);
                let b = cpu.read_op(&ins, 1, bits);
                if ins.op_kind(0) == OpKind::Memory {
                    let addr = cpu.eff_addr(&ins);
                    let (a, _) = cpu.atomic_rmw(addr, bits, |old| old.wrapping_add(b));
                    let _ = cpu.update_flags_add(a, b, bits);
                    cpu.write_op(&ins, 1, bits, a);
                } else {
                    let a = cpu.read_op(&ins, 0, bits);
                    let res = cpu.update_flags_add(a, b, bits);
                    cpu.write_op(&ins, 0, bits, res);
                    cpu.write_op(&ins, 1, bits, a);
                }
            }
            Mnemonic::Cpuid => {
                let leaf = cpu.gpr[0] as u32;
                cpu.last_bits[0] = 64;
                cpu.last_bits[1] = 64;
                cpu.last_bits[2] = 64;
                cpu.last_bits[3] = 64;
                match leaf {
                    0 => {
                        cpu.gpr[0] = 7;
                        cpu.gpr[3] = 0x756e_6547; // "Genu"
                        cpu.gpr[2] = 0x4965_6e69; // "ineI"
                        cpu.gpr[1] = 0x6c65_746e; // "ntel"
                    }
                    1 => {
                        cpu.gpr[0] = 0x0009_06ea;
                        cpu.gpr[3] = 0x0010_0800;
                        // SSE3(0), PCLMULQDQ(1), VMX(5), SSSE3(9), FMA(12), CMPXCHG16B(13),
                        // SSE4.1(19), SSE4.2(20), MOVBE(22), POPCNT(23), AES(25), XSAVE(26),
                        // OSXSAVE(27), AVX(28), F16C(29), RDRAND(30)
                        cpu.gpr[1] = (1 << 0)
                            | (1 << 1)
                            | (1 << 5)
                            | (1 << 9)
                            | (1 << 12)
                            | (1 << 13)
                            | (1 << 19)
                            | (1 << 20)
                            | (1 << 22)
                            | (1 << 23)
                            | (1 << 25)
                            | (1 << 26)
                            | (1 << 27)
                            | (1 << 28)
                            | (1 << 29)
                            | (1 << 30);
                        // FPU(0), TSC(4), CX8(8), CMOV(15), MMX(23), FXSR(24), SSE(25), SSE2(26), HTT(28)
                        cpu.gpr[2] = (1 << 0)
                            | (1 << 4)
                            | (1 << 8)
                            | (1 << 15)
                            | (1 << 23)
                            | (1 << 24)
                            | (1 << 25)
                            | (1 << 26)
                            | (1 << 28);
                    }
                    7 => {
                        cpu.gpr[0] = 0;
                        // FSGSBASE(0), BMI1(3), AVX2(5), SMEP(7), BMI2(8), ERMS(9), INVPCID(10),
                        // RDSEED(18), ADX(19), SHA-NI(29).
                        cpu.gpr[3] = (1 << 0)
                            | (1 << 3)
                            | (1 << 5)
                            | (1 << 7)
                            | (1 << 8)
                            | (1 << 9)
                            | (1 << 10)
                            | (1 << 18)
                            | (1 << 19)
                            | (1 << 29);
                        cpu.gpr[1] = 0;
                        cpu.gpr[2] = 0;
                    }
                    0x8000_0000 => {
                        cpu.gpr[0] = 0x8000_0008;
                        cpu.gpr[3] = 0;
                        cpu.gpr[1] = 0;
                        cpu.gpr[2] = 0;
                    }
                    0x8000_0001 => {
                        cpu.gpr[0] = 0;
                        cpu.gpr[3] = 0;
                        // LAHF_LM(0), LZCNT/ABM(5), SSE4A(6), PREFETCHW(8)
                        cpu.gpr[1] = (1 << 0) | (1 << 5) | (1 << 6) | (1 << 8);
                        // SYSCALL(11), NX(20), RDTSCP(27), LM(29)
                        cpu.gpr[2] = (1 << 11) | (1 << 20) | (1 << 27) | (1 << 29);
                    }
                    0x8000_0002..=0x8000_0004 => {
                        let brand = b"Apple Silicon M-Series (AW2 Native ARM64)       ";
                        let off = ((leaf - 0x8000_0002) as usize) * 16;
                        cpu.gpr[0] = u32::from_le_bytes(brand[off..off + 4].try_into().unwrap()) as u64;
                        cpu.gpr[3] = u32::from_le_bytes(brand[off + 4..off + 8].try_into().unwrap()) as u64;
                        cpu.gpr[1] = u32::from_le_bytes(brand[off + 8..off + 12].try_into().unwrap()) as u64;
                        cpu.gpr[2] = u32::from_le_bytes(brand[off + 12..off + 16].try_into().unwrap()) as u64;
                    }
                    _ => {
                        cpu.gpr[0] = 0;
                        cpu.gpr[3] = 0;
                        cpu.gpr[1] = 0;
                        cpu.gpr[2] = 0;
                    }
                }
            }
            Mnemonic::Xgetbv => {
                cpu.gpr[0] = 7;
                cpu.gpr[2] = 0;
                cpu.last_bits[0] = 64;
                cpu.last_bits[2] = 64;
            }
            Mnemonic::Rdtsc | Mnemonic::Rdtscp => {
                cpu.gpr[0] = GUEST_TSC & 0xffff_ffff;
                cpu.gpr[2] = GUEST_TSC >> 32;
                cpu.last_bits[0] = 64;
                cpu.last_bits[2] = 64;
                if mn == Mnemonic::Rdtscp {
                    cpu.gpr[1] = 0; // IA32_TSC_AUX
                    cpu.last_bits[1] = 64;
                }
            }
            // Unprivileged x86-64 system-table / machine-status store instructions
            // (used by OS runtimes and integrity probes; legal in ring 3 when UMIP=0).
            Mnemonic::Sidt => {
                let a = cpu.eff_addr(&ins);
                // Standard 64-bit Windows IDT descriptor: 256 16-byte gates (limit 0x0fff)
                // at a canonical kernel-space base address.
                cpu.write_mem(a, 16, 0x0fff);
                cpu.write_mem(a.wrapping_add(2), 64, 0xffff_f800_0000_0000);
            }
            Mnemonic::Sgdt => {
                let a = cpu.eff_addr(&ins);
                cpu.write_mem(a, 16, 0x0057);
                cpu.write_mem(a.wrapping_add(2), 64, 0xffff_f800_0000_1000);
            }
            Mnemonic::Sldt => {
                // Windows x64 does not use an LDT; memory operand is always 16 bits.
                cpu.write_op(&ins, 0, 16, 0);
            }
            Mnemonic::Str => {
                // Windows x64 task register selector (KGDT64_SYS_TSS = 0x40).
                cpu.write_op(&ins, 0, 16, 0x0040);
            }
            Mnemonic::Smsw => {
                // Standard protected+paging+NE+WP CR0 machine status word.
                // Memory destination stores low 16 bits; register destination stores
                // 16/32/64 bits according to operand width.
                cpu.write_op(&ins, 0, 16, 0x8005_0033);
            }
            Mnemonic::Stosb | Mnemonic::Stosw | Mnemonic::Stosd | Mnemonic::Stosq => {
                let elem_bytes: u64 = match mn {
                    Mnemonic::Stosb => 1,
                    Mnemonic::Stosw => 2,
                    Mnemonic::Stosd => 4,
                    _ => 8,
                };
                let count = if ins.has_rep_prefix() { cpu.gpr[1] } else { 1 };
                let val = cpu.gpr[0];
                if cpu.df {
                    for _ in 0..count {
                        cpu.write_mem(cpu.gpr[7], (elem_bytes * 8) as u32, val);
                        cpu.gpr[7] = cpu.gpr[7].wrapping_sub(elem_bytes);
                    }
                } else if elem_bytes == 1 && count > 0 {
                    if mem::tso() {
                        // Plain bulk stores: order them after all earlier guest accesses.
                        core::sync::atomic::fence(core::sync::atomic::Ordering::Release);
                    }
                    core::ptr::write_bytes(cpu.gpr[7] as *mut u8, val as u8, count as usize);
                    cpu.gpr[7] = cpu.gpr[7].wrapping_add(count);
                } else {
                    for _ in 0..count {
                        cpu.write_mem(cpu.gpr[7], (elem_bytes * 8) as u32, val);
                        cpu.gpr[7] = cpu.gpr[7].wrapping_add(elem_bytes);
                    }
                }
                cpu.last_bits[7] = 64;
                if ins.has_rep_prefix() {
                    cpu.gpr[1] = 0;
                    cpu.last_bits[1] = 64;
                }
            }
            Mnemonic::Movsb | Mnemonic::Movsw | Mnemonic::Movsd | Mnemonic::Movsq => {
                if mn == Mnemonic::Movsd && (ins.op_kind(0) == OpKind::Register || ins.op_kind(1) == OpKind::Register) {
                    // SSE movsd
                    match (ins.op_kind(0), ins.op_kind(1)) {
                        (OpKind::Register, OpKind::Register) => {
                            if let (Some(d), Some(s)) = (Cpu::xmm_idx(ins.op_register(0)), Cpu::xmm_idx(ins.op_register(1))) {
                                cpu.xmm[d] = (cpu.xmm[d] & !0xFFFF_FFFF_FFFF_FFFF) | (cpu.xmm[s] & 0xFFFF_FFFF_FFFF_FFFF);
                            }
                        }
                        (OpKind::Register, OpKind::Memory) => {
                            if let Some(d) = Cpu::xmm_idx(ins.op_register(0)) {
                                let a = cpu.eff_addr(&ins);
                                cpu.xmm[d] = cpu.read_mem(a, 64) as u128;
                            }
                        }
                        (OpKind::Memory, OpKind::Register) => {
                            if let Some(s) = Cpu::xmm_idx(ins.op_register(1)) {
                                let a = cpu.eff_addr(&ins);
                                cpu.write_mem(a, 64, cpu.xmm[s] as u64);
                            }
                        }
                        _ => {}
                    }
                } else {
                    let elem_bytes: u64 = match mn {
                        Mnemonic::Movsb => 1,
                        Mnemonic::Movsw => 2,
                        Mnemonic::Movsd => 4,
                        _ => 8,
                    };
                    let count = if ins.has_rep_prefix() { cpu.gpr[1] } else { 1 };
                    let total_bytes = count.wrapping_mul(elem_bytes);
                    if cpu.df {
                        let bits = (elem_bytes * 8) as u32;
                        for _ in 0..count {
                            let v = cpu.read_mem(cpu.gpr[6], bits);
                            cpu.write_mem(cpu.gpr[7], bits, v);
                            cpu.gpr[6] = cpu.gpr[6].wrapping_sub(elem_bytes);
                            cpu.gpr[7] = cpu.gpr[7].wrapping_sub(elem_bytes);
                        }
                    } else if total_bytes > 0 {
                        let (src, dst) = (cpu.gpr[6], cpu.gpr[7]);
                        let dist = dst.wrapping_sub(src);
                        if mem::tso() {
                            // Plain bulk copy: order it after all earlier guest accesses.
                            core::sync::atomic::fence(core::sync::atomic::Ordering::Release);
                        }
                        if dist == 0 || dist >= total_bytes {
                            // No destructive overlap (dst before src, or disjoint): x86's
                            // forward element-by-element copy equals memmove.
                            core::ptr::copy(src as *const u8, dst as *mut u8, total_bytes as usize);
                        } else if dist >= elem_bytes {
                            // dst inside (src, src+len): the first `dist` bytes are replicated
                            // (std::fill idiom). Copy in non-overlapping chunks of `dist` bytes.
                            let mut off = 0u64;
                            while off < total_bytes {
                                let n = dist.min(total_bytes - off);
                                core::ptr::copy_nonoverlapping((src + off) as *const u8, (dst + off) as *mut u8, n as usize);
                                off += n;
                            }
                        } else {
                            let bits = (elem_bytes * 8) as u32;
                            for i in 0..count {
                                let v = cpu.read_mem(src.wrapping_add(i * elem_bytes), bits);
                                cpu.write_mem(dst.wrapping_add(i * elem_bytes), bits, v);
                            }
                        }
                        if mem::tso() {
                            // ...and keep its plain loads before later guest loads.
                            core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
                        }
                        cpu.gpr[6] = src.wrapping_add(total_bytes);
                        cpu.gpr[7] = dst.wrapping_add(total_bytes);
                    }
                    cpu.last_bits[6] = 64;
                    cpu.last_bits[7] = 64;
                    if ins.has_rep_prefix() {
                        cpu.gpr[1] = 0;
                        cpu.last_bits[1] = 64;
                    }
                }
            }
            Mnemonic::Xorps | Mnemonic::Xorpd | Mnemonic::Pxor | Mnemonic::Vxorps | Mnemonic::Vxorpd | Mnemonic::Vpxor => {
                if let Some(d) = Cpu::xmm_idx(ins.op_register(0)) {
                    // Legacy SSE: dst ^= src (2 operands). VEX: dst = src1 ^ src2 (3 operands).
                    let (s1_op, s2_op) = if ins.op_count() >= 3 { (1, 2) } else { (0, 1) };
                    let ymm = Cpu::is_ymm(ins.op_register(0));
                    let (a_lo, a_hi) = cpu.vsrc(&ins, s1_op, ymm);
                    let (b_lo, b_hi) = cpu.vsrc(&ins, s2_op, ymm);
                    cpu.vdst(&ins, d, a_lo ^ b_lo, a_hi ^ b_hi);
                }
            }
            _ => {
                if !cpu.exec_simd_mov_int(mn, &ins) && !cpu.exec_simd_fp_ext(mn, &ins) {
                    let cc = ins.condition_code();
                    if cc != ConditionCode::None && ins.flow_control() != FlowControl::ConditionalBranch {
                        if ins.op_count() == 1 {
                            let v = cpu.eval_cc(cc) as u64;
                            cpu.write_op(&ins, 0, 8, v);
                        } else if ins.op_count() == 2 {
                            let bits = Cpu::op_bits(&ins, 0);
                            if cpu.eval_cc(cc) {
                                let v = cpu.read_op(&ins, 1, bits);
                                cpu.write_op(&ins, 0, bits, v);
                            } else if bits == 32 {
                                let cur = cpu.read_op(&ins, 0, 32);
                                cpu.write_op(&ins, 0, 32, cur);
                            }
                        }
                    } else {
                        return false;
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod sse_tests;

#[cfg(all(test, target_arch = "aarch64"))]
mod jit_tests;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gpr_table_matches_register_widths() {
        assert_eq!(Cpu::reg_idx(Register::RAX), Some((0, 64, false)));
        assert_eq!(Cpu::reg_idx(Register::ECX), Some((1, 32, false)));
        assert_eq!(Cpu::reg_idx(Register::DX), Some((2, 16, false)));
        assert_eq!(Cpu::reg_idx(Register::BH), Some((3, 8, true)));
        assert_eq!(Cpu::reg_idx(Register::SPL), Some((4, 8, false)));
        assert_eq!(Cpu::reg_idx(Register::R15D), Some((15, 32, false)));
        assert_eq!(Cpu::reg_idx(Register::R8L), Some((8, 8, false)));
        assert_eq!(Cpu::reg_idx(Register::XMM0), None);
        assert_eq!(Cpu::reg_idx(Register::None), None);
        assert_eq!(Cpu::reg_idx(Register::RIP), None);
    }

    #[test]
    fn caller_extra_args_zero_full_and_partial_windows() {
        unsafe {
            assert_eq!(read_caller_extra_args(0, 0), [0; 8]);
            assert_eq!(read_caller_extra_args(0, u64::MAX), [0; 8]);
            let buf: [u64; 24] = core::array::from_fn(|i| 100 + i as u64);
            let rsp = buf.as_ptr() as u64;
            // Full window: args 9..16 at rsp + 0x40..0x80.
            assert_eq!(read_caller_extra_args(rsp, rsp + 0xc0), core::array::from_fn(|i| 108 + i as u64));
            // Window cut at rsp + 0x58: the first three args, the rest 0.
            assert_eq!(read_caller_extra_args(rsp, rsp + 0x58), [108, 109, 110, 0, 0, 0, 0, 0]);
        }
    }

    fn alloc_guest_code(bytes: &[u8]) -> u64 {
        let mut buf = vec![0xCCu8; bytes.len().max(64) + 32].into_boxed_slice();
        buf[..bytes.len()].copy_from_slice(bytes);
        let leaked: &'static mut [u8] = Box::leak(buf);
        leaked.as_mut_ptr() as u64
    }

    #[test]
    fn interp_burst_self_loop_and_inter_block_chaining() {
        // Multi-block guest program exercising:
        // - entry block calling a helper (`call rel32`)
        // - helper with a tight self-loop (`add rax, rdx; dec rcx; jne self; ret`)
        // - conditional branch + unconditional jmp + fallthrough + ret
        //
        // Layout:
        //   0x00: mov eax, 0             ; B8 00 00 00 00 (5)
        //   0x05: call +0x10 (0x1A)      ; E8 10 00 00 00 (5)
        //   0x0A: add rax, 7             ; 48 83 C0 07    (4)
        //   0x0E: jmp +0x04 (0x14)       ; EB 04          (2)
        //   0x10: nop; nop; nop; nop     ; 90 90 90 90    (4)
        //   0x14: add rax, 1             ; 48 83 C0 01    (4)
        //   0x18: ret                    ; C3             (1)
        //   0x19: nop                    ; 90             (1)
        //   0x1A: add rax, rdx           ; 48 01 D0       (3)  <- loop_head
        //   0x1D: dec rcx                ; 48 FF C9       (3)
        //   0x20: jne -8 (0x1A)          ; 75 F8          (2)
        //   0x22: ret                    ; C3             (1)
        #[rustfmt::skip]
        let code: [u8; 35] = [
            0xB8, 0x00, 0x00, 0x00, 0x00,
            0xE8, 0x10, 0x00, 0x00, 0x00,
            0x48, 0x83, 0xC0, 0x07,
            0xEB, 0x04,
            0x90, 0x90, 0x90, 0x90,
            0x48, 0x83, 0xC0, 0x01,
            0xC3,
            0x90,
            0x48, 0x01, 0xD0,
            0x48, 0xFF, 0xC9,
            0x75, 0xF8,
            0xC3,
        ];
        let ip = alloc_guest_code(&code);
        jit::force(2); // interpreter mode
        TEST_INTERP_BURSTS.with(|c| c.set(0));
        TEST_INTERP_BURST_CHAINS.with(|c| c.set(0));

        // First run warms and stamps all blocks.
        let (r1, _) = unsafe { emu_run_full(ip, [100, 3, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]) };
        assert_eq!(r1, 100 * 3 + 7 + 1);
        let steps1 = test_last_steps();
        // Exact instruction count: 2 (mov+call) + 100*3 (loop) + 1 (ret) + 2 (add+jmp) + 2 (add+ret) = 307.
        assert_eq!(steps1, 307);

        // Second run: every block is stamped -> entire call+loop+ret+jmp+ret runs in ONE burst!
        TEST_INTERP_BURSTS.with(|c| c.set(0));
        TEST_INTERP_BURST_CHAINS.with(|c| c.set(0));
        let (r2, _) = unsafe { emu_run_full(ip, [500, 5, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]) };
        jit::force(0);
        assert_eq!(r2, 500 * 5 + 7 + 1);
        assert_eq!(test_last_steps(), 2 + 500 * 3 + 1 + 2 + 2);
        assert_eq!(
            TEST_INTERP_BURSTS.with(|c| c.get()),
            1,
            "warm multi-block call+self-loop+ret+jmp must finish in a single interp_burst"
        );
        // Chains: call->loop (1) + 499 self-loops + fallthrough->ret (1) + ret->caller (1) + jmp->tail (1) = 503.
        assert_eq!(TEST_INTERP_BURST_CHAINS.with(|c| c.get()), 503);
    }

    #[test]
    fn interp_burst_generic_fallback_and_resume() {
        // Fast op -> generic op (`sidt [rcx]`, K_GENERIC) -> fast op -> ret in one block.
        #[rustfmt::skip]
        let full: [u8; 15] = [
            0x48, 0xC7, 0xC0, 0x2A, 0x00, 0x00, 0x00, // mov rax, 42
            0x0F, 0x01, 0x09,                         // sidt [rcx] (K_GENERIC)
            0x48, 0x83, 0xC0, 0x08,                   // add rax, 8
            0xC3,                                     // ret
        ];
        let ip = alloc_guest_code(&full);
        let mut out = [0u8; 16];
        jit::force(2);
        let (r, _) = unsafe {
            emu_run_full(ip, [out.as_mut_ptr() as u64, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8])
        };
        jit::force(0);
        assert_eq!(r, 50);
        assert_eq!(u16::from_le_bytes([out[0], out[1]]), 0x0fff);
        assert_eq!(test_last_steps(), 4);
    }

    #[test]
    fn interp_burst_va_gen_invalidation_dispatches_new_host_hook() {
        unsafe extern "win64" fn hook_returns_222() -> u64 {
            222
        }
        // Callee at +0x10: mov eax, 111; ret
        // Caller at +0x00: call +0x0B (+0x10); ret
        #[rustfmt::skip]
        let code: [u8; 22] = [
            0xE8, 0x0B, 0x00, 0x00, 0x00,             // call +0x10
            0xC3,                                     // ret
            0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
            0xB8, 0x6F, 0x00, 0x00, 0x00,             // mov eax, 111
            0xC3,                                     // ret
        ];
        let caller_ip = alloc_guest_code(&code);
        let callee_ip = caller_ip + 0x10;

        jit::force(2);
        // Warm & chain caller -> guest callee in interp_burst.
        assert_eq!(unsafe { emu_run_full(caller_ip, [0; 8], [0; 4], [0; 8]).0 }, 111);
        TEST_INTERP_BURSTS.with(|c| c.set(0));
        TEST_INTERP_BURST_CHAINS.with(|c| c.set(0));
        assert_eq!(unsafe { emu_run_full(caller_ip, [0; 8], [0; 4], [0; 8]).0 }, 111);
        assert_eq!(TEST_INTERP_BURSTS.with(|c| c.get()), 1);
        assert_eq!(TEST_INTERP_BURST_CHAINS.with(|c| c.get()), 2); // call->callee + ret->caller

        // Register a host hook at `callee_ip` (bumps `exe::va_gen()`).
        crate::exe::register_native_va(callee_ip, hook_returns_222 as *const () as usize);
        let hooked = unsafe { emu_run_full(caller_ip, [0; 8], [0; 4], [0; 8]).0 };
        crate::exe::unregister_native_va(callee_ip);
        jit::force(0);
        assert_eq!(hooked, 222, "va_gen bump must invalidate stamped fast_gen and dispatch host hook");
    }

    #[test]
    fn interp_burst_code_gen_invalidation() {
        // 1. Verify `note_code_region` bumps `BlockCache::fast_gen` so stamped entries become stale.
        let code: [u8; 6] = [0xB8, 0x0A, 0x00, 0x00, 0x00, 0xC3]; // mov eax, 10; ret
        let ip = alloc_guest_code(&code);
        jit::force(2);
        assert_eq!(unsafe { emu_run_full(ip, [0; 8], [0; 4], [0; 8]).0 }, 10);
        let (old_fgen, stamped_fgen) = TLS_EMU.with(|cell| unsafe {
            let st = (*cell.get()).as_ref().unwrap();
            let e = st.blocks.index[BlockCache::slot(ip)];
            (st.blocks.fast_gen, e.fast_gen)
        });
        assert_ne!(old_fgen, 0);
        assert_eq!(stamped_fgen, old_fgen);

        // 2. Allocate a dynamic-code buffer, register it with `note_code_region` (bumps code_gen),
        //    execute it (must NOT be stamped since `e.gen != 0`), rewrite it, bump code_gen again,
        //    and verify the rewritten code runs.
        let dyn_buf: &'static mut [u8] = Box::leak(vec![0xCCu8; 64].into_boxed_slice());
        dyn_buf[..6].copy_from_slice(&[0xB8, 0x15, 0x00, 0x00, 0x00, 0xC3]); // mov eax, 21; ret
        let dyn_lo = dyn_buf.as_ptr() as usize;
        mem::note_code_region(dyn_lo, dyn_lo + 64);

        let r1 = unsafe { emu_run_full(dyn_lo as u64, [0; 8], [0; 4], [0; 8]).0 };
        assert_eq!(r1, 21);
        let (new_fgen, old_entry_fgen, dyn_entry) = TLS_EMU.with(|cell| unsafe {
            let st = (*cell.get()).as_ref().unwrap();
            let old_e = st.blocks.index[BlockCache::slot(ip)];
            let dyn_e = st.blocks.index[BlockCache::slot(dyn_lo as u64)];
            (st.blocks.fast_gen, old_e.fast_gen, dyn_e)
        });
        assert_ne!(new_fgen, old_fgen, "code_gen bump must advance BlockCache::fast_gen");
        assert_ne!(old_entry_fgen, new_fgen, "previously stamped entry must be invalidated");
        assert_ne!(dyn_entry.gen, 0);
        assert_eq!(dyn_entry.fast_gen, 0, "dynamic JIT blocks (e.gen != 0) must never be fast_gen stamped");

        // Rewrite dynamic code in place and bump `code_gen` again.
        dyn_buf[..6].copy_from_slice(&[0xB8, 0x2A, 0x00, 0x00, 0x00, 0xC3]); // mov eax, 42; ret
        mem::note_code_region(dyn_lo, dyn_lo + 64);
        let r2 = unsafe { emu_run_full(dyn_lo as u64, [0; 8], [0; 4], [0; 8]).0 };
        jit::force(0);
        assert_eq!(r2, 42);
    }

    #[test]
    fn interp_burst_plain_leaf_host_call_and_tail_call() {
        // Register a known DF_PLAIN builtin (`QueryPerformanceCounter`) at a synthetic VA and
        // call it via both guest `call rax` and guest tail `jmp rax` in interpreter mode.
        let qpc_va = 0x7_7700_0100u64;
        let qpc_fn = win32_rt::kernel32::time::QueryPerformanceCounter as *const () as usize;
        crate::exe::register_native_va(qpc_va, qpc_fn);

        // `call rax; ret` (rcx = &counter, rax = qpc_va)
        let call_code = alloc_guest_code(&[0xFF, 0xD0, 0xC3]);
        // `jmp rax` (tail call: rcx = &counter, rax = qpc_va)
        let jmp_code = alloc_guest_code(&[0xFF, 0xE0]);

        jit::force(2);
        TEST_PLAIN_HOST_CALLS.with(|c| c.set(0));
        let mut q1 = 0i64;
        let (r1, _) = unsafe {
            emu_run_full_rax(call_code, [&mut q1 as *mut i64 as u64, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8], qpc_va)
        };
        assert_eq!(r1 as u32, 1);
        assert_ne!(q1, 0);
        assert_eq!(TEST_PLAIN_HOST_CALLS.with(|c| c.get()), 1);

        let mut q2 = 0i64;
        let (r2, _) = unsafe {
            emu_run_full_rax(jmp_code, [&mut q2 as *mut i64 as u64, 0, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8], qpc_va)
        };
        jit::force(0);
        crate::exe::unregister_native_va(qpc_va);
        assert_eq!(r2 as u32, 1);
        assert_ne!(q2, 0);
        assert_eq!(TEST_PLAIN_HOST_CALLS.with(|c| c.get()), 2);
    }
}

#[cfg(all(test, target_arch = "aarch64"))]
mod jit_coverage;

#[cfg(all(test, target_arch = "aarch64"))]
mod jit_bench;

/// Test-only: translate the guest block at `ip` with this thread's translator (forced on)
/// without running it; returns its host code address. The thread's emulator state must
/// exist (any earlier `emu_run_full` on this thread).
#[cfg(all(test, target_arch = "aarch64"))]
unsafe fn test_translate(ip: u64) -> Option<usize> {
    let (blocks, jit) = TLS_EMU.with(|cell| {
        let st = (*cell.get()).as_mut().expect("emulator state (run emu_run_full first)");
        (&mut st.blocks as *mut BlockCache, &mut st.jit as *mut jit::JitCache)
    });
    jit::force(1);
    let r = (*jit).lookup(ip, blocks);
    jit::force(0);
    r
}

/// [`lean_call`] / [`call_lean_in`] (the `DF_LEAN` direct-call path) against the generic
/// bridge ([`host_call_asm`] / [`call_host_dispatched_in`]): same argument image at the callee
/// (x0..x7, q0..q3, stack args 9..16) and same `CallCtx` effects and results.
#[cfg(all(test, target_arch = "aarch64"))]
mod lean_tests {
    use super::*;
    use core::arch::aarch64::uint64x2_t;
    use std::cell::Cell;

    std::thread_local! {
        static SEEN: Cell<[u64; 24]> = const { Cell::new([0; 24]) };
    }

    fn q2(v: uint64x2_t) -> [u64; 2] {
        unsafe { core::mem::transmute(v) }
    }

    #[allow(improper_ctypes_definitions, clippy::too_many_arguments)]
    unsafe extern "C" fn echo(
        a0: u64, a1: u64, a2: u64, a3: u64, a4: u64, a5: u64, a6: u64, a7: u64,
        v0: uint64x2_t, v1: uint64x2_t, v2: uint64x2_t, v3: uint64x2_t,
        s0: u64, s1: u64, s2: u64, s3: u64, s4: u64, s5: u64, s6: u64, s7: u64,
    ) -> u64 {
        let (v0, v1, v2, v3) = (q2(v0), q2(v1), q2(v2), q2(v3));
        let all = [
            a0, a1, a2, a3, a4, a5, a6, a7, v0[0], v0[1], v1[0], v1[1], v2[0], v2[1], v3[0], v3[1],
            s0, s1, s2, s3, s4, s5, s6, s7,
        ];
        SEEN.with(|s| s.set(all));
        all.iter().fold(0u64, |h, &x| h.rotate_left(7) ^ x)
    }

    /// A Win64-declared replacement with 10 integer args (9 and 10 come from the stack).
    #[allow(clippy::too_many_arguments)]
    unsafe extern "win64" fn ten(a: u64, b: u64, c: u64, d: u64, e: u64, f: u64, g: u64, h: u64, i: u64, j: u64) -> u64 {
        a ^ b.rotate_left(1) ^ c.rotate_left(2) ^ d.rotate_left(3) ^ e.rotate_left(4)
            ^ f.rotate_left(5) ^ g.rotate_left(6) ^ h.rotate_left(7) ^ i.rotate_left(8) ^ j.rotate_left(9)
    }

    /// A Win64-declared replacement with a float parameter (arrives in d0 = low xmm0).
    unsafe extern "win64" fn with_f64(x: f64, p: u64) -> u64 {
        x.to_bits() ^ p.rotate_left(13)
    }

    fn inputs() -> ([u64; 8], [u128; 4], [u64; 8]) {
        let gpr = core::array::from_fn(|i| 0x1111_0000_0000_0001u64.wrapping_mul(i as u64 + 3));
        let xmm = core::array::from_fn(|i| {
            ((0xA5A5_0000_0000_0000u128 | i as u128) << 64) | (0x5A5A_0000_0000_0100u128 + i as u128)
        });
        let st = core::array::from_fn(|i| 0xC0DE_0000_0000_0000u64 | (i as u64 + 9));
        (gpr, xmm, st)
    }

    #[test]
    fn lean_call_matches_host_call_asm() {
        let (gpr, xmm, st) = inputs();
        unsafe {
            let r_lean = lean_call(echo as *const () as usize, &gpr, &xmm, &st);
            let seen_lean = SEEN.with(|s| s.get());
            SEEN.with(|s| s.set([0; 24]));
            let (r_asm, _) = host_call_asm(echo as *const () as usize, &gpr, &xmm, &st);
            let seen_asm = SEEN.with(|s| s.get());
            assert_eq!(seen_lean, seen_asm);
            assert_eq!(r_lean, r_asm);
            let mut want = [0u64; 24];
            want[..8].copy_from_slice(&gpr);
            for (i, x) in xmm.iter().enumerate() {
                want[8 + 2 * i] = *x as u64;
                want[9 + 2 * i] = (*x >> 64) as u64;
            }
            want[16..].copy_from_slice(&st);
            assert_eq!(seen_lean, want);

            for f in [ten as *const () as usize, with_f64 as *const () as usize] {
                assert_eq!(lean_call(f, &gpr, &xmm, &st), host_call_asm(f, &gpr, &xmm, &st).0);
            }
            assert_eq!(lean_call(ten as *const () as usize, &gpr, &xmm, &st), ten(gpr[0], gpr[1], gpr[2], gpr[3], gpr[4], gpr[5], gpr[6], gpr[7], st[0], st[1]));
            assert_eq!(lean_call(with_f64 as *const () as usize, &gpr, &xmm, &st), with_f64(f64::from_bits(xmm[0] as u64), gpr[0]));
        }
    }

    /// Lifted-style callee: reads its XMM args and `rax` from `CallCtx`, optionally sets an
    /// XMM result.
    #[allow(clippy::too_many_arguments)]
    unsafe extern "win64" fn tls_callee(a: u64, b: u64, _c: u64, _d: u64, _e: u64, _f: u64, _g: u64, h: u64) -> u64 {
        let x = crate::simd_ops::take_xmm_args();
        let rax = crate::simd_ops::get_rax_in();
        if a & 1 == 1 {
            crate::simd_ops::set_xmm_ret(x[1] ^ x[2]);
        }
        (x[0] as u64) ^ ((x[3] >> 64) as u64) ^ rax.rotate_left(3) ^ b ^ h
    }

    #[test]
    fn call_lean_in_matches_bridge_protocol() {
        let (mut gpr, xmm, st) = inputs();
        let c = crate::callctx::ctx();
        for a in [1u64, 2] {
            gpr[0] = a;
            unsafe {
                let prev = crate::simd_ops::replace_rax_in(0x7777_0000_0000_0042);
                let lean = call_lean_in(c, tls_callee as *const () as usize, &gpr, &xmm, &st);
                let lean_state = (c.xmm_in_set.get(), c.xmm_out.get(), c.xmm_in.get());
                let full = call_host_dispatched_in(
                    c,
                    0x1_4000_1000,
                    tls_callee as *const () as usize,
                    DF_VALID | DF_HOST | DF_GAME_REPL,
                    &gpr,
                    &xmm,
                    &st,
                );
                let full_state = (c.xmm_in_set.get(), c.xmm_out.get(), c.xmm_in.get());
                crate::simd_ops::replace_rax_in(prev);
                assert_eq!(lean, full, "a={a}");
                assert_eq!(lean_state, full_state, "a={a}");
                assert_eq!(lean.1, if a == 1 { xmm[1] ^ xmm[2] } else { xmm[0] });
                assert!(!lean_state.0);
            }
        }
    }

    // --- DF_PLAIN builtins called from inside a lean-called function -------------------

    const VA_OUTER: u64 = 0x1_4000_2000;
    const VA_TRY_ACQ: u64 = 0x1_4000_3000;
    const VA_QPC: u64 = 0x1_4000_3100;
    const VA_REL: u64 = 0x1_4000_3200;

    /// (outgoing, incoming, xmm_in_set, rax_in) after a nested call. (`xmm_in` is only
    /// meaningful while `xmm_in_set`; the bridge rewrites it, the plain path doesn't.)
    type Post = ((u64, u64), (u64, u64), bool, u64);

    std::thread_local! {
        static OBS: Cell<[Option<Post>; 3]> = const { Cell::new([None; 3]) };
        static FWD_AT_ENTRY: Cell<bool> = const { Cell::new(false) };
        static XOUT: Cell<[Option<u128>; 3]> = const { Cell::new([None; 3]) };
    }

    fn post(c: &crate::callctx::CallCtx) -> Post {
        (c.outgoing_rsp.get(), c.incoming_rsp.get(), c.xmm_in_set.get(), c.rax_in.get())
    }

    /// Install `va -> (host_fn, flags)` in this thread's dispatch cache, as
    /// `resolve_dispatch_in` would after a miss.
    fn seed(va: u64, host_fn: usize, flags: u32) {
        use crate::callctx::{DispatchSlot, DCACHE_BITS, DCACHE_LEN};
        let c = crate::callctx::ctx();
        let idx = (((va >> 4) ^ (va >> (4 + DCACHE_BITS))) as usize) & (DCACHE_LEN - 1);
        unsafe {
            (*c.dcache.get())[idx] = DispatchSlot { va, host_fn, flags, gen: crate::exe::va_gen() };
        }
    }

    /// Lifted-style function reached through the lean path. Its first nested call is a
    /// plain builtin made while `xmm_in_set` is still up; the second one has stack args
    /// pending (`call_stk`).
    #[allow(clippy::too_many_arguments)]
    unsafe extern "win64" fn outer(lock: u64, qpc_out: u64, _c: u64, _d: u64, _e: u64, _f: u64, _g: u64, _h: u64) -> u64 {
        let c = crate::callctx::ctx();
        FWD_AT_ENTRY.with(|f| f.set(c.xmm_in_set.get()));
        let mut obs = [None; 3];
        let mut xout = [None; 3];
        let r1 = call_va8(VA_TRY_ACQ, lock, 0, 0, 0, 0, 0, 0, 0) as u8 as u64;
        obs[0] = Some(post(c));
        xout[0] = c.xmm_out.get();
        let stk = [0x5151u64; 24];
        let r2 = crate::simd_ops::call_stk(VA_TRY_ACQ, 0x77, lock, 0, 0, 0, stk.as_ptr() as u64) as u8 as u64;
        obs[1] = Some(post(c));
        xout[1] = c.xmm_out.get();
        let r3 = call_va8(VA_QPC, qpc_out, 0, 0, 0, 0, 0, 0, 0) as u32 as u64;
        obs[2] = Some(post(c));
        xout[2] = c.xmm_out.get();
        call_va8(VA_REL, lock, 0, 0, 0, 0, 0, 0, 0);
        OBS.with(|o| o.set(obs));
        XOUT.with(|o| o.set(xout));
        0xABC0_0000 | r1 | r2 << 4 | r3 << 8
    }

    /// Run `outer` through `call_va8` (lean path) with the builtins dispatched as `flags`;
    /// returns (rax, nested-call post-states, xmm_out after each nested call, outer post-state).
    fn run(builtin_flags: u32) -> (u64, [Option<Post>; 3], [Option<u128>; 3], Post) {
        use win32_rt::{kernel32, sync};
        seed(VA_OUTER, outer as *const () as usize, LEAN_FLAGS);
        seed(VA_TRY_ACQ, sync::TryAcquireSRWLockExclusive as *const () as usize, builtin_flags);
        seed(VA_QPC, kernel32::time::QueryPerformanceCounter as *const () as usize, builtin_flags);
        seed(VA_REL, sync::ReleaseSRWLockExclusive as *const () as usize, builtin_flags);
        let c = crate::callctx::ctx();
        let lock = Box::new(0u64);
        let mut qpc = 0i64;
        unsafe {
            let prev = crate::simd_ops::replace_rax_in(0x7777_0000_0000_0042);
            c.outgoing_rsp.set((0, 0));
            c.incoming_rsp.set((0, 0));
            let rax = call_va8(VA_OUTER, &*lock as *const u64 as u64, &mut qpc as *mut i64 as u64, 3, 4, 5, 6, 7, 8);
            let after = post(c);
            crate::simd_ops::replace_rax_in(prev);
            assert_eq!(*lock, 0, "lock released");
            assert_ne!(qpc, 0, "QPC wrote the counter");
            (rax, OBS.with(|o| o.get()), XOUT.with(|o| o.get()), after)
        }
    }

    #[test]
    fn plain_builtin_first_in_lean_callee_matches_bridge() {
        // Seeded dispatch slots: no generation bump from other tests until the end.
        let _va = crate::exe::hold_va_gen();
        let plain = run(DF_VALID | DF_HOST | DF_PLAIN);
        assert!(FWD_AT_ENTRY.with(|f| f.get()), "first nested call happens with xmm_in_set up");
        let bridge = run(DF_VALID | DF_HOST);
        // TryAcquire: 1 (fresh lock), then 0 (held); QPC: TRUE.
        assert_eq!(plain.0, 0xABC0_0000 | 1 | 0 << 4 | 1 << 8);
        assert_eq!(plain.0, bridge.0, "rax");
        assert_eq!(plain.1, bridge.1, "post-state after each nested builtin call");
        assert_eq!(plain.3, bridge.3, "post-state after the lean call");
        for (i, p) in plain.1.iter().enumerate() {
            let (out, inc, fwd, _) = p.expect("observed");
            assert_eq!((out, inc, fwd), ((0, 0), (0, 0), false), "call {i}: windows and forwarded XMM consumed");
        }
        // rax_in: the lean entry's value, then `call_stk`'s rax (kept by the builtin call).
        assert_eq!(plain.1.map(|p| p.unwrap().3), [0x7777_0000_0000_0042, 0x77, 0x77]);
        // The plain path leaves no XMM result. (The bridge may publish the builtin's leftover
        // q0 when XMM was forwarded: undefined after an integer function.)
        assert_eq!(plain.2, [None; 3]);
        assert_eq!(bridge.2[1..], [None; 2]);
        let (out, inc, fwd, rax_in) = plain.3;
        assert_eq!((out, inc, fwd, rax_in), ((0, 0), (0, 0), false, 0x7777_0000_0000_0042));
    }

    // --- stack args 17..24+ across the call helpers (guest and lifted callees) ---------

    /// Guest x86 code: for k = 5..=`kmax`, `[rcx + 8*(k-5)] = [rsp + 8*k]` (incoming arg k),
    /// then `ret`. Placed outside the exe VA window.
    fn guest_store_args_code(kmax: u32) -> u64 {
        let mut code = Vec::new();
        for k in 5u32..=kmax {
            code.extend_from_slice(&[0x48, 0x8B, 0x84, 0x24]); // mov rax, [rsp+disp32]
            code.extend_from_slice(&(8 * k).to_le_bytes());
            code.extend_from_slice(&[0x48, 0x89, 0x81]); // mov [rcx+disp32], rax
            code.extend_from_slice(&(8 * (k - 5)).to_le_bytes());
        }
        code.push(0xC3); // ret
        loop {
            let m: &'static mut [u8] = Box::leak(vec![0xCCu8; 4096].into_boxed_slice());
            let p = m.as_mut_ptr() as u64;
            if p + 4096 > 0x1_4000_0000 && p < 0x1_4500_0000 {
                continue;
            }
            m[..code.len()].copy_from_slice(&code);
            return p;
        }
    }

    /// Lifted-style callee: copies its frame slots 5..=23 (incoming args 5..23) to `out`.
    /// Slot 24 (arg 24) is above the 24-qword argument area of today's frame.
    #[allow(clippy::too_many_arguments)]
    unsafe extern "win64" fn lifted_store_args(out: u64, _d: u64, _r8: u64, _r9: u64, a5: u64, a6: u64, a7: u64, a8: u64) -> u64 {
        let mut f = crate::simd_ops::LiftedFrame::<256, 8>::new();
        let rsp = crate::simd_ops::init_lifted_stack(&mut f, a5, a6, a7, a8);
        for k in 5..24u64 {
            ((out + 8 * (k - 5)) as *mut u64).write(((rsp + 8 * k) as *const u64).read());
        }
        0x5157
    }

    const VA_LIFT_LEAN: u64 = 0x1_4000_4000;
    const VA_LIFT_HOST: u64 = 0x1_4000_4100;

    /// Caller-side outgoing area: arg k at `[rsp + 8*(k-1)]` holds `0xA000 + k`.
    fn outgoing_args() -> [u64; 32] {
        core::array::from_fn(|i| 0xA000 + i as u64 + 1)
    }

    /// `call_stk` (window `rsp..rsp+0xc0`) into an emulated callee: args 5..24 arrive
    /// (arg5..8 from the call, 9..16 `s_extra`, 17..24 `hi_args` cut at the window end).
    #[test]
    fn call_stk_passes_args_up_to_24_to_a_guest_callee() {
        let code = guest_store_args_code(24);
        let stk = outgoing_args();
        let mut out = [0u64; 20];
        let c = crate::callctx::ctx();
        unsafe {
            crate::simd_ops::call_stk(code, 0, out.as_mut_ptr() as u64, 0, 0, 0, stk.as_ptr() as u64);
        }
        let want: [u64; 20] = core::array::from_fn(|i| 0xA000 + 5 + i as u64);
        assert_eq!(out, want);
        assert_eq!((c.outgoing_rsp.get(), c.incoming_rsp.get()), ((0, 0), (0, 0)));
    }

    /// `call_stk` into a lifted callee, lean (`call_lean_in`) and generic (`DF_HOST`) dispatch:
    /// the callee frame gets args 5..23 (`init_lifted_stack` + `copy_incoming_stack_args`).
    #[test]
    fn call_stk_passes_args_up_to_23_to_a_lifted_callee() {
        // Seeded dispatch slots: no generation bump from other tests until the end.
        let _va = crate::exe::hold_va_gen();
        seed(VA_LIFT_LEAN, lifted_store_args as *const () as usize, LEAN_FLAGS);
        seed(VA_LIFT_HOST, lifted_store_args as *const () as usize, DF_VALID | DF_HOST | DF_GAME_REPL);
        let stk = outgoing_args();
        let want: [u64; 19] = core::array::from_fn(|i| 0xA000 + 5 + i as u64);
        let c = crate::callctx::ctx();
        for va in [VA_LIFT_LEAN, VA_LIFT_HOST] {
            let mut out = [0u64; 19];
            let r = unsafe { crate::simd_ops::call_stk(va, 0, out.as_mut_ptr() as u64, 0, 0, 0, stk.as_ptr() as u64) };
            assert_eq!(r, 0x5157, "va {va:#x}");
            assert_eq!(out, want, "va {va:#x}");
            assert_eq!((c.outgoing_rsp.get(), c.incoming_rsp.get()), ((0, 0), (0, 0)), "va {va:#x}");
        }
    }

    /// `read_s5_call` from an arena frame publishes the frame top as the window end, so an
    /// emulated callee gets args beyond 24 (here 5..26, the cohtml +0x481dc0 -> +0x490110
    /// shape, a `call_va8_xmm` site) as long as they lie inside the caller's frame.
    #[test]
    fn read_s5_call_passes_args_beyond_24_to_a_guest_callee() {
        let code = guest_store_args_code(26);
        let c = crate::callctx::ctx();
        let want: [u64; 22] = core::array::from_fn(|i| 0xA000 + 5 + i as u64);
        for xmm_form in [false, true] {
            let mut out = [0u64; 22];
            unsafe {
                let mut f = crate::simd_ops::LiftedFrame::<256, 40>::new();
                let base = crate::simd_ops::init_lifted_stack(&mut f, 0, 0, 0, 0);
                let rsp = base - 0x100;
                for k in 5..=26u64 {
                    ((rsp + 8 * (k - 1)) as *mut u64).write(0xA000 + k);
                }
                let s5 = crate::simd_ops::read_s5_call(rsp, &f);
                let (s6, s7, s8) = (
                    ((rsp + 0x28) as *const u64).read(),
                    ((rsp + 0x30) as *const u64).read(),
                    ((rsp + 0x38) as *const u64).read(),
                );
                let o = out.as_mut_ptr() as u64;
                if xmm_form {
                    crate::simd_ops::call_va8_xmm(code, o, 0, 0, 0, s5, s6, s7, s8, [0; 4]);
                } else {
                    crate::exe::call_va8(code, o, 0, 0, 0, s5, s6, s7, s8);
                }
            }
            assert_eq!(out, want, "xmm_form {xmm_form}");
            assert_eq!((c.outgoing_rsp.get(), c.incoming_rsp.get()), ((0, 0), (0, 0)), "xmm_form {xmm_form}");
        }
    }

    /// The loader's aarch64 `native_try_thunk` (aw2-loader hooks.rs) minus the
    /// `ORIG_TRAMPOLINE_TAG` branch: host-callable targets through `call_host_for_va`, guest
    /// code through `emu_run_full`. Neither consumes the window `call_va8_caught` publishes.
    unsafe extern "win64" fn test_try_thunk(target: usize, args: *const u64, _n: usize, frame: *mut crate::exe::TryFrame) -> u64 {
        let a = &*(args as *const [u64; crate::exe::TRY_MAX_ARGS]);
        let va = target as u64;
        let xmm = [a[0] as u128, a[1] as u128, a[2] as u128, a[3] as u128];
        let gpr: [u64; 8] = core::array::from_fn(|i| a[i]);
        let st: [u64; 8] = core::array::from_fn(|i| a[8 + i]);
        let (r, x) = match resolve_host_callable(va) {
            Some(h) => call_host_for_va(va, h, gpr, xmm, st),
            None => emu_run_full(va, gpr, xmm, st),
        };
        (*frame).rax = r;
        (*frame).xmm0 = x as u64;
        0
    }

    /// `call_stk_caught` -> `call_va8_caught` -> `exe::try_call` passes only 16 args, but it
    /// publishes the caller's window as `incoming_rsp` first, and `emu_run_full` takes args
    /// 17..24 from there (`hi_args`): an emulated callee still gets args 5..24.
    #[test]
    fn call_stk_caught_passes_args_up_to_24_to_a_guest_callee() {
        crate::exe::set_try_thunk(test_try_thunk as *const () as usize);
        let code = guest_store_args_code(24);
        let stk = outgoing_args();
        let mut out = [0u64; 20];
        let c = crate::callctx::ctx();
        unsafe {
            crate::simd_ops::call_stk_caught(code, 0, out.as_mut_ptr() as u64, 0, 0, 0, stk.as_ptr() as u64);
        }
        let want: [u64; 20] = core::array::from_fn(|i| 0xA000 + 5 + i as u64);
        assert_eq!(out, want);
        assert_eq!((c.outgoing_rsp.get(), c.incoming_rsp.get()), ((0, 0), (0, 0)));
    }

    /// Wide lifted callee (`LiftedFrame<256, 8, 26>`: reads args up to 25): copies its frame
    /// slots 5..=25 to `out`.
    #[allow(clippy::too_many_arguments)]
    unsafe extern "win64" fn lifted_store_args_wide(out: u64, _d: u64, _r8: u64, _r9: u64, a5: u64, a6: u64, a7: u64, a8: u64) -> u64 {
        let mut f = crate::simd_ops::LiftedFrame::<256, 8, 26>::new();
        let rsp = crate::simd_ops::init_lifted_stack(&mut f, a5, a6, a7, a8);
        for k in 5..26u64 {
            ((out + 8 * (k - 5)) as *mut u64).write(((rsp + 8 * k) as *const u64).read());
        }
        0x5158
    }

    const VA_LIFT_LEAN_W: u64 = 0x1_4000_4200;
    const VA_LIFT_HOST_W: u64 = 0x1_4000_4300;

    /// `call_stk_w::<0xd0>` (a call site writing its outgoing area up to `[rsp+0xc8]`, the
    /// cohtml +0x481dc0 shape) passes args 5..26 to emulated callees through every form, and
    /// args 5..25 to a wide lifted callee (lean and `DF_HOST` dispatch). Today's `call_stk`
    /// cuts the same lifted callee at arg24.
    #[test]
    fn call_stk_w_passes_args_up_to_its_limit() {
        // Seeded dispatch slots: no generation bump from other tests until the end.
        let _va = crate::exe::hold_va_gen();
        let code = guest_store_args_code(26);
        let stk = outgoing_args();
        let s = stk.as_ptr() as u64;
        let c = crate::callctx::ctx();
        let want: [u64; 22] = core::array::from_fn(|i| 0xA000 + 5 + i as u64);
        crate::exe::set_try_thunk(test_try_thunk as *const () as usize);
        for form in 0..3 {
            let mut out = [0u64; 22];
            let o = out.as_mut_ptr() as u64;
            unsafe {
                match form {
                    0 => {
                        crate::simd_ops::call_stk_w::<0xd0>(code, 0, o, 0, 0, 0, s);
                    }
                    1 => {
                        crate::simd_ops::call_stk_caught_w::<0xd0>(code, 0, o, 0, 0, 0, s);
                    }
                    _ => {
                        crate::simd_ops::call_stk_xmm_w::<0xd0>(code, 0, o, 0, 0, 0, s, [0; 4]);
                    }
                }
            }
            assert_eq!(out, want, "guest form {form}");
            assert_eq!((c.outgoing_rsp.get(), c.incoming_rsp.get()), ((0, 0), (0, 0)), "form {form}");
        }
        seed(VA_LIFT_LEAN_W, lifted_store_args_wide as *const () as usize, LEAN_FLAGS);
        seed(VA_LIFT_HOST_W, lifted_store_args_wide as *const () as usize, DF_VALID | DF_HOST | DF_GAME_REPL);
        let want: [u64; 21] = core::array::from_fn(|i| 0xA000 + 5 + i as u64);
        for va in [VA_LIFT_LEAN_W, VA_LIFT_HOST_W] {
            let mut out = [0u64; 21];
            let r = unsafe { crate::simd_ops::call_stk_w::<0xd0>(va, 0, out.as_mut_ptr() as u64, 0, 0, 0, s) };
            assert_eq!(r, 0x5158, "va {va:#x}");
            assert_eq!(out, want, "va {va:#x}");
            assert_eq!((c.outgoing_rsp.get(), c.incoming_rsp.get()), ((0, 0), (0, 0)), "va {va:#x}");
            // Today's window: arg24 still arrives, arg25 reads the clean frame's 0.
            let mut out = [0u64; 21];
            unsafe { crate::simd_ops::call_stk(va, 0, out.as_mut_ptr() as u64, 0, 0, 0, s) };
            assert_eq!(out[..20], want[..20], "va {va:#x} call_stk");
            assert_eq!(out[20], 0, "va {va:#x} call_stk arg25");
        }
    }

    /// `tail_stk_w::<0xd8>` (window `[rsp+8, rsp+0xd8)`): the emulated tail callee gets args
    /// 5..26 of the tail site's own incoming area (arg k at `[rsp + 8*k]`), in every form.
    #[test]
    fn tail_stk_w_passes_args_up_to_its_limit() {
        let code = guest_store_args_code(26);
        let stk = outgoing_args();
        let s = stk.as_ptr() as u64;
        let c = crate::callctx::ctx();
        // `stk[k]` (at `s + 8k`) = 0xA000 + k + 1.
        let want: [u64; 22] = core::array::from_fn(|i| 0xA000 + 5 + i as u64 + 1);
        crate::exe::set_try_thunk(test_try_thunk as *const () as usize);
        for form in 0..3 {
            let mut out = [0u64; 22];
            let o = out.as_mut_ptr() as u64;
            unsafe {
                match form {
                    0 => {
                        crate::simd_ops::tail_stk_w::<0xd8>(code, 0, o, 0, 0, 0, s);
                    }
                    1 => {
                        crate::simd_ops::tail_stk_caught_w::<0xd8>(code, 0, o, 0, 0, 0, s);
                    }
                    _ => {
                        crate::simd_ops::tail_stk_xmm_w::<0xd8>(code, 0, o, 0, 0, 0, s, [0; 4]);
                    }
                }
            }
            assert_eq!(out, want, "tail form {form}");
            assert_eq!((c.outgoing_rsp.get(), c.incoming_rsp.get()), ((0, 0), (0, 0)), "form {form}");
        }
    }

    #[test]
    fn unprivileged_descriptor_table_and_msw_stores() {
        // sidt [rcx]; sgdt [rcx+10]; sldt word [rcx+20]; str word [rcx+22];
        // smsw word [rcx+24]; smsw eax; ret
        static CODE: [u8; 22] = [
            0x0F, 0x01, 0x09, // sidt [rcx]
            0x0F, 0x01, 0x41, 0x0A, // sgdt [rcx+10]
            0x0F, 0x00, 0x41, 0x14, // sldt word ptr [rcx+20]
            0x0F, 0x00, 0x49, 0x16, // str word ptr [rcx+22]
            0x0F, 0x01, 0x61, 0x18, // smsw word ptr [rcx+24]
            0x0F, 0x01, 0xE0, // smsw eax
        ];
        let mut full = CODE.to_vec();
        full.push(0xC3); // ret
        let mut out = [0xAAu8; 32];
        let (rax, _) = unsafe {
            emu_run_full(
                full.as_ptr() as u64,
                [out.as_mut_ptr() as u64, 0, 0, 0, 0, 0, 0, 0],
                [0; 4],
                [0; 8],
            )
        };
        assert_eq!(u16::from_le_bytes([out[0], out[1]]), 0x0fff);
        assert_eq!(u64::from_le_bytes(out[2..10].try_into().unwrap()), 0xffff_f800_0000_0000);
        assert_eq!(u16::from_le_bytes([out[10], out[11]]), 0x0057);
        assert_eq!(u64::from_le_bytes(out[12..20].try_into().unwrap()), 0xffff_f800_0000_1000);
        assert_eq!(u16::from_le_bytes([out[20], out[21]]), 0x0000);
        assert_eq!(u16::from_le_bytes([out[22], out[23]]), 0x0040);
        assert_eq!(u16::from_le_bytes([out[24], out[25]]), 0x0033);
        assert_eq!(rax, 0x8005_0033);
    }
}
