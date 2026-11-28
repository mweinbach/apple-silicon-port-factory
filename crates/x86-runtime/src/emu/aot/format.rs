//! AOT `.aot` file format (v5–v7), image/codegen hashing, cache-file lookup, and
//! multi-bank hot/cold `Image` layout.

use crate::emu::jit::{
    self, AotReloc, AotStubOffs, AOT_STUBS_BYTES, RELOC_HOST_FN, RELOC_QPC_STATE,
    RELOC_XINPUT_STATE,
};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use super::builder::split_and_rebase_block;
use super::{building, cache_dir, env_var_any, fallback_cache_dirs_for};

pub(super) const MAGIC: [u8; 8] = *b"AW2AOT\0\x01";
/// Legacy single-bank format version (still cleanly decoded and upgraded to 1-bank `Image`).
pub(super) const FORMAT_VERSION_V5: u32 = 5;
/// Multi-bank format version (stores `stub_banks` table for >112 MiB executables).
pub(super) const FORMAT_VERSION_V6: u32 = 6;
/// Includes both shared timestamp entry points in each bank's stub table.
pub(super) const FORMAT_VERSION_V7: u32 = 7;
/// Includes per-site IC lookup/RAS-miss stub entries (`[u32; 16]`) and 2-way table/IC codegen.
pub(super) const FORMAT_VERSION: u32 = 8;
const LEGACY_STUB_COUNT_V6: usize = 10;
const LEGACY_STUB_COUNT_V7: usize = 12;
/// Bump on any codegen-relevant change the source hash can't see.
const AOT_CODEGEN_VERSION: u64 = 6;

/// Small 128-bit streaming hash (cache keys / file checksums, not cryptographic).
pub(super) struct H128 {
    a: u64,
    b: u64,
    n: u64,
}

impl H128 {
    pub(super) fn new(seed: u64) -> Self {
        H128 {
            a: 0x9E37_79B9_7F4A_7C15 ^ seed,
            b: 0xC2B2_AE3D_27D4_EB4F ^ seed.rotate_left(32),
            n: 0,
        }
    }
    pub(super) fn word(&mut self, w: u64) {
        self.a = (self.a ^ w).wrapping_mul(0x0000_0100_0000_01B3).rotate_left(29);
        self.b = (self.b.wrapping_add(w) ^ (self.b >> 29)).wrapping_mul(0xFF51_AFD7_ED55_8CCD);
        self.n += 1;
    }
    pub(super) fn bytes(&mut self, d: &[u8]) {
        let mut ch = d.chunks_exact(8);
        for c in ch.by_ref() {
            self.word(u64::from_le_bytes([c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7]]));
        }
        let r = ch.remainder();
        if !r.is_empty() {
            let mut t = [0u8; 8];
            t[..r.len()].copy_from_slice(r);
            self.word(u64::from_le_bytes(t) ^ ((r.len() as u64) << 56));
        }
        self.word(d.len() as u64);
    }
    pub(super) fn finish(&self) -> [u64; 2] {
        let f = |x: u64| {
            let x = (x ^ (x >> 33)).wrapping_mul(0xFF51_AFD7_ED55_8CCD);
            let x = (x ^ (x >> 33)).wrapping_mul(0xC4CE_B9FE_1A85_EC53);
            x ^ (x >> 33)
        };
        [f(self.a ^ self.n), f(self.b ^ self.a.rotate_left(17))]
    }
}

/// Hash of the translator / decoder sources (embedded), the format and the `Cpu` layout:
/// any change to them invalidates every cache, unrelated rebuilds keep them.
pub(in crate::emu) fn codegen_hash() -> u64 {
    static V: OnceLock<u64> = OnceLock::new();
    *V.get_or_init(|| {
        const SRC: &[&[u8]] = &[
            include_bytes!("../features.rs"),
            include_bytes!("../cpuid.rs"),
            include_bytes!("../denorm.rs"),
            include_bytes!("../mmx.rs"),
            include_bytes!("../x87.rs"),
            include_bytes!("../xstate.rs"),
            include_bytes!("../jit.rs"),
            include_bytes!("../jit/emit.rs"),
            include_bytes!("../jit/cache.rs"),
            include_bytes!("../jit/stubs.rs"),
            include_bytes!("../jit/translate.rs"),
            include_bytes!("../jit/ops.rs"),
            include_bytes!("../jit/vec.rs"),
            include_bytes!("../jit/ext.rs"),
            include_bytes!("../jit/mmx_gather.rs"),
            include_bytes!("../jit/more.rs"),
            include_bytes!("../jit/x87.rs"),
            include_bytes!("../jit/xmm.rs"),
            include_bytes!("../jit/lazy.rs"),
            include_bytes!("../jit/aot.rs"),
            include_bytes!("../fast.rs"),
            include_bytes!("../fast/x87.rs"),
            include_bytes!("../fast/decode.rs"),
            include_bytes!("../fast/exec.rs"),
            include_bytes!("../fast/ext.rs"),
            include_bytes!("../fast/mmx_gather.rs"),
            include_bytes!("../fast/more.rs"),
            include_bytes!("../mod.rs"),
            include_bytes!("../routes.rs"),
            include_bytes!("../dispatch.rs"),
            include_bytes!("../cpu.rs"),
            include_bytes!("../tsc.rs"),
            include_bytes!("../block_cache.rs"),
            include_bytes!("../interp.rs"),
            include_bytes!("../generic_ops.rs"),
            include_bytes!("../mem.rs"),
            include_bytes!("volatile.rs"),
            include_bytes!("../simd_kernels.rs"),
            include_bytes!("../simd_mov.rs"),
            include_bytes!("../simd_fp.rs"),
            include_bytes!("../regfile.rs"),
        ];
        let mut h = H128::new(AOT_CODEGEN_VERSION);
        for s in SRC {
            h.bytes(s);
        }
        h.word(core::mem::size_of::<crate::emu::Cpu>() as u64);
        h.word(crate::emu::BLK_MAX_INSNS as u64);
        h.word(FORMAT_VERSION as u64);
        h.finish()[0]
    })
}

