//! Guest-to-host and native routing tables: primary-EXE CRT/import-thunk routes (via
//! [`crate::hooks`]), mapped-DLL IAT stubs, byte-detected `__chkstk`/`stack_pos` helpers,
//! `DF_REGFILE` lookup, and per-thread `resolve_dispatch` memoization.

use super::block_cache::{ThreadEmuState, GUEST_FLOOR_MARGIN, GUEST_STACK_MIN_OFF, TLS_EMU};
use super::cpu::{precise_faults, Cpu, FAULT_PENDING, RET_SENTINEL};
use super::{eh, note_null_call, regfile};
use crate::exe::{base, lookup_native_va, rd, teb_self, wr};

/// Check whether `va_or_addr` is a registered native host function pointer in `VA_DISPATCH`
/// (including 6-byte `FF 25 <disp32>` IAT jump thunks in the primary PE module).
#[inline]
pub fn resolve_host_callable(va_or_addr: u64) -> Option<usize> {
    // Special-ABI CRT helpers (`__security_check_cookie` preserves RAX; `__chkstk` takes size
    // in RAX; `_guard_dispatch_icall` takes target in RAX) must never be dispatched as standard
    // Win64 host functions. Registered per-game via `crate::hooks::register_exe_crt_routes`.
    if crate::hooks::is_exe_special_abi_va(va_or_addr) && crate::exe::aw2_registry_active() {
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
// Primary-EXE guest entries (native routing for import thunks & registered CRT/CFG helpers).
//
// When a primary-EXE replacement registry is active, import thunks (`FF 25` `jmp [rip+disp32]`)
// and registered special-ABI CRT helpers (`__security_check_cookie`, `__chkstk`,
// `_guard_dispatch_icall` via `crate::hooks::register_exe_crt_routes`) are routed natively so
// guest callers never translate/interpret exe thunk blocks. Not routed:
// - a non-thunk `entry_ip` (a difftest oracle runs the original exe function as x86 on purpose);
// - any other exe ip (e.g. `AW2_JIT_THUNK_ELIDE`). `AW2_EXE_GUEST_LOG=1` logs those (first 64
//   distinct ips).
// ---------------------------------------------------------------------------------------------

/// How [`exe_route`] handles a guest block start inside a primary EXE with an active registry.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum ExeRoute {
    /// Not routed: execute the guest instructions normally.
    None,
    /// Transfer to this target as a tail jump (stack unchanged): import thunk slot target, or
    /// `rax` for `_guard_dispatch_icall`.
    Jmp(u64),
    /// Return to the caller (pop the return address): `__chkstk` (stacks are committed,
    /// see `apply_fx`) and the `__security_check_cookie` success path.
    Ret,
}

/// Gate for [`exe_route`]: `(preferred base, mapped base, image size)` of the primary exe, or a
/// zero span when the primary-exe replacement registry isn't applied (other exes, unit tests).
#[inline]
pub(super) fn exe_route_gate() -> (u64, u64, u64) {
    if !crate::exe::aw2_registry_active() {
        return (0, 0, 0);
    }
    (crate::exe::preferred_base(), base() as u64, crate::exe::image_size())
}

/// `ip` lies in the primary exe image (`gate` from [`exe_route_gate`]).
#[inline(always)]
pub(super) fn in_exe_gate(gate: (u64, u64, u64), ip: u64) -> bool {
    ip.wrapping_sub(gate.0) < gate.2 || ip.wrapping_sub(gate.1) < gate.2
}

/// Slot target of the exe import thunk `jmp qword ptr [rip+disp32]` at `ip`, if `ip` is one.
#[inline]
pub(super) unsafe fn exe_import_thunk_target(ip: u64) -> Option<u64> {
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
pub(super) fn exe_route_off() -> bool {
    EXE_ROUTE_OFF.with(|c| c.get())
}

#[inline]
pub(super) fn exe_routed_inc() {
    EXE_ROUTED.with(|c| c.set(c.get().wrapping_add(1)));
}

/// Route a guest block start `ip` inside the primary exe (see the section comment). Applies the
/// special helpers' register effects to `cpu`; the caller performs the transfer.
#[cold]
#[inline(never)]
pub(super) unsafe fn exe_route(cpu: &mut Cpu, ip: u64, entry_ip: u64) -> ExeRoute {
    if exe_route_off() {
        return ExeRoute::None;
    }
    if let Some(t) = exe_import_thunk_target(ip) {
        return ExeRoute::Jmp(t);
    }
    if ip != entry_ip {
        let guard_va = crate::hooks::exe_guard_dispatch_va();
        if guard_va != 0 && ip == guard_va {
            return ExeRoute::Jmp(cpu.gpr[0]);
        }
        if crate::hooks::is_exe_chkstk_va(ip) {
            return ExeRoute::Ret;
        }
        let check_cookie_va = crate::hooks::exe_check_cookie_va();
        if check_cookie_va != 0 && ip == check_cookie_va {
            return check_cookie(cpu);
        }
    }
    exe_guest_note(ip, entry_ip);
    ExeRoute::None
}

/// Native `__security_check_cookie`:
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
    let sec_cookie_va = crate::hooks::exe_security_cookie_va();
    let report_va = crate::hooks::exe_report_gsfailure_va();
    if sec_cookie_va == 0 || report_va == 0 {
        return ExeRoute::None;
    }
    let rcx = cpu.gpr[1];
    let cookie = rd::<u64>(to_mapped_addr(sec_cookie_va));
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
        ExeRoute::Jmp(report_va)
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
pub(super) fn dll_route_gate() -> (u64, u64) {
    if !dll_route_enabled() {
        return (0, 0);
    }
    crate::dll_native::iat_gate()
}

/// Slot target of the import stub at block start `ip` (inside the [`dll_route_gate`] span),
/// read now; `None` when `ip` is not a routable stub. The 2-byte opcode check keeps the
/// per-block cost to one load for ordinary DLL code.
#[inline(always)]
pub(super) unsafe fn dll_stub_route(ip: u64) -> Option<u64> {
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
pub(super) unsafe fn resolve_dll_stub_target(mut target: u64) -> Option<u64> {
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
    pub(super) static DLL_CHKSTK_OFF: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
}

/// DLL `__chkstk` invocations routed natively on this thread so far.
#[cfg_attr(not(all(test, target_arch = "aarch64")), allow(dead_code))]
pub(crate) fn dll_chkstk_routed() -> u64 {
    DLL_CHKSTK_ROUTED.with(|c| c.get())
}

#[inline]
pub(super) fn dll_chkstk_enabled() -> bool {
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
pub(super) unsafe fn dll_chkstk_call(cpu: &mut Cpu, sp_ptr: *mut u64) -> Option<(u64, u128)> {
    let caller_rsp = cpu.gpr[4];
    let ret_addr = cpu.rip;
    dll_chkstk_exec(cpu, sp_ptr, caller_rsp, ret_addr)
}

/// Native MSVC x64 `__chkstk` semantics for `jmp chkstk` or block-start entry (`[cpu.gpr[4]]`
/// holds the return address).
#[inline]
pub(super) unsafe fn dll_chkstk_tail(cpu: &mut Cpu, sp_ptr: *mut u64) -> Option<(u64, u128)> {
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
    pub(super) static DLL_STACK_POS_OFF: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
}

/// DLL `stack_pos` invocations routed natively on this thread so far.
#[cfg_attr(not(all(test, target_arch = "aarch64")), allow(dead_code))]
pub(crate) fn dll_stack_pos_routed() -> u64 {
    DLL_STACK_POS_ROUTED.with(|c| c.get())
}

#[inline]
pub(super) fn dll_stack_pos_enabled() -> bool {
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
/// stack coordinate that [`super::interp::emu_run_region`] would establish (`state.sp - lifted_depth - 328`
/// aligned to `16n + 8`) without entering the interpreter loop.
#[cfg_attr(target_arch = "x86_64", allow(dead_code))]
#[inline]
pub(super) unsafe fn dll_stack_pos_va8(c: &crate::callctx::CallCtx) -> u64 {
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
pub(super) unsafe fn dll_stack_pos_call(cpu: &mut Cpu) -> Option<(u64, u128)> {
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
pub(super) unsafe fn dll_stack_pos_tail(cpu: &mut Cpu) -> Option<(u64, u128)> {
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
/// Registered game integer replacement (preserves `xmm0` unless set explicitly).
pub const DF_GAME_REPL: u32 = 4;
/// Registered `FLOAT_RETURN` function (result in `q0`; `rax` preserved for replacements).
pub const DF_FLOAT_RET: u32 = 8;
/// `AW2_TRACE_EAT=1` logging for non-replacement host calls.
pub const DF_TRACE: u32 = 16;
/// `host_fn` is an integer-only win32-rt builtin (at most 8 args, no XMM or stack
/// args, never calls back into guest code): `call_va8` calls it directly.
pub const DF_PLAIN: u32 = 32;
/// `AW2_JOB_MARK` target: the bridges record the call as a `jobprof::K_MARK` span.
pub const DF_MARK: u32 = 64;
/// `host_fn` is a compiled game replacement and the call needs no other ABI fixup (no
/// `DF_FLOAT_RET` / `DF_MARK` / `DF_TRACE`; see [`LEAN_FLAGS`]). `call_va8` and
/// `simd_ops::call_va8_xmm` enter it with a plain AAPCS call (`lean_call`) instead of
/// `host_call_asm`, keeping the `CallCtx` protocol.
pub const DF_LEAN: u32 = 128;
/// The complete flags of a [`DF_LEAN`] target as returned by [`resolve_dispatch`].
pub const LEAN_FLAGS: u32 = DF_VALID | DF_HOST | DF_GAME_REPL | DF_LEAN;
/// `AW2_SHADOW`-selected [`DF_LEAN`] target: `call_va8` / `simd_ops::call_va8_xmm` miss
/// their `LEAN_FLAGS` fast path and the bridges hand the call to `shadow_lean_va8` /
/// `shadow_lean_xmm`, which may start a [`crate::shadow`] probe before the real call.
pub const DF_SHADOW: u32 = 256;
/// Target is a mapped DLL's import stub `jmp qword ptr [rip+disp32]` whose slot lies in the
/// image's IAT directory (`dll_native::iat_stub_slot`); the memoized `host_fn` is the slot
/// address. Only [`resolve_dispatch_memo`] / [`resolve_dispatch_static`] return it;
/// [`resolve_dispatch_in`] replaces it by the slot target's resolution at every call.
pub const DF_IAT: u32 = 512;
/// Leaf host builtin (`DF_PLAIN`) that takes/returns XMM0..3 (`q0..q3` / `q0`), such as
/// pure floating-point math (`sinf`/`cosf`/`powf`/...) or XMM-using Win32/CRT leaves.
pub const DF_PLAIN_XMM: u32 = 1024;
/// Leaf host builtin (`DF_PLAIN`) that reads stack arguments 9..16 (`[rsp+0x40..0x80]`)
/// in addition to register/stack args 1..8. Eligible for widened `xfer_plain` (not 8-arg
/// in-block `plain_call` or `table` plain-host caching).
pub const DF_PLAIN_STACK: u32 = 2048;

/// ABI fixup flags of a call target (independent of which host fn implements it).
#[cold]
#[inline(never)]
fn dispatch_flags(target_va: u64) -> u32 {
    static TRACE_EAT: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    let trace_eat = *TRACE_EAT.get_or_init(|| {
        std::env::var("AW2_TRACE_EAT").is_ok_and(|s| s == "1")
    });
    // Per-game ABI fixups registered via `crate::hooks`: consulted only while the primary-exe
    // replacement registry is active so other executables occupying the same VAs are unaffected.
    let aw2 = crate::exe::aw2_registry_active();
    let is_game_repl = aw2
        && crate::hooks::in_game_repl_window(target_va)
        && !(crate::hooks::in_exe_iat_thunk_window(target_va)
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
    if f == DF_VALID | DF_HOST {
        if is_plain_builtin(h) || crate::hooks::is_extra_plain_native(h) {
            f |= DF_PLAIN;
        } else if is_plain_xmm_builtin(h) {
            f |= DF_PLAIN | DF_PLAIN_XMM;
        }
        #[cfg(test)]
        {
            let extra = test_extra_plain_flags(h);
            if extra != 0 {
                f |= extra;
            }
        }
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

/// Whether `h` is a registered replacement function (collected via `hooks::collect_lean_targets`).
///
/// Those are all rustc-compiled Rust (no `naked`/`global_asm` functions are registered), so
/// they keep the AAPCS64 callee-saved registers. `host_call_asm`'s x19..x28 fence exists
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

/// Hot integer-only leaf builtins safe to call without the Win64 bridge (see [`DF_PLAIN`]):
/// `<= 8` args, no XMM args/ret, never calls back into guest code, never raises SEH.
fn is_plain_builtin(h: usize) -> bool {
    if h == 0 {
        return false;
    }
    static SORTED: std::sync::OnceLock<Box<[usize]>> = std::sync::OnceLock::new();
    let table = SORTED.get_or_init(|| {
        let mut v = win32_rt::plain_builtin_fns().to_vec();
        // Additional verified pure-host leaf exports (never call guest code, <= 8 int args).
        const EXTRA_LEAF_EXPORTS: &[(&[&str], &[&str])] = &[
            (
                &["xinput1_4.dll", "xinput1_3.dll", "xinput9_1_0.dll"],
                &[
                    "XInputGetState",
                    "XInputGetStateEx",
                    "XInputSetState",
                    "XInputGetCapabilities",
                    "XInputEnable",
                    "XInputGetBatteryInformation",
                ],
            ),
            (
                &["ucrtbase.dll", "api-ms-win-crt-stdio-l1-1-0.dll", "msvcrt.dll"],
                &[
                    "__acrt_iob_func",
                    "__stdio_common_vsprintf",
                    "__stdio_common_vswprintf",
                    "__stdio_common_vsprintf_s",
                    "__stdio_common_vswprintf_s",
                    "__stdio_common_vsnprintf_s",
                    "__stdio_common_vsnwprintf_s",
                    "__stdio_common_vsscanf",
                    "__stdio_common_vswscanf",
                ],
            ),
            (
                &["kernel32.dll", "kernelbase.dll"],
                &[
                    "HeapAlloc",
                    "HeapFree",
                    "HeapReAlloc",
                    "HeapSize",
                    "HeapValidate",
                    "HeapCompact",
                    "HeapLock",
                    "HeapUnlock",
                    "GetProcessHeap",
                    "GetSystemTimeAsFileTime",
                    "GetSystemTimePreciseAsFileTime",
                    "GetSystemTime",
                    "GetLocalTime",
                    "FileTimeToSystemTime",
                    "SystemTimeToFileTime",
                    "FileTimeToLocalFileTime",
                    "LocalFileTimeToFileTime",
                    "MultiByteToWideChar",
                    "WideCharToMultiByte",
                    "lstrlenA",
                    "lstrlenW",
                    "lstrcpyA",
                    "lstrcpyW",
                    "lstrcpynW",
                    "GetACP",
                    "GetOEMCP",
                    "IsValidCodePage",
                    "EncodePointer",
                    "DecodePointer",
                    "FlsGetValue",
                    "FlsSetValue",
                    "VirtualQuery",
                    "VirtualQueryEx",
                    "GetCurrentProcessId",
                    "QueryPerformanceFrequency",
                    "QueryInterruptTime",
                    "QueryUnbiasedInterruptTime",
                ],
            ),
        ];
        for &(dlls, names) in EXTRA_LEAF_EXPORTS {
            for &dll in dlls {
                for &name in names {
                    if let Some(p) = win32_rt::lookup(dll, name) {
                        v.push(p as usize);
                    }
                }
            }
        }
        v.retain(|&p| p != 0);
        v.sort_unstable();
        v.dedup();
        v.into_boxed_slice()
    });
    table.binary_search(&h).is_ok()
}

/// Pure floating-point leaf builtins (`DF_PLAIN | DF_PLAIN_XMM`): `<= 8` GPR args + `xmm0..3`,
/// returns `(rax, xmm0)`, never calls guest code or raises SEH.
fn is_plain_xmm_builtin(h: usize) -> bool {
    if h == 0 {
        return false;
    }
    static SORTED: std::sync::OnceLock<Box<[usize]>> = std::sync::OnceLock::new();
    let table = SORTED.get_or_init(|| {
        let mut v = win32_rt::pure_float_math_fns().to_vec();
        v.retain(|&p| p != 0);
        v.sort_unstable();
        v.dedup();
        v.into_boxed_slice()
    });
    table.binary_search(&h).is_ok()
}

#[cfg(test)]
static TEST_EXTRA_DF_FLAGS: std::sync::OnceLock<
    std::sync::Mutex<std::collections::HashMap<usize, u32>>,
> = std::sync::OnceLock::new();

#[cfg(test)]
fn test_extra_plain_flags(h: usize) -> u32 {
    TEST_EXTRA_DF_FLAGS
        .get()
        .and_then(|m| m.lock().ok().and_then(|g| g.get(&h).copied()))
        .unwrap_or(0)
}

/// Test-only: attach extra `DF_PLAIN*` flags (`DF_PLAIN`, `DF_PLAIN_XMM`, `DF_PLAIN_STACK`,
/// `DF_FLOAT_RET`) to a synthetic host function pointer and bump `va_gen` so `resolve_dispatch`
/// re-evaluates it. Pass `0` to clear.
#[cfg(test)]
pub(crate) fn test_set_extra_plain_flags(host_fn: usize, extra_flags: u32) {
    let m = TEST_EXTRA_DF_FLAGS.get_or_init(|| std::sync::Mutex::new(std::collections::HashMap::new()));
    let mut g = m.lock().unwrap_or_else(|e| e.into_inner());
    if extra_flags == 0 {
        g.remove(&host_fn);
    } else {
        g.insert(host_fn, extra_flags);
    }
    drop(g);
    crate::exe::invalidate_dispatch();
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
pub(super) fn dll_route_enabled() -> bool {
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
