//! PE32+ parsing helpers for the Alan Wake 2 hybrid loader.
//!
//! Built on top of `goblin` for the DOS/COFF/optional headers and the section
//! table; the data directories that the loader needs (base relocations,
//! imports, delay-load imports, exports incl. forwarders, TLS, `.pdata`, load
//! config) are parsed here directly because goblin either does not expose them
//! (delay imports, TLS callbacks, relocation blocks) or parses them eagerly in a
//! way that fails on some real-world DLLs.
//!
//! Everything here is pure data parsing: no memory mapping, no OS calls, so the
//! crate builds and tests on any host (native arm64 included).

use anyhow::{anyhow, bail, Context, Result};
use std::collections::HashMap;

pub mod consts {
    pub const IMAGE_FILE_MACHINE_I386: u16 = 0x014c;
    pub const IMAGE_FILE_MACHINE_AMD64: u16 = 0x8664;
    pub const IMAGE_FILE_RELOCS_STRIPPED: u16 = 0x0001;
    pub const IMAGE_FILE_DLL: u16 = 0x2000;

    pub const IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE: u16 = 0x0040;

    pub const IMAGE_DIRECTORY_ENTRY_EXPORT: usize = 0;
    pub const IMAGE_DIRECTORY_ENTRY_IMPORT: usize = 1;
    pub const IMAGE_DIRECTORY_ENTRY_RESOURCE: usize = 2;
    pub const IMAGE_DIRECTORY_ENTRY_EXCEPTION: usize = 3;
    pub const IMAGE_DIRECTORY_ENTRY_SECURITY: usize = 4;
    pub const IMAGE_DIRECTORY_ENTRY_BASERELOC: usize = 5;
    pub const IMAGE_DIRECTORY_ENTRY_DEBUG: usize = 6;
    pub const IMAGE_DIRECTORY_ENTRY_TLS: usize = 9;
    pub const IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG: usize = 10;
    pub const IMAGE_DIRECTORY_ENTRY_IAT: usize = 12;
    pub const IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT: usize = 13;
    pub const IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR: usize = 14;

    pub const COMIMAGE_FLAGS_ILONLY: u32 = 0x0000_0001;
    pub const COMIMAGE_FLAGS_32BITREQUIRED: u32 = 0x0000_0002;
    pub const COMIMAGE_FLAGS_IL_LIBRARY: u32 = 0x0000_0004;
    pub const COMIMAGE_FLAGS_STRONGNAMESIGNED: u32 = 0x0000_0008;
    pub const COMIMAGE_FLAGS_NATIVE_ENTRYPOINT: u32 = 0x0000_0010;
    pub const COMIMAGE_FLAGS_TRACKDEBUGDATA: u32 = 0x0001_0000;
    pub const COMIMAGE_FLAGS_32BITPREFERRED: u32 = 0x0002_0000;

    /// ECMA-335 CLI metadata root signature (`"BSJB"` in little-endian).
    pub const CLR_METADATA_SIGNATURE_BSJB: u32 = 0x424A_5342;

    pub const IMAGE_SCN_CNT_CODE: u32 = 0x0000_0020;
    pub const IMAGE_SCN_CNT_UNINITIALIZED_DATA: u32 = 0x0000_0080;
    pub const IMAGE_SCN_MEM_DISCARDABLE: u32 = 0x0200_0000;
    pub const IMAGE_SCN_MEM_EXECUTE: u32 = 0x2000_0000;
    pub const IMAGE_SCN_MEM_READ: u32 = 0x4000_0000;
    pub const IMAGE_SCN_MEM_WRITE: u32 = 0x8000_0000;

    pub const IMAGE_REL_BASED_ABSOLUTE: u8 = 0;
    pub const IMAGE_REL_BASED_HIGHLOW: u8 = 3;
    pub const IMAGE_REL_BASED_DIR64: u8 = 10;

    pub const DLL_PROCESS_DETACH: u32 = 0;
    pub const DLL_PROCESS_ATTACH: u32 = 1;
    pub const DLL_THREAD_ATTACH: u32 = 2;
    pub const DLL_THREAD_DETACH: u32 = 3;
}
use consts::*;

/// How the bytes handed to [`Pe`] are laid out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    /// Raw file on disk: RVAs must be translated through the section table.
    File,
    /// Already mapped image: offset == RVA.
    Mapped,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DataDirectory {
    pub rva: u32,
    pub size: u32,
}

#[derive(Debug, Clone)]
pub struct Section {
    pub name: String,
    pub rva: u32,
    pub virtual_size: u32,
    pub raw_offset: u32,
    pub raw_size: u32,
    pub characteristics: u32,
}

impl Section {
    pub fn is_exec(&self) -> bool {
        self.characteristics & IMAGE_SCN_MEM_EXECUTE != 0
    }
    pub fn is_write(&self) -> bool {
        self.characteristics & IMAGE_SCN_MEM_WRITE != 0
    }
    pub fn is_read(&self) -> bool {
        self.characteristics & IMAGE_SCN_MEM_READ != 0
    }
    /// Size the section occupies in memory (virtual size, or raw size when the
    /// linker left virtual size zero).
    pub fn mem_size(&self) -> u32 {
        if self.virtual_size == 0 {
            self.raw_size
        } else {
            self.virtual_size
        }
    }
    pub fn contains_rva(&self, rva: u32) -> bool {
        rva >= self.rva && rva < self.rva.wrapping_add(self.mem_size().max(self.raw_size))
    }
}

/// The interesting fields of the COFF + optional header.
#[derive(Debug, Clone)]
pub struct Headers {
    pub machine: u16,
    pub characteristics: u16,
    pub image_base: u64,
    pub entry_rva: u32,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub subsystem: u16,
    pub dll_characteristics: u16,
    pub stack_reserve: u64,
    pub stack_commit: u64,
    pub heap_reserve: u64,
    pub heap_commit: u64,
    pub major_os_version: u16,
    pub minor_os_version: u16,
    pub data_dirs: [DataDirectory; 16],
}

