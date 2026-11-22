//! Helpers for differential tests (original vs Rust on identical inputs).
//!
//! Each decompiled module provides `pub unsafe fn diff_<name>(orig, new)`
//! (a [`crate::registry::DiffFn`]): it transmutes both addresses to the
//! function's `extern "win64"` type, runs a deterministic set of edge cases
//! plus pseudo-random inputs through both, and compares return values and
//! every byte of memory the function may write.

/// SplitMix64 — deterministic input generator (fixed seeds, reproducible).
pub struct Rng(pub u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng(seed)
    }
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    pub fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }
    /// Uniform in `0..n` (n > 0; slight bias is irrelevant here).
    pub fn below(&mut self, n: u64) -> u64 {
        self.next_u64() % n
    }
    pub fn bytes(&mut self, len: usize) -> Vec<u8> {
        (0..len).map(|_| self.next_u64() as u8).collect()
    }
    /// Random float covering normal values, zero, negatives, and large/small magnitudes.
    pub fn f32_any(&mut self) -> f32 {
        match self.below(8) {
            0 => 0.0,
            1 => -0.0,
            2 => (self.next_u32() as f32 / u32::MAX as f32) * 2.0 - 1.0,
            3 => (self.next_u32() as i32) as f32 / 1.0e3,
            _ => (self.next_u32() as f32 / u32::MAX as f32) * 200.0 - 100.0,
        }
    }
}

pub use crate::diff_eq;

/// Bitwise float comparison (treats NaNs with equal bits as equal, -0 != +0).
pub fn f32_bits_eq(a: f32, b: f32) -> bool {
    a.to_bits() == b.to_bits()
}

#[inline]
pub fn eq<T: PartialEq + core::fmt::Debug>(ctx: &str, orig: T, new: T) -> Result<(), String> {
    if orig != new {
        return Err(format!("mismatch: orig={orig:?} new={new:?} for {ctx}"));
    }
    Ok(())
}

/// Prefix of a difftest error meaning "cannot run in this environment"
/// (the runner logs it and does not count it as a failure).
pub const SKIP: &str = "skip:";

// ---------------------------------------------------------------------------
// Mocks: redirect exe callees for the duration of a test
// ---------------------------------------------------------------------------

/// Installed by the loader: patch `va` to jump to `target`, returning a token.
pub type PatchFn = fn(va: u64, target: usize) -> Result<usize, String>;
/// Undo a [`PatchFn`] patch.
pub type UnpatchFn = fn(token: usize);

static PATCHER: std::sync::OnceLock<(PatchFn, UnpatchFn)> = std::sync::OnceLock::new();

/// Called by the loader once the exe is mapped.
pub fn set_patcher(p: PatchFn, u: UnpatchFn) {
    let _ = PATCHER.set((p, u));
}

/// While alive, calls to the exe function at `va` (from the original *and*
/// from Rust code calling it through [`crate::exe::func`]) go to `target`.
/// Use it to give both sides the same deterministic callee (fake allocator,
/// recorder) so callers can be compared without the engine being initialized.
pub struct Mock(usize);

impl Mock {
    pub fn new(va: u64, target: *const ()) -> Result<Mock, String> {
        let (p, _) = PATCHER.get().ok_or_else(|| format!("{SKIP} no patcher (mocks unavailable)"))?;
        p(va, target as usize).map(Mock)
    }
}

impl Drop for Mock {
    fn drop(&mut self) {
        if let Some((_, u)) = PATCHER.get() {
            u(self.0);
        }
    }
}

/// IAT slot of `KERNEL32!VirtualProtect` (used to unprotect other slots).
const VIRTUAL_PROTECT_IAT: u64 = 0x1_4264_5338;
const PAGE_READWRITE: u32 = 0x04;

/// While alive, the exe's import slot at `iat_va` points to `target`, so the
/// original's `call [__imp_X]` (and Rust code calling through the slot) goes
/// to `target`. Use it for imports a test must not really call
/// (`TerminateProcess`) or whose calls it records. The IAT is in read-only
/// `.rdata`; each write unprotects the slot with the exe's own `VirtualProtect`
/// import and restores the previous protection.
pub struct IatMock {
    slot: usize,
    old: usize,
}

impl IatMock {
    pub unsafe fn new(iat_va: u64, target: *const ()) -> Result<IatMock, String> {
        let slot = crate::exe::ptr::<u8>(iat_va) as usize;
        let old = std::ptr::read_unaligned(slot as *const usize);
        if old == 0 {
            return Err(format!("{SKIP} import slot 0x{iat_va:x} is not bound"));
        }
        write_iat_slot(slot, target as usize)?;
        Ok(IatMock { slot, old })
    }

    /// The import's real target (to forward calls from other threads).
    pub fn original(&self) -> usize {
        self.old
    }
}