/// Everything a module's translations depend on besides the block set and the
/// host-callable state (assumptions / relocations). Any mismatch: the cache is ignored.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(in crate::emu) struct AotKey {
    /// Hash of the mapped, relocated executable sections (+ image size).
    pub(in crate::emu) image_hash: [u64; 2],
    pub(in crate::emu) image_base: u64,
    pub(in crate::emu) image_size: u64,
    pub(in crate::emu) codegen_hash: u64,
    pub(in crate::emu) knobs: u64,
}

impl AotKey {
    pub(super) fn words(&self) -> [u64; 6] {
        [
            self.image_hash[0],
            self.image_hash[1],
            self.image_base,
            self.image_size,
            self.codegen_hash,
            self.knobs,
        ]
    }
    pub(super) fn id(&self) -> u64 {
        let mut h = H128::new(0xA07);
        for w in self.words() {
            h.word(w);
        }
        h.finish()[0]
    }
}

/// Verify that the mapped image at `[base, base + size)` is a 64-bit AMD64 PE32+ image
/// (`COFF.Machine == 0x8664`, `OptionalHeader.Magic == 0x020b`).
pub(super) unsafe fn validate_mapped_pe32_plus(base: usize, size: usize) -> Result<(), String> {
    const IMAGE_FILE_MACHINE_AMD64: u16 = 0x8664;
    const IMAGE_NT_OPTIONAL_HDR64_MAGIC: u16 = 0x020B;
    if base == 0 || size < 0x400 {
        return Err("mapped image too small for PE headers".into());
    }
    let rd16 = |off: usize| core::ptr::read_unaligned((base + off) as *const u16);
    let rd32 = |off: usize| core::ptr::read_unaligned((base + off) as *const u32);
    if rd16(0) != 0x5A4D {
        return Err("not a PE image (missing MZ signature)".into());
    }
    let nt = rd32(0x3C) as usize;
    let Some(opt) = nt.checked_add(24) else {
        return Err("invalid PE header offset".into());
    };
    let Some(opt_magic_end) = opt.checked_add(2) else {
        return Err("invalid PE optional header offset".into());
    };
    if opt_magic_end > size || rd32(nt) != 0x0000_4550 {
        return Err("not a PE image (missing PE signature)".into());
    }
    let machine = rd16(nt + 4);
    let size_of_opt = rd16(nt + 20);
    if size_of_opt < 2 {
        return Err("PE has no optional header".into());
    }
    let magic = rd16(opt);
    if magic != IMAGE_NT_OPTIONAL_HDR64_MAGIC {
        return Err(format!(
            "not a PE32+ image (magic 0x{magic:x}, machine 0x{machine:x})"
        ));
    }
    if machine != IMAGE_FILE_MACHINE_AMD64 {
        return Err(format!(
            "unsupported machine 0x{machine:x} (expected AMD64 0x{IMAGE_FILE_MACHINE_AMD64:x})"
        ));
    }
    Ok(())
}

/// Executable sections `[start, end)` of the PE32+ AMD64 image mapped at `[base, base + size)`
/// (empty if the headers are not a valid AMD64 PE32+ image).
pub(super) unsafe fn exec_sections(base: usize, size: usize) -> Vec<(usize, usize)> {
    const CNT_CODE: u32 = 0x20;
    const MEM_EXECUTE: u32 = 0x2000_0000;
    let mut out = Vec::new();
    if validate_mapped_pe32_plus(base, size).is_err() {
        return out;
    }
    let rd16 = |off: usize| core::ptr::read_unaligned((base + off) as *const u16);
    let rd32 = |off: usize| core::ptr::read_unaligned((base + off) as *const u32);
    let nt = rd32(0x3C) as usize;
    let nsec = rd16(nt + 6) as usize;
    let table = nt + 24 + rd16(nt + 20) as usize;
    if table + nsec * 40 > size {
        return out;
    }
    for i in 0..nsec {
        let e = table + i * 40;
        let vsize = rd32(e + 8) as usize;
        let va = rd32(e + 12) as usize;
        let ch = rd32(e + 36);
        if ch & (MEM_EXECUTE | CNT_CODE) == 0 || vsize == 0 {
            continue;
        }
        let end = va.saturating_add(vsize).min(size);
        if va < end {
            out.push((base + va, base + end));
        }
    }
    out
}

