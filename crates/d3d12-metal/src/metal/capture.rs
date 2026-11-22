//! Debug captures of presented frames (`AW2_PRESENT_CAPTURE`).
//!
//! `AW2_PRESENT_CAPTURE=out/frame` with `AW2_PRESENT_CAPTURE_AT=1,30,120`
//! (default `1`) writes `out/frame_<n>.png` with the back buffer contents
//! of those presents, read back on the GPU timeline (independent of window
//! visibility and Screen Recording permission).

use objc2_metal::MTLPixelFormat;
use std::io::Write;
use std::sync::Mutex;

static PENDING_CAPTURE: Mutex<Option<String>> = Mutex::new(None);

/// Request an on-demand PNG capture of the next presented back buffer to `path`.
pub fn request_next(path: impl Into<String>) {
    *PENDING_CAPTURE.lock().unwrap() = Some(path.into());
}

/// True if an on-demand frame capture is waiting for the next present.
pub fn has_pending() -> bool {
    PENDING_CAPTURE.lock().unwrap().is_some()
}

/// Consume the pending on-demand capture destination path, if any.
pub fn take_pending() -> Option<String> {
    PENDING_CAPTURE.lock().unwrap().take()
}

/// Which presents to capture: `(base path, present numbers)`.
pub fn config() -> Option<&'static (String, Vec<u64>)> {
    static C: std::sync::OnceLock<Option<(String, Vec<u64>)>> = std::sync::OnceLock::new();
    C.get_or_init(|| {
        let base = std::env::var("AW2_PRESENT_CAPTURE").ok()?;
        let at = std::env::var("AW2_PRESENT_CAPTURE_AT").unwrap_or_else(|_| "1".into());
        Some((base, at.split(',').filter_map(|s| s.trim().parse().ok()).collect()))
    })
    .as_ref()
}

fn half(h: u16) -> f32 {
    let s = if h & 0x8000 != 0 { -1.0 } else { 1.0 };
    let e = ((h >> 10) & 0x1f) as i32;
    let m = (h & 0x3ff) as f32;
    s * match e {
        0 => m * 2f32.powi(-24),
        31 => f32::INFINITY,
        _ => (1.0 + m / 1024.0) * 2f32.powi(e - 15),
    }
}

/// Tightly packed texels (`bytes_per_row` stride) → 8-bit RGB.
pub fn to_rgb8(fmt: MTLPixelFormat, data: &[u8], w: usize, h: usize, bytes_per_row: usize) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(w * h * 3);
    let q = |f: f32| (f.clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
    for y in 0..h {
        let row = &data[y * bytes_per_row..];
        for x in 0..w {
            match fmt {
                MTLPixelFormat::RGB10A2Unorm => {
                    let v = u32::from_le_bytes(row[x * 4..x * 4 + 4].try_into().unwrap());
                    let r10 = v & 0x3ff;
                    let g10 = (v >> 10) & 0x3ff;
                    let b10 = (v >> 20) & 0x3ff;
                    out.extend_from_slice(&[
                        ((r10 * 255 + 511) / 1023) as u8,
                        ((g10 * 255 + 511) / 1023) as u8,
                        ((b10 * 255 + 511) / 1023) as u8,
                    ]);
                }
                MTLPixelFormat::BGRA8Unorm | MTLPixelFormat::BGRA8Unorm_sRGB => out.extend_from_slice(&[row[x * 4 + 2], row[x * 4 + 1], row[x * 4]]),
                MTLPixelFormat::RGBA8Unorm | MTLPixelFormat::RGBA8Unorm_sRGB => out.extend_from_slice(&row[x * 4..x * 4 + 3]),
                MTLPixelFormat::RGBA16Float => {
                    let c = |i: usize| half(u16::from_le_bytes([row[x * 8 + 2 * i], row[x * 8 + 2 * i + 1]]));
                    out.extend_from_slice(&[q(c(0)), q(c(1)), q(c(2))]);
                }
                _ => return None,
            }
        }
    }
    Some(out)
}

fn crc32(data: &[u8]) -> u32 {
    let mut c = 0xFFFF_FFFFu32;
    for &b in data {
        c ^= b as u32;
        for _ in 0..8 {
            c = if c & 1 != 0 { 0xEDB8_8320 ^ (c >> 1) } else { c >> 1 };
        }
    }
    !c
}

fn chunk(out: &mut Vec<u8>, ty: &[u8; 4], data: &[u8]) {
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    let mut c = ty.to_vec();
    c.extend_from_slice(data);
    out.extend_from_slice(&c);
    out.extend_from_slice(&crc32(&c).to_be_bytes());
}

/// LSB-first bit writer for deflate.
#[allow(dead_code)]
struct Bits {
    out: Vec<u8>,
    acc: u64,
    n: u32,
}

#[allow(dead_code)]
impl Bits {
    fn put(&mut self, v: u32, len: u32) {
        self.acc |= (v as u64) << self.n;
        self.n += len;
        while self.n >= 8 {
            self.out.push(self.acc as u8);
            self.acc >>= 8;
            self.n -= 8;
        }
    }
    /// Huffman codes are sent MSB first.
    fn code(&mut self, c: u32, len: u32) {
        self.put(c.reverse_bits() >> (32 - len), len);
    }
    fn lit(&mut self, s: u32) {
        match s {
            0..=143 => self.code(0x30 + s, 8),
            144..=255 => self.code(0x190 + s - 144, 9),
            256..=279 => self.code(s - 256, 7),
            _ => self.code(0xC0 + s - 280, 8),
        }
    }
    /// Match of `len` (3..=258) at distance 1.
    fn run(&mut self, len: u32) {
        const BASE: [u32; 29] = [3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227, 258];
        const EXTRA: [u32; 29] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0];
        let i = BASE.iter().rposition(|&b| b <= len).unwrap();
        self.lit(257 + i as u32);
        self.put(len - BASE[i], EXTRA[i]);
        self.code(0, 5); // distance code 0 = distance 1
    }
}