impl Headers {
    pub fn is_dll(&self) -> bool {
        self.characteristics & IMAGE_FILE_DLL != 0
    }
    pub fn relocs_stripped(&self) -> bool {
        self.characteristics & IMAGE_FILE_RELOCS_STRIPPED != 0
    }
    pub fn dir(&self, idx: usize) -> DataDirectory {
        self.data_dirs[idx]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ImportName {
    Name { hint: u16, name: String },
    Ordinal(u16),
}

impl std::fmt::Display for ImportName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportName::Name { name, .. } => f.write_str(name),
            ImportName::Ordinal(o) => write!(f, "#{o}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImportEntry {
    pub name: ImportName,
    /// RVA of the IAT slot the loader must fill.
    pub iat_rva: u32,
}

/// `ImgDelayDescr` (delayimp.h).
#[derive(Debug, Clone, Copy, Default)]
pub struct DelayDescriptor {
    pub attributes: u32,
    pub dll_name_rva: u32,
    pub module_handle_rva: u32,
    pub iat_rva: u32,
    pub int_rva: u32,
    pub bound_iat_rva: u32,
    pub unload_iat_rva: u32,
    pub timestamp: u32,
}

#[derive(Debug, Clone)]
pub struct ImportDll {
    pub dll: String,
    pub entries: Vec<ImportEntry>,
    /// `Some` for delay-load imports.
    pub delay: Option<DelayDescriptor>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportTarget {
    Rva(u32),
    /// `"OTHERDLL.Function"` or `"OTHERDLL.#12"`.
    Forwarder(String),
}

#[derive(Debug, Clone)]
pub struct Export {
    /// Biased ordinal (what `GetProcAddress(h, MAKEINTRESOURCE(n))` takes).
    pub ordinal: u32,
    pub name: Option<String>,
    pub target: ExportTarget,
}

#[derive(Debug, Clone, Default)]
pub struct Exports {
    pub dll_name: String,
    pub ordinal_base: u32,
    pub entries: Vec<Export>,
    by_name: HashMap<String, usize>,
    by_ordinal: HashMap<u32, usize>,
}

impl Exports {
    pub fn by_name(&self, name: &str) -> Option<&Export> {
        self.by_name.get(name).map(|&i| &self.entries[i])
    }
    pub fn by_ordinal(&self, ordinal: u32) -> Option<&Export> {
        self.by_ordinal.get(&ordinal).map(|&i| &self.entries[i])
    }
    pub fn lookup(&self, name: &ImportName) -> Option<&Export> {
        match name {
            ImportName::Name { name, .. } => self.by_name(name),
            ImportName::Ordinal(o) => self.by_ordinal(*o as u32),
        }
    }
    /// Nearest named export at or below `rva` (for symbolizing addresses).
    pub fn nearest(&self, rva: u32) -> Option<(&str, u32)> {
        let mut best: Option<(&str, u32)> = None;
        for e in &self.entries {
            if let (Some(n), ExportTarget::Rva(r)) = (&e.name, &e.target) {
                if *r <= rva && best.map_or(true, |(_, b)| *r > b) {
                    best = Some((n.as_str(), *r));
                }
            }
        }
        best
    }
}

/// `IMAGE_TLS_DIRECTORY64`. Addresses are VAs relative to the *preferred*
/// image base as stored in the file (they are covered by base relocations).
#[derive(Debug, Clone, Copy, Default)]
pub struct TlsDirectory {
    pub start_raw_data_va: u64,
    pub end_raw_data_va: u64,
    pub index_va: u64,
    pub callbacks_va: u64,
    pub size_of_zero_fill: u32,
    pub characteristics: u32,
}

impl TlsDirectory {
    pub fn template_size(&self) -> usize {
        self.end_raw_data_va.saturating_sub(self.start_raw_data_va) as usize
    }
    pub fn block_size(&self) -> usize {
        self.template_size() + self.size_of_zero_fill as usize
    }
}

/// `RUNTIME_FUNCTION` from `.pdata`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct RuntimeFunction {
    pub begin_rva: u32,
    pub end_rva: u32,
    pub unwind_rva: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct BaseReloc {
    pub rva: u32,
    pub kind: u8,
}

/// Subset of `IMAGE_LOAD_CONFIG_DIRECTORY64` the loader cares about.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LoadConfig {
    pub size: u32,
    pub security_cookie_va: u64,
    pub guard_cf_check_function_pointer_va: u64,
    pub guard_cf_dispatch_function_pointer_va: u64,
    pub guard_flags: u32,
    /// `IMAGE_LOAD_CONFIG_DIRECTORY64.VolatileMetadataPointer` (`+0x100`), if present.
    pub volatile_metadata_va: u64,
}

/// One compiler-certified code RVA range `[start_rva, end_rva)` from
/// `IMAGE_VOLATILE_METADATA::VolatileInfoRangeTable`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VolatileRange {
    pub start_rva: u32,
    pub end_rva: u32,
}

/// Parsed PE `IMAGE_VOLATILE_METADATA` table (`VolatileMetadataPointer`).
///
/// Within any range in [`Self::ranges`], the MSVC compiler (`/volatileMetadata`) certifies
/// that memory accesses NOT listed in [`Self::access_rvas`] (and not explicit `LOCK`/fence
/// instructions) are non-volatile/non-atomic and do not require x86 TSO acquire/release
/// ordering on weakly-ordered architectures.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VolatileMetadata {
    pub version: u32,
    /// Sorted, non-overlapping covered code RVA half-open ranges `[start_rva, end_rva)`.
    pub ranges: Vec<VolatileRange>,
    /// Sorted, deduplicated instruction RVAs that perform volatile/atomic memory accesses.
    pub access_rvas: Vec<u32>,
}

impl VolatileMetadata {
    /// True if no covered code ranges are present.
    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }

    /// True if instruction `rva` lies inside a compiler-certified covered range.
    #[inline]
    pub fn in_covered_range(&self, rva: u32) -> bool {
        let idx = self.ranges.partition_point(|r| r.start_rva <= rva);
        idx > 0 && rva < self.ranges[idx - 1].end_rva
    }

    /// True if instruction `rva` is explicitly listed in `VolatileAccessTable`.
    #[inline]
    pub fn is_volatile_access(&self, rva: u32) -> bool {
        self.access_rvas.binary_search(&rva).is_ok()
    }

    /// True if TSO ordering may be safely elided for a non-`LOCK` access at `rva`:
    /// inside a covered range AND not listed in `VolatileAccessTable`.
    #[inline]
    pub fn can_elide_tso(&self, rva: u32) -> bool {
        self.in_covered_range(rva) && !self.is_volatile_access(rva)
    }
}

/// A parsed PE32+ image (borrowing the bytes).
pub struct Pe<'a> {
    data: &'a [u8],
    layout: Layout,
    pub headers: Headers,
    pub sections: Vec<Section>,
}

fn rd_u16(d: &[u8], o: usize) -> Result<u16> {
    d.get(o..o + 2)
        .map(|b| u16::from_le_bytes([b[0], b[1]]))
        .ok_or_else(|| anyhow!("read u16 out of bounds at 0x{o:x}"))
}
fn rd_u32(d: &[u8], o: usize) -> Result<u32> {
    d.get(o..o + 4)
        .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
        .ok_or_else(|| anyhow!("read u32 out of bounds at 0x{o:x}"))
}
fn rd_u64(d: &[u8], o: usize) -> Result<u64> {
    d.get(o..o + 8)
        .map(|b| u64::from_le_bytes(b.try_into().unwrap()))
        .ok_or_else(|| anyhow!("read u64 out of bounds at 0x{o:x}"))
}

impl<'a> Pe<'a> {
    /// Parse a PE file as laid out on disk.
    pub fn parse(data: &'a [u8]) -> Result<Self> {
        Self::parse_with_layout(data, Layout::File)
    }

    /// Parse an image that has already been mapped (offset == RVA).
    pub fn parse_mapped(data: &'a [u8]) -> Result<Self> {
        Self::parse_with_layout(data, Layout::Mapped)
    }

    pub fn parse_with_layout(data: &'a [u8], layout: Layout) -> Result<Self> {
        let hdr = goblin::pe::header::Header::parse(data).context("goblin: PE header")?;
        let opt = hdr
            .optional_header
            .ok_or_else(|| anyhow!("PE has no optional header"))?;
        if opt.standard_fields.magic != goblin::pe::optional_header::MAGIC_64 {
            bail!("not a PE32+ image (magic 0x{:x})", opt.standard_fields.magic);
        }
        let wf = &opt.windows_fields;
        let pe_off = hdr.dos_header.pe_pointer as usize;
        let opt_off = pe_off + 4 + goblin::pe::header::SIZEOF_COFF_HEADER;
        // PE32+ optional header: data directories start at +112.
        let nrva = (wf.number_of_rva_and_sizes as usize).min(16);
        let mut data_dirs = [DataDirectory::default(); 16];
        for (i, dd) in data_dirs.iter_mut().enumerate().take(nrva) {
            let o = opt_off + 112 + i * 8;
            *dd = DataDirectory {
                rva: rd_u32(data, o)?,
                size: rd_u32(data, o + 4)?,
            };
        }
        let headers = Headers {
            machine: hdr.coff_header.machine,
            characteristics: hdr.coff_header.characteristics,
            image_base: wf.image_base as u64,
            entry_rva: opt.standard_fields.address_of_entry_point as u32,
            section_alignment: wf.section_alignment as u32,
            file_alignment: wf.file_alignment as u32,
            size_of_image: wf.size_of_image as u32,
            size_of_headers: wf.size_of_headers as u32,
            subsystem: wf.subsystem as u16,
            dll_characteristics: wf.dll_characteristics as u16,
            stack_reserve: wf.size_of_stack_reserve as u64,
            stack_commit: wf.size_of_stack_commit as u64,
            heap_reserve: wf.size_of_heap_reserve as u64,
            heap_commit: wf.size_of_heap_commit as u64,
            major_os_version: wf.major_operating_system_version as u16,
            minor_os_version: wf.minor_operating_system_version as u16,
            data_dirs,
        };
        if headers.machine != IMAGE_FILE_MACHINE_AMD64 {
            bail!("unsupported machine 0x{:x}", headers.machine);
        }
        let mut off = opt_off + hdr.coff_header.size_of_optional_header as usize;
        let goblin_sections = hdr
            .coff_header
            .sections(data, &mut off)
            .context("goblin: section table")?;
        let sections = goblin_sections
            .iter()
            .map(|s| Section {
                name: s
                    .name()
                    .map(|n| n.to_string())
                    .unwrap_or_else(|_| String::from_utf8_lossy(&s.name).trim_end_matches('\0').to_string()),
                rva: s.virtual_address,
                virtual_size: s.virtual_size,
                raw_offset: s.pointer_to_raw_data,
                raw_size: s.size_of_raw_data,
                characteristics: s.characteristics,
            })
            .collect();
        Ok(Pe {
            data,
            layout,
            headers,
            sections,
        })
    }