/// Non-executable initialized data sections `[start, end)` of the PE32+ AMD64 image mapped at
/// `[base, base + size)`, excluding PE headers `[0..0x400)` and discardable sections.
pub(super) unsafe fn data_sections(base: usize, size: usize) -> Vec<(usize, usize)> {
    const CNT_CODE: u32 = 0x20;
    const CNT_INIT_DATA: u32 = 0x40;
    const MEM_DISCARDABLE: u32 = 0x0200_0000;
    const MEM_EXECUTE: u32 = 0x2000_0000;
    let mut out = Vec::new();
    if validate_mapped_pe32_plus(base, size).is_err() {
        return out;
    }
    let rd16 = |off: usize| core::ptr::read_unaligned((base + off) as *const u16);
    let rd32 = |off: usize| core::ptr::read_unaligned((base + off) as *const u32);
    let nt = rd32(0x3C) as usize;
    let Some(nt_end) = nt.checked_add(24) else {
        return out;
    };
    let nsec = rd16(nt + 6) as usize;
    let Some(table) = nt_end.checked_add(rd16(nt + 20) as usize) else {
        return out;
    };
    let Some(table_end) = nsec.checked_mul(40).and_then(|n| table.checked_add(n)) else {
        return out;
    };
    if table_end > size {
        return out;
    }
    for i in 0..nsec {
        let e = table + i * 40;
        let vsize = rd32(e + 8) as usize;
        let va = rd32(e + 12) as usize;
        let ch = rd32(e + 36);
        if (ch & (MEM_EXECUTE | CNT_CODE | MEM_DISCARDABLE)) != 0
            || (ch & CNT_INIT_DATA) == 0
            || vsize == 0
        {
            continue;
        }
        let start = va.max(0x400);
        let end = va.saturating_add(vsize).min(size);
        if start < end {
            out.push((base + start, base + end));
        }
    }
    out
}

pub(super) unsafe fn image_hash(base: usize, size: usize) -> [u64; 2] {
    let mut h = H128::new(size as u64);
    for (s, e) in exec_sections(base, size) {
        h.word((s - base) as u64);
        h.bytes(core::slice::from_raw_parts(s as *const u8, e - s));
    }
    if let Some(vm) = super::volatile::parse_mapped_volatile_metadata(base, size) {
        h.word(0x564F_4C4D_4554_4131u64); // "VOLMETA1" domain separator
        h.word(vm.ranges.len() as u64);
        for &(s, e) in vm.ranges.iter() {
            h.word(((s as u64) << 32) | (e as u64));
        }
        h.word(vm.access_rvas.len() as u64);
        for &rva in vm.access_rvas.iter() {
            h.word(rva as u64);
        }
    }
    h.finish()
}

pub(super) unsafe fn module_key(base: usize, size: usize) -> AotKey {
    module_key_hashed(base, size, None)
}

/// [`module_key`] with the image hash already computed (`h`, e.g. at activation).
pub(super) unsafe fn module_key_hashed(base: usize, size: usize, h: Option<[u64; 2]>) -> AotKey {
    AotKey {
        image_hash: h.unwrap_or_else(|| image_hash(base, size)),
        image_base: base as u64,
        image_size: size as u64,
        codegen_hash: codegen_hash(),
        knobs: jit::aot_knob_bits(),
    }
}

pub(super) fn cache_file(name: &str, key: &AotKey) -> PathBuf {
    cache_dir().join(format!("{name}-{:016x}.aot", key.id()))
}

pub(super) fn profile_file(name: &str, image_hash: &[u64; 2]) -> PathBuf {
    cache_dir().join("profile").join(format!("{name}-{:016x}.tsv", image_hash[0]))
}

// ---------------------------------------------------------------------------------------------
// Image (one module's translations; file format)
// ---------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub(in crate::emu) struct BlockEnt {
    pub(in crate::emu) ip: u64,
    /// Byte offset of the block's hot main-path code (`main` words) from region start.
    pub(in crate::emu) code_off: u32,
    /// Byte offset of the block's cold out-of-line code (`words - main` words) from region start.
    pub(in crate::emu) cold_off: u32,
    pub(in crate::emu) words: u32,
    pub(in crate::emu) main: u32,
    pub(in crate::emu) guest: u32,
    pub(in crate::emu) host_entry: bool,
    pub(in crate::emu) link_lo: u32,
    pub(in crate::emu) link_n: u32,
    pub(in crate::emu) reloc_lo: u32,
    pub(in crate::emu) reloc_n: u32,
    pub(in crate::emu) assume_lo: u32,
    pub(in crate::emu) assume_n: u32,
}

/// A direct-link site (`b`/`bl`), built pointing at its out-of-line fallback.
/// `cond_off != u32::MAX` names a hot conditional branch (`b.cond`/`cbz`/`cbnz`) pointing
/// at `site_off` that `link_pass` directly patches to `target` when within ±1 MiB.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(in crate::emu) struct LinkEnt {
    pub(in crate::emu) site_off: u32,
    pub(in crate::emu) fallback_off: u32,
    pub(in crate::emu) cond_off: u32,
    pub(in crate::emu) target: u64,
}

/// A relocated 8-byte literal slot ([`AotReloc`]): `RELOC_HOST_FN` holds
/// `inline_host_fn(va).0` (whose `.1` must equal `bail`), `RELOC_SLOT_VAL` holds `va`.
/// `src` != 0: `va` is the value of that guest import slot (rebased at load).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(in crate::emu) struct RelocEnt {
    pub(in crate::emu) slot_off: u32,
    pub(in crate::emu) kind: u8,
    pub(in crate::emu) bail: bool,
    pub(in crate::emu) va: u64,
    pub(in crate::emu) src: u64,
}

#[inline(always)]
pub(super) fn word_to_off(w: u32, main: u32, hot_off: u32, cold_off: u32) -> u32 {
    if w < main {
        hot_off + w * 4
    } else {
        cold_off + (w - main) * 4
    }
}

impl RelocEnt {
    pub(super) fn of_split(hot_off: u32, cold_off: u32, main: u32, r: &AotReloc) -> Self {
        RelocEnt {
            slot_off: word_to_off(r.word, main, hot_off, cold_off),
            kind: r.kind,
            bail: r.bail,
            va: r.va,
            src: r.src,
        }
    }
}

