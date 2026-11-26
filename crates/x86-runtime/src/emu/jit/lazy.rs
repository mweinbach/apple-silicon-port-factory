//! Lazy flag / `last_bits` materialisation at block boundaries (`AW2_JIT_FLAGLAZY`).
//!
//! Everything is decided at translation time; no new runtime invariant.
//!
//! - Successor flag liveness looks through a successor block that ends in a direct `jmp` /
//!   `jcc` (`succ_need` depth [`FLAGLAZY_DEPTH`] instead of 1).
//! - `Jcc` terminators branch first and flush flags / `last_bits` per edge (each edge with its
//!   own successor's needs) when the two edges differ: the hot back-edge of a loop no longer
//!   pays for the flags its exit edge needs. `cset`/`strb` don't touch NZCV, so the flush
//!   can sit after the `b.cond`.
//! - The parity byte of an ALU / inc / dec result that stays in its guest register for the
//!   rest of the block is not stored at the op: `Tr::pf` remembers the register and
//!   `flush_flags` (terminator edges, side exits) or the next PF reader stores it. Every op
//!   outside a small whitelist (that provably doesn't write the register) flushes it first.
//! - A pending `last_bits[i]` store at an edge is dropped when the edge's successor block
//!   (statically known, not host-resolved, not run-time generated code) writes guest reg `i`
//!   with an op that sets `last_bits[i]` in both the interpreter (`rset`) and the translator
//!   (`set_lb`) before any op that could read it (only host transfers read it, through
//!   `abi_gpr`) or leave the straight-line block. Whatever way the successor is entered (linked
//!   block, budget exit, interpreter, side exit before the write), its ops run in order and
//!   overwrite `last_bits[i]` before anything reads it.
//! - The remaining zero `last_bits` stores of one flush are merged (`strh` / `str xzr`); a
//!   merged store only covers tracked bytes being zeroed anyway and untracked bytes
//!   (`abi_gpr` never reads them; the translator already leaves them stale).
//!
//! `AW2_JIT_FLAGLAZY=0`: depth-1 liveness, one union flush before the branch, eager PF, one
//! byte store per register: exactly the jit6 code.

use super::cache::{can_speculate_code, resolved_host};
use super::translate::{op_flags, supported, Cond, Tr};
use super::*;
use crate::emu::fast::{self, FastOp};
use crate::emu::{mem, to_mapped_addr, BlockCache};

/// `AW2_JIT_FLAGLAZY` (default 1): lazy flag / `last_bits` materialisation at block
/// boundaries. `0`: jit6 code.
pub(in crate::emu) fn flaglazy_enabled() -> bool {
    #[cfg(test)]
    if let Some(v) = TEST_KNOBS.with(|c| c.get().flaglazy) {
        return v;
    }
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| env_knob("AW2_JIT_FLAGLAZY", 1) != 0)
}

/// Successor-block depth of the flag liveness scan with `AW2_JIT_FLAGLAZY`.
pub(super) const FLAGLAZY_DEPTH: u32 = 3;

/// No pending parity byte (`Tr::pf`).
pub(super) const PF_NONE: u8 = 0xFF;

/// `last_bits` bits of the tracked GPRs (rax, rcx, rdx, r8, r9).
const LB_TRACKED_MASK: u16 = 1 | 1 << 1 | 1 << 2 | 1 << 8 | 1 << 9;

/// Per-edge successor information of a block's terminator: index 0 = taken target (or the
/// only successor), 1 = fall-through of a `Jcc`.
#[derive(Clone, Copy)]
pub(in crate::emu) struct Edges {
    /// Flags the successor may read (`succ_need`).
    pub(in crate::emu) live: [u8; 2],
    /// Tracked GPRs whose `last_bits` the successor overwrites before any reader.
    pub(super) kill: [u16; 2],
}

/// `last_bits` effect of one op of a successor block: `Some(mask)` of the GPRs whose
/// `last_bits` it sets in both the interpreter and the translator (0 = neutral), `None` =
/// stop scanning (control transfer, possible host transfer, generic op).
fn lb_kill_of(op: &FastOp) -> Option<u16> {
    use fast::*;
    let r0 = 1u16 << (op.r0 & 15);
    Some(match op.kind {
        K_MOV_RR | K_MOV_RM | K_MOV_RI | K_MOVZX_RR | K_MOVZX_RM | K_MOVSX_RR | K_MOVSX_RM | K_LEA | K_SETCC_R
        | K_POP_R => r0,
        K_ALU_RR | K_ALU_RI | K_ALU_RM if (op.x & 0x3F) <= A_XOR => r0,
        K_JCC | K_JMP | K_CALL | K_CALL_R | K_CALL_M | K_JMP_R | K_JMP_M | K_RET | K_LOOP | K_SYS_MISC | K_INSN => {
            return None
        }
        _ => 0,
    })
}