    pub fn data(&self) -> &'a [u8] {
        self.data
    }

    pub fn layout(&self) -> Layout {
        self.layout
    }

    pub fn section_for_rva(&self, rva: u32) -> Option<&Section> {
        self.sections.iter().find(|s| s.contains_rva(rva))
    }

    /// Translate an RVA to an offset into `data`.
    pub fn rva_to_offset(&self, rva: u32) -> Option<usize> {
        match self.layout {
            Layout::Mapped => ((rva as usize) < self.data.len()).then_some(rva as usize),
            Layout::File => {
                if rva < self.headers.size_of_headers {
                    return Some(rva as usize);
                }
                let s = self
                    .sections
                    .iter()
                    .find(|s| rva >= s.rva && rva < s.rva + s.raw_size.max(s.virtual_size))?;
                let delta = rva - s.rva;
                if delta >= s.raw_size {
                    return None; // in the zero-filled tail; not backed by file data
                }
                Some(s.raw_offset as usize + delta as usize)
            }
        }
    }

    /// Bytes at `rva` (up to `len`), or an error if not file-backed.
    pub fn bytes_at(&self, rva: u32, len: usize) -> Result<&'a [u8]> {
        let o = self
            .rva_to_offset(rva)
            .ok_or_else(|| anyhow!("rva 0x{rva:x} not backed by file data"))?;
        self.data
            .get(o..o + len)
            .ok_or_else(|| anyhow!("rva 0x{rva:x}+0x{len:x} out of bounds"))
    }

    pub fn u16_at(&self, rva: u32) -> Result<u16> {
        rd_u16(self.bytes_at(rva, 2)?, 0)
    }
    pub fn u32_at(&self, rva: u32) -> Result<u32> {
        rd_u32(self.bytes_at(rva, 4)?, 0)
    }
    pub fn u64_at(&self, rva: u32) -> Result<u64> {
        rd_u64(self.bytes_at(rva, 8)?, 0)
    }

    /// NUL-terminated ASCII string at `rva`.
    pub fn cstr_at(&self, rva: u32) -> Result<String> {
        let o = self
            .rva_to_offset(rva)
            .ok_or_else(|| anyhow!("string rva 0x{rva:x} not mapped"))?;
        let tail = &self.data[o..];
        let end = tail.iter().position(|&b| b == 0).unwrap_or(tail.len());
        Ok(String::from_utf8_lossy(&tail[..end]).into_owned())
    }

    fn va_to_rva(&self, va: u64) -> Result<u32> {
        va.checked_sub(self.headers.image_base)
            .filter(|r| *r < self.headers.size_of_image as u64)
            .map(|r| r as u32)
            .ok_or_else(|| anyhow!("VA 0x{va:x} outside image"))
    }

    // ---------------------------------------------------------------- relocs

    /// All base relocations (IMAGE_REL_BASED_ABSOLUTE padding entries skipped).
    pub fn relocations(&self) -> Result<Vec<BaseReloc>> {
        let dir = self.headers.dir(IMAGE_DIRECTORY_ENTRY_BASERELOC);
        let mut out = Vec::new();
        if dir.rva == 0 || dir.size == 0 {
            return Ok(out);
        }
        let mut pos = 0u32;
        while pos + 8 <= dir.size {
            let page = self.u32_at(dir.rva + pos)?;
            let block = self.u32_at(dir.rva + pos + 4)?;
            if block < 8 {
                break;
            }
            let n = (block - 8) / 2;
            for i in 0..n {
                let e = self.u16_at(dir.rva + pos + 8 + i * 2)?;
                let kind = (e >> 12) as u8;
                if kind == IMAGE_REL_BASED_ABSOLUTE {
                    continue;
                }
                out.push(BaseReloc {
                    rva: page + (e & 0xfff) as u32,
                    kind,
                });
            }
            pos += block;
        }
        Ok(out)
    }

    // ---------------------------------------------------------------- imports

    fn read_thunk_list(&self, int_rva: u32, iat_rva: u32) -> Result<Vec<ImportEntry>> {
        let mut out = Vec::new();
        // Some linkers leave the INT (OriginalFirstThunk) zero; fall back to the IAT.
        let names_rva = if int_rva != 0 { int_rva } else { iat_rva };
        for i in 0.. {
            let thunk = self.u64_at(names_rva + i * 8)?;
            if thunk == 0 {
                break;
            }
            let name = if thunk & (1 << 63) != 0 {
                ImportName::Ordinal((thunk & 0xffff) as u16)
            } else {
                let hn = (thunk & 0x7fff_ffff) as u32;
                ImportName::Name {
                    hint: self.u16_at(hn)?,
                    name: self.cstr_at(hn + 2)?,
                }
            };
            out.push(ImportEntry {
                name,
                iat_rva: iat_rva + i * 8,
            });
        }
        Ok(out)
    }

    /// Regular (eagerly bound) imports.
    pub fn imports(&self) -> Result<Vec<ImportDll>> {
        let dir = self.headers.dir(IMAGE_DIRECTORY_ENTRY_IMPORT);
        let mut out = Vec::new();
        if dir.rva == 0 {
            return Ok(out);
        }
        for i in 0.. {
            let d = dir.rva + i * 20;
            let int_rva = self.u32_at(d)?;
            let name_rva = self.u32_at(d + 12)?;
            let iat_rva = self.u32_at(d + 16)?;
            if int_rva == 0 && name_rva == 0 && iat_rva == 0 {
                break;
            }
            let dll = self.cstr_at(name_rva)?;
            let entries = self
                .read_thunk_list(int_rva, iat_rva)
                .with_context(|| format!("imports of {dll}"))?;
            out.push(ImportDll {
                dll,
                entries,
                delay: None,
            });
        }
        Ok(out)
    }

    /// Delay-load imports (`IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT`).
    pub fn delay_imports(&self) -> Result<Vec<ImportDll>> {
        let dir = self.headers.dir(IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT);
        let mut out = Vec::new();
        if dir.rva == 0 {
            return Ok(out);
        }
        for i in 0.. {
            let d = dir.rva + i * 32;
            let desc = DelayDescriptor {
                attributes: self.u32_at(d)?,
                dll_name_rva: self.u32_at(d + 4)?,
                module_handle_rva: self.u32_at(d + 8)?,
                iat_rva: self.u32_at(d + 12)?,
                int_rva: self.u32_at(d + 16)?,
                bound_iat_rva: self.u32_at(d + 20)?,
                unload_iat_rva: self.u32_at(d + 24)?,
                timestamp: self.u32_at(d + 28)?,
            };
            if desc.dll_name_rva == 0 {
                break;
            }
            if desc.attributes & 1 == 0 {
                // Pre-VC7 delay descriptors use VAs; never seen in x64 binaries.
                bail!("VA-based (attributes=0) delay import descriptor unsupported");
            }
            let dll = self.cstr_at(desc.dll_name_rva)?;
            let entries = self
                .read_thunk_list(desc.int_rva, desc.iat_rva)
                .with_context(|| format!("delay imports of {dll}"))?;
            out.push(ImportDll {
                dll,
                entries,
                delay: Some(desc),
            });
        }
        Ok(out)
    }

    // ---------------------------------------------------------------- exports

    pub fn exports(&self) -> Result<Exports> {
        let dir = self.headers.dir(IMAGE_DIRECTORY_ENTRY_EXPORT);
        let mut ex = Exports::default();
        if dir.rva == 0 {
            return Ok(ex);
        }
        let d = dir.rva;
        ex.dll_name = self.cstr_at(self.u32_at(d + 12)?).unwrap_or_default();
        ex.ordinal_base = self.u32_at(d + 16)?;
        let n_funcs = self.u32_at(d + 20)?;
        let n_names = self.u32_at(d + 24)?;
        let funcs = self.u32_at(d + 28)?;
        let names = self.u32_at(d + 32)?;
        let ords = self.u32_at(d + 36)?;
        let mut names_by_index: HashMap<u32, String> = HashMap::new();
        for i in 0..n_names {
            let name = self.cstr_at(self.u32_at(names + i * 4)?)?;
            let idx = self.u16_at(ords + i * 2)? as u32;
            names_by_index.insert(idx, name);
        }
        for i in 0..n_funcs {
            let rva = self.u32_at(funcs + i * 4)?;
            if rva == 0 {
                continue;
            }
            let target = if rva >= dir.rva && rva < dir.rva + dir.size {
                ExportTarget::Forwarder(self.cstr_at(rva)?)
            } else {
                ExportTarget::Rva(rva)
            };
            let e = Export {
                ordinal: ex.ordinal_base + i,
                name: names_by_index.remove(&i),
                target,
            };
            let idx = ex.entries.len();
            if let Some(n) = &e.name {
                ex.by_name.insert(n.clone(), idx);
            }
            ex.by_ordinal.insert(e.ordinal, idx);
            ex.entries.push(e);
        }
        Ok(ex)
    }

    // ---------------------------------------------------------------- TLS

    pub fn tls(&self) -> Result<Option<TlsDirectory>> {
        let dir = self.headers.dir(IMAGE_DIRECTORY_ENTRY_TLS);
        if dir.rva == 0 {
            return Ok(None);
        }
        let d = dir.rva;
        Ok(Some(TlsDirectory {
            start_raw_data_va: self.u64_at(d)?,
            end_raw_data_va: self.u64_at(d + 8)?,
            index_va: self.u64_at(d + 16)?,
            callbacks_va: self.u64_at(d + 24)?,
            size_of_zero_fill: self.u32_at(d + 32)?,
            characteristics: self.u32_at(d + 36)?,
        }))
    }

    /// RVAs of the TLS callbacks as stored in the file (VA - preferred base).
    pub fn tls_callback_rvas(&self) -> Result<Vec<u32>> {
        let Some(tls) = self.tls()? else {
            return Ok(Vec::new());
        };
        if tls.callbacks_va == 0 {
            return Ok(Vec::new());
        }
        let arr = self.va_to_rva(tls.callbacks_va)?;
        let mut out = Vec::new();
        for i in 0.. {
            let cb = self.u64_at(arr + i * 8)?;
            if cb == 0 {
                break;
            }
            out.push(self.va_to_rva(cb)?);
        }
        Ok(out)
    }

    /// Raw TLS template bytes (the initialized part, without zero fill).
    pub fn tls_template(&self) -> Result<Option<&'a [u8]>> {
        let Some(tls) = self.tls()? else {
            return Ok(None);
        };
        if tls.template_size() == 0 {
            return Ok(Some(&[]));
        }
        let rva = self.va_to_rva(tls.start_raw_data_va)?;
        Ok(Some(self.bytes_at(rva, tls.template_size())?))
    }

    // ---------------------------------------------------------------- .pdata

    pub fn runtime_functions(&self) -> Result<Vec<RuntimeFunction>> {
        let dir = self.headers.dir(IMAGE_DIRECTORY_ENTRY_EXCEPTION);
        let mut out = Vec::with_capacity(dir.size as usize / 12);
        for i in 0..dir.size / 12 {
            let o = dir.rva + i * 12;
            out.push(RuntimeFunction {
                begin_rva: self.u32_at(o)?,
                end_rva: self.u32_at(o + 4)?,
                unwind_rva: self.u32_at(o + 8)?,
            });
        }
        Ok(out)
    }

    // ---------------------------------------------------------------- load config

    pub fn load_config(&self) -> Result<Option<LoadConfig>> {
        let dir = self.headers.dir(IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG);
        if dir.rva == 0 {
            return Ok(None);
        }
        let d = dir.rva;
        let size = self.u32_at(d)?;
        let get = |off: u32| -> u64 {
            if off.checked_add(8).is_some_and(|e| e <= size) {
                self.u64_at(d + off).unwrap_or(0)
            } else {
                0
            }
        };
        Ok(Some(LoadConfig {
            size,
            security_cookie_va: get(0x58),
            guard_cf_check_function_pointer_va: get(0x70),
            guard_cf_dispatch_function_pointer_va: get(0x78),
            guard_flags: if 0x90 + 4 <= size {
                self.u32_at(d + 0x90).unwrap_or(0)
            } else {
                0
            },
            volatile_metadata_va: get(0x100),
        }))
    }

    /// Parse `IMAGE_VOLATILE_METADATA` referenced by
    /// `IMAGE_LOAD_CONFIG_DIRECTORY64.VolatileMetadataPointer` (`+0x100`), if present.
    pub fn volatile_metadata(&self) -> Result<Option<VolatileMetadata>> {
        let Some(lc) = self.load_config()? else {
            return Ok(None);
        };
        if lc.volatile_metadata_va == 0 {
            return Ok(None);
        }
        let vm_rva = match self.va_to_rva(lc.volatile_metadata_va) {
            Ok(r) => r,
            Err(_) if self.layout == Layout::Mapped => {
                let mapped_base = self.data.as_ptr() as u64;
                let Some(rel) = lc
                    .volatile_metadata_va
                    .checked_sub(mapped_base)
                    .filter(|&r| (r as usize) < self.data.len())
                else {
                    return Ok(None);
                };
                rel as u32
            }
            Err(_) => return Ok(None),
        };
        let hdr = match self.bytes_at(vm_rva, 24) {
            Ok(b) => b,
            Err(_) => return Ok(None),
        };
        let vm_size = rd_u32(hdr, 0)?;
        if vm_size < 24 {
            return Ok(None);
        }
        let version = rd_u32(hdr, 4)?;
        let access_rva = rd_u32(hdr, 8)?;
        let access_size = rd_u32(hdr, 12)?;
        let range_rva = rd_u32(hdr, 16)?;
        let range_size = rd_u32(hdr, 20)?;

        let mut ranges = Vec::new();
        if range_rva != 0 && range_size >= 8 {
            if range_size % 8 != 0 {
                return Ok(None);
            }
            let Ok(rbytes) = self.bytes_at(range_rva, range_size as usize) else {
                return Ok(None);
            };
            ranges.reserve((range_size / 8) as usize);
            for chunk in rbytes.chunks_exact(8) {
                let start_rva = u32::from_le_bytes(chunk[0..4].try_into().unwrap());
                let len = u32::from_le_bytes(chunk[4..8].try_into().unwrap());
                if len == 0 {
                    continue;
                }
                let end_rva = start_rva.saturating_add(len);
                if end_rva > start_rva {
                    ranges.push(VolatileRange { start_rva, end_rva });
                }
            }
            ranges.sort_unstable_by_key(|r| (r.start_rva, r.end_rva));
            // Coalesce overlapping/adjacent ranges so binary search is strictly monotone.
            let mut merged: Vec<VolatileRange> = Vec::with_capacity(ranges.len());
            for r in ranges {
                if let Some(last) = merged.last_mut() {
                    if r.start_rva <= last.end_rva {
                        last.end_rva = last.end_rva.max(r.end_rva);
                        continue;
                    }
                }
                merged.push(r);
            }
            ranges = merged;
        }

        let mut access_rvas = Vec::new();
        if access_rva != 0 && access_size >= 4 {
            if access_size % 4 != 0 {
                return Ok(None);
            }
            let Ok(abytes) = self.bytes_at(access_rva, access_size as usize) else {
                return Ok(None);
            };
            access_rvas.reserve((access_size / 4) as usize);
            for chunk in abytes.chunks_exact(4) {
                access_rvas.push(u32::from_le_bytes(chunk.try_into().unwrap()));
            }
            access_rvas.sort_unstable();
            access_rvas.dedup();
        }

        Ok(Some(VolatileMetadata {
            version,
            ranges,
            access_rvas,
        }))
    }

    // ---------------------------------------------------------------- CLR / .NET

    /// Parse `IMAGE_COR20_HEADER` (`IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR`, index 14)
    /// if this PE is a `.NET` / CLR / Mono assembly.
    pub fn clr_header(&self) -> Result<Option<ClrHeader>> {
        let dir = self.headers.dir(IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR);
        if dir.rva == 0 || dir.size < 72 {
            return Ok(None);
        }
        let b = self.bytes_at(dir.rva, 72)?;
        ClrHeader::from_bytes(b).map(Some)
    }

    /// Parse the ECMA-335 `BSJB` metadata root pointed to by `IMAGE_COR20_HEADER.MetaData`.
    pub fn clr_metadata(&self) -> Result<Option<ClrMetadata>> {
        let Some(clr) = self.clr_header()? else {
            return Ok(None);
        };
        if clr.metadata.rva == 0 || clr.metadata.size < 16 {
            return Ok(None);
        }
        let len = (clr.metadata.size as usize).min(4096);
        let Some(off) = self.rva_to_offset(clr.metadata.rva) else {
            return Ok(None);
        };
        let avail = self.data.len().saturating_sub(off).min(len);
        ClrMetadata::from_bytes(&self.data[off..off + avail]).map(Some)
    }
}

