//! Clean-room native ARM64/Rust Bink 2 (`KB2f`..`KB2n`, integer path) video decoder.
//!
//! Decodes multi-slice macroblock frames (32x32 luma/alpha/HDR, 16x16 Cr/Cb) into
//! caller-registered planar frame buffers (`Y`, `cR`, `cB`, optional `A`, optional `H`).

use super::dsp::{
    add_clamp_16x8, avg_16x8_pavgb, avg_32x8_pavgb, avg_8x8_pavgb, clamp_store_16x8, copy_16x16,
    copy_16x8, deblock_horiz_16, deblock_vert_8, idct8x8, idct8x8_dc, mc_chroma_8x8, mc_luma_16x16,
    smooth_dc_pair, smooth_inter_horiz, smooth_inter_vert,
};
use super::tables::{
    AC_COUNT, AC_RUN, AC_VLC1, AC_VLC2, CBP4_DEFAULT, DC_QUANT, DEBLOCK_LUT_A, DEBLOCK_LUT_B,
    DEBLOCK_LUT_C, MV_CLAMP, MV_VLC, QDELTA_LUT, QMAT_INTER, QMAT_INTRA_CHROMA, QMAT_INTRA_LUMA,
    SLICE_COUNT, ZIGZAG_U8,
};

/// Planar destination/reference view for one Bink 2 frame decode.
pub struct FramePlanes<'a> {
    pub ya_width: usize,
    pub ya_height: usize,
    pub ya_pitch: usize,
    pub crcb_pitch: usize,
    pub dst_y: &'a mut [u8],
    pub dst_cr: &'a mut [u8],
    pub dst_cb: &'a mut [u8],
    pub dst_a: Option<&'a mut [u8]>,
    pub dst_h: Option<&'a mut [u8]>,
    pub ref_y: &'a [u8],
    pub ref_cr: &'a [u8],
    pub ref_cb: &'a [u8],
    pub ref_a: Option<&'a [u8]>,
    pub ref_h: Option<&'a [u8]>,
}

/// Slice partitioning for a Bink 2 video stream (`compute_slices`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SliceInfo {
    pub num_slices: u32,
    pub slice_ends: [u32; 8],
}

/// Compute slice count and per-slice ending Y rows (multiples of 32) from the
/// Bink fourCC, open flags, and display dimensions.
pub fn compute_slices(fourcc: u32, open_flags: u32, _width: u32, height: u32) -> SliceInfo {
    let mut info = SliceInfo {
        num_slices: 1,
        slice_ends: [0; 8],
    };
    let kb2f = u32::from_le_bytes(*b"KB2f");
    let kb2g = u32::from_le_bytes(*b"KB2g");
    if fourcc == kb2f {
        info.num_slices = 2;
        let mid = ((height.wrapping_add(32)) >> 1) & !31;
        info.slice_ends[0] = mid;
        if mid != height {
            info.slice_ends[1] = height;
        }
        return info;
    }
    let h_pad = (height.wrapping_add(31)) & !31;
    if fourcc == kb2g {
        if h_pad >= 128 {
            info.num_slices = 2;
            info.slice_ends[0] = (h_pad >> 1) & !31;
            info.slice_ends[1] = h_pad;
            return info;
        }
        info.num_slices = 1;
        info.slice_ends[0] = h_pad;
        return info;
    }
    if h_pad >= 128 {
        let n = SLICE_COUNT[(open_flags & 3) as usize];
        info.num_slices = n;
        if n == 1 {
            info.slice_ends[0] = h_pad;
            return info;
        }
        let mut cur = ((n * 32 - 1 + h_pad) / n) & !31;
        info.slice_ends[0] = cur;
        let mut rem = n - 1;
        if rem == 1 {
            info.slice_ends[1] = h_pad;
            return info;
        }
        let mut idx = 1usize;
        while rem > 1 {
            let step = ((rem * 32 - 1 + h_pad - cur) / rem) & !31;
            cur += step;
            info.slice_ends[idx] = cur;
            idx += 1;
            rem -= 1;
        }
        info.slice_ends[idx] = h_pad;
        return info;
    }
    info.num_slices = 1;
    info.slice_ends[0] = h_pad;
    info
}

/// LSB-first 64-bit buffered word-oriented bitstream reader matching Bink 2's reader layout.
pub struct BitReader<'a> {
    data: &'a [u8],
    base_offset: usize,
    next_word_idx: usize,
    bit_buf: u64,
    bits_left: u32,
}

impl<'a> BitReader<'a> {
    pub fn new(data: &'a [u8], byte_offset: usize) -> Self {
        Self {
            data,
            base_offset: byte_offset,
            next_word_idx: 0,
            bit_buf: 0,
            bits_left: 0,
        }
    }

    #[cold]
    #[inline(never)]
    fn word_at_tail(&self, off: usize) -> u32 {
        if off < self.data.len() {
            let mut b = [0u8; 4];
            let rem = self.data.len() - off;
            b[..rem].copy_from_slice(&self.data[off..]);
            u32::from_le_bytes(b)
        } else {
            0
        }
    }

    #[inline(always)]
    fn word_at(&self, widx: usize) -> u32 {
        let off = self.base_offset + (widx << 2);
        if off + 4 <= self.data.len() {
            // SAFETY: `off + 4 <= self.data.len()` guarantees 4 valid readable bytes.
            unsafe {
                let ptr = self.data.as_ptr().add(off) as *const [u8; 4];
                u32::from_le_bytes(std::ptr::read_unaligned(ptr))
            }
        } else {
            self.word_at_tail(off)
        }
    }

    /// Ensure at least 32 valid bits are buffered in `bit_buf`.
    #[inline(always)]
    fn refill(&mut self) {
        if self.bits_left < 32 {
            let w = self.word_at(self.next_word_idx) as u64;
            self.bit_buf |= w << self.bits_left;
            self.next_word_idx += 1;
            self.bits_left += 32;
        }
    }

    #[inline]
    pub fn byte_pos(&self) -> usize {
        let unread_words = (self.bits_left >> 5) as usize;
        self.base_offset + (self.next_word_idx - unread_words) * 4
    }

    #[inline]
    pub fn align_u32(&mut self) {
        let rem = self.bits_left & 31;
        if rem != 0 {
            self.bit_buf >>= rem;
            self.bits_left -= rem;
        }
    }

    #[inline(always)]
    pub fn read_bit(&mut self) -> u32 {
        if self.bits_left == 0 {
            let w = self.word_at(self.next_word_idx) as u64;
            self.next_word_idx += 1;
            self.bit_buf = w >> 1;
            self.bits_left = 31;
            (w as u32) & 1
        } else {
            let b = (self.bit_buf as u32) & 1;
            self.bit_buf >>= 1;
            self.bits_left -= 1;
            b
        }
    }

    #[inline(always)]
    pub fn read_bits(&mut self, n: u32) -> u32 {
        if n == 0 {
            return 0;
        }
        if self.bits_left < n {
            let w = self.word_at(self.next_word_idx) as u64;
            self.bit_buf |= w << self.bits_left;
            self.next_word_idx += 1;
            self.bits_left += 32;
        }
        let v = self.bit_buf as u32;
        self.bit_buf >>= n;
        self.bits_left -= n;
        let mask = if n >= 32 { u32::MAX } else { (1u32 << n) - 1 };
        v & mask
    }

    #[inline(always)]
    pub fn peek_bits(&self, n: u32) -> u32 {
        if n == 0 {
            return 0;
        }
        let mut v = self.bit_buf;
        if self.bits_left < n {
            let next = self.word_at(self.next_word_idx) as u64;
            v |= next << self.bits_left;
        }
        let mask = if n >= 32 { u32::MAX } else { (1u32 << n) - 1 };
        (v as u32) & mask
    }

    #[inline(always)]
    pub fn skip_bits(&mut self, n: u32) {
        if n == 0 {
            return;
        }
        if self.bits_left < n {
            let w = self.word_at(self.next_word_idx) as u64;
            self.bit_buf |= w << self.bits_left;
            self.next_word_idx += 1;
            self.bits_left += 32;
        }
        self.bit_buf >>= n;
        self.bits_left -= n;
    }

    /// Consume `n` bits (`n <= 32`) when caller already ensured `bits_left >= n` via `refill()`.
    #[inline(always)]
    fn consume_fast(&mut self, n: u32) {
        self.bit_buf >>= n;
        self.bits_left -= n;
    }
}

/// Decode a row-skip or column-skip deblock bitmap (`180022680`).
fn decode_rle_bitmap(out: &mut [u8], mut bit_pos: u32, bs: &mut BitReader, mut count: u32) {
    out.fill(0);
    if count == 0 {
        return;
    }
    let mut byte_idx = 0usize;
    let mut acc = 0u32;

    if bs.read_bit() == 0 {
        while count > 8 {
            let b = bs.read_bits(8);
            acc |= b << bit_pos;
            let next_pos = bit_pos + 8;
            if next_pos > 7 {
                if byte_idx < out.len() {
                    out[byte_idx] = acc as u8;
                }
                byte_idx += 1;
                acc >>= 8;
                bit_pos = next_pos - 8;
            } else {
                bit_pos = next_pos;
            }
            count -= 8;
        }
        if count != 0 {
            let b = bs.read_bits(count);
            acc |= b << bit_pos;
            if byte_idx < out.len() {
                out[byte_idx] = acc as u8;
            }
        }
        return;
    }

    let mut cur_bit = 0u32;
    let mut state = 0i32;

    while count != 0 {
        if bs.read_bit() == 0 {
            if state == 3 {
                cur_bit ^= 1;
            } else {
                cur_bit = bs.read_bit();
            }
            state = 2;
            let rem = count - 1;
            if rem < 4 {
                let extra = bs.read_bits(rem);
                acc |= (cur_bit << bit_pos) | (extra << (bit_pos + 1));
                bit_pos += 1 + rem;
                count = 0;
            } else {
                let extra = bs.read_bits(4);
                acc |= (cur_bit << bit_pos) | (extra << (bit_pos + 1));
                bit_pos += 5;
                count -= 5;
            }
            if bit_pos > 7 {
                if byte_idx < out.len() {
                    out[byte_idx] = acc as u8;
                }
                byte_idx += 1;
                acc >>= 8;
                bit_pos -= 8;
            }
        } else {
            let len_bits = if count < 4 {
                2u32
            } else if count >= 8 && count >= 16 {
                5u32
            } else if count >= 8 {
                4u32
            } else {
                4u32
            };
            let mut run = if state == 3 {
                cur_bit ^= 1;
                len_bits + 1
            } else {
                cur_bit = bs.read_bit();
                len_bits + 2
            };
            if count < run {
                run = count;
            }
            count -= run;
            if count != 0 {
                let extra = bs.read_bits(len_bits);
                state = if extra == (1u32 << len_bits) - 1 {
                    1
                } else {
                    3
                };
                run += extra;
                count = count.saturating_sub(extra);
            }
            let fill_byte = if cur_bit != 0 { 0xffu32 } else { 0u32 };
            while run > 8 {
                acc |= fill_byte << bit_pos;
                if byte_idx < out.len() {
                    out[byte_idx] = acc as u8;
                }
                byte_idx += 1;
                acc >>= 8;
                run -= 8;
            }
            if run != 0 {
                let mask = (1u32 << run) - 1;
                acc |= (fill_byte & mask) << bit_pos;
                bit_pos += run;
                if bit_pos > 7 {
                    if byte_idx < out.len() {
                        out[byte_idx] = acc as u8;
                    }
                    byte_idx += 1;
                    acc >>= 8;
                    bit_pos -= 8;
                }
            }
        }
    }
    if bit_pos != 0 && byte_idx < out.len() {
        out[byte_idx] = acc as u8;
    }
}

/// Move-to-front macroblock mode decoder (`180022ac0`).
#[inline]
fn decode_mode_mtf(bs: &mut BitReader, mtf: &mut [u8; 4]) -> u8 {
    bs.refill();
    let v = (bs.bit_buf as u32) & 7;
    if (v & 1) != 0 {
        bs.consume_fast(1);
        mtf[0]
    } else if (v & 2) != 0 {
        bs.consume_fast(2);
        mtf.swap(0, 1);
        mtf[0]
    } else if v == 0 {
        bs.consume_fast(3);
        mtf.swap(1, 2);
        mtf[1]
    } else {
        bs.consume_fast(3);
        mtf.swap(2, 3);
        mtf[2]
    }
}

/// Decode quantizer delta (`180014ae0`).
#[inline]
fn decode_qdelta(bs: &mut BitReader, prev_q: i32) -> i32 {
    bs.refill();
    let v = (bs.bit_buf as u32) & 0x1ff;
    let mut mag: i32 = if (v & 7) != 0 {
        let m = QDELTA_LUT[(v & 7) as usize] as i32;
        bs.consume_fast((m + 1) as u32);
        m
    } else if (v & 0xf) == 0 {
        bs.consume_fast(9);
        ((v >> 4) + 5) as i32
    } else {
        bs.consume_fast(5);
        (((v >> 4) & 1) + 3) as i32
    };
    if mag != 0 {
        let sign = (bs.bit_buf as u32) & 1;
        bs.consume_fast(1);
        if sign != 0 {
            mag = -mag;
        }
    }
    prev_q.wrapping_add(mag)
}

#[inline]
fn median3_i32(a: i32, b: i32, c: i32) -> i32 {
    let (lo, hi) = if a < b { (a, b) } else { (b, a) };
    c.clamp(lo, hi)
}

#[inline]
fn median3_u8(a: u8, b: u8, c: u8) -> u8 {
    let (lo, hi) = if a < b { (a, b) } else { (b, a) };
    c.clamp(lo, hi)
}

#[inline]
fn pred_grad(a: i16, b: i16, c: i16) -> i16 {
    let mn = a.min(b).min(c);
    let mx = a.max(b).max(c);
    a.wrapping_add(b).wrapping_sub(c).clamp(mn, mx)
}

/// Decode 16-block coded-block-pattern (`1800147b0`).
fn decode_cbp16(bs: &mut BitReader, prev: u32, flags: u32) -> u32 {
    let mut cnt = (prev & 0xffff).count_ones();
    let invert = if cnt > 7 {
        cnt = 16 - cnt;
        0xffffu32
    } else {
        0u32
    };
    let mut cbp = 0u32;
    if bs.read_bit() == 0 {
        if cnt < 4 {
            for _ in 0..4 {
                cbp >>= 4;
                if bs.read_bit() == 0 {
                    cbp |= bs.read_bits(4) << 12;
                }
            }
        } else {
            cbp = bs.read_bits(16);
        }
    }
    cbp ^= invert;
    if ((flags >> 18) & 1) == 0 || (cbp & 0xffff) != 0 {
        if bs.read_bit() != 0 {
            cbp |= cbp << 16;
        }
    }
    cbp
}

/// Decode 4-block coded-block-pattern (`1800149e0`).
fn decode_cbp4(bs: &mut BitReader, prev: u32) -> u32 {
    let def = CBP4_DEFAULT[(prev & 0xf) as usize];
    if bs.read_bit() != 0 {
        return def | (prev & 0xf_0000);
    }
    let mut cbp = bs.read_bits(4);
    if bs.read_bit() != 0 {
        cbp |= cbp << 16;
    }
    cbp
}

/// Decode `N` (4 or 16) dequantized DC deltas (`180012cd0` / `180013af0`).
fn decode_dc_deltas<const N: usize>(bs: &mut BitReader, q: u32) -> ([i16; N], bool) {
    let mut out = [0i16; N];
    if bs.read_bit() == 0 {
        return (out, false);
    }
    let q_idx = (q.max(8) as usize).min(DC_QUANT.len() - 1);
    let scale = DC_QUANT[q_idx] as i32;
    for slot in &mut out {
        bs.refill();
        let peek = (bs.bit_buf as u32) & 0x7ff;
        let k = (!peek).trailing_zeros();
        bs.consume_fast(k + 1);
        if k != 0 {
            let level = if k > 3 {
                let eb = k - 3;
                let extra = (bs.bit_buf as u32) & ((1u32 << eb) - 1);
                bs.consume_fast(eb);
                (extra + 2 + (1u32 << eb)) as i32
            } else {
                k as i32
            };
            let sign = (bs.bit_buf as u32) & 1;
            bs.consume_fast(1);
            let mag = (level.wrapping_mul(scale).wrapping_add(0x200)) >> 10;
            *slot = if sign != 0 { -mag } else { mag } as i16;
        }
    }
    (out, true)
}

