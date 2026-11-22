//! Hashing: XXH64 (stable, dependency-free) for cache keys, and the DXBC container checksum
//! (a modified MD5, `ComputeHashRetail` in DXC) so blobs we build are valid containers.

const P1: u64 = 0x9E37_79B1_85EB_CA87;
const P2: u64 = 0xC2B2_AE3D_27D4_EB4F;
const P3: u64 = 0x1656_67B1_9E37_79F9;
const P4: u64 = 0x85EB_CA77_C2B2_AE63;
const P5: u64 = 0x27D4_EB2F_1656_67C5;

#[inline]
fn round(acc: u64, input: u64) -> u64 {
    acc.wrapping_add(input.wrapping_mul(P2)).rotate_left(31).wrapping_mul(P1)
}
#[inline]
fn merge(acc: u64, val: u64) -> u64 {
    (acc ^ round(0, val)).wrapping_mul(P1).wrapping_add(P4)
}
#[inline]
fn rd64(b: &[u8], i: usize) -> u64 {
    u64::from_le_bytes(b[i..i + 8].try_into().unwrap())
}
#[inline]
fn rd32(b: &[u8], i: usize) -> u32 {
    u32::from_le_bytes(b[i..i + 4].try_into().unwrap())
}

/// XXH64 of `data` with `seed` (reference algorithm).
pub fn xxh64(data: &[u8], seed: u64) -> u64 {
    let len = data.len();
    let mut i = 0;
    let mut h = if len >= 32 {
        let mut v = [
            seed.wrapping_add(P1).wrapping_add(P2),
            seed.wrapping_add(P2),
            seed,
            seed.wrapping_sub(P1),
        ];
        while i + 32 <= len {
            for (k, vk) in v.iter_mut().enumerate() {
                *vk = round(*vk, rd64(data, i + k * 8));
            }
            i += 32;
        }
        let mut h = v[0]
            .rotate_left(1)
            .wrapping_add(v[1].rotate_left(7))
            .wrapping_add(v[2].rotate_left(12))
            .wrapping_add(v[3].rotate_left(18));
        for vk in v {
            h = merge(h, vk);
        }
        h
    } else {
        seed.wrapping_add(P5)
    };
    h = h.wrapping_add(len as u64);
    while i + 8 <= len {
        h ^= round(0, rd64(data, i));
        h = h.rotate_left(27).wrapping_mul(P1).wrapping_add(P4);
        i += 8;
    }
    if i + 4 <= len {
        h ^= (rd32(data, i) as u64).wrapping_mul(P1);
        h = h.rotate_left(23).wrapping_mul(P2).wrapping_add(P3);
        i += 4;
    }
    while i < len {
        h ^= (data[i] as u64).wrapping_mul(P5);
        h = h.rotate_left(11).wrapping_mul(P1);
        i += 1;
    }
    h ^= h >> 33;
    h = h.wrapping_mul(P2);
    h ^= h >> 29;
    h = h.wrapping_mul(P3);
    h ^ (h >> 32)
}

/// 128-bit content hash (two seeded XXH64 lanes). Not cryptographic; collision odds are
/// negligible for a few hundred thousand shaders.
pub fn hash128(data: &[u8]) -> u128 {
    ((xxh64(data, 0x4157_3253_4841_4452) as u128) << 64) | xxh64(data, 0x6177_3273_6861_6472) as u128
}

/// Incremental key builder: feeds length-prefixed fields so field boundaries are unambiguous.
#[derive(Default, Clone)]
pub struct KeyHasher {
    buf: Vec<u8>,
}

