//! Difftest for whole-DLL lifts (`dll_lift::<module>`).
//!
//! 1. Structure: the installed image passed its gate and every entry VA dispatches to its
//!    lifted function (or to a native that was registered before it).
//! 2. Fuzz: every installed entry is run through the original guest code (emulator) and
//!    through its lifted body on identical pseudo-random inputs, comparing `rax`, `xmm0`
//!    (for [`XMM_RET`] bodies), the argument arena and the image's writable sections.
//!    Arguments mix null, small integers and arena pointers; arena words are null, small
//!    integers or forward-only pointers (pointer chases terminate) and the arena tail is
//!    zero. Callees of non-leaf entries go through the normal VA dispatch on both sides (a
//!    lifted callee, a native, or guest code), so each entry's own body is what is compared.
//!    Each trial runs guest, lifted, guest: if the two guest runs differ (heap addresses,
//!    time, state outside the image) the trial is "guest-nondeterministic" and not compared;
//!    a time-dependent lifted result is bracketed by the two guest runs. Non-leaves stop
//!    after [`NONLEAF_ENOUGH`] equal trials or [`NONLEAF_MAX_TIMEOUTS`] double timeouts.
//!    The guest runs take the entry's own VA out of the dispatch table (see
//!    [`guest_run`]): the wrapper reaches it with a guest `call`, which would
//!    otherwise dispatch to the very lifted body under test.
//!
//! Random inputs can fault or run away, so each case runs in a forked child (guest runs,
//! restore, lifted run; one result record each over a pipe) with signal handlers that
//! report the signal and fault address, and a 5 s timeout. A case passes when
//! both sides return equal results, or both fault, or both time out. "Slow guest": the
//! emulator runs a long loop many times slower than the lifted body, so a guest timeout where
//! the lifted run crashed (inside the emulator's address range) or returned is re-run
//! guest-alone in a fresh child with [`SLOW_GUEST_X`] x the budget and classified by its new
//! outcome (both crashed, compared with the lifted result, or still a timeout: a mismatch);
//! the re-runs are counted and listed in the summary. A guest body calling a lifted callee in
//! a hot loop leaves the emulator once per call (renoircore +0x1558c0 calling the lifted
//! pdata-less leaf +0x14d2e0). "Slow
//! lifted", the converse: a lifted body that calls a guest function in a hot loop enters the
//! emulator once per call (`emu_run_full` setup), where the guest run stays inside one
//! emulator run (renoircore +0x160c60: ~10^8 calls of +0x14e700 before that leaf lifted),
//! so a lifted timeout
//! after a returning guest run is re-run lifted-alone with [`SLOW_GUEST_X`] x the budget and
//! classified by its new outcome (still a timeout: a mismatch); counted and listed.
//! "Fresh-child guest re-run": a guest fault against a lifted return is repeated guest-alone
//! in a fresh child from the same starting state and that outcome is classified (a fault
//! again: a mismatch); counted and listed with the first fault's site. g14c's focus sweep had
//! 4 such first-trial guest faults (v8 +0x5f7c10, a `movzx eax, byte [rcx+0x1c]; ret` leaf,
//! and 3 more), 3 of them at the run thread's lstack arena end, none reproducible alone.
//! "Guest returned or timed
//! out, lifted faulted at an address outside [0x10000, 2^47)" is counted as inconclusive: the emulator's
//! `read_mem`/`write_mem` (emu/mod.rs) ignore accesses outside that range (reads give 0,
//! writes are dropped) where native code faults, as on Windows (null page, non-canonical
//! address). A trial where the guest `rax` is the lifted `rax` zero-extended from 8 or 16
//! bits (the emulator's `abi_gpr` narrowing after a partial write, see [`abi_narrow`]) is
//! "abi-narrowed": reported separately with every function listed, never counted as equal.
//! More classes are listed per function and never counted as equal or as a mismatch:
//! - "non-code call": both the guest and the lifted run called a non-code target. Each child
//!   registers every distinct non-zero arena word and argument that is neither inside a
//!   mapped PE image nor already dispatched (small integers, arena pointers: what a call
//!   through an arena "vtable" slot can hit) to [`noncode_trap`] in the VA dispatch table, so
//!   the emulator's call/jmp resolution and the lifted `call_va*` bridge both reach the trap,
//!   which ends that run (natively, under DEP, the call faults; unregistered, the emulator
//!   would execute the data or stop at `rip == 0`). Only one side trapping is a mismatch.
//!   (A call to address 0 cannot be registered; it stays as before.)
//! - "rax_stack": the two `rax` (and/or `xmm0`) values each point into their own run's stack at
//!   or below the entry `rsp` (the guest emulator's stack, probed with `mov rax, rsp; ret`, vs.
//!   the native stack holding the `LiftedFrame`), everything else equal: the address of a dead
//!   local.
//! - "gs_residue": `rax` of each run is the image's `__security_cookie` (load config
//!   `SecurityCookie`, read at compare time, [`security_cookie`]) xor an address in that run's own
//!   stack, everything else equal: a `/GS` function whose last `rax` write is the cookie
//!   check's `xor rcx, rsp` copy (a void function returning the check residue). Equal up to
//!   the frame address.
//! - "nan_sign": `rax` / `xmm0` / memory differ only by a generated default NaN, x86 `0xFFC00000` /
//!   `0xFFF8000000000000` on the guest side vs. AArch64 `0x7FC00000` / `0x7FF8000000000000`
//!   on the lifted side, everything else equal (`rax` under the caller liveness mask, see
//!   "ret_dead"). A known gap of the lift (its float ops run on AArch64, whose generated NaN
//!   is positive), like C++ EH on aarch64.
//! - "null_exit (harness)": the guest run ended at rip 0 without returning (a tail `jmp`
//!   through a zero word; detected by [`null_exit_stub`]). The emulator ends the whole
//!   top-level run there, the lifted tail `call_va8(0)` returns 0 from that function only;
//!   natively both fault. (A `call` to 0 returns 0 at once on both sides.)
//! - "ret_dead": only `rax` and/or `xmm0` differ, and only in what no x86 caller reads: the
//!   function's row in `db/ret_liveness.tsv` / `db/ret_liveness_dll.tsv` (`fnsig retlive`,
//!   the tables `AW2_SHADOW rets=` uses; [`ret_masks`], [`xmm_dead`]) gives the live rax bits
//!   (0 for a dead return) and whether xmm0 is read. Exported C++ functions get their row
//!   from the declared (mangled) return type: `void`, an integer width, a pointer, `float`.
//!   No row: every bit is live.
//! - "mem_stack": only arena qwords differ, each an address in its own run's stack window
//!   (a dead local's address stored to memory; checked per qword in the child).
//! - "frame_overread (harness)": as "mem_stack", and the other differing arena bytes are ONE
//!   contiguous copy (a single byte shift for all of them) out of the function's caller-side
//!   frame above its locals: its return slot `[entry rsp]` (guest: the wrapper stub's return
//!   address; a lifted frame's slot is never written, 0) and caller slots past the lifted
//!   argument window (args >= 24: guest = the stub's copy of the window argument; a lifted
//!   frame copies only its `A` incoming qwords, the rest read 0). Reached when a fuzzed
//!   length makes x86 code copy past its locals (V8 `Assembler::emit_operand` & co. with a
//!   random `Operand` length: `[rsp+0x22]`, len bytes); natively those bytes are whatever the
//!   caller has there. Checked byte by byte in the child ([`frame_image`],
//!   [`overread_shifts`]). `AW2_DLL_DIFF_POISON=1` fills window args >= 24 with slot-tagged
//!   bytes so a focus run's arena dumps show which slot each differing byte came from.
//! - "thread_fault": a thread that a run started (`Thread::Start`, ...) faulted. The child's
//!   handler sees the faulting thread is not the harness thread; the thread usually reads
//!   arena state the harness has restored for the next run, so neither side is to blame.
//! - "null_call": both runs made a call, jump or return to address 0 (a null function
//!   pointer in the arena; reported by the emulator through `emu::NULL_CALL_HOOK`, one
//!   record per run written at once, so a run that dies afterwards still has it). Natively
//!   both fault there; the emulator gives `rax = 0` and goes on (or ends that run), and what
//!   follows reads state neither side defines (e.g. a callee's uninitialised stack result).
//!   Only one side doing so stays a mismatch.
//! - "stack_top_overrun (harness)": the lifted run faulted within a page past the end of its
//!   frame arena (`lstack::arena_end`): a fuzzed length / index walked a frame's accesses up
//!   off all the frames (renoircore +0xb6b10, an inflate-style bit reader copying `bits / 8`
//!   bytes to `[rsp+0x30..]`). Natively that is the thread stack's top, a fault; the guest
//!   emulator's stack is a host heap block whose top does not fault, so the guest run goes on
//!   over host memory (there: returns through a smashed slot to 0). Not compared.
//! - "stack_oob (harness)": the guest run faulted at an address outside its emulator stack
//!   block but within 16 GB of its top, and the lifted run returned or exited without a
//!   fault: a stack-relative access at a fuzzed index (v8 +0x396d60: `mov edx, [rsp+rcx*4]`
//!   with `rcx = movsxd edx`, a 4-entry table). Both sides access their own frame + the same
//!   offset; what is mapped there depends on each side's layout (the lifted frame arena vs the
//!   emulator stack), natively on what lies beyond the thread stack. Not compared.
//! - "unseeded_frame_read (harness)": the guest run crashed (fault / died), the lifted run
//!   returned, and the guest run repeated alone in a fresh child through the zero-padded stub
//!   ([`stub_variant`]`(true)`: its frame holds 0 in the [`ZERO_PAD`] bytes from the callee's
//!   `[rsp+0x100]` up, where the default stub has its uninitialised frame tail, its return
//!   address = the emulator sentinel and `emu_run_full`'s argument area) returned EXACTLY the
//!   lifted outcome (rax, xmm0, arena and data digests). The function read caller-frame bytes
//!   above the fuzzed window, which the harness cannot seed alike (the lifted side reads its
//!   host stack there, 0 in a fresh `LiftedFrame` window tail); natively they are the real
//!   caller's. g21 cohtml +0x133c30 trial 9: a 3x3 cofactor helper indexes a 3-row dword
//!   table in its frame with a fuzzed row 0x1b, reads `[entry rsp+0x10c..0x118)` and uses the
//!   dwords as indices (guest: index 0xDEADBEFB, fault at m + 4 * index = 0x5a37b07422c;
//!   lifted: 0, returns -0.0f).
//!   Only equality classifies; any other re-run outcome stays a MISMATCH. Listed, never equal.
//!   In precise mode the slow-guest re-run's budget is [`PRECISE_SLOW_X`] x larger
//!   ("slow_guest_precise": the interpreter is too slow for 4 x 5 s, cohtml +0x10c880).
//! - C++ throws. Both runs execute under a host `catch (...)` boundary
//!   (`crate::eh_rt::host_try::catch_cxx`): an MSVC C++ exception (`0xE06D7363`) leaving the
//!   function under test is caught there after the unwinding of every frame in between (the
//!   function's own destructors / cleanup included), and the run reports
//!   `[THROW_TAG, ThrowInfo, object digest, state digest]`: the object bytes (size from the
//!   first CatchableType, qwords that are an address in either run's stack masked) read in the
//!   dispatch phase, and the arena + writable-sections digest after the landing.
//!   - "threw_equal": both runs threw, same ThrowInfo, same object, same state after the
//!     unwinding. Listed, never counted equal.
//!   - "threw_mismatch": both runs threw and something differs. A mismatch (and listed).
//!   - "threw_stack_residue": both threw, same ThrowInfo and state; the objects differ only
//!     in qword high dwords that are each the high half of an address in that run's own
//!     stacks ([`THROWX_TAG`]): stale stack bytes in uninitialized padding (msvcp140
//!     +0x12ed0 `_Throw_Cpp_error`: the 4 bytes after `error_code::_Myval` in the
//!     `system_error` built in the thrower's frame). Listed, never counted equal.
//!   - "threw_mem_stack": both threw, same ThrowInfo and object, same writable-section
//!     digest ([`THROWX_TAG`] word 2); the arena after the unwinding differs only in qwords
//!     that are each an address in that run's own stacks (`MEMSTACK_ONLY`, as "mem_stack";
//!     v8 +0x4d35e0). Listed, never counted equal.
//!   - "threw_unrecorded (harness)": the dispatch stopped at a frame without an EH record
//!     before the boundary (a Rust API calling guest code; `CXX_THROW_HOOK`, the unhandled
//!     path) on both sides, same ThrowInfo. Not compared past the throw.
//!   A throw on one side only stays a mismatch.
//! - "seh_unraised (harness)": the lifted run raised STATUS_ACCESS_VIOLATION for a guarded
//!   access (`seh_guards` fns, `lift::eh::EhFrame::access_violation`, [`SEHF_TAG`] with the
//!   address) and the guest run raised none: the guest faulted unhandled at the same address
//!   (an unmapped address inside [0x10000, 2^47): the emulator cannot dispatch it, precise
//!   mode or not), or, without precise guest faults and with the guarded address outside
//!   [0x10000, 2^47) (the emulator read 0 there / dropped the access instead of faulting,
//!   e.g. vftable 0 - 8 in vcruntime140 +0x65f0 / +0x6650 / +0x67c0), the guest went on to a
//!   null-page fault / died / returned / timed out and a guest-alone re-run of the trial with
//!   precise faults (`CHILD_FORCE_PRECISE`) raised at the same address and ended with the
//!   lifted run's exact outcome (result or throw, arena and data digests): "seh_unraised
//!   (precise-confirmed)", counted and listed separately. A refuted one (that re-run raised
//!   elsewhere / not at all, or ended differently) is a MISMATCH when the lifted run
//!   survived (it may have landed in a handler from a wrongly computed address), else it is
//!   classified as usual (e.g. both crashed). Natively both raise it.
//!   Listed with the address, never equal.
//!
//! Precise guest faults (`AW2_DLL_DIFF_PRECISE=1`, [`precise_faults_env`], default off; g18
//! seh_guard_plan.md): the children's guest runs dispatch an access violation at a guest
//! instruction (null page, non-canonical) through the frame-based handlers
//! (`emu::set_precise_faults`), so a guest `__except` lands as natively. Each function then
//! gets a row `<tag> difftest seh\t<name>\t+0x<off>\tlanded_both=.. fault_both_unhandled=..
//! mismatch=.. no_fault_injected=.. seh_unraised=..` (per trial: both raised and both
//! survived with no MISMATCH; both ended in a fault; a one-sided raise or a MISMATCH with a
//! raise; no raise; seh_unraised above), and the summary lists functions with a seh mismatch
//! and the coverage gaps (0 landed_both: no trial raised and survived an access violation on
//! both sides; a gap, not a pass). Focus-only knobs: `AW2_DLL_DIFF_SET=<i>=<hex>,<i>+<off>=<hex>,..`
//! ([`arg_overrides`]: targeted argument registers or arena fields, e.g. a null-page pointer) and
//! `AW2_DLL_DIFF_SLOW_X=<n>` ([`slow_x`]: the slow re-runs' budget multiplier).
//!
//! Every other one-sided outcome (a lifted fault elsewhere, a
//! guest-only fault, a one-sided timeout) is a mismatch. Leaves without a single conclusive
//! (both returned, equal) trial are reported as unverified, with their share per image. If
//! the guest side faults, the lifted side is rerun alone in a second child.
//!
//! Argument shapes rotate per trial so every leaf gets conclusive cases: all arena pointers;
//! mixed null / small integer / pointer; pointer + small integers (buffer, length, flags);
//! pointer pair + small integer (src, dst, count). Exports with a documented C signature
//! ([`signature`]: memcmp, memchr, strchr, ...) get typed arguments every other trial:
//! unaligned buffers, lengths inside the arena, equal prefixes, bytes present in the buffer.
//! Stack arguments 9..31 are fuzzed too (same shapes, own RNG stream, see [`stack_window`])
//! and both sides see the same values: the lifted run gets them as the published incoming
//! window (`emu::set_incoming_rsp`, what a real caller's `read_s5_call` publishes, so a
//! `LiftedFrame<N, F, A>` copies slots 9..A-1 and a tail forwarder passes the window on),
//! the guest run as `emu_run_full`'s stack arguments 9..16 plus `hi_args` 17.., which
//! [`null_exit_stub`] copies down to the callee's `[rsp+0x28..]`.
//!
//! `AW2_DLL_DIFF_FN=<module>:<hex offset>[:<trial>]` narrows the fuzz to one function (see
//! [`focus_for`]) and logs its inputs and outcomes; combine with
//! `AW2_JIT=0` / `AW2_EMU_FAST=0` to compare the emulator's engines on the guest side.
//! `AW2_DLL_DIFF_WALL_S=<secs>` caps the fuzz time per image; functions not reached are
//! reported as "not fuzzed" (non-leaf timeouts can otherwise take over an hour per image).
//! `AW2_DLL_DIFF_SHARD=<k>/<n>` fuzzes only the dispatched entries with ordinal `% n == k`
//! (run n processes in parallel; the per-function `difftest fn` rows of all logs add up).
//!
//! `exe_lift_fuzz` ([`diff_exe`], `AW2_EXE_DIFF_VAS=<file>`) runs the same fuzz over a list of
//! exe replacements (lifted shard bodies): guest = the exe's x86, memory = the exe's writable
//! sections, rows `exe_lift difftest fn\texe\t+0x<va>`.

use super::{LiftedImage, LEAF, XMM_RET};
use crate::dll_native::DllDiff;

const ARENA: usize = 64 * 1024;
const TAIL_ZERO: usize = 4 * 1024;
const TRIALS: u32 = 16;
const TIMEOUT_MS: u64 = 5000;
/// A guest run that timed out where the lifted run crashed is re-run alone with this many
/// times [`TIMEOUT_MS`] (see "Slow guest" in the module docs).
const SLOW_GUEST_X: u64 = 4;

/// The slow re-runs' budget multiplier: [`SLOW_GUEST_X`], or `AW2_DLL_DIFF_SLOW_X=<n>` in focus
/// mode (`AW2_DLL_DIFF_FN`), to let a long guest loop finish (renoir +0x160c60, g18).
fn slow_x() -> u64 {
    static X: std::sync::OnceLock<u64> = std::sync::OnceLock::new();
    *X.get_or_init(|| {
        std::env::var_os("AW2_DLL_DIFF_FN")
            .or_else(|| std::env::var_os("AW2_DLL_DIFF_VAS"))
            .and_then(|_| std::env::var("AW2_DLL_DIFF_SLOW_X").ok())
            .and_then(|v| v.trim().parse().ok())
            .filter(|&x: &u64| x >= 1)
            .unwrap_or(SLOW_GUEST_X)
    })
}

/// Focus mode only: `AW2_DLL_DIFF_SET=<i>=<hex>[,<i>=<hex>...]` overrides argument register `i`
/// (0 = rcx .. 3 = r9, 4..7 = stack args 5..8) of every trial after the fuzzed values are drawn,
/// e.g. `0=40` puts a null-page pointer in rcx to inject an access violation (seh rows).
/// `<i>+<off>=<hex>` (offset hex) instead writes the qword at `arg_i + off` when that lies in
/// the arena (applied after the register overrides), e.g. `1+28=40`: a struct field that the
/// fn dereferences inside its `__try`.
/// Nested objects (g22, the hot cohtml forwarders `mov rcx,[rcx+8]; mov rax,[rcx]; jmp [rax+N]`):
/// a value `@<hex>` is the arena address + hex; `a+<off>=<v>` writes the qword at arena offset
/// `off`; `m+<va>=<v>` writes the qword at VA `va` when it lies in a writable section of the
/// image under test (restored per run with the sections, so it holds for both sides), e.g.
/// `0=@100,a+108=@200,a+200=@300,a+300=<callee va>` = rcx -> obj whose [+8] -> obj2 whose
/// vtable slot 0 is the callee.
#[derive(Clone, Copy, Debug, PartialEq)]
enum SetAt {
    /// Argument register / stack argument `i`.
    Reg(usize),
    /// The qword at `arg_i + off` (in the arena).
    Field(usize, u64),
    /// The qword at arena offset `off`.
    Arena(u64),
    /// The qword at an absolute VA in a writable image section.
    Mem(u64),
}

/// One `AW2_DLL_DIFF_SET` entry: (target, value, value is arena-relative).
type SetEntry = (SetAt, u64, bool);

fn parse_overrides(v: &str) -> Vec<SetEntry> {
    let hex = |s: &str| u64::from_str_radix(s.trim().trim_start_matches("0x"), 16).ok();
    v.split(',')
        .filter_map(|kv| {
            let (k, x) = kv.trim().split_once('=')?;
            let (val, rel) = match x.trim().strip_prefix('@') {
                Some(r) => (hex(r)?, true),
                None => (hex(x)?, false),
            };
            let at = match k.trim().split_once('+') {
                Some(("a", o)) => SetAt::Arena(hex(o)?),
                Some(("m", o)) => SetAt::Mem(hex(o)?),
                Some((i, o)) => SetAt::Field(i.trim().parse().ok().filter(|&i| i < 8)?, hex(o)?),
                None => SetAt::Reg(k.trim().parse().ok().filter(|&i| i < 8)?),
            };
            Some((at, val, rel))
        })
        .collect()
}

fn arg_overrides() -> &'static [SetEntry] {
    static O: std::sync::OnceLock<Vec<SetEntry>> = std::sync::OnceLock::new();
    O.get_or_init(|| {
        if std::env::var_os("AW2_DLL_DIFF_FN").is_none() {
            return Vec::new();
        }
        parse_overrides(&std::env::var("AW2_DLL_DIFF_SET").unwrap_or_default())
    })
}

/// Applies the register / arena entries of `ov` to one trial's arguments and initial arena image.
fn apply_overrides_with(ov: &[SetEntry], gpr: &mut [u64; 8], init: &mut [u8], arena_addr: usize) {
    let val = |v: u64, rel: bool| if rel { (arena_addr as u64).wrapping_add(v) } else { v };
    for &(at, v, rel) in ov {
        if let SetAt::Reg(i) = at {
            gpr[i] = val(v, rel);
        }
    }
    for &(at, v, rel) in ov {
        let a = match at {
            SetAt::Field(i, off) => gpr[i].wrapping_add(off).wrapping_sub(arena_addr as u64),
            SetAt::Arena(off) => off,
            _ => continue,
        };
        if a.checked_add(8).is_some_and(|end| end <= init.len() as u64) {
            init[a as usize..a as usize + 8].copy_from_slice(&val(v, rel).to_le_bytes());
        }
    }
}

fn apply_overrides(gpr: &mut [u64; 8], init: &mut [u8], arena_addr: usize) {
    apply_overrides_with(arg_overrides(), gpr, init, arena_addr);
}

/// The `m+<va>` entries: writes each qword that lies inside one of the writable sections
/// `secs` ([start, end)). Runs in the child after the sections are restored.
unsafe fn apply_mem_overrides(secs: &[(usize, usize)], arena_addr: usize) {
    for &(at, v, rel) in arg_overrides() {
        let SetAt::Mem(va) = at else { continue };
        let va = va as usize;
        if secs.iter().any(|&(s, e)| va >= s && va.checked_add(8).is_some_and(|x| x <= e)) {
            let v = if rel { (arena_addr as u64).wrapping_add(v) } else { v };
            core::ptr::write_unaligned(va as *mut u64, v);
        }
    }
}
/// A non-leaf stops after this many equal trials (each costs a fork and three runs) ...
const NONLEAF_ENOUGH: u32 = 4;
/// ... or after this many trials where both sides timed out.
const NONLEAF_MAX_TIMEOUTS: u32 = 2;
/// Addresses the emulator's `read_mem`/`write_mem` actually access (outside: read 0 / ignored).
const EMU_VALID: core::ops::Range<u64> = 0x1_0000..0x8000_0000_0000;
/// The emulator returns `rax` through `Cpu::abi_gpr` (emu/mod.rs): if the guest's last write
/// to rax was 8- or 16-bit, the value is zero-extended from that width at the guest->host
/// boundary, while the lifted body returns the x86 register as is (upper bits from earlier
/// writes). `Some(width)` when the guest value is what that narrowing makes of the lifted
/// value. Value-based: the emulator's `last_bits` is not visible from here.
fn abi_narrow(guest: u64, lifted: u64) -> Option<u32> {
    [8u32, 16].into_iter().find(|&w| {
        let m = (1u64 << w) - 1;
        guest & !m == 0 && lifted & m == guest && lifted != guest
    })
}

/// `guest != lifted`, and they are equal once each x86 generated NaN in `guest` (a 32-bit
/// lane `0xFFC00000`, or the word `0xFFF8000000000000`) is the AArch64 one (`0x7FC00000`,
/// `0x7FF8000000000000`) at the same place in `lifted`. Directional: only x86 -> AArch64.
fn nan_sign(guest: u64, lifted: u64) -> bool {
    if guest == lifted {
        return false;
    }
    if guest == 0xFFF8_0000_0000_0000 && lifted == 0x7FF8_0000_0000_0000 {
        return true;
    }
    let (g, l) = ([guest as u32, (guest >> 32) as u32], [lifted as u32, (lifted >> 32) as u32]);
    (0..2).all(|i| g[i] == l[i] || (g[i] == 0xFFC0_0000 && l[i] == 0x7FC0_0000))
}

/// Both results differ, and only by generated NaNs (see [`nan_sign`]): in `rax` / `xmm0`,
/// and/or in memory, i.e. the raw arena / data digests differ but the NaN-canonical ones
/// (`gc` / `lc`, see [`nan_canon_hash`]) are equal.
fn nan_sign_only(g: &[u64; 4], l: &[u64; 4], gc: [u64; 2], lc: [u64; 2]) -> bool {
    g != l
        && (0..2).all(|i| g[i] == l[i] || nan_sign(g[i], l[i]))
        && (g[2..] == l[2..] || (gc != [0, 0] && gc == lc))
}