/// Tracked GPRs whose `last_bits` the block at guest `ip` overwrites before anything can
/// read them (see the notes above); 0 when unknown or `lazy` is off.
pub(super) unsafe fn succ_lb_kill(bc: *mut BlockCache, ip: u64, lazy: bool) -> u16 {
    if !lazy || ip == 0 || !can_speculate_code(ip) || resolved_host(bc, ip) || mem::in_jit(to_mapped_addr(ip)) {
        return 0;
    }
    let mut dummy = 0u64;
    let Ok((off, n)) = BlockCache::get(bc, ip, &mut dummy) else { return 0 };
    let mut kill = 0u16;
    for i in 0..n {
        let op = (&(*bc).ops)[off + i];
        if !supported(&op) {
            break;
        }
        match lb_kill_of(&op) {
            Some(m) => kill |= m,
            None => break,
        }
    }
    kill & LB_TRACKED_MASK
}

/// `op` neither writes guest GPR `r` nor is anything but a plain integer / memory / control
/// op (the pending parity byte in `r` survives it).
fn pf_keeps(op: &FastOp, r: u8) -> bool {
    use fast::*;
    let w0 = 1u32 << (op.r0 & 15);
    let writes = match op.kind {
        K_NOP | K_MOV_MR | K_MOV_MI | K_ALU_MR | K_ALU_MI | K_CMPJ_RR | K_CMPJ_RI | K_CMPJ_RM | K_JCC | K_JMP
        | K_JMP_R | K_JMP_M => 0,
        K_CALL | K_CALL_R | K_CALL_M | K_RET | K_PUSH_R => 1 << 4,
        K_POP_R => w0 | 1 << 4,
        K_MOV_RR | K_MOV_RM | K_MOV_RI | K_MOVZX_RR | K_MOVZX_RM | K_MOVSX_RR | K_MOVSX_RM | K_LEA | K_ALU_RR
        | K_ALU_RI | K_ALU_RM | K_INC_R | K_DEC_R | K_SETCC_R => w0,
        _ => return false,
    };
    writes & (1u32 << (r & 15)) == 0
}