/// Decode 16-block DC grid in 4x4 Morton quad order (`180012e70`).
///
/// `dc` holds the left MB's 16 DCs on entry (indices `5, 7, 13, 15` are left edge)
/// and receives the current MB's 16 DCs on exit.
/// `above` holds `[_, _, _, above_left, above0, above1, above2, above3]` if `Some`,
/// or `None` for inter residuals (initial DC prediction = 0).
fn decode_dc16(
    dc: &mut [i32; 16],
    above: Option<&[i32; 8]>,
    q: u32,
    bs: &mut BitReader,
    flags: u32,
    min_dc: i16,
    max_dc: i16,
) {
    let (deltas, nonzero) = decode_dc_deltas::<16>(bs, q);
    if !nonzero && above.is_none() && (flags & 0xa8) == 0xa8 {
        dc.fill(0);
        return;
    }

    let left_edge = (flags & 0x20) != 0;
    let top_edge = (flags & 0x88) != 0;

    let l0 = dc[5] as i16;
    let l1 = dc[7] as i16;
    let l2 = dc[13] as i16;
    let l3 = dc[15] as i16;

    let (al, a0, a1, a2, a3) = match above {
        Some(ab) => (
            ab[3] as i16,
            ab[4] as i16,
            ab[5] as i16,
            ab[6] as i16,
            ab[7] as i16,
        ),
        None => (0, 0, 0, 0, 0),
    };

    let mut v = [0i16; 16];

    // Quad 0 (top-left): blocks 0, 1, 2, 3
    let p0 = match (left_edge, top_edge) {
        (false, false) => pred_grad(l0, a0, al),
        (false, true) => l0,
        (true, false) => a0,
        (true, true) => {
            if above.is_some() {
                0x400
            } else {
                0
            }
        }
    };
    v[0] = p0.wrapping_add(deltas[0]).clamp(min_dc, max_dc);

    let p1 = if !top_edge {
        pred_grad(v[0], a1, a0)
    } else {
        v[0]
    };
    v[1] = p1.wrapping_add(deltas[1]).clamp(min_dc, max_dc);

    let p2 = if !left_edge {
        pred_grad(v[0], l1, l0)
    } else {
        v[0]
    };
    v[2] = p2.wrapping_add(deltas[2]).clamp(min_dc, max_dc);

    v[3] = pred_grad(v[1], v[2], v[0])
        .wrapping_add(deltas[3])
        .clamp(min_dc, max_dc);

    // Quad 1 (top-right): blocks 4, 5, 6, 7
    let p4 = if !top_edge {
        pred_grad(v[1], a2, a1)
    } else {
        v[1]
    };
    v[4] = p4.wrapping_add(deltas[4]).clamp(min_dc, max_dc);

    let p5 = if !top_edge {
        pred_grad(v[4], a3, a2)
    } else {
        v[4]
    };
    v[5] = p5.wrapping_add(deltas[5]).clamp(min_dc, max_dc);

    v[6] = pred_grad(v[4], v[3], v[1])
        .wrapping_add(deltas[6])
        .clamp(min_dc, max_dc);
    v[7] = pred_grad(v[5], v[6], v[4])
        .wrapping_add(deltas[7])
        .clamp(min_dc, max_dc);

    // Quad 2 (bottom-left): blocks 8, 9, 10, 11
    let p8 = if !left_edge {
        pred_grad(v[2], l2, l1)
    } else {
        v[2]
    };
    v[8] = p8.wrapping_add(deltas[8]).clamp(min_dc, max_dc);

    v[9] = pred_grad(v[3], v[8], v[2])
        .wrapping_add(deltas[9])
        .clamp(min_dc, max_dc);

    let p10 = if !left_edge {
        pred_grad(v[8], l3, l2)
    } else {
        v[8]
    };
    v[10] = p10.wrapping_add(deltas[10]).clamp(min_dc, max_dc);

    v[11] = pred_grad(v[9], v[10], v[8])
        .wrapping_add(deltas[11])
        .clamp(min_dc, max_dc);

    // Quad 3 (bottom-right): blocks 12, 13, 14, 15
    v[12] = pred_grad(v[6], v[9], v[3])
        .wrapping_add(deltas[12])
        .clamp(min_dc, max_dc);
    v[13] = pred_grad(v[7], v[12], v[6])
        .wrapping_add(deltas[13])
        .clamp(min_dc, max_dc);
    v[14] = pred_grad(v[12], v[11], v[9])
        .wrapping_add(deltas[14])
        .clamp(min_dc, max_dc);
    v[15] = pred_grad(v[13], v[14], v[12])
        .wrapping_add(deltas[15])
        .clamp(min_dc, max_dc);

    for i in 0..16 {
        dc[i] = v[i] as i32;
    }
}

/// Decode 4-block chroma DC grid (`180013c80`).
fn decode_dc4(
    dc: &mut [i32; 4],
    above: Option<&[i32; 4]>,
    q: u32,
    bs: &mut BitReader,
    flags: u32,
    min_dc: i16,
    max_dc: i16,
) {
    let (deltas, _nonzero) = decode_dc_deltas::<4>(bs, q);
    let left_edge = (flags & 0x20) != 0;
    let top_edge = (flags & 0x88) != 0;

    let l0 = dc[1] as i16;
    let l1 = dc[3] as i16;
    let (al, a0, a1) = match above {
        Some(ab) => (ab[1] as i16, ab[2] as i16, ab[3] as i16),
        None => (0, 0, 0),
    };

    let p0 = match (left_edge, top_edge) {
        (false, false) => pred_grad(l0, a0, al),
        (false, true) => l0,
        (true, false) => a0,
        (true, true) => {
            if above.is_some() {
                0x400
            } else {
                0
            }
        }
    };
    let v0 = p0.wrapping_add(deltas[0]).clamp(min_dc, max_dc);
    let p1 = if !top_edge {
        pred_grad(v0, a1, a0)
    } else {
        v0
    };
    let v1 = p1.wrapping_add(deltas[1]).clamp(min_dc, max_dc);
    let p2 = if !left_edge {
        pred_grad(v0, l1, l0)
    } else {
        v0
    };
    let v2 = p2.wrapping_add(deltas[2]).clamp(min_dc, max_dc);
    let v3 = pred_grad(v1, v2, v0)
        .wrapping_add(deltas[3])
        .clamp(min_dc, max_dc);

    dc[0] = v0 as i32;
    dc[1] = v1 as i32;
    dc[2] = v2 as i32;
    dc[3] = v3 as i32;
}

/// Decode AC coefficients for a 4-block (16x16) quad (`1800123a0`).
///
/// Returns packed 4 bytes of `last_nz` (byte `b` = last nonzero zigzag index in block `b`,
/// or `0` if DC-only).
fn decode_ac_quad(
    mut cbp_bits: u32,
    coeffs: &mut [i16; 256],
    bs: &mut BitReader,
    q: u32,
    qmat: &[[u16; 64]; 4],
    dcs: &[i32; 4],
    info_out: &mut [u16; 4],
    base_flag: u16,
) -> u32 {
    if (cbp_bits & 0xf) == 0 {
        for b in 0..4 {
            let dc = dcs[b] as u16;
            coeffs[b * 64] = dc.wrapping_add(4).wrapping_mul(8) as i16;
            info_out[b] = (dc & 0x1fff) | base_flag | 0x6000;
        }
        return 0;
    }

    let use_alt = (cbp_bits & 0xffff_0000) != 0;
    let vlc_table: &[u8] = if use_alt { &AC_VLC2 } else { &AC_VLC1 };
    let vlc_mask: usize = if use_alt { 0xff } else { 0x1ff };
    let q_phase = (q & 3) as usize;
    let q_shift = (q >> 2) & 31;
    let qm = &qmat[q_phase];
    let mut last_nz_packed = 0u32;

    for b in 0..4 {
        let blk = &mut coeffs[b * 64..(b + 1) * 64];
        let mut nz_count = 0u32;
        let mut last_nz = 0u32;
        if (cbp_bits & 1) != 0 {
            blk.fill(0);
            let mut rem_nz = 0i32;
            let mut idx = 1usize;
            while idx < 64 {
                rem_nz -= 1;
                let pos = if rem_nz < 1 {
                    bs.refill();
                    let entry = vlc_table[(bs.bit_buf as usize) & vlc_mask];
                    let sym = (entry >> 4) as usize;
                    let blen = (entry & 0xf) as u32;
                    bs.consume_fast(blen);
                    let mut run = AC_RUN[sym];
                    rem_nz = AC_COUNT[sym] as i32;
                    if run == 11 {
                        run = (bs.bit_buf as usize) & 0x3f;
                        bs.consume_fast(6);
                    }
                    let p = idx + run;
                    if p >= 64 {
                        break;
                    }
                    p
                } else {
                    idx
                };

                bs.refill();
                let peek = (bs.bit_buf as u32) & 0x7ff;
                let k = (!peek).trailing_zeros();
                let n = k + 1;
                bs.consume_fast(n);
                let mag = if n > 3 {
                    let eb = k - 2;
                    let extra = (bs.bit_buf as u32) & ((1u32 << eb) - 1);
                    bs.consume_fast(eb);
                    extra + 2 + (1u32 << eb)
                } else {
                    n
                };
                let sign = (bs.bit_buf as u32) & 1;
                bs.consume_fast(1);
                let signed = if sign != 0 { -(mag as i32) } else { mag as i32 };
                let zz = ZIGZAG_U8[pos] as usize;
                let deq = ((signed.wrapping_mul(qm[zz] as i32)) << q_shift)
                    .wrapping_add(0x40)
                    >> 7;
                blk[zz] = deq as i16;
                nz_count += 1;
                last_nz = pos as u32;
                idx = pos + 1;
            }
        }
        let dc = dcs[b] as u16;
        blk[0] = dc.wrapping_add(4).wrapping_mul(8) as i16;
        last_nz_packed = (last_nz << 24) | (last_nz_packed >> 8);
        let sp = (u16::from(nz_count < 4) + u16::from(nz_count < 8) + u16::from(nz_count == 0))
            * 0x2000;
        info_out[b] = sp | (dc & 0x1fff) | base_flag;
        cbp_bits = (cbp_bits >> 1) & 0x7fff_ffff;
    }

    last_nz_packed
}

/// Transform a 4-block (16x16) coefficient quad into a 16x16 `i16` buffer (stride `out_stride` in `i16`s).
#[inline]
fn idct_quad(
    coeffs: &[i16; 256],
    mut last_nz_packed: u32,
    out: &mut [i16],
    out_off: usize,
    out_stride: usize,
) {
    let offsets = [0usize, 8, out_stride * 8, out_stride * 8 + 8];
    for b in 0..4 {
        let blk: &[i16; 64] = coeffs[b * 64..(b + 1) * 64].try_into().unwrap();
        let off = out_off + offsets[b];
        if (last_nz_packed & 0xff) == 0 {
            idct8x8_dc(blk[0], out, off, out_stride);
        } else {
            idct8x8(blk, out, off, out_stride);
        }
        last_nz_packed >>= 8;
    }
}

/// Decode a 32x32 intra plane (Y, A, or H) (`180014c80`).
#[allow(clippy::too_many_arguments)]
fn decode_intra_plane16(
    dc: &mut [i32; 16],
    above: &[i32; 8],
    mut dst: Option<(&mut [u8], usize, usize)>,
    bs: &mut BitReader,
    info_out: &mut [u16; 16],
    flags: u32,
    q: u32,
    cbp_state: &mut u32,
) {
    let mut cbp = decode_cbp16(bs, *cbp_state, flags);
    *cbp_state = cbp;
    decode_dc16(dc, Some(above), q, bs, flags, 0, 0x7ff);

    let mut coeffs = [0i16; 256];
    let mut tmp = [0i16; 256];
    for quad in 0..4usize {
        let dcs = [
            dc[quad * 4],
            dc[quad * 4 + 1],
            dc[quad * 4 + 2],
            dc[quad * 4 + 3],
        ];
        let mut qinfo = [0u16; 4];
        let last_nz = decode_ac_quad(
            cbp,
            &mut coeffs,
            bs,
            q,
            &QMAT_INTRA_LUMA,
            &dcs,
            &mut qinfo,
            0x8000,
        );
        info_out[quad * 4..quad * 4 + 4].copy_from_slice(&qinfo);
        cbp >>= 4;

        let is_bot = (quad & 2) != 0;
        let write_this = ((!is_bot && (flags & 4) == 0) || (is_bot && (flags & 3) == 0))
            && last_nz != 0;
        if write_this {
            if let Some((ref mut buf, base_off, stride)) = dst {
                idct_quad(&coeffs, last_nz, &mut tmp, 0, 16);
                let qoff = base_off + (quad & 1) * 16 + if is_bot { stride * 16 } else { 0 };
                clamp_store_16x8(buf, qoff, stride, &tmp, 0, 16);
                clamp_store_16x8(buf, qoff + stride * 8, stride, &tmp, 128, 16);
            }
        }
    }
}

/// Decode a 16x16 intra chroma plane (Cr or Cb) (`180014f00`).
#[allow(clippy::too_many_arguments)]
fn decode_intra_plane4(
    dc: &mut [i32; 4],
    above: &[i32; 4],
    dst: &mut [u8],
    dst_off: usize,
    dst_stride: usize,
    bs: &mut BitReader,
    info_out: &mut [u16; 4],
    flags: u32,
    q: u32,
    cbp_state: &mut u32,
) {
    let cbp = decode_cbp4(bs, *cbp_state);
    *cbp_state = cbp;
    decode_dc4(dc, Some(above), q, bs, flags, 0, 0x7ff);

    let mut coeffs = [0i16; 256];
    let last_nz = decode_ac_quad(
        cbp,
        &mut coeffs,
        bs,
        q,
        &QMAT_INTRA_CHROMA,
        dc,
        info_out,
        0x8000,
    );
    if last_nz != 0 {
        let mut tmp = [0i16; 256];
        idct_quad(&coeffs, last_nz, &mut tmp, 0, 16);
        if (flags & 3) == 0 {
            if (flags & 4) == 0 {
                clamp_store_16x8(dst, dst_off, dst_stride, &tmp, 0, 16);
            }
            clamp_store_16x8(dst, dst_off + dst_stride * 8, dst_stride, &tmp, 128, 16);
        } else {
            clamp_store_16x8(dst, dst_off, dst_stride, &tmp, 0, 16);
        }
    }
}

/// Decode a 32x32 inter plane with residual (Y, A, or H) (`1800152e0`).
#[allow(clippy::too_many_arguments)]
fn decode_inter_plane16(
    mut dst: Option<(&mut [u8], usize, usize)>,
    ref_plane: Option<(&[u8], usize)>,
    bs: &mut BitReader,
    flags: u32,
    clamped_mvs: &[u32; 8],
    q: u32,
    cbp_state: &mut u32,
    scratch: &mut [u8; 256],
) {
    let mut cbp = decode_cbp16(bs, *cbp_state, flags);
    *cbp_state = cbp;

    let ext_bit = (flags >> 18) & 1;
    let min_dc: i16 = if ext_bit != 0 { -2047 } else { -1023 };
    let max_dc: i16 = if ext_bit != 0 { 2047 } else { 1023 };
    let mut dc = [0i32; 16];
    decode_dc16(&mut dc, None, q, bs, 0xa8, min_dc, max_dc);

    let mut res = [0i16; 1024]; // 32x32 residuals (stride 32)
    let mut info = [0u16; 16];
    let mut coeffs = [0i16; 256];

    for quad in 0..4usize {
        let dcs = [
            dc[quad * 4],
            dc[quad * 4 + 1],
            dc[quad * 4 + 2],
            dc[quad * 4 + 3],
        ];
        let mut qinfo = [0u16; 4];
        let last_nz = decode_ac_quad(
            cbp,
            &mut coeffs,
            bs,
            q,
            &QMAT_INTER,
            &dcs,
            &mut qinfo,
            0,
        );
        info[quad * 4..quad * 4 + 4].copy_from_slice(&qinfo);
        cbp >>= 4;

        if ref_plane.is_some() {
            let qoff = (quad & 2) * 256 + (quad & 1) * 16;
            idct_quad(&coeffs, last_nz, &mut res, qoff, 32);

            if (flags & 0x44000) == 0 {
                if (quad & 1) != 0 {
                    // Boundary between left quad's right column and right quad's left column
                    smooth_inter_horiz(
                        &mut res,
                        qoff - 2,
                        32,
                        info[quad * 4 - 3],
                        info[quad * 4],
                    );
                    smooth_inter_horiz(
                        &mut res,
                        qoff + 256 - 2,
                        32,
                        info[quad * 4 - 1],
                        info[quad * 4 + 2],
                    );
                }
                // Internal vertical boundary inside this 16x16 quad
                smooth_inter_horiz(&mut res, qoff + 6, 32, info[quad * 4], info[quad * 4 + 1]);
                smooth_inter_horiz(
                    &mut res,
                    qoff + 256 + 6,
                    32,
                    info[quad * 4 + 2],
                    info[quad * 4 + 3],
                );
            }
        }
    }

    let Some((ref_buf, ref_stride)) = ref_plane else {
        return;
    };

    if (flags & 0x48000) == 0 {
        for col in 0..4usize {
            let x = col * 8;
            let idx_base = (col & 2) * 2 | (col & 1);
            let i0 = info[idx_base];
            let i1 = info[idx_base + 2];
            let i2 = info[idx_base + 8];
            let i3 = info[idx_base + 10];
            smooth_inter_vert(&mut res, 6 * 32 + x, 32, i0, i1);
            smooth_inter_vert(&mut res, 14 * 32 + x, 32, i1, i2);
            smooth_inter_vert(&mut res, 22 * 32 + x, 32, i2, i3);
        }
    }

    let mut pred = [0u8; 256];
    for quad in 0..4usize {
        let to_scratch = if (flags & 7) == 0 {
            false
        } else if (flags & 4) == 0 {
            if quad == 2 {
                continue;
            }
            quad == 3
        } else {
            if quad == 0 {
                continue;
            }
            quad == 1
        };

        let mx = clamped_mvs[quad * 2];
        let my = clamped_mvs[quad * 2 + 1];
        mc_luma_16x16(
            ref_buf,
            ref_stride,
            (mx >> 1) as i32,
            (my >> 1) as i32,
            &mut pred,
            0,
            16,
            (mx & 1) != 0,
            (my & 1) != 0,
        );
        let roff = (quad & 2) * 256 + (quad & 1) * 16;
        if to_scratch {
            add_clamp_16x8(scratch, 0, 16, &res, roff, 32, &pred, 0, 16);
            add_clamp_16x8(scratch, 128, 16, &res, roff + 256, 32, &pred, 128, 16);
        } else if let Some((ref mut dst_buf, dst_base, dst_stride)) = dst {
            let doff = dst_base
                + (quad & 1) * 16
                + if (quad & 2) != 0 { dst_stride * 16 } else { 0 };
            add_clamp_16x8(dst_buf, doff, dst_stride, &res, roff, 32, &pred, 0, 16);
            add_clamp_16x8(
                dst_buf,
                doff + dst_stride * 8,
                dst_stride,
                &res,
                roff + 256,
                32,
                &pred,
                128,
                16,
            );
        }
    }
}

