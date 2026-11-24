//! AOT translation cache, translator side (`AW2_DLL_AOT`; directory, file format and
//! loading live in [`crate::emu::aot`]).
//!
//! An AOT block is the JIT's translation of the same block (`prep_block` + `Tr`, identical
//! analysis) emitted in AOT mode:
//! - into a scratch buffer, as if placed at its final address in an AOT region whose shared
//!   stubs sit at offset 0 (`Asm::vbase`; stub branches are PC-relative, so the code is
//!   valid wherever the region lands);
//! - host function pointers (`plain_call`) from private literal slots, recorded as
//!   relocations against the guest VA they were resolved for;
//! - direct-link sites recorded with their out-of-line fallback (built unlinked);
//! - every host-callable decision (`resolved_host`) recorded as an assumption.

use super::cache::{
    new_tr, prep_block_with, StubAddrs, TrScratch, MAX_BLOCK_BYTES, RESOLVE_REC, STUBS_BYTES,
};
use super::emit::Asm;
use super::lazy::flaglazy_enabled;
use super::stubs::{emit_stubs, emit_stubs_cfg, hostcall_enabled, plaincall_enabled, StubCfg};
use super::translate::stack_tso;
use super::xmm::{fast_nan_enabled, retlit_enabled, xmm_pin_enabled, xmmreg_enabled};
use super::*;
use crate::emu::{mem, to_mapped_addr, BlockCache, BLK_MAX_INSNS};

/// Word offsets of the shared stubs at the start of an AOT region: enter, exit,
/// lookup_rip, lookup_jmp, lookup_call, host_xfer, ras_miss, spill_xmm, reload_xmm,
/// clear_ymm_hi, rdtsc_slow, rdtscp_slow (`u32::MAX` = not emitted).
pub(in crate::emu) type AotStubOffs = [u32; 12];

/// Bytes reserved for the stubs at the start of an AOT region.
pub(in crate::emu) const AOT_STUBS_BYTES: usize = STUBS_BYTES;

/// Upper bound on one block's code (scratch buffer size).
pub(in crate::emu) const AOT_MAX_BLOCK_BYTES: usize = MAX_BLOCK_BYTES;

/// [`AotReloc::kind`]: the slot holds `inline_host_fn(va).0` (whose `.1` must equal `bail`).
pub(in crate::emu) const RELOC_HOST_FN: u8 = 0;
/// [`AotReloc::kind`]: the slot holds `va` itself (an import slot's value, `src` != 0).
pub(in crate::emu) const RELOC_SLOT_VAL: u8 = 1;
/// [`AotReloc::kind`]: the slot holds `win32_rt::kernel32::time::qpc_fast_state_ptr()`.
pub(in crate::emu) const RELOC_QPC_STATE: u8 = 2;

/// A relocated 8-byte literal-pool slot of an AOT block.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(in crate::emu) struct AotReloc {
    /// Pool slot word (block-relative).
    pub word: u32,
    /// [`RELOC_HOST_FN`] / [`RELOC_SLOT_VAL`].
    pub kind: u8,
    pub bail: bool,
    pub va: u64,
    /// Guest import slot `va` was read from (`const_slot_plain`: the code compares the slot
    /// with `va` at run time), 0 = none. At load, `va` follows the slot's current value.
    pub src: u64,
}

/// One block translated in AOT emission mode (word positions are block-relative).
pub(in crate::emu) struct AotBlock {
    pub words: Vec<u32>,
    /// Words on the main path.
    pub main: u32,
    /// Word offset where the literal pool starts (`<= words.len()`; words in
    /// `[0, cold_code_end)` are executable ARM64 instructions, `[cold_code_end, words.len())`
    /// are raw literal pool data words).
    pub cold_code_end: u32,
    /// Guest instructions translated.
    pub guest: u32,
    /// The block's IP is host-callable (`JEnt::fast` = 0: never a direct-link target).
    pub host_entry: bool,
    /// `(site word, fallback word, optional cond branch word or u32::MAX, guest target)`.
    pub links: Vec<(u32, u32, u32, u64)>,
    pub relocs: Vec<AotReloc>,
    /// `(va, host-callable)` decisions the translation depends on.
    pub assume: Vec<(u64, bool)>,
}

