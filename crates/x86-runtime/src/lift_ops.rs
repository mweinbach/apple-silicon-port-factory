//! x86 semantics of 8- and 16-bit shifts for lifted code.
//!
//! x86 masks a shift count to 5 bits for 8/16/32-bit operands, so `shl r16, cl` with
//! `cl & 31` in 16..31 shifts every bit out (result 0) and `sar r16, cl` fills with the sign.
//! Rust's `u16::wrapping_shl` / `wrapping_shr` mask the count to 4 bits (3 for `u8`), so the
//! plain lowering is wrong for counts >= the operand width. The lifters
//! (`tools/decomp_scale/gen_subsystems.py`, DLL mode included) emit these helpers for narrow
//! shifts by `cl` (and for immediate counts >= the width). Flags are derived from the result
//! as before (ZF/SF); a count of 0 leaves the flags alone, which the lifter already treats
//! as unknown.

/// `shl r16, n` (count masked to 5 bits; >= 16 gives 0).
#[inline(always)]
pub fn shl16(v: u16, n: u32) -> u16 {
    let n = n & 31;
    if n < 16 { v << n } else { 0 }
}

/// `shr r16, n` (count masked to 5 bits; >= 16 gives 0).
#[inline(always)]
pub fn shr16(v: u16, n: u32) -> u16 {
    let n = n & 31;
    if n < 16 { v >> n } else { 0 }
}

/// `sar r16, n` (count masked to 5 bits; >= 16 fills with the sign bit).
#[inline(always)]
pub fn sar16(v: u16, n: u32) -> u16 {
    ((v as i16) >> (n & 31).min(15)) as u16
}

/// `shl r8, n` (count masked to 5 bits; >= 8 gives 0).
#[inline(always)]
pub fn shl8(v: u8, n: u32) -> u8 {
    let n = n & 31;
    if n < 8 { v << n } else { 0 }
}

/// `shr r8, n` (count masked to 5 bits; >= 8 gives 0).
#[inline(always)]
pub fn shr8(v: u8, n: u32) -> u8 {
    let n = n & 31;
    if n < 8 { v >> n } else { 0 }
}

/// `sar r8, n` (count masked to 5 bits; >= 8 fills with the sign bit).
#[inline(always)]
pub fn sar8(v: u8, n: u32) -> u8 {
    ((v as i8) >> (n & 31).min(7)) as u8
}

#[cfg(test)]
std::thread_local! {
    static STRICT_TRAPS: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
}

/// Whether a [`trap`] no vectored handler took returns to the lifted code (which then returns
/// rax as is, the old lowering). `AW2_EMU_INT3=return` (A/B only, the emulator's switch for
/// the same instructions); unit tests too unless the thread called [`strict_traps_for_test`]
/// (the lift fuzzers compare lenient results, as the emulator's `lenient_in_tests`).
fn trap_returns() -> bool {
    #[cfg(test)]
    if !STRICT_TRAPS.with(|c| c.get()) {
        return true;
    }
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| std::env::var("AW2_EMU_INT3").is_ok_and(|v| v.trim() == "return"))
}

/// Unit tests: the production [`trap`] behaviour on this thread.
#[cfg(test)]
pub(crate) fn strict_traps_for_test(on: bool) {
    STRICT_TRAPS.with(|c| c.set(on));
}

/// A lifted `int3` (`STATUS_BREAKPOINT`), `ud2` (`STATUS_ILLEGAL_INSTRUCTION`) or `int 0x29`
/// (`__fastfail`, `STATUS_STACK_BUFFER_OVERRUN`, information[0] = rcx) at guest address `va`,
/// raised as the emulator raises them (emu `Int3 | Ud2`): vectored handlers, then the
/// unhandled path (crash report, the application's unhandled-exception filter, process exit
/// with the code; shadow probe children fault and `_exit`). The usual site is the `int3`
/// after a noreturn call (`__scrt_fastfail`, `V8_Fatal`), which the lifters used to lower to
/// a plain `return rax`.
/// - Not resumable: a handler or filter asking to continue is logged and the process exits.
/// - `__fastfail` skips the vectored handlers (and cannot be resumed), as on Windows.
/// - `rsp`: the lifted frame's stack pointer (the guest stack walk and the crash report's
///   stack scan read it), or 0 for frameless lifted code (a zeroed stand-in is used).
/// Returns only under [`trap_returns`]; the lifted code then returns rax.
#[cold]
#[inline(never)]
pub unsafe fn trap(code: u32, va: u64, rcx: u64, rsp: u64) {
    use win32_rt::unwind as uw;
    let fastfail = code == uw::STATUS_STACK_BUFFER_OVERRUN;
    let pad = [0u64; 1024];
    let mut gpr = [0u64; 16];
    gpr[uw::RCX] = rcx;
    gpr[uw::RSP] = if rsp != 0 { rsp } else { pad.as_ptr() as u64 };
    let info = [rcx];
    let mut rec = uw::guest_exception_record(code, va, if fastfail { &info[..] } else { &[] });
    let mut ctx = uw::guest_context(&gpr, &[0u128; 16], va, 0x202);
    if !fastfail && uw::dispatch_veh_guest(&mut rec, &mut ctx) {
        log::error!("lifted: a vectored handler asked to continue exception {code:#x} at {va:#x}; not resumable in lifted code");
    }
    if trap_returns() {
        return;
    }
    log::error!("lifted: exception {code:#x} at {va:#x} (lifted int3 / ud2 / int 0x29), rcx={rcx:#x}");
    let _ = uw::emu_unhandled_exception(&mut rec, &mut ctx, false);
    core::hint::black_box(&pad);
}

#[cfg(test)]
mod trap_tests {
    use super::*;