/// Decode a 16x16 inter chroma plane with residual (Cr or Cb) (`180015830`).
#[allow(clippy::too_many_arguments)]
fn decode_inter_plane4(
    dst: &mut [u8],
    dst_off: usize,
    dst_stride: usize,
    ref_buf: &[u8],
    ref_stride: usize,
    bs: &mut BitReader,
    clamped_mvs: &[u32; 8],
    flags: u32,
    q: u32,
    cbp_state: &mut u32,
    scratch: &mut [u8; 128],
) {
    let cbp = decode_cbp4(bs, *cbp_state);
    *cbp_state = cbp;

    let ext_bit = (flags >> 18) & 1;
    let min_dc: i16 = if ext_bit != 0 { -2047 } else { -1023 };
    let max_dc: i16 = if ext_bit != 0 { 2047 } else { 1023 };
    let mut dc = [0i32; 4];
    decode_dc4(&mut dc, None, q, bs, 0x20a8, min_dc, max_dc);

    let mut coeffs = [0i16; 256];
    let mut info = [0u16; 4];
    let last_nz = decode_ac_quad(cbp, &mut coeffs, bs, q, &QMAT_INTER, &dc, &mut info, 0);

    let mut pred = [0u8; 256];
    let pred_offs = [0usize, 8, 128, 136];
    for q_idx in 0..4usize {
        let mx = clamped_mvs[q_idx * 2];
        let my = clamped_mvs[q_idx * 2 + 1];
        mc_chroma_8x8(
            ref_buf,
            ref_stride,
            (mx >> 2) as i32,
            (my >> 2) as i32,
            &mut pred,
            pred_offs[q_idx],
            16,
            mx & 3,
            my & 3,
        );
    }

    let mut res = [0i16; 256];
    idct_quad(&coeffs, last_nz, &mut res, 0, 16);

    if (flags & 0x44000) == 0 {
        smooth_inter_horiz(&mut res, 6, 16, info[0], info[1]);
        smooth_inter_horiz(&mut res, 128 + 6, 16, info[2], info[3]);
    }
    if (flags & 0x48000) == 0 {
        smooth_inter_vert(&mut res, 6 * 16, 16, info[0], info[2]);
        smooth_inter_vert(&mut res, 6 * 16 + 8, 16, info[1], info[3]);
    }

    if (flags & 4) != 0 {
        add_clamp_16x8(scratch, 0, 16, &res, 0, 16, &pred, 0, 16);
    } else {
        add_clamp_16x8(dst, dst_off, dst_stride, &res, 0, 16, &pred, 0, 16);
    }
    if (flags & 3) != 0 {
        add_clamp_16x8(scratch, 0, 16, &res, 128, 16, &pred, 128, 16);
    } else {
        add_clamp_16x8(
            dst,
            dst_off + dst_stride * 8,
            dst_stride,
            &res,
            128,
            16,
            &pred,
            128,
            16,
        );
    }
}

/// Decode motion vector deltas and apply spatial median prediction (`180014140`).
fn decode_mvs(
    bs: &mut BitReader,
    mv_cur: &mut [i32],
    mv_above: &[i32],
    mb_x_idx: usize,
    flags: u32,
) {
    let count = if bs.read_bit() != 0 { 1usize } else { 4usize };
    let base = mb_x_idx * 8;

    for axis in 0..2usize {
        let mut deltas = [0i32; 4];
        for slot in deltas.iter_mut().take(count) {
            bs.refill();
            let v = (bs.bit_buf as u32) & 0xffff;
            if (v & 0xf) != 0 {
                let e = MV_VLC[(v & 0x7f) as usize];
                let blen = (e as u8 & 7) as u32;
                bs.consume_fast(blen);
                *slot = (e >> 3) as i32;
            } else {
                let raw: u32 = if v == 0 {
                    bs.consume_fast(16);
                    bs.read_bits(16).wrapping_add(0xffff)
                } else {
                    let tz = v.trailing_zeros(); // 4..=15
                    let total_bits = tz * 2 + 1;
                    let w = (bs.bit_buf as u32) & ((1u32 << total_bits) - 1);
                    bs.consume_fast(total_bits);
                    (w >> (tz + 1)).wrapping_add((1u32 << tz) - 1)
                };
                let unzz = ((raw >> 1) as i32) ^ -((raw & 1) as i32);
                *slot = unzz;
            }
        }

        let top_edge = (flags & 0x88) != 0;
        let left_edge = (flags & 0x20) != 0;

        let (q0, q1, q2, q3) = if count == 1 {
            let pred = if !top_edge {
                let ab_q2 = mv_above[base + 4 + axis];
                if !left_edge {
                    let al_q3 = mv_above[base - 2 + axis];
                    let l_q1 = mv_cur[base - 6 + axis];
                    median3_i32(ab_q2, al_q3, l_q1)
                } else {
                    let ab_q3 = mv_above[base + 6 + axis];
                    let ab_q0 = mv_above[base + axis];
                    median3_i32(ab_q2, ab_q3, ab_q0)
                }
            } else if left_edge {
                0
            } else {
                let l_q3 = mv_cur[base - 2 + axis];
                let l_q1 = mv_cur[base - 6 + axis];
                let l_q0 = mv_cur[base - 8 + axis];
                median3_i32(l_q1, l_q0, l_q3)
            };
            let val = pred.wrapping_add(deltas[0]);
            (val, val, val, val)
        } else if !top_edge {
            if left_edge {
                let ab_q0 = mv_above[base + axis];
                let ab_q2 = mv_above[base + 4 + axis];
                let ab_q3 = mv_above[base + 6 + axis];
                let q0 = median3_i32(ab_q0, ab_q2, ab_q3).wrapping_add(deltas[0]);
                let q1 = median3_i32(q0, ab_q2, ab_q3).wrapping_add(deltas[1]);
                let q2 = median3_i32(q0, q1, ab_q2).wrapping_add(deltas[2]);
                let q3 = median3_i32(q0, q1, q2).wrapping_add(deltas[3]);
                (q0, q1, q2, q3)
            } else {
                let ab_q2 = mv_above[base + 4 + axis];
                let ab_q3 = mv_above[base + 6 + axis];
                let al_q3 = mv_above[base - 2 + axis];
                let l_q1 = mv_cur[base - 6 + axis];
                let l_q3 = mv_cur[base - 2 + axis];
                let q0 = median3_i32(l_q1, ab_q2, al_q3).wrapping_add(deltas[0]);
                let q1 = median3_i32(q0, ab_q2, ab_q3).wrapping_add(deltas[1]);
                let q2 = median3_i32(q0, l_q1, l_q3).wrapping_add(deltas[2]);
                let q3 = median3_i32(q0, q1, q2).wrapping_add(deltas[3]);
                (q0, q1, q2, q3)
            }
        } else if left_edge {
            let q0 = deltas[0];
            let q1 = deltas[1].wrapping_add(q0);
            let q2 = deltas[2].wrapping_add(q0);
            let q3 = median3_i32(q0, q1, q2).wrapping_add(deltas[3]);
            (q0, q1, q2, q3)
        } else {
            let l_q0 = mv_cur[base - 8 + axis];
            let l_q1 = mv_cur[base - 6 + axis];
            let l_q3 = mv_cur[base - 2 + axis];
            let q0 = median3_i32(l_q1, l_q0, l_q3).wrapping_add(deltas[0]);
            let q2 = median3_i32(q0, l_q1, l_q3).wrapping_add(deltas[2]);
            let q1 = median3_i32(q0, q2, l_q1).wrapping_add(deltas[1]);
            let q3 = median3_i32(q0, q1, q2).wrapping_add(deltas[3]);
            (q0, q1, q2, q3)
        };

        mv_cur[base + axis] = q0;
        mv_cur[base + 2 + axis] = q1;
        mv_cur[base + 4 + axis] = q2;
        mv_cur[base + 6 + axis] = q3;
    }
}

/// Fill MV predictors for an Intra MB in a P-frame (`180022ea0`).
fn intra_mv_median(mv_cur: &mut [i32], mv_above: &[i32], mb_x_idx: usize, flags: u32) {
    let base = mb_x_idx * 8;
    let top_edge = (flags & 0x88) != 0;
    let left_edge = (flags & 0x20) != 0;
    let is_int = (flags & 0x2000) != 0;

    for axis in 0..2usize {
        let (q0, q1, q2, q3) = if !top_edge {
            if !left_edge {
                let ab_q2 = mv_above[base + 4 + axis];
                let ab_q3 = mv_above[base + 6 + axis];
                let al_q3 = mv_above[base - 2 + axis];
                let l_q1 = mv_cur[base - 6 + axis];
                let l_q3 = mv_cur[base - 2 + axis];
                let q0 = median3_i32(l_q1, ab_q2, al_q3);
                let q1 = median3_i32(q0, ab_q3, ab_q2);
                let q2 = if is_int {
                    median3_i32(q0, l_q1, l_q3)
                } else {
                    median3_i32(q0, q1, ab_q2)
                };
                let q3 = median3_i32(q0, q1, q2);
                (q0, q1, q2, q3)
            } else {
                let ab_q0 = mv_above[base + axis];
                let ab_q2 = mv_above[base + 4 + axis];
                let ab_q3 = mv_above[base + 6 + axis];
                let q0 = median3_i32(ab_q0, ab_q2, ab_q3);
                let q1 = median3_i32(q0, ab_q2, ab_q3);
                let q2 = median3_i32(q0, q1, ab_q2);
                let q3 = median3_i32(q0, q1, q2);
                (q0, q1, q2, q3)
            }
        } else if left_edge {
            (0, 0, 0, 0)
        } else {
            let l_q0 = mv_cur[base - 8 + axis];
            let l_q1 = mv_cur[base - 6 + axis];
            let l_q3 = mv_cur[base - 2 + axis];
            let q0 = median3_i32(l_q0, l_q1, l_q3);
            let q2 = median3_i32(q0, l_q1, l_q3);
            let q1 = median3_i32(q0, q2, l_q1);
            let q3 = median3_i32(q0, q1, q2);
            (q0, q1, q2, q3)
        };
        mv_cur[base + axis] = q0;
        mv_cur[base + 2 + axis] = q1;
        mv_cur[base + 4 + axis] = q2;
        mv_cur[base + 6 + axis] = q3;
    }
}

/// Convert relative MVs to clamped absolute half-pel coordinates (`18001dc10`).
fn clamp_mvs(
    mv_cur: &[i32],
    mb_x_idx: usize,
    x_px: i32,
    y_px: i32,
    w2: i32,
    h2: i32,
    flags: u32,
) -> [u32; 8] {
    let base = mb_x_idx * 8;
    let mut out = [0u32; 8];
    let new_clamp = (flags & 0x20000) != 0;

    for q in 0..4usize {
        let qx = mv_cur[base + q * 2]
            .wrapping_add(((q & 1) as i32) * 32)
            .wrapping_add(x_px * 2);
        let qy = mv_cur[base + q * 2 + 1]
            .wrapping_add(((q >> 1) as i32) * 32)
            .wrapping_add(y_px * 2);

        let cx = clamp_axis(qx, w2, new_clamp);
        let cy = clamp_axis(qy, h2, new_clamp);
        out[q * 2] = cx;
        out[q * 2 + 1] = cy;
    }
    out
}

#[inline]
fn clamp_axis(val: i32, limit2: i32, new_clamp: bool) -> u32 {
    let odd = (val & 1) as usize;
    let span = limit2.wrapping_sub(MV_CLAMP[4 + odd]) as u32;
    if !new_clamp {
        let mn = MV_CLAMP[odd] as u32;
        let u = val as u32;
        if u.wrapping_sub(mn) > span {
            if u < mn {
                mn
            } else {
                span
            }
        } else {
            u
        }
    } else {
        let mn = MV_CLAMP[2 + odd];
        if (val.wrapping_sub(mn) as u32) > span {
            if val < mn {
                mn as u32
            } else {
                (mn as u32).wrapping_add(span)
            }
        } else {
            val as u32
        }
    }
}

/// Sliding-window post-filter & in-loop deblocking state (`18001e250`).
struct PostFilterState {
    // 4 rows of 8 u16s + 4 right-edge u16s per luma-sized plane (Y, A, H)
    y_win: [[u16; 8]; 4],
    y_right: [u16; 4],
    a_win: [[u16; 8]; 4],
    a_right: [u16; 4],
    h_win: [[u16; 8]; 4],
    h_right: [u16; 4],
    // 2 rows of 4 u16s + 2 right-edge u16s per chroma plane (Cr, Cb)
    cr_win: [[u16; 4]; 2],
    cr_right: [u16; 2],
    cb_win: [[u16; 4]; 2],
    cb_right: [u16; 2],
    // Delayed horizontal deblock masks (param_1[0x13] = [u32; 3])
    hdeb: [u32; 3],
}

impl PostFilterState {
    fn new() -> Self {
        Self {
            y_win: [[0; 8]; 4],
            y_right: [0; 4],
            a_win: [[0; 8]; 4],
            a_right: [0; 4],
            h_win: [[0; 8]; 4],
            h_right: [0; 4],
            cr_win: [[0; 4]; 2],
            cr_right: [0; 2],
            cb_win: [[0; 4]; 2],
            cb_right: [0; 2],
            hdeb: [0; 3],
        }
    }
}

#[inline]
fn dc_pair_mask(left: u16, right: u16) -> u32 {
    let a = (left as u32).wrapping_add(0x2000) >> 1;
    let b = (right as u32).wrapping_add(0x2000) & 0x10000;
    (a | b) >> 15
}

#[inline]
fn deblock_idx(a: u16, b: u16) -> usize {
    ((((a >> 2) & 0x1800) | (b & 0x6000)) >> 11) as usize
}

#[inline]
fn vert_weight_idx(a: u16, b: u16) -> usize {
    // DEBLOCK_LUT_A returns byte offset into 16-byte weights table divided by 2 (0, 8, 16, ... 112),
    // which is `weight_index * 8`.
    (DEBLOCK_LUT_A[deblock_idx(a, b)] >> 3) as usize
}

#[inline]
fn horiz_pair_byte(a0: u16, b0: u16, a1: u16, b1: u16) -> u8 {
    DEBLOCK_LUT_B[deblock_idx(a0, b0)] | DEBLOCK_LUT_C[deblock_idx(a1, b1)]
}

#[inline]
fn mv_diff_gt1(a: i32, b: i32) -> bool {
    (a.wrapping_sub(b).wrapping_add(1) as u32) > 2
}

#[inline]
fn apply_hdeb_byte(dst: &mut [u8], off: usize, stride: usize, mask_byte: u8) {
    if mask_byte != 0 {
        let lw = (mask_byte & 0xf) as usize;
        let rw = (mask_byte >> 4) as usize;
        deblock_horiz_16(dst, off, stride, lw, rw);
    }
}

/// Apply delayed horizontal deblocks from the previous MB's right half (`18001df20`).
fn flush_delayed_hdeb(
    y_buf: &mut [u8],
    y_off_m2: usize,
    y_stride: usize,
    cr_buf: &mut [u8],
    cr_off_m2: usize,
    cb_buf: &mut [u8],
    cb_off_m2: usize,
    c_stride: usize,
    hdeb: [u32; 3],
    x_right_half: bool,
) {
    // When called mid-row (`x_right_half == false`), `y_off_m2` is at `(x - 32, y - 2)`,
    // and delayed right-half edges sit at `x - 32 - 16` = `(prev_prev_mb.x + 16)`.
    // When called at end-of-row flush (`x_right_half == true`), `y_off_m2` is at `(x, y - 2)`
    // and edges sit at `(x + 16)` (Wait: `pcVar17` was advanced by `+32`, so `pcVar17 + 16`!).
    let y_dx: isize = if x_right_half { 16 } else { -16 };
    let c_dx: isize = if x_right_half { 0 } else { -16 };

    let [d0, d1, d2] = hdeb;
    let y8 = y_stride * 8;
    let c8 = c_stride * 8;

    if d0 != 0 {
        let b0 = (d0 & 0xff) as u8;
        let b1 = ((d0 >> 8) & 0xff) as u8;
        let b2 = ((d0 >> 16) & 0xff) as u8;
        let b3 = ((d0 >> 24) & 0xff) as u8;
        if b0 != 0 {
            apply_hdeb_byte(y_buf, (y_off_m2 as isize - y8 as isize + y_dx) as usize, y_stride, b0);
        }
        if b1 != 0 {
            apply_hdeb_byte(y_buf, (y_off_m2 as isize + y_dx) as usize, y_stride, b1);
        }
        if b2 != 0 {
            apply_hdeb_byte(y_buf, (y_off_m2 as isize + y8 as isize + y_dx) as usize, y_stride, b2);
        }
        if b3 != 0 {
            apply_hdeb_byte(
                y_buf,
                (y_off_m2 as isize + (y8 * 2) as isize + y_dx) as usize,
                y_stride,
                b3,
            );
        }
    }
    if d1 != 0 {
        let b3 = ((d1 >> 24) & 0xff) as u8;
        if b3 != 0 {
            apply_hdeb_byte(
                y_buf,
                (y_off_m2 as isize + (y8 * 3) as isize + y_dx) as usize,
                y_stride,
                b3,
            );
        }
        let c0 = (d1 & 0xff) as u8;
        let c1 = ((d1 >> 8) & 0xff) as u8;
        let c2 = ((d1 >> 16) & 0xff) as u8;
        if c0 != 0 {
            apply_hdeb_byte(cr_buf, (cr_off_m2 as isize - c8 as isize + c_dx) as usize, c_stride, c0);
        }
        if c1 != 0 {
            apply_hdeb_byte(cr_buf, (cr_off_m2 as isize + c_dx) as usize, c_stride, c1);
        }
        if c2 != 0 {
            apply_hdeb_byte(cr_buf, (cr_off_m2 as isize + c8 as isize + c_dx) as usize, c_stride, c2);
        }
    }
    if d2 != 0 {
        let c0 = (d2 & 0xff) as u8;
        let c1 = ((d2 >> 8) & 0xff) as u8;
        let c2 = ((d2 >> 16) & 0xff) as u8;
        if c0 != 0 {
            apply_hdeb_byte(cb_buf, (cb_off_m2 as isize - c8 as isize + c_dx) as usize, c_stride, c0);
        }
        if c1 != 0 {
            apply_hdeb_byte(cb_buf, (cb_off_m2 as isize + c_dx) as usize, c_stride, c1);
        }
        if c2 != 0 {
            apply_hdeb_byte(cb_buf, (cb_off_m2 as isize + c8 as isize + c_dx) as usize, c_stride, c2);
        }
    }
}

/// Load a 16-block plane's info words into the sliding window (shifting previous MB left).
#[inline]
fn shift_luma_win(win: &mut [[u16; 8]; 4], right: &mut [u16; 4], cur: &[u16; 16], is_first: bool) {
    if is_first {
        for r in 0..4 {
            win[r] = [0; 8];
        }
    } else {
        for r in 0..4 {
            // Save old [4..8] -> [0..4] BEFORE overwriting [4..8]
            let prev = [win[r][4], win[r][5], win[r][6], win[r][7]];
            win[r][0..4].copy_from_slice(&prev);
            win[r][4] = right[r];
        }
    }
    // Row 0: blocks 0, 1, 4, 5
    win[0][5] = cur[0];
    win[0][6] = cur[1];
    win[0][7] = cur[4];
    right[0] = cur[5];
    // Row 1: blocks 2, 3, 6, 7
    win[1][5] = cur[2];
    win[1][6] = cur[3];
    win[1][7] = cur[6];
    right[1] = cur[7];
    // Row 2: blocks 8, 9, 12, 13
    win[2][5] = cur[8];
    win[2][6] = cur[9];
    win[2][7] = cur[12];
    right[2] = cur[13];
    // Row 3: blocks 10, 11, 14, 15
    win[3][5] = cur[10];
    win[3][6] = cur[11];
    win[3][7] = cur[14];
    right[3] = cur[15];
}