/// Shared core for static Mach-O stub emission: emits position-independent stubs into `buf`
/// (`AOT_STUBS_BYTES`) matching `want_stubs` and returns `(offs, host_xfer_bl_byte_off, used_words)`.
unsafe fn emit_static_stubs_inner(
    buf: *mut u32,
    vbase: usize,
    knobs: u64,
    want_stubs: &AotStubOffs,
) -> Result<(AotStubOffs, Option<usize>, usize), String> {
    let want_ras = (want_stubs[6] != u32::MAX).then_some(want_stubs[6] as usize);
    // Try standard production stub flags first, then test/stats counter variants if the
    // `.aot` file was produced by a test or `AW2_JIT_STATS=1` build.
    let candidates: [(bool, bool); 3] = [(false, false), (true, false), (true, true)];
    let mut last_got = AotStubOffs::default();
    for &(count_probe, count_plain) in &candidates {
        if count_plain && !cfg!(test) {
            continue;
        }
        core::ptr::write_bytes(buf, 0, STUBS_BYTES / 4);
        let mut a = Asm::new(buf, vbase, STUBS_BYTES / 4);
        let cfg = StubCfg::from_aot_knobs(knobs, want_ras, count_probe, count_plain);
        let (st, bl_off) = emit_stubs_cfg(&mut a, cfg);
        if a.overflow {
            continue;
        }
        let got = st.offsets();
        if got == *want_stubs {
            return Ok((got, bl_off, a.pos));
        }
        last_got = got;
    }
    Err(format!(
        "static stub layout mismatch (knobs={knobs:#x}): expected {want_stubs:?}, emitted {last_got:?}"
    ))
}

/// Emit position-independent static Mach-O stubs into `buf` (`AOT_STUBS_BYTES`), matching
/// the exact [`AotStubOffs`] layout recorded in a `.aot` file (`knobs` + `want_stubs`).
/// Returns `(emitted_offs, host_xfer_bl_byte_off)` where `host_xfer_bl_byte_off` is the
/// byte offset of the `bl 0` instruction that must receive an `ARM64_RELOC_BRANCH26`
/// relocation pointing to `_port_static_aot_host_xfer` (single-bank direct relocation mode).
#[cfg(test)]
pub(in crate::emu) unsafe fn emit_static_stubs(
    buf: *mut u32,
    vbase: usize,
    knobs: u64,
    want_stubs: &AotStubOffs,
) -> Result<(AotStubOffs, Option<usize>), String> {
    let (got, bl_off, _used) = emit_static_stubs_inner(buf, vbase, knobs, want_stubs)?;
    Ok((got, bl_off))
}

/// Emit position-independent static Mach-O stubs into `buf` (`AOT_STUBS_BYTES`) for a
/// multi-bank `.aot.o` image, routing `host_xfer`'s `bl` locally to a 16-byte ABI-safe
/// `x16` (`XA`) GOT veneer placed in the free tail of the same `AOT_STUBS_BYTES` bank:
///
/// ```text
///   adrp x16, _port_static_aot_host_xfer@GOTPAGE      ; ARM64_RELOC_GOT_LOAD_PAGE21
///   ldr  x16, [x16, _port_static_aot_host_xfer@GOTPAGEOFF] ; ARM64_RELOC_GOT_LOAD_PAGEOFF12
///   br   x16
///   nop
/// ```
///
/// Because every bank's `host_xfer` call resolves via a ±4 GiB GOT load rather than a
/// ±128 MiB `ARM64_RELOC_BRANCH26`, `ld64` links 300–500+ MiB multi-bank `.aot.o` images
/// cleanly with zero branch-range relocation errors. Returns `(emitted_offs, veneer_byte_off)`.
pub(in crate::emu) unsafe fn emit_static_stubs_with_veneer(
    buf: *mut u32,
    vbase: usize,
    knobs: u64,
    want_stubs: &AotStubOffs,
) -> Result<(AotStubOffs, Option<usize>), String> {
    const ARM64_ADRP_X16: u32 = 0x9000_0010;
    const ARM64_LDR_X16_X16_IMM0: u32 = 0xF940_0210;
    const ARM64_BR_X16: u32 = 0xD61F_0200;
    const ARM64_NOP: u32 = 0xD503_201F;

    let (got, bl_off, used_words) = emit_static_stubs_inner(buf, vbase, knobs, want_stubs)?;
    let Some(bl_byte_off) = bl_off else {
        return Ok((got, None));
    };
    let veneer_w = (used_words + 3) & !3;
    if veneer_w + 4 > STUBS_BYTES / 4 {
        return Err(format!(
            "static stub bank exhausted ({used_words} words used of {}); no room for host_xfer GOT veneer",
            STUBS_BYTES / 4
        ));
    }
    *buf.add(veneer_w) = ARM64_ADRP_X16;
    *buf.add(veneer_w + 1) = ARM64_LDR_X16_X16_IMM0;
    *buf.add(veneer_w + 2) = ARM64_BR_X16;
    *buf.add(veneer_w + 3) = ARM64_NOP;

    let bl_w = bl_byte_off / 4;
    let delta_w = veneer_w as i64 - bl_w as i64;
    debug_assert!((-(1i64 << 25)..(1i64 << 25)).contains(&delta_w));
    *buf.add(bl_w) = 0x9400_0000 | ((delta_w as u32) & 0x03FF_FFFF);
    Ok((got, Some(veneer_w * 4)))
}

