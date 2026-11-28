//! Probe outcome types for [`super`].

use win32_rt::libc;

pub const N_OUTCOMES: usize = 9;

/// Termination kind recorded from `waitpid` (`Report::run_exit_kind`).
pub const TERM_NONE: u8 = 0;
pub const TERM_EXITED: u8 = 1;
pub const TERM_SIGNALED: u8 = 2;
pub const TERM_STOPPED: u8 = 3;
pub const TERM_WAIT_ERR: u8 = 4;

/// Decoded child termination from a raw `waitpid` status (or `-errno` when `waitpid < 0`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChildExit {
    NotRun,
    Exited(i32),
    Signaled { sig: i32, core_dumped: bool },
    Stopped(i32),
    WaitError(i32),
}

impl ChildExit {
    pub fn from_raw(raw_status: i32) -> Self {
        if raw_status < 0 {
            return ChildExit::WaitError(-raw_status);
        }
        if libc::WIFEXITED(raw_status) {
            ChildExit::Exited(libc::WEXITSTATUS(raw_status))
        } else if libc::WIFSIGNALED(raw_status) {
            ChildExit::Signaled {
                sig: libc::WTERMSIG(raw_status),
                core_dumped: (raw_status & 0x80) != 0,
            }
        } else if libc::WIFSTOPPED(raw_status) {
            ChildExit::Stopped(libc::WSTOPSIG(raw_status))
        } else {
            ChildExit::WaitError(0)
        }
    }

    pub fn from_parts(kind: u8, code: i32, raw_status: i32) -> Self {
        match kind {
            TERM_NONE => ChildExit::NotRun,
            TERM_EXITED => ChildExit::Exited(code),
            TERM_SIGNALED => ChildExit::Signaled { sig: code, core_dumped: raw_status >= 0 && (raw_status & 0x80) != 0 },
            TERM_STOPPED => ChildExit::Stopped(code),
            TERM_WAIT_ERR => ChildExit::WaitError(code),
            _ => Self::from_raw(raw_status),
        }
    }

    pub fn to_parts(self) -> (u8, i32) {
        match self {
            ChildExit::NotRun => (TERM_NONE, 0),
            ChildExit::Exited(c) => (TERM_EXITED, c),
            ChildExit::Signaled { sig, .. } => (TERM_SIGNALED, sig),
            ChildExit::Stopped(sig) => (TERM_STOPPED, sig),
            ChildExit::WaitError(err) => (TERM_WAIT_ERR, err),
        }
    }

    pub fn describe(self) -> String {
        match self {
            ChildExit::NotRun => "not_run".into(),
            ChildExit::Exited(code) => format!("exited({code})"),
            ChildExit::Signaled { sig, core_dumped } => {
                let name = signal_name(sig);
                if core_dumped {
                    format!("signaled({name}={sig},core)")
                } else {
                    format!("signaled({name}={sig})")
                }
            }
            ChildExit::Stopped(sig) => format!("stopped({}={sig})", signal_name(sig)),
            ChildExit::WaitError(err) => format!("waitpid_err({err})"),
        }
    }
}

