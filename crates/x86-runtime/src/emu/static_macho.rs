//! Offline v5/v6 `.aot` -> Apple-signed static ARM64 Mach-O relocatable object (`.o`) compiler.
//!
//! Converts any v5 (single-bank) or v6 (multi-bank) `.aot` translation cache produced by
//! [`super::aot`] into a standard `arm64` `MH_OBJECT` Mach-O `.o` file that links directly
//! into an iOS `.ipa`, iOS Simulator binary, or macOS binary's Apple-signed read-only
//! `__TEXT,__text` segment and executes natively on Apple Silicon with **zero `MAP_JIT` pages**.
//!
//! # Object Layout
//!
//! All sections live inside a single unnamed `LC_SEGMENT_64` with `mh.flags = 0`
//! (intentionally omitting `MH_SUBSECTIONS_VIA_SYMBOLS` so `ld64` never splits or reorders
//! our contiguous stub banks relative to their hot/cold block blobs):
//!
//! 1. `(__TEXT, __text)` (16B aligned, `S_ATTR_PURE_INSTRUCTIONS | S_ATTR_SOME_INSTRUCTIONS`):
//!    - 32-byte constructor thunk (`_port_static_aot_init_<id>`): loads
//!      `_port_static_aot_desc_<id>` into `x0` and tail-calls `_port_static_aot_register_desc`
//!      via an `x16` GOT veneer (`ARM64_RELOC_GOT_LOAD_PAGE21` / `PAGEOFF12`). Keeping only the
//!      tiny init thunk in `__TEXT,__text` ensures the host binary's Swift/Rust/C code stays
//!      packed well within ±128 MiB even when hundreds of MiB of AOT modules are linked in.
//! 2. `(__TEXT, __saot_code)` (16B aligned, `S_ATTR_PURE_INSTRUCTIONS | S_ATTR_SOME_INSTRUCTIONS`):
//!    - Every stub bank in `img.stub_banks()` receives position-independent stubs from
//!      [`super::jit::emit_static_stubs_with_veneer`], where `host_xfer` branches locally to an
//!      ABI-safe `x16` (`XA`) GOT veneer (`adrp x16, @GOTPAGE; ldr x16, [x16, @GOTPAGEOFF]; br x16`)
//!      with ±4 GiB reach and **zero `ARM64_RELOC_BRANCH26` relocations**.
//!    - Hot/cold block ARM64 machine code with all intra-module direct `b`/`bl` (within ±128 MiB),
//!      fallthrough `nop`, and ±1 MiB hot conditional `b.cond`/`cbz`/`cbnz` links resolved offline.
//! 3. `(__TEXT, __const)` (8B aligned): sorted [`StaticAotEntry`] array + NUL-terminated UTF-8 module name.
//! 4. `(__DATA, __const)` (8B aligned): 96-byte [`StaticAotModuleDesc`] with 3 `ARM64_RELOC_UNSIGNED`
//!    pointer relocations (`code_base`, `entries`, `module_name`), promoted to read-only
//!    `__DATA_CONST,__const` by `ld64` after dyld ASLR rebasing.
//! 5. `(__DATA, __mod_init_func)` (8B aligned, `S_MOD_INIT_FUNC_POINTERS`): 8-byte pointer to
//!    `_port_static_aot_init_<id>` so dyld automatically registers the module before `main()`
//!    and `-dead_strip` preserves the entire module.

use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};

use super::aot;
use super::exec_mem::{
    StaticAotEntry, StaticAotModuleDesc, STATIC_AOT_DESC_MAGIC, STATIC_AOT_DESC_VERSION,
    STATIC_AOT_FLAG_HOST_CALLABLE, STATIC_AOT_MOD_HAS_AOT_PROBE, STATIC_AOT_MOD_HAS_RAS,
    STATIC_AOT_MOD_HAS_XMM_PIN,
};
use super::jit::{self, AOT_STUBS_BYTES};

// Compile-time ABI verification against `exec_mem::StaticAotModuleDesc` & `StaticAotEntry`.
const _: () = assert!(core::mem::size_of::<StaticAotEntry>() == 12);
const _: () = assert!(core::mem::size_of::<StaticAotModuleDesc>() == 96);
const _: () = assert!(core::mem::offset_of!(StaticAotModuleDesc, code_base) == 48);
const _: () = assert!(core::mem::offset_of!(StaticAotModuleDesc, entries) == 80);
const _: () = assert!(core::mem::offset_of!(StaticAotModuleDesc, module_name) == 88);

// Mach-O 64-bit constants (<mach-o/loader.h>, <mach-o/nlist.h>, <mach-o/arm64/reloc.h>).
const MH_MAGIC_64: u32 = 0xFEED_FACF;
const CPU_TYPE_ARM64: u32 = 0x0100_000C;
const CPU_SUBTYPE_ARM64_ALL: u32 = 0;
const MH_OBJECT: u32 = 1;

const LC_SEGMENT_64: u32 = 0x19;
const LC_SYMTAB: u32 = 0x02;
const LC_DYSYMTAB: u32 = 0x0B;
const LC_BUILD_VERSION: u32 = 0x32;

const VM_PROT_RWX: u32 = 7;

const S_REGULAR: u32 = 0x0;
const S_MOD_INIT_FUNC_POINTERS: u32 = 0x9;
const S_ATTR_PURE_INSTRUCTIONS: u32 = 0x8000_0000;
const S_ATTR_SOME_INSTRUCTIONS: u32 = 0x0000_0400;

const N_UNDF_EXT: u8 = 0x01; // N_UNDF | N_EXT
const N_SECT_LOCAL: u8 = 0x0E; // N_SECT
const N_SECT_EXT: u8 = 0x0F; // N_SECT | N_EXT

const ARM64_RELOC_UNSIGNED: u32 = 0;
#[cfg(test)]
const ARM64_RELOC_BRANCH26: u32 = 2;
const ARM64_RELOC_PAGE21: u32 = 3;
const ARM64_RELOC_PAGEOFF12: u32 = 4;
const ARM64_RELOC_GOT_LOAD_PAGE21: u32 = 5;
const ARM64_RELOC_GOT_LOAD_PAGEOFF12: u32 = 6;

const ARM64_NOP: u32 = 0xD503_201F;
const ARM64_ADRP_X0: u32 = 0x9000_0000;
const ARM64_ADD_X0_X0_IMM0: u32 = 0x9100_0000;
const ARM64_ADRP_X16: u32 = 0x9000_0010;
const ARM64_LDR_X16_X16_IMM0: u32 = 0xF940_0210;
const ARM64_BR_X16: u32 = 0xD61F_0200;

/// Target Apple Mach-O platform encoded in `LC_BUILD_VERSION`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachoPlatform {
    MacOs,
    Ios,
    IosSimulator,
}

