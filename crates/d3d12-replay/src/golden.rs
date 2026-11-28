//! Frame readback, PNG encode/decode, and deterministic pixel-hash golden comparison.

use std::path::Path;

use d3d12_metal::layout::PlacedFootprint;
use d3d12_metal::metal::capture;
use d3d12_metal::metal::cmdlist::{CommandAllocator, CommandList, CopyLoc};
use d3d12_metal::metal::queue::{ListType, Queue};
use d3d12_metal::metal::resource::Resource;
use d3d12_metal::types::{dim, heap_type, HeapProperties, ResourceDesc};
use objc2_metal::{MTLBuffer, MTLTextureUsage};

/// Captured 8-bit RGB frame image.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameImage {
    pub width: u32,
    pub height: u32,
    /// Tightly packed RGB8 (`width * height * 3` bytes).
    pub rgb: Vec<u8>,
}

impl FrameImage {
    /// Deterministic 128-bit XXH128 pixel hash covering dimensions and RGB8 bytes.
    pub fn pixel_hash(&self) -> u128 {
        let mut h = dxil_metal::hash::KeyHasher::new("d3d12-replay/golden/rgb8/v1");
        h.u32(self.width);
        h.u32(self.height);
        h.bytes(&self.rgb);
        h.finish()
    }

    pub fn hash_hex(&self) -> String {
        format!("{:032x}", self.pixel_hash())
    }

    pub fn to_png(&self) -> Vec<u8> {
        capture::png_rgb8(self.width, self.height, &self.rgb)
    }