impl Tr<'_> {
    /// Op `k` computed PF (`need_pf`) as the low byte of `res`, its destination guest GPR
    /// `r`: with `AW2_JIT_FLAGLAZY`, leave it there (pending) if no later op of the block
    /// (terminator included) may write `r`. Returns whether it was deferred.
    pub(super) fn pf_defer(&mut self, k: usize, res: u32, r: u8) -> bool {
        // A newer parity value supersedes any pending one.
        self.pf = PF_NONE;
        if !self.lazy || r & 15 == 4 || res != g(r) {
            return false;
        }
        if self.ops[k + 1..].iter().all(|o| pf_keeps(o, r)) {
            self.pf = r & 15;
            true
        } else {
            false
        }
    }

    /// Store the pending parity byte.
    pub(super) fn flush_pf(&mut self) {
        let r = self.pf;
        self.pf = PF_NONE;
        self.a.str(8, g(r), XCPU, OFF_PFV);
        self.fs.mem |= FP;
    }

    /// Before translating op `k` with a parity byte pending: store it if the op may read PF
    /// or isn't on the whitelist; forget it if the op overwrites PF.
    pub(super) fn pf_before_op(&mut self, k: usize) {
        let op = self.ops[k];
        let (r, kl) = op_flags(&op);
        if !pf_keeps(&op, self.pf) || r & FP != 0 {
            self.flush_pf();
        } else if kl & FP != 0 {
            self.pf = PF_NONE;
        }
    }

    /// Flush the translation-time `last_bits` codes, dropping those in `kill` (overwritten
    /// by the successor before any reader) and merging the zero stores.
    pub(super) fn flush_lb_edge(&mut self, kill: u16) {
        if !self.lb_on {
            return;
        }
        if !self.lazy {
            for i in TRACKED {
                self.flush_lb_reg(i as usize);
            }
            return;
        }
        let mut zero = 0u16;
        for i in TRACKED {
            let v = self.lb[i as usize];
            if v == LB_NONE {
                continue;
            }
            self.lb[i as usize] = LB_NONE;
            if kill & (1 << i) != 0 {
                continue;
            }
            if v == 0 {
                zero |= 1 << i;
            } else {
                self.a.movz(false, XS, v as u32, 0);
                self.a.str(8, XS, XCPU, OFF_LB + i as u32);
            }
        }
        const R0: u16 = 1;
        const R1: u16 = 1 << 1;
        const R2: u16 = 1 << 2;
        const R8: u16 = 1 << 8;
        const R9: u16 = 1 << 9;
        // last_bits[2..10]: rdx, (rbx, rsp, rbp, rsi, rdi untracked), r8, r9.
        if zero & (R2 | R8 | R9) == R2 | R8 | R9 && (OFF_LB + 2) % 8 == 0 {
            self.a.str(64, ZR, XCPU, OFF_LB + 2);
            zero &= !(R2 | R8 | R9);
        }
        if zero & (R8 | R9) == R8 | R9 && (OFF_LB + 8) % 2 == 0 {
            self.a.str(16, ZR, XCPU, OFF_LB + 8);
            zero &= !(R8 | R9);
        }
        if zero & (R0 | R1) == R0 | R1 && OFF_LB % 2 == 0 {
            self.a.str(16, ZR, XCPU, OFF_LB);
            zero &= !(R0 | R1);
        }
        for i in TRACKED {
            if zero & (1 << i) != 0 {
                self.a.str(8, ZR, XCPU, OFF_LB + i as u32);
            }
        }
    }

    /// Flags and `last_bits` for leaving through edge `e` (`AW2_JIT_FLAGLAZY`).
    pub(super) fn flush_edge(&mut self, e: usize) {
        let (live, kill) = (self.edge.live[e], self.edge.kill[e]);
        self.flush_flags(live);
        self.flush_lb_edge(kill);
    }

    /// `Jcc` terminator with `AW2_JIT_FLAGLAZY`: branch on the condition first, then flush
    /// each edge with its own successor's needs (one shared flush before the branch when
    /// both edges want the same).
    pub(super) fn jcc_lazy(&mut self, cc: u8, target: u64, fall: u64, near: bool) {
        let split = self.edge.live[0] != self.edge.live[1]
            || (self.lb_on && self.edge.kill[0] != self.edge.kill[1]);
        if !split {
            self.flush_edge(0);
        }
        let taken_needs_flush = split
            && ((self.edge.live[0] & self.fs.host & !self.fs.mem) != 0
                || (self.edge.live[0] & FP != 0 && self.pf != PF_NONE)
                || (self.lb_on
                    && TRACKED
                        .iter()
                        .any(|&i| self.lb[i as usize] != LB_NONE && (self.edge.kill[0] & (1 << i)) == 0)));
        let c = self.cond(cc);
        let taken_host = near && self.target_host;
        // Fast 2-word main-path Jcc (`b.cond/cbz/cbnz #cold_link; [fall flush]; b #fall`) when
        // the taken edge needs no extra flush and is a direct guest target: puts `b #fall` last
        // so chunked AOT hot/cold layout turns fallthrough into a 0-cycle `nop`, and patches
        // `b.cond/cbz/cbnz` directly to `target` when within ±1 MiB.
        if !taken_host && !taken_needs_flush {
            match c {
                Cond::Const(true) => {
                    self.exit_to(target);
                    return;
                }
                Cond::Const(false) => {
                    if split {
                        self.flush_edge(1);
                    }
                    self.exit_to(fall);
                    return;
                }
                Cond::Host(cnd) => {
                    let f = self.a.bcond(cnd);
                    self.cond_links.push((f, target));
                    if split {
                        self.flush_edge(1);
                    }
                    self.exit_to(fall);
                    return;
                }
                Cond::Reg(r, inv) => {
                    let f = if inv { self.a.cbz(false, r) } else { self.a.cbnz(false, r) };
                    self.cond_links.push((f, target));
                    if split {
                        self.flush_edge(1);
                    }
                    self.exit_to(fall);
                    return;
                }
            }
        }
        let fix = match c {
            Cond::Const(true) => None,
            Cond::Const(false) => {
                if split {
                    self.flush_edge(1);
                }
                self.exit_to(fall);
                return;
            }
            Cond::Host(cnd) => Some(self.a.bcond(cnd)),
            Cond::Reg(r, inv) => Some(if inv { self.a.cbz(false, r) } else { self.a.cbnz(false, r) }),
        };
        if let Some(f) = fix {
            let snap = (self.fs, self.lb, self.pf);
            if split {
                self.flush_edge(1);
            }
            self.exit_to(fall);
            self.a.bind(f);
            (self.fs, self.lb, self.pf) = snap;
        }
        let lbk = self.abi_lb();
        if split {
            self.flush_edge(0);
        }
        if taken_host {
            self.emit_tail_target(target, fall, lbk, EXIT_JCC);
        } else {
            self.exit_to(target);
        }
    }
}