pub(in crate::emu) struct Image {
    pub(in crate::emu) stubs: AotStubOffs,
    /// Region byte offsets of every `AOT_STUBS_BYTES` stub bank (16-byte aligned, strictly
    /// ascending, always starting at `0`). Bank `0` lives at `[0..AOT_STUBS_BYTES)`; each
    /// additional bank `k >= 1` occupies `[stub_banks[k]..stub_banks[k] + AOT_STUBS_BYTES)`
    /// inside `code` (reserved as NOP words on disk, stamped with stubs at load / Mach-O emit).
    pub(in crate::emu) stub_banks: Vec<u32>,
    pub(in crate::emu) blocks: Vec<BlockEnt>,
    pub(in crate::emu) links: Vec<LinkEnt>,
    pub(in crate::emu) relocs: Vec<RelocEnt>,
    /// `(va, host-callable)`.
    pub(in crate::emu) assume: Vec<(u64, bool)>,
    /// Code words; word 0 sits at region offset [`AOT_STUBS_BYTES`].
    /// Released via [`Image::release_code_heap`] after installation into `MAP_JIT` when
    /// `verify_mode() == 0` (or zero revivable blocks exist) to avoid holding a redundant
    /// heap copy of mapped AOT machine code.
    pub(in crate::emu) code: Vec<u32>,
    /// Retained word count of `code` after [`Image::release_code_heap`] frees the heap buffer.
    pub(super) code_words_len: usize,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            stubs: AotStubOffs::default(),
            stub_banks: vec![0],
            blocks: Vec::new(),
            links: Vec::new(),
            relocs: Vec::new(),
            assume: Vec::new(),
            code: Vec::new(),
            code_words_len: 0,
        }
    }
}

/// Virtual region base AOT builds are emitted for (never mapped: only differences matter,
/// all branches out of a block are PC-relative).
pub(super) const BUILD_VBASE: usize = 0x2000_0000_0000;
/// Maximum byte span of one stub bank (112 MiB, safely inside ARM64's ±128 MiB `B`/`BL`
/// reach with headroom for `AOT_STUBS_BYTES`, chunks, and a per-bank LSE2 patch slab).
pub(super) const AOT_BANK_SPAN: usize = 112 << 20;
/// Total code ceiling per module across all stub banks (2 GiB: covers 200+ MiB Windows
/// executables while staying well inside Mach-O 32-bit file-offset limits).
pub(super) const MAX_CODE: usize = 2048 << 20;
/// Per-bank LSE2 exact-TSO unaligned-trap patch slab in dynamic `MAP_JIT` regions.
pub(super) const AOT_LSE2_SLAB: usize = 256 << 10;
/// Per-bank far-branch island slab (16 bytes per deduplicated far target: `ldr x16, .+8;
/// br x16; .quad target`) appended after each bank's LSE2 slab so cross-bank and
/// cross-module `b`/`bl` links beyond ±128 MiB direct reach stay chained without falling
/// back to `lookup_rip`.
pub(super) const AOT_FAR_SLAB: usize = 128 << 10;

impl Image {
    pub(super) fn code_bytes(&self) -> usize {
        self.code.len().max(self.code_words_len) * 4
    }

    /// Free the heap-allocated `code` buffer once machine code has been copied into `MAP_JIT`,
    /// preserving `code_bytes()` / `bank_end()` metadata for PC sampling and diagnostics.
    pub(super) fn release_code_heap(&mut self) {
        self.code_words_len = self.code.len().max(self.code_words_len);
        self.code = Vec::new();
    }

    pub(in crate::emu) fn stub_banks(&self) -> &[u32] {
        const SINGLE: [u32; 1] = [0];
        if self.stub_banks.is_empty() {
            &SINGLE
        } else {
            &self.stub_banks
        }
    }

    /// Bank index (`0..stub_banks().len()`) owning region byte offset `off`.
    #[inline]
    pub(in crate::emu) fn bank_index_of(&self, off: u32) -> usize {
        let banks = self.stub_banks();
        if banks.len() <= 1 {
            0
        } else {
            banks.partition_point(|&b| b <= off).saturating_sub(1)
        }
    }

    /// Region byte offset of the stub bank owning region byte offset `off`.
    #[inline]
    pub(in crate::emu) fn bank_of_off(&self, off: u32) -> u32 {
        self.stub_banks()[self.bank_index_of(off)]
    }

    /// Exclusive flat region byte offset ending bank `bank_idx`.
    #[inline]
    pub(in crate::emu) fn bank_end(&self, bank_idx: usize) -> usize {
        let banks = self.stub_banks();
        if bank_idx + 1 < banks.len() {
            banks[bank_idx + 1] as usize
        } else {
            AOT_STUBS_BYTES + self.code_bytes()
        }
    }

    /// Open a new 16-byte-aligned stub bank at the current end of `self.code`, reserving
    /// `AOT_STUBS_BYTES` placeholder NOP words for the bank's shared stubs.
    pub(super) fn start_new_bank(&mut self) -> usize {
        while self.code.len() % 4 != 0 {
            self.code.push(0xD503_201F);
        }
        let bank_off = AOT_STUBS_BYTES + self.code_bytes();
        debug_assert!(bank_off <= u32::MAX as usize && bank_off % 16 == 0);
        if self.stub_banks.is_empty() {
            self.stub_banks.push(0);
        }
        self.stub_banks.push(bank_off as u32);
        self.code.resize(self.code.len() + AOT_STUBS_BYTES / 4, 0xD503_201F);
        bank_off
    }

