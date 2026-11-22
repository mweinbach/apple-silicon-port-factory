//! `v8_libbase.dll` `v8::base::OS::Abort()`: fail loudly instead of continuing.
//!
//! V8's fatal path (`V8_Fatal`, API failures without an embedder handler) prints the
//! message and calls `OS::Abort()`, which ends in an `int3` (`FLAG_hard_abort`, default on)
//! or `raise(SIGABRT)` + `abort()`. On Windows the breakpoint kills the process. The
//! emulator returns from an unhandled `int3` to the native caller instead, so V8 kept
//! running on a broken heap after "unreachable code" (g3 cohtml gate: a 24 GB spin in a
//! page-list walk minutes later, with the first failure's caller lost). The replacement
//! logs the V8 message (the `FailureMessage` on `V8_Fatal`'s frame), a crash report (host
//! backtrace + innermost guest state), a guest backtrace from the caller, and exits with
//! `STATUS_STACK_BUFFER_OVERRUN`, as `__fastfail` would.

use super::DllPatch;

/// `?Abort@OS@base@v8@@SAXXZ` export: `jmp abort_body` (E9 rel32).
const RVA_ABORT_THUNK: usize = 0x2300;
/// `OS::Abort` body (`IsDebuggerPresent` .. `int3`, 0x57 bytes).
const RVA_ABORT: usize = 0x2_57d0;
/// `V8_Fatal`'s `FailureMessage { u64 start = 0xdecade10; char msg[512]; u64 end = 0xdecade11 }`.
const FM_START: u64 = 0xdeca_de10;
const FM_END: u64 = 0xdeca_de11;
const FM_MSG_LEN: usize = 512;

pub(crate) fn patches(_base: usize) -> Vec<DllPatch> {
    vec![
        DllPatch { rva: RVA_ABORT_THUNK, len: 5, fnv: ABORT_THUNK_FNV, target: os_abort as *const () as usize },
        DllPatch { rva: RVA_ABORT, len: 0x57, fnv: 0xaca9_1e59_7ac8_fda3, target: os_abort as *const () as usize },
    ]
}

/// FNV-1a 64 of the 5-byte export thunk (`e9` + rel32 to [`RVA_ABORT`]).
const ABORT_THUNK_FNV: u64 = {
    let rel = (RVA_ABORT as i64 - (RVA_ABORT_THUNK as i64 + 5)) as i32 as u32;
    let b = [0xe9u8, rel as u8, (rel >> 8) as u8, (rel >> 16) as u8, (rel >> 24) as u8];
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    let mut i = 0;
    while i < b.len() {
        h = (h ^ b[i] as u64).wrapping_mul(0x100_0000_01b3);
        i += 1;
    }
    h
};

fn read_u64(addr: u64) -> Option<u64> {
    let mut b = [0u8; 8];
    win32_rt::crash::safe_read(addr, &mut b).then(|| u64::from_le_bytes(b))
}

/// The newest `FailureMessage` text in `[rsp, rsp + 0x1000)` of the guest stack, if any.
fn failure_message(rsp: u64) -> Option<String> {
    if rsp == 0 {
        return None;
    }
    for a in (rsp..rsp + 0x1000).step_by(8) {
        if read_u64(a)? != FM_START || read_u64(a + 8 + FM_MSG_LEN as u64) != Some(FM_END) {
            continue;
        }
        let mut msg = [0u8; FM_MSG_LEN];
        if !win32_rt::crash::safe_read(a + 8, &mut msg) {
            return None;
        }
        let n = msg.iter().position(|&c| c == 0).unwrap_or(FM_MSG_LEN);
        return Some(String::from_utf8_lossy(&msg[..n]).into_owned());
    }
    None
}

/// `v8::base::OS::Abort()` (no arguments, never returns).
pub unsafe extern "win64" fn os_abort() -> u64 {
    // The caller's guest rsp: published by the emulator's native-call path, else the
    // innermost interpreter frame's register file.
    let (incoming, _) = crate::callctx::ctx().incoming_rsp.get();
    let guest = crate::emu::current_guest_state();
    let rsp = if incoming != 0 { incoming } else { guest.map_or(0, |(g, _)| g[4]) };
    let msg = failure_message(rsp);
    log::error!(
        "V8 OS::Abort: {} (guest rsp 0x{rsp:x}); V8 hit a fatal error, terminating (on Windows this is an unhandled int3)",
        msg.as_deref().unwrap_or("<no FailureMessage on the guest stack>")
    );
    win32_rt::crash::report_here("V8 OS::Abort");
    if let Some((g, rip)) = guest {
        win32_rt::crash::aw2_debug_backtrace(rip, g[4], g[5]);
    }
    win32_rt::process::exit_process(0xC000_0409)
}
