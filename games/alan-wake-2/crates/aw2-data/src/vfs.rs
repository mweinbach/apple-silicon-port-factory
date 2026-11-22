use crate::formats::{BinRfx, RmdToc};
use std::fs;
use std::path::{Path, PathBuf};

/// Parsed DDS DX10 texture from a Northlight `.tex` asset for direct upload to `MTLTexture`
#[derive(Debug, Clone)]
pub struct DdsTexture {
    pub width: u32,
    pub height: u32,
    pub mip_count: u32,
    pub dxgi_format: u32,
    pub metal_pixel_format: u64,
    pub bytes_per_row_top_mip: u32,
    pub top_mip_bytes: Vec<u8>,
}

impl DdsTexture {
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        if data.len() < 148 || &data[0..4] != b"DDS " {
            return Err("Not a valid DDS DX10 .tex buffer".to_string());
        }
        let height = u32::from_le_bytes(data[12..16].try_into().unwrap());
        let width = u32::from_le_bytes(data[16..20].try_into().unwrap());
        let mip_count = u32::from_le_bytes(data[28..32].try_into().unwrap()).max(1);
        let dxgi_format = u32::from_le_bytes(data[128..132].try_into().unwrap());

        // Map DXGI_FORMAT to Apple Silicon MTLPixelFormat and block size
        let (metal_pixel_format, block_bytes, is_bc): (u64, u32, bool) = match dxgi_format {
            28 => (70, 4, false),  // DXGI_FORMAT_R8G8B8A8_UNORM -> MTLPixelFormatRGBA8Unorm
            29 => (71, 4, false),  // DXGI_FORMAT_R8G8B8A8_UNORM_SRGB -> MTLPixelFormatRGBA8Unorm_sRGB
            71 => (130, 8, true),  // DXGI_FORMAT_BC1_UNORM -> MTLPixelFormatBC1_RGBA
            72 => (131, 8, true),  // DXGI_FORMAT_BC1_UNORM_SRGB -> MTLPixelFormatBC1_RGBA_sRGB
            74 => (132, 16, true), // DXGI_FORMAT_BC2_UNORM -> MTLPixelFormatBC2_RGBA
            75 => (133, 16, true), // DXGI_FORMAT_BC2_UNORM_SRGB -> MTLPixelFormatBC2_RGBA_sRGB
            77 => (134, 16, true), // DXGI_FORMAT_BC3_UNORM -> MTLPixelFormatBC3_RGBA
            78 => (135, 16, true), // DXGI_FORMAT_BC3_UNORM_SRGB -> MTLPixelFormatBC3_RGBA_sRGB
            80 => (140, 8, true),  // DXGI_FORMAT_BC4_UNORM -> MTLPixelFormatBC4_RUnorm
            83 => (142, 16, true), // DXGI_FORMAT_BC5_UNORM -> MTLPixelFormatBC5_RGUnorm
            87 => (80, 4, false),  // DXGI_FORMAT_B8G8R8A8_UNORM -> MTLPixelFormatBGRA8Unorm
            91 => (81, 4, false),  // DXGI_FORMAT_B8G8R8A8_UNORM_SRGB -> MTLPixelFormatBGRA8Unorm_sRGB
            95 => (150, 16, true), // DXGI_FORMAT_BC6H_UF16 -> MTLPixelFormatBC6H_RGBUfloat
            96 => (151, 16, true), // DXGI_FORMAT_BC6H_SF16 -> MTLPixelFormatBC6H_RGBFloat
            98 => (152, 16, true), // DXGI_FORMAT_BC7_UNORM -> MTLPixelFormatBC7_RGBAUnorm
            99 => (153, 16, true), // DXGI_FORMAT_BC7_UNORM_SRGB -> MTLPixelFormatBC7_RGBAUnorm_sRGB
            other => return Err(format!("Unsupported DXGI_FORMAT {}", other)),
        };

        let (bytes_per_row, top_mip_size) = if is_bc {
            let bw = ((width + 3) / 4).max(1);
            let bh = ((height + 3) / 4).max(1);
            (bw * block_bytes, (bw * bh * block_bytes) as usize)
        } else {
            (width * block_bytes, (width * height * block_bytes) as usize)
        };