    /// Flush a chunk of blocks into `self` inside stub bank `bank_off`, with hot main paths
    /// packed contiguously first and cold out-of-line tails packed immediately after (keeping
    /// all hot<->cold branches well inside ±32 KiB `tbz`/`tbnz` reach and letting consecutive
    /// hot blocks fall through).
    pub(super) fn push_chunk(
        &mut self,
        chunk: Vec<(u64, usize, jit::AotBlock)>,
        offs: &AotStubOffs,
        bank_off: usize,
    ) {
        if chunk.is_empty() {
            return;
        }
        // 1. Lay out all hot main paths back-to-back (4-byte aligned, zero wasted NOPs).
        let base_word = AOT_STUBS_BYTES / 4 + self.code.len();
        let mut hot_words = Vec::with_capacity(chunk.len());
        let mut cur_w = base_word;
        for &(_, _, ref b) in &chunk {
            hot_words.push(cur_w);
            cur_w += b.main as usize;
        }
        // 2. Lay out cold tails immediately after the chunk's hot section, matching each
        //    block's `main % 4` mod-16 alignment so 8B reloc slots and 16B vector literals
        //    preserve their exact alignment relative to the 16B-aligned region start.
        let mut cold_words = Vec::with_capacity(chunk.len());
        let mut cold_pads = Vec::with_capacity(chunk.len());
        for &(_, _, ref b) in &chunk {
            let cold_len = b.words.len() - b.main as usize;
            if cold_len == 0 {
                cold_words.push(cur_w);
                cold_pads.push(0usize);
            } else {
                let want = (b.main as usize) & 3;
                let pad = (want + 4 - (cur_w & 3)) & 3;
                cur_w += pad;
                cold_words.push(cur_w);
                cold_pads.push(pad);
                cur_w += cold_len;
            }
        }
        // 3. Rebase branches (to this bank's stubs + hot/cold split) and append hot, then cold.
        let mut cold_tail = Vec::with_capacity(chunk.len());
        for (i, (ip, from_off, mut b)) in chunk.into_iter().enumerate() {
            let hot_off = (hot_words[i] * 4) as u32;
            let cold_off = (cold_words[i] * 4) as u32;
            split_and_rebase_block(
                &mut b,
                from_off,
                0,
                hot_off as usize,
                cold_off as usize,
                bank_off,
                offs,
            );
            let main = b.main;
            self.blocks.push(BlockEnt {
                ip,
                code_off: hot_off,
                cold_off,
                words: b.words.len() as u32,
                main,
                guest: b.guest,
                host_entry: b.host_entry,
                link_lo: self.links.len() as u32,
                link_n: b.links.len() as u32,
                reloc_lo: self.relocs.len() as u32,
                reloc_n: b.relocs.len() as u32,
                assume_lo: self.assume.len() as u32,
                assume_n: b.assume.len() as u32,
            });
            self.links.extend(b.links.iter().map(|&(s, f, c, t)| LinkEnt {
                site_off: word_to_off(s, main, hot_off, cold_off),
                fallback_off: word_to_off(f, main, hot_off, cold_off),
                cond_off: if c == u32::MAX {
                    u32::MAX
                } else {
                    word_to_off(c, main, hot_off, cold_off)
                },
                target: t,
            }));
            self.relocs.extend(b.relocs.iter().map(|r| RelocEnt::of_split(hot_off, cold_off, main, r)));
            self.assume.extend_from_slice(&b.assume);
            self.code.extend_from_slice(&b.words[..main as usize]);
            cold_tail.push((cold_pads[i], b));
        }
        for (pad, b) in cold_tail {
            for _ in 0..pad {
                self.code.push(0xD503_201F);
            }
            self.code.extend_from_slice(&b.words[b.main as usize..]);
        }
        // Keep the next chunk 16-byte aligned.
        while self.code.len() % 4 != 0 {
            self.code.push(0xD503_201F);
        }
    }

    pub(in crate::emu) fn links_of(&self, b: &BlockEnt) -> &[LinkEnt] {
        &self.links[b.link_lo as usize..(b.link_lo + b.link_n) as usize]
    }
    pub(in crate::emu) fn relocs_of(&self, b: &BlockEnt) -> &[RelocEnt] {
        &self.relocs[b.reloc_lo as usize..(b.reloc_lo + b.reloc_n) as usize]
    }
    pub(in crate::emu) fn assume_of(&self, b: &BlockEnt) -> &[(u64, bool)] {
        &self.assume[b.assume_lo as usize..(b.assume_lo + b.assume_n) as usize]
    }
    pub(super) fn hot_words_of(&self, b: &BlockEnt) -> &[u32] {
        let w = (b.code_off as usize - AOT_STUBS_BYTES) / 4;
        &self.code[w..w + b.main as usize]
    }
    pub(super) fn cold_words_of(&self, b: &BlockEnt) -> &[u32] {
        let cold_len = (b.words - b.main) as usize;
        if cold_len == 0 {
            return &[];
        }
        let w = (b.cold_off as usize - AOT_STUBS_BYTES) / 4;
        &self.code[w..w + cold_len]
    }