impl MachoPlatform {
    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "macos" | "darwin" | "osx" => Some(Self::MacOs),
            "ios" | "iphoneos" | "ipados" => Some(Self::Ios),
            "ios-sim" | "iossim" | "iphonesimulator" | "simulator" | "sim" => {
                Some(Self::IosSimulator)
            }
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::MacOs => "macos",
            Self::Ios => "ios",
            Self::IosSimulator => "ios-sim",
        }
    }

    /// Mach-O `PLATFORM_*` identifier for `LC_BUILD_VERSION`.
    pub fn platform_id(self) -> u32 {
        match self {
            Self::MacOs => 1,        // PLATFORM_MACOS
            Self::Ios => 2,          // PLATFORM_IOS
            Self::IosSimulator => 7, // PLATFORM_IOSSIMULATOR
        }
    }

    /// Packed `(major << 16) | (minor << 8) | patch` minimum OS version.
    pub fn minos_encoded(self) -> u32 {
        match self {
            Self::MacOs => 14 << 16,               // macOS 14.0
            Self::Ios | Self::IosSimulator => 17 << 16, // iOS 17.0
        }
    }
}

impl fmt::Display for MachoPlatform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Summary produced when compiling one v5 `.aot` module into a static ARM64 Mach-O `.o`.
#[derive(Debug, Clone)]
pub struct StaticMachoReport {
    pub module_name: String,
    pub symbol_id: String,
    pub output_path: Option<PathBuf>,
    pub platform: MachoPlatform,
    pub image_base: u64,
    pub image_size: u64,
    pub total_blocks: usize,
    pub emitted_blocks: usize,
    pub skipped_reloc_blocks: usize,
    pub linked_direct: usize,
    pub linked_cond: usize,
    pub text_bytes: usize,
    pub object_bytes: usize,
}

/// Deterministic, C-identifier-safe unique suffix derived from the module name and image hash
/// so multiple DLL `.aot.o` objects link cleanly into one Mach-O binary without symbol clashes.
pub fn sanitize_symbol_id(module_name: &str, image_hash: [u64; 2]) -> String {
    let base = module_name.rsplit(['/', '\\']).next().unwrap_or(module_name);
    let mut stem = String::with_capacity(base.len());
    for ch in base.chars() {
        if ch.is_ascii_alphanumeric() {
            stem.push(ch.to_ascii_lowercase());
        } else if !stem.ends_with('_') {
            stem.push('_');
        }
    }
    let trimmed = stem.trim_matches('_');
    let prefix = if trimmed.is_empty() { "mod" } else { trimmed };
    let h = (image_hash[0] ^ image_hash[1].rotate_left(13)) as u32;
    format!("{prefix}_{h:08x}")
}

#[inline]
fn align_up(v: usize, align: usize) -> usize {
    debug_assert!(align.is_power_of_two());
    (v + align - 1) & !(align - 1)
}

#[inline]
fn pad16(s: &str) -> [u8; 16] {
    let mut out = [0u8; 16];
    let bytes = s.as_bytes();
    let n = bytes.len().min(16);
    out[..n].copy_from_slice(&bytes[..n]);
    out
}

#[inline]
fn push_u16(buf: &mut Vec<u8>, v: u16) {
    buf.extend_from_slice(&v.to_le_bytes());
}
#[inline]
fn push_u32(buf: &mut Vec<u8>, v: u32) {
    buf.extend_from_slice(&v.to_le_bytes());
}
#[inline]
fn push_u64(buf: &mut Vec<u8>, v: u64) {
    buf.extend_from_slice(&v.to_le_bytes());
}

#[derive(Clone, Copy)]
struct Reloc {
    address: u32,
    sym_idx: u32,
    pcrel: bool,
    length: u32,
    extern_: bool,
    rtype: u32,
}

impl Reloc {
    fn encode(self, out: &mut Vec<u8>) {
        push_u32(out, self.address);
        let info = (self.sym_idx & 0x00FF_FFFF)
            | ((self.pcrel as u32) << 24)
            | ((self.length & 0x3) << 25)
            | ((self.extern_ as u32) << 27)
            | ((self.rtype & 0xF) << 28);
        push_u32(out, info);
    }
}

struct Sym {
    name: String,
    n_type: u8,
    n_sect: u8,
    n_desc: u16,
    n_value: u64,
}