        let payload = &data[148..];
        if payload.len() < top_mip_size {
            return Err(format!(
                "DDS payload ({}) smaller than top mip ({})",
                payload.len(),
                top_mip_size
            ));
        }

        // Northlight stores mips either top-down or bottom-up; take the last `top_mip_size` bytes if bottom-up,
        // or first `top_mip_size` bytes. In DDS standard, mip 0 (largest) is at offset 148!
        let top_mip_bytes = payload[0..top_mip_size].to_vec();

        Ok(Self {
            width,
            height,
            mip_count,
            dxgi_format,
            metal_pixel_format,
            bytes_per_row_top_mip: bytes_per_row,
            top_mip_bytes,
        })
    }
}

/// Parses Northlight's `string_table.bin` localization table (`uint32 count`, then `(key, utf16_val)` pairs)
pub fn parse_string_table(data: &[u8], max_entries: usize) -> Vec<(String, String)> {
    if data.len() < 4 {
        return Vec::new();
    }
    let count = u32::from_le_bytes(data[0..4].try_into().unwrap()) as usize;
    let mut pos = 4usize;
    let mut out = Vec::with_capacity(count.min(max_entries));

    for _ in 0..count.min(max_entries) {
        if pos + 4 > data.len() {
            break;
        }
        let klen = u32::from_le_bytes(data[pos..pos + 4].try_into().unwrap()) as usize;
        pos += 4;
        if pos + klen + 4 > data.len() {
            break;
        }
        let key = String::from_utf8_lossy(&data[pos..pos + klen])
            .trim()
            .to_string();
        pos += klen;

        let vlen = u32::from_le_bytes(data[pos..pos + 4].try_into().unwrap()) as usize;
        pos += 4;
        let byte_len = vlen * 2;
        if pos + byte_len > data.len() {
            break;
        }
        let u16s: Vec<u16> = data[pos..pos + byte_len]
            .chunks_exact(2)
            .map(|c| u16::from_le_bytes([c[0], c[1]]))
            .collect();
        pos += byte_len;
        let val = String::from_utf16_lossy(&u16s);
        if !key.is_empty() && !val.is_empty() {
            out.push((key, val));
        }
    }
    out
}

/// Dynamically discovers the game root directory without hardcoding paths or filenames.
pub fn discover_game_dir() -> Option<PathBuf> {
    if let Ok(env_dir) = std::env::var("NORTHLIGHT_GAME_DIR") {
        let p = PathBuf::from(env_dir);
        if is_northlight_game_dir(&p) {
            return Some(p);
        }
    }

    // Check inside the .app bundle: Contents/Resources/Game
    if let Ok(exe) = std::env::current_exe() {
        if let Some(macos_dir) = exe.parent() {
            let bundled = macos_dir.join("../Resources/Game");
            if is_northlight_game_dir(&bundled) {
                return fs::canonicalize(&bundled).ok().or(Some(bundled));
            }
        }
    }

    // Scan standard user game folders dynamically
    if let Ok(home) = std::env::var("HOME") {
        let heroic_dir = PathBuf::from(&home).join("Games/Heroic");
        if let Ok(entries) = fs::read_dir(&heroic_dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if is_northlight_game_dir(&p) {
                    return Some(p);
                }
            }
        }
    }

    None
}

pub fn is_northlight_game_dir(dir: &Path) -> bool {
    if !dir.is_dir() {
        return false;
    }
    let has_shaders = dir.join("data/shaders").is_dir();
    let has_packs = dir.join("data_pack2").is_dir() || dir.join("data_pack").is_dir();
    has_shaders || has_packs
}

/// Recursive directory scanner by extension (zero hardcoded filenames).
/// Does not recurse into symlinked directories so self-referential symlinks in a game
/// folder cannot trigger an infinite traversal loop.
pub fn find_files_with_extension(root: &Path, ext: &str) -> Vec<PathBuf> {
    let mut results = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let Ok(ft) = entry.file_type() else {
                    continue;
                };
                let p = entry.path();
                if ft.is_dir() {
                    stack.push(p);
                } else if (ft.is_file() || (ft.is_symlink() && p.is_file()))
                    && p.extension()
                        .and_then(|e| e.to_str())
                        .map(|e| e.eq_ignore_ascii_case(ext))
                        .unwrap_or(false)
                {
                    results.push(p);
                }
            }
        }
    }
    results.sort();
    results
}