/// `IMAGE_COR20_HEADER` (72 bytes) from `IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ClrHeader {
    pub cb: u32,
    pub major_runtime_version: u16,
    pub minor_runtime_version: u16,
    pub metadata: DataDirectory,
    pub flags: u32,
    pub entry_point_token_or_rva: u32,
    pub resources: DataDirectory,
    pub strong_name_signature: DataDirectory,
    pub code_manager_table: DataDirectory,
    pub vtable_fixups: DataDirectory,
    pub export_address_table_jumps: DataDirectory,
    /// Non-zero for `.NET` ReadyToRun (`R2R`) or NGen images containing precompiled machine code.
    pub managed_native_header: DataDirectory,
}

impl ClrHeader {
    pub fn from_bytes(b: &[u8]) -> Result<Self> {
        if b.len() < 72 {
            bail!("IMAGE_COR20_HEADER too short ({} < 72 bytes)", b.len());
        }
        let dd = |off: usize| -> Result<DataDirectory> {
            Ok(DataDirectory {
                rva: rd_u32(b, off)?,
                size: rd_u32(b, off + 4)?,
            })
        };
        Ok(Self {
            cb: rd_u32(b, 0)?,
            major_runtime_version: rd_u16(b, 4)?,
            minor_runtime_version: rd_u16(b, 6)?,
            metadata: dd(8)?,
            flags: rd_u32(b, 16)?,
            entry_point_token_or_rva: rd_u32(b, 20)?,
            resources: dd(24)?,
            strong_name_signature: dd(32)?,
            code_manager_table: dd(40)?,
            vtable_fixups: dd(48)?,
            export_address_table_jumps: dd(56)?,
            managed_native_header: dd(64)?,
        })
    }