/// FNV-1a over `bytes` with x86 vs AArch64 default NaN patterns canonicalized: every aligned
/// dword 0xFFC00000 (f32 NaN) or 0xFFF80000 (high half of the f64 NaN) hashes as its
/// sign-cleared value. Only compared with another canonical digest.
unsafe fn nan_canon_hash(p: *const u8, n: usize) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let words = n / 4;
    for i in 0..words {
        let mut w = core::ptr::read_unaligned(p.add(i * 4) as *const u32);
        if w == 0xFFC0_0000 || w == 0xFFF8_0000 {
            w &= 0x7FFF_FFFF;
        }
        for b in w.to_le_bytes() {
            h = (h ^ b as u64).wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    for i in words * 4..n {
        h = (h ^ *p.add(i) as u64).wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

/// Size of either stack a run's locals live on (guest emulator stack: 8 MB per thread; the
/// lifted `lstack` arena / native thread stack holding `LiftedFrame`s).
const STACK_SPAN: u64 = 8 << 20;

/// `v` points into a stack whose entry `rsp` is `top`, at or below it.
fn in_stack(v: u64, top: u64) -> bool {
    top != 0 && v <= top && top - v < STACK_SPAN
}

/// Bytes of the emulator's per-thread guest stack (`emu` `GUEST_STACK_QWORDS`): "stack_oob".
const GUEST_STACK_BYTES: u64 = 8 << 20;

/// `v` is a stack address of the run: the guest emulator stack, or the lifted frame arena
/// (both sides use it: the guest run's callees are dispatched to lifted code too).
fn in_either_stack(v: u64, m: &Meta) -> bool {
    use core::sync::atomic::Ordering::Relaxed;
    in_stack(v, m.guest_rsp) || in_stack(v, m.lifted_sp) || (BIG_LO.load(Relaxed)..BIG_HI.load(Relaxed)).contains(&v)
}

/// `d` is the non-zero upper 32 bits of an address in one of the run's stacks (a stale
/// stack-address half left in uninitialized 4-byte struct padding after a 32-bit tag write, as
/// in `throw_note` / `threw_stack_residue`).
fn in_either_stack_hi32(d: u32, m: &Meta) -> bool {
    use core::sync::atomic::Ordering::Relaxed;
    if d == 0 {
        return false;
    }
    let d = d as u64;
    let in_top = |top: u64| top != 0 && ((top.saturating_sub(STACK_SPAN - 1) >> 32)..=(top >> 32)).contains(&d);
    let (bg_lo, bg_hi) = (BIG_LO.load(Relaxed), BIG_HI.load(Relaxed));
    let in_bg = bg_lo < bg_hi && ((bg_lo >> 32)..=((bg_hi - 1) >> 32)).contains(&d);
    in_top(m.guest_rsp) || in_top(m.lifted_sp) || in_bg
}

/// `gw != lw`, their low 32 bits match, and each side's upper 32 bits is the high half of an
/// address in that run's stacks (`{ i32 tag; /* 4B pad */ i64 val }` copied via `movups`).
fn is_stack_pad_qword(gw: u64, lw: u64, m: &Meta) -> bool {
    gw != lw
        && (gw as u32) == (lw as u32)
        && in_either_stack_hi32((gw >> 32) as u32, m)
        && in_either_stack_hi32((lw >> 32) as u32, m)
}

fn is_stack_or_pad(gw: u64, lw: u64, m: &Meta) -> bool {
    (in_either_stack(gw, m) && in_either_stack(lw, m)) || is_stack_pad_qword(gw, lw, m)
}

/// The harness thread's big-frame arena (`simd_ops::big_frame_arena`, set before the forks):
/// lifted frames over 2 KB live there, so it is stack of the lifted run too (v8 +0x8ed0a0,
/// a 0x1078-byte frame: "gs_residue" over an address in it).
static BIG_LO: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);
static BIG_HI: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);

/// The current `__security_cookie` of the PE image mapped at `base` (load config directory,
/// `SecurityCookie` VA at +0x58), or 0 when the image has none ("gs_residue").
unsafe fn security_cookie(base: usize) -> u64 {
    let rd32 = |a: usize| core::ptr::read_unaligned(a as *const u32) as usize;
    if base == 0 || core::ptr::read_unaligned(base as *const u16) != 0x5A4D {
        return 0;
    }
    let nt = base + rd32(base + 0x3C);
    // PE32+ optional header at nt+0x18; data directory 10 (load config) at +0x70 + 10*8.
    if rd32(nt) != 0x4550 || core::ptr::read_unaligned((nt + 0x18) as *const u16) != 0x20B {
        return 0;
    }
    let (rva, size) = (rd32(nt + 0x18 + 0xC0), rd32(nt + 0x18 + 0xC4));
    if rva == 0 || size < 0x60 || rd32(base + rva) < 0x60 {
        return 0;
    }
    let va = core::ptr::read_unaligned((base + rva + 0x58) as *const u64) as usize;
    if va == 0 {
        return 0;
    }
    core::ptr::read_unaligned(va as *const u64)
}

/// First word of the record of a guest run that ended at rip 0 without returning
/// ([`Outcome::NullExit`]): `[NULLEXIT_TAG, 0, 0, 0]`.
const NULLEXIT_TAG: u64 = 0x4E55_4C4C_4E55_4C4C;

/// Guest-side wrapper of the function under test (see [`Outcome::NullExit`]):
/// `sub rsp,0x108`, copy stack args 5..31 down so the callee sees the caller's `[rsp+0x28..]`,
/// `mov r11,[rip+target]`, `call r11`, `mov byte [rip+done],1`, `add rsp,0x108`, `ret`.
/// The emulator ends a whole top-level run when guest code jumps to rip 0 (a tail `jmp`
/// through a zero word; a `call` to 0 returns 0 at once, as the lifted `call_va8(0)` does);
/// `done` then stays 0. The code never changes (the target is a data
/// word on its own page), so translated blocks stay valid. Returns (code address, data
/// address: `[target, done]`).
fn null_exit_stub() -> (u64, *mut u64) {
    let (c, d, _) = null_exit_stub_full();
    (c, d)
}

/// The guest-side return address of the function under test: the instruction after the
/// [`null_exit_stub`]'s `call r11` (what the function finds in its return slot `[rsp]` at
/// entry; a lifted frame's return slot holds 0, see "frame_overread" in the module docs).
fn stub_ret() -> u64 {
    null_exit_stub_full().2
}

/// Language handler of the lifted run's stand-in for the stub's frame: the stub has none,
/// an exception passes it (ExceptionContinueSearch, both phases).
unsafe extern "win64" fn stub_frame_handler(
    _rec: *mut win32_rt::unwind::ExceptionRecord,
    _frame: u64,
    _ctx: *mut win32_rt::unwind::Context,
    _dc: *mut win32_rt::unwind::DispatcherContext,
) -> u32 {
    1
}

fn null_exit_stub_full() -> (u64, *mut u64, u64) {
    stub_variant(0)
}

/// Bytes past the fuzzed window (the callee's `[rsp+0x100..]`) that the zero-padded stub
/// variants ([`stub_variant`]`(1 | 2)`) zero-fill in their own frame: what the function under
/// test then reads above its argument window is 0 there instead of the default stub's
/// uninitialised frame tail, return address (the emulator sentinel) and `emu_run_full`'s
/// argument area ("unseeded_frame_read" in the module docs).
const ZERO_PAD: u32 = 0x800;

/// [`null_exit_stub_full`] (`mode == 0`) or its confirmation variants:
/// - `mode == 1`: frame `0x108 + ZERO_PAD`, copies all `WIN_ARGS` (5..=31) and zeroes
///   `[0x100 .. frame)` (`[callee_entry_rsp + 0x108 ..)`).
/// - `mode == 2`: frame `0x108 + ZERO_PAD`, copies only `5..LIFTED_ARGS_MIN` (5..=23, matching
///   every default `A = 24` `LiftedFrame`) and zeroes unwritten home space `[0x00..0x20)` plus
///   `[0xb8 .. frame)` (`args 24..=31` + `ZERO_PAD`, matching a fresh `LiftedFrame`).
/// - `mode == 4`: frame `0x108 + 0x5_5000` (`rsp % 16 == 8`), copies all `WIN_ARGS` (5..=31)
///   with no zeroing change: together with `lpad`'s `0x540` `lifted_depth` shift in `guest_run`,
///   shifts `guest_entry_rsp` by `-0x5_5540` (and lifted callees by `-0x540`, flipping bit 6 in
///   byte 0) to detect guest trials whose output depends on their own stack address (e.g.
///   out-of-bounds byte reads into a `/GS` `__security_cookie ^ rsp` slot).
fn stub_variant(mode: u8) -> (u64, *mut u64, u64) {
    static STUB: std::sync::OnceLock<(usize, usize, usize)> = std::sync::OnceLock::new();
    static ZSTUB: std::sync::OnceLock<(usize, usize, usize)> = std::sync::OnceLock::new();
    static ZSTUB24: std::sync::OnceLock<(usize, usize, usize)> = std::sync::OnceLock::new();
    static SSTUB: std::sync::OnceLock<(usize, usize, usize)> = std::sync::OnceLock::new();
    let slot = match mode {
        0 => &STUB,
        1 => &ZSTUB,
        4 => &SSTUB,
        _ => &ZSTUB24,
    };
    let &(c, d, ret) = slot.get_or_init(|| unsafe {
        const PAGE: usize = 0x4000;
        // Outgoing area 0x20 + 27 * 8 = 0xf8 <= FRAME; FRAME % 16 == 8 keeps the callee's
        // entry rsp % 16 == 8.
        let frame: u32 = match mode {
            0 => 0x108,
            4 => 0x108 + 0x5_5000,
            _ => 0x108 + ZERO_PAD,
        };
        let p = match mode {
            0 => map_pinned(PIN_STUB, 2 * PAGE, "the guest stub"),
            1 => map_pinned(PIN_STUB + 2 * PAGE, 2 * PAGE, "the zero-padded guest stub"),
            4 => map_pinned(PIN_STUB + 6 * PAGE, 2 * PAGE, "the shifted-rsp guest stub"),
            _ => map_pinned(PIN_STUB + 4 * PAGE, 2 * PAGE, "the 24-arg zero-padded guest stub"),
        };
        let (c, d) = (p, p + PAGE);
        let mut code: Vec<u8> = vec![0x48, 0x81, 0xEC]; // sub rsp, frame
        code.extend(frame.to_le_bytes());
        let copy_args: u32 = if mode == 2 || mode == 3 {
            (LIFTED_ARGS_MIN - 5) as u32
        } else {
            (WIN_ARGS - 4) as u32
        };
        for i in 0..copy_args {
            code.extend([0x4C, 0x8B, 0x9C, 0x24]); // mov r11, [rsp+frame+0x28+8i]
            code.extend((frame + 0x28 + 8 * i).to_le_bytes());
            code.extend([0x4C, 0x89, 0x9C, 0x24]); // mov [rsp+0x20+8i], r11
            code.extend((0x20 + 8 * i).to_le_bytes());
        }
        if mode == 1 || mode == 2 {
            code.extend([0x45, 0x31, 0xDB]); // xor r11d, r11d
            if mode == 2 {
                let mut home = 0u32;
                while home < 0x20 {
                    code.extend([0x4C, 0x89, 0x9C, 0x24]); // mov [rsp+home], r11
                    code.extend(home.to_le_bytes());
                    home += 8;
                }
            }
            let mut off = 0x20 + 8 * copy_args;
            while off < frame {
                code.extend([0x4C, 0x89, 0x9C, 0x24]); // mov [rsp+off], r11
                code.extend(off.to_le_bytes());
                off += 8;
            }
        }
        let rel = |code: &Vec<u8>, insn_len: usize, target: usize| ((target as i64) - (c + code.len() + insn_len) as i64) as i32;
        let r = rel(&code, 7, d);
        code.extend([0x4C, 0x8B, 0x1D]); // mov r11, [rip+target]
        code.extend(r.to_le_bytes());
        code.extend([0x41, 0xFF, 0xD3]); // call r11
        let ret = c + code.len();
        let r = rel(&code, 7, d + 8);
        code.extend([0xC6, 0x05]); // mov byte [rip+done], 1
        code.extend(r.to_le_bytes());
        code.push(1);
        code.extend([0x48, 0x81, 0xC4]); // add rsp, frame
        code.extend(frame.to_le_bytes());
        code.push(0xC3); // ret
        assert!(code.len() <= PAGE);
        core::ptr::copy_nonoverlapping(code.as_ptr(), c as *mut u8, code.len());
        // RUNTIME_FUNCTION + UNWIND_INFO in the data page (past `[target, done]`): `sub rsp,
        // frame` = UWOP_ALLOC_LARGE (size / 8 in the next slot, u16) at prolog offset 7. A C++
        // exception dispatch then walks from the function under test through the stub to the
        // run's sentinel and on to the harness boundary; without it the stub reads as a leaf
        // (return address at [rsp]).
        let ui = d + 0x100;
        let info: [u8; 8] = [0x01, 7, 2, 0, 7, 0x01, (frame / 8) as u8, ((frame / 8) >> 8) as u8];
        core::ptr::copy_nonoverlapping(info.as_ptr(), ui as *mut u8, info.len());
        let rf = (d + 0x120) as *mut [u32; 3];
        rf.write([0, code.len() as u32, (ui - c) as u32]);
        win32_rt::unwind::RtlAddFunctionTable(rf as *const _, 1, c as u64);
        (c, d, ret)
    });
    (c as u64, d as *mut u64, ret as u64)
}

/// Highest fuzzed argument (stack arguments 9..31; a lifted frame's window is at most 32
/// qwords, `GUEST_ARG_SLOT_MAX`).
const WIN_ARGS: usize = 31;
/// Qwords of a trial's stack window in caller-`rsp` form: `w[k - 1]` is argument `k`
/// (`w[0..4]` the home slots, `w[4..8]` = args 5..8 = `gpr[4..8]`, `w[8..31]` args 9..31,
/// `w[31]` = 0 so `emu_run_full`'s 16 `hi_args` (17..32) are all inside the window).
const WIN: usize = 32;

/// Publish `w` as the caller's outgoing-argument window for the next call on this thread
/// (consumed by the callee's first `take_incoming_rsp`).
unsafe fn publish_window(w: &[u64; WIN]) {
    let p = w.as_ptr() as u64;
    crate::emu::set_incoming_rsp(p, p + (WIN as u64) * 8);
}

/// Run guest `va` through [`null_exit_stub`] with stack window `w` (see [`WIN`]):
/// `Ok(Some((rax, xmm0)))` if it returned, `Ok(None)` if the run ended at rip 0, `Err` if a
/// C++ exception left it (caught by the harness's `catch (...)` boundary, module docs "C++
/// throws"; the emulator entry is transparent so the dispatch continues from the stub's frame
/// to the boundary).
///
/// The stub reaches `va` with a guest `call`, and the emulator resolves call targets through
/// the VA dispatch table, so a registered `va` (the lifted body under test) would run instead
/// of its x86. `va`'s own entry is therefore taken out of the table for the run and put back
/// afterwards: the function under test runs as guest code (including recursive self-calls),
/// its callees dispatch as usual. Children only (the test below restores it in-process).
unsafe fn guest_run(
    va: u64,
    gpr: [u64; 8],
    xmm: [u128; 4],
    w: &[u64; WIN],
) -> Result<Option<(u64, u128)>, crate::eh_rt::host_try::Caught> {
    let zmode = CHILD_ZERO_FRAME.load(core::sync::atomic::Ordering::Relaxed);
    let mut lpad = crate::simd_ops::LiftedFrame::<256, 136>::new();
    let (stub, data) = if zmode != 0 {
        if zmode == 2 {
            crate::emu::zero_guest_stack_unused(0x8000);
        } else if zmode == 4 {
            let _ = crate::simd_ops::StackBuf::frame_base(&mut lpad);
        }
        let (c, d, _) = stub_variant(zmode);
        (c, d)
    } else {
        null_exit_stub()
    };
    core::ptr::write_volatile(data, va);
    core::ptr::write_volatile(data.add(1), 0);
    let stack: [u64; 8] = w[8..16].try_into().unwrap();
    let registered = crate::exe::lookup_va_only(va);
    if registered.is_some() {
        crate::exe::unregister_native_va(va);
    }
    publish_window(w);
    let r = crate::eh_rt::host_try::catch_cxx(throw_note, || crate::emu::emu_run_full_rax(stub, gpr, xmm, stack, 0));
    if let Some(p) = registered {
        crate::exe::register_native_va(va, p);
    }
    let r = r?;
    Ok((core::ptr::read_volatile(data.add(1)) & 0xff != 0).then_some(r))
}

/// Focus mode, child only: write the arena after a run to `<dir>/arena_t<t>_<n>_<g|l>.bin`
/// (`n` counts the child's runs: 0 guest, 1 lifted, 2 guest again).
unsafe fn dump_arena(dir: &str, t: u32, guest: bool, arena_addr: usize) {
    static SEQ: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
    let n = SEQ.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
    let p = std::path::Path::new(dir).join(format!("arena_t{t}_{n}_{}.bin", if guest { "g" } else { "l" }));
    let _ = std::fs::write(p, core::slice::from_raw_parts(arena_addr as *const u8, ARENA));
}

/// Child only: the guest entry rsp (see [`guest_entry_rsp`]).
static CHILD_GUEST_RSP: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);
/// Child only: the arena after the child's first guest run.
static GUEST_ARENA: std::sync::Mutex<Vec<u8>> = std::sync::Mutex::new(Vec::new());
/// `[valid: u64][ARENA bytes]`, MAP_SHARED (mapped by the parent before its first fork): the
/// first guest run of a trial's shared child publishes its arena here, so a lifted run re-run
/// alone in a fresh child (guest threw / faulted / timed out, or lifted crashed after a guest
/// return) is compared against the same guest arena ([`memstack_check`]). The parent clears
/// `valid` before each trial's shared child.
static SHARED_GUEST_ARENA: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);

/// Parent: map [`SHARED_GUEST_ARENA`] once and invalidate it for the next trial.
unsafe fn shared_guest_arena_reset() {
    use core::sync::atomic::Ordering::Relaxed;
    let mut p = SHARED_GUEST_ARENA.load(Relaxed);
    if p == 0 {
        const PROT_RW: i32 = 1 | 2;
        const MAP_SHARED_ANON: i32 = 0x0001 | 0x1000;
        let m = mmap(core::ptr::null_mut(), 8 + ARENA, PROT_RW, MAP_SHARED_ANON, -1, 0) as usize;
        if m == usize::MAX || m == 0 {
            return;
        }
        SHARED_GUEST_ARENA.store(m, Relaxed);
        p = m;
    }
    core::ptr::write_volatile(p as *mut u64, 0);
}

/// Child: publish the first guest run's arena (see [`SHARED_GUEST_ARENA`]).
unsafe fn shared_guest_arena_publish(arena: &[u8]) {
    let p = SHARED_GUEST_ARENA.load(core::sync::atomic::Ordering::Relaxed);
    if p != 0 && core::ptr::read_volatile(p as *const u64) == 0 {
        core::ptr::copy_nonoverlapping(arena.as_ptr(), (p + 8) as *mut u8, ARENA);
        core::ptr::write_volatile(p as *mut u64, 1);
    }
}

/// Child: the trial's first guest arena, from this child or published by the shared child.
fn first_guest_arena() -> Option<Vec<u8>> {
    let g = GUEST_ARENA.lock().unwrap_or_else(|e| e.into_inner());
    if g.len() == ARENA {
        return Some(g.clone());
    }
    let p = SHARED_GUEST_ARENA.load(core::sync::atomic::Ordering::Relaxed);
    unsafe {
        (p != 0 && core::ptr::read_volatile(p as *const u64) == 1)
            .then(|| core::slice::from_raw_parts((p + 8) as *const u8, ARENA).to_vec())
    }
}
/// `CANON_TAG` word 3 of a lifted run: its arena differs from the first guest run's, and
/// every differing (aligned) qword is a stack address on both sides.
const MEMSTACK_ONLY: u64 = 0x5745_4D53;

/// `CANON_TAG` word 3 of a lifted run: as [`MEMSTACK_ONLY`], and at least one differing qword
/// holds bytes the function copied out of its caller-side frame above its locals (see
/// [`frame_image`] / [`overread_shifts`]): the "frame_overread" class.
const OVERREAD_ONLY: u64 = 0x5245_5453;

/// Arguments covered by every lifted frame's incoming window: return slot, home space and
/// args 5..23 (`LiftedFrame` `A` = 24 qwords unless the body reads arg24+, then more).
const LIFTED_ARGS_MIN: usize = 24;
/// Bytes of [`frame_image`]: return slot, 4 home slots, args 5..31 (what [`null_exit_stub`]
/// copies down; beyond it the guest side is the stub's own frame, not modelled).
const FRAME_IMAGE: usize = 8 + 0x20 + 8 * (WIN_ARGS - 4);

/// What the function under test finds above its entry rsp, byte by byte, on each side, for
/// the bytes that may legitimately differ: `Some((guest, lifted))` where the guest run sees
/// the stub's return address (return slot) or window argument `k >= LIFTED_ARGS_MIN` and the
/// lifted frame sees 0 (its return slot is never written; slots past its window are not
/// copied in and read 0); `None` where both sides see the same thing (args 5..23) or the
/// guest value is not known (home slots: the stub never writes them).
fn frame_image(rg: u64, w: &[u64; WIN]) -> [Option<(u8, u8)>; FRAME_IMAGE] {
    let mut img = [None; FRAME_IMAGE];
    for (j, b) in rg.to_le_bytes().into_iter().enumerate() {
        img[j] = Some((b, 0));
    }
    for k in LIFTED_ARGS_MIN..=WIN_ARGS {
        // Argument k sits at entry rsp + 8k (k >= 5: [rsp+0x28] is argument 5).
        for (j, b) in w[k - 1].to_le_bytes().into_iter().enumerate() {
            img[8 * k + j] = Some((b, 0));
        }
    }
    img
}

/// Byte shifts `P` (arena byte offset - frame-image offset) under which arena qword `i`
/// (`gw` first guest run, `lw` lifted run) is explained by a copy out of the frame: every
/// differing byte maps to a [`frame_image`] byte that is `(gw byte, lw byte)`. Bytes outside
/// the image (or at `None` positions) must be equal.
fn overread_shifts(i: usize, gw: u64, lw: u64, img: &[Option<(u8, u8)>; FRAME_IMAGE]) -> Vec<i64> {
    let (g, l) = (gw.to_le_bytes(), lw.to_le_bytes());
    let base = (i * 8) as i64;
    (-7i64..FRAME_IMAGE as i64)
        .filter(|&p0| {
            (0..8).all(|j| {
                if g[j] == l[j] {
                    return true;
                }
                let p = p0 + j as i64;
                (0..FRAME_IMAGE as i64).contains(&p) && img[p as usize] == Some((g[j], l[j]))
            })
        })
        .map(|p0| base - p0)
        .collect()
}

/// Pure arena-verdict helper ([`MEMSTACK_ONLY`], [`OVERREAD_ONLY`], or `0`) comparing guest
/// arena `g` against lifted arena `l`:
/// 1. Neutralizes aligned or unaligned 8-byte stack addresses (`in_either_stack` on both sides,
///    e.g. `Assembler::emit_operand` copying `&stack_operand` byte-by-byte to arbitrary `pc_`).
/// 2. Neutralizes aligned qwords whose low 32 bits match and whose upper 32 bits are stale
///    stack-address halves in uninitialized 4-byte struct padding ([`is_stack_pad_qword`]).
/// 3. Checks whether any remaining differences are explained by one contiguous caller-frame
///    overread ([`overread_shifts`], `nz >= 2`).
fn memstack_verdict(
    g: &[u8],
    l: &[u8],
    m: &Meta,
    img: &[Option<(u8, u8)>; FRAME_IMAGE],
) -> u64 {
    if g.len() != l.len() || g == l {
        return 0;
    }
    let len = g.len();
    let mut l_rem = l.to_vec();
    let mut diffs = 0usize;
    let is_stk_at = |s: usize, l_buf: &[u8]| -> bool {
        let gw = u64::from_le_bytes(g[s..s + 8].try_into().unwrap());
        let lw = u64::from_le_bytes(l_buf[s..s + 8].try_into().unwrap());
        gw != lw && in_either_stack(gw, m) && in_either_stack(lw, m)
    };
    let mut b = 0usize;
    let mut last_end = 0usize;
    while b < len {
        if g[b] == l_rem[b] {
            b += 1;
            continue;
        }
        let al = b & !7;
        let mut matched = None;
        if al >= last_end && al + 8 <= len && is_stk_at(al, &l_rem) {
            matched = Some(al);
        } else if len >= 8 {
            let lo = b.saturating_sub(7).max(last_end);
            let hi = b.min(len - 8);
            for s in lo..=hi {
                if is_stk_at(s, &l_rem) {
                    matched = Some(s);
                    break;
                }
            }
        }
        if let Some(s) = matched {
            l_rem[s..s + 8].copy_from_slice(&g[s..s + 8]);
            diffs += 1;
            last_end = s + 8;
            b = s + 8;
        } else {
            b += 1;
        }
    }
    let mut shifts: Option<Vec<i64>> = None;
    let mut nz = 0u32;
    for i in 0..len / 8 {
        let gw = u64::from_le_bytes(g[i * 8..i * 8 + 8].try_into().unwrap());
        let lw = u64::from_le_bytes(l_rem[i * 8..i * 8 + 8].try_into().unwrap());
        if gw == lw {
            continue;
        }
        if is_stack_pad_qword(gw, lw, m) {
            diffs += 1;
            continue;
        }
        let s = overread_shifts(i, gw, lw, img);
        let common: Vec<i64> = match &shifts {
            None => s,
            Some(prev) => prev.iter().copied().filter(|p| s.contains(p)).collect(),
        };
        if common.is_empty() {
            return 0;
        }
        shifts = Some(common);
        nz += gw.to_le_bytes().iter().zip(lw.to_le_bytes()).filter(|(a, b)| *a != b).count() as u32;
    }
    if shifts.is_some() {
        if nz >= 2 {
            OVERREAD_ONLY
        } else {
            0
        }
    } else if diffs > 0 {
        MEMSTACK_ONLY
    } else {
        0
    }
}

/// Child only, after the lifted run: [`MEMSTACK_ONLY`] if the arena differs from the first
/// guest run's only in aligned/unaligned stack addresses or stack-padding high-dword residue;
/// [`OVERREAD_ONLY`] if remaining bytes are one contiguous copy out of the caller-side frame
/// with at least two differing bytes; else 0.
unsafe fn memstack_check(arena_addr: usize, w: &[u64; WIN]) -> u64 {
    let Some(g) = first_guest_arena() else {
        return 0;
    };
    let m = Meta {
        guest_rsp: CHILD_GUEST_RSP.load(core::sync::atomic::Ordering::Relaxed),
        lifted_sp: LIFTED_SP.load(core::sync::atomic::Ordering::Relaxed),
        ..Meta::default()
    };
    let img = frame_image(stub_ret(), w);
    let l = core::slice::from_raw_parts(arena_addr as *const u8, ARENA);
    memstack_verdict(&g, l, &m, &img)
}

/// `AW2_DLL_DIFF_PRECISE=1`: the children's guest runs use precise guest faults
/// (`emu::set_precise_faults`: an access violation at a guest instruction is dispatched to the
/// frame-based handlers from that instruction, as natively), and every function's trials are
/// classified by the SEH faults they raised ("seh" rows: landed_both, fault_both_unhandled,
/// mismatch, no_fault_injected; g18 seh_guard_plan.md). Default off.
fn precise_faults_env() -> bool {
    static P: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *P.get_or_init(|| std::env::var("AW2_DLL_DIFF_PRECISE").is_ok_and(|v| v.trim() == "1"))
}

/// Parent-side switch read by the next [`in_child`] fork: its guest runs use precise guest
/// faults even in default mode (the seh_unraised confirmation re-run, module docs).
static CHILD_FORCE_PRECISE: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);

/// Parent-side switch read by the next [`in_child`] fork (inherited, like
/// [`CHILD_FORCE_PRECISE`]): `0` = default stub, `1` = [`stub_variant`]`(1)` (zeroes past
/// `WIN_ARGS`), `2` = [`stub_variant`]`(2)` (matches default `A = 24` `LiftedFrame` + clean
/// local/home slots), `3` = lifted re-run matching [`stub_variant`]`(2)` (args `24..=31` zeroed +
/// `[entry rsp] = stub_variant(2).2`) for the `unseeded_frame_read` confirmation re-run.
static CHILD_ZERO_FRAME: core::sync::atomic::AtomicU8 = core::sync::atomic::AtomicU8::new(0);

/// Precise guest faults interpret one instruction at a time, many times slower than the
/// default engine: the slow-guest re-run's budget gets this extra factor in precise mode
/// (g21 cohtml +0x10c880: the default-mode guest reached the lifted run's arena-end fault in
/// ~5 s, the precise one timed out at 4 x 5 s; "slow_guest_precise (harness)").
const PRECISE_SLOW_X: u64 = 8;

/// `AW2_DLL_DIFF_FN=<module substring>:<hex offset>[:<trial>]`: fuzz only that function of the
/// matching image (all trials, or one), logging the inputs and each trial's three outcomes.
fn focus_for(module: &str) -> Option<(usize, Option<u32>)> {
    let v = std::env::var("AW2_DLL_DIFF_FN").ok()?;
    let mut it = v.trim().split(':');
    let (m, off) = (it.next()?, it.next()?);
    if !module.to_ascii_lowercase().contains(&m.to_ascii_lowercase()) {
        return None;
    }
    let off = usize::from_str_radix(off.trim_start_matches("0x").trim_start_matches("+0x"), 16).ok()?;
    Some((off, it.next().and_then(|t| t.parse().ok())))
}

/// Live rax bits per entry VA from the return-liveness tables (`AW2_DIFF_RETS=<paths>`,
/// default `db/ret_liveness.tsv,db/ret_liveness_dll.tsv`, read relative to the working
/// directory, else to the repository this crate was built from). Rows decode like
/// `AW2_SHADOW rets=` ([`crate::shadow::RetUse::from_row`]); only rows with dead bits are kept.
fn ret_masks() -> std::collections::HashMap<u64, u64> {
    ret_tables().0.clone()
}

/// Entries whose declared return type is floating point (`sig` = `float` rows of the same
/// tables, e.g. msvcp140 `_Stod`): xmm0 is their return value, compared even when the lifted
/// body carries no [`XMM_RET`] flag (a tail call into the function that sets it).
fn float_rets() -> std::collections::HashSet<u64> {
    ret_tables().1.clone()
}

/// Entries whose xmm0 no caller reads: rows of the same tables that decode to
/// `RetUse::xmm == false` (declared integer / pointer / void return, `sig` `int` / `void`, or
/// an `xmm` column of `0` from the x86 caller analysis). Address-taken (`ind`) rows never
/// qualify. A trial where xmm0 differs only for such an entry is `ret_dead`, not a mismatch.
fn xmm_dead() -> std::collections::HashSet<u64> {
    ret_tables().2.clone()
}

type RetTables = (std::collections::HashMap<u64, u64>, std::collections::HashSet<u64>, std::collections::HashSet<u64>);

fn ret_tables() -> &'static RetTables {
    static TABLES: std::sync::OnceLock<RetTables> = std::sync::OnceLock::new();
    TABLES.get_or_init(|| {
        let paths = std::env::var("AW2_DIFF_RETS").unwrap_or_else(|_| "db/ret_liveness.tsv,db/ret_liveness_dll.tsv".into());
        let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let (mut m, mut fl, mut xd) = (std::collections::HashMap::new(), std::collections::HashSet::new(), std::collections::HashSet::new());
        for p in paths.split(',').map(str::trim).filter(|p| !p.is_empty()) {
            let Some(text) = std::fs::read_to_string(p).ok().or_else(|| std::fs::read_to_string(repo.join(p)).ok()) else {
                log::warn!("difftest: return-liveness table {p} not readable (rax compared in full)");
                continue;
            };
            for line in text.lines() {
                let c: Vec<&str> = line.split('\t').collect();
                if c.len() < 6 || c[0].starts_with('#') {
                    continue;
                }
                let Ok(va) = u64::from_str_radix(c[0].trim_start_matches("0x"), 16) else { continue };
                let ru = crate::shadow::RetUse::from_row(c[2], c[3], c[4] == "1", c[5]);
                if ru.rax_mask != u64::MAX {
                    m.insert(va, ru.rax_mask);
                }
                if c[5] == "float" && c[4] != "1" {
                    fl.insert(va);
                }
                if !ru.xmm && c[4] != "1" {
                    xd.insert(va);
                }
            }
        }
        (m, fl, xd)
    })
}

/// Focus mode: the arguments and the first 0x100 bytes behind each arena-pointer argument.
fn log_inputs(module: &str, off: usize, t: u32, gpr: &[u64; 8], init: &[u8], arena_addr: usize) {
    log::info!("dll_lift difftest focus: {module} +0x{off:x} trial {t}: arena 0x{arena_addr:x} args {gpr:x?}");
    for (i, &a) in gpr.iter().enumerate() {
        let Some(o) = (a as usize).checked_sub(arena_addr).filter(|&o| o < ARENA) else { continue };
        let words: Vec<String> = init[o..(o + 0x100).min(ARENA)]
            .chunks(8)
            .enumerate()
            .filter(|(_, c)| c.len() == 8)
            .map(|(k, c)| format!("+{:02x}={:x}", k * 8, u64::from_le_bytes(c.try_into().unwrap())))
            .collect();
        log::info!("dll_lift difftest focus:   arg{i} [0x{a:x}] {}", words.join(" "));
    }
}

/// One `dll_lift::<module>` difftest per lifted image that is installed.
pub(crate) fn difftests() -> Vec<DllDiff> {
    let mut out = Vec::new();
    for img in super::IMAGES.iter().copied() {
        let Some(first) = img.slices.iter().copied().flatten().next() else { continue };
        if !super::installed(img.name) {
            continue;
        }
        let name: &'static str = Box::leak(format!("dll_lift::{}", img.name.trim_end_matches(".dll")).into_boxed_str());
        out.push(DllDiff { name, addr: first.va as usize, new: first.ptr as usize, diff: diff_image });
    }
    if std::env::var_os("AW2_EXE_DIFF_VAS").is_some() {
        out.push(DllDiff { name: "exe_lift_fuzz", addr: 0, new: 0, diff: diff_exe });
    }
    out
}