/// Northlight Engine Virtual File System mounting all discovered `.rmdtoc` archives and `.binrfx` shader bundles
pub struct NorthlightVfs {
    pub root_dir: PathBuf,
    pub archives: Vec<RmdToc>,
    pub resource_map: std::collections::HashMap<u64, (usize, usize)>,
    pub path_map: std::collections::HashMap<String, (usize, usize)>,
    pub shader_paths: Vec<PathBuf>,
    pub lua_paths: Vec<PathBuf>,
    pub executable_paths: Vec<PathBuf>,
}

impl NorthlightVfs {
    pub fn mount<P: AsRef<Path>>(root: P) -> Result<Self, String> {
        let root_dir = root.as_ref().to_path_buf();
        if !root_dir.exists() {
            return Err(format!("Game directory {:?} does not exist", root_dir));
        }

        let toc_paths = find_files_with_extension(&root_dir, "rmdtoc");
        let mut archives = Vec::with_capacity(toc_paths.len());
        let mut resource_map = std::collections::HashMap::new();
        let mut path_map = std::collections::HashMap::new();

        for tp in toc_paths {
            if let Ok(toc) = RmdToc::open(&tp) {
                let a_idx = archives.len();
                for (f_idx, f) in toc.files.iter().enumerate() {
                    if f.resource_id != 0 {
                        resource_map.insert(f.resource_id, (a_idx, f_idx));
                    }
                    path_map.insert(f.path.clone(), (a_idx, f_idx));
                }
                archives.push(toc);
            }
        }

        let shader_paths = find_files_with_extension(&root_dir, "binrfx");
        let lua_paths = find_files_with_extension(&root_dir, "lua");
        let executable_paths = find_files_with_extension(&root_dir, "exe");

        Ok(Self {
            root_dir,
            archives,
            resource_map,
            path_map,
            shader_paths,
            lua_paths,
            executable_paths,
        })
    }

    pub fn total_virtual_files(&self) -> usize {
        self.archives.iter().map(|a| a.files.len()).sum()
    }

    pub fn total_blobs(&self) -> usize {
        self.archives.iter().map(|a| a.blobs.len()).sum()
    }

    pub fn get_entry_by_path(&self, virtual_path: &str) -> Option<(&RmdToc, &crate::formats::TocFileEntry)> {
        if let Some(&(a_idx, f_idx)) = self.path_map.get(virtual_path) {
            let arch = &self.archives[a_idx];
            return Some((arch, &arch.files[f_idx]));
        }
        None
    }

    pub fn get_entry_by_rid(&self, rid: u64) -> Option<(&RmdToc, &crate::formats::TocFileEntry)> {
        if let Some(&(a_idx, f_idx)) = self.resource_map.get(&rid) {
            let arch = &self.archives[a_idx];
            return Some((arch, &arch.files[f_idx]));
        }
        None
    }

    pub fn read_virtual_file(&self, virtual_path: &str) -> Result<Vec<u8>, String> {
        if let Some((arch, f)) = self.get_entry_by_path(virtual_path) {
            return arch.extract_file(f);
        }
        for arch in &self.archives {
            for f in &arch.files {
                if f.path == virtual_path {
                    return arch.extract_file(f);
                }
            }
        }
        Err(format!("Virtual file '{}' not found in mounted archives", virtual_path))
    }

    pub fn load_shader_bundle(&self, index: usize) -> Result<BinRfx, String> {
        let p = self
            .shader_paths
            .get(index)
            .ok_or_else(|| format!("Shader index {} out of range", index))?;
        BinRfx::open(p)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_files_with_extension_skips_symlink_directory_cycles() {
        let tmp = std::env::temp_dir().join(format!(
            "aw2_data_symlink_loop_test_{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(tmp.join("sub")).unwrap();
        let target_file = tmp.join("sub/test.rmdtoc");
        fs::write(&target_file, b"stub").unwrap();
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(&tmp, tmp.join("self_loop")).unwrap();
            std::os::unix::fs::symlink(tmp.join("sub"), tmp.join("sub/sub_loop")).unwrap();
        }

        let found = find_files_with_extension(&tmp, "rmdtoc");
        let _ = fs::remove_dir_all(&tmp);

        assert_eq!(found, vec![target_file]);
    }
}
