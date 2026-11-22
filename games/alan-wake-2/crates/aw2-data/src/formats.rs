use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

#[inline]
fn read_u32_le(buf: &[u8], off: usize) -> u32 {
    u32::from_le_bytes(buf[off..off + 4].try_into().unwrap())
}

#[inline]
fn read_u64_le(buf: &[u8], off: usize) -> u64 {
    u64::from_le_bytes(buf[off..off + 8].try_into().unwrap())
}

/// Pure-Rust high-speed LZ4 block decompressor matching Northlight's `LZ4_decompress_safe` (VA 0x14246b9e0)
pub fn lz4_decompress_block(src: &[u8], uncompressed_size: usize) -> Result<Vec<u8>, String> {
    let mut dst = Vec::with_capacity(uncompressed_size);
    let mut pos = 0usize;
    let n = src.len();

    while pos < n {
        let token = src[pos];
        pos += 1;

        let mut lit_len = (token >> 4) as usize;
        if lit_len == 15 {
            while pos < n {
                let b = src[pos] as usize;
                pos += 1;
                lit_len += b;
                if b != 255 {
                    break;
                }
            }
        }

        if pos + lit_len > n {
            return Err(format!("LZ4 literal overflow at pos {}", pos));
        }
        dst.extend_from_slice(&src[pos..pos + lit_len]);
        pos += lit_len;

        if pos >= n {
            break;
        }

        if pos + 2 > n {
            return Err("LZ4 truncated match offset".to_string());
        }
        let offset = (src[pos] as usize) | ((src[pos + 1] as usize) << 8);
        pos += 2;
        if offset == 0 || offset > dst.len() {
            return Err(format!("LZ4 invalid match offset {} (dst len {})", offset, dst.len()));
        }

        let mut match_len = ((token & 0x0F) as usize) + 4;
        if (token & 0x0F) == 15 {
            while pos < n {
                let b = src[pos] as usize;
                pos += 1;
                match_len += b;
                if b != 255 {
                    break;
                }
            }
        }

        let start = dst.len() - offset;
        if offset >= match_len {
            dst.extend_from_within(start..start + match_len);
        } else {
            for i in 0..match_len {
                let b = dst[start + i];
                dst.push(b);
            }
        }
    }

    if dst.len() != uncompressed_size {
        return Err(format!(
            "LZ4 size mismatch: got {}, expected {}",
            dst.len(),
            uncompressed_size
        ));
    }
    Ok(dst)
}