    pub fn is_il_only(&self) -> bool {
        self.flags & COMIMAGE_FLAGS_ILONLY != 0
    }

    pub fn is_32bit_required(&self) -> bool {
        (self.flags & COMIMAGE_FLAGS_32BITREQUIRED != 0)
            && (self.flags & COMIMAGE_FLAGS_32BITPREFERRED == 0)
    }

    pub fn is_32bit_preferred(&self) -> bool {
        (self.flags & COMIMAGE_FLAGS_32BITREQUIRED != 0)
            && (self.flags & COMIMAGE_FLAGS_32BITPREFERRED != 0)
    }

    pub fn is_strong_name_signed(&self) -> bool {
        self.flags & COMIMAGE_FLAGS_STRONGNAMESIGNED != 0
    }

    pub fn is_ready_to_run(&self) -> bool {
        self.managed_native_header.rva != 0 && self.managed_native_header.size != 0
    }

    pub fn has_native_vtable_fixups(&self) -> bool {
        self.vtable_fixups.rva != 0 && self.vtable_fixups.size != 0
    }

    /// Classify how this `.NET` / CLR assembly should be executed on Apple Silicon.
    pub fn execution_mode(&self, machine: u16) -> ClrExecutionMode {
        if self.is_ready_to_run() {
            ClrExecutionMode::ReadyToRunHybrid
        } else if !self.is_il_only() || self.has_native_vtable_fixups() {
            ClrExecutionMode::MixedModeX64
        } else if self.is_32bit_required() && machine == IMAGE_FILE_MACHINE_I386 {
            ClrExecutionMode::Il32BitOnly
        } else {
            ClrExecutionMode::PureIlAnyCpu
        }
    }
}

/// ECMA-335 II.24.2.1 CLI Metadata Root (`"BSJB"` header).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ClrMetadata {
    pub major_version: u16,
    pub minor_version: u16,
    /// Standard runtime version string, e.g. `"v4.0.30319"` or `"v2.0.50727"`.
    pub version_string: String,
    /// Metadata stream names (`"#~"`, `"#Strings"`, `"#US"`, `"#GUID"`, `"#Blob"`, `"#Pdb"`).
    pub streams: Vec<String>,
}

impl ClrMetadata {
    pub fn from_bytes(b: &[u8]) -> Result<Self> {
        if b.len() < 16 {
            bail!("CLR metadata root too short ({} < 16 bytes)", b.len());
        }
        let sig = rd_u32(b, 0)?;
        if sig != CLR_METADATA_SIGNATURE_BSJB {
            bail!("invalid CLR metadata signature 0x{sig:08x} (expected BSJB 0x424a5342)");
        }
        let major_version = rd_u16(b, 4)?;
        let minor_version = rd_u16(b, 6)?;
        let ver_len = rd_u32(b, 12)? as usize;
        if 16 + ver_len > b.len() {
            bail!("CLR metadata version string length {ver_len} exceeds buffer");
        }
        let raw_ver = &b[16..16 + ver_len];
        let end = raw_ver.iter().position(|&c| c == 0).unwrap_or(raw_ver.len());
        let version_string = String::from_utf8_lossy(&raw_ver[..end]).into_owned();

        // Align to 4-byte boundary after version string, then read Flags (u16) + Streams (u16)
        let aligned = (16 + ver_len + 3) & !3;
        let mut streams = Vec::new();
        if aligned + 4 <= b.len() {
            let n_streams = rd_u16(b, aligned + 2)? as usize;
            let mut cur = aligned + 4;
            for _ in 0..n_streams.min(16) {
                if cur + 8 > b.len() {
                    break;
                }
                let _s_off = rd_u32(b, cur)?;
                let _s_size = rd_u32(b, cur + 4)?;
                cur += 8;
                let tail = &b[cur..];
                let Some(nul) = tail.iter().position(|&c| c == 0) else {
                    break;
                };
                let s_name = String::from_utf8_lossy(&tail[..nul]).into_owned();
                streams.push(s_name);
                // Stream name is NUL-terminated and padded to 4-byte boundary
                cur += (nul + 1 + 3) & !3;
            }
        }

        Ok(Self {
            major_version,
            minor_version,
            version_string,
            streams,
        })
    }
}