unsafe fn write_iat_slot(slot: usize, value: usize) -> Result<(), String> {
    let vp = std::ptr::read_unaligned(crate::exe::ptr::<usize>(VIRTUAL_PROTECT_IAT));
    if vp == 0 {
        return Err(format!("{SKIP} VirtualProtect is not bound"));
    }
    let mut prot = 0u32;
    if crate::exe::call_va!(vp as u64, (slot, 8usize, PAGE_READWRITE, &mut prot as *mut u32) -> i32) == 0 {
        return Err(format!("VirtualProtect(0x{slot:x}, RW) failed"));
    }
    std::ptr::write_unaligned(slot as *mut usize, value);
    let mut tmp = 0u32;
    let _ = crate::exe::call_va!(vp as u64, (slot, 8usize, prot, &mut tmp as *mut u32) -> i32);
    Ok(())
}

impl Drop for IatMock {
    fn drop(&mut self) {
        unsafe {
            let _ = write_iat_slot(self.slot, self.old);
        }
    }
}

/// Copy of a byte range (e.g. exe globals) that can be put back, so the
/// original and the replacement start from the same state.
pub struct Snapshot {
    at: *mut u8,
    bytes: Vec<u8>,
}

impl Snapshot {
    pub unsafe fn take(at: *mut u8, len: usize) -> Snapshot {
        Snapshot { at, bytes: std::slice::from_raw_parts(at, len).to_vec() }
    }
    pub unsafe fn restore(&self) {
        std::ptr::copy_nonoverlapping(self.bytes.as_ptr(), self.at, self.bytes.len());
    }
    pub unsafe fn current(&self) -> Vec<u8> {
        std::slice::from_raw_parts(self.at, self.bytes.len()).to_vec()
    }
}

fn gpr_parent_idx(r: iced_x86::Register) -> Option<usize> {
    use iced_x86::Register::*;
    match r {
        RAX | EAX | AX | AL | AH => Some(0),
        RCX | ECX | CX | CL | CH => Some(1),
        RDX | EDX | DX | DL | DH => Some(2),
        RBX | EBX | BX | BL | BH => Some(3),
        RSP | ESP | SP | SPL => Some(4),
        RBP | EBP | BP | BPL => Some(5),
        RSI | ESI | SI | SIL => Some(6),
        RDI | EDI | DI | DIL => Some(7),
        R8 | R8D | R8W | R8L => Some(8),
        R9 | R9D | R9W | R9L => Some(9),
        R10 | R10D | R10W | R10L => Some(10),
        R11 | R11D | R11W | R11L => Some(11),
        R12 | R12D | R12W | R12L => Some(12),
        R13 | R13D | R13W | R13L => Some(13),
        R14 | R14D | R14W | R14L => Some(14),
        R15 | R15D | R15W | R15L => Some(15),
        _ => Option::None,
    }
}