#[inline]
fn flush_shift_luma_win(win: &mut [[u16; 8]; 4], right: &[u16; 4]) {
    for r in 0..4 {
        let prev = [win[r][4], win[r][5], win[r][6], win[r][7]];
        win[r][0..4].copy_from_slice(&prev);
        win[r][4] = right[r];
        win[r][5] = 0;
    }
}

#[inline]
fn shift_chroma_win(win: &mut [[u16; 4]; 2], right: &mut [u16; 2], cur: &[u16; 4], is_first: bool) {
    if is_first {
        win[0] = [0, 0, 0, cur[0]];
        win[1] = [0, 0, 0, cur[2]];
    } else {
        win[0] = [win[0][2], win[0][3], right[0], cur[0]];
        win[1] = [win[1][2], win[1][3], right[1], cur[2]];
    }
    right[0] = cur[1];
    right[1] = cur[3];
}

#[inline]
fn flush_shift_chroma_win(win: &mut [[u16; 4]; 2], right: &[u16; 2]) {
    win[0] = [win[0][2], win[0][3], right[0], 0];
    win[1] = [win[1][2], win[1][3], right[1], 0];
}

#[inline]
fn smooth_luma_plane_dc(
    win: &[[u16; 8]; 4],
    above2: &[u16],
    above1: &[u16],
    ab_col: usize,
    dst: &mut [u8],
    dst_off: usize,
    stride: usize,
    thresh: i16,
    flags: u32,
) {
    let s8 = stride * 8;
    // Delayed bottom row of MB above (`y - 8`)
    if (flags & 0x84) == 0 && (above1[ab_col + 1] & 0x8000) != 0 {
        let m0 = dc_pair_mask(above1[ab_col + 1], above1[ab_col + 2]);
        let m1 = dc_pair_mask(above1[ab_col + 3], above1[ab_col + 4]);
        if m0 != 0 {
            smooth_dc_pair(
                above2[ab_col..ab_col + 4].try_into().unwrap(),
                above1[ab_col..ab_col + 4].try_into().unwrap(),
                win[0][0..4].try_into().unwrap(),
                thresh,
                dst,
                dst_off - s8,
                stride,
                m0,
            );
        }
        if m1 != 0 {
            smooth_dc_pair(
                above2[ab_col + 2..ab_col + 6].try_into().unwrap(),
                above1[ab_col + 2..ab_col + 6].try_into().unwrap(),
                win[0][2..6].try_into().unwrap(),
                thresh,
                dst,
                dst_off - s8 + 16,
                stride,
                m1,
            );
        }
    }

    if (win[0][1] & 0x8000) == 0 {
        return;
    }

    let top_row = (flags & 0x80) != 0;
    if (flags & 4) == 0 {
        // Row 0 (`y + 0`)
        let m0 = dc_pair_mask(win[0][1], win[0][2]);
        let m1 = dc_pair_mask(win[0][3], win[0][4]);
        if m0 != 0 {
            let r_above: [u16; 4] = if top_row {
                win[0][0..4].try_into().unwrap()
            } else {
                above1[ab_col..ab_col + 4].try_into().unwrap()
            };
            smooth_dc_pair(
                r_above,
                win[0][0..4].try_into().unwrap(),
                win[1][0..4].try_into().unwrap(),
                thresh,
                dst,
                dst_off,
                stride,
                m0,
            );
        }
        if m1 != 0 {
            let r_above: [u16; 4] = if top_row {
                win[0][2..6].try_into().unwrap()
            } else {
                above1[ab_col + 2..ab_col + 6].try_into().unwrap()
            };
            smooth_dc_pair(
                r_above,
                win[0][2..6].try_into().unwrap(),
                win[1][2..6].try_into().unwrap(),
                thresh,
                dst,
                dst_off + 16,
                stride,
                m1,
            );
        }

        // Row 1 (`y + 8`)
        let m0 = dc_pair_mask(win[1][1], win[1][2]);
        let m1 = dc_pair_mask(win[1][3], win[1][4]);
        if m0 != 0 {
            smooth_dc_pair(
                win[0][0..4].try_into().unwrap(),
                win[1][0..4].try_into().unwrap(),
                win[2][0..4].try_into().unwrap(),
                thresh,
                dst,
                dst_off + s8,
                stride,
                m0,
            );
        }
        if m1 != 0 {
            smooth_dc_pair(
                win[0][2..6].try_into().unwrap(),
                win[1][2..6].try_into().unwrap(),
                win[2][2..6].try_into().unwrap(),
                thresh,
                dst,
                dst_off + s8 + 16,
                stride,
                m1,
            );
        }
    }

    if (flags & 3) == 0 {
        // Row 2 (`y + 16`)
        let m0 = dc_pair_mask(win[2][1], win[2][2]);
        let m1 = dc_pair_mask(win[2][3], win[2][4]);
        if m0 != 0 {
            smooth_dc_pair(
                win[1][0..4].try_into().unwrap(),
                win[2][0..4].try_into().unwrap(),
                win[3][0..4].try_into().unwrap(),
                thresh,
                dst,
                dst_off + s8 * 2,
                stride,
                m0,
            );
        }
        if m1 != 0 {
            smooth_dc_pair(
                win[1][2..6].try_into().unwrap(),
                win[2][2..6].try_into().unwrap(),
                win[3][2..6].try_into().unwrap(),
                thresh,
                dst,
                dst_off + s8 * 2 + 16,
                stride,
                m1,
            );
        }

        // Row 3 (`y + 24`, only on last MB row of frame!)
        if (flags & 0x100) != 0 {
            let m0 = dc_pair_mask(win[3][1], win[3][2]);
            let m1 = dc_pair_mask(win[3][3], win[3][4]);
            if m0 != 0 {
                smooth_dc_pair(
                    win[2][0..4].try_into().unwrap(),
                    win[3][0..4].try_into().unwrap(),
                    win[3][0..4].try_into().unwrap(),
                    thresh,
                    dst,
                    dst_off + s8 * 3,
                    stride,
                    m0,
                );
            }
            if m1 != 0 {
                smooth_dc_pair(
                    win[2][2..6].try_into().unwrap(),
                    win[3][2..6].try_into().unwrap(),
                    win[3][2..6].try_into().unwrap(),
                    thresh,
                    dst,
                    dst_off + s8 * 3 + 16,
                    stride,
                    m1,
                );
            }
        }
    }
}