    /// Structural sanity (a corrupt or hostile file must not make us patch outside the
    /// region, overlap a stub bank, or exceed ARM64 ±128 MiB stub-branch range).
    pub(super) fn valid(&self) -> bool {
        let Some(end) = (AOT_STUBS_BYTES as u64).checked_add(self.code_bytes() as u64) else {
            return false;
        };
        if end > u32::MAX as u64 {
            return false;
        }
        let banks = self.stub_banks();
        if banks.is_empty() || banks[0] != 0 {
            return false;
        }
        for (bi, &bo) in banks.iter().enumerate() {
            if bo % 16 != 0 {
                return false;
            }
            let bo64 = bo as u64;
            let b_end = self.bank_end(bi) as u64;
            if bo64 + AOT_STUBS_BYTES as u64 > b_end || b_end > end {
                return false;
            }
            if bi > 0 && bo <= banks[bi - 1] {
                return false;
            }
            // Every code word inside a bank must stay within ARM64's ±128 MiB `B`/`BL` range
            // of that bank's stubs at `bo`.
            if b_end - bo64 > (128u64 << 20) {
                return false;
            }
        }
        let in_code = |off: u32, len: u64, align: u32| -> bool {
            if off % align != 0 {
                return false;
            }
            let bi = self.bank_index_of(off);
            let lo = banks[bi] as u64 + AOT_STUBS_BYTES as u64;
            let hi = self.bank_end(bi) as u64;
            let off64 = off as u64;
            off64 >= lo && off64.checked_add(len).is_some_and(|e| e <= hi)
        };
        let range = |lo: u32, n: u32, len: usize| (lo as u64 + n as u64) <= len as u64;
        self.blocks.iter().all(|b| {
            b.main <= b.words
                && in_code(b.code_off, b.main as u64 * 4, 4)
                && (b.words == b.main
                    || (in_code(b.cold_off, (b.words - b.main) as u64 * 4, 4)
                        && self.bank_index_of(b.code_off) == self.bank_index_of(b.cold_off)))
                && range(b.link_lo, b.link_n, self.links.len())
                && range(b.reloc_lo, b.reloc_n, self.relocs.len())
                && range(b.assume_lo, b.assume_n, self.assume.len())
        }) && self.links.iter().all(|l| {
            in_code(l.site_off, 4, 4)
                && in_code(l.fallback_off, 4, 4)
                && (l.cond_off == u32::MAX || in_code(l.cond_off, 4, 4))
        }) && self.relocs.iter().all(|r| {
            in_code(r.slot_off, 8, 8)
                && r.kind <= RELOC_XINPUT_STATE
                && (r.kind == RELOC_HOST_FN
                    || r.kind == RELOC_QPC_STATE
                    || r.kind == RELOC_XINPUT_STATE
                    || r.src != 0)
        })
    }
}

fn put32(w: &mut Vec<u8>, v: u32) {
    w.extend_from_slice(&v.to_le_bytes());
}
fn put64(w: &mut Vec<u8>, v: u64) {
    w.extend_from_slice(&v.to_le_bytes());
}

struct Rd<'a> {
    b: &'a [u8],
    p: usize,
}

impl<'a> Rd<'a> {
    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        let s = self.b.get(self.p..self.p.checked_add(n)?)?;
        self.p += n;
        Some(s)
    }
    fn u32(&mut self) -> Option<u32> {
        let s = self.take(4)?;
        Some(u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
    }
    fn u64(&mut self) -> Option<u64> {
        let s = self.take(8)?;
        Some(u64::from_le_bytes([s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7]]))
    }
}

/// Header flag: built in the background during a run; verify at its first load
/// ([`super::runtime::load_module`]), then cleared in the file.
pub(super) const FLAG_VERIFY_PENDING: u32 = 1;

/// Serialise (little-endian, v7): header, stub banks, blocks, links, relocs, assumptions,
/// code, checksum.
pub(super) fn encode(name: &str, key: &AotKey, img: &Image, flags: u32) -> Vec<u8> {
    encode_with_version(name, key, img, flags, FORMAT_VERSION)
}

pub(super) fn encode_with_version(
    name: &str,
    key: &AotKey,
    img: &Image,
    flags: u32,
    version: u32,
) -> Vec<u8> {
    let base = key.image_base;
    let banks = img.stub_banks();
    let cap = 256
        + name.len()
        + banks.len() * 4
        + img.blocks.len() * 52
        + img.links.len() * 20
        + img.relocs.len() * 24
        + img.assume.len() * 16
        + img.code_bytes();
    let mut w = Vec::with_capacity(cap);
    w.extend_from_slice(&MAGIC);
    put32(&mut w, version);
    put32(&mut w, flags);
    for v in key.words() {
        put64(&mut w, v);
    }
    put32(&mut w, name.len() as u32);
    w.extend_from_slice(name.as_bytes());
    let stub_count = if version <= FORMAT_VERSION_V6 {
        LEGACY_STUB_COUNT_V6
    } else if version == FORMAT_VERSION_V7 {
        LEGACY_STUB_COUNT_V7
    } else {
        img.stubs.len()
    };
    for &s in &img.stubs[..stub_count] {
        put32(&mut w, s);
    }
    put32(&mut w, AOT_STUBS_BYTES as u32);
    if version >= FORMAT_VERSION_V6 {
        put32(&mut w, banks.len() as u32);
        for &bo in banks {
            put32(&mut w, bo);
        }
    }
    for n in [
        img.blocks.len(),
        img.links.len(),
        img.relocs.len(),
        img.assume.len(),
        img.code.len(),
    ] {
        put32(&mut w, n as u32);
    }
    for b in &img.blocks {
        for v in [
            b.ip.wrapping_sub(base) as u32,
            b.code_off,
            b.cold_off,
            b.words,
            b.main,
            b.guest,
            b.host_entry as u32,
            b.link_lo,
            b.link_n,
            b.reloc_lo,
            b.reloc_n,
            b.assume_lo,
            b.assume_n,
        ] {
            put32(&mut w, v);
        }
    }
    for l in &img.links {
        put32(&mut w, l.site_off);
        put32(&mut w, l.fallback_off);
        put32(&mut w, l.cond_off);
        put64(&mut w, l.target);
    }
    for r in &img.relocs {
        put32(&mut w, r.slot_off);
        put32(&mut w, r.bail as u32 | (r.kind as u32) << 8);
        put64(&mut w, r.va);
        put64(&mut w, r.src);
    }
    for &(va, host) in &img.assume {
        put64(&mut w, va);
        put64(&mut w, host as u64);
    }
    for &c in &img.code {
        put32(&mut w, c);
    }
    let mut h = H128::new(0xC5);
    h.bytes(&w);
    put64(&mut w, h.finish()[0]);
    w
}