    /// A readable stand-in for the trapping instruction (the crash report dumps code bytes
    /// around `rip`).
    static SITE: [u8; 64] = [0xCC; 64];

    /// Lenient by default in unit tests: `trap` returns (the lift fuzzers rely on it).
    #[test]
    fn trap_returns_in_lenient_tests() {
        unsafe { trap(win32_rt::unwind::STATUS_BREAKPOINT, SITE.as_ptr() as u64 + 16, 0, 0) };
    }

    /// Child half of [`trap_exits_with_the_exception_code`]: a no-op unless spawned by it.
    #[test]
    fn trap_child() {
        let Ok(code) = std::env::var("AW2_LIFT_TRAP_CHILD") else { return };
        let code = u32::from_str_radix(code.trim_start_matches("0x"), 16).unwrap();
        strict_traps_for_test(true);
        unsafe { trap(code, SITE.as_ptr() as u64 + 16, 5, 0) };
        panic!("strict trap returned");
    }

    /// Production behaviour: int3 / ud2 / __fastfail do not return; the process exits with the
    /// exception code (as an unhandled exception on Windows), so the low byte is the status.
    #[test]
    fn trap_exits_with_the_exception_code() {
        use win32_rt::unwind::{STATUS_BREAKPOINT, STATUS_ILLEGAL_INSTRUCTION, STATUS_STACK_BUFFER_OVERRUN};
        let exe = std::env::current_exe().unwrap();
        let logs = std::env::temp_dir().join(format!("aw2_lift_trap_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&logs);
        for code in [STATUS_BREAKPOINT, STATUS_ILLEGAL_INSTRUCTION, STATUS_STACK_BUFFER_OVERRUN] {
            let out = std::process::Command::new(&exe)
                .args(["lift_ops::trap_tests::trap_child", "--exact", "--test-threads=1", "--nocapture"])
                .env("AW2_LIFT_TRAP_CHILD", format!("{code:x}"))
                .env("AW2_LOG_DIR", &logs)
                .env_remove("AW2_EMU_INT3")
                .output()
                .unwrap();
            assert_eq!(
                out.status.code(),
                Some((code & 0xff) as i32),
                "code {code:#x}: {:?}\n{}",
                out.status,
                String::from_utf8_lossy(&out.stderr)
            );
        }
        let _ = std::fs::remove_dir_all(&logs);
    }
}

#[cfg(all(test, not(target_arch = "x86_64")))]
mod tests {
    use super::*;

    /// Run `op ax/al, cl` on the emulator: `mov eax, edx ; <op> ; ret` with rdx = value,
    /// rcx = count; the low `bits` of rax.
    fn emu(op: &[u8], v: u64, n: u64, bits: u32) -> u64 {
        let mut code = vec![0x89, 0xD0];
        code.extend_from_slice(op);
        code.push(0xC3);
        let p = Box::leak(code.into_boxed_slice()).as_ptr() as u64;
        let r = unsafe { crate::emu::emu_run_full(p, [n, v, 0, 0, 0, 0, 0, 0], [0; 4], [0; 8]).0 };
        r & ((1u64 << bits) - 1)
    }

    /// Each helper matches the emulator's x86 for every count 0..64 (cl is masked to 5
    /// bits) on values with the top bit set and clear.
    #[test]
    fn narrow_shifts_match_x86_for_all_counts() {
        const SHL16: &[u8] = &[0x66, 0xD3, 0xE0];
        const SHR16: &[u8] = &[0x66, 0xD3, 0xE8];
        const SAR16: &[u8] = &[0x66, 0xD3, 0xF8];
        const SHL8: &[u8] = &[0xD2, 0xE0];
        const SHR8: &[u8] = &[0xD2, 0xE8];
        const SAR8: &[u8] = &[0xD2, 0xF8];
        for v in [0x0001u64, 0x00ff, 0x5a5a, 0x8001, 0xbbc5_5ff0, 0xffff] {
            for n in 0..64u64 {
                let (w, b) = (v as u16, v as u8);
                assert_eq!(shl16(w, n as u32) as u64, emu(SHL16, v, n, 16), "shl16 {v:x} {n}");
                assert_eq!(shr16(w, n as u32) as u64, emu(SHR16, v, n, 16), "shr16 {v:x} {n}");
                assert_eq!(sar16(w, n as u32) as u64, emu(SAR16, v, n, 16), "sar16 {v:x} {n}");
                assert_eq!(shl8(b, n as u32) as u64, emu(SHL8, v, n, 8), "shl8 {v:x} {n}");
                assert_eq!(shr8(b, n as u32) as u64, emu(SHR8, v, n, 8), "shr8 {v:x} {n}");
                assert_eq!(sar8(b, n as u32) as u64, emu(SAR8, v, n, 8), "sar8 {v:x} {n}");
            }
        }
    }

    /// The counts that broke the plain lowering (exe_lift_fuzz, +0x14155e740): 17 on u16,
    /// 9 on u8. `wrapping_shl` would shift by 1.
    #[test]
    fn count_at_or_above_width() {
        assert_eq!(shl16(0x5ff0, 17), 0);
        assert_eq!(shr16(0x5ff0, 17), 0);
        assert_eq!(sar16(0x8000, 17), 0xffff);
        assert_eq!(sar16(0x7fff, 17), 0);
        assert_eq!(shl8(0x81, 9), 0);
        assert_eq!(shr8(0x81, 9), 0);
        assert_eq!(sar8(0x81, 9), 0xff);
        assert_eq!(shl16(0x5ff0, 32 + 3), 0x5ff0u16 << 3);
        assert_ne!(0x5ff0u16.wrapping_shl(17), 0);
    }
}