/// Per-MB DC smoothing and in-loop edge deblocking (`18001e250`, single-pass mode).
#[allow(clippy::too_many_arguments)]
fn postfilter_mb(
    pf: &mut PostFilterState,
    y_ab2: &mut [u16],
    y_ab1: &mut [u16],
    a_ab2: &mut [u16],
    a_ab1: &mut [u16],
    h_ab2: &mut [u16],
    h_ab1: &mut [u16],
    cr_ab2: &mut [u16],
    cr_ab1: &mut [u16],
    cb_ab2: &mut [u16],
    cb_ab1: &mut [u16],
    cur_y_info: &[u16; 16],
    cur_cr_info: &[u16; 4],
    cur_cb_info: &[u16; 4],
    cur_a_info: &[u16; 16],
    cur_h_info: &[u16; 16],
    mut flags: u32,
    planes: &mut FramePlanes,
    mb_x_idx: usize,
    y_row_px: usize,
    mv_cur: &[i32],
    mv_above: &[i32],
    thresh: i16,
) {
    let ya_s = planes.ya_pitch;
    let c_s = planes.crcb_pitch;
    let ya_row_off = y_row_px * ya_s;
    let c_row_off = (y_row_px >> 1) * c_s;

    let is_first = (flags & 0x20) != 0;
    if is_first {
        pf.hdeb = [0; 3];
        if planes.dst_a.is_some() {
            shift_luma_win(&mut pf.a_win, &mut pf.a_right, cur_a_info, true);
        }
        if planes.dst_h.is_some() {
            shift_luma_win(&mut pf.h_win, &mut pf.h_right, cur_h_info, true);
        }
        shift_luma_win(&mut pf.y_win, &mut pf.y_right, cur_y_info, true);
        shift_chroma_win(&mut pf.cr_win, &mut pf.cr_right, cur_cr_info, true);
        shift_chroma_win(&mut pf.cb_win, &mut pf.cb_right, cur_cb_info, true);
        if (flags & 0x40) == 0 {
            return;
        }
    } else {
        if planes.dst_a.is_some()
            && ((pf.a_win[0][1] | cur_a_info[0] | pf.a_win[0][5] | pf.a_win[0][0]) & 0x8000) != 0
        {
            shift_luma_win(&mut pf.a_win, &mut pf.a_right, cur_a_info, false);
        }
        if planes.dst_h.is_some()
            && ((pf.h_win[0][1] | pf.h_win[0][5] | pf.h_win[0][0] | cur_h_info[0]) & 0x8000) != 0
        {
            shift_luma_win(&mut pf.h_win, &mut pf.h_right, cur_h_info, false);
        }
        if ((pf.y_win[0][1] | pf.y_win[0][0] | pf.y_win[0][5] | cur_y_info[0]) & 0x8000) != 0 {
            shift_luma_win(&mut pf.y_win, &mut pf.y_right, cur_y_info, false);
            shift_chroma_win(&mut pf.cr_win, &mut pf.cr_right, cur_cr_info, false);
            shift_chroma_win(&mut pf.cb_win, &mut pf.cb_right, cur_cb_info, false);
        }
    }

    // `delayed_mx` is the MB column being post-filtered:
    // - On normal iteration (`!is_first`), starts at `mb_x_idx - 1`
    // - On flush iteration (after `flags & 0x40`), runs once more for `mb_x_idx`!
    let mut delayed_mx: isize = mb_x_idx as isize - 1;
    let mut flushing = is_first; // if single-MB wide row, jump directly to flush step

    loop {
        if flushing {
            if (flags & 0x400) != 0 {
                // Finalize last MB's right-edge saved above-row entries & flush trailing hdebs
                let end_ya = (delayed_mx as usize + 1) * 4;
                let end_c = (delayed_mx as usize + 1) * 2;
                if planes.dst_a.is_some() {
                    a_ab2[end_ya] = pf.a_win[2][4];
                    a_ab1[end_ya] = pf.a_win[3][4];
                }
                if planes.dst_h.is_some() {
                    h_ab2[end_ya] = pf.h_win[2][4];
                    h_ab1[end_ya] = pf.h_win[3][4];
                }
                y_ab2[end_ya] = pf.y_win[2][4];
                y_ab1[end_ya] = pf.y_win[3][4];
                cr_ab2[end_c] = pf.cr_win[0][2];
                cr_ab1[end_c] = pf.cr_win[1][2];
                cb_ab2[end_c] = pf.cb_win[0][2];
                cb_ab1[end_c] = pf.cb_win[1][2];

                if (flags & 0x8000) == 0 {
                    let y_off = ya_row_off + (delayed_mx as usize) * 32;
                    let c_off = c_row_off + (delayed_mx as usize) * 16;
                    flush_delayed_hdeb(
                        planes.dst_y,
                        y_off.wrapping_sub(ya_s * 2),
                        ya_s,
                        planes.dst_cr,
                        c_off.wrapping_sub(c_s * 2),
                        planes.dst_cb,
                        c_off.wrapping_sub(c_s * 2),
                        c_s,
                        pf.hdeb,
                        true,
                    );
                }
                return;
            }
            // Advance to the last MB column and shift window with right-edge sentinel 0
            delayed_mx += 1;
            let mut u = flags | 0x400;
            if (flags & 0x20) != 0 {
                u |= 0x600;
            }
            flags = ((u >> 4) & 0x003c_0000) | (u & 0xfc03_ffff);
            if planes.dst_a.is_some() {
                flush_shift_luma_win(&mut pf.a_win, &pf.a_right);
            }
            if planes.dst_h.is_some() {
                flush_shift_luma_win(&mut pf.h_win, &pf.h_right);
            }
            flush_shift_luma_win(&mut pf.y_win, &pf.y_right);
            flush_shift_chroma_win(&mut pf.cr_win, &pf.cr_right);
            flush_shift_chroma_win(&mut pf.cb_win, &pf.cb_right);
        }

        let dmx = delayed_mx as usize;
        let ya_ab = dmx * 4;
        let c_ab = dmx * 2;
        let y_off = ya_row_off + dmx * 32;
        let c_off = c_row_off + dmx * 16;
        let ya8 = ya_s * 8;
        let c8 = c_s * 8;

        // 1. Alpha & HDR DC smoothing
        if let Some(ref mut dst_a) = planes.dst_a {
            smooth_luma_plane_dc(&pf.a_win, a_ab2, a_ab1, ya_ab, dst_a, y_off, ya_s, thresh, flags);
        }
        if let Some(ref mut dst_h) = planes.dst_h {
            smooth_luma_plane_dc(&pf.h_win, h_ab2, h_ab1, ya_ab, dst_h, y_off, ya_s, thresh, flags);
        }

        // 2. Y, Cr, Cb DC smoothing + vertical/horizontal edge deblocking
        let top_row = (flags & 0x80) != 0;
        let cur_is_intra = (pf.y_win[0][1] & 0x8000) != 0;

        if top_row {
            // Top MB row (`y == 0`): smooth row 0 of Y, Cr, Cb
            let m0 = dc_pair_mask(pf.y_win[0][1], pf.y_win[0][2]);
            let m1 = dc_pair_mask(pf.y_win[0][3], pf.y_win[0][4]);
            let mcr = dc_pair_mask(pf.cr_win[0][1], pf.cr_win[0][2]);
            let mcb = dc_pair_mask(pf.cb_win[0][1], pf.cb_win[0][2]);
            if m0 != 0 {
                let w: [u16; 4] = pf.y_win[0][0..4].try_into().unwrap();
                let wb: [u16; 4] = pf.y_win[1][0..4].try_into().unwrap();
                smooth_dc_pair(w, w, wb, thresh, planes.dst_y, y_off, ya_s, m0);
            }
            if m1 != 0 {
                let w: [u16; 4] = pf.y_win[0][2..6].try_into().unwrap();
                let wb: [u16; 4] = pf.y_win[1][2..6].try_into().unwrap();
                smooth_dc_pair(w, w, wb, thresh, planes.dst_y, y_off + 16, ya_s, m1);
            }
            if mcr != 0 {
                smooth_dc_pair(pf.cr_win[0], pf.cr_win[0], pf.cr_win[1], thresh, planes.dst_cr, c_off, c_s, mcr);
            }
            if mcb != 0 {
                smooth_dc_pair(pf.cb_win[0], pf.cb_win[0], pf.cb_win[1], thresh, planes.dst_cb, c_off, c_s, mcb);
            }
        } else {
            // Non-top row: first finish the delayed bottom row (`y - 8`) of the MB above!
            if (flags & 4) == 0 {
                let ab_intra = (y_ab1[ya_ab + 1] & 0x8000) != 0;
                if !ab_intra {
                    // Above MB was Inter: vertical MV-discontinuity deblocking on row `y - 8`
                    if (flags & 0x4000) == 0 {
                        let mv_ab_base = dmx * 8;
                        if (flags & 0x40200) == 0 {
                            if (y_ab1[ya_ab] & 0x8000) == 0 {
                                // Above-left was also Inter
                                let d0 = mv_diff_gt1(mv_above[mv_ab_base - 2], mv_above[mv_ab_base + 4]);
                                let d1 = mv_diff_gt1(mv_above[mv_ab_base - 1], mv_above[mv_ab_base + 5]);
                                if d0 || d1 {
                                    deblock_vert_8(planes.dst_y, y_off - ya8, ya_s, 10);
                                    deblock_vert_8(planes.dst_cr, c_off - c8, c_s, 10);
                                    deblock_vert_8(planes.dst_cb, c_off - c8, c_s, 10);
                                }
                            } else {
                                let wy = vert_weight_idx(y_ab1[ya_ab], y_ab1[ya_ab + 1]);
                                if wy != 0 {
                                    deblock_vert_8(planes.dst_y, y_off - ya8, ya_s, wy);
                                }
                                let wcr = vert_weight_idx(cr_ab1[c_ab], cr_ab1[c_ab + 1]);
                                if wcr != 0 {
                                    deblock_vert_8(planes.dst_cr, c_off - c8, c_s, wcr);
                                }
                                let wcb = vert_weight_idx(cb_ab1[c_ab], cb_ab1[c_ab + 1]);
                                if wcb != 0 {
                                    deblock_vert_8(planes.dst_cb, c_off - c8, c_s, wcb);
                                }
                            }
                        }
                        if (flags & 0x10_0000) == 0 {
                            let d0 = mv_diff_gt1(mv_above[mv_ab_base + 4], mv_above[mv_ab_base + 6]);
                            let d1 = mv_diff_gt1(mv_above[mv_ab_base + 5], mv_above[mv_ab_base + 7]);
                            if d0 || d1 {
                                deblock_vert_8(planes.dst_y, y_off - ya8 + 16, ya_s, 10);
                                deblock_vert_8(planes.dst_cr, c_off - c8 + 8, c_s, 10);
                                deblock_vert_8(planes.dst_cb, c_off - c8 + 8, c_s, 10);
                            }
                        }
                    }
                } else {
                    // Above MB was Intra: DC smooth row `y - 8` + vertical deblock row `y - 8`
                    let m0 = dc_pair_mask(y_ab1[ya_ab + 1], y_ab1[ya_ab + 2]);
                    let m1 = dc_pair_mask(y_ab1[ya_ab + 3], y_ab1[ya_ab + 4]);
                    let mcr = dc_pair_mask(cr_ab1[c_ab + 1], cr_ab1[c_ab + 2]);
                    let mcb = dc_pair_mask(cb_ab1[c_ab + 1], cb_ab1[c_ab + 2]);
                    if m0 != 0 {
                        smooth_dc_pair(
                            y_ab2[ya_ab..ya_ab + 4].try_into().unwrap(),
                            y_ab1[ya_ab..ya_ab + 4].try_into().unwrap(),
                            pf.y_win[0][0..4].try_into().unwrap(),
                            thresh,
                            planes.dst_y,
                            y_off - ya8,
                            ya_s,
                            m0,
                        );
                    }
                    if m1 != 0 {
                        smooth_dc_pair(
                            y_ab2[ya_ab + 2..ya_ab + 6].try_into().unwrap(),
                            y_ab1[ya_ab + 2..ya_ab + 6].try_into().unwrap(),
                            pf.y_win[0][2..6].try_into().unwrap(),
                            thresh,
                            planes.dst_y,
                            y_off - ya8 + 16,
                            ya_s,
                            m1,
                        );
                    }
                    if mcr != 0 {
                        smooth_dc_pair(
                            cr_ab2[c_ab..c_ab + 4].try_into().unwrap(),
                            cr_ab1[c_ab..c_ab + 4].try_into().unwrap(),
                            pf.cr_win[0],
                            thresh,
                            planes.dst_cr,
                            c_off - c8,
                            c_s,
                            mcr,
                        );
                    }
                    if mcb != 0 {
                        smooth_dc_pair(
                            cb_ab2[c_ab..c_ab + 4].try_into().unwrap(),
                            cb_ab1[c_ab..c_ab + 4].try_into().unwrap(),
                            pf.cb_win[0],
                            thresh,
                            planes.dst_cb,
                            c_off - c8,
                            c_s,
                            mcb,
                        );
                    }
                    if (flags & 0x4000) == 0 {
                        if (flags & 0x40200) == 0 {
                            let wy = vert_weight_idx(y_ab1[ya_ab], y_ab1[ya_ab + 1]);
                            if wy != 0 {
                                deblock_vert_8(planes.dst_y, y_off - ya8, ya_s, wy);
                            }
                            let wcr = vert_weight_idx(cr_ab1[c_ab], cr_ab1[c_ab + 1]);
                            if wcr != 0 {
                                deblock_vert_8(planes.dst_cr, c_off - c8, c_s, wcr);
                            }
                            let wcb = vert_weight_idx(cb_ab1[c_ab], cb_ab1[c_ab + 1]);
                            if wcb != 0 {
                                deblock_vert_8(planes.dst_cb, c_off - c8, c_s, wcb);
                            }
                        }
                        if (flags & 0x08_0000) == 0 {
                            let wy = vert_weight_idx(y_ab1[ya_ab + 1], y_ab1[ya_ab + 2]);
                            if wy != 0 {
                                deblock_vert_8(planes.dst_y, y_off - ya8 + 8, ya_s, wy);
                            }
                        }
                        if (flags & 0x10_0000) == 0 {
                            let wy = vert_weight_idx(y_ab1[ya_ab + 2], y_ab1[ya_ab + 3]);
                            if wy != 0 {
                                deblock_vert_8(planes.dst_y, y_off - ya8 + 16, ya_s, wy);
                            }
                            let wcr = vert_weight_idx(cr_ab1[c_ab + 1], cr_ab1[c_ab + 2]);
                            if wcr != 0 {
                                deblock_vert_8(planes.dst_cr, c_off - c8 + 8, c_s, wcr);
                            }
                            let wcb = vert_weight_idx(cb_ab1[c_ab + 1], cb_ab1[c_ab + 2]);
                            if wcb != 0 {
                                deblock_vert_8(planes.dst_cb, c_off - c8 + 8, c_s, wcb);
                            }
                        }
                        if (flags & 0x20_0000) == 0 {
                            let wy = vert_weight_idx(y_ab1[ya_ab + 3], y_ab1[ya_ab + 4]);
                            if wy != 0 {
                                deblock_vert_8(planes.dst_y, y_off - ya8 + 24, ya_s, wy);
                            }
                        }
                    }
                }
            }

            // Now smooth row 0 of current delayed MB if it is Intra
            if cur_is_intra && (flags & 4) == 0 {
                let m0 = dc_pair_mask(pf.y_win[0][1], pf.y_win[0][2]);
                let m1 = dc_pair_mask(pf.y_win[0][3], pf.y_win[0][4]);
                let mcr = dc_pair_mask(pf.cr_win[0][1], pf.cr_win[0][2]);
                let mcb = dc_pair_mask(pf.cb_win[0][1], pf.cb_win[0][2]);
                if m0 != 0 {
                    smooth_dc_pair(
                        y_ab1[ya_ab..ya_ab + 4].try_into().unwrap(),
                        pf.y_win[0][0..4].try_into().unwrap(),
                        pf.y_win[1][0..4].try_into().unwrap(),
                        thresh,
                        planes.dst_y,
                        y_off,
                        ya_s,
                        m0,
                    );
                }
                if m1 != 0 {
                    smooth_dc_pair(
                        y_ab1[ya_ab + 2..ya_ab + 6].try_into().unwrap(),
                        pf.y_win[0][2..6].try_into().unwrap(),
                        pf.y_win[1][2..6].try_into().unwrap(),
                        thresh,
                        planes.dst_y,
                        y_off + 16,
                        ya_s,
                        m1,
                    );
                }
                if mcr != 0 {
                    smooth_dc_pair(
                        cr_ab1[c_ab..c_ab + 4].try_into().unwrap(),
                        pf.cr_win[0],
                        pf.cr_win[1],
                        thresh,
                        planes.dst_cr,
                        c_off,
                        c_s,
                        mcr,
                    );
                }
                if mcb != 0 {
                    smooth_dc_pair(
                        cb_ab1[c_ab..c_ab + 4].try_into().unwrap(),
                        pf.cb_win[0],
                        pf.cb_win[1],
                        thresh,
                        planes.dst_cb,
                        c_off,
                        c_s,
                        mcb,
                    );
                }
            }
        }

        if cur_is_intra {
            // Rows 1, 2, (3 if bottom) DC smooth + vertical deblocking for Intra MB
            if (flags & 4) == 0 {
                let m0 = dc_pair_mask(pf.y_win[1][1], pf.y_win[1][2]);
                let m1 = dc_pair_mask(pf.y_win[1][3], pf.y_win[1][4]);
                if m0 != 0 {
                    smooth_dc_pair(
                        pf.y_win[0][0..4].try_into().unwrap(),
                        pf.y_win[1][0..4].try_into().unwrap(),
                        pf.y_win[2][0..4].try_into().unwrap(),
                        thresh,
                        planes.dst_y,
                        y_off + ya8,
                        ya_s,
                        m0,
                    );
                }
                if m1 != 0 {
                    smooth_dc_pair(
                        pf.y_win[0][2..6].try_into().unwrap(),
                        pf.y_win[1][2..6].try_into().unwrap(),
                        pf.y_win[2][2..6].try_into().unwrap(),
                        thresh,
                        planes.dst_y,
                        y_off + ya8 + 16,
                        ya_s,
                        m1,
                    );
                }
                if (flags & 0x4000) == 0 {
                    if (flags & 0x40200) == 0 {
                        let w0 = vert_weight_idx(pf.y_win[0][0], pf.y_win[0][1]);
                        if w0 != 0 {
                            deblock_vert_8(planes.dst_y, y_off, ya_s, w0);
                        }
                        let wcr = vert_weight_idx(pf.cr_win[0][0], pf.cr_win[0][1]);
                        if wcr != 0 {
                            deblock_vert_8(planes.dst_cr, c_off, c_s, wcr);
                        }
                        let wcb = vert_weight_idx(pf.cb_win[0][0], pf.cb_win[0][1]);
                        if wcb != 0 {
                            deblock_vert_8(planes.dst_cb, c_off, c_s, wcb);
                        }
                        let w1 = vert_weight_idx(pf.y_win[1][0], pf.y_win[1][1]);
                        if w1 != 0 {
                            deblock_vert_8(planes.dst_y, y_off + ya8, ya_s, w1);
                        }
                    }
                    if (flags & 0x08_0000) == 0 {
                        let w0 = vert_weight_idx(pf.y_win[0][1], pf.y_win[0][2]);
                        if w0 != 0 {
                            deblock_vert_8(planes.dst_y, y_off + 8, ya_s, w0);
                        }
                        let w1 = vert_weight_idx(pf.y_win[1][1], pf.y_win[1][2]);
                        if w1 != 0 {
                            deblock_vert_8(planes.dst_y, y_off + ya8 + 8, ya_s, w1);
                        }
                    }
                    if (flags & 0x10_0000) == 0 {
                        let w0 = vert_weight_idx(pf.y_win[0][2], pf.y_win[0][3]);
                        if w0 != 0 {
                            deblock_vert_8(planes.dst_y, y_off + 16, ya_s, w0);
                        }
                        let w1 = vert_weight_idx(pf.y_win[1][2], pf.y_win[1][3]);
                        if w1 != 0 {
                            deblock_vert_8(planes.dst_y, y_off + ya8 + 16, ya_s, w1);
                        }
                        let wcr = vert_weight_idx(pf.cr_win[0][1], pf.cr_win[0][2]);
                        if wcr != 0 {
                            deblock_vert_8(planes.dst_cr, c_off + 8, c_s, wcr);
                        }
                        let wcb = vert_weight_idx(pf.cb_win[0][1], pf.cb_win[0][2]);
                        if wcb != 0 {
                            deblock_vert_8(planes.dst_cb, c_off + 8, c_s, wcb);
                        }
                    }
                    if (flags & 0x20_0000) == 0 {
                        let w0 = vert_weight_idx(pf.y_win[0][3], pf.y_win[0][4]);
                        if w0 != 0 {
                            deblock_vert_8(planes.dst_y, y_off + 24, ya_s, w0);
                        }
                        let w1 = vert_weight_idx(pf.y_win[1][3], pf.y_win[1][4]);
                        if w1 != 0 {
                            deblock_vert_8(planes.dst_y, y_off + ya8 + 24, ya_s, w1);
                        }
                    }
                }
            }
            if (flags & 3) == 0 {
                let m0 = dc_pair_mask(pf.y_win[2][1], pf.y_win[2][2]);
                let m1 = dc_pair_mask(pf.y_win[2][3], pf.y_win[2][4]);
                if m0 != 0 {
                    smooth_dc_pair(
                        pf.y_win[1][0..4].try_into().unwrap(),
                        pf.y_win[2][0..4].try_into().unwrap(),
                        pf.y_win[3][0..4].try_into().unwrap(),
                        thresh,
                        planes.dst_y,
                        y_off + ya8 * 2,
                        ya_s,
                        m0,
                    );
                }
                if m1 != 0 {
                    smooth_dc_pair(
                        pf.y_win[1][2..6].try_into().unwrap(),
                        pf.y_win[2][2..6].try_into().unwrap(),
                        pf.y_win[3][2..6].try_into().unwrap(),
                        thresh,
                        planes.dst_y,
                        y_off + ya8 * 2 + 16,
                        ya_s,
                        m1,
                    );
                }
                if (flags & 0x4000) == 0 {
                    if (flags & 0x40200) == 0 {
                        let w = vert_weight_idx(pf.y_win[2][0], pf.y_win[2][1]);
                        if w != 0 {
                            deblock_vert_8(planes.dst_y, y_off + ya8 * 2, ya_s, w);
                        }
                    }
                    if (flags & 0x08_0000) == 0 {
                        let w = vert_weight_idx(pf.y_win[2][1], pf.y_win[2][2]);
                        if w != 0 {
                            deblock_vert_8(planes.dst_y, y_off + ya8 * 2 + 8, ya_s, w);
                        }
                    }
                    if (flags & 0x10_0000) == 0 {
                        let w = vert_weight_idx(pf.y_win[2][2], pf.y_win[2][3]);
                        if w != 0 {
                            deblock_vert_8(planes.dst_y, y_off + ya8 * 2 + 16, ya_s, w);
                        }
                    }
                    if (flags & 0x20_0000) == 0 {
                        let w = vert_weight_idx(pf.y_win[2][3], pf.y_win[2][4]);
                        if w != 0 {
                            deblock_vert_8(planes.dst_y, y_off + ya8 * 2 + 24, ya_s, w);
                        }
                    }
                }
                if (flags & 0x100) != 0 {
                    let m0 = dc_pair_mask(pf.y_win[3][1], pf.y_win[3][2]);
                    let m1 = dc_pair_mask(pf.y_win[3][3], pf.y_win[3][4]);
                    let mcr = dc_pair_mask(pf.cr_win[1][1], pf.cr_win[1][2]);
                    let mcb = dc_pair_mask(pf.cb_win[1][1], pf.cb_win[1][2]);
                    if m0 != 0 {
                        let w: [u16; 4] = pf.y_win[3][0..4].try_into().unwrap();
                        smooth_dc_pair(
                            pf.y_win[2][0..4].try_into().unwrap(),
                            w,
                            w,
                            thresh,
                            planes.dst_y,
                            y_off + ya8 * 3,
                            ya_s,
                            m0,
                        );
                    }
                    if m1 != 0 {
                        let w: [u16; 4] = pf.y_win[3][2..6].try_into().unwrap();
                        smooth_dc_pair(
                            pf.y_win[2][2..6].try_into().unwrap(),
                            w,
                            w,
                            thresh,
                            planes.dst_y,
                            y_off + ya8 * 3 + 16,
                            ya_s,
                            m1,
                        );
                    }
                    if mcr != 0 {
                        smooth_dc_pair(pf.cr_win[0], pf.cr_win[1], pf.cr_win[1], thresh, planes.dst_cr, c_off + c8, c_s, mcr);
                    }
                    if mcb != 0 {
                        smooth_dc_pair(pf.cb_win[0], pf.cb_win[1], pf.cb_win[1], thresh, planes.dst_cb, c_off + c8, c_s, mcb);
                    }
                    if (flags & 0x4000) == 0 {
                        if (flags & 0x40200) == 0 {
                            let w = vert_weight_idx(pf.y_win[3][0], pf.y_win[3][1]);
                            if w != 0 {
                                deblock_vert_8(planes.dst_y, y_off + ya8 * 3, ya_s, w);
                            }
                            let wcr = vert_weight_idx(pf.cr_win[1][0], pf.cr_win[1][1]);
                            if wcr != 0 {
                                deblock_vert_8(planes.dst_cr, c_off + c8, c_s, wcr);
                            }
                            let wcb = vert_weight_idx(pf.cb_win[1][0], pf.cb_win[1][1]);
                            if wcb != 0 {
                                deblock_vert_8(planes.dst_cb, c_off + c8, c_s, wcb);
                            }
                        }
                        if (flags & 0x08_0000) == 0 {
                            let w = vert_weight_idx(pf.y_win[3][1], pf.y_win[3][2]);
                            if w != 0 {
                                deblock_vert_8(planes.dst_y, y_off + ya8 * 3 + 8, ya_s, w);
                            }
                        }
                        if (flags & 0x10_0000) == 0 {
                            let w = vert_weight_idx(pf.y_win[3][2], pf.y_win[3][3]);
                            if w != 0 {
                                deblock_vert_8(planes.dst_y, y_off + ya8 * 3 + 16, ya_s, w);
                            }
                            let wcr = vert_weight_idx(pf.cr_win[1][1], pf.cr_win[1][2]);
                            if wcr != 0 {
                                deblock_vert_8(planes.dst_cr, c_off + c8 + 8, c_s, wcr);
                            }
                            let wcb = vert_weight_idx(pf.cb_win[1][1], pf.cb_win[1][2]);
                            if wcb != 0 {
                                deblock_vert_8(planes.dst_cb, c_off + c8 + 8, c_s, wcb);
                            }
                        }
                        if (flags & 0x20_0000) == 0 {
                            let w = vert_weight_idx(pf.y_win[3][3], pf.y_win[3][4]);
                            if w != 0 {
                                deblock_vert_8(planes.dst_y, y_off + ya8 * 3 + 24, ya_s, w);
                            }
                        }
                    }
                }
            }

            // Horizontal edge deblocking for Intra MB
            if (flags & 0x8000) == 0 {
                let y_m2 = y_off.wrapping_sub(ya_s * 2);
                let c_m2 = c_off.wrapping_sub(c_s * 2);
                flush_delayed_hdeb(
                    planes.dst_y,
                    y_m2,
                    ya_s,
                    planes.dst_cr,
                    c_m2,
                    planes.dst_cb,
                    c_m2,
                    c_s,
                    pf.hdeb,
                    false,
                );

                let mut next_d0 = 0u32;
                let mut next_d1 = 0u32;
                let mut next_d2 = 0u32;

                if (flags & 0x84) == 0 {
                    let ab_intra = (y_ab1[ya_ab + 1] & 0x8000) != 0;
                    if !ab_intra {
                        if (flags & 0x1000_0000) == 0 {
                            let b = horiz_pair_byte(
                                y_ab1[ya_ab + 1],
                                pf.y_win[0][1],
                                y_ab1[ya_ab + 2],
                                pf.y_win[0][2],
                            );
                            apply_hdeb_byte(planes.dst_y, y_m2, ya_s, b);
                            next_d0 |= (horiz_pair_byte(
                                y_ab1[ya_ab + 3],
                                pf.y_win[0][3],
                                y_ab1[ya_ab + 4],
                                pf.y_win[0][4],
                            ) as u32)
                                << 8;
                        }
                        if (flags & 0x0400_0000) == 0 {
                            let ab_mv = dmx * 8;
                            if mv_diff_gt1(mv_above[ab_mv + 4], mv_above[ab_mv])
                                || mv_diff_gt1(mv_above[ab_mv + 5], mv_above[ab_mv + 1])
                            {
                                next_d1 |= 0x0a;
                                next_d2 |= 0x0a;
                            }
                            if mv_diff_gt1(mv_above[ab_mv + 6], mv_above[ab_mv + 2])
                                || mv_diff_gt1(mv_above[ab_mv + 7], mv_above[ab_mv + 3])
                            {
                                next_d1 |= 0xa0;
                                next_d2 |= 0xa0;
                            }
                        }
                    } else {
                        if (flags & 0x0800_0000) == 0 {
                            let b = horiz_pair_byte(
                                y_ab2[ya_ab + 1],
                                y_ab1[ya_ab + 1],
                                y_ab2[ya_ab + 2],
                                y_ab1[ya_ab + 2],
                            );
                            apply_hdeb_byte(planes.dst_y, y_m2 - ya8, ya_s, b);
                            next_d0 |= horiz_pair_byte(
                                y_ab2[ya_ab + 3],
                                y_ab1[ya_ab + 3],
                                y_ab2[ya_ab + 4],
                                y_ab1[ya_ab + 4],
                            ) as u32;
                        }
                        if (flags & 0x0400_0000) == 0 {
                            next_d1 |= horiz_pair_byte(
                                cr_ab2[c_ab + 1],
                                cr_ab1[c_ab + 1],
                                cr_ab2[c_ab + 2],
                                cr_ab1[c_ab + 2],
                            ) as u32;
                            next_d2 |= horiz_pair_byte(
                                cb_ab2[c_ab + 1],
                                cb_ab1[c_ab + 1],
                                cb_ab2[c_ab + 2],
                                cb_ab1[c_ab + 2],
                            ) as u32;
                        }
                        if (flags & 0x1000_0000) == 0 {
                            let b = horiz_pair_byte(
                                y_ab1[ya_ab + 1],
                                pf.y_win[0][1],
                                y_ab1[ya_ab + 2],
                                pf.y_win[0][2],
                            );
                            apply_hdeb_byte(planes.dst_y, y_m2, ya_s, b);
                            next_d0 |= (horiz_pair_byte(
                                y_ab1[ya_ab + 3],
                                pf.y_win[0][3],
                                y_ab1[ya_ab + 4],
                                pf.y_win[0][4],
                            ) as u32)
                                << 8;
                        }
                    }
                    if (flags & 0x1000_0000) == 0 {
                        next_d1 |= (horiz_pair_byte(
                            cr_ab1[c_ab + 1],
                            pf.cr_win[0][1],
                            cr_ab1[c_ab + 2],
                            pf.cr_win[0][2],
                        ) as u32)
                            << 8;
                        next_d2 |= (horiz_pair_byte(
                            cb_ab1[c_ab + 1],
                            pf.cb_win[0][1],
                            cb_ab1[c_ab + 2],
                            pf.cb_win[0][2],
                        ) as u32)
                            << 8;
                    }
                }
                if (flags & 0x2000_0004) == 0 {
                    let b = horiz_pair_byte(
                        pf.y_win[0][1],
                        pf.y_win[1][1],
                        pf.y_win[0][2],
                        pf.y_win[1][2],
                    );
                    apply_hdeb_byte(planes.dst_y, y_m2.wrapping_add(ya8), ya_s, b);
                    next_d0 |= (horiz_pair_byte(
                        pf.y_win[0][3],
                        pf.y_win[1][3],
                        pf.y_win[0][4],
                        pf.y_win[1][4],
                    ) as u32)
                        << 16;
                }
                if (flags & 3) == 0 {
                    if (flags & 0x4000_0000) == 0 && (flags & 4) == 0 {
                        let b = horiz_pair_byte(
                            pf.y_win[1][1],
                            pf.y_win[2][1],
                            pf.y_win[1][2],
                            pf.y_win[2][2],
                        );
                        apply_hdeb_byte(planes.dst_y, y_m2.wrapping_add(ya8 * 2), ya_s, b);
                        next_d0 |= (horiz_pair_byte(
                            pf.y_win[1][3],
                            pf.y_win[2][3],
                            pf.y_win[1][4],
                            pf.y_win[2][4],
                        ) as u32)
                            << 24;
                    }
                    if (flags & 0x100) != 0 {
                        if (flags & 0x8000_0000) == 0 {
                            let b = horiz_pair_byte(
                                pf.y_win[2][1],
                                pf.y_win[3][1],
                                pf.y_win[2][2],
                                pf.y_win[3][2],
                            );
                            apply_hdeb_byte(planes.dst_y, y_m2.wrapping_add(ya8 * 3), ya_s, b);
                            next_d1 |= (horiz_pair_byte(
                                pf.y_win[2][3],
                                pf.y_win[3][3],
                                pf.y_win[2][4],
                                pf.y_win[3][4],
                            ) as u32)
                                << 24;
                        }
                        if (flags & 0x4000_0000) == 0 {
                            next_d1 |= (horiz_pair_byte(
                                pf.cr_win[0][1],
                                pf.cr_win[1][1],
                                pf.cr_win[0][2],
                                pf.cr_win[1][2],
                            ) as u32)
                                << 16;
                            next_d2 |= (horiz_pair_byte(
                                pf.cb_win[0][1],
                                pf.cb_win[1][1],
                                pf.cb_win[0][2],
                                pf.cb_win[1][2],
                            ) as u32)
                                << 16;
                        }
                    }
                }
                pf.hdeb = [next_d0, next_d1, next_d2];
            }
        } else {
            // Inter MB vertical & horizontal deblocking (`LAB_18001f4b6`)
            let mv_base = dmx * 8;
            if (flags & 0x4000) == 0 {
                let left_intra = (pf.y_win[0][0] & 0x8000) != 0;
                if !left_intra {
                    if (flags & 4) == 0 {
                        if (flags & 0x40200) == 0
                            && (mv_diff_gt1(mv_cur[mv_base - 6], mv_cur[mv_base])
                                || mv_diff_gt1(mv_cur[mv_base - 5], mv_cur[mv_base + 1]))
                        {
                            deblock_vert_8(planes.dst_y, y_off, ya_s, 10);
                            deblock_vert_8(planes.dst_y, y_off + ya8, ya_s, 10);
                            deblock_vert_8(planes.dst_cr, c_off, c_s, 10);
                            deblock_vert_8(planes.dst_cb, c_off, c_s, 10);
                        }
                        if (flags & 0x10_0000) == 0
                            && (mv_diff_gt1(mv_cur[mv_base], mv_cur[mv_base + 2])
                                || mv_diff_gt1(mv_cur[mv_base + 1], mv_cur[mv_base + 3]))
                        {
                            deblock_vert_8(planes.dst_y, y_off + 16, ya_s, 10);
                            deblock_vert_8(planes.dst_y, y_off + ya8 + 16, ya_s, 10);
                            deblock_vert_8(planes.dst_cr, c_off + 8, c_s, 10);
                            deblock_vert_8(planes.dst_cb, c_off + 8, c_s, 10);
                        }
                    }
                    if (flags & 3) == 0 {
                        if (flags & 0x40200) == 0
                            && (mv_diff_gt1(mv_cur[mv_base - 2], mv_cur[mv_base + 4])
                                || mv_diff_gt1(mv_cur[mv_base - 1], mv_cur[mv_base + 5]))
                        {
                            deblock_vert_8(planes.dst_y, y_off + ya8 * 2, ya_s, 10);
                            if (flags & 0x100) != 0 {
                                deblock_vert_8(planes.dst_y, y_off + ya8 * 3, ya_s, 10);
                                deblock_vert_8(planes.dst_cr, c_off + c8, c_s, 10);
                                deblock_vert_8(planes.dst_cb, c_off + c8, c_s, 10);
                            }
                        }
                        if (flags & 0x10_0000) == 0
                            && (mv_diff_gt1(mv_cur[mv_base + 4], mv_cur[mv_base + 6])
                                || mv_diff_gt1(mv_cur[mv_base + 5], mv_cur[mv_base + 7]))
                        {
                            deblock_vert_8(planes.dst_y, y_off + ya8 * 2 + 16, ya_s, 10);
                            if (flags & 0x100) != 0 {
                                deblock_vert_8(planes.dst_y, y_off + ya8 * 3 + 16, ya_s, 10);
                                deblock_vert_8(planes.dst_cr, c_off + c8 + 8, c_s, 10);
                                deblock_vert_8(planes.dst_cb, c_off + c8 + 8, c_s, 10);
                            }
                        }
                    }
                } else {
                    if (flags & 4) == 0 {
                        if (flags & 0x40200) == 0 {
                            let w0 = vert_weight_idx(pf.y_win[0][0], pf.y_win[0][1]);
                            if w0 != 0 {
                                deblock_vert_8(planes.dst_y, y_off, ya_s, w0);
                            }
                            let wcr = vert_weight_idx(pf.cr_win[0][0], pf.cr_win[0][1]);
                            if wcr != 0 {
                                deblock_vert_8(planes.dst_cr, c_off, c_s, wcr);
                            }
                            let wcb = vert_weight_idx(pf.cb_win[0][0], pf.cb_win[0][1]);
                            if wcb != 0 {
                                deblock_vert_8(planes.dst_cb, c_off, c_s, wcb);
                            }
                            let w1 = vert_weight_idx(pf.y_win[1][0], pf.y_win[1][1]);
                            if w1 != 0 {
                                deblock_vert_8(planes.dst_y, y_off + ya8, ya_s, w1);
                            }
                        }
                        if (flags & 0x10_0000) == 0
                            && (mv_diff_gt1(mv_cur[mv_base], mv_cur[mv_base + 2])
                                || mv_diff_gt1(mv_cur[mv_base + 1], mv_cur[mv_base + 3]))
                        {
                            deblock_vert_8(planes.dst_y, y_off + 16, ya_s, 10);
                            deblock_vert_8(planes.dst_y, y_off + ya8 + 16, ya_s, 10);
                            deblock_vert_8(planes.dst_cr, c_off + 8, c_s, 10);
                            deblock_vert_8(planes.dst_cb, c_off + 8, c_s, 10);
                        }
                    }
                    if (flags & 3) == 0 {
                        if (flags & 0x40200) == 0 {
                            let w2 = vert_weight_idx(pf.y_win[2][0], pf.y_win[2][1]);
                            if w2 != 0 {
                                deblock_vert_8(planes.dst_y, y_off + ya8 * 2, ya_s, w2);
                            }
                            if (flags & 0x100) != 0 {
                                let w3 = vert_weight_idx(pf.y_win[3][0], pf.y_win[3][1]);
                                if w3 != 0 {
                                    deblock_vert_8(planes.dst_y, y_off + ya8 * 3, ya_s, w3);
                                }
                                let wcr = vert_weight_idx(pf.cr_win[1][0], pf.cr_win[1][1]);
                                if wcr != 0 {
                                    deblock_vert_8(planes.dst_cr, c_off + c8, c_s, wcr);
                                }
                                let wcb = vert_weight_idx(pf.cb_win[1][0], pf.cb_win[1][1]);
                                if wcb != 0 {
                                    deblock_vert_8(planes.dst_cb, c_off + c8, c_s, wcb);
                                }
                            }
                        }
                        if (flags & 0x10_0000) == 0
                            && (mv_diff_gt1(mv_cur[mv_base + 4], mv_cur[mv_base + 6])
                                || mv_diff_gt1(mv_cur[mv_base + 5], mv_cur[mv_base + 7]))
                        {
                            deblock_vert_8(planes.dst_y, y_off + ya8 * 2 + 16, ya_s, 10);
                            if (flags & 0x100) != 0 {
                                deblock_vert_8(planes.dst_y, y_off + ya8 * 3 + 16, ya_s, 10);
                                deblock_vert_8(planes.dst_cr, c_off + c8 + 8, c_s, 10);
                                deblock_vert_8(planes.dst_cb, c_off + c8 + 8, c_s, 10);
                            }
                        }
                    }
                }
            }

            if (flags & 0x8000) == 0 {
                let y_m2 = y_off.wrapping_sub(ya_s * 2);
                let c_m2 = c_off.wrapping_sub(c_s * 2);
                flush_delayed_hdeb(
                    planes.dst_y,
                    y_m2,
                    ya_s,
                    planes.dst_cr,
                    c_m2,
                    planes.dst_cb,
                    c_m2,
                    c_s,
                    pf.hdeb,
                    false,
                );

                let mut next_d0 = 0u32;
                let mut next_d1 = 0u32;
                let mut next_d2 = 0u32;

                if (flags & 0x84) == 0 {
                    let ab_intra = (y_ab1[ya_ab + 1] & 0x8000) != 0;
                    if !ab_intra {
                        if (flags & 0x1000_0000) == 0 {
                            if mv_diff_gt1(mv_cur[mv_base], mv_above[mv_base + 4])
                                || mv_diff_gt1(mv_cur[mv_base + 1], mv_above[mv_base + 5])
                            {
                                deblock_horiz_16(planes.dst_y, y_m2, ya_s, 10, 10);
                                next_d1 |= 0x0a00;
                                next_d2 |= 0x0a00;
                            }
                            if mv_diff_gt1(mv_cur[mv_base + 2], mv_above[mv_base + 6])
                                || mv_diff_gt1(mv_cur[mv_base + 3], mv_above[mv_base + 7])
                            {
                                next_d0 |= 0xaa00;
                                next_d1 |= 0xa000;
                                next_d2 |= 0xa000;
                            }
                        }
                        if (flags & 0x0400_0000) == 0 {
                            if mv_diff_gt1(mv_above[mv_base + 4], mv_above[mv_base])
                                || mv_diff_gt1(mv_above[mv_base + 5], mv_above[mv_base + 1])
                            {
                                next_d1 |= 0x0a;
                                next_d2 |= 0x0a;
                            }
                            if mv_diff_gt1(mv_above[mv_base + 6], mv_above[mv_base + 2])
                                || mv_diff_gt1(mv_above[mv_base + 7], mv_above[mv_base + 3])
                            {
                                next_d1 |= 0xa0;
                                next_d2 |= 0xa0;
                            }
                        }
                    } else {
                        if (flags & 0x0800_0000) == 0 {
                            let b = horiz_pair_byte(
                                y_ab2[ya_ab + 1],
                                y_ab1[ya_ab + 1],
                                y_ab2[ya_ab + 2],
                                y_ab1[ya_ab + 2],
                            );
                            apply_hdeb_byte(planes.dst_y, y_m2 - ya8, ya_s, b);
                            next_d0 |= horiz_pair_byte(
                                y_ab2[ya_ab + 3],
                                y_ab1[ya_ab + 3],
                                y_ab2[ya_ab + 4],
                                y_ab1[ya_ab + 4],
                            ) as u32;
                        }
                        if (flags & 0x0400_0000) == 0 {
                            next_d1 |= horiz_pair_byte(
                                cr_ab2[c_ab + 1],
                                cr_ab1[c_ab + 1],
                                cr_ab2[c_ab + 2],
                                cr_ab1[c_ab + 2],
                            ) as u32;
                            next_d2 |= horiz_pair_byte(
                                cb_ab2[c_ab + 1],
                                cb_ab1[c_ab + 1],
                                cb_ab2[c_ab + 2],
                                cb_ab1[c_ab + 2],
                            ) as u32;
                        }
                        if (flags & 0x1000_0000) == 0 {
                            let b = horiz_pair_byte(
                                y_ab1[ya_ab + 1],
                                pf.y_win[0][1],
                                y_ab1[ya_ab + 2],
                                pf.y_win[0][2],
                            );
                            apply_hdeb_byte(planes.dst_y, y_m2, ya_s, b);
                            next_d0 |= (horiz_pair_byte(
                                y_ab1[ya_ab + 3],
                                pf.y_win[0][3],
                                y_ab1[ya_ab + 4],
                                pf.y_win[0][4],
                            ) as u32)
                                << 8;
                            next_d1 |= (horiz_pair_byte(
                                cr_ab1[c_ab + 1],
                                pf.cr_win[0][1],
                                cr_ab1[c_ab + 2],
                                pf.cr_win[0][2],
                            ) as u32)
                                << 8;
                            next_d2 |= (horiz_pair_byte(
                                cb_ab1[c_ab + 1],
                                pf.cb_win[0][1],
                                cb_ab1[c_ab + 2],
                                pf.cb_win[0][2],
                            ) as u32)
                                << 8;
                        }
                    }
                }
                if (flags & 3) == 0 {
                    if (flags & 0x4000_0000) == 0 && (flags & 4) == 0 {
                        if mv_diff_gt1(mv_cur[mv_base], mv_cur[mv_base + 4])
                            || mv_diff_gt1(mv_cur[mv_base + 1], mv_cur[mv_base + 5])
                        {
                            deblock_horiz_16(planes.dst_y, y_m2.wrapping_add(ya8 * 2), ya_s, 10, 10);
                        }
                        if mv_diff_gt1(mv_cur[mv_base + 2], mv_cur[mv_base + 6])
                            || mv_diff_gt1(mv_cur[mv_base + 3], mv_cur[mv_base + 7])
                        {
                            next_d0 |= 0xaa00_0000;
                        }
                    }
                    if (flags & 0x4000_0100) == 0x100 {
                        if mv_diff_gt1(mv_cur[mv_base + 4], mv_cur[mv_base])
                            || mv_diff_gt1(mv_cur[mv_base + 5], mv_cur[mv_base + 1])
                        {
                            next_d1 |= 0x000a_0000;
                            next_d2 |= 0x000a_0000;
                        }
                        if mv_diff_gt1(mv_cur[mv_base + 6], mv_cur[mv_base + 2])
                            || mv_diff_gt1(mv_cur[mv_base + 7], mv_cur[mv_base + 3])
                        {
                            next_d1 |= 0x00a0_0000;
                            next_d2 |= 0x00a0_0000;
                        }
                    }
                }
                pf.hdeb = [next_d0, next_d1, next_d2];
            }
        }

        // Save bottom two rows of delayed MB into `ab2` / `ab1` for the next MB row
        if planes.dst_a.is_some() {
            a_ab2[ya_ab..ya_ab + 4].copy_from_slice(&pf.a_win[2][0..4]);
            a_ab1[ya_ab..ya_ab + 4].copy_from_slice(&pf.a_win[3][0..4]);
        }
        if planes.dst_h.is_some() {
            h_ab2[ya_ab..ya_ab + 4].copy_from_slice(&pf.h_win[2][0..4]);
            h_ab1[ya_ab..ya_ab + 4].copy_from_slice(&pf.h_win[3][0..4]);
        }
        y_ab2[ya_ab..ya_ab + 4].copy_from_slice(&pf.y_win[2][0..4]);
        y_ab1[ya_ab..ya_ab + 4].copy_from_slice(&pf.y_win[3][0..4]);
        cr_ab2[c_ab..c_ab + 2].copy_from_slice(&pf.cr_win[0][0..2]);
        cr_ab1[c_ab..c_ab + 2].copy_from_slice(&pf.cr_win[1][0..2]);
        cb_ab2[c_ab..c_ab + 2].copy_from_slice(&pf.cb_win[0][0..2]);
        cb_ab1[c_ab..c_ab + 2].copy_from_slice(&pf.cb_win[1][0..2]);

        if (flags & 0x40) == 0 {
            return;
        }
        flushing = true;
    }
}