/// Test-only: emit live (`static_pic = false`) and static GOT-veneer (`static_pic = true`) stubs
/// for an arbitrary `knobs` vector and return `(live_words, veneer_end_bytes)`.
#[cfg(test)]
pub(in crate::emu) unsafe fn test_emit_stubs_and_veneer_for_knobs(
    knobs: u64,
    count_probe: bool,
    count_plain: bool,
) -> Result<(usize, usize), String> {
    const VBASE: usize = 0x2000_0000_0000;
    let mut live_buf = vec![0u32; STUBS_BYTES / 4];
    let mut a = Asm::new(live_buf.as_mut_ptr(), VBASE, STUBS_BYTES / 4);
    let mut live_cfg = StubCfg::from_aot_knobs(knobs, None, count_probe, count_plain);
    live_cfg.static_pic = false;
    let (st, _) = emit_stubs_cfg(&mut a, live_cfg);
    if a.overflow {
        return Err(format!("live stubs overflowed {STUBS_BYTES} bytes for knobs={knobs:#x}"));
    }
    let live_words = a.pos;
    let want = st.offsets();
    let mut veneer_buf = vec![0u32; STUBS_BYTES / 4];
    let (got, veneer_off) = emit_static_stubs_with_veneer(veneer_buf.as_mut_ptr(), VBASE, knobs, &want)?;
    if got != want {
        return Err(format!("static veneer stubs mismatch: got {got:?}, want {want:?}"));
    }
    let end_bytes = match veneer_off {
        Some(vo) => vo + 16,
        None => live_words * 4,
    };
    Ok((live_words, end_bytes))
}

/// Emit the shared stubs into `buf` (writable, [`AOT_STUBS_BYTES`]), executing at `vbase`.
pub(in crate::emu) unsafe fn aot_emit_stubs(buf: *mut u32, vbase: usize) -> Option<AotStubOffs> {
    let mut a = Asm::new(buf, vbase, STUBS_BYTES / 4);
    let st = emit_stubs(&mut a, ras_enabled());
    if a.overflow {
        return None;
    }
    Some(st.offsets())
}

/// Stub addresses of an AOT region at `vbase` (same convention as [`JitCache`]: 0 = none).
fn aot_stub_addrs(vbase: usize, o: &AotStubOffs) -> StubAddrs {
    let at = |w: u32| if w == u32::MAX { 0 } else { vbase + w as usize * 4 };
    StubAddrs {
        exit: at(o[1]),
        lookup_rip: at(o[2]),
        lookup_jmp: at(o[3]),
        lookup_call: at(o[4]),
        host_xfer: at(o[5]),
        ras_miss: at(o[6]),
        spill_xmm: at(o[7]),
        reload_xmm: at(o[8]),
        clear_ymm_hi: at(o[9]),
        rdtsc_slow: at(o[10]),
        rdtscp_slow: at(o[11]),
    }
}