/// Single fixed-Huffman block; byte runs become distance-1 matches.
#[allow(dead_code)]
fn deflate_rle(raw: &[u8]) -> Vec<u8> {
    let mut b = Bits { out: Vec::new(), acc: 0, n: 0 };
    b.put(1, 1); // BFINAL
    b.put(1, 2); // fixed Huffman
    let mut i = 0;
    while i < raw.len() {
        let v = raw[i];
        b.lit(v as u32);
        let mut j = i + 1;
        while j < raw.len() && raw[j] == v {
            j += 1;
        }
        let mut rem = (j - i - 1) as u32;
        while rem >= 3 {
            let l = rem.min(258);
            b.run(l);
            rem -= l;
        }
        for _ in 0..rem {
            b.lit(v as u32);
        }
        i = j;
    }
    b.lit(256);
    b.put(0, 7); // flush
    b.out
}

#[link(name = "z")]
unsafe extern "C" {
    fn compress2(
        dest: *mut u8,
        dest_len: *mut std::ffi::c_ulong,
        source: *const u8,
        source_len: std::ffi::c_ulong,
        level: std::ffi::c_int,
    ) -> std::ffi::c_int;
}

/// Minimal PNG encoder (8-bit RGB, Up filter, zlib deflate).
pub fn png_rgb8(w: u32, h: u32, rgb: &[u8]) -> Vec<u8> {
    let stride = w as usize * 3;
    let mut raw = Vec::with_capacity((stride + 1) * h as usize);
    for y in 0..h as usize {
        let row = &rgb[y * stride..(y + 1) * stride];
        if y == 0 {
            raw.push(0);
            raw.extend_from_slice(row);
        } else {
            let prev = &rgb[(y - 1) * stride..y * stride];
            raw.push(2);
            raw.extend(row.iter().zip(prev).map(|(a, b)| a.wrapping_sub(*b)));
        }
    }
    let bound = raw.len() + (raw.len() >> 12) + (raw.len() >> 14) + (raw.len() >> 25) + 13;
    let mut z = vec![0u8; bound];
    let mut z_len = bound as std::ffi::c_ulong;
    let rc = unsafe {
        compress2(
            z.as_mut_ptr(),
            &mut z_len,
            raw.as_ptr(),
            raw.len() as std::ffi::c_ulong,
            6,
        )
    };
    if rc == 0 {
        z.truncate(z_len as usize);
    } else {
        z.clear();
        z.extend_from_slice(&[0x78, 0x01]);
        z.extend_from_slice(&deflate_rle(&raw));
        let (mut a, mut b) = (1u32, 0u32);
        for &x in &raw {
            a = (a + x as u32) % 65521;
            b = (b + a) % 65521;
        }
        z.extend_from_slice(&((b << 16) | a).to_be_bytes());
    }
    let mut out = b"\x89PNG\r\n\x1a\n".to_vec();
    let mut ihdr = Vec::new();
    ihdr.extend_from_slice(&w.to_be_bytes());
    ihdr.extend_from_slice(&h.to_be_bytes());
    ihdr.extend_from_slice(&[8, 2, 0, 0, 0]);
    chunk(&mut out, b"IHDR", &ihdr);
    chunk(&mut out, b"IDAT", &z);
    chunk(&mut out, b"IEND", &[]);
    out
}

pub fn write_png(path: &str, w: u32, h: u32, rgb: &[u8]) -> std::io::Result<()> {
    if let Some(dir) = std::path::Path::new(path).parent() {
        if !dir.as_os_str().is_empty() {
            std::fs::create_dir_all(dir)?;
        }
    }
    let tmp = format!("{path}.tmp");
    std::fs::File::create(&tmp)?.write_all(&png_rgb8(w, h, rgb))?;
    std::fs::rename(&tmp, path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc_and_png_header() {
        assert_eq!(crc32(b"IEND"), 0xAE42_6082);
        let p = png_rgb8(2, 1, &[255, 0, 0, 0, 255, 0]);
        assert_eq!(&p[..8], b"\x89PNG\r\n\x1a\n");
    }

    /// `AW2_PNG_SELFTEST=<path>` writes a test image for checking with an
    /// external decoder.
    #[test]
    fn png_selftest_file() {
        let Ok(path) = std::env::var("AW2_PNG_SELFTEST") else { return };
        let (w, h) = (300usize, 200usize);
        let mut rgb = vec![0u8; w * h * 3];
        for y in 0..h {
            for x in 0..w {
                let p = &mut rgb[(y * w + x) * 3..][..3];
                p.copy_from_slice(&if y < 100 { [10, 20, 30] } else { [x as u8, y as u8, (x ^ y) as u8] });
            }
        }
        write_png(&path, w as u32, h as u32, &rgb).unwrap();
    }

    #[test]
    fn rgb10a2_conversion() {
        let v: u32 = 0x3ff | (0x200 << 10) | (0 << 20) | (3 << 30);
        let rgb = to_rgb8(MTLPixelFormat::RGB10A2Unorm, &v.to_le_bytes(), 1, 1, 4).unwrap();
        assert_eq!(rgb, vec![255, 128, 0]);
    }
}