pub fn signal_name(sig: i32) -> &'static str {
    match sig {
        libc::SIGHUP => "SIGHUP",
        libc::SIGINT => "SIGINT",
        libc::SIGQUIT => "SIGQUIT",
        libc::SIGILL => "SIGILL",
        libc::SIGTRAP => "SIGTRAP",
        libc::SIGABRT => "SIGABRT",
        libc::SIGBUS => "SIGBUS",
        libc::SIGFPE => "SIGFPE",
        libc::SIGKILL => "SIGKILL",
        libc::SIGSEGV => "SIGSEGV",
        libc::SIGSYS => "SIGSYS",
        libc::SIGPIPE => "SIGPIPE",
        libc::SIGALRM => "SIGALRM",
        libc::SIGTERM => "SIGTERM",
        _ => "SIG?",
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Outcome {
    Equal = 0,
    Mismatch = 1,
    AbiNarrow = 2,
    Nondet = 3,
    Skipped = 4,
    /// Every difference is a pair of pointers into the probe heap arena (allocation order
    /// or sizes differ, e.g. merged allocations); not counted as a mismatch.
    AllocLayout = 5,
    /// Memory equal; rax/xmm0 differ only in bits no caller reads (`rets=` table:
    /// x86 caller liveness, or the replacement's declared return type). Not equal.
    RetDead = 6,
    /// Every difference (memory qwords, and rax if it differs) is a pair of addresses into
    /// the probe's scratch stacks on BOTH sides (emulator stack below sp, lstack arena below
    /// top, host stack below the hook sp, free big-frame slots): pointers to dead frames
    /// whose values only reflect frame layout. Heap-arena pointer pairs may be mixed in.
    /// Not equal.
    StackLayout = 7,
    /// Every remaining difference changes when the scratch stacks are poisoned (G1 vs G3 or
    /// N vs N3): the bytes come from uninitialised stack memory. Not equal.
    Uninit = 8,
}

impl Outcome {
    pub fn name(self) -> &'static str {
        match self {
            Outcome::Equal => "equal",
            Outcome::Mismatch => "mismatch",
            Outcome::AbiNarrow => "abi_narrow",
            Outcome::Nondet => "nondet",
            Outcome::Skipped => "skipped",
            Outcome::AllocLayout => "alloc_layout",
            Outcome::RetDead => "ret_dead",
            Outcome::StackLayout => "stack_layout",
            Outcome::Uninit => "uninit",
        }
    }
    pub fn from_u32(v: u32) -> Outcome {
        match v {
            0 => Outcome::Equal,
            1 => Outcome::Mismatch,
            2 => Outcome::AbiNarrow,
            3 => Outcome::Nondet,
            5 => Outcome::AllocLayout,
            6 => Outcome::RetDead,
            7 => Outcome::StackLayout,
            8 => Outcome::Uninit,
            _ => Outcome::Skipped,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Reason {
    None = 0,
    GuestTimeout = 1,
    GuestFault = 2,
    DoubleTimeout = 3,
    SharedWrite = 4,
    TooManyPages = 5,
    NativeWildFault = 6,
    ForkFailed = 7,
    OrchestratorDied = 8,
    NativeFault = 9,
    NativeTimeout = 10,
    Setup = 11,
    /// A run reached a host API that cannot work in a forked child (CoreAudio, Metal, ...);
    /// [`Report::host_api`] names it.
    HostUnsafe = 12,
}

const REASONS: [(Reason, &str); 13] = [
    (Reason::None, ""),
    (Reason::GuestTimeout, "guest_timeout"),
    (Reason::GuestFault, "guest_fault"),
    (Reason::DoubleTimeout, "double_timeout"),
    (Reason::SharedWrite, "shared_write"),
    (Reason::TooManyPages, "too_many_pages"),
    (Reason::NativeWildFault, "native_wild_fault"),
    (Reason::ForkFailed, "fork_failed"),
    (Reason::OrchestratorDied, "orchestrator_died"),
    (Reason::NativeFault, "native_fault"),
    (Reason::NativeTimeout, "native_timeout"),
    (Reason::Setup, "setup"),
    (Reason::HostUnsafe, "host_unsafe"),
];

impl Reason {
    pub fn name(self) -> &'static str {
        REASONS[self as usize].1
    }
    pub fn from_u32(v: u32) -> Reason {
        REASONS.get(v as usize).map_or(Reason::None, |r| r.0)
    }
}

/// Fixed-size probe result (pipe record; also returned by `probe_blocking`).
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Report {
    pub seq: u64,
    pub va: u64,
    pub slot: u32,
    pub k: u32,
    pub outcome: u32,
    pub reason: u32,
    /// Per run: G1, N, G2.
    pub rax: [u64; 3],
    pub xmm0: [u128; 3],
    pub pages: [u32; 3],
    pub t_us: [u64; 3],
    pub fault_addr: [u64; 3],
    pub fault_pc: [u64; 3],
    /// Raw `waitpid` status (or `-errno` when `waitpid < 0`) per run: G1, N, G2.
    pub run_status: [i32; 3],
    /// Decoded termination kind ([`TERM_EXITED`], [`TERM_SIGNALED`], ...) per run: G1, N, G2.
    pub run_exit_kind: [u8; 3],
    /// Decoded exit status, terminating signal, or `waitpid` errno per run: G1, N, G2.
    pub run_exit_code: [i32; 3],
    /// Signal recorded by `on_fault` (`RunHdr::fault_sig`) per run: G1, N, G2.
    pub fault_sig: [u32; 3],
    pub diff_bytes: u64,
    pub diff_ranges: u32,
    /// Diff ranges where both sides hold pointers into the probe heap arena.
    pub alloc_layout_ranges: u32,
    /// Diff ranges inside libc malloc regions.
    pub heap_ranges: u32,
    /// Nondet only: N matched G1 or G2 outside the G1/G2 diff mask.
    pub native_consistent: u32,
    pub first_diff: u64,
    pub fork_us: u64,
    pub setup_us: u64,
    pub tracked_mb: u64,
    /// Orchestrator wait status and last stage (diagnostics).
    pub orch_status: i32,
    pub orch_stage: u32,
    /// Return-register use applied by the comparison: rax bits compared, whether xmm0
    /// was compared, and where that came from ([`RetUse`]).
    pub ret_mask: u64,
    pub ret_xmm: u32,
    pub ret_src: u32,
    /// Diff ranges where both sides hold addresses into the scratch stacks.
    pub stack_layout_ranges: u32,
    /// Poisoned runs (G3, N3): [`POISON_RAN`] | [`POISON_G_FAIL`] | [`POISON_N_FAIL`].
    pub poison: u32,
    /// rax of G3 and N3.
    pub rax_p: [u64; 2],
    /// Byte ranges that changed under poisoning: G1 vs G3, N vs N3 (uninitialised reads).
    pub uninit_ranges: [u32; 2],
    /// [`Reason::HostUnsafe`]: the host API (NUL-terminated).
    pub host_api: [u8; win32_rt::shadow::HOST_API_LEN],
}

impl Report {
    /// [`Report::host_api`] as text.
    pub fn host_api_name(&self) -> String {
        let n = self.host_api.iter().position(|&b| b == 0).unwrap_or(self.host_api.len());
        String::from_utf8_lossy(&self.host_api[..n]).into_owned()
    }
}

/// [`Report::poison`]: the poisoned runs ran (only when G1 and N differ).
pub const POISON_RAN: u32 = 1;
/// [`Report::poison`]: G3 did not complete (fault/timeout: control depends on stale stack).
pub const POISON_G_FAIL: u32 = 2;
/// [`Report::poison`]: N3 did not complete.
pub const POISON_N_FAIL: u32 = 4;

/// Which return-register bits callers use (`rets=` table, see `tools/fndb/retlive.py`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RetUse {
    /// rax bits compared (`u64::MAX` = all).
    pub rax_mask: u64,
    /// xmm0 compared (when the dispatch returns xmm0 at all).
    pub xmm: bool,
    /// Source: low nibble for rax, high nibble for xmm0 ([`RET_FULL`], [`RET_X86`], [`RET_SIG`]).
    pub src: u32,
}

/// No information: compare everything.
pub const RET_FULL: u32 = 0;
/// x86 caller liveness (`fnsig retlive`).
pub const RET_X86: u32 = 1;
/// The replacement's declared Rust return type (x86 inconclusive).
pub const RET_SIG: u32 = 2;

impl Default for RetUse {
    fn default() -> Self {
        RetUse { rax_mask: u64::MAX, xmm: true, src: RET_FULL }
    }
}

impl RetUse {
    /// From one `rets=` row: `rax` ∈ {0,8,16,32,64,?}, `xmm` ∈ {0,1,?}, `ind` (callers not
    /// all known), `sig` ∈ {void,int,float,?}. Unknown callers → full compare.
    pub fn from_row(rax: &str, xmm: &str, ind: bool, sig: &str) -> RetUse {
        if ind {
            return RetUse::default();
        }
        let x86_rax = match rax {
            "0" => Some(0u64),
            "8" => Some(0xff),
            "16" => Some(0xffff),
            "32" => Some(0xffff_ffff),
            "64" => Some(u64::MAX),
            _ => None,
        };
        let (rax_mask, rs) = match (x86_rax, sig) {
            (Some(m), _) => (m, RET_X86),
            (None, "void" | "float") => (0, RET_SIG),
            (None, _) => (u64::MAX, RET_FULL),
        };
        // Win64 returns in exactly one of rax / xmm0.
        let (xmm, xs) = match (xmm, sig) {
            ("1", _) => (true, RET_X86),
            ("0", _) => (false, RET_X86),
            (_, "void" | "int") => (false, RET_SIG),
            (_, "float") => (true, RET_SIG),
            _ if x86_rax.is_some_and(|m| m != 0) => (false, RET_X86),
            _ => (true, RET_FULL),
        };
        RetUse { rax_mask, xmm, src: rs | (xs << 4) }
    }

    pub fn src_name(s: u32) -> &'static str {
        match s {
            RET_X86 => "x86",
            RET_SIG => "sig",
            _ => "full",
        }
    }
}

impl Report {
    pub fn outcome(&self) -> Outcome {
        Outcome::from_u32(self.outcome)
    }
    pub fn reason(&self) -> Reason {
        Reason::from_u32(self.reason)
    }
    /// Decoded child termination for run `r` (`0` = G1, `1` = N, `2` = G2).
    pub fn run_exit(&self, r: usize) -> ChildExit {
        ChildExit::from_parts(self.run_exit_kind[r], self.run_exit_code[r], self.run_status[r])
    }
    /// Human-readable termination summary for G1, N, and G2 (for logs, dumps, and test failures).
    pub fn run_diagnostics(&self) -> String {
        ["G1", "N", "G2"]
            .iter()
            .enumerate()
            .map(|(r, name)| {
                format!(
                    "{name}:{} (wait={:#x} fault_sig={} addr={:#x} pc={:#x} t={}us)",
                    self.run_exit(r).describe(),
                    self.run_status[r],
                    self.fault_sig[r],
                    self.fault_addr[r],
                    self.fault_pc[r],
                    self.t_us[r],
                )
            })
            .collect::<Vec<_>>()
            .join(", ")
    }
}
