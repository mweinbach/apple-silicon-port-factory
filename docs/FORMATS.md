# Northlight Engine Asset & Shader Formats (Alan Wake 2)

> **Scope:** Alan Wake 2–specific Northlight Engine binary formats parsed by
> `games/alan-wake-2/crates/aw2-data` (and `crates/dxil-metal --features tools` for `.binrfx`
> prewarming). Universal crates do not depend on these formats.

Reverse-engineered from `AlanWake2.exe` (`v1.2.8` / `v1.2.10`) and verified against all `.rmdtoc`,
`.rmdblob`, and `.binrfx` files in a retail Alan Wake 2 install (`$AW2_GAME_DIR`).

---

## 1. `.rmdtoc` Archive Index (`COTR` v2)

### 1.1 Outer Header (`0x58` = 88 bytes, little-endian)

Every `.rmdtoc` file starts with an unencrypted 88-byte (`22 × uint32`) header:

| Offset | Type | Field | Description |
| :--- | :--- | :--- | :--- |
| `0x00` | `uint32` | `magic` | `'COTR'` (`0x52544F43`, `"RTOC"` reversed) |
| `0x04` | `uint32` | `version` | `2` (`0x00000002`) |
| `0x08` | `uint32` | `toc_chunks_offset` | File offset of the self-chunk descriptor table (always `0x58`) |
| `0x0C` | `uint32` | `toc_chunks_size` | Byte length of the self-chunk descriptor table |
| `0x10` | `uint32` | `blobs_offset` | Offset in decompressed `toc_buf` to the `BlobEntry` array |
| `0x14` | `uint32` | `blobs_count` | Number of `BlobEntry` records (`16` bytes each) |
| `0x18` | `uint32` | `dirs_offset` | Offset in decompressed `toc_buf` to the `DirEntry` array |
| `0x1C` | `uint32` | `dirs_count` | Number of `DirEntry` records (`28` bytes each) |
| `0x20` | `uint32` | `files_offset` | Offset in decompressed `toc_buf` to the `FileEntry` array |
| `0x24` | `uint32` | `files_count` | Number of `FileEntry` records (`32` bytes each) |
| `0x28` | `uint32` | `strings_offset` | Offset in decompressed `toc_buf` to the UTF-8 string pool |
| `0x2C` | `uint32` | `strings_size` | Byte length of the string pool |
| `0x30` | `uint32` | `import_types_offset` | Offset in decompressed `toc_buf` to the `ImportTypeEntry` array |
| `0x34` | `uint32` | `import_types_count` | Number of `ImportTypeEntry` records (`8` bytes each) |
| `0x38` | `uint32` | `metadata_offset` | Offset in decompressed `toc_buf` to serialized resource metadata |
| `0x3C` | `uint32` | `metadata_size` | Byte length of the metadata region |
| `0x40` | `uint32[4]` | `reserved` | Zero (`0, 0, 0, 0`) |
| `0x50` | `uint32` | `chunks_offset` | Offset in decompressed `toc_buf` to the file chunk descriptor stream |
| `0x54` | `uint32` | `chunks_size` | Byte length of the file chunk descriptor stream |

### 1.2 Chunk Descriptor Format (TOC Self-Chunks & File Chunks)

Both `toc_chunks` (at offset `0x58` in `.rmdtoc`) and file `chunks` (inside decompressed `toc_buf`)
use the same variable-length record (`VA 0x14243772a`):

```c
struct NorthlightChunkDesc {
    uint64_t packed_loc;        // +0x00: [0..7] flags, [8..23] blob_index, [24..63] byte_offset
    uint32_t uncompressed_size; // +0x08: Decompressed size (up to 1 MiB = 0x100000)
    uint32_t compressed_size;   // +0x0C: Compressed size in .rmdblob (or .rmdtoc for self-chunks)
    uint64_t extra[];           // +0x10: (flags & 0x0F) extra u64 words; total size = ((flags & 0x0F) + 2) * 8
};
```

- **`flags` (`packed_loc & 0xFF`):** `0x00` = raw uncompressed bytes (`compressed_size == uncompressed_size`); `0x10` = raw **LZ4 block** (`LZ4_decompress_safe` at `VA 0x14246b9e0`).
- **`blob_index` (`(packed_loc >> 8) & 0xFFFF`):** Index into `BlobEntry` (`0` for `.rmdtoc` self-chunks).
- **`byte_offset` (`packed_loc >> 24`):** 40-bit byte offset in the target `.rmdblob` (or in `.rmdtoc` for self-chunks, where payload begins at `0x1000`).