    /// Compare `self` against `expected`, returning mismatch statistics.
    pub fn diff(&self, expected: &FrameImage) -> Result<ImageDiff, String> {
        if (self.width, self.height) != (expected.width, expected.height) {
            return Err(format!(
                "image dimensions differ: got {}x{}, expected {}x{}",
                self.width, self.height, expected.width, expected.height
            ));
        }
        let mut differing_pixels = 0u64;
        let mut max_channel_delta = 0u8;
        for (a, b) in self.rgb.chunks_exact(3).zip(expected.rgb.chunks_exact(3)) {
            let dr = a[0].abs_diff(b[0]);
            let dg = a[1].abs_diff(b[1]);
            let db = a[2].abs_diff(b[2]);
            let m = dr.max(dg).max(db);
            if m > 0 {
                differing_pixels += 1;
                max_channel_delta = max_channel_delta.max(m);
            }
        }
        Ok(ImageDiff {
            total_pixels: self.width as u64 * self.height as u64,
            differing_pixels,
            max_channel_delta,
            got_hash: self.hash_hex(),
            expected_hash: expected.hash_hex(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageDiff {
    pub total_pixels: u64,
    pub differing_pixels: u64,
    pub max_channel_delta: u8,
    pub got_hash: String,
    pub expected_hash: String,
}

impl ImageDiff {
    pub fn matches(&self, max_delta: u8) -> bool {
        self.max_channel_delta <= max_delta
    }
}

/// Outcome of comparing a captured frame against a golden PNG or pixel-hash file.
#[derive(Debug, Clone)]
pub struct GoldenCheck {
    pub path: String,
    pub width: u32,
    pub height: u32,
    pub got_hash: String,
    pub expected_hash: String,
    pub passed: bool,
    pub detail: String,
}

impl std::fmt::Display for GoldenCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "golden {}: {} ({}x{}, hash={}, {})",
            self.path,
            if self.passed { "PASS" } else { "FAIL" },
            self.width,
            self.height,
            self.got_hash,
            self.detail
        )
    }
}

/// Read back a 2D GPU texture (`subresource 0`) into an 8-bit RGB [`FrameImage`].
pub fn readback_texture(queue: &Queue, res: &Resource) -> Result<FrameImage, String> {
    if res.texture().is_none() {
        return Err("golden target resource is not a texture".into());
    }
    let w = res.desc.Width as u32;
    let h = res.desc.Height;
    if w == 0 || h == 0 {
        return Err(format!("golden target has empty dimensions {w}x{h}"));
    }
    let fmt_info = d3d12_metal::format::info(res.desc.Format);
    let bpp = (fmt_info.bytes as u32).max(4);
    let row_pitch = (w.saturating_mul(bpp)).next_multiple_of(256);
    let total_bytes = row_pitch as u64 * h as u64;

    let rb_desc = ResourceDesc {
        Dimension: dim::BUFFER,
        Width: total_bytes,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        SampleCount: 1,
        Layout: 1,
        ..Default::default()
    };
    let hp = HeapProperties { Type: heap_type::READBACK, ..Default::default() };
    let rb = Resource::new_committed(&rb_desc, &hp, MTLTextureUsage::empty(), "replay golden readback")
        .map_err(|e| format!("golden readback buffer: {e:?}"))?;

    let alloc = CommandAllocator::new(ListType::Direct).ok_or("golden readback allocator")?;
    let mut cl = CommandList::new(ListType::Direct).ok_or("golden readback command list")?;
    cl.begin(&alloc)?;
    let fp = PlacedFootprint {
        offset: 0,
        format: res.desc.Format,
        width: w,
        height: h,
        depth: 1,
        row_pitch,
        _pad: 0,
    };
    cl.copy_texture_region(CopyLoc::Footprint(&rb, fp), [0, 0, 0], CopyLoc::Subresource(res, 0), None)?;
    cl.close()?;
    queue.execute_command_lists(&mut [&mut cl]);
    queue.wait_idle();

    let buf = rb.buffer().ok_or("golden readback missing buffer")?;
    let raw = unsafe { std::slice::from_raw_parts(buf.contents().as_ptr() as *const u8, total_bytes as usize) };
    let rgb = capture::to_rgb8(res.mtl_format, raw, w as usize, h as usize, row_pitch as usize)
        .ok_or_else(|| format!("golden readback unsupported pixel format {:?}", res.mtl_format))?;
    Ok(FrameImage { width: w, height: h, rgb })
}

/// Write a golden `.png` (when extension is `.png`) or a pixel-hash text file (`<w>x<h> <hash>\n`).
pub fn write_golden(path: &Path, img: &FrameImage) -> Result<(), String> {
    if let Some(dir) = path.parent() {
        if !dir.as_os_str().is_empty() {
            std::fs::create_dir_all(dir).map_err(|e| format!("create {}: {e}", dir.display()))?;
        }
    }
    if is_png_path(path) {
        let p = path.to_str().ok_or("non-utf8 golden path")?;
        capture::write_png(p, img.width, img.height, &img.rgb)
            .map_err(|e| format!("writing {}: {e}", path.display()))
    } else {
        let text = format!("{}x{} {}\n", img.width, img.height, img.hash_hex());
        std::fs::write(path, text).map_err(|e| format!("writing {}: {e}", path.display()))
    }
}

/// Verify `img` against an existing golden `.png` or pixel-hash file.
pub fn verify_golden(path: &Path, img: &FrameImage, max_delta: u8) -> Result<GoldenCheck, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("reading golden {}: {e}", path.display()))?;
    if bytes.starts_with(b"\x89PNG\r\n\x1a\n") || is_png_path(path) {
        let expected = decode_png_rgb8(&bytes).map_err(|e| format!("decoding {}: {e}", path.display()))?;
        let diff = img.diff(&expected)?;
        let passed = diff.matches(max_delta);
        let detail = if passed {
            format!("0/{} pixels above tol={max_delta} (max_delta={})", diff.total_pixels, diff.max_channel_delta)
        } else {
            format!(
                "{}/{} differing pixels (max_delta={}, tol={max_delta}, expected_hash={})",
                diff.differing_pixels, diff.total_pixels, diff.max_channel_delta, diff.expected_hash
            )
        };
        Ok(GoldenCheck {
            path: path.display().to_string(),
            width: img.width,
            height: img.height,
            got_hash: diff.got_hash,
            expected_hash: diff.expected_hash,
            passed,
            detail,
        })
    } else {
        let text = std::str::from_utf8(&bytes).map_err(|_| format!("golden {} is not PNG or UTF-8 hash", path.display()))?;
        let line = text.lines().map(str::trim).find(|l| !l.is_empty() && !l.starts_with('#')).ok_or("empty golden hash file")?;
        let mut parts = line.split_whitespace();
        let first = parts.next().unwrap();
        let (exp_dims, exp_hash) = match parts.next() {
            Some(h) => (Some(first), h.to_ascii_lowercase()),
            None => (None, first.to_ascii_lowercase()),
        };
        if let Some(dims) = exp_dims {
            let want = format!("{}x{}", img.width, img.height);
            if dims != want {
                return Ok(GoldenCheck {
                    path: path.display().to_string(),
                    width: img.width,
                    height: img.height,
                    got_hash: img.hash_hex(),
                    expected_hash: exp_hash,
                    passed: false,
                    detail: format!("dimensions mismatch: got {want}, expected {dims}"),
                });
            }
        }
        let got_hash = img.hash_hex();
        let passed = got_hash == exp_hash;
        let detail = if passed {
            "exact pixel hash match".into()
        } else {
            format!("hash mismatch: expected {exp_hash}")
        };
        Ok(GoldenCheck {
            path: path.display().to_string(),
            width: img.width,
            height: img.height,
            got_hash,
            expected_hash: exp_hash,
            passed,
            detail,
        })
    }
}