/// Pure-Rust LZ4 block compressor compatible with `lz4_decompress_block` and Northlight's `LZ4_decompress_safe`.
pub fn lz4_compress_block(src: &[u8]) -> Vec<u8> {
    let n = src.len();
    if n == 0 {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(n + (n / 255) + 16);
    if n < 13 {
        emit_lz4_literals(&mut out, src);
        return out;
    }

    const HASH_LOG: u32 = 14;
    const HASH_SIZE: usize = 1 << HASH_LOG;
    let mut table = vec![0u32; HASH_SIZE];
    let hash4 = |p: usize| -> usize {
        let v = u32::from_le_bytes(src[p..p + 4].try_into().unwrap());
        ((v.wrapping_mul(2654435761u32)) >> (32 - HASH_LOG)) as usize
    };

    let mflimit = n - 12;
    let match_limit = n - 5;
    let mut anchor = 0usize;
    let mut pos = 1usize;

    while pos < mflimit {
        let h = hash4(pos);
        let cand = table[h] as usize;
        table[h] = pos as u32;

        if cand < pos
            && pos - cand <= 65535
            && src[cand..cand + 4] == src[pos..pos + 4]
        {
            let mut mlen = 4usize;
            while pos + mlen < match_limit && src[cand + mlen] == src[pos + mlen] {
                mlen += 1;
            }
            let lit_len = pos - anchor;
            let token_pos = out.len();
            let lit_tok = lit_len.min(15) as u8;
            let match_tok = (mlen - 4).min(15) as u8;
            out.push((lit_tok << 4) | match_tok);
            if lit_len >= 15 {
                let mut rem = lit_len - 15;
                while rem >= 255 {
                    out.push(255);
                    rem -= 255;
                }
                out.push(rem as u8);
            }
            out.extend_from_slice(&src[anchor..pos]);
            let offset = (pos - cand) as u16;
            out.extend_from_slice(&offset.to_le_bytes());
            if mlen - 4 >= 15 {
                let mut rem = (mlen - 4) - 15;
                while rem >= 255 {
                    out.push(255);
                    rem -= 255;
                }
                out.push(rem as u8);
            }
            let _ = token_pos;
            pos += mlen;
            anchor = pos;
            if pos >= mflimit {
                break;
            }
            table[hash4(pos - 2)] = (pos - 2) as u32;
            continue;
        }
        pos += 1;
    }

    if anchor < n {
        emit_lz4_literals(&mut out, &src[anchor..]);
    }
    out
}

fn emit_lz4_literals(out: &mut Vec<u8>, lits: &[u8]) {
    let lit_len = lits.len();
    let tok = (lit_len.min(15) as u8) << 4;
    out.push(tok);
    if lit_len >= 15 {
        let mut rem = lit_len - 15;
        while rem >= 255 {
            out.push(255);
            rem -= 255;
        }
        out.push(rem as u8);
    }
    out.extend_from_slice(lits);
}

#[derive(Debug, Clone)]
pub struct ChunkDesc {
    pub flags: u8,
    pub blob_index: u16,
    pub data_offset: u64,
    pub uncompressed_size: u32,
    pub compressed_size: u32,
}

pub fn parse_chunks(buf: &[u8]) -> Vec<ChunkDesc> {
    let mut pos = 0usize;
    let mut out = Vec::new();
    while pos + 16 <= buf.len() {
        let q0 = read_u64_le(buf, pos);
        let uncompressed_size = read_u32_le(buf, pos + 8);
        let compressed_size = read_u32_le(buf, pos + 12);
        let flags = (q0 & 0xFF) as u8;
        let blob_index = ((q0 >> 8) & 0xFFFF) as u16;
        let data_offset = q0 >> 24;
        let entry_len = (((flags & 0x0F) as usize) + 2) * 8;
        out.push(ChunkDesc {
            flags,
            blob_index,
            data_offset,
            uncompressed_size,
            compressed_size,
        });
        pos += entry_len;
    }
    out
}

#[inline]
fn read_u16_le(buf: &[u8], off: usize) -> u16 {
    u16::from_le_bytes(buf[off..off + 2].try_into().unwrap())
}

#[inline]
fn read_f32_le(buf: &[u8], off: usize) -> f32 {
    f32::from_le_bytes(buf[off..off + 4].try_into().unwrap())
}

#[derive(Debug, Clone, Copy)]
pub struct MeshLodInfo {
    pub stream_offset: u32,
    pub vb0_size: u32,
    pub vb1_size: u32,
    pub ib_size: u32,
    pub cluster_size: u32,
    pub extra_size: u32,
}

#[derive(Debug, Clone)]
pub struct MeshMeta {
    pub aabb_min: [f32; 3],
    pub aabb_max: [f32; 3],
    pub sphere_center: [f32; 3],
    pub sphere_radius: f32,
    pub header_size: u32,
    pub lods: Vec<MeshLodInfo>,
}

#[derive(Debug, Clone)]
pub struct TocFileEntry {
    pub path: String,
    pub file_size: u32,
    pub chunk_stream_offset: u32,
    pub chunk_stream_size: u32,
    pub meta_offset: u32,
    pub meta_size: u32,
    pub resource_id: u64,
    pub dependencies: Vec<u64>,
    pub mesh_meta: Option<MeshMeta>,
}

#[derive(Debug, Clone)]
pub struct RmdToc {
    pub toc_path: PathBuf,
    pub version: u32,
    pub blobs: Vec<String>,
    pub import_types: Vec<String>,
    pub files: Vec<TocFileEntry>,
    pub meta_buf: Vec<u8>,
    pub chunks_buf: Vec<u8>,
}

impl RmdToc {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let toc_path = path.as_ref().to_path_buf();
        let data = fs::read(&toc_path).map_err(|e| format!("Failed to read {:?}: {}", toc_path, e))?;
        if data.len() < 0x58 {
            return Err("File too small for .rmdtoc header".to_string());
        }

        let magic = read_u32_le(&data, 0);
        let version = read_u32_le(&data, 4);
        if magic != 0x52544F43 {
            return Err(format!("Invalid .rmdtoc magic: 0x{:08x} (expected COTR)", magic));
        }

        let tc_off = read_u32_le(&data, 0x08) as usize;
        let tc_sz = read_u32_le(&data, 0x0C) as usize;
        let b_off = read_u32_le(&data, 0x10) as usize;
        let b_cnt = read_u32_le(&data, 0x14) as usize;
        let d_off = read_u32_le(&data, 0x18) as usize;
        let d_cnt = read_u32_le(&data, 0x1C) as usize;
        let f_off = read_u32_le(&data, 0x20) as usize;
        let f_cnt = read_u32_le(&data, 0x24) as usize;
        let s_off = read_u32_le(&data, 0x28) as usize;
        let s_sz = read_u32_le(&data, 0x2C) as usize;
        let it_off = read_u32_le(&data, 0x30) as usize;
        let it_cnt = read_u32_le(&data, 0x34) as usize;
        let m_buf_off = read_u32_le(&data, 0x38) as usize;
        let m_buf_sz = read_u32_le(&data, 0x3C) as usize;
        let c_off = read_u32_le(&data, 0x50) as usize;
        let c_sz = read_u32_le(&data, 0x54) as usize;

        let self_chunks = parse_chunks(&data[tc_off..tc_off + tc_sz]);
        let mut toc_buf = Vec::new();
        for ch in self_chunks {
            let off = ch.data_offset as usize;
            let comp = ch.compressed_size as usize;
            let uncomp = ch.uncompressed_size as usize;
            if ch.flags == 0x00 {
                toc_buf.extend_from_slice(&data[off..off + uncomp]);
            } else if ch.flags == 0x10 {
                let dec = lz4_decompress_block(&data[off..off + comp], uncomp)?;
                toc_buf.extend_from_slice(&dec);
            } else {
                return Err(format!("Unsupported TOC chunk flag: 0x{:02x}", ch.flags));
            }
        }

        let strings = &toc_buf[s_off..s_off + s_sz];
        let get_str = |off: usize, len: usize| -> String {
            String::from_utf8_lossy(&strings[off..off + len]).into_owned()
        };

        let mut blobs = Vec::with_capacity(b_cnt);
        for i in 0..b_cnt {
            let base = b_off + i * 16;
            let no = read_u32_le(&toc_buf, base) as usize;
            let nl = read_u32_le(&toc_buf, base + 4) as usize;
            blobs.push(get_str(no, nl));
        }

        let mut import_types = Vec::with_capacity(it_cnt);
        for i in 0..it_cnt {
            let base = it_off + i * 8;
            let no = read_u32_le(&toc_buf, base) as usize;
            let nl = read_u32_le(&toc_buf, base + 4) as usize;
            import_types.push(get_str(no, nl));
        }

        let mut dirs: Vec<(usize, String)> = Vec::with_capacity(d_cnt);
        for i in 0..d_cnt {
            let base = d_off + i * 28;
            let parent_idx = read_u32_le(&toc_buf, base) as usize;
            let no = read_u32_le(&toc_buf, base + 20) as usize;
            let nl = read_u32_le(&toc_buf, base + 24) as usize;
            let name = if i == 0 { String::new() } else { get_str(no, nl) };
            dirs.push((parent_idx, name));
        }

        let build_dir_path = |mut cur: usize| -> String {
            let mut parts = Vec::new();
            while cur > 0 && cur < dirs.len() {
                let (p, ref name) = dirs[cur];
                if !name.is_empty() {
                    parts.push(name.as_str());
                }
                cur = p;
            }
            parts.reverse();
            parts.join("/")
        };

        let meta_buf = if m_buf_off + m_buf_sz <= toc_buf.len() {
            toc_buf[m_buf_off..m_buf_off + m_buf_sz].to_vec()
        } else {
            Vec::new()
        };

        let mut files = Vec::with_capacity(f_cnt);
        for i in 0..f_cnt {
            let base = f_off + i * 32;
            let cs_off = read_u32_le(&toc_buf, base);
            let cs_sz = read_u32_le(&toc_buf, base + 4);
            let p_dir = read_u32_le(&toc_buf, base + 8) as usize;
            let no = read_u32_le(&toc_buf, base + 12) as usize;
            let nl = read_u32_le(&toc_buf, base + 16) as usize;
            let f_sz = read_u32_le(&toc_buf, base + 20);
            let m_off = read_u32_le(&toc_buf, base + 24);
            let m_sz = read_u32_le(&toc_buf, base + 28);

            let dir_str = build_dir_path(p_dir);
            let fname = get_str(no, nl);
            let path = if dir_str.is_empty() {
                fname
            } else {
                format!("{}/{}", dir_str, fname)
            };

            let mut resource_id = 0u64;
            let mut dependencies = Vec::new();
            let mut mesh_meta = None;

            let ms = m_off as usize;
            let me = ms + (m_sz as usize);
            if me <= meta_buf.len() && m_sz >= 8 {
                let mb = &meta_buf[ms..me];
                if read_u32_le(mb, 0) == 0x504B4D44 {
                    let sec_cnt = read_u32_le(mb, 4) as usize;
                    let payload_base = 8 + sec_cnt * 8;
                    if payload_base <= mb.len() {
                        for s in 0..sec_cnt {
                            let t_idx = read_u32_le(mb, 8 + s * 8) as usize;
                            let s_off = read_u16_le(mb, 8 + s * 8 + 4) as usize;
                            let e_off = read_u16_le(mb, 8 + s * 8 + 6) as usize;
                            if payload_base + e_off <= mb.len() && s_off <= e_off {
                                let sec = &mb[payload_base + s_off..payload_base + e_off];
                                let t_name = import_types.get(t_idx).map(|x| x.as_str()).unwrap_or("");
                                if t_name == "r::ResourceMetadata" && sec.len() >= 20 {
                                    resource_id = read_u64_le(sec, 8);
                                    let dep_cnt = read_u32_le(sec, 16) as usize;
                                    for d in 0..dep_cnt {
                                        if 20 + d * 12 + 12 <= sec.len() {
                                            let dep_rid = read_u64_le(sec, 20 + d * 12 + 4);
                                            if dep_rid != 0 {
                                                dependencies.push(dep_rid);
                                            }
                                        }
                                    }
                                } else if t_name == "rend::MeshMetadata" && sec.len() >= 0x50 {
                                    let aabb_min = [
                                        read_f32_le(sec, 0x08),
                                        read_f32_le(sec, 0x0C),
                                        read_f32_le(sec, 0x10),
                                    ];
                                    let aabb_max = [
                                        read_f32_le(sec, 0x14),
                                        read_f32_le(sec, 0x18),
                                        read_f32_le(sec, 0x1C),
                                    ];
                                    let sphere_center = [
                                        read_f32_le(sec, 0x24),
                                        read_f32_le(sec, 0x28),
                                        read_f32_le(sec, 0x2C),
                                    ];
                                    let sphere_radius = read_f32_le(sec, 0x30);
                                    let str_len = read_u32_le(sec, 0x4B) as usize;
                                    let p = 0x4F + str_len;
                                    if p + 0x24 <= sec.len() {
                                        let header_size = read_u32_le(sec, p + 0x1C);
                                        let mut lods = Vec::new();
                                        let mut l = 0usize;
                                        while l < 8 {
                                            let lb = p + 0x24 + l * 0x38;
                                            if lb + 0x20 > sec.len() {
                                                break;
                                            }
                                            let sc = read_u32_le(sec, lb + 0x04);
                                            if sc != 5 {
                                                break;
                                            }
                                            lods.push(MeshLodInfo {
                                                stream_offset: read_u32_le(sec, lb + 0x08),
                                                vb0_size: read_u32_le(sec, lb + 0x0C),
                                                vb1_size: read_u32_le(sec, lb + 0x10),
                                                ib_size: read_u32_le(sec, lb + 0x14),
                                                cluster_size: read_u32_le(sec, lb + 0x18),
                                                extra_size: read_u32_le(sec, lb + 0x1C),
                                            });
                                            l += 1;
                                        }
                                        if !lods.is_empty() {
                                            mesh_meta = Some(MeshMeta {
                                                aabb_min,
                                                aabb_max,
                                                sphere_center,
                                                sphere_radius,
                                                header_size,
                                                lods,
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            files.push(TocFileEntry {
                path,
                file_size: f_sz,
                chunk_stream_offset: cs_off,
                chunk_stream_size: cs_sz,
                meta_offset: m_off,
                meta_size: m_sz,
                resource_id,
                dependencies,
                mesh_meta,
            });
        }

        let chunks_buf = toc_buf[c_off..c_off + c_sz].to_vec();

        Ok(Self {
            toc_path,
            version,
            blobs,
            import_types,
            files,
            meta_buf,
            chunks_buf,
        })
    }

    pub fn extract_file(&self, entry: &TocFileEntry) -> Result<Vec<u8>, String> {
        let parent_dir = self.toc_path.parent().unwrap_or_else(|| Path::new("."));
        let cs_start = entry.chunk_stream_offset as usize;
        let cs_end = cs_start + (entry.chunk_stream_size as usize);
        let chunks = parse_chunks(&self.chunks_buf[cs_start..cs_end]);

        let mut out = Vec::with_capacity(entry.file_size as usize);
        for ch in chunks {
            let blob_name = self
                .blobs
                .get(ch.blob_index as usize)
                .ok_or_else(|| format!("Invalid blob index {}", ch.blob_index))?;
            let blob_path = parent_dir.join(blob_name);
            let mut f = File::open(&blob_path)
                .map_err(|e| format!("Failed to open blob {:?}: {}", blob_path, e))?;
            f.seek(SeekFrom::Start(ch.data_offset))
                .map_err(|e| e.to_string())?;
            let read_len = if ch.flags == 0x00 || ch.compressed_size == 0 {
                ch.uncompressed_size as usize
            } else {
                ch.compressed_size as usize
            };
            let mut raw = vec![0u8; read_len];
            f.read_exact(&mut raw).map_err(|e| e.to_string())?;

            if ch.flags == 0x00 {
                out.extend_from_slice(&raw);
            } else if ch.flags == 0x10 {
                let dec = lz4_decompress_block(&raw, ch.uncompressed_size as usize)?;
                out.extend_from_slice(&dec);
            } else {
                return Err(format!("Unsupported file chunk flag 0x{:02x}", ch.flags));
            }
        }
        Ok(out)
    }
}

#[derive(Debug, Clone)]
pub struct RfxShaderProgram {
    pub stage_index: usize,
    pub stage_name: &'static str,
    pub entry_point: String,
    pub bytecode_size: usize,
    pub shader_model: String,
    pub shader_hash: u64,
    pub dxbc_bytes: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct BinRfx {
    pub file_name: String,
    pub version: u32,
    pub shader_name: String,
    pub programs: Vec<RfxShaderProgram>,
    pub techniques: Vec<String>,
    pub permutations: Vec<String>,
}

impl BinRfx {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let p = path.as_ref();
        let data = fs::read(p).map_err(|e| format!("Failed to read {:?}: {}", p, e))?;
        let file_name = p.file_name().unwrap_or_default().to_string_lossy().into_owned();
        Self::from_bytes(file_name, &data)
    }

    pub fn from_bytes(file_name: impl Into<String>, data: &[u8]) -> Result<Self, String> {
        let file_name = file_name.into();
        if data.len() < 12 || &data[0..4] != b"RFX " {
            return Err("Invalid .binrfx magic (expected 'RFX ')".to_string());
        }
        let version = read_u32_le(data, 4);
        let name_len = read_u32_le(data, 8) as usize;
        if 12 + name_len > data.len() {
            return Err("Truncated .binrfx shader name".to_string());
        }
        let shader_name = String::from_utf8_lossy(&data[12..12 + name_len]).into_owned();
        let mut pos = 12 + name_len;

        let stage_names = ["Vertex", "Pixel", "Compute", "Mesh", "Amplification", "Raytracing"];
        let mut programs = Vec::new();

        for (s, &stage_name) in stage_names.iter().enumerate() {
            if pos + 4 > data.len() {
                return Err(format!("Truncated .binrfx at stage {s}"));
            }
            let cnt = read_u32_le(data, pos) as usize;
            pos += 4;
            for _ in 0..cnt {
                if pos + 4 > data.len() {
                    return Err("Truncated .binrfx entry point length".to_string());
                }
                let en = read_u32_le(data, pos) as usize;
                pos += 4;
                if pos + en + 8 > data.len() {
                    return Err("Truncated .binrfx entry point or bytecode size".to_string());
                }
                let entry_point = String::from_utf8_lossy(&data[pos..pos + en]).into_owned();
                pos += en;
                let bcs = read_u64_le(data, pos) as usize;
                pos += 8;
                if pos + bcs >= data.len() {
                    return Err("Truncated .binrfx DXBC payload".to_string());
                }
                let dxbc_bytes = data[pos..pos + bcs].to_vec();
                pos += bcs;

                let mut shader_model = "DXBC".to_string();
                if dxbc_bytes.len() >= 36 && &dxbc_bytes[0..4] == b"DXBC" {
                    let ccnt = read_u32_le(&dxbc_bytes, 28) as usize;
                    for ci in 0..ccnt {
                        if 32 + ci * 4 + 4 <= dxbc_bytes.len() {
                            let coff = read_u32_le(&dxbc_bytes, 32 + ci * 4) as usize;
                            if coff + 12 <= dxbc_bytes.len() && &dxbc_bytes[coff..coff + 4] == b"DXIL" {
                                let pv = read_u32_le(&dxbc_bytes, coff + 8);
                                shader_model = format!("SM {}.{}", (pv >> 4) & 0xF, pv & 0xF);
                            }
                        }
                    }
                }

                // Skip reflection block
                if data[pos] != 0 {
                    pos += 5;
                    for _ in 0..8 {
                        let gc = read_u32_le(data, pos) as usize;
                        pos += 4;
                        for _ in 0..gc {
                            let sn = read_u32_le(data, pos) as usize;
                            pos += 4 + sn + 21;
                        }
                    }
                    for _ in 0..2 {
                        let cbc = read_u32_le(data, pos) as usize;
                        pos += 4;
                        for _ in range_check(cbc) {
                            pos += 8;
                            let sn1 = read_u32_le(data, pos) as usize;
                            pos += 4 + sn1;
                            let sn2 = read_u32_le(data, pos) as usize;
                            pos += 4 + sn2;
                            let vc = read_u32_le(data, pos) as usize;
                            pos += 4;
                            for _ in 0..vc {
                                let vsn = read_u32_le(data, pos) as usize;
                                pos += 4 + vsn + 17;
                            }
                            pos += 4;
                        }
                    }
                    pos += 1;
                } else {
                    pos += 1;
                }

                let il_cnt = data[pos + 4] as usize;
                pos += 5 + il_cnt * 4;
                let c8c = data[pos] as usize;
                pos += 1 + c8c;
                let shader_hash = read_u64_le(data, pos);
                pos += 12;

                programs.push(RfxShaderProgram {
                    stage_index: s,
                    stage_name,
                    entry_point,
                    bytecode_size: bcs,
                    shader_model,
                    shader_hash,
                    dxbc_bytes,
                });
            }
        }

        let mut techniques = Vec::new();
        if pos + 4 <= data.len() {
            let cnt_a8 = read_u32_le(data, pos) as usize;
            pos += 4;
            for _ in 0..cnt_a8 {
                let sn = read_u32_le(data, pos) as usize;
                pos += 4;
                let tn = String::from_utf8_lossy(&data[pos..pos + sn]).into_owned();
                pos += sn;
                let s1 = read_u32_le(data, pos) as usize;
                pos += 4 + s1 * 52;
                let s2 = read_u32_le(data, pos) as usize;
                pos += 4 + s2 * 20 + 16;
                techniques.push(tn);
            }
        }

        if pos + 8 <= data.len() {
            let s80 = read_u64_le(data, pos) as usize;
            pos += 8;
            if s80 != 0 {
                pos += s80;
                let sn = read_u32_le(data, pos) as usize;
                pos += 4 + sn + 8;
            }
        }

        let mut permutations = Vec::new();
        if pos + 4 <= data.len() {
            let cnt_b8 = read_u32_le(data, pos) as usize;
            pos += 4;
            for _ in 0..cnt_b8 {
                let sn = read_u32_le(data, pos) as usize;
                pos += 4;
                let bn = String::from_utf8_lossy(&data[pos..pos + sn]).into_owned();
                pos += sn + 16;
                permutations.push(bn);
            }
        }

        Ok(Self {
            file_name,
            version,
            shader_name,
            programs,
            techniques,
            permutations,
        })
    }
}

#[inline]
fn range_check(n: usize) -> std::ops::Range<usize> {
    0..n
}