### 1.3 Decompressed `toc_buf` Tables

After decompressing and concatenating the self-chunks at `0x58 .. 0x58 + toc_chunks_size` into
`toc_buf`:

1. **`BlobEntry` (`16` bytes, `<4I`):** `name_offset`, `name_length` (e.g. `"base-en-000.rmdblob"`),
   `unknown_08` (`0`), `unknown_0c` (`0`).
2. **`DirEntry` (`28` bytes, `<7I`):** `parent_dir_index` (`0` for root), `first_subdir_index`,
   `subdir_count`, `first_file_index`, `file_count`, `name_offset`, `name_length` (`Dir[0]` = `""`).
3. **`FileEntry` (`32` bytes, `<8I`):** `chunk_stream_offset` (relative to `chunks_offset`),
   `chunk_stream_size`, `parent_dir_index`, `name_offset`, `name_length`, `file_size`
   (uncompressed total), `meta_offset` (relative to `metadata_offset`), `meta_size`.
4. **`ImportTypeEntry` (`8` bytes, `<2I`):** `name_offset`, `name_length` (C++ RTTI serializer
   name, e.g. `r::FileInfoMetadata`, `rend::MaterialMetadata`, `content::LuaScriptMetadata`).

---

## 2. `.binrfx` Shader Bundle (`RFX ` v58)

All shader bundles in `$AW2_GAME_DIR/data/shaders/build/pc_dx12/*.binrfx` use Northlight's `RFX `
v58 (`0x3A`) container (`VA 0x1420a1000+`).

### 2.1 Header & 6 Stage Arrays

```c
struct BinRfxHeader {
    char     magic[4];       // "RFX " (0x20584652)
    uint32_t version;        // 58 (0x0000003A)
    uint32_t name_len;       // Length of shader family name
    char     name[name_len]; // e.g. "blur", "CoherentGT", "rt_diffusegi"
};
```

Followed immediately by **6 sequential stage arrays** (`stage = 0..5`: Vertex, Pixel, Compute,
Geometry/Hull/Domain, Amplification, Mesh/Raytracing). Each stage starts with `uint32 program_count`
followed by `program_count` `RfxStageProgram` records:

1. `uint32 entry_point_len` + `char entry_point[entry_point_len]` (`"mainVS"`, `"mainPS"`, `"mainCS"`, …)
2. `uint64 bytecode_size` + `uint8 bytecode[bytecode_size]` — standard **DXBC** (`"DXBC"`) wrapping
   **DXIL** (`"DXIL"`, SM 6.5/6.6+ LLVM bitcode `'BC\xc0\xde'`) plus `HASH`/`SFI0`/`PSV0`/`RDAT`.
3. `uint8 has_reflection` — if non-zero:
   - `uint32 reflection_flags`
   - **8 resource binding groups** (CBV/SRV/UAV/Sampler/AccelerationStructure/etc.): each has
     `uint32 count` entries of `(String name, uint32[5] binding_info, uint8 flags)`.
   - **2 constant buffer reflection lists**: each has `uint32 cb_count` entries of
     `(uint32 slot, uint32 space, String name1, String name2, uint32 var_count, RfxCbVar vars[], uint32 byte_size)`.
   - `uint8 trailing_refl_byte`
4. **Input layout & permutation trailer:**
   - `uint32 input_layout_mask`
   - `uint8 input_elem_count` + `uint8[4] elems[input_elem_count]`
   - `uint8 stream_count` + `uint8 streams[stream_count]`
   - `uint64 shader_hash`
   - `uint32 flags_e0`

### 2.2 Technique, Root Signature & Permutation Tables

After all 6 stage arrays:

1. **Technique / pass table (`cnt_a8`):** `uint32 technique_count`, each with `String name`,
   `uint32 pass_count` (`52` bytes/pass), `uint32 state_count` (`20` bytes/state), and `16` bytes
   of pipeline flags (`uint32, uint32, uint64`).
2. **Embedded shared root signature (`s80`):** `uint64 root_sig_size`; if non-zero,
   `uint8 dxbc_root_sig[root_sig_size]`, `String root_sig_name`, `uint64 root_sig_hash`.
3. **Permutation / macro table (`cnt_b8`):** `uint32 permutation_count`, each with
   `String define_name`, `uint64 bitmask_lo`, `uint64 bitmask_hi`.