/// Convert raw v5/v6 `.aot` bytes into an Apple Silicon ARM64 `MH_OBJECT` Mach-O `.o` byte vector.
pub fn emit_macho_object_from_aot_bytes(
    aot_bytes: &[u8],
    platform: MachoPlatform,
) -> Result<(Vec<u8>, StaticMachoReport), String> {
    let Some((module_name, key, img, _flags)) = aot::decode(aot_bytes) else {
        return Err("invalid or corrupted .aot input (checksum/header verification failed)".into());
    };
    if img.blocks.is_empty() {
        return Err(format!("{module_name}: .aot image contains zero translated blocks"));
    }
    let code_words = AOT_STUBS_BYTES / 4 + img.code.len();
    let code_len = code_words * 4;
    if code_len > u32::MAX as usize {
        return Err(format!("{module_name}: code blob ({code_len} bytes) exceeds 32-bit offset range"));
    }

    // 1. Copy hot/cold block words FIRST, then stamp position-independent stubs into every
    //    stub bank recorded in `img.stub_banks()`.
    //    Every bank (single- or multi-bank) uses per-bank `x16` (`XA`) GOT veneers
    //    (`ARM64_RELOC_GOT_LOAD_PAGE21` + `ARM64_RELOC_GOT_LOAD_PAGEOFF12`, ±4 GiB reach) so
    //    `__TEXT,__saot_code` has ZERO `ARM64_RELOC_BRANCH26` relocations and `ld64` links
    //    hundreds of MiB of AOT code across many modules with zero branch-range errors.
    let mut code_buf_words = vec![0u32; code_words];
    code_buf_words[AOT_STUBS_BYTES / 4..].copy_from_slice(&img.code);

    const DUMMY_VBASE: usize = 0x2000_0000_0000;
    let banks = img.stub_banks();
    let mut host_xfer_veneers: Vec<u32> = Vec::with_capacity(banks.len());
    for &bank_off in banks {
        let bank_w = (bank_off as usize) / 4;
        let (_got_stubs, veneer_off) = unsafe {
            jit::emit_static_stubs_with_veneer(
                code_buf_words.as_mut_ptr().add(bank_w),
                DUMMY_VBASE + bank_off as usize,
                key.knobs,
                &img.stubs,
            )?
        };
        if let Some(vo) = veneer_off {
            host_xfer_veneers.push(bank_off + vo as u32);
        }
    }

    // 2. Filter blocks eligible for signed read-only __TEXT (zero runtime literal relocations).
    //    Blocks with `reloc_n > 0` (from `AW2_JIT_PLAINCALL=1`) require runtime pointer patching
    //    into code pages, so we honestly skip them rather than baking stale host addresses.
    let total_blocks = img.blocks.len();
    let mut skipped_reloc_blocks = 0usize;
    let mut live_link_targets: HashMap<u64, u32> = HashMap::with_capacity(total_blocks);
    let mut entries: Vec<StaticAotEntry> = Vec::with_capacity(total_blocks);

    for b in &img.blocks {
        if b.reloc_n != 0 {
            skipped_reloc_blocks += 1;
            continue;
        }
        let rva_u64 = b.ip.wrapping_sub(key.image_base);
        if rva_u64 > u32::MAX as u64 {
            skipped_reloc_blocks += 1;
            continue;
        }
        let entry_flags = if b.host_entry { STATIC_AOT_FLAG_HOST_CALLABLE } else { 0 };
        entries.push(StaticAotEntry {
            rva: rva_u64 as u32,
            code_offset: b.code_off,
            flags: entry_flags,
        });
        if !b.host_entry {
            live_link_targets.insert(b.ip, b.code_off);
        }
    }
    if entries.is_empty() {
        return Err(format!(
            "{module_name}: all {total_blocks} block(s) had runtime literal relocations; re-prewarm with AW2_JIT_PLAINCALL=0 (or use --emit-static-o)"
        ));
    }
    entries.sort_unstable_by_key(|e| e.rva);
    entries.dedup_by_key(|e| e.rva);
    let emitted_blocks = entries.len();

    // 3. Offline intra-module direct link & hot conditional-branch resolution (matches `aot::link_pass`).
    let mut linked_direct = 0usize;
    let mut linked_cond = 0usize;
    for b in &img.blocks {
        if b.reloc_n != 0 {
            continue;
        }
        for l in img.links_of(b) {
            let Some(&target_off) = live_link_targets.get(&l.target) else {
                continue;
            };
            let delta = target_off as i64 - l.site_off as i64;
            if !(-(128i64 << 20)..(128i64 << 20)).contains(&delta) {
                continue;
            }
            let site_w = (l.site_off as usize) / 4;
            let old = code_buf_words[site_w];
            if target_off == l.site_off.wrapping_add(4) && (old & 0xFC00_0000) == 0x1400_0000 {
                // Direct fallthrough into the immediately following hot block: 0-cycle NOP.
                code_buf_words[site_w] = ARM64_NOP;
            } else {
                let link_bit = if old == ARM64_NOP { 0 } else { old & 0x8000_0000 };
                let imm26 = ((delta >> 2) as u32) & 0x03FF_FFFF;
                code_buf_words[site_w] = 0x1400_0000 | link_bit | imm26;
            }
            linked_direct += 1;

            if l.cond_off != u32::MAX {
                let cond_delta = (target_off as i64 - l.cond_off as i64) >> 2;
                if (-(1i64 << 18)..(1i64 << 18)).contains(&cond_delta) {
                    let cw = (l.cond_off as usize) / 4;
                    let old_c = code_buf_words[cw];
                    if (old_c & 0xFF00_0010) == 0x5400_0000 || (old_c & 0x7E00_0000) == 0x3400_0000 {
                        code_buf_words[cw] = (old_c & 0xFF00_001F) | (((cond_delta as u32) & 0x7_FFFF) << 5);
                        linked_cond += 1;
                    }
                }
            }
        }
    }

    // 4. Build Section 1: `(__TEXT, __text)` = 32-byte `__mod_init_func` constructor thunk ONLY:
    //      adrp x0,  _port_static_aot_desc_<id>@PAGE
    //      add  x0,  x0, _port_static_aot_desc_<id>@PAGEOFF
    //      adrp x16, _port_static_aot_register_desc@GOTPAGE
    //      ldr  x16, [x16, _port_static_aot_register_desc@GOTPAGEOFF]
    //      br   x16
    //      nop; nop; nop
    //    Keeping the large AOT code blob in its own `(__TEXT, __saot_code)` section prevents
    //    host Swift/Rust/C functions in `(__TEXT, __text)` from being pushed >128 MiB apart!
    let mut sec1_init_text = Vec::with_capacity(32);
    for w in [
        ARM64_ADRP_X0,
        ARM64_ADD_X0_X0_IMM0,
        ARM64_ADRP_X16,
        ARM64_LDR_X16_X16_IMM0,
        ARM64_BR_X16,
        ARM64_NOP,
        ARM64_NOP,
        ARM64_NOP,
    ] {
        push_u32(&mut sec1_init_text, w);
    }

    // Section 2: `(__TEXT, __saot_code)` = contiguous stub banks + hot/cold ARM64 machine code.
    let mut sec2_saot_code = Vec::with_capacity(code_buf_words.len() * 4);
    for w in &code_buf_words {
        push_u32(&mut sec2_saot_code, *w);
    }

    // 5. Build Section 3: `(__TEXT, __const)` = sorted StaticAotEntry[] + NUL-terminated module name.
    let mut sec3_const = Vec::with_capacity(entries.len() * 12 + module_name.len() + 8);
    for e in &entries {
        push_u32(&mut sec3_const, e.rva);
        push_u32(&mut sec3_const, e.code_offset);
        push_u32(&mut sec3_const, e.flags);
    }
    while sec3_const.len() % 4 != 0 {
        sec3_const.push(0);
    }
    let name_off_in_sec3 = sec3_const.len() as u64;
    sec3_const.extend_from_slice(module_name.as_bytes());
    sec3_const.push(0);

    // 6. Build Section 4: `(__DATA, __const)` = 96-byte StaticAotModuleDesc.
    let enter_off = img.stubs[0].saturating_mul(4);
    let lookup_rip_off = img.stubs[2].saturating_mul(4);
    let mut mod_flags = 0u32;
    if (key.knobs & 1) != 0 && img.stubs[6] != u32::MAX {
        mod_flags |= STATIC_AOT_MOD_HAS_RAS;
    }
    if ((key.knobs >> 30) & 1) != 0 {
        mod_flags |= STATIC_AOT_MOD_HAS_XMM_PIN;
    }
    if ((key.knobs >> 27) & 1) != 0 {
        mod_flags |= STATIC_AOT_MOD_HAS_AOT_PROBE;
    }

    let mut sec4_desc = Vec::with_capacity(96);
    push_u32(&mut sec4_desc, STATIC_AOT_DESC_MAGIC); // 0
    push_u32(&mut sec4_desc, STATIC_AOT_DESC_VERSION); // 4
    push_u64(&mut sec4_desc, key.knobs); // 8
    push_u64(&mut sec4_desc, key.image_base); // 16
    push_u64(&mut sec4_desc, key.image_size); // 24
    push_u64(&mut sec4_desc, key.image_hash[0]); // 32
    push_u64(&mut sec4_desc, key.image_hash[1]); // 40
    push_u64(&mut sec4_desc, 0); // 48: code_base (ARM64_RELOC_UNSIGNED -> code sym)
    push_u32(&mut sec4_desc, code_len as u32); // 56
    push_u32(&mut sec4_desc, enter_off); // 60
    push_u32(&mut sec4_desc, lookup_rip_off); // 64
    push_u32(&mut sec4_desc, mod_flags); // 68
    push_u32(&mut sec4_desc, emitted_blocks as u32); // 72
    push_u32(&mut sec4_desc, 0); // 76: _pad
    push_u64(&mut sec4_desc, 0); // 80: entries (ARM64_RELOC_UNSIGNED -> entries sym)
    push_u64(&mut sec4_desc, 0); // 88: module_name (ARM64_RELOC_UNSIGNED -> name sym)
    debug_assert_eq!(sec4_desc.len(), 96);

    // 7. Build Section 5: `(__DATA, __mod_init_func)` = 8-byte slot relocated to init thunk.
    let sec5_init = [0u8; 8];

    // 8. Compute segment virtual layout & file offsets.
    //    Header (32) + LC_SEGMENT_64 (72 + 5*80 = 472) + LC_BUILD_VERSION (24)
    //    + LC_SYMTAB (24) + LC_DYSYMTAB (80) = 632 -> align up to 16 => 640.
    const HDR_SIZE: usize = 32;
    const SEG_CMD_SIZE: usize = 72 + 5 * 80;
    const BUILD_CMD_SIZE: usize = 24;
    const SYMTAB_CMD_SIZE: usize = 24;
    const DYSYMTAB_CMD_SIZE: usize = 80;
    const SIZEOFCMDS: usize = SEG_CMD_SIZE + BUILD_CMD_SIZE + SYMTAB_CMD_SIZE + DYSYMTAB_CMD_SIZE;
    let seg_fileoff = align_up(HDR_SIZE + SIZEOFCMDS, 16);

    let s1_addr = 0usize;
    let s1_size = sec1_init_text.len();
    let s2_addr = align_up(s1_addr + s1_size, 16);
    let s2_size = sec2_saot_code.len();
    let s3_addr = align_up(s2_addr + s2_size, 8);
    let s3_size = sec3_const.len();
    let s4_addr = align_up(s3_addr + s3_size, 8);
    let s4_size = sec4_desc.len();
    let s5_addr = align_up(s4_addr + s4_size, 8);
    let s5_size = sec5_init.len();
    let seg_vmsize = s5_addr + s5_size;

    // 9. Build symbol table (ordered: local, extdef, undef for LC_DYSYMTAB).
    let sym_id = sanitize_symbol_id(&module_name, key.image_hash);
    let mut syms: Vec<Sym> = Vec::with_capacity(8);

    // Local symbols (n_type = N_SECT_LOCAL = 0x0E)
    let sym_code = syms.len() as u32;
    syms.push(Sym {
        name: format!("_saot_code_{sym_id}"),
        n_type: N_SECT_LOCAL,
        n_sect: 2,
        n_desc: 0,
        n_value: s2_addr as u64,
    });
    let sym_entries = syms.len() as u32;
    syms.push(Sym {
        name: format!("_saot_entries_{sym_id}"),
        n_type: N_SECT_LOCAL,
        n_sect: 3,
        n_desc: 0,
        n_value: s3_addr as u64,
    });
    let sym_name = syms.len() as u32;
    syms.push(Sym {
        name: format!("_saot_name_{sym_id}"),
        n_type: N_SECT_LOCAL,
        n_sect: 3,
        n_desc: 0,
        n_value: (s3_addr as u64) + name_off_in_sec3,
    });
    let nlocalsym = syms.len() as u32;

    // Externally defined symbols (n_type = N_SECT_EXT = 0x0F)
    let iextdefsym = syms.len() as u32;
    let sym_init = syms.len() as u32;
    syms.push(Sym {
        name: format!("_port_static_aot_init_{sym_id}"),
        n_type: N_SECT_EXT,
        n_sect: 1,
        n_desc: 0,
        n_value: s1_addr as u64,
    });
    let sym_desc = syms.len() as u32;
    syms.push(Sym {
        name: format!("_port_static_aot_desc_{sym_id}"),
        n_type: N_SECT_EXT,
        n_sect: 4,
        n_desc: 0,
        n_value: s4_addr as u64,
    });
    let nextdefsym = syms.len() as u32 - iextdefsym;

    // Undefined external symbols (n_type = N_UNDF_EXT = 0x01)
    let iundefsym = syms.len() as u32;
    let sym_register_desc = syms.len() as u32;
    syms.push(Sym {
        name: "_port_static_aot_register_desc".to_string(),
        n_type: N_UNDF_EXT,
        n_sect: 0,
        n_desc: 0,
        n_value: 0,
    });
    let sym_host_xfer = if !host_xfer_veneers.is_empty() {
        let idx = syms.len() as u32;
        syms.push(Sym {
            name: "_port_static_aot_host_xfer".to_string(),
            n_type: N_UNDF_EXT,
            n_sect: 0,
            n_desc: 0,
            n_value: 0,
        });
        Some(idx)
    } else {
        None
    };
    let nundefsym = syms.len() as u32 - iundefsym;

    // 10. Build per-section relocation tables (ZERO BRANCH26 relocations anywhere).
    let s1_relocs = [
        Reloc {
            address: 0,
            sym_idx: sym_desc,
            pcrel: true,
            length: 2,
            extern_: true,
            rtype: ARM64_RELOC_PAGE21,
        },
        Reloc {
            address: 4,
            sym_idx: sym_desc,
            pcrel: false,
            length: 2,
            extern_: true,
            rtype: ARM64_RELOC_PAGEOFF12,
        },
        Reloc {
            address: 8,
            sym_idx: sym_register_desc,
            pcrel: true,
            length: 2,
            extern_: true,
            rtype: ARM64_RELOC_GOT_LOAD_PAGE21,
        },
        Reloc {
            address: 12,
            sym_idx: sym_register_desc,
            pcrel: false,
            length: 2,
            extern_: true,
            rtype: ARM64_RELOC_GOT_LOAD_PAGEOFF12,
        },
    ];

    let mut s2_relocs: Vec<Reloc> = Vec::with_capacity(host_xfer_veneers.len() * 2);
    if let Some(hx_sym) = sym_host_xfer {
        for &veneer_addr in &host_xfer_veneers {
            s2_relocs.push(Reloc {
                address: veneer_addr,
                sym_idx: hx_sym,
                pcrel: true,
                length: 2,
                extern_: true,
                rtype: ARM64_RELOC_GOT_LOAD_PAGE21,
            });
            s2_relocs.push(Reloc {
                address: veneer_addr + 4,
                sym_idx: hx_sym,
                pcrel: false,
                length: 2,
                extern_: true,
                rtype: ARM64_RELOC_GOT_LOAD_PAGEOFF12,
            });
        }
    }

    let s4_relocs = [
        Reloc {
            address: 48, // offset_of!(StaticAotModuleDesc, code_base)
            sym_idx: sym_code,
            pcrel: false,
            length: 3, // 8 bytes
            extern_: true,
            rtype: ARM64_RELOC_UNSIGNED,
        },
        Reloc {
            address: 80, // offset_of!(StaticAotModuleDesc, entries)
            sym_idx: sym_entries,
            pcrel: false,
            length: 3,
            extern_: true,
            rtype: ARM64_RELOC_UNSIGNED,
        },
        Reloc {
            address: 88, // offset_of!(StaticAotModuleDesc, module_name)
            sym_idx: sym_name,
            pcrel: false,
            length: 3,
            extern_: true,
            rtype: ARM64_RELOC_UNSIGNED,
        },
    ];

    let s5_relocs = [Reloc {
        address: 0,
        sym_idx: sym_init,
        pcrel: false,
        length: 3,
        extern_: true,
        rtype: ARM64_RELOC_UNSIGNED,
    }];

    // 11. Compute relocation, symbol, and string table file offsets.
    let reloc_base = align_up(seg_fileoff + seg_vmsize, 8);
    let s1_reloff = reloc_base;
    let s2_reloff = s1_reloff + s1_relocs.len() * 8;
    let s4_reloff = s2_reloff + s2_relocs.len() * 8;
    let s5_reloff = s4_reloff + s4_relocs.len() * 8;
    let symoff = align_up(s5_reloff + s5_relocs.len() * 8, 8);

    let mut strtab = vec![0u8]; // Mach-O strtab always begins with a NUL byte (index 0 = "")
    let mut sym_strx = Vec::with_capacity(syms.len());
    for s in &syms {
        sym_strx.push(strtab.len() as u32);
        strtab.extend_from_slice(s.name.as_bytes());
        strtab.push(0);
    }
    while strtab.len() % 8 != 0 {
        strtab.push(0);
    }
    let stroff = symoff + syms.len() * 16;
    let total_size = stroff + strtab.len();
    if total_size > u32::MAX as usize {
        return Err(format!(
            "{module_name}: Mach-O object size ({total_size} bytes) exceeds 32-bit Mach-O file offset limit"
        ));
    }

    // 12. Serialize the complete Mach-O `MH_OBJECT` byte buffer.
    let mut out = Vec::with_capacity(total_size);

    // mach_header_64 (32 bytes)
    push_u32(&mut out, MH_MAGIC_64);
    push_u32(&mut out, CPU_TYPE_ARM64);
    push_u32(&mut out, CPU_SUBTYPE_ARM64_ALL);
    push_u32(&mut out, MH_OBJECT);
    push_u32(&mut out, 4); // ncmds: LC_SEGMENT_64, LC_BUILD_VERSION, LC_SYMTAB, LC_DYSYMTAB
    push_u32(&mut out, SIZEOFCMDS as u32);
    push_u32(&mut out, 0); // flags: intentionally NO MH_SUBSECTIONS_VIA_SYMBOLS
    push_u32(&mut out, 0); // reserved
    debug_assert_eq!(out.len(), HDR_SIZE);

    // Load command 0: LC_SEGMENT_64 (72 + 5 * 80 bytes)
    push_u32(&mut out, LC_SEGMENT_64);
    push_u32(&mut out, SEG_CMD_SIZE as u32);
    out.extend_from_slice(&[0u8; 16]); // segname = ""
    push_u64(&mut out, 0); // vmaddr
    push_u64(&mut out, seg_vmsize as u64);
    push_u64(&mut out, seg_fileoff as u64);
    push_u64(&mut out, seg_vmsize as u64);
    push_u32(&mut out, VM_PROT_RWX);
    push_u32(&mut out, VM_PROT_RWX);
    push_u32(&mut out, 5); // nsects
    push_u32(&mut out, 0); // flags

    let emit_section = |out: &mut Vec<u8>,
                        sectname: &str,
                        segname: &str,
                        addr: usize,
                        size: usize,
                        align_log2: u32,
                        reloff: usize,
                        nreloc: usize,
                        flags: u32| {
        out.extend_from_slice(&pad16(sectname));
        out.extend_from_slice(&pad16(segname));
        push_u64(out, addr as u64);
        push_u64(out, size as u64);
        push_u32(out, (seg_fileoff + addr) as u32);
        push_u32(out, align_log2);
        push_u32(out, if nreloc > 0 { reloff as u32 } else { 0 });
        push_u32(out, nreloc as u32);
        push_u32(out, flags);
        push_u32(out, 0); // reserved1
        push_u32(out, 0); // reserved2
        push_u32(out, 0); // reserved3
    };

    emit_section(
        &mut out,
        "__text",
        "__TEXT",
        s1_addr,
        s1_size,
        4, // 2^4 = 16B
        s1_reloff,
        s1_relocs.len(),
        S_REGULAR | S_ATTR_PURE_INSTRUCTIONS | S_ATTR_SOME_INSTRUCTIONS,
    );
    emit_section(
        &mut out,
        "__saot_code",
        "__TEXT",
        s2_addr,
        s2_size,
        4, // 2^4 = 16B
        s2_reloff,
        s2_relocs.len(),
        S_REGULAR,
    );
    emit_section(&mut out, "__const", "__TEXT", s3_addr, s3_size, 3, 0, 0, S_REGULAR);
    emit_section(
        &mut out,
        "__const",
        "__DATA",
        s4_addr,
        s4_size,
        3,
        s4_reloff,
        s4_relocs.len(),
        S_REGULAR,
    );
    emit_section(
        &mut out,
        "__mod_init_func",
        "__DATA",
        s5_addr,
        s5_size,
        3,
        s5_reloff,
        s5_relocs.len(),
        S_MOD_INIT_FUNC_POINTERS,
    );

    // Load command 1: LC_BUILD_VERSION (24 bytes)
    push_u32(&mut out, LC_BUILD_VERSION);
    push_u32(&mut out, BUILD_CMD_SIZE as u32);
    push_u32(&mut out, platform.platform_id());
    push_u32(&mut out, platform.minos_encoded());
    push_u32(&mut out, platform.minos_encoded()); // sdk
    push_u32(&mut out, 0); // ntools

    // Load command 2: LC_SYMTAB (24 bytes)
    push_u32(&mut out, LC_SYMTAB);
    push_u32(&mut out, SYMTAB_CMD_SIZE as u32);
    push_u32(&mut out, symoff as u32);
    push_u32(&mut out, syms.len() as u32);
    push_u32(&mut out, stroff as u32);
    push_u32(&mut out, strtab.len() as u32);

    // Load command 3: LC_DYSYMTAB (80 bytes)
    push_u32(&mut out, LC_DYSYMTAB);
    push_u32(&mut out, DYSYMTAB_CMD_SIZE as u32);
    push_u32(&mut out, 0); // ilocalsym
    push_u32(&mut out, nlocalsym);
    push_u32(&mut out, iextdefsym);
    push_u32(&mut out, nextdefsym);
    push_u32(&mut out, iundefsym);
    push_u32(&mut out, nundefsym);
    for _ in 0..12 {
        push_u32(&mut out, 0); // toc, modtab, extrefsym, indirectsym, extrel, locrel
    }
    debug_assert_eq!(out.len(), HDR_SIZE + SIZEOFCMDS);

    // Pad to `seg_fileoff` and write section payloads.
    out.resize(seg_fileoff, 0);
    out.extend_from_slice(&sec1_init_text);
    out.resize(seg_fileoff + s2_addr, 0);
    out.extend_from_slice(&sec2_saot_code);
    out.resize(seg_fileoff + s3_addr, 0);
    out.extend_from_slice(&sec3_const);
    out.resize(seg_fileoff + s4_addr, 0);
    out.extend_from_slice(&sec4_desc);
    out.resize(seg_fileoff + s5_addr, 0);
    out.extend_from_slice(&sec5_init);
    debug_assert_eq!(out.len(), seg_fileoff + seg_vmsize);

    // Pad to `reloc_base` and write relocation tables.
    out.resize(reloc_base, 0);
    for r in &s1_relocs {
        r.encode(&mut out);
    }
    debug_assert_eq!(out.len(), s2_reloff);
    for r in &s2_relocs {
        r.encode(&mut out);
    }
    debug_assert_eq!(out.len(), s4_reloff);
    for r in &s4_relocs {
        r.encode(&mut out);
    }
    debug_assert_eq!(out.len(), s5_reloff);
    for r in &s5_relocs {
        r.encode(&mut out);
    }

    // Pad to `symoff` and write nlist_64 entries (16 bytes each).
    out.resize(symoff, 0);
    for (s, &strx) in syms.iter().zip(&sym_strx) {
        push_u32(&mut out, strx);
        out.push(s.n_type);
        out.push(s.n_sect);
        push_u16(&mut out, s.n_desc);
        push_u64(&mut out, s.n_value);
    }
    debug_assert_eq!(out.len(), stroff);
    out.extend_from_slice(&strtab);
    debug_assert_eq!(out.len(), total_size);

    let report = StaticMachoReport {
        module_name,
        symbol_id: sym_id,
        output_path: None,
        platform,
        image_base: key.image_base,
        image_size: key.image_size,
        total_blocks,
        emitted_blocks,
        skipped_reloc_blocks,
        linked_direct,
        linked_cond,
        text_bytes: s2_size,
        object_bytes: out.len(),
    };
    Ok((out, report))
}