/// Execution strategy for a `.NET` / CLR / Mono PE assembly on macOS ARM64.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClrExecutionMode {
    /// Pure CIL (`AnyCPU` or `x64` `ILONLY`): can execute directly on native macOS ARM64
    /// CoreCLR (`libcoreclr.dylib`) or Mono (`libmono-2.0.dylib`) with zero x86-64 emulation.
    PureIlAnyCpu,
    /// `.NET` ReadyToRun (`R2R`) composite assembly containing both portable CIL and pre-JITted
    /// x86-64 code (`ManagedNativeHeader`). Stripping `ManagedNativeHeader` allows native ARM64
    /// CoreCLR to JIT the CIL directly, or our x86-64->ARM64 AOT can translate the R2R code.
    ReadyToRunHybrid,
    /// Mixed-mode C++/CLI (`IJW` / `VTableFixups`) containing native x86-64 functions and CIL:
    /// requires running under our x86-64->ARM64 translator alongside the CLR bridge.
    MixedModeX64,
    /// 32-bit x86 `ILONLY` assembly with `COMIMAGE_FLAGS_32BITREQUIRED` (without `32BITPREFERRED`):
    /// clearing the `32BITREQUIRED` flag promotes pure-IL assemblies to 64-bit AnyCPU unless they
    /// P/Invoke 32-bit-only native DLLs.
    Il32BitOnly,
}

impl ClrExecutionMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PureIlAnyCpu => "Pure-IL (Native ARM64 CoreCLR/Mono Ready)",
            Self::ReadyToRunHybrid => "ReadyToRun Hybrid (CIL + x86-64 R2R — Strip R2R or AOT)",
            Self::MixedModeX64 => "Mixed-Mode C++/CLI (x86-64 AOT + CLR Bridge)",
            Self::Il32BitOnly => "IL-Only 32-bit Marked (Clear 32BITREQUIRED for 64-bit CLR)",
        }
    }
}

/// Complete `.NET` / CLR inspection result for any PE32 (`0x10b` AnyCPU) or PE32+ (`0x20b` x64) file.
#[derive(Debug, Clone)]
pub struct ClrInspection {
    pub is_pe32_plus: bool,
    pub machine: u16,
    pub is_dll: bool,
    pub clr_dir: DataDirectory,
    pub header: ClrHeader,
    pub metadata: Option<ClrMetadata>,
    pub mode: ClrExecutionMode,
}

/// Inspect a raw PE file (supporting both 32-bit PE32 `0x10b` AnyCPU assemblies and
/// 64-bit PE32+ `0x20b` assemblies) for a `.NET` / CLR `IMAGE_COR20_HEADER`.
pub fn inspect_clr_bytes(data: &[u8]) -> Result<Option<ClrInspection>> {
    let hdr = goblin::pe::header::Header::parse(data).context("goblin: PE header")?;
    let Some(opt) = hdr.optional_header else {
        return Ok(None);
    };
    let is_pe32_plus = match opt.standard_fields.magic {
        goblin::pe::optional_header::MAGIC_64 => true,
        goblin::pe::optional_header::MAGIC_32 => false,
        _ => return Ok(None),
    };
    let pe_off = hdr.dos_header.pe_pointer as usize;
    let opt_off = pe_off + 4 + goblin::pe::header::SIZEOF_COFF_HEADER;
    // Data directories start at +112 in PE32+ and +96 in PE32.
    let dd_base = opt_off + if is_pe32_plus { 112 } else { 96 };
    let nrva = (opt.windows_fields.number_of_rva_and_sizes as usize).min(16);
    if nrva <= IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR {
        return Ok(None);
    }
    let clr_dd_off = dd_base + IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR * 8;
    let clr_dir = DataDirectory {
        rva: rd_u32(data, clr_dd_off)?,
        size: rd_u32(data, clr_dd_off + 4)?,
    };
    if clr_dir.rva == 0 || clr_dir.size < 72 {
        return Ok(None);
    }

    let mut sec_off = opt_off + hdr.coff_header.size_of_optional_header as usize;
    let goblin_sections = hdr
        .coff_header
        .sections(data, &mut sec_off)
        .context("goblin: section table")?;
    let size_of_headers = opt.windows_fields.size_of_headers as u32;

    let rva_to_off = |rva: u32| -> Option<usize> {
        if rva < size_of_headers {
            return ((rva as usize) < data.len()).then_some(rva as usize);
        }
        let s = goblin_sections.iter().find(|s| {
            rva >= s.virtual_address
                && rva < s.virtual_address + s.size_of_raw_data.max(s.virtual_size)
        })?;
        let delta = rva - s.virtual_address;
        if delta >= s.size_of_raw_data {
            return None;
        }
        Some(s.pointer_to_raw_data as usize + delta as usize)
    };

    let clr_file_off = rva_to_off(clr_dir.rva)
        .ok_or_else(|| anyhow!("CLR header RVA 0x{:x} not file-backed", clr_dir.rva))?;
    let clr_bytes = data
        .get(clr_file_off..clr_file_off + 72)
        .ok_or_else(|| anyhow!("CLR header at 0x{clr_file_off:x} out of bounds"))?;
    let header = ClrHeader::from_bytes(clr_bytes)?;

    let metadata = if header.metadata.rva != 0 && header.metadata.size >= 16 {
        rva_to_off(header.metadata.rva).and_then(|moff| {
            let len = (header.metadata.size as usize)
                .min(4096)
                .min(data.len().saturating_sub(moff));
            ClrMetadata::from_bytes(&data[moff..moff + len]).ok()
        })
    } else {
        None
    };

    let machine = hdr.coff_header.machine;
    let is_dll = hdr.coff_header.characteristics & IMAGE_FILE_DLL != 0;
    let mode = header.execution_mode(machine);

    Ok(Some(ClrInspection {
        is_pe32_plus,
        machine,
        is_dll,
        clr_dir,
        header,
        metadata,
        mode,
    }))
}

/// Strip the x86-64 ReadyToRun (`ManagedNativeHeader`) directory from a `.NET` assembly in-place
/// so that a native macOS ARM64 CoreCLR (`libcoreclr.dylib`) JITs the portable CIL bytecode
/// instead of rejecting the x86-64 precompiled code. Returns `Ok(true)` if an R2R header was
/// present and stripped.
pub fn strip_ready_to_run_header(data: &mut [u8]) -> Result<bool> {
    let hdr = goblin::pe::header::Header::parse(data).context("goblin: PE header")?;
    let Some(opt) = hdr.optional_header else {
        return Ok(false);
    };
    let is_pe32_plus = match opt.standard_fields.magic {
        goblin::pe::optional_header::MAGIC_64 => true,
        goblin::pe::optional_header::MAGIC_32 => false,
        _ => return Ok(false),
    };
    let pe_off = hdr.dos_header.pe_pointer as usize;
    let opt_off = pe_off + 4 + goblin::pe::header::SIZEOF_COFF_HEADER;
    let dd_base = opt_off + if is_pe32_plus { 112 } else { 96 };
    let nrva = (opt.windows_fields.number_of_rva_and_sizes as usize).min(16);
    if nrva <= IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR {
        return Ok(false);
    }
    let clr_dd_off = dd_base + IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR * 8;
    let clr_rva = rd_u32(data, clr_dd_off)?;
    let clr_size = rd_u32(data, clr_dd_off + 4)?;
    if clr_rva == 0 || clr_size < 72 {
        return Ok(false);
    }

    let mut sec_off = opt_off + hdr.coff_header.size_of_optional_header as usize;
    let goblin_sections = hdr
        .coff_header
        .sections(data, &mut sec_off)
        .context("goblin: section table")?;
    let size_of_headers = opt.windows_fields.size_of_headers as u32;
    let clr_file_off = if clr_rva < size_of_headers {
        clr_rva as usize
    } else {
        let Some(s) = goblin_sections.iter().find(|s| {
            clr_rva >= s.virtual_address
                && clr_rva < s.virtual_address + s.size_of_raw_data.max(s.virtual_size)
        }) else {
            return Ok(false);
        };
        let delta = clr_rva - s.virtual_address;
        if delta >= s.size_of_raw_data {
            return Ok(false);
        }
        s.pointer_to_raw_data as usize + delta as usize
    };
    if clr_file_off + 72 > data.len() {
        return Ok(false);
    }
    let mnh_rva = rd_u32(data, clr_file_off + 64)?;
    let mnh_size = rd_u32(data, clr_file_off + 68)?;
    if mnh_rva == 0 && mnh_size == 0 {
        return Ok(false);
    }
    // Clear COMIMAGE_FLAGS_IL_LIBRARY (0x4) and ensure COMIMAGE_FLAGS_ILONLY (0x1) is set
    let mut flags = rd_u32(data, clr_file_off + 16)?;
    flags &= !COMIMAGE_FLAGS_IL_LIBRARY;
    flags |= COMIMAGE_FLAGS_ILONLY;
    data[clr_file_off + 16..clr_file_off + 20].copy_from_slice(&flags.to_le_bytes());
    // Zero ManagedNativeHeader (offset +64..+72)
    data[clr_file_off + 64..clr_file_off + 72].fill(0);
    Ok(true)
}

