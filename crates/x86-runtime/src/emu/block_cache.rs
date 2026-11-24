//! Per-thread decoded basic-block cache ([`BlockCache`]), guest stack management
//! ([`ThreadEmuState`]), thread-local emulator state ([`TLS_EMU`], [`CUR_CPU`]), and shadow/EH
//! stack-state helpers.

#[cfg(target_arch = "aarch64")]
use super::aot;
use super::cpu::Cpu;
use super::routes::{resolve_host_callable, to_mapped_addr};
use super::{exec_mem, fast, jit, mem};
use crate::exe::wr;
use iced_x86::{Decoder, DecoderOptions, FlowControl, Instruction};

const GUEST_STACK_QWORDS: usize = 1024 * 1024; // 8 MB per thread

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
pub(super) const BLK_MAX_INSNS: usize = 32;

#[derive(Clone, Copy, Default)]
#[repr(C, align(32))]
pub(super) struct BlkEntry {
    pub(super) ip: u64,
    pub(super) off: u32,
    pub(super) len: u32,
    /// 0 for blocks outside run-time generated code (never revalidated); otherwise the
    /// [`mem::code_gen`] the block was decoded or last revalidated in.
    pub(super) gen: u32,
    /// Non-zero epoch matching [`BlockCache::fast_gen`] when this block is provably safe to
    /// chain directly inside `interp_burst` without re-checking host/JIT/route tables.
    pub(super) fast_gen: u32,
    /// Direct block-chaining cache: `index` slot + 1 for the last taken branch/call/ret target
    /// (`0` = unset). Always validated against `(ip, fast_gen)` before use.
    pub(super) taken_slot: u32,
    /// Direct block-chaining cache: `index` slot + 1 for the fallthrough target (`0` = unset).
    /// Always validated against `(ip, fast_gen)` before use.
    pub(super) fall_slot: u32,
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
pub(super) struct BlockCache {
    pub(super) arena: Box<[Instruction]>,
    pub(super) ops: Box<[fast::FastOp]>,
    pub(super) index: Box<[BlkEntry]>,
    cursor: usize,
    rcache: Box<[RcEntry]>,
    /// Current fast-chain epoch (never 0 once synchronized); bumped whenever `fast_key` changes.
    pub(super) fast_gen: u32,
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
pub(super) fn thunk_elided(va: u64, h: usize) -> bool {
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
pub(super) unsafe fn zeroed_slice<T>(n: usize) -> Box<[T]> {
    let layout = std::alloc::Layout::array::<T>(n).expect("zeroed_slice layout");
    let p = std::alloc::alloc_zeroed(layout) as *mut T;
    if p.is_null() {
        std::alloc::handle_alloc_error(layout);
    }
    Box::from_raw(core::ptr::slice_from_raw_parts_mut(p, n))
}

pub(super) enum BlkErr {
    BadRip(usize),
    Invalid([u8; 8]),
}

impl BlockCache {
    pub(super) fn new() -> Self {
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
    pub(super) fn sync_fast_gen(&mut self, mode_flags: u32) -> u32 {
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
    pub(super) unsafe fn resolve(this: *mut BlockCache, target: u64) -> Option<usize> {
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
    pub(super) fn slot(ip: u64) -> usize {
        ((ip as usize).wrapping_mul(0x9E37_79B9_7F4A_7C15) >> (64 - BLK_INDEX_BITS)) & (BLK_INDEX_LEN - 1)
    }

    /// Return the block starting at `ip` as `(arena offset, length >= 1)`.
    #[inline(always)]
    pub(super) unsafe fn get(this: *mut BlockCache, ip: u64, misses: &mut u64) -> Result<(usize, usize), BlkErr> {
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
pub(super) fn fast_enabled() -> bool {
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
    pub(super) static TEST_FAST_OFF: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
}

/// `AW2_EMU_VERIFY=1` cross-checks every eligible fast op against the generic handler.
pub(super) fn verify_enabled() -> bool {
    static V: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *V.get_or_init(|| std::env::var("AW2_EMU_VERIFY").is_ok_and(|s| s.trim() == "1"))
}

std::thread_local! {
    static FROZEN_TSC: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
}

/// Compare guest executions with a repeatable timestamp on this thread, including lifted calls.
/// Nested comparisons restore their caller's clock mode, also on unwind.
pub fn with_frozen_tsc<R>(run: impl FnOnce() -> R) -> R {
    struct Restore(bool);
    impl Drop for Restore {
        fn drop(&mut self) {
            FROZEN_TSC.set(self.0);
        }
    }
    let _restore = Restore(FROZEN_TSC.replace(true));
    run()
}

pub(crate) fn tsc_is_frozen() -> bool {
    FROZEN_TSC.get() || win32_rt::shadow::in_child() || verify_enabled()
}

#[test]
fn frozen_tsc_scope_is_thread_local_and_restored_on_unwind() {
    assert!(!FROZEN_TSC.get());
    with_frozen_tsc(|| {
        assert!(std::panic::catch_unwind(|| with_frozen_tsc(|| panic!("end comparison"))).is_err());
        assert!(FROZEN_TSC.get());
        assert!(!std::thread::spawn(|| FROZEN_TSC.get()).join().unwrap());
    });
    assert!(!FROZEN_TSC.get());
}

pub(super) struct ThreadEmuState {
    pub(super) _stack: Box<[u64]>,
    pub(super) stack_min: u64,
    pub(super) stack_top: u64,
    pub(super) sp: u64,
    pub(super) blocks: BlockCache,
    pub(super) jit: jit::JitCache,
}

/// Bytes kept free above `stack_min` (the guest stack's floor for the lifted stack account):
/// a lifted budget never reaches below `stack_min + GUEST_FLOOR_MARGIN`, and a nested region
/// whose start (charged with the live lifted depth) would lie below it raises
/// `STATUS_STACK_OVERFLOW` instead (`crate::lstack` "Stack account").
pub(crate) const GUEST_FLOOR_MARGIN: u64 = 64 << 10;
/// `stack_min` offset from the guest stack base.
pub(super) const GUEST_STACK_MIN_OFF: u64 = 256 * 1024 * 8;
/// Lower bound of `stack_top - floor` on any thread (the lifted budget of a thread with no
/// emulator state yet).
pub(crate) const GUEST_ROOM_MIN: u64 =
    (GUEST_STACK_QWORDS as u64) * 8 - 256 - 16 - GUEST_STACK_MIN_OFF - GUEST_FLOOR_MARGIN;

impl ThreadEmuState {
    pub(super) fn new() -> Self {
        let stack = vec![0u64; GUEST_STACK_QWORDS].into_boxed_slice();
        let base_addr = stack.as_ptr() as u64;
        let stack_top = (base_addr + (GUEST_STACK_QWORDS as u64) * 8 - 256) & !0xFu64;
        let stack_min = base_addr + GUEST_STACK_MIN_OFF;
        crate::lstack::note_guest_stack(crate::callctx::ctx(), stack_min + GUEST_FLOOR_MARGIN, stack_top);
        ThreadEmuState {
            _stack: stack,
            stack_min,
            stack_top,
            sp: stack_top,
            blocks: BlockCache::new(),
            jit: jit::JitCache::new(),
        }
    }
}

std::thread_local! {
    pub(super) static TLS_EMU: std::cell::UnsafeCell<Option<ThreadEmuState>> = const { std::cell::UnsafeCell::new(None) };
}

pub(super) struct SpGuard {
    pub(super) sp_ptr: *mut u64,
    pub(super) saved_sp: u64,
    pub(super) saved_teb_stack: Option<(usize, u64, u64, u64)>,
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
    pub(super) static CUR_CPU: core::cell::Cell<*const Cpu> = const { core::cell::Cell::new(core::ptr::null()) };
}

/// Restores the outer frame's [`CUR_CPU`] on every exit path.
pub(super) struct CurCpuGuard(pub(super) *const Cpu);
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
pub(crate) unsafe fn shadow_prepare(
    jit: bool,
    hard: &mut Vec<(usize, usize)>,
    soft: &mut Vec<(usize, usize)>,
) -> (usize, usize) {
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