/// Replace the header flags of an encoded file (checksum recomputed). False: too short.
pub(super) fn set_flags(bytes: &mut [u8], flags: u32) -> bool {
    if bytes.len() < 24 {
        return false;
    }
    bytes[12..16].copy_from_slice(&flags.to_le_bytes());
    let n = bytes.len() - 8;
    let mut h = H128::new(0xC5);
    h.bytes(&bytes[..n]);
    bytes[n..].copy_from_slice(&h.finish()[0].to_le_bytes());
    true
}

/// Parse a v5 single-bank or v6/v7 multi-bank `.aot` file:
/// `(module name, key, image, header flags)`.
pub(in crate::emu) fn decode(bytes: &[u8]) -> Option<(String, AotKey, Image, u32)> {
    if bytes.len() < 16 {
        return None;
    }
    let (body, sum) = bytes.split_at(bytes.len() - 8);
    let mut h = H128::new(0xC5);
    h.bytes(body);
    if h.finish()[0].to_le_bytes() != sum {
        return None;
    }
    let mut r = Rd { b: body, p: 0 };
    if r.take(8)? != MAGIC {
        return None;
    }
    let version = r.u32()?;
    if !(FORMAT_VERSION_V5..=FORMAT_VERSION).contains(&version) {
        return None;
    }
    let flags = r.u32()?;
    let mut k = [0u64; 6];
    for v in k.iter_mut() {
        *v = r.u64()?;
    }
    let key = AotKey {
        image_hash: [k[0], k[1]],
        image_base: k[2],
        image_size: k[3],
        codegen_hash: k[4],
        knobs: k[5],
    };
    let nlen = r.u32()? as usize;
    let name = String::from_utf8(r.take(nlen)?.to_vec()).ok()?;
    let mut img = Image::default();
    let stub_count = if version <= FORMAT_VERSION_V6 {
        LEGACY_STUB_COUNT_V6
    } else if version == FORMAT_VERSION_V7 {
        LEGACY_STUB_COUNT_V7
    } else {
        img.stubs.len()
    };
    img.stubs[stub_count..].fill(u32::MAX);
    for s in &mut img.stubs[..stub_count] {
        *s = r.u32()?;
    }
    if r.u32()? as usize != AOT_STUBS_BYTES {
        return None;
    }
    if version >= FORMAT_VERSION_V6 {
        let nbanks = r.u32()? as usize;
        if nbanks == 0 || nbanks.checked_mul(4).is_none_or(|b| b > body.len() - r.p) {
            return None;
        }
        img.stub_banks.clear();
        img.stub_banks.reserve(nbanks);
        for _ in 0..nbanks {
            img.stub_banks.push(r.u32()?);
        }
    } else {
        img.stub_banks = vec![0];
    }
    let (nb, nl, nr, na, nc) = (
        r.u32()? as usize,
        r.u32()? as usize,
        r.u32()? as usize,
        r.u32()? as usize,
        r.u32()? as usize,
    );
    // Table sizes must fit the remaining body before allocating (overflow-safe).
    let need = nb
        .checked_mul(52)?
        .checked_add(nl.checked_mul(20)?)?
        .checked_add(nr.checked_mul(24)?)?
        .checked_add(na.checked_mul(16)?)?
        .checked_add(nc.checked_mul(4)?)?;
    if need > body.len() - r.p {
        return None;
    }
    img.blocks.reserve(nb);
    for _ in 0..nb {
        let mut v = [0u32; 13];
        for x in v.iter_mut() {
            *x = r.u32()?;
        }
        img.blocks.push(BlockEnt {
            ip: key.image_base.wrapping_add(v[0] as u64),
            code_off: v[1],
            cold_off: v[2],
            words: v[3],
            main: v[4],
            guest: v[5],
            host_entry: v[6] != 0,
            link_lo: v[7],
            link_n: v[8],
            reloc_lo: v[9],
            reloc_n: v[10],
            assume_lo: v[11],
            assume_n: v[12],
        });
    }
    img.links.reserve(nl);
    for _ in 0..nl {
        img.links.push(LinkEnt {
            site_off: r.u32()?,
            fallback_off: r.u32()?,
            cond_off: r.u32()?,
            target: r.u64()?,
        });
    }
    img.relocs.reserve(nr);
    for _ in 0..nr {
        let (slot_off, fl, va, src) = (r.u32()?, r.u32()?, r.u64()?, r.u64()?);
        if fl & !0x3FF != 0 {
            return None;
        }
        img.relocs.push(RelocEnt {
            slot_off,
            kind: (fl >> 8) as u8,
            bail: fl & 1 != 0,
            va,
            src,
        });
    }
    img.assume.reserve(na);
    for _ in 0..na {
        img.assume.push((r.u64()?, r.u64()? != 0));
    }
    img.code.reserve(nc);
    for _ in 0..nc {
        img.code.push(r.u32()?);
    }
    if r.p != body.len() || !img.valid() {
        return None;
    }
    Some((name, key, img, flags))
}