/// Find the `RUNTIME_FUNCTION` containing `rva` in a table sorted by begin RVA
/// (the linker guarantees sorting).
pub fn find_runtime_function(table: &[RuntimeFunction], rva: u32) -> Option<&RuntimeFunction> {
    let idx = table.partition_point(|f| f.begin_rva <= rva);
    if idx == 0 {
        return None;
    }
    let f = &table[idx - 1];
    (rva < f.end_rva).then_some(f)
}

/// Apply base relocations to a mapped image in-place.
///
/// `image` must be the full mapped image (offset == RVA); `delta` is
/// `actual_base - preferred_base`.
pub fn apply_relocations(image: &mut [u8], relocs: &[BaseReloc], delta: i64) -> Result<usize> {
    let mut n = 0;
    for r in relocs {
        let o = r.rva as usize;
        match r.kind {
            IMAGE_REL_BASED_DIR64 => {
                let b = image
                    .get_mut(o..o + 8)
                    .ok_or_else(|| anyhow!("DIR64 reloc out of bounds at 0x{o:x}"))?;
                let v = u64::from_le_bytes(b.try_into().unwrap()).wrapping_add(delta as u64);
                b.copy_from_slice(&v.to_le_bytes());
            }
            IMAGE_REL_BASED_HIGHLOW => {
                let b = image
                    .get_mut(o..o + 4)
                    .ok_or_else(|| anyhow!("HIGHLOW reloc out of bounds at 0x{o:x}"))?;
                let v = u32::from_le_bytes(b.try_into().unwrap()).wrapping_add(delta as u32);
                b.copy_from_slice(&v.to_le_bytes());
            }
            k => bail!("unsupported base relocation type {k} at rva 0x{o:x}"),
        }
        n += 1;
    }
    Ok(n)
}

/// Normalize a DLL name for comparison: lowercase, strip directory, ensure
/// `.dll` extension when none is given (LoadLibrary semantics).
pub fn normalize_dll_name(name: &str) -> String {
    let base = name.rsplit(['\\', '/']).next().unwrap_or(name);
    let mut s = base.to_ascii_lowercase();
    if !s.contains('.') {
        s.push_str(".dll");
    } else if s.ends_with('.') {
        s.pop();
    }
    s
}

/// Unwind info header (`UNWIND_INFO`) — used by the exception/unwind code.
#[derive(Debug, Clone, Copy)]
pub struct UnwindInfoHeader {
    pub version: u8,
    pub flags: u8,
    pub size_of_prolog: u8,
    pub count_of_codes: u8,
    pub frame_register: u8,
    pub frame_offset: u8,
}