impl KeyHasher {
    pub fn new(domain: &str) -> Self {
        let mut k = Self::default();
        k.bytes(domain.as_bytes());
        k
    }
    pub fn bytes(&mut self, b: &[u8]) -> &mut Self {
        self.buf.extend_from_slice(&(b.len() as u64).to_le_bytes());
        self.buf.extend_from_slice(b);
        self
    }
    pub fn u32(&mut self, v: u32) -> &mut Self {
        self.buf.extend_from_slice(&v.to_le_bytes());
        self
    }
    pub fn u64(&mut self, v: u64) -> &mut Self {
        self.buf.extend_from_slice(&v.to_le_bytes());
        self
    }
    pub fn u128(&mut self, v: u128) -> &mut Self {
        self.buf.extend_from_slice(&v.to_le_bytes());
        self
    }
    pub fn finish(&self) -> u128 {
        hash128(&self.buf)
    }
}

// ---- DXBC checksum ---------------------------------------------------------------------------

fn md5_k() -> &'static [u32; 64] {
    static K: std::sync::OnceLock<[u32; 64]> = std::sync::OnceLock::new();
    K.get_or_init(|| std::array::from_fn(|i| ((i as f64 + 1.0).sin().abs() * 4294967296.0) as u32))
}

fn md5_transform(state: &mut [u32; 4], x: &[u32; 16]) {
    const S: [[u32; 4]; 4] = [[7, 12, 17, 22], [5, 9, 14, 20], [4, 11, 16, 23], [6, 10, 15, 21]];
    let kt = md5_k();
    let [mut a, mut b, mut c, mut d] = *state;
    for i in 0..64 {
        let (f, g) = match i / 16 {
            0 => ((b & c) | (!b & d), i),
            1 => ((d & b) | (!d & c), (5 * i + 1) % 16),
            2 => (b ^ c ^ d, (3 * i + 5) % 16),
            _ => (c ^ (b | !d), (7 * i) % 16),
        };
        let k = kt[i];
        let t = d;
        d = c;
        c = b;
        b = b.wrapping_add(
            a.wrapping_add(f).wrapping_add(k).wrapping_add(x[g]).rotate_left(S[i / 16][i % 4]),
        );
        a = t;
    }
    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
}

/// DXBC container checksum over `data` (the container bytes after the 20-byte magic+digest).
pub fn dxbc_checksum(data: &[u8]) -> [u8; 16] {
    let n = data.len();
    let mut state = [0x6745_2301u32, 0xefcd_ab89, 0x98ba_dcfe, 0x1032_5476];
    let words = |b: &[u8; 64]| -> [u32; 16] { std::array::from_fn(|i| rd32(b, i * 4)) };
    let full = n / 64;
    for blk in 0..full {
        let b: &[u8; 64] = data[blk * 64..blk * 64 + 64].try_into().unwrap();
        md5_transform(&mut state, &words(b));
    }
    let rem = &data[full * 64..];
    let bits = (n as u32).wrapping_shl(3);
    let tail = 1 | (n as u32).wrapping_shl(1);
    let mut last = [0u8; 64];
    if rem.len() < 56 {
        // [bits][rem][0x80 0..][tail]
        last[0..4].copy_from_slice(&bits.to_le_bytes());
        last[4..4 + rem.len()].copy_from_slice(rem);
        last[4 + rem.len()] = 0x80;
        last[60..64].copy_from_slice(&tail.to_le_bytes());
        md5_transform(&mut state, &words(&last));
    } else {
        let mut row = [0u8; 64];
        row[..rem.len()].copy_from_slice(rem);
        row[rem.len()] = 0x80;
        md5_transform(&mut state, &words(&row));
        last[0..4].copy_from_slice(&bits.to_le_bytes());
        last[60..64].copy_from_slice(&tail.to_le_bytes());
        md5_transform(&mut state, &words(&last));
    }
    let mut out = [0u8; 16];
    for (i, s) in state.iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&s.to_le_bytes());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xxh64_reference_vectors() {
        assert_eq!(xxh64(b"", 0), 0xEF46_DB37_51D8_E999);
        assert_eq!(xxh64(b"a", 0), 0xD24E_C4F1_A98C_6E5B);
        assert_eq!(xxh64(b"abc", 0), 0x44BC_2CF5_AD77_0999);
        assert_eq!(
            xxh64(b"Nobody inspects the spammish repetition", 0),
            0xFBCE_A83C_8A37_8BF1
        );
    }
}