/// `exe_lift_fuzz` (`AW2_EXE_DIFF_VAS=<file>`, select with `AW2_DIFF_ONLY=exe_lift_fuzz`): the
/// fuzz of [`fuzz_entries`] over exe replacements. The file lists exe VAs (hex, separated by
/// commas or whitespace, as `AW2_SHADOW` takes them); each must be a registry replacement that
/// the VA dispatch table resolves to its function (others are reported and skipped). The guest
/// side is the exe's x86 at the VA, the compared memory the exe's writable sections, the log
/// rows `exe_lift difftest fn\texe\t+0x<va>\t<result>`; `AW2_DLL_DIFF_FN=exe:<va>[:trial]`,
/// `AW2_DLL_DIFF_SHARD` and `AW2_DLL_DIFF_WALL_S` apply as for the DLL lifts. Exe bodies carry
/// no [`LEAF`] / [`XMM_RET`] flags: every entry is fuzzed as a non-leaf and only `rax` is
/// compared as the return value.
unsafe fn diff_exe(_orig: usize, _new: usize) -> Result<u32, String> {
    let file = std::env::var("AW2_EXE_DIFF_VAS").map_err(|_| "AW2_EXE_DIFF_VAS not set".to_string())?;
    let text = std::fs::read_to_string(&file).map_err(|e| format!("AW2_EXE_DIFF_VAS={file}: {e}"))?;
    let by_va: std::collections::HashMap<u64, usize> = crate::registry::all().map(|r| (r.va, r.ptr as usize)).collect();
    let (mut entries, mut seen, mut skipped) = (Vec::new(), std::collections::HashSet::new(), Vec::new());
    for tok in text.split(|c: char| c == ',' || c.is_whitespace()).filter(|t| !t.is_empty()) {
        let Ok(va) = u64::from_str_radix(tok.trim_start_matches("0x"), 16) else {
            return Err(format!("AW2_EXE_DIFF_VAS={file}: bad VA {tok:?}"));
        };
        if !seen.insert(va) {
            continue;
        }
        match by_va.get(&va) {
            Some(&p) if crate::exe::lookup_va_only(va) == Some(p) => {
                entries.push(super::Lifted { va, ptr: p as *const (), flags: 0, alias_rva: 0 })
            }
            Some(_) => skipped.push(format!("0x{va:x} (dispatches elsewhere)")),
            None => skipped.push(format!("0x{va:x} (no replacement)")),
        }
    }
    if !skipped.is_empty() {
        log::warn!("exe_lift difftest: exe: {} VA(s) skipped: {skipped:?}", skipped.len());
    }
    let secs = writable_sections(crate::exe::base());
    let bytes: usize = secs.iter().map(|&(s, e)| e - s).sum();
    let header = format!("{} VAs listed, {} dispatched to their replacement, exe writable sections {} KB", seen.len(), entries.len(), bytes >> 10);
    fuzz_entries("exe_lift", "exe", 0, &secs, &std::collections::HashMap::new(), &entries, &header)
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        // splitmix64
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

/// Writable sections `[start, end)` of the image mapped at `base`.
unsafe fn writable_sections(base: usize) -> Vec<(usize, usize)> {
    let rd16 = |o: usize| core::ptr::read_unaligned((base + o) as *const u16);
    let rd32 = |o: usize| core::ptr::read_unaligned((base + o) as *const u32);
    let nt = rd32(0x3C) as usize;
    let nsec = rd16(nt + 6) as usize;
    let table = nt + 24 + rd16(nt + 20) as usize;
    (0..nsec)
        .filter_map(|i| {
            let e = table + i * 40;
            let (vsize, va, ch) = (rd32(e + 8) as usize, rd32(e + 12) as usize, rd32(e + 36));
            (ch & 0x8000_0000 != 0 && vsize != 0).then_some((base + va, base + va + vsize))
        })
        .collect()
}

/// Arena contents for one trial (see the module docs).
fn fill_arena(rng: &mut Rng, arena: &mut [u8], arena_addr: usize) {
    let words = (ARENA - TAIL_ZERO) / 8;
    for w in 0..ARENA / 8 {
        let v = if w >= words {
            0
        } else {
            match rng.next() % 8 {
                0 | 1 => 0,
                2 | 3 => rng.next() % 16,
                4 => rng.next() & 0x0F0F_0F0F,
                _ => {
                    let here = w * 8;
                    let span = (ARENA - here - 16) as u64;
                    let off = here + 16 + ((rng.next() % span) as usize & !15);
                    (arena_addr + off.min(ARENA - 16)) as u64
                }
            }
        };
        arena[w * 8..w * 8 + 8].copy_from_slice(&v.to_le_bytes());
    }
}

fn arena_ptr(rng: &mut Rng, arena_addr: usize) -> u64 {
    (arena_addr + ((rng.next() as usize % (ARENA / 2)) & !15)) as u64
}

/// One argument of a documented C signature.
#[derive(Clone, Copy)]
enum Ty {
    /// Arena pointer, any alignment (strings end at the latest in the zero tail).
    P,
    /// Byte value (`int c` of memchr/strchr/memset), often one present in the buffer.
    B,
    /// 16-bit character (`wchar_t c`).
    W,
    /// Length that stays inside the arena (pointers start in its first half).
    N,
}

/// Exported functions with documented C signatures, by export name.
fn signature(name: &str) -> Option<&'static [Ty]> {
    use Ty::*;
    Some(match name {
        "memcpy" | "memmove" | "memcmp" => &[P, P, N],
        "memset" | "memchr" => &[P, B, N],
        "strchr" | "strrchr" => &[P, B],
        "wcschr" | "wcsrchr" => &[P, W],
        "strstr" | "wcsstr" | "__std_type_info_compare" | "__std_exception_copy" => &[P, P],
        _ => return None,
    })
}

/// Export RVA -> name of the image mapped at `base`.
unsafe fn exports(base: usize) -> std::collections::HashMap<usize, String> {
    let rd32 = |o: usize| core::ptr::read_unaligned((base + o) as *const u32) as usize;
    let rd16 = |o: usize| core::ptr::read_unaligned((base + o) as *const u16) as usize;
    let nt = rd32(0x3C);
    let dir = rd32(nt + 0x88);
    let mut out = std::collections::HashMap::new();
    if dir == 0 {
        return out;
    }
    let (n, funcs, names, ords) = (rd32(dir + 0x18), rd32(dir + 0x1C), rd32(dir + 0x20), rd32(dir + 0x24));
    for i in 0..n {
        let name = std::ffi::CStr::from_ptr((base + rd32(names + 4 * i)) as *const core::ffi::c_char);
        let rva = rd32(funcs + 4 * rd16(ords + 2 * i));
        out.insert(rva, name.to_string_lossy().into_owned());
    }
    out
}

/// Typed arguments for `sig`; may also edit the arena (`init`) so that comparisons see equal
/// prefixes and searches find their byte.
fn typed_args(rng: &mut Rng, sig: &[Ty], init: &mut [u8], arena_addr: usize) -> [u64; 8] {
    let mut g = [0u64; 8];
    let mut ptrs = Vec::new();
    for (i, t) in sig.iter().enumerate() {
        g[i] = match t {
            Ty::P => {
                let p = arena_ptr(rng, arena_addr) + rng.next() % 16;
                ptrs.push(p);
                p
            }
            Ty::B | Ty::W => match ptrs.first() {
                Some(&p) if rng.next() % 2 == 0 => {
                    let o = p as usize - arena_addr + (rng.next() % 64) as usize;
                    if matches!(t, Ty::B) { init[o] as u64 } else { u16::from_le_bytes([init[o & !1], init[(o & !1) + 1]]) as u64 }
                }
                _ => rng.next() % if matches!(t, Ty::B) { 256 } else { 65536 },
            },
            Ty::N => [0, 1, 7, 8, 15, 16, 31, 32, 33, 63, 64, 100, 255, 256, 1000, 4096][(rng.next() % 16) as usize],
        };
    }
    // Two buffers: often make the second start with a copy of the first (equal prefix).
    if let [a, b, ..] = ptrs[..] {
        if rng.next() % 2 == 0 {
            let (a, b) = (a as usize - arena_addr, b as usize - arena_addr);
            let k = (rng.next() % 300) as usize;
            if a.max(b) + k < ARENA - TAIL_ZERO {
                init.copy_within(a..a + k, b);
            }
        }
    }
    g
}

/// Integer arguments 1..8 for `trial` (shape rotates, see the module docs).
fn args_for(rng: &mut Rng, arena_addr: usize, trial: u32) -> [u64; 8] {
    let mut g = [0u64; 8];
    for (i, v) in g.iter_mut().enumerate() {
        *v = match (trial % 4, i) {
            (0, _) => arena_ptr(rng, arena_addr),
            (2, 0) | (3, 0) | (3, 1) => arena_ptr(rng, arena_addr),
            (2, _) | (3, _) => rng.next() % 64,
            _ => match rng.next() % 4 {
                0 => 0,
                1 => rng.next() % 32,
                _ => arena_ptr(rng, arena_addr),
            },
        };
    }
    g
}

/// A trial's stack window (see [`WIN`]): args 5..8 are `gpr[4..8]`, args 9..31 follow the
/// shapes of [`args_for`] for this trial (all arena pointers; null / small / pointer mix;
/// small integers), drawn from their own stream seeded by `(va, trial)` so the arena and
/// register inputs stay what they were before stack arguments were fuzzed.
fn stack_window(va: u64, trial: u32, arena_addr: usize, gpr: &[u64; 8]) -> [u64; WIN] {
    let mut rng = Rng(va ^ ((trial as u64) << 48) ^ 0x5A5A_57AC);
    let mut w = [0u64; WIN];
    w[4..8].copy_from_slice(&gpr[4..8]);
    for v in w[8..WIN_ARGS].iter_mut() {
        *v = match trial % 4 {
            0 => arena_ptr(&mut rng, arena_addr),
            2 | 3 => rng.next() % 64,
            _ => match rng.next() % 4 {
                0 => 0,
                1 => rng.next() % 32,
                _ => arena_ptr(&mut rng, arena_addr),
            },
        };
    }
    if poison_window() {
        for k in LIFTED_ARGS_MIN..=WIN_ARGS {
            w[k - 1] = poison_slot(k);
        }
    }
    w
}

/// `AW2_DLL_DIFF_POISON=1`: window arguments `k >= LIFTED_ARGS_MIN` (past every default lifted
/// argument window) hold [`poison_slot`]`(k)` instead of fuzzed values.
fn poison_window() -> bool {
    static P: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *P.get_or_init(|| std::env::var("AW2_DLL_DIFF_POISON").is_ok_and(|v| v.trim() == "1"))
}

/// Slot-tagged qword for window argument `k`: bytes `k, 0xA5, k, 0xC3, k, 0x5A, k, 0x96` (low
/// first), so every byte copied out of it names its slot.
fn poison_slot(k: usize) -> u64 {
    let k = k as u64 & 0xff;
    k | 0xA5 << 8 | k << 16 | 0xC3 << 24 | k << 32 | 0x5A << 40 | k << 48 | 0x96 << 56
}

#[allow(non_camel_case_types)]
type pid_t = i32;
extern "C" {
    fn fork() -> pid_t;
    fn pthread_self() -> usize;
    fn pipe(fds: *mut i32) -> i32;
    fn read(fd: i32, buf: *mut u8, n: usize) -> isize;
    fn write(fd: i32, buf: *const u8, n: usize) -> isize;
    fn close(fd: i32) -> i32;
    fn waitpid(pid: pid_t, status: *mut i32, options: i32) -> pid_t;
    fn kill(pid: pid_t, sig: i32) -> i32;
    fn sigaction(sig: i32, act: *const SigAction, old: *mut SigAction) -> i32;
    fn _exit(code: i32) -> !;
    fn mmap(addr: *mut u8, len: usize, prot: i32, flags: i32, fd: i32, off: i64) -> *mut u8;
}

/// Base of the fixed-address harness mappings ([`map_pinned`]): fuzz arenas at
/// `PIN_BASE + k * PIN_STRIDE` (`k` from the image name), the guest stub at [`PIN_STUB`].
/// macOS (arm64) ignores mmap hints below 0x70_0000_0000 (they land at the first free
/// address from 0x70_0000_0000 up, probed on 27.2), so the base must be above that. Anywhere
/// mappings (pinned DLL images at 0x70_.., the guest's VirtualAlloc / stack mappings at
/// 0x74_..-0x7F_..; 0x7E_.. was taken in 6 of 19 g12j focus processes) fill upward from
/// 0x70_.., so 0x5A0_0000_0000 (~5.6 TiB) is far from all of them and below 2^47.
const PIN_BASE: usize = 0x5A0_0000_0000;
const PIN_STRIDE: usize = 0x10_0000;
const PIN_STUB: usize = PIN_BASE - 0x10_0000;

/// Zeroed RW memory at exactly `hint` (macOS honours an mmap hint when the range is free;
/// no `MAP_FIXED`, so nothing mapped is ever replaced). The fuzz inputs are arena pointers
/// and the guest stub's return address lands in arenas, so with fixed addresses a shard
/// trial and `AW2_DLL_DIFF_FN=<module>:<off>:<trial>` see bit-identical inputs. If the range
/// is taken the mapping goes elsewhere (logged): trials stay valid, only not reproducible
/// across processes.
unsafe fn map_pinned(hint: usize, len: usize, what: &str) -> usize {
    const PROT_RW: i32 = 1 | 2;
    const MAP_PRIVATE_ANON: i32 = 0x0002 | 0x1000;
    let p = mmap(hint as *mut u8, len, PROT_RW, MAP_PRIVATE_ANON, -1, 0) as usize;
    if p == usize::MAX || p == 0 {
        panic!("difftest: mmap of {len:#x} bytes for {what} failed");
    }
    if p != hint {
        log::warn!("difftest: {what} could not be pinned at {hint:#x} (mapped at {p:#x}): trials are not reproducible across processes");
    }
    p
}

/// The fuzz arena of image `name` (one per name and process, 64-byte aligned, pinned).
fn pinned_arena(name: &str) -> usize {
    static ARENAS: std::sync::Mutex<Vec<(String, usize)>> = std::sync::Mutex::new(Vec::new());
    let mut a = ARENAS.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(&(_, p)) = a.iter().find(|(n, _)| n == name) {
        return p;
    }
    let k = (super::super::fnv1a(name.as_bytes()) % 64) as usize;
    let p = unsafe { map_pinned(PIN_BASE + k * PIN_STRIDE, ARENA, &format!("the {name} fuzz arena")) };
    a.push((name.to_string(), p));
    p
}

/// Guest heap of the fuzz runs (bytes): see [`det_heap_restart`].
const DET_HEAP: usize = 64 << 20;
/// Where it is pinned: 4 GiB above the fuzz arenas.
const PIN_HEAP: usize = PIN_BASE + 0x1_0000_0000;

/// Base of the pinned guest-heap arena (mapped once, in the parent, before any fork).
fn det_heap() -> usize {
    static BASE: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
    *BASE.get_or_init(|| unsafe { map_pinned(PIN_HEAP, DET_HEAP, "the difftest guest heap") })
}

/// Deterministic guest heap: from here on (in a fuzz child) `malloc` / `HeapAlloc` & co.
/// (`win32_rt::heap`, guest and lifted code alike) bump-allocate from the pinned, zeroed
/// [`det_heap`] arena, restarted before every run, and `free` never reaches the host
/// allocator (the shadow children's heap mode, `win32_rt::shadow::set_heap_arena`). The
/// guest and the lifted run of a trial then see the same addresses for the same allocation
/// sequence (a function that stores or returns a block it allocated, v8 +0x8740
/// RegExpBytecodeGenerator::Expand, is a function of the trial), and a run that frees or
/// reallocs a fuzzed pointer cannot leave the host heap broken for the next run. Removes a
/// nondeterministic input; not a class.
unsafe fn det_heap_restart() {
    let base = det_heap();
    let (cur, end) = win32_rt::shadow::heap_arena();
    if end == base + DET_HEAP && cur > base {
        core::ptr::write_bytes(base as *mut u8, 0, cur.min(end) - base);
    }
    win32_rt::shadow::set_heap_arena(base, DET_HEAP);
}

/// macOS `struct sigaction` (handler union, `sa_mask`, `sa_flags`).
#[repr(C)]
struct SigAction {
    handler: usize,
    mask: u32,
    flags: i32,
}
const SA_SIGINFO: i32 = 0x40;
/// `siginfo_t::si_addr` offset (macOS: signo, errno, code, pid, uid, status, addr).
const SI_ADDR: usize = 24;
/// First word of a fault record: `[FAULT_TAG, fault address, signal | host pc << 8, foreign | host lr]`;
/// `foreign` (bit 0) = 1 when the faulting thread is not the harness thread (see
/// [`HARNESS_THREAD`]). Focus mode logs pc / lr symbolized.
const FAULT_TAG: u64 = 0xFA17_FA17_FA17_FA17;
/// First word of the record of a run that called a non-code target: `[TRAP_TAG, 0, 0, 0]`.
const TRAP_TAG: u64 = 0x7A4B_7A4B_7A4B_7A4B;
/// Result record of a run that threw an MSVC C++ exception caught by the harness boundary:
/// `[THROW_TAG, ThrowInfo address, object digest, state digest]` (see the module docs, "C++
/// throws"; [`throw_note`]).
const THROW_TAG: u64 = 0x7E80_7E80_7E80_7E80;
/// Record of a run whose C++ exception the dispatch could not deliver to the boundary:
/// `[UNREC_TAG, ThrowInfo address, 0, 0]` ([`child_cxx_throw`], "threw_unrecorded").
const UNREC_TAG: u64 = 0x7E81_7E81_7E81_7E81;
/// `AW2_DLL_DIFF_THROW_DUMP` only: `[DUMP_TAG, ThrowInfo, qword index, masked object qword]`
/// before the run's [`THROW_TAG`] record; the parent logs them.
const DUMP_TAG: u64 = 0x7E82_7E82_7E82_7E82;
/// `[THROWX_TAG, loose object digest, data digest, 0]` before a throwing run's [`THROW_TAG`] record:
/// the object digest with, in addition, every aligned dword that is the high half of an
/// address in that run's own stacks masked (stale stack-address halves in uninitialized
/// padding, e.g. after `error_code::_Myval`); see "threw_stack_residue".
const THROWX_TAG: u64 = 0x7E83_7E83_7E83_7E83;
/// Last record of a child that finished: `[STACK_TAG, guest entry rsp, lifted stack top at
/// the lifted call, 0]`.
const STACK_TAG: u64 = 0x57AC_57AC_57AC_57AC;
/// Record that follows a run's result record: `[CANON_TAG, NaN-canonical arena digest,
/// NaN-canonical data digest, 0]` (see [`nan_canon_hash`]).
const CANON_TAG: u64 = 0xCA40_CA40_CA40_CA40;
/// Record written (once per run, at once) when the current run made a call, jump or return to
/// address 0 (`emu::NULL_CALL_HOOK`): `[NULLCALL_TAG, 0, 0, 0]`. It precedes that run's result
/// record, so it belongs to the run whose result comes next.
const NULLCALL_TAG: u64 = 0x4E43_4E43_4E43_4E43;
/// Record written (once per run, at once) when the current run raised an access violation
/// for a guarded lifted access or a precise guest fault (`lift::eh::SEH_FAULT_HOOK`):
/// `[SEHF_TAG, address, 0, 0]` (the first fault's), before that run's result record (as [`NULLCALL_TAG`]).
const SEHF_TAG: u64 = 0x5E4F_5E4F_5E4F_5E4F;

static CHILD_WFD: core::sync::atomic::AtomicI32 = core::sync::atomic::AtomicI32::new(-1);
/// Child only: result records emitted so far, and the count at the last [`NULLCALL_TAG`]
/// record (one per run).
static CHILD_EMITTED: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);
static CHILD_NULL_AT: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(u64::MAX);

/// `emu::NULL_CALL_HOOK` in children: one [`NULLCALL_TAG`] record per run.
fn child_null_call() {
    use core::sync::atomic::Ordering::Relaxed;
    let n = CHILD_EMITTED.load(Relaxed);
    if CHILD_NULL_AT.swap(n, Relaxed) != n {
        let rec: [u8; 32] = unsafe { core::mem::transmute([NULLCALL_TAG, 0u64, 0, 0]) };
        unsafe { write(CHILD_WFD.load(Relaxed), rec.as_ptr(), 32) };
    }
}
static CHILD_SEHF_AT: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(u64::MAX);

/// `lift::eh::SEH_FAULT_HOOK` in children: one [`SEHF_TAG`] record per run.
fn child_seh_fault(addr: u64) {
    use core::sync::atomic::Ordering::Relaxed;
    let n = CHILD_EMITTED.load(Relaxed);
    if CHILD_SEHF_AT.swap(n, Relaxed) != n {
        let rec: [u8; 32] = unsafe { core::mem::transmute([SEHF_TAG, addr, 0, 0]) };
        unsafe { write(CHILD_WFD.load(Relaxed), rec.as_ptr(), 32) };
    }
}

/// `win32_rt::unwind::UNHANDLED_AV_HOOK` in children: an access violation no frame handled
/// (precise guest fault, guarded lifted access) ends the run as the native fault would, with
/// the same [`FAULT_TAG`] record as [`child_fault`] (signal 11, the faulting address).
fn child_unhandled_av(rec: &win32_rt::unwind::ExceptionRecord) {
    unsafe {
        let r: [u8; 32] = core::mem::transmute([FAULT_TAG, rec.information[1], 11u64, 0]);
        write(CHILD_WFD.load(core::sync::atomic::Ordering::Relaxed), r.as_ptr(), 32);
        _exit(70);
    }
}

/// Child only: top of the lifted guest stack just before the lifted run's call. Lifted
/// frames live in the per-thread `lstack` arena (or, if it is off / exhausted, a local slot
/// on the native stack), so this is probed with a throwaway frame (see [`lifted_stack_top`]).
static LIFTED_SP: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(0);

/// Address just above the argument qwords of the next lifted frame on this thread: a
/// throwaway `LiftedFrame` is carved and popped (LIFO), so the callee's frame starts at the
/// same top. Functions with a big frame (`alloc_big_lifted_frame`) live elsewhere and are
/// not covered.
unsafe fn lifted_stack_top() -> u64 {
    let mut f = crate::simd_ops::LiftedFrame::<32, 0>::new();
    let p = crate::simd_ops::StackBuf::frame_base(&mut f) as u64;
    p + 24 * 8
}

/// `va` is executable code as far as a native process is concerned: inside a mapped PE image
/// (the exe or a DLL; images are pinned, so VA = address).
fn code_va(va: u64) -> bool {
    let size = crate::exe::image_size();
    crate::dll_native::in_mapped_image(va)
        || va.wrapping_sub(crate::exe::preferred_base()) < size
        || va.wrapping_sub(crate::exe::base() as u64) < size
}

/// Host function registered (children only) for every non-code value a trial can call: the
/// run that calls it ends with a [`TRAP_TAG`] record, as the original would fault there.
/// Reached through the normal dispatch from both sides (the emulator's call/jmp resolution
/// and the lifted `call_va8*` bridge).
unsafe extern "win64" fn noncode_trap(_: u64, _: u64, _: u64, _: u64) -> u64 {
    let rec: [u8; 32] = core::mem::transmute([TRAP_TAG, 0u64, 0, 0]);
    write(CHILD_WFD.load(core::sync::atomic::Ordering::Relaxed), rec.as_ptr(), 32);
    _exit(70);
}

/// Child-only `win32_rt::unwind::CXX_THROW_HOOK`: a run's C++ exception did not reach the
/// harness boundary (the dispatch stopped at a frame without an EH record). Report its
/// `ThrowInfo` (images are pinned, so the same throw has the same address on both sides) and
/// end the run before the unhandled-exception path.
fn child_cxx_throw(rec: &win32_rt::unwind::ExceptionRecord) {
    let ti = if rec.number_parameters >= 3 { rec.information[2] } else { 0 };
    unsafe {
        let r: [u8; 32] = core::mem::transmute([UNREC_TAG, ti, 0u64, 0]);
        write(CHILD_WFD.load(core::sync::atomic::Ordering::Relaxed), r.as_ptr(), 32);
        _exit(70);
    }
}

/// Stand-in for an object qword that is an address in either run's stack.
const STACK_WORD: u64 = 0x57AC_0B1E_57AC_0B1E;

/// Child, `catch_cxx` inspect function (dispatch phase, the thrower's frames intact): digests
/// of the thrown object, `[exact, loose]` (FNV of its bytes, size folded in). Size = the first
/// CatchableType's `sizeOrOffset` (at most 1 KB); every aligned qword that is an address in the
/// emulator stack, the lstack arena or the big-frame arena (each run's own) reads as
/// [`STACK_WORD`]; `loose` also masks stack-address high dwords ([`THROWX_TAG`]).
unsafe fn throw_note(rec: &win32_rt::unwind::ExceptionRecord) -> [u64; 2] {
    // x64 MSVC: [magic, object, ThrowInfo, image base].
    if rec.number_parameters < 4 {
        return [0, 0];
    }
    let (obj, ti, base) = (rec.information[1], rec.information[2], rec.information[3]);
    if obj == 0 || !code_va(ti) {
        return [0, 0];
    }
    let rd32 = |a: u64| core::ptr::read_unaligned(a as *const i32);
    // ThrowInfo { attributes, pmfnUnwind, pForwardCompat, pCatchableTypeArray } (RVAs);
    // CatchableTypeArray { n, rva[n] }; CatchableType { properties, pType, PMD (12),
    // sizeOrOffset, copyFunction }.
    let cta = base.wrapping_add(rd32(ti + 12) as u32 as u64);
    let size = if code_va(cta) && rd32(cta) > 0 {
        let ct = base.wrapping_add(rd32(cta + 4) as u32 as u64);
        if code_va(ct) { rd32(ct + 20).clamp(0, 1024) as usize } else { 0 }
    } else {
        0
    };
    let mut b = core::slice::from_raw_parts(obj as *const u8, size).to_vec();
    let (es_lo, es_hi) = crate::emu::eh::stack_range();
    let (ls_lo, ls_hi) = (crate::lstack::arena_floor() as u64, crate::lstack::arena_end() as u64);
    let (bg_lo, bg_hi) = (BIG_LO.load(core::sync::atomic::Ordering::Relaxed), BIG_HI.load(core::sync::atomic::Ordering::Relaxed));
    let in_stack = |v: u64| (es_lo..es_hi).contains(&v) || (ls_lo != 0 && (ls_lo..ls_hi).contains(&v)) || (bg_lo..bg_hi).contains(&v);
    for q in b.chunks_exact_mut(8) {
        if in_stack(u64::from_le_bytes((&*q).try_into().unwrap())) {
            q.copy_from_slice(&STACK_WORD.to_le_bytes());
        }
    }
    if std::env::var_os("AW2_DLL_DIFF_THROW_DUMP").is_some() {
        // Children do not log: each masked qword goes to the parent as a DUMP_TAG record.
        let wfd = CHILD_WFD.load(core::sync::atomic::Ordering::Relaxed);
        for (i, c) in b.chunks_exact(8).take(32).enumerate() {
            let r: [u8; 32] = core::mem::transmute([DUMP_TAG, ti, i as u64, u64::from_le_bytes(c.try_into().unwrap())]);
            write(wfd, r.as_ptr(), 32);
        }
    }
    let exact = super::super::fnv1a(&b) ^ (size as u64).rotate_left(48);
    // Loose: also the upper dword of every qword that equals the upper dword of an address in
    // one of this run's stacks (a stale stack-address half in uninitialized padding).
    let his = |lo: u64, hi: u64| if lo < hi { (lo >> 32)..=((hi - 1) >> 32) } else { 1..=0 };
    let stack_hi = |d: u64| d != 0 && (his(es_lo, es_hi).contains(&d) || (ls_lo != 0 && his(ls_lo, ls_hi).contains(&d)) || his(bg_lo, bg_hi).contains(&d));
    for q in b.chunks_exact_mut(8) {
        if stack_hi(u32::from_le_bytes((&q[4..8]).try_into().unwrap()) as u64) {
            q[4..8].copy_from_slice(&0x57AC_0B1Eu32.to_le_bytes());
        }
    }
    [exact, super::super::fnv1a(&b) ^ (size as u64).rotate_left(48)]
}

/// The distinct non-zero values among the arena words and the arguments that are neither
/// code ([`code_va`]) nor already dispatched: what a call through an arena "vtable" can hit.
fn trap_targets(init: &[u8], args: &[u64]) -> Vec<u64> {
    let mut v: Vec<u64> = init.chunks_exact(8).map(|c| u64::from_le_bytes(c.try_into().unwrap())).chain(args.iter().copied()).collect();
    v.sort_unstable();
    v.dedup();
    v.retain(|&x| x != 0 && !code_va(x) && crate::exe::lookup_native_va(x).is_none());
    v
}

/// `mov rax, rsp; ret`.
static RSP_PROBE: [u8; 4] = [0x48, 0x89, 0xE0, 0xC3];

/// The entry `rsp` a top-level `emu_run_full` gives the guest on this thread (the emulator
/// restores its stack pointer after each run, so the function under test gets the same).
unsafe fn guest_entry_rsp() -> u64 {
    crate::emu::emu_run_full(RSP_PROBE.as_ptr() as u64, [0; 8], [0; 4], [0; 8]).0
}

/// Child only: the thread running the trials. A run may start threads (`Thread::Start`)
/// that outlive it and fault later, during another run, on state the harness restored.
static HARNESS_THREAD: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);

/// Child-only fault handler: report a [`FAULT_TAG`] record and exit
/// (async-signal-safe).
extern "C" fn child_fault(sig: i32, info: *mut u8, uc: *mut u8) {
    unsafe {
        // A guarded lifted access (`lift::guarded`): resume the probe with "faulted".
        if (sig == 10 || sig == 11) && !uc.is_null() && cfg!(target_arch = "aarch64") {
            let mc = core::ptr::read_unaligned(uc.add(48) as *const *mut u8);
            if !mc.is_null() && super::guarded::fixup_darwin_ss(mc.add(16) as *mut u64) {
                return;
            }
        }
        let addr = if info.is_null() { 0 } else { core::ptr::read_unaligned(info.add(SI_ADDR) as *const u64) };
        let foreign = (pthread_self() != HARNESS_THREAD.load(core::sync::atomic::Ordering::Relaxed)) as u64;
        // Darwin arm64: ucontext_t.uc_mcontext at +48 -> __darwin_mcontext64 { __es (16 bytes),
        // __ss { x[29], fp, lr, sp, pc, .. } }.
        let (pc, lr) = if uc.is_null() || cfg!(not(target_arch = "aarch64")) {
            (0, 0)
        } else {
            let mc = core::ptr::read_unaligned(uc.add(48) as *const *const u64);
            if mc.is_null() {
                (0, 0)
            } else {
                let ss = (mc as *const u8).add(16) as *const u64;
                (core::ptr::read_unaligned(ss.add(32)), core::ptr::read_unaligned(ss.add(30)))
            }
        };
        const VA48: u64 = 0x0000_FFFF_FFFF_FFFF;
        let rec: [u8; 32] = core::mem::transmute([FAULT_TAG, addr, sig as u64 & 0xff | (pc & VA48) << 8, foreign | (lr & VA48 & !3)]);
        write(CHILD_WFD.load(core::sync::atomic::Ordering::Relaxed), rec.as_ptr(), 32);
        _exit(70);
    }
}
const WNOHANG: i32 = 1;
const SIGKILL: i32 = 9;
/// SIGILL, SIGTRAP, SIGABRT, SIGFPE, SIGBUS, SIGSEGV, SIGSYS (macOS numbering).
const FAULT_SIGNALS: [i32; 7] = [4, 5, 6, 8, 10, 11, 12];