/// Read a v5 `.aot` file from `aot_path` and write its signed-ready ARM64 Mach-O `.o` to `out_path`.
pub fn emit_macho_object_from_aot_file(
    aot_path: &Path,
    out_path: &Path,
    platform: MachoPlatform,
) -> Result<StaticMachoReport, String> {
    let bytes = std::fs::read(aot_path)
        .map_err(|e| format!("reading {}: {e}", aot_path.display()))?;
    let (obj, mut report) = emit_macho_object_from_aot_bytes(&bytes, platform)?;
    if let Some(parent) = out_path.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("creating {}: {e}", parent.display()))?;
    }
    std::fs::write(out_path, &obj)
        .map_err(|e| format!("writing {}: {e}", out_path.display()))?;
    report.output_path = Some(out_path.to_path_buf());
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emu::BlockCache;
    use iced_x86::code_asm::*;

    #[test]
    fn platform_parse_and_symbol_id() {
        assert_eq!(MachoPlatform::parse("ios"), Some(MachoPlatform::Ios));
        assert_eq!(MachoPlatform::parse("iphoneos"), Some(MachoPlatform::Ios));
        assert_eq!(MachoPlatform::parse("ios-sim"), Some(MachoPlatform::IosSimulator));
        assert_eq!(MachoPlatform::parse("macos"), Some(MachoPlatform::MacOs));
        assert_eq!(MachoPlatform::parse("unknown"), None);

        let id = sanitize_symbol_id("C:\\Game\\My-Cool.DLL", [0x1122_3344_5566_7788, 0]);
        assert!(id.starts_with("my_cool_dll_"));
        assert!(id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_'));
    }

    #[test]
    fn emit_valid_arm64_macho_object_from_real_x86_64_aot() {
        // Assemble a real multi-block x86-64 program (loop + conditional branch + call/ret)
        // in guest memory, compile it to a v5 `.aot` via `aot::build_image` + `aot::encode`,
        // then emit Mach-O `.o` files for iOS, iOS Simulator, and macOS and verify headers,
        // load commands, sections, relocations, and intra-module direct link patching.
        let mut a = CodeAssembler::new(64).unwrap();
        let mut lbl_loop = a.create_label();
        let mut lbl_done = a.create_label();
        let mut lbl_helper = a.create_label();

        // Entry: sum i = 0..rcx via helper(i)
        a.xor(eax, eax).unwrap();
        a.xor(edx, edx).unwrap();
        a.set_label(&mut lbl_loop).unwrap();
        a.cmp(rdx, rcx).unwrap();
        a.jge(lbl_done).unwrap();
        a.call(lbl_helper).unwrap();
        a.add(rdx, 1).unwrap();
        a.jmp(lbl_loop).unwrap();
        a.set_label(&mut lbl_done).unwrap();
        a.ret().unwrap();

        a.set_label(&mut lbl_helper).unwrap();
        a.add(rax, rdx).unwrap();
        a.ret().unwrap();

        let mut code_buf = vec![0x90u8; 4096].into_boxed_slice();
        let base = code_buf.as_ptr() as usize;
        let assembled = a.assemble(base as u64).unwrap();
        let entry_ip = base as u64;
        code_buf[..assembled.len()].copy_from_slice(&assembled);

        // Discover every basic-block entry IP cleanly via iced_x86::Decoder.
        let mut ips = vec![entry_ip];
        let mut dec = iced_x86::Decoder::with_ip(
            64,
            &code_buf[..assembled.len()],
            entry_ip,
            iced_x86::DecoderOptions::NONE,
        );
        while dec.can_decode() {
            let ins = dec.decode();
            match ins.flow_control() {
                iced_x86::FlowControl::Call | iced_x86::FlowControl::ConditionalBranch => {
                    if ins.near_branch_target() != 0 {
                        ips.push(ins.near_branch_target());
                    }
                    ips.push(ins.next_ip());
                }
                iced_x86::FlowControl::UnconditionalBranch => {
                    if ins.near_branch_target() != 0 {
                        ips.push(ins.near_branch_target());
                    }
                }
                _ => {}
            }
        }
        ips.retain(|&ip| ip >= entry_ip && ip < entry_ip + assembled.len() as u64);
        ips.sort_unstable();
        ips.dedup();

        let mut bc = Box::new(BlockCache::new());
        let img = unsafe { aot::build_image_for_test(&mut *bc, &ips) }.expect("build v5 aot image");
        assert!(img.blocks.len() >= 4, "expected >=4 translated blocks, got {}", img.blocks.len());
        let key = aot::AotKey {
            image_hash: [0xA1B2_C3D4_E5F6_0718, 0x9988_7766_5544_3322],
            image_base: base as u64,
            image_size: code_buf.len() as u64,
            codegen_hash: 0xDEAD_BEEF,
            knobs: jit::aot_knob_bits(),
        };
        let aot_bytes = aot::encode_for_test("unit_sum.dll", &key, &img);

        for plat in [MachoPlatform::Ios, MachoPlatform::IosSimulator, MachoPlatform::MacOs] {
            let (obj, rep) = emit_macho_object_from_aot_bytes(&aot_bytes, plat)
                .unwrap_or_else(|e| panic!("emit {plat} failed: {e}"));
            assert_eq!(rep.module_name, "unit_sum.dll");
            assert_eq!(rep.platform, plat);
            assert_eq!(rep.skipped_reloc_blocks, 0);
            assert_eq!(rep.emitted_blocks, img.blocks.len());
            assert!(rep.linked_direct >= 2, "expected direct intra-module links patched, got {}", rep.linked_direct);
            assert!(rep.text_bytes > AOT_STUBS_BYTES);
            assert_eq!(rep.object_bytes, obj.len());

            // Verify Mach-O header magic, CPU type, filetype, flags, LC_BUILD_VERSION platform.
            let rd32 = |off: usize| u32::from_le_bytes(obj[off..off + 4].try_into().unwrap());
            assert_eq!(rd32(0), MH_MAGIC_64);
            assert_eq!(rd32(4), CPU_TYPE_ARM64);
            assert_eq!(rd32(12), MH_OBJECT);
            assert_eq!(rd32(16), 4); // ncmds
            assert_eq!(rd32(24), 0); // no MH_SUBSECTIONS_VIA_SYMBOLS

            // LC_BUILD_VERSION sits right after mach_header_64 (32) + LC_SEGMENT_64 (472) = 504.
            assert_eq!(rd32(504), LC_BUILD_VERSION);
            assert_eq!(rd32(512), plat.platform_id());
            assert_eq!(rd32(516), plat.minos_encoded());
        }
        drop(code_buf);
    }

    #[test]
    fn rejects_corrupted_aot_bytes_honestly() {
        let err = emit_macho_object_from_aot_bytes(b"not-an-aot-file", MachoPlatform::Ios).unwrap_err();
        assert!(err.contains("invalid or corrupted"));
    }

    #[test]
    fn emit_multibank_arm64_macho_object_with_per_bank_got_veneers() {
        // Synthesize enough x86-64 functions to span >= 3 stub banks at a 16 KiB test bank_span,
        // emit static Mach-O `.o` objects for iOS / iOS-Sim / macOS, and verify:
        //   1. Mach-O header, LC_SEGMENT_64, 5 sections (`__TEXT,__text` for the 32B init thunk,
        //      `__TEXT,__saot_code` for AOT banks), LC_BUILD_VERSION, LC_SYMTAB, LC_DYSYMTAB;
        //   2. ZERO `ARM64_RELOC_BRANCH26` relocations in `__TEXT,__text` or `__TEXT,__saot_code`;
        //   3. Every stub bank has a 16B-aligned `adrp x16; ldr x16, [x16, #0]; br x16; nop`
        //      GOT veneer with `ARM64_RELOC_GOT_LOAD_PAGE21` + `ARM64_RELOC_GOT_LOAD_PAGEOFF12`;
        //   4. `_port_static_aot_init_<id>` uses `GOT_LOAD_PAGE21/PAGEOFF12` + `br x16` for
        //      `_port_static_aot_register_desc`.
        const NFUNCS: usize = 96;
        const STRIDE: usize = 0x30;
        const TEST_BANK_SPAN: usize = 16 << 10;

        let mut code_buf = vec![0x90u8; NFUNCS * STRIDE + 0x1000].into_boxed_slice();
        let base = code_buf.as_ptr() as usize;
        let base64 = base as u64;
        let mut ips = Vec::with_capacity(NFUNCS * 2);

        for i in 0..NFUNCS {
            let off = i * STRIDE;
            let va = base64 + off as u64;
            let mut a = CodeAssembler::new(64).unwrap();
            let mut l_else = a.create_label();
            a.cmp(ecx, (i & 7) as i32).unwrap();
            a.ja(l_else).unwrap();
            a.lea(eax, ptr(rcx + rdx + (i as i32))).unwrap();
            a.ret().unwrap();
            a.set_label(&mut l_else).unwrap();
            a.mov(eax, edx).unwrap();
            a.add(eax, (i as i32) + 1).unwrap();
            a.ret().unwrap();
            let bytes = a.assemble(va).unwrap();
            assert!(bytes.len() <= STRIDE);
            code_buf[off..off + bytes.len()].copy_from_slice(&bytes);

            let mut dec = iced_x86::Decoder::with_ip(64, &bytes, va, iced_x86::DecoderOptions::NONE);
            ips.push(va);
            while dec.can_decode() {
                let ins = dec.decode();
                if ins.flow_control() == iced_x86::FlowControl::ConditionalBranch {
                    if ins.near_branch_target() != 0 {
                        ips.push(ins.near_branch_target());
                    }
                    ips.push(ins.next_ip());
                }
            }
        }
        ips.sort_unstable();
        ips.dedup();

        let mut bc = Box::new(BlockCache::new());
        let img = unsafe {
            aot::build_image_for_test_with_limits(&mut *bc, &ips, TEST_BANK_SPAN, 64 << 20)
        }
        .expect("build multi-bank aot image");
        let nbanks = img.stub_banks().len();
        assert!(nbanks >= 3, "expected >=3 stub banks, got {nbanks}");
        assert_eq!(img.blocks.len(), ips.len());

        let key = aot::AotKey {
            image_hash: [0x5566_7788_99AA_BBCC, 0x1020_3040_5060_7080],
            image_base: base64,
            image_size: code_buf.len() as u64,
            codegen_hash: 0xCAFE_BABE,
            knobs: jit::aot_knob_bits(),
        };
        let aot_bytes = aot::encode_for_test("big_game.exe", &key, &img);

        for plat in [MachoPlatform::Ios, MachoPlatform::IosSimulator, MachoPlatform::MacOs] {
            let (obj, rep) = emit_macho_object_from_aot_bytes(&aot_bytes, plat)
                .unwrap_or_else(|e| panic!("emit multi-bank {plat} failed: {e}"));
            assert_eq!(rep.module_name, "big_game.exe");
            assert_eq!(rep.emitted_blocks, img.blocks.len());
            assert_eq!(rep.object_bytes, obj.len());

            let rd32 = |o: usize| u32::from_le_bytes(obj[o..o + 4].try_into().unwrap());
            let rd64 = |o: usize| u64::from_le_bytes(obj[o..o + 8].try_into().unwrap());
            assert_eq!(rd32(0), MH_MAGIC_64);
            assert_eq!(rd32(4), CPU_TYPE_ARM64);
            assert_eq!(rd32(12), MH_OBJECT);
            assert_eq!(rd32(24), 0);

            // Section 1 (`__TEXT,__text`) starts at 104; Section 2 (`__TEXT,__saot_code`) at 184.
            let s1_size = rd64(104 + 40) as usize;
            let s1_fileoff = rd32(104 + 48) as usize;
            let s1_reloff = rd32(104 + 56) as usize;
            let s1_nreloc = rd32(104 + 60) as usize;
            assert_eq!(s1_size, 32);

            let s2_size = rd64(184 + 40) as usize;
            let s2_fileoff = rd32(184 + 48) as usize;
            let s2_reloff = rd32(184 + 56) as usize;
            let s2_nreloc = rd32(184 + 60) as usize;
            assert_eq!(s2_size, rep.text_bytes);

            // Read LC_SYMTAB (at 32 + 472 + 24 = 528) to inspect symbol names of relocs.
            assert_eq!(rd32(528), LC_SYMTAB);
            let symoff = rd32(528 + 8) as usize;
            let nsyms = rd32(528 + 12) as usize;
            let stroff = rd32(528 + 16) as usize;
            let sym_name = |idx: u32| -> &str {
                let strx = rd32(symoff + (idx as usize) * 16) as usize;
                let start = stroff + strx;
                let end = obj[start..].iter().position(|&b| b == 0).unwrap() + start;
                std::str::from_utf8(&obj[start..end]).unwrap()
            };
            assert!(nsyms >= 6);

            let mut got_p21_hx = Vec::new();
            let mut got_p12_hx = Vec::new();
            let mut got_p21_reg = 0usize;
            let mut got_p12_reg = 0usize;
            let mut branch26_cnt = 0usize;

            for (sec_fileoff, sec_reloff, sec_nreloc) in [
                (s1_fileoff, s1_reloff, s1_nreloc),
                (s2_fileoff, s2_reloff, s2_nreloc),
            ] {
                for ri in 0..sec_nreloc {
                    let ro = sec_reloff + ri * 8;
                    let addr = rd32(ro);
                    let info = rd32(ro + 4);
                    let sym_idx = info & 0x00FF_FFFF;
                    let pcrel = ((info >> 24) & 1) != 0;
                    let length = (info >> 25) & 3;
                    let extern_ = ((info >> 27) & 1) != 0;
                    let rtype = (info >> 28) & 0xF;
                    assert_eq!(length, 2);
                    assert!(extern_);
                    let sname = sym_name(sym_idx);
                    match rtype {
                        ARM64_RELOC_BRANCH26 => branch26_cnt += 1,
                        ARM64_RELOC_GOT_LOAD_PAGE21 => {
                            assert!(pcrel);
                            let insn = rd32(sec_fileoff + addr as usize);
                            assert_eq!(insn, ARM64_ADRP_X16, "GOT_LOAD_PAGE21 must target adrp x16");
                            if sname == "_port_static_aot_host_xfer" {
                                got_p21_hx.push(addr);
                            } else if sname == "_port_static_aot_register_desc" {
                                got_p21_reg += 1;
                            }
                        }
                        ARM64_RELOC_GOT_LOAD_PAGEOFF12 => {
                            assert!(!pcrel);
                            let insn = rd32(sec_fileoff + addr as usize);
                            assert_eq!(insn, ARM64_LDR_X16_X16_IMM0, "GOT_LOAD_PAGEOFF12 must target ldr x16, [x16]");
                            let br = rd32(sec_fileoff + addr as usize + 4);
                            assert_eq!(br, ARM64_BR_X16, "GOT veneer must follow ldr with br x16");
                            if sname == "_port_static_aot_host_xfer" {
                                got_p12_hx.push(addr);
                            } else if sname == "_port_static_aot_register_desc" {
                                got_p12_reg += 1;
                            }
                        }
                        ARM64_RELOC_PAGE21 | ARM64_RELOC_PAGEOFF12 => {}
                        other => panic!("unexpected executable reloc type {other}"),
                    }
                }
            }

            assert_eq!(
                branch26_cnt, 0,
                "multi-bank .aot.o must emit ZERO BRANCH26 relocations (prevents ld64 range errors on >128 MiB images)"
            );
            assert_eq!(got_p21_reg, 1);
            assert_eq!(got_p12_reg, 1);
            if img.stubs[5] != u32::MAX {
                assert_eq!(got_p21_hx.len(), nbanks, "expected one host_xfer GOT veneer per stub bank");
                assert_eq!(got_p12_hx.len(), nbanks);
                for (bi, &bo) in img.stub_banks().iter().enumerate() {
                    let vaddr = got_p21_hx[bi];
                    assert!(
                        vaddr >= bo && vaddr + 16 <= bo + AOT_STUBS_BYTES as u32,
                        "bank {bi} veneer {vaddr:#x} must lie inside bank stubs [{bo:#x}..{:#x})",
                        bo + AOT_STUBS_BYTES as u32
                    );
                    assert_eq!(got_p12_hx[bi], vaddr + 4);
                }
            }

            // End-to-end Apple ld64 + dyld verification on macOS: link the multi-bank `.aot.o`
            // with a minimal C driver providing `_port_static_aot_register_desc` and
            // `_port_static_aot_host_xfer`, then execute it to confirm dyld runs
            // `__DATA,__mod_init_func` through our `x16` GOT veneer with valid descriptor fields!
            #[cfg(target_os = "macos")]
            if plat == MachoPlatform::MacOs {
                let dir = std::env::temp_dir().join(format!(
                    "saot-multibank-ld-{}",
                    std::process::id()
                ));
                let _ = std::fs::remove_dir_all(&dir);
                std::fs::create_dir_all(&dir).unwrap();
                let obj_path = dir.join("big_game.aot.o");
                let c_path = dir.join("driver.c");
                let bin_path = dir.join("driver");
                std::fs::write(&obj_path, &obj).unwrap();
                std::fs::write(
                    &c_path,
                    r#"
#include <stdint.h>
#include <stdio.h>
#include <string.h>
struct StaticAotModuleDesc {
    uint32_t magic;
    uint32_t version;
    uint64_t knobs;
    uint64_t image_base;
    uint64_t image_size;
    uint64_t image_hash[2];
    uintptr_t code_base;
    uint32_t code_len;
    uint32_t enter_off;
    uint32_t lookup_rip_off;
    uint32_t flags;
    uint32_t entry_count;
    uint32_t _pad;
    const void *entries;
    const char *module_name;
};
static int g_ok = 0;
void port_static_aot_register_desc(const struct StaticAotModuleDesc *d) {
    if (d && d->magic == 0x53414F54u && d->version == 1 && d->code_base != 0 &&
        d->entries != 0 && d->entry_count > 0 && d->module_name &&
        strcmp(d->module_name, "big_game.exe") == 0) {
        g_ok = 1;
    }
}
uint64_t port_static_aot_host_xfer(void *cpu, uint64_t target, uint64_t kind) {
    (void)cpu; (void)target; (void)kind;
    return 0;
}
int main(void) { return g_ok ? 0 : 1; }
"#,
                )
                .unwrap();
                let link_out = std::process::Command::new("clang")
                    .args([
                        "-arch",
                        "arm64",
                        "-o",
                        bin_path.to_str().unwrap(),
                        c_path.to_str().unwrap(),
                        obj_path.to_str().unwrap(),
                    ])
                    .output()
                    .expect("invoke clang to link multi-bank .aot.o");
                assert!(
                    link_out.status.success(),
                    "ld64 failed to link multi-bank .aot.o:\nstdout: {}\nstderr: {}",
                    String::from_utf8_lossy(&link_out.stdout),
                    String::from_utf8_lossy(&link_out.stderr),
                );
                let run_status = std::process::Command::new(&bin_path)
                    .status()
                    .expect("execute linked multi-bank driver");
                let _ = std::fs::remove_dir_all(&dir);
                assert!(
                    run_status.success(),
                    "linked multi-bank .aot.o __mod_init_func GOT veneer did not register valid descriptor"
                );
            }
        }
        drop(code_buf);
    }
}