/// Decode one complete Bink 2 (`KB2f`..`KB2n`, integer path) video packet into `planes`.
///
/// Returns `Ok(())` on success or `Err(&'static str)` if the packet is malformed or uses
/// the legacy floating-point path (`frame_flags & 0x2000 == 0`).
pub fn decode_frame(
    packet: &[u8],
    slices: &SliceInfo,
    is_keyframe: bool,
    has_alpha: bool,
    has_hdr: bool,
    planes: &mut FramePlanes,
) -> Result<(), &'static str> {
    if packet.len() < 8 {
        return Err("bink2 packet too short");
    }
    let mut frame_flags = u32::from_le_bytes([packet[0], packet[1], packet[2], packet[3]]);
    if (frame_flags & 0x2000) == 0 {
        return Err("bink2 legacy float path (bit 13 == 0) is not supported");
    }

    let ya_w = planes.ya_width;
    let ya_h = planes.ya_height;
    let ya_s = planes.ya_pitch;
    let c_s = planes.crcb_pitch;
    if ya_w == 0 || ya_h == 0 || (ya_w & 31) != 0 || (ya_h & 15) != 0 {
        return Err("bink2 invalid plane dimensions");
    }

    let mb_cols = ya_w >> 5;
    let w2 = (ya_w * 2) as i32;
    let h2 = (ya_h * 2) as i32;

    let mut alpha_coded = has_alpha && planes.dst_a.is_some();
    let const_alpha = (frame_flags & 0x80000) != 0;
    let const_alpha_val = (frame_flags >> 24) as u8;
    if const_alpha {
        alpha_coded = false;
        if planes.dst_a.is_none() {
            frame_flags &= !0x80000;
        }
    }
    let hdr_coded = has_hdr && planes.dst_h.is_some();

    let hdr_offset = if slices.num_slices < 3 {
        8usize
    } else {
        (slices.num_slices as usize) * 4
    };
    if hdr_offset > packet.len() {
        return Err("bink2 slice header truncated");
    }

    let mut bs = BitReader::new(packet, hdr_offset);

    let row_skip_len = ((ya_h >> 3) + 7) >> 3;
    let col_skip_len = ((ya_w >> 3) + 7) >> 3;
    let mut row_skip = vec![0u8; row_skip_len.max(1) + 4];
    let mut col_skip = vec![0u8; col_skip_len.max(1) + 4];

    if (frame_flags & 0x10000) != 0 {
        if (frame_flags & 0x8000) == 0 && (ya_h >> 3) > 1 {
            decode_rle_bitmap(&mut row_skip, 1, &mut bs, ((ya_h >> 3) - 1) as u32);
        }
        if (frame_flags & 0x4000) == 0 && (ya_w >> 3) > 1 {
            decode_rle_bitmap(&mut col_skip, 1, &mut bs, ((ya_w >> 3) - 1) as u32);
        }
    }

    let thresh = ((frame_flags & 0xff) << 3) as i16;
    let base_flags = frame_flags & 0x2e000;

    // Row state buffers (double-buffered current/above rows)
    let mut mv_row_a = vec![0i32; mb_cols * 8];
    let mut mv_row_b = vec![0i32; mb_cols * 8];
    let mut q_row_a = vec![[16u8; 2]; mb_cols];
    let mut q_row_b = vec![[16u8; 2]; mb_cols];
    let mut cur_is_a = true;

    let ya_info_len = mb_cols * 4 + 2;
    let c_info_len = mb_cols * 2 + 2;
    let mut y_ab2 = vec![0u16; ya_info_len];
    let mut y_ab1 = vec![0u16; ya_info_len];
    let mut a_ab2 = vec![0u16; ya_info_len];
    let mut a_ab1 = vec![0u16; ya_info_len];
    let mut h_ab2 = vec![0u16; ya_info_len];
    let mut h_ab1 = vec![0u16; ya_info_len];
    let mut cr_ab2 = vec![0u16; c_info_len];
    let mut cr_ab1 = vec![0u16; c_info_len];
    let mut cb_ab2 = vec![0u16; c_info_len];
    let mut cb_ab1 = vec![0u16; c_info_len];

    let mut pf = PostFilterState::new();
    let mut scratch_y = [0u8; 256];
    let mut scratch_a = [0u8; 256];
    let mut scratch_h = [0u8; 256];
    let mut scratch_cr = [0u8; 128];
    let mut scratch_cb = [0u8; 128];

    let mut cur_slice = 0usize;
    let mut y_px = 0usize;
    let mut row_carry = 0u32;

    while cur_slice < slices.num_slices as usize {
        let slice_end = slices.slice_ends[cur_slice] as usize;
        let mut slice_flags = if y_px == 0 {
            0x80u32
        } else {
            let prev_y = y_px - 32;
            let nib = (row_skip[prev_y >> 6] >> (((prev_y >> 3) & 4) as u32)) as u32;
            row_carry | (nib << 28) | 8
        };
        slice_flags |= base_flags;
        if is_keyframe {
            slice_flags |= 0x10000;
        }

        let mut row_flags_state = slice_flags;

        while y_px < slice_end {
            let nib = (row_skip[y_px >> 6] >> (((y_px >> 3) & 4) as u32)) as u32;
            row_flags_state = (nib << 28)
                | ((row_flags_state >> 4) & 0x0c00_0000)
                | (row_flags_state & 0x03ff_ffff);
            let mut row_flags = row_flags_state;
            let next_y = y_px + 32;
            if ya_h <= next_y {
                row_flags |= 0x100;
                if ya_h < next_y {
                    row_flags |= 0x101;
                }
            }

            // Fill constant alpha row if flagged
            if const_alpha {
                if let Some(ref mut dst_a) = planes.dst_a {
                    if (row_flags & 4) == 0 {
                        for r in 0..16 {
                            let off = (y_px + r) * ya_s;
                            dst_a[off..off + ya_w].fill(const_alpha_val);
                        }
                    }
                    if (row_flags & 3) == 0 {
                        for r in 16..32 {
                            let off = (y_px + r) * ya_s;
                            dst_a[off..off + ya_w].fill(const_alpha_val);
                        }
                    }
                }
            }

            // Split current/above MV and Q rows cleanly
            let (mv_cur, mv_above, q_cur, q_above) = if cur_is_a {
                (&mut mv_row_a[..], &mv_row_b[..], &mut q_row_a[..], &q_row_b[..])
            } else {
                (&mut mv_row_b[..], &mv_row_a[..], &mut q_row_b[..], &q_row_a[..])
            };

            if (row_flags & 0x88) == 0 {
                q_cur[0] = q_above[0];
            } else {
                q_cur[0] = [16, 16];
            }

            let mut mtf = [2u8, 3, 1, 0];
            let mut cbp_intra_y = 0u32;
            let mut cbp_intra_cr = 0u32;
            let mut cbp_intra_cb = 0u32;
            let mut cbp_intra_a = 0u32;
            let mut cbp_intra_h = 0u32;
            let mut cbp_inter_y = 0u32;
            let mut cbp_inter_cr = 0u32;
            let mut cbp_inter_cb = 0u32;
            let mut cbp_inter_a = 0u32;
            let mut cbp_inter_h = 0u32;

            let mut dc_y = [0i32; 16];
            let mut dc_a = [0i32; 16];
            let mut dc_h = [0i32; 16];
            let mut dc_cr = [0i32; 4];
            let mut dc_cb = [0i32; 4];

            let mut col_carry = 0u32;

            for mx in 0..mb_cols {
                let x_px = mx * 32;
                let mut f = (col_carry & 0x03fc_0000) | row_flags;
                if mx == 0 {
                    f |= 0x20;
                }
                if mx + 1 >= mb_cols {
                    f |= 0x40;
                }
                if mx == 1 {
                    f |= 0x200;
                }
                let cnib = ((col_skip[x_px >> 6] >> (((x_px >> 3) & 4) as u32)) & 0xf) as u32;
                let mb_flags = (cnib << 22) | ((f >> 4) & 0x003c_0000) | (f & 0xfc03_ffff);
                col_carry = mb_flags;

                let plane_flags = (f & 0x3ffff) | (frame_flags & 0x40000);
                let y_off = y_px * ya_s + x_px;
                let c_off = (y_px >> 1) * c_s + (x_px >> 1);

                let mode = if is_keyframe {
                    0u8
                } else {
                    decode_mode_mtf(&mut bs, &mut mtf)
                };

                let mut cur_y_info = [0u16; 16];
                let mut cur_cr_info = [0u16; 4];
                let mut cur_cb_info = [0u16; 4];
                let mut cur_a_info = [0u16; 16];
                let mut cur_h_info = [0u16; 16];

                match mode {
                    0 => {
                        // Intra MB
                        let mut above_y = [0i32; 8];
                        let mut above_a = [0i32; 8];
                        let mut above_h = [0i32; 8];
                        let mut above_cr = [0i32; 4];
                        let mut above_cb = [0i32; 4];

                        if (f & 0x88) == 0 {
                            let ya_idx = mx * 4;
                            let c_idx = mx * 2;
                            // Sign-extend 12-bit DCs from above row info (`180022c20`)
                            above_y[3] = ((y_ab1[ya_idx] << 4) as i16 >> 4) as i32;
                            for k in 0..4 {
                                above_y[4 + k] = ((y_ab1[ya_idx + 1 + k] << 4) as i16 >> 4) as i32;
                            }
                            above_cr[1] = ((cr_ab1[c_idx] << 4) as i16 >> 4) as i32;
                            above_cr[2] = ((cr_ab1[c_idx + 1] << 4) as i16 >> 4) as i32;
                            above_cr[3] = ((cr_ab1[c_idx + 2] << 4) as i16 >> 4) as i32;
                            above_cb[1] = ((cb_ab1[c_idx] << 4) as i16 >> 4) as i32;
                            above_cb[2] = ((cb_ab1[c_idx + 1] << 4) as i16 >> 4) as i32;
                            above_cb[3] = ((cb_ab1[c_idx + 2] << 4) as i16 >> 4) as i32;
                            if alpha_coded {
                                above_a[3] = ((a_ab1[ya_idx] << 4) as i16 >> 4) as i32;
                                for k in 0..4 {
                                    above_a[4 + k] = ((a_ab1[ya_idx + 1 + k] << 4) as i16 >> 4) as i32;
                                }
                            }
                            if hdr_coded {
                                above_h[3] = ((h_ab1[ya_idx] << 4) as i16 >> 4) as i32;
                                for k in 0..4 {
                                    above_h[4 + k] = ((h_ab1[ya_idx + 1 + k] << 4) as i16 >> 4) as i32;
                                }
                            }

                            let ya8 = ya_s * 8;
                            let c8 = c_s * 8;
                            if (y_ab1[ya_idx] & 0x8000) == 0 && (mb_flags & 0x20) == 0 {
                                above_y[3] = avg_8x8_pavgb(planes.dst_y, y_off - ya8 - 8, ya_s);
                                above_cr[1] = avg_8x8_pavgb(planes.dst_cr, c_off - c8 - 8, c_s);
                                above_cb[1] = avg_8x8_pavgb(planes.dst_cb, c_off - c8 - 8, c_s);
                                if alpha_coded {
                                    if let Some(ref buf) = planes.dst_a {
                                        above_a[3] = avg_8x8_pavgb(buf, y_off - ya8 - 8, ya_s);
                                    }
                                }
                                if hdr_coded {
                                    if let Some(ref buf) = planes.dst_h {
                                        above_h[3] = avg_8x8_pavgb(buf, y_off - ya8 - 8, ya_s);
                                    }
                                }
                            }
                            if (y_ab1[ya_idx + 1] & 0x8000) == 0 {
                                let av_y = avg_32x8_pavgb(planes.dst_y, y_off - ya8, ya_s);
                                above_y[4..8].copy_from_slice(&av_y);
                                let av_cr = avg_16x8_pavgb(planes.dst_cr, c_off - c8, c_s);
                                above_cr[2..4].copy_from_slice(&av_cr);
                                let av_cb = avg_16x8_pavgb(planes.dst_cb, c_off - c8, c_s);
                                above_cb[2..4].copy_from_slice(&av_cb);
                                if alpha_coded {
                                    if let Some(ref buf) = planes.dst_a {
                                        let av_a = avg_32x8_pavgb(buf, y_off - ya8, ya_s);
                                        above_a[4..8].copy_from_slice(&av_a);
                                    }
                                }
                                if hdr_coded {
                                    if let Some(ref buf) = planes.dst_h {
                                        let av_h = avg_32x8_pavgb(buf, y_off - ya8, ya_s);
                                        above_h[4..8].copy_from_slice(&av_h);
                                    }
                                }
                            }
                        }

                        // If previous MB on this row was Inter, derive left-edge DCs via pavgb averages
                        if mx > 0 && (pf.y_win[0][5] & 0x8000) == 0 {
                            let use_top_scratch = (f & 4) != 0;
                            let use_bot_scratch = !use_top_scratch && (f & 3) != 0;
                            if use_top_scratch {
                                dc_y[5] = avg_8x8_pavgb(&scratch_y, 8, 16);
                                dc_y[7] = avg_8x8_pavgb(&scratch_y, 8 + 128, 16);
                                dc_cr[1] = avg_8x8_pavgb(&scratch_cr, 8, 16);
                                dc_cb[1] = avg_8x8_pavgb(&scratch_cb, 8, 16);
                                if alpha_coded {
                                    dc_a[5] = avg_8x8_pavgb(&scratch_a, 8, 16);
                                    dc_a[7] = avg_8x8_pavgb(&scratch_a, 8 + 128, 16);
                                }
                                if hdr_coded {
                                    dc_h[5] = avg_8x8_pavgb(&scratch_h, 8, 16);
                                    dc_h[7] = avg_8x8_pavgb(&scratch_h, 8 + 128, 16);
                                }
                            } else {
                                dc_y[5] = avg_8x8_pavgb(planes.dst_y, y_off - 8, ya_s);
                                dc_y[7] = avg_8x8_pavgb(planes.dst_y, y_off + ya_s * 8 - 8, ya_s);
                                dc_cr[1] = avg_8x8_pavgb(planes.dst_cr, c_off - 8, c_s);
                                dc_cb[1] = avg_8x8_pavgb(planes.dst_cb, c_off - 8, c_s);
                                if alpha_coded {
                                    if let Some(ref buf) = planes.dst_a {
                                        dc_a[5] = avg_8x8_pavgb(buf, y_off - 8, ya_s);
                                        dc_a[7] = avg_8x8_pavgb(buf, y_off + ya_s * 8 - 8, ya_s);
                                    }
                                }
                                if hdr_coded {
                                    if let Some(ref buf) = planes.dst_h {
                                        dc_h[5] = avg_8x8_pavgb(buf, y_off - 8, ya_s);
                                        dc_h[7] = avg_8x8_pavgb(buf, y_off + ya_s * 8 - 8, ya_s);
                                    }
                                }
                            }

                            if use_bot_scratch {
                                dc_y[13] = avg_8x8_pavgb(&scratch_y, 8, 16);
                                dc_y[15] = avg_8x8_pavgb(&scratch_y, 8 + 128, 16);
                                dc_cr[3] = avg_8x8_pavgb(&scratch_cr, 8, 16);
                                dc_cb[3] = avg_8x8_pavgb(&scratch_cb, 8, 16);
                                if alpha_coded {
                                    dc_a[13] = avg_8x8_pavgb(&scratch_a, 8, 16);
                                    dc_a[15] = avg_8x8_pavgb(&scratch_a, 8 + 128, 16);
                                }
                                if hdr_coded {
                                    dc_h[13] = avg_8x8_pavgb(&scratch_h, 8, 16);
                                    dc_h[15] = avg_8x8_pavgb(&scratch_h, 8 + 128, 16);
                                }
                            } else {
                                dc_y[13] = avg_8x8_pavgb(planes.dst_y, y_off + ya_s * 16 - 8, ya_s);
                                dc_y[15] = avg_8x8_pavgb(planes.dst_y, y_off + ya_s * 24 - 8, ya_s);
                                dc_cr[3] = avg_8x8_pavgb(planes.dst_cr, c_off + c_s * 8 - 8, c_s);
                                dc_cb[3] = avg_8x8_pavgb(planes.dst_cb, c_off + c_s * 8 - 8, c_s);
                                if alpha_coded {
                                    if let Some(ref buf) = planes.dst_a {
                                        dc_a[13] = avg_8x8_pavgb(buf, y_off + ya_s * 16 - 8, ya_s);
                                        dc_a[15] = avg_8x8_pavgb(buf, y_off + ya_s * 24 - 8, ya_s);
                                    }
                                }
                                if hdr_coded {
                                    if let Some(ref buf) = planes.dst_h {
                                        dc_h[13] = avg_8x8_pavgb(buf, y_off + ya_s * 16 - 8, ya_s);
                                        dc_h[15] = avg_8x8_pavgb(buf, y_off + ya_s * 24 - 8, ya_s);
                                    }
                                }
                            }
                        }

                        let q_val = (decode_qdelta(&mut bs, q_cur[mx][0] as i32) & 0xff) as u32;
                        q_cur[mx][0] = q_val as u8;

                        decode_intra_plane16(
                            &mut dc_y,
                            &above_y,
                            Some((planes.dst_y, y_off, ya_s)),
                            &mut bs,
                            &mut cur_y_info,
                            plane_flags,
                            q_val,
                            &mut cbp_intra_y,
                        );
                        decode_intra_plane4(
                            &mut dc_cr,
                            &above_cr,
                            planes.dst_cr,
                            c_off,
                            c_s,
                            &mut bs,
                            &mut cur_cr_info,
                            mb_flags,
                            q_val,
                            &mut cbp_intra_cr,
                        );
                        decode_intra_plane4(
                            &mut dc_cb,
                            &above_cb,
                            planes.dst_cb,
                            c_off,
                            c_s,
                            &mut bs,
                            &mut cur_cb_info,
                            mb_flags,
                            q_val,
                            &mut cbp_intra_cb,
                        );
                        if alpha_coded {
                            let dst_a_opt = planes.dst_a.as_deref_mut().map(|b| (b, y_off, ya_s));
                            decode_intra_plane16(
                                &mut dc_a,
                                &above_a,
                                dst_a_opt,
                                &mut bs,
                                &mut cur_a_info,
                                plane_flags,
                                q_val,
                                &mut cbp_intra_a,
                            );
                        }
                        if hdr_coded {
                            let dst_h_opt = planes.dst_h.as_deref_mut().map(|b| (b, y_off, ya_s));
                            decode_intra_plane16(
                                &mut dc_h,
                                &above_h,
                                dst_h_opt,
                                &mut bs,
                                &mut cur_h_info,
                                plane_flags,
                                q_val,
                                &mut cbp_intra_h,
                            );
                        }
                        if !is_keyframe {
                            intra_mv_median(mv_cur, mv_above, mx, mb_flags);
                        }
                    }
                    1 => {
                        // Skip / Copy from previous frame (zero MV)
                        if (f & 4) == 0 {
                            copy_16x16(planes.ref_y, y_off, ya_s, planes.dst_y, y_off, ya_s);
                            copy_16x16(
                                planes.ref_y,
                                y_off + 16,
                                ya_s,
                                planes.dst_y,
                                y_off + 16,
                                ya_s,
                            );
                            copy_16x8(planes.ref_cr, c_off, c_s, planes.dst_cr, c_off, c_s);
                            copy_16x8(planes.ref_cb, c_off, c_s, planes.dst_cb, c_off, c_s);
                        } else {
                            copy_16x16(planes.ref_y, y_off + 16, ya_s, &mut scratch_y, 0, 16);
                            copy_16x8(planes.ref_cr, c_off, c_s, &mut scratch_cr, 0, 16);
                            copy_16x8(planes.ref_cb, c_off, c_s, &mut scratch_cb, 0, 16);
                        }
                        if (f & 3) == 0 {
                            let y16 = ya_s * 16;
                            let c8 = c_s * 8;
                            copy_16x16(
                                planes.ref_y,
                                y_off + y16,
                                ya_s,
                                planes.dst_y,
                                y_off + y16,
                                ya_s,
                            );
                            copy_16x16(
                                planes.ref_y,
                                y_off + y16 + 16,
                                ya_s,
                                planes.dst_y,
                                y_off + y16 + 16,
                                ya_s,
                            );
                            copy_16x8(
                                planes.ref_cr,
                                c_off + c8,
                                c_s,
                                planes.dst_cr,
                                c_off + c8,
                                c_s,
                            );
                            copy_16x8(
                                planes.ref_cb,
                                c_off + c8,
                                c_s,
                                planes.dst_cb,
                                c_off + c8,
                                c_s,
                            );
                        } else if (f & 1) == 0 {
                            copy_16x16(
                                planes.ref_y,
                                y_off + ya_s * 16 + 16,
                                ya_s,
                                &mut scratch_y,
                                0,
                                16,
                            );
                            copy_16x8(planes.ref_cr, c_off + c_s * 8, c_s, &mut scratch_cr, 0, 16);
                            copy_16x8(planes.ref_cb, c_off + c_s * 8, c_s, &mut scratch_cb, 0, 16);
                        } else {
                            copy_16x16(planes.ref_y, y_off + 16, ya_s, &mut scratch_y, 0, 16);
                            copy_16x8(planes.ref_cr, c_off, c_s, &mut scratch_cr, 0, 16);
                            copy_16x8(planes.ref_cb, c_off, c_s, &mut scratch_cb, 0, 16);
                        }
                        if alpha_coded {
                            if let (Some(ref mut da), Some(ra)) = (&mut planes.dst_a, planes.ref_a) {
                                copy_skip_plane16(ra, da, y_off, ya_s, f, &mut scratch_a);
                            }
                        }
                        if hdr_coded {
                            if let (Some(ref mut dh), Some(rh)) = (&mut planes.dst_h, planes.ref_h) {
                                copy_skip_plane16(rh, dh, y_off, ya_s, f, &mut scratch_h);
                            }
                        }
                        mv_cur[mx * 8..mx * 8 + 8].fill(0);
                    }
                    2 => {
                        // Inter MC-only (zero residual)
                        decode_mvs(&mut bs, mv_cur, mv_above, mx, mb_flags);
                        let cmv = clamp_mvs(mv_cur, mx, x_px as i32, y_px as i32, w2, h2, mb_flags);
                        mc_only_plane16(planes.ref_y, planes.dst_y, y_off, ya_s, &cmv, f, &mut scratch_y);
                        mc_only_chroma(
                            planes.ref_cr,
                            planes.dst_cr,
                            planes.ref_cb,
                            planes.dst_cb,
                            c_off,
                            c_s,
                            &cmv,
                            f,
                            &mut scratch_cr,
                            &mut scratch_cb,
                        );
                        if alpha_coded {
                            if let (Some(ref mut da), Some(ra)) = (&mut planes.dst_a, planes.ref_a) {
                                mc_only_plane16(ra, da, y_off, ya_s, &cmv, f, &mut scratch_a);
                            }
                        }
                        if hdr_coded {
                            if let (Some(ref mut dh), Some(rh)) = (&mut planes.dst_h, planes.ref_h) {
                                mc_only_plane16(rh, dh, y_off, ya_s, &cmv, f, &mut scratch_h);
                            }
                        }
                    }
                    3 => {
                        // Inter MC + residual
                        decode_mvs(&mut bs, mv_cur, mv_above, mx, mb_flags);
                        let cmv = clamp_mvs(mv_cur, mx, x_px as i32, y_px as i32, w2, h2, mb_flags);
                        let q_val = (decode_qdelta(&mut bs, q_cur[mx][1] as i32) & 0xff) as u32;
                        q_cur[mx][1] = q_val as u8;

                        decode_inter_plane16(
                            Some((planes.dst_y, y_off, ya_s)),
                            Some((planes.ref_y, ya_s)),
                            &mut bs,
                            plane_flags,
                            &cmv,
                            q_val,
                            &mut cbp_inter_y,
                            &mut scratch_y,
                        );

                        if bs.read_bit() == 0 {
                            cbp_inter_cr = 0;
                            cbp_inter_cb = 0;
                            mc_only_chroma(
                                planes.ref_cr,
                                planes.dst_cr,
                                planes.ref_cb,
                                planes.dst_cb,
                                c_off,
                                c_s,
                                &cmv,
                                f,
                                &mut scratch_cr,
                                &mut scratch_cb,
                            );
                        } else {
                            decode_inter_plane4(
                                planes.dst_cr,
                                c_off,
                                c_s,
                                planes.ref_cr,
                                c_s,
                                &mut bs,
                                &cmv,
                                plane_flags,
                                q_val,
                                &mut cbp_inter_cr,
                                &mut scratch_cr,
                            );
                            decode_inter_plane4(
                                planes.dst_cb,
                                c_off,
                                c_s,
                                planes.ref_cb,
                                c_s,
                                &mut bs,
                                &cmv,
                                plane_flags,
                                q_val,
                                &mut cbp_inter_cb,
                                &mut scratch_cb,
                            );
                        }

                        if alpha_coded {
                            let da_opt = planes.dst_a.as_deref_mut().map(|b| (b, y_off, ya_s));
                            let ra_opt = planes.ref_a.map(|b| (b, ya_s));
                            decode_inter_plane16(
                                da_opt,
                                ra_opt,
                                &mut bs,
                                plane_flags,
                                &cmv,
                                q_val,
                                &mut cbp_inter_a,
                                &mut scratch_a,
                            );
                        }
                        if hdr_coded {
                            let dh_opt = planes.dst_h.as_deref_mut().map(|b| (b, y_off, ya_s));
                            let rh_opt = planes.ref_h.map(|b| (b, ya_s));
                            decode_inter_plane16(
                                dh_opt,
                                rh_opt,
                                &mut bs,
                                plane_flags,
                                &cmv,
                                q_val,
                                &mut cbp_inter_h,
                                &mut scratch_h,
                            );
                        }
                    }
                    _ => unreachable!(),
                }

                postfilter_mb(
                    &mut pf,
                    &mut y_ab2,
                    &mut y_ab1,
                    &mut a_ab2,
                    &mut a_ab1,
                    &mut h_ab2,
                    &mut h_ab1,
                    &mut cr_ab2,
                    &mut cr_ab1,
                    &mut cb_ab2,
                    &mut cb_ab1,
                    &cur_y_info,
                    &cur_cr_info,
                    &cur_cb_info,
                    &cur_a_info,
                    &cur_h_info,
                    mb_flags,
                    planes,
                    mx,
                    y_px,
                    mv_cur,
                    mv_above,
                    thresh,
                );

                // Predict Q for next MB on this row
                if mx + 1 < mb_cols {
                    if (mb_flags & 0x88) == 0 {
                        q_cur[mx + 1][0] =
                            median3_u8(q_above[mx][0], q_above[mx + 1][0], q_cur[mx][0]);
                        q_cur[mx + 1][1] =
                            median3_u8(q_above[mx][1], q_above[mx + 1][1], q_cur[mx][1]);
                    } else {
                        q_cur[mx + 1] = q_cur[mx];
                    }
                }
            }

            cur_is_a = !cur_is_a;
            row_carry = ((row_flags & 4) << 2) | (row_flags & 0xffff_fe60);
            row_flags_state = row_carry;
            y_px = next_y;
        }

        cur_slice += 1;
        row_carry = 0;
        bs.align_u32();
        if cur_slice < slices.num_slices as usize {
            let expected_off = 4 + (cur_slice - 1) * 4;
            if expected_off + 4 <= hdr_offset {
                let hdr_slice_end = u32::from_le_bytes([
                    packet[expected_off],
                    packet[expected_off + 1],
                    packet[expected_off + 2],
                    packet[expected_off + 3],
                ]) as usize;
                let actual = bs.byte_pos();
                if actual != hdr_slice_end {
                    log::warn!(
                        "bink2: slice {cur_slice} pos={actual:#x}, header_val={hdr_slice_end:#x} (pkt_len={:#x})",
                        packet.len()
                    );
                }
            }
        }
    }

    Ok(())
}