#[cfg(test)]
pub(in crate::emu) fn encode_for_test(name: &str, key: &AotKey, img: &Image) -> Vec<u8> {
    encode(name, key, img, 0)
}

pub(super) fn write_atomic(path: &Path, data: &[u8]) -> std::io::Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let tmp = path.with_extension(format!("tmp{}", std::process::id()));
    std::fs::write(&tmp, data)?;
    std::fs::rename(&tmp, path)
}

/// Fast header-only read of `(module_name, AotKey)` from the first bytes of a `.aot` cache
/// file (no full-file read or checksum verification; full integrity checks still run on load).
fn peek_cache_header(path: &Path) -> Option<(String, AotKey)> {
    use std::io::Read;
    let mut f = std::fs::File::open(path).ok()?;
    let mut buf = [0u8; 320];
    let n = f.read(&mut buf).ok()?;
    let mut r = Rd { b: &buf[..n], p: 0 };
    if r.take(8)? != MAGIC {
        return None;
    }
    let version = r.u32()?;
    if !(FORMAT_VERSION_V5..=FORMAT_VERSION).contains(&version) {
        return None;
    }
    let _flags = r.u32()?;
    let mut k = [0u64; 6];
    for v in k.iter_mut() {
        *v = r.u64()?;
    }
    let key = AotKey {
        image_hash: [k[0], k[1]],
        image_base: k[2],
        image_size: k[3],
        codegen_hash: k[4],
        knobs: k[5],
    };
    let nlen = r.u32()? as usize;
    let name = String::from_utf8(r.take(nlen)?.to_vec()).ok()?;
    Some((name, key))
}

pub(super) fn read_cached_preferred_base_in(
    primary_dir: &Path,
    fallback_dirs: &[PathBuf],
    name: &str,
    image_size: usize,
    want_codegen_hash: u64,
) -> Option<usize> {
    if image_size == 0 || name.is_empty() {
        return None;
    }
    let prefix = format!("{name}-");
    let dirs = core::iter::once(primary_dir).chain(fallback_dirs.iter().map(PathBuf::as_path));
    for dir in dirs {
        let Ok(rd) = std::fs::read_dir(dir) else {
            continue;
        };
        let mut best: Option<(std::time::SystemTime, usize)> = None;
        for entry in rd.flatten() {
            let fname = entry.file_name();
            let Some(s) = fname.to_str() else {
                continue;
            };
            if !s.starts_with(&prefix) || !s.ends_with(".aot") {
                continue;
            }
            let p = entry.path();
            let Some((mod_name, key)) = peek_cache_header(&p) else {
                continue;
            };
            if mod_name == name
                && key.image_size == image_size as u64
                && key.codegen_hash == want_codegen_hash
                && key.image_base != 0
            {
                let mtime = entry
                    .metadata()
                    .and_then(|m| m.modified())
                    .unwrap_or(std::time::UNIX_EPOCH);
                if best.is_none_or(|(prev_t, _)| mtime > prev_t) {
                    best = Some((mtime, key.image_base as usize));
                }
            }
        }
        if let Some((_, base)) = best {
            return Some(base);
        }
    }
    None
}

/// If a valid dynamic `.aot` cache exists for PE image `name` matching `image_size` and the
/// current codegen version, return the `image_base` it was compiled for so the PE loader can
/// prefer mapping the module at that exact base (avoiding ASLR-induced AOT cache misses).
///
/// Side-effect free: never initializes `aot_mode()` or creates crash-guard session locks.
pub fn aot_cached_preferred_base(name: &str, image_size: usize) -> Option<usize> {
    if super::disk_aot_disabled() || building() || image_size == 0 {
        return None;
    }
    if env_var_any(&["PORT_AOT", "AW2_DLL_AOT"]).is_some_and(|v| v.trim() == "0") {
        return None;
    }
    let primary = cache_dir();
    let fallbacks = fallback_cache_dirs_for(&primary);
    read_cached_preferred_base_in(&primary, &fallbacks, name, image_size, codegen_hash())
}

pub(super) fn read_cache_file_in(
    primary_dir: &Path,
    fallback_dirs: &[PathBuf],
    name: &str,
    key: &AotKey,
) -> (PathBuf, Option<(Vec<u8>, bool)>) {
    let fname = format!("{name}-{:016x}.aot", key.id());
    let primary = primary_dir.join(&fname);
    if let Ok(bytes) = std::fs::read(&primary) {
        return (primary, Some((bytes, false)));
    }
    for fb_dir in fallback_dirs {
        let fb = fb_dir.join(&fname);
        if let Ok(bytes) = std::fs::read(&fb) {
            log::info!(
                "emu aot: {name}: loading cache from read-only fallback {}",
                fb.display()
            );
            return (fb, Some((bytes, true)));
        }
    }
    (primary, None)
}

/// Read an existing cache file for `(name, key)` from writable [`cache_dir`], falling back
/// read-only to [`super::fallback_cache_dirs`] (`PORT_AOT_FALLBACK_DIR`, bundled
/// `Contents/Resources/AotCache`, and legacy `~/Library/Caches/aw2-port/aot`).
pub(super) fn read_cache_file(name: &str, key: &AotKey) -> (PathBuf, Option<(Vec<u8>, bool)>) {
    if super::disk_aot_disabled() {
        return (PathBuf::new(), None);
    }
    let primary_dir = cache_dir();
    let fallbacks = fallback_cache_dirs_for(&primary_dir);
    read_cache_file_in(&primary_dir, &fallbacks, name, key)
}