/// Outcome of one side of a case.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Outcome {
    /// `(rax, xmm0 low 64, arena FNV, writable-sections digest)`.
    Ret([u64; 4]),
    /// Fault at this address (signal handler record).
    Fault(u64),
    /// Fault on a thread other than the harness thread (one a run started): attributable to
    /// neither side.
    ThreadFault(u64),
    /// Called a non-code target ([`noncode_trap`]).
    NonCode,
    /// Threw an MSVC C++ exception caught by the harness boundary: `[ThrowInfo, object
    /// digest, state digest]` ([`THROW_TAG`]).
    Threw([u64; 3]),
    /// Threw an MSVC C++ exception the dispatch could not deliver to the boundary, with this
    /// `ThrowInfo` ([`child_cxx_throw`]).
    ThrewUnrecorded(u64),
    /// Guest only: the run ended at rip 0 without returning ([`null_exit_stub`]). The
    /// emulator ends the whole top-level run at a `jmp` through a zero word, while the lifted
    /// tail `call_va8(0)` returns 0 from that function only and its caller goes on; natively
    /// both fault.
    NullExit,
    /// Died without a record (abort, ...).
    Died,
    Timeout,
}

impl Outcome {
    fn crashed(self) -> bool {
        matches!(self, Outcome::Fault(_) | Outcome::Died)
    }
}

/// The two stack tops of a child's runs (see [`STACK_TAG`]; 0 = not known), and each run's
/// NaN-canonical (arena, data) digests by run order (see [`CANON_TAG`]; zero = not known).
#[derive(Clone, Copy, Default, Debug)]
struct Meta {
    guest_rsp: u64,
    lifted_sp: u64,
    canon: [[u64; 2]; 3],
    /// The lifted run's arena-vs-first-guest-run verdict ([`MEMSTACK_ONLY`] or 0); for a
    /// lifted run alone, against the guest arena its trial's shared child published.
    memstack: u64,
    /// Run 0's verdict word (a lifted run alone: its [`memstack`](Meta::memstack)).
    memstack0: u64,
    /// Bit i: run i made a call / jump / return to address 0 ([`NULLCALL_TAG`]).
    nullcall: u8,
    /// Bit i: run i raised an access violation for a guarded lifted access / precise guest
    /// fault ([`SEHF_TAG`]). A lifted run alone: bits 0 and 1.
    sehfault: u8,
    /// Run i's first [`SEHF_TAG`] address (index 1 = the lifted run, also alone).
    sehaddr: [u64; 3],
    /// Run i's thrown-object digest with stack-address high dwords masked too
    /// ([`THROWX_TAG`]; 0 = no throw). Index 1 is the lifted run, also when it ran alone.
    throw_loose: [u64; 3],
    /// Run i's data-section digest when it threw ([`THROWX_TAG`] word 2; 0 = no throw), so a
    /// throw whose state differs only in the arena can be told apart ("threw_mem_stack").
    throw_data: [u64; 3],
}