#[inline]
fn copy_skip_plane16(
    ref_buf: &[u8],
    dst_buf: &mut [u8],
    off: usize,
    stride: usize,
    f: u32,
    scratch: &mut [u8; 256],
) {
    if (f & 4) == 0 {
        copy_16x16(ref_buf, off, stride, dst_buf, off, stride);
        copy_16x16(ref_buf, off + 16, stride, dst_buf, off + 16, stride);
    } else {
        copy_16x16(ref_buf, off + 16, stride, scratch, 0, 16);
    }
    if (f & 3) == 0 {
        let s16 = stride * 16;
        copy_16x16(ref_buf, off + s16, stride, dst_buf, off + s16, stride);
        copy_16x16(ref_buf, off + s16 + 16, stride, dst_buf, off + s16 + 16, stride);
    } else if (f & 1) == 0 {
        copy_16x16(ref_buf, off + stride * 16 + 16, stride, scratch, 0, 16);
    } else {
        copy_16x16(ref_buf, off + 16, stride, scratch, 0, 16);
    }
}

#[inline]
fn mc_only_plane16(
    ref_buf: &[u8],
    dst_buf: &mut [u8],
    off: usize,
    stride: usize,
    cmv: &[u32; 8],
    f: u32,
    scratch: &mut [u8; 256],
) {
    if (f & 4) == 0 {
        mc_luma_16x16(
            ref_buf,
            stride,
            (cmv[0] >> 1) as i32,
            (cmv[1] >> 1) as i32,
            dst_buf,
            off,
            stride,
            (cmv[0] & 1) != 0,
            (cmv[1] & 1) != 0,
        );
        mc_luma_16x16(
            ref_buf,
            stride,
            (cmv[2] >> 1) as i32,
            (cmv[3] >> 1) as i32,
            dst_buf,
            off + 16,
            stride,
            (cmv[2] & 1) != 0,
            (cmv[3] & 1) != 0,
        );
    } else {
        mc_luma_16x16(
            ref_buf,
            stride,
            (cmv[2] >> 1) as i32,
            (cmv[3] >> 1) as i32,
            scratch,
            0,
            16,
            (cmv[2] & 1) != 0,
            (cmv[3] & 1) != 0,
        );
    }
    if (f & 3) == 0 {
        let s16 = stride * 16;
        mc_luma_16x16(
            ref_buf,
            stride,
            (cmv[4] >> 1) as i32,
            (cmv[5] >> 1) as i32,
            dst_buf,
            off + s16,
            stride,
            (cmv[4] & 1) != 0,
            (cmv[5] & 1) != 0,
        );
        mc_luma_16x16(
            ref_buf,
            stride,
            (cmv[6] >> 1) as i32,
            (cmv[7] >> 1) as i32,
            dst_buf,
            off + s16 + 16,
            stride,
            (cmv[6] & 1) != 0,
            (cmv[7] & 1) != 0,
        );
    } else {
        mc_luma_16x16(
            ref_buf,
            stride,
            (cmv[6] >> 1) as i32,
            (cmv[7] >> 1) as i32,
            scratch,
            0,
            16,
            (cmv[6] & 1) != 0,
            (cmv[7] & 1) != 0,
        );
    }
}