/// Translate the guest block at `ip` in AOT emission mode, as if placed at `vaddr` in an
/// AOT region at `vbase` with stubs `offs`; `scratch` and `tr_scratch` are reused between calls.
/// None = not translatable (the JIT handles it at run time exactly as today).
pub(in crate::emu) unsafe fn aot_translate(
    bc: *mut BlockCache,
    ip: u64,
    vaddr: usize,
    vbase: usize,
    offs: &AotStubOffs,
    scratch: &mut Vec<u32>,
    tr_scratch: &mut TrScratch,
) -> Option<AotBlock> {
    if mem::in_jit(to_mapped_addr(ip)) {
        return None;
    }
    RESOLVE_REC.with(|r| *r.borrow_mut() = Some(Vec::new()));
    let r = aot_translate_rec(bc, ip, vaddr, vbase, offs, scratch, tr_scratch);
    let assume = RESOLVE_REC.with(|r| r.borrow_mut().take()).unwrap_or_default();
    r.map(|mut b| {
        b.assume = assume;
        b
    })
}

unsafe fn aot_translate_rec(
    bc: *mut BlockCache,
    ip: u64,
    vaddr: usize,
    vbase: usize,
    offs: &AotStubOffs,
    scratch: &mut Vec<u32>,
    tr_scratch: &mut TrScratch,
) -> Option<AotBlock> {
    let p = prep_block_with(bc, ip, &mut tr_scratch.info_factory)?;
    let cap = MAX_BLOCK_BYTES / 4;
    scratch.clear();
    scratch.resize(cap, 0);
    let st = aot_stub_addrs(vbase, offs);
    let ops = &p.ops[..p.k];
    let mut tr = new_tr(Asm::new(scratch.as_mut_ptr(), vaddr, cap), ops, &p, &st, true, tr_scratch);
    let main = tr.translate(p.term, p.live_out);
    let cold_code_end = tr.cold_code_end as u32;
    let n = tr.a.pos;
    let overflow = tr.a.overflow;
    if overflow {
        tr.return_scratch(tr_scratch);
        return None;
    }
    let words = scratch[..n].to_vec();
    // Every link site was bound to its out-of-line fallback (`Tr::translate`): recover it
    // from the site's imm26, and attach any hot conditional branch pointing at `site`.
    let links = tr
        .links
        .iter()
        .map(|&(site, target)| {
            let off = (((words[site] << 6) as i32) >> 6) as i64;
            let cond = tr
                .link_conds
                .iter()
                .find(|&&(s, _)| s == site)
                .map_or(u32::MAX, |&(_, c)| c as u32);
            (site as u32, (site as i64 + off) as u32, cond, target)
        })
        .collect();
    let relocs = if tr.relocs.is_empty() { Vec::new() } else { tr.relocs.clone() };
    tr.return_scratch(tr_scratch);
    Some(AotBlock {
        words,
        main: main as u32,
        cold_code_end,
        guest: p.k as u32,
        host_entry: p.host,
        links,
        relocs,
        assume: Vec::new(),
    })
}

/// Codegen knob vector for the AOT key: every input of the translation / `emit_stubs`
/// that is not guest memory or host-callable state (those are hashed / assumed).
pub(in crate::emu) fn aot_knob_bits() -> u64 {
    let bits: [(u32, u64); 26] = [
        (0, ras_enabled() as u64),
        (1, flags_dead_mask() as u64), // 2 bits
        (3, ro_plain_enabled() as u64),
        (4, memchk_mask() as u64), // 4 bits
        (8, pushpop_enabled() as u64),
        (9, budget_backedge_enabled() as u64),
        (10, xmmreg_enabled() as u64),
        (11, retlit_enabled() as u64),
        (12, flaglazy_enabled() as u64),
        (13, hostcall_enabled() as u64),
        (14, plaincall_enabled() as u64),
        (15, mem::tso() as u64),
        (16, stack_tso() as u64),
        (17, crate::exe::aw2_registry_active() as u64),
        (18, crate::emu::fast_enabled() as u64),
        (19, (BLK_MAX_INSNS as u64) & 0xFF), // 8 bits
        (27, aot_probe_enabled() as u64),
        (28, 3u64), // codegen rev 3 (2 bits: 28..29)
        (30, xmm_pin_enabled() as u64),
        (31, fast_nan_enabled() as u64),
        (32, last_bits_enabled() as u64),
        (33, lse2_trap_enabled() as u64),
        (34, budget_light_enabled() as u64),
        (35, hw_range_enabled() as u64),
        (36, 1u64), // codegen rev 4: XBUD bit-0 ymm_dirty, batched push/pop/ret, JccPair, x_shuf_rr, xmm_hi_live, XA cache, hot/cold split
        (37, 3u64), // codegen rev 7 (bits 37..38): shared timestamp conversion stubs
    ];
    let low = bits.iter().fold(0u64, |k, &(b, v)| k | (v << b));
    debug_assert_eq!(low >> 39, 0, "aot_knob_bits low bitfields overflowed bit 38");
    // Fold a 25-bit digest of all registry-declared AOT-key knobs (`affects_aot_key == true`)
    // into bits 39..=63 so any codegen/stub knob set via `PORT_*`, legacy `AW2_*`, or
    // `game.toml [knobs]` partitions the `.aot` cache key without disturbing bits 0..=38
    // read by `StubCfg::from_aot_knobs` and static Mach-O emission.
    let high = (port_manifest::knobs::aot_key_hash() & ((1u64 << 25) - 1)) << 39;
    low | high
}