/// Run `body(write_record)` in a forked child with a wall budget of `budget_ms`; collect up to
/// three run records. Returns the records, the child's [`Meta`] and whether the child exited
/// normally (`None` = timed out).
unsafe fn in_child(budget_ms: u64, body: &dyn Fn(&dyn Fn([u64; 4]))) -> (Vec<[u64; 4]>, Meta, Option<bool>) {
    let mut fds = [0i32; 2];
    if pipe(fds.as_mut_ptr()) != 0 {
        return (Vec::new(), Meta::default(), Some(false));
    }
    let pid = fork();
    if pid == 0 {
        // Random inputs make the guest side warn on every out-of-range access
        // (`emu: read_mem invalid ...`, gigabytes per run): the child logs errors only.
        log::set_max_level(log::LevelFilter::Error);
        close(fds[0]);
        let wfd = fds[1];
        CHILD_WFD.store(wfd, core::sync::atomic::Ordering::Relaxed);
        HARNESS_THREAD.store(pthread_self(), core::sync::atomic::Ordering::Relaxed);
        // No run may start a thread that outlives it (see `thread_fault`): thread creation
        // fails identically on both sides.
        win32_rt::thread::NO_SPAWN.store(true, core::sync::atomic::Ordering::Relaxed);
        let act = SigAction { handler: child_fault as *const () as usize, mask: 0, flags: SA_SIGINFO };
        for s in FAULT_SIGNALS {
            sigaction(s, &act, core::ptr::null_mut());
        }
        let emit = move |r: [u64; 4]| {
            if r[0] != CANON_TAG {
                CHILD_EMITTED.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
            }
            let b: [u8; 32] = core::mem::transmute(r);
            write(wfd, b.as_ptr(), 32);
        };
        let guest_rsp = guest_entry_rsp();
        CHILD_GUEST_RSP.store(guest_rsp, core::sync::atomic::Ordering::Relaxed);
        crate::emu::NULL_CALL_HOOK.store(child_null_call as fn() as usize, core::sync::atomic::Ordering::Relaxed);
        super::eh::SEH_FAULT_HOOK.store(child_seh_fault as fn(u64) as usize, core::sync::atomic::Ordering::Relaxed);
        win32_rt::unwind::UNHANDLED_AV_HOOK.store(
            child_unhandled_av as fn(&win32_rt::unwind::ExceptionRecord) as usize,
            core::sync::atomic::Ordering::Relaxed,
        );
        crate::emu::set_precise_faults(precise_faults_env() || CHILD_FORCE_PRECISE.load(core::sync::atomic::Ordering::Relaxed));
        win32_rt::unwind::CXX_THROW_HOOK.store(
            child_cxx_throw as fn(&win32_rt::unwind::ExceptionRecord) as usize,
            core::sync::atomic::Ordering::Relaxed,
        );
        body(&emit);
        crate::emu::NULL_CALL_HOOK.store(0, core::sync::atomic::Ordering::Relaxed);
        emit([STACK_TAG, guest_rsp, LIFTED_SP.load(core::sync::atomic::Ordering::Relaxed), 0]);
        _exit(0);
    }
    close(fds[1]);
    if pid < 0 {
        close(fds[0]);
        return (Vec::new(), Meta::default(), Some(false));
    }
    let start = std::time::Instant::now();
    let mut status = 0i32;
    let exited = loop {
        let r = waitpid(pid, &mut status, WNOHANG);
        if r == pid {
            break Some(status & 0x7f == 0 && (status >> 8) & 0xff == 0);
        }
        if r < 0 {
            break Some(false);
        }
        if start.elapsed().as_millis() as u64 > budget_ms {
            kill(pid, SIGKILL);
            waitpid(pid, &mut status, 0);
            break None;
        }
        if start.elapsed().as_micros() < 2_000 {
            std::thread::sleep(std::time::Duration::from_micros(100));
        } else {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    };
    let (mut recs, mut meta) = (Vec::new(), Meta::default());
    let mut buf = [0u8; 32];
    while recs.len() < 4 && read(fds[0], buf.as_mut_ptr(), 32) == 32 {
        let r = core::mem::transmute::<[u8; 32], [u64; 4]>(buf);
        match r[0] {
            STACK_TAG => (meta.guest_rsp, meta.lifted_sp) = (r[1], r[2]),
            DUMP_TAG => log::info!("dll_lift difftest: run {} thrown object ti=0x{:x} q{} = 0x{:x} (stack words masked)", recs.len(), r[1], r[2], r[3]),
            THROWX_TAG => {
                if recs.len() < 3 {
                    meta.throw_loose[recs.len()] = r[1];
                    meta.throw_data[recs.len()] = r[2];
                }
            }
            NULLCALL_TAG if recs.len() < 8 => meta.nullcall |= 1 << recs.len(),
            NULLCALL_TAG => {}
            SEHF_TAG if recs.len() < 8 => {
                meta.sehfault |= 1 << recs.len();
                if recs.len() < 3 {
                    meta.sehaddr[recs.len()] = r[1];
                }
            }
            SEHF_TAG => {}
            CANON_TAG => {
                if let Some(i) = recs.len().checked_sub(1).filter(|&i| i < 3) {
                    meta.canon[i] = [r[1], r[2]];
                    if i == 1 {
                        meta.memstack = r[3];
                    } else if i == 0 {
                        meta.memstack0 = r[3];
                    }
                }
            }
            _ => recs.push(r),
        }
    }
    close(fds[0]);
    if std::env::var_os("AW2_DLL_DIFF_FN").is_some() {
        for (i, r) in recs.iter().enumerate().filter(|(_, r)| r[0] == FAULT_TAG) {
            log::info!(
                "difftest focus: child record {i}: signal {} at address 0x{:x}{}; host pc {}, lr {}",
                r[2] & 0xff,
                r[1],
                if r[3] & 1 != 0 { " (foreign thread)" } else { "" },
                host_sym(r[2] >> 8),
                host_sym(r[3] & !3)
            );
        }
    }
    (recs, meta, exited)
}

/// `symbol+0xoff (0xaddr)` of a host code address (dladdr), for focus-mode fault reports.
fn host_sym(pc: u64) -> String {
    #[repr(C)]
    struct DlInfo {
        fname: *const i8,
        fbase: *mut u8,
        sname: *const i8,
        saddr: *mut u8,
    }
    unsafe extern "C" {
        fn dladdr(addr: *const u8, info: *mut DlInfo) -> i32;
    }
    let mut i = DlInfo { fname: core::ptr::null(), fbase: core::ptr::null_mut(), sname: core::ptr::null(), saddr: core::ptr::null_mut() };
    if pc != 0 && unsafe { dladdr(pc as *const u8, &mut i) } != 0 && !i.sname.is_null() {
        let s = unsafe { std::ffi::CStr::from_ptr(i.sname) }.to_string_lossy();
        return format!("{s}+0x{:x} (0x{pc:x})", pc - i.saddr as u64);
    }
    format!("0x{pc:x}")
}

/// Strip the `--diff-test` harness bit-46 tag from `orig`.
#[inline]
fn untag_diff_orig(orig: usize) -> u64 {
    (orig & !(1usize << 46)) as u64
}

/// Verify that `start_va` inside `[base + text_rva, base + text_rva + text_len)` reaches
/// `target_va` (also in `.text`) via `1..=6` direct `0xE9 rel32` jumps, and `target_va` does not
/// itself start with `0xE9` (matching `dll_lift.py`'s `e9_final` invariant).
unsafe fn verify_e9_chain(
    base: usize,
    text_rva: usize,
    text_len: usize,
    start_va: u64,
    target_va: u64,
) -> Result<u32, String> {
    let text_lo = base.wrapping_add(text_rva) as u64;
    let text_hi = text_lo.wrapping_add(text_len as u64);
    if !(text_lo..text_hi).contains(&target_va) {
        return Err(format!("alias target 0x{target_va:x} outside .text [0x{text_lo:x}, 0x{text_hi:x})"));
    }
    let mut cur = start_va;
    for hop in 1..=6u32 {
        if cur < text_lo || cur.saturating_add(5) > text_hi {
            return Err(format!("alias hop {hop} at 0x{cur:x} outside .text [0x{text_lo:x}, 0x{text_hi:x})"));
        }
        let op = core::ptr::read_unaligned(cur as *const u8);
        if op != 0xE9 {
            return Err(format!("alias hop {hop} at 0x{cur:x} opcode 0x{op:02x} != 0xE9 (expected target 0x{target_va:x})"));
        }
        let rel = core::ptr::read_unaligned((cur + 1) as *const i32) as i64;
        cur = (cur as i64).wrapping_add(5).wrapping_add(rel) as u64;
        if cur == target_va {
            let top = core::ptr::read_unaligned(target_va as *const u8);
            if top == 0xE9 {
                return Err(format!("alias target 0x{target_va:x} itself starts with 0xE9"));
            }
            return Ok(hop);
        }
    }
    Err(format!("alias 0x{start_va:x} did not reach 0x{target_va:x} within 6 E9 hops (at 0x{cur:x})"))
}

/// `orig` is the harness's tagged address of the first entry; all entries are covered.
pub unsafe fn diff_image(orig: usize, _new: usize) -> Result<u32, String> {
    let va0 = untag_diff_orig(orig);
    let img: &LiftedImage = super::IMAGES
        .iter()
        .copied()
        .find(|i| i.slices.iter().copied().flatten().next().is_some_and(|e| e.va == va0))
        .ok_or_else(|| format!("no lifted image starts at 0x{va0:x}"))?;
    let name = img.name;
    let mut cases = 0u32;
    // 1. Structure (dispatch table + DF_ALIAS target/ptr/flags + mapped 0xE9 chain).
    let by_va: std::collections::HashMap<u64, (*const (), u8)> = img
        .slices
        .iter()
        .copied()
        .flatten()
        .map(|e| (e.va, (e.ptr, e.flags)))
        .collect();
    let mut native_before = 0;
    let mut aliases_checked = 0u32;
    for e in img.slices.iter().copied().flatten() {
        match crate::exe::lookup_va_only(e.va).map(super::body_of) {
            Some(p) if p == e.ptr as usize => {}
            Some(_) if super::kept_native(e.va) => native_before += 1,
            None if super::kept_native(e.va) && crate::dll_native::is_dll_chkstk(e.va) => native_before += 1,
            other => return Err(format!("{name}: 0x{:x} dispatches to {other:x?}, lifted 0x{:x}", e.va, e.ptr as usize)),
        }
        if e.flags & super::DF_ALIAS != 0 {
            if e.alias_rva == 0 {
                return Err(format!("{name}: DF_ALIAS entry 0x{:x} has zero alias_rva", e.va));
            }
            let target_va = img.base as u64 + e.alias_rva as u64;
            let Some(&(tptr, tflags)) = by_va.get(&target_va) else {
                return Err(format!("{name}: DF_ALIAS entry 0x{:x} targets non-lifted VA 0x{target_va:x}", e.va));
            };
            if tflags & super::DF_ALIAS != 0 {
                return Err(format!("{name}: DF_ALIAS entry 0x{:x} targets another DF_ALIAS 0x{target_va:x}", e.va));
            }
            if tptr != e.ptr {
                return Err(format!(
                    "{name}: DF_ALIAS entry 0x{:x} ptr {:p} != target 0x{target_va:x} ptr {:p}",
                    e.va, e.ptr, tptr
                ));
            }
            let mask = super::LEAF | super::XMM_RET;
            if (tflags & mask) != (e.flags & mask) {
                return Err(format!(
                    "{name}: DF_ALIAS entry 0x{:x} flags 0x{:x} != target 0x{target_va:x} flags 0x{:x}",
                    e.va,
                    e.flags & mask,
                    tflags & mask
                ));
            }
            verify_e9_chain(img.base, img.text_rva, img.text_len, e.va, target_va)
                .map_err(|why| format!("{name}: DF_ALIAS 0x{:x} -> 0x{target_va:x}: {why}", e.va))?;
            aliases_checked += 1;
        } else if e.alias_rva != 0 {
            return Err(format!("{name}: non-alias entry 0x{:x} has non-zero alias_rva 0x{:x}", e.va, e.alias_rva));
        }
        cases += 1;
    }
    let structural = cases;
    // 2. Fuzz the entries that dispatch to their lifted body (once per VA).
    let mut seen = std::collections::HashSet::new();
    let entries: Vec<super::Lifted> = img
        .slices
        .iter()
        .copied()
        .flatten()
        .filter(|e| seen.insert(e.va) && crate::exe::lookup_va_only(e.va).map(super::body_of) == Some(e.ptr as usize))
        .map(|e| super::Lifted { va: e.va, ptr: e.ptr, flags: e.flags, alias_rva: e.alias_rva })
        .collect();
    let header = format!("{structural} entries dispatched ({native_before} kept native, {aliases_checked} E9 aliases)");
    // Focus mode (`AW2_DLL_DIFF_FN`) on another image: structure only.
    if std::env::var("AW2_DLL_DIFF_FN").is_ok() && focus_for(name).is_none() {
        log::info!("dll_lift difftest: {name}: {header}; fuzz skipped (AW2_DLL_DIFF_FN names another image)");
        return Ok(structural);
    }
    let fuzz = fuzz_entries("dll_lift", name, img.base, &writable_sections(img.base), &exports(img.base), &entries, &header)?;
    Ok(structural + fuzz)
}

/// Fuzz `entries` (module docs, "2. Fuzz"): each entry's guest x86 at `va` against its lifted
/// body `ptr`, [`TRIALS`] trials (non-leaves stop early), every run in a forked child.
/// `tag` / `name` prefix the log rows (`<tag> difftest fn\t<name>\t+0x<va - base>\t<result>`),
/// `secs` are the writable sections whose digest is compared, `exported` maps `va - base` to
/// export names (typed signatures), `header` starts the summary line. Returns the trial count.
unsafe fn fuzz_entries(
    tag: &str,
    name: &str,
    base: usize,
    secs: &[(usize, usize)],
    exported: &std::collections::HashMap<usize, String>,
    entries: &[super::Lifted],
    header: &str,
) -> Result<u32, String> {
    let mut cases = 0u32;
    let snap: Vec<Vec<u8>> = secs.iter().map(|&(s, e)| core::slice::from_raw_parts(s as *const u8, e - s).to_vec()).collect();
    let arena_addr = pinned_arena(name);
    det_heap();
    let _ = stub_variant(0);
    let _ = stub_variant(1);
    let _ = stub_variant(2);
    let _ = stub_variant(4);
    let mut init = vec![0u8; ARENA];
    let (mut both_died, mut both_timeout, mut inconclusive, mut equal, mut nondet) = (0u32, 0u32, 0u32, 0u32, 0u32);
    let (mut slow_guest, mut slow_guest_at) = (0u32, Vec::<String>::new());
    let (mut seh_gaps, mut seh_mm) = (Vec::<String>::new(), Vec::<String>::new());
    let (mut seh_unraised_n, mut seh_unraised_at) = (0u32, Vec::<String>::new());
    let (mut seh_precise_n, mut seh_precise_at) = (0u32, Vec::<String>::new());
    let (mut seh_refuted_n, mut seh_refuted_at) = (0u32, Vec::<String>::new());
    let (mut fresh, mut fresh_at) = (0u32, Vec::<String>::new());
    let (mut guest_fresh, mut guest_fresh_at) = (0u32, Vec::<String>::new());
    let (mut slow_lifted, mut slow_lifted_at) = (0u32, Vec::<String>::new());
    // [non-leaf, leaf]
    let mut fuzzed = [0u32; 2];
    let mut unverified: [Vec<String>; 2] = [Vec::new(), Vec::new()];
    let mut incon_at: Vec<String> = Vec::new();
    let (mut narrow, mut narrow_at) = (0u32, Vec::<String>::new());
    // Listed classes (module docs): trials, and one example per function.
    let (mut noncode, mut noncode_at) = (0u32, Vec::<String>::new());
    let (mut rax_stack, mut rax_stack_at) = (0u32, Vec::<String>::new());
    let (mut ret_dead, mut ret_dead_at) = (0u32, Vec::<String>::new());
    let rets = ret_masks();
    let float_ret = float_rets();
    let xmm_ret_dead = xmm_dead();
    let (mut nan, mut nan_at) = (0u32, Vec::<String>::new());
    let (mut null_exit, mut null_at) = (0u32, Vec::<String>::new());
    let (mut null_call, mut null_call_at) = (0u32, Vec::<String>::new());
    let (mut memstack, mut memstack_at) = (0u32, Vec::<String>::new());
    let (mut retslot, mut retslot_at) = (0u32, Vec::<String>::new());
    let (mut thread_fault, mut thread_at) = (0u32, Vec::<String>::new());
    let (mut threw_equal, mut threw_at) = (0u32, Vec::<String>::new());
    let (mut threw_residue, mut threw_residue_at) = (0u32, Vec::<String>::new());
    let (mut threw_memstack, mut threw_memstack_at) = (0u32, Vec::<String>::new());
    let (mut threw_mismatch, mut threw_mismatch_at) = (0u32, Vec::<String>::new());
    let (mut threw_unrec, mut threw_unrec_at) = (0u32, Vec::<String>::new());
    let (mut gs_residue, mut gs_at) = (0u32, Vec::<String>::new());
    let cookie = security_cookie(base);
    let (mut overrun, mut overrun_at) = (0u32, Vec::<String>::new());
    // Set up here, before the forks: every child's lifted run uses this thread's arena.
    let arena_end = crate::lstack::arena_end() as u64;
    let (big_lo, big_hi) = crate::simd_ops::big_frame_arena();
    BIG_LO.store(big_lo as u64, core::sync::atomic::Ordering::Relaxed);
    BIG_HI.store(big_hi as u64, core::sync::atomic::Ordering::Relaxed);
    // The emulator's stack top on this thread (the children's guest runs start just below it).
    let guest_top = guest_entry_rsp();
    // ... and the translator's code region (see `emu::jit_warm`): no child maps its own.
    crate::emu::jit_warm();
    let (mut stack_oob, mut oob_at) = (0u32, Vec::<String>::new());
    let (mut unseeded, mut unseeded_at) = (0u32, Vec::<String>::new());
    let mut mismatches: Vec<String> = Vec::new();
    let focus = focus_for(name);
    let vas_filter: Option<std::collections::HashSet<usize>> = std::env::var("AW2_DLL_DIFF_VAS")
        .ok()
        .map(|file| {
            std::fs::read_to_string(&file)
                .unwrap_or_default()
                .split(|c: char| c == ',' || c.is_whitespace())
                .filter(|t| !t.is_empty())
                .filter_map(|t| {
                    let mut it = t.trim().split(':');
                    let first = it.next()?;
                    let tok = match it.next() {
                        Some(off_s) => {
                            if !name.to_ascii_lowercase().contains(&first.to_ascii_lowercase()) {
                                return None;
                            }
                            off_s
                        }
                        None => first,
                    };
                    usize::from_str_radix(tok.trim_start_matches("0x").trim_start_matches("+0x"), 16).ok()
                })
                .collect()
        });
    let mut typed = 0u32;
    let wall = std::env::var("AW2_DLL_DIFF_WALL_S").ok().and_then(|s| s.parse::<u64>().ok());
    let t0 = std::time::Instant::now();
    let mut not_fuzzed = 0u32;
    let shard = std::env::var("AW2_DLL_DIFF_SHARD").ok().and_then(|s| {
        let (k, n) = s.split_once('/')?;
        let (k, n) = (k.trim().parse::<usize>().ok()?, n.trim().parse::<usize>().ok()?);
        (n > 0 && k < n).then_some((k, n))
    });
    let mut ordinal = 0usize;
    for e in entries {
        let off = e.va as usize - base;
        if let Some(ref set) = vas_filter {
            if !set.contains(&off) && !set.contains(&(e.va as usize)) {
                continue;
            }
        }
        ordinal += 1;
        if shard.is_some_and(|(k, n)| (ordinal - 1) % n != k) {
            continue;
        }
        if wall.is_some_and(|w| t0.elapsed().as_secs() >= w) {
            not_fuzzed += 1;
            if focus.is_none() {
                log::info!("{tag} difftest fn\t{name}\t+0x{:x}\tnot_fuzzed\t", e.va as usize - base);
            }
            continue;
        }
        if (fuzzed[0] + fuzzed[1]) % 50 == 49 {
            log::info!("{tag} difftest: {name}: {} functions fuzzed ({}s)", fuzzed[0] + fuzzed[1] + 1, t0.elapsed().as_secs());
        }
        let sig = exported.get(&off).and_then(|n| signature(n));
        typed += sig.is_some() as u32;
        if focus.as_ref().is_some_and(|f| f.0 != off) {
            continue;
        }
        let leaf = e.flags & LEAF != 0;
        let (mut fn_equal, mut fn_timeouts, mut fn_mismatch, mut fn_narrow) = (0u32, 0u32, false, false);
        let (mut fn_noncode, mut fn_stack, mut fn_nan, mut fn_null) = (false, false, false, false);
        let mut fn_nullcall = false;
        let mut fn_memstack = false;
        let mut fn_retslot = false;
        let mut fn_ret_dead = false;
        let live_rax = rets.get(&e.va).copied().unwrap_or(u64::MAX);
        let xmm_ret = e.flags & XMM_RET != 0 || float_ret.contains(&e.va);
        let dead_xmm = xmm_ret_dead.contains(&e.va) && !float_ret.contains(&e.va);
        let mut fn_thread = false;
        let mut fn_threw = false;
        let mut fn_threw_residue = false;
        let mut fn_threw_memstack = false;
        let mut fn_threw_unrec = false;
        let mut fn_gs = false;
        let mut fn_overrun = false;
        let mut fn_oob = false;
        let mut fn_unseeded = false;
        // seh rows (precise mode, module docs): landed_both, fault_both_unhandled, mismatch,
        // no_fault_injected, seh_unraised.
        let mut fn_seh = [0u32; 5];
        let mut fn_seh_unraised = false;
        let is_alias = e.flags & super::DF_ALIAS != 0;
        let trial_limit = if focus.is_none() && !precise_faults_env() && is_alias { 1 } else { TRIALS };
        for t in 0..trial_limit {
            if focus.is_none() && !leaf && (fn_equal >= NONLEAF_ENOUGH || fn_timeouts >= NONLEAF_MAX_TIMEOUTS) {
                break;
            }
            let mut rng = Rng(e.va ^ ((t as u64) << 48) ^ 0xA5A5_0000);
            fill_arena(&mut rng, &mut init, arena_addr);
            // Documented C signature: every other trial uses typed arguments.
            let mut gpr = match sig {
                Some(s) if t % 2 == 0 => typed_args(&mut rng, s, &mut init, arena_addr),
                _ => args_for(&mut rng, arena_addr, t),
            };
            apply_overrides(&mut gpr, &mut init, arena_addr);
            let gpr = gpr;
            let mut xmm = [0u128; 4];
            for x in xmm.iter_mut() {
                *x = (((rng.next() % 1000) as f64) / 7.0).to_bits() as u128;
            }
            if focus.as_ref().is_some_and(|f| f.1.is_some_and(|ft| ft != t)) {
                continue;
            }
            let win = stack_window(e.va, t, arena_addr, &gpr);
            if focus.is_some() {
                log_inputs(name, off, t, &gpr, &init, arena_addr);
                log::info!("{tag} difftest focus: {name} +0x{off:x} trial {t}: stack args 9..{WIN_ARGS} {:x?}", &win[8..WIN_ARGS]);
            }
            let traps = trap_targets(&init, &[&gpr[..], &win[8..WIN_ARGS]].concat());
            let (secs, snap, init, traps) = (&secs, &snap, &init, &traps);
            // Focus mode: each run's final arena goes to `$AW2_LOG_DIR/arena_t<t>_<n>_<g|l>.bin`.
            let dump_dir = focus.is_some().then(|| std::env::var("AW2_LOG_DIR").unwrap_or_else(|_| ".".into()));
            let dump_dir = &dump_dir;
            // Runs in the child: restore inputs, call one side, digest.
            let run = move |guest: bool| -> ([u64; 4], [u64; 4]) {
                // rand_s / BCryptGenRandom / RtlGenRandom: the same stream for every run of the
                // trial (win32_rt::det_entropy), so a generator seeded inside the call (V8
                // Isolate::random_number_generator) is a function of the trial too.
                win32_rt::det_entropy::reset((e.va ^ ((t as u64) << 40) ^ 0x5EED_0000_0000_0000) | 1);
                det_heap_restart();
                crate::exe::register_native_vas(traps.iter().map(|&v| (v, noncode_trap as *const () as usize)));
                for (&(s, _), b) in secs.iter().zip(snap) {
                    core::ptr::copy_nonoverlapping(b.as_ptr(), s as *mut u8, b.len());
                }
                apply_mem_overrides(secs, arena_addr);
                core::ptr::copy_nonoverlapping(init.as_ptr(), arena_addr as *mut u8, ARENA);
                // Fresh copy per run: a callee may write its incoming area.
                let w = win;
                // A C++ exception that left the function (caught by the harness boundary).
                let mut threw: Option<crate::eh_rt::host_try::Caught> = None;
                let (r, x) = if guest {
                    match guest_run(e.va, gpr, xmm, &w) {
                        Ok(Some(rx)) => rx,
                        Ok(None) => return ([NULLEXIT_TAG, 0, 0, 0], [CANON_TAG, 0, 0, 0]),
                        Err(c) => {
                            threw = Some(c);
                            (0, 0)
                        }
                    }
                } else {
                    let ltop = lifted_stack_top();
                    LIFTED_SP.store(ltop, core::sync::atomic::Ordering::Relaxed);
                    let zmode = CHILD_ZERO_FRAME.load(core::sync::atomic::Ordering::Relaxed);
                    let mut w_lifted = w;
                    if zmode == 3 {
                        for slot in &mut w_lifted[(LIFTED_ARGS_MIN - 1)..WIN] {
                            *slot = 0;
                        }
                    }
                    let stack: [u64; 8] = w_lifted[8..16].try_into().unwrap();
                    publish_window(&w_lifted);
                    let ret_pc = if zmode == 3 { stub_variant(2).2 } else { stub_ret() };
                    if zmode == 3 && ltop >= 24 * 8 {
                        core::ptr::write_volatile((ltop - 24 * 8) as *mut u64, ret_pc);
                    }
                    let rx = crate::eh_rt::host_try::catch_cxx(throw_note, || {
                        // The guest run's caller frame is the stub's (`call r11`); the lifted
                        // run's stand-in for it, so a stack backtrace taken inside sees the
                        // same caller (as `frame_image` models the stub's return slot).
                        let mut sf = win32_rt::unwind::EhHostFrame::new(stub_frame_handler, core::ptr::null_mut()).with_pc(ret_pc);
                        let _sf = sf.push();
                        crate::emu::call_host_dispatched(e.va, e.ptr as usize, crate::emu::DF_VALID | crate::emu::DF_HOST, gpr, xmm, stack)
                    });
                    if zmode == 3 && ltop >= 24 * 8 {
                        core::ptr::write_volatile((ltop - 24 * 8) as *mut u64, 0);
                    }
                    // A body without a lifted frame never takes the window.
                    crate::emu::set_incoming_rsp(0, 0);
                    match rx {
                        Ok(rx) => rx,
                        Err(c) => {
                            threw = Some(c);
                            (0, 0)
                        }
                    }
                };
                let data = secs
                    .iter()
                    .map(|&(s, en)| super::super::fnv1a(core::slice::from_raw_parts(s as *const u8, en - s)))
                    .fold(0u64, |a, h| a.rotate_left(7) ^ h);
                let ar = super::super::fnv1a(core::slice::from_raw_parts(arena_addr as *const u8, ARENA));
                let data_c = secs.iter().map(|&(s, en)| nan_canon_hash(s as *const u8, en - s)).fold(0u64, |a, h| a.rotate_left(7) ^ h);
                let ar_c = nan_canon_hash(arena_addr as *const u8, ARENA);
                if let Some(d) = dump_dir.as_deref() {
                    dump_arena(d, t, guest, arena_addr);
                }
                let mem = if guest {
                    let mut g = GUEST_ARENA.lock().unwrap_or_else(|e| e.into_inner());
                    if g.is_empty() {
                        g.extend_from_slice(core::slice::from_raw_parts(arena_addr as *const u8, ARENA));
                        shared_guest_arena_publish(&g);
                    }
                    0
                } else {
                    memstack_check(arena_addr, &w)
                };
                if let Some(c) = threw {
                    let ti = if c.rec.number_parameters >= 3 { c.rec.information[2] } else { 0 };
                    let obj = c.note[0];
                    let xr: [u8; 32] = core::mem::transmute([THROWX_TAG, c.note[1], data, 0]);
                    write(CHILD_WFD.load(core::sync::atomic::Ordering::Relaxed), xr.as_ptr(), 32);
                    return ([THROW_TAG, ti, obj, ar.rotate_left(17) ^ data], [CANON_TAG, ar_c, data_c, mem]);
                }
                ([r, if xmm_ret { x as u64 } else { 0 }, ar, data], [CANON_TAG, ar_c, data_c, mem])
            };
            // Guest, lifted, guest: if the two guest runs differ the function is not a
            // function of the restored inputs (heap addresses, time, state outside the image);
            // a time-dependent result of the lifted run is bracketed by the two guest runs.
            shared_guest_arena_reset();
            let (recs, meta, exited) = in_child(TIMEOUT_MS, &|emit: &dyn Fn([u64; 4])| {
                for guest in [true, false, true] {
                    let (r, c) = run(guest);
                    emit(r);
                    emit(c);
                }
            });
            let side = |i: usize, recs: &[[u64; 4]], exited: Option<bool>| match (recs.get(i), exited) {
                (Some(r), _) if r[0] == FAULT_TAG && r[3] & 1 == 1 => Outcome::ThreadFault(r[1]),
                (Some(r), _) if r[0] == FAULT_TAG => Outcome::Fault(r[1]),
                (Some(r), _) if r[0] == TRAP_TAG => Outcome::NonCode,
                (Some(r), _) if r[0] == THROW_TAG => Outcome::Threw([r[1], r[2], r[3]]),
                (Some(r), _) if r[0] == UNREC_TAG => Outcome::ThrewUnrecorded(r[1]),
                (Some(r), _) if r[0] == NULLEXIT_TAG => Outcome::NullExit,
                (Some(r), _) => Outcome::Ret(*r),
                (None, None) => Outcome::Timeout,
                (None, Some(_)) => Outcome::Died,
            };
            let mut g = side(0, &recs[..], exited);
            let mut g_seh = meta.sehfault & 1 != 0;
            let mut g_null = meta.nullcall & 1 != 0;
            let g_loose = meta.throw_loose[0];
            let g_data = meta.throw_data[0];
            // One side alone in a fresh child with its own budget.
            let alone = |guest: bool, budget: u64| {
                let (r2, m2, ex2) = in_child(budget, &|emit: &dyn Fn([u64; 4])| {
                    let (r, c) = run(guest);
                    emit(r);
                    emit(c);
                });
                // A lifted run alone: its loose throw digest where the shared child has it.
                let mut m2 = m2;
                if !guest {
                    m2.throw_loose[1] = m2.throw_loose[0];
                    m2.throw_data[1] = m2.throw_data[0];
                    m2.memstack = m2.memstack0;
                    m2.sehfault |= (m2.sehfault & 1) << 1;
                    m2.sehaddr[1] = m2.sehaddr[0];
                }
                (side(0, &r2[..], ex2), m2, m2.nullcall & 1 != 0)
            };
            let (l, meta, l_null) = match g {
                Outcome::Ret(a) => {
                    let l = side(1, &recs[..], exited);
                    if matches!(l, Outcome::Ret(_)) && side(2, &recs[..], exited) != Outcome::Ret(a) {
                        nondet += 1;
                        cases += 1;
                        continue;
                    }
                    // The child's budget covers all three runs: a slow guest run leaves the lifted
                    // run little of it, so a lifted timeout here is re-run alone. A lifted crash
                    // after a returning guest run is re-run alone too: the guest run may have left
                    // host state outside the restored image / arena broken for the next run (it
                    // freed or realloc'd a fuzzed pointer into the host heap: v8 +0x8740, lifted
                    // SIGTRAP in the malloc freelist). Alone, the lifted run starts from the same
                    // fresh fork the guest run did; that outcome is classified (listed below).
                    if l == Outcome::Timeout {
                        // Slow lifted (module docs): x SLOW_GUEST_X budget alone.
                        let r2 = alone(false, TIMEOUT_MS * slow_x());
                        slow_lifted += 1;
                        if slow_lifted_at.len() < 32 {
                            slow_lifted_at.push(format!("+0x{off:x}:t{t}:{}", if r2.0 == Outcome::Timeout { "timeout" } else { "ok" }));
                        }
                        if matches!(r2.0, Outcome::Ret(_)) && r2.0 != Outcome::Ret(a) {
                            let (rg2, _, exg2) = in_child(TIMEOUT_MS * 2, &|emit: &dyn Fn([u64; 4])| {
                                for _ in 0..2 {
                                    let (r, c) = run(true);
                                    emit(r);
                                    emit(c);
                                }
                            });
                            let (gf0, gf1) = (side(0, &rg2[..], exg2), side(1, &rg2[..], exg2));
                            if gf0 != Outcome::Timeout && gf1 != Outcome::Timeout && (gf0 != Outcome::Ret(a) || gf1 != Outcome::Ret(a)) {
                                nondet += 1;
                                cases += 1;
                                continue;
                            }
                        }
                        r2
                    } else if l.crashed() {
                        let r2 = alone(false, TIMEOUT_MS);
                        if fresh_at.len() < 32 {
                            fresh_at.push(format!("+0x{off:x}:t{t}:{}", if r2.0.crashed() { "crashed" } else { "ok" }));
                        }
                        fresh += 1;
                        if focus.is_some() {
                            log::info!("{tag} difftest focus: {name} +0x{off:x} trial {t}: lifted {l:x?} after the guest run; alone {:x?}", r2.0);
                        }
                        if matches!(r2.0, Outcome::Ret(_)) && r2.0 != Outcome::Ret(a) {
                            // `l` crashed before run 2 (`guest again`) in the shared child (e.g.
                            // `v8.dll +0xe7b6f0` mutating `msvcp140.dll`'s `.data` + storing an OS
                            // `mmap` address): verify two consecutive guest runs agree.
                            let (rg2, _, exg2) = in_child(TIMEOUT_MS * 2, &|emit: &dyn Fn([u64; 4])| {
                                for _ in 0..2 {
                                    let (r, c) = run(true);
                                    emit(r);
                                    emit(c);
                                }
                            });
                            let (gf0, gf1) = (side(0, &rg2[..], exg2), side(1, &rg2[..], exg2));
                            if gf0 != Outcome::Timeout && gf1 != Outcome::Timeout && (gf0 != Outcome::Ret(a) || gf1 != Outcome::Ret(a)) {
                                nondet += 1;
                                cases += 1;
                                continue;
                            }
                        }
                        r2
                    } else {
                        (l, meta, meta.nullcall & 2 != 0)
                    }
                }
                _ => alone(false, TIMEOUT_MS),
            };
            // Slow guest (see the module docs): the emulator runs a long loop many times slower
            // than the lifted body, so a guest timeout against a lifted crash inside the
            // emulator's address range, or against a lifted return, is re-run alone with
            // SLOW_GUEST_X x the budget and classified by what the guest then does (both
            // crashed, compared with the lifted result, or still a timeout: a mismatch).
            let l_ret = matches!(l, Outcome::Ret(_));
            if g == Outcome::Timeout && (l_ret || (l.crashed() && !matches!(l, Outcome::Fault(a) if !EMU_VALID.contains(&a)))) {
                slow_guest += 1;
                let (g2, m2g, n2) = alone(true, TIMEOUT_MS * slow_x() * if precise_faults_env() { PRECISE_SLOW_X } else { 1 });
                (g, g_null, g_seh) = (g2, n2, m2g.sehfault & 1 != 0);
                if slow_guest_at.len() < 32 && (l_ret || g != Outcome::Timeout) {
                    let lw = if l_ret { "ret" } else { "crashed" };
                    let gw = if g == Outcome::Timeout { "timeout" } else if g.crashed() { "crashed" } else { "ok" };
                    slow_guest_at.push(format!("+0x{off:x}:t{t}:lifted {lw}:guest alone {gw}"));
                }
            }
            // Fresh-child guest re-run (module docs): a guest fault against a lifted return is
            // repeated guest-alone in a fresh child (the same starting state); its outcome is
            // what gets classified.
            if l_ret && matches!(g, Outcome::Fault(_)) {
                let a0 = if let Outcome::Fault(a) = g { a } else { 0 };
                let (g2, m2g, n2) = alone(true, TIMEOUT_MS);
                guest_fresh += 1;
                if guest_fresh_at.len() < 32 {
                    let site = if arena_end != 0 && a0.wrapping_sub(arena_end) < 0x10000 {
                        format!("lstack_end+0x{:x}", a0 - arena_end)
                    } else {
                        format!("0x{a0:x}")
                    };
                    let gw = if g2 == Outcome::Timeout { "timeout" } else if g2.crashed() { "crashed" } else { "ok" };
                    guest_fresh_at.push(format!("+0x{off:x}:t{t}:fault@{site}:alone {gw}"));
                }
                (g, g_null, g_seh) = (g2, n2, m2g.sehfault & 1 != 0);
            }
            if focus.is_some() {
                log::info!(
                    "{tag} difftest focus: {name} +0x{off:x} trial {t}: guest {g:x?} / lifted {l:x?} / guest again {:x?} (rax, xmm0, arena, data); {meta:x?}; {} trap targets; null call guest {g_null} lifted {l_null}",
                    side(2, &recs[..], exited),
                    traps.len()
                );
            }
            // seh row kind of this trial (precise mode): an access violation raised on both
            // sides that both survived (returned / threw) = landed_both unless the match below
            // finds a MISMATCH; both ended in an unhandled fault = fault_both_unhandled; a
            // one-sided SEH fault = mismatch; none = no_fault_injected.
            let l_seh = meta.sehfault & 2 != 0;
            let seh_kind = match (g_seh, l_seh) {
                (false, false) => 3,
                _ if g.crashed() && l.crashed() => 1,
                (true, true) if g != Outcome::Timeout && l != Outcome::Timeout => 0,
                _ => 2,
            };
            // seh_unraised (module docs): the lifted run raised an access violation for a
            // guarded access at `la` (and its `__except` claimed it or not) where the guest run
            // raised none. Exact: the guest faulted unhandled at the same address (an unmapped
            // address the emulator cannot dispatch, precise mode or not). Loose (default mode
            // only, la outside [0x10000, 2^47): the emulator read 0 there / dropped the access
            // instead of faulting): the guest then ended on a null-page access (Fault(b) with b
            // in the null page: the child's `emu_unhandled_exception` re-faults at
            // max(address, 0x10); or Died), or returned / timed out / called a non-code target.
            // A loose case is confirmed by re-running the guest alone with precise faults
            // (`CHILD_FORCE_PRECISE`): seh_unraised (precise-confirmed) only if that run raised
            // at la too and its outcome (result / throw and arena, data digests) equals the
            // lifted one; otherwise refuted: a MISMATCH when the lifted run survived (it may
            // have landed in a handler from a wrong address), else classified as usual.
            let la = meta.sehaddr[1];
            let one_sided = l_seh && !g_seh;
            let exact = one_sided && matches!(g, Outcome::Fault(b) if b == la);
            let loose = one_sided
                && !exact
                && !precise_faults_env()
                && !EMU_VALID.contains(&la)
                && match g {
                    Outcome::Fault(b) => b < 0x1_0000,
                    Outcome::Died | Outcome::Ret(_) | Outcome::Timeout | Outcome::NonCode => true,
                    _ => false,
                };
            let (mut confirmed, mut refuted) = (false, false);
            if loose {
                CHILD_FORCE_PRECISE.store(true, core::sync::atomic::Ordering::Relaxed);
                let (gp, mp, _) = alone(true, TIMEOUT_MS);
                CHILD_FORCE_PRECISE.store(false, core::sync::atomic::Ordering::Relaxed);
                confirmed = mp.sehfault & 1 != 0 && mp.sehaddr[0] == la && gp == l;
                refuted = !confirmed;
                if confirmed {
                    seh_precise_n += 1;
                    if seh_precise_at.len() < 64 {
                        seh_precise_at.push(format!("+0x{off:x}:t{t}:0x{la:x}"));
                    }
                } else {
                    seh_refuted_n += 1;
                    if seh_refuted_at.len() < 64 {
                        let ga = if mp.sehfault & 1 != 0 { format!("0x{:x}", mp.sehaddr[0]) } else { "none".into() };
                        seh_refuted_at.push(format!("+0x{off:x}:t{t}:0x{la:x}:precise guest {gp:x?} raised {ga}"));
                    }
                }
                if focus.is_some() {
                    log::info!("{tag} difftest focus: {name} +0x{off:x} trial {t}: seh_unraised precise re-run: guest {gp:x?}, raised {} at 0x{:x}; lifted {l:x?} raised at 0x{la:x}: {}", mp.sehfault & 1 != 0, mp.sehaddr[0], if confirmed { "confirmed" } else { "refuted" });
                }
            }
            let seh_unraised = exact || confirmed;
            match (g, l) {
                _ if seh_unraised => {
                    seh_unraised_n += 1;
                    if !fn_seh_unraised {
                        fn_seh_unraised = true;
                        seh_unraised_at.push(format!("+0x{off:x}:t{t}:0x{la:x}"));
                    }
                }
                _ if refuted && !l.crashed() => {
                    let msg = format!(
                        "+0x{:x} ({}) trial {t} args {gpr:x?}: guest {g:x?} / lifted {l:x?} (rax, xmm0, arena, data); seh_unraised refuted by the precise re-run (lifted raised at 0x{la:x})",
                        e.va as usize - base,
                        if leaf { "leaf" } else { "non-leaf" }
                    );
                    log::error!("{tag} difftest: {name}: MISMATCH {msg}");
                    mismatches.push(msg);
                    fn_mismatch = true;
                    fn_seh[2] += 1;
                    break;
                }
                // A thread started by a run (e.g. `Thread::Start`) faulted, typically on arena
                // state the harness had since restored for the next run: the record ends the
                // child during whichever run is current, so neither side is to blame. Listed,
                // never equal.
                // The guest run jumped to rip 0 (a tail jmp through a zero word): natively a fault
                // there; the lifted side cannot be compared past it. Listed, never equal.
                (Outcome::NullExit, _) => {
                    null_exit += 1;
                    if !fn_null {
                        fn_null = true;
                        null_at.push(format!("+0x{off:x}:t{t}"));
                    }
                }
                // The guest run faulted at a wild stack-relative address (see "stack_oob" in the
                // module docs). Listed, never equal.
                (Outcome::Fault(a), l) if l != Outcome::Timeout && !l.crashed() && guest_top != 0 && {
                    let block = guest_top.saturating_sub(GUEST_STACK_BYTES + 0x10000)..guest_top + 0x10000;
                    !block.contains(&a) && a.abs_diff(guest_top) < 1 << 34
                } =>
                {
                    stack_oob += 1;
                    if !fn_oob {
                        fn_oob = true;
                        let (s, d) = if a >= guest_top { ('+', a - guest_top) } else { ('-', guest_top - a) };
                        oob_at.push(format!("+0x{off:x}:t{t}:top{s}0x{d:x}"));
                    }
                }
                // The lifted run walked up off its frames to the end of its frame arena (see
                // "stack_top_overrun" in the module docs). Listed, never equal.
                (g, Outcome::Fault(a)) if !g.crashed() && arena_end != 0 && a.wrapping_sub(arena_end) < 0x1000 => {
                    overrun += 1;
                    if !fn_overrun {
                        fn_overrun = true;
                        overrun_at.push(format!("+0x{off:x}:t{t}"));
                    }
                }
                (Outcome::ThreadFault(_), _) | (_, Outcome::ThreadFault(_)) => {
                    thread_fault += 1;
                    if !fn_thread {
                        fn_thread = true;
                        thread_at.push(format!("+0x{off:x}:t{t}"));
                    }
                }
                // Both sides called a non-code target: natively a fault on both. A one-sided
                // non-code call is a mismatch (default arm).
                (Outcome::NonCode, Outcome::NonCode) => {
                    noncode += 1;
                    if !fn_noncode {
                        fn_noncode = true;
                        noncode_at.push(format!("+0x{off:x}"));
                    }
                }
                // C++ throws (module docs): both runs threw the same exception (ThrowInfo, object)
                // and left the same state after the unwinding. Listed, never equal. Both threw
                // and something differs: "threw_mismatch", listed here and a mismatch (default
                // arm). A throw on one side only is a mismatch (default arm).
                (Outcome::Threw(a), Outcome::Threw(b)) if a == b => {
                    threw_equal += 1;
                    if !fn_threw {
                        fn_threw = true;
                        threw_at.push(format!("+0x{off:x}:t{t}:ti=0x{:x}", a[0]));
                    }
                }
                // Same ThrowInfo and state; the objects differ only in qword high dwords that
                // are each the high half of an address in that run's own stacks (stale stack
                // bytes in uninitialized padding, e.g. after error_code::_Myval in
                // std::system_error): "threw_stack_residue". Listed, never equal.
                (Outcome::Threw(a), Outcome::Threw(b))
                    if a[0] == b[0] && a[2] == b[2] && g_loose != 0 && g_loose == meta.throw_loose[1] =>
                {
                    threw_residue += 1;
                    if !fn_threw_residue {
                        fn_threw_residue = true;
                        threw_residue_at.push(format!("+0x{off:x}:t{t}:ti=0x{:x}", a[0]));
                    }
                }
                // Same ThrowInfo and object, same data sections; the arena after the unwinding
                // differs from the guest run's only in qwords that hold, on each side, an
                // address in that run's own stacks (see `MEMSTACK_ONLY`; v8 +0x4d35e0: one arena
                // qword = the guest emulator stack vs the lifted frame arena): "threw_mem_stack".
                // Listed, never equal.
                (Outcome::Threw(a), Outcome::Threw(b))
                    if a[..2] == b[..2] && g_data != 0 && g_data == meta.throw_data[1] && meta.memstack == MEMSTACK_ONLY =>
                {
                    threw_memstack += 1;
                    if !fn_threw_memstack {
                        fn_threw_memstack = true;
                        threw_memstack_at.push(format!("+0x{off:x}:t{t}:ti=0x{:x}", a[0]));
                    }
                }
                // The exception did not reach the boundary on either side (same ThrowInfo):
                // "threw_unrecorded (harness)". Listed, never equal.
                (Outcome::ThrewUnrecorded(a), Outcome::ThrewUnrecorded(b)) if a == b => {
                    threw_unrec += 1;
                    if !fn_threw_unrec {
                        fn_threw_unrec = true;
                        threw_unrec_at.push(format!("+0x{off:x}:t{t}:ti=0x{a:x}"));
                    }
                }
                (Outcome::Ret(a), Outcome::Ret(b)) if a == b => {
                    equal += 1;
                    fn_equal += 1;
                }
                // Only the emulator's ABI narrowing of rax differs (see [`abi_narrow`]): not
                // counted as equal; every such function is listed.
                (Outcome::Ret(a), Outcome::Ret(b)) if a[1..] == b[1..] && abi_narrow(a[0], b[0]).is_some() => {
                    narrow += 1;
                    if !fn_narrow {
                        fn_narrow = true;
                        narrow_at.push(format!("+0x{off:x}:{:x}/{:x}", a[0], b[0]));
                    }
                }
                // rax and/or xmm0 is a stack address (or stack-padding high-dword residue) of
                // each run, all else equal.
                (Outcome::Ret(a), Outcome::Ret(b))
                    if a[2..] == b[2..]
                        && (a[0] == b[0] || is_stack_or_pad(a[0], b[0], &meta))
                        && (a[1] == b[1] || is_stack_or_pad(a[1], b[1], &meta)) =>
                {
                    rax_stack += 1;
                    if !fn_stack {
                        fn_stack = true;
                        let i = if a[0] != b[0] { 0 } else { 1 };
                        rax_stack_at.push(format!("+0x{off:x}:{}{:x}/{:x}", if i == 1 { "xmm0 " } else { "" }, a[i], b[i]));
                    }
                }
                // rax = the image's GS cookie xor each run's own stack address, all else equal
                // (see "gs_residue" in the module docs).
                (Outcome::Ret(a), Outcome::Ret(b))
                    if a[1..] == b[1..]
                        && cookie != 0
                        && in_either_stack(a[0] ^ cookie, &meta)
                        && in_either_stack(b[0] ^ cookie, &meta) =>
                {
                    gs_residue += 1;
                    if !fn_gs {
                        fn_gs = true;
                        gs_at.push(format!("+0x{off:x}:{:x}/{:x}", a[0] ^ cookie, b[0] ^ cookie));
                    }
                }
                // Only rax bits / an xmm0 that no x86 caller reads differ (see [`ret_masks`],
                // [`xmm_dead`]).
                (Outcome::Ret(a), Outcome::Ret(b))
                    if a[2..] == b[2..] && (a[1] == b[1] || dead_xmm) && (a[0] ^ b[0]) & live_rax == 0 =>
                {
                    ret_dead += 1;
                    if !fn_ret_dead {
                        fn_ret_dead = true;
                        if a[0] != b[0] {
                            ret_dead_at.push(format!("+0x{off:x}:{:x}/{:x}", a[0], b[0]));
                        } else {
                            ret_dead_at.push(format!("+0x{off:x}:xmm0 {:x}/{:x}", a[1], b[1]));
                        }
                    }
                }
                // x86 vs AArch64 generated NaN in rax / xmm0 / memory, all else equal (rax compared
                // under the caller liveness mask, as `ret_dead`).
                (Outcome::Ret(a), Outcome::Ret(b))
                    if nan_sign_only(
                        &[a[0] & live_rax, a[1], a[2], a[3]],
                        &[b[0] & live_rax, b[1], b[2], b[3]],
                        meta.canon[0],
                        meta.canon[1],
                    ) =>
                {
                    nan += 1;
                    if !fn_nan {
                        fn_nan = true;
                        nan_at.push(format!("+0x{off:x}:{:x}/{:x}", if a[0] != b[0] { a[0] } else { a[1] }, if a[0] != b[0] { b[0] } else { b[1] }));
                    }
                }
                // Only arena stack addresses / stack-padding high-dword residue differ (`rax` /
                // `xmm0` equal or themselves stack addresses / stack-padding residue on both sides).
                // Verified in the child (see `MEMSTACK_ONLY`).
                (Outcome::Ret(a), Outcome::Ret(b))
                    if a[2] != b[2]
                        && a[3] == b[3]
                        && meta.memstack == MEMSTACK_ONLY
                        && (a[0] == b[0] || is_stack_or_pad(a[0], b[0], &meta))
                        && (a[1] == b[1] || is_stack_or_pad(a[1], b[1], &meta)) =>
                {
                    memstack += 1;
                    if !fn_memstack {
                        fn_memstack = true;
                        memstack_at.push(format!("+0x{off:x}"));
                    }
                }
                // As `mem_stack`, and the other differing arena bytes are one contiguous copy
                // out of the function's caller-side frame (return slot: guest = the stub's
                // return address, lifted = 0; caller slots past the lifted window: guest = the
                // window argument, lifted = 0): x86 code read past its locals under a fuzzed
                // length. Verified in the child (see `OVERREAD_ONLY`).
                (Outcome::Ret(a), Outcome::Ret(b))
                    if a[2] != b[2]
                        && a[3] == b[3]
                        && meta.memstack == OVERREAD_ONLY
                        && (a[0] == b[0] || is_stack_or_pad(a[0], b[0], &meta))
                        && (a[1] == b[1] || is_stack_or_pad(a[1], b[1], &meta)) =>
                {
                    retslot += 1;
                    if !fn_retslot {
                        fn_retslot = true;
                        retslot_at.push(format!("+0x{off:x}:t{t}"));
                    }
                }
                // The emulator tolerates accesses outside [0x10000, 2^47) (reads 0, drops writes)
                // that fault natively; the guest then returned, threw, timed out or went on to
                // call a trapped non-code word.
                (
                    Outcome::Ret(_) | Outcome::Threw(_) | Outcome::ThrewUnrecorded(_) | Outcome::Timeout | Outcome::NonCode,
                    Outcome::Fault(a),
                ) if !EMU_VALID.contains(&a) => {
                    inconclusive += 1;
                    if incon_at.len() < 32 {
                        incon_at.push(format!("+0x{:x}@0x{a:x}", e.va as usize - base));
                    }
                }
                // Reverse direction on macOS ARM64 TBI (Top Byte Ignore): the guest faulted on a
                // non-canonical x86-64 address `a` (`!EMU_VALID`) with a non-zero top byte
                // (`a >> 56 != 0`) whose untagged 56-bit address lies inside `EMU_VALID` (mapped
                // on ARM64 because hardware TBI strips bits 56..63).
                (
                    Outcome::Fault(a),
                    Outcome::Ret(_) | Outcome::Threw(_) | Outcome::ThrewUnrecorded(_) | Outcome::Timeout | Outcome::NonCode,
                ) if !EMU_VALID.contains(&a)
                    && (a >> 56) != 0
                    && EMU_VALID.contains(&(a & 0x00FF_FFFF_FFFF_FFFF)) =>
                {
                    inconclusive += 1;
                    if incon_at.len() < 32 {
                        incon_at.push(format!("+0x{:x}@tbi:0x{a:x}", e.va as usize - base));
                    }
                }
                (a, b) if a.crashed() && b.crashed() => both_died += 1,
                (Outcome::Timeout, Outcome::Timeout) => {
                    both_timeout += 1;
                    fn_timeouts += 1;
                }
                // Both runs made a call / jump / return to address 0 (see "null_call" in the
                // module docs): natively both fault there; the emulator went on with rax = 0 (or
                // ended that run) and what followed read state neither side defines. Listed,
                // never equal.
                _ if g_null && l_null => {
                    null_call += 1;
                    if !fn_nullcall {
                        fn_nullcall = true;
                        null_call_at.push(format!("+0x{off:x}:t{t}"));
                    }
                }
                // Guest and lifted diverged (crashed vs returned, or both returned with differing
                // rax / xmm0 / arena / data), and repeating the guest run alone through the
                // zero-padded stub (`mode 1`: 0 past `WIN_ARGS`; `mode 2`: 0 past default `A = 24`
                // `LiftedFrame` window + clean local/home slots; `mode 3`: lifted re-run matching
                // `mode 2` with `[entry rsp] = stub_variant(2).2` when the body indexed its own
                // return-address slot) produced EXACTLY the matching outcome — or (`mode 4`, when
                // both returned) shifting `guest_entry_rsp` / `lstack` with identical inputs caused
                // the guest x86 run itself to produce a different result (`gs != g`, e.g. reading a
                // byte of `/GS` `__security_cookie ^ rsp` out-of-bounds of a stack table). Listed,
                // never equal; any other re-run outcome falls through to MISMATCH.
                _ if g != Outcome::Timeout
                    && matches!(l, Outcome::Ret(_) | Outcome::Fault(_) | Outcome::Threw(_))
                    && {
                        CHILD_ZERO_FRAME.store(1, core::sync::atomic::Ordering::Relaxed);
                        let (mut gz, _, _) = alone(true, TIMEOUT_MS);
                        let mut matched = gz == l;
                        if !matched {
                            CHILD_ZERO_FRAME.store(2, core::sync::atomic::Ordering::Relaxed);
                            (gz, _, _) = alone(true, TIMEOUT_MS);
                            matched = gz == l;
                        }
                        let mut lz_opt = None;
                        if !matched && matches!(gz, Outcome::Ret(_) | Outcome::Fault(_) | Outcome::Threw(_)) {
                            CHILD_ZERO_FRAME.store(3, core::sync::atomic::Ordering::Relaxed);
                            let (lz, _, _) = alone(false, TIMEOUT_MS);
                            matched = gz == lz;
                            lz_opt = Some(lz);
                        }
                        let mut gs_opt = None;
                        if !matched && matches!(g, Outcome::Ret(_)) && matches!(l, Outcome::Ret(_)) {
                            CHILD_ZERO_FRAME.store(4, core::sync::atomic::Ordering::Relaxed);
                            let (gs, _, _) = alone(true, TIMEOUT_MS);
                            matched = gs != Outcome::Timeout && gs != g;
                            gs_opt = Some(gs);
                        }
                        CHILD_ZERO_FRAME.store(0, core::sync::atomic::Ordering::Relaxed);
                        if focus.is_some() {
                            log::info!(
                                "{tag} difftest focus: {name} +0x{off:x} trial {t}: zero-padded guest re-run {gz:x?} vs lifted {l:x?} (mode3 lifted {lz_opt:x?}, mode4 shifted-rsp guest {gs_opt:x?}): {}",
                                if matched { "equal (unseeded_frame_read)" } else { "differs" }
                            );
                        }
                        matched
                    } =>
                {
                    unseeded += 1;
                    if !fn_unseeded {
                        fn_unseeded = true;
                        unseeded_at.push(format!("+0x{off:x}:t{t}"));
                    }
                }
                _ => {
                    if let (Outcome::Threw(a), Outcome::Threw(b)) = (g, l) {
                        threw_mismatch += 1;
                        if threw_mismatch_at.len() < 32 {
                            let what = if a[0] != b[0] { "ti" } else if a[1] != b[1] { "object" } else { "state" };
                            threw_mismatch_at.push(format!("+0x{off:x}:t{t}:ti=0x{:x}/0x{:x}:{what}", a[0], b[0]));
                        }
                    }
                    let msg = format!(
                        "+0x{:x} ({}) trial {t} args {gpr:x?}: guest {g:x?} / lifted {l:x?} (rax, xmm0, arena, data)",
                        e.va as usize - base,
                        if leaf { "leaf" } else { "non-leaf" }
                    );
                    log::error!("{tag} difftest: {name}: MISMATCH {msg}");
                    mismatches.push(msg);
                    fn_mismatch = true;
                    fn_seh[if seh_kind == 3 { 3 } else { 2 }] += 1;
                    break;
                }
            }
            fn_seh[if seh_unraised { 4 } else { seh_kind }] += 1;
            cases += 1;
        }
        let k = leaf as usize;
        fuzzed[k] += 1;
        if fn_equal == 0 && !fn_mismatch {
            unverified[k].push(format!("+0x{:x}", e.va as usize - base));
        }
        let res = if fn_mismatch { "mismatch" } else if fn_equal > 0 { "verified" } else { "unverified" };
        let tags: Vec<&str> =
            [(fn_narrow, "abi_narrow"), (fn_noncode, "noncode"), (fn_stack, "rax_stack"), (fn_ret_dead, "ret_dead"), (fn_nan, "nan_sign"), (fn_null, "null_exit"), (fn_nullcall, "null_call"), (fn_memstack, "mem_stack"), (fn_retslot, "frame_overread"), (fn_thread, "thread_fault"), (fn_threw, "threw_equal"), (fn_threw_residue, "threw_stack_residue"), (fn_threw_memstack, "threw_mem_stack"), (fn_threw_unrec, "threw_unrecorded"), (fn_gs, "gs_residue"), (fn_overrun, "stack_top_overrun"), (fn_oob, "stack_oob"), (fn_unseeded, "unseeded_frame_read"), (fn_seh_unraised, "seh_unraised")]
                .into_iter()
                .filter_map(|(b, s)| b.then_some(s))
                .collect();
        log::info!("{tag} difftest fn\t{name}\t+0x{off:x}\t{res}\t{}", tags.join(","));
        if precise_faults_env() {
            let [lb, fb, mm, nf, un] = fn_seh;
            log::info!("{tag} difftest seh\t{name}\t+0x{off:x}\tlanded_both={lb} fault_both_unhandled={fb} mismatch={mm} no_fault_injected={nf} seh_unraised={un}");
            if lb == 0 {
                seh_gaps.push(format!("+0x{off:x}"));
            }
            if mm > 0 {
                seh_mm.push(format!("+0x{off:x}"));
            }
        }
    }
    if precise_faults_env() {
        log::info!("{tag} difftest: {name}: seh (precise guest faults): {} function(s) with a seh mismatch {seh_mm:?}; {} coverage gap(s) (0 landed_both: no trial raised and survived an access violation on both sides) {seh_gaps:?}", seh_mm.len(), seh_gaps.len());
    }
    log::info!(
        "{tag} difftest: {name}: listed, never equal: {noncode} non-code-call trial(s) in {} function(s) (guest and lifted both called a trapped arena word / argument; natively a fault) {noncode_at:?}; {rax_stack} rax_stack trial(s) in {} function(s) (rax = each run's own stack address, rest equal) {rax_stack_at:?}; {gs_residue} gs_residue trial(s) in {} function(s) (rax = __security_cookie ^ each run's own stack address, rest equal) {gs_at:?}; {ret_dead} ret_dead trial(s) in {} function(s) (only rax bits / xmm0 no caller reads differ, db/ret_liveness*.tsv) {ret_dead_at:?}; {nan} nan_sign trial(s) in {} function(s) (x86 vs AArch64 generated NaN, rest equal; known gap) {nan_at:?}; {null_exit} null_exit (harness) trial(s) in {} function(s) (the guest run ended at rip 0: a tail jmp through a zero word) {null_at:?}; {memstack} mem_stack trial(s) in {} function(s) (only arena qwords differ, each a stack address of its own side) {memstack_at:?}; {retslot} frame_overread (harness) trial(s) in {} function(s) (as mem_stack, and the other differing arena bytes are one contiguous copy out of the function's caller-side frame: return slot = the stub's return address vs 0, caller slots past the lifted arg window = the window argument vs 0) {retslot_at:?}; {thread_fault} thread_fault trial(s) in {} function(s) (a thread a run started faulted; attributable to neither side) {thread_at:?}; {null_call} null_call trial(s) in {} function(s) (both runs called / jumped / returned to address 0, natively a fault; the emulator went on) {null_call_at:?}; {threw_equal} threw_equal trial(s) in {} function(s) (both runs threw the same C++ exception, caught at the harness boundary: same ThrowInfo, object and state after the unwinding) {threw_at:?}; {threw_residue} threw_stack_residue trial(s) in {} function(s) (as threw_equal, but the objects differ in qword high dwords that are each the high half of an address in that run's own stacks: stale stack bytes in uninitialized padding) {threw_residue_at:?}; {threw_memstack} threw_mem_stack trial(s) in {} function(s) (as threw_equal, but the arena after the unwinding differs only in qwords that are each a stack address of its own side; data sections equal) {threw_memstack_at:?}; {threw_mismatch} threw_mismatch trial(s) (both threw, ThrowInfo / object / state differ; each also a MISMATCH) {threw_mismatch_at:?}; {threw_unrec} threw_unrecorded (harness) trial(s) in {} function(s) (both runs' C++ exception stopped at a frame without an EH record before the boundary, same ThrowInfo; not compared past the throw) {threw_unrec_at:?}; {overrun} stack_top_overrun (harness) trial(s) in {} function(s) (the lifted run faulted at the end of its frame arena, walking up off its frames; natively the thread stack top, a fault; the guest emulator stack's top does not fault) {overrun_at:?}; {stack_oob} stack_oob (harness) trial(s) in {} function(s) (the guest run faulted at its emulator stack top +/- a fuzzed offset outside the stack, the lifted run did not crash: layout-dependent, natively the same offset from the thread stack) {oob_at:?}; {seh_unraised_n} seh_unraised (harness) trial(s) in {} function(s) (the lifted run raised an access violation for a guarded access the guest run did not raise: it faulted unhandled at the same address, or without precise guest faults the emulator read 0 outside [0x10000, 2^47) and the guest-alone precise re-run raised there with the lifted outcome (seh_unraised precise-confirmed, below); natively both raise it; off:trial:address) {seh_unraised_at:?}",
        noncode_at.len(),
        rax_stack_at.len(),
        gs_at.len(),
        ret_dead_at.len(),
        nan_at.len(),
        null_at.len(),
        memstack_at.len(),
        retslot_at.len(),
        thread_at.len(),
        null_call_at.len(),
        threw_at.len(),
        threw_residue_at.len(),
        threw_memstack_at.len(),
        threw_unrec_at.len(),
        overrun_at.len(),
        oob_at.len(),
        seh_unraised_at.len()
    );
    log::info!(
        "{tag} difftest: {name}: seh_unraised (precise-confirmed): {seh_precise_n} trial(s) (default mode, guarded address outside [0x10000, 2^47): guest re-run alone with precise faults raised at the same address, outcome identical to the lifted run; off:trial:address) {seh_precise_at:?}; seh_unraised refuted: {seh_refuted_n} trial(s) (precise re-run raised elsewhere / not at all or ended differently; a MISMATCH when the lifted run survived; off:trial:address:precise guest outcome) {seh_refuted_at:?}"
    );
    log::info!(
        "{tag} difftest: {name}: unseeded_frame_read (harness): {unseeded} trial(s) in {} function(s) (the guest re-run alone with 0 in unseeded stack/caller-frame bytes up to {ZERO_PAD:#x} past the lifted window returned/faulted identically to the lifted run; off:trial) {unseeded_at:?}",
        unseeded_at.len()
    );
    let share = |u: usize, n: u32| if n == 0 { 0.0 } else { u as f64 * 100.0 / n as f64 };
    let (nl, lf) = (fuzzed[0] + fuzzed[1], fuzzed[1]);
    let unv = unverified[0].len() + unverified[1].len();
    let nm = mismatches.len();
    let sgx = slow_x();
    log::info!(
        "{tag} difftest: {name}: {header}; fuzzed {nl} functions ({lf} leaves, {typed} with typed signatures) up to x{TRIALS} trials: {equal} equal, {both_died} both crashed, {both_timeout} both timed out, {nondet} guest-nondeterministic, {slow_guest} slow-guest re-run(s) (guest timed out, lifted crashed or returned; guest re-run alone with x{sgx} budget, then classified: off:trial:lifted:guest alone outcome) {slow_guest_at:?}, {fresh} fresh-child lifted re-run(s) (guest returned, lifted crashed after it in the shared child; lifted re-run alone from the guest run's starting state, then classified: off:trial:alone outcome) {fresh_at:?}, {guest_fresh} fresh-child guest re-run(s) (guest faulted, lifted returned; guest re-run alone from the same starting state, then classified: off:trial:first fault site:alone outcome) {guest_fresh_at:?}, {slow_lifted} slow-lifted re-run(s) (guest returned, lifted timed out; lifted re-run alone with x{sgx} budget, then classified: off:trial:alone outcome) {slow_lifted_at:?}, {inconclusive} inconclusive (guest returned or timed out, lifted fault outside the emulator's [0x10000, 2^47)) {incon_at:?}, {narrow} abi-narrowed rax (guest = lifted zero-extended from 8/16 bits, emulator abi_gpr) in {} function(s) {narrow_at:?}; MISMATCH {nm}; verified {}/{nl}, unverified {unv}/{nl} ({:.1}%; leaves {}/{lf}, non-leaves {}/{})",
        narrow_at.len(),
        nl as usize - unv - nm,
        share(unv, nl),
        unverified[1].len(),
        unverified[0].len(),
        fuzzed[0],
    );
    log::info!("{tag} difftest: {name}: unverified leaves {:?}", unverified[1]);
    log::info!("{tag} difftest: {name}: unverified non-leaves {:?}", unverified[0]);
    if not_fuzzed != 0 {
        log::warn!(
            "{tag} difftest: {name}: wall cap AW2_DLL_DIFF_WALL_S={} reached: {not_fuzzed} function(s) not fuzzed (unverified, not in the counts above)",
            wall.unwrap_or(0)
        );
    }
    if nm != 0 {
        return Err(format!("{name}: {nm} mismatching function(s): {}", mismatches.join(" | ")));
    }
    Ok(cases)
}

/// One `dll_regfile::<module>` difftest per installed image with `DF_REGFILE` slices.
pub(crate) fn difftests_regfile() -> Vec<DllDiff> {
    let mut out = Vec::new();
    for img in super::IMAGES.iter().copied() {
        let Some(first) = img.regfile_slices.iter().copied().flatten().next() else { continue };
        if !super::installed(img.name) || !super::regfile_enabled() {
            continue;
        }
        let name: &'static str = Box::leak(format!("dll_regfile::{}", img.name.trim_end_matches(".dll")).into_boxed_str());
        out.push(DllDiff { name, addr: first.va as usize, new: first.ptr as usize, diff: diff_regfile });
    }
    out
}

fn fnv_bytes(mut h: u64, b: &[u8]) -> u64 {
    let mut chunks = b.chunks_exact(8);
    for c in &mut chunks {
        let w = u64::from_le_bytes(c.try_into().unwrap());
        h = (h ^ w).wrapping_mul(0x1000_0000_01B3);
    }
    for &x in chunks.remainder() {
        h = (h ^ x as u64).wrapping_mul(0x1000_0000_01B3);
    }
    h
}

const RF_ARENA: usize = 128 * 1024;
const RF_STACK: usize = 1024 * 1024;
const RF_RSP_OFF: usize = 0x9E000;
const RF_RBP_OFF: usize = 0xA0000;
const RF_GUARD: usize = 64 * 1024;
const RF_HOLE_OFF: usize = 0x6C00;
/// 4 GiB-aligned `PtrComprCageBase` (`r14 & 0xFFFF_FFFF == 0`, matching V8's pointer-compression
/// cage alignment invariant) and deterministic guest stack buffer, each bracketed by 64 KiB
/// `PROT_NONE` guard regions so trials are reproducible across processes and wild probes fault cleanly.
const PIN_RF_ARENA: usize = PIN_BASE + 0x2_0000_0000;
const PIN_RF_STACK: usize = PIN_BASE + 0x2_0100_0000;

#[cfg(test)]
static LAST_REGFILE_TIMEOUTS: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);