impl UnwindInfoHeader {
    pub fn parse(b: &[u8]) -> Option<Self> {
        if b.len() < 4 {
            return None;
        }
        Some(UnwindInfoHeader {
            version: b[0] & 7,
            flags: b[0] >> 3,
            size_of_prolog: b[1],
            count_of_codes: b[2],
            frame_register: b[3] & 0xf,
            frame_offset: b[3] >> 4,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    pub(crate) fn game_dir() -> Option<PathBuf> {
        let p = std::env::var_os("PORT_GAME_DIR")
            .or_else(|| std::env::var_os("AW2_GAME_DIR"))
            .filter(|v| !v.is_empty())
            .map(PathBuf::from)?;
        p.join("AlanWake2.exe").exists().then_some(p)
    }

    #[test]
    fn normalize() {
        assert_eq!(normalize_dll_name("KERNEL32"), "kernel32.dll");
        assert_eq!(normalize_dll_name("C:\\Windows\\System32\\USER32.dll"), "user32.dll");
        assert_eq!(normalize_dll_name("foo."), "foo");
    }

    #[test]
    fn runtime_function_search() {
        let t = [
            RuntimeFunction { begin_rva: 0x10, end_rva: 0x20, unwind_rva: 1 },
            RuntimeFunction { begin_rva: 0x30, end_rva: 0x40, unwind_rva: 2 },
        ];
        assert_eq!(find_runtime_function(&t, 0x15).unwrap().unwind_rva, 1);
        assert!(find_runtime_function(&t, 0x25).is_none());
        assert_eq!(find_runtime_function(&t, 0x30).unwrap().unwind_rva, 2);
        assert!(find_runtime_function(&t, 0x5).is_none());
    }

    #[test]
    fn parse_game_exe() {
        let Some(dir) = game_dir() else {
            eprintln!("skipping: game dir not found");
            return;
        };
        let data = std::fs::read(dir.join("AlanWake2.exe")).unwrap();
        let pe = Pe::parse(&data).unwrap();
        assert_eq!(pe.headers.image_base, 0x1_4000_0000);
        assert_eq!(pe.headers.entry_rva, 0x0183_5330);
        assert!(!pe.headers.is_dll());
        let imps = pe.imports().unwrap();
        assert_eq!(imps.len(), 48);
        let total: usize = imps.iter().map(|d| d.entries.len()).sum();
        assert_eq!(total, 1088);
        assert!(imps.iter().any(|d| d.dll.eq_ignore_ascii_case("KERNEL32.dll")));
        let ex = pe.exports().unwrap();
        assert_eq!(ex.entries.len(), 447);
        assert!(pe.relocations().unwrap().len() > 1000);
        assert_eq!(pe.runtime_functions().unwrap().len(), 142429);
        let _ = pe.tls().unwrap();
        let _ = pe.delay_imports().unwrap();
    }

    #[test]
    fn parse_all_game_dlls() {
        let Some(dir) = game_dir() else {
            eprintln!("skipping: game dir not found");
            return;
        };
        for e in std::fs::read_dir(&dir).unwrap() {
            let p = e.unwrap().path();
            if p.extension().map_or(true, |x| !x.eq_ignore_ascii_case("dll")) {
                continue;
            }
            let data = std::fs::read(&p).unwrap();
            let pe = Pe::parse(&data).unwrap_or_else(|e| panic!("{}: {e:#}", p.display()));
            pe.imports().unwrap_or_else(|e| panic!("{}: {e:#}", p.display()));
            pe.delay_imports().unwrap_or_else(|e| panic!("{}: {e:#}", p.display()));
            pe.exports().unwrap_or_else(|e| panic!("{}: {e:#}", p.display()));
            pe.relocations().unwrap_or_else(|e| panic!("{}: {e:#}", p.display()));
            pe.tls_callback_rvas().unwrap_or_else(|e| panic!("{}: {e:#}", p.display()));
        }
    }

    #[test]
    fn clr_header_and_metadata_and_r2r_strip() {
        let mut bsjb = Vec::new();
        bsjb.extend_from_slice(&CLR_METADATA_SIGNATURE_BSJB.to_le_bytes());
        bsjb.extend_from_slice(&1u16.to_le_bytes()); // Major
        bsjb.extend_from_slice(&1u16.to_le_bytes()); // Minor
        bsjb.extend_from_slice(&0u32.to_le_bytes()); // Reserved
        let ver = b"v4.0.30319\0\0"; // 12 bytes (4-byte aligned)
        bsjb.extend_from_slice(&(ver.len() as u32).to_le_bytes());
        bsjb.extend_from_slice(ver);
        bsjb.extend_from_slice(&0u16.to_le_bytes()); // Flags
        bsjb.extend_from_slice(&2u16.to_le_bytes()); // Streams = 2
        // Stream 0: "#~"
        bsjb.extend_from_slice(&0x60u32.to_le_bytes());
        bsjb.extend_from_slice(&0x100u32.to_le_bytes());
        bsjb.extend_from_slice(b"#~\0\0");
        // Stream 1: "#Strings"
        bsjb.extend_from_slice(&0x160u32.to_le_bytes());
        bsjb.extend_from_slice(&0x80u32.to_le_bytes());
        bsjb.extend_from_slice(b"#Strings\0\0\0\0");

        let meta = ClrMetadata::from_bytes(&bsjb).unwrap();
        assert_eq!(meta.version_string, "v4.0.30319");
        assert_eq!(meta.streams, vec!["#~", "#Strings"]);

        let mut cor20 = [0u8; 72];
        cor20[0..4].copy_from_slice(&72u32.to_le_bytes());
        cor20[4..6].copy_from_slice(&2u16.to_le_bytes());
        cor20[6..8].copy_from_slice(&5u16.to_le_bytes());
        cor20[8..12].copy_from_slice(&0x2080u32.to_le_bytes());
        cor20[12..16].copy_from_slice(&(bsjb.len() as u32).to_le_bytes());
        cor20[16..20].copy_from_slice(&(COMIMAGE_FLAGS_ILONLY | COMIMAGE_FLAGS_IL_LIBRARY).to_le_bytes());
        cor20[20..24].copy_from_slice(&0x0600_0001u32.to_le_bytes());
        // ManagedNativeHeader (ReadyToRun) at +64..+72
        cor20[64..68].copy_from_slice(&0x3000u32.to_le_bytes());
        cor20[68..72].copy_from_slice(&0x400u32.to_le_bytes());

        let hdr = ClrHeader::from_bytes(&cor20).unwrap();
        assert!(hdr.is_il_only());
        assert!(hdr.is_ready_to_run());
        assert_eq!(
            hdr.execution_mode(IMAGE_FILE_MACHINE_AMD64),
            ClrExecutionMode::ReadyToRunHybrid
        );
    }

    #[test]
    fn parse_volatile_metadata_synthetic_file_and_mapped() {
        let image_base = 0x1_8000_0000u64;
        let mut img = vec![0u8; 0x3000];
        let w16 = |b: &mut [u8], o: usize, v: u16| b[o..o + 2].copy_from_slice(&v.to_le_bytes());
        let w32 = |b: &mut [u8], o: usize, v: u32| b[o..o + 4].copy_from_slice(&v.to_le_bytes());
        let w64 = |b: &mut [u8], o: usize, v: u64| b[o..o + 8].copy_from_slice(&v.to_le_bytes());

        // Minimal valid PE32+ with .text @ 0x1000..0x2000 and .rdata @ 0x2000..0x3000.
        w16(&mut img, 0, 0x5A4D);
        w32(&mut img, 0x3C, 0x80);
        w32(&mut img, 0x80, 0x0000_4550);
        w16(&mut img, 0x84, IMAGE_FILE_MACHINE_AMD64);
        w16(&mut img, 0x86, 2); // 2 sections
        w16(&mut img, 0x94, 0xF0); // SizeOfOptionalHeader
        let opt = 0x98;
        w16(&mut img, opt, 0x20B);
        w32(&mut img, opt + 16, 0x1000); // entry
        w64(&mut img, opt + 24, image_base);
        w32(&mut img, opt + 32, 0x1000); // SectionAlignment
        w32(&mut img, opt + 36, 0x200); // FileAlignment
        w32(&mut img, opt + 56, 0x3000); // SizeOfImage
        w32(&mut img, opt + 60, 0x400); // SizeOfHeaders
        w32(&mut img, opt + 108, 16); // NumberOfRvaAndSizes
        // DataDirectory[10] (LoadConfig) @ 0x2000, size 0x140
        w32(&mut img, opt + 112 + 10 * 8, 0x2000);
        w32(&mut img, opt + 112 + 10 * 8 + 4, 0x140);

        let sec0 = opt + 0xF0;
        img[sec0..sec0 + 5].copy_from_slice(b".text");
        w32(&mut img, sec0 + 8, 0x1000);
        w32(&mut img, sec0 + 12, 0x1000);
        w32(&mut img, sec0 + 16, 0x1000);
        w32(&mut img, sec0 + 20, 0x1000);
        w32(&mut img, sec0 + 36, IMAGE_SCN_CNT_CODE | IMAGE_SCN_MEM_EXECUTE | IMAGE_SCN_MEM_READ);

        let sec1 = sec0 + 40;
        img[sec1..sec1 + 6].copy_from_slice(b".rdata");
        w32(&mut img, sec1 + 8, 0x1000);
        w32(&mut img, sec1 + 12, 0x2000);
        w32(&mut img, sec1 + 16, 0x1000);
        w32(&mut img, sec1 + 20, 0x2000);
        w32(&mut img, sec1 + 36, IMAGE_SCN_MEM_READ);

        // IMAGE_LOAD_CONFIG_DIRECTORY64 @ 0x2000
        w32(&mut img, 0x2000, 0x140);
        w32(&mut img, 0x2000 + 0x90, 0x0000_0100); // GuardFlags
        w64(&mut img, 0x2000 + 0x100, image_base + 0x2200); // VolatileMetadataPointer

        // IMAGE_VOLATILE_METADATA @ 0x2200
        w32(&mut img, 0x2200, 24);
        w32(&mut img, 0x2204, 0x8002_8002);
        w32(&mut img, 0x2208, 0x2300); // VolatileAccessTable RVA
        w32(&mut img, 0x220C, 12); // 3 u32 entries (unsorted input to verify sorting)
        w32(&mut img, 0x2210, 0x2400); // VolatileInfoRangeTable RVA
        w32(&mut img, 0x2214, 16); // 2 (start, len) entries

        w32(&mut img, 0x2300, 0x1040);
        w32(&mut img, 0x2304, 0x1010);
        w32(&mut img, 0x2308, 0x1040); // duplicate

        w32(&mut img, 0x2400, 0x1000);
        w32(&mut img, 0x2404, 0x80); // [0x1000, 0x1080)
        w32(&mut img, 0x2408, 0x1200);
        w32(&mut img, 0x240C, 0x40); // [0x1200, 0x1240)

        for layout in [Layout::File, Layout::Mapped] {
            let pe = Pe::parse_with_layout(&img, layout).unwrap();
            let lc = pe.load_config().unwrap().unwrap();
            assert_eq!(lc.size, 0x140);
            assert_eq!(lc.guard_flags, 0x0000_0100);
            assert_eq!(lc.volatile_metadata_va, image_base + 0x2200);

            let vm = pe.volatile_metadata().unwrap().unwrap();
            assert_eq!(vm.version, 0x8002_8002);
            assert_eq!(
                vm.ranges,
                vec![
                    VolatileRange { start_rva: 0x1000, end_rva: 0x1080 },
                    VolatileRange { start_rva: 0x1200, end_rva: 0x1240 },
                ]
            );
            assert_eq!(vm.access_rvas, vec![0x1010, 0x1040]);
            assert!(vm.can_elide_tso(0x1000));
            assert!(!vm.can_elide_tso(0x1010));
            assert!(vm.can_elide_tso(0x1014));
            assert!(!vm.can_elide_tso(0x1040));
            assert!(!vm.can_elide_tso(0x1080)); // outside range
            assert!(!vm.can_elide_tso(0x1100)); // uncovered gap
            assert!(vm.can_elide_tso(0x1200));
            assert!(!vm.can_elide_tso(0x1240));
        }

        // Also test relocated mapped VA (VolatileMetadataPointer relocated to buffer address).
        let mapped_base = img.as_ptr() as u64;
        w64(&mut img, 0x2000 + 0x100, mapped_base + 0x2200);
        let pe_reloc = Pe::parse_mapped(&img).unwrap();
        let vm_reloc = pe_reloc.volatile_metadata().unwrap().unwrap();
        assert!(vm_reloc.can_elide_tso(0x1000));
        assert!(!vm_reloc.can_elide_tso(0x1010));
    }
}