#[allow(clippy::too_many_arguments)]
#[inline]
fn mc_only_chroma(
    ref_cr: &[u8],
    dst_cr: &mut [u8],
    ref_cb: &[u8],
    dst_cb: &mut [u8],
    off: usize,
    stride: usize,
    cmv: &[u32; 8],
    f: u32,
    scratch_cr: &mut [u8; 128],
    scratch_cb: &mut [u8; 128],
) {
    if (f & 4) == 0 {
        mc_chroma_8x8(
            ref_cr,
            stride,
            (cmv[0] >> 2) as i32,
            (cmv[1] >> 2) as i32,
            dst_cr,
            off,
            stride,
            cmv[0] & 3,
            cmv[1] & 3,
        );
        mc_chroma_8x8(
            ref_cr,
            stride,
            (cmv[2] >> 2) as i32,
            (cmv[3] >> 2) as i32,
            dst_cr,
            off + 8,
            stride,
            cmv[2] & 3,
            cmv[3] & 3,
        );
        mc_chroma_8x8(
            ref_cb,
            stride,
            (cmv[0] >> 2) as i32,
            (cmv[1] >> 2) as i32,
            dst_cb,
            off,
            stride,
            cmv[0] & 3,
            cmv[1] & 3,
        );
        mc_chroma_8x8(
            ref_cb,
            stride,
            (cmv[2] >> 2) as i32,
            (cmv[3] >> 2) as i32,
            dst_cb,
            off + 8,
            stride,
            cmv[2] & 3,
            cmv[3] & 3,
        );
    } else {
        mc_chroma_8x8(
            ref_cr,
            stride,
            (cmv[2] >> 2) as i32,
            (cmv[3] >> 2) as i32,
            scratch_cr,
            8,
            16,
            cmv[2] & 3,
            cmv[3] & 3,
        );
        mc_chroma_8x8(
            ref_cb,
            stride,
            (cmv[2] >> 2) as i32,
            (cmv[3] >> 2) as i32,
            scratch_cb,
            8,
            16,
            cmv[2] & 3,
            cmv[3] & 3,
        );
    }
    if (f & 3) == 0 {
        let s8 = stride * 8;
        mc_chroma_8x8(
            ref_cr,
            stride,
            (cmv[4] >> 2) as i32,
            (cmv[5] >> 2) as i32,
            dst_cr,
            off + s8,
            stride,
            cmv[4] & 3,
            cmv[5] & 3,
        );
        mc_chroma_8x8(
            ref_cr,
            stride,
            (cmv[6] >> 2) as i32,
            (cmv[7] >> 2) as i32,
            dst_cr,
            off + s8 + 8,
            stride,
            cmv[6] & 3,
            cmv[7] & 3,
        );
        mc_chroma_8x8(
            ref_cb,
            stride,
            (cmv[4] >> 2) as i32,
            (cmv[5] >> 2) as i32,
            dst_cb,
            off + s8,
            stride,
            cmv[4] & 3,
            cmv[5] & 3,
        );
        mc_chroma_8x8(
            ref_cb,
            stride,
            (cmv[6] >> 2) as i32,
            (cmv[7] >> 2) as i32,
            dst_cb,
            off + s8 + 8,
            stride,
            cmv[6] & 3,
            cmv[7] & 3,
        );
    } else {
        mc_chroma_8x8(
            ref_cr,
            stride,
            (cmv[6] >> 2) as i32,
            (cmv[7] >> 2) as i32,
            scratch_cr,
            8,
            16,
            cmv[6] & 3,
            cmv[7] & 3,
        );
        mc_chroma_8x8(
            ref_cb,
            stride,
            (cmv[6] >> 2) as i32,
            (cmv[7] >> 2) as i32,
            scratch_cb,
            8,
            16,
            cmv[6] & 3,
            cmv[7] & 3,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slices_match_bink2_rules() {
        let kb2n = u32::from_le_bytes(*b"KB2n");
        let s1080 = compute_slices(kb2n, 0, 1920, 1080);
        assert_eq!(s1080.num_slices, 2);
        assert_eq!(&s1080.slice_ends[..2], &[544, 1088]);

        let s4k = compute_slices(kb2n, 0, 3840, 2160);
        assert_eq!(s4k.num_slices, 2);
        assert_eq!(&s4k.slice_ends[..2], &[1088, 2176]);

        let s4k_4 = compute_slices(kb2n, 2, 3840, 2160);
        assert_eq!(s4k_4.num_slices, 4);
        assert_eq!(&s4k_4.slice_ends[..4], &[544, 1088, 1632, 2176]);

        let small = compute_slices(kb2n, 0, 64, 64);
        assert_eq!(small.num_slices, 1);
        assert_eq!(small.slice_ends[0], 64);
    }

    #[test]
    fn bit_reader_cross_word_and_align() {
        let data = [0xAA, 0x55, 0xF0, 0x0F, 0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF1];
        let mut bs = BitReader::new(&data, 0);
        assert_eq!(bs.read_bits(4), 0xA);
        assert_eq!(bs.peek_bits(8), 0x5A);
        assert_eq!(bs.read_bits(12), 0x55A);
        assert_eq!(bs.read_bits(20), 0x20FF0);
        bs.align_u32();
        assert_eq!(bs.byte_pos(), 8);

        // Verify refill() followed by align_u32() preserves unread full word and exact byte_pos.
        let mut bs2 = BitReader::new(&data, 0);
        assert_eq!(bs2.read_bits(5), 0x0A);
        bs2.refill(); // loads second 32-bit word while first is partially consumed
        bs2.align_u32(); // discards remaining 27 bits of word 0, preserves word 1
        assert_eq!(bs2.byte_pos(), 4);
        assert_eq!(bs2.read_bits(8), 0x12);
        assert_eq!(bs2.read_bits(24), 0x785634);
        assert_eq!(bs2.byte_pos(), 8);
        assert_eq!(bs2.read_bits(32), 0xF1DEBC9A);
        assert_eq!(bs2.byte_pos(), 12);
    }
}