fn is_png_path(path: &Path) -> bool {
    path.extension().and_then(|e| e.to_str()).is_some_and(|e| e.eq_ignore_ascii_case("png"))
}

#[link(name = "z")]
unsafe extern "C" {
    fn uncompress(
        dest: *mut u8,
        dest_len: *mut std::ffi::c_ulong,
        source: *const u8,
        source_len: std::ffi::c_ulong,
    ) -> std::ffi::c_int;
}

/// Minimal 8-bit RGB PNG decoder (supports all 5 PNG scanline filters).
pub fn decode_png_rgb8(png: &[u8]) -> Result<FrameImage, String> {
    if png.len() < 8 || &png[..8] != b"\x89PNG\r\n\x1a\n" {
        return Err("not a PNG signature".into());
    }
    let mut pos = 8usize;
    let (mut width, mut height) = (0u32, 0u32);
    let mut saw_ihdr = false;
    let mut idat = Vec::new();
    while pos + 12 <= png.len() {
        let len = u32::from_be_bytes(png[pos..pos + 4].try_into().unwrap()) as usize;
        let ty = &png[pos + 4..pos + 8];
        let data_start = pos + 8;
        let data_end = data_start.checked_add(len).ok_or("PNG chunk length overflow")?;
        if data_end + 4 > png.len() {
            return Err("truncated PNG chunk".into());
        }
        let data = &png[data_start..data_end];
        match ty {
            b"IHDR" => {
                if data.len() != 13 {
                    return Err(format!("invalid IHDR length {}", data.len()));
                }
                width = u32::from_be_bytes(data[0..4].try_into().unwrap());
                height = u32::from_be_bytes(data[4..8].try_into().unwrap());
                let bit_depth = data[8];
                let color_type = data[9];
                if bit_depth != 8 || color_type != 2 {
                    return Err(format!("unsupported PNG format (bit_depth={bit_depth}, color_type={color_type}; expected 8-bit RGB)"));
                }
                saw_ihdr = true;
            }
            b"IDAT" => idat.extend_from_slice(data),
            b"IEND" => break,
            _ => {}
        }
        pos = data_end + 4;
    }
    if !saw_ihdr || width == 0 || height == 0 {
        return Err("PNG missing valid IHDR".into());
    }
    let stride = (width as usize).checked_mul(3).ok_or("PNG stride overflow")?;
    let expected_raw = (stride + 1).checked_mul(height as usize).ok_or("PNG size overflow")?;
    let mut raw = vec![0u8; expected_raw];
    let mut dest_len = expected_raw as std::ffi::c_ulong;
    let rc = unsafe {
        uncompress(
            raw.as_mut_ptr(),
            &mut dest_len,
            idat.as_ptr(),
            idat.len() as std::ffi::c_ulong,
        )
    };
    if rc != 0 || dest_len as usize != expected_raw {
        return Err(format!("zlib uncompress failed (rc={rc}, got={dest_len}, want={expected_raw})"));
    }

    let mut rgb = vec![0u8; stride * height as usize];
    for y in 0..height as usize {
        let filter = raw[y * (stride + 1)];
        let src = &raw[y * (stride + 1) + 1..(y + 1) * (stride + 1)];
        let row_start = y * stride;
        for x in 0..stride {
            let left = if x >= 3 { rgb[row_start + x - 3] } else { 0 };
            let up = if y > 0 { rgb[row_start - stride + x] } else { 0 };
            let up_left = if y > 0 && x >= 3 { rgb[row_start - stride + x - 3] } else { 0 };
            let recon = match filter {
                0 => src[x],
                1 => src[x].wrapping_add(left),
                2 => src[x].wrapping_add(up),
                3 => src[x].wrapping_add(((left as u16 + up as u16) / 2) as u8),
                4 => src[x].wrapping_add(paeth(left, up, up_left)),
                f => return Err(format!("unknown PNG filter {f}")),
            };
            rgb[row_start + x] = recon;
        }
    }
    Ok(FrameImage { width, height, rgb })
}

fn paeth(a: u8, b: u8, c: u8) -> u8 {
    let (a, b, c) = (a as i32, b as i32, c as i32);
    let p = a + b - c;
    let pa = (p - a).abs();
    let pb = (p - b).abs();
    let pc = (p - c).abs();
    if pa <= pb && pa <= pc {
        a as u8
    } else if pb <= pc {
        b as u8
    } else {
        c as u8
    }
}