fn map_guarded_pinned(body_hint: usize, len: usize, what: &str) -> Result<usize, String> {
    unsafe extern "C" {
        fn mprotect(addr: *mut u8, len: usize, prot: i32) -> i32;
    }
    let raw_hint = body_hint - RF_GUARD;
    let total = RF_GUARD + len + RF_GUARD;
    let raw = unsafe { mmap(raw_hint as *mut u8, total, 0, 0x0002 | 0x1000, -1, 0) };
    if raw.is_null() || raw as usize == usize::MAX {
        return Err(format!("dll_regfile: mmap({total:#x}) for {what} failed"));
    }
    let body = unsafe { raw.add(RF_GUARD) };
    if unsafe { mprotect(body, len, 1 | 2) } != 0 {
        return Err(format!("dll_regfile: mprotect({len:#x}) for {what} failed"));
    }
    let addr = body as usize;
    if addr != body_hint {
        log::warn!("dll_regfile: {what} could not be pinned at {body_hint:#x} (mapped at {addr:#x})");
    }
    Ok(addr)
}

fn pinned_regfile_bufs() -> Result<(usize, usize), String> {
    static BUFS: std::sync::OnceLock<(usize, usize)> = std::sync::OnceLock::new();
    if let Some(&pair) = BUFS.get() {
        return Ok(pair);
    }
    let a = map_guarded_pinned(PIN_RF_ARENA, RF_ARENA, "the regfile V8 cage arena")?;
    let s = map_guarded_pinned(PIN_RF_STACK, RF_STACK, "the regfile V8 stack")?;
    let _ = BUFS.set((a, s));
    Ok(*BUFS.get().unwrap())
}

/// Host-page-aligned writable-section snapshot ranges + RAII `PROT_READ` guard on all pure
/// non-writable PE pages (`headers`, `.text`, `.rdata`, `.pdata`, `.reloc`) during `diff_regfile`.
///
/// On native Apple Silicon (`16 KiB` host pages vs `4 KiB` PE section alignment), `loader.rs` maps
/// PE images `PROT_READ | PROT_WRITE`. Expanding `writable_sections` outward to host-page
/// boundaries (so any boundary `.rdata`/`.pdata` bytes sharing a 16 KiB page with `.data` are
/// snapshotted, restored, and digested) and protecting every remaining host page `PROT_READ`
/// guarantees no trial can silently mutate `.text` or `.rdata` between `run_oracle` and `RegfileFn`.
struct RegfileImageGuard {
    rw_pages: Vec<(usize, usize)>,
    ro_pages: Vec<(usize, usize)>,
}

impl RegfileImageGuard {
    unsafe fn new(base: usize) -> Self {
        unsafe extern "C" {
            fn mprotect(addr: *mut u8, len: usize, prot: i32) -> i32;
            fn getpagesize() -> i32;
        }
        let ps = (getpagesize() as usize).max(0x1000);
        let rd32 = |o: usize| core::ptr::read_unaligned((base + o) as *const u32) as usize;
        let nt = rd32(0x3C);
        let size_of_image = rd32(nt + 24 + 56);
        let img_end = (base + size_of_image + ps - 1) & !(ps - 1);

        let mut raw_rw = writable_sections(base);
        raw_rw.sort_unstable_by_key(|&(s, _)| s);
        let mut rw_pages: Vec<(usize, usize)> = Vec::new();
        for (s, e) in raw_rw {
            let ps_s = (s & !(ps - 1)).max(base);
            let ps_e = ((e + ps - 1) & !(ps - 1)).min(img_end);
            if ps_s >= ps_e {
                continue;
            }
            if let Some(last) = rw_pages.last_mut() {
                if ps_s <= last.1 {
                    last.1 = last.1.max(ps_e);
                    continue;
                }
            }
            rw_pages.push((ps_s, ps_e));
        }

        let mut ro_pages: Vec<(usize, usize)> = Vec::new();
        let mut cur = base;
        for &(s, e) in &rw_pages {
            if cur < s {
                ro_pages.push((cur, s));
            }
            cur = cur.max(e);
        }
        if cur < img_end {
            ro_pages.push((cur, img_end));
        }
        for &(s, e) in &ro_pages {
            let _ = mprotect(s as *mut u8, e - s, 1);
        }
        Self { rw_pages, ro_pages }
    }
}

impl Drop for RegfileImageGuard {
    fn drop(&mut self) {
        unsafe extern "C" {
            fn mprotect(addr: *mut u8, len: usize, prot: i32) -> i32;
        }
        for &(s, e) in &self.ro_pages {
            unsafe {
                let _ = mprotect(s as *mut u8, e - s, 1 | 2);
            }
        }
    }
}

const RF_RET_SENTINEL: u64 = 0xDEAD_BEEF_CAFE_0000;