#[cfg(test)]
mod jit_aot_tests {
    use super::*;

    #[test]
    fn static_stub_veneer_layout_and_bl_target() {
        const VBASE: usize = 0x2000_0000_0000;
        let knobs = aot_knob_bits();
        let mut live_buf = vec![0u32; AOT_STUBS_BYTES / 4];
        let want = unsafe { aot_emit_stubs(live_buf.as_mut_ptr(), VBASE) }.expect("live stubs");

        // Single-bank direct relocation mode: bl 0 at host_xfer_bl_off.
        let mut direct_buf = vec![0u32; AOT_STUBS_BYTES / 4];
        let (got_direct, bl_off) = unsafe {
            emit_static_stubs(direct_buf.as_mut_ptr(), VBASE, knobs, &want)
        }
        .expect("direct static stubs");
        assert_eq!(got_direct, want);

        // Multi-bank veneer mode: local bl -> 16B-aligned GOT veneer in stub bank tail.
        let mut veneer_buf = vec![0u32; AOT_STUBS_BYTES / 4];
        let (got_veneer, veneer_off) = unsafe {
            emit_static_stubs_with_veneer(veneer_buf.as_mut_ptr(), VBASE, knobs, &want)
        }
        .expect("veneer static stubs");
        assert_eq!(got_veneer, want);

        if let (Some(bl_b), Some(ven_b)) = (bl_off, veneer_off) {
            assert_eq!(ven_b % 16, 0, "veneer must be 16-byte aligned");
            assert!(ven_b + 16 <= AOT_STUBS_BYTES, "veneer must fit in stub bank");
            assert_eq!(direct_buf[bl_b / 4], 0x9400_0000, "direct mode emits relocatable bl 0");
            let bl_w = veneer_buf[bl_b / 4];
            assert_eq!(bl_w & 0xFC00_0000, 0x9400_0000, "patched instruction must remain BL");
            let imm26 = (((bl_w & 0x03FF_FFFF) as i32) << 6) >> 6;
            assert_eq!((bl_b / 4) as i64 + imm26 as i64, (ven_b / 4) as i64);

            let vw = ven_b / 4;
            assert_eq!(veneer_buf[vw], 0x9000_0010, "adrp x16, 0");
            assert_eq!(veneer_buf[vw + 1], 0xF940_0210, "ldr x16, [x16, #0]");
            assert_eq!(veneer_buf[vw + 2], 0xD61F_0200, "br x16");
            assert_eq!(veneer_buf[vw + 3], 0xD503_201F, "nop");
        } else {
            assert_eq!(bl_off, veneer_off);
        }
    }

    #[test]
    fn aot_knob_bits_folds_registry_aot_key_hash() {
        let prev = std::env::var_os("PORT_JIT_THUNK_ELIDE");
        std::env::remove_var("PORT_JIT_THUNK_ELIDE");
        let base = aot_knob_bits();
        std::env::set_var("PORT_JIT_THUNK_ELIDE", "2");
        let changed = aot_knob_bits();
        match prev {
            Some(v) => std::env::set_var("PORT_JIT_THUNK_ELIDE", v),
            None => std::env::remove_var("PORT_JIT_THUNK_ELIDE"),
        }
        assert_ne!(
            base >> 39,
            changed >> 39,
            "changing an AOT-key knob must change the folded high-bit digest in aot_knob_bits()"
        );
        assert_eq!(
            base & ((1u64 << 39) - 1),
            changed & ((1u64 << 39) - 1),
            "folding registry hash into bits 39..=63 must not alter bits 0..=38"
        );
    }
}