/// Automated differential test for single-indirection leaf functions in `shards/`.
pub unsafe fn shard_leaf_difftest(va: u64, orig: usize, new_fn: usize) -> Result<u32, String> {
    use iced_x86::{Decoder, DecoderOptions, FlowControl, Mnemonic, OpKind, Register};
    let code_addr = orig & !(1usize << 46);
    let bytes = std::slice::from_raw_parts(code_addr as *const u8, 256);
    let mut dec = Decoder::with_ip(64, bytes, va, DecoderOptions::NONE);
    let mut ptr_valid = [false; 16];
    ptr_valid[1] = true; // rcx
    ptr_valid[2] = true; // rdx
    ptr_valid[8] = true; // r8
    ptr_valid[9] = true; // r9

    let mut reached_ret = false;
    let mut writes_rax = false;
    let mut last_rax_bits = 64u32;
    for _ in 0..24 {
        if !dec.can_decode() {
            break;
        }
        let ins = dec.decode();
        if ins.mnemonic() == Mnemonic::Ret {
            reached_ret = true;
            break;
        }
        if ins.flow_control() != FlowControl::Next
            || ins.segment_prefix() == Register::GS
            || ins.is_ip_rel_memory_operand()
            || ins.has_rep_prefix()
            || ins.has_repne_prefix()
            || ins.has_lock_prefix()
            || matches!(
                ins.mnemonic(),
                Mnemonic::Push
                    | Mnemonic::Pop
                    | Mnemonic::Div
                    | Mnemonic::Idiv
                    | Mnemonic::Ud2
                    | Mnemonic::Int3
                    | Mnemonic::Rdtsc
                    | Mnemonic::Rdtscp
                    | Mnemonic::Cpuid
                    | Mnemonic::Xgetbv
                    | Mnemonic::Sidt
                    | Mnemonic::Sgdt
                    | Mnemonic::Sldt
                    | Mnemonic::Str
                    | Mnemonic::Smsw
                    | Mnemonic::Movsb
                    | Mnemonic::Movsw
                    | Mnemonic::Movsq
                    | Mnemonic::Stosb
                    | Mnemonic::Stosw
                    | Mnemonic::Stosd
                    | Mnemonic::Stosq
                    | Mnemonic::Scasb
                    | Mnemonic::Scasw
                    | Mnemonic::Scasd
                    | Mnemonic::Scasq
            )
        {
            return Err(format!("{SKIP} non-linear or stack/global leaf"));
        }
        if matches!(
            ins.mnemonic(),
            Mnemonic::Mul
                | Mnemonic::Cdq
                | Mnemonic::Cqo
                | Mnemonic::Cdqe
                | Mnemonic::Cwde
                | Mnemonic::Cbw
                | Mnemonic::Cmpxchg
        ) {
            writes_rax = true;
            last_rax_bits = 64;
            ptr_valid[0] = false;
            ptr_valid[2] = false;
        }
        for op_i in 0..ins.op_count() {
            match ins.op_kind(op_i) {
                OpKind::Memory => {
                    let b_idx = gpr_parent_idx(ins.memory_base()).ok_or_else(|| format!("{SKIP} mem base"))?;
                    if !ptr_valid[b_idx] || ins.memory_index() != Register::None {
                        return Err(format!("{SKIP} indirect/indexed mem"));
                    }
                    let disp = ins.memory_displacement64() as i64;
                    if !(-256..=1024).contains(&disp) {
                        return Err(format!("{SKIP} mem disp out of range"));
                    }
                }
                OpKind::Register
                | OpKind::Immediate8
                | OpKind::Immediate16
                | OpKind::Immediate32
                | OpKind::Immediate64
                | OpKind::Immediate8to16
                | OpKind::Immediate8to32
                | OpKind::Immediate8to64
                | OpKind::Immediate32to64 => {}
                _ => return Err(format!("{SKIP} special op kind")),
            }
        }
        if ins.op_count() >= 1 && ins.op_kind(0) == OpKind::Register {
            let reg0 = ins.op_register(0);
            if let Some(d_idx) = gpr_parent_idx(reg0) {
                if d_idx == 4 {
                    return Err(format!("{SKIP} modifies rsp"));
                }
                if d_idx == 0 {
                    writes_rax = true;
                    last_rax_bits = match reg0 {
                        Register::AL => 8,
                        Register::AX => 16,
                        _ => 64,
                    };
                }
                ptr_valid[d_idx] = false;
            }
        }
        if matches!(ins.mnemonic(), Mnemonic::Xchg | Mnemonic::Xadd)
            && ins.op_count() >= 2
            && ins.op_kind(1) == OpKind::Register
        {
            if let Some(d_idx) = gpr_parent_idx(ins.op_register(1)) {
                if d_idx == 0 {
                    writes_rax = true;
                    last_rax_bits = 64;
                }
                ptr_valid[d_idx] = false;
            }
        }
    }
    if !reached_ret {
        return Err(format!("{SKIP} no linear ret within 24 insns"));
    }

    let mut rng = Rng::new(va ^ 0xA5A5_5A5A_F0F0_0F0F);
    let mut work_buf = [[0u8; 2048]; 4];
    for case_idx in 0..2u32 {
        let mut init_buf = [[0u8; 2048]; 4];
        for b in &mut init_buf {
            for slot in b.chunks_exact_mut(4) {
                let v = if case_idx == 0 {
                    1u32
                } else {
                    (rng.next_u32() & 0x7f) + 1
                };
                slot.copy_from_slice(&v.to_le_bytes());
            }
        }
        let args = [
            work_buf[0].as_mut_ptr().add(512) as u64,
            work_buf[1].as_mut_ptr().add(512) as u64,
            work_buf[2].as_mut_ptr().add(512) as u64,
            work_buf[3].as_mut_ptr().add(512) as u64,
            0, 0, 0, 0,
        ];
        let xmm_in = [1.5f32.to_bits() as u128, 2.25f32.to_bits() as u128, 0, 0];
        work_buf = init_buf;
        let o_ret = {
            #[cfg(target_arch = "x86_64")]
            {
                let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =
                    core::mem::transmute(orig);
                f(args[0], args[1], args[2], args[3], 0, 0, 0, 0)
            }
            #[cfg(not(target_arch = "x86_64"))]
            {
                crate::emu::emu_run_with_xmm(code_addr as u64, args, xmm_in).0
            }
        };
        let orig_after = work_buf;
        work_buf = init_buf;
        let n_ret = {
            crate::simd_ops::set_tls_xmm_in(xmm_in);
            let (r, _) = crate::emu::call_host_with_xmm(new_fn, args, xmm_in, [0; 8]);
            r
        };
        for i in 0..4 {
            if orig_after[i] != work_buf[i] {
                return Err(format!("case {case_idx} buffer {i} mismatch"));
            }
        }
        let mask = match last_rax_bits {
            8 => 0xffu64,
            16 => 0xffffu64,
            _ => u64::MAX,
        };
        if writes_rax && (o_ret & mask) != (n_ret & mask) {
            return Err(format!(
                "case {case_idx} ret mismatch: orig=0x{:x} new=0x{:x}",
                o_ret & mask,
                n_ret & mask
            ));
        }
    }
    Ok(2)
}