/// Seed deterministic V8 CSA/Torque register-file + cage/root/bytecode/dispatch/stack state for
/// `DF_REGFILE` difftests:
/// - Compressed pointers in `[0x0..0x6800)` are strictly forward-only into `(pos..0x6800)` or
///   point to `RF_HOLE_OFF` (`0x6c01`), so `Context::previous` / prototype chains and hash-table
///   probes always terminate in bounded steps inside `RF_ARENA`.
/// - `RF_STACK` is 1 MiB with `rbp = +0xa0000` (`640 KiB`), keeping `.Wide` Ignition frame offsets
///   `[rbp + i16 * 8]` (`±256 KiB`) and `[rbp - u16 * 8]` (`-512 KiB`) inside `RF_STACK`.
/// - Both buffers sit between 64 KiB `PROT_NONE` guard regions at deterministic hints.
fn seed_regfile_trial(
    va: u64,
    blk: u32,
    t: u32,
    arena_addr: usize,
    stack_addr: usize,
    a_out: &mut [u8],
    s_out: &mut [u8],
) -> crate::emu::Cpu {
    let mut rng = Rng(va ^ (t as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
    a_out.fill(0);
    s_out.fill(0);
    let hole32 = (RF_HOLE_OFF | 1) as u32;
    for d in 0..(0x6800 / 4) {
        let pos = (d * 4) & !0x3F;
        let obj_idx = pos / 64;
        let fwd_lo = pos.max(0x1000) + 0x40;
        let fwd_span = 0x6800usize.saturating_sub(fwd_lo);
        let fwd_ptr = |r: &mut Rng| -> u32 {
            if fwd_span < 0x40 {
                hole32
            } else {
                let off = fwd_lo + ((r.next() as usize % fwd_span) & !0x3F);
                (off | 1) as u32
            }
        };
        let v32: u32 = match d % 16 {
            // `+0x04` (`[tagged + 3]`): keep `0x40` (`32 << 1`, `FixedArray::length`) on the low
            // anchor objects `0x0040..0x0100` and alternate with `hole32` (`WasmInstanceObject`
            // compressed pointer whose `[+7] -> 0x6c41` has `[+3] = 0x40`) across `0x1000..0x6800`.
            1 => {
                if obj_idx < 4 || (obj_idx + t as usize) % 2 == 0 {
                    0x40
                } else {
                    hole32
                }
            }
            // `+0x10` (`[tagged + 0xf]`): `(0x0040 | 1)` (`0x41`) is simultaneously a valid 64B-aligned
            // tagged compressed `HeapObject` pointer to `0x0040` (`[0x41 + 3] = 0x40` length 32) AND
            // a power-of-2-minus-1 `HashTable` probe mask `(0x41 >> 1) - 1 = 31` (`2^5 - 1`) whose
            // entire probe span stays inside `0x0..0x7000`.
            4 => 0x41,
            // `+0x14` / `+0x1c`: keep `hole32` in the low anchor region and on even objects so
            // `NameDictionary` / `GlobalDictionary` quadratic probes terminate immediately.
            5 | 7 if obj_idx < 16 || obj_idx % 2 == 0 => hole32,
            _ => {
                if rng.next() % 2 == 0 {
                    hole32
                } else {
                    fwd_ptr(&mut rng)
                }
            }
        };
        a_out[d * 4..d * 4 + 4].copy_from_slice(&v32.to_le_bytes());
    }
    // Self-contained sentinel object + backing store at `0x6800..0x7000` (`RF_HOLE_OFF = 0x6c00`):
    // `+0x4` (`d % 16 == 1`, `[tagged + 3]`) holds positive Smi length `0x40` (`32 << 1`), `+0x8`
    // (`d % 16 == 2`, `[tagged + 7]`) points to `(0x6c40 | 1)`, `+0xc` (`d % 16 == 3`, `[tagged + 0xb]`
    // `Context::previous` / prototype link) advances monotonically `0x6c01 -> 0x6c41 -> 0x6c81 -> 0x6cc1`
    // where `0x6cc0` terminates `prototype` (`length = 0`) and routes excessive `.ExtraWide` `Context`
    // depths to the upper `PROT_NONE` guard page (`RF_ARENA | 1`) on hop 4, `+0x10` (`d % 16 == 4`)
    // holds `0x41`, and remaining slots hold `hole32` (`0x6c01 == [r13 + 0xa0]`).
    let guard32 = (RF_ARENA | 1) as u32;
    for d in (0x6800 / 4)..(0x7000 / 4) {
        let byte_off = d * 4;
        let v32: u32 = if (0x6cc0..0x6d00).contains(&byte_off) {
            match d % 16 {
                1 => 0,
                3 => guard32,
                4 => 0x41,
                _ => hole32,
            }
        } else {
            match d % 16 {
                1 => 0x40,
                2 => (0x6c40 | 1) as u32,
                3 => {
                    if byte_off < 0x6c40 {
                        (0x6c40 | 1) as u32
                    } else if byte_off < 0x6c80 {
                        (0x6c80 | 1) as u32
                    } else {
                        (0x6cc0 | 1) as u32
                    }
                }
                4 => 0x41,
                _ => hole32,
            }
        };
        a_out[byte_off..byte_off + 4].copy_from_slice(&v32.to_le_bytes());
    }
    // `JSTypedArray` objects in `0x19000..0x1a000` (outside the `0x0..0x7000` compressed-pointer DAG)
    // with off-heap 64-bit `external_pointer` at `+0x28` (`[tagged + 0x27]`) pointing into
    // `arena_addr + 0x18000..0x19000` and `base_pointer = 0` at `+0x30`.
    let ext_buf = (arena_addr + 0x18000) as u64;
    for b in 0..0x1000 {
        a_out[0x18000 + b] = (rng.next() & 0x7F) as u8;
    }
    for obj in (0x19000..0x1a000).step_by(0x40) {
        a_out[obj + 0x00..obj + 0x04].copy_from_slice(&hole32.to_le_bytes());
        a_out[obj + 0x04..obj + 0x08].copy_from_slice(&0x40u32.to_le_bytes());
        a_out[obj + 0x08..obj + 0x0c].copy_from_slice(&hole32.to_le_bytes());
        a_out[obj + 0x0c..obj + 0x10].copy_from_slice(&hole32.to_le_bytes());
        a_out[obj + 0x10..obj + 0x14].copy_from_slice(&0x41u32.to_le_bytes());
        a_out[obj + 0x14..obj + 0x18].copy_from_slice(&hole32.to_le_bytes());
        a_out[obj + 0x1c..obj + 0x20].copy_from_slice(&hole32.to_le_bytes());
        a_out[obj + 0x20..obj + 0x28].copy_from_slice(&64u64.to_le_bytes());
        a_out[obj + 0x28..obj + 0x30].copy_from_slice(&ext_buf.to_le_bytes());
        a_out[obj + 0x30..obj + 0x34].copy_from_slice(&0u32.to_le_bytes());
    }
    // Roots + IsolateData tables (`external_reference_table_`, `builtin_entry_table_`, `builtin_table_`)
    // across `[r13 - 0x1000 .. r13 + 0xa000)` (`r13 = arena_addr + 0x8000`, max builtin displacement
    // `+0x9748`): 64B-aligned tagged `HeapObject` pointers into `0x1000..0x4000` (never untagged or null-page).
    for w in (0x7000 / 8)..(0x12000 / 8) {
        let off = 0x1000 + ((rng.next() as usize % 0x3000) & !0x3F);
        let v = ((arena_addr + off) as u64) | 1;
        a_out[w * 8..w * 8 + 8].copy_from_slice(&v.to_le_bytes());
    }
    // Stack-guard limits (`jslimit` / `real_jslimit` at `[r13 + 0x48..0x58)`) 512 slots below `rsp`,
    // plus sentinel roots (`undefined_value` / `the_hole_value`) at `[r13 + 0xa0..0xb0)`.
    let jslimit = (stack_addr + RF_RSP_OFF - 0x1000) as u64;
    a_out[0x8048..0x8050].copy_from_slice(&jslimit.to_le_bytes());
    a_out[0x8050..0x8058].copy_from_slice(&jslimit.to_le_bytes());
    let hole_full = ((arena_addr + RF_HOLE_OFF) as u64) | 1;
    a_out[0x80a0..0x80a8].copy_from_slice(&hole_full.to_le_bytes());
    a_out[0x80a8..0x80b0].copy_from_slice(&hole_full.to_le_bytes());
    // Bytecode bytes at `[r12 + 0..0x200)` (`r12 = arena_addr + 0x16000`):
    // - `t < 8`: structured Standard / `.Wide` / `.ExtraWide` register (`-2` -> `[rbp - 16]`)
    //   and feedback-slot / constant-pool (`[r12 + 7]`) operands so all three operand widths stay
    //   in-bounds on `RF_STACK` / `RF_ARENA`.
    // - `t >= 8`: general pseudo-random bytes.
    if t < 8 {
        match t {
            0 => {
                a_out[0x16001] = 0xFE; // Standard reg = -2 (`[rbp - 16]`), slot = 1
                a_out[0x16002] = 0x01;
            }
            1 | 5 => {
                a_out[0x16001..0x16005].copy_from_slice(&(-2i32).to_le_bytes()); // ExtraWide reg = -2
                if t == 5 {
                    a_out[0x16005..0x16009].copy_from_slice(&1u32.to_le_bytes());
                    a_out[0x16009..0x1600D].copy_from_slice(&1u32.to_le_bytes());
                }
            }
            2 | 6 => {
                a_out[0x16001..0x16003].copy_from_slice(&(-2i16).to_le_bytes()); // Wide reg = -2
                if t == 6 {
                    a_out[0x16003..0x16005].copy_from_slice(&1u16.to_le_bytes());
                    a_out[0x16005..0x16007].copy_from_slice(&1u16.to_le_bytes());
                }
            }
            3 => {
                // `BytecodeArray::constant_pool` at `[r12 + 7]` with zero operands at `r9 = 0x20`.
                a_out[0x16007..0x1600B].copy_from_slice(&hole32.to_le_bytes());
            }
            4 => {
                a_out[0x16001] = 0xFE; // Standard reg = -2, slot = 0
            }
            _ => {}
        }
    } else {
        for b in 0..0x200 {
            a_out[0x16000 + b] = (rng.next() & 0xFF) as u8;
        }
    }
    // Ignition dispatch table `[r15 + 8*0..256)` (`r15 = arena_addr + 0x14000`).
    for idx in 0..256 {
        let tgt = va.wrapping_add(((idx as u64) * 32) & 0xFFF);
        let o = 0x14000 + idx * 8;
        a_out[o..o + 8].copy_from_slice(&tgt.to_le_bytes());
    }
    // Active stack window around `rsp = +0x9e000`, `rbp = +0xa0000`, with a valid 2-frame
    // saved-`rbp` chain at `[rbp]` -> `rbp + 0x80` -> `rbp + 0x100` and tagged frame slots at
    // `[frame_rbp - 0x8]`, `[frame_rbp - 0x10]`, `[frame_rbp - 0x18]`.
    for w in ((RF_RSP_OFF - 0x2000) / 8)..((RF_RBP_OFF + 0x2000) / 8) {
        let v = match rng.next() % 4 {
            0 | 1 | 2 => ((arena_addr + 0x1000 + ((rng.next() as usize % 0x2000) & !0x3F)) as u64) | 1,
            _ => ((rng.next() % 32) + 1) << 1,
        };
        s_out[w * 8..w * 8 + 8].copy_from_slice(&v.to_le_bytes());
    }
    // Never place an address inside a mapped PE `.text` section at `[rsp]`:
    // - Entry (`blk == 0`) and continuation trial `0` use `RF_RET_SENTINEL` (`>= 2^47`) so
    //   frameless `ret` pops `RF_RET_SENTINEL` cleanly and any dereference faults symmetrically.
    // - Continuation trial `t != 0` places a valid 64B-aligned tagged `HeapObject` in `RF_ARENA`
    //   at `[rsp]` so post-`call` continuations that immediately `pop` caller-saved V8 object
    //   registers (e.g. `RecordWrite` slow-path return `pop r8; pop rdi`) run their post-call body.
    let rsp_top = if blk == 0 || t == 0 {
        RF_RET_SENTINEL
    } else {
        ((arena_addr + 0x1000 + (((va as usize >> 4) * 0x40) & 0x1FC0)) as u64) | 1
    };
    s_out[RF_RSP_OFF..RF_RSP_OFF + 8].copy_from_slice(&rsp_top.to_le_bytes());
    if t == 0 || t == 4 {
        for off in (8..0x100).step_by(8) {
            let smi = ((rng.next() % 8) << 1) as u64;
            s_out[RF_RSP_OFF + off..RF_RSP_OFF + off + 8].copy_from_slice(&smi.to_le_bytes());
        }
    }
    if t != 3 {
        let caller_rbp = (stack_addr + RF_RBP_OFF + 0x80) as u64;
        let outer_rbp = (stack_addr + RF_RBP_OFF + 0x100) as u64;
        s_out[RF_RBP_OFF..RF_RBP_OFF + 8].copy_from_slice(&caller_rbp.to_le_bytes());
        s_out[RF_RBP_OFF + 0x80..RF_RBP_OFF + 0x88].copy_from_slice(&outer_rbp.to_le_bytes());
    }
    for foff in [RF_RBP_OFF, RF_RBP_OFF + 0x80, RF_RBP_OFF + 0x100] {
        for slot in [0x08usize, 0x10, 0x18, 0x20] {
            let fn_obj = ((arena_addr + 0x1800 + ((rng.next() as usize % 0x1000) & !0x3F)) as u64) | 1;
            s_out[foff - slot..foff - slot + 8].copy_from_slice(&fn_obj.to_le_bytes());
        }
    }
    if t == 1 || t == 2 {
        s_out[RF_RBP_OFF - 8..RF_RBP_OFF].copy_from_slice(&hole_full.to_le_bytes());
    }

    let mut cpu = crate::emu::Cpu::new_zeroed();
    cpu.gpr[4] = (stack_addr + RF_RSP_OFF) as u64; // rsp
    cpu.gpr[5] = (stack_addr + RF_RBP_OFF) as u64; // rbp
    cpu.gpr[9] = if t < 8 {
        if t == 3 {
            0x20
        } else if t == 7 {
            4 + (rng.next() % 8)
        } else {
            0
        }
    } else {
        rng.next() % 0x40
    }; // BytecodeOffset / secondary count
    cpu.gpr[12] = (arena_addr + 0x16000) as u64; // BytecodeArray
    cpu.gpr[13] = (arena_addr + 0x8000) as u64; // RootRegister
    cpu.gpr[14] = arena_addr as u64; // PtrComprCageBase
    cpu.gpr[15] = (arena_addr + 0x14000) as u64; // DispatchTable

    for r in [0usize, 1, 2, 3, 6, 7, 8, 10, 11] {
        // `rsi` (`gpr[6]`) is V8's `kContextRegister`: keep it a mapped tagged Context pointer
        // except on fault-probe trial 15 so context-slot loads stay inside `a_out`.
        if r == 6 && t != 15 {
            let off = 0x1800 + ((rng.next() as usize % 0x1800) & !0x3F);
            cpu.gpr[r] = ((arena_addr + off) as u64) | 1;
            continue;
        }
        let tagged_obj = |rng: &mut Rng| -> u64 {
            ((arena_addr + 0x1000 + ((rng.next() as usize % 0x2000) & !0x3F)) as u64) | 1
        };
        cpu.gpr[r] = match t % 4 {
            // CSA accessor mix A: `rax, rcx, rsi, rdi, r8` = tagged objects (`rax` is a `JSTypedArray`
            // on `t == 4, 8`), `rdx, rbx, r10, r11` = small indices `0..4`.
            0 => {
                if r == 0 && (t == 4 || t == 8) {
                    ((arena_addr + 0x19000 + ((rng.next() as usize % 0x1000) & !0x3F)) as u64) | 1
                } else if matches!(r, 2 | 3 | 10 | 11) {
                    rng.next() % 4
                } else {
                    tagged_obj(&mut rng)
                }
            }
            // CSA accessor mix B: `rax, rdx, rsi, rdi, r8` = tagged objects, `rcx, rbx, r10, r11` = small indices/Smis `0..4`
            // (`rbx = 2` = Smi 1 on `t == 1` so positive-depth `Context` walkers take 1 hop).
            1 => {
                if t == 1 && r == 3 {
                    2
                } else if matches!(r, 1 | 3 | 10 | 11) {
                    [0u64, 1, 2, 4][(rng.next() % 4) as usize]
                } else {
                    tagged_obj(&mut rng)
                }
            }
            2 => [
                0u64,
                2,
                0x3FE,
                0xFFE,
                ((arena_addr + 0x1200) as u64) | 1,
                ((arena_addr + 0x2400) as u64) | 1,
                0xFFFF_FFFF_0000_1401,
                0x8000_0000_0000_0002,
            ][(rng.next() % 8) as usize],
            _ => {
                if t == 15 && (r == 0 || r == 1 || r == 2 || r == 3 || r == 6 || r == 7) {
                    0x41 // null-page tagged fault probe (< 0x10000)
                } else if t == 7 && (r == 0 || r == 3) {
                    rng.next() % 4 // small Wasm table / signature indices
                } else if t == 11 && r == 7 {
                    2 // IC `LoadWithVectorDescriptor` Smi feedback slot (`rdi`)
                } else {
                    tagged_obj(&mut rng)
                }
            }
        };
    }
    if t == 15 && (va & 0x20) != 0 {
        cpu.gpr[4] = 0x40; // stack fault probe on half the builtins' 16th trial
    }

    const FP_POOL: [u64; 12] = [
        0x0000_0000_0000_0000, // +0.0
        0x8000_0000_0000_0000, // -0.0
        0x3FF8_0000_0000_0000, // 1.5
        0xC045_4000_0000_0000, // -42.5
        0x7FF8_0000_0000_0000, // +qNaN
        0xFFF8_0000_0000_0000, // -qNaN (indefinite)
        0x7FF0_0000_0000_0000, // +inf
        0xFFF0_0000_0000_0000, // -inf
        0x41E0_0000_0000_0000, // 2^31
        0xC1E0_0000_0020_0000, // -(2^31 + 1)
        0x43E0_0000_0000_0000, // 2^63
        0x0000_0000_0000_0001, // subnormal
    ];
    for i in 0..16 {
        let lo = FP_POOL[(rng.next() as usize) % FP_POOL.len()];
        let hi = rng.next();
        cpu.xmm[i] = ((hi as u128) << 64) | (lo as u128);
        cpu.ymm_hi[i] = rng.next() as u128;
        cpu.last_bits[i] = [8u8, 16, 64][(rng.next() % 3) as usize];
    }
    cpu.set_rflags((rng.next() & 0x8D5) as u32);
    cpu
}

/// Full-register-file difftest (`dll_regfile::<module>`) for `DF_REGFILE` P1/P2/P3 builtins:
/// compares every lifted [`super::LiftedRegfile`] body (both entry `blk = 0` and every registered
/// continuation `(coff, cblk)`) against [`crate::emu::regfile::run_oracle`] across all 16 GPRs
/// (including `rsp`), all 16 XMMs + `ymm_hi`, `RFLAGS`, `last_bits`, `RegfileExit`, V8 heap/root/cage
/// arena, guest stack buffer, and writable PE sections.
unsafe fn diff_regfile(orig: usize, _new: usize) -> Result<u32, String> {
    let va0 = untag_diff_orig(orig);
    let img = super::IMAGES
        .iter()
        .copied()
        .find(|i| i.regfile_slices.iter().copied().flatten().next().is_some_and(|f| f.va == va0))
        .ok_or_else(|| format!("dll_regfile: no image starting at 0x{va0:x}"))?;
    let name = img.name;
    let all = img.regfile_entries();
    if all.is_empty() {
        return Err(format!("{name}: 0 regfile entries"));
    }
    let mut total_conts = 0usize;
    for e in &all {
        if e.len == 0 {
            return Err(format!("{name} +0x{:x}: zero-length regfile entry", e.va - img.base as u64));
        }
        let got = super::lookup_regfile(e.va).map(|(f, blk)| (super::cov::target_of(f as usize), blk));
        if got != Some((e.ptr as usize, 0)) {
            return Err(format!(
                "{name} +0x{:x}: REGFILE_TAB lookup {:?} != ({:#x}, 0)",
                e.va - img.base as u64,
                got,
                e.ptr as usize
            ));
        }
        for &(coff, cblk) in e.conts {
            if coff == 0 || (coff as u32) >= e.len || cblk == 0 {
                return Err(format!(
                    "{name} +0x{:x}: invalid continuation (off={:#x}, blk={}) for len={:#x}",
                    e.va - img.base as u64,
                    coff,
                    cblk,
                    e.len
                ));
            }
            let cva = e.va + coff as u64;
            let cgot = super::lookup_regfile(cva).map(|(f, blk)| (super::cov::target_of(f as usize), blk));
            if cgot != Some((e.ptr as usize, cblk as u32)) {
                return Err(format!(
                    "{name} +0x{:x}+0x{:x}: REGFILE_TAB cont lookup {:?} != ({:#x}, {})",
                    e.va - img.base as u64,
                    coff,
                    cgot,
                    e.ptr as usize,
                    cblk
                ));
            }
            total_conts += 1;
        }
    }

    let focus = focus_for(name);
    let shard: Option<(usize, usize)> = std::env::var("AW2_DLL_DIFF_SHARD")
        .ok()
        .and_then(|s| {
            let (k, n) = s.trim().split_once('/')?;
            let (k, n): (usize, usize) = (k.parse().ok()?, n.parse().ok()?);
            (n > 0 && k < n).then_some((k, n))
        });
    let wall: Option<u64> = std::env::var("AW2_DLL_DIFF_WALL_S").ok().and_then(|s| s.trim().parse().ok());

    let (arena_addr, stack_addr) = pinned_regfile_bufs()?;
    let img_guard = RegfileImageGuard::new(img.base);
    let secs = img_guard.rw_pages.as_slice();
    let sec_init: Vec<Vec<u8>> = secs
        .iter()
        .map(|&(s, e)| unsafe { core::slice::from_raw_parts(s as *const u8, e - s) }.to_vec())
        .collect();

    let restore_secs = |secs: &[(usize, usize)], sec_init: &[Vec<u8>]| {
        for (&(s, e), init) in secs.iter().zip(sec_init.iter()) {
            unsafe {
                core::ptr::copy_nonoverlapping(init.as_ptr(), s as *mut u8, e - s);
            }
        }
    };
    let reset_mem = |a_init: &[u8], s_init: &[u8]| {
        unsafe {
            core::ptr::copy_nonoverlapping(a_init.as_ptr(), arena_addr as *mut u8, RF_ARENA);
            core::ptr::copy_nonoverlapping(s_init.as_ptr(), stack_addr as *mut u8, RF_STACK);
        }
        restore_secs(secs, &sec_init);
    };
    let digest_all = |secs: &[(usize, usize)]| -> u64 {
        let mut h = 0xCBF2_9CE4_8422_2325u64;
        unsafe {
            h = fnv_bytes(h, core::slice::from_raw_parts(arena_addr as *const u8, RF_ARENA));
            h = fnv_bytes(h, core::slice::from_raw_parts(stack_addr as *const u8, RF_STACK));
            for &(s, e) in secs {
                h = fnv_bytes(h, core::slice::from_raw_parts(s as *const u8, e - s));
            }
        }
        h
    };

    let t0 = std::time::Instant::now();
    let mut fuzzed = 0u32;
    let mut verified = 0u32;
    let mut unverified: Vec<String> = Vec::new();
    let mut mismatches: Vec<String> = Vec::new();
    let mut total_eq = 0u32;
    let mut total_fault_eq = 0u32;
    let mut total_both_died = 0u32;
    let mut total_both_timeout = 0u32;
    let mut not_fuzzed = 0u32;

    for (ord, e) in all.iter().enumerate() {
        let off = (e.va - img.base as u64) as usize;
        if let Some((foff, _)) = focus {
            if off != foff {
                continue;
            }
        } else if let Some((k, n)) = shard {
            if ord % n != k {
                continue;
            }
        }
        if let Some(cap) = wall {
            if t0.elapsed().as_secs() >= cap {
                not_fuzzed += 1;
                continue;
            }
        }

        let mut cases: Vec<(u64, u32, u32)> = match focus.and_then(|(_, tr)| tr) {
            Some(tr) => vec![(e.va, 0, tr)],
            None => {
                let mut v: Vec<(u64, u32, u32)> = (0..TRIALS).map(|t| (e.va, 0, t)).collect();
                for &(coff, cblk) in e.conts {
                    let cva = e.va + coff as u64;
                    v.push((cva, cblk as u32, 0));
                    v.push((cva, cblk as u32, 1));
                }
                v
            }
        };

        const RF_OK_TAG: u64 = 0x5246_0001;
        const RF_SIDE_TAG: u64 = 0x5246_0002;
        const RF_MIS_TAG: u64 = 0x5246_EEEE;

        let mut fn_eq = 0u32;
        let mut fn_fault_eq = 0u32;
        let mut fn_died = 0u32;
        let mut fn_timeout = 0u32;
        let mut fn_mis: Option<String> = None;

        // Fast batch path in 16-case chunks so any single continuation hitting a guard page or
        // unmapped address only isolates its 16-case chunk instead of the entire builtin's cases.
        for chunk in cases.chunks_mut(16) {
            let chunk: &[(u64, u32, u32)] = chunk;
            let (recs, _meta, exited) = unsafe {
                in_child(TIMEOUT_MS, &|emit| {
                    let mut a_init = vec![0u8; RF_ARENA];
                    let mut s_init = vec![0u8; RF_STACK];
                    let mut b_eq = 0u64;
                    let mut b_fault_eq = 0u64;
                    for (ci, &(start_va, blk, t)) in chunk.iter().enumerate() {
                        let cpu0 =
                            seed_regfile_trial(start_va, blk, t, arena_addr, stack_addr, &mut a_init, &mut s_init);

                        reset_mem(&a_init, &s_init);
                        crate::emu::regfile_ext::x87_reset();
                        let mut cpu_g = cpu0.clone();
                        let ex_g =
                            crate::emu::regfile::run_oracle(&mut cpu_g, start_va, e.va, e.va + e.len as u64);
                        let x87_g = crate::emu::regfile_ext::x87_snapshot();
                        let dg = digest_all(secs);

                        reset_mem(&a_init, &s_init);
                        crate::emu::regfile_ext::x87_reset();
                        let mut cpu_l = cpu0;
                        let ex_l = (e.ptr)(&mut cpu_l as *mut crate::emu::Cpu as *mut u8, blk);
                        let x87_l = crate::emu::regfile_ext::x87_snapshot();
                        let dl = digest_all(secs);

                        let mut diff_bits = 0u64;
                        if ex_g != ex_l {
                            diff_bits |= 1 << 0;
                        }
                        if cpu_g.gpr != cpu_l.gpr {
                            diff_bits |= 1 << 1;
                        }
                        if cpu_g.xmm != cpu_l.xmm || cpu_g.ymm_hi != cpu_l.ymm_hi {
                            diff_bits |= 1 << 2;
                        }
                        if cpu_g.rflags() != cpu_l.rflags() {
                            diff_bits |= 1 << 3;
                        }
                        if cpu_g.last_bits != cpu_l.last_bits {
                            diff_bits |= 1 << 4;
                        }
                        if dg != dl {
                            diff_bits |= 1 << 5;
                        }
                        if x87_g != x87_l {
                            diff_bits |= 1 << 6;
                        }
                        if diff_bits != 0 {
                            emit([RF_MIS_TAG, ci as u64, diff_bits, ex_g.va ^ ex_l.va]);
                            return;
                        } else if ex_g.kind == crate::emu::regfile::RF_FAULT {
                            b_fault_eq += 1;
                        } else {
                            b_eq += 1;
                        }
                    }
                    emit([RF_OK_TAG, b_eq, b_fault_eq, 0]);
                })
            };

            if exited == Some(true) && recs.len() == 1 && (recs[0][0] == RF_OK_TAG || recs[0][0] == RF_MIS_TAG) {
                if recs[0][0] == RF_MIS_TAG {
                    let (sva, blk, t) = chunk[recs[0][1] as usize];
                    fn_mis = Some(format!(
                        "+0x{off:x}+0x{:x} (blk={blk}) trial {t} diff_bits={:#x}",
                        sva - e.va,
                        recs[0][2]
                    ));
                    break;
                } else {
                    fn_eq += recs[0][1] as u32;
                    fn_fault_eq += recs[0][2] as u32;
                }
            } else {
                for &(start_va, blk, t) in chunk {
                    let run_side = |is_lifted: bool, budget_ms: u64| unsafe {
                        in_child(budget_ms, &|emit| {
                            let mut a_init = vec![0u8; RF_ARENA];
                            let mut s_init = vec![0u8; RF_STACK];
                            let mut cpu =
                                seed_regfile_trial(start_va, blk, t, arena_addr, stack_addr, &mut a_init, &mut s_init);
                            reset_mem(&a_init, &s_init);
                            crate::emu::regfile_ext::x87_reset();
                            let ex = if is_lifted {
                                (e.ptr)(&mut cpu as *mut crate::emu::Cpu as *mut u8, blk)
                            } else {
                                crate::emu::regfile::run_oracle(&mut cpu, start_va, e.va, e.va + e.len as u64)
                            };
                            let d_mem = digest_all(secs);
                            let mut h_cpu = 0xCBF2_9CE4_8422_2325u64;
                            for g in cpu.gpr {
                                h_cpu = fnv_bytes(h_cpu, &g.to_le_bytes());
                            }
                            for x in cpu.xmm {
                                h_cpu = fnv_bytes(h_cpu, &x.to_le_bytes());
                            }
                            for y in cpu.ymm_hi {
                                h_cpu = fnv_bytes(h_cpu, &y.to_le_bytes());
                            }
                            h_cpu = fnv_bytes(h_cpu, &cpu.rflags().to_le_bytes());
                            h_cpu = fnv_bytes(h_cpu, &cpu.last_bits);
                            h_cpu = fnv_bytes(
                                h_cpu,
                                &crate::emu::regfile_ext::x87_snapshot().status_word().to_le_bytes(),
                            );
                            emit([RF_OK_TAG, ex.kind ^ (ex.va << 2), h_cpu, d_mem]);
                            emit([RF_SIDE_TAG, ex.kind, ex.va, 0]);
                        })
                    };
                    let (mut rg, _mg, mut exg) = run_side(false, TIMEOUT_MS);
                    let (rl, _ml, exl) = run_side(true, TIMEOUT_MS);
                    if exg.is_none() && exl.is_some() {
                        let (rg2, _mg2, exg2) = run_side(false, TIMEOUT_MS * slow_x() * PRECISE_SLOW_X);
                        rg = rg2;
                        exg = exg2;
                    }
                    match (exg, exl, rg.len() >= 2 && rg[0][0] == RF_OK_TAG, rl.len() >= 2 && rl[0][0] == RF_OK_TAG)
                    {
                        (Some(true), Some(true), true, true) => {
                            if rg[0] != rl[0] || rg[1] != rl[1] {
                                fn_mis = Some(format!(
                                    "+0x{off:x}+0x{:x} (blk={blk}) trial {t} isolated state mismatch",
                                    start_va - e.va
                                ));
                                break;
                            }
                            if rg[1][1] == crate::emu::regfile::RF_FAULT {
                                fn_fault_eq += 1;
                            } else {
                                fn_eq += 1;
                            }
                        }
                        (Some(false), Some(false), _, _) => {
                            fn_died += 1;
                        }
                        (None, None, _, _) => {
                            fn_timeout += 1;
                        }
                        _ => {
                            fn_mis = Some(format!(
                                "+0x{off:x}+0x{:x} (blk={blk}) trial {t} one-sided crash/timeout ({exg:?} vs {exl:?})",
                                start_va - e.va
                            ));
                            break;
                        }
                    }
                }
                if fn_mis.is_some() {
                    break;
                }
            }
        }
        restore_secs(&secs, &sec_init);

        fuzzed += 1;
        total_eq += fn_eq;
        total_fault_eq += fn_fault_eq;
        total_both_died += fn_died;
        total_both_timeout += fn_timeout;

        let res = if let Some(m) = fn_mis {
            mismatches.push(m);
            "mismatch"
        } else if fn_eq > 0 {
            verified += 1;
            "verified"
        } else {
            unverified.push(format!("+0x{off:x}"));
            "unverified"
        };
        log::info!("dll_lift difftest fn\t{name}\t+0x{off:x}\t{res}\tregfile");
    }

    #[cfg(test)]
    LAST_REGFILE_TIMEOUTS.store(total_both_timeout, core::sync::atomic::Ordering::Relaxed);
    log::info!(
        "dll_regfile difftest: {name}: {} entries (+{total_conts} conts) installed; fuzzed {fuzzed} (x{TRIALS} entry + 2x cont trials): {total_eq} equal (ret/jump/call), {total_fault_eq} equal (fault), {total_both_died} both crashed, {total_both_timeout} both timed out; MISMATCH {}; verified {verified}/{fuzzed}, unverified {}/{fuzzed} {:?}, not_fuzzed {not_fuzzed}",
        all.len(),
        mismatches.len(),
        unverified.len(),
        unverified
    );
    if !mismatches.is_empty() {
        return Err(format!("{name}: {} regfile mismatch(es): {}", mismatches.len(), mismatches.join(" | ")));
    }
    if not_fuzzed != 0 {
        return Err(format!("{name}: {not_fuzzed} regfile entries not fuzzed (wall cap reached)"));
    }
    Ok(verified)
}

#[cfg(test)]
mod tests {
    use super::{frame_image, guest_run, overread_shifts, poison_slot, WIN, WIN_ARGS};

    /// `AW2_DLL_DIFF_SET` forms: register, `arg_i + off` field, arena offset, writable-image VA,
    /// and arena-relative values (`@off`) building `rcx -> obj[+8] -> obj2 -> vtable[0]`.
    #[test]
    fn set_overrides_build_nested_objects() {
        use super::{apply_overrides_with, parse_overrides, SetAt};
        let ov = parse_overrides("0=@100, a+108=@200,a+200=@300,a+300=70001f0b40,1=40,1+28=7,m+7000697150=@200,9=1,x+8=1");
        assert_eq!(
            ov,
            vec![
                (SetAt::Reg(0), 0x100, true),
                (SetAt::Arena(0x108), 0x200, true),
                (SetAt::Arena(0x200), 0x300, true),
                (SetAt::Arena(0x300), 0x7000_1f0b40, false),
                (SetAt::Reg(1), 0x40, false),
                (SetAt::Field(1, 0x28), 7, false),
                (SetAt::Mem(0x70_0069_7150), 0x200, true),
            ]
        );
        let arena = 0x5000_0000usize;
        let mut init = vec![0xAAu8; 0x1000];
        let mut gpr = [0u64; 8];
        apply_overrides_with(&ov, &mut gpr, &mut init, arena);
        let q = |o: usize| u64::from_le_bytes(init[o..o + 8].try_into().unwrap());
        assert_eq!(gpr[0], arena as u64 + 0x100);
        assert_eq!(gpr[1], 0x40);
        assert_eq!(q(0x108), arena as u64 + 0x200);
        assert_eq!(q(0x200), arena as u64 + 0x300);
        assert_eq!(q(0x300), 0x7000_1f0b40);
        // `1+28` points outside the arena (rcx.. = 0x40): no write.
        assert_eq!(init[0x28..0x30], [0xAA; 8]);
    }

    /// [`guest_run`] of a function that must not throw.
    unsafe fn guest_run_checked(va: u64, gpr: [u64; 8], xmm: [u128; 4], w: &[u64; WIN]) -> Option<(u64, u128)> {
        match unsafe { guest_run(va, gpr, xmm, w) } {
            Ok(r) => r,
            Err(c) => panic!("guest run of 0x{va:x} threw 0x{:x}", c.rec.code),
        }
    }

    /// A stack window carrying stack arguments 9..16 (`stack[0..8]`), the rest zero.
    fn w(stack: [u64; 8]) -> [u64; WIN] {
        let mut w = [0u64; WIN];
        w[8..16].copy_from_slice(&stack);
        w
    }

    /// [`super::null_exit_stub`]'s target / done words are process-wide: tests that run guest
    /// code through it take this lock (parallel test threads otherwise see each other's run).
    static STUB_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    fn leak(code: &[u8]) -> u64 {
        Box::leak(code.to_vec().into_boxed_slice()).as_ptr() as u64
    }

    /// [`super::null_exit_stub`]: a guest fn that calls through a zero word ends the run at
    /// rip 0 (`None`); normally returning fns give `Some(rax)`, and register / stack
    /// arguments reach the callee unchanged.
    /// v8.dll +0xbb7a70 (`Assembler::emit_inc(Operand, int)`) trial 15, g11 focus arena dumps:
    /// the guest arena holds 5 bytes of the stub's return address at 0x67c9 and the low bytes
    /// of window args 24..28 at 0x6889 + 8n (lifted: 0), i.e. one copy out of the frame at
    /// shift 0x67c9 (args 5..23 were copied identically on both sides).
    #[test]
    fn frame_overread_is_one_contiguous_copy() {
        let rg = 0x0000_007a_9004_41c1u64;
        let mut win = [0u64; WIN];
        let stack: [u64; 23] = [0x16, 0, 0x23, 0x1e, 0x3c, 0x2d, 0x16, 0x29, 0x2c, 0x1c, 0x27, 0x3b, 0x1f, 0x30, 0x17, 0x19, 0x22, 0x39, 0xb, 0x28, 0x2e, 0x1b, 0x31];
        win[8..WIN_ARGS].copy_from_slice(&stack);
        let img = frame_image(rg, &win);
        // (arena qword index, guest qword, lifted qword)
        let q = |off: usize, g: u64, l: u64| (off / 8, g, l);
        let diffs = [
            q(0x67c8, 0x0000_7a90_0441_c100, 0),
            q(0x6888, 0x1900, 0),
            q(0x6890, 0x2200, 0),
            q(0x6898, 0x3900, 0),
            q(0x68a0, 0x0b00, 0),
            q(0x68a8, 0x2800, 0),
        ];
        let common = diffs.iter().fold(None::<Vec<i64>>, |acc, &(i, g, l)| {
            let s = overread_shifts(i, g, l, &img);
            Some(match acc {
                None => s,
                Some(p) => p.into_iter().filter(|x| s.contains(x)).collect(),
            })
        });
        assert_eq!(common, Some(vec![0x67c9]));
        // A lifted byte that is not 0 there, or an argument byte at the wrong distance, is not
        // an overread.
        assert!(overread_shifts(0x6888 / 8, 0x1900, 0x0100, &img).iter().all(|&p| p != 0x67c9));
        assert!(!overread_shifts(0x6888 / 8, 0x1900, 0, &img).is_empty());
        assert!(!overread_shifts(0x6880 / 8, 0x1900, 0, &img).contains(&0x67c9));
        // Args 5..23 are inside every lifted window: a difference there is never explained.
        assert!(overread_shifts(0x67f0 / 8, 0x3c00, 0, &img).iter().all(|&p| p != 0x67c9));
        // Poisoned slots name themselves.
        assert_eq!(poison_slot(24).to_le_bytes(), [24, 0xA5, 24, 0xC3, 24, 0x5A, 24, 0x96]);
    }

    #[test]
    fn null_exit_stub_detects_call_through_zero() {
        let _g = STUB_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let zero: &'static u64 = Box::leak(Box::new(0u64));
        unsafe {
            // mov eax, 42 ; ret
            let ok = leak(&[0xB8, 42, 0, 0, 0, 0xC3]);
            assert_eq!(guest_run_checked(ok, [0; 8], [0; 4], &w([0; 8])).map(|r| r.0), Some(42));
            // jmp [rcx] (tail call through a zero word): the emulator ends the whole run at rip 0.
            let tail = leak(&[0xFF, 0x21]);
            assert_eq!(guest_run_checked(tail, [zero as *const u64 as u64, 0, 0, 0, 0, 0, 0, 0], [0; 4], &w([0; 8])), None);
            // mov rax, rdx ; jmp [rcx] behind a frame: push rbx ; jmp [rcx]
            let tail2 = leak(&[0x53, 0xFF, 0x21]);
            assert_eq!(guest_run_checked(tail2, [zero as *const u64 as u64, 0, 0, 0, 0, 0, 0, 0], [0; 4], &w([0; 8])), None);
            // call [rcx] ; ret (rcx -> a zero qword): the emulator's call to 0 returns 0 at once
            // (like the lifted `call_va8(0)`), so the run goes on and returns.
            let call0 = leak(&[0xFF, 0x11, 0xC3]);
            assert_eq!(guest_run_checked(call0, [zero as *const u64 as u64, 0x1111, 0, 0, 0, 0, 0, 0], [0; 4], &w([0; 8])).map(|r| r.0), Some(0));
            // mov rax, rdx ; ret
            let a2 = leak(&[0x48, 0x89, 0xD0, 0xC3]);
            assert_eq!(guest_run_checked(a2, [1, 0x22, 3, 4, 5, 6, 7, 8], [0; 4], &w([0; 8])).map(|r| r.0), Some(0x22));
            // mov rax, [rsp+0x28] ; ret   (argument 5)
            let a5 = leak(&[0x48, 0x8B, 0x44, 0x24, 0x28, 0xC3]);
            assert_eq!(guest_run_checked(a5, [1, 2, 3, 4, 0x55, 6, 7, 8], [0; 4], &w([0; 8])).map(|r| r.0), Some(0x55));
            // mov rax, [rsp+0x40] ; ret   (argument 8)
            let a8 = leak(&[0x48, 0x8B, 0x44, 0x24, 0x40, 0xC3]);
            assert_eq!(guest_run_checked(a8, [1, 2, 3, 4, 5, 6, 7, 0x88], [0; 4], &w([0; 8])).map(|r| r.0), Some(0x88));
            // mov rax, [rsp+0x48] ; ret   (argument 9, first stack-only argument)
            let a9 = leak(&[0x48, 0x8B, 0x44, 0x24, 0x48, 0xC3]);
            assert_eq!(guest_run_checked(a9, [0; 8], [0; 4], &w([0x99, 0, 0, 0, 0, 0, 0, 0])).map(|r| r.0), Some(0x99));
            // mov rax, [rsp+0x88] ; ret   (argument 17, from the window's hi_args)
            let a17 = leak(&[0x48, 0x8B, 0x84, 0x24, 0x88, 0, 0, 0, 0xC3]);
            let mut hi = w([0; 8]);
            hi[16] = 0x17;
            hi[30] = 0x31;
            assert_eq!(guest_run_checked(a17, [0; 8], [0; 4], &hi).map(|r| r.0), Some(0x17));
            // mov rax, [rsp+0xf8] ; ret   (argument 31, the last fuzzed slot)
            let a31 = leak(&[0x48, 0x8B, 0x84, 0x24, 0xF8, 0, 0, 0, 0xC3]);
            assert_eq!(guest_run_checked(a31, [0; 8], [0; 4], &hi).map(|r| r.0), Some(0x31));
            // the stub is reusable after a null exit
            assert_eq!(guest_run_checked(ok, [0; 8], [0; 4], &w([0; 8])).map(|r| r.0), Some(42));
        }
    }

    /// [`super::stub_variant`]`(1)` (the `unseeded_frame_read` mode-1 re-run): the window
    /// arguments reach the callee as through the default stub, and every qword past the window
    /// up to [`super::ZERO_PAD`] reads 0 (g21 cohtml +0x133c30 read `[rsp+0x10c..0x118)`).
    #[test]
    fn zero_padded_stub_reads_zero_past_the_window() {
        use core::sync::atomic::Ordering::Relaxed;
        let _g = STUB_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let mut hi = w([0x99, 0, 0, 0, 0, 0, 0, 0]);
        hi[30] = 0x31;
        // mov rax, [rsp+disp32] ; ret
        let load = |disp: u32| {
            let mut c = vec![0x48, 0x8B, 0x84, 0x24];
            c.extend(disp.to_le_bytes());
            c.push(0xC3);
            leak(&c)
        };
        // mov eax, [rsp+0x110] ; ret   (default stub: the low dword of the stub's own return address)
        let ret_lo = leak(&[0x8B, 0x84, 0x24, 0x10, 0x01, 0, 0, 0xC3]);
        unsafe {
            super::CHILD_ZERO_FRAME.store(1, Relaxed);
            let r9 = guest_run_checked(load(0x48), [0; 8], [0; 4], &hi).map(|r| r.0);
            let r31 = guest_run_checked(load(0xf8), [0; 8], [0; 4], &hi).map(|r| r.0);
            let past: Vec<Option<u64>> = [0x100u32, 0x108, 0x110, 0x118, 0x400, 0x100 + super::ZERO_PAD - 8]
                .iter()
                .map(|&d| guest_run_checked(load(d), [0; 8], [0; 4], &hi).map(|r| r.0))
                .collect();
            super::CHILD_ZERO_FRAME.store(0, Relaxed);
            let dflt = guest_run_checked(ret_lo, [0; 8], [0; 4], &hi).map(|r| r.0);
            assert_eq!(r9, Some(0x99));
            assert_eq!(r31, Some(0x31));
            assert!(past.iter().all(|&v| v == Some(0)), "{past:x?}");
            // Through the default stub the same slot is the stub's own return address (the
            // emulator's top-level sentinel): not 0.
            assert_ne!(dflt, Some(0));
        }
    }

    /// [`super::stub_variant`]`(2)` (the `unseeded_frame_read` mode-2 re-run): arguments 5..=23
    /// (`LIFTED_ARGS_MIN - 1`, `[rsp+0x28..0xc0)`) reach the callee unchanged, while unwritten
    /// home space (`[rsp+0x08..0x28)`), arguments 24..=31 (`[rsp+0xc0..0x100)`), local slots
    /// below `[rsp]`, and [`super::ZERO_PAD`] bytes above the window all read 0, matching a
    /// default `A = 24` `LiftedFrame`.
    #[test]
    fn zero_padded_stub_mode2_zeros_args_24_to_31_and_home_slots() {
        use core::sync::atomic::Ordering::Relaxed;
        let _g = STUB_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let mut hi = w([0x99, 0, 0, 0, 0, 0, 0, 0]);
        hi[22] = 0x23; // arg 23 at [rsp+0xb8]
        hi[23] = 0x24; // arg 24 at [rsp+0xc0]
        hi[30] = 0x31; // arg 31 at [rsp+0xf8]
        let load = |disp: u32| {
            let mut c = vec![0x48, 0x8B, 0x84, 0x24];
            c.extend(disp.to_le_bytes());
            c.push(0xC3);
            leak(&c)
        };
        // sub rsp, 0x48 ; mov rax, [rsp+0x38] ; add rsp, 0x48 ; ret  (unwritten local slot)
        let dirty_then_read_local = leak(&[
            0x48, 0x83, 0xEC, 0x48, 0x48, 0x8B, 0x44, 0x24, 0x38, 0x48, 0x83, 0xC4, 0x48, 0xC3,
        ]);
        let write_local = leak(&[
            0x48, 0x83, 0xEC, 0x48, 0x48, 0xC7, 0x44, 0x24, 0x38, 0x77, 0, 0, 0, 0x48, 0x83, 0xC4,
            0x48, 0xC3,
        ]);
        unsafe {
            let _ = guest_run_checked(write_local, [0; 8], [0; 4], &hi);
            super::CHILD_ZERO_FRAME.store(2, Relaxed);
            let r9 = guest_run_checked(load(0x48), [0; 8], [0; 4], &hi).map(|r| r.0);
            let r23 = guest_run_checked(load(0xb8), [0; 8], [0; 4], &hi).map(|r| r.0);
            let local0 = guest_run_checked(dirty_then_read_local, [0; 8], [0; 4], &hi).map(|r| r.0);
            let zeroed: Vec<Option<u64>> = [0x08u32, 0x10, 0x18, 0x20, 0xc0, 0xe0, 0xf8, 0x100, 0x4d8]
                .iter()
                .map(|&d| guest_run_checked(load(d), [0; 8], [0; 4], &hi).map(|r| r.0))
                .collect();
            super::CHILD_ZERO_FRAME.store(0, Relaxed);
            assert_eq!(r9, Some(0x99));
            assert_eq!(r23, Some(0x23));
            assert_eq!(local0, Some(0));
            assert!(zeroed.iter().all(|&v| v == Some(0)), "{zeroed:x?}");
        }
    }

    /// Harness self-test: the guest side runs the entry's own x86 even when the entry VA is
    /// registered in the dispatch table (as every lifted entry under test is), the
    /// registration is back afterwards, and calls *from* guest code still dispatch.
    #[test]
    fn guest_run_bypasses_entry_dispatch() {
        let _g = STUB_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        unsafe extern "win64" fn native7(_: u64, _: u64, _: u64, _: u64) -> u64 {
            7
        }
        let native = native7 as *const () as usize;
        unsafe {
            // mov eax, 42 ; ret   (registered: dispatches to `native7`)
            let entry = leak(&[0xB8, 42, 0, 0, 0, 0xC3]);
            crate::exe::register_native_va(entry, native);
            assert_eq!(guest_run_checked(entry, [0; 8], [0; 4], &w([0; 8])).map(|r| r.0), Some(42));
            assert_eq!(crate::exe::lookup_va_only(entry), Some(native));
            // call [rcx] ; ret   with [rcx] = entry: the callee dispatches to `native7`.
            let slot: &'static u64 = Box::leak(Box::new(entry));
            let caller = leak(&[0xFF, 0x11, 0xC3]);
            let args = [slot as *const u64 as u64, 0, 0, 0, 0, 0, 0, 0];
            assert_eq!(guest_run_checked(caller, args, [0; 4], &w([0; 8])).map(|r| r.0), Some(7));
            crate::exe::unregister_native_va(entry);
        }
    }

    /// `--diff-test` tags `orig` with `1 << 46`; verify [`super::untag_diff_orig`] strips it
    /// and `v8::IMAGE` exposes all 1,643 `DF_REGFILE` embedded builtin entries. With `AW2_TEST_V8_REGFILE_FULL=1`
    /// (`--test-threads=1`), also maps `v8.dll` and runs [`super::diff_regfile`] end-to-end
    /// across all 1,643 entries + continuations against [`crate::emu::regfile::run_oracle`].
    #[test]
    fn regfile_orig_untag_and_v8_oracle_parity() {
        unsafe extern "C" {
            fn munmap(addr: *mut u8, len: usize) -> i32;
        }
        let v8_img = &super::super::v8::IMAGE;
        let entries = v8_img.regfile_entries();
        assert_eq!(entries.len(), 1643);
        let first_va = entries[0].va;
        let tagged = (first_va as usize) | (1usize << 46);
        assert_eq!(super::untag_diff_orig(tagged), first_va);
        assert_eq!(super::untag_diff_orig(first_va as usize), first_va);

        if std::env::var("AW2_TEST_V8_REGFILE_FULL").as_deref() != Ok("1") {
            return;
        }
        let Some(v8_path) = std::env::var_os("PORT_GAME_DIR")
            .or_else(|| std::env::var_os("AW2_GAME_DIR"))
            .filter(|v| !v.is_empty())
            .map(|d| std::path::PathBuf::from(d).join("v8.dll"))
        else {
            return;
        };
        let raw = std::fs::read(&v8_path).expect("read v8.dll");
        let pe = u32::from_le_bytes(raw[0x3c..0x40].try_into().unwrap()) as usize;
        let num_secs = u16::from_le_bytes(raw[pe + 6..pe + 8].try_into().unwrap()) as usize;
        let opt_sz = u16::from_le_bytes(raw[pe + 20..pe + 22].try_into().unwrap()) as usize;
        let opt = pe + 24;
        let pref_base = u64::from_le_bytes(raw[opt + 24..opt + 32].try_into().unwrap());
        let size_of_image = u32::from_le_bytes(raw[opt + 56..opt + 60].try_into().unwrap()) as usize;
        let size_of_headers = u32::from_le_bytes(raw[opt + 60..opt + 64].try_into().unwrap()) as usize;
        let reloc_rva = u32::from_le_bytes(raw[opt + 112 + 5 * 8..opt + 112 + 5 * 8 + 4].try_into().unwrap()) as usize;
        let reloc_sz = u32::from_le_bytes(raw[opt + 112 + 5 * 8 + 4..opt + 112 + 5 * 8 + 8].try_into().unwrap()) as usize;

        let map_len = (size_of_image + 0x3fff) & !0x3fff;
        let base = v8_img.base;
        unsafe {
            let p = super::mmap(base as *mut u8, map_len, 1 | 2, 0x0002 | 0x1000, -1, 0);
            let mapped = p as usize;
            assert!(mapped == base, "could not map v8.dll at {base:#x} (got {mapped:#x})");
            let img_slice = core::slice::from_raw_parts_mut(p, map_len);
            let hdr_n = size_of_headers.min(raw.len()).min(map_len);
            img_slice[..hdr_n].copy_from_slice(&raw[..hdr_n]);
            let sec_tab = opt + opt_sz;
            for i in 0..num_secs {
                let sh = sec_tab + i * 40;
                let vsz = u32::from_le_bytes(raw[sh + 8..sh + 12].try_into().unwrap()) as usize;
                let rva = u32::from_le_bytes(raw[sh + 12..sh + 16].try_into().unwrap()) as usize;
                let rsz = u32::from_le_bytes(raw[sh + 16..sh + 20].try_into().unwrap()) as usize;
                let rptr = u32::from_le_bytes(raw[sh + 20..sh + 24].try_into().unwrap()) as usize;
                let n = vsz.min(rsz);
                if rptr + n <= raw.len() && rva + n <= map_len {
                    img_slice[rva..rva + n].copy_from_slice(&raw[rptr..rptr + n]);
                }
            }
            let delta = (base as u64).wrapping_sub(pref_base);
            if reloc_rva != 0 && reloc_rva + reloc_sz <= map_len {
                let mut cur = reloc_rva;
                let end = reloc_rva + reloc_sz;
                while cur + 8 <= end {
                    let page_rva = u32::from_le_bytes(img_slice[cur..cur + 4].try_into().unwrap()) as usize;
                    let blk_sz = u32::from_le_bytes(img_slice[cur + 4..cur + 8].try_into().unwrap()) as usize;
                    if blk_sz < 8 || cur + blk_sz > end {
                        break;
                    }
                    for off in (8..blk_sz).step_by(2) {
                        let ent = u16::from_le_bytes(img_slice[cur + off..cur + off + 2].try_into().unwrap());
                        if (ent >> 12) == 10 {
                            let at = page_rva + (ent & 0x0fff) as usize;
                            if at + 8 <= map_len {
                                let old = u64::from_le_bytes(img_slice[at..at + 8].try_into().unwrap());
                                img_slice[at..at + 8].copy_from_slice(&old.wrapping_add(delta).to_le_bytes());
                            }
                        }
                    }
                    cur += blk_sz;
                }
            }

            let mut pairs: Vec<(u64, crate::emu::regfile::RegfileFn, u32)> = Vec::new();
            for e in &entries {
                pairs.push((e.va, e.ptr, 0));
                for &(coff, cblk) in e.conts {
                    pairs.push((e.va + coff as u64, e.ptr, cblk as u32));
                }
            }
            super::super::register_regfile_vas(&pairs);
            let res = super::diff_regfile(tagged, entries[0].ptr as usize);
            for &(va, _, _) in &pairs {
                super::super::clear_regfile_va_for_test(va);
            }
            munmap(p, map_len);
            let verified = res.expect("diff_regfile on all 1643 v8 embedded builtins");
            let _timeouts = super::LAST_REGFILE_TIMEOUTS.load(core::sync::atomic::Ordering::Relaxed);
            assert!(verified >= 1595, "expected >=1595/1643 verified, got {verified}");
        }
    }

    /// Regression test for `+0x1a4434+0xcbb (blk=247)`: verifies `seed_regfile_trial` never seeds
    /// `[rsp]` inside mapped PE `.text` (`RF_RET_SENTINEL` on `blk == 0` / `t == 0`, valid tagged
    /// `RF_ARENA` `HeapObject` on continuation `t != 0`), `RegfileImageGuard` page-aligns writable
    /// sections while write-protecting pure RO PE pages (and restores `RW` on drop), and
    /// `rf_7000890434(..., 247)` matches `run_oracle` bit-for-bit across both `t = 0` and `t = 1`.
    #[test]
    fn regfile_seed_trial_sentinel_and_cont_pop_parity() {
        unsafe extern "C" {
            fn munmap(addr: *mut u8, len: usize) -> i32;
            fn getpagesize() -> i32;
        }
        unsafe {
            let ps = (getpagesize() as usize).max(0x1000);
            let fake_len = ps * 3;
            let fake = super::mmap(core::ptr::null_mut(), fake_len, 1 | 2, 0x0002 | 0x1000, -1, 0);
            assert!(!fake.is_null() && fake as usize != usize::MAX);
            let fb = fake as usize;
            // Minimal PE header at `fb`: `.text` `[ps, 2*ps)` RO, `.data` `[2*ps, 2*ps + 0x100)` RW.
            core::ptr::write_unaligned((fb + 0x3C) as *mut u32, 0x80);
            let nt = fb + 0x80;
            core::ptr::write_unaligned((nt + 6) as *mut u16, 2);
            core::ptr::write_unaligned((nt + 20) as *mut u16, 0xF0);
            core::ptr::write_unaligned((nt + 24 + 56) as *mut u32, fake_len as u32);
            let st = nt + 24 + 0xF0;
            core::ptr::write_unaligned((st + 8) as *mut u32, ps as u32);
            core::ptr::write_unaligned((st + 12) as *mut u32, ps as u32);
            core::ptr::write_unaligned((st + 36) as *mut u32, 0x6000_0020);
            core::ptr::write_unaligned((st + 40 + 8) as *mut u32, 0x100);
            core::ptr::write_unaligned((st + 40 + 12) as *mut u32, (2 * ps) as u32);
            core::ptr::write_unaligned((st + 40 + 36) as *mut u32, 0xC000_0040);
            {
                let g = super::RegfileImageGuard::new(fb);
                assert_eq!(g.rw_pages, vec![(fb + 2 * ps, fb + 3 * ps)]);
                assert_eq!(g.ro_pages, vec![(fb, fb + 2 * ps)]);
                core::ptr::write_volatile((fb + 2 * ps) as *mut u64, 0x1234_5678);
            }
            // `Drop` must restore `PROT_READ | PROT_WRITE` on `ro_pages`.
            core::ptr::write_volatile((fb + ps) as *mut u64, 0xCAFE_BABE);
            munmap(fake, fake_len);

            let arena_buf = super::mmap(core::ptr::null_mut(), super::RF_ARENA + 0x1_0000_0000, 0, 0x0002 | 0x1000, -1, 0);
            assert!(!arena_buf.is_null() && arena_buf as usize != usize::MAX);
            let raw_a = arena_buf as usize;
            let arena_addr = (raw_a + 0xFFFF_FFFF) & !0xFFFF_FFFF;
            unsafe extern "C" {
                fn mprotect(addr: *mut u8, len: usize, prot: i32) -> i32;
            }
            assert_eq!(mprotect(arena_addr as *mut u8, super::RF_ARENA, 1 | 2), 0);
            let stack_buf = super::mmap(core::ptr::null_mut(), super::RF_STACK, 1 | 2, 0x0002 | 0x1000, -1, 0);
            assert!(!stack_buf.is_null() && stack_buf as usize != usize::MAX);
            let stack_addr = stack_buf as usize;

            let mut a_init = vec![0u8; super::RF_ARENA];
            let mut s_init = vec![0u8; super::RF_STACK];
            let entry_va = 0x7000_8904_34u64;
            let cont_va = entry_va + 0xcbb;

            for t in 0..16 {
                let _ = super::seed_regfile_trial(entry_va, 0, t, arena_addr, stack_addr, &mut a_init, &mut s_init);
                let top = u64::from_le_bytes(s_init[super::RF_RSP_OFF..super::RF_RSP_OFF + 8].try_into().unwrap());
                assert_eq!(top, super::RF_RET_SENTINEL);
            }
            let _ = super::seed_regfile_trial(cont_va, 247, 0, arena_addr, stack_addr, &mut a_init, &mut s_init);
            let top0 = u64::from_le_bytes(s_init[super::RF_RSP_OFF..super::RF_RSP_OFF + 8].try_into().unwrap());
            assert_eq!(top0, super::RF_RET_SENTINEL);

            let _ = super::seed_regfile_trial(cont_va, 247, 1, arena_addr, stack_addr, &mut a_init, &mut s_init);
            let top1 = u64::from_le_bytes(s_init[super::RF_RSP_OFF..super::RF_RSP_OFF + 8].try_into().unwrap());
            assert_eq!(top1 & 1, 1);
            assert!((arena_addr + 0x1000..arena_addr + 0x3000).contains(&((top1 - 1) as usize)));

            let v8_path = std::env::var_os("PORT_GAME_DIR")
                .or_else(|| std::env::var_os("AW2_GAME_DIR"))
                .filter(|v| !v.is_empty())
                .map(|d| std::path::PathBuf::from(d).join("v8.dll"));
            if let Some(raw) = v8_path.as_deref().and_then(|p| std::fs::read(p).ok()) {
                let code_page = (entry_va as usize) & !(ps - 1);
                let code_len = ps * 2;
                let cp = super::mmap(code_page as *mut u8, code_len, 1 | 2, 0x0002 | 0x1000, -1, 0);
                if cp as usize == code_page {
                    let pe = u32::from_le_bytes(raw[0x3c..0x40].try_into().unwrap()) as usize;
                    let nsec = u16::from_le_bytes(raw[pe + 6..pe + 8].try_into().unwrap()) as usize;
                    let opt_sz = u16::from_le_bytes(raw[pe + 20..pe + 22].try_into().unwrap()) as usize;
                    let sh0 = pe + 24 + opt_sz;
                    let rva_lo = code_page - super::super::v8::IMAGE.base;
                    for i in 0..nsec {
                        let sh = sh0 + i * 40;
                        let vsz = u32::from_le_bytes(raw[sh + 8..sh + 12].try_into().unwrap()) as usize;
                        let srva = u32::from_le_bytes(raw[sh + 12..sh + 16].try_into().unwrap()) as usize;
                        let rsz = u32::from_le_bytes(raw[sh + 16..sh + 20].try_into().unwrap()) as usize;
                        let rptr = u32::from_le_bytes(raw[sh + 20..sh + 24].try_into().unwrap()) as usize;
                        if srva <= rva_lo && rva_lo + code_len <= srva + vsz.min(rsz) {
                            let foff = rptr + (rva_lo - srva);
                            core::ptr::copy_nonoverlapping(raw[foff..].as_ptr(), cp, code_len);
                            break;
                        }
                    }
                    assert_eq!(mprotect(cp, code_len, 1), 0);

                    for t in [0u32, 1u32] {
                        let cpu0 =
                            super::seed_regfile_trial(cont_va, 247, t, arena_addr, stack_addr, &mut a_init, &mut s_init);
                        core::ptr::copy_nonoverlapping(a_init.as_ptr(), arena_addr as *mut u8, super::RF_ARENA);
                        core::ptr::copy_nonoverlapping(s_init.as_ptr(), stack_addr as *mut u8, super::RF_STACK);
                        let mut cpu_g = cpu0.clone();
                        let ex_g = crate::emu::regfile::run_oracle(&mut cpu_g, cont_va, entry_va, entry_va + 0xd2c);
                        let ag = core::slice::from_raw_parts(arena_addr as *const u8, super::RF_ARENA).to_vec();
                        let sg = core::slice::from_raw_parts(stack_addr as *const u8, super::RF_STACK).to_vec();

                        core::ptr::copy_nonoverlapping(a_init.as_ptr(), arena_addr as *mut u8, super::RF_ARENA);
                        core::ptr::copy_nonoverlapping(s_init.as_ptr(), stack_addr as *mut u8, super::RF_STACK);
                        let mut cpu_l = cpu0;
                        let ex_l =
                            super::super::v8::rf_7000890434(&mut cpu_l as *mut crate::emu::Cpu as *mut u8, 247);
                        let al = core::slice::from_raw_parts(arena_addr as *const u8, super::RF_ARENA);
                        let sl = core::slice::from_raw_parts(stack_addr as *const u8, super::RF_STACK);

                        assert_eq!(ex_g, ex_l, "trial {t}: exit parity");
                        assert_eq!(cpu_g.gpr, cpu_l.gpr, "trial {t}: gpr parity");
                        assert_eq!(cpu_g.xmm, cpu_l.xmm, "trial {t}: xmm parity");
                        assert_eq!(cpu_g.rflags(), cpu_l.rflags(), "trial {t}: rflags parity");
                        assert_eq!(cpu_g.last_bits, cpu_l.last_bits, "trial {t}: last_bits parity");
                        assert_eq!(ag.as_slice(), al, "trial {t}: arena parity");
                        assert_eq!(sg.as_slice(), sl, "trial {t}: stack parity");
                        if t == 0 {
                            assert_eq!(ex_l.kind, crate::emu::regfile::RF_FAULT);
                        } else {
                            assert_eq!(ex_l.kind, crate::emu::regfile::RF_RET);
                        }
                    }
                }
                if !cp.is_null() && cp as usize != usize::MAX {
                    munmap(cp, code_len);
                }
            }

            munmap(stack_buf, super::RF_STACK);
            munmap(arena_buf, super::RF_ARENA + 0x1_0000_0000);
        }
    }

    #[test]
    fn verify_e9_chain_and_df_alias_metadata() {
        assert_eq!(core::mem::size_of::<super::super::Lifted>(), 24);
        let mut fake = vec![0xCCu8; 0x300];
        let base = fake.as_ptr() as usize;
        let emit_e9 = |buf: &mut [u8], from_off: usize, to_off: usize| {
            buf[from_off] = 0xE9;
            let rel = (to_off as isize - (from_off as isize + 5)) as i32;
            buf[from_off + 1..from_off + 5].copy_from_slice(&rel.to_le_bytes());
        };
        // Target at +0x200 (`mov eax, 1; ret` — does not begin with 0xE9).
        fake[0x200..0x206].copy_from_slice(&[0xB8, 1, 0, 0, 0, 0xC3]);
        // 1-hop chain: +0x100 -> +0x200.
        emit_e9(&mut fake, 0x100, 0x200);
        // 6-hop chain: +0x110 -> +0x120 -> +0x130 -> +0x140 -> +0x150 -> +0x160 -> +0x200.
        for k in 0..5 {
            emit_e9(&mut fake, 0x110 + k * 0x10, 0x120 + k * 0x10);
        }
        emit_e9(&mut fake, 0x160, 0x200);
        // 7-hop chain: +0x105 -> +0x110 -> ... -> +0x200 (exceeds 6-hop cap).
        emit_e9(&mut fake, 0x105, 0x110);
        // Non-E9 stub (`eb 00`).
        fake[0x180..0x182].copy_from_slice(&[0xEB, 0x00]);

        let tva = (base + 0x200) as u64;
        unsafe {
            assert_eq!(super::verify_e9_chain(base, 0x100, 0x180, (base + 0x100) as u64, tva), Ok(1));
            assert_eq!(super::verify_e9_chain(base, 0x100, 0x180, (base + 0x110) as u64, tva), Ok(6));
            assert!(super::verify_e9_chain(base, 0x100, 0x180, (base + 0x105) as u64, tva).is_err());
            assert!(super::verify_e9_chain(base, 0x100, 0x190, (base + 0x180) as u64, tva).is_err());
            // Target starting with 0xE9 must be rejected (`+0x160` starts with 0xE9).
            assert!(super::verify_e9_chain(base, 0x100, 0x180, (base + 0x150) as u64, (base + 0x160) as u64).is_err());
            // Out-of-.text start or target must be rejected.
            assert!(super::verify_e9_chain(base, 0x110, 0x170, (base + 0x100) as u64, tva).is_err());
            assert!(super::verify_e9_chain(base, 0x100, 0x100, (base + 0x100) as u64, tva).is_err());
        }

        // Every `DF_ALIAS` across all 8 generated `IMAGES` must point to a non-alias target in the
        // same image with identical `ptr` and `LEAF | XMM_RET` flags.
        let mut total_aliases = 0usize;
        for img in super::super::IMAGES.iter().copied() {
            let by_va: std::collections::HashMap<u64, (*const (), u8)> = img
                .slices
                .iter()
                .copied()
                .flatten()
                .map(|e| (e.va, (e.ptr, e.flags)))
                .collect();
            for e in img.slices.iter().copied().flatten() {
                if e.flags & super::super::DF_ALIAS != 0 {
                    assert_ne!(e.alias_rva, 0, "{} 0x{:x}: zero alias_rva", img.name, e.va);
                    let target_va = img.base as u64 + e.alias_rva as u64;
                    let &(tptr, tflags) = by_va
                        .get(&target_va)
                        .unwrap_or_else(|| panic!("{} 0x{:x}: missing alias target 0x{target_va:x}", img.name, e.va));
                    assert_eq!(tflags & super::super::DF_ALIAS, 0, "{} 0x{:x}: chained DF_ALIAS", img.name, e.va);
                    assert_eq!(tptr as usize, e.ptr as usize, "{} 0x{:x}: ptr mismatch", img.name, e.va);
                    let mask = super::super::LEAF | super::super::XMM_RET;
                    assert_eq!(tflags & mask, e.flags & mask, "{} 0x{:x}: flags mismatch", img.name, e.va);
                    total_aliases += 1;
                } else {
                    assert_eq!(e.alias_rva, 0, "{} 0x{:x}: unexpected alias_rva on non-alias", img.name, e.va);
                }
            }
        }
        assert!(total_aliases > 0, "expected generated DF_ALIAS entries across IMAGES");
    }

    #[test]
    fn memstack_verdict_and_shifted_stub_mode4() {
        use core::sync::atomic::Ordering::Relaxed;
        let m = super::Meta {
            guest_rsp: 0x0000_007d_2bd6_f548,
            lifted_sp: 0x0000_0001_217f_ff80,
            ..super::Meta::default()
        };
        let rg = 0x0000_0005_0080_0123u64;
        let win = [0u64; super::WIN];
        let img = super::frame_image(rg, &win);

        // 1) Unaligned 8-byte stack pointers (`% 8 == 2` and `% 8 == 3`, including equal low byte).
        let g_stk = 0x0000_007d_2bd6_f490u64;
        let g_stk_eq_lo = 0x0000_007d_2bd6_f428u64;
        let l_stk = 0x0000_0001_217f_ff28u64;
        for (off, gs) in [(2usize, g_stk), (3usize, g_stk), (2usize, g_stk_eq_lo)] {
            let mut g = vec![0xA5u8; 64];
            let mut l = g.clone();
            g[off..off + 8].copy_from_slice(&gs.to_le_bytes());
            l[off..off + 8].copy_from_slice(&l_stk.to_le_bytes());
            assert_eq!(super::memstack_verdict(&g, &l, &m, &img), super::MEMSTACK_ONLY);
        }

        // Non-stack unaligned difference must remain a mismatch (`0`).
        {
            let mut g = vec![0xA5u8; 64];
            let mut l = g.clone();
            g[2..10].copy_from_slice(&0xdead_beef_cafe_babeu64.to_le_bytes());
            l[2..10].copy_from_slice(&0x1122_3344_5566_7788u64.to_le_bytes());
            assert_eq!(super::memstack_verdict(&g, &l, &m, &img), 0);
        }

        // Unaligned stack pointer + caller-frame overread (`rg` vs `0`) -> `OVERREAD_ONLY`.
        {
            let mut g = vec![0u8; 64];
            let mut l = vec![0u8; 64];
            g[2..10].copy_from_slice(&g_stk.to_le_bytes());
            l[2..10].copy_from_slice(&l_stk.to_le_bytes());
            g[32..40].copy_from_slice(&rg.to_le_bytes());
            assert_eq!(super::memstack_verdict(&g, &l, &m, &img), super::OVERREAD_ONLY);
        }

        // 2) Aligned `is_stack_pad_qword` (`{ i32 tag; /* 4B pad */ }` copied via 128-bit load/store).
        {
            let mut g = vec![0u8; 32];
            let mut l = vec![0u8; 32];
            g[16..24].copy_from_slice(&0x0000_007d_0000_0003u64.to_le_bytes());
            l[16..24].copy_from_slice(&0x0000_0001_0000_0003u64.to_le_bytes());
            assert_eq!(super::memstack_verdict(&g, &l, &m, &img), super::MEMSTACK_ONLY);

            // Different low 32-bit tag or non-stack upper 32 bits must NOT match.
            l[16..24].copy_from_slice(&0x0000_0001_0000_0004u64.to_le_bytes());
            assert_eq!(super::memstack_verdict(&g, &l, &m, &img), 0);
            g[16..24].copy_from_slice(&0x0000_0099_0000_0003u64.to_le_bytes());
            l[16..24].copy_from_slice(&0x0000_0001_0000_0003u64.to_le_bytes());
            assert_eq!(super::memstack_verdict(&g, &l, &m, &img), 0);
        }

        // 3) `stub_variant(4)` (`CHILD_ZERO_FRAME = 4`): preserves stack args 9..=31 and shifts
        // `guest_entry_rsp` by `-0x5_5540` while keeping `rsp % 16 == 8`.
        let _g = STUB_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let mut hi = w([0x99, 0, 0, 0, 0, 0, 0, 0]);
        hi[30] = 0x31;
        let load = |disp: u32| {
            let mut c = vec![0x48, 0x8B, 0x84, 0x24];
            c.extend(disp.to_le_bytes());
            c.push(0xC3);
            leak(&c)
        };
        let get_rsp = leak(&[0x48, 0x89, 0xE0, 0xC3]); // mov rax, rsp ; ret
        unsafe {
            super::CHILD_ZERO_FRAME.store(0, Relaxed);
            let rsp0 = guest_run_checked(get_rsp, [0; 8], [0; 4], &hi).map(|r| r.0);
            super::CHILD_ZERO_FRAME.store(4, Relaxed);
            let rsp4 = guest_run_checked(get_rsp, [0; 8], [0; 4], &hi).map(|r| r.0);
            let r9 = guest_run_checked(load(0x48), [0; 8], [0; 4], &hi).map(|r| r.0);
            let r31 = guest_run_checked(load(0xf8), [0; 8], [0; 4], &hi).map(|r| r.0);
            super::CHILD_ZERO_FRAME.store(0, Relaxed);
            assert_eq!(r9, Some(0x99));
            assert_eq!(r31, Some(0x31));
            let (rsp0, rsp4) = (rsp0.expect("rsp0"), rsp4.expect("rsp4"));
            assert_eq!(rsp0 - rsp4, 0x5_5540);
            assert_eq!(rsp4 & 0xf, 8);
        }
    }
}
